use super::*;
use crate::rng::{Stream, draw};
use crate::state::{FAMILY_CONTEXT, GLOBAL_CONTEXT};
use crate::{DueEvents, KERNEL_FAMILY, Phase, SimBuilder, StateHash, StateWriter, WorldStreams};
use bevy_ecs::prelude::{Component, Query, Res, ResMut, Resource};

#[derive(Component, Clone)]
struct Pos(i64);

impl StateHash for Pos {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.0);
    }
}

#[derive(Component, Clone)]
struct Heat(f64);

impl StateHash for Heat {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.f64(self.0);
    }
}

#[derive(Resource, Clone)]
struct Total(i64);

impl StateHash for Total {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.0);
    }
}

fn move_all(
    clock: Res<SimClock>,
    streams: Res<WorldStreams>,
    mut q: Query<(&StableEntity, &mut Pos, &mut Heat)>,
) {
    let slot = clock.now().as_millis().unsigned_abs();
    for (s, mut p, mut h) in &mut q {
        let r = draw(streams.get(Stream::Population), s.id(), slot, 1);
        p.0 += i64::try_from(r % 7).unwrap() - 3;
        h.0 = h.0 * 0.5 + p.0 as f64;
    }
}

fn on_event(due: Res<DueEvents>, mut queue: ResMut<EventQueue>, mut total: ResMut<Total>) {
    for e in due.iter() {
        total.0 += i64::try_from(e.event.arg).unwrap();
        queue.schedule(e.at.add_millis(1000), e.event);
    }
}

fn sum(q: Query<&Pos>, mut total: ResMut<Total>) {
    total.0 += q.iter().map(|p| p.0).sum::<i64>();
}

/// Greffon de test : 50 entités qui bougent au hasard, un total, un événement périodique.
struct Life {
    version: u32,
    migrates_from: Option<u32>,
    declares_state: bool,
}

impl Plugin for Life {
    fn name(&self) -> &'static str {
        "life"
    }
    fn version(&self) -> u32 {
        self.version
    }
    fn build(&self, b: &mut SimBuilder) {
        if self.declares_state {
            b.state_component::<Pos>("life")
                .state_component::<Heat>("life")
                .state_resource::<Total>("stats");
        }
        if let Some(v) = self.migrates_from {
            b.migrates_from(v);
        }
        b.insert_resource(Total(0));
        let world = b.world_mut();
        for _ in 0..50 {
            let id = world.resource_mut::<StableIds>().allocate().unwrap();
            world.spawn((StableEntity::new(id), Pos(0), Heat(1.0)));
        }
        let subject = StableId::from_snapshot_raw(0);
        let event = TimedEvent {
            kind: 1,
            subject,
            arg: 5,
        };
        world
            .resource_mut::<EventQueue>()
            .schedule(SimTime::from_millis(120), event);
        b.add_systems(Phase::Scheduled, on_event);
        b.add_systems(Phase::Act, move_all);
        b.add_systems(Phase::Consequences, sum);
    }
}

fn life(version: u32) -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Life {
        version,
        migrates_from: None,
        declares_state: true,
    })]
}

fn key() -> WorldKey {
    WorldKey::insecure_from_u64_for_tests(11)
}

fn build(rules: &[Box<dyn Plugin>]) -> Sim {
    Sim::build(SimConfig::default(), rules, &key()).unwrap()
}

fn steps(sim: &mut Sim, n: usize) {
    for _ in 0..n {
        sim.step();
    }
}

fn restore(snap: &MemorySnapshot, rules: &[Box<dyn Plugin>]) -> Result<Sim, RestoreError> {
    Sim::restore(snap, SimConfig::default(), rules, &key())
}

/// Un monde restauré reprend exactement là où l'instantané l'a pris.
#[test]
fn restore_continues_identically() {
    let rules = life(1);
    let mut a = build(&rules);
    steps(&mut a, 100);
    let snap: MemorySnapshot = a.snapshot(SnapshotScope::Full);
    let at_snapshot = a.state_hash();
    steps(&mut a, 100);

    let mut b = restore(&snap, &rules).unwrap();
    assert_eq!(b.now(), snap.now());
    assert_eq!(b.state_hash(), at_snapshot);
    steps(&mut b, 100);
    assert_eq!(b.now(), a.now());
    assert_eq!(b.state_hash(), a.state_hash());
    assert_ne!(b.state_hash(), at_snapshot);
    // Les compteurs continuent : le prochain identifiant n'est pas réutilisé.
    let next_a = a.world.resource_mut::<StableIds>().allocate();
    let next_b = b.world.resource_mut::<StableIds>().allocate();
    assert_eq!(next_a, next_b);
    assert_eq!(next_a.unwrap().index(), 50);
}

/// L5 : une autre empreinte de règles est refusée, sauf migration déclarée.
#[test]
fn restore_checks_rules_fingerprint() {
    let mut a = build(&life(1));
    steps(&mut a, 3);
    let snap: MemorySnapshot = a.snapshot(SnapshotScope::Full);
    match restore(&snap, &life(2)) {
        Err(RestoreError::RulesMismatch { snapshot, current }) => {
            assert_eq!(snapshot, a.rules_fingerprint());
            assert_eq!(current, RulesFingerprint::of(&[("life", 2)]));
        }
        other => panic!("refus attendu : {other:?}"),
    }
    let migrating: Vec<Box<dyn Plugin>> = vec![Box::new(Life {
        version: 2,
        migrates_from: Some(1),
        declares_state: true,
    })];
    let b = restore(&snap, &migrating).unwrap();
    assert_eq!(b.state_hash(), a.state_hash());
    assert!(matches!(
        restore(&snap, &[]),
        Err(RestoreError::RulesMismatch { .. })
    ));
}

#[test]
fn restore_checks_state_layout() {
    let a = build(&life(1));
    let snap: MemorySnapshot = a.snapshot(SnapshotScope::Full);
    let silent: Vec<Box<dyn Plugin>> = vec![Box::new(Life {
        version: 1,
        migrates_from: None,
        declares_state: false,
    })];
    assert_eq!(
        restore(&snap, &silent).unwrap_err(),
        RestoreError::StateLayoutMismatch
    );
}

/// L4 (P25) : ni la clé du monde ni l'une de ses moitiés ne figurent dans l'instantané.
#[test]
fn snapshot_never_contains_world_key() {
    let bytes: [u8; 16] = core::array::from_fn(|i| 0xa0 ^ (i as u8 * 17));
    let world_key = WorldKey::from_bytes(bytes);
    let mut sim = Sim::build(SimConfig::default(), &life(1), &world_key).unwrap();
    steps(&mut sim, 20);
    let snap: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    let image = snap.canonical_bytes();
    assert!(image.len() > 50 * 16, "{} octets", image.len());
    for needle in [&bytes[..], &bytes[..8], &bytes[8..]] {
        assert!(!image.windows(needle.len()).any(|w| w == needle));
    }
}

/// La clé du monde n'agit sur l'empreinte que par les tirages : deux mondes de clés
/// différentes ont la même empreinte tant qu'aucun tirage n'a eu lieu.
#[test]
fn state_hash_does_not_read_keys() {
    let a = Sim::build(SimConfig::default(), &life(1), &key()).unwrap();
    let other = WorldKey::insecure_from_u64_for_tests(12);
    let mut b = Sim::build(SimConfig::default(), &life(1), &other).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    b.step();
    assert_ne!(a.state_hash().family("life"), b.state_hash().family("life"));
}

#[test]
fn families_are_sorted_and_separate() {
    let mut sim = build(&life(1));
    let before = sim.state_hash();
    let names: Vec<_> = before.families.iter().map(|(n, _)| *n).collect();
    assert_eq!(names, ["kernel", "life", "stats"]);
    let mut q = sim.world.query::<&mut Pos>();
    if let Some(mut p) = q.iter_mut(&mut sim.world).next() {
        p.0 = 1;
    }
    let after = sim.state_hash();
    assert_ne!(before.global, after.global);
    assert_ne!(before.family("life"), after.family("life"));
    assert_eq!(before.family("kernel"), after.family("kernel"));
    assert_eq!(before.family("stats"), after.family("stats"));
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// L3 : vecteurs figés de l'empreinte d'état. Changer l'encodage change toutes les empreintes
/// dorées, et se signale dans la PR.
#[test]
fn state_hash_reference_vectors() {
    let empty = build(&[]);
    // Famille du noyau d'un monde neuf, écrite à la main.
    let mut kernel = Vec::new();
    kernel.extend_from_slice(&6_u64.to_le_bytes());
    kernel.extend_from_slice(b"kernel");
    kernel.extend_from_slice(&0_i64.to_le_bytes()); // date
    kernel.extend_from_slice(&0_i64.to_le_bytes()); // origine des jours
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // prochain StableId
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // prochain seq
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // événements en attente
    let digest = blake3::derive_key(FAMILY_CONTEXT, &kernel);
    let hashes = empty.state_hash();
    assert_eq!(hashes.family(KERNEL_FAMILY).unwrap().as_bytes(), &digest);
    let mut global = Vec::new();
    global.extend_from_slice(&1_u64.to_le_bytes());
    global.extend_from_slice(&kernel[..14]);
    global.extend_from_slice(&digest);
    assert_eq!(
        hashes.global.as_bytes(),
        &blake3::derive_key(GLOBAL_CONTEXT, &global)
    );
    assert_eq!(
        hex(hashes.global.as_bytes()),
        "c399c9f5855021141f562afbf02e51b66d093cd02824aabbb473e82f4fcb7986"
    );

    let mut sim = build(&life(1));
    assert_eq!(
        hex(sim.state_hash().global.as_bytes()),
        "116d43fbd79d0c0b9d8dd31f0cc120298f954ca0576ac59d79b315425d0bd61d"
    );
    steps(&mut sim, 10);
    assert_eq!(
        hex(sim.state_hash().global.as_bytes()),
        "c64ee4fd66d8fa9fb7d1a0d7a6925a7d50058899b4caf255e441eb5e48482a01"
    );
}
