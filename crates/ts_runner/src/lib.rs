// spec: revues/architecture.md §5.2, §19.1, §19.2 ; carnet architecture-securite.md SEC0-25, SEC0-26, §9.15
//! Tool host of time_switcher, in no public binary:
//! - [`load_content_dir`] reads a content directory from the disk for `ts_data`, which is a
//!   core crate and does no file access itself;
//! - `ts-data check` ([`data_check`]) checks a content directory and measures its data bounds;
//! - the `ts-sim` runner (phase 0: `ts-sim run` on the toy world of `ts_toy` only), whose parts
//!   are the modules below.

#![forbid(unsafe_code)]

mod content_dir;
pub mod data_check;
pub mod manifest;
pub mod metrics;
pub mod out_dir;
pub mod run;
pub mod scenario;

pub(crate) use content_dir::read_file;
pub use content_dir::{load_content_dir, measure_content_dir};
