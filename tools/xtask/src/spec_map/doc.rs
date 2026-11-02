//! Documents de conception : titres numérotés et identifiants définis, analysés à la main (aucune
//! bibliothèque Markdown).
//! spec: carnet architecture-securite.md §7 ; §9.7

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default)]
pub struct Section {
    pub title: String,
    /// Texte de la section, titre compris, jusqu'au titre numéroté suivant (sert à `diff`).
    pub body: String,
}

#[derive(Debug)]
pub struct IdDef {
    /// Clé de la dernière section numérotée au-dessus de la définition (vide si aucune).
    pub section: String,
    /// Ligne qui définit l'identifiant (sert à `diff`).
    pub text: String,
}

#[derive(Debug, Default)]
pub struct Doc {
    /// Clé de section (`3.1`, `5.3 bis`) vers section.
    pub sections: BTreeMap<String, Section>,
    /// Clés portées par plusieurs titres : un renvoi à l'une d'elles est ambigu.
    pub dups: BTreeSet<String>,
    pub ids: BTreeMap<String, Vec<IdDef>>,
}

impl Doc {
    fn in_section(def: &IdDef, sec: Option<&str>) -> bool {
        sec.is_none_or(|s| def.section == s || def.section.starts_with(&format!("{s}.")))
    }

    /// Un identifiant est défini s'il l'est dans le document, et dans la section citée (ou l'une
    /// de ses sous-sections) quand il y en a une (H2).
    pub fn defines(&self, id: &str, sec: Option<&str>) -> bool {
        self.ids
            .get(id)
            .is_some_and(|defs| defs.iter().any(|d| Self::in_section(d, sec)))
    }

    /// Identifiants définis de la plage `lo` à `hi` (même préfixe), triés.
    pub fn range(&self, lo: &str, hi: &str, sec: Option<&str>) -> Vec<String> {
        let (prefix, a) = split_id(lo);
        let (Ok(a), Ok(b)) = (a.parse::<u64>(), split_id(hi).1.parse::<u64>()) else {
            return Vec::new();
        };
        self.ids
            .iter()
            .filter(|(id, defs)| {
                let (p, n) = split_id(id);
                p == prefix
                    && n.parse::<u64>().is_ok_and(|n| (a..=b).contains(&n))
                    && defs.iter().any(|d| Self::in_section(d, sec))
            })
            .map(|(id, _)| id.clone())
            .collect()
    }
}

pub fn parse(text: &str) -> Doc {
    let mut doc = Doc::default();
    let (mut fence, mut current) = (false, String::new());
    // Points numérotés de premier niveau, cités comme des sous-sections `k.N` (voir `point`).
    let mut points: BTreeMap<String, Section> = BTreeMap::new();
    let (mut open, mut point_dups) = (None::<String>, BTreeSet::new());
    let mut html = false;
    for line in text.lines() {
        let t = line.trim_start();
        let mut id_section = open.clone().unwrap_or_else(|| current.clone());
        // A line that starts inside a comment `<!-- … -->` defines nothing: a title or an
        // identifier there is not part of the document (#18).
        let hidden = !fence && (html || t.starts_with("<!--"));
        if !fence {
            html = html_comment_open(line, html);
        }
        if hidden {
            // Only the text of the section grows, below.
        } else if t.starts_with("```") || t.starts_with("~~~") {
            fence = !fence;
        } else if !fence {
            if let Some(title) = heading(line) {
                open = None;
                if let Some(key) = section_key(title) {
                    // Le titre garde son texte, sans le numéro (et le suffixe) déjà dans la clé.
                    let words = key.split(' ').count();
                    let rest: Vec<&str> = title.split_whitespace().skip(words).collect();
                    if doc.sections.contains_key(&key) {
                        doc.dups.insert(key.clone());
                    }
                    doc.sections.entry(key.clone()).or_insert_with(|| Section {
                        title: rest.join(" "),
                        body: String::new(),
                    });
                    current = key;
                }
            } else {
                if let Some((n, title)) = point(line)
                    && !current.is_empty()
                    && !current.contains(' ')
                {
                    let key = format!("{current}.{n}");
                    if points.contains_key(&key) {
                        point_dups.insert(key.clone());
                    }
                    points.entry(key.clone()).or_insert_with(|| Section {
                        title,
                        body: String::new(),
                    });
                    id_section = key.clone();
                    open = Some(key);
                } else if !line.trim().is_empty() && !line.starts_with([' ', '\t']) {
                    open = None;
                }
                if let Some(id) = defined_id(line) {
                    doc.ids.entry(id).or_default().push(IdDef {
                        section: id_section,
                        text: line.trim().to_owned(),
                    });
                }
            }
        }
        if let Some(p) = open.as_ref().and_then(|k| points.get_mut(k)) {
            p.body.push_str(line);
            p.body.push('\n');
        }
        // Un titre sans numéro (« Phase 0 ») reste dans la section numérotée au-dessus.
        if let Some(s) = doc.sections.get_mut(&current) {
            s.body.push_str(line);
            s.body.push('\n');
        }
    }
    // Un vrai sous-titre `k.N` prime sur le point numéroté du même numéro, sans doublon.
    for (key, section) in points {
        if !doc.sections.contains_key(&key) {
            if point_dups.contains(&key) {
                doc.dups.insert(key.clone());
            }
            doc.sections.insert(key, section);
        }
    }
    doc
}

/// Le texte est, à la fin de `line`, à l'intérieur d'un commentaire HTML `<!-- … -->`, sachant
/// s'il l'était au début de la ligne.
fn html_comment_open(line: &str, mut open: bool) -> bool {
    let mut rest = line;
    loop {
        let (marker, next) = if open { ("-->", false) } else { ("<!--", true) };
        match rest.find(marker) {
            Some(at) => {
                open = next;
                rest = &rest[at + marker.len()..];
            }
            None => return open,
        }
    }
}

/// Point numéroté de premier niveau (`3. Texte`, sans retrait) : numéro et début du texte. La
/// conception cite ces points comme des sous-sections (`§2.3` pour le point 3 de la section 2).
fn point(line: &str) -> Option<(String, String)> {
    let n = line.bytes().take_while(u8::is_ascii_digit).count();
    let rest = line[n..]
        .strip_prefix(". ")
        .filter(|_| (1..=4).contains(&n))?;
    let title: String = rest.replace("**", "").trim().chars().take(80).collect();
    Some((line[..n].to_owned(), title))
}

fn heading(line: &str) -> Option<&str> {
    let n = line.bytes().take_while(|&b| b == b'#').count();
    if (1..=6).contains(&n) {
        line[n..].strip_prefix(' ').map(str::trim)
    } else {
        None
    }
}

/// `\d+(\.\d+)*` : `0`, `3.1`, `4.1.1`.
pub fn is_number(s: &str) -> bool {
    s.split('.')
        .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

/// Clé d'un titre numéroté : `2. Titre` → `2`, `5.3 bis Titre` → `5.3 bis`.
pub fn section_key(title: &str) -> Option<String> {
    let mut words = title.split_whitespace();
    let number = words.next()?.trim_end_matches('.');
    if !is_number(number) {
        return None;
    }
    // Une lettre seule n'est un suffixe que si le mot suivant ne commence pas par une minuscule :
    // `3.10 b Titre` est la section « 3.10 b », mais `3. a priori` est la section 3.
    let (first, next) = (words.next(), words.next());
    Some(match first {
        Some(s)
            if is_suffix(s)
                && (s.len() > 1 || next.is_none_or(|n| !n.starts_with(char::is_lowercase))) =>
        {
            format!("{number} {s}")
        }
        _ => number.to_owned(),
    })
}

/// `bis`, `ter` ou une lettre minuscule seule.
pub fn is_suffix(s: &str) -> bool {
    s == "bis" || s == "ter" || (s.len() == 1 && s.as_bytes()[0].is_ascii_lowercase())
}

/// Identifiant défini par la ligne : première cellule d'un tableau, ou gras en tête d'élément de
/// liste (case à cocher admise) ou de ligne (H2). Dans un tableau dont la première colonne est une
/// date (`| 30/10 | AS-33 | … |`), la deuxième cellule définit aussi un identifiant, mais
/// seulement si elle n'est faite que de lui, éventuellement entre `**` ou accents graves ; les
/// autres cellules n'en définissent jamais.
fn defined_id(line: &str) -> Option<String> {
    let t = line.trim_start();
    if let Some(row) = t.strip_prefix('|') {
        let mut cells = row.split('|');
        let first = cells.next()?.trim().trim_start_matches(['*', '`']);
        if let Some((id, _)) = id_prefix(first) {
            return Some(id);
        }
        let second = cells.next()?.trim().trim_matches(['*', '`']);
        return id_prefix(second)
            .filter(|(_, len)| *len == second.len())
            .map(|(id, _)| id);
    }
    let mut item = t;
    if let Some(rest) = ["- ", "* ", "+ "].iter().find_map(|m| item.strip_prefix(m)) {
        item = rest.trim_start();
    } else {
        let digits = item.bytes().take_while(u8::is_ascii_digit).count();
        if let Some(rest) = item[digits..].strip_prefix(". ").filter(|_| digits > 0) {
            item = rest.trim_start();
        }
    }
    if let Some(rest) = ["[ ]", "[x]", "[X]"]
        .iter()
        .find_map(|c| item.strip_prefix(c))
    {
        item = rest.trim_start();
    }
    id_prefix(item.strip_prefix("**")?).map(|(id, _)| id)
}

/// Identifiant en tête de `s` : `[A-Z][A-Z0-9]*(-[A-Z0-9]+)*-?\d+`, suivi d'un caractère qui n'est
/// ni alphanumérique ni `-`. Rend l'identifiant et sa longueur.
pub fn id_prefix(s: &str) -> Option<(String, usize)> {
    let end = s
        .bytes()
        .position(|b| !(b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'-'))
        .unwrap_or(s.len());
    let run = s[..end].trim_end_matches('-');
    let shaped = run.as_bytes().first().is_some_and(u8::is_ascii_uppercase)
        && run.as_bytes().last().is_some_and(u8::is_ascii_digit)
        && run.split('-').all(|p| !p.is_empty());
    let bounded = !s[run.len()..].starts_with(|c: char| c.is_alphanumeric() || c == '-');
    (shaped && bounded).then(|| (run.to_owned(), run.len()))
}

/// Préfixe et nombre final d'un identifiant : `SEC0-03` → (`SEC0-`, `03`).
pub fn split_id(id: &str) -> (&str, &str) {
    id.split_at(id.trim_end_matches(|c: char| c.is_ascii_digit()).len())
}
