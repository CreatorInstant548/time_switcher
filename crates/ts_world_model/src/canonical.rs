//! Canonical byte encoding, the input of fingerprints.

use crate::{Aabb2, Cm, HeightRange, Level, LevelKind, Point2, Polygon2, Segment2, Vec2};

// spec: revues/generation.md §5.4 ; revues/architecture.md §7.1
/// Accumulates the canonical encoding: fixed-width little-endian integers, a one-byte tag per
/// type and a `u32` count before each sequence. No padding, no platform dependence.
#[derive(Debug, Default)]
pub struct CanonicalWriter {
    bytes: Vec<u8>,
}

impl CanonicalWriter {
    /// Empty writer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one byte.
    pub fn u8(&mut self, v: u8) {
        self.bytes.push(v);
    }

    /// Appends a little-endian `u32`.
    pub fn u32(&mut self, v: u32) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }

    /// Appends a little-endian `u64`.
    pub fn u64(&mut self, v: u64) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }

    /// Appends a little-endian `i32`.
    pub fn i32(&mut self, v: i32) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }

    /// Appends the raw value of a centimetre length.
    pub fn cm(&mut self, v: Cm) {
        self.i32(v.get());
    }

    /// Appends a point without a tag (used inside tagged types).
    pub fn point(&mut self, p: Point2) {
        self.cm(p.x);
        self.cm(p.y);
    }

    /// Appends a sequence length. Every encoded sequence is bounded well below `u32::MAX` by
    /// its constructor (for example [`Polygon2::MAX_VERTICES`]), so the saturation is unreachable.
    pub fn seq_len(&mut self, n: usize) {
        self.u32(u32::try_from(n).unwrap_or(u32::MAX));
    }

    /// The bytes written so far.
    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

/// Type with a canonical encoding. Changing an encoding changes every fingerprint: the
/// reference vectors in the tests must then be updated on purpose.
pub trait Canonical {
    /// Writes `self` to `w`.
    fn write_canonical(&self, w: &mut CanonicalWriter);
}

/// Canonical bytes of `value`.
pub fn canonical_bytes<T: Canonical + ?Sized>(value: &T) -> Vec<u8> {
    let mut w = CanonicalWriter::new();
    value.write_canonical(&mut w);
    w.finish()
}

/// Type tags, fixed forever (a reused tag would let two types collide).
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

impl Canonical for Cm {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::CM);
        w.cm(*self);
    }
}

impl Canonical for Point2 {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::POINT2);
        w.point(*self);
    }
}

impl Canonical for Vec2 {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::VEC2);
        w.cm(self.x);
        w.cm(self.y);
    }
}

impl Canonical for Aabb2 {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::AABB2);
        w.point(self.min());
        w.point(self.max());
    }
}

impl Canonical for Segment2 {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::SEGMENT2);
        w.point(self.a());
        w.point(self.b());
    }
}

impl Canonical for Polygon2 {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::POLYGON2);
        w.seq_len(self.vertices().len());
        for v in self.vertices() {
            w.point(*v);
        }
    }
}

impl Canonical for Level {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
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

impl Canonical for HeightRange {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::HEIGHT_RANGE);
        w.cm(self.bottom());
        w.cm(self.top());
    }
}
