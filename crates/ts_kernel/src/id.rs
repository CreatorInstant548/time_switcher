//! Identifiants stables : `StableId` et son allocateur.

use core::fmt;

/// Bit de tête : marque l'espace de noms réservé à la carte (P28, K9).
const MAP_BIT: u64 = 1 << 63;

// spec: revues/architecture.md §6.1 ; revues/generation.md §7.3, P28 ; carnet architecture-securite.md §9.9 K9
/// Identité durable d'une entité référençable, jamais réutilisée.
///
/// Deux espaces de noms, séparés par le **bit 63** (décidé, K9) :
/// - bit 63 à 0 : entités créées en jeu, indices `0..=2^63 − 1`, données par
///   [`StableIdAllocator`] ;
/// - bit 63 à 1 : lieux de la carte générée (parcelles, bâtiments, rues…), seulement par
///   [`StableId::map`].
///
/// Aucune conversion publique depuis un entier : l'allocateur, [`StableId::map`] et
/// [`StableId::from_snapshot_raw`] sont les seuls constructeurs.
///
/// **Jamais envoyé au client** (architecture §6.1) : il reçoit des poignées éphémères.
///
/// Les extraits `compile_fail` ci-dessous ont chacun un **témoin** qui compile, pour qu'ils ne
/// puissent pas échouer pour une autre raison : rustdoc stable ne vérifie pas leur code
/// d'erreur, qui reste écrit pour la relecture et pour nightly.
///
/// ```
/// let _: ts_kernel::StableId = ts_kernel::StableId::from_snapshot_raw(5);
/// ```
///
/// ```compile_fail,E0277
/// let _: ts_kernel::StableId = 5_u64.into();
/// ```
///
/// ```compile_fail,E0423
/// let _ = ts_kernel::StableId(5);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableId(u64);

crate::assert_size!(StableId, 8);

impl StableId {
    /// Plus grand indice d'un espace de noms : `2^63 − 1`.
    pub const MAX_INDEX: u64 = MAP_BIT - 1;

    /// Identifiant de lieu de la carte d'indice `index`, ou `None` si `index` dépasse
    /// [`StableId::MAX_INDEX`]. Seul accès à l'espace de la carte (P28).
    pub const fn map(index: u64) -> Option<Self> {
        if index <= Self::MAX_INDEX {
            Some(Self(MAP_BIT | index))
        } else {
            None
        }
    }

    /// Vrai pour un identifiant de l'espace de la carte.
    pub const fn is_map(self) -> bool {
        self.0 & MAP_BIT != 0
    }

    /// Indice dans son espace de noms, sans le bit 63.
    pub const fn index(self) -> u64 {
        self.0 & Self::MAX_INDEX
    }

    /// Décodage d'un instantané. **Réservé à `ts_persist`** : aucune autre crate ne fabrique
    /// un identifiant depuis un entier.
    pub const fn from_snapshot_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Valeur brute, pour l'encodage d'un instantané. **Réservé à `ts_persist`.**
    pub const fn to_snapshot_raw(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for StableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = if self.is_map() { "map" } else { "game" };
        write!(f, "StableId({space}:{})", self.index())
    }
}

// spec: revues/architecture.md §6.1 ; carnet architecture-securite.md §9.9 K9
/// Compteur du monde qui alloue les [`StableId`] des entités créées en jeu.
///
/// Il n'est ni `Clone` ni `Default` : une copie, ou une remise à zéro implicite
/// (`std::mem::take`, `unwrap_or_default`, `..Default::default()`, `#[derive(Default)]` sur un
/// état qui le contient), allouerait de nouveau des identifiants déjà rendus. Un monde neuf
/// l'obtient par [`StableIdAllocator::for_new_world`] seulement. Il s'arrête après `2^63 − 1`
/// (K9) : au-delà, [`StableIdAllocator::allocate`] rend `None`, sans jamais déborder dans
/// l'espace de la carte.
///
/// Témoin, puis les deux refus :
///
/// ```
/// let mut alloc = ts_kernel::StableIdAllocator::for_new_world();
/// let _ = alloc.allocate();
/// ```
///
/// ```compile_fail,E0599
/// let _ = ts_kernel::StableIdAllocator::default();
/// ```
///
/// ```compile_fail,E0599
/// let alloc = ts_kernel::StableIdAllocator::for_new_world();
/// let _ = alloc.clone();
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct StableIdAllocator {
    /// Prochain indice ; `MAP_BIT` signifie « épuisé ».
    next: u64,
}

impl StableIdAllocator {
    /// Allocateur d'un **monde neuf** : le premier identifiant est d'indice 0. Un monde chargé
    /// reprend son compteur par [`StableIdAllocator::from_snapshot_raw`].
    pub const fn for_new_world() -> Self {
        Self { next: 0 }
    }

    /// Nouvel identifiant, jamais rendu auparavant, ou `None` si l'espace est épuisé.
    pub const fn allocate(&mut self) -> Option<StableId> {
        if self.next <= StableId::MAX_INDEX {
            let id = StableId(self.next);
            self.next += 1;
            Some(id)
        } else {
            None
        }
    }

    /// Décodage d'un instantané : `None` si `next` dépasse `2^63`. **Réservé à `ts_persist`.**
    pub const fn from_snapshot_raw(next: u64) -> Option<Self> {
        if next <= MAP_BIT {
            Some(Self { next })
        } else {
            None
        }
    }

    /// Prochain indice, pour l'encodage d'un instantané. **Réservé à `ts_persist`.**
    pub const fn to_snapshot_raw(&self) -> u64 {
        self.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_counts_from_zero_in_game_space() {
        let mut alloc = StableIdAllocator::for_new_world();
        let a = alloc.allocate().unwrap();
        let b = alloc.allocate().unwrap();
        assert_eq!((a.index(), b.index()), (0, 1));
        assert!(!a.is_map() && !b.is_map());
        assert_ne!(a, b);
        assert_eq!(alloc.to_snapshot_raw(), 2);
    }

    #[test]
    fn allocator_stops_at_last_index() {
        let mut alloc = StableIdAllocator::from_snapshot_raw(StableId::MAX_INDEX).unwrap();
        let last = alloc.allocate().unwrap();
        assert_eq!(last.to_snapshot_raw(), (1 << 63) - 1);
        assert!(!last.is_map());
        assert_eq!(alloc.allocate(), None);
        assert_eq!(alloc.allocate(), None);
        assert_eq!(alloc.to_snapshot_raw(), 1 << 63);
    }

    #[test]
    fn allocator_snapshot_rejects_map_space() {
        assert!(StableIdAllocator::from_snapshot_raw(1 << 63).is_some());
        assert!(StableIdAllocator::from_snapshot_raw((1 << 63) + 1).is_none());
        assert!(StableIdAllocator::from_snapshot_raw(u64::MAX).is_none());
    }

    #[test]
    fn map_space_is_bit_63() {
        let first = StableId::map(0).unwrap();
        assert!(first.is_map());
        assert_eq!(first.to_snapshot_raw(), 1 << 63);
        let last = StableId::map(StableId::MAX_INDEX).unwrap();
        assert_eq!(last.to_snapshot_raw(), u64::MAX);
        assert_eq!(last.index(), StableId::MAX_INDEX);
        assert_eq!(StableId::map(1 << 63), None);
        assert_eq!(StableId::map(u64::MAX), None);
    }

    #[test]
    fn game_and_map_spaces_never_meet() {
        let mut alloc = StableIdAllocator::for_new_world();
        let game = alloc.allocate().unwrap();
        let map = StableId::map(game.index()).unwrap();
        assert_eq!(game.index(), map.index());
        assert_ne!(game, map);
    }

    #[test]
    fn snapshot_raw_round_trips() {
        for raw in [0, 1, (1 << 63) - 1, 1 << 63, u64::MAX] {
            assert_eq!(StableId::from_snapshot_raw(raw).to_snapshot_raw(), raw);
        }
    }

    #[test]
    fn debug_names_the_space() {
        assert_eq!(
            format!("{:?}", StableId::map(12).unwrap()),
            "StableId(map:12)"
        );
        assert_eq!(
            format!("{:?}", StableId::from_snapshot_raw(7)),
            "StableId(game:7)"
        );
    }
}
