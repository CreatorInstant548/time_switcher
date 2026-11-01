//! Stable identifiers: `StableId` and its allocator.

use core::fmt;

/// Leading bit: marks the namespace reserved for the map (P28, K9).
const MAP_BIT: u64 = 1 << 63;

// spec: revues/architecture.md §6.1 ; revues/generation.md §7.3, P28 ; carnet architecture-securite.md §9.9 K9
/// Durable identity of a referenceable entity, never reused.
///
/// Two namespaces, separated by **bit 63** (decided, K9):
/// - bit 63 at 0: entities created in play, indices `0..=2^63 − 1`, handed out by
///   [`StableIdAllocator`];
/// - bit 63 at 1: places of the generated map (parcels, buildings, streets...), only through
///   [`StableId::map`].
///
/// No public conversion from an integer: the allocator, [`StableId::map`] and
/// [`StableId::from_snapshot_raw`] are the only constructors.
///
/// **Never sent to the client** (architecture §6.1): it receives ephemeral handles.
///
/// Each `compile_fail` snippet below has a **witness** that compiles, so that it cannot fail
/// for another reason: stable rustdoc does not check their error code, which stays written for
/// review and for nightly.
///
/// ```
/// let _: ts_kernel::StableId = ts_kernel::StableId::from_snapshot_raw(5);
/// ```
///
/// ```compile_fail,E0277
/// let _: ts_kernel::StableId = 5_u64.into();
/// ```
///
/// ```compile_fail,E0423
/// let _ = ts_kernel::StableId(5);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableId(u64);

crate::assert_size!(StableId, 8);

impl StableId {
    /// Largest index of a namespace: `2^63 − 1`.
    pub const MAX_INDEX: u64 = MAP_BIT - 1;

    /// Identifier of the map place with index `index`, or `None` if `index` exceeds
    /// [`StableId::MAX_INDEX`]. The only access to the map namespace (P28).
    pub const fn map(index: u64) -> Option<Self> {
        if index <= Self::MAX_INDEX {
            Some(Self(MAP_BIT | index))
        } else {
            None
        }
    }

    /// True for an identifier of the map namespace.
    pub const fn is_map(self) -> bool {
        self.0 & MAP_BIT != 0
    }

    /// Index within its namespace, without bit 63.
    pub const fn index(self) -> u64 {
        self.0 & Self::MAX_INDEX
    }

    /// Decoding of a snapshot. **Reserved to `ts_persist`**: no other crate builds an
    /// identifier from an integer.
    pub const fn from_snapshot_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Raw value, for the encoding of a snapshot. **Reserved to `ts_persist`.**
    pub const fn to_snapshot_raw(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for StableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = if self.is_map() { "map" } else { "game" };
        write!(f, "StableId({space}:{})", self.index())
    }
}

// spec: revues/architecture.md §6.1 ; carnet architecture-securite.md §9.9 K9
/// Counter of the world that allocates the [`StableId`]s of entities created in play.
///
/// It is neither `Clone` nor `Default`: a copy, or an implicit reset
/// (`std::mem::take`, `unwrap_or_default`, `..Default::default()`, `#[derive(Default)]` on a
/// state that contains it), would allocate identifiers that were already handed out again. A
/// new world obtains it only through [`StableIdAllocator::for_new_world`]. It stops after
/// `2^63 − 1` (K9): beyond that, [`StableIdAllocator::allocate`] returns `None`, and never
/// overflows into the map namespace.
///
/// Witness, then the two refusals:
///
/// ```
/// let mut alloc = ts_kernel::StableIdAllocator::for_new_world();
/// let _ = alloc.allocate();
/// ```
///
/// ```compile_fail,E0599
/// let _ = ts_kernel::StableIdAllocator::default();
/// ```
///
/// ```compile_fail,E0599
/// let alloc = ts_kernel::StableIdAllocator::for_new_world();
/// let _ = alloc.clone();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct StableIdAllocator {
    /// Next index; `MAP_BIT` means "exhausted".
    next: u64,
}

impl StableIdAllocator {
    /// Allocator of a **new world**: the first identifier has index 0. A loaded world
    /// resumes its counter through [`StableIdAllocator::from_snapshot_raw`].
    pub const fn for_new_world() -> Self {
        Self { next: 0 }
    }

    /// New identifier, never handed out before, or `None` if the namespace is exhausted.
    pub const fn allocate(&mut self) -> Option<StableId> {
        if self.next <= StableId::MAX_INDEX {
            let id = StableId(self.next);
            self.next += 1;
            Some(id)
        } else {
            None
        }
    }

    /// Decoding of a snapshot: `None` if `next` exceeds `2^63`. **Reserved to `ts_persist`.**
    pub const fn from_snapshot_raw(next: u64) -> Option<Self> {
        if next <= MAP_BIT {
            Some(Self { next })
        } else {
            None
        }
    }

    /// Next index, for the encoding of a snapshot. **Reserved to `ts_persist`.**
    pub const fn to_snapshot_raw(&self) -> u64 {
        self.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_counts_from_zero_in_game_space() {
        let mut alloc = StableIdAllocator::for_new_world();
        let a = alloc.allocate().unwrap();
        let b = alloc.allocate().unwrap();
        assert_eq!((a.index(), b.index()), (0, 1));
        assert!(!a.is_map() && !b.is_map());
        assert_ne!(a, b);
        assert_eq!(alloc.to_snapshot_raw(), 2);
    }

    #[test]
    fn allocator_stops_at_last_index() {
        let mut alloc = StableIdAllocator::from_snapshot_raw(StableId::MAX_INDEX).unwrap();
        let last = alloc.allocate().unwrap();
        assert_eq!(last.to_snapshot_raw(), (1 << 63) - 1);
        assert!(!last.is_map());
        assert_eq!(alloc.allocate(), None);
        assert_eq!(alloc.allocate(), None);
        assert_eq!(alloc.to_snapshot_raw(), 1 << 63);
    }

    #[test]
    fn allocator_snapshot_rejects_map_space() {
        assert!(StableIdAllocator::from_snapshot_raw(1 << 63).is_some());
        assert!(StableIdAllocator::from_snapshot_raw((1 << 63) + 1).is_none());
        assert!(StableIdAllocator::from_snapshot_raw(u64::MAX).is_none());
    }

    #[test]
    fn map_space_is_bit_63() {
        let first = StableId::map(0).unwrap();
        assert!(first.is_map());
        assert_eq!(first.to_snapshot_raw(), 1 << 63);
        let last = StableId::map(StableId::MAX_INDEX).unwrap();
        assert_eq!(last.to_snapshot_raw(), u64::MAX);
        assert_eq!(last.index(), StableId::MAX_INDEX);
        assert_eq!(StableId::map(1 << 63), None);
        assert_eq!(StableId::map(u64::MAX), None);
    }

    #[test]
    fn game_and_map_spaces_never_meet() {
        let mut alloc = StableIdAllocator::for_new_world();
        let game = alloc.allocate().unwrap();
        let map = StableId::map(game.index()).unwrap();
        assert_eq!(game.index(), map.index());
        assert_ne!(game, map);
    }

    #[test]
    fn snapshot_raw_round_trips() {
        for raw in [0, 1, (1 << 63) - 1, 1 << 63, u64::MAX] {
            assert_eq!(StableId::from_snapshot_raw(raw).to_snapshot_raw(), raw);
        }
    }

    #[test]
    fn debug_names_the_space() {
        assert_eq!(
            format!("{:?}", StableId::map(12).unwrap()),
            "StableId(map:12)"
        );
        assert_eq!(
            format!("{:?}", StableId::from_snapshot_raw(7)),
            "StableId(game:7)"
        );
    }
}
