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

**Retirés par AS-50** (décision de l'utilisateur du 01/11, #55 : « les outils de développement prennent trop de temps »). Les crochets `pre-commit`, `pre-merge-commit`, `reference-transaction`, `pre-push` et `commit-msg`, `test-hooks.ps1` et `cargo xtask hooks` (`install` et `verify`) n'existent plus. Les exigences SEC0-12, SEC0-13 et la part locale de SEC0-14 tombent avec eux.

Ce qui reste :
- **`main`** ne bouge que par le *ruleset* du serveur : PR obligatoire, `ci-ok`, historique linéaire, ni poussée forcée ni suppression (§6.4). Dans la copie principale, la règle « jamais éditée, jamais de commit sur `main` » est tenue par la discipline des sessions, et toute poussée vers `main` est refusée par GitHub.
- **Format des commits** (A31) : le titre de la PR est contrôlé en CI (travail `pr-title`) et par `cargo xtask merge`, et c'est lui qui devient le message du *squash*. Les commits d'une branche ne sont plus contrôlés un par un.
- **Secrets** (AS-50 AU5) : `gitleaks` tourne en CI à chaque PR (travail `security`) et sur tout l'historique chaque dimanche, et `cargo xtask check` le lance aussi en local (étape `gitleaks`, [xtask](xtask.md) §1). La **protection des poussées** de GitHub (`secret_scanning_push_protection`) reste active. Sans crochet `pre-push`, un secret hors des motifs de GitHub peut atteindre une branche publique avant la CI : ce risque est accepté. **Tout secret signalé, par la CI ou par GitHub, est révoqué**, jamais seulement effacé de l'historique.
- **Configuration de gitleaks** : `tools/dev/hooks/gitleaks.toml` reste à sa place, parce que la CI et l'étape `gitleaks` la lisent sur `origin/main` (G5). Une exception passe toujours par une PR qui modifie ce fichier.

**Retirer `core.hooksPath` des copies existantes** (AS-50 AU4). Tant qu'il désigne `TS_CACHE\hooks\<empreinte>`, l'ancienne copie épinglée des crochets continue de tourner, avec ses refus. Après la fusion du retrait, archi-sécu le retire, une fois, depuis la copie principale (la configuration est commune à tous les worktrees) :

```
git config --unset-all core.hooksPath
git config --show-origin --get-all core.hooksPath   # ne doit plus rien afficher
```

Si une copie le définit ailleurs (`config.worktree`, `includeIf`, configuration globale), la seconde commande le montre : le retirer au même endroit. Le dossier `TS_CACHE\hooks\` peut ensuite être supprimé à la main. Ces commandes écrivent la configuration de Git : les gardes de Claude Code les refusent aux sessions tant qu'ils sont en place (§3.2).

## 3. Garde-fous de Claude Code

**Retrait prévu (AS-50)** : `guard-git` et `guard-main` doivent laisser la place à des refus natifs de Claude Code, des règles `permissions.deny` dans `.claude/settings.json`, évaluées sans lancer `pwsh` (AU1). Ce changement touche aux réglages de Claude Code et à `CLAUDE.md` : il se fait dans une PR à part, sur une branche `claude/outillage-*` (SEC0-35), sur instruction directe de l'utilisateur. D'ici là, les gardes ci-dessous restent en place.

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
- `main` s'entend **en minuscules** : le *ruleset* ne garde ni `Main` ni `MAIN`, refusés comme destination (ils masqueraient `origin/main` sous Windows). Les options à valeur séparée (`-o`, `--push-option`, `--repo`, `--receive-pack`, `--exec`) sont reconnues aussi abrégées, pour que leur valeur ne passe pas pour le distant. `--force` avec `--tags` ou `--follow-tags` est refusé, puisque les étiquettes seraient forcées aussi ;
- refus de `--all`, de `--branches`, de `--mirror`, des refspecs de correspondance (`:`) ou à motif (`*`), de `refs/remotes/` et du dépôt local `.` ;
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

La forme `-Check -SourceCommit <sha>` (AS-38, §6.2) n'en fait pas partie : le garde la refuse aux sessions. Elle était réservée à l'exception de réparation de `cargo xtask merge`, retirée par AS-50 : plus rien ne la lance.

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

### 3.3 Fusion : `cargo xtask merge <PR>`

`cargo xtask merge` a remplacé `merge-pr.ps1` (T3) : c'est le **seul chemin de fusion**. C'est un chemin sensible, validé par archi-sécu. Le détail est dans [xtask](xtask.md) §6. Depuis AS-50 (#55), il ne contrôle plus que la PR elle-même, en une vingtaine d'appels : ni l'historique de `main` (`verify-main`), ni les réglages GitHub (`-Check`), ni les crochets, ni le binaire (J2), ni l'exception de réparation (AS-38). Dans l'ordre, le premier contrôle qui échoue arrête tout. Les numéros sont ceux qu'affiche la commande (`[1]`, `[3-4]`, etc.) :
- **[1] Copie principale** propre, sur `main`, **égale à `origin/main` dès le lancement** (après `git fetch`), et lancement depuis cette copie. La commande est refusée en mode CI. Ce qui reste de J2, sans coût : `cargo xtask` reconstruit le binaire depuis la copie propre avant de le lancer, donc une copie égale à `origin/main` donne un binaire construit depuis `main`. Une copie en retard est mise à jour par `git pull --ff-only`, puis la fusion est refusée : il suffit de relancer `cargo xtask merge <PR>`, qui reconstruit `xtask` depuis le nouveau `main`.
- **[2] Aucune *issue* d'anomalie ouverte** (AS-50 AU2, AU3 ; AS-52) : une seule lecture paginée de **toutes** les *issues* ouvertes, quels que soient leur auteur et leurs étiquettes (sans filtre `creator` depuis AS-52), puis le filtre `github::is_anomaly` en local. Une lecture en échec refuse la fusion. Suspendent toutes les fusions :
  - une *issue* écrite par le propriétaire (même filtre que P3) dont le titre est celui d'une anomalie de `verify-main` (commit de `main`, ou écart des réglages GitHub), ouverte par `cargo xtask verify-main` lancé en local ;
  - **l'*issue* de la tâche de nuit** (AS-52, AU8) : titre exact « verify-main : anomalie détectée par la tâche périodique », auteur exact `github-actions[bot]` de type `Bot`. Le travail `report` de `nightly.yml` l'ouvre quand `verify-main` échoue ou est annulé ([ci](ci.md) §4). Rien d'autre n'est ajouté : un autre robot, un autre titre (dont « Tâche périodique en échec »), un utilisateur qui reprendrait ce titre, ou une PR ne comptent pas.

  `--dry-run` poursuit les autres contrôles, mais son résultat reste un refus. Une seule définition sert au blocage (`github::is_anomaly`) ; la réutilisation par `verify-main` n'en garde que les *issues* du propriétaire : il ne réutilise une *issue* ouverte que si elle a un titre d'anomalie de `cargo xtask`, est écrite par le propriétaire et porte, sur une ligne à part, la clé de l'anomalie (`Clé : <SHA>` ou `Clé : repo-settings-drift`). Une remontée qui cite seulement la clé n'absorbe donc jamais une anomalie, et l'*issue* de la nuit n'est jamais réutilisée. Une anomalie trouvée la nuit arrête donc les fusions dès la fin de la tâche de nuit. On la lève (J5, ou `-Apply`), puis on ferme l'*issue* ; pour celle de la nuit, la marche à suivre est dans [ci](ci.md) §4 (« Fermer l'*issue* d'anomalie de la nuit »). **Limite** (l'exception AS-38 est retirée) : la PR qui corrige l'anomalie est elle-même refusée tant que l'*issue* est ouverte. On ferme l'*issue* juste avant de la fusionner ; si la correction n'aboutit pas, `verify-main` la rouvre au passage suivant.
- **[3-4] État de la PR** : ouverte, **venue du dépôt lui-même** (`isCrossRepository` faux : jamais une PR de *fork*), base `main`, hors brouillon, `MERGEABLE`, en retard de 0 commit sur `main`, titre conforme (A31).
- **[5] `ci-ok`** vert sur le SHA de tête, venu de `ci.yml` (G3).
- **[6] Liste complète des fichiers**, par l'API `pulls/<n>/files` paginée. Un renommage compte ses deux chemins (I10). Le nombre de fichiers doit égaler `changedFiles`. L'arbre de tête est refusé s'il contient des chemins égaux à la casse près, ou un chemin non ASCII sous un chemin sensible (B1).
- **[7] Chemins sensibles** : la liste `tools/ci/sensitive-paths.txt` d'`origin/main` (G8).
- **[8-9] Workflows** de la tête, contrôlés avec le code de `main` (J1), et actions épinglées (J8), **seulement si la PR modifie `.github/`** (comparé sans tenir compte de la casse). Sinon, les workflows sont ceux de `main`, déjà contrôlés à leur fusion.
- **[10] Approbations**, pour un chemin sensible : `review-ok` **et** `archsec-ok`, chacun en première ligne d'un commentaire de la PR. **Seuls comptent les commentaires du propriétaire du dépôt** (AS-45 P3) : `user.login` égal au propriétaire du *slug*, `author_association: OWNER` et `user.type: User`. Tout autre commentaire est ignoré, dans `merge`, `verify-main` et le report d'AS-40. L'approbation doit être :
  - sur le SHA de tête ;
  - **ou reportés d'un SHA antérieur `A` de la même PR**, après une simple mise à jour depuis `main` (AS-40). Toutes les conditions W1 à W7 doivent être remplies ([xtask](xtask.md) §6.1). `merge` affiche alors `<avis> carried over from <A> to the head <tête> (AS-40, W1 to W5 hold)`. Le report ne coûte que quelques lectures, et seulement quand l'avis n'est pas sur la tête.

  Une **résolution de conflit** dans un fichier de la PR, un fichier ajouté ou retiré, ou un changement de `main` dans un fichier de la PR empêchent le report : l'approbation doit alors être **reposée sur la tête**. Le refus nomme la condition qui a échoué.
- **[11] Fusion** : l'état de la PR est relu. Puis `gh pr merge <PR> --squash --delete-branch --match-head-commit <sha>` et `git pull --ff-only`.

`--dry-run` fait tous les contrôles, sans fusion ; hors de la copie principale, son résultat est au mieux « OK sauf copie principale » (code 3).

**Durée** (#55) : une PR hors des chemins sensibles et de `.github/` coûte 24 processus (12 `git`, 12 `gh`), contre environ 500 avant AS-50 sur l'historique du 01/11 (49 commits), dont la relecture GitHub de chaque commit sensible, `hooks verify` et `repo-settings.ps1 -Check`. Une PR sensible ajoute la lecture des commentaires et `git rev-parse` ; une PR qui modifie `.github/` ajoute J1 et J8.

**Ce qui n'est plus vérifié à la fusion** (AS-50) et où il l'est encore :
- l'historique de `main` : `cargo xtask verify-main`, lancé seul, et chaque nuit (`--origin 7d`, `all` le dimanche) ; une anomalie ouvre une *issue* qui arrête les fusions ([2]), dès la nuit (AS-52) ;
- les réglages GitHub : `repo-settings.ps1 -Check`, lancé chaque jour par l'orchestrateur (AU3, §6.3) ; un écart ouvre une *issue* qui arrête les fusions ([2]) ;
- les crochets : retirés (§2) ;
- le binaire construit depuis `main` (J2) : l'empreinte est retirée ; il reste la règle de l'étape [1] (copie égale à `origin/main` au lancement).

### 3.4 Coût, limites et risques connus

**Coût** : chaque appel d'un outil de commande lance `pwsh`, soit environ 1,2 à 1,8 s, dont 0,8 s pour le seul démarrage de `pwsh`. **Reporté à T3 (décidé)** : un garde compilé dans `xtask` le ramènera à quelques dizaines de millisecondes.

**Ce que les garde-fous n'attrapent pas** :
- un script écrit puis exécuté (`bash x.sh`, `pwsh -File x.ps1`) ;
- un interpréteur qui construit un nom de fichier par morceaux.

**Risques connus**, relevés à la deuxième relecture et non corrigés (décidé) :
- **m5** : une course entre trois sessions dans `Remove-OrphanLease` peut faire croire à deux sessions qu'elles tiennent le même bail. La fenêtre est de l'ordre de la milliseconde. Parade possible : un verrou `target\slots.lock` ouvert avec `FileShare.None` (T3).
- **m6** : `.gitignore` restreint `id_*` de SEC0-09 à `id_rsa*`, `id_dsa*`, `id_ecdsa*` et `id_ed25519*`, puisque `id_*` attraperait des sources légitimes. Cet écart est à noter au carnet d'archi-sécu.

**Risque résiduel (R-ARCH-04)** : une session peut modifier le `.claude/settings.json` de son propre worktree, par un moyen que le crochet ne voit pas, et retirer ainsi ses garde-fous. Restent alors :
- le *ruleset* du serveur (§6.4) ;
- la CI (`ci-ok`, `gitleaks`) ;
- la fusion par `cargo xtask merge`, avec `ci-ok` et les approbations du propriétaire.

Les crochets Git épinglés et la vérification d'intégrité de SEC0-35 par `merge` sont retirés par AS-50.

## 4. Tests

Tous travaillent dans des dépôts jetables sous `TS_CACHE\tmp\`, jamais dans le vrai dépôt :

```powershell
pwsh -NoProfile -File tools/dev/test-guards.ps1        # table des commandes refusées et permises (guard-git, guard-main)
pwsh -NoProfile -File tools/dev/test-slots.ps1         # logique des baux, dans un TS_CACHE jetable
pwsh -NoProfile -File tools/dev/test-slots.ps1 -Real   # plus les vérifications du §6.3, dans les vrais emplacements
pwsh -NoProfile -File tools/dev/test-repo-settings.ps1 # invariants, comparaison ; arguments et refus S4 dans un dépôt jetable dont origin est local ; script entier avec un faux gh, dépôt public et privé, chaque réglage d'AS-45 affaibli (AS-43, AS-45) : jamais d'appel à GitHub
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

C'est un chemin sensible. Il a six sections (deux ajoutées par AS-45, au passage du dépôt en public) :

| Section | Contenu |
|---|---|
| `ruleset_main` (R1) | *Ruleset* actif sur `refs/heads/main`, **sans contournement** (`bypass_actors: []`, le jeton administrateur des sessions est lié lui aussi) :<br>- PR obligatoire, 0 approbation (un seul compte : les approbations restent `review-ok` et `archsec-ok`, vérifiés par `cargo xtask merge`), fusion par squash seulement ;<br>- `ci-ok` requis, **lié à l'application GitHub Actions** (`integration_id` 15368), branche à jour (`strict`) ;<br>- historique linéaire, poussée forcée et suppression interdites. |
| `ruleset_pilotage` (R3) | `refs/heads/pilotage` : poussée forcée et suppression interdites. La poussée directe reste permise. Aucun contournement. |
| `repo` (R2, S1, AS-45) | Squash seulement (`allow_merge_commit` et `allow_rebase_merge` à `false`), titre du squash = titre de la PR, branche supprimée après fusion, **fusion automatique désactivée**. **Dépôt public** (`visibility: public`) ; `security_and_analysis` : `secret_scanning`, `secret_scanning_push_protection` et `dependabot_security_updates` à `enabled`. |
| `actions` (R4, S5, AS-45) | `allowed_actions: selected` avec `sha_pinning_required` ; motifs = liste fermée de SEC0-30 (`actions/checkout@*`, `actions/cache@*`, `actions/cache/restore@*`, `actions/cache/save@*`, `Swatinem/rust-cache@*`, `actions/upload-artifact@*`), sans les actions de GitHub ni les vérifiées ; jeton en lecture seule, aucune approbation de PR par les Actions ; **workflows des PR venues d'un *fork* soumis à approbation pour tout contributeur extérieur** (`fork_pr_contributor_approval: all_external_contributors`). Le réglage des dépôts privés (`fork-pr-workflows-private-repos`, qui répond 422 sur un dépôt public) n'est plus suivi. |
| `private_vulnerability_reporting` (AS-45) | Signalement privé des vulnérabilités activé (`enabled: true`). |
| `interaction_limits` (AS-45) | Limite d'interaction `collaborators_only`, renouvelée par `-Apply` pour la durée la plus longue de l'API (`six_months`). `-Check` compte un écart si elle est absente, différente, ou expire dans moins de 14 jours. |

**Dépôt privé (décidé, AS-45)** : il est refusé comme non public. `-Check` relève l'écart `repo.visibility` (code 1) ; un point d'accès réservé aux dépôts publics qui répond 422 est un écart, et non une erreur.

`integration_id` 15368 est l'`app.id` des contrôles `ci-ok` existants (relevé le 30/10 par `gh api`). Le motif `actions/cache@*` ne couvre pas `actions/cache/restore@…` : chaque sous-action est nommée.

### 6.2 `tools/dev/repo-settings.ps1 -Check | -Apply`

Deux modes exclusifs, plus la forme réservée ci-dessous ; tout autre argument est refusé (code 2).

**Forme réservée `-Check -SourceCommit <sha>` (AS-38)** : `-Check`, mais avec le fichier du commit `<sha>`, lu par `git show <sha>:tools/ci/repo-settings.json`, jamais sur le disque. Le garde la refuse aux sessions (§3.2). Depuis AS-50, `cargo xtask merge` ne la lance plus (l'exception de réparation est retirée) : elle reste dans le script, sans usage, jusqu'à une PR qui le simplifiera.
- Le SHA doit compter exactement 40 caractères hexadécimaux minuscules, le commit doit être présent localement après `git fetch`, et le fichier doit exister dans ce commit. Sinon : code 2.
- Le fichier passe d'abord les invariants S2 de ce script, celui de la copie principale (AS-41) : un fichier affaibli donne le code 1, sans appel à GitHub.
- Les préalables S4 restent tous en place, et les workflows comparés restent ceux d'`origin/main`.
- `-Apply` n'a pas de forme `-SourceCommit`.

- **Source** : le fichier est lu par `git show origin/main:tools/ci/repo-settings.json` (ou `<sha>:…` pour la forme réservée), **jamais sur le disque**. Une modification d'un worktree ne peut donc pas être appliquée avant d'avoir été fusionnée.
- **Ordre des préalables**, chacun avec son message (code 2, sauf l'étape 6, qui rend le code 1) :
  1. arguments (et, pour la forme réservée, format du SHA) ;
  2. préalables S4 ci-dessous ;
  3. `git fetch origin` et égalité avec `origin/main` ;
  4. présence du fichier sur `origin/main` (sinon il le dit et s'arrête) ;
  5. pour la forme réservée : présence du commit `<sha>` après `git fetch`, puis présence du fichier dans ce commit ;
  6. **invariants S2** du fichier lu : un fichier refusé s'arrête là, avec le code 1, avant tout appel à GitHub ;
  7. enfin, distant `origin` sur `github.com`.
- **Préalables** (S4) : lancé depuis la racine de la copie principale, script de cette copie (pas celui d'un worktree), copie propre, sur `main`, égale à `origin/main`. Sinon : code 2.
- **Invariants codés dans le script** (S2, élargis par AS-41). **Chaque champ du fichier a sa valeur sûre fixée dans le script**, et les clés sont fermées : un champ inconnu, absent, d'un autre type (`"false"` en texte) ou d'une autre valeur est refusé. Seuls les paramètres neutres d'AS-30 (`automatic_copilot_code_review_enabled: false`, `required_reviewers: []`) peuvent s'ajouter, et seulement avec leur valeur neutre. Les tables du script couvrent :
  - les deux *rulesets* : `name`, `target: branch`, `enforcement: active`, `bypass_actors: []`, `conditions` qui visent la seule branche ; règles exactement `deletion`, `non_fast_forward`, `required_linear_history`, `pull_request`, `required_status_checks` pour `main`, et `deletion`, `non_fast_forward` pour `pilotage`, sans doublon, sans paramètres pour les trois premières ;
  - `pull_request` : `allowed_merge_methods: ["squash"]`, `required_approving_review_count: 0`, `dismiss_stale_reviews_on_push`, `require_code_owner_review`, `require_last_push_approval`, `required_review_thread_resolution` et `require_extra_approval_for_unattributed_changes` à `false` ;
  - `required_status_checks` : `strict_required_status_checks_policy: true`, `do_not_enforce_on_create: false`, et exactement `[{context: ci-ok, integration_id: 15368}]` ;
  - `repo` : ses huit clés (branche par défaut et autres champs ne passent jamais par `-Apply`), squash seulement, sans fusion automatique, branche supprimée après fusion, titre `PR_TITLE`, `visibility: public`, et les trois clés de `security_and_analysis` à `enabled` ;
  - Actions : `enabled`, `allowed_actions: selected`, `sha_pinning_required`, ni actions de GitHub ni vérifiées, motifs dans la liste fermée, jeton en lecture, aucune approbation par les Actions, et `approval_policy: all_external_contributors` pour les *forks* ;
  - `private_vulnerability_reporting.enabled: true` ; `interaction_limits` : `limit: collaborators_only`, `expiry: six_months`.

  Le fichier est refusé aussi si une action utilisée par les workflows d'`origin/main` n'est couverte par aucun motif (`-Apply` casserait la CI). Affaiblir une règle, ou changer une valeur, demande donc de modifier aussi le script. Un test par champ le vérifie (`test-repo-settings.ps1`).
- **`-Check`** (lecture seule, `GET` seulement) compare GitHub au fichier et affiche chaque écart, avec la valeur attendue et la valeur lue ; code 1 s'il y en a un. Il signale :
  - un fichier refusé par les invariants S2, **avant tout appel à GitHub** (AS-41). La comparaison et l'attestation ci-dessous ne viennent qu'ensuite, avec le fichier accepté, aussi bien sous la forme `-SourceCommit` ;
  - un *ruleset* attendu absent, en double, ou différent (règles indexées par type, listes comparées sans tenir compte de l'ordre) ;
  - tout *ruleset* inconnu qui vise `main` ou `pilotage` (S3 ; motifs `~ALL`, `~DEFAULT_BRANCH` et jokers compris, de façon prudente) ;
  - toute **protection de branche classique** encore présente, sur n'importe quelle branche (S3). `branches?protected=true` compte aussi les branches visées par un *ruleset* : chaque branche listée est relue par `branches/<nom>/protection`. Seul le 404 dont le message est « Branch not protected » vaut « aucune » ; tout autre 404 arrête `-Check` (code 2) ;
  - au-delà de 100 *rulesets* ou de 100 branches protégées, `-Check` s'arrête (code 2) : la pagination n'est pas gérée ;
  - toute différence des champs de `repo`, des quatre ressources d'Actions et du signalement privé des vulnérabilités ; une ressource illisible (404, 409, 422) est un écart ;
  - une limite d'interaction absente, différente, ou qui expire dans moins de 14 jours (AS-45) ;
  - **l'attestation qui remplace l'essai R6.4** (AS-31, T3e-2) : `current_user_can_bypass` doit valoir `never` sur le *ruleset* `main` (lu par `GET` sur ce *ruleset*). Ce champ reste dans la liste des champs ignorés par la comparaison ci-dessous, puisqu'il ne figure pas dans le fichier : il est attesté à part, pour `main` seulement. Les règles effectives de `main` (`rules/branches/main`) doivent contenir `deletion`, `non_fast_forward`, `required_linear_history`, `pull_request` et `required_status_checks`. Sinon, c'est un écart (code 1), qui suspend les fusions (E4).

  Champs ignorés, liste explicite : `id`, `node_id`, `source`, `source_type`, `_links`, `created_at`, `updated_at`, `current_user_can_bypass` d'un *ruleset* ; les champs de l'objet du dépôt absents du fichier ; `selected_actions_url`. Un paramètre de règle présent sur GitHub et absent du fichier est un **écart** (par exemple `do_not_enforce_on_create: true`, qui affaiblirait la règle ; le fichier le fixe donc à `false`). Seule exception, une liste fermée et commentée dans le script, et seulement avec sa valeur neutre : `automatic_copilot_code_review_enabled: false` et `required_reviewers: []`, que GitHub peut renvoyer sans qu'on les envoie. Elle évite qu'un `-Apply` conforme laisse un écart qui suspendrait toutes les fusions.
- **`-Apply`** : refuse un fichier invalide sans rien écrire. Sinon, crée (`POST`) ou remplace (`PUT`, identifiant retrouvé par nom) les deux *rulesets*, règle le dépôt (`PATCH` sans `security_and_analysis`, puis un second `PATCH` pour l'analyse de secrets et la protection des poussées, puis `PUT vulnerability-alerts` et `PUT automated-security-fixes` pour Dependabot) et les quatre ressources d'Actions (`PUT`), active le signalement privé des vulnérabilités (`PUT`), renouvelle la limite d'interaction (`PUT`, `six_months`), puis relance `-Check` et échoue si un écart subsiste. Il ne supprime ni *ruleset* inconnu ni protection classique : `-Check` les signale.

Codes de sortie : 0 conforme, 1 écart ou écriture refusée, 2 usage, préalable non rempli ou erreur.

### 6.3 Qui lance quoi

- **`-Apply`** : l'**orchestrateur**, après la fusion de la PR qui modifie `repo-settings.json` ou le script, depuis la copie principale à jour. Ordre de R6 : fusion, `-Apply`, `-Check` vert, puis essai de poussée directe vers `main`, qui doit être refusé par GitHub. Entre la fusion de T3d et le premier `-Apply`, `-Check` est en écart : les fusions sont suspendues jusqu'à `-Apply`.
- **`-Check`** (AS-50 AU3) : l'**orchestrateur** le lance **chaque jour**, et après tout changement de réglages, depuis la copie principale, par `cargo xtask verify-main`, qui le lance après l'historique (décidé : la tâche de nuit tourne en `--dry-run`, et ne contrôle pas les réglages ; depuis AS-52, son travail `report` ouvre l'*issue* d'anomalie de la nuit, qui arrête aussitôt les fusions, et ce passage local ouvre l'*issue* détaillée de chaque anomalie, J5). `cargo xtask merge` ne le lance plus. **Après une *issue* d'anomalie de la nuit** : l'orchestrateur lance ce `verify-main` local sans attendre, puis ferme l'*issue* de la nuit selon [ci](ci.md) §4 (« Fermer l'*issue* d'anomalie de la nuit »). Le contrôle doit tourner **en local**, puisqu'il est sauté en CI. `verify-main` lance `pwsh -NoProfile -NonInteractive -File <copie principale>\tools\dev\repo-settings.ps1 -Check` :
  - code 1 (écart) : une *issue* `role:architecture` de clé `repo-settings-drift` est ouverte (ou réutilisée) ; tant qu'elle est ouverte, `cargo xtask merge` refuse toute fusion (§3.3, étape [2]). On la ferme après `-Apply` et un `-Check` vert. `repo-settings.ps1 -Check` lancé seul n'ouvre pas d'*issue* : l'orchestrateur l'ouvre lui-même, ou relance `verify-main` ;
  - autre code (préalable, réseau) : `verify-main` échoue ; en `--dry-run`, c'est un avertissement et le résultat reste un refus ;
  - **en CI** (`CI=true` et `GITHUB_ACTIONS=true`), le contrôle est **sauté** avec un avertissement dans le résumé du travail : `GITHUB_TOKEN` n'a pas les droits d'administration. `cargo xtask merge` refuse toujours de tourner en mode CI ;
  - l'historique est vérifié avant les réglages : un `-Check` qui échoue ne cache pas une anomalie de l'historique.

### 6.4 Côté serveur, côté local

| Barrière | Serveur (GitHub) | Local |
|---|---|---|
| PR obligatoire, `ci-ok` de GitHub Actions, branche à jour, historique linéaire | *ruleset* `main` | `cargo xtask merge` (G3, toujours en place, E5) ; chaque nuit, `verify-main` pour les commits sensibles, J3 et G3 sur les commits de la semaine par `--origin 7d`, et J3 sur tout l'historique par `all` le dimanche (AS-42, AS-44) ; une anomalie ouvre une *issue* qui arrête `merge` (AS-50 AU2) |
| Poussée forcée vers `main` | *ruleset* `main` | — (retiré par T3e-1 ; `reference-transaction` retiré par AS-50) |
| Poussée forcée vers `pilotage` et les autres branches | *ruleset* `pilotage` (aucun pour les branches de tâche) | `guard-git` (tant qu'il est en place, §3) |
| Suppression de `main` et de `pilotage` | *rulesets* | `guard-git` (tant qu'il est en place, §3) |
| Poussée directe vers `main` | *ruleset* `main` | — (retiré par T3e-1) |
| Squash seulement, pas de fusion automatique | réglages du dépôt | `cargo xtask merge` (seul chemin de fusion) |
| Actions permises, épinglage par SHA | réglages d'Actions | étape `workflows` de `xtask check` ; J1 et J8 dans `merge` pour une PR qui modifie `.github/` |
| Avis `review-ok` et `archsec-ok` sur les chemins sensibles | — (un seul compte) | `cargo xtask merge`, `verify-main` (J4) |
| Secrets | protection des poussées (`secret_scanning_push_protection`) | `gitleaks` en CI et dans `cargo xtask check` ; tout secret signalé est révoqué (AS-50 AU5) |
| Dérive des réglages eux-mêmes, contournement possible du *ruleset* `main` | — | `repo-settings.ps1 -Check` (attestation comprise), lancé chaque jour par l'orchestrateur (AS-50 AU3) |

**Risque résiduel (R-ARCH-05)** : un script écrit puis exécuté par une session peut appeler `gh api` en écriture avec le jeton administrateur, ce que `guard-git` ne voit pas (§3.4). Les *rulesets* sans contournement le lient pour `main` et `pilotage`, mais il peut modifier les réglages eux-mêmes : `-Check` le détecte après coup, au plus tard au contrôle du jour suivant.

**AS-50** : les crochets Git locaux et les contrôles de l'historique, des réglages et des crochets dans `merge` sont retirés. Ce qui protège encore `main` : le *ruleset*, la CI, la protection des poussées de GitHub, et dans `merge`, `ci-ok` (G3) et les approbations du propriétaire. Un écart de réglages ou d'historique ne bloque plus la fusion par lui-même : il est détecté après coup, et son *issue* arrête les fusions.

T3d ne retire **aucune** barrière locale (S6). Les retraits viennent ensuite, dans l'ordre d'AS-31 : T3e-1 (refus de poussée et de poussée forcée vers `main` dans `pre-push` et `guard-git`, **fait**), l'essai R6.4, remplacé par l'attestation de GitHub que `-Check` relit à chaque exécution (§6.2 ; sauté en CI, comme tout `-Check` ; depuis AS-50, lancé chaque jour par l'orchestrateur et non plus par `cargo xtask merge`), puis T3e-2 (contrôle d'origine de `verify-main` pour les commits non sensibles, **fait** ; son cache local, qui ne servait plus qu'à ce contrôle, est retiré avec lui). AS-50 retire ensuite les crochets Git (`gitleaks` en `pre-push`, E2, compris), puis, dans une PR à part, `guard-main` et la liste blanche de `git` et de `gh`.
