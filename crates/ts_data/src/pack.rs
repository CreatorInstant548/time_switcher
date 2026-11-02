//! The compiled content pack: validated cards sorted by identifier, canonical bytes,
//! fingerprint.

use core::fmt;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use crate::card::{Card, is_valid_card_id, name_fault};
use crate::error::{Code, Diagnostic, LoadErrors, Sink};
use crate::scan::element_starts;
use crate::source::{Limits, SourceFile, is_relative_path};

// spec: revues/developpement-agents.md A4 ; carnet architecture-securite.md §9.13
/// Version of the canonical encoding, written first. Changing the encoding bumps it and
/// changes the identity of every pack.
pub const FORMAT_VERSION: u32 = 1;
/// `derive_key` context of the pack fingerprint.
const KEY_CONTEXT: &str = "time_switcher content pack v1";

/// BLAKE3 fingerprint of the compiled pack: the identity of a content pack. It depends on
/// the canonical bytes only, never on line endings, file order, file names or local paths.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fingerprint([u8; 32]);

impl Fingerprint {
    /// The 32 bytes.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|b| write!(f, "{b:02x}"))
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fingerprint({self})")
    }
}

/// A validated, compiled set of cards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentPack {
    cards: Vec<Card>,
    canonical: Vec<u8>,
    fingerprint: Fingerprint,
}

/// A parsed card with where it came from: file and rank in the file's list.
struct Located {
    card: Card,
    file: usize,
    rank: usize,
}

/// Positions of the cards of each file, scanned only when an error needs one.
struct Positions<'a> {
    files: &'a [SourceFile],
    cache: BTreeMap<usize, Vec<(u32, u32)>>,
}

impl Positions<'_> {
    /// `(file path, (line, column))` of a card; `(1, 1)` if the scanner could not find it.
    fn of(&mut self, l: &Located) -> (String, (u32, u32)) {
        let Some(file) = self.files.get(l.file) else {
            return (String::new(), (0, 0));
        };
        let starts = self
            .cache
            .entry(l.file)
            .or_insert_with(|| element_starts(&String::from_utf8_lossy(strip_bom(&file.bytes))));
        (
            file.path.clone(),
            starts.get(l.rank).copied().unwrap_or((1, 1)),
        )
    }
}

fn strip_bom(bytes: &[u8]) -> &[u8] {
    bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes)
}

impl ContentPack {
    /// Parses, validates and compiles `files`. Never panics on any input (a pack is untrusted:
    /// a solo player can edit it); every problem is a [`Diagnostic`].
    ///
    /// Files are processed in path order, so the errors do not depend on the reading order.
    /// Identifiers and references are checked only when every file parsed and every card is
    /// well formed, so a broken file does not cause a flood of broken references.
    pub fn build(mut files: Vec<SourceFile>, limits: &Limits) -> Result<Self, LoadErrors> {
        let total_bytes = files.iter().fold(0_u64, |sum, f| {
            sum.saturating_add(u64::try_from(f.bytes.len()).unwrap_or(u64::MAX))
        });
        if files.len() > limits.max_files {
            return Err(one(Diagnostic::new(Code::TooManyFiles, "", (0, 0), "")));
        }
        if total_bytes > limits.max_total_bytes {
            return Err(one(Diagnostic::new(Code::TotalTooLarge, "", (0, 0), "")));
        }
        files.sort_by(|a, b| a.path.cmp(&b.path));
        let mut sink = Sink::default();
        let mut located = Vec::new();
        let mut total_cards: usize = 0;
        for (index, file) in files.iter().enumerate() {
            if !is_relative_path(&file.path) {
                sink.push_with(|| Diagnostic::new(Code::BadPath, "", (0, 0), ""));
                continue;
            }
            if u64::try_from(file.bytes.len()).map_or(true, |n| n > limits.max_file_bytes) {
                sink.push_with(|| Diagnostic::new(Code::FileTooLarge, &file.path, (0, 0), ""));
                continue;
            }
            match parse_file(file, limits) {
                Ok(cards) => {
                    total_cards = total_cards.saturating_add(cards.len());
                    if total_cards > limits.max_cards {
                        return Err(one(Diagnostic::new(
                            Code::TooManyCards,
                            &file.path,
                            (0, 0),
                            "",
                        )));
                    }
                    located.extend(cards.into_iter().enumerate().map(|(rank, card)| Located {
                        card,
                        file: index,
                        rank,
                    }));
                }
                Err(d) => sink.push_with(|| d),
            }
        }
        let mut positions = Positions {
            files: &files,
            cache: BTreeMap::new(),
        };
        let by_id = validate(located, &mut positions, &mut sink);
        if let Some(errors) = sink.into_errors() {
            return Err(errors);
        }
        let cards: Vec<Card> = by_id.into_values().map(|l| l.card).collect();
        let Some(canonical) = encode(&cards) else {
            return Err(one(Diagnostic::new(Code::EncodingOverflow, "", (0, 0), "")));
        };
        let fingerprint = fingerprint_of(&canonical);
        Ok(Self {
            cards,
            canonical,
            fingerprint,
        })
    }

    /// Cards, sorted by identifier.
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// The identity of the pack.
    pub fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// The canonical bytes the fingerprint is computed on.
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }
}

fn one(d: Diagnostic) -> LoadErrors {
    LoadErrors::single(d)
}

fn parse_file(file: &SourceFile, limits: &Limits) -> Result<Vec<Card>, Diagnostic> {
    let at = |e: &ron::error::SpannedError| {
        (
            u32::try_from(e.span.start.line).unwrap_or(u32::MAX),
            u32::try_from(e.span.start.col).unwrap_or(u32::MAX),
        )
    };
    parse_cards(&file.bytes, limits.max_depth).map_err(|e| {
        use ron::error::Error as E;
        let (code, detail) = match &e.code {
            E::ExceededRecursionLimit => (Code::RecursionLimit, String::new()),
            E::Utf8Error(_) => (Code::NotUtf8, String::new()),
            E::MissingStructField { field, .. } => (Code::MissingField, (*field).to_owned()),
            E::NoSuchStructField { found, .. } => (Code::UnknownField, found.clone()),
            E::NoSuchEnumVariant { found, .. } => (Code::UnknownKind, found.clone()),
            _ => (Code::RonSyntax, String::new()),
        };
        Diagnostic::new(code, &file.path, at(&e), &detail)
    })
}

/// Parses a card file under the RON recursion limit `depth`. A byte order mark (some editors
/// write one) is not part of the RON text.
fn parse_cards(bytes: &[u8], depth: usize) -> Result<Vec<Card>, ron::error::SpannedError> {
    ron::Options::default()
        .with_recursion_limit(depth)
        .from_bytes::<Vec<Card>>(strip_bom(bytes))
}

/// Whether `bytes` parses as a card file under the RON recursion limit `depth` (measure of
/// M39e, [`crate::Usage`]).
pub(crate) fn parses_within(bytes: &[u8], depth: usize) -> bool {
    parse_cards(bytes, depth).is_ok()
}

/// Field rules (identifier grammar, namespace of the kind, name), identifier uniqueness
/// across all kinds, then reference resolution. Returns the cards by id.
/// References are resolved only if nothing failed before, so rejected cards cannot cause
/// spurious broken references.
fn validate(
    located: Vec<Located>,
    positions: &mut Positions<'_>,
    sink: &mut Sink,
) -> BTreeMap<String, Located> {
    let mut report = |sink: &mut Sink, code: Code, l: &Located, detail: &str| {
        sink.push_with(|| {
            let (file, at) = positions.of(l);
            Diagnostic::new(code, &file, at, detail)
        });
    };
    let mut by_id: BTreeMap<String, Located> = BTreeMap::new();
    for l in located {
        let id = l.card.id().to_owned();
        // spec: attendus-mesurables.md §10.2 (M40a à M40c, M40e)
        if !is_valid_card_id(&id) {
            report(sink, Code::InvalidId, &l, &id);
        } else if !id
            .strip_prefix(l.card.namespace())
            .is_some_and(|rest| rest.starts_with('.'))
        {
            report(sink, Code::WrongNamespace, &l, &id);
        } else if let Some(fault) = name_fault(l.card.name()) {
            report(sink, fault, &l, &id);
        } else {
            match by_id.entry(id) {
                Entry::Occupied(taken) => report(sink, Code::DuplicateId, &l, taken.key()),
                Entry::Vacant(slot) => {
                    slot.insert(l);
                }
            }
        }
    }
    if sink.is_empty() {
        for l in by_id.values() {
            for target in l.card.references() {
                if !by_id.get(target).is_some_and(|t| t.card.is_item()) {
                    report(sink, Code::BrokenReference, l, target);
                }
            }
        }
    }
    by_id
}

/// Canonical encoding: `u32` version, `u32` card count, then per card (sorted by id) a one-byte
/// kind tag and its fields. Integers are little-endian; a string is a `u32` byte length then
/// its UTF-8 bytes; a list is a `u32` count then its elements in authored order. `None` if a
/// length does not fit a `u32`: a length is never saturated, so the encoding stays injective
/// whatever the [`Limits`].
fn encode(cards: &[Card]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    put_u32(&mut out, cards.len())?;
    for card in cards {
        match card {
            Card::Item { id, name } => {
                out.push(0x01);
                put_str(&mut out, id)?;
                put_str(&mut out, name)?;
            }
            Card::Activity { id, name, items } => {
                out.push(0x02);
                put_str(&mut out, id)?;
                put_str(&mut out, name)?;
                put_u32(&mut out, items.len())?;
                items.iter().try_for_each(|i| put_str(&mut out, i))?;
            }
        }
    }
    Some(out)
}

pub(crate) fn put_u32(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.extend_from_slice(&u32::try_from(n).ok()?.to_le_bytes());
    Some(())
}

fn put_str(out: &mut Vec<u8>, s: &str) -> Option<()> {
    put_u32(out, s.len())?;
    out.extend_from_slice(s.as_bytes());
    Some(())
}

fn fingerprint_of(bytes: &[u8]) -> Fingerprint {
    let mut hasher = blake3::Hasher::new_derive_key(KEY_CONTEXT);
    hasher.update(bytes);
    Fingerprint(*hasher.finalize().as_bytes())
}
