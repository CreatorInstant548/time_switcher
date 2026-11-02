// spec: revues/architecture.md §9.1, §9.2 ; carnet architecture-securite.md §9.10 L6, L7, AS-57
//! The scenario of issue #93 (AS-57 K-c): system `a`, with `Commands`, is ordered before `b`,
//! which reads `Foo`; `c` inserts `Foo` through `Commands`, unordered with both. With an
//! automatic sync point after `a`, `c`'s commands could be applied there or at the end of the
//! phase depending on timing, so `b` might see `Foo` on one run and not on the next. With
//! none (K-a), `b` never sees it within the phase, and the next phase always does.

use bevy_ecs::prelude::{Commands, Component, Entity, Has, Query, Res, ResMut, Resource, With};
use bevy_ecs::schedule::IntoScheduleConfigs;
use ts_kernel::{
    CanonicalError, ParallelPhases, Phase, Plugin, Sim, SimBuilder, SimClock, SimConfig,
    StableEntity, StableIds, StateDecode, StateHash, StateReader, StateWriter, Threads,
};

use super::{hex, key};

/// Entities of the scenario.
const ENTITIES: usize = 64;
/// Ticks of one run.
const TICKS: usize = 20;
/// Runs per thread count (AS-57: at least 200).
pub const RUNS: usize = 200;
/// Fingerprint of every run, whatever the number of threads.
pub const EXPECTED: &str = "ba5fa73ea7352b20e7c7b0aa9c80beb70d6e3564ab0e21bc52ff6bf589d950fd";

/// Inserted by `c` in `Act`, removed in `Output`.
#[derive(Component, Clone)]
struct Foo(i64);

impl StateHash for Foo {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.0);
    }
}

impl StateDecode for Foo {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.i64().map(Self)
    }
}

/// Inserted by `a`: its only role is to give `a` commands, hence a sync point after it
/// when automatic sync points are on.
#[derive(Component)]
struct Bar;

/// What the readers saw: `Foo` within the phase of its insertion (`b`), and in the next
/// phase (`e`).
#[derive(Resource, Clone, Default)]
struct Seen {
    in_phase: u64,
    next_phase: u64,
}

impl StateHash for Seen {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u64(self.in_phase);
        w.u64(self.next_phase);
    }
}

impl StateDecode for Seen {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Ok(Self {
            in_phase: r.u64()?,
            next_phase: r.u64()?,
        })
    }
}

fn a(mut commands: Commands, q: Query<Entity, With<StableEntity>>) {
    for e in &q {
        commands.entity(e).insert(Bar);
    }
}

fn b(q: Query<Has<Foo>, With<StableEntity>>, mut seen: ResMut<Seen>) {
    seen.in_phase += q.iter().filter(|&has| has).count() as u64;
}

fn c(mut commands: Commands, clock: Res<SimClock>, q: Query<Entity, With<StableEntity>>) {
    for e in &q {
        commands.entity(e).insert(Foo(clock.now().as_millis()));
    }
}

fn e(q: Query<&Foo>, mut seen: ResMut<Seen>) {
    seen.next_phase += q.iter().count() as u64;
}

fn d(mut commands: Commands, q: Query<Entity, With<Foo>>) {
    for e in &q {
        commands.entity(e).remove::<Foo>();
    }
}

struct DeferredWorld;

impl Plugin for DeferredWorld {
    fn name(&self) -> &'static str {
        "deferred_test"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, builder: &mut SimBuilder) {
        builder
            .state_component::<Foo>("test")
            .transient_component::<Bar>()
            .state_resource::<Seen>("test")
            .insert_resource(Seen::default());
        let world = builder.world_mut();
        for _ in 0..ENTITIES {
            let Some(id) = world.resource_mut::<StableIds>().allocate() else {
                break;
            };
            world.spawn(StableEntity::new(id));
        }
        builder.add_systems(Phase::Act, (a, b.after(a), c));
        builder.add_systems(Phase::Consequences, e);
        builder.add_systems(Phase::Output, d);
    }
}

/// One run of the scenario: `(fingerprint, Foo seen within its phase, Foo seen in the next)`.
fn run(threads: Threads) -> Result<(String, u64, u64), Box<dyn std::error::Error>> {
    let rules: Vec<Box<dyn Plugin>> = vec![Box::new(DeferredWorld)];
    let cfg = SimConfig {
        threads,
        parallel_phases: ParallelPhases::Always,
        ..SimConfig::default()
    };
    let mut sim = Sim::build(cfg, &rules, &key())?;
    for _ in 0..TICKS {
        sim.step();
    }
    let seen = sim.world().resource::<Seen>();
    Ok((hex(&sim), seen.in_phase, seen.next_phase))
}

/// [`RUNS`] runs with `threads`: each must give [`EXPECTED`], never show `Foo` to `b`, and
/// always show it to `e`.
pub fn check(threads: Threads) -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..RUNS {
        let (hash, in_phase, next_phase) = run(threads)?;
        assert_eq!(in_phase, 0, "run {i}: b saw c's Foo within the phase");
        assert_eq!(next_phase, (ENTITIES * TICKS) as u64, "run {i}");
        assert_eq!(hash, EXPECTED, "run {i}");
    }
    Ok(())
}
