//! Bridge with the kernel snapshot: stores its header and canonical state, never a key, and
//! decodes them back through the kernel's own canonical decoder.

use ts_kernel::{CanonicalLimits, MemorySnapshot, StateLayout};

use crate::codec::{Parts, decode_parts, encode_parts};
use crate::error::{DecodeError, EncodeError};
use crate::limits::Limits;
use crate::manifest::Codec;

/// Part holding `MemorySnapshot::header_bytes` (rules fingerprint, tick length, date,
/// active plugins).
pub const META_PART: &str = "kernel.meta";
/// Part holding `MemorySnapshot::canonical_bytes`, the kernel's canonical state.
pub const STATE_PART: &str = "kernel.state";

// spec: revues/architecture.md §12.1 ; carnet architecture-securite.md SEC0-23, §10
/// Encodes a kernel snapshot into an envelope with the parts [`META_PART`] and
/// [`STATE_PART`]. **No key**: the kernel snapshot carries none (L4), and this crate never
/// sees one (Q4).
///
/// # Errors
///
/// See [`encode_parts`].
pub fn encode_snapshot(
    snap: &MemorySnapshot,
    codec: Codec,
    limits: &Limits,
) -> Result<Vec<u8>, EncodeError> {
    let meta = snap.header_bytes();
    encode_parts(
        &[(META_PART, &meta), (STATE_PART, snap.canonical_bytes())],
        codec,
        limits,
    )
}

// spec: carnet architecture-securite.md SEC0-23, §9.13, §10 ; revues/securite.md S24
/// Decodes an envelope written by [`encode_snapshot`]: exactly the two kernel parts, each
/// checked by the codec, then read by `MemorySnapshot::from_canonical` against the state
/// types of `layout`, bounded by `limits.max_part_len`.
///
/// The host then restores it with `Sim::restore(…, &WorldKey)`, which keeps all its checks
/// and is the only path to a world: this crate never rebuilds one (m3 of the review of
/// #30, AS-36).
///
/// # Errors
///
/// Any [`DecodeError`] of [`decode_parts`], a missing or unexpected part, or
/// [`DecodeError::Kernel`].
pub fn decode_snapshot(
    bytes: &[u8],
    limits: &Limits,
    layout: &StateLayout,
) -> Result<MemorySnapshot, DecodeError> {
    snapshot_from_parts(decode_parts(bytes, limits)?, limits, layout)
}

/// Kernel snapshot from checked parts: exactly the two kernel parts, then the kernel's
/// canonical decoder.
pub(crate) fn snapshot_from_parts(
    parts: Parts,
    limits: &Limits,
    layout: &StateLayout,
) -> Result<MemorySnapshot, DecodeError> {
    let [meta, state] = parts.into_exact([META_PART, STATE_PART])?;
    MemorySnapshot::from_canonical(&meta, state, layout, &canonical_limits(limits))
        .map_err(DecodeError::Kernel)
}

/// Bounds of the kernel decoder: the same as the codec's for one part.
pub(crate) const fn canonical_limits(limits: &Limits) -> CanonicalLimits {
    CanonicalLimits {
        max_len: limits.max_part_len,
    }
}
