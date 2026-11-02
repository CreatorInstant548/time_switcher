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
    // Code lines only: the attribute inside a comment or a literal does not count (m2).
    match crate::exemptions::code_lines(source)
        .iter()
        .any(|l| l == expected)
    {
        true => Ok(()),
        false => Err(format!("{expected} absent")),
    }
}

/// The attributes naming `unsafe_code` that need no `// SAFETY:` comment: those that only make the
/// lint stricter, written alone, the crate attribute of [`LOCAL_LINTS_CRATE`] among them.
const STRICT_ATTRIBUTES: [&str; 4] = [
    "#![deny(unsafe_code)]",
    "#![forbid(unsafe_code)]",
    "#[deny(unsafe_code)]",
    "#[forbid(unsafe_code)]",
];

/// First line of a mention of `unsafe_code` in code (comments and literals excluded) that is not
/// a strict attribute ([`STRICT_ATTRIBUTES`]) and has no `// SAFETY:` line comment just before
/// the attribute that holds it (exception of `ts_py`, AS-18). The source is read as rustc reads
/// it, so an attribute split over several lines (`#[allow(\n unsafe_code)]`) is seen whole, and a
/// `// SAFETY:` inside a literal or a block comment is no comment (#91). Fails closed: a mention
/// outside any attribute (a macro argument, say) is reported too, and so is any weakening lint
/// level, `allow`, `expect` and `warn` alike.
pub fn unsafe_allow_without_safety(source: &str) -> Option<usize> {
    let src = crate::exemptions::normalize(source);
    let blank: Vec<bool> = source.lines().map(|l| l.trim().is_empty()).collect();
    // The last non-blank line before `line` is a line comment alone on its line, `// SAFETY:`.
    let safety_before = |line: usize| {
        (1..line)
            .rev()
            .find(|&l| !blank[l - 1])
            .is_some_and(|l| src.safety_lines.contains(&l))
    };
    let tokens = src.tokens();
    let mentions = tokens
        .iter()
        .filter(|&&t| src.word(t) == "unsafe_code")
        .map(|&(start, _)| start);
    for at in mentions {
        let Some((open, close)) = attribute_around(&src, at) else {
            return Some(src.line[at]);
        };
        let text: String = src.text[open..=close].iter().collect();
        if STRICT_ATTRIBUTES.contains(&text.as_str()) || safety_before(src.line[open]) {
            continue;
        }
        return Some(src.line[open]);
    }
    None
}

/// The attribute (`#[…]` or `#![…]`) that holds the character at `at`: index of its `#` and of
/// its closing `]`, brackets of literals ignored.
fn attribute_around(src: &crate::exemptions::Src, at: usize) -> Option<(usize, usize)> {
    let code = |k: usize| !src.string[k];
    let mut depth = 0_usize;
    let mut k = at;
    let open = loop {
        k = k.checked_sub(1)?;
        match src.text[k] {
            ']' if code(k) => depth += 1,
            '[' if code(k) && depth > 0 => depth -= 1,
            '[' if code(k) => break k,
            _ => {}
        }
    };
    let hash = match open.checked_sub(1).map(|p| src.text[p]) {
        Some('#') => open - 1,
        Some('!') if open >= 2 && src.text[open - 2] == '#' => open - 2,
        _ => return None,
    };
    let mut depth = 0_usize;
    for (k, c) in src.text.iter().enumerate().skip(open + 1) {
        match c {
            '[' if code(k) => depth += 1,
            ']' if code(k) && depth == 0 => return Some((hash, k)),
            ']' if code(k) => depth -= 1,
            _ => {}
        }
    }
    None
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
    for (n, line) in manifest.lines().map(toml_code).enumerate() {
        // A line-based reading cannot follow multi-line strings or escapes: refused outside
        // comments, so that no real table hides behind a fake one (no member manifest uses them).
        if ["\"\"\"", "'''", "\\"].iter().any(|s| line.contains(s)) {
            return Err(format!(
                "line {}: multi-line string or backslash escape refused in a key or value of a \
                 manifest with lint tables (comments may hold them)",
                n + 1
            ));
        }
        // `["lints.rust"]` is one key named `lints.rust` for Cargo, not the table [lints.rust]:
        // a quoted segment with a dot is refused in a table name or a key (#91).
        if quoted_dot(line) {
            return Err(format!(
                "line {}: quoted table name or key with a dot refused: {}",
                n + 1,
                line.trim()
            ));
        }
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

/// The code of a TOML line: up to a `#` that is outside quotes. A backslash is not read as an
/// escape here; [`lint_tables`] refuses any backslash in the code, whichever way it would be read.
fn toml_code(line: &str) -> &str {
    let mut quote = None;
    for (i, c) in line.char_indices() {
        match (quote, c) {
            (None, '"' | '\'') => quote = Some(c),
            (Some(q), _) if c == q => quote = None,
            (None, '#') => return &line[..i],
            _ => {}
        }
    }
    line
}

/// A quoted segment (`"…"` or `'…'`, closed or not) of the table name or key of `line` holds a
/// dot. The key ends at the first `=` outside quotes, so `"a=b.c" = 1` is read whole; a line
/// that is not a table header and has no such `=`, or a `,` or `{` before it, is a value (an
/// array continued over lines).
fn quoted_dot(line: &str) -> bool {
    let header = line.trim_start().starts_with('[');
    let (mut quote, mut dot) = (None, false);
    for c in line.chars() {
        match (quote, c) {
            (None, '"' | '\'') => quote = Some(c),
            (Some(q), _) if c == q => quote = None,
            (None, ',' | '{') if !header => return false,
            (None, '=') => return dot,
            (Some(_), '.') => dot = true,
            _ => {}
        }
    }
    header && dot
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

/// Core or rules crate: neither a host nor a tool (as `steps::is_core`).
fn is_core_crate(name: &str) -> bool {
    !TOOLS.contains(&name) && !HOSTS.contains(&name)
}

/// Features of a core crate that the clippy and test passes see (m13): a closed list. Any other
/// feature would hide code (`#[cfg(feature = "extra")]`) from clippy, which does not turn it on.
/// Allowed: an empty `default`; for `ts_kernel`, `public` and the development features; for any
/// core crate, a development feature that only relays the one of `ts_kernel`
/// (`dev-commands = ["ts_kernel/dev-commands"]`).
fn feature_covered(crate_name: &str, feature: &str, items: &[String]) -> bool {
    let dev = DEV_FEATURES.contains(&feature);
    let relay = items.len() == 1 && items[0] == format!("ts_kernel/{feature}");
    match (feature, crate_name == "ts_kernel") {
        ("default", _) => items.is_empty(),
        ("public", kernel) => kernel,
        (_, true) => dev,
        (_, false) => dev && relay,
    }
}

/// The features `features` of the workspace crate `krate` turn on a development feature of
/// `ts_kernel`, directly or through the features of the workspace crates they name, whatever
/// those features are called (#36). `seen` holds the (crate, feature) pairs already followed.
fn reaches_dev(
    meta: &Metadata,
    krate: &str,
    features: &[String],
    seen: &mut BTreeSet<(String, String)>,
) -> bool {
    for item in features.iter().filter(|f| !f.starts_with("dep:")) {
        let (target, feature) = match item.split_once('/') {
            Some((dep, f)) => (dep.trim_end_matches('?'), f),
            None => (krate, item.as_str()),
        };
        if target == "ts_kernel" && DEV_FEATURES.contains(&feature) {
            return true;
        }
        if !seen.insert((target.to_owned(), feature.to_owned())) {
            continue;
        }
        let items = meta.package(target).and_then(|p| p.features.get(feature));
        if items.is_some_and(|items| reaches_dev(meta, target, items, seen)) {
            return true;
        }
    }
    false
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
            if is_core_crate(&p.name) && !feature_covered(&p.name, feature, items) {
                errors.push(format!(
                    "{}: feature {feature} is outside the closed list seen by the clippy passes \
                     (m13): add a pass in steps::clippy and extend the list",
                    p.name
                ));
            }
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
                    "{}: its ts_kernel dependency turns on public",
                    p.name
                ));
            }
        }
        // #36: a dependency on any crate of the workspace, not only ts_kernel, since a crate may
        // relay the development features (`ts_model = { features = ["dev-commands"] }`).
        for d in p
            .dependencies
            .iter()
            .filter(|d| meta.package(&d.name).is_some())
        {
            let mut on = d.features.clone();
            if d.uses_default_features {
                on.push("default".to_owned());
            }
            // Fails closed: a development feature named anywhere, even unresolved, counts.
            let named = d.features.iter().any(|f| {
                DEV_FEATURES
                    .iter()
                    .any(|dev| f == dev || f.ends_with(&format!("/{dev}")))
            });
            let dev = named || reaches_dev(meta, &d.name, &on, &mut BTreeSet::new());
            if HOSTS.contains(&p.name.as_str()) && d.kind.is_none() && dev {
                errors.push(format!(
                    "host {}: normal dependency on {} with a development feature",
                    p.name, d.name
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
        // m2: in a block comment, a doc comment or a string, the attribute does not count.
        for hidden in [
            "/*\n#![forbid(unsafe_code)]\n*/\n",
            "/* a /* b */\n#![forbid(unsafe_code)] */\n",
            "/** doc\n#![forbid(unsafe_code)]\n*/ fn a() {}\n",
            "const S: &str = \"\n#![forbid(unsafe_code)]\n\";\n",
        ] {
            assert!(check_unsafe_root("ts_kernel", hidden).is_err(), "{hidden}");
        }
        assert!(check_unsafe_root("ts_kernel", "/* ok */ #![forbid(unsafe_code)] // x\n").is_ok());
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

    /// #91: the attribute is read as rustc reads it, whatever its layout.
    #[test]
    fn unsafe_exemptions_are_read_whole() {
        let root = "#![deny(unsafe_code)]\n";
        for bad in [
            // Split over lines: the comment must precede the attribute, not its last line.
            "#[allow(\nunsafe_code)]\nfn a() {}\n",
            "#[allow(\n    dead_code,\n    // SAFETY: not here\n    unsafe_code,\n)]\nfn a() {}\n",
            "#[\nallow(unsafe_code)]\nfn a() {}\n",
            "#[cfg_attr(test,\n allow(unsafe_code))]\nfn a() {}\n",
            // Other levels and places weaken the crate attribute too.
            "#[warn(unsafe_code)]\nfn a() {}\n",
            "fn a() {\n    #![allow(unsafe_code)]\n}\n",
            // Outside an attribute: unreadable, refused.
            "m!(unsafe_code);\n",
            // A comment between the SAFETY line and the attribute breaks the pair.
            "// SAFETY: x\n// other\n#[allow(unsafe_code)]\nfn a() {}\n",
            // A `// SAFETY:` in a literal, in a block comment or after code is no comment.
            "const S: &str = \"\n// SAFETY: fake\";\n#[allow(unsafe_code)]\nfn a() {}\n",
            "/* x\n// SAFETY: fake */\n#[allow(unsafe_code)]\nfn a() {}\n",
            "/* x\n// SAFETY: fake\n*/\n#[allow(unsafe_code)]\nfn a() {}\n",
            "fn b() {} // SAFETY: fake\n#[allow(unsafe_code)]\nfn a() {}\n",
            "/// SAFETY: a doc comment\n#[allow(unsafe_code)]\nfn a() {}\n",
        ] {
            let text = format!("{root}{bad}");
            assert!(
                unsafe_allow_without_safety(&text).is_some(),
                "{text}\n(should be refused)"
            );
        }
        for ok in [
            "// SAFETY: pyo3 macro\n#[allow(\n    unsafe_code\n)]\nfn a() {}\n",
            "/* #[allow(unsafe_code)] */\nconst S: &str = \"#[allow(unsafe_code)]\";\n",
            "// SAFETY: x\n#[cfg_attr(test, allow(unsafe_code))]\nfn a() {}\n",
            "  //   SAFETY: x\n\n#[allow(unsafe_code)]\nfn a() {}\n",
            // Attributes that only make the lint stricter need no justification.
            "mod m {\n    #![forbid( unsafe_code )]\n}\n#[deny(unsafe_code)]\nfn a() {}\n",
        ] {
            let text = format!("{root}{ok}");
            assert_eq!(unsafe_allow_without_safety(&text), None, "{text}");
        }
        // The split attribute is reported at its first line.
        assert_eq!(
            unsafe_allow_without_safety(
                "#![deny(unsafe_code)]\nfn a() {}\n#[allow(\nunsafe_code)]\n"
            ),
            Some(3)
        );
        // A crate attribute in another form than the expected one is not exempt.
        assert!(unsafe_allow_without_safety("#![deny(unsafe_code, dead_code)]\n").is_some());
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
            // Escapes and multi-line strings, which a line reading cannot follow.
            format!("{LOCAL}\n[\"\\u006cints\".rustdoc]\nx = \"allow\"\n"),
            format!("description = \"\"\"\n{LOCAL}\"\"\"\n[lints.rust]\nunsafe_code = \"allow\"\n"),
            // #91: a quoted name with a dot is one key for Cargo, not a lint table.
            LOCAL
                .replace("[lints.rust]", "[\"lints.rust\"]")
                .replace("[lints.clippy]", "[\"lints.clippy\"]"),
            LOCAL.replace("[lints.clippy]", "['lints.clippy']"),
            LOCAL.replace("[lints.clippy]", "[lints.\"clippy.x\"]"),
            LOCAL.replace("unwrap_used =", "\"unwrap_used.x\" ="),
            LOCAL.replace("unwrap_used =", "\"un=wrap.used\" ="),
        ];
        for manifest in &bad {
            assert!(check_local_lints(manifest, ROOT).is_err(), "{manifest}");
        }
        // Quoted table names and keys equal to the expected ones are the same tables.
        let quoted = LOCAL
            .replace("[lints.clippy]", "[lints.\"clippy\"]")
            .replace("unwrap_used =", "\"unwrap_used\" =");
        assert_eq!(check_local_lints(&quoted, ROOT), Ok(()));
        // #91: a backslash or a triple quote in a comment is no escape; a `#` in a value is no
        // comment, and dotted values are values.
        let commented = format!("# C:\\Dev \"\"\" '''\n{LOCAL}");
        assert_eq!(check_local_lints(&commented, ROOT), Ok(()));
        let root = ROOT.replace("[profile.dev]", "# a\\b\n[profile.dev] # c:\\x");
        assert_eq!(check_local_lints(LOCAL, &root), Ok(()));
        let valued = LOCAL.replace(
            "name = \"ts_py\"",
            "name = \"ts#py\"\nversion = \"0.1.0\"\ninclude = [\n  \"src/*.rs\",\n  \"a.b\", { x = 1 },\n]",
        );
        assert_eq!(check_local_lints(&valued, ROOT), Ok(()));
        let err = check_local_lints(&format!("x = \"a\\u0062\"\n{LOCAL}"), ROOT).unwrap_err();
        assert!(err.contains("line 1"), "{err}");
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
        // Seven earlier violations, and three features outside the closed list (m13): the
        // kernel's `default` (not empty) and `x`, and `p` of a core crate.
        assert_eq!(errors.len(), 10, "{errors:#?}");
    }

    /// #36: a host's normal dependency on any workspace crate may not turn on a development
    /// feature; a dev-dependency may, and so may a dependency outside the workspace.
    #[test]
    fn host_dependency_on_a_relaying_crate() {
        let run = |dep: &str| {
            let json = format!(
                r#"{{"workspace_root":"/w","packages":[
                {{"name":"ts_kernel","manifest_path":"/w/k/Cargo.toml","features":{{"default":[]}}}},
                {{"name":"ts_model","manifest_path":"/w/m/Cargo.toml",
                  "features":{{"dev-commands":["ts_kernel/dev-commands"]}}}},
                {{"name":"ts_runner","manifest_path":"/w/r/Cargo.toml",
                  "features":{{"default":["sim"],"sim":["sub"],"sub":["ts_model?/dev-commands"],
                  "plain":[]}}}},
                {{"name":"ts_server","manifest_path":"/w/s/Cargo.toml","dependencies":[{dep}]}}]}}"#
            );
            check_features(&parse(&json).unwrap())
        };
        let e = run(r#"{"name":"ts_model","kind":null,"features":["dev-commands"]}"#);
        assert_eq!(
            e,
            ["host ts_server: normal dependency on ts_model with a development feature"]
        );
        // The kernel's feature named through the dependency's own dependency.
        let e = run(r#"{"name":"ts_model","kind":null,"features":["ts_kernel/dev-bench"]}"#);
        assert_eq!(e.len(), 1, "{e:?}");
        assert!(run(r#"{"name":"ts_model","kind":"dev","features":["dev-commands"]}"#).is_empty());
        assert!(run(r#"{"name":"ts_model","kind":null,"features":[]}"#).is_empty());
        // A relay under another name, followed through the features, default included.
        let off = r#""uses_default_features":false"#;
        let e = run(&format!(
            r#"{{"name":"ts_runner","kind":null,"features":["sim"],{off}}}"#
        ));
        assert_eq!(e.len(), 1, "{e:?}");
        assert_eq!(
            run(r#"{"name":"ts_runner","kind":null,"features":[]}"#).len(),
            1
        );
        assert!(
            run(&format!(
                r#"{{"name":"ts_runner","kind":null,"features":["plain"],{off}}}"#
            ))
            .is_empty()
        );
        assert!(run(r#"{"name":"other","kind":null,"features":["dev-commands"]}"#).is_empty());
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

    /// m13: a feature of a core crate outside the closed list is refused, since no clippy pass
    /// turns it on.
    #[test]
    fn core_features_are_a_closed_list() {
        let run = |name: &str, features: &str| {
            let json = format!(
                r#"{{"workspace_root":"/w","packages":[
                {{"name":"ts_kernel","manifest_path":"/w/k/Cargo.toml","features":{{"default":[]}}}},
                {{"name":"{name}","manifest_path":"/w/x/Cargo.toml","features":{features}}}]}}"#
            );
            check_features(&parse(&json).unwrap())
        };
        // The kernel's own and the relayed development features, and `default`.
        assert!(
            run(
                "ts_model",
                r#"{"default":[],"dev-commands":["ts_kernel/dev-commands"]}"#
            )
            .is_empty()
        );
        // Another feature, or a relay that turns something else on.
        for (name, bad) in [
            ("ts_model", r#"{"extra":[]}"#),
            ("ts_model", r#"{"dev-commands":[]}"#),
            ("ts_model", r#"{"dev-commands":["ts_kernel/dev-bench"]}"#),
            ("ts_model", r#"{"public":[]}"#),
            ("ts_game", r#"{"dep:serde":["dep:serde"]}"#),
        ] {
            assert!(!run(name, bad).is_empty(), "{name} {bad}");
        }
        // Hosts and tools are not core crates.
        assert!(run("ts_host", r#"{"extra":[]}"#).is_empty());
        assert!(run("xtask", r#"{"extra":[]}"#).is_empty());
        // The kernel itself.
        let kernel = |features: &str| {
            let json = format!(
                r#"{{"workspace_root":"/w","packages":[{{"name":"ts_kernel","manifest_path":"/w/k/Cargo.toml","features":{features}}}]}}"#
            );
            check_features(&parse(&json).unwrap())
        };
        assert!(
            kernel(r#"{"default":[],"public":[],"dev-bench":[],"external-pilot":[]}"#).is_empty()
        );
        assert!(!kernel(r#"{"default":[],"extra":[]}"#).is_empty());
    }
}
