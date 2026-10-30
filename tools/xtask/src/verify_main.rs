//! `cargo xtask verify-main [--dry-run] [--origin <n>d|all]`: approvals of the sensitive commits
//! of `origin/main` (SEC0-17, AS-31 E3). Every commit after the base must be a single-parent
//! commit, so that its sensitivity can be computed. Since T3e-2, a non-sensitive commit needs
//! nothing else at merge time: the GitHub ruleset of `main` (26213563) enforces the PR, `ci-ok` and
//! linear history server side, and `repo-settings.ps1 -Check` watches that ruleset. A commit that
//! touches a sensitive path is still the squash of a merged PR (J3), with `ci-ok` green on its head
//! (G3), `review-ok` and `archsec-ok` on that head or carried over (AS-40), cross-checked with §9
//! of the architecture lead's log (J4), all read from GitHub on every run (V5, AS-39).
//! `--origin` (AS-42, nightly job) adds J3 alone, one read per commit, for the non-sensitive
//! commits of the window ([`Origin`]): a ruleset disabled for a while, then restored, cannot hide a
//! commit pushed without a PR. An anomaly opens an issue and suspends merges until it is listed in
//! [`ACKNOWLEDGED`] (J5). In CI mode, no `git fetch`: the refs come from the checkout (`refresh`).
//! spec: carnet architecture-securite.md SEC0-17 ; §9.6 G3 ; §9.8 J3, J4, J5
//! Voir aussi l'issue #6, point 2.

use std::process::ExitCode;

use serde_json::Value;

use crate::carry::{Carry, Pr};
use crate::github::{Repo, approved, is_sha};
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

/// Commits checked by `--origin` (AS-42).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// Every commit after the base (weekly run).
    All,
    /// From the tip down to, and including, the first commit whose committer date is before this
    /// Unix time. A backdated commit pushed directly is thus still checked: it is either above a
    /// recent squash, so in the window, or the first old one.
    Since(i64),
}

impl Origin {
    /// `all`, or `<n>d` with `n` in 1..=366, counted back from `now` (Unix seconds).
    pub fn parse(s: &str, now: i64) -> Option<Self> {
        if s == "all" {
            return Some(Self::All);
        }
        let days: i64 = s.strip_suffix('d')?.parse().ok()?;
        (1..=366)
            .contains(&days)
            .then_some(Self::Since(now - days * 86_400))
    }
}

/// Commits of the window, from `git log --format=%H %ct` (tip first): every line down to, and
/// including, the first one dated before `cutoff`.
pub fn window(log: &str, cutoff: i64) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for line in log.lines() {
        let (sha, date) = line
            .split_once(' ')
            .ok_or_else(|| format!("unreadable git log line: {line}"))?;
        let date: i64 = date
            .parse()
            .map_err(|_| format!("unreadable commit date: {line}"))?;
        out.push(sha.to_owned());
        if date < cutoff {
            break;
        }
    }
    Ok(out)
}

/// AS-42 (J3 without G3): `commits/<sha>/pulls` must list exactly one PR whose
/// `merge_commit_sha` is `sha`, merged (`merged_at` set) into `main`. Its number, or the anomaly.
pub fn judge_squash(pulls: &[Value], sha: &str) -> Result<u64, String> {
    let found: Vec<&Value> = pulls
        .iter()
        .filter(|p| p["merge_commit_sha"].as_str() == Some(sha))
        .collect();
    let [pr] = found[..] else {
        return Err(format!(
            "{} PR dont ce commit est le squash, attendu une (J3, AS-42)",
            found.len()
        ));
    };
    let n = pr["number"].as_u64().unwrap_or_default();
    if pr["merged_at"].as_str().is_some_and(|d| !d.is_empty())
        && pr["base"]["ref"].as_str() == Some("main")
        && n > 0
    {
        Ok(n)
    } else {
        Err(format!(
            "PR #{n} non fusionnée dans main par ce commit (J3, AS-42)"
        ))
    }
}

/// Anomalies d'un commit (vide si conforme), avec une note pour l'affichage.
///
/// T3e-2 (AS-31 E3): a non-sensitive commit makes no GitHub read; its PR and `ci-ok` are enforced
/// by the ruleset of `main`. With `origin` (AS-42), it makes exactly one: J3 by [`judge_squash`].
/// It must still have exactly one parent: sensitivity is computed against that parent, so a commit
/// with none or several could not be classified (and `required_linear_history` forbids it anyway).
///
/// V5 (AS-39): for a commit that touches a sensitive path, the PR (`pulls/<n>`: head, `merged`,
/// `merge_commit_sha`), G3, the approvals and then J4 use facts read from GitHub on this run.
fn check_commit(
    repo: &Repo,
    rules: &sensitive::Rules,
    section: &str,
    sha: &str,
    origin: bool,
) -> Result<(Vec<String>, String), String> {
    let parents = repo.git(&["rev-list", "--parents", "-n", "1", sha])?;
    let parent = match parents.split_whitespace().collect::<Vec<_>>()[..] {
        [_, p] => p.to_owned(),
        _ => {
            return Ok((
                vec!["commit de fusion ou sans parent : sensibilité incalculable".to_owned()],
                String::new(),
            ));
        }
    };
    let files = repo.git(&["diff", "--name-only", "-z", "--no-renames", &parent, sha])?;
    let files: Vec<String> = files
        .split('\0')
        .filter(|f| !f.is_empty())
        .map(str::to_owned)
        .collect();
    let hits = sensitive::hits(repo, rules, &parent, sha, &files)?;
    if hits.is_empty() && origin {
        let pulls = repo.api_all(&format!(
            "repos/{}/commits/{sha}/pulls?per_page=100",
            repo.slug
        ))?;
        return Ok(match judge_squash(&pulls, sha) {
            Ok(n) => (
                Vec::new(),
                format!("non sensible, squash de la PR #{n} (AS-42)"),
            ),
            Err(why) => (vec![why], String::new()),
        });
    }
    if hits.is_empty() {
        return Ok((
            Vec::new(),
            "non sensible, PR et ci-ok imposés par le ruleset".to_owned(),
        ));
    }
    let pr = match pull_of(repo, sha)? {
        Ok(pr) => pr,
        Err(why) => return Ok((vec![why], String::new())),
    };
    let n = pr["number"].as_u64().unwrap_or_default();
    let head = pr["head"]["sha"].as_str().unwrap_or_default().to_owned();
    let mut bad = Vec::new();
    let mut note = format!("#{n}, tête {}", &head[..7]);
    if PRE_CI.contains(&(n, sha)) {
        note.push_str(", avant la CI");
    } else if let Some(why) = repo.ci_ok(&head)? {
        bad.push(why);
    } else {
        note.push_str(", ci-ok vert");
    }
    note.push_str(&format!(
        ", sensible ({}), faits GitHub relus (V5)",
        hits.len()
    ));
    let bodies = repo.comments(n)?;
    // AS-40: the same function as merge (W6), on facts read from GitHub on this run (V5).
    let mut carry = Carry::new(
        repo,
        Pr {
            number: n,
            head: &head,
            main: &parent,
            diff: (&parent, sha, &files),
            local_objects: false,
        },
    );
    let mut j4 = head.clone();
    for tag in ["review-ok", "archsec-ok"] {
        if approved(&bodies, tag, &head) {
            continue;
        }
        // J4: an archsec-ok SHA recorded in §9 is tried first.
        let in_section = |s: &str| tag != "archsec-ok" || carnet_approves(section, s);
        match carry.resolve(tag, &bodies, &in_section)? {
            Ok(from) => {
                note.push_str(&format!(", {tag} carried over from {}", &from[..7]));
                if tag == "archsec-ok" {
                    j4 = from;
                }
            }
            Err(why) => bad.push(format!("sensitive path, PR #{n}: {why}")),
        }
    }
    // J4: §9 is read again on every run (V1), against a fresh head (V5), or the SHA the
    // archsec-ok was carried over from (W6).
    if !carnet_approves(section, &j4) {
        bad.push(format!(
            "chemin sensible : « archsec-ok: {j4} » absent du §9 de {CARNET} (J4)"
        ));
    }
    Ok((bad, note))
}

/// Anomalies non levées : (commit, motifs).
pub type Anomalies = Vec<(String, Vec<String>)>;

/// Anomalies non levées de l'historique : (commit, motifs). `Err` si un contrôle n'a pas pu se faire.
/// `origin`: the window of the AS-42 check, if any.
pub fn check(repo: &Repo, origin: Option<Origin>) -> Result<Anomalies, String> {
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
    let in_window = match origin {
        Some(Origin::Since(cutoff)) => Some(window(
            &repo.git(&["log", "--format=%H %ct", "origin/main"])?,
            cutoff,
        )?),
        _ => None,
    };
    let mut anomalies = Vec::new();
    for sha in history {
        if BASE.contains(&sha) {
            println!("  {} base", &sha[..7]);
            continue;
        }
        let j3 = match &in_window {
            Some(w) => w.iter().any(|s| s == sha),
            None => origin.is_some(),
        };
        let (bad, note) = check_commit(repo, &rules, &section, sha, j3)?;
        if bad.is_empty() {
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
    let (code, out, err) = run_settings_check(repo, None)?;
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

/// `pwsh -NoProfile -NonInteractive -File <main copy>\tools\dev\repo-settings.ps1 -Check`, plus
/// `-SourceCommit <sha>` when `source` is given; its output is shown.
fn run_settings_check(
    repo: &Repo,
    source: Option<&str>,
) -> Result<(Option<i32>, String, String), String> {
    let script = repo
        .main
        .join("tools")
        .join("dev")
        .join("repo-settings.ps1");
    let script = script.to_string_lossy();
    let mut args = vec!["-NoProfile", "-NonInteractive", "-File", &script, "-Check"];
    if let Some(sha) = source {
        args.extend(["-SourceCommit", sha]);
    }
    let (code, out, err) = repo.runner.run(&repo.main, "pwsh", &args)?;
    let out = String::from_utf8_lossy(&out).into_owned();
    print!("{out}");
    Ok((code, out, err))
}

/// AS-38, condition 2: drift recomputed with `tools/ci/repo-settings.json` of commit `sha` (the
/// head of a repair PR), which the script reads by `git show <sha>:…`, never from the disk.
/// `Ok(true)` if there is none. Opens no issue; never skipped, so an error in CI mode.
pub fn settings_at(repo: &Repo, sha: &str) -> Result<bool, String> {
    if repo.ci {
        return Err(
            "repo-settings.ps1 -Check -SourceCommit is never skipped: not in CI mode".into(),
        );
    }
    if !is_sha(sha) {
        return Err(format!("-SourceCommit: « {sha} » is not a full SHA"));
    }
    println!("  GitHub settings against tools/ci/repo-settings.json of {sha}");
    let (code, _, err) = run_settings_check(repo, Some(sha))?;
    match code {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(format!(
            "repo-settings.ps1 -Check -SourceCommit {sha} could not conclude (code {code:?}): {}",
            err.trim()
        )),
    }
}

/// History and GitHub settings, judged apart: `merge` lets a settings drift alone through for a
/// repair PR (AS-38).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Health {
    pub history: bool,
    /// `Some(false)` on a drift; `None` when `-Check` could not conclude (`--dry-run` only).
    pub settings: Option<bool>,
}

/// `verify-main`, avec une *issue* par anomalie ; vrai si l'historique et les réglages GitHub sont
/// conformes. L'historique passe d'abord : un `-Check` qui échoue ne le cache pas. En `--dry-run`,
/// un `-Check` qui n'a pas pu conclure compte comme un écart.
pub fn verify_or_report(repo: &Repo, origin: Option<Origin>) -> Result<bool, String> {
    health(repo, origin).map(|h| h.history && h.settings == Some(true))
}

/// [`verify_or_report`] without `--origin`, with the history and the settings verdicts apart.
pub fn verify_parts(repo: &Repo) -> Result<Health, String> {
    health(repo, None)
}

fn health(repo: &Repo, origin: Option<Origin>) -> Result<Health, String> {
    let anomalies = check(repo, origin)?;
    for (sha, bad) in &anomalies {
        let body = format!(
            "`cargo xtask verify-main` : le commit `{sha}` de `main` est en anomalie \
             (chemin sensible sans ses approbations, commit sans PR fusionnée, ou commit non \
             classable), et les fusions sont suspendues.\n\n- {}\n\nPour lever l'anomalie : l'inscrire dans `ACKNOWLEDGED` \
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
        Ok(ok) => Some(ok),
        Err(e) if repo.dry_run => {
            println!("  AVERTISSEMENT (--dry-run) : {e}");
            None
        }
        Err(e) => return Err(e),
    };
    Ok(Health {
        history: anomalies.is_empty(),
        settings,
    })
}

/// Remote refs read by `check`: the history, and the architecture lead's log for J4.
const REFS: [&str; 2] = ["origin/main", "origin/pilotage"];

/// Brings [`REFS`] up to date. In CI mode there is no `git fetch` (m2 of #52, architecture lead's
/// decision of 30/10): the nightly job keeps no Git credentials (`persist-credentials: false`) and
/// its `fetch-depth: 0` checkout already brings both refs; a missing one is a refusal.
fn refresh(repo: &Repo) -> Result<(), String> {
    if !repo.ci {
        return repo.git(&["fetch", "--quiet", "origin"]).map(drop);
    }
    println!("  mode CI : git fetch sauté, références du checkout (fetch-depth: 0)");
    for r in REFS {
        let spec = format!("refs/remotes/{r}^{{commit}}");
        if !repo.ok(
            &repo.main,
            "git",
            &["rev-parse", "--verify", "--quiet", &spec],
        )? {
            return Err(format!(
                "{r} absent en mode CI, où git fetch est sauté : checkout avec fetch-depth: 0 requis"
            ));
        }
    }
    Ok(())
}

/// Wall clock in Unix seconds, for the `--origin <n>d` limit only. Not the tip's date, which a
/// pusher could set.
#[allow(clippy::disallowed_methods)] // outil : limite de la fenêtre d'AS-42.
fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

/// `--dry-run` and `--origin <since>` (AS-42); `None` on any other argument.
fn parse_args(args: &[String], now: i64) -> Option<(bool, Option<Origin>)> {
    let (mut dry_run, mut origin) = (false, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--dry-run" if !dry_run => dry_run = true,
            "--origin" if origin.is_none() => origin = Some(Origin::parse(it.next()?, now)?),
            _ => return None,
        }
    }
    Some((dry_run, origin))
}

pub fn main(args: &[String]) -> ExitCode {
    let Some((dry_run, origin)) = parse_args(args, unix_now()) else {
        eprintln!("usage : cargo xtask verify-main [--dry-run] [--origin <n>d|all]");
        return ExitCode::from(2);
    };
    let result = Repo::open(dry_run, true).and_then(|repo| {
        refresh(&repo)?;
        match origin {
            Some(Origin::All) => println!("  --origin all: J3 on every commit (AS-42)"),
            Some(Origin::Since(t)) => {
                println!("  --origin: J3 on every commit since Unix time {t} (AS-42)");
            }
            None => {}
        }
        verify_or_report(&repo, origin)
    });
    exit_code(result)
}

/// Exit code of `verify-main`: only a conforming history and settings succeed. An anomaly fails
/// the run, `--dry-run` included, so the nightly job fails and `report` opens `nightly-failure`.
fn exit_code(result: Result<bool, String>) -> ExitCode {
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

    /// AS-38: the drift recomputed with the file of a given commit runs `-Check -SourceCommit
    /// <sha>` and opens no issue; a bad SHA or the CI mode runs nothing and is an error.
    #[test]
    fn settings_at_source_commit() {
        let repo = |code: Option<i32>, ci: bool, calls: &Rc<RefCell<Vec<String>>>| Repo {
            main: PathBuf::from("main"),
            here: PathBuf::from("main"),
            slug: "o/r".to_owned(),
            dry_run: false,
            ci,
            cache: std::env::temp_dir(),
            home: None,
            runner: Box::new(Settings(code, Rc::clone(calls))),
        };
        for (code, want) in [(Some(0), Ok(true)), (Some(1), Ok(false))] {
            let calls = Rc::new(RefCell::new(Vec::new()));
            assert_eq!(settings_at(&repo(code, false, &calls), SHA), want);
            let calls = calls.borrow().clone();
            assert_eq!(calls.len(), 1, "{calls:?}");
            assert!(
                calls[0].ends_with(&format!("repo-settings.ps1 -Check -SourceCommit {SHA}")),
                "{calls:?}"
            );
        }
        for (sha, ci, code) in [
            (&SHA[..39], false, Some(0)),
            (&SHA.to_uppercase()[..], false, Some(0)),
            (SHA, true, Some(0)),
            (SHA, false, Some(2)),
        ] {
            let calls = Rc::new(RefCell::new(Vec::new()));
            assert!(
                settings_at(&repo(code, ci, &calls), sha).is_err(),
                "{sha} {ci} {code:?}"
            );
            assert_eq!(calls.borrow().len(), usize::from(code == Some(2)));
        }
    }

    /// Answers `git fetch` and `git rev-parse` of the refs in `.0` with 0, any other
    /// `git rev-parse` with 1; every call is noted.
    struct Refs(Vec<&'static str>, Rc<RefCell<Vec<String>>>);

    impl Run for Refs {
        fn run(&self, _dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
            let line = format!("{prog} {}", args.join(" "));
            self.1.borrow_mut().push(line.clone());
            let code = if line == "git fetch --quiet origin" {
                0
            } else if line.starts_with("git rev-parse --verify --quiet refs/remotes/") {
                i32::from(
                    !self
                        .0
                        .iter()
                        .any(|r| line.ends_with(&format!("/{r}^{{commit}}"))),
                )
            } else {
                return Err(format!("unexpected command: {line}"));
            };
            Ok((Some(code), Vec::new(), String::new()))
        }
    }

    fn refresh_in(ci: bool, present: &[&'static str]) -> (Result<(), String>, Vec<String>) {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let repo = Repo {
            main: PathBuf::from("main"),
            here: PathBuf::from("main"),
            slug: "o/r".to_owned(),
            dry_run: true,
            ci,
            cache: std::env::temp_dir(),
            home: None,
            runner: Box::new(Refs(present.to_vec(), Rc::clone(&calls))),
        };
        let result = refresh(&repo);
        let calls = calls.borrow().clone();
        (result, calls)
    }

    /// m2 of #52: outside CI, `git fetch origin`; in CI mode, never a fetch, and a refusal when
    /// `origin/main` or `origin/pilotage` is missing from the checkout.
    #[test]
    fn ci_mode_skips_fetch_and_needs_both_refs() {
        let (r, calls) = refresh_in(false, &[]);
        assert_eq!(r, Ok(()));
        assert_eq!(calls, ["git fetch --quiet origin"]);
        let (r, calls) = refresh_in(true, &["main", "pilotage"]);
        assert_eq!(r, Ok(()), "{calls:?}");
        assert_eq!(calls.len(), 2, "{calls:?}");
        for (present, missing) in [(["main"], "origin/pilotage"), (["pilotage"], "origin/main")] {
            let (r, calls) = refresh_in(true, &present);
            let e = r.unwrap_err();
            assert!(e.starts_with(missing), "{e}");
            assert!(!calls.iter().any(|c| c.contains("fetch")), "{calls:?}");
        }
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
    /// head is [`HEAD`], changing `file`; §9 approves `HEAD` if `j4`. `over` replaces recorded
    /// answers. Returns the result and calls.
    fn history_check(
        file: &str,
        j4: bool,
        over: &[(String, String)],
    ) -> (Result<Anomalies, String>, Vec<String>) {
        origin_check(file, j4, None, over)
    }

    /// [`history_check`] with `--origin`.
    fn origin_check(
        file: &str,
        j4: bool,
        origin: Option<Origin>,
        over: &[(String, String)],
    ) -> (Result<Anomalies, String>, Vec<String>) {
        let (repo, calls) = fake_repo(file, j4, false, over);
        let result = check(&repo, origin);
        let calls = calls.borrow().clone();
        (result, calls)
    }

    /// The fake repository of [`history_check`], in CI mode if `ci`.
    fn fake_repo(
        file: &str,
        j4: bool,
        ci: bool,
        over: &[(String, String)],
    ) -> (Repo, Rc<RefCell<Vec<String>>>) {
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
        let mut answers = vec![
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
                json!([{"number": 7, "merge_commit_sha": COMMIT,
                        "merged_at": "2026-10-30T10:00:00Z", "base": {"ref": "main"}}])
                .to_string(),
            ),
            (
                "git log --format=%H %ct origin/main".to_owned(),
                format!("{COMMIT} 2000\n{PARENT} 1000\n"),
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
            ("gh issue list".to_owned(), "[]".to_owned()),
        ];
        // The longest prefix wins, the last one among equals: `over` replaces an answer.
        answers.extend_from_slice(over);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let repo = Repo {
            main: PathBuf::from("main"),
            here: PathBuf::from("main"),
            slug: "o/r".to_owned(),
            dry_run: true,
            ci,
            cache: std::env::temp_dir(),
            home: None,
            runner: Box::new(Fake(answers, Rc::clone(&calls))),
        };
        (repo, calls)
    }

    /// PR #7 not found for [`COMMIT`], and no `ci-ok` on its head.
    fn no_pr_no_ci() -> Vec<(String, String)> {
        let s = "gh api --paginate repos/o/r";
        vec![
            (format!("{s}/commits/{COMMIT}/pulls"), "[]".to_owned()),
            (
                format!("{s}/commits/{HEAD}/check-runs"),
                json!({"check_runs": []}).to_string(),
            ),
        ]
    }

    /// Only these first-line approvals on PR #7.
    fn comments(bodies: &[String]) -> Vec<(String, String)> {
        let list: Vec<_> = bodies.iter().map(|b| json!({"body": b})).collect();
        vec![(
            "gh api --paginate repos/o/r/issues/7/comments".to_owned(),
            json!(list).to_string(),
        )]
    }

    fn reasons(r: &Result<Anomalies, String>) -> String {
        format!("{r:?}")
    }

    /// T3e-2 (AS-31 E3): the ruleset of `main` enforces the PR and `ci-ok`, so a non-sensitive
    /// commit without either is no anomaly, and makes no GitHub read. A sensitive one still is (J3,
    /// then G3).
    #[test]
    fn non_sensitive_commit_needs_no_pr_nor_ci_ok() {
        let (r, calls) = history_check("docs/a.md", false, &no_pr_no_ci());
        assert_eq!(r, Ok(vec![]), "{calls:#?}");
        assert_eq!(gh_calls(&calls), 0, "{calls:#?}");
        // Local checks still run.
        assert!(calls.iter().any(|c| c.starts_with("git diff --name-only")));
        let (r, _) = history_check("tools/xtask/src/x.rs", true, &no_pr_no_ci());
        assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{r:?}");
        assert!(reasons(&r).contains("(J3)"), "{r:?}");
        let ci = &no_pr_no_ci()[1..];
        let (r, _) = history_check("tools/xtask/src/x.rs", true, ci);
        assert!(reasons(&r).contains("ci-ok absent"), "{r:?}");
    }

    /// `commits/<COMMIT>/pulls` answers `v`.
    fn pulls(v: &Value) -> Vec<(String, String)> {
        vec![(
            format!("gh api --paginate repos/o/r/commits/{COMMIT}/pulls"),
            v.to_string(),
        )]
    }

    /// AS-42: with `--origin`, a non-sensitive commit must be the squash of a PR merged into
    /// `main`, read with a single request. No PR, an unmerged PR, another merge commit, another
    /// base or two PRs is an anomaly. A sensitive commit keeps its full check (V5).
    #[test]
    fn origin_needs_a_merged_squash_in_one_read() {
        let all = Some(Origin::All);
        let (r, calls) = origin_check("docs/a.md", false, all, &[]);
        assert_eq!(r, Ok(vec![]), "{calls:#?}");
        assert_eq!(gh_calls(&calls), 1, "{calls:#?}");
        let merged = json!({"number": 7, "merge_commit_sha": COMMIT,
                            "merged_at": "2026-10-30T10:00:00Z", "base": {"ref": "main"}});
        let mut cases = vec![json!([]), json!([merged.clone(), merged.clone()])];
        for (k, v) in [
            ("merged_at", Value::Null),
            ("merge_commit_sha", json!(HEAD)),
            ("base", json!({"ref": "dev"})),
        ] {
            let mut p = merged.clone();
            p[k] = v;
            cases.push(json!([p]));
        }
        for c in &cases {
            let (r, calls) = origin_check("docs/a.md", false, all, &pulls(c));
            assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{c}: {r:?}");
            assert!(reasons(&r).contains("(J3, AS-42)"), "{c}: {r:?}");
            assert_eq!(gh_calls(&calls), 1, "{calls:#?}");
        }
        let (r, _) = origin_check("tools/xtask/src/x.rs", true, all, &[]);
        assert_eq!(r, Ok(vec![]));
    }

    /// AS-42: `--origin <n>d` checks from the tip down to, and including, the first commit dated
    /// before the limit (so a backdated tip is still checked); older ones are left to `all`.
    #[test]
    fn origin_window_is_respected() {
        let no_pr = pulls(&json!([]));
        for cutoff in [1500, 3000] {
            let (r, _) = origin_check("docs/a.md", false, Some(Origin::Since(cutoff)), &no_pr);
            assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{cutoff}: {r:?}");
        }
        // A commit already before the limit lies above COMMIT: COMMIT is not read.
        let mut over = no_pr;
        over.push((
            "git log --format=%H %ct origin/main".to_owned(),
            format!("{} 2500\n{COMMIT} 2000\n", "a".repeat(40)),
        ));
        let (r, calls) = origin_check("docs/a.md", false, Some(Origin::Since(3000)), &over);
        assert_eq!(r, Ok(vec![]));
        assert_eq!(gh_calls(&calls), 0, "{calls:#?}");
        assert_eq!(
            window("a 50\nb 40\nc 10\nd 5\n", 30).unwrap(),
            ["a", "b", "c"]
        );
        assert_eq!(window("a 10\nb 40\n", 30).unwrap(), ["a"]);
        assert!(window("a x\n", 30).is_err() && window("a\n", 30).is_err());
        assert_eq!(Origin::parse("7d", 1_000_000), Some(Origin::Since(395_200)));
        assert_eq!(Origin::parse("all", 0), Some(Origin::All));
        for bad in ["0d", "367d", "7", "d", "-1d", "7h", ""] {
            assert_eq!(Origin::parse(bad, 0), None, "{bad}");
        }
    }

    /// `--no-cache` is gone (AS-42); `--origin` takes one value.
    #[test]
    fn parses_arguments() {
        let a = |s: &[&str]| {
            let v: Vec<String> = s.iter().map(|x| (*x).to_owned()).collect();
            parse_args(&v, 1_000_000)
        };
        assert_eq!(a(&[]), Some((false, None)));
        assert_eq!(
            a(&["--dry-run", "--origin", "all"]),
            Some((true, Some(Origin::All)))
        );
        assert_eq!(
            a(&["--origin", "7d"]),
            Some((false, Some(Origin::Since(395_200))))
        );
        for bad in [
            &["--no-cache"][..],
            &["--origin"],
            &["--origin", "7"],
            &["--dry-run", "--dry-run"],
            &["--origin", "all", "--origin", "7d"],
        ] {
            assert_eq!(a(bad), None, "{bad:?}");
        }
    }

    /// The sensitivity of a commit is computed against its single parent: a merge commit, or one
    /// without a parent, stays an anomaly even when its files are not sensitive.
    #[test]
    fn unclassifiable_commit_is_an_anomaly() {
        let over = [(
            "git rev-list --parents".to_owned(),
            format!("{COMMIT} {PARENT} {HEAD}"),
        )];
        let (r, calls) = history_check("docs/a.md", false, &over);
        assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{r:?}");
        assert!(reasons(&r).contains("sensibilité incalculable"), "{r:?}");
        assert_eq!(gh_calls(&calls), 0, "{calls:#?}");
    }

    /// AS-31 E3: a sensitive commit without `review-ok`, without `archsec-ok`, or without its
    /// `archsec-ok` in §9 (J4) stays an anomaly.
    #[test]
    fn sensitive_commit_needs_both_approvals_and_j4() {
        let f = "tools/xtask/src/x.rs";
        let (r, _) = history_check(f, true, &[]);
        assert_eq!(r, Ok(vec![]));
        for (tag, other) in [("review-ok", "archsec-ok"), ("archsec-ok", "review-ok")] {
            let (r, _) = history_check(f, true, &comments(&[format!("{other}: {HEAD}")]));
            assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{r:?}");
            assert!(
                reasons(&r).contains(&format!("« {tag}: {HEAD} » absent")),
                "{r:?}"
            );
        }
        let (r, _) = history_check(f, false, &[]);
        assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{r:?}");
        assert!(
            reasons(&r).contains(&format!("archsec-ok: {HEAD} » absent du §9")),
            "{r:?}"
        );
    }

    /// V5 (AS-39): a sensitive commit reads the PR, G3 and the approvals on GitHub on every run;
    /// nothing is kept between runs.
    #[test]
    fn sensitive_commit_reads_github_every_run() {
        for _ in 0..2 {
            let (r, calls) = history_check("tools/xtask/src/x.rs", true, &[]);
            assert_eq!(r, Ok(vec![]));
            for read in [
                "gh api repos/o/r/pulls/7",
                "/check-runs",
                "/issues/7/comments",
            ] {
                assert!(calls.iter().any(|c| c.contains(read)), "{read}: {calls:#?}");
            }
        }
    }

    /// The nightly job runs `--dry-run --origin <since>` in CI mode: an anomaly still makes the run
    /// fail (non-zero exit), without opening an issue, so `report` opens `nightly-failure`.
    #[test]
    fn anomaly_fails_the_nightly_run() {
        let (repo, calls) = fake_repo("tools/xtask/src/x.rs", false, true, &[]);
        let r = verify_or_report(&repo, None);
        assert_eq!(r, Ok(false));
        let calls = calls.borrow().clone();
        assert!(
            !calls.iter().any(|c| c.starts_with("gh issue create")),
            "{calls:#?}"
        );
        assert!(!calls.iter().any(|c| c.starts_with("pwsh")), "{calls:#?}");
        assert_ne!(exit_code(r), ExitCode::SUCCESS);
        let (repo, _) = fake_repo("tools/xtask/src/x.rs", true, true, &[]);
        let r = verify_or_report(&repo, None);
        assert_eq!(r, Ok(true));
        assert_eq!(exit_code(r), ExitCode::SUCCESS);
        assert_ne!(exit_code(Err("x".to_owned())), ExitCode::SUCCESS);
    }

    fn gh_calls(calls: &[String]) -> usize {
        calls.iter().filter(|c| c.starts_with("gh ")).count()
    }

    /// `check_commit` on a lab history: `x` squashes the PR whose head `h` merged `main` after
    /// both approvals were given on `a`. §9 approves `a` if `j4_on_a`, else `h`; `ci-ok` is green
    /// on `h` if `ci`.
    fn carried_check(j4_on_a: bool, ci: bool) -> Vec<String> {
        let c = crate::carry::lab::case(&format!("vm-carry-{ci}"));
        let tree = format!("{}^{{tree}}", c.h);
        let x = c
            .lab
            .git(&["commit-tree", &tree, "-p", &c.m1, "-m", "squash"]);
        let s = "gh api --paginate repos/o/r";
        let h = c.h.as_str();
        let runs = if ci {
            json!({"check_runs": [{"name": "ci-ok", "status": "completed",
                "conclusion": "success", "head_sha": h, "check_suite": {"id": 5}}]})
        } else {
            json!({"check_runs": []})
        };
        let wfr = json!({"workflow_runs": [{"path": ".github/workflows/ci.yml", "workflow_id": 9,
            "head_sha": h, "check_suite_id": 5, "repository": {"full_name": "o/r"}}]});
        let mut answers = crate::carry::lab::commits(&[&c.a, h]);
        answers.extend([
            (
                format!("{s}/commits/{x}/pulls"),
                json!([{"number": 7, "merge_commit_sha": x}]).to_string(),
            ),
            (
                "gh api repos/o/r/pulls/7".to_owned(),
                json!({"number": 7, "merged": true, "merge_commit_sha": x,
                       "base": {"ref": "main"}, "head": {"sha": h}})
                .to_string(),
            ),
            (
                "gh api repos/o/r/actions/workflows/ci.yml".to_owned(),
                json!({"id": 9}).to_string(),
            ),
            (format!("{s}/commits/{h}/check-runs"), runs.to_string()),
            ("gh api repos/o/r/actions/runs".to_owned(), wfr.to_string()),
            (
                format!("{s}/issues/7/comments"),
                json!([{"body": format!("review-ok: {}", c.a)},
                       {"body": format!("archsec-ok: {}", c.a)}])
                .to_string(),
            ),
        ]);
        let (repo, _) = c.lab.repo(answers);
        let rules = sensitive::Rules::parse("tools/xtask/\n").unwrap();
        let j4 = if j4_on_a { &c.a } else { &c.h };
        let section = format!("## 9. Avis\narchsec-ok: {j4}\n");
        let (bad, note) = check_commit(&repo, &rules, &section, &x, false).unwrap();
        if bad.is_empty() {
            let a = &c.a[..7];
            assert!(
                note.contains(&format!("archsec-ok carried over from {a}")),
                "{note}"
            );
        }
        bad
    }

    /// AS-40 W6: verify-main carries both approvals over with the same function as merge, and
    /// does J4 on `A`, not on the head.
    #[test]
    fn carried_approval_j4_on_a() {
        assert_eq!(carried_check(true, true), Vec::<String>::new());
        let bad = carried_check(false, true);
        assert_eq!(bad.len(), 1, "{bad:?}");
        assert!(
            bad[0].contains("(J4)") && !bad[0].contains("absent de la PR"),
            "{bad:?}"
        );
    }

    /// AS-40 W7: `ci-ok` stays required on the head itself, approvals carried over or not.
    #[test]
    fn carried_approval_needs_ci_ok_on_head() {
        let bad = carried_check(true, false);
        assert_eq!(bad.len(), 1, "{bad:?}");
        assert!(bad[0].contains("ci-ok absent"), "{bad:?}");
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
