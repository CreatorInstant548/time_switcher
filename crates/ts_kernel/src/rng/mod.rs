// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §3.4 SEC0-22 ; §10 ; §9.9 K2 à K6, K10
//! Keyed randomness: a draw is a pure function of (stream key, subject, slot, purpose).
//!
//! Sensitive path (`tools/ci/sensitive-paths.txt`). Any change to the derivation, to the
//! *nonce* layout or to the frozen vectors of `vectors.rs` changes the result of every
//! world.
//!
//! - [`WorldKey`] (128 bits) yields [`StreamKeys`] once: a 256-bit [`StreamKey`] per
//!   [`Stream`], derived by BLAKE3 `derive_key` on the stream **name** (K2, K4).
//! - [`draw`], [`draw_block`] and [`draw_block_at`] draw with XChaCha8 in counter mode (K3).
//! - [`CosmeticSeed`] comes from a private root, separate from the streams (K6, M27).
//!
//! No key is `Copy`, `Clone`, `PartialEq`, `Hash` or serializable, and their `Debug` shows
//! nothing (K5). A witness, then the refusals:
//!
//! ```
//! use ts_kernel::rng::{Stream, StreamKeys, WorldKey, draw};
//! let keys = StreamKeys::derive(&WorldKey::from_bytes([7; 16]));
//! let _ = draw(keys.get(Stream::Combat), ts_kernel::StableId::from_snapshot_raw(1), 0, 0);
//! ```
//!
//! ```compile_fail,E0599
//! let _ = ts_kernel::rng::WorldKey::from_bytes([7; 16]).clone();
//! ```
//!
//! ```compile_fail,E0369
//! use ts_kernel::rng::WorldKey;
//! let _ = WorldKey::from_bytes([7; 16]) == WorldKey::from_bytes([7; 16]);
//! ```
//!
//! ```compile_fail,E0308
//! use ts_kernel::rng::{Stream, StreamKey, StreamKeys, WorldKey};
//! let keys = StreamKeys::derive(&WorldKey::from_bytes([7; 16]));
//! let _: StreamKey = keys.get(Stream::Ai).clone();
//! ```
//!
//! ```compile_fail,E0308
//! use ts_kernel::rng::{StreamKeys, WorldKey, draw};
//! let keys = StreamKeys::derive(&WorldKey::from_bytes([7; 16]));
//! let seed = keys.cosmetic_seed("hair", ts_kernel::StableId::from_snapshot_raw(1), 0);
//! let _ = draw(&seed, ts_kernel::StableId::from_snapshot_raw(1), 0, 0);
//! ```
//!
//! The `compile_fail` snippets have their witness above: stable rustdoc does not check their
//! error code, which stays written for review.

use core::fmt;

use chacha20::XChaCha8;
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};

use crate::StableId;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod vectors;

/// BLAKE3 context of the cosmetic root (K4, K6).
const COSMETIC_CONTEXT: &str = "time_switcher 2026-10-30 cosmetic v1";

// spec: carnet architecture-securite.md §9.9 K5 ; revues/generation.md P25
/// 128-bit world key. Secret online: drawn by a cryptographic generator and kept by the
/// host, never derived from public information (P25).
///
/// The only public constructor is [`WorldKey::from_bytes`]. The test key
/// `insecure_from_u64_for_tests` exists only with `cfg(test)`, `dev-commands` or
/// `external-pilot`, hence never with `public` (SEC0-20 refuses `public` with these two
/// features). A witness, then the refusal, compiled without any development feature:
///
/// ```
/// let _ = ts_kernel::rng::WorldKey::from_bytes([0; 16]);
/// ```
#[cfg_attr(
    not(any(feature = "dev-commands", feature = "external-pilot")),
    doc = "```compile_fail,E0599\nlet _ = ts_kernel::rng::WorldKey::insecure_from_u64_for_tests(1);\n```"
)]
pub struct WorldKey([u8; 16]);

impl WorldKey {
    /// Key read or drawn by the host.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// **Insecure** test key: `seed` in little-endian, then eight zero bytes. Never in a
    /// public binary.
    #[cfg(any(test, feature = "dev-commands", feature = "external-pilot"))]
    pub const fn insecure_from_u64_for_tests(seed: u64) -> Self {
        let s = seed.to_le_bytes();
        Self([
            s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    }
}

impl fmt::Debug for WorldKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WorldKey(..)")
    }
}

// spec: carnet architecture-securite.md §9.9 K2, K5
/// Key of a stream, 256 bits, obtained only through [`StreamKeys::get`]. Same rules as
/// [`WorldKey`]: no copy, no comparison, no display.
pub struct StreamKey([u8; 32]);

impl fmt::Debug for StreamKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamKey(..)")
    }
}

/// Root of the cosmetic domain: private type, never returned by the API (K6).
struct CosmeticRoot([u8; 32]);

// spec: revues/architecture.md §6.4 ; revues/generation.md P26 ; carnet architecture-securite.md §3.4 SEC0-22
/// Random streams, one per domain: changing the rules of one domain shifts no draw of the
/// others (common random numbers, architecture §6.4).
///
/// Closed enumeration. The names are **stable**: the key of a stream is derived from its name
/// ([`Stream::context`]), never from its rank. Ranks only serve as an index in
/// [`StreamKeys`] and are never renumbered; a new stream takes the next rank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Stream {
    /// Decisions of AI-piloted characters.
    Ai = 0,
    /// Combat.
    Combat = 1,
    /// Relations and social exchanges.
    Social = 2,
    /// Economy.
    Economy = 3,
    /// Life of the in-play population.
    Population = 4,
    /// Weather.
    Weather = 5,
    /// Generation of the city (P26).
    CityGen = 6,
    /// Server part of the city, drawn per world (P12, P26).
    CityServer = 7,
    /// Generation of the population (P26).
    PopGen = 8,
    /// Hidden layer of the population (P26).
    PopHidden = 9,
    /// Furnishing (P26).
    Furnish = 10,
    /// Identification and recognition.
    Identification = 11,
}

impl Stream {
    /// Number of streams.
    pub const COUNT: usize = 12;

    /// All streams, in order of their rank.
    pub const ALL: [Self; Self::COUNT] = [
        Self::Ai,
        Self::Combat,
        Self::Social,
        Self::Economy,
        Self::Population,
        Self::Weather,
        Self::CityGen,
        Self::CityServer,
        Self::PopGen,
        Self::PopHidden,
        Self::Furnish,
        Self::Identification,
    ];

    /// Stable name of the stream.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ai => "Ai",
            Self::Combat => "Combat",
            Self::Social => "Social",
            Self::Economy => "Economy",
            Self::Population => "Population",
            Self::Weather => "Weather",
            Self::CityGen => "CityGen",
            Self::CityServer => "CityServer",
            Self::PopGen => "PopGen",
            Self::PopHidden => "PopHidden",
            Self::Furnish => "Furnish",
            Self::Identification => "Identification",
        }
    }

    /// BLAKE3 `derive_key` context of the stream:
    /// `"time_switcher 2026-10-30 stream <Name> v1"` (K4). Hard-coded, as BLAKE3 requires; a
    /// test checks that it agrees with [`Stream::name`].
    pub const fn context(self) -> &'static str {
        match self {
            Self::Ai => "time_switcher 2026-10-30 stream Ai v1",
            Self::Combat => "time_switcher 2026-10-30 stream Combat v1",
            Self::Social => "time_switcher 2026-10-30 stream Social v1",
            Self::Economy => "time_switcher 2026-10-30 stream Economy v1",
            Self::Population => "time_switcher 2026-10-30 stream Population v1",
            Self::Weather => "time_switcher 2026-10-30 stream Weather v1",
            Self::CityGen => "time_switcher 2026-10-30 stream CityGen v1",
            Self::CityServer => "time_switcher 2026-10-30 stream CityServer v1",
            Self::PopGen => "time_switcher 2026-10-30 stream PopGen v1",
            Self::PopHidden => "time_switcher 2026-10-30 stream PopHidden v1",
            Self::Furnish => "time_switcher 2026-10-30 stream Furnish v1",
            Self::Identification => "time_switcher 2026-10-30 stream Identification v1",
        }
    }
}

// spec: carnet architecture-securite.md §9.9 K2, K6
/// Keys derived from a [`WorldKey`], once, when the world is created or loaded: one
/// [`StreamKey`] per stream and the private cosmetic root. No copy or display.
pub struct StreamKeys {
    streams: [StreamKey; Stream::COUNT],
    cosmetic: CosmeticRoot,
}

impl StreamKeys {
    /// Derives all the keys by BLAKE3 `derive_key`, each under its own context.
    pub fn derive(world: &WorldKey) -> Self {
        Self {
            streams: Stream::ALL.map(|s| StreamKey(blake3::derive_key(s.context(), &world.0))),
            cosmetic: CosmeticRoot(blake3::derive_key(COSMETIC_CONTEXT, &world.0)),
        }
    }

    /// Key of stream `stream`. Exhaustive `match` with constant indices: an index outside the
    /// array would be refused at compile time (`unconditional_panic`), so `get` never panics.
    pub const fn get(&self, stream: Stream) -> &StreamKey {
        let s = &self.streams;
        match stream {
            Stream::Ai => &s[0],
            Stream::Combat => &s[1],
            Stream::Social => &s[2],
            Stream::Economy => &s[3],
            Stream::Population => &s[4],
            Stream::Weather => &s[5],
            Stream::CityGen => &s[6],
            Stream::CityServer => &s[7],
            Stream::PopGen => &s[8],
            Stream::PopHidden => &s[9],
            Stream::Furnish => &s[10],
            Stream::Identification => &s[11],
        }
    }

    /// Cosmetic seed of `subject` for purpose `usage` (`cosmetic/<usage>`) and slot
    /// `slot`: keyed BLAKE3, under the cosmetic root, of `usage ‖ subject (u64 LE) ‖ slot
    /// (u64 LE)` (injective: the end has a fixed length). The purpose is a `'static` string,
    /// fixed at compile time by the calling system.
    pub fn cosmetic_seed(&self, usage: &'static str, subject: StableId, slot: u64) -> CosmeticSeed {
        let mut hasher = blake3::Hasher::new_keyed(&self.cosmetic.0);
        hasher.update(usage.as_bytes());
        hasher.update(&subject.to_snapshot_raw().to_le_bytes());
        hasher.update(&slot.to_le_bytes());
        let mut out = [0u8; 8];
        hasher.finalize_xof().fill(&mut out);
        CosmeticSeed(u64::from_le_bytes(out))
    }
}

impl fmt::Debug for StreamKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamKeys(..)")
    }
}

// spec: carnet architecture-securite.md §9.9 K6 ; attendus-mesurables.md M27
/// Seed of a cosmetic effect (appearance, voice...), which may be sent to the client.
///
/// It converts to no key and no draw function accepts it: knowing cosmetic seeds says
/// nothing about the streams (SEC0-22, M27).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CosmeticSeed(u64);

impl CosmeticSeed {
    /// Value of the seed.
    pub const fn value(self) -> u64 {
        self.0
    }
}

// spec: carnet architecture-securite.md §9.9 K3
/// 192-bit XChaCha *nonce*: `subject (u64 LE) ‖ slot (u64 LE) ‖ purpose (u32 LE) ‖ 0u32`.
fn nonce(subject: StableId, slot: u64, purpose: u32) -> [u8; 24] {
    let mut n = [0u8; 24];
    n[..8].copy_from_slice(&subject.to_snapshot_raw().to_le_bytes());
    n[8..16].copy_from_slice(&slot.to_le_bytes());
    n[16..20].copy_from_slice(&purpose.to_le_bytes());
    n
}

// spec: carnet architecture-securite.md §9.9 K3 ; §3.4 SEC0-22
/// Block `block` (512 bits) of the XChaCha8 key stream designated by (subject, slot, purpose),
/// read as eight little-endian `u64`s. Pure function.
///
/// The block index is a `u16` (decided): 65,536 blocks of 8 values per triple are enough, and
/// XChaCha's 32-bit counter can then never run out, so nothing panics.
pub fn draw_block_at(
    key: &StreamKey,
    subject: StableId,
    slot: u64,
    purpose: u32,
    block: u16,
) -> [u64; 8] {
    let mut cipher = XChaCha8::new(&key.0.into(), &nonce(subject, slot, purpose).into());
    if block != 0 {
        cipher.seek(u64::from(block) * 64);
    }
    let mut bytes = [0u8; 64];
    cipher.apply_keystream(&mut bytes);
    let mut out = [0u64; 8];
    for (o, chunk) in out.iter_mut().zip(bytes.as_chunks::<8>().0) {
        *o = u64::from_le_bytes(*chunk);
    }
    out
}

/// First block of (subject, slot, purpose): its 8 values.
pub fn draw_block(key: &StreamKey, subject: StableId, slot: u64, purpose: u32) -> [u64; 8] {
    draw_block_at(key, subject, slot, purpose, 0)
}

// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §9.9 K2, K3
/// Pure draw: the same stream key and the same (subject, slot, purpose) give the same value,
/// whatever the execution order. It is the first value of [`draw_block`].
///
/// `slot` is the tick, the index of a schedule block or a counter specific to the subject;
/// `purpose` tells apart the draws of one system in the same slot.
pub fn draw(key: &StreamKey, subject: StableId, slot: u64, purpose: u32) -> u64 {
    draw_block(key, subject, slot, purpose)[0]
}
