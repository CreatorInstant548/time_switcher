# Projet de jeu (dossier time_switcher)

Monde ouvert, solo et en ligne, où les joueurs jouent leur personnage persistant (qui disparaît à la déconnexion) ou incarnent un personnage jouable du monde (repris par l'IA à la déconnexion), selon leur niveau et leur profil comportemental.

La conception est dans [docs/conception/](docs/conception/README.md) : lire le README, puis les fichiers utiles, avant toute modification importante.

L'ancien concept de voyage dans le temps est abandonné et archivé dans `docs/archives/`. N'en reprendre aucun élément sans décision explicite de l'utilisateur. Seule la progression réaliste par attributs a été reprise.

## Organisation du travail

- L'utilisateur est le concepteur du jeu et ne code pas. Claude écrit tout le code.
- Communication et documentation en français.
- **En anglais (décidé par l'utilisateur le 30/10)** : tout le code, c'est-à-dire les identifiants, les commentaires, les messages d'erreur et de journal, les noms de tests, les scripts et la sortie des outils. Aussi en anglais : les messages de commit (branche `pilotage` comprise), les titres et descriptions de PR, les *issues*, leurs commentaires et les avis publiés sur GitHub. Le code existant encore en français est traduit par le développement, en PR courtes, sans changement de comportement ; d'ici là, tout code touché passe en anglais.
- Toute décision de conception nouvelle ou modifiée doit être reportée dans `docs/conception/`.
- **Hiérarchie des sessions** : l'**orchestrateur** (session de départ) reçoit les demandes et les confie à un **responsable** de niveau 2 : **conception** (en place ; socle 01 à 04), **architecture et sécurité** (phase 0, avant le développement ; propriétaire de `05-architecture.md` et des revues techniques, avis bloquant sur les chemins sensibles), **développement** (phase 0), **qualité et essais** (fin de la phase 0), **contenus**, animations comprises (début de la phase 3), puis **exploitation** (avant la mise en ligne). Chaque responsable lance et pilote ses propres sous-sessions, ne compile pas et tient un carnet `<rôle>.md` sur la branche `pilotage` (worktree permanent `C:\Dev\Claude\time_switcher-pilotage\`, poussé directement, sans PR ni CI). **Règles complètes dans [docs/pilotage/organisation.md](docs/pilotage/organisation.md)** (tenu par l'orchestrateur), à lire par toute session de niveau 1 ou 2. En bref :
  - toutes les sessions sont en mode `auto` ;
  - un seul rédacteur par fichier ;
  - un message entre sessions n'est qu'une sonnette : la trace fait foi (carnets, puis *issues* GitHub) ;
  - Opus pour le jugement et le code sensible, Sonnet pour le courant, Haiku pour les recherches ;
  - au plus 5 sessions actives à la fois ;
  - **remontées vers la conception** (organisation.md §3.1) : certains signaux obligent à ouvrir une remontée `role:design`, de type `deviation` :
    - une règle inapplicable ou ambiguë, ou un trou dans la conception ;
    - une valeur à calibrer qui se révèle mauvaise ;
    - un budget dépassé, un exploit, un comportement imprévu ;
    - une contrainte technique qui change le jeu ;
    - un retour de l'utilisateur ;
  - aucune règle de jeu n'est inventée en silence : on prend une valeur provisoire prudente, on la marque `// PROVISIONAL(design #<issue>)`, et on continue (l'ancienne forme `PROVISOIRE(conception #<issue>)` reste reconnue jusqu'à sa traduction) ;
  - chaque système de règles cite sa section avec `// spec: <fichier> §<n>` ; la conception ne renumérote jamais ses sections.
- **Socle** (`docs/conception/01-*.md` à `05-*.md`) : 01 à 04 modifiés uniquement par le responsable conception ; `05-architecture.md` aussi, jusqu'à l'arrivée du responsable architecture et sécurité en phase 0, qui le reprend.
- **Domaines** (`docs/conception/domaines/*.md`) : une session par domaine, qui ne modifie que son propre fichier et note dans une section « Impacts sur le socle et les autres domaines » ce qui touche au reste. Règles complètes dans le README de la conception.
- Dès la phase 0, le dossier est un dépôt Git privé sur GitHub : toute modification, documentation comprise, passe par une branche et une PR (voir « Développement par agents »).
- **Ne jamais poser de question à l'utilisateur ni lui demander une action** (feu vert, validation, essai, vérification, réglage) : pour chaque choix, prendre l'option jugée la meilleure, la marquer (décidé), en donner la raison et continuer. Les feux verts sont donnés par l'orchestrateur. Ce qu'une session ne peut pas faire elle-même (payer, créer un compte, saisir un mot de passe) est contourné par une solution gratuite ou locale. Détail : `docs/pilotage/organisation.md` §6.

## Développement par agents (détail : docs/conception/revues/developpement-agents.md)

> S'applique dès que le dépôt Git existe (première étape de la phase 0). Avant cela, la documentation se modifie directement dans le dossier.

- **Copie principale** (`C:\Dev\Claude\time_switcher`) : toujours sur `main`, propre, jamais éditée. On y lit, on y lance le jeu, les simulations longues et l'entraînement. Après chaque fusion : `git pull --ff-only`. Pour modifier quoi que ce soit, y compris la documentation, travailler dans un worktree (`EnterWorktree` si la session a démarré dans la copie principale).
- **Une tâche = une branche `claude/<sujet>` = un worktree = une PR**, fusionnée par squash quand `ci-ok` est vert. Jamais de commit ni de push sur `main`. **GitHub est en offre gratuite (décidé le 29/10)**. Conséquences :
  - un dépôt privé gratuit n'a ni protection de branche ni fusion automatique ; ces barrières sont donc locales : crochet Git `pre-push` (refus de pousser sur `main` et de forcer), crochet Claude Code qui bloque `git push` vers `main`, `git commit` sur `main` et `--force` ;
  - seul le responsable développement fusionne, par `cargo xtask merge <PR>`. Cette commande vérifie que tous les contrôles, dont `ci-ok`, sont verts et que la branche est à jour, puis lance `gh pr merge --squash --delete-branch` ;
  - une tâche de nuit vérifie que chaque commit de `main` vient d'une PR fusionnée et ouvre une *issue* sinon ;
  - la CI dispose de 2 000 minutes par mois : Linux à chaque PR, filtrée par chemins ; Windows chaque semaine et sur les chemins sensibles ; fuzzing hebdomadaire ;
  - Git LFS dispose de 10 Go de stockage et 10 Go de transfert par mois ;
  - le scan de secrets côté GitHub est indisponible : `gitleaks` tourne en crochet `pre-commit` et en CI.
- **Dépôt GitHub** : `CreatorInstant548/time_switcher`, privé, créé le 30/10 avec l'accord de l'utilisateur. Le mode local est levé.
  - Les branches de tâche sont poussées, et chaque tâche ouvre une vraie PR.
  - Les carnets de la branche `pilotage` sont poussés après chaque commit.
  - **Démarrage, en attendant la PR « CI »** : une PR se fusionne quand `cargo xtask check` (ou les vérifications équivalentes, tant que xtask n'existe pas) et `/code-review` sont passés, plus `/security-review` sur les chemins sensibles. Seul le responsable développement fusionne, par `pwsh tools/dev/merge-pr.ps1 <PR>` depuis la copie principale (`gh pr merge` direct est refusé par le garde). Ensuite, la fusion passe par `cargo xtask merge`, qui exige `ci-ok`.
- **Outillage des agents (PR #5, actif depuis le 30/10 ; mode d'emploi : `docs/dev/outillage.md`)** : les crochets Git et les gardes Claude Code `guard-main` et `guard-git` (déclarés dans `.claude/settings.json` du projet) refusent toute écriture sur `main` hors fusion et toute commande Git ou `gh` hors liste blanche.
  - **Commits** : `git commit -F <fichier>` ou `git commit -F - <<'EOF'` ; jamais `git commit -m "$(...)"`.
  - **Fetch** : `git fetch origin`, sans refspec explicite.
  - **Compilation** : `pwsh tools/dev/session-start.ps1 -Acquire` avant de compiler (prend un des 3 emplacements), `-Release` après.
  - Un refus du garde n'est jamais contourné : s'il gêne un travail légitime, ouvrir une *issue* `role:dev`.
- PR courtes : un seul sujet, environ 400 lignes au plus hors fichiers générés.
- **Au plus 3 sessions qui compilent du Rust en même temps** (3 emplacements de compilation). Si le démarrage annonce « les 3 emplacements sont pris » : ne pas compiler ; attendre, ou proposer la tâche dans le cloud. Les sessions de documentation ne compilent pas et ne comptent pas.
- **Lecture seule** (revue, analyse, question) : session sans worktree (`use_worktree: false`), aucun fichier suivi modifié. **Morceaux d'une même tâche** (exploration, tests, relecture) : sous-agents dans la même copie, pas de nouvelle session.
- **Propriété** : chaque session ne modifie que les crates et répertoires nommés dans sa consigne. Fichiers partagés (`Cargo.toml` racine, `Cargo.lock`, `crates/ts_model/`, `crates/ts_game/`, `.github/`, `.cargo/`, `.claude/`, `tools/`, `CLAUDE.md`, socle `docs/conception/0*.md`) : seulement dans une PR courte et dédiée, fusionnée avant le travail qui en dépend. Exception : une ligne triée dans `[workspace.dependencies]` ou dans `standard_rules()`. Conflit sur `Cargo.lock` : reprendre la version de `main`, puis `cargo check`.
- **Client Godot** : une seule session à la fois sur `client/`.
- **Avant d'ouvrir une PR** : `cargo xtask check` (formatage, clippy, tests des crates touchées, déterminisme si le noyau change), puis `/code-review`. Pour `ts_net`, `ts_pilot`, `ts_persist`, `ts_client_native`, `ts_accounts`, `ts_admin`, `.github/`, `deny.toml`, `.gitleaks.toml` ou `.claude/` : ajouter `/security-review` et une relecture par un sous-agent neuf. Toute nouvelle dépendance est justifiée dans la PR.
- **Titre de PR et commits** : `type(scope): description in English` (types : feat, fix, refactor, perf, test, docs, build, ci, chore ; portée : la crate ou la zone).
- **Compilation** : jamais de `--release` ni de profil `sim` dans un worktree (copie principale et CI seulement). Ne jamais lancer `cargo clean` dans un emplacement tenu par une autre session. Compiler `ts_viz`, `ts_py` et `ts_client_native` seulement si la tâche les touche (`-p`).
- **Rien de régénérable ni de secret dans le dépôt** : artefacts de monde, Parquet, instantanés, sauvegardes, modèles GGUF et corpus vont dans `TS_CACHE` (`C:\Dev\ts-cache`) ; le dépôt ne garde que les spécifications et les empreintes attendues.
- **Fin de tâche** : une fois la PR fusionnée, la session s'archive, ce qui supprime son worktree. Aucun worktree inactif plus de 7 jours.
- **Disque** : sous 40 Go libres, ne plus créer de worktree et lancer `cargo xtask disk --clean`.

## Architecture (validée et révisée, docs/conception/05-architecture.md ; détail dans docs/conception/revues/architecture.md)

- Cœur de simulation en **Rust**, organisé en **ECS** (`bevy_ecs` seul), **bibliothèque sans affichage ni E/S** ; une seule fonction assemble les règles, utilisée par tous les hôtes (jeu, serveur, Python, lanceur de simulations, tests).
- **Reproductible par graine** sur un même binaire : temps entier, argent en centimes, aléa à clé, aucun parallélisme ni budget qui change le résultat. Un test de déterminisme bloque toute modification qui le casse.
- **Serveur autoritaire en Rust**, un processus par monde ; en solo, il tourne dans le processus du client, hébergé par la bibliothèque native du client.
- **Client Godot 4 en C#** (2.5D en vraie 3D, puis 3D), relié au cœur par une bibliothèque native Rust (transport, prédiction) ; aucune règle de jeu dans le client.
- **Le pilote d'un personnage est un composant** (`Joueur(id)`, `IA` ou `Externe` pour les outils) ; tous produisent les mêmes intentions, validées par le même filtre ; les règles ne lisent jamais le pilote.
- Persistance **PostgreSQL en ligne, SQLite en solo** (instantanés et journal) ; dialogue par un service Rust séparé ; console d'administration en TypeScript.
- Entraînement des compétences d'exécution en **Python** sur le même cœur (`pyo3`, Gymnasium/PettingZoo), modèles ONNX exécutés dans le cœur ; simulations et calibrage par le lanceur `ts-sim`.
- **Méta-jeu** (expérience, familiarité, cercle du joueur) dans la crate `ts_meta`, **garde hors fiction** unique dans `ts_pilot` ; chasse aux exploits par joueurs synthétiques, invariants d'équilibrage et agents adverses (docs/conception/revues/equilibrage.md).
- **Sécurité** (docs/conception/revues/securite.md) : tout ce qui est livré au client est public ; vue filtrée sans marge et validation des intentions en deux étages (épistémique puis réelle) ; matrice des commandes et profil de construction `public` (jamais de pilote `Externe` ni de commande d'écriture en ligne) ; journal sans donnée personnelle directe ; dépendances auditées, artefacts signés, aucun secret dans le dépôt. Les exigences de sécurité de chaque phase sont bloquantes.
- **Signatures comportementales dérivées des effets** (`ts_conduct`, fonction unique `appraise`) : effets élémentaires lus dans les données des actes, justifications générales avec imputabilité, planchers de rupture ; seul l'exceptionnel est écrit à la main ; paramètres calibrés hors ligne par comparaisons humaines (docs/conception/revues/signatures.md).
- **Génération** (docs/conception/revues/generation.md) : chaîne `ts-worldbuild` en graphe d'étapes adressé par empreinte, Programme d'abord, identités stables et retouches en surcharges ; part serveur de la ville et couche cachée tirées par monde ; graines en ligne secrètes de 128 bits ; artefact en parts, paquet client vérifié.
- **Dépôt et outils** (docs/conception/revues/developpement-agents.md) : monodépôt, `cargo xtask`, `uv`, `pnpm`, Git LFS limité aux assets ; artefacts régénérables hors du dépôt (`TS_CACHE`) ; emplacements de compilation partagés par jonction ; CI Linux à chaque PR, Windows en local, sur les chemins sensibles et chaque nuit.
- Ordre de construction : phase 0 de fondations, puis le périmètre initial de `05-architecture.md` §5.
