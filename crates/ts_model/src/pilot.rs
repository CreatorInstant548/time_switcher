//! Public pilot markers: `AiDriven` and `AiObserving`.

// The `Component` derive names `bevy_ecs::…`: the kernel re-export stands in for the crate.
use ts_kernel::bevy_ecs;
use ts_kernel::bevy_ecs::component::Component;

// spec: revues/architecture.md §6.6 ; carnet architecture-securite.md §9.11 M1
/// Marker: the AI produces this character's intents.
///
/// With [`AiObserving`], one of the only two pilot facts a rule may read. The `Pilot`
/// component (`Ai`, `Player`, `External`) and its content are private to `ts_pilot`: other
/// crates cannot even name it, so "the world does not know the players" is checked by the
/// compiler (R13). Only `ts_pilot` adds or removes these markers.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AiDriven;

// spec: revues/architecture.md §6.6 ; carnet architecture-securite.md §9.11 M1
/// Marker: the AI runs in observation only (pilot `Player`, or `External` with observation):
/// it computes what it would have done, without producing intents.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AiObserving;
