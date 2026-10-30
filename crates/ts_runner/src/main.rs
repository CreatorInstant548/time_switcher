// spec: revues/architecture.md §19.2 ; carnet architecture-securite.md §9.15 Y1 à Y4
//! `ts-sim`, the simulation runner (phase 0: `run` on the toy world only).
//!
//! ```text
//! ts-sim run <scenario.ron> --seed <n> --out <dir> [--allow-outside-cache]
//! ```
//!
//! - `--seed` is a **public test seed** (Y2): the world key is derived from it alone.
//! - `--out` must be strictly under `TS_CACHE` (variable, else `C:/Dev/ts-cache` on Windows
//!   and `~/.cache/time_switcher` elsewhere); with
//!   `--allow-outside-cache` it may be elsewhere, but never in a Git working copy (Y3, A6).
//! - Writes `metrics.parquet`, then `manifest.ron` last: a run directory with a manifest is
//!   complete.
#![forbid(unsafe_code)]
#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "command-line tool: results on stdout, errors on stderr"
)]

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use ts_data::Limits;
use ts_runner::out_dir::{self, DEFAULT_CACHE, DEFAULT_CACHE_UNDER_HOME};
use ts_runner::scenario::Scenario;
use ts_runner::{manifest, metrics, run};

const USAGE: &str =
    "usage : ts-sim run <scénario.ron> --seed <n> --out <dossier> [--allow-outside-cache]";

struct Args {
    scenario: PathBuf,
    seed: u64,
    out: PathBuf,
    outside_allowed: bool,
}

fn parse(mut args: impl Iterator<Item = OsString>) -> Result<Args, String> {
    if args.next().as_deref() != Some("run".as_ref()) {
        return Err(USAGE.to_owned());
    }
    let (mut scenario, mut seed, mut out, mut outside_allowed) = (None, None, None, false);
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--seed") => {
                let text = args.next().ok_or(USAGE)?;
                let n = text.to_str().and_then(|t| t.parse::<u64>().ok());
                seed = Some(n.ok_or("--seed attend un entier de 0 à 2^64 - 1")?);
            }
            Some("--out") => out = Some(PathBuf::from(args.next().ok_or(USAGE)?)),
            Some("--allow-outside-cache") => outside_allowed = true,
            Some(a) if a.starts_with('-') => return Err(format!("option inconnue : {a}\n{USAGE}")),
            _ if scenario.is_none() => scenario = Some(PathBuf::from(arg)),
            _ => return Err(USAGE.to_owned()),
        }
    }
    match (scenario, seed, out) {
        (Some(scenario), Some(seed), Some(out)) => Ok(Args {
            scenario,
            seed,
            out,
            outside_allowed,
        }),
        _ => Err(USAGE.to_owned()),
    }
}

/// `TS_CACHE`, else its default (A6): never a source of key or seed, only of the output root.
#[allow(clippy::disallowed_methods)] // tool: reads TS_CACHE and HOME.
fn cache_dir() -> PathBuf {
    let var = |name| std::env::var_os(name).filter(|v| !v.is_empty());
    if let Some(cache) = var("TS_CACHE") {
        return PathBuf::from(cache);
    }
    match var("HOME") {
        Some(home) if cfg!(not(windows)) => PathBuf::from(home).join(DEFAULT_CACHE_UNDER_HOME),
        _ => PathBuf::from(DEFAULT_CACHE),
    }
}

#[allow(clippy::disallowed_methods)] // tool: reads its command line.
fn main() -> ExitCode {
    let cache = cache_dir();
    match parse(std::env::args_os().skip(1)).and_then(|a| run_command(&a, &cache)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ts-sim : {e}");
            ExitCode::FAILURE
        }
    }
}

fn run_command(args: &Args, cache: &Path) -> Result<(), String> {
    let limits = Limits::default();
    let (scenario, file) = Scenario::load(&args.scenario, &limits).map_err(|d| d.to_string())?;
    let cache = out_dir::cache_root(cache).map_err(|e| e.to_string())?;
    let dir =
        out_dir::prepare(&args.out, &cache, args.outside_allowed).map_err(|e| e.to_string())?;
    let outcome = run::run_toy(scenario.days(), args.seed)?;
    let parquet = metrics::encode(&outcome.days).map_err(|e| format!("Parquet : {e}"))?;
    let text = manifest::render(&scenario, &file.bytes, args.seed, &outcome, &parquet)
        .map_err(|e| format!("manifeste : {e}"))?;
    out_dir::write_new(&dir, "metrics.parquet", &parquet).map_err(|e| e.to_string())?;
    out_dir::write_new(&dir, "manifest.ron", text.as_bytes()).map_err(|e| e.to_string())?;
    println!(
        "ts-sim run : {} ({} jours, graine publique {}) : empreinte d'état {}",
        scenario.name,
        scenario.days(),
        args.seed,
        outcome.state_hash
    );
    Ok(())
}
