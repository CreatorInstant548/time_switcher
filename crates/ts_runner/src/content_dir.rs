//! Reads a content directory under the bounds of `ts_data`: symbolic links refused, file
//! count and sizes bounded, relative `/` paths only.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use ts_data::{Code, ContentPack, Diagnostic, Limits, LoadErrors, SourceFile, read_bounded};

// spec: carnet architecture-securite.md SEC0-25, SEC0-26
/// Deepest sub-directory level read. A technical bound, not a game value.
const MAX_DIR_DEPTH: usize = 8;

fn fail(code: Code, file: &str) -> LoadErrors {
    LoadErrors::single(Diagnostic::new(code, file, (0, 0), ""))
}

/// Loads every `*.ron` file (extension in any case) under `root`, sub-directories included,
/// into a [`ContentPack`].
///
/// - A symbolic link or junction, anywhere (the root included), fails the load: never followed.
/// - Entries visited are counted against `limits.max_files`, directories too.
/// - Each file is read under `limits.max_file_bytes` (metadata check, then a bounded read) and
///   all files together under `limits.max_total_bytes`.
/// - Other files are ignored, whatever their name. Errors carry paths relative to `root`,
///   never the local path.
///
/// Residual risk: a link swapped in between the link check and the opening of a file is
/// followed. It takes a concurrent writer in the content directory, who could change the
/// content anyway.
pub fn load_content_dir(root: &Path, limits: &Limits) -> Result<ContentPack, LoadErrors> {
    if fs::symlink_metadata(root)
        .map_err(|_| fail(Code::ReadFailed, ""))?
        .file_type()
        .is_symlink()
    {
        return Err(fail(Code::SymlinkRefused, ""));
    }
    let mut files = Vec::new();
    let mut visited: usize = 0;
    let mut total_bytes: u64 = 0;
    let mut pending = vec![(root.to_path_buf(), String::new(), 0_usize)];
    while let Some((dir, rel, depth)) = pending.pop() {
        for (os_name, path) in read_dir_sorted(&dir).map_err(|()| fail(Code::ReadFailed, &rel))? {
            visited = visited.saturating_add(1);
            if visited > limits.max_files {
                return Err(fail(Code::TooManyFiles, ""));
            }
            let kind = fs::symlink_metadata(&path)
                .map_err(|_| fail(Code::ReadFailed, &rel))?
                .file_type();
            let is_ron = path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("ron"));
            if !(kind.is_symlink() || kind.is_dir() || (kind.is_file() && is_ron)) {
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
            if kind.is_symlink() {
                return Err(fail(Code::SymlinkRefused, &child));
            } else if kind.is_dir() {
                if depth >= MAX_DIR_DEPTH {
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

/// Entries of `dir` as (name, path), sorted by name.
#[allow(clippy::disallowed_methods)] // host: lists a content directory.
fn read_dir_sorted(dir: &Path) -> Result<Vec<(OsString, PathBuf)>, ()> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(|_| ())? {
        let entry = entry.map_err(|_| ())?;
        out.push((entry.file_name(), entry.path()));
    }
    out.sort();
    Ok(out)
}

#[allow(clippy::disallowed_methods)] // host: opens one content file, then reads it bounded.
fn read_file(path: &Path, rel: &str, limits: &Limits) -> Result<SourceFile, Diagnostic> {
    let failed = || Diagnostic::new(Code::ReadFailed, rel, (0, 0), "");
    let file = fs::File::open(path).map_err(|_| failed())?;
    let meta = file.metadata().map_err(|_| failed())?;
    if !meta.is_file() {
        return Err(failed());
    }
    read_bounded(rel, meta.len(), file, limits)
}
