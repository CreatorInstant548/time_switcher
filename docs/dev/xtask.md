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
| `clippy` | `cargo clippy --workspace --all-targets --locked -- -D warnings`, puis la même passe sur le **graphe livré** seul avec `--features ts_kernel/public` : `ts_kernel`, les hôtes présents et leurs dépendances normales de l'espace de travail, sans `--workspace` ni `--all-targets`. Enfin, une passe `--all-targets` sur les crates de cœur par fonctionnalité de développement, l'une après l'autre (`--features ts_kernel/dev-commands`, puis les trois autres) : le code sous `#[cfg(feature = "dev-…")]` est vu aussi. Jamais `--all-features`. Enfin, une passe sur `ts_kernel` avec une **copie de `clippy.toml` sans `allow-invalid = true`** pour les interdictions d'une crate `bevy_*` qui est une dépendance normale de `ts_kernel` (chemins internes que déplace une mise à jour ; `rand` et `getrandom` interdisent l'API de plusieurs versions à la fois et gardent l'indicateur) (`CLIPPY_CONF_DIR`, pour cette passe seulement, copie dans `TS_CACHE/tmp`) : une interdiction rendue morte par une mise à jour de dépendance (chemin disparu ou renommé) est un échec, au lieu de rester silencieuse. La réécriture est fermée : une ligne qui porte `allow-invalid` hors de la forme unique `{ path = "P", reason = "R", allow-invalid = true },` (sans blancs, sur plusieurs lignes, entre apostrophes, chemin échappé, drapeau seul sur sa ligne) est un écart, jamais sautée. Les autres entrées gardent l'indicateur | F1, I2 |
| `test` | `cargo test --locked` sur les crates touchées **et leurs dépendants**, directs ou non. Une modification de `Cargo.toml` racine, `Cargo.lock`, `clippy.toml`, `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml` ou `.cargo/` vaut `--all`. Ignorée si aucune crate n'est touchée | F7 |
| `determinism` | version courte de `cargo xtask determinism` (§7), lancée avec `--all`, si `ts_toy` ou l'une de ses dépendances est touchée, si `data/golden/` ou `tools/xtask/src/determinism.rs` change, ou sur un fichier racine qui vaut `--all`. Ignorée sinon. Les tests de `ts_toy` tournent aussi dans `test` (accord interne, sans valeurs dorées). En CI, `determinism` est dans les listes `--only` de `rust-linux`, de `rust-windows` et du travail hebdomadaire `windows`, et l'an complet tourne chaque dimanche par `cargo xtask determinism --full` ([ci](ci.md) §2 et §4) | V0-09, L6, Y5 |
| `deny` | version de `cargo-deny` comparée au `tools/dev/tools.lock` du dépôt (lisible en CI), puis `cargo deny --locked check`, à chaque exécution | SEC0-03, F9 |
| `audit` | version de `cargo-audit` comparée à `tools.lock`, puis `cargo audit`. Lancé avec `--all`, si `Cargo.lock` change, ou si la base RustSec locale a plus de 7 jours ; ignoré sinon. Si la mise à jour de la base échoue, un second essai `--no-fetch` n'est permis qu'avec une base de moins de 7 jours | SEC0-03, F9 |
| `gitleaks` | en local, binaire et empreinte SHA-256 lus dans le `tools.lock` d'`origin/main`, et configuration `tools/dev/hooks/gitleaks.toml` d'`origin/main`, par `git show`, jamais ceux de la copie de travail ; la configuration est écrite dans un dossier de l'exécution sous `TS_CACHE\tmp`, supprimé à la fin (AS-50 : plus de copie épinglée des crochets) ; en CI, `gitleaks.linux.*` et la configuration posée par `install-tools.sh`, comparée à celle d'`origin/main` (G5) ; `.gitleaksignore` à la racine refusé ; `--redact`. Arbre : `git ls-files -co --exclude-standard`, extrait dans `TS_CACHE\tmp`, sans `target/` ni lien. Commits : `origin/main..HEAD`, avec `--text --no-ext-diff --no-textconv`. Un seul dossier par exécution, créé de façon exclusive sous `TS_CACHE/mp` avec un marqueur : il n'est supprimé à la fin que si le marqueur est bien celui de l'exécution (m8). Un secret probable est listé par règle, fichier et ligne (et commit abrégé), d'après le rapport JSON expurgé, jamais avec sa valeur ni l'auteur du commit (m5) | SEC0-06, SEC0-08, F2 |
| `unsafe` | les racines de chaque crate (cibles `lib`, `cdylib`, `bin`, `proc-macro` et `custom-build` de `cargo metadata`, `xtask` compris) contiennent `#![forbid(unsafe_code)]`, **sans exception anticipée** (`ts_client_native` compris, PY1) ; seule `ts_py` est en `deny`, avec un `// SAFETY:` avant chaque `allow`. Chaque `Cargo.toml` membre contient `[lints] workspace = true`, sans `[lints.*]` local, sauf `ts_py` (AS-18) : ni `[lints]` ni clé `lints…`, mais des tables `[lints.<outil>]` **identiques** aux `[workspace.lints.<outil>]` du `Cargo.toml` racine, comparées clé par clé sans blancs ni commentaires, sauf `unsafe_code = "deny"` à la place de `"forbid"`. Une table de plus ou de moins, une clé en double ou une ligne illisible sont refusées | SEC0-19, F3, AS-18, PY1 |
| `profiles` | `cargo check -p ts_kernel --locked --features public,<f>` échoue **sur notre message** `compile_error!`, pour chacune des 4 fonctionnalités de développement | SEC0-20, E8 |
| `features` | aucune mention de `public` hors de sa déclaration dans `ts_kernel` (dont `ts_kernel/public` et `ts_kernel?/public`) ; `default` de `ts_kernel` vide ; aucun hôte n'active une fonctionnalité de développement, vérifié aussi par `cargo tree -p <hôte> --edges normal,features` ; aucune crate hors outils ne dépend de `ts_toy`, directement ou non, en dépendance normale | SEC0-20, F4, D-DEV-02 |
| `clippy-exemptions` | dans toute crate autre qu'un hôte ou un outil, chaque `.rs` est d'abord **normalisé** comme rustc le lit : commentaires `//` et `/* */` (imbriqués, doc comprise) retirés, préfixes `r#` des identifiants retirés, chaînes et caractères gardés mais ignorés. Sont ensuite refusés, hors chaîne :<br>- **tout identifiant** `disallowed_*` (`disallowed_methods`, `disallowed_types`, `disallowed_macros`, `disallowed_names` et les anciens noms, `blacklisted_name` compris), quel que soit le contexte : attribut, argument de macro, accolades ;<br>- **l'identifiant** `include`, `include_str` ou `include_bytes`, partout, `use` compris ;<br>- `path = …` dans un attribut (`#[path]`, `cfg_attr(…, path = …)`) ;<br>- `allow(…)` ou `expect(…)` de `clippy::all`, `clippy::style` ou `warnings`, quel que soit le caractère qui précède (sauf `.`), et tout `$` dans les arguments d'un `allow`, `expect`, `warn` ou `deny` ;<br>- dans `cfg(…)`, `cfg!(…)` et le prédicat de `cfg_attr(…)`, tout prédicat hors de la **liste fermée** `test`, `feature = "…"`, `all`, `any`, `not` (donc `clippy`, `debug_assertions`, `target_*`, `doc`…) ;<br>- tout fichier autre que `.rs` sous `src/` (liste fermée `NON_RS_SRC_EXEMPT`, vide) ;<br>- les **éléments réservés du noyau** que clippy ne peut pas interdire au cœur sans exemption (#26 m3, m4, module `reserved`), hors code de test (cibles `tests`, `benches` et `examples` à la racine de la crate, fichiers `tests.rs` et `*_tests.rs`, bloc `#[cfg(test)] mod … { … }`) : `WorldKey` hors de `ts_kernel` et `ts_toy`, `from_snapshot_raw` hors de `ts_kernel` et `ts_persist`, `chacha20` hors de `ts_kernel/src/rng/`, `blake3` hors de `ts_kernel/src/rng/`, `state.rs` et `plugin.rs` du noyau, `ts_data`, `ts_persist` et `ts_toy` ; une dépendance normale ou de construction à `chacha20` hors de `ts_kernel`, ou à `blake3` hors de ces quatre crates. C'est un garde-fou contre l'erreur, non contre un contournement voulu (renommage, réexportation dans une crate permise, `StateDecode` de `StableId`), que la revue attrape.<br>Aucun `clippy.toml` ni `.clippy.toml` hors de la racine ; `CLIPPY_CONF_DIR` refusé | E6, F5, I2 ; #26 |
| `local-paths` | dans les fichiers suivis et non ignorés : aucun dossier personnel, c'est-à-dire `Users` ou `home` entre deux séparateurs `/` ou `\`, quelle que soit la casse, ni la valeur de `USERNAME` ou de `COMPUTERNAME` en mot entier. Le résultat donne le fichier et la ligne, jamais la valeur | SEC0-26, F6 |
| `workflows` | chaque fichier de `.github/workflows/` : `permissions: {}` au niveau du workflow ; actions de la liste fermée, épinglées par SHA de 40 caractères ; `actions/checkout` avec `persist-credentials: false` sous son `with:` ; déclencheurs limités à `push`, `pull_request`, `schedule` et `workflow_dispatch` (donc ni `pull_request_target`, ni `workflow_run`), ni `secrets: inherit` ; chaque travail a son propre `runs-on:` et il vaut `ubuntu-24.04` ou `windows-2025` (aucun exécuteur auto-hébergé, voir [ci.md](ci.md) §2). Clés comparées sans tenir compte de la casse (`USES:`, `Run:`).<br>**Interpolations** : seule `${{ runner.… }}` est sûre. Toute autre (`github`, `inputs`, `matrix`, `env`, `steps`, `needs`, fonctions comme `toJSON(github)`), quelle que soit la casse, n'est permise que dans les valeurs sous `env:` ou `with:`, et dans `if:` ; jamais dans un `run:`, qui la reçoit par `env:` et une variable citée.<br>**Toute ligne non reconnue est refusée** : seules sont acceptées `clé: valeur`, `- clé: valeur`, `- scalaire`, les commentaires et le contenu des blocs `|` ou `>`. Sont ainsi refusés : clés explicites `?` et `:`, ancres, alias et balises (`&`, `*`, `!`), collections en ligne autres que `{}`, `[]` ou une liste de mots, `uses:` ou `run:` hors d'une clé de tête de ligne, indentation incohérente (enfant d'une valeur, retrait hors d'un niveau ouvert, frères désalignés), tabulation, `---`, et barre oblique inverse hors d'un bloc, puisque YAML y décode les échappements. Ignorée s'il n'y a aucun workflow | SEC0-29 à 31, F10, I1 |
| `spec-map` | `cargo xtask spec-map check` : chaque renvoi `// spec:` du code désigne un fichier, une section et un identifiant qui existent ; chaque marque provisoire (`PROVISIONAL(design #n)`, ancienne forme `PROVISOIRE(conception #n)`) porte un numéro de remontée ; aucun type de fichier suivi n'est sauté sans être classé ; `known_bad.txt` ne gagne aucune entrée par rapport à `origin/main`. Échoue sans la conception ou sans `origin/pilotage`. Mode d'emploi : [spec-map](spec-map.md) | T4 |

Les étapes `deny`, `gitleaks`, `unsafe` et `features` à `workflows` portent **toujours sur tout le dépôt**, quel que soit le périmètre (F7).

**Chemins affichés** (m6, SEC0-26) : les messages de `check` donnent les chemins relatifs à la racine du dépôt, `$TS_CACHE/…` sous le cache, et le seul nom du fichier pour tout autre chemin absolu ; les arguments absolus des commandes lancées sont affichés de la même façon.

**Arbre lu** (I3) : `git ls-files -z -co --exclude-standard`, découpé sur NUL, donc sans citation des noms. Chaque fichier doit être un fichier ordinaire présent sur le disque. Un lien symbolique (hors de la liste fermée `SYMLINK_EXEMPT`, vide), un fichier suivi mais supprimé, ou un autre objet est un écart, qui fait échouer toute étape qui lit l'arbre : `gitleaks`, `unsafe` pour `ts_py`, `clippy-exemptions`, `local-paths` et `workflows`. Les crates touchées se lisent aussi en `-z`.

## 2. Listes tenues dans le code

Elles sont à compléter **dans la PR qui crée la crate concernée** :
- `manifests::HOSTS` : crates d'hôte (`ts_host`, `ts_server`, `ts_client_native`) ;
- `manifests::TOOLS` : crates d'outil (`ts_py`, `ts_runner`, `ts_viz`, `xtask`). Les hôtes et les outils sont les seules crates qui peuvent s'exempter des interdictions de clippy. Les outils sont aussi les seules qui peuvent dépendre de `ts_toy`. **Toute autre crate est traitée comme une crate de cœur ou de règles** (refus par défaut) ;
- `scan::LOCAL_PATH_EXEMPT` : fichiers exemptés des motifs de chemins fixes, c'est-à-dire seulement le fichier qui définit et teste ces motifs. Les noms d'utilisateur et de machine y restent recherchés.

## 3. Dépendances et exemptions

- **Dépendances** : `serde` (avec la dérivation) et `serde_json`, pour lire `cargo metadata --format-version 1 --no-deps` dans des structures réduites aux champs lus (AS-17 : `serde_json` pour les crates d'outil seulement). Il n'y a ni `clap`, puisque l'analyse des options tient en quelques lignes, ni crate de cryptographie : le SHA-256 de gitleaks est calculé par `certutil` sous Windows et par `sha256sum` ailleurs.
- **Interdictions de clippy** : `xtask` lit les fichiers, l'environnement, la ligne de commande et l'horloge. Ces accès sont regroupés dans `sys.rs`, avec une exemption par fonction, commentée (E6).
- **Objets de remplacement** (`refs/replace/`, #63) : tout processus enfant de `xtask` reçoit `GIT_NO_REPLACE_OBJECTS=1` (équivalent de `git --no-replace-objects`), posé au point unique qui crée les commandes (`sys::base_command`, utilisé par `sys::command` et par `uv`). Aucun appel `git` de `xtask`, direct ou fait par un outil lancé (gitleaks, cargo), ne peut donc l'oublier, et un lanceur de plus doit passer par ce point. Sont hors de cette portée les crochets Git et les scripts `tools/dev/*.ps1`, qui ne sont pas des enfants de `xtask`. Les enfants de `cargo xtask check` (dont les binaires de test) héritent aussi de la variable. Sans cela, un ref déposé dans le dossier Git de la copie principale montrerait à `merge` et à `verify-main` un autre commit, arbre ou blob que celui désigné par le SHA.
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

Sans `--clean`, **rien n'est modifié** : `git worktree prune` tourne avec `-n`, et chaque suppression est annoncée par « supprimerait », ou par « refusé » si le contrôle du périmètre la refuserait (même contrôle qu'avec `--clean`). Dans l'ordre :

0. **Validation de `TS_CACHE`**, avant toute action. `TS_CACHE` est refusé dans trois cas :
   - il est lui-même un lien ou une jonction (le déplacer sur un autre disque se fait donc en changeant la variable `TS_CACHE`, pas par une jonction) ;
   - il est à moins de deux niveaux de la racine du disque ;
   - il n'est ni le chemin de la convention `C:\Dev\ts-cache`, ni un sous-dossier de `C:\Dev\ts-cache\tmp\`, et il ne contient pas de fichier repère `.ts-cache`, posé exprès à la main.

   Avec `--clean`, le refus fait échouer la commande (code 1) avant toute action. Sans `--clean`, seul le bilan s'affiche, avec un avertissement.
1. **Bilan** : le tableau de `slots`, la taille de `TS_CACHE\artifacts`, de `.claude\worktrees` et des archives de Cargo (`registry\cache` seulement, car le parcours de `registry\src` prend une quinzaine de secondes), la liste des worktrees et l'espace libre.
2. **`git worktree prune -v`**, puis nouvelle lecture de `git worktree list`. Si la liste est illisible, rien n'est supprimé. **Dépôt attendu** : `prune` et l'étape 6 ne portent que sur le dépôt du dossier courant, et seulement dans deux cas. Soit `TS_CACHE` est celui de la convention. Soit la copie principale de ce dépôt est un dépôt jetable, sous `TS_CACHE\tmp\` ou sous `C:\Dev\ts-cache\tmp\` (décidé : les bancs d'essai y placent côte à côte leur `TS_CACHE` et leur dépôt). Sinon, ces deux étapes sont sautées, et la sortie le dit.
3. **Baux et emplacements** : seulement si `TS_CACHE\target` est une racine retenue (voir « Suppression sûre ») ; sinon, aucun bail ni aucun emplacement n'est touché, et la commande échoue. Chaque chemin de bail, sa copie `.stale-<pid>` et chaque emplacement passent par le même contrôle de périmètre que les suppressions, avant tout renommage, toute création, toute suppression et toute prise de verrou.

   **Baux orphelins** (`wN.lease` et `main.lease`) : libérés par renommage atomique puis suppression, comme `Remove-OrphanLease`. Un bail redevenu tenu entre-temps est rendu par un lien physique, qui échoue plutôt que d'écraser un bail pris dans l'intervalle.
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
6. **Dossiers laissés par `git worktree remove`** sous `.claude\worktrees` de la copie principale du dépôt attendu. Un dossier n'est retiré que s'il remplit trois conditions : il est **absent de `git worktree list`**, il n'a **pas de `.git`**, et rien n'y a été modifié depuis plus de **24 h**. Sa jonction `target` est retirée seule.

Pour les étapes 5 et 6, une erreur de mesure (sous-dossier illisible) ou un arbre sans date valent « actif » : rien n'est supprimé.

**Suppression sûre (F8).** Les garanties suivantes sont tenues par `disk.rs` :
- **aucun lien n'est suivi** : ni lien symbolique, ni jonction, ni aucun autre point de réanalyse (attribut `FILE_ATTRIBUTE_REPARSE_POINT`). Le lien lui-même est retiré, par `remove_dir` pour un lien de dossier, ce qui retire la jonction et non le contenu de sa cible, ou par `remove_file` sinon. La suppression est un parcours écrit pour cela, jamais `remove_dir_all`. Les tailles et les dates sont aussi mesurées sans suivre les liens ;
- **périmètre** : chaque chemin à supprimer est mis sous forme canonique (dossier parent résolu, nom final gardé, pour qu'un lien final ne soit pas suivi). Il doit être strictement sous `TS_CACHE\target`, `TS_CACHE\artifacts\runs` ou `.claude\worktrees`, elles-mêmes canoniques et à au moins deux niveaux sous la racine du disque. Une racine dont un composant sous `TS_CACHE` ou sous la copie principale est un lien (`target`, `artifacts`, `runs`, `.claude`, `worktrees`) est écartée : rien n'y est supprimé. Un chemin qui passe par une jonction vers l'extérieur est donc refusé ;
- **limite connue** : entre le contrôle d'une entrée et sa lecture, un tiers qui écrit dans `TS_CACHE` pourrait la remplacer par une jonction. Cela suppose déjà la machine compromise.

**Essais.** `cargo test -p xtask` crée de **vraies jonctions** (`mklink /J`) dans des dossiers jetables sous `TS_CACHE\tmp\` (ailleurs que sous Windows : des liens symboliques dans le dossier temporaire du système). Il vérifie que la cible reste intacte après la suppression, que le contrôle du préfixe refuse un chemin qui traverse une jonction, et que le bail et les verrous de Cargo sont exclusifs. Il fait aussi un parcours complet, simulation puis `--clean`, dans un `TS_CACHE` et un dépôt Git jetables. D'autres essais posent une jonction à la place de `TS_CACHE\target`, avec un bail orphelin derrière : rien ne bouge hors de `TS_CACHE`. Ils vérifient aussi le refus d'un `TS_CACHE` non conforme et la règle du dépôt attendu.

**Essayer le binaire à la main** : **jamais sur le vrai `TS_CACHE` ni depuis le vrai dépôt**. `prune` et l'étape 6 portent sur le dépôt du dossier courant, pas sur `TS_CACHE`. Il faut donc :
1. créer sous `C:\Dev\ts-cache\tmp\<banc>\` un `TS_CACHE` jetable (`<banc>\cache`) et un dépôt Git jetable (`<banc>\repo`, par `git init` puis `git worktree add`) ;
2. copier le binaire `xtask.exe` hors du worktree ;
3. le lancer **depuis le dépôt jetable**, avec `TS_CACHE` réglé sur `<banc>\cache`.

**Exemptions de clippy** : `slots.rs` et `disk.rs` parcourent `TS_CACHE`, prennent des baux et lisent l'horloge. Ils portent leurs propres exemptions, une par fonction et commentées (E6), en plus de celles de `sys.rs`.

## 6. Approbations de `cargo xtask merge` et `cargo xtask verify-main`

Une PR qui touche un chemin sensible (`tools/ci/sensitive-paths.txt` d'`origin/main`) doit porter deux avis : `review-ok: <sha>` (relecteur neuf) et `archsec-ok: <sha>` (archi-sécu). Chacun compte seulement s'il occupe **la première ligne** d'un commentaire de la PR, exactement sous cette forme, avec un SHA complet (I3). Une ligne citée plus bas ne compte pas. Le commentaire doit venir du **propriétaire du dépôt** (AS-45 P3) : `user.login` égal au propriétaire du *slug*, `author_association: OWNER` et `user.type: User` ; tout autre auteur est ignoré, puisque sur un dépôt public n'importe qui peut écrire `review-ok: <sha>`.

`merge` (étape 10) et `verify-main` (chaque commit sensible de `main`) appliquent **la même fonction**, `carry::Carry::resolve` (W6). Un avis compte s'il est posé :
- sur le SHA de tête ;
- ou sur un SHA antérieur `A` de la même PR, s'il peut être **reporté** (§6.1).

### 6.1 Report d'une approbation après une mise à jour depuis `main` (AS-40)

Un `review-ok` ou un `archsec-ok` posé sur `A` vaut pour la tête `H` seulement si **toutes** ces conditions sont remplies :
- **W1, même PR** : `A` figure parmi les commits de la PR lus sur GitHub (`pulls/<n>/commits`), et c'est un ancêtre de `H`.
- **W2, mêmes fichiers, mêmes contenus** : la PR modifie le même ensemble de fichiers en `A` et en `H` (de la base de fusion au commit, sans renommage). Chacun a le même identifiant de blob et le même mode en `A` et en `H`. On compare les blobs, jamais les lignes de diff.
- **W3, `main` n'a pas touché ces fichiers** : la base de `A` est un ancêtre de celle de `H`, et aucun fichier de la PR n'y diffère entre les deux bases.
- **W4, rien d'autre ne bouge** : hors des fichiers de la PR, l'arbre de `H` est identique à celui de `main` à sa base. Aucune résolution manuelle ne peut se cacher ailleurs. De plus, la base de `H` doit être le `main` de référence (`origin/main` pour `merge`, le parent du squash pour `verify-main`), et le commit qui arrive sur `main` doit avoir l'arbre de `H` (le squash, pour `verify-main`).
- **W5, même sensibilité** : c'est d'abord la liste `sensitive-paths.txt` d'`origin/main` qui décide si des avis sont exigés. Si oui, un `archsec-ok` n'est reporté que si la liste de la base de `A` rendait déjà la PR sensible. Sinon, un `archsec-ok` **neuf** sur `H` est exigé. Une liste absente ou illisible à la base de `A` compte comme « non sensible ».
- **W6, J4** : `verify-main` fait le recoupement J4 sur `A` quand l'`archsec-ok` est reporté. Le §9 du carnet d'archi-sécu doit donc contenir `archsec-ok: <A>`. Parmi plusieurs SHA reportables, celui que le §9 contient passe en premier.
- **W7, ce qui ne change pas** : `ci-ok` reste exigé sur `H` lui-même.

Si une seule condition échoue, l'approbation doit être **reposée sur `H`**. C'est le cas notamment :
- d'une **résolution de conflit** dans un fichier de la PR : le blob diffère, W2 échoue ;
- d'un fichier ajouté à la PR, ou d'un fichier hors PR modifié : W2 et W4 échouent ;
- d'un fichier retiré de la PR : W2 échoue ;
- d'un fichier de la PR modifié sur `main`, même si la fusion a gardé la version de la PR : W3 échoue ;
- d'un rebasage : W1 échoue.

**Affichage.**
- `merge` affiche, pour chaque avis : `<avis>: <tête>`, ou `<avis> carried over from <A> to the head <tête> (AS-40, W1 to W5 hold)`.
- Un refus dit, pour chaque SHA candidat, la condition qui a échoué, par exemple `W2: <fichier> has another blob or mode at the head than at <A> (conflict resolution?)`.
- `verify-main` note `<avis> carried over from <A>` sur la ligne du commit.

**Lectures.**
- Les arbres de `A` et de `H` :
  - pour `merge`, sont lus en local (`git ls-tree`), depuis les objets apportés par `git fetch origin pull/<n>/head` ;
  - pour `verify-main`, viennent de l'API (`git/trees/<sha>?recursive=1`), sans cache. En CI, `verify-main` n'a pas les objets des PR, puisqu'il ne fait pas de `git fetch`.
- Les bases de fusion viennent de `compare/<x>...<y>`.
- Les bases sont des commits de `main`, lus dans l'historique local.
- Au plus 5 SHA candidats sont examinés, du plus récent au plus ancien, après avoir écarté ceux qui ne sont pas dans la PR.
- Tout est relu sur GitHub à chaque exécution (V5). `verify-main` n'a plus de cache local depuis T3e-2.
- Une lecture qui échoue fait échouer le candidat, jamais l'inverse.

**Portée de `verify-main` depuis T3e-2** (AS-31 E3). Chaque commit de `main` postérieur à la base doit avoir un seul parent, pour que sa sensibilité se calcule. Un commit **non sensible** ne demande rien d'autre et ne lit rien sur GitHub : le *ruleset* `main` impose la PR, `ci-ok` et l'historique linéaire, et `repo-settings.ps1 -Check` surveille ce *ruleset* : l'orchestrateur le lance chaque jour depuis la copie principale (AS-50 AU3 ; il est sauté en CI, et `cargo xtask merge` ne le lance plus). Un commit **sensible** garde tout : squash d'une PR fusionnée (J3), `ci-ok` sur sa tête (G3), `review-ok` et `archsec-ok` sur la tête ou reportés (§6.1), puis J4.

**`--origin <n>d|all`** (AS-42, T3j ; tâche de nuit, `docs/dev/ci.md` §4). Chaque commit non sensible de la fenêtre doit en plus être le squash d'une PR fusionnée dans `main`, dont la tête est un autre commit (J3, AS-44 AA1), lu par **une seule** requête, `commits/<sha>/pulls`. Sur `<n>d`, il faut aussi `ci-ok` vert sur cette tête (G3, même fonction que pour un commit sensible, AS-44 AA2) ; `all` reste en J3 seul (quota, #55). La fenêtre `<n>d` (1 à 366 jours) va du sommet de `main` jusqu'au premier commit daté d'avant la limite qui passe J3 et G3, inclus (AA3) : des commits antidatés sans PR ou sans `ci-ok` échouent, et la lecture continue à travers eux ; `all` couvre tout l'historique postérieur à la base. Sans `--origin` (`cargo xtask merge` compris), rien ne change. `--no-cache`, sans effet depuis T3e-2, est retiré.

## 7. `cargo xtask determinism [--full] [--bless]`

```
cargo xtask determinism            # version courte : 14 jours simulés (étape de check)
cargo xtask determinism --full     # plus l'an complet : 365 jours (chaque semaine en CI, Y5, par une PR `.github/` à part)
cargo xtask determinism --bless    # réécrit les valeurs dorées produites ; à signaler dans la PR
```

Preuve du critère de fin de la phase 0 (V0-09) sur le monde-jouet de `crates/ts_toy` : 1 000 résidents qui vont chaque jour de leur maison à leur travail, par événements datés, donc en saut de temps ; trois d'entre eux passent 30 s en simulation complète chaque semaine, pour exercer le mode tick par tick. Toutes les durées et tailles sont des **paramètres de test**, jamais des valeurs de jeu (D-DEV-02).

1. Les deux exécutables de test de `ts_toy`, `one_thread` (1 fil) et `four_threads` (4 fils), tournent chacun dans **son propre processus** (L6), avec `--nocapture --test-threads=1`, et `--include-ignored` pour `--full`.
2. Chaque processus fait, pour chaque version : une exécution avec un instantané en route (un jour de promenade, à 12:00:10, donc avec des entités en `LodComplete`), encodé puis décodé par `ts_persist`, restauré, puis mené à la fin ; une seconde exécution d'une traite. Les trois empreintes finales doivent être égales. Il lance aussi le générateur-jouet sur le groupe de fils et en ligne, qui doivent donner la même ville.
3. Chaque processus écrit ses résultats sur des lignes `ts-toy-result <clé> <valeur>`. La commande vérifie que chaque processus a bien tourné sur le nombre de fils attendu (`process.threads` vaut 1 pour `one_thread`, 4 pour `four_threads`, ces valeurs étant fixées côté xtask : un test réduit à 1 contre 1 fil est refusé), que les deux processus donnent les mêmes valeurs (sauf `process.threads`), puis les compare à `data/golden/ts_toy.json` : chaque version contre ses propres valeurs (`short.*`, `year.*`, `generator`). Un résultat sans valeur dorée, ou une valeur dorée sans résultat, est un écart.
4. Elle affiche la durée de chaque processus.

`--bless` réécrit seulement l'objet `results` du fichier doré, et seulement si les processus sont d'accord. Le manifeste du même fichier ne contient aucun chemin local (SEC0-26).

## 8. `cargo xtask py`

```
cargo xtask py     # environnement Python, construction de ts_py, puis pytest
```

Environnement Python de la crate `ts_py` (T10, conditions PY1 à PY7 du §9.16 du carnet d'archi-sécu). Elle compile `ts_py` : prendre un emplacement avant (`session-start.ps1 -Acquire`). Dans l'ordre, la première étape qui échoue arrête tout :
1. **`uv` épinglé** : `uv` est cherché dans le `PATH` ; son empreinte SHA-256 (`uv.sha256`, `uv.linux.sha256` hors de Windows) et sa version (`uv.version`) doivent être celles de `tools/dev/tools.lock`. Les messages donnent le nom de l'outil, jamais son chemin (SEC0-26).
2. **`uv sync --locked`** dans `python/` : `python/.venv`, propre au worktree, d'après `python/uv.lock`, commité avec ses empreintes. `pyproject.toml` impose des roues seulement (`no-build`), l'interpréteur installé de `.python-version` (3.14.0, sans téléchargement), des paquets publiés au moins deux semaines avant le verrou (`exclude-newer`), et la version de `uv`. Le groupe `base` contient `numpy`, `pytest` et `maturin`, et rien d'autre.
3. **`maturin develop --uv --locked`**, par `uv run --no-sync` : `ts_py` en profil `dev`, installée dans `.venv`. `uv` ne construit jamais le projet lui-même (`package = false`).
4. **`pytest -p no:cacheprovider`**, par `uv run --no-sync` : `python/tests`.

Les processus enfants ne reçoivent aucune variable `UV_*`, `PIP_*`, `PYTHON*`, `PYO3_*`, `MATURIN_*`, `VIRTUAL_ENV*` ni `CONDA*` : elles pourraient changer l'index, les empreintes, l'interpréteur ou la configuration de `pyo3`. Seules `PYTHONDONTWRITEBYTECODE=1` et `UV_NO_CONFIG=1` sont posées : `uv`, y compris le `uv pip` lancé par maturin, ne lit alors **aucun fichier de configuration**, ni `uv.toml` de l'utilisateur ou du système, ni `[tool.uv]`. Les réglages de `pyproject.toml` qui comptent sont donc repris sur la ligne de commande de `uv sync` : `--no-build`, `--no-python-downloads`, `--python-preference only-system` et `--exclude-newer`, lu dans `[options]` de `uv.lock`. `package` et `default-groups`, qui décrivent le projet, restent lus. `uv` est haché puis lancé par son chemin : un remplacement entre les deux reste possible (risque connu, comme pour gitleaks). Hors de Windows, `uv.linux.sha256` manque encore à `tools.lock` : la commande refuse de tourner.

Le groupe optionnel `viz` de `python/pyproject.toml` (conversion des positions en `.rrd`, [visualisation](visualisation.md)) n'est installé par aucune étape : `uv sync --locked` le retire de `.venv`, et `default-groups` ne contient que `base`.

`ts_py` reste hors de `default-members` (A13) : `cargo clippy --workspace` la vérifie, mais `cargo test` ne construit rien pour elle (`test = false`, `doctest = false`), puisque ses tests sont ceux de `pytest`. Son script de construction (`pyo3-build-config`) lance l'interpréteur Python trouvé (`PYO3_PYTHON`, environnement virtuel, puis `python` ou `python3` du `PATH`) pour lire sa configuration.
