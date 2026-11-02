//! `ts-data check` end to end: the example pack, the measure of each data bound (M39a to
//! M39f), the warning annotation over half of a bound, and the refusals.
#![allow(clippy::disallowed_methods, clippy::unwrap_used)] // test: throwaway directories.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ts_data::Limits;
use ts_runner::measure_content_dir;

const EXAMPLES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/content");
const ITEM: &str = "[Item(id: \"item.x\", name: \"X\")]";

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("ts_data")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn ts_data(args: &[&str], dir: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ts-data"))
        .arg("check")
        .arg(dir)
        .args(args)
        .output()
        .unwrap()
}

fn stdout(o: &Output) -> String {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout.clone()).unwrap()
}

/// A pack of one item card, `depth` directories down, with its registry.
fn nested_pack(name: &str, depth: usize) -> PathBuf {
    let root = scratch(name);
    let mut dir = root.clone();
    for level in 0..depth {
        dir = dir.join(format!("d{level}"));
    }
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("x.ron"), ITEM).unwrap();
    fs::write(root.join("ids.lock"), "item.x active\n").unwrap();
    root
}

#[test]
fn examples_report_every_bound_and_no_warning() {
    let out = stdout(&ts_data(&["--github"], Path::new(EXAMPLES)));
    let ids: Vec<&str> = out
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .filter(|w| w.starts_with("M39"))
        .collect();
    assert_eq!(
        ids,
        ["M39a", "M39b", "M39c", "M39d", "M39e", "M39f"],
        "{out}"
    );
    assert!(out.starts_with("ts-data check: 4 cards"), "{out}");
    assert!(
        !out.contains("::warning") && !out.contains("OVER HALF"),
        "{out}"
    );
}

#[test]
fn measure_of_the_examples() {
    let (pack, usage) = measure_content_dir(Path::new(EXAMPLES), &Limits::default()).unwrap();
    let sizes: Vec<u64> = ["activities.ron", "items.ron"]
        .iter()
        .map(|f| fs::metadata(Path::new(EXAMPLES).join(f)).unwrap().len())
        .collect();
    assert_eq!(usage.file_bytes.0, sizes.iter().copied().max().unwrap());
    assert_eq!(usage.total_bytes, sizes.iter().sum::<u64>());
    // Two card files and the registry: every listed entry counts against M39c.
    assert_eq!(usage.entries, 3);
    assert_eq!(usage.cards, 4);
    assert_eq!(usage.cards, pack.cards().len() as u64);
    // The activities nest one list deeper than the items.
    assert_eq!(usage.depth.1, "activities.ron");
    assert_eq!(usage.dir_depth, 0);
}

/// Sub-directories count in M39c (one entry each) and give M39f.
#[test]
fn walk_measures_entries_and_directory_depth() {
    let root = nested_pack("walk", 3);
    let (_, usage) = measure_content_dir(&root, &Limits::default()).unwrap();
    // ids.lock and d0 at the root, then d1, d2 and x.ron.
    assert_eq!(usage.entries, 5);
    assert_eq!(usage.dir_depth, 3);
    assert_eq!(usage.cards, 1);
    assert_eq!(
        usage.file_bytes,
        (ITEM.len() as u64, "d0/d1/d2/x.ron".to_owned())
    );
}

/// Over half of a bound: flagged, an annotation with `--github` only, and the check passes.
#[test]
fn over_half_of_a_bound_warns_without_failing() {
    // 5 levels of the 8 of M39f: 62.5 %.
    let root = nested_pack("deep", 5);
    let plain = stdout(&ts_data(&[], &root));
    let line = plain.lines().find(|l| l.starts_with("M39f")).unwrap();
    assert!(
        line.contains("62.5 %") && line.ends_with("OVER HALF"),
        "{plain}"
    );
    assert!(plain.contains("1 bound(s) over half used"), "{plain}");
    assert!(!plain.contains("::warning"), "{plain}");
    let github = stdout(&ts_data(&["--github"], &root));
    let notes: Vec<&str> = github.lines().filter(|l| l.starts_with("::")).collect();
    assert_eq!(notes.len(), 1, "{github}");
    assert!(
        notes[0].starts_with(
            "::warning title=M39f data bound::M39f sub-directory depth: 5 of 8 (62.5 %25)"
        ),
        "{github}"
    );
    // Exactly half (4 of 8) does not warn.
    let half = stdout(&ts_data(&["--github"], &nested_pack("half", 4)));
    assert!(
        !half.contains("OVER HALF") && !half.contains("::warning"),
        "{half}"
    );
}

#[test]
fn refusals_fail_the_check() {
    // Card missing from the registry (M40d).
    let root = nested_pack("unregistered", 0);
    fs::write(root.join("ids.lock"), "").unwrap();
    let o = ts_data(&[], &root);
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("DATA_UNREGISTERED"));
    // No registry.
    fs::remove_file(root.join("ids.lock")).unwrap();
    let o = ts_data(&[], &root);
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("ids.lock"));
    // Invalid card.
    let root = nested_pack("invalid", 0);
    fs::write(root.join("x.ron"), "[Item(id: \"item.x\")]").unwrap();
    let o = ts_data(&[], &root);
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("DATA_MISSING_FIELD"));
    // Bad arguments.
    let o = Command::new(env!("CARGO_BIN_EXE_ts-data"))
        .output()
        .unwrap();
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("usage: ts-data check"));
}
