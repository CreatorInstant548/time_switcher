# Outillage des agents

Mode d'emploi de `tools/dev/` (tâche T1 de la phase 0). Sources :
- [revue du développement par agents](../conception/revues/developpement-agents.md) §4.2, §4.4, §6 ;
- les exigences SEC0-06 à SEC0-15 et SEC0-35 du carnet du responsable architecture et sécurité, dont son §9.3.

`TS_CACHE` vaut `C:\Dev\ts-cache` par défaut ; la variable d'environnement `TS_CACHE` le change.

## 1. Emplacements de compilation

`tools/dev/session-start.ps1` ne bloque jamais et peut tourner plusieurs fois sans effet de bord.

- **Au démarrage de session** (crochet `SessionStart`, sans option) :
  - **copie principale** : `target` devient une jonction vers `TS_CACHE\target\main` ;
  - **worktree** : **rien n'est pris**. Le script affiche l'état : emplacement déjà pris, ou nombre d'emplacements libres. Il rappelle aussi la commande à lancer (décidé : les sessions de documentation et les sous-agents en worktree ne doivent pas occuper les 3 emplacements) ;
  - sous 40 Go libres, un avertissement s'affiche (A34).
- **Avant le premier `cargo`**, une tâche qui compile lance :
  ```
  pwsh -NoProfile -File tools/dev/session-start.ps1 -Acquire
  ```
  Le script prend alors un **bail** sur `w1`, `w2` ou `w3`, et crée la jonction `target` :
  - le bail est un fichier `wN.lease`, créé de façon exclusive, qui contient le chemin du worktree ;
  - un bail est **tenu** s'il a moins de 10 minutes, ou **tant que son worktree existe** (fichier `.git`), même si la jonction a disparu après un `git clean -xdf` ;
  - un worktree qui a déjà un bail à son nom le reprend ;
  - un bail **orphelin** (worktree retiré, donc sans `.git`) est libéré par renommage atomique, puis repris ;
  - si aucun emplacement n'est libre, l'avertissement est « les 3 emplacements sont pris » : la session ne compile pas, elle attend, ou la tâche passe au cloud.
- **En fin de tâche** :
  ```
  pwsh -NoProfile -File tools/dev/session-start.ps1 -Release
  ```
  Le script libère le bail et retire la **jonction seule**, sans toucher au contenu de l'emplacement.

**Repli `CARGO_TARGET_DIR`** (§6.3 point 4). Il s'applique quand la racine du worktree dépasse 100 caractères, parce que `link.exe` (MSVC 14.28) échoue au-delà de 260 caractères (voir §5). Le script donne alors la commande à lancer en Bash et en PowerShell. Au démarrage de session, il écrit aussi `export CARGO_TARGET_DIR='…'` dans `CLAUDE_ENV_FILE`, avec les guillemets simples échappés.

## 2. Crochets Git

Les crochets vivent dans `tools/dev/hooks/`. Ils s'exécutent depuis une **copie épinglée hors du dépôt**, `TS_CACHE\hooks\<sha256>\`, désignée par `core.hooksPath` (SEC0-12).

| Crochet | Contrôles |
|---|---|
| `pre-commit` | Refus de **tout commit sur `main`**, sans variable d'exception (SEC0-13).<br>Refus d'un fichier de plus de 5 Mo (SEC0-11).<br>Refus, hors LFS, des extensions `.glb .gltf .png .jpg .ogg .wav .mp3 .otf .ttf .onnx .gguf .pt .safetensors .parquet .sqlite .db .blend .psd .fbx .zip .7z`. Exception : `**/tests/fixtures/**`, jusqu'à 256 Ko.<br>`gitleaks` sur l'index, sur toutes les branches, `pilotage` comprise (SEC0-08). |
| `pre-merge-commit` | Mêmes contrôles, pour un `git merge` qui crée un commit de fusion. |
| `reference-transaction` | `refs/heads/main` ne prend qu'une valeur : celle de `refs/remotes/origin/main`. C'est le cas de l'avance rapide de `git pull --ff-only`, ou d'une création à cette valeur. Sont refusés :<br>- la **suppression** de `main` ;<br>- toute mise à jour quand `origin/main` est absente ;<br>- tout autre déplacement : `revert`, `cherry-pick`, `rebase`, `am`, `reset`, `update-ref`, `branch -f`, `merge`.<br>Ce crochet couvre ce que `pre-commit` ne voit pas (I2). Attention : un `reset --hard` refusé a déjà modifié l'index et les fichiers. On les rétablit par `git reset --hard`, sans argument. |
| `pre-push` | Refus de pousser vers `main` ; refus d'une poussée forcée (historique distant non ancêtre) ; refus de supprimer `pilotage`.<br>`gitleaks` sur la plage poussée. |
| `commit-msg` | Format `type(portée): description`, avec un type parmi `feat fix refactor perf test docs build ci chore` (A31). Restent permis : `Merge …`, `Revert "…"`, `fixup!`, `squash!` et `amend!`. |

**`gitleaks`** :
- il est appelé par son chemin épinglé dans `tools/dev/tools.lock`, et son SHA-256 est **vérifié avant chaque usage** : s'il manque ou si l'empreinte diffère, le crochet refuse (SEC0-06) ;
- il n'utilise que la **configuration épinglée** du jeu de crochets, `tools/dev/hooks/gitleaks.toml`, qui étend les règles par défaut (I3). Les options sont `--config`, `--gitleaks-ignore-path` vers le jeu de crochets, et `--ignore-gitleaks-allow` ;
- `GITLEAKS_CONFIG` et `GITLEAKS_CONFIG_TOML` sont vidées avant l'appel ;
- un `.gitleaksignore` à la racine de la copie de travail est **refusé**, car gitleaks 8.30.1 le lit malgré `--gitleaks-ignore-path` (constaté) ;
- une exception passe par une PR qui modifie `gitleaks.toml`. Le `.gitleaks.toml` du dépôt et la CI viendront avec T13.

**Installation (provisoire, remplacée en T3 par `cargo xtask hooks install`)**, depuis n'importe quelle copie de travail :

```powershell
pwsh -NoProfile -File tools/dev/install-hooks.ps1           # installe et règle core.hooksPath (configuration commune)
pwsh -NoProfile -File tools/dev/install-hooks.ps1 -Verify   # contrôle seulement
```

L'installation fait les opérations suivantes :
- elle vérifie gitleaks ;
- elle copie les crochets, `gitleaks.toml` et `tools.lock` dans `TS_CACHE\hooks\<empreinte du jeu>\`, en lecture seule ;
- elle règle `core.hooksPath` dans `.git/config`, commun à toutes les copies de travail ;
- elle écrit l'empreinte attendue dans `TS_CACHE\hooks\current.sha256`.

`-Verify` contrôle l'empreinte du jeu installé. Il lit aussi la valeur **effective** de `core.hooksPath` dans **chaque** copie de travail, avec `git config --show-origin --get-all`, et refuse toute surcharge hors de la configuration commune : `config.worktree`, `includeIf`, configuration globale (m5).

L'empreinte du jeu est le SHA-256 de la liste `nom NUL sha256 LF` des fichiers, triée par nom en ordre ordinal. Toute modification de `tools/dev/hooks/` ou de `tools.lock` demande de réinstaller.

## 3. Garde-fous de Claude Code

Les crochets sont inscrits dans `.claude/settings.json` du projet (AS-15). La commande est lancée par Git Bash, l'interpréteur par défaut des crochets sous Windows :

```
pwsh -NoProfile -NonInteractive -File "${CLAUDE_PROJECT_DIR}/tools/dev/<script>.ps1"
```

Ce format a été vérifié le 30/10 par `claude -p` (Claude Code 2.1.302).

### 3.1 `guard-main.ps1` (`Edit|Write|NotebookEdit`)

Le chemin est d'abord **rendu canonique** (m2) :
- les variables du dossier personnel sont développées ;
- les préfixes `\\?\` et `\\.\` sont retirés ;
- les flux NTFS sont retirés (`::$DATA`, et tout `:` après la lettre de lecteur) ;
- les points et les espaces finaux de chaque composant sont retirés ;
- les noms courts 8.3 sont résolus par `GetLongPathName`.

Un partage UNC vers l'hôte local (`\\localhost\`, `\\127.0.0.1\`, nom de la machine) ou un partage administratif (`C$`) est refusé, tout comme un nom 8.3 irrésoluble.

Il refuse ensuite :
- **SEC0-35** : `.claude/settings.json` et `.claude/settings.local.json` de toute copie de travail, sauf sur une branche `claude/outillage-*`. Les **réglages utilisateur** (`~/.claude/settings*.json`) sont refusés **toujours**, même sur une telle branche (m4). En cas d'erreur, cette partie **refuse** ;
- **A19** : l'écriture dans la copie principale, hors de `.claude/worktrees/`. C'est un garde-fou de commodité : en cas d'erreur, cette partie laisse passer, avec un avertissement.

### 3.2 `guard-git.ps1`

Il couvre tous les outils qui lancent une commande (I1) : `Bash`, `PowerShell`, `Monitor` et `mcp__terminal__run_in_terminal`. Il lit le champ `tool_input.command` ; pour `run_in_terminal`, il lit aussi le `cwd` de l'appel.
- Un `Monitor` sur WebSocket (`ws`, sans commande) est permis.
- Pour tout autre appel de ces outils, un champ `command` absent ou illisible entraîne un refus.

Il refuse :
- **Commits et poussées** :
  - `git commit` sur `main`, y compris par `-C`, `cd` ou le `cwd` de l'appel ;
  - `git push` vers `main` : `main`, `HEAD:main`, `heads/main`, `refs/heads/main`, `refs/heads//main`, `:main`, ou `HEAD` et poussée sans destination depuis `main` ;
  - une destination préfixée par `+`, `--force`, `-f` (y compris groupé, comme `-uf`), `--force-with-lease`, `--force-if-includes` et leurs abréviations ;
  - `--all`, `--mirror`, et la suppression de `pilotage` ;
  - `send-pack`, `http-push` et `remote-*`, qui poussent sans `pre-push`.
- **Configuration injectée** (B1) :
  - `--no-verify` et ses abréviations, **n'importe où** dans le segment git, ainsi que `-n` groupé dans `commit` ;
  - `-c` et `--config-env` sur `alias.*`, `include.*`, `includeIf.*`, `core.hooksPath`, `remote.*.push` et `remote.*.mirror` ;
  - `git config` en écriture sur ces clés ;
  - toute mention de `GIT_CONFIG*` dans une commande qui contient `git` ;
  - `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR` ou `GIT_INDEX_FILE` avec une écriture Git ;
  - toute écriture, hors de `git`, d'un mot qui désigne `hooksPath`, `.git/config`, `config.worktree`, `.git/hooks` ou `.gitconfig`.
- **`main` sans commit** (I2) : sur `main`, `revert`, `cherry-pick`, `rebase`, `am`, un `reset` vers un commit, ainsi que `merge` et `pull` sans `--ff-only`. Restent permis `--abort` et `--quit`, `git reset` sans argument, et `git pull --ff-only`.
- **Autres écritures sur `main`** : `git update-ref` sur `main` ou `--stdin`, `git branch -f/-d/-D/-m/-c main`, `git checkout -B main` et `git switch -C main`.
- **GitHub** (I4) :
  - `gh pr merge` sous toutes ses formes ;
  - `gh api` en écriture : `-X` ou `--method` autre que `GET`, `-f`, `-F`, `--field`, `--raw-field`, `--input` ;
  - `gh api` sur `contents/`, `merges`, `pulls/*/merge`, `git/` et `graphql`.
- **Git caché** (m1). Si la commande contient une substitution ou une évaluation, **et** l'un des mots `push`, `commit`, `update-ref`, `merge`, `reset`, `rebase`, `config` ou `send-pack`, elle est refusée par prudence. Formes de substitution : `$(`, accent grave en Bash, `$'`, `${`, `iex`, `Invoke-Expression`, `cmd /c`, `^` avec `cmd`, `& (…)`, `& $x`, `$x` en tête de commande.
  - **Conséquence** : un message de commit se passe par fichier (`git commit -F <fichier>`), jamais par `-m "$(cat …)"`.
- **Réglages de Claude Code** (SEC0-35, m3) : toute commande qui désigne un fichier de réglages est refusée. Sont visés `settings.json`, `settings.local.json`, un motif comme `setting[s].json` ou `sett*`, un nom 8.3 `SETTIN~1`, ou le dossier `.claude` lui-même.
  - Seule exception : une **pure lecture**, c'est-à-dire un segment sans redirection ni `--output`, dont la commande de tête est l'une des suivantes : `cat`, `type`, `Get-Content`, `gc`, `less`, `more`, `head`, `tail`, `grep`, `rg`, `Select-String`, `ls`, `dir`, `Get-ChildItem`, `Test-Path`, `Get-FileHash`, `sha256sum`, `wc`, `cd`, ou `git diff`, `git show`, `git log`, `git status`.
  - Le rattachement d'une commande à un fichier se fait **par segment**.
  - `$HOME`, `${HOME}`, `~`, `$env:USERPROFILE` et `%USERPROFILE%` sont développés. Une autre variable dans le chemin vaut un refus.
  - `disableAllHooks` est cherché aussi dans le texte débarrassé des guillemets, des `+` et des barres obliques inverses.
  - Seule exception : une copie de travail sur une branche `claude/outillage-*`, jamais pour les réglages utilisateur.

Chaque commande est lue deux fois :
- une lecture stricte, qui respecte guillemets et échappements de Bash ou de PowerShell, et découpe sur `&&`, `||`, `;`, `|`, `&`, les parenthèses et les fins de ligne ;
- une lecture naïve, qui ignore les guillemets, pour `bash -c "…"` et `pwsh -Command "…"`.

Un refus de l'une suffit, et une erreur interne vaut refus.

### 3.3 Fusion : `tools/dev/merge-pr.ps1 <PR>` (provisoire)

**Provisoire, à supprimer par T3**, qui le remplace par `cargo xtask merge` : il ne doit rester qu'un seul chemin de fusion. C'est un chemin sensible. C'est la seule fusion permise par le garde, qui refuse `gh pr merge`. Dans l'ordre :
1. Il est lancé depuis la **copie principale**, c'est-à-dire le script de la copie principale, jamais une copie modifiée dans un worktree. La copie principale doit être propre et sur `main` ; elle est mise à jour par `git pull --ff-only`.
2. `gh pr view` : PR ouverte, base `main`, hors brouillon, `MERGEABLE`, et en retard de 0 commit sur `main`.
3. **Chemin sensible** : la PR doit porter les commentaires `review-ok: <sha de tête>` **et** `archsec-ok: <sha de tête>`. Sont sensibles :
   - `.claude/`, `.github/`, `.cargo/`, `tools/`, `fuzz/`, `deny.toml`, `.gitleaks.toml`, `rust-toolchain.toml` ;
   - les crates `ts_persist`, `ts_net`, `ts_pilot`, `ts_client_native`, `ts_accounts`, `ts_admin`, `ts_protocol` et `ts_view` ;
   - un `Cargo.toml` dont le diff mentionne `[features]` ou `[profile…]`.
4. **`ci-ok`** vert sur le SHA de tête, **exigé dès que `.github/workflows/ci.yml` existe sur `origin/main`**. Avant cela, le script avertit et rappelle de noter la sortie des vérifications dans le carnet du développement. Un `ci-ok` présent et rouge refuse toujours.
5. `gh pr merge <PR> --squash --delete-branch --match-head-commit <sha>`, puis `git pull --ff-only`.

`-DryRun` fait tous les contrôles en lecture, sans fusion ; il est permis depuis un worktree.

### 3.4 Coût, limites et risque résiduel

**Coût** : chaque appel d'un outil de commande lance `pwsh`, soit environ 1,2 à 1,8 s, dont 0,8 s pour le seul démarrage de `pwsh`. **Reporté à T3 (décidé)** : un garde compilé dans `xtask` le ramènera à quelques dizaines de millisecondes.

**Ce que les garde-fous n'attrapent pas** : scripts écrits puis exécutés, interpréteurs qui construisent un nom de fichier par morceaux, alias déjà présents dans une configuration globale. Ils protègent de l'erreur et de l'injection simple, pas d'une session décidée à contourner.

**Risque résiduel (R-ARCH-04)** : une session peut modifier le `.claude/settings.json` de son propre worktree, par un moyen que le crochet ne voit pas, et retirer ainsi ses garde-fous. Restent alors :
- les crochets Git épinglés hors du dépôt (`pre-commit`, `reference-transaction`, `pre-push`) ;
- la CI (`ci-ok`) ;
- la fusion par `merge-pr.ps1`, puis `xtask merge`, avec la vérification d'intégrité de SEC0-35 (T3).

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
- l'emplacement réel `w1`, pris par `-Acquire`.

| Point | Résultat |
|---|---|
| Attribution d'un emplacement | **Conforme** : `w1` attribué, jonction créée, bail au nom du worktree. |
| Crochet `SessionStart` lancé par Claude Code (point 2) | **Conforme** : vérifié par `claude -p` (§3) ; `pwsh` 7.6 disponible. |
| Compilation à travers la jonction (point 3) | **Conforme** : `cargo test` réussit, avec un chemin le plus long de 254 caractères.<br>**Limite constatée** : à 300 caractères et plus, `link.exe` (MSVC 14.28) échoue avec `LNK1104`. Le même projet compile avec `CARGO_TARGET_DIR` vers l'emplacement. D'où le repli au-delà de 100 caractères de racine (§1). |
| `git clean -xdf` | **Conforme** : la jonction est retirée **sans suivre le lien**, et l'emplacement reste intact (140/140 fichiers). Le bail reste tenu, et `-Acquire` reprend le même emplacement. |
| `git worktree remove` | **Conforme** : réussit sans `--force`, l'emplacement reste intact (140/140 fichiers).<br>**Particularité** : Git retire le worktree et son `.git`, mais **laisse le dossier et la jonction**. Le bail devient orphelin et il est libéré au prochain `-Acquire`. Le dossier laissé se supprime sans risque après avoir retiré la jonction seule, par `[IO.Directory]::Delete("<worktree>\target")`, jamais par un `Remove-Item -Recurse` sur la jonction. Ce nettoyage revient à `cargo xtask disk --clean` (T3). |
| Archivage par l'application | Non essayé ici. Il passe par `git worktree remove` : même comportement attendu. Le premier archivage réel est à observer. |

Le mécanisme de jonction est **adopté** (A9) ; le repli `CARGO_TARGET_DIR` sert seulement aux racines longues. À la fin de l'essai, l'emplacement a été débarrassé des fichiers de l'essai, **puis** le bail a été libéré.
