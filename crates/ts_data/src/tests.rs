//! Tests of the loader: every bound, bad input, uniqueness, references, fingerprint.

use std::io::Read;

use crate::{
    Code, ContentPack, Diagnostic, FORMAT_VERSION, Limits, LoadErrors, SourceFile, read_bounded,
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
        &"a".repeat(65),
    ] {
        let text = format!("[Item(id: \"{id}\", name: \"A\")]");
        assert_eq!(
            codes(build(vec![file("a.ron", &text)])),
            [Code::InvalidId],
            "id {id:?}"
        );
    }
    for name in ["", "tab\\there", &"n".repeat(257)] {
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
    assert_eq!(pack.canonical_bytes(), expected);
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
