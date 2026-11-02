// spec: attendus-mesurables.md §10.2 (M40d)
//! Registry of content identifiers: one append-only text file, `ids.lock` at the content root
//! (`data/content/ids.lock`), for every kind of card. An identifier is never renamed nor
//! reused: once listed, its line stays, and may only change from `active` to `retired`.
//!
//! Format (decided, A15): UTF-8 text, one entry per line, `<identifier> <state>` with a single
//! space, the state being `active` or `retired`. Empty lines and lines starting with `#` are
//! comments, free to change. New entries go at the end. A byte order mark is skipped, as in a
//! card file. Pure functions, no file access: the host reads the file (and its previous
//! version, for [`Registry::check_successor`]).

use std::collections::{BTreeMap, BTreeSet};

use crate::card::is_valid_card_id;
use crate::error::{Code, Diagnostic, LoadErrors, Sink};
use crate::pack::ContentPack;
use crate::source::Limits;

/// Name of the registry file, at the content root.
pub const REGISTRY_FILE: &str = "ids.lock";

/// State of a registered identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdState {
    /// A card of the pack has this identifier.
    Active,
    /// No card has it any more, and none may have it again.
    Retired,
}

/// One registry entry and its 1-based line.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Entry {
    id: String,
    state: IdState,
    line: u32,
}

/// A parsed registry: its entries in file order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Registry {
    entries: Vec<Entry>,
}

fn at(code: Code, line: u32, detail: &str) -> Diagnostic {
    Diagnostic::new(code, REGISTRY_FILE, (line, 0), detail)
}

impl Registry {
    /// Parses the registry text. Refuses a malformed line and an identifier listed twice. The
    /// identifier must follow the card grammar (M40a, namespace first); its namespace is not
    /// checked against the current kinds, so a retired identifier stays valid if its kind goes.
    ///
    /// The registry is bounded by [`Limits::max_total_bytes`] (decided: it lists at most every
    /// card of a pack, plus the retired ones, so the pack's total bound is its natural
    /// ceiling; the per-file bound of the `.ron` files would cap it near 14,000 identifiers).
    pub fn parse(bytes: &[u8], limits: &Limits) -> Result<Self, LoadErrors> {
        if u64::try_from(bytes.len()).map_or(true, |n| n > limits.max_total_bytes) {
            return Err(LoadErrors::single(at(Code::FileTooLarge, 0, "")));
        }
        let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
        let text = core::str::from_utf8(bytes)
            .map_err(|_| LoadErrors::single(at(Code::NotUtf8, 0, "")))?;
        let mut sink = Sink::default();
        let mut entries = Vec::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for (index, raw) in text.split('\n').enumerate() {
            let line = u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1);
            let content = raw.strip_suffix('\r').unwrap_or(raw);
            if content.is_empty() || content.starts_with('#') {
                continue;
            }
            let state = match content.split_once(' ') {
                Some((id, "active")) if is_valid_card_id(id) => (id, IdState::Active),
                Some((id, "retired")) if is_valid_card_id(id) => (id, IdState::Retired),
                _ => {
                    sink.push_with(|| at(Code::RegistrySyntax, line, content));
                    continue;
                }
            };
            let (id, state) = state;
            if !seen.insert(id) {
                sink.push_with(|| at(Code::RegistryDuplicate, line, id));
                continue;
            }
            entries.push(Entry {
                id: id.to_owned(),
                state,
                line,
            });
        }
        match sink.into_errors() {
            Some(errors) => Err(errors),
            None => Ok(Self { entries }),
        }
    }

    /// Entries in file order: identifier and state.
    pub fn entries(&self) -> impl Iterator<Item = (&str, IdState)> {
        self.entries.iter().map(|e| (e.id.as_str(), e.state))
    }

    /// Checks `pack` against the registry: every card identifier is listed and active, and
    /// every active identifier has a card (a removed card is marked retired, never deleted).
    pub fn check_pack(&self, pack: &ContentPack) -> Result<(), LoadErrors> {
        let listed: BTreeMap<&str, &Entry> =
            self.entries.iter().map(|e| (e.id.as_str(), e)).collect();
        let cards: BTreeSet<&str> = pack.cards().iter().map(|c| c.id()).collect();
        let mut sink = Sink::default();
        for id in &cards {
            match listed.get(id) {
                None => sink.push_with(|| at(Code::Unregistered, 0, id)),
                Some(e) if e.state == IdState::Retired => {
                    sink.push_with(|| at(Code::RetiredReused, e.line, id));
                }
                Some(_) => {}
            }
        }
        for e in &self.entries {
            if e.state == IdState::Active && !cards.contains(e.id.as_str()) {
                sink.push_with(|| at(Code::ActiveWithoutCard, e.line, &e.id));
            }
        }
        sink.into_errors().map_or(Ok(()), Err)
    }

    /// Checks that `next` only appends to `self`, its previous version: each entry of `self`
    /// is still at the same rank in `next`, with the same identifier, and the same state or
    /// `active` turned `retired`. Comments are not compared.
    pub fn check_successor(&self, next: &Self) -> Result<(), LoadErrors> {
        let mut sink = Sink::default();
        for (rank, old) in self.entries.iter().enumerate() {
            match next.entries.get(rank) {
                None => sink.push_with(|| at(Code::RegistryRewritten, 0, &old.id)),
                Some(new) => {
                    let same_id = new.id == old.id;
                    let allowed = new.state == old.state
                        || (old.state == IdState::Active && new.state == IdState::Retired);
                    if !(same_id && allowed) {
                        sink.push_with(|| at(Code::RegistryRewritten, new.line, &old.id));
                    }
                }
            }
        }
        sink.into_errors().map_or(Ok(()), Err)
    }
}
