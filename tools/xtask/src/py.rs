//! `cargo xtask py`: the Python environment of `ts_py` (T10). Mode d'emploi : docs/dev/xtask.md §8.
//! spec: carnet architecture-securite.md §9.16
//!
//! In `python/`: `uv sync --locked` (wheels only, hashes of `uv.lock`), then
//! `maturin develop --uv --locked` (crate `ts_py`, `dev` profile), then `pytest`. `uv` is the
//! one pinned in `tools/dev/tools.lock`: version and SHA-256 checked before any use (PY5).

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use crate::steps::lock_value;
use crate::sys;

/// Folder of the Python project, relative to the root of the repository.
const PY_DIR: &str = "python";
/// Pinned tools, relative to the root of the repository.
const TOOLS_LOCK: &str = "tools/dev/tools.lock";
/// Variables removed from the children: they could redirect uv, pip, the interpreter or the
/// pyo3 build (index, hashes, interpreter, `PYTHONPATH`, configuration file).
const STRIPPED_PREFIXES: [&str; 7] = [
    "UV_",
    "PIP_",
    "PYTHON",
    "PYO3_",
    "MATURIN_",
    "VIRTUAL_ENV",
    "CONDA",
];
const USAGE: &str = "usage : cargo xtask py";

pub fn main(args: &[String]) -> ExitCode {
    if !args.is_empty() {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    let root = match sys::stdout_of(
        &std::env::current_dir().unwrap_or_default(),
        "git",
        &["rev-parse", "--show-toplevel"],
    ) {
        Ok(r) => PathBuf::from(r.trim()),
        Err(e) => {
            eprintln!("ÉCHEC : racine du dépôt introuvable ({e})");
            return ExitCode::FAILURE;
        }
    };
    match run(&root) {
        Ok(()) => {
            println!("Résultat : OK");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("Résultat : ÉCHEC : {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(root: &Path) -> Result<(), String> {
    let lock = sys::read_text(&root.join(TOOLS_LOCK))?;
    let uv = pinned_uv(root, &lock)?;
    let dir = root.join(PY_DIR);
    let steps: [(&str, &[&str]); 3] = [
        ("uv sync", &["sync", "--locked"]),
        (
            "maturin develop",
            &["run", "--no-sync", "maturin", "develop", "--uv", "--locked"],
        ),
        (
            "pytest",
            &["run", "--no-sync", "pytest", "-p", "no:cacheprovider"],
        ),
    ];
    for (name, args) in steps {
        println!("\n[{name}]");
        if !run_uv(&uv, &dir, args)? {
            return Err(format!("{name} a échoué"));
        }
    }
    Ok(())
}

/// Lock key of the SHA-256 of the `uv` executable on this platform.
const fn uv_hash_key() -> &'static str {
    if cfg!(windows) {
        "uv.sha256"
    } else {
        "uv.linux.sha256"
    }
}

/// `uv` found on `PATH`, with the version and the SHA-256 of `tools.lock` (PY5). Messages give
/// the tool name, never its path, which holds the home folder (SEC0-26).
fn pinned_uv(root: &Path, lock: &str) -> Result<PathBuf, String> {
    let version = lock_value(lock, "uv.version")?;
    let sha256 = lock_value(lock, uv_hash_key())?.to_ascii_lowercase();
    let name = if cfg!(windows) { "uv.exe" } else { "uv" };
    let path = sys::env("PATH")
        .map(|p| std::env::split_paths(&p).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .map(|d| d.join(name))
        .find(|p| p.is_file())
        .ok_or("uv introuvable dans le PATH")?;
    if sys::sha256(root, &path)? != sha256 {
        return Err(format!(
            "{name} : empreinte SHA-256 différente de tools.lock"
        ));
    }
    let out = sys::stdout_of(root, &path.to_string_lossy(), &["--version"])?;
    match out.split_whitespace().nth(1) {
        Some(v) if v == version => {
            println!("uv {version}, empreinte conforme à tools.lock");
            Ok(path)
        }
        _ => Err(format!(
            "uv {version} attendu (tools.lock), autre version trouvée"
        )),
    }
}

/// True if `name` is removed from the environment of the children.
fn stripped(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    STRIPPED_PREFIXES.iter().any(|p| upper.starts_with(p))
}

/// Runs `uv args` in `dir`, output shown, without the [`STRIPPED_PREFIXES`] variables.
fn run_uv(uv: &Path, dir: &Path, args: &[&str]) -> Result<bool, String> {
    println!("  $ uv {}", args.join(" "));
    let mut cmd = Command::new(uv);
    cmd.args(args).current_dir(dir).stdin(Stdio::null());
    for name in sys::env_names().iter().filter(|n| stripped(n)) {
        cmd.env_remove(name);
    }
    // No bytecode written next to the tests.
    cmd.env("PYTHONDONTWRITEBYTECODE", "1");
    let status = cmd
        .status()
        .map_err(|e| format!("uv impossible à lancer : {e}"))?;
    Ok(status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_redirecting_variables() {
        for name in [
            "UV_INDEX_URL",
            "uv_no_verify_hashes",
            "PIP_INDEX_URL",
            "PYTHONPATH",
            "PYTHONHOME",
            "PYO3_PYTHON",
            "PYO3_CONFIG_FILE",
            "MATURIN_PEP517_ARGS",
            "VIRTUAL_ENV",
            "CONDA_PREFIX",
        ] {
            assert!(stripped(name), "{name}");
        }
        for name in [
            "PATH",
            "CARGO_HOME",
            "TS_CACHE",
            "CARGO_TARGET_DIR",
            "SYSTEMROOT",
        ] {
            assert!(!stripped(name), "{name}");
        }
    }

    #[test]
    fn reads_uv_pin() {
        let lock = "# c\nuv.version = \"0.11.33\"\nuv.sha256 = \"AB\"\nuv.linux.sha256 = \"cd\"\n";
        assert_eq!(lock_value(lock, "uv.version").unwrap(), "0.11.33");
        assert!(lock_value(lock, uv_hash_key()).is_ok());
        assert!(lock_value(lock, "uv.source").is_err());
    }
}
