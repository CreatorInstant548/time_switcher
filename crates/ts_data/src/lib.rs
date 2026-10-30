// spec: revues/architecture.md §5.2 ; carnet architecture-securite.md SEC0-25, SEC0-26
//! Content pack of time_switcher: bounded loading of RON cards, validation, canonical
//! encoding and fingerprint.
//!
//! Phase 0 (T7b): two minimal kinds of card, no game rule and no real catalogue. The crate
//! does no file access (core crate, clippy forbids it): the host opens the files, reads them
//! with [`read_bounded`] and gives them to [`ContentPack::build`]. `ts_runner` is the first host.
//!
//! The input is untrusted (a solo player can edit the content): no function here panics, an
//! invalid card fails the load with a typed [`LoadErrors`], never the process.

#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::arithmetic_side_effects,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::unwrap_used
    )
)]

mod card;
mod error;
mod pack;
mod scan;
mod source;

pub use card::{Card, MAX_ID_BYTES, MAX_NAME_BYTES, is_valid_id, is_valid_name};
pub use error::{Code, Diagnostic, LoadErrors, MAX_DIAGNOSTICS};
pub use pack::{ContentPack, FORMAT_VERSION, Fingerprint};
pub use source::{Limits, SourceFile, is_relative_path, read_bounded};

#[cfg(test)]
mod tests;
