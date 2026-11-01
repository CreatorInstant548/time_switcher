//! Reads a content directory under the bounds of `ts_data`: symbolic links refused, file
//! count and sizes bounded, relative `/` paths only.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

// spec: carnet architecture-securite.md SEC0-25, SEC0-26
use ts_data::{Code, ContentPack, Diagnostic, Limits, LoadErrors, SourceFile, read_bounded};

use crate::out_dir::is_link;

fn fail(code: Code, file: &str) -> LoadErrors {
    LoadErrors::single(Diagnostic::new(code, file, (0, 0), ""))
}

/// Loads every `*.ron` file (extension in any case) under `root`, sub-directories included,
/// into a [`ContentPack`].
///
/// - A symbolic link or junction, anywhere (the root included), fails the load: never followed.
/// - Entries visited are counted against `limits.max_files`, directories too, and the count is
///   checked while a directory is listed, so a huge directory is never held whole.
/// - Directories nested deeper than `limits.max_dir_depth` fail the load.
/// - Each file is read under `limits.max_file_bytes` (metadata check, then a bounded read) and
///   all files together under `limits.max_total_bytes`.
/// - Other files are ignored, whatever their name. Errors carry paths relative to `root`,
///   never the local path.
///
/// # Residual risk R-ARCH-08 (AS-35), accepted
///
/// The checks and the opening are not atomic. A writer who can change the content directory
/// concurrently could, between the link check and `File::open`:
/// - swap a file for a symbolic link: the link is followed, but the handle opened is checked
///   with `File::metadata()` and refused unless it is a regular file, and the read stays
///   bounded by `take(max + 1)`;
/// - replace a parent directory by a link: not detected, the files under it are read;
/// - on Unix, swap a file for a FIFO: `open` blocks until a writer comes (the metadata check
///   runs after the open, so it cannot prevent this).
///
/// All three need write access to the content directory, which already allows changing the
/// content itself. Hardening with `O_NOFOLLOW` or `FILE_FLAG_OPEN_REPARSE_POINT` is tracked
/// separately.
pub fn load_content_dir(root: &Path, limits: &Limits) -> Result<ContentPack, LoadErrors> {
    if is_link(&fs::symlink_metadata(root).map_err(|_| fail(Code::ReadFailed, ""))?) {
        return Err(fail(Code::SymlinkRefused, ""));
    }
    let mut files = Vec::new();
    let mut visited: usize = 0;
    let mut total_bytes: u64 = 0;
    let mut pending = vec![(root.to_path_buf(), String::new(), 0_usize)];
    while let Some((dir, rel, depth)) = pending.pop() {
        let room = limits.max_files.saturating_sub(visited);
        let entries = read_dir_sorted(&dir, room).map_err(|e| match e {
            DirError::Io => fail(Code::ReadFailed, &rel),
            DirError::TooMany => fail(Code::TooManyFiles, ""),
        })?;
        visited = visited.saturating_add(entries.len());
        for (os_name, path) in entries {
            let md = fs::symlink_metadata(&path).map_err(|_| fail(Code::ReadFailed, &rel))?;
            // Any reparse point counts as a link on Windows, as for `--out` (one policy).
            let link = is_link(&md);
            let kind = md.file_type();
            let is_ron = path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("ron"));
            if !(link || kind.is_dir() || (kind.is_file() && is_ron)) {
                continue;
            }
            let Ok(name) = os_name.into_string() else {
                return Err(fail(Code::BadPath, &rel));
            };
            let child = if rel.is_empty() {
                name
            } else {
                format!("{rel}/{name}")
            };
            if link {
                return Err(fail(Code::SymlinkRefused, &child));
            } else if kind.is_dir() {
                if depth >= limits.max_dir_depth {
                    return Err(fail(Code::TooDeep, &child));
                }
                pending.push((path, child, depth.saturating_add(1)));
            } else {
                let file = read_file(&path, &child, limits).map_err(LoadErrors::single)?;
                total_bytes =
                    total_bytes.saturating_add(u64::try_from(file.bytes.len()).unwrap_or(u64::MAX));
                if total_bytes > limits.max_total_bytes {
                    return Err(fail(Code::TotalTooLarge, ""));
                }
                files.push(file);
            }
        }
    }
    ContentPack::build(files, limits)
}

enum DirError {
    Io,
    TooMany,
}

/// Entries of `dir` as (name, path), sorted by name; fails as soon as there are more than
/// `room`.
#[allow(clippy::disallowed_methods)] // host: lists a content directory.
fn read_dir_sorted(dir: &Path, room: usize) -> Result<Vec<(OsString, PathBuf)>, DirError> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(|_| DirError::Io)? {
        let entry = entry.map_err(|_| DirError::Io)?;
        if out.len() >= room {
            return Err(DirError::TooMany);
        }
        out.push((entry.file_name(), entry.path()));
    }
    out.sort();
    Ok(out)
}

/// Opens one file, checks the opened handle is a regular file (AS-35), then reads it bounded.
#[allow(clippy::disallowed_methods)] // host: opens one content file.
pub(crate) fn read_file(path: &Path, rel: &str, limits: &Limits) -> Result<SourceFile, Diagnostic> {
    let failed = || Diagnostic::new(Code::ReadFailed, rel, (0, 0), "");
    let file = fs::File::open(path).map_err(|_| failed())?;
    let meta = file.metadata().map_err(|_| failed())?;
    if !meta.is_file() {
        return Err(failed());
    }
    read_bounded(rel, meta.len(), file, limits)
}
