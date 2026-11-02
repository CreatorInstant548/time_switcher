// spec: carnet architecture-securite.md SEC0-24 (révisé) ; revues/architecture.md §5.2
//! Fuzzing target of the content pack loader (m7 of the review of #47, #48): bounded reading,
//! RON parsing, validation and canonical encoding, with the loader's invariants checked. The
//! body lives in `ts_data::fuzzing::load`, which compiles and is tested on stable.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = ts_data::fuzzing::load(data);
});
