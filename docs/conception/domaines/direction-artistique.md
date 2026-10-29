# Domaine — direction artistique et sonore

> Fichier de **domaine**, géré par la session `conception-direction-artistique` (voir [README](../README.md)).
> Version 0.4.1 — 30/10/2026 (revue d'architecture, tour 2 : T2-5, §12.3 sans contrôle du catalogue d'animations par le paquet ; T2-4, flux `cosmetic/vocalization` nommé au §5.5 et au §8.3).
> Version 0.4 — 30/10/2026 (corrections des revues de clôture : fonction unique `solar_state` ; apparence perçue à la résolution de sa précision, graine de forme des signes seulement à la précision « visage », classe de voix grossière, test de réidentification et test de parenté élargi ; graines cosmétiques `cosmetic/<usage>` et générateurs du client en fonctions pures d'identifiants publics ; contrôle des familles d'animation côté client ; indice de variante des vocalisations ; assets tiers en formats de données, quarantaine, Blender sans exécution automatique, outils épinglés ; LFS en CI ; §2.2, §3.4 à §3.6, §3.10 d, §5.5, §6.9, §7.2, §8.1, §8.3, §9.3, §10.1, nouveau §12.13 ; aucune section renumérotée).
> Version 0.3.1 — 30/10/2026 (arbitrage A10 : la bulle suit l'interface ; le volume donne le bord, le ton passe par une icône et un mot, aussi pour la bulle sans texte ; §4.4, §12.6, §12.10 et §12.12 alignés ; ligne « bulle sans texte » retirée des impacts pour la phase 1).
> Version 0.3 — 30/10/2026 (dernière vague avant la clôture : social v0.7 (ton, volume et durée des actes entendus sans être compris ; signature graphique de bande et scission), interface v0.7 (gestes dévisager, se changer, coupe au salon, radio), économie v0.7 (véhicules par catégorie, âge, état et gamme ; gamme des vêtements ; organisme de redevance musicale fictif) ; emplacement `data/content/appearance/` confirmé ; nouveau §12.12 ; aucune section renumérotée).
> Version 0.2.1 — 30/10/2026 (règle « aucune action de l'utilisateur », arbitrage A8 : aucun achat, sources gratuites téléchargeables sans compte seulement, Mixamo et Freesound écartés, musique des musiciens générée par nos outils, déclaration des contenus produits par IA par la lecture la plus protectrice ; §7.1 à §7.6, §9.1 à §9.3, §12.9, nouveau §12.11).
> Version 0.2 — 29/10/2026 (retours de ia-pnj v1.1, population v0.8, interface v0.6 et combat v0.5 intégrés : familles de gestes et repli, sources musicales du cœur, plages et lois du génome, catalogue vestimentaire et uniformes, hérédité par les parents officiels, voix sonore, formes de bulle, icônes sirène et klaxon, clés de sous-titre, visage non résolu, musiques de relais et de bilan, contrôle du liseré, motifs de cicatrices, brassard ; nouveau §3.10 et §12.10 ; aucune section renumérotée).
> Version 0.1.1 — 29/10/2026 (états du §12.1 mis à jour après la relecture de cohérence : impacts intégrés au socle).
> Version 0.1 — 29/10/2026 (première rédaction).
> Les choix marqués **(décidé)** sont tranchés par cette session, avec leur raison. Les valeurs marquées **[à calibrer]** seront ajustées au prototype du client 2.5D (phase 3), dans la scène étalon du §10.2. La mention **[à vérifier au registre]** signale une licence citée de mémoire, que le responsable contenus doit relire à la source et dater avant tout usage (§9.1). Les décisions qui touchent le socle ou d'autres domaines restent à confirmer par la session principale (§12).

## Sommaire

0. Principes
1. Style visuel
2. Palette, lumière, météo et saisons
3. Les habitants à l'écran
4. Lisibilité dans le monde
5. Son, musique et voix
6. Cahier des charges des assets
7. Sources et critères de choix
8. Ce que les générateurs produisent seuls
9. Registre des licences et Git LFS
10. Mise en œuvre par le responsable contenus
11. Valeurs à calibrer
12. Impacts sur le socle et les autres domaines

---

## 0. Principes

1. **L'utilisateur ne produit aucun asset** (décidé, contrainte de départ). Chaque modèle, texture, animation, son, musique et police vient d'une **génération procédurale**, d'un **outil automatisé** piloté par script, d'une **source libre** ou d'une **source gratuite sous licence propre**, sans aucun achat (§7.6), et chaque choix de ce fichier doit pouvoir être appliqué par le responsable contenus sans aucun geste artistique manuel. *Pourquoi : c'est la condition de production du jeu ; une direction qui exigerait un artiste ne serait pas applicable.*
2. **Le style naît des règles, pas de la main** (décidé). Une direction artistique tenue par des agents doit être faite de **paramètres vérifiables** : palettes en valeurs, densités de texels, budgets, conventions et une scène étalon (§10.2). Tout asset est contrôlé automatiquement contre ces règles (§6.9). *Pourquoi : sans œil humain pour juger chaque pièce, seule une règle mesurable garantit la cohérence d'ensemble.*
3. **Le protocole est sémantique, le client assemble** ([05-architecture.md](../05-architecture.md), §2.5 et §3 ; [revue de la génération](../revues/generation.md), P7, P33). Le cœur et `ts-worldbuild` ne connaissent **aucun asset** : ils produisent des typologies, des travées, des tenues, des gestes. Le client traduit chaque identifiant sémantique en asset par un **catalogue de correspondances** (§6.7). *Pourquoi : changer d'asset, de style ou passer en 3D ne touche alors ni le serveur ni le protocole.*
4. **Aucun figurant, ni à l'image ni au son** (décidé, extension de [01-concept.md](../01-concept.md), §1.1). Toute personne visible est un habitant ; toute **voix**, tout **pas**, tout **moteur** et tout **klaxon** qu'on entend a une **source réelle perçue**. Le fond sonore se limite à des couches impersonnelles (vent, pluie, oiseaux, rumeur lointaine) (§5.2). *Pourquoi : des voix ou des pas sans personne derrière seraient des figurants sonores et démentiraient la promesse du monde.*
5. **Le rendu ne révèle rien** (décidé, [revue de sécurité](../revues/securite.md), principe 1). Tout ce qui est livré au client est public : assets, catalogue, shaders. Aucun asset, aucun effet, aucune musique ne dépend d'une donnée que le personnage ne perçoit pas, et **rien ne distingue à l'image ou au son un personnage piloté par un joueur** (pas de cosmétique propre aux joueurs, même corps, même voix, même naturel). *Pourquoi : le monde ne connaît pas les joueurs, et ce qui n'est pas envoyé ne peut pas être lu.*
6. **Une ville ordinaire, lisible et sobre** (décidé). Orvenne n'est ni une carte postale ni une dystopie : c'est une ville moyenne où l'on vit. La violence est **lisible et lourde, jamais spectaculaire** ([combat.md](combat.md), §12.3). *Pourquoi : le jeu raconte des vies ordinaires pour un public adulte ; le réalisme et la sobriété servent à la fois la crédibilité et l'éthique du jeu.*
7. **Un seul style de la 2.5D à la 3D** (décidé). Les dimensions, les matériaux, les palettes et le catalogue sémantique sont définitifs dès la 2.5D ; la 3D ajoute du détail et des niveaux de détail, jamais un autre style ([ville.md](ville.md), §4.4). *Pourquoi : refaire la direction artistique au passage en 3D coûterait autant que le jeu lui-même.*

---

## 1. Style visuel

### 1.1 Intention : un réalisme stylisé (décidé)

- **Proportions réelles** (1 unité = 1 m, [ville.md](ville.md), §2.2), **formes simplifiées** et **surfaces calmes** : matériaux reconnaissables à leur couleur, leur rugosité et quelques motifs larges (joints de pierre, briques, lames de volet), sans micro-détail (pores, rayures fines, salissures photographiques).
- **Ni photoréalisme, ni dessin animé.** Le photoréalisme demanderait des scans et une finition humaine que personne ne fera, et des visages procéduraux photoréalistes tomberaient dans la vallée de l'étrange. Le dessin animé contredirait le ton adulte et la sobriété de la violence.
- **Référence de ton** : la lumière et les couleurs d'une ville d'Europe de l'Ouest au climat océanique, ciel souvent couvert, pierre claire, zinc et ardoise, briques des faubourgs, béton des années 1960-1970.

*Pourquoi : le réalisme stylisé est le seul style qu'on peut obtenir entièrement par génération et par des sources libres tout en restant crédible en 2.5D, puis en 3D à la troisième personne.*

### 1.2 Ce que la caméra impose (décidé)

La caméra 2.5D est orthographique, à angle fixe d'environ 45°, avec quatre orientations et trois à quatre crans de zoom ([interface.md](interface.md), §6.1). Conséquences pour les assets :

| Règle | Valeur | Pourquoi |
|---|---|---|
| Taille d'un mètre à l'écran (1080 p) | ≈ 50 px au zoom le plus proche, ≈ 12 px en vue de quartier, valeurs reprises par [interface.md](interface.md), §6.1 ; tout changement des crans est signalé à ce domaine avant d'être appliqué **[à calibrer]** | Fixe la densité de texels utile (§6.4) |
| Détail minimal utile | Pas de forme de moins de 10 cm modélisée en 2.5D (poignées, interrupteurs, cadres) ; ils passent en texture ou disparaissent | Invisible à l'écran, coûteux en triangles |
| Lecture des toits | Toits **visibles et variés** (ardoise, zinc, tuile, toit-terrasse, verrière) : ils sont la surface la plus vue en vue de quartier | En vue plongeante, c'est le toit qui identifie un quartier |
| Quatre faces | Tout bâtiment et tout meuble est **fini sur ses quatre faces** | Les quarts de tour montrent tous les côtés |
| Vue en coupe | Murs coupés à hauteur de hanche avec une **tranche de coupe** pleine et sombre ; sols et plafonds avec épaisseur | La coupe doit se lire comme une maquette, pas comme un décor creux ([interface.md](interface.md), §6.2) |
| Intérieurs | Sols contrastés avec les murs ; mobilier d'au moins deux tons de valeur différente du sol | On lit une pièce d'en haut par ses sols et ses meubles |

### 1.3 Identité des quartiers (décidé)

Chaque quartier se reconnaît d'abord à ses **matériaux**, ses **toits** et ses **arbres**, puis à son mobilier urbain et à son état d'entretien. Les gabarits et époques viennent du plan directeur ([ville.md](ville.md), §4.2) ; ce tableau fixe ce qui se voit.

| Quartier | Façades dominantes | Toits | Arbres d'alignement | Mobilier urbain, signes | Entretien moyen **[à calibrer]** |
|---|---|---|---|---|---|
| **La Cité** | Pierre calcaire blonde, enduits clairs, colombages rares, rez-de-chaussée commerciaux à vitrines bois | Ardoise et zinc, lucarnes | Platanes sur la place, peu d'arbres dans les ruelles | Pavés, bornes, lanternes murales, terrasses | Bon |
| **Quartier de la Gare** | Pierre de taille du XIXᵉ autour de la gare, bureaux contemporains (verre, bardage, béton lisse) | Zinc, toits-terrasses techniques | Platanes alignés | Mobilier contemporain, abribus vitrés, panneaux lumineux | Bon le jour, désert la nuit |
| **Les Tilleuls** | Immeubles de rapport en pierre de taille, balcons filants en fer forgé | Zinc et ardoise, combles mansardés | **Tilleuls** | Grilles de jardin, réverbères à crosse, parc | Très bon |
| **Beaulieu** | Pavillons hospitaliers en brique, bâtiments universitaires et résidences des années 1990-2010 | Tuile mécanique, toits-terrasses | Érables, pelouses | Signalétique hospitalière, vélos | Moyen à bon |
| **Faubourg Saint-Roch** | Brique rouge et enduits ocre ou gris, ateliers à verrières, petits immeubles de 3-4 niveaux | Tuile mécanique, sheds d'ateliers | Marronniers isolés, peu d'arbres | Rideaux métalliques, enseignes peintes, affiches, tags | Moyen |
| **Les Quais** | Entrepôts de brique et de tôle, silos, capitainerie en pierre | Tôle ondulée, sheds, toits plats | Saules et friches | Bollards, rails, grues, grillages, éclairage rare | Faible |
| **Les Peupliers** | Barres et tours en béton préfabriqué, panneaux colorés passés, dalle | Toits-terrasses | **Peupliers** en bordure, pelouses rases | Dalle, coursives, centre commercial de quartier, jeux d'enfants | Faible à moyen, inégal |
| **Hautevigne** | Villas en enduit clair et pierre, maisons de bourg, résidences fermées | Tuile plate et ardoise | Chênes, cèdres, haies taillées | Portails, murets, éclairage doux, golf | Très bon |
| **Forges-sur-Ostrelle** | Cités ouvrières en brique, pavillons crépis des années 1970, zone industrielle en bardage métallique | Tuile mécanique, bacs acier | Bouleaux, tilleuls rares | Grands parkings, enseignes de zone commerciale, cheminées d'usine | Moyen, faible en zone industrielle |

- Le **fleuve** et ses berges (quais de pierre dans la Cité, berges enherbées en amont, quais industriels en aval) servent de repère commun.
- Les **lieux faits main** ([ville.md](ville.md), §3.4) suivent le style de leur quartier, avec un matériau ou une silhouette propre qui sert de repère à distance (beffroi de la mairie, verrière de la gare, tour emblématique des Peupliers, cheminée de l'usine principale).

*Pourquoi : en vue plongeante, un quartier se reconnaît en une seconde à ses matériaux, ses toits et ses arbres, et c'est ce qu'un générateur peut appliquer sans erreur à partir du gabarit et de l'époque de chaque parcelle.*

### 1.4 Une fiction sans marques ni insignes réels (décidé)

- **Aucune marque réelle** : véhicules, produits, enseignes, logos, vêtements, téléphones. Les véhicules ont des formes génériques de leur catégorie (citadine, berline, break, utilitaire, bus articulé, camion, scooter), sans calandre ni silhouette reconnaissable d'un modèle existant.
- **Véhicules par catégorie, âge, état et gamme** (décidé, v0.3, [economie.md](economie.md), §5.9) : la **catégorie** choisit la carrosserie ; l'**âge** choisit la génération de formes (trois époques de dessin générique, des plus anguleuses aux plus arrondies) et la patine ; l'**état** choisit rayures, bosses, rouille, saleté et vitres ; la **gamme** ne se voit que par la **finition et les options** (jantes, enjoliveurs, baguettes, peinture métallisée, toit ouvrant, phares). Aucune calandre, aucun logo ni nom de modèle.
- **Marques fictives générées** : enseignes de commerces, marques de produits, compagnie de bus, radios et journal locaux, **organisme de redevance musicale** ([economie.md](economie.md), §3.8) ont des noms et des logos produits par le générateur de noms ([ville.md](ville.md), §4.2, V11) et un générateur de logos simple (forme, initiales, deux couleurs) (§8.3).
- **Insignes fictifs** : police nationale, police municipale, pompiers, secours, justice ont des uniformes **crédibles d'Europe de l'Ouest** (bleu marine, bandes réfléchissantes, gyrophares bleus) mais des **écussons, armoiries et inscriptions inventés**, sans drapeau ni emblème d'un État réel.
- **Aucune personne réelle** : ni visage, ni nom, ni voix (§3.2, §5.5).

*Pourquoi : la ville est fictive, et une marque, un insigne d'État ou un visage réel exposerait le jeu à des litiges que personne ne peut arbitrer au cas par cas.*

### 1.5 Les textes du monde sont rendus, jamais peints (décidé)

- Tout texte lisible du décor (plaques de rue, numéros, enseignes, horaires, affiches, écrans d'arrêt, étiquettes de prix, journal, panneaux d'information) est **rendu à l'exécution** à partir des données (nom, adresse, horaires), avec une police du §4.6, sur une surface prévue par l'élément du kit.
- Seuls les textes **illisibles par nature** (petits caractères d'une affiche lointaine, journal plié) peuvent être peints dans une texture, à partir d'un faux texte.
- Les **noms propres** restent tels quels ; les textes à contenu s'affichent dans la langue du joueur ([interface.md](interface.md), §11.2).

*Pourquoi : les noms viennent de la génération, changent avec les commerces et doivent se traduire ; un texte peint dans une texture obligerait à refaire l'asset à chaque changement.*

### 1.6 Passage à la 3D (décidé)

- Mêmes dimensions, même catalogue, mêmes matériaux de base ; les modèles 3D sont des **versions plus détaillées** des mêmes éléments (ornements modélisés, menuiseries en relief, cheveux en cartes), avec une densité de texels plus haute (§6.4).
- Le style reste le **réalisme stylisé** : pas de bascule vers le photoréalisme, même en vue à l'épaule.
- Les assets 2.5D restent les **niveaux de détail lointains** des modèles 3D quand leur silhouette est la même (§6.2).

*Pourquoi : réutiliser la 2.5D comme niveau de détail lointain amortit tout le travail de la première version.*

---

## 2. Palette, lumière, météo et saisons

### 2.1 Palette (décidé)

| Famille | Teintes de référence **[à calibrer]** | Règle |
|---|---|---|
| Pierre et enduits clairs | Calcaire `#CFC3A8`, enduit crème `#E3D6BC`, enduit gris `#B8B4AA` | Valeur 65 à 85 % |
| Toits | Ardoise `#4A5059`, zinc `#8A9096`, tuile `#A05A3C` | Plus sombres que les façades, pour détacher les volumes |
| Brique et ocres | Brique `#9C5A43`, ocre `#C49A5C` | Saturation ≤ 45 % |
| Béton, métal | Béton `#A7A39B`, bardage `#6E7479`, tôle rouillée `#7D5440` | — |
| Végétation | Été `#5E7A45`, automne `#B07A35`, hiver (bois) `#5A4D42` | Pilotée par la saison (§2.5) |
| Sols | Asphalte `#3E4144`, pavé `#7A746B`, trottoir `#9A958C` | Toujours plus sombres que les façades voisines |
| Accents | Enseignes, vêtements, signalétique, mobilier d'enfants | Seules surfaces autorisées au-delà de 45 % de saturation, jusqu'à 75 % |

- **Décor désaturé, vie colorée** : façades, sols et toits restent dans des teintes naturelles et peu saturées ; la couleur vive est réservée à ce qui vit ou informe (vêtements, enseignes, signalisation, véhicules). *Pourquoi : en 2.5D, les personnages sont petits ; un décor calme les fait ressortir sans contour ni marqueur.*
- **Plage de valeurs** : aucun albédo du décor sous 10 % ni au-dessus de 85 % de luminance. *Pourquoi : laisser de la marge à l'éclairage, de jour comme de nuit, sans blancs brûlés ni noirs bouchés.*
- **Aucune couleur réservée à l'interface dans le décor** : le liseré de la couche joueur et la couleur de contour de cible ([interface.md](interface.md), §2.7 et §13.2) n'apparaissent jamais sur un asset du monde. *Pourquoi : le joueur doit pouvoir distinguer d'un coup d'œil ce qui est du monde et ce qui est du jeu.*
- Les palettes sont des **données** (`client/catalog/palettes.ron`, §6.8) : quartier, époque et entretien y choisissent les couleurs de chaque instance, et le contrôle automatique vérifie les plages (§6.9).

### 2.2 Lumière du jour (décidé)

- **Soleil calculé** à partir de la date et de l'heure du monde (données publiques) et d'une **latitude fictive de 47,5° N** **[à calibrer]**, rangée dans le paquet de contenus : hauteur, azimut, durée du jour (≈ 8 h 30 en décembre, ≈ 16 h en juin), heure dorée, crépuscule. *Pourquoi : les jours courts d'hiver sont une règle du socle ([01-concept.md](../01-concept.md), §3), et un soleil calculé ne coûte aucune donnée du serveur.*
- **Une seule fonction** (décidé, v0.4, clôture de l'architecture C-3) : `solar_state(date, heure) → {hauteur, azimut, niveau d'ambiance}` dans `ts_world`, exposée au client par `ts_client_native` ([05-architecture.md](../05-architecture.md), §3). Le cœur s'en sert pour la perception et la précision de l'apparence (§3.5) ; le client ne la recalcule **jamais** en C#, il rend ce qu'elle donne (ciel, ombres, heure dorée). *Pourquoi : une petite divergence entre deux calculs montrerait un plein jour à l'écran pendant que le serveur refuse un visage « faute de lumière », et l'écran ne serait plus honnête.*
- **Ciel procédural** (ciel physique de Godot et couche de nuages pilotée par la météo), sans image de ciel fixe. *Pourquoi : le ciel doit suivre l'heure et la météo en continu, ce qu'une image fixe ne fait pas.*
- **Une lumière directionnelle avec ombres**, une lumière ambiante venue du ciel, de l'occlusion ambiante à l'écran, aucun éclairage global précalculé. *Pourquoi : la ville change (constructions, dégâts, lumières des fenêtres) et s'assemble à l'exécution ; un éclairage précalculé devrait être refait à chaque changement.*
- **Ciel couvert par défaut** sur une bonne partie de l'année : lumière diffuse, ombres douces. Le climat océanique en fait la lumière ordinaire d'Orvenne, et elle flatte le réalisme stylisé.

### 2.3 La nuit (décidé)

- Sources de lumière réelles : **éclairage public** (lampadaires de chaque quartier, inégal selon [ville.md](ville.md), §6.4), **fenêtres allumées** (selon l'état transmis), **enseignes et vitrines** selon les horaires, phares des véhicules perçus.
- Les fenêtres et enseignes sont des **matériaux émissifs**, pas des lumières ; seules les sources proches du personnage projettent une vraie lumière, dont un petit nombre avec ombres **[nombre à calibrer]**.
- **Nuit lisible** : la nuit est bleutée et sombre, mais le décor reste lisible à l'écran (silhouettes des bâtiments, rues). Les **personnes**, elles, n'apparaissent que si le personnage les perçoit, et la distance de vision dépend de l'éclairage calculé par le serveur ([interface.md](interface.md), §6.4). *Pourquoi : un écran noir ne sert à rien, mais la nuit doit rester un vrai avantage pour qui se cache ; c'est la vue filtrée du serveur, pas l'obscurité de l'image, qui garantit cet avantage.*

### 2.4 Météo (décidé)

La météo est identique pour tout le monde dans un monde et fait partie de la vue ([01-concept.md](../01-concept.md), §3). Le client la rend par des **paramètres globaux de shader** (humidité, neige au sol, vent, brouillard, couverture nuageuse) appliqués à tous les matériaux, plus des particules autour de la caméra.

| Temps | Image | Son (§5.2) |
|---|---|---|
| **Pluie** | Gouttes en particules, sols mouillés et reflets, flaques dans les creux du terrain, parapluies et capuches sur les habitants (tenue, §3.3) | Pluie sur le sol, les toits, les vitres, gouttières |
| **Vent** | Arbres, drapeaux, linge, feuilles mortes en automne | Rafales, sifflements, objets qui claquent |
| **Brouillard** | Brouillard de hauteur, couleurs lointaines estompées, halos des lampadaires | Sons étouffés, réverbération réduite |
| **Neige** | Flocons, neige qui s'accumule sur les surfaces horizontales puis fond ; traces dans la neige non simulées en 2.5D | Pas étouffés, crissements |
| **Canicule** | Lumière dure, ombres nettes, léger voile de chaleur au loin, volets clos, terrasses | Cigales absentes (climat océanique), ventilateurs, rues calmes l'après-midi |
| **Orage** | Ciel sombre, éclairs **plafonnés en fréquence** (§4.5), pluie forte | Tonnerre retardé selon la distance |
| **Crue** | Eau montée sur les berges et les quais ([ville.md](ville.md), §6.5) : la surface de l'eau est un niveau transmis, pas une simulation | Courant fort |

- Le **brouillard** reste un effet d'image : ce qu'il cache vraiment est décidé par le serveur, qui réduit la perception.
- **Effets mouillés et enneigés** : un seul jeu de paramètres pour tous les matériaux (assombrissement, brillance, voile blanc sur les faces tournées vers le ciel), au lieu de textures supplémentaires. *Pourquoi : un shader commun couvre toute la ville sans multiplier les assets.*

### 2.5 Saisons (décidé)

| Saison | Végétation | Lumière | Habitants (tenue, §3.3) | Décor saisonnier |
|---|---|---|---|---|
| Hiver | Arbres nus (branches modélisées), pelouses ternes, haies persistantes | Basse, courte, froide | Manteaux, bonnets, écharpes | Illuminations de fin d'année dans la Cité et à Hautevigne **[période à calibrer]**, sel sur les trottoirs |
| Printemps | Bourgeons puis feuillage clair, fleurs des massifs | Montante | Vestes légères, imperméables | Terrasses qui rouvrent |
| Été | Feuillage dense, pelouses jaunies en canicule | Haute, longue, chaude | Tenues légères | Terrasses, parasols, fenêtres ouvertes |
| Automne | Feuillage roux puis chute, feuilles au sol | Déclinante, dorée | Manteaux mi-saison | Feuilles balayées par les employés municipaux |

- Un **arbre** est un modèle de branches plus des cartes de feuillage dont la couleur et la densité suivent la date ; on ne change pas d'asset selon la saison. *Pourquoi : un seul asset par espèce, piloté par un paramètre, suffit pour quatre saisons.*
- Les fêtes du calendrier sont **civiles et génériques** (fin d'année, fête de la ville, marché de Noël éventuel décidé par la ville) ; les lieux de culte restent ceux de [ville.md](ville.md), §5.2.

### 2.6 Moteur de rendu (décidé)

- Godot 4, rendu **Forward+** sur ordinateur, sans le rendu de compatibilité. *Pourquoi : Forward+ gère beaucoup de lumières (la ville de nuit) et l'instanciation massive dont l'assemblage du kit a besoin.*
- **Post-traitement** sobre : correction colorimétrique par une table de couleurs par moment de la journée et par météo, exposition automatique douce, aucun grain de film, aberration chromatique ni flou de mouvement par défaut.
- Réglages de qualité ([interface.md](interface.md), §11.3) : ombres, distance des niveaux de détail, densité de particules, occlusion ambiante, résolution de rendu ; le style ne dépend d'aucun d'eux. En 3D seulement, éclairage global dynamique en option.
- **Cibles de performance** **[à calibrer]** : 2.5D à 60 images par seconde en 1080 p sur une carte graphique d'entrée de gamme de 2018 avec 4 Go de mémoire vidéo ; 3D à 60 images par seconde en 1080 p sur une carte de milieu de gamme de 2020 avec 8 Go.

---

## 3. Les habitants à l'écran

### 3.1 Principes (décidé)

- Chaque personne rendue est un **habitant** dont l'apparence est **stable** : la même personne a le même visage, la même silhouette et la même voix chaque jour, et vieillit.
- L'apparence est une **donnée du cœur** (le génome d'apparence, §3.2), produite par le générateur de population, transmise au client sous forme **perçue** (§3.5) et assemblée par le client à partir d'un **corps paramétrique** unique et d'un catalogue de cheveux, de vêtements et d'accessoires.
- Les personnages pilotés par un joueur ont **le même corps, le même catalogue, les mêmes animations et le même naturel** que les autres ([interface.md](interface.md), §4.6).

*Pourquoi : 10 000 personnes réelles ne peuvent pas avoir chacune un modèle fait à la main ; un corps paramétrique et un catalogue assemblé, avec une apparence en données, donnent une variété illimitée pour un coût fixe.*

### 3.2 Le génome d'apparence (décidé)

Composant du cœur d'environ **64 octets** **[à mesurer]**, quantifié, sans aucun nom de ressource :

| Groupe | Champs | Origine |
|---|---|---|
| **Silhouette** | Taille, corpulence, musculature, carrure | Âge, sexe, attributs physiques du personnage (force, endurance) ; jamais le milieu ni l'origine |
| **Visage** | ≈ 24 paramètres de forme quantifiés (crâne, front, yeux, nez, bouche, mâchoire, oreilles), asymétrie légère | Tirage, en partie hérité (§3.4) |
| **Peau** | Carnation, taches de rousseur, rides (dérivées de l'âge) | Origine familiale, hérédité |
| **Cheveux et pilosité** | Couleur naturelle, nature (raides à crépus), calvitie (début et rythme), grisonnement (âge de début), barbe possible | Hérédité, âge |
| **Signes distinctifs** | Lunettes, tatouage (zone, motif généré), cicatrice (zone, taille, visibilité ; venue d'une blessure grave ou critique, [combat.md](combat.md), §2.4), grain de beauté marqué | Biographie, santé, profil, puis événements de la vie ([population.md](population.md), §4.9) |
| **Voix sonore** | Hauteur, timbre, débit (§5.5) | Sexe, âge, corpulence, tirage |
| **Goûts vestimentaires** | Style dominant, couleurs préférées, soin | Âge, milieu, profil |

- **Voix sonore et voix de style** (décidé, à la demande de [population.md](population.md), §4.9) : la **voix sonore** du génome dit comment la personne **sonne** (hauteur, timbre, débit) ; la **voix** de l'état intérieur ([population.md](population.md), §5.6 ; [ia-pnj.md](ia-pnj.md), §13.10) dit comment elle **parle** (registre, politesse, humour, tics, vocabulaire). Seule la voix sonore sert au son (vocalisations, voix indistinctes, voix synthétisées plus tard, §5.5). *Pourquoi : ce sont deux données différentes, tirées de causes différentes, et les confondre ferait sonner pareil tous ceux qui parlent pareil.*
- Les **plages** de chaque champ et leurs **lois de tirage** sont fournies au paquet de contenus par ce domaine (§3.10) ; la population fixe d'où vient chaque champ ([population.md](population.md), §4.9).
- **Le vieillissement** fait évoluer de façon déterministe les rides, le grisonnement, la calvitie et la silhouette ; les enfants grandissent selon leur âge.
- La **coiffure** (coupe, longueur) n'est pas dans le génome mais dans l'état courant du personnage : elle change chez le coiffeur ([ville.md](ville.md), §5.3).
- **Carnation et traits du visage n'entrent dans aucun calcul** du cœur : ni première impression, ni décision, ni description de témoin. *Pourquoi : c'est le principe 5 de la population ([population.md](population.md), §0) et la règle du social ([social.md](social.md), §1.3) ; la description d'un témoin s'en tient au sexe apparent, à l'âge, à la taille, à la corpulence, aux vêtements et aux signes distinctifs ([combat.md](combat.md), §4.3).*

### 3.3 La tenue (décidé)

- Chaque habitant a une **garde-robe abstraite** : quelques tenues par usage (quotidien, travail, sortie, sport, intempéries, nuit), chacune faite de **pièces sémantiques** du catalogue vestimentaire (haut, bas, vêtement de dessus, chaussures, couvre-chef, accessoire, sac) avec une **couleur** prise dans une palette (§2.1) et un **état** (propre, froissé, sale, taché, mouillé, déchiré).
- La **tenue du jour** est choisie par l'IA selon l'activité, le lieu, la météo et l'occasion ([ia-pnj.md](ia-pnj.md)), et par le joueur pour le personnage qu'il pilote, **parmi la même garde-robe**.
- **Uniformes et tenues de travail** (police, pompiers, secours, chantier, hôpital, cuisine, livraison, transports) sont des tenues du catalogue, liées au poste et listées au §3.10 ; le **brassard de police** des policiers en civil est une pièce de la garde-robe de leur poste, mise ou retirée par un geste ([combat.md](combat.md), §5.1) ; les **signes de bande** (couleurs, pièce distinctive, tags sur une veste, [social.md](social.md), §6.6) sont des variantes de pièces.
- **Catalogue de départ** **[à calibrer]** : ≈ 150 pièces sémantiques × 6 à 12 couleurs × états, assez pour qu'aucune rue ne montre deux tenues identiques en dehors des uniformes. Ses champs sont fixés au §3.10.
- **Règles de montage** : trois couches au plus sur le corps ; les parties du corps cachées par un vêtement ne sont pas dessinées ; chaque pièce est ajustée au corps paramétrique par ses propres déformations (corpulence, taille, âge).

*Pourquoi : la tenue porte une grande part de ce que les habitants lisent les uns des autres (richesse, métier, négligence, météo, [social.md](social.md), §5.5) ; elle doit donc être une donnée du cœur, et l'asset seulement sa traduction.*

### 3.4 Variété des 10 000 habitants, sans aucun figurant (décidé)

- **Hérédité limitée** : un enfant reçoit la moyenne bruitée de ses parents pour la carnation, la couleur et la nature des cheveux et la silhouette générale, et seulement une **faible part** de leurs traits de visage. La ressemblance familiale existe, mais un **test automatique** vérifie qu'un classificateur qui ne lit que les apparences perçues ne retrouve pas mieux les liens de parenté qu'un observateur humain (aire sous la courbe ≤ 0,70 **[à calibrer]**). Ce **test de parenté** porte sur les apparences à la précision « visage », la plus riche qu'un client modifié reçoive, et couvre aussi les **naissances** en cours de partie et les **fratries** (v0.4, clôture de la sécurité C2) ; il s'ajoute au test de réidentification (§3.5). *Pourquoi : un client modifié lit les paramètres exacts du visage ; une hérédité trop fidèle lui révélerait une filiation cachée, qui peut être un secret du monde.*
- **Hérédité par les parents officiels seulement** (décidé, [population.md](population.md), §4.9) : une filiation secrète tirée par monde ne touche jamais un génome. *Pourquoi : le génome est le même dans tous les mondes d'une famille ; s'il dépendait d'un secret, il le révélerait.*
- **Pas de sosies** : la validation de la population ([population.md](population.md), §1.3) refuse deux habitants non jumeaux dont les visages sont plus proches qu'un seuil, et vérifie la dispersion des silhouettes, des coiffures et des garde-robes par quartier **[seuils à calibrer]**.
- **Répartitions réalistes** : tailles et corpulences selon l'âge et le sexe, part de lunettes, de barbes, de cheveux gris selon l'âge, carnations selon les origines familiales tirées par la population ([population.md](population.md), §2.4).
- **Variété portée par le cœur** : comme toute personne visible est un habitant, la variété d'une rue est celle de ses habitants. Le client n'ajoute **aucune** variation au hasard (pas de couleur tirée côté client), sinon deux joueurs ne verraient pas la même personne.

### 3.5 L'apparence perçue (décidé)

Le serveur n'envoie que l'apparence **perçue** ([05-architecture.md](../05-architecture.md), §2.5). Elle a trois **précisions**, choisies par le serveur selon la distance, la lumière et ce qui masque :

| Précision | Quand | Contenu envoyé |
|---|---|---|
| **Silhouette** | Loin, de nuit, dans le brouillard, de dos | Taille **par classes de 5 cm** **[à calibrer]**, corpulence et carrure **en crans**, couleurs dominantes de la tenue, posture, allure ; **aucun** signe distinctif ni voix fine |
| **Tenue** | Distance moyenne, bonne lumière | En plus : pièces de la tenue et leur état, coiffure, couleur de cheveux ; **signes distinctifs visibles** réduits à leur **type, zone et taille** (le client dessine un motif générique de cette classe) |
| **Visage** | Près, visage découvert et éclairé | En plus : paramètres du visage, peau, expression ; **graine de forme** des cicatrices et des tatouages visibles |

- **Chaque champ part à la résolution de sa précision** (décidé, v0.4, clôtures de l'architecture C-4 et de la sécurité C2 ; [05-architecture.md](../05-architecture.md), §2.5) : aucune valeur fine n'est envoyée à une précision qui ne la justifie pas. *Pourquoi : une graine de forme ou une taille au centimètre envoyée à distance est un identifiant exact et durable, qui permettrait à un client modifié de suivre une personne malgré le renouvellement des poignées, ce qu'un œil ne peut pas faire à cette distance.*
- **Voix** : tant que l'orateur n'est **ni vu de visage ni identifié** par l'auditeur, le client ne reçoit qu'une **classe de voix grossière** (sexe apparent, tranche d'âge, trois timbres) ; la voix sonore fine (§3.2) n'arrive qu'avec la précision « visage » ou l'identification (§5.5).
- **Test de réidentification** (décidé, v0.4) : un test automatique vérifie qu'un classificateur ne rattache pas une personne **masquée** ou vue **en silhouette** au reste de la population mieux qu'un observateur humain (aire sous la courbe ≤ 0,70 **[à calibrer]**), en plus du test de parenté (§3.4).

- **Ce qui masque retire des champs** : capuche, casque, cagoule, lunettes noires, écharpe montée, déguisement. Un visage couvert n'est **jamais** envoyé, même de près ; une perruque ou une tenue d'emprunt est envoyée telle qu'elle se voit.
- Dans le monde, une précision inférieure à « visage » s'affiche par un **visage non résolu** (décidé, v0.2, remplace le « visage neutre » de la v0.1) : traits aplatis et dans l'ombre (sous la capuche, à contre-jour, dans la nuit), jamais un visage net qui passerait pour le vrai. Le client ne garde aucun visage reçu d'une poignée précédente ([revue de sécurité](../revues/securite.md), S6). *Pourquoi : un visage neutre bien dessiné serait lisible de près, surtout en 3D, et ferait croire à une fausse identité ; ce qu'on ne voit pas doit se montrer comme une absence ([interface.md](interface.md), §4.7).*
- Les **portraits** (sélection de départ, « Vos connaissances », proches du carnet, bilan) sont rendus en jeu à partir de l'apparence **connue**, avec un **cadrage et une lumière de portrait neutres** et fixes : silhouette en contre-jour aux couleurs de la tenue pour la précision « silhouette », buste avec **visage dans l'ombre** pour la précision « tenue », portrait complet pour « visage » ; **jamais de visage neutre en gros plan** ([interface.md](interface.md), §4.7).

*Pourquoi : envoyer le visage exact d'une personne masquée permettrait à un client modifié de la reconnaître à coup sûr ; graduer la précision donne au client exactement ce que l'œil du personnage verrait.*

### 3.6 Animation (décidé)

- **Un seul squelette** pour tous les humains, enfants compris (proportions par déformation), et un **catalogue de gestes sémantiques** qui suit les énumérations d'activité et de geste du cœur ([revue d'architecture](../revues/architecture.md), §13.6) : chaque geste du cœur correspond à une ou plusieurs animations du catalogue.
- **Identifiants stables et familles** (décidé, [ia-pnj.md](ia-pnj.md), §2.6) : chaque valeur des énumérations d'activité, de geste et de posture a un identifiant jamais renuméroté et une **famille** (marcher, manipuler, parler, s'asseoir…). Le catalogue fournit **une animation par famille au moins** ; une valeur sans animation propre joue celle de sa famille. Le contrôle se fait **dans le sens des dépendances** (v0.4, clôture de l'architecture C-1 ; [05-architecture.md](../05-architecture.md), §2.2) : la construction du paquet de contenus vérifie seulement l'identifiant stable et la famille ; `cargo xtask assets check`, côté client, refuse une famille sans animation et lève une **alerte** pour une valeur sans animation propre (§6.9). Le cœur ne lit jamais le catalogue du client. *Pourquoi : un personnage ne doit jamais rester figé, et une activité ajoutée au cœur ne doit pas attendre son animation finale.*
- **Gestes demandés par les domaines** (v0.3, [interface.md](interface.md), §5.2 et §5.3 bis ; [ia-pnj.md](ia-pnj.md), §2.6) : **dévisager** (regard soutenu, tête et buste tournés vers la personne, distinct du **coup d'œil** bref de la désignation, [interface.md](interface.md), §4.6) ; **se changer** (court geste d'habillage, joué derrière une porte ou un paravent quand il y en a, sinon abrégé en fondu de tenue) ; **coupe au salon** (coiffeur debout derrière le client assis, gestes de ciseaux et de tondeuse) ; **allumer une radio**, **changer de station**, **régler le volume** (geste de la main sur l'appareil). Chacun appartient à une famille existante (regarder, s'habiller, travailler les cheveux, manipuler).
- **Couches** : locomotion (marche, course, allure selon l'âge, l'humeur, la hâte et la météo, [ia-pnj.md](ia-pnj.md), §13.7), gestes du haut du corps, **posture d'humeur** en couche additive (épaules basses, buste droit, raideur, [interface.md](interface.md), §4.5), regard et pieds par cinématique inverse, respiration.
- **Procédural d'abord** : regard, pose des pieds, port d'objets, balancement de la tête, pivots de rotation à vitesse humaine ([interface.md](interface.md), §4.6) sont calculés, pas animés à la main.
- **Sources** : bibliothèques de capture de mouvement libres, adaptées au squelette commun par un outil de reciblage automatique (§7.4). Taille du catalogue **[à calibrer]** : ≈ 40 animations au prototype, ≈ 200 à la sortie.
- **Niveaux de détail d'animation** : mise à jour complète près du personnage, cadence réduite au loin, poses figées sous un seuil de taille à l'écran.
- **Violence** : gestes de combat lisibles et brefs, sans ralenti, sans mise à mort ; aucun geste violent n'existe dans le catalogue pour un adulte envers un enfant, puisque le cœur ne le produit jamais ([combat.md](combat.md), §12.2).

### 3.7 Le personnage persistant (décidé)

- L'éditeur d'apparence ([interface.md](interface.md), §3.2) n'offre **que l'espace que la population peut produire** : mêmes plages de génome, même catalogue, mêmes coiffures, mêmes voix. Inversement, le générateur de population donne une **probabilité non nulle** à toute combinaison offerte par l'éditeur.
- **Aucun objet d'apparence propre aux joueurs** : ni tenue exclusive, ni cosmétique payant, ni récompense visible.
- Le persistant habille ensuite son personnage avec sa propre garde-robe, qui s'achète en ville comme celle des habitants.

*Pourquoi : une coiffure ou une tenue que seul un joueur peut avoir désignerait le joueur au premier coup d'œil, ce que le socle interdit ([01-concept.md](../01-concept.md), §1.1).*

### 3.8 Les enfants (décidé)

- Corps enfants par déformation du même corps (âges de 0 à 17 ans), poussettes et porte-bébés pour les tout-petits, tenues propres aux âges (cartables, tenues de sport scolaires).
- Les enfants ne sont **jamais** montrés blessés au-delà de ce que le combat permet (bagarres légères, blessures d'accident soignées) ; aucune image ne les montre victimes de violence d'un adulte, qui n'existe pas dans le monde ([combat.md](combat.md), §12.2).

### 3.9 Animaux (décidé)

Aucun animal visible dans la première version : ni chiens, ni chats, ni pigeons. Les oiseaux n'existent qu'au son, dans le fond sonore (§5.2). *Pourquoi : les animaux ne sont pas simulés ([combat.md](combat.md), §12.1) ; un animal ajouté par le client serait différent pour chaque joueur, et un animal de compagnie demanderait une simulation que personne n'a conçue.*

### 3.10 Données d'apparence fournies au paquet de contenus (décidé, v0.2)

*Pourquoi : la population tire les génomes et les garde-robes, et l'éditeur du persistant propose des choix, à partir des mêmes plages et des mêmes catalogues ([population.md](population.md), §4.9, §4.10 et §12.10) ; ces données disent ce qui peut se voir, elles relèvent donc de ce domaine, et elles vivent dans le paquet de contenus parce que le cœur les lit.*

Fichiers RON dans `data/content/appearance/` (emplacement confirmé et inscrit dans [05-architecture.md](../05-architecture.md), §2.2), rédigés et tenus par le responsable contenus, versionnés avec le paquet. Ils ne contiennent **aucun nom d'asset** : la correspondance avec les fichiers reste dans `client/catalog/` (§6.7).

**a) Plages et lois du génome** **[toutes les valeurs à calibrer sur des statistiques publiques d'Europe de l'Ouest]** :

| Champ | Plage | Loi de départ |
|---|---|---|
| Taille adulte | 145 à 205 cm | Normale par sexe (≈ 164 cm, écart 6,5 pour les femmes ; ≈ 177 cm, écart 7 pour les hommes), en partie héritée ; baisse légère après 70 ans |
| Corpulence | 7 crans, de très mince à forte | Selon l'âge et le sexe, déplacée par les attributs physiques et les habitudes ([population.md](population.md), §4.9) ; jamais par le milieu ni l'origine |
| Musculature, carrure | 5 crans | D'après la force et l'endurance |
| Visage | ≈ 24 paramètres quantifiés sur 16 niveaux | Normale centrée par paramètre, asymétrie de 0 à 2 niveaux |
| Carnation | 12 niveaux | D'après l'origine familiale pour les fondateurs, puis hérédité |
| Cheveux | 10 couleurs naturelles × 4 natures | D'après l'origine familiale, puis hérédité |
| Lunettes visibles | Oui ou non | ≈ 15 % avant 20 ans, ≈ 35 % de 20 à 44 ans, ≈ 60 % de 45 à 64 ans, ≈ 80 % après 65 ans, d'après les troubles de la vue |
| Barbe ou moustache | 6 formes | ≈ 30 % des hommes adultes |
| Grisonnement | Âge de début | Médiane ≈ 38 ans, écart ≈ 7 ans |
| Calvitie | Âge de début, rythme | ≈ 25 % des hommes à 30 ans, ≈ 50 % à 50 ans ; rare et diffuse chez les femmes |
| Tatouage visible | Zone, motif | ≈ 20 % des adultes, davantage de 25 à 40 ans, d'après le profil et la biographie |
| Voix sonore | Hauteur, timbre, débit en 8 niveaux chacun | Selon le sexe, l'âge et la corpulence |

- Chaque loi garde un **plancher de probabilité** sur toute sa plage, et aucune combinaison n'est exclue par une dépendance entre champs ([population.md](population.md), §4.9).

**b) Catalogue vestimentaire** : pour chaque pièce sémantique,

| Champ | Contenu |
|---|---|
| `id`, `family` | Identifiant stable et famille (haut, bas, dessus, chaussures, couvre-chef, accessoire, sac) |
| `layer` | Couche portée (1 à 3, §3.3) et zones du corps couvertes (pour le masquage et pour l'apparence perçue) |
| `ages` | Tranches d'âge permises |
| `incomes` | Milieux où la pièce est courante (modeste, moyen, aisé), avec une fréquence ; jamais exclusive |
| `range` (v0.3) | **Gamme** (entrée, moyenne, haut de gamme), qui porte le prix pour l'économie ([economie.md](economie.md), §5.13) ; elle ne se voit **que** par la finition (coupe, matière, coutures, accessoires) et l'état, jamais par une marque ni un logo (§1.4) |
| `sex_frequency` | Fréquence par sexe, **jamais nulle** pour l'un ou l'autre, pour que l'éditeur reste dans l'espace de la population |
| `uses` | Usages (quotidien, travail, sortie, sport, intempéries, nuit) |
| `warmth`, `rain` | Isolation thermique et protection contre la pluie, lues par le confort du cœur ([ia-pnj.md](ia-pnj.md), §2.6) |
| `colors` | Couleurs permises, prises dans les palettes (§2.1) |
| `states` | États possibles (propre, froissé, sale, taché, mouillé, déchiré) |
| `masks` | Champs de l'apparence perçue qu'elle masque (capuche, casque, cagoule, lunettes noires, écharpe montée, §3.5) |
| `group_marks` | Variantes possibles de signe de bande (§3.3) |

**c) Uniformes et tenues de poste** **[liste à compléter avec les postes de l'économie]** :

| Poste | Tenue |
|---|---|
| Police nationale | Tenue de service, tenue d'intervention, **brassard** pour les policiers en civil ([combat.md](combat.md), §5.1) |
| Police municipale | Tenue de service, gilet en été |
| Pompiers | Tenue d'intervention, tenue de caserne |
| Secours | Tenue de secouriste haute visibilité |
| Hôpital | Blouses et tuniques par fonction (soignant, médecin, agent) |
| Chantier, voirie, employés municipaux | Gilet haute visibilité, casque, chaussures de sécurité |
| Cuisine, restauration | Veste de cuisine, tablier |
| Livraison, transports | Blouson aux couleurs de l'entreprise fictive, tenue de conducteur de bus |
| Sécurité privée | Tenue sombre à écusson fictif |
| Justice | Robes d'audience des magistrats et des avocats, portées au tribunal seulement |

- Tous les écussons et inscriptions sont **fictifs** (§1.4).

**d) Motifs de cicatrices et de tatouages** :

- **Cicatrices** ([combat.md](combat.md), §2.4) : motifs par **type** (coupure, plaie pénétrante, plaie par balle, brûlure), par **zone** (les huit zones du corps, plus le visage en détail) et par **taille** (petite, moyenne, grande) ; leur **visibilité** suit la zone (visage, cou et mains toujours visibles, ailleurs selon la tenue, par le champ `layer` des pièces). Le génome ne garde que le type, la zone, la taille et une graine de forme ; le client dessine le motif par décalcomanie. La **graine de forme** n'est envoyée qu'à la précision « visage » ; à la précision « tenue », le client dessine un **motif générique** de la classe (type, zone, taille) (§3.5, v0.4). Même règle pour les tatouages.
- **Tatouages** : motifs générés par graine dans quelques styles (lettrage, ornemental, figuratif simple, signe de bande), sans symbole haineux ni emblème réel **[liste d'exclusion à constituer]**.

---

## 4. Lisibilité dans le monde

### 4.1 Hiérarchie visuelle (décidé)

Ordre de lecture voulu : **les personnes**, puis **ce qu'on peut utiliser** (portes, véhicules, objets), puis le **décor**. Il est obtenu par la valeur et la couleur, jamais par un marqueur permanent :

- décor peu saturé (§2.1), personnes vêtues de couleurs plus franches ;
- silhouettes des personnages légèrement plus contrastées que leur fond par une **lumière de contre-jour** douce, appliquée à **tous** les personnages ;
- objets d'usage courant (portes, poignées, boutons d'ascenseur, distributeurs) d'une valeur qui tranche avec leur mur.

*Pourquoi : l'interface interdit les étiquettes flottantes ([interface.md](interface.md), §4.5) ; c'est donc l'image elle-même qui doit guider l'œil.*

### 4.2 Cible et désignation (décidé)

- La cible désignée a un **contour** de couleur neutre, identique pour une personne, un objet ou une porte, et identique quel que soit le pilote ([interface.md](interface.md), §13.2).
- Aucun objet ne brille ou ne clignote pour signaler qu'il est utilisable ; la liste des **actions possibles** apparaît quand le joueur désigne quelque chose ([interface.md](interface.md), §5.2).

*Pourquoi : un objet qui brille révélerait au joueur ce que le personnage ne sait pas, ou ferait de la ville un jeu de recherche d'objets.*

### 4.3 Deux couches, deux styles (décidé)

| Couche | Style visuel |
|---|---|
| **Monde** (voix intérieure, téléphone, bulles, icônes de son) | Typographie humaniste, coins arrondis, couleurs prises dans la palette du monde, fond semi-transparent ; le téléphone a le style d'un vrai téléphone, sans marque |
| **Joueur** (synchronisation, expérience, familiarité, modération) | Typographie de caractère neutre, cadre fin au **liseré distinctif**, couleur jamais utilisée dans le monde |

*Pourquoi : c'est la traduction graphique du principe des deux couches ([interface.md](interface.md), §0.2 et §2.7).*

- **Teinte du liseré contrôlée** (décidé, v0.2, [interface.md](interface.md), §2.7) : le contrôle automatique (§6.9) vérifie que la teinte du liseré reste à une distance de couleur minimale **[à calibrer]** de toutes les couleurs des palettes du monde (§2.1), y compris dans chacune de leurs transpositions pour daltoniens (§4.5). *Pourquoi : une palette du monde enrichie plus tard ne doit pas finir par contenir, sans que personne le voie, la couleur réservée au jeu.*

### 4.4 Bulles, icônes de son, indicateurs (décidé)

- **Bulles de dialogue** : fond clair, texte sombre, queue vers l'orateur, taille fixe à l'écran quel que soit le zoom, identiques quel que soit le pilote. C'est l'interface qui dessine la bulle ([interface.md](interface.md), §2.8), et ce domaine en suit la règle (décidé, v0.3.1, **arbitrage A10**, remplace les formes par ton des v0.2 et v0.3) :
  - le **volume** (énumération fermée du cœur, [social.md](social.md), §3.2) donne le **trait** du bord, et la forme suit **[à calibrer]** :

| Volume | Bord de la bulle |
|---|---|
| Chuchoté | Pointillés |
| Normal | Bord simple |
| Élevé | Bord double |
| Crié | Bord épais et dentelé |

  - le **ton** (5 valeurs, [ia-pnj.md](ia-pnj.md), §13.10) passe par une **icône et un mot** dans la bulle, jamais par la forme seule ni par la couleur ;
  - une conversation **entendue sans être comprise** suit la même règle : bulle **sans texte**, au bord de son volume, avec l'icône et le mot de son ton, pendant la durée fournie par le cœur (§5.5).

  *Pourquoi : une icône et un mot restent lisibles en petit et pour les daltoniens, sans reposer sur la seule forme (§4.5) ; et une seule source, l'interface, dessine la bulle.*
- Ce domaine fournit les **icônes de ton** dans le même style monochrome que les icônes de son ; les mots de ton sont des textes traduits de l'interface.

- **Icônes de son** : pictogrammes monochromes (voix, pas, verre, coup de feu, moteur, porte, **sirène**, **klaxon**), dont le **flou** suit l'incertitude de position envoyée par le serveur ([interface.md](interface.md), §6.4).
- **Clés de sous-titre** (décidé, v0.2, [interface.md](interface.md), §11.1) : chaque son du catalogue qui a une icône, et chaque son visible qui compte (sirène, alarme, verre brisé, coup de feu, cri, klaxon, moteur qui démarre), porte une **clé de sous-titre** traduite (« sirène au loin », « verre brisé »). Le contrôle automatique refuse un tel son sans clé (§6.9).
- **Indicateurs de danger** en bord d'écran : forme fléchée et pulsation lente, sans couleur seule.

### 4.5 Accessibilité visuelle (décidé)

- **Aucune information portée par la couleur seule**, dans l'interface comme dans les signaux du monde qui comptent (gyrophares, feux tricolores, sang, enseignes d'urgence) : chacun a aussi une forme, un mouvement ou un son.
- **Palettes pour daltoniens** appliquées à l'interface et aux signaux du monde, par une table de couleurs dédiée ; le décor n'en a pas besoin, ses couleurs ne portant aucune information.
- **Protection contre les flashs** : éclairs, gyrophares, éclairs de tir, stroboscopes de la boîte de nuit, néons défectueux sont plafonnés à **3 éclats par seconde** et à une variation de luminance limitée ; l'option d'accessibilité les réduit encore à une lueur continue ([interface.md](interface.md), §11.1). *Pourquoi : c'est le seuil reconnu de risque pour les personnes photosensibles.*
- **Atténuer la violence** ([combat.md](combat.md), §12.3) : chaque asset concerné a sa **variante atténuée** prévue dans le catalogue (décalcomanies de sang retirées, vêtements tachés remplacés par des vêtements froissés, corps au sol remplacé par un fondu vers le drap des secours, sons adoucis, §5.6).
- **Contraste élevé** : l'option assombrit le décor et éclaircit les contours des personnes et des portes, sans changer la scène.

### 4.6 Typographie (décidé)

Polices sous licence **SIL Open Font License** uniquement **[à vérifier au registre]**. Les polices de l'interface sont **retenues** par [interface.md](interface.md), §2.8 (v0.6) :

| Usage | Polices |
|---|---|
| Interface, couche monde | **Inter** (Source Sans 3 en remplacement) |
| Couche joueur | **IBM Plex Sans** |
| Lisibilité renforcée | **Atkinson Hyperlegible** ; **OpenDyslexic** pour l'option dyslexie |
| Signalétique routière et plaques | Barlow, avec chiffres tabulaires |
| Enseignes, affiches, journal | Une dizaine de familles OFL variées (sérif, grotesque, script, pochoir) choisies par le générateur de logos et d'enseignes (§8.3) |

*Pourquoi : la licence OFL autorise l'inclusion dans un jeu vendu et la modification, sans obligation sur le reste du jeu.*

---

## 5. Son, musique et voix

### 5.1 Principes (décidé)

- **Le client ne joue que ce que la vue contient.** Chaque son individualisé (voix, pas, moteur, porte, coup) vient d'une entité ou d'un événement **perçu** ; sa position est celle que le serveur envoie, **déjà floutée** pour ce qui est entendu sans être vu ([05-architecture.md](../05-architecture.md), §2.5 ; [interface.md](interface.md), §6.4).
- **Aucun figurant sonore** (§0.4).
- **Aucune musique ni ambiance ne réagit à ce que le personnage ne perçoit pas** : pas de musique de combat, pas de montée de tension quand un danger approche. *Pourquoi : une musique qui change avant que le danger soit perçu donnerait au joueur une information que son personnage n'a pas.*
- Le **fond sonore** ne dépend que de données publiques : heure, quartier, type de lieu, météo, saison.

### 5.2 Couches du paysage sonore (décidé)

| Couche | Contenu | Source de la donnée |
|---|---|---|
| **Fond** | Vent, pluie, oiseaux selon la saison et l'heure, rumeur lointaine de la ville dont l'intensité suit l'heure et le quartier, eau du fleuve | Publique (heure, météo, carte) |
| **Lieu** | Ton de pièce selon le type de lieu (logement, bar, atelier, hôpital), bourdonnement des machines, ventilation, réfrigérateurs | Carte publique, état visible des lieux (ouvert, éteint) |
| **Sources perçues** | Voix, pas selon le sol et les chaussures, véhicules, portes, objets, radios et télévisions des logements perçus | Vue filtrée |
| **Corps du personnage** | Respiration, essoufflement, effets physiologiques ([interface.md](interface.md), §5.6.4) | État du personnage piloté |
| **Interface** | Sons courts et discrets, distincts selon la couche (monde ou joueur) | Client |

### 5.3 Acoustique (décidé)

- L'**atténuation** des murs, des planchers, des portes et des fenêtres ([ville.md](ville.md), §5.6) est appliquée par le serveur pour décider de ce qui est perçu ; le client applique ensuite un **filtrage cohérent** (son étouffé à travers un mur, plus clair par une porte ouverte), sans jamais rendre audible ce que le serveur n'a pas envoyé.
- **Réverbération par type de lieu** (pièce d'habitation, cage d'escalier, hall de gare, entrepôt, église, rue étroite, dalle ouverte), choisie dans un petit jeu de préréglages.

### 5.4 Musique (décidé)

- **Musique d'abord diégétique** : radios des logements et des voitures, sonos des bars, de la boîte de nuit et des fêtes, orgue de l'église, musiciens qui sont des habitants. *Pourquoi : dans une ville sans figurant vécue en temps réel, la musique appartient aux gens et aux lieux.*
- **Radios fictives** : trois stations au départ **[à calibrer]** (une radio locale de variétés et d'informations, une station de musiques actuelles, une station classique et jazz), avec un **programme déterministe** calculé à partir de l'heure du monde et d'une graine publique. Deux joueurs dans le même bar entendent le même morceau au même moment. Les informations de la radio locale sont d'abord des jingles et de la musique ; des bulletins parlés ne viendront qu'avec les voix synthétisées (§5.5).
- **Sources musicales du cœur** (décidé, v0.2, accord avec [ia-pnj.md](ia-pnj.md), §2.6) : le cœur tient, pour un lieu ou un objet, une **source musicale** et son **volume** ; il ne choisit jamais les morceaux. Le client les choisit de façon **déterministe**, pour que deux joueurs au même endroit entendent la même chose :

| Source tenue par le cœur | Ce que le client en tire |
|---|---|
| **Radio** allumée sur une station | Le programme de la station, calculé à partir de l'heure du monde et d'une graine publique |
| **Sono** d'un bar, d'une boutique, d'une fête (style choisi par la tâche « mettre de la musique ») | Une liste de morceaux du style, tirée à partir de l'identifiant stable du lieu (carte publique) et de l'heure du monde |
| **Musicien** (instrument, style, qualité tirée de sa compétence) | Un morceau du répertoire de l'instrument et du style, tiré de la même façon ; la **qualité** choisit l'interprétation (hésitante, correcte, assurée), rendue en trois niveaux par nos outils à partir de la même partition (§7.4, v0.2.1) **[à calibrer]** |

  Le client n'utilise jamais une poignée pour tirer un morceau, puisqu'elle change à chaque perception ([revue de sécurité](../revues/securite.md), S6). La source n'est envoyée que si elle est **perçue**, à son volume perçu, à travers les murs comme les autres sons (§5.3).
- **Musique d'accompagnement** extra-diégétique **minimale** : menus, écran « Qui jouer ? », passage de relais, bilan d'une incarnation ; en jeu, une option (désactivée par défaut) joue de rares pièces calmes selon l'heure et la météo, jamais selon la situation.
- **Passage de relais et bilan** (décidé, v0.2, [interface.md](interface.md), §11.3) : une **pièce unique** pour tout passage de relais, quel que soit le personnage pris ou rendu ; des **pièces de bilan** choisies seulement d'après ce que l'écran de bilan affiche lui-même (tendance de la synchronisation, issue de l'incarnation), jamais d'après une donnée que l'écran tait. *Pourquoi : une musique propre à un personnage, ou réglée sur une valeur cachée, en dirait plus que l'écran.*
- **Ton musical** : mélancolie urbaine sobre, instrumentation acoustique et électronique légère (piano, cordes, guitare, nappes), sans thème héroïque.

### 5.5 Voix (décidé)

**Première version : aucune parole enregistrée ni synthétisée.**

- Les répliques sont du **texte** dans des bulles et des sous-titres, formulé par le service de dialogue ou le repli systémique ([social.md](social.md), §3.4 et §3.6).
- Chaque réplique s'accompagne de **vocalisations non verbales** courtes adaptées au ton et à la voix du personnage (rire, soupir, « hm », exclamation, toux, cri de douleur), tirées d'une banque classée par âge, sexe apparent et timbre, d'après la **voix sonore** du génome (§3.2), ou seulement d'après la **classe de voix grossière** tant que l'orateur n'est ni vu de visage ni identifié (§3.5, v0.4). Le choix de l'échantillon ne fait **aucun tirage côté client** : le cœur tire la vocalisation sur le flux **`cosmetic/vocalization`**, un seul domaine pour la catégorie, l'intensité et l'**indice de variante** qu'il envoie, pour que deux auditeurs entendent le même son (clôture de l'architecture, S-4).
- Une conversation **entendue sans être comprise** (trop loin, à travers un mur, bribes découpées par le serveur) se rend par des **voix indistinctes** : murmure synthétique sans mots, filtré, au rythme et au ton de la conversation. Par acte entendu sans être compris, le cœur fournit le **ton** (5 valeurs), le **volume** (chuchoté, normal, élevé, crié) et la **durée** ([social.md](social.md), §3.2, v0.3) ; le client en tire la longueur du murmure, son intensité, son débit et le choix des vocalisations, d'après la voix sonore de l'orateur perçu. Il n'invente ni syllabe lisible ni ton absent.

*Pourquoi : les répliques sont formulées à la volée, dans la langue de chaque auditeur et pour 10 000 personnes ; aucun doublage enregistré n'est possible, et une voix synthétisée de qualité moyenne nuirait plus au naturel ([ia-pnj.md](ia-pnj.md), §13.7) qu'un texte bien écrit.*

**Plus tard : voix synthétisées en option** (phase 5 ou après, quand la qualité le permet ; **[faisabilité à vérifier]**). Conditions, toutes bloquantes :

1. **Synthèse dans le client de chaque auditeur**, dans sa langue, à partir du texte qu'il reçoit déjà : le cœur et le service de dialogue n'en savent rien. *Pourquoi : la formulation est déjà propre à chaque auditeur, et la voix reste cosmétique.*
2. **Toutes les paroles sont dites**, celles des PNJ comme celles des joueurs (texte libre traduit compris, [social.md](social.md), §9.2), avec la **voix du personnage** qui parle et le **même délai**. *Pourquoi : si seuls les PNJ parlaient à voix haute, un personnage muet désignerait un joueur.*
3. **Voix sonore tirée du génome** (§3.2) : un petit nombre de modèles de voix, modulés en hauteur, timbre et débit, pour qu'une personne garde sa voix.
4. **Modèles de synthèse** sous licence permettant l'usage commercial, entraînés sur des voix dont le consentement est documenté ; **aucun clonage de voix** d'une personne réelle (§7.5).
5. Option **désactivable**, sous-titres toujours disponibles.

La **lecture à voix haute** des menus et des intuitions pour l'accessibilité ([interface.md](interface.md), §11.1) reste un service distinct, qui peut utiliser la synthèse du système.

### 5.6 Sobriété des sons de violence (décidé)

- Coups, chutes et tirs ont un son **sec, court et réaliste**, sans emphase cinématographique, suivi d'un **silence relatif** et des réactions des témoins.
- Chaque son de violence a une **variante atténuée** (amortie, plus lointaine) pour l'option « atténuer la violence » ; aucun son de blessure détaillé (craquement d'os, chairs) n'existe dans le catalogue.
- Les sons des violences entre proches ne passent que par leurs **signes** : cris étouffés derrière une porte, objet qui tombe ([combat.md](combat.md), §12.3).

### 5.7 Mixage et accessibilité (décidé)

- Quatre canaux réglables : **voix et vocalisations**, **ambiance et effets**, **musique**, **interface** ([interface.md](interface.md), §11.3).
- **Niveaux cibles** **[à calibrer]** : musique −18 LUFS intégrés, ambiances −24 LUFS, vocalisations −20 LUFS, crête vraie ≤ −1 dBTP pour tout fichier.
- Option **plage dynamique réduite** (écoute de nuit) et **son mono**.
- **Sous-titres des sons** importants (« verre brisé », « coup de feu au loin »), en plus des icônes de son.

---

## 6. Cahier des charges des assets

### 6.1 Conventions communes (décidé)

| Convention | Valeur |
|---|---|
| Unités | 1 unité = 1 m ; axe vertical Y ; avant du modèle vers −Z (convention de Godot) |
| Grille du kit | 1 m ; travées de façade de 3 m ; étage 3 m ; rez-de-chaussée commercial 4 m ([ville.md](ville.md), §4.4) |
| Origine | Éléments de kit : coin inférieur gauche de la travée, côté rue ; meubles et objets : centre de l'emprise au sol ; personnages : entre les pieds |
| Ouvertures | Portes de 1 m de large au moins, dégagements de 1,4 m ([ville.md](ville.md), §2.2) |
| Matériaux | Rendu physique (albédo, normale, occlusion-rugosité-métal regroupées), matériaux partagés (§6.5) |
| Valeurs acoustiques | Chaque élément de mur, de plancher, de porte et de fenêtre déclare son atténuation dans le catalogue sémantique, pas dans l'asset ([ville.md](ville.md), §4.4) |
| Métadonnées | Aucune donnée personnelle, aucun chemin de fichier local, aucune métadonnée EXIF ou XMP dans un asset livré |

### 6.2 Niveaux de détail (décidé)

**Ville** (application de [interface.md](interface.md), §6.5 et de la [revue de la génération](../revues/generation.md), P33) :

| Niveau | 2.5D | 3D |
|---|---|---|
| **Complet** | Éléments du kit d'extérieur et d'intérieur, mobilier fixe et perçu | Éléments 3D détaillés du kit, mêmes dimensions |
| **Façade** | Façades et toits du kit, menuiseries simplifiées, aucun intérieur ; fenêtres par matériau émissif | Éléments 2.5D réutilisés comme niveau intermédiaire |
| **Volume** | Volume simple par bâtiment, texturé par typologie et matériau dominant, toit à sa forme | **HLOD par secteur** : volumes fusionnés précalculés par le client à partir des mêmes données, puis imposteurs au lointain |

**Objets et personnages** :

| Catégorie | Niveaux 2.5D | Niveaux 3D |
|---|---|---|
| Personnage | LOD0, LOD1, LOD2 | LOD0 à LOD3 |
| Véhicule | LOD0, LOD1 | LOD0 à LOD2, imposteur |
| Mobilier, objet | LOD0 seul (petits objets) ou LOD0, LOD1 | LOD0 à LOD2 |
| Arbre | LOD0, LOD1 | LOD0 à LOD2, imposteur |

- Les niveaux inférieurs sont produits **automatiquement** par simplification de maillage à l'import, puis contrôlés (§6.9) ; seuls les personnages et les éléments du kit peuvent avoir des niveaux écrits par script.
- Changement de niveau par **fondu bref** ([interface.md](interface.md), §6.5).

### 6.3 Budgets de polygones **[à calibrer]**

En triangles, pour le niveau le plus détaillé :

| Élément | 2.5D | 3D |
|---|---|---|
| Travée de façade (3 × 3 m) | 50 à 200 | 400 à 2 000 |
| Élément d'intérieur (pan de mur, sol, escalier) | 12 à 80 | 50 à 600 |
| Meuble | 50 à 500 | 500 à 4 000 |
| Petit objet (vaisselle, outil, téléphone) | 20 à 150 | 100 à 1 500 |
| Personnage complet (corps, tête, cheveux, tenue) | 4 000 à 6 000 | 25 000 à 40 000 |
| Coiffure | 300 à 800 | 3 000 à 15 000 (cartes) |
| Pièce de vêtement | 300 à 1 500 | 2 000 à 8 000 |
| Véhicule | 1 500 à 3 000 | 15 000 à 40 000 |
| Arbre | 1 000 à 3 000 | 5 000 à 20 000 |
| Lieu fait main entier (avec ses intérieurs) | ≤ 150 000 | ≤ 1 500 000 |

**Budgets d'écran** : en 2.5D, ≤ 1,5 million de triangles et ≤ 1 000 appels de dessin pour une vue de quartier avec environ 150 personnes perçues ; en 3D, ≤ 6 millions de triangles et ≤ 3 000 appels de dessin.

**Squelette** : ≈ 60 os en 2.5D (doigts regroupés par trois), ≈ 90 os en 3D avec doigts ; visage par **déformations** (≈ 50 expressions de base en 3D, une dizaine en 2.5D).

### 6.4 Textures (décidé)

| Règle | 2.5D | 3D |
|---|---|---|
| Densité de texels du décor | **128 px/m** | **512 px/m** |
| Feuilles de garnitures du kit (corniches, encadrements, menuiseries) | 2 048² | 4 096² |
| Matériau répétable | 512² à 1 024² | 2 048² |
| Personnage (peau, corps) | 1 024² | 2 048² à 4 096² |
| Vêtement | 512² | 2 048² |
| Décalcomanies (tags, affiches, taches, fissures) | Atlas de 2 048² | Atlas de 4 096² |

- **Format source** : PNG 8 bits par canal (16 bits pour les cartes de hauteur), sans métadonnées. Godot les importe en **compression vidéo** (BPTC ou S3TC sur ordinateur) avec mipmaps.
- **Canaux regroupés** : occlusion, rugosité et métal dans une seule texture.
- **Matériaux répétables et feuilles de garnitures** plutôt que textures uniques par bâtiment : une façade est texturée par ses matériaux et par les paramètres d'instance (couleur, usure, salissure), jamais par une texture peinte pour elle.
- **Mémoire vidéo** **[à calibrer]** : ≤ 2,5 Go en 2.5D, ≤ 6 Go en 3D en qualité maximale.

### 6.5 Matériaux et shaders (décidé)

- Une **bibliothèque d'environ 60 matériaux** partagés (pierre, enduits, briques, bétons, bois, métaux, verres, tissus, sols, végétaux) **[à calibrer]**, chacun paramétré : teinte (prise dans les palettes), usure, salissure, humidité, neige.
- **Paramètres par instance** pour l'entretien et les dégâts ([ville.md](ville.md), §6.3) : un immeuble mal entretenu a la même géométrie qu'un immeuble soigné, avec plus de salissure, des volets dépareillés et des décalcomanies (fissures, coulures, tags).
- **Un petit jeu de shaders** écrits pour le projet : matériau de bâtiment, peau, cheveux, tissu, végétation saisonnière, eau, verre, émissif de fenêtre et d'enseigne, décalcomanie. Aucun shader importé d'une source tierce sans entrée au registre (§9.1).

### 6.6 Formats (décidé)

| Type | Format livré | Remarque |
|---|---|---|
| Modèles, squelettes, animations | **glTF 2.0 binaire** (`.glb`) | Format ouvert lu nativement par Godot ; un fichier par élément ou par famille d'animations |
| Textures | PNG | Compression vidéo faite par l'import de Godot |
| Musique, ambiances longues | **Ogg Vorbis**, 48 kHz, stéréo | Lues en flux |
| Effets courts, vocalisations | **WAV** 48 kHz, 16 bits, mono (stéréo pour les ambiances ponctuelles) | Compressés à l'import par Godot |
| Polices | OTF ou TTF | OFL (§4.6) |
| Catalogue, palettes, correspondances, registre | **RON** | Données texte suivies dans Git |
| Matériaux et scènes Godot | `.tres`, `.tscn` texte | Générés par script quand c'est possible |

Formats **refusés** dans `client/assets/` : fichiers de travail (`.blend`, `.psd`, `.kra`, `.spp`), FBX, formats propriétaires de bibliothèques, MP3. *Pourquoi : ils sont lourds, non diffables ou liés à un logiciel ; les sources restent hors de Git (§9.3).*

### 6.7 Nommage et catalogue de correspondances (décidé)

- **Identifiants sémantiques** (dans le paquet de contenus, définis par les domaines) : anglais, `snake_case`, sans référence à un fichier. Exemples : `garment.coat_trench`, `kit.facade.old_stone.window_french`, `gesture.phone_check`, `sound_event.glass_break`.
- **Fichiers d'assets** : anglais, `snake_case`, préfixe de famille, nom, variante, niveau de détail : `<famille>_<nom>[_<variante>][_lod<n>].<ext>`.

| Préfixe | Famille | Exemple |
|---|---|---|
| `kit_` | Éléments du kit de ville | `kit_ext_old_stone_bay_window_a_lod0.glb` |
| `furn_` | Mobilier | `furn_kitchen_table_wood_b.glb` |
| `prop_` | Petits objets | `prop_phone_generic_a.glb` |
| `chr_` | Corps, têtes, cheveux | `chr_hair_short_curly_a.glb` |
| `cloth_` | Vêtements | `cloth_coat_trench_lod1.glb` |
| `veh_` | Véhicules | `veh_city_car_a.glb` |
| `veg_` | Végétation | `veg_tree_linden_mature.glb` |
| `mat_`, `tex_` | Matériaux, textures (suffixes `_albedo`, `_normal`, `_orm`, `_emission`, `_height`) | `tex_brick_red_worn_albedo.png` |
| `decal_` | Décalcomanies | `decal_crack_wall_a.png` |
| `anim_` | Animations | `anim_walk_tired.glb` |
| `sfx_`, `amb_`, `mus_`, `vox_`, `ui_` | Effets, ambiances, musiques, vocalisations, interface | `sfx_door_wood_close_03.wav`, `amb_district_cite_night_rain.ogg` |
| `font_` | Polices | `font_inter_regular.otf` |

- **Catalogue de correspondances** (`client/catalog/*.ron`) : pour chaque identifiant sémantique, l'asset de chaque niveau de détail, en 2.5D et en 3D, et sa **variante atténuée** s'il en a une (§4.5). Le contrôle automatique vérifie que **chaque identifiant du paquet de contenus a une correspondance**, et qu'aucune correspondance ne pointe vers un fichier absent.
- **Repli** : un identifiant sans asset prend l'asset générique de sa famille (meuble générique, vêtement générique, animation de la famille de gestes, §3.6) et lève une alerte dans les constructions de développement ; il ne bloque jamais le jeu. Le repli ne s'applique **pas** à la variante atténuée d'un asset de violence ni à la clé de sous-titre d'un son à icône, qui sont obligatoires (§6.9).

### 6.8 Arborescence (décidé)

```
client/
├─ assets/                     assets prêts pour le jeu (Git LFS, non extraits par défaut)
│  ├─ characters/              body/  heads/  hair/  clothing/  accessories/
│  ├─ animations/              locomotion/  gestures/  combat/  facial/
│  ├─ city/                    kit_exterior/<style>/  kit_interior/<type>/  street/  landmarks/
│  ├─ furniture/  props/  vehicles/  vegetation/
│  ├─ materials/               textures/  decals/
│  ├─ audio/                   sfx/  amb/  music/  vox/  ui/
│  └─ fonts/
├─ assets_gen/                 assets générés par les outils (ignoré, lien vers TS_CACHE)
├─ catalog/                    correspondances, palettes, préréglages acoustiques (RON, Git ordinaire)
├─ registry/                   registre des licences (RON, Git ordinaire), crédits générés
└─ scenes/style_reference/     scène étalon (§10.2)

tools/assets/                  scripts des générateurs et des contrôles (Python avec uv, Rust)
```

- Les **landmarks** (lieux faits main) sont des scènes Godot assemblées avec le kit ([ville.md](ville.md), §3.4) : leurs fichiers `.tscn` sont du texte suivi dans Git ordinaire ; seuls leurs éléments propres passent par LFS.

### 6.9 Contrôles automatiques (décidé)

Une commande `cargo xtask assets check` **[nom à confirmer par le développement]**, lancée avant chaque PR qui touche `client/assets/`, `client/catalog/` ou `client/registry/`, et en CI :

1. **Registre** : chaque fichier de `client/assets/` est couvert par une entrée du registre, avec une licence acceptée (§7.3) ; l'empreinte SHA-256 correspond.
2. **Nommage et place** dans l'arborescence.
3. **Format** : glTF valide, échelle et axes, origine, aucun FBX ni fichier de travail.
4. **Budgets** : triangles, os, taille des textures, densité de texels mesurée, durée et niveau sonore (LUFS, crête).
5. **Palette** : albédo moyen et saturation dans les plages de la famille (§2.1).
6. **Métadonnées** : aucune donnée EXIF ou XMP, aucun chemin local, aucun nom de personne dans les champs du glTF ou des fichiers audio.
7. **Catalogue** : couverture complète des identifiants sémantiques, aucune correspondance orpheline ; une animation au moins pour chaque **famille** de gestes et d'activités, refus sinon, et une alerte pour chaque valeur sans animation propre (§3.6).
7 bis. **Obligations sans repli** (v0.2) : tout asset de violence **sans variante atténuée** est refusé ([combat.md](combat.md), §12.3) ; tout son à icône ou son visible qui compte **sans clé de sous-titre** est refusé (§4.4).
7 ter. **Liseré de la couche joueur** (v0.2) : distance de couleur minimale avec toutes les palettes du monde et leurs transpositions pour daltoniens (§4.3).
8. **Texte figé** : détection de texte lisible dans les textures du décor (reconnaissance de caractères), refusée hors liste d'exceptions (§1.5).

*Pourquoi : sans artiste pour relire chaque pièce, ces contrôles sont la direction artistique appliquée ; un asset qui les passe peut entrer sans jugement humain.*

---

## 7. Sources et critères de choix

### 7.1 Ordre de préférence (décidé)

1. **Généré par nos propres outils** (§8), à partir de scripts suivis dans Git : aucune question de licence, régénérable, cohérent par construction.
2. **Domaine public ou CC0** : aucune obligation.
3. **Licences permissives avec attribution** (CC BY 4.0 et 3.0, OFL pour les polices) : attribution générée par le registre (§9.1).
4. **Licences gratuites propres à une source** (libres de droits, sans paiement), autorisant l'inclusion dans un jeu distribué : seulement quand les trois premiers niveaux ne suffisent pas.
5. **IA générative**, sous les conditions du §7.5.

Aucune source **payante** n'est admise (§7.6).

*Pourquoi : plus une source est haute dans cette liste, moins elle coûte, moins elle crée de risque juridique et plus elle s'intègre au style par construction.*

### 7.2 Critères d'acceptation (décidé)

Un asset tiers n'entre que s'il remplit **tous** ces critères :

| Critère | Exigence |
|---|---|
| Licence | Acceptée (§7.3), lue à la source, datée et archivée au registre |
| Provenance | Auteur et adresse d'origine identifiés ; aucune trace d'asset extrait d'un autre jeu ou d'une bibliothèque payante republiée |
| Redistribution | La licence autorise la distribution **dans un paquet de jeu extractible**, puisque tout ce qui est livré au client est public ([revue de sécurité](../revues/securite.md), principe 1) |
| Contenu | Aucune marque, aucun logo, aucun insigne réel, aucune personne réelle reconnaissable (§1.4) ; rien qui contredise les limites de contenu ([combat.md](combat.md), §12) |
| Style | Passe les contrôles de palette, de densité et de budget (§6.9), après une retouche automatisable (recoloration, simplification, redimensionnement) |
| Intégration | Retouche faite par script, reproductible, enregistrée au registre (§9.1) |
| Territoire | Aucune restriction excluant l'Union européenne ou la France |
| Accès (v0.2.1) | Téléchargeable **gratuitement**, **sans compte à créer ni mot de passe à saisir** ([organisation](../../pilotage/organisation.md), §6) |
| Formats (v0.4, clôture de la sécurité C9) | **Formats de données seulement** : glTF, OBJ, PNG, OGG, WAV, OTF, TTF. Aucun fichier `.blend`, `.tscn` ou `.tres` venu d'un tiers n'est ouvert tel quel, car ils peuvent contenir du code exécuté à l'ouverture |

### 7.3 Licences acceptées et refusées (décidé)

| Accepté | Refusé |
|---|---|
| CC0, domaine public | **NC** (pas d'usage commercial) |
| CC BY 4.0, CC BY 3.0 (attribution) | **ND** (pas de modification) |
| SIL OFL (polices) | **SA** et GPL pour les assets (obligation de partage à l'identique, risque sur les œuvres dérivées) |
| MIT, Apache 2.0, BSD, zlib (outils et shaders) | « Usage éditorial seulement », « usage personnel » |
| Licences gratuites libres de droits autorisant l'inclusion dans un jeu distribué | Licences qui excluent un territoire de l'Union européenne |
| — | **Toute source payante**, et toute source qui exige un compte ou un mot de passe pour télécharger (v0.2.1) |
| — | Licences qui interdisent l'usage avec ou pour l'entraînement de modèles, si l'asset doit servir à un générateur |
| — | Toute origine inconnue ou invérifiable |

Un **outil** sous GPL (Blender, par exemple) reste utilisable : sa licence ne s'étend pas à ce qu'on produit avec lui.

### 7.4 Candidats par besoin **[à vérifier au registre]**

Liste de départ pour le responsable contenus ; chaque source est relue à la source avant usage.

| Besoin | Candidats | Rôle |
|---|---|---|
| Corps humain paramétrique, visages, cheveux, vêtements de base | MakeHuman et son greffon Blender MPFB (ressources du système en CC0) | Base du corps paramétrique (§3.2), ajusté et réduit par script |
| Animations | Base de capture de mouvement du CMU Graphics Lab (usage libre) ; bibliothèques d'animations CC0 (Quaternius) ; animations procédurales (§3.6) | Reciblées sur le squelette commun par script |
| Kit de ville | **Générateur maison** (§8.2) ; kits CC0 (Kenney, Quaternius) comme bouche-trous du prototype seulement | Les bouche-trous sont remplacés avant la fin de la phase 3 |
| Mobilier, objets | Poly Haven (CC0), Kenney (CC0), générateurs maison pour les meubles simples | — |
| Matériaux et textures | ambientCG (CC0), Poly Haven (CC0), Material Maker (outil MIT) pour les matériaux procéduraux | Recolorés selon les palettes |
| Végétation | Générateur d'arbres maison ; textures de feuilles CC0 | Une espèce par arbre d'alignement (§1.3) |
| Véhicules | Générateur paramétrique maison (carrosseries génériques) ; kits CC0 pour le prototype | Aucune silhouette de modèle réel |
| Effets sonores | Lots « GDC » de Sonniss (gratuits, libres de droits) ; Kenney (CC0) ; synthèse procédurale (pluie, vent, moteurs) | — |
| Musique des radios et des sonos | Morceaux CC BY (Kevin MacLeod, incompetech) et CC0 ; Free Music Archive (CC BY et CC0 seulement, si téléchargeable sans compte) | Radios fictives (§5.4) ; programme d'une à deux heures par station au départ, en boucle **[à calibrer]** |
| Musique des musiciens (v0.2.1) | **Générée par nos outils** : partitions du domaine public et mélodies composées par script, rendues avec des banques d'instruments libres (SoundFont sous licence permissive) ; la **qualité** du musicien s'obtient en ajoutant des écarts de tempo et des fausses notes | §5.4 ; répertoire par instrument et par style |
| Vocalisations | Banques libres de droits ; synthèse | §5.5 |
| Synthèse vocale (plus tard) | Piper (moteur MIT, licence propre à chaque voix), Kokoro (Apache 2.0) | §5.5 ; une voix n'est retenue que si sa licence et son jeu de données le permettent |
| Icônes | Kenney (CC0), Material Symbols (Apache 2.0), Lucide (ISC) | Interface et icônes de son |
| Polices | Google Fonts sous OFL (§4.6) | — |

Exclus d'office par leurs conditions **[à vérifier au registre]** : banques sonores à licence non commerciale (par exemple les effets de la BBC sous licence RemArc), modèles d'IA dont la licence exclut l'Union européenne (par exemple Hunyuan3D 2), modèles de génération musicale sous licence non commerciale (par exemple les poids de MusicGen). Écartés aussi par la règle d'accès (§7.2, v0.2.1) : **Mixamo** (compte Adobe requis), **Freesound** (compte requis pour télécharger) et toute bibliothèque payante.

### 7.5 IA générative (décidé)

Permise **seulement** sous toutes ces conditions :

1. La licence du modèle autorise l'**usage commercial des sorties**, sans restriction de territoire ; elle est archivée au registre avec la version exacte du modèle.
2. Le jeu de données d'entraînement est **documenté** et compatible avec l'usage commercial.
3. Le registre garde, pour chaque sortie, le **modèle, la version, l'invite et la graine** (§9.1).
4. **Interdits** : imiter le style d'un artiste vivant nommé, produire le visage, la voix ou la marque d'une personne ou d'une entreprise réelle, cloner une voix.
5. Toute sortie passe les mêmes contrôles qu'un autre asset (§6.9).
6. **Déclaration** (décidé, v0.2.1, lecture la plus protectrice selon l'arbitrage A8) : les contenus produits par IA (assets générés retenus, répliques formulées par le service de dialogue) sont **déclarés** auprès des plateformes de distribution et **annoncés aux joueurs** (écran d'information et crédits), à partir du registre (§9.1). Le responsable exploitation vérifie cette lecture sur les textes officiels avant l'ouverture publique. *Pourquoi : déclarer plus qu'il ne faut ne coûte rien, déclarer moins exposerait le jeu à un retrait.*

Usages retenus : textures et décalcomanies (affiches, motifs de tissus, tags), variations de matériaux, effets sonores courts. Usages **exclus au départ** : **musique** et **voix** générées, à cause de l'incertitude sur les droits et sur les données d'entraînement ; modèles 3D générés pour les objets principaux, dont la topologie et la licence sont encore trop incertaines **[à réexaminer à la phase 5]**.

*Pourquoi : l'IA générative peut remplacer l'artiste absent pour les éléments secondaires, mais un asset dont on ne peut pas prouver les droits est une dette que personne ne pourra rembourser après la sortie.*

### 7.6 Coûts (décidé)

- **Aucun achat** (décidé, v0.2.1, arbitrage A8) : ni licence, ni bibliothèque, ni abonnement, ni service, à aucune phase. Seules les sources gratuites, libres ou procédurales sont admises (§7.1 à §7.3).
- Quand un besoin n'est couvert par aucune source gratuite, le responsable contenus le couvre par un **générateur** (§8) ou par une version plus simple de l'asset, et le note dans son carnet ; il ne le présente jamais comme une demande.

*Pourquoi : aucune session ne peut payer, et rien ne doit dépendre d'une action de l'utilisateur ([organisation](../../pilotage/organisation.md), §6) ; un jeu réalisable sans dépense ne bloque jamais sur un achat.*

---

## 8. Ce que les générateurs produisent seuls

### 8.1 Trois producteurs, trois rôles (décidé)

| Producteur | Quand | Ce qu'il produit | Ce qu'il ne produit jamais |
|---|---|---|---|
| **`ts-worldbuild` et le générateur de population** (Rust, cœur) | Construction des artefacts de monde, constructions en jeu | **Données sémantiques** : typologies, travées, ouvertures, toitures, corniches, matériaux et entretien par instance, mobilier par référence au kit, arbres et mobilier urbain (V9), noms, adresses, enseignes ; génomes d'apparence, garde-robes et voix des habitants ; signatures graphiques des bandes ; logos et marques fictives (par graine) | Aucun maillage, aucune texture, aucun son (P7) |
| **Outils d'assets** (`tools/assets/`, scripts Python avec Blender sans interface et **sans `--enable-autoexec`**, Material Maker en ligne de commande, Rust ; Blender, ses greffons comme MPFB et Material Maker **épinglés par version et par empreinte**, v0.4) | Hors ligne, avant le jeu | **Fichiers d'assets** : éléments du kit, meubles simples, arbres, carrosseries, matériaux procéduraux, niveaux de détail, atlas, sons synthétiques | Aucune donnée de monde |
| **Client Godot** | À l'exécution | **Assemblage** : bâtiments à partir du kit, personnages à partir du corps et du catalogue, textes du monde, décalcomanies (tags, affiches, fissures), météo, ciel, saisons, éclairage, HLOD en 3D, voix indistinctes | Aucune variation au hasard qui changerait ce que voient deux joueurs (§3.4) |

### 8.2 Générateurs d'assets hors ligne (décidé)

| Générateur | Entrées | Sorties | Couverture visée |
|---|---|---|---|
| **Kit de façades** | Grammaire par style et époque (proportions, encadrements, corniches, balcons, menuiseries), matériaux | Éléments de travée, angles, rez-de-chaussée, toitures, lucarnes, en 2.5D puis 3D | Tous les styles du §1.3 et de [ville.md](ville.md), §4.4 |
| **Kit d'intérieurs** | Épaisseurs, hauteurs, types de lieu | Pans de murs, sols, plafonds, escaliers, cages d'ascenseur, portes, fenêtres vues de l'intérieur | Tous les types de lieu |
| **Mobilier simple** | Familles paramétriques (tables, chaises, lits, rangements, comptoirs, étagères, sanitaires, cuisines) par niveau de richesse | Meubles et variantes | ≈ 70 % du mobilier ; le reste vient de sources CC0 |
| **Mobilier urbain** | Familles (lampadaires par quartier, bancs, abribus, poubelles, bornes, barrières, panneaux) | Objets de rue | Tout le mobilier urbain |
| **Arbres** | Espèce, âge, taille (croissance par colonisation d'espace) | Branches, cartes de feuillage, niveaux de détail | Toutes les espèces du §1.3 |
| **Véhicules** | Gabarits par catégorie ([economie.md](economie.md), §5.9), trois époques de dessin, options de finition par gamme (§1.4) | Carrosseries génériques, roues, options, intérieurs simplifiés, décalcomanies d'usure | Tous les véhicules de [ville.md](ville.md), §7 |
| **Matériaux** | Graphes procéduraux et textures CC0 | Matériaux recolorables avec usure et salissure | Toute la bibliothèque (§6.5) |
| **Sons synthétiques** | Paramètres (pluie, vent, moteurs, ventilations, bourdonnements) | Boucles et variations | Fond et lieux (§5.2) |
| **Niveaux de détail et atlas** | Tout asset | Simplifications, imposteurs, atlas | Tous les assets |

Ce qui **ne peut pas** se générer de façon crédible et vient donc de sources (§7.4) : le corps humain de base et ses vêtements de base, la capture de mouvement, les sons enregistrés (voix non verbales, Foley, véhicules réels), la musique des radios et des sonos, les polices. La musique jouée par les habitants musiciens, elle, est générée par nos outils (§7.4, v0.2.1).

### 8.3 Générations à l'exécution et par graine (décidé)

- **Tags et signatures de bande** : chaque bande ([social.md](social.md), §6.6) reçoit du cœur une **signature graphique** sémantique : une **graine**, **deux ou trois couleurs** prises dans la palette des accents (§2.1) et un **style** (lettrage, contours, remplissage) ([social.md](social.md), §6.6) ; le client la dessine par un générateur de lettrage à partir de polices OFL déformées. Deux tags de la même bande se ressemblent, comme dans la réalité. Une bande issue d'une **scission** reçoit du cœur une signature **proche** (même style, une couleur gardée) : le générateur rend alors une parenté visible, sans que le client calcule lui-même aucun lien entre bandes.
- **Enseignes, logos, affiches** : composés à partir du nom, du type de commerce, de l'identifiant public du lieu et des polices du §4.6.
- **Fenêtres** : rideaux, voilages et lueurs intérieures variés d'après l'**identifiant public** du logement dans la carte, sans rien dire des occupants que la façade ne montre déjà.
- **Origine des graines** (décidé, v0.4, clôtures de la sécurité C3 et de l'architecture S-5 ; [05-architecture.md](../05-architecture.md), en-tête « Clôture ») :
  - une graine envoyée au client (signature de bande, tatouage, cicatrice ; catégorie, intensité et indice de variante des vocalisations, tirés dans le seul domaine `cosmetic/vocalization`) vient d'un domaine **`cosmetic/<usage>`** de l'aléa à clé, de type distinct, **jamais parent** d'un autre tirage ;
  - un cosmétique identique d'un monde à l'autre (enseigne, rideaux, programme de radio) est dérivé de l'**identifiant public** du lieu ou de l'objet ;
  - chaque générateur du client (radios, sonos, musiciens, voix indistinctes, tags, enseignes, rideaux) est une **fonction pure** du message reçu, de l'heure du monde et d'identifiants publics ; il ne lit **jamais** une poignée, une graine secrète du monde ni l'horloge locale, et un test du client vérifie que deux exécutions donnent le même résultat.

  *Pourquoi : une graine qui serait aussi la mère d'un tirage caché ferait fuir ce tirage, et un rendu qui dépendrait d'une poignée ou de l'horloge différerait d'un joueur à l'autre.*
- **Usure et dégâts** : décalcomanies posées par les paramètres d'entretien et l'état des dégâts transmis.

---

## 9. Registre des licences et Git LFS

### 9.1 Le registre des licences (décidé)

- Fichiers RON dans `client/registry/`, **un fichier par source** (un lot téléchargé, un générateur maison, un modèle d'IA), suivis dans Git ordinaire.
- Chaque entrée contient :

| Champ | Contenu |
|---|---|
| `id` | Identifiant de la source |
| `kind` | `generated`, `cc0`, `attribution`, `free_license`, `ai_generated` |
| `origin` | Adresse d'origine, auteur ou éditeur |
| `license` | Identifiant SPDX quand il existe, sinon nom exact ; copie du texte de licence archivée |
| `checked_on` | Date de lecture de la licence à la source, et session qui l'a lue |
| `archive_sha256` | Empreinte de l'archive d'origine conservée hors de Git (§9.3) |
| `files` | Fichiers de `client/assets/` couverts, avec leur empreinte |
| `modifications` | Script de retouche et ses paramètres |
| `attribution` | Texte exact à afficher dans les crédits, s'il en faut un |
| `obligations` | Autres obligations (mention, lien, interdiction de revente séparée) |
| `ai` | Pour l'IA : modèle, version, invite, graine |
| `access` | Mode d'accès : téléchargement gratuit et direct, sans compte (§7.2) |

- Les **crédits** du jeu sont **générés** à partir du registre ; le contrôle automatique refuse tout fichier non couvert (§6.9).
- Le registre sert aussi aux **déclarations** demandées par les plateformes de distribution sur les contenus produits par IA (§12.9).

*Pourquoi : un registre lisible par machine rend la conformité vérifiable à chaque PR, et non au moment de la sortie, quand il serait trop tard.*

### 9.2 Ce qui va où (décidé)

| Nature | Emplacement | Raison |
|---|---|---|
| Scripts des générateurs, catalogue, palettes, registre, scènes `.tscn` texte | **Git ordinaire** | Petits, diffables, source de vérité |
| Assets **régénérables** par nos outils | **`TS_CACHE\artifacts\assets\<empreinte>\`**, reliés dans `client/assets_gen/` (ignoré) | Règle « rien de régénérable dans le dépôt » ([revue du développement par agents](../revues/developpement-agents.md), A6) |
| Assets **tiers ou non régénérables** prêts pour le jeu (retouchés, au format livré) | **Git LFS**, dans `client/assets/` | A7 |
| Sorties d'IA retenues | **Git LFS** (une sortie d'IA n'est pas reproductible au bit près) | — |
| Archives d'origine des sources tierces, fichiers de travail | **Hors de Git**, dans le stockage froid des sources d'art (A7), avec l'empreinte au registre | Lourds, jamais livrés |
| Assets lourds de la 3D | **Hors de Git**, téléchargés par une commande à empreintes épinglées, sur le modèle des modèles GGUF (A8 de la revue du développement par agents), depuis un **stockage gratuit** : fichiers attachés aux versions du dépôt GitHub (décidé, v0.2.1 ; **[conditions de gratuité à vérifier par l'exploitation]**) | Le quota de LFS ne suffira pas à la 3D |

### 9.3 Règles Git LFS (décidé)

1. **Motifs LFS** : `client/assets/**` pour `*.glb`, `*.png`, `*.ogg`, `*.wav`, `*.otf`, `*.ttf` ([revue du développement par agents](../revues/developpement-agents.md), §5, motifs à compléter par les polices).
2. **Non extrait par défaut** (`lfs.fetchexclude`, A7) : seules la copie principale et la session qui travaille sur `client/` les extraient.
3. **Budget LFS** **[à calibrer]** : ≤ **4 Go** d'assets en 2.5D, historique compris, pour rester dans le quota gratuit de GitHub (≈ 10 Go de stockage et 10 Go de transfert par mois, A7). Au-delà, les assets les plus lourds passent au stockage gratuit hors de Git du §9.2 (décidé, v0.2.1) ; **jamais d'achat de capacité**.
4. **Commettre peu et définitif** : un asset n'entre dans LFS qu'**après** avoir passé les contrôles (§6.9) dans le worktree ; on ne commet pas d'essais successifs, car chaque version reste dans l'historique LFS.
5. **Aucune réécriture d'historique** pour gagner de la place sans décision de l'orchestrateur.
6. **Taille par fichier** **[à calibrer]** : ≤ 16 Mo pour une texture, ≤ 30 Mo pour un morceau de musique ; un fichier plus gros est découpé ou sort de LFS (§9.2).
7. `git lfs prune` dans la commande de nettoyage du disque (revue du développement par agents, §10).
8. **LFS en CI** (v0.4, clôture de l'architecture S-6) : `actions/checkout` toujours avec `lfs: false` ; seul le contrôle des assets récupère les fichiers LFS **modifiés** par la PR (`git lfs pull --include` sur les chemins du diff). *Pourquoi : les téléchargements depuis la CI comptent dans les 10 Go de transfert mensuels, et 4 Go d'assets extraits à chaque passage épuiseraient vite ce quota.*

---

## 10. Mise en œuvre par le responsable contenus

### 10.1 Intégrer un asset (décidé)

1. Vérifier que le besoin ne se couvre pas par un générateur (§7.1).
2. Choisir la source (§7.4), lire la licence **à la source**, l'archiver, remplir l'entrée du registre.
3. Télécharger l'archive vers le stockage froid, noter son empreinte. La décompresser dans un **dossier de quarantaine** neuf, après avoir vérifié que l'empreinte correspond au registre ; n'en garder que les **formats de données** admis (§7.2), et ne lancer aucun script fourni avec l'archive (v0.4, clôture de la sécurité C9).
4. Retoucher **par script** (recoloration, échelle, axes, simplification, niveaux de détail, normalisation sonore, retrait des métadonnées).
5. Ajouter la correspondance au catalogue (§6.7).
6. Lancer les contrôles (§6.9) et vérifier l'asset dans la scène étalon (§10.2).
7. Commettre dans une branche, ouvrir une PR courte (un lot cohérent), selon les règles du dépôt.

### 10.2 La scène étalon (décidé)

- Une scène Godot fixe, `client/scenes/style_reference/` : un îlot du **faubourg Saint-Roch** au bord du **Pont-Vieux**, avec un intérieur de logement, un bar et un atelier, une dizaine d'habitants de tous âges, de jour, au crépuscule et de nuit, sous ciel couvert, pluie et neige.
- Elle produit des **captures de référence** à chaque PR d'assets ; un écart de luminance ou de couleur au-delà d'un seuil **[à calibrer]** est signalé pour relecture.
- C'est le seul endroit où un jugement visuel global est demandé ; il est fait par le responsable contenus sur les captures.

*Pourquoi : le prototype de la phase 1 et du client 2.5D se déroule dans ce morceau de ville ([05-architecture.md](../05-architecture.md), §5) ; c'est là que le style doit être juste en premier.*

### 10.3 Phasage (décidé)

| Phase | Contenus à produire |
|---|---|
| **3** (client 2.5D en solo) | Kits « ancien » et « faubourg » (extérieur et intérieurs de logement modeste, bar, commerce, atelier), mobilier courant, mobilier urbain de la Cité et de Saint-Roch, deux espèces d'arbres, corps paramétrique avec ≈ 40 pièces de vêtement et ≈ 15 coiffures, ≈ 40 animations, fond et lieux sonores de deux quartiers, vocalisations de base, une radio d'une heure, polices et icônes. Scène étalon et contrôles en place |
| **4** (en ligne) | Pas de nouveau besoin artistique ; vérification de l'apparence perçue graduée (§3.5) |
| **5** (extension) | Tous les styles et quartiers, catalogue vestimentaire complet, uniformes, véhicules, intérieurs des services ([ville.md](ville.md), §5.2 à §5.4), lieux faits main, trois radios, puis la 3D et l'étude des voix synthétisées |

Le responsable contenus doit donc arriver **au début de la phase 3** (§12.1).

---

## 11. Valeurs à calibrer

Toutes au prototype de la phase 3, dans la scène étalon :

- taille d'un mètre à l'écran par cran de zoom, densités de texels ;
- palettes de référence et plages de saturation ;
- latitude fictive, durée des crépuscules, intensité de la nuit lisible ;
- nombre de lumières avec ombres la nuit ;
- budgets de triangles, d'appels de dessin et de mémoire vidéo ;
- taille du génome d'apparence, nombre de paramètres de visage, seuils anti-sosies, plafond de l'aire sous la courbe du test de parenté ;
- taille du catalogue vestimentaire et du catalogue d'animations ;
- niveaux sonores cibles ;
- budget LFS et tailles maximales par fichier.

---

## 12. Impacts sur le socle et les autres domaines

Impacts sur le socle intégrés le 29/10 par la session principale. Les demandes des §12.2 à §12.8 ont été transmises aux domaines le 29/10 ; celles du §12.9 attendent le responsable architecture et sécurité (phase 0).

### 12.1 Socle

| Fichier | Impact | État |
|---|---|---|
| [01-concept.md](../01-concept.md), §1.1 | « Aucun figurant » s'étend au **son** : aucune voix, aucun pas, aucun véhicule entendu sans source réelle perçue ; le fond sonore reste impersonnel (§0.4, §5.1) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1 | **Aucune marque, aucun insigne d'État ni aucune personne réels** dans le monde (§1.4) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1 | **Aucun animal visible** dans la première version ; oiseaux au son seulement (§3.9) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §1.1 | Création du persistant : l'éditeur d'apparence n'offre que l'espace produit par la population ; **aucun objet d'apparence propre aux joueurs**, ni cosmétique payant (§3.7) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.5 | **Apparence perçue graduée** en trois précisions (silhouette, tenue, visage) ; un visage masqué n'est jamais envoyé (§3.5) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.2 et §2.14 | Nouveau composant **génome d'apparence** (≈ 64 octets), avec garde-robe abstraite et voix, produit par le générateur de population, à hérédité limitée et testée (§3.2 à §3.4) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.9 | **Voix** : aucune parole dite en première version ; plus tard, synthèse optionnelle **dans le client de chaque auditeur**, pour toutes les paroles quel que soit le pilote, avec le même délai (§5.5) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §3 | Rendu : renvoi à ce fichier ; soleil calculé par le client à partir de la date et de l'heure ; textes du monde rendus à l'exécution ; catalogue de correspondances sémantique → asset dans le client ; aucune variation au hasard côté client (§1.5, §2.2, §3.4, §6.7) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §4 | Langages : outils d'assets en Python (Blender sans interface) et en Rust, dans `tools/assets/` (§8.2) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §5 | Phase 3 : scène étalon, contrôles d'assets, premier jeu de contenus sans achat ; phase 5 : 3D et étude des voix (§10.3) | Intégré au socle le 29/10 |
| [README](../README.md), hiérarchie des sessions | Le **responsable contenus** arrive au **début de la phase 3** (le client 2.5D est déjà en vraie 3D et a besoin d'assets), et non au début de la 3D (§10.3) | Transmis à l'orchestrateur le 29/10 (la hiérarchie des sessions est désormais dans `docs/pilotage/organisation.md`) |
| [README](../README.md), tableau des domaines | Statut du domaine direction artistique : rédigé en v0.1, à relire | Fait le 29/10 |

### 12.2 Domaine population ([population.md](population.md))

**État** : traité par population v0.8 (§1.3, §4.9, §4.10, §7.2, §7.4) ; ses retours sont intégrés ici en v0.2 (§12.10).

- Produire le **génome d'apparence** de chaque habitant (§3.2) : silhouette d'après l'âge, le sexe et les attributs physiques, jamais d'après le milieu ou l'origine ; visage, peau, cheveux par **hérédité limitée** (§3.4) ; voix ; goûts vestimentaires.
- Produire la **garde-robe abstraite** de chaque habitant, d'après l'âge, le milieu, le profil, le métier et l'appartenance à un groupe (§3.3) ; la compléter à chaque arrivée (`PersonFactory`).
- Ajouter à la validation (§1.3) : **anti-sosies**, dispersion des apparences par quartier, **test de parenté** par les apparences perçues (§3.4).
- Faire **vieillir** le génome (rides, grisonnement, calvitie, croissance des enfants).
- Donner une probabilité non nulle à toute combinaison offerte par l'éditeur du persistant (§3.7).

### 12.3 Domaine IA des PNJ ([ia-pnj.md](ia-pnj.md))

**État** : traité par ia-pnj v1.1 (§2.6) ; ses retours sont intégrés ici en v0.2 (§12.10).

- Choisir la **tenue du jour** dans la garde-robe selon l'activité, le lieu, la météo et l'occasion (§3.3) ; se changer en rentrant, prendre un parapluie.
- Aller chez le **coiffeur** de temps en temps (la coiffure fait partie de l'état courant, §3.2).
- Les énumérations de **gestes et d'activités** servent de clés au catalogue d'animations (§3.6) : chaque valeur a un identifiant stable et une famille, vérifiés par le paquet de contenus ; les animations sont vérifiées par `cargo xtask assets check`, côté client (§3.6, §6.9).
- Activités musicales des habitants (jouer d'un instrument, écouter la radio, mettre de la musique dans un bar) : la musique entendue en découle (§5.4).

### 12.4 Domaine interface ([interface.md](interface.md))

**État** : traité par interface v0.6 (§15.12) ; ses retours sont intégrés ici en v0.2 (§12.10).

- Appliquer les styles des deux couches (§4.3), les bulles et icônes de son (§4.4), la typographie (§4.6).
- L'éditeur d'apparence du persistant se limite à l'espace de la population (§3.7).
- Portraits rendus à partir de l'apparence connue (§3.5).
- Accessibilité : plafond de 3 éclats par seconde et option qui les supprime, palettes pour daltoniens appliquées aux signaux du monde, sous-titres des sons, son mono, plage dynamique réduite (§4.5, §5.7).
- Réglages : option « musique d'accompagnement » en jeu ; plus tard, option « voix synthétisées » (§5.4, §5.5).
- Les crans de zoom fixent la densité de texels (§1.2) : tout changement des crans est à signaler.

### 12.5 Domaine ville ([ville.md](ville.md))

**État** : transmis le 29/10 ; aucun retour reçu à la v0.3.

- Les **identités de quartier** du §1.3 (matériaux, toits, espèces d'arbres d'alignement, mobilier urbain) deviennent des paramètres du plan directeur et des gabarits, utilisés par V6 et V9.
- Les **textes** des plaques, enseignes et panneaux sont des données rendues par le client (§1.5).
- Les **marques fictives** (enseignes, compagnie de bus, radios, journal) sont générées en V11 avec leur logo par graine (§1.4, §8.3).
- Les **valeurs d'atténuation** du son et le **type de réverbération** de chaque type de pièce sont dans le catalogue sémantique (§5.3).
- Les illuminations de fin d'année et le décor saisonnier sont des éléments de l'espace public pilotés par la date (§2.5).

### 12.6 Domaine social ([social.md](social.md))

**État** : traité par social v0.7 (§3.2, §6.6) ; retours intégrés en v0.3 (§12.12).

- Chaque bande reçoit une **signature graphique** sémantique (graine, couleurs, style), utilisée pour ses tags et ses signes vestimentaires (§3.3, §8.3).
- Conversations entendues sans être comprises : rendues par des **voix indistinctes** (§5.5), à partir des bribes déjà découpées par le serveur.
- Le **ton** d'une réplique se lit à l'icône et au mot de la bulle (arbitrage A10) et aux vocalisations ; le volume, au bord de la bulle (§4.4, §5.5).

### 12.7 Domaine combat, police et justice ([combat.md](combat.md))

**État** : traité par combat v0.5 (§1.5, §2.4, §4.3, §5.1, §12.3, §16.9) ; ses retours sont intégrés ici en v0.2 (§12.10).

- **Uniformes et insignes fictifs** pour la police, les secours et la justice (§1.4) ; armes génériques sans marque réelle.
- Une **blessure grave** peut laisser une **cicatrice**, signe distinctif du génome d'apparence (§3.2) ; à décider par le combat.
- Chaque asset de violence (sang, vêtement taché, son de coup ou de tir) a sa **variante atténuée** ; aucun son de blessure détaillé (§4.5, §5.6).
- La **cagoule**, la capuche et le déguisement retirent des champs de l'apparence perçue (§3.5) ; leur effet sur la reconnaissance par les témoins reste celui du combat.

### 12.8 Domaine économie ([economie.md](economie.md))

**État** : traité par économie v0.7 (§3.8, §5.9, §5.13) ; retours intégrés en v0.3 (§12.12).

- Les **vêtements** sont achetés et renouvelés sur le poste « habillement » : la garde-robe abstraite s'use et se remplace selon le budget (§3.3) ; à préciser par l'économie (vêtement par pièce ou garde-robe agrégée).
- Les **véhicules** sont des modèles génériques par catégorie, sans marque (§1.4).

### 12.9 Revues techniques (responsable architecture et sécurité, puis développement et exploitation)

**État** : en attente du responsable architecture et sécurité (phase 0).

- **Sécurité** : apparence perçue graduée, visage masqué jamais envoyé, aucun visage conservé d'une poignée à l'autre (§3.5) ; hérédité limitée et test de parenté (§3.4) ; aucun objet d'apparence propre aux joueurs (§3.7) ; aucune musique ni ambiance dépendant de ce qui n'est pas perçu (§5.1) ; aucune métadonnée personnelle dans les assets (§6.1). À relire par la revue de sécurité.
- **Développement par agents** : dossiers `client/catalog/`, `client/registry/`, `client/assets_gen/` (ignoré) et `tools/assets/` dans l'arborescence (§6.8) ; motifs LFS des polices ; commande de contrôle des assets en CI (§6.9) ; budget LFS de 4 Go en 2.5D et stockage externe épinglé pour la 3D (§9.2, §9.3) ; `tools/assets/` est un fichier partagé du dépôt, confié au responsable contenus.
- **Architecture** : faisabilité d'une synthèse vocale locale dans la bibliothèque native du client (modèle ONNX exécuté par `tract`) pour l'étude de la phase 5 (§5.5).
- **Exploitation** : déclaration des contenus produits par IA (assets générés, répliques formulées par le service de dialogue) auprès des plateformes de distribution, et information des joueurs, selon la **lecture la plus protectrice** (décidé, v0.2.1, arbitrage A8), y compris au regard des obligations de transparence du règlement européen sur l'IA ; le responsable exploitation la vérifie sur les textes officiels avant l'ouverture publique (§7.5, §9.1).

### 12.10 Retours des domaines intégrés (v0.2)

| Domaine | Retour | Traitement dans ce fichier | État |
|---|---|---|---|
| [ia-pnj.md](ia-pnj.md) v1.1, §2.6 | Énumérations de gestes, d'activités et de postures à identifiant stable et à **famille**, repli du client sur l'animation de la famille | §3.6, §6.7, §6.9 (point 7) | Intégré |
| | **Source musicale** par lieu ou objet (station, sono, musicien avec instrument, style et qualité ; volume), morceaux choisis par le client | §5.4 : choix déterministe par station, ou par identifiant stable du lieu et heure du monde ; trois niveaux d'interprétation selon la qualité. Accord avec la v0.1 : seule la sono s'ajoute | Intégré |
| [population.md](population.md) v0.8, §4.9, §4.10, §12.10 | Plages et lois du génome, catalogue vestimentaire (âges, milieux, usages, postes), liste des uniformes | §3.10 a, b et c | Intégré |
| | Hérédité par les **parents officiels** seulement | §3.4 | Intégré |
| | **Voix sonore** distincte de la voix de style | §3.2, §5.5 | Intégré |
| [interface.md](interface.md) v0.6, §15.12 | Formes de bulle par ton, dont le **ton sec à bord droit** | §4.4 | Remplacé en v0.3.1 par l'arbitrage A10 : ton par une icône et un mot |
| | Icônes **sirène** et **klaxon** | §4.4 | Intégré |
| | **Clé de sous-titre** pour chaque son à icône et chaque son visible qui compte | §4.4, §6.7, §6.9 (point 7 bis) | Intégré |
| | Portraits : cadrage neutre ; visage dans l'ombre aux précisions « silhouette » et « tenue » ; jamais de visage neutre en gros plan | §3.5 : le « visage neutre » de la v0.1 devient un **visage non résolu**, aussi dans le monde | Intégré |
| | Pièce unique de passage de relais ; pièces de bilan selon ce que le bilan affiche | §5.4 | Intégré |
| | Teinte du liseré de la couche joueur contrôlée contre la palette | §4.3, §6.9 (point 7 ter) | Intégré |
| | Typographie retenue ; crans de zoom ≈ 50 et ≈ 12 px par mètre | §4.6, §1.2 | Intégré |
| [combat.md](combat.md) v0.5, §16.9 | **Motifs de cicatrices** (type, zone, taille, visibilité ; ≈ 30 % des blessures graves, ≈ 70 % des critiques) | §3.2, §3.10 d | Intégré |
| | **Brassard** des policiers en civil | §3.3, §3.10 c | Intégré |
| | Refus de tout asset de violence sans variante atténuée | §6.7, §6.9 (point 7 bis) | Intégré |

**Point signalé à la session principale** : [population.md](population.md), §4.9, inscrit une cicatrice « pour chaque blessure grave de la biographie », alors que [combat.md](combat.md), §2.4, n'en laisse qu'à ≈ 30 % des blessures graves et ≈ 70 % des critiques. Ce domaine suit le combat (§3.10 d). Il reste à aligner la population, pour que les habitants de départ et ceux blessés en cours de partie aient la même proportion de cicatrices. **Arbitré (A9)** : la population s'aligne sur le combat.

**Nouveaux impacts de la v0.2** :

- **Développement** : emplacement `data/content/appearance/` des plages, lois et catalogues d'apparence (§3.10) ; contrôles 7, 7 bis et 7 ter (§6.9).
- **Économie** : la liste des uniformes (§3.10 c) est à compléter avec les postes de l'économie, et le poste « habillement » paie la garde-robe (§12.8).

### 12.11 Règle « aucune action de l'utilisateur » (v0.2.1)

Application de l'arbitrage A8 ([05-architecture.md](../05-architecture.md), en-tête) et de [organisation.md](../../pilotage/organisation.md), §6 : aucune action n'est attendue de l'utilisateur dans ce fichier.

| Point | Avant | Après (décidé) |
|---|---|---|
| Achats (§7.1, §7.3, §7.6) | Licences commerciales achetées, confirmées et payées par l'utilisateur | **Aucun achat** ; sources gratuites, libres ou procédurales seulement |
| Accès aux sources (§7.2, §7.4) | Mixamo et Freesound admis | Toute source qui exige un compte ou un mot de passe est écartée ; animations procédurales et CC0, effets sonores sans Freesound |
| Musique (§5.4, §7.4, §8.2) | Bibliothèques sous licence de synchronisation | Radios et sonos en CC BY et CC0 ; musiciens générés par nos outils |
| Registre (§9.1) | Champ `purchase` avec l'accord de l'utilisateur | Champ `access` (téléchargement gratuit, sans compte) ; type `free_license` à la place de `commercial` |
| Stockage (§9.2, §9.3) | Achat de capacité LFS possible ; stockage de la 3D à choisir | Jamais d'achat de capacité ; assets lourds hors de Git dans un stockage gratuit |
| Points de droit (§7.5, §12.9) | « À faire confirmer par un juriste » | Lecture la plus protectrice : déclaration des contenus produits par IA aux plateformes et information des joueurs, vérifiée par l'exploitation sur les textes officiels avant l'ouverture publique |
| Jugements visuels (§10.2) | Déjà confiés au responsable contenus | Inchangé : aucun essai ni aucune relecture de l'utilisateur |

### 12.12 Dernière vague avant la clôture (v0.3)

| Domaine | Retour | Traitement dans ce fichier | État |
|---|---|---|---|
| [social.md](social.md) v0.7, §12.12 | Par acte entendu sans être compris : **ton** (5 valeurs), **volume** (chuchoté, normal, élevé, crié) et **durée** | §4.4 : le bord de la bulle suit le volume, le ton passe par une icône et un mot (A10, v0.3.1) ; §5.5 : voix indistinctes réglées par ces trois valeurs | Intégré |
| | Signature graphique : graine, deux ou trois couleurs, style ; signature **proche** après une scission | §8.3 | Intégré |
| [interface.md](interface.md) v0.7, §15.13 | Gestes **dévisager**, **se changer**, **coupe au salon**, **allumer une radio** et **changer de station** | §3.6 | Intégré |
| [economie.md](economie.md) v0.7, §14.10 | Véhicules par **catégorie, âge, état et gamme** ; gamme visible seulement par la finition, l'état et les options | §1.4, §8.2 | Intégré |
| | Gamme des vêtements, visible seulement par la finition et l'état ; garde-robe agrégée pour l'économie, pièces visibles inchangées | §3.10 b (champ `range`) | Intégré, sans contradiction avec §3.3 |
| | Organisme de redevance musicale au nom fictif | §1.4 | Intégré |
| Responsable conception | Emplacement `data/content/appearance/` confirmé (05 §2.2) ; cicatrices : la population s'aligne sur le combat (A9) | §3.10, §12.10 | Intégré |

**Impacts de la v0.3 sur d'autres domaines** (aucune demande envoyée, règle de clôture) :

| Destinataire | Impact | État |
|---|---|---|
| [05-architecture.md](../05-architecture.md), §2.5 | L'apparence perçue d'un **véhicule** comprend sa catégorie, son époque de dessin, son état, sa gamme visible et sa couleur, en valeurs sémantiques (§1.4) | Pour la phase 1 |
| [population.md](population.md) | Chaque pièce de la garde-robe de départ porte la **gamme** du personnage (§3.10 b) | Pour la phase 1 |

### 12.13 Revues de clôture (v0.4)

| Revue | Correction | Traitement dans ce fichier | État |
|---|---|---|---|
| Architecture C-3 | Soleil par une seule fonction `solar_state` de `ts_world`, exposée par `ts_client_native` | §2.2 | Intégré |
| Architecture C-4 | Graine de forme des cicatrices et tatouages seulement à la précision « visage » | §3.5, §3.10 d | Intégré |
| Sécurité C2 | Résolution de chaque champ selon la précision ; classe de voix grossière ; test de réidentification ; test de parenté à la précision « visage », naissances et fratries comprises | §3.4, §3.5, §5.5 | Intégré |
| Sécurité C3, architecture S-5 | Graines cosmétiques `cosmetic/<usage>` ; générateurs du client en fonctions pures d'identifiants publics | §8.3 | Intégré |
| Sécurité C9 | Formats de données seulement ; Blender sans `--enable-autoexec` ; quarantaine avec empreinte vérifiée ; outils et greffons épinglés | §7.2, §8.1, §10.1 | Intégré |
| Architecture C-1 | Contrôle des familles d'animation côté client, jamais dans la construction du paquet | §3.6, §6.9 | Intégré (aligne la v0.2, qui faisait refuser par le paquet) |
| Architecture S-4 | Indice de variante des vocalisations tiré par le cœur | §5.5 | Intégré |
| Architecture S-6 | `lfs: false` en CI, seuls les fichiers LFS modifiés récupérés par le contrôle des assets | §9.3, point 8 | Intégré |

**Ce qui reste, sans demande envoyée** (règle de clôture) :

| Destinataire | Impact | État |
|---|---|---|
| [05-architecture.md](../05-architecture.md), [economie.md](economie.md) | Une seule vérité pour l'usure de la garde-robe (architecture S-7) : l'usure agrégée de l'économie fait foi ; les états des pièces du §3.3 deviennent passagers (propre, froissé, sale, taché, mouillé), et « déchiré » ou « usé » se déduit de l'usure agrégée par une fonction déterministe | Pour la phase 1 |
| Développement | Les identifiants des pièces de vêtement et des familles (§3.6, §3.10 b) entrent dans le fichier de verrouillage `data/content/ids.lock` en ajout seul (architecture S-3) | Pour la phase 1 |

Aucune contradiction avec le socle ni avec les autres domaines.
