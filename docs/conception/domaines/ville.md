# Domaine : la ville (monde physique)

> Fichier de **domaine**, géré par la session « Conception — ville » (voir [README](../README.md)).
> Version 0.11 — 30/10/2026 (heures calmes et isolation entre logements demandées par social.md, §5.6 et §9.1 ; arbitrage A8 appliqué : aucune action humaine, §4.1 ; §11).
> Version 0.10 — 29/10/2026 (demandes de la direction artistique et de ia-pnj v1.1, §11.3 ; nouvelles sections §3.5 et §5.3 bis).
> Version 0.9.1 — 29/10/2026 (année de construction dans le registre, demandée par population ; ligne périmée du §11.2 remplacée).
> Version 0.9 — 29/10/2026 (intégration de la revue de la génération procédurale, §11.4).
> Tout le contenu est validé par l'utilisateur, sauf les points marqués **(décidé)**, tranchés par la session ville à la demande de l'utilisateur, avec leur justification. Les valeurs marquées **[à calibrer]** seront ajustées par les sessions au prototype et par le calibrage (`ts-sim`), sans action de l'utilisateur (A8). Les décisions qui touchent le socle ou d'autres domaines restent à confirmer par la session principale (§11).
> Les sections ne sont **jamais renumérotées** (README de la conception, règle 6) : une nouvelle section prend un numéro libre ou un suffixe « bis », car le code citera les sections.

## 1. Choix validés en bref

| Sujet | Choix (validé) |
|---|---|
| Forme du monde | Une **agglomération** : une ville-centre et **deux communes** |
| Inspiration | Ville moyenne **européenne, francophone** |
| Géographie | Un **fleuve** traverse la ville-centre, avec un **port fluvial** |
| Production | **Génération procédurale, figée d'abord** : une seule carte générée puis retouchée par une session (§4.1) ; le même générateur pourra plus tard créer une ville par monde |
| Population | **10 000 habitants**, extensible |
| Figurants | **Aucun** : toute personne visible est un vrai habitant |
| Quartiers | Organisation du §3 |
| Intérieurs | **Sans chargement** : ils font partie du monde |
| Transports en commun | **Bus**, **train régional**, **navette fluviale** |
| Sortie de l'agglomération | Oui, vers un **extérieur abstrait** |
| Évolution | L'usage des bâtiments change, et il y a aussi des **constructions et des démolitions** |
| Dégâts matériels | **Persistants jusqu'à réparation** |
| Noms | Agglomération d'**Orvenne**, sur l'**Ostrelle** (§2.4) |
| Trajets en ligne | **Pas de voyage rapide** (§7.5) |
| Personnage persistant | **Nouveau venu** qui se trouve un logement, apparaît et disparaît dans des **lieux de retrait**, peut travailler, et passe pour quelqu'un de « souvent en déplacement » (§2.5) |

## 2. Taille et forme du monde

### 2.1 L'agglomération (validé)

Une ville-centre de part et d'autre d'un fleuve, une commune aisée en amont sur les coteaux, une commune populaire et industrielle en aval. Entre elles : champs, bois, rocade et voie ferrée.

```
        HAUTEVIGNE, commune aisée (amont, coteaux) ~1 500 hab.
        villas, club, bourg chic, lycée privé
                     |
  ┌───────── ORVENNE, RIVE NORD ──────────┐
  │ La Cité (vieux centre) : mairie, place│
  │ Quartier de la Gare : bureaux, banque │
  │ Les Tilleuls (bourgeois)              │
  │ Beaulieu : hôpital et campus          │
 ~~~~~~~~~ L'OSTRELLE (3 ponts) ~~~~~~~~~~~~> aval
  │ Faubourg Saint-Roch : ateliers, bars  │
  │ Les Quais : port, entrepôts, friches  │
  │ Les Peupliers (grand ensemble)        │
  └────────── ORVENNE, RIVE SUD ──────────┘
                     |
  FORGES-SUR-OSTRELLE, commune populaire et industrielle (aval) ~2 000 hab.
  usines, logistique, cités ouvrières, hypermarché
```

| Grandeur | Valeur |
|---|---|
| Emprise totale | ≈ 6 × 4 km **[à calibrer]** |
| Ville-centre | ≈ 2,5 × 2 km **[à calibrer]** |
| Chaque commune | ≈ 1 × 1 km, à ≈ 3 km du centre **[à calibrer]** |
| Ponts sur le fleuve | 3 dans la ville-centre (dont un pont historique) |

Ces dimensions sont conservées jusqu'au prototype du premier quartier, qui servira à les ajuster (lisibilité, temps de trajet, densité). L'ajustement est fait par les sessions, à partir des mesures du prototype (temps de trajet, densité) et de captures jugées par des sessions évaluatrices **(décidé, [A8](../../pilotage/organisation.md))**.

### 2.2 Échelle

- **Échelle réelle** : 1 unité = 1 mètre. Rues, bâtiments, distances et vitesses sont réalistes, pour rester cohérents avec la progression réaliste par attributs (vitesse de course, endurance).
- **Ville « resserrée »** : pour que chaque logement ait de vrais occupants avec 10 000 habitants, les bâtiments sont un peu **moins hauts et moins nombreux** que dans une vraie ville de cette importance (immeubles de 3 à 6 étages dans le centre, quelques tours dans le grand ensemble). L'agglomération a les services d'une ville moyenne et la population d'une petite ville.
- **Intérieurs lisibles (décidé, [P10](../revues/generation.md))** : les plans types ont des **dimensions minimales élargies** (dégagements d'au moins 1,4 m, portes d'au moins 1 m, chambres d'au moins 2,8 m de large **[à calibrer]**) au lieu d'un agrandissement uniforme des intérieurs. Pourquoi : un agrandissement uniforme obligerait à grossir l'emprise de tous les bâtiments, donc toute la ville, alors que seuls les passages étroits gênent la lecture en 2.5D.
- **Stockage en centimètres (décidé, [P13](../revues/generation.md))** : l'unité de jeu reste le mètre, mais toute la géométrie est enregistrée en **centimètres entiers**. Pourquoi : des entiers donnent les mêmes résultats sur toutes les machines et des empreintes stables, ce qu'exige la reproductibilité par graine.

### 2.3 Population (validé : 10 000, aucun figurant)

- **10 000 habitants**, tous des personnages complets. Le chiffre pourra augmenter si la simulation le permet.
- **Aucun figurant** : personne n'est là « pour faire nombre ». Les rues sont donc plus calmes que dans une grande ville, et leur animation vient des habitants eux-mêmes (heures de pointe, sorties d'école, marché, soirées).
- Ordres de grandeur **[à calibrer]** :

| Grandeur | Valeur indicative |
|---|---|
| Enfants (non jouables) | ≈ 1 800 (18 %) |
| Ménages | ≈ 4 500 (2,2 personnes par ménage) |
| Logements ordinaires | ≈ 4 740 pour ≈ 4 500 ménages, soit ≈ 5 % de vacance pour permettre déménagements et arrivées ([population.md](population.md), §3.1). Le chiffre exact est **calculé par le Programme** (§4.2) à partir des tables cibles, seule source des chiffres **(décidé, [P1](../revues/generation.md), [P41](../revues/generation.md))** |
| Hébergement collectif | Résidences étudiantes ≈ 190 places, foyer de travailleurs ≈ 60 places (dont quelques chambres gardées libres pour les nouveaux venus), maison de retraite ≈ 70 places, squats pour une dizaine de sans-abri |
| Emplois occupés | ≈ 4 100 |
| Établissements (entreprises, commerces, services) | ≈ 500 à 700 |

- **Chaque serveur** (un par fuseau horaire) part de **la même carte initiale** ; ensuite chaque monde évolue de son côté (constructions, commerces, dégâts). Règle du socle : [01-concept.md](../01-concept.md), §1.1.

### 2.4 Noms

Noms fictifs, à consonance française, sans lien avec des lieux réels connus. Ils restent modifiables tant que rien n'est produit.

| Élément | Nom |
|---|---|
| Agglomération | **Agglomération d'Orvenne** |
| Ville-centre | **Orvenne** |
| Fleuve | **l'Ostrelle** |
| Commune aisée (amont, coteaux) | **Hautevigne** |
| Commune populaire et industrielle (aval) | **Forges-sur-Ostrelle**, dite « les Forges » |
| Vieux centre | **la Cité** |
| Gare et affaires | **quartier de la Gare** |
| Quartier bourgeois | **les Tilleuls** |
| Hôpital et campus | **Beaulieu** |
| Faubourg populaire | **faubourg Saint-Roch** |
| Quais et port | **les Quais** |
| Grand ensemble | **les Peupliers** |
| Ponts | **Pont-Vieux** (historique, la Cité ↔ faubourg Saint-Roch), **pont de la Gare**, **pont des Quais** |


### 2.5 Le personnage persistant dans la ville (validé)

Le personnage persistant d'un joueur disparaît à la déconnexion ([02-personnages.md](../02-personnages.md), §1), alors que la ville n'a aucun figurant. Il est donc traité comme un **habitant qui s'absente souvent**.

**Arrivée.** Un nouveau personnage persistant arrive à Orvenne **par le train**, à la gare, avec un petit pécule et **quelques nuits payées** dans un foyer ou un hôtel **[à calibrer]**.

**Logement.**

- Il **loue ou achète un vrai logement** par l'économie, comme n'importe quel habitant : studio, colocation, appartement, maison. Il peut aussi rester à l'hôtel ou au foyer tant qu'il paie.
- Son logement reste **occupé** pendant ses absences : il y habite, même s'il paraît souvent vide.
- Si la demande des joueurs fait monter les loyers et manquer les logements, l'économie peut lancer la **construction de résidences** (§6.2) au lieu de priver les habitants de logement.

**Lieux de retrait** (où il disparaît et réapparaît ; règle reprise dans le socle, [02-personnages.md](../02-personnages.md), §1.1) :

| Lieu de retrait | Disparition | Réapparition |
|---|---|---|
| **Son logement** | En franchissant sa porte d'entrée | En sortant de chez lui |
| **Sa chambre** d'hôtel ou de foyer, tant qu'elle est payée | En entrant dans la chambre | En sortant de la chambre |
| **Un transport qui part** (train, bus, navette) | En montant à bord | En descendant du même moyen de transport, à la même gare ou au même arrêt |
| **Une sortie de la carte** (§7.4) | En quittant l'agglomération par la route, le rail ou le fleuve | En revenant par la même sortie |

- S'il se déconnecte **ailleurs**, il reste **environ une minute** sur place **[à calibrer]**, vulnérable, puis s'éloigne vers le lieu de retrait le plus proche et disparaît **hors de la vue** des habitants. On ne peut donc pas fuir un combat ou une arrestation en se déconnectant.
- **Retrait sous menace (décidé, [E19](../revues/equilibrage.md))** : tant qu'il est **menacé** (affrontement, poursuite, interpellation en cours, policier qui vient pour lui avec un motif, fiche de recherche et policier en vue), **aucun lieu de retrait ne le fait disparaître**. Franchir sa porte, monter dans un transport ou atteindre une sortie le laisse dans le monde (derrière sa porte, dans le bus, à la sortie), piloté par l'automate défensif, jusqu'à la fin de la menace ou jusqu'à un plafond de **5 minutes** **[à calibrer]**. Pourquoi : un lieu de retrait est l'endroit où l'on rentre chez soi, pas une porte de secours en pleine poursuite.
- **Véhicule (décidé, E19)** : un véhicule qui ne lui appartient pas ne quitte **jamais** la carte avec lui ; il est retrouvé à la sortie. Pourquoi : une sortie de la carte ne doit pas faire disparaître une voiture volée.
- **Ce qu'il porte (décidé, [E20](../revues/equilibrage.md))** : en disparaissant dans son **logement** ou sa **chambre**, il y **dépose** ce qu'il porte, sauf ses **effets personnels** (vêtements, téléphone, clés, portefeuille ordinaire avec au plus **200 écus** en espèces **[à calibrer]**). Le reste se trouve dans le logement, où une perquisition peut le découvrir. En partant par un transport ou une sortie hors menace, il emporte ses affaires. Pourquoi : un personnage absent ne doit pas devenir une cachette hors du monde pour une arme ou un butin.
- S'il ne peut plus revenir à son lieu de retrait (expulsé, chambre non payée, logement démoli), il réapparaît **à la gare**.
- Personne ne le voit jamais s'effacer : pour les habitants, il est rentré chez lui ou il a pris le train.
- En **solo**, le monde est en pause tant que le jeu est fermé : le joueur reprend exactement là où il s'était arrêté, sans passer par un lieu de retrait **(décidé)**. Pourquoi : en solo, le serveur tourne dans le processus du jeu et le joueur choisit déjà quand le temps avance ; faire vivre le monde jeu fermé enlèverait ce contrôle et coûterait du calcul sans rien apporter.

**Emploi.**

- Il peut prendre de **vrais emplois**. Les emplois **à la tâche** (intérim, livraisons, extras dans la restauration, travail indépendant) conviennent le mieux à sa présence irrégulière.
- Un emploi **à horaires fixes** est possible, mais s'il ne vient pas travailler, il reçoit des avertissements puis il est **renvoyé**, comme n'importe qui. Un poste indispensable à la ville n'est donc jamais bloqué longtemps.

**Ce que perçoivent les habitants.**

- Pour eux, c'est quelqu'un qui **part souvent** : ses absences sont des séjours **à l'extérieur** (§7.4), comme pour tout habitant.
- Les voisins le voient rentrer et sortir, et peuvent le trouver mystérieux.
- Une **longue absence se remarque** : courrier qui s'accumule, loyer impayé (relances puis expulsion), employeur qui le renvoie.

**Comptage.** Les personnages persistants **ne comptent pas** dans les 10 000 habitants ni dans le classement des paliers, puisqu'ils ont leur propre niveau et sont absents la plupart du temps **(décidé)**. Pourquoi : les paliers mesurent la puissance relative des habitants qui vivent en permanence ; y ajouter des centaines de personnages souvent absents ferait dériver les seuils selon le nombre de joueurs connectés à chaque serveur.
## 3. Quartiers (validé)

### 3.1 Identité des quartiers

| Quartier | Commune | Habitants **[à calibrer]** | Identité | Lieux typiques |
|---|---|---|---|---|
| **Vieux centre** | Ville-centre, rive nord | ≈ 1 100 | Cœur historique, rues irrégulières, commerces de pied d'immeuble, animation de jour et de soirée | Mairie, place centrale, marché couvert, église, bars, restaurants, pont historique |
| **Gare et affaires** | Ville-centre, rive nord | ≈ 600 | Peu d'habitants, beaucoup d'emplois ; vide la nuit | Gare, bureaux, banques, hôtel, agence de presse locale, siège de l'entreprise principale |
| **Quartier bourgeois** | Ville-centre, rive nord | ≈ 900 | Immeubles anciens soignés, calme, professions libérales | Cabinets d'avocats et de médecins, lycée public, parc |
| **Hôpital et campus** | Ville-centre, rive nord | ≈ 700 | Étudiants et soignants, horaires décalés | Hôpital, antenne universitaire, résidences étudiantes, salles de sport |
| **Faubourg populaire** | Ville-centre, rive sud | ≈ 1 300 | Petits immeubles, ateliers, commerces variés, vie de rue | Ateliers d'artisans, bars, épiceries, salle de boxe, garage |
| **Quais et port** | Ville-centre, rive sud | ≈ 300 | Peu habité, industriel, mal éclairé la nuit | Port fluvial, entrepôts, friches, squats, capitainerie |
| **Grand ensemble** | Ville-centre, rive sud | ≈ 1 600 | Tours et barres des années 60-70, dalles, centre commercial de quartier | Collège, centre social, terrain de sport, petit centre commercial |
| **Commune aisée** | Amont, coteaux | ≈ 1 500 | Villas, résidences fermées, petit bourg chic | Mairie, club de tennis et golf, lycée privé, clinique privée, restaurant gastronomique |
| **Commune populaire et industrielle** | Aval | ≈ 2 000 | Cités ouvrières, pavillons modestes, zone industrielle et commerciale | Mairie, usines, plateforme logistique, hypermarché, halte ferroviaire |

### 3.2 Répartition des paliers de puissance

Les paliers sont calculés sur l'ensemble de la population, **enfants compris** ([02-personnages.md](../02-personnages.md), §3.4). La géographie ne fait que **biaiser le lieu de résidence** ; les lieux de travail et de sortie mélangent tout le monde.

| Où vivent… | Quartiers surreprésentés |
|---|---|
| Paliers 0 à 2 (anonymes, ordinaires, établis) | Grand ensemble, faubourg, commune populaire, résidences étudiantes |
| Paliers 3 à 6 (qualifiés à notables) | Quartier bourgeois, vieux centre, pavillons de la commune populaire |
| Paliers 7 à 11 et hors catégorie, par la **richesse** ou l'**autorité** | Commune aisée, quartier bourgeois |
| Paliers 7 à 11, par les **moyens** (chefs de bande, réseaux criminels) | Souvent restés dans leur quartier d'origine (grand ensemble, faubourg, quais) |

### 3.3 Types de logements par quartier (décidé)

Le générateur choisit la taille des logements selon les ménages qui doivent y vivre ([population.md](population.md), §2.3). Pourquoi : chaque ménage doit trouver un logement à sa taille dans le quartier où sa catégorie sociale le place, sans laisser de grands logements vides ni entasser les familles.

| Quartier | Logements dominants |
|---|---|
| La Cité, quartier de la Gare, Beaulieu | Petits logements (studios, deux-pièces) : personnes seules, jeunes couples, étudiants, soignants |
| Les Tilleuls | Grands appartements anciens |
| Faubourg Saint-Roch | Mélange de petits et moyens logements, quelques ateliers avec logement |
| Les Peupliers, Forges-sur-Ostrelle | Grands logements familiaux (quatre et cinq pièces), pavillons modestes aux Forges |
| Les Quais | Très peu de logements : quelques lofts, squats |
| Hautevigne | Villas et maisons, quelques appartements de standing |

### 3.4 Lieux faits à la main

Le générateur produit l'essentiel ; quelques lieux emblématiques sont **faits à la main** puis intégrés à la carte : la mairie et la place centrale, la gare, l'hôpital, le commissariat central, le pont historique, le port, l'usine principale, la tour emblématique du grand ensemble, la plus grande villa des coteaux, le stade.

**Pièces avec ports (décidé, [P8](../revues/generation.md))** : chaque lieu fait main est une **pièce** assemblée dans Godot avec le kit, qui porte ses **ports** (raccords de rue, entrées, quais de livraison). Les pièces sont posées **avant les rues** (étape V2, §4.2), et leurs ports sont raccordés aux axes ; les parcelles voisines prennent leur **époque** et leur **style**. Les intérieurs du commissariat et du tribunal (§5.4) sont des pièces de ce type. Pourquoi : un lieu fait main inséré après coup obligerait à démolir ce qui a été généré à sa place ; posé d'abord, il devient une contrainte que le reste respecte.

### 3.5 Identités visuelles des quartiers (décidé, [direction-artistique.md](direction-artistique.md), §1.3)

Ce qui se voit de chaque quartier (façades et matériaux dominants, toits, **espèces d'arbres d'alignement**, mobilier urbain et signes, entretien moyen) est fixé par la direction artistique, §1.3. Côté ville, ces identités sont des **paramètres du plan directeur et des gabarits morphologiques** (§4.2) :

- **V6 Bâtiments** en tire les matériaux de façade et de toiture de chaque parcelle, selon son quartier et son époque (pierre blonde et ardoise dans la Cité, brique et sheds à Saint-Roch, béton préfabriqué aux Peupliers…) ;
- **V9 Espace public** en tire les arbres d'alignement (platanes, tilleuls, peupliers, chênes et cèdres…) et le mobilier urbain (pavés et lanternes dans la Cité, abribus vitrés à la Gare, bollards et grillages aux Quais…).

Les lieux faits main (§3.4) suivent le style de leur quartier, avec un matériau ou une silhouette propre qui sert de repère à distance. Pourquoi : une seule source pour ce qui se voit, appliquée par le générateur à partir du quartier et de l'époque de chaque parcelle, sans risque que la carte et la direction artistique divergent.

## 4. Production de la ville

### 4.1 Principe (validé : procédural, figé d'abord)

1. On écrit un **générateur procédural** dès le départ.
2. La première version du jeu publie **une seule ville**, générée, retouchée, puis **figée** : c'est la carte initiale de tous les mondes (solo et serveurs).
3. Le même générateur sert ensuite **en cours de partie** pour les constructions (§6.2), et pourra plus tard créer **une ville différente par monde** (§4.6).

**Qui écrit et retouche (décidé, [A8](../../pilotage/organisation.md))** : aucune étape ne demande d'action humaine. « Fait main » désigne ce qui est **écrit en données** au lieu d'être généré : le plan directeur, la chronique, les tables, les catalogues, les **pièces faites main** (§3.4) et les **surcharges** (§4.3) sont écrits par les sessions de contenu, en fichiers texte (RON et scènes Godot). L'éditeur Godot sert d'outil d'aperçu et de contrôle à ces sessions. Les retouches d'Orvenne sont décidées à partir du rapport de validation (V14) et de captures jugées par des sessions évaluatrices. Pourquoi : l'utilisateur ne fait aucune action ; tout ce qui était « à la main » doit donc être une donnée qu'une session peut écrire, relire et régénérer.

### 4.2 Chaîne de génération (décidé, [P1 à P16](../revues/generation.md))

La chaîne détaillée (entrées, coûts, parallélisme) est dans [revues/generation.md](../revues/generation.md), §4 ; ce paragraphe en donne ce qui concerne la ville.

**Entrées faites main** (données versionnées) :

| Entrée | Contenu |
|---|---|
| **Plan directeur** (P2) | Relief de contrôle, tracé de l'Ostrelle et de ses berges, zones inondables, grands axes, ponts, voie ferrée, contours des quartiers avec leur **gabarit morphologique**, leur **époque** et leurs paramètres (densité, hauteur maximale, usages permis, revenu, entretien), emprises des lieux faits main, perspectives, tracés indicatifs des lignes de bus et des pontons, **identités visuelles** de chaque quartier (§3.5). Écrit par une session de contenu, avec aperçu dans Godot (§4.1) |
| **Chronique d'Orvenne** (P3) | ≈ 30 à 50 événements datés **[à calibrer]** (fondation, arrivée du chemin de fer, industrialisation des Forges, construction des Peupliers, crues…), communs aux époques des bâtiments et aux biographies des habitants |
| **Tables cibles** | Ménages, vacance, ratios d'équipement (population §2 et §3) |
| **Catalogues** | Kits, plans types, gabarits morphologiques, grammaires de bâtiments, ensembles de mobilier fixe, thèmes de noms |
| **Pièces faites main** (P8) | Lieux du §3.4 et intérieurs du §5.4, avec leurs ports |
| **Surcharges** (P28) | Retouches attachées à des identifiants stables (§4.3) |

**Étapes** :

| Étape | Contenu |
|---|---|
| **G0 Programme** (P1) | Sans aucun tirage, à partir des tables cibles : ménages attendus par quartier et par classe, **parc de logements** avec la vacance (≈ 4 740 logements, §2.3), établissements par ratios d'équipement, postes par métier. Vérifie les totaux (postes, places d'école, hébergement collectif) avant toute géométrie |
| **V1 Terrain** | Champ de hauteur au pas de 2 m, lit de l'Ostrelle, berges, quais, zones inondables (§6.5) |
| **V2 Réseau principal et ancrages** (P8) | Grands axes accrochés au relief, ponts, voie ferrée ; **pose des pièces faites main** et raccord de leurs ports |
| **V3 Rues** (P4, P5) | Chaque quartier rempli **par couches d'époque** selon son gabarit (organique dans la Cité, percées depuis la gare, damier déformé à Saint-Roch, grand ensemble sur dalle aux Peupliers, rues des coteaux le long des courbes de niveau à Hautevigne…) ; un champ d'orientation assure les raccords entre quartiers |
| **V4 Îlots et parcelles** (P6) | Bandes en front de rue pour les îlots anciens, découpe récursive ailleurs, une parcelle par opération pour les grands ensembles ; chaque parcelle a un **identifiant stable** |
| **V5 Affectation du programme** (P9) | Établissements placés par **aires de chalandise** (couverture maximale, regroupement des bars, répulsion des concurrents), services décidés des §5.2 et §5.3 posés d'abord ; typologies de logements pour tenir le parc ; réserve constructible (§6.2) |
| **V6 Bâtiments** (P7) | **Grammaire de découpe** par typologie et époque sur la grille du kit : volume, niveaux, travées, ouvertures, toiture, **matériaux** selon l'identité du quartier (§3.5), entretien. Sorties **sémantiques** (§4.4) |
| **V7 Plateaux, logements, pièces, mobilier fixe** (P10) | Cages d'escalier, gaines, logements le long des travées, **plans types** ajustés par un petit solveur borné ; cloisons sur les trumeaux, pièces humides superposées, dimensions minimales élargies (§2.2) ; mobilier fixe |
| **V8 Pièces faites main** | Intérieurs des ancrages importés tels quels |
| **V9 Espace public** | Stationnement, aires de livraison (§7.4), éclairage (§6.4), arbres d'alignement et mobilier urbain selon l'identité du quartier (§3.5), bancs, abribus, boîtes aux lettres, plaques et panneaux (§4.4), **emplacements du décor saisonnier** (§6.5) |
| **V10 Transports en commun** (P16) | Arrêts, lignes, horaires et besoins en véhicules et en personnel, **générés à partir des tracés** (§7.3) |
| **V11 Registre, adresses et noms** (P36) | Registre public des lieux (§9.1), **adresses à la française** (numérotation depuis le fleuve ou le centre, impairs à gauche, numéros bis), noms de rues par époque et par thème, sans personne réelle ; **marques fictives** (enseignes des commerces, marques de produits, compagnie de bus de la régie, radios et journal locaux) avec leur nom et une **graine de logo** **(décidé, [direction-artistique.md](direction-artistique.md), §1.4 et §8.3)** : le client dessine le logo (forme, initiales, deux couleurs) à partir de cette graine. Pourquoi : aucune marque réelle dans le monde, et une graine suffit à donner à chaque marque une image stable sans stocker d'image |
| **V12 Candidats de la part serveur** (P12) | Points de pose génériques des caméras, coffres et caches, classe de chaque porte, lieux équipables d'une alarme (§4.3) |
| **V13 Dérivés** (P13 à P15) | Graphes, itinéraires, géométrie physique étagée, maillage de navigation, collisions, graphe acoustique, cellules, secteurs, zones de bâtiment (§8) |
| **V14 Validation** | Contrôles bloquants ou en alerte : tout est relié, chaque logement a une entrée, chaque service obligatoire existe, temps de trajet raisonnables, plans lisibles (critère du §4.6) |

Pourquoi : la carte est commune à toutes les populations, elle doit donc découler des valeurs attendues du Programme et non d'un tirage ; poser les ancrages d'abord et remplir les quartiers par époques donne à chaque quartier sa forme reconnaissable, et placer les commerces près de leurs clients les rend viables par construction.

**Mobilier (décidé, [P11](../revues/generation.md))** : le **mobilier fixe** (cuisine équipée, sanitaires, placards, comptoirs, vitrines, postes de travail, machines, lits d'hôpital, bancs d'église) fait partie de la carte publique. Le **mobilier mobile** et le décor sont produits par la population à l'**emménagement** et déménagent avec le ménage ; leurs modifications (objet déplacé, volé, cassé) sont des modifications d'objets du monde. Pourquoi : le mobilier dépend des occupants, la population en ligne est privée, et une sauvegarde ne doit pas dépendre de la version du générateur.

### 4.3 La ville est une donnée

Principe repris dans le socle ([05-architecture.md](../05-architecture.md), §2, point 11) ; le détail est ici.

- La ville est stockée comme **données** (terrain, rues, parcelles, bâtiments, plans, usages), lues à la fois par le **serveur Rust** (collisions, navigation, règles) et par le **client Godot** (rendu à partir des kits).
- Les lieux faits main sont écrits par une session de contenu comme scènes Godot (§4.1), puis **exportés dans le même format**, pour que le serveur les connaisse aussi.
- Un monde en cours = **carte initiale versionnée + journal des modifications** (constructions, démolitions, changements d'usage, dégâts), sauvegardé dans PostgreSQL.
- **Géométrie en centimètres entiers (décidé, [P13](../revues/generation.md))** (§2.2).
- **Identifiants stables (décidé, [P28](../revues/generation.md))** : parcelles, bâtiments, logements, locaux, pièces, portes, éléments destructibles de l'espace public, arrêts et rues reçoivent un identifiant à leur première création. Quand une étape les régénère, une **réconciliation** les apparie aux anciens par recouvrement géométrique et leur rend leur identifiant. Pourquoi : les mondes enregistrés, les mises à jour de carte (§4.5) et les souvenirs des habitants (« le bar du 12 rue des Tanneurs ») désignent les lieux par leur identifiant.
- **Retouches en surcharges (décidé, P28)** : une retouche de la ville n'est jamais une modification de la sortie générée, mais une **surcharge** attachée à un identifiant stable (« parcelle P-0412 : immeuble d'angle ancien, 5 niveaux ») ou une modification du plan directeur. Elle est réappliquée après chaque régénération ; une surcharge dont la cible a disparu devient une alerte du rapport, que la session de contenu résout (§4.1). Pourquoi : une retouche faite sur la sortie serait perdue à la régénération suivante.
- **Deux parts (décidé, [S33](../revues/securite.md))** : la donnée de la ville se divise en une **part publique** et une **part serveur**.
  - La **part publique** (terrain, rues, parcelles, bâtiments, plans intérieurs, usages publics : une banque est une banque) est lue par le client et le serveur. C'est la seule livrée avec le jeu.
  - La **part serveur** (surveillance, alarmes et leur liaison, caméras et leur champ, niveaux des serrures, coffres, caches, contenu des meubles) n'est **jamais livrée au client**. Le client n'en reçoit que ce que le personnage **perçoit** : une caméra visible sur un mur, une serrure qu'il examine.
  - Pourquoi : tout ce qui est livré au client est lu par les joueurs ; le plan d'un immeuble se devine, pas l'emplacement d'une planque.
- **Couche cachée tirée par monde (décidé, [S3](../revues/securite.md) et [E25](../revues/equilibrage.md))** : à sa création, chaque monde (solo ou serveur) tire avec **sa propre graine** sa couche cachée. Côté ville, elle comprend l'**état des alarmes** (reliée ou non à une société de surveillance ou à la police) et l'**emplacement exact des caches** dans leur bâtiment.
  - **Part serveur entière (décidé, [P12](../revues/generation.md))** : c'est en fait **toute la part serveur** qui est tirée par monde (niveau de chaque serrure selon la classe de porte, la richesse du propriétaire et le quartier ; caméras privées et municipales ; coffres ; lieux réellement équipés d'une alarme et leur liaison ; caches), parmi des **candidats génériques** posés dans la part publique (étape V12, §4.2). Ces candidats ne révèlent rien parce qu'ils existent partout : angles de façade et entrées où une caméra peut se poser, pièces arrière où un coffre peut se trouver, meubles fixes, faux plafonds, caves, combles et gaines où une cache peut se loger. Pourquoi : le solo embarque le serveur ; une part serveur commune à tous les mondes serait lue par tous. La part publique ne contient donc aucun de ces éléments, et la carte initiale reste la même pour tous les mondes (§2.3). Pourquoi : ce qu'on apprend en explorant un monde solo ne doit pas livrer les secrets d'un serveur ou d'une autre partie, et un tirage à graine reste déterministe et peu coûteux.

### 4.4 Kits modulaires

| Élément | Principe |
|---|---|
| Grille | Placement sur une grille de 1 m ; travées de façade de 3 m (une fenêtre) ; hauteur d'étage 3 m, rez-de-chaussée commercial 4 m **[à calibrer]** |
| Styles de façade | Un kit par style : ancien (pierre, balcons, toits de zinc ou de tuile), années 60-70 (béton, barres), contemporain (verre, bardage), pavillon, villa, industriel |
| Intérieurs | Murs, cloisons, portes, fenêtres, sols, escaliers, ascenseurs ; un kit par type de lieu (logement modeste, logement aisé, bureau, commerce, atelier, hôpital…) |
| Mobilier | Ensembles par type de pièce (cuisine, chambre, salon, bureau, salle de soins…) et par niveau de richesse |
| Variations | Couleurs, matériaux, **état d'entretien** (lié au quartier, au propriétaire et aux dégâts, §6.3) |
| Signalétique | **Plaques de rue**, numéros, panneaux de direction, enseignes, **panneaux d'horaires** (commerces, arrêts) et écrans de prochains passages aux arrêts et à la gare ([interface.md](interface.md), §7.2 et §7.3). **Textes rendus (décidé, [direction-artistique.md](direction-artistique.md), §1.5)** : le texte de chaque plaque, enseigne, panneau ou affiche est une **donnée** (nom, adresse, horaires) ; l'élément du kit ne porte qu'une **surface de texte**, et le client rend le texte. Pourquoi : les noms viennent de la génération, changent avec les commerces et doivent se traduire |
| Acoustique | Chaque élément de mur, de plancher, de porte et de fenêtre porte une valeur d'**atténuation du son** (§5.6) **[à calibrer]**. Chaque **type de pièce** (pièce d'habitation, cage d'escalier, hall de gare, entrepôt, église, salle des fêtes, bar) porte aussi son **type de réverbération**, choisi dans un petit jeu de préréglages. Atténuations et réverbérations sont dans le **catalogue sémantique** du kit **(décidé, [direction-artistique.md](direction-artistique.md), §5.3)** |

**Sorties sémantiques (décidé, [P7](../revues/generation.md), [P33](../revues/generation.md))** : le générateur ne produit jamais de maillages, mais des données **sémantiques** (typologie, paramètres, travées, ouvertures, toitures, corniches, références aux éléments du kit). Le **client assemble** les modèles à partir du kit, en 2.5D puis en 3D, avec trois niveaux de détail de la géométrie (complet, façade, volume). Pourquoi : la 3D ne doit demander ni nouvelle donnée serveur ni nouveau protocole.

**2.5D puis 3D** : les modèles 2.5D sont simples et lisibles depuis la caméra orthographique, mais ont **déjà les dimensions définitives**. Le passage à la 3D remplace les modèles par des versions plus détaillées sans rien changer aux dimensions, aux données ni au serveur.

### 4.5 Mises à jour de la carte

Quand une nouvelle version du jeu modifie la carte initiale, les mondes existants (serveurs, parties solo) ont déjà évolué :

- une mise à jour ne change que les **parcelles que le monde n'a pas modifiées** ;
- sur une parcelle modifiée par le monde (construction, démolition, changement d'usage), **l'état du monde l'emporte** ;
- les **refontes importantes** (nouveau quartier, tracé des rues) ne s'appliquent qu'aux **nouveaux mondes** ;
- les lieux de la nouvelle carte sont appariés à ceux du monde par les **identifiants stables** et leur réconciliation (§4.3, **décidé**, [P28](../revues/generation.md)) ;
- **mise à jour différée à la vacance (décidé, [P35](../revues/generation.md))** : un changement du nombre ou de la taille des logements d'un bâtiment, ou des pièces d'un logement **occupé**, attend que le logement concerné soit **vacant** ; les parties communes, les façades et les logements vacants changent tout de suite. Pourquoi : changer le plan d'un logement habité déplacerait les meubles, les objets et les souvenirs de ses occupants sans aucune cause dans le monde.

### 4.6 Une ville par monde

On active la génération d'une ville différente par monde quand le générateur produit des villes jouables **sans aucune retouche** (ni surcharge, ni pièce ajoutée par une session) :

- **Critère (décidé, [P37](../revues/generation.md))** : sur **32 graines** de villes synthétiques et **32 graines** de populations, au moins **95 %** des constructions passent tous les contrôles bloquants de la validation (V14, §4.2) **sans retouche**, et aucune métrique ne sort de sa plage dans plus de 5 % des cas **[à calibrer]**. Pourquoi : « sans retouche » devient mesurable.
- Les plans directeurs de ces villes viennent d'un **générateur de plans directeurs** (fleuve, axes, quartiers avec gabarits et époques, ancrages placés dans des quartiers compatibles) **(décidé, [P2](../revues/generation.md))**.

L'activation se fait dans cet ordre :

1. d'abord **en solo**, après une réinitialisation du compte ;
2. puis pour les **nouveaux serveurs**.

Les lieux faits à la main (§3.4) sont alors placés par le générateur dans un quartier compatible.

## 5. Bâtiments et intérieurs

### 5.1 Accès (validé : sans chargement)

- Les intérieurs font **partie du monde**, dans le même espace que les rues : pas d'écran de chargement, pas d'espace séparé. Ce qui se passe dedans est simulé comme dehors ; on peut voir quelqu'un par une fenêtre, le suivre dans une cage d'escalier.
- **Vue en coupe** en 2.5D : quand le personnage entre, le toit et les étages supérieurs s'effacent et les murs côté caméra s'abaissent ; seul l'étage courant est affiché en détail.
- **Tous les logements et lieux d'activité ont un intérieur** : quasiment tous les habitants sont jouables, donc le logement et le lieu de travail de chacun doivent exister.
- L'accès est réglé par des **droits**, pas par des murs invisibles : lieu public aux heures d'ouverture, clients, employés, résidents, détenteurs d'une clé. Une porte fermée peut être crochetée ou forcée (domaine combat).
- **Espaces au-dessus et en dessous** :
  - les **caves** des immeubles anciens et un **parking souterrain** (quartier de la Gare) sont accessibles ; ils s'affichent comme un **étage −1** dans la vue en coupe ;
  - les **toits-terrasses** des immeubles et des tours sont accessibles par l'escalier de service ;
  - il n'y a **pas d'autre souterrain** (ni métro, ni égouts praticables, ni tunnels).
- **Bords dangereux** : les bords des toits, des quais et des ponts sont des **obstacles** pour le déplacement ordinaire. On ne tombe jamais par accident en marchant ; il faut un geste **volontaire** (enjamber, sauter) pour passer par-dessus ([combat.md](combat.md), §11.9).

### 5.2 Services publics

| Service | Lieu | Remarque |
|---|---|---|
| Mairies | Une par commune | Le maire de la ville-centre est hors catégorie |
| Hôpital | Beaulieu (hôpital et campus) | Avec une **maternité**. Hôpital public : le personnage persistant s'y réveille après sa mort, sauf s'il a les moyens de la clinique privée ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §3) |
| Clinique privée | Hautevigne | Réveil du personnage persistant qui en a les moyens |
| Cabinets médicaux, pharmacies | Tous les quartiers habités | |
| Commissariat central | **Quartier de la Gare**, près de la gare et du pont de la Gare | Police nationale, compétente sur **toute l'agglomération**. Emplacement **(décidé)** : position centrale entre les deux rives, près du pont, de la gare et des grands axes, d'où l'on rejoint vite toute l'agglomération |
| Bâtiment administratif de l'État | Quartier de la Gare | Centre des finances publiques, inspection du travail ([combat.md](combat.md), §5.2). Emplacement **(décidé)** : il rejoint les bureaux et le commissariat dans le quartier administratif et d'affaires |
| Postes de police municipale | Un par commune | Pas de gendarmerie |
| Caserne de pompiers | Rive sud, près d'un pont | |
| Tribunal | La Cité | |
| Prison | **À l'extérieur** | Un détenu est « à l'extérieur » (§7.4) pendant toute sa peine, puis revient |
| Écoles | ≈ 6 écoles publiques, 1 groupe scolaire privé à Hautevigne (école élémentaire et lycée), 2 collèges, 1 lycée public, 1 antenne universitaire | Les enfants y passent leurs journées. L'école privée élémentaire existe **(décidé)** : les familles aisées de Hautevigne ont ainsi un vrai choix d'école, qui marque leur rang social dès l'enfance |
| Maison de retraite | Beaulieu, près de l'hôpital | ≈ 70 places ([population.md](population.md), §7.2) |
| Services sociaux | Centre social des Peupliers, accueil social dans chaque mairie | [social.md](social.md), §6.5 |
| Maison de l'emploi | La Cité, antenne aux Forges | [economie.md](economie.md), §14.2 |
| Poste, banques | Vieux centre, affaires, chaque commune | |
| Transports | Gare, halte ferroviaire, dépôt de bus, pontons de la navette | §7 |
| Culte, cimetière | Église du vieux centre, autres lieux de culte, **cimetière** en périphérie de la ville-centre, côté Beaulieu, avec un petit cimetière communal à Hautevigne et aux Forges | Enterrements et visites aux tombes ([ia-pnj.md](ia-pnj.md), §13.9). Cimetières communaux **(décidé)** : chaque commune enterre ses morts près de chez elle, ce qui donne à chaque enterrement un lieu proche de la famille |
| Pompes funèbres | La Cité, près de l'église ; une agence aux Forges | Organisation des enterrements **(décidé)** : la pratique sociale de l'enterrement a besoin d'un lieu où la famille prépare la cérémonie |
| Salles des fêtes | Une par commune : salle municipale de la Cité, salle des fêtes de Hautevigne, salle des fêtes des Forges | Mariages, fêtes de famille, bals, repas de quartier, louées par la mairie **(décidé)**, [ia-pnj.md](ia-pnj.md), §13.9. Pourquoi : c'est le lieu ordinaire des fêtes d'une commune européenne, et une par commune suffit à ce que chaque famille ait la sienne près de chez elle |
| Salles de réunion | Dans les mairies (salle du conseil, **salle des mariages**), la maison des associations, les bureaux du quartier de la Gare, l'usine principale, l'hôpital et le tribunal | Conseils municipaux, mariages civils, réunions de travail et d'associations ([ia-pnj.md](ia-pnj.md), §13.9). Fournies par le kit d'intérieur de chaque type de lieu **(décidé)** : toute entreprise ou administration d'une certaine taille a la sienne |
| Locaux associatifs | Maison des associations dans la Cité ; salles associatives aux Peupliers, dans le faubourg Saint-Roch et aux Forges | Clubs, associations de quartier, amicales ([social.md](social.md), §7.3) |
| Fourrière | Zone industrielle des Forges | Véhicules enlevés par la police municipale |
| Sport et loisirs | Stade, piscine, salles de sport et de boxe, parcs, médiathèque, cinéma | |

### 5.3 Commerces et lieux de travail

- **Commerces** : supermarchés, hypermarché (commune populaire), épiceries, boulangeries, bars, restaurants, restauration rapide, tabac-presse, coiffeurs, garages, stations-service, boîte de nuit, prêteur sur gages, armurerie de chasse (armes rares, en cohérence avec l'inspiration européenne), marché hebdomadaire sur la place.
- **Grands employeurs** : l'**usine principale** (commune populaire ; « l'entreprise qui fait vivre la ville », dont le dirigeant est hors catégorie), la plateforme logistique, l'hôpital, la mairie, le port, une entreprise de bâtiment (qui réalise les constructions et réparations), les écoles.
- **Hébergement** : un hôtel (quartier de la Gare), un foyer de travailleurs (faubourg Saint-Roch, ≈ 60 places), des studios meublés à louer ; ils accueillent les nouveaux venus, dont les personnages persistants (§2.5). Une partie des chambres d'hôtel est occupée par une **demande extérieure abstraite** (voyageurs) : ces chambres sont fermées à clé et on n'y croise personne, ce qui respecte la règle « aucun figurant » ([economie.md](economie.md), §4.4).
- **Services aux particuliers** (décidé pour les emplacements) : agence d'intérim (quartier de la Gare), notaire (les Tilleuls), étude de **commissaires de justice** et **mandataire judiciaire** (la Cité, près du tribunal), concession automobile (zone commerciale des Forges), garde-meubles (zone industrielle des Forges), siège d'**Orvenne Habitat**, le bailleur social (les Peupliers). Pourquoi : chacun est placé là où se trouvent ses clients (cadres et voyageurs à la Gare, propriétaires aisés aux Tilleuls, locataires du parc social aux Peupliers) ou là où la place ne coûte pas cher (zone des Forges).
- **Bureaux** : assurances, agence immobilière, cabinets d'avocats, journal et radio locaux, banques.
- **Lieux discrets** : friches, squats, quais mal éclairés, arrière-salles. La ville fournit des lieux **peu surveillés** ; c'est leur usage (point de deal, planque) qui en fait des lieux criminels, décidé par les domaines social et combat.

### 5.3 bis Lieux de la vie quotidienne (décidé, [ia-pnj.md](ia-pnj.md), §2.6)

Ces lieux servent aux routines des habitants : tenue, coiffure, entretien du linge et musique.

| Lieu | Où | Pourquoi |
|---|---|---|
| **Salons de coiffure** | Placés par aire de chalandise (V5, §4.2) : au moins un par quartier habité, plusieurs dans la Cité et le faubourg Saint-Roch, un salon haut de gamme à Hautevigne **[à calibrer]** | Chaque habitant y va toutes les 4 à 10 semaines ; un coiffeur proche et un coiffeur habituel sont une relation de quartier |
| **Vestiaires** | Dans le plan de chaque lieu de travail dont les postes portent un **uniforme** ou une tenue de travail : commissariat et postes de police, caserne de pompiers, hôpital et clinique, usine, plateforme logistique, dépôt de bus de la régie, gare, entreprise de bâtiment, services techniques des mairies, cuisines des restaurants, supermarchés et hypermarché. Une pièce réservée aux employés, avec casiers | Les employés s'y changent avant et après le travail ; sans vestiaire, l'uniforme devrait être porté dans la rue |
| **Laveries** | Une laverie en libre-service dans les quartiers de petits logements (la Cité, quartier de la Gare, Beaulieu, faubourg Saint-Roch) et aux Peupliers ; une **buanderie** commune dans le foyer de travailleurs, les résidences étudiantes et la maison de retraite **[à calibrer]** | Les ménages sans machine à laver, les étudiants et les résidents du foyer lavent leur linge ailleurs que chez eux |
| **Lieux de musique** | **Bars** (sono, petite scène dans quelques bars de la Cité et du faubourg), **église** (orgue), **salles des fêtes** (sono, scène), **rue** (places, Pont-Vieux, marché, passages couverts, où jouent les musiciens de rue), **école de musique municipale** dans la Cité (cours des enfants et adolescents, répétitions) | Jouer en public, mettre de la musique et prendre des cours sont des activités de loisir et des postes ; chaque lieu fournit une **source musicale** possible (mobilier fixe : sono, piano, orgue) et une réverbération (§4.4) |

L'école de musique est ajoutée **(décidé)** parce que les cours de musique des enfants ([ia-pnj.md](ia-pnj.md), §8.1) ont besoin d'un lieu.

### 5.4 Intérieurs de la police et de la justice

Demandés par [combat.md](combat.md), §5.1 et §5.2 :

| Lieu | Pièces |
|---|---|
| **Commissariat central** | Accueil, bureaux, cellules de garde à vue, salles d'audition, salle des scellés, armurerie de service, garage des véhicules |
| **Postes de police municipale** | Accueil, bureaux, une cellule de dégrisement, garage |
| **Tribunal** (la Cité) | Salle de la cour d'assises, salle correctionnelle, salle civile, bureaux des juges et du parquet, salle des jurés, bureau du conseiller d'insertion, greffe, accueil |

### 5.5 Serrures, coffres et surveillance

- **Serrures** de trois niveaux de difficulté : **simple**, **multipoints**, **de sûreté**. Le niveau dépend du lieu et des moyens du propriétaire (porte de cave simple, appartement des Tilleuls multipoints, banque de sûreté) ([combat.md](combat.md), §3.1).
- **Coffres** : coffre de particulier, coffre-fort de commerce, salle des coffres de banque, chacun avec son niveau de difficulté.
- **Alarmes** : sur les commerces, les entreprises et une partie des logements aisés ; reliées ou non à une société de surveillance ou à la police. Leur **état** (reliée ou non) est tiré à la création de chaque monde avec sa graine (§4.3, **décidé**, S3).
- **Caméras** : caméras privées (commerces, résidences, entreprises) et **vidéoprotection municipale**, dont le nombre dépend du budget de chaque commune (dense dans la Cité, à la Gare et à Hautevigne ; rare aux Quais et aux Peupliers) **[à calibrer]**.
- Chaque caméra a un **champ** (position, direction, angle, portée), enregistré dans la donnée « surveillance » du registre (§9.1). Ces dispositifs peuvent être endommagés comme le reste du décor (§6.3).
- **Tirage par monde (décidé, [P12](../revues/generation.md))** : le niveau de chaque serrure, les caméras, les coffres et les lieux équipés d'une alarme sont **tirés à la création de chaque monde**, dans les plages de cette section, parmi les candidats de la part publique (§4.3). Les répartitions ci-dessus (banque de sûreté, vidéoprotection dense dans la Cité) sont donc des **tendances**, pas des positions fixes.
- Serrures, coffres, alarmes et caméras appartiennent à la **part serveur** de la ville (§4.3, **décidé**, S33) : le client ne les connaît que lorsque le personnage les perçoit. La ville fournit aussi les **lieux discrets** où se trouvent les caches (§5.3) ; leur emplacement exact dans le bâtiment est tiré par monde (§4.3, **décidé**, S3 et E25).

### 5.6 Propagation du son

Le son traverse les murs et les planchers en s'atténuant, selon les valeurs d'atténuation des éléments du kit (§4.4) : une cloison laisse passer une dispute, un mur porteur l'étouffe, une porte ou une fenêtre ouverte la laisse passer. La ville fournit cette atténuation entre pièces, étages et extérieur ; elle sert à la perception des personnages ([ia-pnj.md](ia-pnj.md), §4.1) et aux icônes de son de l'interface ([interface.md](interface.md), §6.4). Le serveur décide de ce qui est perçu ; le client applique ensuite le filtrage et la **réverbération** du type de pièce (§4.4), sans jamais rendre audible ce que le serveur n'a pas envoyé ([direction-artistique.md](direction-artistique.md), §5.3).

**Isolation entre logements (décidé, [social.md](social.md), §5.11)** : chaque bâtiment reçoit une **classe d'isolation** selon son époque et sa typologie (V6, §4.2), qui fixe l'atténuation des murs mitoyens et des planchers entre logements, et donc la gêne due au bruit entre voisins :

| Classe | Bâtiments | Effet entre voisins **[valeurs à calibrer]** |
|---|---|---|
| **Faible** | Immeubles anciens à planchers bois (la Cité, faubourg Saint-Roch), barres et tours préfabriquées des années 1960-1970 (les Peupliers), foyer de travailleurs | Une radio, une dispute ou des pas s'entendent chez le voisin et à l'étage du dessous |
| **Moyenne** | Immeubles de rapport en pierre (les Tilleuls, quartier de la Gare), cités ouvrières en bande (les Forges), résidences étudiantes | Musique forte et cris s'entendent, une conversation non |
| **Bonne** | Immeubles contemporains (Beaulieu, Hautevigne, constructions en cours de partie), maisons mitoyennes récentes | Seule une musique très forte ou une fête traverse |
| **Sans mitoyen** | Pavillons et villas isolés | Le bruit ne passe que par les fenêtres, vers le jardin ou la rue |

Une rénovation (§6.1) peut améliorer la classe d'un bâtiment. Pourquoi : la gêne entre voisins dépend d'abord des murs ; tirer l'isolation de l'époque et de la typologie, que le générateur connaît déjà, rend les conflits de voisinage plus fréquents là où ils le sont dans une vraie ville (immeubles anciens et grands ensembles).

## 6. Une ville qui évolue

### 6.1 Changements d'usage (validé)

Les bâtiments changent d'usage au fil du temps, poussés par l'économie et les habitants : un commerce ferme et un autre ouvre à sa place, un logement se vide ou est racheté, une friche devient un squat.

### 6.2 Constructions et démolitions (validé)

- Le générateur réserve des **parcelles constructibles** (terrains vagues, friches) ; tout bâtiment peut aussi être **démoli** pour libérer sa parcelle.
- Chaque parcelle constructible a un **propriétaire** (souvent la mairie) et un **prix** ([economie.md](economie.md), §5.4). Le générateur en garde une réserve **près de la gare et des quartiers populaires** (faubourg Saint-Roch, les Peupliers, les Forges) **[à calibrer]**, pour que l'économie puisse y faire construire des résidences quand les logements manquent ([economie.md](economie.md), §5.7).
- Les **règles d'urbanisme** de chaque quartier (usages permis) servent aussi à l'économie pour repérer où un commerce peut ouvrir ([economie.md](economie.md), §4.5).
- Un projet est porté par un **propriétaire** (habitant, entreprise, mairie) et doit respecter les **règles d'urbanisme du quartier** (hauteur maximale, usages permis), éventuellement assouplies par une décision de la mairie.
- Le nouveau bâtiment est créé **par le même générateur**, avec une typologie compatible avec le quartier.
- **Génération en arrière-plan (décidé, [P34](../revues/generation.md))** : dès le dépôt du permis, les étapes V5 à V13 (§4.2) tournent sur la parcelle, sur une **copie** des données et sur un **fil de fond** ; le résultat est **appliqué à une échéance fixe** (fin du gros œuvre, puis livraison). Démolitions et changements d'usage suivent la même règle. Pourquoi : aucun à-coup pendant le jeu, et un résultat qui ne dépend ni de la vitesse de la machine ni de l'heure où le calcul a fini.
- Il est réalisé par une **entreprise de bâtiment**, dont les employés sont des habitants qui travaillent sur le chantier. Le chantier est visible et passe par des étapes (palissade, gros œuvre, finitions), sur une durée réaliste de plusieurs semaines de jeu **[à calibrer]**.
- Une démolition oblige les occupants à **déménager**.

**Qui décide :**

| Acteur | Rôle |
|---|---|
| **Propriétaires** (habitants, entreprises, promoteurs, mairie) | Déposent les projets de construction, de rénovation ou de démolition |
| **Mairie** de la commune | Délivre ou refuse les permis selon les règles d'urbanisme du quartier ; peut les assouplir par décision du conseil municipal |
| **Entreprise de bâtiment** | Réalise le chantier |

Un joueur participe en incarnant l'un de ces acteurs : le propriétaire ou le promoteur qui porte le projet, un élu ou un employé de la mairie qui instruit le permis, un ouvrier ou le patron de l'entreprise de bâtiment.

### 6.3 Dégâts matériels (validé : persistants jusqu'à réparation)

- Les éléments du décor ont un **état** : intact, endommagé, détruit (vitres, portes, serrures, mobilier, véhicules, murs). S'y ajoutent les **salissures** (tags, déchets) et l'**incendie**, qui se propage dans un local. Les **tags** servent aussi de **marqueurs de territoire** : chacun porte la signature de la bande qui l'a peint ([social.md](social.md), §6.6).
- Un dégât reste en l'état **jusqu'à ce qu'un habitant le répare** : vitrier, serrurier, garagiste, entreprise de bâtiment, ou le propriétaire lui-même. Les réparations créent du travail et coûtent de l'argent (domaine économie).
- **Traces temporaires** : sang, éclats de verre, douilles, ruban de scène de crime sont des **entités du décor**. Elles restent en place jusqu'à leur **nettoyage** (habitant, employé municipal, entreprise de nettoyage) ou leur **prélèvement** par la police ([combat.md](combat.md), §3.4).
- **Pas d'effondrement** : un bâtiment ne s'écroule pas ; un incendie grave le rend inutilisable jusqu'à sa rénovation ou sa démolition.
- L'**état d'entretien** d'un quartier se voit (façades, éclairage, propreté) et peut peser sur les prix et le sentiment de sécurité.

### 6.4 Jour et nuit

La ville gère ce que le cycle du jour change dans le décor :

- **éclairage public**, inégal selon les quartiers (bien éclairé dans la Cité, faible aux Quais) et dégradable (lampadaire cassé) ;
- **fenêtres allumées** selon la présence et les activités réelles des occupants ;
- enseignes et vitrines allumées selon les **horaires d'ouverture**.

### 6.5 Météo et saisons

La météo et les saisons sont définies dans le socle ([01-concept.md](../01-concept.md), §3). Leurs effets sur la ville :

| Effet | Détail |
|---|---|
| **Éclairage** | Jours courts en hiver : l'éclairage public et les fenêtres s'allument plus tôt (§6.4) |
| **Routes et trottoirs** | Verglas, neige, flaques : adhérence et vitesse réduites ; déneigement et salage sont des tâches pour les employés municipaux |
| **Végétation** | Arbres, parcs et jardins changent avec les saisons (feuillage, fleurs, pelouses) ; entretien par les jardiniers municipaux et privés |
| **Crue de l'Ostrelle** | Après de fortes pluies, le fleuve peut monter : quais et berges inondés (les Quais, Forges-sur-Ostrelle en aval), navette fluviale suspendue, caves inondées, dégâts persistants à réparer (§6.3) **[à calibrer]** pour la fréquence |
| **Transports** | Navette fluviale suspendue par crue ou tempête ; bus ralentis par la neige |
| **Décor saisonnier** | **Illuminations de fin d'année** dans la Cité et à Hautevigne **[période à calibrer]**, marché de Noël sur la place de la Cité, terrasses et parasols au printemps et en été, feuilles au sol en automne, sel sur les trottoirs en hiver. Ce sont des **éléments de l'espace public** posés par V9 (§4.2), **pilotés par la date** du monde : rien n'est ajouté ni retiré à la main **(décidé, [direction-artistique.md](direction-artistique.md), §2.5)**. Pourquoi : le décor suit le calendrier de chaque monde sans intervention, comme la végétation |

### 6.6 Signes visibles des événements de vie

Les événements de la vie des habitants laissent des traces dans le décor, pour que le joueur puisse les remarquer sans qu'aucune information ne soit affichée ([interface.md](interface.md), §8.4 et §8.6) :

| Événement | Signes visibles |
|---|---|
| Déménagement | Cartons dans le logement et sur le trottoir, camion garé sur l'aire de livraison (§7.4) |
| Fermeture d'un commerce | Rideau baissé, affiche de fermeture, vitrine vide |
| Ouverture d'un commerce | Travaux, affiche « bientôt ouvert », nouvelle enseigne |
| Longue absence | Boîte aux lettres qui déborde, volets fermés |
| Logement à louer ou à vendre | Panneau à la fenêtre ou sur la façade |

## 7. Déplacements et transports

### 7.1 Temps de trajet **[à calibrer]**

À l'échelle réelle, et **en temps réel en ligne** (une journée de jeu = 24 heures réelles), les distances comptent vraiment :

| Trajet | Distance | À pied | Vélo | Bus | Voiture |
|---|---|---|---|---|---|
| Traverser le vieux centre | 0,8 km | 10 min | 3 min | — | 3 min |
| Rive nord ↔ rive sud | 1,5 km | 18 min | 6 min | 8 min | 4 min |
| Ville-centre ↔ une commune | 3,5 km | 42 min | 12 min | 15 min | 7 min |

On peut vivre à pied dans son quartier ; changer de commune demande un moyen de transport.

### 7.2 Déplacements individuels

- **Marche et course** : vitesse selon les attributs et la fatigue.
- **Vélo, trottinette, deux-roues motorisé, voiture, utilitaire, camion** : possédés par les habitants ou les entreprises (domaine économie), garés dans la rue, dans des parkings ou des garages privés, ouverts avec une clé, et pouvant être volés.
- **Taxis** : quelques taxis conduits par des habitants.
- Conduire un véhicule n'est pas « au repos » ([04-joueur.md](../04-joueur.md), §5) : on ne peut pas prendre le contrôle d'un conducteur, mais on peut prendre celui d'un passager.

### 7.3 Transports en commun (validé : bus, train régional, navette fluviale)

| Mode | Desserte | Fréquence **[à calibrer]** | Équipage |
|---|---|---|---|
| **Bus** | 3 lignes de jour : rive nord ↔ rive sud par le centre ; centre ↔ commune aisée ; centre ↔ commune populaire et zone industrielle. Une ligne de nuit | 15 à 30 min le jour, 60 min la nuit | Chauffeurs habitants, dépôt de bus |
| **Train régional** | Gare de la ville-centre et halte de la commune populaire ; la ligne sort de la carte aux deux bouts, vers l'extérieur | 30 à 60 min | Cheminots habitants : pendant que le train est hors carte, ils sont « à l'extérieur » |
| **Navette fluviale** | Pontons : commune aisée (amont), vieux centre, port, commune populaire (aval) | 30 à 60 min | Pilote habitant |

- **Réseau généré (décidé, [P16](../revues/generation.md))** : à partir des **tracés de lignes** du plan directeur, le générateur place les **arrêts** (tous les 300 à 400 m sur les rues praticables par un bus, à moins de 400 m de 90 % des logements desservis **[à calibrer]**), construit les **horaires** sur les fréquences ci-dessus et déduit des temps de parcours le **nombre de véhicules et de conducteurs**, qui alimente les postes du Programme (dépôt de bus, cheminots, pilote de la navette). Pourquoi : des horaires réels et des postes indispensables ne peuvent venir que d'un tracé et d'une durée de parcours.
- Les horaires sont réels et affichés aux arrêts. Si un chauffeur ne vient pas travailler (malade, mort, en grève), le service est **réellement perturbé**.
- Les transports en commun sont exploités par la **régie d'Orvenne** ; leurs prix sont fixés dans [economie.md](economie.md), §1.4 et §8.1.
- **Pas de métro ni de tramway**, peu crédibles pour 10 000 habitants et coûteux en 2.5D (souterrains).

### 7.4 L'extérieur (validé : extérieur abstrait)

- **Points de sortie** : les routes (rocade aux deux bouts, route des coteaux), la voie ferrée (deux bouts), le fleuve (amont et aval).
- **Déménagements** : les arrivées et départs passent par la gare, la halte des Forges et les routes ([population.md](population.md), §7.4). Des **aires de livraison** près de la gare, de la halte et devant les immeubles permettent de garer un camion de déménagement.
- Un habitant peut partir **à l'extérieur** (vacances, déplacement professionnel, visite de famille, déménagement) puis revenir. Pendant son absence, il **n'est pas simulé** en détail : il a un motif, une date de retour, et éventuellement quelques effets statistiques (dépenses, nouvelles).
- De l'extérieur arrivent les **marchandises** (domaine économie) et de **nouveaux habitants**, qui compensent les départs et les décès et deviennent des personnages complets.
- Les camions qui livrent sont conduits par des chauffeurs habitants (entreprise de logistique). Les **trains et péniches de marchandises en transit** sont des décors mobiles sans équipage visible, ce qui n'est pas contraire à la règle « aucun figurant » puisqu'on n'y croise personne.

### 7.5 Pas de voyage rapide en ligne

Règle du socle ([01-concept.md](../01-concept.md), §2) : pas de voyage rapide en ligne. Pour la ville, cela confirme que les **temps de trajet du §7.1** sont vécus en temps réel, en ligne comme en solo (une heure de jeu = une heure réelle), sauf quand le joueur fait avancer le temps en solo. Les transports en commun et les véhicules sont donc essentiels pour changer de commune.

## 8. Zones et niveaux de détail

### 8.1 Découpage spatial

Principe repris dans le socle ([05-architecture.md](../05-architecture.md), §2, point 12) ; le détail est ici.

| Échelle | Taille | Rôle |
|---|---|---|
| **Cellule** | Carré de 64 m **[à calibrer]** | Recherches spatiales (qui est près de qui), choix de ce qui est envoyé à chaque joueur, niveau de détail de la simulation |
| **Secteur** | Environ un quartier (0,5 à 1 km²), soit 12 à 15 secteurs | Chargement, sauvegarde par lots, et plus tard **répartition entre serveurs** (un serveur par groupe de secteurs) |
| **Agglomération** | Tout le monde | Un seul serveur au départ |

- Les intérieurs appartiennent à la cellule de leur bâtiment, tous étages compris.
- Les limites de secteur suivent les **coupures naturelles** (fleuve, voie ferrée, grands axes) pour limiter les passages d'un serveur à l'autre ; les ponts sont des points de passage bien identifiés.
- **Secteurs figés (décidé, [P14](../revues/generation.md))** : les secteurs sont **calculés** par un partitionnement du graphe des lieux (coupures naturelles, équilibre de population et d'emplois), puis **figés** avec leurs identifiants. Pourquoi : la sauvegarde par lots et la répartition entre serveurs ne doivent pas changer à chaque régénération.
- **Zones de bâtiment (décidé, P14)** : un bâtiment de plus de 2 000 m² de plancher ou qui couvre plus de 2 cellules **[à calibrer]** (usine, hôpital, hypermarché, plateforme logistique, lycée, gare) est découpé en **zones** (ailes, halls, étages), qui servent d'unité au niveau de détail à la place du bâtiment entier (§8.2). Pourquoi : un joueur dans un atelier de l'usine ne doit pas mettre ses 300 employés en simulation complète.

### 8.2 Niveaux de détail de la simulation

**Partage des rôles** : la ville définit **où** s'applique chaque niveau ; ce que fait un personnage à chaque niveau (décisions, mémoire, résolution des actions, transitions) est défini dans [ia-pnj.md](ia-pnj.md), §6. Les noms des niveaux sont communs aux deux domaines.

| Niveau | Où il s'applique | Structures de la ville utilisées (§8.3) |
|---|---|---|
| **Complet** | **Bulle** autour de chaque personnage piloté par un joueur : cellules à moins de ≈ 150 m **[à calibrer]**, et tout le bâtiment qu'il occupe (pour un grand bâtiment : sa **zone** et les zones contiguës visibles, §8.1) | Maillage de navigation fin, collisions, état détaillé du décor (portes, objets, dégâts) |
| **Simplifié** | Reste de la **zone chargée** : les secteurs où se trouve un joueur et les secteurs voisins | Graphe des rues, graphe des pièces et des portes, durées de trajet |
| **Statistique** | Reste de l'agglomération | Graphe des lieux et temps de trajet par mode |

**Exceptions** (décidées dans [ia-pnj.md](ia-pnj.md), §6.2 et §2.4) :

- les **personnages importants** ne descendent **jamais en statistique** : ils restent au moins en simplifié, même loin des joueurs. Sont importants les personnages de palier 7 et plus, les hors catégorie, ceux impliqués dans un événement en cours avec un joueur, et ceux récemment incarnés. La ville doit donc fournir les structures du niveau simplifié (graphes des rues et des pièces) **partout**, pas seulement dans la zone chargée ;
- en **solo**, quand le joueur **fait avancer le temps**, une **bulle autour de lui reste en simplifié** ; le reste de l'agglomération passe en statistique (sauf les personnages importants).

Le placement plausible des personnages quand un joueur approche relève aussi de [ia-pnj.md](ia-pnj.md), §6.3 ; la ville y contribue en fournissant le lieu et l'itinéraire prévus par leur emploi du temps.

### 8.3 Structures de navigation fournies

- **Maillage de navigation fin** (trottoirs, rues, intérieurs), par cellule.
- **Graphe des pièces et des portes** de chaque bâtiment.
- **Graphe des rues** pour piétons et véhicules (sens uniques, feux, passages piétons).
- **Graphe des lieux** avec les **temps de trajet par mode** (à pied, vélo, voiture, transports en commun), pour le niveau statistique et pour la planification des personnages.
- **Lignes et horaires** des transports en commun.

Ces structures sont des **données dérivées** (étape V13, §4.2) :

- **Géométrie physique étagée (décidé, [P13](../revues/generation.md))** : par niveau (sol extérieur, étages, cave, toit), des polygones de sol et des segments de murs avec leur plage de hauteur ; escaliers, rampes et ascenseurs relient les niveaux ; collisions en 2D par niveau. Pourquoi : la ville est faite d'étages discrets ; ce modèle coûte dix fois moins qu'une géométrie 3D, et le passage à la 3D ne touche que le rendu.
- **Itinéraires (décidé, [P15](../revues/generation.md))** : une **hiérarchie de contraction personnalisable** (CCH) par mode (marche, vélo, voiture ; recherche sur horaires pour les transports en commun). La structure est calculée une fois ; une rue fermée, une crue ou le verglas ne demandent qu'une personnalisation des poids (≈ 100 ms **[à mesurer]**). Les temps de trajet sont calculés **à la demande** et mis en cache pour les trajets fréquents, sans matrice complète. Pourquoi : une matrice complète coûterait des centaines de mégaoctets et deviendrait fausse à la première rue fermée.
- **Maillage de navigation en cache (décidé, [P31](../revues/generation.md))** : le maillage fin par cellule est **dérivé** des polygones de sol, mis en cache par empreinte et jamais source de vérité ; il est reconstruit localement après une construction ou une démolition. Pourquoi : démarrage rapide et mémoire bornée, sans risque de servir un maillage périmé.

## 9. Ce que la ville fournit aux autres domaines

### 9.1 Le registre des lieux

Chaque lieu (logement, commerce, entreprise, service, espace public) est une entité du monde, décrite par :

| Donnée | Exemple |
|---|---|
| Identifiant et adresse | Commune, rue, numéro, étage, porte |
| Type et usage actuel | Boulangerie, appartement T3, entrepôt |
| Propriétaire et occupants | Personnage, ménage, entreprise, mairie |
| Horaires d'ouverture | 7 h – 19 h 30, fermé le lundi |
| Activités proposées | Acheter, manger, travailler, dormir, se soigner, se divertir, prier, s'entraîner |
| Postes de travail | Métier, nombre, horaires, palier typique, et si le poste est **indispensable** (son absence prolongée bloque un service : chauffeur de bus, médecin des urgences, seul boulanger d'un quartier) ([population.md](population.md), §1.3) |
| Capacité | Nombre de personnes accueillies |
| Règles d'accès | Public, clients, employés, résidents, clé |
| Surveillance | Alarmes, caméras privées et municipales avec leur **champ** (§5.5), gardien, éclairage, passage. **Part serveur** (§4.3) |
| Plan public | Ce qui figure au **plan public** (établissement ouvert au public, enseigne, horaires affichés) ou reste **privé** (logement, local sans enseigne) ([interface.md](interface.md), §7.1) |
| Serrures et coffres | Niveau de chaque serrure et coffre du lieu (§5.5). **Part serveur** (§4.3) |
| Caches | Emplacement exact des caches dans le bâtiment, tiré par monde (§4.3). **Part serveur** |
| Sécurité perçue | Lieu jugé sûr ou dangereux, selon la surveillance, l'entretien, les incidents récents, les incidents connus par la **rumeur** et la **présence des bandes** ([social.md](social.md), §5.3) ; alimente le besoin de sécurité des PNJ ([ia-pnj.md](ia-pnj.md), §9) |
| Activité de nuit | Ouvert ou occupé la nuit (bars, hôpital, police, transports, gardiennage), pour que la nuit en ligne ne soit pas vide |
| Heures calmes | Plages où le lieu et ses abords attendent le calme, dans les **usages du lieu** **(décidé, [social.md](social.md), §5.11)** : logements de 22 h à 7 h et le dimanche matin, hôpital et maison de retraite en permanence dans les chambres, écoles et bureaux sans heures calmes (vides la nuit), bars et boîte de nuit sans heures calmes pendant leur ouverture **[à calibrer]**. Un bâtiment mixte applique aux logements leurs propres heures calmes. Pourquoi : le seuil de gêne due au bruit baisse fortement aux heures calmes, et ce sont les usages du lieu, non l'heure seule, qui disent quand le bruit est déplacé |
| Isolation | Classe d'isolation du bâtiment entre logements (§5.6) |
| État | Entretien, dégâts |
| Parcelle et bâtiment | Pour les constructions et démolitions |
| Année de construction | Année de construction du bâtiment, tirée dans l'**époque** de sa parcelle et de sa couche d'époque (étapes V3 et V6, §4.2) ; pour une construction en cours de partie, la date de livraison. Elle borne la date d'emménagement des biographies à rebours ([population.md](population.md), §1.1 et §5.1). **(décidé)** : l'époque est déjà connue du générateur, et une année précise rend cohérentes les biographies et les conversations (« l'immeuble a été construit en 1968 ») |
| Titre | Propriétaire unique ou copropriété ([economie.md](economie.md), §5.1) |
| Bail | Locataire, loyer et durée du bail en cours ([economie.md](economie.md), §5.2) |
| Valeur estimée | Prix de marché du lieu ([economie.md](economie.md), §5.2) |
| Vacant | Lieu inoccupé, à louer ou à vendre ([economie.md](economie.md), §4.7) |
| Territoire | Bande qui revendique le lieu, s'il y en a une ([social.md](social.md), §6.6) |

**Attentes des habitants (décidé, [ia-pnj.md](ia-pnj.md), §13.8)** : le **type et usage**, les **horaires d'ouverture**, les **règles d'accès** et l'**activité de nuit** du registre disent ce qui est **normal** dans un lieu : une boulangerie ouverte à 7 h, un bureau vide et fermé à clé la nuit, une école calme le dimanche, un bar animé en soirée. L'IA en tire les attentes de chaque habitant sur ses lieux familiers ; un écart (lumière dans un commerce fermé, porte ouverte hors des horaires, quelqu'un dans un local réservé aux employés) est une **anomalie**. Ces données sont lues dans la **part publique** (§4.3) et l'état réel perçu par l'habitant, jamais dans la part serveur qu'il ne connaît pas. Pourquoi : un habitant juge le normal d'après ce qui est affiché et ce qu'il a l'habitude de voir, comme le ferait un joueur.

### 9.2 Par domaine

| Domaine | Ce que la ville fournit |
|---|---|
| **IA des PNJ** | Logements, postes de travail et horaires, lieux pour chaque besoin, écoles pour les enfants, lieux d'oisiveté (bancs, parcs, places), temps de trajet, structures de navigation et de niveaux de détail ; usages publics et horaires pour les **attentes** (§9.1) ; lieux des **pratiques sociales** : cimetières, pompes funèbres, salles des fêtes, salles de réunion (§5.2) ; coiffeurs, vestiaires, laveries et lieux de musique (§5.3 bis) |
| **Économie** | Commerces, entreprises, propriété des bâtiments, logements et véhicules, quartiers aux prix différents (loyers, immobilier), chantiers, réparations, échanges avec l'extérieur, tickets de transport |
| **Combat, police et justice** | Commissariats, caserne, hôpital, tribunal, surveillance par lieu, lieux discrets, armurerie, portes et serrures, dégâts |
| **Social** | Heures calmes (§9.1) et isolation entre logements (§5.6) pour la gêne due au bruit ; lieux de rencontre (bars, marché, associations, lieux de culte, clubs), services sociaux et locaux associatifs (§5.2), voisinage (même immeuble, même palier), **territoires des bandes exprimés en lieux et en cellules** (§8.1, §9.1), tags comme marqueurs de territoire (§6.3), réputation de quartier |
| **Interface** | Carte, adresses, panneaux et plaques de rue, plan et horaires des transports, vue en coupe des bâtiments et choix de l'étage affiché |

## 10. Points ouverts

Aucune question de conception propre à la ville ne reste ouverte. Restent :

- les valeurs **[à calibrer]**, que les sessions ajustent au prototype du premier quartier (§2.1) ;
- les décisions de la ville qui doivent être confirmées par le socle ou par d'autres domaines (§11).

## 11. Impacts sur le socle et les autres domaines

À trancher par la session principale ; rien n'a été modifié ailleurs.

### 11.1 Socle

| Fichier | Impact | État |
|---|---|---|
| [01-concept.md](../01-concept.md), §1.1 | Description du monde : agglomération d'Orvenne sur l'Ostrelle, 10 000 habitants, **aucun figurant**, même carte initiale pour tous les mondes | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 | **Pas de voyage rapide en ligne** ; en solo, une heure de jeu = une heure réelle | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §3 | **Météo et saisons** dans le socle ; effets sur la ville au §6.5 | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4 | Paliers calculés sur **toute la population, enfants compris** (décision de l'utilisateur, contraire à la recommandation de la ville) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4 | Non-jouabilité temporaire : **à l'extérieur**, détenus compris (§5.2) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §3 | Réveil du personnage persistant à l'hôpital public, ou à la clinique privée selon ses moyens | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 | Ville comme donnée, **générateur en Rust**, carte initiale versionnée + journal des modifications, cellules et secteurs | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §1 | Préciser la disparition du personnage persistant à la déconnexion : **lieux de retrait**, délai d'environ une minute ailleurs, réapparition au même lieu de retrait ou à la gare (§2.5) | Intégré au socle le 29/10 (02-personnages.md §1.1) |
| [02-personnages.md](../02-personnages.md), §3.4 | Les personnages persistants ne comptent pas dans la population ni dans le classement des paliers (§2.5) | Intégré au socle le 29/10 (02-personnages.md §1.1) |
| [04-joueur.md](../04-joueur.md), §6 | Au départ, le personnage persistant arrive à Orvenne par le train, avec un petit pécule et quelques nuits payées (§2.5) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 | En solo, le monde est **en pause** quand le jeu est fermé : le joueur reprend là où il s'était arrêté (§2.5) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §1.1 | Retrait sous menace dans tous les lieux de retrait, plafond de 5 minutes ; véhicule qui n'est pas le sien laissé à la sortie ; dépôt de ce qu'il porte en disparaissant chez lui (§2.5, E19, E20) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1 | Même carte initiale pour tous les mondes, mais **couche cachée** tirée par monde avec sa graine (état des alarmes, emplacement des caches) (§4.3, S3, E25) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 | Donnée de la ville en **part publique** livrée au client et **part serveur** jamais livrée ; tirage de la couche cachée à la création du monde (§4.3, S33, S3) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.11, §2.12, §2.14, §2.19, §3 | Chaîne de génération (Programme, plan directeur, V1 à V14), part serveur tirée par monde parmi des candidats, centimètres entiers, identifiants stables, surcharges, géométrie étagée, itinéraires CCH, zones de bâtiment ([revues/generation.md](../revues/generation.md)) | Intégré au socle le 29/10 par la session principale |
| [05-architecture.md](../05-architecture.md), §5 | Pour le premier prototype « un quartier », prendre un morceau du **faubourg Saint-Roch**, le **Pont-Vieux** et le bord de **la Cité** : logements, commerces, bars, un atelier et un trajet à travers le fleuve. Ce prototype sert aussi à ajuster les dimensions (§2.1). Pourquoi : ce morceau rassemble sur peu d'espace les situations les plus variées (logements populaires et anciens, commerces, bars, un atelier, un pont à franchir) | Intégré au socle le 29/10 |

### 11.2 Autres domaines

| Domaine | Impact |
|---|---|
| **IA des PNJ** | Niveaux de détail **harmonisés le 29/10** : noms communs (complet, simplifié, statistique), la ville définit où, [ia-pnj.md](ia-pnj.md) §6 définit quoi, exceptions des personnages importants et de l'accélération en solo reprises au §8.2. Restent côté IA : placement plausible lors du passage d'un niveau à l'autre ; emplois du temps qui incluent les trajets réels et les horaires des transports ; départs et retours de l'extérieur ; vie des enfants (école, famille) ; perception des absences d'un personnage persistant par ses voisins et son employeur, et son comportement automatique pendant le délai avant disparition (§2.5) |
| **Économie** | Propriété et prix des logements, locaux, parcelles et véhicules ; ouverture et fermeture des commerces ; financement des constructions et des réparations ; marchandises venues de l'extérieur ; prix des transports ; personnage persistant (§2.5) : pécule d'arrivée, location et achat par les joueurs, construction de résidences quand la demande monte, emplois à la tâche, avertissements et renvoi pour absence, relances et expulsion pour loyer impayé |
| **Combat, police et justice** | **Pour la phase 1** : le tapage nocturne, s'il entre au catalogue des infractions, peut s'appuyer sur les **heures calmes** du registre (§9.1) plutôt que sur une heure fixe. Crochetage et effraction des portes ; dégâts et incendies (§6.3) ; vol de véhicules ; surveillance des lieux ; confirmer l'organisation de la police (commissariat central pour toute l'agglomération, police municipale par commune, pas de gendarmerie) et la **prison à l'extérieur** (§5.2) ; délai d'environ une minute avant la disparition d'un personnage persistant déconnecté hors d'un lieu de retrait, pour qu'on ne puisse pas fuir un combat ou une arrestation (§2.5) ; la menace retarde aussi la disparition dans un lieu de retrait, et une perquisition du logement peut trouver ce que le persistant y a déposé (§2.5, E19, E20) ; serrures, alarmes, caméras et caches sont des données du serveur, transmises seulement si perçues (§4.3, S33) |
| **Population** (revue de la génération) | Le parc de logements vient du Programme (G0) que la population remplit ; le **mobilier mobile** est produit à l'emménagement et déménage avec le ménage (§4.2, P1, P11) ; critère commun des 32 × 32 graines (§4.6, P37) |
| **Social** | Bandes et territoires de quartier ; réputation des quartiers ; relations de voisinage ; arrivées de nouveaux habitants |
| **Interface** | Vue en coupe et choix de l'étage, y compris l'étage −1 et les toits (§5.1) ; carte et adresses ; ce que le personnage incarné connaît de la ville (un habitant connaît ses lieux habituels, un nouveau venu non) ; signaler au joueur les lieux de retrait et le délai avant disparition, y compris quand la menace l'empêche de partir derrière sa porte (§2.5, E19) ; n'afficher de la part serveur que ce que le personnage perçoit (§4.3, S33) |
| **Direction artistique** | Identités visuelles des quartiers (§3.5), textes rendus et surfaces de texte du kit, réverbération par type de pièce (§4.4), graines de logos des marques fictives (§4.2, V11), décor saisonnier (§6.5) : la ville fournit les données sémantiques, le client les rend ([direction-artistique.md](direction-artistique.md), §12.5) |
| **Population** (création des habitants) | Réglé par [population.md](population.md), §1 et §3 : la population crée les habitants, leurs ménages, leurs emplois et leurs logements à partir du Programme et du registre des lieux (§2.3, §4.2, §9.1) |

### 11.3 Demandes des autres domaines (traitées le 29/10)

| Domaine | Demande | Traitement dans ce fichier |
|---|---|---|
| [population.md](population.md), §12.2 | Maison de retraite et maternité | §5.2 |
| | Hébergement collectif (résidences étudiantes, foyer, squats) | §2.3, §5.3 |
| | ≈ 4 740 logements pour garder 5 % de vacance (chiffre harmonisé par P41) | §2.3 |
| | Types de logements par quartier selon les ménages | §3.3 |
| | École privée élémentaire | Oui, à Hautevigne (§5.2) |
| | Postes indispensables dans le registre | §9.1 |
| | Stationnement des camions de déménagement | §7.4 |
| [economie.md](economie.md), §14.2 | Intérim, Maison de l'emploi, notaire, concession, garde-meubles, fourrière, Orvenne Habitat | §5.2, §5.3 |
| | Titre, bail, valeur estimée, vacant dans le registre | §9.1 |
| | Parcelles constructibles avec propriétaire et prix, réserve près de la gare et des quartiers populaires | §6.2 |
| | Chambres d'hôtel occupées hors champ | §5.3 |
| | Urbanisme et opportunités de commerce, prix des transports | §6.2, §7.3 |
| [social.md](social.md), §12.6 | Services sociaux et locaux associatifs | §5.2 |
| | Territoires de bandes en lieux et cellules, tags comme marqueurs | §6.3, §9.1, §9.2 |
| | Sécurité perçue alimentée par la rumeur et les bandes | §9.1 |
| [combat.md](combat.md), §16.2 | Commissariat central dans le quartier de la Gare | §5.2 |
| | Intérieurs du commissariat et du tribunal | §5.4 |
| | Commissaires de justice et mandataire dans la Cité ; bâtiment administratif de l'État | §5.2, §5.3 |
| | Surveillance (alarmes, caméras et leur champ) | §5.5, §9.1 |
| | Serrures de difficultés différentes et coffres | §5.5, §9.1 |
| | Traces temporaires comme entités du décor | §6.3 |
| | Bords des toits, quais et ponts comme obstacles | §5.1 |
| [interface.md](interface.md), §15.6 | Plan public ou privé dans le registre | §9.1 |
| | Plaques de rue et panneaux d'horaires dans les kits | §4.4 |
| | Propagation du son à travers murs et planchers | §4.4, §5.6 |
| | Éléments de décor des événements de vie | §6.6 |
| [population.md](population.md), §1.1, §5.1 et §12.9 (revue de la génération, P20) | Année de construction de chaque bâtiment dans le registre | §9.1 |
| [direction-artistique.md](direction-artistique.md), §12.5 | Identités de quartier comme paramètres du plan directeur et des gabarits, pour V6 et V9 | §3.5, §4.2 |
| | Textes des plaques, enseignes et panneaux rendus par le client | §4.4 |
| | Marques fictives générées en V11, logo par graine | §4.2 |
| | Atténuation et réverbération par type de pièce dans le catalogue sémantique | §4.4, §5.6 |
| | Illuminations et décor saisonnier pilotés par la date | §4.2, §6.5 |
| [ia-pnj.md](ia-pnj.md) v1.1, §2.6 et §11 | Salons de coiffure, vestiaires des postes à uniforme, laveries, lieux où l'on joue de la musique | §5.3 bis |
| [social.md](social.md) v0.7, §12.12 et §5.11 | Heures calmes dans les usages des lieux | §9.1 |
| | Isolation entre logements selon l'époque et la typologie | §5.6, §9.1 |

### 11.4 Revues intégrées (29/10)

| Revue | Point | Traitement dans ce fichier |
|---|---|---|
| [equilibrage.md](../revues/equilibrage.md), §6.12 | E19 : retrait sous menace, véhicule laissé à la sortie | §2.5 |
| | E20 : dépôt de ce que porte le persistant qui disparaît chez lui | §2.5 |
| | E25 : couche cachée tirée par monde (emplacement des caches) | §4.3, §5.5, §9.1 |
| [securite.md](../revues/securite.md), §9.8 | S33 : part publique et part serveur de la donnée de la ville | §4.3, §5.5, §9.1 |
| | S3 : état des alarmes tiré à la création du monde | §4.3, §5.5 |
| [ia-pnj.md](ia-pnj.md), §11, revue de l'intelligence (v0.8) | Attentes appuyées sur les usages publics et les horaires des lieux (§13.8) | §9.1, §9.2 |
| | Lieux des pratiques sociales : cimetière, salle des fêtes, salle de réunion (§13.9) | §5.2, §9.2 |
| [generation.md](../revues/generation.md), §13.4 | P1, P41 : ≈ 4 740 logements calculés par le Programme | §2.3, §4.2 |
| | P10 : dimensions minimales élargies des plans types | §2.2, §4.2 |
| | P8 : lieux faits main en pièces avec ports, posées avant les rues | §3.4, §4.2 |
| | P1 à P16 : Programme, plan directeur, chaîne V1 à V14 | §4.2 |
| | P11 : mobilier fixe dans la carte, mobilier mobile dans la population | §4.2 |
| | P12 : part serveur entière tirée par monde parmi des candidats génériques | §4.3, §5.5 |
| | P13, P28 : centimètres entiers, identifiants stables, surcharges | §2.2, §4.3 |
| | P7, P33 : sorties sémantiques, assemblage par le client | §4.4 |
| | P28, P35 : réconciliation et mise à jour différée à la vacance | §4.5 |
| | P37, P2 : critère de 95 % sur 32 × 32 graines, générateur de plans directeurs | §4.6 |
| | P34 : constructions générées en arrière-plan, appliquées à échéance | §6.2 |
| | P16 : arrêts, horaires et personnel générés à partir des tracés | §7.3 |
| | P14 : zones de bâtiment, secteurs calculés puis figés | §8.1, §8.2 |
| | P13, P15, P31 : géométrie étagée, itinéraires CCH, maillage de navigation en cache | §8.3 |
| Arbitrage A8 (30/10, [organisation.md](../../pilotage/organisation.md), §6) | Aucune action humaine : écriture des données « faites main », retouches, aperçu dans Godot, ajustements du prototype et calibrage confiés aux sessions | En-tête, §1, §2.1, §4.1, §4.2, §4.3, §4.6, §10 |
