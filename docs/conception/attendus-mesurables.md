# Attendus mesurables

> Fichier tenu par le responsable conception. Créé le 30/10/2026, à la demande de l'orchestrateur ([../pilotage/organisation.md](../pilotage/organisation.md), §3.1).
> Il rassemble les **valeurs cibles** écrites dans la conception, pour que le responsable qualité et essais les suive. La source fait foi : en cas d'écart, c'est le fichier cité qui a raison, et ce tableau doit être corrigé.
> Les valeurs marquées **[à mesurer]** ou **[à calibrer]** dans leur source sont des objectifs de départ. Un objectif manqué remonte au responsable conception comme un `ecart` ; s'il touche à la technique, il va au responsable architecture et sécurité.
> Règle 6 du [README](README.md) : les lignes ne sont jamais renumérotées. Une nouvelle ligne prend le numéro suivant, et une ligne retirée garde son numéro avec la mention « supprimée ».

## 1. Simulation et performances

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M1 | Reproductibilité par graine : un même binaire et une même plateforme donnent la même empreinte, sur 1 fil comme sur N fils | Identique au bit | [05](05-architecture.md) §2.10 | Test de déterminisme (`cargo xtask check`) | 0 |
| M2 | Monde-jouet de 1 000 entités, un an simulé en saut de temps | Même empreinte à chaque exécution | 05 §5, phase 0 | Critère de fin de phase 0 | 0 |
| M3 | Un an simulé pour 10 000 habitants | Moins de 5 min **[à mesurer]** | 05 §2.22 | `ts-sim`, chronométré | 1 |
| M4 | Coût de l'esprit des PNJ en statistique | ≈ 500 opérations par personnage et par jour **[à mesurer]** | 05 §2.13 ; [ia-pnj](domaines/ia-pnj.md) §13.15 | Compteurs d'unités de travail | 1 |
| M5 | Pas de la simulation complète | 20 Hz, sans ralentir le temps en ligne | 05 §2.22 | Indicateurs du serveur | 1 |
| M6 | Équivalence entre niveaux de détail (même règle, mêmes paramètres) | Test d'équivalence vert pour chaque règle, tenue comprise | 05 §2.2, §2.6 | Tests automatiques | 1 |

## 2. Mémoire

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M7 | Mémoire d'un monde de 10 000 habitants | 0,55 à 0,85 Go | Clôture de l'architecture, S-1 | Assertions de taille des composants à la compilation, puis mesure | 0 |
| M8 | Composants de l'esprit | ≈ 2 Ko par personnage, ≈ 20 Mo en tout **[à mesurer]** | ia-pnj §13 | Idem | 1 |
| M9 | Génome d'apparence ; tenue portée ; dates par relation | ≈ 64 o ; ≈ 30 o ; 4 × 2 o, soit ≈ 52 Mo (`SimDay` signé sur 16 bits, ±89 ans autour du jour de départ, saturé au-delà) | 05 §2.2, §2.5 ; clôture de l'architecture, T2-6 ; [README](README.md), A12 | Idem | 1 |

## 3. Génération

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M10 | Création d'un monde solo | ≤ 10 s sur 4 cœurs, plafond 20 s, dont ≤ 5 s de rodage propre **[à mesurer]** | 05 §2.14 ; [population](domaines/population.md) §1.6 | Chronométrage `ts-worldbuild` | 1 |
| M11 | Chaîne complète d'une famille | ≤ 2 min sur 8 cœurs, rodage de famille ≤ 60 s **[à mesurer]** | 05 §2.14 ; [revues/generation.md](revues/generation.md) | Idem | 5 |
| M12 | Mémoire de la génération | ≤ 4 Go | population §9.4 | Mesure | 5 |
| M13 | Fidélité de la population aux tables cibles | Écart aux marges ≤ 3 % ; ≥ 80 % de chaque rôle dans sa fourchette de palier | population §1.3 | Validateurs de `ts-worldbuild` | 1 |
| M14 | Réseau social de départ | Plages cibles du [social](domaines/social.md) §10.2, dont ≥ 98 % des adultes et ≥ 90 % des liens au-dessus de « connaissance » | social §10.2 ; population §5.3 | Validateur du réseau, puis `ts-sim` au fil des années | 1 |
| M15 | Robustesse des plans directeurs | 95 % de réussite sur 32 × 32 graines | [ville](domaines/ville.md) §4.6 (P37) | Campagne de graines | 5 |

## 4. Indistinguabilité et esprit des PNJ

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M16 | Tests à l'aveugle « joueur ou PNJ » | ≤ 60 % de bonnes réponses, **par profil d'adversaire** (six profils) | 05, en-tête (A8, clôture) ; ia-pnj §14.3 | Sessions évaluatrices face à des sessions joueuses | 3 |
| M17 | Discriminateur hors ligne | AUC ≤ 0,70 **[à calibrer]** ; en production, mesurée chaque mois sur les traces étiquetées par le serveur | ia-pnj §14.3 | Discriminateur | 3 |
| M18 | Test « fidèle ou non » (les signatures face au jugement) | AUC ≥ 0,85 | ia-pnj §14 ; [revues/signatures.md](revues/signatures.md) | Sessions évaluatrices | 5 |
| M19 | Qualité du panel de juges | Un juge doit placer ≥ 90 % des ancres ; test-retest à ordre inversé ≥ 90 % ; accord mesuré entre consignes et modèles différents | 05, en-tête ; ia-pnj §13.13, §14.3 | Contrôles du panel | 3 |
| M20 | Aucune différence de protocole selon le pilote (message, champ, cadence, délai) | Test automatique vert | 05 §2.5 | Test de vue | 3 |

## 5. Signatures comportementales

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M21 | Part des actes écrits à la main | Annotations 10 à 15 %, dérogations ≤ 5 % (alerte de la CI à 5 %) | [04](04-joueur.md) §2 ; [revues/signatures-contenu.md](revues/signatures-contenu.md) §5 | Rapport `ts-data signatures` | 1 |
| M22 | Ancres, échelles, planchers | Les 46 ancres dans leur bande pour les 12 archétypes ; échelles monotones ; planchers qui donnent bien la rupture | signatures-contenu | `ts-data check` et invariants `INV-SIG-*` | 1 |
| M23 | Accord du modèle avec le choix majoritaire des juges | ≥ 85 % là où les juges s'accordent à plus de 80 % | revues/signatures.md §4 | Calibrage | 5 |

## 6. Vie privée et sécurité du rendu

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M24 | Test de parenté, à la précision « visage », naissances et fratries comprises | AUC ≤ 0,70 **[à calibrer]** | [direction-artistique](domaines/direction-artistique.md) §3.4 ; population §1.3 | Classificateur sur les apparences perçues | 1 |
| M25 | Test de réidentification d'une personne masquée ou en silhouette | AUC ≤ 0,70 **[à calibrer]** | 05 §2.5 | Idem | 1 |
| M26 | Vue identique quand l'identification échoue | Écrans, vue et journal de conversation identiques | 05 §2.5 | Test automatique | 1 |
| M27 | Aucune valeur envoyée au client n'est une clé, une graine de flux ou un état de l'aléa du cœur | Test automatique vert | 05, en-tête (clôture, C3) | Test | 0 |
| M28 | Profil `public` sans `dev-commands`, `dev-introspection`, `external-pilot`, pause ni banc de pilotage | Erreur de compilation, artefact vérifié, refus de démarrer | 05 §2.3, §5 | CI et vérification de l'artefact | 0 |

## 7. Méta-jeu, équilibrage et modération

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M29 | Rythme de progression | ≈ 100 XP par heure au palier 0 bien joué ; tâches de travail ≤ 30 XP par heure réelle ; niveau 80 en ≈ 300 à 400 h au meilleur palier | 04 §1.1 | Joueurs synthétiques (`ts_meta`, `ts-sim`) | 2 |
| M30 | Invariants d'équilibrage et catalogue d'exploits | Tous verts dans la CI, y compris « sonde de la classe cachée » et « relance puis abandon » | 05 §2.24 ; clôture de l'équilibrage, P2 | CI et agents adverses | 2 |
| M31 | Constance de la modération par session | ≥ 90 % sur l'échantillon rejugé chaque mois **[à calibrer]** | social §9.4 | Rejugement mensuel | 5 |

## 8. Contenus et client

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M32 | Mémoire vidéo | ≤ 2,5 Go en 2.5D, ≤ 6 Go en 3D en qualité maximale **[à calibrer]** | direction-artistique §6 | Scène étalon | 3 |
| M33 | Poids des assets | LFS ≤ 4 Go en 2.5D, historique compris ; texture ≤ 16 Mo ; morceau de musique ≤ 30 Mo | direction-artistique §9.3 | `cargo xtask assets check` | 3 |
| M34 | Accessibilité visuelle | ≤ 3 éclats par seconde, toujours | direction-artistique §4.5 ; [interface](domaines/interface.md) §11.1 | Contrôle des assets et des effets | 3 |
| M35 | Couverture des animations | Au moins une animation par famille de gestes | 05 §2.2 | `cargo xtask assets check` | 3 |

## 9. Outillage

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M36 | Minutes de CI | ≤ 3 000 par mois (GitHub Pro depuis le 30/10/2026) ; mode minimal au-delà de 80 % | 05, en-tête ; clôture de l'architecture, S-6 | Relevé mensuel | 0 |
