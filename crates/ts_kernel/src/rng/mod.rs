// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §3.4 SEC0-22 ; §10 ; §9.9 K2 à K6, K10
//! Aléa à clé : un tirage est une fonction pure de (clé de flux, sujet, créneau, usage).
//!
//! Chemin sensible (`tools/ci/sensitive-paths.txt`). Toute modification de la dérivation, de la
//! disposition du *nonce* ou des vecteurs figés de `vectors.rs` change le résultat de tous les
//! mondes.
//!
//! - [`WorldKey`] (128 bits) donne une fois [`StreamKeys`] : une [`StreamKey`] de 256 bits par
//!   [`Stream`], dérivée par BLAKE3 `derive_key` sur le **nom** du flux (K2, K4).
//! - [`draw`], [`draw_block`] et [`draw_block_at`] tirent par XChaCha8 en mode compteur (K3).
//! - [`CosmeticSeed`] vient d'une racine privée, séparée des flux (K6, M27).
//!
//! Aucune clé n'est `Copy`, `Clone`, `PartialEq`, `Hash` ni sérialisable, et leur `Debug` ne
//! montre rien (K5). Témoin, puis les refus :
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
//! Les extraits `compile_fail` ont leur témoin ci-dessus : rustdoc stable ne vérifie pas leur
//! code d'erreur, qui reste écrit pour la relecture.

use core::fmt;

use chacha20::XChaCha8;
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};

use crate::StableId;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod vectors;

/// Contexte BLAKE3 de la racine cosmétique (K4, K6).
const COSMETIC_CONTEXT: &str = "time_switcher 2026-10-30 cosmetic v1";

// spec: carnet architecture-securite.md §9.9 K5 ; revues/generation.md P25
/// Clé de monde de 128 bits. Secrète en ligne : tirée par un générateur cryptographique et
/// gardée par l'hôte, jamais dérivée d'une information publique (P25).
///
/// Seul constructeur public : [`WorldKey::from_bytes`]. La clé de test
/// `insecure_from_u64_for_tests` n'existe qu'avec `cfg(test)`, `dev-commands` ou
/// `external-pilot`, donc jamais avec `public` (SEC0-20 refuse `public` avec ces deux
/// fonctionnalités). Témoin, puis le refus, compilé sans fonctionnalité de développement :
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
    /// Clé lue ou tirée par l'hôte.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Clé de test **non sûre** : `seed` en petit-boutiste, puis huit octets nuls. Jamais dans
    /// un binaire public.
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
/// Clé d'un flux, 256 bits, obtenue seulement par [`StreamKeys::get`]. Mêmes règles que
/// [`WorldKey`] : ni copie, ni comparaison, ni affichage.
pub struct StreamKey([u8; 32]);

impl fmt::Debug for StreamKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamKey(..)")
    }
}

/// Racine du domaine cosmétique : type privé, jamais rendu par l'API (K6).
struct CosmeticRoot([u8; 32]);

// spec: revues/architecture.md §6.4 ; revues/generation.md P26 ; carnet architecture-securite.md §3.4 SEC0-22
/// Flux d'aléa, un par domaine : changer les règles d'un domaine ne décale aucun tirage des
/// autres (nombres aléatoires communs, architecture §6.4).
///
/// Énumération fermée. Les noms sont **stables** : la clé d'un flux se dérive de son nom
/// ([`Stream::context`]), jamais de son rang. Les rangs servent seulement d'indice dans
/// [`StreamKeys`] et ne sont jamais renumérotés ; un flux nouveau prend le rang suivant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Stream {
    /// Décisions des personnages pilotés par l'IA.
    Ai = 0,
    /// Combat.
    Combat = 1,
    /// Relations et échanges sociaux.
    Social = 2,
    /// Économie.
    Economy = 3,
    /// Vie de la population en jeu.
    Population = 4,
    /// Météo.
    Weather = 5,
    /// Génération de la ville (P26).
    CityGen = 6,
    /// Part serveur de la ville, tirée par monde (P12, P26).
    CityServer = 7,
    /// Génération de la population (P26).
    PopGen = 8,
    /// Couche cachée de la population (P26).
    PopHidden = 9,
    /// Mobilier (P26).
    Furnish = 10,
    /// Identification et reconnaissance.
    Identification = 11,
}

impl Stream {
    /// Nombre de flux.
    pub const COUNT: usize = 12;

    /// Tous les flux, dans l'ordre de leur rang.
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

    /// Nom stable du flux.
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

    /// Contexte BLAKE3 `derive_key` du flux : `"time_switcher 2026-10-30 stream <Nom> v1"` (K4).
    /// Écrit en dur, comme BLAKE3 le demande ; un test vérifie l'accord avec [`Stream::name`].
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
/// Clés dérivées d'une [`WorldKey`], une fois, à la création ou au chargement du monde : une
/// [`StreamKey`] par flux et la racine cosmétique privée. Pas de copie ni d'affichage.
pub struct StreamKeys {
    streams: [StreamKey; Stream::COUNT],
    cosmetic: CosmeticRoot,
}

impl StreamKeys {
    /// Dérive toutes les clés par BLAKE3 `derive_key`, chacune sous son propre contexte.
    pub fn derive(world: &WorldKey) -> Self {
        Self {
            streams: Stream::ALL.map(|s| StreamKey(blake3::derive_key(s.context(), &world.0))),
            cosmetic: CosmeticRoot(blake3::derive_key(COSMETIC_CONTEXT, &world.0)),
        }
    }

    /// Clé du flux `stream`.
    pub const fn get(&self, stream: Stream) -> &StreamKey {
        &self.streams[stream as usize]
    }

    /// Graine cosmétique de `subject` pour l'usage `usage` (`cosmetic/<usage>`) et le créneau
    /// `slot` : BLAKE3 à clé, sous la racine cosmétique, de `usage ‖ subject (u64 LE) ‖ slot
    /// (u64 LE)` (injectif : la fin a une longueur fixe). Le nom d'usage est une chaîne fixe
    /// du système appelant.
    pub fn cosmetic_seed(&self, usage: &str, subject: StableId, slot: u64) -> CosmeticSeed {
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
/// Graine d'un effet cosmétique (apparence, voix…), qui peut être envoyée au client.
///
/// Elle ne se convertit en aucune clé et aucune fonction de tirage ne l'accepte : connaître
/// des graines cosmétiques ne dit rien des flux (SEC0-22, M27).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CosmeticSeed(u64);

impl CosmeticSeed {
    /// Valeur de la graine.
    pub const fn value(self) -> u64 {
        self.0
    }
}

// spec: carnet architecture-securite.md §9.9 K3
/// *Nonce* XChaCha de 192 bits : `subject (u64 LE) ‖ slot (u64 LE) ‖ purpose (u32 LE) ‖ 0u32`.
fn nonce(subject: StableId, slot: u64, purpose: u32) -> [u8; 24] {
    let mut n = [0u8; 24];
    n[..8].copy_from_slice(&subject.to_snapshot_raw().to_le_bytes());
    n[8..16].copy_from_slice(&slot.to_le_bytes());
    n[16..20].copy_from_slice(&purpose.to_le_bytes());
    n
}

// spec: carnet architecture-securite.md §9.9 K3 ; §3.4 SEC0-22
/// Bloc `block` (512 bits) du flux de clé XChaCha8 désigné par (sujet, créneau, usage), lu en
/// huit `u64` petit-boutistes. Fonction pure.
///
/// L'indice de bloc est un `u16` (décidé) : 65 536 blocs de 8 valeurs par triplet suffisent, et
/// le compteur de 32 bits de XChaCha ne peut alors jamais s'épuiser, donc rien ne panique.
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

/// Premier bloc de (sujet, créneau, usage) : ses 8 valeurs.
pub fn draw_block(key: &StreamKey, subject: StableId, slot: u64, purpose: u32) -> [u64; 8] {
    draw_block_at(key, subject, slot, purpose, 0)
}

// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §9.9 K2, K3
/// Tirage pur : même clé de flux et même (sujet, créneau, usage) donnent la même valeur, quel
/// que soit l'ordre d'exécution. C'est la première valeur de [`draw_block`].
///
/// `slot` est le tick, l'indice d'un bloc d'agenda ou un compteur propre au sujet ; `purpose`
/// distingue les tirages d'un même système au même créneau.
pub fn draw(key: &StreamKey, subject: StableId, slot: u64, purpose: u32) -> u64 {
    draw_block(key, subject, slot, purpose)[0]
}
