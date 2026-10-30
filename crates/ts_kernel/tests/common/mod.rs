// spec: revues/architecture.md §9.2, §11.2 ; carnet architecture-securite.md §9.10 L6
//! Monde de test de la reproductibilité sur plusieurs fils (L6), partagé par
//! `parallel_single.rs` et `parallel_four.rs`.
//!
//! Le groupe de fils de `bevy_tasks` ne se règle qu'une fois par processus : chaque fichier
//! de test est un exécutable distinct, donc un processus distinct, et les deux comparent leur
//! empreinte à la même valeur figée.

use std::sync::{Mutex, PoisonError};
use std::thread::ThreadId;

use bevy_ecs::prelude::{Component, Query, Res, ResMut, Resource};
use ts_kernel::rng::{Stream, WorldKey, draw_block};
use ts_kernel::{
    BuildError, CanonicalError, MemorySnapshot, Phase, Plugin, RestoreError, Sim, SimBuilder,
    SimClock, SimConfig, SnapshotScope, StableEntity, StableId, StableIds, StableIndex,
    StateDecode, StateHash, StateReader, StateWriter, Threads, WorldStreams,
};

/// Empreinte globale attendue après [`TICKS`] ticks, quel que soit le nombre de fils.
pub const EXPECTED: &str = "bbe5dc0e03104235bfeb179e14678e8179c46a6780558e31c733e727af58214e";
/// Nombre de ticks.
pub const TICKS: usize = 1000;
/// Nombre d'entités.
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

/// Message croisé : `source` agit sur `target`.
struct Msg {
    source: StableId,
    local_seq: u32,
    target: StableId,
    amount: i64,
}

/// Tampon des messages, rempli dans un ordre quelconque par les fils.
#[derive(Resource, Default)]
struct Outbox(Mutex<Vec<Msg>>);

/// Fils qui ont exécuté une part de `par_iter` (diagnostic, hors état).
#[derive(Resource, Default)]
pub struct Workers(pub Mutex<Vec<ThreadId>>);

/// Message targets, as durable `StableId`s (never `Entity`, which a restore changes).
#[derive(Resource)]
struct Roster(Vec<StableId>);

/// Phase `Act` : chaque entité n'écrit que ses composants (§9.2, règle 2) et poste deux
/// messages croisés.
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
        let block = draw_block(streams.get(Stream::Ai), s.id(), slot, 3);
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

/// Phase `Consequences` : messages triés par (`StableId` source, `seq` local), puis appliqués
/// dans cet ordre (§9.2, règle 3). L'opération flottante n'est pas associative : sans le tri,
/// l'empreinte dépendrait de l'ordre de fin des fils.
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

/// Exécute [`TICKS`] ticks avec `threads` et rend l'empreinte globale en hexadécimal, et le
/// nombre de fils distincts vus dans `par_iter`.
pub fn run(threads: Threads) -> Result<(String, usize), BuildError> {
    let rules = rules();
    let mut sim = Sim::build(config(threads), &rules, &key())?;
    for _ in 0..TICKS {
        sim.step();
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

fn config(threads: Threads) -> SimConfig {
    SimConfig {
        threads,
        ..SimConfig::default()
    }
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
) -> Result<Vec<(String, String)>, RestoreError> {
    let rules = rules();
    let mut out = Vec::new();
    for &k in ks {
        let mut straight =
            Sim::build(config(threads), &rules, &key()).map_err(RestoreError::Build)?;
        for _ in 0..k {
            straight.step();
        }
        let snap: MemorySnapshot = straight.snapshot(SnapshotScope::Full);
        let mut restored = Sim::restore(&snap, config(threads), &rules, &key())?;
        for _ in 0..n {
            straight.step();
            restored.step();
        }
        out.push((hex(&straight), hex(&restored)));
    }
    Ok(out)
}
