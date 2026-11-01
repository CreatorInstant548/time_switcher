//! The ten phases of a tick, one `bevy_ecs` `Schedule` per phase.

use bevy_ecs::schedule::{
    LogLevel, Schedule, ScheduleBuildSettings, ScheduleLabel, SingleThreadedExecutor,
};

use crate::Threads;

// spec: revues/architecture.md §9.1
/// Phase of a tick. Phases run in the order of [`Phase::ALL`]; within a phase, the `bevy_ecs`
/// executor spreads the systems that do not conflict in access.
#[derive(ScheduleLabel, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    /// Intentions and commands received for this tick; validation.
    Input,
    /// Dated events of the tick interval ([`crate::DueEvents`]).
    Scheduled,
    /// Spatial grid and perceived sets.
    Perception,
    /// Reflexes, goal selection, planning.
    Mind,
    /// Execution skills.
    Skills,
    /// Execution of intentions.
    Act,
    /// Injuries, needs, emotions, memory, witnesses.
    Consequences,
    /// Scenery, weather, transport.
    World,
    /// Bubbles and level-of-detail changes.
    Lod,
    /// Finalized events, change hints, measures.
    Output,
}

impl Phase {
    /// Number of phases.
    pub const COUNT: usize = 10;

    /// The phases, in their order of execution.
    pub const ALL: [Self; Self::COUNT] = [
        Self::Input,
        Self::Scheduled,
        Self::Perception,
        Self::Mind,
        Self::Skills,
        Self::Act,
        Self::Consequences,
        Self::World,
        Self::Lod,
        Self::Output,
    ];

    /// Rank of the phase in [`Phase::ALL`].
    pub const fn index(self) -> usize {
        self as usize
    }
}

// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L7
/// `Schedule` of a phase: ordering ambiguities **as errors**, and a single-threaded executor
/// for [`Threads::Single`].
pub(crate) fn phase_schedule(phase: Phase, threads: Threads) -> Schedule {
    let mut schedule = Schedule::new(phase);
    schedule.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Error,
        ..ScheduleBuildSettings::default()
    });
    if threads == Threads::Single {
        schedule.set_executor(SingleThreadedExecutor::new());
    }
    schedule
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phases_are_in_declared_order() {
        for (i, p) in Phase::ALL.iter().enumerate() {
            assert_eq!(p.index(), i);
        }
        assert!(Phase::ALL.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(Phase::ALL[0], Phase::Input);
        assert_eq!(Phase::ALL[Phase::COUNT - 1], Phase::Output);
    }
}
