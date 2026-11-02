//! Kernel items that a core or rule crate must not name outside the crates they are reserved
//! to (#26 m3, m4). Pure functions, no I/O.
//!
//! The source is normalized as in `exemptions` (comments removed, literals marked), so only
//! identifiers of code count. Test code is skipped: the `tests`, `benches` and `examples`
//! targets at the root of a crate, files named `tests.rs` or `*_tests.rs` (by convention
//! declared under `#[cfg(test)]`), and the body of an inline `#[cfg(test)] mod name { … }`.
//!
//! A guard against mistakes, not against a determined author, whom review catches: a
//! renamed dependency or a re-export inside an allowed crate, or `StateDecode` for
//! `StableId`, reach the same items without naming them.
//! spec: carnet architecture-securite.md §9.9 K5, K9

use crate::exemptions::{Src, normalize};

/// One reserved identifier, the path prefixes where code may name it, and why.
struct Reserved {
    ident: &'static str,
    allowed: &'static [&'static str],
    why: &'static str,
}

/// Closed list (decided, #26). `WorldKey` and `from_snapshot_raw` cover their constructors.
const RESERVED: [Reserved; 4] = [
    Reserved {
        ident: "WorldKey",
        allowed: &["crates/ts_kernel/", "crates/ts_toy/"],
        why: "a rule never builds a world key; hosts, the kernel and test crates only (#26 m3)",
    },
    Reserved {
        ident: "from_snapshot_raw",
        allowed: &["crates/ts_kernel/", "crates/ts_persist/"],
        why: "reserved to the snapshot codec (K9, #26 m3); use the allocator or StableId::map",
    },
    Reserved {
        ident: "chacha20",
        allowed: &["crates/ts_kernel/src/rng/"],
        why: "raw primitive of the keyed randomness: draw through a DrawSite (#26 m4)",
    },
    Reserved {
        ident: "blake3",
        allowed: &[
            "crates/ts_kernel/src/rng/",
            "crates/ts_kernel/src/state.rs",
            "crates/ts_kernel/src/plugin.rs",
            "crates/ts_data/",
            "crates/ts_persist/",
            "crates/ts_toy/",
        ],
        why: "raw primitive: draw through a DrawSite, hash through StateHash (#26 m4)",
    },
];

/// Crates that may depend on a raw primitive (normal or build dependency; tests may use any),
/// by package name.
const PRIMITIVE_DEPENDENCIES: [(&str, &[&str]); 2] = [
    ("chacha20", &["ts_kernel"]),
    ("blake3", &["ts_kernel", "ts_data", "ts_persist", "ts_toy"]),
];

/// Why a core crate `krate` may not depend on package `dependency`, if it may not.
pub fn refused_dependency(krate: &str, dependency: &str) -> Option<&'static str> {
    PRIMITIVE_DEPENDENCIES
        .iter()
        .find(|(name, _)| *name == dependency)
        .filter(|(_, allowed)| !allowed.contains(&krate))
        .map(|_| "raw primitive of the keyed randomness or of the state hash (#26 m4)")
}

/// True for a file of test code, by its path: a test target at the root of a crate
/// (`crates/<crate>/tests/…`), or a test module file.
fn is_test_file(rel: &str) -> bool {
    let parts: Vec<&str> = rel.split('/').collect();
    let name = parts.last().copied().unwrap_or_default();
    matches!(parts.get(2), Some(&("tests" | "benches" | "examples"))) && parts.len() > 3
        || name == "tests.rs"
        || name.ends_with("_tests.rs")
}

/// Index of the `close` that matches the `open` at `at`, outside literals.
fn matching(src: &Src, at: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0_usize;
    for k in at..src.text.len() {
        if src.string[k] {
            continue;
        }
        if src.text[k] == open {
            depth += 1;
        } else if src.text[k] == close {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(k);
            }
        }
    }
    None
}

/// Ranges `[start, end]` of the inline `#[cfg(test)] mod name { … }` blocks, with any other
/// attribute and visibility between the two. Any other form stays checked.
fn test_blocks(src: &Src) -> Vec<(usize, usize)> {
    let pattern: Vec<char> = "#[cfg(test)]".chars().collect();
    let tokens = src.tokens();
    let token_at = |k: usize| tokens.iter().find(|t| t.0 == k).map(|&t| (t, src.word(t)));
    let mut blocks = Vec::new();
    let mut k = 0;
    while k + pattern.len() <= src.text.len() {
        if src.text[k..k + pattern.len()] != pattern[..] || src.string[k] {
            k += 1;
            continue;
        }
        let mut p = k + pattern.len();
        while src.text.get(p) == Some(&'#') && src.text.get(p + 1) == Some(&'[') {
            match matching(src, p + 1, '[', ']') {
                Some(end) => p = end + 1,
                None => break,
            }
        }
        if let Some(((_, end), word)) = token_at(p)
            && word == "pub"
        {
            p = end;
            if src.text.get(p) == Some(&'(') {
                p = matching(src, p, '(', ')').map_or(p, |e| e + 1);
            }
        }
        let block = token_at(p)
            .filter(|(_, w)| w == "mod")
            .and_then(|((_, end), _)| token_at(end))
            .filter(|&((_, end), _)| src.text.get(end) == Some(&'{'))
            .and_then(|((_, end), _)| matching(src, end, '{', '}'));
        if let Some(close) = block {
            blocks.push((k, close));
            k = close;
        }
        k += 1;
    }
    blocks
}

/// Reserved identifiers named by non-test code of file `rel` (relative to the root, with
/// `/`): `(line, identifier, why)`.
pub fn reserved_uses(rel: &str, source: &str) -> Vec<(usize, &'static str, &'static str)> {
    if is_test_file(rel) {
        return Vec::new();
    }
    let checked: Vec<&Reserved> = RESERVED
        .iter()
        .filter(|r| !r.allowed.iter().any(|a| rel.starts_with(a)))
        .collect();
    if checked.is_empty() {
        return Vec::new();
    }
    let src = normalize(source);
    let blocks = test_blocks(&src);
    let mut found = Vec::new();
    for t in src.tokens() {
        if blocks.iter().any(|&(a, b)| a <= t.0 && t.0 <= b) {
            continue;
        }
        let word = src.word(t);
        if let Some(r) = checked.iter().find(|r| r.ident == word) {
            found.push((src.line[t.0], r.ident, r.why));
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idents(rel: &str, source: &str) -> Vec<(usize, &'static str)> {
        reserved_uses(rel, source)
            .into_iter()
            .map(|(l, i, _)| (l, i))
            .collect()
    }

    #[test]
    fn rule_crates_cannot_name_reserved_items() {
        let rule = "crates/ts_model/src/lib.rs";
        assert_eq!(
            idents(rule, "use ts_kernel::rng::WorldKey;"),
            [(1, "WorldKey")]
        );
        assert_eq!(
            idents(
                rule,
                "fn a() {}\nlet _ = ts_kernel::StableId::from_snapshot_raw(3);"
            ),
            [(2, "from_snapshot_raw")]
        );
        assert_eq!(
            idents(
                rule,
                "use chacha20::XChaCha8;\nlet h = ::blake3::hash(b\"x\");"
            ),
            [(1, "chacha20"), (2, "blake3")]
        );
        assert_eq!(
            idents(rule, "use ts_kernel::rng::r#WorldKey as K;"),
            [(1, "WorldKey")]
        );
    }

    #[test]
    fn comments_strings_and_other_words_do_not_count() {
        let rule = "crates/ts_game/src/lib.rs";
        assert!(idents(rule, "// a WorldKey\n/// from_snapshot_raw\nfn a() {}").is_empty());
        assert!(idents(rule, "const S: &str = \"WorldKey blake3\";").is_empty());
        assert!(idents(rule, "struct MyWorldKey; fn from_snapshot_raw_like() {}").is_empty());
    }

    #[test]
    fn allowed_crates_and_files() {
        let src = "use ts_kernel::rng::WorldKey;\nuse blake3::Hasher;";
        assert!(idents("crates/ts_toy/src/lib.rs", src).is_empty());
        assert_eq!(
            idents("crates/ts_persist/src/kernel.rs", src),
            [(1, "WorldKey")]
        );
        assert!(idents("crates/ts_kernel/src/state.rs", "use blake3::Hasher;").is_empty());
        assert_eq!(
            idents(
                "crates/ts_kernel/src/sim.rs",
                "use chacha20::X; use blake3::H;"
            ),
            [(1, "chacha20"), (1, "blake3")]
        );
        assert!(idents("crates/ts_kernel/src/rng/mod.rs", "use chacha20::X;").is_empty());
    }

    #[test]
    fn test_code_is_skipped() {
        let src = "use ts_kernel::rng::WorldKey;";
        for rel in [
            "crates/ts_model/tests/a.rs",
            "crates/ts_model/tests/common/mod.rs",
            "crates/ts_model/benches/b.rs",
            "crates/ts_model/src/tests.rs",
            "crates/ts_model/src/sim/snapshot_tests.rs",
        ] {
            assert!(idents(rel, src).is_empty(), "{rel}");
        }
        // Only the test targets at the root of a crate.
        assert_eq!(
            idents("crates/ts_model/src/examples/a.rs", src),
            [(1, "WorldKey")]
        );
        let rule = "crates/ts_model/src/player.rs";
        let inline = "fn a() {}\n#[cfg(test)]\nmod tests {\n    use ts_kernel::rng::WorldKey;\n    fn b() { let _ = { 1 }; }\n}\nuse ts_kernel::rng::WorldKey;";
        assert_eq!(idents(rule, inline), [(7, "WorldKey")]);
        let attrs = "#[cfg(test)]\n#[allow(dead_code)]\npub(crate) mod t { use x::WorldKey; }";
        assert!(idents(rule, attrs).is_empty());
        // Only a whole module is skipped: a single item under cfg(test) stays checked.
        assert_eq!(
            idents(rule, "#[cfg(test)]\nuse ts_kernel::rng::WorldKey;"),
            [(2, "WorldKey")]
        );
        assert_eq!(
            idents(
                rule,
                "#[cfg(test)]\nmod tests;\nuse ts_kernel::rng::WorldKey;"
            ),
            [(3, "WorldKey")]
        );
        // A brace in a string does not end the block early.
        let braces =
            "#[cfg(test)]\nmod t { const S: &str = \"}\"; use a::WorldKey; }\nuse b::WorldKey;";
        assert_eq!(idents(rule, braces), [(3, "WorldKey")]);
    }

    /// The `bevy_ecs` paths of the restore-contract bans in `clippy.toml` were checked against
    /// this version, and `allow-invalid` hides a path that no longer resolves: on a bump,
    /// check each one again (a throwaway file that uses it must be refused), then update this.
    #[test]
    fn clippy_bans_were_checked_against_this_bevy_ecs() {
        let manifest = include_str!("../../../Cargo.toml");
        assert!(
            manifest.contains("bevy_ecs = { version = \"=0.20.0\""),
            "bevy_ecs changed: check the bevy_ecs paths of clippy.toml again (#31)"
        );
        let clippy = include_str!("../../../clippy.toml");
        assert!(clippy.contains("bevy_ecs::system::Local"));
    }

    #[test]
    fn primitive_dependencies() {
        assert_eq!(refused_dependency("ts_kernel", "chacha20"), None);
        assert!(refused_dependency("ts_persist", "chacha20").is_some());
        assert_eq!(refused_dependency("ts_data", "blake3"), None);
        assert!(refused_dependency("ts_model", "blake3").is_some());
        assert_eq!(refused_dependency("ts_model", "serde"), None);
    }
}
