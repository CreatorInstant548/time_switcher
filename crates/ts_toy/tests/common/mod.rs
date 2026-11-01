// spec: revues/architecture.md §22, §9.2, §11.2, §12.1 ; carnet architecture-securite.md §9.10 L6
//! The determinism proof of the toy world, shared by `one_thread.rs` and `four_threads.rs`.
//!
//! Each test file is a separate executable, hence a separate process with its own thread
//! pool (L6). Each prints its results on lines `ts-toy-result <key> <value>`;
//! `cargo xtask determinism` runs both, checks that they agree, and compares them with
//! `data/golden/ts_toy.json`. Inside one process, the tests check that two runs agree and
//! that a snapshot round trip through `ts_persist` gives the uninterrupted run.

#![allow(
    clippy::print_stdout,
    reason = "results read by cargo xtask determinism"
)]

use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};
use ts_kernel::{
    LodComplete, MemorySnapshot, ParallelPhases, RunReport, Sim, SimConfig, SimTime, SnapshotScope,
    StableIndex, StateLayout, StopCondition, Threads, state_hash_digest,
};
use ts_persist::{Codec, Limits, decode_snapshot, encode_snapshot};
use ts_toy::town::{Execution, TownSpec, generate};
use ts_toy::world::{Commuter, STROLL_AT, STROLLERS, Tally, Whereabouts, rules};
use ts_toy::{day_start, hex, public_test_key};

/// Public test seed of the golden values.
pub const SEED: u64 = 1;

/// One version of the proof: its length and the date of the snapshot.
pub struct Version {
    /// Prefix of the result keys.
    pub label: &'static str,
    /// Simulated days.
    pub days: i64,
    /// Day of the snapshot: a stroll day, so that the snapshot holds residents in full
    /// simulation (`LodComplete`).
    pub snapshot_day: i64,
}

/// Short version, run by `cargo xtask check` (Y5). Test parameters.
pub const SHORT: Version = Version {
    label: "short",
    days: 14,
    snapshot_day: 7,
};

/// One simulated year, run every week in CI (Y5). Test parameters.
pub const YEAR: Version = Version {
    label: "year",
    days: 365,
    snapshot_day: 182,
};

/// Prints one result for `cargo xtask determinism`.
pub fn report(key: &str, value: impl std::fmt::Display) {
    println!("ts-toy-result {key} {value}");
}

/// Sets up the process thread pool, as `Sim::build` would, so that the generator alone
/// runs on it.
pub fn pool(threads: Threads) {
    let n = threads.count();
    let pool = ComputeTaskPool::get_or_init(|| TaskPoolBuilder::new().num_threads(n).build());
    assert_eq!(pool.thread_num(), n);
}

/// With [`ParallelPhases::Always`]: on 4 threads, every phase runs on the parallel executor,
/// as before issue #67, the most demanding case for the proof. The default policy reaches
/// the same values (`cargo bench -p ts_toy --bench time_skip`).
fn config(threads: Threads) -> SimConfig {
    SimConfig {
        threads,
        parallel_phases: ParallelPhases::Always,
        ..SimConfig::default()
    }
}

fn build(threads: Threads) -> Sim {
    Sim::build(
        config(threads),
        &rules(TownSpec::PHASE0),
        &public_test_key(SEED),
    )
    .unwrap()
}

fn run(sim: &mut Sim, until: SimTime) -> RunReport {
    sim.run_until(StopCondition::At(until))
}

fn in_full_simulation(sim: &Sim) -> usize {
    let world = sim.world();
    let index = world.resource::<StableIndex>();
    index
        .iter()
        .filter(|&(_, e)| world.get::<LodComplete>(e).is_some())
        .count()
}

/// Checks the end state: every resident made one round trip a day and is home.
fn check_end(sim: &Sim, v: &Version) {
    let world = sim.world();
    let index = world.resource::<StableIndex>();
    assert_eq!(index.len(), TownSpec::PHASE0.people as usize);
    for (id, e) in index.iter() {
        let w = world.get::<Whereabouts>(e).unwrap();
        assert!(world.get::<Commuter>(e).is_some());
        assert_eq!(i64::from(w.trips), v.days, "{id:?}");
    }
    let tally = world.resource::<Tally>();
    assert_eq!((tally.at_work, tally.travelling), (0, 0));
    assert_eq!(in_full_simulation(sim), 0);
}

/// The proof for one version on `threads`: two runs give the same hash, and a snapshot taken
/// in the middle, encoded and decoded by `ts_persist`, then restored and run to the end,
/// gives that hash too. Prints the results.
pub fn prove(v: &Version, threads: Threads) {
    let end = day_start(v.days);
    let snapshot_at = day_start(v.snapshot_day).add_millis(STROLL_AT * 60_000 + 10_000);

    // First run, with a snapshot on the way.
    let mut first = build(threads);
    let rules_hex = hex(first.rules_fingerprint().as_bytes());
    let before = run(&mut first, snapshot_at);
    assert_eq!(in_full_simulation(&first), STROLLERS);
    let snap: MemorySnapshot = first.snapshot(SnapshotScope::Full);
    let bytes = encode_snapshot(&snap, Codec::Zstd, &Limits::default()).unwrap();
    let at_snapshot = first.state_hash();
    let after = run(&mut first, end);
    check_end(&first, v);

    // Second run, straight through.
    let mut second = build(threads);
    let straight = run(&mut second, end);
    assert_eq!(second.state_hash(), first.state_hash(), "two runs differ");
    assert_eq!(straight.ticks, before.ticks + after.ticks);

    // Round trip through the bytes, at a tick k > 0, during a stroll.
    let cfg = config(threads);
    let rules = rules(TownSpec::PHASE0);
    let layout = StateLayout::of(&cfg, &rules).unwrap();
    let decoded = decode_snapshot(&bytes, &Limits::default(), &layout).unwrap();
    let mut restored = Sim::restore(&decoded, cfg, &rules, &public_test_key(SEED)).unwrap();
    assert_eq!(
        restored.state_hash(),
        at_snapshot,
        "right after the restore"
    );
    assert_eq!(in_full_simulation(&restored), STROLLERS);
    run(&mut restored, end);
    assert_eq!(
        restored.state_hash(),
        first.state_hash(),
        "after the restore"
    );

    // The time skip did its work: most ticks were skipped.
    assert!(
        straight.skipped_ticks > 100 * straight.ticks,
        "{straight:?}"
    );

    let label = v.label;
    report(&format!("{label}.rules"), rules_hex);
    report(
        &format!("{label}.world"),
        hex(first.state_hash().global.as_bytes()),
    );
    report(&format!("{label}.ticks"), straight.ticks);
    report(&format!("{label}.skipped"), straight.skipped_ticks);
}

/// The generator on the compute pool and inline gives the same town. Prints its digest.
pub fn generator(threads: Threads) {
    pool(threads);
    let keys = ts_kernel::rng::StreamKeys::derive(&public_test_key(SEED));
    let pooled = generate(&TownSpec::PHASE0, &keys, Execution::Pool);
    let inline = generate(&TownSpec::PHASE0, &keys, Execution::Inline);
    assert_eq!(pooled, inline);
    assert_eq!(pooled.residents.len(), TownSpec::PHASE0.people as usize);
    report("generator", hex(state_hash_digest(&pooled).as_bytes()));
    report("process.threads", threads.count());
}
