// spec: revues/architecture.md §9.2, §11.2 ; carnet architecture-securite.md §9.10 L6
//! Test world for reproducibility across several threads (L6), shared by
//! `parallel_single.rs` and `parallel_four.rs`.
//!
//! The `bevy_tasks` thread pool can only be set once per process: each test file is a
//! separate executable, hence a separate process, and the two compare their fingerprint
//! with the same frozen value.

use std::error::Error;
use std::sync::{Mutex, PoisonError};
use std::thread::ThreadId;

use bevy_ecs::prelude::{Commands, Component, Entity, Has, Query, Res, ResMut, Resource, With};
use ts_kernel::rng::{DrawSite, Stream, WorldKey};
use ts_kernel::{
    CanonicalError, LodComplete, MemorySnapshot, PAR_ITER_FROM, ParallelPhases, Phase, Plugin, Sim,
    SimBuilder, SimClock, SimConfig, SnapshotScope, StableEntity, StableId, StableIds, StableIndex,
    StateDecode, StateHash, StateReader, StateWriter, Threads, WorldStreams, par_for_each_mut,
};

/// Expected global fingerprint after [`TICKS`] ticks, whatever the number of threads.
pub const EXPECTED: &str = "bbe5dc0e03104235bfeb179e14678e8179c46a6780558e31c733e727af58214e";
/// Number of ticks.
pub const TICKS: usize = 1000;
/// Draw sites of the test worlds.
const ACT: DrawSite = DrawSite::new(Stream::Ai, 3, "test.act");
const CLIMB: DrawSite = DrawSite::new(Stream::Ai, 0, "test.climb");
const TOGGLE: DrawSite = DrawSite::new(Stream::Ai, 1, "test.toggle");
/// Number of entities.
const ENTITIES: usize = 2000;

#[derive(Component, Clone)]
struct Pos(i64);

impl StateHash for Pos {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.0);
    }
}

impl StateDecode for Pos {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.i64().map(Self)
    }
}

#[derive(Component, Clone)]
struct Heat(f64);

impl StateHash for Heat {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.f64(self.0);
    }
}

impl StateDecode for Heat {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.f64().map(Self)
    }
}

/// Cross message: `source` acts on `target`.
struct Msg {
    source: StableId,
    local_seq: u32,
    target: StableId,
    amount: i64,
}

/// Buffer of messages, filled in any order by the threads.
#[derive(Resource, Default)]
struct Outbox(Mutex<Vec<Msg>>);

/// Threads that ran a share of `par_iter` (diagnostic, outside the state).
#[derive(Resource, Default)]
pub struct Workers(pub Mutex<Vec<ThreadId>>);

/// Message targets, as durable `StableId`s (never `Entity`, which a restore changes).
#[derive(Resource)]
struct Roster(Vec<StableId>);

/// `Act` phase: each entity writes only its own components (§9.2, rule 2) and posts two
/// cross messages.
fn act(
    clock: Res<SimClock>,
    streams: Res<WorldStreams>,
    roster: Res<Roster>,
    outbox: Res<Outbox>,
    workers: Res<Workers>,
    mut q: Query<(&StableEntity, &mut Pos, &mut Heat)>,
) {
    let slot = clock.now().as_millis().unsigned_abs();
    let ids = &roster.0;
    q.par_iter_mut().for_each(|(s, mut p, mut h)| {
        let block = streams.draw_block(ACT, s.id(), slot, 0);
        p.0 += (block[0] % 11) as i64 - 5;
        h.0 = h.0 * 0.75 + p.0 as f64 * 0.125;
        let msgs = (0..2).map(|i| Msg {
            source: s.id(),
            local_seq: i,
            target: ids[(block[1 + i as usize] % ids.len() as u64) as usize],
            amount: (block[3 + i as usize] % 5) as i64 + 1,
        });
        let mut out = outbox.0.lock().unwrap_or_else(PoisonError::into_inner);
        out.extend(msgs);
        drop(out);
        let me = std::thread::current().id();
        let mut seen = workers.0.lock().unwrap_or_else(PoisonError::into_inner);
        if !seen.contains(&me) {
            seen.push(me);
        }
    });
}

/// `Consequences` phase: messages sorted by (source `StableId`, local `seq`), then applied
/// in that order (§9.2, rule 3). Floating-point operations are not associative: without the
/// sort, the fingerprint would depend on the order in which the threads finish.
fn deliver(
    mut outbox: ResMut<Outbox>,
    index: Res<StableIndex>,
    mut q: Query<(&mut Pos, &mut Heat)>,
) {
    let mut msgs = std::mem::take(outbox.0.get_mut().unwrap_or_else(PoisonError::into_inner));
    msgs.sort_unstable_by_key(|m| (m.source, m.local_seq));
    for m in msgs {
        if let Some(e) = index.get(m.target)
            && let Ok((mut p, mut h)) = q.get_mut(e)
        {
            p.0 -= m.amount;
            h.0 = h.0 * 0.5 + m.amount as f64;
        }
    }
}

struct ParallelWorld;

impl Plugin for ParallelWorld {
    fn name(&self) -> &'static str {
        "parallel_test"
    }
    fn version(&self) -> u32 {
        1
    }
    fn draw_sites(&self) -> &'static [DrawSite] {
        &[ACT]
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Pos>("test")
            .state_component::<Heat>("test")
            .insert_resource(Outbox::default())
            .insert_resource(Workers::default());
        let world = b.world_mut();
        let mut ids = Vec::new();
        for _ in 0..ENTITIES {
            let Some(id) = world.resource_mut::<StableIds>().allocate() else {
                break;
            };
            world.spawn((StableEntity::new(id), Pos(0), Heat(1.0)));
            ids.push(id);
        }
        world.insert_resource(Roster(ids));
        b.add_systems(Phase::Act, act);
        b.add_systems(Phase::Consequences, deliver);
    }
}

/// Runs [`TICKS`] ticks with `threads` and returns the global fingerprint in hexadecimal, and
/// the number of distinct threads seen in `par_iter`.
pub fn run(threads: Threads) -> Result<(String, usize), Box<dyn Error>> {
    let rules = rules();
    let mut sim = Sim::build(config(threads), &rules, &key())?;
    for _ in 0..TICKS {
        sim.step()?;
    }
    let hex = hex(&sim);
    let workers = sim
        .world()
        .resource::<Workers>()
        .0
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .len();
    Ok((hex, workers))
}

fn rules() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(ParallelWorld)]
}

/// With [`ParallelPhases::Always`], so that 4 threads exercise the parallel executor on
/// every phase, as before issue #67.
fn config(threads: Threads) -> SimConfig {
    SimConfig {
        threads,
        parallel_phases: ParallelPhases::Always,
        ..SimConfig::default()
    }
}

// spec: revues/architecture.md §9.1, §9.2 ; carnet architecture-securite.md §9.10 L6
/// Expected global fingerprint of the switching world after [`SWITCHING_TICKS`] ticks,
/// whatever the number of threads and the [`ParallelPhases`] policy.
pub const SWITCHING_EXPECTED: &str =
    "45e1d01154a36850d7b19c9983632bbfd7d48f0a4a17023609410b161a2ab957";
/// Number of ticks of the switching world.
pub const SWITCHING_TICKS: usize = 300;
/// Entities of the switching world: more than [`PAR_ITER_FROM`], so that `climb` takes both
/// branches of [`par_for_each_mut`].
const SWITCHING_ENTITIES: u64 = 96;
const _: () = assert!(SWITCHING_ENTITIES as usize > PAR_ITER_FROM);
/// Threshold of [`ParallelPhases::FromWork`] that puts the switching world's `Act` phase on
/// the parallel executor in about half of its ticks.
pub const SWITCHING_FROM: u32 = 48;

#[derive(Component, Clone)]
struct Level(i64);

impl StateHash for Level {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.0);
    }
}

impl StateDecode for Level {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.i64().map(Self)
    }
}

#[derive(Component, Clone)]
struct Seen(u64);

impl StateHash for Seen {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u64(self.0);
    }
}

impl StateDecode for Seen {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.u64().map(Self)
    }
}

/// Threads that ran `climb`, one per tick that ran it (diagnostic, outside the state).
#[derive(Resource, Default)]
struct Placement(Mutex<Vec<ThreadId>>);

/// `Act` phase, first of two systems that do not conflict: the entities in full simulation
/// climb by a keyed draw, on the pool from [`PAR_ITER_FROM`] of them.
fn climb(
    clock: Res<SimClock>,
    streams: Res<WorldStreams>,
    placement: Res<Placement>,
    mut q: Query<(&StableEntity, &mut Level), With<LodComplete>>,
) {
    let slot = clock.now().as_millis().unsigned_abs();
    let keys = &*streams;
    par_for_each_mut(&mut q, |(s, mut level)| {
        level.0 += (keys.draw(CLIMB, s.id(), slot) % 7) as i64 - 3;
    });
    let mut seen = placement.0.lock().unwrap_or_else(PoisonError::into_inner);
    seen.push(std::thread::current().id());
}

/// `Act` phase, second system: counts the ticks spent in full simulation.
fn count(mut q: Query<&mut Seen, With<LodComplete>>) {
    for mut seen in &mut q {
        seen.0 += 1;
    }
}

/// `Lod` phase: the number of entities in full simulation follows a saw from 0 to all of
/// them, and a keyed draw picks which.
fn toggle(
    mut commands: Commands,
    clock: Res<SimClock>,
    streams: Res<WorldStreams>,
    q: Query<(Entity, &StableEntity, Has<LodComplete>)>,
) {
    let tick = clock.now().as_millis().unsigned_abs() / clock.tick_millis().unsigned_abs();
    let target = (tick * 5) % (SWITCHING_ENTITIES + 1);
    for (e, s, complete) in &q {
        let wanted = streams.draw(TOGGLE, s.id(), tick) % SWITCHING_ENTITIES < target;
        if wanted && !complete {
            commands.entity(e).insert(LodComplete);
        } else if !wanted && complete {
            commands.entity(e).remove::<LodComplete>();
        }
    }
}

struct SwitchingWorld;

impl Plugin for SwitchingWorld {
    fn name(&self) -> &'static str {
        "switching_test"
    }
    fn version(&self) -> u32 {
        1
    }
    fn draw_sites(&self) -> &'static [DrawSite] {
        &[CLIMB, TOGGLE]
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Level>("test")
            .state_component::<Seen>("test")
            .insert_resource(Placement::default());
        let world = b.world_mut();
        for _ in 0..SWITCHING_ENTITIES {
            let Some(id) = world.resource_mut::<StableIds>().allocate() else {
                break;
            };
            world.spawn((StableEntity::new(id), Level(0), Seen(0)));
        }
        b.add_systems(Phase::Act, (climb, count));
        b.add_systems(Phase::Lod, toggle);
    }
}

/// Outcome of [`switching`].
pub struct Switching {
    /// Global fingerprint, in hexadecimal.
    pub hash: String,
    /// Ticks whose `Act` phase ran on the calling thread (single-threaded executor).
    pub on_caller: usize,
    /// Ticks whose `Act` phase ran on a pool thread (parallel executor).
    pub on_pool: usize,
}

/// Runs the switching world for [`SWITCHING_TICKS`] ticks with `threads` and `policy`.
pub fn switching(threads: Threads, policy: ParallelPhases) -> Result<Switching, Box<dyn Error>> {
    let rules: Vec<Box<dyn Plugin>> = vec![Box::new(SwitchingWorld)];
    let cfg = SimConfig {
        threads,
        parallel_phases: policy,
        ..SimConfig::default()
    };
    let mut sim = Sim::build(cfg, &rules, &key())?;
    for _ in 0..SWITCHING_TICKS {
        sim.step()?;
    }
    let me = std::thread::current().id();
    let placement = sim.world().resource::<Placement>();
    let seen = placement.0.lock().unwrap_or_else(PoisonError::into_inner);
    let on_caller = seen.iter().filter(|&&id| id == me).count();
    Ok(Switching {
        hash: hex(&sim),
        on_caller,
        on_pool: seen.len() - on_caller,
    })
}

fn key() -> WorldKey {
    WorldKey::from_bytes(*b"time_switcher L6")
}

fn hex(sim: &Sim) -> String {
    sim.state_hash()
        .global
        .as_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

// spec: revues/architecture.md §11.1, §12.4
/// Round trip (F4): for each `k`, snapshot at tick `k`, restore, run `n` ticks, and return
/// the pairs (straight hash, restored hash), which must be equal.
pub fn round_trips(
    threads: Threads,
    ks: &[usize],
    n: usize,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let rules = rules();
    let mut out = Vec::new();
    for &k in ks {
        let mut straight = Sim::build(config(threads), &rules, &key())?;
        for _ in 0..k {
            straight.step()?;
        }
        let snap: MemorySnapshot = straight.snapshot(SnapshotScope::Full);
        let mut restored = Sim::restore(&snap, config(threads), &rules, &key())?;
        for _ in 0..n {
            straight.step()?;
            restored.step()?;
        }
        out.push((hex(&straight), hex(&restored)));
    }
    Ok(out)
}
