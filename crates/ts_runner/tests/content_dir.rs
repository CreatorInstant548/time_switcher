//! Reads real directories: the example cards, bounds, symbolic links, location independence.
#![allow(clippy::disallowed_methods, clippy::unwrap_used)] // test: throwaway directories.

use std::fs;
use std::path::{Path, PathBuf};

use ts_data::{Code, Limits};
use ts_runner::load_content_dir;

const EXAMPLES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/content");
/// Reference vector: fingerprint of the example cards in `data/content/`. Changing the cards or
/// the canonical encoding changes it on purpose, with the review that goes with it.
const EXAMPLES_FINGERPRINT: &str =
    "4f40e7f18ab6d727a2e1d3f15e9e05c445bfd403aae78d5607224245a9c7aad4";

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("content_dir")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn copy_examples(to: &Path, crlf: bool) {
    for entry in fs::read_dir(EXAMPLES).unwrap() {
        let entry = entry.unwrap();
        let mut text = fs::read_to_string(entry.path())
            .unwrap()
            .replace("\r\n", "\n");
        if crlf {
            text = text.replace('\n', "\r\n");
        }
        fs::write(to.join(entry.file_name()), text).unwrap();
    }
}

#[test]
fn example_cards_reference_vector() {
    let pack = load_content_dir(Path::new(EXAMPLES), &Limits::default()).unwrap();
    assert_eq!(pack.cards().len(), 4);
    assert_eq!(pack.fingerprint().to_string(), EXAMPLES_FINGERPRINT);
}

#[test]
fn fingerprint_does_not_depend_on_the_folder_or_line_endings() {
    let reference = load_content_dir(Path::new(EXAMPLES), &Limits::default()).unwrap();
    let lf = scratch("lf_folder_one");
    let crlf = scratch("crlf_folder_two");
    copy_examples(&lf, false);
    copy_examples(&crlf, true);
    for dir in [&lf, &crlf] {
        let pack = load_content_dir(dir, &Limits::default()).unwrap();
        assert_eq!(pack.fingerprint(), reference.fingerprint());
    }
}

#[test]
fn oversized_file_is_refused_and_the_error_has_no_local_path() {
    let dir = scratch("oversized");
    copy_examples(&dir, false);
    let limits = Limits {
        max_file_bytes: 16,
        ..Limits::default()
    };
    let errors = load_content_dir(&dir, &limits).unwrap_err();
    assert_eq!(errors.diagnostics()[0].code, Code::FileTooLarge);
    let text = errors.to_string();
    assert!(text.contains("activities.ron"), "{text}");
    assert!(!text.contains("content_dir"), "{text}");
    assert!(!text.contains(env!("CARGO_TARGET_TMPDIR")), "{text}");
}

#[test]
fn too_many_entries_are_refused() {
    let dir = scratch("too_many");
    copy_examples(&dir, false);
    let limits = Limits {
        max_files: 1,
        ..Limits::default()
    };
    let errors = load_content_dir(&dir, &limits).unwrap_err();
    assert_eq!(errors.diagnostics()[0].code, Code::TooManyFiles);
}

#[test]
fn sub_directories_are_read_and_other_files_ignored() {
    let dir = scratch("nested");
    fs::create_dir_all(dir.join("a/b")).unwrap();
    fs::write(dir.join("a/b/x.ron"), "[Item(id: \"item.x\", name: \"X\")]").unwrap();
    fs::write(dir.join("notes.txt"), "not a card").unwrap();
    let pack = load_content_dir(&dir, &Limits::default()).unwrap();
    assert_eq!(pack.cards().len(), 1);
    fs::write(dir.join("a/b/y.ron"), "[").unwrap();
    let errors = load_content_dir(&dir, &Limits::default()).unwrap_err();
    assert_eq!(errors.diagnostics()[0].file, "a/b/y.ron");
}

#[test]
fn directories_nested_too_deep_are_refused() {
    let dir = scratch("deep");
    fs::create_dir_all(dir.join("1/2/3/4/5/6/7/8/9")).unwrap();
    let errors = load_content_dir(&dir, &Limits::default()).unwrap_err();
    assert_eq!(errors.diagnostics()[0].code, Code::TooDeep);
}

#[test]
fn missing_directory_is_a_typed_error() {
    let errors = load_content_dir(&scratch("none").join("absent"), &Limits::default()).unwrap_err();
    assert_eq!(errors.diagnostics()[0].code, Code::ReadFailed);
}

/// Creates a symbolic link; `false` when the platform refuses (Windows without the privilege).
fn symlink(target: &Path, link: &Path) -> bool {
    #[cfg(unix)]
    let made = std::os::unix::fs::symlink(target, link);
    #[cfg(windows)]
    let made = if target.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    };
    made.is_ok()
}

#[test]
fn symbolic_links_are_refused() {
    let outside = scratch("link_outside");
    fs::write(
        outside.join("secret.ron"),
        "[Item(id: \"item.s\", name: \"S\")]",
    )
    .unwrap();
    // A link to a file, a link to a directory, and a link as the root.
    let dir = scratch("link_file");
    if !symlink(&outside.join("secret.ron"), &dir.join("l.ron")) {
        // Only Windows without the privilege may skip (the junction test covers it there);
        // anywhere else the link must be creatable.
        if !cfg!(windows) {
            panic!("cannot create a symbolic link");
        }
        return;
    }
    assert_eq!(
        load_content_dir(&dir, &Limits::default())
            .unwrap_err()
            .diagnostics()[0]
            .code,
        Code::SymlinkRefused
    );
    let dir = scratch("link_dir");
    assert!(symlink(&outside, &dir.join("d")));
    assert_eq!(
        load_content_dir(&dir, &Limits::default())
            .unwrap_err()
            .diagnostics()[0]
            .code,
        Code::SymlinkRefused
    );
    let root = scratch("link_root").join("root");
    assert!(symlink(&outside, &root));
    assert_eq!(
        load_content_dir(&root, &Limits::default())
            .unwrap_err()
            .diagnostics()[0]
            .code,
        Code::SymlinkRefused
    );
}

#[test]
fn extension_in_any_case_and_total_size() {
    let dir = scratch("upper");
    fs::write(dir.join("X.RON"), "[Item(id: \"item.x\", name: \"X\")]").unwrap();
    assert_eq!(
        load_content_dir(&dir, &Limits::default())
            .unwrap()
            .cards()
            .len(),
        1
    );
    let limits = Limits {
        max_total_bytes: 8,
        ..Limits::default()
    };
    let errors = load_content_dir(&dir, &limits).unwrap_err();
    assert_eq!(errors.diagnostics()[0].code, Code::TotalTooLarge);
}

#[test]
fn directory_depth_bound_is_a_limit() {
    let dir = scratch("depth_limit");
    fs::create_dir_all(dir.join("a/b")).unwrap();
    fs::write(dir.join("a/b/x.ron"), "[Item(id: \"item.x\", name: \"X\")]").unwrap();
    let shallow = Limits {
        max_dir_depth: 1,
        ..Limits::default()
    };
    assert_eq!(
        load_content_dir(&dir, &shallow).unwrap_err().diagnostics()[0].code,
        Code::TooDeep
    );
    let enough = Limits {
        max_dir_depth: 2,
        ..Limits::default()
    };
    assert_eq!(load_content_dir(&dir, &enough).unwrap().cards().len(), 1);
}

#[test]
fn entry_bound_is_checked_while_listing() {
    let dir = scratch("listing");
    for n in 0..20 {
        fs::write(dir.join(format!("f{n}.txt")), "x").unwrap();
    }
    let limits = Limits {
        max_files: 5,
        ..Limits::default()
    };
    assert_eq!(
        load_content_dir(&dir, &limits).unwrap_err().diagnostics()[0].code,
        Code::TooManyFiles
    );
}

/// A junction needs no privilege on Windows: the refusal is tested there even where symbolic
/// links cannot be created.
#[cfg(windows)]
#[test]
fn junctions_are_refused() {
    let outside = scratch("junction_outside");
    fs::write(
        outside.join("secret.ron"),
        "[Item(id: \"item.s\", name: \"S\")]",
    )
    .unwrap();
    let dir = scratch("junction_dir");
    let link = dir.join("j");
    let made = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(&link)
        .arg(&outside)
        .output()
        .unwrap();
    assert!(made.status.success(), "mklink /J failed");
    let first = |p: &Path| {
        load_content_dir(p, &Limits::default())
            .unwrap_err()
            .diagnostics()[0]
            .code
    };
    assert_eq!(first(&dir), Code::SymlinkRefused);
    assert_eq!(first(&link), Code::SymlinkRefused);
    // A junction named as a content file, in place of a card file (#48): refused when the
    // directory is listed. A swap during the read is tested by the unit tests of the reader.
    let dir = scratch("junction_as_file");
    let made = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(dir.join("cards.ron"))
        .arg(&outside)
        .output()
        .unwrap();
    assert!(made.status.success(), "mklink /J failed");
    assert_eq!(first(&dir), Code::SymlinkRefused);
}
