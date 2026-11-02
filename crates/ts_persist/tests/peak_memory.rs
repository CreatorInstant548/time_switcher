// spec: attendus-mesurables.md §10.3 (M41h)
//! Memory peak of decoding, measured: the allocator of this test binary counts every byte
//! requested and refuses any allocation above a cap. Each decoding runs capped at
//! `Limits::decode_peak_bound` minus its input: going above it aborts the binary, so the bound
//! is guaranteed on these inputs, not only computed. One test function only, so no other test
//! allocates while a cap is set.

#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::print_stdout)]

mod common;

use std::alloc::System;

use bevy_ecs::prelude::Component;
use cap::Cap;
use common::{PART_CONTEXT, digest, forge, stored};
use ts_kernel::rng::WorldKey;
use ts_kernel::{
    CanonicalError, MemorySnapshot, Plugin, Sim, SimBuilder, SimConfig, SnapshotScope,
    StableEntity, StableIds, StateDecode, StateHash, StateLayout, StateReader, StateWriter,
};
use ts_persist::{
    Bound, Codec, DecodeError, EncodeError, Limits, MAX_DECODE_PEAK, PartEntry, decode_parts,
    decode_snapshot, encode_parts, encode_snapshot,
};

#[global_allocator]
static ALLOC: Cap<System> = Cap::new(System, usize::MAX);

const MIB: u64 = 1 << 20;

/// Bounds of the measure: large enough that the decoder's buffers dominate, small enough
/// for a test.
const LIMITS: Limits = Limits {
    max_envelope_len: 8 * MIB,
    max_manifest_len: 1 << 20,
    max_parts: 1024,
    max_name_len: 64,
    max_part_len: 24 * MIB,
    max_total_len: 48 * MIB,
    max_window: MIB,
};

/// Runs `f` with at most `budget` bytes allocated on top of what is allocated now; returns
/// its result and the bytes it still holds afterwards.
fn capped<T>(budget: u64, f: impl FnOnce() -> T) -> (T, usize) {
    let base = ALLOC.allocated();
    let cap = base.saturating_add(usize::try_from(budget).unwrap());
    ALLOC.set_limit(cap).unwrap();
    let out = f();
    let held = ALLOC.allocated().saturating_sub(base);
    ALLOC.set_limit(usize::MAX).unwrap();
    (out, held)
}

/// Decodes `env` capped at the bound of `limits` minus the whole envelope allowance: the
/// input is already allocated, and the unused part of its allowance is not lent to the other
/// terms of the bound.
fn decode_within_bound(env: &[u8], limits: &Limits) {
    assert!(env.len() as u64 <= limits.max_envelope_len);
    let budget = limits.decode_peak_bound() - limits.max_envelope_len;
    let (parts, _) = capped(budget, || decode_parts(env, limits));
    parts.unwrap();
}

/// Bytes that `zstd` compresses well but not to nothing: a period of 251 over a slow ramp.
fn compressible(len: u64, salt: u8) -> Vec<u8> {
    (0..len)
        .map(|i| ((i / 4096) as u8) ^ ((i % 251) as u8) ^ salt)
        .collect()
}

/// A `zstd` frame announcing a window of exactly `window` bytes (a power of two, 1 KiB to
/// 2 GiB), then `data` in raw blocks of 128 KiB: the decoder keeps a whole window, which our
/// encoder (128 KiB window) never asks for.
fn frame_with_window(window: u64, data: &[u8]) -> Vec<u8> {
    let log = window.trailing_zeros();
    assert!(window.is_power_of_two() && (10..=31).contains(&log));
    // Magic, frame header descriptor (no single segment, no size, no checksum), window
    // descriptor: exponent `log - 10`, mantissa 0.
    let mut out = vec![0x28, 0xb5, 0x2f, 0xfd, 0x00, ((log - 10) as u8) << 3];
    let chunks: Vec<&[u8]> = data.chunks(128 << 10).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        // Block header: last-block bit, type 0 (raw), size.
        let last = u32::from(i + 1 == chunks.len());
        let header = last | ((chunk.len() as u32) << 3);
        out.extend_from_slice(&header.to_le_bytes()[..3]);
        out.extend_from_slice(chunk);
    }
    out
}

/// A state component of one byte: the most entities per byte of canonical state.
#[derive(Component, Clone)]
struct Flag(bool);

impl StateHash for Flag {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        self.0.hash_state(w);
    }
}

impl StateDecode for Flag {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        bool::decode_state(r).map(Self)
    }
}

struct Flags(u64);

impl Plugin for Flags {
    fn name(&self) -> &'static str {
        "flags"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Flag>("flags");
        let world = b.world_mut();
        for i in 0..self.0 {
            let id = world.resource_mut::<StableIds>().allocate().unwrap();
            world.spawn((StableEntity::new(id), Flag(i % 2 == 0)));
        }
    }
}

#[test]
fn decoding_stays_under_its_peak_bound() {
    // M41h: the defaults stay under the 2 GiB ceiling (also a compile-time check).
    let default = Limits::default().decode_peak_bound();
    println!("decode peak bound of the default limits: {default} bytes");
    assert!(default <= MAX_DECODE_PEAK);

    // Bounds that allow more than the ceiling are refused, by the decoder and the encoder.
    let loose = Limits {
        max_total_len: 4 << 30,
        ..Limits::default()
    };
    let env = encode_parts(&[("a", b"x")], Codec::Stored, &Limits::default()).unwrap();
    let refused = DecodeError::TooLarge {
        bound: Bound::Peak,
        len: loose.decode_peak_bound(),
        max: MAX_DECODE_PEAK,
    };
    assert_eq!(decode_parts(&env, &loose), Err(refused.clone()));
    assert_eq!(
        encode_parts(&[("a", b"x")], Codec::Stored, &loose),
        Err(EncodeError::Invalid(refused))
    );

    // Two compressed parts at the part bound, filling the total bound: the decompression
    // buffers dominate.
    let (a, b) = (compressible(24 * MIB, 0), compressible(24 * MIB, 0x5a));
    let env = encode_parts(&[("a", &a), ("b", &b)], Codec::Zstd, &LIMITS).unwrap();
    drop((a, b));
    decode_within_bound(&env, &LIMITS);

    // Stored parts filling the envelope: the input and its copy.
    let raw = compressible(7 * MIB, 1);
    let env = forge(vec![stored("raw", &raw)], &raw);
    drop(raw);
    decode_within_bound(&env, &LIMITS);

    // A frame announcing the largest window allowed: the decoder's window buffer.
    let data = compressible(4 * MIB, 2);
    let frame = frame_with_window(LIMITS.max_window, &data);
    let entry = PartEntry {
        name: "w".to_owned(),
        codec: Codec::Zstd,
        stored_len: frame.len() as u64,
        stored_digest: digest(PART_CONTEXT, &frame),
        len: data.len() as u64,
        digest: digest(PART_CONTEXT, &data),
    };
    let env = forge(vec![entry], &frame);
    drop((data, frame));
    decode_within_bound(&env, &LIMITS);

    // As many parts as allowed, with the longest names: the manifest and the vectors.
    let names: Vec<String> = (0..LIMITS.max_parts)
        .map(|i| format!("{i:04}{}", "n".repeat(60)))
        .collect();
    let parts: Vec<(&str, &[u8])> = names.iter().map(|n| (n.as_str(), &b"p"[..])).collect();
    let env = encode_parts(&parts, Codec::Zstd, &LIMITS).unwrap();
    decode_within_bound(&env, &LIMITS);

    // The kernel stage, after the parts: memory held by the decoded state, per byte of
    // canonical state, with the state type that packs the most entities per byte. A measure
    // only, not part of the bound: it depends on the state types of the rules (design #125).
    let rules: Vec<Box<dyn Plugin>> = vec![Box::new(Flags(100_000))];
    let key = WorldKey::from_bytes(*b"peak memory key!");
    let snap: MemorySnapshot = Sim::build(SimConfig::default(), &rules, &key)
        .unwrap()
        .snapshot(SnapshotScope::Full);
    let layout = StateLayout::of(&SimConfig::default(), &rules).unwrap();
    let state = snap.canonical_bytes().len() as u64;
    let env = encode_snapshot(&snap, Codec::Zstd, &Limits::default()).unwrap();
    drop(snap);
    let base = ALLOC.allocated();
    let decoded = decode_snapshot(&env, &Limits::default(), &layout).unwrap();
    let held = ALLOC.allocated().saturating_sub(base);
    drop(decoded);
    println!(
        "kernel stage: {state} bytes of canonical state held as {held} bytes once decoded \
         (x{:.2})",
        held as f64 / state as f64
    );
}
