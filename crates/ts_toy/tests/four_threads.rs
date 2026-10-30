// spec: revues/architecture.md §9.2, §11.2 ; carnet architecture-securite.md §9.10 L6
//! Determinism proof of the toy world on 4 threads. A separate executable from
//! `one_thread.rs`, hence a separate process with its own thread pool.

#[cfg(test)]
mod common;

use core::num::NonZeroU16;

use ts_kernel::Threads;

fn threads() -> Threads {
    Threads::Fixed(NonZeroU16::MIN.saturating_add(3))
}

#[test]
fn short_version() {
    common::prove(&common::SHORT, threads());
}

#[test]
fn generator() {
    common::generator(threads());
}

#[test]
#[ignore = "one simulated year: cargo xtask determinism --full (weekly in CI)"]
fn one_year() {
    common::prove(&common::YEAR, threads());
}
