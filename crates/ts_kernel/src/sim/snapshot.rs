//! Instantané : le trait `Snapshot` et sa mise en œuvre en mémoire. Le codec en octets vient
//! avec `ts_persist` (T8) ; le décodeur canonique est dans `crate::canonical` (#46).

use core::fmt;
use std::collections::BTreeMap;

use super::{
    BuildError, EventQueue, LodComplete, Sim, SimClock, SimConfig, StableIds, TimedEvent,
    complete_ids, write_kernel,
};
use crate::canonical::{self, CanonicalError, CanonicalLimits};
use crate::rng::WorldKey;
use crate::state::{Captured, StableIndex, StateEntry, StateKind, encode_world, stable_entities};
use crate::{
    DayOrigin, Plugin, RulesFingerprint, Scheduled, Scheduler, SimTime, StableEntity, StableId,
    StableIdAllocator,
};

// spec: revues/architecture.md §7.1, §12.3
/// Portée d'un instantané. Les instantanés partiels (morceaux modifiés) viendront avec la
/// persistance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotScope {
    /// Tout l'état.
    Full,
}

// spec: revues/architecture.md §7.1, §12.5 ; carnet architecture-securite.md §9.10 L5
/// Échec d'une restauration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreError {
    /// La simulation ne se construit pas avec ces règles et cette configuration.
    Build(BuildError),
    /// L'empreinte de règles diffère de celle de l'instantané, sans migration déclarée.
    RulesMismatch {
        /// Empreinte enregistrée.
        snapshot: RulesFingerprint,
        /// Empreinte des règles fournies.
        current: RulesFingerprint,
    },
    /// Les types d'état déclarés ne correspondent pas à ceux de l'instantané.
    StateLayoutMismatch,
    /// L'instantané est incohérent.
    Corrupt(&'static str),
    /// The configuration's tick length differs from the snapshot's (M5).
    TickMismatch {
        /// Tick length recorded in the snapshot, in milliseconds.
        snapshot: i64,
        /// Tick length of the given configuration, in milliseconds.
        current: i64,
    },
    /// A game-space `StableId` is not below the restored allocator counter (M3): the
    /// allocator would hand it out again.
    StableIdBeyondCounter(StableId),
}

impl fmt::Display for RestoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Build(e) => write!(f, "construction impossible : {e}"),
            Self::RulesMismatch { snapshot, current } => write!(
                f,
                "règles différentes ({snapshot:?} enregistrées, {current:?} fournies) sans migration"
            ),
            Self::StateLayoutMismatch => f.write_str("types d'état différents de l'instantané"),
            Self::Corrupt(what) => write!(f, "instantané incohérent : {what}"),
            Self::TickMismatch { snapshot, current } => write!(
                f,
                "tick length differs: {snapshot} ms in the snapshot, {current} ms configured"
            ),
            Self::StableIdBeyondCounter(id) => {
                write!(f, "{id:?} is not below the StableId allocator counter")
            }
        }
    }
}

impl std::error::Error for RestoreError {}

// spec: revues/architecture.md §7.1, §12.1 ; carnet architecture-securite.md §9.10 L4, L5
/// Instantané d'une [`Sim`], pris entre deux ticks.
///
/// Il contient l'empreinte et la liste des règles, la date, l'origine des jours, les
/// compteurs (`StableIdAllocator`, `seq`), les événements en attente et les types d'état
/// déclarés par les greffons. **Jamais la clé du monde ni les clés de flux** : `restore` les
/// redérive de la clé qu'on lui passe.
///
/// Seules les entités qui portent [`StableEntity`] sont enregistrées ; une entité sans
/// identité durable, créée par la construction d'un greffon, reste telle que la construction
/// l'a faite.
///
/// Restore contract (F4: snapshot, restore, then N ticks gives the same state as running
/// straight through):
/// - stable entities are respawned in `StableId` order, with their declared state
///   components and their [`crate::LodComplete`] marker; their `Entity` values change;
/// - durable references therefore use `StableId`, and [`crate::StableIndex`] maps them to
///   entities;
/// - any iteration whose effect depends on its order sorts by `StableId`, since query order
///   is not part of the state;
/// - systems are built fresh, so nothing outside the declared state survives a restore:
///   `Local<T>` values, change detection (`Added`, `Changed`, `is_changed`), pending
///   `Message`s, undeclared resources and entities without [`StableEntity`] all start
///   over. A rule plugin therefore keeps no state from one tick to the next outside its
///   declared state: no `Local<T>` across ticks, no game logic on `Added` or `Changed`,
///   `Message`s written and read within the same tick with an explicit phase order, no
///   undeclared game resource or entity, and rebuildable caches declared with
///   `transient_component` (AS-27, see [`Plugin`]);
/// - enforced before the first rule plugin merges (phase 1) by a generic restore test and
///   `clippy.toml` bans, see #31 (AS-27).
// TODO(#31): enforce the no-hidden-state contract automatically.
// spec: revues/architecture.md §11.1, §12.4
pub trait Snapshot: Sized {
    /// Prend l'instantané.
    fn snapshot(sim: &Sim, scope: SnapshotScope) -> Self;

    /// Reconstruit une simulation : construction par `rules` et `cfg`, contrôle de
    /// l'empreinte de règles, puis remise en place de l'état.
    ///
    /// # Errors
    ///
    /// Voir [`RestoreError`].
    fn restore(
        &self,
        cfg: SimConfig,
        rules: &[Box<dyn Plugin>],
        world_key: &WorldKey,
    ) -> Result<Sim, RestoreError>;
}

// spec: revues/architecture.md §12.1 ; carnet architecture-securite.md §9.10 L4, §10
/// Instantané en mémoire : copies des valeurs. [`MemorySnapshot::header_bytes`] and
/// [`MemorySnapshot::canonical_bytes`] give its bytes, and
/// [`MemorySnapshot::from_canonical`] decodes them back; a host stores them with
/// `ts_persist`.
pub struct MemorySnapshot {
    rules: RulesFingerprint,
    plugins: Vec<(String, u32)>,
    tick_ms: i64,
    now: SimTime,
    day_origin: DayOrigin,
    next_stable_id: u64,
    next_seq: u64,
    events: Vec<Scheduled<TimedEvent>>,
    entities: Vec<StableId>,
    /// Stable entities that carry `LodComplete`, sorted.
    complete: Vec<StableId>,
    /// Famille et genre (composant ou non) de chaque type d'état, dans l'ordre de déclaration.
    layout: Vec<(&'static str, bool)>,
    captured: Vec<Captured>,
    canonical: Vec<u8>,
}

impl fmt::Debug for MemorySnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemorySnapshot")
            .field("rules", &self.rules)
            .field("now", &self.now)
            .field("entities", &self.entities.len())
            .field("events", &self.events.len())
            .finish_non_exhaustive()
    }
}

impl MemorySnapshot {
    /// Empreinte des règles de la simulation enregistrée.
    pub const fn rules_fingerprint(&self) -> RulesFingerprint {
        self.rules
    }

    /// Date de l'instantané.
    pub const fn now(&self) -> SimTime {
        self.now
    }

    /// Encodage canonique de tout l'état enregistré, familles triées (celui de l'empreinte).
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    // spec: revues/architecture.md §12.1 ; carnet architecture-securite.md §10
    /// What the canonical state does not hold: rules fingerprint, tick length, date and
    /// active plugins (layout in `crate::canonical`). Never a key (L4).
    pub fn header_bytes(&self) -> Vec<u8> {
        canonical::write_header(&self.rules, self.tick_ms, self.now, &self.plugins)
    }

    // spec: revues/architecture.md §12.1 ; carnet architecture-securite.md §10, SEC0-23
    /// Decodes a snapshot from [`Self::header_bytes`] and [`Self::canonical_bytes`], read
    /// against the state types of `layout`. The bytes are untrusted: they are bounded by
    /// `limits`, checked for canonical form, and the date of the header must equal the date
    /// of the state. Nothing else is trusted either: [`Snapshot::restore`] keeps all its
    /// checks (rules fingerprint, layout, tick length, counters, events) and is the only
    /// path to a [`Sim`]. The snapshot keeps `state` as its canonical bytes: an owned
    /// buffer is taken without a copy.
    ///
    /// # Errors
    ///
    /// The first failure, as a typed [`CanonicalError`]; never a panic.
    pub fn from_canonical(
        header: &[u8],
        state: impl Into<Vec<u8>>,
        layout: &StateLayout,
        limits: &CanonicalLimits,
    ) -> Result<Self, CanonicalError> {
        let state = state.into();
        let d = canonical::decode(header, &state, &layout.entries, limits)?;
        Ok(Self {
            rules: d.header.rules,
            plugins: d.header.plugins,
            tick_ms: d.header.tick_ms,
            now: d.kernel.now,
            day_origin: d.kernel.day_origin,
            next_stable_id: d.kernel.next_stable_id,
            next_seq: d.kernel.next_seq,
            events: d.kernel.events,
            entities: d.kernel.entities,
            complete: d.kernel.complete,
            layout: layout_of(&layout.entries),
            captured: d.captured,
            canonical: state,
        })
    }
}

/// Family and kind (component or not) of each state type, in declaration order.
fn layout_of(entries: &[StateEntry]) -> Vec<(&'static str, bool)> {
    entries
        .iter()
        .map(|e| (e.family, matches!(e.kind, StateKind::Component { .. })))
        .collect()
}

/// Key of the world that `StateLayout::of` builds and drops at once: public, since
/// nothing of that world is kept but the declared state types.
const LAYOUT_KEY: [u8; 16] = *b"ts layout only\0\0";

// spec: revues/architecture.md §12.1 ; carnet architecture-securite.md §10
/// State types declared by a rule set, in declaration order: what decoding a snapshot
/// needs to read the values of each type.
pub struct StateLayout {
    entries: Vec<StateEntry>,
}

impl fmt::Debug for StateLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(layout_of(&self.entries)).finish()
    }
}

impl StateLayout {
    /// Layout of `rules` with `cfg`. The rules are built once into a world that is dropped
    /// at once (with a fixed public key, private to the kernel): no key is needed to decode.
    ///
    /// Contract:
    /// - `cfg` and `rules` must be those later passed to [`Snapshot::restore`];
    /// - the state types a plugin declares must not depend on the world key nor on drawn
    ///   values. Should they, the save is refused by `restore` (value type or re-encoding),
    ///   never misread;
    /// - like [`Sim::build`], the first call fixes the global thread pool, to
    ///   `cfg.threads`.
    ///
    /// # Errors
    ///
    /// The [`BuildError`] of building these rules.
    pub fn of(cfg: &SimConfig, rules: &[Box<dyn Plugin>]) -> Result<Self, BuildError> {
        let sim = Sim::build(cfg.clone(), rules, &WorldKey::from_bytes(LAYOUT_KEY))?;
        Ok(Self { entries: sim.state })
    }
}

fn check_rules(sim: &Sim, snap: &MemorySnapshot) -> Result<(), RestoreError> {
    if sim.rules == snap.rules {
        return Ok(());
    }
    let mismatch = RestoreError::RulesMismatch {
        snapshot: snap.rules,
        current: sim.rules,
    };
    if sim.active.len() != snap.plugins.len() {
        return Err(mismatch);
    }
    for &(name, version) in &sim.active {
        let Some(&(_, old)) = snap.plugins.iter().find(|(n, _)| n == name) else {
            return Err(mismatch);
        };
        if old != version && !sim.migrations.contains(&(name, old)) {
            return Err(mismatch);
        }
    }
    Ok(())
}

impl Snapshot for MemorySnapshot {
    fn snapshot(sim: &Sim, scope: SnapshotScope) -> Self {
        let SnapshotScope::Full = scope;
        let world = &sim.world;
        debug_assert_eq!(
            sim.undeclared_state_components(),
            0,
            "a stable entity carries a component that is neither state nor transient (M2)"
        );
        debug_assert_eq!(
            world.resource::<StableIndex>().conflicts(),
            0,
            "two entities carry the same StableId"
        );
        let entities = stable_entities(world);
        let clock = world.resource::<SimClock>();
        let queue = world.resource::<EventQueue>();
        Self {
            rules: sim.rules,
            plugins: sim.active.iter().map(|&(n, v)| (n.to_owned(), v)).collect(),
            tick_ms: sim.tick_ms,
            now: clock.now,
            day_origin: clock.day_origin,
            next_stable_id: world.resource::<StableIds>().0.to_snapshot_raw(),
            next_seq: queue.next_seq(),
            events: queue.pending().into_iter().cloned().collect(),
            entities: entities.iter().map(|&(id, _)| id).collect(),
            complete: complete_ids(world, &entities),
            layout: layout_of(&sim.state),
            captured: sim
                .state
                .iter()
                .map(|e| match e.kind {
                    StateKind::Component { capture, .. } => {
                        Captured::Component(capture(world, &entities))
                    }
                    StateKind::Resource { capture, .. } => Captured::Resource(capture(world)),
                })
                .collect(),
            canonical: encode_world(world, &sim.state, &|w| write_kernel(world, w)),
        }
    }

    fn restore(
        &self,
        cfg: SimConfig,
        rules: &[Box<dyn Plugin>],
        world_key: &WorldKey,
    ) -> Result<Sim, RestoreError> {
        let mut sim = Sim::build(cfg, rules, world_key).map_err(RestoreError::Build)?;
        check_rules(&sim, self)?;
        if layout_of(&sim.state) != self.layout {
            return Err(RestoreError::StateLayoutMismatch);
        }
        if self.tick_ms != sim.tick_ms {
            return Err(RestoreError::TickMismatch {
                snapshot: self.tick_ms,
                current: sim.tick_ms,
            });
        }
        if self.now.as_millis().rem_euclid(sim.tick_ms) != 0 {
            return Err(RestoreError::Corrupt("date hors de la grille des ticks"));
        }
        let ids = StableIdAllocator::from_snapshot_raw(self.next_stable_id)
            .ok_or(RestoreError::Corrupt("compteur de StableId"))?;
        if let Some(&id) = self
            .entities
            .iter()
            .find(|id| !id.is_map() && id.index() >= self.next_stable_id)
        {
            return Err(RestoreError::StableIdBeyondCounter(id));
        }
        let queue = Scheduler::restore(self.now, self.next_seq, self.events.clone())
            .ok_or(RestoreError::Corrupt("rangs des événements"))?;

        let world = &mut sim.world;
        // Seules les entités d'état sont remplacées. Dans `bevy_ecs` 0.20, les ressources et
        // les observateurs sont aussi des entités : on ne vide donc pas tout le monde.
        for (_, e) in stable_entities(world) {
            world.despawn(e);
        }
        {
            let mut clock = world.resource_mut::<SimClock>();
            clock.now = self.now;
            clock.day_origin = self.day_origin;
        }
        world.insert_resource(StableIds(ids));
        world.insert_resource(EventQueue(queue));
        // Respawned in `StableId` order (I3); the `StableEntity` hooks rebuild the index (I2).
        let mut map = BTreeMap::new();
        for &id in &self.entities {
            let e = world.spawn(StableEntity::new(id)).id();
            if map.insert(id, e).is_some() {
                return Err(RestoreError::Corrupt("StableId en double"));
            }
        }
        // The full-simulation marker drives the time skip (I1).
        for id in &self.complete {
            let Some(&e) = map.get(id) else {
                return Err(RestoreError::Corrupt("LodComplete on an unknown StableId"));
            };
            world.entity_mut(e).insert(LodComplete);
        }
        for (entry, captured) in sim.state.iter().zip(&self.captured) {
            let ok = match (entry.kind, captured) {
                (StateKind::Component { insert, .. }, Captured::Component(values)) => values
                    .iter()
                    .all(|(id, v)| map.get(id).is_some_and(|&e| insert(world, e, v.as_ref()))),
                (StateKind::Resource { insert, .. }, Captured::Resource(v)) => {
                    insert(world, v.as_deref())
                }
                _ => false,
            };
            if !ok {
                return Err(RestoreError::Corrupt("valeur d'état"));
            }
        }
        world.clear_trackers();
        // The restored world must encode to exactly the recorded bytes (F4, AS-36): a value
        // decoded from bytes that does not encode back the same is refused.
        let world = &sim.world;
        if encode_world(world, &sim.state, &|w| write_kernel(world, w)) != self.canonical {
            return Err(RestoreError::Corrupt(
                "canonical encoding differs after restore",
            ));
        }
        Ok(sim)
    }
}

impl Sim {
    /// Prend un instantané (voir [`Snapshot`]).
    pub fn snapshot<S: Snapshot>(&self, scope: SnapshotScope) -> S {
        S::snapshot(self, scope)
    }

    /// Restaure une simulation depuis un instantané (voir [`Snapshot::restore`]).
    ///
    /// # Errors
    ///
    /// Voir [`RestoreError`].
    pub fn restore<S: Snapshot>(
        snap: &S,
        cfg: SimConfig,
        rules: &[Box<dyn Plugin>],
        world_key: &WorldKey,
    ) -> Result<Self, RestoreError> {
        snap.restore(cfg, rules, world_key)
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod canonical_tests;
