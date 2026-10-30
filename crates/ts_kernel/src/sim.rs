//! Construction de la simulation et boucle : `SimConfig`, `Sim::build`, `step`, `run_until`.

use core::fmt;
use core::num::NonZeroU16;
use core::ops::{Deref, DerefMut};
use std::collections::BTreeSet;

use bevy_ecs::component::{Component, ComponentId};
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{QueryState, With};
use bevy_ecs::resource::Resource;
use bevy_ecs::schedule::{Schedule, Schedules};
use bevy_ecs::world::World;
use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};

use crate::phase::phase_schedule;
use crate::rng::{StreamKeys, WorldKey};
use crate::state::{
    StableIndex, StateEntry, StateWriter, hash_world, stable_entities, undeclared_components,
};
use crate::{
    DayOrigin, Phase, Plugin, RulesFingerprint, Scheduled, Scheduler, SimBuilder, SimTime,
    StableEntity, StableId, StableIdAllocator, StateHash, StateHashes,
};

mod snapshot;
pub use snapshot::{MemorySnapshot, RestoreError, Snapshot, SnapshotScope, StateLayout};

// spec: revues/architecture.md §11.3, §12.1 ; carnet architecture-securite.md §9.10 L4, L5
/// Encode l'état du noyau : date, origine des jours, compteur de [`StableId`], compteur `seq`
/// et événements en attente. Jamais les clés de flux.
fn write_kernel(world: &World, w: &mut StateWriter<'_>) {
    let clock = world.resource::<SimClock>();
    clock.now.hash_state(w);
    clock.day_origin.hash_state(w);
    w.u64(world.resource::<StableIds>().0.to_snapshot_raw());
    let queue = world.resource::<EventQueue>();
    w.u64(queue.next_seq());
    let pending = queue.pending();
    w.count(pending.len());
    for e in pending {
        e.at.hash_state(w);
        w.u64(e.seq);
        e.event.hash_state(w);
    }
    // Stable entities (M1), then those in full simulation (I1), both sorted by `StableId`.
    let stable = stable_entities(world);
    w.count(stable.len());
    for &(id, _) in &stable {
        id.hash_state(w);
    }
    let complete = complete_ids(world, &stable);
    w.count(complete.len());
    for id in complete {
        id.hash_state(w);
    }
}

// spec: revues/architecture.md §6.5, §8.1 ; carnet architecture-securite.md §9.10 L2
/// Stable entities that carry [`LodComplete`], sorted by `StableId`.
fn complete_ids(world: &World, stable: &[(StableId, Entity)]) -> Vec<StableId> {
    stable
        .iter()
        .filter(|&&(_, e)| world.get::<LodComplete>(e).is_some())
        .map(|&(id, _)| id)
        .collect()
}

impl StateHash for TimedEvent {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u32(self.kind);
        self.subject.hash_state(w);
        w.u64(self.arg);
    }
}

// spec: revues/architecture.md §7.1, §9.2 ; carnet architecture-securite.md §9.10 L6
/// Nombre de fils de calcul. **Sans effet sur le résultat** (R1).
///
/// Le groupe de fils de `bevy_tasks` est global au processus et ne se règle qu'une fois : la
/// première construction le fixe, et une construction qui en demande un autre nombre échoue
/// ([`BuildError::ThreadPoolMismatch`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Threads {
    /// Un fil : exécuteur séquentiel, groupe de fils d'un fil pour `par_iter`.
    Single,
    /// `n` fils : exécuteur parallèle de `bevy_ecs`.
    Fixed(NonZeroU16),
}

impl Threads {
    /// Nombre de fils du groupe de calcul.
    pub const fn count(self) -> usize {
        match self {
            Self::Single => 1,
            Self::Fixed(n) => n.get() as usize,
        }
    }
}

// spec: revues/architecture.md §7.1, §8.2
/// Configuration d'une simulation. Elle ne contient aucun état du monde et aucun secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimConfig {
    /// Fréquence du tick, en hertz : 20 par défaut (50 ms). Elle doit diviser 1 000.
    pub tick_hz: u32,
    /// Nombre de fils de calcul.
    pub threads: Threads,
    /// Greffons coupés, par nom ; chacun doit être [`Plugin::disableable`].
    pub disabled_plugins: Vec<&'static str>,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            tick_hz: 20,
            threads: Threads::Single,
            disabled_plugins: Vec::new(),
        }
    }
}

// spec: revues/architecture.md §6.5, §8.1 ; carnet architecture-securite.md §9.10 L2
/// Marqueur « simulation complète » d'une entité. Défini ici seulement, il sert au saut de
/// temps ; `ts_model` le réexporte avec `LodSimplified` et `LodStatistical`.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LodComplete;

// spec: revues/architecture.md §8.1, §6.2 ; README.md
/// Horloge du monde, lue par les systèmes : début du tick en cours et origine des jours
/// compacts (AS-22).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimClock {
    now: SimTime,
    tick_ms: i64,
    day_origin: DayOrigin,
}

impl SimClock {
    /// Début du tick en cours.
    pub const fn now(&self) -> SimTime {
        self.now
    }

    /// Durée d'un tick, en millisecondes.
    pub const fn tick_millis(&self) -> i64 {
        self.tick_ms
    }

    /// Origine des jours compacts.
    pub const fn day_origin(&self) -> DayOrigin {
        self.day_origin
    }
}

// spec: revues/architecture.md §8.1
/// Événement daté du noyau. Sa forme est provisoire : `ts_model` fixera les genres
/// d'événements (fins de blocs d'agenda, échéancier, jalons, minuteurs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimedEvent {
    /// Genre de l'événement, attribué par le greffon qui le traite.
    pub kind: u32,
    /// Entité concernée.
    pub subject: StableId,
    /// Argument libre.
    pub arg: u64,
}

// spec: revues/architecture.md §8.1, §6.5
/// Ressource : l'ordonnanceur du monde.
#[derive(Resource, Debug)]
pub struct EventQueue(Scheduler<TimedEvent>);

impl Deref for EventQueue {
    type Target = Scheduler<TimedEvent>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EventQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// spec: revues/architecture.md §8.1, §9.1
/// Ressource : événements dus pendant le tick en cours, dans l'ordre `(at, seq)`. Remplie
/// avant la phase [`Phase::Scheduled`], vidée à la fin du tick.
#[derive(Resource, Debug, Default)]
pub struct DueEvents(Vec<Scheduled<TimedEvent>>);

impl Deref for DueEvents {
    type Target = [Scheduled<TimedEvent>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// spec: revues/architecture.md §6.1 ; carnet architecture-securite.md §9.10 L5
/// Ressource : l'allocateur des [`StableId`] du monde.
#[derive(Resource, Debug)]
pub struct StableIds(StableIdAllocator);

impl StableIds {
    /// Nouvel identifiant, ou `None` si l'espace est épuisé.
    pub const fn allocate(&mut self) -> Option<StableId> {
        self.0.allocate()
    }
}

// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §9.10 L4
/// Ressource : les clés de flux, dérivées de la clé du monde à la construction. Elles ne sont
/// jamais enregistrées ni hachées ; la clé du monde n'est pas gardée.
#[derive(Resource, Debug)]
pub struct WorldStreams(StreamKeys);

impl Deref for WorldStreams {
    type Target = StreamKeys;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// spec: revues/architecture.md §5.3, §7.1, §9.2
/// Échec de la construction d'une [`Sim`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// `tick_hz` nul ou qui ne divise pas 1 000.
    InvalidTickHz(u32),
    /// Deux greffons portent ce nom.
    DuplicatePlugin(&'static str),
    /// Un greffon coupé n'est pas dans les règles.
    UnknownDisabledPlugin(&'static str),
    /// Un greffon obligatoire est coupé.
    NotDisableable(&'static str),
    /// Un greffon dépend d'un greffon absent des règles.
    MissingDependency {
        /// Greffon qui déclare la dépendance.
        plugin: &'static str,
        /// Greffon requis.
        dependency: &'static str,
    },
    /// Un greffon actif dépend d'un greffon coupé.
    DisabledDependency {
        /// Greffon qui déclare la dépendance.
        plugin: &'static str,
        /// Greffon requis, coupé.
        dependency: &'static str,
    },
    /// Le groupe de fils du processus a déjà un autre nombre de fils.
    ThreadPoolMismatch {
        /// Fils demandés.
        wanted: usize,
        /// Fils du groupe existant.
        actual: usize,
    },
    /// Le `Schedule` d'une phase ne se construit pas (ambiguïté d'ordre, cycle…).
    Schedule {
        /// Phase concernée.
        phase: Phase,
        /// Message de `bevy_ecs`.
        message: String,
    },
    /// A plugin depends on a plugin that comes later in the rules.
    DependencyOrder {
        /// Plugin that declares the dependency.
        plugin: &'static str,
        /// Required plugin, placed after it.
        dependency: &'static str,
    },
    /// A plugin switched off ambiguity detection (L7): `ambiguous_with_all` on a system of
    /// `phase`, or, with `phase: None`, `allow_ambiguous_component` or
    /// `allow_ambiguous_resource` on the world.
    AmbiguityOptOut {
        /// Phase concerned, or `None` for a world-wide opt-out.
        phase: Option<Phase>,
    },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTickHz(hz) => write!(f, "tick_hz invalide : {hz} ne divise pas 1000"),
            Self::DuplicatePlugin(n) => write!(f, "greffon en double : {n}"),
            Self::UnknownDisabledPlugin(n) => write!(f, "greffon coupé inconnu : {n}"),
            Self::NotDisableable(n) => write!(f, "le greffon {n} ne peut pas être coupé"),
            Self::MissingDependency { plugin, dependency } => {
                write!(f, "le greffon {plugin} dépend de {dependency}, absent")
            }
            Self::DisabledDependency { plugin, dependency } => {
                write!(f, "le greffon {plugin} dépend de {dependency}, coupé")
            }
            Self::ThreadPoolMismatch { wanted, actual } => {
                write!(f, "{wanted} fils demandés, le processus en a déjà {actual}")
            }
            Self::Schedule { phase, message } => write!(f, "phase {phase:?} : {message}"),
            Self::DependencyOrder { plugin, dependency } => {
                write!(
                    f,
                    "plugin {plugin} depends on {dependency}, which comes after it"
                )
            }
            Self::AmbiguityOptOut { phase: Some(phase) } => {
                write!(
                    f,
                    "a system of phase {phase:?} opts out of ambiguity detection"
                )
            }
            Self::AmbiguityOptOut { phase: None } => {
                f.write_str("the world opts components or resources out of ambiguity detection")
            }
        }
    }
}

impl std::error::Error for BuildError {}

// spec: revues/architecture.md §7.1, §8.1
/// Condition d'arrêt de [`Sim::run_until`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopCondition {
    /// S'arrêter au premier début de tick à cette date ou après.
    At(SimTime),
}

/// Bilan d'un tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepReport {
    /// Début du tick exécuté.
    pub tick_start: SimTime,
    /// Événements dus traités par le tick.
    pub due_events: usize,
}

/// Bilan de [`Sim::run_until`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunReport {
    /// Ticks exécutés.
    pub ticks: u64,
    /// Ticks sautés sans exécution (saut de temps).
    pub skipped_ticks: u64,
}

// spec: revues/architecture.md §7.1, §8.1, §8.2, §9.1
/// La simulation : un monde `bevy_ecs`, un `Schedule` par phase et l'horloge.
pub struct Sim {
    world: World,
    phases: Vec<Schedule>,
    tick_ms: i64,
    active: Vec<(&'static str, u32)>,
    rules: RulesFingerprint,
    complete: QueryState<Entity, With<LodComplete>>,
    state: Vec<StateEntry>,
    migrations: Vec<(&'static str, u32)>,
    /// Components allowed on stable entities (M2).
    known_components: Vec<ComponentId>,
}

impl fmt::Debug for Sim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sim")
            .field("now", &self.now())
            .field("tick_ms", &self.tick_ms)
            .field("active", &self.active)
            .field("rules", &self.rules)
            .finish_non_exhaustive()
    }
}

/// Règle le groupe de fils global, ou vérifie qu'il a déjà le bon nombre de fils.
fn ensure_thread_pool(threads: Threads) -> Result<(), BuildError> {
    let wanted = threads.count();
    let pool = ComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::new()
            .num_threads(wanted)
            .thread_name("ts_kernel".into())
            .build()
    });
    let actual = pool.thread_num();
    if actual == wanted {
        Ok(())
    } else {
        Err(BuildError::ThreadPoolMismatch { wanted, actual })
    }
}

/// Vérifie les noms et les coupures, et rend les greffons actifs, dans l'ordre des règles.
fn active_plugins<'a>(
    cfg: &SimConfig,
    rules: &'a [Box<dyn Plugin>],
) -> Result<Vec<&'a dyn Plugin>, BuildError> {
    let mut names = BTreeSet::new();
    for p in rules {
        if !names.insert(p.name()) {
            return Err(BuildError::DuplicatePlugin(p.name()));
        }
    }
    for &name in &cfg.disabled_plugins {
        let Some(p) = rules.iter().find(|p| p.name() == name) else {
            return Err(BuildError::UnknownDisabledPlugin(name));
        };
        if !p.disableable() {
            return Err(BuildError::NotDisableable(name));
        }
    }
    let active: Vec<&dyn Plugin> = rules
        .iter()
        .filter(|p| !cfg.disabled_plugins.contains(&p.name()))
        .map(|p| &**p)
        .collect();
    // Dependencies are checked before any `build` runs (T6a review m5).
    for (i, p) in active.iter().enumerate() {
        for &dependency in p.dependencies() {
            let plugin = p.name();
            if cfg.disabled_plugins.contains(&dependency) {
                return Err(BuildError::DisabledDependency { plugin, dependency });
            }
            match active.iter().position(|q| q.name() == dependency) {
                None => return Err(BuildError::MissingDependency { plugin, dependency }),
                Some(j) if j > i => {
                    return Err(BuildError::DependencyOrder { plugin, dependency });
                }
                Some(_) => {}
            }
        }
    }
    Ok(active)
}

impl Sim {
    /// Construit un monde neuf à l'époque, avec les greffons `rules` moins ceux que coupe
    /// `cfg`. Les clés de flux sont dérivées de `world_key`, qui n'est pas gardée.
    ///
    /// # Errors
    ///
    /// Voir [`BuildError`] : configuration invalide, dépendance coupée ou absente, groupe de
    /// fils incompatible, ambiguïté d'ordre dans une phase.
    pub fn build(
        cfg: SimConfig,
        rules: &[Box<dyn Plugin>],
        world_key: &WorldKey,
    ) -> Result<Self, BuildError> {
        if cfg.tick_hz == 0 || 1000 % cfg.tick_hz != 0 {
            return Err(BuildError::InvalidTickHz(cfg.tick_hz));
        }
        let tick_ms = i64::from(1000 / cfg.tick_hz);
        let plugins = active_plugins(&cfg, rules)?;
        ensure_thread_pool(cfg.threads)?;

        let mut world = World::new();
        let kernel_components = vec![
            world.register_component::<LodComplete>(),
            world.register_component::<StableEntity>(),
        ];
        world.insert_resource(StableIndex::default());
        world.insert_resource(SimClock {
            now: SimTime::EPOCH,
            tick_ms,
            day_origin: DayOrigin::EPOCH,
        });
        world.insert_resource(EventQueue(Scheduler::new(SimTime::EPOCH)));
        world.insert_resource(DueEvents::default());
        world.insert_resource(StableIds(StableIdAllocator::for_new_world()));
        world.insert_resource(WorldStreams(StreamKeys::derive(world_key)));

        let mut b = SimBuilder {
            world,
            phases: Phase::ALL
                .iter()
                .map(|&p| phase_schedule(p, cfg.threads))
                .collect(),
            current: "",
            state: Vec::new(),
            migrations: Vec::new(),
            known_components: kernel_components,
        };
        for p in &plugins {
            b.current = p.name();
            p.build(&mut b);
        }

        let SimBuilder {
            mut world,
            mut phases,
            state,
            migrations,
            known_components,
            ..
        } = b;
        // L7 cannot be switched off (T6a review m3). `ambiguous_with(x)` edges are not
        // visible through the `bevy_ecs` API; `clippy.toml` bans them in rule crates.
        let world_opt_out = world
            .get_resource::<Schedules>()
            .is_some_and(|s| !s.ignored_scheduling_ambiguities.is_empty());
        if world_opt_out {
            return Err(BuildError::AmbiguityOptOut { phase: None });
        }
        for (schedule, &phase) in phases.iter_mut().zip(Phase::ALL.iter()) {
            if !schedule.graph().ambiguous_with_all.is_empty() {
                return Err(BuildError::AmbiguityOptOut { phase: Some(phase) });
            }
            if let Err(e) = schedule.initialize(&mut world) {
                let message = e.to_string(schedule.graph(), &world);
                return Err(BuildError::Schedule { phase, message });
            }
        }
        let active: Vec<_> = plugins.iter().map(|p| (p.name(), p.version())).collect();
        let rules = RulesFingerprint::of(&active);
        let complete = world.query_filtered::<Entity, With<LodComplete>>();
        Ok(Self {
            world,
            phases,
            tick_ms,
            active,
            rules,
            complete,
            state,
            migrations,
            known_components,
        })
    }

    // spec: revues/architecture.md §11.3
    /// Number of components carried by stable entities that are neither declared state,
    /// declared transient, nor kernel components (M2). Such a component is lost on restore;
    /// [`Sim::snapshot`] checks that this is zero in debug builds.
    pub fn undeclared_state_components(&self) -> usize {
        undeclared_components(&self.world, &self.known_components)
    }

    // spec: revues/architecture.md §7.1, §11.3 ; carnet architecture-securite.md §9.10 L3, L4
    /// Empreintes d'état, par famille et globale. La famille `kernel` couvre l'horloge,
    /// l'origine des jours, l'allocateur de [`StableId`] et les événements en attente ; les
    /// autres, les types d'état déclarés par les greffons. Les clés de flux n'y entrent pas.
    pub fn state_hash(&self) -> StateHashes {
        hash_world(&self.world, &self.state, &|w| write_kernel(&self.world, w))
    }

    /// Début du tick suivant : la date courante du monde.
    pub fn now(&self) -> SimTime {
        self.world.resource::<SimClock>().now
    }

    /// Durée d'un tick, en millisecondes.
    pub const fn tick_millis(&self) -> i64 {
        self.tick_ms
    }

    /// Empreinte des règles actives.
    pub const fn rules_fingerprint(&self) -> RulesFingerprint {
        self.rules
    }

    /// Greffons actifs `(nom, version)`, dans l'ordre des règles.
    pub fn active_plugins(&self) -> &[(&'static str, u32)] {
        &self.active
    }

    /// Le monde, en lecture seule (architecture §7.3 : visualiseur et tests).
    pub const fn world(&self) -> &World {
        &self.world
    }

    // spec: revues/architecture.md §8.1, §8.2, §9.1
    /// Exécute un tick : `Input`, puis les événements dus dans `[now, now + tick[` passent
    /// dans [`DueEvents`], puis les neuf autres phases ; enfin l'horloge avance d'un tick.
    pub fn step(&mut self) -> StepReport {
        let tick_start = self.now();
        let last = tick_start.add_millis(self.tick_ms - 1);
        self.phases[Phase::Input.index()].run(&mut self.world);
        let due = self.world.resource_mut::<EventQueue>().pop_due(last);
        let due_events = due.len();
        self.world.resource_mut::<DueEvents>().0 = due;
        for schedule in &mut self.phases[Phase::Scheduled.index()..] {
            schedule.run(&mut self.world);
        }
        self.world.resource_mut::<DueEvents>().0.clear();
        self.world.resource_mut::<SimClock>().now = last.add_millis(1);
        self.world.clear_trackers();
        StepReport {
            tick_start,
            due_events,
        }
    }

    /// Vrai si une entité porte [`LodComplete`].
    fn has_complete_entities(&mut self) -> bool {
        self.complete.iter(&self.world).next().is_some()
    }

    /// Premier début de tick à `t` ou avant. Only called with `t > now()`, so the result is
    /// at least `now()` and cannot underflow.
    fn tick_floor(&self, t: SimTime) -> SimTime {
        let ms = t.as_millis();
        SimTime::from_millis(ms - ms.rem_euclid(self.tick_ms))
    }

    // spec: revues/architecture.md §8.1, §8.5
    /// Avance jusqu'au premier début de tick à la date d'arrêt ou après.
    ///
    /// **Saut de temps** : tant qu'aucune entité ne porte [`LodComplete`], la boucle ne tique
    /// plus et saute directement au tick qui contient l'événement suivant. Les ticks sautés
    /// sont vides : un événement est donc traité dans le même tick, qu'il y ait saut ou non.
    /// This relies on the time-skip contract stated on [`Plugin`].
    ///
    /// Never panics on extreme dates: a stop date at or before `now()` does nothing, and a
    /// stop date within one tick of `i64::MAX` stops at the last tick start before it
    /// (T6a review m2).
    pub fn run_until(&mut self, stop: StopCondition) -> RunReport {
        let StopCondition::At(at) = stop;
        let mut report = RunReport::default();
        if at <= self.now() {
            return report;
        }
        let floor = self.tick_floor(at);
        let target = if floor == at {
            at
        } else {
            floor.checked_add_millis(self.tick_ms).unwrap_or(floor)
        };
        while self.now() < target {
            if !self.has_complete_entities() {
                let next = self.world.resource::<EventQueue>().peek_next_time();
                let to = match next {
                    Some(t) if t <= self.now() => self.now(),
                    Some(t) if t < target => self.tick_floor(t),
                    _ => target,
                };
                let skipped = to.checked_millis_since(self.now()).unwrap_or(0) / self.tick_ms;
                report.skipped_ticks += skipped.unsigned_abs();
                self.world.resource_mut::<SimClock>().now = to;
                if to == target {
                    break;
                }
            }
            self.step();
            report.ticks += 1;
        }
        report
    }
}

#[cfg(test)]
mod tests;
