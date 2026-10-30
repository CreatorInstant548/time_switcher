//! Minimal card kinds and their field rules. No game rule, no game value.

use serde::Deserialize;

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

    /// Whether the card is an item (the only kind a reference may target).
    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item { .. })
    }
}

// PROVISIONAL(design #44): identifier and name bounds, to confirm by the design.
/// Longest identifier, in bytes.
pub const MAX_ID_BYTES: usize = 64;
/// Longest name, in bytes.
pub const MAX_NAME_BYTES: usize = 256;

/// Whether `id` is `segment(.segment)*` with at most [`MAX_ID_BYTES`] bytes, each segment
/// being a lowercase letter then lowercase letters, digits or `_` (for example `item.bread`).
pub fn is_valid_id(id: &str) -> bool {
    id.len() <= MAX_ID_BYTES
        && !id.is_empty()
        && id.split('.').all(|seg| {
            let mut chars = seg.chars();
            chars.next().is_some_and(|c| c.is_ascii_lowercase())
                && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        })
}

/// Whether `name` is 1 to [`MAX_NAME_BYTES`] bytes with no control character.
pub fn is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= MAX_NAME_BYTES && !name.chars().any(char::is_control)
}
