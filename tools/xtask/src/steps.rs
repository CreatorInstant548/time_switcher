//! Étapes de `cargo xtask check` qui lancent des outils ou lisent le dépôt.
//! spec: carnet architecture-securite.md SEC0-03, 06, 08, 19, 20, 26, 29 à 31 ; §9.5 F1 à F10

use std::cell::OnceCell;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::manifests::{self, DEV_FEATURES, HOSTS, TOOLS};
use crate::metadata::{self, Metadata};
use crate::sys::{self, cargo, run_captured, run_live, stdout_of};
use crate::{exemptions, scan, workflows};

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
/// Liens symboliques permis dans l'arbre (I3), en liste fermée : aucun pour l'instant.
const SYMLINK_EXEMPT: [&str; 0] = [];
/// Fichiers autres que `.rs` permis sous `<crate de cœur>/src` (I2), en liste fermée : aucun.
const NON_RS_SRC_EXEMPT: [&str; 0] = [];
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

    /// Chemins rendus par une commande Git lancée avec `-z` : séparés par NUL, jamais cités (I3).
    fn git_paths(&self, args: &[&str]) -> Result<Vec<String>, String> {
        Ok(split_nul(&self.git(args)?))
    }

    /// Fichiers suivis, plus non suivis et non ignorés (F2). Chacun doit être un fichier ordinaire
    /// présent sur le disque : un lien (hors `SYMLINK_EXEMPT`), un fichier suivi mais supprimé ou
    /// un autre objet font échouer toute étape qui lit l'arbre (I3).
    pub(crate) fn files(&self) -> Result<&[String], String> {
        let init = || {
            let files = self.git_paths(&["ls-files", "-z", "-co", "--exclude-standard"])?;
            let mut errors = Vec::new();
            for f in &files {
                match std::fs::symlink_metadata(self.root.join(f)) {
                    Ok(m) if m.file_type().is_file() => {}
                    Ok(m) if m.file_type().is_symlink() && SYMLINK_EXEMPT.contains(&f.as_str()) => {
                    }
                    Ok(m) if m.file_type().is_symlink() => {
                        errors.push(format!("{f} : lien refusé"))
                    }
                    Ok(_) => errors.push(format!("{f} : pas un fichier ordinaire")),
                    Err(_) => errors.push(format!("{f} : suivi mais absent du disque")),
                }
            }
            if errors.is_empty() {
                return Ok(files);
            }
            errors.iter().for_each(|e| println!("  - {e}"));
            Err(format!(
                "{} fichier(s) illisible(s) dans l'arbre",
                errors.len()
            ))
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
            // Sans renommage, un déplacement compte ses deux chemins ; -z : noms jamais cités.
            let diff = ["diff", "--name-only", "-z", "--no-renames"];
            let mut all = self.git_paths(&[&diff[..], &["origin/main...HEAD"]].concat())?;
            all.extend(self.git_paths(&[&diff[..], &["HEAD"]].concat())?);
            all.extend(self.git_paths(&["ls-files", "-z", "-o", "--exclude-standard"])?);
            Ok(all)
        };
        self.changed
            .get_or_init(init)
            .as_ref()
            .map(Vec::as_slice)
            .map_err(Clone::clone)
    }

    pub(crate) fn read(&self, rel: &str) -> Result<Vec<u8>, String> {
        sys::read(&self.root.join(rel))
    }
}

/// Découpe une sortie `-z` de Git : un chemin par champ NUL, sans citation ni échappement.
fn split_nul(out: &str) -> Vec<String> {
    out.split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect()
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
    if !tri!(run_live(&cx.root, &cargo(), &args)) {
        return Outcome::Fail("cargo clippy (profil public) a échoué".to_owned());
    }
    // Crates de cœur avec chaque fonctionnalité de développement, l'une après l'autre (I2) :
    // le code sous #[cfg(feature = "dev-…")] est vu aussi. Jamais --all-features (E3).
    let core: Vec<&str> = meta
        .packages
        .iter()
        .map(|p| p.name.as_str())
        .filter(|n| is_core(n))
        .collect();
    for f in DEV_FEATURES {
        let args = dev_clippy_args(&core, f);
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        if !tri!(run_live(&cx.root, &cargo(), &args)) {
            return Outcome::Fail(format!("cargo clippy (fonctionnalité {f}) a échoué"));
        }
    }
    Outcome::Ok(format!(
        "espace de travail, public, et {} fonctionnalités de développement",
        DEV_FEATURES.len()
    ))
}

/// Crate de cœur ou de règles : ni hôte, ni outil (refus par défaut).
fn is_core(name: &str) -> bool {
    !TOOLS.contains(&name) && !HOSTS.contains(&name)
}

/// `cargo clippy` des crates de cœur avec une fonctionnalité de développement de `ts_kernel`.
fn dev_clippy_args(core: &[&str], feature: &str) -> Vec<String> {
    let mut args = vec![
        "clippy".to_owned(),
        "--locked".to_owned(),
        "--all-targets".to_owned(),
    ];
    for c in core {
        args.extend(["-p".to_owned(), (*c).to_owned()]);
    }
    args.extend(["--features".to_owned(), format!("ts_kernel/{feature}")]);
    args.extend(["--", "-D", "warnings"].map(str::to_owned));
    args
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
    // Empreinte du jeu installé, écrite par xtask hooks install ; hooks verify recalcule le contenu.
    let current = sys::read_text(&hooks.join("current.sha256")).unwrap_or_default();
    let hex = name.len() == 64 && name.bytes().all(|b| b.is_ascii_hexdigit());
    if !same_parent || !hex || !current.trim().eq_ignore_ascii_case(&name) {
        return Err(
            "core.hooksPath ne désigne pas la copie épinglée TS_CACHE/hooks/<current.sha256> : \
             réinstaller les crochets (cargo xtask hooks install)"
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

/// gitleaks à lancer : binaire, empreinte attendue, configuration et dossier d'exceptions.
struct GitleaksSetup {
    bin: PathBuf,
    sha256: String,
    config: PathBuf,
    ignore_dir: PathBuf,
}

/// Mode CI (A2) : `CI=true` et `GITHUB_ACTIONS=true`. Un exécuteur n'a pas de copie épinglée des
/// crochets ; le mode CI n'est pas plus permissif : binaire vérifié, configuration de `main`.
fn ci_mode() -> bool {
    let is_true = |v: &str| sys::env(v).as_deref() == Some("true");
    is_true("CI") && is_true("GITHUB_ACTIONS")
}

/// Poste local : binaire, empreinte et configuration de la copie épinglée (F2).
fn gitleaks_local(cx: &Ctx) -> Result<GitleaksSetup, String> {
    let (dir, lock) = pinned_lock(cx)?;
    Ok(GitleaksSetup {
        bin: PathBuf::from(lock_value(&lock, "gitleaks.path")?),
        sha256: lock_value(&lock, "gitleaks.sha256")?,
        config: dir.join("gitleaks.toml"),
        ignore_dir: dir,
    })
}

/// Chemin relatif sans `..`, ni racine, ni préfixe de lecteur, quel que soit le système.
fn plain_relative(rel: &str) -> bool {
    let chars = rel
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b"._/-".contains(&b));
    !rel.is_empty()
        && chars
        && Path::new(rel)
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)))
}

/// Mode CI (A2, G5, G6) : binaire Linux décrit par les clés `gitleaks.linux.*` du `tools.lock`
/// du dépôt, à la même version que sous Windows, sous `TS_CACHE`. La configuration, placée à
/// côté par `tools/ci/install-tools.sh`, doit être octet pour octet celle d'`origin/main`
/// (`git show`) : une PR ne peut pas s'ajouter d'exception.
fn gitleaks_ci(cx: &Ctx) -> Result<GitleaksSetup, String> {
    let lock = sys::read_text(&cx.root.join("tools/dev/tools.lock"))?;
    let (linux, windows) = (
        lock_value(&lock, "gitleaks.linux.version")?,
        lock_value(&lock, "gitleaks.version")?,
    );
    if linux != windows {
        return Err(format!(
            "gitleaks.linux.version {linux} différente de gitleaks.version {windows} (G6)"
        ));
    }
    let rel = lock_value(&lock, "gitleaks.linux.path")?;
    if !plain_relative(&rel) {
        return Err("gitleaks.linux.path doit être relatif à TS_CACHE, sans ..".to_owned());
    }
    let bin = sys::ts_cache().join(&rel);
    let dir = bin
        .parent()
        .map(Path::to_path_buf)
        .ok_or("gitleaks.linux.path sans dossier")?;
    let config = dir.join("gitleaks.toml");
    let main = cx.git(&["show", "origin/main:tools/dev/hooks/gitleaks.toml"])?;
    if sys::read_text(&config)? != main {
        return Err(format!(
            "{} diffère de tools/dev/hooks/gitleaks.toml d'origin/main (G5)",
            config.display()
        ));
    }
    if dir.join(".gitleaksignore").exists() {
        return Err(".gitleaksignore à côté de gitleaks en CI refusé (SEC0-08)".to_owned());
    }
    Ok(GitleaksSetup {
        bin,
        sha256: lock_value(&lock, "gitleaks.linux.sha256")?,
        config,
        ignore_dir: dir,
    })
}

/// gitleaks épinglé (SHA-256 vérifié), configuration épinglée, `--redact` (F2, SEC0-08). En CI
/// (A2), binaire et configuration viennent de [`gitleaks_ci`] ; le reste est identique.
pub fn gitleaks(cx: &Ctx) -> Outcome {
    let setup = tri!(if ci_mode() {
        println!("  Mode CI : gitleaks.linux.* de tools.lock, configuration d'origin/main.");
        gitleaks_ci(cx)
    } else {
        gitleaks_local(cx)
    });
    let GitleaksSetup {
        bin,
        sha256,
        config,
        ignore_dir: dir,
    } = setup;
    let expected = sha256.to_ascii_lowercase();
    if !bin.is_file() || tri!(sys::sha256(&cx.root, &bin)) != expected {
        return Outcome::Fail(format!(
            "gitleaks absent ou d'empreinte inattendue ({})",
            bin.display()
        ));
    }
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
    for p in meta.packages.iter().filter(|p| is_core(&p.name)) {
        let dir = format!("{}/", meta.rel_dir(p));
        for rel in files
            .iter()
            .filter(|f| f.starts_with(&dir) && f.ends_with(".rs"))
        {
            let text = String::from_utf8_lossy(&tri!(cx.read(rel))).into_owned();
            errors.extend(
                exemptions::clippy_exemptions(&text)
                    .iter()
                    .map(|l| format!("{rel}:{l} : exemption ou code caché à clippy interdit")),
            );
        }
        // Un fichier non .rs sous src/ ne peut servir qu'à #[path] ou include! (I2).
        let src = format!("{dir}src/");
        for rel in files.iter().filter(|f| {
            f.starts_with(&src) && !f.ends_with(".rs") && !NON_RS_SRC_EXEMPT.contains(&f.as_str())
        }) {
            errors.push(format!(
                "{rel} : fichier autre que .rs sous src/ d'une crate de cœur"
            ));
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

/// `spec-map check` : renvois du code vers la conception et vers `origin/pilotage` (T4).
pub fn spec_map(cx: &Ctx) -> Outcome {
    let (errors, note) = tri!(crate::spec_map::check(cx));
    from_errors(errors, note)
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

    /// A2 : le binaire Linux de la CI reste sous `TS_CACHE`.
    #[test]
    fn ci_gitleaks_path_is_plain_relative() {
        assert!(super::plain_relative("ci/gitleaks/gitleaks"));
        for bad in [
            "",
            "/usr/bin/gitleaks",
            "../gitleaks",
            "ci/../../x",
            "C:/x/gitleaks",
        ] {
            assert!(!super::plain_relative(bad), "{bad}");
        }
    }

    /// v8 : le code sous `#[cfg(feature = "dev-…")]` est vu par une passe de clippy par
    /// fonctionnalité, nommée, jamais `--all-features`.
    #[test]
    fn dev_feature_clippy_passes() {
        for f in crate::manifests::DEV_FEATURES {
            let args = super::dev_clippy_args(&["ts_kernel", "ts_model"], f);
            let feature = format!("ts_kernel/{f}");
            assert!(
                args.windows(2)
                    .any(|w| w[0] == "--features" && w[1] == feature)
            );
            assert!(args.windows(2).any(|w| w[0] == "-p" && w[1] == "ts_model"));
            assert!(
                !args
                    .iter()
                    .any(|a| a == "--all-features" || a == "--workspace")
            );
        }
        assert!(super::is_core("ts_kernel") && super::is_core("ts_rules_x"));
        assert!(!super::is_core("xtask") && !super::is_core("ts_host"));
    }

    #[test]
    fn splits_git_nul_output() {
        let out = ".github/workflows/ci\"x.yml\0docs/a b é.md\0crates/k/src/\\t.rs\0";
        let paths = super::split_nul(out);
        assert_eq!(
            paths,
            [
                ".github/workflows/ci\"x.yml",
                "docs/a b é.md",
                "crates/k/src/\\t.rs"
            ]
        );
        assert!(super::split_nul("").is_empty());
    }
}
