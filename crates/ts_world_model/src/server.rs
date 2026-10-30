//! Server part of the map: what is really placed in a world (locks, cameras, vaults, alarms,
//! caches), drawn per world among the candidates.
//!
//! **Rule: nothing from this module is ever serialized to the client** (P12, S33). The types
//! here implement neither [`Canonical`](crate::Canonical) nor
//! [`ClientShippable`](crate::ClientShippable), and the latter is sealed to the `public`
//! module. A fact of this part reaches a client only through the filtered view, one at a time.

use ts_kernel::StableId;

use crate::{GeometryError, map_id};

// spec: revues/generation.md P12 ; revues/securite.md S33
/// One thing really placed on a candidate spot of this world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlacement {
    candidate: StableId,
}

impl ServerPlacement {
    /// Placement on candidate `index` (`StableId::map`, P28, N4).
    pub fn new(index: u64) -> Result<Self, GeometryError> {
        Ok(Self {
            candidate: map_id(index)?,
        })
    }

    /// Candidate that hosts it.
    pub fn candidate(&self) -> StableId {
        self.candidate
    }
}

// spec: revues/generation.md P12
/// The server part of one world. Empty for now: the draws (W2) arrive with the generator.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ServerPart {
    placements: Vec<ServerPlacement>,
}

impl ServerPart {
    /// Server part with these placements.
    pub fn new(placements: Vec<ServerPlacement>) -> Self {
        Self { placements }
    }

    /// Placements of this world.
    pub fn placements(&self) -> &[ServerPlacement] {
        &self.placements
    }
}
