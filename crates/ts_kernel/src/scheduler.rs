//! Ordonnanceur à événements datés : calendrier à roues à quatre niveaux, plus un tas.

use core::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};

use crate::SimTime;

/// Nombre de cases de chaque niveau : millisecondes d'une seconde, secondes d'une minute,
/// minutes d'une heure, heures d'un jour.
const RADIX: [i64; LEVELS] = [1000, 60, 60, 24];
/// Largeur d'une case de chaque niveau, en millisecondes.
const UNIT: [i64; LEVELS] = [1, 1000, 60_000, 3_600_000];
/// Largeur de l'ensemble d'un niveau, en millisecondes.
const SPAN: [i64; LEVELS] = [1000, 60_000, 3_600_000, SimTime::MILLIS_PER_DAY];
const LEVELS: usize = 4;

// spec: revues/architecture.md §8.1, §11.3
/// Événement daté sorti de l'ordonnanceur : sa date, son rang d'insertion et sa charge.
///
/// L'ordre total est `(at, seq)` : deux événements de même date sortent dans l'ordre où ils
/// ont été insérés.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheduled<E> {
    /// Date de l'événement.
    pub at: SimTime,
    /// Rang d'insertion, unique dans l'ordonnanceur.
    pub seq: u64,
    /// Charge de l'événement.
    pub event: E,
}

impl<E> Scheduled<E> {
    fn key(&self) -> (SimTime, u64) {
        (self.at, self.seq)
    }
}

/// Entrée du tas, ordonnée par `(at, seq)` seulement.
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

/// Un niveau du calendrier : ses cases, et un mot de 64 bits par groupe de 64 cases occupées.
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

    /// Insère en gardant la case triée par `seq` croissant.
    fn insert(&mut self, slot: usize, item: Scheduled<E>) {
        let q = &mut self.slots[slot];
        let pos = q.partition_point(|e| e.seq < item.seq);
        q.insert(pos, item);
        self.occupied[slot / 64] |= 1 << (slot % 64);
    }

    fn mark_if_empty(&mut self, slot: usize) {
        if self.slots[slot].is_empty() {
            self.occupied[slot / 64] &= !(1 << (slot % 64));
        }
    }

    /// Première case occupée d'indice `>= from`.
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
/// Ordonnanceur à événements datés (`Scheduler`).
///
/// - **Calendrier à roues** à quatre niveaux, alignés sur le jour du monde : milliseconde,
///   seconde, minute et heure du **jour courant**. Les dates des jours suivants attendent dans
///   un **tas**, qui garde aussi les dates déjà passées, servies en premier.
/// - **Ordre total** `(at, seq)` : `seq` compte les insertions, dans l'ordre ; il fait partie
///   de l'état du monde.
/// - Aucun tirage ni horloge murale : seule la date de jeu compte.
///
/// Invariant : un événement d'une case du niveau `k` a le même jour et les mêmes chiffres de
/// niveau supérieur à `k` que le curseur, et un chiffre de niveau `k` supérieur au sien
/// (égal ou supérieur au niveau 0). Le curseur n'avance que jusqu'à une date demandée.
#[derive(Debug)]
pub struct Scheduler<E> {
    /// Date jusqu'où le calendrier est déroulé ; jamais au-delà d'une date demandée.
    cursor: SimTime,
    levels: [Level<E>; LEVELS],
    /// Dates d'un jour postérieur au curseur, et dates antérieures au curseur.
    heap: BinaryHeap<Reverse<HeapEntry<E>>>,
    next_seq: u64,
    len: usize,
}

/// Jour de `t` et position dans ce jour, en millisecondes.
fn split(t: SimTime) -> (i64, i64) {
    let ms = t.as_millis();
    (
        ms.div_euclid(SimTime::MILLIS_PER_DAY),
        ms.rem_euclid(SimTime::MILLIS_PER_DAY),
    )
}

/// Chiffre de niveau `k` d'une position dans le jour.
fn digit(rem: i64, k: usize) -> usize {
    // Conversion exacte : le chiffre est dans `0..RADIX[k]`.
    ((rem / UNIT[k]) % RADIX[k]) as usize
}

impl<E> Scheduler<E> {
    /// Ordonnanceur vide, déroulé jusqu'à `start` (la date courante du monde).
    pub fn new(start: SimTime) -> Self {
        Self {
            cursor: start,
            levels: RADIX.map(|n| Level::new(n as usize)),
            heap: BinaryHeap::new(),
            next_seq: 0,
            len: 0,
        }
    }

    /// Nombre d'événements en attente.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Vrai si aucun événement n'attend.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Prochain rang d'insertion.
    pub fn next_seq(&self) -> u64 {
        self.next_seq
    }

    /// Programme `event` à la date `at` et rend son rang. Une date déjà passée est permise :
    /// l'événement sort au prochain appel, avant ceux de dates plus tardives.
    ///
    /// # Panics
    ///
    /// Si le compteur `seq` déborde (2^64 insertions).
    pub fn schedule(&mut self, at: SimTime, event: E) -> u64 {
        let seq = self.next_seq;
        let Some(next) = seq.checked_add(1) else {
            panic!("dépassement de seq");
        };
        self.next_seq = next;
        self.place(Scheduled { at, seq, event });
        self.len += 1;
        seq
    }

    /// Range un événement selon le curseur courant.
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

    /// Plus ancienne date en attente, sans rien dérouler.
    pub fn peek_next_time(&self) -> Option<SimTime> {
        let late = self.heap.peek().map(|Reverse(e)| e.0.at);
        if let Some(t) = late.filter(|&t| t < self.cursor) {
            return Some(t);
        }
        let (_, crem) = split(self.cursor);
        if let Some(s) = self.levels[0].next_occupied(digit(crem, 0)) {
            let base = self.cursor.as_millis() - crem % SPAN[0];
            return Some(SimTime::from_millis(base + s as i64));
        }
        for k in 1..LEVELS {
            if let Some(s) = self.levels[k].next_occupied(digit(crem, k) + 1) {
                return self.levels[k].slots[s].iter().map(|e| e.at).min();
            }
        }
        late
    }

    /// Sort le prochain événement si sa date est au plus `until`, dans l'ordre `(at, seq)`.
    pub fn next_due(&mut self, until: SimTime) -> Option<Scheduled<E>> {
        let item = self.advance(until)?;
        self.len -= 1;
        Some(item)
    }

    /// Sort, dans l'ordre `(at, seq)`, tous les événements de date au plus `until`.
    pub fn pop_due(&mut self, until: SimTime) -> Vec<Scheduled<E>> {
        let mut out = Vec::new();
        while let Some(item) = self.next_due(until) {
            out.push(item);
        }
        out
    }

    fn advance(&mut self, until: SimTime) -> Option<Scheduled<E>> {
        loop {
            // Dates passées d'abord : elles précèdent le curseur, donc tout le calendrier.
            if let Some(Reverse(top)) = self.heap.peek()
                && top.0.at < self.cursor
            {
                if top.0.at > until {
                    return None;
                }
                return self.heap.pop().map(|Reverse(e)| e.0);
            }
            let (_, crem) = split(self.cursor);
            let second_start = self.cursor.as_millis() - crem % SPAN[0];
            if let Some(s) = self.levels[0].next_occupied(digit(crem, 0)) {
                let t = SimTime::from_millis(second_start + s as i64);
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

    /// Avance le curseur au début de la prochaine case occupée d'un niveau supérieur, ou du
    /// prochain jour du tas, et en redistribue le contenu. Rend `Some(false)` s'il n'y a plus
    /// rien, `None` si la prochaine case commence après `until`.
    fn cascade(&mut self, until: SimTime, crem: i64) -> Option<bool> {
        for k in 1..LEVELS {
            let Some(s) = self.levels[k].next_occupied(digit(crem, k) + 1) else {
                continue;
            };
            let parent_start = self.cursor.as_millis() - crem % SPAN[k];
            let start = SimTime::from_millis(parent_start + s as i64 * UNIT[k]);
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
        while let Some(Reverse(top)) = self.heap.peek()
            && split(top.0.at).0 == day
        {
            if let Some(Reverse(e)) = self.heap.pop() {
                self.place(e.0);
            }
        }
        Some(true)
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

    /// Générateur de test : l'aléa à clé du noyau, compteur par appel.
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

    /// Date aléatoire : surtout proche, parfois des jours ou des années plus loin, parfois
    /// dans le passé du curseur, souvent sur une même milliseconde.
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

    /// 10^5 événements au hasard, insérés par vagues entre des sorties partielles : l'ordre de
    /// sortie est celui d'un tri simple par `(at, seq)`.
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
                let seq = s.schedule(t(at), inserted);
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
            assert_eq!(got, expected, "sortie jusqu'à {until}");
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
        // Mêmes dates, rangées d'abord à des niveaux différents puis redescendues.
        for (i, at) in [5000, 70, 5000, 3_600_000 + 1, 70, 5000, 3_600_000 + 1]
            .into_iter()
            .enumerate()
        {
            s.schedule(t(at), i);
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
            s.schedule(t(at), i);
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

    /// Événements à la frontière d'un tick de 50 ms : la date `until` est incluse, la
    /// suivante non.
    #[test]
    fn tick_boundaries_are_inclusive_of_until() {
        let mut s = Scheduler::new(SimTime::EPOCH);
        for at in [0, 49, 50, 99, 100, 999, 1000] {
            s.schedule(t(at), at);
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

    /// Une date déjà passée, ou égale à celle qui est en train de sortir, sort au prochain
    /// appel, dans l'ordre `(at, seq)`.
    #[test]
    fn past_and_current_dates_are_served_next() {
        let mut s = Scheduler::new(SimTime::EPOCH);
        s.schedule(t(500), 0);
        s.schedule(t(500), 1);
        s.schedule(t(800), 2);
        let first = s.next_due(t(600)).map(|e| e.event);
        assert_eq!(first, Some(0));
        s.schedule(t(500), 3);
        s.schedule(t(100), 4);
        s.schedule(t(-DAY), 5);
        assert_eq!(s.peek_next_time(), Some(t(-DAY)));
        let out: Vec<_> = s.pop_due(t(600)).into_iter().map(|e| e.event).collect();
        assert_eq!(out, [5, 4, 1, 3]);
        assert_eq!(s.pop_due(t(800)).len(), 1);
    }

    #[test]
    fn empty_scheduler_has_no_next() {
        let mut s: Scheduler<()> = Scheduler::new(t(-1));
        assert_eq!(s.peek_next_time(), None);
        assert!(s.next_due(t(i64::MAX)).is_none());
        assert_eq!(s.len(), 0);
    }
}
