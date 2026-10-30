// spec: carnet architecture-securite.md SEC0-24 (révisé), AS-36
//! Writes the initial corpus of the `decode` target (`ts_persist::fuzzing::seeds`) into the
//! directory given as argument, with two snapshots of a world of the fuzzing rules. The
//! corpus is regenerable and never committed.

use std::io;
use std::path::PathBuf;

use ts_kernel::rng::WorldKey;
use ts_kernel::{MemorySnapshot, Sim, SimConfig, SnapshotScope};

/// Key of the seed world: public, this world is a test fixture.
const SEED_KEY: [u8; 16] = *b"fuzz seed world\0";

fn main() -> io::Result<()> {
    let dir = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("usage: ts_persist_fuzz_seeds <corpus directory>"))?;
    let rules = ts_persist::fuzzing::rules();
    let mut sim = Sim::build(SimConfig::default(), &rules, &WorldKey::from_bytes(SEED_KEY))
        .map_err(|e| io::Error::other(e.to_string()))?;
    let first: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    for _ in 0..30 {
        sim.step();
    }
    let later: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    std::fs::create_dir_all(&dir)?;
    for (i, seed) in ts_persist::fuzzing::seeds(&[first, later]).iter().enumerate() {
        std::fs::write(dir.join(format!("seed-{i:02}")), seed)?;
    }
    Ok(())
}
