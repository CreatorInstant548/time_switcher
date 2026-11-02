//! `cargo xtask determinism [--full] [--bless]` : preuve de déterminisme du monde-jouet
//! (`crates/ts_toy`), critère de fin de la phase 0 (V0-09).
//! spec: revues/architecture.md §22, §11.2 ; carnet architecture-securite.md §9.10 L6, §9.15 Y5
//!
//! Chaque exécutable de test de `ts_toy` tourne dans **son propre processus** (L6), avec son
//! nombre de fils : 1, puis 4. Chacun fait deux exécutions et un aller-retour d'instantané par
//! `ts_persist`, puis écrit ses résultats sur des lignes `ts-toy-result <clé> <valeur>`. Cette
//! commande vérifie que les processus donnent les mêmes valeurs, puis les compare aux valeurs
//! dorées de `data/golden/ts_toy.json` : la version courte toujours, l'an complet avec `--full`.

use std::collections::{BTreeMap, BTreeSet};
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
/// Test executables of `ts_toy`, each a separate process (L6), with the thread count that
/// process must report (issue #68, m1). Counts that are not the expected ones, or equal ones,
/// would reduce the proof to the same thread count twice.
const PROCESSES: [(&str, u16); 2] = [("one_thread", 1), ("four_threads", 4)];
// The proof compares a single thread with several: the two processes must differ.
const _: () = assert!(PROCESSES[0].1 == 1 && PROCESSES[1].1 > 1);
/// Key under which each process reports its effective thread count.
const THREADS_KEY: &str = "process.threads";
/// Keys specific to one process, never compared.
const PER_PROCESS: [&str; 1] = [THREADS_KEY];
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
/// ce contrôle lui-même, ou un fichier racine qui vaut `--all`.
pub fn touches_toy(meta: &Metadata, changed: &[String]) -> bool {
    let root = changed.iter().any(|f| {
        crate::steps::ROOT_FILES.contains(&f.as_str())
            || f.starts_with(".cargo/")
            || f.starts_with("data/golden/")
            || f.starts_with("tools/xtask/src/determinism")
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

/// Checks that a process ran on the thread count it is meant to prove (issue #68, m1): its
/// reported `process.threads` must be `expected`, and be present.
pub fn check_threads(name: &str, results: &Results, expected: u16) -> Result<(), String> {
    match results.get(THREADS_KEY).map(String::as_str) {
        Some(got) if got == expected.to_string() => Ok(()),
        Some(got) => Err(format!(
            "{name} reported {THREADS_KEY} {got}, expected {expected}"
        )),
        None => Err(format!("{name} did not report {THREADS_KEY}")),
    }
}

/// Résultats communs à tous les processus : sans les clés propres à chacun.
fn shared(r: &Results) -> Results {
    r.iter()
        .filter(|(k, _)| !PER_PROCESS.contains(&k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

/// Écarts entre les processus (L6), sur les clés de l'un comme de l'autre.
pub fn disagreements(runs: &[(&str, Results)]) -> Vec<String> {
    let Some((first_name, first)) = runs.first() else {
        return vec!["aucun processus".to_owned()];
    };
    let reference = shared(first);
    let show = |v: Option<&String>| v.map_or("absent", String::as_str).to_owned();
    let mut errors = Vec::new();
    for (name, r) in &runs[1..] {
        let other = shared(r);
        let keys: BTreeSet<&String> = reference.keys().chain(other.keys()).collect();
        for k in keys {
            let (a, b) = (reference.get(k), other.get(k));
            if a != b {
                errors.push(format!(
                    "{k} : {first_name} {}, {name} {}",
                    show(a),
                    show(b)
                ));
            }
        }
    }
    errors
}

/// Vrai si la clé dorée `key` relève des versions lancées.
fn in_scope(key: &str, full: bool) -> bool {
    full || !key.starts_with(FULL_PREFIX)
}

/// Écarts entre les résultats, communs aux processus, et les valeurs dorées des versions
/// lancées : valeur différente, valeur dorée sans résultat, résultat sans valeur dorée.
pub fn against_golden(results: &Results, golden: &Value, full: bool) -> Vec<String> {
    let Some(expected) = golden.get("results").and_then(Value::as_object) else {
        return vec![format!("{GOLDEN} : objet results absent")];
    };
    let mut errors = Vec::new();
    for (k, v) in expected.iter().filter(|(k, _)| in_scope(k, full)) {
        match (results.get(k), v.as_str()) {
            (Some(got), Some(want)) if got == want => {}
            (Some(got), Some(want)) => {
                errors.push(format!("{k} : {got}, valeur dorée {want}"));
            }
            (None, _) => errors.push(format!("{k} : valeur dorée sans résultat")),
            (_, None) => errors.push(format!("{GOLDEN} : {k} n'est pas une chaîne")),
        }
    }
    for k in results.keys().filter(|k| !expected.contains_key(*k)) {
        errors.push(format!("{k} : résultat sans valeur dorée"));
    }
    errors
}

/// Remplace les valeurs dorées des versions lancées par `results` : celles qui ne sont plus
/// produites disparaissent, celles des autres versions restent.
fn bless(golden: &mut Value, results: &Results, full: bool) -> Result<(), String> {
    let Some(map) = golden.get_mut("results").and_then(Value::as_object_mut) else {
        return Err(format!("{GOLDEN} : objet results absent"));
    };
    map.retain(|k, _| !in_scope(k, full));
    for (k, v) in results {
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
    for (name, expected_threads) in PROCESSES {
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
        check_threads(name, &results, expected_threads)?;
        durations.push(format!("{name} {secs:.1} s"));
        runs.push((name, results));
    }
    let path = root.join(GOLDEN);
    let mut golden: Value = serde_json::from_str(&sys::read_text(&path)?)
        .map_err(|e| format!("{GOLDEN} illisible : {e}"))?;
    let split = disagreements(&runs);
    if !split.is_empty() {
        split.iter().for_each(|e| println!("  - {e}"));
        return Err(format!(
            "les processus à 1 et à 4 fils diffèrent ({} écart(s))",
            split.len()
        ));
    }
    let results = shared(&runs[0].1);
    if bless_it {
        bless(&mut golden, &results, full)?;
        let mut text = serde_json::to_string_pretty(&golden).map_err(|e| e.to_string())?;
        text.push('\n');
        sys::write(&path, text.as_bytes())?;
        println!("  valeurs dorées réécrites : {GOLDEN} (à signaler dans la PR)");
        return Ok(format!("bénies ; {}", durations.join(", ")));
    }
    let errors = against_golden(&results, &golden, full);
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
    fn compares_processes() {
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
        assert!(disagreements(&[("one_thread", one.clone()), ("four_threads", four)]).is_empty());
        // A value that differs, and a key that only the first process gave.
        let bad = results(&[("generator", "h")]);
        assert_eq!(
            disagreements(&[("one_thread", one), ("four_threads", bad)]),
            [
                "generator : one_thread g, four_threads h",
                "short.world : one_thread s, four_threads absent",
            ]
        );
    }

    #[test]
    fn checks_the_effective_thread_counts() {
        let reported = |n: &str| results(&[("generator", "g"), (THREADS_KEY, n)]);
        assert!(check_threads("one_thread", &reported("1"), 1).is_ok());
        assert!(check_threads("four_threads", &reported("4"), 4).is_ok());
        // A process that ran on another thread count than the one it is meant to prove.
        let wrong = check_threads("four_threads", &reported("1"), 4).unwrap_err();
        assert_eq!(wrong, "four_threads reported process.threads 1, expected 4");
        // A process that does not report its thread count proves nothing.
        let missing = check_threads("one_thread", &results(&[("generator", "g")]), 1);
        assert_eq!(
            missing.unwrap_err(),
            "one_thread did not report process.threads"
        );
    }

    #[test]
    fn compares_with_golden() {
        let golden: Value = serde_json::from_str(
            r#"{"results":{"generator":"g","short.world":"s","year.world":"y"}}"#,
        )
        .unwrap();
        let ok = results(&[("generator", "g"), ("short.world", "s")]);
        assert!(against_golden(&ok, &golden, false).is_empty());
        // Without --full, the year is not expected; with it, it is.
        assert_eq!(against_golden(&ok, &golden, true).len(), 1);
        // A value that differs, and a result with no golden value.
        let other = results(&[("generator", "h"), ("short.world", "s"), ("extra", "1")]);
        assert_eq!(against_golden(&other, &golden, false).len(), 2);
    }

    /// Metadata of a small workspace: `ts_toy` depends on `ts_kernel` (normal) and on
    /// `ts_persist` (dev); `ts_viz` is unrelated.
    fn workspace() -> Metadata {
        crate::metadata::parse(
            r#"{"workspace_root":"/w","packages":[
            {"name":"ts_kernel","manifest_path":"/w/crates/ts_kernel/Cargo.toml",
             "dependencies":[]},
            {"name":"ts_persist","manifest_path":"/w/crates/ts_persist/Cargo.toml",
             "dependencies":[{"name":"ts_kernel","kind":null}]},
            {"name":"ts_toy","manifest_path":"/w/crates/ts_toy/Cargo.toml",
             "dependencies":[{"name":"ts_kernel","kind":null},
                             {"name":"ts_persist","kind":"dev"}]},
            {"name":"ts_viz","manifest_path":"/w/crates/ts_viz/Cargo.toml",
             "dependencies":[]},
            {"name":"xtask","manifest_path":"/w/tools/xtask/Cargo.toml",
             "dependencies":[]}]}"#,
        )
        .unwrap()
    }

    #[test]
    fn touches_toy_decides_when_check_runs_the_proof() {
        let meta = workspace();
        let touches = |files: &[&str]| {
            let changed: Vec<String> = files.iter().map(|f| (*f).to_owned()).collect();
            touches_toy(&meta, &changed)
        };
        // The toy world itself, its normal and dev dependencies, and what they pull in.
        assert!(touches(&["crates/ts_toy/src/town.rs"]));
        assert!(touches(&["crates/ts_kernel/src/lib.rs"]));
        assert!(touches(&["crates/ts_persist/src/lib.rs"]));
        // The golden values, this check itself, and the root files that count as `--all`.
        assert!(touches(&["data/golden/ts_toy.json"]));
        assert!(touches(&["tools/xtask/src/determinism.rs"]));
        assert!(touches(&["Cargo.lock"]));
        assert!(touches(&[".cargo/config.toml"]));
        // One matching file among others is enough.
        assert!(touches(&["docs/a.md", "crates/ts_toy/tests/common/mod.rs"]));
        // Nothing to do with the toy world.
        assert!(!touches(&[]));
        assert!(!touches(&["docs/a.md"]));
        assert!(!touches(&["crates/ts_viz/src/lib.rs"]));
        assert!(!touches(&["tools/xtask/src/steps.rs"]));
        assert!(!touches(&["data/other.json", "README.md"]));
    }

    #[test]
    fn bless_replaces_the_versions_run() {
        let mut golden: Value = serde_json::from_str(
            r#"{"manifest":{"a":1},"results":{"year.world":"y","short.old":"o"}}"#,
        )
        .unwrap();
        bless(&mut golden, &results(&[("short.world", "s")]), false).unwrap();
        assert_eq!(golden["results"]["year.world"], "y");
        assert_eq!(golden["results"]["short.world"], "s");
        assert!(golden["results"].get("short.old").is_none());
        assert_eq!(golden["manifest"]["a"], 1);
        // With --full, the year values are replaced too.
        bless(&mut golden, &results(&[("short.world", "s")]), true).unwrap();
        assert!(golden["results"].get("year.world").is_none());
    }
}
