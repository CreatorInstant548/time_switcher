//! The ten phases of a tick, one `bevy_ecs` `Schedule` per phase.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bevy_ecs::error::{BevyError, ErrorContext};
use bevy_ecs::schedule::{
    FixedBitSet, LogLevel, MultiThreadedExecutor, Schedule, ScheduleBuildSettings, ScheduleLabel,
    SingleThreadedExecutor, SystemExecutor, SystemSchedule,
};
use bevy_ecs::world::World;

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

// spec: revues/architecture.md §9.1, §9.2 ; carnet architecture-securite.md §9.10 L7, AS-57
/// `Schedule` of a phase: ordering ambiguities **as errors**, **no automatic sync point**;
/// a single-threaded executor for [`Threads::Single`], an [`AdaptiveExecutor`] for
/// [`Threads::Fixed`], whose switch is returned with the schedule.
///
/// No automatic sync point (AS-57 K-a, #93): `bevy_ecs` would otherwise insert an
/// `ApplyDeferred` after a system with `Commands` that is ordered before another one. Those
/// points are outside ambiguity detection and `Commands` declares no access, so a third
/// system, unordered with both, could have its commands applied at that point or at the end
/// of the phase depending on timing: a result that differs between two parallel runs. With
/// none, the commands of a phase are applied once, at the end of the phase, in the order of
/// the schedule's systems, which depends on the build only. Three things would break this:
/// - an `ApplyDeferred` system, which applies them in the middle of the phase (banned, K-b);
/// - an exclusive system with deferred buffers, which the parallel executor runs with
///   `System::run`, so they apply as soon as it ends (refused by `Sim::build`,
///   `BuildError::DeferredExclusive`);
/// - a run condition with deferred buffers, which the single-threaded executor applies when
///   it evaluates it and the parallel one never (refused, `BuildError::DeferredCondition`).
///
/// (The `Entity` that `Commands::spawn` reserves may still differ between parallel runs: it is not state, and
/// durable references use `StableId`.) Manual points (`ApplyDeferred`) are banned by
/// `clippy.toml` and by `cargo xtask check` (K-b).
pub(crate) fn phase_schedule(phase: Phase, threads: Threads) -> (Schedule, Option<ExecutorSwitch>) {
    let mut schedule = Schedule::new(phase);
    schedule.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Error,
        auto_insert_apply_deferred: false,
        ..ScheduleBuildSettings::default()
    });
    match threads {
        Threads::Single => {
            schedule.set_executor(SingleThreadedExecutor::new());
            (schedule, None)
        }
        Threads::Fixed(_) => {
            let switch = ExecutorSwitch::default();
            schedule.set_executor(AdaptiveExecutor {
                single: SingleThreadedExecutor::new(),
                parallel: MultiThreadedExecutor::new(),
                switch: switch.clone(),
            });
            (schedule, Some(switch))
        }
    }
}

// spec: revues/architecture.md §9.1, §9.2
/// Choice of the executor for the next run of one phase, set by [`crate::Sim`] before each
/// run. It lives outside the `World`: no system can read it, so no rule can depend on it.
#[derive(Debug, Clone, Default)]
pub(crate) struct ExecutorSwitch(Arc<AtomicBool>);

impl ExecutorSwitch {
    /// Selects the parallel executor (`true`) or the single-threaded one (`false`).
    pub(crate) fn set_parallel(&self, parallel: bool) {
        // Set and read on the thread that runs the schedule: no ordering is needed.
        self.0.store(parallel, Ordering::Relaxed);
    }

    fn parallel(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

// spec: revues/architecture.md §9.1, §9.2 ; carnet architecture-securite.md §9.10 L6, L7
/// Executor of a phase with [`Threads::Fixed`]: the parallel executor of `bevy_ecs` or its
/// single-threaded executor, both initialised once on the same `SystemSchedule`, picked
/// before each run by the [`ExecutorSwitch`].
///
/// **The choice changes the speed only** (R1). Every pair of systems whose accesses conflict
/// is explicitly ordered (ambiguities are errors, L7), and with no sync point inside a phase
/// (AS-57), no exclusive system and no run condition with deferred buffers (both refused by
/// `Sim::build`), both executors apply the deferred commands once, at the end of the phase,
/// in the topological order of the schedule; the toy world and the L6 tests already prove
/// that both give the same fingerprint. Neither keeps anything from one run to the next: with
/// `apply_final_deferred` left at its default (`true`; the kernel never changes it), each run
/// ends with every buffer applied and its run state cleared.
///
/// Why (issue #67): each run of the parallel executor sends every system to a pool thread and
/// waits for it: two thread wake-ups, measured at 50 to 60 µs per phase that has systems,
/// even for a system that does almost nothing. `par_iter` uses the pool under either executor.
struct AdaptiveExecutor {
    single: SingleThreadedExecutor,
    parallel: MultiThreadedExecutor,
    switch: ExecutorSwitch,
}

impl SystemExecutor for AdaptiveExecutor {
    fn init(&mut self, schedule: &SystemSchedule) {
        self.single.init(schedule);
        self.parallel.init(schedule);
    }

    fn run(
        &mut self,
        schedule: &mut SystemSchedule,
        world: &mut World,
        skip_systems: Option<&FixedBitSet>,
        error_handler: fn(BevyError, ErrorContext),
    ) {
        if self.switch.parallel() {
            self.parallel
                .run(schedule, world, skip_systems, error_handler);
        } else {
            self.single
                .run(schedule, world, skip_systems, error_handler);
        }
    }

    fn set_apply_final_deferred(&mut self, value: bool) {
        self.single.set_apply_final_deferred(value);
        self.parallel.set_apply_final_deferred(value);
    }
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
