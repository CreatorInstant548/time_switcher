use super::*;
use bevy_ecs::prelude::{IntoScheduleConfigs, Res, ResMut};

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
    fn build(&self, b: &mut SimBuilder) {
        for &d in self.deps {
            b.depends_on(d);
        }
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
    let report = sim.step();
    assert_eq!(report.tick_start, SimTime::EPOCH);
    assert_eq!(sim.now(), SimTime::from_millis(50));
    assert_eq!(sim.world().resource::<PhaseLog>().0, Phase::ALL);
}

/// Événements vus par la phase `Scheduled` : (début du tick, date, rang, genre).
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
            q.schedule(e.at.add_millis(10_000), next);
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
        queue.schedule(
            SimTime::from_millis(at),
            TimedEvent {
                kind,
                subject,
                arg: 0,
            },
        );
    }
    b.add_systems(Phase::Scheduled, record);
}

#[test]
fn due_events_are_those_of_the_tick_in_order() {
    let mut sim = build(&[plugin("timer", timer_setup)]).unwrap();
    assert_eq!(sim.step().due_events, 3);
    assert_eq!(sim.step().due_events, 1);
    assert_eq!(sim.step().due_events, 1);
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
    b.world_mut().spawn(LodComplete);
}

/// Avec ou sans saut de temps, les mêmes événements sont traités dans les mêmes ticks.
#[test]
fn time_skip_matches_ticking() {
    let stop = StopCondition::At(SimTime::from_millis(60_000));
    let mut skipping = build(&[plugin("timer", timer_setup)]).unwrap();
    let skip = skipping.run_until(stop);
    let mut ticking =
        build(&[plugin("timer", timer_setup), plugin("full", complete_setup)]).unwrap();
    let tick = ticking.run_until(stop);

    assert_eq!(skipping.now(), SimTime::from_millis(60_000));
    assert_eq!(ticking.now(), SimTime::from_millis(60_000));
    assert_eq!(
        tick,
        RunReport {
            ticks: 1200,
            skipped_ticks: 0
        }
    );
    // Ticks 0, 50, 100 (120), 10 100 (relance à 10 120) et 30 000.
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
    let r = sim.run_until(StopCondition::At(SimTime::from_millis(1001)));
    assert_eq!(sim.now(), SimTime::from_millis(1050));
    assert_eq!(
        r,
        RunReport {
            ticks: 0,
            skipped_ticks: 21
        }
    );
    let r = sim.run_until(StopCondition::At(SimTime::from_millis(10)));
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

/// L7 : deux écritures de la même ressource sans ordre, dans une phase, sont refusées.
#[test]
fn unordered_writes_are_rejected() {
    let ambiguous = [plugin("amb", |b| {
        b.insert_resource(Counter::default());
        b.add_systems(Phase::Act, (add_one, double));
    })];
    match build(&ambiguous).unwrap_err() {
        BuildError::Schedule { phase, message } => {
            assert_eq!(phase, Phase::Act);
            // Les noms des systèmes demandent la fonctionnalité `debug`, refusée (L1).
            assert!(message.contains("conflicting data access"), "{message}");
        }
        e => panic!("erreur inattendue : {e}"),
    }
    let ordered = [plugin("ord", |b| {
        b.insert_resource(Counter::default());
        b.add_systems(Phase::Act, (add_one, double).chain());
        b.add_systems(Phase::Consequences, add_one);
    })];
    let mut sim = build(&ordered).unwrap();
    sim.step();
    assert_eq!(sim.world().resource::<Counter>().0, 3);
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

/// Les tests de cette crate tournent tous à un fil ; une autre demande est refusée.
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
