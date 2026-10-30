//! Closed skeletons of the exchanges with the core: `Intent`, `Command`, `Event`, `Rejection`.
//!
//! No game rule here. Each list stays **closed** (no `#[non_exhaustive]`): adding a variant
//! breaks every `match`, which shows each place that must handle it.

// spec: revues/architecture.md §7.2, §6.7 ; securite.md S4
/// What a character wants to do, from any pilot (player, AI, external): one closed list,
/// shared by the player and the AI. Empty in phase 0: the variants come with their domains.
///
/// Every intent goes through the same filter in `ts_pilot`, AI included, in **two stages**
/// (S4):
/// 1. **epistemic admissibility**: preconditions evaluated on what the character perceives
///    and knows (beliefs, memories, current perception). This stage alone produces the list
///    of possible actions sent to the client, the reluctances and the training action masks;
/// 2. **real resolution**, at execution: the real state decides, and a failure is a result
///    perceptible in the world (the handle resists), which becomes knowledge of the
///    character.
///
/// The absolute rules (minors, deadly danger to oneself), recusation and linked flows are
/// checked at both stages. A refusal is a [`Rejection`] code, never an internal detail.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Intent {}

// spec: securite.md S4
/// Closed list of rejection codes returned to a pilot (S4). No variant carries a detail: a
/// code must not reveal what the character does not know (a locked door, an empty safe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rejection {
    /// The action is not possible for this character.
    Impossible,
    /// The target is out of reach.
    OutOfReach,
    /// The character is busy.
    Busy,
    /// An absolute rule, a recusation or a linked-flow rule forbids it.
    Forbidden,
}

// spec: revues/architecture.md §7.2, §7.3 ; securite.md S8 ; carnet architecture-securite.md §9.11 M3
/// Out-of-fiction operation from a host, the console, a scenario or a tool; always logged.
///
/// The host attaches its **source** to each command and applies the command matrix (S8).
/// The **write** variants (`Spawn`, `SetComponent`) exist only with the `dev-commands`
/// feature, which the kernel refuses together with `public`: a public server cannot even
/// name them (architecture §7.3). The other commands of architecture §7.2 come with their
/// owners: `AttachPilot`, `DetachPilot` and `Reserve` with `ts_pilot`, `Save` with
/// `ts_persist`, `AdvanceTime` with the host (a normal solo command, S8), `SetParam`,
/// `LedgerReversal` and `KillSwitch` with their domains.
///
/// Without `dev-commands`, the list is empty. Witness, then the refusal:
///
/// ```
/// let _ = core::mem::size_of::<ts_model::Command>();
/// ```
#[cfg_attr(
    not(feature = "dev-commands"),
    doc = "```compile_fail,E0599\nlet _ = ts_model::Command::Spawn;\n```"
)]
#[cfg_attr(
    feature = "dev-commands",
    doc = "```\nlet _ = ts_model::Command::Spawn;\n```"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    /// Creates an entity (scenarios and tools only). Payload (`EntitySpec`) to come.
    #[cfg(feature = "dev-commands")]
    Spawn,
    /// Writes a component (scenarios and tools only). Payload to come.
    #[cfg(feature = "dev-commands")]
    SetComponent,
}

// spec: revues/architecture.md §7.2
/// Kind of an event leaving the core, towards the log, the client (filtered), metrics,
/// rewards and moderation. Empty in phase 0: the kinds come with their domains.
///
/// The envelope (`SimEvent { time, seq, kind, visibility }`, `seq` being a world counter)
/// comes with the event log. Its visibility is `Perceptible`, `Private` or `OutOfFiction`; an
/// out-of-fiction event is never given to a rule system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The lists are empty: these functions compile only because no value can exist.
    #[test]
    fn intent_and_event_are_empty() {
        fn _no_intent(i: Intent) -> ! {
            match i {}
        }
        fn _no_event(e: Event) -> ! {
            match e {}
        }
    }

    /// Without `dev-commands`, no command can be built: the write variants are not compiled.
    #[cfg(not(feature = "dev-commands"))]
    #[test]
    fn command_is_empty_without_dev_commands() {
        fn _no_command(c: Command) -> ! {
            match c {}
        }
    }

    #[cfg(feature = "dev-commands")]
    #[test]
    fn write_commands_exist_with_dev_commands() {
        assert_ne!(Command::Spawn, Command::SetComponent);
    }
}
