//! Simulation construction and loop: `SimConfig`, `Sim::build`, `step`, `run_until`.

use core::fmt;
use core::num::NonZeroU16;
use core::ops::{Deref, DerefMut};
use std::collections::{BTreeMap, BTreeSet};

use bevy_ecs::component::{Component, ComponentId};
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{QueryState, With};
use bevy_ecs::resource::Resource;
use bevy_ecs::schedule::{Schedule, Schedules};
use bevy_ecs::world::World;
use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};

use crate::phase::{ExecutorSwitch, phase_schedule};
use crate::rng::{CosmeticSeed, DrawSite, Draws, Stream, StreamKeys, WorldKey};
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
/// Encodes the kernel state: date, day origin, [`StableId`] counter, `seq` counter and
/// pending events. Never the stream keys.
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
/// Number of compute threads. **No effect on the result** (R1).
///
/// The `bevy_tasks` thread pool is global to the process and can only be set once: the
/// first build fixes it, and a build that asks for another number fails
/// ([`BuildError::ThreadPoolMismatch`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Threads {
    /// One thread: sequential executor, one-thread pool for `par_iter`.
    Single,
    /// `n` threads: `n`-thread pool for `par_iter`, and the parallel `bevy_ecs` executor for
    /// the phases that [`ParallelPhases`] selects.
    Fixed(NonZeroU16),
}

// spec: revues/architecture.md §9.1, §9.2
/// Work from which a phase runs on the parallel executor by default; see
/// [`ParallelPhases::FromWork`]. A speed setting, never a game value.
// PROVISIONAL(design #94): to calibrate on the phase 1 systems; the result does not depend on it.
pub const PARALLEL_FROM_WORK: u32 = 256;

// spec: revues/architecture.md §9.1, §9.2 ; carnet architecture-securite.md §9.10 L6
/// Which phases run on the parallel `bevy_ecs` executor with [`Threads::Fixed`]; the others
/// run on the single-threaded executor, where `par_iter` still uses the whole pool. Ignored
/// with [`Threads::Single`].
///
/// **No effect on the result** (R1), only on speed: see `AdaptiveExecutor` in `phase.rs`.
/// The choice is made before each phase from the configuration and the world state alone,
/// and kept outside the `World`, so no system can see it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParallelPhases {
    /// Every phase, every tick: the behaviour before issue #67. The determinism proofs use it
    /// to exercise the parallel executor on small worlds.
    Always,
    /// Only a phase with at least two systems (a single system gains nothing from the
    /// parallel executor), and only when its **work** reaches this value: the entities in
    /// full simulation ([`LodComplete`]) at the start of the phase, plus the events due in
    /// the tick (none yet in [`Phase::Input`]). Counted in work units, never in time.
    FromWork(u32),
}

impl Default for ParallelPhases {
    fn default() -> Self {
        Self::FromWork(PARALLEL_FROM_WORK)
    }
}

impl Threads {
    /// Number of threads of the compute pool.
    pub const fn count(self) -> usize {
        match self {
            Self::Single => 1,
            Self::Fixed(n) => n.get() as usize,
        }
    }
}

// spec: revues/architecture.md §7.1, §8.2
/// Configuration of a simulation. It holds no world state and no secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimConfig {
    /// Tick frequency, in hertz: 20 by default (50 ms). It must divide 1,000.
    pub tick_hz: u32,
    /// Number of compute threads.
    pub threads: Threads,
    /// Phases on the parallel executor, with [`Threads::Fixed`]. Speed only.
    pub parallel_phases: ParallelPhases,
    /// Disabled plugins, by name; each must be [`Plugin::disableable`].
    pub disabled_plugins: Vec<&'static str>,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            tick_hz: 20,
            threads: Threads::Single,
            parallel_phases: ParallelPhases::default(),
            disabled_plugins: Vec::new(),
        }
    }
}

// spec: revues/architecture.md §6.5, §8.1 ; carnet architecture-securite.md §9.10 L2
/// "Full simulation" marker of an entity. Defined here only, it serves the time skip;
/// `ts_model` re-exports it with `LodSimplified` and `LodStatistical`. It counts only on an
/// entity that carries [`StableEntity`] (#31 m2).
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LodComplete;

// spec: revues/architecture.md §8.1, §6.2 ; README.md
/// Clock of the world, read by the systems: start of the current tick and origin of the
/// compact days (AS-22).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimClock {
    now: SimTime,
    tick_ms: i64,
    day_origin: DayOrigin,
}

impl SimClock {
    /// Start of the current tick.
    pub const fn now(&self) -> SimTime {
        self.now
    }

    /// Duration of a tick, in milliseconds.
    pub const fn tick_millis(&self) -> i64 {
        self.tick_ms
    }

    /// Origin of the compact days.
    pub const fn day_origin(&self) -> DayOrigin {
        self.day_origin
    }
}

// spec: revues/architecture.md §8.1
/// Dated event of the kernel. Its shape is provisional: `ts_model` will fix the event kinds
/// (end of schedule blocks, timetable, milestones, timers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimedEvent {
    /// Kind of the event, assigned by the plugin that handles it.
    pub kind: u32,
    /// Entity concerned.
    pub subject: StableId,
    /// Free argument.
    pub arg: u64,
}

// spec: revues/architecture.md §8.1, §6.5
/// Resource: the scheduler of the world.
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
/// Resource: events due during the current tick, in `(at, seq)` order. Filled before the
/// [`Phase::Scheduled`] phase, emptied at the end of the tick.
#[derive(Resource, Debug, Default)]
pub struct DueEvents(Vec<Scheduled<TimedEvent>>);

impl Deref for DueEvents {
    type Target = [Scheduled<TimedEvent>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// spec: revues/architecture.md §6.1 ; carnet architecture-securite.md §9.10 L5
/// Resource: the allocator of the world's [`StableId`]s.
#[derive(Resource, Debug)]
pub struct StableIds(StableIdAllocator);

impl StableIds {
    /// New identifier, or `None` if the namespace is exhausted.
    pub const fn allocate(&mut self) -> Option<StableId> {
        self.0.allocate()
    }
}

// spec: revues/architecture.md §6.4 ; carnet architecture-securite.md §9.10 L4
/// Resource: the stream keys, derived from the world key at build time, and the draw sites
/// declared by the active plugins. The keys are never recorded or hashed; the world key is
/// not kept.
///
/// Systems draw through [`Self::draw`], [`Self::draw_block`] and [`Self::draws`], or
/// through [`crate::rng::SiteDraw`] in code generic over the holder of the keys (a generator
/// run while building). In debug builds (every test), every one of these panics on a site
/// that no active plugin declares ([`Plugin::draw_sites`]), so that the uniqueness check of
/// the build covers every draw of the rules (#26 m1). The keys themselves are never handed
/// out: no `Deref` to [`StreamKeys`], whose draws check nothing. Witness, then the refusal:
///
/// ```
/// fn draws_here(s: &ts_kernel::WorldStreams) -> &impl ts_kernel::rng::SiteDraw {
///     s
/// }
/// ```
///
/// ```compile_fail,E0308
/// fn keys(s: &ts_kernel::WorldStreams) -> &ts_kernel::rng::StreamKeys {
///     s
/// }
/// ```
#[derive(Resource, Debug)]
pub struct WorldStreams {
    keys: StreamKeys,
    sites: BTreeSet<DrawSite>,
}

impl WorldStreams {
    /// First value of a declared `site`, as [`StreamKeys::draw`].
    #[must_use]
    pub fn draw(&self, site: DrawSite, subject: StableId, slot: u64) -> u64 {
        self.declared(site);
        self.keys.draw(site, subject, slot)
    }

    /// Block `block` of a declared `site`, as [`StreamKeys::draw_block`].
    #[must_use]
    pub fn draw_block(&self, site: DrawSite, subject: StableId, slot: u64, block: u16) -> [u64; 8] {
        self.declared(site);
        self.keys.draw_block(site, subject, slot, block)
    }

    /// Successive values of a declared `site`, as [`StreamKeys::draws`].
    pub fn draws(&self, site: DrawSite, subject: StableId, slot: u64) -> Draws<'_> {
        self.declared(site);
        self.keys.draws(site, subject, slot)
    }

    /// Cosmetic seed, see [`StreamKeys::cosmetic_seed`].
    pub fn cosmetic_seed(&self, usage: &'static str, subject: StableId, slot: u64) -> CosmeticSeed {
        self.keys.cosmetic_seed(usage, subject, slot)
    }

    /// The keys, for the tests of the kernel only.
    #[cfg(test)]
    pub(crate) const fn keys(&self) -> &StreamKeys {
        &self.keys
    }

    fn declared(&self, site: DrawSite) {
        debug_assert!(
            self.sites.contains(&site),
            "draw site {} is declared by no active plugin (Plugin::draw_sites)",
            site.name()
        );
    }
}


// spec: revues/architecture.md §5.3, §7.1, §9.2
/// Failure to build a [`Sim`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// `tick_hz` is zero or does not divide 1,000.
    InvalidTickHz(u32),
    /// Two plugins carry this name.
    DuplicatePlugin(&'static str),
    /// A disabled plugin is not in the rules.
    UnknownDisabledPlugin(&'static str),
    /// A mandatory plugin is disabled.
    NotDisableable(&'static str),
    /// A plugin depends on a plugin absent from the rules.
    MissingDependency {
        /// Plugin that declares the dependency.
        plugin: &'static str,
        /// Required plugin.
        dependency: &'static str,
    },
    /// An active plugin depends on a disabled plugin.
    DisabledDependency {
        /// Plugin that declares the dependency.
        plugin: &'static str,
        /// Required plugin, disabled.
        dependency: &'static str,
    },
    /// The process's thread pool already has another number of threads.
    ThreadPoolMismatch {
        /// Threads requested.
        wanted: usize,
        /// Threads of the existing pool.
        actual: usize,
    },
    /// The `Schedule` of a phase cannot be built (ordering ambiguity, cycle...).
    Schedule {
        /// Phase concerned.
        phase: Phase,
        /// Message from `bevy_ecs`.
        message: String,
    },
    /// A plugin depends on a plugin that comes later in the rules.
    DependencyOrder {
        /// Plugin that declares the dependency.
        plugin: &'static str,
        /// Required plugin, placed after it.
        dependency: &'static str,
    },
    /// Two draw sites of the rules share a stream and a purpose (#26 m1).
    DrawSiteConflict {
        /// Site listed first, and its plugin.
        first: (&'static str, DrawSite),
        /// Site listed next, with the same stream and purpose, and its plugin.
        second: (&'static str, DrawSite),
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
            Self::InvalidTickHz(hz) => write!(f, "invalid tick_hz: {hz} does not divide 1000"),
            Self::DuplicatePlugin(n) => write!(f, "duplicate plugin: {n}"),
            Self::UnknownDisabledPlugin(n) => write!(f, "unknown disabled plugin: {n}"),
            Self::NotDisableable(n) => write!(f, "plugin {n} cannot be disabled"),
            Self::MissingDependency { plugin, dependency } => {
                write!(
                    f,
                    "plugin {plugin} depends on {dependency}, which is absent"
                )
            }
            Self::DisabledDependency { plugin, dependency } => {
                write!(
                    f,
                    "plugin {plugin} depends on {dependency}, which is disabled"
                )
            }
            Self::ThreadPoolMismatch { wanted, actual } => {
                write!(
                    f,
                    "{wanted} threads requested, the process already has {actual}"
                )
            }
            Self::Schedule { phase, message } => write!(f, "phase {phase:?}: {message}"),
            Self::DependencyOrder { plugin, dependency } => {
                write!(
                    f,
                    "plugin {plugin} depends on {dependency}, which comes after it"
                )
            }
            Self::DrawSiteConflict { first, second } => write!(
                f,
                "draw site {} of plugin {} reuses stream {:?} and purpose {} of site {} of plugin {}",
                second.1.name(),
                second.0,
                second.1.stream(),
                second.1.purpose(),
                first.1.name(),
                first.0
            ),
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
/// Stop condition of [`Sim::run_until`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopCondition {
    /// Stop at the first tick start at or after this date.
    At(SimTime),
}

/// Outcome of a tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepReport {
    /// Start of the executed tick.
    pub tick_start: SimTime,
    /// Due events handled by the tick.
    pub due_events: usize,
}

/// Outcome of [`Sim::run_until`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunReport {
    /// Ticks executed.
    pub ticks: u64,
    /// Ticks skipped without execution (time skip).
    pub skipped_ticks: u64,
}

// spec: revues/architecture.md §7.1, §8.1, §8.2, §9.1
/// The simulation: a `bevy_ecs` world, one `Schedule` per phase and the clock.
pub struct Sim {
    world: World,
    phases: Vec<Schedule>,
    /// Indexed like `phases`: with [`Threads::Fixed`], the executor switch of the phase and
    /// its number of systems; `None` with [`Threads::Single`].
    switches: Vec<Option<(ExecutorSwitch, usize)>>,
    parallel_phases: ParallelPhases,
    tick_ms: i64,
    active: Vec<(&'static str, u32)>,
    rules: RulesFingerprint,
    /// Stable entities in full simulation: the only ones the fingerprint sees (#31 m2).
    complete: QueryState<Entity, (With<LodComplete>, With<StableEntity>)>,
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

/// Sets the global thread pool, or checks that it already has the right number of threads.
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

/// Checks the names and the disablings, and returns the active plugins, in rules order.
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
    check_draw_sites(rules)?;
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

// spec: carnet architecture-securite.md §9.9 K3
/// Refuses two draw sites of `rules` (active or not) with the same stream and purpose.
fn check_draw_sites(rules: &[Box<dyn Plugin>]) -> Result<(), BuildError> {
    let mut seen: BTreeMap<(Stream, u32), (&'static str, DrawSite)> = BTreeMap::new();
    for p in rules {
        for &site in p.draw_sites() {
            let second = (p.name(), site);
            if let Some(&first) = seen.get(&(site.stream(), site.purpose())) {
                return Err(BuildError::DrawSiteConflict { first, second });
            }
            seen.insert((site.stream(), site.purpose()), second);
        }
    }
    Ok(())
}

impl Sim {
    /// Builds a new world at the epoch, with the `rules` plugins minus those that `cfg`
    /// disables. The stream keys are derived from `world_key`, which is not kept.
    ///
    /// # Errors
    ///
    /// See [`BuildError`]: invalid configuration, disabled or absent dependency, incompatible
    /// thread pool, ordering ambiguity within a phase.
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
        world.insert_resource(WorldStreams {
            keys: StreamKeys::derive(world_key),
            sites: plugins
                .iter()
                .flat_map(|p| p.draw_sites())
                .copied()
                .collect(),
        });

        let (phases, switches): (Vec<_>, Vec<_>) = Phase::ALL
            .iter()
            .map(|&p| phase_schedule(p, cfg.threads))
            .unzip();
        let mut b = SimBuilder {
            world,
            phases,
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
        // After `initialize`, so that the count includes the sync points that `bevy_ecs`
        // inserted: an upper bound of the systems that can run together (speed only).
        let switches = switches
            .into_iter()
            .zip(&phases)
            .map(|(switch, schedule)| switch.map(|s| (s, schedule.systems_len())))
            .collect();
        let active: Vec<_> = plugins.iter().map(|p| (p.name(), p.version())).collect();
        let rules = RulesFingerprint::of(&active);
        let complete = world.query_filtered::<Entity, (With<LodComplete>, With<StableEntity>)>();
        Ok(Self {
            world,
            phases,
            switches,
            parallel_phases: cfg.parallel_phases,
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
    /// State fingerprints, per family and global. The `kernel` family covers the clock, the
    /// day origin, the [`StableId`] allocator and the pending events; the others, the state
    /// types declared by plugins. The stream keys do not enter it.
    pub fn state_hash(&self) -> StateHashes {
        hash_world(&self.world, &self.state, &|w| write_kernel(&self.world, w))
    }

    /// Start of the next tick: the current date of the world.
    pub fn now(&self) -> SimTime {
        self.world.resource::<SimClock>().now
    }

    /// Duration of a tick, in milliseconds.
    pub const fn tick_millis(&self) -> i64 {
        self.tick_ms
    }

    /// Fingerprint of the active rules.
    pub const fn rules_fingerprint(&self) -> RulesFingerprint {
        self.rules
    }

    /// Active plugins `(name, version)`, in rules order.
    pub fn active_plugins(&self) -> &[(&'static str, u32)] {
        &self.active
    }

    /// The world, read-only (architecture §7.3: viewer and tests).
    pub const fn world(&self) -> &World {
        &self.world
    }

    // spec: revues/architecture.md §8.1, §8.2, §9.1
    /// Runs a tick: `Input`, then the events due in `[now, now + tick[` move into
    /// [`DueEvents`], then the nine other phases; finally the clock advances by one tick.
    pub fn step(&mut self) -> StepReport {
        let tick_start = self.now();
        let last = tick_start.add_millis(self.tick_ms - 1);
        self.run_phase(Phase::Input, 0);
        let due = self.world.resource_mut::<EventQueue>().pop_due(last);
        let due_events = due.len();
        self.world.resource_mut::<DueEvents>().0 = due;
        for &phase in &Phase::ALL[Phase::Scheduled.index()..] {
            self.run_phase(phase, due_events);
        }
        self.world.resource_mut::<DueEvents>().0.clear();
        self.world.resource_mut::<SimClock>().now = last.add_millis(1);
        self.world.clear_trackers();
        StepReport {
            tick_start,
            due_events,
        }
    }

    // spec: revues/architecture.md §9.1, §9.2
    /// Runs one phase. With [`Threads::Fixed`], first picks its executor (speed only, see
    /// [`ParallelPhases`]) from the configuration and the world state alone: nothing is
    /// carried from one tick to the next.
    fn run_phase(&mut self, phase: Phase, due_events: usize) {
        if let Some((switch, systems)) = &self.switches[phase.index()] {
            let parallel = match self.parallel_phases {
                ParallelPhases::Always => true,
                ParallelPhases::FromWork(from) => {
                    *systems >= 2 && {
                        // Exact and cheap: both `With` filters are archetype filters, so the
                        // lower bound is the sum of the matched table sizes.
                        let complete = self.complete.iter(&self.world).size_hint().0;
                        complete.saturating_add(due_events) >= from as usize
                    }
                }
            };
            switch.set_parallel(parallel);
        }
        self.phases[phase.index()].run(&mut self.world);
    }

    /// True if a stable entity carries [`LodComplete`]. Only stable entities count, as in the
    /// fingerprint and the snapshot: a marker on another entity, which a restore drops, must
    /// not change the choice between ticking and skipping (#31 m2).
    fn has_complete_entities(&mut self) -> bool {
        self.complete.iter(&self.world).next().is_some()
    }

    /// First tick start at or before `t`. Only called with `t > now()`, so the result is
    /// at least `now()` and cannot underflow.
    fn tick_floor(&self, t: SimTime) -> SimTime {
        let ms = t.as_millis();
        SimTime::from_millis(ms - ms.rem_euclid(self.tick_ms))
    }

    // spec: revues/architecture.md §8.1, §8.5
    /// Advances to the first tick start at or after the stop date.
    ///
    /// **Time skip**: while no stable entity carries [`LodComplete`], the loop stops ticking and
    /// jumps straight to the tick that contains the next event. The skipped ticks are empty:
    /// an event is therefore handled in the same tick, with or without a skip.
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
