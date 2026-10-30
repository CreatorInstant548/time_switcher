//! `cargo xtask disk [--clean]` : tailles et nettoyage de `TS_CACHE` et des worktrees. Sans `--clean`,
//! rien n'est modifié : chaque action est seulement annoncée.
//! spec: developpement-agents.md §9.2 ; carnet architecture-securite.md §9.5 F8
//!
//! Garanties (F8) :
//! - aucun lien symbolique, aucune jonction ni aucun point de réanalyse n'est suivi : le lien
//!   lui-même est retiré, jamais sa cible (`Remover::tree`) ;
//! - toute suppression vise un chemin sous `TS_CACHE` (`target` et `artifacts\runs` seulement) ou
//!   sous `.claude/worktrees` de la copie principale, après forme canonique du dossier parent et
//!   contrôle du préfixe (`within`) ;
//! - un emplacement tenu n'est jamais touché ; avant de vider un emplacement libre, son bail est
//!   pris par création exclusive, comme dans `session-start.ps1`, et les verrous de Cargo aussi ;
//!   `main` n'est nettoyé que depuis la copie principale ;
//! - un dossier de worktree n'est retiré que s'il est absent de `git worktree list`, sans `.git`,
//!   et inactif depuis plus de 24 h.
//!
//! Limite connue : entre le contrôle d'une entrée et sa lecture, un tiers qui écrit dans
//! `TS_CACHE` pourrait la remplacer par une jonction. Cela suppose déjà la machine compromise.

use std::cell::Cell;
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, SystemTime};

use crate::slots::{self, GIB, SLOTS, Site, State, gib, is_link, measure, now};
use crate::sys;

/// Seuils d'un emplacement : `incremental` supprimé au-delà du premier, emplacement vidé au-delà
/// du second (§9.2, [à calibrer]).
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub incremental: u64,
    pub empty: u64,
}
pub const FREE_SLOT: Limits = Limits {
    incremental: 6 * GIB,
    empty: 10 * GIB,
};
pub const MAIN_SLOT: Limits = Limits {
    incremental: 12 * GIB,
    empty: 20 * GIB,
};
const RUNS_MAX_AGE: Duration = Duration::from_secs(30 * 24 * 3600);
const ORPHAN_IDLE: Duration = Duration::from_secs(24 * 3600);
/// Une exécution qui contient ce fichier n'est jamais supprimée (§9.2 point 6).
const KEEP_MARK: &str = "keep";
/// Le bail pris par `disk` est rafraîchi à ce rythme, bien en deçà des 10 min de grâce.
const REFRESH_EVERY: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotAction {
    Nothing,
    Incremental,
    Empty,
}

pub fn slot_action(bytes: u64, limits: Limits) -> SlotAction {
    if bytes > limits.empty {
        SlotAction::Empty
    } else if bytes > limits.incremental {
        SlotAction::Incremental
    } else {
        SlotAction::Nothing
    }
}

/// Racine permise `base\rel…` : aucun des composants de `rel` n'est un lien (sinon le nettoyage
/// irait dans sa cible), forme canonique, et au moins deux composants sous la racine du disque.
pub fn root(base: &Path, rel: &[&str]) -> Option<PathBuf> {
    let mut path = base.to_path_buf();
    for part in rel {
        path.push(part);
        if is_link(&fs::symlink_metadata(&path).ok()?) {
            return None;
        }
    }
    let canon = fs::canonicalize(&path).ok()?;
    let depth = canon
        .components()
        .filter(|c| matches!(c, Component::Normal(_)))
        .count();
    (depth >= 2).then_some(canon)
}

/// Forme canonique de `path` (dossier parent résolu, nom final gardé tel quel, donc un lien final
/// n'est pas suivi), si elle est strictement sous l'une des racines canoniques.
pub fn within(path: &Path, roots: &[PathBuf]) -> Result<PathBuf, String> {
    let refuse = || {
        format!(
            "{} : hors de TS_CACHE et de .claude/worktrees",
            path.display()
        )
    };
    let name = match path.components().next_back() {
        Some(Component::Normal(n)) => n,
        _ => return Err(refuse()),
    };
    let parent = fs::canonicalize(path.parent().ok_or_else(refuse)?).map_err(|_| refuse())?;
    let candidate = parent.join(name);
    if roots
        .iter()
        .any(|r| candidate.starts_with(r) && candidate != *r)
    {
        Ok(candidate)
    } else {
        Err(refuse())
    }
}

/// Bail pris par `disk`, par création exclusive : un seul gagnant, comme `New-Lease`.
#[derive(Debug)]
pub struct Taken {
    path: PathBuf,
    file: File,
    token: String,
    refreshed: Cell<SystemTime>,
}

impl Taken {
    #[allow(clippy::disallowed_methods)] // outil : création exclusive du bail (F8).
    pub fn take(path: &Path) -> Result<Option<Self>, String> {
        let stamp = now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        // Pas un chemin : sans `.git`, ce bail redevient orphelin 10 min après un arrêt brutal.
        let token = format!("xtask-disk {} {stamp}", std::process::id());
        match OpenOptions::new().write(true).create_new(true).open(path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(token.as_bytes()) {
                    drop(file);
                    let _ = fs::remove_file(path); // jamais de bail vide laissé derrière
                    return Err(format!("bail non écrit : {e}"));
                }
                Ok(Some(Self {
                    path: path.to_path_buf(),
                    file,
                    token,
                    refreshed: Cell::new(now()),
                }))
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(None),
            Err(e) => Err(format!("bail impossible à créer : {e}")),
        }
    }

    /// Rafraîchit le bail s'il a plus d'une minute. Un échec le laisse tel quel : au pire, il
    /// redevient orphelin après 10 min.
    fn refresh(&self) {
        let t = now();
        if t.duration_since(self.refreshed.get())
            .is_ok_and(|d| d >= REFRESH_EVERY)
        {
            let _ = self.file.set_modified(t);
            self.refreshed.set(t);
        }
    }

    pub fn release(self) -> Result<(), String> {
        let Self {
            path, file, token, ..
        } = self;
        drop(file);
        match slots::read_lease(&path) {
            Some(l) if l.owner == token => {
                fs::remove_file(&path).map_err(|e| format!("bail non rendu : {e}"))
            }
            _ => Err(format!("{} a changé : laissé en place", path.display())),
        }
    }
}

/// Libère un bail orphelin par renommage atomique (un seul gagnant), puis le supprime, comme
/// `Remove-OrphanLease`. Rend faux si le bail est redevenu tenu entre-temps : il est alors rendu,
/// sans jamais écraser un bail pris dans l'intervalle (lien physique, qui échoue si le nom existe).
fn release_orphan(lease: &Path, now: SystemTime) -> Result<bool, String> {
    let tmp = lease.with_extension(format!("lease.stale-{}", std::process::id()));
    fs::rename(lease, &tmp).map_err(|e| format!("bail orphelin non libéré : {e}"))?;
    if slots::read_lease(&tmp).is_some_and(|l| slots::lease_held(&l, now)) {
        let back = fs::hard_link(&tmp, lease);
        fs::remove_file(&tmp).map_err(|e| format!("bail redevenu tenu, copie laissée : {e}"))?;
        back.map_err(|e| format!("bail redevenu tenu, non rendu : {e}"))?;
        return Ok(false);
    }
    fs::remove_file(&tmp).map_err(|e| format!("bail orphelin non supprimé : {e}"))?;
    Ok(true)
}

/// Verrous `.cargo-*lock` d'un emplacement (profondeur 3 au plus, sans suivre les liens), pris en
/// exclusif : une compilation en cours fait refuser l'emplacement, et aucune ne démarre pendant le
/// nettoyage. Les fichiers de verrou restent en place.
#[allow(clippy::disallowed_methods)] // outil : verrous de Cargo d'un emplacement (F8).
pub fn cargo_locks(slot: &Path) -> Result<Vec<(PathBuf, File)>, String> {
    let mut held = Vec::new();
    let mut stack = vec![(slot.to_path_buf(), 0)];
    while let Some((dir, depth)) = stack.pop() {
        let entries =
            fs::read_dir(&dir).map_err(|e| format!("{} illisible : {e}", dir.display()))?;
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(md) = fs::symlink_metadata(&path) else {
                continue;
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            if is_link(&md) {
                continue;
            } else if md.is_dir() && depth < 2 {
                stack.push((path, depth + 1));
            } else if md.is_file() && name.starts_with(".cargo-") && name.ends_with("lock") {
                let file = OpenOptions::new().read(true).write(true).open(&path);
                match file.map(|f| f.try_lock().map(|()| f)) {
                    Ok(Ok(f)) => held.push((path, f)),
                    _ => return Err("compilation en cours (verrou de Cargo tenu)".to_owned()),
                }
            }
        }
    }
    Ok(held)
}

/// Suppression qui ne suit jamais un lien (F8).
#[derive(Debug, Default)]
struct Remover<'a> {
    keep: Vec<PathBuf>,
    lease: Option<&'a Taken>,
    errors: Vec<String>,
}

impl Remover<'_> {
    fn fail(&mut self, path: &Path, e: &std::io::Error) -> bool {
        self.errors.push(format!("{} : {e}", path.display()));
        false
    }

    /// Rend vrai si `path` a disparu.
    #[allow(clippy::disallowed_methods)] // outil : read_dir pour vider sans suivre les liens.
    fn tree(&mut self, path: &Path) -> bool {
        if let Some(l) = self.lease {
            l.refresh();
        }
        let md = match fs::symlink_metadata(path) {
            Ok(md) => md,
            Err(e) => return self.fail(path, &e),
        };
        if is_link(&md) {
            // Point de réanalyse : retrait du lien seul. `remove_dir` sur une jonction retire la
            // jonction (RemoveDirectoryW), jamais le contenu de sa cible.
            let r = if link_is_dir(&md) {
                fs::remove_dir(path)
            } else {
                fs::remove_file(path)
            };
            return r.map_or_else(|e| self.fail(path, &e), |()| true);
        }
        if md.is_dir() {
            let entries = match fs::read_dir(path) {
                Ok(it) => it,
                Err(e) => return self.fail(path, &e),
            };
            let mut all = true;
            for entry in entries {
                let gone = match entry {
                    Ok(entry) => self.tree(&entry.path()),
                    Err(e) => self.fail(path, &e),
                };
                all &= gone;
            }
            return all && fs::remove_dir(path).map_or_else(|e| self.fail(path, &e), |()| true);
        }
        if self.keep.iter().any(|k| k == path) {
            return false;
        }
        match fs::remove_file(path) {
            Ok(()) => true,
            Err(e) if e.kind() == ErrorKind::PermissionDenied && clear_readonly(path, &md) => {
                fs::remove_file(path).map_or_else(|e| self.fail(path, &e), |()| true)
            }
            Err(e) => self.fail(path, &e),
        }
    }
}

fn link_is_dir(md: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
        md.file_attributes() & FILE_ATTRIBUTE_DIRECTORY != 0
    }
    #[cfg(not(windows))]
    {
        let _ = md;
        false
    }
}

/// Windows refuse de supprimer un fichier en lecture seule : l'attribut est retiré d'abord.
#[allow(clippy::permissions_set_readonly_false)] // attribut Windows, sur un fichier à supprimer
fn clear_readonly(path: &Path, md: &fs::Metadata) -> bool {
    let mut perm = md.permissions();
    cfg!(windows) && perm.readonly() && {
        perm.set_readonly(false);
        fs::set_permissions(path, perm).is_ok()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Opts {
    pub clean: bool,
    pub now: SystemTime,
    pub free: Limits,
    pub main: Limits,
}

#[derive(Debug)]
struct Run<'a> {
    site: &'a Site,
    o: Opts,
    roots: Vec<PathBuf>,
    /// Tailles mesurées par le tableau, dans l'ordre de `SLOTS`.
    sizes: Vec<u64>,
    errors: Vec<String>,
}

impl Run<'_> {
    fn rel(&self, p: &Path) -> String {
        let s = p
            .strip_prefix(&self.site.cache)
            .or_else(|_| p.strip_prefix(&self.site.worktrees_root));
        s.map_or_else(|_| p.display().to_string(), |r| r.display().to_string())
    }

    /// Supprime `path` sous les racines permises, sans suivre de lien ; ou l'annonce seulement.
    fn remove(&mut self, path: &Path, keep: Vec<PathBuf>, lease: Option<&Taken>) {
        let rel = self.rel(path);
        if !self.o.clean {
            println!("    supprimerait {rel}");
            return;
        }
        match within(path, &self.roots) {
            Err(e) => self.errors.push(e),
            Ok(canon) => {
                let mut rm = Remover {
                    keep,
                    lease,
                    ..Remover::default()
                };
                let gone = rm.tree(&canon);
                match (gone, rm.errors.len()) {
                    (true, _) => println!("    supprimé {rel}"),
                    (false, 0) => println!("    supprimé {rel}, sauf les verrous de Cargo"),
                    (false, n) => println!("    {rel} : suppression incomplète, {n} erreur(s)"),
                }
                self.errors.append(&mut rm.errors);
            }
        }
    }

    #[allow(clippy::disallowed_methods)] // outil : liste d'un dossier sous les racines permises.
    fn children(&mut self, dir: &Path) -> Vec<PathBuf> {
        match fs::read_dir(dir) {
            Ok(it) => {
                let mut v: Vec<PathBuf> = it.filter_map(|e| e.ok().map(|e| e.path())).collect();
                v.sort();
                v
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
            Err(e) => {
                self.errors
                    .push(format!("{} illisible : {e}", dir.display()));
                Vec::new()
            }
        }
    }

    fn orphan_leases(&mut self) {
        println!("\nBaux orphelins :");
        for name in SLOTS {
            let lease = self.site.cache.join("target").join(format!("{name}.lease"));
            match slots::read_lease(&lease) {
                Some(l) if !slots::lease_held(&l, self.o.now) => {
                    if !self.o.clean {
                        println!("  {name} : libérerait le bail de {}", l.owner);
                        continue;
                    }
                    match release_orphan(&lease, self.o.now) {
                        Ok(true) => println!("  {name} : bail de {} libéré", l.owner),
                        Ok(false) => println!("  {name} : bail redevenu tenu, rendu"),
                        Err(e) => self.errors.push(format!("{name}.lease : {e}")),
                    }
                }
                _ => {}
            }
        }
    }

    fn slot(&mut self, name: &str) {
        let path = self.site.slot(name);
        let limits = if name == "main" {
            self.o.main
        } else {
            self.o.free
        };
        let Ok(md) = fs::symlink_metadata(&path) else {
            return println!("  {name} : absent");
        };
        if is_link(&md) {
            return self
                .errors
                .push(format!("{name} : l'emplacement est un lien, jamais suivi"));
        }
        let size = SLOTS
            .iter()
            .position(|n| *n == name)
            .and_then(|i| self.sizes.get(i).copied())
            .unwrap_or_else(|| measure(&path).bytes);
        let action = slot_action(size, limits);
        let (st, _) = slots::state(self.site, name, self.o.now);
        // `main` porte toujours la jonction de la copie principale : seule la session qui y
        // travaille, et lance `disk` depuis celle-ci, peut le nettoyer.
        let active = if name == "main" {
            !self.site.run_from_main()
        } else {
            !self.site.junctions_to(&path).is_empty()
        };
        print!("  {name} : {}, ", gib(size));
        if st == State::Held || (st == State::Orphan && self.o.clean) {
            return println!("tenu : jamais touché");
        }
        if action == SlotAction::Nothing {
            return println!("sous les seuils");
        }
        if active && name == "main" {
            return println!("nettoyé seulement depuis la copie principale");
        }
        if active {
            return println!("jonction d'un worktree actif : jamais touché");
        }
        let targets = match action {
            SlotAction::Nothing => return,
            SlotAction::Incremental => {
                println!("au-delà de {} : incremental", gib(limits.incremental));
                incremental_dirs(&path)
            }
            SlotAction::Empty => {
                println!("au-delà de {} : vidé", gib(limits.empty));
                self.children(&path)
            }
        };
        if !self.o.clean {
            targets
                .iter()
                .for_each(|t| self.remove(t, Vec::new(), None));
            return;
        }
        let lease_path = self.site.cache.join("target").join(format!("{name}.lease"));
        let lease = match Taken::take(&lease_path) {
            Ok(Some(l)) => l,
            Ok(None) => return println!("    bail pris entre-temps : jamais touché"),
            Err(e) => return self.errors.push(format!("{name} : {e}")),
        };
        match cargo_locks(&path) {
            Err(e) => println!("    {e} : jamais touché"),
            Ok(locks) => {
                let keep: Vec<PathBuf> = locks
                    .iter()
                    .filter_map(|(p, _)| fs::canonicalize(p).ok())
                    .collect();
                targets
                    .iter()
                    .for_each(|t| self.remove(t, keep.clone(), Some(&lease)));
                drop(locks);
            }
        }
        if let Err(e) = lease.release() {
            self.errors.push(format!("{name}.lease : {e}"));
        }
    }

    fn runs(&mut self) {
        let dir = self.site.cache.join("artifacts").join("runs");
        println!(
            "\nartifacts\\runs, plus de {} jours :",
            RUNS_MAX_AGE.as_secs() / 86_400
        );
        for run in self.children(&dir) {
            let Ok(md) = fs::symlink_metadata(&run) else {
                continue;
            };
            if is_link(&md) || fs::symlink_metadata(run.join(KEEP_MARK)).is_ok() {
                continue;
            }
            if measure(&run).idle_for(self.o.now, RUNS_MAX_AGE) {
                self.remove(&run, Vec::new(), None);
            }
        }
    }

    fn orphan_worktrees(&mut self) {
        println!("\nDossiers laissés dans .claude\\worktrees :");
        let site = self.site;
        for dir in self.children(&site.worktrees_root) {
            let Ok(md) = fs::symlink_metadata(&dir) else {
                continue;
            };
            let why = if is_link(&md) || !md.is_dir() {
                "lien ou fichier, ignoré"
            } else if site.is_listed(&dir) {
                continue;
            } else if fs::symlink_metadata(dir.join(".git")).is_ok() {
                "contient .git, gardé"
            } else if !measure(&dir).idle_for(self.o.now, ORPHAN_IDLE) {
                "modifié depuis moins de 24 h, gardé"
            } else {
                self.remove(&dir, Vec::new(), None);
                continue;
            };
            println!("  {} : {why}", self.rel(&dir));
        }
    }
}

/// Dossiers `incremental` d'un emplacement : `<profil>/incremental` et `<cible>/<profil>/incremental`.
/// Seuls l'emplacement et ses sous-dossiers directs sont listés, jamais `deps` ni `build`.
fn incremental_dirs(slot: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for a in real_subdirs(slot) {
        for b in real_subdirs(&a) {
            if b.file_name().is_some_and(|n| n == "incremental") {
                found.push(b);
            } else if is_real_dir(&b.join("incremental")) {
                found.push(b.join("incremental"));
            }
        }
    }
    found.sort();
    found
}

fn is_real_dir(p: &Path) -> bool {
    fs::symlink_metadata(p).is_ok_and(|md| md.is_dir() && !is_link(&md))
}

#[allow(clippy::disallowed_methods)] // outil : liste d'un dossier d'emplacement, sans suivre les liens.
fn real_subdirs(dir: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).into_iter().flatten().flatten();
    entries
        .map(|e| e.path())
        .filter(|p| is_real_dir(p))
        .collect()
}

fn cargo_home() -> Option<PathBuf> {
    sys::env("CARGO_HOME")
        .or_else(|| {
            sys::env("USERPROFILE")
                .or_else(|| sys::env("HOME"))
                .map(|h| format!("{h}/.cargo"))
        })
        .map(PathBuf::from)
}

/// Rend les erreurs rencontrées ; vide si tout s'est bien passé.
pub fn run(site: &mut Site, o: Opts) -> Vec<String> {
    let slot_sizes = slots::print_table(site, o.now);
    println!("\nTailles :");
    let sizes = [
        ("TS_CACHE\\artifacts", Some(site.cache.join("artifacts"))),
        (".claude\\worktrees", Some(site.worktrees_root.clone())),
        // Archives seulement : registry\src se régénère, et son parcours prend une quinzaine de secondes.
        (
            "archives de Cargo",
            cargo_home().map(|h| h.join("registry").join("cache")),
        ),
    ];
    for (label, path) in sizes {
        let size = path.map_or_else(|| "inconnu".to_owned(), |p| gib(measure(&p).bytes));
        println!("  {label:<26} {size}");
    }
    println!("Worktrees (git worktree list) :");
    site.worktrees
        .iter()
        .for_each(|w| println!("  {}", slots::show(w)));
    slots::print_free(site);

    let mut errors = Vec::new();
    println!("\ngit worktree prune :");
    let prune: &[&str] = if o.clean {
        &["worktree", "prune", "-v"]
    } else {
        &["worktree", "prune", "-n", "-v"]
    };
    match sys::run_live(&site.repo, "git", prune) {
        Ok(true) => {}
        Ok(false) => errors.push("git worktree prune a échoué".to_owned()),
        Err(e) => errors.push(e),
    }
    if let Err(e) = site.reload_worktrees() {
        errors.push(e);
        return errors; // sans liste fiable des worktrees, rien n'est supprimé
    }
    let roots: Vec<PathBuf> = [
        (site.cache.as_path(), &["target"][..]),
        (site.cache.as_path(), &["artifacts", "runs"][..]),
        (site.main_root.as_path(), &[".claude", "worktrees"][..]),
    ]
    .into_iter()
    .filter_map(|(base, rel)| root(base, rel))
    .collect();
    let mut r = Run {
        site,
        o,
        roots,
        sizes: slot_sizes,
        errors,
    };
    r.orphan_leases();
    println!("\nEmplacements (jamais un emplacement tenu) :");
    for name in ["w1", "w2", "w3", "main"] {
        r.slot(name);
    }
    r.runs();
    r.orphan_worktrees();
    if o.clean {
        slots::print_free(r.site);
    }
    r.errors
}

pub fn main(args: &[String]) -> ExitCode {
    let clean = match args {
        [] => false,
        [a] if a == "--clean" => true,
        _ => {
            eprintln!("usage : cargo xtask disk [--clean]");
            return ExitCode::from(2);
        }
    };
    let mut site = match Site::load(
        sys::ts_cache(),
        &std::env::current_dir().unwrap_or_default(),
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ÉCHEC : {e}");
            return ExitCode::FAILURE;
        }
    };
    let mode = if clean {
        "nettoyage"
    } else {
        "simulation, rien n'est modifié (--clean pour agir)"
    };
    println!("cargo xtask disk : {mode}\n");
    let opts = Opts {
        clean,
        now: now(),
        free: FREE_SLOT,
        main: MAIN_SLOT,
    };
    let errors = run(&mut site, opts);
    if errors.is_empty() {
        println!("\nRésultat : OK");
        return ExitCode::SUCCESS;
    }
    println!("\nRésultat : ÉCHEC ({} erreur(s))", errors.len());
    errors.iter().take(20).for_each(|e| println!("  {e}"));
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Dossier jetable sous `TS_CACHE\tmp`, jamais ailleurs.
    fn scratch(name: &str) -> PathBuf {
        let dir = slots::scratch_root().join(format!("xtask-disk-{name}-{}", std::process::id()));
        if fs::symlink_metadata(&dir).is_ok() {
            fs::remove_dir_all(&dir).unwrap();
        }
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[allow(clippy::disallowed_methods)] // test : fichiers jetables.
    fn put(path: &Path, content: &[u8]) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn file(path: &Path, bytes: usize) {
        put(path, &vec![b'x'; bytes]);
    }

    #[allow(clippy::disallowed_methods)] // test : contenu d'un fichier témoin.
    fn len(path: &Path) -> usize {
        fs::read(path).map_or(0, |b| b.len())
    }

    fn exists(path: &Path) -> bool {
        fs::symlink_metadata(path).is_ok()
    }

    /// Vraie jonction sous Windows (point de réanalyse), lien symbolique ailleurs.
    fn junction(link: &Path, target: &Path) {
        #[cfg(windows)]
        {
            // mklink n'accepte que la barre oblique inverse.
            let win = |p: &Path| p.to_string_lossy().replace('/', "\\");
            let out = std::process::Command::new("cmd")
                .args(["/C", "mklink", "/J", &win(link), &win(target)])
                .output()
                .unwrap();
            let err = String::from_utf8_lossy(&out.stderr);
            assert!(out.status.success(), "mklink /J a échoué : {err}");
        }
        #[cfg(not(windows))]
        std::os::unix::fs::symlink(target, link).unwrap();
        let md = fs::symlink_metadata(link).unwrap();
        assert!(
            is_link(&md),
            "le lien doit être vu comme un point de réanalyse"
        );
    }

    #[test]
    fn thresholds() {
        let l = Limits {
            incremental: 6,
            empty: 10,
        };
        assert_eq!(slot_action(6, l), SlotAction::Nothing);
        assert_eq!(slot_action(7, l), SlotAction::Incremental);
        assert_eq!(slot_action(10, l), SlotAction::Incremental);
        assert_eq!(slot_action(11, l), SlotAction::Empty);
        assert_eq!(FREE_SLOT.incremental, 6 << 30);
        assert_eq!(FREE_SLOT.empty, 10 << 30);
        assert_eq!(MAIN_SLOT.incremental, 12 << 30);
        assert_eq!(MAIN_SLOT.empty, 20 << 30);
    }

    #[test]
    fn removal_never_follows_a_junction() {
        let dir = scratch("junction");
        let outside = dir.join("outside");
        file(&outside.join("sentinel"), 10);
        file(&outside.join("sub").join("deep"), 10);
        file(&dir.join("inside").join("a").join("b"), 10);
        junction(&dir.join("inside").join("a").join("escape"), &outside);
        junction(&dir.join("top-link"), &outside);

        let mut rm = Remover::default();
        // Lien en tête : seul le lien disparaît.
        assert!(rm.tree(&dir.join("top-link")));
        // Lien dans l'arborescence : retiré sans être parcouru.
        assert!(rm.tree(&dir.join("inside")));
        assert!(rm.errors.is_empty(), "{:?}", rm.errors);
        assert!(!exists(&dir.join("inside")) && !exists(&dir.join("top-link")));
        assert_eq!(len(&outside.join("sentinel")), 10);
        assert_eq!(len(&outside.join("sub").join("deep")), 10);
        // La mesure ne suit pas non plus le lien.
        junction(&dir.join("again"), &outside);
        assert_eq!(measure(&dir.join("again")).bytes, 0);
        assert_eq!(measure(&outside).bytes, 20);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn prefix_check_is_canonical() {
        let dir = scratch("prefix");
        let root_dir = dir.join("root");
        let outside = dir.join("outside");
        file(&root_dir.join("ok").join("f"), 1);
        file(&outside.join("victim"), 1);
        junction(&root_dir.join("door"), &outside);
        let roots = vec![root(&dir, &["root"]).unwrap()];
        // Une racine qui est elle-même un lien est refusée.
        junction(&dir.join("fake-root"), &outside);
        assert!(root(&dir, &["fake-root"]).is_none());

        assert!(within(&root_dir.join("ok"), &roots).is_ok());
        // Le lien lui-même est sous la racine : il peut être retiré, sans être suivi.
        assert!(within(&root_dir.join("door"), &roots).is_ok());
        // Mais rien au travers du lien : le parent canonique est hors de la racine.
        assert!(within(&root_dir.join("door").join("victim"), &roots).is_err());
        assert!(within(&root_dir, &roots).is_err());
        assert!(within(&root_dir.join("ok").join(".."), &roots).is_err());
        assert!(within(&outside.join("victim"), &roots).is_err());
        assert!(within(&root_dir.join("..").join("outside"), &roots).is_err());
        // Une racine de disque n'est jamais une racine permise.
        assert!(root(Path::new("/"), &[]).is_none());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn lease_is_exclusive_and_cargo_lock_blocks() {
        let dir = scratch("lease");
        let lease = dir.join("w9.lease");
        let taken = Taken::take(&lease).unwrap().unwrap();
        assert!(Taken::take(&lease).unwrap().is_none());
        taken.release().unwrap();
        assert!(!exists(&lease));

        // Bail orphelin libéré ; bail redevenu tenu rendu tel quel.
        put(&lease, b"C:/nulle/part");
        let later = now() + Duration::from_secs(3600);
        assert!(release_orphan(&lease, later).unwrap());
        assert!(!exists(&lease));
        put(&lease, b"C:/nulle/part");
        assert!(!release_orphan(&lease, now()).unwrap());
        assert_eq!(slots::read_lease(&lease).unwrap().owner, "C:/nulle/part");

        let slot = dir.join("slot");
        file(&slot.join("debug").join(".cargo-lock"), 0);
        let held = cargo_locks(&slot).unwrap();
        assert_eq!(held.len(), 1);
        // Tant qu'un verrou est tenu, l'emplacement est refusé.
        assert!(cargo_locks(&slot).is_err());
        drop(held);
        assert!(cargo_locks(&slot).is_ok());
        fs::remove_dir_all(&dir).unwrap();
    }

    /// Parcours complet dans un `TS_CACHE` et un dépôt jetables : simulation, puis nettoyage.
    #[test]
    fn clean_in_throwaway_cache() {
        let dir = scratch("flow");
        let cache = dir.join("cache");
        let repo = dir.join("repo");
        fs::create_dir_all(&repo).unwrap();
        assert!(sys::run_captured(&repo, "git", &["init", "-q"]).unwrap().0);
        let target = cache.join("target");
        let outside = dir.join("outside");
        file(&outside.join("sentinel"), 5000);
        // w1 : tenu (son worktree a un .git), au-delà des seuils : jamais touché.
        let owner = dir.join("owner");
        fs::create_dir_all(owner.join(".git")).unwrap();
        file(&target.join("w1").join("big"), 2000);
        put(&target.join("w1.lease"), owner.to_string_lossy().as_bytes());
        // w2 : bail orphelin, au-delà du seuil de vidage, avec une jonction vers l'extérieur.
        let w2 = target.join("w2").join("debug");
        file(&w2.join("big"), 2000);
        file(&w2.join(".cargo-lock"), 0);
        junction(&w2.join("escape"), &outside);
        put(&target.join("w2.lease"), b"C:/nulle/part");
        // w3 : entre les seuils : incremental seulement.
        let w3 = target.join("w3").join("debug");
        file(&w3.join("incremental").join("x"), 500);
        file(&w3.join("lib.rlib"), 100);
        // main : sous les seuils.
        file(&target.join("main").join("f"), 5);
        // Exécutions : une ancienne, une marquée keep.
        let runs = cache.join("artifacts").join("runs");
        file(&runs.join("old").join("x"), 1);
        file(&runs.join("kept").join(KEEP_MARK), 1);
        // Worktrees : un dossier orphelin avec sa jonction, un avec .git.
        let wts = repo.join(".claude").join("worktrees");
        file(&wts.join("orphan").join("src.rs"), 1);
        junction(&wts.join("orphan").join("target"), &outside);
        file(&wts.join("with-git").join(".git"), 1);

        let mut site = Site::load(cache.clone(), &repo).unwrap();
        let limits = Limits {
            incremental: 10,
            empty: 1000,
        };
        let mut o = Opts {
            clean: false,
            // Un mois plus tard : baux sans .git orphelins, dossiers inactifs.
            now: now() + Duration::from_secs(31 * 24 * 3600),
            free: limits,
            main: limits,
        };
        let present = [
            target.join("w1").join("big"),
            w2.join("big"),
            w2.join("escape"),
            target.join("w2.lease"),
            w3.join("incremental"),
            runs.join("old"),
            wts.join("orphan"),
        ];
        assert!(run(&mut site, o).is_empty());
        assert!(present.iter().all(|p| exists(p)), "la simulation a modifié");

        o.clean = true;
        let errors = run(&mut site, o);
        assert!(errors.is_empty(), "{errors:?}");
        assert!(
            exists(&target.join("w1").join("big")),
            "emplacement tenu touché"
        );
        assert!(exists(&target.join("w1.lease")));
        assert!(!exists(&w2.join("big")) && !exists(&w2.join("escape")));
        assert!(exists(&w2.join(".cargo-lock")), "verrou de Cargo supprimé");
        assert!(!exists(&target.join("w2.lease")), "bail non rendu");
        assert!(!exists(&w3.join("incremental")) && exists(&w3.join("lib.rlib")));
        assert!(exists(&target.join("main").join("f")));
        assert!(!exists(&runs.join("old")) && exists(&runs.join("kept")));
        assert!(!exists(&wts.join("orphan")) && exists(&wts.join("with-git")));
        // Aucune jonction suivie : la cible est intacte.
        assert_eq!(len(&outside.join("sentinel")), 5000);

        // Un dossier orphelin récent est gardé.
        file(&wts.join("fresh").join("f"), 1);
        o.now = now();
        assert!(run(&mut site, o).is_empty());
        assert!(exists(&wts.join("fresh")));

        // Lancé depuis un autre worktree, `main` n'est jamais vidé, même au-delà des seuils.
        file(&target.join("main").join("big"), 2000);
        site.main_root = owner.clone();
        site.worktrees_root = owner.join(".claude").join("worktrees");
        o.now = now() + Duration::from_secs(31 * 24 * 3600);
        assert!(run(&mut site, o).is_empty());
        assert!(exists(&target.join("main").join("big")));
        fs::remove_dir_all(&dir).unwrap();
    }
}
