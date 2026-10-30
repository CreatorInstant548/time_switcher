//! Temps de jeu entier : `SimTime` et `SimDay`. Aucune lecture d'horloge.

// spec: revues/architecture.md §6.2 ; carnet architecture-securite.md §9.9 K8
/// Instant du monde, en millisecondes depuis l'**époque du monde** (son jour de départ).
///
/// Il peut être **négatif** : les naissances générées précèdent l'époque (K8).
///
/// **Règle de dépassement (décidé) : il panique**, il ne sature jamais. Un `i64` en
/// millisecondes couvre environ ±292 millions d'années ; un dépassement ne peut donc venir que
/// d'un défaut, et un temps saturé figerait l'horloge en silence, avec des événements empilés
/// au même instant. [`SimTime::add_millis`] panique, quel que soit le profil ;
/// [`SimTime::checked_add_millis`] rend une `Option` pour une durée venue des données.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimTime(i64);

crate::assert_size!(SimTime, 8);

impl SimTime {
    /// L'époque du monde.
    pub const EPOCH: Self = Self(0);
    /// Millisecondes par jour de simulation.
    pub const MILLIS_PER_DAY: i64 = 86_400_000;

    /// Instant à `millis` millisecondes de l'époque.
    pub const fn from_millis(millis: i64) -> Self {
        Self(millis)
    }

    /// Millisecondes depuis l'époque.
    pub const fn as_millis(self) -> i64 {
        self.0
    }

    /// Instant décalé de `millis`, ou `None` en cas de dépassement.
    pub const fn checked_add_millis(self, millis: i64) -> Option<Self> {
        match self.0.checked_add(millis) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Instant décalé de `millis`.
    ///
    /// # Panics
    ///
    /// En cas de dépassement de `i64`, quel que soit le profil de construction.
    pub const fn add_millis(self, millis: i64) -> Self {
        match self.checked_add_millis(millis) {
            Some(t) => t,
            None => panic!("dépassement de SimTime"),
        }
    }

    /// Durée écoulée depuis `earlier`, en millisecondes (négative si `earlier` est plus tard),
    /// ou `None` en cas de dépassement.
    pub const fn checked_millis_since(self, earlier: Self) -> Option<i64> {
        self.0.checked_sub(earlier.0)
    }

    /// Jour de l'instant, ou `None` hors de `0..=u16::MAX` jours : avant l'époque, ou plus de
    /// 179 ans après. Jamais tronqué (K8).
    pub const fn day(self) -> Option<SimDay> {
        let day = self.0.div_euclid(Self::MILLIS_PER_DAY);
        if day >= 0 && day <= u16::MAX as i64 {
            // Conversion exacte : `day` est dans `0..=u16::MAX`.
            Some(SimDay(day as u16))
        } else {
            None
        }
    }
}

// spec: revues/architecture.md §6.2 ; attendus-mesurables.md M9 ; carnet architecture-securite.md §9.9 K8
/// Jour depuis l'époque du monde, sur 16 bits (M9). Le jour 0 commence à l'époque.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimDay(u16);

crate::assert_size!(SimDay, 2);

impl SimDay {
    /// Jour d'indice `index` depuis l'époque.
    pub const fn from_index(index: u16) -> Self {
        Self(index)
    }

    /// Indice du jour depuis l'époque.
    pub const fn index(self) -> u16 {
        self.0
    }

    /// Premier instant du jour.
    pub const fn start(self) -> SimTime {
        SimTime(self.0 as i64 * SimTime::MILLIS_PER_DAY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = SimTime::MILLIS_PER_DAY;

    #[test]
    fn day_of_time_floors_and_rejects_out_of_range() {
        assert_eq!(SimTime::EPOCH.day(), Some(SimDay::from_index(0)));
        assert_eq!(
            SimTime::from_millis(DAY - 1).day(),
            Some(SimDay::from_index(0))
        );
        assert_eq!(SimTime::from_millis(DAY).day(), Some(SimDay::from_index(1)));
        assert_eq!(SimTime::from_millis(-1).day(), None);
        assert_eq!(SimTime::from_millis(-DAY).day(), None);
        let last = i64::from(u16::MAX);
        assert_eq!(
            SimTime::from_millis(last * DAY + DAY - 1).day(),
            Some(SimDay::from_index(u16::MAX))
        );
        assert_eq!(SimTime::from_millis((last + 1) * DAY).day(), None);
        assert_eq!(SimTime::from_millis(i64::MAX).day(), None);
        assert_eq!(SimTime::from_millis(i64::MIN).day(), None);
    }

    #[test]
    fn day_start_round_trips() {
        for index in [0, 1, 365, u16::MAX] {
            let day = SimDay::from_index(index);
            assert_eq!(day.start().day(), Some(day));
            assert_eq!(day.start().add_millis(DAY - 1).day(), Some(day));
        }
    }

    #[test]
    fn negative_times_are_ordered() {
        let birth = SimTime::from_millis(-30 * 365 * DAY);
        assert!(birth < SimTime::EPOCH);
        assert_eq!(
            SimTime::EPOCH.checked_millis_since(birth),
            Some(30 * 365 * DAY)
        );
        assert_eq!(
            birth.checked_millis_since(SimTime::EPOCH),
            Some(-30 * 365 * DAY)
        );
    }

    #[test]
    fn checked_arithmetic_detects_overflow() {
        let max = SimTime::from_millis(i64::MAX);
        assert_eq!(max.checked_add_millis(1), None);
        assert_eq!(SimTime::from_millis(i64::MIN).checked_add_millis(-1), None);
        assert_eq!(max.checked_millis_since(SimTime::from_millis(-1)), None);
        assert_eq!(
            SimTime::EPOCH.checked_add_millis(-5),
            Some(SimTime::from_millis(-5))
        );
    }

    #[test]
    #[should_panic(expected = "dépassement de SimTime")]
    fn add_millis_panics_on_overflow() {
        let _ = SimTime::from_millis(i64::MAX).add_millis(1);
    }
}
