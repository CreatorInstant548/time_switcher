// spec: carnet architecture-securite.md SEC0-24 (révisé)
//! Fuzzing target of the snapshot decoder: envelope, manifest, parts, then the kernel's
//! canonical decoder (`MemorySnapshot::from_canonical`, AS-36), also fed directly. The body
//! lives in `ts_persist::fuzzing::decode`, which compiles and is tested on stable.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = ts_persist::fuzzing::decode(data);
});
