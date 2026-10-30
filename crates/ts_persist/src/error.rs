//! Typed errors of the codec: decoding never panics, it returns one of these (SEC0-23, C6).

use core::fmt;

// spec: carnet architecture-securite.md SEC0-23 ; revues/securite.md S10
/// Bound that a decoding or an encoding exceeded (see [`crate::Limits`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    /// Whole envelope, in stored bytes.
    Envelope,
    /// Manifest bytes.
    Manifest,
    /// Number of parts.
    Parts,
    /// Length of a part name, in bytes.
    Name,
    /// Decoded size of one part.
    Part,
    /// Decoded size of all parts together.
    Total,
    /// `zstd` window announced by a frame.
    Window,
}

// spec: carnet architecture-securite.md SEC0-23, §9.13
/// Why bytes were refused. Decoding stops at the first failure, in the order of the checks
/// described in the crate documentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// The input ends before a field it announces.
    Truncated,
    /// A bound is exceeded: `len` (the value read or reached) is above `max`.
    TooLarge {
        /// Exceeded bound.
        bound: Bound,
        /// Value read or reached.
        len: u64,
        /// Allowed maximum.
        max: u64,
    },
    /// The magic is not [`crate::MAGIC`].
    BadMagic,
    /// Unknown format version.
    UnsupportedVersion(u16),
    /// Unknown signature scheme (only `0`, no signature, exists in phase 0).
    UnsupportedSignature(u16),
    /// The lengths of the header, of the manifest and of the parts do not add up.
    LengthMismatch,
    /// The manifest bytes do not match the envelope digest.
    ManifestDigest,
    /// Bytes that do not decode as the expected schema (`what` names the structure).
    Malformed(&'static str),
    /// Bytes are left after a structure that must end the input.
    TrailingBytes(&'static str),
    /// A part name is not `/`-separated segments of `[a-z0-9._-]`, each non-empty and not
    /// starting with `.`.
    InvalidName,
    /// Part names are not strictly increasing (unsorted or duplicated).
    UnsortedNames,
    /// A stored (uncompressed) part has a decoded length or digest that differs from its
    /// stored one.
    CodecMismatch,
    /// The stored bytes of a part do not match their digest.
    PartDigest(String),
    /// The `zstd` frame of a part does not decode.
    Decompress(String),
    /// A part decodes to another length than its manifest says.
    PartLength(String),
    /// The decoded bytes of a part do not match their digest.
    DecodedDigest(String),
    /// A part the caller needs is absent.
    MissingPart(String),
    /// A part the caller does not know is present.
    UnexpectedPart(String),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => f.write_str("input truncated"),
            Self::TooLarge { bound, len, max } => write!(f, "{bound:?} too large: {len} > {max}"),
            Self::BadMagic => f.write_str("not a time_switcher envelope"),
            Self::UnsupportedVersion(v) => write!(f, "unsupported format version {v}"),
            Self::UnsupportedSignature(s) => write!(f, "unsupported signature scheme {s}"),
            Self::LengthMismatch => f.write_str("lengths do not add up"),
            Self::ManifestDigest => f.write_str("manifest digest mismatch"),
            Self::Malformed(what) => write!(f, "malformed {what}"),
            Self::TrailingBytes(what) => write!(f, "trailing bytes after {what}"),
            Self::InvalidName => f.write_str("invalid part name"),
            Self::UnsortedNames => f.write_str("part names unsorted or duplicated"),
            Self::CodecMismatch => f.write_str("stored part differs from its decoded form"),
            Self::PartDigest(p) => write!(f, "part {p}: stored digest mismatch"),
            Self::Decompress(p) => write!(f, "part {p}: zstd frame does not decode"),
            Self::PartLength(p) => write!(f, "part {p}: decoded length mismatch"),
            Self::DecodedDigest(p) => write!(f, "part {p}: decoded digest mismatch"),
            Self::MissingPart(p) => write!(f, "missing part {p}"),
            Self::UnexpectedPart(p) => write!(f, "unexpected part {p}"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Why an encoding was refused: the encoder never writes what the decoder would refuse with
/// the same [`crate::Limits`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    /// The parts break a rule of the format (name, order, bound).
    Invalid(DecodeError),
    /// The serializer failed (`what` names the structure).
    Serialize(&'static str),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(e) => write!(f, "refused by the format: {e}"),
            Self::Serialize(what) => write!(f, "cannot serialize {what}"),
        }
    }
}

impl std::error::Error for EncodeError {}

impl From<DecodeError> for EncodeError {
    fn from(e: DecodeError) -> Self {
        Self::Invalid(e)
    }
}
