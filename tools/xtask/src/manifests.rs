//! Contrôles des crates et des manifestes : `unsafe_code` (SEC0-19, F3), fonctionnalités et
//! `ts_toy` (SEC0-20, F4, D-DEV-02). Fonctions pures, sans E/S.

use std::collections::{BTreeMap, BTreeSet};

use crate::metadata::Metadata;

/// Fonctionnalités de développement de `ts_kernel`, interdites avec `public` (SEC0-20).
pub const DEV_FEATURES: [&str; 4] = [
    "dev-commands",
    "dev-introspection",
    "external-pilot",
    "dev-bench",
];
/// Crates d'hôte : aucune dépendance normale n'y active une fonctionnalité de développement.
pub const HOSTS: [&str; 3] = ["ts_host", "ts_server", "ts_client_native"];
/// Crates d'outil : seules à pouvoir dépendre de `ts_toy` et, avec les hôtes, à s'exempter
/// des interdictions de clippy (E6). Toute autre crate est traitée comme cœur ou règles.
pub const TOOLS: [&str; 4] = ["ts_py", "ts_runner", "ts_viz", "xtask"];
/// Cibles qui sont des racines de crate (F3).
const ROOT_KINDS: [&str; 8] = [
    "lib",
    "rlib",
    "dylib",
    "cdylib",
    "staticlib",
    "proc-macro",
    "bin",
    "custom-build",
];

/// The only crate with local `[lints.*]` tables and `#![deny(unsafe_code)]` (SEC0-19, AS-18).
/// No other exception is anticipated, `ts_client_native` included (PY1).
pub const LOCAL_LINTS_CRATE: &str = "ts_py";

/// Crate roots to check: (crate, source file), every crate of the workspace (PY1).
pub fn crate_roots(meta: &Metadata) -> Vec<(&str, &str)> {
    let mut roots = Vec::new();
    for p in &meta.packages {
        for t in &p.targets {
            if t.kind.iter().any(|k| ROOT_KINDS.contains(&k.as_str())) {
                roots.push((p.name.as_str(), t.src_path.as_str()));
            }
        }
    }
    roots
}

/// `#![forbid(unsafe_code)]` partout ; `ts_py` en `deny` (SEC0-19).
pub fn check_unsafe_root(crate_name: &str, source: &str) -> Result<(), String> {
    let expected = if crate_name == LOCAL_LINTS_CRATE {
        "#![deny(unsafe_code)]"
    } else {
        "#![forbid(unsafe_code)]"
    };
    match source.lines().any(|l| l.trim() == expected) {
        true => Ok(()),
        false => Err(format!("{expected} absent")),
    }
}

/// Première ligne d'un `allow` ou `expect` qui nomme `unsafe_code` sans `// SAFETY:` juste avant
/// (exception de `ts_py`). Les blancs sont ignorés.
pub fn unsafe_allow_without_safety(source: &str) -> Option<usize> {
    let lines: Vec<String> = source
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();
    lines.iter().enumerate().find_map(|(i, line)| {
        let exempts = ["allow(", "expect("].iter().any(|w| line.contains(w));
        let commented = lines[..i].iter().rev().find(|l| !l.is_empty());
        let safety = commented.is_some_and(|l| l.starts_with("//SAFETY:"));
        (exempts && line.contains("unsafe_code") && !safety).then_some(i + 1)
    })
}

/// Chaque membre hérite des lints de l'espace de travail, sans `[lints.*]` local (F3).
pub fn check_lints_section(manifest: &str) -> Result<(), String> {
    let mut section = String::new();
    let mut inherits = false;
    for line in manifest
        .lines()
        .map(|l| l.split('#').next().unwrap_or_default().trim())
    {
        // Quotes dropped too: `["lints".rust]` is `[lints.rust]` for Cargo (PY1).
        let compact: String = line
            .chars()
            .filter(|c| !c.is_whitespace() && !matches!(c, '"' | '\''))
            .collect();
        if let Some(name) = compact.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            section = name.to_owned();
            if section.starts_with("lints.") {
                return Err(format!("section [{section}] locale interdite"));
            }
        } else if (section == "lints" && compact == "workspace=true")
            || (section.is_empty() && compact == "lints.workspace=true")
        {
            inherits = true;
        } else if (section.is_empty() && compact.starts_with("lints"))
            || (section == "lints" && !compact.is_empty())
        {
            return Err(format!("réglage de lints local interdit : {line}"));
        }
    }
    if inherits {
        Ok(())
    } else {
        Err("[lints] workspace = true absent".to_owned())
    }
}

/// Lint tables of a manifest, `tool -> (key -> value)`, for the tables named `<prefix><tool>`
/// (`[lints.rust]`, `[workspace.lints.clippy]`). Lines are compared without blanks or
/// comments; table names and keys also without quotes (`["lints".rust]` is `[lints.rust]`).
/// Fails closed: an array of tables, a line without `=`, a duplicate key, or a key outside
/// these tables that names the lints (`[lints]` with content, dotted root keys) is an error.
fn lint_tables(manifest: &str, prefix: &str) -> Result<BTreeMap<String, Lints>, String> {
    let bare = prefix.trim_end_matches('.');
    let unquote = |s: &str| -> String { s.chars().filter(|c| !matches!(c, '"' | '\'')).collect() };
    let mut tables: BTreeMap<String, Lints> = BTreeMap::new();
    let mut section = String::new();
    for line in manifest
        .lines()
        .map(|l| l.split('#').next().unwrap_or_default())
    {
        let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        if compact.starts_with("[[") && unquote(&compact).contains("lints") {
            return Err(format!("array of tables {compact} refused"));
        }
        if let Some(name) = compact.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            section = unquote(name);
            if let Some(tool) = section.strip_prefix(prefix)
                && tables.insert(tool.to_owned(), Lints::new()).is_some()
            {
                return Err(format!("table [{section}] declared twice"));
            }
            continue;
        }
        if compact.is_empty() {
            continue;
        }
        let in_table = section.strip_prefix(prefix).and_then(|t| tables.get_mut(t));
        let key = compact.split_once('=').map(|(k, v)| (unquote(k), v));
        match in_table {
            Some(entries) => {
                let (key, value) =
                    key.ok_or_else(|| format!("unreadable line in [{section}]: {}", line.trim()))?;
                if entries.insert(key.clone(), value.to_owned()).is_some() {
                    return Err(format!("key {key} twice in [{section}]"));
                }
            }
            None if section == bare
                || (section.is_empty() && unquote(&compact).starts_with(bare)) =>
            {
                return Err(format!(
                    "lint setting outside a [{prefix}<tool>] table: {}",
                    line.trim()
                ));
            }
            None => {}
        }
    }
    Ok(tables)
}

/// Entries of one lint table: key -> value, without blanks.
type Lints = BTreeMap<String, String>;

/// F3 for [`LOCAL_LINTS_CRATE`] (AS-18, PY1): no `[lints] workspace = true`, but local
/// `[lints.<tool>]` tables **identical** to the `[workspace.lints.<tool>]` tables of the root
/// manifest, except `unsafe_code`, `"forbid"` there and `"deny"` here.
pub fn check_local_lints(manifest: &str, root_manifest: &str) -> Result<(), String> {
    let mut expected = lint_tables(root_manifest, "workspace.lints.")
        .map_err(|e| format!("root Cargo.toml: {e}"))?;
    let rust = expected
        .get_mut("rust")
        .and_then(|t| t.get_mut("unsafe_code"))
        .filter(|v| v.as_str() == "\"forbid\"")
        .ok_or("root Cargo.toml: [workspace.lints.rust] without unsafe_code = \"forbid\"")?;
    *rust = "\"deny\"".to_owned();
    let actual = lint_tables(manifest, "lints.")?;
    if actual == expected {
        return Ok(());
    }
    let tools: BTreeSet<&String> = actual.keys().chain(expected.keys()).collect();
    let differ: Vec<String> = tools
        .into_iter()
        .filter(|t| actual.get(*t) != expected.get(*t))
        .map(|t| format!("[lints.{t}]"))
        .collect();
    Err(format!(
        "{} differ from [workspace.lints.*] (only unsafe_code = \"deny\" may change)",
        differ.join(", ")
    ))
}

/// E7, AS-23 point 4 : aucune section `[patch]` ni `[replace]`, ni clé de racine `patch.` ou
/// `replace.` (forme pointée). Rend les lignes fautives.
pub fn patch_sections(manifest: &str) -> Vec<String> {
    let mut root = true;
    let mut out = Vec::new();
    for line in manifest.lines() {
        let squashed: String = line
            .split('#')
            .next()
            .unwrap_or_default()
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '"' && *c != '\'')
            .collect();
        let header = squashed.trim_start_matches('[');
        if squashed.starts_with('[') {
            root = false;
            if header.starts_with("patch") || header.starts_with("replace") {
                out.push(line.trim().to_owned());
            }
        } else if root && (squashed.starts_with("patch") || squashed.starts_with("replace")) {
            // Root keys: dotted (`patch.crates-io…`) or inline table (`patch = { … }`).
            out.push(line.trim().to_owned());
        }
    }
    out
}

/// Fonctionnalités dans les manifestes (F4) et règle D-DEV-02. Rend la liste des écarts.
pub fn check_features(meta: &Metadata) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(kernel) = meta.package("ts_kernel") else {
        return vec!["ts_kernel absent de l'espace de travail".to_owned()];
    };
    if kernel
        .features
        .get("default")
        .is_some_and(|d| !d.is_empty())
    {
        errors.push("ts_kernel : la fonctionnalité default doit être vide".to_owned());
    }
    for p in &meta.packages {
        let is_kernel = p.name == "ts_kernel";
        if !is_kernel && p.features.contains_key("public") {
            errors.push(format!(
                "{} : une fonctionnalité public est réservée à ts_kernel",
                p.name
            ));
        }
        for (feature, items) in &p.features {
            for item in items {
                if item.ends_with("/public") || (is_kernel && item == "public") {
                    errors.push(format!(
                        "{} : la fonctionnalité {feature} active {item}",
                        p.name
                    ));
                }
                let dev = DEV_FEATURES
                    .iter()
                    .any(|f| item.ends_with(&format!("/{f}")));
                if HOSTS.contains(&p.name.as_str()) && dev {
                    errors.push(format!("hôte {} : {feature} active {item}", p.name));
                }
            }
        }
        for d in p.dependencies.iter().filter(|d| d.name == "ts_kernel") {
            if d.features.iter().any(|f| f == "public") {
                errors.push(format!(
                    "{} : sa dépendance ts_kernel active public",
                    p.name
                ));
            }
            let dev = d
                .features
                .iter()
                .any(|f| DEV_FEATURES.contains(&f.as_str()));
            if HOSTS.contains(&p.name.as_str()) && d.kind.is_none() && dev {
                errors.push(format!(
                    "hôte {} : dépendance normale de développement",
                    p.name
                ));
            }
        }
        let tool = TOOLS.contains(&p.name.as_str()) || p.name == "ts_toy";
        if !tool && meta.normal_closure(&p.name).contains("ts_toy") {
            errors.push(format!(
                "{} dépend de ts_toy en dépendance normale (D-DEV-02)",
                p.name
            ));
        }
    }
    errors
}

/// Lignes de `cargo tree -f '{p} {f}'` où une fonctionnalité de développement de ts_kernel est active.
pub fn tree_dev_features(tree: &str) -> Vec<String> {
    tree.lines()
        .filter(|l| l.contains("ts_kernel") && DEV_FEATURES.iter().any(|f| l.contains(f)))
        .map(|l| l.trim().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{parse, tests::sample};

    /// E7, AS-23 point 4.
    #[test]
    fn refuses_patch_and_replace() {
        let ok =
            "[package]\nname = \"a\"\n\n[dependencies]\npatchwork = \"1\"\nreplace-me = \"1\"\n";
        assert!(patch_sections(ok).is_empty());
        for bad in [
            "[patch.crates-io]\nserde = { path = \"x\" }\n",
            "[ patch . \"crates-io\" ]\n",
            "[replace]\n\"serde:1.0.0\" = { path = \"x\" }\n",
            "patch.crates-io.serde = { path = \"x\" }\n[package]\n",
            "replace = {}\n",
            "patch = { crates-io = { serde = { path = \"x\" } } }\n[package]\n",
        ] {
            assert_eq!(patch_sections(bad).len(), 1, "{bad}");
        }
    }

    #[test]
    fn unsafe_roots() {
        assert!(check_unsafe_root("ts_kernel", "//! x\n#![forbid(unsafe_code)]\n").is_ok());
        assert!(check_unsafe_root("ts_kernel", "#![deny(unsafe_code)]\n").is_err());
        assert!(check_unsafe_root("ts_kernel", "// #![forbid(unsafe_code)] plus tard\n").is_err());
        let py = "#![deny(unsafe_code)]\n// SAFETY: macro pyo3\n#[allow(unsafe_code)]\nfn a() {}\n";
        assert!(check_unsafe_root("ts_py", py).is_ok());
        assert!(check_unsafe_root("ts_py", "#![forbid(unsafe_code)]\n").is_err());
        assert_eq!(unsafe_allow_without_safety(py), None);
        let bad = [
            "#[allow(unsafe_code)]",
            "#[allow(dead_code, unsafe_code)]",
            "#[expect( unsafe_code )]",
        ];
        for line in bad {
            let text = format!("fn a() {{}}\n{line}\n");
            assert_eq!(unsafe_allow_without_safety(&text), Some(2), "{line}");
        }
        assert_eq!(crate_roots(&sample()).len(), 1);
    }

    #[test]
    fn lints_sections() {
        assert!(
            check_lints_section("[package]\nname = \"a\"\n\n[lints]\nworkspace = true\n").is_ok()
        );
        assert!(check_lints_section("lints.workspace = true\n[package]\n").is_ok());
        assert!(check_lints_section("[package]\nname = \"a\"\n").is_err());
        assert!(
            check_lints_section(
                "[lints]\nworkspace = true\n[lints.rust]\nunsafe_code = \"allow\"\n"
            )
            .is_err()
        );
        assert!(
            check_lints_section("[lints]\nworkspace = true\nrust = { unsafe_code = \"allow\" }\n")
                .is_err()
        );
        // PY1: a quoted name is the same table for Cargo.
        for bad in [
            "[lints]\nworkspace = true\n[\"lints\".rust]\nunsafe_code = \"allow\"\n",
            "\"lints\".rust.unsafe_code = \"allow\"\n[lints]\nworkspace = true\n",
        ] {
            assert!(check_lints_section(bad).is_err(), "{bad}");
        }
    }

    /// PY1: `ts_client_native` is a crate root like any other, with no anticipated exception.
    #[test]
    fn no_anticipated_unsafe_exception() {
        let m = parse(
            r#"{"workspace_root":"/w","packages":[
            {"name":"ts_client_native","manifest_path":"/w/c/Cargo.toml",
             "targets":[{"kind":["cdylib"],"src_path":"/w/c/src/lib.rs"}]}]}"#,
        )
        .unwrap();
        assert_eq!(crate_roots(&m), [("ts_client_native", "/w/c/src/lib.rs")]);
        let deny = "#![deny(unsafe_code)]\n";
        assert!(check_unsafe_root("ts_client_native", deny).is_err());
        assert!(check_unsafe_root("ts_client_native", "#![forbid(unsafe_code)]\n").is_ok());
    }

    const ROOT: &str = "[workspace]\nmembers = []\n\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\nmissing_debug_implementations = \"warn\"\n\n[workspace.lints.clippy]\nall = { level = \"warn\", priority = -1 }\nunwrap_used = \"warn\"\n\n[profile.dev]\nopt-level = 1\n";
    const LOCAL: &str = "[package]\nname = \"ts_py\" # x\n\n[lints.rust]\nmissing_debug_implementations = \"warn\"\nunsafe_code = \"deny\"\n\n[lints.clippy]\nall={level=\"warn\",priority=-1} # same\nunwrap_used = \"warn\"\n";

    /// AS-18, PY1: the local tables of `ts_py` equal the workspace's, except `unsafe_code`.
    #[test]
    fn local_lints_follow_the_workspace() {
        assert_eq!(check_local_lints(LOCAL, ROOT), Ok(()));
        let bad = [
            // Another level for unsafe_code, or forbid kept.
            LOCAL.replace("\"deny\"", "\"allow\""),
            LOCAL.replace("\"deny\"", "\"forbid\""),
            // A lint removed, added or weakened.
            LOCAL.replace("unwrap_used = \"warn\"\n", ""),
            format!("{LOCAL}expect_used = \"allow\"\n"),
            LOCAL.replace("priority=-1", "priority=0"),
            // Inheritance or extra settings next to the tables.
            format!("{LOCAL}\n[lints]\nworkspace = true\n"),
            format!("lints.rust.unsafe_code = \"allow\"\n{LOCAL}"),
            format!("{LOCAL}\n[lints.rustdoc]\nbroken_intra_doc_links = \"allow\"\n"),
            format!("{LOCAL}\n[lints.rust]\nunsafe_code = \"deny\"\n"),
            format!("{LOCAL}\n[[lints.rust]]\n"),
            LOCAL.replace(
                "unwrap_used = \"warn\"",
                "unwrap_used = \"warn\"\nunwrap_used = \"warn\"",
            ),
            LOCAL.replace("unwrap_used = \"warn\"", "unwrap_used"),
            LOCAL.replace("unsafe_code = \"deny\"", "\"unsafe_code\" = \"allow\""),
            // Quoted names are the same names for Cargo.
            format!("{LOCAL}\n[\"lints\".rustdoc]\nbroken_intra_doc_links = \"allow\"\n"),
            format!("\"lints\".rustdoc.x = \"allow\"\n{LOCAL}"),
            format!("{LOCAL}\n['lints'.\"rust\"]\nunsafe_code = \"allow\"\n"),
            format!("{LOCAL}\n[[\"lints\".rust]]\n"),
        ];
        for manifest in &bad {
            assert!(check_local_lints(manifest, ROOT).is_err(), "{manifest}");
        }
        // Quoted table names and keys equal to the expected ones are the same tables.
        let quoted = LOCAL
            .replace("[lints.clippy]", "[lints.\"clippy\"]")
            .replace("unwrap_used =", "\"unwrap_used\" =");
        assert_eq!(check_local_lints(&quoted, ROOT), Ok(()));
        // The workspace itself must forbid unsafe code.
        let weak = ROOT.replace("\"forbid\"", "\"deny\"");
        assert!(check_local_lints(LOCAL, &weak).is_err());
    }

    #[test]
    fn features_sample_is_clean() {
        assert_eq!(check_features(&sample()), Vec::<String>::new());
    }

    #[test]
    fn features_violations() {
        let m = parse(
            r#"{"workspace_root":"/w","packages":[
            {"name":"ts_kernel","manifest_path":"/w/k/Cargo.toml",
             "features":{"default":["dev-bench"],"public":[],"x":["public"]}},
            {"name":"ts_host","manifest_path":"/w/h/Cargo.toml","features":{"d":["ts_kernel?/dev-commands"]},
             "dependencies":[{"name":"ts_kernel","kind":null,"features":["external-pilot"]},
                             {"name":"ts_toy","kind":null}]},
            {"name":"ts_a","manifest_path":"/w/a/Cargo.toml","features":{"p":["ts_kernel?/public"]},
             "dependencies":[{"name":"ts_kernel","kind":"dev","features":["public"]}]},
            {"name":"ts_toy","manifest_path":"/w/t/Cargo.toml"}]}"#,
        )
        .unwrap();
        let errors = check_features(&m);
        assert_eq!(errors.len(), 7, "{errors:#?}");
    }

    /// Y6, D-DEV-02: `ts_toy` is refused in a normal dependency of a host or a rule crate,
    /// directly or through another crate, and accepted as a dev-dependency or in a tool.
    #[test]
    fn ts_toy_only_in_tests_and_tools() {
        let errors = |extra: &str| {
            let json = format!(
                r#"{{"workspace_root":"/w","packages":[
                {{"name":"ts_kernel","manifest_path":"/w/k/Cargo.toml","features":{{"default":[]}}}},
                {{"name":"ts_toy","manifest_path":"/w/t/Cargo.toml",
                  "dependencies":[{{"name":"ts_kernel","kind":null}}]}},
                {extra}]}}"#
            );
            check_features(&parse(&json).unwrap())
        };
        let pkg = |name: &str, dep: &str, kind: &str| {
            format!(
                r#"{{"name":"{name}","manifest_path":"/w/{name}/Cargo.toml",
                   "dependencies":[{{"name":"{dep}","kind":{kind}}}]}}"#
            )
        };
        // Refused: a host, a rule crate, or a crate of the core, directly.
        for name in [
            "ts_server",
            "ts_client_native",
            "ts_game",
            "ts_model",
            "ts_kernel2",
        ] {
            let e = errors(&pkg(name, "ts_toy", "null"));
            assert_eq!(
                e,
                [format!(
                    "{name} dépend de ts_toy en dépendance normale (D-DEV-02)"
                )]
            );
        }
        // Refused through another crate, even a tool: the host and the middle crate.
        let through = [
            pkg("ts_server", "ts_mid", "null"),
            pkg("ts_mid", "ts_toy", "null"),
        ];
        assert_eq!(errors(&through.join(",")).len(), 2);
        let tool = [
            pkg("ts_server", "ts_runner", "null"),
            pkg("ts_runner", "ts_toy", "null"),
        ];
        assert_eq!(
            errors(&tool.join(",")),
            ["ts_server dépend de ts_toy en dépendance normale (D-DEV-02)"]
        );
        // Accepted: as a dev-dependency, or in a tool.
        assert!(errors(&pkg("ts_game", "ts_toy", r#""dev""#)).is_empty());
        for name in TOOLS {
            assert!(errors(&pkg(name, "ts_toy", "null")).is_empty(), "{name}");
        }
    }

    #[test]
    fn tree_lines() {
        let tree = "ts_host v0.0.0\n└── ts_kernel v0.0.0 (/w/k) default\n    ts_kernel feature \"dev-bench\"\n";
        assert_eq!(tree_dev_features(tree), ["ts_kernel feature \"dev-bench\""]);
    }
}
