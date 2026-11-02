// spec: revues/architecture.md §19.2, §19.3, §8.1 ; carnet architecture-securite.md §9.15 Y2
//! Runs the toy world day by day and records a few indicators per simulated day.

use ts_kernel::{Sim, SimConfig, StopCondition};
use ts_toy::town::TownSpec;
use ts_toy::world::{Tally, rules};
use ts_toy::{day_start, hex, public_test_key};

/// Time of the daily sample, in milliseconds after midnight (08:00: residents are then at
/// work, on the way or still home). A parameter of the tool, not a game value.
pub const SAMPLE_AT_MS: i64 = 8 * 3_600_000;

/// Indicators of one simulated day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayRow {
    /// Day, from 0.
    pub day: i64,
    /// Residents at work at 08:00.
    pub at_work_0800: i64,
    /// Residents on the way at 08:00.
    pub travelling_0800: i64,
    /// Departures during the day.
    pub departures: i64,
    /// Arrivals during the day.
    pub arrivals: i64,
    /// Ticks run during the day.
    pub ticks: i64,
    /// Ticks skipped by the time skip during the day.
    pub skipped_ticks: i64,
}

/// What a run produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    /// Fingerprint of the rules, in hexadecimal.
    pub rules: String,
    /// Global state hash at the end, in hexadecimal.
    pub state_hash: String,
    /// Ticks run.
    pub ticks: u64,
    /// Ticks skipped.
    pub skipped_ticks: u64,
    /// One row per simulated day.
    pub days: Vec<DayRow>,
}

fn signed(n: u64) -> i64 {
    i64::try_from(n).unwrap_or(i64::MAX)
}

/// Runs the toy world (`TownSpec::PHASE0`) for `days` simulated days with the **public test
/// seed** `seed` (Y2: the key comes from that seed only, through `ts_toy::public_test_key`).
pub fn run_toy(days: u32, seed: u64) -> Result<Outcome, String> {
    let mut sim = Sim::build(
        SimConfig::default(),
        &rules(TownSpec::PHASE0),
        &public_test_key(seed),
    )
    .map_err(|e| format!("construction du monde impossible : {e}"))?;
    let tally = |sim: &Sim| sim.world().resource::<Tally>().clone();
    let mut rows = Vec::with_capacity(usize::try_from(days).unwrap_or(0));
    let (mut ticks, mut skipped) = (0_u64, 0_u64);
    for day in 0..i64::from(days) {
        let before = tally(&sim);
        let morning = sim
            .run_until(StopCondition::At(day_start(day).add_millis(SAMPLE_AT_MS)))
            .map_err(|e| e.to_string())?;
        let sample = tally(&sim);
        let rest = sim
            .run_until(StopCondition::At(day_start(day + 1)))
            .map_err(|e| e.to_string())?;
        let after = tally(&sim);
        let day_ticks = morning.ticks + rest.ticks;
        let day_skipped = morning.skipped_ticks + rest.skipped_ticks;
        ticks += day_ticks;
        skipped += day_skipped;
        rows.push(DayRow {
            day,
            at_work_0800: i64::from(sample.at_work),
            travelling_0800: i64::from(sample.travelling),
            departures: signed(after.departures - before.departures),
            arrivals: signed(after.arrivals - before.arrivals),
            ticks: signed(day_ticks),
            skipped_ticks: signed(day_skipped),
        });
    }
    Ok(Outcome {
        rules: hex(sim.rules_fingerprint().as_bytes()),
        state_hash: hex(sim.state_hash().global.as_bytes()),
        ticks,
        skipped_ticks: skipped,
        days: rows,
    })
}
