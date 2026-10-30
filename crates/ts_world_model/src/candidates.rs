//! Generic candidates for the server part (camera spots, vault and cache sites, alarm sites).
//!
//! Candidates exist everywhere and reveal nothing by themselves; what is really placed is
//! drawn per world in the `server` part. They are [`ClientShippable`](crate::ClientShippable)
//! (generation arbitration A13 (#34), architecture and security AS-32): the client package
//! carries them, and the solo host needs them to draw the server part. The module stays
//! separate from `public`; the marker is implemented in `public.rs`, the only place that can
//! name the seal.
//!
//! **Condition C1 (AS-32), for the future generator (V12).** The function that produces the
//! candidates must be a function of the public geometry and the content package alone: it
//! receives no `WorldKey`, no `StreamKeys` and no datum from the per-world draw (W2), and its
//! signature must say so. A candidate that depended on W2 would reveal the real placement.

use ts_kernel::{StableId, StateHash, StateWriter};

use crate::canonical::{tag, write_point};
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

impl StateHash for CandidateSpot {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(tag::CANDIDATE_SPOT);
        self.id.hash_state(w);
        w.u8(match self.kind {
            CandidateKind::Camera => 0,
            CandidateKind::Vault => 1,
            CandidateKind::Cache => 2,
            CandidateKind::Alarm => 3,
        });
        self.level.hash_state(w);
        write_point(self.position, w);
    }
}
