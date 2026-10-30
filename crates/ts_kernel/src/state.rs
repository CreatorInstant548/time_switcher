//! État enregistrable : encodage canonique (`StateHash`), registre des composants et des
//! ressources d'état, empreintes par famille et globale.

use core::any::Any;
use core::fmt;
use std::collections::BTreeMap;

use bevy_ecs::component::{Component, ComponentId};
use bevy_ecs::entity::Entity;
use bevy_ecs::lifecycle::HookContext;
use bevy_ecs::resource::Resource;
use bevy_ecs::world::{DeferredWorld, World};

use crate::canonical::{
    CanonicalError, StateDecode, StateReader, decode_component, decode_resource,
};
use crate::{DayOrigin, Money, SimTime, StableId};

/// Contexte BLAKE3 de l'empreinte d'une famille, écrit en dur.
pub(crate) const FAMILY_CONTEXT: &str = "time_switcher 2026-10-30 state family v1";
/// Contexte BLAKE3 de l'empreinte globale, écrit en dur.
pub(crate) const GLOBAL_CONTEXT: &str = "time_switcher 2026-10-30 state global v1";
/// Famille du noyau : horloge, origine des jours, compteurs et ordonnanceur.
pub const KERNEL_FAMILY: &str = "kernel";

/// Destination des octets canoniques : un hacheur ou un tampon.
enum Sink<'a> {
    Hasher(&'a mut blake3::Hasher),
    Bytes(&'a mut Vec<u8>),
}

// spec: revues/architecture.md §11.3 ; carnet architecture-securite.md §9.10 L3
/// Écrivain de l'encodage canonique : entiers en petit-boutiste, flottants par leurs bits
/// (`to_bits`, jamais de conversion décimale), suites précédées de leur longueur en `u64`.
pub struct StateWriter<'a>(Sink<'a>);

impl fmt::Debug for StateWriter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StateWriter(..)")
    }
}

impl StateWriter<'_> {
    /// Raw bytes, with no length prefix. Crate-private (M4): callers outside the kernel only
    /// get fixed-width or length-prefixed writers, so their encodings stay self-delimiting.
    pub(crate) fn raw(&mut self, bytes: &[u8]) {
        match &mut self.0 {
            Sink::Hasher(h) => {
                h.update(bytes);
            }
            Sink::Bytes(v) => v.extend_from_slice(bytes),
        }
    }

    /// Longueur d'une suite.
    pub fn count(&mut self, n: usize) {
        // Conversion exacte : `usize` tient sur 64 bits sur toutes nos cibles.
        self.u64(n as u64);
    }

    /// Octets précédés de leur longueur.
    pub fn bytes(&mut self, bytes: &[u8]) {
        self.count(bytes.len());
        self.raw(bytes);
    }

    /// Chaîne UTF-8 précédée de sa longueur en octets.
    pub fn str(&mut self, s: &str) {
        self.bytes(s.as_bytes());
    }

    /// `u8`.
    pub fn u8(&mut self, v: u8) {
        self.raw(&[v]);
    }

    /// `u32` LE.
    pub fn u32(&mut self, v: u32) {
        self.raw(&v.to_le_bytes());
    }

    /// `u64` LE.
    pub fn u64(&mut self, v: u64) {
        self.raw(&v.to_le_bytes());
    }

    /// `i64` LE.
    pub fn i64(&mut self, v: i64) {
        self.raw(&v.to_le_bytes());
    }

    /// `f64` par ses bits, LE.
    pub fn f64(&mut self, v: f64) {
        self.u64(v.to_bits());
    }
}

// spec: revues/architecture.md §11.3, §7.1 ; carnet architecture-securite.md §9.10 L3
/// Encodage canonique d'une valeur d'état, pour l'empreinte et pour l'instantané.
///
/// The encoding must be **self-delimiting** (prefix-free): no encoded value is a prefix of
/// another one of the same type, so concatenated values stay unambiguous. Use fixed-width
/// writers, or length-prefixed ones ([`StateWriter::bytes`], [`StateWriter::str`],
/// [`StateWriter::count`] before a sequence). It must depend only on the value: no
/// pointer, no hash-table order, no `Debug` text. Its decoding twin is
/// [`crate::StateDecode`].
pub trait StateHash {
    /// Écrit la valeur.
    fn hash_state(&self, w: &mut StateWriter<'_>);
}

macro_rules! state_hash_int {
    ($($t:ty),*) => {$(
        impl StateHash for $t {
            fn hash_state(&self, w: &mut StateWriter<'_>) {
                w.raw(&self.to_le_bytes());
            }
        }
    )*};
}
state_hash_int!(u8, u16, u32, u64, i8, i16, i32, i64);

impl StateHash for bool {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(u8::from(*self));
    }
}

impl StateHash for f32 {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u32(self.to_bits());
    }
}

impl StateHash for f64 {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.f64(*self);
    }
}

impl StateHash for StableId {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u64(self.to_snapshot_raw());
    }
}

impl StateHash for SimTime {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.as_millis());
    }
}

impl StateHash for DayOrigin {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.days());
    }
}

impl StateHash for Money {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.i64(self.cents());
    }
}

impl<T: StateHash> StateHash for Option<T> {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        match self {
            None => w.u8(0),
            Some(v) => {
                w.u8(1);
                v.hash_state(w);
            }
        }
    }
}

impl<T: StateHash> StateHash for [T] {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.count(self.len());
        for v in self {
            v.hash_state(w);
        }
    }
}

impl<T: StateHash> StateHash for Vec<T> {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        self.as_slice().hash_state(w);
    }
}

// spec: revues/architecture.md §6.1, §11.3
/// Composant : l'identité durable d'une entité. Seules les entités qui le portent entrent
/// dans l'empreinte et dans l'instantané, triées par [`StableId`].
///
/// It is immutable: its hooks keep [`StableIndex`] up to date on spawn, insert, removal
/// and despawn.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[component(immutable, on_insert = index_insert, on_discard = index_discard)]
pub struct StableEntity(StableId);

fn index_insert(mut world: DeferredWorld<'_>, ctx: HookContext) {
    let Some(id) = world.get::<StableEntity>(ctx.entity).map(|s| s.id()) else {
        return;
    };
    if let Some(mut index) = world.get_resource_mut::<StableIndex>() {
        match index.map.get(&id) {
            Some(&e) if e != ctx.entity => index.conflicts += 1,
            _ => {
                index.map.insert(id, ctx.entity);
            }
        }
    }
}

fn index_discard(mut world: DeferredWorld<'_>, ctx: HookContext) {
    let Some(id) = world.get::<StableEntity>(ctx.entity).map(|s| s.id()) else {
        return;
    };
    if let Some(mut index) = world.get_resource_mut::<StableIndex>()
        && index.map.get(&id) == Some(&ctx.entity)
    {
        index.map.remove(&id);
    }
}

// spec: revues/architecture.md §6.1, §9.2, §12.1
/// Resource: the `StableId → Entity` index of the entities that carry [`StableEntity`],
/// kept up to date by that component's hooks and rebuilt by a restore.
///
/// **Rule:** any durable reference (kept across ticks, in a component or a resource) is a
/// [`StableId`], never an `Entity`. An `Entity` is valid only within a tick: a restore
/// respawns every stable entity under a new `Entity`. Look it up here when needed.
///
/// **Rule:** an iteration whose effect depends on its order (accumulation, first match,
/// cross writes) sorts by [`StableId`], for example by walking this index, which is
/// sorted. Query order is not part of the state and changes on restore.
#[derive(Resource, Debug, Default)]
pub struct StableIndex {
    map: BTreeMap<StableId, Entity>,
    /// Insertions refused because the id was already mapped to another entity.
    conflicts: u32,
}

impl StableIndex {
    /// Entity currently carrying `id`.
    pub fn get(&self, id: StableId) -> Option<Entity> {
        self.map.get(&id).copied()
    }

    /// Stable entities, sorted by [`StableId`].
    pub fn iter(&self) -> impl Iterator<Item = (StableId, Entity)> + '_ {
        self.map.iter().map(|(&id, &e)| (id, e))
    }

    /// Number of stable entities.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// True if no entity is stable.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Number of entities given an id that another entity already carried: always zero in
    /// a sound world.
    pub const fn conflicts(&self) -> u32 {
        self.conflicts
    }
}

impl StableEntity {
    /// Entité d'identité `id`.
    pub const fn new(id: StableId) -> Self {
        Self(id)
    }

    /// Identité de l'entité.
    pub const fn id(self) -> StableId {
        self.0
    }
}

/// Valeur d'état capturée, opaque.
pub(crate) type Boxed = Box<dyn Any + Send + Sync>;

/// Valeurs capturées d'un type d'état.
pub(crate) enum Captured {
    Component(Vec<(StableId, Boxed)>),
    Resource(Option<Boxed>),
}

/// Écriture canonique d'un type de composant pour les entités données.
type WriteComponent = fn(&World, &[(StableId, Entity)], &mut StateWriter<'_>);
/// Copie des valeurs d'un type de composant pour les entités données.
type CaptureComponent = fn(&World, &[(StableId, Entity)]) -> Vec<(StableId, Boxed)>;
/// Decoding of a component type's values from the canonical encoding.
type DecodeComponent = fn(&mut StateReader<'_>) -> Result<Vec<(StableId, Boxed)>, CanonicalError>;
/// Decoding of a resource type's value from the canonical encoding.
type DecodeResource = fn(&mut StateReader<'_>) -> Result<Option<Boxed>, CanonicalError>;

/// Accès à un type d'état, sans connaître le type.
#[derive(Clone, Copy)]
pub(crate) enum StateKind {
    Component {
        write: WriteComponent,
        capture: CaptureComponent,
        /// Rend `false` si la valeur n'est pas du bon type.
        insert: fn(&mut World, Entity, &(dyn Any + Send + Sync)) -> bool,
        decode: DecodeComponent,
    },
    Resource {
        write: fn(&World, &mut StateWriter<'_>),
        capture: fn(&World) -> Option<Boxed>,
        /// Insère la valeur, ou retire la ressource si `None` ; `false` si le type ne va pas.
        insert: fn(&mut World, Option<&(dyn Any + Send + Sync)>) -> bool,
        decode: DecodeResource,
    },
}

/// Un type d'état déclaré par un greffon.
#[derive(Clone, Copy)]
pub(crate) struct StateEntry {
    pub(crate) family: &'static str,
    pub(crate) kind: StateKind,
}

fn write_component<C: Component + StateHash>(
    world: &World,
    entities: &[(StableId, Entity)],
    w: &mut StateWriter<'_>,
) {
    let present: Vec<_> = entities
        .iter()
        .filter_map(|&(id, e)| world.get::<C>(e).map(|c| (id, c)))
        .collect();
    w.count(present.len());
    for (id, c) in present {
        id.hash_state(w);
        c.hash_state(w);
    }
}

fn capture_component<C: Component + Clone>(
    world: &World,
    entities: &[(StableId, Entity)],
) -> Vec<(StableId, Boxed)> {
    entities
        .iter()
        .filter_map(|&(id, e)| {
            world
                .get::<C>(e)
                .map(|c| (id, Box::new(c.clone()) as Boxed))
        })
        .collect()
}

fn insert_component<C: Component + Clone>(
    world: &mut World,
    e: Entity,
    v: &(dyn Any + Send + Sync),
) -> bool {
    let Some(c) = v.downcast_ref::<C>() else {
        return false;
    };
    world.entity_mut(e).insert(c.clone());
    true
}

fn write_resource<R: Resource + StateHash>(world: &World, w: &mut StateWriter<'_>) {
    match world.get_resource::<R>() {
        None => w.u8(0),
        Some(r) => {
            w.u8(1);
            r.hash_state(w);
        }
    }
}

fn capture_resource<R: Resource + Clone>(world: &World) -> Option<Boxed> {
    world
        .get_resource::<R>()
        .map(|r| Box::new(r.clone()) as Boxed)
}

fn insert_resource<R: Resource + Clone>(
    world: &mut World,
    v: Option<&(dyn Any + Send + Sync)>,
) -> bool {
    match v.map(|v| v.downcast_ref::<R>()) {
        None => {
            world.remove_resource::<R>();
            true
        }
        Some(Some(r)) => {
            world.insert_resource(r.clone());
            true
        }
        Some(None) => false,
    }
}

impl StateEntry {
    pub(crate) fn component<C: Component + Clone + StateHash + StateDecode>(
        family: &'static str,
    ) -> Self {
        Self {
            family,
            kind: StateKind::Component {
                write: write_component::<C>,
                capture: capture_component::<C>,
                insert: insert_component::<C>,
                decode: decode_component::<C>,
            },
        }
    }

    pub(crate) fn resource<R: Resource + Clone + StateHash + StateDecode>(
        family: &'static str,
    ) -> Self {
        Self {
            family,
            kind: StateKind::Resource {
                write: write_resource::<R>,
                capture: capture_resource::<R>,
                insert: insert_resource::<R>,
                decode: decode_resource::<R>,
            },
        }
    }
}

/// Entités d'identité durable, triées par [`StableId`].
pub(crate) fn stable_entities(world: &World) -> Vec<(StableId, Entity)> {
    world
        .get_resource::<StableIndex>()
        .map(|index| index.iter().collect())
        .unwrap_or_default()
}

// spec: revues/architecture.md §11.3 ; carnet architecture-securite.md §9.10 L3
/// Components carried by stable entities that are not in `known` (M2): such a component
/// is neither hashed nor saved, so a restore would drop it.
pub(crate) fn undeclared_components(world: &World, known: &[ComponentId]) -> usize {
    stable_entities(world)
        .into_iter()
        .filter_map(|(_, e)| world.get_entity(e).ok())
        .map(|e| {
            e.archetype()
                .components()
                .iter()
                .filter(|c| !known.contains(c))
                .count()
        })
        .sum()
}

// spec: revues/architecture.md §11.3
/// Empreinte BLAKE3 de 32 octets.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateDigest([u8; 32]);

impl StateDigest {
    /// Octets de l'empreinte.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for StateDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

// spec: revues/architecture.md §7.1, §11.3 ; carnet architecture-securite.md §9.10 L4
/// Empreintes d'état : une par famille, triées par nom, et l'empreinte globale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateHashes {
    /// Empreinte globale, sur les empreintes des familles triées par nom.
    pub global: StateDigest,
    /// Empreinte de chaque famille, triées par nom.
    pub families: Vec<(&'static str, StateDigest)>,
}

impl StateHashes {
    /// Empreinte d'une famille.
    pub fn family(&self, name: &str) -> Option<StateDigest> {
        self.families
            .iter()
            .find(|(n, _)| *n == name)
            .map(|&(_, d)| d)
    }
}

/// Encode une famille : son nom, puis chaque type d'état dans l'ordre de déclaration.
pub(crate) fn write_family(
    world: &World,
    entities: &[(StableId, Entity)],
    family: &'static str,
    entries: &[StateEntry],
    kernel: Option<&dyn Fn(&mut StateWriter<'_>)>,
    w: &mut StateWriter<'_>,
) {
    w.str(family);
    if let Some(k) = kernel {
        k(w);
    }
    for (i, e) in entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.family == family)
    {
        // Conversion exacte : moins de 2^32 types d'état.
        w.u32(i as u32);
        match e.kind {
            StateKind::Component { write, .. } => write(world, entities, w),
            StateKind::Resource { write, .. } => write(world, w),
        }
    }
}

/// Familles présentes, triées : le noyau et celles des types déclarés.
pub(crate) fn families(entries: &[StateEntry]) -> Vec<&'static str> {
    let mut names: Vec<_> = entries.iter().map(|e| e.family).collect();
    names.push(KERNEL_FAMILY);
    names.sort_unstable();
    names.dedup();
    names
}

/// Empreintes de toutes les familles et globale.
pub(crate) fn hash_world(
    world: &World,
    entries: &[StateEntry],
    kernel: &dyn Fn(&mut StateWriter<'_>),
) -> StateHashes {
    let entities = stable_entities(world);
    let families: Vec<_> = families(entries)
        .into_iter()
        .map(|f| {
            let mut h = blake3::Hasher::new_derive_key(FAMILY_CONTEXT);
            let k = (f == KERNEL_FAMILY).then_some(kernel);
            write_family(
                world,
                &entities,
                f,
                entries,
                k,
                &mut StateWriter(Sink::Hasher(&mut h)),
            );
            (f, StateDigest(*h.finalize().as_bytes()))
        })
        .collect();
    let mut h = blake3::Hasher::new_derive_key(GLOBAL_CONTEXT);
    let mut w = StateWriter(Sink::Hasher(&mut h));
    w.count(families.len());
    for (name, d) in &families {
        w.str(name);
        w.raw(d.as_bytes());
    }
    StateHashes {
        global: StateDigest(*h.finalize().as_bytes()),
        families,
    }
}

/// Contexte BLAKE3 de l'empreinte d'une valeur isolée, écrit en dur.
const VALUE_CONTEXT: &str = "time_switcher 2026-10-30 state value v1";

// spec: revues/architecture.md §11.3 ; carnet architecture-securite.md §10
/// Canonical bytes of one value: exactly what [`StateHash::hash_state`] writes, the bytes
/// the value has inside a snapshot. For tests of encodings outside the kernel; it gives no
/// [`StateWriter`].
pub fn state_hash_bytes<T: StateHash + ?Sized>(value: &T) -> Vec<u8> {
    encode_with(|w| value.hash_state(w))
}

// spec: revues/architecture.md §11.3 ; carnet architecture-securite.md §10
/// BLAKE3 digest of [`state_hash_bytes`], under a context of its own (never equal to a
/// family digest).
pub fn state_hash_digest<T: StateHash + ?Sized>(value: &T) -> StateDigest {
    StateDigest(blake3::derive_key(VALUE_CONTEXT, &state_hash_bytes(value)))
}

/// Bytes written by `f`, in a buffer.
pub(crate) fn encode_with(f: impl FnOnce(&mut StateWriter<'_>)) -> Vec<u8> {
    let mut out = Vec::new();
    f(&mut StateWriter(Sink::Bytes(&mut out)));
    out
}

/// Encodage canonique complet, familles triées, dans un tampon.
pub(crate) fn encode_world(
    world: &World,
    entries: &[StateEntry],
    kernel: &dyn Fn(&mut StateWriter<'_>),
) -> Vec<u8> {
    let entities = stable_entities(world);
    let mut out = Vec::new();
    let mut w = StateWriter(Sink::Bytes(&mut out));
    for f in families(entries) {
        let k = (f == KERNEL_FAMILY).then_some(kernel);
        write_family(world, &entities, f, entries, k, &mut w);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(v: &dyn Fn(&mut StateWriter<'_>)) -> Vec<u8> {
        let mut out = Vec::new();
        v(&mut StateWriter(Sink::Bytes(&mut out)));
        out
    }

    /// Vecteurs figés de l'encodage des valeurs (L3).
    #[test]
    fn value_encoding_reference_vectors() {
        assert_eq!(encode(&|w| 0x0102_u16.hash_state(w)), [2, 1]);
        assert_eq!(encode(&|w| (-1_i32).hash_state(w)), [0xff; 4]);
        assert_eq!(encode(&|w| true.hash_state(w)), [1]);
        assert_eq!(encode(&|w| 1.0_f32.hash_state(w)), [0, 0, 0x80, 0x3f]);
        assert_eq!(
            encode(&|w| (-0.0_f64).hash_state(w)),
            [0, 0, 0, 0, 0, 0, 0, 0x80]
        );
        assert_ne!(
            encode(&|w| 0.0_f64.hash_state(w)),
            encode(&|w| (-0.0_f64).hash_state(w))
        );
        assert_eq!(
            encode(&|w| Some(StableId::from_snapshot_raw(5)).hash_state(w)),
            [1, 5, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(encode(&|w| None::<u8>.hash_state(w)), [0]);
        assert_eq!(
            encode(&|w| vec![7_u8, 8].hash_state(w)),
            [2, 0, 0, 0, 0, 0, 0, 0, 7, 8]
        );
        assert_eq!(
            encode(&|w| w.str("ab")),
            [2, 0, 0, 0, 0, 0, 0, 0, b'a', b'b']
        );
        assert_eq!(
            encode(&|w| Money::from_cents(-2).hash_state(w)),
            (-2_i64).to_le_bytes()
        );
    }

    /// AS-39: one value's bytes are those of `StateHash`, and its digest is frozen.
    #[test]
    fn single_value_reference_vectors() {
        assert_eq!(state_hash_bytes(&0x0102_u16), [2, 1]);
        assert_eq!(
            state_hash_bytes(&Some(StableId::from_snapshot_raw(5))),
            [1, 5, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            state_hash_bytes(&vec![7_u8, 8][..]),
            [2, 0, 0, 0, 0, 0, 0, 0, 7, 8]
        );
        let d = state_hash_digest(&Money::from_cents(-2));
        assert_eq!(
            d.as_bytes(),
            &blake3::derive_key(VALUE_CONTEXT, &(-2_i64).to_le_bytes())
        );
        assert_eq!(
            format!("{d:?}"),
            "78ea69c1a8c2495349302c234a0dafa59b8258ba78e8eafcdb86952eb8ac088f"
        );
        let family = blake3::derive_key(FAMILY_CONTEXT, &(-2_i64).to_le_bytes());
        assert_ne!(d.as_bytes(), &family);
    }
}
