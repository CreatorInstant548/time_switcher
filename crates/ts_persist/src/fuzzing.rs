//! Body of the fuzzing target `fuzz/fuzz_targets/decode.rs` and its initial corpus, kept in
//! the library so that they compile and run on stable (A14: no local nightly).

use ts_kernel::{RulesFingerprint, SimTime};

use crate::codec::{Parts, decode_parts, encode_parts};
use crate::error::DecodeError;
use crate::kernel::{SnapshotImage, image_from_parts};
use crate::limits::Limits;
use crate::manifest::Codec;

// spec: carnet architecture-securite.md SEC0-24 (révisé)
/// Runs every decoder on `data` with [`Limits::fuzzing`]: the envelope and manifest, the
/// parts, then the kernel snapshot on those parts (as `decode_snapshot` does, without
/// decoding the parts twice). It must never panic, whatever `data` is.
///
/// # Errors
///
/// The error of the parts decoder, if any.
pub fn decode(data: &[u8]) -> Result<Parts, DecodeError> {
    let parts = decode_parts(data, &Limits::fuzzing())?;
    let _ = image_from_parts(parts.clone());
    Ok(parts)
}

// spec: carnet architecture-securite.md SEC0-24 (révisé)
/// Initial corpus of the target: valid envelopes, stored and compressed, written by
/// `fuzz/seeds` into the corpus directory (regenerable, never committed).
pub fn seeds() -> Vec<Vec<u8>> {
    let limits = Limits::fuzzing();
    let state: Vec<u8> = (0..2048_u32)
        .flat_map(|i| (i % 251).to_le_bytes())
        .collect();
    let image = SnapshotImage {
        rules: RulesFingerprint::of(&[("life", 1)]),
        now: SimTime::from_millis(120_000),
        state,
    };
    let few: [(&str, &[u8]); 3] = [("a", b""), ("b/c", b"seed"), ("pop.solo/0001", &[7; 300])];
    let mut out = Vec::new();
    for codec in [Codec::Stored, Codec::Zstd] {
        out.extend(encode_parts(&[], codec, &limits).ok());
        out.extend(encode_parts(&few, codec, &limits).ok());
        out.extend(image.encode(codec, &limits).ok());
    }
    out
}
