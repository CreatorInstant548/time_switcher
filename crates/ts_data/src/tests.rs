//! Tests of the loader: every bound, bad input, uniqueness, references, fingerprint.

use std::io::Read;

use crate::{
    Code, ContentPack, Diagnostic, FORMAT_VERSION, IdState, Limits, LoadErrors, NAMESPACES,
    Registry, SourceFile, is_valid_card_id, is_valid_id, read_bounded,
};

const ITEMS: &str = "// spec: revues/architecture.md §5.2\n[\n    Item(id: \"item.bread\", name: \"Pain\"),\n    Item(id: \"item.knife\", name: \"Couteau\"),\n]\n";
const ACTIVITIES: &str = "[\n    Activity(id: \"activity.bake\", name: \"Cuire\", items: [\"item.bread\"]),\n    Activity(id: \"activity.rest\", name: \"Repos\", items: []),\n]\n";

fn file(path: &str, text: &str) -> SourceFile {
    SourceFile {
        path: path.to_owned(),
        bytes: text.as_bytes().to_vec(),
    }
}

fn build(files: Vec<SourceFile>) -> Result<ContentPack, LoadErrors> {
    ContentPack::build(files, &Limits::default())
}

fn codes(r: Result<ContentPack, LoadErrors>) -> Vec<Code> {
    r.unwrap_err()
        .diagnostics()
        .iter()
        .map(|d| d.code)
        .collect()
}

fn first(r: Result<ContentPack, LoadErrors>) -> Diagnostic {
    r.unwrap_err().diagnostics()[0].clone()
}

#[test]
fn loads_the_examples_sorted_by_id() {
    let pack = build(vec![file("b.ron", ITEMS), file("a.ron", ACTIVITIES)]).unwrap();
    let ids: Vec<&str> = pack.cards().iter().map(|c| c.id()).collect();
    assert_eq!(
        ids,
        ["activity.bake", "activity.rest", "item.bread", "item.knife"]
    );
}

#[test]
fn file_over_the_size_bound_is_refused_by_metadata_and_by_content() {
    let limits = Limits {
        max_file_bytes: 8,
        ..Limits::default()
    };
    // Declared size above the bound: refused before any read.
    let d = read_bounded("a.ron", 9, std::io::empty(), &limits).unwrap_err();
    assert_eq!(d.code, Code::FileTooLarge);
    // A file that grew after the metadata check: declared small, stream larger.
    let d = read_bounded("a.ron", 4, &b"123456789abc"[..], &limits).unwrap_err();
    assert_eq!(d.code, Code::FileTooLarge);
    // Exactly at the bound is fine.
    let f = read_bounded("a.ron", 8, &b"12345678"[..], &limits).unwrap();
    assert_eq!(f.bytes, b"12345678");
    // The pack refuses an oversized file even if the host forgot to check.
    let r = ContentPack::build(
        vec![file("a.ron", ITEMS)],
        &Limits {
            max_file_bytes: 8,
            ..Limits::default()
        },
    );
    assert_eq!(codes(r), [Code::FileTooLarge]);
}

#[test]
fn read_bounded_never_reads_more_than_one_byte_past_the_bound() {
    struct Endless(u64);
    impl Read for Endless {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.0 += buf.len() as u64;
            buf.fill(b'x');
            Ok(buf.len())
        }
    }
    let limits = Limits {
        max_file_bytes: 1000,
        ..Limits::default()
    };
    let mut endless = Endless(0);
    let d = read_bounded("a.ron", 0, &mut endless, &limits).unwrap_err();
    assert_eq!(d.code, Code::FileTooLarge);
    assert!(endless.0 <= 1001 + 8192, "read {} bytes", endless.0);
}

#[test]
fn read_error_is_typed() {
    struct Broken;
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("boom"))
        }
    }
    let d = read_bounded("a.ron", 1, Broken, &Limits::default()).unwrap_err();
    assert_eq!(d.code, Code::ReadFailed);
}

#[test]
fn too_many_files() {
    let limits = Limits {
        max_files: 1,
        ..Limits::default()
    };
    let r = ContentPack::build(vec![file("a.ron", "[]"), file("b.ron", "[]")], &limits);
    assert_eq!(codes(r), [Code::TooManyFiles]);
}

#[test]
fn too_many_cards() {
    let limits = Limits {
        max_cards: 3,
        ..Limits::default()
    };
    let r = ContentPack::build(
        vec![file("a.ron", ITEMS), file("b.ron", ACTIVITIES)],
        &limits,
    );
    let d = first(r);
    assert_eq!((d.code, d.file.as_str()), (Code::TooManyCards, "b.ron"));
}

#[test]
fn nesting_over_the_depth_bound() {
    let limits = Limits {
        max_depth: 1,
        ..Limits::default()
    };
    let r = ContentPack::build(vec![file("a.ron", ITEMS)], &limits);
    assert_eq!(codes(r), [Code::RecursionLimit]);
}

#[test]
fn hostile_nesting_fails_without_panic() {
    let text = format!("[{}", "(".repeat(200_000));
    assert_eq!(codes(build(vec![file("a.ron", &text)])), [Code::RonSyntax]);
    let text = format!("[{}", "[".repeat(200_000));
    assert_eq!(codes(build(vec![file("a.ron", &text)])).len(), 1);
}

#[test]
fn malformed_ron_has_file_line_and_stable_code() {
    let d = first(build(vec![file(
        "dir/a.ron",
        "[\n    Item(id: \"item.a\", name: )\n]\n",
    )]));
    assert_eq!(
        (d.code, d.file.as_str(), d.line),
        (Code::RonSyntax, "dir/a.ron", 2)
    );
    assert!(d.column > 0);
    assert_eq!(d.code.as_str(), "DATA_RON_SYNTAX");
    assert!(d.to_string().starts_with("dir/a.ron:2:"));
}

#[test]
fn bad_encoding_and_schema_errors() {
    let bad = SourceFile {
        path: "a.ron".to_owned(),
        bytes: vec![b'[', 0xff, 0xfe],
    };
    assert_eq!(codes(build(vec![bad])), [Code::NotUtf8]);
    let d = first(build(vec![file("a.ron", "[Item(id: \"item.a\")]")]));
    assert_eq!((d.code, d.detail.as_str()), (Code::MissingField, "name"));
    let d = first(build(vec![file(
        "a.ron",
        "[Item(id: \"item.a\", name: \"A\", price: 3)]",
    )]));
    assert_eq!((d.code, d.detail.as_str()), (Code::UnknownField, "price"));
    let d = first(build(vec![file(
        "a.ron",
        "[Vehicle(id: \"v.a\", name: \"A\")]",
    )]));
    assert_eq!((d.code, d.detail.as_str()), (Code::UnknownKind, "Vehicle"));
    let d = first(build(vec![file(
        "a.ron",
        "[Activity(id: \"activity.a\", name: \"A\")]",
    )]));
    assert_eq!((d.code, d.detail.as_str()), (Code::MissingField, "items"));
}

#[test]
fn invalid_id_and_name() {
    for id in [
        "",
        "Item.a",
        "item..a",
        "item.",
        ".item",
        "1item",
        "item.a-b",
        "item",
        "item.a ",
        "item.\u{e9}",
        &format!("item.{}", "a".repeat(60)),
    ] {
        let text = format!("[Item(id: \"{id}\", name: \"A\")]");
        assert_eq!(
            codes(build(vec![file("a.ron", &text)])),
            [Code::InvalidId],
            "id {id:?}"
        );
    }
    for name in [
        "",
        "tab\\there",
        &"n".repeat(257),
        " Pain",
        "Pain ",
        "Pain\u{a0}",
        "\u{3000}Pain",
        // `e` then a combining acute accent: NFD, not NFC (M40e).
        "Pa\u{65}\u{301}n",
    ] {
        let text = format!("[Item(id: \"item.a\", name: \"{name}\")]");
        assert_eq!(
            codes(build(vec![file("a.ron", &text)])),
            [Code::InvalidName],
            "name {name:?}"
        );
    }
}

#[test]
fn duplicate_id_across_files_points_at_the_second_card() {
    let a = file("a.ron", "[Item(id: \"item.x\", name: \"A\")]");
    let b = file(
        "b.ron",
        "[\n    Item(id: \"item.y\", name: \"B\"),\n    Item(id: \"item.x\", name: \"C\"),\n]",
    );
    let d = first(build(vec![b, a]));
    assert_eq!(
        (d.code, d.file.as_str(), d.line, d.column),
        (Code::DuplicateId, "b.ron", 3, 5)
    );
    assert_eq!(d.detail, "item.x");
}

#[test]
fn broken_reference_to_nothing_or_to_a_non_item() {
    let act = "[Activity(id: \"activity.a\", name: \"A\", items: [\"item.none\"])]";
    let d = first(build(vec![file("a.ron", act)]));
    assert_eq!(
        (d.code, d.detail.as_str()),
        (Code::BrokenReference, "item.none")
    );
    let act = "[Activity(id: \"activity.a\", name: \"A\", items: [\"activity.a\"])]";
    assert_eq!(
        codes(build(vec![file("a.ron", act)])),
        [Code::BrokenReference]
    );
}

#[test]
fn errors_are_reported_in_path_order_whatever_the_input_order() {
    let a = file("a.ron", "[");
    let b = file("b.ron", "[");
    let one = build(vec![a.clone(), b.clone()]).unwrap_err();
    let two = build(vec![b, a]).unwrap_err();
    assert_eq!(one, two);
    assert_eq!(one.diagnostics().len(), 2);
}

#[test]
fn bad_paths_are_refused() {
    for path in [
        "/a.ron",
        "../a.ron",
        "a/../b.ron",
        "C:/a.ron",
        "a\\b.ron",
        "",
        "a//b.ron",
    ] {
        assert_eq!(
            codes(build(vec![file(path, "[]")])),
            [Code::BadPath],
            "path {path:?}"
        );
    }
}

#[test]
fn fingerprint_ignores_line_endings_file_order_and_file_layout() {
    let reference = build(vec![file("a.ron", ITEMS), file("b.ron", ACTIVITIES)]).unwrap();
    let crlf = |s: &str| s.replace('\n', "\r\n");
    let variants = [
        vec![
            file("a.ron", &crlf(ITEMS)),
            file("b.ron", &crlf(ACTIVITIES)),
        ],
        vec![file("b.ron", ACTIVITIES), file("a.ron", ITEMS)],
        vec![file("other/dir/x.ron", ACTIVITIES), file("z.ron", ITEMS)],
        vec![file(
            "all.ron",
            &format!(
                "{}{}",
                ITEMS.replace("]\n", ""),
                ACTIVITIES.replacen('[', "", 1)
            ),
        )],
    ];
    for files in variants {
        let pack = build(files).unwrap();
        assert_eq!(pack.fingerprint(), reference.fingerprint());
        assert_eq!(pack.canonical_bytes(), reference.canonical_bytes());
    }
}

#[test]
fn fingerprint_changes_with_the_content() {
    let a = build(vec![file("a.ron", "[Item(id: \"item.a\", name: \"A\")]")]).unwrap();
    let b = build(vec![file("a.ron", "[Item(id: \"item.a\", name: \"B\")]")]).unwrap();
    let c = build(vec![file("a.ron", "[Item(id: \"item.b\", name: \"A\")]")]).unwrap();
    assert_ne!(a.fingerprint(), b.fingerprint());
    assert_ne!(a.fingerprint(), c.fingerprint());
}

#[test]
fn canonical_bytes_and_fingerprint_reference_vector() {
    let pack = build(vec![file("a.ron", "[Item(id: \"item.a\", name: \"A\")]")]).unwrap();
    let mut expected = Vec::new();
    expected.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    expected.extend_from_slice(&1u32.to_le_bytes());
    expected.push(0x01);
    expected.extend_from_slice(&6u32.to_le_bytes());
    expected.extend_from_slice(b"item.a");
    expected.extend_from_slice(&1u32.to_le_bytes());
    expected.extend_from_slice(b"A");
    assert_eq!(pack.canonical_bytes(), expected.as_slice());
    assert_eq!(pack.fingerprint().to_string(), REFERENCE_FINGERPRINT);
    // Keyed derivation: not the plain hash of the bytes.
    assert_ne!(
        pack.fingerprint().as_bytes(),
        blake3::hash(&expected).as_bytes()
    );
}

const REFERENCE_FINGERPRINT: &str =
    "8c59bb3265598f29bd09b3758fd7286d18e349fa40b9d6443386c279a2680d64";

#[test]
fn every_truncation_of_a_valid_file_fails_or_loads_without_panic() {
    let text = format!("{ITEMS}{ACTIVITIES}").replace("]\n[", ",\n");
    for end in 0..=text.len() {
        let mut bytes = text.as_bytes().to_vec();
        bytes.truncate(end);
        let _ = build(vec![SourceFile {
            path: "a.ron".to_owned(),
            bytes,
        }]);
    }
}

#[test]
fn position_scanner_handles_comments_and_strings() {
    let text = "#![enable(implicit_some)]\n[\n    // Item(id: \"x\"),\n    /* , */ Item(id: \"item.a\", name: \"A, [B]\"),\n    Item(id: \"item.a\", name: r#\"x\"y\"#),\n]";
    let d = first(build(vec![file("a.ron", text)]));
    assert_eq!((d.code, d.line, d.column), (Code::DuplicateId, 5, 5));
}

#[test]
fn total_size_bound() {
    let limits = Limits {
        max_total_bytes: 100,
        ..Limits::default()
    };
    let r = ContentPack::build(
        vec![file("a.ron", ITEMS), file("b.ron", ACTIVITIES)],
        &limits,
    );
    assert_eq!(codes(r), [Code::TotalTooLarge]);
}

#[test]
fn byte_order_mark_is_skipped() {
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(ITEMS.as_bytes());
    let with_bom = build(vec![SourceFile {
        path: "a.ron".to_owned(),
        bytes,
    }])
    .unwrap();
    let without = build(vec![file("a.ron", ITEMS)]).unwrap();
    assert_eq!(with_bom.fingerprint(), without.fingerprint());
}

#[test]
fn a_rejected_card_does_not_cause_broken_references() {
    let bad_item = file(
        "items.ron",
        "[Item(id: \"Bad Id\", name: \"A\"), Item(id: \"item.ok\", name: \"B\")]",
    );
    let act = file(
        "act.ron",
        "[Activity(id: \"activity.a\", name: \"A\", items: [\"item.ok\", \"item.none\"])]",
    );
    assert_eq!(codes(build(vec![bad_item, act.clone()])), [Code::InvalidId]);
    let broken_file = file("items.ron", "[");
    assert_eq!(codes(build(vec![broken_file, act])), [Code::RonSyntax]);
}

#[test]
fn detail_is_escaped_and_short() {
    let d = Diagnostic::new(
        Code::InvalidId,
        "a.ron",
        (1, 1),
        "x\n/a.ron:1:1: [DATA_OK]\u{1b}[31m",
    );
    assert!(
        !d.detail.contains('\n') && !d.detail.contains('\u{1b}'),
        "{:?}",
        d.detail
    );
    assert_eq!(d.to_string().lines().count(), 1);
    let long = Diagnostic::new(Code::InvalidId, "a.ron", (1, 1), &"z".repeat(500));
    assert_eq!(long.detail.len(), 80);
}

#[test]
fn many_errors_are_capped_and_counted() {
    let cards: String = (0..200)
        .map(|i| format!("Item(id: \"BAD{i}\", name: \"A\"),"))
        .collect();
    let errors = build(vec![file("a.ron", &format!("[{cards}]"))]).unwrap_err();
    assert_eq!(errors.diagnostics().len(), crate::MAX_DIAGNOSTICS);
    assert_eq!(errors.dropped(), 200 - crate::MAX_DIAGNOSTICS);
    assert!(errors.to_string().contains("136 autre(s) erreur(s)"));
}

#[test]
fn lengths_are_never_saturated() {
    let mut out = Vec::new();
    assert!(crate::pack::put_u32(&mut out, 7).is_some());
    assert_eq!(out, 7_u32.to_le_bytes());
    // On a 32-bit target no `usize` exceeds `u32::MAX`, and there is nothing to refuse.
    if let Ok(big) = usize::try_from(u64::from(u32::MAX) + 1) {
        assert!(crate::pack::put_u32(&mut out, big).is_none());
        assert!(crate::pack::put_u32(&mut out, usize::MAX).is_none());
    }
}

#[test]
fn file_name_is_escaped_in_diagnostics() {
    let d = Diagnostic::new(
        Code::RonSyntax,
        "a\n/b.ron:1:1: [DATA_OK]\u{1b}[31m.ron",
        (1, 1),
        "",
    );
    assert!(
        !d.file.contains('\n') && !d.file.contains('\u{1b}'),
        "{:?}",
        d.file
    );
    assert_eq!(d.to_string().lines().count(), 1);
}

#[test]
fn bad_path_never_echoes_the_path() {
    let d = first(build(vec![file("/absolute/hidden.ron", "[]")]));
    assert_eq!(d.code, Code::BadPath);
    assert!(d.file.is_empty() && d.detail.is_empty());
    assert!(!d.to_string().contains("hidden"));
}

mod fuzzing {
    use crate::fuzzing::{SEPARATOR, limits, load, seeds};
    use crate::{Code, FORMAT_VERSION};

    use super::{ACTIVITIES, ITEMS};

    fn examples() -> Vec<Vec<u8>> {
        vec![ITEMS.as_bytes().to_vec(), ACTIVITIES.as_bytes().to_vec()]
    }

    fn code(data: &[u8]) -> Code {
        load(data).unwrap_err().diagnostics()[0].code
    }

    #[test]
    fn joined_examples_load_as_a_pack() {
        let data = [ITEMS.as_bytes(), ACTIVITIES.as_bytes()].join(&SEPARATOR);
        let pack = load(&data).unwrap();
        assert_eq!(pack.cards().len(), 4);
        assert!(
            pack.canonical_bytes()
                .starts_with(&FORMAT_VERSION.to_le_bytes())
        );
        // One file alone: the activity refers to an item of the other file.
        assert_eq!(code(ACTIVITIES.as_bytes()), Code::BrokenReference);
        assert_eq!(load(ITEMS.as_bytes()).unwrap().cards().len(), 2);
    }

    #[test]
    fn every_seed_runs_and_the_examples_seeds_load() {
        let seeds = seeds(&examples());
        assert!(seeds.len() > 10);
        for seed in &seeds {
            let _ = load(seed);
        }
        // Joined in order and in reverse: the same pack.
        let (Ok(a), Ok(b)) = (load(&seeds[2]), load(&seeds[3])) else {
            panic!("the joined examples must load");
        };
        assert_eq!(a.fingerprint(), b.fingerprint());
    }

    #[test]
    fn hand_written_seeds_reach_their_paths() {
        let seeds = seeds(&[]);
        // Two empty joins first, then each hand-written input alone and after the examples.
        let codes: Vec<Option<Code>> = seeds
            .iter()
            .skip(2)
            .step_by(2)
            .map(|s| load(s).err().map(|e| e.diagnostics()[0].code))
            .collect();
        assert_eq!(
            codes[0], None,
            "byte order mark, raw strings and escapes load"
        );
        assert_eq!(codes[1], None, "comments and trailing commas load");
        assert_eq!(codes[2], Some(Code::DuplicateId));
        assert_eq!(codes[3], Some(Code::BrokenReference));
        assert_eq!(codes[4], Some(Code::InvalidId));
        assert_eq!(codes[5], Some(Code::UnknownField));
        assert_eq!(codes[6], Some(Code::MissingField));
        assert_eq!(codes[7], Some(Code::RonSyntax));
    }

    #[test]
    fn names_with_quotes_and_backslashes_round_trip() {
        let data = br#"[Item(id: "item.q", name: r"a\b"), Item(id: "item.r", name: "say \"hi\" \\ \u{e9}")]"#;
        assert_eq!(load(data).unwrap().cards().len(), 2);
    }

    /// A small stable stand-in for the fuzzer: byte flips, insertions and deletions over the
    /// seeds, with a fixed generator. Any panic is an invariant broken by a real input.
    #[test]
    fn mutated_seeds_keep_the_invariants() {
        let seeds = seeds(&examples());
        let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next = |bound: usize| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            usize::try_from(state >> 33).unwrap() % bound.max(1)
        };
        let alphabet = b"[](){},:\"\\#r/ \n\0aeiZ._019\xc3\xa9\xff";
        for round in 0..3000 {
            let mut data = seeds[round % seeds.len()].clone();
            for _ in 0..1 + next(4) {
                let at = next(data.len() + 1);
                let byte = alphabet[next(alphabet.len())];
                match next(3) {
                    0 if at < data.len() => data[at] = byte,
                    1 if at < data.len() => {
                        data.remove(at);
                    }
                    _ => data.insert(at, byte),
                }
            }
            let _ = load(&data);
        }
    }

    #[test]
    fn bounds_of_the_target() {
        let limits = limits();
        assert_eq!(
            load(b"").err().map(|e| e.diagnostics()[0].code),
            Some(Code::RonSyntax)
        );
        let many = vec![SEPARATOR; limits.max_files];
        assert_eq!(code(&many), Code::TooManyFiles);
        let big = vec![b' '; usize::try_from(limits.max_file_bytes).unwrap() + 1];
        assert_eq!(code(&big), Code::FileTooLarge);
        let empty_lists = [b"[]".as_slice(); 3].join(&SEPARATOR);
        assert_eq!(load(&empty_lists).unwrap().cards().len(), 0);
    }
}

// spec: attendus-mesurables.md §10.2 (M40a, M40b, M40e)
#[test]
fn identifier_grammar_namespace_and_name_rules_of_a15() {
    // M40a: at least two segments for a card, 64 bytes at most; the bare grammar keeps one.
    assert!(is_valid_id("toy_year") && !is_valid_card_id("toy_year"));
    assert!(is_valid_card_id("item.bread") && is_valid_card_id("item.a.b_2"));
    let longest = format!("item.{}", "a".repeat(59));
    assert!(longest.len() == 64 && is_valid_card_id(&longest));
    // M40b: the namespace is the kind in snake_case, whatever the file or its folder.
    assert_eq!(NAMESPACES, ["activity", "item"]);
    let pack = build(vec![
        file("domain/deep/x.ron", "[Item(id: \"item.a\", name: \"A\")]"),
        file(
            "flat.ron",
            "[Activity(id: \"activity.a\", name: \"A\", items: [\"item.a\"])]",
        ),
    ])
    .unwrap();
    for card in pack.cards() {
        assert!(NAMESPACES.contains(&card.namespace()));
        assert!(card.id().starts_with(&format!("{}.", card.namespace())));
    }
    for text in [
        "[Item(id: \"activity.a\", name: \"A\")]",
        "[Item(id: \"act.a\", name: \"A\")]",
        "[Activity(id: \"item.a\", name: \"A\", items: [])]",
        "[Item(id: \"items.a\", name: \"A\")]",
    ] {
        let d = first(build(vec![file("a.ron", text)]));
        assert_eq!(d.code, Code::WrongNamespace, "{text}");
        assert_eq!(d.code.as_str(), "DATA_WRONG_NAMESPACE");
    }
    // M40e: NFC with accents loads, as raw text or RON escapes; 256 bytes is the bound.
    let accents = build(vec![file(
        "a.ron",
        "[Item(id: \"item.a\", name: \"P\u{e2}tisserie à l'étage\")]",
    )])
    .unwrap();
    assert_eq!(accents.cards()[0].name(), "Pâtisserie à l'étage");
    let text = format!("[Item(id: \"item.a\", name: \"{}\")]", "n".repeat(256));
    assert!(build(vec![file("a.ron", &text)]).is_ok());
}

// spec: attendus-mesurables.md §10.2 (M40d)
mod registry {
    use super::*;

    const LOCK: &str = "# header\nactivity.bake active\n\nactivity.rest active\nitem.bread active\nitem.knife active\n";

    fn pack() -> ContentPack {
        build(vec![file("b.ron", ITEMS), file("a.ron", ACTIVITIES)]).unwrap()
    }

    fn parse(text: &str) -> Registry {
        Registry::parse(text.as_bytes(), &Limits::default()).unwrap()
    }

    fn found<T: std::fmt::Debug>(r: Result<T, LoadErrors>) -> Vec<(Code, u32, String)> {
        r.unwrap_err()
            .diagnostics()
            .iter()
            .map(|d| (d.code, d.line, d.detail.clone()))
            .collect()
    }

    #[test]
    fn parses_entries_in_order_with_comments_and_crlf() {
        let r = parse(&LOCK.replace('\n', "\r\n"));
        let ids: Vec<(&str, IdState)> = r.entries().collect();
        assert_eq!(ids.len(), 4);
        assert_eq!(ids[0], ("activity.bake", IdState::Active));
        assert!(parse("").entries().next().is_none());
        // A byte order mark is skipped, as in a card file.
        assert_eq!(parse(&format!("\u{feff}{LOCK}")).entries().count(), 4);
        // Bounded by the pack's total bound.
        let small = Limits {
            max_total_bytes: 8,
            ..Limits::default()
        };
        let f = found(Registry::parse(b"item.a active", &small));
        assert_eq!(f[0].0, Code::FileTooLarge);
        assert_eq!(
            parse("item.a retired").entries().next(),
            Some(("item.a", IdState::Retired))
        );
    }

    #[test]
    fn malformed_lines_and_duplicates_are_refused_with_their_line() {
        for (text, line) in [
            ("item.a", 1),
            ("item.a  active", 1),
            ("item.a active ", 1),
            ("item.a Active", 1),
            (" item.a active", 1),
            ("# c\nitem active", 2),
            ("item.a removed", 1),
            ("Item.a active", 1),
        ] {
            let f = found(Registry::parse(text.as_bytes(), &Limits::default()));
            assert_eq!((f[0].0, f[0].1), (Code::RegistrySyntax, line), "{text:?}");
        }
        let f = found(Registry::parse(
            b"item.a active\nitem.a retired\n",
            &Limits::default(),
        ));
        assert_eq!(f, [(Code::RegistryDuplicate, 2, "item.a".to_owned())]);
        let f = found(Registry::parse(&[b'i', 0xff], &Limits::default()));
        assert_eq!(f[0].0, Code::NotUtf8);
    }

    #[test]
    fn pack_must_match_the_registry() {
        parse(LOCK).check_pack(&pack()).unwrap();
        // A card missing from the registry.
        let lock = LOCK.replace("item.knife active\n", "");
        let f = found(parse(&lock).check_pack(&pack()));
        assert_eq!(f, [(Code::Unregistered, 0, "item.knife".to_owned())]);
        // A retired identifier used again.
        let lock = LOCK.replace("item.knife active", "item.knife retired");
        let f = found(parse(&lock).check_pack(&pack()));
        assert_eq!(f, [(Code::RetiredReused, 6, "item.knife".to_owned())]);
        // An active identifier whose card was removed: it must be marked retired.
        let lock = format!("{LOCK}item.old active\nitem.gone retired\n");
        let f = found(parse(&lock).check_pack(&pack()));
        assert_eq!(f, [(Code::ActiveWithoutCard, 7, "item.old".to_owned())]);
    }

    #[test]
    fn registry_is_append_only() {
        let old = parse(LOCK);
        // Same, appended, retired, comments changed: allowed.
        let appended = format!("{LOCK}item.new active\nitem.newer retired\n");
        let retired = LOCK.replace("item.bread active", "item.bread retired");
        let comments = LOCK.replace("# header", "# other\n# lines");
        for next in [LOCK.to_owned(), appended, retired.clone(), comments] {
            old.check_successor(&parse(&next)).unwrap();
        }
        // Removed, renamed, moved, inserted: refused.
        let removed = LOCK.replace("item.knife active\n", "");
        let renamed = LOCK.replace("item.knife", "item.blade");
        let moved = LOCK.replace(
            "item.bread active\nitem.knife active",
            "item.knife active\nitem.bread active",
        );
        let inserted = LOCK.replace("item.bread", "item.apple active\nitem.bread");
        for next in [removed, renamed, moved, inserted] {
            let f = found(old.check_successor(&parse(&next)));
            assert_eq!(f[0].0, Code::RegistryRewritten, "{next}");
        }
        // Retired back to active: refused.
        let f = found(parse(&retired).check_successor(&old));
        assert_eq!(f, [(Code::RegistryRewritten, 5, "item.bread".to_owned())]);
    }
}
