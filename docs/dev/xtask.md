# `cargo xtask`

Mode d'emploi de la crate d'outil `tools/xtask` (tâche T2b de la phase 0). Sources :
- [revue du développement par agents](../conception/revues/developpement-agents.md) §4.2 ;
- les exigences SEC0-03, 06, 08, 19, 20, 26 et 29 à 31 du carnet du responsable architecture et sécurité, avec les corrections F1 à F7, F9 et F10 de son §9.5.

`tools/xtask/` est un **chemin sensible** : toute modification passe par `/security-review`, un relecteur neuf et l'avis d'archi-sécu.

L'alias `cargo xtask` vaut `cargo run --locked -p xtask --` (`.cargo/config.toml`). `xtask` n'est pas dans `default-members` : il se compile à la demande. Avant la première commande d'un worktree, prendre un emplacement : `pwsh -NoProfile -File tools/dev/session-start.ps1 -Acquire` ([outillage](outillage.md) §1).

## 1. `cargo xtask check`

```
cargo xtask check                      # crates touchées depuis origin/main
cargo xtask check --all                # tout l'espace de travail
cargo xtask check --only workflows     # étapes nommées seulement, séparées par des virgules
cargo xtask check --list               # noms stables des étapes
```

C'est la vérification à lancer avant toute PR ; elle remplace la liste du §1 d'[espace-travail](espace-travail.md). Les étapes tournent dans l'ordre ci-dessous, et la première qui échoue arrête la suite. Chaque étape affiche `OK`, `IGNORÉE : raison` ou `ÉCHEC : raison`, puis un récapitulatif vient à la fin. Le code de sortie vaut 0 seulement si aucune étape n'échoue ; une option ou une étape inconnue rend 2.

**Jamais de réussite silencieuse** : un outil absent, une commande qui ne se lance pas ou une sortie illisible font échouer l'étape. Une étape n'est ignorée que dans les cas écrits ci-dessous, et elle le dit.

| Nom | Contrôle | Référence |
|---|---|---|
| `fmt` | `cargo fmt --all --check` | |
| `clippy` | `cargo clippy --workspace --all-targets --locked -- -D warnings`, puis la même passe sur le **graphe livré** seul avec `--features ts_kernel/public` : `ts_kernel`, les hôtes présents et leurs dépendances normales de l'espace de travail, sans `--workspace` ni `--all-targets`. Enfin, une passe `--all-targets` sur les crates de cœur par fonctionnalité de développement, l'une après l'autre (`--features ts_kernel/dev-commands`, puis les trois autres) : le code sous `#[cfg(feature = "dev-…")]` est vu aussi. Jamais `--all-features` | F1, I2 |
| `test` | `cargo test --locked` sur les crates touchées **et leurs dépendants**, directs ou non. Une modification de `Cargo.toml` racine, `Cargo.lock`, `clippy.toml`, `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml` ou `.cargo/` vaut `--all`. Ignorée si aucune crate n'est touchée | F7 |
| `deny` | version de `cargo-deny` comparée au `tools/dev/tools.lock` du dépôt (lisible en CI), puis `cargo deny --locked check`, à chaque exécution | SEC0-03, F9 |
| `audit` | version de `cargo-audit` comparée à `tools.lock`, puis `cargo audit`. Lancé avec `--all`, si `Cargo.lock` change, ou si la base RustSec locale a plus de 7 jours ; ignoré sinon. Si la mise à jour de la base échoue, un second essai `--no-fetch` n'est permis qu'avec une base de moins de 7 jours | SEC0-03, F9 |
| `gitleaks` | binaire et empreinte SHA-256 lus dans le `tools.lock` de la **copie épinglée** désignée par `core.hooksPath`, qui doit être `TS_CACHE/hooks/<current.sha256>` ; configuration de cette même copie, jamais celle de la copie de travail ; `.gitleaksignore` à la racine refusé ; `--redact`. Arbre : `git ls-files -co --exclude-standard`, extrait dans `TS_CACHE\tmp`, sans `target/` ni lien. Commits : `origin/main..HEAD`, avec `--text --no-ext-diff --no-textconv` | SEC0-06, SEC0-08, F2 |
| `unsafe` | les racines de chaque crate (cibles `lib`, `bin`, `proc-macro` et `custom-build` de `cargo metadata`, `xtask` compris) contiennent `#![forbid(unsafe_code)]` ; `ts_client_native` est exempté, et `ts_py` est en `deny`, avec un `// SAFETY:` avant chaque `allow`. Chaque `Cargo.toml` membre contient `[lints] workspace = true`, sans `[lints.*]` local | SEC0-19, F3 |
| `profiles` | `cargo check -p ts_kernel --locked --features public,<f>` échoue **sur notre message** `compile_error!`, pour chacune des 4 fonctionnalités de développement | SEC0-20, E8 |
| `features` | aucune mention de `public` hors de sa déclaration dans `ts_kernel` (dont `ts_kernel/public` et `ts_kernel?/public`) ; `default` de `ts_kernel` vide ; aucun hôte n'active une fonctionnalité de développement, vérifié aussi par `cargo tree -p <hôte> --edges normal,features` ; aucune crate hors outils ne dépend de `ts_toy`, directement ou non, en dépendance normale | SEC0-20, F4, D-DEV-02 |
| `clippy-exemptions` | dans toute crate autre qu'un hôte ou un outil, chaque `.rs` est d'abord **normalisé** comme rustc le lit : commentaires `//` et `/* */` (imbriqués, doc comprise) retirés, préfixes `r#` des identifiants retirés, chaînes et caractères gardés mais ignorés. Sont ensuite refusés, hors chaîne :<br>- **tout identifiant** `disallowed_*` (`disallowed_methods`, `disallowed_types`, `disallowed_macros`, `disallowed_names` et les anciens noms, `blacklisted_name` compris), quel que soit le contexte : attribut, argument de macro, accolades ;<br>- **l'identifiant** `include`, `include_str` ou `include_bytes`, partout, `use` compris ;<br>- `path = …` dans un attribut (`#[path]`, `cfg_attr(…, path = …)`) ;<br>- `allow(…)` ou `expect(…)` de `clippy::all`, `clippy::style` ou `warnings`, quel que soit le caractère qui précède (sauf `.`), et tout `$` dans les arguments d'un `allow`, `expect`, `warn` ou `deny` ;<br>- dans `cfg(…)`, `cfg!(…)` et le prédicat de `cfg_attr(…)`, tout prédicat hors de la **liste fermée** `test`, `feature = "…"`, `all`, `any`, `not` (donc `clippy`, `debug_assertions`, `target_*`, `doc`…) ;<br>- tout fichier autre que `.rs` sous `src/` (liste fermée `NON_RS_SRC_EXEMPT`, vide).<br>Aucun `clippy.toml` ni `.clippy.toml` hors de la racine ; `CLIPPY_CONF_DIR` refusé | E6, F5, I2 |
| `local-paths` | dans les fichiers suivis et non ignorés : aucun dossier personnel, c'est-à-dire `Users` ou `home` entre deux séparateurs `/` ou `\`, quelle que soit la casse, ni la valeur de `USERNAME` ou de `COMPUTERNAME` en mot entier. Le résultat donne le fichier et la ligne, jamais la valeur | SEC0-26, F6 |
| `workflows` | chaque fichier de `.github/workflows/` : `permissions: {}` au niveau du workflow ; actions de la liste fermée, épinglées par SHA de 40 caractères ; `actions/checkout` avec `persist-credentials: false` sous son `with:` ; ni `pull_request_target`, ni `workflow_run`, ni `secrets: inherit`. Clés comparées sans tenir compte de la casse (`USES:`, `Run:`).<br>**Interpolations** : seule `${{ runner.… }}` est sûre. Toute autre (`github`, `inputs`, `matrix`, `env`, `steps`, `needs`, fonctions comme `toJSON(github)`), quelle que soit la casse, n'est permise que dans les valeurs sous `env:` ou `with:`, et dans `if:` ; jamais dans un `run:`, qui la reçoit par `env:` et une variable citée.<br>**Toute ligne non reconnue est refusée** : seules sont acceptées `clé: valeur`, `- clé: valeur`, `- scalaire`, les commentaires et le contenu des blocs `|` ou `>`. Sont ainsi refusés : clés explicites `?` et `:`, ancres, alias et balises (`&`, `*`, `!`), collections en ligne autres que `{}`, `[]` ou une liste de mots, `uses:` ou `run:` hors d'une clé de tête de ligne, indentation incohérente (enfant d'une valeur, retrait hors d'un niveau ouvert, frères désalignés), tabulation, `---`, et barre oblique inverse hors d'un bloc, puisque YAML y décode les échappements. Ignorée s'il n'y a aucun workflow | SEC0-29 à 31, F10, I1 |
| `spec-map` | `spec-map check`, dès que la sous-commande existe (T4). D'ici là, l'étape est ignorée avec un avertissement | |

Les étapes `deny`, `gitleaks`, `unsafe` et `features` à `workflows` portent **toujours sur tout le dépôt**, quel que soit le périmètre (F7).

**Arbre lu** (I3) : `git ls-files -z -co --exclude-standard`, découpé sur NUL, donc sans citation des noms. Chaque fichier doit être un fichier ordinaire présent sur le disque. Un lien symbolique (hors de la liste fermée `SYMLINK_EXEMPT`, vide), un fichier suivi mais supprimé, ou un autre objet est un écart, qui fait échouer toute étape qui lit l'arbre : `gitleaks`, `unsafe` pour `ts_py`, `clippy-exemptions`, `local-paths` et `workflows`. Les crates touchées se lisent aussi en `-z`.

## 2. Listes tenues dans le code

Elles sont à compléter **dans la PR qui crée la crate concernée** :
- `manifests::HOSTS` : crates d'hôte (`ts_host`, `ts_server`, `ts_client_native`) ;
- `manifests::TOOLS` : crates d'outil (`ts_py`, `ts_runner`, `ts_viz`, `xtask`). Les hôtes et les outils sont les seules crates qui peuvent s'exempter des interdictions de clippy. Les outils sont aussi les seules qui peuvent dépendre de `ts_toy`. **Toute autre crate est traitée comme une crate de cœur ou de règles** (refus par défaut) ;
- `scan::LOCAL_PATH_EXEMPT` : fichiers exemptés des motifs de chemins fixes, c'est-à-dire seulement le fichier qui définit et teste ces motifs. Les noms d'utilisateur et de machine y restent recherchés.

## 3. Dépendances et exemptions

- **Dépendances** : `serde` (avec la dérivation) et `serde_json`, pour lire `cargo metadata --format-version 1 --no-deps` dans des structures réduites aux champs lus (AS-17 : `serde_json` pour les crates d'outil seulement). Il n'y a ni `clap`, puisque l'analyse des options tient en quelques lignes, ni crate de cryptographie : le SHA-256 de gitleaks est calculé par `certutil` sous Windows et par `sha256sum` ailleurs.
- **Interdictions de clippy** : `xtask` lit les fichiers, l'environnement, la ligne de commande et l'horloge. Ces accès sont regroupés dans `sys.rs`, avec une exemption par fonction, commentée (E6).
- **Sortie** : les commandes affichées montrent le nom de l'outil, sans son chemin complet, qui contiendrait le dossier personnel. Seule la sortie propre des outils peut le contenir : par exemple, `cargo audit` donne le chemin de sa base. **Avant de coller la sortie dans une PR, remplacer le dossier personnel par `%USERPROFILE%`.**
