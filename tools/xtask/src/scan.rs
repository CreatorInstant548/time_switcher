//! Chemins locaux dans les fichiers du dépôt (SEC0-26, F6). Fonction pure, sans E/S.
//!
//! Ce fichier est exempté des motifs de chemins fixes (liste fermée `LOCAL_PATH_EXEMPT`) : il les
//! définit et les teste. Les noms d'utilisateur et de machine y restent recherchés.

/// Fichiers exemptés des motifs de chemins fixes (F6), en liste fermée.
pub const LOCAL_PATH_EXEMPT: [&str; 1] = ["tools/xtask/src/scan.rs"];

/// UTF-16 text with a byte order mark, decoded to UTF-8 (m3); `None` for anything else.
fn utf16_to_utf8(bytes: &[u8]) -> Option<Vec<u8>> {
    let little = match bytes {
        [0xFF, 0xFE, ..] => true,
        [0xFE, 0xFF, ..] => false,
        _ => return None,
    };
    let units = bytes[2..].as_chunks::<2>().0.iter().map(|c| match little {
        true => u16::from_le_bytes([c[0], c[1]]),
        false => u16::from_be_bytes([c[0], c[1]]),
    });
    let text: String = char::decode_utf16(units)
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect();
    Some(text.into_bytes())
}

/// Numéros de ligne où figure un chemin de dossier personnel (`/home/`, `\Users\`, séparateurs
/// mêlés, sans tenir compte de la casse), sauf fichier exempté, ou l'un des noms donnés
/// (utilisateur, machine), recherchés partout.
///
/// Un fichier UTF-16 (m3) est lu aussi : décodé s'il commence par une marque d'ordre des octets, et,
/// sinon, dès qu'il contient des octets nuls, en les retirant (le texte ASCII d'un UTF-16 sans
/// marque, ou d'un UTF-32, redevient lisible). Les numéros de ligne sont ceux du fichier.
pub fn local_paths(bytes: &[u8], names: &[String], exempt: bool) -> Vec<usize> {
    let mut lines = scan(bytes, names, exempt);
    if let Some(decoded) = utf16_to_utf8(bytes) {
        lines.extend(scan(&decoded, names, exempt));
    }
    if bytes.contains(&0) {
        let plain: Vec<u8> = bytes.iter().copied().filter(|&b| b != 0).collect();
        lines.extend(scan(&plain, names, exempt));
    }
    lines.sort_unstable();
    lines.dedup();
    lines
}

fn scan(bytes: &[u8], names: &[String], exempt: bool) -> Vec<usize> {
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

    /// m3: UTF-16 and UTF-32 text hides nothing, with or without a byte order mark.
    #[test]
    fn finds_paths_in_utf16() {
        let text = "a\r\nC:\\Users\\x\r\nb\r\nmachine PC-42\r\n";
        let le = |bom: bool| -> Vec<u8> {
            let units = text.encode_utf16().flat_map(u16::to_le_bytes);
            [if bom { vec![0xFF, 0xFE] } else { vec![] }, units.collect()].concat()
        };
        let be = |bom: bool| -> Vec<u8> {
            let units = text.encode_utf16().flat_map(u16::to_be_bytes);
            [if bom { vec![0xFE, 0xFF] } else { vec![] }, units.collect()].concat()
        };
        let names = ["pc-42".to_owned()];
        for bytes in [le(true), le(false), be(true), be(false)] {
            assert_eq!(local_paths(&bytes, &names, false), [2, 4]);
            assert_eq!(local_paths(&bytes, &names, true), [4]);
        }
        let utf32: Vec<u8> = text
            .chars()
            .flat_map(|c| u32::from(c).to_le_bytes())
            .collect();
        assert_eq!(local_paths(&utf32, &[], false), [2]);
        // A non-ASCII name is found when the byte order mark lets the text be decoded.
        let accent = ["élodie".to_owned()];
        let line: Vec<u8> = [
            vec![0xFF, 0xFE],
            "x\nélodie\n"
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect(),
        ]
        .concat();
        assert_eq!(local_paths(&line, &accent, false), [2]);
        // Clean UTF-16 stays clean.
        assert!(local_paths(&le(true)[..8], &names, false).is_empty());
    }
}
