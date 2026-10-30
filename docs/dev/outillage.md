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
| `pre-commit` | Refus de **tout commit sur `main`**, sans variable d'exception (SEC0-13).<br>Refus d'un fichier de plus de 5 Mo (SEC0-11).<br>Refus, hors LFS, des extensions `.glb .gltf .png .jpg .ogg .wav .mp3 .otf .ttf .onnx .gguf .pt .safetensors .parquet .sqlite .db .blend .psd .fbx .zip .7z`. Exception : `**/tests/fixtures/**`, jusqu'à 256 Ko.<br>`gitleaks dir` sur une **extraction des fichiers indexés** (ajoutés ou modifiés). Elle est faite par `git checkout-index` dans `TS_CACHE\tmp`, puis supprimée. Un attribut `-diff`, `binary` ou `textconv` ne masque donc rien (I3 bis). Toutes les branches sont concernées, `pilotage` comprise (SEC0-08). |
| `pre-merge-commit` | Mêmes contrôles, pour un `git merge` qui crée un commit de fusion. |
| `reference-transaction` | `refs/heads/main` ne prend qu'une valeur : celle de `main` **sur le distant**, lue par `git ls-remote origin refs/heads/main`. C'est le cas de l'avance rapide de `git pull --ff-only`. La référence locale `refs/remotes/origin/main` ne fait pas foi, puisqu'elle peut être déplacée à la main (I2 bis). Sont refusés :<br>- la **suppression** de `main` ;<br>- toute mise à jour quand `origin/main` est absente, ou quand **le distant ne répond pas** ;<br>- tout autre déplacement : `revert`, `cherry-pick`, `rebase`, `am`, `reset`, `update-ref`, `branch -f`, `merge`.<br>Sont acceptés : une valeur inchangée, et l'élagage de `git pack-refs` ou `git gc`. Dans ce cas, la référence libre disparaît alors que `packed-refs` contient déjà la même valeur, sans `packed-refs.lock` ; une vraie suppression tient ce verrou (constaté en 2.56).<br>L'appel réseau n'a lieu que lorsque `main` bouge. Attention : un `reset --hard` refusé a déjà modifié l'index et les fichiers. On les rétablit par `git reset --hard`, sans argument. |
| `pre-push` | Refus de pousser vers `main` ; refus d'une poussée forcée (historique distant non ancêtre) ; refus de supprimer `pilotage`.<br>`gitleaks git` sur la plage poussée, avec `--text --no-ext-diff --no-textconv` (I3 bis). |
| `commit-msg` | Format `type(portée): description`, avec un type parmi `feat fix refactor perf test docs build ci chore` (A31). Restent permis : `Merge …`, `Revert "…"`, `fixup!`, `squash!` et `amend!`. |

Les crochets vident, avant tout contrôle, `GIT_CONFIG_PARAMETERS`, `GIT_CONFIG_COUNT`, `GIT_EXTERNAL_DIFF`, `GITLEAKS_CONFIG` et `GITLEAKS_CONFIG_TOML`. Une option `git -c` ou une variable d'environnement ne change donc pas leurs contrôles.

**`gitleaks`** :
- il est appelé par son chemin épinglé dans `tools/dev/tools.lock`, et son SHA-256 est **vérifié avant chaque usage** : s'il manque ou si l'empreinte diffère, le crochet refuse (SEC0-06) ;
- il n'utilise que la **configuration épinglée** du jeu de crochets, `tools/dev/hooks/gitleaks.toml`, qui étend les règles par défaut. Les options sont `--config`, `--gitleaks-ignore-path` vers le jeu de crochets, et `--ignore-gitleaks-allow` ;
- un `.gitleaksignore` à la racine de la copie de travail est **refusé**, car gitleaks 8.30.1 le lit malgré `--gitleaks-ignore-path` (constaté) ;
- une exception passe par une PR qui modifie `gitleaks.toml` ;
- **consigne pour T13** : la CI doit lancer gitleaks de la même façon, c'est-à-dire avec `--text` ou sur des fichiers extraits, et avec la configuration relue du dépôt seulement.

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

`-Verify` contrôle l'empreinte du jeu installé. Il lit aussi la valeur **effective** de `core.hooksPath` dans **chaque** copie de travail, avec `git config --show-origin --get-all`, et refuse toute surcharge hors de la configuration commune : `config.worktree`, `includeIf`, configuration globale.

L'empreinte du jeu est le SHA-256 de la liste `nom NUL sha256 LF` des fichiers, triée par nom en ordre ordinal. Toute modification de `tools/dev/hooks/` ou de `tools.lock` demande de réinstaller.

## 3. Garde-fous de Claude Code

Les crochets sont inscrits dans `.claude/settings.json` du projet (AS-15). La commande est lancée par Git Bash, l'interpréteur par défaut des crochets sous Windows :

```
pwsh -NoProfile -NonInteractive -File "${CLAUDE_PROJECT_DIR}/tools/dev/<script>.ps1"
```

Ce format a été vérifié le 30/10 par `claude -p` (Claude Code 2.1.302).

### 3.1 `guard-main.ps1` (`Edit|Write|NotebookEdit`)

Le chemin est d'abord **rendu canonique** :
- les variables du dossier personnel sont développées ;
- les préfixes `\\?\` et `\\.\` sont retirés ;
- les flux NTFS sont retirés (`::$DATA`, et tout `:` après la lettre de lecteur) ;
- les points et les espaces finaux de chaque composant sont retirés ;
- les noms courts 8.3 sont résolus par `GetLongPathName`.

Un partage UNC vers l'hôte local (`\\localhost\`, `\\127.0.0.1\`, nom de la machine) ou un partage administratif (`C$`) est refusé, tout comme un nom 8.3 irrésoluble.

Il refuse ensuite :
- **SEC0-35** : `.claude/settings.json` et `.claude/settings.local.json` de toute copie de travail, sauf sur une branche `claude/outillage-*`. Les **réglages utilisateur** (`~/.claude/settings*.json`) sont refusés **toujours**, même sur une telle branche. En cas d'erreur, cette partie **refuse** ;
- **A19** : l'écriture dans la copie principale, hors de `.claude/worktrees/`. C'est un garde-fou de commodité : en cas d'erreur, cette partie laisse passer, avec un avertissement.

### 3.2 `guard-git.ps1` : liste blanche

Il couvre tous les outils qui lancent une commande : `Bash`, `PowerShell`, `Monitor` et `mcp__terminal__run_in_terminal`. Il lit le champ `tool_input.command` ; pour `run_in_terminal`, il lit aussi le `cwd` de l'appel.
- Un `Monitor` sur WebSocket (`ws`, sans commande) est permis.
- Pour tout autre appel de ces outils, un champ `command` absent ou illisible entraîne un refus.

**`git` : liste blanche** (décidé à la deuxième relecture : une liste noire ne se ferme jamais).
- **Options globales** : seules `-C <dossier>` et `--no-pager` sont permises (ainsi que `--version` et `--help`). Tout le reste est refusé : `-c`, `--config-env`, `--git-dir`, `--work-tree`, `--attr-source`, `--exec-path`, ou une option inconnue.
- **Préfixe d'environnement** sur un appel `git` (`HOME=`, `XDG_CONFIG_HOME=`, `GIT_*=`, `env …`) : refus.
- **Variable d'environnement modifiée** dans une commande qui contient `git` : refus. Sont visés `export X=`, `Set-Item Env:`, `$env:X =`, `[Environment]::SetEnvironmentVariable`, `GIT_CONFIG*`, et aussi un alias ou une fonction définis dans la même commande.
- **Sous-commandes permises**, avec leurs règles :

| Sous-commande | Règle |
|---|---|
| `status`, `log`, `show`, `diff`, `blame`, `grep`, `describe`, `rev-parse`, `ls-files`, `ls-remote`, `cat-file`, `merge-base`, `shortlog`, `rev-list`, `show-ref`, `for-each-ref`, `name-rev`, `check-ignore`, `check-attr`, `count-objects`, `version`, `help` | Lecture ; `--output` refusé partout |
| `add`, `rm`, `mv`, `restore`, `clean` | Permis ; `.claude` reste soumis à SEC0-35 |
| `commit` | Jamais `-n` ni `--no-verify` ; jamais sur `main`. Un message par `-F <fichier>`, `-F -` avec heredoc cité, ou `-m` sans substitution |
| `switch`, `checkout` | Sans `-b`, `-B`, `-c`, `-C` ni `--force-create` vers `main` |
| `branch` | Aucune création, aucun `-f`, `-d`, `-m` ni `-c` sur `main` ; les listes restent permises |
| `fetch` | Un distant au plus, sans refspec explicite : `refs/remotes/` ne bouge que par `git fetch <distant>` |
| `pull` | Sur `main`, seulement avec `--ff-only` ; ailleurs, `--rebase` est permis. Jamais de refspec avec `:` ni `refs/` |
| `push` | Règles de SEC0-14 (ci-dessous). Refus aussi vers `refs/remotes/` et vers le dépôt local `.` |
| `merge` | Sur `main`, seulement `--ff-only origin/main`, `--abort` et `--quit` |
| `rebase`, `revert`, `cherry-pick` | Hors `main`, ou avec `--abort`/`--quit` ; `rebase` qui désigne `main` refusé |
| `reset` | Sans cible, ou vers `HEAD` ; sur `main`, aussi `--hard` ou `--keep` vers `origin/main`, valeur contrôlée par `reference-transaction` |
| `stash` | `push`, `list`, `apply`, `drop` et `show` (pas `pop`, puisque la pile est partagée entre sessions) |
| `worktree` | `list`, `add`, `remove` et `prune` ; jamais `-B`, ni `-b main` |
| `remote` | Seulement `git remote` et `git remote -v` |
| `config` | Lecture seule : `--get*`, `--list` |
| `tag` | `git tag` et `git tag -l` |
| `lfs` | Tout, sauf `install`, `uninstall` et `update`, qui touchent aux crochets |

Toute **autre sous-commande, et tout alias**, est refusée. Sont ainsi fermés : `update-ref`, `send-pack`, `http-push`, `remote-*`, `config` en écriture, `stash pop`, `gc`, `reflog expire`, `notes`, `apply`, `am`.

**Règles de `push`** :
- refus vers `main` : `main`, `HEAD:main`, `heads/main`, `refs/heads/main`, `refs/heads//main`, `:main`, ou `HEAD` et poussée sans destination depuis `main` ;
- refus d'une destination préfixée par `+`, de `--force`, `-f` (y compris groupé), `--force-with-lease`, `--force-if-includes` et leurs abréviations ;
- refus de `--all`, de `--mirror` et de la suppression de `pilotage`.

**`gh` : liste blanche.** Les options `-R`/`--repo` sont sautées avec leur valeur, où qu'elles soient. Sont permis :
- `pr create|view|list|diff|checks|comment|status|close|edit|ready`, avec `pr edit --base` refusé ;
- `issue create|view|list|comment|edit|close|reopen` ;
- `label list|create|edit` ;
- `run list|view|watch|rerun`, `workflow list|view` ;
- `repo view` ;
- `auth status` ;
- `version` et `help` ;
- `api` en lecture seule : ni `-X`/`--method` autre que `GET`, ni `-f`, `-F`, `--field`, `--raw-field`, `--input`, ni `graphql`, `contents/`, `merges`, `pulls/*/merge` ou `git/`.

Tout le reste est refusé, dont tout alias de `gh` : `pr merge`, `pr review`, `alias set|import`, `extension`, `secret`, `repo edit|delete`, `workflow run`.

`gh api` est relu **sur tout le reste de la ligne**, jusqu'à un `|`, `;` ou `&` hors guillemets, et non sur le seul segment. Les marques `{owner}/{repo}`, les parenthèses (`gh api ("…") -X PUT`) ou les guillemets ne cachent donc ni la méthode, ni les champs, ni le chemin.

Dans le découpage, `{` ne coupe un mot qu'en début de mot, et la `}` qui la ferme reste littérale : `repos/{owner}/{repo}/…` et `HEAD@{0}` sont des mots entiers.

**`gh pr merge` est refusé : la fusion passe par `pwsh tools/dev/merge-pr.ps1 <PR>`**, depuis la copie principale (§3.3). Le message de refus le dit.

**Git caché.** Si la commande contient une substitution ou une évaluation, **et** l'un des mots `push`, `commit`, `update-ref`, `merge`, `reset`, `rebase`, `config` ou `send-pack`, elle est refusée par prudence. Le mot doit être placé :
- après `git` ou `gh` ;
- ou après une commande fabriquée en position de commande (`g${x}it`, `$'\x67it'`, `g^it`, `('gi'+'t')`) ;
- ou n'importe où avec `iex`, `Invoke-Expression`, `eval`, `cmd /c`, ou un accent grave en Bash.

Une affectation en tête (`$x = …`, `$env:X = …`) n'est pas une commande tirée d'une variable.
- Formes de substitution : `$(`, accent grave en Bash, `$'`, `${`, `iex`, `Invoke-Expression`, `cmd /c`, `^` avec `cmd`, `& (…)`, `& $x`, `$x` en tête de commande.
- Seules `$(git rev-parse …)` et `$(pwd)` sont permises, par exemple `cd "$(git rev-parse --show-toplevel)"`.
- `pwsh -EncodedCommand` est refusé.
- Le message de refus propose la forme permise : `git commit -F <fichier>` et `--body-file`.

**Données, pas commandes.**
- Le corps d'un heredoc à délimiteur cité (`<<'EOF'`, `<<"EOF"`, `<<\EOF`) et celui d'une here-string littérale PowerShell (`@'…'@`) ne sont pas analysés.
- La valeur de `--body`, `--title`, `-m` ou `--message` d'un appel `git` ou `gh` ne déclenche pas la règle des réglages.
- Un mot `git` dans les arguments d'une commande de données, comme `echo`, `printf` ou `grep`, n'est pas un appel.
- `bash -c`, `sh -c`, `pwsh -Command`, `cmd /c` et `eval` sont **relus récursivement**, jusqu'à trois niveaux.

**Réglages de Claude Code et configuration de Git** (SEC0-35) :
- toute commande qui désigne un fichier de réglages est refusée. Sont visés `settings.json`, `settings.local.json`, un motif comme `setting[s].json` ou `sett*`, `SETTIN~1`, ou le dossier `.claude` ;
- de même pour la configuration de Git : `.git/config` (y compris `.git/./config`), `config.worktree`, `.git/info/attributes`, `.git/hooks`, `~/.gitconfig`, `~/.config/git/`, `hooksPath` ;
- seule exception, une **pure lecture** : un segment sans redirection ni `--output`, dont la commande de tête est l'une des suivantes : `cat`, `type`, `Get-Content`, `gc`, `less`, `more`, `head`, `tail`, `grep`, `rg`, `Select-String`, `ls`, `dir`, `Get-ChildItem`, `Test-Path`, `Get-FileHash`, `sha256sum`, `wc`, `cd`, ou `git diff|show|log|status|blame|ls-files|grep|cat-file` ;
- `$HOME`, `${HOME}`, `~`, `$env:USERPROFILE` et `%USERPROFILE%` sont développés. Une autre variable dans le chemin vaut un refus ;
- `disableAllHooks` est cherché aussi dans le texte débarrassé des guillemets, des `+` et des barres obliques inverses ;
- pour les réglages de Claude Code, une copie de travail sur une branche `claude/outillage-*` est exceptée, jamais pour les réglages utilisateur.

En cas d'erreur interne, la commande est refusée.

### 3.3 Fusion : `tools/dev/merge-pr.ps1 <PR>` (provisoire)

**Provisoire, à supprimer par T3**, qui le remplace par `cargo xtask merge` : il ne doit rester qu'un seul chemin de fusion. C'est un chemin sensible, validé par archi-sécu, et la seule fusion permise par le garde. Dans l'ordre :
1. Il est lancé depuis la **copie principale**, c'est-à-dire le script de la copie principale, jamais une copie modifiée dans un worktree. La copie principale doit être propre et sur `main` ; elle est mise à jour par `git pull --ff-only`.
2. **Crochets** : `install-hooks.ps1 -Verify` doit réussir. Il vérifie l'empreinte du jeu et la valeur effective dans chaque copie de travail (SEC0-12, I6). Le contrôle de l'origine (`origin-check`, SEC0-17) reste à T3.
3. `gh pr view` : PR ouverte, base `main`, hors brouillon, `MERGEABLE`, et en retard de 0 commit sur `main`.
4. **Liste complète des fichiers**, par l'API `pulls/<n>/files` paginée. Un renommage compte ses deux chemins, `filename` et `previous_filename` (I10). Le nombre de fichiers doit égaler `changedFiles`, sinon la fusion est refusée. *Pourquoi : `gh pr view --json files` s'arrête à 100 fichiers (I5), et `gh pr diff --name-only` ne donne pas l'ancien chemin d'un renommage.*
5. **Chemin sensible** : la PR doit porter les commentaires `review-ok: <sha de tête>` **et** `archsec-ok: <sha de tête>`. Sont sensibles :
   - `.claude/`, `.github/`, `.cargo/`, `tools/`, `fuzz/`, `deny.toml`, `.gitleaks.toml`, `rust-toolchain.toml` ;
   - les crates `ts_persist`, `ts_net`, `ts_pilot`, `ts_client_native`, `ts_accounts`, `ts_admin`, `ts_protocol` et `ts_view` ;
   - un `Cargo.toml` dont le diff mentionne `[features]` ou `[profile…]` ;
   - un `Cargo.lock` dont le diff ajoute une ligne `name = "…"`, c'est-à-dire une crate.
6. **`ci-ok`** vert sur le SHA de tête, **exigé dès que `.github/workflows/ci.yml` existe sur `origin/main`**. Avant cela, le script avertit et rappelle de noter la sortie des vérifications dans le carnet du développement. Un `ci-ok` présent et rouge refuse toujours.
7. `gh pr merge <PR> --squash --delete-branch --match-head-commit <sha>`, puis `git pull --ff-only`.

`-DryRun` fait tous les contrôles en lecture, sans fusion ; il est permis depuis un worktree.

### 3.4 Coût, limites et risques connus

**Coût** : chaque appel d'un outil de commande lance `pwsh`, soit environ 1,2 à 1,8 s, dont 0,8 s pour le seul démarrage de `pwsh`. **Reporté à T3 (décidé)** : un garde compilé dans `xtask` le ramènera à quelques dizaines de millisecondes.

**Ce que les garde-fous n'attrapent pas** :
- un script écrit puis exécuté (`bash x.sh`, `pwsh -File x.ps1`) ;
- un interpréteur qui construit un nom de fichier par morceaux.

Un alias de Git défini ailleurs, par exemple dans un fichier désigné par `XDG_CONFIG_HOME`, passe toujours **au niveau de Git** : `--no-verify` saute `pre-push` par construction. La liste blanche de `guard-git` le refuse pourtant, puisque tout nom hors liste est refusé.

**Risques connus**, relevés à la deuxième relecture et non corrigés (décidé) :
- **m4** : le jeu épinglé `TS_CACHE\hooks\` n'est protégé en écriture que par l'attribut lecture seule. Une modification est détectée par `-Verify`, que lance `merge-pr.ps1` avant chaque fusion.
- **m5** : une course entre trois sessions dans `Remove-OrphanLease` peut faire croire à deux sessions qu'elles tiennent le même bail. La fenêtre est de l'ordre de la milliseconde. Parade possible : un verrou `target\slots.lock` ouvert avec `FileShare.None` (T3).
- **m6** : `.gitignore` restreint `id_*` de SEC0-09 à `id_rsa*`, `id_dsa*`, `id_ecdsa*` et `id_ed25519*`, puisque `id_*` attraperait des sources légitimes. Cet écart est à noter au carnet d'archi-sécu.

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
