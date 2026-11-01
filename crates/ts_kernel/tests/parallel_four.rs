// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L6
//! Reproducibility: 4 threads. Executable separate from `parallel_single.rs`, hence a
//! separate process, with its own thread pool.

mod common;

use core::num::NonZeroU16;
use ts_kernel::Threads;

#[test]
fn four_threads_match_reference() {
    let threads = Threads::Fixed(NonZeroU16::new(4).unwrap());
    let (hash, workers) = common::run(threads).unwrap();
    assert_eq!(bevy_tasks::ComputeTaskPool::get().thread_num(), 4);
    assert!(workers > 1, "par_iter ran on only {workers} thread");
    assert_eq!(hash, common::EXPECTED);
}

/// Snapshot at tick k, restore, then N ticks: same hash as k + N ticks straight.
#[test]
fn four_threads_round_trip() {
    let threads = Threads::Fixed(NonZeroU16::new(4).unwrap());
    for (straight, restored) in common::round_trips(threads, &[0, 9, 40], 30).unwrap() {
        assert_eq!(straight, restored);
    }
}
