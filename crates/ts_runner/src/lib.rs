// spec: revues/architecture.md §5.2 ; carnet architecture-securite.md SEC0-25, SEC0-26
//! Tool host of time_switcher. Phase 0 holds one thing: reading a content directory from the
//! disk for `ts_data`, which is a core crate and does no file access itself. The runner proper
//! (`ts-sim`) comes later and will take this module as it is.

#![forbid(unsafe_code)]

mod content_dir;

pub use content_dir::load_content_dir;
