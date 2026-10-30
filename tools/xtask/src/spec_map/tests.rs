//! Tests de spec-map sur un petit corpus de documents et de code factices, tous en mémoire.
//! spec: carnet architecture-securite.md §7 ; §9.7

use std::collections::BTreeMap;

use super::diff::{check_rev, render_diff};
use super::doc::{self, Doc};
use super::refs::{self, Hit, IdSpec, Item};
use super::report::{markdown, natural, report};
use super::*;

const COMBAT: &str = "docs/conception/domaines/combat.md";
const SECURITE: &str = "docs/conception/revues/securite.md";

fn text(lines: &[&str]) -> String {
    lines.join("\n")
}

fn combat() -> String {
    text(&[
        "# Combat",
        "## 1. Principes",
        "### 1.1 Geste",
        "| D1 | Décision un | oui |",
        "|---|---|---|",
        "- **D2.** Deux",
        "- [ ] **D3** Trois",
        "**E4** gras en tête de ligne",
        "```",
        "## 9. Faux titre dans un bloc de code",
        "- **D99** faux identifiant dans un bloc de code",
        "```",
        "### 1.2 bis Rare",
        "## 2. Suite",
        "### Phase 0 — sans numéro",
        "- **D9** neuf",
        "| F1 | un | |",
        "| F2 | deux | |",
        "| F3 | trois | |",
        "| F4 | quatre | |",
    ])
}

fn securite() -> String {
    text(&[
        "# Sécurité",
        "## 4. Surfaces",
        "### 4.3 Commandes",
        "- **S8 — Matrice des commandes** (décidé)",
        "## 6. Décisions",
        "| S9 | autre | |",
    ])
}

fn carnet() -> String {
    text(&[
        "# Carnet",
        "## 3. Exigences",
        "- [ ] **SEC0-03** (révisé) — un",
        "- [ ] **SEC0-06** — deux",
        "- [ ] **SEC0-29** — trois",
        "- [ ] **SEC0-30** — quatre",
        "- [ ] **SEC0-31** — cinq",
        "## 9. Avis",
        "### 9.5 Consigne",
        "- **F1. Premier**",
    ])
}

fn conception() -> BTreeMap<String, Doc> {
    let mut docs = BTreeMap::new();
    for (path, body) in [
        (COMBAT, combat()),
        (SECURITE, securite()),
        ("docs/conception/domaines/dup.md", "## 1. A".to_owned()),
        ("docs/conception/revues/dup.md", "## 1. B".to_owned()),
        ("docs/conception/05-architecture.md", "## 1. Vue".to_owned()),
    ] {
        docs.insert(path.to_owned(), doc::parse(&body));
    }
    docs
}

fn fetch(name: &str) -> Option<String> {
    (name == "carnet.md").then(carnet)
}

fn model<S: AsRef<str>>(files: &[(&str, S)]) -> Model {
    let sources: Vec<(String, String)> = files
        .iter()
        .map(|(f, t)| ((*f).to_owned(), t.as_ref().to_owned()))
        .collect();
    build(conception(), &sources, &fetch)
}

/// Un fichier Rust avec un seul commentaire, dont le texte est donné après `// `.
fn rust(comment: &str) -> String {
    format!("// {comment}")
}

/// Erreurs (messages seuls) du renvoi `spec:` donné, dans un fichier Rust factice.
fn errors_of(spec: &str) -> Vec<String> {
    let m = model(&[("crates/x/src/lib.rs", rust(&format!("spec: {spec}")))]);
    assert_eq!(m.cites.len(), 1, "{spec}");
    m.cites[0].errors.clone()
}

#[test]
fn parses_headings_and_ids() {
    let d = doc::parse(&combat());
    let keys: Vec<&str> = d.sections.keys().map(String::as_str).collect();
    assert_eq!(keys, ["1", "1.1", "1.2 bis", "2"]);
    assert_eq!(d.sections["1.1"].title, "Geste");
    assert_eq!(d.sections["1.2 bis"].title, "Rare");
    for id in ["D1", "D2", "D3", "E4", "D9"] {
        assert!(d.defines(id, None), "{id}");
    }
    // Un bloc de code ne définit ni titre ni identifiant.
    assert!(!d.defines("D99", None) && !d.sections.contains_key("9"));
    // Définition dans la section, ou dans une sous-section de la section citée.
    assert!(d.defines("D1", Some("1.1")) && d.defines("D1", Some("1")));
    assert!(!d.defines("D1", Some("2")) && !d.defines("D1", Some("1.2 bis")));
    // Un titre sans numéro reste dans la section du dessus.
    assert!(d.defines("D9", Some("2")));
    assert!(d.sections["2"].body.contains("Phase 0"));
    let s = doc::parse(&securite());
    assert!(s.defines("S8", Some("4.3")) && s.defines("S9", None));
}

#[test]
fn recognises_identifier_shapes() {
    for id in ["S8", "M28", "SEC0-35", "AS-12", "R-ARCH-04", "T0-06", "F3"] {
        assert_eq!(
            doc::id_prefix(&format!("{id} suite")),
            Some((id.to_owned(), id.len()))
        );
    }
    for bad in ["8", "ABC", "Sx8", "S8x", "S8-", "S-", "s8"] {
        assert!(
            doc::id_prefix(bad).is_none_or(|(_, n)| n != bad.len()),
            "{bad}"
        );
    }
    assert_eq!(doc::split_id("SEC0-03"), ("SEC0-", "03"));
    assert_eq!(doc::split_id("S8"), ("S", "8"));
}

#[test]
fn parses_the_grammar_of_existing_references() {
    let segs = refs::parse(
        "revues/developpement-agents.md §4.4 A19, §6.2 ; carnet architecture-securite.md \
         SEC0-03 (révisé), 06, 29 à 31 ; §9.5 F1 à F10",
    )
    .unwrap();
    assert_eq!(segs.len(), 3);
    assert!(!segs[0].carnet && segs[0].file == "revues/developpement-agents.md");
    assert_eq!(
        segs[0].items,
        [
            Item {
                section: Some("4.4".into()),
                ids: Some(IdSpec::One("A19".into()))
            },
            Item {
                section: Some("6.2".into()),
                ids: None
            },
        ]
    );
    let ids: Vec<Option<IdSpec>> = segs[1].items.iter().map(|i| i.ids.clone()).collect();
    assert_eq!(
        ids,
        [
            Some(IdSpec::One("SEC0-03".into())),
            Some(IdSpec::One("SEC0-06".into())),
            Some(IdSpec::Range("SEC0-29".into(), "SEC0-31".into())),
        ]
    );
    // Un segment sans fichier reprend le fichier (et le carnet) du précédent.
    assert!(segs[2].carnet && segs[2].file == "architecture-securite.md");
    assert_eq!(
        segs[2].items[0],
        Item {
            section: Some("9.5".into()),
            ids: Some(IdSpec::Range("F1".into(), "F10".into()))
        }
    );
    let suffixed = refs::parse("a.md §3.10 b, §5.3 bis, §7 ter").unwrap();
    let keys: Vec<_> = suffixed[0]
        .items
        .iter()
        .map(|i| i.section.clone().unwrap())
        .collect();
    assert_eq!(keys, ["3.10 b", "5.3 bis", "7 ter"]);
}

#[test]
fn rejects_malformed_references() {
    for bad in [
        "",
        "§3",
        "carnet",
        "a.md §x",
        "a.md S8 S9",
        "a.md 06",
        "a.md S8 à",
        "a.md S8 à T9",
        "a.md S9 à S8",
        "a.md (non fermée",
        "a.md ) trop",
        "a.md F1.",
    ] {
        assert!(refs::parse(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn resolves_names_sections_and_identifiers() {
    let ok = [
        "domaines/combat.md §1.1",
        "combat.md §1.2 bis ; securite.md S8",
        "combat.md §1.1 D1, D2, D3 ; 05-architecture.md §1",
        "securite.md §4.3 S8, §6 S9",
        "combat.md §2 F1 à F4",
        "combat.md",
    ];
    for spec in ok {
        assert_eq!(errors_of(spec), Vec::<String>::new(), "{spec}");
    }
    let m = model(&[("a.rs", rust("spec: combat.md §1.1 D1, D2"))]);
    assert_eq!(m.cites[0].targets.len(), 2);
    assert_eq!(m.cites[0].targets[0].doc, COMBAT);
    let m = model(&[("a.rs", rust("spec: combat.md §2 F1 à F4"))]);
    assert_eq!(m.cites[0].targets.len(), 4);
}

#[test]
fn check_fails_on_dangling_references() {
    let cases = [
        ("absent.md §1", "fichier introuvable"),
        ("combat.md §7", "section §7 introuvable"),
        ("combat.md §1.2", "section §1.2 introuvable"),
        ("combat.md D77", "identifiant D77 non défini"),
        ("combat.md §2 D1", "identifiant D1 non défini"),
        ("combat.md F1 à F9", "identifiant F9 non défini"),
        ("dup.md §1", "nom court ambigu"),
        ("securite.md §4.3 S9", "identifiant S9 non défini"),
        (
            "carnet absent.md SEC0-03",
            "carnet absent.md absent de origin/pilotage",
        ),
        ("../../x.md §1", "fichier introuvable"),
        ("combat.md §1.1 D1,, D2 D3", "élément illisible"),
        ("combat.md §1 (", "parenthèse non fermée"),
    ];
    for (spec, expected) in cases {
        let errors = errors_of(spec);
        assert!(
            errors.iter().any(|e| e.contains(expected)),
            "{spec} : {errors:?}"
        );
    }
    // Les chemins relatifs à docs/conception/ restent permis, même si le nom court est ambigu.
    assert!(errors_of("revues/dup.md §1").is_empty());
}

#[test]
fn resolves_carnets_from_pilotage_only() {
    assert!(errors_of("carnet carnet.md SEC0-03, 06, 29 à 31 ; §9.5 F1").is_empty());
    assert!(errors_of("carnet carnet.md §3 SEC0-03").is_empty());
    // Sans le mot « carnet », le nom n'est cherché que dans la conception.
    assert!(!errors_of("carnet.md SEC0-03").is_empty());
    // Un nom de carnet qui sort de la racine de la branche est refusé sans appel à Git.
    assert!(!errors_of("carnet ../secret.md §1").is_empty());
    let m = model(&[("a.rs", rust("spec: carnet carnet.md SEC0-06"))]);
    assert_eq!(m.cites[0].targets[0].doc, "origin/pilotage:carnet.md");
}

#[test]
fn scans_comment_styles() {
    let rs = text(&[
        "let a = \"// spec: pas.md §1\";",
        "    // spec: a.md §1",
        "//! spec: b.md §2",
        "/// spec: c.md §3",
        "let x = 1; // spec: pas.md §9",
        "// texte spec: pas.md §9",
        "let k = 5; // PROVISOIRE(conception #12)",
        "// PROVISOIRE sans numéro",
        "// la valeur est PROVISOIRE, à revoir",
    ]);
    let hits = refs::scan("crates/x/src/lib.rs", &rs);
    let got: Vec<(usize, String)> = hits
        .iter()
        .map(|f| match &f.hit {
            Hit::Spec(t) => (f.line, format!("spec {t}")),
            Hit::Mark(t) => (f.line, format!("mark {t}")),
        })
        .collect();
    assert_eq!(
        got,
        [
            (2, "spec a.md §1".to_owned()),
            (3, "spec b.md §2".to_owned()),
            (4, "spec c.md §3".to_owned()),
            (7, "mark PROVISOIRE(conception #12)".to_owned()),
            (8, "mark PROVISOIRE sans numéro".to_owned()),
        ]
    );
    // RON se commente par // et non par # (H7) ; Python, TOML, YAML, shell et PowerShell par #.
    assert_eq!(refs::scan("data/x.ron", "// spec: a.md §1").len(), 1);
    assert!(refs::scan("data/x.ron", "# spec: a.md §1").is_empty());
    for path in [
        "x.py",
        "x.toml",
        "x.yml",
        "x.sh",
        "tools/x.ps1",
        "tools/hooks/pre-commit",
    ] {
        assert_eq!(refs::scan(path, "# spec: a.md §1").len(), 1, "{path}");
        assert!(refs::scan(path, "// spec: a.md §1").is_empty(), "{path}");
    }
    assert!(refs::scan("#!/bin/sh", "#!/bin/sh").is_empty());
    // Les types de fichier inconnus ne sont pas analysés.
    assert!(refs::scan("x.json", "// spec: a.md §1").is_empty());
    assert!(scanned("crates/x/lib.rs") && !scanned("docs/archives/x.rs") && !scanned("README.md"));
}

#[test]
fn provisional_marks_need_an_issue() {
    assert_eq!(refs::mark("PROVISOIRE(conception #12)"), Ok(12));
    for bad in [
        "PROVISOIRE",
        "PROVISOIRE(conception #)",
        "PROVISOIRE(conception 12)",
        "PROVISOIRE(conception #12",
    ] {
        assert!(refs::mark(bad).is_err(), "{bad}");
    }
    let both = [
        rust("PROVISOIRE(conception #7)"),
        rust("PROVISOIRE(conception #)"),
    ];
    let m = model(&[("a.rs", both.join("\n"))]);
    let (errors, _) = evaluate(&m, "");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].starts_with("a.rs:2 : PROVISOIRE sans numéro"));
}

#[test]
fn known_bad_list_is_closed_and_cannot_go_stale() {
    let src = rust("spec: absent.md §1");
    let m = model(&[("tools/dev/x.rs", src.clone())]);
    let (errors, tolerated) = evaluate(&m, "");
    assert_eq!((errors.len(), tolerated), (1, 0));
    let entry = "tools/dev/x.rs | 1 | absent.md §1 | fichier supprimé | #12";
    let (errors, tolerated) = evaluate(&m, &format!("# commentaire\n\n{entry}\n"));
    assert_eq!((errors, tolerated), (Vec::new(), 1));
    // Renvoi déplacé : la nouvelle ligne est fausse et l'entrée est périmée.
    let moved = model(&[("tools/dev/x.rs", format!("\n{src}"))]);
    assert_eq!(evaluate(&moved, entry).0.len(), 2);
    // Renvoi corrigé : l'entrée périmée fait échouer.
    let fixed = model(&[("tools/dev/x.rs", rust("spec: combat.md §1"))]);
    let (errors, _) = evaluate(&fixed, entry);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("entrée périmée"));
    // Entrées illisibles, et fichiers de tools/xtask/ (à corriger sur place).
    for bad in [
        "x | 1 | t | r",
        "x | un | t | r | #1",
        "x | 1 | t | r | 12",
        "x | 1 | t |  | #1",
    ] {
        assert_eq!(parse_known(bad).1.len(), 1, "{bad}");
    }
    assert_eq!(
        parse_known("tools/xtask/src/a.rs | 1 | t | r | #1").1.len(),
        1
    );
}

#[test]
fn report_is_sorted_and_lists_gaps() {
    let a = rust("spec: combat.md §1.1 D1, D2 ; securite.md S8");
    let b = rust("spec: combat.md §1.1");
    let c = rust("PROVISOIRE(conception #3)");
    let files = [
        ("z.rs", b.as_str()),
        ("a.rs", a.as_str()),
        ("m.rs", c.as_str()),
    ];
    let m = model(&files);
    let v = report(&m, None);
    assert_eq!(v["schema"], 1);
    let refs_files: Vec<&str> = v["references"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["file"].as_str().unwrap())
        .collect();
    assert_eq!(refs_files, ["a.rs", "z.rs"]);
    let sections = v["sections"].as_array().unwrap();
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0]["section"], "1.1");
    assert_eq!(sections[0]["title"], "Geste");
    assert_eq!(sections[0]["code"].as_array().unwrap().len(), 2);
    assert_eq!(v["provisional"][0]["issue"], 3);
    // Sections sans code, en ordre naturel (2 avant 10), conception seulement.
    let bare: Vec<String> = v["sections_without_code"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            format!(
                "{} {}",
                s["doc"].as_str().unwrap().rsplit('/').next().unwrap(),
                s["section"].as_str().unwrap()
            )
        })
        .collect();
    assert!(bare.contains(&"combat.md 1".to_owned()) && bare.contains(&"securite.md 6".to_owned()));
    assert!(!bare.contains(&"combat.md 1.1".to_owned()));
    // Même entrée, même sortie, au caractère près.
    assert_eq!(report(&model(&files), None), v);
    assert_eq!(markdown(&v), markdown(&report(&model(&files), None)));
    // Filtre par fichier.
    let only = report(&m, Some(SECURITE));
    assert_eq!(only["references"].as_array().unwrap().len(), 1);
    assert!(
        only["sections_without_code"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["doc"] == SECURITE)
    );
    let md = markdown(&v);
    assert!(md.contains("## Sections sans code") && md.contains("conception #3"));
    // Aucun chemin absolu ni séparateur Windows dans la sortie (SEC0-26).
    assert!(!md.contains('\\') && !md.contains(":/") && !md.contains(":\\"));
    assert_eq!(natural("10"), (vec![10], String::new()));
    assert!(natural("2") < natural("10") && natural("5.3") < natural("5.3 bis"));
}

#[test]
fn diff_lists_code_citing_changed_sections() {
    let old_combat = combat().replace("### 1.1 Geste", "### 1.1 Geste\nUn texte de plus.");
    let mut old = conception();
    old.insert(COMBAT.to_owned(), doc::parse(&old_combat));
    old.remove(SECURITE);
    let a = rust("spec: combat.md §1");
    let b = rust("spec: combat.md D2");
    let c = rust("spec: securite.md §4.3");
    let d = rust("spec: combat.md §2");
    let m = model(&[("a.rs", a), ("b.rs", b), ("c.rs", c), ("d.rs", d)]);
    let out = render_diff(&m, &old, "abc");
    // §1.1 a changé : le code qui cite §1 (parent) et la section tout entière de §4.3 (fichier ajouté).
    assert!(
        out.contains("combat.md §1.1 Geste (modifiée)\n\n- `a.rs:1`"),
        "{out}"
    );
    assert!(
        out.contains("securite.md §4.3 Commandes (ajoutée)\n\n- `c.rs:1`"),
        "{out}"
    );
    assert!(!out.contains("d.rs"), "{out}");
    assert!(!out.contains("b.rs"), "D2 n'a pas changé : {out}");
}

#[test]
fn diff_refuses_options_as_revisions() {
    for bad in ["", "-h", "--output=x", "--end-of-options", "a\nb"] {
        assert!(check_rev(bad).is_err(), "{bad:?}");
    }
    for good in ["HEAD~3", "origin/main", "69b4849", "v1.0"] {
        assert!(check_rev(good).is_ok(), "{good}");
    }
}
