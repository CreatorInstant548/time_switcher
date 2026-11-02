//! Versioned envelope: magic, format version, signature scheme, lengths, manifest digest.

use crate::error::{Bound, DecodeError, EncodeError};
use crate::limits::Limits;
use crate::manifest::{Manifest, len64, manifest_digest};

/// Magic of an envelope.
pub const MAGIC: [u8; 8] = *b"TSPERST\0";
/// Format version written, and the only one read.
pub const FORMAT_VERSION: u16 = 1;
/// Header length, before the manifest.
pub const HEADER_LEN: usize = 56;
/// Signature scheme `0`: no signature (phase 0).
const NO_SIGNATURE: u16 = 0;

/// Reader over a slice: every read is checked, a short input is [`DecodeError::Truncated`].
struct Cursor<'a>(&'a [u8]);

impl<'a> Cursor<'a> {
    fn array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        let (head, tail) = self
            .0
            .split_first_chunk::<N>()
            .ok_or(DecodeError::Truncated)?;
        self.0 = tail;
        Ok(*head)
    }

    fn slice(&mut self, n: u64) -> Result<&'a [u8], DecodeError> {
        let n = usize::try_from(n).map_err(|_| DecodeError::Truncated)?;
        let (head, tail) = self.0.split_at_checked(n).ok_or(DecodeError::Truncated)?;
        self.0 = tail;
        Ok(head)
    }
}

// spec: carnet architecture-securite.md SEC0-23, §9.13 ; revues/generation.md §8.1
/// Opens an envelope: checks the header, the lengths, the manifest digest, then decodes and
/// checks the manifest. Returns the manifest and the payload (the stored bytes of the parts,
/// not yet checked against their digests).
///
/// # Errors
///
/// The first failed check, in this order: envelope bound, magic, version, signature scheme,
/// manifest bound, lengths, manifest digest, announced part count, manifest schema, manifest
/// rules.
pub fn open<'a>(bytes: &'a [u8], limits: &Limits) -> Result<(Manifest, &'a [u8]), DecodeError> {
    let max = limits.max_envelope_len;
    if len64(bytes.len()) > max {
        return Err(DecodeError::TooLarge {
            bound: Bound::Envelope,
            len: len64(bytes.len()),
            max,
        });
    }
    let mut c = Cursor(bytes);
    if c.array::<8>()? != MAGIC {
        return Err(DecodeError::BadMagic);
    }
    let version = u16::from_le_bytes(c.array()?);
    if version != FORMAT_VERSION {
        return Err(DecodeError::UnsupportedVersion(version));
    }
    let signature = u16::from_le_bytes(c.array()?);
    if signature != NO_SIGNATURE {
        return Err(DecodeError::UnsupportedSignature(signature));
    }
    let manifest_len = u32::from_le_bytes(c.array()?);
    let payload_len = u64::from_le_bytes(c.array()?);
    let digest = c.array::<32>()?;
    if manifest_len > limits.max_manifest_len {
        return Err(DecodeError::TooLarge {
            bound: Bound::Manifest,
            len: u64::from(manifest_len),
            max: u64::from(limits.max_manifest_len),
        });
    }
    let manifest_bytes = c.slice(u64::from(manifest_len))?;
    let payload = c.slice(payload_len)?;
    if !c.0.is_empty() {
        return Err(DecodeError::TrailingBytes("envelope"));
    }
    if manifest_digest(manifest_bytes) != digest {
        return Err(DecodeError::ManifestDigest);
    }
    check_announced_parts(manifest_bytes, limits)?;
    let (manifest, rest) = postcard::take_from_bytes::<Manifest>(manifest_bytes)
        .map_err(|_| DecodeError::Malformed("manifest"))?;
    if !rest.is_empty() {
        return Err(DecodeError::TrailingBytes("manifest"));
    }
    if manifest.check(limits)? != payload_len {
        return Err(DecodeError::LengthMismatch);
    }
    Ok((manifest, payload))
}

// spec: carnet architecture-securite.md SEC0-23, §9.13
/// Checks the number of parts the manifest announces against `limits` before the manifest
/// is decoded (m4 of #50), so that decoding never builds more entries than allowed.
///
/// The manifest is a postcard structure whose first field is the [`crate::Seq`] of parts: its
/// bytes start with the element count, a postcard varint of `usize`. It is read here with
/// postcard's own `u64` varint, which is the same encoding up to 10 bytes; a count that does
/// not fit in `usize` is above any bound and refused as too large. [`Manifest::check`] checks
/// the count again on the decoded entries.
fn check_announced_parts(manifest_bytes: &[u8], limits: &Limits) -> Result<(), DecodeError> {
    let (count, _) = postcard::take_from_bytes::<u64>(manifest_bytes)
        .map_err(|_| DecodeError::Malformed("manifest"))?;
    let max = u64::from(limits.max_parts);
    if count > max {
        return Err(DecodeError::TooLarge {
            bound: Bound::Parts,
            len: count,
            max,
        });
    }
    Ok(())
}

/// Writes an envelope around the manifest and the stored bytes of the parts, then checks
/// that it opens with the same `limits`.
pub(crate) fn seal(
    manifest: &Manifest,
    stored: &[&[u8]],
    limits: &Limits,
) -> Result<Vec<u8>, EncodeError> {
    let manifest_bytes =
        postcard::to_allocvec(manifest).map_err(|_| EncodeError::Serialize("manifest"))?;
    let manifest_len = u32::try_from(manifest_bytes.len()).map_err(|_| DecodeError::TooLarge {
        bound: Bound::Manifest,
        len: len64(manifest_bytes.len()),
        max: u64::from(limits.max_manifest_len),
    })?;
    let payload_len = stored
        .iter()
        .fold(0_u64, |n, s| n.saturating_add(len64(s.len())));
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    out.extend_from_slice(&NO_SIGNATURE.to_le_bytes());
    out.extend_from_slice(&manifest_len.to_le_bytes());
    out.extend_from_slice(&payload_len.to_le_bytes());
    out.extend_from_slice(&manifest_digest(&manifest_bytes));
    out.extend_from_slice(&manifest_bytes);
    for s in stored {
        out.extend_from_slice(s);
    }
    open(&out, limits)?;
    Ok(out)
}
