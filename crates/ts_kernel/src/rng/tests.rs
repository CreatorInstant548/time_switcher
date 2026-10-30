// spec: carnet architecture-securite.md §3.4 SEC0-22 ; §9.9 K2 à K6
//! Tests de l'aléa à clé : contextes, disposition, uniformité, indépendance et M27.

use std::collections::BTreeSet;

use chacha20::XChaCha8;
use chacha20::cipher::{KeyIvInit, StreamCipher};

use super::*;

fn sid(raw: u64) -> StableId {
    StableId::from_snapshot_raw(raw)
}

fn keys() -> StreamKeys {
    StreamKeys::derive(&WorldKey::from_bytes(*b"time_switcher_t5"))
}

fn words(bytes: &[u8]) -> impl Iterator<Item = u64> + '_ {
    bytes
        .as_chunks::<8>()
        .0
        .iter()
        .map(|c| u64::from_le_bytes(*c))
}

#[test]
fn contexts_are_built_from_names() {
    for (rank, s) in Stream::ALL.into_iter().enumerate() {
        assert_eq!(s as usize, rank, "rang de {s:?}");
        assert_eq!(s.name(), format!("{s:?}"));
        assert_eq!(
            s.context(),
            format!("time_switcher 2026-10-30 stream {} v1", s.name())
        );
    }
    let contexts: BTreeSet<_> = Stream::ALL.iter().map(|s| s.context()).collect();
    assert_eq!(contexts.len(), Stream::COUNT);
    assert!(!contexts.contains(COSMETIC_CONTEXT));
}

/// Rang attendu de chaque flux, par un `match` exhaustif sans joker : un flux ajouté oblige à
/// écrire son rang ici, et un rang changé fait échouer le test.
const fn expected_rank(s: Stream) -> usize {
    match s {
        Stream::Ai => 0,
        Stream::Combat => 1,
        Stream::Social => 2,
        Stream::Economy => 3,
        Stream::Population => 4,
        Stream::Weather => 5,
        Stream::CityGen => 6,
        Stream::CityServer => 7,
        Stream::PopGen => 8,
        Stream::PopHidden => 9,
        Stream::Furnish => 10,
        Stream::Identification => 11,
    }
}

#[test]
fn ranks_are_stable() {
    let keys = keys();
    for s in Stream::ALL {
        let rank = expected_rank(s);
        assert!(rank < Stream::COUNT, "{s:?}");
        assert_eq!(Stream::ALL[rank], s);
        assert_eq!(s as usize, rank);
        assert!(core::ptr::eq(keys.get(s), &keys.streams[rank]), "{s:?}");
    }
}

#[test]
fn stream_keys_are_blake3_derived_and_distinct() {
    let world = WorldKey::from_bytes(*b"time_switcher_t5");
    let keys = StreamKeys::derive(&world);
    let mut seen = BTreeSet::new();
    for s in Stream::ALL {
        assert_eq!(keys.get(s).0, blake3::derive_key(s.context(), &world.0));
        assert!(seen.insert(keys.get(s).0));
    }
    assert!(seen.insert(keys.cosmetic.0));
    assert!(!seen.iter().any(|k| k[..16] == world.0));
}

#[test]
fn debug_shows_no_key_material() {
    let world = WorldKey::from_bytes([0xab; 16]);
    let keys = StreamKeys::derive(&world);
    assert_eq!(format!("{world:?}"), "WorldKey(..)");
    assert_eq!(format!("{keys:?}"), "StreamKeys(..)");
    assert_eq!(format!("{:?}", keys.get(Stream::Ai)), "StreamKey(..)");
}

#[test]
fn insecure_test_key_layout() {
    let k = WorldKey::insecure_from_u64_for_tests(0x0807_0605_0403_0201);
    assert_eq!(k.0, [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn nonce_layout_is_k3() {
    let n = nonce(
        sid(0x1122_3344_5566_7788),
        0x99aa_bbcc_ddee_ff00,
        0x0102_0304,
    );
    let expected: [u8; 24] = [
        0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa,
        0x99, 0x04, 0x03, 0x02, 0x01, 0, 0, 0, 0,
    ];
    assert_eq!(n, expected);
}

#[test]
fn blocks_follow_the_xchacha8_keystream() {
    let keys = keys();
    let key = keys.get(Stream::Social);
    let (subject, slot, purpose) = (sid(9), 77, 3);
    let mut stream = [0u8; 64 * 3];
    let mut cipher = XChaCha8::new(&key.0.into(), &nonce(subject, slot, purpose).into());
    cipher.apply_keystream(&mut stream);
    let all: Vec<u64> = words(&stream).collect();
    let block = draw_block(key, subject, slot, purpose);
    assert_eq!(block[..], all[..8]);
    assert_eq!(draw_block_at(key, subject, slot, purpose, 0), block);
    assert_eq!(
        draw_block_at(key, subject, slot, purpose, 1)[..],
        all[8..16]
    );
    assert_eq!(draw_block_at(key, subject, slot, purpose, 2)[..], all[16..]);
    assert_eq!(draw(key, subject, slot, purpose), block[0]);
}

#[test]
fn draw_is_pure() {
    let (a, b) = (keys(), keys());
    for i in 0..1_000 {
        let s = Stream::ALL[i % Stream::COUNT];
        let i = i as u64;
        assert_eq!(
            draw(a.get(s), sid(i), i * 3, i as u32),
            draw(b.get(s), sid(i), i * 3, i as u32)
        );
    }
}

/// Chi-deux sur 1 024 bacs : 1 023 degrés de liberté, moyenne 1 023, écart type ≈ 45,2. Les
/// bornes à ± 4 écarts types refusent aussi une sortie trop régulière.
fn assert_uniform(values: impl Iterator<Item = u64>, bin: impl Fn(u64) -> usize, what: &str) {
    let mut counts = [0u64; 1024];
    let mut n = 0u64;
    for v in values {
        counts[bin(v)] += 1;
        n += 1;
    }
    let expected = n as f64 / 1024.0;
    let chi2: f64 = counts
        .iter()
        .map(|&c| (c as f64 - expected).powi(2) / expected)
        .sum();
    assert!((842.0..1204.0).contains(&chi2), "{what} : chi-deux {chi2}");
}

#[test]
fn chi_square_over_a_million_draws() {
    let keys = keys();
    let key = keys.get(Stream::Combat);
    let by_subject = || (0..1_000_000u64).map(|i| draw(key, sid(i), 0, 0));
    assert_uniform(
        by_subject(),
        |v| (v >> 54) as usize,
        "bits forts, par sujet",
    );
    assert_uniform(
        by_subject(),
        |v| (v & 1023) as usize,
        "bits faibles, par sujet",
    );
    let by_slot = (0..125_000u64).flat_map(|i| draw_block(key, sid(5), i, 1));
    assert_uniform(
        by_slot,
        |v| (v >> 27 & 1023) as usize,
        "bits du milieu, par créneau",
    );
}

/// Chi-deux conjoint : les 5 bits forts de deux tirages voisins forment un bac sur 1 024. Une
/// dépendance entre voisins déséquilibre les bacs.
#[test]
fn joint_chi_square_of_neighbouring_draws() {
    let keys = keys();
    let key = keys.get(Stream::Population);
    let joint = |a: u64, b: u64| (a >> 59) << 5 | b >> 59;
    let slots =
        (0..1_000_000u64).map(|i| joint(draw(key, sid(7), i, 0), draw(key, sid(7), i + 1, 0)));
    assert_uniform(slots, |v| v as usize, "créneaux voisins, 5 × 5 bits");
    let subjects =
        (0..1_000_000u64).map(|i| joint(draw(key, sid(i), 3, 0), draw(key, sid(i + 1), 3, 0)));
    assert_uniform(subjects, |v| v as usize, "sujets voisins, 5 × 5 bits");
}

/// Équilibre des bits de `a ^ b` sur 10⁵ paires : chaque bit vaut 1 dans la moitié des cas, à
/// ± 4 écarts types (σ ≈ 158), si les deux suites sont indépendantes.
fn assert_independent(pairs: impl Iterator<Item = (u64, u64)>, what: &str) {
    let mut ones = [0u32; 64];
    let mut n = 0u32;
    for (a, b) in pairs {
        assert_ne!(a, b, "{what} : valeurs égales");
        let x = a ^ b;
        for (bit, count) in ones.iter_mut().enumerate() {
            *count += (x >> bit & 1) as u32;
        }
        n += 1;
    }
    for (bit, &count) in ones.iter().enumerate() {
        let deviation = (f64::from(count) - f64::from(n) / 2.0).abs();
        assert!(deviation < 633.0, "{what} : bit {bit}, écart {deviation}");
    }
}

#[test]
fn streams_slots_purposes_and_blocks_are_independent() {
    let keys = keys();
    let (ai, combat) = (keys.get(Stream::Ai), keys.get(Stream::Combat));
    let n = 100_000u64;
    assert_independent(
        (0..n).map(|i| (draw(ai, sid(i), 4, 0), draw(combat, sid(i), 4, 0))),
        "flux",
    );
    assert_independent(
        (0..n).map(|i| (draw(ai, sid(i), 4, 0), draw(ai, sid(i), 5, 0))),
        "créneaux voisins",
    );
    assert_independent(
        (0..n).map(|i| (draw(ai, sid(3), i, 0), draw(ai, sid(3), i + 1, 0))),
        "créneaux successifs",
    );
    assert_independent(
        (0..n).map(|i| (draw(ai, sid(i), 0, 0), draw(ai, sid(i), 0, 1))),
        "usages",
    );
    assert_independent(
        (0..n).map(|i| (draw(ai, sid(i), 0, 0), draw(ai, sid(i | 1 << 63), 0, 0))),
        "espace du jeu et de la carte",
    );
    assert_independent(
        (0..n).map(|i| {
            let (b0, b1) = (
                draw_block_at(ai, sid(i), 0, 0, 0),
                draw_block_at(ai, sid(i), 0, 0, 1),
            );
            (b0[7], b1[0])
        }),
        "blocs successifs",
    );
}

/// M27 : sur 10⁵ graines cosmétiques, aucune n'est égale à un mot d'une clé (monde, flux,
/// racine cosmétique, sous-clé HChaCha) ni à une sortie de l'aléa.
#[test]
fn m27_cosmetic_values_are_never_key_or_rng_state() {
    let world = WorldKey::from_bytes(*b"time_switcher_t5");
    let keys = StreamKeys::derive(&world);
    let mut secret: BTreeSet<u64> = words(&world.0).collect();
    secret.extend(words(&keys.cosmetic.0));
    let n = 100_000u64;
    for s in Stream::ALL {
        let key = keys.get(s);
        secret.extend(words(&key.0));
        for i in 0..n / 8 {
            let nonce = nonce(sid(i), 0, 0);
            let sub =
                chacha20::hchacha::<chacha20::R8>(&key.0.into(), &nonce[..16].try_into().unwrap());
            secret.extend(words(&sub));
            secret.extend(draw_block(key, sid(i), 0, 0));
        }
    }
    let mut cosmetic = BTreeSet::new();
    for i in 0..n {
        for usage in ["hair", "voice"] {
            let seed = keys.cosmetic_seed(usage, sid(i), 0).value();
            assert!(
                !secret.contains(&seed),
                "graine cosmétique {i} ({usage}) = un secret"
            );
            cosmetic.insert(seed);
        }
    }
    assert_eq!(
        cosmetic.len() as u64,
        2 * n,
        "graines cosmétiques en double"
    );
    assert!(
        secret.len() > 1_000_000,
        "échantillon de secrets trop petit"
    );
}

#[test]
fn cosmetic_seed_depends_on_every_input() {
    let keys = keys();
    let base = keys.cosmetic_seed("hair", sid(1), 2);
    assert_eq!(base, keys.cosmetic_seed("hair", sid(1), 2));
    assert_ne!(base, keys.cosmetic_seed("hai", sid(1), 2));
    assert_ne!(base, keys.cosmetic_seed("hair", sid(2), 2));
    assert_ne!(base, keys.cosmetic_seed("hair", sid(1), 3));
    let other = StreamKeys::derive(&WorldKey::from_bytes([1; 16]));
    assert_ne!(base, other.cosmetic_seed("hair", sid(1), 2));
}

/// R5 : coût d'un bloc, HChaCha compris (chaque appel a son propre *nonce*). Le cœur interdit de
/// lire l'horloge (`clippy.toml`), tests compris : la durée se mesure de l'extérieur, en lançant
/// le binaire de test avec et sans ce test (`--ignored --exact rng::tests::block_cost_r5`),
/// puis en divisant l'écart par le nombre de blocs.
#[test]
#[ignore = "mesure R5, chronométrée de l'extérieur"]
fn block_cost_r5() {
    let keys = keys();
    let key = keys.get(Stream::Ai);
    let mut acc = 0u64;
    for i in 0..BLOCK_COST_ITERATIONS {
        acc ^= draw_block(key, sid(i), i, 0)[7];
    }
    assert_ne!(std::hint::black_box(acc), 0);
}

/// Nombre de blocs de la mesure R5.
const BLOCK_COST_ITERATIONS: u64 = 10_000_000;
