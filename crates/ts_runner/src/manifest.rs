// spec: revues/architecture.md §19.2 ; carnet architecture-securite.md SEC0-26, §9.15 Y2
//! `manifest.ron` of a run: versions, scenario, **public** test seed, fingerprint of the
//! rules, final state hash, files written. Never a local path, a user or a machine name, nor
//! the wall-clock time (SEC0-26): the same run gives the same bytes on any machine.

use serde::Serialize;
use ts_toy::{PUBLIC_TEST_SEED_CONTEXT, hex};

use crate::run::Outcome;
use crate::scenario::Scenario;

/// Version of the manifest layout.
pub const FORMAT: u32 = 1;

#[derive(Debug, Serialize)]
struct Manifest<'a> {
    format: u32,
    runner: &'static str,
    runner_version: &'static str,
    scenario: &'a str,
    scenario_blake3: String,
    world: &'static str,
    days: u32,
    seed: Seed,
    rules_fingerprint: &'a str,
    state_hash: &'a str,
    ticks: u64,
    skipped_ticks: u64,
    files: Vec<OutFile>,
}

#[derive(Debug, Serialize)]
struct Seed {
    kind: &'static str,
    value: u64,
    derivation: &'static str,
}

#[derive(Debug, Serialize)]
struct OutFile {
    name: &'static str,
    blake3: String,
    rows: usize,
}

/// The text of `manifest.ron`.
pub fn render(
    scenario: &Scenario,
    scenario_bytes: &[u8],
    seed: u64,
    outcome: &Outcome,
    metrics: &[u8],
) -> Result<String, ron::Error> {
    let manifest = Manifest {
        format: FORMAT,
        runner: "ts-sim run",
        runner_version: env!("CARGO_PKG_VERSION"),
        scenario: &scenario.name,
        scenario_blake3: hex(blake3::hash(scenario_bytes).as_bytes()),
        world: "toy_world (ts_toy, test plugin, D-DEV-02), TownSpec::PHASE0",
        days: scenario.days(),
        seed: Seed {
            kind: "public_test",
            value: seed,
            derivation: PUBLIC_TEST_SEED_CONTEXT,
        },
        rules_fingerprint: &outcome.rules,
        state_hash: &outcome.state_hash,
        ticks: outcome.ticks,
        skipped_ticks: outcome.skipped_ticks,
        files: vec![OutFile {
            name: "metrics.parquet",
            blake3: hex(blake3::hash(metrics).as_bytes()),
            rows: outcome.days.len(),
        }],
    };
    let pretty = ron::ser::PrettyConfig::new().new_line("\n".to_owned());
    ron::ser::to_string_pretty(&manifest, pretty).map(|mut s| {
        s.push('\n');
        s
    })
}
