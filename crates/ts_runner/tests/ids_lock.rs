// spec: attendus-mesurables.md §10.2 (M40d)
//! The identifier registry `data/content/ids.lock`, checked as `ts-data check` will (decided,
//! A15: the check is born with the registry, in phase 0; this test runs in every CI run):
//! - against the example pack in `data/content/`: every card identifier listed and active,
//!   every active identifier with a card;
//! - against its version on `origin/main`: append-only. CI checks out with `fetch-depth: 0`,
//!   so `origin/main` is there; a branch behind `main` must be updated first anyway.
#![allow(clippy::disallowed_methods, clippy::unwrap_used)] // test: reads the repository.

use std::fs;
use std::path::Path;
use std::process::Command;

use ts_data::{Limits, REGISTRY_FILE, Registry};
use ts_runner::load_content_dir;

const CONTENT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/content");
const REPO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const BASE: &str = "origin/main";

fn registry() -> Registry {
    let bytes = fs::read(Path::new(CONTENT).join(REGISTRY_FILE)).unwrap();
    Registry::parse(&bytes).unwrap_or_else(|e| panic!("{e}"))
}

fn git(args: &[&str]) -> std::process::Output {
    Command::new("git")
        .arg("-C")
        .arg(REPO)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn registry_lists_exactly_the_cards_of_the_pack() {
    let pack = load_content_dir(Path::new(CONTENT), &Limits::default()).unwrap();
    registry()
        .check_pack(&pack)
        .unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn registry_only_appends_to_its_version_on_main() {
    let base = format!("{BASE}^{{commit}}");
    let known = git(&["rev-parse", "--verify", "--quiet", &base]);
    assert!(
        known.status.success(),
        "{BASE} is missing: fetch it (CI checks out with fetch-depth: 0)"
    );
    let spec = format!("{BASE}:data/content/{REGISTRY_FILE}");
    // Absent on main: the registry is being created, anything is an append.
    if !git(&["cat-file", "-e", &spec]).status.success() {
        return;
    }
    let shown = git(&["show", &spec]);
    assert!(shown.status.success(), "git show {spec} failed");
    let old = Registry::parse(&shown.stdout).unwrap_or_else(|e| panic!("{BASE}: {e}"));
    old.check_successor(&registry())
        .unwrap_or_else(|e| panic!("{e}"));
}
