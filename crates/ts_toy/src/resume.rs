// spec: revues/architecture.md §11.1, §12.4 ; carnet architecture-securite.md §10
//! Generic resume test of the restore contract (AS-27), for every rule plugin.
//!
//! A world run straight through and a world saved at a checkpoint, written to bytes and read
//! back by `ts_persist`, restored, then run on, must have the same state hash at every later
//! checkpoint. A plugin that keeps state outside its declared state (a `Local`, change
//! detection, a message read a tick later, an undeclared resource or entity) diverges after
//! the restore, even when the hash right after it is still equal. Through the bytes, a
//! `StateHash` and `StateDecode` pair that disagree is caught too.
//!
//! Each rule crate runs [`check_resume`] on its plugins, with a test scenario plugin if it
//! needs one, and `ts_game` runs it on `standard_rules()`. Its dates are test parameters.

use core::fmt;

use ts_kernel::rng::WorldKey;
use ts_kernel::{
    BuildError, MemorySnapshot, Plugin, RestoreError, RestoreTarget, Sim, SimConfig, SimTime,
    SnapshotScope, StopCondition,
};
use ts_persist::{Codec, DecodeError, EncodeError, Limits, decode_snapshot, encode_snapshot};

/// Failure of [`check_resume`].
#[derive(Debug)]
pub enum ResumeError {
    /// Fewer than two checkpoints, or not strictly increasing after the epoch.
    Checkpoints,
    /// The rules do not build.
    Build(BuildError),
    /// The snapshot does not encode.
    Encode(EncodeError),
    /// The encoded snapshot does not decode.
    Decode(DecodeError),
    /// The decoded snapshot does not restore.
    Restore(RestoreError),
    /// The world restored at `resumed_at` differs from the straight run at `at`.
    Diverged {
        /// Checkpoint of the snapshot.
        resumed_at: SimTime,
        /// First checkpoint where the hashes differ (`resumed_at` itself right after the
        /// restore).
        at: SimTime,
    },
}

impl fmt::Display for ResumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checkpoints => f.write_str("checkpoints must be at least two, increasing"),
            Self::Build(e) => write!(f, "build: {e}"),
            Self::Encode(e) => write!(f, "encode: {e}"),
            Self::Decode(e) => write!(f, "decode: {e}"),
            Self::Restore(e) => write!(f, "restore: {e}"),
            Self::Diverged { resumed_at, at } => write!(
                f,
                "restored at {} ms, the world differs at {} ms",
                resumed_at.as_millis(),
                at.as_millis()
            ),
        }
    }
}

impl std::error::Error for ResumeError {}

/// Runs `rules` straight through the `checkpoints`, then, for each checkpoint but the last,
/// saves there through `ts_persist`, restores, and runs on: every later checkpoint must give
/// the hash of the straight run. Returns the number of resumes checked.
///
/// Choose checkpoints after the first tick (k > 0), in the middle of whatever the plugins
/// carry from one tick to the next, and with entities in full simulation at some of them.
///
/// # Errors
///
/// The first [`ResumeError`].
pub fn check_resume(
    cfg: &SimConfig,
    rules: &[Box<dyn Plugin>],
    key: &WorldKey,
    checkpoints: &[SimTime],
) -> Result<usize, ResumeError> {
    let increasing = checkpoints.windows(2).all(|w| w[0] < w[1]);
    if checkpoints.len() < 2 || !increasing || checkpoints[0] <= SimTime::EPOCH {
        return Err(ResumeError::Checkpoints);
    }
    let mut straight = Sim::build(cfg.clone(), rules, key).map_err(ResumeError::Build)?;
    let limits = Limits::default();
    let (mut hashes, mut saves) = (Vec::new(), Vec::new());
    for &at in checkpoints {
        straight.run_until(StopCondition::At(at));
        hashes.push(straight.state_hash());
        let snap: MemorySnapshot = straight.snapshot(SnapshotScope::Full);
        saves.push(encode_snapshot(&snap, Codec::Zstd, &limits).map_err(ResumeError::Encode)?);
    }
    let mut resumes = 0;
    for (i, (&resumed_at, bytes)) in checkpoints.iter().zip(&saves).enumerate() {
        if i + 1 == checkpoints.len() {
            break;
        }
        // As a host loads a save: the rules are built once (#58).
        let target = RestoreTarget::build(cfg.clone(), rules, key).map_err(ResumeError::Build)?;
        let decoded =
            decode_snapshot(bytes, &limits, target.layout()).map_err(ResumeError::Decode)?;
        let mut resumed = target.restore(&decoded).map_err(ResumeError::Restore)?;
        for (&at, expected) in checkpoints.iter().zip(&hashes).skip(i) {
            resumed.run_until(StopCondition::At(at));
            if resumed.state_hash() != *expected {
                return Err(ResumeError::Diverged { resumed_at, at });
            }
        }
        resumes += 1;
    }
    Ok(resumes)
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::{ResMut, Resource};
    use ts_kernel::{
        CanonicalError, LodComplete, Phase, SimBuilder, StableEntity, StableIds, StateDecode,
        StateHash, StateReader, StateWriter,
    };

    use super::*;
    use crate::town::TownSpec;
    use crate::world::{STROLL_AT, rules};
    use crate::{day_start, public_test_key};

    /// A small town, enough residents to stroll: test parameters.
    const TOWN: TownSpec = TownSpec {
        width: 8,
        height: 8,
        people: 40,
        rows_per_chunk: 2,
    };

    /// The toy world, which keeps the contract, resumes everywhere: at its first tick, during
    /// a stroll (entities in full simulation) and between two days.
    #[test]
    fn toy_world_resumes_everywhere() {
        let stroll = day_start(7).add_millis(STROLL_AT * 60_000 + 10_000);
        let checkpoints = [
            SimTime::from_millis(50),
            day_start(1),
            stroll,
            stroll.add_millis(5_000),
            day_start(9),
        ];
        let resumes = check_resume(
            &SimConfig::default(),
            &rules(TOWN),
            &public_test_key(1),
            &checkpoints,
        );
        assert_eq!(resumes.unwrap(), 4);
    }

    /// Declared state: the last value the plugin saw.
    #[derive(Resource, Clone, Default)]
    struct Seen(u64);

    impl StateHash for Seen {
        fn hash_state(&self, w: &mut StateWriter<'_>) {
            w.u64(self.0);
        }
    }

    impl StateDecode for Seen {
        fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
            Ok(Self(r.u64()?))
        }
    }

    /// Undeclared counter, kept from one tick to the next: what the contract forbids.
    #[derive(Resource, Default)]
    struct Hidden(u64);

    fn count(mut hidden: ResMut<Hidden>, mut seen: ResMut<Seen>) {
        hidden.0 += 1;
        seen.0 = hidden.0;
    }

    struct Leaky;

    impl Plugin for Leaky {
        fn name(&self) -> &'static str {
            "leaky"
        }
        fn version(&self) -> u32 {
            1
        }
        fn build(&self, b: &mut SimBuilder) {
            b.state_resource::<Seen>("leaky")
                .insert_resource(Seen::default())
                .insert_resource(Hidden::default());
            let world = b.world_mut();
            if let Some(id) = world.resource_mut::<StableIds>().allocate() {
                world.spawn((StableEntity::new(id), LodComplete));
            }
            b.add_systems(Phase::Act, count);
        }
    }

    /// A plugin with hidden state is caught after the restore, although the hash right
    /// after it is equal.
    #[test]
    fn hidden_state_is_caught() {
        let rules: Vec<Box<dyn Plugin>> = vec![Box::new(Leaky)];
        let checkpoints = [SimTime::from_millis(100), SimTime::from_millis(200)];
        let err = check_resume(
            &SimConfig::default(),
            &rules,
            &public_test_key(1),
            &checkpoints,
        );
        assert!(
            matches!(
                err,
                Err(ResumeError::Diverged { resumed_at, at })
                    if resumed_at == checkpoints[0] && at == checkpoints[1]
            ),
            "{err:?}"
        );
    }

    #[test]
    fn checkpoints_are_checked() {
        let rules: Vec<Box<dyn Plugin>> = Vec::new();
        let key = public_test_key(1);
        let cfg = SimConfig::default();
        for bad in [
            &[][..],
            &[SimTime::from_millis(50)],
            &[SimTime::EPOCH, SimTime::from_millis(50)],
            &[SimTime::from_millis(100), SimTime::from_millis(50)],
        ] {
            assert!(matches!(
                check_resume(&cfg, &rules, &key, bad),
                Err(ResumeError::Checkpoints)
            ));
        }
    }
}
