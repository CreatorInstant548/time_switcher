//! Recherches dans les sources : exemptions de clippy dans le cœur (E6, F5) et chemins locaux
//! (SEC0-26, F6). Fonctions pures, sans E/S.
//!
//! Ce fichier est exempté des motifs de chemins fixes (liste fermée `LOCAL_PATH_EXEMPT`) : il les
//! définit et les teste. Les noms d'utilisateur et de machine y restent recherchés.

/// Lints dont un `allow` ou un `expect` rouvrirait l'horloge, l'aléa ou les E/S dans le cœur.
/// Les anciens noms au singulier restent des alias acceptés par clippy.
const FORBIDDEN_LINTS: [&str; 7] = [
    "clippy::disallowed_methods",
    "clippy::disallowed_types",
    "clippy::disallowed_method",
    "clippy::disallowed_type",
    "clippy::style",
    "clippy::all",
    "warnings",
];

/// Fichiers exemptés des motifs de chemins fixes (F6), en liste fermée.
pub const LOCAL_PATH_EXEMPT: [&str; 1] = ["tools/xtask/src/scan.rs"];

/// Arguments entre parenthèses à partir de `open` (juste après la `(`), jusqu'à la `)` qui ferme.
fn balanced(text: &[char], open: usize) -> String {
    let mut depth = 1;
    let mut end = open;
    while end < text.len() && depth > 0 {
        depth += match text[end] {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        end += 1;
    }
    text[open..end.saturating_sub(1).max(open)].iter().collect()
}

/// Premier argument de niveau 0 (prédicat d'un `cfg_attr`).
fn first_arg(args: &str) -> &str {
    let mut depth = 0;
    for (i, c) in args.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => return &args[..i],
            _ => {}
        }
    }
    args
}

/// Numéros de ligne des formes qui rouvriraient l'horloge, l'aléa ou les E/S dans le cœur sans
/// que clippy le voie (E6, F5, I2) :
/// - `allow(…)` ou `expect(…)` d'un lint interdit, attributs `#[…]`, `#![…]` et `cfg_attr` compris ;
/// - `$` dans les arguments d'un `allow`, `expect`, `warn` ou `deny` (lint venu d'une macro) ;
/// - `#[path = …]` et `include!`, `include_str!`, `include_bytes!` (code hors des `.rs` lus) ;
/// - `clippy` dans un `cfg(…)`, `cfg!(…)` ou le prédicat d'un `cfg_attr(…)` (code caché à clippy).
///
/// Les blancs sont ignorés ; les commentaires ne le sont pas (refus en cas de doute).
pub fn clippy_exemptions(source: &str) -> Vec<usize> {
    let mut text = Vec::new();
    let mut line_of = Vec::new();
    // Un blanc précédait le caractère : début de mot, même collé dans `text`.
    let mut gap = Vec::new();
    for (n, line) in source.lines().enumerate() {
        let mut blank = true;
        for c in line.chars() {
            if c.is_whitespace() {
                blank = true;
                continue;
            }
            text.push(c);
            line_of.push(n + 1);
            gap.push(blank);
            blank = false;
        }
    }
    let mut found = Vec::new();
    for start in 0..text.len() {
        let at = |w: &str| text[start..].iter().take(w.len()).copied().eq(w.chars());
        let prev = start.checked_sub(1).map(|p| text[p]);
        let attr_arg = matches!(prev, Some('[' | ',' | '('));
        let word = gap[start] || !prev.is_some_and(|c| c.is_alphanumeric() || c == '_');
        let inclusion = (attr_arg && at("path="))
            || (word
                && ["include!(", "include_str!(", "include_bytes!("]
                    .iter()
                    .any(|w| at(w)));
        let cfg = word
            && ["cfg(", "cfg!("]
                .iter()
                .find(|w| at(w))
                .is_some_and(|w| balanced(&text, start + w.len()).contains("clippy"));
        let cfg_attr = word
            && at("cfg_attr(")
            && first_arg(&balanced(&text, start + "cfg_attr(".len())).contains("clippy");
        let lint = attr_arg
            && ["allow(", "expect(", "warn(", "deny("]
                .iter()
                .find(|w| at(w))
                .is_some_and(|w| {
                    let args = balanced(&text, start + w.len());
                    let exempts = w.starts_with('a') || w.starts_with('e');
                    args.contains('$')
                        || (exempts && args.split(',').any(|a| FORBIDDEN_LINTS.contains(&a)))
                });
        if inclusion || cfg || cfg_attr || lint {
            found.push(line_of[start]);
        }
    }
    found.dedup();
    found
}

/// Numéros de ligne où figure un chemin de dossier personnel (`/home/`, `\Users\`, séparateurs
/// mêlés, sans tenir compte de la casse), sauf fichier exempté, ou l'un des noms donnés
/// (utilisateur, machine), recherchés partout.
pub fn local_paths(bytes: &[u8], names: &[String], exempt: bool) -> Vec<usize> {
    let lower = bytes.to_ascii_lowercase();
    let is_sep = |b: Option<&u8>| matches!(b, Some(b'/' | b'\\'));
    let mut lines = Vec::new();
    let mut line = 1;
    for i in 0..lower.len() {
        if lower[i] == b'\n' {
            line += 1;
            continue;
        }
        let home = !exempt
            && i > 0
            && is_sep(lower.get(i - 1))
            && ["users", "home"]
                .iter()
                .any(|w| lower[i..].starts_with(w.as_bytes()) && is_sep(lower.get(i + w.len())));
        // Nom entier seulement : `runner` ne doit pas trouver `ts_runner`.
        let word = |b: Option<&u8>| b.is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_');
        let name = names.iter().any(|n| {
            lower[i..].starts_with(n.as_bytes())
                && !(i > 0 && word(lower.get(i - 1)))
                && !word(lower.get(i + n.len()))
        });
        if (home || name) && lines.last() != Some(&line) {
            lines.push(line);
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_every_exemption_form() {
        assert_eq!(
            clippy_exemptions("#[allow(clippy::disallowed_methods)]\nfn a() {}"),
            [1]
        );
        assert_eq!(
            clippy_exemptions("fn a() {}\n#![expect( clippy :: all )]"),
            [2]
        );
        assert_eq!(
            clippy_exemptions("#[cfg_attr(test, allow(dead_code, warnings))]"),
            [1]
        );
        assert_eq!(clippy_exemptions("#[allow(\n  clippy::style,\n)]"), [1]);
        assert_eq!(
            clippy_exemptions("#[allow(clippy::disallowed_type, reason = \"x\")]"),
            [1]
        );
        assert_eq!(clippy_exemptions("// #[allow(warnings)]"), [1]);
    }

    /// Les quatre formes de la relecture neuve (I2), plus les variantes voisines.
    #[test]
    fn finds_hidden_code_forms() {
        let forms = [
            "#[path = \"clock.inc\"] mod clock;",
            "#[path = \"../../../tools/xtask/src/zz_clock.rs\"] mod clock2;",
            "#[cfg(not(clippy))] pub fn now3() {}",
            "macro_rules! quiet { ($($l:tt)*) => { #[allow($($l)*)] pub fn now4() {} } }",
            "#[cfg_attr(clippy, path = \"x.rs\")] mod x;",
            "#[cfg_attr(not(clippy), allow(dead_code))] fn a() {}",
            "const S: &str = include_str!(\"clock.inc\");",
            "std::include!(\"clock.inc\");",
            "const B: &[u8] = include_bytes!(\"x\");",
            "if cfg!(clippy) {}",
            "#[deny($lint)] fn a() {}",
        ];
        for form in forms {
            assert_eq!(
                clippy_exemptions(&format!("fn z() {{}}\n{form}")),
                [2],
                "{form}"
            );
        }
        let ok = "#[cfg(test)]\n#[cfg_attr(test, allow(clippy::unwrap_used))]\nfn path(p: u8) { includes(p); }";
        assert!(clippy_exemptions(ok).is_empty());
    }

    #[test]
    fn ignores_legitimate_code() {
        let ok = "#[allow(dead_code)]\nfn a() { x.expect(\"warnings\"); y.allow(clippy::all); }";
        assert!(clippy_exemptions(ok).is_empty());
        assert!(clippy_exemptions("#[expect(clippy::unwrap_used)]").is_empty());
    }

    #[test]
    fn finds_local_paths() {
        let text = b"a\nC:\\Users\\x\nc:/users/x\n/home/x\nD:\\HOME/x\nhomework /users\n";
        assert_eq!(local_paths(text, &[], false), [2, 3, 4, 5]);
        assert!(local_paths(text, &[], true).is_empty());
        let names = ["pc-42".to_owned()];
        assert_eq!(local_paths(b"ok\nmachine PC-42\n", &names, true), [2]);
        let user = ["runner".to_owned()];
        assert!(local_paths(b"ts_runner runners", &user, false).is_empty());
        assert_eq!(local_paths(b"x\n/c/runner/", &user, true), [2]);
    }
}
