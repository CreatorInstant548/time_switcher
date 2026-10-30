//! Generic candidates for the server part (camera spots, vault and cache sites, alarm sites).
//!
//! Candidates exist everywhere and reveal nothing by themselves; what is really placed is
//! drawn per world in the `server` part. They are **not** [`ClientShippable`](crate::ClientShippable)
//! for now: whether the client package carries them is a design decision still to be taken
//! (see the PR description), and the marker is sealed to the `public` module.

use ts_kernel::StableId;

use crate::canonical::{Canonical, CanonicalWriter, tag};
use crate::{GeometryError, Level, Point2, map_id};

// spec: revues/generation.md P12, V12
/// What a candidate spot may host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CandidateKind {
    /// Possible camera mounting point.
    Camera,
    /// Possible vault location.
    Vault,
    /// Possible hiding place.
    Cache,
    /// Place that may be fitted with an alarm.
    Alarm,
}

// spec: revues/generation.md P12, V12
/// A generic candidate position on one level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateSpot {
    id: StableId,
    kind: CandidateKind,
    level: Level,
    position: Point2,
}

impl CandidateSpot {
    /// Candidate `index` of the map identifier space (`StableId::map`, P28, N4).
    pub fn new(
        index: u64,
        kind: CandidateKind,
        level: Level,
        position: Point2,
    ) -> Result<Self, GeometryError> {
        Ok(Self {
            id: map_id(index)?,
            kind,
            level,
            position,
        })
    }

    /// Stable identifier, in the map space.
    pub fn id(&self) -> StableId {
        self.id
    }

    /// What the spot may host.
    pub fn kind(&self) -> CandidateKind {
        self.kind
    }

    /// Level of the spot.
    pub fn level(&self) -> Level {
        self.level
    }

    /// Position on the level.
    pub fn position(&self) -> Point2 {
        self.position
    }
}

impl Canonical for CandidateSpot {
    fn write_canonical(&self, w: &mut CanonicalWriter) {
        w.u8(tag::CANDIDATE_SPOT);
        w.u64(self.id.to_snapshot_raw());
        w.u8(match self.kind {
            CandidateKind::Camera => 0,
            CandidateKind::Vault => 1,
            CandidateKind::Cache => 2,
            CandidateKind::Alarm => 3,
        });
        self.level.write_canonical(w);
        w.point(self.position);
    }
}
