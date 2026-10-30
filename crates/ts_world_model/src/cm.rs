//! Integer centimetres, geometry errors and float quantization.

use core::fmt;

/// Largest absolute value of a coordinate: 10,000,000 cm, i.e. 100 km.
///
/// A city spans a few kilometres: the margin is one order of magnitude. With this bound, the
/// difference of two coordinates fits in `±2 × 10^7` and the product of two differences in
/// `±4 × 10^14`, so every segment or box computation fits in `i64`. Only polygon areas, which
/// sum up to 65,535 terms, go through `i128` (N1).
pub const MAX_ABS_CM: i32 = 10_000_000;

// spec: revues/generation.md P13
/// Geometry construction or computation error: always refused, never silently corrected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryError {
    /// Value outside `±`[`MAX_ABS_CM`] (an integer or the result of a computation).
    OutOfRange,
    /// NaN or infinite float.
    NotFinite,
    /// Polygon with fewer than 3 vertices.
    TooFewVertices,
    /// Polygon with more than [`crate::Polygon2::MAX_VERTICES`] vertices.
    TooManyVertices,
    /// Polygon with two identical vertices.
    RepeatedVertex,
    /// Polygon whose signed area is zero: collinear vertices, but also a self-intersecting
    /// ring whose lobes cancel (a bow-tie). The general self-intersection check is deferred to
    /// the map validators; a figure-eight with a non-zero net area is accepted for now.
    ZeroArea,
    /// Polygon with three consecutive collinear vertices (including a spike that doubles
    /// back): the ring must be given without them, so that a shape has one encoding.
    CollinearVertices,
    /// Segment whose two endpoints coincide.
    DegenerateSegment,
    /// Box or height range with inverted bounds (or equal bounds for a height range).
    InvalidBounds,
    /// Floor or cellar index outside the allowed range.
    InvalidLevel,
    /// Two places share the same identifier.
    DuplicateId,
    /// Place index outside the map identifier space (`StableId::MAX_INDEX`).
    MapIndexOutOfRange,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::OutOfRange => "value outside the ±100 km coordinate range",
            Self::NotFinite => "NaN or infinite float",
            Self::TooFewVertices => "polygon needs at least 3 vertices",
            Self::TooManyVertices => "polygon has too many vertices",
            Self::RepeatedVertex => "polygon repeats a vertex",
            Self::ZeroArea => "polygon has zero area",
            Self::CollinearVertices => "polygon has three consecutive collinear vertices",
            Self::DegenerateSegment => "segment endpoints coincide",
            Self::InvalidBounds => "bounds are inverted",
            Self::InvalidLevel => "level index out of range",
            Self::DuplicateId => "duplicate place identifier",
            Self::MapIndexOutOfRange => "map index out of range",
        })
    }
}

impl core::error::Error for GeometryError {}

// spec: revues/generation.md P13
/// Length in **integer centimetres**, within `±`[`MAX_ABS_CM`]. Never a stored float.
///
/// No `+` or `-` operator: arithmetic is `checked_*` and returns an error instead of wrapping
/// or saturating. The only path from a float is [`quantize`].
///
/// ```
/// use ts_world_model::Cm;
/// assert!(Cm::new(10_000_000).is_ok());
/// assert!(Cm::new(10_000_001).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cm(i32);

ts_kernel::assert_size!(Cm, 4);

impl Cm {
    /// Zero.
    pub const ZERO: Self = Self(0);
    /// Smallest value.
    pub const MIN: Self = Self(-MAX_ABS_CM);
    /// Largest value.
    pub const MAX: Self = Self(MAX_ABS_CM);

    /// `cm` centimetres, or [`GeometryError::OutOfRange`].
    pub const fn new(cm: i32) -> Result<Self, GeometryError> {
        if cm >= -MAX_ABS_CM && cm <= MAX_ABS_CM {
            Ok(Self(cm))
        } else {
            Err(GeometryError::OutOfRange)
        }
    }

    /// Value in centimetres.
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Widened value, for products (N1).
    pub const fn wide(self) -> i64 {
        self.0 as i64
    }

    /// Sum, or [`GeometryError::OutOfRange`].
    pub const fn checked_add(self, other: Self) -> Result<Self, GeometryError> {
        // Both operands are bounded: the sum cannot overflow `i32`.
        Self::new(self.0 + other.0)
    }

    /// Difference, or [`GeometryError::OutOfRange`].
    pub const fn checked_sub(self, other: Self) -> Result<Self, GeometryError> {
        Self::new(self.0 - other.0)
    }

    /// Negation (always in range, the range is symmetric).
    pub const fn negated(self) -> Self {
        Self(-self.0)
    }
}

// spec: revues/generation.md §5.4
/// Quantizes a length in **metres** (`f64`) to integer centimetres: the single float-to-integer
/// path, applied at the output of every float computation step.
///
/// Exact definition: `round_ties_even(metres × 100.0)`. The multiplication and the rounding are
/// IEEE 754 operations, hence identical on every platform. NaN, infinities and any result
/// outside `±`[`MAX_ABS_CM`] are refused, never saturated (N2). `-0.0` gives `0`.
///
/// ```
/// use ts_world_model::quantize;
/// assert_eq!(quantize(0.125).unwrap().get(), 12); // 12.5 cm: ties to even
/// assert_eq!(quantize(0.375).unwrap().get(), 38); // 37.5 cm: ties to even
/// assert!(quantize(f64::NAN).is_err());
/// ```
pub fn quantize(meters: f64) -> Result<Cm, GeometryError> {
    if !meters.is_finite() {
        return Err(GeometryError::NotFinite);
    }
    let cm = (meters * 100.0).round_ties_even();
    // Exact comparison: the bounds are representable as `f64`.
    if cm.abs() > f64::from(MAX_ABS_CM) {
        return Err(GeometryError::OutOfRange);
    }
    // Integral, finite and in range: the conversion is exact.
    Ok(Cm(cm as i32))
}
