//! `cargo xtask merge <PR> [--dry-run]` : **seul chemin de fusion** (SEC0-15 révisée). Lancé
//! depuis la copie principale, avec le binaire compilé depuis `main` (J2). Les contrôles se font
//! dans l'ordre de la consigne T3a, et le premier qui échoue arrête tout : en cas de doute, refus.
//! `--dry-run` fait tous les contrôles, sans fusion ni *issue* : il poursuit après une suspension
//! des fusions (étape 2) pour tout montrer, mais son résultat reste alors un refus. Il n'appelle
//! jamais `gh pr merge` ni `git pull`, et ne touche ni l'arbre, ni l'index, ni les branches de la
//! copie principale (test `dry_run_never_merges`). Les deux `git fetch` (`origin` et
//! `pull/<n>/head`) écrivent en revanche dans `.git` : références distantes, `FETCH_HEAD`, objets.
//! Hors de la copie principale, son résultat est au mieux « OK sauf J2 », avec le code 3 (m1).
//! spec: carnet architecture-securite.md SEC0-15, SEC0-16 ; §9.6 G3, G8 ; §9.8 J1, J2, J8
//! Voir aussi l'issue #6, conditions T3-a et T3-b.

use std::collections::BTreeMap;
use std::process::ExitCode;

use serde_json::Value;

use crate::github::{Repo, approved, is_sha, same_path};
use crate::{build_inputs, hooks, sensitive, workflows};

/// Empreinte des entrées de construction, inscrite par `build.rs` (J2, AS-23 point 1).
const BUILT_FROM: &str = env!("XTASK_INPUTS_SHA256");
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
fn main_copy(repo: &Repo) -> Result<bool, String> {
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
        if let Err(e) = binary_check(repo) {
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
    binary_check(repo).map(|()| true)
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
fn run(repo: &Repo, pr: u64) -> Result<bool, String> {
    step(
        "1",
        "copie principale propre, sur main, à jour ; binaire construit depuis main (J2)",
    );
    let j2 = main_copy(repo)?;
    step("2", "hooks verify (verify-main : PR suivante de T3a)");
    println!(
        "  AVERTISSEMENT : contrôle de l'origine de main (verify-main, SEC0-17) pas encore en place."
    );
    let hooks_ok = hooks::verify_or_report(repo)?;
    let suspended = (!hooks_ok).then(|| {
        refuse(
            "fusions suspendues : configuration des crochets non conforme",
            "lever l'écart (issue role:architecture), puis cargo xtask hooks verify",
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
    workflows_at(repo, &sha)?;
    step("10", "approbations sur le SHA de tête (chemin sensible)");
    if hits.is_empty() {
        println!("  sans objet");
    } else {
        let bodies = repo.comments(pr)?;
        for tag in ["review-ok", "archsec-ok"] {
            if !approved(&bodies, tag, &sha) {
                return Err(refuse(
                    format!("chemin sensible : commentaire « {tag}: {sha} » absent"),
                    "relecture neuve puis avis d'archi-sécu sur ce SHA",
                ));
            }
        }
        println!("  review-ok et archsec-ok sur {sha}");
    }
    step("11", "fusion");
    let again = pr_state(repo, pr)?;
    if again["headRefOid"].as_str() != Some(sha.as_str()) {
        return Err(refuse(
            "la tête de la PR a changé pendant les contrôles",
            &format!("relancer cargo xtask merge {pr}"),
        ));
    }
    if let Some(why) = suspended {
        return Err(why);
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
            "state,mergeCommit",
        ]);
        let v: Value = view
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_default();
        match v["state"].as_str() {
            Some("MERGED") => {
                println!(
                    "  AVERTISSEMENT : gh pr merge a signalé une erreur après la fusion ({e})."
                );
                Ok(String::new())
            }
            _ => Err(refuse(
                format!("gh pr merge a échoué ({e})"),
                &format!("gh pr view {pr}"),
            )),
        }
    })?;
    repo.git(&["pull", "--ff-only", "--quiet"])?;
    println!(
        "  PR #{pr} fusionnée ({sha}) ; copie principale : {}",
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
            cache: std::env::temp_dir(),
            home: None,
            runner: Box::new(crate::github::System),
        };
        assert_eq!(head_fingerprint(&repo).unwrap(), BUILT_FROM);
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
                Some((_, out)) => Ok((Some(0), out.clone().into_bytes(), String::new())),
                None => Err(format!("commande inattendue : {line}")),
            }
        }
    }

    /// B4, AS-23 point 2 : en `--dry-run`, `gh pr merge` n'est jamais appelé, et aucune commande
    /// n'écrit dans la copie principale (ni `pull`, ni `config`, ni `checkout`, ni *issue*). Seuls
    /// les `git fetch` écrivent, dans `.git`.
    #[test]
    fn dry_run_never_merges() {
        const SHA: &str = "c41e984ba8594dedd592744fcb69a70f3716cd87";
        const MAIN: &str = "43b3509aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let pr = json!({"number": 7, "state": "OPEN", "baseRefName": "main", "headRefOid": SHA,
            "isDraft": false, "mergeable": "MERGEABLE", "changedFiles": 1,
            "title": "docs(essai): x", "url": "u"});
        let check = json!({"name": "ci-ok", "status": "completed", "conclusion": "success",
            "head_sha": SHA, "check_suite": {"id": 5}});
        let wfr = json!({"workflow_runs": [{"path": ".github/workflows/ci.yml", "workflow_id": 9,
            "head_sha": SHA, "check_suite_id": 5, "repository": {"full_name": "o/r"}}]});
        let answers = [
            ("git branch --show-current", "main".to_owned()),
            ("git status --porcelain", String::new()),
            ("git fetch", String::new()),
            ("git rev-parse", MAIN.to_owned()),
            ("git ls-tree", String::new()),
            ("git cat-file -e", String::new()),
            (
                "git cat-file blob origin/main:tools/dev/tools.lock",
                "gitleaks.path = \"x\"\n".to_owned(),
            ),
            (
                "git cat-file blob origin/main:tools/ci/sensitive-paths.txt",
                include_str!("../../ci/sensitive-paths.txt").to_owned(),
            ),
            ("git worktree list", String::new()),
            ("git merge-base", MAIN.to_owned()),
            ("git diff --name-only", "docs/a.md\0".to_owned()),
            (
                &format!("git ls-tree -r -z --name-only --full-tree {SHA}") as &str,
                "docs/a.md\0".to_owned(),
            ),
            ("gh issue list", "[]".to_owned()),
            ("gh pr view 7", pr.to_string()),
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
        ];
        let calls = Rc::new(RefCell::new(Vec::new()));
        let nowhere = std::env::temp_dir().join("xtask-essai-inexistant");
        let repo = Repo {
            main: nowhere.clone(),
            here: nowhere.clone(),
            slug: "o/r".to_owned(),
            dry_run: true,
            cache: nowhere.join("cache"),
            home: None,
            runner: Box::new(Fake {
                calls: Rc::clone(&calls),
                answers: answers
                    .iter()
                    .map(|(k, v)| ((*k).to_owned(), v.clone()))
                    .collect(),
            }),
        };
        // Crochets non conformes dans ce faux dépôt : suspension, mais le dry-run va jusqu'au bout.
        let result = run(&repo, 7);
        let calls = calls.borrow();
        assert!(
            result.is_err_and(|e| e.contains("fusions suspendues")),
            "{calls:#?}"
        );
        assert_eq!(
            calls
                .iter()
                .filter(|c| c.starts_with("gh pr view 7"))
                .count(),
            2,
            "étape 11 atteinte"
        );
        let writes = [
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
        for c in calls.iter() {
            assert!(
                !writes.iter().any(|w| c.starts_with(w)),
                "écriture en --dry-run : {c}"
            );
            assert!(c.starts_with("gh ") || c.starts_with("git "), "{c}");
        }
        assert!(
            calls
                .iter()
                .any(|c| c == "git fetch --quiet origin pull/7/head")
        );
    }
}
