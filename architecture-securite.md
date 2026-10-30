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
- [ ] **SEC0-05** — Toute nouvelle dépendance directe est justifiée dans la demande de fusion : besoin, alternative écartée, licence, présence d'un `build.rs` ou d'une macro procédurale. *Pourquoi : ces deux mécanismes exécutent du code sur le poste à la compilation.* Liste **pré-approuvée** pour la phase 0 : `bevy_ecs` (sans `bevy_app`), `serde`, `ron`, `blake3`, `thiserror`, `smallvec`, `arrayvec`, `chrono`, `chrono-tz`, `arrow`, `parquet`, `pyo3`, `numpy`, `rerun` (`default-features = false`), `proptest`, `arbitrary`, `libfuzzer-sys`, `clap`, `xshell` ou équivalent pour `xtask`, et `chacha20` (RustCrypto, pour l'aléa à clé, AS-12). Ajout du 30/10 (AS-17) : `serde_json`, pour les **crates d'outil seulement** (`xtask`, puis `ts_runner`), et jamais pour le cœur ni le protocole. Une dépendance de **cryptographie, de décodage de format externe ou de réseau** hors de cette liste demande mon avis **avant** la fusion.
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
  - **Révisé le 30/10 (AS-21).** Côté dépôt :
    - le jeton des workflows est en lecture seule, et l'approbation de PR par les Actions est désactivée (vérifié par l'orchestrateur) ;
    - la liste fermée d'actions **n'est pas réglée** : il faudrait un `gh api -X PUT`, que `guard-git` refuse (SEC0-14), et l'orchestrateur ne contourne pas ce refus.

    **Compensation dans le code :** l'étape « workflows » de `cargo xtask check` (F10, PR #11) refuse toute ligne `uses:` hors de la liste fermée ou non épinglée par un SHA de 40 caractères, et elle a des tests. La CI la lance dans son travail `security` (G7, T13), à chaque PR.

    **À vérifier dans mon avis sur la PR de T13 :** ce contrôle tourne bien en CI, il réussit sur les workflows livrés, et un test montre le refus d'une action hors liste.
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
- **Marques provisoires** : `// PROVISOIRE(conception #<id>)`. **Depuis le 30/10 (AS-25) :** `// PROVISIONAL(design #<id>)`. `spec-map` accepte les deux formes jusqu'à la traduction du code existant, puis seulement la nouvelle.
- **Sections** : les titres Markdown de `docs/conception/**/*.md` (hors `docs/archives/`). Les identifiants de décision sont relevés dans les tableaux de décisions et dans les lignes des attendus.

**Sorties**
- `spec-map report` : code → sections, et sections → code ; **sections sans code**, filtrables par fichier et par phase ; liste des `PROVISOIRE` avec leur remontée. Sortie en Markdown sur la sortie standard, et `--json` pour les outils (tableau du responsable qualité, organisation §3.1 point 4).
- `spec-map check` (appelé par `xtask check`) : **échoue** sur un renvoi vers un fichier, une section ou un identifiant qui n'existe pas, sur un nom court ambigu, et sur un `PROVISOIRE` sans numéro.
- `spec-map diff <révision>` : pour les sections modifiées depuis une révision de la conception, liste le code qui les cite. C'est l'outil du responsable conception (« quand une section change, il voit le code touché »).

**Contraintes**
- Lecture seule, sans réseau ni dépendance nouvelle hors de la liste pré-approuvée (analyse des titres à la main, pas de bibliothèque Markdown).
- Déterministe : sortie triée.
- Tests sur un petit corpus de documents et de code factices.

**Révision du 30/10 (H1, H2, H7 de l'avis sur la consigne T4, §9.7), qui fait foi**

- **Marques de commentaire (H7)** :
  - `// spec:`, `//! spec:` et `/// spec:` en Rust, C# et **RON** : RON se commente par `//` et non par `#`, comme le disait par erreur la version initiale ;
  - `# spec:` en Python, TOML, YAML, shell et PowerShell ;
  - les mêmes formes pour `PROVISOIRE(conception #<id>)`.

  Sont analysés les fichiers suivis par Git, sauf `*.md`, `docs/` et `docs/archives/`.
- **Grammaire d'un renvoi (H1)**, conforme aux renvois déjà présents dans le dépôt :
  - **Découpage :** `;` sépare les fichiers. Dans un même fichier, `,` sépare les éléments.
  - **Élément :** c'est une section `§n(.m)*` (avec `bis`, `ter` ou une lettre en option), un identifiant, ou une plage `X à Y` de même préfixe (`SEC0-29 à 31`, `F1 à F10`).
  - **Préfixe repris :** un nombre seul reprend le préfixe de l'identifiant qui le précède (`SEC0-03, 06`).
  - **Contexte :** une section peut précéder un identifiant pour le situer (`§9.5 F1 à F10`).
  - **Parenthèses :** un texte entre parenthèses est ignoré (`(révisé)`, `(chemins sensibles)`).
- **Renvois vers un carnet (H1)** : `carnet <rôle>.md …` se résout dans `origin/pilotage:<rôle>.md`, lu par `git show`, en lecture seule.
  - Si `origin/pilotage` est absent, l'étape **échoue** avec le message « git fetch origin pilotage ». Jamais de réussite silencieuse.
  - La CI (T13) récupère cette branche.
  - Ces renvois sont réécrits vers les revues à la consolidation de fin de phase 0.
- **Identifiants (H2)** : la liste fermée de lettres (S, D, P, E, G, A, M) est remplacée par une règle générale.
  - **Forme :** un identifiant `[A-Z][A-Z0-9]*(-[A-Z0-9]+)*-?\d+`, par exemple `S8`, `M28`, `A19`, `SEC0-35`, `AS-12`, `R-ARCH-04`, `T0-06` ou `F3`.
  - **Validité :** il est valide s'il est **défini** dans le fichier cible. Une définition est soit la première cellule d'une ligne de tableau, soit un gras en tête d'élément de liste (`- **X**`, `- **X.**`, `- [ ] **X**`), soit un gras en tête de ligne.
  - **Dans une section :** quand une section précède l'identifiant, celui-ci doit être défini dans cette section.
  - **Définitions multiples (AS-20, 30/10)** : un identifiant cité sans section reste valide s'il est défini plusieurs fois dans le fichier cible, par exemple dans un tableau de synthèse et dans le détail. `report` signale alors chaque identifiant défini plusieurs fois, avec ses sections, pour que la conception puisse lever le doute. `check` n'échoue pas pour autant.
- **Points numérotés (AS-19, 30/10)** : un élément de liste numérotée de premier niveau, placé directement sous un titre `§k`, définit `§k.N`, sauf si un vrai sous-titre `k.N` existe. Si deux listes sous le même titre définissent le même `§k.N`, le renvoi est **ambigu** et `check` échoue.

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
| 30/10 | Consigne T2a (espace de travail Cargo, V et S) | `side-session-notes/…/consigne-T2a.md` | **Validé avec corrections E1 à E9** (§9.4) ; lancement permis après la fusion de la PR #5 |
| 30/10 | **PR #5 (T1), fusion** | SHA de tête `92ecc11915c3ba6ae49a589621a87467ed89f843` ; `review-ok` de la troisième relecture neuve | **`archsec-ok: 92ecc11915c3ba6ae49a589621a87467ed89f843`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/5#issuecomment-6514007221)) : aucun défaut bloquant au sens d'AS-16 ; conditions du §9.3 remplies ; non bloquant : m7 à m15 et [#6](https://github.com/CreatorInstant548/time_switcher/issues/6) (T3), bloquants pour la fin de la phase 0 ; R-ARCH-05 |
| 30/10 | Consigne T2b (`xtask check`, `slots`, `disk` ; V et S) | `side-session-notes/…/consigne-T2b.md` | **Validé avec corrections F1 à F10** (§9.5) ; `serde_json` accepté pour les outils (AS-17) ; à découper en deux PR ; lancement après la fusion de la PR #10 |
| 30/10 | **PR #10 (T2a), fusion** | SHA de tête `f14fb969ba0d0751b8780b586121cabad282fe44` ; `review-ok` du relecteur neuf | **`archsec-ok: f14fb969ba0d0751b8780b586121cabad282fe44`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/10#issuecomment-6514394030)) : E1 à E9 appliquées ; SEC0-02, 07, 19, 20 conformes ; non bloquant : `try_from_os_rng`, m3, m4 (→ F4, T2b) |
| 30/10 | Consigne T13 (CI GitHub Actions ; V et S) | `scratchpad/…/consigne-T13.md` du responsable développement | **Validé avec corrections G1 à G12** (§9.6) ; pas de `.gitleaks.toml` à la racine ; lancement après la fusion de T2b |
| 30/10 | **PR #11 (T2b, `xtask check`), fusion** | SHA de tête `067a0cbaaa2711ad4b763d9d5e0d63fb6198e6ac` ; `review-ok` du vérificateur neuf (deux tours) | **`archsec-ok: 067a0cbaaa2711ad4b763d9d5e0d63fb6198e6ac`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/11#issuecomment-6515590937)) : F1 à F7, F9 et F10 conformes ; aucune réussite silencieuse ; non bloquant : [#12](https://github.com/CreatorInstant548/time_switcher/issues/12), dont A1 (AS-18 à l'arrivée de `ts_py`), A2 (mode CI de gitleaks, pour T13) et A3 (`git ls-tree` dans `guard-git`) ; écart de taille (≈ 1 900 lignes) accepté |
| 30/10 | Consigne T4 (`xtask spec-map` ; S) | `scratchpad/…/consigne-T4.md` du responsable développement | **Validé avec corrections H1 à H8** (§9.7) ; §7 révisé (grammaire, carnets, identifiants, RON), `spec-map-spec.md` à recopier ; lancement après la fusion de la PR #11 |
| 30/10 | **PR #15 (T4, `spec-map`), fusion** | SHA de tête `516ff46871adf07b6e2e7bc56db8b76ddf116f82` ; `review-ok` du relecteur neuf | **`archsec-ok: 516ff46871adf07b6e2e7bc56db8b76ddf116f82`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/15#issuecomment-6516391405)) : H3 à H6, SEC0-26, AS-19 et AS-20 conformes ; `known_bad.txt` vide ; non bloquant : [#18](https://github.com/CreatorInstant548/time_switcher/issues/18) |
| 30/10 | **PR #14 (T13, CI), contenu** | SHA de tête `5a474134fcd1e3243d23c2bbf72caaed6fc350ce`, `ci-ok` vert ; `review-ok` du relecteur neuf | **`archsec-ok: 5a474134fcd1e3243d23c2bbf72caaed6fc350ce`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/14#issuecomment-6516392004)) : SEC0-29 à 32, G1, G2, G4 à G11, AS-21 et A2 conformes ; non bloquant : [#17](https://github.com/CreatorInstant548/time_switcher/issues/17) (`concurrency`), [#6](https://github.com/CreatorInstant548/time_switcher/issues/6) (T3-a, T3-b, G3) ; **nouvel avis à donner sur le SHA après la mise à jour depuis `main`** |
| 30/10 | Consigne T3a (`xtask merge`, `verify-main`, `hooks` ; V et S) | `scratchpad/…/consigne-T3a.md` du responsable développement | **Validé avec corrections J1 à J8** (§9.8) ; `merge-pr.ps1` et `install-hooks.ps1` supprimés ; deux relecteurs neufs ; lancement après la fusion des PR #16 et #14 |
| 30/10 | **PR #16 (T2b, `slots` et `disk`), fusion** | SHA de tête `95b66f2fdd6a43a47cb68cefcb7ec5d4890c172b` ; `review-ok` du relecteur neuf (delta revérifié) | **`archsec-ok: 95b66f2fdd6a43a47cb68cefcb7ec5d4890c172b`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/16#issuecomment-6516870411)) : F8 respecté (tout point de réanalyse traité comme lien, `within` canonique, bail atomique, orphelins après 24 h) ; non bloquant : m3, m7 ; délai entre contrôle et suppression accepté |
| 30/10 | **PR #14 (T13, CI), fusion** | SHA de tête `e3e2f2a842612bebfba1a8388f86fd296f64344c`, fusion de `5a47413` et de `main` (`14b6c58`) ; `ci-ok` vert ; `review-ok` du delta | **`archsec-ok: e3e2f2a842612bebfba1a8388f86fd296f64344c`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/14#issuecomment-6517058986)) : fichiers de la PR identiques à `5a47413` ; apport identique ligne pour ligne avant et après la fusion |
| 30/10 | Consigne T5 (noyau, première partie ; V et S) | `scratchpad/…/consigne-T5.md` du responsable développement | **Validé avec corrections K1 à K10** (§9.9) ; chemins sensibles `rng/` et `build_profile.rs` validés ; deux PR (T5a, puis T5b pour l'aléa en XChaCha8) ; travail en parallèle de T3a accepté, fusion après T3a |
| 30/10 | **PR #20 (T5a, noyau : temps, identifiants, argent), fusion** | SHA de tête `bbe2c76d144f58f3f977b99ad53baba003cd0ae2`, `ci-ok` vert ; `review-ok` du relecteur neuf | **`archsec-ok: bbe2c76d144f58f3f977b99ad53baba003cd0ae2`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/20#issuecomment-6517813891)) : SEC0-20, K7, K9 et AS-22 (phase 0) conformes ; peut être fusionnée avant T3a par `merge-pr.ps1`, qui exige désormais `ci-ok` et les deux approbations ; non bloquant pour la phase 1 : `Default` de `SimDay`, glissement en fin de journée |
| 30/10 | **PR #24 (T5b, aléa à clé), fusion** | SHA de tête `e5af3b3a191095a888a472c59621ab6e556b0080`, `ci-ok` vert ; deux `review-ok` indépendants (vecteurs recalculés en Python ; modèle de menace) | **`archsec-ok: e5af3b3a191095a888a472c59621ab6e556b0080`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/pull/24#issuecomment-6518935742)) : SEC0-22, AS-12, M27 et K2 à K6 conformes ; K10 accepté avec la sonde `cl.exe` (AS-24) ; [#26](https://github.com/CreatorInstant548/time_switcher/issues/26) bloquante avant le premier tirage d'un système de règles (`DrawSite`) |
| 30/10 | Consigne T6 (noyau, deuxième partie ; V) | `scratchpad/…/consigne-T6.md` du responsable développement | **Validé avec corrections L1 à L8** (§9.10) ; `bevy_ecs` en version exacte, avec `std` et `multi_threaded` seulement ; marqueur `LodComplete` dans le noyau, réexporté par `ts_model` ; empreinte d'état non sensible ; clé du monde hors de l'instantané |
| 30/10 | **PR #23 (T3a, `merge` et `hooks`), fusion** | SHA de tête `dbfaf1311c78188bed3e369c41dd814a085911b8`, `ci-ok` vert ; **sans relecteur indépendant**, sur décision de l'utilisateur (AS-26) | **`archsec-ok: dbfaf1311c78188bed3e369c41dd814a085911b8`** ([comment](https://github.com/CreatorInstant548/time_switcher/pull/23#issuecomment-6521227237)) : avis fondé sur ma propre lecture de la tête ; J1 à J8, G3, G8 et AS-23 conformes ; `merge-pr.ps1` et `install-hooks.ps1` supprimés ; la #25 (`verify-main`) à fusionner juste après ; R-ARCH-07 |
| 30/10 | **PR #27 (T6a : ordonnanceur, phases, greffons, boucle), fusion** | SHA de tête `1100acd4581eff7acad141f278c3ad69c5864a9e` ; `security` vert, compilations en cours ; sans relecteur indépendant (AS-26) | **`archsec-ok: 1100acd4581eff7acad141f278c3ad69c5864a9e`** ([comment](https://github.com/CreatorInstant548/time_switcher/pull/27#issuecomment-6521443676)) : L1 (`bevy_ecs` et `bevy_tasks` en `=0.20.0`, avec `std` et `multi_threaded` seulement), L2 et L7 conformes. Les crates optionnelles listées dans `Cargo.lock` ne sont pas construites, ce que `cargo deny` vert confirme. `WorldStreams` doit être exclu de l'instantané et de l'empreinte (L4, T6b). La fusion attend `ci-ok` |
| 30/10 | **PR #25 (T3a, `verify-main`), fusion** | SHA de tête `7a99acfece62e6f52d4ee561a2be2bd7e36b2d4c`, `ci-ok` vert ; sans relecteur indépendant (AS-26) | **`archsec-ok: 7a99acfece62e6f52d4ee561a2be2bd7e36b2d4c`** ([comment](https://github.com/CreatorInstant548/time_switcher/pull/25#issuecomment-6521528453)) : SEC0-17, G3, J3 à J5 conformes. La mise à jour depuis `main` ne retire de `merge.rs` que l'ancienne étape 2, donc la crainte liée à `-X ours` est levée. Non bloquant : un cache des commits déjà vérifiés, contre le coût linéaire en appels à l'API (T3b) |
| 30/10 | **PR #25, après mise à jour depuis `main` (#27)** | SHA de tête `9bcfde3225b437a78f9a4fc8234900528c3b9ec7`, `ci-ok` vert ; fusion simple, sans conflit | **`archsec-ok: 9bcfde3225b437a78f9a4fc8234900528c3b9ec7`** ([comment](https://github.com/CreatorInstant548/time_switcher/pull/25#issuecomment-6521740976)) : lignes ajoutées et retirées par la PR identiques, octet pour octet, avant et après la mise à jour |
| 30/10 | **PR #30 (T6b : empreinte, instantané, `WorkBudget` ; remplace la #28), fusion** | SHA de tête `cbe11b51e04fa71e058577018c11d35cfe9bc79d`, `ci-ok` vert ; relecteur neuf en cours sur les corrections I1 à I3 | **`archsec-ok: cbe11b51e04fa71e058577018c11d35cfe9bc79d`** ([comment](https://github.com/CreatorInstant548/time_switcher/pull/30#issuecomment-6521795909)) : L3 à L6 conformes ; preuve L1 par `cargo tree`. Deux points d'API confirmés : `state_hash()` sans `HashScope`, et `restore(&WorldKey)`. Non bloquant : un `StableId` en double devra devenir une erreur (T7 ou T8) |
| 30/10 | **PR #30, après F1 et mise à jour depuis `main` (#25)** | SHA de tête `4d1a4aebcb75aabbd01f009bdcbec6c3aaeb9bf8` ; `ci-ok` absent (GitHub Actions bloqué par la facturation du compte) | **`archsec-ok: 4d1a4aebcb75aabbd01f009bdcbec6c3aaeb9bf8`** ([comment](https://github.com/CreatorInstant548/time_switcher/pull/30#issuecomment-6522140160)) : delta inchangé, sauf le texte du contrat AS-27 et les `TODO(#31)` ; aucune ligne de code modifiée. La fusion attend `ci-ok` |
| 30/10 | Consignes T6c, T7a et T9w | `scratchpad/…/consigne-T6c-clippy.md`, `consigne-T7a.md`, `consigne-T9w.md` | **T6c validée avec P1** : interdire seulement `ambiguous_with`, le reste étant refusé à la construction et utilisé par les tests du noyau. **T7a validée avec M1 à M3** : `Pilot` reste privé à `ts_pilot` (architecture §6.6). **T9w validée avec N1 à N4**, dont le trait scellé `ClientShippable` pour la part publique (§9.11) |

### 9.11 Consignes T6c (`clippy.toml`), T7a (`ts_model`) et T9w (`ts_world_model`) (30/10)

**T6c : validé avec correction P1.**
- **P1. Dérogations d'ambiguïté.**
  - **Ce qu'on n'interdit pas dans clippy :** `ambiguous_with_all`, `allow_ambiguous_component` et `allow_ambiguous_resource`.
    - *Pourquoi :* le noyau les refuse déjà à la construction (`BuildError::AmbiguityOptOut`, L7). Ce refus couvre tous les greffons et ne peut pas être désactivé.
    - Les tests du noyau les appellent pour prouver ce refus. Une interdiction dans clippy les frapperait, et une exemption est impossible dans le noyau (E6, F5).
  - **Ce qu'on interdit dans clippy :** `ambiguous_with`, la forme ciblée. Le noyau ne sait pas la détecter à l'exécution, son code le dit.
- **Tables à hachage de `bevy_ecs` :** les interdire est validé. Il faut vérifier par la démonstration si les alias (`bevy_platform::collections::HashMap`) sont déjà couverts par l'interdiction de `hashbrown::HashMap`. Sinon, on les nomme.

**T7a : validé avec corrections M1 à M3.**
- **M1. `Pilot` ne va pas dans `ts_model`.** L'architecture §6.6 dit que le type `Pilot` et son contenu sont **privés à `ts_pilot`**. Les règles ne voient que les marqueurs publics `AiDriven` et `AiObserving`, et « le monde ne connaît pas les joueurs » est vérifié par le compilateur (R13).
  - **Dans T7a :** seulement `PlayerId` (SEC0-21) et les marqueurs `AiDriven` et `AiObserving`.
  - **Plus tard :** `Pilot` et l'interdiction de `External` avec `public` viendront avec la crate `ts_pilot` (phase 1, échange de pilote). Le test `compile_fail` de `Pilot::External` attend lui aussi `ts_pilot`.
- **M2. `PlayerId`.**
  - **Construction :** un seul constructeur, depuis des octets de pseudonyme (HMAC tronqué, fourni par l'hôte), nommé comme tel (`from_pseudonym_bytes`).
  - **Ce qu'il n'a pas :** ni `From` ni `Into` depuis ou vers un entier ou une chaîne, ni `Display`, ni `Serialize`. Son `Debug` est abrégé, par exemple aux 4 premiers octets en hexadécimal.
  - **Empreinte :** `StateHash` est permis, parce que l'identifiant est un pseudonyme et non une donnée personnelle directe.
  - **Comptes :** aucun type d'identifiant de compte dans `ts_model`.
- **M3. Squelettes d'`Intent`, `Command` et `Event`.** `Command` marque déjà les variantes d'écriture comme réservées aux fonctionnalités de développement (matrice des commandes, S8). Elles ne compilent que sous `dev-commands`, même si elles restent vides ou presque.

**T9w : validé avec corrections N1 à N4.**
- **N1. Calculs sans dépassement.**
  - **Le type :** `Cm(i32)` est validé (±21 km).
  - **Produits :** les produits (aires, produits vectoriels, distances au carré) se font en `i64`, et en `i128` si une somme de produits peut dépasser `i64`. Ils ne se font jamais en `i32`.
  - **Tests :** aux bornes de la plage permise.
- **N2. `quantize` unique.**
  - **Signature :** il prend un `f64` en mètres, arrondit au centimètre **au pair le plus proche** (`round_ties_even`), même règle que l'argent, et rend un `Result`.
  - **Refus :** `NaN`, ±∞ et toute valeur hors de la plage. Jamais de saturation silencieuse.
- **N3. Part serveur, fermée par les types (P12, S33).**
  - **Marqueur :** un trait marqueur scellé `ClientShippable`, implémenté **seulement** par les types de la part `public`.
  - **Usage futur :** tout futur empaqueteur du paquet client exigera ce trait.
  - **Parts fermées :** les parts `candidates` et `server` ne peuvent pas l'implémenter hors de la crate, et un test `compile_fail` le montre.
  - *Pourquoi :* une documentation seule ne suffit pas. Le paquet client est public, et y laisser filer la part serveur révélerait serrures, caméras et coffres.
- **N4. Identités de la carte.** Les entités de la carte utilisent `StableId::map(index)` (P28), jamais l'allocateur du monde.

### 9.10 Consigne T6 : noyau, deuxième partie (30/10)

Avis **validé avec corrections L1 à L8**, toutes décidées.

- **L1. `bevy_ecs` : version et fonctionnalités.**
  - **Version :** la dernière version stable publiée depuis au moins deux semaines, épinglée en version exacte (`=0.x.y`) dans `[workspace.dependencies]`. *Pourquoi :* chez Bevy, chaque version mineure change l'API ; sa montée doit être une PR délibérée, jamais une simple mise à jour de `Cargo.lock`. Dependabot peut la proposer, et la PR est alors sensible (AS-23, point 4, et `Cargo.lock` qui ajoute des crates).
  - **Fonctionnalités :** `default-features = false`, avec seulement `std` et `multi_threaded` (nécessaire à `par_iter` et au test sur 4 fils).
  - **Refusées :** `bevy_reflect`, `serialize`, `trace`, `detailed_trace`, `track_location` et tout ce qui tire `bevy_app`. *Pourquoi :* la réflexion donne un accès dynamique aux composants, contraire à la vue filtrée et inutile au cœur. `serialize` serait inutile, puisque l'instantané a son propre codec (T8). Les traces sont un coût.
  - **Justification dans la PR :** la liste des crates ajoutées à `Cargo.lock`, et les macros procédurales et `build.rs` du graphe (`bevy_ecs_macros`, `bevy_macro_utils` et leurs dépendances). `cargo deny check` reste vert, sans aucune exception de licence.
- **L2. Le marqueur « simulation complète » : la frontière est acceptée.**
  - **Origine :** le marqueur est défini **une seule fois**, dans le noyau, sous le nom de l'architecture (`LodComplete`, §6.5). `ts_model` le réexporte et ne le redéfinit jamais.
  - **Rôle dans le noyau :** il sert seulement à décider du saut de temps, et le noyau ne lui donne aucune autre sémantique.
  - **Marqueurs voisins :** `LodSimplified` et `LodStatistical` restent dans `ts_model`, puisque le noyau n'en a pas besoin.
- **L3. L'empreinte d'état n'est pas un chemin sensible.** Elle ne protège aucun secret. Elle sert au déterminisme, à la reprise et aux comparaisons, tandis que l'intégrité des fichiers reste l'affaire des empreintes BLAKE3 des parts dans `ts_persist`, qui est sensible. Son encodage canonique est en revanche **figé par des vecteurs de référence** dans les tests : en changer, c'est changer toutes les empreintes dorées, et cela se signale dans la PR.
- **L4. La clé du monde n'entre jamais dans l'état sauvegardé, ni dans l'empreinte.**
  - `restore(snap, cfg, rules, world_key: &WorldKey)` reçoit la clé de l'hôte. Les `StreamKeys` sont **dérivées de nouveau** à la restauration.
  - Aucune clé n'est `StateHash` ni comprise dans `Snapshot`. Un test vérifie que les octets d'un instantané de test ne contiennent pas la clé.
  - *Pourquoi (P25) :* un instantané circule davantage que la clé, en sauvegarde, en rapport de bogue ou en relecture. L'en-tête du monde (architecture §12.1) ne portera en ligne qu'une **référence** à la clé, pas sa valeur. C'est à préciser dans T8 et en phase 4.
- **L5. `restore` vérifie l'empreinte de règles.** Une empreinte différente de celle de l'instantané est refusée par une erreur typée, sauf si une migration déclarée existe (§12.5). `DayOrigin` et les compteurs, `StableIdAllocator` et `seq`, font partie de l'état.
- **L6. Le test « 1 fil contre 4 » doit vraiment changer le nombre de fils.** Le `ComputeTaskPool` de `bevy_tasks` est global et ne s'initialise qu'une fois par processus. Chaque configuration tourne donc dans un **processus distinct** : un test qui relance l'exécutable de test avec une variable, ou deux tests `#[ignore]` lancés séparément par `xtask`. Le test vérifie, par une assertion, le nombre de fils effectif.
- **L7. Ambiguïtés en erreur.** `ScheduleBuildSettings { ambiguity_detection: LogLevel::Error, .. }` s'applique à chaque `Schedule` de phase. Un test montre qu'il refuse deux systèmes de test qui écrivent la même ressource sans ordre.
- **L8. Découpage.** Au-delà d'environ 600 lignes, on fait deux PR : d'abord l'ordonnanceur, les phases, les greffons et la boucle, ensuite l'empreinte, l'instantané, `WorkBudget` et le test sur plusieurs fils.

**Revue.** T6 ne touche aucun chemin sensible : `/code-review` et un relecteur neuf suffisent. Mon `archsec-ok` reste exigé par `merge`, parce que `Cargo.lock` ajoute des crates. Critères bloquants : L1, L4, L5 et L6, plus SEC0-19, aucun `unsafe` de notre côté.

### 9.9 Consigne T5 : noyau, première partie (30/10)

**Réponses aux deux questions :**
1. **Chemins sensibles : validés.** `crates/ts_kernel/src/rng/` et `crates/ts_kernel/src/build_profile.rs` sont ajoutés à `sensitive-paths.txt` dans la même PR. Les quatre `compile_error!` **déménagent aussi** dans `build_profile.rs`, avec leurs messages inchangés : l'étape `profiles` de `xtask` les cherche mot pour mot.
2. **Parallélisme : accepté.** T5 peut tourner pendant T3a, pour le travail seulement. Sa fusion vient après celle de T3a. *Pourquoi :*
   - les deux tâches ne partagent aucun fichier, puisque T3a lit `sensitive-paths.txt` sans l'écrire ;
   - deux compilations en même temps restent sous la limite de trois ;
   - l'ordre qui compte pour la sécurité, celui des fusions, est conservé.

Avis **validé avec corrections K1 à K10**, toutes décidées.

- **K1. Deux PR d'emblée.**
  - **T5a :** `SimTime`, `SimDay`, `StableId`, `Money`, M7 et le déplacement vers `build_profile.rs`.
  - **T5b :** l'aléa (`rng/`), avec `chacha20` et `blake3`.

  *Pourquoi :* l'aléa est le cœur de SEC0-22. Il mérite sa propre revue, et l'ensemble dépasserait 400 lignes. T5b peut partir dès la fusion de T5a.
- **K2. Les clés de flux sont dérivées une fois, puis gardées.**
  - `WorldKey` donne, à la création du monde, une table `StreamKeys` : une `StreamKey` de 256 bits par flux, obtenue par BLAKE3 `derive_key`.
  - Le tirage prend `&StreamKey`, sans redériver la clé à chaque appel : `draw(&StreamKey, subject, slot, purpose) -> u64`. La fonction reste pure.
  - *Pourquoi : une dérivation BLAKE3 à chaque tirage multiplierait le coût.*
- **K3. Disposition du bloc ChaCha : XChaCha8.**
  - Le *nonce* de 192 bits vaut `subject (u64 LE) ‖ slot (u64 LE) ‖ purpose (u32 LE) ‖ 0u32`. Le compteur de bloc désigne le bloc suivant quand on veut plus de valeurs.
  - Un bloc donne 8 valeurs `u64` en petit-boutiste : `draw` rend la première, `draw_block` rend les 8.
  - *Pourquoi :* le triplet (sujet, créneau, usage) fait 160 bits. Il n'entre pas dans les 128 bits de *nonce* et de compteur de ChaCha8, qu'il soit IETF ou d'origine. XChaCha8 l'accepte tel quel, et la disposition est injective.
  - Le coût de l'étape HChaCha est inclus dans la mesure R5.
- **K4. Contextes de dérivation.**
  - Chaque contexte est une chaîne fixe et unique, construite à partir du **nom** du flux, jamais de son rang : `"time_switcher 2026-10-30 stream <Nom> v1"`, et `"time_switcher 2026-10-30 cosmetic v1"` pour le domaine cosmétique.
  - Ajouter un flux ne change aucun autre flux.
  - Les vecteurs de référence figés sont placés **dans `rng/`**, chemin sensible. Les modifier rompt la reproductibilité de tous les mondes.
- **K5. `WorldKey`.**
  - **Construction :** un seul constructeur public, `from_bytes([u8; 16])`, pour l'hôte qui charge ou tire la clé. La clé de test `insecure_from_u64_for_tests` est réservée à `cfg(any(test, feature = "dev-commands", feature = "external-pilot"))`.
  - **Ce que le type n'a pas :** ni `Copy`, ni `Clone` public, ni `Serialize`, ni `PartialEq`, ni `Hash`. Son `Debug` affiche `WorldKey(..)`.
  - **Clés de flux :** les `StreamKey` suivent les mêmes règles.
  - **Preuve de l'absence de la clé de test :** un *doctest* `compile_fail` suffit. Il est compilé sans fonctionnalité de développement, ce qui couvre `public`.
- **K6. Domaine cosmétique.**
  - **Dérivation :** une clé cosmétique racine, de type privé, est dérivée de `WorldKey` par son contexte propre. `CosmeticSeed(u64)` est tiré de cette racine, par usage et par sujet.
  - **Isolement :** `CosmeticSeed` ne se convertit ni en `WorldKey` ni en `StreamKey`, et aucune fonction de tirage ne l'accepte en entrée.
  - **Vérification :** le test M27 porte sur au moins 10⁵ valeurs.
- **K7. `Money`.**
  - Les barèmes s'expriment en **points de base**, en `i64`.
  - Le calcul passe par un intermédiaire `i128`. L'arrondi au pair le plus proche est symétrique pour les montants négatifs, et le résultat est vérifié : `Option`, jamais d'enroulement.
  - Les tests couvrent les cas `.5`, les montants négatifs et les bornes.
- **K8. `SimDay(u16)` et temps antérieur à l'époque.** Les dates de naissance générées précèdent l'époque du monde, donc `SimTime` peut être négatif. La conversion `SimTime → SimDay` rend une `Option`, ou une erreur typée, hors de `0..=u16::MAX`. On ne tronque jamais en silence.
  - **Remplacé le 30/10 par l'arbitrage A12 de la conception (PR #22) et par AS-22.** `SimDay(i16)` est signé et compté par rapport à une **origine glissante** (`DayOrigin`). La conversion sature au minimum, et au maximum par sécurité ; il n'y a plus d'`Option`. Le mécanisme est décrit à AS-22.
- **K9. `StableId`.**
  - **Carte :** le bit 63 marque l'espace de noms réservé à la carte (P28), et le constructeur `StableId::map(index)` en est le seul accès.
  - **Allocateur :** il refuse de dépasser `2^63 − 1`.
  - **Instantanés :** le décodage passe par un constructeur public nommé `from_snapshot_raw`, documenté comme réservé à `ts_persist`.
  - **Rappel (architecture §6.1) :** `StableId` n'est jamais envoyé au client. C'est le rôle de `ViewHandle`, hors du périmètre de T5.
- **K10. Dépendances.**
  - `chacha20` sans les fonctionnalités par défaut, avec seulement ce qu'exige XChaCha8.
  - `blake3` avec `default-features = false` et `features = ["pure"]`. *Pourquoi :* sans le code C et assembleur SIMD, le `build.rs` n'appelle plus de compilateur C. Il y a donc moins de code exécuté à la construction, et rien à installer. La perte de vitesse est sans effet, puisqu'on ne dérive des clés qu'à la création du monde.
  - Les deux crates sont dans la liste pré-approuvée de SEC0-05, et la PR donne leur justification.

**Revue :** T5a passe par `/security-review`, à cause de `build_profile.rs` et de `sensitive-paths.txt`, puis un relecteur neuf et mon `archsec-ok`. T5b demande en plus un deuxième relecteur neuf, chargé de l'aléa. Critères bloquants :
- pour T5a : SEC0-20, K8 et K9 ;
- pour T5b : SEC0-22, AS-12, M27 et K2 à K6.

### 9.8 Consigne T3a : `xtask merge`, `verify-main` et `hooks` (30/10)

Avis **validé avec corrections J1 à J8**, toutes décidées. Lancement après la fusion de la PR #16, puis de la PR #14. Le découpage T3a / T3b est validé. La suppression de `merge-pr.ps1` dans la PR de T3a remplit la condition du §9.3 : un seul chemin de fusion. La PR de T3a est la dernière fusionnée par `merge-pr.ps1`.

- **J1. Contrôle des workflows rejoué (T3-a) : en mémoire, sans worktree.**
  - `merge` récupère la tête de la PR par `git fetch origin pull/<n>/head`.
  - Il lit chaque `.github/workflows/*` par `git show <sha>:<chemin>` et appelle directement `workflows::check_workflow` : c'est le code de `main`, puisque `merge` tourne depuis la copie principale.
  - *Pourquoi :* le résultat est le même, sans worktree temporaire à créer, à nettoyer et à garder hors de `worktree list`.
- **J2. `merge` ne tourne que depuis la copie principale.** L'alias `cargo xtask` compile la crate `xtask` de la copie où on le lance. Lancé depuis un worktree, `merge` exécuterait donc le code de la branche, qui pourrait sauter ses propres contrôles. Deux barrières :
  - **`guard-git`** permet `cargo xtask merge` seulement si le dossier courant est la racine de la copie principale. Il refuse aussi `cargo run … -p xtask … merge`, et toute autre forme qui lance la fusion depuis ailleurs.
  - **`merge`** vérifie, en plus, que l'exécutable a été compilé dans `<copie principale>\target`, et qu'il a été compilé après le dernier commit de `main`.
- **J3. Lien entre un commit de `main` et sa PR (`verify-main`).** Il passe par l'API, jamais par le titre du commit, qui se falsifie :
  - `gh api repos/{o}/{r}/commits/<sha>/pulls` donne la PR ;
  - il faut `merged = true` et `merge_commit_sha == <sha>` ;
  - le `ci-ok` (G3) et les deux approbations portent sur le `head.sha` de cette PR.
- **J4. Recoupement avec le §9 du carnet.** La ligne cherchée contient littéralement `archsec-ok: <40 caractères hexadécimaux>`. Elle est lue par `git show origin/pilotage:architecture-securite.md`, entre le titre `## 9.` et le titre suivant de même niveau. Si `origin/pilotage` est absente, `verify-main` échoue.
- **J5. Comment lever une anomalie de `verify-main`.** Une anomalie de l'historique ne disparaît pas : un commit poussé hors fusion reste dans `main`, et l'on ne force jamais la poussée. Mécanisme :
  - **La liste :** une liste fermée `ACKNOWLEDGED` dans le code de `xtask` donne, pour chaque anomalie, le SHA, la raison et l'*issue* `role:architecture`.
  - **L'ajout :** il passe par une PR de chemin sensible, avec mon `archsec-ok`.
  - **La suspension des fusions :** tant qu'une anomalie n'est pas dans la liste, `merge` refuse.
  - **L'*issue* :** `verify-main` réutilise l'*issue* ouverte qui porte déjà le même SHA, sans créer de doublon.
- **J6. `hooks verify`, contrôles en plus.**
  - Lecture seule des réglages **utilisateur** `~/.claude/settings.json` : échec si `disableAllHooks` y est vrai, puisqu'il désactiverait aussi les crochets du projet. *Pourquoi : c'est le seul endroit qu'aucun garde du dépôt ne protège. On le lit sans jamais l'écrire, conformément à la décision C4 de l'orchestrateur.*
  - Le `.claude/settings.json` de chaque copie de travail est comparé à `git show HEAD:.claude/settings.json` de cette copie, et à `origin/main` pour la copie principale.
- **J7. Installation des crochets.** La comparaison avec `origin/main` porte sur le contenu, octet pour octet, de chaque fichier de `tools/dev/hooks/` et de `tools/dev/tools.lock`, lus par `git show`. L'empreinte du jeu se calcule comme dans `install-hooks.ps1` (liste triée « nom NUL sha256 LF »), pour que les installations actuelles restent reconnues. `install-hooks.ps1` est **supprimé**, et non réduit à un renvoi : un seul chemin d'installation.
- **J8. Actions épinglées (T3-b).**
  - Une ligne `uses:` sans étiquette en commentaire (`# vX.Y.Z`) est refusée.
  - La comparaison se fait entre le SHA épinglé et `gh api repos/<o>/<r>/commits/<étiquette>` → `sha`. Elle couvre les commits « imposteurs » venus d'un *fork*, car l'étiquette est résolue dans le dépôt officiel.
  - Si l'API ne répond pas, `merge` refuse.

**Risque résiduel, sans changement : R-ARCH-05.** Le §9 de ce carnet et les commentaires de PR sont écrits par le même compte GitHub. Le recoupement J4 prouve la **cohérence** entre deux traces, pas l'auteur. Le risque est accepté.

**Revue de la PR de T3a :** chemins sensibles `tools/xtask/` et `tools/dev/`, donc `/security-review`, deux relecteurs neufs successifs, puis mon `archsec-ok`. *Pourquoi deux relecteurs : c'est désormais l'unique barrière de fusion.* Critères bloquants : SEC0-12, 15, 17, 35, G3, G8, T3-a, T3-b et J1 à J8.

### 9.7 Consigne T4 : `cargo xtask spec-map` (30/10)

Avis **validé avec corrections H1 à H8**, toutes décidées. Lancement après la fusion de la PR #11. Le modèle Sonnet est accepté : la tâche n'est pas sensible en elle-même, mais elle vit dans `tools/xtask/`, d'où les points H4 à H6.

- **H1. Grammaire des renvois.** Les renvois déjà présents dans le dépôt (`tools/dev/`, `tools/xtask/`) n'entrent pas dans la grammaire initiale du §7 : renvois vers un carnet, listes séparées par `,`, plages `à`, préfixes repris, parenthèses. **Le §7 est révisé** (voir « Révision du 30/10 »), et c'est lui qui fait foi. `spec-map-spec.md` doit être recopié de nouveau depuis le §7 révisé.
- **H2. Identifiants.** On abandonne la liste fermée de lettres pour la règle générale : un identifiant est valide s'il est **défini** dans le fichier cible (§7 révisé). *Pourquoi :* les revues et le carnet emploient aussi `RS`, `C`, `R`, `SEC0-`, `AS-`, `T0-`, `V0-` et `R-ARCH-`. Une liste de lettres serait soit trop étroite, ce qui donnerait de faux refus, soit trop large, ce qui laisserait passer des renvois pendants.
- **H3. Renvois existants faux hors de `tools/xtask/`.** La consigne veut les signaler sans les corriger, mais `check` doit passer sur tout le dépôt. Les deux exigences se contredisent. Pour les concilier :
  - une liste fermée, `tools/xtask/src/spec_map/known_bad.txt`, donne pour chaque renvoi faux le fichier, la ligne, le texte du renvoi, la raison et l'*issue* `role:dev` ouverte ;
  - `check` ignore ces entrées-là et échoue sur tout autre renvoi faux ;
  - une entrée qui ne correspond plus à rien fait aussi échouer : la liste ne peut que **diminuer** ;
  - elle doit être **vide à la fin de la phase 0**, à la PR de consolidation.
- **H4. `diff <révision>`.**
  - La révision est vérifiée d'abord par `git rev-parse --verify --end-of-options <rév>^{commit}`.
  - Une valeur qui commence par `-` est refusée.
  - Aucune chaîne de l'utilisateur n'est passée à Git sans `--end-of-options`.
- **H5. Lecture des fichiers.** Ils sont lus par la liste de `Ctx::files()` de la PR #11 : fichiers ordinaires seulement, aucun lien suivi. La sortie, Markdown ou JSON, ne contient que des chemins relatifs avec `/`, jamais un chemin absolu ni un nom d'utilisateur (SEC0-26). Le schéma JSON est fixé et documenté dans `docs/dev/spec-map.md`.
- **H6. Effet sur `check`.** L'étape `spec-map` **s'ajoute** aux autres étapes : le diff ne doit modifier aucune autre étape de `check`. Si l'étape ne peut pas lire la conception ou `origin/pilotage`, elle **échoue**, sans jamais rien ignorer en silence.
- **H7. Commentaires RON.** RON se commente par `//`, pas par `#`. C'est une erreur de la version initiale du §7, corrigée dans la révision.
- **H8. Filtre par phase de `report` : reporté.** La conception ne porte pas la phase dans ses titres. Le filtre par fichier suffit en phase 0. Le filtre par phase sera ajouté quand le responsable qualité en aura besoin (organisation §3.1 point 4).

**Revue :** `tools/xtask/` est un chemin sensible, donc `/security-review`, un relecteur neuf, puis mon `archsec-ok`. Critères bloquants : H3 à H6, plus SEC0-26.

### 9.6 Consigne T13 : intégration continue (30/10)

Avis **validé avec corrections G1 à G12**, toutes décidées. Lancement après la fusion de T2b.

- **G1. Brouillons.** La condition `github.event.pull_request.draft == false` est fausse sur un `push` vers `main`, où `pull_request` est nul : tous les travaux y seraient sautés. Elle devient `github.event_name != 'pull_request' || !github.event.pull_request.draft`.
- **G2. Résultat de `ci-ok`.** Il lit `needs.<travail>.result` travail par travail.
  - `changes`, `pr-title` (sur une PR) et `security` doivent valoir **`success`** : un saut de l'un d'eux est un échec.
  - Seuls les travaux filtrés par chemins (`rust-linux`, `rust-windows`) peuvent être sautés.
  - *Pourquoi :* si `changes` échoue, tout ce qui en dépend est sauté, et un `ci-ok` qui accepte tout saut serait vert à tort.
- **G3. Origine de `ci-ok`.** Une PR peut ajouter, dans n'importe quel workflow, un travail nommé `ci-ok`. `.github/` est un chemin sensible, et mon avis couvre ce cas pour la PR elle-même. **Condition pour T3** (`xtask merge` et `origin-check`) : le contrôle `ci-ok` retenu doit venir du workflow `.github/workflows/ci.yml`, vérifié par le `check_suite` et le `workflow_run`. S'il en existe plusieurs, tous doivent être verts ; sinon, refus. `merge-pr.ps1` échoue déjà de façon sûre quand il en trouve deux. Point ajouté à l'*issue* #6.
- **G4. Auto-test de gitleaks.**
  - **Le faux secret :** il est fabriqué **à l'exécution**, par concaténation dans le script, au format d'une règle par défaut (par exemple un jeton `ghp_` suivi de 36 caractères aléatoires). Aucun fichier du dépôt ne contient de secret détectable. Il n'est jamais affiché : `--redact` vaut pour l'auto-test aussi.
  - **La plage de la PR :** les SHA de base et de tête passent par `env:`, jamais par `${{ }}` dans un `run:`, et `fetch-depth` couvre toute la plage.
- **G5. Une seule configuration gitleaks : pas de `.gitleaks.toml` à la racine** (retiré de la propriété de T13).
  - La CI utilise `tools/dev/hooks/gitleaks.toml`, **dans sa version de `origin/main`** (`git show`), et non dans celle de la PR.
  - Si la PR modifie ce fichier ou `tools.lock`, le travail `security` le signale dans son résumé.
  - *Pourquoi :* deux configurations divergent. Et une PR qui ajoute une exception ne doit pas pouvoir se valider elle-même.
- **G6. Épinglage pour Linux.**
  - **`tools/dev/tools.lock` :** T13 peut y ajouter des lignes `*.linux.*` (archive, SHA-256 de l'archive et du binaire), à la **même version** que sous Windows. C'est une extension de propriété, limitée à ces lignes.
  - **`cargo-deny` :** de préférence le binaire officiel, avec son SHA-256, pour éviter de compiler à chaque exécution.
  - **`cargo-audit` :** `cargo install --locked --version`, mis en cache par `actions/cache` avec une clé sur la version.
  - **Chaîne Rust :** elle est donnée par `rust-toolchain.toml` (la `rustup` de l'image), sans action tierce.
  - **Fuzzing :** une *nightly* épinglée par date.
- **G7. Une seule implémentation du contrôle des workflows : celle de `xtask`** (F10).
  - **Dans la CI :** le travail `security` lance `cargo xtask check` réduit à cette étape. Si T2b n'offre pas encore de sélection d'étape (`--only <étape>`), T13 le note dans son rapport, et c'est un ajout à `xtask`.
  - **Dans `tools/ci/` :** pas de second script de règles. Il ne contient que l'installation épinglée et le calcul des zones.
  - *Pourquoi :* deux implémentations des mêmes règles divergent.
- **G8. Liste des chemins sensibles : un fichier de données unique, `tools/ci/sensitive-paths.txt`**, créé par T13.
  - Il est lu par `changes`, puis par `xtask merge` et par `origin-check` (T3, *issue* #6, point 3).
  - Il reprend exactement le §5 de ce carnet, y compris :
    - les sections `[features]` et `[profile.*]` ;
    - `Cargo.lock` quand il ajoute une crate (ces deux cas sont détectés par un script) ;
    - les modules d'aléa et de profil de `ts_kernel`, dès que T5 les fixe.
- **G9. Tâche périodique.**
  - **Comparaison de `main` :** elle demande `actions: read`.
  - **Le travail qui ouvre l'*issue* :**
    - il n'a que `issues: write` ;
    - il ne fait **aucun `checkout`** ;
    - le texte de l'*issue* ne contient que des données du dépôt sur `main` : nom du travail, lien de l'exécution.
  - **Déclenchement manuel :** les éventuels `inputs` de `workflow_dispatch` ne passent jamais dans un `run:`.
- **G10. Cache et artefacts.**
  - **Cache :** `rust-cache` n'est enregistré que depuis `main` (`save-if`).
  - **Artefacts :** les artefacts téléversés (corpus de fuzzing) sont conservés 7 jours au plus. Ils ne contiennent jamais de sortie de gitleaks non masquée.
- **G11. Dependabot.**
  - **Volume :** mises à jour regroupées, rythme hebdomadaire, `open-pull-requests-limit: 2` par écosystème.
  - **Délai (`cooldown`) : `default-days: 7`.** *Pourquoi : on ne prend pas une version publiée depuis moins d'une semaine ; c'est la fenêtre où l'on découvre la plupart des versions piégées.*
  - **Titre :** `commit-message` réglé pour donner `build(deps): …`, afin que `pr-title` passe.
  - **Revue :** ses PR touchent `Cargo.lock` ou `.github/`, donc des chemins sensibles. Elles passent par mon avis comme les autres.
- **G12. Rapport à l'orchestrateur.**
  - Les réglages recommandés : *Allow select actions*, avec la liste fermée, et refus que les Actions créent ou approuvent des PR.
  - Le relevé des minutes de l'exécution verte.

**Revue de la PR T13 :** chemin sensible `.github/`, donc `/security-review`, un relecteur neuf, puis mon `archsec-ok`. Critères bloquants : SEC0-08 et SEC0-29 à 34, plus G1 à G11.

### 9.5 Consigne T2b : `xtask check`, `slots` et `disk` (30/10)

**Question posée : `serde_json` ou `cargo_metadata` ?** Ce sera **`serde_json`**, avec `serde` en dérivation (AS-17, décidé).
- *Pourquoi :*
  - `cargo_metadata` ajoute `semver`, `camino` et `cargo-platform` pour des champs dont `xtask` n'a pas besoin ;
  - il suit l'évolution du format de Cargo plus lentement que des structures locales réduites aux champs lus (`packages[].name`, `manifest_path`, `features`, `dependencies[].{name, kind, features, uses_default_features}`, `targets[].{kind, src_path}`).
- `serde_json` est du même auteur que `serde`, sous MIT ou Apache-2.0, et très répandu. Son `build.rs` ne fait que détecter des `cfg`.
- Les structures ignorent les champs inconnus, et l'option `--format-version 1` est fixée.
- La justification de SEC0-05 reste à écrire dans la PR.

Ma réponse : **validé avec corrections F1 à F10**, toutes décidées.

- **F1. Passe clippy avec `public`.** Elle porte sur le **graphe livré** seulement, sans `--all-targets` et sans `--workspace` : `cargo clippy -p <crates livrées> --locked --features ts_kernel/public -- -D warnings`. La liste des crates livrées est tenue dans `xtask` : `ts_kernel` aujourd'hui, puis `ts_host`, `ts_server`, `ts_client_native` et leurs dépendances. *Pourquoi :* SEC0-20 permet les fonctionnalités de développement dans les dépendances de développement et les crates d'outil. Avec `--workspace --all-targets`, elles s'unifient avec `public`, et le `compile_error!` ferait échouer la passe à tort.
- **F2. Étape gitleaks.**
  - **Binaire :** son SHA-256 est vérifié contre `tools.lock` avant usage.
  - **Configuration et options :**
    - la configuration est lue dans la copie épinglée désignée par `core.hooksPath`, et non dans la copie de travail ;
    - un `.gitleaksignore` à la racine est refusé ;
    - **`--redact` est obligatoire**. *Pourquoi : la sortie de `xtask check --all` est collée dans le corps des PR et lue par les sessions ; un secret trouvé ne doit jamais y apparaître en clair.*
  - **L'arbre :** ce sont les fichiers suivis, plus les fichiers non suivis et non ignorés (`git ls-files -co --exclude-standard`). Ils sont extraits comme dans le `pre-commit`, ce qui neutralise les attributs. Jamais `target/`, qui est une jonction, ni `TS_CACHE`.
  - **Les commits :** plage `origin/main..HEAD`, avec `--text --no-ext-diff --no-textconv`.
- **F3. `forbid(unsafe_code)`.** Les racines viennent de `cargo metadata` : chaque cible `lib`, `bin`, `proc-macro` et `custom-build` de chaque membre, y compris `tools/xtask`. Ce n'est pas une liste de chemins écrite à la main. En plus, chaque `Cargo.toml` membre doit contenir `[lints] workspace = true`, sans `[lints.rust]` local qui relâcherait `unsafe_code`. Exception de `ts_py`, conforme à SEC0-19 : `deny` au niveau de la crate, et chaque `allow` local précédé de `// SAFETY:`.
  - **Révision du 30/10 (AS-18, question m10).** `forbid` reste dans `[workspace.lints.rust]`. Les deux crates de la liste fermée de SEC0-19, `ts_py` et `ts_client_native`, **n'héritent pas** des lints de l'espace de travail : elles déclarent des tables `[lints.rust]` et `[lints.clippy]` locales.
    - Ces tables doivent être **identiques** à celles de l'espace de travail, à une seule différence près : `unsafe_code = "deny"`.
    - `xtask check` compare ces tables par analyse de lignes, refuse en cas de doute, et refuse ce `[lints]` local dans toute autre crate.
    - Ces deux crates portent `#![deny(unsafe_code)]`, et chaque `allow` local est précédé de `// SAFETY:`.
- **F4. Fonctionnalités : c'est la résolution de Cargo qui fait foi.**
  - **Hôtes :** pour chaque crate d'hôte, `cargo tree -p <hôte> --edges normal,features --locked -f '{p} {f}'`. L'étape échoue si une fonctionnalité de développement de `ts_kernel` y apparaît.
  - **Manifestes :** l'analyse par `cargo metadata` les complète. Elle refuse toute mention de `public` hors de sa déclaration dans `ts_kernel` : dans `features`, y compris les formes `ts_kernel/public` et `ts_kernel?/public`, et dans les `features` d'une dépendance. Elle vérifie aussi que `default` de `ts_kernel` est vide.
  - **`ts_toy` :** la règle D-DEV-02 est vérifiée par la même analyse.
  - *Pourquoi : réimplémenter l'unification de Cargo laisserait passer un cas, par exemple une fonctionnalité `default` d'une crate intermédiaire qui en active une autre.*
- **F5. Exemptions de clippy : la liste des formes refusées est élargie** dans `ts_kernel`, `ts_model`, `ts_data` et les crates de règles.
  - **Attributs refusés :** `allow` ou `expect` portant sur :
    - `clippy::disallowed_methods` et `clippy::disallowed_types` ;
    - **`clippy::style`**, le groupe qui contient ces deux lints ;
    - **`clippy::all`** ;
    - **`warnings`** ;
  - **Formes couvertes :** `#[...]`, `#![...]` et **`cfg_attr(…, allow(…))`**.
  - **Fichier :** tout `clippy.toml` ou `.clippy.toml` autre que celui de la racine est refusé. Un tel fichier, placé dans le dossier d'une crate, **remplacerait** la configuration de la racine.

  *Pourquoi :* chacune de ces formes rouvre l'horloge ou l'aléa dans le cœur, aussi sûrement qu'un `allow` nommé.
- **F6. Chemins locaux.**
  - **Motifs :** sans tenir compte de la casse, avec les deux séparateurs (`C:\Users\` et `C:/Users/`, `/home/`, `/Users/`), plus les valeurs de `USERNAME` et de `COMPUTERNAME` lues dans l'environnement.
  - **Exemptions :** les fichiers de test de `xtask` qui contiennent ces motifs, dans une liste fermée.
  - **Affichage :** les fichiers fautifs sont donnés par leur chemin et leur numéro de ligne, sans recopier la valeur trouvée.
- **F7. Crates touchées.**
  - Les **dépendants inverses** de chaque crate touchée sont testés aussi : une modification de `ts_kernel` teste tout ce qui en dépend.
  - Une modification de `Cargo.toml` racine, `Cargo.lock`, `clippy.toml`, `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml` ou `.cargo/` vaut `--all`.
  - Les étapes 4, 6, 7 et 9 à 12 sont des contrôles de sécurité peu coûteux. Elles portent **toujours sur tout le dépôt**, quel que soit le périmètre.
- **F8. `disk --clean`.**
  - **Jonctions :** il ne suit jamais une jonction ni un lien symbolique. Le code teste explicitement les points de réanalyse avant de descendre dans un dossier ; `remove_dir_all` seul ne suffit pas.
  - **Périmètre :** il ne supprime que sous `TS_CACHE` et sous `.claude/worktrees/` de la copie principale, après mise sous forme canonique du chemin et vérification de son préfixe.
  - **Dossiers laissés par `git worktree remove` :** seulement ceux qui sont absents de `git worktree list`, sans fichier `.git`, et sans modification depuis 24 h.
  - **Emplacements :** avant de vider un emplacement, il prend le bail par le même mécanisme atomique que `session-start.ps1`, et il le rend ensuite.
- **F9. Versions des outils.** Avant usage, `cargo deny --version` et `cargo audit --version` sont comparées à `tools.lock`. Un écart fait échouer l'étape (SEC0-06).
- **F10. Workflows : l'étape 12 applique SEC0-29 à 31 en entier.**
  - **Actions :** épinglées par SHA de 40 caractères et prises dans la **liste fermée** de SEC0-30.
  - **Permissions :** `permissions: {}` au niveau du workflow.
  - **Interdits :**
    - `pull_request_target`, `workflow_run` et `secrets: inherit` ;
    - `actions/checkout` sans `persist-credentials: false`.
  - **Interpolations :** aucune de `github.event.*`, `github.head_ref` ni `inputs.*` dans un `run:`.
  - **Méthode :** une analyse par lignes et par indentation, sans nouvelle dépendance YAML, qui **refuse en cas de doute**.

**Découpage : deux PR d'emblée.** La première, `claude/xtask-check`, porte `check`. C'est un chemin sensible, avec revue complète. La seconde, `claude/xtask-disk`, porte `slots` et `disk`. *Pourquoi :* les 13 étapes, avec F1 à F10, dépassent à elles seules les 400 lignes. Et `disk --clean` supprime des fichiers : il mérite sa propre revue.

**Revue :** `tools/xtask/` est un chemin sensible. Les deux PR passent donc par `/security-review`, un relecteur neuf, puis mon `archsec-ok`. Pour la première, les critères bloquants sont SEC0-03, 06, 08, 19, 20, 26, 30 et 31, plus F1 à F7, F9 et F10. Pour la seconde, c'est F8.

### 9.4 Consigne T2a : espace de travail Cargo (30/10)

Le périmètre est juste : aucune dépendance, `tools/xtask` laissé à T2b, et `public` déclarée seulement dans `ts_kernel`. La décision sur le membre manquant est confirmée : Cargo refuse un membre explicite absent, donc T2b ajoute `tools/xtask` à `members`. Le motif `crates/*` ne le couvre pas.

Corrections à intégrer à la consigne avant le lancement. Elles sont toutes décidées :

- **E1. Groupe de lints clippy.** Écrire `all = { level = "warn", priority = -1 }`, sinon le lint `clippy::lint_groups_priority`, refusé par défaut, fait échouer clippy. Dans `clippy.toml`, mettre `allow-unwrap-in-tests = true` et `allow-expect-in-tests = true`. *Pourquoi : avec `--all-targets -D warnings`, chaque `unwrap` de test échouerait ; ce serait un faux refus.*
- **E2. `overflow-checks = true` dans tous les profils**, `release` et `sim` compris.
  - *Pourquoi :* un dépassement d'entier sur l'argent en centimes ou sur un compteur doit devenir une panique, et jamais un enroulement silencieux. Un enroulement est une création d'argent exploitable, et il diverge entre `dev` et `release`.
  - Le type `Money` de T5 utilisera de toute façon une arithmétique vérifiée. Ce réglage est le filet.
  - `panic` reste `unwind`.
- **E3. Aucune construction avec `--all-features`**, ni en CI, ni dans `xtask`, ni dans la documentation : elle active `public` avec les fonctions de développement et déclenche le `compile_error!`. Les combinaisons sont testées une à une. À écrire dans `docs/dev/espace-travail.md`.
- **E4. `deny.toml` au format de cargo-deny v2** :
  - `[graph] all-features = true`, pour que les dépendances qu'on n'atteint que par une fonctionnalité de développement soient aussi contrôlées ;
  - `[advisories]` :
    - les crates non maintenues sont refusées (`unmaintained = "all"`) ;
    - chaque entrée de `ignore` porte l'identifiant, une raison et une date ;
    - elle est revue à l'audit hebdomadaire (SEC0-04). C'est plus strict que l'« alerte » de SEC0-02, que la v2 n'offre plus.
  - `[sources]` : `allow-registry` limité à crates.io et `allow-git = []`.
  - `[bans]` :
    - `wildcards = "deny"`, avec `allow-wildcard-paths = true` pour les crates internes ;
    - ajouter `openssl-sys` à la liste refusée.
  - `[licenses]` : `confidence-threshold = 0.93`, aucune `exceptions`.
- **E5. `clippy.toml`, interdictions complétées** :
  - aléa :
    - `rand::rng`, le nouveau nom de `thread_rng` depuis rand 0.9 ;
    - `getrandom::fill` et `getrandom::getrandom` ;
    - `std::collections::hash_map::RandomState::new` ;
  - horloge : `std::time::Instant::elapsed` et `std::time::SystemTime::elapsed`, qui lisent l'horloge ;
  - environnement : `std::env::vars` ;
  - E/S, interdites dans le cœur par l'architecture §9.2 : `std::fs::read`, `std::fs::read_to_string`, `std::fs::write`, `std::fs::File::open` et `std::fs::File::create` ;
  - types : `hashbrown::HashMap` et `hashbrown::HashSet`, en plus de ceux de `std`.

  Les chemins de crates absentes du graphe (`rand`, `getrandom`, `hashbrown`) portent `allow-invalid = true` si clippy 1.99 l'accepte. Sinon, ils sont ajoutés avec la crate. T5 pourra autoriser explicitement la table à hachage déterministe qu'il choisira.
- **E6. Exemptions locales.** `#[allow(clippy::disallowed_methods)]` et `#[allow(clippy::disallowed_types)]` (ou `#[expect]`) restent permis dans les crates d'hôte et d'outil, avec un commentaire. **Condition pour T2b** : `xtask check` refuse toute exemption de ce type dans `ts_kernel`, `ts_model`, `ts_data` et les crates de règles. La liste fermée de ces crates est tenue dans `xtask`. *Pourquoi : sans contrôle, une seule ligne `allow` rouvre l'horloge ou l'aléa dans le cœur.*
- **E7. `.cargo/config.toml` et `Cargo.toml` racine** :
  - aucune section `[source]`, `[registries]`, `[patch]` ni `[replace]`, aucun `linker` et aucune `rustflags`. *Pourquoi : ce sont des contournements de SEC0-02.*
  - `Cargo.lock` est commité et n'est pas ignoré (SEC0-01).
- **E8. Preuve du `compile_error!`.** Pour chacune des quatre combinaisons `public,<f>`, la sortie collée dans la PR montre **notre message**, pas seulement un échec. *Pourquoi : un échec d'une autre cause ferait réussir la preuve à tort.*
- **E9. `BUILD_PROFILE` vaut `"public"` si et seulement si `public` est active.** Sans fonctionnalité, il vaut `"dev"`. Le futur refus de démarrer en ligne compare la valeur à `"public"` : il reste donc fermé par défaut. Le test unitaire le vérifie.

**Note pour plus tard, sans effet sur T2a.** `xtask release` (phase 3 ou 4) devra retirer les chemins de la machine de construction du binaire public, par `--remap-path-prefix`. Ces chemins ne vont pas dans `.cargo/config.toml`. *Pourquoi : SEC0-26 et S-journal, aucun nom d'utilisateur dans un artefact livré.*

Revue de la PR T2a : chemins sensibles, donc `/security-review`, relecteur neuf, puis mon `archsec-ok: <sha>`. Critères bloquants : SEC0-02, 07, 19, 20 et 22, et les corrections E1 à E9.

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
| 30/10 | AS-17 | `serde_json` (avec `serde`) ajouté à la liste pré-approuvée de SEC0-05, **pour les crates d'outil seulement** ; `cargo_metadata` écarté | Moins de crates dans le graphe ; des structures locales réduites aux champs lus ; aucun décodage JSON dans le cœur ni dans le protocole |
| 30/10 | AS-18 | Question m10 : `unsafe_code = "forbid"` **reste** dans `[workspace.lints.rust]`. `ts_py` et `ts_client_native`, seules exceptions de SEC0-19, déclarent des tables de lints locales identiques à celles de l'espace de travail, sauf `unsafe_code = "deny"` ; `xtask check` vérifie cette égalité. La proposition de passer l'espace de travail en `deny`, avec `#![forbid]` dans chaque crate, est écartée | Le refus reste **fermé par défaut et garanti par le compilateur** : une crate nouvelle, ou un attribut oublié, ne peut pas compiler d'`unsafe`, même si `xtask` n'a pas tourné. L'exception est nommée dans deux manifestes, et non obtenue par l'absence d'un attribut. La recopie de la table est bornée à deux crates, et contrôlée |
| 30/10 | AS-19 | `spec-map` : un point numéroté de premier niveau sous `§k` définit `§k.N`, sauf si un vrai sous-titre `k.N` existe ; deux définitions du même `§k.N` rendent le renvoi ambigu, et `check` échoue (proposition du responsable développement, retenue) | La conception cite déjà « 05 §2.3 », « §2.10 » et « §2.22 » pour des points numérotés, et ne renumérote jamais ; un doublon doit rester un refus |
| 30/10 | AS-20 | `spec-map` : un identifiant cité sans section est valide s'il est défini plusieurs fois dans le fichier cible ; `report` signale les définitions multiples, et `check` n'échoue pas | Une synthèse qui reprend le détail n'est pas une faute. Exiger une définition unique donnerait de faux refus sur des documents que le développement ne possède pas. Le doute est rendu visible à la conception, sans bloquer le code |
| 30/10 | AS-21 | SEC0-30 révisé : la liste fermée d'actions n'est pas réglée côté dépôt (cela demanderait un `gh api -X PUT`, que `guard-git` refuse) ; elle est compensée par l'étape « workflows » de `xtask check`, lancée en local et en CI | Décision de l'orchestrateur, qui ne contourne pas le garde. Le contrôle du code couvre les mêmes règles ; seule la défense côté serveur manque (R-ARCH-06) |
| 30/10 | AS-22 | **Glissement de l'origine des `SimDay`** (exigence de l'arbitrage A12 de la conception, PR #22). Détail :<br>(1) **Valeur :** une ressource `DayOrigin(i64)`, en jours absolus depuis l'époque du monde, initialisée à 0 et **enregistrée dans l'instantané**. `SimDay(i16)` vaut `jour_absolu − origine`, saturé dans `i16::MIN..=i16::MAX`. Le jour absolu vaut `SimTime.div_euclid(86 400 000)`.<br>(2) **Règle de glissement, en jours entiers, sans calendrier :** au changement de jour, dans la phase fixe de fin de journée, si `jour_courant − origine ≥ 21 900` (60 × 365), l'origine avance de `10 950` (30 × 365). Le jour courant redevient ainsi relatif à environ 30 ans, avec une marge sous `i16::MAX` (32 767), et rien ne change pour les dates récentes.<br>(3) **Réécriture :** au même instant, toutes les `SimDay` stockées sont réécrites par `rebase(d, pas) = d.saturating_sub(pas)`. Elles gardent ainsi la même saturation : une date de plus de 89 ans vaut le minimum.<br>(4) **Couverture :** tout composant ou ressource qui contient une `SimDay` implémente le trait `RebaseDays` et s'enregistre dans `SimBuilder`. Un test vérifie que chaque type enregistré dans l'instantané qui contient une `SimDay` est couvert.<br>(5) **Phases :**<br>– **phase 0 (T5a) :** `SimDay(i16)`, `DayOrigin`, la conversion saturée, la fonction pure `rebase`, et les tests (bornes, négatifs, saturation, et glissement simulé sur 200 ans de jours) ;<br>– **phase 1 (premiers composants datés, relations) :** le système de glissement, le trait `RebaseDays` et son test de couverture ;<br>– **chaque format d'instantané** porte `DayOrigin` dès sa création (T8) | Les règles ne lisent que l'écart avec le jour courant (A12), et cet écart ne change pas quand l'origine glisse, sauf pour les valeurs déjà au minimum. Le glissement est rare, une fois tous les 30 ans de jeu, déterministe et compris dans l'instantané. Une origine fixe en `i32` coûterait 2 octets de plus par date, soit environ 26 Mo selon M9. Les constantes sont **techniques** et non des valeurs de jeu, donc pas de `PROVISOIRE`. À reporter dans 05 §2.5 et dans architecture §6.2 à la consolidation de fin de phase 0 |
| 30/10 | AS-23 | **Quatre écarts de la PR #23 (T3a, `merge` et `hooks`), tranchés :**<br>(1) **J2, date de l'exécutable.** Accepté : on la compare au dernier commit de `main` qui touche une **entrée de construction** de `xtask`, soit `tools/xtask/`, `Cargo.toml` racine, `Cargo.lock`, `rust-toolchain.toml` et `.cargo/`. La liste s'étend donc au-delà de `tools/xtask/` et de `rust-toolchain.toml`.<br>(2) **J2, `--dry-run` depuis un worktree.** Accepté, à condition qu'un test montre qu'en `--dry-run`, `gh pr merge` n'est **jamais** appelé et que la copie principale n'est pas modifiée.<br>(3) **Approbations non filtrées par auteur.** Accepté : c'est le risque R-ARCH-05, car un seul compte GitHub est en jeu. Le recoupement de J4 avec le §9 reste la parade.<br>(4) **`[patch]`, `[replace]` et les sources hors de crates.io dans un `Cargo.toml`.** Ils sont **rendus sensibles**. Ce n'est pas une ligne de `sensitive-paths.txt`, qui ne liste que des chemins : c'est une règle **de contenu** dans `merge`, à côté de celles de `[features]` et `[profile.*]`. Un fragment de diff d'un `Cargo.toml` qui touche `[patch`, `[replace`, `git =`, `registry =`, `branch =`, `tag =` ou `rev =` rend la PR sensible. En plus, **`xtask check` refuse** toute section `[patch]` ou `[replace]` dans un `Cargo.toml` de l'espace de travail : c'est le contrôle automatique d'E7. Il entre dans la #23 si c'est court, sinon dans T3b | (1) Le `cargo run` de l'alias recompile dès qu'une entrée change ; la date sert de défense en profondeur. (2) Une simulation sans écriture ne contourne rien. (4) cargo-deny refuse les sources `git`, mais pas un `[patch]` vers un chemin local : un tel `[patch]` remplacerait sans bruit une crate de crates.io par du code non audité |
| 30/10 | AS-24 | K10 précisé : `blake3` en `features = ["pure"]` est accepté même si son `build.rs` sonde `cl.exe` sur un `main` vide. Aucun code C ni assembleur n'est compilé dans le binaire ni lié | L'objectif de K10, un exécutable sans code C ni SIMD externe, est atteint. La sonde vient d'une crate épinglée et contrôlée par `cargo deny`. Sans compilateur C, la construction passe |
| 30/10 | AS-25 | **Règle de langue de l'utilisateur, transmise par l'orchestrateur.**<br>– **En anglais :** le code et ses commentaires, les messages de commit (y compris ceux de ce carnet), les *issues*, les commentaires de PR et mes avis publiés sur GitHub (`archsec-ok` et verdicts).<br>– **Étiquettes renommées :** `role:design`, `role:qa`, `role:content` et `deviation`.<br>– **Marque provisoire :** `PROVISIONAL(design #<n>)`.<br>– **En français :** ce carnet, `05-architecture.md`, les revues et la communication avec l'utilisateur.<br>– **Mes conditions sur les PR de traduction du développement** (`.claude/`, `.github/`, `tools/`) :<br>(1) une traduction ne change **aucun comportement** : la PR le montre par des tests identiques avant et après ;<br>(2) les messages des `compile_error!` changent **en même temps** que la recherche de l'étape `profiles` de `xtask`, dans la **même** PR ;<br>(3) `spec-map` accepte `PROVISOIRE(conception #n)` et `PROVISIONAL(design #n)` pendant la transition ;<br>(4) le renommage des étiquettes ne passe que par `gh label edit`, déjà dans la liste blanche, sans `gh api` en écriture | Décision de l'utilisateur. Mes conditions visent à ce qu'une traduction ne désarme aucune barrière : un message de refus changé d'un côté seulement ferait échouer la preuve E8 ou, pire, la rendrait muette |
| 30/10 | AS-26 | **Décision de l'utilisateur, transmise par le responsable développement** : plus de relecteur indépendant sur la PR #23, ni sur T6 (#27, #28). La fusion se fait dès que la CI est verte. Pour un chemin sensible, mon `archsec-ok` reste exigé par `merge`. Je le fonde alors sur **ma propre relecture** du SHA vert, ciblée sur les critères bloquants de la consigne. Le `review-ok` posé par le responsable développement est déclaré non indépendant | Décision de l'utilisateur, qui prime. Je garde la seule barrière qui reste, mon avis, et je l'exerce moi-même. Le risque ajouté est R-ARCH-07 |
| 30/10 | AS-27 | **Contrat de reprise des greffons**, après le constat F1 sur la PR #30. Il complète l'architecture §9.2, et sera reporté là à la consolidation de fin de phase 0.<br>(1) **Règle.** Entre deux ticks, un greffon de règles ne garde **aucun état hors de l'état déclaré** (`state_component` / `state_resource` ; entités `StableEntity`). En particulier, dans les crates de règles :<br>– pas de `Local<T>` qui porte de l'état d'un tick à l'autre ;<br>– pas de logique de jeu fondée sur `Added` ni `Changed`, car un système neuf voit tout comme modifié ;<br>– tout `Message` est écrit et lu **dans le même tick**, avec un ordre de phases explicite ;<br>– pas de ressource ni d'entité de jeu non déclarée.<br>Un cache reconstructible est déclaré `transient_component`, et sa reconstruction ne change aucun résultat.<br>(2) **Contrôle automatique.** C'est une **condition bloquante avant la fusion du premier greffon de règles** (phase 1), au même titre que l'*issue* #26 :<br>– un test de reprise générique, obligatoire pour chaque greffon : instantané au tick k, restauration, N ticks, ce qui doit donner la même empreinte que k + N ticks d'affilée, pour plusieurs k > 0 ;<br>– les interdictions de `clippy.toml` sur `bevy_ecs::system::Local`, `bevy_ecs::query::Added` et `bevy_ecs::query::Changed` dans les crates de règles. Il n'y a pas d'exemption locale possible (E6) ; un besoin réel passe par une décision ici.<br>(3) **Phase 0.** La règle est documentée dans `Plugin` et `Snapshot` (PR #30), et le suivi se fait dans l'*issue* #31 | Sans ce contrat, une reprise après panne (architecture §12.4) ou une restauration donnerait un monde qui diverge en silence. L'empreinte reste égale au moment de la restauration et ne la détecte pas : seul le test de reprise sur plusieurs ticks la voit. La reproductibilité R1 est une exigence de l'architecture, et le déterminisme protège aussi la relecture des exploits |
| 30/10 | AS-28 | **GitHub Actions arrêté** (paiement refusé ou plafond de dépenses du compte).<br>(1) **Fusions :** aucune tant que `ci-ok` manque, et aucun assouplissement de `cargo xtask merge` (décision du responsable développement, approuvée).<br>(2) **Runner auto-hébergé : avis défavorable pour l'instant.** Ce serait un service persistant installé sur le poste de l'utilisateur, ce qui change la configuration du système et relève donc de l'utilisateur. Ce service exécuterait le code de chaque PR, workflows modifiés compris, avec les droits du compte et l'accès à son profil (jeton `gh`, `~/.claude`, `TS_CACHE`).<br>(3) **Si l'utilisateur le décide un jour, conditions à respecter :**<br>– un compte Windows local dédié, sans privilège et sans accès au profil de l'utilisateur ;<br>– un runner `--ephemeral`, attaché à ce seul dépôt privé, avec une étiquette dédiée ;<br>– des jobs dans un dossier jetable ;<br>– aucun secret sur la machine du runner ;<br>– le déclencheur `pull_request` pour les branches du dépôt seulement ;<br>– une PR `.github/` qui passe par mon avis.<br>(4) **Recommandation :** l'utilisateur rétablit la facturation ou le plafond. C'est la seule action qui reste de son ressort | Un runner sur le poste annulerait l'isolement que donne aujourd'hui l'exécuteur hébergé par GitHub. Attendre ne coûte que du délai |
| 30/10 | AS-15 | Le crochet Claude Code passe au `.claude/settings.json` du projet, avec la parade SEC0-35 ; écart assumé à la clôture C4, à reporter à la consolidation | Les réglages utilisateur appartiennent à l'utilisateur (décision de l'orchestrateur) |

| N° | Risque | Parade |
|---|---|---|
| R-ARCH-01 | Aucune copie du dépôt hors du poste en mode local | **Levé le 30/10** : dépôt sur GitHub |
| R-ARCH-02 | Le fuzzing de repli, sans guidage par la couverture, trouve moins | **Sans objet** depuis le 30/10 : fuzzing guidé en CI |
| R-ARCH-03 | Pas de *ruleset* : une poussée sur `main` hors de Claude Code est seulement détectée après coup | SEC0-17 ; à revoir si l'offre change (décision de l'utilisateur, jamais demandée) |
| R-ARCH-04 | Une session peut retirer le crochet de son worktree entre deux fusions | Les crochets Git épinglés hors du dépôt bloquent encore le commit et la poussée vers `main` ; SEC0-35 détecte l'écart à la fusion suivante ; `ci-ok` et `origin-check` restent en place |
| R-ARCH-05 | Un script écrit puis exécuté par une session (`bash x.sh`, `pwsh x.ps1`) échappe à `guard-git` et peut lancer `git push --no-verify` vers `main` ; et les approbations `archsec-ok` / `review-ok` ne sont pas authentifiées (un seul compte GitHub) | Aucun garde PreToolUse ne voit l'intérieur d'un script : détection après coup par `origin-check` (SEC0-17), qui recoupe aussi chaque `archsec-ok` avec une ligne du §9 de ce carnet ([#6](https://github.com/CreatorInstant548/time_switcher/issues/6)) ; T3 prioritaire |
| R-ARCH-06 | Sans liste fermée côté dépôt (AS-21), une PR qui modifie à la fois un workflow et le contrôle de `xtask` pourrait faire tourner une action hors liste lors de sa propre exécution de CI | Le jeton est en lecture seule et le dépôt n'a aucun secret : une action hors liste ne peut que lire le code de la PR. `.github/` et `tools/xtask/` sont des chemins sensibles, qui demandent mon avis avant la fusion. Le réglage côté dépôt sera recommandé de nouveau si un secret de dépôt devient nécessaire (phase 4) |
| R-ARCH-07 | Sans relecteur indépendant (AS-26), un défaut que ma relecture ciblée ne voit pas peut atteindre `main` | La CI, `xtask check`, les tests de chaque PR et mon avis, avec une relecture ciblée sur les critères bloquants, restent en place. L'audit de fin de phase 0 (SEC0-28) est confié à une session experte neuve et reprend en entier les PR fusionnées sans relecteur indépendant (#23, puis les suivantes). Le risque est accepté par décision de l'utilisateur |

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
