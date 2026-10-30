// spec: revues/architecture.md §5.1, §5.2, §5.3
//! Rules assembly of time_switcher: [`standard_rules`] is the single list of rule plugins,
//! used by every host (game, server, Python, simulation runner, tests).

#![forbid(unsafe_code)]

use ts_kernel::Plugin;

// spec: revues/architecture.md §5.3, §9.2 ; carnet developpement.md D-DEV-02 ; carnet architecture-securite.md §10
/// The **only** list of rule plugins (D-DEV-02). Every host builds its `ts_kernel::Sim` from
/// it; the toy world and generators of the tests live in `ts_toy`, never here. Empty in
/// phase 0: the first plugins come in phase 1.
///
/// **Contract of each rule plugin (AS-27).** Between two ticks, a rule plugin keeps **no
/// state outside its declared state**, on top of the time-skip contract of
/// [`ts_kernel::Plugin`]: no `Local<T>` carried from one tick to the next, no game logic on
/// `Added` or `Changed`, every message written and read in the same tick, no undeclared game
/// resource or entity. A rebuildable cache is declared transient, and rebuilding it changes
/// no result.
///
/// **Blocking before the first rule plugin (phase 1), issue #31:** the generic resume test
/// (snapshot at tick k, restore, N ticks, same state hash as k + N ticks in a row, for
/// several k > 0) and the `clippy.toml` bans on `Local`, `Added` and `Changed` in rule crates.
#[must_use]
pub fn standard_rules() -> Vec<Box<dyn Plugin>> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use ts_kernel::rng::WorldKey;
    use ts_kernel::{Sim, SimConfig, SimTime};

    use super::*;

    #[test]
    fn sim_builds_from_standard_rules() {
        let rules = standard_rules();
        assert!(rules.is_empty());
        let mut sim =
            Sim::build(SimConfig::default(), &rules, &WorldKey::from_bytes([7; 16])).unwrap();
        assert!(sim.active_plugins().is_empty());
        assert_eq!(sim.now(), SimTime::EPOCH);
        sim.step();
        assert_eq!(sim.now(), SimTime::from_millis(50));
    }
}
