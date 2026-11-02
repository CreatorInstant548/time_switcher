// spec: attendus-mesurables.md §10.1 (M39a à M39f) ; attendus-mesurables.md §10 (méthode commune)
//! Measure of the data bounds: for each bound of [`Limits`] (M39a to M39f), the largest value
//! observed in a pack and its share of the bound. The CI warns from half of a bound, and the
//! bound is then raised by a design PR to 4 times the largest measured value.
//!
//! Pure functions: the host reads the directory and fills the two values only it knows (entries
//! listed, sub-directory depth).

use crate::pack::parses_within;
use crate::source::{Limits, SourceFile};

/// Largest values observed in a pack, one per bound of [`Limits`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    /// M39a: largest file, in bytes, and its relative path.
    pub file_bytes: (u64, String),
    /// M39b: total size of the files, in bytes.
    pub total_bytes: u64,
    /// M39c: directory entries the host listed, as counted against [`Limits::max_files`]
    /// (every entry, directories and ignored files included).
    pub entries: u64,
    /// M39d: cards in the pack.
    pub cards: u64,
    /// M39e: deepest RON nesting of a file, the smallest recursion limit that parses it, and
    /// its relative path.
    pub depth: (u64, String),
    /// M39f: deepest sub-directory level read, the content root being level 0.
    pub dir_depth: u64,
}

/// One bound and its largest observed value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// Number of the bound in the design (`M39a`…).
    pub id: &'static str,
    /// What is measured, in English.
    pub what: &'static str,
    /// Largest observed value.
    pub value: u64,
    /// The bound.
    pub bound: u64,
    /// Relative path of the file that gives the value, if the bound is per file.
    pub at: Option<String>,
}

impl Row {
    /// Share of the bound used, in tenths of a percent (rounded down); a zero bound counts as
    /// fully used as soon as the value is not zero.
    pub fn per_mille(&self) -> u64 {
        self.value
            .saturating_mul(1000)
            .checked_div(self.bound)
            .unwrap_or(if self.value == 0 { 0 } else { u64::MAX })
    }

    /// Whether the value is strictly over half of the bound: the CI warns then (A15, §10).
    pub fn over_half(&self) -> bool {
        self.value > self.bound / 2
    }
}

fn wide(n: usize) -> u64 {
    u64::try_from(n).unwrap_or(u64::MAX)
}

impl Usage {
    /// Measures what the files alone give: largest file, total size and deepest RON nesting.
    /// `entries`, `cards` and `dir_depth` stay at zero, for the host to fill.
    ///
    /// The nesting depth of a file is the smallest RON recursion limit under which it parses,
    /// found by bisection up to [`Limits::max_depth`]: the very quantity the bound refuses. A
    /// file that does not parse even at the bound counts as `max_depth + 1`.
    pub fn of_files(files: &[SourceFile], limits: &Limits) -> Self {
        let mut usage = Self::default();
        for file in files {
            let bytes = wide(file.bytes.len());
            usage.total_bytes = usage.total_bytes.saturating_add(bytes);
            if bytes > usage.file_bytes.0 || usage.file_bytes.1.is_empty() {
                usage.file_bytes = (bytes, file.path.clone());
            }
            let depth = wide(nesting_depth(&file.bytes, limits.max_depth));
            if depth > usage.depth.0 || usage.depth.1.is_empty() {
                usage.depth = (depth, file.path.clone());
            }
        }
        usage
    }

    /// The six bounds, in the order of the design, with their largest observed value.
    pub fn rows(&self, limits: &Limits) -> [Row; 6] {
        let row = |id, what, value, bound, at: Option<&String>| Row {
            id,
            what,
            value,
            bound,
            at: at.filter(|p| !p.is_empty()).cloned(),
        };
        [
            row(
                "M39a",
                "largest file (bytes)",
                self.file_bytes.0,
                limits.max_file_bytes,
                Some(&self.file_bytes.1),
            ),
            row(
                "M39b",
                "total size (bytes)",
                self.total_bytes,
                limits.max_total_bytes,
                None,
            ),
            row(
                "M39c",
                "directory entries",
                self.entries,
                wide(limits.max_files),
                None,
            ),
            row("M39d", "cards", self.cards, wide(limits.max_cards), None),
            row(
                "M39e",
                "RON nesting depth",
                self.depth.0,
                wide(limits.max_depth),
                Some(&self.depth.1),
            ),
            row(
                "M39f",
                "sub-directory depth",
                self.dir_depth,
                wide(limits.max_dir_depth),
                None,
            ),
        ]
    }
}

/// Smallest recursion limit in `0..=max` under which `bytes` parses as a card file, or
/// `max + 1` if none. Parsing succeeds for every limit above the smallest one (a larger limit
/// only removes failures), so a bisection finds it.
fn nesting_depth(bytes: &[u8], max: usize) -> usize {
    if !parses_within(bytes, max) {
        return max.saturating_add(1);
    }
    let (mut low, mut high) = (0_usize, max);
    while low < high {
        let mid = low.saturating_add(high.saturating_sub(low) / 2);
        if parses_within(bytes, mid) {
            high = mid;
        } else {
            low = mid.saturating_add(1);
        }
    }
    high
}
