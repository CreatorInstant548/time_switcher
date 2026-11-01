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
| `rust-linux` | zone `rust` ou `ci` | `cargo xtask check --all --only fmt,clippy,test,determinism,unsafe,profiles,features,clippy-exemptions,local-paths,spec-map` |
| `rust-windows` | push sur `main`, étiquette `ci:windows`, ou zone `sensitive` | même chose, sans `fmt` |
| `ci-ok` | toujours (`always()`) | lit `needs.<travail>.result` (G2) : `changes`, `security` et, sur une PR, `pr-title` doivent valoir `success` ; `rust-linux` et `rust-windows` peuvent valoir `skipped`. Sur un brouillon, il **échoue** au lieu d'être sauté, car GitHub tient un contrôle sauté pour réussi. Délai : 10 min pour le travail, 2 min pour son étape (voir plus bas) |

**Déterminisme en CI** (Y5 du §9.15 du carnet d'archi-sécu, T9c) :
- **Version courte** : l'étape `determinism` de `xtask check` est dans les listes de `rust-linux` et de `rust-windows`. Elle lance les tests de `ts_toy` dans deux processus, à 1 et à 4 fils, et compare leurs résultats entre eux puis aux valeurs dorées de `data/golden/ts_toy.json`. Sa présence dans `rust-linux` est **obligatoire** : comparer les résultats de Linux aux valeurs dorées, bénies en local sous Windows, est la seule preuve entre plateformes (relecture de la #64), dans la limite décrite plus bas. `ci-ok` accepte toujours `rust-linux` sauté : l'étape ne tourne que si la PR touche les zones `rust` ou `ci`.
- **Quand elle tourne** : avec `--all`, l'étape ne filtre pas les fichiers touchés (`touches_toy` ne sert qu'en local, sans `--all`). C'est donc la condition du travail qui décide : zone `rust` ou `ci` pour `rust-linux`. La zone `rust` couvre `crates/` (`ts_toy`, `ts_kernel`, `ts_persist` et toutes les autres dépendances de `ts_toy`), `tools/xtask/` (le vérificateur) et, depuis T9c, `data/golden/` : une PR qui ne change qu'une valeur dorée relance donc la comparaison. Toute PR de la zone `rust` paie la version courte (décidé : 10 s mesurées, et une dépendance de `ts_toy` touchée sans le savoir ne passe pas au travers).
- **An complet** : chaque semaine, travail `determinism` de `nightly.yml` (§4).
- **Valeurs dorées, chemin sensible** (décidé par archi-sécu, T9c) : `data/golden/` est dans `tools/ci/sensitive-paths.txt`. Changer une valeur dorée change la référence de reproductibilité : la PR passe par `/security-review`, l'avis `archsec-ok` et `rust-windows` (zone `sensitive`), en plus de `rust-linux` (zone `rust`).
- **Limite de la comparaison entre plateformes** : la règle R1 de l'architecture ([revue](../conception/revues/architecture.md) §11, D19 ; attendu M1) ne promet la reproductibilité que **sur un même binaire et une même plateforme**. Comparer les résultats de Linux et de Windows aux mêmes valeurs dorées ne tient aujourd'hui que parce que l'état du monde-jouet ne contient **aucun flottant** (champ `manifest.floats` de `data/golden/ts_toy.json`). Le jour où un état doré en contiendra, il faudra des valeurs dorées **par plateforme**, et un écart entre Linux et Windows ne sera plus une erreur en soi.

**Délai de `ci-ok`** (*issue* #61, décidé) : d'après l'incident observé, le `timeout-minutes` d'un travail a aussi compté l'attente d'un exécuteur. Le 30/10, lors d'un incident de GitHub Actions, `ci-ok` a attendu 3 min 25 s et a été annulé à 2 min alors que tout était vert. Le travail a donc 10 min, et son unique étape garde un `timeout-minutes: 2`, qui borne le script lui-même. Coût nul en temps normal : le travail ne fait que lire des résultats (3 s), et un travail n'est facturé que pendant qu'il tourne.

**Zones** (`changes.sh`), calculées sur `git diff <base commune> <tête>` ; une base absente ou inconnue retient toutes les zones :
- `rust` : `crates/`, `tools/xtask/`, `fuzz/`, `.cargo/`, `data/golden/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`, `deny.toml` ;
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
- **cargo-fuzz** : `cargo install --locked --version` avec la *nightly* datée `FUZZ_TOOLCHAIN` de `nightly.yml`. Version 0.13.2 (*issue* #75 ; publiée le 09/06/2026, antérieure à `nightly-2026-10-01`). Pas d'empreinte dans `tools.lock`, comme `cargo-audit` : cargo vérifie celle de crates.io.
- **Contrôle des épingles** (étape `workflows` de `xtask check`, *issue* #75) : chaque appel de `install-tools.sh <outil>` ou `install-tools.sh version <outil>` dans un workflow doit nommer une sous-commande connue du script, et chaque clé que sa fonction lit (`lock_value '<clé>'`) doit être présente et non vide dans `tools.lock`, lue comme le script la lit. Les fonctions appelées par cette fonction comptent aussi. Le contrôle refuse ce qu'il ne sait pas lire : une sous-commande dont la fonction n'a pas la forme `nom() {` en colonne 0, un appel de `lock_value` hors d'une telle fonction (une fonction d'une ligne, par exemple), ou une clé qui n'est pas un littéral entre apostrophes. Seul un appel du script est lu : son nom en premier mot, ou après `bash`, `sh`, une option, `run:` ou un séparateur du shell ; un filtre de chemins ou un `echo` ne compte pas. Il ne vérifie que la présence des clés : les autres refus du script à l'exécution (versions Linux et Windows différentes, `FUZZ_TOOLCHAIN` absent) restent à sa charge. Sans ce contrôle, `fuzz/` est arrivé sans `cargo-fuzz.version`, et le fuzzing du dimanche a échoué à l'installation au lieu d'être refusé dans la PR.

**Actions** (liste fermée, épinglées par SHA de 40 caractères, version en commentaire) :

| Action | Version | SHA |
|---|---|---|
| `actions/checkout` | v7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| `actions/cache` (`restore`, `save`) | v6.1.0 | `55cc8345863c7cc4c66a329aec7e433d2d1c52a9` |
| `Swatinem/rust-cache` | v2.9.2 | `6323deb102c322ba6fcbdcafc7e3dddab59af2b6` |
| `actions/upload-artifact` | v7.0.2 | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` |

`Swatinem/rust-cache` reste en v2.9.2 : la v2.9.3 avait moins de 14 jours (même règle que le `cooldown` de Dependabot).

## 4. Tâche périodique (`nightly.yml`)

- **Chaque nuit** à 03:17 UTC :
  - `advisories` : `cargo audit`, puis `cargo deny --locked check advisories` ;
  - `verify-main` (V3 du §9.14 du carnet d'archi-sécu, T3h ; AS-42, T3j) : `cargo xtask verify-main --dry-run --origin 7d`, soit les approbations de **chaque** commit sensible de `main`, relues sur GitHub, et le contrôle d'origine des commits des 7 derniers jours. Le dimanche et par `workflow_dispatch` : `--origin all`, sur tout l'historique. `--no-cache`, sans effet depuis T3e-2, est retiré. Détails ci-dessous.

**Origine des commits (AS-42, AS-44, R-ARCH-05)** : le *ruleset* `main` impose la PR, mais `-Check` n'atteste que l'état présent. Un script lancé avec le jeton d'administration pourrait désactiver la règle, pousser un commit, puis la rétablir. Avec `--origin`, chaque commit non sensible de la fenêtre doit être le squash d'une PR fusionnée dans `main` (J3) et, en `7d`, avoir `ci-ok` vert sur la tête de cette PR (G3, AS-44 AA2) :
- **J3** : une requête, `commits/<sha>/pulls`. Il faut exactement une PR dont `merge_commit_sha` est le commit, avec `merged_at` renseigné, la base `main`, et une tête **différente** du commit (AS-44 AA1). Sans cette dernière condition, la tête d'une PR poussée telle quelle sur `main`, règle désactivée, passerait, puisque GitHub marque alors la PR comme fusionnée par ce commit ;
- **G3, en `7d` seulement** : la même fonction que pour un commit sensible (`Repo::ci_ok`), soit environ 2 lectures de plus par commit de la semaine. Elle ferme le cas d'un squash fait avec le jeton d'administration, règle désactivée, sans CI. Le passage `all` reste en J3 seul, à cause du quota (*issue* #55) ;
- **fenêtre `7d`** : du sommet de `main` jusqu'au premier commit daté (date du *committer*) d'avant la limite **qui passe J3 et G3**, inclus (AS-44 AA3). La date d'un commit est choisie par celui qui le pousse : des commits antidatés, poussés sans PR ou fusionnés sans `ci-ok`, échouent et ne ferment pas la fenêtre. La lecture continue à travers eux jusqu'à un vrai squash avec sa CI. Le passage `all` du dimanche couvre le reste ;
- un commit en échec est une **anomalie** : le travail échoue, et `report` ouvre `nightly-failure` ;
- les commits sensibles gardent leur contrôle complet (V5), qui comprend déjà J3 et G3.

**`rule-suites` non lu (décidé)** : `GET repos/<dépôt>/rulesets/rule-suites` demande la permission *Administration* en lecture (documentation GitHub des permissions des jetons), que le jeton d'un travail GitHub Actions ne peut pas recevoir. Les contournements (`bypass`) ne sont donc pas lus en CI. Avec G3 sur la fenêtre `7d`, le contrôle d'origine ci-dessus rend visible **sous 24 heures** un commit poussé sans PR, ou fusionné sans `ci-ok`, antidaté ou non : seul un squash conforme ferme la fenêtre. **Exception, une panne d'Actions** (facturation ou incident) : la tâche de nuit ne tourne pas, et la détection attend sa reprise. Pendant ce temps, aucune fusion n'est possible faute de `ci-ok`, et l'orchestrateur lance chaque jour `cargo xtask verify-main --dry-run --origin 7d` dans la copie principale (AS-44 AA4).

**Chemins sensibles (V5, AS-39 ; AS-31 E3)** : pour tout commit de `main` qui touche un chemin sensible, calculé depuis `sensitive-paths.txt` d'`origin/main`, `verify-main` relit sur GitHub, à chaque exécution, `merge` compris :
- `pulls/<n>`, soit la tête, `merged` et `merge_commit_sha` ;
- G3 sur la tête ;
- `review-ok` et `archsec-ok` sur la tête ;

puis il fait J4 avec ces faits frais. Sans `--origin` (`cargo xtask merge`), un commit non sensible ne lit rien sur GitHub depuis T3e-2 : le *ruleset* `main` impose la PR et `ci-ok`. La tâche de nuit lui ajoute le contrôle d'origine ci-dessus.

**Travail `verify-main`** :
- **Droits** : `actions`, `checks`, `contents`, `issues` et `pull-requests` en lecture, rien en écriture. `GH_TOKEN` vaut le jeton du travail.
- **Pas de `git fetch`** (décision d'archi-sécu du 30/10, m2 de la relecture de la #52). Le checkout garde `persist-credentials: false` : sur un dépôt privé, un `git fetch` n'aurait donc aucun identifiant. En mode CI (`CI=true` et `GITHUB_ACTIONS=true`), `verify-main` saute le fetch et lit les références du checkout `fetch-depth: 0`, qui apporte `origin/main` et `origin/pilotage` (J4). Il **refuse** si l'une des deux manque. Aucun identifiant Git dans le travail, pas de `gh auth setup-git`.
- **`TS_CACHE`** vaut `${{ runner.temp }}/ts-cache`, défini sur l'étape : le contexte `runner` n'est pas disponible dans `jobs.<id>.env`. Sans lui, le chemin par défaut `C:/Dev/ts-cache` serait relatif sous Linux.
- **`--dry-run`** (décidé) : le jeton, en lecture seule, ne peut pas ouvrir d'*issue*. Une anomalie affiche l'*issue* qui serait ouverte, et le travail échoue : `--dry-run` compris, une anomalie donne un code de sortie non nul (test `anomaly_fails_the_nightly_run`). `report` ouvre alors `nightly-failure`, avec le lien de l'exécution. L'*issue* `role:architecture` (J5) est ouverte par le prochain `cargo xtask merge` ou `verify-main` lancé en local, qui refait le même contrôle. `--dry-run` ne change rien d'autre ici : en CI, le contrôle des réglages est sauté de toute façon.
- **`repo-settings.ps1 -Check` est sauté en CI** avec un avertissement visible dans le journal et dans le résumé de l'exécution, comme depuis la #42 (`GITHUB_TOKEN` n'a pas les droits d'administration). Il tourne à chaque `cargo xtask merge`.
- **Vérification** : le travail ne se lance pas en local. À la première nuit après la fusion, ou par `workflow_dispatch`, le journal doit montrer, dans l'ordre :
  - `mode CI : git fetch sauté` ;
  - `--origin: J3 and G3 on every commit since Unix time <t> (AS-42, AS-44)`, ou `--origin all: J3 on every commit (AS-42)` ;
  - une ligne `conforme` par commit postérieur à la base (`non sensible, squash de la PR #<n> (AS-42)` dans la fenêtre, `non sensible, PR et ci-ok imposés par le ruleset` hors de la fenêtre, ou les faits relus d'un commit sensible) ;
  - l'avertissement sur `repo-settings.ps1 -Check` ;
  - `Résultat : OK`.
- **Le dimanche** (même heure), en plus, et seulement si `main` a changé depuis la dernière exécution hebdomadaire réussie (`main-changed.sh`, par l'API, avec `actions: read`) :
  - `windows` : `cargo xtask check --all` hors étapes de sécurité de `ci.yml` ;
  - `gitleaks-history` : gitleaks sur tout l'historique de toutes les branches, `--text`, avec son auto-test ;
  - `fuzz` : corpus initial de `decode` écrit par `fuzz/seeds` (`cargo run --locked`, qui refuse aussi un `fuzz/Cargo.lock` périmé, `cargo fuzz` n'ayant pas d'option `--locked`), puis 60 min réparties entre les cibles de `fuzz/`. **Sauté** tant que `fuzz/` n'existe pas. Les plantages (`fuzz/artifacts/`) sont téléversés, gardés 7 jours ;
  - `determinism` (Y5, T9c) : `cargo xtask determinism --full`, sous Linux. La version courte et l'an complet (365 jours simulés, instantané au jour 182) tournent à 1 et à 4 fils, dans des processus distincts, chacun comparé à ses propres valeurs dorées de `data/golden/ts_toy.json` (clés `short.*` et `year.*`). Durée de l'an complet : 75 s mesurées en local (Windows, debug, deux processus), plus la compilation de `ts_toy` et de ses dépendances ; la durée en CI sera relevée à la première exécution du dimanche (en CI, la version courte va environ 6 fois plus vite qu'en local (1 s contre 6,3 s), voir §7). Droits : `actions` et `contents` en lecture. Un échec rejoint `report`, qui ouvre `nightly-failure` ;
  - `windows` comprend aussi l'étape `determinism` (version courte), comme `rust-windows`.
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

Offre GitHub Pro (depuis le 30/10/2026) : **3 000 min par mois** (M36). Un travail est compté à la minute supérieure, et une minute Windows compte double. Le responsable développement relève la consommation chaque semaine ; **à 80 % (2 400 min)**, il passe en **mode minimal** : plus de `rust-windows` sur PR (étiquette retirée, chemins sensibles vérifiés sous Windows par la tâche hebdomadaire seulement), et `nightly.yml` réduit à `advisories` et `verify-main`. `verify-main` reste (décidé) : c'est le contrôle de nuit des approbations des commits sensibles de `main`.

**Coût estimé par exécution** (minutes facturées, à recaler sur les relevés) :

| Cas | `changes` | `pr-title` | `security` | `rust-linux` | `rust-windows` | `ci-ok` | Total |
|---|---|---|---|---|---|---|---|
| PR de documentation | 1 | 1 | 3 | 1 | 2 | 1 | ≈ 9 |
| PR Rust, cache chaud | 1 | 1 | 3 | 4 à 6 | 2 | 1 | ≈ 12 à 14 |
| PR sur un chemin sensible | 1 | 1 | 3 | 4 à 6 | 2 × (8 à 15) | 1 | ≈ 26 à 42 |

Tâche périodique : `advisories` ≈ 2 min par nuit (≈ 60 par mois) ; `verify-main` ≈ 2 à 4 min par nuit, compilation de `xtask`, lecture sur l'API des commits sensibles de l'historique et une requête par commit non sensible de la fenêtre, de tout l'historique le dimanche (≈ 60 à 120 par mois). Le 30/10, pour 40 commits, le passage `all` fait environ 200 lectures de l'API (estimation : 6 à 8 par commit sensible, une par commit non sensible) ; le jeton du travail en permet 1 000 par heure. L'objectif d'AS-39 (mécanisme incrémental avant 150 commits, alerte sous 20 % de quota) reste suivi à part ; le dimanche, si `main` a changé, Windows 2 × (8 à 15), gitleaks 1 à 2, fuzzing 70 à 76 (60 min de fuzzing, plus la *nightly*, `cargo-fuzz` et deux compilations sans cache, le corpus en debug et les cibles avec ASan ; en local, 2 min 20 s pour `cargo-fuzz` et 3 min 13 s pour le corpus ; durée en CI à relever à la première exécution, sous les 90 min du travail), déterminisme complet 3 à 6 (compilation comprise) : ≈ 90 à 115 min par semaine. En mode minimal, le travail `determinism` s'arrête avec les autres travaux du dimanche ; la version courte reste dans `rust-linux`.

**Version courte du déterminisme dans `rust-linux`**, relevé de la PR T9c (exécution 41124769723, cache chaud) : l'étape dure **10 s**, dont environ 9 s pour recompiler les tests de `ts_toy` avec `-p ts_toy` (les fonctionnalités unifiées diffèrent de celles de `--workspace` dans l'étape `test`) et 1 s pour les deux processus (0,2 s à 1 fil, 0,8 s à 4 fils). `rust-linux` a duré 1 min 46 s en tout. Coût négligeable dans le budget.

**Relevé de la PR T13** (30/10, exécution verte 41011335457, sans cache, 2 crates) : `changes` 7 s, `pr-title` 3 s, `security` 42 s, `rust-linux` 41 s, `rust-windows` 78 s (chemin sensible, donc complet), `ci-ok` 3 s ; durée totale 86 s. Facturé : 5 min Linux + 2 min Windows × 2 = **9 min**. Le premier passage, en échec, en a coûté environ 7 : **≈ 16 min** pour la PR. L'API `timing` ne rend plus les minutes facturées (valeurs nulles) : le relevé se fait à partir des durées des travaux, arrondies à la minute supérieure, ou sur la page de facturation. Les estimations ci-dessus seront revues à la hausse avec la taille du code.

**Demandé à `xtask` (T2b, chemin sensible)** : permettre une interpolation dans `jobs.<id>.outputs.*` et dans `concurrency.group`, évaluées par GitHub et jamais par un shell. `changes` transmettrait alors ses zones par `needs`, `rust-windows` serait sauté sans démarrer (2 min par PR de moins) et `concurrency` annulerait les exécutions dépassées.
