# Carnet du responsable architecture et sécurité

> Tenu par cette session seule (`local_83246e4f-58ef-4aba-85ca-86a56a5407e8`), lancée par l'orchestrateur le 30/10/2026. Règles : [organisation.md](../time_switcher/docs/pilotage/organisation.md) §1 et §3.
> Les numéros des exigences (`SEC0-nn`, `T0-nn`, `V0-nn`) ne sont **jamais renumérotés** : une exigence retirée garde son numéro avec la mention « supprimée ».
> Mis à jour le 30/10/2026.

## 1. Rôle et propriété

- Propriétaire depuis le 30/10 de [05-architecture.md](../time_switcher/docs/conception/05-architecture.md) et des revues techniques : [architecture](../time_switcher/docs/conception/revues/architecture.md), [sécurité](../time_switcher/docs/conception/revues/securite.md), [génération](../time_switcher/docs/conception/revues/generation.md), [développement par agents](../time_switcher/docs/conception/revues/developpement-agents.md). Le responsable conception garde 01 à 04, les domaines et le README ; une décision technique qui change le jeu passe par lui.
- Je ne compile pas et je n'écris pas de code : j'écris des décisions, je valide les découpages et les tâches qui touchent une frontière de crate, un format, le protocole ou un chemin sensible, et je donne l'avis final, bloquant, sur les fusions des chemins sensibles (§5).
- **Où vivent mes décisions pendant la phase 0 (décidé)** : dans ce carnet. Elles seront **reportées dans `securite.md` §7, `developpement-agents.md` et, si besoin, 05, par une seule PR de documentation à l'audit de fin de phase 0** (`claude/revues-fin-phase0`). *Pourquoi : en mode local, chaque fusion passe par le responsable développement ; une PR de documentation à chaque retouche des exigences ralentirait le code sans rien protéger, et ce carnet est lisible par toutes les sessions à un chemin fixe. Une décision qui change le jeu, elle, part tout de suite au responsable conception.*

## 2. État

| Tâche | État |
|---|---|
| 1. Exigences bloquantes de la phase 0 et modèle de menaces | **Faite** (§3, §4) |
| 2. Validation du découpage de la phase 0 | **Validé avec corrections** le 30/10 (§9.1) ; feu vert donné dans l'ordre du découpage |
| Fin du mode local | Exigences adaptées au dépôt GitHub (révision du §3, §3.7) |
| 3. Demande de la direction artistique (§12.9) | **Faite** (§8) ; transmise au responsable conception |
| 4. `cargo xtask spec-map` dans le découpage | **Spécifiée** (§7), à inscrire au découpage |

## 3. Exigences de sécurité bloquantes de la phase 0 (décidé)

Source : [securite.md](../time_switcher/docs/conception/revues/securite.md) §7 (phase 0, points 1 à 6), [developpement-agents.md](../time_switcher/docs/conception/revues/developpement-agents.md) §12.5 (point 7), l'en-tête de 05 (« Clôture de la conception », barrières locales) et [generation.md](../time_switcher/docs/conception/revues/generation.md) §12.

**Révision du 30/10, fin du mode local.** Le dépôt privé `CreatorInstant548/time_switcher` existe, en offre gratuite, sans protection de branche ni *ruleset*. Les exigences valent désormais pour ce dépôt réel :
- la CI GitHub Actions s'ajoute à `cargo xtask check` ;
- la fusion se fait côté GitHub par `cargo xtask merge <PR>` (`gh pr merge --squash --delete-branch`), si `ci-ok` est vert sur le dernier commit et la branche à jour ;
- Dependabot est actif ;
- les barrières de `main` restent locales.

Les exigences révisées portent la mention « révisé » ; aucune n'est renumérotée. Les exigences propres à la CI sont SEC0-29 à SEC0-34 (§3.7).

Chaque case est cochée par moi, sur preuve (sortie de commande notée dans le carnet du responsable développement, ou relecture). La phase 0 n'est pas terminée tant qu'une case reste vide.

### 3.1 Chaîne d'approvisionnement

- [ ] **SEC0-01** — `Cargo.lock` et `uv.lock` versionnés ; `--locked` sur toutes les commandes `cargo` de `xtask` ; `uv sync --locked` (S26).
- [ ] **SEC0-02** — `deny.toml` :
  - `advisories` : vulnérabilité et version retirée (*yanked*) refusées ; crate non maintenue en alerte, examinée à l'audit hebdomadaire (SEC0-04) ;
  - `sources` : crates.io seulement ; aucun dépôt git, aucun autre registre ;
  - `licenses` : liste blanche MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0, Unicode-DFS-2016, CC0-1.0, MPL-2.0, BSL-1.0 ; **GPL, LGPL et AGPL refusées partout** (décidé : le client contient le serveur et presque toutes les crates ; un copyleft fort dans le graphe imposerait d'ouvrir le jeu) ;
  - `bans` : crates réseau (`reqwest`, `ureq`, `hyper`, `tokio`, `quinn`, `openssl`, `native-tls`) refusées dans tout l'espace de travail en phase 0. Elles reviendront en phase 4, avec la liste des crates autorisées à les utiliser (`wrappers`).
- [ ] **SEC0-03** (révisé) — `cargo deny check` et `cargo audit` tournent :
  - dans `cargo xtask check`, en local ;
  - dans le travail `security` de la CI : `deny` si Rust ou la CI changent, `audit` si `Cargo.lock` change ;
  - dans la tâche périodique (SEC0-33).

  Un échec bloque. Si la base RustSec ne peut pas être mise à jour, le contrôle échoue, sauf si la copie locale a moins de 7 jours (décidé : jamais de réussite silencieuse sans base).
- [ ] **SEC0-04** (révisé) — Le suivi des dépendances passe par **Dependabot** :
  - alertes et correctifs de sécurité activés (fait par l'orchestrateur) ;
  - `dependabot.yml` pour `cargo` et `github-actions`, puis `uv`, `pnpm` et `nuget` à l'arrivée de ces écosystèmes ; mises à jour hebdomadaires et regroupées.

  Une PR Dependabot suit le même chemin qu'une autre : `ci-ok`, `/code-review`, fusion par `xtask merge`. Une PR Dependabot qui touche `.github/` (mise à jour d'action) est un chemin sensible : elle demande mon avis, et l'épinglage par empreinte doit rester en place.
- [ ] **SEC0-05** — Toute nouvelle dépendance directe est justifiée dans la demande de fusion : besoin, alternative écartée, licence, présence d'un `build.rs` ou d'une macro procédurale. *Pourquoi : ces deux mécanismes exécutent du code sur le poste à la compilation.* Liste **pré-approuvée** pour la phase 0 : `bevy_ecs` (sans `bevy_app`), `serde`, `ron`, `blake3`, `thiserror`, `smallvec`, `arrayvec`, `chrono`, `chrono-tz`, `arrow`, `parquet`, `pyo3`, `numpy`, `rerun` (`default-features = false`), `proptest`, `arbitrary`, `libfuzzer-sys`, `clap`, `xshell` ou équivalent pour `xtask`, et `chacha20` (RustCrypto, pour l'aléa à clé, AS-12). Une dépendance de **cryptographie, de décodage de format externe ou de réseau** hors de cette liste demande mon avis **avant** la fusion.
- [ ] **SEC0-06** — Outils tiers épinglés : `cargo install --locked --version <v>` pour `cargo-deny`, `cargo-audit`, `cargo-fuzz` et `maturin` ; `gitleaks` en binaire officiel, avec son SHA-256 consigné dans le dépôt (`tools/dev/tools.lock` ou équivalent) et vérifié avant usage.
- [ ] **SEC0-07** — Chaîne Rust épinglée par `rust-toolchain.toml` (1.99.1, A14).

### 3.2 Secrets et historique

- [ ] **SEC0-08** (révisé) — `gitleaks`, binaire épinglé par SHA-256 :
  - en `pre-commit`, sur l'index, **sur toutes les branches**, y compris `pilotage` ;
  - en `pre-push`, sur la plage poussée ;
  - dans `xtask check` ;
  - **en CI** : travail `security`, sur toute la plage de la PR ; sur tout l'historique lors de la première exécution, puis chaque semaine.

  La détection de secrets de GitHub n'est pas disponible en dépôt privé gratuit. Un secret vu en CI ou poussé est tenu pour **fuité et remplacé**.
- [ ] **SEC0-09** — `.gitignore` des secrets locaux : `.env*`, `*.pem`, `*.key`, `*.p12`, `*.pfx`, `id_*`, `secrets/` (S28).
- [ ] **SEC0-10** — Aucun secret n'existe en phase 0 et aucun n'est créé. L'installation de `sops` et `age` est **reportée à la phase 4** (décidé : sans secret à chiffrer, l'outil ne protégerait rien ; S28 reste satisfaite par SEC0-08 et SEC0-09).
- [ ] **SEC0-11** — Garde des gros fichiers, dans le `pre-commit` :
  - refus de tout fichier de plus de **5 Mo** ;
  - refus, hors LFS, des extensions d'assets, de modèles et d'artefacts : `.glb`, `.gltf`, `.png`, `.jpg`, `.ogg`, `.wav`, `.mp3`, `.otf`, `.ttf`, `.onnx`, `.gguf`, `.pt`, `.safetensors`, `.parquet`, `.sqlite`, `.db`, `.blend`, `.psd`, `.fbx`, `.zip`, `.7z` ;
  - seule exception : `**/tests/fixtures/**`, jusqu'à 256 Ko par fichier.

  *Pourquoi : l'historique Git est définitif ; un artefact commis par erreur coûte le quota LFS ou la taille du dépôt pour toujours, et les artefacts régénérables vont dans `TS_CACHE` (A6).*

### 3.3 Barrières de `main` en mode local

*Rappel : ces barrières protègent de l'erreur et de l'injection de consignes, pas d'un poste compromis (tout vit sous un seul compte Windows).*

- [ ] **SEC0-12** — Crochets Git (`pre-commit`, `pre-push`, `commit-msg`) installés par `cargo xtask hooks install` depuis une copie épinglée par empreinte **hors du dépôt** (`TS_CACHE\hooks\<sha256>\`), avec `core.hooksPath` qui pointe vers elle. `xtask merge` revérifie l'empreinte avant chaque fusion (clôture C4).
- [ ] **SEC0-13** (révisé) — **Aucun commit local sur `main`, sans exception.**
  - La fusion se fait côté GitHub (SEC0-15) ; la copie principale ne fait que `git pull --ff-only`.
  - Le `pre-commit` refuse tout commit sur `main`. Il n'y a ni variable d'environnement ni autorisation d'exception.
  - Le `pre-push` refuse toute poussée vers `main` et toute poussée forcée.

  *Pourquoi : depuis le dépôt distant, plus aucun chemin légitime ne commet sur `main` en local ; une exception par variable serait une porte de plus.* L'autorisation par arbre du mode local est abandonnée.
- [ ] **SEC0-14** — Crochet Claude Code dans les **réglages utilisateur** (`~/.claude/settings.json`), hors du dépôt (clôture C4). Il refuse :
  - `git commit` sur `main` ;
  - `git push` vers `main` (`HEAD:main`, `refs/heads/main`), une destination préfixée par `+`, et `--force` sous toutes ses formes ;
  - `--no-verify`, `-c core.hooksPath`, `git config core.hooksPath` ;
  - `git update-ref refs/heads/main`, `git branch -f main` ;
  - `gh api` sur `git/refs` ;
  - `Edit` et `Write` dans la copie principale (A19).

  Le worktree `pilotage` est un autre chemin et reste permis.

  **Révisé le 30/10 (AS-15)** : le crochet vit dans le `.claude/settings.json` **du projet**, versionné, et non dans les réglages utilisateur.
  - *Pourquoi :* l'orchestrateur ne modifie pas `~/.claude/settings.json`. C'est la configuration personnelle de l'utilisateur, qui vaut pour tous ses projets (organisation §6 : rien n'est touché hors du projet).
  - *Conséquence :* une session peut retirer le crochet dans son propre worktree, ce que la clôture C4 voulait empêcher. SEC0-35 compense.
- [ ] **SEC0-35** — **Intégrité de la configuration de Claude Code** (ajouté le 30/10) :
  1. Le crochet refuse `Edit` et `Write` sur `.claude/settings.json` et `.claude/settings.local.json`. Il refuse aussi toute commande shell qui écrit dans ces deux fichiers ou qui y mentionne `disableAllHooks`.
     - Seule exception : une branche `claude/outillage-*`, dont la PR est un chemin sensible (§5).
     - *Pourquoi :* `settings.local.json` peut contenir `disableAllHooks`. On se protège ainsi de l'erreur et de l'injection, pas d'une session décidée à contourner.
  2. `.claude/settings.local.json` est dans `.gitignore`. `xtask hooks verify` signale son existence dans une copie de travail.
  3. `xtask hooks verify`, lancé par `xtask merge` et par `origin-check`, parcourt **toutes les copies de travail** (`git worktree list`). Pour chacune, il vérifie que :
     - `.claude/settings.json` est identique à celui de la tête de sa branche ;
     - aucun `settings.local.json` ne contient `disableAllHooks` ni de `hooks` qui remplacent ceux du projet ;
     - `core.hooksPath` pointe vers la copie épinglée de `TS_CACHE\hooks`.

     Un écart **suspend les fusions** et ouvre une *issue* `role:architecture`.
  4. Sur `main`, l'empreinte de `.claude/settings.json` est comparée à la dernière valeur approuvée, inscrite par `xtask hooks install` dans `TS_CACHE\hooks\`. Un changement sans PR sensible approuvée (`archsec-ok`) est une anomalie de SEC0-17.
- [ ] **SEC0-15** (révisé) — `cargo xtask merge <PR>`, seul chemin de fusion, lancé depuis la copie principale. Dans cet ordre :
  1. la copie principale est propre et sur `main`, à jour par `--ff-only` ;
  2. empreinte des crochets installés (SEC0-12) ;
  3. contrôle de l'origine (SEC0-17) ;
  4. la PR n'est pas en brouillon ; la branche est à jour de `main` (`behind_by = 0`) ; **`ci-ok` est vert sur le dernier commit de la tête** (`headRefOid`) ;
  5. pour un chemin sensible (§5), la PR porte deux commentaires qui citent ce même SHA de tête :
     - `review-ok: <sha>`, écrit par la session ou le sous-agent relecteur, distinct de l'auteur, après `/security-review` ;
     - `archsec-ok: <sha>`, écrit par moi ;
  6. `gh pr merge <PR> --squash --delete-branch --match-head-commit <sha>`.

  *Pourquoi :*
  - *la CI vérifie l'arbre exact, et `--match-head-commit` interdit qu'un commit poussé après les contrôles soit fusionné ;*
  - *les approbations sont liées au SHA et restent tracées sur GitHub, ce qui remplace avantageusement un fichier local ;*
  - *`xtask merge` ne compile rien : le responsable développement ne compile pas (organisation §2).*

  L'attestation locale et le fichier `TS_CACHE\reviews\<branche>.ok` du mode local sont abandonnés.
- [ ] **SEC0-16** (révisé) — Le titre de PR devient le message du squash. Il est conventionnel (A31) et vérifié par le travail `pr-title` (SEC0-31). Le registre local des fusions est **abandonné** : GitHub garde la PR, ses contrôles et ses commentaires.
- [ ] **SEC0-17** (révisé) — **Contrôle de l'origine** : `cargo xtask origin-check`, épinglé par empreinte, lancé **hors de la CI**, depuis la copie principale (clôture C4).
  - Il vérifie que chaque commit de `main` postérieur à la base (les commits antérieurs à l'outillage, listés une fois pour toutes) est le squash d'une PR fusionnée, dont le dernier commit avait `ci-ok` vert, et, pour un chemin sensible, les deux commentaires de SEC0-15.
  - Il tourne au début de chaque `xtask merge`, et chaque jour de travail à la première session du responsable développement.
  - Une anomalie ouvre une *issue* `role:architecture` et suspend les fusions jusqu'à explication.
- [ ] **SEC0-18** (révisé) — **Test des barrières** (`cargo xtask test-barriers`), dans un dépôt jetable sous `TS_CACHE\tmp\`. Il vérifie que :
  - le crochet Claude Code refuse chaque variante de SEC0-14 ;
  - un commit sur `main` et une poussée vers `main` ou forcée sont refusés ;
  - l'empreinte des crochets installés est la bonne.

### 3.4 Fondations du code

- [ ] **SEC0-19** — `unsafe_code = "forbid"` dans `[workspace.lints.rust]`, et `#![forbid(unsafe_code)]` dans chaque crate (S26). Exceptions, en **liste fermée** :
  - `ts_client_native` (FFI, phase 3) ;
  - `ts_py`, seulement si les macros de `pyo3` l'exigent : alors `deny` au niveau de la crate et `allow` local, commenté `// SAFETY:`.

  Un script de `xtask check` vérifie la règle.
- [ ] **SEC0-20** — **Profils de construction** (S8, M28) :
  - les fonctionnalités `dev-commands`, `dev-introspection`, `external-pilot`, **`dev-bench`** (banc de pilotage et pause de décision des sessions joueuses, en-tête de 05, A8) et `public` sont déclarées dès la phase 0 dans `ts_kernel`, qui est dans tous les graphes ;
  - `compile_error!` si `public` est active avec l'une des quatre autres ;
  - **`public` n'est activée dans aucun `Cargo.toml`** : seulement en ligne de commande, par la future `xtask release` ;
  - les fonctionnalités de développement ne sont activées par aucune dépendance **normale** des crates d'hôte (`ts_host`, `ts_server`, `ts_client_native`) : seulement par des dépendances de développement ou par les crates d'outils (`ts_runner`, `ts_py`, tests) ; *resolver* 3 ;
  - `xtask check` vérifie que `cargo check -p ts_kernel --features public,<f>` **échoue** pour chacune des quatre fonctionnalités ;
  - `ts_kernel` expose une constante de profil de construction, qui servira au refus de démarrer et à la vérification de l'artefact.
- [ ] **SEC0-21** — **`PlayerId` opaque** dans `ts_model` (S12) :
  - champ privé de 128 bits (HMAC tronqué) ;
  - aucune conversion depuis ou vers un entier, une chaîne ou un identifiant de compte ;
  - `Debug` abrégé ;
  - un seul constructeur depuis des octets de pseudonyme ;
  - **aucun type d'identifiant de compte dans `ts_model` ni dans le cœur**.
- [ ] **SEC0-22** — **Aléa à clé** (P25, P26, clôture C3, M27) :
  - clé de monde de **128 bits** ;
  - flux en énumération fermée, dont `CityGen`, `CityServer`, `PopGen`, `PopHidden` et `Furnish` ;
  - **domaine `cosmetic/<usage>` d'un type distinct**, qu'aucune API ne permet d'utiliser comme clé ni comme parent d'un autre tirage ;
  - test de M27 : aucune valeur cosmétique n'est égale à une clé, à une graine de flux ou à un état de l'aléa ;
  - un constructeur de clé de test depuis un `u64` existe, nommé comme non sûr, et il est **interdit avec `public`** ;
  - **la fonction de tirage est un PRF cryptographique** (AS-12, décidé) :
    - la clé de chaque flux et la clé du domaine cosmétique sont **dérivées** de la clé du monde par BLAKE3 en mode `derive_key`, avec un contexte par flux ;
    - le tirage est **ChaCha8 en mode compteur** : le bloc est désigné par (sujet, créneau, usage), et ses 512 bits donnent 8 valeurs de 64 bits ;
    - **Philox et `wyhash` sont refusés** pour les flux à clé secrète.

    *Pourquoi : les graines en ligne sont secrètes (P25), mais le client voit beaucoup de sorties : résultats de combat, vocalisations, graines cosmétiques. Avec un mélange non cryptographique, retrouver la clé d'un flux à partir de ses sorties permettrait de **prédire** les tirages futurs ou la couche cachée. Avec un PRF et des clés dérivées, une clé de flux exposée ne dit rien des autres flux.* Le coût, de l'ordre de 50 ns par bloc, est à mesurer dans T5 ; une réserve de blocs par sujet et par créneau est permise si elle reste déterministe ;
  - `clippy.toml` interdit dans le noyau et les crates de règles : horloge murale (`SystemTime::now`, `Instant::now`), aléa du système et `rand::thread_rng`.
- [ ] **SEC0-23** — **Codec d'instantané borné**, dans une crate `ts_persist` créée en phase 0, réduite au codec (décidé, §6, V0-06). Il borne :
  - la taille totale, le nombre d'éléments, la profondeur et la longueur des chaînes ;
  - avec un en-tête de version.

  Il ne panique sur aucune entrée : `clippy::unwrap_used`, `expect_used`, `indexing_slicing` et `panic` sont refusés dans le module ; ses erreurs sont typées. Il produit des empreintes par morceau, avec un **arbre BLAKE3 et un manifeste** (format de parts adressées, P30, S23).
- [ ] **SEC0-24** (révisé) — **Cible de fuzzing** du décodeur d'instantané (`fuzz/`, hors de l'espace de travail). Elle tourne en **CI Linux**, avec une chaîne *nightly* datée et épinglée : 10 min à la tâche périodique, **60 min** chaque semaine (A33). Critère : **une exécution de 60 min sans plantage**, dont le lien est noté dans le carnet du responsable développement. Le corpus est sauvegardé comme artefact de la tâche (7 jours). Aucune *nightly* sur le poste : A14 est rétablie, et l'exception du mode local (AS-08) est retirée.
- [ ] **SEC0-25** — Chargement RON de `ts_data` borné (taille de fichier, profondeur), avec des erreurs typées et sans panique. *Pourquoi : en solo, le paquet est une entrée que le joueur peut modifier ; un crash est sans enjeu, mais le même chargeur sert au serveur.*
- [ ] **SEC0-26** — Aucun chemin absolu, nom d'utilisateur ou nom de machine dans les fichiers commis : empreintes dorées, manifestes, `WorldSpec`. Un contrôle dans `xtask check`.

### 3.5 Preuves et audit (critère de fin)

- [ ] **SEC0-27** (révisé) — **Preuves de blocage**, avec leurs liens et sorties notés dans le carnet du responsable développement :
  1. une **PR jetable** qui ajoute une version de crate frappée d'un avis RustSec a **`ci-ok` rouge**, et `xtask merge` la refuse ; elle est fermée sans fusion et sa branche supprimée ;
  2. un faux secret, au format reconnu par `gitleaks` et jamais une vraie clé, est refusé par le `pre-commit` local ;
  3. le travail `security` de la CI contient un **auto-test** : il crée un dépôt temporaire, y commet un faux secret généré à l'exécution, lance la même commande `gitleaks` que pour la PR et exige un échec. Une PR qui casse cette détection rend donc `ci-ok` rouge.

  *Pourquoi l'auto-test : le crochet `pre-push` empêche, à juste titre, de pousser un faux secret, et `--no-verify` est bloqué. Pousser un secret pour prouver la CI demanderait de défaire une barrière. L'auto-test prouve la même chose sans rien affaiblir.*
- [ ] **SEC0-28** — Audit de sécurité de fin de phase 0 : une session experte neuve, en lecture seule, lancée par moi, vérifie §3 et §6, ainsi que les exigences de la génération (§12, phase 0).

**Correspondance** avec securite.md §7, phase 0 :

| Point | Exigences |
|---|---|
| 1 | SEC0-01 à 04, 06, 08 |
| 2 | SEC0-19 |
| 3 | SEC0-20 |
| 4 | SEC0-21 |
| 5 | SEC0-23, 24 |
| 6 | SEC0-09, 10 |
| 7 (A26, A29) | SEC0-12 à 18, 29 à 34 |

### 3.6 Ce que l'offre gratuite ne couvre pas (risques acceptés)

*Remplace la section du mode local le 30/10.*
- **Aucune protection côté serveur** : pas de *ruleset* en dépôt privé gratuit. Le compte peut toujours pousser sur `main` par un autre chemin que Claude Code. Les barrières sont locales (SEC0-12 à 14), et le contrôle de l'origine (SEC0-17) **détecte** après coup, sans empêcher.
- **Aucune détection de secrets côté GitHub** : `gitleaks` seul (SEC0-08).
- **Un seul compte** : approbations, relectures et fusions portent la même identité GitHub. Les commentaires `review-ok` et `archsec-ok` tracent, mais ne prouvent rien. C'est la limite assumée de la clôture (C4).
- R-ARCH-01 (aucune copie hors du poste) est **levé** : le dépôt est sur GitHub.

### 3.7 Intégration continue GitHub Actions (ajouté le 30/10, décidé)

- [ ] **SEC0-29** — **Permissions minimales.**
  - `permissions: {}` au niveau du workflow, puis par travail le strict nécessaire : `contents: read` en général ; `issues: write` pour le seul travail qui ouvre l'*issue* d'échec de la tâche périodique ; `pull-requests: read` si `changes` lit la PR.
  - `actions/checkout` avec `persist-credentials: false` et `lfs: false`.
  - Aucun secret de dépôt en phase 0.
  - Interdits : déclencheurs `pull_request_target` et `workflow_run`, `secrets: inherit`, et `GITHUB_TOKEN` en écriture dans un travail qui exécute du code de la PR.
- [ ] **SEC0-30** — **Actions épinglées par SHA complet** de 40 caractères, avec la version en commentaire.
  - Liste fermée : `actions/checkout`, `actions/cache` ou `Swatinem/rust-cache`, `actions/upload-artifact`.
  - Les outils (`gitleaks`, `cargo-deny`, `cargo-audit`, `cargo-fuzz`) s'installent par une commande épinglée : version et SHA-256 du binaire, ou `cargo install --locked --version`. Aucune action tierce d'installation.
  - Un script du travail `security`, également lancé par `xtask check`, refuse toute référence `uses:` non épinglée ou hors liste.
  - Recommandé à l'orchestrateur : réglage du dépôt *Actions permissions* à *Allow select actions* avec la même liste, et refus que les Actions créent ou approuvent des PR.
- [ ] **SEC0-31** — **Aucune injection de script dans les workflows** : une donnée contrôlée par la PR (titre, corps, nom de branche, messages de commit) n'est **jamais** interpolée par `${{ }}` dans un `run:`. Elle passe par `env:` et une variable citée. C'est indispensable pour le travail `pr-title`. Le script de SEC0-30 refuse aussi `${{ github.event.` dans un `run:`.
- [ ] **SEC0-32** — **`ci-ok`** : un travail unique avec `needs` de tous les autres et `if: always()`. Il échoue si un travail a échoué ou a été annulé ; un travail sauté par filtre de chemins est accepté. `concurrency` annule l'exécution précédente de la même branche, et une PR en brouillon ne lance rien. Le travail `security` (gitleaks avec son auto-test, épinglage, `deny`, `audit`) tourne **à chaque PR, quels que soient les chemins**.
- [ ] **SEC0-33** — **Tâche périodique** (`nightly.yml`), sur `schedule`, seulement si `main` a changé, sauf l'audit :
  - `cargo audit` et `cargo deny check advisories` **chaque jour** : quelques minutes, car une alerte tombe sans changement de code ;
  - Windows complet, `gitleaks` sur tout l'historique et fuzzing de 60 min **chaque semaine** ;
  - en cas d'échec, une *issue* `nightly-failure` est ouverte ou mise à jour.

  Budget : ≤ 2 000 min par mois (M36), Windows compté double. Le responsable développement relève les minutes chaque semaine ; à 80 %, il passe en mode minimal.
- [ ] **SEC0-34** — **Modèle de PR** (A32, organisation §3.1 point 5) : sections Quoi, Pourquoi, Vérifié par, Nouvelles dépendances, Sécurité (chemins sensibles touchés, relectures faites) ; les deux questions d'écart à la conception. **`CODEOWNERS` est sans effet** sans protection de branche : on ne l'ajoute pas (décidé).

## 4. Modèle de menaces de la phase 0 (court)

Le modèle de menaces du jeu est celui de [securite.md](../time_switcher/docs/conception/revues/securite.md) §2 et §3. Il reste valable et n'est pas répété ici. La phase 0 ne livre rien aux joueurs : la surface attaquable est **la chaîne de développement**, et l'enjeu est de **graver dans les fondations** ce qui ne se rattrape pas.

**Biens**

| # | Bien | Pourquoi |
|---|---|---|
| B1 | Intégrité de `main` | Ce code finira chez tous les joueurs et sur les serveurs (RS06) |
| B2 | Le poste de développement | Seule machine : sessions, `gh` connecté, futur matériel de signature |
| B3 | Les propriétés gravées dans les fondations | `PlayerId`, profils de construction, aléa à clé, codec borné : coûteuses à reprendre après la phase 1 |
| B4 | L'historique Git | Définitif : un secret ou un gros fichier y reste |
| B5 | La confidentialité de la conception | Elle décrit des mécanismes à taire ; dépôt privé |

**Acteurs**

| Acteur | Probabilité |
|---|---|
| **Erreur d'agent** : commit sur `main`, contrôle sauté, crochet retiré, secret collé, artefact commis, dépendance mal choisie | Certaine ; la menace principale |
| **Injection de consignes** par un contenu lu : README de crate, page web, sortie d'outil, fichier de données, message | Moyenne |
| **Dépendance piégée** (typosquat, mainteneur compromis) qui exécute du code par `build.rs` ou une macro procédurale | Faible, impact maximal |
| **Futur joueur curieux ou tricheur** qui lira tout ce qui est livré (S1) | Certaine, plus tard ; elle dicte les fondations |

**Menaces et parades**

| # | Menace | Parade |
|---|---|---|
| T0-01 | Un agent écrit sur `main` sans contrôle (commit direct, `--no-verify`, crochet désinstallé) | SEC0-12 à 18 |
| T0-02 | Une fusion porte un arbre non vérifié, ou vérifié avant un rebasage | SEC0-15 (attestation par arbre exact) |
| T0-03 | Un secret ou un gros artefact entre dans l'historique | SEC0-08 à 11 |
| T0-04 | Une dépendance vulnérable ou piégée entre dans le graphe | SEC0-01 à 06 ; graphe réseau vide en phase 0 |
| T0-05 | Une injection pousse un agent à modifier une barrière, un crochet ou `deny.toml` | Chemins sensibles (§5) : relecture distincte et mon avis ; crochet Claude Code hors du dépôt ; la trace fait foi, pas un message |
| T0-06 | Une fonctionnalité de développement fuit dans une construction publique (unification Cargo) | SEC0-20 |
| T0-07 | Une fondation fuit plus tard un secret : identifiant de compte dans le journal, graine cosmétique parente d'un tirage caché, décodeur qui plante | SEC0-21 à 23, 25 |
| T0-08 | Les contrôles réussissent en silence (base RustSec absente, contrôle sauté, outil non épinglé) | SEC0-03, 06 ; attestations ; preuves SEC0-27 |

Menaces ajoutées le 30/10 avec le dépôt GitHub :

| # | Menace | Parade |
|---|---|---|
| T0-09 | Injection de script dans un workflow par le titre ou la branche d'une PR | SEC0-31 |
| T0-10 | Action tierce compromise, ou étiquette d'action déplacée | SEC0-30 ; permissions minimales SEC0-29 |
| T0-11 | Poussée directe sur `main` hors de Claude Code (aucun *ruleset*) | Détectée par SEC0-17, sans être empêchée (§3.6) |
| T0-12 | Une session désactive le crochet Claude Code de son worktree (crochet versionné dans le projet, AS-15) | `.claude/` est un chemin sensible ; crochets Git `pre-commit` et `pre-push` épinglés hors du dépôt ; contrôle d'intégrité SEC0-35 à chaque fusion ; SEC0-17. **Risque résiduel accepté** (R-ARCH-04) : l'écart n'est détecté qu'à la fusion suivante |

**Risque résiduel accepté** : tout vit sur un seul compte Windows et un seul compte GitHub. Jetons, crochets et commentaires d'approbation peuvent être forgés par qui contrôle le poste. Les barrières protègent de l'**erreur** et de l'**injection**, pas de la compromission de la machine (même limite que l'en-tête de 05, « Console et sessions »).

## 5. Chemins sensibles et validations

**Chemins sensibles** : union de `CLAUDE.md` et de la clôture C4, plus deux ajouts décidés ici.

- **Crates** :
  - `ts_net`, `ts_pilot`, `ts_persist`, `ts_client_native`, `ts_accounts`, `ts_admin` ;
  - **ajouts décidés : `ts_protocol` et `ts_view`**, à leur création. *Pourquoi : décodage d'octets non fiables pour l'un, fuite d'information pour l'autre (RS03, RS07, RS16).*
- **Code dans `ts_kernel`** : le module d'aléa à clé et la déclaration des fonctionnalités et du profil. *Pourquoi : SEC0-20 et 22.*
- **Configuration et outillage** :
  - `.github/`, `.claude/`, `.cargo/` ;
  - `tools/xtask/`, `tools/hooks/` (ou l'emplacement retenu pour les crochets), `tools/dev/` ;
  - `deny.toml`, `.gitleaks.toml`, `rust-toolchain.toml`, `fuzz/` ;
  - les sections `[features]` et `[profile.*]` des `Cargo.toml` ;
  - `Cargo.lock` quand il ajoute une crate.

Pour ces chemins :
- `/security-review` ;
- une relecture par une session ou un sous-agent neuf, distinct de l'auteur ;
- **mon avis favorable** sur le SHA de tête de la PR : un commentaire `archsec-ok: <sha>` sur la PR (SEC0-15), et une ligne au §9.

**Validation préalable** (avant le lancement de la tâche) : toute tâche qui crée ou change une frontière de crate, un format (instantané, artefact, paquet, journal), le protocole, une fonctionnalité de construction ou un chemin sensible. Le responsable développement m'envoie sa consigne. Je réponds par « validé », « validé avec corrections » ou « refusé », noté au §9.

## 6. Grille de validation du découpage de la phase 0

Je confronterai le découpage du responsable développement à cette grille. Chaque ligne doit être couverte par une tâche nommée, dans un ordre compatible.

| # | Attendu | Source | Chemin sensible |
|---|---|---|---|
| V0-01 | **Outillage des agents** : crochet `SessionStart` et emplacements, `TS_CACHE`, vérifications bloquantes du §6.3 de la revue (suppression d'un worktree avec jonction, chemins longs), repli `CARGO_TARGET_DIR` | dev-agents §6, §10.8 | Oui (`.claude/`, `tools/dev/`) |
| V0-02 | **Barrières locales** : crochets épinglés et installés par `xtask`, crochet Claude Code dans les réglages utilisateur, `xtask merge` (attestations, autorisation par arbre), registre chaîné, `verify-main`, `test-barriers` | SEC0-12 à 18 | Oui |
| V0-03 | **Espace de travail** : `Cargo.toml` (membres, `default-members`, `exclude`, profils A12, `[workspace.lints]`, *resolver* 3), `rust-toolchain.toml`, `.cargo/config.toml`, `deny.toml`, `clippy.toml` (interdictions de SEC0-22 et de l'architecture §9.2), `rustfmt.toml`, `.gitignore` (secrets, SEC0-09), `.gitattributes` (A4) | dev-agents §10.9 ; SEC0-01, 02, 07, 09 | Oui |
| V0-04 | **`xtask check`** : formatage, clippy, tests des crates touchées, déterminisme si le noyau change, `deny`, `audit`, `gitleaks`, `forbid(unsafe_code)`, incompatibilité des profils, absence de chemins locaux, attestation | SEC0-03, 08, 19, 20, 26 | Oui |
| V0-05 | **`ts_kernel`**, seul d'abord : `SimTime`, `StableId` (avec l'**espace de noms réservé à la carte**, P28), `Money`, aléa à clé de 128 bits et domaine cosmétique, ordonnanceur à roues, phases, greffons, `SimBuilder`, empreinte d'état, trait d'instantané, fonctionnalités et profil | architecture §22.2 ; SEC0-20, 22 ; génération §12.1, 12.2 | En partie (aléa, profils) |
| V0-06 | **`ts_persist`**, réduit au codec borné et au format de parts BLAKE3 avec manifeste ; cible de fuzzing | SEC0-23, 24 ; génération §12.4 | Oui |
| V0-07 | **PR « contrat » `ts_model`**, puis `ts_data` minimal : `PlayerId` opaque, chargement RON borné, paquet avec empreinte calculée sur le paquet compilé (A4) | architecture §22.3 ; SEC0-21, 25 | Non (fichier partagé : PR dédiée) |
| V0-08 | **`ts_world_model` minimal** : types géométriques en centimètres entiers | génération §12.3 (P13) | Non ; frontière de crate, à valider |
| V0-09 | **`ts_runner` minimal** : scénario sans affichage, `metrics.parquet` et manifeste dans `TS_CACHE` ; test de déterminisme (1 fil contre N fils) ; monde-jouet de 1 000 entités sur un an ; **générateur-jouet** (grille de parcelles et ménages) sur 1 et N fils ; aller-retour d'instantané | architecture §22.4 ; génération §12.5 ; M1, M2 | Non |
| V0-10 | `ts_py` minimal (lecture numpy), `ts_viz` minimal | architecture §22.5, §22.6 | Non (`ts_py` : exception `unsafe` à justifier, SEC0-19) |
| V0-11 | **`cargo xtask spec-map`** (§7) | organisation §3.1 point 3 | Non (`tools/xtask/` : avis requis) |
| V0-12 | **Preuves de blocage**, mesures du §10.12 de la revue, fuzzing d'une heure, audit hebdomadaire en place | SEC0-04, 24, 27 ; dev-agents §10.11, §10.12 | — |
| V0-13 | **Test de M27** (aucune valeur cosmétique égale à une clé) et **constante de profil** (M28) | attendus M27, M28 | — |
| V0-14 | **CI GitHub Actions** (ajouté le 30/10) : `ci.yml`, `nightly.yml`, `dependabot.yml`, `.gitleaks.toml`, modèle de PR, script de contrôle des workflows | SEC0-29 à 34 | Oui (`.github/`) |

*Révision du 30/10* : V0-02 se lit avec les SEC0-13 à 17 révisées (fusion côté GitHub, approbations en commentaires liés au SHA, plus d'attestation ni de registre local) ; dans V0-04, l'attestation est retirée ; dans V0-12, l'audit hebdomadaire est remplacé par Dependabot et la tâche périodique.

**Ordre attendu** :
1. V0-01, puis V0-02 et V0-03 : `xtask check` et `merge` doivent exister avant la première fusion de code ; V0-02 et V0-03 touchent toutes deux `tools/xtask/`, donc une seule session ou deux PR successives.
2. V0-05, seul.
3. En parallèle, au plus 3 sessions qui compilent : V0-07, V0-06 et V0-08.
4. V0-09 ; V0-10 et V0-11 en parallèle.
5. V0-12 et V0-13.

`spec-map` ne dépend d'aucune crate du jeu : il peut passer plus tôt, dès que `tools/xtask/` est stable.

## 7. Outil `cargo xtask spec-map` (tâche 4, spécification décidée)

Organisation §3.1 point 3. Construit en phase 0 par une session de tâche du responsable développement (modèle Sonnet), sous `tools/xtask/`. Chemin sensible : mon avis est requis avant la fusion.

**Entrées**
- **Renvois** dans le code, les tests et les données :
  - commentaires `// spec: <fichier> §<section>` (`#` dans les RON et en Python) ;
  - `<fichier>` est un nom relatif à `docs/conception/`, ou un nom court résolu de façon unique : `combat.md` → `domaines/combat.md`, `securite.md` → `revues/securite.md`, `05-architecture.md` ;
  - section : `§\d+(\.\d+)*`, éventuellement suivie de ` bis`, ` ter`, ou d'une lettre (`§3.10 b`) ;
  - **identifiants de décision** admis aussi : `S\d+`, `D\d+`, `P\d+`, `E\d+`, `G\d+`, `A\d+`, `M\d+` (`// spec: securite.md S8`, `// spec: attendus-mesurables.md M1`) ;
  - plusieurs renvois par commentaire, séparés par `;`.
- **Marques provisoires** : `// PROVISOIRE(conception #<id>)`.
- **Sections** : les titres Markdown de `docs/conception/**/*.md` (hors `docs/archives/`). Les identifiants de décision sont relevés dans les tableaux de décisions et dans les lignes des attendus.

**Sorties**
- `spec-map report` : code → sections, et sections → code ; **sections sans code**, filtrables par fichier et par phase ; liste des `PROVISOIRE` avec leur remontée. Sortie en Markdown sur la sortie standard, et `--json` pour les outils (tableau du responsable qualité, organisation §3.1 point 4).
- `spec-map check` (appelé par `xtask check`) : **échoue** sur un renvoi vers un fichier, une section ou un identifiant qui n'existe pas, sur un nom court ambigu, et sur un `PROVISOIRE` sans numéro.
- `spec-map diff <révision>` : pour les sections modifiées depuis une révision de la conception, liste le code qui les cite. C'est l'outil du responsable conception (« quand une section change, il voit le code touché »).

**Contraintes**
- Lecture seule, sans réseau ni dépendance nouvelle hors de la liste pré-approuvée (analyse des titres à la main, pas de bibliothèque Markdown).
- Déterministe : sortie triée.
- Tests sur un petit corpus de documents et de code factices.

## 8. Avis sur la demande de la direction artistique (§12.9) (tâche 3, décidé)

Source : [direction-artistique.md](../time_switcher/docs/conception/domaines/direction-artistique.md), v0.4, §12.9. Aucun de ces avis ne change le jeu.

### 8.1 Sécurité : apparence graduée, hérédité limitée et le reste du §12.9

**Avis : favorable, sans réserve bloquante.** Les mesures du domaine sont conformes à S1, S5, S6 et S7, ainsi qu'aux clôtures C2 et C3, et plus strictes que la revue de sécurité sur la graine de forme des signes distinctifs. Trois conditions de mise en œuvre, qui ne changent pas le jeu :

1. **Par le typage** (phase 3) : le composant génome d'apparence **n'implémente aucun encodage du protocole**. `ts_view` ne peut sérialiser qu'un type **apparence perçue**, construit par une fonction par précision. Un test de propriété vérifie que, pour chaque précision, aucun champ ne porte plus de valeurs distinctes que sa classe (tailles par 5 cm, crans, classe de voix grossière). *Pourquoi : la règle « chaque champ à la résolution de sa précision » est alors garantie par le compilateur et par un test, pas par l'attention de chaque tâche.*
2. **Tests de parenté et de réidentification** (M24, M25) : bloquants pour la phase 1, comme prévu. Je les ajoute à la grille de la phase 1. Le classificateur doit être **entraîné**, pas seulement une distance : un client modifié peut entraîner le sien sur la population solo publique.
3. **Visages vus de près** : la précision « visage » est un identifiant de fait, et un client modifié peut se constituer une base de visages. C'est voulu, puisqu'un visage vu se reconnaît. J'accepte ce risque résiduel, de la famille de RS40, avec le renvoi à l'équilibrage déjà noté pour l'écran « Une nouvelle vie » (clôture de la sécurité, tour 1).

Le reste du §12.9 n'appelle pas de condition nouvelle :
- **Aucun objet d'apparence propre aux joueurs**, et l'éditeur limité à l'espace de la population : conforme à S7.
- **Musique et ambiances seulement d'après le perçu** : conforme, avec les générateurs du client en fonctions pures (§8.3 du domaine), que le test du client vérifie.
- **Aucune métadonnée personnelle dans les assets** : contrôle 6 de `assets check`. J'ajoute que les fichiers texte des assets (RON, `.tscn`, `.tres`) passent aussi par `gitleaks` (SEC0-08) et par le contrôle des chemins locaux (SEC0-26).

### 8.2 Développement par agents : arborescence des assets et Git LFS

**Avis : favorable.** Décisions :
- **Arborescence** : l'arborescence du domaine (§6.8) est ajoutée à celle de la revue (§5.1) :
  - `client/catalog/` et `client/registry/` (Git ordinaire) ;
  - `client/assets_gen/` (ignoré, jonction vers `TS_CACHE\artifacts\assets`) ;
  - `client/scenes/style_reference/` ;
  - `tools/assets/`.
- **`tools/assets/`** appartient au **responsable contenus** dès la phase 3. C'est une exception à la règle « `tools/` est un fichier partagé », qui reste vraie pour le reste de `tools/`. Ce n'est pas un chemin sensible, sauf pour un script qui télécharge : empreinte épinglée et quarantaine dans `TS_CACHE\quarantine\` (clôture C9).
- **Motifs LFS** : `client/assets/**` pour `*.otf` et `*.ttf`, en plus des motifs de la revue (§5.3).
  - Ils restent **commentés jusqu'à la phase 3**.
  - La **première tâche de la phase 3 qui ajoute un asset** les active **avant** tout commit d'asset. *Pourquoi : un binaire commis avant l'activation reste pour toujours hors de LFS.*
  - La garde de SEC0-11 refuse déjà ces extensions hors de LFS.
- **Nom de la commande** : `cargo xtask assets check` (décidé, confirmé). En mode local, elle tourne dans `xtask check` dès qu'un chemin `client/assets/`, `client/catalog/` ou `client/registry/` change.
- **Budgets** : 4 Go de LFS en 2.5D, 16 Mo par texture, 30 Mo par morceau de musique ; ce sont les valeurs du domaine. Le stockage gratuit hors de Git pour la 3D (fichiers attachés aux versions) suppose le dépôt distant. Le responsable exploitation le vérifiera.

### 8.3 Architecture : synthèse vocale locale par `tract` (étude de la phase 5)

**Avis : faisable sous conditions, sans changement d'architecture aujourd'hui.** Conditions pour l'étude de la phase 5 :

1. **Crate dédiée `ts_tts`**, côté client seulement, appelée par `ts_client_native` sur un fil de fond. Jamais liée à `ts_game` ni au serveur, puisque la voix est cosmétique. `tract` garde un seul moteur d'inférence en Rust pur, celui des compétences, sans bibliothèque native supplémentaire.
2. **Modèle** de type VITS ou Piper exporté en ONNX. L'étude commence par un **essai de couverture des opérateurs de `tract`**, avec une parité contre `onnxruntime` hors ligne : c'est le principal risque technique. Budget : facteur temps réel ≤ 0,2 sur 4 cœurs, sans toucher au budget d'image **[à mesurer]**.
3. **Phonémisation sans GPL** : `espeak-ng`, utilisé par Piper, est sous GPL-3 et donc **refusé** (SEC0-02). Il faut un G2P neuronal ou un dictionnaire sous licence permissive, pour le français d'abord.
4. **Licences** : modèles et voix sous licence commerciale, consentement des voix documenté, aucun clonage (§7.5 du domaine) ; inscrits au registre des licences.
5. **Sécurité** : modèle livré dans le paquet signé, chargé seulement depuis le manifeste (S27) ; synthèse paramétrée **seulement par ce que le client a reçu**, donc la classe de voix grossière tant que l'orateur n'est pas identifié. Le délai de synthèse est le même pour toutes les paroles, puisque la synthèse se fait chez l'auditeur.

### 8.4 Exploitation : déclaration des contenus produits par IA

**Avis technique, pour le responsable exploitation, qui vérifie la lecture juridique avant l'ouverture :**

1. **La mention est globale, jamais par réplique ni par personnage** (décidé, bloquant). Elle passe par l'écran d'information, les crédits et les fiches des plateformes. *Pourquoi : une étiquette « formulé par IA » sur une réplique désignerait par contraste les répliques des joueurs (S7, M20).*
   - Toutes les paroles passent déjà par le même service et le même type de message, et celles des joueurs sont traduites par le même service. Une mention globale reste donc **vraie pour tous**.
   - Il en ira de même d'un éventuel marquage des voix synthétisées en phase 5 : toutes les paroles, ou aucune.
2. La liste des contenus produits par IA est **produite par un outil** à partir du registre (`kind: ai_generated`, champ `ai`), par exemple `cargo xtask assets credits --ai`. Elle n'est jamais tenue à la main.
3. **Le registre couvre aussi les modèles** livrés ou téléchargés (ONNX des compétences, GGUF du dialogue local, futurs modèles de voix), avec leur licence et ses obligations de mention. Ils sont listés dans `data/models/gguf.ron` (A8) avec un champ de licence.
4. Les invites conservées au registre ne contiennent **aucune donnée personnelle**. Le registre n'est pas livré : seuls les crédits générés le sont.

## 9. Avis donnés (tâches et fusions)

| Date | Objet | Arbre ou consigne | Avis |
|---|---|---|---|
| 30/10 | Découpage de la phase 0, T0 à T13 ([#2](https://github.com/CreatorInstant548/time_switcher/issues/2)) | `developpement.md`, commits 139ff40 et 8506f54 (branche `pilotage`) | **Validé avec corrections** (§9.1) |
| 30/10 | Découpage v2 (T0 à T13, avec T2a, T2b, T7a, T7b et T9w) | `developpement.md`, commit 1468629 | **Validé avec corrections R1 à R5** (§9.2) ; **feu vert pour T1** |
| 30/10 | Crochet Claude Code dans les réglages du projet (T1, T3) | Message du responsable développement | **Favorable**, avec SEC0-35 : point 1 dans T1, points 2 à 4 dans T3 (`hooks verify`) |
| 30/10 | PR #5 (T1) : décisions I4 et I2 du responsable développement | Message du responsable développement ; PR non encore soumise | **Favorable avec conditions** (§9.3) ; mon avis sur le SHA de tête reste à donner |

### 9.3 Décisions I4 et I2 de la PR #5 (30/10)

- **I4, script de fusion provisoire `tools/dev/merge-pr.ps1`** : favorable.
  - `guard-git` refuse `gh pr merge` et `gh api` en écriture ; c'est conforme à SEC0-14 et plus strict.
  - Conditions :
    1. `ci-ok` est **exigé**, et non « s'il existe », dès que `.github/workflows/ci.yml` est sur `main`. Avant cette date, la PR porte la sortie de `xtask check`, ou des commandes équivalentes, dans le carnet du responsable développement.
    2. La PR de T3 **supprime** le script : il n'y a jamais deux chemins de fusion.
    3. Le script est un chemin sensible, comme `tools/dev/`.
- **I2, crochet `reference-transaction` et refus de `revert`, `cherry-pick`, `rebase`, `am`, `reset` et des fusions non `--ff-only` sur `main`** : favorable. C'est une protection de `main` plus forte que la mienne, et elle couvre la variante `update-ref`. Conditions :
  1. le crochet refuse aussi la **suppression** de `refs/heads/main`, ainsi que toute mise à jour quand `refs/remotes/origin/main` est absente ;
  2. les cas sont ajoutés à `test-guards.ps1` et à SEC0-18.
- **Emplacements de compilation pris à la demande** (`session-start.ps1 -Acquire`) : sans enjeu de sécurité. C'est un écart à A9 de la revue du développement par agents, à reporter à la consolidation de fin de phase 0.
- **Après la deuxième relecture neuve** (30/10) :
  - `guard-git` en **liste blanche** des sous-commandes et options de `git` et de `gh`, avec refus par défaut : favorable. C'est plus robuste qu'une liste noire contre les variantes de syntaxe.
  - Lecture texte forcée pour gitleaks (contre `-diff`) et liste complète des fichiers dans `merge-pr.ps1` : favorable.
  - `reference-transaction` comparé à `ls-remote origin` : favorable, à condition qu'il **refuse si `ls-remote` échoue** (réseau absent), car un contrôle de sécurité doit échouer en position fermée.
  - **SEC0-09 révisée** : le motif `id_*` est restreint à `id_rsa*`, `id_dsa*`, `id_ecdsa*` et `id_ed25519*`. *Pourquoi : `id_*` ignorait en silence des sources légitimes comme `id_ranges.ron` ; `gitleaks` couvre de toute façon le contenu des clés.* À reporter à la consolidation.

### 9.2 Découpage v2 : écarts restants (30/10)

Le v2 reprend la grille et la plupart des corrections. Il a été écrit avant la révision du §3 pour GitHub ; cinq écarts restent, à corriger dans les consignes concernées, sans retarder T1 :

- **R1 (T2b, T3)**
  - **Ni attestation locale `TS_CACHE\attest\`, ni registre chaîné, ni pieds `Checked-Tree` ou `Merge-Request`** (SEC0-15 et 16 révisées). `xtask merge <PR>` vérifie :
    - `ci-ok` vert sur le SHA de tête ;
    - les commentaires `review-ok: <sha>` et `archsec-ok: <sha>` pour un chemin sensible ;
    - les crochets ;
    - l'origine des commits de `main`.

    Puis il fusionne par `gh pr merge --squash --delete-branch --match-head-commit <sha>`.
  - **D-DEV-03 n'est pas confirmée sous cette forme** : l'arbre est vérifié par la CI, et les approbations sont liées au SHA sur GitHub.
- **R2 (T12)** : **aucun faux secret poussé, aucun crochet contourné**. La preuve côté CI passe par l'auto-test du travail `security` (SEC0-27 révisée). Le point « commis sans crochet, refusé par la CI » est retiré.
- **R3 (T13)** : SEC0-29 à 34 en entier :
  - `permissions: {}` au niveau du workflow, puis le minimum par travail ;
  - `persist-credentials: false` et `lfs: false` ;
  - aucune interpolation `${{ github.event… }}` dans un `run:` ;
  - liste fermée d'actions épinglées par SHA, avec un script de contrôle ;
  - auto-test de `gitleaks` ;
  - `cargo audit` **chaque jour**, Windows, fuzzing et historique complet chaque semaine ;
  - pas de `CODEOWNERS`.
- **R4 (D-DEV-05)** : pas de *nightly* locale en repli (AS-08 retirée). Si la CI ne suffit pas, m'en saisir.
- **R5 (T5)** : ChaCha8 **en mode compteur**, avec clés de flux et clé cosmétique **dérivées par BLAKE3 `derive_key`** (SEC0-22, AS-12). Le coût par bloc est mesuré dans la PR.

### 9.1 Découpage de la phase 0 : avis détaillé (30/10)

Il est confronté à la grille du §6 (V0-01 à V0-14) et aux exigences du §3. **La couverture est complète**, et l'ordre est compatible. **Feu vert** donné pour lancer les tâches dans l'ordre du découpage, **à condition d'intégrer les corrections** ci-dessous dans les consignes. Les tâches marquées S demandent toujours mon avis avant leur fusion (SEC0-15).

**Réponses aux cinq questions**

1. **Frontières de crate**
   - **`ts_toy`** : validé. Conditions :
     - `publish = false` ;
     - jamais dépendance **normale** d'une crate de règles ni d'hôte : seulement de `ts_runner`, `ts_py`, `ts_viz` et des tests ;
     - jamais dans `standard_rules()` ;
     - un test de `xtask check` vérifie, par `cargo tree`, qu'aucune crate de règles ne dépend de `ts_toy` ;
     - l'agenda maison-travail porte `// spec: architecture.md §22` et n'est pas une règle de jeu, donc pas de marque `PROVISOIRE`.
   - **`ts_persist` dès la phase 0** : validé, c'est AS-05.
   - **`ts_world_model` minimal** : validé, mais **pas dans T7** (correction C7).
2. **Format des marques de `spec-map`** : il est fixé au §7, plus large que la proposition :
   - nom court de fichier résolu de façon unique ;
   - `§` avec `bis`, `ter` ou une lettre ;
   - identifiants de décision (`S8`, `M1`…) ;
   - `#` comme commentaire dans les RON et en Python ;
   - plusieurs renvois séparés par `;` ;
   - `check` en échec sur un renvoi pendant, un nom ambigu ou un `PROVISOIRE` sans numéro ;
   - `diff <révision>` pour le responsable conception.

   La sortie dans `TS_CACHE` est validée.
3. **Barrières** : corrigées par la fin du mode local (corrections C3a à C3e).
4. ***Nightly* locale** : **refusée**, devenue inutile avec la CI. Le découpage l'a déjà retirée. A14 est rétablie (SEC0-24).
5. **Critère « la CI bloque »** : il redevient littéral, c'est-à-dire `ci-ok` rouge, avec les preuves de SEC0-27 révisée.

**Corrections à intégrer dans les consignes**

- **C1 (T1)**
  - `pre-commit` :
    - `gitleaks` sur toutes les branches, `pilotage` comprise ;
    - **refus de tout commit sur `main`, sans variable d'exception** (SEC0-13 révisée) ;
    - garde des gros fichiers et des extensions hors LFS (SEC0-11).
  - `pre-push` : `gitleaks` sur la plage poussée ; refus de pousser vers `main` et de forcer.
  - `guard-git.ps1` couvre toute la liste de SEC0-14 : ajouter `git update-ref refs/heads/main`, `git branch -f main` et `git config core.hooksPath`.
  - `guard-main.ps1` dans le `.claude/settings.json` du projet : validé. C'est un garde-fou de commodité (A19), pas une barrière, puisqu'il est versionné ; la barrière est `guard-git.ps1` dans les réglages utilisateur.
  - Le `.gitignore` est vérifié et complété pour les secrets (SEC0-09).
- **C2 (T2)**
  - Ajouter la fonctionnalité **`dev-bench`** ; le test d'incompatibilité porte sur `public` + chacune des **quatre** fonctionnalités de développement.
  - Règles de SEC0-20 : `public` jamais dans un `Cargo.toml`, *resolver* 3 ; constante de profil.
  - `deny.toml` selon SEC0-02 : liste blanche de licences, copyleft fort refusé, crates réseau refusées.
  - `xtask check` ajoute :
    - le contrôle des chemins locaux (SEC0-26) ;
    - le script de contrôle des workflows (SEC0-30, SEC0-31) ;
    - `cargo deny` à chaque exécution ;
    - `cargo audit` si `Cargo.lock` change, ou si la base a plus de 7 jours (SEC0-03).
  - Outils épinglés (SEC0-06).
- **C3 (T3)**
  - **a.** `xtask merge <PR>` (et non `<branche>`) suit SEC0-15 révisée. Il **ne lance pas** `xtask check` : `ci-ok` vert sur le SHA de tête en tient lieu, et le responsable développement ne compile pas.
  - **b.** Fusion par `gh pr merge --squash --delete-branch --match-head-commit <sha>`.
  - **c.** Approbations par **commentaires de PR liés au SHA de tête** : `review-ok: <sha>` et `archsec-ok: <sha>`. Pas de fichier `TS_CACHE\reviews\<branche>.ok` : un fichier par branche ne se lie pas au contenu et ne laisse pas de trace partagée.
  - **d.** `origin-check` selon SEC0-17 révisée : il vérifie aussi les deux commentaires pour les chemins sensibles et ouvre une *issue* en cas d'anomalie.
  - **e.** Ajouter **`xtask test-barriers`** (SEC0-18).
- **C4 (T4)** : format du §7, dont `spec-map check` appelé par `xtask check`. Le contrôle de `tools/xtask/` reste sensible : mon avis est requis avant la fusion.
- **C5 (T5)**
  - **Aléa : ChaCha8 en mode compteur avec clés de flux dérivées par BLAKE3, pas Philox** (SEC0-22, AS-12). C'est la seule correction de fond du découpage ; la raison est au §3.4.
  - Le constructeur de clé de test, nommé comme non sûr, est interdit avec `public`.
  - Test de M27.
  - Le flux `Identification` ajouté par le découpage est validé.
- **C6 (T8)** : lints anti-panique dans le module de décodage (SEC0-23) ; le fuzzing tourne en CI (SEC0-24 révisée).
- **C7 (T7)** : **à scinder**. T7a est la PR « contrat » `ts_model`, avec `PlayerId` de 128 bits sans aucun type de compte et `standard_rules()` vide, **fusionnée avant** T7b, qui porte `ts_data` (chargement borné, SEC0-25) et `ts_world_model`. *Pourquoi : A23, un fichier partagé passe par une PR courte et dédiée, fusionnée avant le travail qui en dépend ; T7 entier dépasserait aussi les 400 lignes.* T8 peut tourner en parallèle de T7a.
- **C8 (T9)** : empreintes dorées et manifestes sans chemin local (SEC0-26) ; conditions de `ts_toy` ci-dessus.
- **C9 (T10)** : si `pyo3` exige `unsafe`, appliquer l'exception fermée de SEC0-19, avec une justification dans la PR.
- **C10 (T12)** : preuves selon SEC0-27 révisée. La PR jetable avec une dépendance vulnérable doit avoir `ci-ok` rouge ; le secret se prouve par le `pre-commit` et par l'auto-test de la CI, **jamais** en poussant un faux secret ni en contournant un crochet.
- **C11 (T13)** : SEC0-29 à SEC0-34 en entier. En particulier :
  - `permissions: {}` puis le minimum par travail ;
  - `persist-credentials: false` ;
  - **aucune interpolation `${{ github.event... }}` dans un `run:`**, ce qui concerne `pr-title` ;
  - liste fermée d'actions épinglées par SHA ;
  - auto-test de `gitleaks` ;
  - audit **quotidien** et le reste chaque semaine.
- **C12 (sous-agents au lieu de sessions)** : validé. Le relecteur des chemins sensibles est un **sous-agent neuf, distinct de celui qui a écrit**, et c'est lui qui pose `review-ok`.

## 10. Décisions et risques

| Date | N° | Décision (décidé) | Raison |
|---|---|---|---|
| 30/10 | AS-01 | Exigences de la phase 0 tenues dans ce carnet, puis consolidées dans les revues à l'audit de fin de phase | §1 |
| 30/10 | AS-02 | Copyleft fort (GPL, LGPL, AGPL) refusé partout | Le client contient presque tout le graphe |
| 30/10 | AS-03 | Aucune crate réseau en phase 0 | Rien n'en a besoin ; surface nulle |
| 30/10 | AS-04 | Autorisation de commit sur `main` liée à l'arbre exact vérifié | Une fusion ne peut porter que ce qui a été contrôlé |
| 30/10 | AS-05 | `ts_persist` créé en phase 0, réduit au codec | Garder `ts_kernel` pur ; le codec est une frontière de confiance |
| 30/10 | AS-06 | `ts_protocol` et `ts_view` ajoutés aux chemins sensibles | Décodage non fiable ; fuite d'information |
| 30/10 | AS-07 | Fonctionnalité `dev-bench` pour le banc et la pause de décision | M28 exclut aussi ces deux éléments du profil `public` |
| 30/10 | AS-08 | *Nightly* épinglée tolérée pour le fuzzing seulement ; repli stable sinon | Pas de CI en mode local. **Retirée le 30/10** : le fuzzing passe en CI, A14 est rétablie |
| 30/10 | AS-09 | `sops` et `age` reportés à la phase 4 | Aucun secret avant |
| 30/10 | AS-10 | Mention des contenus produits par IA globale, jamais par réplique | S7 |
| 30/10 | AS-11 | `tools/assets/` appartient au responsable contenus | Demande de la direction artistique, sans risque particulier |
| 30/10 | AS-12 | Aléa à clé : ChaCha8 en mode compteur, clés de flux dérivées par BLAKE3 ; Philox et `wyhash` refusés pour les flux à clé secrète | Un client voit beaucoup de sorties ; il ne doit pas pouvoir retrouver une clé et prédire les tirages. Modifie l'architecture §6.4, à reporter à la consolidation de fin de phase |
| 30/10 | AS-13 | Fin du mode local : fusion côté GitHub, approbations en commentaires de PR liés au SHA, aucun commit local sur `main` | Lier les approbations au contenu exact et les tracer hors du poste |
| 30/10 | AS-14 | Exigences de CI SEC0-29 à 34 ; pas de `CODEOWNERS` | Offre gratuite sans protection de branche : il serait sans effet |
| 30/10 | AS-16 | Avis final sur la PR #5 (T1), après la troisième relecture, la dernière. Un défaut n'est bloquant que s'il permet d'**écrire sur `main` hors fusion**, s'il **fait fuir un secret**, ou s'il produit un **faux refus** sur le travail courant (commit, push, worktree, `gh pr`, `gh issue`). Le reste part en *issues* `role:dev` avec mention de l'exigence SEC0 touchée. Ces *issues* restent **bloquantes pour la fin de la phase 0** (audit SEC0-28), pas pour la fusion de T1 | Décision de pilotage de l'orchestrateur : ces gardes sont une défense en profondeur ; les barrières principales sont la fusion réservée, mon avis et le contrôle de l'origine |
| 30/10 | AS-15 | Le crochet Claude Code passe au `.claude/settings.json` du projet, avec la parade SEC0-35 ; écart assumé à la clôture C4, à reporter à la consolidation | Les réglages utilisateur appartiennent à l'utilisateur (décision de l'orchestrateur) |

| N° | Risque | Parade |
|---|---|---|
| R-ARCH-01 | Aucune copie du dépôt hors du poste en mode local | **Levé le 30/10** : dépôt sur GitHub |
| R-ARCH-02 | Le fuzzing de repli, sans guidage par la couverture, trouve moins | **Sans objet** depuis le 30/10 : fuzzing guidé en CI |
| R-ARCH-03 | Pas de *ruleset* : une poussée sur `main` hors de Claude Code est seulement détectée après coup | SEC0-17 ; à revoir si l'offre change (décision de l'utilisateur, jamais demandée) |
| R-ARCH-04 | Une session peut retirer le crochet de son worktree entre deux fusions | Les crochets Git épinglés hors du dépôt bloquent encore le commit et la poussée vers `main` ; SEC0-35 détecte l'écart à la fusion suivante ; `ci-ok` et `origin-check` restent en place |

## 11. Demandes envoyées

| Date | À | Demande | État |
|---|---|---|---|
| 30/10 | responsable-dev (`local_111a89c2-3a9e-44cf-93b9-31c4421e6559`) | Lire §3, §5, §6 et §7 avant d'envoyer le découpage ; m'envoyer les consignes des tâches de §5 avant leur lancement | Envoyée |
| 30/10 | responsable conception (`local_5b3dcf29-8fbb-426c-b63a-80ab0302c1a6`) | Avis du §8 sur la direction artistique §12.9, pour mettre à jour l'état du §12.9 ; aucune règle de jeu changée | Envoyée |
| 30/10 | orchestrateur | Compte rendu des tâches 1, 3 et 4 ; R-ARCH-01 ; `CLAUDE.md` à aligner sur la clôture C4 (crochet Claude Code dans les réglages utilisateur, contrôle de l'origine hors CI) | Envoyée (compte rendu de fin de tour) |
| 30/10 | responsable-dev | Avis sur le découpage : validé avec corrections C1 à C12 (§9.1) ; réponse aussi sur [#2](https://github.com/CreatorInstant548/time_switcher/issues/2) | Envoyée |
| 30/10 | orchestrateur | Réglages du dépôt recommandés : *Allow select actions* avec la liste de SEC0-30 ; refus que les Actions créent ou approuvent des PR. Inscription de `guard-git.ps1` dans les réglages utilisateur (T1) | Envoyée |

## 12. Demandes reçues

| Date | De | Demande | État |
|---|---|---|---|
| 30/10 | orchestrateur | Tâches 1 à 4 de la consigne de lancement | Faites ; la tâche 2 est validée avec corrections (§9.1) |
| 30/10 | orchestrateur | Fin du mode local : adapter les exigences au dépôt GitHub | Faite : révision du §3, ajout du §3.7 |
| 30/10 | responsable-dev | Valider le découpage de la phase 0 ([#2](https://github.com/CreatorInstant548/time_switcher/issues/2)) | Faite (§9.1) |

## 13. Sessions actives lancées par moi

Aucune.
