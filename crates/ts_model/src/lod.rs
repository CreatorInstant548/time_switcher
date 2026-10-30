//! Level-of-detail markers.

// The `Component` derive names `bevy_ecs::…`: the kernel re-export stands in for the crate.
use ts_kernel::bevy_ecs;
use ts_kernel::bevy_ecs::component::Component;

// spec: revues/architecture.md §6.5, §10.2 ; carnet architecture-securite.md §9.10 L2
/// Marker "full simulation", **re-exported** from the kernel, which needs it for the time skip
/// (architecture §8.1). Never redefined here.
pub use ts_kernel::LodComplete;

// spec: revues/architecture.md §6.5, §10.1, §10.2
/// Marker "simplified simulation": no physics, moves on the street and room graph with
/// durations ([`crate::Location::InPlace`] or [`crate::Location::InTransit`]).
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LodSimplified;

// spec: revues/architecture.md §6.5, §10.1, §10.2
/// Marker "statistical simulation": the character is in a place according to its agenda, and
/// each agenda block is resolved at once.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LodStatistical;
