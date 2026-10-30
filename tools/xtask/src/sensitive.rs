//! Chemins sensibles (§5, G8) : la liste unique `tools/ci/sensitive-paths.txt`, **lue dans
//! `origin/main`**, appliquée avec le code de `main` (T3-a). Même lecture que `tools/ci/changes.sh`.
//! Les chemins se comparent **sans tenir compte de la casse** (B1) : sous Windows (NTFS,
//! `core.ignorecase`), `TOOLS/xtask/x.rs` écrase `tools/xtask/x.rs` au `git pull`.
//! spec: carnet architecture-securite.md §5 ; §9.6 G8
//! Voir aussi l'issue #6, condition T3-a.

use crate::github::Repo;

pub const LIST: &str = "tools/ci/sensitive-paths.txt";

#[derive(Debug, Default)]
pub struct Rules {
    prefixes: Vec<String>,
    exact: Vec<String>,
    sections: Vec<String>,
    lock_new_crate: bool,
    /// `cargo-toml:sources` : `[patch`, `[replace` ou une source hors crates.io (AS-23 point 4).
    toml_sources: bool,
    /// `cargo-lock:source-change` : une crate de `Cargo.lock` change de source.
    lock_source_change: bool,
}

/// Fragments qui, dans une ligne ajoutée ou retirée d'un `Cargo.toml` (espaces retirés), rendent
/// la PR sensible : remplacement de dépendance ou source hors crates.io.
const SOURCE_MARKS: [&str; 7] = [
    "[patch",
    "[replace",
    "git=",
    "registry=",
    "branch=",
    "tag=",
    "rev=",
];

/// Chemin comparable : `/`, sans `./` de tête, en minuscules ASCII.
pub fn norm(path: &str) -> String {
    let p = path.replace('\\', "/");
    let mut p = p.as_str();
    while let Some(r) = p.strip_prefix("./") {
        p = r;
    }
    p.to_ascii_lowercase()
}

/// `*` vaut un caractère ou plus, comme `.+` dans `changes.sh`.
fn glob(pat: &str, s: &str) -> bool {
    match pat.split_once('*') {
        None => pat == s,
        Some((head, tail)) => {
            let Some(rest) = s.strip_prefix(head) else {
                return false;
            };
            (1..=rest.len()).any(|i| rest.is_char_boundary(i) && glob(tail, &rest[i..]))
        }
    }
}

/// Section retenue : `profile.*` vaut aussi `[profile]` nu ; sinon motif `glob`.
fn section_matches(pat: &str, name: &str) -> bool {
    match pat.strip_suffix(".*") {
        Some(base) => {
            name == base
                || name
                    .strip_prefix(base)
                    .is_some_and(|r| r.len() > 1 && r.starts_with('.'))
        }
        None => glob(pat, name),
    }
}

impl Rules {
    /// Entrée inconnue (`truc:…`) : erreur, jamais ignorée.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut r = Self::default();
        for e in text
            .lines()
            .map(|l| l.split('#').next().unwrap_or_default().trim())
            .filter(|e| !e.is_empty())
        {
            if let Some(p) = e.strip_prefix("section:") {
                r.sections.push(p.to_owned());
            } else if e == "cargo-lock:new-crate" {
                r.lock_new_crate = true;
            } else if e == "cargo-toml:sources" {
                r.toml_sources = true;
            } else if e == "cargo-lock:source-change" {
                r.lock_source_change = true;
            } else if e.contains(':') || e.contains('*') {
                return Err(format!("{LIST} : entrée inconnue « {e} »"));
            } else if e.ends_with('/') {
                r.prefixes.push(norm(e));
            } else {
                r.exact.push(norm(e));
            }
        }
        if r.prefixes.is_empty() {
            return Err(format!("{LIST} : aucune entrée"));
        }
        Ok(r)
    }

    /// Chemin sensible, sans tenir compte de la casse (B1).
    pub fn path_hit(&self, f: &str) -> bool {
        let f = norm(f);
        self.prefixes.iter().any(|p| f.starts_with(p.as_str())) || self.exact.contains(&f)
    }

    /// B1 : arbre de tête refusé s'il contient deux chemins égaux à la casse près, ou un chemin
    /// non ASCII sous un préfixe sensible (repli de casse Unicode du système de fichiers).
    pub fn tree_errors(&self, tree: &[String]) -> Vec<String> {
        let mut seen = std::collections::BTreeMap::new();
        let mut out = Vec::new();
        for p in tree {
            if let Some(first) = seen.insert(p.to_lowercase(), p) {
                out.push(format!(
                    "« {first} » et « {p} » ne diffèrent que par la casse"
                ));
            }
            if !p.is_ascii() && (self.path_hit(p) || self.path_hit(&p.to_lowercase())) {
                out.push(format!(
                    "« {p} » : chemin non ASCII sous un chemin sensible"
                ));
            }
        }
        out
    }

    /// Lignes des sections retenues d'un `Cargo.toml`, en-têtes normalisés ; à la racine, les clés
    /// pointées ou en ligne de même premier segment (`features.x = …`, `profile = { … }`).
    pub fn sections_of(&self, toml: &str) -> Vec<String> {
        let firsts: Vec<&str> = self
            .sections
            .iter()
            .map(|p| p.split('.').next().unwrap_or_default())
            .collect();
        let squash = |l: &str| {
            l.chars()
                .filter(|c| !c.is_whitespace() && *c != '"' && *c != '\'')
                .collect::<String>()
        };
        let (mut root, mut keep, mut out) = (true, false, Vec::new());
        for line in toml.lines() {
            let t = line.trim_start();
            if let Some(h) = t.strip_prefix('[') {
                let name = squash(
                    h.trim_start_matches('[')
                        .split(']')
                        .next()
                        .unwrap_or_default(),
                );
                keep = self.sections.iter().any(|p| section_matches(p, &name));
                root = false;
                if keep {
                    out.push(format!("[{name}]"));
                }
            } else if root {
                let k = squash(line);
                let first = k.split(['.', '=']).next().unwrap_or_default();
                if firsts.iter().any(|p| glob(p, first)) {
                    out.push(format!("racine:{line}"));
                }
            } else if keep {
                out.push(line.to_owned());
            }
        }
        out
    }
}

/// Lignes ajoutées ou retirées d'un `Cargo.toml` qui touchent à `[patch]`, `[replace]` ou à une
/// source hors crates.io (comparaison de multiensembles de lignes, espaces retirés).
pub fn source_lines(base: &str, head: &str) -> Vec<String> {
    let squash = |l: &str| {
        l.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    };
    let count = |t: &str| {
        let mut m = std::collections::BTreeMap::<String, i64>::new();
        t.lines()
            .for_each(|l| *m.entry(squash(l)).or_default() += 1);
        m
    };
    let (b, h) = (count(base), count(head));
    let keys: std::collections::BTreeSet<&String> = b.keys().chain(h.keys()).collect();
    keys.into_iter()
        .filter(|k| b.get(*k) != h.get(*k) && SOURCE_MARKS.iter().any(|m| k.contains(m)))
        .cloned()
        .collect()
}

/// Sources de chaque crate d'un `Cargo.lock` (bloc `[[package]]` : `name`, `source`).
pub fn lock_sources(lock: &str) -> std::collections::BTreeSet<(String, String)> {
    let mut out = std::collections::BTreeSet::new();
    for block in lock.split("[[package]]").skip(1) {
        let field = |k: &str| {
            block
                .lines()
                .find_map(|l| l.strip_prefix(k)?.strip_prefix(" = \"")?.strip_suffix('"'))
                .unwrap_or_default()
                .to_owned()
        };
        out.insert((field("name"), field("source")));
    }
    out
}

/// Une crate déjà présente dans la base apparaît avec une source qu'elle n'y avait pas.
pub fn source_changed(base: &str, head: &str) -> bool {
    let (b, h) = (lock_sources(base), lock_sources(head));
    h.iter()
        .any(|(n, src)| !b.contains(&(n.clone(), src.clone())) && b.iter().any(|(bn, _)| bn == n))
}

/// Noms des crates d'un `Cargo.lock` (lignes `name = "…"`).
pub fn lock_crates(lock: &str) -> std::collections::BTreeSet<&str> {
    lock.lines()
        .filter_map(|l| l.strip_prefix("name = \"")?.strip_suffix('"'))
        .collect()
}

/// Règles lues dans `origin/main`, jamais dans la copie de travail (G8).
pub fn rules(repo: &Repo) -> Result<Rules, String> {
    let text = repo
        .blob(&repo.main, "origin/main", LIST)?
        .ok_or_else(|| format!("{LIST} absent d'origin/main"))?;
    Rules::parse(&String::from_utf8(text).map_err(|_| format!("{LIST} n'est pas en UTF-8"))?)
}

/// Fichiers de `paths` qui rendent sensible le passage de `base` à `head`.
pub fn hits(
    repo: &Repo,
    r: &Rules,
    base: &str,
    head: &str,
    paths: &[String],
) -> Result<Vec<String>, String> {
    let text = |rev: &str, f: &str| -> Result<String, String> {
        let b = repo.blob(&repo.main, rev, f)?.unwrap_or_default();
        String::from_utf8(b).map_err(|_| format!("{rev}:{f} n'est pas en UTF-8"))
    };
    let mut out = Vec::new();
    for f in paths {
        let n = norm(f);
        let hit = if r.path_hit(f) {
            true
        } else if n == "cargo.toml" || n.ends_with("/cargo.toml") {
            let (b, h) = (text(base, f)?, text(head, f)?);
            r.sections_of(&b) != r.sections_of(&h)
                || (r.toml_sources && !source_lines(&b, &h).is_empty())
        } else if n == "cargo.lock" {
            let (b, h) = (text(base, f)?, text(head, f)?);
            (r.lock_new_crate && !lock_crates(&h).is_subset(&lock_crates(&b)))
                || (r.lock_source_change && source_changed(&b, &h))
        } else {
            false
        };
        if hit {
            out.push(f.clone());
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real() -> Rules {
        Rules::parse(include_str!("../../ci/sensitive-paths.txt")).unwrap()
    }

    #[test]
    fn reads_the_list() {
        let r = real();
        for f in [
            "tools/xtask/src/merge.rs",
            ".github/workflows/ci.yml",
            "crates/ts_kernel/src/lib.rs",
            "deny.toml",
        ] {
            assert!(r.path_hit(f), "{f}");
        }
        for f in [
            "crates/ts_kernel/src/rng.rs",
            "docs/dev/xtask.md",
            "deny.toml.bak",
            "tools",
        ] {
            assert!(!r.path_hit(f), "{f}");
        }
        assert!(r.lock_new_crate && r.toml_sources && r.lock_source_change);
        // B1 : casse et forme des chemins.
        for f in [
            "TOOLS/xtask/src/merge.rs",
            ".CLAUDE/settings.json",
            ".CARGO/config",
            "./tools/XTASK/build.rs",
            "Deny.toml",
            "rust-toolchain",
            "tools\\dev\\guard-git.ps1",
        ] {
            assert!(r.path_hit(f), "{f}");
        }
        let tree: Vec<String> = [
            "tools/x.rs",
            "TOOLS/x.rs",
            "docs/é.md",
            ".claude/séttings.json",
        ]
        .map(str::to_owned)
        .to_vec();
        let errors = r.tree_errors(&tree);
        assert_eq!(errors.len(), 2, "{errors:?}");
        assert!(
            r.tree_errors(&["docs/é.md".to_owned(), "a".to_owned()])
                .is_empty()
        );
        assert!(Rules::parse("a/\nsection-x:y\n").is_err());
        assert!(Rules::parse("# vide\n").is_err());
    }

    #[test]
    fn cargo_sections() {
        let r = real();
        let base = "[package]\nname = \"a\"\n\n[features]\ndefault = []\n\n[profile.release]\nlto = true\n";
        assert_eq!(
            r.sections_of(base),
            r.sections_of(&base.replace("name = \"a\"", "name = \"b\""))
        );
        for changed in [
            base.replace("default = []", "default = [\"dev-x\"]"),
            base.replace("lto = true", "lto = false"),
            base.replace(
                "[profile.release]",
                "[ profile . \"release\" ]\nopt-level = 0",
            ),
            format!("profile.dev.debug = 0\n{base}"),
            format!("{base}\n[profile]\ndev = {{ debug = 0 }}\n"),
        ] {
            assert_ne!(r.sections_of(base), r.sections_of(&changed), "{changed}");
        }
        assert!(
            section_matches("profile.*", "profile") && section_matches("profile.*", "profile.a.b")
        );
        assert!(
            !section_matches("profile.*", "profiles") && !section_matches("profile.*", "profile.")
        );
        assert!(glob("a*c", "abc") && !glob("a*c", "ac"));
    }

    /// B3 : `[patch]`, `[replace]` et sources hors crates.io.
    #[test]
    fn dependency_sources() {
        let base = "[dependencies]\nserde = \"1\"\n";
        for head in [
            format!("{base}[patch.crates-io]\nserde = {{ path = \"x\" }}\n"),
            format!("{base}[replace]\n\"serde:1.0.0\" = {{ path = \"x\" }}\n"),
            base.replace("\"1\"", "{ git = \"https://evil/serde\" }"),
            base.replace("\"1\"", "{ version = \"1\", registry = \"evil\" }"),
            format!("{base}x = {{ git = \"g\", branch = \"b\" }}\n"),
            format!("{base}x = {{ git = \"g\", rev = \"r\" }}\n"),
        ] {
            assert!(!source_lines(base, &head).is_empty(), "{head}");
        }
        assert!(source_lines(base, &base.replace("\"1\"", "\"1.0.2\"")).is_empty());
        let lock = |src: &str| {
            format!("[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\nsource = \"{src}\"\n")
        };
        let crates = "registry+https://github.com/rust-lang/crates.io-index";
        assert!(source_changed(
            &lock(crates),
            &lock("git+https://evil/serde#abc")
        ));
        assert!(!source_changed(
            &lock(crates),
            &lock(crates).replace("1.0.0", "1.0.1")
        ));
        assert!(!source_changed(
            &lock(crates),
            &format!("{}{}", lock(crates), lock(crates).replace("serde", "neuf"))
        ));
    }

    #[test]
    fn new_crates_in_lock() {
        let base = "[[package]]\nname = \"serde\"\nversion = \"1\"\n";
        let head = format!("{base}\n[[package]]\nname = \"evil\"\n");
        assert!(!lock_crates(&head).is_subset(&lock_crates(base)));
        assert!(lock_crates(base).is_subset(&lock_crates(&head)));
    }
}
