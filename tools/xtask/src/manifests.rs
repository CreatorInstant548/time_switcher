//! Contrôles des crates et des manifestes : `unsafe_code` (SEC0-19, F3), fonctionnalités et
//! `ts_toy` (SEC0-20, F4, D-DEV-02). Fonctions pures, sans E/S.

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

/// Racines de crate à contrôler : (crate, fichier source), sauf `ts_client_native` (FFI).
pub fn crate_roots(meta: &Metadata) -> Vec<(&str, &str)> {
    let mut roots = Vec::new();
    for p in meta
        .packages
        .iter()
        .filter(|p| p.name != "ts_client_native")
    {
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
    let expected = match crate_name {
        "ts_py" => "#![deny(unsafe_code)]",
        _ => "#![forbid(unsafe_code)]",
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
        let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();
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

    #[test]
    fn tree_lines() {
        let tree = "ts_host v0.0.0\n└── ts_kernel v0.0.0 (/w/k) default\n    ts_kernel feature \"dev-bench\"\n";
        assert_eq!(tree_dev_features(tree), ["ts_kernel feature \"dev-bench\""]);
    }
}
