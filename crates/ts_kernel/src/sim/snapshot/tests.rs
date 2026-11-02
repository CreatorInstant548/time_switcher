use super::*;
use crate::rng::{Stream, draw};
use crate::state::{FAMILY_CONTEXT, GLOBAL_CONTEXT};
use crate::{
    DueEvents, KERNEL_FAMILY, Phase, SimBuilder, StableIndex, StateDecode, StateHash, StateReader,
    StateWriter, WorldStreams,
};
use bevy_ecs::prelude::{Component, Query, Res, ResMut, Resource};

#[derive(Component, Clone)]
pub(super) struct Pos(pub(super) i64);

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

#[derive(Resource, Clone)]
pub(super) struct Total(pub(super) i64);

impl StateHash for Total {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.0);
    }
}

impl StateDecode for Total {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.i64().map(Self)
    }
}

fn move_all(
    clock: Res<SimClock>,
    streams: Res<WorldStreams>,
    mut q: Query<(&StableEntity, &mut Pos, &mut Heat)>,
) {
    let slot = clock.now().as_millis().unsigned_abs();
    for (s, mut p, mut h) in &mut q {
        let r = draw(streams.keys().get(Stream::Population), s.id(), slot, 1);
        p.0 += i64::try_from(r % 7).unwrap() - 3;
        h.0 = h.0 * 0.5 + p.0 as f64;
    }
}

fn on_event(due: Res<DueEvents>, mut queue: ResMut<EventQueue>, mut total: ResMut<Total>) {
    for e in due.iter() {
        total.0 += i64::try_from(e.event.arg).unwrap();
        queue.schedule(e.at.add_millis(1000), e.event).unwrap();
    }
}

fn sum(q: Query<&Pos>, mut total: ResMut<Total>) {
    total.0 += q.iter().map(|p| p.0).sum::<i64>();
}

/// Test plugin: 50 entities that move at random, a total, a periodic event.
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
            .schedule(SimTime::from_millis(120), event)
            .unwrap();
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

pub(super) fn key() -> WorldKey {
    WorldKey::insecure_from_u64_for_tests(11)
}

pub(super) fn build(rules: &[Box<dyn Plugin>]) -> Sim {
    Sim::build(SimConfig::default(), rules, &key()).unwrap()
}

pub(super) fn steps(sim: &mut Sim, n: usize) {
    for _ in 0..n {
        sim.step();
    }
}

pub(super) fn restore(
    snap: &MemorySnapshot,
    rules: &[Box<dyn Plugin>],
) -> Result<Sim, RestoreError> {
    Sim::restore(snap, SimConfig::default(), rules, &key())
}

/// A restored world resumes exactly where the snapshot took it.
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
    // The counters continue: the next identifier is not reused.
    let next_a = a.world.resource_mut::<StableIds>().allocate();
    let next_b = b.world.resource_mut::<StableIds>().allocate();
    assert_eq!(next_a, next_b);
    assert_eq!(next_a.unwrap().index(), 50);
}

/// L5: another rules fingerprint is refused, unless a migration is declared.
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

/// L4 (P25): neither the world key nor either of its halves appears in the snapshot.
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

/// The world key acts on the fingerprint only through draws: two worlds with different keys
/// have the same fingerprint as long as no draw has taken place.
#[test]
fn state_hash_does_not_read_keys() {
    let a = Sim::build(SimConfig::default(), &life(1), &key()).unwrap();
    let other = WorldKey::insecure_from_u64_for_tests(12);
    let mut b = Sim::build(SimConfig::default(), &life(1), &other).unwrap();
    assert_eq!(a.state_hash(), b.state_hash());
    b.step();
    assert_ne!(a.state_hash().family("life"), b.state_hash().family("life"));
}

/// L4: `WorldStreams` is excluded from the snapshot. Two worlds that differ only by their
/// key give byte-identical snapshots, and a restore takes its stream keys from the key it
/// is given, never from the snapshot.
#[test]
fn snapshot_excludes_stream_keys() {
    let other = WorldKey::insecure_from_u64_for_tests(12);
    let a = Sim::build(SimConfig::default(), &life(1), &key()).unwrap();
    let b = Sim::build(SimConfig::default(), &life(1), &other).unwrap();
    let snap_a: MemorySnapshot = a.snapshot(SnapshotScope::Full);
    let snap_b: MemorySnapshot = b.snapshot(SnapshotScope::Full);
    assert_eq!(snap_a.canonical_bytes(), snap_b.canonical_bytes());

    let subject = StableId::from_snapshot_raw(3);
    let sample = |sim: &Sim| {
        let streams = sim.world.resource::<WorldStreams>();
        draw(streams.keys().get(Stream::Population), subject, 1, 1)
    };
    let restored = Sim::restore(&snap_a, SimConfig::default(), &life(1), &other).unwrap();
    assert_eq!(sample(&restored), sample(&b));
    assert_ne!(sample(&restored), sample(&a));
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

/// L3: frozen vectors of the state fingerprint. Changing the encoding changes all the
/// golden fingerprints, and is flagged in the PR.
#[test]
fn state_hash_reference_vectors() {
    let empty = build(&[]);
    // Kernel family of a new world, written by hand.
    let mut kernel = Vec::new();
    kernel.extend_from_slice(&6_u64.to_le_bytes());
    kernel.extend_from_slice(b"kernel");
    kernel.extend_from_slice(&0_i64.to_le_bytes()); // date
    kernel.extend_from_slice(&0_i64.to_le_bytes()); // day origin
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // next StableId
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // next seq
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // pending events
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // stable entities (M1)
    kernel.extend_from_slice(&0_u64.to_le_bytes()); // entities carrying LodComplete (I1)
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
        "e1ad42b91777dd76f7aab6b6a24e86ad52cb58f706154a57c63d8a1e2ef59412"
    );

    let mut sim = build(&life(1));
    assert_eq!(
        hex(sim.state_hash().global.as_bytes()),
        "e9a2e39c32d9a0191989c2806ded97e85415152badeef00c582ec26838914c5d"
    );
    steps(&mut sim, 10);
    assert_eq!(
        hex(sim.state_hash().global.as_bytes()),
        "17020f03a312a377b36227536daf3a2bdb35a56ca6622ce427f800c6a2deed96"
    );
}

/// The snapshot decoded back from its own bytes (AS-36).
pub(super) fn through_bytes(snap: &MemorySnapshot, rules: &[Box<dyn Plugin>]) -> MemorySnapshot {
    let layout = StateLayout::of(&SimConfig::default(), rules).unwrap();
    let limits = CanonicalLimits { max_len: 1 << 30 };
    let decoded = MemorySnapshot::from_canonical(
        &snap.header_bytes(),
        snap.canonical_bytes(),
        &layout,
        &limits,
    )
    .unwrap();
    assert_eq!(decoded.header_bytes(), snap.header_bytes());
    assert_eq!(decoded.canonical_bytes(), snap.canonical_bytes());
    decoded
}

/// Snapshot at tick `k`, restore, then `n` ticks: same state as `k + n` ticks straight (F4),
/// from the snapshot in memory and from its bytes.
fn assert_round_trips(rules: &[Box<dyn Plugin>], ks: &[usize], n: usize) {
    for &k in ks {
        let mut straight = build(rules);
        steps(&mut straight, k);
        let snap: MemorySnapshot = straight.snapshot(SnapshotScope::Full);
        let mut restored = [
            restore(&snap, rules).unwrap(),
            restore(&through_bytes(&snap, rules), rules).unwrap(),
        ];
        for r in &restored {
            assert_eq!(r.state_hash(), straight.state_hash(), "at tick {k}");
        }
        steps(&mut straight, n);
        for r in &mut restored {
            steps(r, n);
            assert_eq!(r.state_hash(), straight.state_hash(), "from tick {k}");
        }
    }
}

#[test]
fn life_world_round_trips_at_several_ticks() {
    assert_round_trips(&life(1), &[0, 1, 7, 40], 25);
}

/// Builds a one-plugin rule set.
fn rules_of(p: impl Plugin + 'static) -> Vec<Box<dyn Plugin>> {
    vec![Box::new(p)]
}

/// Spawns `n` stable entities with `Pos(0)`, in reverse `StableId` order when `reverse`.
fn spawn_stable(b: &mut SimBuilder, n: usize, reverse: bool) -> Vec<StableId> {
    let world = b.world_mut();
    let ids: Vec<_> = (0..n)
        .map(|_| world.resource_mut::<StableIds>().allocate().unwrap())
        .collect();
    let mut order = ids.clone();
    if reverse {
        order.reverse();
    }
    for id in order {
        world.spawn((StableEntity::new(id), Pos(0)));
    }
    ids
}

fn bump(mut q: Query<&mut Pos>) {
    for mut p in &mut q {
        p.0 += 1;
    }
}

/// I1: stable entities in full simulation.
struct Full;

impl Plugin for Full {
    fn name(&self) -> &'static str {
        "full"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Pos>("full");
        for id in spawn_stable(b, 3, false) {
            let e = b.world_mut().resource::<StableIndex>().get(id).unwrap();
            b.world_mut().entity_mut(e).insert(LodComplete);
        }
        b.add_systems(Phase::Act, bump);
    }
}

/// I1: `LodComplete` survives a restore, so `run_until` runs and skips the same ticks.
#[test]
fn lod_complete_survives_restore() {
    let rules = rules_of(Full);
    let mut a = build(&rules);
    steps(&mut a, 10);
    let snap: MemorySnapshot = a.snapshot(SnapshotScope::Full);
    let mut b = restore(&snap, &rules).unwrap();
    // The marker goes through the bytes too (AS-36).
    let mut c = restore(&through_bytes(&snap, &rules), &rules).unwrap();
    assert_eq!(c.complete.iter(&c.world).count(), 3);
    let stop = crate::StopCondition::At(SimTime::from_millis(5_000));
    let (ra, rb, rc) = (a.run_until(stop), b.run_until(stop), c.run_until(stop));
    assert_eq!(ra, rb);
    assert_eq!(ra, rc);
    assert_eq!((ra.ticks, ra.skipped_ticks), (90, 0));
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.state_hash(), c.state_hash());
}

/// A day origin other than the epoch goes through the bytes and is restored.
#[test]
fn day_origin_round_trips_through_the_bytes() {
    let rules = life(1);
    let mut a = build(&rules);
    a.world.resource_mut::<SimClock>().day_origin = DayOrigin::from_days(-3);
    steps(&mut a, 5);
    let snap: MemorySnapshot = a.snapshot(SnapshotScope::Full);
    let mut b = restore(&through_bytes(&snap, &rules), &rules).unwrap();
    assert_eq!(
        b.world.resource::<SimClock>().day_origin(),
        DayOrigin::from_days(-3)
    );
    assert_eq!(b.state_hash(), a.state_hash());
    steps(&mut a, 5);
    steps(&mut b, 5);
    assert_eq!(b.state_hash(), a.state_hash());
}

/// I1: the marker is part of the kernel state hash.
#[test]
fn lod_complete_is_hashed() {
    let rules = rules_of(Full);
    let a = build(&rules);
    let mut b = build(&rules);
    let first = b.world.resource::<StableIndex>().iter().next().unwrap().1;
    b.world.entity_mut(first).remove::<LodComplete>();
    assert_ne!(
        a.state_hash().family(KERNEL_FAMILY),
        b.state_hash().family(KERNEL_FAMILY)
    );
}

/// Durable targets kept as `StableId`s, never as `Entity`s (I2).
#[derive(Resource)]
struct Targets(Vec<StableId>);

fn bump_targets(targets: Res<Targets>, index: Res<StableIndex>, mut q: Query<&mut Pos>) {
    for &id in &targets.0 {
        if let Some(mut p) = index.get(id).and_then(|e| q.get_mut(e).ok()) {
            p.0 += 1;
        }
    }
}

struct Linked;

impl Plugin for Linked {
    fn name(&self) -> &'static str {
        "linked"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Pos>("linked");
        let ids = spawn_stable(b, 4, false);
        b.insert_resource(Targets(vec![ids[2], ids[0], ids[2]]));
        b.add_systems(Phase::Act, bump_targets);
    }
}

/// I2: references through `StableIndex` stay valid after a restore.
#[test]
fn stable_index_is_rebuilt_on_restore() {
    assert_round_trips(&rules_of(Linked), &[0, 5], 10);
    let rules = rules_of(Linked);
    let mut sim = build(&rules);
    steps(&mut sim, 3);
    let snap: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    let restored = restore(&snap, &rules).unwrap();
    let index = restored.world.resource::<StableIndex>();
    assert_eq!(index.len(), 4);
    assert_eq!(index.conflicts(), 0);
    for (id, e) in index.iter() {
        let s = restored.world.get::<StableEntity>(e).unwrap();
        assert_eq!(s.id(), id);
    }
}

#[test]
fn stable_index_follows_spawn_and_despawn() {
    let mut sim = build(&[]);
    let id = sim.world.resource_mut::<StableIds>().allocate().unwrap();
    let e = sim.world.spawn(StableEntity::new(id)).id();
    assert_eq!(sim.world.resource::<StableIndex>().get(id), Some(e));
    let twin = sim.world.spawn(StableEntity::new(id)).id();
    assert_eq!(sim.world.resource::<StableIndex>().conflicts(), 1);
    sim.world.despawn(twin);
    assert_eq!(sim.world.resource::<StableIndex>().get(id), Some(e));
    // #31 m1: the count follows the duplicates that are still there.
    assert_eq!(sim.world.resource::<StableIndex>().conflicts(), 0);
    sim.world.despawn(e);
    assert!(sim.world.resource::<StableIndex>().is_empty());
}

/// #31 m1: when the indexed entity leaves, a duplicate left alone takes its place, so it
/// stays in the fingerprint and the snapshot, and the world is sound again.
#[test]
fn duplicate_takes_the_place_of_a_despawned_original() {
    let mut sim = build(&[]);
    let id = sim.world.resource_mut::<StableIds>().allocate().unwrap();
    let e = sim.world.spawn(StableEntity::new(id)).id();
    let twins: Vec<_> = (0..2)
        .map(|_| sim.world.spawn(StableEntity::new(id)).id())
        .collect();
    assert_eq!(sim.world.resource::<StableIndex>().conflicts(), 2);
    sim.world.despawn(e);
    let index = sim.world.resource::<StableIndex>();
    assert_eq!((index.get(id), index.conflicts()), (Some(twins[0]), 1));
    sim.world.despawn(twins[0]);
    let index = sim.world.resource::<StableIndex>();
    assert_eq!((index.get(id), index.conflicts()), (Some(twins[1]), 0));
    let snap: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    assert_eq!(snap.entities, [id]);
}

/// Order-dependent fold, following the rule: walk the index, sorted by `StableId` (I3).
fn fold_sorted(index: Res<StableIndex>, q: Query<&Pos>, mut total: ResMut<Total>) {
    for (id, e) in index.iter() {
        if let Ok(p) = q.get(e) {
            let idx = i64::try_from(id.index()).unwrap();
            total.0 = total.0.wrapping_mul(31).wrapping_add(idx + p.0);
        }
    }
}

struct Reversed;

impl Plugin for Reversed {
    fn name(&self) -> &'static str {
        "reversed"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Pos>("reversed")
            .state_resource::<Total>("reversed")
            .insert_resource(Total(0));
        spawn_stable(b, 20, true);
        b.add_systems(Phase::Act, bump);
        b.add_systems(Phase::Consequences, fold_sorted);
    }
}

/// I3: entities spawned in reverse order; a fold that sorts by `StableId` round-trips.
#[test]
fn sorted_iteration_round_trips() {
    assert_round_trips(&rules_of(Reversed), &[0, 3, 11], 6);
}

/// Time skip and the scheduler cursor across restores, with far and past events.
fn reschedule(
    clock: Res<SimClock>,
    due: Res<DueEvents>,
    mut queue: ResMut<EventQueue>,
    mut total: ResMut<Total>,
) {
    const OFFSETS: [i64; 6] = [37, 1, 61_000, 3_700_000, 90_000_000, -10];
    for e in due.iter() {
        let seq = i64::try_from(e.seq).unwrap();
        total.0 = total
            .0
            .wrapping_mul(31)
            .wrapping_add(e.at.as_millis() ^ seq);
        let offset = OFFSETS[usize::try_from(e.event.arg % 6).unwrap()];
        let at = if offset < 0 {
            clock.now().add_millis(offset)
        } else {
            e.at.add_millis(offset)
        };
        let arg = e.event.arg.wrapping_mul(7).wrapping_add(3);
        queue.schedule(at, TimedEvent { arg, ..e.event }).unwrap();
    }
}

struct Timers;

impl Plugin for Timers {
    fn name(&self) -> &'static str {
        "timers"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_resource::<Total>("timers")
            .insert_resource(Total(0));
        let subject = StableId::from_snapshot_raw(0);
        let mut queue = b.world_mut().resource_mut::<EventQueue>();
        for i in 0..5_i64 {
            let event = TimedEvent {
                kind: 1,
                subject,
                arg: i.unsigned_abs(),
            };
            queue
                .schedule(SimTime::from_millis(13 + 997 * i), event)
                .unwrap();
        }
        b.add_systems(Phase::Scheduled, reschedule);
    }
}

#[test]
fn time_skip_round_trips() {
    let rules = rules_of(Timers);
    let stops: Vec<_> = (1..12_i64)
        .map(|i| SimTime::from_millis(i * 4_321_017))
        .collect();
    let mut straight = build(&rules);
    let mut snaps = Vec::new();
    for &s in &stops {
        straight.run_until(crate::StopCondition::At(s));
        straight.step();
        snaps.push(straight.snapshot::<MemorySnapshot>(SnapshotScope::Full));
    }
    for (k, snap) in snaps.iter().enumerate() {
        // Through the bytes: far, past and pending events round trip too.
        let mut restored = restore(&through_bytes(snap, &rules), &rules).unwrap();
        for &s in &stops[k + 1..] {
            restored.run_until(crate::StopCondition::At(s));
            restored.step();
        }
        assert_eq!(restored.state_hash(), straight.state_hash(), "snapshot {k}");
    }
}

/// M1: the set of stable entities is hashed, even without declared components.
#[test]
fn bare_stable_entity_changes_the_hash() {
    let a = build(&[]);
    let mut b = build(&[]);
    let id = b.world.resource_mut::<StableIds>().allocate().unwrap();
    b.world.spawn(StableEntity::new(id));
    let mut c = build(&[]);
    c.world.resource_mut::<StableIds>().allocate().unwrap();
    assert_ne!(b.state_hash(), c.state_hash());
    assert_ne!(a.state_hash(), c.state_hash());
}

#[derive(Component)]
struct Cache;

struct Cached(bool);

impl Plugin for Cached {
    fn name(&self) -> &'static str {
        "cached"
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Pos>("cached");
        if self.0 {
            b.transient_component::<Cache>();
        }
        let id = spawn_stable(b, 1, false)[0];
        let e = b.world_mut().resource::<StableIndex>().get(id).unwrap();
        b.world_mut().entity_mut(e).insert(Cache);
    }
}

/// M2: an undeclared component on a stable entity is reported; a transient one is not.
#[test]
fn undeclared_components_are_reported() {
    assert_eq!(
        build(&rules_of(Cached(false))).undeclared_state_components(),
        1
    );
    assert_eq!(
        build(&rules_of(Cached(true))).undeclared_state_components(),
        0
    );
    assert_eq!(build(&life(1)).undeclared_state_components(), 0);
}

/// M3: a restored `StableId` at or above the allocator counter is refused.
#[test]
fn restore_refuses_ids_beyond_the_counter() {
    let rules = life(1);
    let sim = build(&rules);
    let mut snap: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    snap.next_stable_id = 49;
    let last = snap.entities[49];
    assert_eq!(
        restore(&snap, &rules).unwrap_err(),
        RestoreError::StableIdBeyondCounter(last)
    );
}

/// M5: restoring with another tick length is refused.
#[test]
fn restore_refuses_another_tick_length() {
    let rules = life(1);
    let sim = build(&rules);
    let snap: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    let cfg = SimConfig {
        tick_hz: 10,
        ..SimConfig::default()
    };
    assert_eq!(
        Sim::restore(&snap, cfg, &rules, &key()).unwrap_err(),
        RestoreError::TickMismatch {
            snapshot: 50,
            current: 100
        }
    );
}
