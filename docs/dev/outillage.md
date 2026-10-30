# Outillage des agents

Mode d'emploi de `tools/dev/` (tâche T1 de la phase 0). Sources : [revue du développement par agents](../conception/revues/developpement-agents.md) §4.2, §4.4, §6, et les exigences SEC0-06 à SEC0-14 et SEC0-35 du carnet du responsable architecture et sécurité.

`TS_CACHE` vaut `C:\Dev\ts-cache` par défaut ; la variable d'environnement `TS_CACHE` le change.

## 1. Emplacements de compilation

Le crochet `SessionStart` lance `tools/dev/session-start.ps1` à chaque démarrage de session. Le script ne bloque jamais et peut tourner plusieurs fois sans effet de bord.

- **Copie principale** : `target` devient une jonction vers `TS_CACHE\target\main`.
- **Worktree** : le script prend un **bail** sur `w1`, `w2` ou `w3`, puis crée la jonction `target`.
  - Le bail est un fichier `wN.lease`, créé de façon exclusive, qui contient le chemin du worktree.
  - Un bail est **tenu** s'il a moins de 10 minutes, ou si son worktree existe encore (fichier `.git`) et que sa jonction `target` pointe vers l'emplacement.
  - Sinon, il est **orphelin** : il est libéré par renommage atomique, puis repris.
- **Bail déjà à son nom** : le worktree reprend son emplacement. C'est le cas après un `git clean -xdf`, qui retire la jonction.
- **Aucun emplacement libre** : avertissement « les 3 emplacements sont pris ». La session ne compile pas : elle attend, ou la tâche passe au cloud.
- **Disque** : sous 40 Go libres, avertissement (A34).
- **Libérer un emplacement à la main** : supprimer `TS_CACHE\target\wN.lease`, seulement si le worktree qu'il nomme n'existe plus.

**Repli `CARGO_TARGET_DIR`** (§6.3 point 4). Au-delà de 100 caractères de racine de worktree, le script écrit `export CARGO_TARGET_DIR='<emplacement>'` dans `CLAUDE_ENV_FILE`, pour les commandes Bash de la session, et rappelle la commande PowerShell équivalente. Raison : `link.exe` (MSVC 14.28) échoue au-delà de 260 caractères (voir §5).

## 2. Crochets Git

Les crochets vivent dans `tools/dev/hooks/`. Ils s'exécutent depuis une **copie épinglée hors du dépôt**, `TS_CACHE\hooks\<sha256>\`, désignée par `core.hooksPath` (SEC0-12).

| Crochet | Contrôles |
|---|---|
| `pre-commit` | Refus de **tout commit sur `main`**, sans variable d'exception (SEC0-13).<br>Refus d'un fichier de plus de 5 Mo (SEC0-11).<br>Refus, hors LFS, des extensions `.glb .gltf .png .jpg .ogg .wav .mp3 .otf .ttf .onnx .gguf .pt .safetensors .parquet .sqlite .db .blend .psd .fbx .zip .7z`. Exception : `**/tests/fixtures/**`, jusqu'à 256 Ko.<br>`gitleaks` sur l'index, sur toutes les branches, `pilotage` comprise (SEC0-08). |
| `pre-merge-commit` | Mêmes contrôles qu'un commit, pour un `git merge` qui crée un commit de fusion (décidé : sinon, une fusion sur `main` échapperait au refus). |
| `pre-push` | Refus de pousser vers `main` ; refus d'une poussée forcée (historique distant non ancêtre) ; refus de supprimer `pilotage`.<br>`gitleaks` sur la plage poussée : `distant..local`, ou, pour une branche nouvelle, les commits absents du distant. |
| `commit-msg` | Format `type(portée): description`, avec un type parmi `feat fix refactor perf test docs build ci chore` (A31). Restent permis : `Merge …`, `Revert "…"`, `fixup!`, `squash!` et `amend!`. |

`gitleaks` est appelé par son chemin épinglé dans `tools/dev/tools.lock`. Son SHA-256 est **vérifié avant chaque usage** : si l'outil manque ou si l'empreinte diffère, le crochet refuse (SEC0-06). Le fichier garde aussi l'empreinte de l'archive officielle, contrôlée par T0 contre le fichier de sommes de la version 8.30.1.

**Installation (provisoire, remplacée en T3 par `cargo xtask hooks install`)**, depuis n'importe quelle copie de travail :

```powershell
pwsh -NoProfile -File tools/dev/install-hooks.ps1           # installe et règle core.hooksPath (configuration commune)
pwsh -NoProfile -File tools/dev/install-hooks.ps1 -Verify   # contrôle seulement
```

Le script fait les opérations suivantes :
- il vérifie gitleaks ;
- il copie les crochets et `tools.lock` dans `TS_CACHE\hooks\<empreinte du jeu>\`, en lecture seule ;
- il règle `core.hooksPath` dans `.git/config`, commun à toutes les copies de travail ;
- il écrit l'empreinte attendue dans `TS_CACHE\hooks\current.sha256`.

L'empreinte du jeu est le SHA-256 de la liste `nom NUL sha256 LF` des fichiers, triée par nom en ordre ordinal. Toute modification de `tools/dev/hooks/` ou de `tools.lock` demande de réinstaller.

## 3. Garde-fous de Claude Code

Les deux crochets `PreToolUse` sont inscrits dans `.claude/settings.json` du projet, avec `SessionStart` (AS-15). La commande est lancée par Git Bash, l'interpréteur par défaut des crochets sous Windows :

```
pwsh -NoProfile -NonInteractive -File "${CLAUDE_PROJECT_DIR}/tools/dev/<script>.ps1"
```

Ce format a été vérifié le 30/10 par une session `claude -p` (Claude Code 2.1.302) dans un projet jetable : le crochet `SessionStart` a créé la jonction, et `guard-git.ps1` a refusé `git push --force`.

**`guard-main.ps1`** (`Edit|Write|NotebookEdit`) est un garde-fou de commodité (A19). Il refuse :
- l'écriture dans la copie principale, hors de `.claude/worktrees/`. La copie principale est le parent du dossier Git commun ;
- l'écriture de `.claude/settings.json` et `.claude/settings.local.json` (SEC0-35, ci-dessous).

Les fichiers hors du dépôt restent permis : mémoire, notes, *scratchpad*, worktree `pilotage`. En cas d'erreur interne, il laisse passer avec un avertissement.

**`guard-git.ps1`** (`Bash|PowerShell`) refuse :
- `git commit` sur `main` (branche du dossier visé : `-C`, `cd`, ou dossier courant) ;
- `git push` :
  - vers `main` : `main`, `HEAD:main`, `refs/heads/main`, `:main`, ou `HEAD` et poussée sans destination depuis `main` ;
  - une destination préfixée par `+` ;
  - `--force`, `-f` (y compris groupé, comme `-uf`), `--force-with-lease`, `--force-if-includes` et leurs abréviations ;
  - `--all` et `--mirror` ;
  - la suppression de `pilotage` ;
- `--no-verify` et ses abréviations, ainsi que `git commit -n` ;
- `-c core.hooksPath`, `git config core.hooksPath` (sauf `--get`), et `GIT_CONFIG_*` avec `hooksPath` ;
- `git update-ref` sur `main` ou `--stdin` ;
- `git branch -f/-d/-D/-m/-c main` ;
- `git checkout -B main` et `git switch -C main` ;
- `gh api` sur `git/refs`.

Le worktree `pilotage` reste permis : commit et poussée de la branche `pilotage`. Chaque commande est lue deux fois :
- une lecture stricte, qui respecte guillemets et échappements de Bash ou de PowerShell, et découpe sur `&&`, `||`, `;`, `|`, `&`, les parenthèses et les fins de ligne ;
- une lecture naïve, qui ignore les guillemets, pour attraper `bash -c "…"`, `pwsh -Command "…"` et `$(…)`.

Un refus de l'une suffit. En cas d'erreur interne, la commande est refusée.

**Intégrité de la configuration de Claude Code (SEC0-35, point 1).** Sont refusés, dans toute copie de travail :
- `Edit`, `Write` et `NotebookEdit` sur `.claude/settings.json` et `.claude/settings.local.json` ;
- toute commande shell qui écrit dans ces fichiers : redirection `>` ou `>>`, `Set-Content`, `Out-File`, `Add-Content`, `tee`, `cp`, `mv`, `rm`, `Copy-Item`, `Move-Item`, `Remove-Item`, `sed -i`, `git checkout`/`restore` du fichier, `[IO.File]::Write…`, etc. ;
- toute commande qui mentionne `disableAllHooks`.

La seule exception est une copie de travail dont la branche courante (`git branch --show-current`) est `claude/outillage-*` : sa PR est un chemin sensible. Les réglages utilisateur (`~/.claude/settings.json`) sont aussi protégés (décidé : aucune session ne les modifie, organisation §6). La vérification de toutes les copies de travail (`xtask hooks verify`) revient à T3.

**Coût** : chaque appel `Bash` ou `PowerShell` lance `pwsh`, soit environ 1,2 à 1,8 s, dont 0,8 s pour le seul démarrage de `pwsh`. Un garde compilé dans `xtask` (T3) le ramènerait à quelques dizaines de millisecondes.

**Ce que les garde-fous n'attrapent pas** : alias Git, variables (`git push origin $B`), scripts écrits puis exécutés, interpréteurs (`python -c`). Ils protègent de l'erreur et de l'injection simple, pas d'une session décidée à contourner.

**Risque résiduel (R-ARCH-04)** : une session peut modifier le `.claude/settings.json` de son propre worktree, par un moyen que le crochet ne voit pas, et retirer ainsi ses garde-fous. Restent alors :
- le `pre-commit` et le `pre-push` épinglés hors du dépôt, qui refusent le commit et la poussée vers `main` ;
- la CI (`ci-ok`) ;
- `xtask merge`, avec la vérification d'intégrité de SEC0-35 (T3).

## 4. Tests

Tous travaillent dans des dépôts jetables sous `TS_CACHE\tmp\`, jamais dans le vrai dépôt :

```powershell
pwsh -NoProfile -File tools/dev/test-guards.ps1        # table des commandes refusées et permises (guard-git, guard-main)
pwsh -NoProfile -File tools/dev/test-hooks.ps1         # crochets Git et install-hooks, dans un TS_CACHE jetable
pwsh -NoProfile -File tools/dev/test-slots.ps1         # logique des baux, dans un TS_CACHE jetable
pwsh -NoProfile -File tools/dev/test-slots.ps1 -Real   # plus les vérifications du §6.3, dans les vrais emplacements
```

## 5. Vérifications bloquantes de §6.3 (30/10/2026)

Conditions de l'essai :
- `test-slots.ps1 -Real` ;
- un worktree jetable d'un dépôt jetable, avec une racine de 90 caractères, soit celle des worktrees de l'application plus 20 de marge ;
- un projet Cargo jetable ;
- l'emplacement réel `w1`.

| Point | Résultat |
|---|---|
| Attribution d'un emplacement | **Conforme** : `w1` attribué, jonction créée, bail au nom du worktree. |
| Crochet `SessionStart` lancé par Claude Code (point 2) | **Conforme** : vérifié par `claude -p` (§3) ; `pwsh` 7.6 disponible. |
| Compilation à travers la jonction (point 3) | **Conforme** : `cargo test` réussit, avec un chemin le plus long de 254 caractères. 43 fichiers sont écrits dans l'emplacement.<br>**Limite constatée** : à 300 caractères et plus, `link.exe` (MSVC 14.28) échoue avec `LNK1104`, faute de prise en charge des chemins longs. Le même projet compile avec `CARGO_TARGET_DIR` vers l'emplacement. D'où le repli automatique au-delà de 100 caractères de racine (§1). |
| `git clean -xdf` | **Conforme** : la jonction est retirée **sans suivre le lien**, et l'emplacement reste intact (140/140 fichiers). Le démarrage suivant reprend le même emplacement. |
| `git worktree remove` | **Conforme** : réussit sans `--force`, l'emplacement reste intact (140/140 fichiers).<br>**Particularité** : Git retire le worktree et son `.git`, mais **laisse le dossier et la jonction**. Le bail devient orphelin, puisque le `.git` a disparu, et il est libéré au démarrage suivant d'une autre session. Le dossier laissé se supprime sans risque après avoir retiré la jonction seule : `[IO.Directory]::Delete("<worktree>\target")`, jamais un `Remove-Item -Recurse` sur la jonction. Ce nettoyage revient à `cargo xtask disk --clean` (T3). |
| Archivage par l'application | Non essayé ici, puisqu'aucune session n'a été archivée. Il passe par `git worktree remove` : même comportement attendu. Le premier archivage réel est à observer. |

Le mécanisme de jonction est **adopté** (A9) ; le repli `CARGO_TARGET_DIR` sert seulement aux racines longues. Le bail de l'essai a été libéré, et l'emplacement débarrassé des fichiers de l'essai.
