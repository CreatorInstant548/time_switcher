// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §3.4 SEC0-22 ; §10 ; §9.9 K2 à K6, K10
//! Keyed randomness: a draw is a pure function of (stream key, subject, slot, purpose).
//!
//! Sensitive path (`tools/ci/sensitive-paths.txt`). Any change to the derivation, to the
//! *nonce* layout or to the frozen vectors of `vectors.rs` changes the result of every
//! world.
//!
//! - [`WorldKey`] (128 bits) yields [`StreamKeys`] once: a 256-bit stream key per
//!   [`Stream`], derived by BLAKE3 `derive_key` on the stream **name** (K2, K4).
//! - A draw goes through a [`DrawSite`], which binds one stream to one purpose under a
//!   name (#26 m1): [`StreamKeys::draw`], [`StreamKeys::draw_block`] and
//!   [`StreamKeys::draws`] draw with XChaCha8 in counter mode (K3). The raw draw functions
//!   and the stream keys stay inside this module.
//! - [`ArtifactSeed`] yields [`ArtifactKeys`]: the generation streams of the map and of a
//!   population family only, never a per-world or play stream (P25, #26 m2).
//! - [`CosmeticSeed`] comes from a private root, separate from the streams (K6, M27).
//!
//! No key is `Copy`, `Clone`, `PartialEq`, `Hash` or serializable, and their `Debug` shows
//! nothing (K5). A witness, then the refusals:
//!
//! ```
//! use ts_kernel::StableIdAllocator;
//! use ts_kernel::rng::{DrawSite, Stream, StreamKeys, WorldKey};
//! const HIT: DrawSite = DrawSite::new(Stream::Combat, 1, "combat.hit");
//! let keys = StreamKeys::derive(&WorldKey::from_bytes([7; 16]));
//! let mut ids = StableIdAllocator::for_new_world();
//! if let Some(subject) = ids.allocate() {
//!     let _ = keys.draw(HIT, subject, 0);
//! }
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
//! ```compile_fail,E0624
//! use ts_kernel::rng::{Stream, StreamKeys, WorldKey};
//! let keys = StreamKeys::derive(&WorldKey::from_bytes([7; 16]));
//! let _ = keys.get(Stream::Ai);
//! ```
//!
//! ```compile_fail,E0308
//! use ts_kernel::rng::{StreamKeys, WorldKey, WORLD_SUBJECT};
//! let keys = StreamKeys::derive(&WorldKey::from_bytes([7; 16]));
//! let seed = keys.cosmetic_seed("hair", WORLD_SUBJECT, 0);
//! let _ = keys.draw(seed, WORLD_SUBJECT, 0);
//! ```
//!
//! ```compile_fail,E0599
//! let _ = ts_kernel::rng::ArtifactStream::CityServer;
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
    /// Key from its bytes, with no check: tests, public test keys and the kernel's own
    /// layout key. A host that loads or draws the key of a world uses [`Self::checked`].
    /// Outside the hosts, persistence and tests, `cargo xtask check` refuses this type
    /// (#26 m3): a rule never builds a key.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    // spec: revues/generation.md P25
    /// Key loaded or drawn by a host, or `None` if it is weak: its 16 bytes all equal (the
    /// zero key included), a buffer left unfilled, or its 8 high bytes all zero, the shape of
    /// a test key (`insecure_from_u64_for_tests`). Neither comes from a cryptographic
    /// generator (P25); a drawn key is refused with probability 2^-64.
    pub const fn checked(bytes: [u8; 16]) -> Option<Self> {
        if is_weak(&bytes) {
            None
        } else {
            Some(Self(bytes))
        }
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

/// True if the 16 bytes are all equal (a buffer left unfilled) or the 8 high ones are all
/// zero (a test key): never a drawn secret.
const fn is_weak(bytes: &[u8; 16]) -> bool {
    let (mut constant, mut high_zero) = (true, true);
    let mut i = 0;
    while i < bytes.len() {
        constant &= bytes[i] == bytes[0];
        if i >= 8 {
            high_zero &= bytes[i] == 0;
        }
        i += 1;
    }
    constant || high_zero
}

impl fmt::Debug for WorldKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WorldKey(..)")
    }
}

// spec: carnet architecture-securite.md §9.9 K2, K5
/// Key of a stream, 256 bits, private to this module: rules reach it only through a
/// [`DrawSite`] (#26 m1). Same rules as [`WorldKey`]: no copy, no comparison, no display.
pub(crate) struct StreamKey([u8; 32]);

impl StreamKey {
    /// Key of `stream` for the 16 bytes of a world key or of an artifact seed: BLAKE3
    /// `derive_key` under the context of the stream (K2, K4), the only derivation of a stream.
    fn derive(stream: Stream, bytes: &[u8; 16]) -> Self {
        Self(blake3::derive_key(stream.context(), bytes))
    }
}

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
/// private stream key per stream and the private cosmetic root. No copy or display. Draws
/// go through a [`DrawSite`] ([`StreamKeys::draw`]).
pub struct StreamKeys {
    streams: [StreamKey; Stream::COUNT],
    cosmetic: CosmeticRoot,
}

impl StreamKeys {
    /// Derives all the keys by BLAKE3 `derive_key`, each under its own context.
    pub fn derive(world: &WorldKey) -> Self {
        Self {
            streams: Stream::ALL.map(|s| StreamKey::derive(s, &world.0)),
            cosmetic: CosmeticRoot(blake3::derive_key(COSMETIC_CONTEXT, &world.0)),
        }
    }

    // spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §9.9 K2, K3
    /// First value of `site` for (`subject`, `slot`): the same keys and inputs give the same
    /// value, whatever the execution order.
    ///
    /// `slot` is the tick, the index of a schedule block or a counter specific to the subject.
    /// **Two calls with the same inputs return the same value.** Two distinct values in one
    /// slot come from two sites, or from the values of one block ([`Self::draw_block`],
    /// [`Self::draws`]); this value is the first of block 0, so never use both on one triple.
    /// A draw with no entity (the weather of the world) takes [`WORLD_SUBJECT`].
    #[must_use]
    pub fn draw(&self, site: DrawSite, subject: StableId, slot: u64) -> u64 {
        draw(self.get(site.stream), subject, slot, site.purpose)
    }

    /// Block `block` (8 values) of `site` for (`subject`, `slot`). Block 0 starts with
    /// [`Self::draw`].
    #[must_use]
    pub fn draw_block(&self, site: DrawSite, subject: StableId, slot: u64, block: u16) -> [u64; 8] {
        draw_block_at(self.get(site.stream), subject, slot, site.purpose, block)
    }

    /// Successive values of `site` for (`subject`, `slot`): block 0, then block 1, and so on,
    /// up to 2^19 values. Its first value is [`Self::draw`].
    pub fn draws(&self, site: DrawSite, subject: StableId, slot: u64) -> Draws<'_> {
        Draws::new(self.get(site.stream), subject, slot, site.purpose)
    }

    /// Key of stream `stream`, inside the kernel only. Exhaustive `match` with constant
    /// indices: an index outside the array would be refused at compile time
    /// (`unconditional_panic`), so `get` never panics.
    pub(crate) const fn get(&self, stream: Stream) -> &StreamKey {
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
#[must_use]
pub(crate) fn draw_block_at(
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
#[must_use]
pub(crate) fn draw_block(key: &StreamKey, subject: StableId, slot: u64, purpose: u32) -> [u64; 8] {
    draw_block_at(key, subject, slot, purpose, 0)
}

// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §9.9 K2, K3
/// Pure draw: the same stream key and the same (subject, slot, purpose) give the same value,
/// whatever the execution order. It is the first value of [`draw_block`]. Outside this
/// module, a draw goes through a [`DrawSite`] ([`StreamKeys::draw`]).
#[must_use]
pub(crate) fn draw(key: &StreamKey, subject: StableId, slot: u64, purpose: u32) -> u64 {
    draw_block(key, subject, slot, purpose)[0]
}

// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §9.9 K3
/// Draw site: one stream and one purpose, bound together under a name, declared once as a
/// `const` next to the system that draws there.
///
/// Two sites of the rules never share their stream and purpose: each plugin lists its
/// sites in [`crate::Plugin::draw_sites`], and [`crate::Sim::build`] refuses a pair listed
/// twice ([`crate::BuildError::DrawSiteConflict`]). Two systems can thus never get the same
/// draws by picking the same purpose, and a public draw (a lock) never reveals a hidden one
/// (a camera). The stream is part of the site, so no system passes the key of another domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DrawSite {
    stream: Stream,
    purpose: u32,
    name: &'static str,
}

impl DrawSite {
    /// Site `name` (for example `"combat.hit"`) of `stream`, with `purpose`.
    pub const fn new(stream: Stream, purpose: u32, name: &'static str) -> Self {
        Self {
            stream,
            purpose,
            name,
        }
    }

    /// Stream of the site.
    pub const fn stream(self) -> Stream {
        self.stream
    }

    /// Purpose of the site, unique within its stream.
    pub const fn purpose(self) -> u32 {
        self.purpose
    }

    /// Name of the site, for diagnostics.
    pub const fn name(self) -> &'static str {
        self.name
    }
}

// spec: carnet architecture-securite.md §9.9 K9
/// Subject of a draw that concerns no entity, such as the weather of the world. Its value is
/// the last index of the map namespace, but it is a draw subject only, never a reference
/// kept in the state. Its site serves world-level draws only, never an entity: since sites
/// are unique, its draws never meet those of an entity or a place, whatever its value.
pub const WORLD_SUBJECT: StableId = StableId::from_snapshot_raw(u64::MAX);

/// Successive values of one (site, subject, slot): see [`StreamKeys::draws`].
pub struct Draws<'a> {
    key: &'a StreamKey,
    subject: StableId,
    slot: u64,
    purpose: u32,
    /// Next block to compute; beyond `u16::MAX`, the values are exhausted.
    next_block: u32,
    block: [u64; 8],
    /// Next value of `block` to return; 8 when the block is used up.
    pos: usize,
}

impl<'a> Draws<'a> {
    const fn new(key: &'a StreamKey, subject: StableId, slot: u64, purpose: u32) -> Self {
        Self {
            key,
            subject,
            slot,
            purpose,
            next_block: 0,
            block: [0; 8],
            pos: 8,
        }
    }
}

impl Iterator for Draws<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.pos >= self.block.len() {
            let block = u16::try_from(self.next_block).ok()?;
            self.block = draw_block_at(self.key, self.subject, self.slot, self.purpose, block);
            self.next_block += 1;
            self.pos = 0;
        }
        let value = self.block.get(self.pos).copied();
        self.pos += 1;
        value
    }
}

impl core::iter::FusedIterator for Draws<'_> {}

impl fmt::Debug for Draws<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Draws(..)")
    }
}

// spec: revues/generation.md P25, P26, §5
/// Streams of a generated artifact (the map, a population family): the only ones an
/// [`ArtifactSeed`] gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArtifactStream {
    /// Generation of the city.
    CityGen,
    /// Generation of the population.
    PopGen,
    /// Furnishing: fixed furniture of the map, movable furniture of the family (P8).
    Furnish,
}

impl ArtifactStream {
    /// The stream, with the same key as in [`StreamKeys`] for the same 16 bytes.
    pub const fn stream(self) -> Stream {
        match self {
            Self::CityGen => Stream::CityGen,
            Self::PopGen => Stream::PopGen,
            Self::Furnish => Stream::Furnish,
        }
    }
}

// spec: revues/generation.md P25 ; revues/securite.md S2
/// Seed of a generated artifact: public for the map and the solo family, secret for the
/// online family (P25).
///
/// Unlike a [`WorldKey`], it yields only the artifact streams ([`ArtifactKeys`]): never
/// `CityServer`, `PopHidden` nor a play stream, which come only from the secret key of a
/// world. A step that holds the public map seed thus cannot draw the server part of the city
/// or the hidden layer under a key known to all (S2).
pub struct ArtifactSeed([u8; 16]);

impl ArtifactSeed {
    /// Seed from its bytes, with no check: the public map and solo family seeds, and tests.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Secret seed loaded or drawn by a host (the online family), or `None` if it is weak,
    /// as [`WorldKey::checked`].
    pub const fn checked(bytes: [u8; 16]) -> Option<Self> {
        if is_weak(&bytes) {
            None
        } else {
            Some(Self(bytes))
        }
    }
}

impl fmt::Debug for ArtifactSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ArtifactSeed(..)")
    }
}

/// Draw site of an artifact stream, see [`DrawSite`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactSite {
    stream: ArtifactStream,
    purpose: u32,
    name: &'static str,
}

impl ArtifactSite {
    /// Site `name` of `stream`, with `purpose`.
    pub const fn new(stream: ArtifactStream, purpose: u32, name: &'static str) -> Self {
        Self {
            stream,
            purpose,
            name,
        }
    }

    /// The same site, to list it with the others or to draw it from [`StreamKeys`].
    pub const fn site(self) -> DrawSite {
        DrawSite::new(self.stream.stream(), self.purpose, self.name)
    }
}

// spec: carnet architecture-securite.md §9.9 K2, K5
/// Keys of the artifact streams, derived from an [`ArtifactSeed`] under the same contexts as
/// [`StreamKeys::derive`]: the same 16 bytes give the same draws. No copy or display.
pub struct ArtifactKeys {
    city_gen: StreamKey,
    pop_gen: StreamKey,
    furnish: StreamKey,
}

impl ArtifactKeys {
    /// Derives the three keys by BLAKE3 `derive_key`.
    pub fn derive(seed: &ArtifactSeed) -> Self {
        Self {
            city_gen: StreamKey::derive(Stream::CityGen, &seed.0),
            pop_gen: StreamKey::derive(Stream::PopGen, &seed.0),
            furnish: StreamKey::derive(Stream::Furnish, &seed.0),
        }
    }

    const fn get(&self, stream: ArtifactStream) -> &StreamKey {
        match stream {
            ArtifactStream::CityGen => &self.city_gen,
            ArtifactStream::PopGen => &self.pop_gen,
            ArtifactStream::Furnish => &self.furnish,
        }
    }

    /// First value of `site`, as [`StreamKeys::draw`].
    #[must_use]
    pub fn draw(&self, site: ArtifactSite, subject: StableId, slot: u64) -> u64 {
        draw(self.get(site.stream), subject, slot, site.purpose)
    }

    /// Block `block` of `site`, as [`StreamKeys::draw_block`].
    #[must_use]
    pub fn draw_block(
        &self,
        site: ArtifactSite,
        subject: StableId,
        slot: u64,
        block: u16,
    ) -> [u64; 8] {
        draw_block_at(self.get(site.stream), subject, slot, site.purpose, block)
    }

    /// Successive values of `site`, as [`StreamKeys::draws`].
    pub fn draws(&self, site: ArtifactSite, subject: StableId, slot: u64) -> Draws<'_> {
        Draws::new(self.get(site.stream), subject, slot, site.purpose)
    }
}

impl fmt::Debug for ArtifactKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ArtifactKeys(..)")
    }
}

/// Seal of [`SiteDraw`]: no other crate implements it.
mod sealed {
    pub trait Sealed {}
}

// spec: carnet architecture-securite.md §9.9 K3
/// Holder of stream keys that draws at a [`DrawSite`], for code generic over who holds the
/// keys: a generator run while building, or by a host or a test.
///
/// Sealed, implemented only by [`StreamKeys`], which only a holder of the [`WorldKey`]
/// derives (hosts, tools, tests), and by [`crate::WorldStreams`], which checks that an
/// active plugin declares the site (#26 m1). A system thus never reaches the keys of the
/// world without that check. Witness, then the refusal:
///
/// ```
/// use ts_kernel::rng::{DrawSite, SiteDraw, Stream, StreamKeys, WORLD_SUBJECT, WorldKey};
/// fn roll(keys: &impl SiteDraw) -> u64 {
///     keys.draw(DrawSite::new(Stream::Weather, 1, "weather.rain"), WORLD_SUBJECT, 0)
/// }
/// let _ = roll(&StreamKeys::derive(&WorldKey::from_bytes([7; 16])));
/// ```
///
/// ```compile_fail,E0277
/// use ts_kernel::StableId;
/// use ts_kernel::rng::{DrawSite, Draws, SiteDraw};
/// struct Mine;
/// impl SiteDraw for Mine {
///     fn draw(&self, _: DrawSite, _: StableId, _: u64) -> u64 { 0 }
///     fn draw_block(&self, _: DrawSite, _: StableId, _: u64, _: u16) -> [u64; 8] { [0; 8] }
///     fn draws(&self, _: DrawSite, _: StableId, _: u64) -> Draws<'_> { todo!() }
/// }
/// ```
pub trait SiteDraw: sealed::Sealed {
    /// First value of `site`, see [`StreamKeys::draw`].
    #[must_use]
    fn draw(&self, site: DrawSite, subject: StableId, slot: u64) -> u64;

    /// Block `block` of `site`, see [`StreamKeys::draw_block`].
    #[must_use]
    fn draw_block(&self, site: DrawSite, subject: StableId, slot: u64, block: u16) -> [u64; 8];

    /// Successive values of `site`, see [`StreamKeys::draws`].
    fn draws(&self, site: DrawSite, subject: StableId, slot: u64) -> Draws<'_>;
}

impl sealed::Sealed for StreamKeys {}
impl sealed::Sealed for crate::WorldStreams {}

impl SiteDraw for StreamKeys {
    fn draw(&self, site: DrawSite, subject: StableId, slot: u64) -> u64 {
        Self::draw(self, site, subject, slot)
    }

    fn draw_block(&self, site: DrawSite, subject: StableId, slot: u64, block: u16) -> [u64; 8] {
        Self::draw_block(self, site, subject, slot, block)
    }

    fn draws(&self, site: DrawSite, subject: StableId, slot: u64) -> Draws<'_> {
        Self::draws(self, site, subject, slot)
    }
}

impl SiteDraw for crate::WorldStreams {
    fn draw(&self, site: DrawSite, subject: StableId, slot: u64) -> u64 {
        Self::draw(self, site, subject, slot)
    }

    fn draw_block(&self, site: DrawSite, subject: StableId, slot: u64, block: u16) -> [u64; 8] {
        Self::draw_block(self, site, subject, slot, block)
    }

    fn draws(&self, site: DrawSite, subject: StableId, slot: u64) -> Draws<'_> {
        Self::draws(self, site, subject, slot)
    }
}
