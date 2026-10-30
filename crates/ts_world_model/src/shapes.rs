//! 2D shapes in integer centimetres: points, vectors, boxes, segments and polygons.

use crate::cm::{Cm, GeometryError};

// spec: revues/generation.md P13
/// Position in the plane of a level, in centimetres.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point2 {
    /// East-west coordinate.
    pub x: Cm,
    /// North-south coordinate.
    pub y: Cm,
}

// spec: revues/generation.md P13
/// Displacement between two points, each component within the [`Cm`] range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec2 {
    /// East-west component.
    pub x: Cm,
    /// North-south component.
    pub y: Cm,
}

impl Point2 {
    /// Point at `(x, y)`.
    pub const fn new(x: Cm, y: Cm) -> Self {
        Self { x, y }
    }

    /// Vector from `self` to `other`, or [`GeometryError::OutOfRange`] when a component leaves
    /// the [`Cm`] range (points more than 100 km apart).
    pub const fn to(self, other: Self) -> Result<Vec2, GeometryError> {
        let x = match other.x.checked_sub(self.x) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let y = match other.y.checked_sub(self.y) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Vec2 { x, y })
    }

    /// `self` moved by `v`, or [`GeometryError::OutOfRange`].
    pub const fn translated(self, v: Vec2) -> Result<Self, GeometryError> {
        let x = match self.x.checked_add(v.x) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let y = match self.y.checked_add(v.y) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Self { x, y })
    }
}

impl Vec2 {
    /// Vector `(x, y)`.
    pub const fn new(x: Cm, y: Cm) -> Self {
        Self { x, y }
    }

    /// Dot product in `i64` (at most `2 × 10^14`, N1).
    pub const fn dot(self, o: Self) -> i64 {
        self.x.wide() * o.x.wide() + self.y.wide() * o.y.wide()
    }

    /// Z component of the cross product in `i64` (at most `2 × 10^14`, N1).
    pub const fn cross(self, o: Self) -> i64 {
        self.x.wide() * o.y.wide() - self.y.wide() * o.x.wide()
    }

    /// Squared length in `i64` (N1).
    pub const fn length_squared(self) -> i64 {
        self.dot(self)
    }
}

// spec: revues/generation.md P13
/// Axis-aligned box with `min <= max` on both axes (a degenerate box is allowed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Aabb2 {
    min: Point2,
    max: Point2,
}

impl Aabb2 {
    /// Box from `min` to `max`, or [`GeometryError::InvalidBounds`].
    pub const fn new(min: Point2, max: Point2) -> Result<Self, GeometryError> {
        if min.x.get() <= max.x.get() && min.y.get() <= max.y.get() {
            Ok(Self { min, max })
        } else {
            Err(GeometryError::InvalidBounds)
        }
    }

    /// Corner with the smallest coordinates.
    pub const fn min(&self) -> Point2 {
        self.min
    }

    /// Corner with the largest coordinates.
    pub const fn max(&self) -> Point2 {
        self.max
    }

    /// Width in centimetres, as `i64` because it can exceed the [`Cm`] range (N1).
    pub const fn width(&self) -> i64 {
        self.max.x.wide() - self.min.x.wide()
    }

    /// Extent along the y axis in centimetres (`i64`, N1).
    pub const fn depth(&self) -> i64 {
        self.max.y.wide() - self.min.y.wide()
    }

    /// Area in square centimetres, at most `4 × 10^14` (`i64`, N1).
    pub const fn area(&self) -> i64 {
        self.width() * self.depth()
    }

    /// Whether `p` lies inside or on the border.
    pub const fn contains(&self, p: Point2) -> bool {
        p.x.get() >= self.min.x.get()
            && p.x.get() <= self.max.x.get()
            && p.y.get() >= self.min.y.get()
            && p.y.get() <= self.max.y.get()
    }

    /// Whether the two boxes share at least one point (borders included).
    pub const fn intersects(&self, o: &Self) -> bool {
        self.min.x.get() <= o.max.x.get()
            && o.min.x.get() <= self.max.x.get()
            && self.min.y.get() <= o.max.y.get()
            && o.min.y.get() <= self.max.y.get()
    }
}

// spec: revues/generation.md P13
/// Segment between two distinct points (a wall, an edge). Endpoint order is kept as given.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Segment2 {
    a: Point2,
    b: Point2,
}

impl Segment2 {
    /// Segment `a`–`b`, or [`GeometryError::DegenerateSegment`] when `a == b`.
    pub fn new(a: Point2, b: Point2) -> Result<Self, GeometryError> {
        if a == b {
            Err(GeometryError::DegenerateSegment)
        } else {
            Ok(Self { a, b })
        }
    }

    /// First endpoint.
    pub const fn a(&self) -> Point2 {
        self.a
    }

    /// Second endpoint.
    pub const fn b(&self) -> Point2 {
        self.b
    }

    /// Squared length in `i64`: each difference is at most `2 × 10^7`, so the sum of two
    /// squares is at most `8 × 10^14` (N1).
    pub const fn length_squared(&self) -> i64 {
        let dx = self.b.x.wide() - self.a.x.wide();
        let dy = self.b.y.wide() - self.a.y.wide();
        dx * dx + dy * dy
    }

    /// Bounding box.
    pub fn bounds(&self) -> Aabb2 {
        bounds_of(&[self.a, self.b])
    }
}

// spec: revues/generation.md P13
/// Simple-looking floor polygon: 3 to [`Polygon2::MAX_VERTICES`] distinct vertices, non-zero
/// area, **counter-clockwise** (the constructor reverses a clockwise input, keeping the first
/// vertex first). Self-intersection is not checked yet (validators come with the generator).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Polygon2 {
    vertices: Vec<Point2>,
}

impl Polygon2 {
    /// Vertex limit: bounds memory and keeps the area sum small (N1).
    pub const MAX_VERTICES: usize = 65_535;

    /// Validates and normalizes `vertices` (see the type documentation).
    pub fn new(mut vertices: Vec<Point2>) -> Result<Self, GeometryError> {
        if vertices.len() < 3 {
            return Err(GeometryError::TooFewVertices);
        }
        if vertices.len() > Self::MAX_VERTICES {
            return Err(GeometryError::TooManyVertices);
        }
        let mut sorted = vertices.clone();
        sorted.sort_unstable();
        if sorted.windows(2).any(|w| w[0] == w[1]) {
            return Err(GeometryError::RepeatedVertex);
        }
        let twice = twice_signed_area(&vertices);
        if twice == 0 {
            return Err(GeometryError::ZeroArea);
        }
        if twice < 0 {
            vertices[1..].reverse();
        }
        Ok(Self { vertices })
    }

    /// Vertices in counter-clockwise order.
    pub fn vertices(&self) -> &[Point2] {
        &self.vertices
    }

    /// Twice the area in square centimetres, always positive. Computed in `i128`: a term is at
    /// most `2 × 10^14` and there can be 65,535 of them, which overflows `i64` (N1).
    pub fn twice_area(&self) -> i128 {
        twice_signed_area(&self.vertices)
    }

    /// Bounding box.
    pub fn bounds(&self) -> Aabb2 {
        bounds_of(&self.vertices)
    }
}

/// Shoelace sum in `i128`; positive for a counter-clockwise ring.
fn twice_signed_area(ring: &[Point2]) -> i128 {
    let mut sum: i128 = 0;
    for (i, p) in ring.iter().enumerate() {
        let q = ring[(i + 1) % ring.len()];
        sum += i128::from(p.x.wide() * q.y.wide() - q.x.wide() * p.y.wide());
    }
    sum
}

/// Bounding box of a non-empty slice (callers guarantee at least one point).
fn bounds_of(points: &[Point2]) -> Aabb2 {
    let mut min = points[0];
    let mut max = points[0];
    for p in points {
        min = Point2::new(min.x.min(p.x), min.y.min(p.y));
        max = Point2::new(max.x.max(p.x), max.y.max(p.y));
    }
    Aabb2 { min, max }
}
