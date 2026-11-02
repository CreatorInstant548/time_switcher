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
/// **Enforced (#31):** the generic resume test `ts_toy::resume::check_resume` runs on this
/// list (test below): a snapshot at several checkpoints after the first tick, through the
/// bytes of a save, then restored, must give the uninterrupted run at every later
/// checkpoint. Each rule crate also runs it on its own plugins, and `clippy.toml` bans
/// `Local`, `Added`, `Changed` and the other change-detection reads in rule crates. The draw sites of the list
/// are unique, or the build fails (#26 m1).
#[must_use]
pub fn standard_rules() -> Vec<Box<dyn Plugin>> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use ts_kernel::rng::WorldKey;
    use ts_kernel::{Sim, SimConfig, SimTime};
    use ts_toy::resume::check_resume;

    use super::*;

    #[test]
    fn sim_builds_from_standard_rules() {
        let rules = standard_rules();
        assert!(rules.is_empty());
        let mut sim =
            Sim::build(SimConfig::default(), &rules, &WorldKey::from_bytes([7; 16])).unwrap();
        assert!(sim.active_plugins().is_empty());
        assert_eq!(sim.now(), SimTime::EPOCH);
        sim.step().unwrap();
        assert_eq!(sim.now(), SimTime::from_millis(50));
    }

    /// AS-27, #31: every rule plugin resumes from a save at any checkpoint after the first
    /// tick. Test dates: the first ticks, then one simulated day.
    #[test]
    fn standard_rules_resume_after_a_restore() {
        let checkpoints =
            [50, 100, 1_000, 60_000, SimTime::MILLIS_PER_DAY].map(SimTime::from_millis);
        let resumes = check_resume(
            &SimConfig::default(),
            &standard_rules(),
            &WorldKey::from_bytes([7; 16]),
            &checkpoints,
        );
        assert_eq!(resumes.unwrap(), checkpoints.len() - 1);
    }
}
