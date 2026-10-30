//! Bounds of decoding (SEC0-23): each host can pass its own, the defaults are cautious.

// spec: carnet architecture-securite.md SEC0-23 ; revues/securite.md S10, S24 ; revues/architecture.md §12.3
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

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_envelope_len: 256 * MIB, // PROVISOIRE(conception #45)
            max_manifest_len: 4 << 20,   // PROVISOIRE(conception #45)
            max_parts: 65_536,           // PROVISOIRE(conception #45)
            max_name_len: 128,           // PROVISOIRE(conception #45)
            max_part_len: 512 * MIB,     // PROVISOIRE(conception #45)
            max_total_len: 1024 * MIB,   // PROVISOIRE(conception #45)
            max_window: 8 * MIB,         // PROVISOIRE(conception #45)
        }
    }
}

impl Limits {
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
