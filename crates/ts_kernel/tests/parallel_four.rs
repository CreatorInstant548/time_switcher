// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L6
//! Reproductibilité : 4 fils. Exécutable distinct de `parallel_single.rs`, donc processus
//! distinct, avec son propre groupe de fils.

mod common;

use core::num::NonZeroU16;
use ts_kernel::Threads;

#[test]
fn four_threads_match_reference() {
    let threads = Threads::Fixed(NonZeroU16::new(4).unwrap());
    let (hash, workers) = common::run(threads).unwrap();
    assert_eq!(bevy_tasks::ComputeTaskPool::get().thread_num(), 4);
    assert!(workers > 1, "par_iter n'a tourné que sur {workers} fil");
    assert_eq!(hash, common::EXPECTED);
}
