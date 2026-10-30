//! Storeys of the stacked physical model and height ranges.

use crate::cm::{Cm, GeometryError};

/// Kind of a [`Level`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LevelKind {
    /// Outdoor ground (terrain, streets, yards).
    Ground,
    /// Floor above the ground, numbered from 1.
    Floor,
    /// Cellar below the ground, numbered from 1.
    Cellar,
    /// Roof, above the top floor.
    Roof,
}

// spec: revues/generation.md P13
/// One storey of the stacked model: outdoor ground, floor `n`, cellar `n` or roof. The order
/// (kind, then index) is for use as a map key, not a vertical order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level {
    kind: LevelKind,
    index: u8,
}

impl Level {
    /// Highest floor number.
    pub const MAX_FLOORS: u8 = 100;
    /// Deepest cellar number.
    pub const MAX_CELLARS: u8 = 10;

    /// Outdoor ground.
    pub const fn ground() -> Self {
        Self {
            kind: LevelKind::Ground,
            index: 0,
        }
    }

    /// Roof.
    pub const fn roof() -> Self {
        Self {
            kind: LevelKind::Roof,
            index: 0,
        }
    }

    /// Floor `n`, `1..=`[`Level::MAX_FLOORS`], or [`GeometryError::InvalidLevel`].
    pub const fn floor(n: u8) -> Result<Self, GeometryError> {
        if n >= 1 && n <= Self::MAX_FLOORS {
            Ok(Self {
                kind: LevelKind::Floor,
                index: n,
            })
        } else {
            Err(GeometryError::InvalidLevel)
        }
    }

    /// Cellar `n`, `1..=`[`Level::MAX_CELLARS`], or [`GeometryError::InvalidLevel`].
    pub const fn cellar(n: u8) -> Result<Self, GeometryError> {
        if n >= 1 && n <= Self::MAX_CELLARS {
            Ok(Self {
                kind: LevelKind::Cellar,
                index: n,
            })
        } else {
            Err(GeometryError::InvalidLevel)
        }
    }

    /// Kind of level.
    pub const fn kind(self) -> LevelKind {
        self.kind
    }

    /// Floor or cellar number; 0 for ground and roof.
    pub const fn index(self) -> u8 {
        self.index
    }
}

// spec: revues/generation.md P13
/// Vertical extent of a wall or a slab, in centimetres: `bottom < top`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeightRange {
    bottom: Cm,
    top: Cm,
}

impl HeightRange {
    /// Range from `bottom` to `top`, or [`GeometryError::InvalidBounds`] unless `bottom < top`.
    pub const fn new(bottom: Cm, top: Cm) -> Result<Self, GeometryError> {
        if bottom.get() < top.get() {
            Ok(Self { bottom, top })
        } else {
            Err(GeometryError::InvalidBounds)
        }
    }

    /// Lower bound.
    pub const fn bottom(&self) -> Cm {
        self.bottom
    }

    /// Upper bound.
    pub const fn top(&self) -> Cm {
        self.top
    }

    /// Extent in centimetres (`i64`, can exceed the [`Cm`] range, N1).
    pub const fn extent(&self) -> i64 {
        self.top.wide() - self.bottom.wide()
    }

    /// Whether the open intervals share some height (touching ranges do not overlap).
    pub const fn overlaps(&self, o: &Self) -> bool {
        self.bottom.get() < o.top.get() && o.bottom.get() < self.top.get()
    }
}
