//! Reads a content directory under the bounds of `ts_data`: symbolic links refused, also when
//! swapped in during the read (#48), file count and sizes bounded, relative `/` paths only.

use std::ffi::OsString;
use std::fs::{self, File, Metadata, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

// spec: carnet architecture-securite.md SEC0-25, SEC0-26
use ts_data::{Code, ContentPack, Diagnostic, Limits, LoadErrors, SourceFile, Usage, read_bounded};

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
/// # Link races (R-ARCH-08, AS-35, #48)
///
/// The checks and the opening are not atomic: a writer who can change the content directory
/// concurrently may swap an entry between its check and its opening. Each file is opened
/// without following a link where `std` allows it, and the opened handle is checked against
/// the entry that was checked (`read_checked`):
/// - **a file swapped for a link is refused**. On Windows the file is opened with
///   `FILE_FLAG_OPEN_REPARSE_POINT`: a file link is opened itself, never its target, and the
///   handle's metadata shows it; a junction (a directory) fails to open. On Unix the device
///   and inode of the opened handle must be those that `symlink_metadata` saw before the
///   opening; a link is its own inode, so a followed link never matches. When the opening
///   fails or the handle does not match, the entry is looked at again: a link there is
///   reported as `SymlinkRefused`, anything else (a file renamed over it by an editor, a link
///   already removed) as `ReadFailed`;
/// - the handle must be a regular file, and the read stays bounded by `take(max + 1)`.
///
/// Still open, accepted (AS-35: `std` without `unsafe` and without `libc`, so neither `openat`
/// nor `O_NOFOLLOW`):
/// - **replacement of a parent directory** by a link between its listing and the opening of
///   a file under it: not detected, the file under the link is read (needs `openat` relative
///   to an opened directory, or `cap-std`);
/// - **on Unix, a FIFO swapped in** after the check: `open` blocks until a writer comes (the
///   check before the opening refuses a FIFO already in place; only the race remains);
/// - on Windows, a file swapped for **another regular file** (rename) is read: it is not a
///   link, and the identity of a file needs an API that `std` has not stabilised.
///
/// All of them need write access to the content directory, which already allows changing the
/// content itself.
pub fn load_content_dir(root: &Path, limits: &Limits) -> Result<ContentPack, LoadErrors> {
    let (files, _) = read_content_dir(root, limits)?;
    ContentPack::build(files, limits)
}

// spec: attendus-mesurables.md §10.1 (M39a to M39f)
/// Loads `root` as [`load_content_dir`] does, and measures the largest value of each data
/// bound ([`Usage`]): the files give their sizes and nesting depth, the walk its entry count
/// and sub-directory depth, the pack its card count. Measured only when the pack loads.
pub fn measure_content_dir(
    root: &Path,
    limits: &Limits,
) -> Result<(ContentPack, Usage), LoadErrors> {
    let (files, walk) = read_content_dir(root, limits)?;
    let mut usage = Usage::of_files(&files, limits);
    let pack = ContentPack::build(files, limits)?;
    usage.entries = u64::try_from(walk.entries).unwrap_or(u64::MAX);
    usage.dir_depth = u64::try_from(walk.dir_depth).unwrap_or(u64::MAX);
    usage.cards = u64::try_from(pack.cards().len()).unwrap_or(u64::MAX);
    Ok((pack, usage))
}

/// What the walk of a content directory saw, for the measure of M39c and M39f.
struct Walk {
    /// Entries listed, as counted against `max_files`.
    entries: usize,
    /// Deepest sub-directory level read (the root is 0).
    dir_depth: usize,
}

/// The walk of [`load_content_dir`]: every bounded `*.ron` file under `root`, unsorted.
fn read_content_dir(root: &Path, limits: &Limits) -> Result<(Vec<SourceFile>, Walk), LoadErrors> {
    if is_link(&fs::symlink_metadata(root).map_err(|_| fail(Code::ReadFailed, ""))?) {
        return Err(fail(Code::SymlinkRefused, ""));
    }
    let mut files = Vec::new();
    let mut visited: usize = 0;
    let mut total_bytes: u64 = 0;
    let mut deepest: usize = 0;
    let mut pending = vec![(root.to_path_buf(), String::new(), 0_usize)];
    while let Some((dir, rel, depth)) = pending.pop() {
        deepest = deepest.max(depth);
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
                let file = read_checked(&path, &child, &md, limits).map_err(LoadErrors::single)?;
                total_bytes =
                    total_bytes.saturating_add(u64::try_from(file.bytes.len()).unwrap_or(u64::MAX));
                if total_bytes > limits.max_total_bytes {
                    return Err(fail(Code::TotalTooLarge, ""));
                }
                files.push(file);
            }
        }
    }
    let walk = Walk {
        entries: visited,
        dir_depth: deepest,
    };
    Ok((files, walk))
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

/// Reads the content file at `path` as [`read_checked`] does, against its `symlink_metadata`
/// taken now: a link is refused, never followed. `rel` names it in errors.
pub(crate) fn read_file(path: &Path, rel: &str, limits: &Limits) -> Result<SourceFile, Diagnostic> {
    let before = fs::symlink_metadata(path)
        .map_err(|_| Diagnostic::new(Code::ReadFailed, rel, (0, 0), ""))?;
    read_checked(path, rel, &before, limits)
}

// spec: carnet architecture-securite.md R-ARCH-08
/// Opens one file and reads it bounded, against `before`, its `symlink_metadata` taken before
/// the opening (AS-35, #48):
/// - `before` must be a regular file, not a link;
/// - the opening does not follow a link on Windows (`FILE_FLAG_OPEN_REPARSE_POINT`);
/// - the opened handle must not be a link, must be a regular file, and on Unix must have the
///   device and inode of `before`: a link, or another file, swapped in since is refused.
///
/// When the opening fails or the handle is another file, the entry is looked at again only to
/// choose the error: `SymlinkRefused` if it is a link now, `ReadFailed` otherwise.
fn read_checked(
    path: &Path,
    rel: &str,
    before: &Metadata,
    limits: &Limits,
) -> Result<SourceFile, Diagnostic> {
    let failed = || Diagnostic::new(Code::ReadFailed, rel, (0, 0), "");
    let refused = || Diagnostic::new(Code::SymlinkRefused, rel, (0, 0), "");
    let swapped = || match fs::symlink_metadata(path).is_ok_and(|now| is_link(&now)) {
        true => refused(),
        false => failed(),
    };
    if is_link(before) {
        return Err(refused());
    }
    if !before.is_file() {
        return Err(failed());
    }
    let file = open_no_follow(path).map_err(|_| swapped())?;
    let opened = file.metadata().map_err(|_| failed())?;
    if is_link(&opened) {
        return Err(refused());
    }
    if !same_file(before, &opened) {
        return Err(swapped());
    }
    if !opened.is_file() {
        return Err(failed());
    }
    read_bounded(rel, opened.len(), file, limits)
}

/// Opens `path` for reading without following a reparse point: a file link is opened itself,
/// not its target, so [`read_checked`] sees it on the handle; a junction (a directory) fails
/// to open.
#[cfg(windows)]
#[allow(clippy::disallowed_methods)] // host: opens one content file.
fn open_no_follow(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;
    /// `FILE_FLAG_OPEN_REPARSE_POINT` of `CreateFileW` (winbase.h).
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
}

/// Opens `path` for reading. A link is followed: [`same_file`] refuses it afterwards.
#[cfg(not(windows))]
#[allow(clippy::disallowed_methods)] // host: opens one content file.
fn open_no_follow(path: &Path) -> io::Result<File> {
    OpenOptions::new().read(true).open(path)
}

/// Whether the opened handle is the entry checked before the opening: same device and inode.
#[cfg(unix)]
fn same_file(before: &Metadata, opened: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    before.dev() == opened.dev() && before.ino() == opened.ino()
}

/// Elsewhere, the identity of a file is not stable in `std`: the opening itself does not
/// follow a link (Windows, [`open_no_follow`]), and [`read_checked`] refuses a link handle.
#[cfg(not(unix))]
fn same_file(_before: &Metadata, _opened: &Metadata) -> bool {
    true
}

#[cfg(test)]
mod tests {
    #![allow(clippy::disallowed_methods)] // test: throwaway files.

    use super::*;

    const CARD: &str = "[Item(id: \"item.x\", name: \"X\")]";

    fn scratch(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("ts_runner_read_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn code(read: Result<SourceFile, Diagnostic>) -> Code {
        read.unwrap_err().code
    }

    #[test]
    fn regular_file_is_read_and_a_directory_is_not() {
        let dir = scratch("regular");
        let path = dir.join("a.ron");
        fs::write(&path, CARD).unwrap();
        let file = read_file(&path, "a.ron", &Limits::default()).unwrap();
        assert_eq!(file.bytes, CARD.as_bytes());
        assert_eq!(
            code(read_file(&dir, "d", &Limits::default())),
            Code::ReadFailed
        );
        let _ = fs::remove_dir_all(&dir);
    }

    /// The race itself: the entry was checked as a regular file (`before`), then swapped for
    /// a symbolic link before the opening.
    #[test]
    fn link_swapped_in_after_the_check_is_refused() {
        let dir = scratch("swap_link");
        let checked = dir.join("checked.ron");
        let target = dir.join("target.ron");
        fs::write(&checked, CARD).unwrap();
        fs::write(&target, CARD).unwrap();
        let before = fs::symlink_metadata(&checked).unwrap();
        let path = dir.join("a.ron");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &path).unwrap();
        #[cfg(windows)]
        if std::os::windows::fs::symlink_file(&target, &path).is_err() {
            // Windows without the privilege: the junction test covers this platform.
            let _ = fs::remove_dir_all(&dir);
            return;
        }
        let read = read_checked(&path, "a.ron", &before, &Limits::default());
        assert_eq!(code(read), Code::SymlinkRefused);
        // Checked as the link itself: refused before any opening.
        let link = fs::symlink_metadata(&path).unwrap();
        let read = read_checked(&path, "a.ron", &link, &Limits::default());
        assert_eq!(code(read), Code::SymlinkRefused);
        let _ = fs::remove_dir_all(&dir);
    }

    /// On Unix, another regular file renamed over the checked one (as an editor saves) is
    /// refused too, as a failed read: no link is involved.
    #[cfg(unix)]
    #[test]
    fn other_file_swapped_in_after_the_check_is_refused() {
        let dir = scratch("swap_file");
        let path = dir.join("a.ron");
        let other = dir.join("other.ron");
        fs::write(&path, CARD).unwrap();
        fs::write(&other, CARD).unwrap();
        let before = fs::symlink_metadata(&path).unwrap();
        fs::rename(&other, &path).unwrap();
        let read = read_checked(&path, "a.ron", &before, &Limits::default());
        assert_eq!(code(read), Code::ReadFailed);
        let _ = fs::remove_dir_all(&dir);
    }

    /// A junction needs no privilege on Windows. Swapped in for a checked file, its opening
    /// with `FILE_FLAG_OPEN_REPARSE_POINT` fails (it is a directory), so its target is never
    /// reached, and the second look reports the link. The handle check (`is_link` on the
    /// opened file) is exercised by the file link test, where the platform allows file links.
    #[cfg(windows)]
    #[test]
    fn junction_swapped_in_after_the_check_is_refused() {
        let dir = scratch("swap_junction");
        let checked = dir.join("checked.ron");
        fs::write(&checked, CARD).unwrap();
        let before = fs::symlink_metadata(&checked).unwrap();
        let outside = dir.join("outside");
        fs::create_dir_all(&outside).unwrap();
        let path = dir.join("a.ron");
        let made = std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&path)
            .arg(&outside)
            .output()
            .unwrap();
        assert!(made.status.success(), "mklink /J failed");
        let read = read_checked(&path, "a.ron", &before, &Limits::default());
        assert_eq!(code(read), Code::SymlinkRefused);
        let _ = fs::remove_dir_all(&dir);
    }
}
