//! `cargo xtask determinism [--full] [--bless]` : preuve de déterminisme du monde-jouet
//! (`crates/ts_toy`), critère de fin de la phase 0 (V0-09).
//! spec: revues/architecture.md §22, §11.2 ; carnet architecture-securite.md §9.10 L6, §9.15 Y5
//!
//! Chaque exécutable de test de `ts_toy` tourne dans **son propre processus** (L6), avec son
//! nombre de fils : 1, puis 4. Chacun fait deux exécutions et un aller-retour d'instantané par
//! `ts_persist`, puis écrit ses résultats sur des lignes `ts-toy-result <clé> <valeur>`. Cette
//! commande vérifie que les processus donnent les mêmes valeurs, puis les compare aux valeurs
//! dorées de `data/golden/ts_toy.json` : la version courte toujours, l'an complet avec `--full`.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;

use serde_json::Value;

use crate::metadata::Metadata;
use crate::steps::{Ctx, Outcome};
use crate::sys::{self, cargo, run_captured, run_live};

/// Valeurs dorées, relatives à la racine du dépôt.
pub const GOLDEN: &str = "data/golden/ts_toy.json";
/// Préfixe des lignes de résultat.
const MARKER: &str = "ts-toy-result ";
/// Exécutables de test de `ts_toy`, chacun un processus distinct (L6).
const PROCESSES: [&str; 2] = ["one_thread", "four_threads"];
/// Clés propres à un processus, jamais comparées.
const PER_PROCESS: [&str; 1] = ["process.threads"];
/// Préfixe des clés de l'an complet, produites seulement avec `--full`.
const FULL_PREFIX: &str = "year.";

type Results = BTreeMap<String, String>;

pub fn main(args: &[String]) -> ExitCode {
    let (mut full, mut bless) = (false, false);
    for a in args {
        match a.as_str() {
            "--full" => full = true,
            "--bless" => bless = true,
            _ => {
                eprintln!("usage : cargo xtask determinism [--full] [--bless]");
                return ExitCode::from(2);
            }
        }
    }
    let root = match sys::stdout_of(
        &std::env::current_dir().unwrap_or_default(),
        "git",
        &["rev-parse", "--show-toplevel"],
    ) {
        Ok(r) => std::path::PathBuf::from(r.trim()),
        Err(e) => {
            eprintln!("ÉCHEC : racine du dépôt introuvable ({e})");
            return ExitCode::FAILURE;
        }
    };
    match run(&root, full, bless) {
        Ok(note) => {
            println!("Résultat : OK ({note})");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("Résultat : ÉCHEC : {e}");
            ExitCode::FAILURE
        }
    }
}

/// Étape `determinism` de `check` : version courte, si le monde-jouet ou ses dépendances
/// changent, si une valeur dorée change, ou avec `--all` (Y5).
pub fn step(cx: &Ctx) -> Outcome {
    let concerned = match cx.all {
        true => true,
        false => {
            let changed = match cx.changed() {
                Ok(c) => c,
                Err(e) => return Outcome::Fail(e),
            };
            let meta = match cx.meta() {
                Ok(m) => m,
                Err(e) => return Outcome::Fail(e),
            };
            touches_toy(meta, changed)
        }
    };
    if !concerned {
        return Outcome::Skipped("ni ts_toy, ni ses dépendances, ni data/golden".to_owned());
    }
    match run(&cx.root, false, false) {
        Ok(note) => Outcome::Ok(note),
        Err(e) => Outcome::Fail(e),
    }
}

/// Vrai si un fichier modifié touche `ts_toy` ou l'une de ses dépendances, les valeurs dorées,
/// ou un fichier racine qui vaut `--all`.
pub fn touches_toy(meta: &Metadata, changed: &[String]) -> bool {
    let root = changed.iter().any(|f| {
        crate::steps::ROOT_FILES.contains(&f.as_str())
            || f.starts_with(".cargo/")
            || f.starts_with("data/golden/")
    });
    let touched = changed
        .iter()
        .filter_map(|f| meta.package_of(f))
        .map(str::to_owned)
        .collect();
    root || meta.with_dependents(touched).contains("ts_toy")
}

/// Lignes de résultat d'une sortie de `cargo test` ; une clé en double est une erreur.
pub fn parse_results(output: &str) -> Result<Results, String> {
    let mut out = Results::new();
    for line in output.lines() {
        // Le banc de test peut écrire « test x ... » sur la même ligne, avant le marqueur.
        let Some((_, rest)) = line.split_once(MARKER) else {
            continue;
        };
        let mut words = rest.split_whitespace();
        let (Some(key), Some(value), None) = (words.next(), words.next(), words.next()) else {
            return Err(format!("ligne de résultat illisible : {line}"));
        };
        if out.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(format!("résultat {key} en double"));
        }
    }
    Ok(out)
}

/// Compare les processus entre eux, puis aux valeurs dorées. Rend les écarts.
pub fn compare(runs: &[(&str, Results)], golden: &Value, full: bool) -> Vec<String> {
    let mut errors = Vec::new();
    let shared = |r: &Results| -> Results {
        r.iter()
            .filter(|(k, _)| !PER_PROCESS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    };
    let Some((first_name, first)) = runs.first() else {
        return vec!["aucun processus".to_owned()];
    };
    let reference = shared(first);
    for (name, r) in &runs[1..] {
        if shared(r) != reference {
            errors.push(format!(
                "{name} et {first_name} donnent des résultats différents"
            ));
            for (k, v) in &shared(r) {
                let other = reference.get(k).map_or("absent", String::as_str);
                if other != v {
                    errors.push(format!("  {k} : {name} {v}, {first_name} {other}"));
                }
            }
        }
    }
    let Some(expected) = golden.get("results").and_then(Value::as_object) else {
        return [errors, vec![format!("{GOLDEN} : objet results absent")]].concat();
    };
    for (k, v) in expected {
        if k.starts_with(FULL_PREFIX) && !full {
            continue;
        }
        match (reference.get(k), v.as_str()) {
            (Some(got), Some(want)) if got == want => {}
            (Some(got), Some(want)) => {
                errors.push(format!("{k} : {got}, valeur dorée {want}"));
            }
            (None, _) => errors.push(format!("{k} : valeur dorée sans résultat")),
            (_, None) => errors.push(format!("{GOLDEN} : {k} n'est pas une chaîne")),
        }
    }
    for k in reference.keys().filter(|k| !expected.contains_key(*k)) {
        errors.push(format!("{k} : résultat sans valeur dorée"));
    }
    errors
}

/// Remplace les résultats dorés produits par ceux-ci ; garde les autres clés.
fn bless(golden: &mut Value, results: &Results) -> Result<(), String> {
    let Some(map) = golden.get_mut("results").and_then(Value::as_object_mut) else {
        return Err(format!("{GOLDEN} : objet results absent"));
    };
    for (k, v) in results
        .iter()
        .filter(|(k, _)| !PER_PROCESS.contains(&k.as_str()))
    {
        map.insert(k.clone(), Value::String(v.clone()));
    }
    Ok(())
}

/// Lance les processus, compare, et rend un résumé avec les durées mesurées.
pub fn run(root: &Path, full: bool, bless_it: bool) -> Result<String, String> {
    let build = ["test", "-p", "ts_toy", "--locked", "--no-run"];
    if !run_live(root, &cargo(), &build)? {
        return Err("compilation des tests de ts_toy impossible".to_owned());
    }
    let mut runs = Vec::new();
    let mut durations = Vec::new();
    for name in PROCESSES {
        let mut args = vec!["test", "-p", "ts_toy", "--locked", "--test", name, "--"];
        args.extend(["--nocapture", "--test-threads=1"]);
        if full {
            args.push("--include-ignored");
        }
        println!("  $ cargo {}", args.join(" "));
        let start = sys::now();
        let (ok, out) = run_captured(root, &cargo(), &args)?;
        let secs = sys::elapsed(start).as_secs_f64();
        if !ok {
            out.lines().for_each(|l| println!("    {l}"));
            return Err(format!("les tests {name} de ts_toy ont échoué"));
        }
        let results = parse_results(&out)?;
        results.iter().for_each(|(k, v)| println!("    {k} = {v}"));
        println!("    durée : {secs:.1} s");
        durations.push(format!("{name} {secs:.1} s"));
        runs.push((name, results));
    }
    let path = root.join(GOLDEN);
    let mut golden: Value = serde_json::from_str(&sys::read_text(&path)?)
        .map_err(|e| format!("{GOLDEN} illisible : {e}"))?;
    let errors = compare(&runs, &golden, full);
    let disagree = errors.iter().any(|e| e.contains("différents"));
    if bless_it && !disagree {
        bless(&mut golden, &runs[0].1)?;
        let mut text = serde_json::to_string_pretty(&golden).map_err(|e| e.to_string())?;
        text.push('\n');
        sys::write(&path, text.as_bytes())?;
        println!("  valeurs dorées réécrites : {GOLDEN} (à signaler dans la PR)");
        return Ok(format!("bénies ; {}", durations.join(", ")));
    }
    if !errors.is_empty() {
        errors.iter().for_each(|e| println!("  - {e}"));
        return Err(format!("{} écart(s) ; voir {GOLDEN}", errors.len()));
    }
    let version = if full {
        "courte et an complet"
    } else {
        "courte"
    };
    Ok(format!(
        "version {version}, 1 et 4 fils d'accord avec {GOLDEN} ; {}",
        durations.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn results(pairs: &[(&str, &str)]) -> Results {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    #[test]
    fn parses_result_lines() {
        let out = "running 2 tests\ntest generator ... ts-toy-result generator ab12\n\
                   ts-toy-result process.threads 4\nok\ntest x ... ok\n";
        assert_eq!(
            parse_results(out).unwrap(),
            results(&[("generator", "ab12"), ("process.threads", "4")])
        );
        assert!(parse_results("ts-toy-result a 1\nts-toy-result a 2\n").is_err());
        assert!(parse_results("ts-toy-result a\n").is_err());
        assert!(parse_results("ts-toy-result a 1 2\n").is_err());
    }

    #[test]
    fn compares_processes_and_golden() {
        let golden: Value = serde_json::from_str(
            r#"{"results":{"generator":"g","short.world":"s","year.world":"y"}}"#,
        )
        .unwrap();
        let one = results(&[
            ("generator", "g"),
            ("short.world", "s"),
            ("process.threads", "1"),
        ]);
        let four = results(&[
            ("generator", "g"),
            ("short.world", "s"),
            ("process.threads", "4"),
        ]);
        let ok = [("one_thread", one.clone()), ("four_threads", four)];
        assert!(compare(&ok, &golden, false).is_empty());
        // Without --full, the year is not expected; with it, it is.
        assert_eq!(compare(&ok, &golden, true).len(), 1);
        // 1 thread against 4 threads.
        let bad = results(&[("generator", "g"), ("short.world", "t")]);
        let errors = compare(
            &[("one_thread", one.clone()), ("four_threads", bad)],
            &golden,
            false,
        );
        assert!(errors[0].contains("différents"), "{errors:?}");
        // Against the golden value, and a result with no golden value.
        let other = results(&[("generator", "h"), ("short.world", "s"), ("extra", "1")]);
        let errors = compare(&[("a", other.clone()), ("b", other)], &golden, false);
        assert_eq!(errors.len(), 2, "{errors:?}");
    }

    #[test]
    fn bless_keeps_other_keys() {
        let mut golden: Value =
            serde_json::from_str(r#"{"manifest":{"a":1},"results":{"year.world":"y"}}"#).unwrap();
        bless(
            &mut golden,
            &results(&[("short.world", "s"), ("process.threads", "1")]),
        )
        .unwrap();
        assert_eq!(golden["results"]["year.world"], "y");
        assert_eq!(golden["results"]["short.world"], "s");
        assert!(golden["results"].get("process.threads").is_none());
        assert_eq!(golden["manifest"]["a"], 1);
    }
}
