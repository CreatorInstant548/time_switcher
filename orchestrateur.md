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

## Demandes envoyées

| Date | À | Demande | État |
|---|---|---|---|
| 29/10 | responsable conception | État des lieux de la conception | En cours |
| 29/10 | responsable conception | Domaine direction artistique et sonore ; carnet `conception.md` | Envoyée |
| 29/10 | responsable conception | Note : arrivée prévue du responsable architecture et sécurité | Envoyée |
| 29/10 | responsable conception | Remplacer la section « Hiérarchie des sessions » du README par un renvoi vers `docs/pilotage/organisation.md` | Faite |

## Contraintes externes

L'utilisateur ne fait aucune action ([organisation.md](../time_switcher/docs/pilotage/organisation.md) §6).
- **GitHub** : offre gratuite. `gh` est connecté au compte `CreatorInstant548`. Dépôt privé `CreatorInstant548/time_switcher` **créé le 30/10 avec l'accord explicite de l'utilisateur**. `main` et `pilotage` sont poussés. Réglages : fusion squash seule, branche supprimée après fusion, Actions en lecture seule, alertes et correctifs Dependabot, étiquettes de rôle. PR #1 ouverte, qui lève le mode local, à fusionner par le responsable développement.
- **Double authentification** : son état n'est pas lisible avec ces droits. GitHub l'impose de toute façon aux comptes qui publient du code. Ce n'est pas un prérequis du projet (décidé).
