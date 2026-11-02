//! Closed list of the `macro_rules!` of the core and rules crates (AS-56). Pure functions.
//!
//! A macro is the only way to assemble an attribute from pieces, which `clippy-exemptions` cannot
//! follow in general. Rather than chase every form, the definitions are listed: a crate that
//! defines another macro fails the check, so that every new macro goes through the review of the
//! architecture and security lead, who is the owner of this list. A net against error, like
//! `reserved.rs`.
//!
//! spec: carnet architecture-securite.md AS-56

use crate::exemptions::normalize;

/// One listed definition: crate, file relative to the crate directory, macro name, and how many
/// times the file defines it (`not_public` has a `public` and a non-`public` form).
pub struct Listed {
    pub krate: &'static str,
    pub file: &'static str,
    pub name: &'static str,
    pub count: usize,
}

const fn listed(
    krate: &'static str,
    file: &'static str,
    name: &'static str,
    count: usize,
) -> Listed {
    Listed {
        krate,
        file,
        name,
        count,
    }
}

/// The definitions allowed in the core and rules crates. Adding one is a change of a sensitive
/// path, reviewed by the architecture and security lead.
pub const ALLOWED: &[Listed] = &[
    listed("ts_kernel", "src/build_profile.rs", "not_public", 2),
    listed("ts_kernel", "src/canonical.rs", "state_decode_int", 1),
    listed("ts_kernel", "src/lib.rs", "assert_size", 1),
    listed("ts_kernel", "src/state.rs", "state_hash_int", 1),
    listed("ts_persist", "src/schema.rs", "leaf", 1),
    listed("ts_persist", "src/schema.rs", "flat_schema", 1),
];

/// Line and name of every `macro_rules! name` of a source, comments and literals excluded.
pub fn definitions(source: &str) -> Vec<(usize, String)> {
    let src = normalize(source);
    let mut out = Vec::new();
    for tok in src.tokens() {
        let bang = src.text.get(tok.1) == Some(&'!') && !src.string[tok.1];
        if !bang || src.word(tok) != "macro_rules" {
            continue;
        }
        let name = src
            .tokens()
            .into_iter()
            .find(|t| t.0 == tok.1 + 1)
            .map(|t| src.word(t));
        // A name that cannot be read is listed under an empty name, which no entry matches.
        out.push((src.line[tok.0], name.unwrap_or_default()));
    }
    out
}

/// Errors for one crate: `found` is `(file relative to the crate, line, name)` for every
/// definition of its sources. A definition missing from the list, a count that differs, and an
/// entry with no definition left are all errors.
pub fn errors(krate: &str, found: &[(String, usize, String)]) -> Vec<String> {
    let mut errors = Vec::new();
    let entries = || ALLOWED.iter().filter(|e| e.krate == krate);
    for (file, line, name) in found {
        if !entries().any(|e| e.file == file && e.name == name) {
            errors.push(format!(
                "{krate}/{file}:{line}: macro_rules! {name} is not in the closed list \
                 (tools/xtask/src/macros.rs, AS-56): a new macro is reviewed by the architecture \
                 and security lead"
            ));
        }
    }
    for e in entries() {
        let n = found
            .iter()
            .filter(|(f, _, name)| f == e.file && name == e.name)
            .count();
        if n != e.count {
            errors.push(format!(
                "{krate}/{}: macro_rules! {} is defined {n} time(s), the closed list says {} \
                 (tools/xtask/src/macros.rs, AS-56)",
                e.file, e.name, e.count
            ));
        }
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(items: &[(&str, usize, &str)]) -> Vec<(String, usize, String)> {
        items
            .iter()
            .map(|(f, l, n)| ((*f).to_owned(), *l, (*n).to_owned()))
            .collect()
    }

    #[test]
    fn finds_definitions_not_mentions() {
        let src = "// macro_rules! in_comment { () => {} }\n\
                   const S: &str = \"macro_rules! in_string\";\n\
                   macro_rules! one { () => {} }\n\
                   /* macro_rules! hidden { () => {} } */\n\
                   #[macro_export]\n\
                   macro_rules!two{()=>{}}\n\
                   macro_rules! r#three { () => {} }\n\
                   fn macro_rules() {}\n";
        assert_eq!(
            definitions(src),
            [
                (3, "one".to_owned()),
                (6, "two".to_owned()),
                (7, "three".to_owned())
            ]
        );
        // A name that cannot be read is still a definition, under an empty name.
        assert_eq!(definitions("macro_rules ! ( ) "), [(1, String::new())]);
    }

    #[test]
    fn the_list_matches_a_listed_crate() {
        let all = found(&[
            ("src/build_profile.rs", 10, "not_public"),
            ("src/build_profile.rs", 20, "not_public"),
            ("src/canonical.rs", 3, "state_decode_int"),
            ("src/lib.rs", 4, "assert_size"),
            ("src/state.rs", 5, "state_hash_int"),
        ]);
        assert!(errors("ts_kernel", &all).is_empty());
        // Another crate has its own entries, and a crate without any has none.
        assert!(errors("ts_toy", &[]).is_empty());
        assert_eq!(errors("ts_persist", &[]).len(), 2);
    }

    #[test]
    fn refuses_what_is_not_listed() {
        let ok = [
            ("src/build_profile.rs", 10, "not_public"),
            ("src/build_profile.rs", 20, "not_public"),
            ("src/canonical.rs", 3, "state_decode_int"),
            ("src/lib.rs", 4, "assert_size"),
            ("src/state.rs", 5, "state_hash_int"),
        ];
        let with = |extra: (&str, usize, &str)| {
            let mut v = ok.to_vec();
            v.push(extra);
            errors("ts_kernel", &found(&v))
        };
        // A new macro, a known name in another file, a name the crate does not list, an
        // unreadable name.
        for extra in [
            ("src/new.rs", 1, "fresh"),
            ("src/sim.rs", 1, "assert_size"),
            ("src/lib.rs", 9, "leaf"),
            ("src/lib.rs", 9, ""),
        ] {
            let e = with(extra);
            assert!(
                e.iter().any(|m| m.contains("not in the closed list")),
                "{extra:?}: {e:?}"
            );
        }
        // A third definition of a listed macro in its file, a missing one, and a stale entry.
        let e = with(("src/build_profile.rs", 30, "not_public"));
        assert!(e.iter().any(|m| m.contains("defined 3 time(s)")), "{e:?}");
        let e = errors("ts_kernel", &found(&ok[1..]));
        assert!(e.iter().any(|m| m.contains("defined 1 time(s)")), "{e:?}");
        let e = errors("ts_kernel", &found(&ok[2..]));
        assert_eq!(e.len(), 1, "{e:?}");
    }

    /// The files that define a listed macro are sensitive paths: a change to one of them needs the
    /// review of the architecture and security lead, like the list itself.
    #[test]
    fn every_listed_file_is_a_sensitive_path() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../ci/sensitive-paths.txt");
        let rules = crate::sensitive::Rules::parse(&crate::sys::read_text(&path).unwrap()).unwrap();
        for e in ALLOWED {
            let file = format!("crates/{}/{}", e.krate, e.file);
            assert!(
                rules.path_hit(&file),
                "{file} is not in tools/ci/sensitive-paths.txt"
            );
        }
    }
}
