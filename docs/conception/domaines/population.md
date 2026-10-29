# Domaine — population

> Fichier de **domaine** : modifié uniquement par la session « Conception — population » (voir [README](../README.md)).
> Version 0.1 — 29/10/2026 : première rédaction complète.
> Version 0.2 — 29/10/2026 : retour de relecture de la session principale. Demande extérieure abstraite reconnue (aucun visiteur **visible**, [economie.md](economie.md), §4.4) ; demandes de l'économie intégrées (métiers et emplois de nuit, propriété, patrimoines, dettes, entreprises et viabilité, §3.4, §4.7) ; demandes du social intégrées (respect et crainte distincts, groupes et bandes de départ, préférences d'attirance, familles d'accueil, §4.8, §5.3, §5.5) ; impacts sur le socle marqués intégrés.
> Version 0.3 — 29/10/2026 : demandes des domaines combat et interface. Police, justice et armes au départ (§3.5) ; casiers et détenus au départ (§5.1, §5.4) ; morts violentes et aucun décès d'enfant (§7.3) ; succession du commissaire et du procureur (§6.2) ; renforts par de vraies arrivées (§7.4) ; état civil, listes électorales et de jurés, faire-part, avis de décès et annonces de naissance (§7.8).
> Version 0.4 — 29/10/2026 : revues d'équilibrage et de sécurité, selon l'arbitrage A1 de la session principale. Deux populations initiales, solo publique et en ligne jamais livrée au client (§9.1, S2) ; couche cachée tirée par monde (§1.6, E25, S3) ; rodage propre de 14 jours par monde (§1.4, §1.6, E25) ; secrets des fiches d'ancrage et des intrigues écrits comme des possibilités (§5.4, §6.3, §6.4) ; population par serveur comme étape suivante (§9.4).
> Version 0.5 — 29/10/2026 : revue de l'intelligence des PNJ ([ia-pnj.md](ia-pnj.md) v0.8, §11 et §13). État initial des nouveaux composants : voix tirée de l'âge, du milieu, du métier et du profil, l'origine ne donnant que des détails de langue ; pensées, humeur, conclusions, thèmes de vie, attitudes, valeurs apprises, traits supposés, faits sensibles et sujets échangés donnés par la génération puis par le rodage (§5.6, §1.2, §1.3, §1.4, §10.3, §12.3).
> Version 0.6 — 29/10/2026 : revues de la génération (P1 à P41) et des signatures (G13, G16). §1 réécrit : programme en tête de chaîne, chronique, grammaire de ménages, distributions stationnaires ; ordre P1 à P12 (situation actuelle, biographie à rebours, emménagement avec le mobilier mobile) ; IPU et recuit, flots à coût minimal, échantillonneur de parcours, réseau par contextes ; contrôles bloquants ou en alerte et métriques du réseau ; rodage de famille par convergence ; création d'un monde en W1 à W8, avec rodage propre sur les 14 jours qui précèdent le départ et solveur à contraintes. Vacance de 5 % (§3.1) ; affectations par flots (§3.1 à §3.3) ; viabilité par la localisation (§3.4) ; profil déplacé par les effets des événements (§4.1) ; `ts-data check` (§4.6) ; chronique dans les biographies (§5.1) ; méthode et métriques du réseau (§5.3) ; graine en ligne secrète, cache privé, paquet client vérifié (§9.1) ; chaîne embarquable (§9.4) ; impacts au §12.9.
> Version 0.7 — 29/10/2026 : reliquat du social ([social.md](social.md) v0.6, §12.11). Plages cibles du réseau prises dans social §10.2 (§5.3) ; profil normatif des groupes de départ (§5.5) ; profil de valeurs des programmes des candidats (§6.4) ; G14.
> Version 0.7.1 — 29/10/2026 : mise à jour d'états. Revue d'architecture finale intégrée au socle (§12.1) ; étapes W1 à W8 vérifiées dans le socle (§12.9).
> Version 0.8 — 29/10/2026 : demandes de la direction artistique ([direction-artistique.md](direction-artistique.md), §3.2 à §3.4, §3.7 et §12.2). Génome d'apparence de chaque habitant : silhouette d'après l'âge, le sexe et les attributs physiques, visage, peau et cheveux par hérédité limitée, voix sonore, goûts vestimentaires (§4.9) ; vieillissement du génome (§4.9) ; probabilité non nulle de toute combinaison de l'éditeur du persistant (§4.9) ; garde-robe abstraite, complétée à chaque arrivée (§4.10) ; contrôles anti-sosies, dispersion par quartier et test de parenté (§1.3) ; étapes P6, P8 et P9 (§1.2) ; naissances et arrivées (§7.2, §7.4) ; impacts au §12.10.
> Version 0.8.1 — 29/10/2026 : reliquats de l'IA des PNJ ([ia-pnj.md](ia-pnj.md) v1.1, §2.6 et §11), de l'interface ([interface.md](interface.md) v0.6, §15.5) et du combat ([combat.md](combat.md) v0.5, §16.4). État de départ des pièces de la garde-robe (§4.10) ; coiffure et longueur de départ, étalées sur le cycle du coiffeur (§4.9) ; plages et distribution de l'éditeur par âge et par sexe, concordance vérifiée (§4.9) ; **historique daté** de la coiffure, de la tenue habituelle et des signes distinctifs (§4.9) ; cicatrices de départ inscrites dans la biographie (§4.9) ; goûts musicaux, instruments et compétence musicale (§4.11, numéro libre, sans renumérotation) ; impacts au §12.3, §12.6, §12.7 et §12.10.
> Version 0.9 — 30/10/2026 : dernière vague avant la clôture de la conception. Arbitrage A9 : cicatrices de départ selon la règle du combat, ≈ 30 % des blessures graves et ≈ 70 % des critiques (§4.9). Signature graphique de chaque bande et du réseau de départ, et signes de bande dans la garde-robe des membres ([social.md](social.md) v0.7, §6.6, §12.12 ; §4.10, §5.5). Garde-robe agrégée de l'économie (valeur de remplacement et usure de départ), lave-linge du ménage, étiquette « valeur affective » des objets hérités, musiciens de départ ([economie.md](economie.md) v0.7, §3.8, §5.10, §5.13, §14.10 ; §1.2, §4.10, §4.11). Arbitrage A8 : aucune action d'un humain ; fiches d'ancrage, chronique et retouches confiées aux sessions de contenus (§0, §1.1, §1.3, §6.3, §9.1, §9.4). Impacts au §12.11.
> Tous les choix de ce fichier ont été pris par la session de domaine, sur consigne de l'utilisateur : ils sont marqués **(décidé)** et accompagnés de leur raison. Les valeurs marquées **[à calibrer]** seront ajustées par le prototypage. Ce qui touche le socle ou d'autres domaines est regroupé au §12, sans avoir été appliqué ailleurs.

Ce domaine décrit **qui habite Orvenne** : comment les 10 000 habitants sont créés au départ (ménages, logements, emplois, écoles, profils, attributs, relations, histoire), puis comment la population se **renouvelle** (naissances, décès, arrivées, départs) sans perdre son équilibre. Il s'appuie sur le socle ([01-concept.md](../01-concept.md) à [05-architecture.md](../05-architecture.md)), sur [ville.md](ville.md) pour les quartiers, les logements et les postes de travail, et sur [ia-pnj.md](ia-pnj.md) pour la façon dont les habitants vivent une fois créés.

**Partage des rôles** : la ville fournit les **lieux** (logements, postes, écoles) ; la population fournit les **personnes** et les **ménages** qui les occupent ; l'IA des PNJ les fait **vivre** ; l'économie fixe les **montants** (salaires, loyers, patrimoines) ; le social fait **évoluer** les relations que la population a semées.

## Sommaire

0. [Principes](#0-principes)
1. [Le générateur de population](#1-le-générateur-de-population)
2. [Pyramide des âges, ménages et répartition](#2-pyramide-des-âges-ménages-et-répartition)
3. [Logements, emplois, écoles](#3-logements-emplois-écoles)
4. [Profils, attributs, santé, palier et apparence](#4-profils-attributs-santé-palier-et-apparence)
5. [Histoire et relations initiales](#5-histoire-et-relations-initiales)
6. [Personnages notables, hors catégorie et faits à la main](#6-personnages-notables-hors-catégorie-et-faits-à-la-main)
7. [Renouvellement de la population](#7-renouvellement-de-la-population)
8. [Les proches à l'extérieur](#8-les-proches-à-lextérieur)
9. [Mondes différents et versions](#9-mondes-différents-et-versions)
10. [Ce que la population fournit aux autres domaines](#10-ce-que-la-population-fournit-aux-autres-domaines)
11. [Points ouverts](#11-points-ouverts)
12. [Impacts sur le socle et les autres domaines](#12-impacts-sur-le-socle-et-les-autres-domaines)

---

## 0. Principes

1. **Une population générée, puis figée et retouchée** (décidé), comme la ville ([ville.md](ville.md), §4.1) : un générateur produit les 10 000 habitants, des **sessions de contenus**, pilotées par le responsable contenus, retouchent quelques personnages et en écrivent d'autres une à une (§6) (décidé, A8), puis cette **population initiale** est figée et partagée par tous les mondes de sa famille (solo ou en ligne), sauf sa **couche cachée**, tirée par monde (§1.6, §9.1). *Pourquoi : c'est le même principe que la carte, qui a fait ses preuves, et il permettra plus tard une population différente par monde (§9).*
2. **Générer une histoire, pas une photo** (décidé) : chaque habitant reçoit une **biographie** datée (où il a grandi, quand il est arrivé, ses emplois, ses couples, ses enfants, ses deuils), et ses attributs, ses compétences, ses relations et ses souvenirs de départ **découlent** de cette biographie. *Pourquoi : des attributs et des relations tirés indépendamment se contrediraient (un « menuisier » sans compétence de menuiserie, deux époux qui ne se souviennent pas de leur mariage) ; partir de l'histoire garantit la cohérence.*
3. **Aucun habitant n'apparaît ni ne disparaît par magie** (décidé) : une naissance a lieu à l'hôpital, un décès laisse un corps, des obsèques et un logement à vider, un nouvel habitant arrive par une vraie entrée de l'agglomération, un départ passe par une vraie sortie ([ia-pnj.md](ia-pnj.md), §9.1). *Pourquoi : c'est la règle « aucun figurant » appliquée au renouvellement, et c'est ce qui rend les changements visibles et crédibles pour les joueurs.*
4. **Les départs viennent des personnages, les arrivées du monde** (décidé) : un habitant ne part que par un **projet** de son IA ([ia-pnj.md](ia-pnj.md), §9.4) ; le réglage global (§7.6) ne fait que rendre ces projets plus ou moins probables. Les arrivées, elles, sont créées par le générateur, selon ce dont la ville manque. *Pourquoi : un départ décidé de l'extérieur serait une suppression déguisée, alors qu'une arrivée n'a par nature aucune histoire dans la ville.*
5. **L'origine ne détermine jamais le comportement** (décidé) : la population a une diversité réaliste d'origines, de prénoms et de noms de famille, mais l'origine n'entre jamais dans le tirage du profil comportemental, des attributs, du palier ou de l'appartenance à une bande. De même, la **silhouette** ne dépend jamais du milieu ni de l'origine, et la carnation et les traits du visage n'entrent dans aucun calcul du cœur (décidé, §4.9). *Pourquoi : un monde où l'origine prédirait la délinquance serait caricatural et blessant, et n'apporterait rien au jeu.*
6. **Un seul générateur pour tout** (décidé) : la même bibliothèque Rust crée la population initiale, les nouveau-nés, les nouveaux arrivants et, plus tard, des populations entières pour d'autres mondes ou pour des simulations de test. Elle est **déterministe** pour une graine donnée. *Pourquoi : un seul code à maintenir, et des populations reproductibles pour le débogage et l'entraînement des IA, comme l'exige la revue d'architecture prévue (« lancer de nombreuses simulations différentes »).*

---

## 1. Le générateur de population

### 1.1 Entrées et sorties

| Entrée | Source |
|---|---|
| **Programme** (étape G0) : ménages attendus par quartier et par classe, parc de logements, établissements et postes, calculés **sans aucun tirage** à partir des tables cibles ; commun aux deux familles (décidé, P1) | [revues/generation.md](../revues/generation.md), §4.4 |
| **Registre des lieux**, qui réalise le programme : logements (type, surface, pièces, quartier, **année de construction** du bâtiment), postes (métier, horaires, palier typique), écoles, hébergements collectifs | [ville.md](ville.md), §9.1 |
| **Tables cibles** par quartier : pyramide des âges, types de ménages, revenus, diplômes, catégories de métier, statut d'occupation, **vacance**, part d'étudiants (§2, §3). Elles sont la **seule source** de ces chiffres (décidé, P41) | Ce domaine |
| **Grammaire de ménages** : lois conditionnelles écrites à la main (type de ménage → âge de la personne de référence → écart d'âge du couple → nombre et âges des enfants compatibles avec l'âge de la mère → décohabitation, grand-parent hébergé) (décidé, P17) | Ce domaine |
| **Tables de taux** des transitions de vie par âge et par génération : fin d'études, emplois, mises en couple, naissances, séparations, arrivées, deuils (P20) | Ce domaine |
| **Chronique d'Orvenne** : 30 à 50 événements datés **[à calibrer]**, communs à la ville et à la population (construction des Peupliers, crues, fermeture d'un haut-fourneau, plan social de l'usine, rénovation de la Cité, élections passées, titre sportif du club) (décidé, P3) | Sessions de contenus (décidé, A8) ([revues/generation.md](../revues/generation.md), §4.2) |
| **Distributions stationnaires** des variables rapides (besoins, stocks des ménages et des commerces, trésorerie courante, humeur, avancement des projets), mesurées par `ts-sim` sur des runs longs et rangées avec leur provenance (décidé, P22) | Paquet de contenus |
| **Fiches d'ancrage** des personnages faits à la main (§6.3) | Sessions de contenus (décidé, A8) |
| Fiches de rôle, catalogue des activités et catalogue des pensées | [ia-pnj.md](ia-pnj.md), §0 et §13 |
| Catalogue des métiers (avec diplômes exigés), salaires, loyers, prix de l'immobilier, conditions de prêt, part extérieure de la clientèle | [economie.md](economie.md), §1.4, §3.8, §4.4, §4.10, §6.3 |
| **Ensembles de mobilier** (lits, tables, sièges, rangements, électroménager, décor, objets personnels) (P11) | Catalogues du paquet de contenus |
| **Plages du génome d'apparence**, lois de répartition (tailles, corpulences, lunettes, barbes, cheveux gris selon l'âge et le sexe), **catalogue vestimentaire** (pièces sémantiques, couleurs, uniformes, signes de bande) et palettes | [direction-artistique.md](direction-artistique.md), §2.1, §3.2 à §3.4 |
| Modèle des relations (valeurs, couches) et des groupes | [social.md](social.md), §1.1, §1.2, §6.1 |
| **Graines** | Famille solo : publique. Famille en ligne : **secrète, sur 128 bits**, tirée par un générateur cryptographique et gardée en zone 2 (décidé, P25). Chaque monde a en plus sa propre graine, secrète en ligne, pour sa couche cachée (§1.6) |

En sortie : les **personnages** (tous leurs composants : identité, âge, attributs, profil, préférences d'attirance, santé, **génome d'apparence** et coiffure, **garde-robe abstraite**, biographie, mémoire, état intérieur, relations, agenda, rôle, comptes et dettes), les **ménages**, l'occupation et la **propriété** des logements, le **mobilier mobile** de chaque logement occupé (décidé, P11), les **entreprises** et leurs propriétaires, l'occupation des postes et des places d'école, les **groupes** de départ (associations, clubs, bandes) et les **proches à l'extérieur** (§8). C'est la **population visible** ; la **couche cachée** est produite à part, à la création de chaque monde (§1.6).

### 1.2 Étapes (décidé, P18)

*Pourquoi cet ordre : on fixe d'abord la **situation actuelle** de chacun (ménage, logement, diplôme, emploi, école), puis on écrit sa **biographie à rebours** depuis cette situation ; la boucle entre biographie et emploi disparaît et les dates sont cohérentes par construction ([revues/generation.md](../revues/generation.md), §4.5, P18). Les fiches d'ancrage passent en premier pour que le reste se construise autour d'elles.*

```
 G0 PROGRAMME       (chaîne de la ville) ménages attendus → logements → établissements → postes, sans tirage
 P1 ANCRAGES        placer les fiches d'ancrage (§6.3) dans leurs logements et leurs postes, avec leurs contraintes
 P2 MÉNAGES         type, âges, liens, revenu, statut d'occupation, diplôme et catégorie de métier des adultes (§2)
 P3 SITUATION       logement de chaque ménage, emploi de chaque actif, école de chaque enfant, hébergement collectif (§3.1 à §3.3)
 P4 LIGNÉES         parenté entre ménages, familles étendues, proches à l'extérieur (§5.2, §8)
 P5 BIOGRAPHIES     vie datée à rebours depuis la situation actuelle, avec la chronique (§5.1)
 P6 PERSONNES       profil, attirance, chronotype, attributs, compétences, santé, voix, génome d'apparence et son historique, goûts musicaux (§4, §4.9, §4.11, §5.6)
 P7 PATRIMOINES     comptes, biens, dettes, propriétaires des logements loués et des entreprises, viabilité (§3.1, §3.4, §4.7)
 P8 EMMÉNAGEMENT    mobilier mobile, décor, objets personnels, instruments et garde-robes de chaque logement occupé (§4.10, §4.11)
 P9 RELATIONS       réseau initial, groupes et bandes (avec leurs signes dans les garde-robes), souvenirs, état intérieur (§4.10, §5.3 à §5.6)
P10 PUISSANCE       influence, niveau, puissance, rang, palier ; ajustement des rôles mal classés (§4.5)
P11 RODAGE          rodage de famille, initialisé à l'équilibre, durée par convergence (§1.4)
P12 VALIDATION      contrôles bloquants et alertes (§1.3)
```

**Méthodes** (décidé) :

| Étape | Méthode | Raison |
|---|---|---|
| **P2 Ménages** (P17) | La **grammaire de ménages** produit un échantillon de quelques dizaines de milliers de ménages candidats par quartier. L'**IPU** (ajustement proportionnel itératif aux deux niveaux, ménages et personnes) leur donne des poids qui tiennent les marges du quartier. Les poids sont convertis en **entiers** (tirage tronqué-répliqué), puis un **recuit** borné échange des ménages pour annuler les écarts restants et tenir **exactement** le nombre de ménages par classe de logement du programme | Tient les marges aux deux niveaux et les entiers exacts, en quelques secondes |
| **P3 Situation** (P19) | Trois **flots à coût minimal** à coûts entiers, quartier par quartier quand c'est possible : ménages → logements, actifs → postes, enfants → écoles (§3.1 à §3.3). Les égalités de coût sont départagées par `StableId` | Optimum global en une passe, sans boucle de corrections, reproductible au bit près |
| **P5 Biographies** (P20) | **Échantillonneur de parcours** à rebours sur les tables de taux, conditionné par ce qui est déjà fixé (âge des enfants présents, ancienneté dans le poste, année de construction du logement) et par la chronique (§5.1) | Dates cohérentes par construction |
| **P6 Génome d'apparence** | Silhouette calculée d'après l'âge, le sexe et les attributs physiques ; visage, peau et cheveux tirés par **hérédité limitée**, des plus âgés aux plus jeunes de chaque lignée pour que les parents soient faits avant leurs enfants ; signes distinctifs lus dans la biographie (§4.9) | Une seule passe, sans correction : le contrôle anti-sosies ne refuse qu'un tirage sur plusieurs milliers **[à mesurer]**, et un refus fait seulement retirer le visage concerné |
| **P8 Emménagement** (P11) | Ensembles de mobilier choisis selon le ménage (taille, âges, enfants, richesse, goûts tirés du profil et de l'époque) et la pièce, placés par règles (contre les murs, passages et débattement des portes libres) | Le mobilier dépend des occupants ; il fait partie de la population, pas de la carte |
| **P9 Relations** (P21) | **Contextes partagés**, **homophilie** et **fermeture triadique**, avec des cibles mesurées (§5.3) | Un réseau crédible et mesurable, en O(N·k) |
| **P10 Puissance** | Comme au §4.5, au plus **3 passes** **[à calibrer]** | Les rôles mal classés deviennent rares grâce aux flots |

- **Corrections rares** (décidé) : grâce au programme, à la localisation des établissements par aires de chalandise (P9 de la revue) et aux flots, les boucles de correction (échanger deux personnes entre deux postes, corriger une entreprise) deviennent l'exception ; elles restent possibles aux étapes P7 et P10.
- **Reproductibilité et parallélisme** (décidé, P26) : tous les tirages passent par `draw(graine de famille, Population, sujet, …)` ; les étapes travaillent par unité (quartier, ménage) en fonctions pures, avec une collecte ordonnée. Le résultat ne dépend pas du nombre de fils.
- **Cache** (décidé, P27) : chaque étape est adressée par l'empreinte de ses entrées ; on ne régénère que ce qui change.
- **Coût** : environ 30 secondes par famille sur la machine de construction de référence, plus le rodage (≤ 60 s) **[à mesurer]** ([revues/generation.md](../revues/generation.md), §4.5).
- **Lave-linge et valeur affective** (décidé, [economie.md](economie.md), §5.10, §5.13) : à l'étape P8, l'équipement en **lave-linge** est tiré par classe ; les studios, le foyer de travailleurs et une partie du parc social n'en ont pas, et leurs ménages lavent à la laverie. Les **objets hérités** d'un parent mort dans la biographie (alliance, montre, bijou, médaille) et les **cadeaux marquants** (cadeau de mariage, photo encadrée d'un disparu) reçoivent l'étiquette **valeur affective**, liée à la personne et à l'événement de la biographie dont ils viennent.
- **Mobilier en cours de partie** : il **déménage avec le ménage** (cartons et camion, §7.4, §7.5), et un nouvel arrivant reçoit le sien à son emménagement.

Ces étapes produisent la **population visible** d'une famille de mondes, solo ou en ligne (§9.1). Les secrets n'y sont pas : ils sont tirés à la création de chaque monde (§1.6).

### 1.3 Validation automatique (décidé)

*Pourquoi : c'est la condition pour pouvoir un jour générer une population par serveur ou par monde sans aucune retouche, comme pour la ville ([ville.md](ville.md), §4.6).*

Chaque contrôle est **bloquant** ou **en alerte** (décidé, P37) ; sauf mention « alerte », il est bloquant. Les résultats et les métriques sont écrits dans le **rapport** de la construction ([revues/generation.md](../revues/generation.md), §9).

| Contrôle | Règle **[seuils à calibrer]** |
|---|---|
| Effectifs et marges | Population, pyramide, types de ménages et habitants par quartier : écart aux marges ≤ 3 % (IPU) ; nombres **exacts** de ménages par classe de logement du programme (P17) |
| Ménages | Chaque mineur vit avec au moins un adulte responsable ; chaque ménage a un logement ou une place en hébergement collectif |
| Logements | Vacance de **5 %** à la génération, soit ≈ 4 740 logements pour ≈ 4 500 ménages (P41) ; pièces par personne dans les bornes de chaque quartier ; loyer ou mensualité ≤ 35 % du revenu, sauf ménages en difficulté voulus |
| Biographies | Dates cohérentes : âge des parents à la naissance, études avant le premier emploi, aucun événement avant la naissance ou après un décès, ancienneté compatible avec l'âge |
| Absurdités | Écarts d'âge des couples et des parents dans des bornes ; aucun événement de biographie avant la construction du bâtiment concerné ; aucun enseignant sans diplôme ; aucun mineur employé hors apprentissage ; aucune personne âgée dépendante seule sans aide |
| Qualification | Aucun titulaire d'un poste sans le diplôme ou l'expérience exigés par la fiche de métier |
| Emplois | Postes occupés ≈ 4 100, postes vacants ≈ 2 à 3 % ; ≈ 12 % d'emplois de nuit pourvus ; chômage entre 7 et 12 % ; chaque poste « indispensable » (chauffeur de bus, médecin de garde, policier de nuit) pourvu ; trajets domicile-travail raisonnables |
| Propriété | ≈ la moitié des ménages propriétaires occupants ; ≈ 15 % des biens à des non-résidents ; logement social à Orvenne Habitat ; chaque bien et chaque entreprise a un propriétaire |
| Patrimoines et dettes | Médiane des adultes ≈ 50 000 écus ; distribution conforme au §4.7 ; mensualités des prêts compatibles avec les revenus |
| Entreprises | Chaque entreprise a une **marge attendue positive** avec sa clientèle réelle et sa part extérieure ([economie.md](economie.md), §4.4) |
| Mobilier mobile | Chaque logement occupé est meublé selon son ménage (autant de couchages que d'occupants) |
| Groupes | Chaque bande a un chef, une hiérarchie et un territoire ; chaque association a un bureau (§5.5) |
| Police et justice | Les ≈ 80 postes du §3.5 pourvus ; aucun casier non effacé chez les policiers, magistrats et greffiers ; aucun détenteur de permis de chasse avec un casier incompatible |
| Mineurs | Aucune table de mortalité non nulle pour un mineur (§7.3) ; aucune préférence d'attirance avant 18 ans (§4.8) |
| Écoles | Chaque enfant en âge scolaire a une place ; effectifs par classe plausibles |
| Paliers | Pour chaque rôle, au moins 80 % des personnes dans la fourchette de palier attendue ± 1 (§4.5) |
| Couverture des catégories | Pour chaque catégorie comportementale et chaque groupe de paliers, un minimum de personnages jouables (§4.6), et **au moins un candidat dans chaque secteur habité** **[à calibrer]** |
| Relations | Liens de parenté réciproques ; aucun adulte sans relation, sauf les isolés voulus ; souvenirs partagés présents chez tous les participants (§5.4) ; métriques du réseau dans leurs plages (§5.3, P21) |
| Chronique (**alerte**) | Chaque événement de la chronique qui doit avoir des témoins en a (§5.1) |
| Hors catégorie | Leur nombre reste proche de 0,1 % de la population (§6.2) |
| État intérieur | Chaque habitant a une voix ; aucun composant au-delà de ses bornes ; chaque pensée et chaque thème de vie a une cause dans la biographie ou les souvenirs ; après rodage, la part des habitants d'humeur sombre et le nombre de crises restent dans des fourchettes plausibles (§5.6) |
| **Anti-sosies** | Aucune paire d'habitants non jumeaux monozygotes dont les visages sont plus proches qu'un seuil de distance **[à calibrer]** ; les jumeaux monozygotes partagent leur visage par construction (§4.9) |
| Dispersion des apparences (**alerte**) | Dans chaque quartier, dispersion minimale des silhouettes, des coiffures et des garde-robes, et aucune tenue de sortie ou du quotidien identique entre deux habitants hors uniformes **[seuils à calibrer]** ([direction-artistique.md](direction-artistique.md), §3.4) |
| **Test de parenté** | Un classificateur qui ne lit que les **apparences perçues** (les trois précisions de [direction-artistique.md](direction-artistique.md), §3.5) ne retrouve les liens de parenté qu'avec une aire sous la courbe **≤ 0,70** **[à calibrer]**, mesurée sur des paires parents-enfants et frères-sœurs contre des paires tirées au hasard dans le même quartier et la même tranche d'âge. Le seuil est fixé par ce domaine et ajusté par `ts-sim`, sans panel humain (décidé, A8) |
| Apparence et éditeur | Chaque génome est dans les plages de l'éditeur du persistant ; chaque habitant a une tenue complète pour chaque usage ; chaque poste à uniforme a son uniforme (§4.9, §4.10) ; chaque signe distinctif a une date et une cause dans la biographie ; les longueurs de coiffure de départ sont étalées sur le cycle du coiffeur (§4.9) ; chaque joueur d'instrument possède son instrument ou en a l'usage (§4.11) |
| Deux populations | Les populations solo et en ligne passent toutes deux l'ensemble de ces contrôles (§9.1) |
| Couche cachée (à la création de chaque monde) | Chaque élément du §1.6 est rempli ; chacun a au moins 3 candidats dans la population visible ; contraintes d'exclusion respectées ; nombres d'informateurs, de policiers corruptibles, de caches et de secrets d'intrigue dans leurs fourchettes ; aucun élément ne contredit la population visible (casier, profil, rôle) ; aucun élément ne modifie un génome d'apparence (§4.9) |

### 1.4 Rodage de famille (décidé)

*Pourquoi : une population tout juste générée a des agendas et des relations « à zéro » ; quelques semaines simulées les mettent en mouvement (disputes en cours, habitudes prises) et vérifient que la ville tient debout avant l'arrivée des joueurs.*

- **Initialisation à l'équilibre** (décidé, P22) : les variables rapides (besoins, stocks des ménages et des commerces, trésorerie courante, humeur, avancement des projets en cours) sont tirées dans leurs **distributions stationnaires** (§1.1). Le rodage devient une **mise en mouvement** et une **vérification**, plus une construction.
- **Durée par convergence** (décidé, P22) : on simule en **statistique**, avec les règles normales de l'IA ([ia-pnj.md](ia-pnj.md), §6), jusqu'à ce que, sur deux fenêtres de 7 jours consécutives, les indicateurs de stabilité (taux d'emploi, faillites, faim, humeur, trésorerie, relations) varient de moins d'un seuil **[à calibrer]**, avec un **minimum de 14 jours** et un **plafond de 60**. La durée atteinte est écrite dans le rapport ; elle reste déterministe : même graine, même durée.
- Le rodage ne crée ni ne supprime personne : naissances, décès, arrivées et départs sont **suspendus** pendant cette phase.
- L'économie tourne normalement (salaires, loyers, achats, stocks, échéances de prêts) : c'est la vérification réelle de la viabilité des entreprises et des budgets des ménages.
- Le rodage fait aussi vivre l'**état intérieur** (décidé, [ia-pnj.md](ia-pnj.md), §13) : pensées, humeur, réflexions nocturnes, conclusions et valeurs apprises évoluent selon les règles de l'IA, à partir de l'état donné par la génération (§5.6).
- **Contrôles de stabilité** : pas de vague de licenciements, de faillites, de faim ou de ruine, pas de mélancolie collective ni de vague de crises. Un indicateur hors de sa plage donne une **alerte** ; au-delà du double de la plage, le contrôle est **bloquant** et renvoie au calibrage (P22).
- Ce rodage se fait **sans la couche cachée** (décidé, E25) : les activités qui en dépendent (payer une dette secrète, retrouver une liaison, renseigner un policier) ne tournent pas, et les stocks des bandes sont comptés au niveau du groupe, sans planque précise. *Pourquoi : la population visible est commune à tous les mondes d'une famille ; la trace d'un secret dans un compte ou un agenda le livrerait à tous.*
- L'état obtenu, avec ses dates exprimées par rapport au jour de départ (§9.2), devient la population initiale versionnée de sa famille (solo ou en ligne).
- Chaque monde fait en plus, à sa création, un **rodage propre** sur les 14 jours qui précèdent son départ (§1.6 ; E25, P23).

### 1.5 Mode « morceau de ville » (décidé)

*Pourquoi : le premier prototype ne simule qu'un quartier ([05-architecture.md](../05-architecture.md), §5) ; il doit pourtant avoir des habitants crédibles, issus du même générateur.*

Le générateur peut produire la population d'un **secteur seulement** (par exemple le morceau de faubourg Saint-Roch, de Pont-Vieux et de la Cité du prototype). Il part du **programme restreint au secteur** (décidé, P1) : ménages attendus, logements, établissements et postes du secteur, puis il applique les mêmes étapes. Les postes, écoles et proches situés hors du secteur sont traités comme **à l'extérieur** du prototype. Le même mode sert aux simulations de test de petite taille. La couche cachée (§1.6) n'y est tirée que pour le secteur.

### 1.6 Création d'un monde : couche cachée et rodage propre (décidé)

*Pourquoi : ce qu'on apprend en explorant une partie solo, ou un autre serveur, ne doit révéler les secrets d'aucun monde ([revues/equilibrage.md](../revues/equilibrage.md), E25 ; [revues/securite.md](../revues/securite.md), S3). Tirer les secrets par monde coûte une seule étape déterministe à la création.*

À la création d'un monde (partie solo, serveur ou `ts-sim`), le processus du monde part de la population visible de sa famille (§9.1) et applique, avec la **graine du monde** (secrète en ligne, 128 bits, P25) ([revues/generation.md](../revues/generation.md), §4.6) :

```
W1 CHARGEMENT      parts de l'artefact vérifiées par empreinte
W2 PART SERVEUR    de la ville (serrures, caméras, coffres, alarmes reliées, emplacement des caches), tirée par la ville (P12)
W3 COUCHE CACHÉE   secrets de la population, tirés par un petit solveur à contraintes parmi les candidats visibles (P24)
W4 TRACES          leurs conséquences dans les personnages : souvenirs, relations secrètes, savoirs, dettes informelles, état intérieur (§5.6)
W5 VALIDATION      contrôles de la couche cachée (§1.3) ; en cas d'échec, tentative suivante (P24)
W6 RODAGE PROPRE   les 14 jours du calendrier qui précèdent le départ, avec la météo et les vacances du monde (P23)
W7 DATES           conversion au calendrier du monde (§9.2)
W8 INSTANTANÉ      premier instantané, sur un fil de fond ; le jeu démarre sans l'attendre
```

**Contenu de la couche cachée** (décidé, E25, S3) :

| Élément | Tiré parmi |
|---|---|
| **Détenteurs des savoirs opaques** (codes, combinaisons, emplacements) | Les personnes dont le rôle peut y donner accès (employés de banque, gérants, membres d'une bande). Le savoir reste une **référence** sans valeur à extraire (« connaît le code du coffre n° 123 », [ia-pnj.md](ia-pnj.md), §4.4) |
| **Qui connaît les caches** (dont les armes illégales, §3.5) | Les membres de la bande ou les usagers du lieu ; l'**emplacement** des caches est tiré à W2 avec la part serveur de la ville (P12) |
| **Contenu des planques et des coffres** | Des fourchettes réalistes, fixées par l'économie et le combat |
| **Informateurs** de la police et leur policier traitant | Les membres du milieu dont le profil le permet ; les policiers de la sûreté et de la brigade anticriminalité (§3.5) |
| **Policiers corruptibles** | Les policiers dont le profil le permet : respect des règles moyen ou bas (§3.5) |
| **Liaisons et dettes secrètes** des intrigues de départ | Les couples, notables et commerçants compatibles avec l'intrigue (§6.4) |
| **Secrets des fiches d'ancrage** | La liste de secrets possibles de chaque fiche (§6.3) |
| Autres **souvenirs secrets** sans trace visible (vol jamais découvert, dette honteuse) | Les personnes dont la biographie et le profil s'y prêtent (§5.4) |

- **Solveur à contraintes** (décidé, P24) : chaque élément est choisi parmi ses candidats par des tirages à clé sans remise, sous des contraintes d'**exclusion** (un informateur n'est ni le chef de sa bande ni un membre de son ménage ; un policier corruptible n'est pas le traitant de son propre informateur) et de **cohérence** avec la population visible et la part serveur de la ville.
- **Tentatives numérotées** (décidé, P24) : en cas d'échec de validation, on passe à la tentative suivante dans le même flux d'aléa (au plus **8** **[à calibrer]**). Au-delà, la création du monde **échoue** avec une erreur qui désigne le candidat manquant. *Pourquoi : un échec révèle un défaut de l'artefact, à corriger dans la chaîne, pas un aléa à cacher.*
- **Toujours plusieurs candidats** (décidé, E25) : chaque élément est tiré parmi au moins **3 candidats** **[à calibrer]** de la population visible. *Pourquoi : si un seul habitant pouvait être l'informateur, la population visible, lisible en solo, le désignerait.*
- **Aucune trace dans le visible** (décidé, E25) : rien de la population visible (profil, rôle, comptes, casier, relations connues, agenda) ne dépend de la couche cachée. Ce qui se voit d'un secret (un retrait d'argent, un rendez-vous discret) n'apparaît qu'avec le rodage propre, puis la vie du monde.
- **Ce qui reste visible** : le nombre de bandes, leurs membres et leurs territoires ; l'existence des coffres, des caches et des alarmes ; les rôles et les profils ; les intrigues publiques (§6.4). Un joueur solo sait qu'il y a des policiers corruptibles et des informateurs, pas lesquels.
- **Rodage propre** (décidé, E25, P23) : il simule en statistique les **14 jours du calendrier qui précèdent** la date de départ du monde **[à calibrer]**, avec la météo et les vacances scolaires de ce monde, aux mêmes règles que le §1.4 (personne n'est créé ni supprimé, l'économie tourne), mais avec la couche cachée en marche ; agendas, relations et stocks divergent d'un monde à l'autre. La population de famille représente donc l'état **14 jours avant le départ** : chaque habitant a le même âge le premier jour dans tous les mondes de sa famille (§9.2). *Pourquoi : un monde lancé en juillet a des habitants en rythme de juillet.*
- **Coût** : la création d'un monde prend au plus 10 secondes en solo sur 4 cœurs (plafond 20 s), dont ≤ 5 s de rodage propre **[à mesurer]** ([revues/generation.md](../revues/generation.md), §4.6).
- **Déterminisme** (décidé, S3) : même graine de monde, même couche cachée ; l'étape est testée par graine. La couche cachée est enregistrée avec l'état du monde, dans la base du serveur ou dans la sauvegarde pour une partie solo.
- **Solo** : un joueur solo peut lire les secrets de **sa** partie dans sa sauvegarde ; cela ne révèle rien d'une autre partie ni d'un serveur (risque accepté : il ne triche que contre lui-même). Une nouvelle partie solo tire une nouvelle couche cachée.

---

## 2. Pyramide des âges, ménages et répartition

### 2.1 Pyramide des âges (décidé)

*Pourquoi : c'est la structure d'une ville moyenne européenne d'aujourd'hui, légèrement rajeunie par le campus, avec les 18 % d'enfants fixés par la ville.*

| Tranche | Part | Habitants | Remarque |
|---|---|---|---|
| 0 – 2 ans | 3 % | ≈ 300 | Crèche, assistante maternelle ou parent au foyer |
| 3 – 10 ans | 8 % | ≈ 800 | École maternelle et élémentaire |
| 11 – 14 ans | 4 % | ≈ 400 | Collège |
| 15 – 17 ans | 3 % | ≈ 300 | Lycée |
| **Enfants (non jouables)** | **18 %** | **≈ 1 800** | |
| 18 – 24 ans | 9 % | ≈ 900 | Dont ≈ 400 étudiants de l'antenne universitaire |
| 25 – 39 ans | 20 % | ≈ 2 000 | |
| 40 – 54 ans | 20 % | ≈ 2 000 | |
| 55 – 64 ans | 13 % | ≈ 1 300 | |
| 65 – 79 ans | 14 % | ≈ 1 400 | |
| 80 ans et plus | 6 % | ≈ 600 | |

Toutes les parts sont **[à calibrer]**. La répartition entre femmes et hommes est d'environ moitié-moitié, avec plus de femmes aux âges élevés.

### 2.2 Ménages (décidé)

*Pourquoi : ces proportions sont celles d'une ville moyenne européenne et donnent la moyenne de 2,2 personnes par ménage retenue par la ville ([ville.md](ville.md), §2.3).*

| Type de ménage | Ménages | Taille moyenne | Personnes |
|---|---|---|---|
| Personne seule | ≈ 1 650 | 1 | ≈ 1 650 |
| Couple sans enfant au foyer | ≈ 1 150 | 2 | ≈ 2 300 |
| Couple avec enfant(s) | ≈ 1 080 | 3,8 | ≈ 4 100 |
| Famille monoparentale | ≈ 400 | 2,5 | ≈ 1 000 |
| Autres (colocation, plusieurs générations sous un même toit) | ≈ 220 | 2,8 | ≈ 620 |
| **Total des ménages ordinaires** | **≈ 4 500** | **2,15** | **≈ 9 670** |
| Hébergement collectif (§3.1) | — | — | ≈ 330 |

Toutes les valeurs sont **[à calibrer]**. Compléments (décidé) :

- **Méthode** (décidé, P17) : les ménages sont tirés par la grammaire de ménages, l'IPU, la conversion en entiers et le recuit (§1.2), **sous la contrainte du parc de logements** du programme : chaque quartier reçoit exactement le nombre de ménages par classe de logement que le programme prévoit. *Pourquoi : la carte est commune aux deux familles, chaque famille doit donc remplir le même parc.*

- **Couples** : des couples de sexes différents et de même sexe (≈ 3 % des couples **[à calibrer]**), mariés, pacsés ou en union libre. *Pourquoi : c'est la réalité d'une ville européenne contemporaine.*
- **Familles recomposées** : environ un dixième des familles avec enfants ; un enfant peut avoir un **parent hors du ménage**, en ville (garde alternée : l'enfant a **deux domiciles** et son agenda alterne) ou à l'extérieur (§8). *Pourquoi : la garde alternée crée des trajets et des obligations qui font vivre les familles.*
- **Jeunes adultes chez leurs parents** : environ la moitié des 18–24 ans qui ne sont pas en résidence étudiante.
- **Personne âgée chez ses enfants** : quelques ménages de plusieurs générations, surtout aux Forges et aux Peupliers.

### 2.3 Répartition par quartier (décidé)

Les effectifs sont ceux de la ville ([ville.md](ville.md), §3.1) ; le générateur y applique les tendances suivantes. *Pourquoi : elles découlent directement de l'identité des quartiers décrite par la ville, et de la répartition des paliers par lieu de résidence ([ville.md](ville.md), §3.2).*

| Quartier | Habitants | Âges | Ménages | Revenus et paliers | Particularités |
|---|---|---|---|---|---|
| **La Cité** (vieux centre) | ≈ 1 100 | Jeunes actifs et personnes âgées | Beaucoup de personnes seules et de couples sans enfant | Mélangés ; paliers 1 à 6 | Commerçants logés au-dessus de leur boutique |
| **Quartier de la Gare** | ≈ 600 | Actifs | Personnes seules, couples | Moyens à aisés | Peu de familles ; logements de fonction (gardien, hôtelier) |
| **Les Tilleuls** (bourgeois) | ≈ 900 | Familles établies, retraités aisés | Couples avec enfants, couples âgés | Aisés ; paliers 3 à 8 | Professions libérales, cadres, quelques notables |
| **Beaulieu** (hôpital, campus) | ≈ 700 | Très jeune (étudiants) et soignants | Étudiants en résidence ou en colocation | Modestes à moyens | Résidences étudiantes, **maison de retraite** (§3.1), horaires décalés |
| **Faubourg Saint-Roch** | ≈ 1 300 | Mélangé, jeunes adultes | Tous types | Modestes ; paliers 0 à 4, quelques 5 à 8 par les moyens | Artisans, **foyer de travailleurs**, petite délinquance, figures de quartier |
| **Les Quais** | ≈ 300 | Adultes | Personnes seules | Très modestes | Mariniers, gardiens, **squats et sans-abri** (§3.1), quelques lofts |
| **Les Peupliers** (grand ensemble) | ≈ 1 600 | Le plus jeune : beaucoup d'enfants | Familles nombreuses, monoparentales | Modestes ; paliers 0 à 3, quelques 6 à 9 par les moyens | Logement social ; bandes ; forte vie associative |
| **Hautevigne** | ≈ 1 500 | Plutôt âgé, familles aisées | Couples, familles | Aisés à très riches ; la plupart des paliers 8 à 11 et des hors catégorie par la richesse ou l'autorité | Villas, résidences fermées, lycée privé |
| **Forges-sur-Ostrelle** | ≈ 2 000 | Mélangé, familles | Couples avec enfants, plusieurs générations | Ouvriers et employés ; paliers 0 à 4, cadres de l'usine en pavillon | Familles enracinées depuis des générations (§5.2) |

### 2.4 Nom, genre, origine (décidé)

- **Prénoms** tirés selon l'**année de naissance** (les prénoms à la mode changent d'une génération à l'autre), le milieu social et l'origine familiale ; **noms de famille** partagés par la lignée (§5.2), avec les usages courants (nom d'usage après mariage, double nom). *Pourquoi : on reconnaît l'âge et le milieu d'un personnage à son prénom, ce qui rend la ville crédible sans rien afficher.*
- **Origines** : majorité de familles de la région, part réaliste de familles venues d'autres régions ou d'autres pays, d'arrivée ancienne ou récente **[parts à calibrer]**. Rappel du principe 5 (§0) : l'origine n'influence ni le profil, ni les attributs, ni le palier, ni la criminalité.
- **Métiers et sexe** : la répartition des métiers entre femmes et hommes est un peu **plus équilibrée que la réalité** et **jamais exclusive**. *Pourquoi : rester crédible sans caricature, et laisser à tout joueur des personnages de tout genre dans tous les métiers.*
- **Langues** : tout le monde parle français ; certains parlent aussi une autre langue (savoir utilisable en jeu).

---

## 3. Logements, emplois, écoles

### 3.1 Logements et hébergement collectif (décidé)

*Pourquoi : faire correspondre la taille du ménage à celle du logement, avec un peu de jeu, est ce qui distingue une ville crédible d'une répartition au hasard.*

- Chaque ménage est logé dans un logement du **registre des lieux** ([ville.md](ville.md), §9.1) de son quartier, de taille adaptée (une pièce par personne environ, avec une marge de sous-occupation chez les retraités et de surpeuplement léger dans les quartiers modestes).
- **Affectation par flot à coût minimal** (décidé, P19) : le coût d'un couple ménage–logement, en entiers, compte l'écart de taille (pièces par personne), l'écart entre loyer ou prix et revenu (taux d'effort), le statut d'occupation et l'ancienneté souhaitée ; un ménage ne quitte son quartier que si le programme le prévoit. Les égalités sont départagées par `StableId`.
- **Vacance** (décidé, P41) : **5 %** à la génération, soit ≈ 4 500 logements occupés sur ≈ 4 740 et ≈ 240 logements vacants **[à calibrer]**, répartis dans tous les quartiers, pour les arrivées, les déménagements et les personnages persistants. Le chiffre vient des tables cibles, seule source, et c'est le programme qui dimensionne le parc ; le régulateur garde ensuite sa cible de 4 à 5 % (§7.6). *Pourquoi : deux valeurs pour une même grandeur, ici et dans [ville.md](ville.md), §2.3, finiraient par diverger.*
- **Propriété des ≈ 4 740 logements** (décidé, à la demande de [economie.md](economie.md), §5.1 et §14.4 ; *pourquoi : ce sont les parts retenues par l'économie, réalistes pour une ville moyenne où le grand ensemble est social*) **[à calibrer]** :

| Propriétaire | Logements | Où |
|---|---|---|
| **Propriétaires occupants** | ≈ 2 250 (la moitié des ménages) | Surtout Hautevigne, Tilleuls, pavillons des Forges ; peu aux Peupliers et à Beaulieu |
| **Orvenne Habitat** (logement social) | ≈ 950 | Presque tous les Peupliers, une partie des cités des Forges |
| **Propriétaires non résidents** (investisseurs extérieurs, gérés par l'agence immobilière) | ≈ 700 (≈ 15 %) | Surtout la Cité, la Gare, Beaulieu (petits logements loués) |
| **Habitants bailleurs** | ≈ 800, possédés par ≈ 450 ménages (≈ 10 %) | Partout, surtout la Cité et le faubourg |
| Entreprises et employeurs (logements de fonction) | Quelques dizaines | Gardiens, concierges, logement de l'hôtelier |

Les ≈ 240 logements vacants sont répartis entre ces propriétaires. Les **locaux professionnels** suivent la même logique : possédés par l'entreprise qui les occupe, par un habitant bailleur ou par un non-résident. Les montants (loyers, prix) viennent de l'économie ([economie.md](economie.md), §1.4, §6.3).
- **Statut d'occupation** qui en découle : propriétaire occupant, locataire du parc privé, locataire d'Orvenne Habitat, logé par son employeur, hébergé chez un proche. La date d'achat ou d'entrée dans les lieux est une étape de la biographie (§5.1).
- **Ancienneté dans le logement** : tirée selon l'âge et le type de ménage ; elle règle la force des liens de voisinage (§5.3).

**Hébergement collectif** (≈ 330 personnes, **[à calibrer]**) :

| Lieu | Places occupées | Habitants |
|---|---|---|
| Résidences étudiantes (Beaulieu) | ≈ 190 | Étudiants, renouvelés chaque année en partie (§7.5) |
| Foyer de travailleurs (faubourg Saint-Roch) | ≈ 60 | Travailleurs seuls, souvent âgés ou en transition ; quelques chambres gardées libres pour les nouveaux venus |
| Maison de retraite (Beaulieu, près de l'hôpital) | ≈ 70 | Personnes très âgées dépendantes ; lieu ajouté à la demande de ce domaine (§12) |
| Squats et rue (les Quais) | ≈ 10 | Sans-abri et occupants de squats |

*Pourquoi les sans-abri* : la ville prévoit des squats ([ville.md](ville.md), §3.1) et aucun figurant ne peut les occuper ; une poignée d'habitants sans domicile est réaliste et donne des rôles et des histoires (entraide, petite délinquance, réinsertion).

### 3.2 Emplois (décidé)

*Pourquoi : un taux d'emploi réaliste et une affectation par compatibilité donnent une main-d'œuvre crédible, tout en laissant des postes libres aux nouveaux venus et aux joueurs.*

**Ordres de grandeur** **[à calibrer]** :

| Groupe | Effectif | Remarque |
|---|---|---|
| Actifs occupés | ≈ 4 250 | ≈ 76 % des 25–64 ans, une partie des 18–24 ans, quelques plus de 65 ans |
| … dont sur les postes de l'agglomération | **≈ 4 100** | Les postes de la ville sont tous occupés par des habitants : il n'y a pas de travailleurs venus de l'extérieur, puisqu'il n'y a pas de figurants |
| … dont **navetteurs sortants** | ≈ 150 | Travaillent hors de la carte ; partent chaque jour par le train ou la route et sont « à l'extérieur » pendant leurs heures de travail ([ia-pnj.md](ia-pnj.md), §9) |
| Chômeurs | ≈ 370 | ≈ 8 % des actifs, dans la fourchette de 7 à 12 % demandée par l'économie ([economie.md](economie.md), §14.4) ; davantage aux Peupliers et aux Forges |
| Étudiants | ≈ 400 | Certains avec un petit emploi |
| Retraités | ≈ 2 100 | Pensions venues de l'extérieur (économie) |
| Autres inactifs | ≈ 900 | Au foyer, invalidité, longue maladie, sans domicile |
| Postes vacants | ≈ 100 (2 à 3 %) | Pour les arrivées, les jeunes qui entrent dans la vie active et les personnages persistants |

**Affectation** (étape P3 du §1.2, décidé) :

- Les **métiers** sont ceux du catalogue de l'économie ([economie.md](economie.md), §3.8), avec leur palier typique, leur contrat courant et leurs horaires ; leur nombre par secteur découle des établissements du registre.
- Chaque poste du registre ([ville.md](ville.md), §9.1) indique un **métier**, des horaires et un **palier typique**.
- **Flot à coût minimal** (décidé, P19) : le coût d'un couple actif–poste, en entiers, compte l'écart de qualification (diplôme et catégorie de métier tirés avec le ménage, §2.2), le temps de trajet domicile-travail par le mode du ménage, la compatibilité du profil avec le métier et le chronotype pour les postes de nuit. Chaque actif n'a pour candidats que les **50 postes compatibles les plus proches** **[à calibrer]**, pour garder un graphe creux ; les égalités sont départagées par `StableId`. *Pourquoi : l'optimum global en une passe remplace les échanges et corrections successifs, et reste reproductible au bit près.*
- On pourvoit d'abord les postes **rares** (médecin, chirurgien, pilote de la navette, directeur), puis les postes **indispensables** (horaires de nuit compris), puis les autres, par paliers de coût.
- **Le profil est tiré avant le métier** (§4.1) : les gens choisissent un métier qui leur convient, plus ou moins bien. Le métier n'impose pas le profil, il le **favorise** (un policier est plus souvent légaliste, mais pas toujours). Exception : les rôles des fiches d'ancrage (§6.3).
- **Ancienneté** dans le poste tirée selon l'âge ; elle donne les compétences de métier (§4.3) et la place dans la hiérarchie (composante « autorité » de l'influence).
- **Emplois de nuit** : **≈ 12 % des emplois** (≈ 500 postes, de nuit ou en roulement : santé, sécurité, usine en 3×8, logistique, bars, boulangers, transports, propreté), selon [economie.md](economie.md), §3.4. Ils sont pourvus en priorité par des personnages au chronotype tardif, sans exclusivité.
- **Activités parallèles** : certains habitants ont, en plus d'un emploi ou à la place, une activité secondaire (bénévole, élu municipal, entraîneur du club, revendeur). Pour les activités illégales, environ **2 % des adultes** au départ **[à calibrer]** : les membres des bandes et du réseau (§5.5), et des indépendants (cambrioleurs, receleurs, travailleurs au noir). Ils sont tirés parmi les profils compatibles (respect des règles bas), **jamais selon l'origine** (§0).

### 3.3 Écoles et garde des jeunes enfants (décidé)

*Pourquoi : la carte scolaire et le choix du privé par les familles aisées sont les règles réelles, et elles fabriquent des amitiés de quartier.*

| Âge | Affectation |
|---|---|
| 0 – 2 ans | Parent au foyer, grand-parent proche, assistante maternelle (habitante), ou crèche, selon l'emploi des parents et leurs revenus |
| 3 – 10 ans | École publique du **secteur** du domicile (≈ 6 écoles, ≈ 130 élèves chacune) ; quelques familles choisissent une école privée si la ville en prévoit une |
| 11 – 14 ans | Collège du secteur (≈ 2 collèges, ≈ 200 élèves chacun) |
| 15 – 17 ans | Lycée public (≈ 200), **lycée privé** de Hautevigne (≈ 100, surtout des familles aisées), ou apprentissage dans une entreprise de la ville |
| 18 ans et plus | Antenne universitaire (≈ 400 étudiants, dont certains vivent chez leurs parents) |

- **Flot à coût minimal** (décidé, P19) : enfants → écoles selon la **carte scolaire** (secteur du domicile), le choix du privé dépendant du revenu et de la tradition familiale ; égalités départagées par `StableId`.
- Chaque enfant a une **classe** (niveau scolaire stocké, pas recalculé, voir §9.2) et des **camarades** (§5.3).
- Les enseignants, éducateurs et personnels sont des habitants affectés à l'étape P3.

### 3.4 Entreprises et leurs propriétaires (décidé)

*Pourquoi : l'économie modélise les entreprises ([economie.md](economie.md), §4.1) mais demande à la population de dire à qui elles appartiennent ; dans une ville sans figurants, chaque commerce doit avoir un vrai patron et chaque société de vrais associés.*

- Chaque établissement du registre ([ville.md](ville.md), §9.1) est rattaché à une **entreprise** (≈ 500 à 700), dont le générateur fixe la **forme** et les **propriétaires** **[parts à calibrer]** :

| Forme | Propriétaires | Exemples |
|---|---|---|
| **Indépendant** | Un habitant, souvent avec son conjoint ou un enfant qui l'aide | Boulangerie, bar, garage, plombier, cabinet médical |
| **Petite société** | Deux à cinq associés habitants, parfois de la même famille | Entreprise de bâtiment, restaurant, agence immobilière |
| **Grande société** | Une famille de Hautevigne, des associés habitants et une part extérieure | Usine principale (propriété de la plus grande fortune, §6.2), plateforme logistique, hypermarché |
| **Succursale extérieure** | Un propriétaire extérieur abstrait ; un **directeur habitant** | Banques, assurances, enseigne de supermarché |
| **Service public, association** | Commune, État, Orvenne Habitat, association ([economie.md](economie.md), §4.9) | Mairies, hôpital, écoles, foyer de travailleurs, régie des bus |
| **Clandestine** | Le chef de la bande ou du réseau (§5.5) | Point de vente, recel, atelier de maquillage de véhicules |

- Le **gérant** (souvent le propriétaire), l'**organigramme** et les contrats sont ceux de l'affectation des emplois (§3.2). La **date de création** ou de reprise de l'entreprise est une étape de la biographie du propriétaire (fondée par lui, héritée de ses parents, rachetée).
- **Viabilité par la localisation** (décidé, P9 de [revues/generation.md](../revues/generation.md)) : les établissements sont placés par la ville selon des **aires de chalandise** (couverture des clients, regroupement et répulsion des concurrents) ; un commerce placé près de ses clients est viable par construction, et les corrections ci-dessous deviennent **rares**.
- **Contrôle de viabilité** (à la demande de [economie.md](economie.md), §14.4) : avant de figer la population, le générateur estime pour chaque entreprise la **marge attendue** à partir de la **clientèle réelle** de son quartier (ménages, revenus, distance, concurrents), de sa **part extérieure** éventuelle ([economie.md](economie.md), §4.4), de ses charges (loyer, salaires, stocks) et de ses prix. Une entreprise dont la marge attendue est négative est corrigée dans cet ordre : effectif réduit, prix ajustés dans la fourchette du marché, local moins cher, puis, en dernier recours, **changement d'usage** du local (avec la ville, étape 8 de [ville.md](ville.md), §4.2). Le **rodage** (§1.4) confirme le résultat. *Pourquoi : une ville qui ferait faillite dans son premier mois ne serait pas crédible, et l'économie ne doit pas avoir à rattraper une génération déséquilibrée.*
- Quelques entreprises démarrent volontairement **en difficulté** (trésorerie basse, dette, clientèle en baisse) **[≈ 5 %, à calibrer]**, pour que des faillites, des reprises et des projets naissent dans les premiers mois.

### 3.5 Police, justice et armes au départ (décidé)

*Pourquoi : le domaine combat fixe l'organisation et les effectifs ([combat.md](combat.md), §5.1, §5.2, §14) ; la règle « aucun figurant » impose que chacun de ces postes soit tenu par un habitant créé ici.*

**Effectifs à pourvoir** (≈ 80 personnes **[à calibrer]**), tous marqués **indispensables** (§1.3) :

| Service | Effectif | Remarques pour le générateur |
|---|---|---|
| Police nationale | ≈ 36 | Commissaire central (hors catégorie, fiche d'ancrage, §6.2), commissaire adjoint (fiche d'ancrage, son suppléant), 4 brigades de roulement de 5, brigade anticriminalité (4), sûreté (7, dont un technicien de police scientifique et deux enquêteurs spécialisés), soutien (2) |
| Polices municipales | ≈ 17 | Orvenne 8 + 2 agents de surveillance de la voie publique, Forges 4, Hautevigne 3 |
| Tribunal | ≈ 13 | Président, juges (dont juge d'instruction, juge des enfants, juge de l'application des peines), procureur (hors catégorie, fiche d'ancrage), deux substituts, greffiers |
| Autres | ≈ 20 | Conseiller d'insertion et de probation (1), commissaires de justice (2 et un clerc), mandataire judiciaire (1), contrôleurs fiscaux (2), inspecteur du travail (1), avocats (≈ 10, aux Tilleuls), armurier (1) |
| Fonctions cumulées | — | Trois **juges consulaires** tirés parmi les commerçants et chefs d'entreprise ; la fonction de **médecin légiste** confiée à un urgentiste de l'hôpital |

- **Profils** : ces métiers attirent davantage les profils **légalistes**, sans exclusivité (§3.2) : quelques policiers, greffiers ou avocats ont un respect des règles moyen ou bas, ce qui rend possibles la corruption et les informateurs ([combat.md](combat.md), §5.7). **Lesquels** sont corruptibles est tiré par monde parmi eux (§1.6, E25).
- **Casier** : aucun casier non effacé chez les policiers, magistrats et greffiers (métiers où l'employeur le vérifie, [combat.md](combat.md), §8.7).
- **Biographies** : les policiers et magistrats sont souvent **arrivés par mutation** (une étape de leur biographie) ; une partie des policiers vivent dans d'autres quartiers que ceux qu'ils patrouillent, quelques-uns aux Peupliers ou aux Forges, où ils ont grandi.
- **Relations** : les collègues de brigade sont une équipe (§5.5) ; des liens de connaissance relient policiers, magistrats et avocats ; quelques policiers ont un **informateur** dans le milieu, inscrit comme relation et comme secret (§5.4) ; qui informe qui est tiré par monde avec la couche cachée (§1.6, E25).

**Armes au départ** ([combat.md](combat.md), §1.5) **[à calibrer]** :

- **≈ 80 chasseurs déclarés**, surtout à Hautevigne et aux Forges : la chasse est un loisir de leur biographie (compétence de tir correspondante), ils ont un **permis**, une ou deux armes déclarées rangées dans une **armoire forte** à leur domicile, et sont membres de la **société de chasse** (une association de plus au §5.5). Aucun n'a de casier qui interdirait le permis.
- **≈ 10 à 15 armes illégales** dans le milieu, surtout dans le réseau des Quais et chez les chefs de bande (§5.5), cachées dans des planques connues de quelques membres (savoir opaque). L'emplacement exact des planques, leur contenu et leurs détenteurs sont tirés par monde (§1.6, S3).
- Les armes de service de la police sont rangées au commissariat : elles relèvent du domaine combat, pas des habitants.

---

## 4. Profils, attributs, santé, palier et apparence

### 4.1 Profil comportemental (décidé)

*Pourquoi : un tirage commun pour tous, légèrement influencé par la famille, donne une population variée où chaque profil de joueur trouve des personnages proches, sans stéréotypes.*

- **Adultes** : les neuf axes du profil ([04-joueur.md](../04-joueur.md), §2) sont tirés dans une **distribution de population** continue, centrée, avec de faibles corrélations réalistes entre axes (la bienveillance et la violence s'opposent un peu, la stabilité émotionnelle et l'impulsivité aussi) **[à calibrer]**.
- **Ressemblance familiale** : le profil d'un adulte est tiré en partie à partir de celui de ses parents **lorsqu'ils vivent en ville** (environ 40 % de ressemblance, 60 % propre à chacun **[à calibrer]**), et les couples se ressemblent un peu (on se choisit). *Pourquoi : des familles où l'on se ressemble sans être identiques rendent les conflits et les alliances familiales crédibles.*
- **Biographie** : quelques événements marquants de la biographie (§5.1) déplacent légèrement le profil dans la direction des **effets de ces événements** (décidé, [revues/signatures.md](../revues/signatures.md), G13), comme pour les enfants ([ia-pnj.md](ia-pnj.md), §8.5).
- **Enfants** : profil de départ tiré de celui des parents et du climat familial, puis **malléable** jusqu'à 18 ans ([ia-pnj.md](ia-pnj.md), §8.5).
- Les **personnages faits à la main** ont un profil fixé ou borné par leur fiche (§6.3).

### 4.2 Chronotype (décidé)

Tiré à la création dans une distribution continue (lève-tôt ↔ couche-tard), puis **modulé par l'âge** (adolescents et jeunes adultes plus tardifs, personnes âgées plus matinales), conformément au socle ([02-personnages.md](../02-personnages.md), §2). Il influence légèrement l'affectation aux emplois de nuit (§3.2). *Pourquoi : c'est la règle du socle ; le lien avec les emplois de nuit est la seule corrélation utile, et elle reste faible pour ne pas figer les rôles.*

### 4.3 Attributs et compétences (décidé)

*Pourquoi : calculer les attributs à partir de l'âge et du vécu avec le modèle de progression réaliste garantit qu'ils évolueront ensuite selon les mêmes règles que celles qui les ont produits.*

- **Potentiel** : chaque personne a un potentiel par attribut (force, endurance, coordination, perception, mémoire, temps de réaction…), tiré dans une distribution de population, en partie hérité des parents pour les enfants et les lignées (§5.2).
- **Valeur actuelle** = potentiel × **courbe d'âge** (pic physique vers 25–30 ans, déclin progressif ; mémoire et raisonnement plus stables) × effet du **vécu** (sport pratiqué, métier physique, sédentarité, blessures anciennes, maladies), calculé avec le **modèle de progression et de désentraînement** du socle.
- **Compétences** : chaque année de pratique dans la biographie (métier, études, loisirs, sport, activités illégales) fait progresser les compétences correspondantes selon le même modèle. Un menuisier de 50 ans avec 30 ans de métier est un excellent menuisier ; un ancien militaire garde une partie de ses compétences de combat.
- **Normalisation** : les attributs sont ensuite exprimés par rapport à la population du monde (50 = la moyenne), comme le veut le socle ([02-personnages.md](../02-personnages.md), §3.1).
- **Savoirs de départ** : lieux connus (quartier de résidence, trajets habituels, lieu de travail, lieux fréquentés depuis des années), personnes connues (§5.3), savoirs de métier, et **savoirs opaques** liés au rôle (codes du lieu de travail, combinaisons, secrets) ([ia-pnj.md](ia-pnj.md), §4.4). Le rôle dit qui **peut** détenir un savoir opaque ; les détenteurs précis sont tirés par monde (§1.6, E25).

### 4.4 Santé de départ (décidé)

*Pourquoi : les décès naturels doivent être annoncés par une santé qui décline (§7.3), donc la santé de fond doit exister dès le départ, avec des fréquences réalistes.*

- **Vitalité de fond** : une valeur qui décroît avec l'âge, plus ou moins vite selon le mode de vie et l'hérédité ; elle porte le risque de décès naturel (§7.3).
- **Maladies chroniques et handicaps** (diabète, maladie cardiaque, douleurs de dos, mobilité réduite, troubles de la vue ou de l'audition), à des fréquences réalistes selon l'âge **[à calibrer]** ; ils pèsent sur les attributs et sur l'agenda (rendez-vous médicaux, traitements).
- **Habitudes** : tabac, alcool, activité physique, alimentation, et quelques **dépendances** (alcool, drogues, jeu) à des fréquences réalistes **[à calibrer]**.
- **États en cours** au départ : quelques grossesses (≈ 75, cohérent avec le rythme des naissances, §7.2), convalescences, maladies passagères, hospitalisations.

### 4.5 Influence, puissance et palier (décidé)

*Pourquoi : les paliers sont définis par rang, donc leurs proportions sont toujours justes ; ce qu'il faut maîtriser, c'est que les bons rôles se retrouvent aux bons paliers, comme dans les exemples du socle.*

- Les cinq composantes de l'**influence** ([02-personnages.md](../02-personnages.md), §3.2) sont calculées une fois le reste en place : **autorité** (place dans l'organisation du poste), **richesse** (patrimoine net du §4.7), **réseau** (graphe des relations initiales, §5.3), **moyens** (organisation dirigée : équipe, bande, service de police), **notoriété** (rôle public, ancienneté en ville, fiches d'ancrage).
- Le **niveau personnel** découle des attributs et des trois meilleures compétences (§4.3).
- La **puissance**, puis le **rang** sur toute la population, enfants compris, donnent le palier ([02-personnages.md](../02-personnages.md), §3.4). La répartition par palier (40 % au palier 0, ≈ 15 au palier 11, ≈ 10 hors catégorie) est donc respectée par construction.
- **Fourchette par rôle** : chaque fiche de rôle indique un **palier attendu** (caissière : 1 ; policier expérimenté : 4 ; chef de bande : 7…) d'après les exemples du socle. Si trop de personnes d'un rôle tombent hors de leur fourchette, le générateur ajuste leurs compétences, leur ancienneté ou leur place dans l'organisation, puis recalcule (étape P10 du §1.2, au plus 3 passes).
- **Enfants** : presque tous au palier 0 ; quelques adolescents remarquables (sportif de haut niveau, enfant de grande fortune) peuvent être plus haut, sans effet puisqu'ils ne sont pas jouables.

### 4.6 Couverture des catégories comportementales (décidé)

*Pourquoi : la règle d'accès exige un profil de joueur proche de celui du personnage ([04-joueur.md](../04-joueur.md), §4) ; si une catégorie était absente d'un palier, les joueurs de ce profil seraient bloqués.*

- Le générateur vérifie que **chaque catégorie comportementale** compte un minimum de personnages jouables dans **chaque groupe de paliers** (0–2, 3–5, 6–8, 9–11) **[minimum à calibrer]**.
- Il veille en particulier à la **sélection de départ** du joueur, tirée au hasard parmi les **adultes de palier 0** ([04-joueur.md](../04-joueur.md), §6) : ces quelque 2 200 adultes couvrent toutes les catégories, ce qui laisse à tout nouveau joueur des personnages proches de lui à mesure que son profil se forme.
- En cas de manque, il **déplace légèrement** le profil de personnages compatibles, plutôt que d'en créer de nouveaux.
- **Par secteur** (décidé, [revues/generation.md](../revues/generation.md), §9.4) : chaque catégorie et chaque groupe de paliers a au moins un candidat dans chaque secteur habité **[à calibrer]**.
- **Contrôle des données** (décidé, [revues/signatures.md](../revues/signatures.md), G16) : avant toute génération, `ts-data check` vérifie que chaque catégorie dispose d'au moins **deux moyens fidèles** pour chaque besoin important, et que les **recettes de cœur de métier** de chaque rôle sont fidèles aux profils que le générateur tire pour lui (§3.2, §4.3). *Pourquoi : un personnage jouable dont la catégorie n'aurait aucun moyen fidèle de se nourrir, ou dont le métier exigerait des actes contraires à son profil, serait injouable pour les joueurs de ce profil.*

### 4.7 Patrimoine, revenus et dettes de départ (décidé)

*Pourquoi : l'économie fixe les prix et la médiane ([economie.md](economie.md), §1.3) et demande à la population de répartir les patrimoines ; les déduire de la biographie (salaires gagnés, achats, héritages) les rend cohérents avec la vie de chacun, au lieu d'un tirage au hasard.*

**Méthode** : le patrimoine de chaque adulte est **reconstitué à partir de sa biographie** (§5.1), avec les prix et les taux de l'économie :

- **épargne accumulée** chaque année selon ses revenus de l'époque, la taille de son foyer et sa propension à dépenser (profil : organisation, impulsivité) ;
- **achat du logement** à la date prévue par la biographie, financé par un **prêt immobilier** aux conditions de l'économie ([economie.md](economie.md), §4.10 : 3,5 %, jusqu'à 25 ans) ; le capital restant dû est calculé à la date de départ ;
- **héritages**, donations, divorces (partage), faillites, dettes de jeu ou d'usure, tels que la biographie les a placés ;
- **parts d'entreprise** (§3.4), **véhicules** et **objets de valeur** ;
- **espèces** : de petites sommes chez la plupart des gens, davantage chez les personnes âgées, les commerçants et ceux qui vivent de l'économie parallèle ([economie.md](economie.md), §1.2).

Le générateur ajuste ensuite l'épargne (jamais les biens) pour atteindre la distribution cible **[à calibrer]** :

| Groupe | Patrimoine net typique (écus) | Richesse (socle) |
|---|---|---|
| Étudiants, jeunes locataires, chômeurs, sans-abri | 0 à 5 000 (parfois négatif) | 0 à 30 |
| Locataires actifs modestes | 5 000 à 20 000 | 30 à 45 |
| Locataires aisés, jeunes propriétaires endettés | 20 000 à 100 000 | 45 à 55 |
| **Médiane des adultes** | **≈ 50 000** | **50** |
| Propriétaires ayant fini de rembourser, retraités propriétaires | 150 000 à 400 000 | 55 à 65 |
| Professions libérales, patrons de PME, cadres dirigeants | 300 000 à 1,5 million | 65 à 75 |
| Grandes familles de Hautevigne (quelques-unes) | 2 à 10 millions | 75 à 85 |
| Plus grande fortune (propriétaire de l'usine principale) | ≈ 50 millions | ≈ 90 |

La queue de la distribution suit une loi de Pareto, comme dans la réalité : le 1 % le plus riche (≈ 80 adultes, presque tous à Hautevigne et aux Tilleuls) détient une part importante du total **[à calibrer]**.

**Revenus** : le salaire de chacun est placé dans la fourchette de son métier ([economie.md](economie.md), §1.4) selon sa compétence et son ancienneté ; les retraites selon sa carrière ; les prestations selon sa situation (règles de l'économie, §8.3).

**Dettes en cours** **[parts à calibrer]** :

| Dette | Qui |
|---|---|
| Prêt immobilier | ≈ 60 % des propriétaires occupants de moins de 65 ans ; les habitants bailleurs qui ont acheté à crédit |
| Prêt à la consommation | ≈ 20 % des ménages (voiture, travaux) |
| Prêt professionnel | Une partie des indépendants et des petites sociétés (§3.4) |
| Ménages en difficulté | ≈ 3 % des ménages : découvert permanent, retards de loyer, incident bancaire, qui démarrent avec des relances |
| Dettes entre habitants | Quelques centaines de petites dettes entre proches, inscrites comme **engagements** ([ia-pnj.md](ia-pnj.md), §4.2) |
| Dettes illégales | Quelques habitants endettés auprès du réseau (usure), dont celui d'une intrigue de départ (§6.4) |

### 4.8 Préférences d'attirance (décidé)

*Pourquoi : le social en a besoin pour la formation des couples ([social.md](social.md), §1.6) ; les attribuer à la création, de façon cohérente avec les couples existants, évite des couples de départ incompatibles avec leurs propres préférences.*

Chaque **adulte** reçoit des **préférences d'attirance** :

- **Orientation** : attirance pour les hommes, pour les femmes, ou pour les deux, avec des parts réalistes (environ 93 %, 3 à 4 % et 3 à 4 % selon les cas **[à calibrer]**), et quelques personnes très peu attirées par autrui ;
- **Tranche d'âge** préférée, centrée sur son propre âge, plus ou moins large selon la personne ; elle exclut toujours les mineurs (règle absolue de [social.md](social.md), §1.6, garantie par construction : l'attirance envers un mineur vaut toujours zéro) ;
- **Traits appréciés** : deux ou trois traits parmi les axes du profil (bienveillance, extraversion, stabilité…), le statut, l'apparence et la forme physique, avec un **degré d'exigence**. L'**apparence** lue ici est la silhouette, le soin et le style de la tenue, jamais la carnation ni les traits du visage (décidé, §4.9 ; *pourquoi : ils n'entrent dans aucun calcul du cœur*, [direction-artistique.md](direction-artistique.md), §3.2).

Règles :

- Les **couples existants** sont tirés compatibles avec les préférences de leurs deux membres ; une part d'entre eux a une attirance **émoussée** (couples usés), source possible de séparations ou d'infidélités.
- Les **enfants** n'ont pas de préférences d'attirance : elles sont tirées le jour de leurs **18 ans**, en partie à partir de leur vécu. Le statut « couple » entre adolescents reste une simple étiquette ([social.md](social.md), §1.6).
- Les préférences ne changent pas avec le temps, sauf la tranche d'âge, qui suit l'âge du personnage.

### 4.9 Génome d'apparence (décidé)

*Pourquoi : l'apparence est une donnée du cœur, produite ici puis transmise au client sous forme perçue ([direction-artistique.md](direction-artistique.md), §3.1, §3.5 ; [05-architecture.md](../05-architecture.md), §2.2). Puisqu'il n'y a aucun figurant, la variété d'une rue est exactement celle de ses habitants : le générateur la porte seul.*

Chaque habitant reçoit à l'étape P6 (§1.2) un **génome d'apparence** d'environ 64 octets **[à mesurer]**, dont les champs sont fixés par la direction artistique ([direction-artistique.md](direction-artistique.md), §3.2). Ce domaine fixe **d'où vient** chaque champ :

| Groupe | Tiré d'après | Règle (décidé) |
|---|---|---|
| **Silhouette** (taille, corpulence, musculature, carrure) | Âge, sexe, attributs physiques (force, endurance, §4.3), habitudes et santé (§4.4) | **Jamais** d'après le milieu ni l'origine (§0, principe 5). La taille adulte est tirée d'après le sexe et en partie de la moyenne des parents ; la corpulence et la musculature suivent les attributs et les habitudes, et évoluent avec eux en cours de partie |
| **Visage** (≈ 24 paramètres de forme, asymétrie légère) | Tirage, avec **hérédité limitée** | Chaque enfant reçoit seulement une **faible part** des traits de visage de ses parents (≈ 25 % **[à calibrer]**), le reste est propre. Les **jumeaux monozygotes** partagent le même visage, à une asymétrie près ; les jumeaux dizygotes sont des frères et sœurs ordinaires |
| **Peau** (carnation, taches de rousseur) | Origine familiale (§2.4), hérédité | Moyenne bruitée des parents ; pour les fondateurs d'une lignée, tirée d'après l'origine familiale. Les **rides** ne sont pas tirées : elles découlent de l'âge et du vieillissement (ci-dessous) |
| **Cheveux et pilosité** (couleur naturelle, nature, calvitie, grisonnement, barbe possible) | Hérédité, sexe, âge | Couleur et nature : moyenne bruitée des parents. Âge de début et rythme de la **calvitie** et du **grisonnement** : en partie hérités, avec des parts réalistes selon l'âge **[à calibrer]** |
| **Signes distinctifs** | Biographie, santé, profil | **Lunettes** d'après les troubles de la vue (§4.4) ; **cicatrice** tirée pour chaque blessure **grave** ou **critique** de la biographie (agression, accident ; une opération lourde compte comme une blessure grave), à sa date, selon la **règle du combat** : ≈ 30 % des graves, ≈ 70 % des critiques **[à calibrer]**, par l'aléa à clé de la blessure (décidé, arbitrage A9 ; [combat.md](combat.md), §2.1, §2.4 ; santé, §4.4) ; **tatouage** d'après le profil, la biographie (prison, armée, jeunesse) et l'appartenance à une bande (§5.5) ; grain de beauté marqué par tirage. Aucun signe sans cause, sauf le grain de beauté |
| **Voix sonore** (hauteur, timbre, débit) | Sexe, âge, corpulence, tirage | À distinguer de la **voix** de l'état intérieur (registre, politesse, humour, vocabulaire, §5.6) : la voix sonore dit comment la personne **sonne** ([direction-artistique.md](direction-artistique.md), §5.5), l'autre comment elle **parle**. Toutes deux sont stables, sauf l'effet de l'âge |
| **Goûts vestimentaires** (style dominant, couleurs préférées, soin) | Âge, milieu, profil | Tirés dans des lois par âge et par milieu, déplacées par le profil (un consciencieux soigne sa tenue, un extraverti ose les couleurs) ; l'origine n'y entre pas. Ils guident la garde-robe (§4.10) et la **coiffure** de départ |

- **Coiffure** (décidé) : elle n'est pas dans le génome, mais dans l'état courant du personnage ([direction-artistique.md](direction-artistique.md), §3.2). La population en tire une **de départ** (coupe voulue) d'après l'âge, le sexe, les cheveux, les goûts et le métier ; elle change ensuite chez le coiffeur ([ville.md](ville.md), §5.3 ; [ia-pnj.md](ia-pnj.md), §2.6). La **longueur** de départ est tirée **uniformément** entre la coupe fraîche et le seuil de visite propre au personnage (décidé). *Pourquoi : si tout le monde partait d'une coupe fraîche, toute la ville irait chez le coiffeur la même semaine ; un tirage uniforme sur le cycle donne d'emblée le rythme stationnaire.*
- **Hérédité par les parents officiels** (décidé) : l'hérédité ne suit que les liens de parenté de la population **visible** (§5.2). Une filiation secrète tirée par monde (§1.6, §6.4) ne modifie jamais un génome. *Pourquoi : le génome est le même dans tous les mondes d'une famille ; s'il dépendait d'un secret, il le révélerait. L'hérédité limitée et le test de parenté (§1.3) font qu'un parent caché ne se trahit pas non plus par une ressemblance trop forte avec son enfant officiel.*
- **Fiches d'ancrage** (décidé) : une fiche peut fixer ou borner des champs du génome (âge apparent, silhouette, signes distinctifs), jamais la carnation ni le visage, qui restent tirés comme pour tous (§6.3). *Pourquoi : un personnage fait à la main ne doit pas se reconnaître à un visage de commande, ni porter une apparence liée à son rôle.*
- **Les proches à l'extérieur** (§8) n'ont pas de génome. Celui qui vient s'installer en ville (§7.4, §8.3) reçoit le sien à son arrivée, par hérédité depuis ses parents ou vers ses enfants qui vivent déjà en ville.

**Historique daté de l'apparence** (décidé ; [interface.md](interface.md), §4.7, §15.5 ; [05-architecture.md](../05-architecture.md), §2.5). Chaque habitant a, à côté de son génome, un court **historique** des changements que le vieillissement ne calcule pas : **coupe de cheveux**, **tenue habituelle** (la tenue la plus portée pour chaque usage) et **signes distinctifs** (cicatrice, tatouage, lunettes). Avec le génome et la date, il permet au serveur de reconstituer le portrait d'une personne au **dernier contact** ; les rides, le grisonnement, la calvitie et la taille se recalculent à l'âge de cette date.

- **Au départ**, la population écrit cet historique d'après la biographie : chaque signe distinctif à la date de sa cause (l'agression, l'accident, l'opération, le tatouage, les premières lunettes) ; une **coupe antérieure** quand la biographie a un événement marquant récent (rupture, mariage, nouvel emploi, [ia-pnj.md](ia-pnj.md), §2.6), et la coupe de départ à partir de cette date ; la tenue habituelle actuelle, et une tenue antérieure quand le métier ou le milieu a changé.
- **Profondeur** : l'historique de départ remonte au plus loin à la date du plus ancien dernier contact des souvenirs de départ (§5.4), et compte au plus **8 entrées** par personne **[à calibrer]**. Un contact plus ancien que la première entrée se reconstitue avec celle-ci.
- **En cours de partie**, le cœur y ajoute chaque changement (coupe chez le coiffeur, nouvelle cicatrice, nouvelle tenue habituelle) ; la population ne fait que l'initialiser.

**Plages et distribution de l'éditeur** (décidé ; [interface.md](interface.md), §3.2, §15.5). Concordance vérifiée avec la demande de l'interface :

- la population publie, dans le paquet de contenus, les **plages** de chaque champ du génome **par âge et par sexe**, celles-là mêmes que ses lois parcourent ;
- les boutons « **Autre visage** » et « **Tirer au hasard** » tirent dans la **distribution d'un fondateur de lignée** : les lois de la population pour l'âge, le sexe et les attributs choisis, l'origine familiale étant elle-même tirée selon ses parts dans la population (§2.4). *Pourquoi : le persistant n'a pas de parents en ville ; tirer comme pour un fondateur donne exactement les apparences que la ville contient ;*
- la **musculature** et la corpulence du persistant découlent de sa force et de son endurance par la même règle que pour les habitants (silhouette, ci-dessus).

**Vieillissement du génome** (décidé). Le génome ne change que par des règles **déterministes** qui lisent l'âge, le génome et l'état du personnage :

- **Rides** : elles apparaissent et se creusent avec l'âge, plus tôt avec le tabac, l'exposition au soleil des métiers d'extérieur et une vitalité basse (§4.4) **[courbes à calibrer]** ;
- **grisonnement** et **calvitie** : ils commencent à l'âge tiré dans le génome et avancent à leur rythme ;
- **silhouette** : les **enfants grandissent** d'après une courbe de croissance qui mène à leur taille adulte, et leurs proportions suivent l'âge ([direction-artistique.md](direction-artistique.md), §3.8) ; chez l'adulte, la corpulence et la musculature suivent les attributs physiques et les habitudes, et la taille baisse un peu chez les plus âgés ;
- **signes distinctifs** : une blessure grave ou critique en cours de partie peut laisser une cicatrice, selon la même règle du combat (A9), un tatouage s'ajoute par un acte du personnage ; aucun signe ne disparaît, sauf les lunettes, qui suivent la vue ;
- **voix sonore** : elle mue à l'adolescence et vieillit avec l'âge ;
- le **visage** ne change pas de forme, hors du vieillissement de la peau. Le **rodage** (§1.4, §1.6) applique ces règles comme le reste de la vie, si bien que chaque habitant a l'apparence de son âge le premier jour.

**Espace de l'éditeur du persistant** (décidé, [direction-artistique.md](direction-artistique.md), §3.7 ; [02-personnages.md](../02-personnages.md), §1.1). Toute combinaison que l'éditeur d'apparence propose a une **probabilité non nulle** dans le générateur :

- les lois de chaque champ ont un **plancher** de probabilité sur toute leur plage **[à calibrer]**, sans aucune combinaison exclue par une règle de dépendance (une carnation et une couleur de cheveux rares ensemble restent possibles) ;
- l'éditeur et le générateur lisent les **mêmes plages** et les **mêmes catalogues** de coiffures, de voix et de vêtements, dans le paquet de contenus ; une plage élargie d'un côté l'est de l'autre ;
- un **test automatique** vérifie que chaque valeur et chaque paire de champs de l'éditeur ont une probabilité non nulle dans les lois de la population, et que chaque élément du catalogue de l'éditeur est porté par au moins un habitant d'une population de référence **[taille à calibrer]**.

*Pourquoi : un persistant dont l'apparence serait impossible chez un habitant serait désigné comme joueur au premier coup d'œil, ce que le socle interdit ([01-concept.md](../01-concept.md), §1.1).*

### 4.10 Garde-robe abstraite (décidé)

*Pourquoi : la tenue dit beaucoup de ce que les habitants lisent les uns des autres (richesse, métier, négligence, météo, [social.md](social.md), §5.5) ; elle doit donc être produite avec la personne, d'après ce qu'elle est, et non tirée par le client ([direction-artistique.md](direction-artistique.md), §3.3).*

Chaque habitant reçoit à l'étape P8 (§1.2), avec le mobilier de son logement, une **garde-robe abstraite** : quelques tenues par usage (quotidien, travail, sortie, sport, intempéries, nuit), faites de pièces sémantiques du catalogue vestimentaire, chacune avec sa couleur et son état.

| Entrée | Effet sur la garde-robe |
|---|---|
| **Âge** | Pièces propres à l'âge (cartable, tenue de sport scolaire, vêtements de bébé, pièces de personne âgée) |
| **Milieu** et patrimoine (§4.7) | Nombre de tenues, qualité et état des pièces, part de pièces usées |
| **Profil** et goûts vestimentaires (§4.9) | Style dominant, couleurs, soin (pièces propres ou froissées, assorties ou non) |
| **Métier** | **Uniforme ou tenue de travail** du poste (police, pompiers, secours, chantier, hôpital, cuisine, livraison, transports), fournie par l'employeur ; tenue de bureau selon le palier |
| **Groupe** (étape P9) | **Signes de bande** pris dans la **signature graphique** de la bande (ses couleurs, une pièce distinctive, un tag sur une veste, [social.md](social.md), §6.6 ; §5.5), d'autant plus nombreux que le rang et la loyauté du membre sont élevés (social §6.7) ; maillot du club, tenue d'une association ; ajoutés quand les groupes sont formés. Les signes de bande sont des pièces de la garde-robe : la tenue du jour les porte ou les retire selon la pression de la police et l'occasion ([ia-pnj.md](ia-pnj.md), §2.6) |
| **Climat** | Au moins une tenue d'intempéries et une tenue d'hiver, selon la saison de départ |

- **La garde-robe est un bien** (décidé) : elle s'use, se salit, se lave, se répare et s'achète en ville ([economie.md](economie.md)) ; le persistant habille son personnage avec la sienne, achetée de la même façon ([direction-artistique.md](direction-artistique.md), §3.7). La population ne fait que la **donner au départ** et à chaque création de personne.
- **Valeur et usure de départ** (décidé ; [economie.md](economie.md), §5.13) : pour l'économie, la garde-robe est **une seule valeur** par personnage. Sa **valeur de remplacement** de départ est celle de sa classe (R1 ≈ 600, R3 ≈ 1 500, R5 ≈ 4 000, aisée ≈ 15 000 par adulte ; la moitié pour un enfant **[à calibrer]**), modulée par le soin. Son **usure** de départ est tirée **uniformément** entre 0 et le seuil d'achat propre au personnage, comme la longueur de cheveux (§4.9). *Pourquoi : sinon toute la ville achèterait ses vêtements la même semaine.* Les uniformes, fournis par l'employeur, n'entrent pas dans cette valeur.
- **État de départ des pièces** (décidé ; [ia-pnj.md](ia-pnj.md), §2.6) : l'**usure visible** des pièces est répartie de façon à correspondre à l'usure agrégée, plus marquée chez les négligés ; la **propreté** (propre, froissé, sale, taché) d'après la distribution stationnaire du cycle de lavage du ménage, mesurée par `ts-sim` (§1.1) ; aucune pièce n'est mouillée ni déchirée au départ, sauf chez les sans-abri. Le rodage (§1.4) fait ensuite tourner le cycle. *Pourquoi : un départ où tout est propre et neuf ferait laver toute la ville le premier jour.*
- **Variété** (décidé) : les tirages de couleurs et de pièces d'un même quartier sont **décorrélés** entre ménages, et le contrôle de dispersion (§1.3) refuse les répétitions ; seuls les uniformes se répètent.
- **Complétée à chaque création** (décidé) : un nouveau-né (§7.2) reçoit une garde-robe de bébé d'après le milieu de ses parents ; un nouvel arrivant (§7.4) arrive avec la sienne, faite par le même générateur ; un enfant qui grandit, un nouvel emploi (uniforme) ou une entrée dans une bande la font évoluer par les règles de la vie, pas par le générateur.
- La **tenue du jour** est choisie par l'IA dans cette garde-robe ([ia-pnj.md](ia-pnj.md)) ; la population n'en tire aucune.

### 4.11 Goûts musicaux, instruments et compétence musicale (décidé)

*Pourquoi : la radio choisie, l'instrument dont on joue le soir et le musicien du bar viennent de la vie des habitants ([ia-pnj.md](ia-pnj.md), §2.6 ; [direction-artistique.md](direction-artistique.md), §5.4) ; il faut donc les donner au départ.*

- **Goûts musicaux** : quelques genres préférés et rejetés, avec leur intensité, tirés à l'étape P6 d'après la **génération** (la musique de ses 15 à 25 ans pèse le plus), le **milieu**, le **profil** (l'ouverture élargit les goûts) et, en partie, les goûts des parents et du conjoint ; jamais d'après l'origine (§0, principe 5). Les enfants de moins de **10 ans** **[à calibrer]** écoutent ce qu'écoute leur ménage ; leurs goûts propres sont tirés à cet âge, puis évoluent avec la vie ([ia-pnj.md](ia-pnj.md)).
- **Compétence musicale** : calculée comme toute compétence (§4.3) à partir des années de pratique de la biographie : cours dans l'enfance, pratique de loisir, groupe, chorale, métier de musicien. Parts réalistes de pratiquants selon l'âge et le milieu **[à calibrer]** ; les enfants inscrits en cours de musique le sont aussi au départ ([ia-pnj.md](ia-pnj.md), §8.1).
- **Instruments possédés** : chaque pratiquant actuel possède son instrument, placé dans le logement à l'étape P8 avec le mobilier (un piano est un meuble) ; quelques anciens pratiquants en gardent un qui ne sert plus. Les musiciens de métier (organiste, musicien de bar, professeur de musique) sont pourvus par le flot des postes (§3.2), leur instrument étant à eux ou au lieu.
- **Musiciens de départ** (décidé, [economie.md](economie.md), §3.8) : les postes de musique (organiste de l'église, professeur de musique, animateur de la radio locale) sont pourvus par le flot des postes (§3.2) parmi les habitants qui ont la compétence. Les **musiciens de bar, de fête ou de rue** ne sont pas des postes : quelques pratiquants de bon niveau **[nombre à calibrer]**, de profil extraverti ou à petits revenus, reçoivent cette activité régulière au départ, avec leurs lieux et leurs soirs habituels, une réputation tirée de leur compétence et, pour les musiciens de bar, une relation avec les patrons qui les engagent.
- **Postes et lieux** : la tâche « mettre de la musique » d'un bar ou d'une boutique relève du poste, pas de la population (ia-pnj §2.6).

---

## 5. Histoire et relations initiales

### 5.1 Biographie (décidé)

*Pourquoi : une biographie datée est la source commune des attributs, des relations, des souvenirs et des projets, et elle donne au joueur une vie à découvrir.*

Chaque adulte reçoit une **biographie** : une liste d'**étapes de vie** datées, tirées à rebours depuis sa situation actuelle pour rester compatibles avec elle.

| Étape | Exemples |
|---|---|
| Origine | Né à Orvenne (dans quel quartier) ou ailleurs ; âge d'arrivée et motif (emploi, études, couple, famille) |
| Formation | Diplôme, apprentissage, abandon d'études |
| Emplois | Emplois successifs, licenciements, chômage, création d'entreprise, service militaire ou carrière militaire |
| Vie de couple | Rencontres, unions, séparations, veuvage |
| Enfants | Naissances, enfants partis de la maison (en ville ou à l'extérieur, §8) |
| Logements | Déménagements dans la ville |
| Événements marquants | Deuil, accident, maladie grave, agression subie, condamnation et détention, réussite sportive, héritage, faillite |

Proportions de départ **[à calibrer]** : environ la moitié des adultes nés dans l'agglomération (davantage aux Forges, moins à Beaulieu et dans le quartier de la Gare).

**Méthode** (décidé, P20) : un **échantillonneur de parcours** remonte le temps depuis la situation actuelle (étape P3). Il tire les dates des transitions dans les tables de taux par âge et par génération, **conditionnées** par ce qui est déjà fixé : l'âge des enfants présents fixe des naissances, l'ancienneté dans le poste fixe la date d'embauche, et l'**année de construction** du bâtiment borne la date d'emménagement.

**Chronique** (décidé, P3) : la chronique d'Orvenne (§1.1) injecte des **événements collectifs** dans les biographies (licenciés du plan social de l'usine, sinistrés d'une crue, anciens élèves du collège avant sa reconstruction, supporters du titre du club). Chacun devient une étape datée et un **souvenir partagé** entre ceux qui l'ont vécu (§5.4), avec les rancunes et les sujets de conversation qui en découlent. *Pourquoi : des repères collectifs (« depuis la fermeture du haut-fourneau… ») rendent les biographies et les conversations crédibles et rattachent la population à la ville.*

**Casiers judiciaires** (décidé, à la demande de [combat.md](combat.md), §8.7 et §16.4 ; *pourquoi : le casier pèse sur l'embauche et la récidive, il doit donc exister dès le départ et découler de la biographie*) :

- environ **5 % des adultes** ont un **casier non effacé** au départ **[à calibrer]** : chaque condamnation est une étape datée de leur biographie (infraction, peine), cohérente avec leur profil, et les délais d'effacement du domaine combat sont appliqués à la date de départ ;
- environ **1 % des adultes** sont d'**anciens détenus** (étape « détention » dans leur biographie, avec ses effets : emploi perdu, relations distendues, contacts dans le milieu) ;
- les **mesures éducatives** reçues avant 18 ans sont effacées ([combat.md](combat.md), §8.8) mais restent dans les souvenirs de l'intéressé et de son entourage ;
- les casiers sont plus fréquents dans le milieu (§5.5), **jamais tirés selon l'origine** (§0). Les biographies des enfants sont courtes (naissance, crèche, écoles, événements familiaux).

### 5.2 Lignées et familles étendues (décidé)

*Pourquoi : dans une ville moyenne, beaucoup d'habitants ont de la famille tout près ; ces liens donnent des visites, des gardes d'enfants, des héritages et des querelles de famille.*

- L'étape P4 (§1.2) relie des ménages par la **parenté** : parents et enfants adultes, frères et sœurs, grands-parents, oncles, tantes et cousins.
- Environ **la moitié des adultes** ont au moins un parent, un enfant adulte, un frère ou une sœur **dans l'agglomération** **[à calibrer]** ; beaucoup plus aux Forges (familles ouvrières enracinées), beaucoup moins chez les étudiants et les nouveaux venus.
- Une lignée partage un **nom de famille**, une part du **potentiel** des attributs et du **profil** (§4.1, §4.3), et parfois un **patrimoine** (maison de famille, commerce transmis).
- Les autres membres de la famille vivent **à l'extérieur** (§8).

### 5.3 Réseau initial (décidé)

*Pourquoi : chaque type de lien vient d'un lieu partagé (logement, travail, école, loisirs), ce qui donne un réseau crédible et des voisins qui se connaissent vraiment.*

Les relations ont la forme fixée par le social ([social.md](social.md), §1.1) : relation **orientée** (ce que A pense de B), avec un degré de connaissance, l'**affection**, la **confiance**, le **respect** et la **crainte** comme **quatre valeurs distinctes**, des étiquettes de liens, un **bilan** (services rendus et reçus, torts, promesses) et la date du **dernier contact**. Leur valeur de départ découle de l'**historique** que la biographie donne au lien (§5.1) :

- l'**affection** et la **confiance** viennent de la durée du lien, de la fréquence des rencontres, de la compatibilité des profils et des événements passés (services, torts) ;
- le **respect** vient de ce que A sait de B : compétence, statut, conduite, jugés selon les valeurs de A (un légaliste respecte le commissaire, un hors-la-loi le chef de bande) ;
- la **crainte** vient du pouvoir de B sur A (patron, propriétaire, créancier) ou de sa violence connue (membre d'une bande pour les habitants de son territoire, conjoint violent) ;
- le **bilan** résume les événements anciens de l'historique, comme si la consolidation de la mémoire avait déjà eu lieu ([ia-pnj.md](ia-pnj.md), §4.3) ;
- les relations ne sont **pas symétriques** : un employé peut craindre et respecter son patron, qui le connaît à peine.

| Lien | Règle de création **[ordres de grandeur à calibrer]** |
|---|---|
| **Famille** | Tous les liens de la lignée (§5.2) ; affection tirée, avec une part de familles **brouillées** |
| **Couple** | Conjoint actuel ; **ex-conjoints** vivant en ville (liens souvent tendus, parfois cordiaux) |
| **Voisins** | Même palier ou même petit immeuble : se connaissent presque tous de nom après un an ; même rue ou même tour : quelques-uns. Plus forts chez les extravertis, les anciens et les familles avec enfants |
| **Collègues** | Même équipe : tous ; même établissement : selon la taille ; supérieurs et subordonnés connus |
| **Camarades** | Même classe (enfants), mêmes études, ancien collège commun (adultes nés en ville) |
| **Amis** | 3 à 15 amis par adulte, choisis par **ressemblance** (âge, quartier, milieu, profil, loisirs) parmi les personnes rencontrées dans les lieux ci-dessus, puis complétés par les amis de leurs amis |
| **Habitués** | Client régulier d'un commerce, patient d'un médecin, membre d'un club ou d'une association : liens faibles mais nombreux, qui font de la ville un lieu où « l'on se connaît » |
| **Conflits** | ≈ 5 % des adultes ont un **conflit actif** (voisinage, héritage, ancien couple, rivalité professionnelle, dette impayée) |
| **Milieu** | Liens internes aux bandes et réseaux (§5.5) : loyauté et crainte envers le chef, respect entre membres ; liens avec leurs clients et leurs rivaux ; **crainte** et parfois **respect** des habitants de leur territoire envers eux |

- **Nombre** : environ **60 relations complètes** (degré « connaissance » ou plus) par habitant en moyenne **[à calibrer]**, soit de l'ordre de 600 000 liens pour la ville, plus **100 à 300 relations compactes** (« reconnu de vue », « connu de nom ») tirées des lieux fréquentés. Ces nombres restent sous les plafonds par couche du social (≈ 5 intimes, 15 proches, 130 connaissances, 500 visages et noms, [social.md](social.md), §1.2).
- **Départ et mémoire** (décidé, P41) : ≈ 600 000 relations complètes est le chiffre **au départ** ; la **mémoire** est dimensionnée sur les **plafonds** du social (150 complètes et 500 compactes par personne), vers lesquels le monde converge avec le temps.

**Méthode** (décidé, P21) : les liens viennent de **contextes partagés** (ménage, lignée, palier et immeuble, rue, équipe et établissement, classe, club, commerce habituel, groupe), chacun avec une probabilité de lien selon sa taille et l'**ancienneté** commune donnée par la biographie. Les **amitiés** sont choisies par **homophilie** parmi les personnes rencontrées dans ces contextes, puis complétées par **fermeture triadique** (les amis des amis). La recherche passe par les cellules et les contextes, jamais par toutes les paires, d'où un coût en O(N·k). *Pourquoi : les communautés d'Orvenne viennent de ses lieux ; ce modèle les reproduit et se mesure.*

**Métriques cibles** (décidé, P21), vérifiées par la validation (§1.3) **[plages à calibrer]**. Le validateur du réseau de départ utilise les **plages cibles de [social.md](social.md), §10.2**, qui font foi ; le tableau ci-dessous les recopie pour la lecture (décidé). *Pourquoi : le social fournit ces plages parce que ses règles doivent les maintenir ensuite ; une seule source évite qu'elles divergent.*

| Métrique | Plage cible |
|---|---|
| Relations complètes par adulte | Moyenne ≈ 60 ; 5ᵉ centile ≥ 10 (hors isolés voulus) ; 95ᵉ ≤ 150 |
| Amis par adulte | 3 à 15, médiane ≈ 7 |
| Coefficient de regroupement des amitiés | 0,2 à 0,4 |
| Assortativité par âge des amitiés | ≥ 0,5 |
| Part des amitiés entre quartiers | 20 à 40 % |
| Distance moyenne dans le graphe des connaissances | ≤ 5 |
| Composante géante | ≥ 98 % des adultes |
| Réciprocité de connaissance (A connaît B ⇒ B connaît A) | ≥ 90 % des liens au-dessus du degré « connaissance », sauf asymétries voulues (patron, notable) |

- **Isolés** : quelques personnes ont volontairement très peu de liens (personne âgée seule, nouvel arrivant, marginal), pour la variété.
- **Notoriété** : savoir qui est le maire ou le patron de l'usine est un **savoir**, pas une relation ; il est donné selon la notoriété de la personne connue.
- **Groupes** : la population inscrit chacun dans ses **groupes de départ** (§5.5) ; leur fonctionnement relève du domaine social.

### 5.4 Souvenirs de départ (décidé)

*Pourquoi : un petit nombre de souvenirs fondateurs, partagés entre ceux qui les ont vécus, suffit à donner de la profondeur sans dépasser le budget de mémoire de l'IA.*

- Chaque adulte reçoit de **10 à 30 souvenirs fondateurs** **[à calibrer]**, tirés de sa biographie et de ses relations : rencontre avec son conjoint, naissance de ses enfants, mort d'un parent, premier emploi, licenciement, dispute avec un voisin, service rendu par un ami, agression subie, victoire sportive. Les enfants en ont moins.
- Le reste de son passé est **déjà résumé** dans ses relations et ses savoirs, comme si la consolidation de la mémoire avait déjà eu lieu ([ia-pnj.md](ia-pnj.md), §4.3).
- **Événements partagés** : un événement vécu par plusieurs personnes (un mariage, une bagarre, un accident) est créé **une seule fois** et inscrit dans la mémoire de chaque participant, avec **son** émotion et **son** point de vue (le témoin, la victime et l'agresseur ne s'en souviennent pas de la même façon). *Pourquoi : deux personnages ne peuvent pas avoir des souvenirs incompatibles du même événement.*
- **Secrets** : une part des souvenirs sont des **secrets** (liaison, vol ancien, enfant caché, dette honteuse), connus d'un petit nombre de personnes ; ils alimentent les intrigues (§6.4). Ceux qui n'ont **aucune trace** dans la population visible (liaison, vol jamais découvert, dette honteuse) font partie de la **couche cachée**, tirée par monde (décidé, §1.6, E25) ; ceux qui en ont une (enfant caché inscrit dans les lignées, condamnation au casier) restent dans la population visible. *Pourquoi : un secret commun à tous les mondes d'une famille serait publié dès la première semaine.*
- **Situation en cours** : chacun commence avec ses **motivations** évaluées, 1 à 3 **projets** en cours, des **engagements** (rendez-vous, dettes, promesses) et son **agenda** ([ia-pnj.md](ia-pnj.md), §1.2, §2.1, §3.4). Une petite part des habitants (≈ 2 % **[à calibrer]**) est **à l'extérieur** au départ (vacances, déplacement, détention) et reviendra dans les premiers jours ou les premiers mois.
- **Détenus au départ** (décidé ; *pourquoi : leurs retours donnent dès les premiers mois des histoires de réinsertion, de vengeance ou de rivalité dans les bandes, [combat.md](combat.md), §9.3*) : **10 à 15 habitants** **[à calibrer]** commencent le monde **en détention** : quelques-uns en **détention provisoire** dans l'attente d'un procès (qui aura lieu au tribunal de la Cité dans les premiers mois), les autres condamnés, avec une **date de retour** calculée selon les règles du domaine combat (réductions de peine comprises, libération conditionnelle possible). Leur situation en ville est déjà celle que décrit [combat.md](combat.md), §9.2 : emploi perdu après un mois, conjoint seul au foyer, enfants confiés selon le §7.7, suppléant en place dans la bande, logement conservé ou perdu selon le ménage. Les détenus d'une même période se connaissent (relations de codétenus).

### 5.5 Groupes de départ (décidé)

*Pourquoi : le social décrit les groupes comme des données ([social.md](social.md), §6.1) mais en laisse la création initiale à la population ; une ville qui démarre sans clubs ni bandes devrait attendre des mois pour en voir naître.*

Le générateur crée les groupes selon les fiches de groupe du social, avec leurs **membres**, leurs **rangs** et, pour les bandes, leur **territoire** :

| Type | Création **[ordres de grandeur à calibrer]** |
|---|---|
| **Foyers, familles élargies, couples** | Directement issus des ménages et des lignées (§2.2, §5.2) |
| **Équipes de travail** | Issues de l'affectation des emplois et des organigrammes (§3.2, §3.4) |
| **Classes d'école** | Issues de l'affectation scolaire (§3.3) |
| **Cercles d'amis** | Pas créés explicitement : ils sont **détectés** par le social à partir des amitiés semées (§5.3) |
| **Associations et clubs** | ≈ 40 groupes : club de boxe du faubourg, club de tennis et golf de Hautevigne, club de football et ses supporters, centre social des Peupliers, comité des fêtes, parents d'élèves, association de commerçants, syndicats de l'usine et de l'hôpital, chorale, club de pêche des Quais, société de chasse (§3.5)… Environ **40 % des adultes** et la moitié des enfants sont membres d'au moins un groupe, selon leurs loisirs, leur profil (extraversion, bienveillance) et leur quartier. Chacun a un **bureau** (président, trésorier) tiré parmi ses membres les plus anciens et les plus investis |
| **Communautés religieuses** | La paroisse de l'église de la Cité et les autres lieux de culte ([ville.md](ville.md), §5.2) ; pratiquants réguliers et occasionnels, avec un responsable de culte (fiche d'ancrage) |
| **Bandes et réseau** | Voir ci-dessous |

**Bandes de départ** : **trois bandes et un réseau** (dans la fourchette de 2 à 4 de [social.md](social.md), §6.6) :

| Groupe | Territoire | Membres **[à calibrer]** | Activité principale |
|---|---|---|---|
| **Bande des Peupliers** | Halls et dalle des tours, terrain de sport, abords du petit centre commercial | ≈ 25, dont quelques adolescents comme petites mains | Point de vente de drogue, petits vols |
| **Bande du faubourg** | Quelques rues autour de la salle de boxe et des bars, arrière-salles | ≈ 15 | Recel, racket de quelques commerces, bagarres |
| **Bande des Forges** | Cité ouvrière, abords de la halte ferroviaire | ≈ 10 | Vols de véhicules et de pièces, trafics |
| **Réseau des Quais** | Entrepôts, friches et squats des Quais | ≈ 20 | Approvisionne les bandes, usure, blanchiment ; dirigé par le **parrain** (hors catégorie, §6.2) |

- Chaque groupe a une **hiérarchie** complète (chef, lieutenants, membres, petites mains) tirée selon les fiches de rôle du social et de l'économie (chef de réseau, revendeur, guetteur), avec des membres de profils compatibles (respect des règles bas, violence variable), jamais selon l'origine (§0).
- **Signature graphique** (décidé, [social.md](social.md), §6.6) : chaque bande et le réseau reçoivent à l'étape P9 leur signature (une **graine**, deux ou trois **couleurs**, un **style** de lettrage), tirée avec la graine de famille et gardée tant que le groupe existe ; deux groupes de départ n'ont jamais les mêmes couleurs dominantes. Elle fait partie de la population visible. *Pourquoi : les tags et les couleurs se voient dès le premier jour ; ils doivent être les mêmes dans tous les mondes d'une famille.*
- Leur **territoire** est donné en lieux et en cellules ([ville.md](ville.md), §8.1), marqué par des **tags** dessinés d'après cette signature ; les habitants du territoire démarrent avec la **crainte** et le **respect** qui correspondent à l'emprise de la bande (§5.3).
- Leurs **relations entre eux** sont semées : le réseau fournit les trois bandes ; la bande des Peupliers et celle du faubourg sont **rivales** (c'est l'une des intrigues de départ, §6.4).
- Les chefs sont des **fiches d'ancrage** (§6.3).

**Profil normatif des groupes** (décidé, [social.md](social.md), §6.1 ; [revues/signatures.md](../revues/signatures.md), G14) : chaque groupe de départ reçoit son **profil normatif**, c'est-à-dire le point ou la zone de l'espace du profil qui dit ce qu'il valorise ou condamne. Il est pris dans sa fiche de groupe (bande : loyauté, silence, force ; association caritative : bienveillance ; club sportif : effort, esprit d'équipe), puis ajusté vers le profil de ses dirigeants et de ses membres les plus anciens. La **culture d'une entreprise** est son profil normatif, tiré pour l'essentiel du profil de son gérant (§3.4). *Pourquoi : `appraise` juge les manquements d'un membre avec ce profil dès le premier jour, et un groupe dont les normes contrediraient tous ses membres serait incohérent.*

**Familles d'accueil** : une dizaine de ménages **[à calibrer]**, choisis parmi les profils très bienveillants et stables, ont la fiche de rôle **famille d'accueil** ([social.md](social.md), §7.3) ; quelques-uns accueillent déjà un enfant placé au départ, avec l'histoire correspondante dans la biographie de l'enfant et de ses parents.

### 5.6 État intérieur de départ : voix, pensées, conclusions, valeurs apprises (décidé)

*Pourquoi : la revue de l'intelligence des PNJ ajoute des composants bornés à chaque personnage ([ia-pnj.md](ia-pnj.md), §11 « Revue de l'intelligence (v0.8) » et §13) ; un habitant qui démarrerait avec ces composants vides n'aurait ni humeur, ni rancune, ni habitudes le premier jour, ce qui contredirait sa biographie.*

**Règle générale** (décidé, [ia-pnj.md](ia-pnj.md), §13) : la génération remplit chaque composant à partir de la **biographie**, des **souvenirs de départ** et des **relations** (§5.1 à §5.4), avec les **mêmes règles écrites en données** que le jeu, puis le rodage (§1.4) les met en mouvement. *Pourquoi : des règles uniques donnent des états de départ semblables à ceux que le jeu produira, et rien n'est tiré sans cause.*

| Composant ([ia-pnj.md](ia-pnj.md)) | État initial donné par la génération | Ce qu'y ajoute le rodage |
|---|---|---|
| **Voix** (§13.10) : registre, longueur des phrases, politesse, humour, tics, vocabulaire de métier | Tirée à l'étape P6 (§1.2) d'après l'**âge**, le **milieu** (quartier d'enfance, études, milieu des parents), le **métier** et le **profil**. L'**origine** ne donne que des détails de langue (langue parlée en famille, quelques expressions), jamais la politesse, l'humour ou la brutalité (§0, principe 5). Les retraités gardent le vocabulaire de leur ancien métier | Rien : la voix est stable |
| **Pensées** (§13.2), au plus 16 | Tirées des souvenirs **récents ou encore vifs** : deuil de l'année, licenciement, dispute en cours, naissance, emménagement, succès. Leur intensité décroît avec l'ancienneté du souvenir et la régulation émotionnelle. La plupart des adultes commencent avec **2 à 6 pensées** **[à calibrer]** | Les pensées des événements du rodage ; les plus anciennes s'estompent |
| **Humeur** (§13.2) | Calculée à partir des pensées, des besoins et du stress, jamais tirée seule : elle a toujours une cause | Recalculée en continu |
| **Conclusions** (§13.4) | Une **réflexion rétrospective** : les règles de la réflexion nocturne parcourent la biographie en une passe, avec les biais du profil. La certitude décroît avec l'ancienneté des faits, et certaines conclusions sont **fausses**, comme en jeu | Celles des réflexions nocturnes du rodage |
| **Thèmes de vie** (§13.4), au plus 3 | Tirés des faits forts et durables de la biographie : deuil d'un enfant ou d'un conjoint, rancune ancienne, ambition contrariée, grand amour, honte d'un acte. Environ **un tiers des adultes** en a au moins un **[à calibrer]**. Les fiches d'ancrage et les intrigues publiques (§6.3, §6.4) peuvent en imposer | Peuvent naître ou s'éteindre (pardon) |
| **Attitudes** (§13.4) | Opinions sur des personnes, des lieux, des commerces et des sujets publics (mairie, usine, bandes), tirées des relations, des groupes (syndicat, club, supporters) et des souvenirs | Évoluent avec la persuasion et les conclusions |
| **Valeurs apprises** (§13.6), au plus 32 | Pour ce que l'habitant fréquente depuis longtemps (domicile et quartier, lieu de travail, commerces habituels choisis selon la proximité, le budget et le profil, trajets, activités de loisir), proches de la **qualité réelle** de la chose, avec le bonus d'**habitude**. S'y ajoutent les **aversions** tirées des souvenirs (un parc où l'on a été agressé). La valeur vers laquelle elles reviennent reste la valeur par défaut du catalogue | Ajustées aux résultats réels : prix, attentes, durée des trajets |
| **Traits supposés** (§13.5) | Pour chaque relation, le vrai profil de l'autre plus un **écart** qui diminue avec la proximité et l'ancienneté du lien ; un menteur habile paraît plus honnête qu'il n'est | Corrigés par les actes observés |
| **Faits sensibles et déclarations** (§13.5), au plus 8 | Pour chaque secret **visible** (§5.4) : qui, selon l'intéressé, le sait (les vrais confidents, plus quelques croyances fausses), et la version donnée à ceux qui ont été trompés | Suivent les rumeurs du rodage |
| **Sujets échangés** (§13.10) | Pour les relations proches, 1 à 3 sujets récents **[à calibrer]**, tirés des projets et des événements en cours (« alors, cet entretien ? », « comment va ta mère ? ») | Rien en statistique ; ils se remplissent dès la vie en simulation complète ou simplifiée |

- **Couche cachée** (décidé, §1.6, E25) : les pensées, conclusions, faits sensibles et déclarations liés à un secret de la couche cachée (une liaison, une dette envers le réseau, un informateur qui craint d'être découvert) ne sont pas dans la population visible. Ils sont inscrits à l'étape W4 (« traces ») de la création du monde, puis mis en mouvement par le rodage propre. *Pourquoi : une peur ou une culpabilité commune à tous les mondes désignerait le secret.*
- **Naissances** (§7.2) : un nouveau-né reçoit sa voix à la naissance, d'après le milieu de ses parents et son profil ; l'âge la module ensuite, et le vocabulaire de métier s'ajoute au premier emploi. Il démarre sans conclusion, valeur apprise ni thème.
- **Arrivées** (§7.4) : un nouvel arrivant reçoit le même traitement à partir de sa biographie extérieure. Il connaît peu la ville : ses valeurs apprises portent sur ses premiers lieux (logement, travail), et ses pensées de départ comprennent celles du déménagement (fatigue, espoir, regret).
- **Bornes** : la génération respecte les plafonds de l'IA des PNJ (16 pensées, 3 thèmes, 8 faits sensibles, 32 valeurs **[à calibrer]**), soit environ 2 Ko par habitant ([ia-pnj.md](ia-pnj.md), §11).

---

## 6. Personnages notables, hors catégorie et faits à la main

### 6.1 Notables (décidé)

On appelle **notables** les habitants de palier 6 et plus (≈ 400 personnes) et ceux qui ont un **rôle public** quel que soit leur palier (élus, commissaire, directeur d'école, curé, journaliste, président de club). *Pourquoi : ce sont ceux que les joueurs remarqueront et dont les décisions touchent les autres ; ils méritent un soin particulier.*

- Leur biographie est plus détaillée (plus d'étapes, plus de souvenirs, plus de secrets).
- Ils ont davantage de **liens entre eux** : les notables d'une ville moyenne se connaissent (conseil municipal, club de Hautevigne, associations, dîners).
- Le générateur crée autour d'eux les **organisations** qu'ils dirigent (équipe municipale, direction de l'usine, état-major de la bande) en affectant leurs subordonnés.
- Les notables de palier 7 et plus sont des **personnages importants** ([ia-pnj.md](ia-pnj.md), §6.2).

### 6.2 Hors catégorie (décidé)

*Pourquoi : les critères du socle ([02-personnages.md](../02-personnages.md), §3.4) désignent des fonctions précises ; les fixer à la main garantit qu'elles existent dès le départ et qu'elles sont tenues par des personnages soignés.*

Environ **10 hors catégorie** (0,1 % de la population), tous **faits à la main** (§6.3), choisis parmi les fonctions qui répondent aux critères du socle (influence trop grande, ressources critiques, rôle indispensable) :

| Fonction | Critère du socle |
|---|---|
| Maire d'Orvenne | Influence trop grande |
| Propriétaire et dirigeant de l'usine principale (la plus grande fortune, ≈ 50 millions d'écus, §4.7) | Entreprise qui fait vivre la ville |
| Commissaire central (police de toute l'agglomération) | Forces armées |
| Procureur de la République (tribunal de la Cité) | Influence trop grande |
| Directeur de l'hôpital | Rôle indispensable |
| Parrain du réseau criminel des Quais | Influence trop grande, forces armées |
| Chef de la deuxième grande famille de Hautevigne (banque régionale, immobilier) | Influence trop grande, système financier |
| Député de la circonscription (domicile à Hautevigne, souvent à l'extérieur) | Influence trop grande |
| Directeur du port fluvial et de la logistique | Ressources critiques (transports, marchandises) |
| Directeur d'Orvenne Habitat (logement social) | Ressources critiques : logement de milliers d'habitants ([economie.md](economie.md), §14.1) |

**[liste à calibrer]**

- **Rang et critères** : le générateur leur donne une puissance qui les place en tête du classement. Si l'un d'eux tombe plus tard en dessous du seuil, les critères suffisent à le garder **non jouable** ; si un habitant dépasse le seuil sans répondre à un critère, il devient hors catégorie par le rang, comme le prévoit le socle.
- **Succession** : quand un hors catégorie meurt, part ou perd sa fonction, son remplaçant vient d'abord **de l'intérieur** (élection, promotion, prise de pouvoir dans la bande), selon les domaines social, économie et combat ; ce n'est qu'à défaut que le générateur crée une **arrivée** (§7.4), par exemple un nouveau directeur nommé de l'extérieur.
- **Commissaire central et procureur** (décidé, à la demande de [combat.md](combat.md), §5.1 et §16.4 ; *pourquoi : ce sont des fonctions nommées par l'État, où la promotion interne est la règle quand un candidat a le grade, et une nomination extérieure sinon*) : le **commissaire adjoint** (ou, pour le procureur, le plus ancien **substitut**) est promu s'il remplit les conditions (grade, ancienneté, absence de casier et d'affaire en cours) ; à défaut, ou si l'État le décide après une crise, un successeur **nommé de l'extérieur** arrive par le train (§7.4). Pendant l'intérim, le suppléant exerce la fonction ([social.md](social.md), §6.8). Le successeur devient hors catégorie par les critères du socle, comme son prédécesseur.

### 6.3 Personnages faits à la main : les fiches d'ancrage (décidé)

*Pourquoi : écrire entièrement 10 000 personnages est impossible, mais une soixantaine de figures écrites à la main donnent son caractère à la ville, comme les lieux faits main le font pour la carte ([ville.md](ville.md), §3.3).*

- Environ **60 personnages** **[à calibrer]** sont décrits par des **fiches d'ancrage** : la dizaine de hors catégorie, les autres élus et responsables, et des **figures de quartier** (le patron du bar du Pont-Vieux, la gardienne de la tour emblématique des Peupliers, l'entraîneur de la salle de boxe du faubourg, le vieux marinier des Quais, la boulangère de la Cité, les chefs de bande, une grande championne qui s'entraîne sur l'Ostrelle, une ancienne figure de la télévision retirée à Hautevigne…).
- Une fiche fixe **ce qui compte** et laisse le reste au générateur : rôle, lieu de vie et de travail, âge approximatif, bornes du profil, deux ou trois **relations clés** (avec d'autres fiches ou avec des personnes à créer : « un fils de 20 ans qui fréquente la bande »), une liste de **secrets possibles**, et les **intrigues** auxquelles il participe (§6.4).
- **Secrets des fiches** (décidé, E25) : la fiche propose trois ou quatre secrets possibles **[à calibrer]** ; chaque monde en tire un ou deux avec sa couche cachée (§1.6). *Pourquoi : un secret écrit une fois pour toutes figurerait dans la population solo, lisible par tous.*
- Les mêmes fiches servent aux deux populations initiales ; dans la population en ligne, noms, détails et placements sont tirés à nouveau (§9.1, S2).
- Le générateur crée ces personnages **en premier** (étape P1 du §1.2) et construit leur ménage, leur famille et leur entourage autour d'eux.
- Les fiches d'ancrage sont **jouables** comme les autres (sauf les hors catégorie), aux conditions habituelles.
- Elles ne leur donnent **aucun destin protégé** : une figure de quartier peut mourir, partir ou changer, comme tout le monde.
- **Rédaction** (décidé, A8) : les fiches d'ancrage, la chronique (§1.1) et les retouches (§9.1) sont écrites par une **session de contenus** pilotée par le responsable contenus, dans le paquet de contenus, puis acceptées par les seuls contrôles automatiques (§1.3) et `ts-data check`. Aucune étape n'attend l'action ni la relecture d'un humain. *Pourquoi : règle A8 ; un contenu écrit en données et contrôlé automatiquement n'a pas besoin d'un humain pour entrer dans la chaîne.*

### 6.4 Intrigues de départ (décidé)

*Pourquoi : un monde qui démarre sans tension met des semaines à produire des histoires ; quelques intrigues semées au départ donnent tout de suite des projets, des conflits et des objectifs aux personnages.*

- Une vingtaine d'**intrigues** **[à calibrer]** sont semées entre les personnages : rivalité pour la mairie aux prochaines élections, conflit social qui couve à l'usine, guerre de territoire entre deux bandes, héritage disputé dans une famille de Hautevigne, liaison entre deux notables, dette d'un commerçant envers le réseau, journaliste qui enquête sur le port.
- Une intrigue n'est **qu'un état de départ** : des relations, des souvenirs, des secrets et des projets placés dans les personnages concernés. Il n'y a **aucun scénario** : la suite dépend entièrement de l'IA et des joueurs.
- **Candidats et programmes** (décidé, [social.md](social.md), §6.11 ; G14) : pour la rivalité pour la mairie, et pour les autres élections à venir, la population désigne les **candidats de départ** (le maire sortant et un ou deux rivaux, fiches d'ancrage ou notables, §6.1) et donne à chaque programme un **profil de valeurs**, un point ou une zone de l'espace du profil, tiré du profil du candidat et des attentes de ses soutiens (quartier, groupes, intérêts). Les électeurs votent selon l'accord entre leurs convictions et ce profil. *Pourquoi : le vote du social a besoin de programmes comparables aux convictions des électeurs dès la première élection.*
- **Intrigues publiques et intrigues secrètes** (décidé, E25, S3) : les intrigues qui se voient (rivalité pour la mairie, conflit social à l'usine, guerre de territoire, héritage disputé, journaliste qui enquête) font partie de la population visible. Les **liaisons** et les **dettes secrètes** (liaison entre deux notables, dette d'un commerçant envers le réseau) font partie de la couche cachée : leurs participants sont tirés par monde parmi les candidats compatibles (§1.6). *Pourquoi : savoir à l'avance qui trompe qui ou qui doit de l'argent au réseau supprimerait l'enquête.*
- Le fonctionnement des intrigues (rumeurs, organisations, élections) relève du domaine social ; la population ne fait que les semer.

---

## 7. Renouvellement de la population

### 7.1 Vue d'ensemble (décidé)

*Pourquoi : des rythmes proches de la réalité, mais un peu moins de départs, donnent une ville vivante où l'on voit naître, mourir, arriver et partir, sans trop priver les joueurs des personnages qu'ils ont appris à connaître.*

Ordres de grandeur annuels pour 10 000 habitants **[à calibrer]** :

| Mouvement | Par an | En ligne (temps réel) |
|---|---|---|
| Naissances | ≈ 105 (10,5 ‰) | Environ une tous les 3 à 4 jours |
| Décès | ≈ 90 (9 ‰), morts violentes et accidents compris | Environ un tous les 4 jours |
| Départs définitifs | ≈ 400 (4 %), dont ≈ 130 étudiants | Environ un par jour, en pic en été |
| Arrivées | ≈ 385, pour compenser | Environ un par jour, en pic à la rentrée |
| Déménagements à l'intérieur de l'agglomération | ≈ 250 ménages | Quelques-uns par semaine |

Les départs réels d'une ville moyenne sont plus nombreux (de l'ordre de 6 à 8 % par an) ; le jeu en garde environ la moitié. En solo, les mêmes taux s'appliquent au **temps de jeu**, accélérations comprises.

### 7.2 Naissances (décidé)

**Le monde a de vraies naissances** (réponse à la question laissée ouverte par [ia-pnj.md](ia-pnj.md), §10). *Pourquoi : les personnages vieillissent avec le calendrier ([02-personnages.md](../02-personnages.md), §1.2) ; sans naissances, la ville vieillirait et les enfants qui deviendront un jour des adultes jouables viendraient tous de l'extérieur, sans aucun lien avec la ville.*

1. **Projet d'enfant** : un couple (ou une personne seule) peut former le projet « avoir un enfant », selon l'âge, la relation, la situation financière, le nombre d'enfants déjà présents et le profil ([ia-pnj.md](ia-pnj.md), §3.4). Une petite part des grossesses est **imprévue** **[à calibrer]**.
2. **Grossesse** : la conception est **abstraite**, rien n'est montré (décidé ; *pourquoi : aucun intérêt de jeu, et un sujet qu'il n'y a pas lieu de représenter*). La grossesse dure environ neuf mois de jeu : elle se remarque peu à peu, réduit progressivement l'endurance, ajoute des rendez-vous médicaux et conduit à un **congé** (domaine économie). L'entourage l'apprend et réagit (souvenirs, projets).
3. **Pas de fausse couche liée aux actions des joueurs** dans la première version (décidé ; *pourquoi : sujet trop douloureux pour en faire un enjeu de jeu*). Les violences contre une femme enceinte relèvent du domaine combat, qui les traite comme des violences graves.
4. **Naissance** à l'hôpital de Beaulieu (maternité), sauf rare exception. Le nouveau-né est créé par le générateur : sexe, **prénom** choisi par les parents (selon leur âge, leur milieu et leur profil), **potentiel** d'attributs hérité en partie des parents, **profil de départ** hérité en partie, puis malléable ([ia-pnj.md](ia-pnj.md), §8.5), **génome d'apparence** par hérédité limitée depuis ses parents officiels (§4.9) et **garde-robe** de bébé d'après le milieu des parents (§4.10).
5. Le ménage, son agenda et ses projets sont mis à jour (garde, congé parental, allocations familiales versées selon les règles de l'économie, besoin d'un logement plus grand).
6. La naissance est **déclarée** à l'état civil, et les parents envoient un **faire-part** et peuvent passer une **annonce** dans le journal local (§7.8).

Un joueur peut incarner un personnage enceint, à condition que celui-ci soit jouable : les effets de la grossesse s'appliquent normalement.

### 7.3 Décès naturels (décidé)

*Pourquoi : une mort annoncée par une santé qui décline laisse le temps aux proches, et aux joueurs qui incarnent ces personnages, de vivre la fin d'une vie, alors qu'une suppression au hasard serait incompréhensible.*

- Chaque personnage a un **risque de décès** calculé à partir de sa **vitalité de fond** (§4.4), de son âge, de ses maladies et de ses habitudes, calibré sur des tables de mortalité réalistes **[à calibrer]**.
- **La plupart des décès sont annoncés** : maladie déclarée, soins, hospitalisation ou entrée en maison de retraite, déclin, puis décès. Une part minoritaire (≈ 20 % **[à calibrer]**) est **soudaine** (crise cardiaque, accident vasculaire).
- Les **morts violentes et accidentelles** ne sont **pas tirées** par la population : elles naissent de la simulation (domaines combat et ville). Repères annuels, hors actes des joueurs **[à calibrer]** : **1 à 3 morts violentes intentionnelles** ([combat.md](combat.md), §10.9) et quelques morts accidentelles (route, chute, noyade). Les tables de mortalité naturelle sont donc calibrées sur le total visé (≈ 90) **moins** ces décès attendus, soit ≈ 85 décès naturels ; le réglage global (§7.6) compte tous les décès, quelle qu'en soit la cause.
- **Aucun décès d'enfant dans la première version** (décidé). Le domaine combat garantit déjà qu'aucun enfant ne meurt du fait d'un personnage ([combat.md](combat.md), §12.2) ; la population y ajoute que les **tables de mortalité des mineurs sont nulles** : un enfant peut tomber gravement malade, être hospitalisé longtemps et garder des séquelles, mais il guérit. Il n'y a ni mort subite du nourrisson, ni enfant mort-né. *Pourquoi : c'est le même choix que l'absence de fausse couche (§7.2) ; dans la réalité, ces décès seraient de l'ordre d'un tous les deux ou trois ans pour 1 800 enfants, ce qui ne change rien à la démographie, alors que le sujet est trop douloureux pour en faire un événement de jeu.*
- **Suites d'un décès** : constat (par un médecin, ou le médecin légiste en cas de mort suspecte, [combat.md](combat.md), §5.2), **avis de décès** et faire-part (§7.8), obsèques (église ou autre lieu de culte, cimetière), deuil des proches (souvenir marquant, baisse d'humeur, projets), **succession** (logement, biens, entreprise : domaine économie), **garde des enfants** mineurs (§7.7), poste libéré. Les familiarités des joueurs avec le défunt deviennent inutiles.
- Rappel du socle : la mort d'un personnage jouable est **définitive** ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §4). Un joueur qui incarne un personnage au moment de sa mort naturelle ne subit **aucune pénalité** s'il est resté fidèle à sa nature ([04-joueur.md](../04-joueur.md), §1.1).

### 7.4 Arrivées (décidé)

*Pourquoi : des arrivées choisies selon ce dont la ville manque gardent l'équilibre, et une arrivée physique, annoncée à l'avance, reste crédible pour les voisins.*

**Qui arrive** : le générateur crée un ménage selon les **besoins** relevés par le réglage global (§7.6) : logements vacants à occuper, postes non pourvus, tranches d'âge en déficit, part d'étudiants. Chaque arrivée a un **motif** :

| Motif | Exemple |
|---|---|
| Embauche ou mutation | Un infirmier recruté par l'hôpital, un nouveau professeur, un cadre muté à l'usine |
| Études | Étudiants de première année, en septembre (§7.5) |
| Rapprochement familial | Un parent âgé qui vient vivre près de ses enfants, un couple qui revient au pays : la personne est alors **tirée de la liste des proches à l'extérieur** d'un habitant (§8.3) |
| Nouveau départ | Une famille qui cherche un loyer moins cher, une personne seule qui change de vie |
| Remplacement d'une fonction | Un nouveau directeur, un nouveau médecin, un commissaire ou un procureur nommé, quand aucun habitant ne peut prendre le poste (§6.2) |
| **Dotation de l'État** | **Renforts de police** quand la criminalité monte durablement ([combat.md](combat.md), §5.1), postes d'enseignants à l'ouverture d'une classe, soignants : de vrais nouveaux habitants mutés, qui arrivent après le délai d'un recrutement **[à calibrer]**. Il n'y a jamais de renfort temporaire venu de l'extérieur |

**Comment** :

1. Le ménage est généré **complet**, par le même générateur que la population initiale (`PersonFactory`) : biographie à l'extérieur, profil, attributs, préférences d'attirance, santé, **génome d'apparence** avec son historique, **garde-robe**, goûts musicaux et instruments (§4.9 à §4.11), **patrimoine** (épargne, éventuellement le produit de la vente d'un logement à l'extérieur, dettes, reconstitués comme au §4.7 ; l'argent entre par le compte extérieur, [economie.md](economie.md), §2.2), proches à l'extérieur, et quelques **liens en ville** si son motif en donne (employeur, famille, ancien ami). Il démarre avec **peu de relations locales** ([social.md](social.md), §1.3).
2. Il trouve un **logement** à distance (agence immobilière, Orvenne Habitat, propriétaire habitant), ou une chambre à l'hôtel ou au foyer en attendant (domaine économie). Le propriétaire, l'agence et les voisins proches savent qu'**on va emménager** quelques jours à quelques semaines avant **[à calibrer]**.
3. Il **arrive physiquement** par une entrée de l'agglomération (train, route, avec un camion de déménagement), puis s'installe. Personne ne l'a vu avant ce moment : il n'a fait aucune visite préalable (décidé ; *pourquoi : une visite exigerait un personnage présent avant son arrivée, donc un figurant temporaire*).
4. C'est un **nouveau venu** : il connaît peu la ville et peu de gens (ce que l'interface en montre relève du domaine interface). Il est **jouable** aux conditions habituelles dès son arrivée ; son palier est souvent bas au début, faute de réseau.

### 7.5 Départs et autres mouvements de ménages (décidé)

*Pourquoi : la population possède le registre des ménages ; les décisions viennent de l'IA et les relations du social, mais quelqu'un doit tenir à jour qui vit avec qui et où.*

**Départs définitifs** : ils naissent des **projets** des personnages ([ia-pnj.md](ia-pnj.md), §9.4) : emploi ailleurs, rapprochement familial, retraite au soleil, fuite après un crime, échec, études finies.

- **Un départ s'annonce** (décidé) : sauf fuite, un projet de départ dure plusieurs semaines **[à calibrer]** (recherche d'emploi, préavis, cartons, adieux) et se voit (annonces aux proches, souvenirs, intuitions pour le joueur qui l'incarne). *Pourquoi : un joueur attaché à un personnage le voit venir, et peut même jouer ce départ ou, en l'incarnant, faire changer les choses.*
- **Aucune protection** liée aux joueurs : un personnage souvent incarné peut partir comme les autres (décidé). *Pourquoi : l'IA ne connaît pas les joueurs ([ia-pnj.md](ia-pnj.md), §0) ; protéger les favoris figerait le monde et pourrait être exploité.*
- Le partant **emporte son patrimoine** : ses biens sont vendus ou mis en location, et l'argent part vers le compte extérieur ([economie.md](economie.md), §2.2 et §14.4).
- Le personnage parti **quitte le monde** ([ia-pnj.md](ia-pnj.md), §9.4) ; il est **archivé** (pour l'historique et les souvenirs des autres) et devient un **proche à l'extérieur** de ceux qui lui étaient proches (§8). Il ne revient jamais comme personnage complet (décidé ; *pourquoi : cohérence avec ia-pnj.md §9.4, qui rend définitivement inutiles les familiarités avec lui*).

**Étudiants** : environ un tiers des étudiants partent chaque été (fin d'études) et sont remplacés à la **rentrée de septembre** par de nouveaux étudiants (§7.4). Ce cycle annuel est un des rythmes visibles de la ville.

**Mouvements à l'intérieur de la ville**, décidés par l'IA, tenus à jour par la population :

| Mouvement | Effet sur les ménages |
|---|---|
| Mise en couple, emménagement ensemble | Deux ménages fusionnent ; un logement se libère |
| Séparation, divorce | Un ménage se divise ; l'un des deux cherche un logement ; garde des enfants (souvent alternée) |
| Décohabitation | Un jeune adulte quitte ses parents (studio, colocation, résidence) |
| Déménagement interne | Logement plus grand, moins cher, plus près du travail, autre quartier |
| Accueil d'un parent âgé | Un ménage s'agrandit, ou la personne entre en maison de retraite |
| Expulsion, perte de logement | Hébergement chez un proche, au foyer, ou à la rue |

Les dynamiques de couple (rencontre, séparation) relèvent du domaine social ; le marché du logement relève de l'économie.

### 7.6 Le réglage global (décidé)

*Pourquoi : la population doit rester stable sans qu'on crée ni supprime personne d'autorité ; un régulateur qui ne fait que moduler des probabilités garde des départs et des arrivées naturels.*

Un **régulateur démographique** calcule chaque mois de jeu **[à calibrer]** quelques indicateurs et les compare à des cibles :

| Indicateur | Cible **[à calibrer]** |
|---|---|
| Nombre d'habitants | 10 000 ± 2 %, ou plus si la ville s'agrandit (ci-dessous) |
| Pyramide des âges | Celle du §2.1 ± quelques points par tranche |
| Taux de logements vacants | ≈ 4 à 5 % |
| Taux de chômage | ≈ 8 % |
| Postes indispensables vacants | Aucun au-delà de quelques semaines |
| Part d'étudiants | ≈ 4 % |

Il agit sur **trois leviers seulement** :

1. un **multiplicateur des projets de départ** ([ia-pnj.md](ia-pnj.md), §9.4), qui rend ces projets plus ou moins probables, borné entre 0,5 et 2 **[à calibrer]** ;
2. le **rythme des arrivées** ;
3. la **composition des arrivées** (âge, type de ménage, métiers recherchés).

Les arrivées **imposées** par un poste à pourvoir (fonction indispensable, dotation de l'État, §7.4) passent toujours : le régulateur les compte dans la population et compense, si besoin, par la composition des arrivées suivantes.

Règles :

- Il ne crée ni ne supprime jamais un habitant directement, et ne touche **ni aux naissances ni aux décès**, qui suivent leurs propres règles réalistes.
- Il réagit **lentement** (moyenne sur plusieurs mois, zone sans réaction autour de la cible) pour éviter les à-coups.
- **Les personnages persistants ne comptent pas** dans ses cibles ([ville.md](ville.md), §2.5) : ils occupent pourtant des logements et des postes, ce qui réduit naturellement la vacance. Le régulateur ne fait **jamais partir des habitants** parce que des joueurs arrivent : si les logements manquent, c'est l'économie qui lance des constructions ([ville.md](ville.md), §2.5 et §6.2).
- **Ville extensible** : la cible d'habitants suit la **capacité d'accueil** (logements et postes créés par les constructions), dans la limite d'un **plafond de calcul** fixé par serveur **[à calibrer]** ([01-concept.md](../01-concept.md), §1.1 : 10 000 habitants, extensible).

### 7.7 Enfants sans parent (décidé)

*Pourquoi : un enfant ne peut jamais rester seul ([ia-pnj.md](ia-pnj.md), §8.1), et sa prise en charge doit suivre la logique réelle : d'abord la famille.*

Quand un mineur perd ses parents (décès, départ forcé, détention, incapacité), il est confié, dans l'ordre : à l'**autre parent** ; à un **membre de la famille en ville** (grand-parent, oncle, tante, frère ou sœur majeur) ; à un **proche à l'extérieur**, et il **part** alors avec lui ; à une **famille d'accueil** habitante (§5.5). Les biens dont il hérite sont gérés par son tuteur jusqu'à ses 18 ans ([economie.md](economie.md), §5.11). La décision passe par les services sociaux (domaine social) ; la population met à jour les ménages.

### 7.8 État civil, listes électorales et annonces (décidé)

*Pourquoi : la justice a besoin de jurés ([combat.md](combat.md), §8.2), les élections d'électeurs ([social.md](social.md), §6.11), et l'interface montre les événements de vie par le courrier et le journal ([interface.md](interface.md), §15.5) ; tout cela suppose un registre tenu à jour au fil des naissances, décès, arrivées et départs, que la population possède déjà.*

**Registres** tenus par la population, consultables par les autres domaines (et, dans le monde, par les employés de mairie qui y ont accès) :

| Registre | Contenu | Mis à jour |
|---|---|---|
| **État civil** | Naissances, décès, mariages et pacs, divorces, filiation | À chaque événement (§7.2, §7.3 ; mariages et divorces décidés par le social) |
| **Domiciles** | Ménage et logement de chacun | Arrivées, départs, déménagements (§7.4, §7.5) |
| **Listes électorales** | Adultes inscrits dans leur commune : ≈ 90 % des adultes au départ **[à calibrer]**. Un nouvel arrivant s'inscrit dans les mois qui suivent (projet ordinaire, selon son profil) ; un jeune est inscrit d'office à 18 ans ; un décès ou un départ le retire | Arrivées, majorités, décès, départs |
| **Listes de jurés** | Tirées des listes électorales, hors personnages persistants (qui ne sont pas des habitants permanents, [ville.md](ville.md), §2.5) ; le tirage d'une session écarte ensuite les personnes liées à l'affaire ([combat.md](combat.md), §8.2) | Une fois par an **[à calibrer]**, puis à chaque session |

**Annonces des événements de vie** (à la demande de [interface.md](interface.md), §15.5) : chaque événement produit des **objets de courrier** et un **contenu pour le journal local**, émis par la population au moment de l'événement :

| Événement | Courrier | Journal local |
|---|---|---|
| **Naissance** | **Faire-part** envoyé par les parents à leurs proches (famille, amis, relations au moins « proches » en ville) | **Annonce de naissance** (prénom, parents, date), si les parents la passent |
| **Décès** | **Faire-part de décès** envoyé par la famille aux proches du défunt, avec le lieu et la date des **obsèques** | **Avis de décès** (nom, âge, date et lieu des obsèques), aussi affiché à la porte du lieu de culte |
| **Mariage, pacs** | **Faire-part** envoyé par les mariés | Annonce dans la rubrique du carnet, si les mariés le souhaitent |

- **Qui envoie** : le ménage concerné (ou la famille du défunt) ; envoyer un faire-part ou passer une annonce est un geste ordinaire de son agenda, plus ou moins probable selon le profil (un couple très discret peut s'en passer) et son coût (l'annonce est payante, domaine économie). Par défaut, l'avis de décès est **toujours** publié, par la famille ou à défaut par la mairie. *Pourquoi : un décès doit être connu en ville pour que les obsèques réunissent du monde et que les joueurs l'apprennent.*
- **Destinataires** : les habitants qui ont une relation de proche ou d'intime avec l'émetteur ou le défunt, la famille en ville, et les **voisins de palier** pour un décès. Les proches à l'extérieur sont prévenus de façon abstraite (§8.2).
- **Ce que l'annonce provoque** : un **savoir** chez chaque lecteur (« Mme Roux est décédée ; obsèques jeudi à 10 h à l'église de la Cité »), un **engagement** possible (assister aux obsèques, rendre visite à la maternité) selon la relation, et une circulation de la nouvelle par la rumeur ([social.md](social.md), §4).
- Le **journal local** est une entreprise de presse de la ville ([interface.md](interface.md), §15.5) : la population lui fournit le contenu, l'entreprise le publie dans le numéro suivant.

---

## 8. Les proches à l'extérieur

### 8.1 Principe (décidé)

*Pourquoi : chacun a de la famille et des amis ailleurs ; sans eux, les visites, les deuils lointains, les héritages et les rapprochements familiaux n'auraient pas de source, et simuler ces gens complètement serait inutile.*

Chaque habitant a une liste de **proches à l'extérieur** : des **relations abstraites**, qui ne sont pas des personnages simulés ([ia-pnj.md](ia-pnj.md), §9.1).

| Donnée | Exemple |
|---|---|
| Identité | Prénom, nom, âge, sexe |
| Lien | Mère, frère, fille, ancien collègue, ami d'enfance |
| Éloignement | Région voisine, autre région, étranger (règle la durée et le coût d'une visite) |
| Relation | Affection, fréquence des contacts (comme une relation ordinaire) |
| État | Vivant ou décédé, en bonne santé ou malade, situation (marié, enfants…) |

- Environ **3 à 10 proches à l'extérieur** par adulte **[à calibrer]** : les membres de la famille proche qui ne vivent pas en ville (§5.2), et quelques amis. Les membres d'une même famille partagent les mêmes proches à l'extérieur (une même grand-mère pour toute la fratrie).
- Ils sont **légers** : quelques valeurs par proche, aucun agenda, aucune position.

### 8.2 Ce qu'ils font (décidé)

- **Contacts à distance** : appels et messages, résolus de façon abstraite ; ils satisfont un peu le besoin de **lien social** et entretiennent la relation.
- **Visites** : la motivation « relations » envers un proche à l'extérieur fait naître des **séjours à l'extérieur** ([ia-pnj.md](ia-pnj.md), §9.1), avec leurs effets au retour.
- **Nouvelles** : à faible fréquence, un proche **vit des événements** tirés au sort selon son âge (mariage, naissance, maladie, décès, difficultés) **[taux à calibrer]**. Ils produisent des **visites** (mariage, enterrement), des **projets** (aider un parent malade, se rapprocher), des **héritages** (domaine économie) ou des **deuils**.
- **Les proches à l'extérieur ne viennent jamais en ville** en visite (décidé). *Pourquoi : ils ne sont pas des personnages complets, et les faire apparaître le temps d'une visite créerait une personne inventée visible, contraire à la règle du socle ([01-concept.md](../01-concept.md), §1.1) ; les visites se font donc toujours dans l'autre sens.*
- Cette règle n'interdit pas une **demande extérieure abstraite**, sans personne visible : chambres d'hôtel réservées par des voyageurs non simulés, marquées occupées et fermées à clé, groupes extérieurs du restaurant gastronomique et du golf, commandes de l'usine ([economie.md](economie.md), §4.4 ; [01-concept.md](../01-concept.md), §1.1). Ces voyageurs ne sont **ni des habitants ni des proches à l'extérieur** : ils n'ont ni nom, ni relation, ni présence.

### 8.3 Passage d'un monde à l'autre (décidé)

- **Un habitant qui part** devient un proche à l'extérieur pour ses proches restés en ville (§7.5), avec ses vraies données (nom, âge, relation).
- **Un proche qui emménage** en ville (rapprochement familial, §7.4) devient un **personnage complet**, généré pour être **cohérent** avec ce que ses proches savent déjà de lui (âge, lien, histoire commune, souvenirs partagés). *Pourquoi : c'est la façon la plus naturelle de faire arriver des gens qui ont déjà des liens en ville.*

---

## 9. Mondes différents et versions

### 9.1 Deux populations initiales, une couche cachée par monde (décidé)

*Pourquoi : les habitants sont liés aux logements et aux postes de la carte initiale commune ([01-concept.md](../01-concept.md), §1.1) ; mais la population solo est lisible dans le client, et ce qu'on apprend en solo ne doit révéler les secrets d'aucun serveur (arbitrage A1 de la session principale ; [revues/securite.md](../revues/securite.md), S2, S3 ; [revues/equilibrage.md](../revues/equilibrage.md), E25).*

- Tous les mondes partent de la **même carte initiale**.
- **Deux populations initiales** (décidé, S2), produites par la même chaîne `ts-worldbuild` :
  - la **population solo**, publique et livrée avec le jeu : toutes les parties solo en partent ;
  - la **population en ligne**, produite avec les mêmes tables cibles et les mêmes fiches d'ancrage mais une autre graine (noms, détails et placements tirés à nouveau), validée de la même façon (§1.3) et **jamais livrée au client** : elle n'existe que sur les serveurs, dans une zone à accès restreint. Tous les serveurs en partent.
- **Une famille, une population visible** (décidé, E25) : tous les mondes d'une même famille (solo ou en ligne) partagent la même **population visible** (personnes, ménages, logements, emplois, profils, liens connus).
- **Une couche cachée par monde** (décidé, E25, S3) : à sa création, chaque monde tire avec sa propre graine sa couche cachée, puis fait son rodage propre de 14 jours (§1.6).
- Ce que les deux familles ont en commun (nombre de bandes, fonctions des hors catégorie, rôles des fiches d'ancrage) est public ; le joueur solo sait qu'il existe un parrain ou un commissaire en ligne, pas qui ils sont ni où ils vivent.
- **Retouches** (décidé) : les retouches des sessions de contenus (§0, §6.3) sont écrites comme des contraintes des tables cibles ou des fiches d'ancrage, jamais sur une personne générée. *Pourquoi : une retouche faite sur un habitant de la population solo n'aurait pas d'équivalent dans la population en ligne.*
- Les deux populations sont **versionnées** ensemble, comme la carte ([ville.md](ville.md), §4.3) : une version = les tables cibles, les fiches d'ancrage, les retouches, une graine par population et les deux résultats après rodage. Une mise à jour s'applique aux deux.
- **Protection de la famille en ligne** (décidé, [revues/generation.md](../revues/generation.md), P25, P29, P30) :
  - sa **graine** est **secrète**, sur **128 bits**, tirée par un générateur cryptographique et gardée en zone 2 ; sans elle, le code public ne permet pas de reconstruire la population en ligne ;
  - elle n'est **construite qu'en zone 2**, sur une machine de construction contrôlée, avec un **cache privé** ; l'intégration continue publique et le cache public (carte, dérivés, famille solo) ne la construisent jamais et n'ont jamais sa graine ;
  - l'artefact est découpé en **parts adressées par empreinte** (`pop.solo` publique, `pop.online` en zone 2 seulement, rapport privé de la famille en ligne) ; un **contrôle automatique** de la chaîne de publication vérifie qu'aucune empreinte de `pop.online` ni du rapport privé n'apparaît dans le manifeste du **paquet client**. *Pourquoi : S2 tient alors par une vérification, pas par une discipline.*
- **Fuite** (décidé, S2) : si la population en ligne fuit, les **nouveaux** serveurs partent d'une nouvelle version (nouvelle graine) ; les serveurs existants ont déjà divergé et gardent leur couche cachée.
- Chaque monde évolue ensuite de son côté ; son état est enregistré dans PostgreSQL comme le reste ([05-architecture.md](../05-architecture.md), §2.8), ou dans la sauvegarde pour une partie solo.

### 9.2 Date de départ d'un monde (décidé)

*Pourquoi : deux serveurs ouverts à des mois d'intervalle, ou deux parties solo commencées en mars et en novembre, doivent avoir les mêmes habitants visibles, avec les mêmes âges.*

- Toutes les dates de la population initiale (naissances, étapes des biographies, souvenirs, engagements) sont stockées **par rapport au jour de départ** (« né il y a 34 ans et 120 jours »).
- À la création d'un monde, elles sont converties en **dates du calendrier** de ce monde. Chaque habitant a donc le même âge le premier jour, quel que soit ce jour.
- Le **niveau scolaire** des enfants est stocké tel quel au départ (pas recalculé à partir de la date de naissance), puis avance normalement à chaque rentrée.
- Les **vacances scolaires**, saisons et projets saisonniers (vacances d'été) se placent selon le calendrier du monde : un monde qui démarre en juillet a des habitants en vacances dès les premiers jours.

### 9.3 Mises à jour de la population initiale (décidé)

*Pourquoi : c'est la même règle que pour la carte ([ville.md](ville.md), §4.5) : l'histoire d'un monde l'emporte toujours sur une nouvelle version.*

- Une nouvelle version de la population initiale ne **modifie jamais** les habitants d'un monde existant : ils ont vécu depuis, et les changer ferait disparaître des histoires et des familiarités.
- Un **nouveau personnage fait à la main** peut entrer dans un monde existant **par une arrivée** (§7.4), si son rôle y a encore un sens (un poste à pourvoir, une famille à rejoindre) ; sinon il n'existe que dans les nouveaux mondes.
- Les **refontes** (tables cibles, nouvelles fiches d'ancrage nombreuses) ne s'appliquent qu'aux **nouveaux mondes**.

### 9.4 Une population par serveur, puis par monde (décidé)

*Pourquoi : la population en ligne privée montre qu'une même carte peut accueillir plusieurs populations ; aller jusqu'à une population par serveur ne dépend donc que de la validation automatique, alors qu'une population par monde avec sa propre carte attend la ville par monde.*

- **Étape 1, la population en ligne privée** (§9.1, S2) : prête avant toute ouverture publique d'un serveur. C'est la première étape de la population par monde.
- **Étape 2, une population par serveur** sur la carte commune (décidé, E25) : prévue plus tard, en priorité pour les serveurs en ligne, dès que la **validation automatique** (§1.3) suffit sans aucune retouche ; chaque nouveau serveur tire alors sa population visible avec sa propre graine.
- **Étape 3, une population par monde avec sa ville** : en même temps que la génération d'une ville par monde ([ville.md](ville.md), §4.6), et dans le même ordre : d'abord en solo après une réinitialisation du compte, puis pour les nouveaux serveurs.
- Conditions des étapes 2 et 3 : la **validation automatique** (§1.3) suffit, sans aucune retouche ; les **fiches d'ancrage** sont placées par le générateur dans des quartiers et des logements compatibles, avec des noms et des détails tirés à nouveau, pour que deux mondes n'aient pas les mêmes figures.
- **Chaîne embarquable** (décidé, P40) : `ts-worldbuild` est une **bibliothèque Rust** sans Python, sans réseau ni service externe, à mémoire bornée (≤ 4 Go), avec validation et tentatives automatiques. La chaîne d'une famille doit tourner en **≤ 3 minutes** sur un serveur pour l'étape 2, et sur la machine solo pour l'étape 3 **[à mesurer]**. *Pourquoi : c'est la condition pour générer une population au lancement d'un serveur ou d'une partie, sans passer par la machine de construction.*
- Le même mécanisme permet de produire, pour le développement et l'entraînement des IA, **autant de populations de test que nécessaire**, de toutes tailles (§1.5).

---

## 10. Ce que la population fournit aux autres domaines

### 10.1 À l'économie

| Donnée | Contenu |
|---|---|
| **Main-d'œuvre** | Pour chaque adulte : situation (en emploi, chômeur, étudiant, retraité, inactif), métier du catalogue et poste, ≈ 12 % d'emplois de nuit, qualifications et compétences, ancienneté, disponibilités (horaires possibles, chronotype), mobilité (permis, véhicule) (§3.2) |
| **Ménages consommateurs** | Taille, âges, enfants, revenus, statut d'occupation, équipement (véhicules), habitudes (tabac, sorties, dépendances) |
| **Patrimoines et dettes** | Comptes, espèces, biens, parts d'entreprise, prêts en cours, ménages en difficulté, dettes entre habitants (§4.7) |
| **Registre de propriété initial** | Propriétaire de chaque logement, local et entreprise : propriétaires occupants, Orvenne Habitat, non-résidents, habitants bailleurs (§3.1, §3.4) |
| **Entreprises viables** | Forme, propriétaires, gérant, organigramme, marge attendue positive vérifiée, quelques entreprises en difficulté voulues (§3.4) |
| **Héritiers** | Lignées en ville et proches à l'extérieur, pour les successions (§5.2, §8) |
| **Revenus venus de l'extérieur** | Navetteurs sortants (§3.2), pensions de retraite, bourses d'études, aides sociales |
| **Postes vacants et recrutement extérieur** | Les postes que la population ne peut pas pourvoir de l'intérieur donnent lieu à une **arrivée** (§7.4) |
| **Événements de vie** | Naissances (congés), décès (succession, obsèques), séparations (logement), arrivées (logement réservé à distance), départs (logement et poste libérés) |

### 10.2 Au social

| Donnée | Contenu |
|---|---|
| **Réseau initial** | ≈ 600 000 relations complètes et des relations compactes, orientées, avec affection, confiance, **respect** et **crainte** distincts, bilan et dernier contact (§5.3) |
| **Familles** | Lignées, couples, ex-conjoints, familles brouillées, gardes alternées (§5.2) |
| **Préférences d'attirance** | Pour chaque adulte (§4.8) |
| **Groupes de départ** | ≈ 40 associations et clubs, communautés religieuses, trois bandes et un réseau avec hiérarchie et territoire, familles d'accueil (§5.5) |
| **Souvenirs et secrets** | Événements partagés, secrets connus de quelques-uns (§5.4) |
| **Intrigues de départ** | Une vingtaine de situations de tension semées entre notables et habitants (§6.4) |
| **Mouvements** | Nouveaux venus à accueillir, départs, deuils, naissances : autant d'occasions pour le réseau d'évoluer |

### 10.3 À l'IA des PNJ

Des personnages complets et prêts à vivre (§5.4), avec leur **état intérieur** de départ : voix, pensées, humeur, conclusions, thèmes de vie, attitudes, valeurs apprises, traits supposés, faits sensibles et sujets échangés (§5.6) ; le **réglage global** des projets de départ (§7.6) ; de nouveaux projets et états à prendre en charge (projet d'enfant, grossesse, fin de vie, emménagement, séparation, décohabitation, accueil d'un parent, envoi d'un faire-part, présence aux obsèques).

### 10.4 Au combat, à la police et à la justice

| Donnée | Contenu |
|---|---|
| **Effectifs** | ≈ 80 habitants pour la police, la justice et les contrôles, dont le commissaire central et le procureur (§3.5, §6.2) |
| **Armes** | ≈ 80 chasseurs déclarés avec permis, ≈ 10 à 15 armes illégales dans le milieu (§3.5) |
| **Milieu** | ≈ 2 % des adultes, trois bandes et un réseau avec hiérarchie et territoire (§3.2, §5.5) |
| **Casiers** | ≈ 5 % des adultes avec un casier non effacé, ≈ 1 % d'anciens détenus (§5.1) |
| **Détenus au départ** | 10 à 15 habitants, avec date de retour, dont quelques-uns en attente de procès (§5.4) |
| **Listes de jurés** | Tirées des listes électorales, hors personnages persistants (§7.8) |
| **Recrutement** | Successeurs et renforts de police par de vraies arrivées (§6.2, §7.4) |
| **Couche cachée** | Par monde : informateurs et policiers traitants, policiers corruptibles, contenu des planques et des caches d'armes et ceux qui les connaissent ; leur emplacement et l'état des alarmes viennent de la part serveur de la ville (§1.6) |

### 10.5 À l'interface

Les **faire-part**, **avis de décès** et **annonces de naissance** comme objets de courrier et contenus du journal local (§7.8) ; les cartons et le camion d'un déménagement (§7.4, §7.5).

### 10.6 À la direction artistique

Le **génome d'apparence** et la **coiffure** de départ de chaque habitant, avec leurs règles de vieillissement (§4.9), et sa **garde-robe abstraite** (§4.10). Le client n'y ajoute aucune variation ; l'éditeur du persistant lit les mêmes plages et les mêmes catalogues (§4.9). L'**historique daté** de la coiffure, de la tenue habituelle et des signes distinctifs, pour les portraits de l'interface (§4.9) ; les **goûts musicaux** et les **instruments**, sources de la musique entendue (§4.11).

---

## 11. Points ouverts

Aucune question de conception propre à la population ne reste ouverte. Restent :

- les valeurs **[à calibrer]** : pyramide, ménages, taux d'emploi, nombre de relations et de souvenirs, taux de naissances, de décès et de départs, bornes du régulateur, nombre de fiches d'ancrage et d'intrigues, seuil de convergence et bornes du rodage de famille (14 à 60 jours), durée du rodage propre (14 jours), nombre maximal de tentatives de la couche cachée (8), nombre de postes candidats par actif (50), plages des métriques du réseau, nombre d'événements de la chronique, nombre minimal de candidats par élément caché, nombre de secrets possibles par fiche d'ancrage, nombre de pensées et de sujets échangés au départ, part des adultes ayant un thème de vie, part héritée des traits de visage, seuil anti-sosies, seuils de dispersion des apparences par quartier, aire sous la courbe du test de parenté (0,70), plancher de probabilité des champs du génome, courbes de vieillissement, nombre d'entrées de l'historique d'apparence (8), âge des goûts musicaux propres (10 ans), parts de musiciens, nombre de musiciens de bar et de rue au départ, valeurs de remplacement des garde-robes par classe, part des ménages sans lave-linge ; elles s'ajusteront au prototype du cœur, puis à la simulation de la ville entière ;
- les décisions qui doivent être confirmées par le socle ou par d'autres domaines (§12).

---

## 12. Impacts sur le socle et les autres domaines

À trancher par la session principale ou par la session du domaine concerné ; rien n'a été modifié ailleurs.

### 12.1 Socle

| Fichier | Impact | État |
|---|---|---|
| [02-personnages.md](../02-personnages.md), §1.2 | Le monde a de **vraies naissances** et des **décès naturels** annoncés par le déclin de la santé (§7.2, §7.3) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4 | Les hors catégorie sont **faits à la main** au départ (≈ 10 fonctions, §6.2) et les **critères** les gardent non jouables même si leur rang baisse | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1 | « Aucun figurant » veut dire **aucune personne inventée visible** : les proches à l'extérieur ne viennent jamais en ville, les nouveaux venus arrivent pour s'installer, et une **demande extérieure abstraite** reste possible sans présence visible (§8.2 ; [economie.md](economie.md), §4.4). « Extensible » veut dire que la population suit la capacité d'accueil, dans la limite d'un plafond de calcul par serveur (§7.6) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1 | Tous les mondes partent de la même **population initiale**, comme de la même carte ; les dates sont relatives au jour de départ du monde (§9.1, §9.2) | Intégré au socle le 29/10 ; **remplacé** en v0.4 par la ligne suivante |
| [04-joueur.md](../04-joueur.md), §6 | La sélection de départ est tirée parmi les **adultes de palier 0**, dont la population garantit qu'ils couvrent toutes les catégories comportementales (§4.6) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 | **Générateur de population** en Rust, à côté du générateur de ville : déterministe par graine, utilisé pour la population initiale, les naissances, les arrivées et les populations de test ; population initiale **versionnée** comme la carte (§0, §9) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §5 | Le prototype du cœur utilise le **mode « morceau de ville »** du générateur pour peupler le quartier du prototype (§1.5) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1 et §7.1 | Remplace la ligne « même population initiale » ci-dessus : même carte pour tous les mondes ; **deux populations initiales**, solo publique et en ligne jamais livrée au client ; même population visible par famille, **couche cachée** tirée par monde et rodage propre de 14 jours ; extraction des données du client réglée ainsi (§1.6, §9.1 ; E25, S2, S3) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 (`ts-worldbuild`, générateur) | `ts-worldbuild` produit **deux** populations initiales ; l'artefact en ligne reste en zone serveur et n'entre jamais dans le paquet du client ; la **création d'un monde** comprend l'étape à graine de la couche cachée puis le rodage propre (§1.6 ; S2, S3, E25) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.8 | La couche cachée est enregistrée avec l'état du monde (PostgreSQL, ou sauvegarde solo) (§1.6) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.2 ; [02-personnages.md](../02-personnages.md), §1.1 | Le générateur produit le **génome d'apparence** et la **garde-robe abstraite** ; l'éditeur du persistant n'offre que l'espace que la population peut produire (§4.9, §4.10) | Déjà à jour d'après la session principale (05 §2.2 : génome d'apparence ; 02 §1.1 : éditeur du persistant) |
| README, revue d'architecture finale | Le générateur de population, déterministe et capable de produire des populations de toutes tailles, répond à la contrainte « possibilité de lancer de nombreuses simulations différentes » (§9.4) | Intégré au socle le 29/10 ([05-architecture.md](../05-architecture.md), §2.14 : populations de test de toutes tailles, mode « morceau de ville ») |

### 12.2 Domaine ville

Transmis à la session ville par la session principale le 29/10.

- Ajouter une **maison de retraite** (≈ 70 places, Beaulieu près de l'hôpital) et une **maternité** à l'hôpital (§3.1, §7.2).
- Confirmer les capacités d'hébergement collectif : **résidences étudiantes** ≈ 190 places, **foyer de travailleurs** ≈ 60 places dont quelques chambres libres pour les nouveaux venus, **squats** pour une dizaine de sans-abri (§3.1).
- Logements ordinaires : réglé par la revue de la génération (P41) : vacance de **5 %**, soit ≈ 4 740 logements pour ≈ 4 500 ménages, dimensionnés par le programme à partir des tables cibles (§3.1).
- Le **mélange des types de logements** par quartier doit suivre les ménages du §2.3 : petits logements dans la Cité, à la Gare et à Beaulieu, grands logements familiaux aux Peupliers et aux Forges, villas à Hautevigne.
- Préciser s'il existe une **école privée** élémentaire (§3.3).
- Les **postes de travail** du registre doivent indiquer un **palier typique** (déjà prévu, [ville.md](ville.md), §9.1) et marquer les postes **indispensables** (§1.3, §3.2).
- Les arrivées et départs passent par la **gare**, la **halte des Forges** et les **routes** : prévoir de quoi garer un camion de déménagement (§7.4).

### 12.3 Domaine IA des PNJ

- **Naissances** : la question de [ia-pnj.md](ia-pnj.md), §10 est tranchée : oui, de vraies naissances (§7.2).
- Nouveaux **projets** à prévoir dans les recettes : avoir un enfant, emménager ensemble, se séparer, quitter ses parents, déménager dans la ville, accueillir un parent âgé, aider un proche malade à l'extérieur (§7.2, §7.5, §8.2).
- Nouveaux **états et fiches de rôle** : grossesse, nourrisson (déjà prévu), fin de vie et soins palliatifs, résident de maison de retraite, étudiant, sans-abri, navetteur sortant (absent chaque jour ouvré, §3.2), enfant en garde alternée (deux domiciles, §2.2).
- Les **projets de départ** doivent durer plusieurs semaines et être visibles, sauf fuite (§7.5) ; le **multiplicateur** du régulateur s'applique à leur probabilité (§7.6).
- Les **visites** aux proches à l'extérieur vont toujours de la ville vers l'extérieur (§8.2) ; les contacts à distance (appels, messages) satisfont un peu le lien social.
- Le **rodage** (§1.4) utilise la simulation statistique de l'IA pendant environ 60 jours de jeu, sans naissances, décès ni mouvements.
- **Revue de l'intelligence (v0.8)** : la demande de [ia-pnj.md](ia-pnj.md), §11 (état initial des nouveaux composants) est **traitée dans ce fichier** (v0.5, §5.6). Restent côté IA des PNJ :
  - permettre à la génération d'appliquer les **règles de la réflexion nocturne** en une passe sur la biographie (réflexion rétrospective) ;
  - fournir le **catalogue des pensées** et la **valeur par défaut** des choses du catalogue des valeurs apprises ;
  - accepter que le **vocabulaire de métier** de la voix s'ajoute au premier emploi et change avec le métier, alors que le reste de la voix est tiré une fois pour toutes (§5.6).
- **Reliquat de [ia-pnj.md](ia-pnj.md) v1.1** (§2.6, §11) : **traité** en v0.8.1. Garde-robe abstraite et état de départ de ses pièces (§4.10) ; coiffure voulue et longueur de départ étalée sur le cycle du coiffeur (§4.9) ; goûts musicaux, instruments possédés et compétence musicale (§4.11). Reste côté IA des PNJ : faire évoluer les goûts musicaux avec la vie, et tirer la coupe voulue après un événement marquant à partir de la coupe de départ.

### 12.4 Domaine économie

Les demandes de [economie.md](economie.md), §14.4 sont **traitées dans ce fichier** (v0.2) : métiers du catalogue et ≈ 12 % d'emplois de nuit (§3.2), chômage de départ à ≈ 8 % (§3.2), propriété (§3.1), patrimoines, revenus et dettes (§4.7), entreprises, propriétaires et contrôle de viabilité (§3.4), liens familiaux pour les successions (§5.2, §8), patrimoine apporté par les arrivants et emporté par les partants (§7.4, §7.5), allocations familiales à la naissance (§7.2). Restent côté économie :

- Fournir au générateur les **prix, salaires et taux** utilisés pour reconstituer les patrimoines (§4.7) et estimer la viabilité (§3.4), ainsi que l'**automate de gestion** qui tourne pendant le rodage (§1.4).
- **Revenus venus de l'extérieur** : salaires des ≈ 150 navetteurs sortants, pensions de ≈ 2 100 retraités, bourses, aides (§3.2).
- ≈ 100 **postes vacants** au départ ; une offre d'emploi non pourvue longtemps déclenche un **recrutement extérieur**, donc une arrivée (§7.4).
- **Congés** de maternité et de paternité, **frais d'obsèques**, **successions** et héritages (y compris venus de proches à l'extérieur), **logement réservé à distance** par les nouveaux venus (§7.2 à §7.4, §8.2).
- **Orvenne Habitat** (≈ 950 logements), **propriétaires non résidents** (≈ 700) et **habitants bailleurs** (≈ 800) (§3.1).
- L'hôtel n'a **aucun visiteur visible** venu de l'extérieur : sa clientèle extérieure est la **demande abstraite** décidée par l'économie ([economie.md](economie.md), §4.4) ; s'y ajoutent les personnages persistants, les nouveaux venus en attente de logement et les habitants (§7.4, §8.2).

### 12.5 Domaine social

Les points ci-dessous sont repris par [social.md](social.md), §12.3. Ses demandes à la population sont **traitées dans ce fichier** (v0.2) : graphe initial avec historique, respect et crainte distincts (§5.3), groupes de départ dont trois bandes et un réseau avec hiérarchie et territoire (§5.5), préférences d'attirance (§4.8), familles d'accueil (§5.5, §7.7), nouveaux arrivants avec peu de relations locales (§7.4). Restent côté social :

- Fournir les **fiches de groupe** (associations, clubs, bandes) et les **fiches de rôle** de groupe (président, chef, lieutenant, famille d'accueil) dont le générateur se sert (§5.5).
- Faire **évoluer** le réseau initial (§5.3) : dynamique des relations, couples (rencontres, séparations), amitiés, conflits.
- Faire vivre les **intrigues de départ** et les **groupes** semés par la population (§6.4).
- **Succession** des notables et des hors catégorie : élections, promotions, prises de pouvoir dans les bandes (§6.2).
- **Services sociaux** et **familles d'accueil** pour les enfants sans parent (§7.7).
- **Accueil des nouveaux venus** et réactions aux départs et aux deuils (§7.3 à §7.5).

### 12.6 Domaine combat, police et justice

Les demandes de [combat.md](combat.md), §16.4 sont **traitées dans ce fichier** (v0.3) : effectifs de police, de justice et des métiers associés (§3.5) ; armes au départ (§3.5) ; milieu criminel (§3.2, §5.5), casiers et anciens détenus (§5.1), habitants en détention avec une date de retour (§5.4) ; morts violentes comptées dans la mortalité (§7.3) ; aucun enfant ne meurt, ni du fait d'un personnage ni de cause naturelle dans la première version (§7.3) ; listes de jurés (§7.8) ; succession du commissaire et du procureur (§6.2) ; renforts de police par de vraies arrivées (§7.4). Ce que la population avait demandé au combat (milieu, casiers, morts violentes, violences contre une femme enceinte) y est repris. Restent côté combat :

- Fournir les règles de calcul des **dates de retour** (réductions de peine, libération conditionnelle) et des **délais d'effacement** des casiers, que le générateur applique à la date de départ (§5.1, §5.4).
- Préciser que les décès d'enfants de cause naturelle sont, eux aussi, **exclus** dans la première version (§7.3), ce qui complète [combat.md](combat.md), §12.2.
- **Reliquat de [combat.md](combat.md) v0.5** (§16.4) : **traité** en v0.8.1. Les cicatrices de départ viennent des agressions, accidents et opérations de la biographie, à leur date, et entrent dans les signes distinctifs du génome et dans son historique (§4.9) ; depuis la v0.9, elles sont tirées selon les probabilités du combat §2.4 (arbitrage A9). Rien ne reste côté combat.

### 12.7 Domaine interface

Les demandes de ce fichier sont traitées par [interface.md](interface.md), §15.5 (ce que connaît un nouveau venu, événements de vie rendus perceptibles, familiarité devenue inutile). Sa demande en retour est **traitée dans ce fichier** (v0.3) : faire-part, avis de décès et annonces de naissance émis comme courrier et contenus du journal local (§7.8). Reste côté interface :

- Montrer les **listes de jurés** et la convocation d'un juré comme un courrier ordinaire, et la carte d'électeur reçue à 18 ans ou après une inscription (§7.8).

**Reliquat de [interface.md](interface.md) v0.6** (§15.5) : **traité** en v0.8.1. Plages du génome par âge et par sexe, et distribution d'un fondateur de lignée pour « Autre visage » et « Tirer au hasard », concordantes avec la v0.8 (§4.9) ; **historique daté** de la coiffure, de la tenue habituelle et des signes distinctifs, initialisé d'après la biographie (§4.9). Reste côté interface : rien ; le serveur reconstitue le portrait avec le génome, l'âge à la date et cet historique.

### 12.8 Revues d'équilibrage et de sécurité

Les modifications demandées par [revues/equilibrage.md](../revues/equilibrage.md), §6.11 (E25) et [revues/securite.md](../revues/securite.md), §9.7 (S2, S3) sont **traitées dans ce fichier** (v0.4), selon l'arbitrage A1 de la session principale : deux populations initiales (§9.1), couche cachée et rodage propre par monde (§1.4, §1.6), population par serveur comme étape suivante (§9.4). Conséquences pour les autres domaines :

- **Ville** : fournir, pour chaque bâtiment ou territoire concerné, la liste des **emplacements possibles** d'une cache, et marquer les lieux équipés d'une alarme ; l'emplacement des caches et l'état des alarmes sont tirés par monde avec la **part serveur de la ville** (étape W2, P12), et la population tire seulement qui les connaît ([ville.md](ville.md), §5.5 ; §1.6).
- **IA des PNJ** : les savoirs opaques restent des **références** ; leurs détenteurs sont fixés à la création du monde, après la population visible ([ia-pnj.md](ia-pnj.md), §4.4 ; §1.6).
- **Économie** : fixer les **fourchettes** du contenu des coffres et des planques (argent, marchandises), et les montants possibles des dettes secrètes envers le réseau (§1.6, §6.4).
- **Combat** : fixer les fourchettes des armes et des produits des planques ; accepter que les informateurs, leurs policiers traitants et les policiers corruptibles soient tirés par monde parmi les profils compatibles (§1.6, §3.5).
- **Social** : les liaisons et les dettes secrètes des intrigues de départ n'existent qu'à la création du monde ; les rumeurs et les enquêtes du social partent de cet état (§6.4).

### 12.9 Revues de la génération et des signatures

Les modifications demandées par [revues/generation.md](../revues/generation.md), §13.5 et [revues/signatures.md](../revues/signatures.md), §9.11 sont **traitées dans ce fichier** (v0.6) :

- **Génération** : programme en tête de chaîne, chronique, grammaire de ménages et distributions stationnaires en entrée, mobilier mobile en sortie (§1.1, P1, P3, P11, P17, P22) ; nouvel ordre P1 à P12 et méthodes (§1.2, P18 à P21) ; contrôles bloquants ou en alerte, métriques du réseau, vacance de 5 % (§1.3, §3.1, §5.3, P37, P41) ; rodage de famille par convergence (§1.4, P22) ; morceau de ville par le programme restreint (§1.5) ; création d'un monde en W1 à W8, rodage propre sur les 14 jours qui précèdent le départ, solveur à contraintes et tentatives numérotées (§1.6, P12, P23, P24) ; ménages sous la contrainte du parc (§2.2) ; affectations par flots (§3.1 à §3.3, P19) ; viabilité par la localisation (§3.4) ; biographie à rebours et chronique (§5.1, P20, P3) ; graine en ligne secrète, cache privé, paquet client vérifié (§9.1, P25, P29, P30) ; chaîne embarquable (§9.4, P40).
- **Signatures** : la biographie déplace le profil dans la direction des effets des événements (§4.1, G13) ; `ts-data check` vérifie la couverture des catégories par les moyens et les recettes de métier (§4.6, G16).

Conséquences pour les autres domaines et le socle :

- **Socle** ([05-architecture.md](../05-architecture.md), §2.14) : à jour. **Vérifié** le 29/10 par la session principale : le §2.14 décrit la part serveur, la couche cachée, le rodage de 14 jours et le budget de 10 s en solo, et renvoie explicitement aux étapes W1 à W8 du §1.6.
- **Ville** : le **programme** (G0) est calculé à partir des tables cibles de ce domaine ; le registre doit donner l'**année de construction** de chaque bâtiment (§5.1) ; la part serveur de la ville (W2 : serrures, caméras, coffres, alarmes, emplacement des caches) est tirée avant la couche cachée de la population, qui s'y réfère (§1.6). Le §12.8 est corrigé dans ce sens : la ville tire l'emplacement des caches et l'état des alarmes, la population tire qui les connaît.
- **Économie** : le catalogue des métiers doit indiquer le **diplôme ou l'expérience exigés** (contrôle de qualification, §1.3) ; la localisation par aires de chalandise rend les corrections de viabilité rares (§3.4).
- **IA des PNJ** : `ts-sim` mesure les **distributions stationnaires** des variables rapides (besoins, stocks, trésorerie, humeur, projets) qui initialisent le rodage (§1.4).
- **Social** : la **mémoire** des relations est dimensionnée sur ses plafonds (150 complètes, 500 compactes par personne), et non sur les ≈ 600 000 relations du départ (§5.3).
- **Signatures** (`ts-data check`) : vérifier la couverture des catégories par les moyens fidèles et les recettes de métier avant chaque génération (§4.6).
- **Social** (v0.7) : la demande de [social.md](social.md), §12.11 est traitée : plages du §10.2 utilisées par le validateur (§5.3), profil normatif des groupes (§5.5), profil de valeurs des programmes (§6.4). Reste côté social : un profil normatif type dans chaque **fiche de groupe**.

### 12.10 Domaine direction artistique

Les demandes de [direction-artistique.md](direction-artistique.md), §12.2 sont **traitées dans ce fichier** (v0.8) : génome d'apparence avec silhouette d'après l'âge, le sexe et les attributs physiques, hérédité limitée, voix sonore et goûts vestimentaires (§4.9) ; garde-robe abstraite, complétée à chaque création de personne (§4.10, §7.2, §7.4) ; contrôles anti-sosies, dispersion par quartier et test de parenté (§1.3) ; vieillissement du génome (§4.9) ; probabilité non nulle de toute combinaison de l'éditeur (§4.9). Reste à l'extérieur de ce fichier :

- **Direction artistique** : fournir dans le paquet de contenus les **plages** de chaque champ du génome et les **lois de répartition** (tailles, corpulences, lunettes, barbes, cheveux gris selon l'âge et le sexe), le catalogue vestimentaire avec, pour chaque pièce, les âges, les milieux, les usages et les postes auxquels elle convient, et la liste des **uniformes** par poste (§1.1, §4.10).
- **Interface** : l'éditeur du persistant lit les mêmes plages et catalogues que le générateur ; le test de probabilité non nulle bloque toute plage de l'éditeur absente des lois de la population (§4.9).
- **Combat** : une blessure grave ou critique **peut** laisser une cicatrice dans le génome, au départ (lue dans la biographie : agression, accident, opération) comme en cours de partie, selon la règle du combat §2.4 (§4.9) ; **traité** en v0.8.1, aligné en v0.9 (A9).
- **Social** : les **traits appréciés** des préférences d'attirance ne lisent de l'apparence que la silhouette et la tenue, jamais la carnation ni le visage (§4.8).
- **Sécurité** : le test de parenté et la règle « aucune filiation secrète dans le génome » protègent les secrets de famille tirés par monde (§1.3, §4.9 ; [revues/securite.md](../revues/securite.md), S3).

### 12.11 Dernière vague avant la clôture (v0.9)

| Demande | Où | État |
|---|---|---|
| Arbitrage **A9** : cicatrices selon la règle du combat §2.4 (≈ 30 % des blessures graves, ≈ 70 % des critiques) | §4.9, §12.6, §12.10 | Intégré en v0.9 |
| [social.md](social.md) v0.7, §12.12 : signature graphique de chaque bande et du réseau de départ ; signes de bande dans la garde-robe | §5.5, §4.10 | Intégré en v0.9 |
| [economie.md](economie.md) v0.7, §14.10 : valeur de remplacement et usure de départ de la garde-robe, lave-linge, valeur affective des objets hérités et des cadeaux marquants, musiciens | §4.10, §1.2, §4.11 | Intégré en v0.9 |
| Arbitrage **A8** : aucune action d'un humain ; fiches d'ancrage, chronique et retouches écrites par les sessions de contenus et acceptées par les contrôles automatiques ; seuil du test de parenté sans panel humain | §0, §1.1, §1.3, §6.3, §9.1, §9.4 | Appliqué en v0.9 ; les seules mentions de l'utilisateur sont les marques historiques de l'en-tête |

Ces ajouts touchent d'autres domaines, sans demande envoyée (clôture de la conception) ; état **pour la phase 1** :

- **Contenus** : la session de contenus écrit les fiches d'ancrage, la chronique et les retouches comme des données du paquet de contenus (§6.3).
- **IA des PNJ** : le seuil d'achat de vêtements et celui du coiffeur sont propres au personnage et servent aussi à étaler l'état de départ (§4.9, §4.10).
- **Social** : une bande créée en cours de partie tire sa signature par la règle du social ; seules celles de départ sont tirées par la population (§5.5).

Aucune contradiction relevée avec les autres domaines.
