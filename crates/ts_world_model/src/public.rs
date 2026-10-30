//! Public part of the map: what the client package may contain.
//!
//! Everything in this module is public by construction: it is identical for all worlds and
//! reveals nothing about a given world (S33). The sealed marker [`ClientShippable`] is
//! implemented here and nowhere else.

use ts_kernel::StableId;

use crate::canonical::{Canonical, CanonicalWriter, tag};
use crate::{GeometryError, HeightRange, Level, Polygon2, map_id};

/// Private to this module on purpose: neither `candidates`, nor `server`, nor any other crate
/// can name it, so none of them can implement [`ClientShippable`].
mod sealed {
    pub trait Sealed {}
}

// spec: revues/generation.md P12 ; revues/securite.md S33
/// Marker of the types that may be written to the client package. Sealed: only the types of
/// the `public` module implement it (N3). Any future packager must require it:
///
/// ```
/// use ts_world_model::{ClientShippable, public::PublicMap};
/// fn ship<T: ClientShippable>() {}
/// ship::<PublicMap>();
/// ```
///
/// **The seal.** The two examples below differ only by the `ClientShippable` line. The first
/// proves the setup compiles: a local type that satisfies every other bound (`Canonical`).
/// The second adds the impl and must fail, and the only missing bound is the private
/// `Sealed` supertrait. (rustdoc stable does not check the error code, which would be E0277;
/// the passing control is what makes the failing example reliable.)
///
/// ```
/// use ts_world_model::{Canonical, CanonicalWriter};
/// struct Mine;
/// impl Canonical for Mine { fn write_canonical(&self, _: &mut CanonicalWriter) {} }
/// ```
///
/// ```compile_fail
/// use ts_world_model::{Canonical, CanonicalWriter, ClientShippable};
/// struct Mine;
/// impl Canonical for Mine { fn write_canonical(&self, _: &mut CanonicalWriter) {} }
/// impl ClientShippable for Mine {} // only `Sealed` is missing
/// ```
///
/// Inside the crate, `candidates` and `server` cannot name `public::sealed` (module privacy),
/// which a doctest cannot test.
///
/// The two examples below only show that the candidate and server types do **not** implement
/// the marker today. They prove nothing about whether they could: the guarantee is the seal
/// above.
///
/// ```compile_fail
/// use ts_world_model::{ClientShippable, candidates::CandidateSpot};
/// fn ship<T: ClientShippable>() {}
/// ship::<CandidateSpot>();
/// ```
///
/// ```compile_fail
/// use ts_world_model::{ClientShippable, server::ServerPart};
/// fn ship<T: ClientShippable>() {}
/// ship::<ServerPart>();
/// ```
pub trait ClientShippable: sealed::Sealed + Canonical {}

// spec: revues/generation.md P13, V13
/// A place of the public map: a floor outline on one level with its height range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPlace {
    id: StableId,
    level: Level,
    outline: Polygon2,
    height: HeightRange,
}

impl PublicPlace {
    /// Place `index` of the map identifier space (`StableId::map`, P28, N4).
    pub fn new(
        index: u64,
        level: Level,
        outline: Polygon2,
        height: HeightRange,
    ) -> Result<Self, GeometryError> {
        Ok(Self {
            id: map_id(index)?,
            level,
            outline,
            height,
        })
    }

    /// Stable identifier, in the map space.
    pub fn id(&self) -> StableId {
        self.id
    }

    /// Level of the place.
    pub fn level(&self) -> Level {
        self.level
    }

    /// Floor outline.
    pub fn outline(&self) -> &Polygon2 {
        &self.outline
    }

    /// Height range.
    pub fn height(&self) -> HeightRange {
        self.height
    }
}

// spec: revues/generation.md P12
/// The public part of a map: places in ascending identifier order, without duplicates.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PublicMap {
    places: Vec<PublicPlace>,
}

impl PublicMap {
    /// Sorts `places` by identifier; a duplicate identifier is refused.
    pub fn new(mut places: Vec<PublicPlace>) -> Result<Self, GeometryError> {
        places.sort_unstable_by_key(PublicPlace::id);
        if places.windows(2).any(|w| w[0].id == w[1].id) {
            return Err(GeometryError::DuplicateId);
        }
        Ok(Self { places })
    }

    /// Places in ascending identifier order.
    pub fn places(&self) -> &[PublicPlace] {
        &self.places
    }
}

impl Canonical for PublicPlace {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::PUBLIC_PLACE);
        w.u64(self.id.to_snapshot_raw());
        self.level.write_canonical(w);
        self.outline.write_canonical(w);
        self.height.write_canonical(w);
    }
}

impl Canonical for PublicMap {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::PUBLIC_MAP);
        w.seq_len(self.places.len());
        for p in &self.places {
            p.write_canonical(w);
        }
    }
}

impl sealed::Sealed for PublicPlace {}
impl sealed::Sealed for PublicMap {}
impl ClientShippable for PublicPlace {}
impl ClientShippable for PublicMap {}
