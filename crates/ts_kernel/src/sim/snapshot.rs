//! Instantané : le trait `Snapshot` et sa mise en œuvre en mémoire. Le codec en octets vient
//! avec `ts_persist` (T8).

use core::fmt;
use std::collections::BTreeMap;

use super::{
    BuildError, EventQueue, Sim, SimClock, SimConfig, StableIds, TimedEvent, write_kernel,
};
use crate::rng::WorldKey;
use crate::state::{Boxed, StateKind, encode_world, stable_entities};
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

/// Valeurs capturées d'un type d'état.
enum Captured {
    Component(Vec<(StableId, Boxed)>),
    Resource(Option<Boxed>),
}

// spec: revues/architecture.md §12.1 ; carnet architecture-securite.md §9.10 L4
/// Instantané en mémoire : copies des valeurs, sans format d'octets. Il sert aux tests et aux
/// remises à zéro ; [`MemorySnapshot::canonical_bytes`] donne l'encodage canonique de l'état.
pub struct MemorySnapshot {
    rules: RulesFingerprint,
    plugins: Vec<(&'static str, u32)>,
    now: SimTime,
    day_origin: DayOrigin,
    next_stable_id: u64,
    next_seq: u64,
    events: Vec<Scheduled<TimedEvent>>,
    entities: Vec<StableId>,
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
        let Some(&(_, old)) = snap.plugins.iter().find(|(n, _)| *n == name) else {
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
        let entities = stable_entities(world);
        let clock = world.resource::<SimClock>();
        let queue = world.resource::<EventQueue>();
        Self {
            rules: sim.rules,
            plugins: sim.active.clone(),
            now: clock.now,
            day_origin: clock.day_origin,
            next_stable_id: world.resource::<StableIds>().0.to_snapshot_raw(),
            next_seq: queue.next_seq(),
            events: queue.pending().into_iter().cloned().collect(),
            entities: entities.iter().map(|&(id, _)| id).collect(),
            layout: sim
                .state
                .iter()
                .map(|e| (e.family, matches!(e.kind, StateKind::Component { .. })))
                .collect(),
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
        let layout: Vec<_> = sim
            .state
            .iter()
            .map(|e| (e.family, matches!(e.kind, StateKind::Component { .. })))
            .collect();
        if layout != self.layout {
            return Err(RestoreError::StateLayoutMismatch);
        }
        if self.now.as_millis().rem_euclid(sim.tick_ms) != 0 {
            return Err(RestoreError::Corrupt("date hors de la grille des ticks"));
        }
        let ids = StableIdAllocator::from_snapshot_raw(self.next_stable_id)
            .ok_or(RestoreError::Corrupt("compteur de StableId"))?;
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
        let mut map = BTreeMap::new();
        for &id in &self.entities {
            let e = world.spawn(StableEntity::new(id)).id();
            if map.insert(id, e).is_some() {
                return Err(RestoreError::Corrupt("StableId en double"));
            }
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
