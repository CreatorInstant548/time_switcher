// spec: revues/architecture.md §9.2, §11.2 ; carnet architecture-securite.md §9.10 L6
//! Determinism proof of the toy world on 1 thread. A separate executable from
//! `four_threads.rs`, hence a separate process with its own thread pool.

#[cfg(test)]
mod common;

use ts_kernel::Threads;

const THREADS: Threads = Threads::Single;

#[test]
fn short_version() {
    common::prove(&common::SHORT, THREADS);
}

#[test]
fn generator() {
    common::generator(THREADS);
}

#[test]
#[ignore = "one simulated year: cargo xtask determinism --full (weekly in CI)"]
fn one_year() {
    common::prove(&common::YEAR, THREADS);
}
