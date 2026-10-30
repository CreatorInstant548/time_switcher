# Carnet de l'orchestrateur

> Tenu par l'orchestrateur seul. Règles : [organisation.md](../time_switcher/docs/pilotage/organisation.md). Mis à jour le 29/10/2026.

## État

- Conception terminée et cohérente. Toutes les revues sont intégrées : architecture, équilibrage, sécurité, intelligence des PNJ, signatures, génération et développement par agents.
- Les derniers reliquats ont été traités le 29/10 :

  | Fichier | Version |
  |---|---|
  | combat.md | 0.4.2 |
  | population.md | 0.7.1 |
  | ville.md | 0.9.1 |
  | ia-pnj.md | 1.0.1 |
  | social.md | 0.6.1 |

- Domaine « direction artistique et sonore » confié au responsable conception le 29/10.
- **Conception terminée le 30/10** (README v3.0) : les trois revues de clôture sont satisfaites, et le tableau des attendus mesurables est créé.
- **Phase 0 lancée le 30/10** :
  - dépôt Git local : `main` et la branche orpheline `pilotage` (worktree `C:\Dev\Claude	ime_switcher-pilotage\`) ;
  - la création du dépôt GitHub (`gh repo create --private --push`) a été **refusée par le classificateur de permissions du mode auto**, motif « Public Data-Sharing Upload ». Je ne le contourne pas. Le travail continue en **mode local** (CLAUDE.md) ;
  - responsables lancés : architecture et sécurité, et développement.

## Feuille de route

Voir [feuille-de-route.md](feuille-de-route.md).

## Sessions actives

| Session | Identifiant | Rôle |
|---|---|---|
| responsable-conception | `local_5b3dcf29-8fbb-426c-b63a-80ab0302c1a6` | Niveau 2 |
| responsable-archi-secu | `local_83246e4f-58ef-4aba-85ca-86a56a5407e8` | Niveau 2, lancé le 30/10 |
| responsable-dev | `local_111a89c2-3a9e-44cf-93b9-31c4421e6559` | Niveau 2, lancé le 30/10 |
| 7 sessions de domaine, 6 sessions de revue | identifiants dans le carnet du responsable conception | Niveau 3, pilotées par le responsable conception, affichées sous l'orchestrateur |
| conception-direction-art | `local_d27a53f4-3489-40db-b5e8-71e23e687d7a` | Niveau 3, lancée par le responsable conception |

L'ancienne session sur le voyage dans le temps (`local_df83bc0e-9984-4caa-9e36-7795996c6c4b`) a été archivée le 29/10 à la demande de l'utilisateur. Le concept reste archivé dans `docs/archives/`.

## Constats

- 30/10 : le responsable développement n'a pas pu lancer de sessions, l'application refusant une session lancée par une autre. Le responsable conception, lui, avait pu en lancer une. Repli : sous-agents isolés en worktree, avec branche et PR. Inscrit dans organisation.md §2 par la PR #3.
- 30/10 : demande du responsable développement d'inscrire `guard-git.ps1` dans `~/.claude/settings.json`. **Refusée (décidé)** : c'est la configuration personnelle de l'utilisateur, et elle touche tous ses projets. Le crochet ira dans le `.claude/settings.json` du projet, par la PR de T1, avec revue de sécurité.

- 30/10 : T1 (outillage des agents) a été fusionnée (PR #5, 93fc98a) après trois relectures. J'ai coupé court : la troisième relecture était la dernière, et seuls trois cas bloquaient (écriture sur `main`, fuite de secret, faux refus). Les gardes sont actifs. Règles reportées dans CLAUDE.md par la PR #9. T2a (espace de travail Cargo) est en cours.

## Demandes envoyées

| Date | À | Demande | État |
|---|---|---|---|
| 29/10 | responsable conception | État des lieux de la conception | En cours |
| 29/10 | responsable conception | Domaine direction artistique et sonore ; carnet `conception.md` | Envoyée |
| 29/10 | responsable conception | Note : arrivée prévue du responsable architecture et sécurité | Envoyée |
| 29/10 | responsable conception | Remplacer la section « Hiérarchie des sessions » du README par un renvoi vers `docs/pilotage/organisation.md` | Faite |

## Contraintes externes

L'utilisateur ne fait aucune action ([organisation.md](../time_switcher/docs/pilotage/organisation.md) §6).
- **GitHub** : offre gratuite. `gh` est connecté au compte `CreatorInstant548`. Dépôt privé `CreatorInstant548/time_switcher` **créé le 30/10 avec l'accord explicite de l'utilisateur**. `main` et `pilotage` sont poussés. Réglages : fusion squash seule, branche supprimée après fusion, Actions en lecture seule, alertes et correctifs Dependabot, étiquettes de rôle. PR #1 fusionnée (f60e5bc) : le mode local est levé. PR #3 ouverte (repli par sous-agents), à fusionner par le responsable développement.
- **Double authentification** : son état n'est pas lisible avec ces droits. GitHub l'impose de toute façon aux comptes qui publient du code. Ce n'est pas un prérequis du projet (décidé).
- **Actions GitHub (SEC0-30, 30/10)** : d'après l'API, le jeton des workflows est déjà en lecture seule et Actions ne peut pas approuver de PR. La liste fermée des actions n'est pas réglée, car guard-git refuse `gh api -X PUT` (SEC0-14). Pas de contournement, pas d'exception au garde (décidé) : un réglage du dépôt ne justifie pas d'ouvrir une écriture par `gh api`. Compensation : un contrôle `xtask`/CI qui refuse tout `uses:` hors de la liste fermée ou non épinglé par SHA complet. Le dev et archi-sécu l'ont noté (AS-21).
- **Actions bloqué (30/10, depuis 11:44 UTC)**
  - **Symptôme** : chaque tâche échoue en 2 à 3 s avec « recent account payments have failed or your spending limit needs to be increased ».
  - **Mesure** : environ 400 minutes facturées sur 2 000 (somme des durées des 224 tâches, Windows compté double). Le quota n'est donc pas la cause probable. Le compte a été créé le 25/10.
  - **Cause** : sans doute la facturation du compte, illisible sans la portée `user`. L'utilisateur est informé, sans qu'on lui demande d'action.
  - **Décisions** :
    - pas de fusion pour l'instant ;
    - le développement continue en local ;
    - le dev relance l'exécution 41051384808 toutes les 2 h ;
    - runner auto-hébergé écarté (installation d'un service et téléchargement d'un binaire : accord de l'utilisateur requis ; et `gh api POST` refusé par le garde) ;
    - mode dégradé `ci-local: <sha> pass` (proposé) : verdict d'archi-sécu attendu. Chemins sensibles exclus sauf accord explicite. CI relancée sur `main` au retour d'Actions.
  - **Verdict d'archi-sécu (AS-29)** : le mode dégradé est accepté avec les conditions D1 à D6, mais il y a une impasse d'amorçage. La PR qui l'ajoute ne peut pas obtenir `ci-ok`, et aucun contournement n'est permis.
  - **Décidé** :
    - le dev prépare une PR d'amorçage, qui contient le code et `tools/ci/degraded-mode.txt`, et la laisse ouverte ;
    - seul l'utilisateur peut la fusionner depuis GitHub, ou rétablir Actions. Il en est informé, sans qu'on lui demande rien ;
    - si Actions revient, le fichier est retiré et la PR est fusionnée normalement.
- **GitHub Pro (30/10, après-midi)**
  - L'utilisateur est passé à Pro et autorise l'usage de ses fonctions. Actions fonctionne de nouveau, et les exécutions 41051384808, 41053714027 et 41050288846 sont relancées.
  - Le mode dégradé (AS-29, T3c) est abandonné. L'issue #32 sera fermée quand `ci-ok` repassera au vert.
  - **Décidé : T3d**, réglages GitHub décrits dans des fichiers (`tools/github/*.json` et `tools/dev/repo-settings.ps1` avec `-Check` et `-Apply`) :
    - ruleset sur `main` : PR obligatoire, `ci-ok` requis, historique linéaire, poussée forcée et suppression interdites, liste de contournement vide, 0 approbation ;
    - ruleset sur `pilotage` : poussée forcée et suppression interdites ;
    - Actions : la liste fermée de SEC0-30, épinglage par SHA, jeton en lecture seule.
  - `guard-git` n'autorise que `pwsh tools/dev/repo-settings.ps1 -Apply`, et c'est l'orchestrateur qui le lance. Pas de fusion automatique GitHub. Le quota passe à 3 000 minutes par mois (M36).
  - CLAUDE.md passé à Pro par la PR #41 (fusionnée).
  - **Protection appliquée** : T3d fusionnée (#42, 3c80c9d), puis `-Apply` lancé par l'orchestrateur.
    - Rulesets actifs : `main` (26213563) et `pilotage` (26213564), avec une liste de contournement vide.
    - `ci-ok` est lié à `integration_id` 15368, ce que GitHub accepte : E5 est sans objet.
    - Les permissions Actions sont appliquées.
  - **Écart relevé par `-Check`** : GitHub ajoute d'office `require_extra_approval_for_unattributed_changes: true`, un paramètre non documenté.
    - **Décidé** : le fichier le fixe à false. Aucune approbation n'est exigée, puisqu'un seul compte sert à tout, et cette approbation supplémentaire risquerait de bloquer les fusions.
    - Le dev en fait une PR d'une ligne, avec `archsec-ok` et une exception étroite à la suspension de `merge`. Après sa fusion, l'orchestrateur relance `-Apply` puis `-Check`.
  - **Suite** : T3e-1, puis l'essai de poussée R6.4, puis T3e-2.
