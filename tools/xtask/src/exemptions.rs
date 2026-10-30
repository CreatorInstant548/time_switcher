//! Code des crates de cœur que clippy ne verrait pas, ou qu'il laisserait passer (E6, F5, I2).
//! Fonctions pures, sans E/S.
//!
//! La source est d'abord **normalisée** comme rustc la lit : commentaires (`//`, `/* */` imbriqués,
//! doc comprise) retirés, préfixes `r#` des identifiants retirés, chaînes et caractères gardés
//! mais marqués. Seuls les motifs hors chaîne comptent. Les règles portent ensuite sur des
//! identifiants entiers, ce qui ferme les formes par commentaire, par macro et par accolades.

/// Identifiants refusés partout : les lints d'interdiction de clippy, anciens noms compris
/// (`disallowed_method`, `disallowed_type`, `blacklisted_name`), et l'inclusion de code.
fn forbidden_ident(token: &str) -> bool {
    token.contains("disallowed_")
        || token == "blacklisted_name"
        || matches!(token, "include" | "include_str" | "include_bytes")
}

/// Groupes ou lints dont un `allow` ou un `expect` couvrirait les interdictions.
const FORBIDDEN_GROUPS: [&str; 3] = ["clippy::all", "clippy::style", "warnings"];
/// Seuls prédicats permis dans `cfg(…)`, `cfg!(…)` et le prédicat de `cfg_attr(…)`. Tout autre
/// (`clippy`, `debug_assertions`, `target_*`, `doc`…) cacherait du code à clippy ou à l'artefact.
const CFG_IDENTS: [&str; 5] = ["test", "feature", "all", "any", "not"];

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Source normalisée, sans blancs : pour chaque caractère, sa ligne, un blanc (ou commentaire)
/// juste avant lui, et son appartenance à une chaîne ou à un caractère littéral.
#[derive(Default)]
struct Src {
    text: Vec<char>,
    line: Vec<usize>,
    gap: Vec<bool>,
    string: Vec<bool>,
}

impl Src {
    fn push(&mut self, c: char, line: usize, gap: bool, string: bool) {
        self.text.push(c);
        self.line.push(line);
        self.gap.push(gap);
        self.string.push(string);
    }

    /// Identifiants hors chaîne : (début, fin exclue). Un blanc coupe un identifiant.
    fn tokens(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        let mut k = 0;
        while k < self.text.len() {
            if self.string[k] || !is_ident(self.text[k]) {
                k += 1;
                continue;
            }
            let start = k;
            k += 1;
            while k < self.text.len() && is_ident(self.text[k]) && !self.gap[k] && !self.string[k] {
                k += 1;
            }
            out.push((start, k));
        }
        out
    }

    fn word(&self, (start, end): (usize, usize)) -> String {
        self.text[start..end].iter().collect()
    }

    /// Fin (exclue) des arguments ouverts juste avant `open`, parenthèses des chaînes ignorées.
    fn close(&self, open: usize) -> usize {
        let mut depth = 1;
        let mut k = open;
        while k < self.text.len() {
            if !self.string[k] {
                match self.text[k] {
                    '(' => depth += 1,
                    ')' if depth == 1 => return k,
                    ')' => depth -= 1,
                    _ => {}
                }
            }
            k += 1;
        }
        k
    }

    /// Fin du premier argument de niveau 0 entre `open` et `end`.
    fn first_arg(&self, open: usize, end: usize) -> usize {
        let mut depth = 0;
        for k in open..end {
            if self.string[k] {
                continue;
            }
            match self.text[k] {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 0 => return k,
                _ => {}
            }
        }
        end
    }

    fn slice(&self, start: usize, end: usize) -> String {
        self.text[start..end].iter().collect()
    }
}

/// Retire commentaires et blancs, retire `r#` devant un identifiant, marque les littéraux.
fn normalize(source: &str) -> Src {
    let s: Vec<char> = source.chars().collect();
    let mut src = Src::default();
    let (mut i, mut line, mut gap) = (0, 1, true);
    let at = |k: usize| s.get(k).copied();
    while i < s.len() {
        let c = s[i];
        let prev_ident = i > 0 && is_ident(s[i - 1]);
        if c.is_whitespace() {
            line += usize::from(c == '\n');
            gap = true;
            i += 1;
        } else if c == '/' && at(i + 1) == Some('/') {
            while i < s.len() && s[i] != '\n' {
                i += 1;
            }
            gap = true;
        } else if c == '/' && at(i + 1) == Some('*') {
            let mut depth = 0;
            while i < s.len() {
                if s[i] == '/' && at(i + 1) == Some('*') {
                    depth += 1;
                    i += 2;
                } else if s[i] == '*' && at(i + 1) == Some('/') {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    line += usize::from(s[i] == '\n');
                    i += 1;
                }
            }
            gap = true;
        } else if c == 'r'
            && !prev_ident
            && at(i + 1) == Some('#')
            && at(i + 2).is_some_and(|d| d.is_alphabetic() || d == '_')
        {
            i += 2; // r#ident : l'identifiant seul compte
        } else if c == 'r'
            && (!prev_ident || (matches!(s[i - 1], 'b' | 'c') && (i < 2 || !is_ident(s[i - 2]))))
            && at(i + 1 + s[i + 1..].iter().take_while(|&&h| h == '#').count()) == Some('"')
        {
            // Chaîne brute r#"…"# : jusqu'au guillemet suivi du même nombre de #.
            let hashes = s[i + 1..].iter().take_while(|&&h| h == '#').count();
            let start = i;
            i += hashes + 2;
            while i < s.len()
                && !(s[i] == '"'
                    && s[i + 1..]
                        .iter()
                        .take(hashes)
                        .filter(|&&h| h == '#')
                        .count()
                        == hashes)
            {
                i += 1;
            }
            i = (i + 1 + hashes).min(s.len());
            for &d in &s[start..i] {
                line += usize::from(d == '\n');
                src.push(d, line, gap, true);
                gap = false;
            }
        } else if c == '"' || c == '\'' {
            // Chaîne "…" (échappements compris), ou caractère 'x' / '\…' ; sinon durée de vie.
            let len = if c == '"' {
                let mut k = i + 1;
                while k < s.len() && s[k] != '"' {
                    k += if s[k] == '\\' { 2 } else { 1 };
                }
                Some(k + 1 - i)
            } else if at(i + 1) == Some('\\') {
                s.get(i + 3..)
                    .and_then(|r| r.iter().position(|&d| d == '\''))
                    .map(|p| p + 4)
            } else if at(i + 2) == Some('\'') {
                Some(3)
            } else {
                None
            };
            let end = len.map_or(i + 1, |l| (i + l).min(s.len()));
            for &d in &s[i..end] {
                line += usize::from(d == '\n');
                src.push(d, line, gap, len.is_some());
                gap = false;
            }
            i = end;
        } else {
            src.push(c, line, gap, false);
            gap = false;
            i += 1;
        }
    }
    src
}

/// Numéros de ligne des formes refusées dans une crate de cœur :
/// - tout identifiant `disallowed_*` (lints d'interdiction, anciens noms compris), quel que soit
///   le contexte : attribut, argument de macro, accolades ;
/// - l'identifiant `include`, `include_str` ou `include_bytes`, partout, `use` compris ;
/// - `path = …` dans un attribut (`#[path]`, `cfg_attr(…, path = …)`) ;
/// - `allow(…)` ou `expect(…)` de `clippy::all`, `clippy::style` ou `warnings`, et tout `$` dans
///   les arguments d'un `allow`, `expect`, `warn` ou `deny` ;
/// - un prédicat de `cfg(…)`, `cfg!(…)` ou `cfg_attr(…)` hors de `test`, `feature = "…"`, `all`,
///   `any` et `not`.
pub fn clippy_exemptions(source: &str) -> Vec<usize> {
    let src = normalize(source);
    let mut found = Vec::new();
    let next = |end: usize, w: &str| {
        src.text[end..]
            .iter()
            .take(w.chars().count())
            .copied()
            .eq(w.chars())
            && !src.string.get(end).copied().unwrap_or(false)
    };
    for tok in src.tokens() {
        let (start, end) = tok;
        let word = src.word(tok);
        let prev = start.checked_sub(1).map(|p| src.text[p]);
        let path = word == "path" && next(end, "=") && matches!(prev, Some('[' | ',' | '('));
        let lint = matches!(word.as_str(), "allow" | "expect" | "warn" | "deny")
            && prev != Some('.')
            && next(end, "(")
            && {
                let close = src.close(end + 1);
                let args = src.slice(end + 1, close);
                let group = matches!(word.as_str(), "allow" | "expect")
                    && args.split(',').any(|a| FORBIDDEN_GROUPS.contains(&a));
                args.contains('$') || group
            };
        let cfg = match word.as_str() {
            "cfg" if next(end, "(") => Some((end + 1, src.close(end + 1))),
            "cfg" if next(end, "!(") => Some((end + 2, src.close(end + 2))),
            "cfg_attr" if next(end, "(") => {
                let close = src.close(end + 1);
                Some((end + 1, src.first_arg(end + 1, close)))
            }
            _ => None,
        };
        let cfg = cfg.is_some_and(|(open, close)| {
            src.tokens()
                .into_iter()
                .filter(|&(s, _)| s >= open && s < close)
                .any(|t| !CFG_IDENTS.contains(&src.word(t).as_str()))
        });
        if forbidden_ident(&word) || path || lint || cfg {
            found.push(src.line[start]);
        }
    }
    found.sort_unstable();
    found.dedup();
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flagged(form: &str) -> bool {
        !clippy_exemptions(form).is_empty()
    }

    #[test]
    fn direct_exemptions() {
        assert_eq!(
            clippy_exemptions("#[allow(clippy::disallowed_methods)]\nfn a() {}"),
            [1]
        );
        assert_eq!(
            clippy_exemptions("fn a() {}\n#![expect( clippy :: all )]"),
            [2]
        );
        assert!(flagged("#[cfg_attr(test, allow(dead_code, warnings))]"));
        assert!(flagged("#[allow(\n  clippy::style,\n)]"));
        assert!(flagged("#[allow(clippy::disallowed_type, reason = \"x\")]"));
        assert!(flagged("#[expect(clippy::disallowed_types)]"));
        assert!(flagged("#[allow(clippy::blacklisted_name)]"));
        assert!(flagged("const N: &str = stringify!(disallowed_macros);"));
    }

    /// Les quatre formes de la première relecture (I2).
    #[test]
    fn first_review_forms() {
        assert!(flagged("#[path = \"clock.inc\"] mod clock;"));
        assert!(flagged(
            "#[path = \"../../../tools/xtask/src/zz_clock.rs\"] mod clock2;"
        ));
        assert!(flagged("#[cfg(not(clippy))] pub fn now3() {}"));
        assert!(flagged(
            "macro_rules! quiet { ($($l:tt)*) => { #[allow($($l)*)] pub fn now4() {} } }"
        ));
    }

    /// Les variantes de la vérification du delta (I2-bis à I2-sexies), dans l'ordre de `i2.py`.
    #[test]
    fn delta_review_variants() {
        // v1 : code caché en profil de développement.
        assert!(flagged("#[cfg(not(debug_assertions))]\npub fn now5() {}"));
        // v2 : commentaire qui coupe cfg.
        assert!(flagged("#[cfg/**/(not(clippy))]\npub fn now6() {}"));
        // v3 : commentaire dans le nom du lint.
        assert!(flagged(
            "#[allow(clippy::/**/disallowed_methods)]\npub fn now7() {}"
        ));
        // v4 : include sous un autre nom.
        assert_eq!(
            clippy_exemptions("use std::include as inc;\ninc!(\"../clock_inc.txt\");"),
            [1]
        );
        // v5 : argument de macro entre accolades.
        let v5 = "macro_rules! q { ($a:meta) => { #[$a] pub fn now8() {} } }\n\
                  q!{allow(clippy::disallowed_methods)}";
        assert_eq!(clippy_exemptions(v5), [2]);
        // v6 et v7 : commentaire qui coupe path ou include.
        assert!(flagged("#[path/**/= \"../clock.inc\"]\npub mod c;"));
        assert!(flagged("include/**/!(\"../clock_inc.txt\");"));
        // v8 : #[cfg(feature = "dev-bench")] est un prédicat permis ; le code est vu par les
        // passes de clippy avec chaque fonctionnalité de développement (steps::dev_clippy_args).
        assert!(!flagged(
            "#[cfg(feature = \"dev-bench\")]\npub fn now9() {}"
        ));
        // v9 : identifiant brut.
        assert!(flagged(
            "#[allow(clippy::r#disallowed_methods)]\npub fn now10() {}"
        ));
    }

    /// Une variante de plus par règle.
    #[test]
    fn one_more_variant_per_rule() {
        assert!(flagged("q!{ allow(clippy::all) }")); // groupe, après une accolade
        assert!(flagged("#[r#allow(warnings)]")); // r# sur l'attribut
        assert!(flagged("#[deny($lint)] fn a() {}")); // $ dans deny
        assert!(flagged("#[cfg(target_os = \"linux\")] fn a() {}")); // prédicat target_*
        assert!(flagged("if cfg!(doc) {}")); // cfg!
        assert!(flagged(
            "#[cfg_attr(debug_assertions, allow(dead_code))] fn a() {}"
        ));
        assert!(flagged("#[cfg_attr(test, path = \"x.rs\")] mod x;")); // path sous cfg_attr
        assert!(flagged("use std::include_bytes;")); // include_bytes dans un use
        assert!(flagged("/* a /* b */ c */ #[allow(warnings)] fn a() {}")); // après un commentaire imbriqué
    }

    #[test]
    fn ignores_comments_strings_and_legitimate_code() {
        let ok = [
            "// #[allow(warnings)] disallowed_methods include!(x)",
            "/// doc : #[cfg(clippy)]\n//! include_str!\nfn a() {}",
            "/* /* #[allow(warnings)] */ disallowed_methods */ fn a() {}",
            "const S: &str = \"include!( #[allow(warnings)] disallowed_methods cfg(clippy)\";",
            "const R: &str = r#\"cfg(clippy) \"disallowed_types\" \"#;",
            "const C: char = '\"'; const D: char = '\\''; fn f<'a>(x: &'a str) -> &'a str { x }",
            "#[allow(dead_code)]\nfn a() { x.expect(\"warnings\"); y.allow(clippy::all); }",
            "#[expect(clippy::unwrap_used)]\n#[cfg(test)]\nmod tests {}",
            "#[cfg(all(feature = \"public\", not(any(test, feature = \"dev-bench\"))))] fn a() {}",
            "#[cfg_attr(test, allow(clippy::unwrap_used))]\nfn path(p: u8) { includes(p); }",
            "pub const P: &str = if cfg!(feature = \"public\") { \"public\" } else { \"dev\" };",
        ];
        for form in ok {
            assert!(clippy_exemptions(form).is_empty(), "{form}");
        }
    }
}
