//! Public part of the map: what the client package may contain.
//!
//! Everything in this module is public by construction: it is identical for all worlds and
//! reveals nothing about a given world (S33). The sealed marker [`ClientShippable`] is
//! implemented here and nowhere else: for the types of this module and for the generic
//! candidates of `candidates` (generation arbitration A13 (#34), AS-32), which stay a separate
//! module. Only the per-world `server` part is excluded.

use ts_kernel::{StableId, StateHash, StateWriter};

use crate::candidates::CandidateSpot;
use crate::canonical::tag;
use crate::server::{ServerPart, ServerPlacement};
use crate::{GeometryError, HeightRange, Level, Polygon2, map_id};

/// Private to this module on purpose: neither `candidates`, nor `server`, nor any other crate
/// can name it, so none of them can implement [`ClientShippable`].
mod sealed {
    pub trait Sealed {}
}

// spec: revues/generation.md P12 ; revues/securite.md S33
/// Marker of the types that may be written to the client package (N3). Sealed: only
/// `public.rs` implements it, for the types of the `public` module **and** for the generic
/// candidates of the `candidates` module (generation arbitration A13 (#34), AS-32). The
/// per-world `server` part never implements it. Any future packager must require it:
///
/// ```
/// use ts_world_model::{ClientShippable, public::PublicMap};
/// fn ship<T: ClientShippable>() {}
/// ship::<PublicMap>();
/// ```
///
/// **The seal.** The two examples below differ only by the `ClientShippable` line. The first
/// proves the setup compiles: a local type that satisfies every other bound (`StateHash`).
/// The second adds the impl and must fail, and the only missing bound is the private
/// `Sealed` supertrait. (rustdoc stable does not check the error code, which would be E0277;
/// the passing control is what makes the failing example reliable.)
///
/// ```
/// use ts_kernel::{StateHash, StateWriter};
/// struct Mine;
/// impl StateHash for Mine { fn hash_state(&self, _: &mut StateWriter<'_>) {} }
/// ```
///
/// ```compile_fail
/// use ts_kernel::{StateHash, StateWriter};
/// use ts_world_model::ClientShippable;
/// struct Mine;
/// impl StateHash for Mine { fn hash_state(&self, _: &mut StateWriter<'_>) {} }
/// impl ClientShippable for Mine {} // only `Sealed` is missing
/// ```
///
/// Inside the crate, `candidates` and `server` cannot name `public::sealed` (module privacy),
/// which a doctest cannot test: the candidates are shippable only because `public.rs`
/// implements the marker for them (generation arbitration A13 (#34), AS-32), and `server` has
/// no such impl. The compile-time assertion `ServerPart: !ClientShippable` at the end of
/// `public.rs` breaks the build if someone adds that impl there.
///
/// ```
/// use ts_world_model::{ClientShippable, candidates::CandidateSpot};
/// fn ship<T: ClientShippable>() {}
/// ship::<CandidateSpot>();
/// ```
///
/// The next example only shows that the server type does **not** implement the marker today.
/// It proves nothing about whether it could: the guarantee is the seal above.
///
/// ```compile_fail
/// use ts_world_model::{ClientShippable, server::ServerPart};
/// fn ship<T: ClientShippable>() {}
/// ship::<ServerPart>();
/// ```
pub trait ClientShippable: sealed::Sealed + StateHash {}

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

impl StateHash for PublicPlace {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::PUBLIC_PLACE);
        self.id.hash_state(w);
        self.level.hash_state(w);
        self.outline.hash_state(w);
        self.height.hash_state(w);
    }
}

impl StateHash for PublicMap {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::PUBLIC_MAP);
        w.count(self.places.len());
        for p in &self.places {
            p.hash_state(w);
        }
    }
}

impl sealed::Sealed for PublicPlace {}
impl sealed::Sealed for PublicMap {}
impl ClientShippable for PublicPlace {}
impl ClientShippable for PublicMap {}

// Generation arbitration A13 (#34), AS-32: the generic candidates are public by design. They
// live in `candidates`, but only this module can name the seal, so the impl is here.
impl sealed::Sealed for CandidateSpot {}
impl ClientShippable for CandidateSpot {}

// N3, hardening: `ServerPart: !ClientShippable` (and the same for `ServerPlacement`; a new type
// of `server.rs` is added to the list below), checked at compile time. Hand-written with the
// trait-ambiguity trick of `static_assertions::assert_not_impl_any!`, without the dependency.
// `ServerPart` matches the blanket impl (`A = ()`); if it also implemented `ClientShippable`, it
// would match the second impl (`A = u8`), the call `some_item` could not pick one (E0283) and
// the build would fail, even for an impl added inside this crate.
const _: fn() = || {
    trait AmbiguousIfImpl<A> {
        fn some_item() {}
    }
    impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
    impl<T: ?Sized + ClientShippable> AmbiguousIfImpl<u8> for T {}
    let _ = <ServerPart as AmbiguousIfImpl<_>>::some_item;
    let _ = <ServerPlacement as AmbiguousIfImpl<_>>::some_item;
};
