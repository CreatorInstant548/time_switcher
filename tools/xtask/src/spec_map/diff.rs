//! `spec-map diff <révision>` : code qui cite les sections de conception modifiées.
//! spec: carnet architecture-securite.md §7 ; §9.7

use std::collections::{BTreeMap, BTreeSet};

use super::doc::{self, Doc};
use super::report::Where;
use super::{CONCEPTION, Model, Target, load};
use crate::steps::Ctx;
use crate::sys;

/// Un diff de conception ne prend jamais une option pour une révision (H4).
pub(super) fn check_rev(rev: &str) -> Result<(), String> {
    if rev.is_empty() || rev.starts_with('-') || rev.contains(['\0', '\n']) {
        return Err(format!("révision refusée : « {rev} »"));
    }
    Ok(())
}

type Changes = (
    BTreeMap<(String, String), &'static str>,
    BTreeSet<(String, String)>,
);

/// Sections et identifiants dont le texte diffère entre deux états de la conception.
fn changes(old: &BTreeMap<String, Doc>, new: &BTreeMap<String, Doc>) -> Changes {
    let norm = |s: &str| {
        s.lines()
            .map(str::trim_end)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_owned()
    };
    let (mut secs, mut ids) = (BTreeMap::new(), BTreeSet::new());
    let empty = Doc::default();
    let paths: BTreeSet<&String> = old
        .keys()
        .chain(new.keys())
        .filter(|p| p.starts_with(CONCEPTION))
        .collect();
    for p in paths {
        let (o, n) = (old.get(p).unwrap_or(&empty), new.get(p).unwrap_or(&empty));
        for k in o.sections.keys().chain(n.sections.keys()) {
            let state = match (o.sections.get(k), n.sections.get(k)) {
                (None, Some(_)) => "ajoutée",
                (Some(_), None) => "supprimée",
                (Some(a), Some(b)) if norm(&a.body) != norm(&b.body) => "modifiée",
                _ => continue,
            };
            secs.insert((p.clone(), k.clone()), state);
        }
        let text = |d: &Doc, id: &String| {
            d.ids
                .get(id)
                .map(|v| v.iter().map(|x| norm(&x.text)).collect::<Vec<_>>())
        };
        for id in o.ids.keys().chain(n.ids.keys()) {
            if text(o, id) != text(n, id) {
                ids.insert((p.clone(), id.clone()));
            }
        }
    }
    (secs, ids)
}

/// Code qui cite les sections ou identifiants modifiés ; une section citée couvre ses sous-sections.
pub(super) fn render_diff(m: &Model, old: &BTreeMap<String, Doc>, rev: &str) -> String {
    let (secs, ids) = changes(old, &m.docs);
    let mut out = format!("# spec-map diff {rev}\n\n");
    let mut with_code = 0;
    let mut heads: Vec<(String, Where)> = Vec::new();
    for ((d, k), state) in &secs {
        let title = m
            .docs
            .get(d)
            .and_then(|x| x.sections.get(k))
            .map_or("", |s| s.title.as_str());
        let covers = |s: &String| s == k || k.starts_with(&format!("{s}."));
        let w: Where = m
            .cites
            .iter()
            .filter(|c| {
                let cites_it = |t: &&Target| t.doc == *d && t.id.is_none();
                c.targets
                    .iter()
                    .filter(cites_it)
                    .any(|t| t.section.as_ref().is_some_and(covers))
            })
            .map(|c| (c.file.clone(), c.line))
            .collect();
        heads.push((format!("{d} §{k} {title} ({state})"), w));
    }
    for (d, id) in &ids {
        let w: Where = m
            .cites
            .iter()
            .filter(|c| {
                c.targets
                    .iter()
                    .any(|t| t.doc == *d && t.id.as_ref() == Some(id))
            })
            .map(|c| (c.file.clone(), c.line))
            .collect();
        heads.push((format!("{d} {id} (identifiant modifié)"), w));
    }
    for (head, w) in heads.iter().filter(|h| !h.1.is_empty()) {
        with_code += 1;
        out += &format!("## {head}\n\n");
        w.iter().for_each(|(f, l)| out += &format!("- `{f}:{l}`\n"));
        out.push('\n');
    }
    out += &format!(
        "{} changement(s) de conception, dont {with_code} cité(s) par du code.\n",
        heads.len()
    );
    out
}

pub(super) fn from_git(cx: &Ctx, rev: &str) -> Result<String, String> {
    check_rev(rev)?;
    let m = load(cx)?;
    let spec = format!("{rev}^{{commit}}");
    let sha = sys::stdout_of(
        &cx.root,
        "git",
        &["rev-parse", "--verify", "--end-of-options", &spec],
    )?;
    let sha = sha.trim();
    let listed = sys::stdout_of(
        &cx.root,
        "git",
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            "--end-of-options",
            sha,
            "--",
            "docs/conception",
        ],
    )?;
    let mut old = BTreeMap::new();
    for path in listed.split('\0').filter(|p| p.ends_with(".md")) {
        let blob = format!("{sha}:{path}");
        let text = sys::stdout_of(&cx.root, "git", &["show", "--end-of-options", &blob])?;
        old.insert(path.to_owned(), doc::parse(&text));
    }
    Ok(render_diff(&m, &old, &sha[..sha.len().min(12)]))
}
