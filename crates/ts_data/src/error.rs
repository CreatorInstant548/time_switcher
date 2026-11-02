//! Typed load errors: a stable code, a relative file, a position.

use core::fmt;

// spec: revues/architecture.md §5.2 ; carnet architecture-securite.md SEC0-25
/// Stable code of a load error. The English name and [`Code::as_str`] never change once
/// published; the French text is for content authors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Code {
    /// A file is larger than [`crate::Limits::max_file_bytes`].
    FileTooLarge,
    /// The files together are larger than [`crate::Limits::max_total_bytes`].
    TotalTooLarge,
    /// More files than [`crate::Limits::max_files`].
    TooManyFiles,
    /// Directories nested deeper than the host reads.
    TooDeep,
    /// More cards than [`crate::Limits::max_cards`].
    TooManyCards,
    /// A file could not be read.
    ReadFailed,
    /// A symbolic link (or junction) was met: never followed.
    SymlinkRefused,
    /// The relative path of a file is not a plain relative path.
    BadPath,
    /// The file is not valid UTF-8.
    NotUtf8,
    /// The file is not valid RON, or does not match the card schema.
    RonSyntax,
    /// RON nesting deeper than [`crate::Limits::max_depth`].
    RecursionLimit,
    /// A required field is missing.
    MissingField,
    /// A field that the card does not have.
    UnknownField,
    /// A kind of card that does not exist.
    UnknownKind,
    /// A length does not fit the canonical encoding (a `u32`).
    EncodingOverflow,
    /// The identifier does not follow the identifier grammar (M40a).
    InvalidId,
    /// The name is empty, too long, not in NFC, has a control character, or leading or
    /// trailing whitespace (M40e).
    InvalidName,
    /// Two cards have the same identifier.
    DuplicateId,
    /// A reference does not resolve to an item card.
    BrokenReference,
    /// The first segment of the identifier is not the namespace of the card's kind (M40b).
    WrongNamespace,
    /// A line of the identifier registry is malformed (M40d).
    RegistrySyntax,
    /// An identifier is listed twice in the registry (M40d).
    RegistryDuplicate,
    /// An identifier of the pack is missing from the registry (M40d).
    Unregistered,
    /// A retired identifier is used again by a card (M40d).
    RetiredReused,
    /// An identifier active in the registry has no card: mark it retired (M40d).
    ActiveWithoutCard,
    /// A registry line was removed, moved or changed other than from active to retired
    /// (M40d: append-only).
    RegistryRewritten,
}

impl Code {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileTooLarge => "DATA_FILE_TOO_LARGE",
            Self::TotalTooLarge => "DATA_TOTAL_TOO_LARGE",
            Self::TooManyFiles => "DATA_TOO_MANY_FILES",
            Self::TooDeep => "DATA_TOO_DEEP",
            Self::TooManyCards => "DATA_TOO_MANY_CARDS",
            Self::ReadFailed => "DATA_READ_FAILED",
            Self::SymlinkRefused => "DATA_SYMLINK_REFUSED",
            Self::BadPath => "DATA_BAD_PATH",
            Self::NotUtf8 => "DATA_NOT_UTF8",
            Self::RonSyntax => "DATA_RON_SYNTAX",
            Self::RecursionLimit => "DATA_RECURSION_LIMIT",
            Self::MissingField => "DATA_MISSING_FIELD",
            Self::UnknownField => "DATA_UNKNOWN_FIELD",
            Self::UnknownKind => "DATA_UNKNOWN_KIND",
            Self::EncodingOverflow => "DATA_ENCODING_OVERFLOW",
            Self::InvalidId => "DATA_INVALID_ID",
            Self::InvalidName => "DATA_INVALID_NAME",
            Self::DuplicateId => "DATA_DUPLICATE_ID",
            Self::BrokenReference => "DATA_BROKEN_REFERENCE",
            Self::WrongNamespace => "DATA_WRONG_NAMESPACE",
            Self::RegistrySyntax => "DATA_REGISTRY_SYNTAX",
            Self::RegistryDuplicate => "DATA_REGISTRY_DUPLICATE",
            Self::Unregistered => "DATA_UNREGISTERED_ID",
            Self::RetiredReused => "DATA_RETIRED_ID_REUSED",
            Self::ActiveWithoutCard => "DATA_ACTIVE_ID_WITHOUT_CARD",
            Self::RegistryRewritten => "DATA_REGISTRY_REWRITTEN",
        }
    }

    /// Message for content authors, in French.
    pub const fn message_fr(self) -> &'static str {
        match self {
            Self::FileTooLarge => "fichier trop volumineux",
            Self::TotalTooLarge => "contenu total trop volumineux",
            Self::TooManyFiles => "trop de fichiers",
            Self::TooDeep => "dossiers imbriqués trop profondément",
            Self::TooManyCards => "trop de fiches",
            Self::ReadFailed => "lecture du fichier impossible",
            Self::SymlinkRefused => "lien symbolique refusé",
            Self::BadPath => "chemin de fichier invalide",
            Self::NotUtf8 => "le fichier n'est pas en UTF-8",
            Self::RonSyntax => "RON mal formé ou fiche non conforme",
            Self::RecursionLimit => "imbrication RON trop profonde",
            Self::MissingField => "champ obligatoire absent",
            Self::UnknownField => "champ inconnu",
            Self::UnknownKind => "type de fiche inconnu",
            Self::EncodingOverflow => "longueur trop grande pour l'encodage",
            Self::InvalidId => "identifiant invalide",
            Self::InvalidName => "nom invalide",
            Self::DuplicateId => "identifiant déjà utilisé",
            Self::BrokenReference => "référence vers une fiche objet introuvable",
            Self::WrongNamespace => {
                "l'identifiant ne commence pas par l'espace de noms de sa sorte"
            }
            Self::RegistrySyntax => "ligne du registre des identifiants mal formée",
            Self::RegistryDuplicate => "identifiant présent deux fois dans le registre",
            Self::Unregistered => "identifiant absent du registre ids.lock",
            Self::RetiredReused => "identifiant retiré réutilisé",
            Self::ActiveWithoutCard => {
                "identifiant actif sans fiche : le marquer retiré dans le registre"
            }
            Self::RegistryRewritten => {
                "registre réécrit : ligne supprimée, déplacée ou modifiée (ajout seul)"
            }
        }
    }
}

/// Longest `detail` kept: text from the input is never copied whole into an error.
const MAX_DETAIL: usize = 80;
/// Diagnostics kept in a [`LoadErrors`]; the rest is dropped.
pub const MAX_DIAGNOSTICS: usize = 64;

// spec: carnet architecture-securite.md SEC0-26
/// One error: stable code, file relative to the content root (with `/`, never an absolute
/// path), 1-based position (0 when it does not apply), and a short detail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Stable code.
    pub code: Code,
    /// Relative path of the file, escaped like `detail`; empty for a pack-wide error.
    pub file: String,
    /// 1-based line, 0 if unknown.
    pub line: u32,
    /// 1-based column, 0 if unknown.
    pub column: u32,
    /// Short context (an identifier or a field name): at most 80 characters of the input, escaped.
    pub detail: String,
}

impl Diagnostic {
    /// Builds a diagnostic; `file` and `detail` are escaped (no newline or terminal escape
    /// survives), and `detail` is cut to its first 80 characters first.
    pub fn new(code: Code, file: &str, position: (u32, u32), detail: &str) -> Self {
        Self {
            code,
            file: file.chars().flat_map(char::escape_debug).collect(),
            line: position.0,
            column: position.1,
            // Input text is escaped: no newline or terminal escape can forge a line of output.
            detail: detail
                .chars()
                .take(MAX_DETAIL)
                .flat_map(char::escape_debug)
                .collect(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: [{}] {}",
            self.file,
            self.line,
            self.column,
            self.code.as_str(),
            self.code.message_fr()
        )?;
        if !self.detail.is_empty() {
            write!(f, " ({})", self.detail)?;
        }
        Ok(())
    }
}

/// A failed load: at least one diagnostic, at most [`MAX_DIAGNOSTICS`] kept.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadErrors {
    diagnostics: Vec<Diagnostic>,
    dropped: usize,
}

impl LoadErrors {
    /// Wraps a first diagnostic and the others; beyond [`MAX_DIAGNOSTICS`] they are counted,
    /// not kept.
    pub fn new(first: Diagnostic, others: Vec<Diagnostic>) -> Self {
        let mut diagnostics = vec![first];
        diagnostics.extend(others);
        let dropped = diagnostics.len().saturating_sub(MAX_DIAGNOSTICS);
        diagnostics.truncate(MAX_DIAGNOSTICS);
        Self {
            diagnostics,
            dropped,
        }
    }

    /// Wraps one diagnostic.
    pub fn single(d: Diagnostic) -> Self {
        Self::new(d, Vec::new())
    }

    /// The diagnostics kept, never empty.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// How many further diagnostics were dropped.
    pub fn dropped(&self) -> usize {
        self.dropped
    }
}

/// Collects diagnostics up to [`MAX_DIAGNOSTICS`] and counts the rest without building them,
/// so a hostile input cannot make the error path expensive.
#[derive(Debug, Default)]
pub(crate) struct Sink {
    items: Vec<Diagnostic>,
    dropped: usize,
}

impl Sink {
    pub(crate) fn push_with(&mut self, make: impl FnOnce() -> Diagnostic) {
        if self.items.len() < MAX_DIAGNOSTICS {
            self.items.push(make());
        } else {
            self.dropped = self.dropped.saturating_add(1);
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn into_errors(self) -> Option<LoadErrors> {
        let mut items = self.items.into_iter();
        let first = items.next()?;
        let mut errors = LoadErrors::new(first, items.collect());
        errors.dropped = errors.dropped.saturating_add(self.dropped);
        Some(errors)
    }
}

impl fmt::Display for LoadErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in &self.diagnostics {
            writeln!(f, "{d}")?;
        }
        if self.dropped > 0 {
            writeln!(f, "... et {} autre(s) erreur(s)", self.dropped)?;
        }
        Ok(())
    }
}

impl core::error::Error for LoadErrors {}
