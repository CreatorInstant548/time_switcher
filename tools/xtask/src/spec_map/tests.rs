//! Tests de spec-map sur un petit corpus de documents et de code factices, tous en mémoire.
//! spec: carnet architecture-securite.md §7 ; §9.7

use std::collections::BTreeMap;

use serde_json::json;

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
fn handles_ambiguous_titles() {
    let d = doc::parse(&text(&[
        "## 3. a priori",
        "## 4. y compris",
        "## 5. b Lettre",
        "## 6. Phase 0",
        "## 6. Phase 1",
        "- **Z1** défini deux fois",
    ]));
    let keys: Vec<&str> = d.sections.keys().map(String::as_str).collect();
    // Un mot en minuscules après la lettre est le début du titre, pas un suffixe.
    assert_eq!(keys, ["3", "4", "5 b", "6"]);
    assert_eq!(d.sections["3"].title, "a priori");
    // Un numéro de section porté par deux titres est ambigu.
    assert!(d.dups.contains("6") && !d.dups.contains("3"));
    let mut docs = BTreeMap::new();
    docs.insert("docs/conception/d.md".to_owned(), d);
    let m = build(docs, &[("a.rs".to_owned(), rust("spec: d.md §6"))], &fetch);
    assert!(m.cites[0].errors[0].contains("ambiguë"));
    let m2 = model(&[("a.rs", rust("spec: combat.md §1"))]);
    assert!(m2.cites[0].errors.is_empty());
}

#[test]
fn numbered_points_are_subsections() {
    let d = doc::parse(&text(&[
        "## 2. Choix",
        "1. **Premier** point",
        "2. Deuxième",
        "   1. imbriqué, en retrait",
        "3. **P7 — Troisième** (décidé)",
        "   suite du paragraphe, en retrait",
        "### Phase 0 — sans numéro",
        "4. Quatrième, sous un titre sans numéro",
        "## 3. Autre",
        "1. seul",
        "## 4. Conflit",
        "1. point un",
        "### 4.1 Vrai sous-titre",
        "2. point deux",
        "## 5. Deux listes",
        "1. a",
        "",
        "Du texte.",
        "1. b",
        "### 5.3 bis Suffixe",
        "1. ignoré sous une clé à suffixe",
    ]));
    let keys: Vec<&str> = d.sections.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "2", "2.1", "2.2", "2.3", "2.4", "3", "3.1", "4", "4.1", "4.1.2", "5", "5.1", "5.3 bis"
        ]
    );
    assert_eq!(d.sections["2.1"].title, "Premier point");
    // Pas de point « 2.1 » en double pour la ligne en retrait.
    assert!(!d.dups.contains("2.1") && d.sections["2.2"].body.contains("imbriqué"));
    // Le point est une section : son texte, et les identifiants qu'il définit.
    assert!(d.sections["2.3"].body.contains("suite du paragraphe"));
    assert!(d.defines("P7", Some("2.3")) && d.defines("P7", Some("2")));
    assert!(!d.defines("P7", Some("2.2")));
    // Le vrai sous-titre prime, sans doublon ; le point sous ce sous-titre est la section 4.1.2.
    assert_eq!(d.sections["4.1"].title, "Vrai sous-titre");
    assert!(!d.dups.contains("4.1"));
    // Deux listes sous la même section rendent le numéro ambigu.
    assert!(d.dups.contains("5.1"));
    let mut twice = BTreeMap::new();
    twice.insert(
        "docs/conception/05-architecture.md".to_owned(),
        doc::parse(&text(&[
            "## 5. Deux listes",
            "1. a",
            "",
            "Du texte.",
            "1. b",
        ])),
    );
    let amb = build(
        twice,
        &[("a.rs".to_owned(), rust("spec: 05-architecture.md §5.1"))],
        &fetch,
    );
    assert!(amb.cites[0].errors[0].contains("ambiguë"));
    let mut docs = BTreeMap::new();
    docs.insert("docs/conception/05-architecture.md".to_owned(), d);
    let m = build(
        docs,
        &[
            (
                "a.rs".to_owned(),
                rust("spec: 05-architecture.md §2.3 ; §2.9"),
            ),
            ("b.rs".to_owned(), rust("spec: 05-architecture.md §2.3 P7")),
        ],
        &fetch,
    );
    assert_eq!(m.cites[0].errors.len(), 1);
    assert!(m.cites[0].errors[0].contains("§2.9 introuvable"));
    assert!(m.cites[1].errors.is_empty());
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
        "let q = '\"'; let m = 3; // PROVISOIRE(conception #4)",
        "let p = \"C:\\\\\"; // PROVISOIRE(conception #5)",
        "// Spec: d.md §4",
        "// valeur PROVISOIRE(conception #)",
    ]);
    let hits = refs::scan("crates/x/src/lib.rs", &rs);
    let got: Vec<(usize, String)> = hits
        .iter()
        .map(|f| match &f.hit {
            Hit::Spec(t) => (f.line, format!("spec {t}")),
            Hit::Mark(t) => (f.line, format!("mark {t}")),
            Hit::Bad(t, _) => (f.line, format!("bad {t}")),
        })
        .collect();
    assert_eq!(
        got,
        [
            (2, "spec a.md §1".to_owned()),
            (3, "spec b.md §2".to_owned()),
            (4, "spec c.md §3".to_owned()),
            (5, "bad spec: pas.md §9".to_owned()),
            (7, "mark PROVISOIRE(conception #12)".to_owned()),
            (8, "mark PROVISOIRE sans numéro".to_owned()),
            (9, "mark PROVISOIRE, à revoir".to_owned()),
            (10, "mark PROVISOIRE(conception #4)".to_owned()),
            (11, "mark PROVISOIRE(conception #5)".to_owned()),
            (12, "bad Spec: d.md §4".to_owned()),
            (13, "mark PROVISOIRE(conception #)".to_owned()),
        ]
    );
    // PowerShell échappe par l'accent grave : une barre oblique inverse finale n'échappe rien.
    let ps = "$p = \"C:\\dir\\\"  # PROVISOIRE(conception #6)";
    assert_eq!(refs::scan("tools/x.ps1", ps).len(), 1);
    assert!(refs::scannable("a.rs") && !refs::scannable("a.png"));
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
        "PROVISOIRE(conception #0)",
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
    assert!(errors[0].starts_with("a.rs:2 : PROVISOIRE/PROVISIONAL sans numéro"));
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
fn report_flags_identifiers_defined_twice() {
    let mut docs = BTreeMap::new();
    docs.insert(
        SECURITE.to_owned(),
        doc::parse(&text(&[
            "## 4. X",
            "### 4.3 Y",
            "- **S8 — a**",
            "## 6. Décisions",
            "| S8 | b |",
            "| S9 | c |",
        ])),
    );
    let sources = [("a.rs".to_owned(), rust("spec: securite.md S8"))];
    let m = build(docs, &sources, &fetch);
    // Toujours valide : check reste vert.
    assert_eq!(evaluate(&m, "").0, Vec::<String>::new());
    let v = report(&m, None);
    let twice = v["identifiers_defined_twice"].as_array().unwrap();
    assert_eq!(twice.len(), 1);
    assert_eq!(twice[0]["id"], "S8");
    assert_eq!(twice[0]["sections"], json!(["4.3", "6"]));
    assert!(markdown(&v).contains("Identifiants définis plusieurs fois (1)"));
    assert!(markdown(&v).contains("securite.md S8 : §4.3, §6"));
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
    assert!(md.contains("## Sections sans code") && md.contains("remontée #3"));
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

/// #18: silent successes of the scan, each reproduced as it was reported.
#[test]
fn scan_has_no_silent_successes() {
    let marks = |path: &str, text: &str| -> Vec<(usize, String)> {
        refs::scan(path, text)
            .into_iter()
            .map(|f| match f.hit {
                Hit::Spec(t) => (f.line, format!("spec {t}")),
                Hit::Mark(t) => (f.line, format!("mark {t}")),
                Hit::Bad(t, _) => (f.line, format!("bad {t}")),
            })
            .collect()
    };
    // A UTF-8 byte order mark before a `# spec:` on line 1 of a PowerShell file.
    assert_eq!(
        marks("tools/x.ps1", "\u{feff}# spec: a.md §1\n"),
        [(1, "spec a.md §1".to_owned())]
    );
    // An apostrophe inside a word does not open a string and hide the mark.
    let yaml = "step: l'étape   # PROVISOIRE(conception #3)\nit's: x # spec: a.md §1\n";
    assert_eq!(
        marks("x.yml", yaml),
        [
            (1, "mark PROVISOIRE(conception #3)".to_owned()),
            (2, "bad spec: a.md §1".to_owned())
        ]
    );
    // A real quoted string still hides what it holds, with or without a Python prefix.
    assert!(marks("x.py", "s = 'a # PROVISOIRE' + r'# spec: a.md §1'\n").is_empty());
    // The mark is the word, with or without a number; the English form is read too.
    let rs = "// valeur PROVISOIRE\n// PROVISIONAL(design #44): bounds\nlet a = 1; /* PROVISOIRE */\n\
              /* PROVISIONAL(design #7) */\n// XPROVISIONAL and PROVISOIRES are other words\n";
    assert_eq!(
        marks("x.rs", rs),
        [
            (1, "mark PROVISOIRE".to_owned()),
            (2, "mark PROVISIONAL(design #44): bounds".to_owned()),
            (3, "mark PROVISOIRE".to_owned()),
            (4, "mark PROVISIONAL(design #7)".to_owned()),
        ]
    );
    // A multi-line block comment is read line by line; `spec:` in a block is refused.
    let block = "/*\n * PROVISOIRE(conception #8)\n * spec: a.md §1\n */\nlet z = 1;\n";
    assert_eq!(
        marks("x.rs", block),
        [
            (2, "mark PROVISOIRE(conception #8)".to_owned()),
            (3, "bad spec: a.md §1".to_owned())
        ]
    );
    // Several comments on one line, a PowerShell block, a TypeScript template and `rb''` strings.
    assert_eq!(
        marks(
            "x.rs",
            "let x = 1; /* a */ // PROVISIONAL(design #12)\n/* b */ /* PROVISOIRE\n */\n"
        ),
        [
            (1, "mark PROVISIONAL(design #12)".to_owned()),
            (2, "mark PROVISOIRE".to_owned())
        ]
    );
    assert_eq!(
        marks(
            "tools/x.ps1",
            "<# PROVISIONAL(design #4) #>\n<#\n spec: a.md §1\n#>\n"
        ),
        [
            (1, "mark PROVISIONAL(design #4)".to_owned()),
            (3, "bad spec: a.md §1".to_owned())
        ]
    );
    assert_eq!(
        marks("web/x.ts", "const g = `src/**/*.ts`;\n// spec: a.md §1\n"),
        [(2, "spec a.md §1".to_owned())]
    );
    assert!(
        marks(
            "x.py",
            "x = rb'#PROVISIONAL(design #1)' + br'# spec: a.md §1'\n"
        )
        .is_empty()
    );
    // SQL and shaders are analysed; `--` is not a comment in a Rust file.
    assert_eq!(
        marks("db/x.sql", "SELECT 1; -- PROVISIONAL(design #2)\n").len(),
        1
    );
    assert_eq!(marks("a/x.gdshader", "// spec: a.md §1\n").len(), 1);
    assert_eq!(marks("tools/x.psd1", "# spec: a.md §1\n").len(), 1);
    assert!(marks("x.rs", "let a = 1 -- PROVISOIRE(conception #1)\n").is_empty());
    // Both forms of mark carry an issue number.
    assert_eq!(refs::mark("PROVISIONAL(design #12)"), Ok(12));
    for bad in [
        "PROVISIONAL",
        "PROVISIONAL(design #)",
        "PROVISIONAL(conception #3)",
        "PROVISOIRE(design #3)",
    ] {
        assert!(refs::mark(bad).is_err(), "{bad}");
    }
}

/// #18: every tracked type is analysed or classed; an unknown one fails the check.
#[test]
fn file_types_are_all_classed() {
    for known in [
        "a.rs",
        "b.sql",
        "c.psd1",
        "Cargo.lock",
        "d.json",
        "e.PNG",
        "tools/hooks/pre-commit",
    ] {
        assert!(refs::known_type(known), "{known}");
    }
    assert!(!refs::known_type("db/f.xyz"));
    let files: Vec<String> = ["a.rs", "docs/x.png", "db/f.xyz", "db/g.xyz", "h.README"]
        .map(str::to_owned)
        .to_vec();
    let errors = unknown_types(&files);
    assert_eq!(errors.len(), 2, "{errors:?}");
    assert!(errors[0].starts_with("h.README : file type `readme`"));
    assert!(errors[1].starts_with("db/f.xyz : file type `xyz`"));
}

/// #18: an element made only of parentheses would be dropped silently.
#[test]
fn lone_parenthesised_elements_are_refused() {
    for bad in [
        "combat.md, (§3)",
        "combat.md §1 ; (securite.md §4)",
        "(combat.md §3)",
    ] {
        let errors = errors_of(bad);
        assert!(
            errors.iter().any(|e| e.contains("seul entre parenthèses")),
            "{bad}: {errors:?}"
        );
    }
    // A note in parentheses next to an element stays free text.
    assert!(errors_of("combat.md §1 (voir aussi §9, sans effet), D1").is_empty());
    assert!(errors_of("combat.md §1.1 D1 (note)").is_empty());
}

/// #18: a title or an identifier in an HTML comment is not part of the document.
#[test]
fn html_comments_define_nothing() {
    let d = doc::parse(&text(&[
        "## 1. Réel",
        "<!--",
        "## 2. Commenté",
        "- **X1** commenté",
        "-->",
        "<!-- ## 3. Sur une ligne -->",
        "<!-- ouvert ## 4. Caché",
        "- **X2** caché --> - **X3** suite",
        "## 5. Après",
        "- **X4** réel",
        "```",
        "<!--",
        "```",
        "## 6. Encore réel",
    ]));
    let keys: Vec<&str> = d.sections.keys().map(String::as_str).collect();
    assert_eq!(keys, ["1", "5", "6"]);
    assert!(d.ids.contains_key("X4") && !d.ids.contains_key("X1") && !d.ids.contains_key("X2"));
}

/// #18, decided on the issue: a table row defines an identifier in its first two cells when the
/// cell is only that identifier (a date-first table of decisions); other cells never do.
#[test]
fn identifiers_in_the_second_cell_of_a_table() {
    let d = doc::parse(&text(&[
        "## 7. Décisions",
        "| 30/10 | AS-33 | décision un | AS-99 |",
        "| 30/10 | **AS-34** | deux |",
        "| 31/10 | `AS-35` | trois |",
        "| 31/10 | AS-36 suite du texte | quatre |",
        "| 31/10 | voir AS-37 | cinq |",
        "| E5 | AS-40 | six |",
    ]));
    for id in ["AS-33", "AS-34", "AS-35", "E5"] {
        assert!(d.defines(id, Some("7")), "{id}");
    }
    for id in ["AS-36", "AS-37", "AS-99", "AS-40"] {
        assert!(!d.defines(id, None), "{id}");
    }
}

/// H3: the list can only shrink, whatever the review of the PR.
#[test]
fn known_bad_entries_must_be_on_main() {
    let main = "# note\na.rs | 3 | x.md §1 | raison | #12\nb.rs | 4 | y.md §2 | autre | #13\n";
    assert!(grown_entries(main, main).is_empty());
    // Removing an entry, reordering, changing blanks or comments: fine.
    let shrunk = "  b.rs | 4 | y.md §2 | autre | #13  \n# autre commentaire\n";
    assert!(grown_entries(shrunk, main).is_empty());
    // A new entry, or a changed one (a moved line), is refused with its line number.
    let grown = format!("{main}c.rs | 9 | z.md §3 | nouveau | #14\n");
    assert_eq!(grown_entries(&grown, main).len(), 1);
    assert!(grown_entries(&grown, main)[0].starts_with("known_bad.txt:4 :"));
    let moved = main.replace("| 3 |", "| 5 |");
    assert_eq!(grown_entries(&moved, main).len(), 1);
    assert_eq!(grown_entries(main, "").len(), 2);
}

/// H6 and H5: `load` reads the tree through git, and fails rather than succeed on nothing.
#[test]
fn load_fails_closed_and_reads_every_encoding() {
    use crate::carry::lab::Lab;
    let check = |lab: &Lab| Ctx::new(lab.dir.clone(), false);

    // No conception document.
    let lab = Lab::new("sm-load-empty");
    lab.commit(&[("crates/x/src/lib.rs", Some("// spec: a.md §1\n"))]);
    let err = load(&check(&lab)).unwrap_err();
    assert!(err.contains("aucun document de conception"), "{err}");

    // Conception present, `origin/pilotage` absent.
    let lab = Lab::new("sm-load-nopilotage");
    let head = lab.commit(&[(
        "docs/conception/domaines/combat.md",
        Some("## 1. A\n- **D1** un\n"),
    )]);
    let err = load(&check(&lab)).unwrap_err();
    assert!(err.contains("origin/pilotage absent"), "{err}");

    // Both present: every encoding of a scripts is read, a carnet comes from the branch.
    lab.git(&["update-ref", "refs/remotes/origin/pilotage", &head]);
    let ps_bom = "\u{feff}# spec: combat.md D1\n";
    lab.commit(&[
        (
            "crates/x/src/lib.rs",
            Some("// spec: combat.md §1 D1\n// PROVISIONAL(design #5)\n"),
        ),
        ("tools/bom.ps1", Some(ps_bom)),
    ]);
    // Windows-1252 (a byte that is not UTF-8) and UTF-16 with a byte order mark: untracked is read too.
    let cp1252 = b"# \xe9t\xe9 spec: combat.md D1\n# spec: combat.md \xa7 D1\n$a = 1  # PROVISOIRE(conception #6)\n";
    crate::sys::write(&lab.dir.join("tools/cp1252.ps1"), cp1252).unwrap();
    let utf16: Vec<u8> = [0xFF, 0xFE]
        .into_iter()
        .chain(
            "# spec: combat.md D1\r\n"
                .encode_utf16()
                .flat_map(u16::to_le_bytes),
        )
        .collect();
    crate::sys::write(&lab.dir.join("tools/utf16.ps1"), &utf16).unwrap();
    // A binary file without extension is skipped; one with an analysed extension is an error.
    crate::sys::write(&lab.dir.join("data"), &[0, 159, 146, 150]).unwrap();
    let m = load(&check(&lab)).unwrap();
    let cites: Vec<(&str, usize)> = m.cites.iter().map(|c| (c.file.as_str(), c.line)).collect();
    assert!(cites.contains(&("tools/bom.ps1", 1)), "{cites:?}");
    assert!(cites.contains(&("tools/utf16.ps1", 1)), "{cites:?}");
    assert!(cites.contains(&("crates/x/src/lib.rs", 1)), "{cites:?}");
    assert!(m.cites.iter().all(|c| c.file != "data"));
    let marks: Vec<(&str, u64)> = m
        .marks
        .iter()
        .filter_map(|k| Some((k.file.as_str(), *k.issue.as_ref().ok()?)))
        .collect();
    assert!(
        marks.contains(&("tools/cp1252.ps1", 6)) && marks.contains(&("crates/x/src/lib.rs", 5)),
        "{marks:?}"
    );
    crate::sys::write(&lab.dir.join("tools/bin.ps1"), &[0, 1, 2, 255]).unwrap();
    let err = load(&check(&lab)).unwrap_err();
    assert!(
        err.contains("tools/bin.ps1") && err.contains("binary content"),
        "{err}"
    );
}
