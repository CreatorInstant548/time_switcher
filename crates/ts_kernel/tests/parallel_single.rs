// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L6
//! Reproducibility: 1 thread. Executable separate from `parallel_four.rs`, hence a
//! separate process, with its own thread pool.

mod common;

use ts_kernel::Threads;

#[test]
fn one_thread_matches_reference() {
    let (hash, _) = common::run(Threads::Single).unwrap();
    assert_eq!(bevy_tasks::ComputeTaskPool::get().thread_num(), 1);
    assert_eq!(hash, common::EXPECTED);
}

/// Snapshot at tick k, restore, then N ticks: same hash as k + N ticks straight.
#[test]
fn one_thread_round_trips() {
    for (straight, restored) in common::round_trips(Threads::Single, &[0, 9, 40], 30).unwrap() {
        assert_eq!(straight, restored);
    }
}
