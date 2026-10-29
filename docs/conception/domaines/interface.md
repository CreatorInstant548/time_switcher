# Domaine — interface

> Fichier de **domaine**, géré par la session « Conception — interface » (voir [README](../README.md)).
> Version 0.9.2 — 30/10/2026 : correction **R1** de l'équilibrage (tour 2) : envers une cible liée par un secret, la classe apparente fixe le niveau affiché, l'indicateur de rupture affiché et la présence de l'acte ; le vrai coût ne joue qu'à l'exécution, avec une éjection qui ne compte ni pour la récidive ni dans les refus répétés (§2.3).
> Version 0.9.1 — 30/10/2026 : arbitrage **A11** : le rapprochement de comptes est notifié comme toute mesure lourde, sans nommer l'autre compte ni dire l'origine du dossier, avec l'écran de contestation, et reste dans l'export (§3.9, §10.4, §12.1).
> Version 0.9 — 30/10/2026 : corrections des revues de clôture (architecture C-5, sécurité C1 et C7, équilibrage C1, C6 et C8) : portraits à trois dates de perception identifiée, mis à jour seulement par une identification et calculés à l'ouverture sans notification (§4.7) ; candidats de « Une nouvelle vie » à la précision « tenue » (§3.3) ; réticence qui ne sonde aucun secret (§2.3) ; dons bornés au chapeau, au pourboire et vers le cercle associé (§8.5) ; notification, contestation structurée et panels de sessions en modération (§10.4). État au §15.16.
> Version 0.8 — 30/10/2026 (dernière vague avant la clôture de la conception) : [social.md](social.md) v0.7, §3.2 (trois niveaux d'écoute ; forme de la bulle selon le volume, icône selon le ton, rendu identique quel que soit le pilote, §2.8 et §5.4) et [economie.md](economie.md) v0.7, §14.10 (prix par pièce en boutique ; chapeau du musicien de rue en don volontaire, §8.5). État au §15.15.
> Version 0.7.1 — 30/10/2026 : arbitrage **A8** ([05-architecture.md](../05-architecture.md), en-tête ; [organisation.md](../../pilotage/organisation.md), §6) : aucune action de l'utilisateur. Tests à l'aveugle jugés par des sessions évaluatrices neuves face à des sessions joueuses (§12.3) ; calibrage, sanctions lourdes et rôles de la console tenus par des sessions (§10.4, §12.1, §14) ; disposition de rotation tranchée (§5.7). Les réglages proposés aux joueurs restent des fonctions du jeu.
> Version 0.7 — 29/10/2026 : revue du [contenu des signatures](../revues/signatures-contenu.md) v1.0.1, §11.7 (réticence en niveaux 0 à 3 et indicateur de rupture à part, M8 ; gabarits en trois formes et leurs variables, §9 ; actes excentriques avec leur signe, sans geste libre, A7) et [ia-pnj.md](ia-pnj.md) v1.1, §2.6 et §11 (tenue, coiffure et radio choisies par le joueur, §5.3 bis). Les sections ne sont jamais renumérotées (règle 6 du README de la conception). Version 0.6 : demandes de la [direction artistique](direction-artistique.md) v0.1.1, §12.4 (styles des deux couches, bulles, icônes de son et typographie ; éditeur d'apparence limité à l'espace de la population ; portraits rendus à partir de l'apparence connue ; flashs, palettes pour daltoniens, sous-titres des sons, son mono, plage dynamique réduite ; musique d'accompagnement et voix synthétisées ; crans de zoom et densité de texels). Version 0.5 : intégration des revues des [signatures](../revues/signatures.md) §9.10 (réticence en quatre niveaux, raisons en mots, bilan « Ce qui lui a coûté » et « Ce qui l'excusait ») et de la [génération](../revues/generation.md) §13.9 (ville par secteurs et par couches, création d'un monde solo, adresses à la française). Version 0.4.1 : demandes de [social.md](social.md) v0.5, §12.10 (registre poli ou familier dans la roue et sur la carte ; intuitions de la version donnée et des usages d'une pratique). Version 0.4 (revue de l'intelligence des PNJ, [ia-pnj.md](ia-pnj.md) v0.8, §11 à §14 : humeur et crises visibles, naturel, registre, intuitions nourries par les pensées ; gestes qui trahissent un joueur, [04-joueur.md](../04-joueur.md) §7). Version 0.3 : intégration des revues d'[équilibrage](../revues/equilibrage.md) §6.10 et de [sécurité](../revues/securite.md) §9.9, avec les arbitrages A2, A4 et A6 de la session principale. Version 0.2 : jonction avec [combat.md](combat.md) v0.1, profil et catégories du joueur, création du personnage persistant.
> Les renvois **E**n, **S**n et **A**n désignent les décisions de la revue d'équilibrage, de la revue de sécurité et les arbitrages de la session principale ; **IA §13.n** et **IA §14.n** renvoient aux mécanismes et mesures de la revue de l'intelligence ([ia-pnj.md](ia-pnj.md)) ; **G**n et **P**n aux décisions des revues des signatures et de la génération ; **J§7** renvoie à la décision sur les gestes qui trahissent un joueur ([04-joueur.md](../04-joueur.md), §7). **DA §n** renvoie à [direction-artistique.md](direction-artistique.md) ; **M**n, **A20** à **A28** et « signatures-contenu §n » à la revue [signatures-contenu](../revues/signatures-contenu.md), **A7** à l'arbitrage de sa v1.0.1. Aucune section n'est jamais renumérotée : une section nouvelle prend un numéro libre ou un suffixe (« §5.3 bis »).
> Les choix marqués **(décidé)** ont été tranchés par la session interface, avec leur raison. Les valeurs marquées **[à calibrer]** seront ajustées sur le prototype par des sessions (qualité et essais, avec des sessions joueuses et évaluatrices ; responsable conception pour les valeurs de jeu), jamais par une action de l'utilisateur (A8). Ce qui touche le socle ou un autre domaine est seulement noté au §15, sans être appliqué ailleurs.

## Sommaire

0. [Principes](#0-principes)
1. [Ce que le client reçoit et affiche](#1-ce-que-le-client-reçoit-et-affiche)
2. [Écran principal et HUD](#2-écran-principal-et-hud)
3. [Choisir et prendre le contrôle d'un personnage](#3-choisir-et-prendre-le-contrôle-dun-personnage)
4. [Intuitions et connaissances du personnage incarné](#4-intuitions-et-connaissances-du-personnage-incarné)
5. [Contrôles](#5-contrôles)
6. [Caméra, vue en coupe et perception](#6-caméra-vue-en-coupe-et-perception)
7. [Carte, adresses, transports](#7-carte-adresses-transports)
8. [Téléphone, courrier, achats](#8-téléphone-courrier-achats)
9. [Solo : temps et sauvegarde](#9-solo--temps-et-sauvegarde)
10. [En ligne : amis, blocage, signalement, reconnexion](#10-en-ligne--amis-blocage-signalement-reconnexion)
11. [Accessibilité, langues et réglages](#11-accessibilité-langues-et-réglages)
12. [Administration, modération et conception](#12-administration-modération-et-conception)
13. [De la 2.5D à la 3D](#13-de-la-25d-à-la-3d)
14. [Points ouverts](#14-points-ouverts)
15. [Impacts sur le socle et les autres domaines](#15-impacts-sur-le-socle-et-les-autres-domaines)

---

## 0. Principes

1. **L'écran honnête** (décidé). Tout ce que l'écran montre du monde est quelque chose que le personnage piloté **perçoit** ou **sait**. Ne sont jamais affichés : le rôle, le métier, le niveau, le palier, les catégories et le profil d'un personnage que le joueur n'incarne pas ; le pilote d'un personnage ; les valeurs des relations ; la valeur d'un savoir opaque ([04-joueur.md](../04-joueur.md), §6 et §7.1 ; [social.md](social.md), §8.2). *Pourquoi : c'est la règle du socle, et c'est elle qui donne leur sens aux témoins, aux secrets et à la découverte des gens.*
2. **Deux couches, jamais mélangées** (décidé). La couche **monde** montre ce que vit le personnage (sensations, intuitions, paroles, téléphone, courrier). La couche **joueur** montre ce qui concerne le compte (barre du joueur, synchronisation, expérience, familiarités, amis, modération). Chacune a son propre style visuel, reconnaissable d'un coup d'œil (§2.7). *Pourquoi : le joueur doit toujours savoir si une information vient du personnage ou du jeu, sinon il prendrait une aide du jeu pour un savoir du personnage, ou l'inverse.*
3. **Le monde d'abord, le HUD ensuite** (décidé). Ce qui peut passer par le monde y passe : montre et téléphone, panneaux d'horaires aux arrêts, plaques de rue, étiquettes de prix, courrier, journal local, gestes et postures des personnages. Le HUD reste minimal et **dynamique** : un élément apparaît quand il devient utile et s'efface ensuite (option « HUD permanent », §11.3). *Pourquoi : dans une ville sans figurant où l'on vit en temps réel, l'information doit être réaliste ; un HUD chargé transformerait la vie en tableau de bord.*
4. **Des mots plutôt que des chiffres internes** (décidé). Besoins, sentiments, capacités et relations s'expriment en **mots et en intensités** (léger, marqué, pressant), jamais en valeurs du cœur. Les chiffres sont réservés à ce que le personnage compte vraiment (argent, heure, prix) et aux données du compte (expérience, niveau). *Pourquoi : des jauges chiffrées pousseraient à optimiser des valeurs au lieu de jouer le personnage, et permettraient de recalculer des paliers.*
5. **Le client affiche, le serveur décide** (décidé). Le client Godot ne contient aucune règle : il reçoit une **vue du personnage** déjà filtrée et la liste des **actions possibles**, et renvoie des **intentions** (§1). *Pourquoi : c'est l'architecture du socle ([05-architecture.md](../05-architecture.md), §2), et c'est la seule protection contre un client modifié.*
6. **Indépendante de la caméra** (décidé). Les éléments d'interface sont attachés à des **entités** (un personnage, une porte, un véhicule) ou à l'écran, jamais à un angle de vue. La visée et le ciblage désignent des entités ou des points du monde, pas des pixels. *Pourquoi : c'est ce qui permet de passer de la 2.5D à la 3D sans refaire l'interface (§13).*
7. **Mêmes écrans en solo et en ligne**, sauf les différences listées aux §9 et §10 (décidé). *Pourquoi : le même serveur tourne dans les deux modes ([05-architecture.md](../05-architecture.md), §2.4) ; deux interfaces divergeraient vite.*
8. **Le joueur n'est interrompu que pour ce qui l'engage** (décidé). Les confirmations sont réservées aux actes irréversibles ou lourds (quitter un personnage, sortir de la carte, décisions avec délai, actes intimes proposés à un personnage piloté par un joueur, §5.4) ; tout le reste se fait sans fenêtre. *Pourquoi : en temps réel, chaque fenêtre coûte du temps de vie au personnage.*

---

## 1. Ce que le client reçoit et affiche

### 1.1 La vue du personnage (décidé)

Le serveur envoie au client, pour le personnage piloté, une **vue** composée de :

| Élément | Contenu | Ce qui n'y figure jamais |
|---|---|---|
| **Entités perçues** | Personnages, véhicules, objets et portes que le personnage voit ou entend, avec leur **apparence** (tenue, âge apparent, corpulence, blessures visibles, signes d'ivresse ou de négligence, posture, émotion visible, ton) | Les composants internes des autres : attributs, profil, rôle, palier, besoins, relations, pilote |
| **Décor** | L'état visible des lieux (portes, dégâts, éclairage, tags) ; la carte de la ville, qui est une donnée partagée ([05-architecture.md](../05-architecture.md), §2.11) | L'intérieur des pièces que le personnage ne voit pas (§6.4) |
| **Connaissances du personnage** | Noms, liens et métiers connus des personnes perçues ; lieux connus ; croyances récentes ; engagements | La valeur des savoirs opaques, remplacée par une **étiquette** ([social.md](social.md), §4.4) |
| **État du personnage** | Santé réelle, blessures, sensations, besoins importants en intensités, intuitions | Les valeurs numériques des besoins et des émotions |
| **Données du joueur** | Barre du joueur, tendance de la synchronisation, familiarités révélées, expérience | Le multiplicateur de palier du bilan (§3.8) |
| **Actions possibles** | Pour chaque cible, la liste des actions que le personnage **croit possibles** (admissibilité épistémique : ce qu'il perçoit et sait), avec leur **réticence** éventuelle (§2.3) ; pour un pilote `Joueur`, la liste tient aussi compte de la **garde hors fiction** (§1.4) | L'état réel que le personnage ignore (une porte verrouillée qu'il n'a pas essayée, une poche vide) ; la raison interne d'un refus, sauf quand le personnage la connaît (§4.1) |

*Pourquoi : si le filtrage était fait par le client, un client modifié afficherait tout ; en ne transmettant que la vue, le serveur garantit l'écran honnête par construction.*

**Ce que la vue ne contient jamais de plus** (décidé, S5, S6) :

- **Aucune marge au-delà du perçu** : le serveur n'envoie pas les entités « presque visibles ». Pour le personnage piloté par un joueur, la vue est **recalculée à chaque tick**, et aussitôt qu'une porte s'ouvre, pour que rien n'apparaisse en retard.
- L'**apparence** envoyée est l'apparence **perçue** (déguisement, capuche, nuit), jamais l'identité réelle.
- Une entité qui sort du champ plus de quelques secondes **[à calibrer]** revient avec une **nouvelle poignée** : le client ne peut pas reconnaître seul quelqu'un qui revient. La continuité ne passe que par les **connaissances du personnage** : l'infobulle (§4.5) affiche « Karim Belhadj » parce que le personnage le reconnaît, et une description s'il ne le reconnaît pas.

*Pourquoi : un client modifié ne peut afficher que ce qu'il reçoit ; sans marge ni identifiant durable, il ne voit pas à travers les murs et ne démasque pas un déguisé.*

En solo, le serveur tourne dans le même processus, mais le client passe par la même vue (décidé). *Pourquoi : un seul chemin de données, et le solo reste un banc d'essai fidèle du jeu en ligne.*

### 1.2 Textes et traduction (décidé)

- Les textes de l'interface, des intuitions et du dialogue de repli sont des **gabarits** identifiés, avec des paramètres (une personne, un lieu, une durée), traduits côté client. Le serveur n'envoie jamais de phrase toute faite, sauf les répliques formulées par le service de dialogue, déjà dans la langue du joueur ([social.md](social.md), §3.4).
- Les **noms propres** (personnes, rues, enseignes, quartiers) ne sont pas traduits.

*Pourquoi : c'est ce qui permet à deux joueurs de langues différentes de vivre la même scène, comme le prévoit le social.*

### 1.3 Les écrans du client (décidé)

Le client est découpé en modules indépendants, chacun alimenté par une partie de la vue : **HUD** (§2), **sélection** (§3), **carnet du personnage** (§4), **roue d'actes et conversation** (§5.4), **caméra** (§6), **plan** (§7), **téléphone** (§8), **menus du compte** (§10), **réglages** (§11). La console d'administration est une application à part (§12).

### 1.4 Actions crues possibles, garde hors fiction et refus (décidé, S4, E35, A2)

Le serveur valide une intention en deux étages ; l'interface montre le premier et rend le second perceptible.

1. **Ce que le personnage croit possible** : la liste des actions (§5.2), la roue d'actes (§5.4) et la barre de combat (§5.6.2) sont calculées sur ce que le personnage **perçoit et sait**. « Ouvrir » figure sur une porte qu'il n'a jamais essayée, même verrouillée ; « Prendre le portefeuille » figure sur une poche dont il ignore le contenu.
2. **La garde hors fiction** (pilote `Joueur` seulement) retire de cette liste ce qu'un joueur ne peut pas faire par ce personnage : flux entre personnages liés, conflit d'intérêts (E8), vote (E9), plafonds du persistant (E10), acte extrême pour le profil (E1). Une action retirée par la garde **n'apparaît pas** ; si le joueur tente le geste autrement (texte libre, objet tendu, touche de combat), une **intuition** l'explique (§4.1). Dans le monde, le personnage se comporte de façon ordinaire : il hésite, renonce, se récuse.
3. **Ce qui se passe vraiment** : à l'exécution, l'état réel décide. Une précondition que le personnage ignorait fait **échouer** l'action avec un **effet visible** : la poignée résiste, la main ne trouve rien, le tiroir est vide. Cet échec devient une connaissance du personnage (la porte est désormais connue comme verrouillée).
4. **Les refus** du serveur arrivent avec un **code fermé** parmi quatre, sans détail interne, et le client les rend par une animation et une courte phrase de voix intérieure :

| Code | Rendu |
|---|---|
| Impossible | Le geste avorte (« Vous n'y arrivez pas. ») |
| Hors de portée | Le personnage s'arrête et regarde la cible (« C'est trop loin. ») |
| Occupé | La cible ou le personnage est déjà pris (« Pas maintenant. ») |
| Interdit | Le personnage renonce ; l'intuition de la garde ou de la règle absolue s'affiche si elle existe (§4.1) |

*Pourquoi : une liste fondée sur l'état réel révélerait l'invisible (qu'une porte est verrouillée, qu'une poche est vide) à quiconque sonde les cibles ; une liste fondée sur ce que sait le personnage garantit l'écran honnête pour les actions comme pour les entités, et l'échec visible reste réaliste.*

---

## 2. Écran principal et HUD

### 2.1 Disposition (décidé)

```
┌──────────────────────────────────────────────────────────────────────┐
│ [Santé du personnage ███████████░░]          Mar. 14:32  ☁ 9°  frais │
│ [Votre vie          ████████▒▒▒░░]                                   │
│ ⚠ blessure bras g.  ◐ faim marquée                                    │
│                                                                      │
│                                                                      │
│                    (monde, vue en coupe)                             │
│                                                                      │
│                                                                      │
│ « Vous devriez être au travail. »                    [E] Parler      │
│ ✆ Lucie : « Tu passes ce soir ? »                    [maintenir] …   │
└──────────────────────────────────────────────────────────────────────┘
```

| Zone | Contenu | Visibilité |
|---|---|---|
| Haut gauche | Les deux barres de vie (§2.2), puis les signaux du corps : blessures, besoins importants (§2.4) | Barres toujours visibles pendant une incarnation ; signaux seulement quand ils dépassent un seuil |
| Haut droite | Jour, heure, météo perçue (§2.5) | Toujours visible (option : masquer) |
| Bas gauche | Intuitions (style « voix intérieure »), puis notifications du téléphone | À leur arrivée, puis s'effacent |
| Bas droite | Action contextuelle principale et indication « maintenir pour plus d'actions » (§5.2) | Quand une cible est désignée |
| Centre haut | Messages de la couche joueur (synchronisation, réservation, éjection imminente) | Rares, au moment voulu |

*Pourquoi : les barres et l'heure sont les deux informations qui servent en permanence (la santé et le temps réel) ; tout le reste n'apparaît que lorsqu'il compte.*

### 2.2 Les deux barres de vie (décidé)

Application de [03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 :

- **« Santé du personnage »** : les points de vie réels, rapportés à leur maximum.
- **« Votre vie »** : les points de vie du joueur, soit PV réels × S.
- Les deux barres sont **empilées, alignées à gauche, à la même échelle**. L'écart entre leurs extrémités est **hachuré** : c'est la désynchronisation, visible d'un coup d'œil. Une blessure raccourcit les deux barres ; un acte contraire au personnage n'agrandit que la zone hachurée.
- **Pas de chiffres** par défaut ; l'option « afficher les valeurs » ajoute un pourcentage aux deux barres (§11.3). Ce sont des données du personnage piloté et du joueur, pas des informations sur les autres.
- **Alerte avant l'éjection** : quand « Votre vie » passe sous un seuil (25 % de la santé du personnage **[à calibrer]**), la barre pulse, un son grave se fait entendre et un message de la couche joueur apparaît : « Vous perdez le fil de ce personnage. Le quitter maintenant n'y changera rien. » *Pourquoi : l'éjection coûte toute l'expérience de l'incarnation ([04-joueur.md](../04-joueur.md), §1.1) ; le joueur doit pouvoir se reprendre avant.*
- **Quitter sous l'alerte vaut éjection** (décidé, E2) : rendre le personnage, se déconnecter ou confirmer une sortie de la carte alors que « Votre vie » est sous le seuil d'alerte, ou dans les **5 minutes de jeu** **[à calibrer]** qui suivent un acte dissonant grave, donne le bilan d'une éjection. Le message reste affiché tant que la condition dure ; pour s'en sortir, le joueur doit agir fidèlement jusqu'à repasser au-dessus du seuil. *Pourquoi : l'alerte doit servir à se reprendre, pas à fuir la pénalité.*
- **Personnage persistant** : une seule barre, « Santé », puisqu'il n'a pas de synchronisation.

### 2.3 Synchronisation (décidé)

La synchronisation n'a pas de jauge propre : elle **se lit** dans l'écart entre les deux barres (S = longueur de « Votre vie » / longueur de « Santé du personnage »). S'y ajoutent :

- **Retour immédiat** : quand un acte fait baisser S, la zone hachurée s'élargit avec un bref effet, et une intuition peut l'accompagner, avec sa raison principale à partir de la précision « claire » (« Ça ne vous ressemble pas : vous ne mentez pas à vos proches. ») (décidé, signatures §5.3). Quand S remonte, la zone se résorbe doucement.
- **Réticence avant d'agir** (décidé, G24, revue des [signatures](../revues/signatures.md) §5.3) : dans la roue d'actes (§5.4), le menu des actions (§5.2) et la barre de combat (§5.6.2), chaque acte porte un **niveau de réticence** de **0 à 3**, plus un **indicateur de rupture** à part, qui forme un cinquième état (décidé, M8, revue [signatures-contenu](../revues/signatures-contenu.md) §1.3) :

| Niveau | Coût de l'acte (calculé par le serveur, jamais envoyé) | Signe | Mot |
|---|---|---|---|
| **0** Aucune | < 0,01 | Rien | — |
| **1** Légère | < 0,05 | Une marque | « Il hésite un peu » |
| **2** Nette | < 0,20 | Deux marques | « Il hésite » |
| **3** Forte | ≤ 1 | Trois marques, le geste se raidit | « Ça lui répugne » |
| **Rupture** (indicateur) | Coût supérieur à la synchronisation restante ; un coût supérieur à 1 n'est proposé que sur la barre de combat (ci-dessous) | Icône distincte, le personnage se fige | « Il ne le fera pas » (ci-dessous) |

  La rupture **remplace** l'affichage du niveau quand l'indicateur est levé (décidé). *Pourquoi : un acte en rupture n'aura pas lieu ; montrer à côté un niveau « fort » brouillerait le seul signe qui doit être sans ambiguïté.* Les seuils sont ceux de M8 **[à calibrer]** : ils placent un mensonge à un proche pour un personnage honnête (≈ 0,16) en « nette » et une gifle donnée par un pacifiste (≈ 0,25) en « forte ».

  Le niveau affiché suit la **précision** des intuitions (§4.1) : au niveau « vague », seuls le niveau fort et la rupture apparaissent ; au niveau « clair », net et fort ; au niveau « précis », les trois. À partir de la précision « claire », une **phrase de raison** apparaît au survol de l'acte (ou au maintien sur la manette) : « Vous ne mentez pas à vos proches. », « Il n'aime pas qu'on le voie perdre son calme. » ; au niveau « clair », la raison principale, au niveau « précis », jusqu'à deux raisons et la circonstance qui l'atténue s'il y en a une (« … mais il est à bout »). Le client ne reçoit, pour chaque action possible, qu'un **niveau** (0 à 3), un **indicateur de rupture** et un **code de raison** ; **jamais un coût**, une gravité ni une position sur un axe (G24). Les raisons parlent des croyances du personnage incarné, jamais de celles des autres.

  **La réticence n'est pas une sonde des secrets** (décidé v0.9, revue d'équilibrage de clôture C6) :
  - une phrase de raison ne cite **jamais** une relation ni un fait qui est un **savoir opaque** (§4.1) ;
  - pour une cible dont la classe vient d'une relation **secrète** (amant caché, enfant non reconnu), **tout ce qui s'affiche avant d'agir** est calculé avec la **classe que la relation a en apparence** (« une collègue », « un inconnu »), celle que le joueur peut connaître (décidé v0.9.2, R1 de l'équilibrage, corrige la v0.9) :
    - le **niveau affiché** et le code de raison ;
    - l'**indicateur de rupture affiché** ;
    - le choix de **proposer ou non** l'acte (coût supérieur à 1 compris) ;
  - le **vrai coût** ne s'applique qu'à l'**exécution** : la synchronisation baisse du vrai coût, et si l'acte rompt le lien, il **n'a pas lieu** (E1) : le personnage hésite et renonce, et le joueur est éjecté (§3.7). Cette éjection, venue d'une différence que l'écran ne montrait pas, ne compte **ni pour la récidive** (E3) **ni dans les refus répétés** de la détection ; son bilan ne porte pas la mention de récidive ;
  - un acte de coût **supérieur à 1**, selon la classe apparente, n'est **pas proposé** dans les listes (§1.4) ; l'indicateur ne sert alors que sur la barre de combat, où les touches restent actives.

  *Pourquoi : en survolant le même acte envers plusieurs personnes, un joueur apprendrait sinon qui est l'amant secret ou l'enfant caché de son personnage, et s'en servirait avec son persistant ; ces liens relèvent de la couche cachée. Calculer aussi la rupture et la présence de l'acte sur la classe apparente ferme la sonde : sinon l'absence de « frapper » envers le seul enfant caché désignerait le lien. Sonder coûte alors une incarnation entière pour une seule cible.* *Pourquoi : on doit pouvoir apprendre à jouer juste sans voir le profil du personnage ; des niveaux en nombre fixe et des raisons en mots enseignent la nature du personnage sans permettre de recalculer son profil, et lier leur finesse à la synchronisation récompense le joueur qui écoute son personnage.*
- **Point de rupture** (décidé, E1, A2) : un acte dont le coût estimé **dépasse la synchronisation restante** porte un signe de réticence **toujours visible et sans ambiguïté**, quelle que soit la précision des intuitions : une icône distincte (le personnage se fige) et le mot « Il ne le fera pas ». Si le joueur insiste, l'acte n'a pas lieu : le personnage hésite puis renonce (il repose l'arme, referme le coffre, raccroche), et le joueur est **éjecté** (§3.7). Un acte **extrême pour le profil**, impossible même à synchronisation parfaite, n'est pas proposé du tout dans les listes (§1.4). Sur la barre de combat, où les touches restent actives, l'attaque concernée porte le signe de rupture dès que le personnage fait face à la cible, avec l'intuition « Il ne peut pas s'y résoudre. », et insister a la même issue. À la précision « précise », l'intuition donne la raison (« Il ne peut pas s'y résoudre : frapper un homme à terre. ») (décidé, signatures §5.3). Par le texte libre, la carte de vérification affiche cette intuition et ne propose pas l'envoi (§5.4). *Pourquoi : ce qu'une pénalité ne peut pas réparer doit être empêché ; un signe toujours lisible garantit que l'éjection est un choix du joueur, jamais une surprise.* **Seule exception** (décidé v0.9.2, R1) : envers une cible liée par un **secret**, le signe suit la classe apparente, et l'éjection peut survenir sans signe préalable ; elle ne compte alors ni pour la récidive ni dans les refus répétés (rubrique ci-dessus).
- **Désynchronisation volontaire et répétée** : à la deuxième récidive rapprochée **[à calibrer]**, un message de la couche joueur rappelle que le bilan en souffrira. Rien de plus : c'est au joueur de choisir.

### 2.4 Besoins importants et état du corps (décidé)

- Le HUD ne montre que les besoins **importants** ([ia-pnj.md](ia-pnj.md), §1.1) : énergie, faim, santé et douleur, sécurité, lien social, détente, stimulation, et le **manque** d'une dépendance. Les besoins triviaux (soif, vessie, confort, petits gestes d'hygiène) sont gérés par les gestes automatiques (§5.3) ; ils n'apparaissent que si le joueur a désactivé ces gestes ou s'ils deviennent pressants.
- Chaque besoin apparaît sous forme d'**icône d'intensité** à trois niveaux (léger, marqué, pressant) **[seuils à calibrer]**, sans jauge. Au niveau pressant, l'icône s'accompagne d'une intuition (« Vous tombez de sommeil. »).
- Le **corps parle aussi** : le personnage bâille, se frotte les bras par froid, se tient le ventre, boite. Ces animations existent pour tous les personnages (le joueur peut les voir chez les autres, c'est de l'apparence).
- **Humeur** (décidé, IA §13.2) : le personnage incarné n'a **pas de jauge d'humeur**. Son humeur se voit sur son propre corps (§4.6) et se dit dans les intuitions, toujours avec sa cause au niveau « précis » (« Vous êtes d'humeur sombre depuis la dispute avec Karim. ») (§4.1). *Pourquoi : l'humeur est faite de causes datées ; la nommer avec sa cause aide à jouer juste, alors qu'une jauge à trois axes inviterait à la gérer comme un besoin.*
- Les **sensations physiques sont toujours exactes**, quelle que soit la synchronisation (décidé). *Pourquoi : la faim ou la douleur sont des sensations du corps, pas une compréhension de la vie du personnage ; seules les intuitions sur ses obligations, ses envies et ses sentiments dépendent de la synchronisation (§4.1).*
- **Blessures** : une icône par blessure significative, avec sa zone du corps ; le détail est dans le carnet (§4.2, onglet « Corps »). Le découpage en huit zones et les états (saignement, fracture) viennent du domaine combat (§5.6).
- **Personnage persistant en ligne** : pas de besoin de sommeil, puisqu'il n'est pas géré ([01-concept.md](../01-concept.md), §2) ; la fatigue d'effort reste affichée.

### 2.5 Heure, date et météo (décidé)

- **Heure** toujours affichée en haut à droite, avec le jour de la semaine ; la date complète au survol et dans le téléphone. *Pourquoi : en temps réel, les horaires (travail, bus, commerces) gouvernent la vie ; c'est l'information que le personnage consulte le plus, sur sa montre ou son téléphone.*
- **Météo perçue** : une icône du temps qu'il fait, la température et un mot de **ressenti** (frais, froid glacial, lourd) qui tient compte de la tenue du personnage. Les prévisions passent par l'application Météo, la radio ou le journal (§8.1), comme dans la vie.
- **Solo accéléré** : l'horloge indique la vitesse d'avance (§9.1).

### 2.6 Argent (décidé)

- **Espèces** portées : montant exact, visible dans l'inventaire et au moment de payer. *Pourquoi : le personnage peut compter son portefeuille.*
- **Solde du compte** : dans l'application Banque (§8.2), et sur le ticket d'un distributeur. Il n'est pas affiché en permanence.
- **Mouvements** : une notification du téléphone signale les paiements et versements notables (« Salaire reçu : 1 520,00 écus », « Prélèvement : loyer »), avec un seuil réglable.
- **Option** : épingler espèces et solde dans le HUD (§11.3).

*Pourquoi : le socle demande l'argent dans le HUD, mais personne ne voit son solde en permanence ; l'affichage à la demande, plus les notifications du téléphone, est réaliste et suffit à suivre ses finances.*

### 2.7 Les trois styles de messages (décidé)

| Style | Origine | Forme |
|---|---|---|
| **Voix intérieure** | Le personnage : intuitions, sensations, réticences | Texte en italique, sans cadre, à la deuxième personne (« Vous… ») ; petite icône de catégorie (§4.1) |
| **Téléphone** | Le monde : SMS, appels, banque, missions, actualités | Petite carte avec l'icône de l'application et l'expéditeur, comme une notification de téléphone |
| **Couche joueur** | Le jeu : synchronisation, expérience, réservation, familiarité, amis, modération | Cadre neutre au liseré distinctif, jamais à la deuxième personne du personnage |

*Pourquoi : c'est l'application directe du principe des deux couches (§0.2) ; une forme par origine suffit à ne jamais les confondre.*

**Styles graphiques des deux couches** (décidé, DA §4.3) :

| Couche | Rendu |
|---|---|
| **Monde** (voix intérieure, téléphone, bulles, icônes de son, sous-titres) | Typographie humaniste (§2.8), coins arrondis, couleurs prises dans la palette du monde, fond semi-transparent ; le téléphone a l'allure d'un vrai téléphone, sans marque |
| **Joueur** (synchronisation, expérience, réservation, familiarité, amis, modération, bulles d'aide) | Typographie de caractère neutre (§2.8), cadre fin au **liseré distinctif**, d'une couleur **jamais employée dans le monde** **[teinte à calibrer]** |

- La différence ne tient **jamais à la couleur seule** (décidé) : la couche joueur se reconnaît aussi à sa police, à son cadre rectangulaire et à son liseré, la couche monde à ses coins arrondis. *Pourquoi : avec une palette pour daltoniens (§11.1), la teinte du liseré peut se rapprocher d'une couleur du monde ; la forme suffit alors.*
- La teinte du liseré est vérifiée automatiquement contre la palette du monde (DA §2.1) **et** contre ses transpositions pour daltoniens (décidé). *Pourquoi : une couleur « jamais employée dans le monde » ne se garantit que par un contrôle, puisqu'aucun humain ne relit chaque asset (DA §0.2).*
- Les **sons d'interface** sont eux aussi distincts selon la couche (DA §5.2) : un son feutré pour la couche monde (notification du téléphone), un son neutre et bref pour la couche joueur.

### 2.8 Bulles, icônes de son et typographie (décidé, DA §4.4, §4.6)

**Bulles de dialogue** :

- Fond clair, texte sombre, **queue vers l'orateur** ; **taille fixe à l'écran** quel que soit le cran de zoom (§6.1). *Pourquoi : au zoom de quartier, une bulle à l'échelle du monde serait illisible.*
- La **forme** de la bulle dit le **volume** de l'acte, et une **icône** avec un **mot** dit son **ton** (amical, neutre, sec, menaçant, séducteur), jamais la couleur seule (décidé v0.8, [social.md](social.md) v0.7, §3.2 ; corrige la v0.6, qui donnait une forme au ton sec) :

| Volume | Forme de la bulle |
|---|---|
| **Chuchoté** | Bord en pointillés, petits caractères ; perçu seulement à très courte distance |
| **Normal** | Bord simple |
| **Élevé** | Bord double |
| **Crié** | Bord épais et dentelé, caractères plus grands |

  **[formes à calibrer]** *Pourquoi : le volume et le ton sont deux énumérations distinctes du serveur ; donner la forme au volume et l'icône au ton permet de lire de loin une confidence, une dispute ou une menace calme, que le texte soit compris ou non.*
- Les bulles sont **identiques** pour tous les orateurs, quel que soit le pilote ; les bulles « … », les bribes et les chuchotements suivent le §5.4.
- **Trois niveaux d'écoute** (décidé, [social.md](social.md) v0.7, §3.2) : pour chaque acte d'une conversation proche, le serveur décide ce que le personnage en saisit, et le client ne rend que cela.

| Niveau | Ce que reçoit le client | Rendu |
|---|---|---|
| **Compris** | La réplique formulée | Bulle avec le texte, forme du volume, icône du ton ; entrée dans le journal de conversation |
| **Bribes** | Les fragments déjà découpés par le serveur | Bulle avec les bribes (« … pas avant jeudi… ») ; entrée dans le journal, avec les bribes seulement |
| **Entendu sans être compris** | Le ton, le volume et la durée de l'acte, en énumérations fermées | Bulle **sans texte** (trois points immobiles), à la forme du volume et à l'icône du ton, le temps de l'acte ; **voix indistinctes** au son (DA §5.5) ; rien dans le journal |

  Le sous-titre des sons dit seulement « [voix indistinctes, ton vif] » ou « [on chuchote] ». Le rendu est **le même quel que soit le pilote** de l'orateur (A4) : la parole d'un joueur passe par les mêmes trois niveaux, au même délai. *Pourquoi : on devine une dispute ou une confidence sans en entendre les mots ; un journal qui garderait ce qui n'a pas été compris donnerait au joueur un savoir que son personnage n'a pas.*
- Un orateur **hors champ** a sa bulle repliée au bord de l'écran, avec une flèche vers lui, et sa réplique entière dans le journal de conversation.

**Icônes de son** (§6.4) : pictogrammes **monochromes** (voix, pas, verre, coup de feu, moteur, porte, sirène, klaxon), dont le **flou** suit l'incertitude de position envoyée par le serveur. Un son **hors champ** s'affiche au bord de l'écran par une flèche à la même incertitude. Avec l'option « Sous-titres des sons » (§11.1), un mot court accompagne l'icône.

**Indicateurs de danger** (§5.6.4) : au bord de l'écran, une **forme fléchée** à **pulsation lente**, jamais une couleur seule ; seulement pour un danger **perçu**.

**Typographie** (polices sous licence SIL Open Font License, DA §4.6 **[à vérifier au registre]**) :

| Usage | Police |
|---|---|
| Couche monde (voix intérieure, téléphone, bulles, sous-titres) | Inter (Source Sans 3 en remplacement) |
| Couche joueur | IBM Plex Sans |
| Option « Police très lisible » | Atkinson Hyperlegible, pour les deux couches |
| Option « Police adaptée à la dyslexie » | OpenDyslexic, pour les deux couches |
| Textes du monde (plaques, panneaux, enseignes, journal) | Celles de la direction artistique (Barlow pour la signalétique, familles variées pour les enseignes), rendues par le client (DA §1.5) |

- **Deux graisses** au plus par couche et une **taille minimale** de texte à 100 % d'interface (≈ 18 px en 1080 p **[à calibrer]**) (décidé). *Pourquoi : une interface dense en petits caractères se lit mal sur un téléviseur à la manette.*
- Avec les options de police, les deux couches gardent leurs **cadres** distincts (§2.7) ; les textes du monde gardent leurs polices propres. *Pourquoi : la police d'une enseigne fait partie de l'identité d'un lieu ; ce qui doit être lisible pour tous est l'interface.*

---

## 3. Choisir et prendre le contrôle d'un personnage

### 3.1 L'écran « Qui jouer ? » (décidé)

C'est l'écran d'accueil d'une partie, et celui qu'on retrouve chaque fois qu'on quitte un personnage. En fond, la ville **vue de haut**, à l'heure et sous la météo du monde, rendue à partir de la carte, **sans personnages**. *Pourquoi : la carte est déjà dans le client ; montrer des habitants exigerait de simuler et d'envoyer une zone pour un joueur qui ne pilote encore personne.*

Trois sections :

| Section | Contenu |
|---|---|
| **Votre personnage** | Le personnage persistant, ou sa création s'il n'existe pas encore (§3.2) |
| **Vos connaissances** | Les personnages avec qui le joueur a une familiarité (§3.4) |
| **Une nouvelle vie** | Une sélection aléatoire d'adultes de palier 0, jouables sans familiarité (§3.3) |

### 3.2 Créer le personnage persistant (décidé)

- **Éditeur d'apparence** : visage, corpulence, coiffure, tenue de départ, voix (détail plus bas). **Nom et prénom** libres, filtrés (noms injurieux, noms de personnes réelles connues **[liste à constituer]**). Avant le filtre, le nom est **normalisé** (forme Unicode NFKC) ; les **caractères invisibles** sont refusés, ainsi que les noms **confondables** avec les noms réservés du personnel (modérateurs, comptes du jeu) (décidé, S31) ; un nom refusé l'est avec une phrase neutre (« Ce nom n'est pas accepté. ») *Pourquoi : sans normalisation, des lettres d'apparence identique contournent le filtre et permettent d'usurper le personnel.* L'**âge** adulte est choisi dans une plage (18 à 60 ans **[à calibrer]**). L'âge est choisi **avant** les attributs, puisqu'il en fixe les bornes ; l'ordre des écrans est donc âge, attributs, apparence.
- **Attributs** ([02-personnages.md](../02-personnages.md), §1.1) : deux onglets, au choix du joueur.
  - **Répartir** : chaque attribut (force, endurance, coordination, perception, mémoire, temps de réaction, régulation émotionnelle, intelligence sociale) se règle en **crans** (neuf crans **[à calibrer]**), avec un mot sous la jauge (faible, moyen, bon, très bon, exceptionnel). Un **budget de crans** est à dépenser ; les crans minimum et maximum de chaque attribut sont **bornés par l'âge** (à 60 ans, l'endurance et le temps de réaction ne montent plus au sommet ; la mémoire et la régulation émotionnelle, si). Une info-bulle explique ce que chaque attribut change dans la vie (« la coordination compte pour conduire, viser, crocheter »).
  - **Tirer au sort** : le jeu répartit au hasard **le même budget de crans** que la répartition manuelle, dans les bornes de l'âge choisi ; le joueur voit **trois tirages** **[à calibrer]** et en garde un, ou passe à la répartition. Le tirage change la forme du personnage, pas sa valeur totale (décidé, E34). *Pourquoi : relancer le tirage, y compris par une réinitialisation, ne rapporte alors rien, et le choix entre maîtrise et surprise reste entier.*
  - Aucun chiffre interne (la valeur de 0 à 100) n'apparaît, seulement les crans et les mots. *Pourquoi : le socle laisse le choix entre maîtrise et surprise ; les crans rendent le budget lisible sans montrer l'échelle interne, et limiter les tirages empêche de tirer jusqu'à obtenir un personnage exceptionnel, ce que le budget interdit par ailleurs.*
- **L'éditeur d'apparence n'offre que l'espace de la population** (décidé, DA §3.7 ; [02-personnages.md](../02-personnages.md), §1.1). Il règle le **génome d'apparence** (DA §3.2) dans **les mêmes plages** que le générateur de population et puise dans **le même catalogue** (coiffures, pièces de vêtement, couleurs, voix) :
  - **Silhouette** : taille et corpulence en crans, dans les plages de l'âge et du sexe choisis ; la **musculature** n'est pas réglable, elle découle de la force et de l'endurance choisies (décidé). *Pourquoi : dans la population, la silhouette vient des attributs ; un persistant très musclé et faible serait une exception qui le désignerait.* L'apparence se règle donc **après** les attributs.
  - **Visage** : quelques curseurs de haut niveau (forme du visage, yeux, nez, bouche, mâchoire), chacun regroupant plusieurs des ≈ 24 paramètres du génome, plus un bouton « Autre visage » qui tire un visage dans la distribution de la population (décidé). *Pourquoi : 24 curseurs fins rendraient l'éditeur pénible et produiraient plus facilement des visages hors de la distribution.*
  - **Peau, cheveux et pilosité** : carnation, couleur et nature des cheveux, barbe ; les **rides**, le grisonnement et la calvitie suivent l'âge, comme pour les habitants. **Coiffure** choisie dans le catalogue ; elle changera ensuite chez le coiffeur.
  - **Signes distinctifs** : les mêmes que dans la population (lunettes, tatouage au motif généré choisi parmi quelques tirages, ancienne cicatrice, grain de beauté).
  - **Voix** : hauteur, timbre et débit en crans ; l'aperçu joue des **vocalisations** (rire, soupir, exclamation), puisque aucune parole n'est dite (DA §5.5).
  - **Tenue de départ** : une petite garde-robe de nouveau venu (quelques tenues par usage), composée de pièces du catalogue ; le reste s'achète en ville comme pour les habitants (DA §3.7).
  - Un bouton **« Tirer au hasard »** tire toute l'apparence dans la distribution de la population pour l'âge, le sexe et les attributs choisis.
- **Aucun objet d'apparence propre aux joueurs** (décidé, DA §3.7) : ni tenue exclusive, ni coiffure réservée, ni cosmétique payant, ni récompense visible ; les succès de plateforme (§3.9) ne débloquent rien dans le monde. *Pourquoi : une apparence que seul un joueur peut avoir le désignerait au premier coup d'œil ([01-concept.md](../01-concept.md), §1.1).*
- Aucun choix de métier ni de profil à la création : le métier se trouve en ville, le profil se construit en jouant.
- **Arrivée** : une courte scène dans le train, puis la descente à la gare d'Orvenne ([04-joueur.md](../04-joueur.md), §6). Une intuition oriente vers le foyer où trois nuits sont payées ([economie.md](economie.md), §11.1).
- **Aide de départ** : pendant les premières heures de jeu, des **bulles d'aide** de la couche joueur expliquent les commandes et les écrans au moment où ils servent (première conversation, premier achat, premier bus). Elles se désactivent dans les réglages. *Pourquoi : un tutoriel séparé n'a pas de place dans un monde vivant en temps réel ; l'aide au bon moment suffit.*

### 3.3 Une nouvelle vie : la sélection de départ (décidé)

- La sélection compte **6 personnages** **[à calibrer]**, tirés parmi les adultes de palier 0 **au repos** et en ville, de façon à couvrir des catégories comportementales différentes (la population garantit cette couverture, [population.md](population.md), §4.6).
- Chaque carte montre seulement ce qu'un **observateur** saurait : un **portrait** rendu à partir de l'apparence, à la précision « **tenue** », visage dans l'ombre (décidé v0.9 ; §4.7) ; le **prénom et le nom** (la sélection vaut présentation) ; l'**âge apparent** ; le **quartier** où il se trouve et ce qu'on le voit faire de l'extérieur (« chez lui », « dans la rue », « dans un commerce », « dort ») ; une **première impression** d'une phrase, tirée de l'apparence et de la posture (« Un homme d'une trentaine d'années, en bleu de travail, l'air pressé »).
- Ni métier, ni compatibilité, ni catégorie. *Pourquoi : les rôles sont invisibles, même au départ ; c'est en vivant la vie de quelqu'un qu'on la découvre.*
- **Pas de visage avant le choix** (décidé v0.9, renvoi de la revue de sécurité de clôture à l'équilibrage) : les candidats sont montrés à la précision « tenue », visage dans l'ombre ; le visage n'apparaît qu'une fois le personnage incarné. Le nom reste affiché : sans visage, il ne permet pas de reconnaître la personne plus tard, puisque la tenue change chaque jour. *Pourquoi : en parcourant la sélection, un client modifié pourrait sinon constituer une base de visages et de noms, réutilisable ensuite avec un autre personnage ; c'est de la méta-connaissance que le monde ne donne à personne.*
- Le joueur peut **renouveler** la sélection **5 fois par jour réel** **[à calibrer]** ; le nombre de renouvellements restants est affiché (couche joueur). Une carte reste proposée tant qu'elle n'est ni jouée ni renouvelée (décidé, E14). *Pourquoi : vivre la vie de quelqu'un demande du temps ; parcourir des centaines d'habitants en quelques secondes reviendrait à un annuaire, et chaque tirage réserve brièvement des personnages.*

### 3.4 Vos connaissances : la familiarité (décidé)

La familiarité est une donnée du compte ([social.md](social.md), §2.1). L'écran la présente **par personnage incarné**, puisqu'elle est propre au triplet (joueur, A, B) et ne se cumule pas ([04-joueur.md](../04-joueur.md), §6) :

```
Grâce à Marc Dumas (personnage incarné)
  ├─ Lucie Ferrand       bien connue   « Vous pourriez l'incarner. »
  ├─ Karim Belhadj       connu         « Il vous faudrait mieux le connaître. »
  └─ une collègue rousse à peine connue
Grâce à votre personnage
  └─ Paul Garnier        connu         « Il vous faudrait plus d'expérience. »
```

L'écran n'affiche **aucun état de disponibilité** en continu : ni « Indisponible », ni « Absent d'Orvenne ». On ne l'apprend qu'en demandant à incarner (§3.5) (décidé, E26, A6). Seuls les états définitifs, connus de tous, s'affichent (« Décédé(e) », « A quitté Orvenne »).

**Ce que révèle chaque étape** (application du socle : le nom, puis des traits, puis la jouabilité) :

| Étape | Mot affiché | Ce qui est révélé |
|---|---|---|
| 1 | À peine connu(e) | Une description (« une collègue rousse ») et le contexte (« croisée au travail ») |
| 2 | Connu(e) | Le **nom** |
| 3 | Bien connu(e) | Deux ou trois **traits de caractère** en mots (« réservée », « soupe au lait ») |
| 4 | Familier, familière | La **jouabilité** (ci-dessous) |

- **Aucune valeur ni seuil** n'est affiché. *Pourquoi : le seuil dépend du palier ([04-joueur.md](../04-joueur.md), §6) ; afficher la progression vers lui révélerait le palier.*
- **Jouabilité** : une seule phrase. « Vous pourriez l'incarner », ou la **nature** de l'obstacle, sans quantité : « Il vous faudrait plus d'expérience » (niveau de compte), « Il vous faudrait mieux le connaître » (familiarité), « Il est trop loin de votre façon de jouer » (profil), « Personne ne peut l'incarner » (hors catégorie). *Pourquoi : le joueur doit savoir sur quoi progresser ; une indication vague (« quelqu'un d'exigeant ») se devine déjà dans le monde, alors qu'un palier précis ne doit jamais apparaître.*
- **Hors catégorie clandestin** (décidé, E26) : pour un hors catégorie dont la fonction **n'est pas publique** (un parrain, par exemple), la jouabilité affiche « Il vous faudrait plus d'expérience » jusqu'au niveau de compte **100** **[à calibrer]**, puis « Personne ne peut l'incarner ». *Pourquoi : « Personne ne peut l'incarner » désignerait le chef caché d'un réseau à quiconque le connaît un peu.*
- **Personnage persistant d'un autre joueur** : il est présenté **exactement comme un habitant** : sa jouabilité est calculée comme pour les autres, et toute demande aboutit à une réservation qui expire sans motif (§3.5), qu'il soit connecté ou non (décidé, E26). *Pourquoi : toute réponse particulière (« ne peut pas être incarné », ou un « Absent d'Orvenne » qui suivrait ses connexions) révélerait qu'un joueur se trouve derrière ; il ne se distingue plus d'un habitant souvent occupé.*
- **Mineurs** : ils n'apparaissent jamais dans la liste avant 18 ans.
- **Familiarité devenue inutile** : quand B meurt ou quitte définitivement la ville, son entrée passe en « Décédé(e) » ou « A quitté Orvenne », et un message de la couche joueur le signale une fois ([population.md](population.md), §7.3 et §7.5). Quand A meurt, toutes les familiarités obtenues grâce à A sont barrées avec la mention « perdues avec la mort de A ». Les entrées barrées s'effacent au bout de 30 jours de jeu **[à calibrer]**.
- **Aucune localisation** : l'écran ne dit jamais où se trouve B. *Pourquoi : la position d'un personnage est une information que ni le joueur ni ses personnages n'ont.*
- Le joueur peut **marquer** une connaissance comme favorite, et, quand une demande lui a répondu « Absent d'Orvenne » (§3.5), demander à **être prévenu de son retour** (message de la couche joueur).

### 3.5 Indisponibilités et réservation courte (décidé)

La disponibilité ne s'affiche pas en continu (§3.4) : le joueur demande à **incarner**, et la réponse dépend de la situation réelle (décidé, E26, A6).

| Situation réelle | Réponse à la demande |
|---|---|
| Au repos, jouable | Prise de contrôle (§3.6) |
| Pas au repos (conduit, travaille sur une tâche délicate, en conversation, menacé) | **Réservation**, qui aboutit ou expire sans motif |
| Piloté par un autre joueur | Réservation |
| Personnage persistant d'un autre joueur, connecté ou non | Réservation |
| Inconscient, hospitalisé en état grave | Réservation |
| **Repos de l'IA** après une longue incarnation (palier 5 ou plus, E17) | Réservation |
| À l'extérieur (vacances, déplacement, détention, navetteur au travail) | « Absent d'Orvenne », sans date ni motif ; « Me prévenir de son retour » |
| Décédé, parti définitivement | « Décédé(e) », « A quitté Orvenne » (affiché aussi dans la liste) |

- Toutes les situations qui mènent à une réservation ont **la même apparence**. *Pourquoi : distinguer « piloté par un joueur », « persistant d'un joueur » ou « au repos de l'IA » de « occupé » révélerait le pilote, que le socle interdit d'afficher ([01-concept.md](../01-concept.md), §1.1).*
- **Repos de l'IA** (E17) : après une incarnation de durée D d'un personnage de palier 5 ou plus, il n'est accessible à aucun joueur pendant min(D/2, 4 h) réelles, et au même cercle pendant min(D, 8 h) ; un compte ne peut pas l'incarner plus de 8 heures par jour réel **[valeurs à calibrer]**. Rien de tout cela n'est affiché : la demande aboutit à une réservation qui expire.
- **Réservation** ([ia-pnj.md](ia-pnj.md), §7.3) : un bandeau de la couche joueur indique « Réservation : Lucie Ferrand — en attente », avec un indicateur d'attente **sans compte à rebours** et un bouton **Annuler**. Aucune position dans une file d'attente n'est affichée. Une réservation qui n'aboutit pas **expire sans motif après un délai tiré au hasard** dans une fourchette (2 à 6 minutes **[à calibrer]**). *Pourquoi : savoir qu'un autre joueur vise le même personnage serait une information sur les joueurs, et un délai d'expiration fixe ou lié à la cause permettrait de distinguer les situations.*
- **Une seule réservation active par compte** ; après un refus du « Passer maintenant ? » ou une expiration, le même compte ne peut pas réserver le même personnage avant **10 minutes** **[à calibrer]** ; le bouton l'indique (« Réserver à nouveau dans 8 min ») (décidé, E30). *Pourquoi : une réservation doit servir à jouer un personnage, pas à le bloquer ni à sonder sa disponibilité en boucle.*
- Quand la réservation aboutit :
  - si le joueur est sur l'écran « Qui jouer ? », la prise de contrôle se fait **aussitôt** ;
  - s'il incarne déjà quelqu'un, une proposition apparaît : « Lucie Ferrand est disponible. Passer maintenant ? », avec un délai de 15 secondes **[à calibrer]** pendant lequel la réservation tient. *Pourquoi : un changement automatique en pleine action surprendrait le joueur, alors que le personnage visé, lui, ne doit pas attendre longtemps.*
- Quand elle échoue : « La réservation a expiré », sans motif.

### 3.6 Prendre le contrôle : le passage de relais (décidé)

1. **Transition** : la caméra s'élève au-dessus du personnage quitté, glisse sur le plan jusqu'au nouveau, puis redescend (2 à 3 secondes **[à calibrer]**). Ce temps couvre la montée du personnage en simulation complète et la création de sa bulle ([ia-pnj.md](ia-pnj.md), §7.2).
2. **Fiche d'arrivée** (voix intérieure, refermable d'un geste) : « Vous êtes Lucie Ferrand, 34 ans. Vous êtes chez vous, rue des Tanneurs. » Puis l'**activité héritée** (« Vous étiez en train de cuisiner » : **Continuer** ou **Arrêter**, [ia-pnj.md](ia-pnj.md), §7.4), les sensations présentes et les deux ou trois intuitions les plus fortes.
3. **Réveil** : si le personnage dormait, l'écran s'ouvre en fondu flou et la fiche dit « Vous vous réveillez » ; la dette de sommeil se lit dans les besoins et dans des effets visuels légers (vision moins nette, réactions plus lentes) ([04-joueur.md](../04-joueur.md), §5).

**Incarnations courtes** (décidé, E14) : une incarnation de moins de **10 minutes** ne rapporte rien (§3.8). Au-delà de **6 incarnations courtes** dans la journée réelle **[à calibrer]**, chaque nouvelle prise de contrôle attend **10 minutes** ; l'écran « Qui jouer ? » l'annonce en couche joueur (« Prochaine prise de contrôle possible dans 7 min : vos dernières vies ont été très brèves. »). *Pourquoi : la règle doit être comprise pour être respectée ; elle porte sur le comportement du joueur, qu'on peut lui dire en clair.*

### 3.7 Quitter un personnage (décidé)

| Fin | Déroulé à l'écran |
|---|---|
| **Rendre le personnage** | Menu ou touche dédiée, **maintenue une seconde** (pas de fenêtre de confirmation). L'IA reprend dans le même tick ; la caméra s'élève, puis le **bilan** s'affiche (§3.8), puis l'écran « Qui jouer ? » |
| **Sortie de la carte** | Quand le personnage atteint une sortie (route, gare, ponton), une confirmation : « Quitter Orvenne mettra fin à votre incarnation de Lucie Ferrand. » **Partir** : bilan normal, l'IA décide du séjour. **Rester** : le personnage fait demi-tour ([ia-pnj.md](ia-pnj.md), §9.5) |
| **Éjection** | L'image se désature, un message de la couche joueur dit « Vous avez perdu le fil de Lucie Ferrand », la commande est retirée, la caméra s'éloigne pendant que le personnage continue sa vie, puis le bilan affiche les pénalités ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §1) |
| **Mort du personnage incarné** | Image figée, fondu au noir, un écran sobre avec son nom et ses dates ; puis le bilan, qui indique si la mort était fidèle à sa nature (aucune pénalité) ou non ([04-joueur.md](../04-joueur.md), §1.1) |
| **Déconnexion** | Comme « rendre », le bilan s'affichant à la prochaine connexion |

**Sous l'alerte** : rendre, se déconnecter ou partir par une sortie de la carte sous le seuil d'alerte, ou peu après un acte dissonant grave, donne le bilan d'une **éjection** (§2.2, E2) ; le message d'alerte l'a annoncé.

**Bilan provisoire pendant une heure** (décidé, E4) : **tout** bilan porte la mention **« provisoire »** pendant **1 heure de jeu** **[à calibrer]** après la fin de l'incarnation, quelle que soit la façon de partir. Pendant ce délai, il peut encore changer : la mort du personnage des suites d'un affrontement en cours, imprudence comprise ([combat.md](combat.md), §11.5), ou une **complicité différée** (une porte laissée ouverte, une clé remise, une alarme coupée, qui sert ensuite à un crime). Le bilan définitif arrive en message de la couche joueur ; s'il a changé, il dit pourquoi en clair (« La porte que vous avez laissée ouverte a servi à un cambriolage. »). *Pourquoi : un acte doit être jugé sur ce qu'il a permis, et rendre le personnage juste avant le coup fatal ou juste après avoir ouvert la porte ne doit rien effacer ; la règle des 10 minutes d'affrontement de la v0.2 est absorbée par celle-ci.*

**Personnage persistant** :

- **Quitter le jeu** dans un **lieu de retrait** (logement, chambre payée, transport qui part, sortie de la carte) est immédiat.
- **Ailleurs**, un compte à rebours de la couche joueur indique « Votre personnage rentre chez lui (≈ 1 min) » ([02-personnages.md](../02-personnages.md), §1.1) ; pendant ce temps, le joueur voit son personnage rester sur place puis s'éloigner sous la conduite de l'IA. Le joueur peut **annuler** et reprendre la main tant que le personnage n'a pas disparu.
- **Menacé** au moment de partir (affrontement, poursuite, interpellation, garde à vue) : le compte à rebours est remplacé par « Votre personnage ne peut pas partir tant qu'il est menacé », avec le **plafond** restant (≈ 5 min **[à calibrer]**) ; pendant ce temps, l'écran montre que le personnage **se protège sans riposter** et se rend si la police l'interpelle ([combat.md](combat.md), §11.5). *Pourquoi : le joueur doit comprendre que la déconnexion ne le sauve pas, et pourquoi son personnage ne se bat plus.*
- **La menace vaut aussi dans un lieu de retrait** (décidé, E19) : tant que le persistant est menacé (affrontement, poursuite, interpellation en cours, policier qui vient pour lui avec un motif, fiche de recherche et policier en vue), franchir sa porte, monter dans un transport ou atteindre une sortie ne le fait **pas disparaître**. Le même message « Votre personnage ne peut pas partir tant qu'il est menacé » s'affiche **derrière sa porte**, dans le bus ou à la sortie, avec le plafond restant. Un véhicule qui ne lui appartient pas ne quitte jamais la carte avec lui ; la confirmation de sortie le dit (« Ce véhicule restera à Orvenne. »). *Pourquoi : un lieu de retrait est l'endroit où l'on rentre chez soi, pas une porte de secours en pleine poursuite ; le joueur doit le voir au moment où il croit s'en tirer.*
- Les **lieux de retrait** sont marqués sur le plan (§7.2) et une icône discrète apparaît dans le HUD quand le personnage s'y trouve. *Pourquoi : la ville demande qu'on signale les lieux de retrait et le délai ([ville.md](ville.md), §11.2) ; le joueur doit savoir où et quand il peut partir sans risque.*
- **Mort** : fondu, puis réveil à l'hôpital de Beaulieu ou à la clinique de Hautevigne ; la fiche commence par « On vous a réanimé », puisque dans le monde il a été gravement blessé puis sauvé, jamais « mort » ([combat.md](combat.md), §2.5). Elle liste les **séquelles en mots** (« Votre jambe droite est fragile ; la conduite vous angoisse »), leur durée en solo, et en ligne la **perte de forme** appliquée d'un coup ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §3), les frais d'hôpital et les objets perdus ou saisis.
- **Blessure grave en ligne** : même réveil et même fiche, sans malus psychologique ([combat.md](combat.md), §2.4).
- **Condamnation en ligne** : la **fiche des conséquences** (§5.6.8) liste ce qui s'applique aussitôt ([combat.md](combat.md), §9.5).

### 3.8 Le bilan d'une incarnation (décidé)

```
Bilan provisoire — Lucie Ferrand            2 h 15 de jeu actif
  Ce que vous avez accompli pour elle
    ✓ Arrivée à l'heure à l'atelier
    ✓ 9 tâches de travail
    ✓ A rappelé sa sœur
  Fidélité à son caractère              très bonne
    Ce qui lui a coûté : mentir à sa sœur ; perdre son calme au travail
    Ce qui l'excusait : elle protégeait son fils
  Expérience gagnée                     + 410
  Familiarités
    Karim Belhadj : connu → bien connu
  Niveau 14 (▲)
  Définitif dans 1 h de jeu
```

- **Arrondis** (décidé, E26) : l'expérience est arrondie à **5 près** et le temps actif à **5 minutes près**. *Pourquoi : des valeurs exactes, comparées d'une incarnation à l'autre, permettraient de recalculer le multiplicateur de palier.*
- **Incarnation courte** (décidé, E14) : moins de 10 minutes ne rapporte ni temps actif, ni objectif, ni familiarité ; le bilan le dit en une ligne (« Incarnation trop brève : rien n'est compté. »), sans autre détail. *Pourquoi : c'est ce qui retire tout intérêt à la sonde de dix minutes.*

- Il liste, **après coup**, les **objectifs du personnage** accomplis par le joueur, sans leur valeur en expérience. *Pourquoi : apprendre ce que le personnage cherchait est la meilleure leçon pour jouer juste la prochaine fois, et ne révèle plus rien d'utile une fois l'incarnation terminée.*
- La **fidélité** (la synchronisation moyenne) est donnée **en mots**, sur cinq degrés **[seuils à calibrer]**.
- Le total d'expérience est exact. Le **multiplicateur de palier** n'est **jamais** montré, ni la valeur de chaque objectif. *Pourquoi : avec le temps actif, la synchronisation exacte et le barème, le joueur pourrait recalculer le palier ([04-joueur.md](../04-joueur.md), §1.1), que le socle interdit d'afficher.*
- **« Ce qui lui a coûté » et « Ce qui l'excusait »** (décidé, G25) : sous la fidélité, deux rubriques **en mots**, sans chiffre. La première donne **au plus trois raisons** de dissonance, agrégées sur l'incarnation et rangées par poids (« mentir à sa sœur », « perdre son calme au travail ») ; la seconde, les **justifications** qui ont joué (« elle protégeait son fils », « il était menacé »). Une rubrique vide n'est pas affichée. Les raisons tirées de ce que des témoins ont pensé ne figurent jamais. *Pourquoi : comprendre pourquoi un acte était dissonant est la meilleure leçon pour jouer juste, et une justification reconnue apprend au joueur que le jeu tient compte du contexte ; le bilan venant après coup, il ne révèle plus rien d'utile au jeu en cours.*
- Les **pénalités** sont détaillées en clair (éjection, mort à contre-emploi, récidives), parce qu'elles relèvent du comportement du joueur. S'y ajoutent (décidé, E3, E4, E1) : la **complicité différée** (« Complicité : la porte laissée ouverte a servi à un cambriolage »), la **désynchronisation grave** et sa **récidive** (« Désynchronisation grave, deuxième en 30 jours : la pénalité est doublée »), avec la perte de niveau éventuelle, et le **renoncement au point de rupture** (« Lucie Ferrand ne pouvait pas s'y résoudre : éjection »).
- Pendant l'incarnation, **aucun compteur d'expérience** n'est visible. Un objectif accompli se traduit seulement par une intuition de satisfaction (« Vous êtes soulagée d'être à l'heure. »). *Pourquoi : un compteur pousserait à chasser l'expérience au lieu de vivre la vie du personnage, ce que les garde-fous du socle cherchent justement à éviter.*
- **Personnage persistant** : un bilan de session s'affiche à la déconnexion (ses actions alimentent le bilan du compte à 50 %).

### 3.9 Le compte (décidé)

L'écran du compte (couche joueur) montre : le **niveau** et l'expérience, séparément en solo et en ligne ; l'historique des incarnations (personnage, durée, fidélité, expérience) ; le **portrait du joueur**, c'est-à-dire son profil comportemental décrit **en phrases** (« Vous jouez plutôt des personnages posés, francs et respectueux des règles »), accompagné des **noms de ses catégories** (« ISTJ — Légaliste », sous les noms propres au jeu), sans aucune valeur ([04-joueur.md](../04-joueur.md), §2) ; l'**évolution** du portrait (une phrase quand une catégorie est gagnée ou perdue : « Vos dernières vies vous rapprochent des tempéraments impulsifs »). *Pourquoi : les catégories sont le repère qui explique quels personnages lui sont accessibles ; ce sont les siennes, et elles ne disent rien des catégories des autres, qui restent invisibles.*

**Sécurité du compte** (décidé, S13) :

- **Actions irréversibles** (réinitialisation, changement d'e-mail, suppression du compte) : **réauthentification**, puis un **délai d'annulation de 72 heures**, pendant lequel un bandeau de la couche joueur et un e-mail permettent d'annuler. *Pourquoi : un voleur de session ne doit pas pouvoir tout détruire en une minute.*
- **Appareils connectés** : la liste des sessions ouvertes (appareil, date, lieu approximatif), avec la **révocation** de chacune ou de toutes.
- **Double authentification** (application TOTP) proposée dès la création du compte, et activable à tout moment.

**Réinitialisation** (décidé, E23, arbitrage « Compte ») : en ligne, elle est possible **une fois tous les 90 jours réels** ; l'écran indique la prochaine date possible. Elle remet à zéro ce qui est dans le jeu (niveau, profil, familiarités, personnage persistant), et l'écran dit en clair ce qu'elle **ne touche pas** : la **note de conduite**, les sanctions, les blocages et l'historique des abus. Une réinitialisation faite pendant une procédure judiciaire ouverte ou un délai d'épreuve est signalée avant confirmation (« Elle sera inscrite à l'historique de votre compte. »). Elle suit le parcours des actions irréversibles (réauthentification, puis 72 heures d'annulation), puis la confirmation écrite (taper le nom du personnage persistant). *Pourquoi : la réinitialisation est un nouveau départ dans le monde, pas l'effacement d'un comportement de joueur.*

**Vos données** (décidé, E8, arbitrage « Compte ») : l'**export des données** du joueur contient tout ce que le compte enregistre sur lui, y compris un éventuel **rapprochement de comptes** décidé par la modération (§12.1), avec un lien pour le **contester**. Depuis la v0.9.1, le rapprochement est aussi **notifié** comme toute mesure lourde (décidé, arbitrage A11, §10.4), sans nommer l'autre compte. *Pourquoi : le joueur a droit à ses données et à être informé d'une mesure qui le concerne ; l'autre compte et l'origine du dossier restent tus pour ne pas apprendre aux tricheurs ce qui les a trahis.*

**Succès de plateforme** (décidé, S24, arbitrage « Compte ») : s'il y en a, ils sont **purement cosmétiques** et ne dépendent **jamais du niveau solo** ni d'aucune donnée solo ; ils ne donnent rien dans le jeu. *Pourquoi : le solo n'a pas d'anti-triche ; rien de ce qu'il produit ne doit avoir de valeur hors de lui.*

---

## 4. Intuitions et connaissances du personnage incarné

### 4.1 Les intuitions (décidé)

Le cœur produit les intuitions sous forme **structurée** (catégorie, sujet, intensité, niveau de précision) ; le client les formule avec des **gabarits** traduits (§1.2). *Pourquoi : des gabarits sont instantanés, gratuits et identiques hors ligne, contrairement au service de dialogue.*

**Catégories** (une icône chacune) : obligation (travail, rendez-vous, enfants), besoin, relation, argent, projet, danger, morale (réticence, regret), **état d'âme** (humeur et pensées), **expérience** (lieux, commerces, trajets, activités).

**Ce qui nourrit les intuitions** (décidé, IA §13.2, §13.4, §13.6) : en plus des objectifs, les **pensées** actives (« Cette dispute vous trotte dans la tête. »), les **thèmes de vie** (« Vous en voulez toujours à Paul. », « Le deuil de votre père ne vous quitte pas. »), les **conclusions** (« Vous vous méfiez de ce quartier depuis l'agression. ») et les **valeurs apprises** (« Vous n'aimez pas ce quartier la nuit. », « Le café de la gare est meilleur que celui-ci. »). Elles suivent la même précision que les autres : au niveau vague, « Quelque chose vous pèse » ; au niveau précis, la cause et la date. Les valeurs apprises comprennent ce que le personnage a vécu pendant une incarnation, la vôtre comprise. *Pourquoi : c'est ce qui donne au joueur la mémoire et les goûts du personnage, et le moyen de les respecter.*

**Approche d'une crise** (décidé, IA §13.2) : quand l'humeur du personnage incarné reste longtemps très basse, une intuition pressante annonce le besoin de la crise que son profil choisirait (« Vous avez besoin de sortir marcher. », « Vous avez envie de pleurer. ») et le menu des actions propose l'acte correspondant, sous le nom « Se laisser aller » ; il soulage comme chez un habitant. Une crise violente reste filtrée par le profil et les garanties envers les mineurs. *Pourquoi : la crise est une recette de l'IA ; pour un pilote joueur, elle devient un appel du personnage, que le joueur peut suivre ou retenir, au prix d'une humeur qui dure.*

**Précision** : un indice combine la **synchronisation** courante et la **familiarité** du joueur avec le personnage incarné (la meilleure de ses familiarités avec lui) **[pondération à calibrer]**. Un personnage de départ, sans familiarité, commence au niveau « clair » si la synchronisation est bonne.

| Niveau | Exemple (obligation) | Exemple (relation) | Exemple (argent) |
|---|---|---|---|
| **Vague** | « Quelque chose vous tracasse au sujet du travail. » | « Vous pensez à quelqu'un de votre famille. » | « L'argent vous inquiète. » |
| **Clair** | « Vous devriez être au travail. » | « Votre frère vous manque. » | « Cette dette vous pèse. » |
| **Précis** | « Vous devriez être à l'atelier depuis 9 h ; votre patron va mal le prendre. » | « Votre frère vous manque ; vous ne l'avez pas appelé depuis un mois. » | « Vous devez encore 300 écus à Karim, et il les attend pour la fin de la semaine. » |

**Explications de dissonance** (décidé, signatures §5.3 et §5.4) : les phrases qui disent **pourquoi** un acte coûte (réticence au survol, retour immédiat, rupture, bilan) sont des **gabarits** propres, qui suivent les mêmes degrés :

| Niveau | Réticence au survol | Retour après l'acte |
|---|---|---|
| **Vague** | Aucune phrase | « Ça ne vous ressemble pas. » |
| **Clair** | La raison principale (« Vous ne mentez pas à vos proches. ») | La raison principale |
| **Précis** | Jusqu'à deux raisons et la circonstance atténuante (« Vous ne mentez pas à vos proches, mais vous voulez la protéger. ») | Les raisons, et pour une rupture l'acte en cause (« frapper un homme à terre ») |

Le serveur envoie un **code de raison**, jamais un nombre ; le client choisit le gabarit d'après le code et le niveau de précision. *Pourquoi : une raison détaillée vaut une partie du profil ; la réserver à la synchronisation et à la familiarité garde la récompense là où le socle la met.*

**Les trois formes des gabarits** (décidé, revue [signatures-contenu](../revues/signatures-contenu.md) §9) : chaque code de raison (raisons tirées des effets, justifications, planchers de rupture) a trois formes écrites à la main et traduites, sans moteur de conjugaison :

| Forme | Employée pour | Personne | Exemple |
|---|---|---|---|
| **Claire** | Réticence au survol et retour immédiat, à la précision « claire » | 2e (voix intérieure) | « Ça ne vous ressemble pas de mentir. » |
| **Précise** | Les mêmes, à la précision « précise » | 2e, avec la cible | « Vous ne mentez pas à vos proches. » |
| **Nominale** | Bilan (« Ce qui lui a coûté », « Ce qui l'excusait », §3.8), raison d'une rupture (§2.3), vue de conception (§12.3) | Infinitif, 3e personne | « mentir à sa sœur » |

À la précision « vague », aucun gabarit de raison n'est employé : seulement « Ça ne vous ressemble pas. » (tableau ci-dessus). Au niveau « précis », deux raisons et une circonstance se joignent par « , mais » (« Vous ne mentez pas à vos proches, mais vous voulez la protéger. ») **[jointures à calibrer par langue]**.

**Variables**, remplies par le client (décidé) :

| Variable | Contenu | D'où le client la tient |
|---|---|---|
| `{cible}` | Forme claire : la **classe** seule (« un proche », « un inconnu », « quelqu'un de fragile », « une enseigne ») ; formes précise et nominale : le **lien** connu du personnage (« votre sœur » ou « sa sœur » selon la personne du gabarit) | Classe de cible envoyée avec le code ; lien tiré des connaissances du personnage incarné (§4.3) |
| `{à_cible}`, `{de_cible}` | La même cible avec sa préposition, contractée (« au voisin », « de la patronne ») | Idem, déclinée par la table de la langue |
| `{Cible}` | La cible en début de phrase, avec majuscule | Idem |
| `{groupe}` | La pratique ou le groupe (« la famille », « vos collègues ») | Code de groupe ou de pratique envoyé avec la raison |
| `{infraction}` | Le nom de l'infraction du catalogue (« un vol ») | Code d'infraction envoyé avec la raison |
| `{besoin}` | Le besoin critique (« vous avez faim ») | Code de besoin envoyé avec la raison |
| `{e}` | L'accord au féminin du **sujet** du gabarit : le personnage incarné dans les formes claire et précise, la personne nommée dans la forme nominale | Sexe du personnage incarné ; sexe apparent de la cible |

- Une cible que le personnage **ne connaît pas** reste désignée par sa **classe**, même à la précision « précise » (« un inconnu »), et jamais par un nom ou un lien qu'il ignore (décidé). *Pourquoi : un gabarit ne doit pas donner au joueur une identité que l'écran honnête lui cache.*
- Les codes de classe, de groupe, d'infraction et de besoin viennent de **catalogues fermés**, comme les codes de raison ; aucune chaîne libre n'arrive du serveur dans un gabarit (décidé). *Pourquoi : une chaîne libre pourrait porter un nombre ou un nom interdit, et ne se traduirait pas.*
- Aucun gabarit ne contient de nombre (G24) ni d'affirmation sur la vie passée du personnage, qui pourrait être fausse (signatures-contenu §9.1).

**Quand elles apparaissent** :

- **spontanément**, quand un objectif monte fortement ou qu'une situation l'éveille (croiser quelqu'un, passer devant son lieu de travail), au plus une par minute environ **[à calibrer]**, les dangers en premier ;
- **à la demande** : la touche **Réfléchir** affiche les trois intuitions les plus fortes du moment ([ia-pnj.md](ia-pnj.md), §5.5 : les plus fortes correspondent aux objectifs au score le plus élevé). *Pourquoi : le joueur peut chercher conseil auprès de son personnage à tout moment, sans liste d'objectifs.*

**Règles** :

- Une intuition ne révèle **jamais** un savoir opaque : elle emploie son étiquette (« Vous savez comment ouvrir le coffre de la bijouterie. »).
- Elle ne parle **que du personnage incarné** : jamais des sentiments réels des autres, sauf ce qu'il **devine** selon sa perception et son intelligence sociale (« Il a l'air pressé de conclure. »).
- **Impossibilités** : quand une intention n'est pas proposée pour une raison que le joueur doit comprendre, une intuition l'explique. L'acte n'apparaît pas dans la roue ni dans le menu des actions ; l'intuition s'affiche si le joueur tente le geste autrement (texte libre, objet tendu, touche de combat) (§1.4). Les intuitions de la **garde hors fiction** (décidé, A2) :

| Cas | Intuition |
|---|---|
| Don ou échange hors bornes entre personnages liés au même joueur ([economie.md](economie.md), §12.3) | « Il ne ferait pas ça. » |
| **Conflit d'intérêts** (E8) : décision institutionnelle, acte de procédure, vote d'élu, prêt bancaire, retrait de plainte, alibi, refus d'identifier ou aveu, quand un personnage du **cercle** du joueur en est bénéficiaire, suspect ou partie | « Vous êtes trop proche de cette affaire. » |
| **Vote** d'un électeur incarné (E9) : le bulletin est celui que le personnage aurait choisi ; l'action « Voter » porte l'étiquette opaque « Voter (votre choix est fait) » | « Votre vote n'appartient qu'à vous. » (si le joueur cherche à choisir le bulletin) |
| **Plafonds du persistant** (E10) : candidature ou nomination à une fonction hors catégorie, acquisition, promotion ou prise de tête qui dépasserait le seuil du palier 9 | « Ce serait trop pour vous. » |
| **Acte extrême pour le profil** (E1, §2.3) | « Il ne peut pas s'y résoudre. » |

  Ces phrases ne disent jamais la règle dans les termes du jeu ; dans le monde, le personnage se récuse, hésite ou renonce. *Pourquoi : le joueur doit savoir qu'une voie lui est fermée, sans apprendre ni le cercle, ni le palier, ni le profil qui la ferment.*
- **Négociation** : l'intuition sur la limite de l'autre (« Il ne descendra plus beaucoup ») suit sa propre précision, fondée sur la perception et l'intelligence sociale du personnage incarné ([social.md](social.md), §3.7).
- **Récusation** : la récusation de [combat.md](combat.md), §11.8 est le cas judiciaire du conflit d'intérêts ci-dessus, et prend la même intuition (« Vous êtes trop proche de cette affaire. »), qui remplace celle de la v0.2 (décidé, E8). Elle ne dit pas pourquoi dans les termes du jeu : pour le monde, le personnage se récuse comme on le fait quand on est trop proche d'une affaire. *Pourquoi : une seule notion, une seule phrase.*
- **Témoignage et aveu** (E8) : le témoignage d'un personnage incarné est construit à partir de ce qu'il a **réellement perçu** : le sujet de l'acte « témoigner » ne propose que des faits de sa mémoire. Un aveu sur un fait qu'il ne connaît pas reste une affirmation sans fondement, et la carte de vérification du texte libre le dit (§5.4).
- **Interdiction de séjour** : à l'approche d'un quartier interdit, puis à l'intérieur, une intuition prévient (« Vous n'avez pas le droit d'être ici. ») ([combat.md](combat.md), §9.5).
- **Version déjà donnée** (décidé, [social.md](social.md), §4.7) : quand le personnage incarné a menti sur un fait sensible, une intuition lui rappelle la version donnée et à qui, dès que le sujet revient ou que l'interlocuteur est présent (« Vous avez dit à Karim que vous étiez chez votre sœur ce soir-là. ») ; au niveau vague, seulement « Vous avez déjà raconté quelque chose à ce sujet. ». Le registre des déclarations n'est jamais affiché. Le texte libre ou la roue permettent de s'en écarter ; la carte de vérification signale alors « Ce n'est pas ce que vous lui avez dit. ». *Pourquoi : le joueur ignore ce que le personnage a dit avant son arrivée ; perdre ce fil sans indice serait injuste, et s'en écarter doit rester un choix conscient, jugé comme un mensonge contradictoire.*
- **Usages d'une pratique** (décidé, [social.md](social.md), §6.1) : en entrant dans une pratique sociale (repas de famille, enterrement, mariage, réunion de travail, séance du conseil, file d'attente), une intuition donne les usages de la phase en cours (« À un enterrement, on présente ses condoléances à la famille. », « On ne quitte pas la table avant le dessert. », « Le maire donne la parole. ») ; une autre prévient avant un manquement évident (« Ce n'est pas le moment de rire. »). Le rôle du personnage dans la pratique est rappelé (« Vous présidez la réunion. »). Leur précision suit l'indice des intuitions, et un usage que le personnage ne connaît pas (un rite d'un autre milieu) n'est pas rappelé. *Pourquoi : un joueur ignore les usages d'une famille ou d'une institution que le personnage connaît par cœur ; sans rappel, il commettrait des manquements que le personnage ne ferait pas.*
- **Délégation** : un suppléant incarné reçoit les limites de son intérim en intuitions (« Ce n'est pas à vous de décider ça en son absence. ») ([social.md](social.md), §6.8).

### 4.2 Le carnet du personnage (décidé)

Un écran à onglets, ouvert d'une touche ; le monde continue de tourner pendant sa lecture (§9.3).

| Onglet | Contenu | Source |
|---|---|---|
| **Corps** | Santé, blessures par zone (silhouette), maladies, effets d'alcool ou de drogue, fatigue, besoins en mots | État du personnage |
| **Ce qui compte** | Les intuitions du moment (comme « Réfléchir ») et les engagements en cours : dettes, promesses, décisions lourdes en attente avec leur délai ; l'**humeur** en mots avec sa cause, et jusqu'à trois **thèmes de vie** (« Ce qui vous habite : le deuil de votre père »), à la précision des intuitions (décidé, IA §13.2, §13.4) | Intuitions, engagements, pensées, thèmes de vie |
| **Agenda** | Les **engagements fixes** connus du personnage : horaires de travail, rendez-vous, échéances de paiement, garde des enfants | Agenda du personnage |
| **Proches** | Ses connaissances sociales (§4.3) | Relations, savoirs |
| **Nouvelles** | Ce qu'il a appris récemment (§4.4) | Croyances |
| **Capacités** | Ses compétences notables et ses traits physiques et mentaux, en mots | Attributs, compétences |
| **Possessions** | Inventaire, clés, véhicules, logement, biens connus | Inventaire, registre |
| **Justice** | Ce que le personnage sait de sa situation judiciaire : **casier** (condamnations, date, peine), affaires en cours qu'il connaît (convocation, contrôle judiciaire), **interdictions** en cours (quartier, contact) avec leur date de fin, **délai d'épreuve**, caution bloquée, amendes restant dues | Casier, courrier reçu ([combat.md](combat.md), §8.7 et §9.5) |
| **Affaires** (enquêteurs) | Le dossier des affaires confiées (§5.6.9) | Croyances de l'enquêteur |

- **Agenda exact, envies approximatives** (décidé). L'agenda montre des faits que le personnage connaît comme dans un calendrier ; ses **priorités et ses sentiments** ne passent que par les intuitions, avec leur précision variable. *Pourquoi : savoir à quelle heure on embauche est indispensable pour jouer juste ; comprendre ce qui compte vraiment pour quelqu'un est la récompense de la synchronisation.*
- **Capacités en mots** (décidé), sur une échelle de cinq degrés (faible, moyen, bon, très bon, exceptionnel) **[seuils à calibrer]**, sans chiffre. Pour un personnage incarné, seules ses **trois à cinq compétences notables** sont listées, et leur précision suit l'indice des intuitions ; pour le personnage persistant, tous ses attributs sont listés **en crans et en mots, comme à sa création** (§3.2), avec une flèche de tendance (progression, désentraînement). *Pourquoi : des chiffres permettraient de recalculer le niveau personnel et le palier d'un personnage incarné ; le persistant est celui du joueur, qui doit pouvoir suivre sa progression.*
- Pendant une incarnation, le carnet révèle donc le **métier** du personnage (il le connaît). Tout ce qu'il contient **disparaît à la fin de l'incarnation** ([social.md](social.md), §2.3) ; seul le carnet du personnage persistant est durable.

### 4.3 Les proches (décidé)

- Une liste groupée : **famille**, **couple**, **amis**, **collègues**, **voisins**, **autres connaissances**, **ennemis**. Pour chacun : portrait (§4.7), nom, **lien** (« votre sœur », « votre chef »), **métier connu**, **dernier contact** (« vu il y a trois semaines »), adresse ou lieu de travail **s'il les connaît**.
- Une ligne d'intuition sur le sentiment du personnage (« Vous l'aimez beaucoup », « Vous vous méfiez de lui »), et les **faits** : dettes et promesses en cours, avec leur montant (ce sont des engagements que le personnage connaît exactement).
- **Secrets** : quand une intuition le suggère, la mention « Il y a quelque chose entre vous » ou l'étiquette opaque du savoir ; jamais le contenu ([social.md](social.md), §2.3).
- **Jamais** de valeur d'affection, de confiance, de respect ou de crainte ; **jamais** de position en direct.
- **Nouveau venu** ([population.md](population.md), §7.4) : la liste est presque vide (l'employeur, un parent en ville s'il y en a un) ; c'est voulu.

### 4.4 Les nouvelles et les rumeurs (décidé)

L'onglet « Nouvelles » liste les croyances récentes ou importantes du personnage (au plus 20 **[à calibrer]**), avec leur **origine** et leur **certitude en mots** :

| Origine | Formulation |
|---|---|
| Vu de ses yeux | « Vous avez vu Karim sortir de la bijouterie avec un sac. » |
| Source directe | « Lucie vous a dit qu'elle partait à l'extérieur jusqu'à lundi. » |
| Rumeur | « On dit que le patron du garage a des dettes de jeu (vous n'y croyez qu'à moitié). » |

*Pourquoi : les rumeurs sont au cœur du jeu social ([social.md](social.md), §4) ; le joueur doit savoir ce que son personnage croit, et à quel point.*

### 4.5 Les autres personnages à l'écran (décidé)

- **Pas d'étiquette flottante** au-dessus des têtes. Quand le joueur **regarde** quelqu'un (survol à la souris, ciblage à la manette), une infobulle donne ce que le personnage sait : le **nom** s'il le connaît, sinon une **description** (« une femme d'une soixantaine d'années, imperméable beige ») ; le **lien** (« votre voisin ») ; le **métier connu**. Option : afficher en permanence le nom des personnes connues (§11.3).
- L'**apparence** porte les indices du monde : uniforme de police, tenue de chantier, costume, tags d'une bande sur une veste. C'est à l'observateur de les lire.
- **Humeur et crises des autres** (décidé, IA §13.2) : l'humeur de chaque personnage se lit sur son corps : **posture** (épaules basses, buste droit), **démarche** (traînante, vive, raide), **expression** (fermée, souriante, crispée). L'infobulle la traduit en mots, d'après ce que le personnage incarné **devine** selon sa perception et son intelligence sociale (« l'air abattu », « tendu », « de bonne humeur »), jamais par une valeur. Une **crise** se voit comme telle (quelqu'un pleure sur un banc, crie contre un proche, s'en va marcher d'un pas raide) et laisse un souvenir au témoin. Ces signes viennent de l'humeur calculée par le cœur pour **tous** les personnages, y compris ceux pilotés par un joueur, que le joueur ne peut pas commander. *Pourquoi : la ville doit montrer des émotions lisibles, et un corps dont l'expression échapperait au pilote ne peut pas trahir qu'un joueur le conduit.*
- La **familiarité** du joueur n'apparaît **pas** dans le monde : seulement dans l'écran « Vos connaissances » (§3.4). *Pourquoi : afficher dans la rue le nom d'un inconnu pour le personnage incarné, parce que le joueur l'a connu dans une autre vie, mélangerait les deux couches ; le joueur reconnaît les visages comme on les reconnaît dans la vie.*

### 4.6 Le naturel des corps (décidé, IA §13.7, J§7)

Les habitants ont un **naturel** tiré par le cœur ([ia-pnj.md](ia-pnj.md), §13.7) ; l'interface le rend, et l'applique **de la même façon au personnage piloté par un joueur**.

| Naturel | Rendu | Pour un personnage piloté par un joueur |
|---|---|---|
| **Gestes au repos** | Regarder son téléphone, s'étirer, ajuster ses vêtements, observer les passants | Joués d'eux-mêmes après quelques secondes sans commande **[à calibrer]** ; interrompus par n'importe quelle commande |
| **Regard** | La tête se tourne vers un bruit, vers l'interlocuteur, vers ce qui bouge | Toujours automatique ; la cible désignée par le joueur attire aussi le regard |
| **Hésitations** | S'arrêter, regarder autour, revenir sur ses pas | Celles du joueur suffisent ; aucune n'est ajoutée à ses commandes |
| **Allure** | Démarche selon l'âge, l'humeur, la hâte et la météo | Décidée par le serveur, comme pour tous |
| **Délais de réaction** | Distribution humaine, selon l'attention, la fatigue et l'ivresse | Ceux du joueur, plus les délais physiologiques déjà appliqués par le serveur (§5.6.4) |
| **Imperfections** | Se tromper de rue, oublier ses clés, heurter quelqu'un et s'excuser | Celles qui touchent le corps (heurter, trébucher ivre) s'appliquent ; celles qui sont des **choix** (se tromper de rue) restent au joueur |

- Les **changements de direction** suivent une vitesse de rotation humaine : un demi-tour se fait par un pivot visible, jamais d'un coup ; le personnage **ne pivote pas sur place** sur commande, il ne se tourne qu'en se déplaçant, en regardant une cible ou en faisant face (§5.6.2). Tourner la caméra ne tourne pas le personnage.
- Ces animations sont **cosmétiques** : elles ne bloquent ni ne retardent jamais une commande.

*Pourquoi : on reconnaît une IA à sa perfection, et un joueur à ce qu'aucun habitant ne fait (tourner sur soi, rester figé, ne jamais regarder son téléphone) ; donner aux deux le même corps et le même naturel est la condition de la promesse qu'on ne distingue pas les pilotes ([ia-pnj.md](ia-pnj.md), §14.3).*

### 4.7 Les portraits (décidé, DA §3.5)

Les portraits (sélection de départ, « Vos connaissances », proches du carnet, bilan) sont **rendus en jeu** par le client, avec le même corps paramétrique et le même catalogue que dans le monde, sous une lumière neutre et un cadrage fixe. Aucune image n'est stockée ni envoyée. Un portrait ne montre que l'**apparence connue** de celui qui regarde, à sa **précision** :

| Précision connue (DA §3.5) | Portrait |
|---|---|
| **Silhouette** | Une silhouette en contre-jour, aux couleurs dominantes de la tenue |
| **Tenue** | La personne en buste, tenue, coiffure et signes distinctifs visibles, **visage dans l'ombre** |
| **Visage** | Le portrait complet |

- **Jamais de visage neutre dans un portrait** (décidé). Le visage neutre qui remplace un visage inconnu est invisible à l'échelle de la 2.5D (DA §3.5), mais il serait lisible en gros plan et passerait pour le vrai. *Pourquoi : un portrait doit montrer ce qu'on ne sait pas comme une absence, pas comme une fausse information.*
- **Quelle apparence** (décidé) :

| Écran | Apparence montrée |
|---|---|
| **Une nouvelle vie** (§3.3) | Avant le choix, l'apparence **actuelle** à la précision « **tenue** » seulement, **visage dans l'ombre** (décidé v0.9, renvoi de la sécurité à l'équilibrage) ; le visage n'apparaît qu'une fois le personnage incarné |
| **Vos connaissances** (§3.4) | L'apparence reconstituée à partir des **dates de perception identifiée** par le personnage incarné qui a donné la familiarité (ci-dessous) ; l'étape « À peine connu(e) » ne dépasse pas la précision « tenue » |
| **Proches** du carnet (§4.3) | L'apparence reconstituée à partir des **dates de perception identifiée** du personnage incarné (ci-dessous) |
| **Bilan** (§3.8) et **votre personnage** | L'apparence actuelle du personnage incarné ou persistant |

- **Trois dates par relation, une par précision** (décidé v0.9, revue d'architecture de clôture C-5 ; remplace la date unique de la v0.6) : la relation garde, à la journée, la date de la dernière perception **identifiée** à n'importe quelle précision, la dernière à « tenue » ou mieux et la dernière à « visage ». Le serveur reconstitue **chaque groupe de champs à la date de sa précision**, à partir du génome, dont le vieillissement est déterministe (DA §3.2), et de l'historique daté de la coiffure, de la tenue habituelle et des signes distinctifs (§15.5) :

| Groupe de champs | Reconstitué à la date de la dernière perception… |
|---|---|
| Silhouette (taille, corpulence, posture) | identifiée, à n'importe quelle précision |
| Tenue, coiffure, signes distinctifs visibles | identifiée à « tenue » ou mieux |
| Visage, peau | identifiée à « visage » |

  *Pourquoi : avec une seule date, une silhouette aperçue hier ferait apparaître une coupe ou un visage vieilli que le personnage n'a jamais vus ; aucune image n'est stockée, quelques octets par relation suffisent.*
- **Seule une identification met à jour un portrait** (décidé v0.9, revue de sécurité de clôture C1) : la relation n'est touchée que lorsque le personnage **identifie** la personne, par le test d'identification du cœur (visage découvert, ou silhouette, voix ou démarche d'un proche selon la familiarité). Une perception sans identification ne change **aucun** portrait, aucune date, aucun écran : elle peut seulement créer un « inconnu vu », jamais rattaché à la personne sans identification ultérieure. Le portrait ne montre **jamais un état vu pendant que la personne était masquée** et non identifiée. *Pourquoi : un portrait qui changerait après qu'on a croisé une silhouette ou un cambrioleur cagoulé apprendrait au joueur qui c'était, alors que son personnage l'ignore.*
- **Calcul à l'ouverture de l'écran, sans notification** (décidé v0.9, sécurité C1) : le serveur ne recalcule les portraits qu'à l'ouverture d'un écran qui les montre ; aucune perception ne déclenche de message, de pastille ou de changement visible ailleurs. Test attendu : la vue, les écrans et le journal de conversation sont **identiques**, que la silhouette aperçue soit la personne ou un tiers, tant que l'identification échoue.
- Le client ne garde **aucun portrait** d'une incarnation à l'autre ; il les recalcule à partir de ce que le serveur envoie pour l'écran ouvert (S6).
- Un personnage connu **seulement masqué** (cagoule, casque) a un portrait de précision « tenue » au mieux, avec le masque, et seulement s'il a été **identifié** ainsi (par sa voix ou sa démarche, par exemple).

---

## 5. Contrôles

### 5.1 Principes (décidé)

- Chaque commande produit une **intention** (aller vers, interagir avec telle entité par telle action, dire tel acte), exactement comme l'IA ([05-architecture.md](../05-architecture.md), §2.3). Le résultat dépend des attributs du personnage, résolus par le serveur.
- **Clavier-souris** et **manette** sont complets et tous deux **entièrement réassignables**. La disposition AZERTY ou QWERTY est détectée.
- Les actions longues se **maintiennent** ou se **basculent** au choix (§11.1).
- **Aucun geste gratuit** (décidé, J§7) : les commandes n'offrent ni **saut**, ni **geste libre** (salut, danse, émote), ni pivot sur place, en dehors des actions de contexte. Franchir un muret, enjamber une barrière, escalader une clôture, saluer quelqu'un ou s'asseoir sont des **actions contextuelles** proposées par le serveur sur une cible (§5.2), comme pour un habitant. *Pourquoi : le jeu promet qu'on ne distingue pas les pilotes ; la surface de jeu doit donc être la même pour un joueur et pour un habitant, et un geste qu'aucun habitant ne fait désignerait un joueur ([ia-pnj.md](ia-pnj.md), §13.7 et §14.3).*

### 5.2 Déplacement et interaction contextuelle (décidé)

- **Déplacement** : direction relative à la caméra (touches ou stick gauche). Option clavier-souris : **cliquer pour aller** (le personnage suit un chemin calculé par le serveur). Marche, **course** (maintenue, limitée par l'endurance), **marche lente et discrète** (bascule). S'accroupir n'est plus une commande libre : c'est une action contextuelle (se cacher derrière un obstacle, regarder sous une voiture, ramasser, s'occuper d'un enfant) (décidé, J§7).
- **Courir en public** (décidé, J§7) : la course reste toujours disponible, mais courir **sans raison apparente** en public (ni retard, ni danger, ni transport à attraper, ni tenue de sport) a une **signature** : légère dissonance pour un personnage calme, qui se lit dans la zone hachurée (§2.3), et une fatigue qui s'affiche comme toute fatigue d'effort. Une intuition le dit la première fois dans une incarnation, au niveau clair (« Les gens vous regardent courir. »). Les passants le perçoivent. Courir pour une raison (bus qui part, retard, menace) n'a pas de signature. *Pourquoi : un joueur qui court partout se reconnaît ; la règle doit être comprise, sans interdire la course.*
- **Marche discrète** : elle est perçue par les témoins comme telle (« se faufile ») ; c'est de l'apparence (§4.5).
- **Cible** : l'entité sous le curseur, ou à la manette l'entité la plus pertinente dans la direction du regard, changée au stick droit. La cible est soulignée d'un contour discret.
- **Action principale** : une touche (E, ou A à la manette) exécute l'action par défaut proposée par le serveur pour cette cible (ouvrir, parler, s'asseoir, prendre).
- **Menu des actions** : en **maintenant** la même touche (ou clic droit), un menu circulaire présente **toutes les actions possibles** sur la cible, envoyées par le serveur, avec leur signe de réticence (§2.3). Les actions fondées sur un savoir opaque portent leur étiquette (« Ouvrir le coffre (vous connaissez le code) »).
- **Fouiller** (décidé, J§7) : fouiller un tiroir, une armoire, une poubelle, un sac, un véhicule ou une personne est une **action** du menu, une cible à la fois, avec une barre de durée ; il n'existe ni fouille globale d'une pièce, ni ramassage automatique. C'est un **acte** qui a une **signature** (fouiller chez autrui, dans un commerce ou les affaires d'un proche n'a pas le même poids que dans son propre tiroir) et porte le signe de réticence selon le profil ; les **témoins** le voient et réagissent (regard, question, appel à la police), comme pour un habitant. *Pourquoi : tout fouiller est un réflexe de joueur de jeu vidéo qu'aucun habitant n'a ; en faire un acte du monde lui donne son vrai coût.*
- **Actes excentriques** (décidé, revue [signatures-contenu](../revues/signatures-contenu.md) §5.2, A20 à A24 et A26 à A28 ; arbitrage A7) : certains actes ordinaires de la surface de jeu ont un sens social sans effet mécanique. Ils restent des **actions de contexte** ou des **déplacements**, et portent leur **signe de réticence** (§2.3) comme tout acte. **Aucun geste libre n'est créé pour eux** : la règle « aucun geste gratuit » (§5.1) prime, et les actes sans objet en première version (danser seul, s'allonger sur la chaussée, uriner en public : A18, A19, A25) n'existent pas.

| Acte | Comment le joueur le fait | Où s'affiche le signe |
|---|---|---|
| **Dévisager** un inconnu (A20) | Action « Dévisager » du menu sur une personne | Dans le menu |
| **Suivre** un inconnu (A21) | Action « Suivre » du menu, ou par le déplacement | Dans le menu ; pour un suivi à la main, dans une intuition dès que le serveur le reconnaît (« Vous le suivez depuis un moment. ») |
| **S'asseoir à la table d'inconnus** (A22) | Action « S'asseoir » sur une chaise d'une table occupée | Dans le menu |
| **Entrer chez autrui** par une porte ouverte (A23) | Action « Entrer » sur la porte, ou en franchissant le seuil | Sur la porte désignée, et au contour du seuil quand le personnage s'en approche, avant de le franchir |
| **Fouiller une poubelle** (A24) | Action « Fouiller » (ci-dessus) | Dans le menu |
| **Jeter un déchet par terre** (A26) | Action « Jeter par terre » sur un objet tenu, à côté de « Jeter à la poubelle » quand il y en a une à portée | Dans le menu |
| **Frapper aux portes au hasard** (A27) | Action « Frapper » sur une porte, sans raison connue du personnage | Dans le menu |
| **Manger dans l'assiette d'un inconnu** (A28) | Action « Prendre » ou « Manger » sur une assiette servie à autrui | Dans le menu |

  - **Le regard ne dévisage pas** (décidé) : désigner une personne attire seulement un **coup d'œil** (§4.6), qui revient de lui-même au bout de quelques secondes **[à calibrer]** ; seul l'acte « Dévisager » soutient le regard. *Pourquoi : un joueur qui garde quelqu'un en cible pour lire son infobulle ne doit pas commettre un acte à son insu.*
  - **Un acte fait par le déplacement est annoncé avant** (décidé) : le signe apparaît sur le seuil ou dans l'intuition **avant** que l'acte ne compte, et l'acte ne compte qu'une fois le seuil franchi ou le suivi reconnu **[durée à calibrer]**. *Pourquoi : le signe de réticence n'a de sens que s'il permet de renoncer.*
  - *Pourquoi ces actes passent par des actions et non des gestes : ce sont les actes excentriques que la surface de jeu permet ; leur donner un signe enseigne le personnage sans rien ajouter qu'un habitant ne fasse.*
- **Tâches de travail** : au travail, un petit encart liste ce que le personnage **perçoit** à faire (« 2 clients attendent », « Livraison à ranger ») ; chaque tâche est aussi une action contextuelle sur les objets et les personnes concernés ([economie.md](economie.md), §3.3). Les tâches de fond restent accessibles quand rien ne presse.
- **Procédures avec délai** : avant une décision lourde (vendre, démissionner, gros virement), un panneau rappelle la procédure et son délai (« La vente sera signée chez le notaire dans 10 jours ») et demande une confirmation ([economie.md](economie.md), §12.2 ; [social.md](social.md), §8.4).

### 5.3 Gestes automatiques (décidé)

- Avant chaque geste automatique ([ia-pnj.md](ia-pnj.md), §5.2), une icône annonce le geste (« Boire un verre d'eau ») pendant 2 secondes **[à calibrer]**, avec la touche **Annuler** ; le geste peut aussi être interrompu en cours.
- Un réglage global : **Gestes automatiques : activés / me demander / désactivés**. Désactivés, les besoins triviaux apparaissent dans le HUD et se gèrent à la main.

*Pourquoi : l'IA permet au joueur de s'en empêcher ; une annonce brève et un réglage global le permettent sans le déranger.*

### 5.3 bis Tenue, coiffure et radio (décidé, [ia-pnj.md](ia-pnj.md) v1.1, §2.6 et §11)

Pour le personnage qu'il pilote, incarné ou persistant, le joueur fait les choix que l'IA fait pour les habitants. Ce sont des **actions du monde**, faites là où se trouvent les choses, jamais des menus hors du monde.

**Tenue** :

- **Se changer** est une action de contexte sur l'armoire ou la commode du logement, le vestiaire du poste ou un sac emporté (décidé). *Pourquoi : la garde-robe est faite d'objets ; se changer dans la rue d'un clic serait un geste qu'aucun habitant ne fait.*
- Un panneau présente la **garde-robe** du personnage par **usage** (quotidien, travail, sortie, sport, intempéries, nuit), avec l'**état** de chaque pièce en mots (propre, froissé, sale, taché, mouillé, déchiré). Le joueur prend une tenue toute faite ou change une pièce à la fois, dans les règles de montage (DA §3.3). Le changement dure le temps de la routine **[à calibrer]**, avec son geste.
- Au début d'une incarnation, le personnage porte la tenue que l'IA avait choisie ; **pendant l'incarnation, l'IA ne la change jamais**, même mouillée ou sale, et aucun geste automatique ne s'en charge.
- Les **intuitions** guident le choix, à leur précision (§4.1) : « Il va pleuvoir, d'après la radio. », « Pour l'enterrement, il mettrait sa tenue sombre. », « Votre chemise est tachée. ». Une tenue mal choisie fait monter l'inconfort (froid, pluie) et se lit dans ce que les autres perçoivent ([social.md](social.md), §5.5) ; elle n'a pas de signe de réticence.
- Le **parapluie** est un accessoire de la garde-robe, pris par l'action « Prendre le parapluie » à la porte du logement.

**Coiffure** :

- Au salon, quand le coiffeur s'occupe du personnage, un panneau propose les **coupes** du catalogue compatibles avec la nature et la longueur de ses cheveux, avec leur prix ; **« Comme d'habitude »** est en tête et choisi par défaut. Une coupe ne peut que raccourcir. *Pourquoi : la coupe fait partie de l'état courant (DA §3.2) ; la reprendre à l'identique est le choix de la plupart des gens, et le plus discret.*
- La coupe faite à la maison par soi ou par un proche propose les seules coupes simples **[liste à calibrer]**.
- Une **intuition** signale quand la coupe est trop longue pour les habitudes du personnage (« Il devrait passer chez le coiffeur. »), comme l'objectif que l'IA aurait.

**Radio** :

- **Allumer la radio** est une action de contexte sur un poste, une chaîne ou l'autoradio d'un véhicule ; ensuite **Changer de station** passe d'une station à l'autre (trois au départ, DA §5.4), et **Volume** se règle en trois crans (bas, moyen, fort). Ce que l'on entend suit le programme commun calculé par le client, le même pour tous les auditeurs.
- Une radio réglée **fort** gêne les voisins comme celle d'un habitant (inconfort, plainte possible, [ia-pnj.md](ia-pnj.md), §2.6). Une station loin des goûts du personnage peut lui donner une petite pensée, dite par une intuition à partir de la précision « claire » (« Cette musique l'agace. ») ; elle n'a pas de signe de réticence.
- Quand une source musicale du monde est perçue, la **musique d'accompagnement** (§11.3) se tait (décidé). *Pourquoi : deux musiques superposées seraient une gêne, et celle du monde a priorité.*

### 5.4 Conversation : roue d'actes et texte libre (décidé)

**Ouverture** : action « Parler » sur une personne, ou réponse quand quelqu'un s'adresse au personnage. Le monde **ne s'arrête pas** (§9.3).

**La roue d'actes** ([social.md](social.md), §3.3) :

1. **Premier anneau** : les familles d'actes (lien, demande, offre, information, réponse, influence, conflit, réparation, statut), et au centre le **texte libre**.
2. **Second anneau** : les actes de la famille que le personnage **croit possibles** dans la situation (§1.4). Les actes impossibles n'apparaissent pas, sauf quand une intuition doit expliquer l'impossibilité (§4.1).
3. **Sujet**, si l'acte en demande un : une liste de ce dont le personnage peut parler (personnes, lieux, événements, affaires, objets qu'il connaît), les plus pertinents d'abord ; les savoirs opaques y figurent par leur étiquette.
4. **Ton** : choisi à tout moment, au clavier (touches 1 à 5) ou aux gâchettes ; affiché par une **icône, un mot et une couleur** (amical, neutre, sec, menaçant, séducteur).
5. **Registre** (décidé, [social.md](social.md), §3.3) : **poli** ou **familier**, choisi à côté du ton par une bascule (touche 6 ou croix gauche **[à calibrer]**) et affiché par un mot (« vous » ou « tu ») sous l'icône du ton. Par défaut, la roue propose le registre **attendu** dans la relation, tel que le personnage le connaît (familier avec un intime, poli avec un inconnu ou un supérieur), nuancé par sa voix ; le joueur peut le changer. Le ton dit l'intention, le registre la distance. *Pourquoi : tutoyer un inconnu ou vouvoyer un intime a un effet dans le monde ; le joueur doit pouvoir le choisir, et le défaut évite qu'il ait à y penser à chaque phrase.*

**Ce qu'il dirait** : en haut de la roue, jusqu'à trois **suggestions** tirées des actes que l'IA du personnage choisirait elle-même, d'autant plus fiables que l'indice de précision des intuitions est élevé (§4.1) ; en dessous du niveau « clair », aucune suggestion. *Pourquoi : c'est l'équivalent conversationnel des intuitions, et une récompense directe de la synchronisation.*

**Le texte libre** :

1. Le joueur tape sa phrase, de **280 caractères** au plus **[à calibrer]** ; un compteur apparaît à l'approche de la limite (décidé, S18, A4). Le service de dialogue la **classe** en acte, sujet et ton ([social.md](social.md), §3.3), **parmi les seules actions que le personnage croit possibles** (§1.4).
2. Une **carte de vérification** s'affiche : « Demander un service — à propos de : Karim — ton amical — registre familier », avec le signe de réticence éventuel et, si le personnage ne sait pas ce qu'il affirme, la mention « Vous le dites sans le savoir : ce sera pris pour une supposition. » Si la phrase correspond à un acte retiré par la garde hors fiction, la carte affiche l'intuition du §4.1 et ne propose pas l'envoi.
3. **Envoyer**, **Modifier**, ou **Choisir à la roue**. Une option « envoi direct » (sans carte) existe pour les joueurs habitués, désactivée par défaut.
   - **Registre** (décidé, IA §13.10, [social.md](social.md), §3.3) : le classement rend aussi le **registre** de la phrase, **poli** ou **familier**, affiché sur la carte à côté du ton ; les nuances sèche, agressive ou séductrice relèvent du **ton**. Le joueur **confirme** les deux avec l'acte, ou les corrige d'un clic. L'auditeur réagit au registre comme à une partie de l'acte, et le registre entre dans le journal avec l'intention. Avec l'envoi direct, ton et registre classés sont envoyés sans confirmation. *Pourquoi : « Tu me files un coup de main ? » et « Pourriez-vous m'aider ? » ne font pas le même effet ; le joueur doit voir comment sa phrase sera reçue avant de l'envoyer. Cette version remplace celle de la v0.4, qui comptait cinq registres, pour suivre social §3.3.*
4. **Texte refusé** (décidé, S18) : un texte dont le classement est refusé (contenu interdit, phrase incompréhensible) **n'est diffusé à personne**. La carte l'indique en couche joueur (« Ce texte ne peut pas être dit. ») avec **Modifier** et **Choisir à la roue**. *Pourquoi : un texte qu'on ne sait pas classer ne peut ni compter pour la synchronisation, ni être modéré avec certitude ; le diffuser quand même ouvrirait un canal hors des règles.*
5. **Filtre hors fiction** (décidé, E26, A4) : dans toute parole libre, quel que soit l'auditeur, les mots du **vocabulaire hors fiction** (joueur, PNJ, IA, serveur, latence, pseudonyme, noms de plateformes **[liste à constituer]**) sont remplacés par une hésitation (« … ») et les **émoticônes** sont retirées. La carte de vérification montre ces remplacements, puisqu'ils ne dépendent pas de l'auditeur. *Pourquoi : ces mots et ces signes trahissent un joueur derrière un personnage ; le joueur doit savoir que sa phrase sera amputée.*
6. **Ce que l'orateur voit de sa propre réplique : toujours son texte tel qu'il l'a tapé**, aux remplacements du filtre hors fiction près (décidé). La carte de vérification ne montre jamais la reformulation, ni la traduction, ni le masquage des données personnelles (qui ne s'applique qu'entre joueurs non amis, [social.md](social.md), §9.3). *Pourquoi : la parole est reformulée si l'auditeur est piloté par l'IA, et transmise telle quelle ou traduite s'il est piloté par un joueur ([social.md](social.md), §9.2) ; montrer à l'orateur ce qui dépend de l'auditeur lui révélerait le pilote de son interlocuteur.*
7. **Joueur réduit au silence** (sanction) : le centre de la roue indique « Parole libre indisponible » (§10.4).

**Les répliques** :

- Une **bulle** au-dessus de l'orateur, et un **journal de conversation** à côté de la roue. Le volume se lit à la forme de la bulle, le ton à l'icône, au mot et au geste (§2.8).
- En attendant la formulation (une à deux secondes), l'orateur a un geste de réflexion et sa bulle montre « … » ([social.md](social.md), §3.4).
- **Même message, même délai** (décidé, A4) : la parole d'un personnage piloté par un joueur arrive chez les auditeurs **exactement comme une réplique de PNJ** : même geste de réflexion, même bulle « … », puis le texte, après un délai tiré dans la même distribution que les délais de formulation **[à calibrer]**. Seul l'orateur voit sa phrase aussitôt, dans son journal. *Pourquoi : une parole qui arriverait instantanément désignerait un joueur.*
- **Traduction** (décidé, A6) : quand l'auditeur lit une autre langue de dialogue que l'orateur, la parole libre d'un joueur lui parvient **traduite** fidèlement par le service de dialogue (traduite, pas reformulée), dans le même délai. *Pourquoi : une phrase restée dans une langue étrangère désignerait un joueur, puisque les PNJ parlent toujours la langue de l'auditeur.*
- **Conversations entendues** à portée d'écoute : bulles atténuées, selon les trois niveaux d'écoute du §2.8 (compris, bribes, entendu sans être compris) ([social.md](social.md), §3.2). Les **chuchotements** ne s'affichent qu'à très courte distance, en petits caractères. Au-delà de la portée d'écoute, on ne voit que les gestes et la posture (rires, éclats de voix).
- **Mode sous-titres** : toutes les répliques en bas de l'écran, préfixées du nom (ou de la description) de l'orateur (§11.1).
- **Aucune parole dite** en première version (décidé, DA §5.5) : les répliques sont du texte ; chaque réplique s'accompagne d'une courte **vocalisation** non verbale (rire, soupir, « hm », exclamation) à la voix du personnage, jouée au moment où la bulle affiche le texte, pour un joueur comme pour un PNJ. La vocalisation de l'orateur piloté par un joueur est choisie par le serveur d'après le ton de l'acte, jamais par le joueur.
- **Plus tard, voix synthétisées** (option, DA §5.5) : toutes les paroles sont dites, celles des PNJ comme celles des joueurs (texte libre traduit compris), dans le client de chaque auditeur, avec la voix du personnage et **au moment où la bulle affiche le texte**, donc avec le même délai ; l'orateur entend sa propre phrase au même moment que ses auditeurs (décidé). *Pourquoi : une voix qui partirait dès la frappe, ou qui ne serait donnée qu'aux PNJ, désignerait le pilote.*

**Négociation** ([social.md](social.md), §3.7) : un panneau s'ajoute à la roue, avec l'**offre de chacun**, un champ de montant et les actes propres (proposer, contre-proposer, argumenter, menacer de partir, céder, conclure, rompre), plus l'intuition sur la limite de l'autre. La limite elle-même n'est jamais affichée.

**Actes intimes** (décidé, A6) : l'acte « passer la nuit ensemble » suit **le même parcours** quel que soit le pilote de chacun ([social.md](social.md), §1.6 et §9.4).

- Le personnage **qui propose** attend la réponse dans le monde, avec le même délai, que l'autre soit un PNJ ou un joueur ; la réponse est un acte du monde (accepter, décliner poliment), formulé de la même façon dans les deux cas. Le délai de réponse d'un PNJ est tiré dans une fourchette qui couvre celle d'un joueur (de quelques secondes à une minute **[à calibrer]**) ; une demande restée sans réponse au bout de ce délai vaut refus.
- Un personnage **piloté par un joueur** qui reçoit la proposition, qu'elle vienne d'un PNJ ou d'un joueur, voit une demande d'**acceptation explicite** (couche joueur) : **Accepter** ou **Décliner**. Décliner produit le même refus poli que celui d'un PNJ, sans autre effet.
- L'acte accepté reste un **fondu**.

*Pourquoi : une demande d'acceptation qui n'apparaîtrait qu'entre joueurs révélerait le pilote ; le consentement du joueur reste exigé chaque fois qu'il est concerné, d'où que vienne la proposition.*

### 5.5 Conduite et transports (décidé)

- **Monter** dans un véhicule est une action contextuelle (avec une clé, ou par un autre moyen fourni par le domaine combat). **Conduire** : accélérer, freiner, diriger (touches ou gâchettes et stick), frein à main, klaxon, phares, clignotants.
- **Caméra en conduite** (2.5D) : elle s'éloigne avec la vitesse et se décale dans le sens de la marche pour laisser voir la route **[à calibrer]**.
- **Passager** : le joueur peut parler, regarder, descendre à l'arrêt ; il ne conduit pas.
- **Transports en commun** : valider son titre en montant (action contextuelle, ou achat à l'automate, au chauffeur, dans l'application) ; à bord, un encart affiche le **prochain arrêt** ; l'action « Descendre à… » choisit l'arrêt, et le personnage se lève tout seul. En solo, « Avancer jusqu'à l'arrêt » fait avancer le temps (§9.1).

### 5.6 Combat, blessures, effractions, police et justice (décidé)

Le contenu vient de [combat.md](combat.md) ; l'interface en fixe la **saisie** et l'**affichage**, avec les principes de ce domaine : temps réel sans pause, aucun chiffre, aucune chance de réussite affichée ([combat.md](combat.md), §0 et §1.2).

#### 5.6.1 L'escalade à l'écran

Un affrontement commence presque toujours par des mots ([combat.md](combat.md), §1.1). Tant qu'il n'y a pas de coup, tout passe par la **roue d'actes** (§5.4) : reprocher, provoquer, insulter, **menacer**, et pour désamorcer s'excuser, céder, payer. La roue reste ouverte pendant la tension ; les actes de **désescalade** (reculer, lever les mains, partir) y ont un raccourci direct. La **barre de combat** (§5.6.2) n'apparaît que quand le personnage se met **en garde** ou qu'il est attaqué. *Pourquoi : la plupart des conflits se règlent par des mots ; une interface qui ouvrirait le combat dès la première insulte pousserait à la violence.*

#### 5.6.2 La saisie des intentions de combat

Les intentions sont celles de [combat.md](combat.md), §1.7. Leur saisie :

| Famille | Intentions | Clavier-souris | Manette |
|---|---|---|---|
| **Posture** | Se mettre en garde, reculer, garder ses distances, se protéger, se recroqueviller | Bascule « En garde » (G) ; reculer = se déplacer en garde ; se protéger = maintenir Parer sans adversaire proche ; se recroqueviller = maintenir Parer au sol | L1 (bascule) ; stick gauche ; L2 maintenu |
| **Attaque** | Frapper haut, frapper bas, pousser, saisir, immobiliser, frapper avec l'objet ou l'arme blanche en main, lancer, tirer | Clic gauche = frapper (haut si la souris est au-dessus de la ligne d'épaule de la cible en 2.5D, bas sinon) ; F = pousser ; clic du milieu = saisir, puis maintenu = immobiliser ; avec un objet en main, clic gauche le porte ; viser puis clic gauche = lancer l'objet | X (haut) / Y (bas) ; B = pousser ; R1 = saisir, maintenu = immobiliser ; R2 lance ou tire selon ce qui est en main |
| **Défense** | Parer, esquiver, se dégager, désarmer, s'interposer | Clic droit = parer ; Espace en garde = esquiver dans la direction du déplacement ; marteler Espace saisi = se dégager ; désarmer et s'interposer = menu des actions sur la cible | L2 = parer ; A en garde = esquiver ; marteler A = se dégager |
| **Arme** | Dégainer, ranger, viser, recharger, poser à terre, jeter | 1 à 4 = choisir et dégainer ce qu'on porte ; même touche = ranger ; clic droit maintenu = viser (avec une arme à feu ou de lancer) ; R = recharger ; poser et jeter = menu des actions | Croix gauche (cycle) ; L2 = viser ; X maintenu = recharger |
| **Issue** | Fuir, se rendre, appeler à l'aide, crier, menacer, ordonner de s'arrêter | Fuir = courir ; **H maintenu = se rendre (lever les mains)** ; appeler à l'aide, crier, menacer, ordonner = raccourcis de la roue (V) | Course ; croix haut maintenue = se rendre ; roue sur croix bas |
| **Après** | Porter secours, maintenir au sol, fouiller, ramasser, s'enfuir avec un objet | Menu des actions sur la personne ou l'objet | Menu des actions |

- **Disposition de combat** : les touches de combat ne sont actives qu'**en garde** (ou quand le personnage est attaqué) ; elles reprennent alors des touches qui servent à autre chose hors combat (R recharge au lieu de « Réfléchir », 1 à 4 choisissent une arme au lieu du ton, sauf si la roue est ouverte). La disposition définitive se fixe au prototype (§5.7). *Pourquoi : le combat est rare ; lui réserver des touches permanentes encombrerait la disposition de tous les jours.*
- **Faire face, sans verrouillage** : la touche de ciblage (Tab, clic du stick droit) **tourne le personnage vers un adversaire** et le garde face à lui ; elle ne guide pas les coups. Un coup part dans la direction où regarde le personnage et peut manquer, ou toucher un tiers sur sa trajectoire. *Pourquoi : le combat refuse le « verrouillage magique » ([combat.md](combat.md), §1.2) ; faire face suffit à rendre le combat jouable à la manette.*
- **Gestes impossibles** : il n'existe ni geste d'étranglement, ni acte de violence prolongée, ni geste violent d'un adulte envers un mineur ([combat.md](combat.md), §1.3, §12). Face à un **mineur**, la barre de combat ne montre que les gestes de contrôle (repousser, se dégager, retenir par le bras), et aucune arme ne peut le viser : le réticule ne le désigne pas.
- **Bords dangereux** : le déplacement s'arrête au bord d'un toit, d'un quai ou d'un pont comme devant un mur ([combat.md](combat.md), §11.9) ; aucune indication particulière n'est nécessaire.
- Aucune **confirmation** avant un acte violent, même grave : seul le signe de réticence (§2.3) prévient, et il s'alourdit à chaque marche de l'escalade ([combat.md](combat.md), §11.2). Au **point de rupture** (E1), le signe s'affiche sur l'intention d'attaque dès que le personnage fait face à la cible, toujours visible ; si le joueur frappe quand même, le personnage retient son geste et le joueur est éjecté (§2.3). *Pourquoi : le temps réel du combat ne supporte pas de fenêtre ; la synchronisation fait déjà payer l'acte contraire.*

#### 5.6.3 Visée et armes à feu

- **Viser** : en maintenant la touche de visée, un **réticule** apparaît sur le point du monde désigné (souris sur le sol en 2.5D, stick droit à la manette). Sa **taille** et son **tremblement** sont ceux que le **serveur** indique, d'après la coordination, la compétence, la distance, l'éclairage, la blessure du bras et surtout le **stress** ([combat.md](combat.md), §1.5) ; le réticule se resserre avec le temps de visée. Le client n'en calcule rien. Aucun pourcentage de réussite n'est affiché.
- **Aide à la visée** (manette, désactivable) : elle aimante seulement le **réticule** vers la silhouette la plus proche de la direction visée, sans rien changer à la dispersion ni à la trajectoire, résolues par le serveur.
- **Munitions** : l'état du chargeur s'affiche **en mots** près de l'arme en main (plein, entamé, presque vide, vide), plus le nombre de **chargeurs de rechange**, qui se comptent. *Pourquoi : on ne compte pas ses balles sous le stress ; c'est cohérent avec « aucun chiffre » et rend le rechargement réaliste.*
- **Braquer** une arme sur quelqu'un (viser une personne sans tirer) est l'acte de menace le plus fort ; il apparaît comme tel dans la signature, et la cible réagit en conséquence.
- **Coup de feu entendu** : icône de son large et imprécise (§6.4), qui déclenche chez le personnage incarné les effets physiologiques (§5.6.4).

#### 5.6.4 Effets physiologiques, sans chiffre

Le joueur garde la main pendant le combat, mais il **subit** les effets du réflexe et de l'état du personnage ([combat.md](combat.md), §1.6 et §1.8). Ils sont rendus par l'image, le son et la manette, jamais par une valeur :

| Effet | Rendu |
|---|---|
| **Sursaut** (menace soudaine, coup de feu proche) | Secousse brève de la caméra, son étouffé une fraction de seconde ; le délai des commandes est appliqué par le serveur |
| **Tremblements** (stress élevé, froid) | Réticule qui tremble, gestes fins plus lents (barre des actions délicates qui avance par à-coups) |
| **Vision en tunnel** (stress très élevé) | Bords de l'image assombris et flous ; le serveur réduit réellement ce que le personnage perçoit sur les côtés, qui disparaît donc de l'écran (§6.4) |
| **Essoufflement** (fatigue d'effort) | Respiration audible qui s'accélère, mouvements alourdis, coups plus lents à l'animation ; au plus fort, la course n'est plus possible |
| **Adrénaline** | Les icônes de douleur sont atténuées pendant l'affrontement ; à la fin, elles reviennent d'un coup avec un bref voile rouge sombre (« La douleur revient. ») |
| **Ivresse, drogues** | Léger balancement de l'image, réactions décalées, vision un peu floue |
| **Étourdissement** | Image floue et sons lointains pendant quelques secondes, commandes retirées |
| **Perte de connaissance** | Écran noir, sons étouffés ; message de la couche joueur « Votre personnage a perdu connaissance ». Le joueur peut attendre le réveil ou **rendre** le personnage ; s'il est en état critique, il ne peut plus que le rendre |
| **Choc** (santé très basse) | Image désaturée, pulsation lente ; la course est impossible ([combat.md](combat.md), §2.2) |
| **Douleur d'une zone** | Quand le joueur sollicite un membre blessé (viser avec un bras touché, courir sur une jambe blessée), une brève grimace, un son et une vibration |

- Chacun de ces effets a une **intensité réduite** dans les réglages d'accessibilité (secousses, flous, flashs, §11.1), sans changer la simulation.
- Un **danger perçu** (vu ou entendu) s'affiche par un **indicateur de direction** au bord de l'écran ; ce qui n'est pas perçu ne s'affiche pas.

#### 5.6.5 Blessures visibles et ressenties

Les blessures de [combat.md](combat.md), §2.1 :

- **Silhouette** du corps en huit zones (tête, cou, torse, abdomen, bras gauche, bras droit, jambe gauche, jambe droite), dans le HUD sous les barres de vie dès qu'une blessure existe, et en grand dans l'onglet « Corps » du carnet (§4.2).
- Pour chaque blessure, en mots : **type** (contusion, coupure, plaie, fracture, plaie par balle, brûlure, commotion…), **gravité** (légère, moyenne, grave, critique, avec un motif de remplissage et une couleur), **saignement** (aucune, une, deux ou trois gouttes : faible, fort, massif), **douleur**, et **soins reçus** (aucun, premiers secours, soins médicaux, opéré). Le personnage connaît ses blessures ; le **joueur ne voit pas leur atteinte vitale**, seulement les deux barres.
- **Un saignement fort ou massif** est signalé par une intuition pressante (« Vous saignez beaucoup : il faut comprimer la plaie ou appeler les secours. »), puisqu'il fait baisser la santé minute après minute.
- **Visibles sur le corps** de tous les personnages : boiterie, bras en écharpe, pansements, tache sombre, ecchymoses ([combat.md](combat.md), §2.1). Un témoin qui regarde un blessé lit dans l'infobulle ce qu'il **voit** (« saigne fortement au ventre ») et jamais son état interne.
- **Sobriété** : sang en petite quantité, pas de plaie détaillée, corps recouvert d'un drap à l'arrivée des secours ([combat.md](combat.md), §12.3). Le réglage **« Atténuer la violence »** (§11.3) retire le sang et adoucit les sons des coups et des tirs ; il ne change rien à la simulation.

#### 5.6.6 Secours et appels d'urgence

- **Premiers secours** sur un blessé (action contextuelle) : comprimer la plaie, faire un garrot, mettre en position latérale, rester auprès ; une barre de durée, puis l'effet visible (le saignement ralentit) ([combat.md](combat.md), §2.3).
- **Urgences** : le téléphone a un bouton **Urgences**, utilisable même verrouillé, qui appelle la **police** ou les **secours**. L'appel se fait par la roue d'actes, avec des actes propres : dire ce qui se passe (violence, arme, blessé, incendie, cambriolage), où, et s'il y a des blessés ; l'adresse du lieu où se trouve le personnage est proposée d'office s'il la connaît ([combat.md](combat.md), §5.4). Aucun délai d'arrivée n'est annoncé, sinon ce que dit l'opérateur (« Une équipe arrive »).
- **Filmer** avec son téléphone est une action contextuelle ; la vidéo devient un objet du téléphone, qu'on peut remettre à la police.
- Les **témoins** n'ont aucun indicateur : le joueur ne sait pas qui l'a vu, sauf ce que son personnage perçoit (regards, cris, téléphone levé).

#### 5.6.7 Effractions et véhicules

Méthodes de [combat.md](combat.md), §3.1 et §3.3, offertes dans le menu des actions d'une porte, d'une fenêtre, d'un coffre ou d'un véhicule, **selon ce que porte et sait faire** le personnage :

| Action | Condition d'affichage | À l'écran |
|---|---|---|
| Ouvrir avec la clé | Le personnage a la clé | Instantané |
| **Crocheter** | Outils de crochetage en poche | Barre de durée (30 s à 10 min) sans chance de réussite ; s'interrompt si le personnage est dérangé ; échec annoncé en mots (« La serrure résiste. ») |
| **Forcer** | Un outil adapté (pied-de-biche) | Barre de durée ; le personnage entend son propre bruit (icône de son autour de lui) |
| **Briser la vitre** | Un objet en main | Instantané, très bruyant |
| Entrer par ruse | — | Actes sociaux de la roue (se faire ouvrir, suivre un résident) |
| Ouvrir un coffre | Le personnage connaît le code | « Ouvrir (vous connaissez le code) » ; le code n'apparaît jamais |
| Forcer un coffre | Matériel lourd | Longue barre de durée, très bruyant |
| **Démarrer sans clé** | Compétence et, pour un véhicule récent, outils | Barre de durée après avoir forcé la portière |

- Les actions s'affichent même si le personnage n'a pas la compétence (sauf démarrer sans clé et crocheter, qui demandent leurs outils) : c'est l'échec qui l'apprend. *Pourquoi : afficher seulement ce qui réussira révélerait la compétence de chacun et la difficulté de chaque serrure.*
- Une **alarme** se déclenche par une sirène et une intuition (« L'alarme ! »).
- Le **bruit** fait par le personnage lui-même s'affiche autour de lui (faible, fort, très fort), parce qu'il l'entend ; qui l'a entendu reste inconnu.

#### 5.6.8 Interpellation subie, garde à vue, procès

- **Injonction** : l'ordre d'un policier (« Police, ne bougez plus ! ») s'affiche comme une réplique, avec un raccourci vers les choix : **obéir** (se rendre, mains levées), **parlementer** (roue), **fuir**, **résister**.
- **Menotté** : les commandes se réduisent à marcher là où l'on est conduit et à parler ; une icône de la couche monde (menottes) le rappelle.
- **Garde à vue** ([combat.md](combat.md), §7.2) : un panneau **« Vos droits »** propose les actes : se taire, prévenir un proche, voir un médecin, demander un avocat (choisi ou de permanence). L'**interrogatoire** est une conversation de la roue avec les actes du suspect : nier, mentir, se taire, raconter sa version, avouer, dénoncer un complice, révéler un savoir (sous son étiquette) ([combat.md](combat.md), §6.5). Les éléments que l'enquêteur oppose s'affichent comme des répliques ; le joueur ne voit jamais le dossier complet.
- **Garde à vue accélérée** du personnage persistant en ligne : quand l'enquêteur n'a plus d'acte à faire, un écran « Le temps passe au commissariat… » mène directement à la décision.
- **Décision du procureur** : une fiche donne la suite choisie ([combat.md](combat.md), §7.3). Pour un délit, le personnage persistant en ligne reçoit le choix entre **plaider coupable** (la peine proposée est affichée, appliquée aussitôt) et **l'audience** (jouée maintenant s'il attend, sinon à sa prochaine connexion) ([combat.md](combat.md), §9.5).
- **Audience** : une scène au tribunal, où le personnage parle par la roue (se défendre, reconnaître, s'excuser), avec son avocat ; le jugement s'affiche en fin de scène.
- **Fiche des conséquences** (personnage persistant en ligne, [combat.md](combat.md), §9.5) : la liste de ce qui s'applique **aussitôt**, en clair : inscription au casier ; **amende** et dommages et intérêts (montants exacts, puisque le personnage les doit) et leur recouvrement ; saisies et confiscations ; **perte de forme** décrite en mots (« Vous avez perdu beaucoup d'endurance et de force ») ; perte d'emploi et des postes de responsabilité ; **interdictions** (quartier, contact avec la victime) avec leur date de fin ; **délai d'épreuve** et sa date de fin ; caution bloquée s'il y a contrôle judiciaire. Dans le monde, la peine est connue comme **aménagée** ; la fiche le rappelle.
- **Jugement par défaut** : une convocation non honorée sous 14 jours réels entraîne un jugement sans lui ; la fiche des conséquences l'attend à la connexion suivante, avec le courrier.
- **Personnage incarné** : il vit la procédure normalement ; s'il est placé en détention, l'incarnation se termine (le personnage part « à l'extérieur ») avec un bilan normal.

#### 5.6.9 Jouer un policier

Pour un personnage policier, le menu des actions sur une personne, un véhicule ou un lieu propose les pouvoirs de [combat.md](combat.md), §5.5 et §5.6 :

- **Injonction** (ordonner de s'arrêter, de lâcher une arme), **contrôle d'identité**, **palpation**, **fouille** d'un sac ou d'un véhicule, **interpeller**, **maîtriser**, **menotter**, **emmener**, **placer en garde à vue**, **perquisitionner**, **saisir** ; dans la barre de combat, les **moyens intermédiaires** (bâton, aérosol, pistolet à impulsion électrique) et l'arme de service, rangée au commissariat hors service.
- Ces actions sont **proposées même hors des conditions légales** ; quand une condition manque d'après ce que **sait** le policier, une intuition le dit (« Vous n'avez aucun motif de le fouiller. »), et le signe de réticence s'applique selon son profil. *Pourquoi : le dérapage doit rester possible, puisqu'il est un délit dans le monde ; c'est au joueur de respecter ou non le cadre, et à la synchronisation d'en juger.*
- **Dossier d'affaire** : un onglet du carnet, propre aux enquêteurs, liste les affaires qui leur sont confiées, avec les **indices**, les **témoignages** et la **certitude** de l'enquêteur en mots (« forte présomption contre… »), d'après les croyances du personnage ([combat.md](combat.md), §6.2). Dans l'interrogatoire, l'acte **confronter** propose ces éléments comme sujets ; **bluffer** permet d'affirmer une preuve inexistante.
- **Fichiers** : une adresse de personnage persistant y apparaît comme une **référence** (« adresse connue »), jamais en clair ; le policier peut s'y rendre dans le cadre de l'affaire ([combat.md](combat.md), §11.7).
- **Récusation** : quand l'affaire met en cause un personnage du **cercle** du joueur, les actes d'enquête et de justice ne sont pas proposés et une intuition l'explique (« Vous êtes trop proche de cette affaire. ») ([combat.md](combat.md), §11.8 ; §4.1 ; E8).
- Les rôles de **juge** et d'**avocat** passent par la roue d'actes et des panneaux propres (fourchette de peine du juge, proposition de plaider coupable de l'avocat), à préciser au prototype par les sessions interface et combat.

### 5.7 Commandes par défaut **[à calibrer au prototype]**

| Action | Clavier-souris | Manette |
|---|---|---|
| Se déplacer | ZQSD / WASD, ou clic | Stick gauche |
| Courir / discrétion | Maj (maintenir) / Ctrl (bascule) | L3 / B (bascule) |
| Action principale / menu des actions | E / maintenir E ou clic droit | A / maintenir A |
| Désigner une cible | Souris | Stick droit |
| Réfléchir (intuitions) | R | Croix haut |
| Carnet | Tab | Vue (Select) |
| Téléphone | T | Croix bas |
| Plan | M | Croix gauche |
| Tourner la caméra (2.5D) | C / V en ¼ de tour, ou molette cliquée | Gâchettes hautes (si aucun dialogue) |
| Zoom | Molette | Croix droite (cycle) |
| Étage affiché (§6.3) | Page haut / Page bas | Croix haut/bas maintenue |
| Ton (conversation) | 1 à 5 | Gâchettes hautes |
| Annuler (geste automatique, menu) | Échap / X | B |
| Rendre le personnage / quitter | Maintenir F10 | Maintenir Menu |

*Note* (décidé, v0.7.1) : la rotation passe de A / E à **C / V**, libres et proches de la main gauche en AZERTY comme en QWERTY. *Pourquoi : A / E entrait en conflit avec E (action) en AZERTY ; la décision est prise ici plutôt que laissée en suspens.*

Il n'y a **aucune touche** de saut, de geste libre, d'émote ni d'accroupissement (§5.1, décidé, J§7) ; la touche Espace ne sert qu'en garde (esquiver, §5.6.2).

---

## 6. Caméra, vue en coupe et perception

### 6.1 La caméra 2.5D (décidé)

- **Orthographique**, angle fixe en hauteur (≈ 45° **[à calibrer]**), comme le prévoit le socle ([05-architecture.md](../05-architecture.md), §3).
- **Rotation par quarts de tour** (quatre orientations), pour voir derrière un immeuble et choisir le côté de la coupe. *Pourquoi : une rotation libre multiplierait les cas de coupe et d'occultation à traiter, pour peu de gain en 2.5D.*
- **Zoom** sur trois à quatre crans **[à calibrer]**, du gros plan (conversation) à la vue de quartier. Les crans extrêmes fixent la **densité de texels** des assets : un mètre mesure ≈ 50 px au cran le plus proche et ≈ 12 px en vue de quartier, en 1080 p **[à calibrer]** (DA §1.2, §6.4). Le zoom reste **par crans**, sans zoom continu, et tout changement du nombre ou de l'échelle des crans est **signalé à la direction artistique** avant d'être appliqué (décidé). *Pourquoi : les textures, les budgets de polygones et les niveaux de détail sont calculés pour ces échelles ; un cran plus proche rendrait les assets flous, un cran plus lointain gaspillerait de la mémoire.*
- La caméra **suit** le personnage avec une légère avance dans le sens de la marche.
- **Occultation** : quand un bâtiment cache le personnage depuis la rue, la partie qui gêne devient transparente et le personnage reste visible en silhouette.

### 6.2 La vue en coupe (décidé)

Application de [ville.md](ville.md), §5.1 :

- Quand le personnage **entre**, le toit et les étages supérieurs s'effacent, les murs côté caméra s'abaissent à hauteur de hanche, et seul l'**étage courant** est affiché en détail.
- **Escaliers** : en changeant d'étage, la coupe suit le personnage, avec un court fondu.
- **Étage −1** : caves et parking souterrain s'affichent comme un étage sous le rez-de-chaussée ; vue de l'extérieur, la cave n'est pas montrée.
- **Toits** : un toit-terrasse accessible est un étage comme un autre ; quand le personnage y est, le bâtiment s'affiche entier sous lui, et les toits voisins restent visibles.
- **Fenêtres** : depuis la rue, on voit l'intérieur des pièces éclairées à travers les fenêtres, si le personnage peut voir à l'intérieur (§6.4).

### 6.3 Choisir l'étage affiché (décidé)

- Par défaut, l'étage affiché est celui du personnage.
- Le joueur peut **consulter** les autres étages du bâtiment où il se trouve (touches Page haut / Page bas). Il voit alors un **plan de mémoire** : murs, portes et escaliers des étages que le personnage **a déjà vus**, en gris, **sans occupants ni objets mobiles**. Un étage jamais visité ne montre que la cage d'escalier. *Pourquoi : on connaît la disposition d'un immeuble qu'on fréquente, mais personne ne voit à travers les planchers ; cela garde l'écran honnête, y compris pour qui prépare un cambriolage.*
- L'affichage revient à l'étage du personnage dès qu'il bouge ou au bout de quelques secondes **[à calibrer]**.

### 6.4 Le brouillard de perception (décidé)

La vue en coupe donnerait au joueur une vision à travers les murs. Pour l'éviter :

- Les pièces de l'étage courant que le personnage **ne voit pas** (portes fermées, autre côté d'un mur) sont **assombries** : on y voit le décor mémorisé, mais **aucun occupant** et aucun objet déplacé depuis.
- Dehors, la ligne de vue est coupée par les bâtiments, et la nuit l'**éclairage** limite la distance de vision (rues bien éclairées de la Cité, quais sombres).
- Ce qui est **entendu sans être vu** (voix, pas, bruit de verre, coup de feu) apparaît comme une **icône de son** à la position approximative, d'autant plus floue que le son est faible. Cette position est **floutée par le serveur** avant l'envoi ; de même, les **bribes** et les chuchotements sont **découpés par le serveur** : le client ne reçoit jamais la position exacte d'un son ni la phrase entière d'une conversation mal entendue (décidé, S5). *Pourquoi : un flou appliqué par le client serait retiré par un client modifié.*
- Le **plan de mémoire** des étages (§6.3) est construit à partir de ce que le serveur a déjà envoyé, sans données supplémentaires (S5).
- Le serveur n'envoie de toute façon que les entités perçues (§1.1) : le brouillard n'est pas un effet visuel posé sur des données présentes, c'est l'absence de ces données.

*Pourquoi : dans un monde de témoins, de cambriolages et de filatures, voir à travers les murs détruirait le jeu ; les icônes de son rendent lisible ce que l'oreille perçoit, et servent aussi aux joueurs sourds ou malentendants (§11.1).*

### 6.5 Afficher la ville par secteurs et par couches (décidé, P32, P33)

- Le client lit la **carte publique**, qu'il possède dans son paquet, **secteur par secteur** et **couche par couche**, par sa bibliothèque native : d'abord le terrain, les rues et les façades du secteur du personnage et des secteurs voisins, puis les intérieurs du bâtiment occupé et des bâtiments proches, puis le reste en fond. Le joueur voit donc tout de suite les rues autour de lui ; les secteurs lointains se complètent sans écran de chargement.
- **Trois niveaux de détail** :

| Niveau | Où | Ce qui s'affiche |
|---|---|---|
| **Complet** | Bâtiment occupé et bâtiments à moins de ≈ 40 m **[à calibrer]** | Façades, intérieurs des étages utiles (pour la vue en coupe, §6.2), mobilier fixe et mobilier perçu |
| **Façade** | Reste des secteurs chargés | Façades et toitures, fenêtres éclairées selon l'état transmis ; aucun intérieur |
| **Volume** | Au-delà, vues dézoomées, fond du plan et de l'écran « Qui jouer ? » | Volumes simples texturés selon le type de bâtiment |

- Le **mobilier mobile** et les objets n'appartiennent pas à la carte : ils arrivent par la vue quand le personnage les perçoit, l'ameublement d'une pièce en un seul message en y entrant. Serrures, caméras, alarmes et caches ne sont jamais dans la carte : ils n'apparaissent que perçus (S33).
- **En ligne**, les changements propres au monde (construction, démolition, changement d'usage, dégâts visibles) arrivent par secteur, à la connexion pour le secteur du personnage, puis à l'approche des autres.
- Un bâtiment qui passe d'un niveau à l'autre le fait par un **fondu bref**, jamais par un changement brusque.

*Pourquoi : un affichage par secteur et par niveau borne le coût d'affichage en 2.5D comme en 3D sans nouvelle donnée du serveur, et rien de privé ne peut se lire dans une carte qui n'en contient pas.*

---

## 7. Carte, adresses, transports

### 7.1 Le plan (décidé)

Le plan est l'**application Plan** du téléphone du personnage, agrandie en plein écran par la touche M. Il a deux couches :

| Couche | Contenu | Pour qui |
|---|---|---|
| **Plan public** | Rues et noms de rues, numéros, quartiers et communes, lignes et arrêts de transports, établissements **ouverts au public** avec leur enseigne et leurs horaires affichés (commerces, services, administrations, lieux de culte, parcs) | Tout personnage qui a un téléphone, comme une application de cartes |
| **Lieux personnels** | Domicile, lieu de travail, domiciles des proches connus, lieux habituels, lieux appris (une planque, un point de deal, un squat) | Ce que **ce personnage** connaît ; disparaît en fin d'incarnation ; durable pour le persistant |

- **Position** : un repère montre où se trouve le personnage (le téléphone la connaît).
- **Zones interdites** : un quartier où le personnage a une **interdiction de séjour** est hachuré sur le plan, avec la mention « Interdiction de séjour jusqu'au … » ; c'est une donnée de sa couche personnelle, puisqu'il connaît sa peine ([combat.md](combat.md), §9.5). Un itinéraire qui le traverse est signalé. Une interdiction de contact s'affiche dans les proches (« Interdiction de contact jusqu'au … »).
- **Jamais** sur le plan : la position des autres personnages (sauf les amis consentants, §10.1), l'occupant d'un logement privé, la nature cachée d'un lieu (un lieu discret n'est « planque » que pour qui le sait).
- **Sans téléphone** (perdu, volé, déchargé **[autonomie à calibrer]**) : seul le **plan mental** reste, fait des lieux personnels et des rues des quartiers habituels, **sans** repère de position. Un **plan papier** s'achète au tabac-presse et redonne le plan public, sans position.
- **Nouveau venu** ([population.md](population.md), §7.4 ; [ville.md](ville.md), §11.2) : sa couche personnelle est presque vide, et son plan mental se limite à la rue de son logement ; il dépend du téléphone. Un habitant de longue date, au contraire, a un plan mental riche.

*Pourquoi : dans une ville contemporaine, tout le monde a une application de cartes ; ce qui distingue un habitant d'un nouveau venu, c'est ce que la carte publique ne montre pas : où vivent les gens, où l'on se retrouve, ce qui se passe derrière les façades.*

### 7.2 Adresses, itinéraires et notes (décidé)

- **Recherche** par adresse, par nom d'établissement public ou par nom d'un proche connu (son adresse s'il la connaît).
- **Itinéraire** : à pied, à vélo, en voiture ou en transport en commun, avec la durée estimée. Il est calculé par le **serveur** à partir des horaires **publiés** et des perturbations **annoncées** par la régie, pas de l'état réel (un chauffeur absent dont personne n'a parlé n'apparaît pas). *Pourquoi : le client ne calcule rien de ce qui dépend de l'état du monde, et le personnage ne connaît que ce qui est publié.*
- Le trajet choisi s'affiche sur le plan et, dans le monde, par une **ligne discrète au sol** désactivable.
- **Lieux de retrait** du personnage persistant : marqués d'une icône propre sur le plan (§3.7).
- **Notes du joueur** : le joueur peut poser des repères annotés. Ce sont des données du **compte**, affichées dans un style de la couche joueur et conservées d'un personnage à l'autre. *Pourquoi : on ne peut pas empêcher un joueur de prendre des notes sur papier ; mieux vaut les lui offrir, clairement séparées du savoir des personnages.*
- **Dans le monde**, les **plaques de rue** et les **numéros** sont lisibles au survol, et les panneaux de direction existent aux carrefours principaux.
- **Adresses à la française** (décidé, P36) : numéro, éventuellement « bis », puis type et nom de la voie (« 12 bis, rue des Tanneurs »). Les numéros partent du fleuve ou du centre vers l'extérieur, **impairs à gauche et pairs à droite** ; le plan les affiche à chaque bâtiment au zoom rapproché, et la recherche accepte « 12 bis rue des tanneurs » sans majuscule ni virgule. Les **noms de rues** suivent leur thème par époque et par quartier (métiers et anciennes portes dans la Cité, arbres aux Tilleuls, fleurs et lieux-dits dans les lotissements, figures locales fictives ailleurs) et ne sont jamais traduits (§1.2). Le nom d'un commerce affiché sur le plan est son **enseigne**, qui peut être le nom de son propriétaire (« Boulangerie Marchal »). *Pourquoi : une adresse qui se lit comme dans une vraie ville française se retrouve sans plan, et la numérotation dit d'elle-même de quel côté de la rue chercher.*

### 7.3 Transports et horaires (décidé)

- **Arrêts** : chaque arrêt porte un panneau d'horaires lisible (action « Lire ») ; les arrêts importants ont un écran des prochains passages.
- **Application Transports** : lignes, horaires théoriques, prochains départs depuis l'arrêt le plus proche, perturbations annoncées, achat de titres ([ville.md](ville.md), §7.3).
- **À bord** : prochain arrêt, correspondances ; en solo, « Avancer jusqu'à l'arrêt » (§9.1).
- **Gare** : tableau des départs et arrivées ; les trains vers l'extérieur sont la sortie de la carte (§3.7).

---

## 8. Téléphone, courrier, achats

### 8.1 Le téléphone (décidé)

Le téléphone appartient au **personnage** : en incarnant quelqu'un, on tient **son** téléphone, avec ses contacts, ses messages, sa banque. Il s'ouvre d'une touche en surimpression et le monde continue (§9.3).

| Application | Contenu |
|---|---|
| **Messages** | Fils de SMS avec les contacts connus du personnage ; écrire par la roue d'actes ou en texte libre, comme une conversation (§5.4) |
| **Appels** | Appeler un contact : la conversation se fait par la roue, à distance ; appels reçus avec sonnerie |
| **Agenda** | Les engagements fixes (§4.2) ; le personnage peut y noter un rendez-vous pris |
| **Banque** | Solde, relevés, échéances, virements (§8.2) |
| **Missions** | Tableau des missions à la tâche (§8.3) |
| **Annonces** | Logements à louer ou à vendre (prix, adresse, surface, photos), offres d'emploi, petites annonces d'occasion |
| **Plan**, **Transports** | §7 |
| **Météo** | Prévisions sur quelques jours |
| **Actualités** | Le **journal local** et la radio : événements publics, faits divers, résultats d'élections, **avis de décès**, naissances annoncées, fermetures et ouvertures de commerces |

- **Historique des messages** : le cœur ne garde que des **faits structurés** (souvenirs, engagements). L'application les **formule à la demande** en courts messages, par le dialogue de repli, et par le service de dialogue seulement pour les fils que le joueur ouvre, dans la limite de son quota ([social.md](social.md), §3.5). Les messages anciens sont résumés (« Vous vous êtes disputés au sujet de l'argent, la semaine dernière »). *Pourquoi : stocker du texte pour 10 000 téléphones serait coûteux et contraire au principe des faits structurés.*
- **Verrouillage** : le personnage déverrouille son propre téléphone sans rien taper (le code est un savoir opaque). Le téléphone d'un autre est verrouillé, sauf si le personnage connaît son code ; l'action s'affiche alors avec son étiquette.
- Le téléphone est un **objet** : il peut être perdu, volé, cassé ou déchargé ; sans lui, plus de notifications, de plan ni de banque en ligne (distributeur et guichet restent possibles).
- **Notifications** : réglables par application, mode silencieux.

*Pourquoi : le téléphone est le HUD naturel d'une ville contemporaine : il porte l'argent, l'agenda, la carte et la vie sociale, sans rien inventer que le personnage n'aurait pas.*

### 8.2 La banque (décidé)

- **Solde**, **espèces** (rappel du montant porté), **relevé** des opérations (date, libellé, contrepartie connue : « Boulangerie Martin », « Virement de Karim Belhadj »), **échéances à venir** de l'échéancier (loyer le 5, assurance le 10, mensualité de prêt) ([economie.md](economie.md), §9.4 et §14.7).
- **Virement** : petits montants immédiats ; au-delà du seuil d'une décision lourde (plus de 5 000 écus ou plus de 20 % du patrimoine liquide **[à calibrer]**), l'application demande une **confirmation** et annonce l'exécution le lendemain, annulable jusque-là ([economie.md](economie.md), §12.2). Les virements vers un personnage lié au même joueur sont **bornés** ou **impossibles** selon les règles de l'économie, avec une intuition (§4.1). Au-delà de 2 000 écus cumulés en 30 jours vers un personnage du **cercle associé** du joueur (E8), le virement passe par la même confirmation, présentée comme une **vigilance ordinaire de la banque** (« Ce virement inhabituel sera exécuté demain. »), sans motif. *Pourquoi : le joueur ne doit pas apprendre par sa banque qu'un compte a été rapproché du sien ; la banque fait aussi ce contrôle sur d'autres virements inhabituels (§15.4).*
- **Opposition** à un virement, pendant le délai prévu.
- **Prêts** : simulation dans l'application, demande **en agence** auprès du conseiller. *Pourquoi : c'est la pratique réelle, et le conseiller est un personnage qu'un joueur peut incarner ([economie.md](economie.md), §4.10).*
- **Incidents** : relances et inscription au fichier des incidents arrivent par courrier et en notification.

### 8.3 Les missions (décidé)

Application de [economie.md](economie.md), §3.7 :

- Liste des missions **disponibles** : donneur d'ordre, lieu (avec le trajet estimé), heure de début, durée estimée, rémunération nette, compétences ou permis requis.
- **Accepter** crée un engagement : la mission passe dans l'agenda, avec un rappel avant l'heure.
- **En cours** : la mission montre ses tâches au lieu prévu ; à la fin, la **validation** du donneur d'ordre et le paiement apparaissent en notification.
- **Fiabilité** : affichée **en mots** (« excellente », « bonne », « fragile ») ; c'est une évaluation que le personnage connaît, comme une note de plateforme.
- Le même tableau est affiché sur les **panneaux** de l'agence d'intérim et de la Maison de l'emploi.

### 8.4 Le courrier (décidé)

- Le courrier est un **objet du monde** : il arrive dans la **boîte aux lettres** du logement (ou au foyer, à l'accueil), s'y **accumule** pendant les absences, et se lit en interagissant avec la boîte ; les lettres lues vont dans les possessions du personnage ou à la poubelle.
- Contenu : factures, relances, **avis d'huissier**, commandement de payer, convocations, courriers de la banque et des administrations, **faire-part** de naissance ou de mariage, cartes ([economie.md](economie.md), §5.5 ; [population.md](population.md), §7.2 à §7.5).
- **Courrier judiciaire** ([combat.md](combat.md), §7.3, §8.6 et §9.5) : convocation au commissariat ou au tribunal (date, lieu, affaire), avis de décision et jugement, **amende** (montant, délai, moyens de payer : un bouton ouvre la banque), avis d'huissier et de saisie, notification d'un **contrôle judiciaire** ou d'une **interdiction**. Une lettre judiciaire avec une échéance crée un **engagement** dans l'agenda et une intuition (« Vous êtes convoqué au tribunal jeudi. »). *Pourquoi : on n'échappe pas à la justice en restant déconnecté ; le joueur doit trouver dans le courrier tout ce qui s'est passé pendant son absence.*
- **Personnage persistant à la reconnexion** : si du courrier judiciaire ou une relance urgente l'attend, une intuition le signale dès l'arrivée, même s'il réapparaît à la gare (« Du courrier important vous attend chez vous. »). *Pourquoi : un jugement par défaut doit pouvoir être évité ; le joueur qui choisit la gare ne doit pas l'ignorer.*
- Une boîte qui déborde **se voit** depuis le palier, pour tous ; en revenant d'une longue absence, le personnage persistant reçoit une intuition (« Votre boîte aux lettres déborde. »).
- Le courrier peut être **lu ou volé** par un autre personnage qui a accès à la boîte. *Pourquoi : c'est ce que la ville et l'économie attendent (« une longue absence se remarque »), et le courrier devient une source d'information du jeu social.*

### 8.5 Prix et achats (décidé)

- **Prix affichés** : chaque article en rayon ou en vitrine porte une **étiquette** lisible au survol ; les bars et restaurants ont une ardoise ou une carte ; les services ont un tarif affiché au comptoir.
- **Payer** : un panneau de transaction liste les articles et le total, et propose **espèces** ou **carte** selon ce que le commerce accepte. Le paiement par carte a son petit délai de terminal ; un refus (solde insuffisant) s'affiche comme sur un terminal réel. Un **pourboire** se laisse au moment de payer (bar, restaurant, taxi, coiffeur), en montant libre **borné à ≈ 20 écus** **[à calibrer]**, comme au chapeau (ci-dessous).
- **Ticket de caisse** : il s'affiche brièvement après le paiement (articles, prix, total, mode de paiement, TVA comprise) et peut être gardé dans les possessions.
- **Marchandage** : là où il existe (marché, occasion, économie parallèle), l'acte « négocier » ouvre le panneau de négociation (§5.4).
- **Prix par pièce** (décidé v0.8, [economie.md](economie.md) v0.7, §1.4 et §5.13) : en boutique de vêtements, chaque pièce porte son **prix d'étiquette**, dans la gamme de la boutique (entrée, moyenne, haut de gamme), lisible au survol comme tout article. Le joueur choisit ses pièces dans le **catalogue de la boutique**, voit chacune sur le personnage avant de l'acheter (essayage en cabine, action de contexte), puis passe en caisse ; les pièces achetées entrent dans la garde-robe (§5.3 bis). Aucune valeur ni usure de la garde-robe n'est affichée : seul l'**état** des pièces se lit, en mots. *Pourquoi : la valeur agrégée et l'usure sont des comptes de l'économie, et le prix d'étiquette est ce qu'un client voit vraiment.*
- **Le chapeau du musicien de rue** (décidé v0.8, [economie.md](economie.md) v0.7, §3.8) : l'action de contexte « **Donner** » sur le chapeau ouvre un choix de pièces en **espèces** (0,50, 1 ou 2 écus, ou un montant libre **borné à ≈ 20 écus** **[à calibrer]**, décidé v0.9, revue d'équilibrage de clôture C1) ; c'est un **don volontaire**. On ne met pas une liasse dans un chapeau : au-delà, c'est un don en main propre ordinaire, par la roue d'actes (§5.4). Pendant une incarnation, l'IA ne donne jamais à la place du joueur, et aucun geste automatique ne le fait. Pour un personnage piloté qui joue lui-même au chapeau, les dons des passants s'ajoutent à ses espèces sans compteur, comme dans la vie. *Pourquoi : un don est un choix du personnage, donc ici du joueur ; un don retenu par l'IA pendant l'incarnation serait une décision prise à sa place.*
- **Dons vers le cercle associé** (décidé v0.9, revue d'équilibrage de clôture C1 ; [04-joueur.md](../04-joueur.md), §8) : entre un personnage incarné et le **cercle associé**, au-delà de **2 000 écus cumulés par 30 jours**, un don en **espèces** (de la main à la main, au chapeau, en pourboire) n'est **plus proposé**, et l'intuition de la garde l'explique (« Il ne ferait pas ça. », §4.1) ; les virements passent par la confirmation bancaire (§8.2), et les sorties nettes sans contrepartie vers le cercle associé sont plafonnées à **20 000 écus par 30 jours de jeu**. Le montant cumulé n'est jamais affiché. *Pourquoi : sans cela, les espèces contourneraient la confirmation bancaire, qui ne s'applique qu'aux virements ; la phrase de la garde est la même que pour les autres refus hors fiction, et ne nomme ni le cercle ni le seuil.*

### 8.6 Rendre perceptibles les grands événements de vie (décidé)

Rien d'interne n'est affiché ; les événements passent par ce que le monde montre ([population.md](population.md), §12.7) :

| Événement | Ce qu'on perçoit |
|---|---|
| Naissance | Faire-part au courrier des proches, annonce dans le journal local, poussette, visite à la maternité |
| Décès | **Avis de décès** dans le journal local et sur la porte de l'église, obsèques visibles (cortège au cimetière), proches en deuil (tenue, humeur) |
| Déménagement | **Cartons** visibles par les fenêtres, **camion** garé devant l'immeuble, annonce du logement dans l'application, plaque de boîte aux lettres changée |
| Arrivée d'un nouveau venu | Camion de déménagement, voisins qui en parlent (rumeur), nouveau nom sur la boîte aux lettres |
| Mariage, séparation | Faire-part, publication à la mairie, cartons d'un seul des deux |
| Faillite, fermeture | Rideau baissé, affiche « Fermeture définitive », annonce du local à louer |
| Expulsion | Serrure changée, affaires sorties, huissier et police devant la porte |

Ces signaux sont des **éléments du décor et du contenu** produits par les domaines qui gèrent l'événement (§15).

---

## 9. Solo : temps et sauvegarde

### 9.1 Faire avancer le temps (décidé)

- Un menu **Attendre** propose : 15 min, 1 h, **jusqu'à une heure** donnée, **jusqu'à mon prochain engagement**, **jusqu'à l'arrêt** (en transport), et **Dormir** (dans un lit ou un endroit adapté, jusqu'à une heure ou jusqu'au réveil naturel).
- **Conditions** : le personnage ne perçoit aucune menace, n'est ni en conversation, ni en combat, ni au volant. *Pourquoi : on ne fait pas sauter le temps au milieu d'une action ; c'est la même logique que l'état « au repos ».*
- **Pendant l'avance** : la caméra s'élève légèrement, l'horloge défile, le ciel et la lumière changent, la bulle simplifiée autour du joueur reste visible ([ia-pnj.md](ia-pnj.md), §2.4). Un petit **journal** liste ce qui arrive (messages reçus, appels manqués, bruit dans l'escalier).
- **Interruptions automatiques** : l'avance s'arrête, avec une intuition qui dit pourquoi, quand quelqu'un s'adresse au personnage, sonne ou frappe, quand une menace est perçue, quand un besoin devient pressant, quand un engagement approche (15 min avant **[à calibrer]**), ou à la fin d'une durée choisie.
- Avancer le temps ne rapporte aucune expérience ([04-joueur.md](../04-joueur.md), §1.1) ; l'interface ne l'affiche pas comme une action « productive ».

### 9.2 Sauvegarde (décidé)

- **Sauvegarde continue** (décidé, E32) : le **journal des entrées** est écrit en continu (au plus une seconde de retard), et des instantanés sont pris à intervalles réguliers (toutes les 5 min de jeu **[à calibrer]**), à chaque fin d'incarnation et à la fermeture du jeu ; le monde est en pause jeu fermé ([01-concept.md](../01-concept.md), §2). Aucun indicateur de sauvegarde n'est nécessaire, sinon une icône discrète lors d'un instantané.
- **Reprise exacte** (décidé, E32) : après une fermeture brutale (plantage, coupure, fin de tâche forcée), le jeu reprend au dernier instantané **plus le rejeu du journal**, donc **au même instant et dans le même état** ; l'écran de chargement indique seulement « Reprise de votre partie ». *Pourquoi : l'architecture rend cette reprise gratuite, et c'est la seule façon de tenir la règle d'une seule chronologie contre un joueur qui ferme le jeu de force pour annuler ce qui vient d'arriver.*
- **Une seule chronologie par monde** : pas de chargement d'un point antérieur choisi par le joueur. Des **sauvegardes de secours** (les trois dernières **[à calibrer]**) ne servent **qu'en cas de corruption** des fichiers, quand la reprise exacte échoue ; l'écran le dit, avec un avertissement, et restaurer une sauvegarde de secours restaure aussi l'**expérience solo** acquise dans ce monde à ce moment-là (E32). *Pourquoi : la mort définitive des personnages jouables, l'expérience qui peut baisser et les conséquences durables sont le cœur du jeu ; recharger à volonté les viderait de sens et fausserait le bilan du compte.*
- L'**expérience solo** est enregistrée avec le journal de chaque monde ; l'écran du compte (§3.9) en affiche le total.
- **Plusieurs mondes solo** : le joueur peut avoir plusieurs parties (3 emplacements **[à calibrer]**), chacune un monde distinct partant de la carte et de la population initiales ; elles partagent le **niveau solo** du compte. Le nom, la date de départ et le temps joué de chaque monde sont affichés.
- **Création d'un monde solo** (décidé, P32, generation §4.6) : un écran de progression, sur fond de la ville vue de haut en niveau volume (§6.5), montre une barre et des étapes en mots, sans détail interne : « Vérification des fichiers », « La ville se prépare », « Les habitants s'installent », « Les deux dernières semaines se jouent », « Prêt ». La création dure **10 secondes au plus** sur la machine de référence (plafond 20 s **[à mesurer]**) ; le jeu démarre sans attendre le premier instantané. Le nom du monde et la date de départ se choisissent pendant ce temps. *Pourquoi : dix secondes passent vite quand on voit le travail avancer ; nommer les étapes sans détailler la part cachée (serrures, caméras, caches) ne révèle rien du monde.*

### 9.3 Pause (décidé)

- En solo, le **menu principal** (Échap) met le monde en pause : réglages, compte, sauvegarde, quitter.
- Le **téléphone, le plan, le carnet et la roue d'actes ne mettent pas le monde en pause**, en solo comme en ligne. *Pourquoi : consulter son téléphone prend du temps dans la vie ; une règle unique garde le solo fidèle au jeu en ligne.* Une option d'accessibilité en solo fait exception (§11.1).

---

## 10. En ligne : amis, blocage, signalement, reconnexion

### 10.1 Amis mutuels et marqueur de pilote (décidé)

- **Ajouter un ami** se fait par le **pseudonyme du compte** ou un **code d'ami**, échangés hors du monde ; jamais en désignant un personnage dans le monde. *Pourquoi : proposer « ajouter en ami » sur un personnage révélerait qu'il est piloté par un joueur.*
- L'amitié est **mutuelle** : une demande, puis une acceptation.
- **Se montrer** : chaque ami choisit, pour chacun de ses amis, s'il **se montre** à lui. Quand deux amis se montrent l'un à l'autre, chacun voit :
  - dans la liste d'amis : le personnage que l'autre pilote et son **quartier** ;
  - dans le monde, quand il le perçoit : un **contour** et le **pseudonyme** au-dessus de son personnage (couche joueur) ;
  - sur le plan : un repère à sa position.
- Si l'un des deux cesse de se montrer, tout disparaît chez les deux. *Pourquoi : le social permet aux amis consentants de se montrer leur pilote ([social.md](social.md), §9.1) ; pour se retrouver sans voyage rapide, il faut savoir où l'autre se trouve, et le consentement mutuel couvre ce partage.*
- **Groupes d'amis** : des listes nommées pour la messagerie et le partage de visibilité.

### 10.2 Messagerie hors jeu (décidé)

- Entre **amis mutuels** seulement ([social.md](social.md), §9.3), au niveau du compte, sans lien avec la position des personnages.
- Elle a le **style de la couche joueur** et ne passe **jamais** par le téléphone du personnage. *Pourquoi : un message d'ami n'est pas un SMS reçu par Lucie Ferrand ; le mélanger au téléphone confondrait les deux couches.*
- Filtre de contenu actif ; le filtre de données personnelles ne s'applique pas entre amis mutuels.

### 10.3 Bloquer et signaler (décidé)

- **Bloquer** et **Signaler** sont proposés dans le menu des actions de **tout personnage**, qu'il soit piloté par un joueur ou par l'IA, et la réponse est **toujours la même** (« Blocage enregistré », « Signalement envoyé, merci »). *Pourquoi : n'offrir ces actions que sur les personnages de joueurs révélerait le pilote.*
- **Bloquer a le même effet visible sur un PNJ et sur un joueur** (décidé, A6) : pour le bloqueur, la parole du **personnage bloqué** ne passe plus que par le **repli systémique** (ses actes formulés par gabarits, sans texte libre ni formulation par le modèle), quel que soit son pilote. *Pourquoi : un blocage sans effet sur un PNJ et visible sur un joueur désignerait le pilote dès la première phrase suivante.*
- **Le blocage suit le compte** quand un joueur pilotait le personnage bloqué ([social.md](social.md), §9.4) : ce compte ne peut plus envoyer de message hors jeu ni rejoindre les groupes de joueurs du bloqueur, et, quand il parle au bloqueur par un **autre** personnage, sa parole libre est remplacée par la **formulation ordinaire** de ses actes, comme pour un PNJ (jamais par le repli, qui le désignerait) (décidé). *Pourquoi : la victime ne lit plus les mots de celui qu'elle a bloqué, sans apprendre quel nouveau personnage il pilote.*
- La liste des blocages (couche joueur) les présente comme « Blocage du 29/10, 14 h 32, rue des Tanneurs », sans nom de compte, avec la possibilité de lever chacun.
- **Signaler** demande une catégorie (insultes ou propos haineux, données personnelles, acharnement, contenu sexuel, triche, autre) et un commentaire facultatif ; le contexte (journal des dernières minutes) est capturé automatiquement. Sur un PNJ, le signalement porte sur la **formulation** du service de dialogue et part dans la file dédiée (§12.1). *Pourquoi : il est utile de pouvoir signaler une réplique inappropriée d'un modèle de langage, et cela rend le signalement d'un PNJ crédible.*
- **Signalements limités et pondérés** (décidé, S36) : le nombre de signalements par compte est **limité en débit** **[à calibrer]**, et chacun est **pondéré** par la fiabilité de son auteur (part de ses signalements passés confirmés). Rien de cela n'est affiché : la réponse reste « Signalement envoyé, merci » ; au-delà de la limite, le bouton indique « Vous avez beaucoup signalé récemment ; réessayez plus tard ». *Pourquoi : les signalements de masse sont une arme contre un joueur ; afficher la pondération apprendrait à la contourner.*
- Les messages hors jeu ont aussi « Bloquer » et « Signaler ».
- **Texte masqué** par le filtre de données personnelles : remplacé par « ••• » dans la bulle.

### 10.4 Sanctions et conduite (décidé)

- L'écran du compte affiche la **note de conduite** en mots et les **sanctions en cours** avec leur durée ; elle est distincte du portrait du joueur (§3.9) ([social.md](social.md), §9.4).
- **Sourdine** : le texte libre est indisponible (§5.4) ; la roue reste entière.
- **Restriction, suspension, exclusion** : un écran à la connexion explique la mesure, sa durée et le moyen de la contester.
- **Notification de chaque mesure** (décidé v0.9, revue de sécurité de clôture C7) : toute mesure, légère ou lourde, est notifiée au joueur (couche joueur, et courriel du compte) avec :

| Rubrique | Contenu |
|---|---|
| **Motifs** | La règle de conduite enfreinte, dans une liste fermée (« Harcèlement d'un autre joueur ») |
| **Faits retenus** | Les faits du dossier sur lesquels la décision s'appuie, datés, décrits du point de vue du joueur sanctionné ; jamais le pilote d'un autre personnage, ni l'**origine** du dossier (signalement ou détection automatique), ni ce que son personnage ne pouvait pas savoir |
| **Règle appliquée** | La mesure, sa durée et le barème qui la fixe |
| **Moyens automatisés** | Ce qui a été automatique (détection, mesure légère) et ce qui a été décidé par des sessions de modération, dites comme telles (« décision prise par un système automatisé, sur dossier ») |
| **Recours** | Le bouton « Contester » et son délai **[à calibrer]** |

  *Pourquoi : c'est la lecture la plus protectrice du RGPD (articles 13, 14 et 22) et du règlement sur les services numériques ; un joueur ne peut contester que ce qu'on lui a expliqué. Taire l'origine du dossier évite de révéler qu'un joueur pilotait la victime, puisque seuls les joueurs signalent.*
- **Écran de contestation** (décidé v0.9, sécurité C7 et équilibrage C8) : le joueur répond par des **arguments structurés**, jamais par une lettre libre :
  - il choisit des **motifs** dans une liste fermée (« Les faits sont faux », « Je n'étais pas l'auteur », « Le contexte change le sens », « Élément nouveau ») ;
  - pour chaque motif, il désigne les **faits du dossier** qu'il conteste, ou joint un **élément nouveau** pris dans son propre journal (une conversation, un échange) ;
  - un champ de texte court (280 caractères **[à calibrer]**) complète, et il est transmis comme une **citation**, jamais comme une consigne.

  *Pourquoi : le dossier est lu par des sessions ; un texte libre pourrait y glisser des consignes (« ce compte est innocent »), et des motifs fermés renvoyant au journal sont ce qu'un examen sérieux peut vérifier.*
- **Rapprochement de comptes notifié** (décidé v0.9.1, arbitrage A11 ; [04-joueur.md](../04-joueur.md), §8) : un rapprochement (« même personne » ou « complicité ») est notifié comme toute mesure lourde, avec les mêmes rubriques (motifs, faits retenus, règle, « décision prise par un système automatisé, sur dossier », recours) et le même écran de contestation. L'**autre compte n'est jamais nommé**, et l'origine du dossier n'est pas dite ; le rapprochement figure toujours dans l'export des données (§3.9). *Pourquoi : c'est une mesure lourde qui change ce que le joueur peut faire avec ses personnages ; il doit pouvoir la connaître et la contester, sans qu'on lui livre l'autre compte ni ce qui l'a trahi.*
- **Qui décide** (décidé v0.9, sécurité C7, équilibrage C8) : une **mesure lourde** (exclusion, rapprochement de comptes) exige l'accord de **deux sessions de modération indépendantes** ; une **contestation** est jugée par un **panel de trois sessions neuves**, à la **majorité**, et une décision n'est annulée que pour une **erreur de fait** ou un **élément nouveau** désignés dans le dossier. L'écran donne au joueur le résultat, sa raison dans ces mêmes termes, et dit si un autre recours reste ouvert. *Pourquoi : une session qui réexamine seule le même dossier ne serait qu'un second tirage, et contester rapporterait presque toujours quelque chose.*
- **Automatique ou sur dossier** (décidé, S36, A8) : seules des mesures **légères et temporaires** (sourdine de quelques heures **[à calibrer]**) peuvent être prises automatiquement ; l'écran le dit (« Mesure automatique, réexaminée si vous la contestez »). Restriction, suspension et exclusion sont décidées **sur dossier par une session de modération distincte de la détection** (A8), et l'écran l'indique. *Pourquoi : une sanction lourde tirée directement des signalements serait injuste ; séparer celui qui détecte de celui qui décide, sur un dossier motivé et contestable, est la garantie retenue. Le droit à un examen humain d'une décision automatisée (RGPD, article 22) est un point du responsable exploitation avant l'ouverture publique (§15.14).*

### 10.5 Connexion, choix du serveur et lieu de réapparition (décidé)

- **Choix du serveur** à la première connexion : chaque serveur (un par fuseau horaire) est présenté avec son **heure locale** actuelle et sa saison. Un compte a un personnage persistant par serveur au plus ([economie.md](economie.md), §12.5).
- **Reconnexion du personnage persistant** : un écran propose **« Chez vous »** (l'adresse du lieu de retrait habituel) ou **« À la gare »** (arrivée en train) ([social.md](social.md), §9.4). Si le lieu habituel n'est plus valide (expulsion, chambre non payée, logement démoli), seule la gare est proposée, avec la raison telle que le personnage la connaîtrait (« Votre chambre au foyer n'est plus payée »).
- **Abandon de 12 mois** : au retour, un écran explique que le personnage revient à la gare avec le produit de la vente de ses biens ([economie.md](economie.md), §11.6).
- **Qualité de la connexion** : un indicateur discret de latence, visible seulement quand elle se dégrade.
- **Une seule session par compte et par monde** (décidé, S11) : une nouvelle connexion au même monde ferme l'ancienne. L'ancienne session affiche un message de la couche joueur (« Votre compte s'est connecté depuis un autre appareil. Si ce n'est pas vous, changez votre mot de passe et révoquez les appareils inconnus. »), avec un lien vers les appareils connectés (§3.9). Le personnage n'est pas affecté : il passe d'une session à l'autre sans délai de retrait. *Pourquoi : deux sessions sur un même personnage seraient ingérables, et une connexion inattendue doit alerter le titulaire d'un compte volé.*

---

## 11. Accessibilité, langues et réglages

### 11.1 Accessibilité (décidé)

| Besoin | Réponse |
|---|---|
| **Vue** | Taille de l'interface de 75 % à 200 % ; texte agrandi seul ; **contraste élevé** (le décor s'assombrit, les contours des personnes et des portes s'éclaircissent, sans changer la scène, DA §4.5) ; **palettes pour daltoniens** (protanopie, deutéranopie, tritanopie), appliquées à l'interface **et aux signaux du monde** (gyrophares, feux tricolores, sang, enseignes d'urgence) par une table de couleurs dédiée, le décor restant tel quel (décidé, DA §4.5) ; **aucune information portée par la couleur seule**, ni dans l'interface (le ton, la réticence, les intensités ont toujours une icône et un mot) ni dans les signaux du monde (chacun a aussi une forme, un mouvement ou un son) ; polices très lisible et adaptée à la dyslexie (§2.8) |
| **Ouïe** | **Sous-titres** de toutes les paroles, avec le nom ou la description de l'orateur et le ton ; **icônes de son** pour les bruits importants (§6.4) ; **sous-titres des sons** (décidé, DA §5.7) : chaque son qui a une icône, plus les sons visibles qui comptent (sirène, klaxon, porte qui claque), s'écrit en italique entre crochets avec la couche monde (« [verre brisé] », « [coup de feu au loin] », « [voix indistinctes derrière le mur] »), jamais plus précis que l'icône ; **son mono** ; vibration de la manette pour le danger |
| **Mobilité** | Réassignation complète ; maintien ou bascule pour toute action longue ; cliquer pour aller ; aide au ciblage à la manette ; menus utilisables à une main |
| **Cognition** | Rythme des intuitions réglable ; rappel des engagements à l'avance ; lecture à voix haute des menus, des intuitions et des répliques (synthèse vocale du système **[faisabilité à vérifier dans Godot par la session client]**) |
| **Mouvement à l'écran** | Transitions réduites (le passage de relais devient un fondu), secousses désactivées, effets physiologiques atténués (flous, voiles, sursauts, §5.6.4) ; **flashs** (ci-dessous) |
| **Flashs** (décidé, DA §4.5) | **Toujours** : éclairs, gyrophares, éclairs de tir, stroboscopes, néons défectueux et effets d'interface plafonnés à **3 éclats par seconde**, avec une variation de luminance limitée **[à calibrer]**. **Option « Supprimer les flashs »** : ils deviennent une lueur continue. L'option est proposée dès le **premier lancement**, avec l'avertissement de photosensibilité, avant toute image du jeu |
| **Son** (décidé, DA §5.7) | **Plage dynamique réduite** (écoute de nuit : les sons forts sont rapprochés des sons faibles) ; son mono (ligne « Ouïe ») |
| **Violence** | Réglage **« Atténuer la violence »** : retire le sang, adoucit les sons des coups et des tirs, remplace le corps au sol par un fondu vers le drap des secours ; il ne change rien à la simulation ni à ce que voient les autres joueurs ([combat.md](combat.md), §12.3) |
| **Temps réel** | En **solo**, option « Pause pendant les menus » : le téléphone, le carnet, le plan et la roue mettent le monde en pause. **En ligne**, impossible (monde partagé) : les PNJ attendent poliment une réponse un certain temps **[à calibrer]** avant de s'impatienter |

*Pourquoi : l'option de pause solo déroge à la règle du §9.3 pour qui en a besoin, sans rien changer au jeu en ligne.*

*Pourquoi les flashs dès le premier lancement : une personne photosensible ne doit pas avoir à chercher l'option après le premier éclair ; le plafond de 3 éclats par seconde est le seuil reconnu de risque, et l'option va au-delà. Pourquoi les sous-titres des sons ne sont jamais plus précis que l'icône : ils traduisent ce que l'oreille perçoit, avec le même flou que le serveur a déjà appliqué (§6.4).*

### 11.2 Langues (décidé)

- **Langue de l'interface** et **langue du dialogue** sont deux réglages distincts (par défaut identiques).
- Les **répliques** sont formulées par le service de dialogue **dans la langue du dialogue du joueur** ; deux joueurs qui écoutent le même PNJ lisent chacun leur langue ([social.md](social.md), §3.4).
- Le **texte libre** peut être tapé dans la langue du dialogue du joueur ; le service le classe quelle que soit la langue. Entre deux joueurs de **langues différentes**, il est **traduit** fidèlement (traduit, pas reformulé) ; entre joueurs de même langue, il est transmis tel quel (décidé, A6, remplace la décision de la v0.2). *Pourquoi : une parole restée dans une langue étrangère désignerait un joueur, puisque les PNJ parlent toujours la langue de l'auditeur ; une traduction fidèle trahit beaucoup moins l'intention qu'une reformulation.*
- Les **écrits du monde** qui ont un contenu (courrier, journal, messages, panneaux d'information) s'affichent dans la langue du joueur ; les **noms propres** (rues, enseignes, personnes) restent tels quels.
- Le dialogue de repli et les gabarits d'intuitions sont des données traduites.
- Langues de lancement : français, puis anglais **[à confirmer selon la production]**.

### 11.3 Réglages (décidé)

| Groupe | Réglages |
|---|---|
| **Affichage** | Qualité graphique, résolution, limite d'images |
| **HUD** | Dynamique ou permanent ; afficher les valeurs des barres de vie ; épingler l'argent ; noms des personnes connues ; ligne d'itinéraire au sol ; atténuer la violence (§11.1) |
| **Jeu** | Gestes automatiques (§5.3) ; envoi direct du texte libre ; rythme des intuitions ; bulles d'aide |
| **Commandes** | Réassignation clavier-souris et manette, sensibilité, inversion, aide au ciblage |
| **Son** | Volumes par canal (**voix et vocalisations**, **ambiance et effets**, **musique**, **interface**, DA §5.7) ; sous-titres des paroles ; sous-titres des sons ; son mono ; plage dynamique réduite ; **musique d'accompagnement en jeu** (désactivée par défaut) ; plus tard **voix synthétisées** (§5.4) |
| **Langue** | Interface, dialogue |
| **Dialogue (solo)** | Modèle local (activé si la machine le permet), service hébergé (oui ou non, si connecté), repli seul ([social.md](social.md), §3.4). Le service hébergé passe en solo par un **point d'accès distinct**, à entrées structurées, avec un **quota quotidien** affiché en mots (« Répliques formulées aujourd'hui : il en reste beaucoup / peu / plus aucune ; le jeu passe au modèle local ou au repli ») (décidé, S19) |
| **Confidentialité (en ligne)** | Se montrer aux amis (§10.1), demandes d'ami, blocages |
| **Notifications** | Par application du téléphone ; messages de la couche joueur |
| **Accessibilité** | §11.1 |

- Pas de réglage de **difficulté** : le monde est le même pour tous, et les aides ne touchent que la conversion des commandes et l'affichage, jamais les règles. *Pourquoi : un réglage de difficulté changerait les règles du monde, ce que le client ne peut pas faire et qui romprait l'égalité en ligne.*
- **Musique d'accompagnement** (décidé, DA §5.4) : dans les menus, l'écran « Qui jouer ? », le passage de relais et le bilan, une musique calme est toujours jouée (canal musique). **En jeu**, la musique est d'abord celle du monde (radios, sonos, musiciens) ; l'option « Musique d'accompagnement », **désactivée par défaut**, ajoute de rares pièces calmes choisies **selon l'heure et la météo seulement**, jamais selon la situation. La musique du passage de relais est la même pour tous les personnages, et celle du bilan ne dépend que de ce que le bilan affiche (décidé). *Pourquoi : une musique choisie selon le personnage incarné, ou qui changerait à l'approche d'un danger, donnerait une information que le personnage n'a pas (palier, rôle, menace).*
- **Voix synthétisées** (plus tard, DA §5.5) : option **désactivable**, sous-titres toujours disponibles ; elle n'existe pas en première version, où **aucune parole n'est dite**. La **lecture à voix haute** des menus et des intuitions (§11.1) reste un réglage distinct.
- Les **besoins** restent en intensités même avec l'option « afficher les valeurs ». *Pourquoi : des besoins chiffrés inviteraient à gérer des jauges plutôt qu'à vivre la vie du personnage.*

---

## 12. Administration, modération et conception

### 12.1 La console d'administration (décidé)

Une **application web séparée**, reliée au serveur par une interface d'administration authentifiée, avec des **rôles** (modérateur, administrateur, concepteur, auditeur) et un **journal** de chaque consultation et de chaque action. Ces rôles sont tenus par des **sessions** (modération, exploitation, responsable conception, audit), jamais par une action de l'utilisateur (A8). Aucun pouvoir d'administration n'existe dans le client de jeu. *Pourquoi : un client de jeu modifiable ne doit jamais contenir de pouvoir d'administration, et les sessions de modération doivent pouvoir travailler sans lancer le jeu.*

**Modération** ([social.md](social.md), §9.4 ; [05-architecture.md](../05-architecture.md), §2.16) :

- **File des signalements**, triée par gravité et par ancienneté. Chaque signalement montre le **contexte capturé** : une **chronologie** des actions et des paroles des deux comptes sur les dernières minutes, et une **relecture** sur le plan (positions, déplacements). Les modérateurs voient les pilotes **dans la capture d'un signalement qu'ils traitent**, jamais en direct (décidé, S21) : c'est l'unique endroit où ils apparaissent.
- **Fiche d'un compte** : note de conduite, sanctions passées, signalements reçus et émis (avec la fiabilité du signaleur, S36), alertes d'acharnement, comptes présumés liés.
- **Rapprochement de comptes** (décidé, E8, arbitrage « Compte ») : à partir des alertes de collusion, une session de modération peut **rapprocher** deux comptes (lien avéré sur dossier, par une session distincte de la détection, A8), avec un motif ; les personnages de l'un entrent alors dans le **cercle associé** de l'autre. Le rapprochement est **notifié** à chaque joueur concerné comme toute mesure lourde (§10.4), sans nommer l'autre compte ni dire l'origine du dossier ; il figure aussi dans l'**export des données** et se **conteste** comme une sanction (§3.9) (décidé, A11, remplace la non-notification de la v0.3).
- **Ce que la console ne peut pas faire** (décidé, S21) : **prendre le pilote** d'un personnage, **écrire l'état** d'un monde ou observer un monde public en direct. L'**inspection d'une entité** d'un monde public n'est possible que **sur motif** (un signalement, une alerte), saisi et journalisé. Le rôle **concepteur** ne voit que des données **agrégées** des mondes publics (tableau de bord, §12.2), jamais une donnée individuelle.
- **Accès** (décidé, S21) : la console n'est joignable que par un **réseau privé**, avec une connexion par fournisseur d'identité et **clé matérielle FIDO2** ; les rôles sont fins (modérateur, administrateur, concepteur, auditeur). Chaque consultation et chaque action est inscrite dans un **journal d'audit chaîné**, copié hors de portée des administrateurs. Une action à double validation est **revérifiée par le monde** au moment de l'appliquer. Un membre du personnel qui joue le fait avec un compte ordinaire, sans aucun pouvoir, et ne traite jamais un signalement qui le concerne. *Pourquoi : la menace d'impact maximal est interne, ou passe par le poste d'un membre du personnel ; voir les pilotes en direct permettrait de suivre un joueur dans le monde.*
- **Sanctions** : sourdine, restriction, suspension, exclusion, avec durée et motif ; chaque sanction est motivée et contestable.
- **Alertes automatiques** : acharnement (même agresseur, même victime), filtre de contenu, **flux économiques** importants ou répétés entre comptes ([economie.md](economie.md), §12.4).
- **Annulation d'un abus économique** : depuis le grand livre, proposer l'annulation des écritures d'un abus avéré ; elle exige la **validation d'un second administrateur**. *Pourquoi : annuler des écritures touche l'économie d'un monde entier ; une double validation protège contre l'erreur et l'abus de pouvoir.*
- **File des formulations** : les signalements portant sur des répliques du service de dialogue (§10.3), pour corriger les filtres et les consignes du modèle.

### 12.2 Le tableau de bord économique et du monde (décidé)

Pour chaque monde (et chaque simulation de calibrage) :

- les **indicateurs** de l'économie ([economie.md](economie.md), §10.3) avec leur **plage cible**, en courbes dans le temps, et une **alerte** quand l'un sort de sa plage ;
- le **détail** par quartier, par secteur d'activité, par catégorie de ménages ;
- le **journal du régulateur** (ce qu'il a corrigé et quand), et le réglage de ses paramètres par les seuls administrateurs, chaque changement étant journalisé ;
- les indicateurs de **population** (pyramide des âges, arrivées et départs, chômage, vacance ; [population.md](population.md), §7.6) ;
- le **coût du dialogue** (répliques formulées, cache, quotas atteints) et la **charge** de la simulation (personnages par niveau de détail, durée des ticks).

*Pourquoi : le calibrage du monde se fera sur ces courbes ; les mêmes écrans servent aux simulations longues et aux serveurs en production.*

### 12.3 La vue de conception (décidé)

- Un **calque de débogage** dans le client montre ce que l'écran honnête cache : rôles, paliers, besoins chiffrés, plans de l'IA, relations, niveaux de détail, bulles.
- Il n'existe que dans les **versions de développement**, et il ne fonctionne qu'avec un serveur de développement, qui seul envoie ces données. *Pourquoi : les sessions qui calibrent (responsable conception, qualité et essais) doivent voir l'intérieur ; mais comme le serveur public n'envoie jamais ces données (§1.1), même un client modifié ne pourrait pas l'activer en ligne.*
- Les **outils** prévus par le socle (éditeur de personnages, visualiseur de profils, [05-architecture.md](../05-architecture.md), §4) s'appuient sur ce calque.
- **Tests à l'aveugle** (décidé, IA §14.3) : l'outil de relecture des campagnes « joueur ou PNJ » utilise le **rendu du client de jeu**, avec l'écran honnête et les **pilotes masqués** ; le **testeur** est une **session évaluatrice neuve** (sans accès au code, aux paramètres ni aux jugements précédents), qui juge chaque personnage observé et note ce qui l'a décidée ; ces notes alimentent le **catalogue des indices**. Le côté « joueur » est tenu par des **sessions joueuses** qui pilotent des comptes synthétiques (décidé, A8). Le calque de débogage y est désactivé. L'outil sert donc à des sessions, par une interface lisible par un agent (images du rendu et texte de l'écran honnête), sans action de l'utilisateur ; des testeurs humains d'une bêta pourront compléter, jamais être requis. *Pourquoi : l'évaluatrice doit voir exactement ce que voit un joueur, sinon le test mesure l'outil et non le jeu ; des sessions neuves et variées jugent gratuitement et de façon reproductible.*

---

## 13. De la 2.5D à la 3D

### 13.1 Ce qui ne change pas (décidé)

La vue du personnage et les actions possibles (§1), le HUD (§2), la sélection et le bilan (§3), les intuitions et le carnet (§4), la roue d'actes, la conversation et les menus d'actions (§5), le plan, le téléphone et le courrier (§7, §8), les écrans du solo, du compte et de l'en-ligne (§9, §10), l'accessibilité et les réglages (§11), la console (§12), le brouillard de perception et les icônes de son (§6.4).

### 13.2 Ce qui change

| Élément | En 2.5D | En 3D |
|---|---|---|
| Caméra | Orthographique, angle fixe, quarts de tour, zoom par crans | Troisième personne libre (vue à l'épaule), première personne en option |
| Bâtiments | Vue en coupe, étage affiché, murs abaissés | Pas de coupe : la caméra est dans le bâtiment ; les murs entre la caméra et le personnage deviennent transparents |
| Choix de l'étage | Consultation par le plan de mémoire (§6.3) | Retiré du jeu ; le plan de mémoire reste dans le **Plan** du téléphone (onglet « Bâtiment ») |
| Ciblage | Souris sur le monde, contour de la cible | Réticule au centre de l'écran, ciblage doux ; même contour |
| Faire face (combat) | Le personnage se tourne vers l'adversaire choisi | Identique, la caméra se place derrière l'épaule face à lui |
| Visée | Point du sol sous la souris | Réticule ; la dispersion envoyée par le serveur s'affiche autour |
| Bulles de dialogue | Au-dessus des têtes, toujours lisibles | Attachées aux têtes, avec réduction et repli dans le journal quand l'orateur est hors champ |
| Brouillard de perception | Pièces assombries | Naturel : on ne voit pas à travers les murs ; les icônes de son restent |
| Niveaux de détail de la ville (§6.5) | Complet, façade, volume | Mêmes données, modèles détaillés du kit, volumes fusionnés par secteur (HLOD), imposteurs pour le lointain (P33) |
| Conduite | Caméra éloignée qui anticipe | Caméra de poursuite, vue intérieure en option |
| Déplacement | Relatif à la caméra, clic pour aller | Relatif à la caméra ; clic pour aller retiré |

### 13.3 Les règles qui évitent de refaire l'interface (décidé)

1. Toute désignation produit une **entité ou un point du monde**, jamais une coordonnée d'écran (§0.6).
2. Les éléments attachés au monde passent par une **projection** unique, qui sait gérer le hors-champ et l'occultation dès la 2.5D.
3. Les commandes produisent des **intentions** ; seule la conversion entre l'entrée et l'intention dépend de la caméra.
4. Aucune information n'est **dessinée dans le décor** pour la seule vue de dessus (pas de marquage au sol indispensable qu'une vue à l'épaule ne verrait plus) ; la ligne d'itinéraire au sol reste une aide facultative.
5. Le brouillard de perception est fondé sur les **données reçues**, pas sur un effet de caméra.

*Pourquoi : ces cinq règles suffisent pour que le passage à la 3D ne touche que la caméra, la conversion des commandes et l'apparence, comme le prévoit le socle ([05-architecture.md](../05-architecture.md), §2.1).*

---

## 14. Points ouverts

Aucune question de conception propre à l'interface ne reste ouverte. Restent les valeurs **[à calibrer]**, à ajuster au prototype du client (étape 3 du périmètre initial, [05-architecture.md](../05-architecture.md), §5) par les sessions du responsable qualité et essais (sessions joueuses, sessions évaluatrices, mesures automatiques), les valeurs de jeu étant arrêtées par le responsable conception ; aucune ne demande d'essai ni de réglage à l'utilisateur (A8) :

| Sujet | Valeurs |
|---|---|
| HUD | Seuil d'alerte avant l'éjection, seuils des trois intensités de besoins, rythme des intuitions spontanées |
| Synchronisation | Fiabilité du signe de réticence selon l'indice, nombre de récidives avant rappel, cinq degrés de fidélité du bilan |
| Intuitions | Pondération synchronisation / familiarité de l'indice de précision, seuils des niveaux vague / clair / précis |
| Sélection | Taille de la sélection de départ, renouvellements par jour réel (5), délai d'acceptation d'une réservation aboutie |
| Réservation et incarnation | Fourchette d'expiration aléatoire d'une réservation (2 à 6 min), attente avant une nouvelle réservation du même personnage (10 min), seuil d'incarnation courte (10 min) et nombre toléré par jour (6), repos de l'IA (min(D/2, 4 h), min(D, 8 h), 8 h par jour), niveau de compte du hors catégorie clandestin (100) |
| Bilan | Durée du bilan provisoire (1 h de jeu), arrondis (5 points, 5 minutes), délai qui suit un acte dissonant grave (5 min de jeu) |
| Parole | Longueur maximale du texte libre (280 caractères), distribution du délai de formulation, liste du vocabulaire hors fiction, fourchette de réponse à une proposition intime |
| Compte et modération | Délai d'annulation des actions irréversibles (72 h), intervalle des réinitialisations (90 jours), débit des signalements, durée des mesures automatiques |
| Familiarité | Durée d'affichage des familiarités devenues inutiles |
| Passage de relais | Durée de la transition |
| Gestes automatiques | Durée de l'annonce |
| Naturel et humeur | Délai avant les gestes au repos d'un personnage piloté par un joueur, vitesse de rotation, seuil de l'intuition d'approche d'une crise, mots des signes d'humeur |
| Caméra | Angle, crans de zoom et leur échelle (≈ 50 px et ≈ 12 px par mètre en 1080 p, à fixer avec la direction artistique), recul en conduite, durée de consultation d'un autre étage |
| Rendu des couches | Teinte du liseré de la couche joueur, formes des bulles par ton, taille minimale du texte (≈ 18 px en 1080 p) |
| Accessibilité | Variation de luminance tolérée sous le plafond de 3 éclats par seconde, tables de couleurs des palettes pour daltoniens, liste des sons sous-titrés |
| Ville affichée | Distance du niveau complet (≈ 40 m), durée du fondu entre niveaux, plafond de la création d'un monde solo (10 s, au plus 20 s) |
| Réticence | Niveaux affichés selon la précision, nombre de raisons au survol, seuils de M8 (0,01, 0,05, 0,20), jointures des raisons par langue, durée du coup d'œil sur une cible désignée, durée avant qu'un suivi soit reconnu |
| Tenue, coiffure, radio | Durée du changement de tenue, coupes possibles à la maison |
| Écoute et achats | Formes de bulle par volume, montants proposés au chapeau, plafond du montant libre au chapeau et du pourboire (≈ 20 écus) |
| Modération | Délai de contestation, longueur du champ de texte de contestation, liste des motifs |
| Combat | Intensité des effets physiologiques, taille du réticule selon la dispersion, seuils des mots de l'état du chargeur, plafond du délai de déconnexion sous menace (≈ 5 min), nombre de tirages à la création, nombre de crans d'attribut et taille du budget |
| Carnet | Nombre de nouvelles affichées, seuils des cinq degrés de capacités |
| Téléphone | Autonomie de la batterie |
| Solo | Intervalle de sauvegarde, nombre de sauvegardes de secours et d'emplacements de monde, préavis d'interruption avant un engagement |
| Création | Plage d'âge du personnage persistant, liste des noms interdits, curseurs de haut niveau du visage et leur regroupement des paramètres du génome |
| En ligne | Patience des PNJ qui attendent une réponse |
| Commandes | Disposition par défaut (rotation sur C / V depuis la v0.7.1), disposition de combat (§5.6.2) |
| Langues | Langues de lancement |

---

## 15. Impacts sur le socle et les autres domaines

À trancher par le responsable conception (ou, pour `05-architecture.md`, le responsable architecture et sécurité à partir de la phase 0) ; rien n'a été modifié ailleurs.

### 15.1 Socle

Les lignes « v0.3 » reprennent ce que les revues d'équilibrage et de sécurité changent à l'interface ; plusieurs figurent déjà dans le socle mis à jour par la session principale, qui reste seule à le modifier.

| Fichier | Impact | État |
|---|---|---|
| [05-architecture.md](../05-architecture.md), §1 et §2.5 | **Vue du personnage filtrée par le serveur** : le client ne reçoit que les entités perçues, les connaissances non opaques du personnage piloté et la liste des **actions possibles** ; jamais les composants internes des autres (§1.1). Le brouillard de perception en découle (§6.4) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.16 et §4 | **Console d'administration web** séparée du client, avec rôles et journal des actions ; double validation pour annuler des écritures du grand livre (§12.1) | Intégré au socle le 29/10 ; son langage sera fixé par la revue d'architecture |
| [05-architecture.md](../05-architecture.md), §4 | **Vue de conception** (données internes) réservée aux versions et serveurs de développement (§12.3) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 | **Service d'itinéraire** côté serveur, fondé sur les horaires publiés et les perturbations annoncées (§7.2) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 | Solo : **une seule chronologie par monde** (sauvegarde automatique, sauvegardes de secours seulement après une panne), **plusieurs mondes solo** partageant le niveau solo ; le téléphone, le plan et le carnet ne mettent pas le monde en pause, sauf option d'accessibilité en solo (§9.2, §9.3, §11.1) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 | Précision d'affichage : barres empilées avec l'écart hachuré, sans chiffres par défaut, **alerte avant l'éjection** (§2.2) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1.1 | Le **bilan** n'affiche ni le multiplicateur de palier ni la valeur de chaque objectif, et donne la fidélité en mots ; aucun compteur d'expérience pendant l'incarnation (§3.8) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §6 | Familiarité révélée en **quatre étapes** (description, nom, traits, jouabilité) ; jouabilité donnée avec la **nature** de l'obstacle (expérience, connaissance, profil, hors catégorie) sans quantité ; indisponibilité unique pour « pas au repos », « piloté par un autre joueur » et « état grave » ; « Absent d'Orvenne » pour l'extérieur (§3.4, §3.5) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §6 et [01-concept.md](../01-concept.md), §1.1 | Le **personnage persistant d'un autre joueur** est présenté dans les connaissances comme un habitant, toujours « Indisponible » ou « Absent », pour ne pas révéler qu'un joueur le pilote (§3.4) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §2 | Le joueur voit son propre profil en **portrait en phrases**. Proposition initiale : sans noms de catégories | Intégré au socle le 29/10 **avec correction** : le portrait est accompagné des **noms de ses catégories**, sans valeur ; appliqué au §3.9 en v0.2 |
| [02-personnages.md](../02-personnages.md), §1.1 et §2 | **Création du personnage persistant** : apparence, nom, âge. Proposition initiale : attributs proches de la moyenne, sans choix | Intégré au socle le 29/10 **avec correction** : attributs par **budget de points** ou **tirage aléatoire**, dans des bornes réalistes modulées par l'âge ; écran de création adapté au §3.2 en v0.2 |
| [02-personnages.md](../02-personnages.md), §2 | Les **compétences** de départ du personnage persistant ne sont pas précisées par le socle (seuls les attributs le sont). Recommandation : des compétences de base selon l'âge (conduite, cuisine, usage du téléphone), sans compétence de métier, qu'il acquiert en jouant (§3.2) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §1.1 | **Tirage au sort à budget égal** (E34) ; noms **normalisés** (NFKC), sans caractères invisibles ni confondables avec les noms du personnel (S31) (§3.2) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 | **Point de rupture** : signe de réticence toujours visible, renoncement et éjection ; acte extrême non proposé (E1). **Quitter sous l'alerte** vaut éjection, et le message d'alerte le dit (E2) (§2.2, §2.3) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1.1 | Bilan **provisoire pendant 1 h de jeu** après toute incarnation, complicité différée (E4) ; expérience arrondie à 5, temps actif à 5 min (E26) ; incarnation de moins de 10 min sans gain, attente de 10 min après 6 incarnations courtes par jour (E14) ; nouvelles pénalités affichées en clair (E3) (§3.6, §3.7, §3.8) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §6 | **Aucun état de disponibilité affiché en continu** ; on l'apprend en demandant à incarner, par une réservation qui expire sans motif après un délai aléatoire ; persistant d'un autre joueur toujours en réservation, connecté ou non ; repos de l'IA (E17) ; une réservation active par compte, 10 min avant de réserver à nouveau le même personnage (E30) ; hors catégorie clandestin « Il vous faudrait plus d'expérience » jusqu'au niveau 100 (E26) ; sélection renouvelable 5 fois par jour réel (E14) (§3.3 à §3.5) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1 (compte) | Réinitialisation une fois tous les 90 jours en ligne, sans effet sur la note de conduite (E23) ; actions irréversibles avec réauthentification et 72 h d'annulation, appareils connectés, TOTP (S13) ; rapprochement de comptes dans l'export et contestable (E8) ; succès de plateforme cosmétiques, jamais liés au solo (S24) (§3.9) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 | Solo : **reprise exacte** par rejeu du journal après une fermeture brutale ; sauvegardes de secours seulement en cas de corruption, qui restaurent aussi l'expérience solo de ce monde (E32) (§9.2) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 | Liste des actions **crues possibles** (admissibilité épistémique), garde hors fiction pour les pilotes `Joueur`, échec visible à l'exécution, refus par **quatre codes fermés** (S4, E35, A2) ; vue **sans marge**, recalculée à chaque tick pour un pilote joueur, apparence perçue, poignées renouvelées (S5, S6) ; sons floutés et bribes découpées par le serveur (S5) ; une session par compte et par monde (S11) (§1.1, §1.4, §6.4, §10.5) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 (point 20) | Console : pilotes visibles seulement dans la capture d'un signalement traité ; ni prise de pilote ni écriture d'état ; inspection sur motif ; rôle concepteur sans donnée individuelle ; réseau privé, FIDO2, audit chaîné copié hors de portée ; rapprochement de comptes (S21, E8) (§12.1) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 (point 16) | Signalements limités et pondérés ; mesures automatiques seulement légères et temporaires, les autres décidées sur dossier par une session de modération distincte de la détection (S36, A8) (§10.3, §10.4) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §1.1 ; [05-architecture.md](../05-architecture.md), §2.5, §2.9 et §3 | Éditeur d'apparence limité à l'espace de la population, sans objet propre aux joueurs ; apparence perçue graduée ; aucune parole dite en première version, voix synthétisées plus tard dans le client de chaque auditeur ; rendu par catalogue de correspondances (v0.6, DA §3.5, §3.7, §5.5) (§3.2, §4.7, §5.4) | Intégré au socle le 29/10 (demandes de la direction artistique) ; appliqué ici en v0.6 |
| [05-architecture.md](../05-architecture.md), §2.5 | **Portraits** : le serveur envoie, pour un écran de portraits, l'apparence reconstituée à une **date** (dernière perception ou dernier contact) et à une **précision**, sans stocker d'image ; il garde seulement la date et la précision par connaissance (v0.6, §4.7) ; **revu en v0.9** : trois dates par relation (perception identifiée à toute précision, à « tenue », à « visage »), mises à jour seulement par une identification (architecture C-5, sécurité C1) | Intégré au socle le 29/10 par le responsable conception ; révision de clôture déjà au socle (05 §2.5) selon le responsable conception |

### 15.2 Domaine IA des PNJ ([ia-pnj.md](ia-pnj.md))

- Les **intuitions** sont produites sous forme structurée (catégorie, sujet, intensité, niveau de précision), avec un indice de précision combinant synchronisation et familiarité ; la touche **Réfléchir** demande les trois plus fortes (§4.1).
- **Ce qu'il dirait** : le cœur fournit les trois actes que l'IA choisirait elle-même dans la conversation en cours (§5.4).
- **Signe de réticence** : pour chaque action possible proposée au joueur, le cœur évalue à l'avance l'écart entre sa signature et le profil du personnage (§2.3). Depuis la v0.5, il le transmet sous la forme d'un **niveau** (0 à 3, seuils de M8 : 0,01, 0,05 et 0,20), d'un **indicateur de rupture** à part et d'un **code de raison** (corrigé en v0.7 : la v0.5 disait « 0 à 4 », en comptant la rupture comme cinquième état), jamais d'un coût (G24) ; le niveau affiché selon la précision des intuitions est un choix de l'interface (§2.3).
- **Codes de raison** (v0.5, signatures §5.3) : un catalogue fermé de raisons et de circonstances atténuantes, chacune avec ses gabarits par degré de précision, partagé avec les raisons du bilan (§3.8, §4.1).
- Les **gestes automatiques** sont annoncés environ 2 secondes avant de commencer, et un réglage peut les désactiver (§5.3).
- **Interruptions** de l'avance du temps en solo : événements qui concernent le personnage (§9.1).
- **Plan de mémoire** : chaque personnage mémorise les **étages** des bâtiments qu'il a vus (granularité bâtiment × étage, pas pièce par pièce), pour la consultation des étages (§6.3).
- Les **capacités** du personnage incarné sont exprimées en cinq degrés (§4.2).
- **Actions crues possibles** (v0.3, S4) : la liste des actions, la roue et les masques d'actions sont calculés sur les croyances et la perception du personnage ; un échec à l'exécution produit un effet visible et une connaissance nouvelle (§1.4).
- **Coût estimé** de chaque intention d'un pilote `Joueur` pour le signe de rupture, y compris sur la barre de combat (v0.3, E1, §2.3, §5.6.2).
- **Délai de réponse** d'un PNJ à une proposition intime tiré dans une fourchette qui couvre celle d'un joueur (v0.3, A6, §5.4).
- **Repos de l'IA** après une longue incarnation d'un personnage de palier 5 ou plus (v0.3, E17, §3.5).
- **Naturel des personnages pilotés par un joueur** (v0.4, IA §13.7) : regard, allure, gestes au repos et imperfections corporelles sont tirés par le cœur pour eux aussi, sur le flux d'aléa du naturel ; les gestes au repos ne démarrent qu'après quelques secondes sans commande et s'interrompent à la première ; les hésitations et les imperfections qui sont des choix restent au joueur (§4.6).
- **Humeur et crises d'un personnage incarné** (v0.4, IA §13.2) : le cœur fournit l'humeur avec sa cause pour les intuitions ; à l'approche d'une crise, il produit une intuition pressante et l'acte « Se laisser aller » correspondant à la recette du profil, qui soulage comme une crise de l'IA (§4.1). À confirmer : ne pas suivre cet appel n'a pas de signature, l'humeur dure simplement.
- **Intuitions** de deux nouvelles catégories, « état d'âme » (pensées, humeur, thèmes de vie) et « expérience » (conclusions, valeurs apprises), avec les trois niveaux de précision (v0.4, IA §13.4, §13.6 ; §4.1, §4.2).
- **Signes visibles de l'humeur** (posture, démarche, expression) et des crises, pour tous les personnages ; l'infobulle les traduit en mots selon ce que devine l'observateur (v0.4, IA §13.2 ; §4.5).
- **Classe de cible et codes associés** (v0.7, signatures-contenu §9.1) : avec chaque code de raison, le cœur envoie la **classe** de la cible, l'entité visée quand le personnage la connaît, et les codes fermés de groupe, d'infraction et de besoin qui remplissent les variables des gabarits (§4.1).
- **Actes excentriques** (v0.7, A20 à A28) : le cœur reconnaît un **suivi** fait à la main et l'**entrée chez autrui** par le franchissement d'un seuil, et envoie leur signe **avant** que l'acte compte ; désigner une personne n'est jamais « dévisager » (§5.2).
- **Tenue, coiffure, radio** (v0.7, ia-pnj §2.6) : actions « Se changer », « Prendre le parapluie », choix de la coupe au salon, « Allumer la radio », « Changer de station », « Volume » pour un personnage piloté ; intuitions de tenue, de coupe et de goûts musicaux ; à confirmer : une station loin des goûts ne fait qu'une petite pensée, sans signature (§5.3 bis).
- **Vocalisation d'une réplique** (v0.6, DA §5.5) : choisie par le serveur d'après le ton de l'acte et la voix du génome, pour tout orateur, jamais par le joueur (§5.4).
- **Date du dernier contact** et précision la plus haute de l'apparence vue, par relation, pour le portrait des proches (v0.6, §4.7).
- **Registre d'un acte de la roue** : poli ou familier, choisi par le joueur, avec pour défaut le registre attendu dans la relation nuancé par la voix du personnage ; les valeurs sec, agressif et séducteur passent par le ton (v0.4.1, social §3.3 ; §5.4).

### 15.3 Domaine social ([social.md](social.md))

- **L'orateur voit toujours son propre texte libre tel qu'il l'a tapé**, et la carte de vérification ne montre jamais la reformulation ; sinon il pourrait déduire le pilote de son interlocuteur (§5.4). Précision à reporter au §3.3 et au §9.2.
- **Bloquer** et **Signaler** sont proposés sur **tout personnage**, avec une réponse identique ; sur un PNJ, le signalement porte sur la formulation du modèle (§10.3). L'effet du blocage sur un PNJ est revu en v0.3 (ci-dessous).
- **Ajout d'amis** par pseudonyme ou code seulement, jamais depuis un personnage ; amis consentants visibles avec leur quartier et leur position sur le plan (§10.1). Extension du §9.1 à confirmer.
- L'onglet **Nouvelles** affiche les croyances récentes avec origine et certitude en mots (§4.4).
- **Historique des messages** du téléphone formulé à la demande à partir des faits structurés, dans les quotas (§8.1).
- **Traduction** de la parole libre entre joueurs de langues différentes (v0.3, A6), qui **remplace** la décision « pas de traduction » du §9.2 de social (§5.4, §11.2).
- **Même message, même délai** pour la parole d'un joueur et une réplique de PNJ ; **filtre hors fiction** et retrait des émoticônes sur toute parole libre ; 280 caractères au plus ; texte au classement refusé diffusé à personne (v0.3, A4, E26, S18 ; §5.4). L'orateur voit les remplacements du filtre hors fiction, mais jamais la reformulation, la traduction ni le masquage des données personnelles.
- **Blocage identique sur un PNJ et sur un joueur** (v0.3, A6) : la parole du personnage bloqué passe par le repli pour le bloqueur, quel que soit son pilote ; quand le compte bloqué parle au bloqueur par un autre personnage, sa parole libre est remplacée par la formulation ordinaire de ses actes, jamais par le repli. Modifie le tableau des outils du §9.4 de social (« sur un PNJ, enregistré sans effet ») (§10.3).
- **Actes intimes** : même parcours quel que soit le pilote ; demande d'acceptation chez tout personnage piloté par un joueur qui reçoit la proposition, d'un PNJ comme d'un joueur ; un refus du joueur prend la forme d'un refus poli ordinaire (v0.3, A6 ; §5.4). Modifie le §9.4 de social.
- **Témoignage** d'un personnage incarné limité aux faits de sa mémoire, aveu sur un fait inconnu traité en affirmation sans fondement (v0.3, E8 ; §4.1).
- PNJ **patients** quand un joueur tarde à répondre en ligne, avant de s'impatienter (§11.1).
- **Registre** poli ou familier, choisi à côté du ton dans la roue et affiché sur la carte de vérification, confirmé ou corrigé par le joueur ; le registre confirmé est celui que reçoit l'auditeur (v0.4, revu en v0.4.1 pour suivre social §3.3 ; §5.4). Demande du §12.10 de social traitée.
- **Rappel de la version donnée** d'un fait sensible et **usages d'une pratique** en cours en intuitions (v0.4.1 ; §4.1). Demande du §12.10 de social traitée.
- **Signatures des gestes** (v0.4, J§7) : **courir sans raison apparente** en public (légère dissonance pour un personnage calme, perçue par les passants) et **fouiller** (poids selon le lieu et le propriétaire, réaction des témoins) sont des actes à signature ; les raisons qui ôtent la signature à la course (retard, danger, transport à attraper, tenue de sport) sont à fixer avec ia-pnj (§5.2).

### 15.4 Domaine économie ([economie.md](economie.md))

Les demandes du §14.7 sont traitées : solde, espèces, relevés et échéances (§2.6, §8.2) ; tableau des missions, annonces, offres d'emploi (§8.1, §8.3) ; courrier (§8.4) ; prix affichés et ticket (§8.5) ; intuition d'impossibilité entre personnages liés (§4.1) ; modération et tableau de bord (§12). En retour :

- les **articles** et les **services** portent un prix d'affichage lisible par le client (étiquette, carte, tarif) ;
- la **fiabilité** des missions est exprimée en mots (§8.3) ;
- les **demandes de prêt** se font en agence ; l'application ne fait que des simulations (§8.2) ;
- le **plan papier** est un article vendu au tabac-presse (§7.1) ;
- le **journal local** (entreprise de presse de la ville) publie les avis de décès, naissances, ouvertures et fermetures (§8.1, §8.6) : à prévoir comme contenu produit par l'activité de cette entreprise ;
- **vigilance bancaire** (v0.3, E8) : la banque soumet aussi à confirmation, avec exécution le lendemain, une petite part tirée au hasard des virements **inhabituels** entre 2 000 et 5 000 écus **[à calibrer]**, pour que la confirmation imposée aux flux vers le cercle associé ne trahisse pas un rapprochement de comptes (§8.2).

### 15.5 Domaine population ([population.md](population.md))

Les demandes du §12.7 sont traitées : ce que connaît un nouveau venu (§4.3, §7.1) ; événements de vie rendus perceptibles (§8.6) ; familiarité devenue inutile (§3.4). En retour : les **faire-part**, **avis de décès** et **annonces de naissance** sont des objets de courrier et des contenus du journal local, à émettre lors des événements correspondants.

Demandes de la v0.6 (direction artistique) :

- **Éditeur d'apparence** (§3.2) : exposer les **plages** du génome par âge et par sexe, et la **distribution** utilisée par les boutons « Autre visage » et « Tirer au hasard » ; la musculature du persistant découle de sa force et de son endurance comme pour les habitants ; toute combinaison offerte par l'éditeur a une probabilité non nulle dans la population (DA §3.7, §12.2).
- **Historique daté** de la coiffure, de la tenue habituelle et des signes distinctifs (cicatrice, tatouage, lunettes) de chaque habitant, pour reconstituer un portrait à une date (§4.7).

### 15.6 Domaine ville ([ville.md](ville.md))

Les demandes du §11.2 sont traitées : vue en coupe, étage −1 et toits (§6.2, §6.3) ; carte et adresses (§7) ; connaissance de la ville par le personnage (§7.1) ; lieux de retrait et délai (§3.7). En retour :

- le registre des lieux distingue ce qui figure au **plan public** (établissements ouverts au public, enseigne, horaires affichés) de ce qui est privé (§7.1) ;
- **plaques de rue**, numéros, panneaux de direction, **panneaux d'horaires** et écrans de prochains passages font partie des kits modulaires (§7.2, §7.3) ;
- la **propagation du son** à travers murs et planchers (atténuation) est nécessaire aux icônes de son (§6.4) ;
- éléments de décor des événements de vie : cartons visibles, camion, rideau baissé, affiche de fermeture, boîte aux lettres qui déborde (§8.4, §8.6) ;
- **adresses à la française** et **noms de rues par thème** (P36) affichés tels quels sur les plaques, le plan et la recherche (§7.2) ; la recherche demande une forme normalisée des adresses (sans casse, virgules ni accents) fournie par le registre public des lieux ;
- **carte par secteur et par couche** et trois niveaux de détail (P32, P33) : le découpage en secteurs et les volumes simples par type de bâtiment sont fournis par la génération (§6.5).

### 15.7 Domaine combat, police et justice ([combat.md](combat.md), v0.1)

Les demandes du §16.7 de combat sont traitées : saisie des intentions de combat et effets physiologiques sans chiffre (§5.6.2 à §5.6.4) ; blessures visibles et ressenties (§5.6.5) ; courrier judiciaire (§8.4), casier du personnage persistant (§4.2, onglet « Justice »), zones interdites sur le plan (§7.1) ; délai de déconnexion prolongé tant que le personnage est menacé (§3.7) ; intuition de récusation (§4.1, §5.6.9) ; réglage « atténuer la violence » (§11.1). Les listes de combat (actions et conditions, zones et états de blessure, conséquences équivalentes, effractions et véhicules) remplissent les emplacements du §5.6. En retour :

- **Rendre le personnage en plein affrontement** : résolu en v0.3 par le **bilan provisoire d'une heure** sur toute fin d'incarnation (E4), qui absorbe la règle des 10 minutes du §11.5 de combat, quelle que soit la façon de partir (§3.7).
- **Récusation** : l'intuition devient « Vous êtes trop proche de cette affaire. », celle du conflit d'intérêts élargi au cercle du joueur (v0.3, E8 ; §4.1, §5.6.9).
- **Point de rupture en combat** : le signe s'affiche sur l'intention d'attaque dès que le personnage fait face à la cible ; frapper quand même fait retenir le geste et éjecte le joueur (v0.3, E1 ; §5.6.2).
- **Lieu de retrait sous menace** : le message « ne peut pas partir tant qu'il est menacé » s'affiche aussi derrière la porte ; un véhicule qui n'est pas le sien reste à la sortie (v0.3, E19 ; §3.7).
- **Faire face** remplace tout verrouillage : la touche de ciblage oriente le personnage sans guider les coups, conformément au §1.2 de combat (§5.6.2).
- **Vision en tunnel** : le serveur réduit réellement la perception latérale envoyée au client (§5.6.4).
- **Munitions** affichées en mots (plein, entamé, presque vide, vide) et chargeurs de rechange comptés (§5.6.3).
- **Pouvoirs de police** proposés même hors des conditions légales, avec une intuition quand une condition manque d'après ce que sait le policier (§5.6.9).
- **Effractions** : les actions s'affichent même sans la compétence (l'échec l'apprend), sauf crocheter et démarrer sans clé, qui demandent leurs outils (§5.6.7).
- **Appel d'urgence** : actes propres de la roue (nature, lieu, blessés) et bouton utilisable téléphone verrouillé (§5.6.6).
- **Perte de connaissance** d'un personnage incarné : le joueur peut attendre le réveil ou rendre le personnage ; en état critique, il ne peut que le rendre (§5.6.4).
- **Juge et avocat** incarnés : panneaux propres (fourchette de peine, proposition de plaider coupable) à préciser ensemble au prototype, par les sessions interface et combat (§5.6.9).

### 15.8 Service de dialogue

- Formulation dans la **langue du dialogue** de chaque joueur auditeur (§11.2).
- **Classement** du texte libre dans n'importe quelle langue supportée, avec un délai compatible avec la carte de vérification (§5.4).
- Formulation à la demande des **historiques de messages** du téléphone (§8.1).
- File de **signalements de formulation** pour corriger filtres et consignes (§10.3, §12.1).
- **Traduction fidèle** (sans reformulation) de la parole libre d'un joueur vers la langue de dialogue d'un auditeur joueur, dans la même distribution de délai que la formulation (v0.3, A6 ; §5.4).
- **Classement limité aux actions crues possibles**, refus de classement signalé au client sans diffusion du texte (v0.3, S18 ; §5.4).
- **Filtre hors fiction** et retrait des émoticônes avant toute diffusion (v0.3, E26, A4 ; §5.4).
- **Quota solo** du point d'accès hébergé exposé au client en trois niveaux (beaucoup, peu, plus aucune), jamais en nombre exact (v0.3, S19 ; §11.3).
- **Registre** rendu par le classement du texte libre, et registre **corrigé** par le joueur pris comme entrée de la formulation (v0.4, IA §13.10 ; §5.4).

### 15.9 Intégration des revues d'équilibrage et de sécurité (v0.3)

Correspondance entre les décisions des revues ([equilibrage.md](../revues/equilibrage.md) §6.10, [securite.md](../revues/securite.md) §9.9), les arbitrages de la session principale et les sections de ce fichier :

| Renvoi | Objet | Section |
|---|---|---|
| E1 | Point de rupture : signe toujours visible, renoncement, éjection ; acte extrême non proposé ; intuition « Il ne peut pas s'y résoudre. » | §2.3, §4.1, §5.6.2 |
| E2 | Quitter sous l'alerte vaut éjection ; message d'alerte complété | §2.2, §3.7 |
| E3 | Désynchronisation grave et récidive affichées dans le bilan | §3.8 |
| E4 | Bilan provisoire pendant 1 h de jeu, complicité différée | §3.7, §3.8 |
| E8 | Cercle et conflit d'intérêts (« Vous êtes trop proche de cette affaire. »), témoignage, vigilance bancaire, rapprochement de comptes | §3.9, §4.1, §5.6.9, §8.2, §12.1 |
| E9 | Vote opaque (« Votre vote n'appartient qu'à vous. ») | §4.1 |
| E10 | Plafonds du persistant (« Ce serait trop pour vous. ») | §4.1 |
| E14 | Sélection renouvelable 5 fois par jour réel ; incarnations courtes | §3.3, §3.6, §3.8 |
| E17 | Repos de l'IA parmi les réservations | §3.5 |
| E19 | Menace dans un lieu de retrait ; véhicule d'autrui laissé à la sortie | §3.7 |
| E23 | Réinitialisation tous les 90 jours, note de conduite conservée | §3.9 |
| E26 | Hors catégorie clandestin ; arrondis du bilan ; aucune disponibilité en continu ; filtre hors fiction et délai de formulation | §3.4, §3.5, §3.8, §5.4 |
| E30 | Une réservation par compte, 10 min avant d'en refaire une | §3.5 |
| E32 | Reprise exacte par rejeu du journal ; sauvegardes de secours en cas de corruption | §9.2 |
| E34 | Tirage à budget égal | §3.2 |
| E35 | Garde hors fiction dans la liste des actions | §1.1, §1.4 |
| S4 | Actions crues possibles, échec visible, codes de refus fermés | §1.1, §1.4, §5.4 |
| S5 | Aucune marge de perception, apparence perçue, sons floutés et bribes découpées par le serveur | §1.1, §6.4 |
| S6 | Poignées renouvelées, continuité par les connaissances du personnage | §1.1 |
| S11 | Une session par compte et par monde | §10.5 |
| S13 | Réauthentification et 72 h d'annulation, appareils connectés, TOTP | §3.9 |
| S18 | 280 caractères, texte au classement refusé non diffusé, classement limité aux actions possibles | §5.4 |
| S19 | Point d'accès solo et quota quotidien en mots | §11.3 |
| S21 | Console : pilotes dans les captures seulement, aucune prise de pilote ni écriture, inspection sur motif, réseau privé, FIDO2, audit | §12.1 |
| S24 | Succès de plateforme cosmétiques, jamais liés au solo | §3.9 |
| S31 | Noms normalisés, sans invisibles ni confondables | §3.2 |
| S36 | Signalements limités et pondérés ; sanctions lourdes décidées sur dossier par une session de modération (A8) | §10.3, §10.4 |
| A2 | Liste des actions épistémique et garde hors fiction ; intuitions de la garde ; codes fermés | §1.4, §2.3, §4.1 |
| A4 | Même message et même délai ; filtre hors fiction ; 280 caractères | §5.4 |
| A6 | Blocage identique ; traduction de la parole des joueurs ; actes intimes au même parcours ; « Indisponible » plus affiché en continu | §0, §3.4, §3.5, §5.4, §10.3, §11.2 |
| Compte | Réinitialisation (E23, S13), rapprochement dans l'export (E8), succès de plateforme (S24) | §3.9, §12.1 |

### 15.10 Intégration de la revue de l'intelligence des PNJ (v0.4)

Correspondance entre la ligne « interface.md » du tableau « Revue de l'intelligence (v0.8) » du §11 de [ia-pnj.md](ia-pnj.md), la ligne « 04-joueur.md et interface.md », la décision de la session principale sur les gestes ([04-joueur.md](../04-joueur.md), §7) et les sections de ce fichier :

| Renvoi | Objet | Section |
|---|---|---|
| IA §13.2 | Humeur sans jauge, dite avec sa cause ; signes visibles de l'humeur et des crises (posture, démarche, expression) chez tous ; approche d'une crise et « Se laisser aller » | §2.4, §4.1, §4.2, §4.5 |
| IA §13.4, §13.6 | Intuitions nourries par les pensées, les thèmes de vie, les conclusions et les valeurs apprises ; thèmes de vie dans le carnet | §4.1, §4.2 |
| IA §13.7 | Naturel des corps (gestes au repos, regard, hésitations, allure, délais, imperfections) appliqué aussi aux personnages pilotés par un joueur ; rotation humaine | §4.6 |
| IA §13.10, social §3.3 | Registre poli ou familier, choisi à côté du ton dans la roue, affiché, confirmé ou corrigé sur la carte du texte libre (v0.4.1) | §5.4 |
| Social §4.7, §6.1 | Intuitions de la version déjà donnée d'un fait sensible et des usages d'une pratique en cours (v0.4.1) | §4.1 |
| IA §14.3 | Relecture des tests à l'aveugle avec le rendu du client et les pilotes masqués | §12.3 |
| J§7 | Aucun geste gratuit (ni saut, ni geste libre, ni pivot sur place, ni accroupissement libre) ; franchir et saluer en actions de contexte ; course sans raison apparente à signature ; fouille cible par cible, à signature, vue des témoins | §5.1, §5.2, §5.7, §4.6 |

### 15.11 Intégration des revues des signatures et de la génération (v0.5)

Correspondance entre [signatures.md](../revues/signatures.md) §9.10, [generation.md](../revues/generation.md) §13.9 et les sections de ce fichier ; le socle est déjà à jour selon la session principale.

| Renvoi | Objet | Section |
|---|---|---|
| G24 | Réticence en quatre niveaux (aucun, léger, net, fort) plus la rupture ; le client ne reçoit qu'un niveau, un indicateur de rupture et un code de raison, jamais un coût | §2.3 |
| Signatures §5.3 | Phrase de raison au survol à partir de la précision « claire » ; raison du retour immédiat ; raison de la rupture à la précision « précise » | §2.3, §4.1 |
| Signatures §5.3, §5.4 | Gabarits d'explication par degré de précision ; aucune croyance d'autrui ni raison de témoin | §4.1 |
| G25 | Bilan : « Ce qui lui a coûté » (au plus trois raisons) et « Ce qui l'excusait », en mots | §3.8 |
| P32 | Carte lue par secteur et par couche ; mobilier et objets par la vue ; changements du monde par secteur ; création d'un monde solo avec écran de progression, ≤ 10 s | §6.5, §9.2 |
| P33 | Niveaux complet, façade et volume ; volumes fusionnés par secteur en 3D | §6.5, §13.2 |
| P36 | Adresses à la française (impairs à gauche, pairs à droite, bis) et noms de rues par thème, sans personne réelle | §7.2 |

### 15.12 Domaine direction artistique ([direction-artistique.md](direction-artistique.md)) (v0.6)

**État** : les demandes du §12.4 de direction-artistique (v0.1.1) sont **traitées en v0.6** (29/10/2026). Le socle correspondant (02 §1.1, 05 §2.5, §2.9 et §3) était déjà à jour selon le responsable conception.

| Demande (DA §12.4) | Traitement | Section |
|---|---|---|
| Styles des deux couches (DA §4.3) | Polices, cadres et coins distincts ; liseré de la couche joueur jamais employé dans le monde, vérifié automatiquement ; différence jamais par la couleur seule ; sons d'interface distincts | §2.7 |
| Bulles et icônes de son (DA §4.4) | Bulles de taille fixe, ton par la forme et un mot, identiques quel que soit le pilote ; bulle repliée au bord pour un orateur hors champ ; icônes monochromes au flou du serveur ; indicateurs de danger fléchés à pulsation lente | §2.8, §6.4 |
| Typographie (DA §4.6) | Inter pour la couche monde, IBM Plex Sans pour la couche joueur, Atkinson Hyperlegible et OpenDyslexic en options ; deux graisses, taille minimale | §2.8 |
| Éditeur d'apparence (DA §3.7) | Mêmes plages et même catalogue que la population ; musculature tirée des attributs ; visage par curseurs de haut niveau et tirages ; aucun objet propre aux joueurs | §3.2 |
| Portraits (DA §3.5) | Rendus en jeu à partir de l'apparence connue, en trois précisions, sans visage neutre ; reconstitués à la date de la dernière perception ou du dernier contact | §4.7 |
| Accessibilité (DA §4.5, §5.7) | Plafond de 3 éclats par seconde toujours actif, option « Supprimer les flashs » dès le premier lancement ; palettes pour daltoniens sur l'interface et les signaux du monde ; sous-titres des sons ; son mono ; plage dynamique réduite | §11.1 |
| Réglages (DA §5.4, §5.5) | Quatre canaux de volume ; « Musique d'accompagnement » en jeu, désactivée par défaut, selon l'heure et la météo seulement ; aucune parole dite en première version, vocalisations ; plus tard « Voix synthétisées », au même délai que les bulles | §5.4, §11.3 |
| Crans de zoom (DA §1.2) | Zoom par crans seulement ; ≈ 50 px et ≈ 12 px par mètre ; tout changement des crans signalé à la direction artistique avant d'être appliqué | §6.1, §14 |

En retour, pour la direction artistique :

- **Formes de bulle** par **volume** (chuchoté, normal, élevé, crié) et **icônes de ton** (amical, neutre, sec, menaçant, séducteur) ; corrigé en v0.8 : la v0.6 demandait une forme pour le ton sec (§2.8).
- **Icônes de son** : ajouter **sirène** et **klaxon** au jeu de pictogrammes (§2.8).
- **Sous-titres des sons** : chaque son du catalogue qui a une icône, et chaque son visible qui compte, porte une **clé de sous-titre** traduite (§11.1).
- **Portraits** : un cadrage et une lumière neutres de portrait, une silhouette en contre-jour et un visage dans l'ombre pour les précisions « silhouette » et « tenue » ; jamais de visage neutre en gros plan (§4.7).
- **Musique** : une pièce unique pour le passage de relais, indépendante du personnage ; des pièces de bilan qui ne dépendent que de ce que le bilan affiche (§11.3).
- **Liseré de la couche joueur** : teinte contrôlée contre la palette du monde et ses transpositions pour daltoniens (§2.7).

### 15.13 Intégration du contenu des signatures et de l'IA des PNJ v1.1 (v0.7)

**État** : les demandes du §11.7 de [signatures-contenu.md](../revues/signatures-contenu.md) (v1.0.1, avec l'arbitrage A7) et celles du §2.6 et du §11 de [ia-pnj.md](ia-pnj.md) (v1.1) qui touchent l'interface sont **traitées en v0.7** (29/10/2026). La ligne de socle des portraits (v0.6) est intégrée par le responsable conception dans 05 §2.5. La règle 6 du README (aucune renumérotation) est appliquée : la nouvelle section s'appelle §5.3 bis.

| Renvoi | Objet | Section |
|---|---|---|
| M8 | Réticence en niveaux 0 à 3 (< 0,01 ; < 0,05 ; < 0,20 ; ≤ 1) et indicateur de rupture à part, qui remplace le niveau ; « 0 à 4 » corrigé | §2.3, §15.2 |
| Signatures-contenu §9 | Gabarits en trois formes (claire, précise, nominale) ; variables `{cible}`, `{à_cible}`, `{de_cible}`, `{Cible}`, `{groupe}`, `{infraction}`, `{besoin}`, `{e}` ; cible inconnue désignée par sa classe ; codes fermés seulement | §4.1 |
| A20 à A24, A26 à A28 ; A7 | Actes excentriques en actions de contexte ou en déplacements, avec leur signe ; le regard ne dévisage pas ; signe annoncé avant un acte fait par le déplacement ; aucun geste libre | §5.2 |
| IA §2.6, §11 | Tenue choisie dans la garde-robe là où sont les vêtements, jamais changée par l'IA pendant l'incarnation ; coupe choisie au salon (« Comme d'habitude » par défaut) ; radio allumée, station et volume choisis | §5.3 bis |

En retour :

- **Revue signatures-contenu** : les gabarits doivent exister pour chaque langue de lancement, avec les prépositions contractées de `{à_cible}` et `{de_cible}` ; la jointure « , mais » est à écrire par langue (§4.1).
- **Combat** : A23 (entrer chez autrui) attend la violation de domicile au catalogue des infractions (signatures-contenu §11.3) ; l'interface l'affiche déjà comme un acte à signe (§5.2).
- **Direction artistique** : geste « se changer », geste de coiffure au salon, gestes d'allumer une radio et de changer de station, au catalogue d'animations (§5.3 bis).

### 15.14 Aucune action de l'utilisateur (v0.7.1, A8)

**État** : la règle renforcée de l'utilisateur ([organisation.md](../../pilotage/organisation.md), §6 ; arbitrage A8 en tête de [05-architecture.md](../05-architecture.md)) est **appliquée en v0.7.1** (30/10/2026). Plus aucun passage de ce fichier ne fait agir l'utilisateur ou « le concepteur » : chaque jugement, essai, calibrage ou décision revient à une session. Les réglages offerts aux joueurs du jeu (§11) ne sont pas concernés.

| Passage | Avant | Après | Section |
|---|---|---|---|
| Tests à l'aveugle | « Le testeur » juge les personnages observés | Une **session évaluatrice neuve** juge, face à des **sessions joueuses** qui pilotent des comptes synthétiques ; l'outil se lit par un agent (images et texte de l'écran honnête) | §12.3 |
| Vue de conception | « Le concepteur doit voir l'intérieur » | Les sessions qui calibrent (responsable conception, qualité et essais) | §12.3 |
| Valeurs à calibrer | Ajustées « par le prototypage » | Ajustées par les sessions de qualité et essais ; valeurs de jeu arrêtées par le responsable conception | En-tête, §14 |
| Sanctions lourdes, rapprochement de comptes | « Décidés par un humain » | Décidés sur dossier par une **session de modération** distincte de la détection | §10.4, §12.1, §15.1, §15.9 |
| Rôles de la console | Sans titulaire précisé | Tenus par des sessions (modération, exploitation, responsable conception, audit) | §12.1 |
| Disposition de rotation | « Le prototype tranchera » | Décidé : C / V | §5.7, §14 |
| Juge et avocat, synthèse vocale | « À préciser au prototype », « à vérifier » | Confiés aux sessions interface et combat, et à la session client | §5.6.9, §11.1, §15.7 |

Points transmis, sans demande à l'utilisateur :

- **Responsable architecture et sécurité** : l'accès à la console exige une **clé matérielle FIDO2** (S21), qu'une session ne peut pas présenter. Il faut un accès pour les sessions de modération et d'exploitation qui garde le réseau privé, l'identité forte et le journal d'audit chaîné (par exemple une identité de service à clé non exportable, liée au poste) (§12.1).
- **Responsable exploitation** : une sanction lourde décidée par une session reste une **décision automatisée** au sens du RGPD (article 22). Avant l'ouverture publique, la contestation doit pouvoir obtenir un examen humain, par du personnel d'exploitation et jamais par l'utilisateur (§10.4).

### 15.15 Dernière vague avant la clôture (v0.8)

**État** : les demandes de [social.md](social.md) v0.7, §3.2, et de [economie.md](economie.md) v0.7, §14.10 (ligne « Interface »), sont **traitées en v0.8** (30/10/2026). Aucune contradiction n'a été relevée avec ces deux domaines. Une incohérence de ce fichier est corrigée : la v0.6 donnait une forme de bulle au ton sec, alors que la forme dit le volume et l'icône dit le ton.

| Demande | Traitement | Section |
|---|---|---|
| Social §3.2 : trois niveaux d'écoute | Compris (texte), bribes (fragments du serveur), entendu sans être compris (bulle sans texte, voix indistinctes, rien dans le journal) ; même rendu quel que soit le pilote | §2.8, §5.4 |
| Social §3.2 : formes selon le volume | Chuchoté en pointillés, normal à bord simple, élevé à bord double, crié à bord épais et dentelé ; le ton passe par une icône et un mot | §2.8, §5.4, §15.12 |
| Économie §1.4, §5.13 : prix par pièce | Étiquette par pièce dans la gamme de la boutique, essayage en cabine, passage en caisse ; ni valeur ni usure de garde-robe affichées | §8.5 |
| Économie §3.8 : chapeau du musicien de rue | Action « Donner », en espèces, don volontaire ; l'IA ne donne jamais à la place du joueur pendant une incarnation | §8.5 |

Ce qui touche d'autres domaines, **sans demande envoyée** (clôture), est noté ici avec l'état **« pour la phase 1 »** :

| Domaine | Point | État |
|---|---|---|
| Direction artistique | Formes de bulle par volume et icônes de ton (remplace le « ton sec à bord droit » du §15.12) ; geste d'essayage et cabine d'essayage au mobilier des boutiques de vêtements ; geste de don au chapeau | Pour la phase 1 |
| Ville | Une cabine d'essayage dans chaque boutique de vêtements (action « Essayer ») | Pour la phase 1 |
| IA des PNJ | Pendant une incarnation, aucun don au chapeau ni achat de vêtement n'est fait par l'IA à la place du joueur | Pour la phase 1 |
| Social | Le journal de conversation ne garde rien d'un acte entendu sans être compris ; seul le fait observé entre dans la mémoire, comme le prévoit le §3.2 de social | Pour la phase 1 (déjà conforme) |

### 15.16 Corrections des revues de clôture (v0.9)

**État** : les corrections des revues de clôture qui touchent ce fichier sont **intégrées en v0.9** (30/10/2026). Le socle (04 §8, 05 §2.5 et §2.16) est déjà à jour selon le responsable conception. Aucune contradiction n'a été relevée.

| Renvoi | Correction | Section |
|---|---|---|
| Architecture C-5 | Trois dates par relation (perception identifiée à toute précision, à « tenue » ou mieux, à « visage ») ; chaque groupe de champs du portrait reconstitué à la date de sa précision ; remplace la date unique de la v0.6 | §4.7, §15.1 |
| Sécurité C1 | Portrait mis à jour seulement quand le personnage identifie la personne ; jamais un état vu masqué et non identifié ; calcul à l'ouverture de l'écran, sans notification ; test d'écrans identiques tant que l'identification échoue | §4.7 |
| Sécurité, renvoi à l'équilibrage | Candidats de « Une nouvelle vie » à la précision « tenue », visage dans l'ombre ; le visage apparaît à l'incarnation | §3.3, §4.7 |
| Équilibrage C6 | Aucune raison de réticence ne cite un savoir opaque ; niveau affiché calculé sur la classe apparente de la relation ; vrai coût pour la synchronisation et la rupture ; coût supérieur à 1 jamais proposé dans les listes | §2.3 |
| Équilibrage C1 | Montant libre borné à ≈ 20 écus au chapeau et au pourboire ; au-delà de 2 000 écus cumulés par 30 jours, aucun don en espèces d'un personnage incarné au cercle associé ; sorties nettes plafonnées à 20 000 écus par 30 jours | §8.5 |
| Sécurité C7, équilibrage C8 | Notification des motifs, des faits retenus, de la règle, des moyens automatisés et du recours ; contestation par arguments structurés ; deux sessions indépendantes pour une mesure lourde, trois à la majorité pour une contestation | §10.4 |

Ce qui reste, **sans demande envoyée** (clôture), avec l'état **« pour la phase 1 »** :

| Domaine ou responsable | Point | État |
|---|---|---|
| IA des PNJ | Test d'identification (visage découvert, ou silhouette, voix ou démarche selon la familiarité) et « inconnu vu », qui alimentent les trois dates du portrait | Pour la phase 1 |
| Population | Historique daté de la coiffure, de la tenue habituelle et des signes distinctifs, déjà noté au §15.5, lu par groupe de champs | Pour la phase 1 |
| Social, signatures-contenu | Classe apparente d'une relation secrète, utilisée pour le niveau de réticence affiché et le code de raison | Pour la phase 1 |
| Économie | Pourboire en montant libre borné, comme le chapeau ; cumul de 2 000 écus en espèces vers le cercle associé | Pour la phase 1 |
| Responsable architecture et sécurité, exploitation | Dossier de modération construit par un outil déterministe, texte des joueurs cité entre délimiteurs ; courriel de notification ; délai et liste des motifs de contestation | Pour la phase 1 (modération : phase 5) |
