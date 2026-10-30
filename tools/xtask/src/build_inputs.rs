//! Entrées de la construction de `xtask` et leur empreinte (J2, AS-23 point 1). Ce module est
//! partagé par `build.rs`, qui inscrit l'empreinte dans le binaire, et par `merge`, qui la
//! recalcule sur `HEAD` de la copie principale : un binaire qui n'a pas été construit depuis ces
//! entrées exactes est refusé, sans dépendre d'aucune horloge.
//! spec: carnet architecture-securite.md §9.8 J2

/// Dossiers d'entrée (tout fichier dessous) et fichiers d'entrée, depuis la racine du dépôt.
pub const DIRS: [&str; 2] = ["tools/xtask/", ".cargo/"];
pub const FILES: [&str; 4] = [
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "rust-toolchain",
];

/// Chemin du dépôt (séparateur `/`) qui est une entrée.
pub fn is_input(path: &str) -> bool {
    DIRS.iter().any(|d| path.starts_with(d)) || FILES.contains(&path)
}

/// SHA-256 de la liste « chemin NUL sha256 LF », triée par chemin.
pub fn fingerprint(files: &[(String, Vec<u8>)]) -> String {
    let mut lines: Vec<String> = files
        .iter()
        .map(|(p, b)| format!("{p}\0{}\n", crate::sha256::hex(b)))
        .collect();
    lines.sort();
    crate::sha256::hex(lines.concat().as_bytes())
}
