//! `cargo xtask verify-main [--dry-run] [--origin <n>d|all]`: approvals of the sensitive commits
//! of `origin/main` (SEC0-17, AS-31 E3). Every commit after the base must be a single-parent
//! commit, so that its sensitivity can be computed. Since T3e-2, a non-sensitive commit needs
//! nothing else at merge time: the GitHub ruleset of `main` (26213563) enforces the PR, `ci-ok` and
//! linear history server side, and `repo-settings.ps1 -Check` watches that ruleset. A commit that
//! touches a sensitive path is still the squash of a merged PR (J3), with `ci-ok` green on its head
//! (G3), `review-ok` and `archsec-ok` on that head or carried over (AS-40), cross-checked with §9
//! of the architecture lead's log (J4), all read from GitHub on every run (V5, AS-39).
//! `--origin` (AS-42, AS-44, nightly job) adds J3 (one read), plus G3 on `<n>d`, for the non-sensitive
//! commits of the window ([`Origin`]): a ruleset disabled for a while, then restored, cannot hide a
//! commit pushed without a PR. An anomaly opens an issue and suspends merges until it is listed in
//! [`ACKNOWLEDGED`] (J5). In CI mode, no `git fetch origin`: the refs come from the checkout
//! (`refresh`). The one exception is an approval carried over by AS-55, whose check replays the
//! PR's merges: `carry` then fetches `pull/<n>/head` once, anonymously (public repository), and
//! never for a PR from a fork (X4).
//! spec: carnet architecture-securite.md SEC0-17 ; §9.6 G3 ; §9.8 J3, J4, J5
//! Voir aussi l'issue #6, point 2.

use std::collections::BTreeMap;
use std::process::ExitCode;

use serde_json::Value;

use crate::carry::{Carry, Pr};
use crate::github::{ANOMALY_TITLE, DRIFT_TITLE, Repo, approved, is_sha};
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

/// Commits checked by `--origin` (AS-42).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// Every commit after the base (weekly run).
    All,
    /// From the tip down to the first commit committed before this Unix time that passes its
    /// check (J3 and G3), included (AS-44 AA3). Committer dates are set by the pusher: backdated
    /// commits pushed without a PR fail J3, so the walk goes on through them until a real squash.
    Since(i64),
}

/// What `--origin` adds to a non-sensitive commit: nothing, J3 (`all`), or J3 and G3 (`<n>d`,
/// AS-44 AA2; `all` stays J3 alone for the API quota, #55).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Extra {
    Nothing,
    J3,
    J3G3,
}

impl Origin {
    /// `all`, or `<n>d` with `n` in 1..=366 (digits only, no sign nor leading zero), counted back
    /// from `now` (Unix seconds).
    pub fn parse(s: &str, now: i64) -> Option<Self> {
        if s == "all" {
            return Some(Self::All);
        }
        let n = s.strip_suffix('d')?;
        if n.starts_with('0') || !n.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let days: i64 = n.parse().ok()?;
        (1..=366)
            .contains(&days)
            .then_some(Self::Since(now - days * 86_400))
    }
}

/// Committer dates of `git log --format=%H %ct`, by commit.
fn dates(log: &str) -> Result<BTreeMap<&str, i64>, String> {
    log.lines()
        .map(|line| {
            let (sha, date) = line
                .split_once(' ')
                .ok_or_else(|| format!("unreadable git log line: {line}"))?;
            let date = date
                .parse()
                .map_err(|_| format!("unreadable commit date: {line}"))?;
            Ok((sha, date))
        })
        .collect()
}

/// J3: `commits/<sha>/pulls` must list exactly one PR whose `merge_commit_sha` is `sha`, merged
/// (`merged_at` set) into `main`, and whose head is another commit. Its number and head, or the
/// anomaly. For a non-sensitive commit with `--origin` (AS-42), and for every sensitive commit
/// (V5: read on this run; `merged_at` stands for the `merged` of `pulls/<n>`, no longer read, #55).
///
/// Head != `sha` (AS-44 AA1): a PR head pushed straight to `main`, ruleset off, is marked merged
/// by GitHub with `merge_commit_sha` set to that very commit; a squash is always a new commit.
///
/// A rebase merge of a one-commit PR also gives a new commit (new committer) whose
/// `merge_commit_sha` is that commit: J3 cannot tell it from a squash. Harmless (review of #73):
/// `allow_rebase_merge` is `false` and pinned by S2 in `tools/ci/repo-settings.json` (checked by
/// `-Check`), the ruleset of `main` only allows `squash` (`allowed_merge_methods`), and on the
/// `<n>d` window G3 still requires `ci-ok` on the PR head.
pub fn judge_squash(pulls: &[Value], sha: &str) -> Result<(u64, String), String> {
    let found: Vec<&Value> = pulls
        .iter()
        .filter(|p| p["merge_commit_sha"].as_str() == Some(sha))
        .collect();
    let [pr] = found[..] else {
        return Err(format!(
            "{} PR dont ce commit est le squash, attendu une (J3)",
            found.len()
        ));
    };
    let n = pr["number"].as_u64().unwrap_or_default();
    let head = pr["head"]["sha"].as_str().unwrap_or_default();
    if head == sha {
        return Err(format!(
            "commit égal à la tête de la PR #{n} : poussé tel quel, pas un squash (J3)"
        ));
    }
    if is_sha(head)
        && pr["merged_at"].as_str().is_some_and(|d| !d.is_empty())
        && pr["base"]["ref"].as_str() == Some("main")
        && n > 0
    {
        Ok((n, head.to_owned()))
    } else {
        Err(format!(
            "PR #{n} non fusionnée dans main par ce commit (J3)"
        ))
    }
}

/// Anomalies d'un commit (vide si conforme), avec une note pour l'affichage.
///
/// T3e-2 (AS-31 E3): a non-sensitive commit makes no GitHub read; its PR and `ci-ok` are enforced
/// by the ruleset of `main`. With `--origin` (AS-42), J3 by [`judge_squash`] in one read; on the
/// `<n>d` window, G3 too, by the same [`Repo::ci_ok`] as a sensitive commit (AS-44 AA2).
/// It must still have exactly one parent: sensitivity is computed against that parent, so a commit
/// with none or several could not be classified (and `required_linear_history` forbids it anyway).
///
/// V5 (AS-39): for a commit that touches a sensitive path, the PR (`commits/<sha>/pulls`: head,
/// `merged_at`, `merge_commit_sha`), G3, the approvals and then J4 use facts read from GitHub on
/// this run. GitHub reads per commit (#55, AS-39): none for a non-sensitive commit without
/// `--origin`; 1 with `all`; 3 on the `<n>d` window (J3, then the check runs and the workflow runs
/// of G3; the `ci.yml` id is read once per run); 4 for a sensitive commit (the same 3, plus its
/// comments), and more only when an approval must be carried over (AS-40).
fn check_commit(
    repo: &Repo,
    rules: &sensitive::Rules,
    section: &str,
    sha: &str,
    origin: Extra,
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
    if hits.is_empty() && origin == Extra::Nothing {
        return Ok((
            Vec::new(),
            "non sensible, PR et ci-ok imposés par le ruleset".to_owned(),
        ));
    }
    // J3 in one read (#55): for a sensitive commit too, this list read on this run gives the PR,
    // its head and its merge (V5); `pulls/<n>` is not read again.
    let pulls = repo.api_all(&format!(
        "repos/{}/commits/{sha}/pulls?per_page=100",
        repo.slug
    ))?;
    let (n, head) = match judge_squash(&pulls, sha) {
        Ok(pr) => pr,
        Err(why) => return Ok((vec![why], String::new())),
    };
    if hits.is_empty() {
        let mut note = format!("non sensible, squash de la PR #{n} (AS-42)");
        if origin == Extra::J3G3 {
            if PRE_CI.contains(&(n, sha)) {
                note.push_str(", avant la CI");
            } else if let Some(why) = repo.ci_ok(&head)? {
                return Ok((vec![why], note));
            } else {
                note.push_str(", ci-ok vert");
            }
        }
        return Ok((Vec::new(), note));
    }
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
                let rule = if carry.by_as55(&from) { " (AS-55)" } else { "" };
                note.push_str(&format!(", {tag} carried over from {}{rule}", &from[..7]));
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
    let log = match origin {
        Some(Origin::Since(_)) => repo.git(&["log", "--format=%H %ct", "origin/main"])?,
        _ => String::new(),
    };
    let dates = dates(&log)?;
    // Tip first, for the window of `Since`; shown oldest first.
    let mut extra = match origin {
        None => Extra::Nothing,
        Some(Origin::All) => Extra::J3,
        Some(Origin::Since(_)) => Extra::J3G3,
    };
    let mut verdicts = Vec::new();
    for &sha in history.iter().rev().filter(|s| !BASE.contains(*s)) {
        let (bad, note) = check_commit(repo, &rules, &section, sha, extra)?;
        if let Some(Origin::Since(cutoff)) = origin {
            let date = *dates
                .get(sha)
                .ok_or_else(|| format!("{sha} absent de git log"))?;
            // AS-44 AA3: only an old commit that passed J3 and G3 closes the window.
            if extra == Extra::J3G3 && date < cutoff && bad.is_empty() {
                extra = Extra::Nothing;
            }
        }
        verdicts.push((sha, bad, note));
    }
    let mut anomalies = Vec::new();
    for b in history.iter().filter(|s| BASE.contains(*s)) {
        println!("  {} base", &b[..7]);
    }
    for (sha, bad, note) in verdicts.into_iter().rev() {
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
                   d'administration ; l'orchestrateur le lance chaque jour depuis la copie \
                   principale (AS-50 AU3)";
        println!("  AVERTISSEMENT : {msg}");
        summary_warning(msg);
        return Ok(true);
    }
    let (code, out, err) = run_settings_check(repo)?;
    match code {
        Some(0) => Ok(true),
        Some(1) => {
            let body = format!(
                "`repo-settings.ps1 -Check` : les réglages GitHub du dépôt diffèrent de \
                 `tools/ci/repo-settings.json` d'`origin/main`, et les fusions sont suspendues \
                 tant que cette issue est ouverte (AS-50 AU2, AU3).\n\n\
                 ```\n{}\n```\n\nPour lever l'écart : corriger le fichier par une PR de chemin \
                 sensible, ou relancer `-Apply` depuis la copie principale (orchestrateur).",
                out.chars().take(4000).collect::<String>().trim_end()
            );
            repo.open_issue("repo-settings-drift", DRIFT_TITLE, &body)?;
            Ok(false)
        }
        _ => Err(format!(
            "repo-settings.ps1 -Check n'a pas pu conclure (code {code:?}) : {}",
            err.trim()
        )),
    }
}

/// Adds `msg` as a warning to the summary of the GitHub Actions job, when there is one (CI).
fn summary_warning(msg: &str) {
    // Not from the tests, which also run in CI.
    if let Some(path) = sys::env("GITHUB_STEP_SUMMARY").filter(|_| !cfg!(test)) {
        let path = std::path::PathBuf::from(path);
        let mut text = sys::read(&path).unwrap_or_default();
        text.extend_from_slice(format!("\n> [!WARNING]\n> {msg}\n").as_bytes());
        if let Err(e) = sys::write(&path, &text) {
            println!("  (résumé du travail non écrit : {e})");
        }
    }
}

/// AS-39 (#55): below this share of the GitHub API quota left, `verify-main` warns.
const QUOTA_ALERT_PERCENT: u64 = 20;

/// The core quota of a `rate_limit` answer: (requests left, limit per hour), and whether what is
/// left is under [`QUOTA_ALERT_PERCENT`] of the limit.
fn quota(v: &Value) -> Result<(u64, u64, bool), String> {
    let core = &v["resources"]["core"];
    match (core["remaining"].as_u64(), core["limit"].as_u64()) {
        (Some(left), Some(limit)) if limit > 0 && left <= limit => Ok((
            left,
            limit,
            left.saturating_mul(100) < limit.saturating_mul(QUOTA_ALERT_PERCENT),
        )),
        _ => Err("unreadable rate_limit answer".to_owned()),
    }
}

/// AS-39 (#55): the GitHub API quota left after the pass, in the log and, when it is low, as a
/// warning in the job summary. `rate_limit` costs no quota; it is read once, without the V4
/// retries, since this line never changes the result.
fn report_quota(repo: &Repo) {
    let read = repo
        .runner
        .run(&repo.main, "gh", &["api", "rate_limit"])
        .and_then(|(code, out, err)| match code {
            Some(0) => serde_json::from_slice(&out).map_err(|e| format!("unreadable answer: {e}")),
            _ => Err(err.trim().to_owned()),
        });
    match read.and_then(|v| quota(&v)) {
        Ok((left, limit, low)) => {
            println!("  GitHub API quota: {left} of {limit} requests left this hour");
            if low {
                let msg = format!(
                    "GitHub API quota low after verify-main: {left} of {limit} requests left \
                     (alert under {QUOTA_ALERT_PERCENT} %, AS-39, #55)"
                );
                println!("  WARNING: {msg}");
                if repo.ci {
                    println!("::warning::{msg}");
                }
                summary_warning(&msg);
            }
        }
        Err(e) => println!("  GitHub API quota not read: {e}"),
    }
}

/// `pwsh -NoProfile -NonInteractive -File <main copy>\tools\dev\repo-settings.ps1 -Check`; its
/// output is shown.
fn run_settings_check(repo: &Repo) -> Result<(Option<i32>, String, String), String> {
    let script = repo
        .main
        .join("tools")
        .join("dev")
        .join("repo-settings.ps1");
    let script = script.to_string_lossy();
    let args = ["-NoProfile", "-NonInteractive", "-File", &script, "-Check"];
    let (code, out, err) = repo.runner.run(&repo.main, "pwsh", &args)?;
    let out = String::from_utf8_lossy(&out).into_owned();
    print!("{out}");
    Ok((code, out, err))
}

/// `verify-main`, avec une *issue* par anomalie ; vrai si l'historique et les réglages GitHub sont
/// conformes. L'historique passe d'abord : un `-Check` qui échoue ne le cache pas. En `--dry-run`,
/// un `-Check` qui n'a pas pu conclure compte comme un écart.
pub fn verify_or_report(repo: &Repo, origin: Option<Origin>) -> Result<bool, String> {
    let anomalies = check(repo, origin)?;
    for (sha, bad) in &anomalies {
        let body = format!(
            "`cargo xtask verify-main` : le commit `{sha}` de `main` est en anomalie \
             (chemin sensible sans ses approbations, commit sans PR fusionnée, ou commit non \
             classable), et les fusions sont suspendues.\n\n- {}\n\nPour lever l'anomalie : l'inscrire dans `ACKNOWLEDGED` \
             (`tools/xtask/src/verify_main.rs`) par une PR de chemin sensible, avec `archsec-ok` (J5).",
            bad.join("\n- ")
        );
        repo.open_issue(sha, &format!("{ANOMALY_TITLE} {}", &sha[..7]), &body)?;
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
fn unix_now() -> i64 {
    crate::slots::now()
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
        let result = refresh(&repo).and_then(|()| {
            match origin {
                Some(Origin::All) => println!("  --origin all: J3 on every commit (AS-42)"),
                Some(Origin::Since(t)) => {
                    println!(
                        "  --origin: J3 and G3 on every commit since Unix time {t} (AS-42, AS-44)"
                    );
                }
                None => {}
            }
            verify_or_report(&repo, origin)
        });
        // Whatever the result, a failure included (#55).
        report_quota(&repo);
        result
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
    use crate::github::{Output, Run, issues_path, owner_item};

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

    /// Answers `repo-settings.ps1 -Check` with a fixed exit code, and the anomaly issue list with `[]`.
    struct Settings(Option<i32>, Rc<RefCell<Vec<String>>>);

    impl Run for Settings {
        fn run(&self, _dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
            let line = format!("{prog} {}", args.join(" "));
            self.1.borrow_mut().push(line.clone());
            match prog {
                "pwsh" => Ok((self.0, b"  ECART : x\n".to_vec(), "erreur".to_owned())),
                "gh" if line == format!("gh api --paginate {}", issues_path("o/r")) => {
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
            ci_workflow: Default::default(),
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
        assert!(calls.iter().any(|c| c.ends_with(&issues_path("o/r"))));
        assert!(!calls.iter().any(|c| c.starts_with("gh issue create")));
        for code in [Some(2), Some(64), None] {
            assert!(settings(code, false, false).0.is_err(), "{code:?}");
        }
        let (r, calls) = settings(Some(1), true, false);
        assert_eq!(r, Ok(true));
        assert!(calls.is_empty(), "{calls:?}");
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
            ci_workflow: Default::default(),
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
            (
                "git cat-file blob origin/pilotage:architecture-securite.md".to_owned(),
                carnet,
            ),
            (
                "git cat-file blob origin/main:tools/ci/sensitive-paths.txt".to_owned(),
                include_str!("../../ci/sensitive-paths.txt").to_owned(),
            ),
            (
                "git rev-list --reverse origin/main".to_owned(),
                format!("{}\n{PARENT}\n{COMMIT}\n", BASE[0]),
            ),
            (
                "git rev-list --parents".to_owned(),
                format!("{COMMIT} {PARENT}"),
            ),
            ("git diff --name-only".to_owned(), format!("{file}\0")),
            (
                format!("{s}/commits/{COMMIT}/pulls"),
                json!([{"number": 7, "merge_commit_sha": COMMIT,
                        "merged_at": "2026-10-30T10:00:00Z", "base": {"ref": "main"},
                        "head": {"sha": HEAD}}])
                .to_string(),
            ),
            (
                "git log --format=%H %ct origin/main".to_owned(),
                format!("{COMMIT} 2000\n{PARENT} 1000\n"),
            ),
            (
                "gh api repos/o/r/actions/workflows/ci.yml".to_owned(),
                json!({"id": 9}).to_string(),
            ),
            (format!("{s}/commits/{HEAD}/check-runs"), runs.to_string()),
            (format!("{s}/actions/runs"), wfr.to_string()),
            (
                format!("{s}/issues/7/comments"),
                json!([
                    owner_item(&format!("review-ok: {HEAD}")),
                    owner_item(&format!("archsec-ok: {HEAD}"))
                ])
                .to_string(),
            ),
            (
                format!("gh api --paginate {}", issues_path("o/r")),
                "[]".to_owned(),
            ),
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
            ci_workflow: Default::default(),
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
        let list: Vec<_> = bodies.iter().map(|b| owner_item(b)).collect();
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
    /// base, two PRs, or a commit that is the PR head itself is an anomaly. A sensitive commit
    /// keeps its full check (V5).
    #[test]
    fn origin_needs_a_merged_squash_in_one_read() {
        let all = Some(Origin::All);
        let (r, calls) = origin_check("docs/a.md", false, all, &[]);
        assert_eq!(r, Ok(vec![]), "{calls:#?}");
        assert_eq!(gh_calls(&calls), 1, "{calls:#?}");
        let merged = json!({"number": 7, "merge_commit_sha": COMMIT,
                            "merged_at": "2026-10-30T10:00:00Z", "base": {"ref": "main"},
                            "head": {"sha": HEAD}});
        let mut cases = vec![json!([]), json!([merged.clone(), merged.clone()])];
        // Review of #73: a PR head pushed as is to main, which GitHub then marks merged with
        // merge_commit_sha set to it; and a head that cannot be read.
        for (k, v) in [
            ("merged_at", Value::Null),
            ("merge_commit_sha", json!(HEAD)),
            ("base", json!({"ref": "dev"})),
            ("head", json!({"sha": COMMIT})),
            ("head", Value::Null),
        ] {
            let mut p = merged.clone();
            p[k] = v;
            cases.push(json!([p]));
        }
        for c in &cases {
            let (r, calls) = origin_check("docs/a.md", false, all, &pulls(c));
            assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{c}: {r:?}");
            assert!(reasons(&r).contains("(J3)"), "{c}: {r:?}");
            assert_eq!(gh_calls(&calls), 1, "{calls:#?}");
        }
        let (r, _) = origin_check("tools/xtask/src/x.rs", true, all, &[]);
        assert_eq!(r, Ok(vec![]));
    }

    /// AS-44: on the `<n>d` window, a non-sensitive squash also needs `ci-ok` green on its head
    /// (AA2, same `ci_ok` as a sensitive commit), and a PR head pushed as is to `main` fails J3
    /// (AA1).
    #[test]
    fn window_needs_ci_ok_and_a_new_commit() {
        let week = Some(Origin::Since(1000));
        let (r, calls) = origin_check("docs/a.md", false, week, &[]);
        assert_eq!(r, Ok(vec![]), "{calls:#?}");
        assert!(
            calls.iter().any(|c| c.contains("/check-runs")),
            "{calls:#?}"
        );
        let no_ci = &no_pr_no_ci()[1..];
        let (r, _) = origin_check("docs/a.md", false, week, no_ci);
        assert!(r.as_ref().is_ok_and(|a| a.len() == 1), "{r:?}");
        assert!(reasons(&r).contains("ci-ok absent"), "{r:?}");
        let pushed = json!([{"number": 7, "merge_commit_sha": COMMIT, "merged_at": "x",
                             "base": {"ref": "main"}, "head": {"sha": COMMIT}}]);
        let (r, calls) = origin_check("docs/a.md", false, week, &pulls(&pushed));
        assert!(reasons(&r).contains("poussé tel quel"), "{r:?}");
        assert_eq!(gh_calls(&calls), 1, "{calls:#?}");
    }

    /// How a commit of [`chain`] came to `main`.
    #[derive(Clone, Copy)]
    enum Via {
        /// Pushed without a PR.
        Push,
        /// Squash of a merged PR, `ci-ok` green on its head.
        Squash,
        /// Squash of a merged PR whose head has no `ci-ok` (admin merge, ruleset off).
        NoCi,
    }

    /// Head without `ci-ok` in [`chain`].
    const NO_CI: &str = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

    /// Non-sensitive history after the base, oldest first: (commit digit, committer date, how it
    /// came). Returns the anomalous digits and the digits whose PR was read.
    fn chain(commits: &[(char, i64, Via)], origin: Origin) -> (Vec<char>, Vec<char>) {
        let sha = |d: char| d.to_string().repeat(40);
        let mut log: Vec<String> = commits
            .iter()
            .map(|&(d, t, _)| format!("{} {t}", sha(d)))
            .collect();
        log.reverse();
        let list: Vec<String> = commits.iter().map(|c| sha(c.0)).collect();
        let mut over = vec![
            (
                "git rev-list --reverse origin/main".to_owned(),
                format!("{}\n{PARENT}\n{}\n", BASE[0], list.join("\n")),
            ),
            (
                "git log --format=%H %ct origin/main".to_owned(),
                log.join("\n"),
            ),
            (
                format!("gh api --paginate repos/o/r/commits/{NO_CI}/check-runs"),
                json!({"check_runs": []}).to_string(),
            ),
        ];
        for (n, &(d, _, via)) in (1u64..).zip(commits) {
            let s = sha(d);
            let pr = |head: &str| {
                json!([{"number": n, "merge_commit_sha": s, "merged_at": "x",
                        "base": {"ref": "main"}, "head": {"sha": head}}])
            };
            let v = match via {
                Via::Push => json!([]),
                Via::Squash => pr(HEAD),
                Via::NoCi => pr(NO_CI),
            };
            over.push((
                format!("gh api --paginate repos/o/r/commits/{s}/pulls"),
                v.to_string(),
            ));
        }
        let (r, calls) = origin_check("docs/a.md", false, Some(origin), &over);
        let first = |s: &str| s.chars().next().unwrap();
        let bad = r.unwrap().iter().map(|(s, _)| first(s)).collect();
        let read: Vec<char> = commits
            .iter()
            .filter(|c| {
                calls
                    .iter()
                    .any(|l| l.contains(&format!("{}/pulls", sha(c.0))))
            })
            .map(|c| c.0)
            .collect();
        // #55: the `ci.yml` workflow id is read once per pass when G3 finds a `ci-ok` (a squash
        // read on the `<n>d` window), however many commits it judges; never otherwise.
        let g3 = matches!(origin, Origin::Since(_))
            && commits
                .iter()
                .any(|c| matches!(c.2, Via::Squash) && read.contains(&c.0));
        let ids = calls.iter().filter(|c| c.contains("workflows/ci.yml"));
        assert_eq!(ids.count(), usize::from(g3), "{calls:#?}");
        (bad, read)
    }

    /// AS-42, AS-44 AA3: `--origin <n>d` reads from the tip down to the first commit dated before
    /// the limit that passes J3 and G3, included; older ones are left to `all`. Backdated commits
    /// pushed without a PR, or squashed without `ci-ok`, all fail, so the walk goes through them,
    /// a backdated tip included.
    #[test]
    fn origin_window_is_respected() {
        use Via::{NoCi, Push, Squash};
        let since = Origin::Since(3000);
        // 2 is the first old commit and passes: 1, without a PR, is not read.
        let ok = [('1', 900, Push), ('2', 1000, Squash), ('3', 5000, Squash)];
        assert_eq!(chain(&ok, since), (vec![], vec!['2', '3']));
        assert_eq!(chain(&ok, Origin::All), (vec!['1'], vec!['1', '2', '3']));
        // 4 and 3 were pushed backdated without a PR, under a fresh squash 5.
        let forged = [
            ('1', 700, Push),
            ('2', 800, Squash),
            ('3', 900, Push),
            ('4', 1000, Push),
            ('5', 5000, Squash),
        ];
        assert_eq!(
            chain(&forged, since),
            (vec!['3', '4'], vec!['2', '3', '4', '5'])
        );
        assert_eq!(
            chain(&forged[..4], since),
            (vec!['3', '4'], vec!['2', '3', '4'])
        );
        // AA3: 3, old and squashed without ci-ok, does not close the window; 2 does.
        let no_ci = [
            ('1', 700, Push),
            ('2', 800, Squash),
            ('3', 900, NoCi),
            ('4', 5000, Squash),
        ];
        assert_eq!(chain(&no_ci, since), (vec!['3'], vec!['2', '3', '4']));
        // `all` stays J3 alone (#55): 3 passes there.
        assert_eq!(
            chain(&no_ci, Origin::All),
            (vec!['1'], vec!['1', '2', '3', '4'])
        );
        assert_eq!(Origin::parse("7d", 1_000_000), Some(Origin::Since(395_200)));
        assert_eq!(Origin::parse("all", 0), Some(Origin::All));
        for bad in ["0d", "367d", "7", "d", "-1d", "+7d", "07d", "7h", ""] {
            assert_eq!(Origin::parse(bad, 0), None, "{bad}");
        }
        assert!(dates("a x\n").is_err() && dates("a\n").is_err());
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
    /// nothing is kept between runs. #55: in 4 reads, plus the `ci.yml` id once per pass; the PR
    /// comes from `commits/<sha>/pulls`, and `pulls/<n>` is no longer read.
    #[test]
    fn sensitive_commit_reads_github_every_run() {
        for _ in 0..2 {
            let (r, calls) = history_check("tools/xtask/src/x.rs", true, &[]);
            assert_eq!(r, Ok(vec![]));
            for read in [
                &format!("/commits/{COMMIT}/pulls") as &str,
                "/check-runs",
                &format!("/actions/runs?head_sha={HEAD}&"),
                "/issues/7/comments",
                "/actions/workflows/ci.yml",
            ] {
                let n = calls.iter().filter(|c| c.contains(read)).count();
                assert_eq!(n, 1, "{read}: {calls:#?}");
            }
            assert_eq!(gh_calls(&calls), 5, "{calls:#?}");
            assert!(!calls.iter().any(|c| c.contains("/pulls/7")), "{calls:#?}");
        }
    }

    /// #55, AS-39: on the `<n>d` window, a non-sensitive commit costs 3 reads (J3, then the check
    /// runs and the workflow runs of G3), plus the `ci.yml` id once per pass.
    #[test]
    fn window_commit_costs_three_reads() {
        let (r, calls) = origin_check("docs/a.md", false, Some(Origin::Since(1000)), &[]);
        assert_eq!(r, Ok(vec![]), "{calls:#?}");
        assert_eq!(gh_calls(&calls), 4, "{calls:#?}");
    }

    /// AS-39 (#55): the quota left is read from `rate_limit`; under 20 % of the limit, an alert.
    #[test]
    fn quota_alert_under_a_fifth() {
        let q = |left: u64, limit: u64| {
            quota(&json!({"resources": {"core": {"remaining": left, "limit": limit}}}))
        };
        assert_eq!(q(1000, 1000), Ok((1000, 1000, false)));
        assert_eq!(q(200, 1000), Ok((200, 1000, false)));
        assert_eq!(q(199, 1000), Ok((199, 1000, true)));
        assert_eq!(q(0, 5000), Ok((0, 5000, true)));
        for bad in [
            json!({}),
            json!({"resources": {"core": {"remaining": 1}}}),
            json!({"resources": {"core": {"remaining": 1, "limit": 0}}}),
            json!({"resources": {"core": {"remaining": 9, "limit": 5}}}),
            json!({"resources": {"core": {"remaining": "1", "limit": 5}}}),
        ] {
            assert!(quota(&bad).is_err(), "{bad}");
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
                json!([{"number": 7, "merge_commit_sha": x, "merged_at": "x",
                        "base": {"ref": "main"}, "head": {"sha": h}}])
                .to_string(),
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
            (format!("{s}/actions/runs"), wfr.to_string()),
            (
                format!("{s}/issues/7/comments"),
                json!([
                    owner_item(&format!("review-ok: {}", c.a)),
                    owner_item(&format!("archsec-ok: {}", c.a))
                ])
                .to_string(),
            ),
        ]);
        let (repo, _) = c.lab.repo(answers);
        let rules = sensitive::Rules::parse("tools/xtask/\n").unwrap();
        let j4 = if j4_on_a { &c.a } else { &c.h };
        let section = format!("## 9. Avis\narchsec-ok: {j4}\n");
        let (bad, note) = check_commit(&repo, &rules, &section, &x, Extra::Nothing).unwrap();
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
