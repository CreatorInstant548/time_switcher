//! Étapes de `cargo xtask check` qui lancent des outils ou lisent le dépôt.
//! spec: carnet architecture-securite.md SEC0-03, 06, 08, 19, 20, 26, 29 à 31 ; §9.5 F1 à F10

use std::cell::OnceCell;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::manifests::{self, DEV_FEATURES, HOSTS, TOOLS};
use crate::metadata::{self, Metadata};
use crate::sys::{self, cargo, run_captured, run_live, stdout_of};
use crate::{scan, workflows};

pub enum Outcome {
    Ok(String),
    Skipped(String),
    Fail(String),
}

/// Fichiers de la racine dont la modification vaut `--all` (F7).
const ROOT_FILES: [&str; 7] = [
    "Cargo.toml",
    "Cargo.lock",
    "clippy.toml",
    ".clippy.toml",
    "deny.toml",
    "rustfmt.toml",
    "rust-toolchain.toml",
];
/// Âge maximal de la base RustSec locale (SEC0-03).
const MAX_DB_AGE: Duration = Duration::from_secs(7 * 24 * 3600);

pub struct Ctx {
    pub root: PathBuf,
    pub all: bool,
    meta: OnceCell<Result<Metadata, String>>,
    files: OnceCell<Result<Vec<String>, String>>,
    changed: OnceCell<Result<Vec<String>, String>>,
}

impl Ctx {
    pub fn new(root: PathBuf, all: bool) -> Self {
        let (meta, files, changed) = (OnceCell::new(), OnceCell::new(), OnceCell::new());
        Self {
            root,
            all,
            meta,
            files,
            changed,
        }
    }

    fn meta(&self) -> Result<&Metadata, String> {
        self.meta
            .get_or_init(|| metadata::load(&self.root))
            .as_ref()
            .map_err(Clone::clone)
    }

    fn git(&self, args: &[&str]) -> Result<String, String> {
        stdout_of(&self.root, "git", args)
    }

    fn git_lines(&self, args: &[&str]) -> Result<Vec<String>, String> {
        Ok(self
            .git(args)?
            .lines()
            .filter(|l| !l.is_empty())
            .map(str::to_owned)
            .collect())
    }

    /// Fichiers ordinaires présents sur le disque : suivis, plus non suivis et non ignorés (F2).
    /// Ni lien, ni fichier supprimé mais encore indexé.
    fn files(&self) -> Result<&[String], String> {
        let init = || {
            let args = [
                "-c",
                "core.quotepath=off",
                "ls-files",
                "-co",
                "--exclude-standard",
            ];
            let mut files = self.git_lines(&args)?;
            files.retain(|f| {
                std::fs::symlink_metadata(self.root.join(f)).is_ok_and(|m| m.file_type().is_file())
            });
            Ok(files)
        };
        self.files
            .get_or_init(init)
            .as_ref()
            .map(Vec::as_slice)
            .map_err(Clone::clone)
    }

    /// Fichiers modifiés depuis `origin/main`, commités ou non (F7).
    fn changed(&self) -> Result<&[String], String> {
        let init = || {
            // Sans renommage, un déplacement compte ses deux chemins ; noms non ASCII lisibles.
            let diff = [
                "-c",
                "core.quotepath=off",
                "diff",
                "--name-only",
                "--no-renames",
            ];
            let mut all = self.git_lines(&[&diff[..], &["origin/main...HEAD"]].concat())?;
            all.extend(self.git_lines(&[&diff[..], &["HEAD"]].concat())?);
            let others = [
                "-c",
                "core.quotepath=off",
                "ls-files",
                "-o",
                "--exclude-standard",
            ];
            all.extend(self.git_lines(&others)?);
            Ok(all)
        };
        self.changed
            .get_or_init(init)
            .as_ref()
            .map(Vec::as_slice)
            .map_err(Clone::clone)
    }

    fn read(&self, rel: &str) -> Result<Vec<u8>, String> {
        sys::read(&self.root.join(rel))
    }
}

fn status(ok: bool, what: &str) -> Outcome {
    if ok {
        Outcome::Ok(String::new())
    } else {
        Outcome::Fail(format!("{what} a échoué"))
    }
}

fn from_errors(errors: Vec<String>, ok: String) -> Outcome {
    if errors.is_empty() {
        return Outcome::Ok(ok);
    }
    for e in &errors {
        println!("  - {e}");
    }
    Outcome::Fail(format!("{} écart(s)", errors.len()))
}

macro_rules! tri {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Outcome::Fail(e),
        }
    };
}

pub fn fmt(cx: &Ctx) -> Outcome {
    status(
        tri!(run_live(&cx.root, &cargo(), &["fmt", "--all", "--check"])),
        "cargo fmt",
    )
}

/// Espace de travail entier, puis graphe livré seul avec `public` (F1).
pub fn clippy(cx: &Ctx) -> Outcome {
    let args = [
        "clippy",
        "--workspace",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ];
    if !tri!(run_live(&cx.root, &cargo(), &args)) {
        return Outcome::Fail("cargo clippy a échoué".to_owned());
    }
    let meta = tri!(cx.meta());
    let mut shipped = BTreeSet::new();
    for root in ["ts_kernel"]
        .into_iter()
        .chain(HOSTS)
        .filter(|n| meta.package(n).is_some())
    {
        shipped.extend(meta.normal_closure(root));
    }
    let mut args = vec!["clippy", "--locked", "--features", "ts_kernel/public"];
    shipped.iter().for_each(|p| args.extend(["-p", p.as_str()]));
    args.extend(["--", "-D", "warnings"]);
    status(
        tri!(run_live(&cx.root, &cargo(), &args)),
        "cargo clippy (profil public)",
    )
}

pub fn test(cx: &Ctx) -> Outcome {
    let crates: BTreeSet<String>;
    let mut args = vec!["test", "--locked"];
    let changed = if cx.all {
        None
    } else {
        Some(tri!(cx.changed()))
    };
    let root_change = changed.is_some_and(|c| {
        c.iter()
            .any(|f| ROOT_FILES.contains(&f.as_str()) || f.starts_with(".cargo/"))
    });
    if let (Some(changed), false) = (changed, root_change) {
        let meta = tri!(cx.meta());
        let touched = changed
            .iter()
            .filter_map(|f| meta.package_of(f))
            .map(str::to_owned)
            .collect();
        crates = meta.with_dependents(touched);
        if crates.is_empty() {
            return Outcome::Skipped("aucune crate touchée".to_owned());
        }
        crates.iter().for_each(|c| args.extend(["-p", c.as_str()]));
    } else {
        args.push("--workspace");
    }
    status(tri!(run_live(&cx.root, &cargo(), &args)), "cargo test")
}

/// Copie épinglée des crochets, désignée par `core.hooksPath` (F2) : jamais la copie de travail.
fn pinned_dir(cx: &Ctx) -> Result<PathBuf, String> {
    let dir = PathBuf::from(cx.git(&["config", "--get", "core.hooksPath"])?.trim());
    let name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let hooks = sys::ts_cache().join("hooks");
    let same_parent = dir.parent().is_some_and(|p| {
        p.to_string_lossy()
            .replace('\\', "/")
            .eq_ignore_ascii_case(&hooks.to_string_lossy().replace('\\', "/"))
    });
    // Empreinte du jeu installé, écrite par install-hooks.ps1 ; -Verify recalcule le contenu.
    let current = sys::read_text(&hooks.join("current.sha256")).unwrap_or_default();
    let hex = name.len() == 64 && name.bytes().all(|b| b.is_ascii_hexdigit());
    if !same_parent || !hex || !current.trim().eq_ignore_ascii_case(&name) {
        return Err(
            "core.hooksPath ne désigne pas la copie épinglée TS_CACHE/hooks/<current.sha256> : \
             réinstaller les crochets (tools/dev/install-hooks.ps1)"
                .to_owned(),
        );
    }
    Ok(dir)
}

fn lock_value(lock: &str, key: &str) -> Result<String, String> {
    lock.lines()
        .filter_map(|l| l.split_once('='))
        .find(|(k, _)| k.trim() == key)
        .map(|(_, v)| v.trim().trim_matches('"').to_owned())
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("{key} absent de tools.lock"))
}

fn pinned_lock(cx: &Ctx) -> Result<(PathBuf, String), String> {
    let dir = pinned_dir(cx)?;
    let lock = sys::read_text(&dir.join("tools.lock"))?;
    Ok((dir, lock))
}

/// Version installée d'un outil cargo, comparée au `tools.lock` du dépôt (F9). Une version n'est
/// pas une empreinte : elle se lit dans le dépôt, donc aussi en CI, sans crochets installés.
fn tool_version(cx: &Ctx, tool: &str) -> Result<(), String> {
    let lock = sys::read_text(&cx.root.join("tools/dev/tools.lock"))?;
    let expected = lock_value(&lock, &format!("cargo-{tool}.version"))?;
    let out = stdout_of(&cx.root, &cargo(), &[tool, "--version"])?;
    let actual = out.split_whitespace().last().unwrap_or_default();
    match actual == expected {
        true => Ok(()),
        false => Err(format!(
            "cargo-{tool} {actual} installé, {expected} attendu (tools.lock)"
        )),
    }
}

pub fn deny(cx: &Ctx) -> Outcome {
    tri!(tool_version(cx, "deny"));
    status(
        tri!(run_live(&cx.root, &cargo(), &["deny", "--locked", "check"])),
        "cargo deny",
    )
}

fn advisory_db_age() -> Option<Duration> {
    let home = sys::env("CARGO_HOME").map(PathBuf::from).or_else(|| {
        sys::env("USERPROFILE")
            .or_else(|| sys::env("HOME"))
            .map(|h| Path::new(&h).join(".cargo"))
    })?;
    let git = home.join("advisory-db").join(".git");
    ["FETCH_HEAD", "index"]
        .iter()
        .filter_map(|f| sys::age(&git.join(f)))
        .min()
}

/// `cargo audit` si `Cargo.lock` change, avec `--all`, ou si la base a plus de 7 jours. Si la
/// base ne peut pas être mise à jour, la copie locale ne sert que si elle a moins de 7 jours.
pub fn audit(cx: &Ctx) -> Outcome {
    tri!(tool_version(cx, "audit"));
    let age = advisory_db_age();
    let fresh = age.is_some_and(|a| a < MAX_DB_AGE);
    let lock_changed = cx
        .changed()
        .map_or(true, |c| c.iter().any(|f| f == "Cargo.lock"));
    if !cx.all && !lock_changed && fresh {
        return Outcome::Skipped(
            "Cargo.lock inchangé et base RustSec de moins de 7 jours".to_owned(),
        );
    }
    if tri!(run_live(&cx.root, &cargo(), &["audit"])) {
        return Outcome::Ok(String::new());
    }
    if !fresh {
        return Outcome::Fail(
            "cargo audit a échoué, et la base locale a plus de 7 jours".to_owned(),
        );
    }
    println!("  Nouvel essai sur la base locale, de moins de 7 jours :");
    match tri!(run_live(&cx.root, &cargo(), &["audit", "--no-fetch"])) {
        true => Outcome::Ok("base locale, mise à jour impossible".to_owned()),
        false => Outcome::Fail("cargo audit a échoué".to_owned()),
    }
}

/// gitleaks épinglé (SHA-256 vérifié), configuration épinglée, `--redact` (F2, SEC0-08).
pub fn gitleaks(cx: &Ctx) -> Outcome {
    let (dir, lock) = tri!(pinned_lock(cx));
    let bin = PathBuf::from(tri!(lock_value(&lock, "gitleaks.path")));
    let expected = tri!(lock_value(&lock, "gitleaks.sha256")).to_ascii_lowercase();
    if !bin.is_file() || tri!(sys::sha256(&cx.root, &bin)) != expected {
        return Outcome::Fail(format!(
            "gitleaks absent ou d'empreinte inattendue ({})",
            bin.display()
        ));
    }
    let config = dir.join("gitleaks.toml");
    if !config.is_file() {
        return Outcome::Fail("gitleaks.toml absent de la copie épinglée".to_owned());
    }
    if cx.root.join(".gitleaksignore").exists() {
        return Outcome::Fail(".gitleaksignore à la racine refusé (SEC0-08)".to_owned());
    }
    let (cfg, ign) = (
        config.to_string_lossy().into_owned(),
        dir.to_string_lossy().into_owned(),
    );
    let common = [
        "--config",
        &cfg,
        "--gitleaks-ignore-path",
        &ign,
        "--ignore-gitleaks-allow",
        "--redact",
        "--no-banner",
        "--log-level",
        "warn",
    ];
    let bin = bin.to_string_lossy().into_owned();

    let scan = sys::ts_cache()
        .join("tmp")
        .join(format!("xtask-gitleaks-{}", std::process::id()));
    let extracted = extract(cx, &scan);
    let tree_ok = extracted.and_then(|()| {
        let target = scan.to_string_lossy().into_owned();
        run_live(
            &cx.root,
            &bin,
            &[&["dir"][..], &common, &[target.as_str()]].concat(),
        )
    });
    let _ = std::fs::remove_dir_all(&scan); // dossier créé ici, qui ne contient que des copies
    if !tri!(tree_ok) {
        return Outcome::Fail("gitleaks a trouvé un secret probable dans l'arbre".to_owned());
    }
    tri!(cx.git(&["rev-parse", "--verify", "--quiet", "origin/main"]));
    let range = "--log-opts=--text --no-ext-diff --no-textconv origin/main..HEAD";
    match tri!(run_live(
        &cx.root,
        &bin,
        &[&["git"][..], &common, &[range, "."]].concat()
    )) {
        true => Outcome::Ok("arbre et commits depuis origin/main".to_owned()),
        false => Outcome::Fail("gitleaks a trouvé un secret probable dans les commits".to_owned()),
    }
}

/// Copie des fichiers ordinaires de l'arbre, comme le `pre-commit` : aucun attribut ne masque
/// leur contenu ; ni `target/` ni lien. Un `.gitleaksignore` est renommé.
fn extract(cx: &Ctx, scan: &Path) -> Result<(), String> {
    std::fs::create_dir_all(scan.parent().unwrap_or(scan)).map_err(|e| e.to_string())?;
    std::fs::create_dir(scan).map_err(|e| format!("{} : {e}", scan.display()))?;
    for rel in cx.files()?.iter().filter(|f| !f.starts_with("target/")) {
        let src = cx.root.join(rel);
        let mut dst = scan.join(rel);
        if dst.file_name().is_some_and(|n| n == ".gitleaksignore") {
            dst.set_file_name(".gitleaksignore.lu");
        }
        let copied = dst
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::copy(&src, &dst));
        copied.map_err(|e| format!("copie de {rel} impossible : {e}"))?;
    }
    Ok(())
}

pub fn unsafe_code(cx: &Ctx) -> Outcome {
    let meta = tri!(cx.meta());
    let mut errors = Vec::new();
    for (name, src) in manifests::crate_roots(meta) {
        if let Err(e) =
            sys::read_text(Path::new(src)).and_then(|t| manifests::check_unsafe_root(name, &t))
        {
            errors.push(format!("{name} ({src}) : {e}"));
        }
    }
    // ts_py : chaque allow ou expect de unsafe_code, dans tout fichier, suit un // SAFETY:.
    if let Some(py) = meta.package("ts_py") {
        let dir = format!("{}/", meta.rel_dir(py));
        for rel in tri!(cx.files())
            .iter()
            .filter(|f| f.starts_with(&dir) && f.ends_with(".rs"))
        {
            let text = String::from_utf8_lossy(&tri!(cx.read(rel))).into_owned();
            if let Some(line) = manifests::unsafe_allow_without_safety(&text) {
                errors.push(format!(
                    "{rel}:{line} : exemption de unsafe_code sans // SAFETY:"
                ));
            }
        }
    }
    for p in &meta.packages {
        if let Err(e) = sys::read_text(Path::new(&p.manifest_path))
            .and_then(|t| manifests::check_lints_section(&t))
        {
            errors.push(format!("{} : {e}", p.name));
        }
    }
    from_errors(errors, format!("{} crate(s)", meta.packages.len()))
}

/// `public` avec chaque fonctionnalité de développement doit échouer sur notre message (SEC0-20, E8).
pub fn profiles(cx: &Ctx) -> Outcome {
    for f in DEV_FEATURES {
        let features = format!("public,{f}");
        let args = [
            "check",
            "-p",
            "ts_kernel",
            "--locked",
            "--features",
            &features,
        ];
        println!("  $ cargo {}", args.join(" "));
        let (ok, out) = tri!(run_captured(&cx.root, &cargo(), &args));
        let message = format!("profil public : la fonctionnalité `{f}` est interdite");
        if ok || !out.contains(&message) {
            return Outcome::Fail(format!(
                "public,{f} n'échoue pas sur le compile_error! attendu"
            ));
        }
        println!("  échec attendu : « {message} »");
    }
    Outcome::Ok("4 combinaisons refusées".to_owned())
}

pub fn features(cx: &Ctx) -> Outcome {
    let meta = tri!(cx.meta());
    let mut errors = manifests::check_features(meta);
    for host in HOSTS.iter().filter(|h| meta.package(h).is_some()) {
        let args = [
            "tree",
            "-p",
            host,
            "--edges",
            "normal,features",
            "--locked",
            "-f",
            "{p} {f}",
        ];
        let tree = tri!(stdout_of(&cx.root, &cargo(), &args));
        errors.extend(
            manifests::tree_dev_features(&tree)
                .into_iter()
                .map(|l| format!("{host} : {l}")),
        );
    }
    from_errors(errors, "manifestes et graphe des hôtes".to_owned())
}

/// Exemptions de clippy hors des hôtes et des outils (E6, F5) ; `clippy.toml` hors de la racine.
pub fn clippy_exemptions(cx: &Ctx) -> Outcome {
    let meta = tri!(cx.meta());
    let mut errors = Vec::new();
    if sys::env("CLIPPY_CONF_DIR").is_some() {
        errors.push("CLIPPY_CONF_DIR remplace clippy.toml : refusé".to_owned());
    }
    let files = tri!(cx.files());
    for p in meta
        .packages
        .iter()
        .filter(|p| !TOOLS.contains(&p.name.as_str()) && !HOSTS.contains(&p.name.as_str()))
    {
        let dir = format!("{}/", meta.rel_dir(p));
        for rel in files
            .iter()
            .filter(|f| f.starts_with(&dir) && f.ends_with(".rs"))
        {
            let text = String::from_utf8_lossy(&tri!(cx.read(rel))).into_owned();
            errors.extend(
                scan::clippy_exemptions(&text)
                    .iter()
                    .map(|l| format!("{rel}:{l} : exemption interdite")),
            );
        }
    }
    let mut dirs: BTreeSet<String> = meta.packages.iter().map(|p| meta.rel_dir(p)).collect();
    dirs.extend(
        files
            .iter()
            .filter_map(|f| f.rsplit_once('/').map(|(d, _)| d.to_owned())),
    );
    for dir in dirs.iter().flat_map(|d| {
        d.match_indices('/')
            .map(|(i, _)| &d[..i])
            .chain([d.as_str()])
    }) {
        for name in ["clippy.toml", ".clippy.toml"] {
            if !dir.is_empty() && cx.root.join(dir).join(name).exists() {
                errors.push(format!(
                    "{dir}/{name} : configuration de clippy hors de la racine"
                ));
            }
        }
    }
    if cx.root.join(".clippy.toml").exists() {
        errors.push(".clippy.toml à la racine : seul clippy.toml est permis".to_owned());
    }
    from_errors(errors, String::new())
}

pub fn local_paths(cx: &Ctx) -> Outcome {
    // Variables de Windows nommées par F6 ; absentes en CI Linux, où les motifs fixes restent.
    let names: Vec<String> = ["USERNAME", "COMPUTERNAME"]
        .iter()
        .filter_map(|v| sys::env(v))
        .map(|v| v.to_ascii_lowercase())
        .filter(|v| v.len() >= 3)
        .collect();
    let mut errors = Vec::new();
    for rel in tri!(cx.files()) {
        let exempt = scan::LOCAL_PATH_EXEMPT.contains(&rel.as_str());
        let lines = scan::local_paths(&tri!(cx.read(rel)), &names, exempt);
        errors.extend(
            lines
                .iter()
                .map(|l| format!("{rel}:{l} : chemin, utilisateur ou machine locale")),
        );
    }
    from_errors(
        errors,
        format!("{} nom(s) local(aux) recherchés", names.len()),
    )
}

pub fn workflows(cx: &Ctx) -> Outcome {
    let files = tri!(cx.files());
    let mut errors = Vec::new();
    let mut count = 0;
    for rel in files.iter().filter(|f| f.starts_with(".github/workflows/")) {
        if !(rel.ends_with(".yml") || rel.ends_with(".yaml")) {
            errors.push(format!("{rel} : fichier inattendu dans .github/workflows/"));
            continue;
        }
        count += 1;
        let text = String::from_utf8_lossy(&tri!(cx.read(rel))).into_owned();
        errors.extend(
            workflows::check_workflow(&text)
                .into_iter()
                .map(|e| format!("{rel} : {e}")),
        );
    }
    match (count, errors.is_empty()) {
        (0, true) => Outcome::Skipped("aucun workflow".to_owned()),
        _ => from_errors(errors, format!("{count} workflow(s)")),
    }
}

pub fn spec_map(_: &Ctx) -> Outcome {
    println!("  AVERTISSEMENT : la sous-commande spec-map n'existe pas encore (T4).");
    Outcome::Skipped("spec-map absent (T4)".to_owned())
}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_tools_lock() {
        let lock =
            "# c\ncargo-deny.version = \"0.20.2\"\ngitleaks.sha256 = \"ab\"\nx.version = \"\"\n";
        assert_eq!(
            super::lock_value(lock, "cargo-deny.version").unwrap(),
            "0.20.2"
        );
        assert!(super::lock_value(lock, "x.version").is_err());
        assert!(super::lock_value(lock, "cargo-audit.version").is_err());
    }
}
