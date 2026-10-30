//! Manifest of the parts: names, codecs, lengths and BLAKE3 digests (P30).

use serde::{Deserialize, Serialize};

use crate::error::{Bound, DecodeError};
use crate::limits::Limits;
use crate::schema::{Schema, Seq, flat_schema};

/// BLAKE3 digest, 32 bytes.
pub type Digest = [u8; 32];

/// BLAKE3 context of the digest of a part's bytes, written in full.
const PART_CONTEXT: &str = "time_switcher 2026-10-30 persist part v1";
/// BLAKE3 context of the digest of the manifest bytes (the root), written in full.
const MANIFEST_CONTEXT: &str = "time_switcher 2026-10-30 persist manifest v1";

fn digest(context: &str, bytes: &[u8]) -> Digest {
    *blake3::Hasher::new_derive_key(context)
        .update(bytes)
        .finalize()
        .as_bytes()
}

/// Digest of the bytes of a part, stored or decoded.
pub(crate) fn part_digest(bytes: &[u8]) -> Digest {
    digest(PART_CONTEXT, bytes)
}

/// Digest of the manifest bytes, carried by the envelope. A future signature signs it.
pub(crate) fn manifest_digest(bytes: &[u8]) -> Digest {
    digest(MANIFEST_CONTEXT, bytes)
}

// spec: revues/architecture.md §12.1 ; revues/generation.md §8.1
/// How a part is stored. An unknown value is refused by the decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Codec {
    /// Bytes as they are.
    Stored,
    /// One `zstd` frame, decoded by `ruzstd` (pure Rust).
    Zstd,
}

impl Schema for Codec {
    const DEPTH: usize = 0;
}

flat_schema! {
    // spec: revues/generation.md §8.1, P30
    /// One part: name, codec, then the length and digest of the stored bytes and of the
    /// decoded bytes. The stored digest is checked before decompression, the decoded one
    /// after.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PartEntry {
        /// Identifier in `[a-z0-9._/-]`, never a file path.
        pub name: String,
        /// Storage of the part.
        pub codec: Codec,
        /// Stored length, in the payload.
        pub stored_len: u64,
        /// Digest of the stored bytes.
        pub stored_digest: Digest,
        /// Decoded length.
        pub len: u64,
        /// Digest of the decoded bytes.
        pub digest: Digest,
    }
}

flat_schema! {
    // spec: revues/generation.md §8.1, P30 ; carnet architecture-securite.md §9.13
    /// List of the parts, strictly sorted by name; the payload holds their stored bytes in
    /// this order.
    #[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
    pub struct Manifest {
        /// Parts, sorted by name.
        pub parts: Seq<PartEntry>,
    }
}

fn too_large(bound: Bound, len: u64, max: u64) -> Result<(), DecodeError> {
    if len > max {
        Err(DecodeError::TooLarge { bound, len, max })
    } else {
        Ok(())
    }
}

/// Length as `u64`, saturated (lengths of memory never exceed it on our targets).
pub(crate) fn len64(n: usize) -> u64 {
    u64::try_from(n).unwrap_or(u64::MAX)
}

/// Checks a part name: length, then alphabet.
pub(crate) fn check_name(name: &str, limits: &Limits) -> Result<(), DecodeError> {
    too_large(
        Bound::Name,
        len64(name.len()),
        u64::from(limits.max_name_len),
    )?;
    let allowed = |b: &u8| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'/' | b'-');
    if name.is_empty() || !name.as_bytes().iter().all(allowed) {
        return Err(DecodeError::InvalidName);
    }
    Ok(())
}

impl Manifest {
    // spec: carnet architecture-securite.md SEC0-23
    /// Checks the manifest against `limits` and the rules of the format; returns the payload
    /// length that the parts announce.
    pub(crate) fn check(&self, limits: &Limits) -> Result<u64, DecodeError> {
        let parts = &self.parts.0;
        too_large(
            Bound::Parts,
            len64(parts.len()),
            u64::from(limits.max_parts),
        )?;
        let mut previous: Option<&str> = None;
        let mut total: u64 = 0;
        let mut payload: u64 = 0;
        for p in parts {
            check_name(&p.name, limits)?;
            if previous.is_some_and(|q| q >= p.name.as_str()) {
                return Err(DecodeError::UnsortedNames);
            }
            previous = Some(&p.name);
            too_large(Bound::Part, p.len, limits.max_part_len)?;
            total = total.saturating_add(p.len);
            too_large(Bound::Total, total, limits.max_total_len)?;
            if p.codec == Codec::Stored && (p.stored_len != p.len || p.stored_digest != p.digest) {
                return Err(DecodeError::CodecMismatch);
            }
            payload = payload
                .checked_add(p.stored_len)
                .ok_or(DecodeError::LengthMismatch)?;
        }
        Ok(payload)
    }
}
