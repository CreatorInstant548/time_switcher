// spec: revues/architecture.md §8.5, §9.2
//! Wall time of the toy world in time skip, per thread count (issue #67).
//!
//! ```text
//! cargo bench -p ts_runner --bench time_skip --profile <dev|sim> -- [--days <n>] [--repeat <n>] [--threads <n>]...
//! ```
//!
//! The bench runs itself once per thread count (1, 2, 4 and 8 by default) and per
//! repetition (3 by default), interleaved, each run in **its own process** since the compute
//! pool is global to a process (L6). It prints, per thread count, the fastest and the median
//! run, then checks that every process ends on the same state hash: the thread count must
//! change the speed only, never the result (R1). The timed span is one straight `run_until`
//! over `--days` simulated days (365 by default); building the world (generator included) is
//! not timed. The fastest run is the most stable figure on a loaded machine.
//!
//! A measurement tool, not a test: the wall clock is read here, never in the kernel.

#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::disallowed_methods,
    reason = "bench tool: reads its command line and the wall clock, prints its results"
)]

use core::num::NonZeroU16;
use std::process::{Command, ExitCode};
use std::time::Instant;

use ts_kernel::{Sim, SimConfig, StopCondition, Threads};
use ts_toy::town::TownSpec;
use ts_toy::world::rules;
use ts_toy::{day_start, hex, public_test_key};

/// Public test seed of the golden values (`data/golden/ts_toy.json`).
const SEED: u64 = 1;
/// Thread counts measured by default.
const DEFAULT_THREADS: [u16; 4] = [1, 2, 4, 8];
/// Prefix of a child's result line.
const MARKER: &str = "ts-time-skip ";

struct Args {
    days: i64,
    repeat: u32,
    threads: Vec<u16>,
    child: bool,
}

fn parse() -> Result<Args, String> {
    let mut args = Args {
        days: 365,
        repeat: 3,
        threads: Vec::new(),
        child: false,
    };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        let mut value = |name: &str| it.next().ok_or_else(|| format!("{name} expects a value"));
        match a.as_str() {
            "--days" => {
                let v = value("--days")?;
                args.days = v.parse().map_err(|_| format!("invalid --days: {v}"))?;
            }
            "--repeat" => {
                let v = value("--repeat")?;
                args.repeat = v.parse().map_err(|_| format!("invalid --repeat: {v}"))?;
            }
            "--threads" => {
                let v = value("--threads")?;
                let n = v.parse().map_err(|_| format!("invalid --threads: {v}"))?;
                args.threads.push(n);
            }
            "--child" => args.child = true,
            // Added by `cargo bench`.
            "--bench" => {}
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    if args.threads.is_empty() {
        args.threads = DEFAULT_THREADS.to_vec();
    }
    if args.repeat == 0 {
        return Err("--repeat must be at least 1".to_owned());
    }
    Ok(args)
}

/// One measurement, in this process: `<ticks> <micros> <hash>`.
fn child(days: i64, threads: u16) -> Result<String, String> {
    let threads = match NonZeroU16::new(threads) {
        None => return Err("--threads must be at least 1".to_owned()),
        Some(n) if n.get() == 1 => Threads::Single,
        Some(n) => Threads::Fixed(n),
    };
    let cfg = SimConfig {
        threads,
        ..SimConfig::default()
    };
    let mut sim = Sim::build(cfg, &rules(TownSpec::PHASE0), &public_test_key(SEED))
        .map_err(|e| e.to_string())?;
    let start = Instant::now();
    let report = sim.run_until(StopCondition::At(day_start(days)));
    let micros = start.elapsed().as_micros();
    let hash = hex(sim.state_hash().global.as_bytes());
    Ok(format!("{} {micros} {hash}", report.ticks))
}

/// Runs one child process: `(ticks, micros, hash)`.
fn measure(exe: &std::path::Path, days: i64, threads: u16) -> Result<(u128, u128, String), String> {
    let out = Command::new(exe)
        .args(["--child", "--days", &days.to_string()])
        .args(["--threads", &threads.to_string()])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!(
            "{threads} threads: {} ({})",
            out.status,
            stderr.trim()
        ));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text
        .lines()
        .find_map(|l| l.strip_prefix(MARKER))
        .ok_or_else(|| {
            let stderr = String::from_utf8_lossy(&out.stderr);
            format!("{threads} threads: no result ({})", stderr.trim())
        })?;
    let malformed = || format!("malformed result: {line}");
    let mut f = line.split(' ');
    let ticks = f
        .next()
        .and_then(|v| v.parse().ok())
        .ok_or_else(malformed)?;
    let micros = f
        .next()
        .and_then(|v| v.parse().ok())
        .ok_or_else(malformed)?;
    let hash = f.next().ok_or_else(malformed)?.to_owned();
    Ok((ticks, micros, hash))
}

fn parent(args: &Args) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    println!(
        "toy world, {} residents, {} simulated days, seed {SEED}, {} run(s) per thread count",
        TownSpec::PHASE0.people,
        args.days,
        args.repeat
    );
    // Per thread count: the wall times of its runs, in microseconds.
    let mut runs: Vec<Vec<u128>> = vec![Vec::new(); args.threads.len()];
    let (mut ticks, mut hashes) = (None, Vec::new());
    for _ in 0..args.repeat {
        for (i, &n) in args.threads.iter().enumerate() {
            let (t, micros, hash) = measure(&exe, args.days, n)?;
            if *ticks.get_or_insert(t) != t {
                return Err(format!("{n} threads ran {t} ticks, another run {ticks:?}"));
            }
            runs[i].push(micros);
            hashes.push(hash);
        }
    }
    let ticks = ticks.unwrap_or(0);
    println!("threads       ticks   fastest (ms)   µs/tick   median (ms)   µs/tick");
    for (&n, times) in args.threads.iter().zip(&mut runs) {
        times.sort_unstable();
        let (fastest, median) = (times[0], times[times.len() / 2]);
        let per_tick = |micros: u128| micros.checked_div(ticks).unwrap_or(0);
        println!(
            "{n:>7} {ticks:>11} {:>14} {:>9} {:>13} {:>9}",
            fastest / 1000,
            per_tick(fastest),
            median / 1000,
            per_tick(median)
        );
    }
    if hashes.windows(2).any(|w| w[0] != w[1]) {
        return Err(format!(
            "the thread count changed the state hash: {hashes:?}"
        ));
    }
    if let Some(h) = hashes.first() {
        println!("same state hash in every process: {h}");
    }
    Ok(())
}

fn main() -> ExitCode {
    let result = parse().and_then(|args| {
        if args.child {
            let &[n] = &args.threads[..] else {
                return Err("--child takes exactly one --threads".to_owned());
            };
            child(args.days, n).map(|line| println!("{MARKER}{line}"))
        } else {
            parent(&args)
        }
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("time_skip: {e}");
            ExitCode::FAILURE
        }
    }
}
