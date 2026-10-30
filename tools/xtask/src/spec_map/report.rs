//! Sortie de `spec-map report` : JSON, puis Markdown tiré du JSON.
//! spec: carnet architecture-securite.md §7 ; §9.7

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use super::{CONCEPTION, Model, Target};

/// Ordre naturel des clés de section : `2` avant `10`, `5.3` avant `5.3 bis`.
pub(super) fn natural(key: &str) -> (Vec<u64>, String) {
    let (num, sfx) = key.split_once(' ').unwrap_or((key, ""));
    let parts = num.split('.').filter_map(|p| p.parse().ok()).collect();
    (parts, sfx.to_owned())
}

pub(super) type Where = BTreeSet<(String, usize)>;

fn code(w: &Where) -> Vec<Value> {
    w.iter()
        .map(|(f, l)| json!({"file": f, "line": l}))
        .collect()
}

/// Rapport en JSON (schéma dans docs/dev/spec-map.md §3). `only` limite à un fichier de conception.
pub(super) fn report(m: &Model, only: Option<&str>) -> Value {
    let wanted = |t: &Target| only.is_none_or(|o| o == t.doc);
    let (mut sections, mut ids): (BTreeMap<_, Where>, BTreeMap<_, Where>) = Default::default();
    let mut references = Vec::new();
    for c in &m.cites {
        let targets: Vec<&Target> = c.targets.iter().filter(|t| wanted(t)).collect();
        if targets.is_empty() {
            continue;
        }
        for t in &targets {
            // Un renvoi à un identifiant situé dans une section compte pour les deux.
            let at = (c.file.clone(), c.line);
            if let Some(id) = &t.id {
                ids.entry((t.doc.clone(), id.clone()))
                    .or_default()
                    .insert(at.clone());
            }
            if let Some(s) = &t.section {
                sections
                    .entry((t.doc.clone(), s.clone()))
                    .or_default()
                    .insert(at);
            }
        }
        let list: Vec<Value> = targets
            .iter()
            .map(|t| json!({"doc": t.doc, "section": t.section, "id": t.id}))
            .collect();
        references.push(json!({"file": c.file, "line": c.line, "targets": list}));
    }
    let mut sections_cited: Vec<_> = sections.iter().collect();
    sections_cited.sort_by_key(|((d, k), _)| (d.clone(), natural(k)));
    let sections_cited: Vec<Value> = sections_cited
        .into_iter()
        .map(|((d, k), w)| {
            let title = m
                .docs
                .get(d)
                .and_then(|x| x.sections.get(k))
                .map(|s| s.title.clone());
            json!({"doc": d, "section": k, "title": title, "code": code(w)})
        })
        .collect();
    let identifiers: Vec<Value> = ids
        .iter()
        .map(|((d, id), w)| json!({"doc": d, "id": id, "code": code(w)}))
        .collect();
    let (mut bare, mut ids_bare) = (Vec::new(), 0);
    for (path, d) in m.docs.iter().filter(|(p, _)| p.starts_with(CONCEPTION)) {
        if only.is_some_and(|o| o != path) {
            continue;
        }
        let mut keys: Vec<_> = d.sections.iter().collect();
        keys.sort_by_key(|(k, _)| natural(k));
        for (k, s) in keys {
            if !sections.contains_key(&(path.clone(), k.clone())) {
                bare.push(json!({"doc": path, "section": k, "title": s.title}));
            }
        }
        ids_bare += d
            .ids
            .keys()
            .filter(|id| !ids.contains_key(&(path.clone(), (*id).clone())))
            .count();
    }
    let provisional: Vec<Value> = m
        .marks
        .iter()
        .map(|k| json!({"file": k.file, "line": k.line, "issue": k.issue.as_ref().ok(), "text": k.text}))
        .collect();
    json!({
        "schema": 1,
        "references": references,
        "sections": sections_cited,
        "identifiers": identifiers,
        "sections_without_code": bare,
        "identifiers_without_code": ids_bare,
        "provisional": provisional,
    })
}

fn place(v: &Value) -> String {
    format!("{}:{}", v["file"].as_str().unwrap_or_default(), v["line"])
}

pub(super) fn markdown(v: &Value) -> String {
    let list = |k: &str| v[k].as_array().cloned().unwrap_or_default();
    let places = |c: &Value| {
        let all: Vec<String> = c["code"].as_array().map_or(Vec::new(), |a| {
            a.iter().map(|x| format!("`{}`", place(x))).collect()
        });
        all.join(", ")
    };
    let mut out = String::from("# spec-map : rapport\n\n## Code vers conception\n\n");
    for r in list("references") {
        let to: Vec<String> = r["targets"]
            .as_array()
            .map_or(Vec::new(), |a| a.iter().map(target_label).collect());
        out += &format!("- `{}` : {}\n", place(&r), to.join(" ; "));
    }
    out += "\n## Conception vers code\n\n";
    for s in list("sections") {
        out += &format!(
            "- {} §{} {} : {}\n",
            s["doc"].as_str().unwrap_or_default(),
            s["section"].as_str().unwrap_or_default(),
            s["title"].as_str().unwrap_or_default(),
            places(&s)
        );
    }
    for i in list("identifiers") {
        out += &format!(
            "- {} {} : {}\n",
            i["doc"].as_str().unwrap_or_default(),
            i["id"].as_str().unwrap_or_default(),
            places(&i)
        );
    }
    let bare = list("sections_without_code");
    out += &format!("\n## Sections sans code ({})\n\n", bare.len());
    for s in &bare {
        out += &format!(
            "- {} §{} {}\n",
            s["doc"].as_str().unwrap_or_default(),
            s["section"].as_str().unwrap_or_default(),
            s["title"].as_str().unwrap_or_default()
        );
    }
    out += &format!(
        "\nIdentifiants sans code : {}\n",
        v["identifiers_without_code"]
    );
    let marks = list("provisional");
    out += &format!("\n## PROVISOIRE ({})\n\n", marks.len());
    for k in &marks {
        let issue = k["issue"]
            .as_u64()
            .map_or("SANS NUMÉRO".to_owned(), |n| format!("conception #{n}"));
        out += &format!("- `{}` : {issue}\n", place(k));
    }
    out
}

fn target_label(t: &Value) -> String {
    let get = |k: &str| t[k].as_str().map(str::to_owned);
    let parts = [
        get("doc"),
        get("section").map(|s| format!("§{s}")),
        get("id"),
    ];
    parts.into_iter().flatten().collect::<Vec<_>>().join(" ")
}
