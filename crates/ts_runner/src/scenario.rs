// spec: revues/architecture.md §19.1 ; carnet architecture-securite.md SEC0-25, §9.15 Y4
//! The scenario of `ts-sim run`: a RON file read and parsed under the **same bounds** as the
//! content cards of `ts_data` (file size, RON depth), with typed errors and no panic.
//!
//! Phase 0 knows one world, the toy world of `ts_toy`, and one setting, the duration:
//!
//! ```ron
//! Scenario(name: "toy_year", world: Toy, duration: Days(365))
//! ```

use std::path::Path;

use serde::Deserialize;
use ts_data::{Code, Diagnostic, Limits, SourceFile, is_valid_id};

/// Longest run a scenario may ask, in simulated days. A bound of the tool, not a game value.
pub const MAX_DAYS: u32 = 3660;
/// Name given to the scenario file in errors: never its local path (SEC0-26).
const SHOWN: &str = "scenario.ron";

/// A parsed, validated scenario.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    /// Name, in the identifier grammar of `ts_data` (for example `toy_year`).
    pub name: String,
    /// World to run.
    pub world: World,
    /// Simulated time to run.
    pub duration: Duration,
}

/// Worlds `ts-sim` can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum World {
    /// The toy world of `ts_toy` (test plugin, D-DEV-02), on `TownSpec::PHASE0`.
    Toy,
}

/// A simulated duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Duration {
    /// Whole simulated days, 1 to [`MAX_DAYS`].
    Days(u32),
}

impl Scenario {
    /// Simulated days to run.
    pub fn days(&self) -> u32 {
        match self.duration {
            Duration::Days(d) => d,
        }
    }

    /// Parses `file` under `limits`, as `ts_data` parses a card file, then validates it.
    pub fn parse(file: &SourceFile, limits: &Limits) -> Result<Self, Diagnostic> {
        if u64::try_from(file.bytes.len()).map_or(true, |n| n > limits.max_file_bytes) {
            return Err(Diagnostic::new(Code::FileTooLarge, SHOWN, (0, 0), ""));
        }
        let bytes = file
            .bytes
            .strip_prefix(&[0xEF, 0xBB, 0xBF])
            .unwrap_or(&file.bytes);
        let scenario: Self = ron::Options::default()
            .with_recursion_limit(limits.max_depth)
            .from_bytes(bytes)
            .map_err(|e| {
                use ron::error::Error as E;
                let (code, detail) = match &e.code {
                    E::ExceededRecursionLimit => (Code::RecursionLimit, String::new()),
                    E::Utf8Error(_) => (Code::NotUtf8, String::new()),
                    E::MissingStructField { field, .. } => {
                        (Code::MissingField, (*field).to_owned())
                    }
                    E::NoSuchStructField { found, .. } => (Code::UnknownField, found.clone()),
                    E::NoSuchEnumVariant { found, .. } => (Code::UnknownKind, found.clone()),
                    _ => (Code::RonSyntax, String::new()),
                };
                let at = (
                    u32::try_from(e.span.start.line).unwrap_or(u32::MAX),
                    u32::try_from(e.span.start.col).unwrap_or(u32::MAX),
                );
                Diagnostic::new(code, SHOWN, at, &detail)
            })?;
        if !is_valid_id(&scenario.name) {
            return Err(Diagnostic::new(Code::InvalidId, SHOWN, (0, 0), "name"));
        }
        if !(1..=MAX_DAYS).contains(&scenario.days()) {
            return Err(Diagnostic::new(Code::RonSyntax, SHOWN, (0, 0), "duration"));
        }
        Ok(scenario)
    }

    /// Reads the scenario file at `path` (a symbolic link or junction is refused, also when
    /// swapped in during the read, and the read is bounded, as in `load_content_dir`), then
    /// parses it. Also returns the bytes read.
    pub fn load(path: &Path, limits: &Limits) -> Result<(Self, SourceFile), Diagnostic> {
        let file = crate::read_file(path, SHOWN, limits)?;
        Ok((Self::parse(&file, limits)?, file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Result<Scenario, Code> {
        let file = SourceFile {
            path: SHOWN.to_owned(),
            bytes: text.as_bytes().to_vec(),
        };
        Scenario::parse(&file, &Limits::default()).map_err(|d| d.code)
    }

    #[test]
    fn valid_scenario() {
        let s = parse("\u{feff}Scenario(name: \"toy_year\", world: Toy, duration: Days(365))");
        assert_eq!(s.map(|s| s.days()), Ok(365));
    }

    #[test]
    fn invalid_scenarios_have_typed_errors() {
        let cases = [
            ("(name: \"a\", world: Toy)", Code::MissingField),
            (
                "(name: \"a\", world: Toy, duration: Days(1), seed: 3)",
                Code::UnknownField,
            ),
            (
                "(name: \"a\", world: Orvenne, duration: Days(1))",
                Code::UnknownKind,
            ),
            (
                "(name: \"A b\", world: Toy, duration: Days(1))",
                Code::InvalidId,
            ),
            (
                "(name: \"a\", world: Toy, duration: Days(0))",
                Code::RonSyntax,
            ),
            (
                "(name: \"a\", world: Toy, duration: Days(3661))",
                Code::RonSyntax,
            ),
            (
                "(name: \"a\", world: Toy, duration: Days(-1))",
                Code::RonSyntax,
            ),
            ("(name: \"a\"", Code::RonSyntax),
        ];
        for (text, code) in cases {
            assert_eq!(parse(text).map(|_| ()), Err(code), "{text}");
        }
    }

    #[test]
    fn bounds_of_ts_data_apply() {
        let limits = Limits::default();
        let big = usize::try_from(limits.max_file_bytes).unwrap() + 1;
        let file = |bytes: Vec<u8>| SourceFile {
            path: SHOWN.to_owned(),
            bytes,
        };
        let code = |f: &SourceFile, l: &Limits| Scenario::parse(f, l).unwrap_err().code;
        assert_eq!(code(&file(vec![b' '; big]), &limits), Code::FileTooLarge);
        // The RON depth bound is wired: the scenario nests two levels (struct, then `Days`).
        let valid = file(b"(name: \"a\", world: Toy, duration: Days(1))".to_vec());
        let shallow = Limits {
            max_depth: 1,
            ..limits
        };
        assert_eq!(code(&valid, &shallow), Code::RecursionLimit);
        assert!(Scenario::parse(&valid, &limits).is_ok());
    }
}
