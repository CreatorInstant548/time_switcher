//! `Location`: where a character is, at each level of detail.

use ts_kernel::bevy_ecs::component::Component;
// `bevy_ecs`: the `Component` derive names `bevy_ecs::…`, and the kernel re-export stands in.
use ts_kernel::{SimTime, StableId, bevy_ecs};

// spec: revues/architecture.md §6.5, §10.1 ; revues/generation.md P13
/// Position of a character (component).
///
/// - [`Location::Physical`] in full simulation: exact position.
/// - [`Location::InPlace`] or [`Location::InTransit`] in simplified and statistical
///   simulation, which reason in places and trips. The exact position is rebuilt only when the
///   character goes up to full simulation (architecture §10.2).
///
/// Minimal phase 0 form (decided, T7a): only the fields whose types exist today. The activity
/// in a place (`ActivityId`) and the route of a trip come with the activity catalogue and
/// with `ts_mobility`. Progress is written as **dates**, never as a fraction updated every
/// tick: a per-tick counter would break the time-skip contract of `ts_kernel::Plugin`.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum Location {
    /// Full simulation: point on a floor, and the spatial grid cell that holds it.
    Physical {
        /// Point on the floor, in metres.
        pos: Position,
        /// Spatial grid cell (64 m cells, architecture §6.5) that holds `pos`; whoever moves
        /// `pos` keeps it in step.
        cell: GridCell,
        /// Floor (level) of the physical geometry.
        floor: Floor,
    },
    /// In a place (building, room, parcel…), since a date.
    InPlace {
        /// The place, a map or game identity.
        place: StableId,
        /// Arrival in the place.
        since: SimTime,
    },
    /// On the way between two places.
    InTransit {
        /// Place of departure.
        from: StableId,
        /// Destination.
        to: StableId,
        /// Departure date.
        departed: SimTime,
        /// Expected arrival date, never before `departed`.
        arrives: SimTime,
    },
}

// spec: revues/architecture.md §6.3 ; revues/generation.md P13
/// Point on a floor, in metres (`f32`, 1 unit = 1 m, architecture §6.3).
///
/// Two coordinates (decided, T7a): the physical geometry is made of discrete floors with 2D
/// collisions per floor (P13), so the height is the [`Floor`], not a third coordinate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// East, in metres.
    pub x: f32,
    /// North, in metres.
    pub y: f32,
}

// spec: revues/architecture.md §6.5, §10.2
/// Cell of the spatial grid, by integer coordinates (the cell size belongs to the grid).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridCell {
    /// Column.
    pub x: i32,
    /// Row.
    pub y: i32,
}

// spec: revues/generation.md P13 ; domaines/ville.md
/// Floor of the physical geometry: 0 for the outdoor ground and ground floors, positive
/// upstairs, negative below ground (cellars and underground car park at −1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Floor(pub i8);

impl Floor {
    /// Outdoor ground and ground floors.
    pub const GROUND: Self = Self(0);
}
