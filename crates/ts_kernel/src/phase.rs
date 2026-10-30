//! Les dix phases d'un tick, un `Schedule` `bevy_ecs` par phase.

use bevy_ecs::schedule::{
    LogLevel, Schedule, ScheduleBuildSettings, ScheduleLabel, SingleThreadedExecutor,
};

use crate::Threads;

// spec: revues/architecture.md §9.1
/// Phase d'un tick. Les phases s'exécutent dans l'ordre de [`Phase::ALL`] ; à l'intérieur
/// d'une phase, l'exécuteur de `bevy_ecs` répartit les systèmes sans conflit d'accès.
#[derive(ScheduleLabel, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    /// Intentions et commandes reçues pour ce tick ; validation.
    Input,
    /// Événements datés de l'intervalle du tick ([`crate::DueEvents`]).
    Scheduled,
    /// Grille spatiale et ensembles perçus.
    Perception,
    /// Réflexes, sélection d'objectif, planification.
    Mind,
    /// Compétences d'exécution.
    Skills,
    /// Exécution des intentions.
    Act,
    /// Blessures, besoins, émotions, mémoire, témoins.
    Consequences,
    /// Décor, météo, transports.
    World,
    /// Bulles et changements de niveau de détail.
    Lod,
    /// Événements finalisés, indices de changements, mesures.
    Output,
}

impl Phase {
    /// Nombre de phases.
    pub const COUNT: usize = 10;

    /// Les phases, dans leur ordre d'exécution.
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

    /// Rang de la phase dans [`Phase::ALL`].
    pub const fn index(self) -> usize {
        self as usize
    }
}

// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L7
/// `Schedule` d'une phase : ambiguïtés d'ordre **en erreur**, et exécuteur à un seul fil pour
/// [`Threads::Single`].
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
