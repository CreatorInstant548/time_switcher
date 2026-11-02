//! `cargo xtask merge <PR> [--dry-run]` : **seul chemin de fusion** (SEC0-15 révisée). Lancé
//! depuis la copie principale. Les contrôles se font dans l'ordre, et le premier qui échoue
//! arrête tout : en cas de doute, refus. Since AS-50 (#55), merge only makes the checks of the PR
//! itself, in a few calls: no open anomaly issue of `verify-main` (AU2), PR state (no fork, base
//! `main`, up to date, title), `ci-ok` (G3), files and sensitive paths, workflows when the PR
//! changes `.github/`, approvals (with the AS-40 carry-over). The history (`verify-main`), the
//! GitHub settings (`-Check`) and the hooks are no longer checked here.
//! `--dry-run` fait tous les contrôles, sans fusion ni *issue*. Il n'appelle jamais
//! `gh pr merge` ni `git pull`, et ne touche ni l'arbre, ni l'index, ni les branches de la
//! copie principale (test `dry_run_never_merges`). Les deux `git fetch` (`origin` et
//! `pull/<n>/head`) écrivent en revanche dans `.git` : références distantes, `FETCH_HEAD`, objets.
//! Hors de la copie principale, son résultat est au mieux « OK sauf copie principale », avec le
//! code 3 (m1).
//! spec: carnet architecture-securite.md SEC0-15, SEC0-16 ; §9.6 G3, G8 ; §9.8 J1, J8
//! Voir aussi l'issue #6, conditions T3-a et T3-b.

use std::collections::BTreeMap;
use std::process::ExitCode;

use serde_json::Value;

use crate::carry::{Carry, Pr};
use crate::github::{Repo, is_anomaly, is_sha, same_path};
use crate::{sensitive, workflows};

const TYPES: [&str; 9] = [
    "feat", "fix", "refactor", "perf", "test", "docs", "build", "ci", "chore",
];

/// Titre conventionnel `type(portée): description` (A31), sur une seule ligne.
pub fn title_ok(t: &str) -> bool {
    let Some((ty, rest)) = t.split_once('(') else {
        return false;
    };
    let Some((scope, rest)) = rest.split_once(')') else {
        return false;
    };
    let rest = rest.strip_prefix('!').unwrap_or(rest);
    let scope_ok = !scope.is_empty()
        && scope
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_./-".contains(&b));
    let desc = rest.strip_prefix(": ").unwrap_or_default();
    TYPES.contains(&ty)
        && scope_ok
        && !desc.is_empty()
        && !desc.starts_with(' ')
        && !t.contains(['\n', '\r'])
}

/// Chemins de la réponse `pulls/<n>/files` : les deux d'un renommage ; et le nombre de fichiers.
pub fn pr_files(items: &[Value]) -> Result<(usize, Vec<String>), String> {
    let mut paths = Vec::new();
    for f in items {
        paths.push(
            f["filename"]
                .as_str()
                .ok_or("fichier sans nom dans la réponse")?
                .to_owned(),
        );
        if let Some(prev) = f["previous_filename"].as_str() {
            paths.push(prev.to_owned());
        }
    }
    paths.sort();
    paths.dedup();
    Ok((items.len(), paths))
}

/// Refus, avec la commande qui débloque.
fn refuse(why: impl std::fmt::Display, fix: &str) -> String {
    format!("{why}\n  Pour débloquer : {fix}")
}

fn step(n: &str, what: &str) {
    println!("\n[{n}] {what}");
}

/// Étape 1 : copie principale propre, sur `main`, égale à `origin/main` dès le lancement.
/// Rend faux en `--dry-run` lancé hors de la copie principale (m1).
///
/// AS-50 retires J2 (binary fingerprint). What remains of it costs no process: `cargo xtask`
/// rebuilds the binary from the clean main copy before it runs, so a copy equal to `origin/main`
/// at launch means a binary built from `main`. A copy behind it is brought up to date by
/// `git pull --ff-only`, then the merge is refused: the next launch rebuilds `xtask` from the new
/// `main` instead of checking with the code of the old one.
fn main_copy(repo: &Repo) -> Result<bool, String> {
    if repo.git(&["branch", "--show-current"])? != "main" {
        return Err(refuse(
            "la copie principale n'est pas sur main",
            "git switch main dans la copie principale",
        ));
    }
    if !repo.git(&["status", "--porcelain"])?.is_empty() {
        return Err(refuse(
            "la copie principale n'est pas propre",
            "git status dans la copie principale",
        ));
    }
    repo.git(&["fetch", "--quiet", "origin"])?;
    let revs = repo.git(&["rev-parse", "HEAD", "origin/main"])?;
    let behind = match revs.lines().collect::<Vec<_>>()[..] {
        [head, main] => head != main,
        _ => return Err(format!("git rev-parse illisible : {revs}")),
    };
    if behind && repo.dry_run {
        // La vraie fusion fait git pull --ff-only ; les contrôles suivants lisent origin/main.
        println!(
            "  AVERTISSEMENT (--dry-run) : copie principale en retard d'origin/main ; la vraie \
             fusion la met à jour, puis demande de relancer."
        );
    } else if behind {
        repo.git(&["pull", "--ff-only", "--quiet"])?;
        return Err(refuse(
            "la copie principale était en retard d'origin/main : elle vient d'être mise à jour \
             par git pull --ff-only, et ce binaire a été construit depuis l'ancien main",
            "relancer cargo xtask merge <PR> : cargo reconstruit xtask depuis le nouveau main",
        ));
    }
    if same_path(&repo.here, &repo.main) {
        return Ok(true);
    }
    if repo.dry_run {
        println!(
            "  AVERTISSEMENT (--dry-run) : lancé hors de la copie principale ({}).",
            repo.here.display()
        );
        return Ok(false);
    }
    Err(refuse(
        format!(
            "lancé depuis {}, pas depuis la copie principale",
            repo.here.display()
        ),
        "cd vers la copie principale, puis cargo xtask merge <PR>",
    ))
}

/// AS-50 AU2 and AS-52 (AU8): the open anomaly issues, those of `verify-main` written by the
/// owner (P3, the same filter as the approvals) and the nightly one of `github-actions[bot]`, by
/// one paginated read of all the open issues, whatever their author and labels. [`is_anomaly`]
/// is applied here; a pull request is not an issue. Their numbers. A failed read is a refusal.
fn open_anomalies(repo: &Repo) -> Result<Vec<u64>, String> {
    let list = repo.api_all(&format!(
        "repos/{}/issues?state=open&per_page=100",
        repo.slug
    ))?;
    // The same definition as the reuse in `Repo::open_issue`: an anomaly that `verify-main`
    // reported is always an issue counted here, and so is the nightly one (AS-52).
    Ok(list
        .iter()
        .filter(|i| is_anomaly(i, &repo.slug))
        .map(|i| i["number"].as_u64().unwrap_or_default())
        .collect())
}

/// PR ouverte, venue du dépôt lui-même (pas d'un *fork*), base `main`, hors brouillon,
/// fusionnable, en retard de 0 commit, titre A31.
fn pr_state(repo: &Repo, pr: u64) -> Result<Value, String> {
    let v = repo.gh(&[
        "pr",
        "view",
        &pr.to_string(),
        "-R",
        &repo.slug,
        "--json",
        "number,state,baseRefName,headRefOid,isDraft,isCrossRepository,mergeable,changedFiles,\
         title,url",
    ])?;
    let p: Value = serde_json::from_str(&v).map_err(|e| format!("gh pr view illisible : {e}"))?;
    let sha = p["headRefOid"]
        .as_str()
        .filter(|s| is_sha(s))
        .ok_or("SHA de tête illisible")?;
    let fix_pr = format!("gh pr view {pr}");
    if p["state"] != "OPEN" {
        return Err(refuse(format!("PR #{pr} à l'état {}", p["state"]), &fix_pr));
    }
    // A fork's PR is never merged: its branch is outside the repository (AS-50).
    if p["isCrossRepository"] != false {
        return Err(refuse(
            format!(
                "PR #{pr} venue d'un fork (isCrossRepository: {})",
                p["isCrossRepository"]
            ),
            "reprendre le changement sur une branche du dépôt",
        ));
    }
    if p["baseRefName"] != "main" {
        return Err(refuse(
            format!("base {}, attendu main", p["baseRefName"]),
            "rouvrir la PR vers main",
        ));
    }
    if p["isDraft"] != false {
        return Err(refuse("PR en brouillon", &format!("gh pr ready {pr}")));
    }
    if p["mergeable"] != "MERGEABLE" {
        return Err(refuse(
            format!("PR non fusionnable ({})", p["mergeable"]),
            "résoudre les conflits, ou réessayer si GitHub calcule encore (UNKNOWN)",
        ));
    }
    let cmp = repo.api(&format!("repos/{}/compare/main...{sha}", repo.slug))?;
    if cmp["behind_by"].as_u64() != Some(0) {
        return Err(refuse(
            format!(
                "branche en retard de {} commit(s) sur main",
                cmp["behind_by"]
            ),
            "mettre la branche à jour depuis origin/main et pousser",
        ));
    }
    if !p["title"].as_str().is_some_and(title_ok) {
        return Err(refuse(
            format!("titre non conforme (A31) : {}", p["title"]),
            &format!("gh pr edit {pr} --title \"type(portée): description\""),
        ));
    }
    Ok(p)
}

/// J1 : workflows de la tête, lus en mémoire et contrôlés avec le code de `main` ; puis J8 : chaque
/// action épinglée correspond à l'étiquette de son commentaire dans le dépôt officiel.
fn workflows_at(repo: &Repo, sha: &str) -> Result<(), String> {
    let ls = repo.git(&[
        "ls-tree",
        "-r",
        "-z",
        "--full-tree",
        sha,
        ".github/workflows/",
    ])?;
    let mut errors = Vec::new();
    let mut pins = BTreeMap::new();
    for entry in ls.split('\0').filter(|e| !e.is_empty()) {
        let (meta, path) = entry
            .split_once('\t')
            .ok_or("sortie de git ls-tree illisible")?;
        if !(path.ends_with(".yml") || path.ends_with(".yaml")) || !meta.starts_with("100644 blob ")
        {
            errors.push(format!(
                "{path} : entrée inattendue dans .github/workflows/ ({meta})"
            ));
            continue;
        }
        let text = String::from_utf8(
            repo.blob(&repo.main, sha, path)?
                .ok_or("workflow disparu")?,
        )
        .map_err(|_| format!("{path} n'est pas en UTF-8"))?;
        errors.extend(
            workflows::check_workflow(&text)
                .into_iter()
                .map(|e| format!("{path} : {e}")),
        );
        for p in workflows::pinned_uses(&text) {
            match p.tag {
                None => errors.push(format!(
                    "{path}:{} : uses: sans étiquette sûre en commentaire (J8)",
                    p.line
                )),
                Some(tag) => {
                    pins.entry((p.repo.to_ascii_lowercase(), tag))
                        .or_insert_with(Vec::new)
                        .push((path.to_owned(), p.line, p.sha));
                }
            }
        }
    }
    if errors.is_empty() {
        for ((action, tag), uses) in &pins {
            // `refs/tags/` : l'étiquette est résolue comme étiquette du dépôt officiel, jamais
            // comme branche ni comme SHA (un SHA de *fork* se résoudrait en lui-même).
            if tag.len() >= 7 && tag.bytes().all(|b| b.is_ascii_hexdigit()) {
                errors.push(format!(
                    "{action}@{tag} : l'étiquette ressemble à un SHA (J8)"
                ));
                continue;
            }
            let c = repo
                .api(&format!("repos/{action}/commits/refs/tags/{tag}"))
                .map_err(|e| {
                    format!(
                        "{action}@{tag} : étiquette introuvable ou API muette ({e}) : refus (J8)"
                    )
                })?;
            let want = c["sha"].as_str().unwrap_or_default();
            for (path, line, sha) in uses {
                if !is_sha(want) || sha != want {
                    errors.push(format!(
                        "{path}:{line} : {action}@{sha} ne correspond pas à {tag} ({want}) (T3-b)"
                    ));
                }
            }
        }
    }
    if errors.is_empty() {
        println!("  OK ({} action(s) épinglée(s) vérifiée(s))", pins.len());
        return Ok(());
    }
    errors.iter().for_each(|e| println!("  - {e}"));
    Err(refuse(
        format!("{} écart(s) dans les workflows de la tête", errors.len()),
        "corriger .github/workflows/ dans la PR",
    ))
}

/// Vrai si tout est conforme ; faux en `--dry-run` lancé hors de la copie principale (m1).
fn run(repo: &Repo, pr: u64) -> Result<bool, String> {
    // A merge is made from the main copy, never by a CI job.
    if repo.ci {
        return Err(refuse(
            "CI et GITHUB_ACTIONS valent true : mode CI",
            "lancer cargo xtask merge <PR> depuis la copie principale, hors CI",
        ));
    }
    step("1", "copie principale propre, sur main, à jour");
    let main_ok = main_copy(repo)?;
    step(
        "2",
        "no open anomaly issue: verify-main by the owner, or the nightly run (AS-50 AU2, AS-52)",
    );
    let open = open_anomalies(repo)?;
    let mut suspended = None;
    if open.is_empty() {
        println!("  none");
    } else {
        let list: Vec<String> = open.iter().map(|n| format!("#{n}")).collect();
        let why = refuse(
            format!(
                "merges suspended: open anomaly issue(s): {}",
                list.join(", ")
            ),
            "clear the anomaly (J5, -Apply for the settings; for the nightly issue, read its run \
             and run cargo xtask verify-main locally), then close the issue (docs/dev/ci.md §4)",
        );
        if !repo.dry_run {
            return Err(why);
        }
        // --dry-run shows every other check; its result stays this refusal.
        println!(
            "  REFUSED: {why}\n  --dry-run goes on with the other checks; the result stays a refusal."
        );
        suspended = Some(why);
    }
    step(
        "3-4",
        "PR ouverte, hors fork, base main, fusionnable, à jour ; titre (A31)",
    );
    let p = pr_state(repo, pr)?;
    let sha = p["headRefOid"].as_str().unwrap_or_default().to_owned();
    println!(
        "  {} : {} ({sha})",
        p["url"].as_str().unwrap_or_default(),
        p["title"].as_str().unwrap_or_default()
    );
    step("5", "ci-ok vert sur la tête, venu de ci.yml (G3)");
    if let Some(why) = repo.ci_ok(&sha)? {
        return Err(refuse(
            why,
            &format!("gh pr checks {pr}, puis corriger ou relancer la CI"),
        ));
    }
    step("6", "liste complète des fichiers (API paginée, renommages)");
    let items = repo.api_all(&format!(
        "repos/{}/pulls/{pr}/files?per_page=100",
        repo.slug
    ))?;
    let (count, mut paths) = pr_files(&items)?;
    if Some(count as u64) != p["changedFiles"].as_u64() {
        return Err(refuse(
            format!(
                "liste des fichiers incomplète ({count} lus, {} annoncés)",
                p["changedFiles"]
            ),
            "réessayer ; au-delà de 3 000 fichiers, découper la PR",
        ));
    }
    repo.git(&["fetch", "--quiet", "origin", &format!("pull/{pr}/head")])?;
    if !repo.ok(
        &repo.main,
        "git",
        &["cat-file", "-e", &format!("{sha}^{{commit}}")],
    )? {
        return Err(refuse(
            format!("tête {sha} introuvable après git fetch"),
            "réessayer",
        ));
    }
    // B1 : sous Windows, deux chemins égaux à la casse près s'écrasent au git pull.
    let rules = sensitive::rules(repo)?;
    let tree = repo.git(&["ls-tree", "-r", "-z", "--name-only", "--full-tree", &sha])?;
    let tree: Vec<String> = tree
        .split('\0')
        .filter(|f| !f.is_empty())
        .map(str::to_owned)
        .collect();
    let case = rules.tree_errors(&tree);
    if !case.is_empty() {
        case.iter().for_each(|e| println!("  - {e}"));
        return Err(refuse(
            "arbre de tête refusé : casse ou chemin non ASCII (B1)",
            "renommer les chemins dans la PR",
        ));
    }
    let base = repo.git(&["merge-base", "origin/main", &sha])?;
    let local = repo.git(&["diff", "--name-only", "-z", "--no-renames", &base, &sha])?;
    paths.extend(
        local
            .split('\0')
            .filter(|f| !f.is_empty())
            .map(str::to_owned),
    );
    paths.sort();
    paths.dedup();
    println!("  {count} fichier(s)");
    step("7", "chemins sensibles (liste d'origin/main, G8)");
    let hits = sensitive::hits(repo, &rules, &base, &sha, &paths)?;
    println!(
        "  {}",
        if hits.is_empty() {
            "aucun".to_owned()
        } else {
            hits.join(", ")
        }
    );
    step(
        "8-9",
        "workflows de la tête rejoués avec le code de main (J1) ; actions épinglées (J8)",
    );
    // AS-50: the workflows of main were checked when they were merged; only a PR that changes
    // `.github/` (compared without case) needs them checked again.
    if paths
        .iter()
        .any(|p| p.to_ascii_lowercase().starts_with(".github/"))
    {
        workflows_at(repo, &sha)?;
    } else {
        println!("  sans objet : la PR ne modifie pas .github/");
    }
    step(
        "10",
        "approbations sur le SHA de tête, ou reportées d'un SHA antérieur (chemin sensible, AS-40)",
    );
    if hits.is_empty() {
        println!("  sans objet");
    } else {
        let bodies = repo.comments(pr)?;
        let main_sha = repo.git(&["rev-parse", "origin/main"])?;
        let mut carry = Carry::new(
            repo,
            Pr {
                number: pr,
                head: &sha,
                main: &main_sha,
                diff: (&base, &sha, &paths),
                // Fetched above (pull/<n>/head): trees read locally, not by the API (m2).
                local_objects: true,
            },
        );
        for tag in ["review-ok", "archsec-ok"] {
            // W6: the same function as verify-main; W7: ci-ok was checked on the head (step 5).
            match carry.resolve(tag, &bodies, &|_| true)? {
                Ok(from) if from == sha => println!("  {tag}: {sha}"),
                Ok(from) => println!("  {tag} {}", carry.describe(&from)),
                Err(why) => {
                    return Err(refuse(
                        format!("sensitive path: {why}"),
                        "fresh review, then the architecture lead's approval on this SHA",
                    ));
                }
            }
        }
    }
    step("11", "fusion");
    if let Some(why) = suspended {
        return Err(why);
    }
    // m5: state read again right before gh pr merge (head SHA, mergeable, behind_by = 0).
    let again = pr_state(repo, pr)?;
    if again["headRefOid"].as_str() != Some(sha.as_str()) {
        return Err(refuse(
            "the PR head changed during the checks",
            &format!("run cargo xtask merge {pr} again"),
        ));
    }
    if repo.dry_run {
        println!("  --dry-run : tous les contrôles sont passés ; aucune fusion (PR #{pr}, {sha}).");
        return Ok(main_ok);
    }
    let n = pr.to_string();
    repo.gh(&[
        "pr",
        "merge",
        &n,
        "-R",
        &repo.slug,
        "--squash",
        "--delete-branch",
        "--match-head-commit",
        &sha,
    ])
    .or_else(|e| {
        // gh sort en erreur si la fusion distante a réussi mais pas la suppression de la branche
        // locale (tenue par un worktree) : l'état de la PR fait foi.
        let view = repo.gh(&[
            "pr",
            "view",
            &n,
            "-R",
            &repo.slug,
            "--json",
            "state,headRefOid,mergeCommit",
        ]);
        let v: Value = view
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_default();
        // m4: the fallback only accepts a merge of the checked head.
        match (v["state"].as_str(), v["headRefOid"].as_str()) {
            (Some("MERGED"), Some(h)) if h == sha => {
                println!(
                    "  AVERTISSEMENT : gh pr merge a signalé une erreur après la fusion ({e})."
                );
                Ok(String::new())
            }
            _ => Err(refuse(
                format!("gh pr merge failed ({e})"),
                &format!("gh pr view {pr}"),
            )),
        }
    })?;
    // m4: the PR is merged on GitHub; a failed local update is not a refusal.
    if let Err(e) = repo.git(&["pull", "--ff-only", "--quiet"]) {
        println!("  PR #{pr} merged ({sha}); local update failed: run git pull --ff-only ({e})");
        return Ok(true);
    }
    println!(
        "  PR #{pr} merged ({sha}); main copy: {}",
        repo.git(&["log", "--oneline", "-1"])?
    );
    Ok(true)
}

pub fn main(args: &[String]) -> ExitCode {
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let rest: Vec<&String> = args.iter().filter(|a| *a != "--dry-run").collect();
    let pr = match rest[..] {
        [n] => n.parse::<u64>().ok().filter(|n| *n > 0),
        _ => None,
    };
    let Some(pr) = pr else {
        eprintln!("usage : cargo xtask merge <PR> [--dry-run]");
        return ExitCode::from(2);
    };
    match Repo::open(dry_run, true).and_then(|repo| run(&repo, pr)) {
        Ok(true) => {
            println!("\nRésultat : OK");
            ExitCode::SUCCESS
        }
        Ok(false) => {
            println!(
                "\nRésultat : OK sauf copie principale (--dry-run lancé hors de la copie principale)"
            );
            ExitCode::from(3)
        }
        Err(e) => {
            println!("\nRésultat : REFUS : {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::Path;
    use std::rc::Rc;

    use serde_json::json;

    use crate::github::{ANOMALY_TITLES, NIGHTLY_ANOMALY_TITLE, Output, Run, owner_item};

    use super::*;

    #[test]
    fn titles() {
        for ok in [
            "feat(xtask): cargo xtask merge",
            "fix(ts_net)!: x",
            "docs(docs/dev): é",
        ] {
            assert!(title_ok(ok), "{ok}");
        }
        for bad in [
            "feat: x",
            "feature(x): y",
            "feat(): y",
            "feat(x):y",
            "feat(x):  y",
            "feat(x): y\nz",
            "feat(a b): y",
        ] {
            assert!(!title_ok(bad), "{bad}");
        }
    }

    /// Réponse `pulls/<n>/files` enregistrée (champs utiles) : un renommage compte ses deux chemins.
    #[test]
    fn files_with_renames() {
        let items = [
            json!({"filename": "docs/x.ps1", "previous_filename": "tools/dev/guard-git.ps1", "status": "renamed"}),
            json!({"filename": "README.md", "status": "modified"}),
        ];
        let (n, paths) = pr_files(&items).unwrap();
        assert_eq!(n, 2);
        assert_eq!(
            paths,
            ["README.md", "docs/x.ps1", "tools/dev/guard-git.ps1"]
        );
        assert!(pr_files(&[json!({"status": "added"})]).is_err());
    }

    /// Lanceur enregistreur : réponses par préfixe de commande (le plus long gagne), appels notés.
    struct Fake {
        calls: Rc<RefCell<Vec<String>>>,
        answers: Vec<(String, String)>,
    }

    impl Run for Fake {
        fn run(&self, _dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
            let line = format!("{prog} {}", args.join(" "));
            self.calls.borrow_mut().push(line.clone());
            let best = self
                .answers
                .iter()
                .filter(|(k, _)| line.starts_with(k.as_str()))
                .max_by_key(|(k, _)| k.len());
            match best {
                // A failed command: exit code 1, the rest of the answer on stderr.
                Some((_, out)) if out.starts_with(FAILS) => {
                    Ok((Some(1), Vec::new(), out[FAILS.len()..].to_owned()))
                }
                Some((_, out)) => Ok((Some(0), out.clone().into_bytes(), String::new())),
                None => Err(format!("commande inattendue : {line}")),
            }
        }
    }

    /// Prefix of a [`Fake`] answer that makes the command fail.
    const FAILS: &str = "exit 1: ";

    const SHA: &str = "c41e984ba8594dedd592744fcb69a70f3716cd87";
    const MAIN: &str = "43b3509aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    /// Commands that write to the main copy or to GitHub.
    const WRITES: [&str; 18] = [
        "gh pr merge",
        "gh issue create",
        "gh pr edit",
        "git pull",
        "git config",
        "git checkout",
        "git switch",
        "git reset",
        "git commit",
        "git push",
        "git update-ref",
        "git merge ",
        "git branch -f",
        "git branch -d",
        "git branch -D",
        "git branch -m",
        "git worktree add",
        "git stash",
    ];

    /// Plays `merge` on recorded answers for a sensitive-free PR #7 whose every check passes.
    fn scenario(dry_run: bool) -> (Result<bool, String>, Vec<String>) {
        scenario_with(dry_run, Vec::new())
    }

    /// [`scenario`], where `extra` answers replace those of the same prefix.
    fn scenario_with(
        dry_run: bool,
        extra: Vec<(String, String)>,
    ) -> (Result<bool, String>, Vec<String>) {
        scenario_in(dry_run, true, extra)
    }

    /// [`scenario_with`], run from the main copy if `in_main`, else from another folder.
    fn scenario_in(
        dry_run: bool,
        in_main: bool,
        extra: Vec<(String, String)>,
    ) -> (Result<bool, String>, Vec<String>) {
        // One folder per call: tests run in parallel.
        static RUN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = RUN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let base = std::env::temp_dir().join(format!(
            "xtask-merge-test-{}-{dry_run}-{n}",
            std::process::id()
        ));
        let (main, other) = (base.join("main"), base.join("other"));
        for d in [&main, &other] {
            std::fs::create_dir_all(d).unwrap();
        }
        let pr = json!({"number": 7, "state": "OPEN", "baseRefName": "main", "headRefOid": SHA,
            "isDraft": false, "isCrossRepository": false, "mergeable": "MERGEABLE",
            "changedFiles": 1, "title": "docs(test): x", "url": "u"});
        let check = json!({"name": "ci-ok", "status": "completed", "conclusion": "success",
            "head_sha": SHA, "check_suite": {"id": 5}});
        let wfr = json!({"workflow_runs": [{"path": ".github/workflows/ci.yml", "workflow_id": 9,
            "head_sha": SHA, "check_suite_id": 5, "repository": {"full_name": "o/r"}}]});
        let answers: Vec<(String, String)> = [
            ("git branch --show-current", "main".to_owned()),
            ("git status --porcelain", String::new()),
            ("git fetch", String::new()),
            ("git pull", String::new()),
            ("git rev-parse", MAIN.to_owned()),
            ("git rev-parse HEAD origin/main", format!("{MAIN}\n{MAIN}")),
            ("git ls-tree", String::new()),
            ("git log --oneline -1", "x".to_owned()),
            ("git cat-file -e", String::new()),
            (
                "git cat-file blob origin/main:tools/ci/sensitive-paths.txt",
                include_str!("../../ci/sensitive-paths.txt").to_owned(),
            ),
            (
                "gh api --paginate repos/o/r/issues?state=open",
                "[]".to_owned(),
            ),
            ("git merge-base", MAIN.to_owned()),
            ("git diff --name-only", "docs/a.md\0".to_owned()),
            (
                &format!("git ls-tree -r -z --name-only --full-tree {SHA}") as &str,
                "docs/a.md\0".to_owned(),
            ),
            ("gh pr view 7", pr.to_string()),
            ("gh pr merge 7", String::new()),
            (
                "gh api repos/o/r/compare/",
                json!({"behind_by": 0}).to_string(),
            ),
            (
                "gh api repos/o/r/actions/workflows/ci.yml",
                json!({"id": 9}).to_string(),
            ),
            (
                "gh api --paginate repos/o/r/commits/",
                json!({"check_runs": [check]}).to_string(),
            ),
            ("gh api repos/o/r/actions/runs", wfr.to_string()),
            (
                "gh api --paginate repos/o/r/pulls/7/files",
                json!([{"filename": "docs/a.md"}]).to_string(),
            ),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v))
        .chain(extra) // `max_by_key` keeps the last of equal prefixes
        .collect();
        let calls = Rc::new(RefCell::new(Vec::new()));
        let repo = Repo {
            main: main.clone(),
            here: if in_main { main } else { other },
            slug: "o/r".to_owned(),
            dry_run,
            ci: false,
            cache: base.join("cache"),
            runner: Box::new(Fake {
                calls: Rc::clone(&calls),
                answers,
            }),
        };
        let result = run(&repo, 7);
        let _ = std::fs::remove_dir_all(&base); // temporary files written above only
        let calls = calls.borrow().clone();
        (result, calls)
    }

    /// B4, AS-23 point 2: a `--dry-run` whose every check passes never calls `gh pr merge`, and
    /// runs no command that writes to the main copy or to GitHub. Only the `git fetch` write, in
    /// `.git`. Run outside the main copy: « OK sauf copie principale ».
    #[test]
    fn dry_run_never_merges() {
        for in_main in [true, false] {
            let (result, calls) = scenario_in(true, in_main, Vec::new());
            assert_eq!(result, Ok(in_main), "{calls:#?}");
            assert_eq!(
                calls
                    .iter()
                    .filter(|c| c.starts_with("gh pr view 7"))
                    .count(),
                2
            );
            for c in &calls {
                assert!(
                    !WRITES.iter().any(|w| c.starts_with(w)),
                    "write in --dry-run: {c}"
                );
                assert!(c.starts_with("gh ") || c.starts_with("git "), "{c}");
            }
            assert!(
                calls
                    .iter()
                    .any(|c| c == "git fetch --quiet origin pull/7/head")
            );
        }
        // Without --dry-run, outside the main copy: refused before any GitHub read.
        let (result, calls) = scenario_in(false, false, Vec::new());
        assert!(
            result
                .unwrap_err()
                .contains("pas depuis la copie principale"),
            "{calls:#?}"
        );
        assert!(!calls.iter().any(|c| c.starts_with("gh ")), "{calls:#?}");
    }

    /// AS-50 (#55): merge no longer checks the history, the GitHub settings nor the hooks. A PR
    /// that does not change `.github/` runs few commands: no `repo-settings.ps1`, no history walk,
    /// no workflow replay (J1, J8).
    #[test]
    fn merge_reads_only_the_pr() {
        let (result, calls) = scenario(false);
        assert_eq!(result, Ok(true), "{calls:#?}");
        for absent in [
            "pwsh",
            "git rev-list",
            "origin/pilotage",
            "git worktree",
            "git config",
            ".github/workflows/",
            "commits/refs/tags/",
        ] {
            assert!(
                !calls.iter().any(|c| c.contains(absent)),
                "{absent}: {calls:#?}"
            );
        }
        // 24 commands for a PR outside the sensitive paths and `.github/`: 12 `git`, 12 `gh`.
        assert!(calls.len() <= 24, "{} commands: {calls:#?}", calls.len());
        // A PR that changes a workflow (path compared without case) gets J1 and J8; it is
        // sensitive, so it carries both approvals.
        let extra = vec![
            (
                "gh api --paginate repos/o/r/pulls/7/files".to_owned(),
                json!([{"filename": ".GitHub/workflows/x.yml"}]).to_string(),
            ),
            (
                "gh api --paginate repos/o/r/issues/7/comments".to_owned(),
                json!([
                    owner_item(&format!("review-ok: {SHA}")),
                    owner_item(&format!("archsec-ok: {SHA}"))
                ])
                .to_string(),
            ),
        ];
        let (result, calls) = scenario_with(false, extra);
        assert_eq!(result, Ok(true), "{calls:#?}");
        assert!(
            calls
                .iter()
                .any(|c| c == &format!("git ls-tree -r -z --full-tree {SHA} .github/workflows/")),
            "{calls:#?}"
        );
    }

    /// AS-50 AU2, AU3: an open anomaly issue of `verify-main` (a commit of `main`, or the GitHub
    /// settings) written by the owner stops the merge before the PR is read; the same issue from
    /// another account, a pull request, or another owner's issue does not.
    #[test]
    fn open_anomaly_issue_stops_merges() {
        let titled = |title: &str| {
            let mut v = owner_item("x");
            v["title"] = json!(title);
            v["number"] = json!(41);
            v
        };
        let anomaly = || titled(&format!("{} 1234567", ANOMALY_TITLES[0]));
        let issues = |list: Value| {
            vec![(
                "gh api --paginate repos/o/r/issues?state=open".to_owned(),
                list.to_string(),
            )]
        };
        for title in ANOMALY_TITLES {
            let (result, calls) = scenario_with(false, issues(json!([titled(title)])));
            let e = result.unwrap_err();
            assert!(e.contains("merges suspended") && e.contains("#41"), "{e}");
            assert!(
                !calls
                    .iter()
                    .any(|c| c.starts_with("gh pr view") || c.starts_with("gh pr merge")),
                "{calls:#?}"
            );
        }
        // --dry-run goes on with every other check, and its result stays the refusal.
        let anomaly_list = issues(json!([anomaly()]));
        let (result, calls) = scenario_in(true, true, anomaly_list);
        assert!(result.unwrap_err().contains("#41"));
        assert!(
            calls.iter().any(|c| c.contains("/check-runs")),
            "{calls:#?}"
        );
        assert!(!calls.iter().any(|c| c.starts_with("gh pr merge")));
        let mut stranger = anomaly();
        stranger["user"]["login"] = json!("someone");
        stranger["author_association"] = json!("NONE");
        let mut pr = anomaly();
        pr["pull_request"] = json!({});
        let mut other = owner_item("x");
        other["title"] = json!("repo-settings drift");
        other["number"] = json!(42);
        let (result, calls) = scenario_with(false, issues(json!([stranger, pr, other])));
        assert_eq!(result, Ok(true), "{calls:#?}");
    }

    /// AS-52 (AU8): the issue that the nightly `report` job opens, written by `github-actions[bot]`
    /// under [`NIGHTLY_ANOMALY_TITLE`], stops the merge; another bot, another title, another
    /// account or a pull request does not. All open issues are read once, without the `creator`
    /// filter; a failed read refuses the merge.
    #[test]
    fn nightly_anomaly_issue_stops_merges() {
        let read = "gh api --paginate repos/o/r/issues?state=open&per_page=100";
        let issues = |list: Value| vec![(read.to_owned(), list.to_string())];
        let bot = || {
            json!({"title": NIGHTLY_ANOMALY_TITLE, "number": 43, "body": "Run: u",
                   "user": {"login": "github-actions[bot]", "type": "Bot"},
                   "author_association": "NONE",
                   "labels": [{"name": "nightly-failure"}, {"name": "role:architecture"}]})
        };
        let (result, calls) = scenario_with(false, issues(json!([bot()])));
        let e = result.unwrap_err();
        assert!(e.contains("merges suspended") && e.contains("#43"), "{e}");
        assert_eq!(
            calls.iter().filter(|c| c.contains("/issues?")).count(),
            1,
            "{calls:#?}"
        );
        assert!(calls.iter().any(|c| c == read), "{calls:#?}");
        assert!(!calls.iter().any(|c| c.contains("creator=")), "{calls:#?}");
        assert!(
            !calls
                .iter()
                .any(|c| c.starts_with("gh pr view") || c.starts_with("gh pr merge")),
            "{calls:#?}"
        );
        let mut other_bot = bot();
        other_bot["user"]["login"] = json!("dependabot[bot]");
        let mut other_title = bot();
        other_title["title"] = json!("Tâche périodique en échec");
        let mut user = bot();
        user["user"] = json!({"login": "someone", "type": "User"});
        let mut owner = owner_item("x");
        owner["title"] = json!(NIGHTLY_ANOMALY_TITLE);
        owner["number"] = json!(44);
        let mut pr = bot();
        pr["pull_request"] = json!({});
        let harmless = json!([other_bot, other_title, user, owner, pr]);
        let (result, calls) = scenario_with(false, issues(harmless));
        assert_eq!(result, Ok(true), "{calls:#?}");
        // A failed read of the issues refuses the merge before the PR is read (4xx: no retry).
        let failed = vec![(read.to_owned(), format!("{FAILS}gh: Not Found (HTTP 404)"))];
        let (result, calls) = scenario_with(false, failed);
        let e = result.unwrap_err();
        assert!(e.contains("HTTP 404"), "{e}");
        assert!(
            !calls
                .iter()
                .any(|c| c.starts_with("gh pr view") || c.starts_with("gh pr merge")),
            "{calls:#?}"
        );
    }

    /// What remains of J2 (AS-50): a main copy behind `origin/main` at launch is updated by
    /// `git pull --ff-only`, then the merge is refused before any GitHub read, so that the next
    /// launch rebuilds `xtask` from the new `main`. `--dry-run` only warns and pulls nothing.
    #[test]
    fn behind_main_copy_is_pulled_then_refused() {
        let behind = vec![(
            "git rev-parse HEAD origin/main".to_owned(),
            format!("{SHA}\n{MAIN}"),
        )];
        let (result, calls) = scenario_with(false, behind.clone());
        let e = result.unwrap_err();
        assert!(e.contains("relancer cargo xtask merge"), "{e}");
        assert!(calls.iter().any(|c| c.starts_with("git pull --ff-only")));
        assert!(!calls.iter().any(|c| c.starts_with("gh ")), "{calls:#?}");
        let (result, calls) = scenario_in(true, true, behind);
        assert_eq!(result, Ok(true), "{calls:#?}");
        assert!(!calls.iter().any(|c| c.starts_with("git pull")));
    }

    /// AS-50: a PR from a fork is never merged.
    #[test]
    fn fork_pr_is_refused() {
        let pr = json!({"number": 7, "state": "OPEN", "baseRefName": "main", "headRefOid": SHA,
            "isDraft": false, "isCrossRepository": true, "mergeable": "MERGEABLE",
            "changedFiles": 1, "title": "docs(test): x", "url": "u"});
        let (result, calls) =
            scenario_with(false, vec![("gh pr view 7".to_owned(), pr.to_string())]);
        assert!(result.unwrap_err().contains("fork"));
        assert!(
            !calls
                .iter()
                .any(|c| c.contains("pull/7/head") || c.starts_with("gh pr merge")),
            "{calls:#?}"
        );
        let mut unknown = pr.clone();
        unknown.as_object_mut().unwrap().remove("isCrossRepository");
        let (result, _) = scenario_with(
            false,
            vec![("gh pr view 7".to_owned(), unknown.to_string())],
        );
        assert!(result.unwrap_err().contains("fork"));
    }

    /// A merge is never made in CI mode: nothing runs.
    #[test]
    fn ci_mode_never_merges() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let repo = Repo {
            main: std::path::PathBuf::from("main"),
            here: std::path::PathBuf::from("main"),
            slug: "o/r".to_owned(),
            dry_run: false,
            ci: true,
            cache: std::env::temp_dir(),
            runner: Box::new(Fake {
                calls: Rc::clone(&calls),
                answers: Vec::new(),
            }),
        };
        assert!(run(&repo, 7).is_err());
        assert!(calls.borrow().is_empty());
    }

    /// AS-40 W7: approvals on an earlier SHA never stand in for `ci-ok` on the head: without it,
    /// merge refuses before reading any approval.
    #[test]
    fn carried_approvals_need_ci_ok_on_head() {
        let comments = json!([
            owner_item(&format!("review-ok: {MAIN}")),
            owner_item(&format!("archsec-ok: {MAIN}"))
        ]);
        let extra = vec![
            (
                "gh api --paginate repos/o/r/commits/".to_owned(),
                json!({"check_runs": []}).to_string(),
            ),
            (
                "gh api --paginate repos/o/r/issues/7/comments".to_owned(),
                comments.to_string(),
            ),
        ];
        let (result, calls) = scenario_with(false, extra);
        let e = result.unwrap_err();
        assert!(e.contains(&format!("ci-ok absent sur {SHA}")), "{e}");
        assert!(
            !calls
                .iter()
                .any(|c| c.contains("/comments") || c.starts_with("gh pr merge"))
        );
    }

    /// AS-40, merge wiring: the scenario PR now changes a sensitive file; both approvals were given
    /// on `a`, then `main` was merged in without touching the PR's file. With `a` among the PR's
    /// commits the approvals are carried over and the head is merged; without it, refusal (W1).
    #[test]
    fn carries_approvals_over_a_pure_update() {
        let a = "a".repeat(40);
        let f = "tools/xtask/src/x.rs";
        // m2: merge reads the trees of `a` and of the head from the fetched objects.
        let entry = |blob: &str| format!("100644 blob {}\t{f}\0", blob.repeat(40));
        let comments = json!([
            owner_item(&format!("review-ok: {a}")),
            owner_item(&format!("archsec-ok: {a}"))
        ]);
        let extra = |listed: &[&str], a_blob: &str| -> Vec<(String, String)> {
            let listed: Vec<_> = listed.iter().map(|s| json!({"sha": s})).collect();
            [
                (
                    "gh api --paginate repos/o/r/pulls/7/files".to_owned(),
                    json!([{"filename": f}]).to_string(),
                ),
                ("git diff --name-only".to_owned(), format!("{f}\0")),
                (
                    format!("git ls-tree -r -z --name-only --full-tree {SHA}"),
                    format!("{f}\0"),
                ),
                (
                    "gh api --paginate repos/o/r/issues/7/comments".to_owned(),
                    comments.to_string(),
                ),
                (
                    "gh api --paginate repos/o/r/pulls/7/commits".to_owned(),
                    json!(listed).to_string(),
                ),
                (
                    format!("gh api repos/o/r/compare/{a}...{SHA}"),
                    json!({"merge_base_commit": {"sha": a}}).to_string(),
                ),
                (
                    format!("gh api repos/o/r/compare/{MAIN}..."),
                    json!({"merge_base_commit": {"sha": MAIN}}).to_string(),
                ),
                (format!("git ls-tree -r -z --full-tree {SHA}"), entry("b")),
                (
                    format!("git ls-tree -r -z --full-tree {SHA} .github/workflows/"),
                    String::new(),
                ),
                (format!("git ls-tree -r -z --full-tree {a}"), entry(a_blob)),
                (
                    format!("git cat-file blob {MAIN}:tools/ci/sensitive-paths.txt"),
                    include_str!("../../ci/sensitive-paths.txt").to_owned(),
                ),
            ]
            .into_iter()
            .collect()
        };
        let merged = |calls: &[String]| calls.iter().any(|c| c.starts_with("gh pr merge"));
        let (result, calls) = scenario_with(false, extra(&[&a, SHA], "b"));
        assert_eq!(result, Ok(true), "{calls:#?}");
        assert!(merged(&calls));
        assert!(
            !calls.iter().any(|c| c.contains("/git/trees/")),
            "{calls:#?}"
        );
        let (result, calls) = scenario_with(false, extra(&[SHA], "b"));
        let e = result.unwrap_err();
        assert!(e.contains("W1: not among"), "{e}");
        assert!(!merged(&calls));
        // The local tree of `a` has another blob: W2.
        let (result, calls) = scenario_with(false, extra(&[&a, SHA], "d"));
        let e = result.unwrap_err();
        assert!(
            e.contains("W2: tools/xtask/src/x.rs has another blob"),
            "{e}"
        );
        assert!(!merged(&calls));
    }

    /// Witness: the same scenario without `--dry-run` merges the checked head.
    #[test]
    fn real_run_merges_checked_head() {
        let (result, calls) = scenario(false);
        assert_eq!(result, Ok(true), "{calls:#?}");
        let merge =
            format!("gh pr merge 7 -R o/r --squash --delete-branch --match-head-commit {SHA}");
        assert_eq!(
            calls.iter().filter(|c| **c == merge).count(),
            1,
            "{calls:#?}"
        );
        // m5: the PR state is read again right before gh pr merge.
        let at = calls.iter().position(|c| *c == merge).unwrap();
        assert!(
            calls[..at]
                .iter()
                .rev()
                .take(2)
                .any(|c| c.starts_with("gh pr view 7")),
            "{calls:#?}"
        );
    }

    /// A sensitive PR without `review-ok` or without `archsec-ok` on its head (nor carried over):
    /// refused, never merged.
    #[test]
    fn sensitive_pr_needs_both_approvals() {
        let f = "tools/xtask/src/x.rs";
        for given in ["review-ok", "archsec-ok"] {
            let extra = vec![
                (
                    "gh api --paginate repos/o/r/pulls/7/files".to_owned(),
                    json!([{ "filename": f }]).to_string(),
                ),
                ("git diff --name-only".to_owned(), format!("{f}\0")),
                (
                    "gh api --paginate repos/o/r/issues/7/comments".to_owned(),
                    json!([owner_item(&format!("{given}: {SHA}"))]).to_string(),
                ),
            ];
            let (result, calls) = scenario_with(false, extra);
            let e = result.unwrap_err();
            assert!(e.contains("sensitive path"), "{e}");
            assert!(!calls.iter().any(|c| c.starts_with("gh pr merge")));
        }
    }
}
