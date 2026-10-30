// spec: carnet architecture-securite.md SEC0-24 (révisé)
//! Writes the initial corpus of the `decode` target (`ts_persist::fuzzing::seeds`) into the
//! directory given as argument. The corpus is regenerable and never committed.

use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let dir = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("usage: ts_persist_fuzz_seeds <corpus directory>"))?;
    std::fs::create_dir_all(&dir)?;
    for (i, seed) in ts_persist::fuzzing::seeds().iter().enumerate() {
        std::fs::write(dir.join(format!("seed-{i:02}")), seed)?;
    }
    Ok(())
}
