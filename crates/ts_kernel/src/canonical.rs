// spec: revues/architecture.md §11.3, §12.1 ; carnet architecture-securite.md AS-36, SEC0-23, C6
//! Decoder of the canonical encoding: the twin of [`StateWriter`] and of the snapshot
//! encoding. It reads **untrusted bytes** (a solo save), so:
//! - no panic on any input: the anti-panic lints below are denied in this module, with no
//!   local exemption (C6); every failure is a typed [`CanonicalError`];
//! - no length or count is trusted: nothing is allocated from a value read in the input
//!   (no `with_capacity`); a count is checked against the remaining bytes **and** against
//!   [`CanonicalLimits::max_len`], and each element of a sequence must consume at least one
//!   byte, so a sequence never runs past the input;
//! - the encoding must be canonical: tags are `0` or `1`, identifiers and events strictly
//!   increasing, families and state indices exactly those of the rules.
//!
//! **Header** (part `kernel.meta` of a save), written by `MemorySnapshot::header_bytes`:
//! rules fingerprint (32 raw bytes), tick length (`i64` ms), date (`i64` ms), then the
//! active plugins: count, then for each its name ([`StateWriter::str`]) and version (`u32`).
//!
//! **State** (part `kernel.state`): the canonical encoding of the state hash, families
//! sorted by name; see `write_family` and `write_kernel`.
//!
//! Decoding builds no [`crate::Sim`]: `restore` stays the only path to one, with all its
//! checks (AS-36 condition 2).

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::arithmetic_side_effects
)]

use core::fmt;
use std::collections::BTreeSet;

use crate::state::{Boxed, Captured, StateEntry, StateKind, StateWriter, encode_with, families};
use crate::{
    DayOrigin, KERNEL_FAMILY, Money, RulesFingerprint, Scheduled, SimTime, StableId, TimedEvent,
};

// spec: carnet architecture-securite.md SEC0-23, AS-36
/// Bounds of the canonical decoder. `ts_persist` passes the same values as its own
/// `Limits` (`max_part_len`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalLimits {
    /// Largest input, in bytes, and largest count a sequence may announce.
    pub max_len: u64,
}

// spec: carnet architecture-securite.md SEC0-23, AS-36
/// Why canonical bytes were refused. Positions `at` are byte offsets in the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalError {
    /// The input is longer than [`CanonicalLimits::max_len`].
    TooLarge {
        /// Input length.
        len: u64,
        /// Allowed maximum.
        max: u64,
    },
    /// The input ends inside a value.
    Truncated {
        /// Offset of the value.
        at: u64,
    },
    /// A count announces more than the remaining bytes or the bound.
    CountTooLarge {
        /// Offset of the count.
        at: u64,
        /// Announced count.
        count: u64,
    },
    /// An element of a sequence consumed no byte.
    EmptyElement {
        /// Offset of the element.
        at: u64,
    },
    /// A tag (`bool`, `Option`, resource presence) is neither `0` nor `1`.
    InvalidTag {
        /// Offset of the tag.
        at: u64,
        /// Tag read.
        tag: u8,
    },
    /// A string is not UTF-8.
    InvalidUtf8 {
        /// Offset of the string.
        at: u64,
    },
    /// Bytes are left after the last value.
    TrailingBytes {
        /// Offset of the first byte left.
        at: u64,
    },
    /// The families or the state types differ from those the rules declare.
    Layout(&'static str),
    /// A value is valid but not in canonical form (order, duplicate).
    NotCanonical(&'static str),
    /// The plugin list of the header does not give its rules fingerprint.
    RulesFingerprint,
    /// The date of the header differs from the date of the state.
    DateMismatch {
        /// Date in the header, in milliseconds.
        header: i64,
        /// Date in the state, in milliseconds.
        state: i64,
    },
    /// A [`StateDecode`] implementation refused a value.
    Invalid(&'static str),
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge { len, max } => write!(f, "canonical input too large: {len} > {max}"),
            Self::Truncated { at } => write!(f, "truncated value at {at}"),
            Self::CountTooLarge { at, count } => write!(f, "count {count} too large at {at}"),
            Self::EmptyElement { at } => write!(f, "empty sequence element at {at}"),
            Self::InvalidTag { at, tag } => write!(f, "invalid tag {tag} at {at}"),
            Self::InvalidUtf8 { at } => write!(f, "invalid UTF-8 at {at}"),
            Self::TrailingBytes { at } => write!(f, "trailing bytes at {at}"),
            Self::Layout(what) => write!(f, "state layout differs from the rules: {what}"),
            Self::NotCanonical(what) => write!(f, "not canonical: {what}"),
            Self::RulesFingerprint => f.write_str("plugin list does not match its fingerprint"),
            Self::DateMismatch { header, state } => {
                write!(
                    f,
                    "header date {header} ms differs from state date {state} ms"
                )
            }
            Self::Invalid(what) => write!(f, "invalid value: {what}"),
        }
    }
}

impl std::error::Error for CanonicalError {}

/// Exact conversion on our 64-bit targets; saturates otherwise.
fn len64(n: usize) -> u64 {
    u64::try_from(n).unwrap_or(u64::MAX)
}

// spec: revues/architecture.md §11.3 ; carnet architecture-securite.md AS-36
/// Bounded reader of the canonical encoding, the twin of [`StateWriter`].
pub struct StateReader<'a> {
    rest: &'a [u8],
    len: usize,
    max_count: u64,
}

impl fmt::Debug for StateReader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StateReader")
            .field("position", &self.position())
            .field("remaining", &self.rest.len())
            .finish()
    }
}

impl<'a> StateReader<'a> {
    /// Reader over `bytes`.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::TooLarge`] if `bytes` is longer than `limits.max_len`.
    pub fn new(bytes: &'a [u8], limits: &CanonicalLimits) -> Result<Self, CanonicalError> {
        let len = len64(bytes.len());
        if len > limits.max_len {
            return Err(CanonicalError::TooLarge {
                len,
                max: limits.max_len,
            });
        }
        Ok(Self {
            rest: bytes,
            len: bytes.len(),
            max_count: limits.max_len,
        })
    }

    /// Offset of the next byte.
    pub fn position(&self) -> u64 {
        len64(self.len.saturating_sub(self.rest.len()))
    }

    /// Bytes left.
    pub const fn remaining(&self) -> usize {
        self.rest.len()
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], CanonicalError> {
        let at = self.position();
        let (head, tail) = self
            .rest
            .split_at_checked(n)
            .ok_or(CanonicalError::Truncated { at })?;
        self.rest = tail;
        Ok(head)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], CanonicalError> {
        let at = self.position();
        let (head, tail) = self
            .rest
            .split_first_chunk::<N>()
            .ok_or(CanonicalError::Truncated { at })?;
        self.rest = tail;
        Ok(*head)
    }

    /// `u8`.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::Truncated`].
    pub fn u8(&mut self) -> Result<u8, CanonicalError> {
        self.array::<1>().map(u8::from_le_bytes)
    }

    /// `u32` LE.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::Truncated`].
    pub fn u32(&mut self) -> Result<u32, CanonicalError> {
        self.array().map(u32::from_le_bytes)
    }

    /// `u64` LE.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::Truncated`].
    pub fn u64(&mut self) -> Result<u64, CanonicalError> {
        self.array().map(u64::from_le_bytes)
    }

    /// `i64` LE.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::Truncated`].
    pub fn i64(&mut self) -> Result<i64, CanonicalError> {
        self.array().map(i64::from_le_bytes)
    }

    /// `f64` from its bits, LE.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::Truncated`].
    pub fn f64(&mut self) -> Result<f64, CanonicalError> {
        self.u64().map(f64::from_bits)
    }

    /// `0` or `1`, as written for a `bool`, an `Option` or a resource's presence.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::Truncated`], [`CanonicalError::InvalidTag`].
    pub fn tag(&mut self) -> Result<bool, CanonicalError> {
        let at = self.position();
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            tag => Err(CanonicalError::InvalidTag { at, tag }),
        }
    }

    /// Length of a sequence ([`StateWriter::count`]), never above the remaining bytes nor
    /// [`CanonicalLimits::max_len`].
    ///
    /// # Errors
    ///
    /// [`CanonicalError::Truncated`], [`CanonicalError::CountTooLarge`].
    pub fn count(&mut self) -> Result<usize, CanonicalError> {
        let at = self.position();
        let count = self.u64()?;
        if count > len64(self.rest.len()) || count > self.max_count {
            return Err(CanonicalError::CountTooLarge { at, count });
        }
        usize::try_from(count).map_err(|_| CanonicalError::CountTooLarge { at, count })
    }

    /// Bytes preceded by their length ([`StateWriter::bytes`]).
    ///
    /// # Errors
    ///
    /// See [`Self::count`].
    pub fn bytes(&mut self) -> Result<&'a [u8], CanonicalError> {
        let n = self.count()?;
        self.take(n)
    }

    /// UTF-8 string preceded by its length ([`StateWriter::str`]).
    ///
    /// # Errors
    ///
    /// See [`Self::count`]; [`CanonicalError::InvalidUtf8`].
    pub fn str(&mut self) -> Result<&'a str, CanonicalError> {
        let at = self.position();
        let bytes = self.bytes()?;
        core::str::from_utf8(bytes).map_err(|_| CanonicalError::InvalidUtf8 { at })
    }

    /// Sequence written as a count then the elements: `f` decodes one element, which must
    /// consume at least one byte. The vector grows with the decoded elements only.
    ///
    /// # Errors
    ///
    /// See [`Self::count`]; [`CanonicalError::EmptyElement`]; any error of `f`.
    pub fn seq<T>(
        &mut self,
        mut f: impl FnMut(&mut Self) -> Result<T, CanonicalError>,
    ) -> Result<Vec<T>, CanonicalError> {
        let n = self.count()?;
        let mut out = Vec::new();
        for _ in 0..n {
            let at = self.position();
            let before = self.rest.len();
            out.push(f(self)?);
            if self.rest.len() == before {
                return Err(CanonicalError::EmptyElement { at });
            }
        }
        Ok(out)
    }

    /// Ends the input.
    ///
    /// # Errors
    ///
    /// [`CanonicalError::TrailingBytes`] if bytes are left.
    pub fn finish(self) -> Result<(), CanonicalError> {
        if self.rest.is_empty() {
            Ok(())
        } else {
            Err(CanonicalError::TrailingBytes {
                at: self.position(),
            })
        }
    }
}

// spec: revues/architecture.md §11.3, §12.1 ; carnet architecture-securite.md AS-36
/// Decoding twin of [`crate::StateHash`]: reads exactly what `hash_state` writes, and
/// refuses with a typed error anything else. Required by state components and resources.
pub trait StateDecode: Sized {
    /// Reads a value.
    ///
    /// # Errors
    ///
    /// Any [`CanonicalError`]; never a panic.
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError>;
}

macro_rules! state_decode_int {
    ($($t:ty),*) => {$(
        impl StateDecode for $t {
            fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
                r.array().map(<$t>::from_le_bytes)
            }
        }
    )*};
}
state_decode_int!(u8, u16, u32, u64, i8, i16, i32, i64);

impl StateDecode for bool {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.tag()
    }
}

impl StateDecode for f32 {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.u32().map(f32::from_bits)
    }
}

impl StateDecode for f64 {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.f64()
    }
}

impl StateDecode for StableId {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.u64().map(Self::from_snapshot_raw)
    }
}

impl StateDecode for SimTime {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.i64().map(Self::from_millis)
    }
}

impl StateDecode for DayOrigin {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.i64().map(Self::from_days)
    }
}

impl StateDecode for Money {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.i64().map(Self::from_cents)
    }
}

impl<T: StateDecode> StateDecode for Option<T> {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        if r.tag()? {
            T::decode_state(r).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<T: StateDecode> StateDecode for Vec<T> {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.seq(T::decode_state)
    }
}

/// Identifiers in strictly increasing order, as the kernel writes them.
fn sorted_ids(
    r: &mut StateReader<'_>,
    what: &'static str,
) -> Result<Vec<StableId>, CanonicalError> {
    let ids = r.seq(StableId::decode_state)?;
    if ids.windows(2).any(|w| matches!(w, [a, b] if a >= b)) {
        return Err(CanonicalError::NotCanonical(what));
    }
    Ok(ids)
}

/// Values of a state component: count, then `(StableId, value)` by increasing id.
pub(crate) fn decode_component<C: StateDecode + Send + Sync + 'static>(
    r: &mut StateReader<'_>,
) -> Result<Vec<(StableId, Boxed)>, CanonicalError> {
    let mut last = None;
    r.seq(|r| {
        let id = StableId::decode_state(r)?;
        if last.is_some_and(|l| l >= id) {
            return Err(CanonicalError::NotCanonical("component ids"));
        }
        last = Some(id);
        Ok((id, Box::new(C::decode_state(r)?) as Boxed))
    })
}

/// Value of a state resource: presence tag, then the value.
pub(crate) fn decode_resource<R: StateDecode + Send + Sync + 'static>(
    r: &mut StateReader<'_>,
) -> Result<Option<Boxed>, CanonicalError> {
    if r.tag()? {
        Ok(Some(Box::new(R::decode_state(r)?)))
    } else {
        Ok(None)
    }
}

/// Header of a snapshot: what its canonical state does not hold.
pub(crate) struct Header {
    pub(crate) rules: RulesFingerprint,
    pub(crate) tick_ms: i64,
    pub(crate) now: SimTime,
    pub(crate) plugins: Vec<(String, u32)>,
}

/// Encodes a header (see the module documentation).
pub(crate) fn write_header(
    rules: &RulesFingerprint,
    tick_ms: i64,
    now: SimTime,
    plugins: &[(String, u32)],
) -> Vec<u8> {
    encode_with(|w: &mut StateWriter<'_>| {
        w.raw(rules.as_bytes());
        w.i64(tick_ms);
        w.i64(now.as_millis());
        w.count(plugins.len());
        for (name, version) in plugins {
            w.str(name);
            w.u32(*version);
        }
    })
}

fn decode_header(bytes: &[u8], limits: &CanonicalLimits) -> Result<Header, CanonicalError> {
    let mut r = StateReader::new(bytes, limits)?;
    let rules = RulesFingerprint::from_bytes(r.array()?);
    let tick_ms = r.i64()?;
    let now = SimTime::decode_state(&mut r)?;
    let plugins = r.seq(|r| Ok((r.str()?.to_owned(), r.u32()?)))?;
    r.finish()?;
    let mut names = BTreeSet::new();
    for (name, _) in &plugins {
        // `RulesFingerprint::of` needs u32 lengths; a duplicate name is not a rule set.
        if u32::try_from(name.len()).is_err() || !names.insert(name.as_str()) {
            return Err(CanonicalError::NotCanonical("plugin names"));
        }
    }
    if u32::try_from(plugins.len()).is_err() {
        return Err(CanonicalError::NotCanonical("plugin count"));
    }
    let list: Vec<(&str, u32)> = plugins.iter().map(|(n, v)| (n.as_str(), *v)).collect();
    if RulesFingerprint::of(&list) != rules {
        return Err(CanonicalError::RulesFingerprint);
    }
    Ok(Header {
        rules,
        tick_ms,
        now,
        plugins,
    })
}

/// Kernel family of the state: clock, counters, events, stable entities.
pub(crate) struct KernelState {
    pub(crate) now: SimTime,
    pub(crate) day_origin: DayOrigin,
    pub(crate) next_stable_id: u64,
    pub(crate) next_seq: u64,
    pub(crate) events: Vec<Scheduled<TimedEvent>>,
    pub(crate) entities: Vec<StableId>,
    pub(crate) complete: Vec<StableId>,
}

fn decode_kernel(r: &mut StateReader<'_>) -> Result<KernelState, CanonicalError> {
    let now = SimTime::decode_state(r)?;
    let day_origin = DayOrigin::decode_state(r)?;
    let next_stable_id = r.u64()?;
    let next_seq = r.u64()?;
    let events = r.seq(|r| {
        let at = SimTime::decode_state(r)?;
        let seq = r.u64()?;
        let event = TimedEvent {
            kind: r.u32()?,
            subject: StableId::decode_state(r)?,
            arg: r.u64()?,
        };
        Ok(Scheduled { at, seq, event })
    })?;
    if events
        .windows(2)
        .any(|w| matches!(w, [a, b] if (a.at, a.seq) >= (b.at, b.seq)))
    {
        return Err(CanonicalError::NotCanonical("event order"));
    }
    let entities = sorted_ids(r, "stable entities")?;
    let complete = sorted_ids(r, "LodComplete entities")?;
    if complete
        .iter()
        .any(|id| entities.binary_search(id).is_err())
    {
        return Err(CanonicalError::NotCanonical(
            "LodComplete on an unknown StableId",
        ));
    }
    Ok(KernelState {
        now,
        day_origin,
        next_stable_id,
        next_seq,
        events,
        entities,
        complete,
    })
}

/// A snapshot decoded from its header and canonical state.
pub(crate) struct Decoded {
    pub(crate) header: Header,
    pub(crate) kernel: KernelState,
    /// Values of each state type, in declaration order.
    pub(crate) captured: Vec<Captured>,
}

// spec: revues/architecture.md §11.3, §12.1 ; carnet architecture-securite.md AS-36
/// Decodes a header and a canonical state against the state types `entries` declared by
/// the rules: families sorted, each with exactly its state types, in declaration order.
pub(crate) fn decode(
    header: &[u8],
    state: &[u8],
    entries: &[StateEntry],
    limits: &CanonicalLimits,
) -> Result<Decoded, CanonicalError> {
    let header = decode_header(header, limits)?;
    let mut r = StateReader::new(state, limits)?;
    let mut kernel = None;
    // Sized by the rules, never by the input.
    let mut captured: Vec<Option<Captured>> = entries.iter().map(|_| None).collect();
    for family in families(entries) {
        if r.str()? != family {
            return Err(CanonicalError::Layout("family"));
        }
        if family == KERNEL_FAMILY {
            kernel = Some(decode_kernel(&mut r)?);
        }
        for (i, e) in entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.family == family)
        {
            if usize::try_from(r.u32()?).ok() != Some(i) {
                return Err(CanonicalError::Layout("state index"));
            }
            let value = match e.kind {
                StateKind::Component { decode, .. } => Captured::Component(decode(&mut r)?),
                StateKind::Resource { decode, .. } => Captured::Resource(decode(&mut r)?),
            };
            let slot = captured
                .get_mut(i)
                .ok_or(CanonicalError::Layout("state index"))?;
            *slot = Some(value);
        }
    }
    r.finish()?;
    let kernel = kernel.ok_or(CanonicalError::Layout("kernel family"))?;
    let captured: Vec<Captured> = captured
        .into_iter()
        .collect::<Option<_>>()
        .ok_or(CanonicalError::Layout("state type"))?;
    if header.now != kernel.now {
        return Err(CanonicalError::DateMismatch {
            header: header.now.as_millis(),
            state: kernel.now.as_millis(),
        });
    }
    let unknown = |id: &StableId| kernel.entities.binary_search(id).is_err();
    for c in &captured {
        if let Captured::Component(values) = c
            && values.iter().any(|(id, _)| unknown(id))
        {
            return Err(CanonicalError::NotCanonical(
                "component on an unknown StableId",
            ));
        }
    }
    Ok(Decoded {
        header,
        kernel,
        captured,
    })
}
