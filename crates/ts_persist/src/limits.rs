//! Bounds of decoding (SEC0-23): each host can pass its own, the defaults are cautious.

// spec: carnet architecture-securite.md SEC0-23 ; revues/securite.md S10, S24 ; revues/architecture.md §12.3 ; attendus-mesurables.md §10.3
/// Bounds checked while decoding, and while encoding so that the encoder never writes what
/// the decoder refuses. The depth needs no runtime bound: the schema has no recursive type
/// ([`crate::Schema`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// Whole envelope, in stored bytes.
    pub max_envelope_len: u64,
    /// Manifest bytes.
    pub max_manifest_len: u32,
    /// Number of parts.
    pub max_parts: u32,
    /// Part name, in bytes.
    pub max_name_len: u32,
    /// Decoded size of one part.
    pub max_part_len: u64,
    /// Decoded size of all parts together: decompression stops there (no decompression
    /// bomb).
    pub max_total_len: u64,
    /// Largest `zstd` window a frame may announce: it bounds the decoder memory.
    pub max_window: u64,
}

const MIB: u64 = 1 << 20;

// spec: attendus-mesurables.md §10.3 (M41h)
// PROVISIONAL(design #125): the ceiling covers the parts stage only (`decode_parts`); the
// state the kernel's canonical decoder builds afterwards is bounded by `max_part_len` in
// bytes only, its memory depends on the state types of the rules.
/// Ceiling of the decoding memory peak, 2 GiB: the starting value of M41h. A decoding (and
/// an encoding, which checks its output with the decoder) refuses bounds whose
/// [`Limits::decode_peak_bound`] is above it.
pub const MAX_DECODE_PEAK: u64 = 2 << 30;

/// Memory of the `ruzstd` decoder besides its window buffer: tables, block buffers and
/// scratch, measured below 1 MiB (test `peak_memory`).
const ZSTD_SCRATCH: u64 = MIB;

impl Default for Limits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// spec: attendus-mesurables.md §10.3 (M41h)
const _: () = assert!(
    Limits::DEFAULT.decode_peak_bound() <= MAX_DECODE_PEAK,
    "the default bounds must keep the decoding peak under M41h"
);

impl Limits {
    /// [`Limits::default`]: the starting values of A15, the same in solo and online; a host
    /// may pass stricter bounds. They are recalibrated in phase 1 from the measured sizes.
    const DEFAULT: Self = Self {
        // spec: attendus-mesurables.md §10.3 (M41a)
        max_envelope_len: 256 * MIB,
        // spec: attendus-mesurables.md §10.3 (M41d)
        max_manifest_len: 4 << 20,
        // spec: attendus-mesurables.md §10.3 (M41e)
        max_parts: 65_536,
        // spec: attendus-mesurables.md §10.3 (M41f)
        max_name_len: 128,
        // spec: attendus-mesurables.md §10.3 (M41c)
        max_part_len: 512 * MIB,
        // spec: attendus-mesurables.md §10.3 (M41b)
        max_total_len: 1024 * MIB,
        // spec: attendus-mesurables.md §10.3 (M41g)
        max_window: 8 * MIB,
    };

    // spec: attendus-mesurables.md §10.3 (M41h)
    /// Upper bound, in bytes requested from the allocator, of the memory that decoding an
    /// envelope with [`crate::decode_parts`] holds at its peak, the input included:
    /// - the input envelope, held by the caller ([`Limits::max_envelope_len`]);
    /// - every decoded part, all held at once ([`Limits::max_total_len`]);
    /// - while a part grows, its previous buffer, at most one part
    ///   ([`Limits::max_part_len`]);
    /// - the `zstd` decoder: its window buffer, twice while it grows, plus its scratch;
    /// - per part, the manifest entry and the decoder's vectors, each up to three times its
    ///   size while it grows, and the name.
    ///
    /// The test `peak_memory` measures the actual peak against this bound. The canonical
    /// decoding of the kernel parts by [`crate::decode_snapshot`] comes after it and is not
    /// included: it depends on the state types of the rules (see that test).
    pub const fn decode_peak_bound(&self) -> u64 {
        const fn size(n: usize) -> u64 {
            n as u64
        }
        let per_part = 3_u64
            .saturating_mul(
                size(size_of::<crate::PartEntry>())
                    .saturating_add(size(size_of::<&[u8]>()))
                    .saturating_add(size(size_of::<(String, Vec<u8>)>())),
            )
            .saturating_add(self.max_name_len as u64);
        self.max_envelope_len
            .saturating_add(self.max_total_len)
            .saturating_add(self.max_part_len)
            .saturating_add(self.max_window.saturating_mul(2))
            .saturating_add(ZSTD_SCRATCH)
            .saturating_add((self.max_parts as u64).saturating_mul(per_part))
    }

    /// Small bounds for the fuzzing target: a decoding never allocates more than a few
    /// mebibytes, so the fuzzer finds crashes rather than memory exhaustion.
    pub const fn fuzzing() -> Self {
        Self {
            max_envelope_len: MIB,
            max_manifest_len: 64 << 10,
            max_parts: 256,
            max_name_len: 64,
            max_part_len: 4 * MIB,
            max_total_len: 16 * MIB,
            max_window: MIB,
        }
    }
}
