//! Budgets de calcul en unités de travail.

// spec: revues/architecture.md §9.4, §11.3
/// Budget de calcul en **unités de travail** (nœuds explorés, recherches de chemin,
/// transitions de niveau…), jamais en temps : le résultat ne dépend pas de la machine.
///
/// Une dépense qui dépasse le reste est refusée en entier, et rien n'est dépensé.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkBudget {
    limit: u32,
    used: u32,
}

impl WorkBudget {
    /// Budget neuf de `limit` unités.
    pub const fn new(limit: u32) -> Self {
        Self { limit, used: 0 }
    }

    /// Dépense `units` unités si le reste suffit ; rend `false` sans rien dépenser sinon.
    pub const fn try_spend(&mut self, units: u32) -> bool {
        if units <= self.remaining() {
            self.used += units;
            true
        } else {
            false
        }
    }

    /// Unités restantes.
    pub const fn remaining(&self) -> u32 {
        self.limit - self.used
    }

    /// Unités dépensées.
    pub const fn used(&self) -> u32 {
        self.used
    }

    /// Limite du budget.
    pub const fn limit(&self) -> u32 {
        self.limit
    }

    /// Vrai si plus rien ne reste.
    pub const fn is_exhausted(&self) -> bool {
        self.used == self.limit
    }

    /// Remet le budget à plein, par exemple au tick suivant.
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
