// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L6
//! Reproducibility: 4 threads. Executable separate from `parallel_single.rs`, hence a
//! separate process, with its own thread pool.

mod common;

use core::num::NonZeroU16;
use ts_kernel::{ParallelPhases, Threads};

#[test]
fn four_threads_match_reference() {
    let threads = Threads::Fixed(NonZeroU16::new(4).unwrap());
    let (hash, workers) = common::run(threads).unwrap();
    assert_eq!(bevy_tasks::ComputeTaskPool::get().thread_num(), 4);
    assert!(workers > 1, "par_iter ran on only {workers} thread");
    assert_eq!(hash, common::EXPECTED);
}

// spec: revues/architecture.md §9.1, §9.2
/// The executor of each phase changes the speed only: every policy gives the reference
/// fingerprint, also reached on 1 thread, and the policy does pick the executor. The
/// placement checks rely on `bevy_ecs =0.20.0`: its parallel executor never runs a system
/// on the calling thread; revisit them when `bevy_ecs` changes.
#[test]
fn executor_choice_does_not_change_result() {
    let threads = Threads::Fixed(NonZeroU16::new(4).unwrap());
    let always = common::switching(threads, ParallelPhases::Always).unwrap();
    assert_eq!(always.hash, common::SWITCHING_EXPECTED);
    assert_eq!((always.on_caller, always.on_pool > 0), (0, true));
    let never = common::switching(threads, ParallelPhases::FromWork(u32::MAX)).unwrap();
    assert_eq!(never.hash, common::SWITCHING_EXPECTED);
    assert_eq!((never.on_caller > 0, never.on_pool), (true, 0));
    let mixed = common::switching(threads, ParallelPhases::FromWork(common::SWITCHING_FROM));
    let mixed = mixed.unwrap();
    assert_eq!(mixed.hash, common::SWITCHING_EXPECTED);
    assert!(
        mixed.on_caller > 0 && mixed.on_pool > 0,
        "the executor never switched"
    );
    for policy in [ParallelPhases::FromWork(0), ParallelPhases::default()] {
        assert_eq!(
            common::switching(threads, policy).unwrap().hash,
            common::SWITCHING_EXPECTED,
            "{policy:?}"
        );
    }
}

/// Snapshot at tick k, restore, then N ticks: same hash as k + N ticks straight.
#[test]
fn four_threads_round_trip() {
    let threads = Threads::Fixed(NonZeroU16::new(4).unwrap());
    for (straight, restored) in common::round_trips(threads, &[0, 9, 40], 30).unwrap() {
        assert_eq!(straight, restored);
    }
}
