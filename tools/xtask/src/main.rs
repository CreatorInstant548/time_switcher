//! `cargo xtask` : outils du dépôt time_switcher. Mode d'emploi : docs/dev/xtask.md.
//! spec: developpement-agents.md §4.2 ; carnet architecture-securite.md §9.5

#![forbid(unsafe_code)]
// Outil en ligne de commande : il écrit sur la console.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod disk;
mod exemptions;
mod github;
mod hooks;
mod manifests;
mod merge;
mod metadata;
mod scan;
mod sensitive;
mod sha256;
mod slots;
mod spec_map;
mod steps;
mod sys;
mod workflows;

use std::path::PathBuf;
use std::process::ExitCode;

use steps::{Ctx, Outcome};

type Step = (&'static str, &'static str, fn(&Ctx) -> Outcome);

/// Étapes de `check`, dans l'ordre d'exécution. Les noms sont stables (`--only`, CI).
const STEPS: [Step; 13] = [
    ("fmt", "formatage (cargo fmt)", steps::fmt),
    (
        "clippy",
        "clippy, espace de travail puis profil public",
        steps::clippy,
    ),
    (
        "test",
        "tests des crates touchées et de leurs dépendants",
        steps::test,
    ),
    ("deny", "cargo deny", steps::deny),
    ("audit", "cargo audit (base RustSec)", steps::audit),
    (
        "gitleaks",
        "secrets : arbre et commits depuis origin/main",
        steps::gitleaks,
    ),
    (
        "unsafe",
        "forbid(unsafe_code) et [lints] hérités",
        steps::unsafe_code,
    ),
    (
        "profiles",
        "public exclut les fonctionnalités de développement",
        steps::profiles,
    ),
    (
        "features",
        "fonctionnalités des manifestes et des hôtes",
        steps::features,
    ),
    (
        "clippy-exemptions",
        "aucune exemption de clippy dans le cœur",
        steps::clippy_exemptions,
    ),
    (
        "local-paths",
        "aucun chemin, utilisateur ni machine locale",
        steps::local_paths,
    ),
    ("workflows", "workflows GitHub Actions", steps::workflows),
    ("spec-map", "spec-map check", steps::spec_map),
];

const USAGE: &str = "usage : cargo xtask check [--all] [--only <étape>[,<étape>…]] | cargo xtask check --list | cargo xtask slots | cargo xtask disk [--clean] | cargo xtask merge <PR> [--dry-run] | cargo xtask hooks install|verify [--dry-run]";

struct Options {
    all: bool,
    list: bool,
    only: Option<Vec<String>>,
}

fn parse(args: &[String]) -> Result<Options, String> {
    let mut opts = Options {
        all: false,
        list: false,
        only: None,
    };
    let mut it = args.iter().map(String::as_str);
    if it.next() != Some("check") {
        return Err(USAGE.to_owned());
    }
    while let Some(arg) = it.next() {
        match arg {
            "--all" => opts.all = true,
            "--list" => opts.list = true,
            "--only" => {
                let names: Vec<String> = it
                    .next()
                    .ok_or(USAGE)?
                    .split(',')
                    .map(str::to_owned)
                    .collect();
                if let Some(bad) = names
                    .iter()
                    .find(|n| !STEPS.iter().any(|s| s.0 == n.as_str()))
                {
                    return Err(format!(
                        "étape inconnue : « {bad} » (voir cargo xtask check --list)"
                    ));
                }
                opts.only.get_or_insert_with(Vec::new).extend(names);
            }
            _ => return Err(format!("option inconnue : « {arg} »\n{USAGE}")),
        }
    }
    Ok(opts)
}

fn main() -> ExitCode {
    let args = sys::args();
    match args.first().map(String::as_str) {
        Some("slots") => return slots::main(&args[1..]),
        Some("disk") => return disk::main(&args[1..]),
        Some("spec-map") => return spec_map::run(&args[1..]),
        Some("merge") => return merge::main(&args[1..]),
        Some("hooks") => return hooks::main(&args[1..]),
        _ => {}
    }
    let opts = match parse(&args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    if opts.list {
        STEPS
            .iter()
            .for_each(|(name, label, _)| println!("{name:<18} {label}"));
        return ExitCode::SUCCESS;
    }
    let root = match sys::stdout_of(
        &std::env::current_dir().unwrap_or_default(),
        "git",
        &["rev-parse", "--show-toplevel"],
    ) {
        Ok(r) => PathBuf::from(r.trim()),
        Err(e) => {
            eprintln!("ÉCHEC : racine du dépôt introuvable ({e})");
            return ExitCode::FAILURE;
        }
    };
    let cx = Ctx::new(root, opts.all);
    let selected: Vec<&Step> = STEPS
        .iter()
        .filter(|s| {
            opts.only
                .as_ref()
                .is_none_or(|o| o.iter().any(|n| n == s.0))
        })
        .collect();
    let scope = if opts.all {
        "tout l'espace de travail"
    } else {
        "crates touchées depuis origin/main"
    };
    println!(
        "cargo xtask check : {} étape(s), tests sur {scope}",
        selected.len()
    );

    let mut results: Vec<(&str, String)> = Vec::new();
    let mut failed = false;
    for (i, (name, label, run)) in selected.iter().enumerate() {
        println!("\n[{}/{}] {name} : {label}", i + 1, selected.len());
        let line = match run(&cx) {
            Outcome::Ok(note) if note.is_empty() => "OK".to_owned(),
            Outcome::Ok(note) => format!("OK ({note})"),
            Outcome::Skipped(why) => format!("IGNORÉE : {why}"),
            Outcome::Fail(why) => {
                failed = true;
                format!("ÉCHEC : {why}")
            }
        };
        println!("  {line}");
        results.push((name, line));
        if failed {
            break;
        }
    }
    println!("\nRécapitulatif :");
    for (name, _, _) in &selected {
        let line = results
            .iter()
            .find(|r| r.0 == *name)
            .map_or("non lancée", |r| r.1.as_str());
        println!("  {name:<18} {line}");
    }
    if failed {
        println!("Résultat : ÉCHEC");
        return ExitCode::FAILURE;
    }
    println!("Résultat : OK");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn parses_options() {
        let o = parse(&args("check --all --only fmt,workflows")).unwrap();
        assert!(o.all && !o.list);
        assert_eq!(o.only.unwrap(), ["fmt", "workflows"]);
        assert!(parse(&args("check --list")).unwrap().list);
        assert!(parse(&args("check --only nope")).is_err());
        assert!(parse(&args("check --only")).is_err());
        assert!(parse(&args("check --force")).is_err());
        assert!(parse(&args("slots")).is_err());
    }

    #[test]
    fn step_names_are_unique() {
        let mut names: Vec<&str> = STEPS.iter().map(|s| s.0).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), STEPS.len());
    }
}
