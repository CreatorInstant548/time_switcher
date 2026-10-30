//! Script de construction de `xtask` (J2, AS-23 point 1), sans dépendance, sans réseau et sans
//! écriture : il lit les entrées de la construction (`build_inputs`), déclare
//! `rerun-if-changed` sur chacune, et inscrit leur empreinte dans le binaire
//! (`XTASK_INPUTS_SHA256`). `merge` la compare à celle de `HEAD` de la copie principale.
//! spec: carnet architecture-securite.md §9.8 J2

#![forbid(unsafe_code)]
// Script de construction : il parle à cargo par la sortie standard et lit les fichiers du dépôt.
#![allow(clippy::print_stdout, clippy::disallowed_methods)]

#[path = "src/sha256.rs"]
#[allow(dead_code)]
mod sha256;

#[path = "src/build_inputs.rs"]
mod build_inputs;

use std::path::{Path, PathBuf};

/// Fichiers sous `dir`, chemins relatifs à `root` avec `/` ; un lien est lu comme le fichier visé,
/// et `merge` le refusera, puisque Git n'y voit pas le même contenu.
fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) -> Result<(), String> {
    let err = |e: std::io::Error| format!("{} : {e}", dir.display());
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(err)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<_, _>>()
        .map_err(err)?;
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(root, &p, out)?;
        } else {
            let rel = p.strip_prefix(root).map_err(|e| e.to_string())?;
            let rel = rel.to_string_lossy().replace('\\', "/");
            if !build_inputs::is_input(&rel) {
                return Err(format!("{rel} : hors des entrées de construction"));
            }
            out.push((
                rel,
                std::fs::read(&p).map_err(|e| format!("{} : {e}", p.display()))?,
            ));
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?;
    let root = Path::new(&manifest).join("..").join("..");
    let mut files = Vec::new();
    for d in build_inputs::DIRS {
        let dir = root.join(d);
        if dir.is_dir() {
            println!("cargo:rerun-if-changed={}", dir.display());
            walk(&root, &dir, &mut files)?;
        }
    }
    for f in build_inputs::FILES {
        let path = root.join(f);
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
            files.push((
                f.to_owned(),
                std::fs::read(&path).map_err(|e| e.to_string())?,
            ));
        }
    }
    println!(
        "cargo:rustc-env=XTASK_INPUTS_SHA256={}",
        build_inputs::fingerprint(&files)
    );
    Ok(())
}
