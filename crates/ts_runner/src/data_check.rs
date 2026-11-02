// spec: attendus-mesurables.md §10.1 (M39a to M39f) ; attendus-mesurables.md §10.2 (M40d)
//! `ts-data check <content-dir> [--github]`: loads a content directory, checks it against its
//! identifier registry, and reports the measure of each data bound (M39a to M39f): the largest
//! observed value and its share of the bound.
//!
//! A bound used strictly over half is flagged (A15, §10: the CI warns at 50 %, and the bound is
//! then raised by a design PR to 4 times the measure). With `--github`, each flagged bound is
//! also printed as a GitHub Actions warning annotation. A flagged bound never fails the check:
//! only an invalid pack or registry does.

use std::fmt::Write as _;

use ts_data::{Limits, REGISTRY_FILE, Registry, Row};

use crate::{measure_content_dir, read_file};

/// Usage line of `ts-data`.
pub const USAGE: &str = "usage: ts-data check <content-dir> [--github]";

/// Arguments of `ts-data check`.
#[derive(Debug, PartialEq, Eq)]
pub struct Args {
    /// The content directory.
    pub root: std::path::PathBuf,
    /// Also print GitHub Actions warning annotations.
    pub github: bool,
}

/// Parses `check <content-dir> [--github]`; each option once.
pub fn parse(mut args: impl Iterator<Item = std::ffi::OsString>) -> Result<Args, String> {
    if args.next().as_deref() != Some("check".as_ref()) {
        return Err(USAGE.to_owned());
    }
    let (mut root, mut github) = (None, false);
    for arg in args {
        match arg.to_str() {
            Some("--github") if !github => github = true,
            Some(a) if a.starts_with('-') => {
                return Err(format!("unknown or repeated option: {a}\n{USAGE}"));
            }
            _ if root.is_none() => root = Some(std::path::PathBuf::from(arg)),
            _ => return Err(USAGE.to_owned()),
        }
    }
    let root = root.ok_or(USAGE)?;
    Ok(Args { root, github })
}

/// Runs the check on `args.root` under `limits`; the report on success, the errors otherwise.
pub fn run(args: &Args, limits: &Limits) -> Result<String, String> {
    let (pack, usage) = measure_content_dir(&args.root, limits).map_err(|e| e.to_string())?;
    // The registry lists every card of a pack plus the retired ones: the pack's total bound is
    // its ceiling, as in `Registry::parse`, not the per-file bound of the `.ron` files.
    let registry_limits = Limits {
        max_file_bytes: limits.max_total_bytes,
        ..*limits
    };
    let file = read_file(
        &args.root.join(REGISTRY_FILE),
        REGISTRY_FILE,
        &registry_limits,
    )
    .map_err(|d| d.to_string())?;
    let registry = Registry::parse(&file.bytes, limits).map_err(|e| e.to_string())?;
    registry.check_pack(&pack).map_err(|e| e.to_string())?;
    let rows = usage.rows(limits);
    let mut out = format!(
        "ts-data check: {} cards, registry {REGISTRY_FILE} consistent, pack {}\n",
        pack.cards().len(),
        pack.fingerprint()
    );
    for row in &rows {
        let _ = writeln!(out, "{}", render(row));
    }
    let flagged = rows.iter().filter(|r| r.over_half()).count();
    if flagged > 0 {
        let _ = writeln!(
            out,
            "{flagged} bound(s) over half used: raise each to 4 times its measure, rounded up to a \
             power of 2, by a design PR (attendus-mesurables.md §10)"
        );
    }
    if args.github {
        for row in rows.iter().filter(|r| r.over_half()) {
            let _ = writeln!(out, "{}", annotation(row));
        }
    }
    Ok(out)
}

/// `x.y %` from tenths of a percent; `>1000 %` when the share does not fit.
fn percent(per_mille: u64) -> String {
    if per_mille > 10_000 {
        return ">1000 %".to_owned();
    }
    format!("{}.{} %", per_mille / 10, per_mille % 10)
}

/// A relative path as printed, escaped by `ts_data::escape_for_log`: control characters
/// escaped, so it cannot end a line; every `#` escaped, so it cannot hold the `##[` or `##vso[`
/// commands that the Actions runner looks for anywhere in a line; a leading `::` escaped, since
/// the runner reads a `::` command at the start of a line, after any whitespace. In this
/// report a path never comes first on a line, but the error lines of `Diagnostic` start with
/// one (the same function escapes them in `Diagnostic::new`), so the rule is the same.
fn printable(path: &str) -> String {
    ts_data::escape_for_log(path)
}

/// One report line.
fn render(row: &Row) -> String {
    let mut line = format!(
        "{}  {:<22} {:>10} of {:<10} {:>8}",
        row.id,
        row.what,
        row.value,
        row.bound,
        percent(row.per_mille())
    );
    if let Some(at) = &row.at {
        let _ = write!(line, "  {}", printable(at));
    }
    if row.over_half() {
        line.push_str("  OVER HALF");
    }
    line
}

/// A GitHub Actions warning annotation for a bound used over half. The message is escaped as
/// the workflow command format requires (`%`, CR and LF); the title is a fixed bound number.
fn annotation(row: &Row) -> String {
    let mut message = format!(
        "{} {}: {} of {} ({}), over half of the bound",
        row.id,
        row.what,
        row.value,
        row.bound,
        percent(row.per_mille())
    );
    if let Some(at) = &row.at {
        let _ = write!(message, ", in {}", printable(at));
    }
    message.push_str(": raise it by a design PR (attendus-mesurables.md §10)");
    let message: String = message
        .chars()
        .map(|c| match c {
            '%' => "%25".to_owned(),
            '\r' => "%0D".to_owned(),
            '\n' => "%0A".to_owned(),
            c => c.to_string(),
        })
        .collect();
    format!("::warning title={} data bound::{message}", row.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(value: u64, bound: u64, at: Option<&str>) -> Row {
        Row {
            id: "M39a",
            what: "largest file (bytes)",
            value,
            bound,
            at: at.map(str::to_owned),
        }
    }

    #[test]
    fn percent_has_one_decimal() {
        assert_eq!(percent(0), "0.0 %");
        assert_eq!(percent(572), "57.2 %");
        assert_eq!(percent(1000), "100.0 %");
        assert_eq!(percent(10_000), "1000.0 %");
        assert_eq!(percent(u64::MAX), ">1000 %");
    }

    #[test]
    fn line_flags_over_half_only() {
        let line = render(&row(512, 1024, Some("a.ron")));
        assert!(line.contains("50.0 %") && line.ends_with("a.ron"), "{line}");
        let line = render(&row(513, 1024, None));
        assert!(line.ends_with("OVER HALF"), "{line}");
    }

    #[test]
    fn paths_cannot_forge_output_lines() {
        let path = Some("a\n::error::forged%0A.ron");
        let line = render(&row(1, 2, path));
        assert!(!line.contains('\n'), "{line}");
        let note = annotation(&row(2, 2, path));
        assert!(!note.contains('\n'), "{note}");
        assert!(
            note.starts_with("::warning title=M39a data bound::M39a "),
            "{note}"
        );
        assert!(note.contains(r"a\n::error::forged%250A.ron"), "{note}");
        assert!(note.contains("(100.0 %25)"), "{note}");
        // The legacy commands of the runner match anywhere in a line.
        for text in [
            render(&row(1, 2, Some("##[error]x"))),
            annotation(&row(2, 2, Some("##vso[x"))),
        ] {
            assert!(!text.contains("##"), "{text}");
        }
    }

    #[test]
    fn arguments() {
        let args = |list: &[&str]| parse(list.iter().map(std::ffi::OsString::from));
        let ok = args(&["check", "dir", "--github"]).unwrap();
        assert_eq!(
            ok,
            Args {
                root: "dir".into(),
                github: true
            }
        );
        assert!(!args(&["check", "dir"]).unwrap().github);
        for bad in [
            &["check"][..],
            &["run", "dir"],
            &["check", "a", "b"],
            &["check", "dir", "--github", "--github"],
            &["check", "dir", "--other"],
            &[],
        ] {
            assert!(args(bad).is_err(), "{bad:?}");
        }
    }
}
