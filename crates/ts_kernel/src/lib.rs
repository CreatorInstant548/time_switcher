// spec: 05-architecture.md §2.3 ; revues/architecture.md §6
//! Noyau de simulation de time_switcher, sans affichage ni E/S.
//!
//! Phase 0 : profil de construction (SEC0-20), temps, identifiants stables et argent. Chemin
//! sensible : ce fichier déclare le module `build_profile`, qui porte les `compile_error!`.

#![forbid(unsafe_code)]

// spec: attendus-mesurables.md M7
/// Vérifie à la compilation que `size_of::<Type>()` vaut `n` octets (M7).
///
/// ```
/// ts_kernel::assert_size!(u32, 4);
/// ```
///
/// ```compile_fail
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

mod build_profile;
mod id;
mod money;
mod time;

pub use build_profile::{BUILD_PROFILE, DevFeature, active_dev_features};
pub use id::{StableId, StableIdAllocator};
pub use money::{BasisPoints, Money, div_round_half_even};
pub use time::{SimDay, SimTime};
