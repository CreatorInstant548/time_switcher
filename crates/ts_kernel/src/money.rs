//! Argent entier : `Money` en centimes, barèmes en points de base, arrondi bancaire unique.

// spec: revues/architecture.md §6.3 ; carnet architecture-securite.md §9.9 K7
/// Somme d'argent en **centimes d'écu**. Jamais de flottant.
///
/// Addition, soustraction et négation sont vérifiées (`checked_*`) : elles rendent `None` au
/// lieu d'enrouler. Il n'y a pas d'opérateur `+` ni `-`, pour que chaque dépassement soit
/// traité par l'appelant (décidé).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Money(i64);

crate::assert_size!(Money, 8);

impl Money {
    /// Zéro.
    pub const ZERO: Self = Self(0);

    /// Somme de `cents` centimes.
    pub const fn from_cents(cents: i64) -> Self {
        Self(cents)
    }

    /// Nombre de centimes.
    pub const fn cents(self) -> i64 {
        self.0
    }

    /// Somme, ou `None` en cas de dépassement.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Différence, ou `None` en cas de dépassement.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Opposé, ou `None` pour `i64::MIN` centimes.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Part de la somme selon un barème : `self × rate / 10 000`, arrondie au pair par
    /// [`div_round_half_even`] (calcul en `i128`), ou `None` si le résultat sort de `i64`.
    pub fn checked_apply(self, rate: BasisPoints) -> Option<Self> {
        let product = i128::from(self.0) * i128::from(rate.0);
        let rounded = div_round_half_even(product, i128::from(BasisPoints::DENOMINATOR))?;
        i64::try_from(rounded).ok().map(Self)
    }
}

// spec: revues/architecture.md §6.3 ; carnet architecture-securite.md §9.9 K7
/// Barème en **points de base** : 10 000 points valent 100 %. Négatif permis (remise, perte).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BasisPoints(i64);

impl BasisPoints {
    /// Points de base pour 100 %.
    pub const DENOMINATOR: i64 = 10_000;

    /// Barème de `points` points de base.
    pub const fn new(points: i64) -> Self {
        Self(points)
    }

    /// Nombre de points de base.
    pub const fn points(self) -> i64 {
        self.0
    }
}

// spec: revues/architecture.md §6.3 ; carnet architecture-securite.md §9.9 K7
/// **Règle d'arrondi unique du cœur** : `numerator / denominator` arrondi au plus proche, les
/// cas à mi-chemin allant à l'entier **pair** (arrondi bancaire). La règle est symétrique :
/// l'arrondi de `−x` est l'opposé de celui de `x`.
///
/// Rend `None` si `denominator <= 0`. Ne déborde jamais : il n'y a de pas d'arrondi que si le
/// reste est non nul, donc si `denominator >= 2`, et le quotient tronqué vaut alors au plus
/// `|numerator| / 2` en valeur absolue.
pub fn div_round_half_even(numerator: i128, denominator: i128) -> Option<i128> {
    if denominator <= 0 {
        return None;
    }
    // Division tronquée vers zéro : le reste a le signe du numérateur.
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    let twice = remainder.unsigned_abs() * 2;
    let divisor = denominator.unsigned_abs();
    let away_from_zero = twice > divisor || (twice == divisor && quotient % 2 != 0);
    if away_from_zero {
        Some(quotient + numerator.signum())
    } else {
        Some(quotient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bps(points: i64) -> BasisPoints {
        BasisPoints::new(points)
    }

    #[test]
    fn half_cases_go_to_even() {
        assert_eq!(div_round_half_even(5, 2), Some(2));
        assert_eq!(div_round_half_even(7, 2), Some(4));
        assert_eq!(div_round_half_even(1, 2), Some(0));
        assert_eq!(div_round_half_even(3, 2), Some(2));
        assert_eq!(div_round_half_even(-5, 2), Some(-2));
        assert_eq!(div_round_half_even(-7, 2), Some(-4));
        assert_eq!(div_round_half_even(-1, 2), Some(0));
    }

    #[test]
    fn non_half_cases_go_to_nearest() {
        assert_eq!(div_round_half_even(6, 4), Some(2)); // 1,5
        assert_eq!(div_round_half_even(5, 4), Some(1)); // 1,25
        assert_eq!(div_round_half_even(7, 4), Some(2)); // 1,75
        assert_eq!(div_round_half_even(-5, 4), Some(-1));
        assert_eq!(div_round_half_even(-7, 4), Some(-2));
        assert_eq!(div_round_half_even(9, 3), Some(3));
        assert_eq!(div_round_half_even(0, 7), Some(0));
    }

    #[test]
    fn rounding_is_symmetric() {
        for n in -50..=50 {
            for d in 1..=12 {
                assert_eq!(
                    div_round_half_even(-n, d),
                    div_round_half_even(n, d).map(|q| -q),
                    "{n}/{d}"
                );
            }
        }
    }

    #[test]
    fn rounding_rejects_bad_denominator_and_handles_bounds() {
        assert_eq!(div_round_half_even(1, 0), None);
        assert_eq!(div_round_half_even(1, -2), None);
        assert_eq!(div_round_half_even(i128::MAX, 1), Some(i128::MAX));
        assert_eq!(div_round_half_even(i128::MIN, 1), Some(i128::MIN));
        assert_eq!(div_round_half_even(i128::MAX, 2), Some(i128::MAX / 2 + 1));
        assert_eq!(div_round_half_even(i128::MIN, 2), Some(i128::MIN / 2));
        assert_eq!(div_round_half_even(i128::MIN, i128::MAX), Some(-1));
    }

    #[test]
    fn apply_rate_in_basis_points() {
        let m = Money::from_cents;
        assert_eq!(m(10_000).checked_apply(bps(1_950)), Some(m(1_950)));
        // 0,5 centime : au pair.
        assert_eq!(m(1).checked_apply(bps(5_000)), Some(m(0)));
        assert_eq!(m(3).checked_apply(bps(5_000)), Some(m(2)));
        assert_eq!(m(-1).checked_apply(bps(5_000)), Some(m(0)));
        assert_eq!(m(-3).checked_apply(bps(5_000)), Some(m(-2)));
        assert_eq!(m(5).checked_apply(bps(-5_000)), Some(m(-2)));
        // 2,5 % de 1,00 écu = 2,5 centimes.
        assert_eq!(m(100).checked_apply(bps(250)), Some(m(2)));
        assert_eq!(m(100).checked_apply(bps(350)), Some(m(4)));
        assert_eq!(m(123).checked_apply(bps(0)), Some(Money::ZERO));
        assert_eq!(m(i64::MAX).checked_apply(bps(10_000)), Some(m(i64::MAX)));
        assert_eq!(m(i64::MIN).checked_apply(bps(10_000)), Some(m(i64::MIN)));
        assert_eq!(m(i64::MAX).checked_apply(bps(10_001)), None);
        assert_eq!(m(i64::MIN).checked_apply(bps(-10_000)), None);
        assert_eq!(m(i64::MAX).checked_apply(bps(i64::MAX)), None);
        assert_eq!(m(i64::MIN).checked_apply(bps(i64::MIN)), None);
    }

    #[test]
    fn checked_arithmetic_never_wraps() {
        let m = Money::from_cents;
        assert_eq!(m(150).checked_add(m(-50)), Some(m(100)));
        assert_eq!(m(150).checked_sub(m(200)), Some(m(-50)));
        assert_eq!(m(i64::MAX).checked_add(m(1)), None);
        assert_eq!(m(i64::MIN).checked_sub(m(1)), None);
        assert_eq!(m(i64::MIN).checked_neg(), None);
        assert_eq!(m(-7).checked_neg(), Some(m(7)));
    }
}
