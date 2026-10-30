//! Chemins sensibles (§5, G8) : la liste unique `tools/ci/sensitive-paths.txt`, **lue dans
//! `origin/main`**, appliquée avec le code de `main` (T3-a). Même lecture que `tools/ci/changes.sh`.
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
            } else if e.contains(':') || e.contains('*') {
                return Err(format!("{LIST} : entrée inconnue « {e} »"));
            } else if e.ends_with('/') {
                r.prefixes.push(e.to_owned());
            } else {
                r.exact.push(e.to_owned());
            }
        }
        if r.prefixes.is_empty() {
            return Err(format!("{LIST} : aucune entrée"));
        }
        Ok(r)
    }

    pub fn path_hit(&self, f: &str) -> bool {
        self.prefixes.iter().any(|p| f.starts_with(p.as_str())) || self.exact.iter().any(|e| e == f)
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
        let hit = if r.path_hit(f) {
            true
        } else if f == "Cargo.toml" || f.ends_with("/Cargo.toml") {
            r.sections_of(&text(base, f)?) != r.sections_of(&text(head, f)?)
        } else if f == "Cargo.lock" && r.lock_new_crate {
            let (b, h) = (text(base, f)?, text(head, f)?);
            !lock_crates(&h).is_subset(&lock_crates(&b))
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
        assert!(r.lock_new_crate);
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

    #[test]
    fn new_crates_in_lock() {
        let base = "[[package]]\nname = \"serde\"\nversion = \"1\"\n";
        let head = format!("{base}\n[[package]]\nname = \"evil\"\n");
        assert!(!lock_crates(&head).is_subset(&lock_crates(base)));
        assert!(lock_crates(base).is_subset(&lock_crates(&head)));
    }
}
