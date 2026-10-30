//! Temps de jeu entier : `SimTime`, `SimDay` et `DayOrigin`. Aucune lecture d'horloge.

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

    /// Jour absolu de l'instant depuis l'époque, arrondi vers −∞ (division euclidienne : −1 ms
    /// est le jour −1).
    pub const fn absolute_day(self) -> i64 {
        self.0.div_euclid(Self::MILLIS_PER_DAY)
    }

    // spec: README.md ; attendus-mesurables.md M9
    // Arbitrage A12 (tableau du README) et AS-22 (carnet architecture-securite.md), qui
    // remplace K8 pour SimDay ; spec-map ne lit pas ces identifiants, placés en 2e colonne.
    /// Jour compact de l'instant, relatif à `origin` : `jour_absolu − origine`, **saturé** dans
    /// `i16::MIN..=i16::MAX`, sans `Option`. Une date plus ancienne vaut [`SimDay::MIN`], qui se
    /// lit « il y a 89 ans ou plus » ; une date plus tardive vaut [`SimDay::MAX`], par sécurité,
    /// puisque l'origine glisse avant (AS-22).
    pub const fn day(self, origin: DayOrigin) -> SimDay {
        let relative = self.absolute_day().saturating_sub(origin.0);
        if relative < i16::MIN as i64 {
            SimDay::MIN
        } else if relative > i16::MAX as i64 {
            SimDay::MAX
        } else {
            // Conversion exacte : `relative` est dans `i16::MIN..=i16::MAX`.
            SimDay(relative as i16)
        }
    }
}

// spec: README.md ; attendus-mesurables.md M9
// Arbitrage A12 (tableau du README) et AS-22 (carnet architecture-securite.md).
/// Origine des jours compacts, en jours absolus depuis l'époque. Elle vaut
/// [`DayOrigin::EPOCH`] à la création du monde et n'avance que par pas de
/// [`SimDay::REBASE_STEP`] (AS-22). Son enregistrement dans l'instantané vient avec T8.
///
/// Pas de `Default` : une remise à zéro implicite décalerait toutes les dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DayOrigin(i64);

impl DayOrigin {
    /// Origine d'un monde neuf : l'époque.
    pub const EPOCH: Self = Self(0);

    /// Origine à `days` jours absolus de l'époque.
    pub const fn from_days(days: i64) -> Self {
        Self(days)
    }

    /// Jours absolus depuis l'époque.
    pub const fn days(self) -> i64 {
        self.0
    }
}

// spec: README.md ; attendus-mesurables.md M9
// Arbitrage A12 (tableau du README) et AS-22 (carnet architecture-securite.md).
/// Jour compact, **signé** sur 16 bits (M9, A12), relatif à une [`DayOrigin`] : environ
/// ±89 ans. Les règles ne lisent que l'écart entre deux jours ([`SimDay::days_since`]), jamais
/// la valeur brute, qui change quand l'origine glisse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimDay(i16);

crate::assert_size!(SimDay, 2);

impl SimDay {
    /// Plus ancien jour représentable : « il y a 89 ans ou plus ».
    pub const MIN: Self = Self(i16::MIN);
    /// Plus tardif jour représentable.
    pub const MAX: Self = Self(i16::MAX);
    /// Seuil de glissement (AS-22, constante technique) : quand le jour courant l'atteint,
    /// l'origine avance de [`SimDay::REBASE_STEP`]. 60 × 365 jours.
    pub const REBASE_THRESHOLD: i16 = 21_900;
    /// Pas de glissement (AS-22, constante technique) : 30 × 365 jours.
    pub const REBASE_STEP: i16 = 10_950;

    /// Jour d'indice `index` relatif à l'origine (négatif avant elle).
    pub const fn from_index(index: i16) -> Self {
        Self(index)
    }

    /// Indice du jour relatif à l'origine.
    pub const fn index(self) -> i16 {
        self.0
    }

    /// Écart en jours avec `earlier` (négatif si `earlier` est plus tard).
    pub const fn days_since(self, earlier: Self) -> i32 {
        self.0 as i32 - earlier.0 as i32
    }

    /// Même jour après un glissement de l'origine de `step` jours : `self − step`, saturé
    /// (AS-22). Fonction pure ; une date déjà au minimum y reste.
    pub const fn rebase(self, step: i16) -> Self {
        Self(self.0.saturating_sub(step))
    }

    /// Premier instant du jour, pour l'origine `origin`, ou `None` en cas de dépassement.
    pub const fn start(self, origin: DayOrigin) -> Option<SimTime> {
        match origin.0.checked_add(self.0 as i64) {
            Some(days) => match days.checked_mul(SimTime::MILLIS_PER_DAY) {
                Some(ms) => Some(SimTime(ms)),
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = SimTime::MILLIS_PER_DAY;
    const O: DayOrigin = DayOrigin::EPOCH;

    fn day(ms: i64) -> SimDay {
        SimTime::from_millis(ms).day(O)
    }

    #[test]
    fn day_of_time_floors_toward_minus_infinity() {
        assert_eq!(day(0).index(), 0);
        assert_eq!(day(DAY - 1).index(), 0);
        assert_eq!(day(DAY).index(), 1);
        assert_eq!(day(-1).index(), -1);
        assert_eq!(day(-DAY).index(), -1);
        assert_eq!(day(-DAY - 1).index(), -2);
    }

    #[test]
    fn day_of_time_saturates_at_bounds() {
        let min = i64::from(i16::MIN);
        let max = i64::from(i16::MAX);
        // Borne basse : i16::MIN jours, juste avant et juste après.
        assert_eq!(day(min * DAY), SimDay::MIN);
        assert_eq!(day(min * DAY - 1), SimDay::MIN);
        assert_eq!(day((min - 1) * DAY), SimDay::MIN);
        assert_eq!(day((min + 1) * DAY), SimDay::from_index(i16::MIN + 1));
        assert_eq!(day((min + 1) * DAY - 1), SimDay::MIN);
        // Borne haute : i16::MAX jours, juste avant et juste après.
        assert_eq!(day(max * DAY), SimDay::MAX);
        assert_eq!(day(max * DAY + DAY - 1), SimDay::MAX);
        assert_eq!(day(max * DAY - 1), SimDay::from_index(i16::MAX - 1));
        assert_eq!(day((max + 1) * DAY), SimDay::MAX);
        // Extrêmes de SimTime et d'origine.
        assert_eq!(day(i64::MIN), SimDay::MIN);
        assert_eq!(day(i64::MAX), SimDay::MAX);
        let t = SimTime::EPOCH;
        assert_eq!(t.day(DayOrigin::from_days(i64::MIN)), SimDay::MAX);
        assert_eq!(t.day(DayOrigin::from_days(i64::MAX)), SimDay::MIN);
    }

    #[test]
    fn day_is_relative_to_origin() {
        let origin = DayOrigin::from_days(10_950);
        let t = SimTime::from_millis(11_000 * DAY + 5);
        assert_eq!(t.day(origin).index(), 50);
        assert_eq!(SimTime::EPOCH.day(origin).index(), -10_950);
        assert_eq!(
            SimDay::from_index(50).start(origin),
            Some(SimTime::from_millis(11_000 * DAY))
        );
    }

    #[test]
    fn day_start_round_trips() {
        for origin in [O, DayOrigin::from_days(-3), DayOrigin::from_days(21_900)] {
            for index in [i16::MIN, -1, 0, 1, 365, i16::MAX] {
                let d = SimDay::from_index(index);
                let start = d.start(origin).unwrap();
                assert_eq!(start.day(origin), d);
                assert_eq!(start.add_millis(DAY - 1).day(origin), d);
            }
        }
        assert_eq!(SimDay::MAX.start(DayOrigin::from_days(i64::MAX)), None);
        assert_eq!(
            SimDay::MIN.start(DayOrigin::from_days(i64::MIN / DAY)),
            None
        );
    }

    #[test]
    fn rebase_saturates_and_keeps_gaps() {
        let step = SimDay::REBASE_STEP;
        assert_eq!(SimDay::from_index(21_900).rebase(step).index(), 10_950);
        assert_eq!(SimDay::MIN.rebase(step), SimDay::MIN);
        assert_eq!(SimDay::from_index(i16::MIN + 5).rebase(step), SimDay::MIN);
        let (a, b) = (SimDay::from_index(20_000), SimDay::from_index(-3_000));
        assert_eq!(a.rebase(step).days_since(b.rebase(step)), a.days_since(b));
        assert_eq!(SimDay::MAX.days_since(SimDay::MIN), 65_535);
    }

    /// Glissement simulé jour par jour sur 200 ans (AS-22) : le jour courant reste sous
    /// `i16::MAX`, et chaque date gardée a exactement la valeur et l'écart qu'aurait sa
    /// conversion avec l'origine courante, saturée au minimum au-delà de 89 ans environ.
    #[test]
    fn rebase_over_200_years_keeps_current_day_in_range() {
        let mut origin = DayOrigin::EPOCH;
        // (jour absolu de la date, date compacte gardée et réécrite à chaque glissement)
        let mut stored: Vec<(i64, SimDay)> = Vec::new();
        let mut shifts = 0;
        for abs in 0..=200 * 365_i64 {
            let now = SimTime::from_millis(abs * DAY);
            let mut current = now.day(origin);
            if current.index() >= SimDay::REBASE_THRESHOLD {
                origin = DayOrigin::from_days(origin.days() + i64::from(SimDay::REBASE_STEP));
                for (_, d) in &mut stored {
                    *d = d.rebase(SimDay::REBASE_STEP);
                }
                let rebased = current.rebase(SimDay::REBASE_STEP);
                current = now.day(origin);
                assert_eq!(current, rebased);
                shifts += 1;
            }
            assert!(current.index() < SimDay::REBASE_THRESHOLD);
            assert!(current < SimDay::MAX);
            if abs % 500 == 0 {
                stored.push((abs, current));
            }
            for &(at, d) in &stored {
                let expected = SimTime::from_millis(at * DAY).day(origin);
                assert_eq!(d, expected, "date du jour {at}, au jour {abs}");
                if expected != SimDay::MIN {
                    assert_eq!(i64::from(current.days_since(d)), abs - at);
                }
            }
        }
        assert!(shifts >= 5, "{shifts} glissements");
        // La plus ancienne date (jour 0, il y a 200 ans) est saturée au minimum.
        assert_eq!(stored[0].1, SimDay::MIN);
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
        assert_eq!(birth.day(O).index(), -10_950);
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
