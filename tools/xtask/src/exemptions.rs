//! Code des crates de cœur que clippy ne verrait pas, ou qu'il laisserait passer (E6, F5, I2).
//! Fonctions pures, sans E/S.
//!
//! La source est d'abord **normalisée** comme rustc la lit : commentaires (`//`, `/* */` imbriqués,
//! doc comprise) retirés, préfixes `r#` des identifiants retirés, chaînes et caractères gardés
//! mais marqués. Seuls les motifs hors chaîne comptent. Les règles portent ensuite sur des
//! identifiants entiers, ce qui ferme les formes par commentaire, par macro et par accolades.

use std::collections::BTreeMap;

/// Names of the clippy lints that apply the `clippy.toml` bans, old names included
/// (`disallowed_method`, `disallowed_type`, `blacklisted_name`): refused wherever they appear.
/// Any other `disallowed_*` identifier is refused only when written as a clippy lint path
/// (`clippy::disallowed_…`), so that `fn disallowed_moves()` stays legitimate (m14).
const DISALLOWED_LINTS: [&str; 9] = [
    "disallowed_methods",
    "disallowed_method",
    "disallowed_types",
    "disallowed_type",
    "disallowed_macros",
    "disallowed_names",
    "disallowed_fields",
    "disallowed_script_idents",
    "blacklisted_name",
];
/// Words after which `!(` is a negation, not a macro call.
const NOT_A_MACRO: [&str; 21] = [
    "if", "while", "match", "return", "in", "else", "let", "break", "continue", "as", "mut", "ref",
    "move", "loop", "for", "unsafe", "async", "await", "yield", "where", "impl",
];
/// Code inclusion macros, refused when called, named in a path, imported by `use` or handed to
/// another macro as an argument (which could call them), but not as a plain name (m14).
const INCLUDES: [&str; 3] = ["include", "include_str", "include_bytes"];

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
pub(crate) struct Src {
    pub(crate) text: Vec<char>,
    pub(crate) line: Vec<usize>,
    gap: Vec<bool>,
    pub(crate) string: Vec<bool>,
    /// The character belongs to a raw identifier (`r#if`): its `r#` was removed, but it is not
    /// the keyword.
    raw: Vec<bool>,
    /// The next identifier is raw (its `r#` has just been removed).
    pending_raw: bool,
    /// Lines holding a `// SAFETY:` line comment with no code before it (not in a literal nor
    /// in a block comment), in increasing order.
    pub(crate) safety_lines: Vec<usize>,
}

impl Src {
    fn push(&mut self, c: char, line: usize, gap: bool, string: bool) {
        self.text.push(c);
        self.line.push(line);
        self.gap.push(gap);
        self.string.push(string);
        self.raw.push(self.pending_raw && is_ident(c));
        if !is_ident(c) {
            self.pending_raw = false;
        }
    }

    /// Identifiants hors chaîne : (début, fin exclue). Un blanc coupe un identifiant.
    pub(crate) fn tokens(&self) -> Vec<(usize, usize)> {
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

    pub(crate) fn word(&self, (start, end): (usize, usize)) -> String {
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

    /// The text is `s` just before `at`, outside any literal.
    fn follows(&self, at: usize, s: &str) -> bool {
        let n = s.chars().count();
        at >= n && self.text[at - n..at].iter().copied().eq(s.chars()) && !self.string[at - 1]
    }

    /// The delimiter at `k` opens a region of tokens that rustc hands to a macro, which this
    /// reader cannot follow: the arguments of a call `name!(…)` (blanks before the `!` are
    /// gone, so `k !(x)` is read like `k!(x)`), or the body of `macro_rules! name { … }`.
    /// A `!` after a keyword (`if !(x)`) is a negation, not a call. `ends` maps the end of
    /// each token to the token.
    fn opens_macro(&self, k: usize, ends: &BTreeMap<usize, (usize, usize)>) -> Option<bool> {
        if k >= 1 && self.text[k - 1] == '!' && !self.string[k - 1] {
            // A raw identifier (`r#if!(..)`) is a macro name, not the keyword.
            let call = ends
                .get(&(k - 1))
                .is_some_and(|&t| self.raw[t.0] || !NOT_A_MACRO.contains(&self.word(t).as_str()));
            return call.then_some(false);
        }
        let &(name, _) = ends.get(&k)?;
        let rules = name >= 1
            && self.text[name - 1] == '!'
            && ends
                .get(&(name - 1))
                .is_some_and(|&t| self.word(t) == "macro_rules");
        rules.then_some(true)
    }

    /// For each character: inside a region handed to a macro (see [`Src::opens_macro`]), and
    /// inside the body of a `macro_rules!`. Such a region is read strictly: a form that is only
    /// allowed outside it is refused inside.
    pub(crate) fn macro_regions(&self) -> (Vec<bool>, Vec<bool>) {
        let ends: BTreeMap<usize, (usize, usize)> =
            self.tokens().into_iter().map(|t| (t.1, t)).collect();
        // One entry per open delimiter: 0 an ordinary one, 1 the arguments of a call, 2 the body
        // of a `macro_rules!`.
        let (mut stack, mut calls, mut bodies) = (Vec::new(), 0_usize, 0_usize);
        let mut any = vec![false; self.text.len()];
        let mut rules = vec![false; self.text.len()];
        for k in 0..self.text.len() {
            let close = !self.string[k] && matches!(self.text[k], ')' | ']' | '}');
            if !self.string[k] && matches!(self.text[k], '(' | '[' | '{') {
                let kind = match self.opens_macro(k, &ends) {
                    Some(true) => 2,
                    Some(false) => 1,
                    None => 0,
                };
                stack.push(kind);
                calls += usize::from(kind == 1);
                bodies += usize::from(kind == 2);
            }
            (any[k], rules[k]) = (calls + bodies > 0, bodies > 0);
            if close {
                match stack.pop() {
                    Some(1) => calls -= 1,
                    Some(2) => bodies -= 1,
                    _ => {}
                }
            }
        }
        (any, rules)
    }

    /// Lines of the attributes assembled from a metavariable in a `macro_rules!` body: `#[` or
    /// `#![` then `$name` followed by anything but `]` (`#[$a(not($b))]`, `#[$k = $v]`,
    /// `#[$l($t::$g)]`, `#[$($t)*]`). `$(#[$m])*` forwards an attribute whole and stays allowed,
    /// and so does the matcher `#[$m:meta]`.
    fn metavariable_attributes(&self, rules: &[bool]) -> Vec<usize> {
        let t = &self.text;
        let mut lines = Vec::new();
        for k in (0..t.len()).filter(|&k| t[k] == '#' && !self.string[k] && rules[k]) {
            let mut j = k + 1;
            if t.get(j) == Some(&'!') {
                j += 1;
            }
            if t.get(j) != Some(&'[') || t.get(j + 1) != Some(&'$') {
                continue;
            }
            j += 2;
            let name = t[j..].iter().take_while(|&&c| is_ident(c)).count();
            j += name;
            let whole = name > 0 && t.get(j) == Some(&']');
            let matcher = name > 0 && t.get(j) == Some(&':') && {
                let frag = t[j + 1..].iter().take_while(|&&c| is_ident(c)).count();
                frag > 0 && t.get(j + 1 + frag) == Some(&']')
            };
            if !(whole || matcher) {
                lines.push(self.line[k]);
            }
        }
        lines
    }

    /// For each character: inside a `use` declaration, up to its closing `;`.
    fn in_use(&self) -> Vec<bool> {
        let mut out = vec![false; self.text.len()];
        let mut end = 0;
        for tok in self.tokens() {
            let generics = self.text.get(tok.1) == Some(&'<'); // `use<'a>`, precise capturing
            if tok.0 < end || self.word(tok) != "use" || generics {
                continue;
            }
            let (mut depth, mut k) = (0_i32, tok.1);
            while k < self.text.len() && !(self.text[k] == ';' && depth <= 0 && !self.string[k]) {
                if !self.string[k] {
                    depth += i32::from(self.text[k] == '{') - i32::from(self.text[k] == '}');
                }
                k += 1;
            }
            let stop = k.min(out.len());
            out[tok.0..stop].fill(true);
            end = k;
        }
        out
    }
}

/// The lines of code of a Rust source as rustc reads it: comments removed (block comments and
/// doc comments included), blanks removed, string and character literals left out. One entry per
/// source line that holds code. An attribute inside a comment or a literal is not in it (m2).
pub fn code_lines(source: &str) -> Vec<String> {
    let src = normalize(source);
    let mut lines: Vec<(usize, String)> = Vec::new();
    for k in (0..src.text.len()).filter(|&k| !src.string[k]) {
        match lines.last_mut() {
            Some((line, text)) if *line == src.line[k] => text.push(src.text[k]),
            _ => lines.push((src.line[k], src.text[k].to_string())),
        }
    }
    lines.into_iter().map(|(_, text)| text).collect()
}

/// Removes comments and blanks, removes `r#` before an identifier, marks the literals.
pub(crate) fn normalize(source: &str) -> Src {
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
            let start = i;
            while i < s.len() && s[i] != '\n' {
                i += 1;
            }
            // A real `// SAFETY:` line comment, with no code before it on its line (#91).
            let alone = src.line.last() != Some(&line);
            let compact: String = s[start..i].iter().filter(|c| !c.is_whitespace()).collect();
            if alone && compact.starts_with("//SAFETY:") {
                src.safety_lines.push(line);
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
            i += 2; // r#ident: only the identifier counts
            src.pending_raw = true;
        } else if c == 'r'
            && (!prev_ident || (matches!(s[i - 1], 'b' | 'c') && (i < 2 || !is_ident(s[i - 2]))))
            && at(i + 1 + s[i + 1..].iter().take_while(|&&h| h == '#').count()) == Some('"')
        {
            // Raw string r#"…"#: up to the quote followed by as many #.
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
            // String "…" (escapes included), or character 'x' / '\…'; else a lifetime.
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
/// - le nom d'un lint d'interdiction (`DISALLOWED_LINTS`, anciens noms compris), quel que soit le
///   contexte, et tout `clippy::disallowed_*` ;
/// - `include`, `include_str` ou `include_bytes` appelé (`!`), dans un chemin, dans un `use` ou
///   dans les arguments d'une macro ;
/// - `path = …` dans un attribut (`#[path]`, `cfg_attr(…, path = …)`, argument de macro) ;
/// - `allow(…)` ou `expect(…)` de `clippy::all`, `clippy::style` ou `warnings`, et tout `$` dans
///   les arguments d'un `allow`, `expect`, `warn` ou `deny` ;
/// - un prédicat de `cfg(…)` (sauf après un identifiant, `.` ou `::`, comme `fn cfg(level)`),
///   `cfg!(…)` ou `cfg_attr(…)` hors de `test`, `feature = "…"`, `all`, `any` et `not`.
pub fn clippy_exemptions(source: &str) -> Vec<usize> {
    let src = normalize(source);
    let mut found = Vec::new();
    let next = |end: usize, w: &str| next_is(&src, end, w);
    let ((in_macro, rules), in_use) = (src.macro_regions(), src.in_use());
    found.extend(src.metavariable_attributes(&rules));
    for tok in src.tokens() {
        let (start, end) = tok;
        let word = src.word(tok);
        let prev = start.checked_sub(1).map(|p| src.text[p]);
        // Inside a region handed to a macro, nothing is exempt: tokens there become whatever the
        // macro makes of them (`m!(x cfg(not(clippy)))` into `#[cfg(not(clippy))]`).
        let strict = in_macro[start];
        // Outside such a region, `cfg` after an identifier (`fn cfg(…)`), a `.` or a `::` names
        // something else.
        let named = !strict && prev.is_some_and(|c| is_ident(c) || c == '.' || c == ':');
        let inclusion = includes_code(&src, tok, &in_macro, &in_use);
        let lint = matches!(word.as_str(), "allow" | "expect" | "warn" | "deny")
            && (prev != Some('.') || strict)
            && next(end, "(")
            && {
                let close = src.close(end + 1);
                let args = src.slice(end + 1, close);
                let group = matches!(word.as_str(), "allow" | "expect")
                    && args.split(',').any(|a| FORBIDDEN_GROUPS.contains(&a));
                args.contains('$') || group
            };
        let cfg = match word.as_str() {
            "cfg" if next(end, "(") && !named => Some((end + 1, src.close(end + 1))),
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
        let disallowed = DISALLOWED_LINTS.contains(&word.as_str())
            || (word.starts_with("disallowed_") && src.follows(start, "clippy::"));
        if disallowed || inclusion || lint || cfg {
            found.push(src.line[start]);
        }
    }
    found.sort_unstable();
    found.dedup();
    found
}

/// The text after `end` starts with `w`, outside literals.
fn next_is(src: &Src, end: usize, w: &str) -> bool {
    src.text[end..]
        .iter()
        .take(w.chars().count())
        .copied()
        .eq(w.chars())
        && !src.string.get(end).copied().unwrap_or(false)
}

/// The token `tok` brings in code or data from another file: `include`, `include_str` or
/// `include_bytes` called (`!`), in a path, in a `use` or in the arguments of a macro, or
/// `path = …` in an attribute (`#[path]`, `cfg_attr(…, path = …)`, macro argument).
fn includes_code(src: &Src, tok: (usize, usize), in_macro: &[bool], in_use: &[bool]) -> bool {
    let (start, end) = tok;
    let word = src.word(tok);
    let prev = start.checked_sub(1).map(|p| src.text[p]);
    let strict = in_macro[start];
    let opens = matches!(prev, Some('[' | ',' | '(')) || strict;
    let path = word == "path" && next_is(src, end, "=") && opens;
    let include = INCLUDES.contains(&word.as_str())
        && (next_is(src, end, "!") || prev == Some(':') || in_use[start] || strict);
    path || include
}

/// Lines where `source` brings in another file (see [`includes_code`]), whose content a check
/// reading the `.rs` files of a crate would not see (#91, `ts_py`).
pub fn file_inclusions(source: &str) -> Vec<usize> {
    let src = normalize(source);
    let ((in_macro, _), in_use) = (src.macro_regions(), src.in_use());
    let mut found: Vec<usize> = src
        .tokens()
        .into_iter()
        .filter(|&tok| includes_code(&src, tok, &in_macro, &in_use))
        .map(|(start, _)| src.line[start])
        .collect();
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

    /// m12: a macro that puts an attribute received as an argument, called with braces.
    #[test]
    fn multi_step_macro_forms() {
        let m = "macro_rules! m { ($a:meta) => { #[$a] pub mod c; } }\n";
        assert!(flagged(&format!("{m}m!{{path = \"../clock.inc\"}}")));
        assert!(flagged(&format!("{m}m!{{ path = \"x.rs\" }}")));
        assert!(flagged(&format!("{m}m!(path = \"x.rs\")")));
        assert!(flagged(&format!("{m}m!{{cfg(clippy)}}")));
        // A plain assignment to a variable named `path`, at the start of a block, is not one.
        assert!(!flagged(
            "fn f() { let mut path = 1; if true { path = 2; } let _ = path; }"
        ));
    }

    /// m14: names that merely look like a ban are no longer refused.
    #[test]
    fn no_false_refusals() {
        for ok in [
            "fn disallowed_moves() {}",
            "fn f(include: bool) -> bool { include }",
            "fn f() { let include = true; if include { g(include); } }",
            "struct S { include: Vec<u8> }\nfn f(include: Vec<u8>) -> S { S { include } }",
            "fn f(x: &Set) -> bool { x.include(1) }",
            "fn cfg(level: u8) -> u8 { level }\nfn g(x: &T) -> u8 { x.cfg(5) }",
            "fn f() { let path = 1; let _ = path; }",
            "fn f(include: bool) { if !(include) { g(); } }",
            "fn f() -> impl Sized + use<'a> { let include = 1; include }",
            "fn includes() {}\nfn include_all() {}",
        ] {
            assert!(!flagged(ok), "{ok}");
        }
        // The exact lint names, and any `clippy::disallowed_*`, stay refused.
        for bad in [
            "fn disallowed_methods() {}",
            "#[allow(clippy::disallowed_whatever)] fn a() {}",
            "#[allow(clippy::r#disallowed_whatever)] fn a() {}",
            "use std::{vec, include};",
            "use std::{include, vec};",
            "pub use core::include_str;",
            "m!(include)",
            "m!{ include_bytes }",
            "let _ = include!(\"x.rs\");",
            "let _ = std::include_str!(\"x\");",
            "fn f() { g(cfg(x)); }",
            "m!(x; cfg(clippy))",
            "m!(a | cfg(clippy))",
            "m!({cfg(clippy)})",
            "fn f() { let x = cfg(5); }",
            "m!(a = cfg(clippy))",
        ] {
            assert!(flagged(bad), "{bad}");
        }
    }

    /// m2: only real code lines count; comments and literals do not.
    #[test]
    fn code_lines_skip_comments_and_literals() {
        let src = "/* a\n#![forbid(unsafe_code)]\n*/\n/// #![forbid(unsafe_code)]\n\
                   //! #![forbid(unsafe_code)]\nconst S: &str = \"#![forbid(unsafe_code)]\";\n\
                   #![forbid( unsafe_code )] /* x */\nfn f() {}\n";
        let lines = code_lines(src);
        let hits = lines
            .iter()
            .filter(|l| *l == "#![forbid(unsafe_code)]")
            .count();
        assert_eq!(hits, 1, "{lines:?}");
        assert!(lines.contains(&"constS:&str=;".to_owned()), "{lines:?}");
        assert!(code_lines("/* /* #![forbid(unsafe_code)] */ */").is_empty());
    }

    /// #91, review of this PR: a file that brings in another one is reported, so that the
    /// `unsafe_code` scan of `ts_py` reads all its code.
    #[test]
    fn file_inclusions_are_reported() {
        for (form, line) in [
            ("fn a() {}\ninclude!(\"evil.in\");", 2),
            ("#[path = \"../x.rs\"]\nmod x;", 1),
            ("#[cfg_attr(test, path = \"x.rs\")]\nmod x;", 1),
            ("const B: &[u8] = std::include_bytes!(\"x\");", 1),
            (
                "macro_rules! k { ($i:ident) => { $i!(\"x\"); } }\nk!(include);",
                2,
            ),
        ] {
            assert_eq!(file_inclusions(form), [line], "{form}");
        }
        for ok in [
            "fn path(include: u8) -> u8 { include }",
            "// include!(\"x\")\nconst S: &str = \"#[path = \\\"x\\\"]\";",
        ] {
            assert!(file_inclusions(ok).is_empty(), "{ok}");
        }
    }

    /// Review of PR #111: inside a macro call or a `macro_rules!` body, tokens are read strictly.
    #[test]
    fn macro_regions_are_read_strictly() {
        let def = "macro_rules! m { (x $a:meta) => { #[$a] fn f() -> u8 { 1 } } }\n";
        let item = "macro_rules! p { (x $a:meta) => { #[$a] pub mod c; } }\n";
        for bad in [
            format!("{def}m!(x cfg(not(clippy)));"),
            format!("{def}m!(x . cfg(not(clippy)));"),
            format!("{def}m!(fn cfg(not(clippy)));"),
            format!("{def}m ! (x cfg(not(clippy)));"),
            format!("{item}p!(x path = \"../clock.inc\");"),
            format!("{item}p![x path = \"../clock.inc\"];"),
            "macro_rules! n { ($b:tt) => { const S: &str = include_str $b (\"../x.inc\"); } }\nn!(!);"
                .to_owned(),
            "macro_rules! k { ($i:ident) => { const T: &str = $i!(\"../x.inc\"); } }\nk !(include_str);"
                .to_owned(),
            "macro_rules! q ( ($b:tt) => ( include_str $b (\"x\") ); );".to_owned(),
            format!("{def}m!(x . allow(clippy::all));"),
        ] {
            assert!(flagged(&bad), "{bad}");
        }
        // Outside a macro, and after a keyword, the same names stay legitimate.
        for ok in [
            "fn cfg(level: u8) -> u8 { level }",
            "fn f(include: bool) { if !(include) { g(); } }",
            "fn f(c: bool) -> bool { while !(c) { } return !(c); }",
            "fn f(path: u8) -> u8 { let mut path = path; path = 2; path }",
            "macro_rules! plain { ($x:expr) => { $x + 1 } }\nfn g() -> u8 { plain!(2) }",
            "fn disallowed_moves() {}",
        ] {
            assert!(!flagged(ok), "{ok}");
        }
    }

    /// Review of PR #111, round two: a raw identifier is a macro name, not the keyword.
    #[test]
    fn raw_keywords_are_macro_names() {
        for bad in [
            "r#if!(include_str)",
            "r#if!(x cfg(not(clippy)))",
            "r#as![x path = \"../c.inc\"]",
            "r#match!{include_bytes}",
            "r#while !(include_str)",
            "r#in!(x . allow(clippy::all))",
        ] {
            assert!(flagged(bad), "{bad}");
        }
        // The keyword itself is still a negation.
        assert!(!flagged("fn f(include: bool) { if !(include) { g(); } }"));
        assert!(flagged("#[allow(clippy::disallowed_fields)] fn a() {}"));
    }

    /// AS-56: in a `macro_rules!` body an attribute is forwarded whole or not assembled.
    #[test]
    fn attributes_assembled_from_metavariables_are_refused() {
        let body =
            |inner: &str| format!("macro_rules! m {{ ($a:meta, $b:meta) => {{ {inner} }} }}");
        for bad in [
            "#[$a(not($b))] fn f() {}",
            "#[$k = $v] mod c;",
            "#[$l($t::$g)] fn f() {}",
            "#![$a(x)]",
            "#[$($t)*] fn f() {}",
            "#[$a:meta = x] fn f() {}",
            "#[ $a ( x ) ] fn f() {}",
        ] {
            assert!(flagged(&body(bad)), "{bad}");
        }
        // Forwarded whole, matchers, plain attributes and the same text outside a macro body.
        for ok in [
            "$(#[$m])* fn f() {}",
            "#[$a] fn f() {}",
            "#[inline] fn f() {}",
            "#[derive(Debug)] struct S;",
        ] {
            assert!(!flagged(&body(ok)), "{ok}");
        }
        assert!(!flagged(
            "macro_rules! p { ($(#[$m:meta])* $v:vis fn $n:ident) => { $(#[$m])* $v fn $n() {} } }"
        ));
        assert!(!flagged(
            "fn f() { let a = 1; #[allow(dead_code)] let b = 2; }"
        ));
    }
}
