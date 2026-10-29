# Revue du développement par agents

> Version 1.0 — 29/10/2026. Document à part, relu puis intégré par la session principale (voir [README](../README.md), « Déroulement de la conception »).
> Objet : la meilleure façon de faire écrire ce projet par des agents Claude Code (sessions parallèles, sous-agents, worktrees, sessions dans le cloud) et de l'héberger sur GitHub, en **limitant la duplication des copies de travail sur le disque**.
> Les choix marqués **(décidé)** sont tranchés avec leur raison ; **[à mesurer]** : valeur à relever pendant la phase 0 ; **[à vérifier]** : fait extérieur (tarif, comportement d'un outil) à confirmer au moment de la mise en place.
> Les décisions sont numérotées **A1 à A36**. Elles sont distinctes des arbitrages A1 à A6 de la relecture du 29/10 ([README](../README.md)) ; à l'intégration, les citer sous la forme « développement A12 », ou les renuméroter **G1 à G36**.

## Sommaire

0. Résumé et recommandation principale
1. État de la machine (mesuré le 29/10/2026)
2. Où va vraiment le disque
3. Stratégies de partage : ce qui est retenu, ce qui est écarté
4. Décisions
5. Structure du dépôt, `.gitignore` et `.gitattributes`
6. Emplacements de compilation : mécanisme détaillé
7. Règles de travail des agents (à reporter dans CLAUDE.md)
8. Configuration GitHub et intégration continue
9. Procédure de nettoyage
10. Mise en place pendant la phase 0
11. Ce que l'utilisateur fait ou valide lui-même
12. Modifications proposées à CLAUDE.md, au socle et aux revues
13. Risques résiduels

---

## 0. Résumé et recommandation principale

**Le disque ne part pas dans le code.** Les sources et la documentation de ce projet pèseront quelques dizaines de mégaoctets par copie. Presque tout le poids d'une copie vient de ce qu'on y **compile** et de ce qu'on y **génère** : le dossier `target/` de Cargo (8 à 12 Go par copie mesurés sur un autre projet Rust de cette machine), puis l'environnement Python avec PyTorch, le cache d'import de Godot, les assets en Git LFS et les sorties de simulation. Un worktree Git ne coûte presque rien en lui-même, puisque ses objets sont partagés ; c'est **la compilation dans chaque worktree** qui duplique le disque. Elle duplique aussi le temps : une copie neuve recompile toutes les dépendances.

**Recommandation principale (décidé)** : une copie principale sur `main`, à laquelle aucun agent ne touche, et des worktrees éphémères, un par tâche, comme le fait déjà l'application. Mais **les worktrees ne compilent pas chez eux** : leur dossier `target/` est une **jonction NTFS** vers l'un des **trois emplacements de compilation** partagés (`C:\Dev\ts-cache\target\w1` à `w3`), attribués automatiquement au démarrage de la session par un crochet (*hook*) Claude Code, plus un emplacement réservé à la copie principale. Le disque consommé par la compilation est donc **borné par le nombre d'emplacements (4)**, et non plus par le nombre de worktrees créés. Un nouveau worktree hérite des dépendances déjà compilées : sa première compilation prend quelques dizaines de secondes au lieu de plusieurs minutes. Le reste suit la même idée :

- **profils de compilation allégés** : dépendances optimisées mais sans informations de débogage, aucune compilation `release` dans les worktrees, crates lourdes et facultatives hors des membres par défaut ;
- **liens durs automatiques** pour Python (`uv`) et la console web (`pnpm`) ;
- **artefacts générés hors du dépôt**, dans `C:\Dev\ts-cache\artifacts`, avec rétention ;
- Git LFS limité aux assets prêts pour le jeu, et **non extrait** dans les worktrees qui n'en ont pas besoin ;
- au plus **trois sessions qui compilent** en même temps ; sessions en lecture seule sans worktree ; sous-agents plutôt que nouvelles sessions pour les morceaux d'une même tâche ; cloud pour les tâches Linux longues ;
- worktrees **supprimés automatiquement** à la fusion de leur PR.

**GitHub (décidé)** : dépôt **privé** sur un compte **GitHub Pro**, nécessaire pour protéger `main` dans un dépôt privé (environ 4 $ par mois [à vérifier]). Fusion par **squash** des PR quand le contrôle unique `ci-ok` est vert, sans file de fusion. CI Linux à chaque PR ; Windows est vérifié en local par chaque agent, et en CI sur les chemins sensibles, sur `main` et chaque nuit. Les exigences de sécurité de la phase 0 sont bloquantes dès le premier jour.

**Gain estimé** (§2.3) : pour une copie principale et six worktrees, ce qui arrive vite quand les sessions s'accumulent, l'empreinte passe d'environ **75 Go à environ 25 Go** en phases 1 et 2, et d'environ **150 à 200 Go à environ 55 à 70 Go** en phase 5. Surtout, elle **ne croît plus** avec le nombre de sessions **[à mesurer]**.

---

## 1. État de la machine (mesuré le 29/10/2026)

Commandes en lecture seule (`Get-Volume`, `Get-Disk`, `Get-Partition`, tailles de dossiers, outil de stockage de l'application).

| Élément | Constat | Conséquence |
|---|---|---|
| Système | Windows 10 Home 10.0.19045 | Ni *Dev Drive* (Windows 11) ni déduplication de données (Windows Server) |
| Processeur, mémoire | Intel i5-8300H (4 cœurs, 8 fils), 32 Go | Trois compilations Rust simultanées au plus restent supportables |
| Carte graphique | NVIDIA GTX 1050, 4 Go | Entraînement sur processeur en local (petits réseaux PPO) ; PyTorch CPU suffit |
| Disque système `C:` | SSD Sabrent 1 To, NTFS, **930 Go, 267 Go libres** | Tout le projet et les caches au même endroit, ce qui permet les liens durs |
| Second disque | HDD HGST 1 To, partition `H:` **illisible** depuis Windows (système de fichiers inconnu, taille 0) | Non utilisé par le projet (A35) |
| `G:` | Lecteur virtuel de Google Drive (`GoogleDriveFS` actif) | Possible stockage froid des sources d'art (A7), sans disque local s'il est en flux |
| Chemins longs | `LongPathsEnabled = 1` ; `core.longpaths` de Git **non défini** | À activer dans le dépôt (A4) |
| Liens symboliques | Mode développeur inactif, `core.symlinks = false` | On utilise des **jonctions**, qui ne demandent aucun droit administrateur |
| Fins de ligne | `core.autocrlf = true` (configuration système de Git) | Risque pour les empreintes des contenus : `.gitattributes` impose LF (A4) |
| Outils présents | git 2.56 et git-lfs, `gh` connecté avec les droits `repo` et `workflow`, Rust 1.99.1 (MSVC), uv 0.11, pnpm 12, Node 25 (nvm), Python 3.14, .NET SDK 3.1 et 5.0 | Il manque `protoc` (à remplacer par `protox`, en Rust, A15), Godot et un SDK .NET récent (phase 3) |
| Caches déjà présents | `~/.cargo` 1,15 Go ; `~/.rustup` 1,38 Go ; NuGet 4,8 Go ; cache npm 0,76 Go ; magasin pnpm 0,26 Go | Déjà partagés par toutes les copies |
| Worktrees de l'application | Emplacement : **dans le projet** (`<dépôt>/.claude/worktrees`). Aujourd'hui, 4 worktrees d'un autre projet (`dichon_online`) occupent 10,8 Go ; leurs dossiers `target/` mesurent **8,2 et 11,5 Go**, leurs `node_modules` 0,17 à 0,27 Go | C'est la mesure de référence : sans précaution, `target/` représente plus de 90 % d'un worktree |

---

## 2. Où va vraiment le disque

### 2.1 Empreinte par copie de travail, sans précaution

Estimations pour ce projet (environ 35 crates, `bevy_ecs`, `serde`/`ron`, `parquet`, `pyo3`, `rusqlite`, puis `quinn`, `tokio-postgres`, `tract`, `rerun`). Les bornes hautes correspondent à la phase 5.

| Poste | Phase 0 | Phases 1 et 2 | Phase 5 | Dupliqué par worktree ? |
|---|---|---|---|---|
| Sources, docs, contenus RON | 0,02 Go | 0,05 Go | 0,1 à 0,3 Go | Oui, mais négligeable |
| Objets Git (`.git`) | 0,01 Go | 0,05 Go | 0,3 Go (+ LFS) | **Non** : partagés par `git worktree` |
| `target/` debug (dev, tests, clippy) | 1,5 à 3 Go | 6 à 10 Go | 12 à 20 Go | **Oui : le poste principal** |
| `target/` release ou profil de simulation | 0,5 Go | 2 à 4 Go | 4 à 8 Go | Oui, si on en compile dans le worktree |
| Compilation de `ts_py` par maturin (autre jeu de fonctionnalités) | 0,3 Go | 1 Go | 1 à 2 Go | Oui |
| `.venv` Python (numpy, pyarrow, gymnasium) | 0,3 Go | 0,3 Go | 0,3 Go | Oui, avec `pip` |
| PyTorch dans le `.venv` | — | 1 à 1,5 Go (CPU) | 1,5 Go (CPU) ; 3 à 5 Go (CUDA) | Oui, avec `pip` |
| `node_modules` (console React) | — | — | 0,3 à 0,6 Go | Oui, avec `npm` |
| Godot : `.godot/` (imports, C# compilé) | — | — | 0,5 à 3 Go | Oui |
| Assets en Git LFS extraits | — | — | 1 à 5 Go, plus la même quantité dans `.git/lfs` | Oui (copie extraite) |
| Artefacts de monde, Parquet, instantanés, enregistrements Rerun | 0,1 Go | 1 à 5 Go | 5 à 30 Go | Oui, s'ils sont écrits dans la copie |
| Modèles ONNX | — | — | < 0,1 Go | Négligeable |
| Modèles GGUF (`llama.cpp`) | — | — | 2 à 8 Go par modèle | Oui, s'ils sont dans la copie |

**Partagé quoi qu'on fasse** : `~/.cargo` (registre, sources téléchargées ; Cargo le nettoie seul depuis la version 1.88), `~/.rustup` (1,4 Go par chaîne installée), cache `uv`, magasin `pnpm`, paquets NuGet, objets Git et LFS de `.git`.

### 2.2 Ce qui fait vraiment grossir le disque

1. **`target/` par worktree** : au moins 6 à 10 Go dès la phase 1, multipliés par chaque worktree, y compris ceux qu'on a oubliés.
2. **Les worktrees qui survivent** à leur tâche : l'application les garde tant que la session n'est pas archivée, et l'archivage automatique est aujourd'hui désactivé (« Never »).
3. Plus tard, les **sorties de simulation** et les **modèles GGUF**, s'ils sont écrits dans les copies.

### 2.3 Avant et après les mesures

Hypothèse de travail : une copie principale et six worktrees (trois actifs, trois inactifs ou oubliés). Ce cas est courant sans règle de durée de vie : l'autre projet de la machine en a déjà quatre.

| Poste | Avant, phases 1-2 | Après, phases 1-2 | Avant, phase 5 | Après, phase 5 |
|---|---|---|---|---|
| Compilation Rust | 7 × 9 = 63 Go | copie principale 6 Go + 3 emplacements × 4 Go = **18 Go** | 7 × 18 = 126 Go | 10 + 3 × 7 = **31 Go** |
| Python | 7 × 1,5 = 10 Go | cache uv 2 Go + PyTorch dans la seule copie principale ; venvs en liens durs ≈ 0,3 Go | 7 × 2 = 14 Go | 3 Go |
| Web | — | — | 7 × 0,5 = 3,5 Go | magasin 0,6 Go |
| Godot et LFS | — | — | 7 × 4 = 28 Go (+ `.git/lfs`) | copie principale + 1 worktree client = 8 Go |
| Artefacts | dans les copies, 3 à 10 Go | `ts-cache\artifacts`, plafond 10 Go | 10 à 30 Go | plafond 15 Go (+ GGUF 4 Go) |
| Sources des 7 copies | 0,4 Go | 0,4 Go | 1 Go | 1 Go |
| **Total** | **≈ 75 Go** | **≈ 25 à 30 Go** | **≈ 150 à 200 Go** | **≈ 55 à 70 Go** |

Toutes ces valeurs sont **[à mesurer]** pendant la phase 0 (§10, étape 12), puis à chaque phase, avec `cargo xtask disk`. **Attention à la mesure** : l'Explorateur Windows compte un fichier à liens durs dans chaque dossier où il apparaît ; seul l'espace libre du volume fait foi.

---

## 3. Stratégies de partage : ce qui est retenu, ce qui est écarté

| Stratégie | Verdict | Pourquoi |
|---|---|---|
| `git worktree` plutôt que des clones | **Retenu** (A1) | Objets partagés ; c'est déjà le mécanisme de l'application |
| Un seul `CARGO_TARGET_DIR` partagé par tous les worktrees | **Écarté** (A10) | (1) Le verrou du dossier de compilation **sérialise** les sessions. (2) Les binaires finaux (`target/debug/ts-sim.exe`, `CARGO_BIN_EXE_*`) sont **écrasés** par un autre worktree : un test peut passer sur le code d'une autre branche. (3) Sous Windows, un `.exe` ou une `.dll` en cours d'exécution **ne peut pas être remplacé**, et la compilation d'une autre session échoue. (4) Les crates de l'espace de travail ont le même chemin relatif d'un worktree à l'autre, donc elles se **recompilent en alternance**. |
| `build.build-dir` partagé (dossier des fichiers intermédiaires) | **Écarté pour l'instant** | Sa stabilisation et son verrouillage fin ne sont pas confirmés pour Cargo 1.99 [à vérifier] ; il garderait les points (1) et (4). À réexaminer quand le verrouillage fin sera stable. |
| **Emplacements de compilation attribués** (un `target/` partagé **en série**, jamais en même temps) | **Retenu** (A9) | Garde le partage des dépendances déjà compilées, sans aucun des quatre risques ci-dessus, puisqu'un emplacement ne sert qu'à un seul worktree à la fois |
| `sccache` | **Écarté** (A11) | Il économise du temps, **pas du disque** : chaque `target/` garde ses fichiers, et le cache ajoute 5 à 10 Go. Il ne met pas en cache la compilation incrémentale des crates de l'espace de travail. Les emplacements réchauffés donnent déjà l'essentiel du gain de temps. |
| Registre Cargo et `~/.cargo` partagés | **Déjà le cas** | `CARGO_HOME` est global ; nettoyage automatique depuis Cargo 1.88 |
| venv unique pour tous les worktrees | **Écarté** | Une installation éditable de `ts_py` pointerait vers une seule branche ; avec `uv`, un venv par worktree ne coûte presque rien |
| `uv` avec cache global et **liens durs** | **Retenu** (A16) | Mode par défaut d'`uv` sous Windows ; le cache est sur le même volume |
| `pnpm` avec magasin global | **Retenu** (A17) | `node_modules` en liens durs ; les scripts d'installation des dépendances sont désactivés par défaut, ce qui renforce S26 |
| Caches Godot partagés | **Écarté** | `.godot/` contient des chemins et un état d'éditeur ; on limite plutôt le nombre de copies qui ouvrent le client (A18) |
| Git LFS | **Retenu, ciblé** (A7) | Assets prêts pour le jeu et modèles ONNX seulement ; non extraits dans les worktrees qui ne touchent pas au client |
| Stockage externe pour les gros binaires | **Retenu** (A6, A7, A8) | Artefacts régénérables, sources d'art, modèles GGUF : hors de Git |
| Artefacts générés hors du dépôt | **Retenu** (A6) | Le déterminisme par graine permet de ne suivre que les **empreintes** attendues |
| *Sparse checkout* | **Écarté en local** | Les sources sont petites ; le seul poste lourd (LFS) se règle par `lfs.fetchexclude` |
| *Partial clone* | **CI seulement** | En local, il n'y a qu'un dépôt ; en CI, `fetch-depth: 2` suffit |
| Jonctions NTFS | **Retenu** (A9) | Sans droit administrateur, transparentes pour tous les outils (Bash, PowerShell, maturin, commandes de l'utilisateur) |
| Liens symboliques | **Écarté** | Exigent le mode développeur ou des droits administrateur |
| Compression NTFS (`compact /c`) des emplacements | **En réserve** | Gain probable de 40 à 50 % sur les `.rlib` et `.pdb`, au prix de compilations plus lentes [à mesurer] ; activée seulement si l'espace libre passe sous 60 Go |
| `cargo clean` ciblé, `cargo-sweep` | **Seuils par `cargo xtask disk`** (A25) | D'abord les dossiers `incremental/`, puis vidage complet d'un emplacement libre au-delà d'un seuil ; aucun outil de plus à auditer |
| Outils de nettoyage de l'application | **Retenu** (A25) | Archivage automatique à la fermeture de la PR et après 7 jours d'inactivité ; nettoyage mensuel des worktrees inactifs |

---

## 4. Décisions

### 4.1 Dépôt

- **A1 — Un monodépôt Git, des worktrees, aucun clone** (décidé). *Raison : cœur, protocole, client et outils changent souvent ensemble ; un seul dépôt donne des modifications atomiques, une CI et un seul jeu d'objets partagé par les worktrees.*
- **A2 — Un second dépôt privé `time_switcher-ops`, à partir de la phase 4 seulement** (décidé) : secrets chiffrés (`sops` + `age`, S28), manifestes de déploiement signés, paramètres d'exploitation. La population en ligne et le paquet d'exploitation n'y sont pas non plus : ils vivent dans le stockage des serveurs (zone 2, S2, S29). Ce dépôt n'est **jamais** ouvert dans une session d'agent ordinaire. *Raison : la revue de sécurité sépare la production du développement, et un agent qui ne voit pas un secret ne peut pas le divulguer.*
- **A3 — Dépôt GitHub privé, sur un compte GitHub Pro** (décidé). *Raison : le jeu n'est pas annoncé et sa conception décrit des mécanismes qu'il vaut mieux taire ; dans un dépôt privé, la protection de branche et les *rulesets* exigent l'offre Pro (dans l'offre gratuite, ils ne couvrent que les dépôts publics), qui apporte aussi environ 3 000 minutes de CI par mois [à vérifier].* Repli si Pro n'est pas souscrit : dépôt privé gratuit, mêmes règles appliquées par les agents et par un crochet local qui refuse un `push` vers `main`, mais sans garantie côté serveur.
- **A4 — Fins de ligne LF partout, chemins longs activés** (décidé) : `.gitattributes` avec `* text=auto eol=lf`, `core.autocrlf=false` et `core.longpaths=true` dans la configuration du dépôt. Par ailleurs, l'empreinte du paquet de contenus se calcule sur le **paquet compilé**, jamais sur le texte source. *Raison : avec `autocrlf=true`, une fiche RON n'aurait pas les mêmes octets sous Windows et sous Linux, ce qui fausserait les empreintes et les tests dorés.*
- **A5 — L'arborescence du §5** (décidé). *Raison : elle reprend la revue d'architecture §5.1 et ajoute ce que demande le travail par agents (outils, crochets, CI, emplacement des données).*
- **A6 — Aucun artefact régénérable dans le dépôt** (décidé) : artefacts de monde, paquets compilés, Parquet, instantanés, sauvegardes, enregistrements Rerun et corpus de fuzzing vont dans `TS_CACHE` (`C:\Dev\ts-cache` sous Windows, `~/.cache/time_switcher` sous Linux). Le dépôt ne suit que les **spécifications** (`WorldSpec`, scénarios) et les **empreintes attendues**. *Raison : grâce à la reproductibilité par graine, un artefact se reconstruit à l'identique à partir de sa spécification et d'un binaire donné ; le stocker n'apporte que du poids.* Les artefacts livrés (population solo, paquet de contenus) sont produits une fois par la tâche de publication, signés (S27) et attachés à la version, jamais commis.
- **A7 — Git LFS réservé aux assets prêts pour le jeu et aux modèles ONNX, à partir de la phase 3, et non extrait par défaut** (décidé). La configuration commune du dépôt fixe `lfs.fetchexclude = client/assets/**,data/models/**` : les fichiers restent des pointeurs dans les worktrees. La copie principale et le worktree client lèvent l'exclusion par `git config --worktree` (avec `extensions.worktreeConfig=true`), puis lancent `git lfs pull`. Les **sources d'art** (`.blend`, `.psd`, WAV) restent hors de Git, dans un dossier Google Drive (`G:`) ou sur un disque d'archives. *Raison : un fichier LFS existe deux fois par copie (dans `.git/lfs` et dans la copie extraite), et le quota gratuit de GitHub est d'environ 10 Go de stockage et 10 Go de transfert par mois [à vérifier].*
- **A8 — Les modèles GGUF ne passent jamais par Git** (décidé) : `cargo xtask fetch-models` les télécharge depuis une source épinglée dans `data/models/gguf.ron` (adresse et SHA-256), vers `TS_CACHE\models`, et refuse tout fichier dont l'empreinte diffère. *Raison : quelques gigaoctets par modèle, et une empreinte vérifiée protège la chaîne de production (S26, S27).*

### 4.2 Compilation

- **A9 — Quatre emplacements de compilation attribués par jonction** (décidé) : `ts-cache\target\main` pour la copie principale, `w1`, `w2` et `w3` pour les worktrees. Le crochet `SessionStart` attribue un emplacement libre et crée la jonction `target` (mécanisme au §6). *Raison : le disque de compilation est borné à 4 emplacements, quel que soit le nombre de worktrees, et une nouvelle copie hérite de dépendances déjà compilées ; un emplacement ne sert jamais à deux copies en même temps, ce qui écarte tous les risques d'un `target/` commun.*
- **A10 — Pas de `CARGO_TARGET_DIR` unique partagé** (décidé). *Raison : verrou, binaires écrasés, fichiers verrouillés par Windows et recompilations croisées (§3).*
- **A11 — Pas de `sccache`** (décidé). *Raison : il fait gagner du temps sans faire gagner de disque, et les emplacements réchauffés couvrent déjà ce temps.*
- **A12 — Profils allégés ; `release` et simulations longues seulement dans la copie principale et en CI** (décidé) :

  ```toml
  # Cargo.toml (racine)
  [profile.dev]
  opt-level = 1                 # crates du projet : rapides à compiler, assez vives pour les tests
  debug = "line-tables-only"    # traces de pile avec lignes, sans débogueur complet
  incremental = true

  [profile.dev.package."*"]     # dépendances : optimisées, sans informations de débogage
  opt-level = 2
  debug = false

  [profile.ci]                  # CI : petit cache, compilation non incrémentale
  inherits = "dev"
  incremental = false
  debug = false

  [profile.sim]                 # ts-sim, déterminisme long, bancs : copie principale et CI seulement
  inherits = "release"
  debug = false
  codegen-units = 16
  incremental = false

  [profile.release]             # publication
  debug = false
  lto = "thin"
  ```

  *Raison : sous MSVC, les informations de débogage des dépendances (`.pdb`) pèsent souvent plus que le code ; optimiser les dépendances rend les tests et les petites simulations assez rapides pour ne jamais compiler en `release` dans un worktree.* Les **empreintes dorées** valent pour un couple (profil, plateforme). Si l'équivalence entre profils se vérifie [à mesurer], une seule valeur suffit ; sinon, les tests dorés tournent dans le profil `ci`.
- **A13 — Crates lourdes et facultatives hors des membres par défaut** (décidé) : `default-members` exclut `ts_viz` (Rerun), `ts_py` (pyo3), `ts_client_native` (cdylib) et `xtask` ; `fuzz/` est exclu de l'espace de travail. On compile ces crates seulement avec `-p`. *Raison : `cargo test` dans un worktree ne compile alors que le cœur et les règles, soit plusieurs gigaoctets et plusieurs minutes de moins ; `rerun` s'utilise sans le visualiseur (`default-features = false`).*
- **A14 — Une chaîne Rust épinglée, sans *nightly* en local** (décidé) : `rust-toolchain.toml` fixe la version stable (1.99.1 au départ), changée seulement par une PR dédiée, qui supprime ensuite l'ancienne chaîne (`rustup toolchain uninstall`). Le fuzzing (*nightly*) tourne en CI ou dans le cloud. *Raison : une chaîne pèse environ 1,4 Go, et chaque version épinglée qui reste installée s'ajoute.*
- **A15 — `cargo xtask` comme lanceur de tâches ; `protox` au lieu de `protoc`** (décidé). `xtask` est une crate Rust de l'espace de travail, avec les commandes `check`, `disk`, `slots`, `determinism`, `py`, `fetch-models` et `client-native`. *Raison : rien à installer, le même outil sous Windows, sous Linux et en CI, et du code que Claude écrit et teste comme le reste ; `protox`, en Rust pur, compile les schémas protobuf sans binaire extérieur.*

### 4.3 Python, web, Godot

- **A16 — `uv` ; un `.venv` par worktree en liens durs ; PyTorch CPU, dans la copie principale seulement** (décidé). `pyproject.toml` sépare un groupe de base (numpy, pyarrow, gymnasium, pettingzoo, pytest, maturin) d'un groupe `train` (torch CPU, onnx, onnxruntime). Les worktrees n'installent que la base (`uv sync --locked`), et `link-mode = "hardlink"` est explicite dans `[tool.uv]`. `ts_py` se compile par `cargo xtask py` (maturin, profil `dev`), dans l'emplacement du worktree. *Raison : PyTorch est le seul paquet Python lourd ; il ne sert qu'à l'entraînement, qui tourne dans la copie principale, sur une machine d'entraînement ou dans le cloud ; la GTX 1050 n'apporte rien aux petits réseaux PPO prévus.*
- **A17 — `pnpm` au lieu de `npm` pour la console** (décidé) : `pnpm-lock.yaml`, `pnpm install --frozen-lockfile`, scripts d'installation des dépendances refusés sauf liste `onlyBuiltDependencies`, `pnpm audit`. *Raison : `node_modules` en liens durs depuis un seul magasin, et un réglage plus sûr que `npm ci --ignore-scripts` ; modification de S26 proposée au §12.*
- **A18 — Une seule session à la fois sur `client/`** (décidé) : `.godot/` est ignoré, les fichiers `*.import` sont suivis, la bibliothèque native est copiée par `cargo xtask client-native` dans `client/addons/ts_native/bin/` (ignoré). Un SDK .NET récent (8 ou plus, selon la version de Godot retenue [à vérifier]) s'installe en phase 3. *Raison : scènes `.tscn` et ressources Godot se fusionnent mal, et seules deux copies (la principale et le worktree client) paient alors le cache d'import.*

### 4.4 Organisation des agents

- **A19 — Copie principale sur `main`, jamais éditée** (décidé) : elle sert à lire, à lancer le jeu, aux simulations longues et à l'entraînement. Elle se met à jour par `git pull --ff-only` après chaque fusion. Un crochet `PreToolUse` refuse `Edit` et `Write` sur un fichier de la copie principale. La session principale fait ses propres modifications dans un worktree (outil `EnterWorktree`). *Raison : une copie stable sert de référence à l'utilisateur et aux sessions en lecture seule, et aucune session ne casse le travail d'une autre en changeant de branche.*
- **A20 — Une tâche = une branche `claude/<sujet>` = un worktree = une PR fusionnée par squash** (décidé). *Raison : c'est le fonctionnement de l'application, et une PR courte se relit, se teste et s'annule facilement.*
- **A21 — Au plus trois sessions qui compilent du Rust en même temps** (décidé), soit autant que d'emplacements ; les sessions de documentation (sans compilation) ne comptent pas. Une quatrième tâche de code attend ou part dans le cloud (A24). *Raison : 4 cœurs et 32 Go supportent trois compilations parallèles ; au-delà, chacune ralentit sans que l'ensemble avance plus vite.* Répartition conseillée :

  | Phase | Sessions de code locales | Découpage |
  |---|---|---|
  | 0 | 1, puis 2 | `ts_kernel` d'abord, seul (tout en dépend) ; en parallèle : CI, `xtask` et crochets (`.github/`, `tools/`) |
  | 1 | 3 | Après une PR « contrat » sur `ts_model` : `ts_body` + `ts_ai` ; `ts_mobility` + `ts_perception` + `ts_world_model` ; `ts_economy` + `ts_persist` ; `ts_combat` et l'arène ensuite |
  | 2 | 2 | `ts_pilot` (central, une seule session) ; `ts_meta` + joueurs synthétiques dans `ts_runner` |
  | 3 | 3 | `client/` (exclusif, A18) ; `ts_view` + `ts_protocol` ; `ts_host` + `ts_client_native` |
  | 4 | 3, plus le cloud | `ts_net` + `ts_server` (cloud possible) ; `ts_accounts` + `ts_persist` PostgreSQL ; `ts_admin` + `admin-web/` |
  | 5 | 3, plus le cloud | Par étape de la revue d'architecture §22 ; campagnes, fuzzing et entraînement dans le cloud ou dans la copie principale |

- **A22 — Les sessions en lecture seule n'ont pas de worktree ; les sous-agents passent avant de nouvelles sessions** (décidé). Revues, analyses, questions et vérifications de cohérence s'ouvrent avec `use_worktree: false` dans la copie principale, et ne modifient aucun fichier suivi. Exploration, écriture de tests, relecture et essais d'une même modification se font par **sous-agents**, dans le worktree de la session. *Raison : un sous-agent partage la copie et l'emplacement de compilation, donc ne coûte aucun disque.*
- **A23 — Chaque session possède des crates et des répertoires ; les fichiers partagés passent par des PR « contrat »** (décidé). La consigne de chaque session nomme les chemins qu'elle possède ; elle ne touche à rien d'autre. Fichiers partagés : `Cargo.toml` racine, `Cargo.lock`, `crates/ts_model/`, `crates/ts_game/`, `.github/`, `.cargo/`, `.claude/`, `tools/`, `CLAUDE.md`, `docs/conception/0*.md`. Ils se modifient seulement dans une PR courte et dédiée, fusionnée **avant** le travail qui en dépend. Exceptions mécaniques : ajouter une dépendance (une ligne triée dans `[workspace.dependencies]`) et un greffon (une ligne triée dans `standard_rules()`). En cas de conflit sur `Cargo.lock`, on reprend la version de `main` et on relance `cargo check`, sans jamais fusionner le fichier à la main. *Raison : les conflits naissent presque tous dans ces quelques fichiers ; `members = ["crates/*"]` et les listes triées, une entrée par ligne, font le reste.*
- **A24 — Le cloud pour les tâches Linux longues ou autonomes** (décidé) : fuzzing d'une heure, campagnes de stabilité, crates serveur (`ts_net`, `ts_server`, `ts_accounts`), mises à jour de dépendances, et toute tâche de code quand les trois emplacements sont pris. Jamais pour `client/`, `ts_client_native` ni le reste du code propre à Windows. *Raison : aucun disque local ; une session dans le cloud part de la branche poussée ou de `main`, et ne rend pas compte : la session principale vérifie sa PR.*
- **A25 — Durée de vie des worktrees : jusqu'à la fusion** (décidé) : archivage automatique de la session à la fermeture de sa PR, qui supprime un worktree propre ; archivage après **7 jours** d'inactivité ; nettoyage mensuel des worktrees inactifs depuis 30 jours (outil de l'application) ; `cargo xtask disk --clean` chaque semaine (§9). *Raison : l'accumulation de worktrees oubliés est la première cause de disque perdu, comme sur l'autre projet de la machine.*

### 4.5 GitHub et intégration continue

- **A26 — `main` protégée par un *ruleset*** (décidé) : PR obligatoire, avec **zéro approbation** (un compte seul ne peut pas approuver sa propre PR) ; contrôle requis unique `ci-ok`, branche à jour avant fusion ; historique linéaire ; force-push et suppression interdits ; aucun contournement. Fusion **par squash uniquement**, titre de la PR comme message ; fusion automatique permise ; branche supprimée après fusion. *Raison : la protection automatique remplace la relecture humaine, puisque l'utilisateur ne lit pas le code (S26).*
- **A27 — Pas de file de fusion** (décidé). *Raison : la file de fusion de GitHub n'est pas proposée aux comptes personnels, et avec trois PR en vol au plus, la règle « branche à jour avant fusion » suffit.*
- **A28 — Linux en CI à chaque PR ; Windows en local, et en CI sur les chemins sensibles, sur `main` et chaque nuit** (décidé). Avant toute PR, chaque agent lance `cargo xtask check` sous Windows. La CI Windows tourne sur la PR si elle touche `ts_client_native`, `ts_py`, `client/`, `.cargo/`, `Cargo.lock`, `rust-toolchain.toml`, `.github/` ou `tools/`, ou si elle porte l'étiquette `ci:windows`. *Raison : les minutes Windows comptent double [à vérifier] ; or les agents travaillent déjà sous Windows, et la CI nocturne attrape le reste en moins de 24 heures.* Le test de déterminisme (1 fil contre N) tourne sur les deux systèmes. Les empreintes ne sont pas comparées **entre** plateformes, ce que le socle n'exige pas.
- **A29 — Sécurité de la CI conforme à S26, bloquante dès la phase 0** (décidé) : `cargo-deny` (vulnérabilités, licences, sources : crates.io seulement, interdictions) et `gitleaks` à chaque PR ; `cargo-audit` quand `Cargo.lock` change, et chaque nuit ; `--locked` partout ; actions **épinglées par empreinte de commit**, vérifiées par un script ; `permissions: contents: read` par défaut ; aucun secret dans les tâches de PR ; Dependabot pour cargo, github-actions, uv, pnpm et NuGet, regroupé et hebdomadaire. Un test vérifie que les fonctionnalités `public` et `dev-commands` **ne compilent pas ensemble**, et un script, que chaque crate hors `ts_client_native` porte `#![forbid(unsafe_code)]`. *Raison : la détection de secrets de GitHub est payante dans un dépôt privé, et ces contrôles sont la seule relecture de sécurité systématique.*
- **A30 — Caches de CI économes** (décidé) : `Swatinem/rust-cache` (épinglé), enregistré seulement depuis `main` et lu par les PR ; profil `ci` sans informations de débogage ; `concurrency` qui annule l'exécution précédente d'une même branche ; une PR en brouillon ne lance pas la CI. *Raison : le cache de GitHub est limité à 10 Go par dépôt [à vérifier], et chaque minute compte dans le quota.*
- **A31 — Commits conventionnels, descriptions en français** (décidé) : `type(portée): description`, où le type est `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `build`, `ci` ou `chore`, et la portée, la crate ou la zone (`ts_kernel`, `ci`, `conception`) ; par exemple `feat(ts_kernel): ajoute l'ordonnanceur à roues`. Le titre de la PR devient le commit de `main` et la CI vérifie son format. Pied de message `Co-Authored-By` de Claude ; adresse de commit **noreply** de GitHub. *Raison : un historique lisible par l'utilisateur, en français, qui reste filtrable par crate, sans adresse personnelle publiée.*
- **A32 — Relecture automatique locale, sans agent dans la CI** (décidé). Avant d'ouvrir une PR de code, la session lance `/code-review` (niveau moyen) et corrige. Pour les PR qui touchent `ts_net`, `ts_pilot`, `ts_persist` (codec), `ts_client_native`, `ts_accounts`, `ts_admin`, `.github/`, `deny.toml`, `.gitleaks.toml` ou `.claude/`, elle ajoute `/security-review` et une relecture par un sous-agent neuf (niveau élevé). La session principale relit les PR de Dependabot de la même manière avant de les fusionner. *Raison : aucun humain ne relit le code ; une action Claude dans la CI demanderait une clé d'API dans les secrets du dépôt et coûterait à chaque poussée.*
- **A33 — Fuzzing : 10 minutes par cible chaque nuit, 60 minutes chaque semaine** (décidé), en CI Linux ; le critère de phase 0 « une heure sans plantage » se vérifie dans le cloud ou par la tâche hebdomadaire. *Raison : une heure chaque nuit coûterait à elle seule environ 1 800 minutes par mois.*

### 4.6 Garde-fous de la machine

- **A34 — Seuil d'alerte à 40 Go libres** (décidé) : sous ce seuil, le crochet de démarrage avertit ; on ne crée plus de worktree, et on lance `cargo xtask disk --clean`. Sous 60 Go, la compression NTFS des emplacements devient permise (§3). *Raison : les compilations Rust et les mises à jour de Windows demandent de la marge, et un disque plein casse les builds de façon peu lisible.*
- **A35 — Le disque `H:` (HDD) n'est pas utilisé par le projet** (décidé), tant que l'utilisateur n'a pas vérifié son contenu. *Raison : il est illisible par Windows, il peut contenir des données (chiffrement, système Linux), et un disque dur serait trop lent pour compiler ; s'il est vide et formaté par l'utilisateur, il pourra servir aux archives froides (vieux Parquet, sources d'art).*
- **A36 — L'emplacement des worktrees de l'application reste dans le projet** (décidé) : `.claude/worktrees/` est ignoré par Git et exclu de l'espace de travail Cargo (`exclude = [".claude"]`). *Raison : une fois `target/` sorti par jonction, un worktree ne pèse que ses sources, et le déplacer n'apporterait rien.*

---

## 5. Structure du dépôt, `.gitignore` et `.gitattributes`

### 5.1 Arborescence

```
time_switcher/                         copie principale (branche main, jamais éditée)
├─ .cargo/config.toml                  alias xtask, options communes (rien de propre à une machine)
├─ .claude/
│  ├─ settings.json                    crochets SessionStart et PreToolUse (suivi)
│  ├─ settings.local.json              réglages personnels (ignoré)
│  └─ worktrees/                       worktrees de l'application (ignoré)
├─ .github/
│  ├─ workflows/ci.yml                 PR et main
│  ├─ workflows/nightly.yml            Windows complet, fuzzing, cargo-audit, tests longs
│  ├─ dependabot.yml
│  └─ pull_request_template.md
├─ .gitattributes  .gitignore  .gitleaks.toml  .lfsconfig (phase 3)
├─ Cargo.toml  Cargo.lock  rust-toolchain.toml  deny.toml  clippy.toml  rustfmt.toml
├─ CLAUDE.md  README.md
├─ crates/                             les crates de la revue d'architecture §5.1
│  ├─ ts_kernel/ … ts_viz/
│  └─ ts_protocol/proto/               schémas protobuf (compilés par protox)
├─ fuzz/                               cibles cargo-fuzz (exclu de l'espace de travail)
├─ tools/
│  ├─ xtask/                           check, disk, slots, determinism, py, fetch-models, client-native
│  └─ dev/                             session-start.ps1, guard-main.ps1
├─ python/
│  ├─ pyproject.toml  uv.lock
│  ├─ ts_gym/                          environnements, entraînement, export, calibrage
│  └─ tests/
├─ client/                             projet Godot 4 C# (phase 3)
│  ├─ project.godot  TimeSwitcher.csproj
│  ├─ scenes/  scripts/  ui/
│  ├─ assets/                          assets prêts pour le jeu (LFS, non extraits par défaut)
│  └─ addons/ts_native/                chargeur ; bin/ ignoré (copié par xtask)
├─ admin-web/                          console TypeScript (phase 4) : package.json, pnpm-lock.yaml, src/
├─ data/
│  ├─ content/                         fiches RON : rôles, métiers, recettes, actes, tables, paramètres
│  ├─ population/                      paramètres RON du générateur de population
│  ├─ scenarios/                       scénarios RON, dont arenes/ et exploits/ (équilibrage E39)
│  ├─ worlds/                          WorldSpec et empreintes attendues (aucun artefact)
│  ├─ golden/                          empreintes dorées et événements clés attendus
│  └─ models/                          registre : manifest.ron suivis, model.onnx en LFS (phase 5), gguf.ron
└─ docs/
   ├─ conception/                      socle, domaines, revues (inchangé)
   ├─ archives/
   └─ dev/                             guide de développement : commandes xtask, procédures
```

Hors du dépôt, sur le même volume :

```
C:\Dev\ts-cache\                       TS_CACHE (régénérable en entier, sauf models\ qui se retélécharge)
├─ target\main\  target\w1\  target\w2\  target\w3\    emplacements de compilation
├─ target\w1.lease …                   bail : chemin du worktree qui tient l'emplacement
├─ artifacts\packs\<sha256>\           paquets de contenus compilés
├─ artifacts\worlds\<sha256>\          artefacts de monde (ts-worldbuild)
├─ artifacts\runs\<date>-<scénario>-<sha8>\   sorties ts-sim (manifest, Parquet, rapport)
├─ fuzz\corpus\                        corpus de fuzzing locaux
└─ models\gguf\                        modèles de langage locaux (phase 5)
```

`ts-sim` reçoit son dossier de sortie par `--out` ; à défaut, `xtask` passe `TS_CACHE\artifacts\runs`. Le cœur ne lit jamais de variable d'environnement : seuls les hôtes et les outils le font, ce qui respecte la règle du cœur sans E/S.

### 5.2 `.gitignore` proposé

```gitignore
# --- Rust ---
/target
/fuzz/target/
/fuzz/corpus/
/fuzz/artifacts/
**/*.rs.bk

# --- Claude Code ---
/.claude/worktrees/
/.claude/settings.local.json

# --- Python ---
.venv/
__pycache__/
*.py[cod]
.pytest_cache/
.ruff_cache/
.mypy_cache/
*.egg-info/
*.pyd

# --- Web ---
node_modules/
admin-web/dist/
.pnpm-store/

# --- Godot / .NET (les fichiers *.import restent suivis) ---
client/.godot/
client/bin/
client/obj/
client/export/
client/addons/ts_native/bin/
*.user

# --- Artefacts régénérables : ils vont dans TS_CACHE ---
/artifacts/
/out/
*.parquet
*.rrd
*.tspack
*.tsworld
*.tssnap
*.sqlite
*.sqlite3
*.db
*.gguf
*.pt
*.pth
*.ckpt
*.safetensors
*.onnx
!data/models/**/*.onnx
perf.data*
flamegraph*.svg

# --- Secrets (S28) : jamais dans le dépôt ---
.env
.env.*
!.env.example
*.pem
*.key
*.p12
*.pfx
*.age
secrets/

# --- Systèmes et éditeurs ---
Thumbs.db
desktop.ini
.DS_Store
.vscode/
.idea/
*.log
```

### 5.3 `.gitattributes` proposé

```gitattributes
# Fins de ligne : LF partout (A4)
*            text=auto eol=lf
*.bat        text eol=crlf
*.cmd        text eol=crlf
*.sln        text eol=crlf

# Binaires
*.png  binary
*.jpg  binary
*.ico  binary
*.ttf  binary
*.otf  binary

# Générés : repliés dans les diffs de GitHub
Cargo.lock      linguist-generated=true
uv.lock         linguist-generated=true
pnpm-lock.yaml  linguist-generated=true

# Git LFS (à activer en phase 3 et 5, A7)
# client/assets/**/*.glb  filter=lfs diff=lfs merge=lfs -text
# client/assets/**/*.png  filter=lfs diff=lfs merge=lfs -text
# client/assets/**/*.ogg  filter=lfs diff=lfs merge=lfs -text
# client/assets/**/*.wav  filter=lfs diff=lfs merge=lfs -text
# data/models/**/*.onnx   filter=lfs diff=lfs merge=lfs -text
```

---

## 6. Emplacements de compilation : mécanisme détaillé

### 6.1 Principe

- Un **emplacement** est un dossier `C:\Dev\ts-cache\target\<nom>` qui ne contient **que des fichiers régénérables**. Le pire cas, un emplacement vidé par erreur, coûte une recompilation et ne perd aucune donnée.
- `main` est réservé à la copie principale. `w1`, `w2` et `w3` sont prêtés aux worktrees, avec un **bail** : un fichier `wN.lease` qui contient le chemin du worktree.
- Un bail est **libre** si le fichier n'existe pas, ou si le worktree qu'il nomme n'existe plus ou ne pointe plus vers cet emplacement (worktree supprimé ou archivé). Sinon, il est **tenu**.
- Au démarrage de chaque session, le crochet `SessionStart` lance `tools/dev/session-start.ps1`. Si `target` est déjà une jonction, le script ne fait rien. Sinon, il prend un bail libre (création exclusive du fichier, sans course possible) et crée la jonction `target` vers l'emplacement. S'il ne trouve aucun bail libre, il affiche un avertissement : la session ne compile pas, et attend ou passe la tâche au cloud.
- Un worktree que l'application réutilise garde son emplacement, déjà chaud. Un emplacement repris par un autre worktree garde les dépendances compilées ; seules les crates du projet se recompilent.

### 6.2 Script de démarrage (esquisse à finaliser en phase 0)

```powershell
# tools/dev/session-start.ps1 : appelé par le crochet SessionStart ; idempotent, ne bloque jamais
$ErrorActionPreference = 'Stop'
try {
  $cache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
  $slots = Join-Path $cache 'target'
  $top   = (git rev-parse --show-toplevel).Trim() -replace '/', '\'
  $link  = Join-Path $top 'target'
  $isMain = Test-Path (Join-Path $top '.git') -PathType Container   # .git est un fichier dans un worktree
  function JunctionOf($p) { $i = Get-Item $p -Force -ErrorAction SilentlyContinue
                            if ($i -and $i.LinkType -eq 'Junction') { "$($i.Target)" } }
  New-Item -ItemType Directory -Force $slots | Out-Null

  if (JunctionOf $link) { "Emplacement de compilation : $(JunctionOf $link)" }
  elseif (Test-Path $link) { "AVERTISSEMENT : target/ local (hors emplacement). Voir cargo xtask slots." }
  else {
    $slot = $null
    if ($isMain) { $slot = Join-Path $slots 'main' }
    else {
      foreach ($n in 'w1','w2','w3') {
        $lease = Join-Path $slots "$n.lease"; $dir = Join-Path $slots $n
        if (Test-Path $lease) {
          $owner = (Get-Content $lease -Raw).Trim()
          if ((JunctionOf (Join-Path $owner 'target')) -eq $dir) { continue }   # bail tenu
          Remove-Item $lease -Force                                            # bail orphelin
        }
        try { $f = [IO.File]::Open($lease, 'CreateNew', 'Write'); $f.Dispose()
              Set-Content -Path $lease -Value $top -NoNewline; $slot = $dir; break } catch { continue }
      }
    }
    if ($slot) { New-Item -ItemType Directory -Force $slot | Out-Null
                 New-Item -ItemType Junction -Path $link -Target $slot | Out-Null
                 "Emplacement de compilation attribué : $slot" }
    else { "AVERTISSEMENT : les 3 emplacements sont pris. Ne pas compiler ici : attendre, ou passer la tâche au cloud." }
  }
  $free = [math]::Round((Get-PSDrive C).Free / 1GB)
  if ($free -lt 40) { "AVERTISSEMENT : $free Go libres (< 40). Lancer cargo xtask disk --clean ; ne créer aucun worktree." }
} catch { "session-start : $($_.Exception.Message)" }
exit 0
```

Réglages des crochets dans `.claude/settings.json` (le format exact de la commande sous Windows est [à vérifier] en phase 0) :

```json
{
  "hooks": {
    "SessionStart": [
      { "hooks": [ { "type": "command",
        "command": "pwsh -NoProfile -File \"$CLAUDE_PROJECT_DIR/tools/dev/session-start.ps1\"" } ] }
    ],
    "PreToolUse": [
      { "matcher": "Edit|Write|NotebookEdit",
        "hooks": [ { "type": "command",
          "command": "pwsh -NoProfile -File \"$CLAUDE_PROJECT_DIR/tools/dev/guard-main.ps1\"" } ] }
    ]
  }
}
```

`guard-main.ps1` lit l'appel d'outil sur l'entrée standard. Si le fichier visé est dans la copie principale (et non sous `.claude/worktrees/`), il refuse (code de sortie 2) avec le message : « Copie principale en lecture seule : travaillez dans un worktree (EnterWorktree). » Les fichiers hors du dépôt (mémoire, notes, *scratchpad*) restent permis.

### 6.3 Points à vérifier en phase 0 (bloquants pour adopter le mécanisme)

1. **Suppression d'un worktree qui contient la jonction** : créer un worktree jetable, lui attribuer un emplacement, compiler, puis le supprimer par `git worktree remove` et par l'archivage de l'application. Vérifier que l'emplacement reste intact, ou au pire qu'il est vidé sans autre dégât. Faire de même avec `git clean -xdf`.
2. **Crochet `SessionStart`** : il s'exécute bien dans le dossier du worktree, avec `pwsh` disponible.
3. **Chemins** : `rustc`, `link.exe` et maturin compilent bien à travers la jonction (chemins longs).
4. **Repli** si le point 1 échoue : le crochet écrit `CARGO_TARGET_DIR=<emplacement>` dans `CLAUDE_ENV_FILE` au lieu de créer une jonction, et `xtask` lit le bail pour l'utilisateur. Le partage reste le même ; seule la commande manuelle de l'utilisateur hors session perd l'emplacement.

---

## 7. Règles de travail des agents (à reporter dans CLAUDE.md)

Texte prêt à coller, sous un titre « Développement par agents » :

```markdown
## Développement par agents (détail : docs/conception/revues/developpement-agents.md)

- **Copie principale** (`C:\Dev\Claude\time_switcher`) : toujours sur `main`, propre, jamais éditée. On y lit, on y lance le jeu, les simulations longues et l'entraînement. Après chaque fusion : `git pull --ff-only`. Pour modifier quoi que ce soit, y compris la documentation, travailler dans un worktree (`EnterWorktree` si la session a démarré dans la copie principale).
- **Une tâche = une branche `claude/<sujet>` = un worktree = une PR**, fusionnée par squash quand `ci-ok` est vert (`gh pr merge --squash --auto`). Jamais de commit ni de push sur `main`. PR courtes : un seul sujet, environ 400 lignes au plus hors fichiers générés.
- **Au plus 3 sessions qui compilent du Rust en même temps** (3 emplacements de compilation). Si le démarrage annonce « les 3 emplacements sont pris » : ne pas compiler ; attendre, ou proposer la tâche dans le cloud. Les sessions de documentation ne compilent pas et ne comptent pas.
- **Lecture seule** (revue, analyse, question) : session sans worktree (`use_worktree: false`), aucun fichier suivi modifié. **Morceaux d'une même tâche** (exploration, tests, relecture) : sous-agents dans la même copie, pas de nouvelle session.
- **Propriété** : chaque session ne modifie que les crates et répertoires nommés dans sa consigne. Fichiers partagés (`Cargo.toml` racine, `Cargo.lock`, `crates/ts_model/`, `crates/ts_game/`, `.github/`, `.cargo/`, `.claude/`, `tools/`, `CLAUDE.md`, socle `docs/conception/0*.md`) : seulement dans une PR courte et dédiée, fusionnée avant le travail qui en dépend. Exception : une ligne triée dans `[workspace.dependencies]` ou dans `standard_rules()`. Conflit sur `Cargo.lock` : reprendre la version de `main`, puis `cargo check`.
- **Client Godot** : une seule session à la fois sur `client/`.
- **Avant d'ouvrir une PR** : `cargo xtask check` (formatage, clippy, tests des crates touchées, déterminisme si le noyau change), puis `/code-review`. Pour `ts_net`, `ts_pilot`, `ts_persist`, `ts_client_native`, `ts_accounts`, `ts_admin`, `.github/`, `deny.toml`, `.gitleaks.toml` ou `.claude/` : ajouter `/security-review` et une relecture par un sous-agent neuf. Toute nouvelle dépendance est justifiée dans la PR.
- **Titre de PR et commits** : `type(portée): description en français` (types : feat, fix, refactor, perf, test, docs, build, ci, chore ; portée : la crate ou la zone).
- **Compilation** : jamais de `--release` ni de profil `sim` dans un worktree (copie principale et CI seulement). Ne jamais lancer `cargo clean` dans un emplacement tenu par une autre session. Compiler `ts_viz`, `ts_py` et `ts_client_native` seulement si la tâche les touche (`-p`).
- **Rien de régénérable ni de secret dans le dépôt** : artefacts de monde, Parquet, instantanés, sauvegardes, modèles GGUF et corpus vont dans `TS_CACHE` (`C:\Dev\ts-cache`) ; le dépôt ne garde que les spécifications et les empreintes attendues.
- **Fin de tâche** : une fois la PR fusionnée, la session s'archive, ce qui supprime son worktree. Aucun worktree inactif plus de 7 jours.
- **Disque** : sous 40 Go libres, ne plus créer de worktree et lancer `cargo xtask disk --clean`.
```

---

## 8. Configuration GitHub et intégration continue

### 8.1 Dépôt et compte

| Réglage | Valeur |
|---|---|
| Propriétaire, nom | Compte GitHub de l'utilisateur, `time_switcher`, **privé** |
| Offre | **GitHub Pro** (A3) |
| Branche par défaut | `main` |
| Fusions | Squash uniquement ; message = titre et description de la PR ; fusion automatique permise ; branches supprimées après fusion ; « Always suggest updating pull request branches » |
| *Ruleset* `main` | PR obligatoire (0 approbation) ; contrôle requis `ci-ok` (strict : branche à jour) ; historique linéaire ; force-push et suppression interdits ; aucun contournement |
| *Ruleset* `v*` (étiquettes) | Création réservée à la tâche de publication (phase 5) |
| Actions | Actions de GitHub et actions épinglées par empreinte seulement ; `GITHUB_TOKEN` en lecture par défaut ; aucun secret avant la phase 4 ; environnement `release` avec approbation de l'utilisateur pour la signature (S27, phase 5) |
| Sécurité | Alertes et mises à jour de sécurité Dependabot ; `gitleaks` en CI (la détection de secrets de GitHub est payante en privé) ; double authentification du compte, clé FIDO2 de préférence |
| Identité des commits | Adresse noreply de GitHub, dans la configuration du dépôt ; nom `Livenerst` (déjà configuré) |

### 8.2 Tâches de `ci.yml` (PR non brouillon et `main`)

| Tâche | Quand | Contenu |
|---|---|---|
| `changes` | Toujours | `git diff --name-only HEAD^1 HEAD` (`fetch-depth: 2`) donne les zones touchées : rust, python, web, client, ci, docs |
| `pr-title` | PR | Format conventionnel (script, sans action tierce) |
| `security` | Toujours | `gitleaks` sur le diff ; vérification de l'épinglage des actions ; `cargo-deny check` si rust ou ci ; `cargo-audit` si `Cargo.lock` change |
| `rust-linux` | rust ou ci | `cargo fmt --check` ; `cargo clippy --workspace --all-targets --locked -- -D warnings` ; `cargo test --workspace --locked --profile ci` (déterminisme 1 contre N fils, aller-retour d'instantané, invariants, scénarios d'exploits sur un morceau de ville) ; test « `public` + `dev-commands` ne compile pas » ; vérification de `forbid(unsafe_code)` |
| `rust-windows` | Chemins sensibles (A28), étiquette `ci:windows`, `main` | Comme `rust-linux`, sans `fmt` |
| `python` | python ou `ts_py` | `uv sync --locked` ; `cargo xtask py` ; `pytest` |
| `web` | web (phase 4) | `pnpm install --frozen-lockfile` ; lint ; tests ; build ; `pnpm audit` |
| `client` | client (phase 3) | Compilation C# en mode sans affichage de Godot [à définir en phase 3] |
| **`ci-ok`** | Toujours | `needs` de toutes les tâches, `if: always()` ; échoue si une tâche a échoué (une tâche sautée est acceptée). **Seul contrôle requis**, ce qui permet de sauter des tâches selon les chemins sans bloquer la fusion. |

Une PR qui ne touche que la documentation ne lance que `changes`, `pr-title`, `security` et `ci-ok`, en moins de deux minutes.

### 8.3 `nightly.yml` (chaque nuit, seulement si `main` a changé)

- Windows complet, avec le profil `sim` : déterminisme long et scénario doré d'un an de 1 000 entités (critère de la phase 0).
- `cargo-audit` (une nouvelle alerte peut tomber sans aucun changement de code).
- Fuzzing : 10 minutes par cible ; le dimanche, 60 minutes pour le codec d'instantané (A33).
- Plus tard : équivalence entre niveaux, stabilité longue, performance (revue d'architecture §21).
- En cas d'échec, la tâche ouvre ou met à jour une *issue* `nightly-failure` (seule permission d'écriture : `issues: write`), que la session principale traite le jour même.

### 8.4 Budget de minutes (offre Pro, environ 3 000 minutes par mois [à vérifier])

| Poste | Hypothèse | Minutes par mois |
|---|---|---|
| PR de code, Linux | 80 PR × 12 min | 960 |
| PR de documentation | 40 × 2 min | 80 |
| Windows sur chemins sensibles et `main` | 25 × 20 min × 2 | 1 000 |
| Nuit (Windows, audit, fuzzing) | 20 nuits × (15 × 2 + 15) | 900 |
| **Total** | | **≈ 2 940** [à mesurer] |

Si le budget est dépassé, dans l'ordre : passer les tâches de nuit sur un jour sur deux ; déplacer le fuzzing et les campagnes dans le cloud (A24) ; en dernier recours, ajouter des minutes payantes, décision de l'utilisateur. Un exécuteur auto-hébergé sur la machine de l'utilisateur est **écarté** : il consommerait le disque et le processeur que l'on cherche à préserver, et ouvrirait la machine aux tâches de CI.

### 8.5 Modèle de PR (`pull_request_template.md`)

Sections : **Quoi** · **Pourquoi** (décision de conception concernée, fichier et section) · **Vérifié par** (commandes lancées, dont `cargo xtask check` sous Windows) · **Nouvelles dépendances** (justification, S26) · **Sécurité** (zones sensibles touchées, relectures faites) · **Docs à mettre à jour**.

---

## 9. Procédure de nettoyage

### 9.1 Automatique

| Mécanisme | Effet |
|---|---|
| Archivage de la session à la fermeture de sa PR (réglage de l'application) | Supprime le worktree s'il est propre ; le bail de l'emplacement devient orphelin, donc libre |
| Archivage après 7 jours d'inactivité (réglage de l'application) | Même effet pour les sessions abandonnées |
| Suppression des branches distantes après fusion (GitHub) | Garde le dépôt distant propre |
| Nettoyage du cache global de Cargo (Cargo ≥ 1.88) | `~/.cargo/registry` reste borné |
| Crochet de démarrage | Avertit sous 40 Go libres |

### 9.2 Hebdomadaire, par la session principale : `cargo xtask disk --clean`

1. Affiche l'espace libre, la taille de chaque emplacement, de `TS_CACHE\artifacts`, de `.claude/worktrees` et des caches globaux, ainsi que la liste des baux et des worktrees.
2. `git worktree prune` (références de worktrees disparus).
3. Libère les baux orphelins.
4. Dans chaque emplacement **libre** : supprime `debug/incremental` (et celui des autres profils) si l'emplacement dépasse **6 Go**, et le vide entièrement (`cargo clean`) s'il dépasse **10 Go** [à calibrer]. Il ne touche jamais à un emplacement tenu.
5. Dans `target\main` : même règle, avec des seuils de 12 et 20 Go.
6. `artifacts\runs` : supprime les exécutions de plus de 30 jours, sauf celles marquées `keep` ; `artifacts\worlds` et `packs` : garde ceux qui sont cités par `data/worlds/*.ron`, plus les trois plus récents ; plafond total de 15 Go [à calibrer].
7. `uv cache prune` ; `pnpm store prune` (à partir de la phase 4).

### 9.3 Mensuelle, par la session principale

1. Outil de l'application : nettoyage des worktrees inactifs depuis 30 jours (carte d'approbation montrée à l'utilisateur ; ne supprime jamais de modification non commise sans accord explicite).
2. `git gc` dans la copie principale ; `git lfs prune` (à partir de la phase 3).
3. `rustup toolchain list` : supprime les chaînes qui ne sont plus épinglées.
4. Bilan du budget de minutes de la CI et du quota LFS.

### 9.4 En urgence (moins de 40 Go libres)

Dans l'ordre, en s'arrêtant dès que l'espace revient au-dessus de 60 Go : `cargo xtask disk --clean` → vider tous les emplacements libres → archiver les sessions terminées → purger `artifacts\runs` au-delà de 7 jours → compresser les emplacements (`compact /c /s`) → signaler la situation à l'utilisateur.

---

## 10. Mise en place pendant la phase 0

Étapes pour la session principale, dans l'ordre. Elles précèdent les étapes de code de la revue d'architecture §22 (phase 0), ou s'y mêlent.

1. **Relire et intégrer** ce document (§12) : `CLAUDE.md`, `05-architecture.md`, les revues d'architecture et de sécurité, le README de la conception.
2. **Initialiser Git** dans `C:\Dev\Claude\time_switcher` : `git init -b main` ; `git config core.longpaths true` ; `git config core.autocrlf false` ; `git config extensions.worktreeConfig true` ; adresse noreply. Écrire `.gitignore`, `.gitattributes` et un `README.md` court.
3. **Premier commit** (documentation seule) : `docs(conception): importe la conception terminée`. Vérifier auparavant qu'aucun fichier ne ressemble à un secret.
4. **Utilisateur** : souscrire GitHub Pro et vérifier la double authentification (§11).
5. **Créer le dépôt privé** et pousser (`gh repo create <compte>/time_switcher --private --source . --push`), avec l'accord de l'utilisateur dans la conversation, puisque c'est une action sortante.
6. **Configurer GitHub** par `gh api` : options de fusion, *ruleset* de `main` (le contrôle `ci-ok` s'ajoute dès que la CI existe), permissions des Actions, Dependabot.
7. **Réglages de l'application**, chacun approuvé par l'utilisateur sur une carte : `auto_archive_on_pr_close = true` ; `auto_archive_inactive_days = 7`.
8. **PR « outillage des agents »** : `tools/dev/session-start.ps1`, `guard-main.ps1`, `.claude/settings.json`, création de `C:\Dev\ts-cache\`. Faire les vérifications bloquantes du §6.3 ; à défaut, appliquer le repli.
9. **PR « espace de travail »** : `Cargo.toml` (membres, `default-members`, `exclude`, profils de A12, `[workspace.lints]`), `rust-toolchain.toml`, `.cargo/config.toml`, `deny.toml`, `clippy.toml`, `rustfmt.toml`, `tools/xtask` (`check`, `disk`, `slots`), `ts_kernel` vide avec `#![forbid(unsafe_code)]`, fonctionnalités `dev-commands`, `dev-introspection`, `external-pilot` et `public` avec leur `compile_error!`.
10. **PR « CI »** : `ci.yml` avec `ci-ok`, `nightly.yml`, `dependabot.yml`, `.gitleaks.toml`, modèle de PR ; actions épinglées par empreinte. Puis ajouter `ci-ok` comme contrôle requis du *ruleset*.
11. **Preuves de blocage** (critère de la phase 0, S26) : une PR jetable qui ajoute une dépendance à vulnérabilité connue, et une autre qui contient un faux secret au format reconnu par `gitleaks` (jamais une vraie clé). Les deux doivent être **rouges** ; on les ferme sans fusion et on supprime les branches.
12. **Mesures** : taille d'un emplacement après `cargo xtask check` ; taille de la copie principale après une compilation `sim` ; temps de première compilation dans un emplacement froid et dans un emplacement chaud ; minutes de CI par PR. Reporter les valeurs dans ce document, à la place des [à mesurer].
13. **Code de la phase 0** (revue d'architecture §22) : `ts_kernel` d'abord, dans une seule session ; puis, en parallèle, `ts_model` + `ts_data`, `ts_runner` + test de déterminisme, `ts_py` minimal, `ts_viz` minimal, en respectant A21 et A23.

---

## 11. Ce que l'utilisateur fait ou valide lui-même

L'utilisateur ne lit pas le code et n'approuve aucune PR : la CI et les relectures automatiques en tiennent lieu. Lui restent les actions que Claude ne doit pas faire seul :

| Quand | Action | Pourquoi l'utilisateur |
|---|---|---|
| Avant l'étape 5 | **Souscrire GitHub Pro** (environ 4 $ par mois [à vérifier]) | Paiement |
| Avant l'étape 5 | Vérifier que la **double authentification** est active sur le compte GitHub (clé FIDO2 de préférence) | Réglage de sécurité du compte |
| Étape 5 | Donner son accord à la création du dépôt privé et au premier envoi | Action sortante |
| Étape 7 | Approuver les cartes des réglages d'archivage automatique | Réglages de l'application |
| Chaque mois | Approuver la carte de nettoyage des worktrees inactifs | Suppression |
| Phase 3 | Approuver l'installation de Godot 4 (.NET) et d'un SDK .NET récent ; les anciens SDK .NET 3.1 et 5.0 (environ 1 Go) peuvent être désinstallés s'ils ne servent à aucun autre projet | Logiciels de la machine |
| Facultatif | Examiner le disque `H:` (HDD de 1 To illisible) ; s'il est vide, le formater en NTFS pour des archives froides | Risque de perte de données |
| Phase 5 | Détenir la clé matérielle de signature et approuver les publications (environnement `release`, S27) | Intégrité de la production |
| À chaque jalon | Juger les démonstrations de phase et les décisions de conception | Rôle de concepteur |

---

## 12. Modifications proposées à CLAUDE.md, au socle et aux revues

### 12.1 [CLAUDE.md](../../../CLAUDE.md)

- **Ajouter** la section « Développement par agents » du §7, telle quelle, après « Organisation du travail ».
- **Dans « Organisation du travail »**, ajouter une puce : « Depuis la phase 0, le dossier est un dépôt Git privé sur GitHub ; toute modification, documentation comprise, passe par une branche et une PR (voir « Développement par agents »). »
- **Dans « Architecture »**, ajouter une puce : « **Dépôt et outils** (docs/conception/revues/developpement-agents.md) : monodépôt, `cargo xtask`, `uv`, `pnpm`, Git LFS limité aux assets ; artefacts régénérables hors du dépôt (`TS_CACHE`) ; emplacements de compilation partagés par jonction ; CI Linux à chaque PR, Windows en local, sur les chemins sensibles et chaque nuit. »

### 12.2 [README.md](../README.md) de la conception

- Dans « Déroulement de la conception », ajouter l'étape 6 : « Une **revue du développement par agents** ([revues/developpement-agents.md](revues/developpement-agents.md)) fixe le dépôt, GitHub, la CI et les règles de travail des agents, selon le même principe (document à part, relu puis intégré). »
- Ajouter une ligne au tableau des relectures de cohérence une fois l'intégration faite.

### 12.3 [05-architecture.md](../05-architecture.md)

| Endroit | Modification |
|---|---|
| §4 Langages | Ajouter une ligne « Outillage » : « `cargo xtask` (Rust) pour les tâches ; `uv` pour Python ; `pnpm` pour la console ; `protox` pour les schémas protobuf. » |
| §5, phase 0 | Ajouter au début : « dépôt Git privé sur GitHub, `main` protégée par le contrôle `ci-ok`, règles de travail des agents et emplacements de compilation ([revues/developpement-agents.md](revues/developpement-agents.md), §10) ». Ajouter au critère de fin : « et un worktree neuf compile dans un emplacement partagé ». |
| §2.25 Sécurité | Remplacer « dépendances verrouillées et auditées » par « dépendances verrouillées (`Cargo.lock`, `uv.lock`, `pnpm-lock.yaml`) et auditées ». |

### 12.4 [revues/architecture.md](architecture.md)

| Endroit | Modification |
|---|---|
| §5.1 | Compléter l'arborescence avec `tools/xtask`, `tools/dev`, `fuzz/`, `.claude/`, `.github/`, et les sous-dossiers de `data/` (`content`, `population`, `scenarios` avec `arenes` et `exploits`, `worlds`, `golden`, `models`), selon le §5.1 de ce document. Ajouter : « `default-members` exclut `ts_viz`, `ts_py`, `ts_client_native` et `xtask` ; `fuzz/` est hors de l'espace de travail. » |
| §18.7 | Après « Les gros fichiers sont gérés par Git LFS », ajouter : « non extraits par défaut dans les worktrees (`lfs.fetchexclude`) ; les modèles GGUF ne passent jamais par Git (`cargo xtask fetch-models`, empreinte épinglée). » |
| §19.2 | Ajouter : « Le répertoire de run est donné par `--out` ; par défaut, les outils utilisent `TS_CACHE/artifacts/runs`, hors du dépôt. » |
| §21 | Remplacer la dernière phrase par : « CI Linux à chaque modification ; Windows vérifié par l'agent avant chaque PR, par la CI sur les chemins sensibles et sur `main`, et chaque nuit. Les empreintes dorées valent pour un couple (profil, plateforme). `clippy` avec les interdictions du §9.2 ; formatage automatique. » |
| §22, phase 0, point 1 | Remplacer par : « Dépôt Git et GitHub, outillage des agents, espace de travail Cargo, CI Linux et Windows, règles `clippy` ([developpement-agents.md](developpement-agents.md), §10). » |

### 12.5 [revues/securite.md](securite.md)

| Endroit | Modification |
|---|---|
| S26, première puce | Remplacer « `package-lock.json` » par « `pnpm-lock.yaml` », « `npm ci --ignore-scripts` » par « `pnpm install --frozen-lockfile` (scripts des dépendances refusés sauf liste `onlyBuiltDependencies`) », et « `pip install --require-hashes` » par « `uv sync --locked` ». |
| S26, deuxième puce | Remplacer « `npm audit` » par « `pnpm audit` ». |
| S26, puce `gitleaks` | Ajouter : « (la détection de secrets de GitHub est payante dans un dépôt privé) ; contrôle requis unique `ci-ok` sur `main` protégée ; `GITHUB_TOKEN` en lecture par défaut. » |
| S26, puce fuzzing | Remplacer « tâche de nuit » par « 10 minutes par cible chaque nuit, 60 minutes chaque semaine ». |
| §7, phase 0 | Ajouter : « 7. `main` protégée (PR obligatoire, `ci-ok` requis, ni force-push ni contournement) ; vérification automatique de l'épinglage des actions (A26, A29). » |
| S28 | Préciser : « le dépôt de déploiement est un dépôt privé distinct (`time_switcher-ops`), créé en phase 4 et jamais ouvert dans une session d'agent ordinaire. » |

### 12.6 [revues/equilibrage.md](equilibrage.md)

Aucune modification de fond. Les scénarios d'exploits de E39 vivent dans `data/scenarios/exploits/` et tournent dans la tâche `rust-linux` à chaque PR, puis en entier chaque nuit.

### 12.7 Domaines

Aucune modification.

### 12.8 Mémoire de Claude

Hors du dépôt et hors de mon périmètre. Il serait utile que la session principale y note : « Copie principale jamais éditée ; travailler dans un worktree ; au plus 3 sessions qui compilent. »

---

## 13. Risques résiduels

| Risque | Parade |
|---|---|
| La suppression d'un worktree vide l'emplacement à travers la jonction | Vérification bloquante du §6.3 ; au pire, une recompilation ; repli par `CARGO_TARGET_DIR` |
| Un agent compile sans emplacement (crochet absent, quatrième session) | Avertissement au démarrage ; `cargo xtask disk` signale tout `target/` qui n'est pas une jonction ; nettoyage hebdomadaire |
| Quota de minutes de CI dépassé | Ordre de repli du §8.4 ; Windows en local |
| Un bogue propre à Windows atteint `main` entre deux nuits | `cargo xtask check` local obligatoire ; *issue* de nuit corrigée le jour même |
| Les règles par agent ne sont qu'une convention (un agent peut éditer par une commande shell) | Le *ruleset* GitHub est la vraie barrière ; le crochet n'est qu'un filet de sécurité |
| Tarifs et offres GitHub modifiés | Valeurs marquées [à vérifier] ; repli de A3 |
| Les fichiers verrouillés par Windows dans la copie principale (jeu lancé pendant une compilation) | Une seule compilation à la fois dans la copie principale ; fermer le jeu avant `cargo xtask client-native` |
