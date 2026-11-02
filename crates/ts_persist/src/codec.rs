//! Parts: encoding, and decoding in the order of the checks (envelope, manifest, parts).

use std::borrow::Cow;
use std::io::Read;

use ruzstd::decoding::StreamingDecoder;
use ruzstd::decoding::errors::{FrameDecoderError, FrameHeaderError};
use ruzstd::encoding::{CompressionLevel, compress_to_vec};

use crate::envelope::{open, seal};
use crate::error::{Bound, DecodeError, EncodeError};
use crate::limits::{Limits, check_peak};
use crate::manifest::{Codec, Manifest, PartEntry, check_name, len64, part_digest};
use crate::schema::Seq;

/// Window that `ruzstd` 0.9.1 announces at [`CompressionLevel::Fastest`] (128 KiB), checked
/// by the test of the window bound.
pub(crate) const ENCODER_WINDOW: u64 = 128 << 10;
/// Read size of the decompression loop.
const CHUNK: usize = 16 << 10;

// spec: revues/generation.md §8.1, P30
/// Decoded parts, checked against the manifest, sorted by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parts(Vec<(String, Vec<u8>)>);

impl Parts {
    /// Names, sorted.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|(n, _)| n.as_str())
    }

    /// Bytes of part `name`.
    pub fn get(&self, name: &str) -> Option<&[u8]> {
        let i = self
            .0
            .binary_search_by(|(n, _)| n.as_str().cmp(name))
            .ok()?;
        self.0.get(i).map(|(_, b)| b.as_slice())
    }

    // spec: revues/generation.md P30
    /// Takes exactly the parts `names`, in that order. The names must be distinct: a repeated
    /// name is reported as [`DecodeError::MissingPart`].
    ///
    /// # Errors
    ///
    /// [`DecodeError::UnexpectedPart`] for a part outside `names`, then
    /// [`DecodeError::MissingPart`] for a name with no part.
    pub fn into_exact<const N: usize>(self, names: [&str; N]) -> Result<[Vec<u8>; N], DecodeError> {
        let mut found: [Option<Vec<u8>>; N] = [const { None }; N];
        for (name, bytes) in self.0 {
            match names
                .iter()
                .zip(found.iter_mut())
                .find(|(n, _)| **n == name)
            {
                Some((_, slot)) => *slot = Some(bytes),
                None => return Err(DecodeError::UnexpectedPart(name)),
            }
        }
        let mut out = [const { Vec::new() }; N];
        for ((name, bytes), slot) in names.iter().zip(found).zip(out.iter_mut()) {
            *slot = bytes.ok_or_else(|| DecodeError::MissingPart((*name).to_owned()))?;
        }
        Ok(out)
    }
}

/// Bounds that can be checked before any compression or hashing: memory peak of `limits`
/// (M41h), number of parts, names,
/// decoded sizes, and the window of our own encoder.
fn precheck(sorted: &[&(&str, &[u8])], codec: Codec, limits: &Limits) -> Result<(), DecodeError> {
    check_peak(limits)?;
    let count = len64(sorted.len());
    if count > u64::from(limits.max_parts) {
        return Err(DecodeError::TooLarge {
            bound: Bound::Parts,
            len: count,
            max: u64::from(limits.max_parts),
        });
    }
    let mut total: u64 = 0;
    for &&(name, raw) in sorted {
        check_name(name, limits)?;
        let len = len64(raw.len());
        total = total.saturating_add(len);
        for (bound, len, max) in [
            (Bound::Part, len, limits.max_part_len),
            (Bound::Total, total, limits.max_total_len),
        ] {
            if len > max {
                return Err(DecodeError::TooLarge { bound, len, max });
            }
        }
    }
    if codec == Codec::Zstd && ENCODER_WINDOW > limits.max_window {
        return Err(DecodeError::TooLarge {
            bound: Bound::Window,
            len: ENCODER_WINDOW,
            max: limits.max_window,
        });
    }
    Ok(())
}

// spec: revues/architecture.md §12.1 ; revues/generation.md §8.1, P30
/// Encodes `parts` (name, bytes) into one envelope, each part stored with `codec`.
///
/// # Errors
///
/// [`EncodeError::Invalid`] if a name is invalid or repeated, or if a bound of `limits` is
/// exceeded: the encoder never writes an envelope that the decoder refuses with the same
/// `limits`. The cheap bounds are checked before any compression; the envelope is then
/// checked by the decoder's own [`open`].
pub fn encode_parts(
    parts: &[(&str, &[u8])],
    codec: Codec,
    limits: &Limits,
) -> Result<Vec<u8>, EncodeError> {
    let mut sorted: Vec<&(&str, &[u8])> = parts.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    precheck(&sorted, codec, limits)?;
    let mut entries = Vec::new();
    let mut stored = Vec::new();
    for &&(name, raw) in &sorted {
        let digest = part_digest(raw);
        let (bytes, stored_digest): (Cow<'_, [u8]>, _) = match codec {
            Codec::Stored => (Cow::Borrowed(raw), digest),
            Codec::Zstd => {
                let frame = compress_to_vec(raw, CompressionLevel::Fastest);
                let d = part_digest(&frame);
                (Cow::Owned(frame), d)
            }
        };
        entries.push(PartEntry {
            name: name.to_owned(),
            codec,
            stored_len: len64(bytes.len()),
            stored_digest,
            len: len64(raw.len()),
            digest,
        });
        stored.push(bytes);
    }
    let manifest = Manifest {
        parts: Seq(entries),
    };
    let slices: Vec<&[u8]> = stored.iter().map(AsRef::as_ref).collect();
    seal(&manifest, &slices, limits)
}

// spec: carnet architecture-securite.md SEC0-23, §9.13 ; revues/securite.md S10, S24
/// Decodes an envelope into its parts. The envelope and the manifest are checked first, then
/// the stored digest of every part, and only then is each part decompressed, at most up to
/// its announced length, and checked against its decoded digest.
///
/// # Errors
///
/// The first failed check, as a typed [`DecodeError`]; never a panic.
pub fn decode_parts(bytes: &[u8], limits: &Limits) -> Result<Parts, DecodeError> {
    let (manifest, payload) = open(bytes, limits)?;
    let mut rest = payload;
    let mut stored = Vec::new();
    for p in &manifest.parts.0 {
        let n = usize::try_from(p.stored_len).map_err(|_| DecodeError::Truncated)?;
        let (s, tail) = rest.split_at_checked(n).ok_or(DecodeError::Truncated)?;
        if part_digest(s) != p.stored_digest {
            return Err(DecodeError::PartDigest(p.name.clone()));
        }
        stored.push(s);
        rest = tail;
    }
    if !rest.is_empty() {
        return Err(DecodeError::LengthMismatch);
    }
    let mut out = Vec::new();
    for (p, s) in manifest.parts.0.into_iter().zip(stored) {
        let bytes = match p.codec {
            // Same length and digest as the stored bytes: `Manifest::check` requires it.
            Codec::Stored => s.to_vec(),
            Codec::Zstd => {
                let bytes = decompress(&p.name, s, p.len, limits)?;
                if part_digest(&bytes) != p.digest {
                    return Err(DecodeError::DecodedDigest(p.name));
                }
                bytes
            }
        };
        out.push((p.name, bytes));
    }
    Ok(Parts(out))
}

// spec: carnet architecture-securite.md SEC0-23, §9.13
/// Decodes one `zstd` frame into exactly `len` bytes. The window the frame announces is
/// bounded first; then the output is read through `take(len + 1)`, and the loop stops as soon
/// as it would pass `len` (no decompression bomb). The buffer grows geometrically with the
/// bytes actually produced, its capacity capped at `len`, which `limits` already bounds:
/// nothing is allocated up front from a length read in the input (Q2).
fn decompress(
    name: &str,
    stored: &[u8],
    len: u64,
    limits: &Limits,
) -> Result<Vec<u8>, DecodeError> {
    let failed = || DecodeError::Decompress(name.to_owned());
    let too_long = || DecodeError::PartLength(name.to_owned());
    let mut src = stored;
    let decoder = StreamingDecoder::new_with_max_window_size(&mut src, limits.max_window).map_err(
        |e| match e {
            FrameDecoderError::WindowSizeTooBig { requested, max } => DecodeError::TooLarge {
                bound: Bound::Window,
                len: requested,
                max,
            },
            // At or above `ruzstd`'s own maximum, so above any configured bound (m6).
            FrameDecoderError::FrameHeaderError(FrameHeaderError::WindowTooBig { got }) => {
                DecodeError::TooLarge {
                    bound: Bound::Window,
                    len: got,
                    max: limits.max_window,
                }
            }
            _ => failed(),
        },
    )?;
    let cap = usize::try_from(len).map_err(|_| too_long())?;
    let mut reader = decoder.take(len.saturating_add(1));
    let mut out = Vec::new();
    let mut chunk = [0_u8; CHUNK];
    loop {
        let n = reader.read(&mut chunk).map_err(|_| failed())?;
        if n == 0 {
            break;
        }
        let piece = chunk.get(..n).ok_or_else(failed)?;
        let room = cap.saturating_sub(out.len());
        if n > room {
            return Err(too_long());
        }
        if out.capacity().saturating_sub(out.len()) < n {
            out.reserve_exact(out.capacity().max(n).min(room));
        }
        out.extend_from_slice(piece);
    }
    drop(reader);
    if out.len() != cap {
        return Err(too_long());
    }
    if !src.is_empty() {
        return Err(DecodeError::TrailingBytes("zstd frame"));
    }
    Ok(out)
}
