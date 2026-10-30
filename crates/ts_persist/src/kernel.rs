//! Bridge with the kernel snapshot: encodes what a `MemorySnapshot` exposes, never a key.

use serde::{Deserialize, Serialize};
use ts_kernel::{MemorySnapshot, RulesFingerprint, SimTime};

use crate::codec::{Parts, decode_parts, encode_parts};
use crate::error::{DecodeError, EncodeError};
use crate::limits::Limits;
use crate::manifest::Codec;
use crate::schema::flat_schema;

/// Part holding [`SnapshotImage::rules`] and [`SnapshotImage::now`] (postcard).
pub const META_PART: &str = "kernel.meta";
/// Part holding [`SnapshotImage::state`], the kernel's canonical encoding.
pub const STATE_PART: &str = "kernel.state";

flat_schema! {
    /// Content of [`META_PART`].
    #[derive(Serialize, Deserialize)]
    pub struct Meta {
        /// Rules fingerprint.
        pub rules: [u8; 32],
        /// Date of the snapshot, in milliseconds.
        pub now_ms: i64,
    }
}

// spec: revues/architecture.md §12.1 ; carnet architecture-securite.md §9.10 L4, §9.13
/// What a kernel snapshot exposes outside the kernel: its rules fingerprint, its date and
/// the canonical encoding of the whole state. **No key**: the kernel snapshot carries none
/// (L4), and this crate never sees one (Q4).
///
/// The kernel cannot yet rebuild a `MemorySnapshot` from these bytes (#46): once it can,
/// [`decode_snapshot`] returns one, and the host restores it with `restore(…, &WorldKey)`,
/// which keeps all its checks (m3 of the review of #30).
// TODO(#46): decode into a `MemorySnapshot` once the kernel offers a decoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotImage {
    /// Rules fingerprint of the snapshot.
    pub rules: RulesFingerprint,
    /// Date of the snapshot.
    pub now: SimTime,
    /// Canonical encoding of the state (`MemorySnapshot::canonical_bytes`).
    pub state: Vec<u8>,
}

impl SnapshotImage {
    /// Image of a kernel snapshot.
    pub fn of(snap: &MemorySnapshot) -> Self {
        Self {
            rules: snap.rules_fingerprint(),
            now: snap.now(),
            state: snap.canonical_bytes().to_vec(),
        }
    }

    /// Encodes the image (see [`encode_snapshot`]).
    ///
    /// # Errors
    ///
    /// See [`encode_parts`].
    pub fn encode(&self, codec: Codec, limits: &Limits) -> Result<Vec<u8>, EncodeError> {
        encode(&self.rules, self.now, &self.state, codec, limits)
    }
}

fn encode(
    rules: &RulesFingerprint,
    now: SimTime,
    state: &[u8],
    codec: Codec,
    limits: &Limits,
) -> Result<Vec<u8>, EncodeError> {
    let meta = Meta {
        rules: *rules.as_bytes(),
        now_ms: now.as_millis(),
    };
    let meta = postcard::to_allocvec(&meta).map_err(|_| EncodeError::Serialize("snapshot meta"))?;
    encode_parts(&[(META_PART, &meta), (STATE_PART, state)], codec, limits)
}

// spec: revues/architecture.md §12.1 ; carnet architecture-securite.md SEC0-23
/// Encodes a kernel snapshot into an envelope with the parts [`META_PART`] and
/// [`STATE_PART`].
///
/// # Errors
///
/// See [`encode_parts`].
pub fn encode_snapshot(
    snap: &MemorySnapshot,
    codec: Codec,
    limits: &Limits,
) -> Result<Vec<u8>, EncodeError> {
    encode(
        &snap.rules_fingerprint(),
        snap.now(),
        snap.canonical_bytes(),
        codec,
        limits,
    )
}

// spec: carnet architecture-securite.md SEC0-23, §9.13 ; revues/securite.md S24
/// Decodes an envelope written by [`encode_snapshot`]: exactly the two kernel parts, each
/// checked, then the metadata decoded by postcard from a checked, bounded buffer.
///
/// # Errors
///
/// Any [`DecodeError`] of [`decode_parts`], a missing or unexpected part, or malformed
/// metadata.
pub fn decode_snapshot(bytes: &[u8], limits: &Limits) -> Result<SnapshotImage, DecodeError> {
    image_from_parts(decode_parts(bytes, limits)?)
}

/// Kernel snapshot from checked parts: exactly the two kernel parts, then the metadata.
pub(crate) fn image_from_parts(parts: Parts) -> Result<SnapshotImage, DecodeError> {
    let [meta, state] = parts.into_exact([META_PART, STATE_PART])?;
    let (meta, rest) = postcard::take_from_bytes::<Meta>(&meta)
        .map_err(|_| DecodeError::Malformed("snapshot meta"))?;
    if !rest.is_empty() {
        return Err(DecodeError::TrailingBytes("snapshot meta"));
    }
    Ok(SnapshotImage {
        rules: RulesFingerprint::from_bytes(meta.rules),
        now: SimTime::from_millis(meta.now_ms),
        state,
    })
}
