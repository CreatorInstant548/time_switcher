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

**Artefacts périmés d'un autre worktree** (#56, quatre occurrences le 30/10 et le 01/11). Les artefacts des crates de l'espace de travail portent le même nom d'un worktree à l'autre (Cargo les nomme d'après leur chemin relatif à la racine), et l'empreinte de Cargo n'est qu'une comparaison de dates : une sortie écrite par un autre worktree peut paraître à jour. Symptômes : un test de documentation qui lie le mauvais `rlib` (« no `Canonical` in the root » alors que les sources sont celles de `main`), des imports périmés dans un binaire de test, ou un binaire de test qui garde le chemin d'un autre worktree (`env!("CARGO_MANIFEST_DIR")`, `env!("CARGO_BIN_EXE_ts-sim")`, `file!()`) et échoue avec `NotFound`. Cargo ne recompile pas quand seule cette valeur change.
- **Parade (décidé : la moins coûteuse et la plus sûre)** : l'emplacement note dans `last-worktree.txt` (dans l'emplacement, donc effacé par `cargo xtask disk --clean`) le dernier worktree qui l'a pris. À l'`-Acquire`, si c'est un autre worktree, ou si la note manque, le script supprime les entrées `.fingerprint\<crate>-<16 chiffres hexadécimaux>` **des seules crates de l'espace de travail** (noms lus dans les `Cargo.toml` du worktree, y compris `fuzz/`). Cargo recompile ces crates au prochain appel ; les dépendances externes restent en cache. Les liens trouvés dans `.fingerprint` sont retirés, jamais suivis ; si la liste des crates est illisible ou vide, ou si une suppression échoue, un avertissement s'affiche et la note n'est pas écrite, donc le prochain `-Acquire` recommence. La clé est le chemin du worktree : le même chemin donne les mêmes chemins absolus, et un worktree recréé à cet endroit a des sources plus récentes, que Cargo voit.
- **Écartées** : un `CARGO_TARGET_DIR` par worktree (perd le cache partagé des dépendances, qui est la raison d'être des emplacements) ; `cargo clean -p` (lance Cargo, prend son verrou, plus lent, et reste interdit sur un emplacement tenu par une autre session) ; mettre à jour les dates des sources (fragile : ne couvre ni `include_bytes!` ni les données).
- **Si le symptôme apparaît quand même** (emplacement pris sans passer par `-Acquire`, ou avant ce correctif) : supprimer `last-worktree.txt` dans l'emplacement que la session tient, puis relancer `-Acquire` ; ou, au minimum, toucher le fichier source concerné (sans changer son contenu). Jamais `cargo clean` dans un emplacement tenu par une autre session.

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

Si une copie le définit ailleurs (`config.worktree`, `includeIf`, configuration globale), la seconde commande le montre : le retirer au même endroit. Le dossier `TS_CACHE\hooks\` peut ensuite être supprimé à la main.

## 3. Refus natifs de Claude Code et fusion

**Retrait des gardes (AS-50 AU1, décision de l'utilisateur du 02/11 : « les dev tools prennent trop de temps »).** Les gardes `guard-main` et `guard-git` (crochets `PreToolUse` qui lançaient `pwsh` à chaque appel d'un outil, soit 1,2 à 1,8 s) sont supprimés, avec leur bibliothèque `guard-lib.ps1` et leur test `test-guards.ps1`. À leur place, des règles `permissions.deny` de `.claude/settings.json`, évaluées par Claude Code lui-même, sans lancer de processus. Le seul crochet qui reste dans ce fichier est `SessionStart` (§1).

Ce qui disparaît avec eux, et qui n'est plus tenu que par la discipline des sessions, le *ruleset* et la revue : la liste blanche de `git` et de `gh`, le refus d'écrire dans la copie principale (A19), le refus de modifier `.claude/settings*.json` hors d'une branche `claude/outillage-*` (SEC0-35), le refus de `--no-verify` et des commandes de configuration de Git.

### 3.1 Ce que contient `permissions.deny`

455 règles : **91 motifs de base**, chacun écrit sous cinq formes, `Bash(…)`, `PowerShell(…)`, `PowerShell(& …)`, `Bash(cmd /c …)` et `PowerShell(cmd /c …)`. Les règles sont **générées** par `tools/dev/deny-rules.ps1`, qui porte la liste des motifs de base (§3.2) ; le tableau ci-dessous la résume, le script fait foi.

| Famille | Motifs de base (ils commencent tous par `git` ou `gh`) |
|---|---|
| Fusion (1) | `gh pr merge:*` : la fusion passe par `cargo xtask merge` (§3.3) |
| Poussée forcée ou vers `main` (31) | `git push --force:*`, `git push -f:*`, `git push --force-with-lease:*`, `git push --force*` (valeur collée : `--force-with-lease=<réf>`), `git push * --force*`, `git push * -f *`, `git push * -f`, options groupées `git push -fu*`, `git push -uf*`, `git push * -fu*`, `git push * -uf*` ; vers `main` : `git push * main`, `git push * main *`, `git push * main:*`, `git push * HEAD:main*`, `git push *:main*` (`origin <branche>:main`, et la suppression `origin :main`), `git push * refs/heads/main*`, `git push *:refs/heads/main*` ; `git push * +*` ; les mêmes après `git -C *` (11 motifs : `git -C * push *--force*`, `-f*`, `* -f`, `* -f *`, `-uf*`, `* -fu*`, `* -uf*`, `* main`, `* main *`, `*:main*`, `* refs/heads/main*`, `* +*`) |
| Écriture dans l'API de `gh` (54) | `gh api *-X DELETE*`, `gh api *--method DELETE*`, `gh api *-X PATCH*`, `gh api *--method PATCH*` (dont `PATCH repos/<slug>`, équivalent de `gh repo edit`, et `PATCH git/refs`, qui déplace une branche) ; graphies collées `-XDELETE`, `-XPUT`, `-XPATCH`, `-XPOST`, et `--method=`, `-X=` quelle que soit la méthode ; pour chacune des dix ressources `rulesets`, `/protection`, `actions/permissions`, `interaction-limits`, `private-vulnerability-reporting`, `/merge` (`pulls/<n>/merge` et `merges`), `collaborators`, `keys`, `hooks` et `environments`, quatre motifs (`-X` ou `--method` placé avant ou après la ressource, toute méthode) ; mutations GraphQL `gh api graphql*mergePullRequest*`, `*RepositoryRuleset*`, `*BranchProtectionRule*`, `*updateRepository*` |
| Réglages et secrets du dépôt (5) | `gh repo edit:*`, `gh repo delete:*`, `gh secret:*`, `gh workflow disable:*`, `gh ruleset:*` |

Ordres de poussée courants couverts : `git push -f`, `git push --force`, `git push --force-with-lease[=<réf>]`, `git push -fu origin <branche>`, `git push origin +<réf>`, `git push origin main`, `git push origin HEAD:main`, `git push origin <branche>:main`, `git push origin :main`, `git push origin refs/heads/main`, `git -C <dossier> push …` sous les mêmes formes, et les mêmes précédés de `& ` ou de `cmd /c`. Un faux refus reste possible sur une branche dont le nom contient `:main` ou se termine par ` main` : renommer la branche.

**`gh api` sans `-X` n'est pas toujours une lecture** : `-f`, `-F`, `--field`, `--raw-field` et `--input` en font un `POST`. Ces formes ne sont refusées que par les motifs GraphQL ci-dessus ; un `POST` implicite sur une autre ressource (par exemple l'ajout d'une clé de déploiement ou d'un *webhook*) passe ces règles, comme tout ce que le §3.4 énumère.

**Limite : une ceinture, pas un verrou.** La correspondance se fait par préfixe ou par motif sur le texte de la commande. Elle est donc contournable : autres options globales (`git -c <clé>=<valeur> push -f`, `git --no-pager push -f`), alias, variable, `bash -c`, script écrit puis exécuté, interpréteur qui construit le nom par morceaux. Ces refus arrêtent l'erreur, non l'attaque. **Le verrou reste le *ruleset* du serveur** (AS-50 AU1, §6.4) : PR obligatoire, `ci-ok`, historique linéaire, ni poussée forcée ni suppression, aucun contournement. Les réglages eux-mêmes sont surveillés par `repo-settings.ps1 -Check` (§6.3).

Un refus de Claude Code n'est jamais contourné : s'il gêne un travail légitime, ouvrir une *issue* `role:dev`.

**Vérification manuelle après la fusion** : dans une **nouvelle session** (les réglages sont lus au démarrage), la commande `gh pr merge 1 --help` doit être refusée par Claude Code. Rien d'autre à lancer, et surtout rien qui écrive.

### 3.2 Étendre ou modifier les refus

`.claude/settings.json` est un chemin sensible (`tools/ci/sensitive-paths.txt`). Toute modification passe par une PR sur une branche `claude/outillage-*`, avec une relecture par un sous-agent neuf, `/security-review`, puis `review-ok` et `archsec-ok` du propriétaire avant `cargo xtask merge`.

**Génération reproductible** (#119, décidé : un script plutôt qu'une liste tenue à la main, pour que les cinq formes ne divergent jamais). La liste des motifs de base vit dans `tools/dev/deny-rules.ps1` :

```
pwsh -NoProfile -File tools/dev/deny-rules.ps1 -Check   # le fichier est-il égal à la génération ? (code 1 sinon)
pwsh -NoProfile -File tools/dev/deny-rules.ps1 -Write   # réécrit le seul tableau deny, le reste du fichier intact
pwsh -NoProfile -File tools/dev/deny-rules.ps1 -List    # affiche les règles générées
```

Pour un nouveau motif : l'ajouter à la liste de base du script, lancer `-Write` (ou recopier la sortie de `-List`), relire le diff, puis le tester à la main dans une session neuve. Le travail `security` de la CI lance `-Check` à chaque PR : un fichier modifié à la main, sans le script, est refusé. L'ordre est fixe : toutes les formes `Bash(…)`, puis toutes les formes `PowerShell(…)`, puis, motif par motif, `PowerShell(& …)`, `Bash(cmd /c …)` et `PowerShell(cmd /c …)`. Plus aucun garde n'empêche une session de modifier ce fichier dans son worktree : seules la revue et le contrôle des chemins sensibles à la fusion le font (§3.4).

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

  Un changement de `main` dans un fichier de la PR n'empêche plus le report si la mise à jour n'est faite que de fusions propres de `main` et que le diff de la PR reste identique octet pour octet (AS-55, X1 à X4, [xtask](xtask.md) §6.2). `merge` affiche alors `<avis> carried over from <A> to <tête> (AS-55: clean merges of main, identical diff)`.

  Une **résolution de conflit**, même rendue à l'identique, une ligne ajoutée dans un commit de fusion, un commit ordinaire après `A`, un fichier ajouté ou retiré, ou une différence d'espaces dans les lignes de la PR empêchent le report : l'approbation doit alors être **reposée sur la tête**. Le refus nomme la condition qui a échoué.
- **[11] Fusion** : l'état de la PR est relu. Puis `gh pr merge <PR> --squash --delete-branch --match-head-commit <sha>` et `git pull --ff-only`.

`--dry-run` fait tous les contrôles, sans fusion ; hors de la copie principale, son résultat est au mieux « OK sauf copie principale » (code 3).

**Durée** (#55) : une PR hors des chemins sensibles et de `.github/` coûte 24 processus (12 `git`, 12 `gh`), contre environ 500 avant AS-50 sur l'historique du 01/11 (49 commits), dont la relecture GitHub de chaque commit sensible, `hooks verify` et `repo-settings.ps1 -Check`. Une PR sensible ajoute la lecture des commentaires et `git rev-parse` ; une PR qui modifie `.github/` ajoute J1 et J8.

**Ce qui n'est plus vérifié à la fusion** (AS-50) et où il l'est encore :
- l'historique de `main` : `cargo xtask verify-main`, lancé seul, et chaque nuit (`--origin 7d`, `all` le dimanche) ; une anomalie ouvre une *issue* qui arrête les fusions ([2]), dès la nuit (AS-52) ;
- les réglages GitHub : `repo-settings.ps1 -Check`, lancé chaque jour par l'orchestrateur (AU3, §6.3) ; un écart ouvre une *issue* qui arrête les fusions ([2]) ;
- les crochets : retirés (§2) ;
- le binaire construit depuis `main` (J2) : l'empreinte est retirée ; il reste la règle de l'étape [1] (copie égale à `origin/main` au lancement).

### 3.4 Limites et risques connus

**Ce que les refus natifs n'attrapent pas** : tout ce qui change la forme du texte de la commande (§3.1, ordre des options, alias, script, interpréteur). Le *ruleset* et `-Check` sont la seule garantie.

**Risques connus**, relevés à la deuxième relecture et non corrigés (décidé) :
- **m5** : une course entre trois sessions dans `Remove-OrphanLease` peut faire croire à deux sessions qu'elles tiennent le même bail. La fenêtre est de l'ordre de la milliseconde. Parade possible : un verrou `target\slots.lock` ouvert avec `FileShare.None` (T3).
- **m6** : `.gitignore` restreint `id_*` de SEC0-09 à `id_rsa*`, `id_dsa*`, `id_ecdsa*` et `id_ed25519*`, puisque `id_*` attraperait des sources légitimes. Cet écart est à noter au carnet d'archi-sécu.

**Risque résiduel (R-ARCH-04)** : une session peut modifier le `.claude/settings.json` de son propre worktree et retirer ainsi les refus, sans qu'aucun garde le voie (AS-50). Restent alors :
- le *ruleset* du serveur (§6.4) ;
- la CI (`ci-ok`, `gitleaks`) ;
- la fusion par `cargo xtask merge`, qui exige `ci-ok`, puis `review-ok` et `archsec-ok` du propriétaire pour `.claude/` (chemin sensible).

Les crochets Git épinglés, les gardes de Claude Code et la vérification d'intégrité de SEC0-35 par `merge` sont retirés par AS-50.

## 4. Tests

Tous travaillent dans des dépôts jetables sous `TS_CACHE\tmp\`, jamais dans le vrai dépôt :

```powershell
pwsh -NoProfile -File tools/dev/test-slots.ps1         # logique des baux, dans un TS_CACHE jetable
pwsh -NoProfile -File tools/dev/test-slots.ps1 -Real   # plus les vérifications du §6.3, dans les vrais emplacements
pwsh -NoProfile -File tools/dev/deny-rules.ps1 -Check   # refus natifs de .claude/settings.json égaux à leur génération (§3.2), aussi en CI
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
| Crochet `SessionStart` lancé par Claude Code (point 2) | **Conforme** : vérifié le 30/10 par `claude -p` (Claude Code 2.1.302) ; `pwsh` 7.6 disponible. |
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

**Forme réservée `-Check -SourceCommit <sha>` (AS-38)** : `-Check`, mais avec le fichier du commit `<sha>`, lu par `git show <sha>:tools/ci/repo-settings.json`, jamais sur le disque. Depuis AS-50, `cargo xtask merge` ne la lance plus (l'exception de réparation est retirée) : elle reste dans le script, sans usage, jusqu'à une PR qui le simplifiera.
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
| Poussée forcée vers `pilotage` et les autres branches | *ruleset* `pilotage` (aucun pour les branches de tâche) | — (retiré par AS-50 AU1 ; refus natifs de `git push -f` et `--force`, ceinture seulement, §3.1) |
| Suppression de `main` et de `pilotage` | *rulesets* | — (retiré par AS-50 AU1) |
| Poussée directe vers `main` | *ruleset* `main` | — (retiré par T3e-1) |
| Squash seulement, pas de fusion automatique | réglages du dépôt | `cargo xtask merge` (seul chemin de fusion) |
| Actions permises, épinglage par SHA | réglages d'Actions | étape `workflows` de `xtask check` ; J1 et J8 dans `merge` pour une PR qui modifie `.github/` |
| Avis `review-ok` et `archsec-ok` sur les chemins sensibles | — (un seul compte) | `cargo xtask merge`, `verify-main` (J4) |
| Secrets | protection des poussées (`secret_scanning_push_protection`) | `gitleaks` en CI et dans `cargo xtask check` ; tout secret signalé est révoqué (AS-50 AU5) |
| Dérive des réglages eux-mêmes, contournement possible du *ruleset* `main` | — | `repo-settings.ps1 -Check` (attestation comprise), lancé chaque jour par l'orchestrateur (AS-50 AU3) |

**Risque résiduel (R-ARCH-05)** : un script écrit puis exécuté par une session peut appeler `gh api` en écriture avec le jeton administrateur, ce que les refus natifs de Claude Code ne voient pas (§3.1, §3.4). Les *rulesets* sans contournement le lient pour `main` et `pilotage`, mais il peut modifier les réglages eux-mêmes : `-Check` le détecte après coup, au plus tard au contrôle du jour suivant.

**AS-50** : les crochets Git locaux et les contrôles de l'historique, des réglages et des crochets dans `merge` sont retirés. Ce qui protège encore `main` : le *ruleset*, la CI, la protection des poussées de GitHub, et dans `merge`, `ci-ok` (G3) et les approbations du propriétaire. Un écart de réglages ou d'historique ne bloque plus la fusion par lui-même : il est détecté après coup, et son *issue* arrête les fusions.

T3d ne retire **aucune** barrière locale (S6). Les retraits viennent ensuite, dans l'ordre d'AS-31 : T3e-1 (refus de poussée et de poussée forcée vers `main` dans `pre-push` et `guard-git`, **fait** puis retirés par AS-50), l'essai R6.4, remplacé par l'attestation de GitHub que `-Check` relit à chaque exécution (§6.2 ; sauté en CI, comme tout `-Check` ; depuis AS-50, lancé chaque jour par l'orchestrateur et non plus par `cargo xtask merge`), puis T3e-2 (contrôle d'origine de `verify-main` pour les commits non sensibles, **fait** ; son cache local, qui ne servait plus qu'à ce contrôle, est retiré avec lui). AS-50 retire ensuite les crochets Git (`gitleaks` en `pre-push`, E2, compris), puis, dans une PR à part (AU1), `guard-main` et `guard-git`, remplacés par les refus natifs de §3.1.
