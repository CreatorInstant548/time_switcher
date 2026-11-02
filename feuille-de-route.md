# Feuille de route

> Tenue par l'orchestrateur seul ([organisation.md](../time_switcher/docs/pilotage/organisation.md)). Le contenu détaillé de chaque phase et ses critères de fin sont dans [05-architecture.md](../time_switcher/docs/conception/05-architecture.md) §5 et dans la [revue d'architecture](../time_switcher/docs/conception/revues/architecture.md) §22 : ce fichier ne les recopie pas, il dit **où on en est, ce qui vient ensuite et qui s'en charge**. Le plan détaillé à l'intérieur d'une phase (tâches, dépendances, ordre) appartient au responsable développement, dans son carnet puis dans le tableau GitHub Project.
> Mis à jour le 02/11/2026.

## Jalons

| Phase | Contenu (résumé) | Rôles qui démarrent | Ce que tu pourras voir | État |
|---|---|---|---|---|
| Conception | Socle, 8 domaines, revues | Responsable conception | Les documents | **Terminée le 30/10** |
| 0. Fondations | Dépôt GitHub, règles des agents, noyau déterministe, CI, outils | Architecture et sécurité, puis développement | Un rapport : un monde-jouet de 1 000 entités tourne un an simulé, identique à chaque exécution | **En cours depuis le 30/10**, sur un dépôt public depuis le 01/11 ; critère de sortie prouvé (#64), point 2 de §5 validé par la qualité le 02/11 |
| 1. Prototype du cœur | Quelques centaines d'habitants pilotés par l'IA dans un morceau de ville ; économie minimale ; combat à mains nues | Qualité et essais | Une visualisation de la simulation, sans jeu | — |
| 2. Échange de pilote | Prendre et rendre un personnage, synchronisation, garde hors fiction, chasse aux exploits | — | Des rapports de robots joueurs | — |
| 3. Client Godot 2.5D solo | Se déplacer, incarner, sauvegarder, avancer le temps ; premiers assets et environ 40 animations | Contenus (au début de la phase) | **Première version jouable**, mise à ta disposition (sans obligation d'essai) | — |
| 4. En ligne | Serveur séparé, comptes, deux joueurs dans le quartier | — | Une partie à deux | — |
| 5. Extension | Ville complète, domaines complets, dialogue, entraînement, 3D | Exploitation, avant toute ouverture publique | Le jeu qui grandit, version après version | — |

## Prochaines étapes

Fermeture de la phase 0, selon [organisation.md](../time_switcher/docs/pilotage/organisation.md) §5 :

1. **Audit de sécurité** de fin de phase, par le responsable architecture et sécurité.
2. **Contrôle de conformité** à la conception, par le responsable conception, une fois les lots du développement fusionnés.
3. **Zéro *issue* ouverte** : chaque *issue* est soit corrigée, soit close avec motif, soit reportée ci-dessous.

## Reliquat par phase

Ce sont les *issues* reportées (organisation.md §5, point 5). Seul le travail neuf que 05 §5 place dans une phase plus tardive peut être reporté, et c'est l'orchestrateur qui le décide. Chaque *issue* reportée est close avec un lien vers cette section. Elle est rouverte à l'ouverture de sa phase.

### Phase 1

| *Issue* | Sujet | Rôle | Motif du report | Décidé le |
|---|---|---|---|---|
| [#94](https://github.com/CreatorInstant548/time_switcher/issues/94) | Calibrer `PARALLEL_FROM_WORK` et `PAR_ITER_FROM` sur les systèmes de la phase 1 (AS-51) | dev, puis qualité pour les mesures | Les systèmes à mesurer n'existent pas encore. Les valeurs actuelles ne changent pas le résultat. Les marqueurs pointent déjà vers #94 (PR #99). | 02/11 |
| [#71](https://github.com/CreatorInstant548/time_switcher/issues/71) (suite de la revue d'architecture §19.2) | `ts-sim run` : `--seeds`, `--jobs`, `events.parquet`, `snapshots/`, `report.html` | dev, puis qualité | C'est du travail neuf. La phase 0 n'exige qu'une exécution et `metrics.parquet` (T9b). Plusieurs graines et la trace des événements serviront à la première calibration, en phase 1 (décidé). La dette de #71 est réglée par la PR #106. À l'ouverture de la phase 1, ouvrir une *issue* neuve qui renvoie à #71. | 02/11 |

### Phase 3

| *Issue* | Sujet | Rôle | Motif du report | Décidé le |
|---|---|---|---|---|
| [#48](https://github.com/CreatorInstant548/time_switcher/issues/48) (dernier point) | Déplacer le lecteur de contenu borné dans l'hôte de jeu `ts_host`, avec `openat` ou `cap-std` | dev, avec `archsec-ok` | `ts_host` arrive en phase 3 (revue d'architecture §22). Le durcissement contre les liens est déjà fait par la #103 dans `ts_runner`. Décidé par archi-sécu (carnet `b5cfde6`). À l'ouverture de la phase 3, ouvrir une *issue* neuve qui renvoie à #48. | 02/11 |

### Phase 5

| *Issue* | Sujet | Rôle | Motif du report | Décidé le |
|---|---|---|---|---|
| [#50](https://github.com/CreatorInstant548/time_switcher/issues/50) | Faire signer par le manifeste de `ts_persist` tout l'en-tête de l'enveloppe ; la politique de signature appartient à l'hôte (exigence AS-37) | dev, avec `archsec-ok` | La signature des artefacts et du client n'arrive qu'en phase 5 (S27, securite.md §7). Les petits points m4 et m7 sont corrigés en phase 0. m5 est suivi dans #45. | 02/11 |

## Outils déjà installés

| Outil | Version | Emplacement | Date |
|---|---|---|---|
| Godot (.NET), mode autonome | 4.7.2 | `C:\Dev\tools\godot\Godot_v4.7.2-stable_mono_win64\` | 29/10 |
| SDK .NET | 10.0.403 | Installé pour toute la machine | 29/10 |
