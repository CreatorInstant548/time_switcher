//! `PlayerId`: opaque per-world player pseudonym.

use core::fmt;

use ts_kernel::{CanonicalError, StateDecode, StateHash, StateReader, StateWriter};

// spec: securite.md S12 ; carnet architecture-securite.md SEC0-21, §9.11 M2
/// Pseudonym of a player in **one** world: 128 bits, `HMAC(world secret key, account id)`
/// truncated, computed by the host. The account mapping exists only in `ts_accounts`, so a
/// leaked world database links to no account and two worlds do not cross-match (S12).
///
/// It is not personal data in itself, so it may enter the state hash and the snapshots
/// through [`PlayerId::canonical_bytes`] and its [`StateHash`] encoding. On purpose, it has:
/// - a single constructor, [`PlayerId::from_pseudonym_bytes`], fed by the host (plus its
///   [`StateDecode`], which reads back a snapshot the host restores);
/// - no `From`/`Into` from or to an integer, a string or bytes, no `FromStr`, no `Default`;
/// - no `Display` and no `Serialize`: it is never shown nor sent as an account identifier;
/// - an abridged `Debug`: the first 4 bytes in hexadecimal.
///
/// There is no account identifier type in `ts_model` nor in the core.
///
/// Each `compile_fail` example below has a compiling **witness** first, so that it cannot
/// fail for another reason (stable rustdoc does not check the error code).
///
/// ```
/// let id = ts_model::PlayerId::from_pseudonym_bytes([7; 16]);
/// let _ = format!("{id:?}");
/// ```
///
/// ```compile_fail,E0423
/// let _ = ts_model::PlayerId([7; 16]);
/// ```
///
/// ```compile_fail,E0277
/// let _: ts_model::PlayerId = 7_u128.into();
/// ```
///
/// ```compile_fail,E0277
/// let _: ts_model::PlayerId = [7_u8; 16].into();
/// ```
///
/// ```compile_fail,E0277
/// let _: ts_model::PlayerId = "player".parse().unwrap();
/// ```
///
/// ```compile_fail,E0277
/// let id = ts_model::PlayerId::from_pseudonym_bytes([7; 16]);
/// let _ = format!("{id}");
/// ```
///
/// ```compile_fail,E0599
/// let _ = ts_model::PlayerId::default();
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerId([u8; 16]);

ts_kernel::assert_size!(PlayerId, 16);

impl PlayerId {
    /// Pseudonym computed by the host (truncated HMAC). The only constructor: no other crate
    /// makes a `PlayerId` from an integer, a string or an account identifier.
    pub const fn from_pseudonym_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Canonical encoding, for the state hash and the snapshots: the 16 pseudonym bytes, in
    /// the order the host gave them. Ordering ([`Ord`]) is the lexicographic order of these
    /// bytes.
    pub const fn canonical_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

// spec: carnet architecture-securite.md §9.11 M2, §9.10 L3
/// State hash encoding: the 16 [`PlayerId::canonical_bytes`], fixed width, so self-delimiting.
impl StateHash for PlayerId {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        for &b in &self.0 {
            w.u8(b);
        }
    }
}

/// Decoding twin of the [`StateHash`] encoding: the 16 bytes, in order (AS-36).
impl StateDecode for PlayerId {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        let mut bytes = [0; 16];
        for b in &mut bytes {
            *b = r.u8()?;
        }
        Ok(Self(bytes))
    }
}

impl fmt::Debug for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PlayerId(")?;
        for b in &self.0[..4] {
            write!(f, "{b:02x}")?;
        }
        f.write_str("…)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff,
    ];

    /// Frozen vectors: changing the encoding changes every recorded state hash.
    #[test]
    fn canonical_encoding_reference_vectors() {
        assert_eq!(
            PlayerId::from_pseudonym_bytes(BYTES).canonical_bytes(),
            &BYTES
        );
        assert_eq!(
            PlayerId::from_pseudonym_bytes([0; 16]).canonical_bytes(),
            &[0; 16]
        );
        assert_eq!(
            PlayerId::from_pseudonym_bytes([0xff; 16]).canonical_bytes(),
            &[0xff; 16]
        );
    }

    /// The `StateHash` encoding is the 16 bytes, in order, inside the canonical snapshot.
    #[test]
    fn state_hash_encoding_is_the_sixteen_bytes() {
        use ts_kernel::bevy_ecs;
        use ts_kernel::bevy_ecs::resource::Resource;
        use ts_kernel::rng::WorldKey;
        use ts_kernel::{MemorySnapshot, Plugin, Sim, SimBuilder, SimConfig, SnapshotScope};

        #[derive(Resource, Clone)]
        struct Holder(PlayerId);
        impl StateHash for Holder {
            fn hash_state(&self, w: &mut StateWriter<'_>) {
                self.0.hash_state(w);
            }
        }
        impl StateDecode for Holder {
            fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
                PlayerId::decode_state(r).map(Self)
            }
        }
        struct Probe([u8; 16]);
        impl Plugin for Probe {
            fn name(&self) -> &'static str {
                "probe"
            }
            fn version(&self) -> u32 {
                1
            }
            fn build(&self, b: &mut SimBuilder) {
                b.insert_resource(Holder(PlayerId::from_pseudonym_bytes(self.0)));
                b.state_resource::<Holder>("probe");
            }
        }
        let build = |bytes| {
            let rules: Vec<Box<dyn Plugin>> = vec![Box::new(Probe(bytes))];
            Sim::build(SimConfig::default(), &rules, &WorldKey::from_bytes([3; 16])).unwrap()
        };
        let sim = build(BYTES);
        let snap: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
        assert!(snap.canonical_bytes().windows(16).any(|w| w == BYTES));
        let mut other = BYTES;
        other[15] = 0;
        let a = sim.state_hash();
        let b = build(other).state_hash();
        assert_ne!(a.family("probe"), b.family("probe"));
        assert_eq!(a.family("kernel"), b.family("kernel"));
    }

    #[test]
    fn debug_shows_four_bytes_only() {
        let id = PlayerId::from_pseudonym_bytes(BYTES);
        assert_eq!(format!("{id:?}"), "PlayerId(00112233…)");
        assert_eq!(format!("{id:#?}"), "PlayerId(00112233…)");
        let text = format!("{id:?}");
        assert!(!text.contains("44556677"));
    }

    #[test]
    fn order_is_lexicographic_on_bytes() {
        let mut low = BYTES;
        low[15] = 0xfe;
        let mut high = [0; 16];
        high[0] = 0x01;
        let low = PlayerId::from_pseudonym_bytes(low);
        let mid = PlayerId::from_pseudonym_bytes(BYTES);
        let high = PlayerId::from_pseudonym_bytes(high);
        assert!(low < mid && mid < high);
        assert_ne!(low, mid);
    }
}
