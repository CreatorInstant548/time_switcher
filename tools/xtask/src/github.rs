//! Accès à Git et à GitHub pour `merge`, `verify-main` et `hooks`. Tout est en lecture, sauf
//! [`Repo::open_issue`] ; la fusion elle-même reste dans `merge`. Un appel qui échoue ou une
//! réponse illisible est une erreur, jamais une réussite.
//! spec: carnet architecture-securite.md SEC0-15, SEC0-17 ; §9.6 G3 ; §9.8 J3, J5

use std::path::{Path, PathBuf};

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
    /// `TS_CACHE` et dossier personnel (réglages de Claude Code), remplaçables en test.
    pub cache: PathBuf,
    pub home: Option<PathBuf>,
    pub runner: Box<dyn Run>,
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

impl Repo {
    /// `github` faux : distant hors de GitHub permis (`hooks`, essais dans un dépôt jetable) ; le
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
            home: sys::home(),
            runner: Box::new(System),
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
        pages(&self.gh(&["api", "--paginate", path])?)
    }

    pub fn api(&self, path: &str) -> Result<Value, String> {
        serde_json::from_str(&self.gh(&["api", path])?)
            .map_err(|e| format!("réponse de GitHub illisible pour {path} : {e}"))
    }

    /// Corps de tous les commentaires d'une PR.
    pub fn comments(&self, pr: u64) -> Result<Vec<String>, String> {
        let all = self.api_all(&format!(
            "repos/{}/issues/{pr}/comments?per_page=100",
            self.slug
        ))?;
        all.iter()
            .map(|c| {
                c["body"]
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| "commentaire sans corps".to_owned())
            })
            .collect()
    }

    /// `ci-ok` sur `sha` selon G3 : `Ok(None)` si vert, `Ok(Some(motif))` sinon ; `Err` si GitHub
    /// ne répond pas.
    pub fn ci_ok(&self, sha: &str) -> Result<Option<String>, String> {
        let s = &self.slug;
        let wf = self.api(&format!("repos/{s}/actions/workflows/ci.yml"))?;
        let wid = wf["id"]
            .as_u64()
            .ok_or("identifiant du workflow ci.yml illisible")?;
        let pages = self.api_all(&format!(
            "repos/{s}/commits/{sha}/check-runs?check_name=ci-ok&per_page=100"
        ))?;
        let mut runs = Vec::new();
        for p in &pages {
            runs.extend(
                p["check_runs"]
                    .as_array()
                    .ok_or("liste des contrôles illisible")?
                    .iter()
                    .cloned(),
            );
        }
        let mut suites = Vec::new();
        for r in &runs {
            let id = r["check_suite"]["id"]
                .as_u64()
                .ok_or("check_suite illisible")?;
            let w = self.api(&format!(
                "repos/{s}/actions/runs?check_suite_id={id}&per_page=100"
            ))?;
            suites.push((
                id,
                w["workflow_runs"].as_array().cloned().unwrap_or_default(),
            ));
        }
        Ok(judge_ci(&runs, &suites, sha, wid, s).err())
    }

    /// Ouvre une *issue* `role:architecture`, ou réutilise celle, ouverte, qui porte déjà `key`
    /// (J5). Avec `--dry-run`, affiche seulement celle qu'il ouvrirait.
    pub fn open_issue(&self, key: &str, title: &str, body: &str) -> Result<(), String> {
        if self.slug.is_empty() {
            return Err(format!(
                "distant hors de GitHub : issue « {title} » impossible"
            ));
        }
        let list = self.gh(&[
            "issue",
            "list",
            "-R",
            &self.slug,
            "--state",
            "open",
            "--label",
            ISSUE_LABEL,
            "--limit",
            "500",
            "--json",
            "number,title,body",
        ])?;
        for i in pages(&list)? {
            let text = format!(
                "{} {}",
                i["title"].as_str().unwrap_or(""),
                i["body"].as_str().unwrap_or("")
            );
            if text.contains(key) {
                println!(
                    "  issue #{} déjà ouverte pour {key} : réutilisée",
                    i["number"]
                );
                return Ok(());
            }
        }
        let body =
            format!("{body}\n\nClé : `{key}`\n\nOuverte par `cargo xtask` (SEC0-17, SEC0-35).");
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

/// Un commentaire dont la **première ligne** est exactement `<tag>: <sha>` (I3) : une ligne citée
/// plus bas, dans un avis défavorable par exemple, ne vaut pas approbation.
pub fn approved(bodies: &[String], tag: &str, sha: &str) -> bool {
    let want = format!("{tag}: {sha}");
    bodies
        .iter()
        .any(|b| b.lines().next().is_some_and(|l| l.trim_end() == want))
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
