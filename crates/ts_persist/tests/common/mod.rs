//! Helpers shared by the integration tests: an independent writer of envelopes, so that a
//! test can forge any manifest with consistent digests.

#![cfg(test)]
#![allow(dead_code)]

use ts_persist::{Codec, Digest, FORMAT_VERSION, MAGIC, Manifest, PartEntry, Seq};

/// Contexts of the format, written again here: a change in the crate breaks the tests.
pub const PART_CONTEXT: &str = "time_switcher 2026-10-30 persist part v1";
pub const MANIFEST_CONTEXT: &str = "time_switcher 2026-10-30 persist manifest v1";

pub fn digest(context: &str, bytes: &[u8]) -> Digest {
    *blake3::Hasher::new_derive_key(context)
        .update(bytes)
        .finalize()
        .as_bytes()
}

/// Entry of a part stored as is.
pub fn stored(name: &str, bytes: &[u8]) -> PartEntry {
    let d = digest(PART_CONTEXT, bytes);
    PartEntry {
        name: name.to_owned(),
        codec: Codec::Stored,
        stored_len: bytes.len() as u64,
        stored_digest: d,
        len: bytes.len() as u64,
        digest: d,
    }
}

/// Envelope with the given header fields, manifest bytes and payload, digest recomputed.
pub fn forge_raw(version: u16, signature: u16, manifest: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut out = MAGIC.to_vec();
    out.extend_from_slice(&version.to_le_bytes());
    out.extend_from_slice(&signature.to_le_bytes());
    out.extend_from_slice(&(manifest.len() as u32).to_le_bytes());
    out.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    out.extend_from_slice(&digest(MANIFEST_CONTEXT, manifest));
    out.extend_from_slice(manifest);
    out.extend_from_slice(payload);
    out
}

/// Envelope around `entries` (in the given order) and `payload`, with consistent digests.
pub fn forge(entries: Vec<PartEntry>, payload: &[u8]) -> Vec<u8> {
    let manifest = postcard::to_allocvec(&Manifest {
        parts: Seq(entries),
    })
    .unwrap();
    forge_raw(FORMAT_VERSION, 0, &manifest, payload)
}
