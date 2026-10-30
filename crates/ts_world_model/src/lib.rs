// spec: revues/generation.md §12 ; revues/architecture.md §5.2
//! Map model of time_switcher: integer-centimetre geometry and the three map parts.
//!
//! Phase 0 (T9w): geometry types, `quantize`, the kernel `StateHash` encoding, and the `public`,
//! `candidates` and `server` parts, separated from day one (P12). Depends on `ts_kernel` only;
//! the link to `ts_model` waits for T7a. No generator, map, street graph, collision or file
//! format yet.

#![forbid(unsafe_code)]

mod canonical;
mod cm;
mod level;
mod shapes;

pub mod candidates;
pub mod public;
pub mod server;

pub use cm::{Cm, GeometryError, MAX_ABS_CM, quantize};
pub use level::{HeightRange, Level, LevelKind};
pub use public::ClientShippable;
pub use shapes::{Aabb2, Point2, Polygon2, Segment2, Vec2};

use ts_kernel::StableId;

// spec: revues/generation.md P28 ; revues/architecture.md §6.1
/// Identifier of map entity `index`: always in the map namespace, never from the allocator
/// (N4).
pub fn map_id(index: u64) -> Result<StableId, GeometryError> {
    StableId::map(index).ok_or(GeometryError::MapIndexOutOfRange)
}

#[cfg(test)]
mod tests;
