//! Accès au système. xtask est une crate d'outil : elle s'exempte des interdictions de clippy
//! au plus près, ici seulement (E6).

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
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

#[allow(clippy::disallowed_methods)] // outil : xtask lit les fichiers du dépôt.
pub fn read(path: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("lecture impossible de {} : {e}", path.display()))
}

pub fn read_text(path: &Path) -> Result<String, String> {
    String::from_utf8(read(path)?).map_err(|_| format!("{} n'est pas en UTF-8", path.display()))
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

/// Makes every Git run by `cmd`, or by a tool it starts (gitleaks, cargo, uv), ignore the
/// replacement objects (`refs/replace/`): a ref planted there could otherwise show xtask another
/// commit, tree or blob than the one named by its SHA (#63). `GIT_NO_REPLACE_OBJECTS` is the
/// environment form of `git --no-replace-objects`; it is set for every child, so that no call can
/// forget it. Every child of xtask must be prepared by this function or by [`command`].
pub fn harden_git_env(cmd: &mut Command) {
    cmd.env("GIT_NO_REPLACE_OBJECTS", "1");
}

fn command(root: &Path, prog: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(prog);
    cmd.args(args).current_dir(root);
    for var in SCRUBBED_ENV {
        cmd.env_remove(var);
    }
    harden_git_env(&mut cmd);
    cmd
}

/// Lance une commande, sortie affichée. Un outil absent est une erreur, jamais une réussite.
pub fn run_live(root: &Path, prog: &str, args: &[&str]) -> Result<bool, String> {
    // Nom seul : le chemin complet d'un outil contient souvent le dossier personnel (SEC0-26).
    let shown = Path::new(prog)
        .file_stem()
        .map_or(prog.into(), |s| s.to_string_lossy());
    println!("  $ {shown} {}", args.join(" "));
    let status = command(root, prog, args)
        .stdin(Stdio::null())
        .status()
        .map_err(|e| format!("{prog} introuvable ou impossible à lancer : {e}"))?;
    Ok(status.success())
}

/// Lance une commande et rend (réussite, sortie standard et sortie d'erreur).
pub fn run_captured(root: &Path, prog: &str, args: &[&str]) -> Result<(bool, String), String> {
    let out = command(root, prog, args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{prog} introuvable ou impossible à lancer : {e}"))?;
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
        .map_err(|e| format!("{prog} introuvable ou impossible à lancer : {e}"))?;
    if !out.status.success() {
        return Err(format!("échec de « {prog} {} »", args.join(" ")));
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
        .map_err(|e| format!("{prog} introuvable ou impossible à lancer : {e}"))?;
    Ok((
        out.status.code(),
        out.stdout,
        String::from_utf8_lossy(&out.stderr).into_owned(),
    ))
}

#[allow(clippy::disallowed_methods)] // outil : copie épinglée des crochets, corps d'issue.
pub fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(path, bytes)
        .map_err(|e| format!("écriture impossible de {} : {e}", path.display()))
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
        return Err(format!(
            "empreinte SHA-256 incalculable pour {}",
            file.display()
        ));
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
        Err(format!(
            "empreinte SHA-256 illisible pour {}",
            file.display()
        ))
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
        /// Commit with `a` and `b`, replaced by the commit with `a` only.
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
        // `run_live` only shows its output: `hash-object` tells the real blob from the fake one.
        let real = r.lab.git(&["rev-parse", &blob_spec]);
        let check = ["rev-parse", "--verify", &format!("{real}^{{blob}}")];
        assert_eq!(run_live(&r.lab.dir, "git", &check), Ok(true));
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
        let mut uv = Command::new("uv");
        harden_git_env(&mut uv);
        assert_eq!(value(&uv), Some(Some("1".to_owned())));
    }
}
