use super::*;
use bevy_ecs::prelude::{IntoScheduleConfigs, Res, ResMut};

use crate::rng::SiteDraw;

struct TestPlugin {
    name: &'static str,
    version: u32,
    disableable: bool,
    deps: &'static [&'static str],
    setup: fn(&mut SimBuilder),
}

impl Plugin for TestPlugin {
    fn name(&self) -> &'static str {
        self.name
    }
    fn version(&self) -> u32 {
        self.version
    }
    fn disableable(&self) -> bool {
        self.disableable
    }
    fn dependencies(&self) -> &'static [&'static str] {
        self.deps
    }
    fn build(&self, b: &mut SimBuilder) {
        (self.setup)(b);
    }
}

fn plugin(name: &'static str, setup: fn(&mut SimBuilder)) -> Box<dyn Plugin> {
    Box::new(TestPlugin {
        name,
        version: 1,
        disableable: false,
        deps: &[],
        setup,
    })
}

fn key() -> WorldKey {
    WorldKey::insecure_from_u64_for_tests(7)
}

fn build(rules: &[Box<dyn Plugin>]) -> Result<Sim, BuildError> {
    Sim::build(SimConfig::default(), rules, &key())
}

fn build_with(disabled: &[&'static str], rules: &[Box<dyn Plugin>]) -> Result<Sim, BuildError> {
    let cfg = SimConfig {
        disabled_plugins: disabled.to_vec(),
        ..SimConfig::default()
    };
    Sim::build(cfg, rules, &key())
}

#[derive(Resource, Default)]
struct PhaseLog(Vec<Phase>);

#[test]
fn phases_run_in_order_once_per_tick() {
    let rules = [plugin("order", |b| {
        b.insert_resource(PhaseLog::default());
        for p in Phase::ALL {
            b.add_systems(p, move |mut log: ResMut<PhaseLog>| log.0.push(p));
        }
    })];
    let mut sim = build(&rules).unwrap();
    assert_eq!(sim.now(), SimTime::EPOCH);
    let report = sim.step().unwrap();
    assert_eq!(report.tick_start, SimTime::EPOCH);
    assert_eq!(sim.now(), SimTime::from_millis(50));
    assert_eq!(sim.world().resource::<PhaseLog>().0, Phase::ALL);
}

/// Events seen by the `Scheduled` phase: (tick start, date, rank, kind).
#[derive(Resource, Default)]
struct Seen(Vec<(i64, i64, u64, u32)>);

const RESCHEDULE: u32 = 1;

fn record(
    clock: Res<SimClock>,
    due: Res<DueEvents>,
    mut q: ResMut<EventQueue>,
    mut seen: ResMut<Seen>,
) {
    for e in due.iter() {
        let at = e.at.as_millis();
        seen.0
            .push((clock.now().as_millis(), at, e.seq, e.event.kind));
        if e.event.kind == RESCHEDULE {
            let next = TimedEvent { kind: 2, ..e.event };
            q.schedule(e.at.add_millis(10_000), next).unwrap();
        }
    }
}

fn timer_setup(b: &mut SimBuilder) {
    b.insert_resource(Seen::default());
    let subject = StableId::from_snapshot_raw(3);
    let mut queue = b.world_mut().resource_mut::<EventQueue>();
    for (at, kind) in [
        (0, 0),
        (49, 0),
        (50, 0),
        (120, RESCHEDULE),
        (30_000, 0),
        (49, 0),
    ] {
        queue
            .schedule(
                SimTime::from_millis(at),
                TimedEvent {
                    kind,
                    subject,
                    arg: 0,
                },
            )
            .unwrap();
    }
    b.add_systems(Phase::Scheduled, record);
}

#[test]
fn due_events_are_those_of_the_tick_in_order() {
    let mut sim = build(&[plugin("timer", timer_setup)]).unwrap();
    assert_eq!(sim.step().unwrap().due_events, 3);
    assert_eq!(sim.step().unwrap().due_events, 1);
    assert_eq!(sim.step().unwrap().due_events, 1);
    assert_eq!(
        sim.world().resource::<Seen>().0,
        [
            (0, 0, 0, 0),
            (0, 49, 1, 0),
            (0, 49, 5, 0),
            (50, 50, 2, 0),
            (100, 120, 3, RESCHEDULE)
        ]
    );
    assert!(sim.world().resource::<DueEvents>().is_empty());
}

fn complete_setup(b: &mut SimBuilder) {
    let id = b
        .world_mut()
        .resource_mut::<StableIds>()
        .allocate()
        .unwrap();
    b.world_mut().spawn((StableEntity::new(id), LodComplete));
}

fn unstable_complete_setup(b: &mut SimBuilder) {
    b.world_mut().spawn(LodComplete);
}

/// #31 m2: a full-simulation marker on an entity with no durable identity, which the
/// fingerprint does not see and a restore drops, does not stop the time skip.
#[test]
fn only_stable_entities_stop_the_time_skip() {
    let stop = StopCondition::At(SimTime::from_millis(60_000));
    let mut alone = build(&[plugin("timer", timer_setup)]).unwrap();
    let mut marked = build(&[
        plugin("timer", timer_setup),
        plugin("unstable", unstable_complete_setup),
    ])
    .unwrap();
    assert_eq!(
        alone.run_until(stop).unwrap(),
        marked.run_until(stop).unwrap()
    );
    assert_eq!(alone.state_hash(), marked.state_hash());
}

/// With or without a time skip, the same events are handled in the same ticks.
#[test]
fn time_skip_matches_ticking() {
    let stop = StopCondition::At(SimTime::from_millis(60_000));
    let mut skipping = build(&[plugin("timer", timer_setup)]).unwrap();
    let skip = skipping.run_until(stop).unwrap();
    let mut ticking =
        build(&[plugin("timer", timer_setup), plugin("full", complete_setup)]).unwrap();
    let tick = ticking.run_until(stop).unwrap();

    assert_eq!(skipping.now(), SimTime::from_millis(60_000));
    assert_eq!(ticking.now(), SimTime::from_millis(60_000));
    assert_eq!(
        tick,
        RunReport {
            ticks: 1200,
            skipped_ticks: 0
        }
    );
    // Ticks 0, 50, 100 (120), 10,100 (restart at 10,120) and 30,000.
    assert_eq!(
        skip,
        RunReport {
            ticks: 5,
            skipped_ticks: 1195
        }
    );
    let seen = &skipping.world().resource::<Seen>().0;
    assert_eq!(seen, &ticking.world().resource::<Seen>().0);
    assert_eq!(seen[5], (10_100, 10_120, 6, 2));
    assert_eq!(seen.len(), 7);
}

#[test]
fn run_until_stops_at_next_tick_boundary() {
    let mut sim = build(&[]).unwrap();
    let r = sim
        .run_until(StopCondition::At(SimTime::from_millis(1001)))
        .unwrap();
    assert_eq!(sim.now(), SimTime::from_millis(1050));
    assert_eq!(
        r,
        RunReport {
            ticks: 0,
            skipped_ticks: 21
        }
    );
    let r = sim
        .run_until(StopCondition::At(SimTime::from_millis(10)))
        .unwrap();
    assert_eq!(r, RunReport::default());
    assert_eq!(sim.now(), SimTime::from_millis(1050));
}

fn nothing(_: &mut SimBuilder) {}

fn with(name: &'static str, disableable: bool, deps: &'static [&'static str]) -> Box<dyn Plugin> {
    Box::new(TestPlugin {
        name,
        version: 2,
        disableable,
        deps,
        setup: nothing,
    })
}

#[test]
fn cut_dependency_fails_the_build() {
    let rules = [with("a", true, &[]), with("b", false, &["a"])];
    assert!(build(&rules).is_ok());
    assert_eq!(
        build_with(&["a"], &rules).unwrap_err(),
        BuildError::DisabledDependency {
            plugin: "b",
            dependency: "a"
        }
    );
    assert_eq!(
        build(&[with("b", false, &["a"])]).unwrap_err(),
        BuildError::MissingDependency {
            plugin: "b",
            dependency: "a"
        }
    );
}

#[test]
fn invalid_plugin_lists_fail_the_build() {
    let rules = [with("a", true, &[]), with("b", false, &[])];
    assert_eq!(
        build_with(&["b"], &rules).unwrap_err(),
        BuildError::NotDisableable("b")
    );
    assert_eq!(
        build_with(&["zzz"], &rules).unwrap_err(),
        BuildError::UnknownDisabledPlugin("zzz")
    );
    assert_eq!(
        build(&[with("a", true, &[]), with("a", false, &[])]).unwrap_err(),
        BuildError::DuplicatePlugin("a")
    );
}

#[test]
fn rules_fingerprint_covers_active_plugins_only() {
    let rules = [with("b", false, &[]), with("a", true, &[])];
    let all = build(&rules).unwrap();
    assert_eq!(all.active_plugins(), [("b", 2), ("a", 2)]);
    assert_eq!(
        all.rules_fingerprint(),
        RulesFingerprint::of(&[("a", 2), ("b", 2)])
    );
    let cut = build_with(&["a"], &rules).unwrap();
    assert_eq!(cut.rules_fingerprint(), RulesFingerprint::of(&[("b", 2)]));
}

#[derive(Resource, Default)]
struct Counter(u32);

fn add_one(mut c: ResMut<Counter>) {
    c.0 += 1;
}

fn double(mut c: ResMut<Counter>) {
    c.0 *= 2;
}

/// L7: two writes of the same resource with no order, in one phase, are refused.
#[test]
fn unordered_writes_are_rejected() {
    let ambiguous = [plugin("amb", |b| {
        b.insert_resource(Counter::default());
        b.add_systems(Phase::Act, (add_one, double));
    })];
    match build(&ambiguous).unwrap_err() {
        BuildError::Schedule { phase, message } => {
            assert_eq!(phase, Phase::Act);
            // System names require the `debug` feature, which is refused (L1).
            assert!(message.contains("conflicting data access"), "{message}");
        }
        e => panic!("unexpected error: {e}"),
    }
    let ordered = [plugin("ord", |b| {
        b.insert_resource(Counter::default());
        b.add_systems(Phase::Act, (add_one, double).chain());
        b.add_systems(Phase::Consequences, add_one);
    })];
    let mut sim = build(&ordered).unwrap();
    sim.step().unwrap();
    assert_eq!(sim.world().resource::<Counter>().0, 3);
}

/// Two unordered writers of `Counter` in one phase, or one opt-out of the detection.
enum Ambiguity {
    Unordered,
    WithAll,
    WorldResource,
}

struct Ambiguous(Phase, Ambiguity);

impl Plugin for Ambiguous {
    fn name(&self) -> &'static str {
        "ambiguous"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.insert_resource(Counter::default());
        match self.1 {
            Ambiguity::Unordered => {
                b.add_systems(self.0, (add_one, double));
            }
            Ambiguity::WithAll => {
                b.add_systems(self.0, (add_one.ambiguous_with_all(), double));
            }
            Ambiguity::WorldResource => {
                b.world_mut().allow_ambiguous_resource::<Counter>();
                b.add_systems(self.0, (add_one, double));
            }
        }
    }
}

/// L7 in **every** phase: unordered writes and opt-outs are refused (T6a review m3).
#[test]
fn ambiguities_are_refused_in_every_phase() {
    for phase in Phase::ALL {
        let unordered: [Box<dyn Plugin>; 1] = [Box::new(Ambiguous(phase, Ambiguity::Unordered))];
        assert!(
            matches!(build(&unordered), Err(BuildError::Schedule { phase: p, .. }) if p == phase),
            "{phase:?}"
        );
        let with_all: [Box<dyn Plugin>; 1] = [Box::new(Ambiguous(phase, Ambiguity::WithAll))];
        assert_eq!(
            build(&with_all).unwrap_err(),
            BuildError::AmbiguityOptOut { phase: Some(phase) }
        );
        let world: [Box<dyn Plugin>; 1] = [Box::new(Ambiguous(phase, Ambiguity::WorldResource))];
        assert_eq!(
            build(&world).unwrap_err(),
            BuildError::AmbiguityOptOut { phase: None }
        );
    }
}

/// A plugin listed before its dependency is refused before any `build` runs, so a `build`
/// that reads the dependency's resource never panics (T6a review m5).
#[test]
fn dependency_must_come_first() {
    let reads_counter = Box::new(TestPlugin {
        name: "reader",
        version: 1,
        disableable: false,
        deps: &["counter"],
        setup: |b| {
            let _ = b.world_mut().resource::<Counter>().0;
        },
    });
    let owner = plugin("counter", |b| {
        b.insert_resource(Counter::default());
    });
    assert_eq!(
        build(&[reads_counter, owner]).unwrap_err(),
        BuildError::DependencyOrder {
            plugin: "reader",
            dependency: "counter"
        }
    );
}

/// `run_until` never panics at the bounds of `i64` (T6a review m2), and never takes the
/// clock beyond the saved range (#113): a stop date whose tick would end beyond 2^62 ms is
/// refused before anything runs, never clamped.
#[test]
fn run_until_handles_extreme_dates() {
    let at = |ms| StopCondition::At(SimTime::from_millis(ms));
    let max = SimTime::MAX_SAVED_ABS_MILLIS;
    let edge = max - max % 50;
    assert!(
        edge < max,
        "the grid must leave a gap below 2^62 for this test"
    );
    let mut sim = build(&[]).unwrap();
    assert_eq!(sim.run_until(at(i64::MIN)), Ok(RunReport::default()));
    let before = sim.state_hash();
    // `edge + 1` is savable, but its tick ends at `edge + 50`, beyond the bound.
    for (far, requested) in [
        (i64::MAX, i64::MAX),
        (i64::MAX - 1, i64::MAX - 1),
        (max + 1, max + 1),
        (edge + 1, edge + 50),
    ] {
        assert_eq!(
            sim.run_until(at(far)),
            Err(ClockOutOfRange {
                now: SimTime::EPOCH,
                requested: SimTime::from_millis(requested),
            }),
            "{far}"
        );
        assert_eq!(sim.now(), SimTime::EPOCH);
        assert_eq!(sim.state_hash(), before);
    }
    // The last tick start within the bound is reached, by the time skip.
    let r = sim.run_until(at(edge)).unwrap();
    assert_eq!((r.ticks, sim.now()), (0, SimTime::from_millis(edge)));
    assert_eq!(sim.run_until(at(edge)), Ok(RunReport::default()));
    let before = sim.state_hash();
    assert_eq!(
        sim.run_until(at(max)),
        Err(ClockOutOfRange {
            now: SimTime::from_millis(edge),
            requested: SimTime::from_millis(edge + 50),
        })
    );
    assert_eq!(
        sim.step(),
        Err(ClockOutOfRange {
            now: SimTime::from_millis(edge),
            requested: SimTime::from_millis(edge + 50),
        })
    );
    assert_eq!(sim.now(), SimTime::from_millis(edge));
    assert_eq!(sim.state_hash(), before);
}

#[test]
fn tick_hz_must_divide_1000() {
    for hz in [0, 7, 3000] {
        let cfg = SimConfig {
            tick_hz: hz,
            ..SimConfig::default()
        };
        assert_eq!(
            Sim::build(cfg, &[], &key()).unwrap_err(),
            BuildError::InvalidTickHz(hz)
        );
    }
    let cfg = SimConfig {
        tick_hz: 1000,
        ..SimConfig::default()
    };
    assert_eq!(Sim::build(cfg, &[], &key()).unwrap().tick_millis(), 1);
}

/// The tests of this crate all run on one thread; a request for another is refused.
#[test]
fn thread_pool_is_fixed_per_process() {
    build(&[]).unwrap();
    let cfg = SimConfig {
        threads: Threads::Fixed(NonZeroU16::new(3).unwrap()),
        ..SimConfig::default()
    };
    assert_eq!(
        Sim::build(cfg, &[], &key()).unwrap_err(),
        BuildError::ThreadPoolMismatch {
            wanted: 3,
            actual: 1
        }
    );
}

struct Drawing(&'static str, &'static [DrawSite]);

impl Plugin for Drawing {
    fn name(&self) -> &'static str {
        self.0
    }
    fn version(&self) -> u32 {
        1
    }
    fn disableable(&self) -> bool {
        true
    }
    fn draw_sites(&self) -> &'static [DrawSite] {
        self.1
    }
    fn build(&self, _: &mut SimBuilder) {}
}

const LOCK: DrawSite = DrawSite::new(Stream::CityServer, 1, "city.lock");
const CAMERA: DrawSite = DrawSite::new(Stream::CityServer, 1, "city.camera");
const HIT: DrawSite = DrawSite::new(Stream::Combat, 1, "combat.hit");

/// #26 m1: two sites of the rules with the same stream and purpose fail the build, even
/// when one plugin is disabled; the same purpose in another stream is allowed.
#[test]
fn draw_sites_are_unique_in_the_rules() {
    let ok: Vec<Box<dyn Plugin>> = vec![
        Box::new(Drawing("locks", &[LOCK])),
        Box::new(Drawing("combat", &[HIT])),
    ];
    assert!(build(&ok).is_ok());
    let conflict = BuildError::DrawSiteConflict {
        first: ("locks", LOCK),
        second: ("cameras", CAMERA),
    };
    let rules: Vec<Box<dyn Plugin>> = vec![
        Box::new(Drawing("locks", &[LOCK])),
        Box::new(Drawing("cameras", &[HIT, CAMERA])),
    ];
    assert_eq!(build(&rules).unwrap_err(), conflict);
    assert_eq!(build_with(&["cameras"], &rules).unwrap_err(), conflict);
    assert_eq!(
        conflict.to_string(),
        "draw site city.camera of plugin cameras reuses stream CityServer and purpose 1 of \
         site city.lock of plugin locks"
    );
    let twice: Vec<Box<dyn Plugin>> = vec![Box::new(Drawing("locks", &[LOCK, LOCK]))];
    assert!(matches!(
        build(&twice),
        Err(BuildError::DrawSiteConflict { .. })
    ));
}

/// #26 m1: a system draws only at a site that an active plugin declares; the world streams
/// refuse any other in debug builds, so the uniqueness check covers every draw.
#[test]
fn world_streams_draw_declared_sites_only() {
    let rules: Vec<Box<dyn Plugin>> = vec![Box::new(Drawing("combat", &[HIT]))];
    let sim = build(&rules).unwrap();
    let streams = sim.world().resource::<WorldStreams>();
    let subject = StableId::from_snapshot_raw(4);
    assert_eq!(
        streams.draw(HIT, subject, 9),
        streams.draws(HIT, subject, 9).next().unwrap()
    );
    assert_eq!(
        streams.draw(HIT, subject, 9),
        streams.draw_block(HIT, subject, 9, 0)[0]
    );
    let undeclared = std::panic::catch_unwind(|| streams.draw(LOCK, subject, 9));
    // Tests run with debug assertions (profiles `dev` and `ci`).
    assert!(undeclared.is_err());
}

/// The lock of the review of #109 (I1): a helper over any holder of keys, as a generator.
fn roll_lock(keys: &impl SiteDraw, who: StableId) -> u64 {
    keys.draw(LOCK, who, 0)
}

fn lock_system(streams: Res<WorldStreams>) {
    let _ = roll_lock(&*streams, StableId::from_snapshot_raw(4));
}

struct Locks;

impl Plugin for Locks {
    fn name(&self) -> &'static str {
        "locks"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.add_systems(Phase::Act, lock_system);
    }
}

/// #26 m1, review of #109 (I1): `cameras` declares (CityServer, 1); `locks` declares
/// nothing and draws `LOCK`, the same pair. Every public path from the world streams refuses
/// it in debug builds: the inherent draws, the sealed trait used by a generic helper, and a
/// system of the plugin. No path hands out the keys themselves (no `Deref`, see the
/// `compile_fail` of `WorldStreams`), so a lock can never reveal a camera.
#[test]
fn undeclared_sites_are_refused_on_every_path() {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    let rules: Vec<Box<dyn Plugin>> =
        vec![Box::new(Drawing("cameras", &[CAMERA])), Box::new(Locks)];
    let mut sim = build(&rules).unwrap();
    let who = StableId::from_snapshot_raw(4);
    let streams = sim.world().resource::<WorldStreams>();
    let _ = streams.draw(CAMERA, who, 0);
    let paths: [&dyn Fn(); 6] = [
        &|| {
            let _ = streams.draw(LOCK, who, 0);
        },
        &|| {
            let _ = streams.draw_block(LOCK, who, 0, 0);
        },
        &|| {
            let _ = streams.draws(LOCK, who, 0);
        },
        &|| {
            let _ = roll_lock(streams, who);
        },
        &|| {
            let _ = SiteDraw::draw_block(streams, LOCK, who, 0, 0);
        },
        &|| {
            let _ = SiteDraw::draws(streams, LOCK, who, 0);
        },
    ];
    // Tests run with debug assertions (profiles `dev` and `ci`).
    for (i, path) in paths.iter().enumerate() {
        assert!(catch_unwind(AssertUnwindSafe(path)).is_err(), "path {i}");
    }
    assert!(catch_unwind(AssertUnwindSafe(|| sim.step())).is_err());
}

// spec: revues/architecture.md §9.1, §9.2 ; carnet architecture-securite.md AS-57
/// No phase gets an automatic sync point (#93): a system with `Commands` ordered before
/// another adds none, so the phase holds exactly its own systems, whereas a schedule with the
/// default settings of `bevy_ecs` holds one more.
#[test]
fn phases_insert_no_sync_point() {
    use bevy_ecs::system::Commands;

    fn writes(mut commands: Commands) {
        commands.insert_resource(PhaseLog::default());
    }
    fn reads(_: Res<Counter>) {}
    fn writes_too(mut commands: Commands) {
        commands.insert_resource(PhaseLog::default());
    }
    let rules = [plugin("sync", |b| {
        b.insert_resource(Counter::default());
        b.add_systems(Phase::Act, (writes, reads.after(writes), writes_too));
    })];
    let sim = build(&rules).unwrap();
    assert_eq!(sim.phases[Phase::Act.index()].systems_len(), 3);
    let mut default = Schedule::new(Phase::Act);
    default.add_systems((writes, reads.after(writes), writes_too));
    default.initialize(&mut World::new()).unwrap();
    assert_eq!(
        default.systems_len(),
        4,
        "the default settings insert a sync point"
    );
}

/// State of the executor probe: ticks on which `Counter` was seen.
#[derive(Resource, Clone, Default)]
struct SeenCounter(u64);

impl StateHash for SeenCounter {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u64(self.0);
    }
}

impl crate::StateDecode for SeenCounter {
    fn decode_state(r: &mut crate::StateReader<'_>) -> Result<Self, crate::CanonicalError> {
        r.u64().map(Self)
    }
}

/// Buffer of the `Deferred<T>` form below: inserts `Counter` when applied.
#[derive(Default)]
struct InsertCounter;

impl bevy_ecs::system::SystemBuffer for InsertCounter {
    fn apply(&mut self, _: &bevy_ecs::system::SystemMeta, world: &mut World) {
        world.insert_resource(Counter(1));
    }

    fn queue(&mut self, _: &bevy_ecs::system::SystemMeta, _: bevy_ecs::world::DeferredWorld) {}
}

/// Builds `setup` under the single-threaded executor and under the parallel one (one thread,
/// every phase parallel): both results.
fn under_both_executors(setup: fn(&mut SimBuilder)) -> [Result<StateHashes, BuildError>; 2] {
    let one = NonZeroU16::new(1).unwrap();
    [
        (Threads::Single, ParallelPhases::default()),
        (Threads::Fixed(one), ParallelPhases::Always),
    ]
    .map(|(threads, parallel_phases)| {
        let cfg = SimConfig {
            threads,
            parallel_phases,
            ..SimConfig::default()
        };
        let mut sim = Sim::build(cfg, &[plugin("deferred_exclusive", setup)], &key())?;
        for _ in 0..5 {
            sim.step().unwrap();
        }
        Ok(sim.state_hash())
    })
}

// spec: revues/architecture.md §9.1, §9.2 ; carnet architecture-securite.md AS-57
/// Review of #93: an exclusive system with deferred buffers, through `pipe` with `Commands`
/// or with `Deferred<T>`, is refused at build time, identically under both executors (the
/// parallel one would apply its buffers mid-phase, the single-threaded one at the end).
/// Without the exclusive half, both executors give the same world.
#[test]
fn exclusive_systems_with_deferred_buffers_are_refused() {
    use bevy_ecs::system::{Commands, Deferred, In, IntoSystem};

    fn commands(mut c: Commands) {
        c.insert_resource(Counter(1));
    }
    fn deferred(mut d: Deferred<InsertCounter>) {
        let _ = &mut *d;
    }
    fn exclusive(_: In<()>, _: &mut World) {}
    fn reads(c: Option<Res<Counter>>, mut seen: ResMut<SeenCounter>) {
        seen.0 += u64::from(c.is_some());
    }
    let refused = Err(BuildError::DeferredExclusive { phase: Phase::Act });
    let piped: fn(&mut SimBuilder) = |b| {
        b.state_resource::<SeenCounter>("probe")
            .insert_resource(SeenCounter::default());
        b.add_systems(Phase::Act, (commands.pipe(exclusive), reads).chain());
    };
    assert_eq!(
        under_both_executors(piped),
        [refused.clone(), refused.clone()]
    );
    let buffered: fn(&mut SimBuilder) = |b| {
        b.state_resource::<SeenCounter>("probe")
            .insert_resource(SeenCounter::default());
        b.add_systems(Phase::Act, (deferred.pipe(exclusive), reads).chain());
    };
    assert_eq!(under_both_executors(buffered), [refused.clone(), refused]);
    let plain: fn(&mut SimBuilder) = |b| {
        b.state_resource::<SeenCounter>("probe")
            .insert_resource(SeenCounter::default());
        b.add_systems(Phase::Act, (commands, reads).chain());
    };
    let [single, parallel] = under_both_executors(plain);
    assert_eq!(single.unwrap(), parallel.unwrap());
}
