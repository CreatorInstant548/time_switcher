//! Accès au système. xtask est une crate d'outil : elle s'exempte des interdictions de clippy
//! au plus près, ici seulement (E6).

use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime};

/// Variables qui changeraient la configuration de Git ou de gitleaks, vidées comme dans les crochets.
const SCRUBBED_ENV: [&str; 5] = [
    "GIT_CONFIG_PARAMETERS",
    "GIT_CONFIG_COUNT",
    "GIT_EXTERNAL_DIFF",
    "GITLEAKS_CONFIG",
    "GITLEAKS_CONFIG_TOML",
];

#[allow(clippy::disallowed_methods)] // outil : xtask lit sa ligne de commande.
pub fn args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

#[allow(clippy::disallowed_methods)] // outil : CARGO, TS_CACHE, CARGO_HOME, USERNAME, COMPUTERNAME, USERPROFILE.
pub fn env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

/// Names of the environment variables of this process (`cargo xtask py` strips some of them
/// from its children).
#[allow(clippy::disallowed_methods)] // tool: reads the names, never the values.
pub fn env_names() -> Vec<String> {
    std::env::vars_os()
        .map(|(k, _)| k.to_string_lossy().into_owned())
        .collect()
}

/// Root of the repository being checked, set once by `Ctx::new`: messages show paths relative to
/// it (m6, SEC0-26).
static ROOT: OnceLock<PathBuf> = OnceLock::new();

pub fn set_root(root: &Path) {
    let _ = ROOT.set(root.to_path_buf());
}

/// `path` without the leading `base`, comparing components (so `C:/a` equals `C:`), ignoring
/// the case on Windows.
fn strip(path: &Path, base: &Path) -> Option<PathBuf> {
    let same = |a: Component, b: Component| match cfg!(windows) {
        true => a.as_os_str().eq_ignore_ascii_case(b.as_os_str()),
        false => a == b,
    };
    let mut rest = path.components();
    for b in base.components() {
        if !same(rest.next()?, b) {
            return None;
        }
    }
    Some(rest.as_path().to_path_buf())
}

/// `path` relative to `base` with `/`, if it lies under it (component-wise, so `a/tree` does not
/// match `a/tree2/x`).
pub fn relative_to(path: &Path, base: &Path) -> Option<String> {
    let rel = strip(path, base)?;
    let parts: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    Some(parts.join("/"))
}

/// A path as shown in a message: relative to `root` with `/`, `$TS_CACHE/…` under the cache,
/// otherwise its file name only. An absolute path would give the home folder away (m6, SEC0-26).
fn display_path(path: &Path, root: Option<&Path>, cache: &Path) -> String {
    let join = |p: &Path| {
        let parts: Vec<String> = p
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();
        parts.join("/")
    };
    if let Some(rel) = root.and_then(|r| strip(path, r)) {
        return match rel.as_os_str().is_empty() {
            true => ".".to_owned(),
            false => join(&rel),
        };
    }
    if let Some(rel) = strip(path, cache) {
        return format!("$TS_CACHE/{}", join(&rel));
    }
    if !path.is_absolute() {
        return join(path);
    }
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    format!("<outside the repository>/{name}")
}

pub fn shown(path: &Path) -> String {
    display_path(path, ROOT.get().map(PathBuf::as_path), &ts_cache())
}

/// A command-line argument as printed: an absolute path is shown by [`shown`].
fn shown_arg(arg: &str) -> String {
    match Path::new(arg).is_absolute() {
        true => shown(Path::new(arg)),
        false => arg.to_owned(),
    }
}

#[allow(clippy::disallowed_methods)] // tool: xtask reads the files of the repository.
pub fn read(path: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", shown(path)))
}

pub fn read_text(path: &Path) -> Result<String, String> {
    String::from_utf8(read(path)?).map_err(|_| format!("{} is not UTF-8", shown(path)))
}

#[allow(clippy::disallowed_methods)] // outil : âge de la base RustSec (SEC0-03).
pub fn age(path: &Path) -> Option<Duration> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    SystemTime::now().duration_since(modified).ok()
}

/// Instant présent, pour afficher une durée (`determinism`), jamais pour un résultat.
#[allow(clippy::disallowed_methods)] // outil : durée mesurée et affichée.
pub fn now() -> Instant {
    Instant::now()
}

/// Durée écoulée depuis `start`.
#[allow(clippy::disallowed_methods)] // outil : durée mesurée et affichée.
pub fn elapsed(start: Instant) -> Duration {
    start.elapsed()
}

/// Pause before retrying a GitHub read (V4).
pub fn sleep(d: Duration) {
    std::thread::sleep(d);
}

pub fn ts_cache() -> PathBuf {
    PathBuf::from(env("TS_CACHE").unwrap_or_else(|| "C:/Dev/ts-cache".to_owned()))
}

pub fn cargo() -> String {
    env("CARGO").unwrap_or_else(|| "cargo".to_owned())
}

/// The one place that creates a child process of xtask (apart from tests and the system probes
/// of `slots`): every Git run by the child, or by a tool it starts (gitleaks, cargo, uv), ignores
/// the replacement objects (`refs/replace/`). A ref planted there could otherwise show xtask
/// another commit, tree or blob than the one named by its SHA (#63). `GIT_NO_REPLACE_OBJECTS` is
/// the environment form of `git --no-replace-objects`.
pub fn base_command(prog: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut cmd = Command::new(prog);
    cmd.env("GIT_NO_REPLACE_OBJECTS", "1");
    cmd
}

fn command(root: &Path, prog: &str, args: &[&str]) -> Command {
    let mut cmd = base_command(prog);
    cmd.args(args).current_dir(root);
    for var in SCRUBBED_ENV {
        cmd.env_remove(var);
    }
    cmd
}

/// Lance une commande, sortie affichée. Un outil absent est une erreur, jamais une réussite.
pub fn run_live(root: &Path, prog: &str, args: &[&str]) -> Result<bool, String> {
    // Nom seul : le chemin complet d'un outil contient souvent le dossier personnel (SEC0-26).
    let shown = Path::new(prog)
        .file_stem()
        .map_or(prog.into(), |s| s.to_string_lossy());
    let printed: Vec<String> = args.iter().map(|a| shown_arg(a)).collect();
    println!("  $ {shown} {}", printed.join(" "));
    let status = command(root, prog, args)
        .stdin(Stdio::null())
        .status()
        .map_err(|e| format!("cannot run {}: {e}", shown_arg(prog)))?;
    Ok(status.success())
}

/// Lance une commande et rend (réussite, sortie standard et sortie d'erreur).
pub fn run_captured(root: &Path, prog: &str, args: &[&str]) -> Result<(bool, String), String> {
    run_captured_env(root, prog, args, &[])
}

/// [`run_captured`] with extra environment variables for the child only.
pub fn run_captured_env(
    root: &Path,
    prog: &str,
    args: &[&str],
    env: &[(&str, &str)],
) -> Result<(bool, String), String> {
    let out = command(root, prog, args)
        .envs(env.iter().copied())
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("cannot run {}: {e}", shown_arg(prog)))?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    Ok((out.status.success(), text))
}

/// Sortie standard d'une commande qui doit réussir.
pub fn stdout_of(root: &Path, prog: &str, args: &[&str]) -> Result<String, String> {
    let out = command(root, prog, args)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("cannot run {}: {e}", shown_arg(prog)))?;
    if !out.status.success() {
        return Err(format!("{} {} failed", shown_arg(prog), args.join(" ")));
    }
    String::from_utf8(out.stdout).map_err(|_| format!("sortie non UTF-8 de {prog}"))
}

/// Code de sortie (`None` si tué), sortie standard brute et sortie d'erreur, séparées.
pub fn outputs(
    root: &Path,
    prog: &str,
    args: &[&str],
) -> Result<(Option<i32>, Vec<u8>, String), String> {
    let out = command(root, prog, args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("cannot run {}: {e}", shown_arg(prog)))?;
    Ok((
        out.status.code(),
        out.stdout,
        String::from_utf8_lossy(&out.stderr).into_owned(),
    ))
}

#[allow(clippy::disallowed_methods)] // tool: issue bodies, files of the run.
pub fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(path, bytes).map_err(|e| format!("cannot write {}: {e}", shown(path)))
}

/// Name of the file that marks a [`Scratch`] folder as created by this run.
const SCRATCH_MARKER: &str = ".xtask-scratch";

/// Folders of earlier runs left by a killed process: `xtask-<label>-*` under `base` whose marker
/// is more than a day old. Only folders that carry the marker are touched.
#[allow(clippy::disallowed_methods)] // tool: lists its own scratch folders.
fn sweep(base: &Path, label: &str) {
    let prefix = format!("xtask-{label}-");
    let Ok(entries) = std::fs::read_dir(base) else {
        return;
    };
    for entry in entries.flatten() {
        let (dir, name) = (
            entry.path(),
            entry.file_name().to_string_lossy().into_owned(),
        );
        let stale = age(&dir.join(SCRATCH_MARKER)).is_some_and(|a| a > Duration::from_secs(86_400));
        let plain = std::fs::symlink_metadata(&dir).is_ok_and(|m| m.is_dir());
        if name.starts_with(&prefix) && plain && stale {
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
}

/// A folder made for one run under `TS_CACHE/tmp` (m8). It is created exclusively under a name that
/// no earlier run can have (process id, clock and counter), holds a marker with a token, and is
/// removed on drop only while the marker still carries that token: a folder this run did not
/// create is never deleted.
#[derive(Debug)]
pub struct Scratch {
    dir: PathBuf,
    token: String,
}

impl Scratch {
    pub fn new(label: &str) -> Result<Self, String> {
        Self::new_in(&ts_cache().join("tmp"), label)
    }

    #[allow(clippy::disallowed_methods)] // tool: the marker is written with `create_new`.
    pub fn new_in(base: &Path, label: &str) -> Result<Self, String> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let count = COUNTER.fetch_add(1, Ordering::Relaxed);
        let token = format!("{}-{nanos}-{count}", std::process::id());
        std::fs::create_dir_all(base).map_err(|e| format!("{}: {e}", shown(base)))?;
        let dir = base.join(format!("xtask-{label}-{token}"));
        // `create_dir` fails when the folder exists: this run owns the one it creates.
        std::fs::create_dir(&dir).map_err(|e| format!("{}: {e}", shown(&dir)))?;
        sweep(base, label);
        let this = Self { dir, token };
        let marker = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(this.dir.join(SCRATCH_MARKER))
            .and_then(|mut f| std::io::Write::write_all(&mut f, this.token.as_bytes()));
        if let Err(e) = marker {
            // Created just above by this run, so it is ours to remove even without a marker.
            let _ = std::fs::remove_dir_all(&this.dir);
            return Err(format!("{}: {e}", shown(base)));
        }
        Ok(this)
    }

    /// A path inside the folder, not created.
    pub fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    fn owned(&self) -> bool {
        let is_dir = std::fs::symlink_metadata(&self.dir).is_ok_and(|m| m.is_dir());
        is_dir && read(&self.dir.join(SCRATCH_MARKER)).is_ok_and(|m| m == self.token.as_bytes())
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.owned() {
            let _ = std::fs::remove_dir_all(&self.dir); // marker checked: created by this run
        }
    }
}

/// SHA-256 d'un fichier, par l'outil du système (aucune crate de cryptographie ajoutée).
pub fn sha256(root: &Path, file: &Path) -> Result<String, String> {
    let path = file.to_string_lossy();
    // certutil écrit dans la page de code de la console : seule la ligne hexadécimale compte.
    let (ok, text) = if cfg!(windows) {
        run_captured(root, "certutil", &["-hashfile", &path, "SHA256"])?
    } else {
        run_captured(root, "sha256sum", &[&path])?
    };
    if !ok {
        return Err(format!("SHA-256 cannot be computed for {}", shown(file)));
    }
    let hex: String = if cfg!(windows) {
        text.lines().nth(1).unwrap_or_default().replace(' ', "")
    } else {
        text.split_whitespace()
            .next()
            .unwrap_or_default()
            .to_owned()
    };
    let hex = hex.trim().to_ascii_lowercase();
    if hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        Ok(hex)
    } else {
        Err(format!("SHA-256 unreadable for {}", shown(file)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::carry::lab::Lab;
    use crate::github::{Repo, System};

    /// A repository with a replace ref on each kind of object, so that a read which honours them
    /// is told apart from one which does not (#63).
    struct Replaced {
        lab: Lab,
        /// The commit with `a` only.
        only_a: String,
        /// Commit with `a` and `b`, replaced by `only_a`.
        commit: String,
        /// Commit with `a` and `b`, whose tree is replaced by the tree with `a` only.
        tree_commit: String,
        /// Commit with `a` and `b` whose blob `a` is replaced by another blob.
        blob_commit: String,
    }

    fn replaced(name: &str) -> Replaced {
        let lab = Lab::new(name);
        let only_a = lab.commit(&[("a", Some("real a\n"))]);
        let commit = lab.commit(&[("b", Some("real b\n"))]);
        lab.git(&["replace", &commit, &only_a]);
        // Another history, so that these commits are not themselves replaced.
        lab.git(&["checkout", "-q", "--orphan", "second"]);
        let tree_one = lab.commit(&[("a", Some("tree a\n")), ("b", None)]);
        let tree_commit = lab.commit(&[("b", Some("tree b\n"))]);
        let (tree_old, tree_new) = (
            lab.git(&["rev-parse", &format!("{tree_commit}^{{tree}}")]),
            lab.git(&["rev-parse", &format!("{tree_one}^{{tree}}")]),
        );
        lab.git(&["replace", &tree_old, &tree_new]);
        lab.git(&["checkout", "-q", "--orphan", "third"]);
        let blob_commit = lab.commit(&[("a", Some("blob a\n")), ("b", Some("blob b\n"))]);
        write(&lab.dir.join("fake.txt"), b"fake a\n").unwrap();
        let fake_blob = lab.git(&["hash-object", "-w", "fake.txt"]);
        let real_blob = lab.git(&["rev-parse", &format!("{blob_commit}:a")]);
        lab.git(&["replace", &real_blob, &fake_blob]);
        Replaced {
            lab,
            only_a,
            commit,
            tree_commit,
            blob_commit,
        }
    }

    /// Files of `rev` as read by xtask's own launcher.
    fn names(lab: &Lab, rev: &str) -> String {
        let out = stdout_of(&lab.dir, "git", &["ls-tree", "-r", "--name-only", rev]).unwrap();
        out.split_whitespace().collect::<Vec<_>>().join(",")
    }

    /// The control: plain Git does honour the replace refs, so the tests below mean something.
    #[test]
    fn control_plain_git_honours_replace_refs() {
        let r = replaced("sys-control");
        assert_eq!(r.lab.git(&["ls-tree", "-r", "--name-only", &r.commit]), "a");
        assert_eq!(
            r.lab.git(&["ls-tree", "-r", "--name-only", &r.tree_commit]),
            "a"
        );
        let spec = format!("{}:a", r.blob_commit);
        assert_eq!(r.lab.git(&["cat-file", "blob", &spec]), "fake a");
    }

    /// Every launcher of `sys` reads the real objects: files of a commit, files of a tree, blob.
    #[test]
    fn launchers_ignore_replace_refs() {
        let r = replaced("sys-launchers");
        assert_eq!(names(&r.lab, &r.commit), "a,b");
        assert_eq!(names(&r.lab, &r.tree_commit), "a,b");
        let blob_spec = format!("{}:a", r.blob_commit);
        let args = ["cat-file", "blob", &blob_spec];
        let (code, out, _) = outputs(&r.lab.dir, "git", &args).unwrap();
        assert_eq!((code, out.as_slice()), (Some(0), &b"blob a\n"[..]));
        let (ok, text) = run_captured(&r.lab.dir, "git", &args).unwrap();
        assert!(ok);
        assert_eq!(text, "blob a\n");
        assert_eq!(stdout_of(&r.lab.dir, "git", &args).unwrap(), "blob a\n");
        // `run_live` only shows its output, so the exit code tells: the commit differs from
        // `only_a` unless the replace ref is honoured.
        let diff = ["diff", "--quiet", &r.commit, &r.only_a];
        assert_eq!(run_live(&r.lab.dir, "git", &diff), Ok(false));
    }

    /// The `Repo` reads of `merge` and `verify-main` (`git`, `blob`) go through the same launcher.
    #[test]
    fn repo_reads_ignore_replace_refs() {
        let r = replaced("sys-repo");
        let repo = Repo {
            main: r.lab.dir.clone(),
            here: r.lab.dir.clone(),
            slug: "o/r".to_owned(),
            dry_run: true,
            ci: false,
            cache: r.lab.dir.join(".cache"),
            runner: Box::new(System),
        };
        let ls = repo
            .git(&["ls-tree", "-r", "--name-only", &r.commit])
            .unwrap();
        assert_eq!(ls.split_whitespace().collect::<Vec<_>>(), ["a", "b"]);
        let blob = repo.blob(&r.lab.dir, &r.blob_commit, "a").unwrap();
        assert_eq!(blob.as_deref(), Some(&b"blob a\n"[..]));
    }

    /// m6: messages show repository paths relative to the root, the cache as `$TS_CACHE`, and any
    /// other absolute path by its file name only.
    #[test]
    fn paths_are_shown_without_the_home_folder() {
        let base = std::env::temp_dir();
        let (root, cache) = (base.join("repo"), base.join("cache"));
        let show = |p: &Path| display_path(p, Some(&root), &cache);
        assert_eq!(show(&root.join("a").join("b.rs")), "a/b.rs");
        let mixed = format!("{}/a{}b.rs", root.display(), std::path::MAIN_SEPARATOR);
        assert_eq!(show(Path::new(&mixed)), "a/b.rs");
        assert_eq!(show(&root), ".");
        assert_eq!(
            show(&cache.join("tmp").join("x.json")),
            "$TS_CACHE/tmp/x.json"
        );
        assert_eq!(
            show(&base.join("elsewhere").join("f.txt")),
            "<outside the repository>/f.txt"
        );
        assert_eq!(show(Path::new("rel/x.rs")), "rel/x.rs");
        assert_eq!(
            display_path(&root.join("a"), None, &cache),
            "<outside the repository>/a"
        );
    }

    /// m8: a scratch folder is unique, removed by its owner, and never removed once its marker
    /// is not the owner's.
    #[test]
    fn scratch_removes_only_its_own_folder() {
        let base = std::env::temp_dir().join("xtask-scratch-test");
        let (a, b) = (
            Scratch::new_in(&base, "t").unwrap(),
            Scratch::new_in(&base, "t").unwrap(),
        );
        assert_ne!(a.path("."), b.path("."));
        assert!(a.path(SCRATCH_MARKER).is_file());
        write(&a.path("x"), b"data").unwrap();
        let kept = a.path(".");
        drop(a);
        assert!(!kept.exists());
        // A folder whose marker is another run's, or that has none: left alone.
        let foreign = b.path(".");
        write(&b.path(SCRATCH_MARKER), b"another run").unwrap();
        drop(b);
        assert!(foreign.is_dir());
        let c = Scratch::new_in(&base, "t").unwrap();
        std::fs::remove_file(c.path(SCRATCH_MARKER)).unwrap();
        let bare = c.path(".");
        drop(c);
        assert!(bare.is_dir());
        std::fs::remove_dir_all(&base).unwrap();
    }

    /// A folder of a killed run (marker older than a day) is swept by the next run; a young one,
    /// or one without a marker, is not.
    #[test]
    #[allow(clippy::disallowed_methods)] // test: ages a file by setting its modification time.
    fn scratch_sweeps_only_old_marked_folders() {
        let base = std::env::temp_dir().join("xtask-sweep-test");
        let old = Scratch::new_in(&base, "s").unwrap();
        let (old_dir, young_dir, bare_dir) = (
            old.path("."),
            base.join("xtask-s-young"),
            base.join("xtask-s-bare"),
        );
        std::mem::forget(old); // as if killed
        for d in [&young_dir, &bare_dir] {
            std::fs::create_dir_all(d).unwrap();
        }
        write(&young_dir.join(SCRATCH_MARKER), b"x").unwrap();
        let aged = std::time::SystemTime::now() - Duration::from_secs(2 * 86_400);
        let marker = std::fs::File::options()
            .write(true)
            .open(old_dir.join(SCRATCH_MARKER))
            .unwrap();
        marker.set_modified(aged).unwrap();
        drop(marker);
        let next = Scratch::new_in(&base, "s").unwrap();
        assert!(!old_dir.exists());
        assert!(young_dir.is_dir() && bare_dir.is_dir());
        drop(next);
        std::fs::remove_dir_all(&base).unwrap();
    }

    /// The setting is made by the functions that prepare every child, whatever the program.
    #[test]
    fn every_command_sets_the_variable() {
        let value = |cmd: &Command| {
            cmd.get_envs()
                .find(|(k, _)| *k == "GIT_NO_REPLACE_OBJECTS")
                .map(|(_, v)| v.map(|v| v.to_string_lossy().into_owned()))
        };
        for prog in ["git", "cargo", "gitleaks", "gh"] {
            let cmd = command(Path::new("."), prog, &["x"]);
            assert_eq!(value(&cmd), Some(Some("1".to_owned())), "{prog}");
        }
        assert_eq!(value(&base_command("uv")), Some(Some("1".to_owned())));
    }
}
