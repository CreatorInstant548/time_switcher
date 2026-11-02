//! Body of the fuzzing target `fuzz/fuzz_targets/decode.rs` and its initial corpus, kept in
//! the library so that they compile and run on stable (A14: no local nightly). Development
//! only: absent from the `public` profile (see the crate root).

use std::sync::OnceLock;

use bevy_ecs::prelude::{Component, Resource};
use ts_kernel::{
    CanonicalError, EventQueue, LodComplete, MemorySnapshot, Money, Plugin, SimBuilder, SimConfig,
    SimTime, StableEntity, StableId, StableIds, StateDecode, StateHash, StateLayout, StateReader,
    StateWriter, TimedEvent, bevy_ecs,
};

use crate::codec::{Parts, decode_parts, encode_parts};
use crate::error::DecodeError;
use crate::kernel::{canonical_limits, encode_snapshot, snapshot_from_parts};
use crate::limits::Limits;
use crate::manifest::Codec;

/// State component of the fuzzing rules: one of each kind of value the kernel decodes.
#[derive(Component, Clone)]
struct Cell {
    at: SimTime,
    owner: Option<StableId>,
    tags: Vec<u8>,
    lit: bool,
    heat: f32,
}

impl StateHash for Cell {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        self.at.hash_state(w);
        self.owner.hash_state(w);
        self.tags.hash_state(w);
        self.lit.hash_state(w);
        self.heat.hash_state(w);
    }
}

impl StateDecode for Cell {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Ok(Self {
            at: SimTime::decode_state(r)?,
            owner: Option::decode_state(r)?,
            tags: Vec::decode_state(r)?,
            lit: bool::decode_state(r)?,
            heat: f32::decode_state(r)?,
        })
    }
}

/// State resource of the fuzzing rules.
#[derive(Resource, Clone)]
struct Ledger(Vec<Money>);

impl StateHash for Ledger {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        self.0.hash_state(w);
    }
}

impl StateDecode for Ledger {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Vec::decode_state(r).map(Self)
    }
}

/// Rules of the fuzzing target: a family before `kernel` and one after it, three stable
/// entities (one in full simulation), two pending events.
struct FuzzRules;

impl Plugin for FuzzRules {
    fn name(&self) -> &'static str {
        "fuzz"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Cell>("cells")
            .state_resource::<Ledger>("ledger")
            .insert_resource(Ledger(vec![Money::from_cents(-5), Money::from_cents(120)]));
        let world = b.world_mut();
        for (i, (ms, heat)) in [(0_u8, (0, 0.0)), (1, (50, 0.5)), (2, (100, -1.5))] {
            let Some(id) = world.resource_mut::<StableIds>().allocate() else {
                return;
            };
            let cell = Cell {
                at: SimTime::from_millis(ms),
                owner: (i > 0).then_some(id),
                tags: vec![i; usize::from(i)],
                lit: i == 1,
                heat,
            };
            let mut e = world.spawn((StableEntity::new(id), cell));
            if i == 2 {
                e.insert(LodComplete);
            }
            let event = TimedEvent {
                kind: u32::from(i),
                subject: id,
                arg: u64::from(i),
            };
            let scheduled = world
                .resource_mut::<EventQueue>()
                .schedule(SimTime::from_millis(1000), event);
            debug_assert!(scheduled.is_ok(), "a fixed date within the saved range");
        }
    }
}

/// Rules of the fuzzing target, for `fuzz/seeds` and the tests.
pub fn rules() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(FuzzRules)]
}

/// Layout of [`rules`], built once. `None` would make [`decode`] skip the kernel decoder:
/// the tests check that it is built.
pub fn kernel_layout() -> Option<&'static StateLayout> {
    static LAYOUT: OnceLock<Option<StateLayout>> = OnceLock::new();
    LAYOUT
        .get_or_init(|| StateLayout::of(&SimConfig::default(), &rules()).ok())
        .as_ref()
}

/// Direct input of the kernel decoder: header length (`u16` LE), header, state.
fn split_direct(data: &[u8]) -> Option<(&[u8], &[u8])> {
    let (len, rest) = data.split_first_chunk::<2>()?;
    rest.split_at_checked(usize::from(u16::from_le_bytes(*len)))
}

// spec: carnet architecture-securite.md SEC0-24 (révisé), §10
/// Runs every decoder on `data` with [`Limits::fuzzing`], without `restore`:
/// - the whole chain, envelope, manifest, parts, then `MemorySnapshot::from_canonical` on
///   the kernel parts (as `decode_snapshot` does);
/// - `from_canonical` directly on `data` read as a header length (`u16` LE), a header and a
///   state: the digests of the envelope stop almost every mutation before the kernel
///   decoder, so this way the fuzzer reaches it.
///
/// It must never panic, whatever `data` is.
///
/// # Errors
///
/// The error of the parts decoder, if any.
pub fn decode(data: &[u8]) -> Result<Parts, DecodeError> {
    let limits = Limits::fuzzing();
    let layout = kernel_layout();
    if let (Some(layout), Some((header, state))) = (layout, split_direct(data)) {
        let _ = MemorySnapshot::from_canonical(header, state, layout, &canonical_limits(&limits));
    }
    let parts = decode_parts(data, &limits)?;
    if let Some(layout) = layout {
        let _ = snapshot_from_parts(parts.clone(), &limits, layout);
    }
    Ok(parts)
}

// spec: carnet architecture-securite.md SEC0-24 (révisé), §10
/// Initial corpus of the target: valid envelopes, stored and compressed, including one per
/// snapshot of `snapshots` (taken from a world of [`rules`]), and each snapshot as a direct
/// input of the kernel decoder. Written by `fuzz/seeds` into the corpus directory
/// (regenerable, never committed).
pub fn seeds(snapshots: &[MemorySnapshot]) -> Vec<Vec<u8>> {
    let limits = Limits::fuzzing();
    let few: [(&str, &[u8]); 3] = [("a", b""), ("b/c", b"seed"), ("pop.solo/0001", &[7; 300])];
    let mut out = Vec::new();
    for codec in [Codec::Stored, Codec::Zstd] {
        out.extend(encode_parts(&[], codec, &limits).ok());
        out.extend(encode_parts(&few, codec, &limits).ok());
        for snap in snapshots {
            out.extend(encode_snapshot(snap, codec, &limits).ok());
        }
    }
    for snap in snapshots {
        let header = snap.header_bytes();
        if let Ok(len) = u16::try_from(header.len()) {
            out.push([&len.to_le_bytes(), &header[..], snap.canonical_bytes()].concat());
        }
    }
    out
}
