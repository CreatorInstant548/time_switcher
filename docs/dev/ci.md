# Intégration continue (GitHub Actions)

Mode d'emploi de `.github/` et de `tools/ci/` (tâche T13 de la phase 0). Sources :
- [revue du développement par agents](../conception/revues/developpement-agents.md) §8.2 et §8.3 ;
- les exigences SEC0-29 à SEC0-34 du carnet du responsable architecture et sécurité, avec les corrections G1 à G12 de son §9.6.

`.github/` et `tools/ci/` sont des **chemins sensibles** : `/security-review`, un relecteur neuf et l'avis `archsec-ok` d'archi-sécu.

## 1. Fichiers

| Fichier | Rôle |
|---|---|
| `.github/workflows/ci.yml` | CI de chaque PR (hors brouillon) et de chaque push sur `main` |
| `.github/workflows/nightly.yml` | Tâche périodique sur `main` (§4) |
| `.github/dependabot.yml` | Mises à jour de Cargo et des actions (§5) |
| `.github/pull_request_template.md` | Modèle de PR (SEC0-34) ; pas de `CODEOWNERS` |
| `tools/ci/changes.sh` | Zones touchées par la plage de la PR (§2) |
| `tools/ci/sensitive-paths.txt` | Liste unique des chemins sensibles (G8), lue aussi plus tard par `xtask merge` et `origin-check` |
| `tools/ci/install-tools.sh` | Installation épinglée des outils et auto-test de gitleaks (§3) |
| `tools/ci/main-changed.sh` | `main` a-t-il changé depuis la dernière exécution hebdomadaire réussie (§4) |

`tools/ci/` ne contient aucune règle de contrôle des workflows : la seule implémentation est l'étape `workflows` de `cargo xtask check` (G7).

## 2. Travaux de `ci.yml` et filtres de chemins

Déclencheurs : `pull_request` (`opened`, `synchronize`, `reopened`, `ready_for_review`) et `push` sur `main`. Chaque travail porte la condition `github.event_name != 'pull_request' || !github.event.pull_request.draft` (G1) : un brouillon ne lance rien, un push sur `main` lance tout. `permissions: {}` au niveau du workflow, `contents: read` par travail au plus ; `actions/checkout` avec `persist-credentials: false` et `lfs: false`.

| Travail | Quand | Contenu |
|---|---|---|
| `changes` | toujours | `tools/ci/changes.sh --summary` : tableau des zones et chemins sensibles touchés, dans le résumé de l'exécution |
| `pr-title` | PR seulement | titre au format `type(portée): description`, même motif que le crochet `commit-msg` ; le titre passe par `env:` (SEC0-31) |
| `security` | toujours, quels que soient les chemins | gitleaks épinglé et son auto-test ; `cargo xtask check --only workflows,gitleaks,deny`, plus `audit` si `Cargo.lock` change. Avertit si la PR touche la zone `ci` (G5 étendu, voir plus bas) |
| `rust-linux` | zone `rust` ou `ci` | `cargo xtask check --all --only fmt,clippy,test,unsafe,profiles,features,clippy-exemptions,local-paths,spec-map` |
| `rust-windows` | push sur `main`, étiquette `ci:windows`, ou zone `sensitive` | même chose, sans `fmt` |
| `ci-ok` | toujours (`always()`) | lit `needs.<travail>.result` (G2) : `changes`, `security` et, sur une PR, `pr-title` doivent valoir `success` ; `rust-linux` et `rust-windows` peuvent valoir `skipped`. Sur un brouillon, il **échoue** au lieu d'être sauté, car GitHub tient un contrôle sauté pour réussi |

**Zones** (`changes.sh`), calculées sur `git diff <base commune> <tête>` ; une base absente ou inconnue retient toutes les zones :
- `rust` : `crates/`, `tools/xtask/`, `fuzz/`, `.cargo/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`, `deny.toml` ;
- `python` : `python/`, `*.py`, `pyproject.toml`, `uv.lock` ; `web` : `admin-web/`, `package.json`, `pnpm-*.yaml` ; `client` : `client/` (aucun travail encore) ;
- `ci` : ce que la CI exécute pour se contrôler, soit `.github/`, `tools/ci/`, `tools/xtask/`, `tools/dev/tools.lock`, `tools/dev/hooks/gitleaks.toml` ; `docs` : `docs/`, `*.md` ;
- `sensitive` : `tools/ci/sensitive-paths.txt`, **union** de sa version d'`origin/main` et de celle de la PR (une PR ne peut pas s'en retirer). Les sections `[features]` et `[profile.*]` d'un `Cargo.toml` sont comparées entre la base et la tête, clés pointées de la racine comprises ; `Cargo.lock` est sensible quand il ajoute un nom de crate ;
- `lock` : `Cargo.lock` ; `gitleaks_config` : `tools/dev/hooks/gitleaks.toml`, `tools/dev/tools.lock`.

**Pourquoi chaque travail recalcule ses zones** (décidé) : l'étape `workflows` de `xtask check` refuse toute interpolation hors de `env:`, `with:` et `if:`, donc aussi dans `jobs.<id>.outputs` et `concurrency.group`. Un travail ne peut donc pas transmettre ses zones aux autres, et `ci.yml` n'a pas de `concurrency`. `rust-linux` et `rust-windows` démarrent toujours, recalculent les zones, puis sautent leurs étapes si rien ne les concerne : ils valent alors `success`. L'ajout de ces deux clés au contrôle est demandé (§7).

**Limite de confiance** (relecture neuve, I1) : sur `pull_request`, GitHub exécute le `ci.yml`, les scripts, `xtask` et `tools.lock` **de la PR**. Pour une PR de la zone `ci`, `ci-ok` ne garantit donc rien : le travail `security` l'avertit dans le résumé, et la protection repose sur la relecture des chemins sensibles et `archsec-ok`. Les sources des archives sont limitées aux publications officielles de gitleaks et de cargo-deny. **Demandé à T3** (`xtask merge`, `origin-check`) : recalculer les chemins sensibles avec le code et la liste d'`origin/main`, et **revérifier le titre** de la PR juste avant la fusion (il devient le message du squash, SEC0-16, et peut être modifié après un `ci-ok` vert, l'événement `edited` n'étant pas écouté pour ne pas tout relancer à chaque retouche du corps).

**L'étiquette `ci:windows`** est lue au prochain push de la PR : l'événement `labeled` ne relance pas la CI (décidé, pour ne pas payer une exécution par étiquette).

**Branche `pilotage`** : l'étape `spec-map` (T4) lit `origin/pilotage`. `fetch-depth: 0` récupère toutes les branches avec le jeton du checkout, sans le garder dans la configuration (`persist-credentials: false`). `rust-linux` et `rust-windows` vérifient sa présence avant de compiler.

**Profil** : `xtask check` ne prend pas encore de profil. Le profil `ci` de `Cargo.toml` est reproduit sur `dev` par `CARGO_INCREMENTAL=0` et `CARGO_PROFILE_DEV_DEBUG=0`.

**Cache** : `Swatinem/rust-cache`, avec `save-if` sur `refs/heads/main` (G10) : une PR lit le cache de `main`, sans jamais l'écrire. `cargo-audit` est mis en cache par `actions/cache`, enregistré depuis `main` seulement.

## 3. Outils épinglés

Les valeurs viennent de `tools/dev/tools.lock` (clés `*.linux.*`, même version que sous Windows, G6) ; aucune action tierce d'installation.

- **Chaîne Rust** : canal de `rust-toolchain.toml`, installé par la `rustup` de l'image.
- **gitleaks** : archive téléchargée en HTTPS depuis `github.com`, SHA-256 de l'archive puis de l'exécutable vérifiés, installé dans `TS_CACHE/ci/gitleaks/` (`TS_CACHE` vaut `$RUNNER_TEMP/ts-cache` en CI). La configuration posée à côté est `tools/dev/hooks/gitleaks.toml` **d'`origin/main`** (`git show`, G5).
- **Auto-test** (`install-tools.sh gitleaks-selftest`, G4) : un faux jeton `ghp_` est fabriqué à l'exécution par concaténation, jamais commité ni affiché (`--redact`). gitleaks doit le trouver dans un dossier, puis dans un commit marqué `-diff` lu avec `--text`. Sinon le travail échoue.
- **`cargo xtask check`, étape `gitleaks`, mode CI** (A2) : actif quand `CI=true` et `GITHUB_ACTIONS=true`. Binaire désigné par `gitleaks.linux.path` (relatif à `TS_CACHE`) et vérifié par `gitleaks.linux.sha256` ; configuration refusée si elle diffère d'un octet de celle d'`origin/main`. Options identiques au mode local. En local, rien ne change.
- **cargo-deny** : binaire officiel (`x86_64-unknown-linux-musl`), archive vérifiée par son SHA-256.
- **cargo-audit** : `cargo install --locked --version`, dans `~/ts-tools/cargo-audit`.
- **cargo-fuzz** : `cargo install --locked --version` avec la *nightly* datée `FUZZ_TOOLCHAIN` de `nightly.yml` ; sa version s'ajoute à `tools.lock` avec `fuzz/`.

**Actions** (liste fermée, épinglées par SHA de 40 caractères, version en commentaire) :

| Action | Version | SHA |
|---|---|---|
| `actions/checkout` | v7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| `actions/cache` (`restore`, `save`) | v6.1.0 | `55cc8345863c7cc4c66a329aec7e433d2d1c52a9` |
| `Swatinem/rust-cache` | v2.9.2 | `6323deb102c322ba6fcbdcafc7e3dddab59af2b6` |
| `actions/upload-artifact` | v7.0.2 | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` |

`Swatinem/rust-cache` reste en v2.9.2 : la v2.9.3 avait moins de 14 jours (même règle que le `cooldown` de Dependabot).

## 4. Tâche périodique (`nightly.yml`)

- **Chaque nuit** à 03:17 UTC, travail `advisories` : `cargo audit`, puis `cargo deny --locked check advisories`.
- **Le dimanche** (même heure), en plus, et seulement si `main` a changé depuis la dernière exécution hebdomadaire réussie (`main-changed.sh`, par l'API, avec `actions: read`) :
  - `windows` : `cargo xtask check --all` hors étapes de sécurité de `ci.yml` ;
  - `gitleaks-history` : gitleaks sur tout l'historique de toutes les branches, `--text`, avec son auto-test ;
  - `fuzz` : 60 min réparties entre les cibles de `fuzz/`. **Sauté** tant que `fuzz/` n'existe pas. Les plantages sont téléversés, gardés 7 jours.
- **`workflow_dispatch`** (essais) lance tout, sans contrôle de changement de `main`. Aucune entrée (`inputs`).
- **Limite acceptée** (décidé) : « réussie » porte sur toute l'exécution. Tant qu'`advisories` échoue sur un avis RustSec, les travaux hebdomadaires repassent chaque dimanche, même sans changement de `main` (≈ 30 min de plus par semaine). L'*issue* `nightly-failure` signale déjà la cause.
- Un push sur `main` n'analyse que l'arbre (plage `origin/main..HEAD` vide) : ses commits ont été analysés dans la PR, et le seront dans l'historique complet du dimanche.
- **Échec ou annulation** (dépassement de délai compris) : le travail `report`, seul avec `issues: write`, sans checkout, ouvre l'*issue* `nightly-failure` ou la complète d'un commentaire. Le texte ne contient que les noms des travaux en échec et le lien de l'exécution (G9).

## 5. Dependabot

`cargo` et `github-actions`, chaque lundi, regroupés en une PR par écosystème, `open-pull-requests-limit: 2`, `cooldown: default-days: 7`, titres `build(deps): …`. Ses PR touchent `Cargo.lock` ou `.github/` : elles passent par l'avis d'archi-sécu.

## 6. Lire un échec

1. Ouvrir la PR, puis **Checks** : le premier travail rouge, hors `ci-ok`, est la cause ; `ci-ok` ne fait que la reporter, et son journal donne le résultat de chaque travail.
2. `changes` et `security` écrivent un résumé (zones, chemins sensibles, avertissement G5) sur la page de l'exécution.
3. En local : `gh pr checks <n>`, puis `gh run view <id> --log-failed`.
4. Reproduire : `cargo xtask check --all --only <étape>`, avec le nom d'étape affiché dans le journal.
5. gitleaks n'affiche jamais un secret en clair (`--redact`). Un secret trouvé est tenu pour fuité dès qu'il a été poussé : le révoquer, puis corriger.

## 7. Budget

Offre gratuite : **2 000 min par mois** (M36). Un travail est compté à la minute supérieure, et une minute Windows compte double. Le responsable développement relève la consommation chaque semaine ; **à 80 % (1 600 min)**, il passe en **mode minimal** : plus de `rust-windows` sur PR (étiquette retirée, chemins sensibles vérifiés sous Windows par la tâche hebdomadaire seulement), et `nightly.yml` réduit à `advisories`.

**Coût estimé par exécution** (minutes facturées, à recaler sur les relevés) :

| Cas | `changes` | `pr-title` | `security` | `rust-linux` | `rust-windows` | `ci-ok` | Total |
|---|---|---|---|---|---|---|---|
| PR de documentation | 1 | 1 | 3 | 1 | 2 | 1 | ≈ 9 |
| PR Rust, cache chaud | 1 | 1 | 3 | 4 à 6 | 2 | 1 | ≈ 12 à 14 |
| PR sur un chemin sensible | 1 | 1 | 3 | 4 à 6 | 2 × (8 à 15) | 1 | ≈ 26 à 42 |

Tâche périodique : `advisories` ≈ 2 min par nuit (≈ 60 par mois) ; le dimanche, si `main` a changé, Windows 2 × (8 à 15), gitleaks 1 à 2, fuzzing 61 à 62 (une fois `fuzz/` créé) : ≈ 80 à 95 min par semaine.

**Relevé de la PR T13** (30/10, exécution verte 41011335457, sans cache, 2 crates) : `changes` 7 s, `pr-title` 3 s, `security` 42 s, `rust-linux` 41 s, `rust-windows` 78 s (chemin sensible, donc complet), `ci-ok` 3 s ; durée totale 86 s. Facturé : 5 min Linux + 2 min Windows × 2 = **9 min**. Le premier passage, en échec, en a coûté environ 7 : **≈ 16 min** pour la PR. L'API `timing` ne rend plus les minutes facturées (valeurs nulles) : le relevé se fait à partir des durées des travaux, arrondies à la minute supérieure, ou sur la page de facturation. Les estimations ci-dessus seront revues à la hausse avec la taille du code.

**Demandé à `xtask` (T2b, chemin sensible)** : permettre une interpolation dans `jobs.<id>.outputs.*` et dans `concurrency.group`, évaluées par GitHub et jamais par un shell. `changes` transmettrait alors ses zones par `needs`, `rust-windows` serait sauté sans démarrer (2 min par PR de moins) et `concurrency` annulerait les exécutions dépassées.
