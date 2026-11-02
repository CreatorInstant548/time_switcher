// spec: revues/architecture.md §18.4, §22 ; carnet architecture-securite.md §9.16 VZ4, VZ5, VZ7
//! Development visualisation of time_switcher, phase 0 (T11, V0-10).
//!
//! [`positions`] runs the toy world of `ts_toy` day by day and writes where each resident is,
//! once a day, as a plain Parquet file (`positions.parquet`). A separate, optional Python
//! script (`python/viz/`, group `viz`) converts that file to a Rerun recording; the Rerun
//! viewer is a desktop program installed apart. No `rerun` crate is in the Rust tree (AS-48).

#![forbid(unsafe_code)]

pub mod positions;
