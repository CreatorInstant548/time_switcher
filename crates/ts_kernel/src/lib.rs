// spec: 05-architecture.md §2.3 ; revues/architecture.md §6
//! Noyau de simulation de time_switcher, sans affichage ni E/S.
//!
//! Phase 0 : profil de construction (SEC0-20), temps, identifiants stables, argent, aléa à clé,
//! ordonnanceur à événements datés, phases du tick, greffons, boucle de simulation, empreinte
//! d'état, instantané et budgets de calcul. Chemin
//! sensible : ce fichier déclare le module `build_profile`, qui porte les `compile_error!`.

#![forbid(unsafe_code)]

// spec: attendus-mesurables.md M7
/// Vérifie à la compilation que `size_of::<Type>()` vaut `n` octets (M7).
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
            ::core::concat!("taille inattendue pour ", ::core::stringify!($t), " (M7)"),
        );
    };
}

mod budget;
mod build_profile;
mod id;
mod money;
mod phase;
mod plugin;
pub mod rng;
mod scheduler;
mod sim;
mod state;
mod time;

/// `bevy_ecs`, réexporté pour que les crates de règles en partagent la version.
pub use bevy_ecs;

pub use budget::WorkBudget;
pub use build_profile::{BUILD_PROFILE, DevFeature, active_dev_features};
pub use id::{StableId, StableIdAllocator};
pub use money::{BasisPoints, Money, div_round_half_even};
pub use phase::Phase;
pub use plugin::{Plugin, RulesFingerprint, SimBuilder};
pub use scheduler::{Scheduled, Scheduler};
pub use sim::{
    BuildError, DueEvents, EventQueue, LodComplete, MemorySnapshot, RestoreError, RunReport, Sim,
    SimClock, SimConfig, Snapshot, SnapshotScope, StableIds, StepReport, StopCondition, Threads,
    TimedEvent, WorldStreams,
};
pub use state::{
    KERNEL_FAMILY, StableEntity, StableIndex, StateDigest, StateHash, StateHashes, StateWriter,
};
pub use time::{DayOrigin, SimDay, SimTime};
