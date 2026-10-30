# Espace de travail Cargo

Mode d'emploi de l'espace de travail (tâche T2a de la phase 0). Sources : [revue du développement par agents](../conception/revues/developpement-agents.md) §4.2 (A12 à A14), exigences SEC0-01, 02, 05, 07, 19, 20 et 22 et corrections E1 à E9 du carnet du responsable architecture et sécurité.

## 1. Commandes

Avant la première commande `cargo` d'un worktree : `pwsh -NoProfile -File tools/dev/session-start.ps1 -Acquire` ([outillage](outillage.md) §1). Jusqu'à `cargo xtask check` (T2b), les vérifications avant une PR sont :

```
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo deny check
cargo audit
cargo check -p ts_kernel --locked --features public                # doit réussir
cargo check -p ts_kernel --locked --features public,dev-commands   # doit échouer sur notre compile_error!
```

La dernière se répète avec `dev-introspection`, `external-pilot` et `dev-bench`.

**Jamais `--all-features`** (E3), ni en local, ni en CI, ni dans `xtask` : elle active `public` avec les fonctionnalités de développement et déclenche le `compile_error!`. Les combinaisons se testent une à une. Seul `cargo deny` lit toutes les fonctionnalités (`[graph] all-features = true`), sans rien compiler.

`--locked` partout : `Cargo.lock` est commité (SEC0-01). `default-members` ne contient que le cœur (A13) ; les crates lourdes ou facultatives (`ts_viz`, `ts_py`, `ts_client_native`, `xtask`) se compilent seulement avec `-p`.

## 2. Profils

| Profil | Usage | Réglages |
|---|---|---|
| `dev` | worktrees, tests | `opt-level = 1`, `debug = "line-tables-only"`, incrémental ; dépendances en `opt-level = 2` sans débogage |
| `ci` | CI | hérite de `dev`, non incrémental, sans débogage |
| `sim` | `ts-sim`, déterminisme long, bancs | hérite de `release`, `codegen-units = 16` ; **copie principale et CI seulement** |
| `release` | publication | `lto = "thin"`, sans débogage ; **jamais dans un worktree** |

`overflow-checks = true` dans tous les profils (E2), `test` et `bench` compris par héritage : un dépassement d'entier panique au lieu d'enrouler, ce qui empêche une création d'argent silencieuse. `panic` reste `unwind`.

La chaîne est épinglée par `rust-toolchain.toml` (1.99.1, A14), changée seulement par une PR dédiée.

## 3. Profil de construction de `ts_kernel` (SEC0-20)

- Fonctionnalités `dev-commands`, `dev-introspection`, `external-pilot`, `dev-bench` et `public`, toutes vides et aucune par défaut.
- `public` exclut les quatre autres, par un `compile_error!` en français pour chaque paire.
- `public` ne s'active **dans aucun `Cargo.toml`** : seulement en ligne de commande, par la future `xtask release`. Les fonctionnalités de développement ne sont activées par aucune dépendance normale d'une crate d'hôte.
- `ts_kernel::BUILD_PROFILE` vaut `"public"` si et seulement si `public` est active, sinon `"dev"` (E9). `ts_kernel::active_dev_features()` liste les fonctionnalités de développement compilées, pour le refus de démarrer et la vérification de l'artefact.

## 4. Dépendances

- **Une ligne triée par dépendance** dans `[workspace.dependencies]` du `Cargo.toml` racine ; les crates y renvoient par `workspace = true`.
- Toute nouvelle dépendance directe est **justifiée dans la PR** (SEC0-05) : besoin, alternative écartée, licence, présence d'un `build.rs` ou d'une macro procédurale. Une crate de cryptographie, de décodage de format externe ou de réseau hors de la liste pré-approuvée demande l'avis d'archi-sécu avant la fusion.
- `deny.toml` (format v2) : crates.io seulement, aucun dépôt git ; licences en liste blanche, GPL, LGPL et AGPL refusées ; vulnérabilités, versions retirées et crates non maintenues refusées ; crates réseau et TLS refusées en phase 0 ; versions `*` refusées. Une entrée `ignore` porte l'identifiant, la raison et la date, et se revoit chaque semaine.
- Interdits (E7) : sections `[source]`, `[registries]`, `[patch]` et `[replace]`, `linker`, `rustflags` et `target-dir`, dans `Cargo.toml` comme dans `.cargo/config.toml`.

## 5. Lints et interdictions de clippy

- `unsafe_code = "forbid"` pour tout l'espace de travail, et `#![forbid(unsafe_code)]` dans chaque crate (SEC0-19). Seules exceptions, en liste fermée : `ts_client_native`, et `ts_py` si `pyo3` l'exige.
- Chaque crate déclare `[lints] workspace = true`. Les lints de clippy sont en `warn`, et `-D warnings` les rend bloquants. `unwrap` et `expect` restent permis dans les tests.
- `clippy.toml` interdit l'horloge (`Instant::now`, `SystemTime::now`, `elapsed`, `chrono::Utc::now`, `Local::now`), l'environnement et la ligne de commande (`env::var`, `var_os`, `vars`, `args`), les fichiers (`fs::read`, `read_to_string`, `read_dir`, `write`, `File::open`, `File::create`, `OpenOptions::open`), l'aléa du système (fonctions et types de `rand` et de `getrandom`, `SeedableRng::from_os_rng`, `RandomState::new`) et les tables `HashMap` et `HashSet` de `std` et de `hashbrown`. Chaque interdiction porte sa raison ; les chemins des crates absentes du graphe portent `allow-invalid = true`.
- **Exemption locale : crates d'hôte et d'outil seulement** (E6), par `#[allow(clippy::disallowed_methods)]` ou `#[allow(clippy::disallowed_types)]` sur l'élément le plus petit possible, avec un commentaire qui dit pourquoi. Elle est **refusée** dans `ts_kernel`, `ts_model`, `ts_data` et les crates de règles ; `xtask check` le vérifiera (T2b).
