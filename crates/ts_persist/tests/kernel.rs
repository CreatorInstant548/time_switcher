//! Bridge with the kernel: a real snapshot round trip, the world key never encoded, and the
//! compression ratio on a town of 10,000 inhabitants.

#![cfg(test)]

mod common;

use bevy_ecs::prelude::{Component, Query, Res};
use common::{forge, stored};
use ts_kernel::rng::{Stream, WorldKey, draw};
use ts_kernel::{
    MemorySnapshot, Money, Phase, Plugin, Sim, SimBuilder, SimConfig, SnapshotScope, StableEntity,
    StableId, StableIds, StateHash, StateWriter, WorldStreams,
};
use ts_persist::{
    Codec, DecodeError, Limits, META_PART, STATE_PART, SnapshotImage, decode_snapshot,
    encode_parts, encode_snapshot,
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

fn walk(streams: Res<WorldStreams>, mut q: Query<(&StableEntity, &mut Pos, &mut Life)>) {
    for (s, mut p, mut l) in &mut q {
        let r = draw(streams.get(Stream::Population), s.id(), 0, 9);
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
            let d = |slot| draw(streams.get(Stream::Population), id, slot, 0);
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

fn town(n: u64) -> MemorySnapshot {
    let rules: Vec<Box<dyn Plugin>> = vec![Box::new(Town(n))];
    let mut sim = Sim::build(SimConfig::default(), &rules, &WorldKey::from_bytes(KEY)).unwrap();
    for _ in 0..3 {
        sim.step();
    }
    sim.snapshot(SnapshotScope::Full)
}

#[test]
fn real_snapshot_round_trip() {
    let snap = town(200);
    for codec in [Codec::Stored, Codec::Zstd] {
        let bytes = encode_snapshot(&snap, codec, &Limits::default()).unwrap();
        let image = decode_snapshot(&bytes, &Limits::default()).unwrap();
        assert_eq!(image, SnapshotImage::of(&snap));
        assert_eq!(image.state, snap.canonical_bytes());
        assert_eq!(image.rules, snap.rules_fingerprint());
        assert_eq!(image.now, snap.now());
        assert_eq!(image.encode(codec, &Limits::default()).unwrap(), bytes);
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
    let meta = postcard::to_allocvec(&([0_u8; 32], 0_i64)).unwrap();
    let only_meta = encode_parts(&[(META_PART, &meta)], Codec::Stored, &d).unwrap();
    assert_eq!(
        decode_snapshot(&only_meta, &d),
        Err(DecodeError::MissingPart(STATE_PART.to_owned()))
    );
    let extra = encode_parts(
        &[(META_PART, &meta), (STATE_PART, b"s"), ("kernel.zzz", b"")],
        Codec::Stored,
        &d,
    )
    .unwrap();
    assert_eq!(
        decode_snapshot(&extra, &d),
        Err(DecodeError::UnexpectedPart("kernel.zzz".to_owned()))
    );
    let mut long = meta.clone();
    long.push(0);
    let env = forge(
        vec![stored(META_PART, &long), stored(STATE_PART, b"s")],
        &[&long[..], b"s"].concat(),
    );
    assert_eq!(
        decode_snapshot(&env, &d),
        Err(DecodeError::TrailingBytes("snapshot meta"))
    );
    let short = &meta[..meta.len() - 1];
    let env = forge(
        vec![stored(META_PART, short), stored(STATE_PART, b"s")],
        &[short, b"s"].concat(),
    );
    assert_eq!(
        decode_snapshot(&env, &d),
        Err(DecodeError::Malformed("snapshot meta"))
    );
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
