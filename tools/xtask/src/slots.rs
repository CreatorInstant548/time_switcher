//! `cargo xtask slots` : état des emplacements de compilation, en **lecture seule**. L'attribution
//! reste à `tools/dev/session-start.ps1 -Acquire|-Release` ; ce module en reprend la règle des baux.
//! Il porte aussi les lectures communes avec `disk` : baux, tailles sans suivre les liens, site.
//! spec: developpement-agents.md §6.1, §9.2 ; docs/dev/outillage.md §1

use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{Duration, SystemTime};

use crate::sys;

/// Emplacements de `TS_CACHE\target` : `main` pour la copie principale, `w1` à `w3` prêtés par bail.
pub const SLOTS: [&str; 4] = ["main", "w1", "w2", "w3"];
/// Délai de grâce de `session-start.ps1` : un bail plus jeune est tenu.
pub const GRACE: Duration = Duration::from_secs(10 * 60);
pub const GIB: u64 = 1 << 30;

#[allow(clippy::disallowed_methods)] // outil : âge des baux et des dossiers.
pub fn now() -> SystemTime {
    SystemTime::now()
}

/// Lien symbolique, jonction ou tout autre point de réanalyse : jamais suivi (F8).
pub fn is_link(md: &Metadata) -> bool {
    if md.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        md.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    false
}

/// Taille et dernière modification d'une arborescence, **sans suivre aucun lien** : un lien compte
/// pour zéro et sa date est ignorée.
#[derive(Debug, Default, Clone, Copy)]
pub struct Tree {
    pub bytes: u64,
    pub newest: Option<SystemTime>,
    pub errors: u64,
}

impl Tree {
    /// Inactif depuis plus de `idle` à la date `now`. Une erreur de mesure, ou un arbre sans
    /// date, vaut « actif » : rien n'est alors supprimé.
    pub fn idle_for(&self, now: SystemTime, idle: Duration) -> bool {
        self.errors == 0
            && self
                .newest
                .is_some_and(|t| now.duration_since(t).is_ok_and(|d| d > idle))
    }
}

#[allow(clippy::disallowed_methods)] // outil : xtask parcourt TS_CACHE et les worktrees.
pub fn measure(path: &Path) -> Tree {
    let mut tree = Tree::default();
    let mut stack = vec![path.to_path_buf()];
    while let Some(p) = stack.pop() {
        let Ok(md) = fs::symlink_metadata(&p) else {
            tree.errors += 1;
            continue;
        };
        if is_link(&md) {
            continue;
        }
        if let Ok(t) = md.modified() {
            tree.newest = Some(tree.newest.map_or(t, |n| n.max(t)));
        }
        if md.is_dir() {
            match fs::read_dir(&p) {
                Ok(entries) => stack.extend(entries.filter_map(|e| e.ok().map(|e| e.path()))),
                Err(_) => tree.errors += 1,
            }
        } else {
            tree.bytes += md.len();
        }
    }
    tree
}

/// Bail `wN.lease` : chemin du worktree propriétaire et date de dernière écriture.
#[derive(Debug)]
pub struct Lease {
    pub owner: String,
    pub modified: SystemTime,
}

pub fn read_lease(path: &Path) -> Option<Lease> {
    let md = fs::symlink_metadata(path).ok()?;
    let owner = String::from_utf8_lossy(&sys::read(path).ok()?)
        .trim_start_matches('\u{feff}')
        .trim()
        .to_owned();
    Some(Lease {
        owner,
        modified: md.modified().ok()?,
    })
}

/// Règle de `Test-LeaseHeld` : tenu s'il a moins de 10 minutes, ou si son worktree existe encore
/// (fichier ou dossier `.git`). Un bail daté du futur est tenu.
pub fn lease_held(lease: &Lease, now: SystemTime) -> bool {
    let young = now
        .duration_since(lease.modified)
        .map_or(true, |age| age < GRACE);
    let owner = Path::new(&lease.owner);
    young || (owner.is_absolute() && fs::symlink_metadata(owner.join(".git")).is_ok())
}

/// Ce que l'outil sait du dépôt et de `TS_CACHE`.
#[derive(Debug)]
pub struct Site {
    pub cache: PathBuf,
    /// Copie de travail d'où `xtask` est lancé (commandes Git).
    pub repo: PathBuf,
    /// Copie principale (parent du dossier Git commun).
    pub main_root: PathBuf,
    /// Racine des worktrees de l'application : `.claude/worktrees` de la copie principale.
    pub worktrees_root: PathBuf,
    /// Chemins rendus par `git worktree list`.
    pub worktrees: Vec<PathBuf>,
}

impl Site {
    pub fn load(cache: PathBuf, cwd: &Path) -> Result<Self, String> {
        let repo =
            PathBuf::from(sys::stdout_of(cwd, "git", &["rev-parse", "--show-toplevel"])?.trim());
        let common = sys::stdout_of(
            &repo,
            "git",
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?;
        let main = Path::new(common.trim())
            .parent()
            .ok_or("dossier Git commun sans parent")?
            .to_path_buf();
        let mut site = Self {
            cache,
            repo,
            worktrees_root: main.join(".claude").join("worktrees"),
            main_root: main,
            worktrees: Vec::new(),
        };
        site.reload_worktrees()?;
        Ok(site)
    }

    pub fn reload_worktrees(&mut self) -> Result<(), String> {
        let out = sys::stdout_of(
            &self.repo,
            "git",
            &["worktree", "list", "--porcelain", "-z"],
        )?;
        self.worktrees = parse_worktrees(&out);
        Ok(())
    }

    pub fn slot(&self, name: &str) -> PathBuf {
        self.cache.join("target").join(name)
    }

    /// Worktrees dont la jonction `target` mène à cet emplacement.
    pub fn junctions_to(&self, slot: &Path) -> Vec<&Path> {
        let Ok(slot) = fs::canonicalize(slot) else {
            return Vec::new();
        };
        self.worktrees
            .iter()
            .filter(|wt| {
                let link = wt.join("target");
                fs::symlink_metadata(&link).is_ok_and(|md| is_link(&md))
                    && fs::canonicalize(&link).is_ok_and(|t| t == slot)
            })
            .map(PathBuf::as_path)
            .collect()
    }

    pub fn is_listed(&self, dir: &Path) -> bool {
        self.worktrees.iter().any(|wt| same_dir(wt, dir))
    }

    /// `xtask` est lancé depuis la copie principale.
    pub fn run_from_main(&self) -> bool {
        same_dir(&self.repo, &self.main_root)
    }
}

fn same_dir(a: &Path, b: &Path) -> bool {
    let canon = fs::canonicalize(a).ok();
    (canon.is_some() && canon == fs::canonicalize(b).ok()) || same_text(a, b)
}

fn same_text(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| {
        p.to_string_lossy()
            .replace('\\', "/")
            .trim_end_matches('/')
            .to_ascii_lowercase()
    };
    norm(a) == norm(b)
}

/// Sortie de `git worktree list --porcelain -z` : un champ par NUL, `worktree <chemin>` d'abord.
pub fn parse_worktrees(out: &str) -> Vec<PathBuf> {
    out.split('\0')
        .filter_map(|f| f.strip_prefix("worktree "))
        .map(PathBuf::from)
        .collect()
}

/// État d'un emplacement, tel que l'affichent `slots` et `disk`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// `main` : réservé à la copie principale, sans bail.
    Main,
    Free,
    Held,
    /// Bail présent mais ni récent ni rattaché à un worktree existant.
    Orphan,
}

pub fn state(site: &Site, name: &str, now: SystemTime) -> (State, Option<Lease>) {
    if name == "main" {
        return (State::Main, None);
    }
    match read_lease(&site.cache.join("target").join(format!("{name}.lease"))) {
        None => (State::Free, None),
        Some(l) if lease_held(&l, now) => (State::Held, Some(l)),
        Some(l) => (State::Orphan, Some(l)),
    }
}

pub fn gib(bytes: u64) -> String {
    #[allow(clippy::cast_precision_loss)] // affichage seulement
    let v = bytes as f64 / GIB as f64;
    format!("{v:.1} Go").replace('.', ",")
}

/// Espace libre du disque qui porte `path`, par l'outil du système (aucune crate ajoutée).
pub fn free_bytes(path: &Path) -> Result<u64, String> {
    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("pwsh");
        // Le chemin passe par l'environnement, jamais dans le texte de la commande.
        c.args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "[IO.DriveInfo]::new([IO.Path]::GetPathRoot([IO.Path]::GetFullPath($env:XTASK_FREE_PATH))).AvailableFreeSpace",
        ])
        .env("XTASK_FREE_PATH", path);
        c
    } else {
        let mut c = Command::new("df");
        c.arg("-Pk").arg(path);
        c
    };
    let out = cmd
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("espace libre illisible : {e}"))?;
    let text = String::from_utf8_lossy(&out.stdout);
    let value = if cfg!(windows) {
        text.trim().parse::<u64>().ok()
    } else {
        text.lines()
            .nth(1)
            .and_then(|l| l.split_whitespace().nth(3))
            .and_then(|k| k.parse::<u64>().ok())
            .map(|k| k * 1024)
    };
    value
        .filter(|_| out.status.success())
        .ok_or_else(|| "espace libre illisible".to_owned())
}

/// Tableau des emplacements, commun à `slots` et `disk` ; rend leurs tailles, dans l'ordre de `SLOTS`.
pub fn print_table(site: &Site, now: SystemTime) -> Vec<u64> {
    let mut sizes = Vec::new();
    println!(
        "Emplacements de compilation ({}) :",
        show(&site.cache.join("target"))
    );
    for name in SLOTS {
        let slot = site.slot(name);
        let (st, lease) = state(site, name, now);
        let label = match st {
            State::Main => "principal",
            State::Free => "libre",
            State::Held => "tenu",
            State::Orphan => "orphelin",
        };
        let bytes = measure(&slot).bytes;
        sizes.push(bytes);
        let size = match fs::symlink_metadata(&slot) {
            Err(_) => "absent".to_owned(),
            Ok(md) if is_link(&md) => "LIEN, non suivi".to_owned(),
            Ok(_) => gib(bytes),
        };
        let owner = match (st, &lease) {
            (State::Main, _) => "copie principale".to_owned(),
            (_, Some(l)) => l.owner.clone(),
            (_, None) => "aucun".to_owned(),
        };
        let links = site.junctions_to(&slot);
        let junction = match links.as_slice() {
            [] => "aucune".to_owned(),
            [one]
                if lease
                    .as_ref()
                    .is_some_and(|l| same_text(one, Path::new(&l.owner))) =>
            {
                "celle du bail".to_owned()
            }
            _ => links.iter().map(|p| show(p)).collect::<Vec<_>>().join(", "),
        };
        println!("  {name:<4} {label:<9} {size:>8}   bail : {owner} ; jonction : {junction}");
    }
    sizes
}

/// Chemin affiché avec le séparateur du système.
pub fn show(p: &Path) -> String {
    let s = p.display().to_string();
    if cfg!(windows) {
        s.replace('/', "\\")
    } else {
        s
    }
}

pub fn print_free(site: &Site) {
    match free_bytes(&site.cache) {
        Ok(b) if b < 40 * GIB => {
            println!("Espace libre : {} (AVERTISSEMENT : moins de 40 Go)", gib(b));
        }
        Ok(b) => println!("Espace libre : {}", gib(b)),
        Err(e) => println!("Espace libre : {e}"),
    }
}

pub fn main(args: &[String]) -> ExitCode {
    if !args.is_empty() {
        eprintln!(
            "usage : cargo xtask slots (lecture seule ; attribution par tools/dev/session-start.ps1)"
        );
        return ExitCode::from(2);
    }
    let site = match Site::load(
        sys::ts_cache(),
        &std::env::current_dir().unwrap_or_default(),
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ÉCHEC : {e}");
            return ExitCode::FAILURE;
        }
    };
    print_table(&site, now());
    print_free(&site);
    println!(
        "Attribution : pwsh -NoProfile -File tools/dev/session-start.ps1 -Acquire ; fin de tâche : -Release."
    );
    ExitCode::SUCCESS
}

/// Dossier des essais : `TS_CACHE\tmp` sous Windows, le dossier temporaire du système ailleurs,
/// où le `TS_CACHE` par défaut n'est pas un chemin absolu.
#[cfg(test)]
pub fn scratch_root() -> PathBuf {
    if cfg!(windows) {
        sys::ts_cache().join("tmp")
    } else {
        std::env::temp_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_worktree_list() {
        let out = "worktree C:/a\0HEAD 1\0branch refs/heads/main\0\0worktree C:/b c\0detached\0\0";
        assert_eq!(
            parse_worktrees(out),
            [PathBuf::from("C:/a"), PathBuf::from("C:/b c")]
        );
    }

    #[test]
    fn lease_rule_matches_session_start() {
        let t0 = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000);
        let lease = |owner: &str| Lease {
            owner: owner.to_owned(),
            modified: t0,
        };
        // Récent : tenu, quel que soit le propriétaire.
        assert!(lease_held(
            &lease("nulle-part"),
            t0 + Duration::from_secs(60)
        ));
        // Daté du futur : tenu.
        assert!(lease_held(
            &lease("nulle-part"),
            t0 - Duration::from_secs(60)
        ));
        // Ancien, propriétaire sans .git ou relatif : orphelin.
        assert!(!lease_held(&lease("nulle-part"), t0 + GRACE + GRACE));
        assert!(!lease_held(&lease(""), t0 + GRACE + GRACE));
        // Ancien, propriétaire existant avec .git : tenu.
        let dir = scratch_root().join(format!("xtask-lease-{}", std::process::id()));
        fs::create_dir_all(dir.join(".git")).unwrap();
        assert!(lease_held(
            &lease(&dir.to_string_lossy()),
            t0 + GRACE + GRACE
        ));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn idle_needs_old_tree() {
        let t0 = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000);
        let day = Duration::from_secs(24 * 3600);
        let tree = Tree {
            newest: Some(t0),
            ..Tree::default()
        };
        assert!(!tree.idle_for(t0 + day, day));
        assert!(tree.idle_for(t0 + day + Duration::from_secs(1), day));
        assert!(!tree.idle_for(t0 - day, day));
        // Erreur de mesure ou arbre sans date : actif.
        let unreadable = Tree { errors: 1, ..tree };
        assert!(!unreadable.idle_for(t0 + day + day, day));
        assert!(!Tree::default().idle_for(t0, day));
    }

    #[test]
    fn formats_gib() {
        assert_eq!(gib(3 * GIB / 2), "1,5 Go");
    }
}
