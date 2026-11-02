//! Bridge with the kernel: a real snapshot round trip, the F4 round trip through the bytes,
//! the world key never encoded, the fuzzing seeds, and the compression ratio on a town of
//! 10,000 inhabitants.

#![cfg(test)]

mod common;

use bevy_ecs::prelude::{Component, Query, Res};
use common::{forge, stored};
use ts_kernel::rng::{DrawSite, Stream, WorldKey};

const WALK: DrawSite = DrawSite::new(Stream::Population, 9, "test.walk");
const SETTLE: DrawSite = DrawSite::new(Stream::Population, 0, "test.settle");
use ts_kernel::{
    CanonicalError, CanonicalLimits, MemorySnapshot, Money, Phase, Plugin, Sim, SimBuilder,
    SimConfig, SnapshotScope, StableEntity, StableId, StableIds, StateDecode, StateHash,
    StateLayout, StateReader, StateWriter, WorldStreams,
};
use ts_persist::{
    Codec, DecodeError, Limits, META_PART, STATE_PART, decode_snapshot, encode_parts,
    encode_snapshot, fuzzing,
};

/// Position in centimetres, in a town of 10 km.
#[derive(Component, Clone)]
struct Pos(i64, i64);

impl StateHash for Pos {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.0);
        w.i64(self.1);
    }
}

impl StateDecode for Pos {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Ok(Self(r.i64()?, r.i64()?))
    }
}

/// Household, money, age in days, hunger.
#[derive(Component, Clone)]
struct Life {
    home: StableId,
    purse: Money,
    age_days: u32,
    hunger: f64,
}

impl StateHash for Life {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        self.home.hash_state(w);
        self.purse.hash_state(w);
        w.u32(self.age_days);
        w.f64(self.hunger);
    }
}

impl StateDecode for Life {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Ok(Self {
            home: StableId::decode_state(r)?,
            purse: Money::decode_state(r)?,
            age_days: r.u32()?,
            hunger: r.f64()?,
        })
    }
}

fn walk(streams: Res<WorldStreams>, mut q: Query<(&StableEntity, &mut Pos, &mut Life)>) {
    for (s, mut p, mut l) in &mut q {
        let r = streams.draw(WALK, s.id(), 0);
        p.0 += i64::try_from(r % 201).unwrap() - 100;
        l.hunger = (l.hunger + 0.01).min(1.0);
    }
}

/// A town: `n` inhabitants with plausible values drawn from the world streams.
struct Town(u64);

impl Plugin for Town {
    fn name(&self) -> &'static str {
        "town"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Pos>("town")
            .state_component::<Life>("town");
        let world = b.world_mut();
        let first = StableId::from_snapshot_raw(0);
        for i in 0..self.0 {
            let id = world.resource_mut::<StableIds>().allocate().unwrap();
            let streams = world.resource::<WorldStreams>();
            let d = |slot| streams.draw(SETTLE, id, slot);
            let pos = Pos(
                i64::try_from(d(1) % 1_000_000).unwrap(),
                i64::try_from(d(2) % 1_000_000).unwrap(),
            );
            let life = Life {
                home: if i < 4 {
                    first
                } else {
                    StableId::from_snapshot_raw(i - i % 4)
                },
                purse: Money::from_cents(i64::try_from(d(3) % 500_000).unwrap()),
                age_days: u32::try_from(d(4) % 30_000).unwrap(),
                hunger: f64::from(u32::try_from(d(5) % 100).unwrap()) / 100.0,
            };
            world.spawn((StableEntity::new(id), pos, life));
        }
        b.add_systems(Phase::Act, walk);
    }
}

/// A key with no long run of zeros or repeats: any copy of it in the bytes would show.
const KEY: [u8; 16] = *b"\x9e\x37\x79\xb9\x7f\x4a\x7c\x15\xf3\x9c\xc0\x60\x5c\xed\xc8\x34";

fn town_rules(n: u64) -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Town(n))]
}

fn town_sim(n: u64) -> Sim {
    Sim::build(
        SimConfig::default(),
        &town_rules(n),
        &WorldKey::from_bytes(KEY),
    )
    .unwrap()
}

fn town(n: u64) -> MemorySnapshot {
    let mut sim = town_sim(n);
    for _ in 0..3 {
        sim.step();
    }
    sim.snapshot(SnapshotScope::Full)
}

fn town_layout(n: u64) -> StateLayout {
    StateLayout::of(&SimConfig::default(), &town_rules(n)).unwrap()
}

#[test]
fn real_snapshot_round_trip() {
    let snap = town(200);
    let layout = town_layout(200);
    for codec in [Codec::Stored, Codec::Zstd] {
        let bytes = encode_snapshot(&snap, codec, &Limits::default()).unwrap();
        let decoded = decode_snapshot(&bytes, &Limits::default(), &layout).unwrap();
        assert_eq!(decoded.header_bytes(), snap.header_bytes());
        assert_eq!(decoded.canonical_bytes(), snap.canonical_bytes());
        assert_eq!(decoded.rules_fingerprint(), snap.rules_fingerprint());
        assert_eq!(decoded.now(), snap.now());
        assert_eq!(
            encode_snapshot(&decoded, codec, &Limits::default()).unwrap(),
            bytes
        );
    }
}

/// F4 (AS-36 condition 4, AS-27): snapshot of a real world at tick k, encoded and decoded
/// by `ts_persist`, restored with the key: the same state hash right after the restore and
/// after N more ticks as the uninterrupted run, for several k, k > 0 included.
#[test]
fn f4_round_trip_through_the_bytes() {
    const N: usize = 6;
    let rules = town_rules(50);
    let layout = town_layout(50);
    for k in [0, 1, 4, 9] {
        for codec in [Codec::Stored, Codec::Zstd] {
            let mut straight = town_sim(50);
            for _ in 0..k {
                straight.step();
            }
            let snap: MemorySnapshot = straight.snapshot(SnapshotScope::Full);
            let bytes = encode_snapshot(&snap, codec, &Limits::default()).unwrap();
            let decoded = decode_snapshot(&bytes, &Limits::default(), &layout).unwrap();
            let mut restored = Sim::restore(
                &decoded,
                SimConfig::default(),
                &rules,
                &WorldKey::from_bytes(KEY),
            )
            .unwrap();
            assert_eq!(restored.state_hash(), straight.state_hash(), "k = {k}");
            for _ in 0..N {
                straight.step();
                restored.step();
            }
            assert_eq!(
                restored.state_hash(),
                straight.state_hash(),
                "k = {k} + {N}"
            );
        }
    }
}

/// L4, Q4: the encoded snapshot never contains the world key.
#[test]
fn world_key_never_encoded() {
    let snap = town(200);
    for codec in [Codec::Stored, Codec::Zstd] {
        let bytes = encode_snapshot(&snap, codec, &Limits::default()).unwrap();
        assert!(!bytes.windows(KEY.len()).any(|w| w == KEY), "{codec:?}");
        // Neither half of it either.
        for half in KEY.chunks(8) {
            assert!(!bytes.windows(8).any(|w| w == half), "{codec:?}");
        }
    }
}

#[test]
fn snapshot_needs_exactly_the_kernel_parts() {
    let d = Limits::default();
    let layout = town_layout(3);
    let snap = town(3);
    let (meta, state) = (snap.header_bytes(), snap.canonical_bytes().to_vec());
    let only_meta = encode_parts(&[(META_PART, &meta)], Codec::Stored, &d).unwrap();
    assert_eq!(
        decode_snapshot(&only_meta, &d, &layout).unwrap_err(),
        DecodeError::MissingPart(STATE_PART.to_owned())
    );
    let extra = encode_parts(
        &[
            (META_PART, &meta),
            (STATE_PART, &state),
            ("kernel.zzz", b""),
        ],
        Codec::Stored,
        &d,
    )
    .unwrap();
    assert_eq!(
        decode_snapshot(&extra, &d, &layout).unwrap_err(),
        DecodeError::UnexpectedPart("kernel.zzz".to_owned())
    );
    let mut long = meta.clone();
    long.push(0);
    let env = forge(
        vec![stored(META_PART, &long), stored(STATE_PART, &state)],
        &[&long[..], &state].concat(),
    );
    assert_eq!(
        decode_snapshot(&env, &d, &layout).unwrap_err(),
        DecodeError::Kernel(CanonicalError::TrailingBytes {
            at: meta.len() as u64
        })
    );
    let short = &meta[..meta.len() - 1];
    let env = forge(
        vec![stored(META_PART, short), stored(STATE_PART, &state)],
        &[short, &state].concat(),
    );
    assert!(matches!(
        decode_snapshot(&env, &d, &layout),
        Err(DecodeError::Kernel(CanonicalError::Truncated { .. }))
    ));
}

/// The date is written twice, in `kernel.meta` and in the state: they must agree.
#[test]
fn meta_date_must_match_the_state() {
    let d = Limits::default();
    let mut sim = town_sim(3);
    let before: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    sim.step();
    let after: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    let env = encode_parts(
        &[
            (META_PART, &before.header_bytes()),
            (STATE_PART, after.canonical_bytes()),
        ],
        Codec::Stored,
        &d,
    )
    .unwrap();
    assert_eq!(
        decode_snapshot(&env, &d, &town_layout(3)).unwrap_err(),
        DecodeError::Kernel(CanonicalError::DateMismatch {
            header: 0,
            state: 50
        })
    );
}

/// The kernel decoder is bounded by the codec's part bound.
#[test]
fn kernel_decoder_takes_the_codec_bound() {
    let snap = town(3);
    let len = snap.canonical_bytes().len() as u64;
    let bytes = encode_snapshot(&snap, Codec::Stored, &Limits::default()).unwrap();
    let limits = Limits {
        max_part_len: len,
        ..Limits::default()
    };
    decode_snapshot(&bytes, &limits, &town_layout(3)).unwrap();
    let from_kernel = MemorySnapshot::from_canonical(
        &snap.header_bytes(),
        snap.canonical_bytes(),
        &town_layout(3),
        &CanonicalLimits { max_len: len - 1 },
    );
    assert_eq!(
        from_kernel.unwrap_err(),
        CanonicalError::TooLarge { len, max: len - 1 }
    );
}

/// The initial corpus: every envelope seed decodes, snapshots included, and every direct
/// seed is read by the kernel decoder.
#[test]
fn seeds_decode() {
    let rules = fuzzing::rules();
    let mut sim = Sim::build(SimConfig::default(), &rules, &WorldKey::from_bytes(KEY)).unwrap();
    let first: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    for _ in 0..30 {
        sim.step();
    }
    let later: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    let seeds = fuzzing::seeds(&[first, later]);
    assert_eq!(seeds.len(), 10);
    let layout = StateLayout::of(&SimConfig::default(), &rules).unwrap();
    // The fuzz body reaches the kernel decoder only if its layout builds.
    assert!(fuzzing::kernel_layout().is_some());
    let limits = Limits::fuzzing();
    for s in &seeds[..8] {
        fuzzing::decode(s).unwrap();
    }
    for i in [2, 3, 6, 7] {
        decode_snapshot(&seeds[i], &limits, &layout).unwrap();
    }
    for s in &seeds[8..] {
        let (len, rest) = s.split_at(2);
        let (h, st) = rest.split_at(usize::from(u16::from_le_bytes([len[0], len[1]])));
        let snap = MemorySnapshot::from_canonical(
            h,
            st,
            &layout,
            &CanonicalLimits {
                max_len: limits.max_part_len,
            },
        )
        .unwrap();
        Sim::restore(
            &snap,
            SimConfig::default(),
            &rules,
            &WorldKey::from_bytes(KEY),
        )
        .unwrap();
        assert!(fuzzing::decode(s).is_err());
    }
    assert!(fuzzing::decode(b"").is_err());
}

/// Compression on a town of 10,000 inhabitants (architecture §12.3 budget: 250 to 400 MB in
/// memory, 60 to 120 MB compressed, a ratio of about 2 to 6.7).
#[test]
fn compression_ratio_on_ten_thousand_inhabitants() {
    let snap = town(10_000);
    let stored = encode_snapshot(&snap, Codec::Stored, &Limits::default()).unwrap();
    let zstd = encode_snapshot(&snap, Codec::Zstd, &Limits::default()).unwrap();
    let ratio = stored.len() as f64 / zstd.len() as f64;
    assert!(
        ratio > 1.5,
        "stored {} B, zstd {} B, ratio {ratio:.2}",
        stored.len(),
        zstd.len()
    );
}
