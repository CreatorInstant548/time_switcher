//! Bounds and bounded reading of source files. No file access here: the host opens the files
//! and gives readers (core crate, no I/O).

use std::io::Read;

use crate::error::{Code, Diagnostic};

// spec: carnet architecture-securite.md SEC0-25 ; carnet architecture-securite.md §9.13 T2
/// Bounds on the content read. An input over a bound fails the load.
// PROVISIONAL(design #43): all five values, to confirm by the design.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Largest file, in bytes.
    pub max_file_bytes: u64,
    /// Largest total size of all files, in bytes.
    pub max_total_bytes: u64,
    /// Most files in a pack.
    pub max_files: usize,
    /// Most cards in a pack.
    pub max_cards: usize,
    /// RON recursion limit (nesting depth).
    pub max_depth: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_file_bytes: 1 << 20,
            max_total_bytes: 64 << 20,
            max_files: 4096,
            max_cards: 65_536,
            max_depth: 16,
        }
    }
}

/// A content file already read: path relative to the content root, with `/`, and bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceFile {
    /// Relative path, never absolute (SEC0-26).
    pub path: String,
    /// Content, at most [`Limits::max_file_bytes`].
    pub bytes: Vec<u8>,
}

/// Reads a file under the size bound. `declared_len` comes from the metadata and is checked
/// first; the read itself stops at `max_file_bytes + 1` and fails above `max_file_bytes`, so a
/// file that grows after the metadata check is refused too. The declared size never sizes a
/// buffer.
pub fn read_bounded<R: Read>(
    path: &str,
    declared_len: u64,
    reader: R,
    limits: &Limits,
) -> Result<SourceFile, Diagnostic> {
    let too_large = || Diagnostic::new(Code::FileTooLarge, path, (0, 0), "");
    if declared_len > limits.max_file_bytes {
        return Err(too_large());
    }
    let mut bytes = Vec::new();
    reader
        .take(limits.max_file_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|_| Diagnostic::new(Code::ReadFailed, path, (0, 0), ""))?;
    if u64::try_from(bytes.len()).map_or(true, |n| n > limits.max_file_bytes) {
        return Err(too_large());
    }
    Ok(SourceFile {
        path: path.to_owned(),
        bytes,
    })
}

/// Whether `path` is a plain relative path with `/`: no empty, `.` or `..` component, no
/// backslash, no `:`, no leading `/`.
pub fn is_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':', '\0'])
        && path
            .split('/')
            .all(|c| !c.is_empty() && c != "." && c != "..")
}
