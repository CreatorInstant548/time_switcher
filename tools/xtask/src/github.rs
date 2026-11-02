//! Accès à Git et à GitHub pour `merge` et `verify-main`. Tout est en lecture, sauf
//! [`Repo::open_issue`] ; la fusion elle-même reste dans `merge`. Un appel qui échoue ou une
//! réponse illisible est une erreur, jamais une réussite.
//! spec: carnet architecture-securite.md SEC0-15, SEC0-17 ; §9.6 G3 ; §9.8 J3, J5

use std::cell::OnceCell;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;

use crate::sys;

/// Seul workflow dont le travail `ci-ok` compte (G3).
pub const CI_WORKFLOW: &str = ".github/workflows/ci.yml";
/// Étiquette des *issues* d'anomalie.
const ISSUE_LABEL: &str = "role:architecture";

/// Sortie d'une commande : code (`None` si tuée), sortie standard brute, sortie d'erreur.
pub type Output = (Option<i32>, Vec<u8>, String);

/// Lanceur des commandes `git` et `gh` : le vrai ([`System`]), ou un enregistreur dans les tests,
/// qui prouve par exemple qu'un `--dry-run` n'appelle jamais `gh pr merge` (AS-23 point 2).
pub trait Run {
    fn run(&self, dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String>;

    /// Wait before retrying a GitHub read (V4); tests replace it so they never sleep.
    fn pause(&self, d: Duration) {
        sys::sleep(d);
    }
}

/// HTTP statuses of a failed `gh api` read that are retried as transient (V4).
const TRANSIENT: [u16; 4] = [500, 502, 503, 504];
/// Pauses before the 2nd and 3rd attempts: 3 attempts at most (V4).
const BACKOFF: [u64; 2] = [2, 5];

/// HTTP status quoted by `gh` in its error output (`gh: Not Found (HTTP 404)`, `HTTP 502: …`).
pub fn http_status(stderr: &str) -> Option<u16> {
    stderr.match_indices("HTTP ").find_map(|(i, m)| {
        let rest = &stderr[i + m.len()..];
        let code = rest.get(..3)?;
        let whole = !rest[3..].starts_with(|c: char| c.is_ascii_digit());
        (whole && code.bytes().all(|b| b.is_ascii_digit()))
            .then(|| code.parse().ok())
            .flatten()
    })
}

/// `gh api` arguments that make a plain `GET`: no method, field or input option (V4). Only such a
/// read is ever retried.
pub fn is_get(args: &[&str]) -> bool {
    args.first() == Some(&"api")
        && args[1..]
            .iter()
            .all(|a| *a == "--paginate" || !a.starts_with('-'))
}

pub struct System;

impl Run for System {
    fn run(&self, dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
        sys::outputs(dir, prog, args)
    }
}

pub struct Repo {
    /// Copie principale (parent du dossier Git commun).
    pub main: PathBuf,
    /// Copie de travail d'où la commande est lancée.
    pub here: PathBuf,
    /// `propriétaire/dépôt`, tiré du distant `origin`.
    pub slug: String,
    pub dry_run: bool,
    /// Mode CI (A2) : les réglages GitHub n'y sont pas lisibles (`GITHUB_TOKEN` sans droits
    /// d'administration), `verify-main` saute leur contrôle avec un avertissement (AS-30 R5).
    pub ci: bool,
    /// `TS_CACHE`, remplaçable en test.
    pub cache: PathBuf,
    pub runner: Box<dyn Run>,
    /// Id of the `ci.yml` workflow, read once per run however many commits G3 judges (#55).
    pub ci_workflow: OnceCell<u64>,
}

/// Même chemin une fois résolus les liens et les jonctions, sans tenir compte de la casse.
pub fn same_path(a: &Path, b: &Path) -> bool {
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a
            .to_string_lossy()
            .eq_ignore_ascii_case(&b.to_string_lossy()),
        _ => false,
    }
}

/// `propriétaire/dépôt` d'une adresse GitHub (https ou ssh) ; `None` pour tout autre hôte.
pub fn slug_of(url: &str) -> Option<String> {
    let rest = url
        .trim()
        .strip_prefix("https://github.com/")
        .or_else(|| url.trim().strip_prefix("git@github.com:"))?;
    let rest = rest.strip_suffix(".git").unwrap_or(rest);
    let (owner, repo) = rest.split_once('/')?;
    let ok = |s: &str| {
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
    };
    (ok(owner) && ok(repo) && !repo.contains('/')).then(|| format!("{owner}/{repo}"))
}

/// Valeurs JSON d'une sortie de `gh api --paginate` : pages concaténées, tableaux aplatis.
pub fn pages(text: &str) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for v in serde_json::Deserializer::from_str(text).into_iter::<Value>() {
        match v.map_err(|e| format!("réponse de GitHub illisible : {e}"))? {
            Value::Array(items) => out.extend(items),
            other => out.push(other),
        }
    }
    Ok(out)
}

/// Items of the list `key` in every page of a paginated object answer (`check_runs`,
/// `workflow_runs`); a page without that list is unreadable.
fn page_items(pages: &[Value], key: &str) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for p in pages {
        let items = p[key]
            .as_array()
            .ok_or_else(|| format!("liste {key} illisible"))?;
        out.extend(items.iter().cloned());
    }
    Ok(out)
}

impl Repo {
    /// `github` faux : distant hors de GitHub permis (essais dans un dépôt jetable) ; le
    /// `slug` reste vide et toute *issue* échoue.
    pub fn open(dry_run: bool, github: bool) -> Result<Self, String> {
        let cwd =
            std::env::current_dir().map_err(|e| format!("dossier courant illisible : {e}"))?;
        let git = |args: &[&str]| sys::stdout_of(&cwd, "git", args).map(|s| s.trim().to_owned());
        let here = PathBuf::from(git(&["rev-parse", "--show-toplevel"])?);
        let common = PathBuf::from(git(&[
            "rev-parse",
            "--path-format=absolute",
            "--git-common-dir",
        ])?);
        let main = common
            .parent()
            .ok_or("dossier Git commun sans parent")?
            .to_path_buf();
        let url = git(&["config", "--get", "remote.origin.url"])?;
        let slug = match slug_of(&url) {
            Some(s) => s,
            None if !github => String::new(),
            None => return Err(format!("distant origin hors de github.com : {url}")),
        };
        Ok(Self {
            main,
            here,
            slug,
            dry_run,
            ci: crate::steps::ci_mode(),
            cache: sys::ts_cache(),
            runner: Box::new(System),
            ci_workflow: OnceCell::new(),
        })
    }

    /// Sortie brute d'une commande qui doit réussir ; sa sortie d'erreur est dans le refus.
    pub fn out(&self, dir: &Path, prog: &str, args: &[&str]) -> Result<Vec<u8>, String> {
        let (code, out, err) = self.runner.run(dir, prog, args)?;
        if code != Some(0) {
            return Err(format!(
                "échec de « {prog} {} » : {}",
                args.join(" "),
                err.trim()
            ));
        }
        Ok(out)
    }

    /// Sortie texte d'une commande qui doit réussir, sans espaces de fin.
    pub fn text(&self, dir: &Path, prog: &str, args: &[&str]) -> Result<String, String> {
        let out = self.out(dir, prog, args)?;
        let s = String::from_utf8(out).map_err(|_| format!("sortie non UTF-8 de {prog}"))?;
        Ok(s.trim_end().to_owned())
    }

    /// Vrai si la commande sort avec le code 0.
    pub fn ok(&self, dir: &Path, prog: &str, args: &[&str]) -> Result<bool, String> {
        Ok(self.runner.run(dir, prog, args)?.0 == Some(0))
    }

    /// `git` dans la copie principale ; sortie sans espaces de fin.
    pub fn git(&self, args: &[&str]) -> Result<String, String> {
        self.text(&self.main, "git", args)
    }

    /// `gh` dans la copie principale.
    pub fn gh(&self, args: &[&str]) -> Result<String, String> {
        self.text(&self.main, "gh", args)
    }

    /// Contenu de `<rév>:<chemin>`, `None` si le chemin n'y existe pas. La révision doit exister :
    /// sinon c'est une erreur, et non un fichier absent.
    pub fn blob(&self, dir: &Path, rev: &str, path: &str) -> Result<Option<Vec<u8>>, String> {
        if !self.ok(
            dir,
            "git",
            &["cat-file", "-e", &format!("{rev}^{{commit}}")],
        )? {
            return Err(format!("révision {rev} introuvable dans {}", dir.display()));
        }
        let spec = format!("{rev}:{path}");
        if !self.ok(dir, "git", &["cat-file", "-e", &spec])? {
            return Ok(None);
        }
        self.out(dir, "git", &["cat-file", "blob", &spec]).map(Some)
    }

    /// `gh api` en lecture, toutes pages.
    pub fn api_all(&self, path: &str) -> Result<Vec<Value>, String> {
        self.read_api(&["api", "--paginate", path], pages)
    }

    pub fn api(&self, path: &str) -> Result<Value, String> {
        self.read_api(&["api", path], |t| {
            serde_json::from_str(t)
                .map_err(|e| format!("réponse de GitHub illisible pour {path} : {e}"))
        })
    }

    /// A `gh api` `GET`, retried with bounded backoff on HTTP 500, 502, 503 or 504 and on an
    /// empty or unparsable (truncated) response; never on 4xx, never for a write (V4). After the
    /// last attempt the error stays a refusal. `parse` must only decode JSON.
    fn read_api<T>(
        &self,
        args: &[&str],
        parse: impl Fn(&str) -> Result<T, String>,
    ) -> Result<T, String> {
        let retry = is_get(args);
        let mut attempt = 0;
        loop {
            let (code, out, err) = self.runner.run(&self.main, "gh", args)?;
            let (why, transient) = if code == Some(0) {
                match String::from_utf8(out) {
                    Ok(t) if !t.trim().is_empty() => match parse(&t) {
                        Ok(v) => return Ok(v),
                        Err(e) => (e, true),
                    },
                    _ => (
                        format!("empty or non UTF-8 answer to gh {}", args.join(" ")),
                        true,
                    ),
                }
            } else {
                let status = http_status(&err);
                // #55: an exhausted quota (403 or 429) is said plainly; it is never retried.
                let quota = if err.to_ascii_lowercase().contains("rate limit") {
                    "GitHub API quota exhausted (rate limit), retry after its reset: "
                } else {
                    ""
                };
                (
                    format!("{quota}échec de « gh {} » : {}", args.join(" "), err.trim()),
                    status.is_some_and(|s| TRANSIENT.contains(&s)),
                )
            };
            let Some(wait) = BACKOFF.get(attempt).filter(|_| retry && transient) else {
                return Err(why);
            };
            println!(
                "  GitHub read failed, attempt {} of {}, retrying in {wait} s: {why}",
                attempt + 1,
                BACKOFF.len() + 1
            );
            self.runner.pause(Duration::from_secs(*wait));
            attempt += 1;
        }
    }

    /// Bodies of the PR comments written by the repository owner ([`by_owner`], AS-45 P3); every
    /// other comment is ignored. Used by `merge`, `verify-main` and the carry-over (AS-40).
    pub fn comments(&self, pr: u64) -> Result<Vec<String>, String> {
        let all = self.api_all(&format!(
            "repos/{}/issues/{pr}/comments?per_page=100",
            self.slug
        ))?;
        all.iter()
            .filter(|c| by_owner(c, &self.slug))
            .map(|c| {
                c["body"]
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| "commentaire sans corps".to_owned())
            })
            .collect()
    }

    /// `ci-ok` sur `sha` selon G3 : `Ok(None)` si vert, `Ok(Some(motif))` sinon ; `Err` si GitHub
    /// ne répond pas. Two reads (#55): the `ci-ok` check runs of `sha`, then, only if there is one,
    /// the workflow runs of `sha` in a single list (`head_sha`), never one read per check suite.
    /// The `ci.yml` workflow id is read once per run ([`Repo::ci_workflow`]).
    pub fn ci_ok(&self, sha: &str) -> Result<Option<String>, String> {
        let s = &self.slug;
        let wid = self.ci_workflow_id()?;
        let pages = self.api_all(&format!(
            "repos/{s}/commits/{sha}/check-runs?check_name=ci-ok&per_page=100"
        ))?;
        let runs = page_items(&pages, "check_runs")?;
        if runs.is_empty() {
            return Ok(judge_ci(&runs, &[], sha, wid, s).err());
        }
        let pages = self.api_all(&format!(
            "repos/{s}/actions/runs?head_sha={sha}&per_page=100"
        ))?;
        let workflow_runs = page_items(&pages, "workflow_runs")?;
        let mut suites = Vec::new();
        for r in &runs {
            let id = r["check_suite"]["id"]
                .as_u64()
                .ok_or("check_suite illisible")?;
            let of_suite = workflow_runs
                .iter()
                .filter(|w| w["check_suite_id"].as_u64() == Some(id))
                .cloned()
                .collect();
            suites.push((id, of_suite));
        }
        Ok(judge_ci(&runs, &suites, sha, wid, s).err())
    }

    /// Id of `.github/workflows/ci.yml`, read on GitHub the first time only (#55).
    fn ci_workflow_id(&self) -> Result<u64, String> {
        if let Some(id) = self.ci_workflow.get() {
            return Ok(*id);
        }
        let wf = self.api(&format!("repos/{}/actions/workflows/ci.yml", self.slug))?;
        let id = wf["id"]
            .as_u64()
            .ok_or("identifiant du workflow ci.yml illisible")?;
        Ok(*self.ci_workflow.get_or_init(|| id))
    }

    /// Ouvre une *issue* d'anomalie `role:architecture`, ou réutilise celle, ouverte, qui porte
    /// déjà `key` (J5). Avec `--dry-run`, affiche seulement celle qu'il ouvrirait. `title` must
    /// start with one of [`ANOMALY_TITLES`]; only an issue that [`tracks`] `key` is reused, so a
    /// reused issue always stops `merge` (AS-50 AU2): another issue that merely quotes the key
    /// (a remontée, a note) never absorbs the anomaly.
    pub fn open_issue(&self, key: &str, title: &str, body: &str) -> Result<(), String> {
        if self.slug.is_empty() {
            return Err(format!(
                "distant hors de GitHub : issue « {title} » impossible"
            ));
        }
        if !ANOMALY_TITLES.iter().any(|a| title.starts_with(a)) {
            return Err(format!(
                "issue « {title} » : not an anomaly title (AS-50 AU2)"
            ));
        }
        // Read through the API, which gives the author association that `gh issue list` lacks:
        // only an issue of the owner can stand for the anomaly (AS-45 P3).
        let list = self.api_all(&issues_path(&self.slug))?;
        if let Some(i) = list.iter().find(|i| tracks(i, &self.slug, key)) {
            println!(
                "  issue #{} déjà ouverte pour {key} : réutilisée",
                i["number"]
            );
            return Ok(());
        }
        let body = format!(
            "{body}\n\n{}\n\nOuverte par `cargo xtask` (SEC0-17, SEC0-35).",
            key_marker(key)
        );
        if self.dry_run {
            println!("  --dry-run : ouvrirait l'issue « {title} » ({ISSUE_LABEL}) :\n{body}");
            return Ok(());
        }
        let tmp = self.cache.join("tmp");
        std::fs::create_dir_all(&tmp).map_err(|e| format!("{} : {e}", tmp.display()))?;
        let file = tmp.join(format!("xtask-issue-{}.md", std::process::id()));
        sys::write(&file, body.as_bytes())?;
        let f = file.to_string_lossy().into_owned();
        let out = self.gh(&[
            "issue",
            "create",
            "-R",
            &self.slug,
            "--title",
            title,
            "--label",
            ISSUE_LABEL,
            "--body-file",
            &f,
        ]);
        let _ = std::fs::remove_file(&file); // corps recopié dans l'issue
        println!("  issue ouverte : {}", out?.trim());
        Ok(())
    }
}

/// Décision G3 sur les contrôles `ci-ok` d'un SHA et les *workflow runs* de leurs *check suites*.
/// Tous les `ci-ok` doivent être verts ; au moins un doit venir de `ci.yml` de ce dépôt.
pub fn judge_ci(
    runs: &[Value],
    suites: &[(u64, Vec<Value>)],
    sha: &str,
    wid: u64,
    slug: &str,
) -> Result<(), String> {
    if runs.is_empty() {
        return Err(format!("ci-ok absent sur {sha}"));
    }
    let mut from_ci = 0;
    for r in runs {
        let (status, conclusion) = (
            r["status"].as_str().unwrap_or("?"),
            r["conclusion"].as_str().unwrap_or("aucune"),
        );
        if r["name"].as_str() != Some("ci-ok") || r["head_sha"].as_str() != Some(sha) {
            return Err(format!(
                "contrôle inattendu dans la réponse ({})",
                r["name"]
            ));
        }
        if status != "completed" || conclusion != "success" {
            return Err(format!(
                "un contrôle ci-ok n'est pas vert sur {sha} ({status}, {conclusion})"
            ));
        }
        let id = r["check_suite"]["id"].as_u64();
        let wfr = suites
            .iter()
            .find(|(s, _)| Some(*s) == id)
            .map(|(_, w)| w.as_slice())
            .unwrap_or_default();
        if wfr.iter().any(|w| {
            w["path"].as_str() == Some(CI_WORKFLOW)
                && w["workflow_id"].as_u64() == Some(wid)
                && w["head_sha"].as_str() == Some(sha)
                && w["check_suite_id"].as_u64() == id
                && w["repository"]["full_name"]
                    .as_str()
                    .is_some_and(|n| n.eq_ignore_ascii_case(slug))
        }) {
            from_ci += 1;
        }
    }
    match from_ci {
        0 => Err(format!("aucun ci-ok de {CI_WORKFLOW} sur {sha} (G3)")),
        _ => Ok(()),
    }
}

/// Title of the issue that `verify-main` opens for an anomalous commit of `main`, followed by
/// its short SHA.
pub const ANOMALY_TITLE: &str = "verify-main : anomalie sur le commit";
/// Title of the issue that `verify-main` opens for a drift of the GitHub settings (`-Check`).
pub const DRIFT_TITLE: &str =
    "verify-main : réglages GitHub différents de tools/ci/repo-settings.json";
/// Title prefixes of the anomaly issues that `cargo xtask` opens (AS-50 AU2, AU3).
pub const ANOMALY_TITLES: [&str; 2] = [ANOMALY_TITLE, DRIFT_TITLE];
/// Exact title of the issue that the `report` job of `nightly.yml` opens when its `verify-main`
/// job fails or is cancelled (AS-52, AU8). The workflow writes the same text (test
/// `nightly_writes_the_anomaly_title`).
pub const NIGHTLY_ANOMALY_TITLE: &str = "verify-main : anomalie détectée par la tâche périodique";
/// Login of the GitHub Actions bot, author of the issues opened with the job token (AS-52).
const ACTIONS_BOT: &str = "github-actions[bot]";

/// An anomaly issue that `cargo xtask` opens: an issue (not a pull request) written by the owner
/// (P3), whose title starts with one of [`ANOMALY_TITLES`]. The only kind [`tracks`] reuses.
fn owner_anomaly(item: &Value, slug: &str) -> bool {
    item.get("pull_request").is_none()
        && by_owner(item, slug)
        && item["title"]
            .as_str()
            .is_some_and(|t| ANOMALY_TITLES.iter().any(|a| t.starts_with(a)))
}

/// AS-52 (AU8): the issue of the nightly `report` job, an issue (not a pull request) whose title
/// is exactly [`NIGHTLY_ANOMALY_TITLE`] and whose author is exactly [`ACTIONS_BOT`] with account
/// type `Bot`. A user login cannot contain `[bot]`, and only collaborators can open issues; any
/// workflow of this repository with `issues: write` could open it too, which only stops merges.
fn nightly_anomaly(item: &Value) -> bool {
    item.get("pull_request").is_none()
        && item["user"]["login"].as_str() == Some(ACTIONS_BOT)
        && item["user"]["type"].as_str() == Some("Bot")
        && item["title"].as_str() == Some(NIGHTLY_ANOMALY_TITLE)
}

/// AS-50 AU2 and AS-52, the one definition of an anomaly issue: one of the owner (P3) with an
/// anomaly title of `cargo xtask verify-main`, or the nightly issue of `github-actions[bot]`
/// ([`NIGHTLY_ANOMALY_TITLE`]). An open one stops `cargo xtask merge`. Counting more issues can
/// only stop a merge, never let one through.
pub fn is_anomaly(item: &Value, slug: &str) -> bool {
    owner_anomaly(item, slug) || nightly_anomaly(item)
}

/// Line written by [`Repo::open_issue`] in the body of the issue for `key`.
fn key_marker(key: &str) -> String {
    format!("Clé : `{key}`")
}

/// The owner's anomaly issue whose body carries the marker of `key`: the only kind of issue
/// [`Repo::open_issue`] reuses for `key`. Never the nightly bot issue (AS-52): its body is not
/// written by `cargo xtask`, and it is closed once the night's anomaly is cleared.
pub fn tracks(item: &Value, slug: &str, key: &str) -> bool {
    owner_anomaly(item, slug)
        && item["body"]
            .as_str()
            .is_some_and(|b| b.lines().any(|l| l.trim_end() == key_marker(key)))
}

/// API path of the open anomaly issues (`role:architecture`), all pages.
pub fn issues_path(slug: &str) -> String {
    format!("repos/{slug}/issues?state=open&labels={ISSUE_LABEL}&per_page=100")
}

/// AS-45 P3: a comment or issue counts only when its author is the repository owner: login equal
/// to the owner part of `slug` (logins are unique regardless of case), `author_association` equal
/// to `OWNER`, and account type `User`. On a public repository anyone can write
/// `review-ok: <sha>`; the three conditions are all required.
pub fn by_owner(item: &Value, slug: &str) -> bool {
    let Some((owner, _)) = slug.split_once('/') else {
        return false;
    };
    !owner.is_empty()
        && item["user"]["login"]
            .as_str()
            .is_some_and(|l| l.eq_ignore_ascii_case(owner))
        && item["author_association"].as_str() == Some("OWNER")
        && item["user"]["type"].as_str() == Some("User")
}

/// A comment or issue of the owner `o` of the test slug `o/r`, as the API returns it.
#[cfg(test)]
pub fn owner_item(body: &str) -> Value {
    serde_json::json!({"body": body, "user": {"login": "o", "type": "User"},
                       "author_association": "OWNER"})
}

/// Un commentaire dont la **première ligne** est exactement `<tag>: <sha>` (I3) : une ligne citée
/// plus bas, dans un avis défavorable par exemple, ne vaut pas approbation.
pub fn approved(bodies: &[String], tag: &str, sha: &str) -> bool {
    approved_shas(bodies, tag).iter().any(|s| s == sha)
}

/// SHAs approved by a comment whose first line is exactly `<tag>: <sha>` (I3), most recent
/// first, without repeats (AS-40).
pub fn approved_shas(bodies: &[String], tag: &str) -> Vec<String> {
    let prefix = format!("{tag}: ");
    let mut out: Vec<String> = Vec::new();
    for b in bodies.iter().rev() {
        let line = b.lines().next().map(str::trim_end).unwrap_or_default();
        if let Some(sha) = line.strip_prefix(prefix.as_str()).filter(|s| is_sha(s))
            && !out.iter().any(|s| s == sha)
        {
            out.push(sha.to_owned());
        }
    }
    out
}

/// SHA de commit complet, en hexadécimal minuscule.
pub fn is_sha(s: &str) -> bool {
    s.len() == 40 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    const SHA: &str = "e3e2f2a842612bebfba1a8388f86fd296f64344c";
    const SLUG: &str = "CreatorInstant548/time_switcher";

    fn run(suite: u64, conclusion: &str) -> Value {
        json!({"name": "ci-ok", "status": "completed", "conclusion": conclusion, "head_sha": SHA,
               "check_suite": {"id": suite}, "app": {"slug": "github-actions"}})
    }

    fn wfr(suite: u64, path: &str, wid: u64) -> Value {
        json!({"id": 1, "path": path, "head_sha": SHA, "event": "pull_request", "workflow_id": wid,
               "check_suite_id": suite, "repository": {"full_name": SLUG}})
    }

    /// Réponses enregistrées le 30/10 pour la PR #14 (champs utiles seulement).
    #[test]
    fn judges_ci_ok_by_origin() {
        let ci = |s| (s, vec![wfr(s, CI_WORKFLOW, 401414764)]);
        assert!(judge_ci(&[run(7, "success")], &[ci(7)], SHA, 401414764, SLUG).is_ok());
        // G3 : un ci-ok d'un autre workflow ne compte pas, et doit lui aussi être vert.
        let other = (8, vec![wfr(8, ".github/workflows/nightly.yml", 401467331)]);
        assert!(
            judge_ci(
                &[run(8, "success")],
                std::slice::from_ref(&other),
                SHA,
                401414764,
                SLUG
            )
            .is_err()
        );
        assert!(
            judge_ci(
                &[run(7, "success"), run(8, "failure")],
                &[ci(7), other.clone()],
                SHA,
                401414764,
                SLUG
            )
            .is_err()
        );
        assert!(
            judge_ci(
                &[run(7, "success"), run(8, "success")],
                &[ci(7), other],
                SHA,
                401414764,
                SLUG
            )
            .is_ok()
        );
        assert!(judge_ci(&[], &[], SHA, 401414764, SLUG).is_err());
        assert!(judge_ci(&[run(7, "skipped")], &[ci(7)], SHA, 401414764, SLUG).is_err());
        // Bon chemin, mais autre identifiant de workflow ou autre dépôt.
        assert!(
            judge_ci(
                &[run(7, "success")],
                &[(7, vec![wfr(7, CI_WORKFLOW, 1)])],
                SHA,
                401414764,
                SLUG
            )
            .is_err()
        );
        let mut fork = wfr(7, CI_WORKFLOW, 401414764);
        fork["repository"]["full_name"] = json!("evil/time_switcher");
        assert!(
            judge_ci(
                &[run(7, "success")],
                &[(7, vec![fork])],
                SHA,
                401414764,
                SLUG
            )
            .is_err()
        );
    }

    /// Plays the scripted outputs in order (the last one repeats); notes calls and pauses.
    struct Script {
        outs: Vec<Output>,
        log: Log,
    }

    /// Calls and pauses (seconds) seen by a [`Script`].
    type Log = std::rc::Rc<std::cell::RefCell<(Vec<String>, Vec<u64>)>>;

    impl Run for Script {
        fn run(&self, _dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
            let mut log = self.log.borrow_mut();
            log.0.push(format!("{prog} {}", args.join(" ")));
            Ok(self.outs[(log.0.len() - 1).min(self.outs.len() - 1)].clone())
        }

        fn pause(&self, d: Duration) {
            self.log.borrow_mut().1.push(d.as_secs());
        }
    }

    fn scripted(outs: Vec<Output>) -> (Repo, Log) {
        let log = Log::default();
        let repo = Repo {
            main: PathBuf::from("main"),
            here: PathBuf::from("main"),
            slug: "o/r".to_owned(),
            dry_run: false,
            ci: false,
            cache: std::env::temp_dir().join(format!("xtask-retry-{}", std::process::id())),
            runner: Box::new(Script {
                outs,
                log: std::rc::Rc::clone(&log),
            }),
            ci_workflow: OnceCell::new(),
        };
        (repo, log)
    }

    fn ok(s: &str) -> Output {
        (Some(0), s.as_bytes().to_vec(), String::new())
    }

    fn http(code: u16) -> Output {
        (
            Some(1),
            Vec::new(),
            format!("gh: HTTP {code}: Bad Gateway (https://api.github.com/x)\n"),
        )
    }

    /// V4: GET retried on 5xx and on an empty or truncated answer, at most 3 attempts with growing
    /// pauses, then a refusal; never on 4xx.
    #[test]
    fn retries_transient_reads_only() {
        for first in [http(502), http(500), ok(""), ok("{\"id\":"), ok("[1]\n[")] {
            let (repo, pauses) = scripted(vec![first, ok("{\"id\":9}\n")]);
            assert_eq!(repo.api("repos/o/r/x").unwrap()["id"], 9);
            assert_eq!(pauses.borrow().1, [2]);
            let (repo, _) = scripted(vec![http(503), ok("[{\"a\":1}]")]);
            assert_eq!(repo.api_all("repos/o/r/y").unwrap().len(), 1);
        }
        let (repo, pauses) = scripted(vec![http(502)]);
        let e = repo.api("repos/o/r/x").unwrap_err();
        assert!(e.contains("HTTP 502"), "{e}");
        assert_eq!(pauses.borrow().1, [2, 5]);
        assert_eq!(pauses.borrow().0.len(), 3);
        for code in [404, 403, 422, 501] {
            let (repo, pauses) = scripted(vec![http(code), ok("{}")]);
            assert!(repo.api_all("repos/o/r/x").is_err(), "{code}");
            assert!(pauses.borrow().1.is_empty(), "{code}");
            assert_eq!(pauses.borrow().0.len(), 1, "{code}");
        }
        // #55: an exhausted quota is named, and not retried.
        let limited = (
            Some(1),
            Vec::new(),
            "gh: API rate limit exceeded for installation ID 1. (HTTP 403)".to_owned(),
        );
        let (repo, pauses) = scripted(vec![limited, ok("{}")]);
        let e = repo.api("repos/o/r/x").unwrap_err();
        assert!(e.starts_with("GitHub API quota exhausted"), "{e}");
        assert!(pauses.borrow().1.is_empty());
        assert_eq!(http_status("gh: Not Found (HTTP 404)"), Some(404));
        assert_eq!(http_status("gh: HTTP 5023"), None);
        assert_eq!(http_status("connection reset"), None);
    }

    /// #55: G3 reads the `ci.yml` workflow id once per run, then two lists per commit (check runs,
    /// workflow runs of the SHA), and no workflow runs when there is no `ci-ok`.
    #[test]
    fn ci_ok_reads_the_workflow_id_once() {
        let wid = 401414764;
        let checks = ok(&json!({"check_runs": [run(7, "success")]}).to_string());
        let ours = |suite| {
            let mut w = wfr(suite, CI_WORKFLOW, wid);
            w["repository"]["full_name"] = json!("o/r");
            w
        };
        // Suite 8 is another run of the same SHA: it is filtered out by its check suite.
        let wfrs = json!({"workflow_runs": [ours(8), ours(7)]});
        let (repo, log) = scripted(vec![
            ok(&json!({"id": wid}).to_string()),
            checks.clone(),
            ok(&wfrs.to_string()),
            checks,
            ok(&wfrs.to_string()),
            ok(&json!({"check_runs": []}).to_string()),
        ]);
        assert_eq!(repo.ci_ok(SHA), Ok(None));
        assert_eq!(repo.ci_ok(SHA), Ok(None));
        let absent = repo.ci_ok(SHA).unwrap().unwrap_or_default();
        assert!(absent.contains("ci-ok absent"), "{absent}");
        let calls = log.borrow().0.clone();
        let runs = format!("gh api --paginate repos/o/r/actions/runs?head_sha={SHA}&per_page=100");
        assert_eq!(calls.len(), 6, "{calls:#?}");
        assert_eq!(calls[0], "gh api repos/o/r/actions/workflows/ci.yml");
        assert_eq!(
            calls.iter().filter(|c| **c == runs).count(),
            2,
            "{calls:#?}"
        );
        assert!(calls[1..].iter().all(|c| !c.contains("workflows/ci.yml")));
    }

    /// V4: a write is never retried, even on a transient status.
    #[test]
    fn never_retries_writes() {
        assert!(is_get(&["api", "--paginate", "repos/o/r/x"]));
        for w in [
            &["api", "-X", "POST", "repos/o/r/x"][..],
            &["api", "--method", "PUT", "repos/o/r/x"],
            &["api", "repos/o/r/x", "-f", "a=b"],
            &["api", "repos/o/r/x", "--input", "f"],
            &["pr", "merge", "7"],
        ] {
            assert!(!is_get(w), "{w:?}");
        }
        let (repo, pauses) = scripted(vec![ok("[]"), http(502), ok("u")]);
        assert!(repo.open_issue("k", DRIFT_TITLE, "b").is_err());
        assert!(pauses.borrow().1.is_empty());
        let calls = pauses.borrow().0.clone();
        let creates = calls
            .iter()
            .filter(|c| c.starts_with("gh issue create"))
            .count();
        assert_eq!(creates, 1, "{calls:?}");
        let _ = std::fs::remove_dir_all(&repo.cache); // temporary issue body of this test only
    }

    /// AS-45 P3: the same approval text counts only from the owner; another account, a bot, or
    /// another author association is ignored, in `comments` and in the anomaly issue lookup.
    #[test]
    fn keeps_only_the_owner_items() {
        let text = format!("review-ok: {SHA}");
        let owner = owner_item(&text);
        assert!(by_owner(&owner, "o/r"));
        let mut upper = owner.clone();
        upper["user"]["login"] = json!("O");
        assert!(by_owner(&upper, "o/r"));
        let mut others = Vec::new();
        for (path, value) in [
            ("/user/login", json!("someone")),
            ("/user/login", json!("o-bot")),
            ("/user/login", Value::Null),
            ("/user/type", json!("Bot")),
            ("/user/type", json!("Organization")),
            ("/user/type", Value::Null),
            ("/author_association", json!("MEMBER")),
            ("/author_association", json!("COLLABORATOR")),
            ("/author_association", json!("CONTRIBUTOR")),
            ("/author_association", json!("NONE")),
            ("/author_association", json!("owner")),
            ("/author_association", Value::Null),
        ] {
            let mut c = owner.clone();
            *c.pointer_mut(path).unwrap() = value.clone();
            assert!(!by_owner(&c, "o/r"), "{path} = {value}");
            others.push(c);
        }
        let mut no_user = owner.clone();
        no_user.as_object_mut().unwrap().remove("user");
        assert!(!by_owner(&no_user, "o/r"));
        assert!(!by_owner(&owner, "r") && !by_owner(&owner, "/r") && !by_owner(&owner, "x/r"));
        // Through Repo::comments: only the owner's comment is returned, before any other check.
        let (repo, log) = scripted(vec![ok(&json!(others).to_string())]);
        assert!(repo.comments(7).unwrap().is_empty());
        assert_eq!(
            log.borrow().0,
            ["gh api --paginate repos/o/r/issues/7/comments?per_page=100"]
        );
        let mut all = others.clone();
        all.push(owner.clone());
        let (repo, _) = scripted(vec![ok(&json!(all).to_string())]);
        assert_eq!(repo.comments(7).unwrap(), std::slice::from_ref(&text));
        assert!(approved(&repo.comments(7).unwrap(), "review-ok", SHA));
        // An issue carrying the anomaly key counts only from the owner, and never a PR.
        let mut pr = anomaly_issue(owner.clone(), DRIFT_TITLE, "k");
        pr["pull_request"] = json!({});
        let mut listed: Vec<Value> = others
            .iter()
            .cloned()
            .map(|v| anomaly_issue(v, DRIFT_TITLE, "k"))
            .collect();
        listed.push(pr);
        // The strangers' issues and the PR do not count: the issue is opened.
        assert_eq!(opens(&listed, "k", DRIFT_TITLE), Ok(true));
        // The owner's issue is reused: nothing is created.
        let mine = anomaly_issue(owner, DRIFT_TITLE, "k");
        assert_eq!(opens(&[mine], "k", DRIFT_TITLE), Ok(false));
    }

    /// An open issue of the owner, number 3, with `title` and the marker of `key` in its body.
    fn anomaly_issue(mut v: Value, title: &str, key: &str) -> Value {
        v["title"] = json!(title);
        v["body"] = json!(format!(
            "Constat.\n\n{}\n\nOuverte par `cargo xtask`.",
            key_marker(key)
        ));
        v["number"] = json!(3);
        v
    }

    /// `open_issue(key, title)` with `listed` as the open issues: `Ok(true)` if it creates an
    /// issue, `Ok(false)` if it reuses one.
    fn opens(listed: &[Value], key: &str, title: &str) -> Result<bool, String> {
        let (mut repo, log) = scripted(vec![ok(&json!(listed).to_string()), ok("u")]);
        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        repo.cache = std::env::temp_dir().join(format!("xtask-owner-{}-{n}", std::process::id()));
        let r = repo.open_issue(key, title, "b");
        let _ = std::fs::remove_dir_all(&repo.cache); // temporary issue body of this test only
        r?;
        let calls = log.borrow().0.clone();
        assert_eq!(
            calls[0],
            format!("gh api --paginate {}", issues_path("o/r"))
        );
        Ok(calls.iter().any(|c| c.starts_with("gh issue create")))
    }

    /// Review of #96 (AS-50 AU2): an open owner issue `role:architecture` that only quotes the key
    /// (a remontée, a note), under another title or without the key marker, never absorbs the
    /// anomaly: a new anomaly issue is opened, which stops `merge`. Every reused issue is an
    /// anomaly issue by [`is_anomaly`], the same definition as `merge` step [2].
    #[test]
    fn only_an_anomaly_issue_is_reused() {
        let sha = "1111111111111111111111111111111111111111";
        let title = format!("{ANOMALY_TITLE} 1111111");
        let owner = owner_item("x");
        let mut quoting = owner.clone();
        quoting["title"] = json!("Remontée : écart repo-settings-drift et commit 1111111");
        quoting["body"] = json!(format!("Voir {sha} et repo-settings-drift.\nClé : `{sha}`"));
        quoting["number"] = json!(5);
        for key in [sha, "repo-settings-drift"] {
            let t = if key == sha {
                title.as_str()
            } else {
                DRIFT_TITLE
            };
            assert!(!is_anomaly(&quoting, "o/r") && !tracks(&quoting, "o/r", key));
            assert_eq!(opens(std::slice::from_ref(&quoting), key, t), Ok(true));
        }
        // An anomaly title without the marker of this key (another commit): not reused either.
        let other = anomaly_issue(owner.clone(), &title, &"2".repeat(40));
        assert!(is_anomaly(&other, "o/r") && !tracks(&other, "o/r", sha));
        assert_eq!(opens(&[other], sha, &title), Ok(true));
        // The marker must be a whole line: a quoted marker inside a sentence does not count.
        let mut inline = anomaly_issue(owner.clone(), &title, sha);
        inline["body"] = json!(format!("Voir la {} plus haut.", key_marker(sha)));
        assert_eq!(opens(&[inline], sha, &title), Ok(true));
        // The anomaly issue of this very key is reused, and it is one that stops merge.
        let same = anomaly_issue(owner, &title, sha);
        assert!(is_anomaly(&same, "o/r") && tracks(&same, "o/r", sha));
        assert_eq!(opens(&[same], sha, &title), Ok(false));
        // Only anomaly issues are opened.
        assert!(opens(&[], "k", "Remontée").is_err());
    }

    /// The nightly issue as the API returns it: author `github-actions[bot]`, type `Bot`.
    fn nightly_issue() -> Value {
        json!({"title": NIGHTLY_ANOMALY_TITLE, "number": 9, "body": "Run: u",
               "user": {"login": "github-actions[bot]", "type": "Bot"},
               "author_association": "NONE"})
    }

    /// AS-52 (AU8): the nightly issue of `github-actions[bot]` with the exact title is an
    /// anomaly; another bot, another title, a user account or a pull request is not.
    #[test]
    fn counts_the_nightly_bot_issue() {
        let bot = nightly_issue();
        assert!(is_anomaly(&bot, "o/r"));
        for (path, value) in [
            ("/user/login", json!("dependabot[bot]")),
            ("/user/login", json!("github-actions")),
            ("/user/login", json!("GitHub-Actions[bot]")),
            ("/user/login", json!("o")),
            ("/user/login", Value::Null),
            ("/user/type", json!("User")),
            ("/user/type", Value::Null),
            ("/title", json!("Tâche périodique en échec")),
            ("/title", json!(format!("{NIGHTLY_ANOMALY_TITLE} "))),
            ("/title", json!(format!("Re: {NIGHTLY_ANOMALY_TITLE}"))),
            ("/title", json!(format!("{ANOMALY_TITLE} 1234567"))),
            ("/title", json!(DRIFT_TITLE)),
            ("/title", Value::Null),
        ] {
            let mut v = bot.clone();
            *v.pointer_mut(path).unwrap() = value.clone();
            assert!(!is_anomaly(&v, "o/r"), "{path} = {value}");
        }
        let mut pr = bot.clone();
        pr["pull_request"] = json!({});
        assert!(!is_anomaly(&pr, "o/r"));
        // The owner may not stand in for the bot on this title: P3 covers only xtask's titles.
        let mut owner = owner_item("x");
        owner["title"] = json!(NIGHTLY_ANOMALY_TITLE);
        assert!(!is_anomaly(&owner, "o/r"));
        // A stranger using the title is not counted either.
        let mut stranger = owner.clone();
        stranger["user"]["login"] = json!("someone");
        stranger["author_association"] = json!("NONE");
        assert!(!is_anomaly(&stranger, "o/r"));
    }

    /// AS-52: `open_issue` never reuses the nightly bot issue, even when its body carries the
    /// key marker: it only reuses an owner's issue that [`tracks`] the key.
    #[test]
    fn never_reuses_the_nightly_issue() {
        let mut bot = nightly_issue();
        bot["labels"] = json!([{"name": "role:architecture"}]);
        bot["body"] = json!(format!("Run: u\n{}", key_marker("k")));
        assert!(is_anomaly(&bot, "o/r") && !tracks(&bot, "o/r", "k"));
        assert_eq!(opens(&[bot], "k", DRIFT_TITLE), Ok(true));
    }

    /// AS-52: the `report` job of `nightly.yml` opens the issue under the very title that
    /// [`is_anomaly`] counts, and only after `verify-main` fails or is cancelled.
    #[test]
    fn nightly_writes_the_anomaly_title() {
        let yml = include_str!("../../../.github/workflows/nightly.yml");
        let line = format!("NIGHTLY_ANOMALY_TITLE: \"{NIGHTLY_ANOMALY_TITLE}\"");
        assert_eq!(
            yml.lines().filter(|l| l.trim() == line).count(),
            1,
            "{line}"
        );
        let gate = "if: always() && (needs.verify-main.result == 'failure' || \
                    needs.verify-main.result == 'cancelled')";
        assert_eq!(yml.lines().filter(|l| l.trim() == gate).count(), 1);
        assert!(!NIGHTLY_ANOMALY_TITLE.contains(['"', '\\', '$', '`']));
    }

    #[test]
    fn reads_paginated_output() {
        let v = pages("[{\"a\":1},{\"a\":2}]\n[{\"a\":3}]{\"check_runs\":[]}").unwrap();
        assert_eq!(v.len(), 4);
        assert!(pages("[{\"a\":1}").is_err());
        assert!(pages("").unwrap().is_empty());
    }

    #[test]
    fn reads_approvals_and_slugs() {
        let b = vec![
            format!("review-ok: {SHA}  \r\nRelu, rien de bloquant.\n"),
            format!("archsec-ok: {}", &SHA[..39]),
            // I3 : la ligne citée dans un avis défavorable ne compte pas.
            format!("Changements demandés. Après correction :\narchsec-ok: {SHA}"),
        ];
        assert!(approved(&b, "review-ok", SHA));
        assert!(!approved(&b, "archsec-ok", SHA));
        for bad in [
            format!("pas review-ok: {SHA}"),
            format!(" review-ok: {SHA}"),
            format!("review-ok:  {SHA}"),
            format!("review-ok: {}", SHA.to_uppercase()),
            format!("Relu.\nreview-ok: {SHA}"),
        ] {
            assert!(
                !approved(std::slice::from_ref(&bad), "review-ok", SHA),
                "{bad}"
            );
        }
        assert_eq!(
            slug_of("https://github.com/CreatorInstant548/time_switcher.git").as_deref(),
            Some(SLUG)
        );
        assert_eq!(slug_of("git@github.com:o/r").as_deref(), Some("o/r"));
        assert!(slug_of("https://evil.example/o/r.git").is_none());
        assert!(slug_of("https://github.com/o/r/x").is_none());
        assert!(is_sha(SHA) && !is_sha(&SHA.to_uppercase()) && !is_sha(&SHA[1..]));
    }
}
