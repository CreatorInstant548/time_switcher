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
| 2. Validation du découpage de la phase 0 | Grille prête (§6) ; **en attente du découpage** du responsable développement |
| 3. Demande de la direction artistique (§12.9) | **Faite** (§8) ; transmise au responsable conception |
| 4. `cargo xtask spec-map` dans le découpage | **Spécifiée** (§7), à inscrire au découpage |

## 3. Exigences de sécurité bloquantes de la phase 0 (décidé)

Source : [securite.md](../time_switcher/docs/conception/revues/securite.md) §7 (phase 0, points 1 à 6), [developpement-agents.md](../time_switcher/docs/conception/revues/developpement-agents.md) §12.5 (point 7), l'en-tête de 05 (« Clôture de la conception », barrières locales) et [generation.md](../time_switcher/docs/conception/revues/generation.md) §12, **adaptés au mode local** (`CLAUDE.md`) : pas de dépôt distant, donc ni CI, ni Dependabot, ni *ruleset*. La CI est remplacée par `cargo xtask check`, et la protection de `main` par les barrières locales et `cargo xtask merge`.

Chaque case est cochée par moi, sur preuve (sortie de commande notée dans le carnet du responsable développement, ou relecture). La phase 0 n'est pas terminée tant qu'une case reste vide.

### 3.1 Chaîne d'approvisionnement

- [ ] **SEC0-01** — `Cargo.lock` et `uv.lock` versionnés ; `--locked` sur toutes les commandes `cargo` de `xtask` ; `uv sync --locked` (S26).
- [ ] **SEC0-02** — `deny.toml` :
  - `advisories` : vulnérabilité et version retirée (*yanked*) refusées ; crate non maintenue en alerte, examinée à l'audit hebdomadaire (SEC0-04) ;
  - `sources` : crates.io seulement ; aucun dépôt git, aucun autre registre ;
  - `licenses` : liste blanche MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0, Unicode-DFS-2016, CC0-1.0, MPL-2.0, BSL-1.0 ; **GPL, LGPL et AGPL refusées partout** (décidé : le client contient le serveur et presque toutes les crates ; un copyleft fort dans le graphe imposerait d'ouvrir le jeu) ;
  - `bans` : crates réseau (`reqwest`, `ureq`, `hyper`, `tokio`, `quinn`, `openssl`, `native-tls`) refusées dans tout l'espace de travail en phase 0. Elles reviendront en phase 4, avec la liste des crates autorisées à les utiliser (`wrappers`).
- [ ] **SEC0-03** — `cargo xtask check` lance `cargo deny check` et `cargo audit`, et échoue si l'un échoue. Si la base RustSec ne peut pas être mise à jour, le contrôle échoue, sauf si la copie locale a moins de 7 jours (décidé : jamais de réussite silencieuse sans base).
- [ ] **SEC0-04** — Tant que le mode local dure, `cargo xtask audit` (même contenu, sur `main`) tourne **chaque semaine**, lancé par le responsable développement, avec le résultat noté dans son carnet. Il remplace Dependabot et le `cargo-audit` nocturne.
- [ ] **SEC0-05** — Toute nouvelle dépendance directe est justifiée dans la demande de fusion : besoin, alternative écartée, licence, présence d'un `build.rs` ou d'une macro procédurale. *Pourquoi : ces deux mécanismes exécutent du code sur le poste à la compilation.* Liste **pré-approuvée** pour la phase 0 : `bevy_ecs` (sans `bevy_app`), `serde`, `ron`, `blake3`, `thiserror`, `smallvec`, `arrayvec`, `chrono`, `chrono-tz`, `arrow`, `parquet`, `pyo3`, `numpy`, `rerun` (`default-features = false`), `proptest`, `arbitrary`, `libfuzzer-sys`, `clap`, `xshell` ou équivalent pour `xtask`. Une dépendance de **cryptographie, de décodage de format externe ou de réseau** hors de cette liste demande mon avis **avant** la fusion.
- [ ] **SEC0-06** — Outils tiers épinglés : `cargo install --locked --version <v>` pour `cargo-deny`, `cargo-audit`, `cargo-fuzz` et `maturin` ; `gitleaks` en binaire officiel, avec son SHA-256 consigné dans le dépôt (`tools/dev/tools.lock` ou équivalent) et vérifié avant usage.
- [ ] **SEC0-07** — Chaîne Rust épinglée par `rust-toolchain.toml` (1.99.1, A14).

### 3.2 Secrets et historique

- [ ] **SEC0-08** — `gitleaks` :
  - en `pre-commit`, sur l'index, **sur toutes les branches**, y compris `pilotage` ;
  - dans `xtask merge`, sur toute la plage `main..<branche>` avant la fusion ;
  - dans `xtask check`.

  Un secret détecté est tenu pour fuité et **remplacé**, même sans dépôt distant.
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
- [ ] **SEC0-13** — **Aucun commit sur `main` hors de `xtask merge`.**
  - `xtask merge`, une fois ses contrôles passés, écrit une autorisation à usage unique dans le répertoire Git commun : l'empreinte d'arbre de l'index (`git write-tree`).
  - Le `pre-commit` sur `main` refuse tout commit dont l'arbre ne correspond pas à cette autorisation, puis l'efface.

  *Pourquoi : l'autorisation est liée au contenu exact qui a été vérifié, et pas seulement à l'outil.* (Décidé.)
- [ ] **SEC0-14** — Crochet Claude Code dans les **réglages utilisateur** (`~/.claude/settings.json`), hors du dépôt (clôture C4). Il refuse :
  - `git commit` sur `main` ;
  - `git push` vers `main` (`HEAD:main`, `refs/heads/main`), une destination préfixée par `+`, et `--force` sous toutes ses formes ;
  - `--no-verify`, `-c core.hooksPath`, `git config core.hooksPath` ;
  - `git update-ref refs/heads/main`, `git branch -f main` ;
  - `gh api` sur `git/refs` ;
  - `Edit` et `Write` dans la copie principale (A19).

  Le worktree `pilotage` est un autre chemin et reste permis.
- [ ] **SEC0-15** — `cargo xtask merge <branche>` (mode local), dans cet ordre :
  1. la copie principale est propre et sur `main` ;
  2. la branche est à jour de `main`, donc fusionnable en avance rapide ;
  3. une **attestation de `xtask check`** existe pour l'**arbre exact** de la branche (`TS_CACHE\attest\<arbre>.json` : chaîne d'outils, commandes, résultats, date) ;
  4. pour un chemin sensible (§5), l'attestation porte aussi la `/security-review`, la relecture d'une session ou d'un sous-agent **distinct de l'auteur**, et **mon avis favorable** sur ce même arbre ;
  5. vérification de l'origine (SEC0-17) et des empreintes des crochets (SEC0-12) ;
  6. `gitleaks` sur la plage (SEC0-08) ;
  7. `git merge --squash`, puis commit.

  *Pourquoi : un arbre rebasé sur un `main` qui a bougé change d'empreinte, ce qui force un nouveau contrôle, comme la règle « branche à jour » de GitHub.*
- [ ] **SEC0-16** — Message de squash :
  - titre conventionnel (A31) ;
  - pieds `Merge-Request: <identifiant au carnet du responsable développement>`, `Checked-Tree: <empreinte>` et `Co-Authored-By` ;
  - chaque fusion ajoute une ligne au **registre des fusions** `TS_CACHE\merge\ledger.jsonl`, **chaîné** (chaque ligne porte l'empreinte de la précédente).
- [ ] **SEC0-17** — **Contrôle de l'origine** : `cargo xtask verify-main`, script épinglé par empreinte, lancé depuis la copie principale.
  - Il vérifie que chaque commit de `main` postérieur à la base (les commits de documentation antérieurs à l'outillage, listés une fois pour toutes) porte les pieds de SEC0-16, et figure au registre avec le même arbre.
  - Il tourne au début de chaque `xtask merge` et chaque semaine avec SEC0-04 (pas de tâche de nuit sans CI).
  - Une anomalie suspend les fusions jusqu'à explication dans ce carnet.
- [ ] **SEC0-18** — **Test des barrières** (`cargo xtask test-barriers`), dans un dépôt jetable sous `TS_CACHE\tmp\`. Il vérifie que :
  - le crochet Claude Code refuse chaque variante de SEC0-14 ;
  - un commit sur `main` sans autorisation est refusé ;
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
  - `clippy.toml` interdit dans le noyau et les crates de règles : horloge murale (`SystemTime::now`, `Instant::now`), aléa du système et `rand::thread_rng`.
- [ ] **SEC0-23** — **Codec d'instantané borné**, dans une crate `ts_persist` créée en phase 0, réduite au codec (décidé, §6, V0-06). Il borne :
  - la taille totale, le nombre d'éléments, la profondeur et la longueur des chaînes ;
  - avec un en-tête de version.

  Il ne panique sur aucune entrée : `clippy::unwrap_used`, `expect_used`, `indexing_slicing` et `panic` sont refusés dans le module ; ses erreurs sont typées. Il produit des empreintes par morceau, avec un **arbre BLAKE3 et un manifeste** (format de parts adressées, P30, S23).
- [ ] **SEC0-24** — **Cible de fuzzing** du décodeur d'instantané (`fuzz/`, hors de l'espace de travail ; corpus dans `TS_CACHE`). Critère : **1 h sans plantage**, lancé localement depuis la copie principale.
  - Exception à A14 (décidé) : une chaîne *nightly* datée et épinglée, installée une seule fois, pour le fuzzing seulement.
  - Repli si `cargo-fuzz` ne fonctionne pas sous Windows MSVC : `xtask fuzz --stable` (entrées tirées par `arbitrary`, sans guidage par la couverture), pendant 1 h, avec le risque résiduel noté ici ; le vrai fuzzing est refait dès que la CI Linux existe.
- [ ] **SEC0-25** — Chargement RON de `ts_data` borné (taille de fichier, profondeur), avec des erreurs typées et sans panique. *Pourquoi : en solo, le paquet est une entrée que le joueur peut modifier ; un crash est sans enjeu, mais le même chargeur sert au serveur.*
- [ ] **SEC0-26** — Aucun chemin absolu, nom d'utilisateur ou nom de machine dans les fichiers commis : empreintes dorées, manifestes, `WorldSpec`. Un contrôle dans `xtask check`.

### 3.5 Preuves et audit (critère de fin)

- [ ] **SEC0-27** — **Preuves de blocage**, avec leurs sorties notées dans le carnet du responsable développement :
  1. une branche jetable qui ajoute une version de crate frappée d'un avis RustSec est refusée par `xtask check` et par `xtask merge` ;
  2. un faux secret, au format reconnu par `gitleaks` et jamais une vraie clé, est refusé par le `pre-commit` ;
  3. le même faux secret, commis **sans crochet** dans un dépôt jetable sous `TS_CACHE\tmp\`, est refusé par le contrôle de plage de `xtask merge`.

  Les branches sont ensuite supprimées.
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
| 7 (A26, A29) | SEC0-12 à 18 |

### 3.6 Ce que le mode local ne couvre pas (risques acceptés jusqu'au dépôt distant)

- Pas de compilation ni de test sous Linux : le déterminisme n'est vérifié que sous Windows. Le socle n'exige pas l'égalité entre plateformes ; la première CI Linux pourra révéler des erreurs de compilation.
- Pas de protection côté serveur : toutes les barrières sont locales et contournables par qui contrôle le poste.
- Pas d'alertes Dependabot : remplacées par l'audit hebdomadaire (SEC0-04).
- Pas de copie hors du poste : une panne de disque perd le dépôt. Je le signale à l'orchestrateur ; ce n'est pas une exigence de sécurité de la phase 0, mais un risque de disponibilité (**R-ARCH-01**, §10).

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

**Risque résiduel accepté** : tout vit sur un seul compte Windows. Jetons, crochets, attestations et registre peuvent être forgés par qui contrôle le poste. Les barrières protègent de l'**erreur** et de l'**injection**, pas de la compromission de la machine (même limite que l'en-tête de 05, « Console et sessions »).

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
- **mon avis favorable** sur l'arbre exact, noté au §9 avec son empreinte et reporté dans l'attestation de SEC0-15.

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
| — | — | — | — |

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
| 30/10 | AS-08 | *Nightly* épinglée tolérée pour le fuzzing seulement ; repli stable sinon | Pas de CI en mode local |
| 30/10 | AS-09 | `sops` et `age` reportés à la phase 4 | Aucun secret avant |
| 30/10 | AS-10 | Mention des contenus produits par IA globale, jamais par réplique | S7 |
| 30/10 | AS-11 | `tools/assets/` appartient au responsable contenus | Demande de la direction artistique, sans risque particulier |

| N° | Risque | Parade |
|---|---|---|
| R-ARCH-01 | Aucune copie du dépôt hors du poste en mode local | Signalé à l'orchestrateur : une copie locale hors du disque système ou chiffrée, sans service tiers, est à son arbitrage |
| R-ARCH-02 | Le fuzzing de repli, sans guidage par la couverture, trouve moins | Refaire le fuzzing guidé dès que la CI Linux existe |

## 11. Demandes envoyées

| Date | À | Demande | État |
|---|---|---|---|
| 30/10 | responsable-dev (`local_111a89c2-3a9e-44cf-93b9-31c4421e6559`) | Lire §3, §5, §6 et §7 avant d'envoyer le découpage ; m'envoyer les consignes des tâches de §5 avant leur lancement | Envoyée |
| 30/10 | responsable conception (`local_5b3dcf29-8fbb-426c-b63a-80ab0302c1a6`) | Avis du §8 sur la direction artistique §12.9, pour mettre à jour l'état du §12.9 ; aucune règle de jeu changée | Envoyée |
| 30/10 | orchestrateur | Compte rendu des tâches 1, 3 et 4 ; R-ARCH-01 ; `CLAUDE.md` à aligner sur la clôture C4 (crochet Claude Code dans les réglages utilisateur, contrôle de l'origine hors CI) | Envoyée (compte rendu de fin de tour) |

## 12. Demandes reçues

| Date | De | Demande | État |
|---|---|---|---|
| 30/10 | orchestrateur | Tâches 1 à 4 de la consigne de lancement | 1, 3 et 4 faites ; 2 en attente du découpage |

## 13. Sessions actives lancées par moi

Aucune.
