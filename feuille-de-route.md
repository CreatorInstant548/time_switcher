# Feuille de route

> Tenue par l'orchestrateur seul ([organisation.md](../time_switcher/docs/pilotage/organisation.md)). Le contenu détaillé de chaque phase et ses critères de fin sont dans [05-architecture.md](../time_switcher/docs/conception/05-architecture.md) §5 et dans la [revue d'architecture](../time_switcher/docs/conception/revues/architecture.md) §22 : ce fichier ne les recopie pas, il dit **où on en est, ce qui vient ensuite et qui s'en charge**. Le plan détaillé à l'intérieur d'une phase (tâches, dépendances, ordre) appartient au responsable développement, dans son carnet puis dans le tableau GitHub Project.
> Mis à jour le 29/10/2026.

## Jalons

| Phase | Contenu (résumé) | Rôles qui démarrent | Ce que tu pourras voir | État |
|---|---|---|---|---|
| Conception | Socle, 8 domaines, revues | Responsable conception | Les documents | **Terminée le 30/10** |
| 0. Fondations | Dépôt GitHub, règles des agents, noyau déterministe, CI, outils | Architecture et sécurité, puis développement | Un rapport : un monde-jouet de 1 000 entités tourne un an simulé, identique à chaque exécution | **En cours depuis le 30/10**, en mode local |
| 1. Prototype du cœur | Quelques centaines d'habitants pilotés par l'IA dans un morceau de ville ; économie minimale ; combat à mains nues | Qualité et essais | Une visualisation de la simulation, sans jeu | — |
| 2. Échange de pilote | Prendre et rendre un personnage, synchronisation, garde hors fiction, chasse aux exploits | — | Des rapports de robots joueurs | — |
| 3. Client Godot 2.5D solo | Se déplacer, incarner, sauvegarder, avancer le temps ; premiers assets et environ 40 animations | Contenus (au début de la phase) | **Première version jouable**, mise à ta disposition (sans obligation d'essai) | — |
| 4. En ligne | Serveur séparé, comptes, deux joueurs dans le quartier | — | Une partie à deux | — |
| 5. Extension | Ville complète, domaines complets, dialogue, entraînement, 3D | Exploitation, avant toute ouverture publique | Le jeu qui grandit, version après version | — |

## Prochaines étapes

1. Le responsable conception finit le domaine direction artistique et sonore. Il intègre aussi :
   - le passage à GitHub gratuit ;
   - le nouveau document sur le contenu des signatures.
2. Phase 0, lancée par l'orchestrateur dès que le responsable conception annonce la conception terminée, direction artistique comprise (décidé : les fondations ne dépendent pas de la direction artistique, mais un seul front de travail à la fois garde la consommation et la cohérence sous contrôle) :
   - création du dépôt privé et premier envoi ;
   - lancement du responsable architecture et sécurité, qui valide le découpage ;
   - lancement du responsable développement.

## Outils déjà installés

| Outil | Version | Emplacement | Date |
|---|---|---|---|
| Godot (.NET), mode autonome | 4.7.2 | `C:\Dev\tools\godot\Godot_v4.7.2-stable_mono_win64\` | 29/10 |
| SDK .NET | 10.0.403 | Installé pour toute la machine | 29/10 |
