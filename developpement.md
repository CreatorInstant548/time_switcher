# Carnet du responsable développement

> Tenu par le responsable développement seul. Règles : [organisation.md](../time_switcher/docs/pilotage/organisation.md). Mis à jour le 30/10/2026.

## État

- Phase 0 (fondations) en cours depuis le 30/10, en **mode local** (`CLAUDE.md`) : pas de dépôt distant, pas de CI GitHub, aucun push.
- Découpage ci-dessous envoyé au responsable architecture et sécurité le 30/10 ; **validation attendue** pour les tâches marquées « V ».
- Rien n'est encore fusionné sur `main` (2 commits de documentation).

## Critère de fin de la phase 0 (05 §5, point 0 ; revue d'architecture §22 ; sécurité §7 ; génération §12)

| N° | Condition | Preuve attendue | État |
|---|---|---|---|
| F1 | Monde-jouet de 1 000 entités qui suivent un agenda, un an simulé en saut de temps, même empreinte à chaque exécution, sur 1 fil et sur N fils (M1, M2) | `cargo xtask determinism` + run `sim` dans la copie principale | — |
| F2 | La vérification bloque une dépendance vulnérable **et** un secret commis (en mode local : `cargo xtask check` et crochet `pre-commit`, à la place de la CI) | Deux branches jetables rouges, puis supprimées | — |
| F3 | Un worktree neuf compile dans un emplacement partagé | Mesure de T12 (froid / chaud) | — |
| F4 | Aller-retour d'instantané vert ; Python lit l'état | Tests `ts_kernel`/`ts_persist` ; test `ts_py` | — |
| F5 | Sécurité : `forbid(unsafe_code)` vérifié ; `public` incompatible avec `dev-commands`, `dev-introspection`, `external-pilot` (M28) ; `PlayerId` opaque ; codec d'instantané borné avec empreinte par morceau ; fuzzing du codec une heure sans plantage ; aucune clé ni graine exposée (M27) | `xtask check`, test de non-compilation, fuzz 1 h | — |
| F6 | Génération : `draw` à clé de 128 bits avec les flux `CityGen`, `CityServer`, `PopGen`, `PopHidden`, `Furnish` ; espace de `StableId` réservé à la carte ; géométrie en centimètres entiers ; format de parts adressées (BLAKE3) ; générateur-jouet identique sur 1 et N fils | Tests | — |
| F7 | Assertions de taille des composants à la compilation (M7) | Macro dans `ts_kernel` | — |

## Découpage de la phase 0

Légende : **V** = attend la validation du responsable architecture et sécurité (frontière de crate, format ou chemin sensible) ; **S** = chemin sensible (`/security-review` + relecture par un sous-agent neuf + avis d'archi-sécu avant fusion). Chaque tâche = une branche `claude/<sujet>` = un worktree = une « PR » locale notée ci-dessous. Une PR ≈ 400 lignes au plus hors fichiers générés.

| Tâche | Branche | Contenu | Propriété (seuls chemins modifiables) | Dépend de | Modèle | Marques |
|---|---|---|---|---|---|---|
| T0 | aucune (machine) | Créer `C:\Dev\ts-cache\` (`target\{main,w1,w2,w3}`, `artifacts\`, `hooks\`, `tools\`) ; installer `cargo-audit` (`cargo install --locked`) ; `gitleaks` en binaire officiel épinglé par SHA-256 dans `C:\Dev\tools\gitleaks\` (repli décidé si le téléchargement est refusé : `ripsecrets` par `cargo install --locked`) ; relever les versions | Hors dépôt seulement | — | Sonnet | — |
| T1 | `claude/outillage-agents` | `tools/dev/session-start.ps1` (baux et jonctions, §6.2), `tools/dev/guard-main.ps1` (refus d'Edit/Write dans la copie principale), `tools/dev/guard-git.ps1` (refus de `git push` vers `main`, `git commit` sur `main` hors `xtask merge`, `--force`, `+refspec`, `--no-verify`, `-c core.hooksPath`, `gh api …git/refs`), `tools/dev/hooks/{pre-commit,pre-push}` (gitleaks ; refus de pousser `main` et de forcer), `.claude/settings.json` (SessionStart + PreToolUse guard-main), `docs/dev/outillage.md` ; vérifications bloquantes de la revue dev-agents §6.3 (suppression d'un worktree à jonction, `git clean -xdf`, chemins longs), repli `CARGO_TARGET_DIR` si échec | `tools/dev/`, `.claude/settings.json`, `docs/dev/` | T0 | Opus | V, S |
| T2 | `claude/espace-travail` | `Cargo.toml` racine (`members = ["crates/*", "tools/xtask"]`, `default-members`, `exclude = [".claude", "fuzz"]`, profils A12, `[workspace.lints]`), `rust-toolchain.toml` (1.99.1), `.cargo/config.toml` (alias `xtask`), `deny.toml`, `clippy.toml` (interdictions §9.2 : `HashMap` d'itération, `Instant`, `SystemTime`, `thread_rng`…), `rustfmt.toml` ; `crates/ts_kernel` vide avec `#![forbid(unsafe_code)]` et les fonctionnalités `dev-commands`, `dev-introspection`, `external-pilot`, `public` + `compile_error!` ; `tools/xtask` : `check` (fmt, clippy `-D warnings`, tests des crates touchées, `cargo deny check`, `cargo audit` si `Cargo.lock` change, scan de secrets, vérif. `forbid(unsafe_code)`, test « `public` + `dev-commands` ne compile pas »), `slots`, `disk [--clean]` | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `deny.toml`, `clippy.toml`, `rustfmt.toml`, `crates/ts_kernel/` (squelette), `tools/xtask/` | T1 | Opus | V, S |
| T3 | `claude/xtask-merge` | `cargo xtask hooks install|verify` (copie épinglée par empreinte dans `TS_CACHE\hooks`, `core.hooksPath` vers elle) ; `cargo xtask merge <branche>` mode local : refuse hors copie principale ou si `main` sale, vérifie crochets, branche à jour (`merge-base`), lance `xtask check` dans un worktree temporaire de la branche, exige un fichier d'approbation de relecture pour les chemins sensibles (`TS_CACHE\reviews\<branche>.ok`, écrit par la session relectrice), puis `git merge --squash` + commit au format, suppression de la branche ; mode GitHub prévu derrière une option (`gh pr checks` + `ci-ok` + `gh pr merge --squash --delete-branch`) ; `cargo xtask origin-check` (chaque commit de `main` vient d'une fusion notée) | `tools/xtask/src/{merge,hooks,origin}.rs` | T2 | Opus | V, S |
| T4 | `claude/xtask-spec-map` | `cargo xtask spec-map` : lit les `// spec: <fichier> §<n>` et `// PROVISOIRE(conception #<n>)` du code, les titres de sections de `docs/conception/`, produit la correspondance dans les deux sens (Markdown dans `TS_CACHE\artifacts\spec-map.md`, résumé à l'écran), signale un renvoi vers une section inexistante (erreur dans `xtask check`) | `tools/xtask/src/spec_map.rs` | T2 | Sonnet | V (format des marques) |
| T5 | `claude/ts-kernel-1` | `ts_kernel` : `SimTime(i64)` ms, `SimDay(u16)`, `StableId(u64)` avec espaces de noms (carte réservée, P28), `Money(i64)` centimes + arrondi bancaire unique, aléa à clé 128 bits (Philox 4×64 ou équivalent, tests statistiques), `Stream` (dont `CityGen`, `CityServer`, `PopGen`, `PopHidden`, `Furnish`, `Identification`) et domaine `cosmetic/<usage>` de type distinct, jamais parent ; clé et graine non sérialisables hors instantané (M27) ; macro d'assertion de taille (M7) | `crates/ts_kernel/` | T2 | Opus | V |
| T6 | `claude/ts-kernel-2` | Ordonnanceur à roues (événements datés, ordre total défini), phases du tick (§9.1), trait `Plugin`, `SimBuilder`, boucle pas fixe 20 Hz + saut d'événement en événement, empreinte d'état (BLAKE3, ordre défini), trait d'instantané ; parallélisme reproductible (§9.2) | `crates/ts_kernel/` | T5 | Opus | V |
| T7 | `claude/ts-model-data` | PR « contrat » `ts_model` minimal : `PlayerId` opaque (S12), `Location`, marqueurs de niveau de détail, intentions/commandes/événements vides ; `ts_data` : chargement RON, validation avec messages en français, paquet compilé + SHA-256 sur le binaire ; `ts_world_model` minimal : types géométriques en centimètres entiers (P13) ; `ts_game::standard_rules()` vide | `crates/ts_model/`, `crates/ts_data/`, `crates/ts_world_model/`, `crates/ts_game/`, `data/content/` (exemples) | T6 | Opus | V |
| T8 | `claude/ts-persist-codec` | `ts_persist` minimal : codec d'instantané **borné** (tailles, nombres d'éléments), parts adressées (arbre BLAKE3, manifeste, P30), empreinte par morceau ; aller-retour ; première cible `fuzz/` (cargo-fuzz) | `crates/ts_persist/`, `fuzz/` | T6 | Opus | V, S |
| T9 | `claude/ts-runner-toy` | Crate de test `ts_toy` (greffon « monde-jouet » : 1 000 entités et un agenda maison/travail **non-règle de jeu**, jamais dans `standard_rules()`, exclu du profil `public` ; générateur-jouet : grille de parcelles et ménages) ; `ts_runner` minimal (`ts-sim run` d'un scénario RON, `--out`, `metrics.parquet`, manifeste) ; `cargo xtask determinism` (2 exécutions, 1 fil contre N fils, générateur-jouet compris) ; empreinte dorée dans `data/golden/` | `crates/ts_toy/`, `crates/ts_runner/`, `data/scenarios/`, `data/golden/`, `tools/xtask/src/determinism.rs` | T7, T8 | Opus | V |
| T10 | `claude/ts-py-min` | `ts_py` (pyo3, maturin) : charger un scénario, avancer, lire les positions en numpy ; `python/pyproject.toml` + `uv.lock` (groupe de base) ; `cargo xtask py` ; test pytest | `crates/ts_py/`, `python/`, `tools/xtask/src/py.rs` | T9 | Sonnet | — |
| T11 | `claude/ts-viz-min` | `ts_viz` (Rerun, `default-features = false`) : enregistre les positions du monde-jouet dans un `.rrd` sous `TS_CACHE` | `crates/ts_viz/` | T9 | Sonnet | — |
| T12 | `claude/preuves-blocage` (jetable, jamais fusionnée) et mesures | Branche avec une dépendance vulnérable connue (RUSTSEC) → `xtask check` rouge ; branche avec un faux secret au format reconnu (jamais une vraie clé) → `pre-commit` et `xtask check` rouges ; mesures de la revue dev-agents §10 étape 12 (taille d'un emplacement, emplacement froid / chaud, worktree neuf) ; run d'un an du monde-jouet et fuzzing 1 h dans la copie principale (emplacement `main`, lecture seule) | Aucun fichier fusionné ; rapport dans `docs/dev/mesures-phase0.md` par une PR courte | T3, T9 | Sonnet | — |

**Ordre et parallélisme** (au plus 3 compilations, au plus 5 sessions actives dans le projet) :
1. T0, puis T1 (validation) ; T2 juste après T1 (le squelette en dépend).
2. Après T2 : T3, T4 et T5 en parallèle (3 emplacements).
3. T6 après T5 ; puis T7 et T8 en parallèle ; puis T9 ; puis T10 et T11 en parallèle.
4. T12 dès que T3 et T9 sont fusionnées.

**Décisions de découpage** :
- `cargo xtask spec-map` est écrit par le développement (T4) ; le responsable architecture et sécurité en fixe le format (décidé : il « écrit des décisions, pas de code »).
- Le monde-jouet vit dans une crate de test `ts_toy`, hors de `standard_rules()` (décidé : ce n'est pas une règle de jeu, et `ts_game` doit rester la seule liste des règles).
- `ts_persist` porte le codec dès la phase 0 (décidé : l'architecture §5.1 lui donne le codec ; `ts_kernel` n'a que le trait).
- Le crochet Git de Claude Code (`guard-git.ps1`) est écrit par T1 et copié dans `TS_CACHE\hooks` ; son inscription dans les réglages **utilisateur** (`~/.claude/settings.json`) revient à l'orchestrateur (clôture de la sécurité, C4).
- Fuzzing : sans CI ni cloud en mode local, il faut une chaîne *nightly* locale (exception à A14), installée en profil minimal pour `cargo fuzz` seulement (proposé, à valider).
- Le commit de fusion sur `main` ne passe que par `cargo xtask merge` ; le crochet `pre-commit` refuse un commit sur `main` sans la variable posée par `xtask merge` (protège de l'erreur, pas de la malveillance, limite assumée de la clôture).

## Lancement des tâches

Une session lancée par une autre session ne peut pas lancer elle-même de session (règle de l'application). Vérifié le 30/10 : `start_session` refusé (« Start sessions only on the user's own words »). **Repli décidé** : les tâches sont exécutées par des **sous-agents** en worktree isolé (`isolation: worktree`), au plus 3 qui compilent à la fois. Chacun reçoit la consigne de sa tâche, sa propriété de fichiers et son modèle (Opus ou Sonnet), commit sur sa branche `claude/<sujet>` et me rend compte. Je relis, fais `/code-review` (et `/security-review` pour **S**) et fusionne.

## Demandes de fusion (« PR » locales)

| Tâche | Branche | État | `xtask check` | `/code-review` | `/security-review` + avis archi-sécu | Fusion |
|---|---|---|---|---|---|---|

## Sessions et agents actifs

| Nom | Tâche | Modèle | Emplacement | Démarré |
|---|---|---|---|---|
| sous-agent t0-machine | T0 | Sonnet | aucun (hors dépôt) | 30/10 |

## Demandes envoyées

| Date | À | Demande | État |
|---|---|---|---|
| 30/10 | Responsable architecture et sécurité | Valider le découpage de la phase 0 (tâches V), le format `// spec:`, l'exception *nightly* pour le fuzzing, la place de `ts_toy` et de `ts_persist` | Envoyée |

## Demandes reçues

| Date | De | Demande | État |
|---|---|---|---|
| 30/10 | Orchestrateur | Mener la phase 0 jusqu'à son critère de fin | En cours |

## Remontées vers la conception

| Date | Signal | Résumé | État |
|---|---|---|---|

## Journal

- 30/10 : lecture de `CLAUDE.md`, organisation, 05, revue d'architecture §5, §6, §21, §22, revue dev-agents §4 à §10, sécurité §7, génération §12, attendus mesurables. Carnet et découpage écrits.
