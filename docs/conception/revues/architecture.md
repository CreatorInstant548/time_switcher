# Revue d'architecture

> Document de **revue** (dossier `revues/`) : rédigé par la session « revue d'architecture », il ne modifie rien. La session principale le relit et intègre au socle ce qu'elle retient (liste au §24).
> Version 1.0 — 29/10/2026. Base relue : `CLAUDE.md`, [README](../README.md), socle `01` à `05` (surtout [05-architecture.md](../05-architecture.md)), les sept domaines (sections niveaux de détail, coût de calcul, impacts, et les parties qui fixent la charge réelle du moteur).
> Les choix marqués **(décidé)** sont tranchés par cette revue, avec leur raison ; les valeurs **[à calibrer]** seront ajustées au prototype ; les **objectifs** chiffrés sont des cibles de performance à vérifier par mesure.

Trois contraintes guident toute la revue ; elles sont rappelées par leur numéro :

- **C1 — Moteur détaché du rendu** : le cœur ne dépend d'aucun moteur graphique ; passer de la 2.5D à la 3D ne touche pas le cœur.
- **C2 — Entraînement facile** : cœur sans affichage, plus rapide que le temps réel, en parallèle, piloté depuis Python, avec les mêmes règles que le jeu ; seules les compétences d'exécution sont entraînées, puis exportées et exécutées dans le cœur.
- **C3 — Simulations nombreuses et variées** : mondes de toutes tailles, populations et graines différentes, scénarios scriptés, simulations accélérées de plusieurs années, tests reproductibles, comparaisons.

## Sommaire

0. [Résumé](#0-résumé)
1. [La charge réelle du moteur](#1-la-charge-réelle-du-moteur)
2. [Diagnostic de l'architecture actuelle](#2-diagnostic-de-larchitecture-actuelle)
3. [Principes de l'architecture recommandée](#3-principes-de-larchitecture-recommandée)
4. [Vue d'ensemble : processus et hôtes](#4-vue-densemble--processus-et-hôtes)
5. [Découpage en crates et en modules](#5-découpage-en-crates-et-en-modules)
6. [Modèle de données du cœur](#6-modèle-de-données-du-cœur)
7. [API du cœur](#7-api-du-cœur)
8. [Boucle de simulation et pas de temps](#8-boucle-de-simulation-et-pas-de-temps)
9. [Ordonnancement des systèmes ECS](#9-ordonnancement-des-systèmes-ecs)
10. [Niveaux de détail](#10-niveaux-de-détail)
11. [Déterminisme et reproductibilité](#11-déterminisme-et-reproductibilité)
12. [Persistance et sauvegardes](#12-persistance-et-sauvegardes)
13. [Réseau et client](#13-réseau-et-client)
14. [Service de dialogue](#14-service-de-dialogue)
15. [Comptes, pilotes et modération](#15-comptes-pilotes-et-modération)
16. [Console d'administration et tableaux de bord](#16-console-dadministration-et-tableaux-de-bord)
17. [Générateurs de ville et de population, construction d'un monde](#17-générateurs-de-ville-et-de-population-construction-dun-monde)
18. [Chaîne d'entraînement](#18-chaîne-dentraînement)
19. [Chaîne de simulation et de calibrage](#19-chaîne-de-simulation-et-de-calibrage)
20. [Performances et montée en charge](#20-performances-et-montée-en-charge)
21. [Tests et qualité](#21-tests-et-qualité)
22. [Plan de mise en œuvre](#22-plan-de-mise-en-œuvre)
23. [Récapitulatif des décisions](#23-récapitulatif-des-décisions)
24. [Modifications proposées au socle](#24-modifications-proposées-au-socle)
25. [Risques résiduels](#25-risques-résiduels)

---

## 0. Résumé

L'architecture validée est **saine dans ses fondations** : cœur Rust sans affichage, ECS, serveur autoritaire, pilote en composant, mêmes intentions pour le joueur et l'IA, vue filtrée par le serveur, trois niveaux de détail aux mêmes règles, ville et population comme données produites par des générateurs à graine. Elle répond déjà en grande partie à C1. Elle reste en revanche **muette ou fragile sur C2 et C3**, et contient deux choix qu'il faut revoir.

Principales recommandations :

1. **Reproductibilité par graine** (§11) : remplacer « pas de déterminisme strict » par une reproductibilité bit à bit **sur un même binaire et une même plateforme**, indépendante du nombre de fils d'exécution. Sans elle, ni les tests, ni les comparaisons de calibrage, ni l'entraînement, ni la reprise après panne, ni la relecture d'un bug ne sont fiables. Son coût est une discipline de code, pas une perte de performance.
2. **Deux horloges sur un même temps simulé** (§8) : un **pas fixe** de 50 ms pour la simulation complète, et un **ordonnanceur à événements datés** pour les niveaux simplifié et statistique, l'échéancier et les tâches quotidiennes. Quand plus rien n'est en simulation complète, le temps saute d'événement en événement : c'est ce qui rend possibles l'avance du temps en solo et les simulations de plusieurs années (objectif : **un an en moins de 5 minutes** pour 10 000 habitants).
3. **Les mêmes règles pour tous les hôtes** (§5) : le jeu est une fonction `ts_game::standard_rules()` qui assemble des greffons (*plugins*) de domaine ; le serveur, le solo, Python et le lanceur de simulations appellent tous cette fonction et n'ajoutent aucune règle.
4. **Une API du cœur explicite** (§7) : cinq types d'échanges — **intentions** (pilotes), **commandes** (hors fiction), **observations** (vues filtrées et tenseurs), **événements** (faits datés), **instantanés** (état) — identiques pour le client, Python, les tests et la console.
5. **Un troisième type de pilote** `Externe` (§6.6) pour les robots de test, les agents Python et les relectures, sans la sémantique joueur (synchronisation, bilan).
6. **Pas de découpage d'un monde entre plusieurs serveurs** (§20) : un monde = un processus autoritaire, parallélisé en interne ; on monte en charge verticalement et en multipliant les mondes. Les secteurs restent l'unité de chargement, de sauvegarde et de niveau de détail.
7. **Persistance par instantanés incrémentaux et journal des entrées** (§12), avec **PostgreSQL en ligne et SQLite en solo** (un joueur solo n'installe pas de serveur PostgreSQL).
8. **Le client parle au cœur par une bibliothèque native Rust** (§13) : en solo, elle héberge le serveur dans le processus ; en ligne, elle porte la connexion QUIC ; dans les deux cas, elle fournit la prédiction de mouvement avec le code même du serveur. Le C# ne contient aucune règle et aucun code réseau critique.
9. **Une chaîne d'entraînement complète** (§18) : liaisons `pyo3`, environnements Gymnasium et PettingZoo vectorisés, arènes décrites en données, export ONNX, inférence par lots dans le cœur avec `tract`, registre de modèles versionnés, compétence écrite à la main toujours disponible en repli.
10. **Une chaîne de simulation et de calibrage** (§19) : scénarios en données, lanceur `ts-sim` (exécution, balayage de paramètres, comparaison, rapport, relecture, bissection), sorties Parquet, indicateurs définis une seule fois pour le cœur, la console et les rapports.
11. **Langages tranchés** : service de dialogue en **Rust** ; console d'administration en **TypeScript** (React) sur une API Rust ; contenus en **RON** compilés en paquet de données ; analyse et entraînement en Python.
12. **Plan de mise en œuvre réordonné** (§22) : une phase 0 de fondations (noyau, temps, aléa, ordonnanceur, instantanés, lanceur, test de déterminisme, liaison Python minimale) avant tout contenu ; le niveau statistique et l'ordonnanceur entrent dès le prototype du cœur, car on ne greffe pas des niveaux de détail après coup.

---

## 1. La charge réelle du moteur

Synthèse des ordres de grandeur fixés par les domaines (tous **[à calibrer]**), qui dimensionnent l'architecture.

| Élément | Ordre de grandeur | Source |
|---|---|---|
| Habitants simulés | 10 000, extensible jusqu'à un plafond par serveur | [01-concept.md](../01-concept.md) §1.1 ; [population.md](../domaines/population.md) §7.6 |
| Personnages en simulation complète | 50 à 150 par bulle de joueur ; 100 à 300 décisions par seconde et par bulle | [ia-pnj.md](../domaines/ia-pnj.md) §6.4 |
| Personnages en simulation simplifiée | quelques milliers, dont quelques centaines d'« importants » qui ne descendent jamais en statistique | [ia-pnj.md](../domaines/ia-pnj.md) §6.2 |
| Fréquence de décision en complet | à chaque événement, et au moins toutes les 0,5 à 1 s | [ia-pnj.md](../domaines/ia-pnj.md) §6.1 |
| Blocs d'agenda | quelques-uns par personnage et par jour (≈ 60 000 résolutions par jour de jeu) | [ia-pnj.md](../domaines/ia-pnj.md) §6.4 |
| Relations | ≈ 1,5 million complètes et ≈ 5 millions compactes (≈ 100 Mo) | [social.md](../domaines/social.md) §10.2 |
| Souvenirs détaillés | 100 à 300 par personnage | [ia-pnj.md](../domaines/ia-pnj.md) §4.3 |
| Calculs quotidiens | composante Réseau sur 1,5 million d'arêtes, caches de réputation, clôture des entreprises, propagation statistique des rumeurs | [social.md](../domaines/social.md) §10.2 ; [economie.md](../domaines/economie.md) §9.4 |
| Grand livre | quelques écritures par bloc d'agenda en statistique, une par acte en complet ; de l'ordre de 10⁵ écritures par jour de jeu | [economie.md](../domaines/economie.md) §2.3, §9.2 |
| Affaires de police | quelques dizaines actives, quelques centaines d'indices | [combat.md](../domaines/combat.md) §13.3 |
| Affrontements en complet | rares hors joueurs ; coût dominé par la perception et les trajectoires | [combat.md](../domaines/combat.md) §13.3 |
| Rodage d'une population | ≈ 60 jours de jeu en statistique | [population.md](../domaines/population.md) §1.4 |
| Dialogue | borné par les quotas des joueurs, pas par la taille de la ville | [social.md](../domaines/social.md) §3.5 |

Trois enseignements :

1. **La taille de la ville n'est pas le problème** : 10 000 personnages en statistique tiennent en mémoire (< 1 Go, §20.4) et coûtent peu, à condition que ce niveau soit **piloté par événements** et non par un pas fixe.
2. **Le coût qui croît est celui des joueurs** : bulles complètes, perception, recherche de chemin, réplication réseau. C'est lui qu'il faut borner (§20).
3. **Les données globales sont fortement couplées** : graphe social, grand livre unique, affaires de police à l'échelle de l'agglomération, régulateurs démographique et économique. Elles se prêtent mal à une répartition géographique entre machines (§2.2, risque R4).

---

## 2. Diagnostic de l'architecture actuelle

### 2.1 Ce qui est solide

| Choix ([05-architecture.md](../05-architecture.md) §2) | Pourquoi c'est solide |
|---|---|
| Cœur Rust sans affichage, rendu seulement dans le client (§2.1, §3) | Répond directement à C1 ; le cœur peut tourner dans un serveur, un test, un processus Python |
| ECS `bevy_ecs` utilisé seul (§2.2) | Traitement en masse, ordonnanceur parallèle, détection de changements (utile à la sauvegarde incrémentale et à la réplication), aucune dépendance graphique |
| Pilote en composant, mêmes intentions pour joueur et IA (§2.3) | C'est la clé de C2 : un agent Python n'est qu'un pilote de plus ; l'échange de pilote devient un changement de composant |
| Observation par l'IA pendant l'incarnation, reprise dans le même tick (§2.3) | Pas d'état caché hors du cœur ; tout est composant ([ia-pnj.md](../domaines/ia-pnj.md) §0, principe 5) |
| Serveur autoritaire, vue filtrée par le serveur (§2.5) | Sécurité par construction ; et c'est déjà la bonne forme d'**observation** pour l'IA et pour l'entraînement (on n'observe que ce que le personnage perçoit) |
| Même serveur en solo et en ligne, solo passant par la même vue (§2.4 ; [interface.md](../domaines/interface.md) §1.1) | Un seul chemin de données ; le solo est un banc d'essai fidèle |
| Trois niveaux de détail aux mêmes règles, résolution plus grossière (§2.6) | Permet 10 000 habitants et l'accélération ; les tables simplifiées sont calibrées sur la simulation complète ([combat.md](../domaines/combat.md) §13.2) |
| Ville et population comme données, générateurs Rust à graine, versions (§2.11, §2.14) | Base directe de C3 (mondes de toutes tailles, populations de test, mode « morceau de ville ») |
| Économie en grand livre en partie double (§2.15) | Invariant vérifiable automatiquement (la somme des comptes ne varie que par les flux extérieurs), traçabilité, calibrage |
| IA : décisions écrites à la main, seules les compétences d'exécution apprises (§2.13) | Périmètre d'apprentissage étroit, donc entraînable et vérifiable ; décisions explicables |
| Une seule entité par organisation (§2.17) | Évite deux sources de vérité ; naturel en ECS |
| Console d'administration séparée du client, vue de conception réservée au développement (§2.20) | Aucun pouvoir dans un client modifiable |

### 2.2 Risques

| # | Risque | Gravité | Réponse de la revue |
|---|---|---|---|
| R1 | **« Pas de déterminisme strict »** (§2.10) : interprété au sens large, il autorise des sources de hasard non maîtrisées (horloge murale, ordre d'itération des tables de hachage, budgets en millisecondes, résultats asynchrones). On perd alors la reproduction des bugs, la comparaison à graine égale, la reprise après panne par rejeu, et les tests de non-régression | **Haute** (contredit C2 et C3) | Reproductibilité par graine, §11 |
| R2 | **Budgets de calcul en temps** : [ia-pnj.md](../domaines/ia-pnj.md) §3.3 et §6.4 bornent GOAP « en temps de calcul » et par tick. Un budget en millisecondes rend le résultat dépendant de la machine et de sa charge | Moyenne | Budgets en **nombre de nœuds** et en nombre d'opérations, §9.4 |
| R3 | **Pas de temps non défini** : ni fréquence de tick, ni articulation entre pas fixe et blocs d'agenda. Un pas fixe appliqué partout rendrait l'accélération et les simulations longues très coûteuses | **Haute** (C3) | Deux horloges, §8 |
| R4 | **« Un serveur par zone si nécessaire »** (§2.7) : répartir un même monde entre machines exige des transactions entre serveurs pour le grand livre, le graphe social, les affaires, les régulateurs et les déplacements. C'est le chantier le plus coûteux et le plus risqué d'un jeu en ligne, pour une charge qui ne l'exige pas | **Haute** | Un monde = un processus ; parallélisme interne ; plus de mondes, §20.3 |
| R5 | **PostgreSQL comme seule persistance** (§2.8) : inadapté au solo (installer un serveur de base de données chez le joueur) et aux milliers de simulations courtes ; risque de lier le schéma relationnel à chaque composant | Moyenne | Instantanés + journal ; PostgreSQL en ligne, SQLite en solo, fichiers pour les simulations, §12 |
| R6 | **Dialogue asynchrone et incertain** : si la simulation attend le texte du modèle, ou si le texte influence l'état, le monde devient non reproductible et sensible aux pannes du service | Moyenne | Le texte est cosmétique pour la simulation, §14.2 |
| R7 | **Prédiction côté client** (§1, « prédiction du personnage du joueur ») : elle impose de réécrire les règles de mouvement en C#, en contradiction avec « aucune règle dans le client », et avec un risque de divergence | Moyenne | Prédiction par le code Rust du serveur, livré dans la bibliothèque native du client, §13.4 |
| R8 | **QUIC en C#** : `System.Net.Quic` dépend de msquic et n'est pas pris en charge sur toutes les plateformes (Windows 10 notamment) | Moyenne | Réseau dans la bibliothèque native Rust (`quinn`), §13.2 |
| R9 | **Couplage entre domaines** : l'IA a besoin des prix, l'économie des décisions de l'IA, le social des deux, le combat de tous. Sans règle de dépendance, les crates forment des cycles et chaque modification recompile tout | Moyenne | Couche `ts_model` de données partagées et systèmes par domaine, §5.2 |
| R10 | **Équivalence entre niveaux de détail** : « mêmes issues en moyenne » est une promesse qui se dégrade silencieusement à chaque modification de règle | Moyenne | Tests d'équivalence statistique automatiques, §10.6 et §21 |
| R11 | **Identifiants d'entité ECS** : un `Entity` de `bevy_ecs` est réutilisé après destruction et change d'une session à l'autre ; s'il sert de référence dans les souvenirs, les relations, le grand livre ou le réseau, la sauvegarde et la relecture se corrompent | Moyenne | `StableId`, §6.1 |
| R12 | **Argent en virgule flottante** (non précisé) : un grand livre en flottants perd la partie double | Moyenne | Montants entiers, §6.3 |
| R13 | **Personne ne vérifie que « le monde ne connaît pas les joueurs »** : un système de règle qui lirait le composant pilote briserait la règle sans que rien ne le signale | Faible mais insidieux | Composant pilote privé, §6.6 |
| R14 | **Évolution de `bevy_ecs`** : l'API change à chaque version mineure | Faible | Version épinglée, montée de version volontaire et isolée, §25 |

### 2.3 Manques

- **Aucune API du cœur** : rien ne dit ce qui entre (intentions, commandes) et ce qui sort (vues, événements, mesures, instantanés), ni comment un outil externe pilote le cœur. C'est le manque principal pour C2 et C3.
- **Aucune chaîne d'entraînement** au-delà de « Python, sur le même cœur » ([05-architecture.md](../05-architecture.md) §4) : ni liaison, ni environnement, ni format d'export, ni inférence, ni versionnage des modèles.
- **Aucune chaîne de simulation** : ni description de scénario, ni lanceur, ni format de sortie, ni comparaison, alors que les domaines comptent sur des simulations longues pour calibrer l'économie, la démographie et la criminalité.
- **Aucune boucle de simulation** décrite : pas de temps, ordre des systèmes, rythme des niveaux de détail, synchronisation avec l'heure réelle, changement d'heure.
- **Format d'échange** entre Rust et C# non choisi ; chargement de la carte par le client non décrit.
- **Comptes et méta-jeu** (niveau de compte, profil, familiarité, bilans, amis, sanctions) non placés : dans le monde, hors du monde, quel service ?
- **Contenus en données** (fiches de rôle, recettes HTN, catalogue d'actes, fiches de métier, tables) : format, validation et versionnage non décrits.
- **Outils de visualisation sans Godot** pour regarder une simulation sans affichage ou une relecture.
- **Langages** encore ouverts : service de dialogue, console d'administration.

### 2.4 Ce qui contredit les trois contraintes

| Contrainte | Contradiction ou fragilité actuelle | Correction |
|---|---|---|
| C1 | Aucune contradiction de fond. Fragilités : prédiction de mouvement à réécrire en C# (R7) ; client qui risquerait de dépendre d'identifiants internes ou de noms d'animation | Prédiction par la bibliothèque native (§13.4) ; protocole **sémantique** (activité, geste, apparence), jamais des ressources graphiques (§13.6) |
| C2 | Pas de déterminisme (R1) ; pas de pilote externe ; pas d'API par lots ; pas de réinitialisation rapide ; risque de « chemins d'entraînement » qui contourneraient les règles | §11, §6.6, §7, §18 |
| C3 | Pas d'horloge à événements (R3) ; persistance PostgreSQL seule (R5) ; pas de description de scénario ni de lanceur ; pas de sorties mesurées communes | §8, §12, §19 |

---

## 3. Principes de l'architecture recommandée

1. **Le cœur est une bibliothèque, pas un programme** (décidé). `ts_sim` ne lit ni horloge murale, ni réseau, ni disque, ni variable d'environnement ; il reçoit des entrées datées et rend des sorties. Les **hôtes** (serveur, solo, Python, lanceur, tests) l'entourent. *Pourquoi : c'est la condition commune de C1, C2 et C3 ; tout ce qui touche au monde extérieur se trouve dans un hôte remplaçable.*
2. **Un seul jeu de règles** (décidé). Les règles sont assemblées par une seule fonction ; aucun hôte n'ajoute, ne retire ou ne modifie une règle, sauf la désactivation de greffons déclarés **désactivables** pour les arènes (§5.3). *Pourquoi : « les mêmes règles que le jeu » (C2) n'est garanti que si c'est structurellement impossible de faire autrement.*
3. **Reproductible par graine** (décidé, §11). *Pourquoi : tests, comparaisons, entraînement et relecture en dépendent.*
4. **Temps simulé entier, deux horloges** (décidé, §8). *Pourquoi : le pas fixe pour ce que le joueur voit, les événements datés pour tout le reste.*
5. **Organisation par domaine, pas par niveau de détail** (décidé). Chaque règle a ses trois résolutions (complète, simplifiée, statistique) **dans le même module**, avec les mêmes paramètres et un test d'équivalence. *Pourquoi : c'est la seule façon de tenir dans la durée la règle « mêmes règles, résolution plus grossière ».*
6. **Le monde ne connaît pas les joueurs, par construction** (décidé, §6.6). *Pourquoi : règle du socle ([01-concept.md](../01-concept.md) §1.1) ; elle doit être vérifiée par le compilateur, pas par la vigilance.*
7. **Les données hors fiction restent hors du monde** (décidé). Journaux d'incarnation, comptes, familiarités, modération, mesures : produits par le cœur sous forme d'événements, stockés par les hôtes ; aucun système de règle ne peut les lire. *Pourquoi : c'est ce qui garantit que le journal des actions ne sert jamais de preuve dans le monde ([combat.md](../domaines/combat.md) §0).*
8. **Tout ce qui est réglable est une donnée versionnée** (décidé) : contenus, tables, paramètres, modèles entraînés. Un monde enregistre les versions qu'il utilise. *Pourquoi : une simulation ou un bug n'est reproductible que si l'on sait exactement avec quelles données il a tourné.*

---

## 4. Vue d'ensemble : processus et hôtes

### 4.1 Schéma

```
                         ┌──────────────────────────────────────────────┐
                         │ CLIENT GODOT 4 (C#)                          │
                         │ rendu 2.5D puis 3D · interface · entrées     │
                         │ audio · aucune règle                         │
                         │        │ messages ts_protocol (octets)       │
                         │ ┌──────▼─────────────────────────────────┐   │
                         │ │ ts_client_native (Rust, cdylib)        │   │
                         │ │ transport mémoire (solo) / QUIC (ligne)│   │
                         │ │ prédiction : ts_mobility               │   │
                         │ │ solo : héberge ts_host (serveur local) │   │
                         │ └──────┬─────────────────────────────────┘   │
                         └────────┼─────────────────────────────────────┘
                                  │ QUIC (en ligne)
┌─────────────────────────────────▼─────────────────────────────────────┐
│ SERVEUR DE MONDE ts_server (Rust) — un processus par monde, autoritaire│
│  hôte : sessions · rythme temps réel · pilotes · vues · journal       │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ CŒUR ts_sim + règles ts_game::standard_rules()                   │ │
│  │ (bibliothèque : aucune E/S, aucune horloge murale)               │ │
│  └──────────────────────────────────────────────────────────────────┘ │
└──┬───────────────┬────────────────────┬───────────────────┬───────────┘
   │               │                    │                   │ API interne
┌──▼─────────┐ ┌───▼──────────────┐ ┌───▼───────────────┐ ┌─▼───────────────┐
│ PostgreSQL │ │ ts_dialogue      │ │ ts_accounts       │ │ ts_admin (Rust) │
│ instantanés│ │ formulation,     │ │ comptes, niveaux, │ │ API web, rôles, │
│ journal,   │ │ classement,      │ │ profils, familia- │ │ journal, double │
│ grand livre│ │ filtres, quotas, │ │ rités, bilans,    │ │ validation      │
│ indicateurs│ │ cache            │ │ amis, sanctions   │ │      ▲          │
└────────────┘ └──────────────────┘ └───────────────────┘ └──────┼──────────┘
                                                                  │ HTTPS
                                                    ┌─────────────┴──────────┐
                                                    │ console web (TypeScript)│
                                                    └────────────────────────┘

OUTILS (mêmes crates, sans réseau ni rendu)
  ts-sim (lanceur de scénarios, balayages, comparaisons, relectures)
  ts-worldbuild (génération ville + population + rodage + validation)
  ts_py / ts_gym (Python : environnements d'entraînement, analyses)
  ts-viz (visualiseur de développement, optionnel)
```

### 4.2 Les hôtes du cœur

| Hôte | Rythme du temps | Pilotes | Persistance | Sorties |
|---|---|---|---|---|
| **Serveur en ligne** `ts_server` | Temps réel du fuseau | `Joueur`, `IA` | PostgreSQL | Vues réseau, journal, indicateurs, événements hors fiction |
| **Solo** `ts_host` dans `ts_client_native` | Temps réel, avance du temps à la demande | `Joueur`, `IA` | SQLite | Vues en mémoire |
| **Lanceur** `ts-sim` | Aussi vite que possible | `IA`, `Externe` (scripts) | Fichiers | Parquet, instantanés, rapports |
| **Python** `ts_py` | Pas à pas, par lots | `IA`, `Externe` (agents) | Mémoire, fichiers | Tableaux numpy, événements |
| **Tests** (Rust) | Pas à pas | Tous | Mémoire | Assertions, empreintes d'état |

Tous construisent le cœur de la même façon (§7.1). Le solo et le serveur partagent le même code d'hôte (`ts_host`), avec deux transports et deux stockages (décidé). *Pourquoi : c'est la forme concrète de « le même serveur en solo et en ligne » ([05-architecture.md](../05-architecture.md) §2.4).*

---

## 5. Découpage en crates et en modules

### 5.1 L'espace de travail (décidé)

Un seul dépôt, un espace de travail Cargo, un projet Godot, un paquet Python et une application web.

```
time_switcher/
├─ crates/
│  ├─ ts_kernel        noyau : temps, StableId, argent, aléa à clé, ordonnanceur,
│  │                   phases, greffons, instantanés, empreinte d'état
│  ├─ ts_model         types partagés : composants, ressources, intentions,
│  │                   événements, commandes (données seulement, aucun système)
│  ├─ ts_data          paquet de contenus : chargement RON, validation, compilation,
│  │                   versions (rôles, recettes, actes, métiers, tables, paramètres)
│  ├─ ts_world_model   modèle de la ville : terrain, rues, parcelles, bâtiments,
│  │                   plans, registre des lieux, graphes, maillages ; format de fichier
│  │
│  │  ── règles (un greffon par domaine, systèmes seulement) ──
│  ├─ ts_body          attributs, santé, blessures, fatigue, désentraînement, vieillissement
│  ├─ ts_mobility      cinématique, collisions, navigation hiérarchique, véhicules,
│  │                   transports en commun, service d'itinéraire
│  ├─ ts_perception    grille spatiale, vision, ouïe, son, ensemble perçu
│  ├─ ts_ai            besoins, émotions, agenda, sélection d'objectif, HTN, GOAP,
│  │                   projets, mémoire, observation, reprise, au repos
│  ├─ ts_skills        trait Skill, compétences écrites, compétences apprises,
│  │                   encodage des observations, inférence par lots (tract)
│  ├─ ts_combat        résolution des gestes, réflexes, armes, tables hors bulle
│  ├─ ts_justice       témoins, affaires, police, justice, détention, criminalité
│  ├─ ts_social        relations, graphe, actes sociaux, croyances, rumeurs,
│  │                   réputation, groupes, délibérations
│  ├─ ts_economy       comptes, grand livre, échéancier, entreprises, marché,
│  │                   propriété, État, régulateur économique
│  ├─ ts_population    naissances, décès, arrivées, départs, ménages, régulateur
│  ├─ ts_world         météo, saisons, calendrier, décor, dégâts, constructions
│  ├─ ts_lod           bulles, niveaux, transitions, plancher d'importance
│  ├─ ts_pilot         composant pilote (privé), échange, synchronisation, bilan,
│  │                   journal d'incarnation, validation des intentions
│  ├─ ts_view          construction des vues filtrées, actions possibles, vue de
│  │                   conception (fonction de développement)
│  ├─ ts_metrics       définition des indicateurs et de leurs plages cibles
│  ├─ ts_game          standard_rules() : assemble tous les greffons ; seul point d'entrée
│  │
│  │  ── générateurs ──
│  ├─ ts_citygen       générateur de ville (déterministe par graine)
│  ├─ ts_popgen        générateur de population (déterministe par graine)
│  ├─ ts_worldbuild    chaîne ville → population → rodage → validation → artefact
│  │
│  │  ── hôtes et services ──
│  ├─ ts_protocol      schémas protobuf, types générés Rust et C#
│  ├─ ts_persist       trait WorldStore, codec d'instantanés, journal, migrations ;
│  │                   moteurs : fichiers, SQLite, PostgreSQL
│  ├─ ts_host          hôte commun solo/serveur : boucle temps réel, sessions, pilotes
│  ├─ ts_net           transports : mémoire, QUIC (quinn)
│  ├─ ts_server        binaire du serveur de monde
│  ├─ ts_client_native bibliothèque native du client (C ABI)
│  ├─ ts_dialogue      service de dialogue (bibliothèque + binaire)
│  ├─ ts_accounts      service des comptes
│  ├─ ts_admin         API d'administration
│  ├─ ts_runner        lanceur ts-sim
│  ├─ ts_py            liaisons Python (pyo3)
│  └─ ts_viz           visualiseur de développement (optionnel)
├─ python/ts_gym/      environnements, entraînement, export, calibrage, rapports
├─ client/             projet Godot 4 C#
├─ admin-web/          console TypeScript
├─ data/               contenus, cartes, populations, modèles, scénarios
└─ docs/
```

*Pourquoi ce grain : un crate par domaine donne des frontières que le compilateur fait respecter, des temps de compilation incrémentale raisonnables malgré `bevy_ecs`, et des greffons qu'on peut désactiver un par un pour les arènes. Regrouper davantage mélangerait les domaines ; découper davantage multiplierait les interfaces sans bénéfice.*

### 5.2 Règles de dépendance (décidé)

```
ts_kernel ◄── ts_model ◄── ts_data, ts_world_model
                 ▲
     ┌───────────┼────────────── greffons de domaine (ts_body … ts_view)
     │           │               dépendent de ts_kernel, ts_model, ts_data,
     │           │               ts_world_model ; JAMAIS les uns des autres
     │        ts_game ◄── assemble tous les greffons
     │           ▲
     │   hôtes : ts_host, ts_server, ts_client_native, ts_runner, ts_py
     │
ts_citygen, ts_popgen ◄── ts_worldbuild (qui utilise aussi ts_game pour le rodage)
```

- Les domaines **communiquent par les données** (composants et ressources de `ts_model`) et par les **événements**, jamais par des appels directs. *Pourquoi : c'est le fonctionnement naturel d'un ECS, et cela supprime les cycles (R9).*
- Quand un domaine doit **demander** un calcul à un autre (l'IA veut le coût relatif d'un achat, [economie.md](../domaines/economie.md) §6.8), le service est déclaré dans `ts_model` comme **trait** et fourni par le domaine propriétaire à l'assemblage. Exemple : `trait PriceOracle { fn relative_cost(&self, who: StableId, price: Money) -> f32; }`.
- Les composants **privés** d'un domaine (états intermédiaires) restent dans son crate.
- `ts_popgen` est appelé à l'exécution (naissances, arrivées) à travers un trait `PersonFactory` de `ts_model`, implémenté par `ts_popgen` et branché par `ts_game`.

### 5.3 Les greffons (décidé)

`bevy_ecs` est utilisé **seul**, sans `bevy_app` (confirmation du socle) : la boucle, le pas de temps et l'ordonnanceur à événements sont à nous (§8). Un greffon minimal suffit :

```rust
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn disableable(&self) -> bool { false }    // seuls certains greffons peuvent être coupés
    fn build(&self, b: &mut SimBuilder);       // composants, ressources, systèmes, événements,
                                               // codecs d'instantané, indicateurs
}

pub fn standard_rules() -> Vec<Box<dyn Plugin>>;   // ts_game : la liste unique
```

- Greffons **désactivables** pour les arènes et les tests ciblés : `economy`, `social`, `population`, `justice`, `world.weather`. Tous les autres sont obligatoires. Désactiver un greffon ne change **pas** les règles des autres ; un greffon qui a besoin d'un autre le déclare, et la construction échoue si la dépendance est coupée. *Pourquoi : une arène de combat n'a pas besoin des loyers, mais un combat doit s'y résoudre exactement comme en ville.*
- La liste des greffons actifs et leur version font partie de l'**empreinte de règles** enregistrée dans chaque instantané et chaque manifeste de simulation.

### 5.4 Contenus en données (décidé)

- Format source : **RON**, un fichier par fiche (rôle, métier, recette HTN, acte social, catalogue d'activités, table de résolution, paramètres de domaine). *Pourquoi : RON exprime directement les énumérations et structures des types Rust (préconditions, effets), se lit bien et se valide contre ces types ; un seul format pour tout le contenu.*
- `ts-data check` valide les références croisées (une recette qui cite une action inconnue, un métier sans lieu possible) avec des messages en français ; `ts-data build` compile un **paquet de contenus** binaire et calcule son empreinte (SHA-256). Les mondes, simulations et modèles entraînés référencent cette empreinte.
- Les paramètres **[à calibrer]** des domaines sont tous dans le paquet, jamais en dur dans le code, et peuvent être **surchargés** par un scénario (§19.1).

---

## 6. Modèle de données du cœur

### 6.1 Identifiants (décidé)

- `StableId(u64)` : identité durable de toute entité qui peut être référencée (personnage, lieu, véhicule, objet de valeur, organisation, affaire, compte). Allouée par un compteur du monde, jamais réutilisée, sauvegardée. Toute référence durable (souvenir, relation, écriture, croyance, affaire) utilise `StableId`, jamais `Entity`. Une table `StableId → Entity` est reconstruite au chargement. *Pourquoi : R11.*
- `ViewHandle(u32)` : poignée **éphémère par session** donnée au client pour chaque entité perçue. Le serveur fait la correspondance. *Pourquoi : un identifiant stable envoyé au client permettrait à un client modifié de suivre une personne d'une session à l'autre, ou de la reconnaître déguisée, sans que le personnage la reconnaisse ; la reconnaissance doit passer par les connaissances du personnage ([interface.md](../domaines/interface.md) §1.1).*

### 6.2 Temps (décidé)

- `SimTime(i64)` : millisecondes depuis l'**époque du monde** (son jour de départ, [population.md](../domaines/population.md) §9.2). Monotone, jamais lu sur l'horloge murale dans le cœur.
- Le **calendrier local** (date, heure, jour de la semaine, saison) est une **vue** calculée à partir de `SimTime` et du fuseau du monde (`chrono-tz`). Les agendas sont écrits en heure locale et convertis en `SimTime` au moment de planifier. Au **changement d'heure**, une heure locale peut manquer ou se répéter : la conversion choisit la première occurrence, et un bloc d'agenda garde sa durée réelle. *Pourquoi : en ligne, le monde suit le calendrier réel du fuseau ([01-concept.md](../01-concept.md) §3) ; un temps interne monotone évite les sauts et doublons d'événements.*

### 6.3 Argent et quantités (décidé)

- `Money(i64)` en **centimes d'écu**. Aucune somme d'argent en flottant dans le cœur. Les barèmes en pourcentage produisent des montants arrondis par une règle unique (arrondi bancaire), et l'arrondi est lui-même une écriture si nécessaire. *Pourquoi : le grand livre en partie double doit s'équilibrer exactement (R12).*
- Positions et vitesses en `f32` (échelle réelle, 1 unité = 1 m, [ville.md](../domaines/ville.md) §2.2), attributs et besoins en `f32` ; les valeurs qui servent de **seuils persistants** (stocks, compteurs) en entiers.

### 6.4 Aléa (décidé)

Un **générateur à clé** (*counter-based*) plutôt qu'un état aléatoire par entité :

```rust
// tirage pur : même clé → même valeur, quel que soit l'ordre d'exécution
fn draw(world_seed: u64, stream: Stream, subject: StableId, slot: u64, purpose: u32) -> u64;
// Stream : un flux par domaine (Ai, Combat, Social, Economy, Population, Weather, ...)
// slot   : le tick, ou l'indice du bloc d'agenda, ou un compteur propre au sujet
```

- Fonction de mélange rapide et de bonne qualité (Philox ou un hachage de type `wyhash`), testée statistiquement.
- *Pourquoi : (1) le résultat ne dépend pas de l'ordre dans lequel les systèmes parallèles tirent leurs nombres, donc pas du nombre de fils ; (2) aucun état aléatoire à sauvegarder ; (3) des **nombres aléatoires communs** entre deux simulations comparées : changer un paramètre de l'économie ne décale pas les tirages du combat, ce qui réduit fortement la variance des comparaisons de calibrage (C3).*

### 6.5 Composants, ressources et lieux

- **Composants** par personnage : identité, âge, attributs, profil, chronotype, santé et blessures, besoins, émotions, agenda, plan courant, mémoire bornée, croyances, engagements, rôle, compte, inventaire, position (`Location`), niveau de détail, état « au repos », importance.
- `Location` (décidé) : `Physical { pos, cell, floor }` en simulation complète, `InPlace { place: StableId, activity, progress }` ou `InTransit { route, progress }` sinon. *Pourquoi : les niveaux simplifié et statistique raisonnent en lieux et en trajets ([ia-pnj.md](../domaines/ia-pnj.md) §6.1) ; la position exacte n'est reconstruite qu'à la montée en complet.*
- **Ressources globales** (en structures de tableaux, indexées par un indice dense) : `SocialGraph` (relations complètes et compactes, adjacence par personne), `Ledger` (écritures du jour, clôtures), `Scheduler` (événements datés), `SpatialGrid` (cellules de 64 m), `PlaceRegistry`, `CaseBook` (affaires), `Calendar`, `Weather`. *Pourquoi : les calculs de graphe (composante Réseau, propagation) et le grand livre sont des traitements en masse sur des tableaux contigus, bien plus efficaces qu'un composant par relation.*
- Les **marqueurs** de niveau de détail sont des composants vides (`LodComplete`, `LodSimplified`, `LodStatistical`) pour que chaque système ne parcoure que les archétypes qui le concernent (§10.2).

### 6.6 Le pilote (décidé)

```rust
// ts_pilot — le type et son contenu sont privés au crate
enum Pilot {
    Ai,
    Player(PlayerId),          // sémantique joueur : synchronisation, bilan, journal d'incarnation
    External(ControllerId),    // robots de test, agents Python, relectures : aucune sémantique joueur
}
// marqueurs publics, seuls visibles des autres domaines
struct AiDriven;               // l'IA produit les intentions
struct AiObserving;            // l'IA tourne en observation (pilote Player ou External)
```

- Les systèmes de règle ne voient que `AiDriven` ou `AiObserving`, jamais l'identité du pilote. Le type `Pilot` est privé à `ts_pilot`, qui n'expose son contenu qu'aux hôtes et à `ts_view` par une petite API d'accès ; les autres crates ne peuvent même pas le nommer. *Pourquoi : « le monde ne connaît pas les joueurs » devient une propriété vérifiée par le compilateur (R13).*
- `External` a deux modes : **avec observation** (l'IA calcule ce qu'elle aurait fait, utile pour comparer un agent à l'IA) ou **sans** (moins coûteux, pour l'entraînement).
- Les robots de charge qui doivent se comporter comme des joueurs (test du serveur en ligne) utilisent `Player` avec des comptes de test.

### 6.7 Intentions et validation

La **validation des intentions** est un passage obligé, dans `ts_pilot`, pour tous les pilotes, IA comprise (décidé) :

1. l'intention est-elle **possible** (préconditions, portée, état du corps) ? — même calcul que la liste des « actions possibles » envoyée au client ;
2. respecte-t-elle les **règles absolues** : aucun geste violent d'un adulte envers un mineur, aucune arme visant un mineur ([02-personnages.md](../02-personnages.md) §3.4), aucune intention de mise en danger mortelle de soi ([combat.md](../domaines/combat.md) §11.9) ;
3. pour un pilote `Player` : la **récusation** ([combat.md](../domaines/combat.md) §11.8) et les flux liés interdits ([economie.md](../domaines/economie.md) §12.3).

*Pourquoi : un seul filtre protège contre un joueur, une recette mal écrite et une compétence apprise qui aurait trouvé une faille ; il fournit aussi les **masques d'actions** de l'entraînement (§18.3).*

---

## 7. API du cœur

### 7.1 Construction et pilotage

```rust
pub struct WorldSpec { /* d'où vient le monde : artefact versionné, carte + population,
                          morceau de ville, monde synthétique, arène (§17.4) */ }
pub struct SimConfig {
    pub seed: u64,
    pub tick_hz: u32,                    // 20 par défaut (§8.2)
    pub threads: Threads,                // Single | Fixed(n) | Auto — sans effet sur le résultat
    pub disabled_plugins: Vec<&'static str>,
    pub overrides: ParamOverrides,       // surcharges de paramètres du paquet de contenus
    pub lod: LodPolicy,                  // Normal | ForceStatistical | ForceComplete(zone) …
    pub checks: Checks,                  // invariants coûteux activés (tests, développement)
}

impl Sim {
    pub fn build(spec: &WorldSpec, cfg: SimConfig, rules: &[Box<dyn Plugin>]) -> Result<Sim, BuildError>;

    // temps
    pub fn step(&mut self) -> StepReport;                          // un tick de 50 ms
    pub fn run_until(&mut self, stop: StopCondition) -> RunReport; // date, événement, prédicat
    pub fn advance(&mut self, req: AdvanceRequest) -> AdvanceOutcome; // avance du temps (§8.5)
    pub fn now(&self) -> SimTime;

    // entrées
    pub fn submit(&mut self, pilot: PilotKey, intent: Intent) -> Result<IntentTicket, Rejection>;
    pub fn submit_batch(&mut self, batch: &IntentBatch) -> BatchResult;
    pub fn command(&mut self, cmd: Command) -> CommandResult;

    // sorties
    pub fn view(&self, pilot: PilotKey, since: ViewCursor) -> ViewDelta;   // vue filtrée
    pub fn observe_batch(&self, req: &ObsRequest, out: &mut [f32]);       // tenseurs (§18.3)
    pub fn query(&self, pilot: PilotKey, q: Query) -> QueryResult;        // lecture seule
    pub fn drain_events(&mut self, sink: &mut dyn EventSink);
    pub fn metrics(&self) -> &MetricsFrame;

    // état
    pub fn snapshot(&self, scope: SnapshotScope) -> Snapshot;
    pub fn restore(snap: &Snapshot, cfg: SimConfig, rules: &[Box<dyn Plugin>]) -> Result<Sim, RestoreError>;
    pub fn state_hash(&self, scope: HashScope) -> StateHash;      // par famille de composants
}
```

### 7.2 Les cinq types d'échanges (décidé)

| Type | Sens | Qui | Dans la fiction ? | Journalisé ? |
|---|---|---|---|---|
| **Intention** (`Intent`) | entre | pilotes (joueur, IA, externe) | Oui : ce que le personnage veut faire | Oui (entrées externes) |
| **Commande** (`Command`) | entre | hôtes, console, scénarios, outils | Non : opération hors fiction | Oui, toujours |
| **Observation** (`ViewDelta`, tenseurs) | sort | client, agents, outils | Ce que le personnage perçoit et sait | Non |
| **Événement** (`SimEvent`) | sort | journal, client (filtré), mesures, récompenses, modération | Les deux, selon sa visibilité | Selon la visibilité |
| **Instantané** (`Snapshot`) | sort/entre | persistance, tests, réinitialisation | — | — |

**Intentions** : liste fermée, commune au joueur et à l'IA ; elle reprend les intentions de combat ([combat.md](../domaines/combat.md) §1.7), les actes sociaux ([social.md](../domaines/social.md) §3.1) et les actions du catalogue d'activités.

```rust
pub enum Intent {
    Move { dir: Vec2, pace: Pace },                    // déplacement continu
    GoTo { target: Target, mode: TravelMode },         // déplacement par itinéraire
    Interact { target: Target, action: ActionId },     // porte, objet, poste de travail…
    Combat(CombatIntent),                              // garde, frapper, parer, viser, tirer…
    Social { act: ActId, to: Option<Target>, subject: Option<SubjectRef>, tone: Tone },
    Drive(DriveIntent),
    UseItem { item: ItemRef, action: ActionId },
    Cancel,                                            // interrompre l'action ou le geste en cours
}
// Target côté client = ViewHandle ou point du monde ; côté cœur = StableId ou point
```

**Commandes** : `AttachPilot`, `DetachPilot { reason }`, `Reserve`, `Spawn(EntitySpec)`, `SetComponent` (scénarios et outils, **interdite** sur un serveur public), `Save`, `AdvanceTime`, `SetParam` (console, journalisée), `LedgerReversal { entries, approvals }` (double validation, [interface.md](../domaines/interface.md) §12.1), `KillSwitch { skill }` (§18.7).

**Événements** : `SimEvent { time, seq, kind, visibility }`, `seq` étant un compteur du monde (ordre total, reproductible). Visibilité :

- `Perceptible { at, radius, senses }` : peut être perçu (coup, cri, chute) ; la vue d'un joueur ne le contient que si le personnage le perçoit ;
- `Private(StableId)` : ne concerne que l'intérieur d'un personnage (souvenir formé, besoin franchissant un seuil, intuition) ;
- `OutOfFiction` : prise et reprise de pilote, synchronisation, bilan, éléments de modération ; jamais transmis à un système de règle (principe 7).

**Requêtes** (`Query`) : lectures sans effet pour l'interface — itinéraire selon les horaires **connus du personnage** ([interface.md](../domaines/interface.md) §7.2), fil de messages à formuler, carnet. Elles ne changent jamais l'état et n'entrent donc pas dans la reproductibilité.

### 7.3 Ce que l'API exclut (décidé)

- Aucun accès direct au `World` ECS depuis un hôte, sauf `ts_viz` et les tests, en lecture.
- Aucune commande d'écriture d'état (`SetComponent`, `Spawn` libre) sur un serveur public : elles n'existent que dans les constructions de développement et d'outils (option de compilation `dev-commands`). *Pourquoi : un scénario de test doit pouvoir placer un agresseur armé ; un serveur public, jamais.*

---

## 8. Boucle de simulation et pas de temps

### 8.1 Deux horloges, un seul temps (décidé)

| Horloge | Ce qu'elle fait avancer | Rythme |
|---|---|---|
| **Pas fixe** (`Tick`) | Tout ce qui est en simulation **complète** : intentions, mouvements, collisions, combat, perception, décisions étalées, compétences | 20 Hz (50 ms) **[à calibrer]** |
| **Ordonnanceur à événements** (`Scheduler`) | Fins de blocs d'agenda des niveaux simplifié et statistique, activités simplifiées, échéancier économique, jalons des affaires, naissances et décès, météo, tâches quotidiennes, minuteurs | Exactement à la date de chaque événement |

- Les deux horloges partagent `SimTime`. À chaque tick, l'ordonnanceur traite les événements dont la date est dans l'intervalle du tick ; entre deux ticks, rien d'autre ne se passe. Les égalités de date sont départagées par `seq` (ordre d'insertion), donc de façon reproductible.
- L'ordonnanceur est un **calendrier à roues** (*timing wheel*) à plusieurs niveaux (seconde, minute, heure, jour), avec un tas pour les dates lointaines. *Pourquoi : insertion et extraction en temps constant pour des centaines de milliers d'événements en attente.*
- **Saut de temps** : quand aucune entité n'est en simulation complète (simulation de calibrage, avance du temps en solo, monde sans joueur), la boucle ne tique plus ; elle **saute directement à l'événement suivant**. *Pourquoi : c'est ce qui rend une année de simulation statistique coûteuse en événements, pas en ticks (≈ 630 millions de ticks par an évités), d'où les performances du §20.2.*

### 8.2 Le tick de 50 ms (décidé)

*Pourquoi 20 Hz : les temps de réaction humains (≈ 150 à 300 ms) et la fenêtre de parade du combat ([combat.md](../domaines/combat.md) §1.2) sont résolus à 50 ms près ; c'est la fréquence usuelle des serveurs autoritaires de jeux d'action non compétitifs, et elle laisse un budget confortable (§20.1). Les intentions portent une date fine (en millisecondes) pour que le délai de réaction d'un geste soit exact à l'intérieur du tick.*

Rythmes dérivés, tous **étalés** sur les entités (un sous-ensemble défini par `StableId mod N` à chaque tick) :

| Traitement | Rythme | Étalement |
|---|---|---|
| Intentions, mouvement, collisions, résolution des gestes | 20 Hz | — |
| Compétences d'exécution (écrites ou apprises) | 10 Hz | 2 groupes |
| Perception (vision, ouïe) | 5 Hz | 4 groupes ; immédiate sur un événement fort (coup de feu) |
| Sélection d'objectif et planification | sur événement, et au moins 1 à 2 Hz | 10 à 20 groupes |
| Bulles et transitions de niveau | 1 Hz | budget de transitions par tick |
| Réplication vers les clients | 20 Hz pour le proche, 5 à 10 Hz pour le lointain | par joueur |

### 8.3 Hôte en ligne : rester à l'heure (décidé)

- L'hôte cale `SimTime` sur l'horloge murale du fuseau ; il exécute un tick toutes les 50 ms réelles.
- **Retard** : si un tick déborde, l'hôte exécute des ticks de rattrapage (au plus 3 d'affilée) ; si le retard persiste, il **réduit la charge** (rayon des bulles, fréquence de perception lointaine, §20.3) plutôt que de ralentir le temps. *Pourquoi : en ligne, une journée de jeu doit durer exactement 24 heures réelles ([01-concept.md](../01-concept.md) §2).*
- L'hôte lit les entrées réseau, les date au tick de leur réception, les soumet au cœur, et journalise ce qu'il a soumis (§12.3).

### 8.4 Hôte solo : temps réel et pause

Comme en ligne, sans rattrapage contraint (le solo peut ralentir légèrement sur une machine faible). Jeu fermé ou menu principal : la boucle s'arrête ([interface.md](../domaines/interface.md) §9.3).

### 8.5 Avance du temps en solo (décidé)

```
advance(jusqu'à T, interruptions):
    bulle du joueur → simplifiée ; reste → statistique (sauf plancher d'importance)
    tant que maintenant < T :
        e = prochain événement de l'ordonnanceur
        sauter à min(e.date, T) ; traiter les événements dus
        si un événement concerne le personnage du joueur et figure dans les
           déclencheurs d'interruption (ia-pnj §2.4) : arrêter, renvoyer la raison
    bulle du joueur → complète (reconstruction à partir de l'agenda, ia-pnj §6.3)
```

- L'avance est **découpée en tranches** (par exemple une heure de jeu) pour que l'interface fasse défiler l'horloge et le journal d'avance ([interface.md](../domaines/interface.md) §9.1), et pour qu'une sauvegarde puisse être faite entre deux tranches.
- Objectif : une nuit de 8 heures sautée en **moins d'une seconde** pour 10 000 habitants sur une machine de joueur (4 cœurs), un mois en moins de 15 secondes.

---

## 9. Ordonnancement des systèmes ECS

### 9.1 Les phases d'un tick (décidé)

Un `Schedule` `bevy_ecs` par phase, exécutées dans cet ordre strict ; à l'intérieur d'une phase, l'exécuteur parallèle de `bevy_ecs` répartit les systèmes sans conflit d'accès.

| # | Phase | Contenu |
|---|---|---|
| 1 | `Input` | Intentions et commandes reçues pour ce tick, triées par (date fine, `StableId`, `seq`) ; validation (§6.7) |
| 2 | `Scheduled` | Événements datés de l'intervalle : fins de blocs d'agenda, échéancier, jalons, minuteurs |
| 3 | `Perception` | Mise à jour de la grille spatiale ; ensembles perçus des groupes du tick |
| 4 | `Mind` | Réflexes, sélection d'objectif, HTN, GOAP (budget en nœuds), observation pendant l'incarnation |
| 5 | `Skills` | Compétences d'exécution → intentions des personnages `AiDriven` ; inférence par lots |
| 6 | `Act` | Exécution des intentions : mouvement, collisions, gestes de combat, actes sociaux, transactions |
| 7 | `Consequences` | Blessures, santé, besoins, émotions, mémoire, témoins, réputation locale, affaires |
| 8 | `World` | Décor, dégâts, météo, transports en commun, construction |
| 9 | `Lod` | Bulles, montées et descentes de niveau (1 Hz étalé) |
| 10 | `Output` | Événements finalisés, indices de changements pour les vues, mesures du tick |

Les intentions produites en phase 5 sont exécutées en phase 6 **du même tick** : l'IA et le joueur ont donc la même latence de traitement côté serveur.

### 9.2 Règles de parallélisme reproductible (décidé)

1. **Aucune ambiguïté d'ordre** : la détection d'ambiguïtés de `bevy_ecs` est réglée en **erreur** ; deux systèmes qui écrivent la même donnée dans une phase doivent être ordonnés explicitement.
2. **Itérations parallèles** (`par_iter`) seulement quand chaque entité n'écrit que ses propres composants ; l'aléa vient de `draw` (§6.4).
3. **Écritures croisées** (un coup touche un autre personnage, un achat touche un commerce) : elles passent par des **tampons de messages** collectés par fil, puis **triés** par (`StableId` source, `seq` local) avant application. *Pourquoi : l'ordre d'application ne dépend plus de l'ordre de fin des fils.*
4. **Réductions** (sommes, comptages, indicateurs) en entiers, ou en flottants **après tri** des termes.
5. **Interdits dans les crates de règles** (règle `clippy` `disallowed-types` et `disallowed-methods`) : `std::collections::HashMap`/`HashSet` (remplacés par `BTreeMap`, `IndexMap` ou des tableaux indexés, dont l'ordre d'itération est fixé), `Instant::now`, `SystemTime`, `rand::thread_rng`, variables d'environnement, E/S.

### 9.3 Calculs lourds différés à échéance fixe (décidé)

Les calculs quotidiens (composante Réseau, caches de réputation, propagation statistique des rumeurs, indices immobiliers) sont lancés à une date fixe sur une **copie** des données d'entrée, calculés sur un fil de fond, et **appliqués à une date fixe** plus tard (par exemple 4 h 00 → 4 h 30 du monde). Si le calcul n'est pas fini à l'échéance, la boucle l'attend. *Pourquoi : le calcul ne gêne pas le tick, et son résultat entre dans le monde à un moment qui ne dépend pas de la vitesse de la machine.* En saut de temps, l'attente est simplement le temps du calcul.

### 9.4 Budgets de calcul (décidé)

Tous les budgets du cœur sont exprimés en **unités de travail**, jamais en temps : nœuds explorés par GOAP (par recherche et par tick), recherches de chemin par tick, transitions de niveau par tick, formulations demandées par minute. *Pourquoi : R2 ; un budget en temps rend la simulation dépendante de la machine.* Les valeurs sont réglées par mesure pour tenir le budget en temps du §20.1 sur la machine de référence.

---

## 10. Niveaux de détail

La conception est fixée par [ville.md](../domaines/ville.md) §8.2 (où) et [ia-pnj.md](../domaines/ia-pnj.md) §6 (quoi). Cette section dit **comment** les mettre en œuvre.

### 10.1 Ce que fait chaque niveau, techniquement

| Niveau | Horloge | Position | Navigation | Décision | Économie | Social |
|---|---|---|---|---|---|---|
| **Complet** | Pas fixe | `Physical` | Maillage fin, collisions | Réflexes, utilité, HTN, GOAP borné, compétences | Une écriture par acte | Actes, formulation possible |
| **Simplifié** | Événements par activité | `InPlace` / `InTransit` | Graphe des rues et des pièces, durées | Utilité, HTN seul | Une écriture par activité | Conversations résolues par règle |
| **Statistique** | Événements par bloc d'agenda | `InPlace` / `InTransit` | Graphe des lieux, temps par mode | Bloc résolu en une fois | Écritures agrégées par bloc | Dérive par co-présence |

### 10.2 Mise en œuvre (décidé)

- Un composant `LodState { level, since, floor, pinned_until }` et un **marqueur vide** par niveau. Les systèmes filtrent par marqueur (`Query<…, With<LodComplete>>`) : un personnage statistique ne coûte rien aux systèmes du complet. Le changement d'archétype à chaque transition est peu coûteux à ce rythme.
- Les **bulles** sont recalculées à 1 Hz : ensemble des cellules de 64 m à moins de `r` d'un personnage piloté par un joueur, plus le bâtiment qu'il occupe ; la **zone chargée** (simplifié) est l'ensemble des secteurs des joueurs et de leurs voisins. Hystérésis : rayon de montée < rayon de descente, durée minimale à un niveau.
- **Transitions** traitées en phase `Lod` avec un budget par tick ; les montées vers le complet passent avant les descentes. Une montée **reconstruit** la position et l'activité à partir du bloc d'agenda en cours, avec un tirage `draw` (placement reproductible).
- Le **plancher d'importance** ([ia-pnj.md](../domaines/ia-pnj.md) §6.2) est un composant recalculé chaque nuit et à chaque événement qui rend un personnage important (affaire avec un joueur, incarnation récente).

### 10.3 Bulles adaptatives (décidé)

Le rayon de bulle `r` (≈ 150 m nominal) diminue quand le nombre de personnages en complet approche du **plafond du serveur** (§20.3), jusqu'à un minimum (≈ 60 m **[à calibrer]**) ; les joueurs dans les zones les plus denses sont réduits en premier. *Pourquoi : la charge du complet croît avec le nombre de joueurs ([ia-pnj.md](../domaines/ia-pnj.md) §6.4) ; un plafond explicite garantit le tick plutôt que de le laisser déborder.*

### 10.4 Une règle, trois résolutions (décidé)

Chaque domaine écrit ses règles sous la forme :

```rust
mod groceries {                         // exemple : faire les courses
    pub struct Params { /* lus dans le paquet de contenus */ }
    pub fn resolve_complete(...)    { /* acte par acte */ }
    pub fn resolve_simplified(...)  { /* une écriture à la fin de l'activité */ }
    pub fn resolve_statistical(...) { /* barème par bloc, mêmes prix, même choix de commerce */ }
    #[cfg(test)] mod equivalence { /* §10.6 */ }
}
```

### 10.5 Tables hors bulle calibrées sur le complet

Les tables du simplifié et du statistique (issue d'un affrontement, réussite d'un crime, taux d'événements par lieu, [combat.md](../domaines/combat.md) §13.1) sont des **données** du paquet, produites par la chaîne de calibrage (§19.4) à partir de milliers de simulations complètes, avec leur provenance (version des règles, graines, date).

### 10.6 Tests d'équivalence (décidé)

Pour chaque règle à trois résolutions, un test lance le **même scénario** à chaque niveau sur N graines et compare les distributions des sorties (argent dépensé, besoins satisfaits, relations, issues) par un test statistique avec tolérance. Ces tests tournent chaque nuit en intégration continue. *Pourquoi : R10 ; sans eux, l'équivalence promise par les domaines se perd à la première retouche.*

---

## 11. Déterminisme et reproductibilité

### 11.1 Le besoin

| Usage | Ce qu'il exige |
|---|---|
| Tests de non-régression, scénarios dorés | Même entrée → même résultat, à chaque exécution |
| Relecture d'un bug signalé (serveur ou solo) | Rejouer à partir d'un instantané et des entrées journalisées |
| Reprise après panne en ligne | Instantané + rejeu du journal jusqu'à l'instant de la panne (§12.4) |
| Calibrage, comparaison A/B | Deux variantes à graines égales ; aléa commun pour réduire la variance |
| Entraînement | Réinitialiser un épisode identique, évaluer deux modèles sur les mêmes situations |
| Bissection d'une divergence | Trouver le premier tick où deux exécutions diffèrent, et la famille de composants |

Le socle a écarté le **déterminisme strict** pour une bonne raison : un déterminisme multi-plateforme, exigé par un réseau en *lockstep*, imposerait l'arithmétique en virgule fixe et une physique maison, pour un bénéfice nul ici (le serveur est autoritaire et envoie l'état). Mais il ne faut pas jeter la reproductibilité avec : sans elle, aucun des usages du tableau n'est possible.

### 11.2 Les niveaux possibles

| Niveau | Garantie | Coût | Retenu |
|---|---|---|---|
| R0 | Aucune | Nul | Non |
| **R1** | **Même binaire, même plateforme, mêmes données, même graine, mêmes entrées journalisées → même état bit à bit, quel que soit le nombre de fils** | **Discipline de code (§9.2) ; ≈ 5 % d'effort de développement ; perte de performance négligeable** | **Oui** |
| R2 | R1 entre plateformes et compilateurs | Mathématiques transcendantes maison, pas d'instructions spécifiques, tests croisés | Visé sans garantie (§11.4) |
| R3 | Lockstep réseau | Virgule fixe, physique maison | Non |

**Décision** : **R1** (décidé). *Pourquoi : il rend possibles tous les usages du §11.1 pour un coût de discipline, sans arithmétique spéciale ; R2 et R3 coûteraient beaucoup pour des usages que le serveur autoritaire rend inutiles.* La formule du socle « pas de déterminisme strict » devient : **« pas de déterminisme multi-plateforme ni de lockstep ; reproductibilité par graine sur un même binaire »** (§24).

### 11.3 Ce qu'il faut pour R1

1. **Entrées closes** : le cœur ne lit rien d'autre que `SimConfig`, les données versionnées, l'instantané initial et les entrées datées (§3, principe 1).
2. **Aléa à clé** (§6.4) ; aucun autre générateur.
3. **Ordre total** : tri des entrées, tampons de messages triés, `seq` du monde, ordonnanceur départagé par `seq`, pas d'itération de tables de hachage (§9.2).
4. **Budgets en unités de travail** (§9.4) ; calculs de fond appliqués à échéance fixe (§9.3).
5. **Inférence reproductible** : même modèle, même composition de lots (triée par `StableId`), exécution `tract` sans parallélisme interne non reproductible (§18.6).
6. **Pas de moteur physique général dans le cœur** au départ : contrôleur cinématique sur le maillage de navigation, collisions par `parry`, modèle de véhicule cinématique (bicyclette), balistique par lancer de rayons. Si un jour un moteur de corps rigides est nécessaire, `rapier` avec l'option `enhanced-determinism` (décidé). *Pourquoi : la physique de gameplay d'une ville à pied et en voiture n'a pas besoin de corps rigides ; c'est aussi la source de non-reproductibilité la plus fréquente.*
7. **Données hors simulation exclues** : les textes du dialogue ne changent jamais l'état (§14.2).
8. **Empreintes d'état** par famille de composants (`state_hash`), calculées à intervalles réguliers en développement et dans les tests, pour localiser une divergence.

### 11.4 Mesures peu coûteuses vers R2

Fonctions transcendantes par le crate `libm` (implémentation Rust pure) plutôt que les fonctions de la plateforme ; pas de contraction automatique en FMA (comportement par défaut de Rust) ; cible x86-64 de base pour les serveurs. Un test compare chaque nuit les empreintes Linux et Windows ; une divergence est signalée mais ne bloque pas.

### 11.5 Où la reproductibilité s'arrête (assumé)

- En ligne, les **entrées** (intentions des joueurs, réponses de classement du texte libre, commandes de la console) arrivent du dehors : elles sont journalisées avec leur tick, et c'est ce journal qui rend la partie rejouable.
- Changer de binaire (mise à jour du serveur) casse le rejeu : on fait toujours un **instantané complet avant une mise à jour**, et l'on archive les binaires et paquets de contenus des versions déployées, pour pouvoir rejouer un bug avec la version où il est apparu.

---

## 12. Persistance et sauvegardes

### 12.1 Modèle (décidé)

Un monde enregistré = **en-tête** + **instantanés** + **journal**, plus des **projections** relationnelles pour ce qui doit être interrogé hors du jeu.

| Élément | Contenu | Format |
|---|---|---|
| En-tête | Identifiant du monde, graine, époque et fuseau, versions (carte, population initiale, paquet de contenus, modèles, règles, binaire) | Petit document |
| Instantané | État complet ou partiel : composants par **morceaux** (secteur × famille de composants), ressources globales (graphe social, grand livre du jour, ordonnanceur, affaires), compteurs (`StableId`, `seq`) | Binaire `serde` + `postcard`, enveloppe versionnée, compressé `zstd` |
| Journal | Entrées datées (intentions externes, commandes, résultats de classement), en ordre ; plus les événements utiles à la reprise | Ajout seul |
| Projections | Écritures du grand livre ([economie.md](../domaines/economie.md) §2.3), journaux d'incarnation, indicateurs quotidiens, événements hors fiction pour la modération | Tables relationnelles |

*Pourquoi : un schéma relationnel par composant lierait chaque évolution de règle à une migration de base, et la sauvegarde de 10 000 personnages par lignes serait lente ; l'instantané binaire versionné se charge vite, et les projections gardent les requêtes que la console et l'économie demandent vraiment.*

### 12.2 Moteurs de stockage (décidé)

Un trait unique `WorldStore` (écrire un morceau d'instantané, ajouter au journal, lire, transaction), trois moteurs :

| Moteur | Usage | Pourquoi |
|---|---|---|
| **PostgreSQL** | Serveurs en ligne, comptes, modération, console | Le choix du socle ; transactions, sauvegardes, réplication, requêtes de la console |
| **SQLite** (un fichier par monde, mode WAL) | Solo | Un fichier, transactions atomiques, résistant aux pannes, aucune installation |
| **Fichiers** (répertoire de run) | Simulations, entraînement, tests | Aucun service ; artefacts faciles à archiver et à comparer |

### 12.3 Instantanés incrémentaux

- La **détection de changements** de `bevy_ecs` marque les morceaux modifiés ; seul ce qui a changé est réécrit. Un instantané **complet** est refait périodiquement (compaction).
- La **copie** des morceaux modifiés est faite entre deux ticks (de l'ordre de quelques millisecondes) ; la sérialisation, la compression et l'écriture se font sur un fil de fond. *Pourquoi : la sauvegarde ne doit jamais geler le jeu.*
- Taille estimée d'un monde de 10 000 habitants : 250 à 400 Mo en mémoire, 60 à 120 Mo compressés **[à mesurer]** ; un instantané incrémental courant pèse quelques mégaoctets.

| Hôte | Journal | Incrémental | Complet |
|---|---|---|---|
| En ligne | écriture groupée chaque seconde | toutes les 5 min réelles | chaque nuit, et avant toute mise à jour |
| Solo | à chaque tranche d'avance et toutes les 30 s | toutes les 5 min de jeu ([interface.md](../domaines/interface.md) §9.2), fin d'incarnation, fermeture | à la fermeture ; les 3 dernières générations servent de **sauvegardes de secours** |
| Simulations | optionnel | à la demande (par exemple chaque année simulée) | en fin de run |

### 12.4 Reprise après panne (en ligne)

Dernier instantané + **rejeu du journal** jusqu'au dernier lot écrit. Avec R1, l'état retrouvé est exactement celui d'avant la panne, à une seconde près ; le rejeu va bien plus vite que le temps réel. *Pourquoi : c'est la promesse d'un monde persistant ; sans rejeu, on perdrait jusqu'à 5 minutes de vie de la ville à chaque panne.*

### 12.5 Versions et migrations (décidé)

- Chaque famille de composants a un **numéro de schéma** ; une montée de version fournit une fonction de migration Rust, testée sur un **corpus d'anciennes sauvegardes** archivées.
- Les règles de mise à jour de la carte et de la population initiale ([ville.md](../domaines/ville.md) §4.5 ; [population.md](../domaines/population.md) §9.3) sont des migrations de données : elles ne touchent que les parcelles non modifiées par le monde, jamais les habitants existants.

### 12.6 Données hors fiction

Le **journal d'incarnation** (pour la synchronisation, le bilan et la modération) est une projection des événements `OutOfFiction` et des événements de l'entité incarnée pendant l'incarnation. Il est stocké par l'hôte, hors du monde ; aucun système de règle ne peut le lire ([05-architecture.md](../05-architecture.md) §2.18).

---

## 13. Réseau et client

### 13.1 Protocole (décidé)

- **Schémas protobuf** dans `ts_protocol`, avec types générés pour Rust (`prost`) et C# (`Google.Protobuf`). *Pourquoi : outillage mûr dans les deux langages, règles d'évolution des schémas (numéros de champs) adaptées à un jeu qui vivra des années, un seul langage de description pour le client, les services internes et la console.*
- Le **même message** circule en solo (transport mémoire) et en ligne (QUIC) ([interface.md](../domaines/interface.md) §1.1).

| Sens | Messages |
|---|---|
| Client → serveur | `IntentMsg` (avec numéro de séquence client pour la réconciliation), `QueryMsg` (itinéraire, fil de messages, carnet), `UiAck`, `SessionMsg` |
| Serveur → client, non fiable (datagrammes QUIC) | `ViewDelta` : entités perçues, champs modifiés seulement, positions quantifiées |
| Serveur → client, fiable (flux QUIC) | `ViewEvent` (événements perçus, répliques, intuitions, interruptions, courrier), `IntentAck` (dernière intention traitée + état faisant foi du personnage), `MapPatch` (journal des modifications de la carte), `AccountMsg` |

### 13.2 Transport (décidé)

- **QUIC** par `quinn`, côté serveur et **côté client dans la bibliothèque native** ; flux fiables pour les événements, datagrammes pour les deltas de vue. *Pourquoi : R8 ; le C# n'a pas à dépendre de `System.Net.Quic`, et le client et le serveur partagent la même pile.*
- Chiffrement TLS 1.3 natif de QUIC ; authentification par jeton délivré par `ts_accounts`.

### 13.3 La bibliothèque native du client (décidé)

`ts_client_native` est une bibliothèque dynamique Rust appelée depuis C# par une petite interface C :

```c
ts_handle ts_open_solo(const uint8_t* config, size_t len);   // héberge ts_host + SQLite
ts_handle ts_open_online(const uint8_t* config, size_t len); // connexion QUIC
void      ts_send(ts_handle h, const uint8_t* msg, size_t len);
size_t    ts_poll(ts_handle h, uint8_t* buf, size_t cap);     // messages ts_protocol
void      ts_predict(ts_handle h, const uint8_t* input, size_t len, uint8_t* out, size_t cap);
void      ts_close(ts_handle h);
```

- En solo, le serveur tourne **sur un fil** de cette bibliothèque, dans le processus du jeu, comme le veut le socle ; le client ne voit que des messages. Un mode « serveur local en processus séparé » existe pour le développement (même binaire que `ts_server`, transport par boucle locale). *Pourquoi : un seul chemin de données ; le client C# ne sait pas s'il est en solo ou en ligne, sauf pour les écrans propres à chaque mode.*
- Le client Godot charge aussi les **données de la carte** (format de `ts_world_model`, morceaux par secteur) par cette bibliothèque, qui les lui rend sous forme de messages protobuf ; il applique les `MapPatch` reçus.

### 13.4 Prédiction et réconciliation (décidé)

- Le client prédit **le mouvement de son personnage** en appelant `ts_predict`, qui exécute le code de `ts_mobility` (cinématique et collisions locales) — **le même code que le serveur**. Il rejoue ses intentions non confirmées à chaque `IntentAck`. *Pourquoi : R7 ; aucune règle n'est réécrite en C#, et la prédiction ne diverge que par l'état des autres, que le serveur corrige.*
- Les gestes de combat ne sont **pas** prédits : leur effet vient du serveur ; seule l'animation de départ du geste est jouée immédiatement par le client.
- Les autres entités sont **interpolées** avec un retard de 100 ms **[à calibrer]**.
- **Compensation de latence limitée** ([combat.md](../domaines/combat.md) §1.2) : le serveur garde 250 ms d'historique des positions et résout un tir ou un coup dans le passé de l'attaquant, dans cette limite.

### 13.5 Gestion de l'intérêt

La vue d'un joueur est construite à partir de **l'ensemble perçu** du personnage (le même que celui de son IA, [ia-pnj.md](../domaines/ia-pnj.md) §0, principe 3) : c'est à la fois le filtre honnête et la gestion de l'intérêt réseau. Les cellules voisines servent de pré-filtre. Débit estimé : 50 à 150 entités perçues, deltas quantifiés, de l'ordre de 10 à 40 Ko/s par joueur au pire **[à mesurer]**.

### 13.6 Le client et le passage à la 3D (C1)

- Le protocole est **sémantique** : type d'entité, apparence décrite (tenue, corpulence, âge apparent, blessures visibles), activité et geste en cours (énumérations du cœur), posture, émotion visible, ton. Jamais un nom de ressource, d'animation ou de modèle. *Pourquoi : le passage à la 3D change les ressources et la caméra, pas le protocole ([interface.md](../domaines/interface.md) §13).*
- La règle « toute désignation produit une entité ou un point du monde » ([interface.md](../domaines/interface.md) §13.3) est portée par `Target` (§7.2).
- Le client C# est organisé en modules alimentés par la vue (HUD, sélection, carnet, conversation, caméra, plan, téléphone, comptes, réglages), conformément à [interface.md](../domaines/interface.md) §1.3.

### 13.7 Vue de conception

`ts_view::debug` produit des messages `DebugView` (rôles, paliers, besoins chiffrés, plans, relations, bulles) **uniquement** dans les constructions compilées avec l'option `dev-introspection`, que les serveurs publics n'ont pas (décidé). *Pourquoi : ce qui n'est pas compilé ne peut pas être activé par un client modifié ([interface.md](../domaines/interface.md) §12.3).*

### 13.8 Visualiseur sans Godot (décidé)

`ts_viz` enregistre, en option, l'état d'une simulation sans affichage ou d'une relecture vers **Rerun** (journal temporel 2D/3D, séries, filtrage), depuis Rust ou Python. *Pourquoi : pour l'entraînement et le calibrage (C2, C3), il faut regarder une simulation sans lancer le client ; Rerun le fait sans aucune dépendance du cœur (le visualiseur est un hôte).*

---

## 14. Service de dialogue

### 14.1 Langage et forme (décidé)

**Rust** (`ts_dialogue`, `axum` + `tokio`), sous deux formes : un **binaire** hébergé en ligne, partagé par les serveurs de monde ; une **bibliothèque** intégrée à l'hôte solo. *Pourquoi : le service partage avec le cœur les types des faits structurés (nécessaires au filtre de sortie), il est surtout de l'orchestration asynchrone de requêtes (où Rust excelle), et cela évite un troisième langage côté serveur. Le socle laissait « Rust ou C# » ouvert.*

### 14.2 Le texte est cosmétique pour la simulation (décidé)

- Le **cœur décide** de l'acte, du sujet, du ton, et de la **durée** de la réplique (selon l'acte et le registre) ; il émet un événement `Speech` reproductible.
- L'**hôte** décide qui doit lire un texte (joueurs à portée), envoie la demande de formulation avec sa priorité ([social.md](../domaines/social.md) §3.5), et transmet la réponse aux clients concernés.
- Le cœur **n'attend jamais** le texte. Si la formulation n'est pas arrivée à temps, le client affiche le **repli systémique** à partir du gabarit et de ses paramètres, qu'il traduit lui-même ([interface.md](../domaines/interface.md) §1.2). Le geste de réflexion pendant l'attente est un effet du client.
- *Pourquoi : R6 ; la simulation reste reproductible et indifférente aux pannes, à la latence et au modèle utilisé.*

### 14.3 Le classement du texte libre

Le texte tapé par le joueur va du client à l'hôte, puis au service, qui le **classe** en acte, sujet et ton ; la carte de vérification est affichée ([interface.md](../domaines/interface.md) §5.4) ; c'est la **confirmation du joueur** qui produit l'intention `Social`, journalisée comme toute entrée. Entre deux joueurs, le texte est transmis tel quel après le filtre de modération ([social.md](../domaines/social.md) §9.2).

### 14.4 Interface du service

```rust
struct FormulateRequest {
    act: ActId, subject: Option<SubjectFacts>, tone: Tone,
    speaker: SpeakerSummary,           // âge, milieu, registre, tempérament, émotion
    relation_words: RelationWords,     // « méfiant, le respecte »
    facts: Vec<Fact>,                  // faits autorisés ; savoir opaque = seulement son existence
    recent_lines: Vec<LineRef>,
    languages: Vec<Lang>,              // une formulation par langue d'auditeur
    priority: Priority, quota_key: AccountId, deadline_ms: u32,
}
struct FormulateResponse { lines: Vec<(Lang, String)>, used_facts: Vec<FactId> }
```

- **Fournisseurs** derrière un trait `Formulator` : modèle hébergé par une API (fournisseur interchangeable ; un petit modèle rapide de la famille Claude, comme `claude-haiku-5-5`, est un bon candidat pour la latence de 1 à 2 s et le coût), modèle **local** en solo par un processus annexe `llama.cpp` (`llama-server`, API compatible) lancé par l'hôte solo si la machine le permet. *Pourquoi le processus annexe : isoler une bibliothèque C++ lourde du processus du jeu, et en changer sans recompiler.*
- **Filtre de sortie** : toute réplique qui cite un nom propre, un nombre, un lieu ou un fait absent des `facts` fournis est rejetée ([social.md](../domaines/social.md) §3.4) ; puis la **modération** de contenu ([combat.md](../domaines/combat.md) §16.8).
- **Cache** par (acte, ton, registre, tranche de relation, langue) pour les actes fréquents et peu chargés ; **quotas** par compte et par heure ; **file des formulations signalées** pour corriger consignes et filtres.

---

## 15. Comptes, pilotes et modération

### 15.1 Comptes hors du monde (décidé)

`ts_accounts` (Rust, PostgreSQL) porte ce qui appartient au **joueur** et non au monde : compte, niveau de compte et niveau solo, expérience, profil comportemental, bilans, familiarités (clé : monde × personnage), amis mutuels, messagerie hors jeu, note de conduite, sanctions ([04-joueur.md](../04-joueur.md) ; [social.md](../domaines/social.md) §9). En solo, les mêmes données sont locales (SQLite), séparées des données en ligne.

### 15.2 Prise et reprise de pilote

Le module `ts_pilot` de l'hôte vérifie les **conditions d'accès** ([04-joueur.md](../04-joueur.md) §4) en combinant des lectures privilégiées du cœur (palier, catégories, état « au repos ») et les données du compte, puis envoie la commande `AttachPilot`. À la fin d'une incarnation, il calcule le bilan à partir du journal d'incarnation et l'envoie à `ts_accounts`. Le cœur ne reçoit jamais de donnée de compte.

### 15.3 Modération (décidé)

- **Filtres de texte** (contenu, données personnelles) : dans `ts_dialogue` pour les formulations, et dans l'hôte pour le texte entre joueurs.
- **Capture d'un signalement** : l'hôte garde, par compte connecté, un **tampon circulaire** des dernières minutes (événements hors fiction, actions, paroles, positions à 2 Hz) ; le signalement en fige une copie, qui alimente la chronologie et la relecture sur le plan de la console ([interface.md](../domaines/interface.md) §12.1).
- **Détections** (acharnement, chasse aux nouveaux venus, abus du monde partagé, flux économiques entre comptes) : des **tâches d'analyse** dans un processus de modération qui lit les projections hors fiction dans PostgreSQL ; elles produisent des alertes pour la console, jamais d'effet dans le monde. *Pourquoi : ce sont des règles sur les comptes, pas sur les personnages ; elles n'ont rien à faire dans le cœur, et les placer dehors garde le principe « le monde ne connaît pas les joueurs ».*

---

## 16. Console d'administration et tableaux de bord

### 16.1 Langage (décidé)

- **Interface web en TypeScript**, avec React et Vite, client d'API **généré** depuis la description OpenAPI de `ts_admin`. *Pourquoi : c'est l'écosystème le plus fourni pour des tableaux, des graphiques et des formulaires d'administration ; le typage généré garde la console alignée sur l'API.*
- **API d'administration en Rust** (`ts_admin`, `axum`) : authentification (OpenID Connect), rôles (modérateur, administrateur, concepteur), **journal de chaque consultation et de chaque action** en ajout seul, circuit de **double validation** (annulation d'écritures du grand livre). Elle parle aux serveurs de monde par une API interne non exposée sur Internet. *Pourquoi : le serveur de monde reste léger et n'est jamais exposé ; l'authentification et le journal sont au même endroit.*

### 16.2 Indicateurs : une seule définition (décidé)

- `ts_metrics` définit chaque indicateur (formule, unité, plage cible tirée du paquet de contenus) : économie ([economie.md](../domaines/economie.md) §10.3), population ([population.md](../domaines/population.md) §7.6), criminalité ([combat.md](../domaines/combat.md) §10.9), social, IA (satisfaction des besoins), charge (personnages par niveau, durée des phases du tick), dialogue (répliques, cache, quotas).
- Les **régulateurs** du monde (économique, démographique) lisent ces mêmes indicateurs dans le cœur ; la console et les rapports de simulation en affichent les séries. *Pourquoi : le tableau de bord doit montrer exactement ce que le régulateur voit, et les simulations de calibrage doivent mesurer ce que la production mesurera ([interface.md](../domaines/interface.md) §12.2).*
- Stockage : séries quotidiennes dans PostgreSQL (en ligne) et en Parquet (simulations) ; mesures techniques du serveur (durée du tick, mémoire) dans Prometheus avec des tableaux Grafana réservés à l'exploitation.
- La console a une section **Simulations** qui affiche les runs importés (`ts-sim import`) avec les mêmes graphiques que les mondes en ligne.

---

## 17. Générateurs de ville et de population, construction d'un monde

### 17.1 Générateur de ville (`ts_citygen`)

- Déterministe par graine et paramètres ; étapes de [ville.md](../domaines/ville.md) §4.2 ; sortie : `CityModel` de `ts_world_model`.
- **Exports** depuis le même modèle : données serveur (registre des lieux, graphes des lieux, des rues et des pièces, maillage de navigation par cellule, géométrie de collision) et données client (morceaux par secteur avec références aux kits modulaires).
- **Maillage de navigation dérivé directement de la géométrie** du générateur (rues, trottoirs, pièces, portes sont déjà des polygones), sans voxelisation (décidé). *Pourquoi : plus rapide, exact, et recalculable localement quand un bâtiment est construit ou démoli en cours de partie.*
- Utilisé **en cours de partie** pour les constructions ([ville.md](../domaines/ville.md) §6.2) : appelé par `ts_world`, avec un tirage `draw`, donc reproductible.
- **Éditeur Godot** : un greffon d'éditeur C# appelle le générateur par la bibliothèque native, et exporte les lieux faits main au format de `ts_world_model` ([ville.md](../domaines/ville.md) §4.3).

### 17.2 Générateur de population (`ts_popgen`)

- Étapes de [population.md](../domaines/population.md) §1.2 ; validation du §1.3 sous forme de **contrôles automatiques** qui produisent un rapport.
- Fonctions d'exécution exposées par le trait `PersonFactory` (naissances, ménages arrivants), qui tirent leur aléa du flux `Population` du monde.

### 17.3 La chaîne de construction d'un monde (`ts-worldbuild`)

```
ts-worldbuild --map orvenne@1.0 --pop-params data/population/orvenne.ron --seed 7 \
              --rodage 60d --out worlds/orvenne-initial@1.0.tsw
   1. carte : version publiée (ou génération + retouches)
   2. population : génération par graine, validation
   3. rodage : 60 jours en statistique avec ts_game::standard_rules()
   4. validation de stabilité ; recalage des dates sur le jour de départ
   5. artefact : instantané + en-tête + rapport de validation, adressé par son empreinte
```

Créer un monde (serveur ou partie solo) = charger l'artefact, convertir les dates relatives au calendrier du monde ([population.md](../domaines/population.md) §9.2), et tirer une **graine propre au monde** pour que deux mondes divergent.

### 17.4 Mondes de toutes tailles (C3) (décidé)

`WorldSpec` couvre :

| Variante | Contenu | Usage |
|---|---|---|
| `Artifact(id)` | Monde initial publié | Jeu, calibrage final |
| `Sectors { map, sectors, pop_seed }` | Mode « morceau de ville » ([population.md](../domaines/population.md) §1.5) | Prototype, tests de quartier |
| `Synthetic { city_params, pop_scale, seed }` | Ville **générée** sans retouche, de quelques secteurs à plusieurs communes, population à l'échelle (de 500 à 30 000 habitants) | Tests de charge, calibrage sur des villes différentes, plafond de calcul |
| `Arena { layout, actors }` | Petit décor (bar, rue, parking, segment de route) sans population | Entraînement des compétences, tests de combat |
| `Snapshot(path)` | Un instantané quelconque | Relecture, reprise d'une situation |

---

## 18. Chaîne d'entraînement

### 18.1 Ce qui est entraîné (rappel)

Seules les **compétences d'exécution** : combat (mains nues, arme blanche, tir, maîtrise d'une personne), fuite, poursuite à pied et en véhicule, conduite, déplacement en foule ([ia-pnj.md](../domaines/ia-pnj.md) §3.5 ; [combat.md](../domaines/combat.md) §13.2). Jamais la sélection d'objectif ni la planification.

### 18.2 Le trait `Skill` (décidé)

```rust
pub trait Skill {
    fn id(&self) -> SkillId;
    fn obs_spec(&self) -> &ObsSpec;                  // version et empreinte du format d'observation
    fn act_spec(&self) -> &ActSpec;                  // intentions permises à cette compétence
    fn decide_batch(&self, obs: &[f32], mask: &[u8], out: &mut [ActionOut]);
}
// ScriptedSkill : écrite à la main (référence et repli)
// LearnedSkill  : modèle ONNX chargé, inférence tract
```

- La planification fournit un **but de compétence** (`SkillGoal` : neutraliser X, fuir vers la sortie Y, garder la distance, suivre Z, conduire jusqu'à W) ; la compétence ne choisit que **comment** l'atteindre.
- Une compétence ne produit que des **intentions** de sa liste, validées comme les autres (§6.7) : une compétence apprise ne peut rien faire qu'un joueur ne puisse faire.

### 18.3 Observations et actions (décidé)

- L'encodage `encode(ensemble perçu, état propre, but) → [f32; N]` est **une seule fonction**, dans `ts_skills`, utilisée à l'entraînement et en jeu. *Pourquoi : la première cause d'échec d'un modèle exporté est un encodage d'observation différent entre l'entraînement et l'exécution.*
- Contenu type : état propre (attributs normalisés, fatigue, stress, douleur, ivresse, blessures, arme), but (cible en coordonnées relatives, mode), les K entités perçues les plus proches (position et vitesse relatives, posture, arme visible, croyance d'hostilité), rayons de dégagement autour du personnage (obstacles du maillage). **Uniquement ce que le personnage perçoit** ([ia-pnj.md](../domaines/ia-pnj.md) §0, principe 3).
- Actions : direction et allure de déplacement, intention de geste (liste fermée), choix de cible parmi les K ; **masque** des actions invalides fourni par la validation (§6.7).
- Les **attributs et l'état** sont dans l'observation : un même modèle sert au vétéran reposé et au passant terrifié, et les limites physiques (temps de réaction, coordination) s'appliquent à l'exécution des intentions comme pour un joueur ([ia-pnj.md](../domaines/ia-pnj.md) §0, principe 4).

### 18.4 Python : liaisons et environnements (décidé)

- `ts_py` : liaisons **`pyo3`**, paquet construit par **`maturin`**, tableaux **numpy sans copie** (crate `numpy`), GIL relâché pendant les pas. Python 3.12 ou plus.
- `ts_gym` fournit :
  - `SkillVecEnv` : API **vectorisée** Gymnasium (`gymnasium.vector.VectorEnv`), N mondes-arènes dans un processus, pas en parallèle sur plusieurs fils Rust (un monde par fil, sans parallélisme interne, donc trivialement reproductible) ;
  - `SkillParallelEnv` : API **PettingZoo** `ParallelEnv` pour le multi-agent (affrontements, poursuites, foule) ;
  - `World` : accès général (charger un scénario, avancer, lire les événements et les mesures) pour les analyses et le calibrage.

```python
from ts_gym import SkillVecEnv

env = SkillVecEnv("data/scenarios/arenes/rue_nuit_1v1.ron", skill="combat.mains_nues",
                  num_worlds=512, threads=16, seed=0, randomize="attributs")
obs, info = env.reset()                    # obs : [num_agents, obs_dim] float32
for _ in range(10_000):
    actions = policy(obs, info["action_mask"])
    obs, reward_terms, terminated, truncated, info = env.step(actions)
    reward = reward_terms @ weights        # termes calculés en Rust, pondérés en Python
```

- **Réinitialisation rapide** : une arène est reconstruite depuis sa description compilée et une graine en moins d'une milliseconde ; pour des situations plus grandes, restauration d'un instantané en mémoire (`Sim::restore`). Les mondes terminés sont réinitialisés automatiquement dans le lot.
- **Récompenses** : le cœur calcule un vecteur de **termes** standard à partir des événements (dégâts infligés et subis, distance au but, contact perdu, collisions, chute, épuisement, sortie atteinte) ; Python les pondère et peut lire les événements bruts. *Pourquoi : la vitesse du calcul en Rust, la souplesse du réglage en Python.*
- **Mêmes règles** : l'environnement ne fait que (1) construire un monde par `standard_rules()` avec des greffons désactivables coupés, (2) attacher des pilotes `External`, (3) soumettre des intentions, (4) lire observations et événements. Les commandes de mise en place n'existent que dans la description du scénario (§19.1).

### 18.5 Entraînement (décidé)

- **PyTorch**, avec une implémentation PPO propre au projet, dérivée de CleanRL (courte, lisible, modifiable). *Pourquoi : des politiques petites et des environnements maison n'ont pas besoin d'un cadre lourd ; une implémentation lisible est plus facile à corriger et à faire évoluer.*
- **Auto-affrontement** (*self-play*) avec une ligue d'anciennes versions pour le combat ; **programme progressif** (*curriculum*) ; **randomisation** des attributs, de l'état, du décor et de l'éclairage.
- Politiques : perceptron multicouche (2 à 3 couches de 128 à 256), éventuellement une couche récurrente GRU pour la poursuite et la conduite.
- **Débit visé** : une arène de 2 à 8 personnages en simulation complète coûte de l'ordre de 10 à 50 µs par tick, soit plus de 20 000 ticks par seconde et par cœur (plus de 1 000 fois le temps réel) — objectif à mesurer.

### 18.6 Export, vérification et inférence (décidé)

1. **Export ONNX** (jeu d'opérateurs figé, limité aux couches denses, activations, normalisation, GRU).
2. **`ts-model check`** : charge le modèle dans `tract`, compare ses sorties à celles de PyTorch sur 10 000 observations de test (tolérance fixée), vérifie les empreintes `obs_spec` et `act_spec`, mesure le coût d'inférence.
3. **Inférence dans le cœur** par **`tract`** (Rust pur, sans dépendance native), **par lots** : en phase `Skills`, tous les personnages qui utilisent la même compétence sont regroupés (ordre trié par `StableId`) en un seul appel. *Pourquoi tract plutôt qu'ONNX Runtime : aucune bibliothèque native à livrer, pas de fils internes incontrôlés, reproductibilité plus simple ; les modèles visés sont petits, et tract suffit largement.*
4. Coût visé : moins de 5 µs par personnage et par décision ; quelques dizaines de personnages en combat à 10 Hz = négligeable.

### 18.7 Versionnage et déploiement des modèles (décidé)

- **Registre** dans `data/models/<compétence>/<version>/` : `model.onnx`, `manifest.ron` (empreintes des formats, version des règles et du paquet de contenus d'entraînement, scénarios et graines, métriques d'évaluation, commit, date), adressé par empreinte. Les gros fichiers sont gérés par Git LFS.
- **Promotion** : un modèle n'entre dans le paquet de contenus qu'après la **suite d'évaluation** : taux de succès contre la compétence écrite et contre la version précédente sur des scénarios et graines fixés ; **plausibilité** (pas de tremblement, pas d'oscillation, pas d'exploitation d'une faille des règles) ; **monotonie** selon les attributs (plus de compétence de combat → meilleurs résultats) ; coût d'inférence ; simulations de calibrage de la criminalité sans dérive.
- Le paquet de contenus référence la version de chaque compétence ; l'en-tête d'un monde l'enregistre (la relecture exige le même modèle).
- **Repli** : la compétence écrite reste toujours disponible ; si un modèle manque, ne se charge pas ou a un format incompatible, le cœur bascule sur elle et le signale. La console peut couper un modèle à chaud (`KillSwitch`), avec effet au tick suivant, journalisé.

---

## 19. Chaîne de simulation et de calibrage

### 19.1 Les scénarios (décidé)

Un scénario est une **donnée RON** : monde (§17.4), règles (greffons désactivés, surcharges de paramètres), graines, acteurs et mise en place, **chronologie** d'actions datées, conditions d'arrêt, mesures à enregistrer. Pas de langage de script embarqué dans le cœur pour l'instant (décidé). *Pourquoi : un format déclaratif garde la reproductibilité et la lisibilité ; l'orchestration complexe (balayages, optimisation) se fait en Python, autour du cœur, pas dedans.*

```ron
Scenario(
    name: "ville_5_ans_reference",
    world: Artifact("orvenne-initial@1.0"),
    seeds: Range(1, 32),
    lod: ForceStatistical,
    duration: Years(5),
    overrides: { "economie.regulateur.actif": true },
    timeline: [
        At(Days(400), CloseBusiness("usine_principale")),   // choc économique scripté
    ],
    record: [
        Metrics(set: "calibrage_standard", every: Days(1)),
        Events(kinds: ["Crime", "Death", "Bankruptcy", "Departure"], sample: 1.0),
        Snapshots(every: Years(1)),
    ],
)
```

```ron
Scenario(
    name: "rue_nuit_1v1",
    world: Arena(layout: "arenes/rue_eclairage_faible"),
    rules: Standard(disable: ["economy", "social", "population"]),
    actors: [
        Actor(id: "a", template: "adulte", randomize: Attributes(spread: 0.3), pilot: External("a")),
        Actor(id: "b", template: "adulte", randomize: Attributes(spread: 0.3), pilot: External("b")),
    ],
    setup: [ Hostility("a", "b", 0.8), SkillGoal("a", Neutralize("b")), SkillGoal("b", Neutralize("a")) ],
    end: Any([Incapacitated, Fled, Timeout(Seconds(90))]),
)
```

### 19.2 Le lanceur `ts-sim` (décidé)

| Commande | Rôle |
|---|---|
| `ts-sim run <scénario> [--seeds] [--jobs]` | Exécute, un monde par cœur, sans affichage ; écrit un répertoire de run |
| `ts-sim sweep <balayage>` | Balayage de paramètres (grille ou hypercube latin) ; l'optimisation (Optuna) se pilote depuis Python |
| `ts-sim compare <runA> <runB>` | Compare les distributions des indicateurs entre deux variantes (intervalles sur les graines, aléa commun) |
| `ts-sim report <run>` | Rapport HTML : séries avec plages cibles, alertes, distributions, journal des régulateurs |
| `ts-sim replay <instantané> <journal>` | Rejoue une partie ou un bug, éventuellement vers `ts-viz` |
| `ts-sim bisect <runA> <runB>` | Trouve le premier tick et la famille de composants où deux exécutions divergent |
| `ts-sim import <run>` | Charge un run dans la base de la console (§16.2) |

Répertoire de run : `manifest.ron` (versions du binaire, des règles, du paquet de contenus, des modèles, scénario, graines, surcharges, machine, durée), `metrics.parquet`, `events.parquet`, `snapshots/`, `report.html`, `log.txt`.

### 19.3 Les sorties mesurées

Indicateurs de `ts_metrics` (§16.2), par jour simulé et par graine, avec leur plage cible ; événements échantillonnés ; profils de performance (durée des phases, personnages par niveau). L'analyse fine se fait en Python (`polars`, graphiques), à partir des mêmes fichiers Parquet.

### 19.4 Les boucles de calibrage (décidé)

1. **Stabilité de long terme** : simulations statistiques de 5 à 10 ans sur 16 à 32 graines ; objectif : tous les indicateurs dans leur plage ([economie.md](../domaines/economie.md) §10.3 ; [population.md](../domaines/population.md) §7.6 ; [combat.md](../domaines/combat.md) §10.9). Les paramètres des régulateurs et des barèmes sont réglés par optimisation bayésienne (Optuna), la fonction objectif étant la distance aux plages cibles.
2. **Tables hors bulle** : milliers d'affrontements, de crimes et d'activités en **simulation complète** dans des arènes et des morceaux de ville, attributs randomisés ; ajustement des tables du simplifié et du statistique (régression, tables par tranches) ; écriture dans le paquet de contenus avec provenance ; vérification par les tests d'équivalence (§10.6).
3. **Chocs scénarisés** : fermeture de l'usine, canicule, vague de cambriolages, afflux de personnages persistants riches ; on vérifie que le monde réagit de façon plausible et revient à l'équilibre.
4. **Comparaisons de versions** : avant chaque mise en production d'un changement de règles ou d'un modèle, `ts-sim compare` entre l'ancienne et la nouvelle version sur le scénario de référence.

---

## 20. Performances et montée en charge

### 20.1 Budget par tick (objectifs)

Serveur de référence : 8 cœurs récents, un monde de 10 000 habitants, 64 joueurs connectés, ≈ 3 000 personnages en complet au plus.

| Phase | Budget (temps écoulé, parallélisme compris) |
|---|---|
| `Input` (réception, validation) | 1 ms |
| `Scheduled` (événements du simplifié et du statistique) | 3 ms |
| `Perception` (1/4 des personnages en complet par tick) | 5 ms |
| `Mind` (décisions étalées, GOAP borné) | 5 ms |
| `Skills` (dont inférence) | 2 ms |
| `Act` (mouvement, collisions, combat, transactions) | 5 ms |
| `Consequences` + `World` | 3 ms |
| `Lod` | 1 ms |
| `Output` + construction des vues | 5 ms |
| **Total** | **≤ 30 ms sur 50 ms** (marge de 40 % pour les pointes) |

Ordre de grandeur unitaire visé : ≈ 10 µs par personnage en complet et par tick, toutes phases comprises, avant parallélisme ; quelques µs par résolution de bloc statistique.

### 20.2 Débit sans affichage (objectifs)

| Mode | Objectif |
|---|---|
| Statistique, 10 000 habitants, 8 cœurs | **1 an simulé en moins de 5 minutes** (≈ 100 000 fois le temps réel) ; rodage de 60 jours en moins de 1 minute |
| Avance du temps en solo, 4 cœurs | Une nuit de 8 heures en moins de 1 seconde ; un mois en moins de 15 secondes |
| Prototype de quartier (quelques centaines d'habitants, une bulle) | Au moins 50 fois le temps réel |
| Arène d'entraînement (2 à 8 personnages) | Plus de 20 000 ticks par seconde et par cœur |
| Campagne de calibrage | 32 graines × 10 ans en une nuit sur une machine de 32 cœurs |

*Base du calcul : ≈ 60 000 blocs d'agenda par jour à quelques µs, plus les tâches quotidiennes (composante Réseau : une vingtaine d'itérations sur 1,5 million d'arêtes, de l'ordre de 50 à 100 ms en Rust) ; ≈ 0,3 à 0,5 s par jour simulé sur un cœur, divisé par le parallélisme.*

### 20.3 Montée en charge (décidé)

- **Un monde = un processus autoritaire**, parallélisé en interne par l'exécuteur de `bevy_ecs` et des itérations parallèles. Pas de répartition d'un monde entre plusieurs serveurs (décidé). *Pourquoi : R4 ; la charge est bornée (10 000 habitants, un plafond de joueurs), les données globales sont fortement couplées, et un seul écrivain simplifie le grand livre, la reproductibilité et la persistance.*
- **Plafond de joueurs par monde** : 64 en première version, objectif 150 **[à calibrer par tests de charge]**. Au-delà, on ouvre **un autre monde** sur le même fuseau. *Pourquoi : dans une ville de 10 000 habitants sans figurant, des centaines de joueurs simultanés changeraient la nature du monde avant de poser un problème de calcul.*
- **Plafond du complet** ≈ 3 000 personnages **[à calibrer]**, tenu par les bulles adaptatives (§10.3) et la baisse de fréquence de perception du lointain.
- **Plafond de population** ([population.md](../domaines/population.md) §7.6) : vérifié par un monde synthétique de 30 000 habitants, qui doit tenir le budget du statistique.
- **Si un jour un seul processus ne suffit plus**, dans cet ordre : (1) déporter la diffusion réseau vers des relais (le serveur de monde reste seul autoritaire) ; (2) déporter la construction des vues ; (3) seulement ensuite, envisager une répartition par groupes de secteurs, qui exigerait une révision de cette architecture. Les **secteurs** restent l'unité de chargement, de sauvegarde, de niveau de détail et de parallélisme, ce qui garde cette porte ouverte.

### 20.4 Mémoire (estimation)

| Poste | Estimation |
|---|---|
| Personnages (composants hors mémoire et relations) | ≈ 50 Mo |
| Souvenirs (10 000 × 200 × ≈ 64 octets) | ≈ 130 Mo |
| Relations (graphe social) | ≈ 100 Mo |
| Ville (registre, graphes, maillages, collisions) | 100 à 300 Mo |
| Grand livre du jour, ordonnanceur, affaires, divers | < 50 Mo |
| **Total** | **≈ 0,5 à 1 Go par monde** |

### 20.5 Points chauds à surveiller

- **Co-présence** en statistique dans les grands lieux (l'usine, l'hypermarché) : appliquer la dérive aux seules relations existantes plus K rencontres tirées, jamais à toutes les paires.
- **Perception** dans les foules : grille de 64 m subdivisée localement, limitation du nombre d'entités suivies par personnage.
- **Recherche de chemin** : hiérarchique (lieux → rues et pièces → maillage local), chemins partagés vers une même destination, cache par fiche de rôle.
- **GOAP** : bornes en nœuds, cache des plans réussis.
- **Réplication** : quantification, deltas, fréquence selon la distance.

### 20.6 Outillage de mesure

`tracing` pour les journaux et les durées de phase, `tracing-tracy` pour le profilage en développement, indicateurs de charge dans `ts_metrics`, et un **banc de performance** en intégration continue (scénarios fixes, régression signalée au-delà de 10 %).

---

## 21. Tests et qualité

| Famille | Ce qu'elle vérifie | Quand |
|---|---|---|
| Tests unitaires de règle | Formules, recettes, barèmes | À chaque modification |
| **Scénarios dorés** | Un scénario + une graine → empreinte d'état et événements clés attendus | À chaque modification |
| **Déterminisme** | Deux exécutions identiques ; 1 fil contre N fils ; même empreinte | À chaque modification |
| **Aller-retour d'instantané** | Instantané puis restauration puis suite = exécution sans interruption | À chaque modification |
| **Rejeu** | Instantané + journal = état final enregistré | À chaque modification |
| **Invariants** | Grand livre équilibré, somme de l'argent constante hors flux extérieurs, aucun mineur sans adulte responsable, règles absolues jamais violées, aucun `StableId` orphelin | En continu dans les tests, et en option sur les serveurs de développement |
| **Équivalence entre niveaux** | Distributions comparables complet / simplifié / statistique | Chaque nuit |
| **Stabilité longue** | 5 ans statistiques sur plusieurs graines dans les plages | Chaque nuit (scénario réduit), chaque semaine (complet) |
| **Performance** | Banc fixe, régression > 10 % | Chaque nuit |
| **Migrations** | Corpus d'anciennes sauvegardes chargées et rejouées | À chaque changement de schéma |
| **Charge en ligne** | Robots `Player` scriptés, N joueurs, tick tenu | Avant chaque phase en ligne |
| **Modèles** | Suite d'évaluation des compétences apprises | À chaque promotion |

Intégration continue sous Windows et Linux ; `clippy` avec les interdictions du §9.2 ; formatage automatique.

---

## 22. Plan de mise en œuvre

L'ordre du périmètre initial ([05-architecture.md](../05-architecture.md) §5) est conservé, avec une **phase 0** ajoutée et quelques éléments avancés.

### Phase 0 — Fondations (à construire en premier)

*Pourquoi d'abord : ce sont les éléments qu'on ne peut pas ajouter après coup sans tout reprendre (reproductibilité, temps, ordonnanceur, instantanés, API) ; ils sont petits et rendent tout le reste testable.*

1. Espace de travail Cargo, intégration continue Windows et Linux, règles `clippy`.
2. `ts_kernel` : `SimTime`, `StableId`, `Money`, aléa à clé, ordonnanceur à roues, phases, greffons, `SimBuilder`, empreinte d'état, trait d'instantané.
3. `ts_model` et `ts_data` minimaux (chargement RON, validation, paquet avec empreinte).
4. `ts_runner` minimal : exécuter un scénario sans affichage, écrire `metrics.parquet` et le manifeste ; **test de déterminisme** (deux exécutions, 1 fil contre N fils).
5. `ts_py` minimal : charger un scénario, avancer, lire des positions dans un tableau numpy. *Pourquoi si tôt : une journée de travail qui fixe la forme de l'API par lots avant qu'elle ne se fige autour du seul serveur.*
6. `ts_viz` (Rerun) minimal.

**Terminé quand** : un monde-jouet de 1 000 entités qui suivent un agenda tourne un an simulé en saut de temps, deux exécutions donnent la même empreinte, l'aller-retour d'instantané est vert, et Python lit l'état.

### Phase 1 — Prototype du cœur (socle §5.1)

1. `ts_world_model` et une première carte du morceau de **faubourg Saint-Roch, Pont-Vieux et bord de la Cité** ([ville.md](../domaines/ville.md) §11.1) : `ts_citygen` réduit aux étapes nécessaires, ou carte saisie à la main dans le format définitif ; graphes et maillage de navigation dérivé.
2. `ts_popgen` en mode « morceau de ville » (quelques dizaines à quelques centaines d'habitants).
3. `ts_body`, `ts_mobility`, `ts_perception`, `ts_ai` (besoins, agenda, utilité, HTN avec quelques recettes), puissance et paliers.
4. **Niveaux complet et statistique** dès cette phase, avec l'ordonnanceur et le premier test d'équivalence (besoins et agenda) ; le simplifié vient en phase 5. *Pourquoi : le rodage, l'avance du temps et les simulations longues en dépendent, et c'est la forme du code de chaque règle (§10.4) qui doit être juste dès la première règle.*
5. `ts_economy` minimal : comptes, grand livre entier, salaires et loyers par l'échéancier.
6. `ts_combat` écrit à la main (mains nues), première **arène** et `SkillVecEnv` avec la compétence écrite (aucun entraînement encore : on valide la chaîne).
7. `ts_persist` fichiers et SQLite : instantané, journal, aller-retour.

**Terminé quand** : le quartier vit 30 jours de jeu en statistique en moins de 10 s, une bulle de 60 personnages tient à 20 Hz sous 2 ms par tick, les tests de déterminisme et d'aller-retour sont verts.

### Phase 2 — Échange de pilote (socle §5.2)

`ts_pilot` : pilote privé, `AttachPilot` et `DetachPilot`, état « au repos », réservation courte, observation de l'IA, reprise dans le même tick, synchronisation, éjection, journal d'incarnation, bilan ; validation des intentions avec les règles absolues. Tout est testé **sans client**, par des robots `External` et `Player` scriptés dans des scénarios.

### Phase 3 — Client Godot 2.5D en solo (socle §5.3)

`ts_protocol`, `ts_view` (vue filtrée, actions possibles, poignées éphémères), `ts_host`, `ts_client_native` (transport mémoire, prédiction), projet Godot C# (caméra orthographique, kits simples, HUD minimal, saisie des intentions), sauvegarde automatique SQLite, avance du temps avec interruptions, vue de conception de développement.

### Phase 4 — En ligne (socle §5.4)

`ts_server`, `ts_net` (QUIC), `ts_accounts` minimal, moteur PostgreSQL de `ts_persist`, deux joueurs dans le quartier, reprise après panne par rejeu, robots de charge, `ts_admin` et une console squelette (authentification, rôles, journal, inspection d'entité en développement).

### Phase 5 — Extension (socle §5.5)

Dans cet ordre, chaque étape s'appuyant sur la précédente :

1. Ville complète : `ts_citygen` complet, éditeur Godot, `ts_popgen` complet, `ts-worldbuild` avec rodage et validation.
2. Niveau **simplifié**, bulles adaptatives, plancher d'importance, tests d'équivalence complets.
3. Domaines complets : économie, social, justice, population, monde (météo, constructions).
4. Chaîne de **calibrage** : scénarios de référence, `sweep`, `compare`, `report`, import dans la console ; première campagne de stabilité sur 5 ans.
5. **Service de dialogue** : repli systémique (déjà côté client), puis service hébergé, puis modèle local en solo ; classement du texte libre.
6. **Modération** et tableau de bord complet.
7. **Entraînement** des compétences, dans l'ordre fuite → poursuite à pied → combat à mains nues → conduite → foule → armes, chaque modèle passant la suite d'évaluation avant promotion.
8. Durcissement des performances (plafonds de joueurs, banc de charge), 3D.

---

## 23. Récapitulatif des décisions

| # | Décision (décidé) | Raison | § |
|---|---|---|---|
| D1 | Le cœur est une bibliothèque sans E/S ni horloge murale ; les hôtes l'entourent | Condition commune de C1, C2, C3 | 3 |
| D2 | Un seul jeu de règles, `ts_game::standard_rules()`, utilisé par tous les hôtes ; seuls certains greffons sont désactivables | « Mêmes règles que le jeu » garanti par la structure | 5.3 |
| D3 | Domaines en crates séparés, communiquant par `ts_model` et les événements | Pas de cycles, compilation incrémentale, greffons désactivables | 5.2 |
| D4 | Contenus en RON compilés en paquet versionné par empreinte | Un format lisible, validé contre les types ; reproductibilité | 5.4 |
| D5 | `StableId` pour toute référence durable ; `ViewHandle` éphémère pour le client | Sauvegardes et relectures sûres ; pas de suivi d'identité par un client modifié | 6.1 |
| D6 | Temps simulé entier monotone, calendrier local calculé | Changements d'heure, reproductibilité | 6.2 |
| D7 | Argent en centimes entiers | Partie double exacte | 6.3 |
| D8 | Aléa à clé par flux de domaine | Indépendant du parallélisme, rien à sauvegarder, aléa commun pour les comparaisons | 6.4 |
| D9 | Pilote `Ai` / `Player` / `External`, type privé, marqueurs publics | Outils et entraînement sans sémantique joueur ; le monde ne connaît pas les joueurs par construction | 6.6 |
| D10 | Validation unique des intentions pour tous les pilotes, règles absolues comprises | Même protection contre joueur, recette fautive et modèle appris ; masques d'actions | 6.7 |
| D11 | API en cinq échanges : intentions, commandes, observations, événements, instantanés | Même interface pour client, Python, tests, console | 7 |
| D12 | Pas fixe de 20 Hz pour le complet + ordonnanceur à événements pour le reste, avec saut de temps | Accélération et simulations de plusieurs années | 8 |
| D13 | Hôte en ligne : rattrapage limité puis réduction de charge, jamais de ralentissement du temps | Une journée de jeu = 24 h réelles | 8.3 |
| D14 | Dix phases ordonnées par tick ; parallélisme sans ambiguïté, tampons triés, interdits `clippy` | Reproductibilité indépendante du nombre de fils | 9 |
| D15 | Calculs quotidiens lourds sur un fil de fond, appliqués à échéance fixe | Pas de gel du tick, résultat reproductible | 9.3 |
| D16 | Budgets en unités de travail, jamais en temps | Indépendance vis-à-vis de la machine | 9.4 |
| D17 | Marqueurs de niveau, bulles à 1 Hz avec hystérésis, bulles adaptatives sous plafond | Coût borné du complet | 10 |
| D18 | Une règle = trois résolutions dans le même module + test d'équivalence | Tenir « mêmes règles, résolution plus grossière » dans la durée | 10.4 |
| D19 | **Reproductibilité R1** (même binaire, même plateforme), pas de lockstep ni de déterminisme multi-plateforme | Tests, relecture, reprise, calibrage, entraînement, pour un coût de discipline | 11 |
| D20 | Pas de moteur de corps rigides dans le cœur au départ (cinématique + `parry`) ; `rapier` déterministe si besoin un jour | Suffisant pour le gameplay ; source majeure de non-reproductibilité évitée | 11.3 |
| D21 | Persistance = en-tête + instantanés incrémentaux + journal + projections relationnelles | Chargement rapide, reprise par rejeu, requêtes utiles conservées | 12.1 |
| D22 | PostgreSQL en ligne, SQLite en solo, fichiers pour les simulations | Pas de serveur de base chez le joueur ; simulations sans service | 12.2 |
| D23 | Protobuf pour tous les échanges entre langages et processus | Outillage Rust et C#, évolution des schémas | 13.1 |
| D24 | QUIC (`quinn`) dans la bibliothèque native du client | Pile commune, pas de dépendance à `System.Net.Quic` | 13.2 |
| D25 | Bibliothèque native du client : solo dans le processus, réseau en ligne, prédiction par le code du serveur | Aucune règle en C#, un seul chemin de données | 13.3 |
| D26 | Protocole sémantique, jamais de ressource graphique | Passage à la 3D sans toucher au protocole | 13.6 |
| D27 | Vue de conception compilée seulement en développement | Impossible à activer sur un serveur public | 13.7 |
| D28 | Visualiseur de développement par Rerun | Voir une simulation sans Godot | 13.8 |
| D29 | Service de dialogue en Rust ; modèle local en solo par processus annexe `llama.cpp` | Types partagés avec le cœur ; isolation du code C++ | 14.1 |
| D30 | Le texte formulé est cosmétique : le cœur n'attend jamais le service | Reproductibilité, robustesse aux pannes | 14.2 |
| D31 | Comptes et méta-jeu dans `ts_accounts`, hors du monde | Séparation fiction / joueur | 15.1 |
| D32 | Détections de modération hors du cœur, sur les projections hors fiction | Règles de comptes, pas de personnages | 15.3 |
| D33 | Console en TypeScript (React) sur une API Rust | Écosystème d'administration ; API typée générée | 16.1 |
| D34 | Indicateurs définis une fois (`ts_metrics`) pour régulateurs, console et rapports | Le tableau de bord montre ce que le régulateur voit | 16.2 |
| D35 | Maillage de navigation dérivé de la géométrie du générateur | Exact, rapide, recalculable en cours de partie | 17.1 |
| D36 | Chaîne `ts-worldbuild` produisant un artefact de monde adressé par empreinte | Population initiale versionnée et reproductible | 17.3 |
| D37 | `WorldSpec` : artefact, morceau de ville, ville synthétique, arène, instantané | Mondes de toutes tailles (C3) | 17.4 |
| D38 | `pyo3` + `maturin`, environnements Gymnasium vectorisés et PettingZoo | Standard de l'apprentissage par renforcement, numpy sans copie | 18.4 |
| D39 | Encodage des observations unique, partagé entraînement / jeu | Première cause d'échec des modèles exportés évitée | 18.3 |
| D40 | PyTorch + PPO maison dérivé de CleanRL | Simple, lisible, suffisant pour de petites politiques | 18.5 |
| D41 | Export ONNX, vérification de parité, inférence par lots avec `tract` | Rust pur, reproductible, aucune dépendance native | 18.6 |
| D42 | Registre de modèles versionnés, suite d'évaluation, repli écrit, coupure à chaud | Qualité contrôlée, aucun risque de blocage | 18.7 |
| D43 | Scénarios déclaratifs en RON, pas de langage de script embarqué ; orchestration en Python | Reproductibilité et lisibilité | 19.1 |
| D44 | Lanceur `ts-sim` (run, sweep, compare, report, replay, bisect, import), sorties Parquet | Chaîne de simulation complète (C3) | 19.2 |
| D45 | Un monde = un processus ; pas de répartition d'un monde entre serveurs ; plus de mondes au-delà du plafond de joueurs | Charge bornée, données couplées, simplicité | 20.3 |
| D46 | Phase 0 de fondations ; complet + statistique dès le prototype | On ne greffe pas la reproductibilité ni les niveaux de détail après coup | 22 |

---

## 24. Modifications proposées au socle

À appliquer par la session principale si elle les retient.

### 24.1 [05-architecture.md](../05-architecture.md)

| § | Texte actuel (résumé) | Proposition |
|---|---|---|
| §1 | Schéma client / serveur / base / dialogue | Remplacer par le schéma du §4.1 : bibliothèque native du client, serveur de monde avec cœur en bibliothèque, services `ts_dialogue`, `ts_accounts`, `ts_admin`, outils (`ts-sim`, `ts-worldbuild`, Python) |
| §2.1 | Cœur sans affichage | Ajouter : le cœur est une **bibliothèque sans E/S ni horloge murale** ; serveur, solo, Python, lanceur et tests sont des **hôtes** ; les règles sont assemblées par une fonction unique utilisée par tous les hôtes (D1, D2) |
| §2.2 | ECS `bevy_ecs` seul | Ajouter : sans `bevy_app` ; boucle et ordonnanceur propres ; domaines en greffons séparés communiquant par des données partagées et des événements (D3) |
| §2.3 | Pilote `Joueur(id)` ou `IA` | Ajouter un pilote `Externe` pour les outils, les tests et l'entraînement, sans sémantique joueur ; le type pilote est privé et les règles ne voient que « piloté par l'IA » ou « IA en observation » (D9) ; validation unique des intentions, règles absolues comprises (D10) |
| §2.4 | Même serveur, dans le même processus en solo | Préciser : le client parle au cœur par une **bibliothèque native Rust** qui héberge le serveur en solo, porte QUIC en ligne et fournit la prédiction de mouvement (D24, D25) |
| §2.5 | Serveur autoritaire, prédiction par le client | Préciser : la prédiction utilise le **code de mouvement du serveur** livré dans la bibliothèque native ; poignées d'entités éphémères par session ; protocole sémantique (D5, D25, D26) |
| §2.6 | Trois niveaux de détail | Ajouter : **pas fixe de 20 Hz** pour le complet, **ordonnanceur à événements** pour le simplifié, le statistique et l'échéancier, avec **saut de temps** quand rien n'est en complet ; bulles adaptatives sous un plafond ; une règle = trois résolutions dans un même module, avec test d'équivalence (D12, D17, D18) |
| §2.7 | Zones, puis un serveur par zone si nécessaire | Remplacer par : **un monde = un processus autoritaire**, parallélisé en interne ; pas de répartition d'un monde entre serveurs ; au-delà du plafond de joueurs, un autre monde sur le même fuseau ; les secteurs restent l'unité de chargement, de sauvegarde, de niveau de détail et de parallélisme (D45) |
| §2.8 | Persistance PostgreSQL, journal des actions | Remplacer par : **instantanés incrémentaux + journal des entrées + projections relationnelles** ; **PostgreSQL en ligne, SQLite en solo**, fichiers pour les simulations ; reprise après panne par rejeu ; le journal d'incarnation est une projection hors du monde (D21, D22) |
| §2.9 | Service de dialogue | Ajouter : en **Rust** ; le texte formulé est **cosmétique** pour la simulation, le cœur n'attend jamais le service ; modèle local en solo par processus annexe (D29, D30) |
| §2.10 | Pas de déterminisme strict | Remplacer par : **reproductibilité par graine sur un même binaire et une même plateforme**, indépendante du nombre de fils ; ni lockstep ni déterminisme multi-plateforme ; temps entier, argent en centimes entiers, aléa à clé, budgets en unités de travail ; pas de moteur de corps rigides dans le cœur au départ (D6, D7, D8, D16, D19, D20) |
| §2.13 | IA, compétences entraînables | Ajouter : chaîne d'entraînement — `pyo3`, environnements Gymnasium et PettingZoo vectorisés, export ONNX, inférence par lots `tract`, registre de modèles versionnés, compétence écrite en repli (D38 à D42) |
| §2.14 | Générateur de population | Ajouter : chaîne `ts-worldbuild` (ville → population → rodage → validation → artefact adressé par empreinte) ; mondes synthétiques et arènes pour les tests et l'entraînement (D36, D37) |
| §2.16 | Modération | Préciser : les détections tournent hors du cœur, sur les données hors fiction (D32) |
| §2.20 | Outils hors jeu | Ajouter : console en **TypeScript** (React) sur une API d'administration Rust ; indicateurs définis une seule fois pour régulateurs, console et rapports ; lanceur de simulations `ts-sim` ; visualiseur de développement (D33, D34, D44, D28) |
| Nouveau §2.21 | — | **API du cœur** : intentions, commandes, observations, événements, instantanés (D11) |
| Nouveau §2.22 | — | **Simulations et calibrage** : scénarios déclaratifs, lanceur, sorties Parquet, boucles de calibrage (D43, D44) |
| §4 | Tableau des langages | Service de dialogue : **Rust** (au lieu de « Rust ou C# [à décider] ») ; console d'administration : **TypeScript** ; contenus : **RON** ; Python : entraînement, calibrage, analyses ; protocole : **protobuf** ; bibliothèque native du client : Rust |
| §5 | Périmètre initial en 5 étapes | Ajouter une **étape 0 « Fondations »** (§22) ; à l'étape 1, ajouter les niveaux **complet et statistique** avec l'ordonnanceur, la persistance fichiers/SQLite, une arène et l'environnement Python avec la compétence écrite ; à l'étape 5, ordonner l'extension comme au §22 |

### 24.2 [CLAUDE.md](../../../CLAUDE.md), section « Architecture »

Remplacer ou compléter les puces par :

- Cœur de simulation en **Rust**, organisé en **ECS** (`bevy_ecs` seul), **bibliothèque sans affichage ni E/S** ; une seule fonction assemble les règles, utilisée par tous les hôtes (jeu, serveur, Python, lanceur de simulations, tests).
- **Reproductible par graine** sur un même binaire : temps entier, argent en centimes, aléa à clé, aucun parallélisme ni budget qui change le résultat.
- **Serveur autoritaire en Rust**, un processus par monde ; en solo, il tourne dans le processus du client, hébergé par la bibliothèque native du client.
- **Client Godot 4 en C#**, relié au cœur par une bibliothèque native Rust (transport, prédiction) ; aucune règle de jeu dans le client.
- **Le pilote d'un personnage est un composant** (`Joueur(id)`, `IA` ou `Externe` pour les outils) ; tous produisent les mêmes intentions, validées par le même filtre.
- Persistance **PostgreSQL en ligne, SQLite en solo** (instantanés et journal) ; dialogue par un service Rust séparé ; console d'administration en TypeScript.
- Entraînement des compétences d'exécution en **Python** sur le même cœur (`pyo3`, Gymnasium/PettingZoo), modèles ONNX exécutés dans le cœur.

---

## 25. Risques résiduels

| Risque | Parade |
|---|---|
| La discipline de reproductibilité s'érode avec le temps | Test de déterminisme bloquant à chaque modification ; interdits `clippy` ; `ts-sim bisect` pour localiser vite |
| `bevy_ecs` change d'API à chaque version | Version épinglée ; montée de version dans une modification isolée, validée par toute la suite de tests |
| Les tables hors bulle et les tests d'équivalence coûtent du temps de calcul en intégration continue | Scénarios réduits chaque nuit, complets chaque semaine ; machine dédiée |
| Les compétences apprises trouvent des failles des règles | Validation unique des intentions, critères de plausibilité, coupure à chaud, repli écrit |
| Coût et latence du dialogue hébergé | Quotas, cache, priorités, repli systémique toujours prêt ; le cœur n'en dépend pas |
| Migrations de sauvegardes sur des années de monde en ligne | Numéros de schéma par famille, corpus d'anciennes sauvegardes, instantané complet avant chaque mise à jour |
| Plafond de joueurs atteint plus tôt que prévu | Bulles adaptatives, relais réseau, ouverture d'un autre monde ; tests de charge avant chaque phase en ligne |
| Volume du grand livre en ligne (de l'ordre de 10⁷ à 10⁸ écritures par an et par monde) | Tables partitionnées par mois, archivage des années closes, agrégats pour la console |
| Les performances estimées ici sont des objectifs non mesurés | Banc de performance dès la phase 0 ; les chiffres du §20 sont à remplacer par des mesures à la fin de la phase 1 |
