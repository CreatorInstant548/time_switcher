//! Integer game time: `SimTime`, `SimDay` and `DayOrigin`. No clock is ever read.

// spec: revues/architecture.md §6.2 ; carnet architecture-securite.md §9.9 K8
/// An instant of the world, in milliseconds since the **world epoch** (its starting day).
///
/// It can be **negative**: generated births precede the epoch (K8).
///
/// **Overflow rule (decided): it panics**, it never saturates. An `i64` in milliseconds
/// covers about ±292 million years; an overflow can therefore only come from a defect, and a
/// saturated time would silently freeze the clock, with events piling up at the same instant.
/// [`SimTime::add_millis`] panics, whatever the profile; [`SimTime::checked_add_millis`]
/// returns an `Option` for a duration that comes from data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimTime(i64);

crate::assert_size!(SimTime, 8);

impl SimTime {
    /// The world epoch.
    pub const EPOCH: Self = Self(0);
    /// Milliseconds per simulation day.
    pub const MILLIS_PER_DAY: i64 = 86_400_000;

    /// The instant `millis` milliseconds from the epoch.
    pub const fn from_millis(millis: i64) -> Self {
        Self(millis)
    }

    /// Milliseconds since the epoch.
    pub const fn as_millis(self) -> i64 {
        self.0
    }

    /// The instant shifted by `millis`, or `None` on overflow.
    pub const fn checked_add_millis(self, millis: i64) -> Option<Self> {
        match self.0.checked_add(millis) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// The instant shifted by `millis`.
    ///
    /// # Panics
    ///
    /// On `i64` overflow, whatever the build profile.
    pub const fn add_millis(self, millis: i64) -> Self {
        match self.checked_add_millis(millis) {
            Some(t) => t,
            None => panic!("SimTime overflow"),
        }
    }

    /// Time elapsed since `earlier`, in milliseconds (negative if `earlier` is later),
    /// or `None` on overflow.
    pub const fn checked_millis_since(self, earlier: Self) -> Option<i64> {
        self.0.checked_sub(earlier.0)
    }

    /// Absolute day of the instant since the epoch, rounded toward −∞ (Euclidean division:
    /// −1 ms is day −1).
    pub const fn absolute_day(self) -> i64 {
        self.0.div_euclid(Self::MILLIS_PER_DAY)
    }

    // spec: README.md ; attendus-mesurables.md M9
    // Arbitrage A12 (README table) and AS-22 (carnet architecture-securite.md), which
    // replaces K8 for SimDay; spec-map does not read these identifiers, placed in the 2nd column.
    /// Compact day of the instant, relative to `origin`: `absolute_day − origin`, **saturated**
    /// in `i16::MIN..=i16::MAX`, with no `Option`. An older date is [`SimDay::MIN`], which
    /// reads "89 years ago or more"; a later date is [`SimDay::MAX`], as a safety measure,
    /// since the origin slides forward (AS-22).
    pub const fn day(self, origin: DayOrigin) -> SimDay {
        let relative = self.absolute_day().saturating_sub(origin.0);
        if relative < i16::MIN as i64 {
            SimDay::MIN
        } else if relative > i16::MAX as i64 {
            SimDay::MAX
        } else {
            // Exact conversion: `relative` is in `i16::MIN..=i16::MAX`.
            SimDay(relative as i16)
        }
    }
}

// spec: README.md ; attendus-mesurables.md M9
// Arbitrage A12 (README table) and AS-22 (carnet architecture-securite.md).
/// Origin of the compact days, in absolute days since the epoch. It is
/// [`DayOrigin::EPOCH`] when the world is created and only advances in steps of
/// [`SimDay::REBASE_STEP`] (AS-22). Its recording in the snapshot comes with T8.
///
/// No `Default`: an implicit reset would shift every date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DayOrigin(i64);

impl DayOrigin {
    /// Origin of a new world: the epoch.
    pub const EPOCH: Self = Self(0);

    /// Origin `days` absolute days from the epoch.
    pub const fn from_days(days: i64) -> Self {
        Self(days)
    }

    /// Absolute days since the epoch.
    pub const fn days(self) -> i64 {
        self.0
    }
}

// spec: README.md ; attendus-mesurables.md M9
// Arbitrage A12 (README table) and AS-22 (carnet architecture-securite.md).
/// Compact day, **signed** on 16 bits (M9, A12), relative to a [`DayOrigin`]: about ±89
/// years. Rules only read the difference between two days ([`SimDay::days_since`]), never the
/// raw value, which changes when the origin slides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimDay(i16);

crate::assert_size!(SimDay, 2);

impl SimDay {
    /// Oldest representable day: "89 years ago or more".
    pub const MIN: Self = Self(i16::MIN);
    /// Latest representable day.
    pub const MAX: Self = Self(i16::MAX);
    /// Sliding threshold (AS-22, technical constant): when the current day reaches it, the
    /// origin advances by [`SimDay::REBASE_STEP`]. 60 × 365 days.
    pub const REBASE_THRESHOLD: i16 = 21_900;
    /// Sliding step (AS-22, technical constant): 30 × 365 days.
    pub const REBASE_STEP: i16 = 10_950;

    /// Day with index `index` relative to the origin (negative before it).
    pub const fn from_index(index: i16) -> Self {
        Self(index)
    }

    /// Index of the day relative to the origin.
    pub const fn index(self) -> i16 {
        self.0
    }

    /// Difference in days from `earlier` (negative if `earlier` is later).
    pub const fn days_since(self, earlier: Self) -> i32 {
        self.0 as i32 - earlier.0 as i32
    }

    /// The same day after the origin slides by `step` days: `self − step`, saturated
    /// (AS-22). Pure function; a date already at the minimum stays there.
    pub const fn rebase(self, step: i16) -> Self {
        Self(self.0.saturating_sub(step))
    }

    /// First instant of the day, for origin `origin`, or `None` on overflow.
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
        // Lower bound: i16::MIN days, just before and just after.
        assert_eq!(day(min * DAY), SimDay::MIN);
        assert_eq!(day(min * DAY - 1), SimDay::MIN);
        assert_eq!(day((min - 1) * DAY), SimDay::MIN);
        assert_eq!(day((min + 1) * DAY), SimDay::from_index(i16::MIN + 1));
        assert_eq!(day((min + 1) * DAY - 1), SimDay::MIN);
        // Upper bound: i16::MAX days, just before and just after.
        assert_eq!(day(max * DAY), SimDay::MAX);
        assert_eq!(day(max * DAY + DAY - 1), SimDay::MAX);
        assert_eq!(day(max * DAY - 1), SimDay::from_index(i16::MAX - 1));
        assert_eq!(day((max + 1) * DAY), SimDay::MAX);
        // Extremes of SimTime and of the origin.
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

    /// Slide simulated day by day over 200 years (AS-22): the current day stays below
    /// `i16::MAX`, and each kept date has exactly the value and the difference that its
    /// conversion with the current origin would have, saturated at the minimum beyond about
    /// 89 years.
    #[test]
    fn rebase_over_200_years_keeps_current_day_in_range() {
        let mut origin = DayOrigin::EPOCH;
        // (absolute day of the date, compact date kept and rewritten at each slide)
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
                assert_eq!(d, expected, "date of day {at}, on day {abs}");
                if expected != SimDay::MIN {
                    assert_eq!(i64::from(current.days_since(d)), abs - at);
                }
            }
        }
        assert!(shifts >= 5, "{shifts} slides");
        // The oldest date (day 0, 200 years ago) is saturated at the minimum.
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
    #[should_panic(expected = "SimTime overflow")]
    fn add_millis_panics_on_overflow() {
        let _ = SimTime::from_millis(i64::MAX).add_millis(1);
    }
}
