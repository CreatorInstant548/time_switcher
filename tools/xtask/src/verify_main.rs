//! `cargo xtask verify-main [--dry-run] [--no-cache]` : origine de chaque commit de `origin/main`
//! (SEC0-17). Les faits lus sur GitHub sont gardés dans un cache local (`verify_cache`, V1 à V3).
//! Chaque commit postérieur à la base est le squash d'une PR fusionnée, reliée par l'API (J3), avec
//! `ci-ok` vert sur sa tête (G3) et, pour un chemin sensible, `review-ok` et `archsec-ok` sur cette
//! tête, recoupés avec le §9 du carnet d'archi-sécu (J4). Une anomalie ouvre une *issue* et suspend
//! les fusions tant qu'elle n'est pas inscrite dans [`ACKNOWLEDGED`] (J5).
//! spec: carnet architecture-securite.md SEC0-17 ; §9.6 G3 ; §9.8 J3, J4, J5
//! Voir aussi l'issue #6, point 2.

use std::process::ExitCode;

use serde_json::Value;

use crate::github::{Repo, approved, is_sha};
use crate::verify_cache::{Cache, Facts};
use crate::{sensitive, sys};

/// Commits d'avant l'outillage, sans PR : la base, liste fermée.
const BASE: [&str; 2] = [
    "8ca4088bd38829d6c5aaa9281c4fa32b33a95e35", // docs(conception): importe la conception terminée
    "2b5681461e46cc1cc008e786b8b04ccd22b4993f", // docs(claude): mode local tant que GitHub est indisponible
];

/// PR fusionnées avant l'arrivée de `ci.yml` sur `main` (PR #14, dd83f66) : `ci-ok` n'y est pas
/// exigé (G3, §9.3 condition 1). Liste fermée : (PR, commit de squash).
const PRE_CI: [(u64, &str); 9] = [
    (1, "f60e5bce934eabf1a30453c9f094181581760c9e"),
    (4, "de84d92610e8e89eab7ee975a2ce08e25f2053a4"),
    (3, "499120938e49c500f217170ac7fcad5ba3031166"),
    (5, "93fc98a5f277de3fb443ce1ec1544d1c0354882d"),
    (9, "2362c64c37834ab004e5367c27440d7dbc4a36ae"),
    (10, "5209fd2f2205cf6b3cb75fd11c60a99ff3c9e7f0"),
    (11, "69b4849934434ac87f371d763f9daff5b0341117"),
    (15, "cea47c82498dc37cd27e54046db5b52c1422961d"),
    (16, "14b6c580405a653409faddd30ba4dae70600d222"),
];

/// Anomalies levées (J5) : (commit, raison, *issue* `role:architecture`). Ajout par une PR de
/// chemin sensible, avec `archsec-ok`.
const ACKNOWLEDGED: &[(&str, &str, u64)] = &[];

const CARNET: &str = "architecture-securite.md";

/// Section `## 9.` du carnet, jusqu'au titre suivant de même niveau.
pub fn section_9(carnet: &str) -> Option<String> {
    let mut lines = carnet.lines().skip_while(|l| !l.starts_with("## 9."));
    let first = lines.next()?;
    let rest: Vec<&str> = lines.take_while(|l| !l.starts_with("## ")).collect();
    Some(format!("{first}\n{}", rest.join("\n")))
}

/// Le §9 contient littéralement `archsec-ok: <sha>`, non suivi d'un autre chiffre hexadécimal (J4).
pub fn carnet_approves(section: &str, sha: &str) -> bool {
    let needle = format!("archsec-ok: {sha}");
    section
        .match_indices(&needle)
        .any(|(i, _)| !section[i + needle.len()..].starts_with(|c: char| c.is_ascii_hexdigit()))
}

/// La PR dont `sha` est le squash, par l'API (J3) ; `Err(motif)` d'anomalie dans `Ok`.
fn pull_of(repo: &Repo, sha: &str) -> Result<Result<Value, String>, String> {
    let s = &repo.slug;
    let list = repo.api_all(&format!("repos/{s}/commits/{sha}/pulls?per_page=100"))?;
    let found: Vec<u64> = list
        .iter()
        .filter(|p| p["merge_commit_sha"].as_str() == Some(sha))
        .filter_map(|p| p["number"].as_u64())
        .collect();
    let [n] = found[..] else {
        return Ok(Err(format!(
            "{} PR dont ce commit est le squash, attendu une (J3)",
            found.len()
        )));
    };
    let pr = repo.api(&format!("repos/{s}/pulls/{n}"))?;
    let ok = pr["merged"].as_bool() == Some(true)
        && pr["merge_commit_sha"].as_str() == Some(sha)
        && pr["base"]["ref"].as_str() == Some("main")
        && pr["head"]["sha"].as_str().is_some_and(is_sha);
    Ok(if ok {
        Ok(pr)
    } else {
        Err(format!(
            "PR #{n} non fusionnée dans main par ce commit (J3)"
        ))
    })
}

/// Anomalies d'un commit (vide si conforme), avec une note pour l'affichage et les faits lus sur
/// GitHub. Les faits de `cached` remplacent leur lecture ; tout le reste est recalculé (V1).
fn check_commit(
    repo: &Repo,
    rules: &sensitive::Rules,
    section: &str,
    sha: &str,
    cached: Option<&Facts>,
) -> Result<(Vec<String>, String, Facts), String> {
    let parents = repo.git(&["rev-list", "--parents", "-n", "1", sha])?;
    let parent = match parents.split_whitespace().collect::<Vec<_>>()[..] {
        [_, p] => p.to_owned(),
        _ => {
            return Ok((
                vec!["commit de fusion ou sans parent : pas un squash".to_owned()],
                String::new(),
                Facts::default(),
            ));
        }
    };
    let cached = cached.filter(|f| f.merge_commit_sha == sha && is_sha(&f.head) && f.pr > 0);
    let mut facts = match cached {
        Some(f) => f.clone(),
        None => match pull_of(repo, sha)? {
            Ok(pr) => Facts {
                pr: pr["number"].as_u64().unwrap_or_default(),
                merge_commit_sha: sha.to_owned(),
                head: pr["head"]["sha"].as_str().unwrap_or_default().to_owned(),
                ..Facts::default()
            },
            Err(why) => return Ok((vec![why], String::new(), Facts::default())),
        },
    };
    let (n, head) = (facts.pr, facts.head.clone());
    let mut bad = Vec::new();
    let mut note = format!("#{n}, tête {}", &head[..7]);
    if cached.is_some() {
        note.push_str(", faits GitHub en cache");
    }
    if PRE_CI.contains(&(n, sha)) {
        note.push_str(", avant la CI");
    } else if facts.ci_ok {
        note.push_str(", ci-ok vert");
    } else if let Some(why) = repo.ci_ok(&head)? {
        bad.push(why);
    } else {
        facts.ci_ok = true;
        note.push_str(", ci-ok vert");
    }
    let files = repo.git(&["diff", "--name-only", "-z", "--no-renames", &parent, sha])?;
    let files: Vec<String> = files
        .split('\0')
        .filter(|f| !f.is_empty())
        .map(str::to_owned)
        .collect();
    let hits = sensitive::hits(repo, rules, &parent, sha, &files)?;
    if !hits.is_empty() {
        note.push_str(&format!(", sensible ({})", hits.len()));
        if !(facts.review_ok && facts.archsec_ok) {
            let bodies = repo.comments(n)?;
            facts.review_ok = approved(&bodies, "review-ok", &head);
            facts.archsec_ok = approved(&bodies, "archsec-ok", &head);
        }
        for (tag, present) in [
            ("review-ok", facts.review_ok),
            ("archsec-ok", facts.archsec_ok),
        ] {
            if !present {
                bad.push(format!(
                    "chemin sensible : commentaire « {tag}: {head} » absent de la PR #{n}"
                ));
            }
        }
        // J4 is never cached: §9 is read again on every run (V1).
        if !carnet_approves(section, &head) {
            bad.push(format!(
                "chemin sensible : « archsec-ok: {head} » absent du §9 de {CARNET} (J4)"
            ));
        }
    }
    Ok((bad, note, facts))
}

/// Anomalies non levées : (commit, motifs).
pub type Anomalies = Vec<(String, Vec<String>)>;

/// Anomalies non levées de l'historique : (commit, motifs). `Err` si un contrôle n'a pas pu se faire.
/// Les faits GitHub des commits conformes vont au cache, même si un commit suivant échoue : une
/// erreur passagère ne fait pas tout relire au prochain essai.
pub fn check(repo: &Repo, use_cache: bool) -> Result<Anomalies, String> {
    let carnet = repo
        .blob(&repo.main, "origin/pilotage", CARNET)
        .map_err(|e| format!("origin/pilotage illisible ({e}) : J4 impossible"))?
        .ok_or_else(|| format!("{CARNET} absent d'origin/pilotage"))?;
    let section =
        section_9(&String::from_utf8_lossy(&carnet)).ok_or("§9 absent du carnet d'archi-sécu")?;
    let rules = sensitive::rules(repo)?;
    let history = repo.git(&["rev-list", "--reverse", "origin/main"])?;
    let history: Vec<&str> = history.lines().collect();
    if let Some(b) = BASE.iter().find(|b| !history.contains(b)) {
        return Err(format!(
            "commit de base {b} absent d'origin/main : historique réécrit"
        ));
    }
    let mut cache = Cache::open(repo, use_cache)?;
    let result = walk(repo, &rules, &section, &history, &mut cache);
    cache.save();
    result
}

fn walk(
    repo: &Repo,
    rules: &sensitive::Rules,
    section: &str,
    history: &[&str],
    cache: &mut Cache,
) -> Result<Anomalies, String> {
    let mut anomalies = Vec::new();
    for &sha in history {
        if BASE.contains(&sha) {
            println!("  {} base", &sha[..7]);
            continue;
        }
        let (bad, note, facts) = check_commit(repo, rules, section, sha, cache.get(sha))?;
        if bad.is_empty() {
            cache.put(sha, facts);
            println!("  {} conforme ({note})", &sha[..7]);
        } else if let Some((_, why, issue)) = ACKNOWLEDGED.iter().find(|a| a.0 == sha) {
            println!("  {} anomalie levée : {why} (issue #{issue})", &sha[..7]);
        } else {
            println!("  {} ANOMALIE ({note}) : {}", &sha[..7], bad.join(" ; "));
            anomalies.push((sha.to_owned(), bad));
        }
    }
    Ok(anomalies)
}

/// Dérive des réglages GitHub (AS-30 R5, AS-31 E4) : `repo-settings.ps1 -Check` de la copie
/// principale, en lecture seule. `Ok(true)` si conformes, ou en CI, où le contrôle est sauté avec un
/// avertissement dans le résumé du travail (`GITHUB_TOKEN` n'a pas les droits d'administration) ;
/// `Ok(false)` sur un écart (code 1), avec une *issue* ; `Err` si `-Check` n'a pas pu conclure.
pub fn settings_or_report(repo: &Repo) -> Result<bool, String> {
    println!("  réglages GitHub : repo-settings.ps1 -Check");
    if repo.ci {
        let msg = "repo-settings.ps1 -Check sauté en CI : GITHUB_TOKEN n'a pas les droits \
                   d'administration ; il tourne à chaque cargo xtask merge (AS-30 R5)";
        println!("  AVERTISSEMENT : {msg}");
        // Not from the tests, which also run in CI.
        if let Some(path) = sys::env("GITHUB_STEP_SUMMARY").filter(|_| !cfg!(test)) {
            let path = std::path::PathBuf::from(path);
            let mut text = sys::read(&path).unwrap_or_default();
            text.extend_from_slice(format!("\n> [!WARNING]\n> {msg}\n").as_bytes());
            if let Err(e) = sys::write(&path, &text) {
                println!("  (résumé du travail non écrit : {e})");
            }
        }
        return Ok(true);
    }
    let script = repo
        .main
        .join("tools")
        .join("dev")
        .join("repo-settings.ps1");
    let script = script.to_string_lossy();
    let args = ["-NoProfile", "-NonInteractive", "-File", &script, "-Check"];
    let (code, out, err) = repo.runner.run(&repo.main, "pwsh", &args)?;
    let out = String::from_utf8_lossy(&out);
    print!("{out}");
    match code {
        Some(0) => Ok(true),
        Some(1) => {
            let body = format!(
                "`repo-settings.ps1 -Check` : les réglages GitHub du dépôt diffèrent de \
                 `tools/ci/repo-settings.json` d'`origin/main`, et les fusions sont suspendues.\n\n\
                 ```\n{}\n```\n\nPour lever l'écart : corriger le fichier par une PR de chemin \
                 sensible, ou relancer `-Apply` depuis la copie principale (orchestrateur).",
                out.chars().take(4000).collect::<String>().trim_end()
            );
            repo.open_issue(
                "repo-settings-drift",
                "verify-main : réglages GitHub différents de tools/ci/repo-settings.json",
                &body,
            )?;
            Ok(false)
        }
        _ => Err(format!(
            "repo-settings.ps1 -Check n'a pas pu conclure (code {code:?}) : {}",
            err.trim()
        )),
    }
}

/// `verify-main`, avec une *issue* par anomalie ; vrai si l'historique et les réglages GitHub sont
/// conformes. L'historique passe d'abord : un `-Check` qui échoue ne le cache pas. En `--dry-run`,
/// un `-Check` qui n'a pas pu conclure compte comme un écart.
pub fn verify_or_report(repo: &Repo, use_cache: bool) -> Result<bool, String> {
    let anomalies = check(repo, use_cache)?;
    for (sha, bad) in &anomalies {
        let body = format!(
            "`cargo xtask verify-main` : le commit `{sha}` de `main` n'a pas l'origine attendue, et les \
             fusions sont suspendues.\n\n- {}\n\nPour lever l'anomalie : l'inscrire dans `ACKNOWLEDGED` \
             (`tools/xtask/src/verify_main.rs`) par une PR de chemin sensible, avec `archsec-ok` (J5).",
            bad.join("\n- ")
        );
        repo.open_issue(
            sha,
            &format!("verify-main : anomalie sur le commit {}", &sha[..7]),
            &body,
        )?;
    }
    let settings = match settings_or_report(repo) {
        Ok(ok) => ok,
        Err(e) if repo.dry_run => {
            println!("  AVERTISSEMENT (--dry-run) : {e}");
            false
        }
        Err(e) => return Err(e),
    };
    Ok(anomalies.is_empty() && settings)
}

pub fn main(args: &[String]) -> ExitCode {
    let dry_run = args.iter().any(|a| a == "--dry-run");
    // --no-cache: full check of every commit (nightly job, V3); the cache is rewritten.
    let no_cache = args.iter().any(|a| a == "--no-cache");
    if args.iter().any(|a| a != "--dry-run" && a != "--no-cache") {
        eprintln!("usage : cargo xtask verify-main [--dry-run] [--no-cache]");
        return ExitCode::from(2);
    }
    let result = Repo::open(dry_run, true).and_then(|repo| {
        repo.git(&["fetch", "--quiet", "origin"])?;
        verify_or_report(&repo, !no_cache)
    });
    match result {
        Ok(true) => {
            println!("Résultat : OK");
            ExitCode::SUCCESS
        }
        Ok(false) => {
            println!(
                "Résultat : ÉCHEC (anomalies de l'historique ou des réglages GitHub : fusions suspendues)"
            );
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("ÉCHEC : {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    use serde_json::json;

    use super::*;
    use crate::github::{Output, Run};

    const SHA: &str = "e3e2f2a842612bebfba1a8388f86fd296f64344c";

    #[test]
    fn reads_section_9() {
        let carnet = format!(
            "# C\n## 8. Avant\narchsec-ok: {SHA}\n## 9. Avis\n| x | **`archsec-ok: {SHA}`** |\n### 9.8 Sous\n\
             archsec-ok: {}0\n## 10. Après\narchsec-ok: {}\n",
            &SHA[..39],
            "a".repeat(40)
        );
        let s = section_9(&carnet).unwrap();
        assert!(s.contains("9.8") && !s.contains("## 10.") && !s.contains("## 8."));
        assert!(carnet_approves(&s, SHA));
        assert!(!carnet_approves(&s, &"a".repeat(40)));
        // Un SHA plus long qui commence par celui-ci ne compte pas.
        assert!(!carnet_approves(&format!("archsec-ok: {SHA}f"), SHA));
        assert!(!carnet_approves(&format!("archsec-ok:{SHA}"), SHA));
        assert!(section_9("# rien\n## 10. x\n").is_none());
    }

    /// Answers `repo-settings.ps1 -Check` with a fixed exit code, and `gh issue list` with `[]`.
    struct Settings(Option<i32>, Rc<RefCell<Vec<String>>>);

    impl Run for Settings {
        fn run(&self, _dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
            let line = format!("{prog} {}", args.join(" "));
            self.1.borrow_mut().push(line.clone());
            match prog {
                "pwsh" => Ok((self.0, b"  ECART : x\n".to_vec(), "erreur".to_owned())),
                "gh" if line.starts_with("gh issue list") => {
                    Ok((Some(0), b"[]".to_vec(), String::new()))
                }
                _ => Err(format!("unexpected command: {line}")),
            }
        }
    }

    fn settings(code: Option<i32>, ci: bool, dry_run: bool) -> (Result<bool, String>, Vec<String>) {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let repo = Repo {
            main: PathBuf::from("main"),
            here: PathBuf::from("main"),
            slug: "o/r".to_owned(),
            dry_run,
            ci,
            cache: std::env::temp_dir(),
            home: None,
            runner: Box::new(Settings(code, Rc::clone(&calls))),
        };
        let result = settings_or_report(&repo);
        let calls = calls.borrow().clone();
        (result, calls)
    }

    /// AS-30 R5, AS-31 E4: drift (code 1) suspends merges and opens an issue; a `-Check` that
    /// could not conclude is an error; in CI the check is skipped without running anything.
    #[test]
    fn settings_check_codes() {
        let (r, calls) = settings(Some(0), false, false);
        assert_eq!(r, Ok(true));
        assert_eq!(calls.len(), 1);
        assert!(
            calls[0].starts_with("pwsh -NoProfile -NonInteractive -File ")
                && calls[0].ends_with("repo-settings.ps1 -Check"),
            "{calls:?}"
        );
        // Drift: false, and the issue is only shown with --dry-run (no gh issue create).
        let (r, calls) = settings(Some(1), false, true);
        assert_eq!(r, Ok(false));
        assert!(calls.iter().any(|c| c.starts_with("gh issue list")));
        assert!(!calls.iter().any(|c| c.starts_with("gh issue create")));
        for code in [Some(2), Some(64), None] {
            assert!(settings(code, false, false).0.is_err(), "{code:?}");
        }
        let (r, calls) = settings(Some(1), true, false);
        assert_eq!(r, Ok(true));
        assert!(calls.is_empty(), "{calls:?}");
    }

    const COMMIT: &str = "1111111111111111111111111111111111111111";
    const PARENT: &str = "2b5681461e46cc1cc008e786b8b04ccd22b4993f";
    const HEAD: &str = "c41e984ba8594dedd592744fcb69a70f3716cd87";

    /// Recorded answers by longest command prefix; every call is noted.
    struct Fake(Vec<(String, String)>, Rc<RefCell<Vec<String>>>);

    impl Run for Fake {
        fn run(&self, _dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
            let line = format!("{prog} {}", args.join(" "));
            self.1.borrow_mut().push(line.clone());
            let best = self
                .0
                .iter()
                .filter(|(k, _)| line.starts_with(k.as_str()))
                .max_by_key(|(k, _)| k.len());
            match best {
                Some((_, out)) => Ok((Some(0), out.clone().into_bytes(), String::new())),
                None => Err(format!("unexpected command: {line}")),
            }
        }
    }

    /// `check` on a history of the two base commits plus [`COMMIT`], the squash of PR #7 whose
    /// head is [`HEAD`], changing `file`; §9 approves `HEAD` if `j4`. Returns the result and calls.
    fn history_check(
        cache: &Path,
        file: &str,
        j4: bool,
        use_cache: bool,
    ) -> (Result<Anomalies, String>, Vec<String>) {
        let s = "gh api --paginate repos/o/r";
        let runs = json!({"check_runs": [{"name": "ci-ok", "status": "completed",
            "conclusion": "success", "head_sha": HEAD, "check_suite": {"id": 5}}]});
        let wfr = json!({"workflow_runs": [{"path": ".github/workflows/ci.yml", "workflow_id": 9,
            "head_sha": HEAD, "check_suite_id": 5, "repository": {"full_name": "o/r"}}]});
        let carnet = format!(
            "## 9. Avis\n{}\n",
            if j4 {
                format!("archsec-ok: {HEAD}")
            } else {
                String::new()
            }
        );
        let answers = [
            ("git cat-file -e".to_owned(), String::new()),
            ("git cat-file blob origin/pilotage:architecture-securite.md".to_owned(), carnet),
            (
                "git cat-file blob origin/main:tools/ci/sensitive-paths.txt".to_owned(),
                include_str!("../../ci/sensitive-paths.txt").to_owned(),
            ),
            (
                "git rev-list --reverse origin/main".to_owned(),
                format!("{}\n{PARENT}\n{COMMIT}\n", BASE[0]),
            ),
            ("git rev-list --parents".to_owned(), format!("{COMMIT} {PARENT}")),
            ("git diff --name-only".to_owned(), format!("{file}\0")),
            (
                format!("{s}/commits/{COMMIT}/pulls"),
                json!([{"number": 7, "merge_commit_sha": COMMIT}]).to_string(),
            ),
            (
                "gh api repos/o/r/pulls/7".to_owned(),
                json!({"number": 7, "merged": true, "merge_commit_sha": COMMIT,
                       "base": {"ref": "main"}, "head": {"sha": HEAD}})
                .to_string(),
            ),
            ("gh api repos/o/r/actions/workflows/ci.yml".to_owned(), json!({"id": 9}).to_string()),
            (format!("{s}/commits/{HEAD}/check-runs"), runs.to_string()),
            ("gh api repos/o/r/actions/runs".to_owned(), wfr.to_string()),
            (
                format!("{s}/issues/7/comments"),
                json!([{"body": format!("review-ok: {HEAD}")}, {"body": format!("archsec-ok: {HEAD}")}])
                    .to_string(),
            ),
        ];
        let calls = Rc::new(RefCell::new(Vec::new()));
        let repo = Repo {
            main: PathBuf::from("main"),
            here: PathBuf::from("main"),
            slug: "o/r".to_owned(),
            dry_run: true,
            ci: false,
            cache: cache.to_path_buf(),
            home: None,
            runner: Box::new(Fake(answers.into_iter().collect(), Rc::clone(&calls))),
        };
        let result = check(&repo, use_cache);
        let calls = calls.borrow().clone();
        (result, calls)
    }

    fn gh_calls(calls: &[String]) -> usize {
        calls.iter().filter(|c| c.starts_with("gh ")).count()
    }

    /// Issue #8, V1: a new commit is checked on GitHub and its facts cached; on the next run the
    /// cached commit makes no GitHub call. `--no-cache` reads everything again.
    #[test]
    fn cached_commit_is_skipped_new_commit_checked() {
        let cache = std::env::temp_dir().join(format!("xtask-vm-skip-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&cache); // leftover of this test only
        let (r, calls) = history_check(&cache, "docs/a.md", false, true);
        assert_eq!(r, Ok(vec![]), "{calls:#?}");
        assert!(gh_calls(&calls) >= 4, "{calls:#?}");
        let (r, calls) = history_check(&cache, "docs/a.md", false, true);
        assert_eq!(r, Ok(vec![]));
        assert_eq!(gh_calls(&calls), 0, "{calls:#?}");
        // Local checks still run on a cache hit.
        assert!(calls.iter().any(|c| c.starts_with("git diff --name-only")));
        let (r, calls) = history_check(&cache, "docs/a.md", false, false);
        assert_eq!(r, Ok(vec![]));
        assert!(gh_calls(&calls) >= 4, "{calls:#?}");
        // A sensitive path appears (same facts): approvals are read once, then cached.
        let (r, calls) = history_check(&cache, "tools/xtask/src/x.rs", true, true);
        assert_eq!(r, Ok(vec![]));
        assert_eq!(gh_calls(&calls), 1, "{calls:#?}");
        let (r, calls) = history_check(&cache, "tools/xtask/src/x.rs", true, true);
        assert_eq!(r, Ok(vec![]));
        assert_eq!(gh_calls(&calls), 0, "{calls:#?}");
        // V1: J4 is recomputed on every run, cache or not.
        let (r, _) = history_check(&cache, "tools/xtask/src/x.rs", false, true);
        let r = r.unwrap();
        assert_eq!(r.len(), 1);
        assert!(r[0].1.iter().any(|b| b.contains("(J4)")), "{r:?}");
        let _ = std::fs::remove_dir_all(&cache); // temporary files of this test only
    }

    /// A corrupted cache, or one keyed on another `sensitive-paths.txt`, leads to a full check.
    #[test]
    fn unusable_cache_means_full_check() {
        let cache = std::env::temp_dir().join(format!("xtask-vm-bad-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&cache); // leftover of this test only
        let (r, _) = history_check(&cache, "docs/a.md", false, true);
        assert_eq!(r, Ok(vec![]));
        let file = cache.join("verify-main").join("cache.json");
        let text = sys::read_text(&file).unwrap();
        for bad in [
            "{".to_owned(),
            text.replace(
                &crate::sha256::hex(include_bytes!("../../ci/sensitive-paths.txt")),
                &"0".repeat(64),
            ),
        ] {
            sys::write(&file, bad.as_bytes()).unwrap();
            let (r, calls) = history_check(&cache, "docs/a.md", false, true);
            assert_eq!(r, Ok(vec![]));
            assert!(gh_calls(&calls) >= 4, "{calls:#?}");
        }
        let _ = std::fs::remove_dir_all(&cache); // temporary files of this test only
    }

    #[test]
    fn closed_lists_are_full_shas() {
        assert!(BASE.iter().all(|s| is_sha(s)));
        assert!(PRE_CI.iter().all(|(n, s)| *n > 0 && is_sha(s)));
        assert!(
            ACKNOWLEDGED
                .iter()
                .all(|(s, why, issue)| is_sha(s) && !why.is_empty() && *issue > 0)
        );
    }
}
