//! `cargo xtask hooks install|verify [--dry-run]` : crochets Git épinglés hors du dépôt
//! (`TS_CACHE\hooks\<empreinte>\`) et intégrité de la configuration de Claude Code. Remplace
//! `tools/dev/install-hooks.ps1`, dont il garde le calcul d'empreinte (J7).
//! spec: carnet architecture-securite.md SEC0-06, SEC0-12, SEC0-35 ; §9.8 J6, J7
//! Voir aussi l'issue #6, point 1.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::Value;

use crate::github::{Repo, same_path};
use crate::{sha256, sys};

const HOOKS: &str = "tools/dev/hooks";
const LOCK: &str = "tools/dev/tools.lock";
const LOCK_NAME: &str = "tools.lock";

/// Jeu de crochets : (nom, contenu), trié par nom.
type Set = Vec<(String, Vec<u8>)>;

/// Empreinte du jeu, comme `install-hooks.ps1` : SHA-256 de la liste « nom NUL sha256 LF »,
/// triée par nom en ordre ordinal (noms ASCII seulement : ordre UTF-16 et ordre des octets égaux).
pub fn set_hash(set: &Set) -> Result<String, String> {
    let mut names: Vec<&(String, Vec<u8>)> = set.iter().collect();
    names.sort_by(|a, b| a.0.cmp(&b.0));
    let mut list = Vec::new();
    for (name, bytes) in names {
        if !name.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(format!(
                "nom de crochet non ASCII ou avec espace : « {name} »"
            ));
        }
        list.extend_from_slice(format!("{name}\0{}\n", sha256::hex(bytes)).as_bytes());
    }
    Ok(sha256::hex(&list))
}

/// Jeu attendu, lu dans `origin/main` par Git, jamais dans la copie de travail (SEC0-35, J7).
fn origin_set(repo: &Repo) -> Result<Set, String> {
    let ls = repo.git(&[
        "ls-tree",
        "-z",
        "--full-tree",
        "origin/main",
        &format!("{HOOKS}/"),
    ])?;
    let mut set = Set::new();
    for entry in ls.split('\0').filter(|e| !e.is_empty()) {
        let (meta, path) = entry
            .split_once('\t')
            .ok_or("sortie de git ls-tree illisible")?;
        let name = path.strip_prefix(&format!("{HOOKS}/")).unwrap_or(path);
        if !(meta.starts_with("100644 blob ") || meta.starts_with("100755 blob "))
            || name.contains('/')
            || name == LOCK_NAME
        {
            return Err(format!(
                "{path} : entrée inattendue dans {HOOKS} d'origin/main ({meta})"
            ));
        }
        let bytes = repo
            .blob(&repo.main, "origin/main", path)?
            .ok_or("crochet disparu")?;
        set.push((name.to_owned(), bytes));
    }
    let lock = repo
        .blob(&repo.main, "origin/main", LOCK)?
        .ok_or("tools.lock absent d'origin/main")?;
    set.push((LOCK_NAME.to_owned(), lock));
    set.sort();
    Ok(set)
}

/// Fichiers d'un dossier, plus `tools.lock` pris à part s'il est donné, lus sur le disque. Un
/// lien, un dossier ou un nom en double est une erreur.
fn disk_set(dir: &Path, lock: Option<&Path>) -> Result<Set, String> {
    let mut set = Set::new();
    for (name, file) in sys::entries(dir)? {
        if !file {
            return Err(format!(
                "{} : entrée autre qu'un fichier ordinaire",
                dir.join(&name).display()
            ));
        }
        set.push((name.clone(), sys::read(&dir.join(&name))?));
    }
    if let Some(lock) = lock {
        set.push((LOCK_NAME.to_owned(), sys::read(lock)?));
    }
    set.sort();
    if set.windows(2).any(|w| w[0].0 == w[1].0) {
        return Err(format!("{} : {LOCK_NAME} en double", dir.display()));
    }
    Ok(set)
}

/// Écarts entre deux jeux, fichier par fichier, octet pour octet.
fn diff(want: &Set, got: &Set, what: &str) -> Vec<String> {
    let names = |s: &Set| {
        s.iter()
            .map(|e| e.0.clone())
            .collect::<std::collections::BTreeSet<_>>()
    };
    let mut out = Vec::new();
    for n in names(want).union(&names(got)) {
        let (w, g) = (
            want.iter().find(|e| &e.0 == n),
            got.iter().find(|e| &e.0 == n),
        );
        if w.map(|e| &e.1) != g.map(|e| &e.1) {
            out.push(format!("{what} : {n} diffère d'origin/main"));
        }
    }
    out
}

fn lock_value(set: &Set, key: &str) -> Result<String, String> {
    let lock = set
        .iter()
        .find(|e| e.0 == LOCK_NAME)
        .map(|e| String::from_utf8_lossy(&e.1).into_owned());
    lock.unwrap_or_default()
        .lines()
        .filter_map(|l| l.split_once('='))
        .find(|(k, _)| k.trim() == key)
        .map(|(_, v)| v.trim().trim_matches('"').to_owned())
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("{key} absent de tools.lock"))
}

/// gitleaks présent et d'empreinte attendue (SEC0-06).
fn gitleaks(repo: &Repo, set: &Set) -> Result<(), String> {
    let path = PathBuf::from(lock_value(set, "gitleaks.path")?);
    let want = lock_value(set, "gitleaks.sha256")?.to_ascii_lowercase();
    if !path.is_file() || sys::sha256(&repo.main, &path)? != want {
        return Err(format!(
            "gitleaks absent ou d'empreinte inattendue ({})",
            path.display()
        ));
    }
    Ok(())
}

fn pinned_root() -> PathBuf {
    sys::ts_cache().join("hooks")
}

pub fn install(repo: &Repo) -> Result<String, String> {
    if !same_path(&repo.here, &repo.main) {
        return Err(format!(
            "hooks install se lance depuis la copie principale ({}), pas depuis {}",
            repo.main.display(),
            repo.here.display()
        ));
    }
    repo.git(&["fetch", "--quiet", "origin"])?;
    let want = origin_set(repo)?;
    let errors = diff(
        &want,
        &disk_set(&repo.main.join(HOOKS), Some(&repo.main.join(LOCK)))?,
        "copie principale",
    );
    if !errors.is_empty() {
        return Err(format!(
            "{} ; mettre la copie principale à jour (git pull --ff-only)",
            errors.join(" ; ")
        ));
    }
    gitleaks(repo, &want)?;
    let hash = set_hash(&want)?;
    let root = pinned_root();
    let dest = root.join(&hash);
    if dest.exists() {
        let errors = diff(&want, &disk_set(&dest, None)?, &dest.display().to_string());
        if !errors.is_empty() {
            return Err(format!(
                "{} existe avec un contenu différent : à examiner, rien n'est remplacé",
                dest.display()
            ));
        }
    } else {
        let stage = root.join(format!("staging-{}", std::process::id()));
        std::fs::create_dir_all(&stage).map_err(|e| format!("{} : {e}", stage.display()))?;
        for (name, bytes) in &want {
            let f = stage.join(name);
            sys::write(&f, bytes)?;
            let mut perm = std::fs::metadata(&f)
                .map_err(|e| e.to_string())?
                .permissions();
            perm.set_readonly(true);
            std::fs::set_permissions(&f, perm).map_err(|e| format!("{} : {e}", f.display()))?;
        }
        std::fs::rename(&stage, &dest).map_err(|e| format!("{} : {e}", dest.display()))?;
    }
    let value = dest.to_string_lossy().replace('\\', "/");
    repo.git(&["config", "--local", "core.hooksPath", &value])?;
    sys::write(&root.join("current.sha256"), hash.as_bytes())?;
    Ok(format!(
        "crochets installés : {value} (core.hooksPath de la configuration commune)"
    ))
}

/// Réglages locaux : ni `disableAllHooks`, ni crochets (SEC0-35 point 3).
pub fn local_settings_errors(text: &str) -> Vec<String> {
    let v: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => return vec![format!("JSON illisible ({e})")],
    };
    let mut out = Vec::new();
    if text.to_ascii_lowercase().contains("disableallhooks") {
        out.push("mentionne disableAllHooks".to_owned());
    }
    if v.get("hooks")
        .is_some_and(|h| !h.is_null() && h != &Value::Object(Default::default()))
    {
        out.push("déclare des crochets (hooks)".to_owned());
    }
    out
}

/// Réglages utilisateur, lus sans jamais être écrits : `disableAllHooks` doit être absent ou faux (J6).
pub fn user_settings_errors(text: &str) -> Vec<String> {
    match serde_json::from_str::<Value>(text) {
        Err(e) => vec![format!("JSON illisible ({e})")],
        Ok(v) => match v.get("disableAllHooks") {
            None | Some(Value::Bool(false)) => Vec::new(),
            Some(x) => vec![format!("disableAllHooks vaut {x}")],
        },
    }
}

/// Une copie de travail : `core.hooksPath` effectif et réglages de Claude Code.
fn check_worktree(
    repo: &Repo,
    wt: &Path,
    pinned: &Path,
    common_cfg: &Path,
    errors: &mut Vec<String>,
) -> Result<(), String> {
    let w = wt.display();
    let lines = sys::stdout_of(
        wt,
        "git",
        &["config", "--show-origin", "--get-all", "core.hooksPath"],
    )
    .unwrap_or_default();
    let lines: Vec<&str> = lines.lines().collect();
    match lines.as_slice() {
        [one] => {
            let (origin, value) = one.split_once('\t').unwrap_or((one, ""));
            let file = origin.strip_prefix("file:").map(|f| wt.join(f));
            if !file.is_some_and(|f| same_path(&f, common_cfg)) {
                errors.push(format!(
                    "{w} : core.hooksPath vient de {origin}, attendu la configuration commune"
                ));
            }
            if !same_path(Path::new(value), pinned) {
                errors.push(format!(
                    "{w} : core.hooksPath = « {value} », attendu {}",
                    pinned.display()
                ));
            }
        }
        _ => errors.push(format!(
            "{w} : {} valeur(s) de core.hooksPath, attendu une seule",
            lines.len()
        )),
    }
    let settings = ".claude/settings.json";
    let disk = wt.join(settings);
    let disk = disk.is_file().then(|| sys::read(&disk)).transpose()?;
    if disk != repo.blob(wt, "HEAD", settings)? {
        errors.push(format!("{w} : {settings} diffère de celui de HEAD"));
    }
    if same_path(wt, &repo.main) && disk != repo.blob(&repo.main, "origin/main", settings)? {
        errors.push(format!("{w} : {settings} diffère de celui d'origin/main"));
    }
    let local = wt.join(".claude/settings.local.json");
    if local.exists() {
        println!("  {w} : .claude/settings.local.json existe (signalé, SEC0-35)");
        let text = sys::read_text(&local)?;
        errors.extend(
            local_settings_errors(&text)
                .into_iter()
                .map(|e| format!("{w} : settings.local.json {e}")),
        );
    }
    Ok(())
}

/// Empreinte attendue et écarts constatés ; `Err` si un contrôle n'a pas pu se faire.
pub fn verify(repo: &Repo) -> Result<(String, Vec<String>), String> {
    let want = origin_set(repo)?;
    let hash = set_hash(&want)?;
    let pinned = pinned_root().join(&hash);
    let mut errors = Vec::new();
    if pinned.is_dir() {
        errors.extend(diff(
            &want,
            &disk_set(&pinned, None)?,
            &pinned.display().to_string(),
        ));
    } else {
        errors.push(format!(
            "copie épinglée absente : {} (cargo xtask hooks install)",
            pinned.display()
        ));
    }
    let current = sys::read_text(&pinned_root().join("current.sha256")).unwrap_or_default();
    if current.trim() != hash {
        errors.push(format!(
            "current.sha256 vaut « {} », attendu {hash}",
            current.trim()
        ));
    }
    if let Err(e) = gitleaks(repo, &want) {
        errors.push(e);
    }
    let common = repo.git(&["rev-parse", "--path-format=absolute", "--git-common-dir"])?;
    let common_cfg = Path::new(&common).join("config");
    let list = repo.git(&["worktree", "list", "--porcelain"])?;
    let mut n = 0;
    for wt in list.lines().filter_map(|l| l.strip_prefix("worktree ")) {
        let wt = Path::new(wt);
        if !wt.is_dir() {
            println!(
                "  {} : dossier absent, ignoré (git worktree prune)",
                wt.display()
            );
            continue;
        }
        check_worktree(repo, wt, &pinned, &common_cfg, &mut errors)?;
        n += 1;
    }
    match sys::home().map(|h| h.join(".claude").join("settings.json")) {
        Some(user) if user.is_file() => {
            let text = sys::read_text(&user)?;
            errors.extend(
                user_settings_errors(&text)
                    .into_iter()
                    .map(|e| format!("réglages utilisateur : {e}")),
            );
        }
        Some(_) => {}
        None => {
            errors.push("dossier personnel introuvable : réglages utilisateur non lus".to_owned())
        }
    }
    println!(
        "  hooks verify : {hash}, {n} copie(s) de travail, {} écart(s)",
        errors.len()
    );
    Ok((hash, errors))
}

/// `hooks verify` avec *issue* en cas d'écart ; vrai si tout est conforme.
pub fn verify_or_report(repo: &Repo) -> Result<bool, String> {
    let (hash, errors) = verify(repo)?;
    if errors.is_empty() {
        return Ok(true);
    }
    errors.iter().for_each(|e| println!("  - {e}"));
    // Une issue par jeu attendu : les écarts d'un même jeu s'y rattachent.
    let key = format!("hooks-verify:{hash}");
    let body = format!(
        "`cargo xtask hooks verify` a relevé des écarts, et les fusions sont suspendues :\n\n- {}",
        errors.join("\n- ")
    );
    repo.open_issue(
        &key,
        "hooks verify : configuration des crochets non conforme",
        &body,
    )?;
    Ok(false)
}

pub fn main(args: &[String]) -> ExitCode {
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let action = args.first().map(String::as_str);
    if args.iter().skip(1).any(|a| a != "--dry-run")
        || !matches!(action, Some("install" | "verify"))
        || (dry_run && action == Some("install"))
    {
        eprintln!("usage : cargo xtask hooks install | cargo xtask hooks verify [--dry-run]");
        return ExitCode::from(2);
    }
    let result = Repo::open(dry_run, false).and_then(|repo| match action {
        Some("install") => install(&repo).map(|m| println!("{m}")),
        _ => repo
            .git(&["fetch", "--quiet", "origin"])
            .and_then(|_| verify_or_report(&repo))
            .and_then(|ok| ok.then_some(()).ok_or_else(|| "écarts relevés".to_owned())),
    });
    match result {
        Ok(()) => {
            println!("Résultat : OK");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ÉCHEC : {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_hash_matches_install_hooks() {
        // Valeur calculée par Get-SetHash de install-hooks.ps1 sur ce jeu (PowerShell 7, 30/10).
        let set: Set = vec![
            ("tools.lock".into(), b"x = \"y\"\n".to_vec()),
            ("b".into(), b"2\n".to_vec()),
            ("a".into(), b"1\n".to_vec()),
        ];
        assert_eq!(
            set_hash(&set).unwrap(),
            "a0e8023fea2d7841c2c50c6b1e5842b0800a9ce3c1226912f472bacb3c4f02d8"
        );
        assert!(set_hash(&vec![("é".into(), vec![])]).is_err());
        let set: Set = vec![("a".into(), b"1\n".to_vec()), ("b".into(), b"2\n".to_vec())];
        let changed: Set = vec![
            ("a".into(), b"1\r\n".to_vec()),
            ("b".into(), b"2\n".to_vec()),
        ];
        assert_eq!(diff(&set, &changed, "x").len(), 1);
        assert_eq!(diff(&set, &set[..1].to_vec(), "x").len(), 1);
    }

    #[test]
    fn settings_checks() {
        assert!(local_settings_errors("{\"permissions\":{}}").is_empty());
        assert!(local_settings_errors("{\"hooks\":{}}").is_empty());
        assert_eq!(
            local_settings_errors("{\"disableAllHooks\":false}").len(),
            1
        );
        assert_eq!(
            local_settings_errors("{\"hooks\":{\"PreToolUse\":[]}}").len(),
            1
        );
        assert_eq!(local_settings_errors("{").len(), 1);
        assert!(user_settings_errors("{\"disableAllHooks\":false,\"model\":\"x\"}").is_empty());
        assert_eq!(user_settings_errors("{\"disableAllHooks\":true}").len(), 1);
        assert_eq!(
            user_settings_errors("{\"disableAllHooks\":\"true\"}").len(),
            1
        );
        assert_eq!(user_settings_errors("pas du JSON").len(), 1);
    }
}
