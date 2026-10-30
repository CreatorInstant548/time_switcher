//! `cargo xtask spec-map` : lien entre le code et la conception. Mode d'emploi : docs/dev/spec-map.md.
//! spec: carnet architecture-securite.md §7 ; §9.7

mod diff;
mod doc;
mod refs;
mod report;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::ExitCode;

use crate::steps::Ctx;
use crate::sys;
use doc::Doc;
use refs::{Hit, IdSpec, Item, Segment};

const CONCEPTION: &str = "docs/conception/";
const PILOTAGE: &str = "origin/pilotage";
/// Liste fermée des renvois faux hors de `tools/xtask/` (H3). Elle ne peut que diminuer.
const KNOWN_BAD: &str = "tools/xtask/src/spec_map/known_bad.txt";
const USAGE: &str =
    "usage : cargo xtask spec-map report [--json] [--file <nom>] | check | diff <révision>";

#[derive(Debug, Clone, PartialEq)]
struct Target {
    doc: String,
    section: Option<String>,
    id: Option<String>,
}

#[derive(Debug)]
struct Cite {
    file: String,
    line: usize,
    text: String,
    targets: Vec<Target>,
    errors: Vec<String>,
}

#[derive(Debug)]
struct Mark {
    file: String,
    line: usize,
    text: String,
    issue: Result<u64, String>,
}

/// Documents (conception et carnets), renvois et marques, tous triés.
#[derive(Debug)]
struct Model {
    docs: BTreeMap<String, Doc>,
    cites: Vec<Cite>,
    marks: Vec<Mark>,
}

/// Fichiers analysés : tout ce que Git suit, sauf `*.md` et `docs/` (archives comprises).
fn scanned(path: &str) -> bool {
    !path.starts_with("docs/") && !path.to_ascii_lowercase().ends_with(".md")
}

fn carnet_key(name: &str) -> String {
    format!("{PILOTAGE}:{name}")
}

/// Nom de carnet sûr : un nom de fichier `.md` simple, sans séparateur.
fn carnet_name_ok(name: &str) -> bool {
    name.ends_with(".md")
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'))
        && !name.starts_with(['.', '-'])
}

/// Fichier de conception désigné par un nom relatif à `docs/conception/` ou par un nom court.
fn resolve_name(docs: &BTreeMap<String, Doc>, name: &str) -> Result<String, String> {
    let exact = format!("{CONCEPTION}{name}");
    if docs.contains_key(&exact) {
        return Ok(exact);
    }
    let all = docs.keys().filter(|k| k.starts_with(CONCEPTION));
    let found: Vec<&String> = all
        .filter(|k| !name.contains('/') && k.rsplit('/').next() == Some(name))
        .collect();
    match found.as_slice() {
        [] => Err(format!("fichier introuvable : {name}")),
        [one] => Ok((*one).clone()),
        many => Err(format!(
            "nom court ambigu : {name} ({})",
            many.iter()
                .map(|k| k.trim_start_matches(CONCEPTION))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn doc_of(docs: &BTreeMap<String, Doc>, seg: &Segment) -> Result<String, String> {
    if !seg.carnet {
        return resolve_name(docs, &seg.file);
    }
    let key = carnet_key(&seg.file);
    if docs.contains_key(&key) {
        Ok(key)
    } else {
        Err(format!("carnet {} absent de {PILOTAGE}", seg.file))
    }
}

/// Vérifie un élément contre le document cible et rend les cibles qu'il désigne (H2).
fn check_item(doc: &Doc, path: &str, item: &Item, targets: &mut Vec<Target>) -> Result<(), String> {
    let sec = item.section.as_deref();
    if let Some(s) = sec
        && !doc.sections.contains_key(s)
    {
        return Err(format!("{path} : section §{s} introuvable"));
    }
    let at = sec.map_or(String::new(), |s| format!(" §{s}"));
    let target = |id: Option<String>| Target {
        doc: path.to_owned(),
        section: sec.map(str::to_owned),
        id,
    };
    match &item.ids {
        None => targets.push(target(None)),
        Some(IdSpec::One(id)) if doc.defines(id, sec) => targets.push(target(Some(id.clone()))),
        Some(IdSpec::One(id)) => return Err(format!("{path}{at} : identifiant {id} non défini")),
        Some(IdSpec::Range(lo, hi)) => {
            for end in [lo, hi] {
                if !doc.defines(end, sec) {
                    return Err(format!("{path}{at} : identifiant {end} non défini"));
                }
            }
            targets.extend(
                doc.range(lo, hi, sec)
                    .into_iter()
                    .map(|id| target(Some(id))),
            );
        }
    }
    Ok(())
}

/// Un renvoi sans élément (`// spec: combat.md`) cite le fichier seul.
fn resolve(docs: &BTreeMap<String, Doc>, segs: &[Segment]) -> (Vec<Target>, Vec<String>) {
    let (mut targets, mut errors) = (Vec::new(), Vec::new());
    for seg in segs {
        let path = match doc_of(docs, seg) {
            Ok(p) => p,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };
        if seg.items.is_empty() {
            targets.push(Target {
                doc: path,
                section: None,
                id: None,
            });
            continue;
        }
        for item in &seg.items {
            if let Err(e) = check_item(&docs[&path], &path, item, &mut targets) {
                errors.push(e);
            }
        }
    }
    (targets, errors)
}

/// Construit le modèle. `fetch` rend le texte d'un carnet de `origin/pilotage`, ou rien s'il est absent.
fn build(
    conception: BTreeMap<String, Doc>,
    sources: &[(String, String)],
    fetch: &dyn Fn(&str) -> Option<String>,
) -> Model {
    let mut docs = conception;
    let (mut parsed, mut marks) = (Vec::new(), Vec::new());
    for (file, text) in sources {
        for f in refs::scan(file, text) {
            match f.hit {
                Hit::Spec(t) => parsed.push((file, f.line, refs::parse(&t), t)),
                Hit::Mark(t) => marks.push(Mark {
                    file: file.clone(),
                    line: f.line,
                    issue: refs::mark(&t),
                    text: t,
                }),
            }
        }
    }
    for segs in parsed.iter().filter_map(|p| p.2.as_ref().ok()) {
        for name in segs.iter().filter(|s| s.carnet).map(|s| &s.file) {
            let key = carnet_key(name);
            if !docs.contains_key(&key)
                && carnet_name_ok(name)
                && let Some(text) = fetch(name)
            {
                docs.insert(key, doc::parse(&text));
            }
        }
    }
    let mut cites: Vec<Cite> = parsed
        .into_iter()
        .map(|(file, line, segs, text)| {
            let (targets, errors) = match segs {
                Ok(segs) => resolve(&docs, &segs),
                Err(e) => (Vec::new(), vec![e]),
            };
            Cite {
                file: file.clone(),
                line,
                text,
                targets,
                errors,
            }
        })
        .collect();
    cites.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    marks.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    Model { docs, cites, marks }
}

/// Lecture d'un fichier du dépôt ; l'erreur ne donne que le chemin relatif (SEC0-26).
fn bytes(cx: &Ctx, rel: &str) -> Result<Vec<u8>, String> {
    cx.read(rel)
        .map_err(|_| format!("{rel} : lecture impossible"))
}

fn text_of(cx: &Ctx, rel: &str) -> Result<String, String> {
    String::from_utf8(bytes(cx, rel)?).map_err(|_| format!("{rel} n'est pas en UTF-8"))
}

/// Lit le dépôt par `Ctx::files()` (aucun lien suivi, H5) et le carnet par `git show` (H1).
/// Sans la conception ou sans `origin/pilotage`, c'est une erreur : jamais de réussite silencieuse (H6).
fn load(cx: &Ctx) -> Result<Model, String> {
    let files = cx.files()?;
    let mut docs = BTreeMap::new();
    for f in files
        .iter()
        .filter(|f| f.starts_with(CONCEPTION) && f.ends_with(".md"))
    {
        docs.insert(f.clone(), doc::parse(&text_of(cx, f)?));
    }
    if docs.is_empty() {
        return Err(format!("aucun document de conception sous {CONCEPTION}"));
    }
    let tip = format!("{PILOTAGE}^{{commit}}");
    let args = ["rev-parse", "--verify", "--quiet", "--end-of-options", &tip];
    if !sys::run_captured(&cx.root, "git", &args)?.0 {
        return Err(format!("{PILOTAGE} absent : git fetch origin pilotage"));
    }
    let mut sources = Vec::new();
    for f in files.iter().filter(|f| scanned(f)) {
        // Un fichier qui n'est pas en UTF-8 est binaire : il ne porte aucun renvoi.
        if let Ok(text) = String::from_utf8(bytes(cx, f)?) {
            sources.push((f.clone(), text));
        }
    }
    let fetch = |name: &str| {
        let spec = format!("{PILOTAGE}:{name}");
        let shown = sys::run_captured(&cx.root, "git", &["show", "--end-of-options", &spec]);
        shown.ok().filter(|(ok, _)| *ok).map(|(_, text)| text)
    };
    Ok(build(docs, &sources, &fetch))
}

/// Entrée de `known_bad.txt` : `fichier | ligne | texte | raison | #issue` (H3).
#[derive(Debug)]
struct Known {
    file: String,
    line: usize,
    text: String,
}

fn parse_known(text: &str) -> (Vec<Known>, Vec<String>) {
    let (mut known, mut errors) = (Vec::new(), Vec::new());
    let entries = text
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty() && !l.trim_start().starts_with('#'));
    for (i, l) in entries {
        let f: Vec<&str> = l.splitn(5, " | ").map(str::trim).collect();
        let line = f.get(1).and_then(|n| n.parse().ok());
        let issue_ok = f
            .get(4)
            .and_then(|n| n.strip_prefix('#'))
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()));
        let (Some(line), true) = (line, f.len() == 5 && issue_ok && !f[3].is_empty()) else {
            errors.push(format!(
                "known_bad.txt:{} : entrée illisible (fichier | ligne | texte | raison | #issue)",
                i + 1
            ));
            continue;
        };
        if f[0].starts_with("tools/xtask/") {
            errors.push(format!(
                "known_bad.txt:{} : {} se corrige dans le fichier, pas dans la liste",
                i + 1,
                f[0]
            ));
            continue;
        }
        known.push(Known {
            file: f[0].to_owned(),
            line,
            text: f[2].to_owned(),
        });
    }
    (known, errors)
}

/// Erreurs de `check` : renvois et marques faux hors de la liste fermée, et entrées périmées (H3).
fn evaluate(m: &Model, known_text: &str) -> (Vec<String>, usize) {
    let (known, mut errors) = parse_known(known_text);
    let mut used = BTreeSet::new();
    let cites = m
        .cites
        .iter()
        .filter(|c| !c.errors.is_empty())
        .map(|c| (&c.file, c.line, &c.text, c.errors.join(" ; ")));
    let marks = m
        .marks
        .iter()
        .filter_map(|k| Some((&k.file, k.line, &k.text, k.issue.as_ref().err()?.clone())));
    let mut findings: Vec<_> = cites.chain(marks).collect();
    findings.sort();
    let mut tolerated = 0;
    for (file, line, text, msg) in findings {
        let hit = known
            .iter()
            .position(|k| k.file == *file && k.line == line && k.text == *text);
        match hit {
            Some(i) => {
                used.insert(i);
                tolerated += 1;
            }
            None => errors.push(format!("{file}:{line} : {msg} [{text}]")),
        }
    }
    for (_, k) in known.iter().enumerate().filter(|(i, _)| !used.contains(i)) {
        errors.push(format!(
            "known_bad.txt : entrée périmée ({}:{}), le renvoi est corrigé ou a disparu : retirer la ligne",
            k.file, k.line
        ));
    }
    (errors, tolerated)
}

/// Étape `spec-map` de `check` : erreurs et note de réussite.
pub fn check(cx: &Ctx) -> Result<(Vec<String>, String), String> {
    let m = load(cx)?;
    let known = text_of(cx, KNOWN_BAD)?;
    let (errors, tolerated) = evaluate(&m, &known);
    let note = format!(
        "{} renvoi(s), {} marque(s) PROVISOIRE, {tolerated} renvoi(s) faux connu(s)",
        m.cites.len(),
        m.marks.len()
    );
    Ok((errors, note))
}

fn root() -> Result<PathBuf, String> {
    let here = std::env::current_dir().unwrap_or_default();
    let top = sys::stdout_of(&here, "git", &["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(top.trim()))
}

fn execute(args: &[String]) -> Result<ExitCode, String> {
    let words: Vec<&str> = args.iter().map(String::as_str).collect();
    let cx = || Ok::<_, String>(Ctx::new(root()?, false));
    match words.as_slice() {
        ["check"] => {
            let (errors, note) = check(&cx()?)?;
            errors.iter().for_each(|e| println!("  - {e}"));
            if errors.is_empty() {
                println!("OK ({note})");
                return Ok(ExitCode::SUCCESS);
            }
            println!("ÉCHEC : {} écart(s)", errors.len());
            Ok(ExitCode::FAILURE)
        }
        ["report", opts @ ..] => {
            let (mut json_out, mut only) = (false, None);
            let mut it = opts.iter();
            while let Some(&o) = it.next() {
                match o {
                    "--json" => json_out = true,
                    "--file" => only = Some(*it.next().ok_or(USAGE)?),
                    _ => return Err(format!("option inconnue : « {o} »\n{USAGE}")),
                }
            }
            let cx = cx()?;
            let m = load(&cx)?;
            let only = only.map(|n| resolve_name(&m.docs, n)).transpose()?;
            let value = report::report(&m, only.as_deref());
            if json_out {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
                );
            } else {
                print!("{}", report::markdown(&value));
            }
            Ok(ExitCode::SUCCESS)
        }
        ["diff", rev] => {
            print!("{}", diff::from_git(&cx()?, rev)?);
            Ok(ExitCode::SUCCESS)
        }
        _ => Err(USAGE.to_owned()),
    }
}

/// Point d'entrée de `cargo xtask spec-map …` : 0 si tout va bien, 1 sur un écart ou une erreur.
pub fn run(args: &[String]) -> ExitCode {
    match execute(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("ÉCHEC : {e}");
            ExitCode::FAILURE
        }
    }
}
