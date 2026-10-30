//! Envelope, manifest and parts: reference vectors, bounds, truncation, forgeries.

#![cfg(test)]

mod common;

use common::{PART_CONTEXT, digest, forge, forge_raw, stored};
use ts_persist::{
    Bound, Codec, DecodeError, FORMAT_VERSION, HEADER_LEN, Limits, MAX_SCHEMA_DEPTH, Manifest,
    PartEntry, Schema, decode_parts, encode_parts, open,
};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn sample(codec: Codec) -> Vec<u8> {
    let parts: [(&str, &[u8]); 3] = [
        ("pop.solo/0002", &[9; 700]),
        ("map.public", b"streets"),
        ("a", b""),
    ];
    encode_parts(&parts, codec, &Limits::default()).unwrap()
}

fn too_large(r: Result<impl Sized, DecodeError>, bound: Bound) {
    match r {
        Err(DecodeError::TooLarge { bound: b, .. }) => assert_eq!(b, bound),
        Err(e) => panic!("{bound:?}: unexpected error {e:?}"),
        Ok(_) => panic!("{bound:?}: accepted"),
    }
}

/// Reference vectors: any change to the layout, the contexts or postcard breaks them.
#[test]
fn reference_vectors() {
    let empty = encode_parts(&[], Codec::Stored, &Limits::default()).unwrap();
    assert_eq!(
        hex(&empty),
        concat!(
            "5453504552535400",                                                 // magic
            "0100",                                                             // format version 1
            "0000",                                                             // no signature
            "01000000",                                                         // manifest length
            "0000000000000000",                                                 // payload length
            "ccd010f9558f6ed73733a47de5709f224d0dab6144d89769584808b4a599ba5b", // manifest digest
            "00"                                                                // manifest: no part
        )
    );
    let one = encode_parts(&[("a", b"hi")], Codec::Stored, &Limits::default()).unwrap();
    let hi = "340547f5e37baf955c2b316fbad3f1ee3b830e0f6cb56496b329f2ace3baae3a";
    assert_eq!(hex(&digest(PART_CONTEXT, b"hi")), hi);
    assert_eq!(
        hex(&one),
        [
            "5453504552535400",
            "0100",
            "0000",
            "46000000",         // manifest length: 70
            "0200000000000000", // payload length: 2
            "e34c4ad6ffd49e4936216c0b1108813467498cad5dac1433adf91e35bf663027", // manifest digest
            "01",               // one part
            "0161",             // name "a"
            "00",               // codec Stored
            "02",
            hi, // stored length and digest
            "02",
            hi,     // decoded length and digest
            "6869", // payload "hi"
        ]
        .concat()
    );
    assert_eq!(HEADER_LEN, 56);
    assert_eq!(FORMAT_VERSION, 1);
}

#[test]
fn round_trip_stored_and_zstd() {
    for codec in [Codec::Stored, Codec::Zstd] {
        let parts = decode_parts(&sample(codec), &Limits::default()).unwrap();
        assert_eq!(
            parts.names().collect::<Vec<_>>(),
            ["a", "map.public", "pop.solo/0002"]
        );
        assert_eq!(parts.get("a"), Some(&b""[..]));
        assert_eq!(parts.get("map.public"), Some(&b"streets"[..]));
        assert_eq!(parts.get("pop.solo/0002"), Some(&[9; 700][..]));
    }
}

#[test]
fn unknown_version_magic_and_signature() {
    let good = sample(Codec::Stored);
    let (_, payload) = open(&good, &Limits::default()).unwrap();
    let manifest = &good[HEADER_LEN..good.len() - payload.len()];
    let v2 = forge_raw(2, 0, manifest, payload);
    assert_eq!(
        decode_parts(&v2, &Limits::default()),
        Err(DecodeError::UnsupportedVersion(2))
    );
    let signed = forge_raw(FORMAT_VERSION, 1, manifest, payload);
    assert_eq!(
        decode_parts(&signed, &Limits::default()),
        Err(DecodeError::UnsupportedSignature(1))
    );
    let mut magic = good.clone();
    magic[0] ^= 1;
    assert_eq!(
        decode_parts(&magic, &Limits::default()),
        Err(DecodeError::BadMagic)
    );
}

/// Each bound, exceeded by one, is refused; at the bound, accepted.
#[test]
fn every_bound_exceeded_by_one() {
    let env = sample(Codec::Zstd);
    let (manifest, _) = open(&env, &Limits::default()).unwrap();
    let longest_name = manifest.parts.0.iter().map(|p| p.name.len()).max().unwrap();
    let largest = manifest.parts.0.iter().map(|p| p.len).max().unwrap();
    let total: u64 = manifest.parts.0.iter().map(|p| p.len).sum();
    let manifest_len = env.len()
        - HEADER_LEN
        - manifest
            .parts
            .0
            .iter()
            .map(|p| p.stored_len as usize)
            .sum::<usize>();
    let d = Limits::default();
    let cases: [(Limits, Limits, Bound); 7] = [
        (
            Limits {
                max_envelope_len: env.len() as u64,
                ..d
            },
            Limits {
                max_envelope_len: env.len() as u64 - 1,
                ..d
            },
            Bound::Envelope,
        ),
        (
            Limits {
                max_manifest_len: manifest_len as u32,
                ..d
            },
            Limits {
                max_manifest_len: manifest_len as u32 - 1,
                ..d
            },
            Bound::Manifest,
        ),
        (
            Limits { max_parts: 3, ..d },
            Limits { max_parts: 2, ..d },
            Bound::Parts,
        ),
        (
            Limits {
                max_name_len: longest_name as u32,
                ..d
            },
            Limits {
                max_name_len: longest_name as u32 - 1,
                ..d
            },
            Bound::Name,
        ),
        (
            Limits {
                max_part_len: largest,
                ..d
            },
            Limits {
                max_part_len: largest - 1,
                ..d
            },
            Bound::Part,
        ),
        (
            Limits {
                max_total_len: total,
                ..d
            },
            Limits {
                max_total_len: total - 1,
                ..d
            },
            Bound::Total,
        ),
        (
            Limits {
                max_window: 128 << 10,
                ..d
            },
            Limits {
                max_window: (128 << 10) - 1,
                ..d
            },
            Bound::Window,
        ),
    ];
    for (at, over, bound) in cases {
        decode_parts(&env, &at).unwrap();
        too_large(decode_parts(&env, &over), bound);
        // The encoder refuses what the decoder would refuse (the window is a decoder bound).
        if bound != Bound::Window {
            let parts: [(&str, &[u8]); 3] = [
                ("pop.solo/0002", &[9; 700]),
                ("map.public", b"streets"),
                ("a", b""),
            ];
            assert!(
                encode_parts(&parts, Codec::Zstd, &over).is_err(),
                "{bound:?}"
            );
        }
    }
}

/// A frame that decodes to far more than its announced length stops one byte past it.
#[test]
fn decompression_bomb_stops_at_the_bound() {
    let zeros = vec![0_u8; 16 << 20];
    let frame =
        ruzstd::encoding::compress_to_vec(&zeros[..], ruzstd::encoding::CompressionLevel::Fastest);
    assert!(frame.len() < 64 << 10, "{}", frame.len());
    let claimed = 1024_u64;
    let entry = PartEntry {
        name: "bomb".to_owned(),
        codec: Codec::Zstd,
        stored_len: frame.len() as u64,
        stored_digest: digest(PART_CONTEXT, &frame),
        len: claimed,
        digest: digest(PART_CONTEXT, &zeros[..1024]),
    };
    let env = forge(vec![entry], &frame);
    assert_eq!(
        decode_parts(&env, &Limits::default()),
        Err(DecodeError::PartLength("bomb".to_owned()))
    );
}

/// Every prefix of a small envelope is refused, without panic.
#[test]
fn truncated_at_every_position() {
    for codec in [Codec::Stored, Codec::Zstd] {
        let env = sample(codec);
        for n in 0..env.len() {
            assert!(
                decode_parts(&env[..n], &Limits::default()).is_err(),
                "{codec:?} {n}"
            );
        }
    }
}

/// Any flipped bit is refused: header fields, manifest (digest), or parts (digests).
#[test]
fn every_flipped_byte_is_refused() {
    for codec in [Codec::Stored, Codec::Zstd] {
        let env = sample(codec);
        for i in 0..env.len() {
            let mut bad = env.clone();
            bad[i] ^= 0x10;
            assert!(
                decode_parts(&bad, &Limits::default()).is_err(),
                "{codec:?} {i}"
            );
        }
    }
}

#[test]
fn forged_manifests_are_refused() {
    let d = Limits::default();
    let a = stored("a", b"x");
    let b = stored("b", b"yz");
    // Consistent forgery is accepted: the helper writes the format right.
    assert!(decode_parts(&forge(vec![a.clone(), b.clone()], b"xyz"), &d).is_ok());
    // Wrong part digest.
    let mut wrong = b.clone();
    wrong.stored_digest = digest(PART_CONTEXT, b"zz");
    wrong.digest = wrong.stored_digest;
    assert_eq!(
        decode_parts(&forge(vec![a.clone(), wrong], b"xyz"), &d),
        Err(DecodeError::PartDigest("b".to_owned()))
    );
    // Missing part bytes, extra part bytes.
    assert_eq!(
        decode_parts(&forge(vec![a.clone(), b.clone()], b"xy"), &d),
        Err(DecodeError::LengthMismatch)
    );
    assert_eq!(
        decode_parts(&forge(vec![a.clone(), b.clone()], b"xyzw"), &d),
        Err(DecodeError::LengthMismatch)
    );
    // Unsorted and duplicated names.
    assert_eq!(
        decode_parts(&forge(vec![b.clone(), a.clone()], b"yzx"), &d),
        Err(DecodeError::UnsortedNames)
    );
    assert_eq!(
        decode_parts(&forge(vec![a.clone(), a.clone()], b"xx"), &d),
        Err(DecodeError::UnsortedNames)
    );
    // Invalid names.
    for name in ["", "A", "a b", "é", "a\\b"] {
        let mut e = a.clone();
        e.name = name.to_owned();
        assert_eq!(
            decode_parts(&forge(vec![e], b"x"), &d),
            Err(DecodeError::InvalidName),
            "{name:?}"
        );
    }
    // A stored part whose decoded digest differs from its stored digest.
    let mut mismatch = a.clone();
    mismatch.digest = [0; 32];
    assert_eq!(
        decode_parts(&forge(vec![mismatch], b"x"), &d),
        Err(DecodeError::CodecMismatch)
    );
    // Wrong envelope digest.
    let mut env = forge(vec![a.clone()], b"x");
    env[24] ^= 1;
    assert_eq!(decode_parts(&env, &d), Err(DecodeError::ManifestDigest));
    // Unknown codec, and bytes after the manifest, behind a correct digest.
    let mut manifest = postcard::to_allocvec(&Manifest {
        parts: ts_persist::Seq(vec![a.clone()]),
    })
    .unwrap();
    let codec_at = 1 + 1 + 1; // count, name length, name
    manifest[codec_at] = 2;
    assert_eq!(
        decode_parts(&forge_raw(FORMAT_VERSION, 0, &manifest, b"x"), &d),
        Err(DecodeError::Malformed("manifest"))
    );
    manifest[codec_at] = 0;
    manifest.push(0);
    assert_eq!(
        decode_parts(&forge_raw(FORMAT_VERSION, 0, &manifest, b"x"), &d),
        Err(DecodeError::TrailingBytes("manifest"))
    );
    // An announced count far above the bytes present fails cleanly, without allocating it.
    let huge = [0xff, 0xff, 0xff, 0xff, 0x0f];
    assert_eq!(
        decode_parts(&forge_raw(FORMAT_VERSION, 0, &huge, b""), &d),
        Err(DecodeError::Malformed("manifest"))
    );
}

/// A zstd part followed by bytes outside its frame is refused.
#[test]
fn bytes_after_the_frame_are_refused() {
    let raw = b"chunk of state";
    let mut frame =
        ruzstd::encoding::compress_to_vec(&raw[..], ruzstd::encoding::CompressionLevel::Fastest);
    frame.push(0);
    let entry = PartEntry {
        name: "p".to_owned(),
        codec: Codec::Zstd,
        stored_len: frame.len() as u64,
        stored_digest: digest(PART_CONTEXT, &frame),
        len: raw.len() as u64,
        digest: digest(PART_CONTEXT, raw),
    };
    assert_eq!(
        decode_parts(&forge(vec![entry], &frame), &Limits::default()),
        Err(DecodeError::TrailingBytes("zstd frame"))
    );
}

/// The schema has no recursive type: its depth is a compile-time constant (Q2).
#[test]
fn schema_depth_is_fixed() {
    assert_eq!(PartEntry::DEPTH, 1);
    assert_eq!(Manifest::DEPTH, 3);
    const { assert!(Manifest::DEPTH <= MAX_SCHEMA_DEPTH) };
}

#[test]
fn seeds_decode() {
    let seeds = ts_persist::fuzzing::seeds();
    assert_eq!(seeds.len(), 6);
    for s in &seeds {
        ts_persist::fuzzing::decode(s).unwrap();
    }
    assert!(ts_persist::fuzzing::decode(b"").is_err());
}
