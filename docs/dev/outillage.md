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
| `pre-push` | Refus d'une poussée forcée (historique distant non ancêtre) vers toute branche **autre que `main`** ; refus de supprimer `main` ou `pilotage`.<br>La poussée directe et la poussée forcée vers le `main` du distant **`origin`** ne sont plus refusées ici depuis T3e-1 (AS-31) : le *ruleset* `main` de GitHub (26213563) les refuse pour tout le monde (§6.4). Vers le `main` de tout autre distant (autre nom, chemin ou URL), que le *ruleset* ne garde pas, la poussée reste refusée.<br>`gitleaks git` sur la plage poussée, **`main` comprise**, avec `--text --no-ext-diff --no-textconv` (I3 bis, AS-31 E2). Vers `main`, la plage est `<distant>..<local>`, ou, si la valeur distante est inconnue ici, tout ce qui manque aux branches suivies du distant. |
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
- **vers `main`, permis depuis T3e-1** (AS-31), **seulement vers le distant `origin` nommé** : poussée directe (`main`, `HEAD:main`, `heads/main`, `refs/heads/main`, `git push origin` ou `git push origin HEAD` depuis `main`) et poussée forcée (`+…:main`, `--force`, `-f`, `--force-with-lease` avec `main` pour seule destination nommée). Le *ruleset* `main` de GitHub les refuse (§6.4). Vers un autre distant, un chemin, une URL, avec `--repo`, ou sans distant nommé depuis `main` (`git push`), la poussée vers `main` reste refusée : le *ruleset* ne garde que `origin` ;
- refus de la **suppression** de `main` et de `pilotage` (`:main`, `--delete`, `-d`, y compris groupé ou abrégé) ;
- refus d'une poussée forcée vers toute **autre** destination : destination préfixée par `+`, ou `--force`, `-f` (y compris groupé), `--force-with-lease`, `--force-if-includes` et leurs abréviations, dès qu'une destination n'est pas `main`, **ou qu'aucune n'est nommée** (`git push -f`, `git push -f origin`, `HEAD`) : la destination implicite dépend de `push.default` et de la configuration ;
- refus de `--all`, de `--mirror`, de `refs/remotes/` et du dépôt local `.` ;
- `--no-verify` reste refusé partout, `main` comprise : `gitleaks` de `pre-push` ne se saute pas (AS-31 E2).

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

**`tools/dev/repo-settings.ps1`** (§6) ne se lance que sous l'une de ces formes exactes, **seule** dans la commande et **depuis la racine de la copie principale** (AS-30 R5, S4) :

```
pwsh tools/dev/repo-settings.ps1 -Check
pwsh tools/dev/repo-settings.ps1 -Apply
pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Check
pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Apply
```

Tout autre segment qui cite le script est refusé : autre argument, `;` ou `&&`, redirection, autre chemin (`./tools/…`, barres obliques inverses, chemin absolu), motif (`repo-set*`, ou un motif dans `tools/` avec `repo-s`, `sett` ou `~<chiffre>` comme `tools/d?v/repo-setting?.ps1`), nom court 8.3 (`REPO-S~1.PS1`), nom fabriqué par concaténation (`'repo-'+'settings.ps1'`, ou tout nom du script qui n'apparaît en clair qu'une fois les guillemets et les `+` retirés), `pwsh -Command`, `bash -c`, `&`, `Start-Process`, argument tiré d'une variable. Un segment de lecture ou de données peut le citer : `cat`, `grep`, `git diff`, `git add`, titre de PR. Le fichier `tools/ci/repo-settings.json` contient `settings.json` dans son nom : la règle SEC0-35 refuse donc de le nommer dans une commande d'écriture (`git add tools/ci/repo-settings.json`). On passe par son dossier : `git add tools/ci/`.

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
pwsh -NoProfile -File tools/dev/test-repo-settings.ps1 # invariants, comparaison ; arguments et refus S4 dans un dépôt jetable dont origin est local : jamais d'appel à GitHub
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

## 6. Réglages GitHub (T3d)

Sources : AS-30 (R1 à R6), AS-31 (E4, E5) et §9.12 (S1 à S6) du carnet d'archi-sécu. Depuis le passage à GitHub Pro (décidé par l'utilisateur le 30/10), les réglages du dépôt sont **décrits dans un fichier versionné**, relu comme du code, et appliqués par une seule commande.

### 6.1 Fichier de valeurs : `tools/ci/repo-settings.json`

C'est un chemin sensible. Il a quatre sections :

| Section | Contenu |
|---|---|
| `ruleset_main` (R1) | *Ruleset* actif sur `refs/heads/main`, **sans contournement** (`bypass_actors: []`, le jeton administrateur des sessions est lié lui aussi) :<br>- PR obligatoire, 0 approbation (un seul compte : les approbations restent `review-ok` et `archsec-ok`, vérifiés par `cargo xtask merge`), fusion par squash seulement ;<br>- `ci-ok` requis, **lié à l'application GitHub Actions** (`integration_id` 15368), branche à jour (`strict`) ;<br>- historique linéaire, poussée forcée et suppression interdites. |
| `ruleset_pilotage` (R3) | `refs/heads/pilotage` : poussée forcée et suppression interdites. La poussée directe reste permise. Aucun contournement. |
| `repo` (R2, S1) | Squash seulement (`allow_merge_commit` et `allow_rebase_merge` à `false`), titre du squash = titre de la PR, branche supprimée après fusion, **fusion automatique désactivée**. |
| `actions` (R4, S5) | `allowed_actions: selected` avec `sha_pinning_required` ; motifs = liste fermée de SEC0-30 (`actions/checkout@*`, `actions/cache@*`, `actions/cache/restore@*`, `actions/cache/save@*`, `Swatinem/rust-cache@*`, `actions/upload-artifact@*`), sans les actions de GitHub ni les vérifiées ; jeton en lecture seule, aucune approbation de PR par les Actions ; **aucun workflow pour les PR venues d'un *fork*** (réglage des dépôts privés). |

`integration_id` 15368 est l'`app.id` des contrôles `ci-ok` existants (relevé le 30/10 par `gh api`). Le motif `actions/cache@*` ne couvre pas `actions/cache/restore@…` : chaque sous-action est nommée.

### 6.2 `tools/dev/repo-settings.ps1 -Check | -Apply`

Deux modes exclusifs ; tout autre argument est refusé (code 2).

- **Source** : le fichier est lu par `git show origin/main:tools/ci/repo-settings.json`, **jamais sur le disque**. Une modification d'un worktree ne peut donc pas être appliquée avant d'avoir été fusionnée. Ordre des préalables, chacun avec son message (code 2) : arguments, préalables S4 ci-dessous, `git fetch origin` et égalité avec `origin/main`, présence du fichier sur `origin/main` (sinon il le dit et s'arrête), et enfin distant `origin` sur `github.com`.
- **Préalables** (S4) : lancé depuis la racine de la copie principale, script de cette copie (pas celui d'un worktree), copie propre, sur `main`, égale à `origin/main`. Sinon : code 2.
- **Invariants codés dans le script** (S2). Le fichier est refusé s'il manque à `ruleset_main` l'une des règles `pull_request`, `required_status_checks` (`ci-ok`, `strict`, `integration_id` 15368), `required_linear_history`, `non_fast_forward`, `deletion` ; si un `bypass_actors` n'est pas vide ; si un *ruleset* vise une autre branche ; si `repo` contient une autre clé que ses six clés (visibilité, branche par défaut… ne passent jamais par `-Apply`), permet autre chose que le squash ou la fusion automatique ; si `allowed_actions` n'est pas `selected`, si `sha_pinning_required` manque, si un motif sort de la liste fermée, si le jeton passe en écriture, si les Actions peuvent approuver, ou si les workflows des *forks* sont permis. Il est refusé aussi si une action utilisée par les workflows d'`origin/main` n'est couverte par aucun motif (`-Apply` casserait la CI). Affaiblir une règle demande donc de modifier aussi le script.
- **`-Check`** (lecture seule, `GET` seulement) compare GitHub au fichier et affiche chaque écart, avec la valeur attendue et la valeur lue ; code 1 s'il y en a un. Il signale :
  - un fichier refusé par les invariants ;
  - un *ruleset* attendu absent, en double, ou différent (règles indexées par type, listes comparées sans tenir compte de l'ordre) ;
  - tout *ruleset* inconnu qui vise `main` ou `pilotage` (S3 ; motifs `~ALL`, `~DEFAULT_BRANCH` et jokers compris, de façon prudente) ;
  - toute **protection de branche classique** encore présente, sur n'importe quelle branche (S3). `branches?protected=true` compte aussi les branches visées par un *ruleset* : chaque branche listée est relue par `branches/<nom>/protection`. Seul le 404 dont le message est « Branch not protected » vaut « aucune » ; tout autre 404 arrête `-Check` (code 2) ;
  - au-delà de 100 *rulesets* ou de 100 branches protégées, `-Check` s'arrête (code 2) : la pagination n'est pas gérée ;
  - toute différence des champs de `repo` et des quatre ressources d'Actions.

  Champs ignorés, liste explicite : `id`, `node_id`, `source`, `source_type`, `_links`, `created_at`, `updated_at`, `current_user_can_bypass` d'un *ruleset* ; les champs de l'objet du dépôt absents du fichier ; `selected_actions_url`. Un paramètre de règle présent sur GitHub et absent du fichier est un **écart** (par exemple `do_not_enforce_on_create: true`, qui affaiblirait la règle ; le fichier le fixe donc à `false`). Seule exception, une liste fermée et commentée dans le script, et seulement avec sa valeur neutre : `automatic_copilot_code_review_enabled: false` et `required_reviewers: []`, que GitHub peut renvoyer sans qu'on les envoie. Elle évite qu'un `-Apply` conforme laisse un écart qui suspendrait toutes les fusions.
- **`-Apply`** : refuse un fichier invalide sans rien écrire. Sinon, crée (`POST`) ou remplace (`PUT`, identifiant retrouvé par nom) les deux *rulesets*, règle le dépôt (`PATCH`) et les quatre ressources d'Actions (`PUT`), puis relance `-Check` et échoue si un écart subsiste. Il ne supprime ni *ruleset* inconnu ni protection classique : `-Check` les signale.

Codes de sortie : 0 conforme, 1 écart ou écriture refusée, 2 usage, préalable non rempli ou erreur.

### 6.3 Qui lance quoi

- **`-Apply`** : l'**orchestrateur**, après la fusion de la PR qui modifie `repo-settings.json` ou le script, depuis la copie principale à jour. Ordre de R6 : fusion, `-Apply`, `-Check` vert, puis essai de poussée directe vers `main`, qui doit être refusé par GitHub. Entre la fusion de T3d et le premier `-Apply`, `-Check` est en écart : les fusions sont suspendues jusqu'à `-Apply`.
- **`-Check`** : `cargo xtask verify-main`, donc **à chaque `cargo xtask merge`** (étape 2) et chaque nuit (AS-31 E4). Le contrôle de nuit doit tourner **en local**, dans la copie principale, puisqu'il est sauté en CI. Cette tâche planifiée n'existe pas encore : elle relève de T3e, suivie par l'issue que le responsable développement ouvrira. `verify-main` lance `pwsh -NoProfile -NonInteractive -File <copie principale>\tools\dev\repo-settings.ps1 -Check` :
  - code 1 (écart) : une *issue* `role:architecture` de clé `repo-settings-drift` est ouverte (ou réutilisée), et les fusions sont suspendues ;
  - autre code (préalable, réseau) : `verify-main` échoue ; en `--dry-run`, c'est un avertissement et le résultat reste un refus ;
  - **en CI** (`CI=true` et `GITHUB_ACTIONS=true`), le contrôle est **sauté** avec un avertissement dans le résumé du travail : `GITHUB_TOKEN` n'a pas les droits d'administration. Pour que ce saut ne serve pas à contourner le contrôle, `cargo xtask merge` **refuse** de tourner en mode CI ;
  - l'historique est vérifié avant les réglages : un `-Check` qui échoue ne cache pas une anomalie de l'historique.

### 6.4 Côté serveur, côté local

| Barrière | Serveur (GitHub) | Local |
|---|---|---|
| PR obligatoire, `ci-ok` de GitHub Actions, branche à jour, historique linéaire | *ruleset* `main` | `cargo xtask merge` (G3, toujours en place, E5) |
| Poussée forcée vers `main` | *ruleset* `main` | — (retiré par T3e-1) ; `reference-transaction` pour la copie locale de `main` |
| Poussée forcée vers `pilotage` et les autres branches | *ruleset* `pilotage` (aucun pour les branches de tâche) | `pre-push`, `guard-git` |
| Suppression de `main` et de `pilotage` | *rulesets* | `pre-push`, `guard-git` ; `reference-transaction` pour la copie locale de `main` |
| Poussée directe vers `main` | *ruleset* `main` | — (retiré par T3e-1) |
| Squash seulement, pas de fusion automatique | réglages du dépôt | `cargo xtask merge` (seul chemin de fusion) |
| Actions permises, épinglage par SHA | réglages d'Actions | étape `workflows` de `xtask check` |
| Avis `review-ok` et `archsec-ok` sur les chemins sensibles | — (un seul compte) | `cargo xtask merge`, `verify-main` (J4) |
| Secrets | — (pas de détection en Pro sur un dépôt privé) | `gitleaks` en `pre-commit`, `pre-push` et CI |
| Dérive des réglages eux-mêmes | — | `repo-settings.ps1 -Check` dans `verify-main` |

**Risque résiduel (R-ARCH-05)** : un script écrit puis exécuté par une session peut appeler `gh api` en écriture avec le jeton administrateur, ce que `guard-git` ne voit pas (§3.4). Les *rulesets* sans contournement le lient pour `main` et `pilotage`, mais il peut modifier les réglages eux-mêmes : `-Check` le détecte après coup, à la fusion suivante ou la nuit.

T3d ne retire **aucune** barrière locale (S6). Les retraits viennent ensuite, dans l'ordre d'AS-31 : T3e-1 (refus de poussée et de poussée forcée vers `main` dans `pre-push` et `guard-git`, **fait**), l'essai R6.4 par l'orchestrateur, puis T3e-2 (contrôle d'origine de `verify-main`). Restent sans changement : `gitleaks` en `pre-push` (E2), `pre-commit`, `commit-msg`, `reference-transaction`, `guard-main`, la liste blanche de `git` et de `gh` (dont `--no-verify`), `cargo xtask merge` et `verify-main`. Après la fusion de T3e-1, `cargo xtask hooks install` est relancé depuis la copie principale pour épingler le nouveau `pre-push`.
