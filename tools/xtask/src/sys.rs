//! Accès au système. xtask est une crate d'outil : elle s'exempte des interdictions de clippy
//! au plus près, ici seulement (E6).

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};

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

fn command(root: &Path, prog: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(prog);
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

/// Noms des entrées d'un dossier, avec vrai si c'est un fichier ordinaire (lien : faux).
#[allow(clippy::disallowed_methods)] // outil : contenu de la copie épinglée des crochets.
pub fn entries(dir: &Path) -> Result<Vec<(String, bool)>, String> {
    let err = |e: std::io::Error| format!("lecture impossible de {} : {e}", dir.display());
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).map_err(err)? {
        let e = e.map_err(err)?;
        let file = std::fs::symlink_metadata(e.path()).is_ok_and(|m| m.file_type().is_file());
        out.push((e.file_name().to_string_lossy().into_owned(), file));
    }
    out.sort();
    Ok(out)
}

/// Dossier personnel de l'utilisateur (réglages de Claude Code, lus sans jamais être écrits).
pub fn home() -> Option<PathBuf> {
    env("USERPROFILE")
        .or_else(|| env("HOME"))
        .map(PathBuf::from)
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
