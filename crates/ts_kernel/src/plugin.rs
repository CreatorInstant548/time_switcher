//! Greffons : le trait `Plugin`, le constructeur `SimBuilder` et l'empreinte de règles.

use core::fmt;

use bevy_ecs::component::{Component, ComponentId};
use bevy_ecs::resource::Resource;
use bevy_ecs::schedule::{IntoScheduleConfigs, Schedule};
use bevy_ecs::system::ScheduleSystem;
use bevy_ecs::world::World;

use crate::state::StateEntry;
use crate::{Phase, StateHash};

// spec: revues/architecture.md §5.3
/// Greffon de règles. `standard_rules()` (`ts_game`) en donne la liste unique.
///
/// Un greffon déclare dans [`Plugin::build`] ses composants, ses ressources et ses systèmes par
/// phase, et dans [`Plugin::dependencies`] les greffons dont il dépend. Désactiver un greffon
/// ne change pas les règles des autres ; la construction échoue si un greffon actif dépend d'un
/// greffon coupé.
///
/// **Time-skip contract** (architecture §8.1): while no entity carries
/// [`crate::LodComplete`], [`crate::Sim::run_until`] skips the ticks that have no due event.
/// A system must therefore leave the state unchanged on a tick with no due event and no
/// entity in full simulation: no per-tick counter, no `Local` accumulator, no polling of the
/// clock. Otherwise skipping and ticking would give different worlds.
///
/// **No hidden state between ticks** (architecture §11.1, §12.4): a restore builds fresh
/// systems and keeps only the declared state, yet restoring at any tick k > 0 must give
/// exactly the uninterrupted run. A plugin therefore keeps **no** state from one tick to the
/// next outside its declared state (the components and resources registered with
/// [`SimBuilder::state_component`] and [`SimBuilder::state_resource`]). In particular
/// (AS-27):
/// - no `Local<T>` kept from one tick to the next;
/// - no game logic based on `Added` or `Changed` (nor on `is_changed`): change ticks
///   restart on restore;
/// - a `Message` is written **and** read within the same tick, with an explicit phase
///   order between writer and reader;
/// - no undeclared game resource or game entity: every persistent entity carries
///   [`crate::StableEntity`];
/// - a cache that can be rebuilt from the state is declared with
///   [`SimBuilder::transient_component`].
///
/// Enforced before the first rule plugin merges (phase 1) by a generic restore test and
/// `clippy.toml` bans, see #31 (AS-27).
// TODO(#31): enforce this contract automatically.
// spec: revues/architecture.md §11.1, §12.4
pub trait Plugin {
    /// Nom stable et unique du greffon (par exemple `"economy"`).
    fn name(&self) -> &'static str;

    /// Version des règles du greffon : elle entre dans l'[`RulesFingerprint`] et change dès
    /// qu'une règle du greffon change.
    fn version(&self) -> u32;

    /// Vrai si le greffon peut être coupé par [`crate::SimConfig::disabled_plugins`].
    fn disableable(&self) -> bool {
        false
    }

    /// Plugins this one needs. Each must be active and come **earlier** in the rules, so
    /// that it is built first. They are checked before any [`Plugin::build`] runs, so a
    /// wrong order is a [`crate::BuildError`], never a panic in `build`.
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    /// Déclare le contenu du greffon.
    fn build(&self, b: &mut SimBuilder);
}

// spec: revues/architecture.md §5.3, §9.1
/// Ce qu'un greffon déclare pendant la construction d'une [`crate::Sim`].
pub struct SimBuilder {
    pub(crate) world: World,
    pub(crate) phases: Vec<Schedule>,
    pub(crate) current: &'static str,
    /// Types d'état, dans l'ordre de déclaration.
    pub(crate) state: Vec<StateEntry>,
    /// Migrations déclarées, `(greffon, version d'origine)`.
    pub(crate) migrations: Vec<(&'static str, u32)>,
    /// Components allowed on stable entities: declared state and transient components.
    pub(crate) known_components: Vec<ComponentId>,
}

impl SimBuilder {
    /// Enregistre un type de composant.
    pub fn register_component<C: Component>(&mut self) -> &mut Self {
        self.world.register_component::<C>();
        self
    }

    /// Ajoute ou remplace une ressource.
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    /// Ajoute des systèmes à une phase.
    pub fn add_systems<M>(
        &mut self,
        phase: Phase,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.phases[phase.index()].add_systems(systems);
        self
    }

    // spec: revues/architecture.md §11.3, §12.1 ; carnet architecture-securite.md §9.10 L3
    /// Déclare un composant d'état de la famille `family` : il entre dans l'empreinte et dans
    /// l'instantané, pour les entités qui portent [`crate::StableEntity`]. L'ordre de
    /// déclaration fixe l'ordre d'encodage.
    pub fn state_component<C: Component + Clone + StateHash>(
        &mut self,
        family: &'static str,
    ) -> &mut Self {
        let id = self.world.register_component::<C>();
        self.known_components.push(id);
        self.state.push(StateEntry::component::<C>(family));
        self
    }

    // spec: revues/architecture.md §11.3
    /// Declares a component that stable entities may carry without it being state: a cache
    /// that the plugin rebuilds from the state. It is neither hashed nor saved, and a restore
    /// drops it.
    pub fn transient_component<C: Component>(&mut self) -> &mut Self {
        let id = self.world.register_component::<C>();
        self.known_components.push(id);
        self
    }

    /// Déclare une ressource d'état de la famille `family`, comme [`Self::state_component`].
    pub fn state_resource<R: Resource + Clone + StateHash>(
        &mut self,
        family: &'static str,
    ) -> &mut Self {
        self.state.push(StateEntry::resource::<R>(family));
        self
    }

    // spec: revues/architecture.md §12.5 ; carnet architecture-securite.md §9.10 L5
    /// Déclare que le greffon en construction sait reprendre un instantané de sa version
    /// `version`. Sans cette déclaration, une autre version fait refuser la restauration.
    pub fn migrates_from(&mut self, version: u32) -> &mut Self {
        self.migrations.push((self.current, version));
        self
    }

    /// Monde en construction, pour placer un état initial.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

impl fmt::Debug for SimBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimBuilder")
            .field("current", &self.current)
            .finish_non_exhaustive()
    }
}

/// Contexte de dérivation de l'empreinte de règles (BLAKE3 `derive_key`), écrit en dur.
const RULES_CONTEXT: &str = "time_switcher 2026-10-30 rules fingerprint v1";

// spec: revues/architecture.md §5.3, §12.1
/// Empreinte de règles : BLAKE3 des noms et des versions des greffons actifs, **triés par
/// nom**. Elle est enregistrée dans chaque instantané et chaque manifeste de simulation.
///
/// Encodage : nombre de greffons (`u32` LE), puis pour chacun la longueur du nom (`u32` LE),
/// le nom en UTF-8 et la version (`u32` LE).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RulesFingerprint([u8; 32]);

impl RulesFingerprint {
    /// Empreinte d'une liste `(nom, version)`, dans n'importe quel ordre.
    ///
    /// # Panics
    ///
    /// Si la liste ou un nom dépasse `u32::MAX` éléments ou octets.
    pub fn of(plugins: &[(&str, u32)]) -> Self {
        let mut sorted = plugins.to_vec();
        sorted.sort_unstable();
        let mut h = blake3::Hasher::new_derive_key(RULES_CONTEXT);
        h.update(&len_u32(sorted.len()).to_le_bytes());
        for (name, version) in sorted {
            h.update(&len_u32(name.len()).to_le_bytes());
            h.update(name.as_bytes());
            h.update(&version.to_le_bytes());
        }
        Self(*h.finalize().as_bytes())
    }

    /// Octets de l'empreinte.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Empreinte lue dans un instantané. **Réservé à `ts_persist`.**
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

fn len_u32(n: usize) -> u32 {
    let Ok(n) = u32::try_from(n) else {
        panic!("longueur hors de u32 dans l'empreinte de règles");
    };
    n
}

impl fmt::Debug for RulesFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RulesFingerprint(")?;
        for b in &self.0[..8] {
            write!(f, "{b:02x}")?;
        }
        f.write_str("…)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn fingerprint_ignores_order_and_sees_versions() {
        let a = RulesFingerprint::of(&[("economy", 1), ("combat", 3)]);
        let b = RulesFingerprint::of(&[("combat", 3), ("economy", 1)]);
        assert_eq!(a, b);
        assert_ne!(a, RulesFingerprint::of(&[("combat", 3), ("economy", 2)]));
        assert_ne!(a, RulesFingerprint::of(&[("combat", 3)]));
        // Noms collés : l'encodage par longueur les sépare.
        assert_ne!(
            RulesFingerprint::of(&[("ab", 1), ("c", 1)]),
            RulesFingerprint::of(&[("a", 1), ("bc", 1)])
        );
    }

    /// Vecteurs figés : changer l'encodage change toutes les empreintes enregistrées.
    #[test]
    fn fingerprint_reference_vectors() {
        assert_eq!(
            hex(RulesFingerprint::of(&[]).as_bytes()),
            "e113a6d6a5a0134e05c73e4a419109676c05cd114d49fbfded9000b097b729d8"
        );
        assert_eq!(
            hex(RulesFingerprint::of(&[("combat", 3), ("economy", 1)]).as_bytes()),
            "a6eff6530d607d78fc72d4d0933a629b46ac98faa4b30973eb6440cd2c2ab373"
        );
        // L'encodage documenté, écrit à la main.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2_u32.to_le_bytes());
        for (name, version) in [("combat", 3_u32), ("economy", 1)] {
            bytes.extend_from_slice(&u32::try_from(name.len()).unwrap().to_le_bytes());
            bytes.extend_from_slice(name.as_bytes());
            bytes.extend_from_slice(&version.to_le_bytes());
        }
        assert_eq!(
            &blake3::derive_key(RULES_CONTEXT, &bytes),
            RulesFingerprint::of(&[("economy", 1), ("combat", 3)]).as_bytes()
        );
    }
}
