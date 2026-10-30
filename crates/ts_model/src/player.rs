//! `PlayerId`: opaque per-world player pseudonym.

use core::fmt;

// spec: securite.md S12 ; carnet architecture-securite.md SEC0-21, §9.11 M2
/// Pseudonym of a player in **one** world: 128 bits, `HMAC(world secret key, account id)`
/// truncated, computed by the host. The account mapping exists only in `ts_accounts`, so a
/// leaked world database links to no account and two worlds do not cross-match (S12).
///
/// It is not personal data in itself, so it may enter the state hash and the snapshots
/// through [`PlayerId::canonical_bytes`]. On purpose, it has:
/// - a single constructor, [`PlayerId::from_pseudonym_bytes`], fed by the host;
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
