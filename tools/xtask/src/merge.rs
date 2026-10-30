//! `cargo xtask merge <PR> [--dry-run]` : **seul chemin de fusion** (SEC0-15 révisée). Lancé
//! depuis la copie principale, avec le binaire compilé depuis `main` (J2). Les contrôles se font
//! dans l'ordre de la consigne T3a, et le premier qui échoue arrête tout : en cas de doute, refus.
//! `--dry-run` fait tous les contrôles, sans fusion ni *issue* : il poursuit après une suspension
//! des fusions (étape 2) pour tout montrer, mais son résultat reste alors un refus. Il n'appelle
//! jamais `gh pr merge` ni `git pull`, et ne touche ni l'arbre, ni l'index, ni les branches de la
//! copie principale (test `dry_run_never_merges`). Les deux `git fetch` (`origin` et
//! `pull/<n>/head`) écrivent en revanche dans `.git` : références distantes, `FETCH_HEAD`, objets.
//! Hors de la copie principale, son résultat est au mieux « OK sauf J2 », avec le code 3 (m1).
//! A GitHub settings drift alone lets through only a repair PR (AS-38, step 6a).
//! spec: carnet architecture-securite.md SEC0-15, SEC0-16 ; §9.6 G3, G8 ; §9.8 J1, J2, J8
//! Voir aussi l'issue #6, conditions T3-a et T3-b.

use std::collections::BTreeMap;
use std::process::ExitCode;

use serde_json::Value;

use crate::carry::{Carry, Pr};
use crate::github::{Repo, is_sha, same_path};
use crate::{build_inputs, hooks, sensitive, verify_main, workflows};

use crate::BUILT_FROM;

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

/// AS-38: the only file a repair PR may change while the GitHub settings drift.
pub const SETTINGS_FILE: &str = "tools/ci/repo-settings.json";

/// AS-38, condition 1: the PR changes `tools/ci/repo-settings.json` and nothing else, by the full
/// list of step 6 (`count` files from the API; `paths`: both paths of a rename, local diff
/// included). Paths compare byte for byte: a case variant or a Windows 8.3 alias of the file
/// (G8, B1) is another path, so a refusal.
pub fn repair_only(count: usize, paths: &[String]) -> Result<(), String> {
    if count == 1 && paths == [SETTINGS_FILE] {
        return Ok(());
    }
    let others: Vec<&str> = paths
        .iter()
        .map(String::as_str)
        .filter(|p| *p != SETTINGS_FILE)
        .collect();
    let alias = others.iter().any(|p| {
        sensitive::norm(p) == SETTINGS_FILE || p.split(['/', '\\']).any(sensitive::short_name)
    });
    Err(format!(
        "not a repair PR: it must change {SETTINGS_FILE} only ({count} file(s) listed; other \
         paths: {}{})",
        if others.is_empty() {
            "none".to_owned()
        } else {
            others.join(", ")
        },
        if alias {
            "; a case variant or 8.3 name of the file is another path (G8)"
        } else {
            ""
        }
    ))
}

/// Merges suspended by a settings drift alone: names the AS-38 exception and what follows it.
fn drift_refusal(why: &str) -> String {
    refuse(
        format!(
            "merges suspended: the GitHub settings drift from {SETTINGS_FILE} (issue \
             repo-settings-drift). AS-38 exception: only a PR that changes {SETTINGS_FILE} alone, \
             whose file passes the S2 invariants (AS-41) and leaves no drift, goes through, with a conforming \
             history and hooks and every other check green. Refused: {why}"
        ),
        "align tools/ci/repo-settings.json with GitHub in such a PR, or the orchestrator runs \
         pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Apply from the main copy; after a \
         repair PR is merged, the orchestrator runs -Apply, then -Check",
    )
}

/// A refusal that `--dry-run` notes and goes past: its result stays a refusal (the first one).
fn hold(repo: &Repo, suspended: &mut Option<String>, why: String) -> Result<(), String> {
    if !repo.dry_run {
        return Err(why);
    }
    println!("  REFUS: {why}\n  --dry-run goes on with the checks; the result stays a refusal.");
    suspended.get_or_insert(why);
    Ok(())
}

/// Refus, avec la commande qui débloque.
fn refuse(why: impl std::fmt::Display, fix: &str) -> String {
    format!("{why}\n  Pour débloquer : {fix}")
}

fn step(n: &str, what: &str) {
    println!("\n[{n}] {what}");
}

/// Empreinte des entrées de construction de `xtask` dans `HEAD` de la copie principale.
fn head_fingerprint(repo: &Repo) -> Result<String, String> {
    let ls = repo.git(&["ls-tree", "-r", "-z", "--full-tree", "HEAD"])?;
    let mut files = Vec::new();
    for entry in ls.split('\0').filter(|e| !e.is_empty()) {
        let (meta, path) = entry
            .split_once('\t')
            .ok_or("sortie de git ls-tree illisible")?;
        if !build_inputs::is_input(path) {
            continue;
        }
        let oid = match meta.split(' ').collect::<Vec<_>>()[..] {
            ["100644" | "100755", "blob", oid] => oid,
            _ => {
                return Err(format!(
                    "{path} : entrée de construction inattendue ({meta})"
                ));
            }
        };
        files.push((
            path.to_owned(),
            repo.out(&repo.main, "git", &["cat-file", "blob", oid])?,
        ));
    }
    Ok(build_inputs::fingerprint(&files))
}

/// J2 : binaire sous `<copie principale>\target`, construit depuis les entrées exactes de `HEAD`.
fn binary_check(repo: &Repo) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("exécutable introuvable : {e}"))?;
    // Chemins canoniques (jonction de l'emplacement résolue, casse du disque) : comparaison par composants.
    let (exe_c, target) = (
        std::fs::canonicalize(&exe),
        std::fs::canonicalize(repo.main.join("target")),
    );
    let inside = matches!((exe_c, target), (Ok(e), Ok(t)) if e.starts_with(&t));
    if !inside {
        return Err(refuse(
            format!(
                "binaire hors de {}\\target ({})",
                repo.main.display(),
                exe.display()
            ),
            "lancer cargo xtask merge <PR> depuis la racine de la copie principale",
        ));
    }
    let want = head_fingerprint(repo)?;
    if BUILT_FROM != want {
        return Err(refuse(
            format!(
                "binaire construit depuis d'autres entrées que HEAD de main (empreinte {BUILT_FROM}, attendu {want})"
            ),
            "relancer cargo xtask merge <PR> depuis la racine de la copie principale : cargo reconstruit xtask",
        ));
    }
    Ok(())
}

/// Étape 1 : copie principale propre, sur `main`, à jour d'`origin/main` (par `--ff-only`).
/// Rend faux en `--dry-run` si J2 n'est pas rempli (m1).
fn main_copy(repo: &Repo, j2_check: J2Check) -> Result<bool, String> {
    let pull = "git pull --ff-only dans la copie principale";
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
    if !repo.dry_run {
        repo.git(&["pull", "--ff-only", "--quiet"])?;
    }
    let behind = repo.git(&["rev-parse", "HEAD"])? != repo.git(&["rev-parse", "origin/main"])?;
    if behind && repo.dry_run {
        // La vraie fusion fait git pull --ff-only ; les contrôles suivants lisent origin/main.
        println!(
            "  AVERTISSEMENT (--dry-run) : copie principale en retard d'origin/main ({pull})."
        );
    } else if behind {
        return Err(refuse(
            "la copie principale n'est pas à jour d'origin/main",
            pull,
        ));
    }
    if repo.dry_run {
        let mut j2 = true;
        if let Err(e) = j2_check(repo) {
            println!(
                "  AVERTISSEMENT (--dry-run) : contrôle J2 non rempli, bloquant sans --dry-run :\n  {e}"
            );
            j2 = false;
        }
        if !same_path(&repo.here, &repo.main) {
            println!(
                "  AVERTISSEMENT (--dry-run) : lancé hors de la copie principale ({}).",
                repo.here.display()
            );
            j2 = false;
        }
        return Ok(j2);
    }
    if !same_path(&repo.here, &repo.main) {
        return Err(refuse(
            format!(
                "lancé depuis {}, pas depuis la copie principale",
                repo.here.display()
            ),
            "cd vers la copie principale, puis cargo xtask merge <PR>",
        ));
    }
    j2_check(repo).map(|()| true)
}

/// PR ouverte, base `main`, hors brouillon, fusionnable, en retard de 0 commit, titre A31.
fn pr_state(repo: &Repo, pr: u64) -> Result<Value, String> {
    let v = repo.gh(&[
        "pr",
        "view",
        &pr.to_string(),
        "-R",
        &repo.slug,
        "--json",
        "number,state,baseRefName,headRefOid,isDraft,mergeable,changedFiles,title,url",
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

/// Vrai si tout est conforme ; faux en `--dry-run` quand seul J2 manque (m1).
/// J2 check of the running binary: [`binary_check`], replaced only in tests.
type J2Check = fn(&Repo) -> Result<(), String>;

fn run(repo: &Repo, pr: u64) -> Result<bool, String> {
    run_with(repo, pr, binary_check)
}

fn run_with(repo: &Repo, pr: u64, j2_check: J2Check) -> Result<bool, String> {
    // The CI mode skips repo-settings.ps1 -Check: never in a merge (AS-31 E4).
    if repo.ci {
        return Err(refuse(
            "CI et GITHUB_ACTIONS valent true : mode CI, où les réglages GitHub ne sont pas contrôlés",
            "lancer cargo xtask merge <PR> hors CI, sans ces variables",
        ));
    }
    step(
        "1",
        "copie principale propre, sur main, à jour ; binaire construit depuis main (J2)",
    );
    let j2 = main_copy(repo, j2_check)?;
    step(
        "2",
        "verify-main (SEC0-17, réglages GitHub AS-30 R5) et hooks verify (SEC0-35)",
    );
    let health = verify_main::verify_parts(repo, true)?;
    let hooks_ok = hooks::verify_or_report(repo)?;
    // AS-38, condition 3: a settings drift alone leaves one way through, a repair PR (step 6a).
    let repair = health.history && hooks_ok && health.settings == Some(false);
    if repair {
        println!(
            "  settings drift alone: merges suspended except for a repair PR (AS-38, step 6a)"
        );
    }
    // `settings: None`: -Check could not conclude, which only --dry-run goes past.
    let mut suspended = (!health.history || !hooks_ok || health.settings.is_none()).then(|| {
        refuse(
            "fusions suspendues : anomalie de l'historique ou des crochets, ou réglages GitHub \
             non contrôlés ; un écart des réglages, s'il y en a un, n'ouvre alors aucune \
             exception (AS-38)",
            "lever l'anomalie (issue role:architecture), puis cargo xtask verify-main et \
             cargo xtask hooks verify ; pour un écart des réglages (issue repo-settings-drift), \
             l'orchestrateur lance pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Apply \
             depuis la copie principale",
        )
    });
    match &suspended {
        Some(why) if !repo.dry_run => return Err(why.clone()),
        Some(_) => println!(
            "  REFUS : fusions suspendues ; --dry-run poursuit les contrôles, le résultat restera un refus."
        ),
        None => {}
    }
    step(
        "3-4",
        "PR ouverte, base main, fusionnable, à jour ; titre (A31)",
    );
    let p = pr_state(repo, pr)?;
    let sha = p["headRefOid"].as_str().unwrap_or_default().to_owned();
    println!(
        "  {} : {} ({sha})",
        p["url"].as_str().unwrap_or_default(),
        p["title"].as_str().unwrap_or_default()
    );
    // AS-38: during a drift, a PR of more than one file is refused here, before any fetch.
    if repair && p["changedFiles"].as_u64() != Some(1) {
        let why = format!("not a repair PR: {} files changed", p["changedFiles"]);
        hold(repo, &mut suspended, drift_refusal(&why))?;
    }
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
    if repair {
        step("6a", "settings repair exception (AS-38)");
        // Condition 2 only once condition 1 holds: the script reads the head's file. The main
        // copy's script (equal to origin/main, S4) first applies its S2 invariants to that file,
        // which pin every protective value (AS-41): a file that records a weakening is refused.
        let why = if let Err(e) = repair_only(count, &paths) {
            Some(e)
        } else if !tree.iter().any(|f| f == SETTINGS_FILE) {
            Some(format!(
                "not a repair PR: {SETTINGS_FILE} is deleted at the head"
            ))
        } else if verify_main::settings_at(repo, &sha)? {
            None
        } else {
            Some(format!(
                "{SETTINGS_FILE} of the head {sha} fails the S2 invariants of origin/main's \
                 script (AS-41), or the drift recomputed with it is not empty"
            ))
        };
        match why {
            None => println!(
                "  OK: repair PR, no drift with the head's file; approvals required (step 10)"
            ),
            Some(why) => hold(repo, &mut suspended, drift_refusal(&why))?,
        }
    }
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
    workflows_at(repo, &sha)?;
    step(
        "10",
        "approbations sur le SHA de tête, ou reportées d'un SHA antérieur (chemin sensible, AS-40)",
    );
    // AS-38, condition 4: a repair PR needs both approvals, whatever the sensitive list says.
    if hits.is_empty() && !repair {
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
                Ok(from) => println!(
                    "  {tag} carried over from {from} to the head {sha} (AS-40, W1 to W5 hold)"
                ),
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
        return Ok(j2);
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
    if repair {
        println!(
            "  AS-38: repair PR merged; the orchestrator now runs, from the main copy, \
             pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Apply, then -Check"
        );
    }
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
                "\nRésultat : OK sauf J2 (--dry-run hors de la copie principale, ou binaire non construit depuis main)"
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

    use crate::github::{Output, Run};

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

    /// B2 : sur une copie propre, l'empreinte inscrite par `build.rs` égale celle recalculée sur
    /// `HEAD` par Git. Ignoré par défaut (exige une copie sans modification non commise) :
    /// `cargo test -p xtask -- --ignored built_from_head`.
    #[test]
    #[ignore = "copie de travail propre exigée"]
    fn built_from_head() {
        let root = crate::sys::stdout_of(Path::new("."), "git", &["rev-parse", "--show-toplevel"])
            .unwrap();
        let root = std::path::PathBuf::from(root.trim());
        let repo = Repo {
            main: root.clone(),
            here: root,
            slug: String::new(),
            dry_run: true,
            ci: false,
            cache: std::env::temp_dir(),
            home: None,
            runner: Box::new(crate::github::System),
        };
        assert_eq!(head_fingerprint(&repo).unwrap(), BUILT_FROM);
    }

    /// Lanceur enregistreur : réponses par préfixe de commande (le plus long gagne), appels notés.
    /// Exit code by command suffix in `codes` (the longest wins), 0 otherwise.
    struct Fake {
        calls: Rc<RefCell<Vec<String>>>,
        answers: Vec<(String, String)>,
        codes: Vec<(String, i32)>,
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
            let code = self
                .codes
                .iter()
                .filter(|(k, _)| line.ends_with(k.as_str()))
                .max_by_key(|(k, _)| k.len())
                .map_or(0, |(_, c)| *c);
            match best {
                Some((_, out)) => Ok((Some(code), out.clone().into_bytes(), String::new())),
                None => Err(format!("commande inattendue : {line}")),
            }
        }
    }

    /// Changes to [`scenario`]: answers replacing those of the same prefix, exit codes by command
    /// suffix, and a broken `hooks verify` (wrong `current.sha256`).
    #[derive(Default)]
    struct Setup {
        extra: Vec<(String, String)>,
        codes: Vec<(String, i32)>,
        break_hooks: bool,
    }

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

    /// Plays `merge` on recorded answers for a sensitive-free PR #7 whose every check passes,
    /// `hooks verify` included (temporary cache with the pinned set, fake gitleaks, empty home).
    fn scenario(dry_run: bool, j2_check: J2Check) -> (Result<bool, String>, Vec<String>) {
        scenario_with(dry_run, j2_check, Vec::new())
    }

    /// [`scenario`], where `extra` answers replace those of the same prefix.
    fn scenario_with(
        dry_run: bool,
        j2_check: J2Check,
        extra: Vec<(String, String)>,
    ) -> (Result<bool, String>, Vec<String>) {
        scenario_setup(
            dry_run,
            j2_check,
            Setup {
                extra,
                ..Setup::default()
            },
        )
    }

    /// [`scenario`], changed by `setup`.
    fn scenario_setup(
        dry_run: bool,
        j2_check: J2Check,
        setup: Setup,
    ) -> (Result<bool, String>, Vec<String>) {
        let Setup {
            extra,
            codes,
            break_hooks,
        } = setup;
        // One folder per call: tests run in parallel.
        static RUN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let run = RUN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let base = std::env::temp_dir().join(format!(
            "xtask-merge-test-{}-{dry_run}-{run}",
            std::process::id()
        ));
        let (main, cache, home) = (base.join("main"), base.join("cache"), base.join("home"));
        for d in [&main, &home] {
            std::fs::create_dir_all(d).unwrap();
        }
        let gitleaks = base.join("gitleaks.exe");
        crate::sys::write(&gitleaks, b"fake gitleaks").unwrap();
        let lock = format!(
            "gitleaks.path = \"{}\"\ngitleaks.sha256 = \"{}\"\n",
            gitleaks.display(),
            crate::sha256::hex(b"fake gitleaks")
        );
        let set = vec![("tools.lock".to_owned(), lock.clone().into_bytes())];
        let hash = hooks::set_hash(&set).unwrap();
        std::fs::create_dir_all(cache.join("hooks").join(&hash)).unwrap();
        crate::sys::write(
            &cache.join("hooks").join(&hash).join("tools.lock"),
            lock.as_bytes(),
        )
        .unwrap();
        let current = if break_hooks { "0".repeat(64) } else { hash };
        crate::sys::write(
            &cache.join("hooks").join("current.sha256"),
            current.as_bytes(),
        )
        .unwrap();

        let pr = json!({"number": 7, "state": "OPEN", "baseRefName": "main", "headRefOid": SHA,
            "isDraft": false, "mergeable": "MERGEABLE", "changedFiles": 1,
            "title": "docs(test): x", "url": "u"});
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
            ("git ls-tree", String::new()),
            ("git ls-files", String::new()),
            ("git log --oneline -1", "x".to_owned()),
            ("git cat-file -e", String::new()),
            ("git cat-file blob origin/main:tools/dev/tools.lock", lock),
            (
                "git cat-file blob origin/main:tools/ci/sensitive-paths.txt",
                include_str!("../../ci/sensitive-paths.txt").to_owned(),
            ),
            ("git worktree list", String::new()),
            (
                "git cat-file blob origin/pilotage:architecture-securite.md",
                "## 9. Avis\n".to_owned(),
            ),
            (
                "git rev-list --reverse origin/main",
                "8ca4088bd38829d6c5aaa9281c4fa32b33a95e35\n2b5681461e46cc1cc008e786b8b04ccd22b4993f\n"
                    .to_owned(),
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
                "pwsh -NoProfile -NonInteractive -File",
                "repo-settings -Check : conforme\n".to_owned(),
            ),
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
            here: main,
            slug: "o/r".to_owned(),
            dry_run,
            ci: false,
            cache,
            home: Some(home),
            runner: Box::new(Fake {
                calls: Rc::clone(&calls),
                answers,
                codes,
            }),
        };
        let result = run_with(&repo, 7, j2_check);
        let _ = std::fs::remove_dir_all(&base); // temporary files written above only
        let calls = calls.borrow().clone();
        (result, calls)
    }

    /// B4, AS-23 point 2: a `--dry-run` whose every check passes never calls `gh pr merge`, and
    /// runs no command that writes to the main copy or to GitHub. Only the `git fetch` write, in
    /// `.git`. The binary is not under the fake main copy's `target`, so J2 fails: « OK sauf J2 ».
    #[test]
    fn dry_run_never_merges() {
        let (result, calls) = scenario(true, binary_check);
        assert_eq!(result, Ok(false), "{calls:#?}");
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
            // AS-30 R5: the only other command is the read-only `repo-settings.ps1 -Check`.
            let settings = c.starts_with("pwsh ") && c.ends_with("repo-settings.ps1 -Check");
            assert!(
                c.starts_with("gh ") || c.starts_with("git ") || settings,
                "{c}"
            );
        }
        assert_eq!(
            calls.iter().filter(|c| c.starts_with("pwsh ")).count(),
            1,
            "{calls:#?}"
        );
        assert!(
            calls
                .iter()
                .any(|c| c == "git fetch --quiet origin pull/7/head")
        );
    }

    /// AS-31 E4: in CI mode, where `repo-settings.ps1 -Check` is skipped, merge runs nothing.
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
            home: None,
            runner: Box::new(Fake {
                calls: Rc::clone(&calls),
                answers: Vec::new(),
                codes: Vec::new(),
            }),
        };
        assert!(run_with(&repo, 7, |_| Ok(())).is_err());
        assert!(calls.borrow().is_empty());
    }

    /// AS-40 W7: approvals on an earlier SHA never stand in for `ci-ok` on the head: without it,
    /// merge refuses before reading any approval.
    #[test]
    fn carried_approvals_need_ci_ok_on_head() {
        let comments = json!([{"body": format!("review-ok: {MAIN}")},
                              {"body": format!("archsec-ok: {MAIN}")}]);
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
        let (result, calls) = scenario_with(false, |_| Ok(()), extra);
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
        let comments = json!([{"body": format!("review-ok: {a}")},
                              {"body": format!("archsec-ok: {a}")}]);
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
        let (result, calls) = scenario_with(false, |_| Ok(()), extra(&[&a, SHA], "b"));
        assert_eq!(result, Ok(true), "{calls:#?}");
        assert!(merged(&calls));
        assert!(
            !calls.iter().any(|c| c.contains("/git/trees/")),
            "{calls:#?}"
        );
        let (result, calls) = scenario_with(false, |_| Ok(()), extra(&[SHA], "b"));
        let e = result.unwrap_err();
        assert!(e.contains("W1: not among"), "{e}");
        assert!(!merged(&calls));
        // The local tree of `a` has another blob: W2.
        let (result, calls) = scenario_with(false, |_| Ok(()), extra(&[&a, SHA], "d"));
        let e = result.unwrap_err();
        assert!(
            e.contains("W2: tools/xtask/src/x.rs has another blob"),
            "{e}"
        );
        assert!(!merged(&calls));
    }

    /// Witness: the same scenario without `--dry-run` (J2 replaced) merges the checked head.
    #[test]
    fn real_run_merges_checked_head() {
        let (result, calls) = scenario(false, |_| Ok(()));
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

    /// AS-38: PR #7 changes only `tools/ci/repo-settings.json`, with both approvals on its head,
    /// while `-Check` drifts (code 1) and `-Check -SourceCommit <head>` does not. `more` answers
    /// and `codes` replace those of the same key.
    fn repair_setup(more: Vec<(String, String)>, codes: Vec<(String, i32)>) -> Setup {
        let comments = json!([{"body": format!("review-ok: {SHA}")},
                              {"body": format!("archsec-ok: {SHA}")}]);
        let mut extra: Vec<(String, String)> = vec![
            (
                "gh api --paginate repos/o/r/pulls/7/files".to_owned(),
                json!([{ "filename": SETTINGS_FILE }]).to_string(),
            ),
            (
                "git diff --name-only".to_owned(),
                format!("{SETTINGS_FILE}\0"),
            ),
            (
                format!("git ls-tree -r -z --name-only --full-tree {SHA}"),
                format!("{SETTINGS_FILE}\0"),
            ),
            (
                "gh api --paginate repos/o/r/issues/7/comments".to_owned(),
                comments.to_string(),
            ),
            ("gh issue list".to_owned(), "[]".to_owned()),
            ("gh issue create".to_owned(), "u".to_owned()),
        ];
        extra.extend(more);
        let mut all = vec![
            ("repo-settings.ps1 -Check".to_owned(), 1),
            (format!("repo-settings.ps1 -Check -SourceCommit {SHA}"), 0),
        ];
        all.extend(codes);
        Setup {
            extra,
            codes: all,
            break_hooks: false,
        }
    }

    fn repair(
        more: Vec<(String, String)>,
        codes: Vec<(String, i32)>,
    ) -> (Result<bool, String>, Vec<String>) {
        scenario_setup(false, |_| Ok(()), repair_setup(more, codes))
    }

    fn merged(calls: &[String]) -> bool {
        calls.iter().any(|c| c.starts_with("gh pr merge"))
    }

    /// `repo-settings.ps1 -Check -SourceCommit <head>` was run.
    fn head_check(calls: &[String]) -> bool {
        let tail = format!("repo-settings.ps1 -Check -SourceCommit {SHA}");
        calls
            .iter()
            .any(|c| c.starts_with("pwsh -NoProfile -NonInteractive -File ") && c.ends_with(&tail))
    }

    fn assert_drift_refusal(e: &str) {
        assert!(
            e.contains("AS-38 exception") && e.contains("-Apply, then -Check"),
            "{e}"
        );
    }

    /// AS-38, nominal case: despite the drift, a repair PR whose every other check passes is
    /// merged. The drift is recomputed with the file of the PR's head (`-SourceCommit <head>`,
    /// read by `git show` in the script), not with the disk's nor `origin/main`'s; never `-Apply`.
    #[test]
    fn repair_pr_merges_despite_drift() {
        let (result, calls) = repair(vec![], vec![]);
        assert_eq!(result, Ok(true), "{calls:#?}");
        assert!(merged(&calls) && head_check(&calls), "{calls:#?}");
        assert_eq!(
            calls.iter().filter(|c| c.starts_with("pwsh ")).count(),
            2,
            "{calls:#?}"
        );
        assert!(!calls.iter().any(|c| c.contains("-Apply")), "{calls:#?}");
        // AS-41: the script run is the main copy's (S2 of origin/main by S4), never the head's.
        let sep = std::path::MAIN_SEPARATOR;
        let main_script =
            format!("main{sep}tools{sep}dev{sep}repo-settings.ps1 -Check -SourceCommit {SHA}");
        assert!(
            calls.iter().any(|c| c.ends_with(&main_script)),
            "{calls:#?}"
        );
        // Both approvals were read, though the sensitive list is not consulted for them.
        assert!(calls.iter().any(|c| c.contains("/issues/7/comments")));
    }

    /// AS-38, condition 1: another file, by the local diff or as the other path of a rename, or
    /// a case variant of the file (G8): refused, without recomputing the drift.
    #[test]
    fn repair_refused_if_another_file() {
        for (files, local) in [
            (
                json!([{ "filename": SETTINGS_FILE }]),
                format!("{SETTINGS_FILE}\0docs/a.md\0"),
            ),
            (
                json!([{"filename": SETTINGS_FILE, "previous_filename": "tools/ci/old.json"}]),
                format!("{SETTINGS_FILE}\0"),
            ),
            (
                json!([{ "filename": "tools/ci/Repo-Settings.json" }]),
                "tools/ci/Repo-Settings.json\0".to_owned(),
            ),
            // AS-41: a head that also changes the script (S2 invariants) is no repair PR.
            (
                json!([{ "filename": SETTINGS_FILE }]),
                format!("{SETTINGS_FILE}\0tools/dev/repo-settings.ps1\0"),
            ),
        ] {
            let more = vec![
                (
                    "gh api --paginate repos/o/r/pulls/7/files".to_owned(),
                    files.to_string(),
                ),
                ("git diff --name-only".to_owned(), local.clone()),
                (
                    format!("git ls-tree -r -z --name-only --full-tree {SHA}"),
                    local,
                ),
            ];
            let (result, calls) = repair(more, vec![]);
            let e = result.unwrap_err();
            assert!(e.contains("not a repair PR"), "{e}");
            assert_drift_refusal(&e);
            assert!(!merged(&calls) && !head_check(&calls), "{calls:#?}");
        }
        // --dry-run goes on, but the result stays a refusal.
        let more = vec![(
            "git diff --name-only".to_owned(),
            format!("{SETTINGS_FILE}\0docs/a.md\0"),
        )];
        let (result, calls) = scenario_setup(true, |_| Ok(()), repair_setup(more, vec![]));
        assert_drift_refusal(&result.unwrap_err());
        assert!(!merged(&calls));
        // More than one file announced: refused at step 3-4, before ci-ok and any fetch.
        let pr = json!({"number": 7, "state": "OPEN", "baseRefName": "main", "headRefOid": SHA,
            "isDraft": false, "mergeable": "MERGEABLE", "changedFiles": 2,
            "title": "docs(test): x", "url": "u"});
        let (result, calls) = repair(vec![("gh pr view 7".to_owned(), pr.to_string())], vec![]);
        let e = result.unwrap_err();
        assert!(e.contains("2 files changed"), "{e}");
        assert_drift_refusal(&e);
        assert!(
            !calls
                .iter()
                .any(|c| c.contains("pull/7/head") || c.contains("/check-runs")),
            "{calls:#?}"
        );
        // The file deleted at the head: refused, the drift is not recomputed.
        let tree = format!("git ls-tree -r -z --name-only --full-tree {SHA}");
        let (result, calls) = repair(vec![(tree, "docs/a.md\0".to_owned())], vec![]);
        let e = result.unwrap_err();
        assert!(e.contains("deleted at the head"), "{e}");
        assert!(!merged(&calls) && !head_check(&calls), "{calls:#?}");
    }

    /// A `-Check` that could not conclude is no drift: `--dry-run` goes on, but it opens no
    /// repair path and the result stays a refusal.
    #[test]
    fn unconcluded_check_is_no_repair_path() {
        let codes = vec![("repo-settings.ps1 -Check".to_owned(), 2)];
        let (result, calls) = scenario_setup(true, |_| Ok(()), repair_setup(vec![], codes));
        let e = result.unwrap_err();
        assert!(e.contains("non contrôlés"), "{e}");
        assert!(!merged(&calls) && !head_check(&calls), "{calls:#?}");
    }

    #[test]
    fn repair_only_paths() {
        let p = |v: &[&str]| v.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        assert!(repair_only(1, &p(&[SETTINGS_FILE])).is_ok());
        assert!(repair_only(2, &p(&[SETTINGS_FILE])).is_err());
        assert!(repair_only(1, &p(&[])).is_err());
        for alias in ["TOOLS/ci/repo-settings.json", "tools/ci/REPO-S~1.JSO"] {
            let e = repair_only(1, &p(&[alias])).unwrap_err();
            assert!(e.contains("(G8)"), "{e}");
        }
    }

    /// AS-38, condition 2: the drift recomputed with the head's file is not empty: refused.
    #[test]
    fn repair_refused_if_head_file_drifts() {
        let code = vec![(format!("repo-settings.ps1 -Check -SourceCommit {SHA}"), 1)];
        let (result, calls) = repair(vec![], code);
        let e = result.unwrap_err();
        assert!(e.contains("is not empty"), "{e}");
        assert_drift_refusal(&e);
        assert!(head_check(&calls) && !merged(&calls), "{calls:#?}");
    }

    /// AS-38, condition 3: a history anomaly, or non-conforming hooks, suspends every merge,
    /// repair PRs included.
    #[test]
    fn repair_refused_if_history_or_hooks_fail() {
        let x = "9".repeat(40);
        let more = vec![
            (
                "git rev-list --reverse origin/main".to_owned(),
                format!(
                    "8ca4088bd38829d6c5aaa9281c4fa32b33a95e35\n\
                     2b5681461e46cc1cc008e786b8b04ccd22b4993f\n{x}\n"
                ),
            ),
            ("git rev-list --parents".to_owned(), x.clone()),
        ];
        let (result, calls) = repair(more, vec![]);
        assert!(result.unwrap_err().contains("fusions suspendues"));
        assert!(!merged(&calls) && !head_check(&calls), "{calls:#?}");
        let mut setup = repair_setup(vec![], vec![]);
        setup.break_hooks = true;
        let (result, calls) = scenario_setup(false, |_| Ok(()), setup);
        assert!(result.unwrap_err().contains("fusions suspendues"));
        assert!(!merged(&calls) && !head_check(&calls), "{calls:#?}");
    }

    /// AS-38, condition 4: a missing approval, or no `ci-ok` on the head: refused.
    #[test]
    fn repair_refused_without_approval_or_ci_ok() {
        let comments = json!([{"body": format!("review-ok: {SHA}")}]);
        let more = vec![(
            "gh api --paginate repos/o/r/issues/7/comments".to_owned(),
            comments.to_string(),
        )];
        let (result, calls) = repair(more, vec![]);
        let e = result.unwrap_err();
        assert!(e.contains("archsec-ok"), "{e}");
        assert!(!merged(&calls), "{calls:#?}");
        let more = vec![(
            "gh api --paginate repos/o/r/commits/".to_owned(),
            json!({"check_runs": []}).to_string(),
        )];
        let (result, calls) = repair(more, vec![]);
        let e = result.unwrap_err();
        assert!(e.contains(&format!("ci-ok absent sur {SHA}")), "{e}");
        assert!(!merged(&calls), "{calls:#?}");
    }
}
