// spec: carnet architecture-securite.md §9.15 Y3, §9.5 F8 ; revues/developpement-agents.md A6 ; revues/architecture.md §19.2
//! The run directory of `ts-sim run` (`--out`). Rules (Y3):
//! - `TS_CACHE` is a real directory (not a link), at least two levels below the root of the
//!   disk, outside any Git working copy; its canonical form is the reference;
//! - `--out` has no `..`, no `.git` name, and on Windows no name ending in a dot or a space
//!   (checked again after `absolute`, which rewrites such names); **no existing component of
//!   it is a link** (symbolic link or junction, any reparse point on Windows): nothing is
//!   followed;
//! - its canonical form must be **strictly under** `TS_CACHE`; with the explicit option it may
//!   be elsewhere, but **never inside a Git working copy** (A6: no artefact in the repository);
//! - missing directories are created one by one, each checked afterwards; files are created
//!   with `create_new`, so an existing file or link is never overwritten nor followed.
//!
//! Residual risk, accepted as R-ARCH-08 (AS-35): the checks and the creation are not atomic. A
//! writer of `TS_CACHE` could swap a directory for a link in between; that writer can already
//! change every run stored there.

use std::fmt;
use std::fs::{self, Metadata, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::path::{Component, Path, PathBuf};

/// `TS_CACHE` when the variable is not set, on Windows (A6, convention of `tools/dev`).
pub const DEFAULT_CACHE: &str = "C:/Dev/ts-cache";
/// Elsewhere, `TS_CACHE` defaults to this directory under `HOME` (A6).
pub const DEFAULT_CACHE_UNDER_HOME: &str = ".cache/time_switcher";

/// Why a run directory is refused. Never carries a local path (SEC0-26).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutError {
    /// `TS_CACHE` is missing, a link, not a directory, too close to the root, or in a Git
    /// working copy.
    BadCache,
    /// `--out` has a `..` component, no final name, or is not a directory.
    BadPath,
    /// An existing component of `--out` is a symbolic link or a junction.
    Link,
    /// Not strictly under `TS_CACHE`, and the explicit option was not given.
    OutsideCache,
    /// Inside a Git working copy: always refused (A6).
    InRepository,
    /// A file of the run already exists.
    Exists,
    /// Another input/output error.
    Io(ErrorKind),
}

impl fmt::Display for OutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::BadCache => {
                "TS_CACHE refusé : absent, lien, pas un dossier, trop près de la \
                               racine du disque, ou dans une copie de travail Git"
            }
            Self::BadPath => "--out invalide : composant `..`, pas de nom final, ou pas un dossier",
            Self::Link => "--out passe par un lien symbolique ou une jonction : refusé",
            Self::OutsideCache => {
                "--out n'est pas strictement sous TS_CACHE (option --allow-outside-cache)"
            }
            Self::InRepository => "--out est dans une copie de travail Git : toujours refusé (A6)",
            Self::Exists => "le dossier de run contient déjà ce fichier",
            Self::Io(kind) => return write!(f, "erreur d'entrée-sortie : {kind}"),
        };
        f.write_str(text)
    }
}

/// Whether `md` is a symbolic link or, on Windows, any reparse point (junctions included).
pub fn is_link(md: &Metadata) -> bool {
    if md.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        md.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    false
}

/// A Git working copy (main copy or worktree: `.git` directory or file) holds `canon`.
fn in_repository(canon: &Path) -> bool {
    canon
        .ancestors()
        .any(|a| fs::symlink_metadata(a.join(".git")).is_ok())
}

/// Canonical form of `TS_CACHE`, after its checks.
pub fn cache_root(cache: &Path) -> Result<PathBuf, OutError> {
    let md = fs::symlink_metadata(cache).map_err(|_| OutError::BadCache)?;
    if is_link(&md) || !md.is_dir() {
        return Err(OutError::BadCache);
    }
    let canon = fs::canonicalize(cache).map_err(|_| OutError::BadCache)?;
    let depth = canon
        .components()
        .filter(|c| matches!(c, Component::Normal(_)))
        .count();
    if depth < 2 || in_repository(&canon) {
        return Err(OutError::BadCache);
    }
    Ok(canon)
}

/// Whether `path` has a component that `--out` never allows: `..`, a `.git` name (it would turn
/// the run directory into a working copy for [`in_repository`]), and on Windows a name ending in
/// a dot or a space (Win32 strips them, so `.git.` is `.git` there, and only a verbatim path
/// can name them).
fn has_refused_component(path: &Path) -> bool {
    // In a verbatim path (`\\?\`), `.` and `..` are plain names to `components`, not to Windows.
    path.components().any(|c| match c {
        Component::ParentDir => true,
        Component::Normal(n) => {
            n == "." || n == ".." || n.eq_ignore_ascii_case(".git") || has_win32_trailing(n)
        }
        _ => false,
    })
}

#[cfg(windows)]
fn has_win32_trailing(name: &std::ffi::OsStr) -> bool {
    use std::os::windows::ffi::OsStrExt;
    name.encode_wide()
        .last()
        .is_some_and(|u| u == u16::from(b'.') || u == u16::from(b' '))
}

#[cfg(not(windows))]
fn has_win32_trailing(_: &std::ffi::OsStr) -> bool {
    false
}

/// Checks `out` against the canonical `cache` (from [`cache_root`]), creates the missing
/// directories, and returns the canonical run directory.
pub fn prepare(out: &Path, cache: &Path, outside_allowed: bool) -> Result<PathBuf, OutError> {
    if has_refused_component(out) {
        return Err(OutError::BadPath);
    }
    let out = std::path::absolute(out).map_err(|_| OutError::BadPath)?;
    // `absolute` rewrites names on Windows (it strips trailing dots and spaces: `.git.` becomes
    // `.git`), so the same check runs again on what it returns, which is what gets created.
    if has_refused_component(&out) {
        return Err(OutError::BadPath);
    }
    // A link is refused at the deepest existing prefix, and anywhere at or under `TS_CACHE`; a
    // system link above it (`/var` on some systems) is resolved by `canonicalize` below.
    let in_cache = |p: &Path| {
        let parent = p.parent().and_then(|q| fs::canonicalize(q).ok());
        match (parent, p.file_name()) {
            (Some(q), Some(name)) => q.join(name).starts_with(cache),
            _ => true,
        }
    };
    // Every existing prefix, from the deepest one; the missing names are kept for creation.
    let mut missing = Vec::new();
    let mut base: Option<&Path> = None;
    for prefix in out.ancestors() {
        match fs::symlink_metadata(prefix) {
            Ok(md) if is_link(&md) && (base.is_none() || in_cache(prefix)) => {
                return Err(OutError::Link);
            }
            Ok(_) if base.is_some() => {}
            Ok(md) if md.is_dir() => base = Some(prefix),
            Ok(_) => return Err(OutError::BadPath),
            Err(e) if base.is_none() => match e.kind() {
                ErrorKind::NotFound => missing.push(prefix.file_name().ok_or(OutError::BadPath)?),
                // Never skip a name below the existing base: the run would land in its parent.
                kind => return Err(OutError::Io(kind)),
            },
            // Above an existing directory: a prefix that cannot be inspected (a drive or share
            // root); `canonicalize` below resolves it.
            Err(_) => {}
        }
    }
    let base = base.ok_or(OutError::BadPath)?;
    let mut dir = fs::canonicalize(base).map_err(|e| OutError::Io(e.kind()))?;
    if in_repository(&dir) {
        return Err(OutError::InRepository);
    }
    let target = missing.iter().rev().fold(dir.clone(), |p, n| p.join(n));
    let under_cache = target.starts_with(cache) && target != cache;
    if !under_cache && !outside_allowed {
        return Err(OutError::OutsideCache);
    }
    for name in missing.iter().rev() {
        dir.push(name);
        match fs::create_dir(&dir) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
            Err(e) => return Err(OutError::Io(e.kind())),
        }
        let md = fs::symlink_metadata(&dir).map_err(|e| OutError::Io(e.kind()))?;
        if is_link(&md) || !md.is_dir() {
            return Err(OutError::Link);
        }
    }
    Ok(dir)
}

/// Fails if one of `names` already exists in `dir`: checked before a run, so that a run is
/// never computed for nothing (`write_new` checks again when writing).
pub fn ensure_absent(dir: &Path, names: &[&str]) -> Result<(), OutError> {
    match names
        .iter()
        .any(|n| fs::symlink_metadata(dir.join(n)).is_ok())
    {
        true => Err(OutError::Exists),
        false => Ok(()),
    }
}

/// Writes a new file `name` in the run directory `dir`; an existing file or link is an error.
#[allow(clippy::disallowed_methods)] // host: writes one file of the run directory.
pub fn write_new(dir: &Path, name: &str, bytes: &[u8]) -> Result<(), OutError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(name))
        .map_err(|e| match e.kind() {
            ErrorKind::AlreadyExists => OutError::Exists,
            kind => OutError::Io(kind),
        })?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|e| OutError::Io(e.kind()))
}
