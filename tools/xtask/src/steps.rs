//! Étapes de `cargo xtask check` qui lancent des outils ou lisent le dépôt.
//! spec: carnet architecture-securite.md SEC0-03, 06, 08, 19, 20, 26, 29 à 31 ; §9.5 F1 à F10

use std::cell::OnceCell;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::manifests::{self, DEV_FEATURES, HOSTS, TOOLS};
use crate::metadata::{self, Metadata};
use crate::sys::{self, cargo, run_captured, run_live, stdout_of};
use crate::{exemptions, macros, reserved, scan, workflows};

pub enum Outcome {
    Ok(String),
    Skipped(String),
    Fail(String),
}

/// Fichiers de la racine dont la modification vaut `--all` (F7).
pub(crate) const ROOT_FILES: [&str; 7] = [
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
        sys::set_root(&root);
        let (meta, files, changed) = (OnceCell::new(), OnceCell::new(), OnceCell::new());
        Self {
            root,
            all,
            meta,
            files,
            changed,
        }
    }

    pub(crate) fn meta(&self) -> Result<&Metadata, String> {
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
    pub(crate) fn changed(&self) -> Result<&[String], String> {
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

    /// Files modified since `HEAD`, or new (not ignored), and not committed (m11). The steps read
    /// the disk, not `HEAD`: with such files, the result may differ from what will be merged.
    pub(crate) fn uncommitted(&self) -> Result<Vec<String>, String> {
        let mut all = self.git_paths(&["diff", "--name-only", "-z", "--no-renames", "HEAD"])?;
        all.extend(self.git_paths(&["ls-files", "-z", "-o", "--exclude-standard"])?);
        all.sort();
        all.dedup();
        Ok(all)
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
        return Outcome::Fail("cargo clippy failed".to_owned());
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
        return Outcome::Fail("cargo clippy (public profile) failed".to_owned());
    }
    // Core crates with each development feature, one after the other (I2): the code under
    // #[cfg(feature = "dev-…")] is seen too, in the kernel and in every core crate that relays
    // the feature (#36). Never --all-features (E3).
    let core = core_crates(meta);
    for f in DEV_FEATURES {
        let args = dev_clippy_args(&core, &relays(meta, &core, f), f);
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        if !tri!(run_live(&cx.root, &cargo(), &args)) {
            return Outcome::Fail(format!("cargo clippy (feature {f}) failed"));
        }
    }
    let live = tri!(live_bans(cx));
    Outcome::Ok(format!(
        "workspace, public, {} development features, {live} live bans",
        DEV_FEATURES.len()
    ))
}

/// An entry of `clippy.toml` in the one layout the file uses, a whole entry on one line:
/// `{ path = "P", reason = "R" },` with or without `, allow-invalid = true` after the reason.
struct Entry<'a> {
    path: &'a str,
    reason: &'a str,
    /// The entry has `allow-invalid = true`.
    lenient: bool,
    /// What follows the entry: `,` or nothing.
    tail: &'a str,
}

/// Reads a line in that layout. `P` is plain (`[A-Za-z0-9_:]`) and `R` has no quote or escape.
fn entry(line: &str) -> Option<Entry<'_>> {
    let rest = line.trim().strip_prefix("{ path = \"")?;
    let (path, rest) = rest.split_once('"')?;
    let rest = rest.strip_prefix(", reason = \"")?;
    let (reason, rest) = rest.split_once('"')?;
    let (lenient, rest) = match rest.strip_prefix(", allow-invalid = true }") {
        Some(tail) => (true, tail),
        None => (false, rest.strip_prefix(" }")?),
    };
    let plain = !path.is_empty()
        && path
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b':');
    (plain && !reason.contains('\\') && matches!(rest, "" | ",")).then_some(Entry {
        path,
        reason,
        lenient,
        tail: rest,
    })
}

/// Crates whose banned paths are internal locations that an upgrade moves (the hash collections of
/// bevy): their entries must resolve. The others (`rand`, `getrandom`, `chrono`) ban the public API
/// of several versions at once, so one of two alternative paths is always unresolved.
const STRICT_CRATE_PREFIX: &str = "bevy_";

/// `clippy.toml` with `allow-invalid = true` removed from every entry of a bevy crate that is in
/// `closure` (m1 of the review of PR #33): such an entry must resolve, so one that became dead after
/// a dependency upgrade is a warning. The other entries keep the flag. Fails closed: a line that
/// carries `allow-invalid` in any other layout than [`entry`] (no blanks around `=`,
/// several lines, single quotes, an escaped crate name, two entries on a line, a flag on a line
/// of its own) is an error, never skipped; so are a backslash outside comments, a multi-line
/// string and a lenient entry on a bevy crate outside `closure`. Returns the text and the number
/// of entries made strict.
fn strict_clippy_config(text: &str, closure: &BTreeSet<String>) -> Result<(String, usize), String> {
    let (mut out, mut strict) = (String::new(), 0);
    for line in text.lines() {
        let comment = line.trim_start().starts_with('#');
        // In a multi-line string a line starting with `#` is text, not a comment: none may open,
        // so that every `#` line is a real comment (review of #114).
        if !comment && (line.contains("\"\"\"") || line.contains("'''")) {
            return Err(format!(
                "clippy.toml: multi-line string refused: {}",
                line.trim()
            ));
        }
        // An escape can spell any key (`"allow\\u002dinvalid"`, `"allow\\x2dinvalid"` in TOML
        // 1.1): no backslash at all outside comments (review of #114).
        if !comment && line.contains('\\') {
            return Err(format!(
                "clippy.toml: backslash refused outside comments: {}",
                line.trim()
            ));
        }
        // Only a line with the flag is rewritten, and it must be in the one layout; a line
        // without it (an entry already strict, possibly spread over lines) is left as it is.
        let line = if line.trim_start().starts_with('#') || !line.contains("allow-invalid") {
            line.to_owned()
        } else {
            let Some(e) = entry(line) else {
                return Err(format!(
                    "clippy.toml: entry layout not understood: {}",
                    line.trim()
                ));
            };
            let krate = e.path.split("::").next().unwrap_or(e.path);
            let bevy = krate.starts_with(STRICT_CRATE_PREFIX);
            // A bevy crate the kernel does not depend on (a typo in the name) would keep its
            // flag, and the ban stay dead without a word (review of #114).
            if e.lenient && bevy && !closure.contains(krate) {
                return Err(format!(
                    "clippy.toml: {krate} is not in the dependency graph of ts_kernel, so its \
                     ban cannot be checked (a typo, or extend kernel_closure): {}",
                    line.trim()
                ));
            }
            if e.lenient && bevy {
                strict += 1;
                let indent = &line[..line.len() - line.trim_start().len()];
                format!(
                    "{indent}{{ path = \"{}\", reason = \"{}\" }}{}",
                    e.path, e.reason, e.tail
                )
            } else {
                line.to_owned()
            }
        };
        out.push_str(&line);
        out.push('\n');
    }
    Ok((out, strict))
}

/// Normal dependencies of `ts_kernel`, whole graph, as library names (`bevy_ecs`).
fn kernel_closure(cx: &Ctx) -> Result<BTreeSet<String>, String> {
    let args = [
        "tree",
        "-p",
        "ts_kernel",
        "-e",
        "normal",
        "--prefix",
        "none",
        "--locked",
    ];
    let out = stdout_of(&cx.root, &cargo(), &args)?;
    let names: BTreeSet<String> = out
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(|n| n.replace('-', "_"))
        .collect();
    // The root of the tree is the crate itself: an empty or unreadable listing is an error.
    match names.contains("ts_kernel") {
        true => Ok(names),
        false => Err("cargo tree -p ts_kernel did not list ts_kernel".to_owned()),
    }
}

/// The bans of `clippy.toml` on crates the kernel depends on still resolve (m1 of the review of
/// PR #33): clippy runs on `ts_kernel` with `allow-invalid` removed from those entries, from a
/// copy of the configuration under `TS_CACHE`, set for that run only (`CLIPPY_CONF_DIR`). Without
/// it, a ban made dead by a dependency upgrade is silent. Returns the number of entries checked.
fn live_bans(cx: &Ctx) -> Result<usize, String> {
    let text = String::from_utf8(cx.read("clippy.toml")?)
        .map_err(|_| "clippy.toml is not UTF-8".to_owned())?;
    let (strict, count) = strict_clippy_config(&text, &kernel_closure(cx)?)?;
    let scratch = sys::Scratch::new("clippy-config")?;
    let dir = scratch.path("conf");
    std::fs::create_dir(&dir).map_err(|e| format!("{}: {e}", sys::shown(&dir)))?;
    sys::write(&dir.join("clippy.toml"), strict.as_bytes())?;
    let conf = dir.to_string_lossy().into_owned();
    // No colour: CI forces it, and the alarm lines are recognised by their first word.
    let args = [
        "clippy",
        "--color",
        "never",
        "--locked",
        "-p",
        "ts_kernel",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ];
    println!(
        "  $ cargo {} (CLIPPY_CONF_DIR: strict copy of clippy.toml)",
        args.join(" ")
    );
    let env = [
        ("CLIPPY_CONF_DIR", conf.as_str()),
        ("CARGO_TERM_COLOR", "never"),
    ];
    let (ok, out) = sys::run_captured_env(&cx.root, &cargo(), &args, &env)?;
    // A ban that does not resolve is a plain warning, which `-D warnings` does not turn into a
    // failure: any `warning` or `error` line is one (a clean run prints none).
    let alarms: Vec<&str> = out
        .lines()
        .filter(|l| l.starts_with("warning") || l.starts_with("error"))
        .collect();
    alarms.iter().for_each(|l| println!("  {l}"));
    match (ok, alarms.is_empty()) {
        (true, true) => Ok(count),
        (false, true) => {
            Err("cargo clippy failed on ts_kernel with the strict copy of clippy.toml".to_owned())
        }
        _ => Err(
            "a clippy.toml ban on a bevy crate no longer resolves (allow-invalid removed \
                  for this run): update the path, or the ban is dead"
                .to_owned(),
        ),
    }
}

/// Crate de cœur ou de règles : ni hôte, ni outil (refus par défaut).
fn is_core(name: &str) -> bool {
    !TOOLS.contains(&name) && !HOSTS.contains(&name)
}

/// The core and rule crates of the workspace.
fn core_crates(meta: &Metadata) -> Vec<&str> {
    meta.packages
        .iter()
        .map(|p| p.name.as_str())
        .filter(|n| is_core(n))
        .collect()
}

/// The core crates other than `ts_kernel` that declare the development feature `feature`, a relay
/// of the kernel's (`dev-commands = ["ts_kernel/dev-commands"]`, #36).
fn relays<'a>(meta: &Metadata, core: &[&'a str], feature: &str) -> Vec<&'a str> {
    core.iter()
        .copied()
        .filter(|c| *c != "ts_kernel")
        .filter(|c| {
            meta.package(c)
                .is_some_and(|p| p.features.contains_key(feature))
        })
        .collect()
}

/// `--features` value that turns `feature` on in the kernel and in each relaying crate.
fn dev_features_value(relays: &[&str], feature: &str) -> String {
    std::iter::once("ts_kernel")
        .chain(relays.iter().copied())
        .map(|c| format!("{c}/{feature}"))
        .collect::<Vec<_>>()
        .join(",")
}

/// `cargo clippy` of the core crates with a development feature of `ts_kernel`, and the same
/// feature of the crates that relay it (#36).
fn dev_clippy_args(core: &[&str], relays: &[&str], feature: &str) -> Vec<String> {
    let mut args = vec![
        "clippy".to_owned(),
        "--locked".to_owned(),
        "--all-targets".to_owned(),
    ];
    for c in core {
        args.extend(["-p".to_owned(), (*c).to_owned()]);
    }
    args.extend(["--features".to_owned(), dev_features_value(relays, feature)]);
    args.extend(["--", "-D", "warnings"].map(str::to_owned));
    args
}

/// `cargo test` of the crates that relay a development feature, with it (#36): their gated unit
/// and doc tests. Each relay turns the kernel's feature on itself. The kernel's own gated code is
/// small and partly compiled under `cfg(test)`; four extra kernel test runs would cost more than
/// they find (decided).
fn dev_test_args(relays: &[&str], feature: &str) -> Vec<String> {
    let mut args = vec!["test".to_owned(), "--locked".to_owned()];
    for c in relays {
        args.extend(["-p".to_owned(), (*c).to_owned()]);
    }
    let value: Vec<String> = relays.iter().map(|c| format!("{c}/{feature}")).collect();
    args.extend(["--features".to_owned(), value.join(",")]);
    args
}

pub fn test(cx: &Ctx) -> Outcome {
    // The crates tested, `None` for the whole workspace.
    let mut crates: Option<BTreeSet<String>> = None;
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
        let selected = crates.insert(meta.with_dependents(touched));
        if selected.is_empty() {
            return Outcome::Skipped("no crate touched".to_owned());
        }
        selected
            .iter()
            .for_each(|c| args.extend(["-p", c.as_str()]));
    } else {
        args.push("--workspace");
    }
    if !tri!(run_live(&cx.root, &cargo(), &args)) {
        return Outcome::Fail("cargo test failed".to_owned());
    }
    // #36: the crates that relay a development feature, tested with it, when they are tested.
    let meta = tri!(cx.meta());
    let core = core_crates(meta);
    let tested = |c: &&str| crates.as_ref().is_none_or(|s| s.contains(*c));
    let mut passes = 0;
    for f in DEV_FEATURES {
        let relays: Vec<&str> = relays(meta, &core, f).into_iter().filter(tested).collect();
        if relays.is_empty() {
            continue;
        }
        let extra = dev_test_args(&relays, f);
        let extra: Vec<&str> = extra.iter().map(String::as_str).collect();
        if !tri!(run_live(&cx.root, &cargo(), &extra)) {
            return Outcome::Fail(format!("cargo test (feature {f}) failed"));
        }
        passes += 1;
    }
    Outcome::Ok(format!("cargo test, {passes} development feature pass(es)"))
}

pub(crate) fn lock_value(lock: &str, key: &str) -> Result<String, String> {
    lock.lines()
        .filter_map(|l| l.split_once('='))
        .find(|(k, _)| k.trim() == key)
        .map(|(_, v)| v.trim().trim_matches('"').to_owned())
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("{key} absent de tools.lock"))
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

/// gitleaks to run: binary, expected SHA-256, configuration and folder of exceptions.
struct GitleaksSetup {
    bin: PathBuf,
    sha256: String,
    config: PathBuf,
    ignore_dir: PathBuf,
}

/// Mode CI (A2) : `CI=true` et `GITHUB_ACTIONS=true`. Le mode CI n'est pas plus permissif :
/// binaire vérifié, configuration de `main`.
pub fn ci_mode() -> bool {
    let is_true = |v: &str| sys::env(v).as_deref() == Some("true");
    is_true("CI") && is_true("GITHUB_ACTIONS")
}

/// Local machine (AS-50: no more pinned hook copy): binary and SHA-256 from `tools.lock`, and
/// configuration from `gitleaks.toml`, both read from `origin/main` by `git show`, never from the
/// working copy, so a branch cannot point at another binary nor add an exception (F2, G5). The
/// configuration is written to the `config` folder of this run's scratch folder.
fn gitleaks_local(cx: &Ctx, scratch: &sys::Scratch) -> Result<GitleaksSetup, String> {
    let lock = cx.git(&["show", "origin/main:tools/dev/tools.lock"])?;
    let config = cx.git(&["show", "origin/main:tools/dev/hooks/gitleaks.toml"])?;
    let (bin, sha256) = (
        PathBuf::from(lock_value(&lock, "gitleaks.path")?),
        lock_value(&lock, "gitleaks.sha256")?,
    );
    let dir = scratch.path("config");
    std::fs::create_dir(&dir).map_err(|e| format!("{}: {e}", sys::shown(&dir)))?;
    sys::write(&dir.join("gitleaks.toml"), config.as_bytes())?;
    Ok(GitleaksSetup {
        bin,
        sha256,
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
            "{} differs from tools/dev/hooks/gitleaks.toml of origin/main (G5)",
            sys::shown(&config)
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

/// A finding of gitleaks' JSON report. Only these fields are read: `Match` and `Secret` are
/// redacted by `--redact`, and the author, e-mail and message of a commit are never shown.
#[derive(serde::Deserialize)]
struct Finding {
    #[serde(rename = "RuleID")]
    rule: String,
    #[serde(rename = "File")]
    file: String,
    #[serde(rename = "StartLine")]
    line: u64,
    #[serde(rename = "Commit", default)]
    commit: String,
}

/// One line per finding of a gitleaks JSON report: rule, file, line and abbreviated commit, never
/// the secret (m5). `scan` is the folder that was scanned: the report gives absolute paths for it.
fn findings(report: &str, scan: Option<&Path>) -> Result<Vec<String>, String> {
    let list: Vec<Finding> =
        serde_json::from_str(report).map_err(|e| format!("gitleaks report unreadable: {e}"))?;
    // Paths of the report, normalised to `/`; a file under the scanned folder is shown relative to it.
    let relative = |file: &str| {
        let file = file.replace('\\', "/");
        let under = scan.and_then(|s| sys::relative_to(Path::new(&file), s));
        match (under, Path::new(&file).is_absolute()) {
            (Some(rel), _) => rel,
            (None, true) => sys::shown(Path::new(&file)),
            (None, false) => file,
        }
    };
    Ok(list
        .iter()
        .map(|f| {
            let commit: String = f.commit.chars().take(12).collect();
            let at = if commit.is_empty() {
                String::new()
            } else {
                format!(" (commit {commit})")
            };
            format!("{} at {}:{}{at}", f.rule, relative(&f.file), f.line)
        })
        .collect())
}

/// Runs `gitleaks <sub> <common> --report-path <report> <tail>`, then reads the JSON report: the
/// findings, or an error when the tool failed without one (never a silent success).
fn run_gitleaks(
    cx: &Ctx,
    bin: &str,
    sub: &str,
    common: &[&str],
    report: &Path,
    tail: &[&str],
) -> Result<Vec<String>, String> {
    let path = report.to_string_lossy().into_owned();
    let extra = ["--report-format", "json", "--report-path", path.as_str()];
    let args = [&[sub][..], common, &extra, tail].concat();
    let ok = run_live(&cx.root, bin, &args)?;
    let text = sys::read_text(report).map_err(|_| "gitleaks wrote no report".to_owned())?;
    let scan = (sub == "dir").then(|| Path::new(tail.last().copied().unwrap_or_default()));
    let found = findings(&text, scan)?;
    match (ok, found.is_empty()) {
        (false, true) => Err("gitleaks failed without reporting a finding".to_owned()),
        _ => Ok(found),
    }
}

fn show_findings(found: &[String]) {
    found.iter().take(50).for_each(|f| println!("  - {f}"));
    if found.len() > 50 {
        println!("  - ... and {} more", found.len() - 50);
    }
}

/// Pinned gitleaks (SHA-256 checked), pinned configuration, `--redact` (F2, SEC0-08). In CI
/// (A2), binary and configuration come from [`gitleaks_ci`]; the rest is the same. Findings are
/// listed from the JSON report, by rule, file and line.
pub fn gitleaks(cx: &Ctx) -> Outcome {
    let scratch = tri!(sys::Scratch::new("gitleaks"));
    let setup = tri!(if ci_mode() {
        println!("  CI mode: gitleaks.linux.* of tools.lock, configuration of origin/main.");
        gitleaks_ci(cx)
    } else {
        gitleaks_local(cx, &scratch)
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
            "gitleaks missing or with an unexpected SHA-256 ({})",
            sys::shown(&bin)
        ));
    }
    if !config.is_file() {
        return Outcome::Fail("gitleaks.toml of origin/main missing".to_owned());
    }
    if cx.root.join(".gitleaksignore").exists() {
        return Outcome::Fail(".gitleaksignore at the root refused (SEC0-08)".to_owned());
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

    let scan = scratch.path("tree");
    tri!(extract(cx, &scan));
    let target = scan.to_string_lossy().into_owned();
    let tree_report = scratch.path("tree.json");
    let tree = tri!(run_gitleaks(
        cx,
        &bin,
        "dir",
        &common,
        &tree_report,
        &[&target]
    ));
    if !tree.is_empty() {
        show_findings(&tree);
        return Outcome::Fail(format!(
            "gitleaks found {} probable secret(s) in the tree",
            tree.len()
        ));
    }
    tri!(cx.git(&["rev-parse", "--verify", "--quiet", "origin/main"]));
    let range = "--log-opts=--text --no-ext-diff --no-textconv origin/main..HEAD";
    let git_report = scratch.path("commits.json");
    let commits = tri!(run_gitleaks(
        cx,
        &bin,
        "git",
        &common,
        &git_report,
        &[range, "."]
    ));
    if !commits.is_empty() {
        show_findings(&commits);
        return Outcome::Fail(format!(
            "gitleaks found {} probable secret(s) in the commits",
            commits.len()
        ));
    }
    Outcome::Ok("tree and commits since origin/main".to_owned())
}

/// Copy of the ordinary files of the tree, like the old `pre-commit`: no attribute hides their
/// content; no `target/` and no link. A `.gitleaksignore` is renamed. `scan` is created here,
/// exclusively, inside the run's scratch folder.
fn extract(cx: &Ctx, scan: &Path) -> Result<(), String> {
    std::fs::create_dir(scan).map_err(|e| format!("{}: {e}", sys::shown(scan)))?;
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
        copied.map_err(|e| format!("cannot copy {rel}: {e}"))?;
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
            errors.push(format!("{name} ({}): {e}", sys::shown(Path::new(src))));
        }
    }
    // ts_py: every attribute naming unsafe_code, in any .rs file, follows a // SAFETY: line, and
    // no file brings in another one (`include!`, `#[path]`) that this scan would not read (#91).
    if let Some(py) = meta.package(manifests::LOCAL_LINTS_CRATE) {
        let dir = format!("{}/", meta.rel_dir(py));
        for rel in tri!(cx.files())
            .iter()
            .filter(|f| f.starts_with(&dir) && f.ends_with(".rs"))
        {
            let text = String::from_utf8_lossy(&tri!(cx.read(rel))).into_owned();
            if let Some(line) = manifests::unsafe_allow_without_safety(&text) {
                errors.push(format!(
                    "{rel}:{line}: unsafe_code named outside the crate attribute without a \
                     // SAFETY: line just before"
                ));
            }
            for line in exemptions::file_inclusions(&text) {
                errors.push(format!(
                    "{rel}:{line}: include!, include_str!, include_bytes! or #[path] refused in \
                     {}: the unsafe_code scan reads only its .rs files",
                    manifests::LOCAL_LINTS_CRATE
                ));
            }
        }
    }
    // F3: inherited lints everywhere; local tables equal to the workspace's in ts_py (AS-18).
    let root_manifest = tri!(sys::read_text(&cx.root.join("Cargo.toml")));
    for p in &meta.packages {
        let check = |t: String| {
            if p.name == manifests::LOCAL_LINTS_CRATE {
                manifests::check_local_lints(&t, &root_manifest)
            } else {
                manifests::check_lints_section(&t)
            }
        };
        if let Err(e) = sys::read_text(Path::new(&p.manifest_path)).and_then(check) {
            errors.push(format!("{} : {e}", p.name));
        }
    }
    from_errors(errors, format!("{} crate(s)", meta.packages.len()))
}

/// Message of the `compile_error!` raised by `ts_kernel` for `public` together with the
/// development feature `f` (`crates/ts_kernel/src/build_profile.rs`), searched for word for word
/// by [`profiles`]. A test checks that the kernel still carries it.
fn profile_error(f: &str) -> String {
    format!("public profile: feature `{f}` is forbidden (SEC0-20, M28)")
}

/// `public` with each development feature must fail on our own message (SEC0-20, E8).
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
        let message = profile_error(f);
        if ok || !out.contains(&message) {
            // Error lines without a path separator only (SEC0-26: no local path in the output).
            out.lines()
                .filter(|l| l.starts_with("error") && !l.contains(['/', '\\']))
                .for_each(|l| println!("  {l}"));
            return Outcome::Fail(format!(
                "public,{f} does not fail on the expected compile_error!"
            ));
        }
        println!("  expected failure: \"{message}\"");
    }
    Outcome::Ok(format!("{} combinations refused", DEV_FEATURES.len()))
}

/// `cargo tree` of a host: normal edges and their features, for every target, so that a
/// dependency or a feature that only a non-host platform activates is seen too (m1).
fn tree_args(host: &str) -> Vec<String> {
    [
        "tree",
        "-p",
        host,
        "--edges",
        "normal,features",
        "--target",
        "all",
        "--locked",
        "-f",
        "{p} {f}",
    ]
    .map(str::to_owned)
    .to_vec()
}

pub fn features(cx: &Ctx) -> Outcome {
    let meta = tri!(cx.meta());
    let mut errors = manifests::check_features(meta);
    // E7 : ni [patch] ni [replace], à la racine comme dans chaque crate.
    let root = cx.root.join("Cargo.toml");
    let manifests = std::iter::once(root).chain(
        meta.packages
            .iter()
            .map(|p| PathBuf::from(&p.manifest_path)),
    );
    for m in manifests {
        let text = tri!(sys::read_text(&m));
        errors.extend(
            manifests::patch_sections(&text)
                .into_iter()
                .map(|l| format!("{}: {l} forbidden (E7)", sys::shown(&m))),
        );
    }
    for host in HOSTS.iter().filter(|h| meta.package(h).is_some()) {
        let args = tree_args(host);
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
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
/// Also the kernel items reserved to some crates, and the raw primitives (#26 m3, m4).
pub fn clippy_exemptions(cx: &Ctx) -> Outcome {
    let meta = tri!(cx.meta());
    let mut errors = Vec::new();
    if sys::env("CLIPPY_CONF_DIR").is_some() {
        errors.push("CLIPPY_CONF_DIR remplace clippy.toml : refusé".to_owned());
    }
    let files = tri!(cx.files());
    let mut read = 0;
    for p in meta.packages.iter().filter(|p| is_core(&p.name)) {
        let dir = format!("{}/", meta.rel_dir(p));
        // Every `macro_rules!` of the crate: file relative to the crate, line, name (AS-56).
        let mut macros = Vec::new();
        let sources: Vec<&String> = files
            .iter()
            .filter(|f| f.starts_with(&dir) && f.ends_with(".rs"))
            .collect();
        // m7: a core crate none of whose files is read would pass without being looked at.
        if sources.is_empty() {
            errors.push(format!("{}: no .rs file read under {dir}", p.name));
        }
        read += sources.len();
        for rel in sources {
            let text = String::from_utf8_lossy(&tri!(cx.read(rel))).into_owned();
            macros.extend(
                macros::definitions(&text)
                    .into_iter()
                    .map(|(line, name)| (rel[dir.len()..].to_owned(), line, name)),
            );
            errors.extend(
                exemptions::clippy_exemptions(&text)
                    .iter()
                    .map(|l| format!("{rel}:{l} : exemption ou code caché à clippy interdit")),
            );
            // #26 m3, m4: what clippy cannot ban in the core, which may not exempt itself.
            errors.extend(
                reserved::reserved_uses(rel, &text)
                    .into_iter()
                    .map(|(l, ident, why)| format!("{rel}:{l}: {ident} is reserved: {why}")),
            );
        }
        for d in p
            .dependencies
            .iter()
            .filter(|d| d.kind.as_deref() != Some("dev"))
        {
            if let Some(why) = reserved::refused_dependency(&p.name, &d.name) {
                errors.push(format!("{}: dependency on {}: {why}", p.name, d.name));
            }
        }
        errors.extend(macros::errors(&p.name, &macros));
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
    from_errors(errors, format!("{read} core source file(s) read"))
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
    let mut flows = Vec::new();
    for rel in files.iter().filter(|f| f.starts_with(".github/workflows/")) {
        if !(rel.ends_with(".yml") || rel.ends_with(".yaml")) {
            errors.push(format!("{rel} : fichier inattendu dans .github/workflows/"));
            continue;
        }
        let text = String::from_utf8_lossy(&tri!(cx.read(rel))).into_owned();
        errors.extend(
            workflows::check_workflow(&text)
                .into_iter()
                .map(|e| format!("{rel} : {e}")),
        );
        flows.push((rel.clone(), text));
    }
    let count = flows.len();
    if count > 0 {
        // Every installer call needs its pins in tools.lock (#75), read by the script itself (#95).
        let mut check = |args: &[&str]| installer_check(&cx.root, args);
        errors.extend(workflows::tool_lock_errors(&flows, &mut check));
    }
    match (count, errors.is_empty()) {
        (0, true) => Outcome::Skipped("aucun workflow".to_owned()),
        _ => from_errors(errors, format!("{count} workflow(s)")),
    }
}

/// `install-tools.sh check <args>` in `root` (#95): the script reads its own `tools.lock` values
/// and installs nothing. Its reason is its last line of output.
pub(crate) fn installer_check(root: &Path, args: &[&str]) -> Result<(), String> {
    let bash = sys::bash(root)?;
    let all = [&[workflows::INSTALL_SCRIPT, "check"], args].concat();
    let (ok, out) = run_captured(root, &bash, &all)?;
    if ok {
        return Ok(());
    }
    let reason = out.lines().rfind(|l| !l.trim().is_empty());
    Err(reason
        .map_or("failed without a message", |l| {
            l.trim_start_matches("FAILED (install-tools): ")
        })
        .to_owned())
}

/// `spec-map check` : renvois du code vers la conception et vers `origin/pilotage` (T4).
pub fn spec_map(cx: &Ctx) -> Outcome {
    let (errors, note) = tri!(crate::spec_map::check(cx));
    from_errors(errors, note)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

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
            let args = super::dev_clippy_args(&["ts_kernel", "ts_model"], &["ts_model"], f);
            // #36: the relay of ts_model is turned on with the kernel's feature.
            let feature = format!("ts_kernel/{f},ts_model/{f}");
            assert!(
                args.windows(2)
                    .any(|w| w[0] == "--features" && w[1] == feature)
            );
            let test = super::dev_test_args(&["ts_model"], f);
            assert_eq!(
                test.join(" "),
                format!("test --locked -p ts_model --features ts_model/{f}")
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

    /// m1: the host graph is read for every target.
    #[test]
    fn host_tree_covers_every_target() {
        let args = super::tree_args("ts_host");
        assert!(args.windows(2).any(|w| w[0] == "--target" && w[1] == "all"));
        assert!(args.windows(2).any(|w| w[0] == "-p" && w[1] == "ts_host"));
        assert!(args.iter().any(|a| a == "--locked"));
    }

    /// m5: a finding is listed by rule, file and line, with a short commit, and nothing else of
    /// the report (secret, author, e-mail, message) is shown.
    #[test]
    fn lists_gitleaks_findings_without_the_secret() {
        let report = r#"[
          {"RuleID":"github-pat","Description":"d","StartLine":3,"EndLine":3,"Match":"REDACTED",
           "Secret":"REDACTED","File":"C:\\scratch\\tree\\src\\a.rs","Commit":"",
           "Author":"Someone","Email":"a@b.c","Message":"msg"},
          {"RuleID":"aws-access-token","StartLine":9,"File":"b/c.txt",
           "Commit":"0123456789abcdef0123","Author":"Someone","Email":"a@b.c"}]"#;
        let scan = Path::new("C:/scratch/tree");
        let lines = super::findings(report, Some(scan)).unwrap();
        assert_eq!(
            lines,
            [
                "github-pat at src/a.rs:3",
                "aws-access-token at b/c.txt:9 (commit 0123456789ab)"
            ]
        );
        assert!(
            lines
                .iter()
                .all(|l| !l.contains("Someone") && !l.contains('@'))
        );
        assert!(super::findings("[]", None).unwrap().is_empty());
        assert!(super::findings("not json", None).is_err());
    }

    /// m1 (PR #33): `allow-invalid` is removed only from the entries of crates in the closure.
    #[test]
    fn strict_clippy_config_follows_the_closure() {
        let text = "allow-unwrap-in-tests = true\n\
            disallowed-types = [\n\
            \x20   { path = \"bevy_ecs::entity::EntityHashMap\", reason = \"r\", allow-invalid = true },\n\
            \x20   { path = \"rand::rng\", reason = \"r\", allow-invalid = true },\n\
            \x20   { path = \"std::collections::HashMap\", reason = \"r\" },\n\
            ]\n";
        let closure: BTreeSet<String> = ["bevy_ecs".to_owned()].into();
        let (out, strict) = super::strict_clippy_config(text, &closure).unwrap();
        assert_eq!(strict, 1);
        assert!(out.contains("{ path = \"bevy_ecs::entity::EntityHashMap\", reason = \"r\" },"));
        assert!(out.contains("{ path = \"rand::rng\", reason = \"r\", allow-invalid = true },"));
        assert_eq!(out.lines().count(), text.lines().count());
        // An entry of the closure that cannot be read is refused, never skipped (review of #114).
        for odd in [
            "{ allow-invalid = true, path = \"bevy_ecs::X\", reason = \"r\" },",
            "{ path = \"bevy_ecs::X\", reason = \"r\", allow-invalid=true },",
            "{path = \"bevy_ecs::X\", reason = \"r\", allow-invalid = true},",
            "{ path=\"bevy_ecs::X\", reason = \"r\", allow-invalid = true },",
            "{ path = \"bevy_ecs::X\", reason = 'r', allow-invalid = true },",
            "{ path = 'bevy_ecs::X', reason = \"r\", allow-invalid = true },",
            "{ path = \"bevy_ecs::X\", reason = \"r\", allow-invalid = true }, # note",
            "{ path = \"\\u0062evy_ecs::X\", reason = \"r\", allow-invalid = true },",
            "{ path = \"bevy_ecs::X\", allow-invalid = true },",
            "{ path = \"bevy_ecs::X\",\n  reason = \"r\", allow-invalid = true },",
            "{ path = \"bevy_ecs::X\", reason = \"r\",\n  allow-invalid = true },",
            "    allow-invalid = true,",
            "[[disallowed-types]]\npath = \"bevy_ecs::X\"\nallow-invalid = true",
            "{ path = \"bevy_ecs::X\", reason = \"r \\\" q\", allow-invalid = true },",
            "{ path = \"bevy_ecs::X\", reason = \"r\", \"allow\\u002dinvalid\" = true },",
            "{ path = \"bevy_ecs::X\", reason = \"r\", \"\\U00000061llow-invalid\" = true },",
            "{ path = \"bevy_ecs::X\", reason = \"caf\\u00e9\" },",
        ] {
            assert!(super::strict_clippy_config(odd, &closure).is_err(), "{odd}");
        }
        // Review of #114: any backslash outside comments (`\x2d` of TOML 1.1 included), a
        // multi-line string (its `#` lines are text), and a bevy crate outside the closure.
        for odd in [
            "{ path = \"bevy_ecs::X\", reason = \"r\", \"allow\\x2dinvalid\" = true },",
            "{ path = \"rand::rng\", reason = \"a \\\\ b\" },",
            "x = \"\"\"\n# { path = \"bevy_ecs::X\", reason = \"r\", allow-invalid = true },\n\"\"\"",
            "x = '''\n# y\n'''",
            "{ path = \"bevy_ecz::X\", reason = \"r\", allow-invalid = true },",
        ] {
            assert!(super::strict_clippy_config(odd, &closure).is_err(), "{odd}");
        }
        for fine in [
            "# a comment with a \\ backslash",
            "# never write '''raw''' strings here",
            "{ path = \"bevy_ecz::X\", reason = \"r\" },",
            "{ path = \"rand::rng\", reason = \"r\", allow-invalid = true },",
        ] {
            assert!(
                super::strict_clippy_config(fine, &closure).is_ok(),
                "{fine}"
            );
        }
        // Whatever the crate: a flag in a layout that cannot be read is an error too.
        assert!(
            super::strict_clippy_config("{path=\"rand::rng\", allow-invalid=true},", &closure)
                .is_err()
        );
        // A strict (flag-free) entry may span lines, and other keys are left alone.
        let multi = "x = 1\n{ path = \"bevy_ecs::X\",\n  reason = \"r\" },\n";
        assert_eq!(
            super::strict_clippy_config(multi, &closure).unwrap().0,
            multi
        );
        // The real file: its entries on bevy crates all become strict.
        let real = include_str!("../../../clippy.toml");
        let all: BTreeSet<String> = [
            "bevy_ecs",
            "bevy_utils",
            "bevy_platform",
            "rand",
            "hashbrown",
        ]
        .map(str::to_owned)
        .into();
        let (out, count) = super::strict_clippy_config(real, &all).unwrap();
        assert!(count > 0);
        // Every bevy entry is strict; the others keep their flag.
        assert!(
            !out.lines()
                .any(|l| l.contains("path = \"bevy_") && l.contains("allow-invalid"))
        );
        assert!(
            out.lines()
                .any(|l| l.contains("path = \"rand::") && l.contains("allow-invalid"))
        );
        // Comments and two entries on one line are not rewritten silently.
        let commented = "# { path = \"bevy_ecs::X\", allow-invalid = true },\n";
        assert_eq!(super::strict_clippy_config(commented, &all).unwrap().1, 0);
        let two = "{ path = \"bevy_ecs::X\", reason = \"r\", allow-invalid = true }, { path = \"bevy_ecs::Y\", reason = \"r\" },\n";
        assert!(super::strict_clippy_config(two, &all).is_err());
    }

    /// The `profiles` step searches the output for these messages: the kernel must carry exactly
    /// one `compile_error!` per development feature, under the `cfg` of that very feature, with
    /// the text the step expects.
    #[test]
    fn kernel_carries_the_profile_errors() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/ts_kernel/src/build_profile.rs");
        let source = crate::sys::read_text(&path)
            .expect("build_profile.rs moved: update this test and the `profiles` step");
        for f in crate::manifests::DEV_FEATURES {
            let literal = format!("\"{}\"", super::profile_error(f));
            assert_eq!(source.matches(&literal).count(), 1, "{f}: message");
            let head = &source[..source.find(&literal).unwrap_or_default()];
            let cfg = head.rfind("#[cfg(all(").unwrap_or_default();
            let expected = format!("#[cfg(all(feature = \"public\", feature = \"{f}\"))]");
            let gate = head[cfg..].split_whitespace().collect::<Vec<_>>().join(" ");
            assert!(gate.starts_with(&expected), "{f}: gate {gate}");
        }
        let count = source
            .lines()
            .filter(|l| l.trim_start().starts_with("compile_error!("))
            .count();
        assert_eq!(
            count,
            crate::manifests::DEV_FEATURES.len(),
            "compile_error! count"
        );
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
