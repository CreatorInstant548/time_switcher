//! Lecture de `cargo metadata --format-version 1 --no-deps`, réduite aux champs lus (AS-17).
//! Les champs inconnus sont ignorés.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub packages: Vec<Package>,
    pub workspace_root: String,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub manifest_path: String,
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    /// `None` pour une dépendance normale, sinon `dev` ou `build`.
    pub kind: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pub kind: Vec<String>,
    pub src_path: String,
}

pub fn load(root: &Path) -> Result<Metadata, String> {
    let args = ["metadata", "--format-version", "1", "--no-deps", "--locked"];
    let json = crate::sys::stdout_of(root, &crate::sys::cargo(), &args)?;
    parse(&json)
}

pub fn parse(json: &str) -> Result<Metadata, String> {
    serde_json::from_str(json).map_err(|e| format!("sortie de cargo metadata illisible : {e}"))
}

fn slashes(path: &str) -> String {
    path.replace('\\', "/")
}

impl Metadata {
    pub fn package(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }

    /// Dossier d'une crate, relatif à la racine, avec des `/`.
    pub fn rel_dir(&self, pkg: &Package) -> String {
        let root = slashes(&self.workspace_root);
        let manifest = slashes(&pkg.manifest_path);
        let dir = manifest.strip_suffix("/Cargo.toml").unwrap_or(&manifest);
        dir.strip_prefix(&root)
            .unwrap_or(dir)
            .trim_start_matches('/')
            .to_owned()
    }

    /// Crate qui contient un fichier (chemin relatif à la racine) : le dossier le plus long gagne.
    pub fn package_of(&self, file: &str) -> Option<&str> {
        self.packages
            .iter()
            .map(|p| (self.rel_dir(p), p.name.as_str()))
            .filter(|(dir, _)| file.starts_with(&format!("{dir}/")))
            .max_by_key(|(dir, _)| dir.len())
            .map(|(_, name)| name)
    }

    /// Crates de l'espace de travail atteintes par les dépendances normales, départ compris.
    pub fn normal_closure(&self, start: &str) -> BTreeSet<String> {
        self.closure([start.to_owned()].into(), |p, d| {
            (d.kind.is_none() && d.name != p.name).then(|| d.name.clone())
        })
    }

    /// Crates touchées et tous leurs dépendants, directs ou non, de toute nature (F7).
    pub fn with_dependents(&self, touched: BTreeSet<String>) -> BTreeSet<String> {
        let mut set = touched;
        loop {
            let before = set.len();
            for p in &self.packages {
                if p.dependencies.iter().any(|d| set.contains(&d.name)) {
                    set.insert(p.name.clone());
                }
            }
            if set.len() == before {
                return set;
            }
        }
    }

    fn closure(
        &self,
        mut set: BTreeSet<String>,
        edge: impl Fn(&Package, &Dependency) -> Option<String>,
    ) -> BTreeSet<String> {
        let mut todo: Vec<String> = set.iter().cloned().collect();
        while let Some(name) = todo.pop() {
            let Some(p) = self.package(&name) else {
                continue;
            };
            for next in p.dependencies.iter().filter_map(|d| edge(p, d)) {
                if self.package(&next).is_some() && set.insert(next.clone()) {
                    todo.push(next);
                }
            }
        }
        set
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn sample() -> Metadata {
        parse(
            r#"{"workspace_root":"C:\\w","version":1,"packages":[
            {"name":"ts_kernel","manifest_path":"C:\\w\\crates\\ts_kernel\\Cargo.toml",
             "features":{"default":[],"public":[],"dev-commands":[]},"dependencies":[],
             "targets":[{"kind":["lib"],"src_path":"C:\\w\\crates\\ts_kernel\\src\\lib.rs"}]},
            {"name":"ts_model","manifest_path":"C:\\w\\crates\\ts_model\\Cargo.toml",
             "dependencies":[{"name":"ts_kernel","kind":null,"features":[]},
                             {"name":"ts_toy","kind":"dev","features":[]}]},
            {"name":"ts_toy","manifest_path":"C:\\w\\crates\\ts_toy\\Cargo.toml",
             "dependencies":[{"name":"ts_model","kind":null}]},
            {"name":"xtask","manifest_path":"C:\\w\\tools\\xtask\\Cargo.toml","dependencies":[]}]}"#,
        )
        .unwrap()
    }

    #[test]
    fn maps_files_to_packages() {
        let m = sample();
        assert_eq!(
            m.package_of("crates/ts_kernel/src/lib.rs"),
            Some("ts_kernel")
        );
        assert_eq!(m.package_of("tools/xtask/src/main.rs"), Some("xtask"));
        assert_eq!(m.package_of("crates/ts_kernel_x/a.rs"), None);
        assert_eq!(m.package_of("docs/a.md"), None);
    }

    #[test]
    fn closures_follow_the_right_edges() {
        let m = sample();
        let names = |s: BTreeSet<String>| s.into_iter().collect::<Vec<_>>();
        assert_eq!(
            names(m.normal_closure("ts_model")),
            ["ts_kernel", "ts_model"]
        );
        let deps = m.with_dependents(["ts_kernel".to_owned()].into());
        assert_eq!(names(deps), ["ts_kernel", "ts_model", "ts_toy"]);
    }
}
