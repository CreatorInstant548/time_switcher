# Carnet du responsable développement

> Tenu par le responsable développement seul. Règles : [organisation.md](../time_switcher/docs/pilotage/organisation.md). Mis à jour le 30/10/2026.

## État

- Phase 0 (fondations) en cours depuis le 30/10. **Mode local levé le 30/10** : dépôt privé `CreatorInstant548/time_switcher` ; vraies PR ; carnet poussé après chaque commit ; demandes entre rôles en *issues* `role:*`.
- Découpage **validé avec corrections** par archi-sécu le 30/10 ([issue #2](https://github.com/CreatorInstant548/time_switcher/issues/2), son §9.1) : feu vert pour lancer dans l'ordre, consignes avec C1 à C12.
- `main` : PR #1 fusionnée (`CLAUDE.md`, fin du mode local), f60e5bc. Aucun code encore.

## Critère de fin de la phase 0 (05 §5, point 0 ; revue d'architecture §22 ; sécurité §7 ; génération §12)

| N° | Condition | Preuve attendue | État |
|---|---|---|---|
| F1 | Monde-jouet de 1 000 entités qui suivent un agenda, un an simulé en saut de temps, même empreinte à chaque exécution, sur 1 fil et sur N fils (M1, M2) | `cargo xtask determinism` + run `sim` dans la copie principale | — |
| F2 | La vérification bloque une dépendance vulnérable **et** un secret commis (CI GitHub : contrôle `ci-ok` rouge ; en local : `cargo xtask check` et crochet `pre-commit`) | Deux PR jetables rouges, fermées sans fusion | — |
| F3 | Un worktree neuf compile dans un emplacement partagé | Mesure de T12 (froid / chaud) | — |
| F4 | Aller-retour d'instantané vert ; Python lit l'état | Tests `ts_kernel`/`ts_persist` ; test `ts_py` | — |
| F5 | Sécurité : `forbid(unsafe_code)` vérifié ; `public` incompatible avec `dev-commands`, `dev-introspection`, `external-pilot` (M28) ; `PlayerId` opaque ; codec d'instantané borné avec empreinte par morceau ; fuzzing du codec une heure sans plantage ; aucune clé ni graine exposée (M27) | `xtask check`, test de non-compilation, fuzz 1 h | — |
| F6 | Génération : `draw` à clé de 128 bits avec les flux `CityGen`, `CityServer`, `PopGen`, `PopHidden`, `Furnish` ; espace de `StableId` réservé à la carte ; géométrie en centimètres entiers ; format de parts adressées (BLAKE3) ; générateur-jouet identique sur 1 et N fils | Tests | — |
| F7 | Assertions de taille des composants à la compilation (M7) | Macro dans `ts_kernel` | — |

## Découpage de la phase 0

> Version 2 du 30/10, alignée sur la grille V0-01 à V0-13 et les exigences SEC0-01 à 34 du carnet `architecture-securite.md` (§3, §5, §6, §7). La version 1 est remplacée.

Légende :
- **V** : la consigne est envoyée au responsable architecture et sécurité **avant** le lancement, qui répond « validé », « validé avec corrections » ou « refusé » (son §9).
- **S** : chemin sensible. Avant la fusion : `/security-review`, relecture par un sous-agent neuf distinct de l'auteur, et avis favorable d'archi-sécu sur l'arbre exact.

Chaque tâche = une branche `claude/<sujet>` = un worktree = une PR GitHub, d'environ 400 lignes au plus hors fichiers générés.

| Tâche | Grille | Branche | Contenu | Propriété (seuls chemins modifiables) | Dépend de | Modèle | Marques |
|---|---|---|---|---|---|---|---|
| T0 | — | aucune | `C:\Dev\ts-cache\` ; `cargo-audit` ; `gitleaks` binaire officiel vérifié par SHA-256 dans `C:\Dev\tools\gitleaks\` | Hors dépôt | — | Sonnet | — |
| T1 | V0-01, V0-02 (en partie) | `claude/outillage-agents` | `tools/dev/session-start.ps1` (baux, jonctions, alerte sous 40 Go) ; `tools/dev/guard-claude.ps1` (crochet Claude Code de SEC0-14 : toutes les variantes git/gh, plus Edit/Write dans la copie principale), destiné aux réglages **utilisateur** ; `tools/dev/hooks/pre-commit` (gitleaks sur l'index, toutes branches ; garde des gros fichiers SEC0-11 ; refus de tout commit sur `main`, D-DEV-03), `pre-push` (gitleaks sur la plage ; refus de `main`, de `+` et de `--force`), `commit-msg` (format A31) ; `tools/dev/tools.lock` (versions et SHA-256, SEC0-06) ; `.claude/settings.json` (SessionStart ; PreToolUse `Edit|Write|NotebookEdit` → `guard-main.ps1` ; PreToolUse `Bash|PowerShell` → `guard-git.ps1`) ; SEC0-35 : refus d'écrire `.claude/settings*.json` et de mentionner `disableAllHooks`, sauf sur une branche `claude/outillage-*` ; `.gitignore` (`id_*`, `.claude/settings.local.json`, SEC0-09) ; `docs/dev/outillage.md` ; vérifications bloquantes de dev-agents §6.3, repli `CARGO_TARGET_DIR` si échec ; installation provisoire des crochets par `tools/dev/install-hooks.ps1`, remplacée par `xtask hooks install` en T3 | `tools/dev/`, `.claude/settings.json`, `.gitignore`, `docs/dev/` | T0 | Opus | V, S |
| T2a | V0-03, V0-05 (profils), V0-13 (constante) | `claude/espace-travail` | `Cargo.toml` racine (`members`, `default-members` sans `ts_viz`, `ts_py`, `ts_client_native`, `xtask` ; `exclude = [".claude", "fuzz"]` ; profils A12 ; `[workspace.lints]` avec `unsafe_code = "forbid"` ; *resolver* 3) ; `rust-toolchain.toml` 1.99.1 ; `.cargo/config.toml` (alias `xtask`) ; `deny.toml` (SEC0-02) ; `clippy.toml` (SEC0-22, architecture §9.2) ; `rustfmt.toml` ; squelette `crates/ts_kernel` : `#![forbid(unsafe_code)]`, fonctionnalités `dev-commands`, `dev-introspection`, `external-pilot`, `dev-bench`, `public` avec `compile_error!`, constante `BUILD_PROFILE` | Fichiers racine cités, `.cargo/`, `crates/ts_kernel/` (squelette) | T1 | Opus | V, S |
| T2b | V0-04 | `claude/xtask-check` | `tools/xtask` : `check` (fmt ; clippy `-D warnings --locked` ; tests des crates touchées ; déterminisme si le noyau change ; `cargo deny check` ; `cargo audit` si `Cargo.lock` change, base RustSec de moins de 7 jours ; gitleaks ; `forbid(unsafe_code)` avec la liste fermée de SEC0-19 ; échec attendu de `cargo check -p ts_kernel --features public,<f>` pour les 4 fonctionnalités ; absence de chemins locaux, SEC0-26 ; `spec-map check` dès qu'il existe) ; script de contrôle des workflows (SEC0-30, 31) ; test `cargo tree` : aucune crate de règles ne dépend de `ts_toy` ; `slots` ; `disk [--clean]` | `tools/xtask/` | T2a | Opus | V, S |
| T13 | SEC0-29 à 34 | `claude/ci` | `.github/workflows/ci.yml` (`changes`, `pr-title`, `security` : gitleaks sur la plage, épinglage des actions, `cargo-deny`, `cargo-audit` ; `rust-linux` ; `rust-windows` sur chemins sensibles ; `ci-ok`) ; `periodic.yml` hebdomadaire (Windows, fuzzing 60 min, `cargo-audit`, gitleaks sur tout l'historique) ; `dependabot.yml` (cargo, github-actions) ; `.gitleaks.toml` ; `pull_request_template.md` avec les deux questions de l'organisation §3.1 ; actions épinglées par empreinte ; `permissions: contents: read` ; budget ≤ 2 000 min/mois | `.github/`, `.gitleaks.toml` | T2b | Opus | V, S |
| T4 | V0-11 | `claude/xtask-spec-map` | `cargo xtask spec-map report|check|diff` selon le §7 d'archi-sécu, sans dépendance nouvelle, tests sur un corpus factice | `tools/xtask/src/spec_map*` et sa ligne de commande | T2b | Sonnet | V, S |
| T3 | V0-02 | `claude/xtask-merge` | `xtask hooks install|verify` (copie épinglée par empreinte dans `TS_CACHE\hooks\<sha256>\`, `core.hooksPath`) ; `xtask merge <PR>` selon SEC0-15 révisée (copie principale propre sur `main` ; `verify-main` ; empreinte des crochets ; `ci-ok` vert sur le SHA de tête, sans relancer `xtask check` ; branche à jour ; pour un chemin sensible, commentaires `review-ok: <sha>` et `archsec-ok: <sha>` sur ce SHA ; `gh pr merge --squash --delete-branch --match-head-commit <sha>` ; `git pull --ff-only`), sans attestation ni registre local ; `xtask hooks verify` étendu (SEC0-35) : parcourt `git worktree list`, vérifie que chaque `.claude/settings.json` est identique à celui de la tête de sa branche, qu'aucun `settings.local.json` ne contient `disableAllHooks` ni de crochets qui remplacent ceux du projet, que `core.hooksPath` pointe vers la copie épinglée ; sur `main`, empreinte de `settings.json` comparée à la dernière valeur approuvée inscrite par `hooks install` ; un écart suspend les fusions et ouvre une issue `role:architecture` ; appelé par `xtask merge` et `verify-main` ; `xtask verify-main` (SEC0-17) ; `xtask test-barriers` (SEC0-18) | `tools/xtask/`, `tools/dev/hooks/` (ajustements) | T2b, T13 | Opus | V, S |
| T5 | V0-05, V0-13 (M27) | `claude/ts-kernel-1` | `SimTime(i64)`, `SimDay(u16)`, `StableId(u64)` avec espace de noms réservé à la carte (P28), `Money(i64)` et arrondi bancaire unique ; aléa à clé de 128 bits : ChaCha8 en mode compteur, clés de flux et clé cosmétique dérivées par BLAKE3 `derive_key`, coût par bloc mesuré dans la PR ; flux en énumération fermée (`CityGen`, `CityServer`, `PopGen`, `PopHidden`, `Furnish`, `Identification`, etc.) ; domaine `cosmetic/<usage>` de type distinct, jamais parent ; clé de test depuis `u64`, nommée non sûre et interdite avec `public` ; test de M27 ; macro d'assertion de taille (M7) | `crates/ts_kernel/` | T3 | Opus | V, S (aléa) |
| T6 | V0-05 | `claude/ts-kernel-2` | Ordonnanceur à roues (ordre total) ; phases du tick (architecture §9.1) ; trait `Plugin` ; `SimBuilder` ; boucle à pas fixe de 20 Hz et saut d'événement en événement ; empreinte d'état BLAKE3 en ordre défini ; trait d'instantané ; parallélisme reproductible (§9.2) | `crates/ts_kernel/` | T5 | Opus | V |
| T7a | V0-07 | `claude/ts-model-contrat` | PR « contrat » : `ts_model` (`PlayerId` opaque de 128 bits, SEC0-21 ; `Location` ; marqueurs de niveau de détail ; squelettes des intentions, commandes et événements) ; `ts_game::standard_rules()` vide | `crates/ts_model/`, `crates/ts_game/` | T6 | Opus | V |
| T7b | V0-07 | `claude/ts-data-min` | `ts_data` : RON borné (SEC0-25), validation avec messages en français, paquet compilé et SHA-256 sur le binaire (A4) ; fiches d'exemple | `crates/ts_data/`, `data/content/` | T7a | Sonnet | V |
| T8 | V0-06 | `claude/ts-persist-codec` | `ts_persist` réduit au codec borné (SEC0-23) et aux parts BLAKE3 avec manifeste (P30) ; aller-retour ; cible `fuzz/` du décodeur | `crates/ts_persist/`, `fuzz/` | T6 | Opus | V, S |
| T9w | V0-08 | `claude/ts-world-model-min` | `ts_world_model` minimal : types géométriques en centimètres entiers (P13) | `crates/ts_world_model/` | T6 | Sonnet | V |
| T9 | V0-09 | `claude/ts-runner-toy` | Crate de test `ts_toy` (monde-jouet de 1 000 entités avec un agenda maison/travail, **pas une règle de jeu** : hors de `standard_rules()`, jamais dans une dépendance normale d'un hôte ; générateur-jouet avec une grille de parcelles et des ménages) ; `ts_runner` (`ts-sim run` d'un scénario RON, `--out` dans `TS_CACHE`, `metrics.parquet`, manifeste sans chemin local) ; `xtask determinism` (2 exécutions, 1 fil contre N fils, générateur-jouet compris, aller-retour d'instantané) ; empreinte dorée dans `data/golden/` | `crates/ts_toy/`, `crates/ts_runner/`, `data/scenarios/`, `data/golden/`, `tools/xtask/src/determinism*` | T7b, T8, T9w | Opus | V |
| T10 | V0-10 | `claude/ts-py-min` | `ts_py` (pyo3, maturin) : charger, avancer, lire les positions en numpy ; exception `unsafe` de SEC0-19 si les macros l'exigent ; `python/pyproject.toml` et `uv.lock` (groupe de base) ; `xtask py` ; pytest | `crates/ts_py/`, `python/`, `tools/xtask/src/py*` | T9 | Sonnet | V |
| T11 | V0-10 | `claude/ts-viz-min` | `ts_viz` (Rerun, `default-features = false`) : positions du monde-jouet en `.rrd` sous `TS_CACHE` ; si `rerun` tire une crate réseau bannie (SEC0-02), arrêt et avis d'archi-sécu | `crates/ts_viz/` | T9 | Sonnet | V |
| T12 | V0-12, V0-13 | `claude/preuves-blocage-*` (jetables), puis `claude/mesures-phase0` | Preuves de SEC0-27 :<br>- une dépendance frappée d'un avis RustSec est refusée par `xtask check`, `ci-ok` et `xtask merge` ;<br>- un faux secret est refusé par le `pre-commit` (jamais poussé, aucun crochet contourné) ;<br>- côté CI, l'auto-test du travail `security` (R2).<br>Mesures de dev-agents §10.12 ; worktree neuf dans un emplacement partagé ; run d'un an du monde-jouet, deux fois ; fuzzing d'une heure ; audit hebdomadaire en place ; rapport `docs/dev/mesures-phase0.md` | `docs/dev/mesures-phase0.md` seulement ; branches jetables jamais fusionnées | T3, T9, T13 | Sonnet | — |

**Ordre et parallélisme** (au plus 3 compilations, au plus 5 sessions actives dans le projet) :
1. T0, T1, T2a, T2b, l'une après l'autre (elles touchent `tools/` et les fichiers racine partagés).
2. Après T2b : T13 et T4 en parallèle ; puis T3, qui a besoin de la CI pour `ci-ok`.
3. Après T3 : T5, seule ; puis T6.
4. Après T6 : T7a, T8 et T9w en parallèle ; T7b après T7a.
5. T9 ; puis T10 et T11 en parallèle.
6. T12.

**Fusions avant T3** : par `gh pr merge --squash --delete-branch`, après `cargo xtask check` (ou les commandes équivalentes avant T2b) et `/code-review`. Pour une tâche **S**, il faut en plus `/security-review`, une relecture neuve et l'avis d'archi-sécu. Après T3, la fusion passe seulement par `cargo xtask merge <PR>`.

**Décisions de découpage (décidé)** :
- **D-DEV-01** : `spec-map` est écrit par le développement (T4), selon la spécification d'archi-sécu, qui donne son avis avant la fusion.
- **D-DEV-02** : le monde-jouet et le générateur-jouet vivent dans une crate de test, `ts_toy`, hors de `standard_rules()`. *Pourquoi : ce ne sont pas des règles de jeu, et `ts_game` doit rester la seule liste des règles.*
- **D-DEV-03** : avec GitHub, la fusion se fait côté serveur. Il n'y a donc **aucun commit local sur `main`**, et le `pre-commit` refuse tout commit sur `main`, sans exception. L'arbre est vérifié par la CI (`ci-ok` sur le SHA de tête), et les approbations sont des commentaires de PR liés à ce SHA (R1, C3c).
- **D-DEV-04** : le crochet Claude Code (`guard-git.ps1`) est inscrit dans le `.claude/settings.json` **du projet**, versionné (décision de l'orchestrateur du 30/10 : il ne modifie pas les réglages personnels de l'utilisateur). Avis favorable d'archi-sécu sous condition de SEC0-35 (T1 et T3). Risque résiduel accepté (R-ARCH-04) : une session qui retire le crochet de son worktree est détectée à la fusion suivante, et les crochets Git épinglés bloquent toujours `main`.
- **D-DEV-05** : le fuzzing d'une heure passe par la tâche hebdomadaire de la CI Linux (`cargo-fuzz`, avec *nightly* en CI seulement, A14). Pas de *nightly* locale (R4) : si la CI ne suffit pas, archi-sécu est saisi.
- **D-DEV-06** : les tâches sont exécutées par des sous-agents en worktree isolé (voir « Lancement des tâches ») ; le relecteur d'un chemin sensible est un sous-agent neuf, distinct de l'auteur, qui pose `review-ok: <sha>` (C12, validé).

**Validation** : archi-sécu, 30/10 : v1 **validée avec corrections C1 à C12** (son §9.1), v2 **validée avec corrections R1 à R5** et **feu vert pour T1** (son §9.2, issue #2). Corrections intégrées ci-dessus et reprises dans chaque consigne.

## Lancement des tâches

Une session lancée par une autre session ne peut pas lancer elle-même de session (règle de l'application). Vérifié le 30/10 : `start_session` refusé (« Start sessions only on the user's own words »). **Repli décidé** : les tâches sont exécutées par des **sous-agents** en worktree isolé (`isolation: worktree`), au plus 3 qui compilent à la fois. Chacun reçoit la consigne de sa tâche, sa propriété de fichiers et son modèle (Opus ou Sonnet), commit sur sa branche `claude/<sujet>` et me rend compte. Le sous-agent pousse sa branche et ouvre la PR. Je relis, fais `/code-review` (et `/security-review` + avis d'archi-sécu pour **S**) et fusionne par `gh pr merge --squash --delete-branch` (puis `cargo xtask merge` une fois T3 et T13 fusionnées).

## PR

| Tâche | Branche / PR | État | `xtask check` | `/code-review` | `/security-review` + avis archi-sécu | Fusion |
|---|---|---|---|---|---|---|
| (orchestrateur) | `claude/fin-mode-local`, #1 | Fusionnée | sans objet (documentation) | relue | sans objet | f60e5bc |
| (conception) | `claude/da-declaration-ia`, #4 | Fusionnée | sans objet (documentation, pas encore de CI) | relue | sans objet | de84d92 |
| (orchestrateur) | `claude/sous-agents`, #3 | Fusionnée | sans objet (documentation) | relue | sans objet | 4991209 |
| T1 | `claude/outillage-agents`, [#5](https://github.com/CreatorInstant548/time_switcher/pull/5), tête 41e0a7d | En relecture | sans objet (pas encore de xtask) ; tests du sous-agent : guards 131/131, hooks 16/16, slots 16/16 | **9 constats** (1 bloquant : emplacement pris par toute session de worktree) | relecture neuve : **refusée** (B1 alias `--no-verify` ; I1 outils hors matcher ; I2 `main` déplacé sans commit ; I3 gitleaks désactivable ; I4 `gh pr merge`/`gh api`) ; lot de 13 corrections renvoyé à l'auteur | — |

## Sessions et agents actifs

| Nom | Tâche | Modèle | Emplacement | Démarré |
|---|---|---|---|---|
| sous-agent t0-machine | T0 | Sonnet | aucun (hors dépôt) | 30/10, **terminée** |
| sous-agent t1-outillage | T1 | Opus | worktree isolé, `claude/outillage-agents` | 30/10 |

## Demandes envoyées

| Date | À | Demande | État |
|---|---|---|---|
| 30/10 | Responsable architecture et sécurité | Valider le découpage de la phase 0 — [#2](https://github.com/CreatorInstant548/time_switcher/issues/2) | Validé avec corrections C1 à C12 |

## Demandes reçues

| Date | De | Demande | État |
|---|---|---|---|
| 30/10 | Orchestrateur | Mener la phase 0 jusqu'à son critère de fin | En cours |
| 30/10 | Orchestrateur | Fin du mode local : fusionner la PR #1, pousser les carnets, vraies PR | Fait (PR #1 fusionnée, f60e5bc) |

## Remontées vers la conception

| Date | Signal | Résumé | État |
|---|---|---|---|

## Journal

- 30/10 : lecture de `CLAUDE.md`, organisation, 05, revue d'architecture §5, §6, §21, §22, revue dev-agents §4 à §10, sécurité §7, génération §12, attendus mesurables. Carnet et découpage écrits.
- 30/10 : mode local levé par l'orchestrateur. PR #1 fusionnée (squash), `main` à jour. Issue #2 ouverte pour archi-sécu. Ajout de T13 (CI). Aucune branche locale de tâche n'existait encore.
- 30/10 : exigences d'archi-sécu lues (SEC0-01 à 28, grille V0-01 à 13, spec-map §7). Découpage v2 : T2 scindée (T2a, T2b), T7 scindée (T7a, T7b), T9w ajoutée, T3 en mode GitHub, ordre de la grille.
- 30/10 : découpage validé avec corrections par archi-sécu ; v2 corrigée (ni attestation ni registre local ; ChaCha8 et BLAKE3 ; approbations par commentaires liés au SHA).
- 30/10 : **T0 terminée** : `C:\Dev\ts-cache\` créé ; cargo-deny 0.20.2, cargo-audit 0.22.2, gitleaks 8.30.1 (`C:\Dev\tools\gitleaks\`, archive SHA-256 d29144de…afc4e conforme au fichier officiel). **T1 lancée** (sous-agent Opus, worktree isolé).
- 30/10 : PR #4 (direction artistique v0.4.2, documentation) fusionnée, de84d92. Orchestrateur : `guard-git.ps1` dans le `.claude/settings.json` **du projet**, pas dans les réglages utilisateur (décision de l'orchestrateur) ; consigne de T1 modifiée ; archi-sécu informé. **Contrainte** : une session peut modifier le `settings.json` de son worktree ; restent `pre-push`/`pre-commit` épinglés, `ci-ok`, `xtask merge`.
- 30/10 : PR #3 (organisation, repli par sous-agents, règle de commit sur `pilotage`) fusionnée, 4991209. Archi-sécu : SEC0-35 ajoutée à T1 (consigne envoyée au sous-agent) et à T3.
- 30/10 : T1 livrée (PR #5). `/code-review` élevé : 9 constats. Relecture de sécurité par un sous-agent neuf lancée. Corrections à envoyer en un lot.
- 30/10 : relecture de sécurité de la PR #5 : refusée (1 bloquant, 4 importants, 6 mineurs ; commentaire sur la PR). Lot de 13 corrections envoyé au sous-agent T1. Décisions : `tools/dev/merge-pr.ps1` seul chemin de fusion jusqu'à T3 ; crochet `reference-transaction` sur `main` ; emplacements pris à la demande (`session-start.ps1 -Acquire`), plus au démarrage ; coût de `pwsh` reporté à T3 ; taille de la PR acceptée (exception notée). Archi-sécu informé.
