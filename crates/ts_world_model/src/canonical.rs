//! Encoding of the map types: the kernel's `StateHash`, the one and only canonical encoding.
//!
//! This is the input of fingerprints. There is no second encoding in this crate: a type is
//! encoded by implementing [`StateHash`], and the bytes are those of the kernel's
//! [`StateWriter`] (little-endian integers, `u64` counts before sequences).

use ts_kernel::{StateHash, StateWriter};

use crate::{Aabb2, Cm, HeightRange, Level, LevelKind, Point2, Polygon2, Segment2, Vec2};

/// Type tags, fixed forever (a reused tag would let two types collide). The kernel's integers
/// carry no tag, so each type of this crate starts with its own.
pub(crate) mod tag {
    pub const CM: u8 = 0x01;
    pub const POINT2: u8 = 0x02;
    pub const VEC2: u8 = 0x03;
    pub const AABB2: u8 = 0x04;
    pub const SEGMENT2: u8 = 0x05;
    pub const POLYGON2: u8 = 0x06;
    pub const LEVEL: u8 = 0x07;
    pub const HEIGHT_RANGE: u8 = 0x08;
    pub const PUBLIC_PLACE: u8 = 0x10;
    pub const PUBLIC_MAP: u8 = 0x11;
    pub const CANDIDATE_SPOT: u8 = 0x20;
}

// spec: revues/generation.md §5.4 ; revues/architecture.md §7.1, §11.3
/// Writes a point without a tag (used inside tagged types). Inside a tagged type, always use
/// this, never `Point2::hash_state`, which adds its own tag byte: the two must not be mixed in
/// one encoding.
pub(crate) fn write_point(p: Point2, w: &mut StateWriter<'_>) {
    p.x.get().hash_state(w);
    p.y.get().hash_state(w);
}

impl StateHash for Cm {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::CM);
        self.get().hash_state(w);
    }
}

impl StateHash for Point2 {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::POINT2);
        write_point(*self, w);
    }
}

impl StateHash for Vec2 {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::VEC2);
        self.x.get().hash_state(w);
        self.y.get().hash_state(w);
    }
}

impl StateHash for Aabb2 {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::AABB2);
        write_point(self.min(), w);
        write_point(self.max(), w);
    }
}

impl StateHash for Segment2 {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::SEGMENT2);
        write_point(self.a(), w);
        write_point(self.b(), w);
    }
}

impl StateHash for Polygon2 {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::POLYGON2);
        w.count(self.vertices().len());
        for v in self.vertices() {
            write_point(*v, w);
        }
    }
}

impl StateHash for Level {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::LEVEL);
        w.u8(match self.kind() {
            LevelKind::Ground => 0,
            LevelKind::Floor => 1,
            LevelKind::Cellar => 2,
            LevelKind::Roof => 3,
        });
        w.u8(self.index());
    }
}

impl StateHash for HeightRange {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::HEIGHT_RANGE);
        self.bottom().get().hash_state(w);
        self.top().get().hash_state(w);
    }
}
