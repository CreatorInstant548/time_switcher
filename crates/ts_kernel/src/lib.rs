// spec: 05-architecture.md §2.3 ; revues/architecture.md §6
//! Simulation kernel of time_switcher, with no display and no I/O.
//!
//! Phase 0: build profile (SEC0-20), time, stable identifiers, money, keyed randomness,
//! dated-event scheduler, tick phases, plugins, simulation loop, state fingerprint,
//! snapshot, canonical decoder and compute budgets. Sensitive path: this file declares the
//! `build_profile` module, which carries the `compile_error!`s, and the `canonical` module,
//! which reads untrusted bytes (AS-36).

#![forbid(unsafe_code)]

// spec: attendus-mesurables.md M7
/// Checks at compile time that `size_of::<Type>()` is `n` bytes (M7).
///
/// ```
/// ts_kernel::assert_size!(u32, 4);
/// ```
///
/// ```compile_fail,E0080
/// ts_kernel::assert_size!(u32, 8);
/// ```
#[macro_export]
macro_rules! assert_size {
    ($t:ty, $n:expr) => {
        const _: () = ::core::assert!(
            ::core::mem::size_of::<$t>() == $n,
            ::core::concat!("unexpected size for ", ::core::stringify!($t), " (M7)"),
        );
    };
}

mod budget;
mod build_profile;
mod canonical;
mod id;
mod money;
mod phase;
mod plugin;
pub mod rng;
mod scheduler;
mod sim;
mod state;
mod time;

/// `bevy_ecs`, re-exported so that the rules crates share its version.
pub use bevy_ecs;

pub use budget::WorkBudget;
pub use build_profile::{BUILD_PROFILE, DevFeature, active_dev_features};
pub use canonical::{CanonicalError, CanonicalLimits, StateDecode, StateReader};
pub use id::{StableId, StableIdAllocator};
pub use money::{BasisPoints, Money, div_round_half_even};
pub use phase::Phase;
pub use plugin::{Plugin, RulesFingerprint, SimBuilder};
pub use scheduler::{Scheduled, Scheduler};
pub use sim::{
    BuildError, DueEvents, EventQueue, LodComplete, MemorySnapshot, RestoreError, RunReport, Sim,
    SimClock, SimConfig, Snapshot, SnapshotScope, StableIds, StateLayout, StepReport,
    StopCondition, Threads, TimedEvent, WorldStreams,
};
pub use state::{
    KERNEL_FAMILY, StableEntity, StableIndex, StateDigest, StateHash, StateHashes, StateWriter,
    state_hash_bytes, state_hash_digest,
};
pub use time::{DayOrigin, SimDay, SimTime};
