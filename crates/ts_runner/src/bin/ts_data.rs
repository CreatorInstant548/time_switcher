// spec: attendus-mesurables.md §10.1 (M39a à M39f)
//! `ts-data`, the content tool (phase 0: `check` only).
//!
//! ```text
//! ts-data check <content-dir> [--github]
//! ```
//!
//! Loads the content directory under the default bounds, checks it against its identifier
//! registry, and prints the measure of each data bound. `--github` adds a warning annotation
//! for each bound used over half. Exit code 1 if the pack or the registry is invalid.
#![forbid(unsafe_code)]
#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "command-line tool: report on stdout, errors on stderr"
)]

use std::process::ExitCode;

use ts_data::Limits;
use ts_runner::data_check;

#[allow(clippy::disallowed_methods)] // tool: reads its command line.
fn main() -> ExitCode {
    let report = data_check::parse(std::env::args_os().skip(1))
        .and_then(|args| data_check::run(&args, &Limits::default()));
    match report {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ts-data: {e}");
            ExitCode::FAILURE
        }
    }
}
