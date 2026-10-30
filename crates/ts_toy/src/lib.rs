// spec: revues/architecture.md §22, §8.1, §9.2, §11, §12
//! Test crate of time_switcher: the **toy world** and the **toy generator** that prove the
//! end criterion of phase 0 (V0-09). A world of 1,000 entities following a home/work agenda
//! runs one simulated year by time skip, with the same state hash from one run to the next,
//! on 1 thread as on N threads, and through a snapshot round trip.
//!
//! **Not a game rule** (D-DEV-02): nothing here is in `standard_rules()`, and no host or rule
//! crate may depend on this crate (checked by `cargo xtask check`, features step). Every
//! duration and size below is a **test parameter**, not a game value: none comes from the
//! design documents, and none may be reused as one.
//!
//! - [`town`]: the toy generator, a grid of parcels and households drawn from keyed streams,
//!   computed in parallel chunks merged in a fixed order (architecture §9.2).
//! - [`world`]: the toy world plugin, which follows the kernel contracts (time skip, AS-27).

#![forbid(unsafe_code)]

pub mod town;
pub mod world;

use ts_kernel::rng::WorldKey;
use ts_kernel::{SimTime, StateDigest};

/// BLAKE3 context of the public test seeds, shared with `ts-sim` (T9b, Y2).
pub const PUBLIC_TEST_SEED_CONTEXT: &str = "time_switcher public test seed v1";

// spec: carnet architecture-securite.md §9.15 Y2 ; revues/generation.md P25
/// World key of a **public** test seed: the first 16 bytes of BLAKE3
/// `derive_key(PUBLIC_TEST_SEED_CONTEXT, seed as u64 LE)`. Public by construction: never
/// the key of an online world.
pub fn public_test_key(seed: u64) -> WorldKey {
    let full = blake3::derive_key(PUBLIC_TEST_SEED_CONTEXT, &seed.to_le_bytes());
    let mut key = [0_u8; 16];
    key.copy_from_slice(&full[..16]);
    WorldKey::from_bytes(key)
}

/// Start of day `day` (day 0 starts at the epoch).
pub const fn day_start(day: i64) -> SimTime {
    SimTime::from_millis(day * SimTime::MILLIS_PER_DAY)
}

/// Lower-case hexadecimal of a digest.
pub fn hex(digest: &StateDigest) -> String {
    digest
        .as_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use ts_kernel::StableId;
    use ts_kernel::rng::{Stream, StreamKeys, draw};

    use super::*;

    /// Each seed gives its own key; the key of seed 1 is frozen by the golden values.
    #[test]
    fn public_test_keys_follow_the_seed() {
        let first = |seed| {
            let keys = StreamKeys::derive(&public_test_key(seed));
            draw(keys.get(Stream::Ai), StableId::from_snapshot_raw(0), 0, 0)
        };
        assert_eq!(first(1), first(1));
        assert_ne!(first(1), first(2));
        assert_eq!(day_start(2), SimTime::from_millis(172_800_000));
    }
}
