//! Parts: encoding, and decoding in the order of the checks (envelope, manifest, parts).

use std::borrow::Cow;
use std::io::Read;

use ruzstd::decoding::StreamingDecoder;
use ruzstd::decoding::errors::FrameDecoderError;
use ruzstd::encoding::{CompressionLevel, compress_to_vec};

use crate::envelope::{open, seal};
use crate::error::{Bound, DecodeError, EncodeError};
use crate::limits::Limits;
use crate::manifest::{Codec, Manifest, PartEntry, check_name, len64, part_digest};
use crate::schema::Seq;

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
        self.0
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, b)| b.as_slice())
    }

    /// Number of parts.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// True if there is no part.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // spec: revues/generation.md P30
    /// Takes exactly the parts `names` (distinct), in that order.
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

// spec: revues/architecture.md §12.1 ; revues/generation.md §8.1, P30
/// Encodes `parts` (name, bytes) into one envelope, each part stored with `codec`.
///
/// # Errors
///
/// [`EncodeError::Invalid`] if a name is invalid or repeated, or if a bound of `limits` is
/// exceeded: the encoder never writes an envelope that the decoder refuses with the same
/// `limits`.
pub fn encode_parts(
    parts: &[(&str, &[u8])],
    codec: Codec,
    limits: &Limits,
) -> Result<Vec<u8>, EncodeError> {
    let mut sorted: Vec<&(&str, &[u8])> = parts.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    let mut entries = Vec::new();
    let mut stored = Vec::new();
    for &&(name, raw) in &sorted {
        check_name(name, limits)?;
        let bytes: Cow<'_, [u8]> = match codec {
            Codec::Stored => Cow::Borrowed(raw),
            Codec::Zstd => Cow::Owned(compress_to_vec(raw, CompressionLevel::Fastest)),
        };
        entries.push(PartEntry {
            name: name.to_owned(),
            codec,
            stored_len: len64(bytes.len()),
            stored_digest: part_digest(&bytes),
            len: len64(raw.len()),
            digest: part_digest(raw),
        });
        stored.push(bytes);
    }
    let manifest = Manifest {
        parts: Seq(entries),
    };
    manifest.check(limits)?;
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
            Codec::Stored => s.to_vec(),
            Codec::Zstd => decompress(&p.name, s, p.len, limits)?,
        };
        if len64(bytes.len()) != p.len {
            return Err(DecodeError::PartLength(p.name));
        }
        if part_digest(&bytes) != p.digest {
            return Err(DecodeError::DecodedDigest(p.name));
        }
        out.push((p.name, bytes));
    }
    Ok(Parts(out))
}

// spec: carnet architecture-securite.md SEC0-23, §9.13
/// Decodes one `zstd` frame. The window the frame announces is bounded first; then the
/// output is read through `take(len + 1)`, so decompression stops one byte past the
/// announced length (no decompression bomb), and the output buffer grows only with the bytes
/// actually produced (Q2).
fn decompress(
    name: &str,
    stored: &[u8],
    len: u64,
    limits: &Limits,
) -> Result<Vec<u8>, DecodeError> {
    let failed = || DecodeError::Decompress(name.to_owned());
    let mut src = stored;
    let decoder = StreamingDecoder::new_with_max_window_size(&mut src, limits.max_window).map_err(
        |e| match e {
            FrameDecoderError::WindowSizeTooBig { requested, max } => DecodeError::TooLarge {
                bound: Bound::Window,
                len: requested,
                max,
            },
            _ => failed(),
        },
    )?;
    let mut out = Vec::new();
    decoder
        .take(len.saturating_add(1))
        .read_to_end(&mut out)
        .map_err(|_| failed())?;
    if len64(out.len()) != len {
        return Err(DecodeError::PartLength(name.to_owned()));
    }
    if !src.is_empty() {
        return Err(DecodeError::TrailingBytes("zstd frame"));
    }
    Ok(out)
}
