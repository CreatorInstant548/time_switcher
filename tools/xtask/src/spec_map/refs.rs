//! Renvois `spec:` et marques de valeur provisoire dans le code : repérage des commentaires, puis grammaire
//! d'un renvoi.
//! spec: carnet architecture-securite.md §7 ; §9.7

use super::doc::{id_prefix, is_number, is_suffix, split_id};

#[derive(Clone, Copy)]
struct Style {
    /// `//`, `//!` et `///` : Rust, C#, RON, TypeScript.
    slash: bool,
    /// `#` : Python, TOML, YAML, shell, PowerShell et fichiers sans extension.
    hash: bool,
    /// `--` : SQL.
    dashes: bool,
    /// Caractère d'échappement dans une chaîne entre guillemets (PowerShell : l'accent grave).
    escape: u8,
}

#[derive(Debug, PartialEq)]
pub enum Hit {
    /// Texte du renvoi, après `spec:`.
    Spec(String),
    /// Texte de la marque, à partir du mot qui la porte.
    Mark(String),
    /// Commentaire qui ressemble à un renvoi sans en être un : texte et raison.
    Bad(String, String),
}

#[derive(Debug, PartialEq)]
pub struct Found {
    pub line: usize,
    pub hit: Hit,
}

fn style(path: &str) -> Option<Style> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let ext = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => ext.to_ascii_lowercase(),
        _ => String::new(),
    };
    let (slash, hash, dashes, escape) = match ext.as_str() {
        "rs" | "cs" | "ron" | "ts" | "tsx" | "js" | "mjs" | "glsl" | "wgsl" | "hlsl"
        | "gdshader" | "frag" | "vert" | "shader" => (true, false, false, b'\\'),
        "ps1" | "psm1" | "psd1" => (false, true, false, b'`'),
        "" | "py" | "toml" | "yml" | "yaml" | "sh" | "gd" => (false, true, false, b'\\'),
        "sql" => (false, false, true, b'\\'),
        _ => return None,
    };
    Some(Style {
        slash,
        hash,
        dashes,
        escape,
    })
}

/// Extensions de fichiers suivis qui ne peuvent porter ni renvoi ni marque (données, binaires,
/// fichiers générés). Liste fermée : tout autre type inconnu fait échouer `check`, pour qu'un
/// nouveau type de fichier soit classé (analysé dans `style`, ou ici) au lieu d'être sauté en
/// silence.
const NOT_SCANNED: [&str; 30] = [
    "lock", "json", "txt", "md", "png", "jpg", "jpeg", "gif", "webp", "svg", "ico", "bmp",
    "parquet", "zip", "gz", "zst", "bin", "pdf", "woff", "woff2", "ttf", "otf", "wav", "ogg",
    "mp3", "glb", "gltf", "exr", "hdr", "onnx",
];

/// Le type du fichier est connu : analysé, ou classé parmi ceux qui ne portent rien.
pub fn known_type(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    let ext = name.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase());
    style(path).is_some() || ext.is_some_and(|e| NOT_SCANNED.contains(&e.as_str()))
}

/// Un fichier de ce type peut porter des renvois : les autres ne sont même pas lus.
pub fn scannable(path: &str) -> bool {
    style(path).is_some()
}

/// Nature d'un commentaire sur sa ligne.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    /// `//`, `#` ou `--`.
    No,
    /// `/* … */` fermé sur la ligne.
    Closed,
    /// `/* …` ouvert : les lignes suivantes en font partie jusqu'à `*/`.
    Open,
}

/// Texte d'un commentaire de bloc : sans `*` ni `!` de tête ou de queue (`/** … */`, ` * …`).
fn block_text(s: &str) -> &str {
    s.trim()
        .trim_start_matches(['*', '!'])
        .trim()
        .trim_end_matches('*')
        .trim()
}

/// Une apostrophe au milieu d'un mot (`l'étape`) n'ouvre pas une chaîne, sauf après un préfixe de
/// chaîne de Python (`r'…'`, `f'…'`).
fn apostrophe_in_word(b: &[u8], i: usize) -> bool {
    let word = |c: u8| c.is_ascii_alphanumeric() || c >= 0x80;
    let prefix = matches!(
        b[i - 1],
        b'r' | b'R' | b'b' | b'B' | b'f' | b'F' | b'u' | b'U'
    ) && (i < 2 || !word(b[i - 2]));
    word(b[i - 1]) && !prefix
}

/// Premier commentaire de la ligne hors d'une chaîne : son texte, s'il est seul sur la ligne, et
/// sa nature.
fn comment(line: &str, st: Style) -> Option<(&str, bool, Block)> {
    let b = line.as_bytes();
    let (mut quoted, mut single, mut i) = (false, false, 0);
    while i < b.len() {
        let alone = || line[..i].trim().is_empty();
        match b[i] {
            c if c == st.escape && quoted => i += 1,
            b'"' if !single => quoted = !quoted,
            // Littéral de caractère de Rust : '"', 'x' ou '\"'.
            b'\'' if st.slash && !quoted => {
                if b.get(i + 2) == Some(&b'\'') {
                    i += 2;
                } else if b.get(i + 1) == Some(&b'\\') && b.get(i + 3) == Some(&b'\'') {
                    i += 3;
                }
            }
            b'\'' if (st.hash || st.dashes) && !quoted => {
                if single || i == 0 || !apostrophe_in_word(b, i) {
                    single = !single;
                }
            }
            b'/' if !quoted && st.slash && b.get(i + 1) == Some(&b'/') => {
                let text = line[i + 2..].trim_start_matches(['/', '!']).trim();
                return Some((text, alone(), Block::No));
            }
            b'/' if !quoted && st.slash && b.get(i + 1) == Some(&b'*') => {
                let rest = &line[i + 2..];
                return Some(match rest.find("*/") {
                    Some(end) => (block_text(&rest[..end]), alone(), Block::Closed),
                    None => (block_text(rest), alone(), Block::Open),
                });
            }
            b'-' if !quoted && !single && st.dashes && b.get(i + 1) == Some(&b'-') => {
                let text = line[i + 2..].trim_start_matches('-').trim();
                return Some((text, alone(), Block::No));
            }
            b'#' if !quoted
                && !single
                && st.hash
                && (i == 0 || b[i - 1].is_ascii_whitespace())
                && b.get(i + 1) != Some(&b'!') =>
            {
                return Some((
                    line[i + 1..].trim_start_matches('#').trim(),
                    alone(),
                    Block::No,
                ));
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Mots qui marquent une valeur provisoire : l'ancienne forme et la forme anglaise.
const MARK_WORDS: [&str; 2] = ["PROVISOIRE", "PROVISIONAL"];

/// Début de la première marque du commentaire : un de `MARK_WORDS`, en mot entier.
fn find_mark(c: &str) -> Option<usize> {
    let is_word = |ch: char| ch.is_alphanumeric() || ch == '_';
    MARK_WORDS
        .iter()
        .filter_map(|w| {
            c.match_indices(w)
                .find(|&(at, _)| {
                    let before = c[..at].chars().next_back();
                    let after = c[at + w.len()..].chars().next();
                    !before.is_some_and(is_word) && !after.is_some_and(is_word)
                })
                .map(|(at, _)| at)
        })
        .min()
}

/// Renvois (commentaire `//`, `#` ou `--` seul sur la ligne, qui commence par `spec:`) et marques
/// provisoires (commentaire seul, en fin de ligne ou de bloc `/* */`, qui contient un des mots de
/// `MARK_WORDS` : la marque doit porter un numéro de remontée). Un commentaire qui ressemble à
/// un renvoi sans en être un est signalé. Un BOM UTF-8 en tête du fichier est ignoré.
pub fn scan(path: &str, text: &str) -> Vec<Found> {
    let Some(st) = style(path) else {
        return Vec::new();
    };
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut found = Vec::new();
    let mut in_block = false;
    for (i, line) in text.lines().enumerate() {
        let (c, alone, block) = if in_block {
            match line.find("*/") {
                Some(end) => (block_text(&line[..end]), true, Block::Closed),
                None => (block_text(line), true, Block::Open),
            }
        } else {
            match comment(line, st) {
                Some(x) => x,
                None => continue,
            }
        };
        in_block = block == Block::Open;
        let near_miss = c
            .to_ascii_lowercase()
            .strip_prefix("spec")
            .is_some_and(|r| r.trim_start().starts_with(':'));
        let hit = match c.strip_prefix("spec:") {
            _ if block != Block::No && near_miss => Hit::Bad(
                c.to_owned(),
                "renvoi spec: dans un commentaire de bloc : utiliser un commentaire de ligne"
                    .to_owned(),
            ),
            Some(rest) if alone => Hit::Spec(rest.trim().to_owned()),
            Some(_) => Hit::Bad(
                c.to_owned(),
                "renvoi spec: en fin de ligne : le mettre dans un commentaire seul sur sa ligne"
                    .to_owned(),
            ),
            None if near_miss => Hit::Bad(
                c.to_owned(),
                "forme proche de spec: (minuscules, deux-points collés)".to_owned(),
            ),
            None => match find_mark(c) {
                Some(at) => Hit::Mark(c[at..].to_owned()),
                None => continue,
            },
        };
        found.push(Found { line: i + 1, hit });
    }
    found
}

/// Numéro de la remontée d'une marque (un entier non nul, comme une issue GitHub) :
/// forme anglaise `design #<n>`, ou ancienne forme `conception #<n>` (voir le message d'erreur).
pub fn mark(c: &str) -> Result<u64, String> {
    let bad = || {
        "PROVISOIRE sans numéro de remontée (attendu : PROVISIONAL(design #<n>), ou l'ancienne \
         forme PROVISOIRE(conception #<n>))"
            .to_owned()
    };
    let rest = ["PROVISIONAL(design #", "PROVISOIRE(conception #"]
        .iter()
        .find_map(|p| c.strip_prefix(p))
        .ok_or_else(bad)?;
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    if !rest[digits..].starts_with(')') {
        return Err(bad());
    }
    match rest[..digits].parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(bad()),
    }
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

/// Un élément (entre deux `,` ou `;`) fait seulement de parenthèses serait ignoré : `§3` y
/// disparaîtrait sans être vérifié. Refusé : le citer sans parenthèses.
fn lone_parens(s: &str) -> Result<(), String> {
    let (mut depth, mut outside, mut inside) = (0_u32, false, false);
    for ch in s.chars().chain([';']) {
        match ch {
            '(' => {
                depth += 1;
                inside |= depth == 1;
            }
            ')' => depth = depth.saturating_sub(1),
            ',' | ';' if depth == 0 => {
                if inside && !outside {
                    return Err(
                        "élément seul entre parenthèses, ignoré : le citer sans parenthèses"
                            .to_owned(),
                    );
                }
                (outside, inside) = (false, false);
            }
            c if depth == 0 && !c.is_whitespace() => outside = true,
            _ => {}
        }
    }
    Ok(())
}

/// Grammaire d'un renvoi (H1) : `;` sépare les fichiers, `,` les éléments d'un fichier. Un segment
/// sans nom de fichier reprend celui du précédent.
pub fn parse(text: &str) -> Result<Vec<Segment>, String> {
    let stripped = strip_parens(text)?;
    lone_parens(text)?;
    let text = stripped;
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
