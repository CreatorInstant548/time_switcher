//! Dated-event scheduler: a four-level timing wheel, plus a heap.

use core::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};

use crate::SimTime;
use crate::canonical::MAX_COUNTER;

// spec: revues/architecture.md §8.1, §12.1
/// Event refused by [`Scheduler::schedule`]: a snapshot could not hold it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleError {
    /// The date is beyond [`SimTime::MAX_SAVED_ABS_MILLIS`].
    DateOutOfRange(SimTime),
    /// The insertion counter reached 2^62, the bound of a snapshot.
    CounterExhausted,
}

impl core::fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DateOutOfRange(at) => {
                write!(
                    f,
                    "event date {} ms is out of the saved range",
                    at.as_millis()
                )
            }
            Self::CounterExhausted => f.write_str("event counter exhausted"),
        }
    }
}

impl std::error::Error for ScheduleError {}

/// Number of slots of each level: milliseconds of a second, seconds of a minute,
/// minutes of an hour, hours of a day.
const RADIX: [i64; LEVELS] = [1000, 60, 60, 24];
/// Width of a slot of each level, in milliseconds.
const UNIT: [i64; LEVELS] = [1, 1000, 60_000, 3_600_000];
/// Width of the whole of a level, in milliseconds.
const SPAN: [i64; LEVELS] = [1000, 60_000, 3_600_000, SimTime::MILLIS_PER_DAY];
const LEVELS: usize = 4;

// spec: revues/architecture.md §8.1, §11.3
/// Dated event leaving the scheduler: its date, its insertion rank and its payload.
///
/// The total order is `(at, seq)`: two events with the same date leave in the order in which
/// they were inserted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheduled<E> {
    /// Date of the event.
    pub at: SimTime,
    /// Insertion rank, unique within the scheduler.
    pub seq: u64,
    /// Payload of the event.
    pub event: E,
}

impl<E> Scheduled<E> {
    fn key(&self) -> (SimTime, u64) {
        (self.at, self.seq)
    }
}

/// Heap entry, ordered by `(at, seq)` only.
#[derive(Debug)]
struct HeapEntry<E>(Scheduled<E>);

impl<E> PartialEq for HeapEntry<E> {
    fn eq(&self, other: &Self) -> bool {
        self.0.key() == other.0.key()
    }
}
impl<E> Eq for HeapEntry<E> {}
impl<E> PartialOrd for HeapEntry<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<E> Ord for HeapEntry<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.key().cmp(&other.0.key())
    }
}

#[cfg(test)]
std::thread_local! {
    /// Test-only count of elements shifted by sorted inserts (complexity regression test).
    static SHIFTS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// One level of the wheel: its slots, and one 64-bit word per group of 64 occupied slots.
#[derive(Debug)]
struct Level<E> {
    slots: Vec<VecDeque<Scheduled<E>>>,
    occupied: Vec<u64>,
}

impl<E> Level<E> {
    fn new(len: usize) -> Self {
        Self {
            slots: (0..len).map(|_| VecDeque::new()).collect(),
            occupied: vec![0; len.div_ceil(64)],
        }
    }

    /// Inserts while keeping the slot sorted by increasing `seq`.
    ///
    /// Callers insert in increasing `seq` order (new events carry the largest `seq`; cascaded
    /// batches are sorted by `seq` first), so this is an append. The sorted insert stays as
    /// a safety net.
    fn insert(&mut self, slot: usize, item: Scheduled<E>) {
        let q = &mut self.slots[slot];
        let pos = q.partition_point(|e| e.seq < item.seq);
        #[cfg(test)]
        SHIFTS.with(|s| s.set(s.get() + (q.len() - pos)));
        q.insert(pos, item);
        self.occupied[slot / 64] |= 1 << (slot % 64);
    }

    fn mark_if_empty(&mut self, slot: usize) {
        if self.slots[slot].is_empty() {
            self.occupied[slot / 64] &= !(1 << (slot % 64));
        }
    }

    /// First occupied slot with index `>= from`.
    fn next_occupied(&self, from: usize) -> Option<usize> {
        let mut word = from / 64;
        let mut bits = *self.occupied.get(word)? & (!0_u64 << (from % 64));
        loop {
            if bits != 0 {
                return Some(word * 64 + bits.trailing_zeros() as usize);
            }
            word += 1;
            bits = *self.occupied.get(word)?;
        }
    }
}

// spec: revues/architecture.md §8.1, §6.5 ; §9.2
/// Dated-event scheduler (`Scheduler`).
///
/// - **Timing wheel** with four levels, aligned on the world day: millisecond, second,
///   minute and hour of the **current day**. Dates of the following days wait in a
///   **heap**, which also keeps the dates already past, served first.
/// - **Total order** `(at, seq)`: `seq` counts the insertions, in order; it is part of the
///   state of the world.
/// - No draw and no wall clock: only the game date counts.
///
/// Invariant: an event in a slot of level `k` has the same day and the same digits above
/// level `k` as the cursor, and a level-`k` digit greater than the cursor's (equal or
/// greater at level 0). The cursor only advances up to a requested date.
///
/// Only the kernel builds or restores one (#123). Witness, then the two refusals:
///
/// ```
/// fn pending(q: &ts_kernel::Scheduler<()>) -> usize {
///     q.len()
/// }
/// let _ = ts_kernel::SimTime::EPOCH;
/// let _: Vec<ts_kernel::Scheduled<()>> = Vec::new();
/// ```
///
/// ```compile_fail,E0624
/// let _ = ts_kernel::Scheduler::<()>::new(ts_kernel::SimTime::EPOCH);
/// ```
///
/// ```compile_fail,E0624
/// let _ = ts_kernel::Scheduler::<()>::restore(ts_kernel::SimTime::EPOCH, 0, Vec::new());
/// ```
#[derive(Debug)]
pub struct Scheduler<E> {
    /// Date up to which the wheel is unrolled; never beyond a requested date.
    cursor: SimTime,
    levels: [Level<E>; LEVELS],
    /// Dates of a day after the cursor, and dates before the cursor.
    heap: BinaryHeap<Reverse<HeapEntry<E>>>,
    next_seq: u64,
    len: usize,
}

/// Day of `t` and position within that day, in milliseconds.
fn split(t: SimTime) -> (i64, i64) {
    let ms = t.as_millis();
    (
        ms.div_euclid(SimTime::MILLIS_PER_DAY),
        ms.rem_euclid(SimTime::MILLIS_PER_DAY),
    )
}

/// Level-`k` digit of a position within the day.
fn digit(rem: i64, k: usize) -> usize {
    // Exact conversion: the digit is in `0..RADIX[k]`.
    ((rem / UNIT[k]) % RADIX[k]) as usize
}

impl<E> Scheduler<E> {
    /// Empty scheduler, unrolled up to `start` (the current date of the world). **Kernel
    /// only** (#123): a rule holding [`crate::EventQueue`] cannot build a new scheduler to
    /// put in its place, which would reset its counter.
    pub(crate) fn new(start: SimTime) -> Self {
        Self {
            cursor: start,
            levels: RADIX.map(|n| Level::new(n as usize)),
            heap: BinaryHeap::new(),
            next_seq: 0,
            len: 0,
        }
    }

    /// Number of pending events.
    pub fn len(&self) -> usize {
        self.len
    }

    /// True if no event is waiting.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Next insertion rank.
    pub fn next_seq(&self) -> u64 {
        self.next_seq
    }

    // spec: revues/architecture.md §8.1, §12.1 ; carnet architecture-securite.md §10
    /// Schedules `event` at date `at` and returns its rank. A date already past is allowed:
    /// the event leaves on the next call, before those with later dates.
    ///
    /// # Errors
    ///
    /// Refuses, without scheduling, what a snapshot could not hold, so that every save can be
    /// read back: a date beyond [`SimTime::MAX_SAVED_ABS_MILLIS`] (a sentinel such as
    /// `i64::MAX` included), or a 2^62nd insertion.
    pub fn schedule(&mut self, at: SimTime, event: E) -> Result<u64, ScheduleError> {
        if !at.is_savable() {
            return Err(ScheduleError::DateOutOfRange(at));
        }
        let seq = self.next_seq;
        if seq >= MAX_COUNTER {
            return Err(ScheduleError::CounterExhausted);
        }
        self.next_seq = seq + 1;
        self.place(Scheduled { at, seq, event });
        self.len += 1;
        Ok(seq)
    }

    /// Places an event according to the current cursor.
    fn place(&mut self, item: Scheduled<E>) {
        let (day, rem) = split(item.at);
        let (cday, crem) = split(self.cursor);
        if item.at < self.cursor || day != cday {
            self.heap.push(Reverse(HeapEntry(item)));
            return;
        }
        let k = (1..LEVELS)
            .rev()
            .find(|&k| digit(rem, k) != digit(crem, k))
            .unwrap_or(0);
        self.levels[k].insert(digit(rem, k), item);
    }

    // spec: revues/architecture.md §12.1, §11.3
    /// Pending events, in `(at, seq)` order, for the fingerprint and the snapshot.
    /// The cursor is not part of it: it changes no output.
    pub fn pending(&self) -> Vec<&Scheduled<E>> {
        let mut out: Vec<_> = self
            .levels
            .iter()
            .flat_map(|l| l.slots.iter().flatten())
            .chain(self.heap.iter().map(|Reverse(e)| &e.0))
            .collect();
        out.sort_unstable_by_key(|e| e.key());
        out
    }

    // spec: revues/architecture.md §12.1 ; carnet architecture-securite.md §9.10 L5, §10
    /// Scheduler restored at date `now`, with its counter and its events, ranks included.
    /// **Kernel only** (#123): its one caller is the restore of a snapshot.
    ///
    /// `None` if a rank is duplicated or is not below `next_seq`, or if a value is beyond what
    /// [`Scheduler::schedule`] would write (#123, same family as #113): `now` or an event date
    /// not savable ([`SimTime::is_savable`]), or `next_seq` above 2^62. Every scheduler it
    /// returns therefore saves to bytes that read back.
    pub(crate) fn restore(now: SimTime, next_seq: u64, items: Vec<Scheduled<E>>) -> Option<Self> {
        if !now.is_savable() || next_seq > MAX_COUNTER {
            return None;
        }
        let mut seqs = Vec::with_capacity(items.len());
        for e in &items {
            if !e.at.is_savable() {
                return None;
            }
            seqs.push(e.seq);
        }
        seqs.sort_unstable();
        if seqs.windows(2).any(|w| w[0] == w[1]) || seqs.last().is_some_and(|&s| s >= next_seq) {
            return None;
        }
        let mut s = Self::new(now);
        s.next_seq = next_seq;
        s.len = items.len();
        // `seq` order keeps every slot insert an append (see `cascade`).
        let mut items = items;
        items.sort_unstable_by_key(|e| e.seq);
        for item in items {
            s.place(item);
        }
        Some(s)
    }

    /// Oldest pending date, without unrolling anything.
    pub fn peek_next_time(&self) -> Option<SimTime> {
        let late = self.heap.peek().map(|Reverse(e)| e.0.at);
        if let Some(t) = late.filter(|&t| t < self.cursor) {
            return Some(t);
        }
        let (_, crem) = split(self.cursor);
        if let Some(s) = self.levels[0].next_occupied(digit(crem, 0)) {
            return Some(self.slot_start(crem, 0, s));
        }
        for k in 1..LEVELS {
            if let Some(s) = self.levels[k].next_occupied(digit(crem, k) + 1) {
                return self.levels[k].slots[s].iter().map(|e| e.at).min();
            }
        }
        late
    }

    /// Pops the next event if its date is at most `until`, in `(at, seq)` order.
    pub fn next_due(&mut self, until: SimTime) -> Option<Scheduled<E>> {
        let item = self.advance(until)?;
        self.len -= 1;
        Some(item)
    }

    /// Pops, in `(at, seq)` order, all the events with a date at most `until`.
    pub fn pop_due(&mut self, until: SimTime) -> Vec<Scheduled<E>> {
        let mut out = Vec::new();
        while let Some(item) = self.next_due(until) {
            out.push(item);
        }
        out
    }

    fn advance(&mut self, until: SimTime) -> Option<Scheduled<E>> {
        loop {
            // Past dates first: they precede the cursor, hence the whole wheel.
            if let Some(Reverse(top)) = self.heap.peek()
                && top.0.at < self.cursor
            {
                if top.0.at > until {
                    return None;
                }
                return self.heap.pop().map(|Reverse(e)| e.0);
            }
            let (_, crem) = split(self.cursor);
            if let Some(s) = self.levels[0].next_occupied(digit(crem, 0)) {
                let t = self.slot_start(crem, 0, s);
                if t > until {
                    return None;
                }
                self.cursor = t;
                let item = self.levels[0].slots[s].pop_front();
                self.levels[0].mark_if_empty(s);
                return item;
            }
            if !self.cascade(until, crem)? {
                return None;
            }
        }
    }

    /// Advances the cursor to the start of the next occupied slot of a higher level, or of the
    /// next day in the heap, and redistributes its content. Returns `Some(false)` if there is
    /// nothing left, `None` if the next slot starts after `until`.
    fn cascade(&mut self, until: SimTime, crem: i64) -> Option<bool> {
        for k in 1..LEVELS {
            let Some(s) = self.levels[k].next_occupied(digit(crem, k) + 1) else {
                continue;
            };
            let start = self.slot_start(crem, k, s);
            if start > until {
                return None;
            }
            self.cursor = start;
            let items = core::mem::take(&mut self.levels[k].slots[s]);
            self.levels[k].mark_if_empty(s);
            for item in items {
                self.place(item);
            }
            return Some(true);
        }
        let Some(Reverse(top)) = self.heap.peek() else {
            return Some(false);
        };
        let (day, _) = split(top.0.at);
        let start = SimTime::from_millis(day * SimTime::MILLIS_PER_DAY);
        if start > until {
            return None;
        }
        self.cursor = start;
        let mut batch = Vec::new();
        while let Some(Reverse(top)) = self.heap.peek()
            && split(top.0.at).0 == day
        {
            if let Some(Reverse(e)) = self.heap.pop() {
                batch.push(e.0);
            }
        }
        // The heap yields `(at, seq)` order; placing in `seq` order makes every slot insert
        // an append, so a day boundary costs O(n log n), not O(n²) (T6a review I1).
        batch.sort_unstable_by_key(|e| e.seq);
        for item in batch {
            self.place(item);
        }
        Some(true)
    }

    /// Start of slot `s` of level `k`, in the level-`k` span that holds the cursor
    /// (`crem` is the cursor's position in its day). Computed as an offset from the cursor
    /// (at least the slot's earliest pending date), so it never overflows near `i64::MIN`
    /// (T6a review m1).
    fn slot_start(&self, crem: i64, k: usize, s: usize) -> SimTime {
        // Exact conversion: `s` is below `RADIX[k]`.
        let offset = s as i64 * UNIT[k] - crem % SPAN[k];
        SimTime::from_millis(self.cursor.as_millis() + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StableId;
    use crate::rng::{Stream, StreamKeys, WorldKey, draw};
    use std::collections::BTreeMap;

    const DAY: i64 = SimTime::MILLIS_PER_DAY;

    fn t(ms: i64) -> SimTime {
        SimTime::from_millis(ms)
    }

    /// Test generator: the core's keyed randomness, a counter per call.
    struct Draws {
        keys: StreamKeys,
        n: u64,
    }

    impl Draws {
        fn new(seed: u64) -> Self {
            let world = WorldKey::insecure_from_u64_for_tests(seed);
            Self {
                keys: StreamKeys::derive(&world),
                n: 0,
            }
        }

        fn below(&mut self, bound: u64) -> u64 {
            self.n += 1;
            let subject = StableId::from_snapshot_raw(0);
            draw(self.keys.get(Stream::Population), subject, self.n, 7) % bound
        }
    }

    /// Random date: mostly near, sometimes days or years away, sometimes in the cursor's
    /// past, often on the same millisecond.
    fn random_time(d: &mut Draws, now: i64) -> i64 {
        match d.below(8) {
            0 => now + d.below(50) as i64,
            1 => now + d.below(1000) as i64 * 50,
            2 => now + d.below(4 * 3_600_000) as i64,
            3 => now + d.below(40 * DAY as u64) as i64,
            4 => now + d.below(3) as i64 * 365 * DAY,
            5 => now - d.below(5000) as i64,
            6 => now - 20 * 365 * DAY + d.below(DAY as u64) as i64,
            _ => (now / 1000 + d.below(3) as i64) * 1000,
        }
    }

    /// 10^5 random events, inserted in waves between partial pops: the pop order is that of a
    /// simple sort by `(at, seq)`.
    #[test]
    fn order_matches_sorted_reference_on_100k_events() {
        let mut d = Draws::new(42);
        let start = -3 * DAY + 123;
        let mut s = Scheduler::new(t(start));
        let mut reference: BTreeMap<(i64, u64), u32> = BTreeMap::new();
        let mut now = start;
        let mut inserted = 0_u32;
        let mut popped = 0_usize;
        while inserted < 100_000 || !reference.is_empty() {
            let wave = d.below(400).min(u64::from(100_000 - inserted));
            for _ in 0..wave {
                let at = random_time(&mut d, now);
                let seq = s.schedule(t(at), inserted).unwrap();
                reference.insert((at, seq), inserted);
                inserted += 1;
            }
            assert_eq!(s.len(), reference.len());
            assert_eq!(
                s.peek_next_time(),
                reference.keys().next().map(|&(at, _)| t(at))
            );
            let step = match d.below(4) {
                0 => d.below(50) as i64,
                1 => d.below(60_000) as i64,
                2 => d.below(2 * DAY as u64) as i64,
                _ => d.below(200 * DAY as u64) as i64,
            };
            let until = if inserted >= 100_000 {
                i64::MAX / 2
            } else {
                now + step
            };
            let got = s.pop_due(t(until));
            let expected: Vec<_> = reference
                .range(..=(until, u64::MAX))
                .map(|(&k, &v)| (k, v))
                .collect();
            for &(k, _) in &expected {
                reference.remove(&k);
            }
            let got: Vec<_> = got
                .into_iter()
                .map(|e| ((e.at.as_millis(), e.seq), e.event))
                .collect();
            assert_eq!(got, expected, "output up to {until}");
            popped += got.len();
            now = until.min(now + 400 * DAY);
        }
        assert_eq!(popped, 100_000);
        assert!(s.is_empty());
        assert_eq!(s.next_seq(), 100_000);
    }

    #[test]
    fn equal_dates_leave_in_insertion_order() {
        let mut s = Scheduler::new(SimTime::EPOCH);
        // Same dates, first placed at different levels and then cascaded back down.
        for (i, at) in [5000, 70, 5000, 3_600_000 + 1, 70, 5000, 3_600_000 + 1]
            .into_iter()
            .enumerate()
        {
            s.schedule(t(at), i).unwrap();
        }
        let out: Vec<_> = s
            .pop_due(t(DAY))
            .into_iter()
            .map(|e| (e.at.as_millis(), e.event))
            .collect();
        assert_eq!(
            out,
            [
                (70, 1),
                (70, 4),
                (5000, 0),
                (5000, 2),
                (5000, 5),
                (3_600_001, 3),
                (3_600_001, 6)
            ]
        );
    }

    #[test]
    fn far_dates_wait_in_heap_and_come_back_in_order() {
        let mut s = Scheduler::new(SimTime::EPOCH);
        let far = [10 * 365 * DAY, DAY, 2 * DAY - 1, DAY, 400 * DAY + 7];
        for (i, &at) in far.iter().enumerate() {
            s.schedule(t(at), i).unwrap();
        }
        assert_eq!(s.peek_next_time(), Some(t(DAY)));
        assert!(s.pop_due(t(DAY - 1)).is_empty());
        let out: Vec<_> = s
            .pop_due(t(i64::MAX))
            .into_iter()
            .map(|e| e.event)
            .collect();
        assert_eq!(out, [1, 3, 2, 4, 0]);
    }

    /// Events at the boundary of a 50 ms tick: the date `until` is included, the next one
    /// is not.
    #[test]
    fn tick_boundaries_are_inclusive_of_until() {
        let mut s = Scheduler::new(SimTime::EPOCH);
        for at in [0, 49, 50, 99, 100, 999, 1000] {
            s.schedule(t(at), at).unwrap();
        }
        let tick = |s: &mut Scheduler<i64>, start: i64| -> Vec<i64> {
            s.pop_due(t(start + 49))
                .into_iter()
                .map(|e| e.event)
                .collect()
        };
        assert_eq!(tick(&mut s, 0), [0, 49]);
        assert_eq!(tick(&mut s, 50), [50, 99]);
        assert_eq!(tick(&mut s, 100), [100]);
        assert_eq!(s.peek_next_time(), Some(t(999)));
        assert_eq!(tick(&mut s, 950), [999]);
        assert_eq!(tick(&mut s, 1000), [1000]);
        assert!(s.is_empty());
    }

    /// A date already past, or equal to the one being popped, leaves on the next call, in
    /// `(at, seq)` order.
    #[test]
    fn past_and_current_dates_are_served_next() {
        let mut s = Scheduler::new(SimTime::EPOCH);
        s.schedule(t(500), 0).unwrap();
        s.schedule(t(500), 1).unwrap();
        s.schedule(t(800), 2).unwrap();
        let first = s.next_due(t(600)).map(|e| e.event);
        assert_eq!(first, Some(0));
        s.schedule(t(500), 3).unwrap();
        s.schedule(t(100), 4).unwrap();
        s.schedule(t(-DAY), 5).unwrap();
        assert_eq!(s.peek_next_time(), Some(t(-DAY)));
        let out: Vec<_> = s.pop_due(t(600)).into_iter().map(|e| e.event).collect();
        assert_eq!(out, [5, 4, 1, 3]);
        assert_eq!(s.pop_due(t(800)).len(), 1);
    }

    /// A scheduler restored at another cursor date pops the same events, in the same order,
    /// and continues the numbering.
    #[test]
    fn restore_keeps_order_and_counter() {
        let mut d = Draws::new(9);
        let mut a = Scheduler::new(SimTime::EPOCH);
        for i in 0..2000 {
            a.schedule(t(random_time(&mut d, 0)), i).unwrap();
        }
        a.pop_due(t(3_600_000));
        let items: Vec<_> = a.pending().into_iter().cloned().collect();
        assert!(items.windows(2).all(|w| w[0].key() < w[1].key()));
        let mut b = Scheduler::restore(t(3_600_000), a.next_seq(), items).unwrap();
        assert_eq!(b.len(), a.len());
        assert_eq!(a.schedule(t(5), 1), b.schedule(t(5), 1));
        assert_eq!(a.pop_due(t(i64::MAX / 2)), b.pop_due(t(i64::MAX / 2)));
        let dup = vec![
            Scheduled {
                at: t(1),
                seq: 0,
                event: (),
            },
            Scheduled {
                at: t(2),
                seq: 0,
                event: (),
            },
        ];
        assert!(Scheduler::restore(SimTime::EPOCH, 5, dup).is_none());
        let late = vec![Scheduled {
            at: t(1),
            seq: 5,
            event: (),
        }];
        assert!(Scheduler::restore(SimTime::EPOCH, 5, late).is_none());
    }

    /// #123: a restore is bounded as `schedule` is, at each bound.
    #[test]
    fn restore_is_bounded_like_schedule() {
        const M: i64 = SimTime::MAX_SAVED_ABS_MILLIS;
        let at = |ms: i64| {
            vec![Scheduled {
                at: t(ms),
                seq: 0,
                event: (),
            }]
        };
        for ms in [-M, M] {
            assert!(Scheduler::restore(t(ms), 1, at(ms)).is_some(), "{ms}");
        }
        for ms in [i64::MIN, -M - 1, M + 1, i64::MAX] {
            assert!(Scheduler::restore(t(0), 1, at(ms)).is_none(), "event {ms}");
            assert!(
                Scheduler::<()>::restore(t(ms), 0, Vec::new()).is_none(),
                "now {ms}"
            );
        }
        assert!(Scheduler::<()>::restore(t(0), MAX_COUNTER, Vec::new()).is_some());
        for next_seq in [MAX_COUNTER + 1, u64::MAX] {
            assert!(Scheduler::<()>::restore(t(0), next_seq, Vec::new()).is_none());
        }
    }

    /// T6a review I1: many next-day events in one hour, scheduled out of date order, cascade
    /// at midnight without shifting slot contents (a bound on operations, not on time).
    #[test]
    fn day_boundary_cascade_is_not_quadratic() {
        let mut d = Draws::new(5);
        let mut s = Scheduler::new(SimTime::EPOCH);
        let n = 20_000;
        for i in 0..n {
            let at = DAY + 8 * 3_600_000 + d.below(3_600_000) as i64;
            s.schedule(t(at), i).unwrap();
        }
        SHIFTS.with(|c| c.set(0));
        let out = s.pop_due(t(2 * DAY));
        assert_eq!(out.len(), n);
        assert!(out.windows(2).all(|w| w[0].key() < w[1].key()));
        let shifts = SHIFTS.with(core::cell::Cell::get);
        assert_eq!(shifts, 0, "{shifts} elements shifted");
    }

    /// T6a review m1: dates at the saved bounds and cursors at the bounds of `i64` never
    /// overflow, and the output still matches a sorted reference.
    #[test]
    fn extreme_dates_do_not_overflow() {
        const M: i64 = SimTime::MAX_SAVED_ABS_MILLIS;
        let marks = [
            -M,
            -M + 1,
            -M + 999,
            -M + DAY,
            -DAY,
            0,
            DAY,
            M - DAY,
            M - 1,
            M,
        ];
        for &start in &[i64::MIN, -M, -1, 0, M, i64::MAX - 1] {
            let mut s = Scheduler::new(t(start));
            let mut reference = BTreeMap::new();
            for (i, &at) in marks.iter().chain(marks.iter().rev()).enumerate() {
                let seq = s.schedule(t(at), i).unwrap();
                reference.insert((at, seq), i);
            }
            for &until in &marks {
                assert_eq!(
                    s.peek_next_time(),
                    reference.keys().next().map(|&(at, _)| t(at))
                );
                let got: Vec<_> = s.pop_due(t(until)).into_iter().map(|e| e.event).collect();
                let expected: Vec<_> = reference
                    .range(..=(until, u64::MAX))
                    .map(|(_, &v)| v)
                    .collect();
                reference.retain(|&(at, _), _| at > until);
                assert_eq!(got, expected, "start {start}, until {until}");
            }
            assert!(s.is_empty());
        }
    }

    /// #31: a date or a counter that a snapshot could not hold is refused, and nothing is
    /// scheduled.
    #[test]
    fn unsaveable_events_are_refused() {
        const M: i64 = SimTime::MAX_SAVED_ABS_MILLIS;
        let mut s = Scheduler::new(SimTime::EPOCH);
        for at in [i64::MIN, -M - 1, M + 1, i64::MAX] {
            assert_eq!(
                s.schedule(t(at), ()),
                Err(ScheduleError::DateOutOfRange(t(at)))
            );
        }
        assert_eq!((s.len(), s.next_seq()), (0, 0));
        let mut s = Scheduler::restore(SimTime::EPOCH, MAX_COUNTER - 1, Vec::new()).unwrap();
        assert_eq!(s.schedule(t(M), ()), Ok(MAX_COUNTER - 1));
        assert_eq!(s.schedule(t(0), ()), Err(ScheduleError::CounterExhausted));
        assert_eq!((s.len(), s.next_seq()), (1, MAX_COUNTER));
    }

    #[test]
    fn empty_scheduler_has_no_next() {
        let mut s: Scheduler<()> = Scheduler::new(t(-1));
        assert_eq!(s.peek_next_time(), None);
        assert!(s.next_due(t(i64::MAX)).is_none());
        assert_eq!(s.len(), 0);
    }
}
