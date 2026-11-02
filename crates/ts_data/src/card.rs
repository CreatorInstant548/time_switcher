//! Minimal card kinds and their field rules. No game rule, no game value.

use serde::Deserialize;
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

use crate::error::Code;

/// One content card. A file holds a RON list of cards, for example
/// `[Item(id: "item.bread", name: "Bread")]`.
///
/// Unknown fields are refused, so a typo in a field name fails the load.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Card {
    /// A kind of object.
    Item {
        /// Unique identifier.
        id: String,
        /// Display name.
        name: String,
    },
    /// A kind of activity.
    Activity {
        /// Unique identifier.
        id: String,
        /// Display name.
        name: String,
        /// Identifiers of item cards that the activity mentions (a reference only).
        items: Vec<String>,
    },
}

impl Card {
    /// The identifier.
    pub fn id(&self) -> &str {
        match self {
            Self::Item { id, .. } | Self::Activity { id, .. } => id,
        }
    }

    /// The display name.
    pub fn name(&self) -> &str {
        match self {
            Self::Item { name, .. } | Self::Activity { name, .. } => name,
        }
    }

    /// Identifiers this card refers to.
    pub fn references(&self) -> &[String] {
        match self {
            Self::Item { .. } => &[],
            Self::Activity { items, .. } => items,
        }
    }

    /// Namespace of the card's kind, one of [`NAMESPACES`]: the first segment of its
    /// identifier must be this (M40b).
    pub fn namespace(&self) -> &'static str {
        match self {
            Self::Item { .. } => ITEM,
            Self::Activity { .. } => ACTIVITY,
        }
    }

    /// Whether the card is an item (the only kind a reference may target).
    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item { .. })
    }
}

// spec: attendus-mesurables.md §10.2 (M40a, M40e)
/// Longest identifier, in bytes.
pub const MAX_ID_BYTES: usize = 64;
/// Longest name, in bytes.
pub const MAX_NAME_BYTES: usize = 256;

// spec: attendus-mesurables.md §10.2 (M40b)
/// Closed list of the card-kind namespaces, sorted: the name of each kind of [`Card`] in
/// `snake_case`. A new kind of card adds its namespace here, in the same design PR.
pub const NAMESPACES: [&str; 2] = [ACTIVITY, ITEM];
/// Namespace of [`Card::Activity`].
const ACTIVITY: &str = "activity";
/// Namespace of [`Card::Item`].
const ITEM: &str = "item";

/// Whether `id` is `segment(.segment)*` with 1 to [`MAX_ID_BYTES`] bytes, each segment being a
/// lowercase letter then lowercase letters, digits or `_` (for example `toy_year` or
/// `item.bread`). This is the bare grammar, also used for names that are not card
/// identifiers (scenario names); a card identifier also needs [`is_valid_card_id`].
pub fn is_valid_id(id: &str) -> bool {
    id.len() <= MAX_ID_BYTES
        && !id.is_empty()
        && id.split('.').all(|seg| {
            let mut chars = seg.chars();
            chars.next().is_some_and(|c| c.is_ascii_lowercase())
                && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        })
}

// spec: attendus-mesurables.md §10.2 (M40a)
/// Whether `id` is a card identifier: the grammar of [`is_valid_id`] with **at least two**
/// segments (namespace first). Whether the namespace is the one of the card's kind is
/// checked by the pack ([`Card::namespace`]).
pub fn is_valid_card_id(id: &str) -> bool {
    is_valid_id(id) && id.contains('.')
}

/// Most combining marks in a row that a name may hold (M40e).
const MAX_MARK_RUN: usize = 2;

// spec: attendus-mesurables.md §10.2 (M40e)
/// Why `name` is not a valid display name, or `None` if it is valid. The code names the class
/// of the first fault found, in this order: size, then the characters from left to right, then
/// the edges, then the normalization form. Every class has its own code (decided, A17):
/// [`Code::InvalidName`] (empty, over [`MAX_NAME_BYTES`] bytes, leading or trailing space,
/// not in NFC), [`Code::NameControl`] (Cc), [`Code::NameFormat`] (Cf),
/// [`Code::NamePrivateUse`] (Co), [`Code::NameUnassigned`] (Cn), [`Code::NameLineSeparator`]
/// (Zl, Zp), [`Code::NameSpace`] (any space other than U+0020, U+00A0 and U+202F) and
/// [`Code::NameMarkRun`] (more than two combining marks in a row).
///
/// The categories are those of the Unicode version of the NFC check (a test pins that the two
/// tables agree), so refusing Cn means the NFC form of an accepted name cannot change later.
pub fn name_fault(name: &str) -> Option<Code> {
    if name.is_empty() || name.len() > MAX_NAME_BYTES {
        return Some(Code::InvalidName);
    }
    // At most `MAX_NAME_BYTES` bytes from here: the scan is cheap whatever the input.
    let mut marks = 0_usize;
    for c in name.chars() {
        let category = c.general_category();
        let fault = match category {
            GeneralCategory::Control => Some(Code::NameControl),
            GeneralCategory::Format => Some(Code::NameFormat),
            GeneralCategory::PrivateUse => Some(Code::NamePrivateUse),
            GeneralCategory::Unassigned => Some(Code::NameUnassigned),
            GeneralCategory::LineSeparator | GeneralCategory::ParagraphSeparator => {
                Some(Code::NameLineSeparator)
            }
            GeneralCategory::SpaceSeparator if !matches!(c, ' ' | '\u{a0}' | '\u{202f}') => {
                Some(Code::NameSpace)
            }
            _ => None,
        };
        if fault.is_some() {
            return fault;
        }
        if matches!(
            category,
            GeneralCategory::NonspacingMark
                | GeneralCategory::SpacingMark
                | GeneralCategory::EnclosingMark
        ) {
            marks = marks.saturating_add(1);
            if marks > MAX_MARK_RUN {
                return Some(Code::NameMarkRun);
            }
        } else {
            marks = 0;
        }
    }
    if name.trim() != name || !unicode_normalization::is_nfc(name) {
        return Some(Code::InvalidName);
    }
    None
}

// spec: attendus-mesurables.md §10.2 (M40e)
/// Whether `name` is a valid display name: [`name_fault`] finds nothing.
pub fn is_valid_name(name: &str) -> bool {
    name_fault(name).is_none()
}
