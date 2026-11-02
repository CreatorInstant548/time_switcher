// spec: revues/architecture.md §18.4 ; carnet architecture-securite.md §9.16 VZ3, VZ4, VZ5, VZ7
//! `ts-viz`, the development visualisation tool (phase 0: `positions` on the toy world).
//!
//! ```text
//! ts-viz positions <scenario.ron> --seed <n> --out <dir> [--allow-outside-cache]
//! ```
//!
//! - Same scenario file, same `--seed` (a **public test seed**, Y2) and same rules for `--out`
//!   (strictly under `TS_CACHE`, Y3) as `ts-sim run`; it reuses `ts_runner` for all three.
//! - Writes `positions.parquet` (see `ts_viz::positions`), and nothing else. The conversion to
//!   a Rerun recording is the optional Python script `python/viz/positions_to_rrd.py`.
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
use ts_viz::positions;

const USAGE: &str =
    "usage: ts-viz positions <scenario.ron> --seed <n> --out <dir> [--allow-outside-cache]";

struct Args {
    scenario: PathBuf,
    seed: u64,
    out: PathBuf,
    outside_allowed: bool,
}

fn parse(mut args: impl Iterator<Item = OsString>) -> Result<Args, String> {
    if args.next().as_deref() != Some("positions".as_ref()) {
        return Err(USAGE.to_owned());
    }
    let (mut scenario, mut seed, mut out, mut outside_allowed) = (None, None, None, false);
    // Each option once: a repeated `--seed` must not silently replace the first.
    let once = |slot_is_set: bool, name: &str| match slot_is_set {
        true => Err(format!("{name} given twice\n{USAGE}")),
        false => Ok(()),
    };
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--seed") => {
                once(seed.is_some(), "--seed")?;
                let text = args.next().ok_or(USAGE)?;
                let n = text.to_str().and_then(|t| t.parse::<u64>().ok());
                seed = Some(n.ok_or("--seed expects an integer from 0 to 2^64 - 1")?);
            }
            Some("--out") => {
                once(out.is_some(), "--out")?;
                out = Some(PathBuf::from(args.next().ok_or(USAGE)?));
            }
            Some("--allow-outside-cache") => {
                once(outside_allowed, "--allow-outside-cache")?;
                outside_allowed = true;
            }
            Some(a) if a.starts_with('-') => return Err(format!("unknown option: {a}\n{USAGE}")),
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

/// `TS_CACHE`, else its default (A6): only the root of the output, never a source of key or
/// seed. Same rule as `ts-sim` (its copy is private to that binary).
#[allow(clippy::disallowed_methods)] // tool: reads TS_CACHE and HOME.
fn cache_dir() -> Result<PathBuf, String> {
    let var = |name| std::env::var_os(name).filter(|v| !v.is_empty());
    if let Some(cache) = var("TS_CACHE") {
        return Ok(PathBuf::from(cache));
    }
    if cfg!(windows) {
        return Ok(PathBuf::from(DEFAULT_CACHE));
    }
    var("HOME")
        .map(|home| PathBuf::from(home).join(DEFAULT_CACHE_UNDER_HOME))
        .ok_or_else(|| "neither TS_CACHE nor HOME is set".to_owned())
}

#[allow(clippy::disallowed_methods)] // tool: reads its command line.
fn main() -> ExitCode {
    let args = parse(std::env::args_os().skip(1));
    match args.and_then(|a| run_command(&a, &cache_dir()?)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ts-viz: {e}");
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
    out_dir::ensure_absent(&dir, &[positions::FILE_NAME]).map_err(|e| e.to_string())?;
    let encoded = positions::encode(&scenario, &file.bytes, args.seed)?;
    out_dir::write_new(&dir, positions::FILE_NAME, &encoded.bytes).map_err(|e| e.to_string())?;
    println!(
        "ts-viz positions: {} ({} days, public seed {}): {} rows, recording {}, state hash {}",
        scenario.name,
        scenario.days(),
        args.seed,
        encoded.rows,
        encoded.recording_id,
        encoded.state_hash
    );
    Ok(())
}
