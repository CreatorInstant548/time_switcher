// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L6
//! Reproducibility: 1 thread. Executable separate from `parallel_four.rs`, hence a
//! separate process, with its own thread pool.

mod common;

use ts_kernel::{ParallelPhases, Threads};

#[test]
fn one_thread_matches_reference() {
    let (hash, _) = common::run(Threads::Single).unwrap();
    assert_eq!(bevy_tasks::ComputeTaskPool::get().thread_num(), 1);
    assert_eq!(hash, common::EXPECTED);
}

/// The switching world reaches on 1 thread the fingerprint that every policy reaches on 4
/// (issue #67); the policy is ignored there, every phase runs on the calling thread.
#[test]
fn one_thread_switching_world() {
    let mixed = ParallelPhases::FromWork(common::SWITCHING_FROM);
    for policy in [ParallelPhases::Always, mixed, ParallelPhases::default()] {
        let run = common::switching(Threads::Single, policy).unwrap();
        assert_eq!(run.hash, common::SWITCHING_EXPECTED, "{policy:?}");
        assert_eq!((run.on_caller > 0, run.on_pool), (true, 0), "{policy:?}");
    }
}

/// Snapshot at tick k, restore, then N ticks: same hash as k + N ticks straight.
#[test]
fn one_thread_round_trips() {
    for (straight, restored) in common::round_trips(Threads::Single, &[0, 9, 40], 30).unwrap() {
        assert_eq!(straight, restored);
    }
}
