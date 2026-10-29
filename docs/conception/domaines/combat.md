# Domaine — combat, police et justice

> Fichier de **domaine** : modifié uniquement par la session « Conception — combat, police et justice » (voir [README](../README.md)).
> Version 0.1 — 29/10/2026 : première rédaction complète. Traite les demandes adressées à ce domaine par [ville.md](ville.md) (§11.2), [ia-pnj.md](ia-pnj.md) (§11), [population.md](population.md) (§12.6), [economie.md](economie.md) (§7.8 et §14.6) et [social.md](social.md) (§12.5).
> Version 0.2 — 29/10/2026 : intégration des revues d'équilibrage ([revues/equilibrage.md](../revues/equilibrage.md) §6.7 : E1, E3, E8, E19, E21, E22, E23, E27, E28, E29) et de sécurité ([revues/securite.md](../revues/securite.md) §9.11 : S9, S35), avec les arbitrages de la session principale A3 (cercle du joueur) et A5 (défense par posture). Sections touchées : principe 5, §1.2, §1.3, §1.5, §1.7, §5.4, §7.2, §9.5, §11.2 à §11.6, §11.8, §12.2, §15, §16.
> Version 0.3 — 29/10/2026 : intégration de la revue de l'intelligence des PNJ ([ia-pnj.md](ia-pnj.md) v0.8, §11 ligne « combat.md », §13) : anomalies (§13.8), faits sensibles et soupçon (§13.5), attroupements (§13.9), habituation (§13.11), conclusion « lieu dangereux » (§13.4), cohérence entre niveaux (§13.12). Sections touchées : §4.2, §4.5, §5.3, §5.4, §6.2, §6.4, §11.2, §13.1, §15, §16.
> Version 0.4 — 29/10/2026 : intégration de la revue des signatures ([revues/signatures.md](../revues/signatures.md) §9.8 : G1, G3 à G9, avec les renvois G16, G22, G23 et G28) et de la revue de la génération procédurale ([revues/generation.md](../revues/generation.md) §13.10 : P12, P13). Sections touchées : §1.2, §1.7, §3.1, §3.2, §4.1, §8.1, §10.1, §11.1 à §11.3, §15, §16.
> Version 0.4.1 — 29/10/2026 : reliquat du social ([social.md](social.md) v0.6, §12.11) : l'ordre de frapper d'un chef de bande est une instigation, jugée comme une complicité (§10.2).
> Version 0.4.2 — 29/10/2026 : reliquats croisés demandés par la session responsable de la conception : aucun décès d'enfant, même naturel, dans la première version ([population.md](population.md), §7.3 et §12.6 ; §12.2) ; transmission à la succession des condamnations pour la mort du défunt, y compris après le partage ([economie.md](economie.md), §5.11 et §14.8, E31 ; §8.7). Sections touchées : §8.7, §12.2, §16.4, §16.5.
> Version 0.5 — 29/10/2026 : intégration de la direction artistique ([direction-artistique.md](direction-artistique.md) §12.7 : uniformes et insignes fictifs, armes génériques, cicatrices, variantes atténuées, masques) et de la revue des signatures du contenu ([revues/signatures-contenu.md](../revues/signatures-contenu.md) §11.3 : violation de domicile, A23 ; magnitude de la violation de la loi, M2 ; escalade sans « insulter », M11 ; « Fouiller », D31). Sections touchées : §1.5, §1.7, §2.4, §4.3, §5.1, §8.1, §8.5, §11.2, §12.3, §16.
> Version 0.6 — 30/10/2026 : dernière vague avant la clôture de la conception. Demandes de [social.md](social.md) v0.7 (§12.12 : tapage nocturne, usurpation de fonction, couleurs des bandes) et d'[economie.md](economie.md) v0.7 (§14.10 : client ivre, vente à un mineur, jeux clandestins) ; application de l'arbitrage **A8** (aucune action de l'utilisateur ni d'un humain : session de modération sur dossier, panel de sessions évaluatrices) et de la **règle 6** du README (aucune section renumérotée : les ajouts sont des puces et des lignes de tableau). Sections touchées : en-tête, §1.2, §6.4, §8.1, §8.5, §10.3, §10.9, §11.6, §11.8, §15, §16.
> **Numérotation** : les sections de ce fichier ne sont jamais renumérotées (règle 6 du [README](../README.md)) ; une nouvelle section prend un numéro libre ou un suffixe (« §5.4 bis »), une section retirée garde son numéro avec la mention « supprimée ».
> Tous les choix de ce fichier ont été pris par la session de domaine, sur consigne de l'utilisateur : ils sont marqués **(décidé)** et accompagnés de leur raison. Les valeurs marquées **[à calibrer]** seront ajustées par le prototypage et la chaîne de calibrage (`ts-sim`), sans action de l'utilisateur (A8). Ce qui touche le socle ou un autre domaine est rassemblé au §16 et n'est appliqué nulle part ailleurs.

Ce domaine décrit la violence et ce que la société en fait : les affrontements et les blessures, les effractions et les dégâts, les témoins, la police, les enquêtes, la justice, la détention, la criminalité organisée ou non, et les règles propres aux joueurs. Il s'appuie sur le socle, en particulier [01-concept.md](../01-concept.md) (§2 : conséquences équivalentes en ligne), [02-personnages.md](../02-personnages.md) (attributs, paliers, non-jouabilité), [03-vie-synchronisation.md](../03-vie-synchronisation.md) (deux barres de vie, mort) et [04-joueur.md](../04-joueur.md) (signature comportementale, bilan) ; sur [ville.md](ville.md) pour les lieux, les portes, la surveillance et les dégâts ; sur [ia-pnj.md](ia-pnj.md) pour les réflexes, la perception, la mémoire et les compétences ; sur [population.md](population.md) pour le milieu criminel de départ ; sur [economie.md](economie.md) pour l'économie parallèle, les traces et le mécanisme d'amende et de saisie ; sur [social.md](social.md) pour les croyances, les rumeurs, les bandes, la loi du silence et la modération.

## Sommaire

0. [Principes](#0-principes)
1. [Affrontements](#1-affrontements)
2. [Blessures, santé et mort](#2-blessures-santé-et-mort)
3. [Effractions, vols et dégâts matériels](#3-effractions-vols-et-dégâts-matériels)
4. [Témoins et victimes](#4-témoins-et-victimes)
5. [La police](#5-la-police)
6. [Les enquêtes](#6-les-enquêtes)
7. [Interpellation, garde à vue et poursuites](#7-interpellation-garde-à-vue-et-poursuites)
8. [La justice](#8-la-justice)
9. [Détention et conséquences équivalentes](#9-détention-et-conséquences-équivalentes)
10. [La criminalité](#10-la-criminalité)
11. [Règles pour les joueurs](#11-règles-pour-les-joueurs)
12. [Limites de contenu](#12-limites-de-contenu)
13. [Niveaux de détail et coût de calcul](#13-niveaux-de-détail-et-coût-de-calcul)
14. [Fiches de rôle et effectifs fournis](#14-fiches-de-rôle-et-effectifs-fournis)
15. [Points ouverts](#15-points-ouverts)
16. [Impacts sur le socle et les autres domaines](#16-impacts-sur-le-socle-et-les-autres-domaines)

---

## 0. Principes

1. **La violence est rare, brève et coûteuse** (décidé). Dans une ville moyenne européenne, la plupart des conflits se règlent par des mots, et la plupart des bagarres durent quelques secondes. Un coup a des suites durables : blessure, plainte, rumeur, rancune, casier. *Pourquoi : c'est ce qui rend la violence crédible et lourde de sens dans un monde sans figurants, où chaque victime est un vrai habitant.*
2. **Aucune magie, aucun chiffre affiché** (décidé). Un affrontement est résolu par les attributs réalistes, les compétences et l'état du moment (fatigue, stress, blessures), selon la règle de la progression réaliste : `performance = capacité × compétence × modificateur d'état`. Le joueur choisit ses gestes ; le personnage les réussit selon ses capacités. *Pourquoi : c'est la règle du socle pour toutes les actions, déjà appliquée aux actes sociaux ([social.md](social.md), §3.1).*
3. **Joueur et IA font les mêmes gestes** (décidé). Les intentions de combat, d'effraction ou d'enquête sont les mêmes pour un joueur et pour l'IA, exécutées par les mêmes systèmes, avec les mêmes limites ([ia-pnj.md](ia-pnj.md), §0). *Pourquoi : c'est la règle « le pilote est un composant » ; sinon la reprise par l'IA se verrait.*
4. **Le monde ne sait que ce qu'il a perçu** (décidé). La police, la justice et les témoins ne disposent que de croyances, avec leur certitude ([social.md](social.md), §4). Il n'existe aucune vérité accessible au monde : un coupable peut échapper à la justice, un innocent être condamné. Le **journal des actions** du serveur ([05-architecture.md](../05-architecture.md), §2.8) ne sert jamais de preuve dans le monde. *Pourquoi : c'est le principe « l'IA n'en sait pas plus que le personnage » appliqué aux institutions ; une police omnisciente rendrait le crime impossible et le jeu sans intérêt.*
5. **La justice du monde ne connaît pas les joueurs** (décidé). Un personnage piloté par un joueur est arrêté, jugé et puni exactement comme un autre. Les seules règles hors fiction de ce domaine sont : les **conséquences équivalentes** du personnage persistant en ligne et sa garde à vue accélérée (§7.2, §9.5), le **délai de déconnexion** (§11.5), la **récusation et le conflit d'intérêts** du cercle du joueur (§11.8), les **protections contre le harcèlement** (§11.6, §11.7) et la séparation entre **crime dans le jeu** et **abus envers des personnes ou le monde partagé**, traité par la modération (§11.6). Les limites de contenu (§12) ne sont pas des règles hors fiction : elles retirent des actes du catalogue pour tous. *Pourquoi : c'est la règle du social ([social.md](social.md), §0, principe 2), et les seules exceptions sont celles qu'impose le temps réel ou la protection des joueurs.*
6. **Des limites de contenu garanties par construction** (décidé). Ce qui est exclu (violences sexuelles, violence d'un adulte envers un enfant, torture, suicide…) n'existe pas dans le catalogue des actions : on ne peut pas le faire, ni comme joueur ni comme IA (§12). *Pourquoi : c'est la méthode déjà retenue pour la règle adulte-mineur ([social.md](social.md), §1.6) ; une interdiction par construction ne dépend ni de la modération ni de la bonne volonté.*
7. **La criminalité naît des personnages** (décidé). Il n'y a ni « taux de crime » imposé, ni événement scripté : un crime est une recette choisie par un personnage selon ses besoins, ses projets, son profil et l'occasion ([ia-pnj.md](ia-pnj.md), §1.4 et §3.3). Les taux de référence du §10.9 servent seulement à calibrer et à tester. *Pourquoi : c'est la seule façon de garder une criminalité cohérente avec les personnes qui la commettent, et de la laisser changer quand le monde change.*

---

## 1. Affrontements

### 1.1 Le déroulé d'un affrontement (décidé)

*Pourquoi : la plupart des violences réelles sont précédées d'une escalade visible ; la représenter donne aux personnages, et aux joueurs, l'occasion de l'éviter.*

Un affrontement passe en général par une **escalade**, que chaque participant peut interrompre :

| Étape | Exemples | Ce qui se joue |
|---|---|---|
| **Tension** | Regard appuyé, reproche, provocation, insulte | Actes sociaux de conflit ([social.md](social.md), §3.1) ; les témoins remarquent |
| **Intimidation** | Menace, s'approcher trop près, bousculer, montrer une arme | Acte « menacer » ; la cible cède, fuit, appelle à l'aide ou riposte selon sa **crainte** et son profil |
| **Coups** | Gifle, coup de poing, saisie, mise au sol | Corps à corps (§1.3) |
| **Arme** | Couteau sorti, objet saisi, arme à feu brandie ou utilisée | §1.4 et §1.5 ; changement de nature de l'affrontement pour tous les présents |

- **Désescalade** : s'excuser, reculer, lever les mains, partir, payer, céder ce qui est exigé. Une désescalade réussie dépend des actes choisis et de l'état de l'adversaire (colère, ivresse, crainte d'être vu).
- **Attaque sans escalade** : une agression par surprise (dans le dos, à l'improviste) saute les premières étapes. Elle est plus dangereuse pour la victime (§1.2) et plus lourde pour son auteur (préméditation, §8.1).
- **Fin d'un affrontement** : fuite d'un camp, reddition (acte « se rendre », mains levées), victime hors de combat (au sol, inconsciente), intervention d'un tiers (témoin, vigile, police), ou épuisement des deux camps. Un affrontement à mains nues dure rarement plus de **30 à 60 secondes** **[à calibrer]**, à cause de la fatigue (§1.6).

### 1.2 La résolution d'un geste (décidé)

*Pourquoi : une résolution en temps réel, sans tour ni dé visible, reprend la règle de performance du socle et convient au même titre au joueur, à l'IA et au serveur autoritaire.*

Le combat se déroule **en temps réel**, sans tour de jeu ni pause. Chaque geste est une **intention** (§1.7) résolue par le serveur à sa réception :

```
chance de toucher = f(précision de l'attaquant, défense du défenseur, distance, angle, surprise, visibilité)
précision        = coordination × compétence de combat × état (fatigue, stress, douleur, ivresse)
défense          = temps de réaction × compétence de combat × état, réduite si le défenseur est pris par surprise,
                   occupé ou attaqué par plusieurs adversaires
effet            = f(force et puissance de l'attaquant, arme, zone touchée, protection, masse du défenseur)
```

- **Surprise** : un défenseur qui ne perçoit pas l'attaque ([ia-pnj.md](ia-pnj.md), §4.1) n'a presque aucune défense. Un guet-apens est donc très efficace, ce qui est réaliste.
- **Nombre** : la défense se répartit entre les adversaires ; à trois contre un, le défenseur seul ne pare plus grand-chose.
- **Temps de réaction** : il fixe le délai entre l'intention et le geste, et la fenêtre pour parer ou esquiver. Il est dégradé par la fatigue accumulée (dette de sommeil), l'alcool, les drogues et l'âge.
- **La défense est une posture** (décidé, A5 ; répond à [revues/securite.md](../revues/securite.md) §8, point 4) : le joueur choisit une **posture défensive** (garde, esquive, se couvrir, reculer, §1.7) et c'est le **personnage** qui l'applique, avec **son propre temps de réaction**. Une intention de défense n'a d'effet qu'une fois ce temps de réaction écoulé, compté depuis le moment où le personnage **perçoit** le geste adverse ; réagir plus vite que le personnage n'apporte rien. La fenêtre et la qualité de la parade viennent de la formule de défense ci-dessus. L'IA suit exactement la même règle. *Pourquoi : la performance doit venir du personnage et non de la main du joueur, comme le veut la règle de performance du socle (principe 2) ; une parade automatique par programme perd ainsi presque tout son intérêt.*
- **Le personnage domine la précision et la défense** (décidé, S35) : la dispersion, la stabilisation de la visée avec le temps et le temps de réaction du personnage pèsent bien plus que l'habileté de la main du joueur (§1.5). Des parades ou des tirs d'une **régularité surhumaine** (délais entre perception et parade, précision des tirs, comparés par compte à la population des joueurs) produisent une **alerte** de modération, jamais une sanction automatique ; cette détection, hors fiction, reste en place malgré la posture défensive. La sanction éventuelle est décidée par une **session de modération** distincte de la détection, sur dossier, journalisée et contestable (décidé, A8). *Pourquoi : les seuls avantages qu'un programme d'assistance pourrait encore tirer doivent être visibles, et la sanction doit venir d'un examen du dossier, jamais du seul seuil.*
- **Environnement** : l'éclairage (nuit, lampadaires cassés, [ville.md](ville.md), §6.4), la météo (pluie qui réduit la visibilité et couvre les bruits, sol glissant qui fait chuter, [01-concept.md](../01-concept.md), §3), l'encombrement (une cage d'escalier, un bar plein) et la hauteur (escaliers, toits, berges de l'Ostrelle) changent l'issue.
- **Pas de verrouillage magique** : un coup part dans une direction ; il peut toucher un tiers qui se trouve sur sa trajectoire, sous réserve des limites du §12.2.
- **Géométrie étagée** (décidé, P13) : lignes de vue, trajectoires de tir et de lancer, couverture et portée reposent sur le **modèle physique étagé** de la ville : par niveau (sol, étages, cave, toit), des segments de murs avec leur **plage de hauteur**, des **ouvertures** (fenêtres, portes, balcons) et des liaisons entre niveaux, en centimètres entiers. Un tir traverse une ouverture, passe au-dessus d'un muret bas, s'arrête sur un mur plein ; un tir d'un étage vers la rue suit la hauteur des deux niveaux. Le passage à la 3D ne change que le rendu, pas ces règles. *Pourquoi : c'est le modèle du cœur pour toute la ville ; il suffit au combat d'une ville européenne (rues, cages d'escalier, fenêtres) et coûte dix fois moins qu'une géométrie 3D.*
- **Latence en ligne** : le serveur résout les coups avec une compensation de latence **limitée**, pour qu'un joueur mal connecté ne soit ni avantagé ni injouable. Elle est **calculée par le serveur** à partir du temps d'aller-retour mesuré, jamais déclarée par le client, et plafonnée à **250 ms** **[à calibrer]** ; la date fine d'une intention est **bornée par le serveur** à partir de ce temps mesuré, ce qui empêche d'antidater une parade ou de profiter d'une coupure volontaire du réseau (décidé, S9). *Pourquoi : tout ce qui vient du client peut être falsifié ; seul le serveur peut dire quand un geste a eu lieu.*

### 1.3 Le corps à corps (décidé)

*Pourquoi : les gestes retenus couvrent ce que fait réellement une personne qui se bat, et chacun a un effet distinct sur les blessures et sur la qualification judiciaire.*

| Geste | Effet | Attributs et compétences |
|---|---|---|
| **Frapper** (poing, pied, tête), en visant haut ou bas | Contusion, coupure de l'arcade, fracture (nez, côtes, main de l'attaquant) ; un coup à la tête peut **étourdir** ou **assommer** | Force, puissance, coordination ; boxe ou autre sport de combat |
| **Pousser, bousculer** | Déséquilibre, **chute** (le plus grand danger réel d'une bagarre : la tête contre le sol, un escalier, l'eau) | Force, masse |
| **Saisir, immobiliser** | Contrôle de l'adversaire, mise au sol, clé de bras ; sert aussi à retenir quelqu'un jusqu'à l'arrivée de la police | Force, coordination ; lutte, techniques d'intervention de la police |
| **Se dégager** | Rompre une saisie | Force, coordination |
| **Parer, esquiver, se protéger** | Réduire ou éviter un coup ; se protéger la tête en se recroquevillant. C'est une **posture** que le personnage applique avec son temps de réaction (§1.2, A5) | Temps de réaction, coordination |
| **Désarmer** | Retirer une arme des mains de l'adversaire ; risqué face à un couteau | Coordination, compétence ; très peu probable sans entraînement |

- **Étourdissement et perte de connaissance** : un coup à la tête bien placé (ou une chute) peut provoquer un étourdissement de quelques secondes ou une perte de connaissance **[durées et probabilités à calibrer]**. Un personnage assommé est à la merci de son adversaire ; s'acharner sur lui change la qualification (§8.1) et la réaction des témoins.
- **Les poings tuent rarement** : la mort à mains nues vient surtout des **chutes** et des coups répétés à la tête d'une personne au sol.
- Il n'existe **pas** de geste d'étranglement (décidé). *Pourquoi : c'est un moyen de tuer silencieux qui banaliserait l'assassinat et serait pénible à représenter ; l'immobilisation suffit pour contrôler quelqu'un.*

### 1.4 Les armes blanches et les objets (décidé)

*Pourquoi : dans une ville européenne, l'arme d'un agresseur est bien plus souvent un couteau ou un objet saisi sur place qu'une arme à feu.*

- **Couteaux** : tout foyer en possède (cuisine), certains en portent un sur eux (canif, couteau de chasse, cran d'arrêt dans le milieu). Une **coupure** saigne ; une **plaie pénétrante** au tronc peut tuer en quelques minutes sans secours (§2.3). Porter un couteau en public sans motif légitime est un délit (§8.1).
- **Objets improvisés** : bouteille, chaise, batte, barre, outil, pierre. Tout objet du décor qui peut être saisi a une **masse** et une **forme** (contondant, coupant) qui en font une arme ; il garde les **empreintes** de celui qui l'a tenu (§6.3).
- **Lancer** un objet est possible ; la précision suit la coordination et la distance.
- **Moyens de défense** : l'aérosol lacrymogène et la matraque télescopique sont des **armes de catégorie réglementée** (§1.5), réservés à la police en service ; un civil qui en porte commet un délit.

### 1.5 Les armes à feu (décidé)

*Pourquoi : en Europe, les armes à feu sont rares et très encadrées ; les garder rares préserve à la fois le réalisme de la ville et le poids de chaque coup de feu.*

**Qui en possède** au départ **[nombres à calibrer]** :

| Détenteurs | Armes | Nombre indicatif | Remarque |
|---|---|---|---|
| **Police nationale** | Pistolet de service, quelques fusils à pompe et pistolets à impulsion électrique | Une arme par policier | Rangées au **commissariat** hors service (décidé : *pourquoi : cela évite des dizaines d'armes chez des particuliers et donne un vrai sens au vol d'une arme de service*) |
| **Police municipale** | Moyens non létaux (bâton, aérosol) ; **arme à feu sur décision du maire** | Selon la commune | Le maire peut décider d'armer sa police municipale (budget et vote du conseil, [social.md](social.md), §6.11) (décidé ; *pourquoi : c'est un vrai débat politique local en Europe, et un levier pour les élus*) |
| **Chasseurs** | Fusils et carabines de chasse, déclarés, avec permis | ≈ 80 détenteurs, surtout à Hautevigne et aux Forges | Rangés dans une armoire forte ; cible de cambriolage |
| **Armurerie** | Stock de fusils de chasse et de munitions | Petit stock | Vend sur présentation du permis ([economie.md](economie.md), §1.4) |
| **Milieu** | Pistolets illégaux, fusils volés ou à canon scié | ≈ 10 à 15 armes, surtout dans le réseau des Quais et chez les chefs de bande | Prix et circuits : [economie.md](economie.md), §7.3 |

**Règles** :

- **Pas d'armes automatiques ni d'explosifs** dans le monde (décidé). *Pourquoi : ils n'ont rien à faire dans une ville moyenne européenne, et ils permettraient des violences de masse incompatibles avec un monde partagé.*
- **Munitions comptées** : chaque arme a un chargeur et des munitions en nombre réel ; les munitions s'achètent à l'armurerie (permis) ou s'obtiennent par le milieu, en petite quantité.
- **Tirer** demande de **dégainer**, de **viser** (temps, stabilité) et de **tirer**. La précision dépend de la coordination, de la compétence de tir, de la distance, de l'éclairage et surtout du **stress** : sous stress, la plupart des tireurs, policiers compris, manquent leur cible à quelques mètres (décidé, d'après la réalité). Une balle manquée continue sa course (§12.2 pour les enfants). La **dispersion**, la **stabilisation** du réticule avec le temps de visée et le temps de réaction du personnage dominent la précision : un tir instantané après un mouvement brusque est pénalisé par le serveur, quelle que soit la précision de la souris, et l'aide à la visée de la manette est la même pour tous sans rien changer à la dispersion (décidé, S35 ; voir §1.2).
- **Le bruit porte loin** : un coup de feu s'entend dans tout le quartier **[portée à calibrer]**, déclenche les réflexes de tous ceux qui l'entendent (§1.8) et une vague d'appels à la police.
- **Braquer** une arme sur quelqu'un est un acte de menace d'une force extrême : presque tout le monde cède ou fuit (§1.8), sauf un personnage très violent, désespéré ou entraîné.
- **Armes génériques** (décidé, [direction-artistique.md](direction-artistique.md), §1.4) : pistolets, fusils de chasse, armes de service et armes blanches ont des formes génériques de leur catégorie, sans marque, modèle ni gravure d'un fabricant réel ; seules la catégorie et les caractéristiques mécaniques (calibre, capacité, portée) comptent dans le cœur. *Pourquoi : la ville est fictive, et une arme réelle reconnaissable ferait de la publicité à un fabricant ou exposerait le jeu à un litige.*
- **Permis et contrôle** : détenir une arme sans autorisation, la porter, la céder sont des délits (§8.1). Une arme a un **numéro** et peut porter un **signalement** comme un objet de valeur ([economie.md](economie.md), §5.10) ; une douille ou une balle retrouvée peut être rattachée à une arme saisie (§6.3).

### 1.6 Fatigue, stress, douleur et ivresse (décidé)

*Pourquoi : ce sont les états temporaires du socle ; ce sont eux, plus que la force brute, qui décident de l'issue d'un affrontement réel.*

| État | Effet en combat |
|---|---|
| **Fatigue d'effort** (endurance anaérobie) | Monte très vite : après 30 à 60 secondes d'effort intense, les coups faiblissent, la défense baisse, le souffle manque **[à calibrer]**. Un personnage endurant et entraîné tient plus longtemps |
| **Fatigue accumulée** (dette de sommeil) | Temps de réaction et perception dégradés ([04-joueur.md](../04-joueur.md), §5) |
| **Stress** | Courbe en cloche ([ia-pnj.md](ia-pnj.md), §1.3) : un peu améliore la réaction ; trop donne la **vision en tunnel** (perception réduite, ce qui rend aussi le témoin moins fiable, §4.3), les **tremblements** (précision fine et tir dégradés) et la panique (§1.8). L'entraînement (policiers, boxeurs, anciens militaires) déplace la courbe |
| **Douleur** | Réduit la performance de la zone blessée et la concentration. Pendant l'affrontement, l'**adrénaline** masque une partie de la douleur ; elle revient après, souvent d'un coup **[à calibrer]** |
| **Ivresse, drogues** | Temps de réaction et coordination en baisse, désinhibition (seuil de violence abaissé, [ia-pnj.md](ia-pnj.md), §1.4), douleur moins ressentie ; l'alcool est la première cause des bagarres de bar |
| **Froid, chaleur** | Le froid dégrade la dextérité (tir, crochetage) ; la chaleur réduit l'endurance |

### 1.7 Les intentions de combat (décidé)

*Pourquoi : une liste fermée d'intentions est ce que produisent à la fois le joueur et l'IA ; elle sert aussi à la synchronisation, puisque chaque intention reçoit une signature comportementale, calculée à partir de ses effets.*

| Famille | Intentions |
|---|---|
| **Posture** | Se mettre en garde, reculer, garder ses distances, se protéger, se recroqueviller |
| **Attaque** | Frapper (haut, bas), pousser, saisir, immobiliser, frapper avec un objet ou une arme blanche, lancer un objet, tirer |
| **Défense** | Posture défensive : garde (parer), esquive, se couvrir, reculer, appliquée par le personnage avec son temps de réaction (§1.2, A5) ; gestes ponctuels : se dégager, désarmer, s'interposer pour protéger quelqu'un |
| **Arme** | Dégainer, ranger, viser, recharger, poser à terre, jeter |
| **Issue** | Fuir, se rendre (lever les mains), appeler à l'aide, crier pour alerter, menacer, ordonner de s'arrêter (policier) |
| **Après** | Porter secours (§2.3), maintenir quelqu'un au sol, fouiller, ramasser, s'enfuir avec un objet |

- La **signature comportementale** d'une intention n'est pas écrite : elle est **dérivée de ses effets** (décidé, G1, G3), lus dans les données mécaniques de ce domaine et sur les **croyances** de l'acteur :
  - la **blessure attendue** d'après le modèle de blessure (§2.1) donne le tort physique (douleur, blessure légère, incapacité, grave, mortelle) ; pousser, saisir ou immobiliser sans blessure attendue donne un usage de la force ; menacer ou braquer une arme donne une **contrainte**, selon l'intensité de la menace ;
  - l'**infraction qualifiée** par le catalogue (§8.1) donne la violation de la loi ;
  - la **classe de la cible**, telle que l'acteur la croit (intime, inconnu, ennemi, personne vulnérable ou à terre), module la gravité (G4) ;
  - le **contexte** agit par les **justifications générales** de la revue ([revues/signatures.md](../revues/signatures.md), §3.6), en particulier la légitime défense et la défense d'autrui (§11.1). Frapper pour protéger son enfant n'a donc pas le coût de frapper un passant, sans aucune variante écrite.

  *Pourquoi : les données qui résolvent un coup (blessure attendue, qualification) disent déjà ce qu'il fait ; en tirer la signature garantit qu'elle reste cohérente avec le combat et la justice, et retire des centaines de nombres écrits à la main.* La gravité monte avec l'escalade (§11.2).
- **Fouiller** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), D31) : fouiller une personne ou la palper donne une **humiliation** de 0,1 et une **contrainte** de 0,1 envers la cible, par le dériveur D31, sans valeur écrite. Pour un policier qui fouille dans le cadre de ses pouvoirs (§5.5), la justification de **devoir de fonction** (J6) en neutralise l'essentiel. Fouiller un objet ou un lieu (sac abandonné, pièce) n'a pas de cible et ne porte que ses autres effets (prise, violation de domicile).
- La manière de saisir ces intentions (touches, roue, visée) relève du domaine interface.

### 1.8 Les réflexes (décidé)

*Pourquoi : [ia-pnj.md](ia-pnj.md), §3.1 confie à ce domaine le choix entre fuir, se défendre et se figer ; le relier au profil et à l'état fait du réflexe un révélateur du tempérament, comme prévu.*

Face à un **danger soudain** (agression, arme brandie, coup de feu, bagarre qui éclate près de soi), un personnage piloté par l'IA réagit d'abord par un réflexe, qui court-circuite la planification tant que la menace dure ([ia-pnj.md](ia-pnj.md), §3.1).

| Réflexe | Favorisé par |
|---|---|
| **Fuir** | Stabilité émotionnelle faible, violence faible, issue disponible, menace forte (arme), personnage physiquement faible ou âgé |
| **Se figer** | Stabilité émotionnelle très faible, surprise totale, absence d'issue, enfant jeune ; dure quelques secondes, puis bascule vers la fuite ou la défense |
| **Se défendre** | Violence élevée, force et compétence de combat, entraînement, colère, ivresse, absence d'issue |
| **Appeler à l'aide, crier** | Présence d'autres personnes, faible capacité à se défendre |
| **Protéger** | Un proche ou un enfant menacé : **réflexe de protection** ([ia-pnj.md](ia-pnj.md), §8.4), qui passe avant la sécurité propre chez les personnages bienveillants ; il prend la forme de s'interposer, d'emmener l'enfant, de le couvrir |
| **Se soumettre** | Menace écrasante (arme braquée), crainte forte : lever les mains, donner ce qui est exigé |

```
score d'un réflexe = base du profil × état (stress, fatigue, ivresse, blessures) × situation (issue, nombre, arme, proche menacé)
```

- Le réflexe choisi par l'IA d'un personnage **incarné** n'est pas imposé au joueur (décidé) : le joueur garde la main, mais il subit les **effets physiologiques** (sursaut d'une fraction de seconde, tremblements, vision en tunnel, essoufflement). L'IA en observation ([ia-pnj.md](ia-pnj.md), §5.1) calcule tout de même le réflexe qu'elle aurait eu : il sert à juger la synchronisation et l'imprudence (§11.3). *Pourquoi : retirer la main au joueur serait frustrant ; la synchronisation mesure déjà s'il agit comme le personnage.*
- Le seul état qui retire la main au joueur est la **perte de connaissance** (et l'étourdissement de quelques secondes).
- **Panique de foule** : un coup de feu ou une bagarre violente dans un lieu plein provoque des fuites en chaîne, des bousculades et des chutes, chacun réagissant à ce qu'il perçoit et à la fuite des autres.

### 1.9 Se défendre, intervenir, retenir (décidé)

*Pourquoi : ce sont les situations où un personnage ordinaire, ou un joueur, use de la force sans être l'agresseur ; le droit européen les reconnaît et la simulation doit les distinguer.*

- **Légitime défense** : riposter à une agression actuelle, de façon proportionnée, pour soi ou pour autrui. Elle est reconnue par la justice si le dossier le montre (témoins, blessures, contexte), ce qui n'est pas toujours le cas (§8.3).
- **Intervenir** : un témoin peut s'interposer, séparer, retenir un agresseur. C'est risqué, et c'est son profil qui le décide (§4.2).
- **Retenir l'auteur d'un flagrant délit** : n'importe qui peut retenir l'auteur d'un délit flagrant jusqu'à l'arrivée de la police (saisir, immobiliser, enfermer brièvement dans une pièce), sans le frapper au-delà du nécessaire (décidé, d'après le droit européen courant). *Pourquoi : cela donne un rôle aux vigiles, aux commerçants et aux passants courageux sans en faire des policiers.*

---

## 2. Blessures, santé et mort

### 2.1 Les blessures localisées (décidé)

*Pourquoi : la progression réaliste prévoit des blessures localisées (un bras blessé dégrade la visée) ; les relier aux zones et aux types donne des effets lisibles et des malus de mort conformes au tableau du socle.*

Chaque blessure est une entité attachée au personnage, décrite par :

| Champ | Valeurs |
|---|---|
| **Zone** | Tête, cou, torse, abdomen, bras gauche, bras droit, jambe gauche, jambe droite |
| **Type** | Contusion, coupure, plaie pénétrante, fracture, plaie par balle, brûlure, commotion, noyade ou asphyxie, intoxication |
| **Gravité** | Légère, moyenne, grave, critique |
| **Saignement** | Aucun, faible, fort, massif |
| **Douleur** | Selon le type et la gravité |
| **Soins reçus** | Aucun, premiers secours, soins médicaux, chirurgie |
| **Origine** | Cause, date, auteur s'il est connu du cœur : sert au constat médical, à l'enquête et au malus en cas de mort |

**Effets localisés** **[ampleurs à calibrer]** :

| Zone | Effets |
|---|---|
| Tête | Étourdissement, perte de connaissance, perception et mémoire dégradées (le souvenir de l'agression peut être flou, §4.3) |
| Bras, main | Force et coordination du membre : frapper, viser, tirer, crocheter, conduire, porter |
| Jambe | Mobilité : marcher, courir, fuir, monter un escalier ; une fracture de jambe immobilise |
| Torse, abdomen | Endurance, souffle ; risque vital en cas de plaie pénétrante ou par balle |

Les blessures **se voient** : boiterie, bras en écharpe, pansements, ecchymoses, vêtements tachés. Elles ont les effets sociaux du domaine social (questions, inquiétude, méfiance, [social.md](social.md), §5.5) et constituent des **indices** (§6.3).

### 2.2 Le lien avec les points de vie réels (décidé)

*Pourquoi : le socle définit une barre de points de vie réels et la mort à zéro ; il faut une règle unique qui la relie aux blessures, aux maladies et aux saignements.*

Les **points de vie réels** (0 à 100) mesurent la santé globale ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §1). Ils sont calculés à partir de l'état du corps :

```
PV réels = 100 − Σ (atteinte vitale de chaque blessure) − perte de sang − atteinte des maladies et intoxications
```

- **Atteinte vitale** : fixée par la zone, le type et la gravité **[barème à calibrer]**. Ordres de grandeur : contusion légère 1 à 3 ; coupure moyenne 5 à 10 ; fracture 10 à 20 ; plaie pénétrante à l'abdomen 25 à 40 ; balle dans le torse 40 à 70.
- **Perte de sang** : elle augmente tant que le saignement n'est pas arrêté : un saignement faible s'arrête seul ; fort, il retire ≈ 1 à 2 PV par minute ; massif, ≈ 5 PV par minute **[à calibrer]**.
- **Seuils** **[à calibrer]** :

  | PV réels | État |
  |---|---|
  | Moins de 50 | Douleur forte, performance nettement réduite |
  | Moins de 25 | État de **choc** : risque de perte de connaissance, le personnage ne peut plus courir |
  | 10 ou moins | **État critique** : inconscient, il meurt en quelques minutes sans soins |
  | 0 | **Mort** |

- La **barre du joueur** suit la formule du socle : `PV du joueur = PV réels × S`. Une blessure fait baisser les deux barres ; un joueur déjà désynchronisé est donc éjecté plus tôt quand son personnage est blessé, et l'IA reprend dans le même tick avec ses réflexes ([ia-pnj.md](ia-pnj.md), §5.3).
- Un personnage en état critique, inconscient ou hospitalisé en état grave **n'est pas jouable** (socle, [02-personnages.md](../02-personnages.md), §3.4).

### 2.3 Premiers secours et urgences (décidé)

*Pourquoi : entre la blessure et la mort, il y a les minutes pendant lesquelles quelqu'un peut agir ; c'est là que se jouent la plupart des issues réelles, et un rôle précieux pour les joueurs.*

- **Premiers secours** : n'importe qui peut comprimer une plaie, faire un garrot, mettre en position latérale, rester auprès de la victime. L'efficacité dépend de la compétence de **secourisme** (pompiers, soignants, policiers, sauveteurs, certains habitants formés). Un saignement fort ou massif peut être **réduit** ou **arrêté**.
- **Appeler les secours** : un appel (téléphone) déclenche l'envoi d'une équipe des **pompiers** (caserne de la rive sud, [ville.md](ville.md), §5.2) et, si c'est grave, d'une équipe médicale de l'**hôpital** de Beaulieu. Ces équipes sont des habitants en service, qui viennent par la route : leur **délai réel** dépend de la distance, du trafic, de la météo et de leur disponibilité.
- **À l'hôpital** : urgences, bloc opératoire, réanimation. Les chances de survie dépendent du délai et de la gravité ; une plaie par balle au torse est souvent mortelle si l'on arrive après la première heure **[à calibrer]**.
- **Constat** : tout blessé par violence soigné à l'hôpital fait l'objet d'un constat ; une blessure par arme est **signalée à la police** par l'hôpital (décidé). *Pourquoi : c'est la pratique réelle, et une source d'affaires indépendante de la plainte de la victime.*

### 2.4 Guérison et séquelles (décidé)

*Pourquoi : des durées réalistes donnent leur poids aux blessures, mais le personnage persistant en ligne ne doit pas être immobilisé en temps réel, comme le socle l'a déjà décidé pour la mort.*

- **Durées** réalistes **[à calibrer]** : une contusion guérit en quelques jours, une coupure en une à deux semaines, une fracture en 6 à 12 semaines, une plaie grave opérée en plusieurs semaines de convalescence. Pendant ce temps, les effets localisés diminuent progressivement ; le désentraînement s'applique ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §3.3).
- **Séquelles** : une blessure grave peut laisser une séquelle permanente légère (cicatrice visible, douleur chronique, perte d'une capacité), comme une mort répétée.
- **Cicatrices** (décidé, [direction-artistique.md](direction-artistique.md), §3.2) : à la guérison d'une blessure **grave ou critique** de type coupure, plaie pénétrante, plaie par balle ou brûlure, une **cicatrice** peut s'inscrire dans les **signes distinctifs** du génome d'apparence, avec sa zone, sa taille (petite, moyenne, grande) et sa visibilité (visage, cou et mains toujours visibles ; ailleurs, visible selon la tenue) **[probabilités à calibrer : ≈ 30 % pour une blessure grave, ≈ 70 % pour une blessure critique, plus si elle a été soignée tard]**. Une cicatrice est **permanente**, et elle est tirée par l'aléa à clé de la blessure, donc reproductible. Le **personnage persistant** reçoit ses cicatrices comme les autres, y compris quand une blessure grave est traitée comme la mort du socle (ci-dessous). Une cicatrice visible est un **signe distinctif** que les témoins retiennent (§4.3) et que la police peut noter dans un signalement (§6.4) ; elle a aussi les effets sociaux d'un signe distinctif ([social.md](social.md), §5.5). Certaines cicatrices sont présentes **au départ**, inscrites dans la biographie (agression, accident, opération). *Pourquoi : la séquelle visible existait déjà ici, et la direction artistique lui offre une place dans le génome ; c'est elle qui donne aux blessures une trace durable, utile à l'enquête et lisible par les autres, sans aucun asset propre à un joueur.*
- **Personnages du monde** : ils guérissent avec le temps du monde, en temps réel en ligne ; un blessé grave est hospitalisé, absent de son travail (arrêt maladie, [economie.md](economie.md), §3.6), visité par ses proches.
- **Personnage persistant** (décidé) :
  - **blessures légères et moyennes** : elles guérissent avec le temps réel écoulé, qu'il soit connecté ou non (il se soigne pendant ses absences), avec leurs effets tant qu'elles durent ;
  - **en ligne, blessure grave** (hospitalisation nécessaire) : elle est traitée comme la mort du socle ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §3.2) : il sort de l'hôpital immédiatement, blessures soignées, avec une baisse d'attributs équivalente à la convalescence théorique, mais **sans** malus psychologique de mort et **sans** compter comme une mort pour la règle des morts répétées ;
  - **en solo**, la convalescence se déroule normalement, et le joueur peut faire avancer le temps.

  *Pourquoi : c'est l'application directe de la règle de convalescence du socle à un cas qu'elle ne couvrait pas ; sans elle, une fracture immobiliserait le personnage persistant deux mois réels.*

### 2.5 La mort violente (décidé)

*Pourquoi : la mort violente doit avoir toutes les suites d'une vraie mort (constat, enquête, deuil, succession), et rejoindre les règles du socle pour les deux types de personnages.*

- **Causes** : les types de blessure se rattachent aux causes du socle ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §3.1) : contusions et commotions → « coups, agression » ; coupures et plaies pénétrantes → « arme blanche » ; plaies par balle → « arme à feu » ; chute → « chute » ; incendie → « brûlure » ; noyade dans l'Ostrelle → « noyade » ; overdose, produit dangereux → « intoxication ». Quand plusieurs blessures ont contribué, la cause retenue est celle de la plus forte atteinte vitale.
- **Personnage jouable** : mort **définitive** (socle). Suites : constat par un médecin, **enquête** systématique (§6.1), corps à la morgue de l'hôpital, obsèques, deuil des proches, succession ([economie.md](economie.md), §5.11), familiarités devenues inutiles ([population.md](population.md), §7.3).
- **Personnage persistant** : réveil à l'hôpital public ou à la clinique privée, avec le malus de la cause (socle). Dans le monde, il est **gravement blessé, réanimé et soigné**, jamais « mort » : les témoins l'ont vu tomber, les soignants l'ont « sauvé » (décidé). *Pourquoi : un habitant mort qui revient le lendemain briserait la cohérence du monde ; une réanimation réussie est crédible et laisse intactes l'enquête et la plainte.* Ce qu'il portait peut avoir été volé ou saisi comme pièce à conviction (socle, §3.3).
- **Les morts violentes comptent** dans la mortalité totale du régulateur démographique ([population.md](population.md), §7.3 et §7.6).

### 2.6 Personnes vulnérables (décidé)

*Pourquoi : la même violence n'a pas les mêmes effets sur tous, et la loi protège davantage certaines victimes.*

- **Âge** : un personnage âgé a une **vitalité de fond** plus basse ([population.md](population.md), §4.4) ; une chute peut le tuer.
- **Grossesse** : les violences contre une femme enceinte sont des **violences aggravées** (§8.1), avec une peine plus lourde et une réprobation sociale très forte ; **aucune fausse couche** ne résulte des actions des personnages ([population.md](population.md), §7.2).
- **Enfants** : voir les limites du §12.2.
- **Handicap, maladie** : la vulnérabilité connue de la victime aggrave la qualification.

---

## 3. Effractions, vols et dégâts matériels

### 3.1 Portes, serrures et vitres (décidé)

*Pourquoi : la ville règle l'accès par des droits et des portes ([ville.md](ville.md), §5.1) et confie à ce domaine le crochetage et l'effraction ; chaque méthode doit avoir son coût et sa trace.*

| Méthode | Ce qu'il faut | Durée **[à calibrer]** | Bruit | Trace |
|---|---|---|---|---|
| **Utiliser une clé** | La clé, ou un double | Instantané | Aucun | Aucune |
| **Crocheter** | Outils et compétence de **crochetage** ; serrure plus ou moins difficile (simple, multipoints, de sûreté) | 30 s à 10 min | Faible | Rayures que seul un spécialiste remarque |
| **Forcer** | Outil (pied-de-biche) et force | 10 s à 2 min | Fort | Porte ou serrure **endommagée** (dégât persistant, [ville.md](ville.md), §6.3) |
| **Briser une vitre** | Un objet | Instantané | Très fort | Vitre **détruite** |
| **Entrer par ruse** | Se faire ouvrir (faux livreur, faux agent), suivre un résident | Variable | Aucun | Témoin qui se souvient |

- Crocheter est une tâche délicate : le personnage n'est **pas au repos** ([04-joueur.md](../04-joueur.md), §5) ; le froid et le stress dégradent la dextérité.
- **Coffres** : un coffre s'ouvre avec son code (**savoir opaque**, [ia-pnj.md](ia-pnj.md), §4.4), ou se force avec du matériel lourd, longtemps et bruyamment. Le joueur ne voit jamais le code.
- **Alarmes** : les lieux qui en ont (banques, bijouterie, armurerie, commerces aisés, villas de Hautevigne, entrepôts surveillés) déclenchent une sirène et un appel à la société de surveillance ou à la police. La ville en donne la présence dans la donnée **surveillance** de chaque lieu ([ville.md](ville.md), §9.1).
- **Tirage par monde** (décidé, P12) : la carte publique ne contient que des **candidats** (classe de chaque porte, emplacements possibles d'un coffre, lieux équipables d'une alarme). Ce qui est réellement posé est **tiré à la création de chaque monde** dans les plages ci-dessous **[toutes à calibrer]** ; un joueur qui connaît son monde solo n'apprend donc rien des serrures et des coffres d'un serveur.

  | Classe de porte (ville) | Ménage ou établissement modeste | Moyen | Aisé, ou commerce de valeur |
  |---|---|---|---|
  | **Porte palière, d'entrée de maison** | Simple 70 %, multipoints 30 % | Simple 35 %, multipoints 55 %, de sûreté 10 % | Multipoints 40 %, de sûreté 60 % |
  | **Porte de service, arrière de commerce** | Simple 60 %, multipoints 40 % | Simple 30 %, multipoints 60 %, de sûreté 10 % | Multipoints 30 %, de sûreté 70 % |
  | **Vitrine** | Verre simple et rideau absent 50 %, rideau métallique 50 % | Rideau métallique 85 %, vitrage renforcé 15 % | Vitrage renforcé et rideau métallique 100 % |
  | **Cave, local commun** | Simple 85 %, cadenas 15 % | Simple 70 %, multipoints 30 % | Multipoints 80 %, de sûreté 20 % |

  - Le **quartier** décale ces tirages d'une classe vers le haut ou vers le bas selon la sécurité perçue et l'histoire des cambriolages ([ville.md](ville.md), §9.1) ; une porte déjà forcée et réparée est remplacée par un niveau supérieur dans 50 % des cas.
  - **Coffres** : jamais dans un logement modeste ; dans 10 à 20 % des logements aisés (≈ 40 à 60 % à Hautevigne) ; dans tous les établissements qui manient des espèces ou des objets de valeur (banque, bijouterie, armurerie, bureau de tabac, pharmacie, prêteur sur gages) et dans 30 à 50 % des autres commerces. Coffre **léger** (forçable en 20 à 40 min) ou **lourd** (matériel lourd, plus d'une heure), selon la richesse.
  - **Caméras privées** : présentes dans tous les établissements à coffre obligatoire, dans 40 à 60 % des autres commerces de la Cité et de la Gare, 15 à 30 % ailleurs, 5 à 10 % des halls d'immeuble ; 1 à 3 caméras par lieu, posées sur ses points candidats ; conservation des images de 3 à 30 jours.
  - **Alarmes** : tous les établissements à coffre obligatoire ; 20 à 40 % des autres commerces ; 30 à 50 % des villas de Hautevigne, 5 à 10 % des autres maisons ; chaque alarme est **reliée** (société de surveillance ou police) ou seulement **sonore**, à parts égales hors banques et bijouteries, toujours reliées.

  *Pourquoi : le solo embarque le serveur ; une part serveur identique dans tous les mondes serait lue par tous, et la protection des secrets de la ville ne tiendrait pas.*

### 3.2 Vidéoprotection et surveillance (décidé)

*Pourquoi : les caméras sont l'indice le plus fréquent des enquêtes contemporaines ; il faut savoir où elles sont et ce qu'elles valent.*

- **Caméras privées** : banques, commerces, parkings, entrepôts, quelques immeubles ; leurs images sont conservées quelques jours **[à calibrer]** puis effacées.
- **Vidéoprotection municipale** : sur les places, aux abords de la gare et des pontons, sur les grands axes de la Cité, selon le budget voté par chaque commune ([economie.md](economie.md), §8.4) ; images consultables par la police.
- L'emplacement des caméras privées est **tiré par monde** (§3.1, P12) ; la vidéoprotection municipale, votée et publique, ne l'est pas.
- Une caméra enregistre ce qui se passe **dans son champ**, calculé sur la géométrie étagée (§1.2, P13) : la qualité de l'image dépend de l'éclairage, de la distance et de la météo. Un visage masqué ou une capuche réduit l'identification ; une silhouette, une démarche, un véhicule, une heure restent exploitables.
- Les **gardiens et agents de sécurité** (usine, port, parking, hôpital) font des rondes ([economie.md](economie.md), §3.3), voient et interviennent selon leur fiche de rôle.
- **Filmer avec son téléphone** est un acte possible pour un témoin (§4.2) : la vidéo devient un indice comme une image de caméra.

### 3.3 Vols de véhicules (décidé)

*Pourquoi : la ville et l'économie ont prévu des véhicules ouverts avec une clé et pouvant être volés ; il faut dire comment et à quel risque.*

- **Voler** : forcer la portière (bruit, dégât), puis démarrer sans clé (compétence, et outils pour les véhicules récents, beaucoup plus difficiles) ; ou voler les **clés** (cambriolage, vestiaire, voiturier distrait). Un véhicule **non verrouillé, clé dessus** se vole en un instant.
- **Ce que devient le véhicule** : il est utilisé puis abandonné (souvent brûlé), revendu tel quel, **maquillé** ou **démonté** dans un garage complice ([economie.md](economie.md), §7.3 et §7.4 ; bande des Forges, [population.md](population.md), §5.5).
- **Signalement** : le propriétaire porte plainte ; le véhicule est **signalé** au registre. Une patrouille qui le croise le reconnaît par sa plaque si elle la contrôle ; un contrôle routier le révèle.

### 3.4 Dégâts, incendies et tags (décidé)

*Pourquoi : les dégâts sont persistants jusqu'à réparation ([ville.md](ville.md), §6.3) et coûtent de l'argent ([economie.md](economie.md), §5.8) ; ce domaine en fixe les causes violentes et les suites pénales.*

- **Dégradations** : casser une vitrine, une porte, un lampadaire, rayer une voiture, renverser du mobilier urbain. Chaque dégât passe à l'état endommagé ou détruit et reste en place jusqu'à réparation.
- **Tags** : marqueurs de territoire des bandes ([social.md](social.md), §6.6) ou simple vandalisme ; signés, ils indiquent l'auteur à qui sait les lire.
- **Incendie volontaire** : allumer un feu demande une source (briquet, carburant). Le feu se **propage dans le local** ([ville.md](ville.md), §6.3), produit de la fumée qui intoxique, déclenche l'intervention des pompiers et, s'il y a doute, une **enquête** (départ de feu, accélérant). Un incendie criminel est un délit grave, un crime s'il y avait des personnes à l'intérieur.
- **Accidents** : les dégâts accidentels (collision, maladresse) ne sont pas des délits, sauf faute (conduite en état d'ivresse, délit de fuite).
- **Traces temporaires** : sang, verre brisé, douilles, objets tombés restent sur place jusqu'à leur nettoyage par les agents d'entretien ou le propriétaire, ou leur prélèvement par la police ; une **scène d'enquête** est bouclée (ruban) pendant les constatations **[durée à calibrer]**.

---

## 4. Témoins et victimes

### 4.1 Qui voit, qui entend (décidé)

*Pourquoi : la perception filtrée de [ia-pnj.md](ia-pnj.md), §4.1 est la condition pour que les témoins aient un sens ; il suffit de préciser ce que la violence rend perceptible.*

- **Voir** : selon la ligne de vue, la distance, l'éclairage, la météo, l'attention (occupé ou non) et l'attribut de perception. On voit à travers une fenêtre, dans une cage d'escalier, depuis un balcon. La ligne de vue est calculée sur la géométrie étagée (§1.2, P13).
- **Entendre** : cris, bris de verre, coups contre une porte, à travers les murs d'un immeuble ; un coup de feu dans tout le quartier. La pluie et le bruit (bar, chantier, circulation) couvrent les sons.
- **Ce qui est retenu** : un **souvenir** structuré (qui, quoi, où, quand, émotion) et des **croyances** : « X a frappé Y », ou, si l'auteur est inconnu, une **description** (§4.3).

### 4.2 Réactions immédiates des témoins (décidé)

*Pourquoi : [ia-pnj.md](ia-pnj.md), §11 demande que les témoins appellent la police, interviennent, fuient ou se taisent selon leur respect des règles et leur bienveillance ; ce tableau en fait une règle unique, qui dépend aussi de la peur et du nombre de témoins.*

Chaque témoin d'une violence choisit sa réaction, d'abord par réflexe (§1.8), puis par la sélection d'objectif :

| Réaction | Favorisée par |
|---|---|
| **Fuir, s'éloigner** | Crainte, stabilité émotionnelle faible, arme visible, présence d'un enfant à protéger |
| **Se figer, regarder** | Surprise, curiosité, distance jugée sûre |
| **Appeler la police ou les secours** | Respect des règles, bienveillance, téléphone en main, distance sûre ; **c'est la réaction la plus fréquente** |
| **Intervenir** (s'interposer, séparer, crier « arrêtez ») | Bienveillance élevée, violence moyenne ou élevée, force, lien avec la victime, cohésion du quartier ([social.md](social.md), §5.3), agresseur seul et sans arme |
| **Secourir la victime** (après) | Bienveillance, compétence de secourisme |
| **Filmer** | Curiosité, téléphone, distance sûre |
| **Encourager, se joindre à l'agresseur** | Appartenance au même groupe, hostilité envers la victime, violence élevée, ivresse |
| **Se taire et partir** | Crainte de l'agresseur ou de sa bande (**loi du silence**, [social.md](social.md), §6.6), respect des règles faible, méfiance envers la police |

- **Effet du nombre** (décidé) : plus il y a de témoins, moins chacun intervient personnellement, mais plus il est probable que l'un d'eux appelle la police. *Pourquoi : c'est un effet bien établi, et il évite des foules qui se jettent toutes sur un agresseur.*
- **Témoin d'un acte inhabituel** : il juge l'acte d'après l'idée qu'il se fait de l'auteur ([social.md](social.md), §5.4) : un proche cherche une explication, un inconnu conclut « c'est un violent ».
- **Effets durables** : le témoin met à jour sa relation avec l'auteur (crainte en hausse, affection en baisse, respect selon ses valeurs, [social.md](social.md), §1.4), peut en parler (rumeur), et la **sécurité perçue** du lieu baisse ([ville.md](ville.md), §9.1).
- **Anomalies** (décidé, [ia-pnj.md](ia-pnj.md) §13.8) : un personnage n'a pas besoin de **voir** la violence pour réagir. Les écarts à ses attentes sur un lieu familier, quand ils relèvent de ce domaine, sont des anomalies : porte forcée, vitre brisée, inconnu dans son logement, cri, coup de feu, fumée, corps au sol, sang, douille, ruban de scène, objet disparu, proche qui ne rentre pas, voiture qui le suit. Leurs réponses (regarder, s'enquérir, s'inquiéter, alerter, fuir) passent par les **recettes de ce tableau** : « alerter » est l'appel à la police ou aux secours (§5.4), « regarder » peut mener à intervenir ou à secourir, « fuir » suit le réflexe (§1.8). Plus l'anomalie est grave, plus l'appel est probable et rapide. C'est ainsi qu'un **cambriolage** est découvert au retour (§10.5) et qu'un voisin appelle pour une porte entrouverte la nuit. *Pourquoi : la plupart des crimes réels sont signalés par quelqu'un qui remarque que quelque chose cloche, pas par un témoin direct.*
- **Attroupements** (décidé, [ia-pnj.md](ia-pnj.md) §13.9) : un incident visible (bagarre terminée, blessé, accident, arrestation) attire des **curieux** selon leur profil. Ils aident, filment, commentent et propagent la nouvelle ; ils deviennent des **témoins** à entendre (§6.4) et leurs vidéos des **images** (§6.3), mais ils **piétinent la scène**, qui perd des traces (§6.3), et ils gênent les secours. La **panique de foule** (§1.8) reste prioritaire dès qu'une menace est perçue : un attroupement se disperse de lui-même si l'agresseur revient ou si une arme réapparaît. La police le disperse à son arrivée (§5.3). *Pourquoi : un incident sans curieux paraît vide, et l'attroupement a des effets réels sur l'enquête, dans les deux sens.*
- **« Ce lieu est dangereux »** (décidé, [ia-pnj.md](ia-pnj.md) §13.4) : un personnage agressé, menacé ou effrayé dans un lieu, comme victime ou comme témoin, peut en tirer pendant sa réflexion nocturne la conclusion « ce lieu est dangereux ». Elle nourrit son **besoin de sécurité** : il évite le lieu ou ne s'y rend plus seul ni le soir, change d'itinéraire, demande à être raccompagné, s'équipe (serrure, alarme, [economie.md](economie.md)), signale le lieu à la police municipale ou à son élu, et, si la conclusion devient un thème durable, envisage de déménager. La conclusion peut être fausse ou exagérée selon le profil (un anxieux conclut plus vite) et se corrige avec le temps. *Pourquoi : c'est par ces évitements que la violence d'un quartier se lit dans la vie de ses habitants, sans aucun taux imposé.*

### 4.3 Le souvenir d'un témoin (décidé)

*Pourquoi : un témoignage n'a de valeur pour l'enquête que s'il peut être imprécis, faux ou oublié, comme un vrai.*

Un témoin qui ne connaît pas l'auteur garde une **description** faite de traits structurés : sexe apparent, âge approximatif, taille, corpulence, vêtements, signe distinctif, véhicule, direction de fuite, arme.

```
précision de chaque trait = perception × attention × durée d'observation × éclairage × (1 − effet du stress) × (1 − distance)
```

- **Stress** : un témoin terrifié retient l'arme plutôt que le visage (focalisation sur l'arme) ; la vision en tunnel efface les détails.
- **Connaissance de l'auteur** : un témoin qui le **reconnaît** (de vue ou de nom, [social.md](social.md), §1.1) le désigne directement, avec une certitude élevée, mais il peut se tromper de personne si l'éclairage était mauvais.
- **Oubli et déformation** : le souvenir s'efface et se déforme avec le temps ([ia-pnj.md](ia-pnj.md), §4.3) et à chaque fois qu'il est raconté ([social.md](social.md), §4.3) ; une rumeur entendue peut contaminer le souvenir (on finit par « se souvenir » de ce qu'on a entendu dire).
- **Masques et déguisements** (décidé, [direction-artistique.md](direction-artistique.md), §3.5) : le témoin ne retient que ce qu'il a **perçu**, et l'apparence perçue a déjà perdu les champs que masque l'auteur. Ce domaine fixe l'effet sur le souvenir **[ampleurs à calibrer]** :

  | Ce qui masque | Champs perdus | Effet sur le souvenir |
  |---|---|---|
  | **Cagoule**, casque intégral, écharpe montée avec capuche | Visage, cheveux, signes distinctifs du visage | Aucune reconnaissance par le visage, même d'un proche ; restent la silhouette, la voix, l'allure, les vêtements et les signes distinctifs visibles ailleurs (mains, cou) |
  | **Capuche** ou casquette baissée seule, lunettes noires | Cheveux ; visage en partie | Précision du visage divisée par deux environ ; un proche reconnaît encore souvent l'auteur à son allure et à ses traits visibles |
  | **Déguisement** (perruque, tenue d'emprunt, faux uniforme) | Aucun : il remplace des champs | Le témoin retient **le déguisement** comme une vraie description : il décrit de bonne foi une perruque blonde ou un uniforme, ce qui égare l'enquête jusqu'à ce que le déguisement soit retrouvé (§6.3) |

  Un auteur masqué reste **reconnaissable par sa voix** pour qui la connaît bien, et le masque lui-même est un **indice** et une circonstance aggravante (§8.1). Porter une cagoule dans la rue, hors du froid, est une **anomalie** qui attire l'attention et peut motiver un appel (§4.2). *Pourquoi : le serveur décide de ce qui se voit, ce domaine décide de ce qu'on en retient ; ensemble, ils rendent le masque utile au criminel sans le rendre infaillible.*
- **Reconnaissance** : présenté à une photo, à une personne ou à un alignement (§6.4), le témoin reconnaît l'auteur avec une probabilité qui dépend de la précision de son souvenir ; il peut aussi désigner à tort quelqu'un qui ressemble à sa description.

### 4.4 Les enfants témoins et victimes (décidé)

*Pourquoi : [ia-pnj.md](ia-pnj.md), §8.3 et [social.md](social.md), §7.2 laissent à ce domaine le poids du témoignage d'un enfant devant la police et la justice.*

- Les enfants **perçoivent et retiennent** comme les adultes, avec une précision qui dépend de l'âge ; les plus jeunes **déforment** davantage (§4.3).
- **Réactions** : un enfant face à la violence a peur, pleure, se fige ou court vers un adulte de confiance ; il le raconte ensuite à ses parents, à ses amis, à son enseignant, ce qui peut lancer une rumeur ou un signalement.
- **Poids du témoignage** : la police et la justice reprennent les **coefficients de crédibilité** du social (≈ 0,3 avant 6 ans, 0,5 de 6 à 10 ans, 0,75 de 11 à 14 ans, 0,9 de 15 à 17 ans **[à calibrer]**). Une audition menée par un enquêteur formé (compétence d'audition des mineurs) réduit les déformations de l'entretien, pas celles du souvenir.
- **Règle de preuve** : le témoignage d'un enfant **suffit à ouvrir une enquête**, mais **ne suffit pas seul à condamner** pour un délit ou un crime : il faut un autre élément (indice, autre témoin, aveu) (décidé). *Pourquoi : c'est conforme à la prudence judiciaire réelle, et cela évite qu'un seul récit d'enfant, peut-être déformé, décide du sort d'un adulte.*
- **Effets sur l'enfant** : une violence vue ou subie laisse un souvenir marquant qui déplace son profil ([ia-pnj.md](ia-pnj.md), §8.5), peut faire baisser ses résultats scolaires, inquiéter ses parents et donner lieu à un suivi (services sociaux, [social.md](social.md), §7.3).

### 4.5 Les victimes : porter plainte ou non (décidé)

*Pourquoi : beaucoup de victimes réelles ne portent pas plainte ; ce « chiffre noir » doit venir de leurs raisons, pas d'un taux.*

Une victime (ou ses proches) décide de **porter plainte** au commissariat ou au poste de police municipale selon :

| Pousse à porter plainte | Retient |
|---|---|
| Respect des règles, confiance dans la police | Crainte de l'auteur ou de sa bande, loi du silence |
| Gravité du préjudice | Honte (usure, escroquerie, [social.md](social.md), §6.6) |
| Besoin d'une plainte pour l'**assurance** (cambriolage, vol de véhicule, [economie.md](economie.md), §5.8) | Appartenance au milieu (« on règle ça entre nous ») |
| Soutien des proches | Lien avec l'auteur (famille, conjoint, ami) |
| Espoir de récupérer son bien | Petit préjudice, sentiment que « ça ne sert à rien » |

- Une victime qui ne porte pas plainte peut **se venger** (projet de vengeance, [social.md](social.md), §1.5), demander la protection d'une bande rivale ou d'un proche, ou simplement éviter le lieu (conclusion « ce lieu est dangereux », §4.2).
- La plainte crée une **affaire** (§6.1) et une **croyance** de la police.
- **Plainte mensongère** : déclarer un faux vol (fraude à l'assurance) ou accuser à tort est un délit (dénonciation calomnieuse, §8.1) ; elle crée une affaire comme une vraie, que l'enquête peut démasquer.

---

## 5. La police

### 5.1 Organisation (décidé)

*Pourquoi : la ville a fixé un commissariat central pour toute l'agglomération et un poste de police municipale par commune, sans gendarmerie ([ville.md](ville.md), §5.2) ; ce domaine confirme cette organisation et la peuple.*

L'organisation de la ville est **confirmée** : **police nationale** au commissariat central, compétente sur toute l'agglomération ; **police municipale** dans chaque commune ; pas de gendarmerie.

**Police nationale** (≈ 36 personnes **[à calibrer]**), financée par l'État ([economie.md](economie.md), §8.5) :

| Service | Effectif | Rôle |
|---|---|---|
| **Commissaire central** | 1 (hors catégorie, [population.md](population.md), §6.2) | Dirige ; fixe les priorités ; rend compte au procureur et au préfet (à l'extérieur) |
| **Commissaire adjoint** | 1 (palier ≈ 6) | Seconde ; suppléant désigné ([social.md](social.md), §6.8) |
| **Brigades de roulement** (police secours) | 4 brigades de 5, dont un **chef de poste** | Accueil, appels d'urgence, patrouilles en voiture, interventions, garde à vue ; 24 h sur 24 en roulement |
| **Brigade anticriminalité** | 4 | Patrouilles en civil, surtout le soir et la nuit, flagrants délits |
| **Sûreté** (enquêteurs) | 7, dont un **technicien de police scientifique** et deux enquêteurs spécialisés (stupéfiants, affaires financières) | Enquêtes, auditions, gardes à vue, perquisitions |
| **Soutien** | 2 | Accueil administratif, plaintes simples, dossiers |

**Polices municipales** (≈ 17 personnes **[à calibrer]**), financées par chaque commune ([economie.md](economie.md), §8.4) :

| Commune | Effectif | Particularité |
|---|---|---|
| Orvenne | 8 policiers municipaux, 2 agents de surveillance de la voie publique (stationnement) | Centre, gare, marché, fourrière |
| Forges-sur-Ostrelle | 4 | Zone commerciale, abords de la halte |
| Hautevigne | 3 | Rondes dans les quartiers résidentiels, surveillance des villas en vacances |

- **Compétences** : la police nationale a tous les pouvoirs d'enquête et d'interpellation ; la police municipale veille à la tranquillité publique, au stationnement, à la circulation, constate les contraventions, intervient en flagrant délit et remet les auteurs à la police nationale, mais n'enquête pas (décidé, d'après le modèle européen courant). *Pourquoi : deux polices aux rôles distincts donnent aux maires un vrai levier (effectifs, armement, priorités) sans doubler l'enquête.*
- **Uniformes et insignes fictifs** (décidé, [direction-artistique.md](direction-artistique.md), §1.4) : la police nationale, les polices municipales, les pompiers, les secours et la justice (robes des magistrats et des avocats) ont des tenues **crédibles d'Europe de l'Ouest**, mais des écussons, armoiries, grades et inscriptions **inventés**, sans drapeau ni emblème d'un État réel. Les véhicules de police et de secours suivent la même règle. L'uniforme et le véhicule sérigraphié **se voient** : ils font reconnaître un policier en tenue, alors que la brigade anticriminalité et les enquêteurs en civil ne se reconnaissent qu'à leur brassard, sorti au moment d'agir. *Pourquoi : la reconnaissance d'un policier compte pour les témoins, les suspects et le joueur ; l'apparence doit la porter sans emprunter à un État réel.*
- **La nuit** : à 3 h du matin, ≈ 6 à 8 policiers sont en service (une brigade de roulement, parfois la brigade anticriminalité) **[à calibrer]**, ce qui rend la police moins présente et plus lente, comme en vrai.
- **Renforts** : en cas d'événement exceptionnel (émeute, fusillade), le commissaire peut rappeler les agents de repos ; les renforts extérieurs n'existent pas (aucun figurant). Si la criminalité de l'agglomération augmente durablement, l'État augmente la **dotation** de postes, pourvue par de vraies **arrivées** ([population.md](population.md), §7.4) (décidé). *Pourquoi : c'est la seule façon de renforcer la police sans faire apparaître de figurants, et elle prend le temps d'un vrai recrutement.*

### 5.2 Les autres services de l'État et de la justice (décidé)

*Pourquoi : l'économie demande les fiches d'inspecteur du travail, de contrôleur fiscal, d'huissier, de juge et de mandataire ([economie.md](economie.md), §14.6) ; la règle « aucun figurant » oblige à ce qu'ils soient des habitants.*

| Service | Effectif **[à calibrer]** | Lieu |
|---|---|---|
| **Tribunal** : président, juges (dont juge d'instruction, juge des enfants, juge de l'application des peines), procureur (hors catégorie) et deux substituts, greffiers | ≈ 13 | Tribunal de la Cité |
| **Service pénitentiaire d'insertion et de probation** : un conseiller | 1 | Bureau au tribunal |
| **Tribunal de commerce** : juges consulaires élus parmi les commerçants et chefs d'entreprise | 3, bénévoles, en plus de leur métier | Tribunal de la Cité |
| **Commissaires de justice** (huissiers) : une étude | 2 et un clerc | La Cité |
| **Mandataire judiciaire** | 1 | La Cité |
| **Avocats** | ≈ 10, en cabinets | Les Tilleuls ([ville.md](ville.md), §3.1) |
| **Centre des finances publiques** : contrôleurs fiscaux | 2 | Bâtiment administratif (§16) |
| **Inspection du travail** | 1 inspecteur | Bâtiment administratif (§16) |
| **Médecin légiste** | Fonction tenue par un médecin urgentiste de l'hôpital | Hôpital de Beaulieu |

### 5.3 Missions et routines (décidé)

*Pourquoi : les routines des policiers relèvent des fiches de rôle ([ia-pnj.md](ia-pnj.md), §11) ; une liste de tâches les rend jouables en temps réel comme les autres métiers ([economie.md](economie.md), §3.3).*

Les policiers vivent leurs journées en **tâches**, comme tous les métiers :

| Tâche | Durée **[à calibrer]** | Qui |
|---|---|---|
| Patrouille (à pied, en voiture, à vélo) dans un secteur | 30 min à 2 h | Roulement, municipale, BAC |
| Prendre un appel, envoyer une patrouille | 2 min | Chef de poste |
| Intervention sur appel | 15 min à 2 h | Roulement |
| Enregistrer une plainte | 20 à 45 min | Accueil, roulement |
| Contrôle routier, contrôle d'identité | 5 à 15 min | Roulement, municipale |
| Constatations sur une scène | 30 min à 4 h | Sûreté, technicien |
| Enquête de voisinage, audition de témoin | 15 min à 1 h | Sûreté |
| Interrogatoire en garde à vue | 30 min à 2 h | Sûreté |
| Rédiger un procès-verbal, un rapport | 15 à 45 min | Tous |
| Surveillance, filature | 1 à 4 h | BAC, sûreté |
| Escorte (vers le tribunal, vers la prison à l'extérieur) | 30 min à une demi-journée | Roulement |
| Assistance à une expulsion ([economie.md](economie.md), §5.5), à un huissier | 1 à 2 h | Roulement |
| Contrôle des registres du prêteur sur gages et de l'armurerie ([economie.md](economie.md), §7.5) | 30 min | Sûreté, chaque mois **[à calibrer]** |
| Ordre public (manifestation, grève, match, fête) | Variable | Tous |
| Enlèvement d'un véhicule en fourrière | 20 min | Municipale |

- **Où patrouiller** : le commissaire et le chef de poste répartissent les patrouilles selon la **carte des incidents connus** de la police (plaintes, appels, interventions récentes), les **priorités** (le commissaire, le maire pour la municipale) et les lieux sensibles (gare, abords des écoles aux heures d'entrée et de sortie, points de deal connus, sorties des bars). La police ne voit que ce qu'elle a appris (principe 4).
- **Disperser un attroupement** (décidé, [ia-pnj.md](ia-pnj.md) §13.9) : à son arrivée sur un incident, la patrouille fait reculer les curieux, protège la scène et les secours, et **relève l'identité** de ceux qui disent avoir vu ou filmé. Les curieux obéissent selon leur respect des règles et leur méfiance envers la police ; celui qui refuse de s'éloigner reçoit un ordre ferme, puis peut être poursuivi pour refus d'obtempérer (§8.1). *Pourquoi : c'est la première tâche réelle d'une patrouille sur un incident, et elle décide de ce qui reste de la scène.*
- **Effet de la présence policière** : une patrouille visible fait baisser l'occasion de commettre un crime pour ceux qui la voient ou la savent présente (§10.1) ; une pression durable sur un point de deal le **déplace** dans le territoire ([social.md](social.md), §6.6).
- Un policier incarné par un joueur fait ces tâches, qui rapportent de l'expérience comme toutes les obligations du jour ([04-joueur.md](../04-joueur.md), §1.1).

### 5.4 Les appels d'urgence (décidé)

*Pourquoi : la police ne peut intervenir que si elle est prévenue, et son délai d'arrivée doit être réel, comme celui des secours.*

1. Un témoin, une victime ou une alarme **appelle** ; l'appel arrive au **chef de poste** du commissariat (la nuit aussi), qui le qualifie : violence en cours, arme, blessé, cambriolage en cours, trouble, fait ancien.
2. Il l'**ordonne par priorité** **[à calibrer]** : 1) atteinte à la vie (arme, blessé grave, violence en cours) ; 2) délit en cours (cambriolage, vol avec violence) ; 3) trouble (tapage, ivresse, dispute) ; 4) fait passé (plainte à venir).
3. Il envoie la **patrouille libre la plus proche** ; s'il n'y en a pas, l'appel attend. Pour une priorité 1, il peut retirer une patrouille d'une tâche moins urgente.
4. La patrouille vient **par la route**, à vitesse d'urgence (gyrophare, sirène) : son **délai réel** dépend de la distance, du trafic et de la météo (de l'ordre de 5 à 15 minutes dans l'agglomération **[à calibrer]**).
5. Sur place : faire cesser la violence, secourir, interpeller en flagrant délit, relever les identités des témoins, protéger la scène, appeler la sûreté si c'est grave.

Les appels **multiples** pour le même fait (coup de feu entendu par vingt personnes) sont regroupés.

**Appels identifiables et saturation** (décidé, E29) :

- Chaque appel est **identifiable** par le téléphone du personnage qui appelle. Un **faux appel** est un délit (appel malveillant, §8.1) qui ouvre une affaire.
- Quand la file des appels de **priorité 1** dépasse un seuil **[à calibrer]**, le commissariat **rappelle des agents hors service** et l'hôpital déclenche un **plan blanc** (personnel rappelé). Ce sont de vrais habitants, tirés de leur vie du moment, sans aucun figurant.
- Des incidents simultanés déclenchés par des personnages d'un même cercle de joueur (§11.8) sont signalés à la modération (hors fiction ; le monde n'en sait rien).

*Pourquoi : saturer la police doit rester une stratégie criminelle possible et risquée, pas une manœuvre gratuite.*

**Habituation aux fausses alertes et aux provocations** (décidé, [ia-pnj.md](ia-pnj.md) §13.11, avec E29) :

- **Chez les habitants** : une fausse alerte répétée (alarme qui sonne souvent pour rien, cris de chahut, pétards) fait de moins en moins réagir les voisins, qui appellent moins et moins vite, puis finissent par appeler pour s'en **plaindre** (trouble, priorité 3). Une **provocation** répétée par la même personne (bousculer, insulter, faire mine de frapper) fait de moins en moins sursauter, puis met en colère, fait appeler la police ou porter plainte (harcèlement dans le monde, §8.1). Le réflexe de danger du §1.8 n'est pas émoussé par l'habituation quand la menace devient réelle (arme visible, coup porté). *Pourquoi : c'est la réaction humaine réelle, et elle ferme la boucle qui consisterait à faire sursauter un habitant à volonté.*
- **Chez la police** : le chef de poste se souvient des appels **démentis** sur place. Un appelant identifié dont plusieurs appels se sont révélés faux **[nombre à calibrer]** voit ses appels suivants **vérifiés** (rappel, patrouille déjà sur zone) avant l'envoi d'une patrouille, et une alarme connue pour ses déclenchements intempestifs est traitée en priorité 2. Une priorité 1 **corroborée** (autre appelant, coup de feu entendu, blessé signalé) n'est jamais retardée. Chaque faux appel reste un délit qui ouvre une affaire (E29). *Pourquoi : crier au loup doit avoir les effets de la réalité, y compris celui d'être moins cru, sans jamais permettre de neutraliser la police face à une vraie urgence.*

### 5.5 Pouvoirs de la police (décidé)

*Pourquoi : des pouvoirs bornés et réalistes donnent à la police des moyens d'agir, et aux autres personnages des droits que le joueur peut faire valoir.*

| Pouvoir | Condition |
|---|---|
| **Contrôle d'identité** | Raison de soupçonner une infraction, réquisition du procureur sur un lieu et une durée, ou zone sensible (gare) |
| **Palpation** | Lors d'une interpellation ou d'un risque pour la sécurité |
| **Fouille d'un véhicule, d'un sac** | Flagrant délit, ou accord de la personne |
| **Interpellation** | Flagrant délit, soupçon sérieux, mandat (§7.1) |
| **Garde à vue** | Soupçon d'un délit puni de prison ou d'un crime (§7.2) |
| **Perquisition** | Flagrant délit, ou autorisation du procureur ou du juge ; en présence de l'occupant ou de deux témoins |
| **Réquisitions** | Avec l'accord du procureur : relevés bancaires, images de caméras, relevés téléphoniques, registres (§6.3) |
| **Saisie** | Objets, armes, espèces, véhicules liés à l'infraction (§8.6) |

Un policier qui sort de ces conditions commet une **faute**, parfois un délit (§5.6), et rend la procédure **fragile** : un avocat compétent peut la faire annuler (§8.3).

### 5.6 L'usage de la force (décidé)

*Pourquoi : la police est la seule à pouvoir user légalement de la force ; une échelle graduée, déclinée par le profil de chaque policier, rend à la fois crédibles les interventions et possibles les dérapages.*

Échelle de la force, à monter seulement si l'étape précédente ne suffit pas :

1. **Présence** et **injonction** (« police, arrêtez-vous », « lâchez ça ») ;
2. **Contrainte physique** : saisir, immobiliser, menotter ;
3. **Moyens intermédiaires** : bâton, aérosol lacrymogène, pistolet à impulsion électrique ;
4. **Arme à feu** : seulement face à une menace **immédiate de mort ou de blessure grave** (légitime défense, pour soi ou pour autrui).

- **Selon le profil** : un policier violent ou peu stable monte l'échelle plus vite ; un policier calme et entraîné la monte lentement. Les recettes « interpeller » et « maîtriser » ([ia-pnj.md](ia-pnj.md), §3.3) suivent cette échelle, filtrée par le profil comme toute recette.
- **Dérapages** : un usage disproportionné est un délit aggravé (violences par une personne dépositaire de l'autorité publique, §8.1). Il peut être vu, filmé, signalé ; le **procureur** confie alors l'enquête à un autre service (abstrait, à l'extérieur, qui envoie ses conclusions) **[délai à calibrer]**. Le policier risque une sanction, une suspension, des poursuites.
- **Refus d'obtempérer**, **rébellion**, **outrage** sont des délits pour celui qui résiste (§8.1).
- Un policier blessé ou tué fait l'objet d'une mobilisation exceptionnelle de toute la police (priorité absolue de l'enquête).

### 5.7 Corruption, informateurs et loyauté (décidé)

*Pourquoi : une police incorruptible et omnisciente serait irréaliste ; ses liens avec le milieu doivent passer par les mécanismes ordinaires de relations, d'argent et de loyauté.*

- **Corruption** : un policier peut accepter un paiement pour fermer les yeux, prévenir d'une opération, détourner une enquête ou faire disparaître une pièce, selon son honnêteté, son respect des règles, ses dettes et sa loyauté envers son service ([social.md](social.md), §6.7). Le paiement et ses traces suivent [economie.md](economie.md), §7.6 (valeur de référence : 12 mois de revenu ou plus pour un risque grave). Une corruption découverte est un délit grave pour les deux (§8.1).
- **Informateurs** : les enquêteurs entretiennent quelques **informateurs** dans le milieu (relation de confiance, petits paiements, indulgence pour des délits mineurs). Un informateur transmet des croyances (qui deale où, qui prépare quoi). S'il est découvert, il reçoit l'étiquette « **balance** » ([social.md](social.md), §6.6), avec les risques qui vont avec.
- **Infiltration du milieu dans la police** : un policier peut être lié par sa famille ou ses amitiés à un membre de bande ; ses relations pèsent sur ses décisions comme sur celles de n'importe qui.

---

## 6. Les enquêtes

### 6.1 Comment la police apprend un fait (décidé)

*Pourquoi : la police ne sait que ce qu'on lui apprend ; l'inventaire de ses sources permet à chaque domaine d'y contribuer.*

| Source | Exemple |
|---|---|
| **Plainte** de la victime | Cambriolage, agression, escroquerie |
| **Appel** d'un témoin, alarme | Bagarre, coup de feu, cri |
| **Constat** par une patrouille | Flagrant délit, véhicule signalé, point de deal |
| **Signalement** d'une institution | Hôpital (blessure par arme, §2.3), banque (dépôt d'espèces, [economie.md](economie.md), §7.7), assurance (fraude soupçonnée, [economie.md](economie.md), §4.10), administration fiscale, inspection du travail, école et services sociaux (enfant en danger, [social.md](social.md), §7.3), prêteur sur gages honnête (objet signalé) |
| **Mort suspecte** | Toute mort violente ou inexpliquée constatée par un médecin |
| **Dénonciation, rumeur** | Un voisin légaliste rapporte un train de vie « louche » ([social.md](social.md), §4.5) ou une rumeur |
| **Informateur** | §5.7 |

Chaque fait connu crée ou complète une **affaire**.

### 6.2 L'affaire (décidé)

*Pourquoi : une entité « affaire » faite de croyances, avec leur certitude, est la forme qui convient à un monde où rien n'est su avec certitude ; c'est aussi l'unité que l'on peut faire avancer statistiquement loin des joueurs.*

Une **affaire** est une entité du monde, tenue par le commissariat :

| Composant | Contenu |
|---|---|
| **Faits** | Ce qui est supposé s'être passé : infraction, lieu, date, victimes |
| **Croyances de l'enquête** | Pour chaque hypothèse (« X est l'auteur », « Y était complice », « l'arme est un couteau »), une **certitude** de 0 à 1 et ses sources, au format des croyances du social ([social.md](social.md), §4.1) |
| **Indices** | Éléments matériels recueillis (§6.3), avec leur état d'analyse |
| **Témoignages** | Croyances transmises par les témoins, pondérées par leur crédibilité (§6.4) |
| **Suspects** | Personnes visées, avec la certitude correspondante |
| **Enquêteur** | Le policier chargé, et sa charge de travail |
| **Statut** | Ouverte, en attente (pas de piste), transmise au procureur, classée, jugée |

- Les croyances de l'affaire sont **partagées** par les enquêteurs qui y travaillent : un enquêteur qui lit le dossier connaît ce que contient le dossier, pas plus.
- **Seuils de certitude** **[à calibrer]** :

  | Seuil | Permet |
  |---|---|
  | ≈ 0,3 | Interroger comme témoin, surveiller, demander des réquisitions |
  | ≈ 0,5 | Placer en garde à vue, perquisitionner (avec autorisation) |
  | ≈ 0,7 | Transmettre au procureur pour poursuites |
  | Jugement | Le tribunal décide selon sa propre lecture du dossier (§8.3) |

- **Soupçon et incohérences** (décidé, [ia-pnj.md](ia-pnj.md) §13.5) : l'affaire reçoit les mêmes indices que le soupçon d'un habitant. Sont des indices qui font monter la certitude d'une hypothèse : **deux versions** d'un même fait données par la même personne (le registre des déclarations du personnage garde la version qu'il a donnée ; un menteur la reprend, mais peut s'en écarter s'il l'a oubliée, s'il est stressé ou ivre, et se trahit ainsi) ; une version **contraire** à ce qu'un autre a vu ou à un indice matériel ; une **anomalie** signalée (§4.2) ; une dépense ou une générosité hors des usages (train de vie, §6.3). Une incohérence ne prouve rien seule : elle justifie d'entendre à nouveau, de confronter (§6.5) et de vérifier. *Pourquoi : c'est par leurs contradictions que les menteurs se trahissent, et le registre des déclarations rend ces contradictions possibles sans aucune règle propre à la police.*
- **Fichier des antécédents** : la police garde les identités, photos, empreintes, signes distinctifs et modes opératoires des personnes déjà mises en cause. Un indice ou une description est d'abord comparé à ce fichier : **un primo-délinquant est plus difficile à identifier** qu'un habitué (décidé, d'après la réalité).

### 6.3 Les indices (décidé)

*Pourquoi : des indices réalistes mais limités, adaptés à une ville moyenne, donnent aux criminels des précautions à prendre et aux enquêteurs des pistes, sans tomber dans une police scientifique infaillible.*

| Indice | Comment il naît | Ce qu'il apporte | Limites |
|---|---|---|---|
| **Empreintes digitales** | Objet ou surface touchés sans gants | Identifie une personne **si elle est au fichier** | Gants ; surfaces rugueuses ; effacées au nettoyage |
| **ADN** (sang, salive, cheveu) | Blessure de l'auteur, mégot, verre | Identifie une personne au fichier ; confirme un suspect | Analyse **à l'extérieur**, délai de jours à semaines **[à calibrer]** ; réservée aux affaires graves |
| **Images** (caméras, vidéos de témoins) | Champ d'une caméra (§3.2) | Silhouette, vêtements, véhicule, heure, parfois visage | Conservation limitée, éclairage, masque |
| **Objets** | Arme abandonnée, vêtement, outil, objet volé retrouvé | Lien avec un propriétaire ou une arme | Doivent être trouvés |
| **Douilles, balles** | Tir | Rattachement à une arme saisie | Il faut l'arme |
| **Traces** | Effraction (outil), pas dans la boue, peinture d'un véhicule | Mode opératoire, outil, type de véhicule | Pluie et passage les effacent |
| **Objets signalés** | Plainte pour vol ([economie.md](economie.md), §5.10) | Un objet signalé retrouvé chez quelqu'un le met en cause (recel) | Il faut le contrôler |
| **Relevés bancaires** | Grand livre : opérations **déclarées** ([economie.md](economie.md), §2.3) | Paiements, virements, retraits ; incohérences | Les espèces n'y sont pas |
| **Relevés téléphoniques** | Appels et messages | **Qui a appelé qui**, et le **secteur** où se trouvait le téléphone ([ville.md](ville.md), §8.1) | Réservés aux délits graves ; pas le contenu ; téléphone éteint ou laissé chez soi |
| **Registres** | Prêteur sur gages, armurerie, propriété, véhicules | Qui a déposé, acheté, possède quoi | Prête-nom |
| **Train de vie** | Observation, rumeurs ([social.md](social.md), §4.5) | Piste d'enrichissement illégal | Ce n'est qu'une piste |

- **Recueillir** un indice demande d'être sur place, de le **voir** (perception, compétence de constatation) et de le **prélever** sans le contaminer ; une scène piétinée par les passants perd ses traces. La météo efface les traces extérieures.
- **Précautions** : les criminels prudents (compétence, profil organisé) portent des gants, se masquent, évitent les caméras, laissent leur téléphone ; c'est leur savoir-faire qui fait la différence, pas une règle du jeu.

### 6.4 Témoignages, auditions et identification (décidé)

*Pourquoi : un témoignage est un transfert de croyances d'un témoin à l'enquête ; réutiliser la règle « croire ou non » du social garde un seul mécanisme.*

- **Enquête de voisinage** : les enquêteurs interrogent les habitants proches (porte à porte, commerçants, gardiens). Chaque personne décide de **dire ce qu'elle sait**, d'en dire une partie, de **mentir** ou de **se taire**, selon sa relation avec l'auteur et avec la victime, sa crainte (loi du silence), son respect des règles et sa confiance dans la police.
- **Audition** : une suite d'actes sociaux (demander une information, insister, confronter, rassurer) ; l'enquêteur reçoit des croyances dont la certitude est calculée par la règle du social ([social.md](social.md), §4.2) : confiance envers le témoin, origine (vu ou entendu dire), cohérence avec le dossier, nombre de sources indépendantes, âge (§4.4).
- **Identification** : présentation de photos du fichier ou alignement de personnes au commissariat ; le témoin reconnaît ou non (§4.3). Une reconnaissance incertaine est notée comme telle.
- **Faux témoignage** : mentir à la police ou au tribunal est un délit (§8.1) ; un mensonge peut être démasqué par l'intelligence sociale de l'enquêteur ou par la confrontation avec d'autres éléments.
- **Couleurs des bandes** (décidé, v0.6, [social.md](social.md) §6.6) : la police connaît les **signes** des bandes et du réseau de l'agglomération (couleurs, pièce distinctive, signature graphique des tags) comme un **savoir professionnel**, transmis entre collègues, et apprend ceux d'une bande nouvelle (scission) par ses constats et ses informateurs. Elle traduit ainsi une description (« veste aux couleurs des Peupliers ») ou un tag en **piste** vers une bande, et oriente ses patrouilles vers les membres qui portent ces signes. Porter les couleurs d'une bande n'est **pas** une infraction et ne justifie pas un contrôle à lui seul ; c'est un indice qui s'ajoute aux autres (§6.2). Les membres retirent leurs signes sous la pression de la police, ce qui rend cette piste moins sûre là où la police est la plus présente. *Pourquoi : c'est ainsi que la police réelle lit un quartier ; garder le signe comme simple indice évite qu'une tenue suffise à mettre en cause quelqu'un.*
- **Pression sur les témoins** : une bande peut menacer un témoin pour qu'il se taise ou se rétracte (subornation de témoin, §8.1) ; le témoin cède selon sa crainte.
- **Ce que l'auteur croit que les autres savent** (décidé, [ia-pnj.md](ia-pnj.md) §13.5) : un crime, un acte violent ou un acte dissonant grave devient un **fait sensible** pour son auteur, qui tient la liste de ceux qui, **selon lui**, le savent (le passant qu'il a vu le regarder, le complice, celui à qui il s'est confié). Selon son profil, cette croyance déclenche les conduites de ce domaine et du social : **peur d'être dénoncé** (pensée de crainte, évitement), **pression sur un témoin** (le ménager, le menacer, l'acheter : subornation, §8.1), **préparer une version** (alibi, récit répété) ou **avouer avant d'être découvert** (§11.2). L'auteur se trompe comme tout le monde : il peut menacer quelqu'un qui n'a rien vu et ignorer le témoin qu'il n'a pas remarqué. *Pourquoi : la peur d'être découvert est le moteur réel de la plupart des pressions sur les témoins et des aveux, et elle doit s'appuyer sur ce que l'auteur croit, pas sur la vérité.*

### 6.5 L'interrogatoire (décidé)

*Pourquoi : [social.md](social.md), §4.4 et [ia-pnj.md](ia-pnj.md), §4.4 prévoient l'interrogatoire comme voie de transfert d'un savoir opaque et confient sa résolution à ce domaine.*

L'interrogatoire d'un suspect en garde à vue est une conversation d'actes sociaux, dans un cadre fixé :

| Actes de l'enquêteur | Actes du suspect |
|---|---|
| Questionner, **confronter** à un indice ou à un témoignage, bluffer (affirmer avoir une preuve), rassurer, presser, proposer de « dire sa version », annoncer les conséquences (vraies) | **Nier**, mentir, **se taire** (droit reconnu), raconter une version, **avouer**, **dénoncer** un complice, révéler un savoir |

Le suspect choisit selon :

```
tendance à parler = poids des éléments qu'on lui oppose (ce qu'il croit que la police sait)
                  + honnêteté + fatigue et stress de la garde à vue
                  − loyauté envers ses complices ou sa bande − crainte de leurs représailles
                  − conseils de son avocat − stabilité émotionnelle
```

- **Bluff** : une preuve inventée par l'enquêteur peut faire céder un suspect peu sûr de lui ; un suspect qui sait qu'elle ne peut exister (il portait des gants) n'y croit pas. Le bluff est permis à l'enquêteur dans les limites de la loyauté de la preuve : il ne peut pas fabriquer un faux document (décidé, d'après le droit européen courant).
- **Savoirs opaques** : un suspect qui cède peut **révéler un savoir** (emplacement d'une planque, code d'un local, nom d'un fournisseur) : c'est un transfert de **référence** vers l'enquêteur, opaque pour le joueur qui incarne l'un ou l'autre ([ia-pnj.md](ia-pnj.md), §4.4). L'enquêteur peut ensuite **agir** avec (perquisitionner la planque).
- **Pas de violence ni de torture** : la police n'a aucun acte de violence dans le cadre de l'interrogatoire ; une violence pendant la garde à vue est un délit grave et annule les aveux (§12).
- **Le milieu « interroge » aussi** : une bande qui veut savoir qui l'a dénoncée use de la **menace** ([social.md](social.md), §4.4, voie « pression »), et au besoin d'une violence ponctuelle (coups), résolue comme toute violence ; il n'existe pas d'acte de violence prolongée pour faire parler (§12.1).
- **Joueur et interrogatoire** : un joueur qui incarne un enquêteur choisit ses actes et leur réussite dépend des capacités du personnage ; un joueur qui incarne un suspect choisit ce que dit le personnage, y compris s'il révèle un savoir opaque (sous son étiquette) ; la synchronisation juge ses choix (un suspect loyal qui balance tout se désynchronise).

### 6.6 Les enquêtes financières et les contrôles (décidé)

*Pourquoi : l'économie a prévu des traces (signalements bancaires, grand livre, train de vie) et attend de ce domaine qu'il en fasse des enquêtes ([economie.md](economie.md), §7.7 et §14.6).*

- **Signalement bancaire** (dépôt d'espèces au-delà du seuil, dépôts fractionnés, virements inhabituels) : il part vers un service national (abstrait, à l'extérieur), qui le renvoie à l'enquêteur financier du commissariat après un délai de quelques semaines **[à calibrer]**, si l'affaire paraît sérieuse.
- **Enquête patrimoniale** : l'enquêteur rapproche les revenus **déclarés** (grand livre), le patrimoine (registre de propriété, véhicules), le train de vie observé (témoignages, rumeurs) et les mouvements bancaires. Un écart net fait monter la certitude d'un enrichissement illégal et peut mener à la **saisie** des avoirs (§8.6) même sans preuve du trafic d'origine (délit de non-justification de ressources, §8.1).
- **Contrôle fiscal** : tiré au sort, plus probable en cas d'incohérence ([economie.md](economie.md), §8.2) ; le contrôleur examine les déclarations, les achats et les ventes d'une entreprise, et repère les fausses recettes du blanchiment. Il inflige des redressements et des pénalités, et transmet au procureur les fraudes graves.
- **Inspection du travail** : contrôles sur place (chantiers, restaurants, garages), plus probables après une dénonciation ; elle constate le **travail au noir** et transmet ([economie.md](economie.md), §7.2).
- **Assurances** : un sinistre suspect (incendie juste avant une faillite, vol déclaré sans effraction) est signalé ; l'enquête cherche la fraude.

### 6.7 Capacité et priorités (décidé)

*Pourquoi : une police aux moyens limités doit choisir ses affaires ; c'est réaliste, et c'est ce qui laisse aux criminels prudents une vraie chance.*

- Chaque enquêteur suit un nombre limité d'**affaires actives** (≈ 10 à 20 **[à calibrer]**), choisies par **priorité** : gravité (mort, violence grave, arme), victime vulnérable, trouble public, pression du maire ou de la rumeur, lien avec une affaire en cours.
- Les petites affaires sans piste (vol simple, dégradation sans témoin) sont **classées** rapidement, ce qui est la réalité de la plupart d'entre elles.
- **Taux d'élucidation** visés **[à calibrer]**, qui servent de repères de calibrage et non de règle : homicides ≈ 80 %, violences avec auteur connu de la victime ≈ 70 %, cambriolages ≈ 10 %, vols de véhicules ≈ 10 à 15 %, vols simples ≈ 5 %.
- **Prescription** : une affaire non élucidée est close au bout d'un délai qui dépend de sa gravité **[à calibrer]** ; un meurtre ne l'est jamais dans la durée de vie d'un monde.

---

## 7. Interpellation, garde à vue et poursuites

### 7.1 L'interpellation (décidé)

*Pourquoi : c'est le moment où l'enquête redevient physique, avec ses risques ; il doit pouvoir être joué ou résolu loin des joueurs.*

- **En flagrant délit** : sur place, par la patrouille, ou par un habitant qui retient l'auteur (§1.9).
- **Sur enquête** : quand la certitude atteint le seuil (§6.2), l'enquêteur organise l'interpellation : au domicile tôt le matin, au travail, dans la rue. Pour un suspect dangereux, plusieurs équipages.
- **Personne recherchée** : si le suspect est introuvable, une **fiche de recherche** (photo, signalement) est diffusée aux policiers ; un policier qui le croise et l'a lue le reconnaît selon sa perception.
- **Réaction du suspect** : se laisser faire, **fuir** (à pied, en véhicule : poursuite, §1.1), **résister** (rébellion), selon son profil, la gravité de ce qu'il risque, ses chances et sa fatigue. Une fuite en véhicule met en danger les autres (refus d'obtempérer, §8.1).
- **Fuite à l'extérieur** : un suspect peut quitter l'agglomération. Il est alors **en fuite à l'extérieur** : il n'est plus simulé ; il est arrêté à l'extérieur au bout d'un délai tiré au sort **[à calibrer]**, puis jugé et éventuellement détenu (§9.1), ou bien il ne revient jamais (départ définitif, [ia-pnj.md](ia-pnj.md), §9.4).

### 7.2 La garde à vue (décidé)

*Pourquoi : c'est le cadre réel de l'interrogatoire ; ses délais en temps réel conviennent aux personnages du monde, mais pas au personnage persistant en ligne, qui a une règle propre.*

- **Lieu** : les cellules et salles d'audition du commissariat central (le poste municipal ne fait que remettre la personne).
- **Durée** : 24 heures, prolongeable de 24 heures par le procureur pour un délit grave, davantage pour un trafic organisé **[à calibrer]**, en temps du monde (réel en ligne).
- **Droits** : se taire, prévenir un proche, voir un médecin, être assisté d'un **avocat** (choisi ou de permanence, §7.4). Un suspect qui connaît ses droits et a un bon avocat est plus difficile à faire parler.
- **Pendant la garde à vue** : auditions, confrontations, perquisition en sa présence, vérifications.
- **Jouabilité** : un personnage en garde à vue ou en détention provisoire **n'est pas jouable** (décidé). Un joueur qui incarnait déjà le personnage au moment de l'interpellation peut continuer à le jouer pendant la garde à vue (auditions comprises). *Pourquoi : prendre le contrôle d'un gardé à vue permettrait de saboter une procédure ou de lui faire avouer ce qu'il n'a pas fait ; le joueur qui l'incarnait déjà, lui, vit simplement la suite de ses actes.* Ce joueur ne peut **avouer que des faits de la mémoire du personnage** (un fait que le personnage ignore reste une affirmation sans fondement), ni rien dire **en faveur d'un personnage de son cercle** (§11.8) (décidé, E8). *Pourquoi : sinon l'aveu ou le témoignage du gardé à vue deviendrait un moyen de couvrir un complice ou d'endosser un crime à sa place.*
- **Personnage persistant en ligne** : garde à vue **accélérée** (décidé) : les actes utiles (auditions, confrontations) se jouent normalement, puis le temps restant est sauté dès que l'enquêteur n'a plus d'acte à faire ; la décision (§7.3) tombe aussitôt. S'il se **déconnecte** pendant la garde à vue, il est réputé **garder le silence**, la procédure se termine sans lui, et il réapparaît à la reconnexion à la sortie du commissariat, avec la décision dans son courrier. *Pourquoi : c'est l'application de la règle du socle, qui refuse d'immobiliser le personnage persistant en temps réel, sans lui permettre d'échapper à la procédure.*

### 7.3 Les suites possibles (décidé)

*Pourquoi : la réponse pénale réelle est graduée ; reprendre ses voies principales donne des suites proportionnées et des procès seulement quand ils comptent.*

À la fin de la garde à vue, ou sur dossier, le **procureur** (ou un substitut) décide, selon la gravité, la certitude du dossier et les antécédents :

| Suite | Pour | Effet |
|---|---|---|
| **Classement sans suite** | Dossier trop faible, faits minimes | Aucun, mais l'affaire reste au fichier |
| **Rappel à la loi, avertissement** | Primo-délinquant, faits légers | Inscrit au fichier |
| **Amende forfaitaire** | Contraventions, usage de stupéfiants | Payée sur place ou par courrier |
| **Composition pénale** | Petits délits reconnus | Amende, stage, réparation, sans procès |
| **Plaider coupable** (reconnaissance préalable de culpabilité) | Délits reconnus | Peine proposée par le procureur, homologuée par un juge le jour même ; plus légère d'environ un tiers **[à calibrer]** |
| **Comparution immédiate** | Délits flagrants, auteur en garde à vue | Procès le jour même ou le lendemain |
| **Convocation au tribunal** | Délits | Procès dans quelques semaines |
| **Information judiciaire** | Crimes, affaires complexes | Confiée au **juge d'instruction** ; mise en examen, **contrôle judiciaire** ou **détention provisoire** (à l'extérieur), procès plusieurs mois plus tard |

### 7.4 Les avocats (décidé)

*Pourquoi : la qualité de la défense change réellement les issues, et les avocats sont des habitants jouables ([economie.md](economie.md), §3.8).*

- Tout suspect et tout prévenu peut être assisté d'un avocat **choisi** (honoraires, [economie.md](economie.md), §1.4) ou **de permanence** (commis d'office, payé par l'État au titre de l'aide juridictionnelle pour les personnes modestes).
- La **compétence** de l'avocat (droit pénal, plaidoirie) et le temps qu'il consacre au dossier pèsent sur l'issue : contestation d'un indice, d'un témoignage, d'une procédure irrégulière (§5.5), circonstances atténuantes, négociation d'un plaider-coupable.
- Les bandes et le réseau paient des avocats à leurs membres ([economie.md](economie.md), §7.6), ce qui est aussi un signe de loyauté.
- Un avocat est tenu au **secret professionnel** : ce que lui confie son client lui est confié comme un savoir couvert par un engagement de discrétion très fort.

---

## 8. La justice

### 8.1 Les infractions (décidé)

*Pourquoi : un catalogue fermé d'infractions, chacune avec sa qualification et sa peine type, sert à la fois à l'IA (coût d'une recette illégale), à la synchronisation, à la police (priorités) et au tribunal (barème).*

Trois catégories, sur le modèle européen courant : **contraventions** (tribunal de police, amende), **délits** (tribunal correctionnel, jusqu'à 10 ans de prison), **crimes** (cour d'assises, au-delà).

Chaque infraction du catalogue porte : sa **catégorie**, sa **peine type** pour un primo-délinquant et sa **peine maximale** (§8.5) et ses **circonstances aggravantes** possibles.

- **Plus de signature écrite** (décidé, G1, G3) : une infraction ne porte pas de signature comportementale. Le catalogue fournit sa **qualification** et sa peine type, qui donnent l'effet de **violation de la loi**, d'autant plus grave que la peine l'est (contravention, délit selon sa peine type, crime) ([revues/signatures.md](../revues/signatures.md), §3.2), selon le barème M2 du §8.5. Le reste de la signature vient des **effets de l'acte** lui-même : tort physique, contrainte, prise du bien d'autrui, tromperie, tort matériel, mise en danger. Un vol avec violence porte ainsi la prise, la blessure attendue et la violation de la loi, sans rien écrire de plus. *Pourquoi : la même infraction peut être commise par des actes très différents ; c'est l'acte, pas son nom juridique, qui dit ce qu'il révèle du personnage.*

| Famille | Infractions |
|---|---|
| **Atteintes aux personnes** | Violences légères, violences ayant entraîné une incapacité (selon sa durée), violences aggravées, menaces, harcèlement (dans le monde), homicide involontaire (accident, conduite), meurtre, assassinat (préméditation), non-assistance à personne en danger, mise en danger d'autrui, **violation de domicile** (entrer ou se maintenir chez autrui sans y être invité, même par une porte ouverte ; décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), A23), **mise en danger d'un mineur** (se tenir au contact de mineurs pendant un affrontement ou s'en couvrir, §12.2, E27) |
| **Atteintes aux biens** | Vol simple, vol avec violence, vol en réunion, cambriolage (vol avec effraction dans un local habité), vol de véhicule, vol à l'étalage, vol de caisse (par un employé : abus de confiance), recel, dégradations, tags, incendie volontaire |
| **Extorsions** | Racket, chantage, usure, extorsion de fonds sous la menace |
| **Trafics** | Usage de stupéfiants, revente, trafic organisé ; contrebande de cigarettes ; détention, port, cession et trafic d'armes |
| **Fraudes et probité** | Escroquerie, abus de faiblesse, abus de confiance, fraude fiscale, travail dissimulé, fraude à l'assurance, fraude aux prestations, blanchiment, non-justification de ressources, **corruption** active et passive (agents publics, policiers, élus), achat ou divulgation de secrets professionnels |
| **Justice et autorité** | Faux témoignage, subornation de témoin, dénonciation calomnieuse, refus d'obtempérer, rébellion, outrage, violences contre un policier, violences par une personne dépositaire de l'autorité publique, évasion, non-respect d'une interdiction prononcée par le tribunal ou d'une surveillance électronique, **appel malveillant** aux services d'urgence (§5.4, E29), **usurpation de fonction** et port illégal d'un uniforme ou d'un insigne de police, de secours ou de justice ([social.md](social.md), §5.5) |
| **Route et ordre public** | Conduite en état d'ivresse ou sous stupéfiants, délit de fuite, excès de vitesse, stationnement, ivresse publique, tapage, dont le **tapage nocturne** ([social.md](social.md), §5.11), port d'arme blanche sans motif |
| **Débits et jeux** | Servir de l'alcool à un client **manifestement ivre**, **vente d'alcool ou de tabac à un mineur** ([economie.md](economie.md), §7.3), **tenue de jeux d'argent clandestins** ([economie.md](economie.md), §7.6) |

**Circonstances aggravantes** : victime **mineure**, **femme enceinte** ([population.md](population.md), §7.2), personne âgée ou vulnérable, policier ou autre agent public, conjoint ou ex-conjoint ; **arme** ; **réunion** (plusieurs auteurs) ; **préméditation**, guet-apens ; dans un établissement scolaire ou à ses abords ; masqué. Chacune alourdit la peine **[ampleur à calibrer]**.

**Complicité** (décidé, [social.md](social.md) §6.7) : aider, faciliter ou **ordonner** une infraction (instigation, §10.2) rend complice ; le complice encourt la même peine que l'auteur. *Pourquoi : c'est la règle européenne courante, et elle permet de poursuivre ceux qui commandent sans agir.*

**Infractions de voisinage, de tenue et de débit** (décidé, v0.6, [social.md](social.md) §12.12 ; [economie.md](economie.md) §14.10) :

- **Tapage nocturne** : la musique ou le bruit qui dépasse les heures calmes, quand un voisin appelle la police au lieu de tolérer ou de demander ([social.md](social.md), §5.11). C'est un **trouble** de priorité 3 (§5.4) : une patrouille vient, constate si le bruit dure encore, et dresse une contravention ; si le bruit a cessé, elle se contente d'un rappel. La récidive au même logement est connue du chef de poste, comme les appels démentis. *Pourquoi : c'est l'aboutissement réel de l'escalade du bruit entre voisins, et une charge de travail ordinaire pour la police de nuit.*
- **Usurpation de fonction** : porter sans la fonction un uniforme, un brassard ou un insigne de police, de secours ou de justice, ou se faire passer pour un agent. Un tel déguisement trompe les témoins comme les autres (§4.3), et le fait d'être pris pour un policier ([social.md](social.md), §5.5) peut servir à une escroquerie, à un vol ou à une intrusion ; l'usurpation s'ajoute alors à ces infractions. La tenue elle-même s'obtient par un vol (commissariat, caserne, vestiaire de l'hôpital) ou par une imitation de mauvaise qualité, que les vrais agents remarquent. *Pourquoi : la tenue fait reconnaître une fonction ; la porter sans droit doit avoir un prix, sinon elle deviendrait le déguisement idéal.*
- **Servir un client manifestement ivre** : la contravention vise le **débit** (le serveur et l'exploitant), constatée lors d'un contrôle ou après une bagarre ou un accident à la sortie d'un bar. La signature reste celle de l'économie (mise en danger 0,1) ; la qualification ajoute la violation de la loi d'une contravention.
- **Vente d'alcool ou de tabac à un mineur** : la vente **en connaissance de cause** n'est jamais proposée ([economie.md](economie.md), §7.3). L'infraction existe pour le vendeur qui a vendu à un adolescent d'allure adulte **sans vérifier son âge** : la justice le juge sur le fait, mais la signature, tirée des croyances du vendeur, ne porte aucune violation de la loi s'il le croyait majeur (D21). *Pourquoi : la règle absolue de l'économie est préservée, et le catalogue couvre le seul cas réel qui reste possible.*
- **Jeux clandestins** : seuls l'**organisateur** (patron de bar, bande) et ceux qui tiennent la table sont poursuivis ; les simples joueurs ne le sont pas (décidé). Voir §10.3. *Pourquoi : c'est la pratique la plus courante en Europe, et elle porte l'effort de la police sur ceux qui tirent profit du jeu.*

**Ce qui n'existe pas** dans le catalogue, parce que les actes correspondants n'existent pas (§12) : violences sexuelles, violence physique d'un adulte envers un enfant, torture, séquestration, enlèvement, empoisonnement.

### 8.2 Les juridictions (décidé)

*Pourquoi : la ville place le tribunal dans la Cité ([ville.md](ville.md), §5.2) ; y rassembler toutes les juridictions, cour d'assises comprise, garde les procès visibles et jouables, avec des magistrats et des jurés habitants.*

| Juridiction | Juge | Siège |
|---|---|---|
| **Tribunal de police** | Un juge | Tribunal de la Cité |
| **Tribunal correctionnel** | Un ou trois juges | Tribunal de la Cité |
| **Cour d'assises** | Trois magistrats et **six jurés** tirés au sort parmi les habitants | Tribunal de la Cité, en **sessions** périodiques **[à calibrer]** |
| **Juge des enfants**, tribunal pour enfants | Juge des enfants | Tribunal de la Cité |
| **Tribunal judiciaire (civil)** | Un juge | Dettes, expulsions ([economie.md](economie.md), §5.5), divorces ([economie.md](economie.md), §5.12), successions disputées, dommages et intérêts |
| **Tribunal de commerce** | Juges consulaires | Faillites, litiges entre entreprises ([economie.md](economie.md), §4.6) |
| **Appel** | **À l'extérieur** (cour d'appel régionale) | Le dossier part ; la décision revient après un délai **[à calibrer]** |

- La cour d'assises siège à Orvenne (décidé), même si, dans la réalité, elle se tiendrait au chef-lieu du département. *Pourquoi : sans figurants, un procès criminel tenu à l'extérieur serait invisible ; à Orvenne, les grandes affaires deviennent des événements de la ville, avec des jurés qui sont des voisins.*
- **Jurés** (décidé) : six adultes inscrits sur les listes électorales, tirés au sort pour une session, hors personnages persistants (souvent absents) et hors personnes liées à l'affaire. Être juré est une **obligation** de l'agenda pendant quelques jours ; l'employeur doit libérer le salarié. Un joueur qui incarne un juré participe au délibéré (§8.3). *Pourquoi : c'est une vraie expérience civique, jouable, et une source de conversations et de rumeurs.*
- L'**appel** se tient à l'extérieur : le monde n'en voit que la décision, ce qui borne le coût des procès sans rien retirer d'essentiel.

### 8.3 Le procès (décidé)

*Pourquoi : la décision du tribunal doit venir du dossier, comme celle de l'enquête, mais laisser leur part aux témoins, à l'avocat, au profil du juge et au hasard des audiences.*

**Déroulé** : audience **publique** (tout le monde peut entrer, dans la limite des places) : lecture des faits, interrogatoire du prévenu, **témoins** cités (une obligation de leur agenda : ils viennent, ou non, et peuvent changer de version), parties civiles, réquisitions du procureur, plaidoirie de la défense, dernier mot du prévenu, délibéré, décision.

**Résolution** :

```
certitude du tribunal = lecture du dossier (indices, témoignages pondérés, aveux)
                      ± ce qui se passe à l'audience (témoin qui se rétracte ou confirme, contestation par l'avocat, attitude du prévenu)
culpabilité retenue si certitude du tribunal ≥ seuil de condamnation (≈ 0,8 [à calibrer])
peine = peine type de l'infraction (§8.5) × circonstances (aggravantes, atténuantes) × antécédents (casier) × sévérité du juge (profil)
```

- **Circonstances atténuantes** : légitime défense partielle, provocation, aveux, regrets, réparation du dommage, situation personnelle (emploi, enfants), jeune âge.
- **Légitime défense** reconnue : relaxe.
- **Cour d'assises** : chaque juré et chaque magistrat se forme sa propre certitude, puis **délibère** : une conversation d'actes (persuader, affirmer, douter), dont l'issue suit les règles d'influence du social ([social.md](social.md), §3.1). La condamnation demande une majorité qualifiée (sept voix sur neuf, d'après le modèle français **[à calibrer]**).
- **Juge incarné par un joueur** : il conduit l'audience (questions, ordre des témoins) et choisit la décision **dans la fourchette légale** ; une décision éloignée de ce que déciderait le personnage (un juge sévère qui relaxe un récidiviste) le désynchronise. Le procureur (hors catégorie) n'est jamais incarné.
- **Erreurs judiciaires** : elles sont possibles (faux témoignages, reconnaissance erronée, aveux d'un suspect qui protège quelqu'un), et le jeu ne les corrige pas (principe 4). Un condamné peut faire **appel** ; un fait nouveau (aveu du vrai coupable) peut mener à une **révision** **[délais à calibrer]**.
- **Délais** **[à calibrer]**, en temps du monde : comparution immédiate le jour même ou le lendemain ; audience correctionnelle 2 à 8 semaines après la convocation ; procès d'assises 6 à 18 mois après les faits.
- La **chronique judiciaire** du journal local rend compte des procès : c'est une source d'**information** pour toute la ville ([social.md](social.md), §4.1).

### 8.4 Les peines (décidé)

*Pourquoi : les peines européennes ne se résument pas à la prison ; les alternatives gardent souvent le condamné dans la ville, donc jouable, ce qui convient à un monde où chacun compte.*

| Peine | Effet dans le monde |
|---|---|
| **Amende** | Prélèvement par le mécanisme de l'économie (§8.6) |
| **Jours-amende** | Un montant par jour, proportionnel aux revenus, sur un nombre de jours ; non payés, ils se transforment en détention |
| **Travail d'intérêt général** | Des heures de travail non payé pour la commune ou une association ([economie.md](economie.md), §4.9), inscrites dans l'**agenda** : des tâches comme un emploi, sans salaire |
| **Sursis simple** | Peine suspendue ; une nouvelle condamnation pendant le délai d'épreuve la fait exécuter |
| **Sursis probatoire** | Avec **obligations** (travailler, se soigner, indemniser la victime, rencontrer le conseiller d'insertion) et **interdictions** ; suivi par le juge de l'application des peines |
| **Détention à domicile sous surveillance électronique** (bracelet) | Pour les courtes peines (jusqu'à 1 an **[à calibrer]**) : le condamné reste chez lui avec des **horaires** imposés (travail autorisé, couvre-feu) ; tout manquement est détecté et peut le renvoyer en prison. Il reste **jouable** |
| **Emprisonnement ferme** | **Détention à l'extérieur** (§9.1) |
| **Interdictions** | De séjour dans un quartier ou un lieu, de contact avec la victime, de détenir une arme, d'exercer une profession (policier, enseignant, agent de sécurité), de conduire (suspension du permis), de gérer une entreprise |
| **Confiscation** | De l'arme, du véhicule, des avoirs issus de l'infraction (§8.6) |
| **Dommages et intérêts** | Versés à la victime (§8.6) |
| **Stage** | Sensibilisation (stupéfiants, sécurité routière, violences), quelques demi-journées dans l'agenda |

- Le **bracelet électronique** est retenu (décidé). *Pourquoi : il existe dans toute l'Europe, et il garde dans la ville, avec des contraintes jouables, des personnages qu'une courte peine de prison ferait disparaître des mois.*
- Un **manquement** aux obligations, interdictions ou horaires est constaté (contrôle, bracelet, signalement) et peut entraîner la révocation du sursis ou l'incarcération.

### 8.5 Le barème indicatif (décidé)

*Pourquoi : l'économie attend de ce domaine le barème des amendes et des peines ([economie.md](economie.md), §8.2 et §14.6) ; une table unique sert au procureur, au tribunal et à la conversion pour le personnage persistant.*

Peines types pour un **primo-délinquant** majeur, sans circonstance aggravante ni atténuante. La récidive les **double** environ ; chaque circonstance aggravante les augmente **[toutes valeurs à calibrer]**. Les montants sont en écus ; les durées en temps du monde.

| Infraction | Peine type | Maximum |
|---|---|---|
| Stationnement gênant | Amende de 35 | — |
| Tapage nocturne | Amende de 68 | 450 |
| Ivresse publique | Amende de 150 | 150 |
| Excès de vitesse | Amende de 135, retrait de points | Suspension du permis |
| Usage de stupéfiants | Amende forfaitaire de 200 | 1 an |
| Port d'arme blanche sans motif | 500 et confiscation | 2 ans |
| Vol à l'étalage, vol simple | Rappel à la loi ou 300 à 1 000 | 3 ans |
| Dégradation, tag | Réparation ou 500 à 1 500, travail d'intérêt général | 2 ans |
| Violences légères | 1 000 ou sursis de 2 mois | 3 ans |
| Violences avec incapacité de plus de 8 jours | Sursis de 6 mois à 1 an | 5 ans |
| Menaces, harcèlement | Sursis de 3 mois, interdiction de contact | 3 ans |
| Violation de domicile | Sursis de 3 mois | 1 an |
| Usurpation de fonction, port illégal d'uniforme ou d'insigne | 1 500 ou sursis de 3 mois, confiscation de la tenue | 3 ans |
| Servir un client manifestement ivre | Amende de 135 (exploitant ou serveur) | 750 |
| Vente d'alcool ou de tabac à un mineur | Amende de 1 000 | 7 500, interdiction temporaire de vendre |
| Tenue de jeux d'argent clandestins | Sursis de 6 mois, confiscation des enjeux et de la caisse | 3 ans |
| Recel | Sursis de 6 mois, confiscation | 5 ans |
| Vol avec violence | 1 an dont 6 mois ferme | 5 ans |
| Cambriolage | 1 an dont 6 mois ferme | 7 ans |
| Vol de véhicule | Sursis de 6 mois à 1 an | 5 ans |
| Escroquerie, abus de faiblesse, abus de confiance | Sursis de 1 an, dommages et intérêts | 5 ans |
| Fraude à l'assurance, aux prestations | Sursis, remboursement, amende | 5 ans |
| Travail dissimulé (employeur) | 3 000 à 15 000 | 3 ans |
| Fraude fiscale | Redressement, pénalités, et au-delà d'un seuil sursis | 5 ans |
| Conduite en état d'ivresse | 500, suspension du permis | 2 ans |
| Racket, extorsion | 2 ans dont 1 ferme | 7 ans |
| Usure | 1 an ferme, confiscation | 5 ans |
| Revente de stupéfiants | 1 an dont 6 mois ferme | 10 ans |
| Trafic organisé (chef de réseau) | 6 ans ferme, confiscation | 20 ans (crime) |
| Détention, port d'arme à feu illégale | 1 an dont 6 mois ferme | 5 ans |
| Trafic d'armes | 3 ans ferme | 10 ans |
| Corruption (agent public, policier, élu) | 2 ans dont 1 ferme, interdiction d'exercer | 10 ans |
| Blanchiment, non-justification de ressources | 2 ans, confiscation | 5 à 10 ans |
| Faux témoignage, subornation | Sursis de 6 mois | 5 ans |
| Rébellion, outrage, refus d'obtempérer | 500 à sursis de 3 mois | 2 à 5 ans |
| Incendie volontaire (sans victime) | 2 ans dont 1 ferme | 10 ans |
| Homicide involontaire (route) | 2 ans, suspension du permis | 5 ans |
| Meurtre | 15 ans | 30 ans |
| Assassinat | 25 ans | Perpétuité |

- **Magnitude de la violation de la loi** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), M2) : l'effet de violation de la loi (§8.1) suit la **peine type** de ce barème, non le maximum : contravention 0,10 ; délit puni d'une amende ou d'un rappel à la loi 0,25 ; sursis de 6 mois au plus 0,35 ; sursis plus long ou ferme de 6 mois au plus 0,45 ; ferme de 6 mois à 2 ans 0,55 ; ferme de plus de 2 ans 0,70 ; crime 0,85 ; meurtre 0,95 ; assassinat 1 **[à calibrer]**. Les circonstances aggravantes **propres à l'auteur** (arme, réunion, préméditation, masque, qualité d'agent public) ajoutent **0,05** chacune, **0,15** au plus. Celles qui tiennent à la **victime** (mineure, enceinte, âgée ou vulnérable, conjoint, policier) ne s'ajoutent pas : elles passent par la **classe de cible** (G4). *Pourquoi : le maximum rapprochait trop un vol simple d'un cambriolage, et compter la victime dans les deux termes aurait doublé le poids de sa vulnérabilité.*
- **Amendes et revenus** : pour les délits, le tribunal module le montant selon les revenus du condamné (décidé). *Pourquoi : une même amende ne pèse pas pareil pour un chômeur et un patron, comme le coût relatif de l'IA ([ia-pnj.md](ia-pnj.md), §3.2).*
- **Bénéficiaires** : les amendes vont à l'**État** (compte extérieur, [economie.md](economie.md), §2.2), sauf les contraventions constatées par la police municipale, qui vont à la **commune** ([economie.md](economie.md), §8.2).

### 8.6 Amendes, saisies, confiscations et indemnisation (décidé)

*Pourquoi : l'économie fournit le mécanisme de prélèvement et de vente aux enchères ([economie.md](economie.md), §7.8) ; ce domaine en fixe l'ordre et les cas, sans jamais créer ni détruire d'argent.*

**Recouvrement** d'une amende, de dommages et intérêts ou d'un redressement :

1. paiement volontaire dans un délai **[à calibrer]**, ou échéancier accordé ;
2. **prélèvement sur les comptes** du condamné ;
3. **saisie sur salaire**, plafonnée à une fraction du salaire ([economie.md](economie.md), §5.5) ;
4. **saisie et vente aux enchères** de biens : véhicules, puis objets de valeur, puis, pour les très grosses sommes, biens immobiliers autres que la résidence principale ;
5. le reste demeure une **dette** envers l'État ou la victime, qui ne s'efface pas.

**Saisies pendant l'enquête** : espèces, stupéfiants, armes, véhicules et objets liés à l'infraction sont **saisis** et gardés au commissariat (scellés) ; les stupéfiants et les armes sont détruits après jugement (la valeur des stupéfiants sort de l'économie comme un stock détruit, ce qui est déjà le cas d'une marchandise perdue).

**Confiscation** des avoirs criminels : les espèces saisies, et les biens acquis avec l'argent d'un trafic (y compris par un prête-nom, s'il est démontré), sont confisqués au profit de l'**État** ; les biens sont vendus aux enchères et le produit va au compte « État ».

**Indemnisation des victimes** :

- le condamné doit des **dommages et intérêts** à la victime (transfert entre comptes, recouvré comme ci-dessus) ;
- pour les violences graves et les morts violentes, si l'auteur est inconnu ou insolvable, un **fonds de garantie** de l'État (compte extérieur « État ») indemnise la victime ou ses proches **[montants à calibrer]**, puis se retourne contre l'auteur s'il est retrouvé (décidé). *Pourquoi : c'est le mécanisme réel en Europe ; il passe par le compte extérieur prévu à cet effet, donc l'économie reste fermée et sans création d'argent.*
- l'**assurance** de la victime rembourse les dégâts et se retourne contre l'auteur identifié ([economie.md](economie.md), §5.8).

### 8.7 Le casier judiciaire (décidé)

*Pourquoi : le casier est la mémoire des institutions ; il pèse sur l'embauche et sur les peines futures, mais il est distinct de la réputation, qui ne vient que de ce que les gens savent.*

- Le **casier** est un composant du personnage : la liste de ses condamnations (infraction, date, peine), tenue par l'État.
- **Qui le voit** : la police et la justice entièrement ; un **employeur** seulement pour certains métiers (police, sécurité, enseignement, garde d'enfants, banque) ou si le candidat le fournit à sa demande ([economie.md](economie.md), §3.6) ; jamais les autres habitants. Les habitants ne savent qu'une condamnation existe que s'ils l'ont apprise (procès suivi, journal, rumeur), ce qui est une affaire de **réputation** ([social.md](social.md), §5).
- **Effacement** : les condamnations disparaissent du casier après un délai qui dépend de la peine **[à calibrer]** (3 ans pour une amende, 10 ans pour une peine de prison, jamais pour un crime dans la durée d'un monde).
- **Effets** : peines plus lourdes en cas de récidive ; refus d'embauche dans certains métiers ; refus de permis de chasse ou de port d'arme ; méfiance d'un propriétaire qui l'apprendrait.
- **Indignité successorale** (décidé, [economie.md](economie.md), §5.11 et §14.8 ; revue d'équilibrage E31) : une condamnation définitive pour la **mort** d'une personne (meurtre, assassinat, violences ayant entraîné la mort), comme **auteur** ou comme **complice**, y compris par instigation (§8.1, §10.2), est **transmise par la justice à la succession** du défunt. Si le partage n'est pas encore fait, l'héritier condamné en est exclu ; s'il est déjà fait, la transmission ouvre la **restitution** de sa part, recouvrée comme une créance de victime (§8.6). La transmission ne dépend pas du délai écoulé depuis le partage, et elle vaut aussi pour une condamnation par défaut (§9.5). L'homicide involontaire (accident de la route, par exemple) n'est **pas** transmis. *Pourquoi : l'économie exclut l'héritier indigne mais ne peut pas savoir qui a tué ; seule la condamnation l'établit, et sans transmission après le partage, il suffirait de faire traîner l'enquête pour garder l'héritage.*
- **Départ** : une petite part des adultes a un casier au départ, dont quelques anciens détenus, inscrit dans leur biographie ([population.md](population.md), §5.1).

### 8.8 Les mineurs (décidé)

*Pourquoi : des adolescents commettent de petits délits (bandes, vols, bagarres) ; la justice des mineurs, distincte et éducative en Europe, doit les traiter sans en faire des adultes.*

- **Moins de 13 ans** : aucune sanction pénale ; des **mesures éducatives** (avertissement, suivi), et l'affaire passe aux **services sociaux** ([social.md](social.md), §7.3).
- **13 à 17 ans** : **juge des enfants** ; d'abord des mesures éducatives (réparation, suivi, placement éducatif) ; des peines réduites de moitié par rapport aux adultes, rarement de la détention (dans un établissement pour mineurs, à l'extérieur).
- **Responsabilité des parents** : ils paient les dommages et intérêts causés par leur enfant mineur ; un mineur délinquant est un **signe** qui alimente l'escalade de la négligence ([social.md](social.md), §7.3).
- **Recrutement par une bande** : faire travailler un mineur dans un trafic est une circonstance aggravante pour l'adulte.
- Le casier d'un mineur est **effacé** à 18 ans pour les mesures éducatives (décidé). *Pourquoi : l'adulte jouable qu'il devient ne doit pas porter toute sa vie une bêtise d'adolescent.*

### 8.9 La justice civile (décidé)

*Pourquoi : le social et l'économie renvoient ici les dettes entre particuliers, les expulsions et les litiges ; une procédure unique, menée par les mêmes juges et le même huissier, les traite.*

| Litige | Procédure | Suite |
|---|---|---|
| **Dette entre particuliers** ([social.md](social.md), §1.8) | Le créancier saisit le tribunal avec une preuve (reconnaissance de dette, témoins, virement) ; **injonction de payer** | Si le débiteur ne paie pas, l'**huissier** recouvre par les voies du §8.6 |
| **Loyers impayés, expulsion** ([economie.md](economie.md), §5.5) | Procédure de l'économie | Expulsion par l'huissier, avec le **concours de la police** si l'occupant refuse ; trêve hivernale |
| **Dommages** (dégât, accident) | Plainte ou demande civile | Dommages et intérêts, souvent réglés entre assurances |
| **Opération dissonante après une incarnation** ([economie.md](economie.md), §12.2) | Le personnage porte plainte pour escroquerie ou abus de faiblesse | L'enquête cherche des **manœuvres** du bénéficiaire (mensonges, pressions vus ou rapportés) : s'il y en a, poursuite et restitution ; sinon, **classement**, puisque le personnage a agi lui-même aux yeux du monde (décidé). *Pourquoi : le monde ne connaît pas les joueurs ; la protection est d'abord dans les délais de rétractation de l'économie, et la justice ne peut punir que ce qu'elle peut constater* |

---

## 9. Détention et conséquences équivalentes

### 9.1 La prison à l'extérieur (décidé)

*Pourquoi : la ville a placé la prison à l'extérieur ([ville.md](ville.md), §5.2) ; une détention qui suit le modèle des séjours à l'extérieur de l'IA réutilise toute la mécanique existante.*

La **prison à l'extérieur** est **confirmée**. Un personnage condamné à de la prison ferme, ou placé en détention provisoire, devient **« à l'extérieur », motif détention** ([ia-pnj.md](ia-pnj.md), §9) :

- **Départ réel** : il est conduit par une **escorte** de policiers jusqu'à une sortie de l'agglomération par la route ; les policiers de l'escorte sont eux-mêmes à l'extérieur le temps du trajet (quelques heures **[à calibrer]**), puis reviennent.
- **Non jouable** pendant toute la détention (socle).
- **Durée** : la peine prononcée, moins les **réductions de peine** (environ un tiers pour bonne conduite **[à calibrer]**) ; une **libération conditionnelle** est possible à mi-peine, décidée par le juge de l'application des peines selon un tirage qui tient compte du profil, des incidents de détention (§9.2) et d'un **projet de sortie** (un logement, un emploi, un proche qui l'attend).
- **Détention provisoire** : en attente de procès ; elle compte dans la peine finale ; une relaxe la termine.
- **Retour** : comme tout retour de séjour ([ia-pnj.md](ia-pnj.md), §9.3) : il réapparaît à la gare ou à un arrêt de bus d'une sortie, à la date prévue, puis rentre chez lui, s'il a encore un chez-lui.

### 9.2 Pendant la détention (décidé)

*Pourquoi : [ia-pnj.md](ia-pnj.md), §9.2 laisse à ce domaine le contenu du résumé statistique de la détention ; la vie en ville, elle, continue réellement autour de l'absent.*

**Dans la ville**, la vie continue avec les règles normales :

| Domaine | Ce qui se passe |
|---|---|
| **Emploi** | Un salarié incarcéré plus d'**un mois** **[à calibrer]** perd son emploi (abandon de poste, licenciement pour faute) ; un fonctionnaire est suspendu, puis révoqué si la peine est lourde ; une entreprise qu'il gérait passe à son suppléant ou à l'automate de gestion ([economie.md](economie.md), §11.5) |
| **Argent** | Plus de salaire ; l'**échéancier** continue (loyer, crédit) ; il peut donner **procuration** à un proche avant de partir ; les dettes s'accumulent si personne ne paie |
| **Logement** | Le ménage reste ; une personne seule qui ne paie plus finit expulsée ([economie.md](economie.md), §5.5) ; ses affaires vont au garde-meubles |
| **Famille** | Le conjoint assume seul le foyer ; les enfants mineurs sans autre parent sont confiés selon [population.md](population.md), §7.7 ; un couple peut se séparer (projet du social) |
| **Groupe** | Son **suppléant** prend son poste ; au-delà d'une durée **[à calibrer]**, l'intérim devient définitif, et son retour crée une rivalité ([social.md](social.md), §6.8) |
| **Relations** | Elles dérivent normalement pendant l'absence ([social.md](social.md), §1.5) ; seuls les proches qui lui **rendent visite** (un séjour « visite » à l'extérieur, [ia-pnj.md](ia-pnj.md), §9.1) les entretiennent |

**Résumé statistique de la détention**, tiré au sort et appliqué au retour **[probabilités à calibrer]** :

| Événement | Effet |
|---|---|
| Désentraînement | Endurance et forme en baisse selon la durée ; la force peut être **maintenue** ou progresser si son profil le pousse à s'entraîner en cellule |
| Formation | Un diplôme ou une compétence acquise (cuisine, mécanique, lecture) qui aide à se réinsérer |
| Incidents | Bagarres (blessures, souvenir, allongement de peine), sanctions disciplinaires |
| Dépendance | Elle peut s'aggraver ou au contraire se sevrer |
| Contacts | Relations avec les autres détenus **habitants d'Orvenne** détenus en même temps (de vraies relations) ; et une **connaissance du milieu extérieur**, abstraite, qui facilite l'accès aux trafics ([economie.md](economie.md), §7.3) |
| Souvenirs | Souvenir marquant de la détention (stress, méfiance, ou endurcissement), sans changement du profil d'un adulte (socle) |

### 9.3 Le retour (décidé)

*Pourquoi : la sortie de prison est un moment de vie difficile et riche, qui doit naître des conditions concrètes du personnage.*

- **Suivi** : obligations de la libération conditionnelle ou du sursis probatoire (rendez-vous avec le conseiller d'insertion, recherche d'emploi, interdictions) dans l'agenda, pendant le **délai d'épreuve** **[à calibrer]**.
- **Difficultés** : casier (embauche, §8.7), logement souvent perdu (foyer, hébergement chez un proche), relations distendues, étiquette « **ancien détenu** » chez ceux qui le savent.
- **Projets** qui naissent naturellement de sa situation ([ia-pnj.md](ia-pnj.md), §3.4) : trouver un emploi et un logement, renouer avec sa famille, reprendre sa place dans la bande, se venger de celui qui l'a dénoncé, ou rompre avec le milieu.
- Il redevient **jouable** dès son retour, aux conditions habituelles.
- **Habitants en détention au départ** : quelques habitants commencent le monde en détention, avec une date de retour, et reviendront dans les premiers mois ([population.md](population.md), §5.4).

### 9.4 Le personnage persistant en solo (décidé)

*Pourquoi : le socle prévoit qu'en solo la peine s'écoule avec le temps de jeu ; il faut dire ce que fait le joueur pendant ce temps.*

En **solo**, le personnage persistant condamné à de la prison ferme est **réellement détenu à l'extérieur**, comme un habitant. Pendant sa détention, le joueur peut **incarner d'autres personnages** jouables, ou **faire avancer le temps** jusqu'à sa libération. Au retour, les effets du §9.2 et du §9.3 s'appliquent. Il ne peut pas reprendre son personnage persistant avant la fin de la peine (réductions et libération conditionnelle comprises).

### 9.5 Le personnage persistant en ligne : conséquences équivalentes immédiates (décidé)

*Pourquoi : le socle remplace la prison en ligne par des conséquences équivalentes immédiates ([01-concept.md](../01-concept.md), §2) et laisse à ce domaine leur détail ; une conversion par mois de prison, sur le modèle de la règle de convalescence, rend la sanction proportionnée et lisible.*

**Procédure** :

- **Interpellé** : garde à vue accélérée (§7.2), puis, pour un délit, le choix entre le **plaider coupable** (peine proposée et appliquée aussitôt, plus légère) et l'**audience** (jouée pendant la session s'il attend, sinon fixée à sa prochaine connexion).
- **Non interpellé** (enquête à distance) : quand le dossier est transmis, une **convocation** l'attend dans son courrier ; s'il ne se présente pas à l'audience dans un délai de **14 jours réels** **[à calibrer]**, connecté ou non, il est **jugé par défaut** et les conséquences s'appliquent quand même (décidé). *Pourquoi : les procédures continuent pendant les absences, comme celles de l'économie ([economie.md](economie.md), §12.5) ; on n'échappe pas à la justice en restant déconnecté.* Le jugement par défaut ne retient **aucun aménagement favorable** (ni plaider-coupable, ni circonstance atténuante liée à la comparution) et s'accompagne d'un **mandat** : tout contrôle de police mène aussitôt à la garde à vue (décidé, E22). *Pourquoi : ne pas se présenter ne doit jamais coûter moins cher que comparaître.*

**Conversion d'une peine de prison ferme de M mois** (une fois les réductions habituelles appliquées) **[toutes valeurs à calibrer]** :

| Conséquence | Règle |
|---|---|
| **Casier** | Inscription normale |
| **Amende équivalente** | `M × revenu mensuel de référence × 0,5`, où le revenu de référence est **le plus grand** du revenu de référence de l'économie ([economie.md](economie.md), §7.8 : revenu net moyen des trois derniers mois ou salaire minimum) et du **patrimoine net connu ÷ 120** (patrimoine déclaré et biens saisis) (décidé, E22 : *pourquoi : une condamnation lourde doit coûter plus qu'une taxe à un joueur riche qui ne déclare pas de revenu*) ; recouvrée par le mécanisme du §8.6, en plus des amendes et dommages et intérêts prononcés |
| **Saisies et confiscations** | Celles prononcées par le tribunal, exécutées aussitôt |
| **Désentraînement** | Baisse d'attributs **immédiate**, égale au désentraînement de M mois de détention selon le modèle de la progression réaliste, plafonnée à celle de 6 mois (décidé : *pourquoi : c'est la même logique que la convalescence théorique du socle ; la détention est moins immobilisante qu'un lit d'hôpital, et le plafond évite qu'une longue peine rende le personnage inutilisable*) |
| **Emploi** | Perdu si M ≥ 1, comme pour un habitant détenu ; interdiction d'exercer si elle est prononcée |
| **Groupes** | Il perd ses postes de responsabilité (bande, association, entreprise : son suppléant le remplace) |
| **Interdictions** | Interdiction de séjour dans le **quartier des faits** et de contact avec la victime, pendant M mois réels (au moins 1) ; les autres interdictions prononcées s'appliquent aussi en temps réel |
| **Délai d'épreuve** | Pendant `2 × M` mois réels (au moins 6) : toute nouvelle condamnation est jugée en **récidive** (peine doublée) |
| **Surveillance électronique** | Si **M ≥ 6** : pendant `min(M, 3)` mois réels, **quand il est connecté**, il doit être dans son lieu de retrait de **22 h à 6 h** (heure du serveur) et ne peut quitter l'agglomération qu'en déclarant son séjour ; une violation est un délit jugé en **récidive**. Absent, il est réputé chez lui. Dans le monde, c'est le bracelet électronique ordinaire (§8.4) (décidé, E22 : *pourquoi : une longue peine doit contraindre aussi un joueur organisé ; la règle « pas de prison en temps réel » est conservée, car c'est une contrainte d'horaires qui n'immobilise pas le personnage et ne vaut que pendant le temps connecté*) |

- **Dans le monde**, la condamnation du personnage persistant est connue comme une **peine aménagée** (sursis probatoire, jours-amende, interdictions), quel que soit son quantum ; personne ne s'étonne de le voir libre (décidé). *Pourquoi : comme sa disparition à la déconnexion, c'est une convention que le monde ne remarque pas ; la chronique judiciaire et la rumeur font connaître la condamnation, et la réputation fait le reste.*
- **Non-respect d'une interdiction** : il est constaté si un policier le contrôle ou le reconnaît dans le lieu interdit, ou si la victime le signale ; c'est un délit, jugé en récidive.
- **Contrôle judiciaire** au lieu de la détention provisoire : interdictions et **caution** (somme bloquée sur son compte, rendue à l'issue s'il les a respectées).
- **Abandon du compte** : la procédure d'abandon de l'économie ([economie.md](economie.md), §11.6) prend le pas ; une condamnation par défaut reste au casier pour un éventuel retour.
- **Bilan du compte** : une condamnation n'enlève **aucune expérience** en elle-même : le jeu ne fait pas la morale. Les crimes du personnage persistant modifient en revanche le **profil comportemental** du joueur ([04-joueur.md](../04-joueur.md), §2), ce qui change les personnages qu'il peut incarner.

---

## 10. La criminalité

### 10.1 Pourquoi et quand un personnage commet un crime (décidé)

*Pourquoi : la criminalité doit naître des personnages (principe 7) ; la théorie des « activités routinières » (un auteur motivé, une cible, l'absence de gardien) se traduit directement en préconditions et en coût de recettes.*

Un crime est une **recette** de planification ([ia-pnj.md](ia-pnj.md), §3.3), choisie comme les autres :

| Élément | Traduction |
|---|---|
| **Motivation** | Un besoin ou une motivation qui la rend utile : argent (motivation argent, dettes), **manque** d'une dépendance, statut (monter dans la bande), vengeance (projet), devoir envers le groupe (ordre du chef), stimulation (adolescents), désespoir (besoin critique) |
| **Profil** | Le coût de la recette est celui de l'**agrégat des effets de ses étapes** (décidé, G8, G9) : la recette n'ajoute aucun effet propre, sa signature est calculée à la compilation du paquet, et les gestes d'un même crime se cumulent dans l'épisode, si bien que le découper en petits gestes ne coûte pas moins. Au-delà d'un seuil, elle est écartée, sauf **moyens désespérés** ([ia-pnj.md](ia-pnj.md), §1.4) |
| **Occasion** | Une **cible** connue du personnage (une caisse pleine le soir, une maison dont il sait les occupants en vacances, une voiture ouverte) et des **moyens** (outils, compétence, complices, véhicule) |
| **Risque perçu** | Estimé avec ce que **sait** le personnage : surveillance du lieu ([ville.md](ville.md), §9.1), passage, éclairage, présence policière vue ou connue, témoins probables, gravité de la peine, son propre casier. Un personnage prudent (organisé, anxieux) renonce plus vite |

- Les **savoirs** comptent : savoir qu'un voisin est en vacances ([ia-pnj.md](ia-pnj.md), §9.1), qu'un commerçant dépose sa caisse le samedi, qu'une villa a une alarme, est un savoir qui rend une cible accessible. Les rumeurs alimentent donc la criminalité.
- **Préparation** : un crime important (cambriolage d'une bijouterie, racket d'un nouveau commerce) est un **projet**, individuel ou collectif ([social.md](social.md), §6.9) : repérage, recrutement de complices, achat de matériel, rendez-vous, exécution, partage, recel.

### 10.2 Bandes, territoires et conflits (décidé)

*Pourquoi : le social a fixé ce qu'est une bande et son territoire ([social.md](social.md), §6.6) et laisse à ce domaine les affrontements, les sanctions internes et la réponse policière.*

**Les trois bandes et le réseau** de départ ([population.md](population.md), §5.5) sont **confirmés**, avec leurs activités. Le **milieu criminel** représente ≈ 2 % des adultes au départ, membres des bandes et du réseau ou indépendants, jamais tirés selon l'origine.

**Escalade d'un conflit de territoire** entre bandes rivales, une étape après l'autre, chaque étape étant décidée par les chefs (ou leurs suppléants) selon leur profil, leurs moyens et la loyauté de leurs membres :

| Étape | Exemples | Effets |
|---|---|---|
| 1. Provocation | Tags recouverts, insultes, présence sur le territoire de l'autre | Rumeurs, sécurité perçue en baisse |
| 2. Intimidation | Menaces contre des revendeurs, filatures, véhicule dégradé | Crainte, guetteurs renforcés |
| 3. Bagarres | Affrontements à mains nues ou avec objets entre membres | Blessés, plaintes rares (loi du silence), interventions de police |
| 4. Expédition punitive | Agression d'un membre isolé, d'un point de deal rival, avec armes blanches | Blessés graves, enquête, pression policière forte |
| 5. Règlement de comptes | Usage d'arme à feu | **Rare** **[à calibrer]** ; mort possible ; mobilisation de toute la police ; peur dans tout le quartier |

- **Désescalade** : trêve, accord de partage (alliance décidée par le social), médiation d'une figure respectée, intervention du réseau des Quais qui fournit les deux bandes et n'a pas intérêt à la guerre, ou défaite nette d'un camp.
- **Contrôle du territoire** : chaque incident change le score de contrôle de chaque bande (présence, crainte, respect, absence de rivaux, [social.md](social.md), §6.6).
- **Sanctions internes** : une bande punit les manquements (dette envers la caisse, marchandise perdue, désobéissance, trahison) selon ses normes et le profil de son chef : avertissement, amende, exclusion, **violence** (coups), et pour l'étiquette « **balance** », une violence grave. Ces violences sont des infractions comme les autres, rarement dénoncées.
- **L'ordre de frapper** (décidé, [social.md](social.md) §6.7 ; [revues/signatures.md](../revues/signatures.md) G14) : le chef qui ordonne une violence (sanction interne, représailles, racket) commet une **instigation**. Sa signature reprend celle du tort commandé, pondérée **[à calibrer]**, et il en paie le coût comme s'il agissait lui-même. Pour la justice, l'instigation est une **complicité** (§8.1), poursuivie si le dossier établit l'ordre : un témoignage, l'aveu de l'exécutant ou des relevés téléphoniques (§6.3). L'exécutant reste auteur ; l'ordre reçu n'atténue son acte que dans la mesure prévue par la justification d'ordre (J5) pour un personnage respectueux de la hiérarchie. *Pourquoi : c'est la règle du droit réel, et sans elle un chef de bande ne répondrait jamais des violences qu'il commande.*
- **Réponse de la police** : après un incident grave, le commissaire décide d'une **pression** sur le territoire (patrouilles, brigade anticriminalité, contrôles) ; la sûreté peut monter une **opération** contre un point de deal ou un chef : c'est un **projet collectif** de la police ([social.md](social.md), §6.9) : surveillance, filatures, réquisitions, informateurs, puis interpellations simultanées et perquisitions. Sous la pression, les points de deal **se déplacent** ([social.md](social.md), §6.6) ; une opération réussie désorganise la bande (chef incarcéré, succession disputée, [social.md](social.md), §6.7).

### 10.3 Les trafics (décidé)

*Pourquoi : l'économie fixe les flux, les prix et les chaînes ([economie.md](economie.md), §7.3) ; ce domaine fixe les risques à chaque maillon.*

| Maillon | Risque principal | Infraction |
|---|---|---|
| Consommateur | Contrôle avec produit sur soi | Usage (amende forfaitaire) |
| Guetteur, revendeur au point de deal | Flagrant délit lors d'une surveillance ou d'un passage de la brigade anticriminalité | Revente |
| Nourrice (garde le stock chez elle) | Perquisition après dénonciation ou filature | Détention en vue de la revente |
| Chef de point, de bande | Enquête longue : informateurs, filatures, relevés téléphoniques, train de vie | Trafic, association |
| Réseau des Quais | Enquête financière, blanchiment | Trafic organisé, blanchiment |
| Armes | Contrôle, perquisition, arme retrouvée après un tir | Détention, cession, trafic d'armes |
| Pièces et véhicules volés | Contrôle d'un garage, plaque signalée | Recel, vol de véhicules |
| Jeux clandestins (arrière-salle d'un bar, bande ; [economie.md](economie.md), §7.6) | Dénonciation d'un perdant endetté ou d'un concurrent, rumeur, bagarre à la table, contrôle d'un bar | Tenue de jeux clandestins, travail dissimulé, blanchiment |

- Une **saisie** de stock ou d'espèces est une vraie perte pour la bande (caisse de groupe, [economie.md](economie.md), §7.6), qui peut provoquer des dettes internes et des violences.
- Les **adolescents** recrutés comme guetteurs ou coursiers ([social.md](social.md), §6.6) n'ont aucun acte d'arme (§12.2) ; leur arrestation relève de la justice des mineurs (§8.8).
- **Jeux clandestins** (décidé, v0.6, [economie.md](economie.md) §7.6) : pour une bande, c'est une **activité** comme un point de deal, avec une table, un lieu et des habitués. Les **dettes de jeu** d'un perdant sont des dettes envers la bande, recouvrées comme celles de l'usure (§10.4) : menaces, puis violences, ou obligation de rendre un service. Une partie qui tourne mal (accusation de tricherie) est une source ordinaire de bagarres. *Pourquoi : les jeux sont une source d'argent liquide et un moyen de tenir des débiteurs ; ce sont les dettes et les violences qui les font connaître de la police.*

### 10.4 Racket et usure (décidé)

*Pourquoi : le social a fixé les cibles et les réactions ([social.md](social.md), §6.6), l'économie les flux ([economie.md](economie.md), §7.6) ; ce domaine fixe l'escalade de la contrainte et sa répression.*

**Escalade de la contrainte** envers un commerçant qui refuse de payer ou un débiteur en défaut **[délais à calibrer]** :

1. menace verbale, insistance, visites répétées ;
2. **dégât** (vitrine brisée, tag, véhicule rayé), qui sert d'avertissement ;
3. **agression** légère ;
4. agression grave ou incendie du local : rare, réservée aux chefs violents.

- Chaque étape est un acte décidé par son auteur selon son profil ; un racketteur peu violent s'arrête à la menace.
- Le commerçant ou le débiteur peut **porter plainte** à chaque étape (§4.5) ; la plainte ouvre une affaire prioritaire si elle est soutenue par des témoins ou par l'association des commerçants (résistance collective, [social.md](social.md), §6.6).
- Un **paiement** régulier laisse peu de traces (espèces) : l'enquête repose surtout sur les témoignages et sur l'écart entre recettes et dépenses du commerce lors d'un contrôle fiscal.

### 10.5 Cambriolages (décidé)

*Pourquoi : c'est le crime contre les biens le plus marquant pour les habitants, et celui qui relie le mieux savoirs, effraction, recel et assurance.*

1. **Cible** : un logement ou un commerce que le cambrioleur sait vide (vacances, horaires de travail, personnage persistant souvent absent — ses absences se remarquent, [ville.md](ville.md), §2.5) et pas trop surveillé.
2. **Repérage** : observer les habitudes, les fenêtres allumées, le courrier qui s'accumule (perception et mémoire).
3. **Entrée** : crochetage, forçage, vitre (§3.1) ; une alarme le fait fuir ou le presse.
4. **Butin** : espèces, objets de valeur à **propriétaire légitime** ([economie.md](economie.md), §5.10), armes de chasse, clés de véhicule.
5. **Fuite** et **recel** (§10.7).

- Si les occupants rentrent pendant le cambriolage, ou s'il y avait quelqu'un : fuite, menace, ou violence selon le profil du cambrioleur ; cela devient un **vol avec violence**, bien plus grave.
- **Suites pour la victime** : dégâts, plainte (nécessaire pour l'assurance), sentiment d'insécurité (besoin de sécurité, projet d'installer une alarme ou de déménager), rumeur dans l'immeuble.

### 10.6 Les autres vols (décidé)

*Pourquoi : chaque forme de vol oppose des attributs différents, ce qui donne des rôles variés aux voleurs comme à ceux qui les surveillent.*

| Vol | Résolution | Surveillance |
|---|---|---|
| **À la tire** (pickpocket) | Dextérité et compétence du voleur contre perception et attention de la victime ; foule, bus, marché | La victime peut s'en apercevoir sur le moment ou plus tard |
| **À l'arraché** | Vitesse, surprise ; la victime peut tomber (blessure) | Témoins, caméras |
| **À l'étalage** | Discrétion contre attention des employés et du vigile ; caméras | Le vigile retient le voleur (§1.9) |
| **Vol de caisse** (par un employé) | Accès à la caisse ; écart de caisse constaté au comptage | Le gérant soupçonne ; enquête interne, plainte, renvoi pour faute grave ([economie.md](economie.md), §3.6) |
| **Vol de marchandises** | Camion, entrepôt, chantier ; souvent avec un complice interne | Gardiens, inventaires |
| **Vol de véhicule** | §3.3 | Signalement, plaques |

### 10.7 Recel et prêteur sur gages (décidé)

*Pourquoi : sans recel, le vol ne rapporte rien ; contrôler les receleurs est donc l'un des leviers les plus efficaces de la police.*

- Le voleur écoule auprès d'un **receleur** (indépendant ou bande), à prix cassé ([economie.md](economie.md), §7.4). Le receleur prend le risque de garder des objets **signalés**.
- **Contrôle du prêteur sur gages** : chaque mois **[à calibrer]**, un enquêteur vérifie son registre (identités des déposants, objets) contre les objets signalés. Un prêteur honnête prévient lui-même la police ; un prêteur complice falsifie son registre, ce qui est un délit de recel et peut lui coûter son commerce.
- Un objet signalé retrouvé chez quelqu'un fait monter la certitude de recel, ou de vol s'il n'explique pas sa provenance.

### 10.8 Fraudes, corruption et blanchiment (décidé)

*Pourquoi : l'économie fournit les traces (grand livre, signalements, contrôles) et les montants ; il fallait les relier à des infractions et à des enquêtes.*

| Fraude | Comment elle est découverte |
|---|---|
| **Escroquerie** (mensonge réussi pour obtenir de l'argent ou un bien) | Plainte de la victime quand elle comprend ; témoins du mensonge ; virement tracé |
| **Abus de faiblesse** (personne âgée, malade, vulnérable) | Proches qui s'inquiètent, banque qui signale des virements inhabituels, services sociaux |
| **Fraude fiscale**, travail dissimulé | Contrôle fiscal, inspection du travail, dénonciation (§6.6) |
| **Fraude à l'assurance** (faux vol, incendie volontaire) | Signalement de l'assureur, incohérences de l'enquête (pas d'effraction, accélérant, difficultés financières du déclarant) |
| **Fraude aux prestations** | Croisement des déclarations, dénonciation, train de vie |
| **Corruption** d'un agent public, d'un policier, d'un élu ([social.md](social.md), §6.11) | Dénonciation (rival, subordonné honnête), train de vie du corrompu, virement tracé, décision anormale (permis accordé contre l'avis du service, enquête qui s'arrête sans raison) |
| **Achat de secrets** | Plainte de l'entreprise volée, utilisation du secret qui trahit la fuite (coffre ouvert sans effraction : enquête sur ceux qui connaissaient le code) |
| **Blanchiment** | Signalement bancaire, contrôle fiscal du commerce de façade, enquête patrimoniale (§6.6) |

### 10.9 Repères de calibrage (décidé)

*Pourquoi : sans taux imposé (principe 7), il faut tout de même des repères pour régler les seuils des recettes et les taux de la simulation statistique.*

Pour 10 000 habitants et par an, sans compter les actes des joueurs **[toutes valeurs à calibrer]**, d'après les ordres de grandeur d'une ville moyenne européenne, un peu relevés pour les violences graves :

| Fait | Commis | Connus de la police |
|---|---|---|
| Morts violentes intentionnelles | 1 à 3 | Toutes |
| Violences physiques | ≈ 150 | ≈ 60 |
| Vols avec violence | ≈ 15 | ≈ 12 |
| Cambriolages de logements | ≈ 40 | ≈ 35 |
| Vols de véhicules et dans les véhicules | ≈ 60 | ≈ 45 |
| Vols simples (dont à l'étalage) | ≈ 400 | ≈ 150 |
| Dégradations, tags | ≈ 300 | ≈ 80 |
| Affaires de stupéfiants | — | ≈ 40 |
| Escroqueries, fraudes | ≈ 80 | ≈ 40 |

- Le nombre de morts violentes est volontairement plus élevé que la réalité d'une ville de cette taille (≈ 0,1 à 0,2 par an) (décidé). *Pourquoi : avec trois bandes et un réseau, quelques drames par an rendent le milieu crédible ; ils restent assez rares pour qu'un meurtre soit un événement dont toute la ville parle.*
- Un **tableau de bord de la criminalité** (faits commis, faits connus, élucidations, par quartier) est tenu pour le responsable conception, la chaîne de calibrage et l'exploitation, sans que personne ait à le consulter, comme celui de l'économie ([economie.md](economie.md), §10.3). Il ne pilote rien directement : seule la **dotation de postes de police** réagit à une hausse durable (§5.1).

---

## 11. Règles pour les joueurs

### 11.1 Se défendre (décidé)

*Pourquoi : se défendre est conforme à la nature de presque tout le monde ; la synchronisation doit le refléter, tout en distinguant la riposte de l'acharnement.*

- Le **coût** d'un geste violent dépend du contexte par deux règles générales, sans aucune variante écrite (décidé, G5, G4) :
  - **J1, légitime défense**, et **J2, défense d'autrui** (0,9 pour un intime ou un enfant, 0,7 pour un inconnu **[à calibrer]**), réduisent fortement la gravité du tort physique, de la force et de la contrainte, et effacent la violation de la loi pour la part proportionnée ([revues/signatures.md](../revues/signatures.md), §3.6) ;
  - le **plafond de proportionnalité** limite cette réduction à la gravité de la menace : riposter à une gifle par une gifle ne coûte presque rien ; sortir une arme face à des poings garde tout l'excès au-delà de la menace ;
  - **poursuivre un agresseur qui fuit** n'est plus couvert, faute d'agression actuelle ;
  - **s'acharner** sur quelqu'un au sol frappe une cible de classe **vulnérable** (× 1,3 **[à calibrer]**), sans justification puisque la menace a cessé.

  *Pourquoi : deux règles et un plafond disent ce que dit le droit (riposter, oui ; en faire plus que nécessaire, non) pour tous les gestes présents et futurs.*
- **Imputabilité** (décidé, G6) : **provoquer pour pouvoir se défendre ne justifie rien**. Une justification ne vaut pas si l'acteur a lui-même créé la situation dans le même épisode : celui qui a provoqué la bagarre n'est pas en légitime défense, comme en droit. Pour un pilote `Joueur`, la garde hors fiction neutralise aussi une agression ou une provocation causée par un personnage de son **cercle** dans les 30 jours de jeu (§11.8), et ignore l'ivresse ou la colère qu'il a lui-même entretenues pendant l'incarnation. *Pourquoi : des règles de contexte générales sont exactement ce qu'un joueur chercherait à mettre en scène.*
- Pour un personnage **pacifiste ou anxieux**, fuir, appeler à l'aide ou céder est plus fidèle que se battre ; un joueur qui le fait se battre perd un peu de synchronisation, sauf pour protéger un proche (réflexe de protection, [ia-pnj.md](ia-pnj.md), §8.4).
- La **justice** du monde, elle, juge la légitime défense sur ce qu'elle sait (§8.3) : un joueur qui s'est vraiment défendu peut être poursuivi si personne ne l'a vu, et l'inverse.

### 11.2 Blesser et tuer pendant une incarnation (décidé)

*Pourquoi : le socle empêche de tuer le personnage incarné par la désynchronisation, mais pas de lui faire tuer quelqu'un d'autre ; il faut que la nature du personnage reste le frein principal, et que les suites soient réelles.*

- **Escalade de signatures** : l'escalade (menacer, frapper, prendre une arme, frapper avec, viser, tirer, s'acharner) est une **échelle déclarée** dans les données, dont chaque marche **domine** la précédente sur chaque couple (axe, sens) après dérivation, ce qui rend son coût **au moins égal** pour tout profil et toute cible ; cette **dominance** est vérifiée automatiquement par le contrôle de cohérence du paquet (décidé, G16, G22, M11). « **Insulter** » ne fait pas partie de cette échelle : il relève de l'**échelle de la parole** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md) §8 ; [revues/signatures-contenu.md](../revues/signatures-contenu.md), M11). *Pourquoi : insulter et menacer tirent des axes différents ; aucune règle honnête ne rend l'un toujours pire que l'autre.* Un personnage doux perd sa synchronisation **pendant** l'escalade, et le joueur est **toujours éjecté avant** l'acte irréparable : un acte dont le coût de synchronisation dépasse la synchronisation restante **n'a pas lieu** (**point de rupture**, décidé, E1). Le personnage hésite puis renonce (il baisse le poing, repose l'arme), le joueur est éjecté avec le bilan d'une éjection, et l'IA reprend dans le même tick et désamorce selon ses réflexes ([ia-pnj.md](ia-pnj.md), §5.3). Un homicide ou une violence grave **à contre-emploi** coûte plus que la synchronisation maximale : il est impossible même à synchronisation parfaite. C'est le **plancher R1** (décidé, G7) : un tort physique de gravité attendue grave ou mortelle, après justifications non imputables, coûte plus que 1 pour un personnage peu violent ou bienveillant, et pour presque tous les profils envers une victime inconnue sans aucune justification ; il est évalué sur le **profil de base**, jamais sur le profil effectif (ivresse, colère) ([revues/signatures.md](../revues/signatures.md), §3.8). *Pourquoi : ce qu'une pénalité ne peut pas réparer (la mort d'un habitant) doit être empêché, pas seulement puni, et une règle sur l'effet vaut pour tous les gestes présents et futurs.*
- **Provoquer pour pouvoir se défendre ne justifie rien** (imputabilité, G6, §11.1) : la riposte à une agression qu'on a soi-même provoquée garde tout son coût, et R1 s'applique comme sans justification.
- Un personnage **violent** (membre de bande, profil hors-la-loi) peut aller plus loin sans se désynchroniser : c'est sa nature. Mais tuer un inconnu sans motif a une signature extrême (cruauté), éloignée de presque tous les profils, même violents.
- **Après la reprise** : le personnage se souvient de tout ; un acte dissonant grave lui laisse un souvenir **dissonant** ([ia-pnj.md](ia-pnj.md), §4.5). Selon son profil, il peut se cacher, fuir, se justifier, ou **se rendre à la police** et avouer (recette nouvelle demandée à l'IA, §16) ; un personnage légaliste et honnête, rongé par un meurtre qu'il n'aurait jamais commis, finit souvent par se dénoncer. L'acte devient un **fait sensible** (§6.4) : la **honte** de l'acte ([ia-pnj.md](ia-pnj.md), §13.1), qui peut devenir un thème de vie (ia-pnj §13.4), et la **peur d'être dénoncé** par ceux qu'il croit témoins (ia-pnj §13.5) pèsent ensemble sur le choix entre se taire, faire pression et **avouer avant d'être découvert** (décidé, [ia-pnj.md](ia-pnj.md) §13.5). Les suites judiciaires sont **réelles** et portent sur lui : il paie pour ce que le joueur a fait.
- **Bilan** : un homicide ou une violence grave commis à contre-emploi est traité comme une **désynchronisation volontaire grave** dans le bilan d'incarnation ([04-joueur.md](../04-joueur.md), §1.1) (décidé ; impact sur le socle, §16). *Pourquoi : le personnage, et la ville, supportent pour des années les conséquences d'un acte que le joueur a imposé ; la pénalité doit être à la hauteur.* Avec le point de rupture, cette règle ne vise plus que les violences graves qui passent **sous** le seuil (personnage peu violent mais pas doux, synchronisation élevée). Une désynchronisation volontaire grave coûte, en plus de l'expérience de l'incarnation, **25 %** de l'expérience requise pour le niveau actuel, **doublés** à chaque récidive dans les 30 jours réels, et le niveau peut baisser (décidé, E3) **[valeurs à calibrer]**. *Pourquoi : un joueur qui force chaque jour des personnages contre leur nature ne joue plus un rôle, et son niveau doit le refléter.*
- Un acte violent **fidèle** à la nature du personnage (un chef de bande qui règle un compte) n'a pas de pénalité : le jeu ne fait pas la morale. Ses suites (police, vengeance, prison) suffisent.

### 11.3 Mourir pendant une incarnation (décidé)

*Pourquoi : le socle distingue la mort fidèle (aucune pénalité) de la mort par imprudence ou à contre-emploi (perte de l'expérience) ; il faut un critère calculable.*

- La mort d'un personnage jouable est **définitive** (socle) ; l'incarnation se termine avec son bilan, et les familiarités obtenues grâce à lui sont perdues.
- **Critère d'imprudence** : l'IA en observation calcule, à chaque moment clé de l'affrontement (début, escalade, arme sortie, blessure), le **réflexe et la décision qu'elle aurait choisis** (§1.8). La mort est jugée **imprudente** si le joueur a engagé, prolongé ou aggravé un affrontement que le personnage aurait fui ou désamorcé, ou s'il a négligé une issue évidente (soins, fuite, reddition) que le personnage aurait prise. Sinon, la mort est **fidèle** : aucune pénalité.
- Exemple : un retraité incarné qui s'interpose pour protéger son petit-fils et meurt sous les coups meurt **fidèlement** ; le même qui provoque trois hommes armés meurt **imprudemment**.
- **Morts imprudentes répétées** (décidé, E28) **[valeurs à calibrer]** : la première dans les 30 jours réels coûte l'expérience de l'incarnation ; la deuxième, en plus, **10 %** de l'expérience du niveau ; la troisième et les suivantes, **25 %**, avec un signalement à la détection de l'abus du monde partagé (§11.6). Une mort imprudente pendant la période probatoire d'un compte compte double. *Pourquoi : la mort définitive d'un habitant prive tout le serveur, et la répétition est la signature du sabotage.*
- Pour un personnage **prudent**, provoquer une violence mortelle évidente (insulter un homme armé, se jeter dans une bagarre au couteau) atteint le **point de rupture** (§11.2, E1) : l'acte n'a pas lieu et le joueur est éjecté (décidé). C'est le **plancher R4** (décidé, G7) : un risque pour soi dont la probabilité de mort attendue atteint 0,2 **[à calibrer]**, alors que le personnage connaît une issue sans risque, coûte plus que 1 pour un profil prudent (stabilité basse ou bon jugement) ([revues/signatures.md](../revues/signatures.md), §3.8). *Pourquoi : la mise en danger mortelle évitable d'un personnage prudent est un acte extrême pour son profil, comme l'homicide pour un personnage doux.*

### 11.4 Le personnage persistant qui commet un crime (décidé)

*Pourquoi : le personnage persistant n'a pas de synchronisation ; ses crimes doivent avoir des suites dans le monde, et peser sur le profil du joueur, pas sur une note morale.*

- Il est **un habitant comme un autre** pour les témoins, la police et la justice : plainte, enquête, interpellation, procès, avec les conséquences équivalentes en ligne (§9.5) et la détention en solo (§9.4).
- Ses actes **construisent le profil** du joueur ([04-joueur.md](../04-joueur.md), §2) : une série de violences rapproche le joueur des catégories « violentes » et l'éloigne des personnages doux, dont il ne remplit plus la condition de profil.
- **Recherché** : s'il fait l'objet d'une fiche de recherche, la police peut l'attendre à son domicile ; à la reconnexion, le joueur peut choisir de réapparaître à la gare ([social.md](social.md), §9.4), mais la gare est un lieu où la police patrouille, et l'affaire suit son cours (jugement par défaut, §9.5).
- **Mourir n'efface rien** : réveillé à l'hôpital (l'hôpital public sous garde s'il était interpellé, poursuivi ou recherché, §11.5, E21), il reste poursuivi ; ses dettes et amendes restent dues ([economie.md](economie.md), §11.6).

### 11.5 Se déconnecter pendant un affrontement (décidé)

*Pourquoi : la ville fixe un délai d'environ une minute pour qu'on ne fuie ni un combat ni une arrestation en se déconnectant ([ville.md](ville.md), §2.5) ; ce domaine en précise le comportement et les cas limites.*

**Personnage persistant** :

- Pendant le **délai d'environ une minute** **[à calibrer]**, son corps est piloté par un **automate défensif** minimal : il se protège, recule, se rend à la police si elle l'interpelle, ne riposte jamais et ne fuit pas en courant (décidé). *Pourquoi : se déconnecter doit toujours être désavantageux en plein affrontement ; un automate qui se battrait bien deviendrait une tactique.*
- S'il est **encore menacé** à la fin de la minute (affrontement en cours, poursuivi, interpellé, en garde à vue), sa disparition est **retardée** jusqu'à la fin de la menace, dans la limite d'un plafond **[≈ 5 minutes, à calibrer]**, au-delà duquel il est réputé s'être rendu ou avoir perdu connaissance, selon la situation.
- **La menace retarde la disparition partout, lieux de retrait compris** (décidé, E19) : tant qu'il est menacé (affrontement, poursuite, interpellation en cours, policier qui vient pour lui avec un motif, fiche de recherche et policier en vue), franchir sa porte, monter dans un transport ou atteindre une sortie de la carte ne le fait **pas** disparaître ; il reste dans le monde (derrière sa porte, dans le bus, à la sortie), piloté par l'automate défensif, jusqu'à la fin de la menace ou au plafond. Un **véhicule qui ne lui appartient pas** ne quitte jamais la carte avec lui : il est retrouvé à la sortie. *Pourquoi : un lieu de retrait est l'endroit où l'on rentre chez soi, pas une porte de secours en pleine poursuite.*
- **Interpellé** pendant ce délai : garde à vue accélérée (§7.2), sans lui.
- **État critique** (saignement fort, choc, inconscience) : il ne disparaît **jamais** ; son sort est simulé jusqu'au bout (secours, hôpital, survie ou mort) avec les règles normales, connecté ou non (décidé, E21).
- **Tué** pendant ce délai : règles normales de la mort du personnage persistant (socle), avec une exception : tué ou gravement blessé alors qu'il était **interpellé, poursuivi par la police ou recherché**, il se réveille à l'**hôpital public** sous la garde de la police (la clinique privée n'est pas proposée), et la garde à vue accélérée commence à sa reconnexion (décidé, E21). Cette exception vaut aussi hors de tout délai de déconnexion. *Pourquoi : mourir ne doit être ni une évasion, ni une façon d'échapper à la procédure.*

**Personnage jouable incarné** :

- L'IA reprend **dans le même tick** avec ses réflexes ([ia-pnj.md](ia-pnj.md), §5.3) : le personnage se défend ou fuit selon sa nature.
- Le **bilan** est calculé à la déconnexion, mais si le personnage **meurt des suites du même affrontement** dans les **10 minutes** **[à calibrer]**, sa mort est comptée dans le bilan de cette incarnation, imprudence comprise (décidé). *Pourquoi : sinon, se déconnecter juste avant de mourir effacerait la pénalité d'une mort imprudente.*
- Une coupure de réseau est traitée de la même façon : le serveur ne peut pas la distinguer d'une déconnexion volontaire.

### 11.6 Le PvP en ligne, le harcèlement et l'abus du monde partagé (décidé)

*Pourquoi : le social distingue la violence dans le jeu, traitée par la police du monde, du harcèlement d'une personne réelle, traité par la modération ([social.md](social.md), §9.4) ; il faut y ajouter la destruction gratuite d'un monde que d'autres partagent.*

- **Violence entre personnages pilotés par des joueurs** : elle est **permise** et se règle **dans le monde** (police, justice, réputation, vengeance). Le monde ignore qui pilote qui ; aucun lieu n'est « zone protégée » (décidé). *Pourquoi : une zone protégée signalerait les joueurs et contredirait un monde qui ne les connaît pas ; la police, les témoins et les lieux de retrait protègent déjà.*
- **Harcèlement** (traité par la modération, [social.md](social.md), §9.4) : le même compte qui s'en prend de façon répétée au personnage persistant d'un même joueur, quels que soient les personnages qu'il utilise ; s'y ajoute, dans ce domaine, la **chasse aux nouveaux venus** : un compte qui agresse ou tue de façon répétée des personnages persistants **différents** peu après leur arrivée à la gare est signalé automatiquement (décidé). *Pourquoi : c'est la forme de nuisance la plus probable contre les nouveaux joueurs, et elle échappe à la détection de l'acharnement, qui vise une seule victime.*
- **Abus du monde partagé** (décidé) : les habitants sont de vrais personnages, et leur mort est **définitive** pour tous les joueurs du serveur. Une série de **violences gratuites** commise par un même compte (plusieurs morts ou blessés graves sans lien entre eux en peu de temps **[seuils à calibrer]**) n'est plus seulement un crime dans le monde : c'est un **abus**, signalé automatiquement à la modération avec la capture du journal, et sanctionné sur la **note de conduite** du compte : les restrictions légères et temporaires sont automatiques ; une suspension ou une exclusion est décidée par une **session de modération** distincte de la détection, sur dossier, journalisée et contestable (décidé, A8, S36). *Pourquoi : un joueur sans synchronisation (personnage persistant) pourrait sinon détruire des vies que des centaines d'autres joueurs connaissent ; la note de conduite, distincte du profil, est faite pour cela.* S'y ajoutent, commis **par incarnation** (décidé, E3, E4, E28) **[seuils à calibrer]** : les **refus répétés du point de rupture** (trois dans la même journée réelle, §11.2), les **désynchronisations graves répétées** (trois en 30 jours réels), les **morts imprudentes répétées** (§11.3) et les **facilitations suivies d'un crime** (ouvrir une porte, remettre une clé, couper une alarme, dont un crime profite ensuite). La **note de conduite survit à la réinitialisation** du compte, avec les sanctions, les blocages et l'historique des abus (décidé, E23). *Pourquoi : on peut abîmer le monde partagé à travers des habitants incarnés autant qu'avec son persistant, et la réinitialisation est un nouveau départ dans le monde, pas l'effacement d'un comportement.*
- Les autres freins restent ceux du monde : armes rares, police armée, victimes qui fuient, témoins, réveil à l'hôpital avec malus, conséquences judiciaires, profil qui dérive vers la cruauté.
- **Ce que perd la victime** : ce qu'elle portait (espèces, objets) peut être volé ; ses comptes et ses biens sont intacts ([economie.md](economie.md), §11.6).
- **Solo** : il n'y a pas de modération ; le joueur fait ce qu'il veut dans son propre monde, dans les seules limites de contenu du §12.

### 11.7 Jouer un policier, un juge, un avocat, un criminel (décidé)

*Pourquoi : quasiment tous les personnages sont jouables ; les métiers de ce domaine doivent l'être aussi, avec leurs pouvoirs, leurs obligations et leurs objectifs, sans devenir des outils d'abus.*

- **Policier** : patrouiller, répondre aux appels, enquêter, interroger, interpeller, avec les pouvoirs et les limites du §5.5. Les tâches du métier et les étapes d'une enquête sont des **objectifs du personnage** qui rapportent de l'expérience ([04-joueur.md](../04-joueur.md), §1.1). Un abus de pouvoir (violence gratuite, fouille sans motif, consultation d'un fichier hors de toute affaire) est une faute ou un délit dans le monde, et désynchronise un policier légaliste. La consultation d'un fichier concernant le **personnage persistant d'un autre joueur** hors de toute affaire est en outre **journalisée** pour la modération (décidé ; *pourquoi : c'est le moyen le plus direct de retrouver le domicile d'un joueur pour le harceler*).
- **Juge** : conduire une audience et décider dans la fourchette légale (§8.3) ; le procureur, hors catégorie, n'est jamais incarné.
- **Avocat** : défendre, conseiller en garde à vue, négocier un plaider-coupable ; ses honoraires sont son revenu.
- **Criminel** : guetter, revendre, cambrioler, receler, selon sa fiche de rôle ; ces tâches sont aussi des objectifs du personnage et rapportent de l'expérience si elles sont fidèles à sa nature (décidé). *Pourquoi : c'est la règle du socle, qui récompense le fait de bien jouer un personnage, quel qu'il soit.*
- Les **savoirs opaques** du personnage restent opaques (codes, planques, sources) ; le **domicile** d'un personnage persistant figurant dans un fichier est affiché comme une référence (« adresse connue ») sans être révélé au joueur (décidé), même si le personnage peut s'y rendre dans le cadre d'une affaire.

### 11.8 La récusation et le conflit d'intérêts (décidé)

*Pourquoi : c'est la règle des flux liés de l'économie ([economie.md](economie.md), §12.3) appliquée à la police et à la justice : un joueur ne doit pas pouvoir enquêter, juger ou témoigner dans sa propre affaire, ni dans celle d'un ami ou d'un complice, sous un autre visage.*

La récusation devient l'application, dans ce domaine, de la **règle unique de conflit d'intérêts** de la revue d'équilibrage (décidé, E8, A3). Elle porte sur le **cercle** du joueur, donnée hors fiction que le monde ignore :

1. les **personnages liés** au joueur : son personnage persistant et tous les personnages qu'il a incarnés depuis 30 jours de jeu (même définition que l'économie) ;
2. les personnages liés à ses **amis mutuels** ;
3. les personnages liés aux comptes qu'une **session de modération** a **rapprochés** du sien, sur dossier (collusion constatée), sans que les intéressés en soient avertis (décidé, A8 : la session de modération remplace le modérateur humain).

Les **comptes présumés liés** par la détection automatique ([revues/securite.md](../revues/securite.md), S15) n'entrent **pas** dans le cercle : ils ne produisent que des alertes pour la modération (décidé, A3). *Pourquoi : une règle qui retire des actions à un joueur doit reposer sur un lien avéré, jamais sur un soupçon statistique.*

Quand une affaire met en cause, comme suspect, victime ou partie, un personnage du **cercle** du joueur :

- un personnage qu'il **incarne** ne peut faire aucun acte d'enquête ni de justice dans cette affaire (enquêter, interroger, contrôler, juger, délibérer comme juré, requérir) ;
- il ne peut pas non plus faire, comme victime ou témoin, les actes qui orienteraient l'affaire : **retirer une plainte**, **refuser d'identifier** un suspect, **fournir un alibi**, **avouer** (décidé, E8) ;
- dans les deux cas, les intentions ne sont pas proposées et une intuition l'explique (« Vous êtes trop proche de cette affaire. ») ; dans le monde, le personnage se récuse, ou laisse suivre son cours la décision que son IA avait prise ;
- il peut en revanche **témoigner**, mais le témoignage est **construit à partir de la mémoire réelle** du personnage (ce qu'il a perçu), sans mensonge possible ; la police le juge comme les autres (§6.4).

Plus largement, un **aveu** fait par un personnage incarné, dans quelque affaire que ce soit, ne porte que sur des faits de sa mémoire (§7.2). Les autres décisions concernées par la même règle (décisions administratives, votes d'élus, prêts bancaires) relèvent du socle et des domaines concernés (§16).

C'est l'une des règles hors fiction de ce domaine (principe 5).

### 11.9 La mise en danger volontaire (décidé)

*Pourquoi : un joueur pourrait faire mourir un personnage incarné sans désynchronisation, en le jetant d'un toit ; et le jeu ne représente pas le suicide (§12.1).*

- Il n'existe **aucune intention** de mise en danger mortelle évidente de soi : sauter d'un toit ou d'un pont, se jeter sous un véhicule, rester sous l'eau. Le déplacement s'arrête au bord d'un toit, d'un quai ou d'un pont comme devant un mur, sauf pour un geste maîtrisé que le personnage sait faire (descendre une échelle, sauter d'un muret bas).
- Les **chutes accidentelles** restent possibles : poussé pendant une bagarre, glissade sur le verglas, fuite précipitée dans un escalier.

---

## 12. Limites de contenu

### 12.1 Ce qui est exclu (décidé)

*Pourquoi : certains contenus n'apportent rien au jeu et blesseraient des joueurs réels, ou deviendraient des outils de harcèlement ; les exclure par construction est la seule garantie fiable (principe 6).*

| Exclu | Comment |
|---|---|
| **Violences sexuelles**, sous toutes leurs formes | Aucun acte, aucune infraction, aucune recette, aucun souvenir, aucune rumeur de ce type ; le service de dialogue refuse de les formuler ; un joueur qui les évoque contre un autre joueur relève de la modération ([social.md](social.md), §9.4) |
| **Violence physique d'un adulte envers un enfant** | Aucun geste violent n'est possible d'un adulte vers un mineur (§12.2) |
| **Mort d'un enfant du fait d'un personnage** | §12.2 |
| **Torture**, mutilation, acharnement prolongé | Pas d'acte de violence prolongée ; un personnage hors de combat qu'on continue à frapper perd connaissance et l'affrontement s'arrête (§1.3) |
| **Séquestration, enlèvement, prise d'otage** | Pas d'acte pour retenir quelqu'un au-delà de l'immobilisation momentanée (§1.9) et de la garde à vue par la police (décidé : *pourquoi : immobiliser le personnage d'un autre joueur pendant des heures serait un harcèlement, et le sujet n'apporte rien à une ville ordinaire*) |
| **Suicide** | Ni acte (§11.9), ni cause de décès des habitants, ni sujet de dialogue formulé par le service |
| **Empoisonnement criminel** | Pas d'acte d'empoisonner ; les intoxications restent accidentelles (produit dangereux, overdose, monoxyde de carbone) (décidé : *pourquoi : c'est un moyen de tuer indétectable et un outil de nuisance facile contre les joueurs, qui demanderait en plus une toxicologie complexe*) |
| **Violence motivée par l'origine** | Aucune recette ne vise une personne pour son origine ([population.md](population.md), §0, principe 5) ; les propos haineux d'un joueur relèvent de la modération |
| **Armes automatiques, explosifs, attentats** | Absents du monde (§1.5) |
| **Cruauté envers les animaux** | Si des animaux sont simulés plus tard, aucun acte violent envers eux n'existe |

### 12.2 Les enfants (décidé)

*Pourquoi : la règle adulte-mineur du socle couvre la séduction ([02-personnages.md](../02-personnages.md), §3.4) ; il faut la même garantie par construction pour la violence, demandée par [ia-pnj.md](ia-pnj.md) et [social.md](social.md).*

- **Aucun geste violent d'un adulte envers un mineur** n'existe dans le catalogue, ni pour l'IA ni pour un joueur. Face à un mineur, un adulte n'a que des **gestes de contrôle** : repousser, se dégager, retenir par le bras, et, pour un policier, interpeller et menotter un adolescent sans le frapper.
- **Aucune arme ne peut viser un mineur**, et **aucun tir, lancer ou coup n'est proposé si sa trajectoire passerait près d'un mineur** (« pas d'angle »), au lieu d'être résolu comme manqué ; pour la police et l'IA, c'est la consigne réelle de ne pas tirer (décidé, E27). *Pourquoi : un tir résolu comme manqué faisait d'un enfant un bouclier sûr pour un adulte.*
- **Se servir des mineurs** (décidé, E27) : un adulte qui **reste au contact de mineurs** pendant un affrontement, ou qui les met entre lui et une menace, commet une **mise en danger d'un mineur** (infraction, §8.1) ; s'il est piloté par un joueur, il est **signalé automatiquement** à la modération. Aucune intention de **recruter, payer ou faire agir un mineur** pour un délit n'est proposée à un pilote `Joueur` ; les recettes de l'IA qui font des adolescents des guetteurs restent une réalité du monde. Les mineurs **s'éloignent en priorité** d'un affrontement, et les adultes proches les emmènent. *Pourquoi : les règles absolues protègent les mineurs ; elles ne doivent jamais protéger un adulte qui se sert d'eux.*
- **Aucun enfant ne meurt du fait d'un personnage** (joueur ou IA) : un enfant pris dans un accident de la route, une chute ou un incendie est **blessé**, puis soigné. Il n'y a pas non plus de décès d'enfant de cause naturelle dans la première version : les tables de mortalité des mineurs sont nulles (décidé, [population.md](population.md), §7.3) ; **aucun enfant ne meurt dans le monde**, quelle qu'en soit la cause. *Pourquoi : c'est la même règle que l'absence de fausse couche liée aux actions des joueurs ([population.md](population.md), §7.2), pour la même raison.*
- **Entre mineurs** : la violence se limite aux **bagarres** à mains nues, sans arme, aux blessures légères (bagarres de cour, conflits d'adolescents, [social.md](social.md), §7.1). Les mineurs n'ont **aucun acte avec une arme**.
- **Mineurs auteurs** : des adolescents peuvent commettre des vols, des dégradations, des bagarres entre eux, servir de guetteurs ; un adolescent peut voler à l'arraché un adulte, mais ne peut pas le frapper avec une arme. La réponse est celle de la justice des mineurs (§8.8).
- **Négligence** : elle reste traitée par le social ([social.md](social.md), §7.3).
- **Méfiance** envers un adulte inconnu qui aborde un enfant ([social.md](social.md), §7.3) : elle s'applique aussi aux abords des écoles, où la police municipale est présente aux heures d'entrée et de sortie.

### 12.3 Ce que le jeu montre et ne montre pas (décidé)

*Pourquoi : la violence doit être lisible et lourde, pas spectaculaire ; la sobriété sert le réalisme et l'éthique du jeu.*

| Montré | Non montré |
|---|---|
| Les gestes (coups, chute, arme brandie, tir), le **son**, la peur des témoins | Gros plans, ralentis, mises à mort spectaculaires |
| Les blessures **sobrement** : tache sombre, sang en petite quantité, pansements, boiterie, bras en écharpe | Démembrement, chairs, sang abondant, plaies détaillées |
| Un corps au sol, puis **recouvert** d'un drap à l'arrivée des secours | Le corps après la mort, en détail |
| Les **suites** : secours, police, ruban, voisins, journal, deuil, procès | — |
| Les violences entre proches par leurs **signes** (cris étouffés derrière une porte, ecchymoses, départ du domicile, plainte) | Les violences entre proches mises en scène |

- **Réglage d'affichage** « atténuer la violence » (décidé) : il retire le sang et adoucit les sons ; il ne change rien à la simulation. *Pourquoi : chacun doit pouvoir régler son confort sans changer le monde des autres ; la forme relève du domaine interface.*
- **Variantes atténuées et sons sobres** (décidé, [direction-artistique.md](direction-artistique.md), §4.5 et §5.6) : chaque asset de violence (tache de sang, vêtement taché, plaie pansée, son de coup, de chute ou de tir) a une **variante atténuée**, utilisée quand le réglage est actif : pas de sang, vêtement simplement froissé ou sali, son amorti et plus lointain. Le catalogue ne contient **aucun son de blessure détaillé** (craquement d'os, chairs), avec ou sans le réglage. Le contrôle des assets refuse un asset de violence sans sa variante. *Pourquoi : le réglage n'a de valeur que s'il couvre tous les assets, et un son de blessure détaillé contredirait la sobriété du tableau ci-dessus.*
- Les **violences entre conjoints** existent dans la simulation, chez des profils violents, avec une fréquence calibrée sur la réalité, parce que leurs suites (peur, départ, plainte, services sociaux, procès) font partie de la vie d'une ville ; elles ne sont jamais mises en scène, seulement perçues par leurs signes (décidé).

### 12.4 Public visé (décidé)

*Pourquoi : la violence réaliste, les drogues, la criminalité organisée et les conséquences judiciaires en font un jeu pour adultes.*

Le jeu vise un **public adulte** (classification 18 ans). Cette décision touche le socle et y est reportée (§16.1).

---

## 13. Niveaux de détail et coût de calcul

### 13.1 Le combat seulement en simulation complète (décidé)

*Pourquoi : le combat geste par geste n'a de sens que là où un joueur peut le voir ; ailleurs, une résolution par table, avec les mêmes attributs, donne les mêmes issues en moyenne pour un coût négligeable.*

| Niveau | Affrontements | Crimes | Enquêtes | Procès |
|---|---|---|---|---|
| **Complet** | Geste par geste (§1) | Joués : effraction, vol, fuite, témoins réels | Actes physiques joués : constatations, auditions, interpellations | Audience jouée si un joueur est dans la salle |
| **Simplifié** | **Table de résolution** au début de l'affrontement : vainqueur, blessures, mort éventuelle, selon les attributs, les armes, le nombre et la surprise | Résolus par recette en une fois : réussite ou échec, butin, dégâts, traces abstraites, témoins présents dans le même lieu selon leur agenda | Étapes résolues par taux selon les indices, la charge de l'enquêteur et sa compétence | Résolution par la formule du §8.3 |
| **Statistique** | Comme simplifié | **Tirés au sort** selon des taux par lieu, par activité et par profil (cambriolage d'un logement vide, vol au marché), calibrés sur les repères du §10.9 | Avancement statistique quotidien de chaque affaire | Formule du §8.3 |

- **Arrivée d'un joueur** : un affrontement déjà résolu hors de la bulle laisse son **état résultant** (blessé au sol, secours ou police en route, traces) ; le joueur arrive sur les suites, jamais au milieu d'un combat recalculé (décidé). *Pourquoi : c'est la règle de reconstruction de l'IA ([ia-pnj.md](ia-pnj.md), §6.3), qui garantit un état toujours plausible.*
- **Interpellation hors de la bulle** : elle est résolue (le suspect se laisse emmener, fuit ou résiste selon la table) ; un joueur près du commissariat voit arriver la voiture.
- **Personnages importants** : les policiers et magistrats impliqués dans une affaire qui concerne un joueur deviennent **importants** ([ia-pnj.md](ia-pnj.md), §6.2) et restent au moins en simulation simplifiée, pour que l'enquête avance avec des recettes et non au hasard.
- **Solo accéléré** : les mêmes règles statistiques s'appliquent ; une nuit sautée peut contenir un cambriolage dans l'immeuble, découvert au réveil.
- **Blessures et guérison** : hors de la bulle, elles évoluent par formule (saignement, soins, guérison), à chaque bloc d'agenda.
- **Mêmes traces, mêmes lois** (décidé, [ia-pnj.md](ia-pnj.md) §13.12) : un affrontement ou un crime résolu en simplifié ou en statistique écrit les **mêmes faits** que s'il avait été joué en complet : souvenirs et pensées des parties et des témoins, **faits sensibles** de l'auteur avec ceux qu'il croit témoins (§6.4), anomalies laissées aux habitants (porte forcée, objet disparu, §4.2), conclusions « lieu dangereux » possibles, traces matérielles et rumeurs. Les **tables** du simplifié et du statistique (crimes tirés au sort, réactions des témoins, appels, plaintes, avancement des affaires) sont **recalibrées** par `ts-sim` sur des simulations en complet qui incluent les nouveaux comportements : découvertes par anomalie, attroupements, pressions sur les témoins, aveux anticipés, incohérences de versions, habituation aux fausses alertes. Un test compare leurs distributions ([ia-pnj.md](ia-pnj.md), §14.2). *Pourquoi : un joueur qui revient dans un quartier ne doit pas pouvoir dire, aux suites d'un crime, à quel niveau il a été simulé.*

### 13.2 Les compétences de combat (décidé)

*Pourquoi : c'est la règle de [ia-pnj.md](ia-pnj.md), §3.5 : écrire d'abord, entraîner ensuite, et ne jamais apprendre les décisions.*

- **D'abord écrites à la main** : des comportements d'exécution (gérer la distance, choisir le geste selon l'état de l'adversaire, parer, se dégager, viser) pour chaque compétence : bagarre ordinaire, sport de combat, technique d'intervention policière, arme blanche, tir, fuite, poursuite à pied et en véhicule, maîtrise d'une personne.
- **Puis entraînables** : seules ces compétences d'exécution pourront être **entraînées** en Python sur le cœur, sans affichage, par des milliers d'affrontements simulés ([05-architecture.md](../05-architecture.md), §4). Chaque compétence entraînée reçoit les attributs et l'état du personnage : un vétéran reposé et un passant terrifié n'en tirent pas la même chose.
- **Jamais apprises** : se battre ou fuir, sortir une arme ou non, s'acharner ou s'arrêter restent des décisions des couches réflexes et sélection d'objectif, écrites à la main.
- Les **tables de résolution** du niveau simplifié (§13.1) sont calibrées à partir des affrontements simulés en complet, pour que les deux niveaux donnent les mêmes issues en moyenne.

### 13.3 Ordres de grandeur **[à calibrer]**

| Élément | Estimation |
|---|---|
| Affrontements en simulation complète | Rares : quelques-uns par heure et par serveur hors actes des joueurs ; coût dominé par la perception et les trajectoires |
| Affaires ouvertes | Quelques centaines par an et par monde ; quelques dizaines actives en même temps |
| Mise à jour statistique des affaires | Une passe par jour de jeu, négligeable |
| Indices et traces | Quelques centaines d'entités actives, effacées au nettoyage ou au classement |

---

## 14. Fiches de rôle et effectifs fournis

*Pourquoi : les rôles sont des données ([ia-pnj.md](ia-pnj.md), §0) et les métiers en ont une partie économique ([economie.md](economie.md), §3.1) ; ce domaine fournit les fiches de ses métiers et de ses rôles, au même format.*

| Fiche | Palier typique | Recettes et actions propres |
|---|---|---|
| Policier de roulement (gardien de la paix) | 3 | Patrouiller, intervenir, interpeller, maîtriser, enregistrer une plainte, contrôler, escorter |
| Chef de poste | 4 | Prendre les appels, ordonner les priorités, envoyer les patrouilles, surveiller la garde à vue |
| Policier de la brigade anticriminalité | 4 | Patrouille en civil, surveillance, flagrant délit |
| Enquêteur de la sûreté (officier de police judiciaire) | 4 à 5 | Constatations, auditions, garde à vue, perquisition, réquisitions, gérer des informateurs |
| Technicien de police scientifique | 3 à 4 | Relever les traces, prélever, rattacher aux fichiers |
| Enquêteur financier | 4 à 5 | Enquête patrimoniale, signalements bancaires |
| Commissaire adjoint | 6 | Diriger, répartir, décider d'une opération |
| Commissaire central | Hors catégorie | Priorités, relations avec le procureur et le maire |
| Policier municipal | 3 | Patrouille, stationnement, circulation, abords des écoles, flagrant délit |
| Agent de surveillance de la voie publique | 1 | Stationnement |
| Agent de sécurité, vigile (existe dans l'économie) | 1 | Rondes, contrôle d'accès, retenir un voleur |
| Juge (siège), juge d'instruction, juge des enfants, juge de l'application des peines | 6 à 7 | Conduire l'audience, décider, mettre en examen, suivre les peines |
| Procureur | Hors catégorie | Orienter les affaires, requérir |
| Substitut du procureur | 6 | Permanence, décisions de garde à vue, réquisitions |
| Greffier | 3 | Tenir les audiences et les dossiers |
| Conseiller d'insertion et de probation | 3 | Suivi des condamnés, projets de sortie |
| Juge consulaire (tribunal de commerce) | Celui de son métier | Juger les faillites, en plus de son métier |
| Commissaire de justice (huissier) | 5 | Signifier, constater, saisir, expulser |
| Mandataire judiciaire | 5 | Gérer les liquidations |
| Avocat (existe dans l'économie) | 5 | Conseiller, défendre, plaider, négocier |
| Contrôleur fiscal | 4 | Contrôler, redresser, transmettre |
| Inspecteur du travail | 4 | Contrôler les chantiers et les établissements |
| Médecin légiste (fonction d'un urgentiste) | Celui de son métier | Constater, examiner un corps |
| Armurier | 2 à 3 | Vendre sur permis, tenir le registre |
| **Juré** (rôle temporaire) | — | Assister au procès, délibérer |
| **Détenu** (état, à l'extérieur) | — | Voir §9 |
| **Personne sous bracelet** (état) | — | Horaires imposés dans l'agenda |
| Rôles du milieu : guetteur, revendeur, nourrice, chef de point, cambrioleur, receleur, voleur de véhicules, usurier, homme de main, chef de bande, chef de réseau | Ceux de [economie.md](economie.md), §3.8 | Recettes des §10.2 à §10.8 |

**Effectifs** : §5.1 et §5.2, ≈ 80 personnes en tout pour la police, la justice et les contrôles **[à calibrer]**, à créer par le générateur de population.

---

## 15. Points ouverts

Aucune question de conception propre à ce domaine ne reste ouverte. Restent les valeurs **[à calibrer]**, à ajuster par le prototype du cœur ([05-architecture.md](../05-architecture.md), §5) puis par des affrontements et des années simulés en accéléré :

| Sujet | Valeurs |
|---|---|
| Affrontements | Durées de fatigue, probabilités d'étourdissement et de perte de connaissance, effet du stress sur la précision, portée du bruit d'un tir, compensation de latence (plafond de 250 ms, bornes de la date fine), temps de réaction appliqué aux postures défensives, pénalité du tir instantané, seuils d'alerte de régularité surhumaine |
| Armes | Nombre d'armes légales et illégales au départ, munitions, prix |
| Signatures | Facteurs de J1 et J2, facteur de la classe « vulnérable », seuils des planchers R1 et R4 (paramètres de [revues/signatures.md](../revues/signatures.md), calibrés par les comparaisons d'un panel de sessions évaluatrices neuves, A8) |
| Part serveur (P12) | Plages de tirage des serrures, des coffres, des caméras privées et des alarmes (§3.1), décalage par quartier, durée de conservation des images |
| Blessures | Barème d'atteinte vitale, vitesses de saignement, seuils des PV, durées de guérison, chances de survie selon le délai des secours |
| Témoins | Poids des facteurs de réaction, effet du nombre, précision des descriptions, probabilités de reconnaissance, coefficients des enfants, gravité des anomalies propres à ce domaine, part des curieux qui filment ou piétinent la scène |
| Victimes | Probabilités de plainte par type de fait |
| Police | Effectifs, présence de nuit, délais d'intervention, charge des enquêteurs, seuils de certitude, délais des analyses et des signalements, fréquence des contrôles, seuil de saturation qui déclenche le rappel des agents et le plan blanc, nombre d'appels démentis avant vérification, poids d'une incohérence de version dans la certitude d'une affaire |
| Justice | Seuil de condamnation, barème des peines et des amendes, majorité des assises, délais de procédure, réductions de peine, délais d'effacement du casier, prescription |
| Détention | Durée avant perte d'emploi, probabilités du résumé, durée avant qu'un intérim devienne définitif |
| Personnage persistant | Coefficient de l'amende équivalente, diviseur du patrimoine (120), plafond du désentraînement, durées des interdictions et du délai d'épreuve, seuil (6 mois), durée et horaires de la surveillance électronique, délai avant jugement par défaut, délai de déconnexion et son plafond |
| Joueurs | Délai de rattachement d'une mort à une incarnation après déconnexion, pénalités des désynchronisations graves (25 %, doublement) et des morts imprudentes répétées (10 %, 25 %), seuils de détection de l'abus du monde partagé (refus du point de rupture, désynchronisations, morts imprudentes, facilitations) et de la chasse aux nouveaux venus |
| Criminalité | Repères annuels du §10.9, fréquence des règlements de comptes, taux de la simulation statistique (recalibrés sur le complet avec les comportements de la v0.3) |

---

## 16. Impacts sur le socle et les autres domaines

Ces points ne sont **pas appliqués** ailleurs : ils sont à trancher par la session principale ou par la session du domaine concerné.

### 16.1 Socle

*v0.3* : la revue de l'intelligence n'ajoute **aucun impact** sur le socle depuis ce domaine ; ses effets sur le socle sont portés par [ia-pnj.md](ia-pnj.md), §11.

*v0.4* : les revues des signatures et de la génération sont **déjà intégrées au socle** par la session principale ; ce domaine n'ajoute aucune demande.

*v0.5* : la direction artistique et la revue des signatures du contenu sont **déjà intégrées au socle** par le responsable conception ; ce domaine n'ajoute aucune demande.

*v0.6* : aucun impact sur le socle. Ce fichier est aligné sur l'arbitrage A8 (en-tête de [05-architecture.md](../05-architecture.md)) : les décisions de modération (rapprochement de comptes, suspension, exclusion, sanction d'une régularité surhumaine) reviennent à une session de modération sur dossier, et les paramètres des signatures à un panel de sessions évaluatrices (§1.2, §11.6, §11.8, §15). Aucune contradiction relevée.

| Fichier | Impact | État |
|---|---|---|
| [01-concept.md](../01-concept.md), §2 | Détail des **conséquences équivalentes** du personnage persistant en ligne : conversion par mois de prison (amende équivalente, désentraînement plafonné, perte d'emploi et des postes de groupe, interdictions et délai d'épreuve en temps réel), **garde à vue accélérée**, plaider-coupable ou audience, **jugement par défaut** après 14 jours réels ; dans le monde, la peine est perçue comme aménagée (§7.2, §9.5) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 | En **solo**, le personnage persistant est réellement détenu à l'extérieur ; le joueur incarne d'autres personnages ou fait avancer le temps (§9.4) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1 | **Public adulte** (classification 18 ans) (§12.4) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4 | Non jouable temporairement : **en garde à vue** et en détention provisoire ; un joueur qui incarnait déjà le personnage continue (§7.2). Une personne sous bracelet électronique reste jouable (§8.4) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4, critère 4 | Ajouter à la règle adulte-mineur : **aucun geste violent d'un adulte envers un mineur**, aucune arme ne vise ni n'atteint un mineur, **aucun enfant ne meurt du fait d'un personnage** ; entre mineurs, bagarres sans arme seulement (§12.2) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 | Les PV réels sont calculés à partir des blessures localisées, de la perte de sang et des maladies ; seuils de choc, d'état critique et de mort (§2.2) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §3 | Correspondance entre types de blessure et causes du tableau des malus (§2.5) ; dans le monde, le personnage persistant « mort » est **réanimé**, pas mort (§2.5) ; en ligne, une **blessure grave non mortelle** du personnage persistant suit la règle de la convalescence théorique, sans malus psychologique ni compte dans les morts répétées ; les blessures légères guérissent en temps réel, connecté ou non (§2.4) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §2 et [04-joueur.md](../04-joueur.md), §1.1 | **Critère d'imprudence** : comparaison avec le réflexe et la décision que l'IA en observation aurait choisis (§11.3) ; une mort survenue dans les 10 minutes après une déconnexion en plein affrontement est comptée dans le bilan de l'incarnation (§11.5) ; un homicide ou une violence grave **à contre-emploi** compte comme désynchronisation volontaire grave (§11.2) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §2 | Une condamnation n'enlève pas d'expérience en elle-même ; les crimes du personnage persistant modifient le profil du joueur (§9.5, §11.4) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §7 | Règle hors fiction de **récusation** des personnages liés dans les affaires de police et de justice, sur le modèle des flux liés de l'économie (§11.8) ; **aucune intention de mise en danger mortelle de soi** (§11.9) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 | Combat résolu par le serveur avec une **compensation de latence limitée** (§1.2) ; entité **affaire** faite de croyances (§6.2) ; résolution des affrontements par table hors de la bulle, calibrée sur la simulation complète (§13) ; compétences de combat entraînables sur le cœur sans affichage (§13.2) ; le **journal des actions** ne sert jamais de preuve dans le monde (§0) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.16 | Modération : détection de la **chasse aux nouveaux venus** et de l'**abus du monde partagé** (série de violences gratuites) ; journalisation de la consultation d'un fichier de police concernant le personnage persistant d'un autre joueur (§11.6, §11.7) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §2 et [04-joueur.md](../04-joueur.md), §1.1 | **Point de rupture** dans le combat : un homicide ou une violence grave à contre-emploi n'a jamais lieu ; pénalité des désynchronisations graves (25 % du niveau, doublée en récidive) et des morts imprudentes répétées (10 %, 25 %) ; mise en danger mortelle évidente d'un personnage prudent au point de rupture (§11.2, §11.3 ; E1, E3, E28) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 | Conséquences équivalentes : amende équivalente sur le plus grand du revenu de référence et du patrimoine net connu ÷ 120 ; jugement par défaut sans aménagement favorable, avec mandat ; **surveillance électronique** (22 h – 6 h, temps connecté seulement) pour une peine convertie de 6 mois ou plus ; « pas de prison en temps réel » conservé (§9.5 ; E22) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 et [03-vie-synchronisation.md](../03-vie-synchronisation.md), §3 | Personnage persistant : la menace retarde la disparition **partout**, lieux de retrait compris ; un véhicule d'autrui ne quitte pas la carte avec lui ; en état critique, il ne disparaît jamais ; tué ou gravement blessé alors qu'interpellé, poursuivi ou recherché, réveil à l'hôpital public sous garde (§11.5 ; E19, E21) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §7 | La récusation devient la règle de **conflit d'intérêts** sur le **cercle** du joueur (personnages liés, amis mutuels, comptes rapprochés par une session de modération sur dossier, A8 ; les comptes présumés liés ne produisent que des alertes), étendue aux actes de victime et de témoin (retirer une plainte, refuser d'identifier, alibi, aveu) ; aveu limité à la mémoire du personnage (§7.2, §11.8 ; E8, A3) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §7 et [05-architecture.md](../05-architecture.md), §2.16 | Abus du monde partagé étendu aux incarnations (refus répétés du point de rupture, désynchronisations graves, morts imprudentes, facilitations suivies d'un crime) ; **note de conduite qui survit à la réinitialisation** ; signalement automatique de la mise en danger d'un mineur ; signalement des incidents simultanés d'un même cercle ; alerte de régularité surhumaine en combat (§5.4, §11.6, §12.2, §1.2 ; E3, E4, E23, E27, E28, E29, S35) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4, critère 4 | Règle adulte-mineur : aucun tir, lancer ou coup n'est proposé si sa trajectoire passerait près d'un mineur (au lieu d'un tir résolu comme manqué) ; aucune intention de recruter ou de faire agir un mineur pour un délit pour un pilote `Joueur` (§12.2 ; E27) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 | **Défense par posture** appliquée avec le temps de réaction du personnage, compté depuis la perception du geste adverse (A5) ; compensation de latence calculée par le serveur et plafonnée, date fine des intentions bornée par le serveur (S9) ; précision dominée par le personnage (S35) (§1.2, §1.5) | Intégré au socle le 29/10 |

### 16.2 Ville ([ville.md](ville.md))

- **Confirmés** : commissariat central pour toute l'agglomération, un poste de police municipale par commune, pas de gendarmerie, prison à l'extérieur, délai d'environ une minute avant la disparition d'un personnage persistant déconnecté hors d'un lieu de retrait (§5.1, §9.1, §11.5).
- **Emplacement du commissariat central** : proposé dans le **quartier de la Gare**, près de la gare et du pont de la Gare, pour sa position centrale entre les deux rives (§5.1).
- **Intérieurs** à prévoir : au commissariat, cellules de garde à vue, salles d'audition, salle des scellés, armurerie de service, garage des véhicules ; au tribunal, salle de la cour d'assises, salles correctionnelle et civile, bureaux des juges et du parquet, salle des jurés, bureau du conseiller d'insertion ; une **étude de commissaires de justice** et un **mandataire** dans la Cité (§5.2).
- Un **bâtiment administratif** de l'État (centre des finances publiques, inspection du travail), proposé dans le quartier de la Gare (§5.2).
- **Surveillance** des lieux : alarmes, caméras privées, vidéoprotection municipale selon le budget des communes, avec leur champ, pour la donnée « surveillance » du registre (§3.1, §3.2).
- **Serrures** de difficultés différentes (simple, multipoints, de sûreté) et coffres, dans l'état du décor (§3.1).
- **Traces temporaires** comme entités du décor (sang, verre, douilles, ruban de scène) jusqu'au nettoyage ou au prélèvement (§3.4).
- Bords des toits, des quais et des ponts traités comme des **obstacles** pour le déplacement volontaire (§11.9).
- *v0.4* : la part publique fournit la **classe** de chaque porte (palière, de service, vitrine, cave), les emplacements candidats des coffres et des caméras et les lieux équipables d'une alarme ; ce domaine fournit les **plages de tirage** (§3.1, P12). La **géométrie étagée** doit porter les plages de hauteur des murs, murets et garde-corps, et les ouvertures (fenêtres, portes, balcons) dont dépendent lignes de vue, tirs et champ des caméras (§1.2, §3.2, §4.1, P13).
- *v0.3* : la **sécurité perçue** d'un lieu baisse aussi avec les conclusions « ce lieu est dangereux » de ses usagers (§4.2, ia-pnj §13.4) ; les alarmes ont un historique de **déclenchements intempestifs** connu de la police (§5.4).
- *v0.2* : les **lieux de retrait** et les sorties de la carte ne font plus disparaître un personnage persistant **menacé** ; il reste derrière sa porte, dans le transport ou à la sortie, et un véhicule d'autrui est retrouvé à la sortie (§11.5, E19).

### 16.3 IA des PNJ ([ia-pnj.md](ia-pnj.md))

- **Réflexes** : tableau de choix (fuir, se figer, se défendre, appeler à l'aide, protéger, se soumettre) et formule de score (§1.8) ; panique de foule.
- **Mode observation** : calculer, aux moments clés d'un affrontement, le réflexe et la décision que l'IA aurait choisis, pour la synchronisation et le critère d'imprudence (§11.3).
- **Recettes** nouvelles : se rendre à la police et avouer après un acte dissonant grave (§11.2) ; porter plainte (§4.5) ; témoigner, se taire, mentir à la police (§6.4) ; intervenir, secourir, filmer (§4.2) ; recettes de crime avec estimation du risque à partir des savoirs du personnage (§10.1) ; recettes des métiers de police et de justice (§14).
- **Témoins** : réactions et souvenir structuré d'une description, avec l'effet du stress et de l'arme (§4.2, §4.3).
- **Enfants** : aucune intention violente envers un mineur, aucun acte d'arme pour un mineur (§12.2).
- **Détention** : contenu du résumé statistique et effets au retour (§9.2, §9.3) ; état « sous bracelet » qui impose des horaires à l'agenda (§8.4) ; obligations de jurés et de témoins cités dans l'agenda (§8.2, §8.3).
- **Fuite à l'extérieur** d'un suspect : arrestation tirée au sort ou départ définitif (§7.1).
- *v0.6* (**pour la phase 1**) : recettes de l'**organisateur de jeux clandestins** et du recouvrement des dettes de jeu par la bande (§10.3) ; recette du **serveur** qui refuse ou non de servir un client ivre, et du **vendeur** qui demande ou non l'âge d'un adolescent d'allure adulte (§8.1) ; savoir professionnel des **couleurs des bandes** chez les policiers (§6.4).
- *v0.5* : une **cagoule** portée hors du froid est une anomalie de ce domaine (§4.3) ; les recettes qui font entrer chez autrui sans invitation portent la **violation de domicile** (§8.1, A23) ; le choix de se masquer ou de se déguiser relève du savoir-faire du criminel (§6.3), et un **déguisement** retrouvé corrige les souvenirs des témoins (§4.3).
- *v0.4* : les coûts des recettes de crime et des gestes de combat viennent de `appraise` sur les effets (blessure attendue, contrainte, qualification) et l'agrégat des étapes (G1, G8) ; les recettes n'ont plus de signature écrite. Un **témoin** juge une violence sur ses propres croyances : s'il n'a pas vu l'agression initiale, il juge la riposte sans J1 (G28, §4.2).
- *v0.3* (revue de l'intelligence, ia-pnj §13) : ce domaine utilise les **anomalies** (§13.8) et demande d'y inclure ses catégories (porte forcée, vitre brisée, sang, douille, corps, ruban de scène, coup de feu entendu), avec la réponse « alerter » branchée sur l'appel d'urgence (§4.2, §5.4) ; les **faits sensibles** (§13.5) sont créés par tout crime, acte violent ou acte dissonant grave de leur auteur, avec ceux qu'il a vus le voir, et alimentent les recettes de pression sur un témoin, de version préparée et d'aveu anticipé (§6.4, §11.2) ; le **registre des déclarations** sert aussi aux auditions de police (§6.2) ; les **curieux** des attroupements (§13.9) obéissent à l'ordre de dispersion selon leur respect des règles (§5.3) ; l'**habituation** (§13.11) ne doit jamais émousser le réflexe face à une menace réelle (§1.8, §5.4) ; la conclusion « ce lieu est dangereux » (§13.4) se forme aussi chez les témoins, pas seulement chez les victimes (§4.2) ; les tables statistiques du crime sont recalibrées avec celles de l'IA (§13.1, §13.12).
- *v0.2* : **point de rupture** (hésiter puis renoncer, reprise dans le même tick) dans l'escalade d'un affrontement (§11.2, E1) ; **postures défensives** appliquées avec le temps de réaction du personnage, la même règle pour l'IA (§1.2, A5) ; rappel des **agents hors service** et des soignants (**plan blanc**) quand la file des urgences sature, en interrompant leur activité réelle (§5.4, E29) ; les **mineurs s'éloignent en priorité** d'un affrontement et les adultes proches les emmènent ; la consigne « pas d'angle » près d'un mineur vaut pour l'IA (§12.2, E27) ; la mémoire du personnage borne aveux et témoignages des incarnés (§7.2, §11.8, E8).

### 16.4 Population ([population.md](population.md))

- **Effectifs** à créer : police nationale (≈ 36), polices municipales (≈ 17), tribunal (≈ 13), conseiller d'insertion, commissaires de justice, mandataire, contrôleurs fiscaux, inspecteur du travail, avocats (≈ 10), armurier (§5.1, §5.2, §14).
- **Armes au départ** : ≈ 80 chasseurs déclarés, ≈ 10 à 15 armes illégales dans le milieu (§1.5).
- **Milieu criminel** confirmé (≈ 2 % des adultes, jamais selon l'origine), **casiers** et anciens détenus, quelques habitants **en détention** avec une date de retour (§8.7, §9.3, §10.2).
- **Morts violentes** : 1 à 3 par an hors joueurs, comptées dans la mortalité (§10.9).
- **Aucun enfant ne meurt du fait d'un personnage** (§12.2), à respecter dans les tables de mortalité.
- *v0.5* : certaines **cicatrices** sont présentes au départ, inscrites dans la biographie (agression, accident, opération), dans les signes distinctifs du génome d'apparence (§2.4).
- *v0.4.2* : **traité**. La demande de population §12.6 est intégrée : §12.2 reprend l'absence de tout décès d'enfant, même naturel, dans la première version (tables de mortalité des mineurs nulles, [population.md](population.md), §7.3). Aucune autre demande.
- **Listes de jurés** : adultes inscrits, hors personnages persistants (§8.2).
- **Succession** du commissaire central et du procureur : par promotion interne si possible, sinon par une arrivée nommée de l'extérieur (§5.1).
- **Renforts de police** par la dotation de l'État, pourvus par de vraies arrivées (§5.1).
- *v0.2* : les agents et soignants **rappelés** en cas de saturation sont des habitants existants, pas des arrivées (§5.4, E29).

### 16.5 Économie ([economie.md](economie.md))

- **Barème** des amendes et des peines, modulation par les revenus, bénéficiaires (État ou commune) (§8.5).
- **Ordre de recouvrement** des amendes, dommages et intérêts et redressements : prélèvement, saisie sur salaire, saisie et vente aux enchères, dette (§8.6).
- **Confiscation** des avoirs criminels au profit de l'État ; destruction des stupéfiants et armes saisis (§8.6).
- **Fonds de garantie des victimes** par le compte « État », qui se retourne contre l'auteur (§8.6).
- **Amende équivalente** du personnage persistant en ligne et **caution** du contrôle judiciaire (§9.5).
- **Aide juridictionnelle** (avocat de permanence payé par l'État) et honoraires d'avocats (§7.4).
- **Jours-amende**, **travail d'intérêt général** non payé, détention à domicile sous bracelet (le condamné peut travailler) (§8.4).
- **Détention** : perte d'emploi au-delà d'un mois, suspension puis révocation d'un fonctionnaire, procuration à un proche (§9.2).
- Fiches de métier fournies au §14 (inspecteur du travail, contrôleur fiscal, commissaire de justice, juges, mandataire, et métiers de police).
- **Budgets communaux** : effectifs et armement de la police municipale, vidéoprotection (§3.2, §5.1).
- **Primes d'assurance** selon la criminalité du quartier, recours de l'assureur contre l'auteur identifié (§8.6).
- *v0.6* : **traité** pour la demande du §14.10 : servir un client manifestement ivre et vendre de l'alcool ou du tabac à un mineur entrent au catalogue (§8.1, §8.5), la vente en connaissance de cause restant non proposée ; les jeux clandestins sont une activité de bande, avec dettes de jeu recouvrées comme l'usure (§10.3). **Pour la phase 1** : l'interdiction temporaire de vendre de l'alcool ou du tabac s'applique au commerce (licence suspendue), et les enjeux et la caisse confisqués vont au compte « État » (§8.6).
- *v0.4.2* : **traité**. L'indignité successorale (§5.11, §14.8, E31) a sa contrepartie au §8.7 : la justice transmet à la succession toute condamnation définitive, comme auteur ou complice (instigation comprise), pour la mort du défunt, même après le partage ; l'économie exclut l'héritier ou recouvre sa part comme une créance de victime. Hors champ : l'homicide involontaire.
- *v0.4* : la **richesse** du ménage ou de l'établissement, et le maniement d'espèces ou d'objets de valeur, servent au tirage des serrures, des coffres, des caméras et des alarmes (§3.1, P12).
- *v0.3* : le **besoin de sécurité** nourri par la conclusion « ce lieu est dangereux » crée une demande de serrures, d'alarmes et de raccompagnements, et peut motiver un déménagement (§4.2).
- *v0.2* : l'**amende équivalente** prend le plus grand du revenu de référence (§7.8) et du **patrimoine net connu ÷ 120** ; l'économie fournit ce patrimoine (déclaré et saisi) (§9.5, E22). Le **cercle** du joueur (§11.8, E8, A3) est le même que celui des flux associés de l'économie.

### 16.6 Social ([social.md](social.md))

- Les **coefficients de crédibilité** des enfants sont repris par la police et la justice ; un témoignage d'enfant ouvre une enquête mais ne suffit pas seul à condamner (§4.4).
- **Interrogatoire** : actes du suspect et de l'enquêteur, formule de la tendance à parler, transfert de savoirs opaques (§6.5).
- **Délibéré des assises** comme conversation d'actes d'influence (§8.3).
- **Chronique judiciaire** du journal local comme source d'informations pour toute la ville (§8.3).
- Étiquettes nouvelles : « **ancien détenu** », « **recherché** » (pour ceux qui ont vu sa photo), « **violent** » après une condamnation connue (§8.7, §9.3).
- **Escalade des conflits de territoire** et des sanctions internes, que les alliances, trêves et successions du social complètent (§10.2).
- **Opération de police** comme projet collectif du groupe « commissariat » (§10.2).
- **Modération** : chasse aux nouveaux venus et abus du monde partagé, en plus de l'acharnement (§11.6).
- Les **violences entre conjoints** existent, perçues par leurs signes ; leurs suites passent par les relations, le départ du domicile et les services sociaux (§12.3).
- *v0.6* : **traité** pour la demande du §12.12 : le **tapage nocturne** est au catalogue (il y était déjà comme « tapage » ; §8.1 précise la réponse de la police) ; l'**usurpation de fonction** par le port d'un uniforme ou d'un insigne est un délit (§8.1, §8.5) ; la police **reconnaît les couleurs des bandes** comme un savoir professionnel, sans que les porter soit une infraction (§6.4). **Pour la phase 1** : l'appel à la police pour tapage est la quatrième marche de l'escalade du bruit (social §5.11), et une patrouille venue pour un tapage laisse un souvenir à la source du bruit (rancune envers le voisin qui a appelé).
- *v0.5* : « **insulter** » sort de l'escalade du combat et rejoint l'**échelle de la parole** (§11.2, M11) ; une **cicatrice** visible a les effets sociaux d'un signe distinctif (§2.4).
- *v0.3* : les **pressions sur un témoin** et l'**aveu anticipé** passent par les actes sociaux (menacer, acheter, confier) et visent ceux que l'auteur **croit** témoins (§6.4, ia-pnj §13.5) ; les **curieux** propagent la nouvelle d'un incident comme une rumeur ordinaire (§4.2) ; la conclusion « ce lieu est dangereux » circule aussi par la rumeur et alimente la réputation d'un quartier (§4.2).
- *v0.2* : la **modération** reçoit aussi les signalements de mise en danger d'un mineur, les incidents simultanés d'un même cercle, les abus commis par incarnation et les alertes de régularité surhumaine ; la note de conduite survit à la réinitialisation (§1.2, §5.4, §11.6, §12.2 ; E3, E23, E27, E29, S35). Les actes de victime et de témoin (retirer une plainte, refuser d'identifier, alibi, aveu) ne sont pas proposés à un incarné dont le cercle est en cause, et son témoignage est construit à partir de sa mémoire (§11.8, E8).

### 16.7 Interface ([interface.md](interface.md))

- Saisie des **intentions de combat** (garde, coups haut et bas, saisie, visée, tir, reddition) et affichage des **effets physiologiques** (sursaut, tremblements, vision en tunnel, essoufflement), sans aucun chiffre (§1.6, §1.7, §1.8).
- **Blessures** visibles sur le personnage et ressenties par le joueur (zone douloureuse, saignement), en plus des deux barres de vie (§2.1, §2.2).
- **Courrier judiciaire** : convocation, décision, amende, avis d'huissier ; consultation de son **casier** pour le personnage persistant ; **zones interdites** de séjour signalées au joueur sur la carte (§9.5).
- **Délai de déconnexion** et sa prolongation tant que le personnage est menacé (§11.5).
- Intuition expliquant une intention **indisponible** pour cause de récusation (§11.8).
- Réglage « **atténuer la violence** » (§12.3).
- *v0.5* : le réglage « atténuer la violence » bascule tous les assets de violence sur leur **variante atténuée** (§12.3) ; le brassard des policiers en civil se voit une fois sorti (§5.1).
- *v0.4* : le bilan et la réticence citent les raisons codées du verdict, dont les justifications (« vous défendiez votre fils ») et leur absence quand l'agression était provoquée (§11.1, G6, G23).
- *v0.2* : saisie de la défense comme **posture** (garde, esquive, se couvrir, reculer), sans geste de parade minuté à la main (§1.2, §1.7, A5) ; hésitation visible puis renoncement au **point de rupture** (§11.2, E1) ; intentions de tir, de lancer ou de coup **non proposées** près d'un mineur (« pas d'angle », §12.2, E27) ; intuition « Vous êtes trop proche de cette affaire. » pour le conflit d'intérêts (§11.8, E8) ; horaires de **surveillance électronique** et déclaration de séjour hors de l'agglomération, dans le courrier judiciaire et sur la carte (§9.5, E22).

### 16.8 Service de dialogue

- Ne formule **jamais** de contenu de violence sexuelle, de torture, de suicide ni de violence envers un enfant ; formule les menaces, les interrogatoires et les témoignages comme des actes ordinaires (§12.1).
- Reçoit, pour un interrogatoire, les faits structurés que l'enquêteur oppose au suspect, jamais la valeur d'un savoir opaque (§6.5).

### 16.9 Direction artistique ([direction-artistique.md](direction-artistique.md))

*v0.5* : les quatre demandes du §12.7 sont **traitées**.

| Demande | Réponse | État |
|---|---|---|
| Uniformes et insignes fictifs ; armes génériques (§1.4) | §5.1 (police, secours, justice, véhicules ; brassard des policiers en civil) ; §1.5 (armes sans marque ni modèle réel) | Traité |
| Cicatrice d'une blessure grave dans le génome (§3.2) | §2.4 : blessure grave ou critique (coupure, plaie pénétrante, par balle, brûlure), ≈ 30 % ou ≈ 70 % **[à calibrer]**, zone, taille et visibilité ; permanente, aussi pour le personnage persistant | Traité |
| Variante atténuée de chaque asset de violence ; aucun son de blessure détaillé (§4.5, §5.6) | §12.3 : variante obligatoire, refusée par le contrôle des assets si elle manque ; aucun son de blessure détaillé, avec ou sans le réglage | Traité |
| Effet des masques sur la reconnaissance (§3.5) | §4.3 : table cagoule, capuche, déguisement ; voix reconnaissable par les proches ; déguisement retenu comme une vraie description | Traité |

Demande en retour : la direction artistique prévoit, pour les **cicatrices**, des motifs par zone et par taille, et le **brassard** de police comme pièce de tenue.

### 16.10 Revue des signatures du contenu ([revues/signatures-contenu.md](../revues/signatures-contenu.md))

*v0.5* : les quatre demandes du §11.3 sont **traitées** : violation de domicile au §8.1 et au §8.5 (sursis de 3 mois, maximum 1 an ; l'annotation A23 peut porter `LawViolation`) ; barème M2 de la violation de la loi au §8.5, avec les aggravations propres à l'auteur (+0,05, au plus +0,15) et celles de la victime laissées à la classe de cible ; escalade commençant à « menacer » au §11.2 (M11) ; « Fouiller » dérivé par D31 au §1.7, avec J6 pour la police.
