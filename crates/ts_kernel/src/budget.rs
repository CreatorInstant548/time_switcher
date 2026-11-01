//! Compute budgets in work units.

// spec: revues/architecture.md §9.4, §11.3
/// Compute budget in **work units** (nodes explored, path searches, level transitions...),
/// never in time: the result does not depend on the machine.
///
/// A spend that exceeds the remainder is refused in full, and nothing is spent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkBudget {
    limit: u32,
    used: u32,
}

impl WorkBudget {
    /// A fresh budget of `limit` units.
    pub const fn new(limit: u32) -> Self {
        Self { limit, used: 0 }
    }

    /// Spends `units` units if the remainder suffices; returns `false` and spends nothing
    /// otherwise.
    pub const fn try_spend(&mut self, units: u32) -> bool {
        if units <= self.remaining() {
            self.used += units;
            true
        } else {
            false
        }
    }

    /// Units remaining.
    pub const fn remaining(&self) -> u32 {
        self.limit - self.used
    }

    /// Units spent.
    pub const fn used(&self) -> u32 {
        self.used
    }

    /// The budget limit.
    pub const fn limit(&self) -> u32 {
        self.limit
    }

    /// True if nothing remains.
    pub const fn is_exhausted(&self) -> bool {
        self.used == self.limit
    }

    /// Resets the budget to full, for example at the next tick.
    pub const fn refill(&mut self) {
        self.used = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spending_is_all_or_nothing() {
        let mut b = WorkBudget::new(10);
        assert!(b.try_spend(4));
        assert!(b.try_spend(6));
        assert!(b.is_exhausted());
        assert!(!b.try_spend(1));
        assert!(b.try_spend(0));
        assert_eq!((b.used(), b.remaining(), b.limit()), (10, 0, 10));
        b.refill();
        assert!(!b.try_spend(11));
        assert_eq!(b.used(), 0);
        assert!(b.try_spend(u32::MIN));
        let mut max = WorkBudget::new(u32::MAX);
        assert!(max.try_spend(u32::MAX));
        assert!(!max.try_spend(1));
    }
}
