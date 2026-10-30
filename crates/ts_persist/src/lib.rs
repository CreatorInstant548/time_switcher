// spec: revues/architecture.md §12.1 ; revues/generation.md §8.1, P30 ; carnet architecture-securite.md SEC0-23
//! Bounded snapshot codec and addressed-parts format of time_switcher.
//!
//! Phase 0 (T8): a versioned envelope, a manifest of parts with BLAKE3 digests, `zstd`
//! compression in pure Rust (`ruzstd`), bounded decoding, and the bridge to the kernel
//! snapshot. No storage engine, journal, incremental snapshot or signature yet.
//!
//! A file is one envelope:
//!
//! | Offset | Size | Field |
//! |---|---|---|
//! | 0 | 8 | magic [`MAGIC`] |
//! | 8 | 2 | format version, `u16` LE ([`FORMAT_VERSION`]) |
//! | 10 | 2 | signature scheme, `u16` LE: `0` (none) is the only value accepted |
//! | 12 | 4 | manifest length, `u32` LE |
//! | 16 | 8 | payload length, `u64` LE |
//! | 24 | 32 | BLAKE3 digest of the manifest bytes |
//! | 56 | … | manifest (postcard), then the stored bytes of each part, in manifest order |
//!
//! The manifest lists each part (name, codec, stored length and digest, decoded length and
//! digest), sorted by name. The digests form a tree: the envelope carries the manifest digest
//! (the root), the manifest carries the digest of each part (the leaves). A part named
//! `<part>/<chunk>` is a chunk of a larger part (sector x family, architecture §12.1).
//!
//! **Order of checks** (archsec Q3): envelope, then manifest digest, then manifest content
//! and bounds, then the stored digest of **every** part, and only then decompression and the
//! decoded digest of each part. Nothing is decoded by postcard or `zstd` before its bytes
//! match a digest.
//!
//! **Signature** (P30, not in phase 0): a future format version signs the manifest digest;
//! the signature scheme field is where it goes. Today any non-zero scheme is refused.
//!
//! **No panic on any input** (SEC0-23, C6): the lints below are denied in the whole library,
//! with no local exemption (E6); every failure is a typed [`DecodeError`]. **No allocation
//! sized by a length read from the input** (Q2): buffers grow only as bytes are actually
//! decoded. **No recursive type** in the schema (Q2): the depth is fixed at compile time.
//!
//! The world key is never encoded (L4, Q4): the kernel snapshot carries none, and this crate
//! never sees one. The kernel parts are read back by the kernel's own canonical decoder
//! (`MemorySnapshot::from_canonical`, AS-36); the host then calls the kernel's `restore` with
//! the key. This crate never rebuilds a world itself.

#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::arithmetic_side_effects
)]

mod codec;
mod envelope;
mod error;
mod kernel;
mod limits;
mod manifest;
mod schema;

// Development API (m2 of the review of #49): the fuzzing body and its corpus. It is absent
// from the `public` profile (`not_public!`) and hidden from the documentation. A Cargo
// feature of this crate would hide it from clippy instead: `cargo xtask check` lints this
// crate with the default features only.
ts_kernel::not_public! {
    #[doc(hidden)]
    pub mod fuzzing;
}

pub use codec::{Parts, decode_parts, encode_parts};
pub use envelope::{FORMAT_VERSION, HEADER_LEN, MAGIC, open};
pub use error::{Bound, DecodeError, EncodeError};
pub use kernel::{META_PART, STATE_PART, decode_snapshot, encode_snapshot};
pub use limits::Limits;
pub use manifest::{Codec, Digest, Manifest, PartEntry};
pub use schema::{MAX_SCHEMA_DEPTH, Schema, Seq};
