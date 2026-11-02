//! Body of the fuzzing target `fuzz/fuzz_targets/content.rs` and its initial corpus, kept in
//! the library so that they compile and run on stable (A14: no local nightly). Development
//! only: absent from the `public` profile (see the crate root).
//!
//! Input format: the files of a pack, separated by a `0` byte (RON text never needs one). The
//! files are named `f00.ron`, `f01.ron`, and so on, in input order. A raw card file is thus a
//! one-file input, and a few card files joined by `0` a whole pack.

use crate::{Card, ContentPack, FORMAT_VERSION, Limits, LoadErrors, SourceFile, read_bounded};

/// Separator of the files of an input.
pub const SEPARATOR: u8 = 0;

/// Bounds of the target: small enough for fast runs, large enough for every error path.
pub fn limits() -> Limits {
    Limits {
        max_file_bytes: 64 << 10,
        max_total_bytes: 256 << 10,
        max_files: 8,
        max_cards: 512,
        max_dir_depth: 0,
        ..Limits::default()
    }
}

/// Splits `data` into source files, each read through [`read_bounded`] as a host does. At
/// most `max_files + 1` files, so the file bound is reached but never a huge list.
fn split(data: &[u8], limits: &Limits) -> Result<Vec<SourceFile>, LoadErrors> {
    let mut files = Vec::new();
    for (i, bytes) in data
        .split(|b| *b == SEPARATOR)
        .take(limits.max_files.saturating_add(1))
        .enumerate()
    {
        let path = format!("f{i:02}.ron");
        let declared = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        files.push(read_bounded(&path, declared, bytes, limits).map_err(LoadErrors::single)?);
    }
    Ok(files)
}

// spec: carnet architecture-securite.md SEC0-24 (révisé), §10 ; revues/architecture.md §5.2
/// Loads `data` as a pack under [`limits`]: bounded reading, RON parsing, validation and
/// canonical encoding, then checks three invariants of the loader:
/// - the same files under other names (the input order reversed, then renamed) are valid
///   exactly when these are, and then give the same pack: neither file names nor their order
///   change the result;
/// - a valid pack starts its canonical bytes with [`FORMAT_VERSION`] and its card count;
/// - a valid pack written back as one RON file loads to the same cards and fingerprint.
///
/// It returns errors for any invalid `data` and never panics on them.
///
/// # Panics
///
/// Only when an invariant fails: that is a finding of the fuzzer.
pub fn load(data: &[u8]) -> Result<ContentPack, LoadErrors> {
    let limits = limits();
    let files = split(data, &limits)?;
    let renamed: Vec<SourceFile> = files
        .iter()
        .rev()
        .enumerate()
        .map(|(i, f)| SourceFile {
            path: format!("f{i:02}.ron"),
            bytes: f.bytes.clone(),
        })
        .collect();
    let pack = ContentPack::build(files, &limits);
    let other = ContentPack::build(renamed, &limits);
    assert_eq!(
        pack.is_ok(),
        other.is_ok(),
        "file names or order changed whether the pack is valid"
    );
    if let (Ok(a), Ok(b)) = (&pack, &other) {
        assert_eq!(a, b, "file names or order changed the pack");
    }
    if let Ok(pack) = &pack {
        let count = u32::try_from(pack.cards().len()).unwrap_or(u32::MAX);
        let head = [FORMAT_VERSION.to_le_bytes(), count.to_le_bytes()].concat();
        assert!(pack.canonical_bytes().starts_with(&head), "canonical header");
        let again = SourceFile {
            path: "again.ron".to_owned(),
            bytes: to_ron(pack.cards()).into_bytes(),
        };
        let unbounded = Limits {
            max_file_bytes: u64::MAX,
            max_total_bytes: u64::MAX,
            ..limits
        };
        assert_eq!(
            ContentPack::build(vec![again], &unbounded).as_ref(),
            Ok(pack),
            "a valid pack written back as RON does not load the same"
        );
    }
    pack
}

/// Writes valid cards back as one RON list. Identifiers have no character to escape, and a
/// valid name has no control character: only `\` and `"` are escaped.
fn to_ron(cards: &[Card]) -> String {
    let quote = |s: &str| format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""));
    let mut out = String::from("[\n");
    for card in cards {
        let line = match card {
            Card::Item { id, name } => format!("Item(id: {}, name: {}),\n", quote(id), quote(name)),
            Card::Activity { id, name, items } => {
                let items: Vec<String> = items.iter().map(|i| quote(i)).collect();
                format!(
                    "Activity(id: {}, name: {}, items: [{}]),\n",
                    quote(id),
                    quote(name),
                    items.join(", ")
                )
            }
        };
        out.push_str(&line);
    }
    out.push(']');
    out
}

/// Hand-written inputs that reach the syntax and validation paths the example cards do not:
/// byte order mark, raw strings and escapes, comments, and one input per kind of error.
const HAND_WRITTEN: [&str; 8] = [
    "\u{feff}[Item(id: \"item.a\", name: r#\"Raw \"quoted\"\"#), Item(id: \"item.b\", name: \"\\u{e9}t\\\"\\\\\")]",
    "/* block */ [ // line\n Activity(id: \"activity.a\", name: \"A\", items: [\"item.a\",],), Item(id: \"item.a\", name: \"A\"),\n]",
    "[Item(id: \"item.a\", name: \"A\"), Item(id: \"item.a\", name: \"B\")]",
    "[Activity(id: \"activity.a\", name: \"A\", items: [\"item.missing\", \"activity.a\"])]",
    "[Item(id: \"Item.Bad\", name: \"\"), Item(id: \"item.c\", name: \"tab\\there\")]",
    "[Item(id: \"item.a\", name: \"A\", extra: 1), Tool(id: \"tool.a\")]",
    "[Activity(id: \"activity.a\", name: \"A\")]",
    "[[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]",
];

// spec: carnet architecture-securite.md SEC0-24 (révisé), §10
/// Initial corpus of the target: each of `examples` alone, all of them joined (in order and
/// in reverse), and the hand-written inputs, alone and joined after the examples. Written by
/// `fuzz/seeds` into the corpus directory (regenerable, never committed).
pub fn seeds(examples: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let join = |parts: &[&[u8]]| parts.join(&SEPARATOR);
    let all: Vec<&[u8]> = examples.iter().map(Vec::as_slice).collect();
    let mut reversed = all.clone();
    reversed.reverse();
    let mut out: Vec<Vec<u8>> = examples.to_vec();
    out.push(join(&all));
    out.push(join(&reversed));
    for text in HAND_WRITTEN {
        out.push(text.as_bytes().to_vec());
        let mut with = all.clone();
        with.push(text.as_bytes());
        out.push(join(&with));
    }
    out
}
