# Revue de la génération procédurale

> Document de **revue**, rédigé par la session « Revue de la génération procédurale » le 29/10/2026. Il ne modifie aucun autre fichier : la session principale le relit, puis l'intègre au socle et transmet aux domaines les modifications du §13, comme pour les revues d'architecture, d'équilibrage et de sécurité.
> Périmètre : toute la génération procédurale du projet (ville, bâtiments, intérieurs, mobilier, transports, population, réseau social, état intérieur de départ, rodage, couche cachée, chaîne `ts-worldbuild`, artefacts, chargement, construction en cours de partie, outils de validation et d'édition). Les signatures comportementales sont hors périmètre (revue séparée).
> Les choix marqués **(décidé)** sont tranchés par cette revue, avec leur raison. Les valeurs **[à mesurer]** sont des objectifs de performance à vérifier par le banc de mesure ; les valeurs **[à calibrer]** sont des réglages de conception.
> Contraintes respectées : reproductibilité par graine sur un même binaire (05 §2.10) ; générateurs en Rust partagés par le serveur et les outils (05 §2.11, §2.14) ; carte livrée au client limitée à la part publique (S33) ; population en ligne jamais livrée (S2) ; contraintes C1 à C3 ; un monde = un processus (05 §2.7).

## Sommaire

0. [Résumé](#0-résumé)
1. [Méthode](#1-méthode)
2. [Analyse de l'existant](#2-analyse-de-lexistant)
3. [Comparaison des méthodes](#3-comparaison-des-méthodes)
4. [La chaîne de génération retenue](#4-la-chaîne-de-génération-retenue)
5. [Reproductibilité et parallélisme](#5-reproductibilité-et-parallélisme)
6. [Budgets de performance](#6-budgets-de-performance)
7. [Incrémentalité et cache](#7-incrémentalité-et-cache)
8. [Chargement et flux](#8-chargement-et-flux)
9. [Validateurs, métriques et outils](#9-validateurs-métriques-et-outils)
10. [Évolutivité](#10-évolutivité)
11. [Décisions](#11-décisions)
12. [Exigences par phase](#12-exigences-par-phase)
13. [Modifications proposées au socle et aux domaines](#13-modifications-proposées-au-socle-et-aux-domaines)
14. [Risques résiduels](#14-risques-résiduels)

---

## 0. Résumé

La conception de la génération est **solide sur le fond** : une ville et une population générées puis figées, des lieux et des personnages faits à la main qui servent d'ancrages, une population qui « génère une histoire, pas une photo », un générateur unique et déterministe, une validation automatique, un rodage, une couche cachée tirée par monde et une chaîne `ts-worldbuild` adressée par empreinte. Ce sont exactement les bons principes.

Ce qui manque, c'est **l'ingénierie de la chaîne** : dans quel ordre les choses se résolvent, avec quels algorithmes, à quel coût, et comment on évite de tout refaire à chaque retouche. La revue relève quatre problèmes de fond et deux failles :

1. **Une dépendance circulaire ville ↔ population.** La ville choisit la taille des logements « selon les ménages qui doivent y vivre » (ville §3.3), et la population loge ses ménages dans les logements de la ville (population §1.2, étape 3). De même, la biographie est tirée « à rebours depuis la situation actuelle » (étape 5) avant que l'emploi actuel soit affecté (étape 6), qui dépend lui-même des diplômes de la biographie. **Réponse** : une étape **Programme** en tête de chaîne, calculée sur les tables cibles (valeurs attendues, aucun tirage), que la ville réalise et que les deux populations remplissent ; puis la situation actuelle avant la biographie (P1, P18).
2. **Une fuite de sécurité par le solo.** Le solo embarque le serveur, donc la **part serveur** de la carte (niveaux des serrures, caméras et leur champ, emplacement des coffres, lieux équipés d'une alarme) : S33 ne protège rien si cette part est la même en ligne. **Réponse** : toute la part serveur de la ville est **tirée par monde** parmi des candidats génériques de la part publique, comme la couche cachée de la population (P12).
3. **Une seconde faille : la graine.** La population en ligne se reconstruit à partir du code public, des tables publiques et de **sa graine** : si la graine n'est pas secrète et longue, S2 ne protège rien. **Réponse** : graines secrètes de 128 bits tirées par un générateur cryptographique et gardées en zone 2, fonction `draw` à clé de 128 bits (P25).
4. **Le mobilier « généré à la demande par graine de pièce »** (ville §4.2) lie chaque sauvegarde à la version exacte du code du générateur, ignore les occupants (une chambre d'enfant chez un retraité) et, en ligne, révélerait la composition des ménages privés. **Réponse** : mobilier fixe dans la carte publique ; mobilier mobile dans l'état de la population, produit à l'emménagement et emporté au déménagement (P11).
5. **Aucune incrémentalité** : toute retouche refait toute la chaîne. **Réponse** : un graphe d'étapes adressé par empreinte, à la granularité de l'unité (quartier, îlot, parcelle, bâtiment, cellule), des **identités stables** par réconciliation géométrique et des retouches écrites comme des **surcharges** (P27, P28).
6. **Des goulots non chiffrés** : temps de trajet (une matrice complète coûterait une minute de calcul et des centaines de mégaoctets), maillage de navigation de toute l'agglomération, plans de 40 000 pièces, grands bâtiments en simulation complète d'un bloc. **Réponse** : hiérarchies de contraction personnalisables (P15), données dérivées en cache (P31), plans types paramétriques (P10), zones de bâtiment (P14), et des budgets chiffrés (§6).

**Les cinq améliorations principales** : (1) le Programme en tête de chaîne, qui rend la ville et les deux populations cohérentes par construction ; (2) la part serveur de la ville et les graines secrètes, qui ferment deux fuites ; (3) le graphe d'étapes adressé par empreinte avec identités stables, qui rend la génération incrémentale et les mises à jour de carte sûres ; (4) les méthodes retenues par couche (gabarits morphologiques par époque, grammaire de découpe sur le kit, plans types, aires de chalandise, synthèse IPU, flots à coût minimal, réseau par contextes, chronique d'Orvenne) ; (5) les données dérivées en cache et le flux par secteurs et par couches, qui donnent une création de monde solo en **≤ 10 s** et une chaîne complète d'une famille en **≤ 2 min** sur 8 cœurs **[à mesurer]**.

---

## 1. Méthode

- Lecture du socle (01 à 05), des domaines ville et population en entier, des passages utiles d'ia-pnj (§6, §13), d'economie (§4, §6.3), de social (§10.2), et des revues d'architecture (§1, §5, §6.4, §9, §10, §12, §13, §17, §19, §20, §22), de sécurité (S2, S3, S24, S33, §7) et d'équilibrage (E25).
- Pour chaque étape de génération : entrées, sorties, algorithme, coût, parallélisme, ce qui la rend incrémentale, ce qui la valide.
- Les ordres de grandeur viennent des domaines (10 000 habitants, ≈ 4 500 ménages, ≈ 4 750 logements, ≈ 600 établissements, ≈ 4 100 postes, emprise de 6 × 4 km) ; les coûts sont des estimations d'ingénieur à vérifier par le banc de mesure de la revue d'architecture (§20.6).
- Les choix suivent l'esprit du projet : décisions écrites à la main, poids et seuils calibrés hors ligne, aucun modèle appris dans la génération (ia-pnj §13.13).

---

## 2. Analyse de l'existant

### 2.1 Forces

| Force | Où | Pourquoi c'est juste |
|---|---|---|
| Générer puis figer, retouches à la main, même générateur en cours de partie | ville §4.1, population §0 | C'est la seule façon d'obtenir une ville soignée sans renoncer au procédural ni aux constructions en jeu |
| Lieux et personnages faits à la main **en premier**, le reste construit autour | ville §3.4, population §1.2 et §6.3 | Les ancrages donnent le caractère ; le procédural remplit sans les contredire |
| « Générer une histoire, pas une photo » | population §0 | La cohérence entre attributs, compétences, relations et souvenirs découle d'une cause commune |
| Événements partagés créés une seule fois | population §5.4 | Aucun souvenir incompatible entre deux témoins |
| Mêmes règles en génération et en jeu (réflexion rétrospective, progression) | population §4.3, §5.6 | L'état de départ ressemble à ce que le jeu produira |
| Validation automatique riche | population §1.3, ville §4.2 | Condition de la ville et de la population par monde |
| Couche cachée tirée par monde, au moins 3 candidats | population §1.6, E25, S3 | Le solo ne révèle rien d'un serveur |
| Maillage de navigation dérivé de la géométrie, sans voxels | architecture §17.1 (D35) | Exact, rapide, recalculable localement |
| `WorldSpec` de toutes tailles | architecture §17.4 (D37) | Indispensable à C3 et aux tests de charge |
| Aléa à clé (`draw`) | architecture §6.4 | Rend le parallélisme reproductible sans effort ; c'est aussi la clé de l'incrémentalité (§7) |

### 2.2 Lacunes

| # | Lacune | Conséquence | Réponse |
|---|---|---|---|
| L1 | Dépendance circulaire ville ↔ population (types de logements, usages) | Ordre indéfini ; la carte dépendrait d'un tirage de population alors qu'elle est commune aux deux familles | P1 |
| L2 | Biographie (étape 5) avant l'emploi (étape 6), qui dépend des diplômes de la biographie | Boucle sans point fixe ; biographies incompatibles avec l'emploi trouvé | P18 |
| L3 | Aucune méthode choisie pour les rues, les parcelles, les bâtiments, les intérieurs | Risque de ville uniforme ou illisible, de plans absurdes | P4 à P10 |
| L4 | Usages placés « avec quotas par quartier », viabilité corrigée après coup | Commerces mal placés, boucle de corrections étapes 6 à 10 coûteuse | P9 |
| L5 | Mobilier généré à la demande par graine de pièce | Sauvegardes liées à la version du code ; mobilier sans rapport avec les occupants ; fuite des ménages en ligne | P11 |
| L6 | Part serveur de la carte identique pour tous les mondes, embarquée par le solo | Serrures, caméras, coffres et lieux équipés d'alarme connus de tous | P12 |
| L7 | Graine de la population en ligne non protégée | La population privée se reconstruit avec le code public | P25 |
| L8 | Aucun identifiant stable des parcelles et des bâtiments entre versions de carte | La règle de mise à jour « parcelles non modifiées » (ville §4.5) est inapplicable | P28, P35 |
| L9 | Aucune incrémentalité, retouches non formalisées pour la ville | Chaque retouche refait toute la chaîne ; retouches perdues à la régénération | P27, P28 |
| L10 | Réseau de transport en commun, stationnement, éclairage, mobilier urbain absents des étapes | Données nécessaires à l'IA, à l'économie et au combat produites nulle part | P16, §4.4 |
| L11 | Rodage de famille sur une saison fixe, monde lancé dans n'importe quelle saison | Habitants en « rythme de novembre » un jour de juillet | P23 |
| L12 | Durée du rodage fixée sans critère | 60 jours peuvent être trop ou trop peu | P22 |
| L13 | Aucune histoire commune de la ville | Biographies sans repères collectifs ; bâtiments sans date cohérente avec leurs habitants | P3 |
| L14 | Grands bâtiments (usine, hôpital) entièrement en complet dès qu'un joueur y entre | Des centaines de personnages en complet d'un coup | P14 |
| L15 | Chaîne non embarquable (aucune exigence) | La population ou la ville par monde en solo seraient impossibles sur la machine du joueur | P40 |

### 2.3 Goulots de performance

| Goulot | Estimation naïve | Réponse |
|---|---|---|
| Temps de trajet entre ≈ 6 000 lieux, par mode | Matrice 6 000² × 4 modes ≈ 144 M valeurs (≈ 300 Mo) et ≈ 6 000 recherches de Dijkstra par mode (≈ 1 min) | Hiérarchies de contraction personnalisables, requêtes en microsecondes, cache par paire fréquente (P15) |
| Maillage de navigation de toute l'agglomération (≈ 5 900 cellules) | 30 à 60 s de calcul, 40 à 80 Mo | Donnée dérivée en cache adressé par empreinte, préremplie, résidente par cellule utile (P31) |
| Plans intérieurs (≈ 40 000 pièces) | Solveur général : minutes, échecs | Plans types paramétriques et petit solveur local borné (P10) |
| Relations de départ (≈ 600 000) | Comparaison de toutes les paires : 5·10⁷ paires | Contextes partagés, voisinage par cellules, fermeture triadique en O(N·k) (P21) |
| Rodage de famille (60 jours) et rodage propre (14 jours) | Fixé par la revue d'architecture : 60 jours < 1 min | Initialisation à l'équilibre, durée par critère de convergence, calendrier réel (P22, P23) |
| Régénération complète à chaque retouche | Plusieurs minutes | Graphe d'étapes et granularité par unité (P27) |
| Chargement du client | Toute la carte d'un bloc | Flux par secteur et par couche (P32) |

### 2.4 Incohérences entre documents

| Sujet | Valeurs en conflit | Réponse |
|---|---|---|
| Vacance des logements | ville §2.3 : ≈ 4 750 logements, 5 % ; population §3.1 : ≈ 4 700, 4 % | Une seule source, les tables cibles : vacance de **5 %** à la génération, soit ≈ 4 740 logements ; le régulateur garde sa cible de 4 à 5 % (P41) |
| Nombre de relations | population §5.3 : ≈ 60 complètes par habitant, ≈ 600 000 ; social §10.2 et architecture §1 : ≈ 1,5 million complètes, ≈ 5 millions compactes | Les deux sont justes à deux moments : **≈ 600 000 au départ**, plafonds du social (150 et 500 par personne) pour **dimensionner la mémoire** ; le monde converge entre les deux (P41) |
| Lieux faits main | ville §4.2 : insérés à l'étape 9 ; ville §3.4 et population : ancrages d'abord | Posés **avant les rues** (P8) |
| Mobilier | ville §4.2 : à la demande ; S33 : contenu des meubles en part serveur | P11 |

---

## 3. Comparaison des méthodes

### 3.1 Réseau de rues et morphologie

| Méthode | Principe | Forces | Faiblesses | Verdict |
|---|---|---|---|---|
| **Croissance simulée** (agents, modèles d'urbanisation) | Simuler des siècles de croissance | Histoire crédible, irrégularités naturelles | Lente, difficile à guider, non incrémentale, résultat imprévisible | **Locale seulement** : croissance organique bornée pour le vieux centre (P5) |
| **Systèmes de règles et L-systèmes** de rues | Prolonger des segments par règles | Rapide, varié | Difficile à contraindre (raccords, ponts, relief) | Non retenu seul |
| **Champs de tenseurs** (orientation des rues) | Un champ d'orientation guide les tracés | Raccords doux entre trames, suit le relief et le fleuve | Demande un post-traitement topologique | **Retenu** pour orienter et raccorder les trames (P5) |
| **Gabarits de quartiers** (morphologies paramétrées) | Une bibliothèque de formes urbaines : faubourg en damier, grand ensemble en dalle, lotissement en raquettes | Contrôlable, lisible, rapide, conforme aux identités de ville §3 | Répétitif si mal paramétré | **Retenu** comme cœur de la génération (P4, P5) |
| **Solveurs de contraintes** | Satisfaire des contraintes globales | Précis | Explosent avec la taille | **Local** : affectation du programme, localisation des services (P9) |
| **Wave function collapse** | Pavage par contraintes d'adjacence | Excellent pour le décor local | Aucune contrainte globale (connexité, accès), contradictions et retours en arrière au coût imprévisible, rendu « tuilé » | **Décor seulement** (P7) |
| **Apprentissage** (réseaux génératifs de plans) | Appris sur des villes réelles | Réaliste en surface | Non explicable, données sous licence, contrôle faible, contraire à l'esprit du projet | **Rejeté** |

**Conclusion** : une ville européenne se lit comme une **superposition d'époques**. On l'obtient par un **plan directeur** fait main (terrain, fleuve, axes, contours et époques des quartiers, ancrages) puis par des **gabarits morphologiques** appliqués **époque par époque** (P4), raccordés par un champ d'orientation.

### 3.2 Bâtiments et façades

| Méthode | Verdict |
|---|---|
| **Grammaire de découpe** (volumes puis façades découpés en étages et travées, à la manière de CGA) | **Retenue** : s'aligne naturellement sur la grille du kit (1 m, travées de 3 m, étages de 3 m, rez-de-chaussée de 4 m, ville §4.4), produit des données sémantiques (typologie, travées, ouvertures) que le client assemble en 2.5D puis en 3D |
| Catalogue de bâtiments complets | Utile pour les bâtiments uniques (lieux faits main) ; trop répétitif pour 5 000 bâtiments |
| Wave function collapse sur façades | Seulement pour les variations d'ornement d'une travée, si besoin |

### 3.3 Intérieurs

| Méthode | Principe | Verdict |
|---|---|---|
| Subdivision récursive (arbres de découpe, *treemaps*) | Rapide | Plans peu crédibles (pièces en enfilade, couloirs absents) |
| Croissance sur grille (pièces qui grandissent depuis des germes) | Bonnes formes | Échecs de connexité à corriger |
| Optimisation stochastique d'un graphe de programme (méthodes bayésiennes) | Plans très crédibles | Lent (secondes par plan), réglage délicat |
| **Plans types paramétriques + petit solveur** | Une bibliothèque de plans types par typologie et taille (T1 à T5, traversant ou sur cour, local commercial, atelier, bureau), chacun un graphe de pièces avec contraintes, ajusté au rectangle du logement par un solveur borné | **Retenu** (P10) : crédible, rapide, contrôlable, explicable ; les concepteurs écrivent des plans types comme ils écrivent des fiches d'ancrage |
| Apprentissage | — | Rejeté (mêmes raisons) |

**Générés à l'avance ou à la demande ?** Les **plans** (murs, pièces, portes, escaliers) sont générés **à l'avance** : ils sont dans la part publique (S33, RS39 accepté), le niveau simplifié a besoin du graphe des pièces **partout** (ville §8.2, personnages importants), et les adresses, la vue en coupe et l'économie (surfaces, loyers) en dépendent. Coût faible (§6.3). Le **mobilier** est traité au P11.

### 3.4 Population

| Méthode | Principe | Forces | Faiblesses | Verdict |
|---|---|---|---|---|
| Tirage indépendant par tables | Tirer chaque ménage selon des tables | Simple | Marges non tenues, totaux faux | Rejeté seul |
| **IPF / IPU** (ajustement proportionnel itératif, par ménages et par personnes) | Repondérer un échantillon pour tenir des marges aux deux niveaux | Rapide, tient les marges ménages **et** personnes | Demande un échantillon de départ | **Retenu**, avec un échantillon produit par une **grammaire de ménages** écrite à la main (P17) |
| Optimisation combinatoire (recuit) | Choisir des ménages pour tenir des cibles entières | Tient exactement des entiers | Plus lent | **Retenu en finition** sur les écarts restants (P17) |
| Réseaux bayésiens, MCMC | Lois jointes sans échantillon | Riches | Lourds à écrire et à valider | Partiellement : la grammaire de ménages est un réseau de lois conditionnelles écrit à la main |
| Échantillonnage sous contraintes pour les affectations | — | — | — | Remplacé par des **flots à coût minimal** (P19), exacts et déterministes |
| Modèles génératifs appris | — | — | — | Rejetés |

### 3.5 Réseau social

| Modèle | Verdict |
|---|---|
| Graphe aléatoire uniforme | Rejeté : aucune structure |
| Modèle de configuration (degrés imposés) | Insuffisant : ni regroupement, ni homophilie |
| Modèle à blocs stochastiques | Bon pour les communautés, mais les communautés d'Orvenne viennent des **lieux**, pas de blocs abstraits |
| Graphes exponentiels (ERGM) | Trop lents à ajuster pour 10 000 nœuds |
| **Contextes partagés + homophilie + fermeture triadique** | **Retenu** (P21) : c'est déjà l'esprit de population §5.3 ; on le précise avec des cibles mesurées (degrés, regroupement, assortativité, communautés) |

### 3.6 Récapitulatif des méthodes retenues

| Couche | Méthode |
|---|---|
| Plan directeur | Fait main en données (P2) ; générateur de plans directeurs pour les villes synthétiques |
| Histoire | Chronique d'Orvenne commune à la ville et à la population (P3) |
| Morphologie | Couches d'époque, gabarits morphologiques par quartier (P4) |
| Rues | Axes guidés ; trames par gabarit ; champ d'orientation ; croissance organique bornée pour la Cité (P5) |
| Parcelles | Découpe récursive par boîte orientée et bandes en front de rue (P6) |
| Bâtiments | Grammaire de découpe sur la grille du kit ; WFC pour le décor seulement (P7) |
| Lieux faits main | Pièces avec ports, posées avant les rues (P8) |
| Usages | Aires de chalandise et couverture (P9) |
| Intérieurs | Plans types paramétriques et petit solveur (P10) |
| Mobilier | Fixe dans la carte ; mobile dans la population (P11) |
| Part serveur | Tirée par monde parmi des candidats génériques (P12) |
| Ménages | Grammaire + IPU + entiers + recuit (P17) |
| Affectations | Flots à coût minimal (P19) |
| Biographies | Échantillonneur de parcours à rebours (P20) |
| Réseau social | Contextes + homophilie + fermeture triadique (P21) |
| Itinéraires | Hiérarchies de contraction personnalisables (P15) |

---

## 4. La chaîne de génération retenue

### 4.1 Vue d'ensemble

```
ENTRÉES FAITES MAIN (data/, versionnées, publiques sauf graines en ligne)
  plan directeur · chronique · tables cibles · catalogues (métiers, kits, plans types,
  gabarits morphologiques, grammaires) · fiches d'ancrage · pièces faites main · surcharges
        │
        ▼
 G0 PROGRAMME ─────────── ménages attendus → logements → établissements → postes
        │                  (valeurs attendues, aucun tirage : commun aux deux familles)
        ▼
 ┌──────────────── VILLE (une seule carte) ─────────────────────────────────────┐
 │ V1 terrain → V2 réseau principal + ancrages → V3 rues par quartier            │
 │ → V4 îlots et parcelles → V5 affectation du programme → V6 bâtiments          │
 │ → V7 plateaux, logements, pièces, mobilier fixe → V8 pièces faites main       │
 │ → V9 espace public (stationnement, éclairage, végétation, mobilier urbain)    │
 │ → V10 transports en commun → V11 registre, adresses, noms                     │
 │ → V12 candidats de la part serveur → V13 dérivés (graphes, itinéraires,       │
 │   navigation, collisions, acoustique, cellules, secteurs) → V14 validation    │
 └──────────────────────────────────────────────────────────────────────────────┘
        │  carte publique + candidats
        ▼
 ┌──────── POPULATION, par famille (solo : graine publique ; en ligne : graine secrète, zone 2) ┐
 │ P1 ancrages → P2 ménages (IPU) → P3 situation actuelle (logement, diplôme, emploi,          │
 │ école) → P4 lignées → P5 biographies à rebours → P6 personnes → P7 patrimoines, entreprises │
 │ → P8 emménagement (mobilier mobile) → P9 relations, groupes, souvenirs, état intérieur      │
 │ → P10 puissance et paliers → P11 rodage de famille → P12 validation                         │
 └──────────────────────────────────────────────────────────────────────────────────────────────┘
        │
        ▼
 EMPAQUETAGE : parts adressées par empreinte (carte publique, candidats, dérivés,
               population solo | population en ligne), rapport, manifeste signé
        │
        ▼
 CRÉATION D'UN MONDE (dans le processus du monde, avec la graine du monde)
   W1 chargement → W2 part serveur de la ville → W3 couche cachée de la population
   → W4 traces → W5 validation → W6 rodage propre (14 jours avant le départ)
   → W7 dates au calendrier → W8 premier instantané
        │
        ▼
 EN COURS DE PARTIE : constructions (V5 à V13 sur une parcelle), arrivées et naissances
   (PersonFactory), emménagements (P8), données dérivées recalculées localement
```

### 4.2 Les entrées faites main

| Entrée | Contenu | Format | Qui l'écrit |
|---|---|---|---|
| **Plan directeur** (P2) | Courbes de niveau de contrôle, tracé de l'Ostrelle et de ses berges, zones inondables, axes principaux (rocade, routes entre communes, avenues, ponts, voie ferrée), contours des quartiers avec leur **gabarit morphologique**, leur **époque** et leurs paramètres (densité, hauteur maximale, usages permis, niveau de revenu, entretien), emprises des **ancrages** (lieux faits main), **perspectives** (axe → repère visé), tracés indicatifs des lignes de bus et des pontons | RON + géométrie en centimètres entiers | Concepteurs, dans l'éditeur Godot (P39) |
| **Chronique d'Orvenne** (P3) | ≈ 30 à 50 événements datés **[à calibrer]** : fondation du bourg, arrivée du chemin de fer (gare, quartier de la Gare), industrialisation des Forges, guerres, construction des Peupliers (1965-1972), crues historiques, fermeture d'un haut-fourneau, plan social, ouverture de l'hypermarché, rénovation de la Cité, élections municipales passées, fait divers marquant, titre sportif du club | RON | Concepteurs |
| **Tables cibles** | Celles de population §2 et §3 (pyramide, ménages, revenus, métiers, statut d'occupation), plus la **vacance**, les **ratios d'équipement** (P9) et les taux de la grammaire de ménages | RON | Concepteurs, calibrées |
| **Catalogues** | Métiers (economie §3.8), kits (dimensions, travées, atténuation acoustique), **plans types**, **gabarits morphologiques**, **grammaires de bâtiments**, ensembles de mobilier, noms (prénoms par cohorte, noms de famille, thèmes de rues) | RON, compilés dans le paquet de contenus | Concepteurs |
| **Fiches d'ancrage** | Population §6.3 | RON | Concepteurs |
| **Pièces faites main** (P8) | Lieux du ville §3.4 et intérieurs de ville §5.4 (commissariat, tribunal), assemblés dans Godot avec le kit, exportés au format de `ts_world_model`, avec leurs **ports** (raccords de rue, entrées, quais de livraison) | Format `ts_world_model` | Concepteurs |
| **Surcharges** (P28) | Retouches de la carte générée, attachées à des identifiants stables | RON | Concepteurs, dans l'éditeur |
| **Graines** | Carte, famille solo (publiques) ; famille en ligne (secrète, zone 2) (P25) | — | Chaîne de publication |

### 4.3 Étapes : entrées, sorties, coût, parallélisme

Les coûts sont des objectifs sur la machine de construction de référence (8 cœurs) **[à mesurer]** ; « unité » désigne le grain de parallélisme et de cache (§7).

| Étape | Entrées | Sorties | Unité | Coût 1 cœur | Coût 8 cœurs |
|---|---|---|---|---|---|
| G0 Programme | Tables, plan directeur, catalogues | Ménages attendus par quartier et classe ; logements par taille et standing ; établissements par type ; postes par métier ; écarts | Agglomération | 0,5 s | 0,5 s |
| V1 Terrain | Plan directeur | Champ de hauteur (2 m), lit et berges, zones inondables | Tuile de 512 m | 1 s | 0,2 s |
| V2 Réseau principal et ancrages | Plan directeur, pièces | Graphe des axes, ponts, voie ferrée ; emprises réservées | Agglomération | 0,5 s | 0,5 s |
| V3 Rues | Gabarit et époque de chaque quartier, champ d'orientation | Graphe des rues avec profils (largeur, trottoirs, sens) | Quartier | 3 s | 0,6 s |
| V4 Îlots et parcelles | Rues, gabarits | Parcelles avec front de rue, époque, contraintes | Îlot | 2 s | 0,4 s |
| V5 Affectation du programme | Parcelles, programme | Typologie et usage de chaque parcelle ; réserve constructible | Quartier | 3 s | 0,8 s |
| V6 Bâtiments | Parcelle, typologie, grammaire | Volume, étages, travées, ouvertures, toiture, entretien | Parcelle | 5 s | 0,8 s |
| V7 Plans intérieurs et mobilier fixe | Bâtiment, programme du bâtiment, plans types | Logements et locaux, pièces, portes, escaliers, ascenseurs, mobilier fixe | Bâtiment | 25 s | 4 s |
| V8 Pièces faites main | Pièces exportées | Bâtiments et intérieurs des ancrages | Pièce | 1 s | 0,2 s |
| V9 Espace public | Rues, bâtiments, programme | Stationnement, aires de livraison, éclairage, arbres, bancs, boîtes aux lettres, abribus | Quartier | 2 s | 0,4 s |
| V10 Transports | Tracés indicatifs, rues, programme | Arrêts, lignes, horaires, besoins en véhicules et en personnel | Agglomération | 1 s | 1 s |
| V11 Registre, adresses, noms | Tout ce qui précède, chronique | Registre public des lieux, adresses, noms de rues, horaires, activités | Quartier | 1 s | 0,3 s |
| V12 Candidats de la part serveur | Bâtiments, pièces, mobilier fixe | Points de pose (caméras, coffres, caches), classes de portes, lieux équipables d'alarme | Bâtiment | 2 s | 0,3 s |
| V13 Dérivés | Géométrie complète | Graphes des rues, des pièces, des lieux ; hiérarchies d'itinéraires par mode ; maillage de navigation et collisions par cellule ; graphe acoustique ; cellules ; secteurs | Cellule / mode | 60 s | 10 s |
| V14 Validation de la ville | Tout | Rapport, erreurs bloquantes, alertes, métriques | Quartier | 10 s | 2 s |
| P1 à P10 Population (une famille) | Carte publique, programme, tables, fiches, catalogues | Population visible avant rodage | Quartier puis global | 40 s | 12 s |
| P11 Rodage de famille | Population, règles standard | Population rodée | Monde | — | ≤ 60 s |
| P12 Validation et rapport | Tout | Rapport | Global | 10 s | 3 s |
| Empaquetage | Parts | Artefacts adressés, manifeste | Part | 10 s | 3 s |
| **Total d'une famille, à froid** (ville comprise) | | | | | **≤ 2 min** |

Les deux familles (solo et en ligne) ne se construisent **jamais sur la même machine** : la famille en ligne se construit en zone 2 (P29), où la ville est reprise du cache public par son empreinte. Chaque construction tient en ≤ 2 min (plafond 3 min).

### 4.4 La ville, étape par étape

**G0 — Programme (décidé, P1).** À partir des tables cibles, le programme calcule, **sans aucun tirage**, les **ménages attendus** par quartier et par classe (taille, type, revenu, statut d'occupation), puis le **parc de logements** qui les accueille avec la vacance (5 %, P41) et la marge de sous-occupation et de surpeuplement du population §3.1, puis les **établissements** par type par des ratios d'équipement (P9), puis les **postes** par métier (effectifs types par établissement), et vérifie les totaux : postes ≈ actifs occupés sur place + vacance de 2 à 3 % ; places d'école ≥ enfants ; places d'hébergement collectif. Un écart est une erreur de conception, signalée avant toute géométrie. *Pourquoi : la carte est commune aux deux familles ; elle doit donc découler des valeurs attendues, et chaque famille remplit ensuite le même parc.*

**V1 — Terrain.** Champ de hauteur au pas de 2 m, interpolé entre les courbes de contrôle du plan directeur, avec un bruit borné par zone (coteaux de Hautevigne, plaine des Forges). Le lit de l'Ostrelle est creusé le long de son tracé ; berges, quais et zones inondables (crue, ville §6.5) en découlent. Coût négligeable, parallèle par tuile.

**V2 — Réseau principal et ancrages (décidé, P8).** Les axes du plan directeur sont lissés et accrochés au relief (pente maximale par type de voie **[à calibrer]** : ≈ 8 % pour les axes, ≈ 12 % pour les dessertes). Les **pièces faites main** sont posées à cette étape, avec leurs ports raccordés aux axes : mairie et place, gare, Pont-Vieux, commissariat, hôpital, port, usine, tour des Peupliers, grande villa, stade. Les parcelles voisines héritent de leur époque et de leur style (règle de contexte). *Pourquoi : un lieu fait main inséré après coup oblige à démolir ce qui a été généré à sa place ; posé d'abord, il devient une contrainte comme une fiche d'ancrage.*

**V3 — Rues par gabarit morphologique (décidé, P4, P5).** Chaque quartier est rempli **par couches d'époque**, de la plus ancienne à la plus récente, chaque couche respectant ce que les précédentes ont posé :

| Quartier | Époque principale | Gabarit | Méthode |
|---|---|---|---|
| La Cité | Avant 1800 | Organique, rayonnant autour de la place, de l'église et du pont | Croissance locale bornée depuis des noyaux, rues étroites (4 à 8 m), îlots irréguliers de 30 à 70 m |
| Quartier de la Gare | 1850-1910 | Percées régulières depuis la gare | Avenue axée sur la façade de la gare (perspective), trame régulière, îlots de 80 à 120 m |
| Les Tilleuls | 1870-1930 | Trame bourgeoise, square | Trame régulière, alignement continu, cours intérieures |
| Faubourg Saint-Roch | 1820-1930 | Damier déformé le long de l'ancienne route | Trame orientée par le champ (route, fleuve), îlots de 50 à 90 m avec cours d'ateliers |
| Les Quais | 1850-1970 | Grandes emprises portuaires | Grands îlots, voies de desserte, cours de chargement |
| Les Peupliers | 1965-1972 | Grand ensemble sur dalle | Gabarit de plan-masse : barres et tours posées sur une grille d'implantation, dalle, parkings, voies de desserte, espaces verts |
| Beaulieu | 1960-2000 | Campus et hôpital | Gabarit de campus : grandes emprises, allées, parkings |
| Hautevigne | Bourg ancien + 1960-2010 | Bourg compact, villas sur les coteaux, résidences fermées | Rues des coteaux **le long des courbes de niveau**, grandes parcelles, impasses |
| Forges-sur-Ostrelle | Bourg ancien, cités ouvrières 1890-1930, pavillons 1960-2000, zone d'activité 1980+ | Cités en bandes, lotissements en raquettes, zone à grandes parcelles | Gabarits par époque, accolés |

Le **champ d'orientation** (tenseurs construits à partir du fleuve, des axes et des courbes de niveau) donne la direction des trames et assure les raccords entre quartiers. Après tracé : fusion des nœuds proches, suppression des segments trop courts, angles d'intersection ≥ 30°, profils de rue par hiérarchie (largeur, trottoirs, stationnement, sens uniques dans la Cité). *Pourquoi : chaque quartier de ville §3 a une forme reconnaissable, et c'est la superposition des époques qui rend une ville européenne crédible.*

**V4 — Îlots et parcelles (décidé, P6).** Les îlots sont les faces du graphe des rues. Ils sont découpés selon leur époque :

- **îlots anciens et bourgeois** : bandes en front de rue (profondeur 12 à 25 m), puis découpe par la largeur de façade (5 à 8 m dans la Cité, 10 à 15 m aux Tilleuls), cœur d'îlot en cours ou en jardins ;
- **îlots modernes, lotissements, zones d'activité** : découpe récursive par boîte orientée, chaque coupe perpendiculaire au plus long côté, jusqu'à la surface cible (400 à 800 m² en lotissement, 2 000 à 20 000 m² en zone d'activité), avec accès à la rue garanti ;
- **grand ensemble et campus** : une seule grande parcelle par opération.

Chaque parcelle porte son époque, ses contraintes (hauteur maximale, recul, usages permis) et son **identifiant stable** (P28).

**V5 — Affectation du programme (décidé, P9).** Par quartier, on choisit la typologie de chaque parcelle pour réaliser le parc de logements et les établissements du programme :

1. **Établissements** : pour chaque type (boulangerie, pharmacie, école, bar, garage…), le programme donne un nombre et une **aire de chalandise** (rayon et population de seuil **[à calibrer]** : une boulangerie pour ≈ 1 500 habitants à moins de 400 m, une école élémentaire à moins de 800 m, une pharmacie par quartier habité). Les établissements sont placés par un algorithme glouton de **couverture maximale** sur les locaux possibles (rez-de-chaussée sur rue, places, angles), avec des bonus de **regroupement** pour les types qui s'attirent (bars de la Cité et du faubourg, commerces autour de la place et du marché) et de **répulsion** entre concurrents. Les services du ville §5.2 et §5.3 dont l'emplacement est décidé sont posés en premier.
2. **Logements** : affectation des typologies (immeuble ancien de 3 à 6 niveaux, immeuble moderne, barre, tour, pavillon, villa, logement d'atelier) aux parcelles pour tenir le parc par taille et standing, par un recuit borné sur le quartier (coût : écart au programme, respect des contraintes, cohérence d'époque, variété).
3. **Réserve constructible** : parcelles laissées en friche ou terrain vague près de la gare et des quartiers populaires (ville §6.2).

*Pourquoi : un commerce placé là où sont ses clients est viable par construction ; le contrôle de viabilité de population §3.4 devient une vérification, et la boucle de corrections des étapes 6 à 10 disparaît presque.*

**V6 — Bâtiments (décidé, P7).** Une **grammaire de découpe** par typologie et par époque, sur la grille du kit : volume (emprise, gabarit, retraits, toiture), niveaux (rez-de-chaussée commercial de 4 m ou logements, étages de 3 m, comble, cave pour les immeubles anciens, ville §5.1), façades en **travées de 3 m**, ouvertures par travée, balcons, corniches, enseignes. L'**état d'entretien** dépend du quartier et du propriétaire (ville §6.3). Le **wave function collapse** n'est employé, si besoin, que pour des variations d'ornement d'une travée ou des sols de cours, jamais pour la structure. Les sorties sont **sémantiques** (typologie, paramètres, références au kit), jamais des maillages : le client assemble les modèles, en 2.5D puis en 3D (05 §3).

**V7 — Plateaux, logements, pièces, mobilier fixe (décidé, P10).**

1. **Plateau** : chaque niveau reçoit sa ou ses **cages d'escalier** (et un ascenseur à partir de R+4 **[à calibrer]**), ses paliers et ses **gaines** (pièces humides superposées d'un étage à l'autre).
2. **Logements** : le plateau est découpé en logements le long des travées, selon le programme du bâtiment (tailles T1 à T5), par un remplissage unidimensionnel des travées autour du palier.
3. **Pièces** : chaque logement reçoit un **plan type** de sa taille et de sa configuration (traversant, sur rue, sur cour, d'angle), ajusté au rectangle par un **petit solveur** borné en nombre d'itérations. Règles : chaque pièce de vie a une fenêtre, salle d'eau et WC peuvent être aveugles et sont près de la gaine ; entrée → séjour → cuisine ; chambres desservies par un dégagement ; **les cloisons tombent sur les trumeaux** (jamais au milieu d'une fenêtre), ce qui garantit l'accord entre façade et intérieur ; dimensions minimales élargies pour la lisibilité en 2.5D (dégagements ≥ 1,4 m, portes ≥ 1 m, chambres ≥ 2,8 m de large **[à calibrer]**), plutôt qu'un agrandissement uniforme qui obligerait à grossir toute la ville (ville §2.2).
4. **Locaux** : plans types de local commercial (vitrine, réserve, WC), restaurant (salle, cuisine, réserve), atelier, bureau ; les grands équipements (école, collège, mairie, hôpital, usine) sont des **programmes de pièces** posés par le même solveur sur des trames plus larges, ou des pièces faites main.
5. **Mobilier fixe** : cuisine équipée, sanitaires, placards, comptoirs, vitrines, postes de travail, machines, lits d'hôpital, bancs d'église. C'est ce qui ne déménage pas.

En cas d'échec du solveur, on essaie le plan type suivant de la même classe (tentatives numérotées dans le flux `CityGen`, §5) ; au-delà de 8 tentatives **[à calibrer]**, le logement change de taille dans le programme du bâtiment et l'écart est signalé.

**V8 — Pièces faites main.** Les intérieurs des ancrages sont importés tels quels, avec leurs pièces, leurs portes, leur mobilier fixe et leurs candidats de part serveur.

**V9 — Espace public.** Stationnement sur rue et parkings (dimensionnés sur les véhicules attendus du programme, ≈ 1,2 véhicule par ménage hors du centre **[à calibrer]**), garages, aires de livraison (ville §7.4), **éclairage public** (densité selon le quartier, faible aux Quais, ville §6.4), arbres et parcs, bancs, poubelles, abribus, boîtes aux lettres, panneaux et plaques (ville §4.4). Chaque élément destructible a un identifiant stable.

**V10 — Transports en commun (décidé, P16).** À partir des tracés indicatifs du plan directeur : arrêts tous les 300 à 400 m **[à calibrer]** sur les rues praticables par un bus, à moins de 400 m de 90 % des logements desservis ; horaires construits sur les fréquences du ville §7.3 ; nombre de véhicules et de conducteurs déduit des temps de parcours, qui alimente le programme des postes (dépôt de bus, cheminots, pilote de la navette). *Pourquoi : les horaires réels et les postes indispensables de ces services ne peuvent venir que d'un tracé et d'une durée de parcours.*

**V11 — Registre, adresses et noms (décidé, P36).** Le registre public des lieux (ville §9.1, part publique) est rempli : type, usage, horaires, activités, postes, capacité, règles d'accès, plan public. **Adresses** à la française : numérotation depuis le fleuve ou le centre vers l'extérieur, impairs à gauche et pairs à droite, numéros bis là où une parcelle a été redécoupée. **Noms de rues** tirés de thèmes par époque et par quartier (métiers et anciennes portes dans la Cité, arbres aux Tilleuls, fleurs et lieux-dits dans les lotissements, figures locales **fictives** et événements de la chronique ailleurs), sans aucune personne réelle. Noms des commerces et des enseignes tirés d'un catalogue, puis remplacés par les noms des propriétaires quand la population les connaît (« Boulangerie Marchal »).

**V12 — Candidats de la part serveur (décidé, P12).** La part publique reçoit des **candidats génériques**, qui ne révèlent rien parce qu'ils existent partout : points de pose possibles d'une caméra (angles de façade sur rue, entrées, caisses, couloirs), emplacements possibles d'un coffre (pièces arrière, bureaux, chambres), emplacements possibles d'une cache (meubles et éléments fixes à fond, faux plafonds, caves, combles, gaines), **classe** de chaque porte (palière, de service, vitrine, cave) et liste des lieux **équipables** d'une alarme. Ce qui est réellement posé est tiré par monde (W2).

**V13 — Données dérivées (décidé, P13, P15, P31).**

- **Graphes** des rues (piétons, véhicules, sens, feux, passages), des pièces et des portes, des lieux.
- **Itinéraires** : une **hiérarchie de contraction personnalisable** par mode (marche, vélo, voiture, transports en commun par une recherche sur horaires). La structure est calculée une fois sur la topologie ; une modification des poids (verglas, crue, rue fermée, chantier) ne demande qu'une **personnalisation** de l'ordre de 100 ms **[à mesurer]**. Les temps de trajet entre lieux sont calculés **à la demande** (quelques microsecondes) et mis en cache pour les paires fréquentes (domicile-travail, domicile-commerces habituels). *Pourquoi : la matrice complète coûterait des centaines de mégaoctets et une minute de calcul, et deviendrait fausse à la première rue fermée.*
- **Géométrie physique étagée** (P13) : par niveau (sol extérieur, étages, cave, toit), des polygones 2D de sol et des segments de murs avec leur plage de hauteur ; le relief extérieur par le champ de hauteur ; escaliers, rampes et ascenseurs comme liaisons entre niveaux ; lignes de vue en 2.5D (segments et plages de hauteur, ouvertures). Collisions par `parry2d` par niveau. Coordonnées en **centimètres entiers**. *Pourquoi : la ville est faite d'étages discrets ; un modèle étagé coûte dix fois moins en mémoire et en calcul qu'une géométrie 3D, et le passage à la 3D ne touche que le rendu (C1).*
- **Maillage de navigation** fin par cellule de 64 m, dérivé des polygones de sol de chaque niveau (D35), et **graphe acoustique** (atténuation entre pièces, étages et extérieur, ville §5.6).
- **Cellules** et **secteurs** : les secteurs (12 à 15) sont calculés par un partitionnement du graphe des lieux qui suit les coupures naturelles (fleuve, voie ferrée, grands axes), avec un équilibre de population et d'emplois, puis figés avec leurs identifiants.
- **Zones de bâtiment** (P14) : un bâtiment de plus de 2 000 m² de plancher ou qui couvre plus de 2 cellules **[à calibrer]** (usine, hôpital, hypermarché, plateforme logistique, lycée, gare) est découpé en **zones** (ailes, halls, étages) qui servent d'unité au niveau de détail à la place du bâtiment entier.

**V14 — Validation de la ville** : §9.2 et §9.3.

### 4.5 La population, étape par étape

L'ordre est révisé (décidé, P18) : on fixe d'abord la **situation actuelle** de chacun, puis on écrit sa **biographie à rebours** depuis cette situation. Les numéros entre parenthèses renvoient aux étapes actuelles de population §1.2.

| Étape | Contenu | Méthode | Coût (1 famille) |
|---|---|---|---|
| **P1 Ancrages** (1) | Fiches d'ancrage placées dans leurs logements et postes, avec leurs contraintes | Affectation directe, contrôle des conflits | < 0,1 s |
| **P2 Ménages** (2) | Ménages de chaque quartier : type, âges, sexe, liens dans le ménage, revenu, statut d'occupation, **niveau de diplôme** et **catégorie de métier** de chaque adulte | Grammaire de ménages + IPU + entiers + recuit (P17), sous la contrainte du parc de logements du quartier | 2 s |
| **P3 Situation actuelle** (3, 6) | Logement de chaque ménage ; emploi de chaque actif ; école de chaque enfant ; hébergement collectif | Flots à coût minimal (P19) | 2 s |
| **P4 Lignées** (4) | Parenté entre ménages, familles étendues, proches à l'extérieur | Appariement par compatibilité d'âges, de quartiers et de noms ; tirage à clé | 1 s |
| **P5 Biographies** (5) | Vie datée à rebours depuis la situation actuelle | Échantillonneur de parcours sur tables de taux et chronique (P20) | 3 s |
| **P6 Personnes** (7) | Profil, attirance, chronotype, attributs, compétences, santé, voix | Comme population §4 et §5.6, à partir de la biographie | 2 s |
| **P7 Patrimoines et entreprises** (8) | Comptes, biens, dettes, propriétaires des entreprises, contrôle de viabilité | Comme population §3.4 et §4.7 ; viabilité presque toujours tenue grâce à P9 | 2 s |
| **P8 Emménagement** (nouveau) | Mobilier mobile, décor et objets personnels de chaque logement occupé | Ensembles de mobilier selon la taille du ménage, les âges, la richesse et les goûts (P11) | 2 s |
| **P9 Relations, groupes, souvenirs, état intérieur** (9) | Réseau initial, groupes et bandes, souvenirs fondateurs, événements partagés, état intérieur | Contextes + homophilie + fermeture triadique (P21) ; souvenirs et état intérieur comme population §5.4 à §5.6 | 10 s |
| **P10 Puissance et paliers** (10) | Influence, niveau, puissance, rang, palier, ajustement des rôles mal classés | Comme population §4.5 ; au plus 3 passes **[à calibrer]** | 3 s |
| **P11 Rodage de famille** (11) | Mise en mouvement | Initialisation à l'équilibre, durée par convergence (P22) | ≤ 60 s |
| **P12 Validation** (12) | Contrôles | §9.4 à §9.7 | 3 s |

**P2 — Ménages (décidé, P17).** Une **grammaire de ménages** écrite à la main (type de ménage → âge de la personne de référence → écart d'âge du couple → nombre et âges des enfants compatibles avec l'âge de la mère → décohabitation, grand-parent hébergé) produit un échantillon de départ de quelques dizaines de milliers de ménages candidats par quartier. L'**IPU** (ajustement proportionnel itératif aux deux niveaux, ménages et personnes) leur donne des poids qui tiennent les marges du quartier (pyramide, types de ménages, revenus, statut d'occupation, diplômes, catégories de métier). Les poids sont convertis en **entiers** (tirage tronqué-répliqué), puis un **recuit** borné échange des ménages pour annuler les écarts restants et tenir **exactement** le nombre de ménages par classe de logement (contrainte du parc, P1). Tous les tirages passent par `draw(graine de famille, Population, sujet, …)`.

**P3 — Situation actuelle (décidé, P19).** Trois **flots à coût minimal** à coûts entiers, résolus quartier par quartier quand c'est possible :

- **ménages → logements** : coût = écart de taille (pièces par personne), écart entre loyer ou prix et revenu (taux d'effort), statut d'occupation, ancienneté souhaitée ; un ménage ne quitte son quartier que si le programme le prévoit ;
- **actifs → postes** : coût = écart de qualification (diplôme, catégorie de métier), distance domicile-travail (temps de trajet par le mode du ménage, P15), chronotype pour les postes de nuit ; postes rares puis indispensables d'abord (population §3.2), par paliers de coût ; candidats limités aux 50 plus proches compatibles **[à calibrer]** pour garder un graphe creux ;
- **enfants → écoles** : carte scolaire (secteur du domicile), choix du privé selon le revenu et la tradition familiale.

Les égalités de coût sont départagées par `StableId`, ce qui rend le résultat unique. *Pourquoi : un flot à coût minimal donne l'optimum global en une fois, sans boucle de corrections, et reste reproductible au bit près.*

**P5 — Biographies (décidé, P20).** Un **échantillonneur de parcours** remonte le temps depuis la situation actuelle : il tire les dates des transitions (fin d'études, premier emploi, emplois successifs, mises en couple, naissances des enfants présents, arrivée en ville, achat du logement, séparations, deuils) dans des **tables de taux** par âge et par génération, **conditionnées** par ce qui est déjà fixé (l'âge des enfants présents fixe des naissances ; l'ancienneté dans le poste fixe la date d'embauche ; l'année de construction du bâtiment borne la date d'emménagement). La **chronique** (P3) injecte des événements collectifs : licenciés du plan social de l'usine, sinistrés de la crue, anciens élèves du collège avant sa reconstruction. *Pourquoi : des repères collectifs donnent des souvenirs partagés, des rancunes et des conversations crédibles (« depuis la fermeture du haut-fourneau… ») et rattachent la population à la ville.*

**P8 — Emménagement (décidé, P11).** Le mobilier **mobile** (lits, tables, sièges, rangements, électroménager, décor, objets personnels) est produit pour chaque logement occupé à partir du ménage (taille, âges, enfants, richesse, goûts tirés du profil et de l'époque) et de la pièce (type, surface, mobilier fixe), par placement à règles (contre les murs, zones de passage et de débattement des portes gardées libres). Il fait partie de la **population** de la famille, pas de la carte. Il **déménage avec le ménage** (cartons et camion, ville §6.6), et un nouvel arrivant reçoit le sien à son emménagement (`PersonFactory`). Taille estimée : ≈ 400 000 objets, ≈ 5 Mo **[à mesurer]**.

**P9 — Réseau social (décidé, P21).** Les liens viennent de **contextes partagés** (ménage, lignée, palier et immeuble, rue, équipe et établissement, classe, club, commerce habituel, groupe), chacun avec une probabilité de lien selon la taille du contexte et l'**ancienneté** commune (biographie). Les **amitiés** sont choisies par homophilie (âge, quartier, milieu, profil, loisirs) parmi les personnes rencontrées dans ces contextes, puis complétées par **fermeture triadique** (les amis des amis). Recherche des voisins proches par cellules et par contextes, jamais sur toutes les paires : coût en O(N·k). Cibles mesurées (§9.5) : distribution des degrés, coefficient de regroupement des amitiés, assortativité par âge et par milieu, part des liens entre quartiers, isolés voulus. Nombre de départ : **≈ 60 relations complètes** et 100 à 300 compactes par habitant (population §5.3) ; la mémoire est dimensionnée sur les plafonds du social (P41).

### 4.6 Création d'un monde

Dans le processus du monde (serveur, solo, `ts-sim`), avec la **graine du monde** (P25) :

| Étape | Contenu | Coût solo, 4 cœurs **[à mesurer]** |
|---|---|---|
| **W1 Chargement** | Parts de l'artefact vérifiées par empreinte ; dérivés pris dans le cache ou reconstruits | ≤ 3 s |
| **W2 Part serveur de la ville** (P12) | Tirage par monde parmi les candidats : niveau de chaque serrure (selon la classe de porte, la richesse du propriétaire, le quartier), caméras privées et municipales (budget par commune, ville §5.5), coffres, lieux réellement équipés d'une alarme et leur liaison, emplacement des caches | ≤ 0,2 s |
| **W3 Couche cachée de la population** | Population §1.6, A, par un petit solveur à contraintes (P24) | ≤ 0,2 s |
| **W4 Traces** | Population §1.6, B | ≤ 0,2 s |
| **W5 Validation** | §9.6 ; en cas d'échec, tentative suivante (P24) | ≤ 0,2 s |
| **W6 Rodage propre** (P23) | Les **14 jours du calendrier qui précèdent** la date de départ du monde, avec la météo du monde et ses vacances scolaires | ≤ 5 s |
| **W7 Dates** | Conversion au calendrier (population §9.2) | ≤ 0,1 s |
| **W8 Premier instantané** | Sur un fil de fond ; le jeu démarre sans l'attendre, le journal couvre l'écart | — |
| **Total** | | **≤ 10 s** (plafond 20 s) |

### 4.7 Génération en cours de partie

| Cas | Ce qui est généré | Où et quand | Effet sur le tick |
|---|---|---|---|
| **Construction** (P34) | V5 à V13 sur la parcelle : bâtiment, plans, mobilier fixe, candidats, dérivés locaux (graphes, personnalisation des itinéraires, navigation des cellules touchées) | Lancé au dépôt du permis sur une **copie** des entrées, sur un fil de fond ; **appliqué à une échéance fixe** (fin du gros œuvre, puis livraison), comme les calculs lourds de l'architecture §9.3 | Application ≤ 2 ms **[à mesurer]** : échange de pointeurs et deltas de graphes |
| **Démolition** | Retrait de la parcelle, dérivés locaux | Idem | Idem |
| **Changement d'usage** | Plans types du nouveau local, mobilier fixe | Idem, à la fin des travaux | Idem |
| **Dégâts** | Aucun : couche d'**état** séparée de la géométrie (ville §6.3) | — | — |
| **Naissance, arrivée** | `PersonFactory` : personne ou ménage complet, mobilier mobile | Tâche de l'ordonnanceur | Étalé |
| **Emménagement, déménagement** | Mobilier mobile déplacé ou complété (P8) | Tâche de l'ordonnanceur | Étalé |
| **Bâtiment de ville par monde** (plus tard) | Toute la chaîne V | À la création du monde | — |

Toute génération en cours de partie tire sa graine de `draw(graine du monde, CityGen ou Population, identifiant du projet ou de la personne, …)` : son résultat ne dépend ni de la vitesse de la machine ni de l'heure à laquelle le fil de fond a fini.

---

## 5. Reproductibilité et parallélisme

**Règles (décidé, P26)** :

1. **Fonctions pures par unité.** Chaque étape parallèle est une fonction pure de ses entrées sur une unité (quartier, îlot, parcelle, bâtiment, cellule, ménage) ; son aléa vient uniquement de `draw(graine, flux, sujet, emplacement, but)`, avec `sujet` = identifiant stable de l'unité et `emplacement` = numéro de tentative. Les résultats sont collectés **dans l'ordre des identifiants** (itérations parallèles ordonnées). Le résultat ne dépend donc ni du nombre de fils ni de l'ordre de fin.
2. **Flux dédiés** : `CityGen`, `CityServer` (part serveur par monde), `PopGen`, `PopHidden` (couche cachée), `Furnish`. Changer une règle de la population ne décale aucun tirage de la ville, et inversement (nombres aléatoires communs, architecture §6.4).
3. **Optimisations globales** (programme, flots à coût minimal, recuits, partition des secteurs) : algorithmes déterministes à coûts **entiers**, séquentiels ou parallélisés par quartier, départage des égalités par `StableId`, nombre d'itérations fixé (jamais « jusqu'à ce que le temps s'écoule »).
4. **Géométrie en entiers** (P13) : toutes les coordonnées stockées en centimètres entiers, les calculs internes en flottants **quantifiés à la sortie de chaque étape**. *Pourquoi : la reproductibilité n'est promise que sur un même binaire, mais la quantification rend les empreintes de la carte stables entre Windows et Linux dans la quasi-totalité des cas, ce qui permet à l'éditeur (Windows) et à la construction (Linux) de partager le cache.* Une comparaison croisée des empreintes Windows/Linux tourne en intégration continue comme **alerte**, non bloquante.
5. **Test de déterminisme** de la phase 0 étendu aux générateurs : une génération d'une ville synthétique de quelques secteurs et de sa population sur 1 fil puis sur N fils doit donner la même empreinte.
6. **Pas de budget en temps** dans les générateurs : toutes les bornes sont en itérations ou en tentatives (05 §2.10).

**Graines (décidé, P25)** :

- La graine de la carte et celle de la famille solo sont publiques (elles sont dans le paquet solo, de toute façon lisible).
- La graine de la **famille en ligne** et la graine de chaque **monde en ligne** sont **secrètes** : 128 bits tirés d'un générateur cryptographique, conservés en zone 2 avec les autres secrets (S28), jamais dérivés d'une information publique (nom du serveur, date). *Pourquoi : la population en ligne se reconstruit à partir du code public, des tables publiques et de sa graine ; seule la graine protège S2.*
- `draw` prend une **clé de 128 bits** (au lieu d'un `u64`), avec une fonction de mélange à clé de cette taille (Philox 4×64 ou équivalent). *Pourquoi : avec 64 bits, deviner la graine en comparant le nom d'une fiche d'ancrage observé en ligne coûterait de l'ordre de 2⁶⁴ évaluations rapides, à la portée d'un attaquant motivé ; 128 bits rendent l'attaque impossible pour un coût nul s'il est décidé en phase 0.*
- Une graine de monde solo est tirée localement ; elle n'a pas besoin d'être secrète (S24).

---

## 6. Budgets de performance

Machine de construction de référence : 8 cœurs récents, 32 Go. Machine solo de référence : 4 cœurs, 8 Go, disque SSD. Serveur de référence : celui de l'architecture §20.1. Toutes les valeurs sont **[à mesurer]** par le banc de performance de l'intégration continue (architecture §20.6), avec une alerte au-delà de 10 % de régression.

### 6.1 Temps de génération

| Opération | Objectif | Plafond |
|---|---|---|
| Programme (G0) | ≤ 0,5 s | 2 s |
| Ville complète à froid (V1 à V14) | ≤ 25 s sur 8 cœurs | 60 s |
| Population d'une famille avant rodage (P1 à P10) | ≤ 15 s sur 8 cœurs | 40 s |
| Rodage de famille (60 jours au plus) | ≤ 60 s sur 8 cœurs (architecture §20.2) | 120 s |
| Validation et rapport | ≤ 10 s | 30 s |
| **Chaîne complète d'une famille, à froid** | **≤ 2 min** | 3 min |
| Chaîne complète, deux familles (deux machines) | ≤ 2 × 2 min | — |
| Retouche d'une parcelle dans l'éditeur (V6 à V13 locaux, aperçu) | ≤ 300 ms | 1 s |
| Retouche d'un paramètre de quartier (V3 à V14 du quartier) | ≤ 5 s | 15 s |
| Reconstruction après retouche de la ville, population comprise, **sans** rodage | ≤ 30 s | 60 s |
| Prototype « morceau de ville » (carte du quartier + population du secteur) | ≤ 5 s | 15 s |
| Ville synthétique de 30 000 habitants (test de plafond) | ≤ 6 min | 12 min |

### 6.2 Création et chargement d'un monde

| Opération | Objectif | Plafond |
|---|---|---|
| Création d'un monde solo (W1 à W7), 4 cœurs | **≤ 10 s** | 20 s |
| Création d'un monde en ligne, 8 cœurs | ≤ 15 s | 60 s |
| Redémarrage d'un serveur depuis son dernier instantané, avant rejeu | ≤ 15 s | 30 s |
| Reprise d'une partie solo (instantané + rejeu du journal) | ≤ 5 s | 15 s |
| Premier affichage du client solo (secteur du joueur, extérieurs) | ≤ 2 s après la fin du chargement du monde | 5 s |
| Chargement d'un secteur voisin par le client (fond) | ≤ 200 ms | 500 ms |
| Population par serveur (plus tard) : famille entière + rodage à la création | ≤ 3 min | 10 min |
| Ville et population par monde en solo (plus tard), 4 cœurs | ≤ 3 min | 6 min |

### 6.3 Tailles des artefacts

| Part | Contenu | Brut | Compressé (`zstd`) |
|---|---|---|---|
| Carte publique | Terrain (≈ 12 Mo brut au pas de 2 m), rues, parcelles, bâtiments (≈ 5 000 × ≈ 0,5 Ko), pièces (≈ 40 000 × ≈ 150 o), ouvertures, mobilier fixe (≈ 150 000 objets), espace public, registre public, noms | ≈ 35 Mo | **≤ 20 Mo** |
| Candidats de la part serveur | Points de pose, classes de portes, lieux équipables | ≈ 5 Mo | ≤ 2 Mo |
| Dérivés (cache prérempli) | Navigation, collisions étagées, hiérarchies d'itinéraires, graphe acoustique | ≈ 100 Mo | ≤ 40 Mo |
| Population d'une famille, rodée | Personnes et composants (≈ 50 Mo), souvenirs de départ (≈ 20 Mo), relations (≈ 45 Mo), état intérieur (≈ 20 Mo), biographies, mobilier mobile, comptes et registres | ≈ 150 Mo | **≤ 60 Mo** |
| **Paquet solo, données du monde** | Carte publique + candidats + dérivés + famille solo | | **≤ 120 Mo** |

### 6.4 Mémoire

| Contexte | Objectif |
|---|---|
| Pic de la chaîne complète d'une famille | ≤ 4 Go |
| Pic de création d'un monde solo | ≤ 1,5 Go |
| Ville résidente dans un serveur (carte, registre, graphes, itinéraires, collisions étagées) | ≤ 150 Mo, plus le maillage de navigation résident borné à 64 Mo **[à calibrer]** |
| Monde complet de 10 000 habitants en serveur | 0,5 à 1 Go (architecture §20.4) |
| Client 2.5D, données de carte résidentes (≤ 9 secteurs) | ≤ 300 Mo mémoire vive + vidéo |

### 6.5 Coûts en cours de partie

| Opération | Objectif |
|---|---|
| Temps de trajet entre deux lieux (requête de hiérarchie) | ≤ 5 µs ; ≥ 90 % des requêtes du statistique servies par le cache |
| Personnalisation des itinéraires après une fermeture de rue ou une crue | ≤ 150 ms, sur un fil de fond |
| Maillage de navigation d'une cellule reconstruit | ≤ 10 ms, sur un fil de fond |
| Génération d'une construction (une parcelle) | ≤ 2 s sur un fil de fond |
| Application d'une construction dans le tick | ≤ 2 ms |
| `PersonFactory`, un ménage arrivant complet | ≤ 20 ms, tâche de l'ordonnanceur |
| Envoi d'une parcelle modifiée au client (`MapPatch`) | ≤ 100 Ko |

---

## 7. Incrémentalité et cache

### 7.1 Graphe d'étapes adressé par empreinte (décidé, P27)

`ts-worldbuild` est un **graphe d'étapes** dont chaque nœud a une **clé** :

```
clé = empreinte( nom de l'étape, VERSION de l'étape, paramètres utilisés,
                 empreintes des parts d'entrée, graine, identifiant de l'unité )
```

- Chaque étape déclare une constante `VERSION`, augmentée à chaque changement de comportement ; un **test doré** par étape (sortie de référence sur une petite ville) échoue si le comportement change sans que la version change. *Pourquoi : une clé fondée sur le seul code source invaliderait tout à chaque commentaire, une clé sans version servirait des résultats périmés.*
- Une étape ne lit **que** les paramètres qu'elle déclare : changer le prix du pain n'invalide pas les rues.
- Si la clé existe dans le cache, la sortie est réutilisée ; sinon l'étape tourne et sa sortie est enregistrée.
- Empreintes par **BLAKE3** sur la forme sérialisée canonique (postcard), en arbre (une part = la racine des empreintes de ses morceaux).

### 7.2 Granularité

| Étape | Unité de cache | Ce qu'une retouche locale régénère |
|---|---|---|
| G0 Programme | Agglomération | Tout ce qui en dépend **par quartier** : seuls les quartiers dont le programme change sont reconstruits |
| V1 Terrain | Tuile de 512 m | Tuiles touchées |
| V3 Rues | Quartier | Le quartier, puis ses îlots |
| V4 Parcelles | Îlot | L'îlot |
| V5 Affectation | Quartier | Le quartier (recuit local) |
| V6, V7, V12 | Parcelle, bâtiment | La parcelle et son bâtiment |
| V9 Espace public | Quartier | Le quartier |
| V13 Navigation, collisions | Cellule | Les cellules touchées |
| V13 Itinéraires | Mode | Personnalisation seule si la topologie ne change pas |
| P2, P3 Ménages, logements | Quartier | Les quartiers touchés |
| P4 à P10 | Famille | Toute la population (≤ 15 s) |
| P11 Rodage | Famille | Tout le rodage ; en développement, `--rodage court` ou `--rodage 0` |

Pour qu'une retouche locale ne change pas les ménages ailleurs, les tirages ont pour sujet l'identifiant stable de l'unité (§5) : une unité dont les entrées n'ont pas changé produit exactement la même sortie.

### 7.3 Identités stables (décidé, P28)

Les parcelles, bâtiments, logements, locaux, pièces, portes, éléments destructibles de l'espace public, arrêts et rues reçoivent un **identifiant stable** à leur première création. Quand une étape les régénère, une **réconciliation** les apparie aux anciens par recouvrement géométrique (parcelles et bâtiments : rapport intersection sur union ≥ 0,8 **[à calibrer]** ; pièces : même logement et même rôle ; rues : même tracé à une tolérance près) ; un élément apparié garde son identifiant, un nouvel élément en reçoit un neuf, un élément disparu est retiré. *Pourquoi : les mondes enregistrés, les surcharges, le cache, les mises à jour de carte (ville §4.5) et les souvenirs des habitants (« le bar du 12 rue des Tanneurs ») désignent tous des lieux par leur identifiant ; s'il changeait à chaque régénération, aucun de ces mécanismes ne survivrait à une retouche.*

Les identifiants de lieux sont des `StableId` dans un espace de noms réservé à la carte, distinct de celui des entités créées en jeu (architecture §6.1).

### 7.4 Retouches écrites comme des surcharges (décidé, P28)

Une retouche de la ville n'est **jamais** une modification de la sortie générée : c'est une **surcharge** attachée à un identifiant stable (« parcelle P-0412 : typologie = immeuble d'angle ancien, 5 niveaux » ; « bâtiment B-1180 : plan type du logement 3 = T3 traversant variante B » ; « rue R-077 : nom = rue des Tanneurs »), ou une modification du **plan directeur**. Les surcharges sont réappliquées après chaque régénération ; une surcharge dont la cible a disparu ou n'est plus compatible devient une **alerte** du rapport, que le concepteur résout. C'est la même règle que les retouches de la population, écrites comme des contraintes des tables ou des fiches (population §9.1). *Pourquoi : une retouche faite sur la sortie serait perdue à la régénération suivante.*

### 7.5 Deux caches séparés (décidé, P29)

- **Cache public** : carte, candidats, dérivés, famille solo. Partagé entre les postes des développeurs et l'intégration continue (stockage d'objets). Ses entrées ne sont jamais publiées telles quelles : une **construction de publication** recalcule tout à froid et compare ses empreintes au cache, ce qui vérifie aussi la reproductibilité.
- **Cache privé** : famille en ligne et tout ce qui en dérive, **uniquement en zone 2**, sur une machine de construction contrôlée. L'intégration continue publique ne construit **jamais** la famille en ligne et n'a jamais sa graine. *Pourquoi : un cache partagé contenant la population en ligne serait une fuite de S2 par la porte de service (RS28).*

### 7.6 Mises à jour de carte et mondes existants (décidé, P35)

Les règles de ville §4.5 s'appliquent grâce aux identifiants stables, avec une précision : une mise à jour qui change le **nombre ou la taille des logements** d'un bâtiment, ou les pièces d'un logement **occupé**, est **différée** jusqu'à ce que le logement concerné soit vacant ; les parties communes, façades et logements vacants changent tout de suite. *Pourquoi : changer le plan d'un logement habité déplacerait les meubles, les objets et les souvenirs de ses occupants sans aucune cause dans le monde.* Les identifiants de carte des mondes existants sont migrés par la même réconciliation (architecture §12.5).

---

## 8. Chargement et flux

### 8.1 Format de l'artefact (décidé, P30)

L'artefact de monde (architecture §17.3) devient un **ensemble de parts adressées par empreinte**, décrit par un manifeste signé :

| Part | Découpage interne | Public ? |
|---|---|---|
| `map.public` | Par **secteur** et par **couche** : terrain, rues et espace public, bâtiments, intérieurs, mobilier fixe, registre | Oui |
| `map.candidates` | Par secteur | Oui |
| `map.derived` | Par cellule (navigation, collisions), par mode (itinéraires), global (acoustique, secteurs) | Oui (cache) |
| `pop.solo` | Par famille de composants et par secteur de résidence (format d'instantané de l'architecture §12.1) | Oui |
| `pop.online` | Idem | **Non** : zone 2 seulement |
| `report` | Rapport de validation et métriques | `pop.online` : zone 2 |

- **Paquet du client** = `map.public`, `map.candidates`, `map.derived`, `pop.solo`. Un **contrôle automatique** de la chaîne de publication vérifie qu'aucune empreinte de `pop.online` ni du rapport privé n'apparaît dans le manifeste du client. *Pourquoi : S2 tient alors par une vérification, pas par une discipline.*
- **Serveur en ligne** = `map.public`, `map.candidates`, `map.derived`, `pop.online`.
- Même **codec** que les instantanés (postcard + `zstd`, enveloppe versionnée, décodage **borné**, architecture §12.1, S10) : aucun nouveau format à durcir.

### 8.2 Côté serveur (et hôte solo)

- Le serveur charge **toute** la carte publique, les candidats et la population : le niveau simplifié a besoin des graphes des pièces partout (ville §8.2).
- Les **dérivés** sont pris dans le cache prérempli s'ils correspondent à l'empreinte de la géométrie, sinon reconstruits. Le maillage de navigation fin n'est **résident** que pour les cellules utiles (bulles, plus une marge de préchargement), avec un budget de mémoire et une éviction des cellules les moins récemment utilisées ; une cellule absente est chargée ou reconstruite sur un fil de fond dès qu'une bulle s'en approche. Si une bulle en a besoin avant la fin, la boucle l'attend (c'est une fonction pure de la géométrie : le résultat ne dépend pas du moment).
- La **part serveur** tirée par monde (W2) est enregistrée avec l'état du monde, comme la couche cachée (05 §2.8).

### 8.3 Côté client : flux par secteurs et par couches (décidé, P32)

- Le client possède la carte publique initiale dans son paquet (identique pour tous les mondes, 05 §2.11). Il la lit par la bibliothèque native, **secteur par secteur** et **couche par couche** : d'abord terrain, rues et façades du secteur du joueur et des voisins, puis les intérieurs du bâtiment occupé et des bâtiments proches, puis le reste en fond.
- Le décodage se fait sur un fil de la bibliothèque native ; Godot reçoit des messages prêts à instancier (références au kit, transformations quantifiées).
- **En ligne**, les modifications propres au monde (constructions, démolitions, changements d'usage, dégâts visibles) arrivent par `MapPatch` (architecture §13.1) **par secteur**, à la connexion pour le secteur du joueur, puis à l'approche des autres, puis en direct.
- Le **mobilier mobile** et les objets ne font pas partie de la carte : ce sont des entités, envoyées par la vue filtrée quand le personnage les perçoit (05 §2.5). Pour éviter un flot de petits messages en entrant dans une pièce, le serveur envoie l'**ameublement perçu d'une pièce** en un seul message, que le client garde pour la session.
- La part serveur n'est **jamais** envoyée en bloc : une caméra visible, une serrure examinée, une alarme qui sonne arrivent par la vue (S33).

### 8.4 Niveaux de détail de la géométrie (décidé, P33)

| Niveau | Où | Contenu instancié |
|---|---|---|
| **Complet** | Bâtiment occupé par le personnage, bâtiments à moins de ≈ 40 m **[à calibrer]** (on voit par les fenêtres) | Façades, intérieurs de tous les étages utiles, mobilier fixe et perçu, coupe |
| **Façade** | Reste des secteurs chargés | Façades et toitures assemblées depuis le kit, aucun intérieur ; fenêtres éclairées selon l'état transmis (ville §6.4) |
| **Volume** | Au-delà (vues dézoomées, plan) | Volumes simples texturés par typologie |

- Assemblage par **instanciation multiple** (une instance par élément de kit et par secteur) : quelques centaines d'appels de dessin pour un écran 2.5D.
- **En 3D** : mêmes données, modèles détaillés du kit, **HLOD par secteur** (volumes fusionnés précalculés côté client à partir des mêmes données), imposteurs pour le lointain, occlusion. Les dimensions du kit ne changent pas (ville §4.4) ; le générateur fournit déjà toitures, corniches et ouvertures, dont la 3D a besoin. *Pourquoi : C1 ; la 3D ne doit demander ni nouvelle donnée serveur, ni nouveau protocole.*

### 8.5 Zones de bâtiment et niveau de détail de la simulation (décidé, P14)

La règle « tout le bâtiment qu'il occupe passe en complet » (ville §8.2) s'applique à la **zone de bâtiment** pour les grands bâtiments (V13) : un joueur dans un atelier de l'usine met en complet son hall et les zones contiguës visibles, pas les 300 postes de l'usine. *Pourquoi : la charge du complet est bornée par joueur (architecture §20.3) ; un grand bâtiment d'un bloc la ferait exploser à chaque visite.*

---

## 9. Validateurs, métriques et outils

### 9.1 Principes (décidé, P37)

- Chaque contrôle est **bloquant** (la chaîne échoue, le rapport dit où) ou **alerte** (signalée, chiffrée, suivie d'une construction à l'autre).
- Chaque contrôle est une fonction pure de l'artefact, testée sur des cas pièges ; les seuils sont des données du paquet de contenus.
- Chaque métrique a une **plage cible** (comme les indicateurs de `ts_metrics`) et figure dans le rapport, avec son évolution par rapport à la construction précédente.
- **Critère de la ville par monde** (ville §4.6, population §9.4) : sur 32 graines de villes synthétiques et 32 graines de populations, au moins **95 %** des constructions passent tous les contrôles bloquants **sans retouche**, et aucune métrique ne sort de sa plage dans plus de 5 % des cas **[à calibrer]**.

### 9.2 Ville

| Contrôle | Règle | Type |
|---|---|---|
| Programme | Écarts entre programme et réalisation par quartier : logements par taille ≤ 2 %, établissements obligatoires tous présents | Bloquant |
| Connexité piétonne | Chaque porte d'entrée de chaque bâtiment est reliée au réseau piéton ; aucune île de navigation non voulue | Bloquant |
| Accès des véhicules | Chaque bâtiment d'activité a un accès de livraison ; chaque logement a une place de stationnement à moins de 300 m **[à calibrer]** | Bloquant / alerte |
| Logements | Chaque logement a une entrée, un accès à l'escalier, une pièce de vie avec fenêtre ; ascenseur à partir de R+4 | Bloquant |
| Accessibilité des services | 95 % des logements à moins de 400 m d'une boulangerie ou d'une épicerie, 800 m d'une école élémentaire, 400 m d'un arrêt de bus **[à calibrer]** | Alerte |
| Temps de trajet | Distribution domicile-travail (après population) : médiane ≈ 15 min, 90 % ≤ 35 min **[à calibrer]** | Alerte |
| Géométrie des rues | Angles ≥ 30°, segments ≥ 8 m, pentes par type, largeurs par hiérarchie | Bloquant |
| Parcelles | Front de rue ≥ 5 m, rapport de forme borné, accès garanti | Bloquant |
| Bâtiments | Emprise dans la parcelle, reculs, hauteur ≤ maximum du quartier, cloisons sur les trumeaux | Bloquant |
| Cohérence usage-bâtiment | Commerce en rez-de-chaussée sur rue ; activité bruyante ou industrielle dans les zones permises ; bar hors des rues de lotissement | Bloquant |
| Chronologie | Année de construction cohérente avec l'époque de la parcelle et la chronique | Bloquant |
| Activité de nuit | Chaque secteur habité a au moins un lieu ouvert ou occupé la nuit (ville §9.1) | Alerte |
| Transports | Horaires réalisables (temps de parcours ≤ intervalle × véhicules) ; chaque arrêt accessible à pied | Bloquant |
| Stationnement | Places ≥ véhicules attendus × 1,1 par quartier | Alerte |
| Adresses | Numéros uniques par rue, parité respectée, sens croissant depuis le fleuve ou le centre | Bloquant |
| Part serveur | Chaque lieu équipable a au moins un candidat de chaque sorte prévue ; chaque bande a au moins 3 emplacements de cache candidats dans son territoire (population §1.6) | Bloquant |
| Ancrages | Chaque pièce faite main raccordée par tous ses ports | Bloquant |

### 9.3 Intérieurs

| Contrôle | Règle | Type |
|---|---|---|
| Connexité | Chaque pièce est accessible depuis l'entrée du logement ; chaque logement depuis le palier | Bloquant |
| Dimensions | Minimums par type de pièce (lisibilité en 2.5D) | Bloquant |
| Fenêtres | Pièces de vie éclairées ; aucune cloison au milieu d'une fenêtre | Bloquant |
| Portes | Aucun débattement contre un autre ; passage libre ≥ 0,9 m | Bloquant |
| Pièces humides | Près d'une gaine ; superposées à ± 1 travée d'un étage à l'autre | Alerte |
| Mobilier | Aucun meuble ne bloque un passage ou une porte ; chaque pièce a l'équipement minimal de son type | Bloquant |
| Variété | Un même plan type ne se répète pas plus de 3 fois dans un même immeuble, sauf typologie répétitive voulue (barre, tour) | Alerte |

### 9.4 Population

Tous les contrôles de population §1.3 restent, avec en plus :

| Contrôle | Règle | Type |
|---|---|---|
| Marges | Écart aux marges de chaque quartier ≤ 3 % (IPU), entiers exacts sur les classes de logement | Bloquant |
| Taux d'effort | Loyer ou mensualité ≤ 35 % du revenu, sauf ménages en difficulté voulus | Bloquant |
| Occupation | Pièces par personne dans les bornes, par quartier | Bloquant |
| Qualification | Aucun titulaire d'un poste sans le diplôme ou l'expérience exigés par la fiche de métier | Bloquant |
| Absurdités | Écarts d'âge des couples et des parents dans des bornes ; aucun événement de biographie avant la construction du bâtiment concerné ; aucun enseignant sans diplôme ; aucun mineur employé hors apprentissage ; aucune personne âgée dépendante seule sans aide | Bloquant |
| Chronique | Chaque événement de la chronique qui doit avoir des témoins en a | Alerte |
| Couverture | Population §4.6 ; et, pour chaque catégorie comportementale et chaque groupe de paliers, au moins un candidat dans chaque secteur habité **[à calibrer]** | Bloquant |
| Mobilier mobile | Chaque logement occupé est meublé selon son ménage (lits = occupants) | Bloquant |

### 9.5 Réseau social (décidé, P21)

| Métrique | Plage cible **[à calibrer]** |
|---|---|
| Relations complètes par adulte | Moyenne ≈ 60, 5ᵉ centile ≥ 10 (hors isolés voulus), 95ᵉ ≤ 150 |
| Amis par adulte | 3 à 15, médiane ≈ 7 |
| Coefficient de regroupement des amitiés | 0,2 à 0,4 |
| Assortativité par âge (amitiés) | ≥ 0,5 |
| Part des amitiés entre quartiers | 20 à 40 % |
| Distance moyenne dans le graphe des connaissances | ≤ 5 |
| Composante géante | ≥ 98 % des adultes |
| Réciprocité de connaissance (A connaît B ⇒ B connaît A) | ≥ 90 % des liens au-dessus du degré « connaissance » ; asymétries voulues (patron, notable) |

### 9.6 Couche cachée et part serveur (décidé, P24)

- Tirage par un **petit solveur à contraintes** : chaque élément choisi parmi ses candidats (≥ 3) par des tirages à clé sans remise ; contraintes d'exclusion (un informateur n'est ni le chef de sa bande ni un membre de son ménage ; un policier corruptible n'est pas le traitant de son propre informateur) et de cohérence avec la population visible.
- En cas d'échec de validation : **tentative suivante** dans le même flux (`emplacement` = numéro de tentative), au plus 8 **[à calibrer]** ; au-delà, la création du monde échoue avec une erreur qui désigne le candidat manquant (c'est un défaut de l'artefact, à corriger dans la chaîne, pas un aléa à cacher).
- Contrôles de population §1.6 et §1.3, plus : distribution des niveaux de serrure et du nombre de caméras par commune dans les plages de ville §5.5.

### 9.7 Rodage (décidé, P22)

- **Initialisation à l'équilibre** : les variables rapides (besoins, stocks des ménages et des commerces, trésorerie courante, humeur, avancement des projets en cours) sont tirées dans leurs **distributions stationnaires**, mesurées par `ts-sim` sur des runs longs et rangées dans le paquet de contenus avec leur provenance. Le rodage devient une **mise en mouvement** et une **vérification**, plus une construction.
- **Durée par convergence** : le rodage de famille s'arrête quand, sur deux fenêtres de 7 jours consécutives, les indicateurs de stabilité (taux d'emploi, faillites, faim, humeur, trésorerie, relations) varient de moins d'un seuil **[à calibrer]**, avec un minimum de 14 jours et un plafond de 60. La durée atteinte est écrite dans le rapport. Elle reste déterministe : même graine, même durée.
- Contrôles de stabilité de population §1.4 et §5.6, en alerte au-delà de leur plage, en bloquant au-delà du double.

### 9.8 Lisibilité et métriques morphologiques (décidé, P38)

- **Éléments de Lynch** : chaque quartier a au moins un **nœud** (place, carrefour animé) et un **repère** visible depuis ses voies principales ; les grands repères (clocher de la Cité, tour des Peupliers, cheminée de l'usine, gare, ponts) sont visibles depuis au moins 30 % des rues de leur secteur **[à calibrer]** ; les perspectives du plan directeur aboutissent bien sur leur repère.
- **Distinction des quartiers** : un indice de différence entre quartiers voisins (typologies, hauteurs, largeurs de rue, matériaux, végétation) au-dessus d'un seuil.
- **Morphométrie** : distributions de la taille des îlots, de la longueur des segments, de l'**entropie d'orientation** des rues, de la densité d'intersections, du coefficient d'emprise bâtie et de la hauteur, par époque, comparées à des **valeurs de référence** de villes européennes moyennes calculées hors ligne. Seules des **statistiques agrégées** entrent dans le dépôt, jamais une géométrie réelle. *Pourquoi : c'est un moyen objectif de dire « crédible », et les statistiques ne posent aucun problème de licence.*

### 9.9 Rapport, visualisation, différences

- `ts-worldbuild` écrit un **rapport HTML** (même outil que `ts-sim report`) : cartes SVG par quartier avec les couches (époques, typologies, usages, aires de chalandise, couverture des services, densités), histogrammes des métriques avec leurs plages, contrôles en échec avec un lien vers l'élément, temps et mémoire par étape, taux de réussite du cache.
- `ts-worldbuild diff <artefact A> <artefact B>` : ce qui a changé, par identifiant stable (parcelles, bâtiments, logements, ménages), avec les métriques avant et après. *Pourquoi : c'est l'outil de relecture d'une retouche et d'une mise à jour de carte.*
- **Visualiseur** (`ts-viz`, Rerun) : couches de la carte, graphes, maillage, réseau social (par quartier), et lecture pas à pas du rodage.
- Le rapport de la famille en ligne reste en zone 2 (P29).

### 9.10 Scénarios `ts-sim`

| Scénario | But |
|---|---|
| `worldbuild_robustesse` : 32 villes synthétiques × 32 populations | Critère de la ville par monde (§9.1) |
| `worldbuild_plafond` : ville synthétique de 30 000 habitants | Budgets (§6.1), plafond de population |
| `rodage_convergence` : rodage de 90 jours sur 16 graines | Mesure des distributions stationnaires et de la durée utile (P22) |
| `monde_creation` : 64 graines de monde sur la famille solo | Taux de réussite de W2 à W5, diversité de la couche cachée entre mondes |
| `construction_en_jeu` : 50 constructions et démolitions sur un an | Tick tenu, identifiants stables, itinéraires cohérents |
| `mise_a_jour_carte` : monde vieilli d'un an + nouvelle version de carte | P35 : rien d'occupé ne bouge, rien ne casse |
| `saisons_de_depart` : mêmes graines, départ en janvier, avril, juillet, octobre | P23 : agendas cohérents avec la saison dès le premier jour |

### 9.11 Édition dans Godot (décidé, P39)

- Un **greffon d'éditeur** Godot (C#) appelle le générateur par une bibliothèque native **de développement** (`ts_editor_native`, compilée avec les fonctionnalités de développement, jamais livrée).
- L'éditeur modifie **le plan directeur et les surcharges**, jamais la sortie générée ; chaque modification déclenche une régénération incrémentale (§7.2) et un aperçu en ≤ 300 ms pour une parcelle **[à mesurer]**.
- Les **pièces faites main** sont assemblées avec le kit dans des scènes Godot, puis exportées au format de `ts_world_model` par un script d'export ; l'intégration continue les réimporte et les valide (ports, dimensions, candidats).
- L'éditeur affiche les contrôles en échec sur la carte et les métriques du quartier courant.

---

## 10. Évolutivité

| Évolution | Ce que la chaîne doit déjà permettre |
|---|---|
| **Population extensible** | `PersonFactory` et l'emménagement (P8) produisent des ménages complets en jeu ; les constructions ajoutent des logements par la même grammaire ; le régulateur suit la capacité d'accueil (population §7.6) |
| **Plafond par serveur** | Budgets vérifiés sur la ville synthétique de 30 000 habitants (§9.10) ; les données sont par secteur, le coût du statistique est linéaire |
| **Population par serveur** (population §9.4, étape 2) | La chaîne d'une famille tourne en ≤ 3 min sur un serveur, sans service externe ; les fiches d'ancrage sont placées par le générateur ; la validation suffit (§9.1) |
| **Ville et population par monde** (étape 3) | Chaîne **embarquable** (P40) : `ts-worldbuild` est une bibliothèque Rust sans Python, sans réseau, à mémoire bornée (≤ 4 Go), avec validation et tentatives automatiques ; un **générateur de plans directeurs** produit des plans plausibles (fleuve, axes, quartiers avec gabarits et époques, ancrages placés dans des quartiers compatibles) ; budget de 3 min sur la machine solo |
| **Passage à la 3D** | Données sémantiques (typologies, travées, toitures, corniches, ouvertures, mobilier par références au kit) ; dimensions définitives dès la 2.5D ; HLOD côté client (P33) ; modèle physique étagé inchangé (P13) |
| **Nouveaux contenus** | Plans types, gabarits, grammaires, ensembles de mobilier et thèmes de noms sont des données ; ajouter un gabarit ne demande aucun code |
| **Nouvelle version de carte** | Identités stables, surcharges, mise à jour différée à la vacance (P28, P35) |

---

## 11. Décisions

| # | Décision (décidé) | Raison | § |
|---|---|---|---|
| P1 | **Programme d'abord** : une étape G0 calcule sur les tables cibles, sans tirage, les ménages attendus, le parc de logements, les établissements et les postes ; la ville le réalise, chaque famille le remplit | La carte est commune aux deux familles, elle ne peut dépendre d'aucun tirage de population ; la circularité ville ↔ population disparaît | 4.4 |
| P2 | **Plan directeur** fait main en données (terrain de contrôle, fleuve, axes, quartiers avec gabarit, époque et paramètres, ancrages, perspectives, tracés de lignes) ; un générateur de plans directeurs pour les villes synthétiques | La forme d'ensemble d'Orvenne est une décision de conception, le détail est procédural | 4.2 |
| P3 | **Chronique d'Orvenne** : ≈ 30 à 50 événements datés communs à la ville (époques des bâtiments) et à la population (biographies, souvenirs partagés) | Des repères collectifs rendent biographies, conversations et bâtiments cohérents entre eux | 4.2, 4.5 |
| P4 | **Couches d'époque et gabarits morphologiques** par quartier ; pas de croissance simulée complète | Une ville européenne se lit comme une superposition d'époques, et les gabarits restent contrôlables et rapides | 3.1, 4.4 |
| P5 | **Rues** : axes guidés, trames par gabarit, champ d'orientation pour les raccords, croissance organique bornée pour la Cité, rues des coteaux le long des courbes de niveau | Chaque quartier a sa forme reconnaissable et le relief est respecté | 4.4 |
| P6 | **Parcelles** : bandes en front de rue pour les îlots anciens, découpe récursive par boîte orientée ailleurs, une parcelle par opération pour les grands ensembles | C'est la structure foncière réelle de chaque époque | 4.4 |
| P7 | **Bâtiments** par grammaire de découpe sur la grille du kit ; WFC réservé au décor local | La grammaire produit des données sémantiques alignées sur le kit ; WFC ne tient aucune contrainte globale | 3.2, 4.4 |
| P8 | **Lieux faits main** = pièces avec ports, posées avant les rues ; parcelles voisines harmonisées | Un ancrage posé d'abord devient une contrainte, au lieu d'écraser ce qui a été généré | 4.4 |
| P9 | **Usages par aires de chalandise** (couverture maximale, regroupement et répulsion), services décidés posés d'abord | Un commerce placé près de ses clients est viable par construction | 4.4 |
| P10 | **Intérieurs générés à l'avance** par plans types paramétriques et petit solveur borné ; cloisons sur les trumeaux ; pièces humides superposées ; minimums élargis pour la 2.5D | Plans crédibles, rapides, accordés à la façade ; le niveau simplifié et la part publique en ont besoin partout | 3.3, 4.4 |
| P11 | **Mobilier fixe dans la carte publique, mobilier mobile dans la population** (emménagement, déménagement avec le ménage) ; remplace la génération à la demande par graine de pièce | Le mobilier dépend des occupants, la population en ligne est privée, et une sauvegarde ne doit pas dépendre de la version du générateur | 4.5 |
| P12 | **Part serveur de la ville tirée par monde** (serrures, caméras, coffres, alarmes, caches) parmi des candidats génériques de la part publique | Le solo embarque le serveur : une part serveur commune serait lue par tous, S33 ne tiendrait pas | 4.4, 4.6 |
| P13 | **Géométrie en centimètres entiers et modèle physique étagé** (2D par niveau, hauteurs, liaisons ; `parry2d` par niveau) | Dix fois moins de mémoire et de calcul qu'une géométrie 3D, empreintes stables, 3D limitée au rendu | 4.4, 5 |
| P14 | **Zones de bâtiment** pour les grands bâtiments, unité du niveau de détail | Un joueur à l'usine ne doit pas mettre 300 personnes en complet | 4.4, 8.5 |
| P15 | **Itinéraires par hiérarchies de contraction personnalisables**, requêtes à la demande avec cache, aucune matrice complète | Microsecondes par requête, quelques mégaoctets, et une crue se répercute en 100 ms | 4.4 |
| P16 | **Réseau de transport généré** (arrêts, lignes, horaires, besoins en véhicules et en personnel) | Les horaires réels et les postes indispensables en découlent | 4.4 |
| P17 | **Ménages par grammaire + IPU + entiers + recuit**, sous la contrainte du parc de logements | Tient les marges aux deux niveaux et les entiers exacts, en quelques secondes | 3.4, 4.5 |
| P18 | **Situation actuelle avant la biographie** : ménage, logement, diplôme, emploi et école d'abord, biographie à rebours ensuite | Supprime la boucle biographie ↔ emploi | 4.5 |
| P19 | **Affectations par flots à coût minimal** à coûts entiers, départage par `StableId` | Optimum global en une passe, reproductible, sans boucle de corrections | 4.5 |
| P20 | **Biographies par échantillonneur de parcours à rebours** sur tables de taux, conditionné par la situation actuelle et la chronique | Dates cohérentes par construction | 4.5 |
| P21 | **Réseau social par contextes, homophilie et fermeture triadique**, avec des cibles de métriques | Un réseau crédible et mesurable en O(N·k) | 4.5, 9.5 |
| P22 | **Initialisation à l'équilibre** des variables rapides et **durée de rodage par convergence** (14 à 60 jours) | Le rodage met en mouvement et vérifie ; sa durée est justifiée par une mesure | 9.7 |
| P23 | **Rodage propre sur les 14 jours du calendrier précédant le départ** du monde, avec sa météo et ses vacances | Un monde lancé en juillet a des habitants en rythme de juillet | 4.6 |
| P24 | **Couche cachée par un petit solveur à contraintes**, tentatives numérotées (≤ 8), échec = erreur de construction | Tirage reproductible et cohérent ; un échec révèle un défaut de l'artefact | 9.6 |
| P25 | **Graines secrètes de 128 bits** pour la famille et les mondes en ligne (générateur cryptographique, zone 2) ; `draw` à clé de 128 bits | Sans graine secrète et longue, la population en ligne se reconstruit avec le code public | 5 |
| P26 | **Parallélisme par fonctions pures par unité**, flux d'aléa dédiés, collecte ordonnée, optimisations globales déterministes, aucun budget en temps | Résultat identique quel que soit le nombre de fils | 5 |
| P27 | **Graphe d'étapes adressé par empreinte**, version explicite par étape avec test doré, granularité par unité | Ne régénérer que ce qui change, sans jamais servir un résultat périmé | 7.1, 7.2 |
| P28 | **Identités stables par réconciliation géométrique** ; retouches de la ville écrites comme des **surcharges** | Mondes, souvenirs, cache et mises à jour désignent les lieux par leur identifiant | 7.3, 7.4 |
| P29 | **Deux caches** : public (développeurs, intégration continue) et privé (zone 2) ; la famille en ligne ne se construit qu'en zone 2 | Un cache partagé ne doit jamais contenir la population en ligne | 7.5 |
| P30 | **Artefact en parts adressées** (carte publique, candidats, dérivés, population par famille, rapport), manifeste signé, contrôle automatique du paquet client | S2 tient par une vérification automatique | 8.1 |
| P31 | **Données dérivées en cache adressé par empreinte**, jamais source de vérité, préremplies, résidentes selon le besoin | Démarrage rapide, mémoire bornée, reconstruction locale après modification | 8.2 |
| P32 | **Flux client par secteur et par couche**, codec des instantanés, mobilier et objets par la vue | Premier affichage rapide, aucun nouveau format, rien de privé dans la carte | 8.3 |
| P33 | **Niveaux de détail de la géométrie** (complet, façade, volume ; HLOD en 3D) | Coût d'affichage borné en 2.5D comme en 3D, sans nouvelle donnée serveur | 8.4 |
| P34 | **Construction en cours de partie** générée sur un fil de fond à partir d'une copie, appliquée à échéance fixe | Aucun à-coup de tick, résultat indépendant de la machine | 4.7 |
| P35 | **Mise à jour de carte différée à la vacance** pour les logements occupés | On ne déplace pas les meubles d'un habitant sans cause dans le monde | 7.6 |
| P36 | **Adresses à la française** et noms de rues par époque et thème, sans personne réelle | Lisibilité et crédibilité, sans référence réelle (ville §2.4) | 4.4 |
| P37 | **Validateurs bloquants ou en alerte**, rapport HTML, critère de la ville par monde : 95 % de réussite sans retouche sur 32 × 32 graines | « Sans retouche humaine » (ville §4.6) devient mesurable | 9.1 |
| P38 | **Métriques de lisibilité et morphométrie** comparées à des statistiques agrégées de villes réelles | « Crédible » devient mesurable, sans géométrie réelle dans le dépôt | 9.8 |
| P39 | **Éditeur Godot** sur le plan directeur et les surcharges, aperçu incrémental, bibliothèque d'édition de développement jamais livrée | Les retouches survivent aux régénérations ; aucune fonction de développement dans le client | 9.11 |
| P40 | **Chaîne embarquable** : `ts-worldbuild` est une bibliothèque sans Python ni service, à mémoire bornée | Condition de la population par serveur et de la ville par monde en solo | 10 |
| P41 | **Harmonisation des chiffres** : tables cibles comme seule source ; vacance de 5 % (≈ 4 740 logements) ; ≈ 600 000 relations au départ, plafonds du social pour la mémoire | Deux valeurs pour une même grandeur finissent toujours par diverger | 2.4 |

---

## 12. Exigences par phase

Les phases sont celles de 05 §5 et de l'architecture §22. Une exigence est **bloquante** pour la fin de sa phase, comme celles de la revue de sécurité.

### Phase 0 — Fondations

1. `draw` à **clé de 128 bits** et flux `CityGen`, `CityServer`, `PopGen`, `PopHidden`, `Furnish` (P25, P26).
2. Espace de noms de `StableId` réservé à la carte (P28).
3. Types géométriques en centimètres entiers dans `ts_world_model` (P13).
4. Format de **parts adressées** (empreintes BLAKE3 en arbre, manifeste) avec le codec borné des instantanés (P30).
5. Test de déterminisme étendu à un générateur-jouet (une grille de parcelles et des ménages) : 1 fil contre N fils (P26).

**Terminé quand** (en plus des critères existants) : le générateur-jouet donne la même empreinte sur 1 et N fils.

### Phase 1 — Prototype du cœur

1. `ts_world_model` avec **part publique, candidats et part serveur** séparés dès le premier jour (P12).
2. `ts_citygen` réduit au prototype (faubourg Saint-Roch, Pont-Vieux, bord de la Cité) : plan directeur du morceau, G0, rues de deux gabarits (Cité organique, faubourg), parcelles, deux typologies d'immeuble, local commercial, atelier, bar, plans types de ces typologies, mobilier fixe, Pont-Vieux comme pièce faite main.
3. `ts_popgen` en mode « morceau de ville » avec G0, IPU, flots à coût minimal et biographies à rebours (P17 à P20) ; emménagement (P11).
4. Dérivés : graphes, itinéraires par hiérarchie (au moins la marche), collisions étagées, navigation par cellule en cache (P13, P15, P31).
5. Création d'un monde : W2 (part serveur) et W3 à W5 (couche cachée) par solveur à tentatives numérotées (P24), testés par graine.
6. Rodage avec initialisation à l'équilibre et mesure de convergence (P22).
7. Graphe d'étapes avec cache à la granularité **de l'étape** (la granularité par unité vient en phase 5).
8. Contrôles bloquants de connexité, logements, intérieurs, affectations ; rapport HTML minimal.

**Terminé quand** : le morceau de ville et sa population se génèrent en ≤ 5 s, se recréent à l'identique sur 1 et N fils, et passent leurs contrôles ; une retouche d'un paramètre de quartier ne refait que les étapes en aval.

### Phase 2 — Échange de pilote

1. La part serveur et la couche cachée sont dans les données du monde enregistré et du rejeu ; un test vérifie que deux mondes de graines différentes n'ont ni les mêmes caches ni les mêmes serrures.

### Phase 3 — Client Godot 2.5D en solo

1. Lecture de la carte **par secteur et par couche** par la bibliothèque native ; assemblage par instanciation multiple ; niveaux complet, façade, volume ; coupe (P32, P33).
2. Mobilier mobile et objets par la vue, ameublement perçu d'une pièce en un message (P32).
3. **Contrôle automatique du paquet client** : aucune part privée, aucune part serveur, aucun rapport privé (P30).
4. Création d'un monde solo en ≤ 10 s sur la machine solo de référence, avec écran de progression (§6.2).
5. Greffon d'éditeur minimal : afficher la carte générée, éditer une surcharge, aperçu (P39).

### Phase 4 — En ligne

1. Graine de famille en ligne et graines de monde tirées par générateur cryptographique, stockées en zone 2 (P25).
2. Cache privé en zone 2 ; l'intégration continue publique ne voit ni la graine ni la famille en ligne (P29).
3. `MapPatch` par secteur à la connexion et à l'approche (P32).
4. Redémarrage du serveur ≤ 15 s avant rejeu (§6.2).

### Phase 5 — Extension

1. **Ville complète** : tous les gabarits et époques, toutes les typologies et plans types, pièces faites main, espace public, transports, adresses et noms, zones de bâtiment ; granularité de cache par unité ; identités stables par réconciliation ; surcharges ; éditeur complet ; `ts-worldbuild diff` ; métriques de lisibilité et morphométrie (P2 à P16, P27, P28, P36 à P39).
2. **Budgets** du §6 tenus et suivis par le banc de performance.
3. **Robustesse** : scénarios du §9.10 ; critère des 95 % mesuré (il conditionne la population par serveur et la ville par monde).
4. **Constructions en jeu** (P34) et **mises à jour de carte** différées à la vacance (P35), testées par scénario.
5. **Avant toute ouverture publique d'un monde en ligne** : famille en ligne construite en zone 2 avec sa graine secrète, contrôle du paquet client vert (P25, P29, P30), en plus des exigences de la revue de sécurité.
6. **3D** : HLOD par secteur, modèles détaillés du kit, sans changement de données (P33).

---

## 13. Modifications proposées au socle et aux domaines

À intégrer par la session principale ; les domaines appliquent eux-mêmes ce qui les concerne.

### 13.1 [05-architecture.md](../05-architecture.md)

| Section | Modification |
|---|---|
| En-tête | Ajouter : « Complété le 29/10/2026 par la [revue de la génération](revues/generation.md) (P1 à P41), qui détaille la chaîne de génération, ses budgets, son cache et ses validateurs. » |
| §2.10 (reproductibilité) | Ajouter : « L'aléa à clé prend une **clé de 128 bits** ; les graines de la famille en ligne et des mondes en ligne sont **secrètes**, tirées par un générateur cryptographique et gardées en zone 2 (P25). Les générateurs suivent les mêmes règles : fonctions pures par unité, flux dédiés, collecte ordonnée, aucun budget en temps (P26). » |
| §2.11 (ville) | Ajouter : « La géométrie est stockée en **centimètres entiers** ; le modèle physique du cœur est **étagé** (2D par niveau, hauteurs, liaisons entre niveaux), la 3D ne concerne que le rendu (P13). La **part serveur** de la ville (serrures, caméras, coffres, alarmes, caches) est **tirée par monde** parmi des candidats génériques de la part publique (P12). Le mobilier fixe appartient à la carte, le mobilier mobile à la population (P11). Les lieux de la carte ont des **identifiants stables** conservés par réconciliation ; les retouches sont des **surcharges** (P28). » |
| §2.12 (découpage spatial) | Ajouter : « Les grands bâtiments sont découpés en **zones de bâtiment**, unité du niveau de détail (P14) ; les secteurs sont calculés par partition du graphe des lieux le long des coupures naturelles, puis figés. » |
| §2.14 (générateur de population, `ts-worldbuild`) | Remplacer la description de la chaîne par : « `ts-worldbuild` est un **graphe d'étapes adressé par empreinte** (P27) : **Programme** (ménages attendus, logements, établissements, postes, sans tirage, P1) → ville → population par famille (situation actuelle puis biographie à rebours, P17 à P21) → rodage (initialisation à l'équilibre, durée par convergence, P22) → validation → **artefact en parts** (carte publique, candidats, dérivés, population solo ou en ligne, P30). La famille en ligne ne se construit qu'en zone 2, avec un cache séparé (P29). À la création d'un monde : part serveur de la ville, couche cachée, rodage propre sur les 14 jours précédant la date de départ (P12, P23, P24). La chaîne est une bibliothèque embarquable (P40). » |
| §2.19 (itinéraires) | Ajouter : « Itinéraires par **hiérarchies de contraction personnalisables** par mode, requêtes à la demande avec cache (P15). » |
| §2.22 (boucle) | Ajouter aux calculs différés : « les **constructions** sont générées sur un fil de fond à partir d'une copie et appliquées à échéance fixe (P34). » |
| §3 (rendu) | Ajouter : « Le client lit la carte **par secteur et par couche** et l'affiche en trois niveaux de détail (complet, façade, volume ; HLOD en 3D) (P32, P33). » |
| §5, phases 0 à 5 | Ajouter les exigences du §12 de cette revue, et un renvoi : « Chaque phase a aussi ses exigences de génération ([revues/generation.md](revues/generation.md), §12). » |

### 13.2 [CLAUDE.md](../../../CLAUDE.md), section « Architecture »

Ajouter une puce : « **Génération** (docs/conception/revues/generation.md) : chaîne `ts-worldbuild` en graphe d'étapes adressé par empreinte, Programme d'abord, identités stables et retouches en surcharges ; part serveur de la ville et couche cachée tirées par monde ; graines en ligne secrètes de 128 bits ; artefact en parts, paquet client vérifié. »

### 13.3 [README.md](../README.md)

Ajouter la revue de la génération au point 5 du « Déroulement de la conception », et une ligne au tableau des relectures quand elle sera intégrée.

### 13.4 [domaines/ville.md](../domaines/ville.md)

| Section | Modification |
|---|---|
| §2.3 | Logements ordinaires : ≈ 4 740 pour 5 % de vacance, chiffre calculé par le Programme à partir des tables cibles (P1, P41) |
| §2.2 | Remplacer l'agrandissement des intérieurs par des **dimensions minimales élargies** des plans types (dégagements, portes, largeur des chambres) (P10) |
| §3.4 | Les lieux faits main sont des **pièces avec ports**, posées **avant les rues** ; les parcelles voisines prennent leur époque et leur style (P8) |
| §4.2 | Remplacer le tableau des étapes par la chaîne V1 à V14 de cette revue (§4.4), précédée du Programme (G0) et du **plan directeur** (P1, P2) ; ajouter la **chronique** (P3), les **gabarits morphologiques par époque** (P4, P5), les parcelles (P6), la grammaire de bâtiments (P7), les aires de chalandise (P9), les plans types (P10), l'espace public, les transports (P16), les adresses et noms (P36), les candidats de la part serveur (P12) et les dérivés (P13 à P15) |
| §4.2, dernier paragraphe (mobilier) | Remplacer : « Le **mobilier fixe** (cuisine, sanitaires, placards, comptoirs, équipements) fait partie de la carte publique. Le **mobilier mobile** et le décor sont produits par la population à l'emménagement et déménagent avec le ménage ; leurs modifications sont des modifications d'objets du monde (P11). » |
| §4.3 | Préciser : la part serveur **entière** (serrures, caméras et leur champ, coffres, alarmes et leur liaison, caches) est tirée par monde parmi des **candidats génériques** de la part publique ; la part publique ne contient que ces candidats (P12). Ajouter : géométrie en centimètres entiers, identifiants stables, retouches en surcharges (P13, P28) |
| §4.4 | Ajouter : les sorties du générateur sont **sémantiques** (typologies, travées, ouvertures, toitures, références au kit) ; le client assemble (P7, P33) |
| §4.5 | Ajouter : identités stables par réconciliation ; mise à jour **différée à la vacance** pour les logements occupés (P28, P35) |
| §4.6 | Critère « sans retouche humaine » : 95 % de réussite des contrôles bloquants sur 32 × 32 graines synthétiques (P37) ; générateur de plans directeurs (P2) |
| §5.5 | Niveaux des serrures, caméras, coffres et lieux équipés d'alarme **tirés par monde** dans les plages de cette section (P12) |
| §6.2 | Ajouter : la construction est générée sur un fil de fond dès le permis et appliquée à échéance fixe (P34) |
| §7.3 | Ajouter : arrêts, horaires et besoins en personnel **générés** à partir des tracés de lignes (P16) |
| §8.1 | Ajouter les **zones de bâtiment** (P14) ; les secteurs sont calculés puis figés |
| §8.3 | Ajouter : géométrie physique étagée ; itinéraires par hiérarchies personnalisables ; maillage de navigation en cache dérivé (P13, P15, P31) |
| §11 | Noter ces impacts comme venant de la revue de la génération |

### 13.5 [domaines/population.md](../domaines/population.md)

| Section | Modification |
|---|---|
| §1.1 | Ajouter aux entrées : le **Programme** (P1), la **chronique** (P3), la grammaire de ménages, les distributions stationnaires (P22) ; aux sorties : le **mobilier mobile** des logements occupés (P11) |
| §1.2 | Remplacer l'ordre des étapes par P1 à P12 de cette revue (§4.5) : la **situation actuelle** (logement, diplôme, emploi, école) avant la **biographie à rebours** ; ajouter l'**emménagement** (P11, P18) ; préciser les méthodes : grammaire + IPU + entiers + recuit (P17), flots à coût minimal (P19), échantillonneur de parcours (P20), réseau par contextes (P21) |
| §1.3 | Ajouter les contrôles du §9.4 et les métriques du réseau du §9.5 ; vacance de 5 % (P41) |
| §1.4 | Rodage de famille : initialisation à l'équilibre, durée par convergence entre 14 et 60 jours (P22) |
| §1.5 | Le mode « morceau de ville » part du Programme restreint au secteur (P1) |
| §1.6 | Rodage propre sur les **14 jours du calendrier précédant le départ**, avec la météo du monde (P23) ; tirage par solveur à contraintes, tentatives numérotées (≤ 8), échec = erreur (P24) ; la part serveur de la ville est tirée à la même étape (P12) ; graine du monde secrète en ligne (P25) |
| §2.2 | Préciser que les ménages sont tirés sous la contrainte du parc de logements du Programme (P17) |
| §3.1, §3.2, §3.3 | Affectations par flots à coût minimal à coûts entiers, départage par identifiant (P19) |
| §3.4 | La viabilité est obtenue d'abord par la localisation des établissements (aires de chalandise, P9) ; les corrections deviennent rares |
| §5.1 | Ajouter la **chronique** comme source d'événements de biographie et de souvenirs partagés (P3) ; biographie conditionnée par l'année de construction du logement |
| §5.3 | Méthode et cibles du réseau (P21, §9.5) ; préciser que ≈ 600 000 relations au départ et les plafonds du social dimensionnent la mémoire (P41) |
| §9.1 | Ajouter : graine de la famille en ligne secrète (128 bits, zone 2), cache privé, famille en ligne construite seulement en zone 2, paquet client vérifié (P25, P29, P30) |
| §9.4 | Ajouter : chaîne embarquable et budget de 3 min pour la population par serveur (P40) |
| §12 | Noter ces impacts comme venant de la revue de la génération |

### 13.6 [domaines/economie.md](../domaines/economie.md)

| Section | Modification |
|---|---|
| §4.4, §4.5 | Fournir au Programme les **ratios d'équipement** et les **aires de chalandise** par type d'établissement (population de seuil, rayon, regroupement ou répulsion) ; les opportunités mensuelles (§4.5) utilisent le même calcul de demande non servie (P9) |
| §6.3 | Les loyers et prix de départ servent au coût du flot ménages → logements (taux d'effort) (P19) |
| §9 (niveaux) | Fournir les **distributions stationnaires** des stocks et trésoreries pour l'initialisation à l'équilibre (P22) |

### 13.7 [domaines/ia-pnj.md](../domaines/ia-pnj.md)

| Section | Modification |
|---|---|
| §6.3 | Montée en complet par **zone de bâtiment** pour les grands bâtiments (P14) |
| §6.5 | Rodage : initialisation à l'équilibre et durée par convergence pour la famille ; rodage propre sur les 14 jours précédant le départ (P22, P23) ; fournir les distributions stationnaires des besoins, de l'humeur et de l'avancement des projets |
| §13.6 | Les valeurs apprises de départ sur le domicile portent aussi sur le mobilier mobile du ménage (P11) |

### 13.8 [domaines/social.md](../domaines/social.md)

| Section | Modification |
|---|---|
| §10.2 | Préciser que ≈ 1,5 million et ≈ 5 millions sont des **plafonds** de dimensionnement ; le réseau de départ compte ≈ 600 000 relations complètes (P41) ; fournir les plages cibles des métriques du §9.5 |

### 13.9 [domaines/interface.md](../domaines/interface.md)

| Section | Modification |
|---|---|
| Chargement et carte | Le client affiche la carte par secteur et par couche, avec les niveaux complet, façade et volume ; l'écran de création d'un monde solo montre une progression (≤ 10 s) (P32, P33) |
| Adresses | Numérotation à la française et noms de rues par thème (P36) pour les plaques et la carte |

### 13.10 [domaines/combat.md](../domaines/combat.md)

| Section | Modification |
|---|---|
| Serrures, coffres, surveillance | Fournir les **plages** de tirage par monde des niveaux de serrure (par classe de porte et richesse), des coffres et des caméras privées (P12) |
| Lignes de vue, tirs | S'appuyer sur la géométrie **étagée** (segments, plages de hauteur, ouvertures) (P13) |

### 13.11 Revues

Les documents de revue ne sont pas modifiés. La session principale note dans 05 que la revue de la génération **complète** les décisions D35 (navigation dérivée : en cache, P31), D36 (`ts-worldbuild` : graphe d'étapes, Programme, parts, P1, P27, P30), D37 (`WorldSpec` : générateur de plans directeurs pour `Synthetic`, P2) et l'architecture §6.4 (clé de 128 bits, P25), et la revue de sécurité S2 (graine secrète, cache privé, contrôle du paquet, P25, P29, P30) et S33 (part serveur tirée par monde, P12).

---

## 14. Risques résiduels

| Risque | Pourquoi il reste | Suivi |
|---|---|---|
| Plans types trop peu nombreux, intérieurs répétitifs | Le coût d'écriture est humain | Alerte de variété (§9.3) ; bibliothèque enrichie au fil des phases |
| Gabarits morphologiques reconnaissables d'un quartier à l'autre | Même cause | Morphométrie et indice de distinction (§9.8) |
| Réconciliation géométrique ambiguë après une grosse retouche | Un appariement par recouvrement peut se tromper | `ts-worldbuild diff` relu avant publication ; seuils réglables |
| Distributions stationnaires périmées après un changement de règles | Elles sont mesurées sur une version donnée | Provenance enregistrée ; recalcul par `rodage_convergence` dans la boucle de calibrage |
| Empreintes de carte différentes entre Windows et Linux | Les flottants internes peuvent différer malgré la quantification | Alerte en intégration continue ; la construction de publication fait foi |
| Fuite de la graine en ligne par un initié | Même cause que RS28 | Nouvelle version de la famille en ligne pour les nouveaux mondes (S2) |
| Candidats génériques de la part serveur un peu informatifs | Un point de pose « possible » réduit l'espace de recherche | Candidats nombreux et présents partout ; le nombre réel posé est tiré par monde |
| Budget de création d'un monde solo dépassé sur une petite machine | Le rodage propre domine | Écran de progression ; plafond de 20 s ; si le banc le montre nécessaire, la durée du rodage propre (14 jours **[à calibrer]**) est revue par une décision de conception, jamais réduite selon la machine |
