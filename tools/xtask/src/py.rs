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
const USAGE: &str = "usage: cargo xtask py";

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
            eprintln!("FAILED: repository root not found ({e})");
            return ExitCode::FAILURE;
        }
    };
    match run(&root) {
        Ok(()) => {
            println!("Result: OK");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("Result: FAILED: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(root: &Path) -> Result<(), String> {
    let lock = sys::read_text(&root.join(TOOLS_LOCK))?;
    let uv = pinned_uv(root, &lock)?;
    let dir = root.join(PY_DIR);
    let exclude_newer = exclude_newer(&sys::read_text(&dir.join("uv.lock"))?)?;
    // Configuration files are not read (UV_NO_CONFIG, set in `run_uv`): the settings of
    // pyproject.toml that matter are passed here, so that no user-level uv.toml adds any.
    let sync = [
        "sync",
        "--locked",
        "--no-build",
        "--no-python-downloads",
        "--python-preference",
        "only-system",
        "--exclude-newer",
        &exclude_newer,
    ];
    let steps: [(&str, &[&str]); 3] = [
        ("uv sync", &sync),
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
            return Err(format!("{name} failed"));
        }
    }
    Ok(())
}

/// `exclude-newer` of the `[options]` of `uv.lock`: an RFC 3339 date, checked by its characters.
fn exclude_newer(uv_lock: &str) -> Result<String, String> {
    let value = uv_lock
        .lines()
        .take_while(|l| !l.starts_with("[[package]]"))
        .filter_map(|l| l.split_once('='))
        .find(|(k, _)| k.trim() == "exclude-newer")
        .map(|(_, v)| v.trim().trim_matches('"').to_owned())
        .ok_or("exclude-newer absent from python/uv.lock")?;
    let date = value.len() == 20
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || b"-:TZ".contains(&b));
    if date {
        Ok(value)
    } else {
        Err("exclude-newer of python/uv.lock is not a UTC date".to_owned())
    }
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
    let key = uv_hash_key();
    let sha256 = lock_value(lock, key)
        .map_err(|_| format!("{key} absent from tools.lock: no pinned uv for this platform"))?
        .to_ascii_lowercase();
    let name = if cfg!(windows) { "uv.exe" } else { "uv" };
    let path = sys::env("PATH")
        .map(|p| std::env::split_paths(&p).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .map(|d| d.join(name))
        .find(|p| p.is_file())
        .ok_or("uv not found on PATH")?;
    let actual = sys::sha256(root, &path).map_err(|_| format!("{name}: SHA-256 unreadable"))?;
    if actual != sha256 {
        return Err(format!("{name}: SHA-256 differs from tools.lock"));
    }
    let out = sys::stdout_of(root, &path.to_string_lossy(), &["--version"])
        .map_err(|_| format!("{name} --version failed"))?;
    match out.split_whitespace().nth(1) {
        Some(v) if v == version => {
            println!("uv {version}, SHA-256 as in tools.lock");
            Ok(path)
        }
        _ => Err(format!(
            "uv {version} expected (tools.lock), another version found"
        )),
    }
}

/// True if `name` is removed from the environment of the children.
fn stripped(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    STRIPPED_PREFIXES.iter().any(|p| upper.starts_with(p))
}

/// Runs `uv args` in `dir`, output shown, without the [`STRIPPED_PREFIXES`] variables and
/// without any uv configuration file (`UV_NO_CONFIG`, inherited by the `uv pip` of maturin).
fn run_uv(uv: &Path, dir: &Path, args: &[&str]) -> Result<bool, String> {
    println!("  $ uv {}", args.join(" "));
    let mut cmd = Command::new(uv);
    cmd.args(args).current_dir(dir).stdin(Stdio::null());
    for name in sys::env_names().iter().filter(|n| stripped(n)) {
        cmd.env_remove(name);
    }
    sys::harden_git_env(&mut cmd);
    cmd.env("UV_NO_CONFIG", "1");
    // No bytecode written next to the tests.
    cmd.env("PYTHONDONTWRITEBYTECODE", "1");
    let status = cmd
        .status()
        .map_err(|e| format!("uv cannot be started: {e}"))?;
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
    fn reads_exclude_newer_of_the_lock() {
        let lock = "version = 1\n\n[options]\nexclude-newer = \"2026-10-17T00:00:00Z\"\n\n[[package]]\nexclude-newer = \"x\"\n";
        assert_eq!(exclude_newer(lock).unwrap(), "2026-10-17T00:00:00Z");
        assert!(exclude_newer("version = 1\n").is_err());
        for bad in ["2026-10-17 --index x", "2026-10-17T00:00:00+0", ""] {
            let text = format!("[options]\nexclude-newer = \"{bad}\"\n");
            assert!(exclude_newer(&text).is_err(), "{bad}");
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
