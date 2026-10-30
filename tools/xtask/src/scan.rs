//! Chemins locaux dans les fichiers du dépôt (SEC0-26, F6). Fonction pure, sans E/S.
//!
//! Ce fichier est exempté des motifs de chemins fixes (liste fermée `LOCAL_PATH_EXEMPT`) : il les
//! définit et les teste. Les noms d'utilisateur et de machine y restent recherchés.

/// Fichiers exemptés des motifs de chemins fixes (F6), en liste fermée.
pub const LOCAL_PATH_EXEMPT: [&str; 1] = ["tools/xtask/src/scan.rs"];

/// Numéros de ligne où figure un chemin de dossier personnel (`/home/`, `\Users\`, séparateurs
/// mêlés, sans tenir compte de la casse), sauf fichier exempté, ou l'un des noms donnés
/// (utilisateur, machine), recherchés partout.
pub fn local_paths(bytes: &[u8], names: &[String], exempt: bool) -> Vec<usize> {
    let lower = bytes.to_ascii_lowercase();
    let is_sep = |b: Option<&u8>| matches!(b, Some(b'/' | b'\\'));
    let mut lines = Vec::new();
    let mut line = 1;
    for i in 0..lower.len() {
        if lower[i] == b'\n' {
            line += 1;
            continue;
        }
        let home = !exempt
            && i > 0
            && is_sep(lower.get(i - 1))
            && ["users", "home"]
                .iter()
                .any(|w| lower[i..].starts_with(w.as_bytes()) && is_sep(lower.get(i + w.len())));
        // Nom entier seulement : `runner` ne doit pas trouver `ts_runner`.
        let word = |b: Option<&u8>| b.is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_');
        let name = names.iter().any(|n| {
            lower[i..].starts_with(n.as_bytes())
                && !(i > 0 && word(lower.get(i - 1)))
                && !word(lower.get(i + n.len()))
        });
        if (home || name) && lines.last() != Some(&line) {
            lines.push(line);
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_local_paths() {
        let text = b"a\nC:\\Users\\x\nc:/users/x\n/home/x\nD:\\HOME/x\nhomework /users\n";
        assert_eq!(local_paths(text, &[], false), [2, 3, 4, 5]);
        assert!(local_paths(text, &[], true).is_empty());
        let names = ["pc-42".to_owned()];
        assert_eq!(local_paths(b"ok\nmachine PC-42\n", &names, true), [2]);
        let user = ["runner".to_owned()];
        assert!(local_paths(b"ts_runner runners", &user, false).is_empty());
        assert_eq!(local_paths(b"x\n/c/runner/", &user, true), [2]);
    }
}
