//! Canonical decoder (AS-36): reference vectors, every bound, truncation at every position,
//! counts above the remaining bytes, non-canonical forms, and the checks `restore` keeps.

use super::tests::{Pos, Total, build, restore};
use super::*;
use crate::{
    CanonicalError, CanonicalLimits, Phase, SimBuilder, StateDecode, StateLayout, StateReader,
};

/// One stable entity with `Pos(-2)`, a resource `Total(7)`, one pending event.
struct Small(u32);

impl Plugin for Small {
    fn name(&self) -> &'static str {
        "small"
    }
    fn version(&self) -> u32 {
        self.0
    }
    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Pos>("small")
            .state_resource::<Total>("small")
            .insert_resource(Total(7));
        let world = b.world_mut();
        let id = world.resource_mut::<StableIds>().allocate().unwrap();
        world.spawn((StableEntity::new(id), Pos(-2)));
        let event = TimedEvent {
            kind: 1,
            subject: id,
            arg: 5,
        };
        world
            .resource_mut::<EventQueue>()
            .schedule(SimTime::from_millis(120), event);
        b.add_systems(Phase::Act, |mut q: bevy_ecs::system::Query<&mut Pos>| {
            for mut p in &mut q {
                p.0 -= 1;
            }
        });
    }
}

fn small() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Small(1))]
}

const LIMITS: CanonicalLimits = CanonicalLimits { max_len: 1 << 20 };

fn layout(rules: &[Box<dyn Plugin>]) -> StateLayout {
    StateLayout::of(&SimConfig::default(), rules).unwrap()
}

fn decode(header: &[u8], state: &[u8]) -> Result<MemorySnapshot, CanonicalError> {
    MemorySnapshot::from_canonical(header, state, &layout(&small()), &LIMITS)
}

/// Header written by hand: fingerprint, tick, date, plugins.
fn header(plugins: &[(&str, u32)], fingerprint_of: &[(&str, u32)], tick: i64, now: i64) -> Vec<u8> {
    let mut out = RulesFingerprint::of(fingerprint_of).as_bytes().to_vec();
    out.extend_from_slice(&tick.to_le_bytes());
    out.extend_from_slice(&now.to_le_bytes());
    out.extend_from_slice(&(plugins.len() as u64).to_le_bytes());
    for (name, version) in plugins {
        out.extend_from_slice(&(name.len() as u64).to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&version.to_le_bytes());
    }
    out
}

fn small_header() -> Vec<u8> {
    header(&[("small", 1)], &[("small", 1)], 50, 0)
}

/// Offsets of the fields of [`small_state`].
const EVENTS_COUNT: usize = 14 + 32;
const ENTITIES_COUNT: usize = EVENTS_COUNT + 8 + 36;
const COMPLETE_COUNT: usize = ENTITIES_COUNT + 16;
const POS_ID: usize = COMPLETE_COUNT + 8 + 13 + 4 + 8;
const TOTAL_TAG: usize = POS_ID + 16 + 4;

/// Canonical state of a fresh `Small` world, written by hand.
fn small_state() -> Vec<u8> {
    let mut s = Vec::new();
    let push_str = |s: &mut Vec<u8>, v: &str| {
        s.extend_from_slice(&(v.len() as u64).to_le_bytes());
        s.extend_from_slice(v.as_bytes());
    };
    push_str(&mut s, "kernel");
    s.extend_from_slice(&0_i64.to_le_bytes()); // date
    s.extend_from_slice(&0_i64.to_le_bytes()); // day origin
    s.extend_from_slice(&1_u64.to_le_bytes()); // next StableId
    s.extend_from_slice(&1_u64.to_le_bytes()); // next seq
    assert_eq!(s.len(), EVENTS_COUNT);
    s.extend_from_slice(&1_u64.to_le_bytes()); // one event
    s.extend_from_slice(&120_i64.to_le_bytes());
    s.extend_from_slice(&0_u64.to_le_bytes()); // seq
    s.extend_from_slice(&1_u32.to_le_bytes()); // kind
    s.extend_from_slice(&0_u64.to_le_bytes()); // subject
    s.extend_from_slice(&5_u64.to_le_bytes()); // arg
    assert_eq!(s.len(), ENTITIES_COUNT);
    s.extend_from_slice(&1_u64.to_le_bytes()); // one stable entity
    s.extend_from_slice(&0_u64.to_le_bytes());
    s.extend_from_slice(&0_u64.to_le_bytes()); // none in full simulation
    push_str(&mut s, "small");
    s.extend_from_slice(&0_u32.to_le_bytes()); // state type 0: Pos
    s.extend_from_slice(&1_u64.to_le_bytes());
    assert_eq!(s.len(), POS_ID);
    s.extend_from_slice(&0_u64.to_le_bytes());
    s.extend_from_slice(&(-2_i64).to_le_bytes());
    s.extend_from_slice(&1_u32.to_le_bytes()); // state type 1: Total
    assert_eq!(s.len(), TOTAL_TAG);
    s.push(1);
    s.extend_from_slice(&7_i64.to_le_bytes());
    s
}

fn put(bytes: &mut [u8], at: usize, value: &[u8]) {
    bytes[at..at + value.len()].copy_from_slice(value);
}

/// Frozen vectors: the kernel writes exactly these bytes, and decodes them back into a
/// snapshot that restores the same world.
#[test]
fn reference_vectors() {
    let sim = build(&small());
    let snap: MemorySnapshot = sim.snapshot(SnapshotScope::Full);
    assert_eq!(snap.header_bytes(), small_header());
    assert_eq!(snap.canonical_bytes(), small_state());
    let decoded = decode(&small_header(), &small_state()).unwrap();
    assert_eq!(decoded.now(), SimTime::EPOCH);
    assert_eq!(decoded.rules_fingerprint(), sim.rules_fingerprint());
    let restored = restore(&decoded, &small()).unwrap();
    assert_eq!(restored.state_hash(), sim.state_hash());

    let empty = build(&[]);
    let snap: MemorySnapshot = empty.snapshot(SnapshotScope::Full);
    assert_eq!(snap.header_bytes(), header(&[], &[], 50, 0));
    let decoded = MemorySnapshot::from_canonical(
        &snap.header_bytes(),
        snap.canonical_bytes(),
        &layout(&[]),
        &LIMITS,
    )
    .unwrap();
    assert_eq!(
        restore(&decoded, &[]).unwrap().state_hash(),
        empty.state_hash()
    );
}

/// Each input is bounded by `max_len`: at the bound it decodes, one byte above it does not.
#[test]
fn input_bound() {
    let (h, s) = (small_header(), small_state());
    let at = |max_len: usize| CanonicalLimits {
        max_len: max_len as u64,
    };
    let l = layout(&small());
    let both = h.len().max(s.len());
    assert!(MemorySnapshot::from_canonical(&h, s.clone(), &l, &at(both)).is_ok());
    let too_large = |len: usize| CanonicalError::TooLarge {
        len: len as u64,
        max: (len - 1) as u64,
    };
    let e = MemorySnapshot::from_canonical(&h, s.clone(), &l, &at(h.len() - 1)).unwrap_err();
    assert_eq!(e, too_large(h.len()));
    let big_header = at(h.len().max(s.len() - 1));
    let e = MemorySnapshot::from_canonical(&h, s.clone(), &l, &big_header).unwrap_err();
    assert_eq!(e, too_large(s.len()));
}

/// Every proper prefix of the header and of the state is refused, and so is one byte more.
#[test]
fn truncation_at_every_position() {
    let (h, s) = (small_header(), small_state());
    for cut in 0..h.len() {
        let e = decode(&h[..cut], &s).unwrap_err();
        assert!(
            matches!(
                e,
                CanonicalError::Truncated { .. } | CanonicalError::CountTooLarge { .. }
            ),
            "header cut at {cut}: {e:?}"
        );
    }
    for cut in 0..s.len() {
        let e = decode(&h, &s[..cut]).unwrap_err();
        assert!(
            matches!(
                e,
                CanonicalError::Truncated { .. } | CanonicalError::CountTooLarge { .. }
            ),
            "state cut at {cut}: {e:?}"
        );
    }
    let longer = [&s[..], &[0]].concat();
    assert_eq!(
        decode(&h, &longer).unwrap_err(),
        CanonicalError::TrailingBytes { at: s.len() as u64 }
    );
}

/// A count above the remaining bytes is refused before any element is read.
#[test]
fn count_above_remaining_bytes() {
    let len = small_state().len();
    // One more than the bytes left after the count of the complete entities.
    let just_over = (len - COMPLETE_COUNT - 8 + 1) as u64;
    for (at, count) in [
        (EVENTS_COUNT, u64::MAX),
        (ENTITIES_COUNT, 1 << 40),
        (COMPLETE_COUNT, just_over),
        (0, 1000),
    ] {
        let mut s = small_state();
        put(&mut s, at, &count.to_le_bytes());
        assert_eq!(
            decode(&small_header(), &s).unwrap_err(),
            CanonicalError::CountTooLarge {
                at: at as u64,
                count
            }
        );
    }
    // Exactly the bytes left is not a count error (the elements are then malformed).
    let mut s = small_state();
    put(&mut s, COMPLETE_COUNT, &(just_over - 1).to_le_bytes());
    assert!(!matches!(
        decode(&small_header(), &s),
        Err(CanonicalError::CountTooLarge { .. }) | Ok(_)
    ));
    let mut h = small_header();
    put(&mut h, 48, &u64::MAX.to_le_bytes());
    assert_eq!(
        decode(&h, &small_state()).unwrap_err(),
        CanonicalError::CountTooLarge {
            at: 48,
            count: u64::MAX
        }
    );
}

#[derive(Debug)]
struct Nothing;

impl StateDecode for Nothing {
    fn decode_state(_: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Ok(Self)
    }
}

/// Values decode exactly as `StateHash` writes them; a zero-byte element cannot make a
/// sequence run past its count.
#[test]
fn value_reader() {
    fn read(bytes: &[u8]) -> StateReader<'_> {
        StateReader::new(bytes, &LIMITS).unwrap()
    }
    let mut r = read(&[2, 1, 0xff, 0xff, 0xff, 0xff, 1, 0, 0, 0x80, 0x3f]);
    assert_eq!(u16::decode_state(&mut r), Ok(0x0102));
    assert_eq!(i32::decode_state(&mut r), Ok(-1));
    assert_eq!(bool::decode_state(&mut r), Ok(true));
    assert_eq!(f32::decode_state(&mut r), Ok(1.0));
    r.finish().unwrap();
    let mut r = read(&[1, 5, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        Option::<StableId>::decode_state(&mut r),
        Ok(Some(StableId::from_snapshot_raw(5)))
    );
    assert_eq!(Option::<u8>::decode_state(&mut r), Ok(None));
    let mut r = read(&[2, 0, 0, 0, 0, 0, 0, 0, 7, 8]);
    assert_eq!(Vec::<u8>::decode_state(&mut r), Ok(vec![7, 8]));
    let mut r = read(&[2, 0, 0, 0, 0, 0, 0, 0, b'a', b'b']);
    assert_eq!(r.str(), Ok("ab"));
    let mut r = read(&[2, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xfe]);
    assert_eq!(r.str(), Err(CanonicalError::InvalidUtf8 { at: 0 }));
    assert_eq!(
        bool::decode_state(&mut read(&[2])),
        Err(CanonicalError::InvalidTag { at: 0, tag: 2 })
    );
    let mut r = read(&[3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]);
    assert_eq!(
        Vec::<Nothing>::decode_state(&mut r).unwrap_err(),
        CanonicalError::EmptyElement { at: 8 }
    );
}

/// Valid values in another order, another layout or with dangling ids are refused.
#[test]
fn non_canonical_forms() {
    let h = small_header();
    let refused = |patch: &dyn Fn(&mut Vec<u8>)| {
        let mut s = small_state();
        patch(&mut s);
        decode(&h, &s).unwrap_err()
    };
    assert_eq!(
        refused(&|s| s[TOTAL_TAG] = 2),
        CanonicalError::InvalidTag {
            at: TOTAL_TAG as u64,
            tag: 2
        }
    );
    assert_eq!(refused(&|s| s[8] = b'K'), CanonicalError::Layout("family"));
    assert_eq!(
        refused(&|s| put(s, TOTAL_TAG - 4, &0_u32.to_le_bytes())),
        CanonicalError::Layout("state index")
    );
    assert_eq!(
        refused(&|s| put(s, POS_ID, &3_u64.to_le_bytes())),
        CanonicalError::NotCanonical("component on an unknown StableId")
    );
    // Complete entities must be stable ones.
    let mut s = small_state();
    put(&mut s, COMPLETE_COUNT, &1_u64.to_le_bytes());
    s.splice(COMPLETE_COUNT + 8..COMPLETE_COUNT + 8, 9_u64.to_le_bytes());
    assert_eq!(
        decode(&h, &s).unwrap_err(),
        CanonicalError::NotCanonical("LodComplete on an unknown StableId")
    );
    // Two equal entities.
    let mut s = small_state();
    put(&mut s, ENTITIES_COUNT, &2_u64.to_le_bytes());
    s.splice(ENTITIES_COUNT + 8..ENTITIES_COUNT + 8, 0_u64.to_le_bytes());
    assert_eq!(
        decode(&h, &s).unwrap_err(),
        CanonicalError::NotCanonical("stable entities")
    );
    // Another rule set: its layout has no `small` family, so those bytes are left over.
    let other = MemorySnapshot::from_canonical(&h, small_state(), &layout(&[]), &LIMITS);
    assert_eq!(
        other.unwrap_err(),
        CanonicalError::TrailingBytes {
            at: (COMPLETE_COUNT + 8) as u64
        }
    );
}

/// The header is checked against itself and against the state.
#[test]
fn header_checks() {
    let s = small_state();
    assert_eq!(
        decode(&header(&[("small", 1)], &[("small", 1)], 50, 1000), &s).unwrap_err(),
        CanonicalError::DateMismatch {
            header: 1000,
            state: 0
        }
    );
    assert_eq!(
        decode(&header(&[("small", 1)], &[("small", 2)], 50, 0), &s).unwrap_err(),
        CanonicalError::RulesFingerprint
    );
    let twice = [("small", 1), ("small", 1)];
    assert_eq!(
        decode(&header(&twice, &twice, 50, 0), &s).unwrap_err(),
        CanonicalError::NotCanonical("plugin names")
    );
}

/// `from_canonical` bypasses none of the checks of `restore` (AS-36 condition 2).
#[test]
fn restore_keeps_its_checks() {
    let s = small_state();
    let other_rules = decode(&header(&[("small", 2)], &[("small", 2)], 50, 0), &s).unwrap();
    assert!(matches!(
        restore(&other_rules, &small()),
        Err(RestoreError::RulesMismatch { .. })
    ));
    let other_tick = decode(&header(&[("small", 1)], &[("small", 1)], 100, 0), &s).unwrap();
    assert_eq!(
        restore(&other_tick, &small()).unwrap_err(),
        RestoreError::TickMismatch {
            snapshot: 100,
            current: 50
        }
    );
    let mut counter = small_state();
    put(&mut counter, 30, &0_u64.to_le_bytes());
    let beyond = decode(&small_header(), &counter).unwrap();
    assert_eq!(
        restore(&beyond, &small()).unwrap_err(),
        RestoreError::StableIdBeyondCounter(StableId::from_snapshot_raw(0))
    );
    let mut seq = small_state();
    put(&mut seq, 38, &0_u64.to_le_bytes());
    let stale = decode(&small_header(), &seq).unwrap();
    assert_eq!(
        restore(&stale, &small()).unwrap_err(),
        RestoreError::Corrupt("event ranks")
    );
    let mut off_grid = small_state();
    put(&mut off_grid, 14, &7_i64.to_le_bytes());
    let h = header(&[("small", 1)], &[("small", 1)], 50, 7);
    let late = decode(&h, &off_grid).unwrap();
    assert_eq!(
        restore(&late, &small()).unwrap_err(),
        RestoreError::Corrupt("date off the tick grid")
    );
}

/// AS-39: the bytes of one value are those it has inside a snapshot.
#[test]
fn single_value_bytes_match_the_snapshot() {
    let pos = crate::state_hash_bytes(&Pos(-2));
    assert_eq!(pos, (-2_i64).to_le_bytes());
    let s = small_state();
    assert_eq!(&s[POS_ID + 8..POS_ID + 16], &pos[..]);
    assert_eq!(&s[TOTAL_TAG + 1..], &crate::state_hash_bytes(&Total(7))[..]);
    let event = TimedEvent {
        kind: 1,
        subject: StableId::from_snapshot_raw(0),
        arg: 5,
    };
    let at = EVENTS_COUNT + 8 + 16;
    assert_eq!(&s[at..at + 20], &crate::state_hash_bytes(&event)[..]);
}

/// A plugin with no content, named at will.
struct Named(&'static str);

impl Plugin for Named {
    fn name(&self) -> &'static str {
        self.0
    }
    fn version(&self) -> u32 {
        1
    }
    fn build(&self, _: &mut SimBuilder) {}
}

/// The header lists the plugins sorted by name, whatever the rules order, and refuses any
/// other order: one byte string per rule set.
#[test]
fn plugins_are_sorted_in_the_header() {
    let rules: Vec<Box<dyn Plugin>> = vec![Box::new(Named("zz")), Box::new(Named("aa"))];
    let snap: MemorySnapshot = build(&rules).snapshot(SnapshotScope::Full);
    let sorted = [("aa", 1), ("zz", 1)];
    assert_eq!(snap.header_bytes(), header(&sorted, &sorted, 50, 0));
    let unsorted = header(&[("zz", 1), ("aa", 1)], &sorted, 50, 0);
    let refused =
        MemorySnapshot::from_canonical(&unsorted, snap.canonical_bytes(), &layout(&rules), &LIMITS);
    assert_eq!(
        refused.unwrap_err(),
        CanonicalError::NotCanonical("plugin names")
    );
    let decoded = MemorySnapshot::from_canonical(
        &snap.header_bytes(),
        snap.canonical_bytes(),
        &layout(&rules),
        &LIMITS,
    )
    .unwrap();
    restore(&decoded, &rules).unwrap();
}

/// Dates stay far from `i64` overflow: a forged date cannot make a later tick overflow.
#[test]
fn dates_are_bounded() {
    let out = CanonicalError::Invalid("date out of range");
    let far = (1_i64 << 62) + 1;
    let mut s = small_state();
    put(&mut s, 14, &far.to_le_bytes());
    let h = header(&[("small", 1)], &[("small", 1)], 50, far);
    assert_eq!(decode(&h, &s).unwrap_err(), out);
    let h = header(&[("small", 1)], &[("small", 1)], 50, -far);
    assert_eq!(decode(&h, &small_state()).unwrap_err(), out);
    let mut s = small_state();
    put(&mut s, EVENTS_COUNT + 8, &i64::MIN.to_le_bytes());
    assert_eq!(decode(&small_header(), &s).unwrap_err(), out);
    let mut s = small_state();
    put(&mut s, 22, &((1_i64 << 35) + 1).to_le_bytes());
    assert_eq!(
        decode(&small_header(), &s).unwrap_err(),
        CanonicalError::Invalid("day origin out of range")
    );
    // At the bound, the date is accepted and the world still ticks.
    let edge = (1_i64 << 62) - (1_i64 << 62) % 50;
    let mut s = small_state();
    put(&mut s, 14, &edge.to_le_bytes());
    let h = header(&[("small", 1)], &[("small", 1)], 50, edge);
    let mut sim = restore(&decode(&h, &s).unwrap(), &small()).unwrap();
    sim.step();
}

/// Offsets of the two counters in [`small_state`].
const NEXT_STABLE_ID: usize = 30;
const NEXT_SEQ: usize = 38;

/// The counters and the tick length are bounded: at their maximum, a typed error, never a
/// later panic (review I1 of #57).
#[test]
fn counters_and_tick_are_bounded() {
    for (at, what) in [
        (NEXT_STABLE_ID, "StableId counter out of range"),
        (NEXT_SEQ, "event counter out of range"),
    ] {
        for v in [u64::MAX, (1 << 62) + 1] {
            let mut s = small_state();
            put(&mut s, at, &v.to_le_bytes());
            assert_eq!(
                decode(&small_header(), &s).unwrap_err(),
                CanonicalError::Invalid(what)
            );
        }
    }
    for tick in [i64::MIN, -1, 0, 1001, i64::MAX] {
        let h = header(&[("small", 1)], &[("small", 1)], tick, 0);
        assert_eq!(
            decode(&h, &small_state()).unwrap_err(),
            CanonicalError::Invalid("tick length out of range")
        );
    }
    // At the bound, both counters still work after the restore.
    let mut s = small_state();
    put(&mut s, NEXT_STABLE_ID, &(1_u64 << 62).to_le_bytes());
    put(&mut s, NEXT_SEQ, &(1_u64 << 62).to_le_bytes());
    let mut sim = restore(&decode(&small_header(), &s).unwrap(), &small()).unwrap();
    let id = sim.world.resource_mut::<StableIds>().allocate();
    assert_eq!(id.map(StableId::index), Some(1 << 62));
    let event = TimedEvent {
        kind: 2,
        subject: StableId::from_snapshot_raw(0),
        arg: 0,
    };
    let seq = sim
        .world
        .resource_mut::<EventQueue>()
        .schedule(SimTime::from_millis(500), event);
    assert_eq!(seq, 1 << 62);
    sim.step();
}
