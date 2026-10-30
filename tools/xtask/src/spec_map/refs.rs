//! Renvois `spec:` et marques PROVISOIRE dans le code : repérage des commentaires, puis grammaire
//! d'un renvoi.
//! spec: carnet architecture-securite.md §7 ; §9.7

use super::doc::{id_prefix, is_number, is_suffix, split_id};

#[derive(Clone, Copy)]
struct Style {
    /// `//`, `//!` et `///` : Rust, C#, RON, TypeScript.
    slash: bool,
    /// `#` : Python, TOML, YAML, shell, PowerShell et fichiers sans extension.
    hash: bool,
}

#[derive(Debug, PartialEq)]
pub enum Hit {
    /// Texte du renvoi, après `spec:`.
    Spec(String),
    /// Texte du commentaire, qui commence par PROVISOIRE.
    Mark(String),
}

#[derive(Debug, PartialEq)]
pub struct Found {
    pub line: usize,
    pub hit: Hit,
}

fn style(path: &str) -> Option<Style> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let ext = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => ext,
        _ => "",
    };
    let (slash, hash) = match ext {
        "rs" | "cs" | "ron" | "ts" | "tsx" | "js" | "mjs" => (true, false),
        "" | "py" | "toml" | "yml" | "yaml" | "sh" | "ps1" | "psm1" | "gd" => (false, true),
        _ => return None,
    };
    Some(Style { slash, hash })
}

/// Premier commentaire de la ligne hors d'une chaîne : son texte, et s'il est seul sur la ligne.
fn comment(line: &str, st: Style) -> Option<(&str, bool)> {
    let b = line.as_bytes();
    let mut quoted = false;
    for (i, &c) in b.iter().enumerate() {
        let alone = || line[..i].trim().is_empty();
        match c {
            b'"' if i == 0 || b[i - 1] != b'\\' => quoted = !quoted,
            b'/' if !quoted && st.slash && b.get(i + 1) == Some(&b'/') => {
                let text = line[i + 2..].trim_start_matches(['/', '!']).trim();
                return Some((text, alone()));
            }
            b'#' if !quoted
                && st.hash
                && (i == 0 || b[i - 1].is_ascii_whitespace())
                && b.get(i + 1) != Some(&b'!') =>
            {
                return Some((line[i + 1..].trim_start_matches('#').trim(), alone()));
            }
            _ => {}
        }
    }
    None
}

/// Renvois (commentaire seul sur la ligne, qui commence par `spec:`) et marques PROVISOIRE
/// (commentaire seul ou en fin de ligne, qui commence par PROVISOIRE). Un fichier d'un type
/// inconnu n'est pas analysé.
pub fn scan(path: &str, text: &str) -> Vec<Found> {
    let Some(st) = style(path) else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let Some((c, alone)) = comment(line, st) else {
            continue;
        };
        let hit = match c.strip_prefix("spec:") {
            Some(rest) if alone => Hit::Spec(rest.trim().to_owned()),
            _ if c.starts_with("PROVISOIRE") => Hit::Mark(c.to_owned()),
            _ => continue,
        };
        found.push(Found { line: i + 1, hit });
    }
    found
}

/// Numéro de la remontée d'une marque : `PROVISOIRE(conception #12)`.
pub fn mark(c: &str) -> Result<u64, String> {
    let bad = || "PROVISOIRE sans numéro de remontée (attendu : PROVISOIRE(conception #<n>))";
    let rest = c
        .strip_prefix("PROVISOIRE(conception #")
        .ok_or_else(|| bad().to_owned())?;
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 || !rest[digits..].starts_with(')') {
        return Err(bad().to_owned());
    }
    rest[..digits].parse().map_err(|_| bad().to_owned())
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdSpec {
    One(String),
    /// Bornes incluses, même préfixe.
    Range(String, String),
}

#[derive(Debug, PartialEq)]
pub struct Item {
    /// Clé de section citée : `3.1`, `5.3 bis`.
    pub section: Option<String>,
    pub ids: Option<IdSpec>,
}

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub carnet: bool,
    pub file: String,
    pub items: Vec<Item>,
}

/// Texte hors des parenthèses, qui sont ignorées (H1).
fn strip_parens(s: &str) -> Result<String, String> {
    let (mut depth, mut out) = (0_u32, String::new());
    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth = depth.checked_sub(1).ok_or("parenthèse fermante en trop")?,
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    if depth == 0 {
        Ok(out)
    } else {
        Err("parenthèse non fermée".to_owned())
    }
}

/// Grammaire d'un renvoi (H1) : `;` sépare les fichiers, `,` les éléments d'un fichier. Un segment
/// sans nom de fichier reprend celui du précédent.
pub fn parse(text: &str) -> Result<Vec<Segment>, String> {
    let text = strip_parens(text)?;
    let mut segs: Vec<Segment> = Vec::new();
    for part in text.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let mut words = part.split_whitespace().peekable();
        let carnet = words.next_if_eq(&"carnet").is_some();
        let named = words.next_if(|w| w.ends_with(".md"));
        let (carnet, file) = match (named, segs.last()) {
            (Some(f), _) => (carnet, f.to_owned()),
            (None, _) if carnet => return Err("« carnet » sans nom de fichier".to_owned()),
            (None, Some(prev)) => (prev.carnet, prev.file.clone()),
            (None, None) => return Err("aucun fichier cité".to_owned()),
        };
        let rest = words.collect::<Vec<_>>().join(" ");
        let mut prefix = None;
        let items = rest
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| parse_item(s, &mut prefix))
            .collect::<Result<_, _>>()?;
        segs.push(Segment {
            carnet,
            file,
            items,
        });
    }
    if segs.is_empty() {
        return Err("renvoi vide".to_owned());
    }
    Ok(segs)
}

/// `§n(.m)* [bis|ter|lettre] [ID [à ID|N]]`, ou `ID [à ID|N]`, ou `N` (préfixe repris).
fn parse_item(s: &str, prefix: &mut Option<String>) -> Result<Item, String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut rest = &words[..];
    let mut section = None;
    if let Some(num) = rest[0].strip_prefix('§') {
        if !is_number(num) {
            return Err(format!("section illisible : « {} »", rest[0]));
        }
        rest = &rest[1..];
        section = Some(match rest.first() {
            Some(&sfx) if is_suffix(sfx) => {
                rest = &rest[1..];
                format!("{num} {sfx}")
            }
            _ => num.to_owned(),
        });
    }
    let ids = match rest {
        [] => None,
        [one] => Some(IdSpec::One(id_ref(one, prefix)?)),
        [lo, "à", hi] => {
            let (lo, hi) = (id_ref(lo, prefix)?, id_ref(hi, prefix)?);
            let (a, b) = (split_id(&lo), split_id(&hi));
            let ordered =
                matches!((a.1.parse::<u64>(), b.1.parse::<u64>()), (Ok(x), Ok(y)) if x <= y);
            if a.0 != b.0 || !ordered {
                return Err(format!("plage incohérente : « {lo} à {hi} »"));
            }
            Some(IdSpec::Range(lo, hi))
        }
        _ => return Err(format!("élément illisible : « {s} »")),
    };
    Ok(Item { section, ids })
}

/// Identifiant entier, ou nombre seul qui reprend le préfixe du précédent (`SEC0-03, 06`).
fn id_ref(word: &str, prefix: &mut Option<String>) -> Result<String, String> {
    if word.bytes().all(|b| b.is_ascii_digit()) {
        let p = prefix
            .as_deref()
            .ok_or_else(|| format!("nombre « {word} » sans identifiant avant lui"))?;
        return Ok(format!("{p}{word}"));
    }
    match id_prefix(word) {
        Some((id, n)) if n == word.len() => {
            *prefix = Some(split_id(&id).0.to_owned());
            Ok(id)
        }
        _ => Err(format!("identifiant illisible : « {word} »")),
    }
}
