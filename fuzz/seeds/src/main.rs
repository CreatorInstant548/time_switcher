// spec: carnet architecture-securite.md SEC0-24 (révisé), §10
//! Writes the initial corpus of each fuzzing target into `<directory>/<target>`, the directory
//! `cargo fuzz run <target>` reads by default:
//! - `decode` (`ts_persist::fuzzing::seeds`): two snapshots of a world of the fuzzing rules;
//! - `content` (`ts_data::fuzzing::seeds`): the example cards of `data/content/` and
//!   hand-written inputs.
//!
//! The corpus is regenerable and never committed.

use std::io;
use std::path::{Path, PathBuf};

use ts_kernel::rng::WorldKey;
use ts_kernel::{MemorySnapshot, Sim, SimConfig, SnapshotScope};

/// Key of the seed world: public, this world is a test fixture.
const SEED_KEY: [u8; 16] = *b"fuzz seed world\0";
/// Example cards, the base of the `content` corpus.
const EXAMPLES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/content");

fn main() -> io::Result<()> {
    let dir = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("usage: ts_persist_fuzz_seeds <corpus directory>"))?;
    write(&dir.join("decode"), &decode_seeds()?)?;
    write(&dir.join("content"), &ts_data::fuzzing::seeds(&examples()?))
}

fn decode_seeds() -> io::Result<Vec<Vec<u8>>> {
    let rules = ts_persist::fuzzing::rules();
    let mut sim = Sim::build(
        SimConfig::default(),
        &rules,
        &WorldKey::from_bytes(SEED_KEY),
    )
    .map_err(|e| io::Error::other(e.to_string()))?;
    let first: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    for _ in 0..30 {
        sim.step().map_err(io::Error::other)?;
    }
    let later: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    Ok(ts_persist::fuzzing::seeds(&[first, later]))
}

/// The `*.ron` files of `data/content/` and its sub-directories (extension in any case, as
/// the loader reads them), sorted by path.
fn examples() -> io::Result<Vec<Vec<u8>>> {
    let mut paths = Vec::new();
    let mut pending = vec![PathBuf::from(EXAMPLES)];
    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            let path = entry.path();
            if kind.is_dir() {
                pending.push(path);
            } else if kind.is_file()
                && path
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("ron"))
            {
                paths.push(path);
            }
        }
    }
    paths.sort();
    if paths.is_empty() {
        return Err(io::Error::other("no example card in data/content"));
    }
    paths.iter().map(std::fs::read).collect()
}

fn write(dir: &Path, seeds: &[Vec<u8>]) -> io::Result<()> {
    std::fs::create_dir_all(dir)?;
    for (i, seed) in seeds.iter().enumerate() {
        std::fs::write(dir.join(format!("seed-{i:02}")), seed)?;
    }
    Ok(())
}
