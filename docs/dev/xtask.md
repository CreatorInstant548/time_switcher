# `cargo xtask`

Mode d'emploi de la crate d'outil `tools/xtask` (tâches T2b et T2c de la phase 0). Sources :
- [revue du développement par agents](../conception/revues/developpement-agents.md) §4.2, §6 et §9.2 ;
- les exigences SEC0-03, 06, 08, 19, 20, 26 et 29 à 31 du carnet du responsable architecture et sécurité, avec les corrections F1 à F10 de son §9.5.

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

## 4. `cargo xtask slots`

```
cargo xtask slots
```

**Lecture seule.** Pour chacun des emplacements `main`, `w1`, `w2` et `w3` de `TS_CACHE\target`, il affiche :
- l'état du bail : `principal` pour `main`, qui n'a pas de bail, puis `tenu`, `libre` ou `orphelin`, selon la règle de `session-start.ps1` (bail de moins de 10 minutes, ou dont le worktree a encore un `.git`) ;
- le propriétaire écrit dans le bail ;
- la taille, mesurée sans suivre aucun lien ;
- la jonction : les worktrees de `git worktree list` dont `target` mène à cet emplacement, ou `celle du bail`.

Il affiche enfin l'espace libre du disque de `TS_CACHE`, avec un avertissement sous 40 Go. Il n'attribue ni ne libère rien : c'est le rôle de `tools/dev/session-start.ps1 -Acquire` et `-Release` ([outillage](outillage.md) §1).

## 5. `cargo xtask disk [--clean]`

```
cargo xtask disk           # simulation : affiche ce qui serait fait, ne modifie rien
cargo xtask disk --clean   # nettoyage (§9.2 de la revue), par la session principale, chaque semaine
```

Sans `--clean`, **rien n'est modifié** : `git worktree prune` tourne avec `-n`, et chaque suppression est annoncée par « supprimerait ». Dans l'ordre :

1. **Bilan** : le tableau de `slots`, la taille de `TS_CACHE\artifacts`, de `.claude\worktrees` et des archives de Cargo (`registry\cache` seulement, car le parcours de `registry\src` prend une quinzaine de secondes), la liste des worktrees et l'espace libre.
2. **`git worktree prune -v`**, puis nouvelle lecture de `git worktree list`. Si la liste est illisible, rien n'est supprimé.
3. **Baux orphelins** (`wN.lease` et `main.lease`) : libérés par renommage atomique puis suppression, comme `Remove-OrphanLease`. Un bail redevenu tenu entre-temps est rendu par un lien physique, qui échoue plutôt que d'écraser un bail pris dans l'intervalle.
4. **Emplacements**, `w1` à `w3` puis `main` :

   | Emplacement | `incremental` supprimé au-delà de | Emplacement vidé au-delà de |
   |---|---|---|
   | `w1`, `w2`, `w3` (libres seulement) | 6 Go | 10 Go |
   | `main` | 12 Go | 20 Go |

   Les seuils sont ceux du §9.2, à calibrer. Les dossiers `incremental` visés sont `<profil>\incremental` et `<cible>\<profil>\incremental`.

   Un emplacement `w1` à `w3` **tenu**, ou vers lequel mène la jonction d'un worktree de la liste, n'est **jamais touché**. `main` porte toujours la jonction de la copie principale : il n'est nettoyé que si `disk` est lancé **depuis la copie principale**, par la session qui y travaille (décidé : depuis un worktree, rien ne dit qu'une simulation n'y tourne pas). Avant d'agir sur un emplacement, `disk` procède ainsi :
   - il prend son bail `<nom>.lease` par **création exclusive**, comme `New-Lease` ; si le bail existe déjà, il n'y touche pas. Ce bail est rafraîchi chaque minute pendant le nettoyage pour rester tenu ; après un arrêt brutal, il redevient orphelin au bout de 10 minutes, puisqu'il ne nomme aucun worktree ;
   - il prend en exclusif les verrous de Cargo existants (`.cargo-*lock`, à trois niveaux au plus) : une compilation en cours fait renoncer à l'emplacement, et aucune ne démarre dans ces profils pendant le nettoyage. Ces fichiers de verrou restent en place. Un profil sans verrou n'est pas protégé de cette façon : pour `w1` à `w3`, le bail suffit, et pour `main`, c'est la session principale qui lance `disk`.

   Il rend le bail à la fin. Une suppression incomplète est signalée avec son nombre d'erreurs, et le code de sortie vaut alors 1.
5. **`artifacts\runs`** : une exécution est supprimée si rien n'y a été modifié depuis plus de **30 jours**, sauf si elle contient un fichier `keep`. Les règles de `artifacts\worlds` et `packs` (§9.2 point 6) viendront avec ces artefacts.
6. **Dossiers laissés par `git worktree remove`** sous `.claude\worktrees` de la copie principale. Un dossier n'est retiré que s'il remplit trois conditions : il est **absent de `git worktree list`**, il n'a **pas de `.git`**, et rien n'y a été modifié depuis plus de **24 h**. Sa jonction `target` est retirée seule.

**Suppression sûre (F8).** Les garanties suivantes sont tenues par `disk.rs` :
- **aucun lien n'est suivi** : ni lien symbolique, ni jonction, ni aucun autre point de réanalyse (attribut `FILE_ATTRIBUTE_REPARSE_POINT`). Le lien lui-même est retiré, par `remove_dir` pour un lien de dossier, ce qui retire la jonction et non le contenu de sa cible, ou par `remove_file` sinon. La suppression est un parcours écrit pour cela, jamais `remove_dir_all`. Les tailles et les dates sont aussi mesurées sans suivre les liens ;
- **périmètre** : chaque chemin à supprimer est mis sous forme canonique (dossier parent résolu, nom final gardé, pour qu'un lien final ne soit pas suivi). Il doit être strictement sous `TS_CACHE\target`, `TS_CACHE\artifacts\runs` ou `.claude\worktrees`, elles-mêmes canoniques et à au moins deux niveaux sous la racine du disque. Un chemin qui passe par une jonction vers l'extérieur est donc refusé ;
- **limite connue** : entre le contrôle d'une entrée et sa lecture, un tiers qui écrit dans `TS_CACHE` pourrait la remplacer par une jonction. Cela suppose déjà la machine compromise.

**Essais.** `cargo test -p xtask` crée de **vraies jonctions** (`mklink /J`) dans des dossiers jetables sous `TS_CACHE\tmp\` (ailleurs que sous Windows : des liens symboliques dans le dossier temporaire du système). Il vérifie que la cible reste intacte après la suppression, que le contrôle du préfixe refuse un chemin qui traverse une jonction, et que le bail et les verrous de Cargo sont exclusifs. Il fait aussi un parcours complet, simulation puis `--clean`, dans un `TS_CACHE` et un dépôt Git jetables. **Ne jamais lancer `disk --clean` pour essayer** sur le vrai `TS_CACHE` : prendre un `TS_CACHE` jetable, désigné par la variable `TS_CACHE`.

**Exemptions de clippy** : `slots.rs` et `disk.rs` parcourent `TS_CACHE`, prennent des baux et lisent l'horloge. Ils portent leurs propres exemptions, une par fonction et commentées (E6), en plus de celles de `sys.rs`.
