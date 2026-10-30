// spec: revues/architecture.md §5.1, §5.2
//! Shared types of time_switcher: components, resources, intents, commands and events.
//!
//! Data only, no system and no game rule. Rule crates communicate through these types,
//! never through direct calls (architecture §5.2). This crate depends on `ts_kernel` only.
//!
//! Phase 0 contract (T7a):
//! - [`PlayerId`], the opaque per-world pseudonym of a player (S12);
//! - the public pilot markers [`AiDriven`] and [`AiObserving`]: the `Pilot` component itself
//!   stays private to `ts_pilot` (architecture §6.6, R13);
//! - the level-of-detail markers, [`LodComplete`] being the kernel's own;
//! - a minimal [`Location`];
//! - the closed skeletons of [`Intent`], [`Command`] and [`Event`], and the closed
//!   [`Rejection`] codes.

#![forbid(unsafe_code)]

mod exchange;
mod location;
mod lod;
mod pilot;
mod player;

pub use exchange::{Command, Event, Intent, Rejection};
pub use location::{Floor, GridCell, Location, Position};
pub use lod::{LodComplete, LodSimplified, LodStatistical};
pub use pilot::{AiDriven, AiObserving};
pub use player::PlayerId;
