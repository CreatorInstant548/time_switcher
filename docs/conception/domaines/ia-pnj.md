# Domaine — IA des PNJ

> Fichier de **domaine** : modifié uniquement par la session « Conception — IA des PNJ » (voir [README](../README.md)).
> Version 0.2 — 29/10/2026 (corrections de la relecture de cohérence : niveaux de détail harmonisés avec la ville, vie des enfants, séjours à l'extérieur, renvois au socle mis à jour).
> Version 0.3 — 29/10/2026 : tous les points encore proposés sont décidés par la session de domaine, sur consigne de l'utilisateur, avec leur raison.
> Version 0.4 — 29/10/2026 : compléments demandés par les domaines population, économie et social (situations de vie, dépendances, recettes et projets, croyances et relations, réparations après une incarnation, gestion automatique et rodage).
> Version 0.5 — 29/10/2026 : compléments demandés par les domaines combat et interface (réflexes et panique de foule, observation pendant un affrontement, recettes de police et de justice, souvenir des témoins, garanties envers les mineurs, détention, bracelet et fuite, aides au joueur incarné, interruptions de l'avance du temps, plan de mémoire des étages).
> Version 0.6 — 29/10/2026 : budgets de calcul de la planification et des décisions exprimés en unités de travail (nœuds, opérations), pour la reproductibilité par graine.
> Version 0.7 — 29/10/2026 : intégration des revues d'équilibrage (E1, E6, E11, E13, E17, E18, E30) et de sécurité (S3, S4), avec les arbitrages de la session principale A1 (couche cachée tirée par monde) et A2 (validation des intentions en deux étages).
> Version 0.8 — 29/10/2026 : reliquats des revues (actes intimes, bulletin opaque, rétractation sans biais de la parole donnée) ; revue de l'intelligence des PNJ (§12), mécanismes d'intelligence et de réalisme (§13) et mesure de l'intelligence (§14).
> Version 0.9 — 29/10/2026 : alignement sur le social v0.5 (§12.10) : ton et registre distingués, champs étendus tenus dans les seules relations complètes, pardon accepté qui éteint la rancune et son thème de vie.
> Version 1.0 — 29/10/2026 : intégration des revues des signatures (G1 à G30) et de la génération (P1 à P41) : signature dérivée des effets par `appraise` (`ts_conduct`), épisode et coût marginal, planchers de rupture, réticence en quatre niveaux, témoins et traits supposés par `appraise`, calibrage des signatures avant les décisions ; zones de bâtiment, rodage à l'équilibre et par convergence, rodage propre sur les 14 jours qui précèdent le départ, mobilier mobile.
> Version 1.0.1 — 29/10/2026 : honnêteté des commerces (arnaque) ajoutée aux valeurs apprises, apprise en une fois et tenue aussi en statistique (§13.6) ; trois lignes du tableau « Socle » du §11 passées à « Intégré au socle le 29/10 » ; renvoi du point de rupture corrigé (03-vie-synchronisation §1, pénalité en 04-joueur §1.1).
> Version 1.1 — 29/10/2026 : demandes de la direction artistique ([direction-artistique.md](direction-artistique.md), §12.3) : tenue du jour choisie dans la garde-robe, se changer et prendre un parapluie, coiffeur, énumérations de gestes et d'activités clés du catalogue d'animations, activités musicales et sources de musique (§2.6).
> Version 1.1.1 — 29/10/2026 : reprises de [revues/signatures-contenu.md](../revues/signatures-contenu.md), §11.6 : J4 couvre le besoin critique d'un proche dépendant (§1.4 ; M6) ; R4 limité au personnage prudent (§5.3 ; M5) ; les 12 archétypes servent de grille à la lisibilité du caractère (§14.1) ; réticence en niveaux 0 à 3 plus l'indicateur de rupture (§5.6 ; M8).
> Version 1.1.2 — 29/10/2026 : vocalisation de chaque réplique choisie par une règle du cœur pour tout orateur (§3.6) ; dernier contact et précision la plus haute par relation pour reconstituer un portrait à une date (§4.1), historique daté de la coiffure et de la tenue habituelle (§2.6) ([interface.md](interface.md) v0.6, §15.2). Règle 6 du README : aucune section n'est renumérotée ; les nouvelles prennent un numéro libre ou un suffixe « bis ».
> Version 1.1.3 — 29/10/2026 : arbitrage A8, aucune action de l'utilisateur : les tests à l'aveugle et « fidèle ou non » sont jugés par un panel de sessions évaluatrices neuves face à des sessions joueuses, cibles inchangées (§14.3, §14.1) ; comparaisons des signatures par le même panel et indicateur inatteignable arbitré par le responsable conception (§13.13) ; délais humains tirés de données publiées (§10). Les marques « (validé) » restent : ce sont des décisions passées.
> Version 1.2 — 30/10/2026 : demandes du social v0.7 (§12.12) et de l'interface v0.7 (§15.2, §15.13) : escalade du bruit entre voisins et réponses de la source, retrait des signes de bande, actions de radio en trois crans, se changer là où sont les vêtements (§2.6) ; tenue attendue et `opens: [J8]` dans les pratiques (§13.9) ; classe de la cible et actes faits par le déplacement (§5.6). Impacts sur les autres domaines notés pour la phase 1 (§11).
> Version 1.3 — 30/10/2026 : corrections des revues de clôture (architecture C-1, C-2, C-5, C-6, S-4 ; sécurité C1 ; équilibrage C5, C6, C7) : contrôle des gestes en deux temps, tenue stockée à tous les niveaux, bruit des absents (§2.6) ; vocalisation sur flux dédié avec indice de variante (§3.6) ; quatre dates par relation, mises à jour sur identification (§4.1) ; réticence sans savoir opaque (§5.6) ; règles des panels et surfaces des sessions joueuses et des juges (§13.13, §14). Ce qui touche d'autres fichiers est noté pour la phase 1 (§11).
> Version 1.3.1 — 30/10/2026 : retouches du tour 2 de la revue d'architecture : flux `cosmetic/vocalization` (§3.6 ; T2-4) ; correction R1 de l'équilibrage, classe apparente d'une cible liée par un secret pour l'affichage et la proposition, vrai coût à l'exécution, éjection hors récidive (§5.6) ; juges servis par relecture du journal puis vue de l'observateur (§14.3 ; T2-3) ; ligne 05 du §11 « Direction artistique » corrigée (T2-5) ; T2-1 et T2-2 notés pour la phase 1 (§11).
> Les choix tranchés par l'utilisateur sont marqués **(validé)**. Les choix pris par la session de domaine, sur consigne de l'utilisateur, sont marqués **(décidé)** et accompagnés de leur raison. Les valeurs marquées **[à calibrer]** seront ajustées par le prototypage.

Ce domaine décrit comment l'IA fait vivre les personnages du monde : leurs besoins, leurs journées, leurs décisions, leur mémoire, la reprise après une incarnation, le coût de calcul, l'état « au repos », la vie des enfants et les séjours à l'extérieur. Il s'appuie sur le socle, en particulier [01-concept.md](../01-concept.md), [02-personnages.md](../02-personnages.md), [04-joueur.md](../04-joueur.md) et [05-architecture.md](../05-architecture.md), ainsi que sur [ville.md](ville.md) pour les lieux, les trajets et le découpage spatial.

La **création initiale** des habitants (ménages, emplois, logements), le registre des ménages, les naissances, décès, arrivées et départs relèvent du domaine **population** ([population.md](population.md)). Ce domaine-ci part de personnages déjà créés et décrit comment ils vivent. Il s'appuie aussi sur [economie.md](economie.md) (métiers, budget, recettes économiques), [social.md](social.md) (relations, actes sociaux, croyances, groupes), [combat.md](combat.md) (réflexes, témoins, police, justice, détention) et [interface.md](interface.md) (ce que le joueur voit du personnage incarné).

## Sommaire

0. [Principes](#0-principes)
1. [Besoins et motivations](#1-besoins-et-motivations)
2. [Emplois du temps et routines](#2-emplois-du-temps-et-routines)
3. [Prise de décision](#3-prise-de-décision)
4. [Mémoire et relations](#4-mémoire-et-relations)
5. [Incarnation, reprise et objectifs du personnage](#5-incarnation-reprise-et-objectifs-du-personnage)
6. [Niveaux de détail et coût de calcul](#6-niveaux-de-détail-et-coût-de-calcul)
7. [L'état « au repos »](#7-létat--au-repos-)
8. [La vie des enfants](#8-la-vie-des-enfants)
9. [Séjours à l'extérieur](#9-séjours-à-lextérieur)
10. [Points ouverts](#10-points-ouverts)
11. [Impacts sur le socle et les autres domaines](#11-impacts-sur-le-socle-et-les-autres-domaines)
12. [Revue de l'intelligence des PNJ](#12-revue-de-lintelligence-des-pnj)
13. [Mécanismes d'intelligence et de réalisme](#13-mécanismes-dintelligence-et-de-réalisme)
14. [Mesurer l'intelligence et le réalisme](#14-mesurer-lintelligence-et-le-réalisme)

---

## 0. Principes

1. **Une seule IA générique, des rôles décrits en données** (validé, repris de l'archive). Un métier ou un rôle (boulanger, policier, revendeur, mère au foyer) est une **fiche de données** : agenda type, recettes de planification propres, savoirs, actions permises, objectifs typiques. Ajouter un rôle ne demande pas de nouvelle IA. Pour un métier, la partie économique de la fiche est la **fiche de métier** du domaine économie ([economie.md](economie.md), §3.1 ; catalogue au §3.8, rôles du prototype au §3.9) (décidé ; *pourquoi : un seul format sert à l'IA, à l'embauche, à la paie et au personnage incarné*). Les situations de vie particulières ont aussi leur fiche ou leur état (§2.5).
2. **L'IA est un pilote comme un autre** (socle, [05-architecture.md](../05-architecture.md) §2.3) : elle produit les mêmes **intentions** qu'un joueur, exécutées par les mêmes systèmes.
3. **L'IA n'en sait pas plus que le personnage** (décidé) : elle ne décide qu'à partir de ce qu'il perçoit (filtré par sa perception, son attention, son stress) et de ce qu'il a en mémoire. Pas d'omniscience : les actions qu'il envisage sont jugées possibles d'après ce qu'il perçoit et sait, jamais d'après l'état réel du monde (§3.7). *Pourquoi : une IA qui en saurait plus que le personnage tricherait, et rendrait incohérents les témoins et les savoirs opaques.*
4. **L'IA est limitée par les mêmes attributs qu'un joueur** (décidé) : temps de réaction, coordination, fatigue et blessures dégradent ses intentions exactement comme celles d'un joueur. Un PNJ épuisé conduit mal, qu'il soit piloté par l'IA ou par un joueur. *Pourquoi : joueur et IA produisent les mêmes intentions (socle) ; s'ils n'avaient pas les mêmes limites, la reprise par l'IA se verrait.*
5. **L'état est dans le cœur, pas dans l'IA** (décidé) : besoins, agenda, mémoire et plans sont des composants du personnage. On peut changer de pilote ou de niveau de détail à tout moment sans rien perdre. *Pourquoi : c'est ce qui rend possibles l'échange de pilote et les niveaux de détail de l'architecture ECS du socle.*
6. **Fidélité par construction** (décidé) : chaque façon d'agir porte une **signature comportementale**, la même que celle qui sert à mesurer la synchronisation du joueur ([04-joueur.md](../04-joueur.md), §3). L'IA choisit ses moyens en fonction du coût de cette signature pour son propre profil. La signature est **dérivée des effets** de l'acte et calculée par la fonction unique `appraise` de la bibliothèque `ts_conduct` (décidé ; [revues/signatures.md](../revues/signatures.md), G1 à G6 ; *pourquoi : une grammaire d'effets écrite une fois remplace des milliers de signatures écrites à la main, et une seule fonction garantit le même jugement pour l'IA, le joueur, les témoins et les souvenirs*). Un personnage piloté par l'IA est donc toujours « synchronisé avec lui-même », et le même outil sert aux deux. *Pourquoi : une seule mesure de fidélité, partagée par l'IA et la synchronisation, garantit que l'IA joue « juste » au sens même où l'on juge le joueur.*

---

## 1. Besoins et motivations

Les besoins sont **universels** ; le profil comportemental change **leur poids** et surtout **les moyens acceptables** pour les satisfaire (§1.4).

Niveau de détail retenu : **simulation de vie détaillée** (validé).

### 1.1 Besoins courts

Chaque besoin est une jauge de 0 (satisfait) à 100 (critique), qui monte avec le temps ou l'activité et qui baisse quand on le satisfait. Les vitesses et les seuils sont **[à calibrer]**.

| Besoin | Monte avec | Se satisfait par | Effets quand il est élevé | Catégorie |
|---|---|---|---|---|
| **Énergie (sommeil)** | Temps éveillé, effort | Dormir | Dette de sommeil : réactions ralenties, perception dégradée, irritabilité, micro-sommeils (progression réaliste) | Important |
| **Faim** | Temps, effort | Manger | Baisse d'endurance et de concentration, irritabilité | Important |
| **Soif** | Temps, chaleur, effort | Boire | La déshydratation dégrade la cognition avant la force | Trivial |
| **Vessie** | Temps, boissons | Toilettes | Inconfort croissant, puis incident gênant (effet social) | Trivial |
| **Hygiène** | Temps, effort, saleté | Se laver (gestes courts) ; douche (activité de routine) | Réactions négatives des autres personnages, baisse d'estime de soi | Mixte |
| **Confort** | Froid, chaleur, pluie, vent (météo et saisons, [01-concept.md](../01-concept.md), §3), tenue inadaptée, position pénible, bruit | S'abriter, se couvrir, s'asseoir, changer d'endroit | Le froid dégrade la dextérité puis la cognition ; la chaleur réduit l'endurance ; une exposition prolongée peut rendre malade | Trivial |
| **Santé / douleur** | Blessures, maladie | Soins, repos, médicaments | Performance dégradée, recherche de soins | Important |
| **Sécurité** | Menaces perçues, lieux jugés dangereux | S'éloigner, se protéger, chercher de l'aide | Stress, fuite, état non « au repos » au-delà d'un seuil (§7) | Important |
| **Lien social** | Temps sans contact réel | Conversations, activités partagées | Ennui, tristesse, recherche de compagnie | Important |
| **Détente** | Travail, stress, effort mental | Loisirs, repos | Fatigue mentale, irritabilité, erreurs | Important |
| **Stimulation** | Routine, absence de nouveauté | Nouveaux lieux, activités, rencontres | Ennui, prise de risque accrue | Important |

Les besoins **triviaux** sont gérés par des gestes automatiques pendant une incarnation (§5.2). Le besoin **mixte** (hygiène) l'est pour ses gestes courts (se laver les mains), tandis que la douche reste une activité de la routine. Les effets sociaux d'une hygiène négligée sont fixés par le domaine social ([social.md](social.md), §5.5).

**Dépendances** (décidé) : certains personnages ont en plus un besoin de **manque** lié à une dépendance (tabac, alcool, cannabis, cocaïne) ([02-personnages.md](../02-personnages.md), §2 ; [economie.md](economie.md), §7.3).

- Il monte avec le temps écoulé depuis la dernière prise, d'autant plus vite que la dépendance est forte **[à calibrer]**.
- Élevé, il cause irritabilité, stress et baisse de concentration, et pousse à **se procurer le produit**, par des moyens filtrés par le profil comme les autres (§1.4). Un manque critique peut faire franchir le seuil des moyens désespérés.
- **Se sevrer** est un projet (§3.4), plus ou moins probable selon le profil, la santé et l'entourage.
- Pendant une incarnation, consommer un produit **légal déjà en sa possession** (une cigarette, un verre) peut être un geste automatique au repos (§5.2) ; se procurer le produit, ou consommer un produit illégal, ne l'est jamais.

*Pourquoi : traiter la dépendance comme un besoin réutilise toute la mécanique existante, et crée la vraie demande dont l'économie parallèle a besoin.*

### 1.2 Motivations longues (décidé)

*Pourquoi : les besoins courts expliquent la journée, mais il faut des motivations lentes pour faire naître des projets et des vies qui évoluent.*

Les motivations ne bougent pas d'heure en heure : elles mesurent la satisfaction du personnage vis-à-vis de sa vie, réévaluée environ une fois par jour de jeu **[à calibrer]**. Quand une motivation est insatisfaite, elle fait naître des **projets** (§3.4).

| Motivation | Mesure | Exemples de projets |
|---|---|---|
| **Argent** | Revenus, épargne et dettes par rapport aux besoins du foyer | Faire des heures en plus, chercher un emploi ou un meilleur emploi, demander une prestation, emprunter, rembourser une dette, petit trafic |
| **Relations** | Qualité des relations proches (famille, amis, couple) | Se réconcilier, trouver un partenaire, emménager ensemble, avoir un enfant, se séparer, accueillir un parent âgé, aider un proche malade |
| **Statut / ambition** | Position sociale et professionnelle par rapport à ses attentes | Obtenir une promotion, monter son affaire, acheter un logement, quitter ses parents, prendre la tête d'une bande |
| **Devoir** | Obligations envers le travail, la famille, une organisation | Tenir ses engagements, protéger un proche, servir son organisation |
| **Convictions** | Écart entre le monde et ses valeurs | Aider des inconnus, dénoncer un crime, se venger d'une injustice |

### 1.3 Émotions et stress (décidé)

*Pourquoi : ces états existent déjà dans la progression réaliste ; les relier aux décisions évite de créer un second système d'humeur.*

Les états temporaires de la progression réaliste (stress, peur, colère, tristesse, joie) modulent les décisions :

- le **stress** suit une courbe en cloche (un peu améliore la performance, trop la dégrade) ;
- au-delà d'un seuil, la **panique** déclenche les réflexes (§3.1) et rend le personnage non « au repos » ;
- la **colère** et la peur augmentent le poids des réactions immédiates par rapport aux plans ;
- la **régulation émotionnelle** (attribut) et la **stabilité émotionnelle** (profil) règlent la vitesse de montée et de retombée ;
- (décidé, v0.8) ces états sont produits par l'**évaluation des événements** et complétés par des **pensées** datées, une **humeur** lente et des **crises** (§13.1, §13.2).

### 1.4 Lien avec le profil comportemental (décidé)

*Pourquoi : filtrer les moyens plutôt que les besoins garde des besoins universels et fait porter la personnalité sur ce qui se voit : la façon d'agir.*

Le profil agit de deux façons : il **pondère les besoins** et il **filtre les moyens**.

| Dimension du profil | Effet sur les besoins | Effet sur les choix |
|---|---|---|
| Introversion ↔ Extraversion | Le lien social monte plus vite chez l'extraverti ; l'introverti récupère de la détente en solitude | Loisirs seul ou en groupe, lieux calmes ou animés |
| Sensation ↔ Intuition | La stimulation monte plus vite chez l'intuitif | Lieux et activités connus ou nouveaux ; projets concrets à court terme ou plans à long terme |
| Pensée ↔ Sentiment | — | Poids donné à l'efficacité et au gain, ou à l'effet sur les autres |
| Jugement ↔ Perception | — | Régularité de l'agenda, ponctualité, tenue des engagements, facilité à s'écarter du plan |
| Stabilité émotionnelle | Vitesse de montée du stress | Seuil de panique, impulsivité (action immédiate plutôt que plan) |
| Bienveillance | Les besoins des proches et des inconnus comptent dans ses objectifs | Aider, ou au contraire nuire, comme moyen acceptable |
| Respect des règles | — | Coût des moyens illégaux (voler, frauder, trafiquer) |
| Honnêteté | — | Coût du mensonge et de la manipulation comme moyen |
| Violence | — | Coût de la force comme moyen, seuil de réponse violente |

**Règle de choix des moyens** (décidé ; [revues/signatures.md](../revues/signatures.md), §3.5 et §3.9) : la signature d'une recette de planification (§3.2) est l'**agrégat des effets de ses étapes** ; son coût pour un personnage suit le modèle de **tolérance et de poids** par axe : plus l'effet est grave, moins de profils le tolèrent et plus le dépassement coûte. Les états passagers (ivresse, colère, panique) décalent le **profil effectif**, pas la signature. Au-delà d'un seuil **[à calibrer]**, la recette est écartée, sauf en situation désespérée : un besoin critique **de l'acteur ou d'un proche dépendant** (enfant, parent à charge) sans moyen fidèle connu ouvre la justification **J4** (nécessité ; décidé, M6, [revues/signatures-contenu.md](../revues/signatures-contenu.md), §3.1 ; *pourquoi : voler pour nourrir son enfant est le cas le plus courant de la nécessité*), qui peut pousser un honnête homme affamé à voler, avec un fort stress et un souvenir dissonant (§4.5). *Pourquoi : la tolérance dit ce que la distance à un point ne dit pas : un menteur n'est pas dissonant quand il dit la vérité, un honnête homme l'est quand il ment, d'autant plus que le mensonge est grave.*

---

## 2. Emplois du temps et routines

### 2.1 Agenda + besoins (validé)

Chaque personnage a un **agenda hebdomadaire** qui fixe ses **obligations** : sommeil, travail, trajets, enfants, rendez-vous. Le **temps libre** est rempli selon ses besoins et ses motivations. Un besoin urgent ou un danger peut **interrompre** l'agenda.

L'agenda est généré à partir de (décidé ; *pourquoi : ce sont les sources qui fixent un emploi du temps dans la vraie vie*) :

- la **fiche de rôle** (métier, horaires, lieu de travail) ;
- le **foyer** (domicile, conjoint, enfants scolarisés, personnes à charge) ;
- le **chronotype** du personnage (lève-tôt ou couche-tard), qui décale ses heures de sommeil et ses loisirs. C'est un **trait tiré à la création et modulé par l'âge** (adolescents plus tardifs, personnes âgées plus matinales) (décidé ; *pourquoi : c'est réaliste, peu coûteux, et cela étale naturellement l'activité de la ville sur la journée*) ;
- ses **engagements** (rendez-vous pris par lui ou par un joueur qui l'incarnait, §5.3).

### 2.2 Régularité (décidé)

*Pourquoi : des obligations fortes mais non absolues permettent retards et absences, qui créent des conséquences et des histoires.*

- Les obligations ont une **priorité élevée** mais pas absolue : un personnage peut être en retard, sauter un repas, poser un jour de congé ou ne pas aller au travail.
- La régularité dépend du profil : un personnage très « Jugement » suit son agenda à quelques minutes près ; un personnage très « Perception » varie ses horaires et s'en écarte plus facilement.
- **Variations** : semaine et week-end, jours fériés, vacances scolaires, congés, maladie, événements de la vie (déménagement, naissance, licenciement), séjours à l'extérieur (§9).
- **Météo et saisons** ([01-concept.md](../01-concept.md), §3) : elles changent le choix des activités et des trajets (moins de sorties et plus de bus sous la pluie, terrasses et parcs l'été, soirées plus courtes en hiver), la tenue choisie, et la vitesse de montée du besoin de confort. Chaque activité de loisir et chaque mode de trajet a une sensibilité à la météo, qui entre dans son score de faisabilité (§3.2) (décidé ; *pourquoi : c'est la façon la plus simple de faire jouer la météo dans toutes les décisions, sans règle spéciale*).

### 2.3 Temps réel en ligne (décidé)

*Pourquoi : le socle impose le temps réel ; des durées réalistes et des objectifs découpés en tâches gardent le jeu jouable par sessions courtes.*

En ligne, une journée de jeu dure 24 heures réelles ([01-concept.md](../01-concept.md), §2). Conséquences :

- Les durées sont **réalistes** : un PNJ dort 7 à 9 heures réelles, une journée de travail dure environ 8 heures réelles.
- Grâce au **serveur par fuseau horaire**, un joueur qui se connecte le soir trouve le monde dans sa soirée : les gens rentrent chez eux, les bars se remplissent.
- La nuit n'est pas vide : une part des personnages a des **horaires de nuit** (sécurité, soins, bars, transport, livraison, criminalité). Cette part dépend des domaines économie et ville (§11).
- **Objectifs de travail granulaires** : comme une journée de travail dure 8 heures réelles, les objectifs liés au travail sont découpés en **tâches** (servir un client, terminer une réparation, livrer un colis) qui rapportent chacune un peu d'expérience, dans la limite d'un plafond horaire (§5.4). Le joueur peut rendre le personnage au travail à tout moment : l'IA termine la journée.

### 2.4 Accélération en solo (décidé)

*Pourquoi : appliquer les mêmes règles avec une résolution plus grossière garantit qu'avancer le temps ne change pas la nature du monde.*

En solo, une heure de jeu dure une heure réelle hors accélération ([01-concept.md](../01-concept.md), §2). Quand le joueur fait avancer le temps (attendre, dormir, sauter des heures), le monde passe en simulation **statistique** (§6), sauf une **bulle autour du joueur en simulation simplifiée**.

- Les blocs d'agenda sont **résolus en une fois** : « 8 heures de travail » donne un salaire, de la fatigue, des souvenirs résumés et des événements tirés au sort (dispute avec un collègue, client mémorable).
- Les règles sont **les mêmes** qu'en temps réel, seule la résolution est plus grossière : un personnage simulé en accéléré vit la même vie, en moins détaillé.
- **Interruptions** (décidé) : le cœur signale à l'interface les événements qui concernent le personnage du joueur et arrêtent l'avance du temps ([interface.md](interface.md), §9.1) : quelqu'un s'adresse à lui, sonne ou frappe ; une menace est perçue ; un besoin franchit son seuil d'urgence ; un engagement approche (environ 15 minutes avant **[à calibrer]**). Ce sont les mêmes événements que ceux qui relancent la sélection d'objectif (§3.2), et l'avance n'est possible que dans des conditions proches de l'état « au repos » (§7.1). *Pourquoi : réutiliser les déclencheurs de l'IA garantit que le joueur n'est jamais réveillé pour rien ni privé d'un événement qui le concerne.*

### 2.5 Situations de vie particulières (décidé)

*Pourquoi : une fiche ou un état par situation réutilise l'IA générique ; ces situations changent surtout l'agenda et les besoins, demandés par le domaine population ([population.md](population.md), §12.3).*

Un **état** s'ajoute à la fiche de rôle du personnage ; une **fiche** la remplace.

| Situation | Forme | Effets sur l'agenda et les décisions |
|---|---|---|
| **Grossesse** | État | Environ neuf mois de jeu ; endurance réduite progressivement ; rendez-vous médicaux ; congé ; préparatifs (logement plus grand, achats) ; l'entourage l'apprend peu à peu ([population.md](population.md), §7.2) |
| **Nourrisson** | Fiche | Voir §8.1 (0 à 2 ans) |
| **Étudiant** | Fiche | Cours à l'antenne universitaire, révisions, petit emploi possible, budget serré, sorties ; départ fréquent en fin d'études ([population.md](population.md), §7.5) |
| **Navetteur sortant** | Fiche | Travaille hors de la carte : part chaque jour ouvré par le train ou la route et reste **à l'extérieur** pendant ses heures de travail (§9), avec un résumé minimal au retour (salaire, fatigue) ; non jouable pendant ce temps ([population.md](population.md), §3.2) |
| **Sans-abri** | Fiche | Pas de domicile : agenda fait de lieux d'accueil (foyer, distribution de repas, gare, bancs) ; besoins de sommeil, d'hygiène, de confort et de sécurité difficiles à satisfaire ; mendicité, petits boulots ; projets de sortie de la rue (prestation, hébergement, emploi) |
| **Résident de maison de retraite** | Fiche | Agenda fixé par l'établissement (repas, soins, activités) ; mobilité souvent réduite ; les visites des proches sont les grands moments de lien social |
| **Fin de vie, soins palliatifs** | État | À domicile, à l'hôpital ou en maison de retraite ; la santé et la douleur dominent ; agenda très réduit ; les proches multiplient les visites et des projets naissent (se réconcilier, régler ses affaires, transmettre). La jouabilité suit le socle (état grave, [02-personnages.md](../02-personnages.md), §3.4) |
| **Enfant en garde alternée** | Fiche (enfant) | Deux domiciles ; l'agenda alterne selon le calendrier de garde (par exemple une semaine sur deux) ; les obligations d'accompagnement passent d'un parent à l'autre ([population.md](population.md), §2.2) |
| **Sous bracelet électronique** | État | Détention à domicile pour une courte peine : l'agenda reçoit des **horaires imposés** (travail autorisé, couvre-feu) ; un manquement est détecté et peut renvoyer en prison. Le personnage reste jouable ([combat.md](combat.md), §8.4) |
| **Sortie de détention, sursis** | État | Obligations de suivi (rendez-vous avec le conseiller d'insertion, recherche d'emploi, interdictions) dans l'agenda pendant le délai d'épreuve ([combat.md](combat.md), §9.3) |
| **Juré, témoin cité** | Rôle temporaire | Obligation de l'agenda pendant une session d'assises ou à la date d'audience ; l'employeur doit libérer le salarié ([combat.md](combat.md), §8.2 et §8.3) |

### 2.6 Tenue, coiffure, gestes et musique (décidé ; direction artistique)

*Pourquoi : ce que l'on voit et entend d'un habitant (sa tenue, sa coiffure, ses gestes, sa musique) doit venir de sa vie et non d'un tirage du client ; c'est la règle du domaine direction artistique ([direction-artistique.md](direction-artistique.md), §0 et §3.4), et ces choix restent des routines bon marché.*

**Tenue du jour** (décidé ; [direction-artistique.md](direction-artistique.md), §3.3) :

- **Quand** : la tenue est choisie au lever, avant de sortir et à chaque changement d'**usage** (travail, sortie, sport, nuit), parmi les tenues de la **garde-robe abstraite** du personnage.
- **Comment** : l'usage vient de l'activité prévue par l'agenda ; parmi les tenues de cet usage, un petit score retient la meilleure, avec le tirage parmi les trois meilleures du §13.7 :
  - **lieu** : uniforme ou tenue de travail liée au poste, tenue soignée pour un lieu chic ;
  - **météo** : celle que le personnage **connaît**, vue par la fenêtre ou annoncée par la radio et le téléphone, jamais la météo réelle à venir ; sous la pluie, le froid ou le vent, la tenue « intempéries » ou une couche de plus ;
  - **occasion** : enterrement, mariage, entretien d'embauche, rendez-vous amoureux, venues des pratiques sociales (§13.9) et des engagements ; la **tenue attendue** est celle que la pratique déclare dans ses données ([social.md](social.md), §5.5) ;
  - **goûts vestimentaires** et **soin** du génome d'apparence ; **état** des pièces : un personnage soigné ne remet pas une pièce sale ou tachée, un négligé, si ;
  - **signes de bande** (décidé ; [social.md](social.md), §6.6) : un membre porte les couleurs ou la pièce distinctive de sa bande d'autant plus que son rang et sa loyauté sont élevés. Il les **retire** quand la pression de la police qu'il perçoit est forte (contrôles subis, arrestations connues, patrouilles dans le territoire), pour aller hors du territoire, ou pour une occasion qui ne s'y prête pas (entretien, enterrement hors du milieu). Ôter un signe (couvre-chef, foulard, veste) ou le cacher (capuche, veste fermée) est un **geste court**, possible partout, à la différence de se changer. *Pourquoi : les signes de bande sont un message qu'on n'envoie qu'à ceux qui doivent le lire, et on les enlève en sortant de chez soi comme on enlève un insigne.*
- **Se changer** : un geste de routine de quelques minutes **[à calibrer]**. On se change en rentrant (tenue de maison ou de nuit), avant le travail (chez soi ou au vestiaire du poste), avant une sortie, et quand on est **mouillé** ou sale. On se change, comme le joueur, **là où sont les vêtements** : armoire ou commode du logement, vestiaire du poste, sac emporté ; mouillé loin de chez soi, on attend d'y être (décidé ; [interface.md](interface.md), §5.3 bis).
- **Parapluie** : c'est un accessoire de la garde-robe, pris à la porte du logement par la même action que le joueur, si la pluie est probable d'après ce que le personnage sait **[seuil à calibrer]**. Il peut l'oublier (imperfections, §13.7) ; sans parapluie sous la pluie, le confort monte (§1.1), le personnage s'abrite ou presse le pas, et sa tenue passe à l'état « mouillé ».
- **Entretien** : les pièces se salissent et s'usent avec les activités. Laver (machine, laverie) est une activité de routine ; une garde-robe usée ou incomplète fait naître un projet d'achat de vêtements selon le budget (§3.4 ; [economie.md](economie.md)).
- **Personnage piloté** : le joueur choisit sa tenue parmi la même garde-robe ; l'IA en observation ne change jamais la tenue d'un personnage incarné. Une tenue mal choisie fait monter le confort et compte dans ce que les autres lisent de lui ([social.md](social.md), §5.5).

**Coiffeur** (décidé ; [direction-artistique.md](direction-artistique.md), §3.2) :

- La **coiffure** (coupe, longueur) fait partie de l'état courant. La longueur pousse avec le temps, de façon déterministe.
- Quand l'écart à la coupe voulue dépasse un seuil propre au personnage (soin, style, budget, métier), la course « aller chez le coiffeur » devient un objectif candidat du temps libre, comme les courses. Fréquence typique : toutes les 4 à 10 semaines **[à calibrer]**.
- Le salon est choisi comme tout commerce : valeurs apprises (§13.6), réputation et fidélité ([social.md](social.md), §5.9) ; un coiffeur habituel est une relation à part entière.
- Une **nouvelle coupe** est plus probable après un événement de vie marquant (rupture, nouvel emploi, mariage), par une conclusion ou un thème de vie (§13.4). Sinon on reprend la même.
- Sans moyens ou sans mobilité (sans-abri, personne âgée isolée), la coupe est faite à la maison par soi ou par un proche, ou attend.
- En simulation statistique, la visite est un résultat du bloc d'agenda, qui remet la longueur à zéro ; son tirage utilise la clé `(identifiant stable, date de la visite)`. Pour un personnage incarné, le joueur choisit la coupe au salon.
- **Historique daté** (décidé ; [interface.md](interface.md), §4.7 et §15.5) : chaque changement de coupe est daté dans un court historique, et la longueur se recalcule entre deux visites. De même, chaque changement de la **tenue habituelle** (garde-robe renouvelée, nouvel uniforme) est daté. Au plus 8 entrées de chaque sorte **[à calibrer]**, les plus anciennes résumées. *Pourquoi : c'est ce qui permet au serveur de montrer une connaissance telle qu'on l'a vue la dernière fois.*

**Gestes et activités, clés du catalogue d'animations** (décidé ; [direction-artistique.md](direction-artistique.md), §3.6, §6.7 et §6.9 ; [revues/architecture.md](../revues/architecture.md), §13.6) :

- Ce que l'IA rend visible passe par des **énumérations fermées**, rangées dans les données : activité en cours, geste (étapes des recettes, gestes automatiques du §5.2, gestes au repos du §13.7), posture d'humeur (§13.2).
- Chaque valeur a un **identifiant stable** et une **famille** (marcher, manipuler, parler, s'asseoir, etc.). Les identifiants ne sont jamais renumérotés, pour que les journaux restent relisibles.
- **Contrôle en deux temps, selon le sens des dépendances** (décidé ; revue d'architecture de clôture, C-1) :
  - la construction du paquet de contenus vérifie seulement que chaque valeur d'activité, de geste, de posture et de vocalisation a un **identifiant stable** et une **famille** prise dans une liste fermée, rangée avec les données ;
  - côté client, `cargo xtask assets check` vérifie qu'au moins une animation existe pour chaque **famille**, et lève une **alerte** pour chaque valeur sans animation propre ([direction-artistique.md](direction-artistique.md), §6.9).
  - Le cœur ne dépend jamais du catalogue du client : c'est le client qui lit le paquet, jamais l'inverse. *Pourquoi : `ts-sim`, `ts_py` et les tests doivent pouvoir construire le paquet sans le client, et un domaine doit pouvoir ajouter un geste sans toucher au client.*
- En attendant une animation propre, le client joue celle de la **famille**. *Pourquoi : un personnage ne doit jamais rester figé, et ajouter une activité au cœur ne doit pas attendre l'animation finale.*
- Le cœur n'envoie que le geste **perçu** (vue filtrée, [05-architecture.md](../05-architecture.md), §2.5) : un geste discret (glisser un objet, crocheter) n'apparaît chez un observateur que s'il le voit.

**Activités musicales et musique entendue** (décidé ; [direction-artistique.md](direction-artistique.md), §5.4) :

- **Activités de loisir en données**, comme les autres, avec leur sensibilité au lieu, à l'heure et au voisinage :
  - **jouer d'un instrument** : il faut l'instrument chez soi et la compétence (musique), qui progresse en jouant ; enfants et adolescents en cours de musique (§8.1) ;
  - **écouter la radio ou de la musique** : chez soi, en voiture, au travail selon le poste ; la station est choisie selon les **goûts musicaux** du personnage, tirés à la génération ;
  - **mettre de la musique** dans un bar, une boutique ou une fête : c'est une tâche du poste (serveur, patron) ou de l'hôte, choisie selon l'heure et la clientèle ;
  - **jouer en public** : musicien de bar ou de rue, organiste de l'église, répétition d'un groupe ; c'est une activité ou un poste, payé ou non ([economie.md](economie.md)).
- **La musique entendue en découle** : le cœur tient, pour un lieu ou un objet, une **source musicale** (radio allumée sur une station, sono, musicien en train de jouer) avec son volume ; il ne choisit pas les morceaux. Le volume d'un appareil a **trois crans** (bas, moyen, fort), les mêmes que ceux du joueur ([interface.md](interface.md), §5.3 bis).
  - Pour une radio, la station suffit : le programme est calculé par le client à partir de l'heure du monde et d'une graine publique ([direction-artistique.md](direction-artistique.md), §5.4).
  - Pour un musicien, le cœur donne l'instrument, le style et la qualité (sa compétence) ; le client en tire la musique.
- **Effets** : une source trop forte fait monter le besoin de **confort** (bruit) des voisins, qui peut mener à une plainte, une dispute ou une rancune (escalade ci-dessous ; [social.md](social.md), §5.11). Une musique entendue est une **occasion** (§13.7) et fait naître une petite pensée (§13.2) selon les goûts **[à calibrer]**.
- En simulation statistique, il n'y a pas de source musicale : l'activité est un résultat du bloc (compétence, plaisir, gêne des voisins tirée).

**Radio : les mêmes actions que le joueur** (décidé ; [interface.md](interface.md), §5.3 bis) : l'IA se sert des actions de contexte du joueur :

- **Allumer la radio** sur un poste, une chaîne ou l'autoradio ;
- **Changer de station**, selon ses goûts musicaux ; une station imposée (au travail, chez un hôte) loin de ses goûts lui donne une petite pensée ;
- **Volume** en trois crans. Le cran dépend de l'heure, de la présence d'autres personnes, des murs et du profil (bienveillance, respect des règles) : un voisin attentif ne met presque jamais le cran fort la nuit.

*Pourquoi : un seul jeu d'actions pour tous les pilotes ; une radio réglée fort par l'IA ou par un joueur gêne de la même façon.*

**Bruit entre voisins : recettes de l'escalade** (décidé ; [social.md](social.md), §5.11) :

- **Le voisin gêné** choisit, par la sélection d'utilité (§3.2), parmi quatre recettes, selon son profil (stabilité émotionnelle, bienveillance, violence, respect des règles), sa relation avec la source, l'heure et la **répétition** (lassitude, §13.11) :
  1. **Tolérer** : aucune action, une petite pensée négative (§13.2) ;
  2. **Demander de baisser** : sonner, frapper à la porte ou au mur, par l'acte « demander un service », sur le ton que fixe son humeur ;
  3. **Se disputer** : quand la demande a échoué ou que le bruit revient ;
  4. **Se plaindre** : au propriétaire ou au bailleur, au syndic, à la mairie, ou à la police pour un tapage nocturne.
  En général, chaque recette suppose que la précédente a échoué ; un profil irritable ou une heure très tardive peut sauter un pas **[à calibrer]**.
- **La source** répond par l'une de trois recettes, selon son profil, sa relation avec le voisin et le ton de la demande :
  - **baisser et s'excuser** : volume au cran bas ou radio éteinte ; le tort est en partie réparé ;
  - **promettre sans tenir** : un engagement, tenu ou non selon son caractère (§5.3), manqué si le bruit revient ;
  - **ignorer ou répondre sèchement**, ce qui fait monter l'escalade.
  Un joueur qui est la source répond par ses propres actes, et les mêmes suites s'appliquent.
- **Absents et imputation** (décidé ; revue d'équilibrage de clôture, C5) :
  - les sources allumées par un **personnage persistant** s'éteignent à sa disparition, comme à son départ de la carte ;
  - le bruit n'est imputé à l'occupant que si le voisin gêné le **croit présent** (sa dernière perception de lui, ses habitudes connues). Sinon, la gêne reste une pensée sans coupable, ou vise l'inconnu qu'il a vu entrer.
  - *Pourquoi : sinon, une radio laissée dans un logement vide ou une sono allumée chez un absent ferait accuser quelqu'un qui n'y est pour rien.*
- Chaque gêne entre au bilan de la relation comme un petit tort ; répétée, elle mène par la réflexion nocturne à une rancune (§13.4).
- **Niveaux** : complet et simplifié ; en statistique, l'escalade avance d'un pas par tirage ([social.md](social.md), §5.11).

*Pourquoi : ce sont des recettes ordinaires, choisies comme les autres ; le social en fixe seulement les suites.*

**Coût** : environ 10 opérations par changement de tenue, quelques changements par jour ; le coiffeur et la musique sont des objectifs et des activités ordinaires. **Niveaux** : la coiffure et les sources musicales sont tenues en complet et en simplifié. La **tenue portée est stockée à tous les niveaux** (≈ 30 octets ; décidé ; revue d'architecture de clôture, C-2) :

- en statistique, elle n'est mise à jour qu'au **changement d'usage** du bloc d'agenda, avec l'aléa à clé `(identifiant stable, jour, usage)` et la météo connue à l'heure de ce changement ;
- les états passagers (mouillé, sale) se déduisent de la météo et de l'activité du bloc ;
- au passage en simplifié ou en complet, rien n'est recalculé ;
- le test d'équivalence entre niveaux compare les distributions de tenue du complet et du statistique (§13.12).

*Pourquoi : un recalcul au passage de niveau dépendrait du trajet des joueurs : le même passant changerait de tenue entre deux regards, ou serait habillé à 7 h pour la pluie de 15 h ; 0,3 Mo pour 10 000 habitants ne vaut pas cette incohérence.*

---

## 3. Prise de décision

### 3.1 Les couches

```
PERCEPTION         ce que le personnage voit, entend, sait (filtré par ses attributs)
      │
      ▼
RÉFLEXES           danger soudain → fuir, se défendre, se figer, appeler à l'aide
      │            (court-circuitent la planification)
      ▼
1. SÉLECTION       score d'utilité de chaque objectif candidat
   D'OBJECTIF      (besoins, obligations de l'agenda, projets, engagements)
      │
      ▼
2. PLANIFICATION   HTN : recettes décomposées (vie courante)
                   GOAP : recherche limitée quand aucune recette ne marche
      │
      ▼
3. COMPÉTENCES     navigation, conduite, combat, conversation, tâches de métier
      │
      ▼
INTENTIONS         les mêmes que celles d'un joueur
```

**Réflexes** (décidé) : réactions immédiates à un danger soudain (coup de feu, agression, accident). Le choix entre fuir, se défendre ou se figer dépend du profil (violence, stabilité émotionnelle) et des attributs (force, état). Ils prennent la main tant que la menace dure, puis rendent la main à la sélection d'objectif. *Pourquoi : planifier face à un coup de feu serait trop lent et peu crédible ; un réflexe est immédiat et révèle le tempérament.*

Le détail des réflexes est fixé par le domaine combat ([combat.md](combat.md), §1.8) et repris ici (décidé ; *pourquoi : un seul tableau, appliqué par l'IA, évite deux définitions concurrentes*) :

- **six réflexes** : fuir, se figer, se défendre, appeler à l'aide, **protéger** (un proche ou un enfant, §8.4), se soumettre ;
- `score d'un réflexe = base du profil × état (stress, fatigue, ivresse, blessures) × situation (issue, nombre, arme, proche menacé)` ;
- **se figer** ne dure que quelques secondes, puis bascule vers la fuite ou la défense ;
- face à un **mineur**, « se défendre » ne propose que des gestes de contrôle (§8.6) ;
- pour un personnage **incarné**, le réflexe n'est pas imposé au joueur : il est seulement calculé en observation (§5.1).

**Panique de foule** (décidé) : dans un lieu plein, chacun réagit à ce qu'il perçoit **et à la fuite des autres** : voir des gens fuir est traité comme une menace perçue, plus faible que la menace elle-même, ce qui propage la fuite de proche en proche (bousculades, chutes). En simulation simplifiée, la panique se résout par l'évacuation du lieu et des blessures légères tirées au sort. *Pourquoi : la contagion par la perception suffit à produire un mouvement de foule crédible sans règle collective.*

Les **attroupements** de curieux autour d'un incident sont décrits au §13.9.

### 3.2 Sélection d'objectif (décidé)

*Pourquoi : un score d'utilité est simple à régler et à expliquer, et l'inertie évite les personnages qui hésitent sans fin.*

Chaque objectif candidat reçoit un score :

```
score = urgence × poids du profil × faisabilité × (1 + inertie si c'est l'objectif en cours)
```

- **urgence** : niveau du besoin, priorité de l'obligation, échéance d'un engagement ;
- **poids du profil** : §1.4 ;
- **faisabilité** : moyens disponibles, lieu, heure, météo, coût estimé de la meilleure recette. Le coût en argent est mesuré par le **coût relatif** : prix rapporté au **disponible** (solde moins la réserve pour les charges à venir), modulé par la **propension à dépenser** (profil, motivation argent, revenu), fournis par l'économie ([economie.md](economie.md), §6.8) (décidé ; *pourquoi : un même prix ne pèse pas pareil pour un cadre et pour un chômeur en fin de mois*) ;
- **inertie** : un bonus pour l'objectif en cours, qui évite qu'un personnage change d'avis sans arrêt **[à calibrer]**.

La sélection est refaite à chaque **événement** (fin d'une activité, besoin qui franchit un seuil, perception d'une menace, sollicitation par quelqu'un) et à intervalle régulier **[à calibrer]**.

(décidé, v0.8) Le score tient aussi compte de l'**humeur** (§13.2), des souvenirs rappelés (§13.3), des **conclusions** (§13.4), des **valeurs apprises** et des habitudes (§13.6) et des **occasions** (§13.7) ; entre options très proches, le choix est tiré parmi les meilleures (§13.7).

### 3.3 Planification hybride HTN + GOAP (validé)

**HTN** (réseau hiérarchique de tâches) pour la vie courante : des **recettes** écrites à la main qui décomposent un objectif en sous-tâches, jusqu'aux actions élémentaires.

```
Manger
 ├─ cuisiner chez soi      si : à la maison, nourriture disponible, temps
 ├─ acheter un sandwich    si : argent, commerce ouvert à proximité
 ├─ aller au restaurant    si : argent, temps, envie de sortir
 ├─ se faire inviter       si : un proche disponible, relation suffisante
 └─ voler de la nourriture si : profil compatible ou besoin critique
```

- Les recettes générales sont communes à tous ; chaque **fiche de rôle** ajoute ses propres recettes (un policier a une recette « interpeller », un revendeur une recette « écouler la marchandise »).
- Chaque recette a des **préconditions** (lieu, argent, objets, savoirs, heure). Elle **n'écrit pas** de signature (décidé ; G8) : ses étapes élémentaires portent des **effets mécaniques** (dégâts attendus, écritures du grand livre, croyance créée, infraction qualifiée), et sa signature, l'agrégat de leurs effets, est **compilée** avec le paquet de contenus (`ts-data build`) ; celle d'un projet est celle de sa méthode la plus probable (§1.4). *Pourquoi : c'est la seule façon que l'IA, qui juge des plans, et le joueur, qui fait des gestes, utilisent vraiment la même définition.* Les préconditions sont évaluées sur les **croyances** du personnage, jamais sur l'état réel (§3.7) : il croit le commerce ouvert à cette heure, il croit avoir la clé sur lui.
- **Recettes fournies par les autres domaines** (décidé ; *pourquoi : chaque domaine connaît ses règles, l'IA n'en garde que le choix*) :
  - **économiques** ([economie.md](economie.md), §14.3) : chercher un emploi, demander une prestation, chercher un logement, gérer une entreprise (gérant), réapprovisionner, fixer un prix, prendre une mission, emprunter, mettre en gage, receler, blanchir, réclamer une dette, faire opposition ;
  - **sociales** : chaque **acte social** du catalogue ([social.md](social.md), §3.1) est une recette, avec ses préconditions, sa signature et ses effets (§3.6).
  - **face à la violence et à la justice**, selon les règles de [combat.md](combat.md) : appeler la police ou les secours, intervenir, secourir, filmer, se taire et partir (combat §4.2) ; porter plainte ou non (combat §4.5) ; témoigner, se taire, mentir à la police (combat §6.4) ; **se rendre et avouer** après un acte dissonant grave (combat §11.2), en particulier pour un personnage légaliste et honnête ;
  - **crimes** : chaque crime est une recette ([combat.md](combat.md), §10.1), avec une **motivation**, un coût selon le profil, une **occasion** (cible et moyens connus du personnage) et un **risque perçu** estimé **à partir de ses seuls savoirs** (surveillance du lieu, passage, police vue ou connue, témoins probables, peine, son casier), qui entre dans la faisabilité (§3.2). Un crime important est un projet (§3.4) ;
  - **métiers de police et de justice** : recettes propres des fiches de rôle fournies par le domaine combat ([combat.md](combat.md), §14).

**GOAP** (planification par recherche) en **secours** (décidé) : quand aucune recette HTN n'est applicable (situation imprévue, comme une porte fermée à clé sans clé, un commerce fermé, un véhicule en panne), l'IA lance une recherche **limitée** parmi les actions élémentaires pour trouver un enchaînement qui mène au but. *Pourquoi : le cantonner au secours garde la vie courante prévisible et peu coûteuse, tout en évitant qu'un personnage reste bloqué.*

- La recherche est **bornée** en **unités de travail** (profondeur et nombre de nœuds explorés **[à calibrer]**), jamais en temps de calcul, pour éviter les plans absurdes et les coûts imprévus tout en gardant la simulation **reproductible par graine** ([05-architecture.md](../05-architecture.md), §2.10).
- Comme les recettes, la recherche part de ce que le personnage **croit** de la situation (§3.7) : un plan peut donc échouer à l'exécution, et cet échec, perçu, relance la sélection d'objectif.
- **Coût de chemin** (décidé ; G8) : GOAP utilise le **coût de l'agrégat des effets** du chemin pour le profil, par `appraise`. L'agrégat ne fait que croître quand on ajoute une action, si bien que la recherche bornée reste correcte et qu'un plan contraire à la nature du personnage est écarté comme une recette. *Pourquoi : le secours ne doit pas devenir une porte dérobée vers des moyens que les recettes refusent.*
- Elle n'est utilisée qu'en simulation complète (§6) ; en simulation simplifiée ou statistique, l'échec d'une recette se traduit par l'abandon de l'objectif ou un report.
- Un plan GOAP qui réussit souvent peut être **proposé comme nouvelle recette** au responsable conception, qui l'arbitre (A8 ; outil d'analyse, pas d'apprentissage automatique dans le jeu).

### 3.4 Projets à moyen et long terme (décidé)

*Pourquoi : les projets donnent une direction aux vies et sont la source naturelle des objectifs importants qui rapportent de l'expérience.*

Une motivation insatisfaite (§1.2) fait naître un **projet** : un objectif long, découpé en étapes, qui occupe le temps libre et parfois l'agenda (« économiser pour une voiture », « obtenir une promotion », « se venger de X »).

- Un personnage a quelques projets actifs à la fois (1 à 3 **[à calibrer]**).
- Les étapes sont planifiées par les mêmes recettes HTN.
- **Catalogue de projets** (décidé ; *pourquoi : ce sont les grands tournants de vie que les domaines population, économie et social font passer par l'IA*) :

  | Famille | Projets |
  |---|---|
  | **Foyer et famille** ([population.md](population.md), §7.2, §7.5, §8.2) | Avoir un enfant, emménager ensemble, se séparer, quitter ses parents, déménager dans la ville, accueillir un parent âgé, aider un proche malade à l'extérieur (§9.1) |
  | **Économie** ([economie.md](economie.md), §4.5, §4.10) | Créer une entreprise, acheter un logement, économiser pour un achat, rembourser une dette, changer d'emploi ; ils s'appuient sur les **opportunités** et le **crédit** |
  | **Social** ([social.md](social.md)) | Se réconcilier, trouver un partenaire, entrer dans un groupe, y monter en grade, se venger |
  | **Santé** | Se sevrer d'une dépendance (§1.1), se soigner, reprendre une activité physique |
  | **Départ** | Partir définitivement (§9.4) |

- **Projets de départ** (décidé) : sauf **fuite**, qui est immédiate, ils durent plusieurs semaines **[à calibrer]** (recherche d'emploi ailleurs, préavis, cartons, adieux) et se voient (annonces aux proches, souvenirs, intuitions pour le joueur qui incarne le personnage). Leur probabilité est multipliée par le **multiplicateur** du régulateur démographique ([population.md](population.md), §7.5 et §7.6). *Pourquoi : un joueur attaché à un personnage doit voir venir son départ, et peut même le faire changer en l'incarnant.*
- **Projets collectifs** (décidé) : réglés par le domaine social ([social.md](social.md), §6.9) : chaque étape confiée à un membre devient un **engagement** dans son agenda, que l'IA traite comme les autres (§5.3). *Pourquoi : cela réutilise l'agenda, les engagements et les objectifs du personnage sans mécanisme nouveau.*
- Un projet peut être abandonné (découragement, changement de situation) ou réussir, ce qui satisfait la motivation et peut changer la vie du personnage (nouvel emploi, nouveau palier).
- Les projets sont la principale source d'**objectifs importants** du personnage (§5.4).

### 3.5 Compétences d'exécution et apprentissage (validé)

- **Au départ, tout est écrit à la main** : sélection d'objectif, recettes, compétences.
- **Plus tard**, seules les **compétences d'exécution** (combat, fuite, poursuite, conduite, déplacement en foule) pourront être **entraînées** en Python sur le cœur ([05-architecture.md](../05-architecture.md), §4). Chaque compétence entraînée tient compte des attributs et de l'état du personnage : un vétéran reposé et un passant terrifié n'utilisent pas la même compétence de la même façon.
- **Les décisions ne sont jamais apprises** : la sélection d'objectif et la planification restent écrites à la main, pour garder des personnalités maîtrisées et explicables.
- Les compétences de métier (cuisiner, réparer, vendre) restent **scriptées**.
- (décidé, v0.8) Les **poids** des décisions sont calibrés hors ligne, jamais appris en jeu (§13.13). Les gestes du **naturel** (§13.7) et les groupes d'occasion (§13.9) font partie des compétences d'exécution qui pourront être entraînées.

### 3.6 Dialogue (décidé)

*Pourquoi : c'est la règle du socle, et garder abstraites les conversations entre PNJ évite un coût de modèle de langage démesuré.*

Conformément au socle ([05-architecture.md](../05-architecture.md), §2.9), **le cœur décide de ce que le personnage veut dire** (accepter, refuser, se méfier, négocier, mentir), à partir de ses besoins, de son profil, de sa mémoire et de ses relations ; le service de dialogue ne fait que le formuler. Les conversations entre PNJ restent **abstraites** (échange d'informations et effet sur la relation, sans texte), sauf à portée d'écoute d'un joueur. Le détail relève du domaine social ([social.md](social.md), §3).

**Choix des interlocuteurs et des sujets** (décidé) : une conversation est une suite d'**actes sociaux** ([social.md](social.md), §3.1), chacun choisi comme une recette. L'IA décide **à qui parler** et **de quoi** par la même sélection d'utilité (§3.2) :

- le **besoin de lien social** oriente vers les proches et les personnes appréciées ;
- les **projets** en cours orientent vers ceux qui peuvent aider (un employeur, un prêteur, un allié) ;
- les **informations à transmettre** (une nouvelle importante pour l'interlocuteur, un témoignage) sont choisies selon leur intérêt pour lui, la relation, et les **engagements de discrétion** ;
- les **obligations** imposent certains échanges (le client, le collègue, l'enseignant).

*Pourquoi : un seul mécanisme de choix pour tout ce que fait le personnage, y compris parler.*

L'état du discours, la mémoire de ce qu'on s'est dit, la voix propre de chaque personnage et le ton du joueur sont décrits au §13.10 (décidé, v0.8).

**Vocalisation d'une réplique** (décidé ; [interface.md](interface.md), §5.4 et §15.2 ; [direction-artistique.md](direction-artistique.md), §5.5) : chaque réplique s'accompagne d'une courte vocalisation non verbale (rire, soupir, « hm », exclamation, et aussi toux ou cri de douleur selon l'état du corps). Elle est choisie par une **règle du cœur**, appliquée à la résolution de l'acte social et non par la décision de l'IA, pour **tout orateur**, qu'il soit piloté par l'IA ou par un joueur, et jamais par le joueur.
- **Entrées** : le **ton** de l'acte (§13.10), l'**humeur** et les pensées fortes du moment (§13.2), l'issue de l'échange (une offense reçue, une bonne nouvelle), l'état du corps (douleur, ivresse, rhume) et la **voix** du génome d'apparence.
- **Sortie** : une **catégorie** de vocalisation, une **intensité**, valeurs d'une énumération stable (§2.6), et un **indice de variante** (un octet). Le client prend l'échantillon de cet indice dans la banque classée par âge, sexe apparent et timbre, **sans aucun tirage**, et deux auditeurs entendent le même son.
- Le tirage utilise un **flux d'aléa dédié** (`cosmetic/vocalization`), à la clé `(orateur, identifiant de l'acte)`, jamais l'ordre de traitement dans le tick (décidé ; revue d'architecture de clôture, S-4 et T2-4). C'est un seul domaine cosmétique pour la catégorie, l'intensité et l'indice de variante, comme les générations à l'exécution de [direction-artistique.md](direction-artistique.md), §8.3 : il est reproductible et ne déplace aucun autre tirage. Une réplique peut n'avoir aucune vocalisation, selon la voix et le ton **[part à calibrer]**.

*Pourquoi : le pilote n'entre dans aucune règle ; si l'IA choisissait la vocalisation des PNJ et que le joueur ne choisissait rien, elle ne passerait pas par le même chemin selon le pilote et trahirait qui parle. Une règle unique du cœur, nourrie par le ton et l'humeur, garantit le même rendu pour tous.*

**Demandes d'actes intimes** (décidé ; A6, [social.md](social.md), §9.4) : quand un personnage, incarné ou non, demande un acte intime à un PNJ, celui-ci répond par son IA, à travers la **même demande d'acceptation** que celle qu'un joueur recevrait, avec un délai tiré dans la **même distribution** qu'une réponse de joueur. La réponse dépend de la relation (affection, attirance, confiance), du profil, du statut du couple de chacun ([social.md](social.md), §1.6), du lieu et du moment. Un personnage ivre au-delà d'un seuil **[à calibrer]**, inconscient, en crise (§13.2) ou sous la menace refuse toujours, et aucune demande n'existe envers un mineur (règle absolue). Les actes restent abstraits. *Pourquoi : une demande qui ne suivrait pas le même chemin selon le pilote révélerait qui pilote l'autre, et le consentement doit être celui du personnage.*

### 3.7 Validation des intentions en deux étages (décidé ; S4, arbitrage A2)

*Pourquoi : une liste d'actions calculée sur l'état réel révélerait ce que le personnage ne peut pas savoir (porte verrouillée, poche vide), au joueur comme à l'IA ; la calculer sur ce qu'il sait garde l'écran honnête et l'IA sans omniscience, avec une seule règle pour les deux pilotes ([revues/securite.md](../revues/securite.md), §4.1.2).*

Toute intention, qu'elle vienne de l'IA ou d'un joueur, passe deux étages.

1. **Admissibilité épistémique**, avant l'exécution : les préconditions sont évaluées sur ce que le personnage **perçoit et sait** (perception courante, croyances, souvenirs, savoirs, dont les savoirs opaques comme références, §4.4).
   - C'est cet étage qui produit la **liste des actions possibles** envoyée à l'interface, le **signe de réticence** (§5.6), le **coût de synchronisation estimé** utilisé par le point de rupture (§5.3 et §5.6 ; [revues/equilibrage.md](../revues/equilibrage.md), E1), qui est le **verdict d'`appraise`** sur les croyances du personnage, avec, pour un pilote `Joueur`, le **masque d'imputabilité** de la garde (justifications et décalages d'état causés par le personnage ou par le cercle du joueur, neutralisés ; [revues/signatures.md](../revues/signatures.md), §3.7) (décidé), et les **masques d'actions** de l'entraînement des compétences (§3.5).
   - Pour un pilote `IA`, la sélection d'objectif, les recettes et la recherche GOAP ne s'appuient que sur cet étage : **l'IA ne choisit jamais une action sur la base d'un état qu'elle ignore**.
   - Pour un pilote `Joueur`, cet étage applique en plus les **règles absolues** et la **garde hors fiction** de `ts_pilot` (E35) : cercle et flux liés, conflit d'intérêts, point de rupture, plafonds du persistant, vote opaque. L'IA en observation fournit le coût estimé (§5.1) ; le cœur ne reçoit aucune donnée de compte, seulement le résultat de la garde.
2. **Résolution réelle**, à l'exécution : l'état réel décide. Un échec est **perceptible dans le monde** (la poignée résiste, la main ne trouve rien dans le tiroir, le coffre ne s'ouvre pas) et devient aussitôt une **croyance corrigée** du personnage (§4.2), qui relance la sélection d'objectif (§3.2) : l'IA cherche une autre recette, comme un habitant devant une porte fermée. Les règles absolues, la récusation et les flux liés y sont vérifiés une seconde fois (S4).

**États inconnus** (décidé) : quand le personnage ignore l'état d'une chose, il le **présume** d'après ce qui est public ou habituel : un commerce est ouvert à ses heures affichées, la porte d'un logement d'autrui est fermée à clé, le contenu d'un tiroir ou d'une poche est inconnu, donc « fouiller » reste possible. *Pourquoi : c'est ainsi qu'agit une vraie personne, et une présomption tirée de données publiques ne révèle rien de caché.*

**Coût et déterminisme** (décidé) : l'IA raisonnait déjà sur la perception et la mémoire du personnage (§0, principe 3), donc l'étage épistémique ne lui ajoute pas de calcul ; la résolution réelle n'est faite qu'une fois, à l'exécution. Les deux étages sont déterministes, ce qui garde la reproductibilité par graine ([05-architecture.md](../05-architecture.md), §2.10). Les refus renvoyés au client utilisent une **liste fermée de codes** (impossible, hors de portée, occupé, interdit), sans raison interne. *Pourquoi : la séparation des deux étages est une forme du code, fixée dès la première règle, et non un calcul de plus.*

---

## 4. Mémoire et relations

### 4.1 Perception (décidé)

*Pourquoi : c'est la condition pour que les témoins, la discrétion et les savoirs aient un sens.*

Seul ce que le personnage **perçoit** entre en mémoire : la vision et l'ouïe sont filtrées par l'attribut de perception, l'attention (occupé ou non), le stress, l'éclairage et la distance. Un témoin distrait ne se souvient pas du visage du voleur.

**Dernier contact et portrait** (décidé ; [interface.md](interface.md), §4.7 et §15.2 ; [05-architecture.md](../05-architecture.md), §2.5 ; revues de clôture : architecture C-5, sécurité C1) :

- **Quatre dates par relation**, à la journée, gardées aussi dans la forme compacte des visages et des noms :
  - le **dernier échange**, qui sert à la dérive ([social.md](social.md), §1.1 et §1.4) ;
  - la **dernière perception**, quelle que soit la précision ;
  - la dernière perception à la précision **« tenue » ou mieux** ;
  - la dernière perception à la précision **« visage »**.
  C'est environ 4 × 2 octets par relation **[à mesurer]**.
- **Portrait reconstitué par précision** : chaque groupe de champs du portrait est reconstitué à la date de **sa** précision. La silhouette et les couleurs dominantes sont reconstituées à la dernière perception ; les pièces de la tenue, la coiffure et les signes visibles à la dernière date « tenue » ; le visage et la peau à la dernière date « visage ». Le serveur s'appuie sur le génome, au vieillissement déterministe, et sur l'historique daté de la coiffure, de la tenue habituelle et des signes distinctifs (§2.6 ; [population.md](population.md)). Le personnage ne voit donc jamais une coupe ou un visage vieilli qu'il n'a pas vus.
- **Seulement sur identification** : ces dates, et l'apparence datée qui en découle, ne changent que quand le personnage **identifie** la personne. L'identification est un test du cœur sur ce qui est perçu :
  - précision « visage » à découvert ;
  - ou silhouette, voix ou démarche d'un **proche**, selon la familiarité.
- **Sans identification**, aucune relation n'est touchée. La perception peut au plus créer un **« inconnu vu »**, que rien ne rattache à la personne sans une identification ultérieure. Un portrait ne montre jamais un état vu pendant que la personne était masquée et non identifiée.
- **Calcul à la demande** : les portraits ne sont recalculés qu'à l'ouverture de l'écran, sans notification au moment de la perception. Un test vérifie que la vue, les écrans et le journal de conversation sont identiques, que la silhouette aperçue soit la personne ou un tiers, tant que l'identification échoue.
- La dérive des relations reste calculée sur les seuls **échanges** : apercevoir quelqu'un met à jour son portrait, pas l'affection.

*Pourquoi : on ne sait qu'une connaissance a vieilli ou changé de coiffure qu'en la revoyant assez bien pour le voir, et on ne sait que la silhouette de la nuit était elle que si on l'a reconnue ; quatre dates coûtent bien moins qu'une image par relation.*

### 4.2 Les quatre mémoires (décidé)

*Pourquoi : des faits structurés plutôt que du texte restent cohérents quel que soit le modèle de dialogue et se traitent en masse.*

| Mémoire | Contenu | Forme |
|---|---|---|
| **Souvenirs** | Événements vécus : qui, quoi, où, quand, émotion ressentie, importance | Faits structurés, jamais du texte |
| **Savoirs** | Lieux connus, personnes connues (nom, métier, domicile), compétences, secrets, codes ; les informations sur le monde sont des **croyances** (décidé) | Faits structurés ; chaque croyance porte une **certitude**, une **origine** (vu, source directe, rumeur), une **source** et un éventuel **engagement de discrétion** ([social.md](social.md), §4.1) ; les secrets sont **opaques** (§4.4) |
| **Relations** | Pour chaque personne connue : degré de connaissance, affection, confiance, **respect** et **crainte** (deux valeurs distinctes), liens (famille, collègue, voisin), **bilan** (services, torts, promesses) et **date du dernier contact** (décidé) | Relation orientée, au format du domaine social ([social.md](social.md), §1.1) ; elle alimente le graphe social et la composante « Réseau » de l'influence |
| **Engagements** | Rendez-vous, promesses, dettes, services à rendre | Entrées de l'agenda, avec échéance et personne concernée |

Les **relations** d'un PNJ (internes au monde) sont distinctes de la **familiarité** d'un joueur ([04-joueur.md](../04-joueur.md), §6), qui est propre au joueur.

**Le souvenir d'un témoin** (décidé) : un témoin qui ne connaît pas l'auteur d'un fait garde une **description structurée** (sexe apparent, âge approximatif, taille, corpulence, vêtements, signe distinctif, véhicule, direction de fuite, arme), chaque trait avec sa propre précision ; un stress élevé fait retenir l'arme plutôt que le visage ; le souvenir s'efface, se déforme quand il est raconté et peut être contaminé par une rumeur ([combat.md](combat.md), §4.3). Un témoin qui reconnaît l'auteur le désigne directement. *Pourquoi : c'est une croyance comme les autres, avec une certitude par trait, ce qui rend les témoignages imparfaits sans mécanisme nouveau.*

**Plan de mémoire des bâtiments** (décidé) : chaque personnage mémorise les **étages** des bâtiments qu'il a vus, à la granularité bâtiment × étage, pas pièce par pièce ; l'interface s'en sert pour montrer les étages connus ([interface.md](interface.md), §6.3). Comme les autres savoirs, ce souvenir s'efface lentement s'il n'est pas entretenu. *Pourquoi : un indicateur par étage coûte très peu pour 10 000 personnages, et suffit à garder l'écran honnête.*

**Compléments v0.8** (décidé) : rappel des souvenirs (§13.3), conclusions et thèmes de vie (§13.4), modèle d'autrui (§13.5), valeurs apprises (§13.6) et attentes (§13.8) s'appuient sur ces quatre mémoires sans en créer de nouvelle.

### 4.3 Oubli et consolidation (décidé)

*Pourquoi : borner la mémoire est indispensable pour 10 000 personnages, et la résumer dans les relations en garde l'essentiel.*

- La rétention d'un souvenir dépend de son **importance** et de l'**émotion** ressentie, et de l'attribut de **mémoire à long terme**.
- Les souvenirs anciens sont **résumés dans les relations** puis effacés : « il m'a aidé trois fois » s'inscrit dans le **bilan** de la relation et devient de la confiance.
- Chaque personnage garde un nombre limité de souvenirs détaillés (de l'ordre de 100 à 300 **[à calibrer]**), ce qui borne le coût de stockage.
- Les savoirs peu utilisés s'oublient progressivement (un code jamais tapé finit par s'oublier), selon le modèle de désentraînement.

### 4.4 Savoirs opaques (décidé)

*Pourquoi : c'est l'application directe de la parade du socle contre l'espionnage par incarnation.*

Application de la règle du socle ([04-joueur.md](../04-joueur.md), §7.1) :

- Un savoir secret est une **référence** dans le cœur (« connaît le code du coffre n° 123 »), jamais une valeur affichable.
- La planification s'en sert comme **précondition** (« ouvrir le coffre » demande de connaître son code).
- Le client ne reçoit que la **possibilité d'action** (« ouvrir le coffre »), jamais le code.
- Le service de dialogue reçoit seulement « le personnage connaît le code », pas sa valeur : il ne peut pas le révéler.
- Un savoir peut se **transmettre** d'un personnage à un autre dans le monde (confidence, trahison, interrogatoire) : c'est un transfert de référence entre personnages, qui reste opaque pour le joueur, même s'il incarne l'un des deux.
- **Couche cachée tirée par monde** (décidé ; S3, E25, arbitrage A1) : à sa création, chaque monde, solo ou serveur, tire avec sa propre graine les **détenteurs des savoirs opaques**, le **contenu des caches et des coffres**, les **informateurs** et les **policiers corruptibles** (parmi ceux dont le profil le permet). Pour l'IA, ce ne sont pas des données à part : ce sont des savoirs, des objets et des rôles secondaires qu'elle utilise comme les autres. *Pourquoi : ce qu'on apprend dans un monde ne doit pas révéler les secrets d'un autre, et une étape déterministe de la création garde la reproductibilité par graine.*
- Les savoirs opaques restent des **références** : il n'existe aucune valeur à extraire, ni du client, ni du service de dialogue. Les actions qui s'en servent sont admises d'après les savoirs du personnage, jamais d'après l'état réel (§3.7).

### 4.5 Souvenirs d'une incarnation : souvenir complet et dissonance (validé)

Le personnage se souvient de **tout** ce qu'il a fait pendant une incarnation, comme de sa propre vie. Les actes **contraires à sa nature** (signature éloignée de son profil) sont marqués **dissonants** (mécanisme décidé ; *pourquoi : réagir selon son profil rend la dissonance crédible sans changer la personnalité, comme l'exige le socle*) :

- ils lui causent du **stress** ou du **regret**, ou il les **justifie**, selon son profil (un personnage peu bienveillant se trouve des excuses, un personnage bienveillant culpabilise) ;
- il peut chercher à **réparer** : s'excuser, rembourser, rendre ce qu'il a volé, éviter la personne ;
- (décidé ; [revues/signatures.md](../revues/signatures.md), §3.10 et §5.5) le souvenir dissonant **garde les deux raisons** du verdict d'`appraise` (« il a menti à sa sœur ») ; il est **réévalué** si le personnage apprend des conséquences réelles (la porte qu'il a laissée ouverte a servi à un cambriolage) : la culpabilité peut grandir, sans que la synchronisation soit redébitée ; la **réparation** est choisie d'après l'**effet dominant** : avouer ou s'excuser pour une tromperie ou une trahison, rendre ou faire opposition pour une prise ou un don, s'excuser, éviter ou se rendre à la police pour une violence, rattraper l'obligation manquée, prévenir ou changer la serrure pour une facilitation. *Pourquoi : les raisons du verdict disent déjà ce qu'il y a à réparer, sans table écrite pour chaque acte* ;
- **sa personnalité ne change pas** ([04-joueur.md](../04-joueur.md), §7) : il vit seulement avec les conséquences.

Les autres personnages se souviennent de ce qu'ils ont vu, sans savoir qu'un joueur pilotait.

---

## 5. Incarnation, reprise et objectifs du personnage

### 5.1 L'IA en observation (décidé)

*Pourquoi : c'est ce qui permet la reprise sans rupture et le calcul des objectifs du personnage pendant l'incarnation.*

Pendant une incarnation, le pilote est `Joueur(id)`, mais l'IA du personnage **continue de tourner en observation**, à faible coût. Elle ne produit aucune intention ; elle :

- fait évoluer les besoins, les émotions et l'agenda (obligations manquées, retards) ;
- enregistre dans la mémoire du personnage ce qu'il fait et perçoit, comme siens, en marquant les actes dissonants ;
- enregistre les **engagements** pris par le joueur ;
- tient à jour la liste des **objectifs que le personnage poursuivrait lui-même** (§5.4), ce qui sert aux bonus d'expérience et aux intuitions (§5.5) ;
- calcule, aux **moments clés d'un affrontement** (début, escalade, arme sortie, blessure), le **réflexe et la décision qu'elle aurait choisis**, et les enregistre dans le journal de l'incarnation : ils servent à la synchronisation et au **critère d'imprudence** qui juge la mort d'un personnage incarné ([combat.md](combat.md), §1.8 et §11.3) (décidé ; *pourquoi : seule l'IA du personnage peut dire ce qu'il aurait fait, et ne le calculer qu'aux moments clés en limite le coût*) ;
- évalue, pour **chaque intention** du joueur avant son exécution, son **coût de synchronisation estimé**, par le calcul du signe de réticence (§5.6), sous forme de **coût marginal** : `appraise` reçoit l'**épisode** du personnage, sa conduite récente (pour chaque type d'effet et chaque contrepartie, la somme des magnitudes, sur une heure de jeu, ou sur 30 jours pour les dons, prises et abus), et l'acte coûte `coût(épisode + acte) − coût(épisode)` (décidé ; G9 ; *pourquoi : un acte découpé en gestes coûte alors exactement autant que l'acte entier, et l'épisode, composant du personnage d'au plus 8 entrées, est le même pour l'IA et pour le joueur*) ; la garde hors fiction le compare à la synchronisation restante (point de rupture, §5.3 ; E1, E35) ; ce coût et les refus de la garde sont enregistrés dans le journal de l'incarnation (E38) (décidé ; *pourquoi : l'IA est la seule à connaître le profil du personnage, et le calcul existe déjà pour le signe de réticence*) ;
- fournit à l'interface les aides du joueur incarné (§5.6).

### 5.2 Gestes automatiques pendant l'incarnation (validé)

Les besoins **triviaux** (soif, vessie, confort, gestes d'hygiène courts) sont satisfaits automatiquement par de **courts gestes** quand l'occasion se présente (boire un verre d'eau en passant dans la cuisine, aller aux toilettes au bureau), sauf si le joueur s'en empêche. Le joueur ne gère que les besoins importants : manger, dormir, santé, sécurité, lien social, détente, stimulation.

(décidé) Les gestes automatiques ne se déclenchent que si le personnage est au repos et ne prennent que quelques secondes **[à calibrer]** ; le joueur peut toujours les interrompre. *Pourquoi : ainsi ils ne gênent jamais une action en cours du joueur.*

(décidé) Chaque geste est **annoncé** environ 2 secondes **[à calibrer]** avant de commencer, pour que le joueur puisse l'annuler, et un réglage permet de les mettre en « me demander » ou de les désactiver ; désactivés, les besoins triviaux sont gérés à la main ([interface.md](interface.md), §5.3). *Pourquoi : c'est la forme concrète du « sauf si le joueur s'en empêche ».*

### 5.3 Reprise sans rupture (décidé)

*Pourquoi : sans téléportation ni retour en arrière, les autres joueurs ne voient aucune incohérence ; les réflexes passent d'abord parce qu'une éjection peut survenir en pleine action.*

Quand le joueur rend le personnage, en est éjecté ou se déconnecte, l'IA redevient pilote **dans la même image** (même tick de simulation), sans téléportation ni retour en arrière :

1. **Bilan de situation** : où il est, quelle heure, état de ses besoins, menaces, engagements nouveaux.
2. **Si le personnage n'est pas au repos** (éjection en plein combat, en conduisant) : les réflexes et les compétences prennent la main immédiatement (se défendre, fuir, garer le véhicule en sécurité).
3. **Re-planification** : sélection d'objectif normale. L'agenda reprend au bloc en cours : s'il devait être au travail, il y va, en retard.
4. **Réaction à ce qui s'est passé** : les souvenirs dissonants agissent sur ses émotions et ses projets (§4.5).

**Éjection par le point de rupture** (décidé ; E1) : quand la garde hors fiction refuse un acte dont le coût estimé dépasse la synchronisation restante, l'acte **n'a pas lieu**. Le personnage **hésite puis renonce** par un geste naturel (il repose l'arme, referme le coffre, raccroche), que l'IA choisit parmi les façons d'abandonner l'acte en cours ; le joueur est éjecté et l'IA reprend dans le même tick, selon les étapes ci-dessus. Les **planchers de rupture** garantissent (décidé ; G7) qu'un acte **extrême pour le profil** coûte plus que la synchronisation maximale : quatre règles sur les effets (violence grave, don massif, facilitation contre les siens, mise en danger mortelle de soi, qui ne vaut que pour un **personnage prudent** : jugement ≥ 0,4, soit `JP` ≤ −0,4, ou stabilité ≤ −0,2 avec violence ≤ 0 ; décidé, M5, [revues/signatures-contenu.md](../revues/signatures-contenu.md), §3.2 ; *pourquoi : l'axe de stabilité réunit l'anxieux et l'impulsif, et seul l'anxieux sans goût de la force est prudent*), évaluées en dernier sur le **profil de base**, avec les seules justifications non imputables ([revues/signatures.md](../revues/signatures.md), §3.8) : il est impossible même à synchronisation parfaite. Le personnage garde le souvenir d'une hésitation, **sans dissonance** pour l'acte refusé, qui n'a pas eu lieu ; les gestes qui l'ont précédé gardent leur marque. Le point de rupture ne concerne que les pilotes `Joueur` : l'IA a déjà sa propre limite, le seuil de choix des moyens (§1.4). *Pourquoi : le renoncement doit ressembler à un comportement ordinaire du personnage, et on ne regrette pas un acte qu'on n'a pas commis.*

**Engagements pris par le joueur (validé)** : ils entrent dans l'agenda et la mémoire du personnage, qui les **tient selon son caractère**, exactement comme ceux qu'il aurait pris lui-même. Il les tient plus ou moins selon son honnêteté, son organisation (Jugement ↔ Perception), sa relation avec la personne et le coût de l'engagement pour lui. Un personnage honnête paiera la dette contractée par le joueur ; un menteur pourra l'« oublier ».

**Engagements au profit du cercle du joueur** (décidé ; E6) : un engagement pris pendant une incarnation au profit d'un personnage du **cercle du joueur** ou d'un personnage piloté par un autre joueur (contrat immédiat, promesse de paiement, reconnaissance de dette) est **réévalué à la reprise comme une décision lourde**, avec un **délai de rétractation de 7 jours de jeu** **[à calibrer]**, pendant lequel l'IA peut l'annuler **sans indemnité** s'il est dissonant ; dans le monde, c'est le droit de rétractation ordinaire. L'IA ne connaît pas le cercle, qui est une donnée hors fiction : la garde hors fiction marque l'engagement « à réévaluer », et l'IA le réévalue alors comme une décision lourde dissonante (« se rétracter », ci-dessous), **sans le biais de la parole donnée** : un personnage honnête n'est pas tenu par une dette, une promesse de paiement ou un contrat signés à sa place au profit du cercle ; seule compte la dissonance de l'engagement pour lui ([social.md](social.md), §1.8 et §8.4). *Pourquoi : un contrat signé pour un complice transporte de la valeur dans le temps, et le personnage reste libre de ne pas tenir ce qu'il n'aurait jamais signé.*

**Réparer après une incarnation** (décidé) : selon son profil et l'importance de l'acte, l'IA peut choisir, comme objectifs ordinaires (§3.2), de :

- **s'expliquer** auprès des témoins d'actes dissonants (« je n'étais pas moi-même »), s'excuser ou réparer ; une explication acceptée rend une partie du crédit perdu ([social.md](social.md), §5.4) ;
- **se rétracter** pendant le délai d'une décision lourde dissonante (vente, mariage, démission, gros prêt), comme pour un engagement qu'il ne veut pas tenir ([social.md](social.md), §8.4). Pour un **mariage ou un pacs** dont l'autre partie appartient au cercle du joueur ou est pilotée par un autre joueur, la rétractation pendant le délai de publication se décide **sans le biais de la parole donnée** : seule compte la dissonance du mariage pour le personnage (décidé ; E31 ; *pourquoi : un mariage arrangé entre complices ne doit pas tenir au nom d'une honnêteté que le personnage n'a jamais engagée*) ;
- **réparer une opération économique dissonante** : réclamer l'argent, faire opposition à un virement, annuler une vente non encore signée, porter plainte ([economie.md](economie.md), §12.2).

*Pourquoi : ce sont les moyens réalistes de « vivre avec les conséquences » sans annuler ce qui s'est passé, comme le veut le socle.*

### 5.4 Les objectifs du personnage (décidé)

*Pourquoi : rattacher les objectifs à ce que l'IA poursuivrait vraiment empêche le joueur de fabriquer ses propres sources d'expérience.*

**Définition** : un objectif du personnage est un objectif que **son IA poursuivrait elle-même** à ce moment-là, c'est-à-dire présent dans sa liste d'objectifs candidats : obligation de l'agenda, besoin important, étape d'un projet, engagement. Il est accompli quand la simulation constate l'événement correspondant (le colis est livré, la dette est remboursée).

Barème, dans la fourchette du socle ([04-joueur.md](../04-joueur.md), §1.1 : 10 à 200 XP), sauf les tâches de travail, plus petites ([revues/equilibrage.md](../revues/equilibrage.md), E11) **[à calibrer]** :

| Type d'objectif | Exemples | XP |
|---|---|---|
| Besoin important ou geste de routine | Manger, dormir à une heure raisonnable, faire ses courses | 10 |
| Tâche de travail (décidé ; E11) | Servir un client, terminer une réparation, livrer un colis | 1 à 5 selon sa durée (2 en moyenne), dans la limite de **30 par heure réelle**, avec des rendements décroissants au-delà de **4 heures** de travail dans la journée |
| Obligation du jour | Être à l'heure (20), finir sa journée ou sa mission (20 à 50), aller chercher les enfants | 20 à 50 |
| Étape de projet | Décrocher un entretien, rembourser une partie de la dette, renouer le contact | 50 à 100 |
| Aboutissement de projet, événement majeur | Obtenir la promotion, se réconcilier, conclure la grosse affaire | 100 à 200 |

**Garde-fous** :

- les objectifs sont **générés par l'IA du personnage**, jamais choisis par le joueur ;
- un objectif accompli **par un moyen contraire à sa nature** (voler pour manger quand on est honnête) ne rapporte aucun bonus, en plus de faire baisser la synchronisation ;
- les objectifs de routine ont un **plafond quotidien** et des rendements décroissants ([04-joueur.md](../04-joueur.md), §1.1) ;
- un objectif provoqué artificiellement (affamer volontairement le personnage pour le faire manger) ne rapporte pas plus que le plafond de routine ;
- **barème du travail** (décidé ; E11) : une tâche créée par un personnage du **cercle** du joueur (le complice qui commande dix cafés) ne compte pas ;
- **objectifs liés** (décidé ; E13) : le bonus d'un objectif est
  1. **nul** si l'événement qui l'accomplit, ou la situation qu'il résout (dispute, panne, dette, besoin), a été **causé par une décision d'un personnage du cercle** du joueur dans les 30 jours de jeu, sauf le personnage incarné lui-même, dans la même incarnation, qui mène son propre travail ou son propre projet ;
  2. ramené au **barème de routine** (10, sous plafond quotidien) si l'objectif est un **engagement que le joueur a fait prendre** au personnage (promesse, rendez-vous), sauf les engagements de travail (missions, contrats), qui suivent le barème du travail ;
  3. **proratisé** pour les étapes et aboutissements de projet : bonus × part des étapes accomplies pendant des incarnations de **ce joueur**, avec un minimum de 20 % ;
  4. **réduit de moitié** si l'objectif est accompli avec une contrepartie pilotée par un autre joueur, et limité à **un aboutissement par paire de comptes et par 30 jours**.

  Ces valeurs sont **[à calibrer]**. *Pourquoi : un objectif doit venir de la vie du personnage, pas d'une mise en scène du joueur ou de ses complices, ni d'un chemin que l'IA a fait seule.*

**Partage des rôles** (décidé) : ces règles sont appliquées **hors fiction** par le méta-jeu (`ts_meta`, E36), à partir du journal d'incarnation (E38). L'IA ne connaît ni le cercle ni les comptes ; elle fournit seulement, pour chaque objectif, la **situation qui l'a fait naître** et son auteur selon les liens de cause du cœur (qui a provoqué la panne, qui a fait la promesse), et, pour chaque projet, la liste de ses étapes avec leur date, que le journal rapproche des incarnations. *Pourquoi : l'IA reste une simulation du monde qui ignore qui joue, et le méta-jeu garde seul les règles qui portent sur les joueurs.*

### 5.5 Intuitions (validé)

Le joueur ne voit pas de liste d'objectifs : le personnage **ressent** ses obligations et ses envies sous forme d'**intuitions** (« vous devriez être au travail », « votre frère vous manque », « cette dette vous pèse ») (décidé ; *pourquoi : lier leur précision à la synchronisation récompense le joueur qui joue juste*) :

- la **précision** des intuitions augmente avec la **synchronisation** et la **familiarité** du joueur avec ce personnage ;
- une intuition ne révèle jamais un savoir opaque (« vous connaissez un moyen d'ouvrir ce coffre », jamais le code) ;
- les intuitions les plus fortes correspondent aux objectifs au score le plus élevé (§3.2) ;
- (décidé, v0.8) elles puisent aussi dans les pensées, les thèmes de vie et les valeurs apprises (« cette dispute vous trotte dans la tête », « vous n'aimez pas ce quartier la nuit ») (§13.2, §13.4, §13.6).

Les intuitions sont reprises par le socle : la règle des rôles invisibles ne concerne que les personnages que le joueur n'incarne pas ([04-joueur.md](../04-joueur.md), §6). La forme exacte relève du domaine interface ([interface.md](interface.md), §4.1).

### 5.6 Ce que l'IA fournit à l'interface du personnage incarné (décidé)

*Pourquoi : toutes ces aides viennent de calculs que l'IA fait déjà pour elle-même ; les fournir au joueur l'aide à jouer juste sans lui montrer le profil, et leur fiabilité récompense la synchronisation.*

Un **indice de précision** combine la synchronisation courante et la meilleure familiarité du joueur avec le personnage **[pondération à calibrer]** ([interface.md](interface.md), §4.1). Il règle la fiabilité de toutes les aides ci-dessous.

| Aide | Ce que fournit le cœur |
|---|---|
| **Intuitions** | Sous forme **structurée** : catégorie (obligation, besoin, relation, argent, projet, danger, morale), sujet, intensité, niveau de précision (vague, clair, précis). Les plus fortes correspondent aux objectifs au score le plus élevé (§3.2) ; la touche **Réfléchir** demande les **trois plus fortes** du moment. L'interface les formule avec des gabarits |
| **Ce qu'il dirait** | Dans une conversation, les **trois actes** que l'IA choisirait elle-même (§3.6), d'autant plus fiables que l'indice est élevé ; aucun en dessous du niveau « clair » ([interface.md](interface.md), §5.4) |
| **Signe de réticence** | Pour **chaque action possible** proposée au joueur (roue d'actes, menu des actions, texte libre classé), le verdict d'`appraise` pour le profil du personnage, évalué **à l'avance** avec le même calcul que le choix des moyens (§1.4). Vers le client ne partent que **quatre niveaux** quantifiés, **0 à 3** (aucune, légère, nette, forte ; seuils de coût 0,01, 0,05 et 0,20, M8, [revues/signatures-contenu.md](../revues/signatures-contenu.md), §1.3), l'indicateur de **rupture**, à part (coût supérieur à la synchronisation restante ou à 1) et un **code de raison** que l'interface met en mots (« Elle ne ment pas à ses proches. ») ; aucun coût, aucune gravité ni aucune valeur du profil (décidé ; G24 ; *pourquoi : tout ce qui part vers le client est public, et un coût exact permettrait de reconstituer un profil*) ; au début d'une incarnation, seuls les écarts flagrants sont signalés ([interface.md](interface.md), §2.3). Ce calcul est fait à l'**étage épistémique** (§3.7). Le **même coût estimé** sert au **point de rupture** : un acte dont le coût dépasse la synchronisation restante n'est pas exécuté, et le personnage renonce par un geste naturel (§5.3) ; pour un tel acte, le signe est toujours visible et sans ambiguïté (décidé ; E1) |
| **Annonce des gestes automatiques** | Le geste prévu, environ 2 secondes avant (§5.2) |
| **Capacités** | Pour les **trois à cinq compétences notables** du personnage incarné, un degré parmi cinq (faible, moyen, bon, très bon, exceptionnel), calculé d'après la valeur rapportée à la population ([02-personnages.md](../02-personnages.md), §3.1), jamais un chiffre ; la précision suit l'indice ([interface.md](interface.md), §4.2) |
| **Impossibilités** | La raison, quand une intention n'est pas proposée et que le joueur doit la comprendre (« Il ne ferait pas ça. ») ; pour un acte refusé par le point de rupture, « Il ne peut pas s'y résoudre. » (E1). La liste des actions proposées est celle de l'étage épistémique (§3.7) : une action absente ne révèle jamais un état que le personnage ignore |

Aucune de ces aides ne révèle un savoir opaque, ni les sentiments réels des autres personnages : seulement ce que le personnage incarné **devine** selon sa perception et son intelligence sociale.

**Cible des raisons** (décidé ; [interface.md](interface.md), §4.1 ; [revues/signatures-contenu.md](../revues/signatures-contenu.md), §9.1) :

- Avec chaque code de raison, le cœur envoie la **classe** de la cible (un proche, un inconnu, quelqu'un de fragile, une enseigne).
- Il n'envoie l'entité visée que si le personnage la **connaît**. Une cible inconnue reste désignée par sa seule classe.
- Les variables des gabarits sont remplies par des **codes fermés** de groupe, d'infraction et de besoin, jamais par du texte libre.

*Pourquoi : un gabarit ne doit pas donner au joueur une identité que son personnage ignore.*

**Réticence et savoirs opaques** (décidé ; revue d'équilibrage de clôture, C6) :

- Une phrase de raison ne cite **jamais** une relation ni un fait qui est un **savoir opaque** (§4.4) : l'amant secret ou l'enfant caché d'un personnage incarné ne se révèle pas par la raison d'un refus.
- **Cible liée par un secret** (décidé ; R1, revue d'équilibrage) : la classe **apparente** de la relation (un voisin, un collègue) sert à tout ce qui est montré ou proposé : le **niveau affiché**, l'**indicateur de rupture** affiché et le choix de **proposer l'acte** ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 ; [05-architecture.md](../05-architecture.md), §2.5).
- Le **vrai coût** ne s'applique qu'à l'**exécution**, pour la perte de synchronisation et la rupture. Si le vrai coût dépasse la limite, l'acte n'a pas lieu et le joueur est éjecté (E1), comme au §5.3.
  - Cette éjection ne compte **ni pour la récidive** (E3) **ni dans les refus répétés** suivis par la modération : le joueur ne pouvait pas la prévoir.
  - L'**IA en observation** garde le vrai coût pour son **estimation** (§5.1), puisqu'elle connaît son personnage ; seul ce qui part vers le client suit la classe apparente.
- Un acte de coût supérieur à 1 (coût calculé sur la classe apparente pour une cible liée par un secret) **n'est pas proposé** dans les listes d'actions ; l'indicateur « coût > 1 » ne sert que sur la barre de combat ([interface.md](interface.md), §2.3).

*Pourquoi : survoler des actes envers plusieurs cibles ne doit rien apprendre que le personnage ne sait pas ou qu'il cache, sinon le persistant du joueur s'en servirait.*

**Actes faits par le déplacement** (décidé ; [interface.md](interface.md), §5.2) : le cœur reconnaît deux actes à partir du seul mouvement :

- un **suivi** : rester derrière la même personne au-delà d'une durée et d'un nombre de changements de direction **[à calibrer]** ;
- l'**entrée chez autrui** : franchir le seuil d'un lieu privé sans y être invité.

Le signe de réticence part **avant** que l'acte compte : à l'approche du seuil, ou dès qu'un suivi devient probable. L'acte ne compte qu'au franchissement du seuil ou quand le suivi est reconnu.

C'est une règle du cœur, appliquée à tout personnage quel que soit son pilote. Une IA qui suit quelqu'un par une recette produit le même acte, et la personne suivie peut s'en apercevoir comme d'une anomalie (§13.8).

*Pourquoi : un signe n'a de sens que s'il permet de renoncer, et un acte fait en marchant ne doit pas échapper au jugement que reçoit le même acte choisi dans un menu.*

### 5.7 Le bulletin d'un électeur incarné (décidé ; E9)

*Pourquoi : quelques joueurs coordonnés ne doivent pas pouvoir choisir le maire, et le vote d'un personnage exprime sa vie, que seule son IA connaît.*

Le bulletin d'un personnage incarné est un **choix opaque**, comme un savoir opaque (§4.4). Le joueur peut conduire le personnage au bureau de vote ; au moment du dépôt, la garde hors fiction (E35) substitue au choix du joueur le bulletin que l'IA en observation (§5.1) aurait déposé, par le **même calcul** que pour un électeur PNJ ([social.md](social.md), §6.11) : relation avec chaque candidat, réputation du candidat dans ses communautés, accord entre ses convictions et le programme, avis de ses proches, et ses conclusions et son humeur (§13.4, §13.2).

- Le joueur ne choisit pas le bulletin. Une intuition peut lui dire vers qui penche le personnage, avec la précision habituelle (§5.5), puisque le personnage connaît son propre penchant.
- Faire s'abstenir un électeur assidu est une **obligation civique manquée** (petite dissonance).
- Un **élu** incarné vote librement en conseil, sous la synchronisation et sous le conflit d'intérêts (E8).

---

## 6. Niveaux de détail et coût de calcul

### 6.1 Trois niveaux (décidé)

*Pourquoi : trois niveaux suffisent pour 10 000 personnages, et le partage « où » / « quoi » avec la ville évite deux définitions concurrentes.*

**Partage avec la ville** : la ville définit **où** s'applique chaque niveau (cellules de 64 m, secteurs ≈ quartiers, [ville.md](ville.md), §8) ; ce domaine définit **ce que fait** un personnage à chaque niveau. Les zones de chaque niveau sont définies dans [ville.md](ville.md), §8.2, et ne sont pas redéfinies ici. En résumé : complet près des joueurs et dans le bâtiment qu'ils occupent (la **zone de bâtiment** pour un grand bâtiment, §6.3), simplifié dans le reste de leurs secteurs et les secteurs voisins, statistique dans le reste de l'agglomération.

| Niveau | Ce que fait un personnage | Fréquence de décision |
|---|---|---|
| **Complet** | Perception, réflexes, sélection d'objectif, HTN + GOAP, compétences, déplacements physiques, gestes, conversations | À chaque événement, et au moins toutes les 0,5 à 1 s **[à calibrer]** |
| **Simplifié** | Pas de physique : déplacements sur le graphe des rues et des pièces avec des durées ; HTN seul ; interactions entre PNJ résolues par des règles simples quand ils sont dans le même lieu ; seuls les événements notables sont mémorisés | À chaque changement de bloc d'agenda ou événement notable |
| **Statistique** | Le personnage est « dans un lieu » selon son agenda ; blocs d'agenda résolus en une fois ; besoins calculés par formule ; événements tirés au sort selon des taux par lieu et par activité ; souvenirs résumés | Une fois par bloc d'agenda (quelques fois par jour de jeu) |

Deux règles de ce domaine complètent le découpage spatial de la ville, qui les reprend :

- le **plancher selon l'importance** (§6.2) ;
- en **solo**, quand le joueur fait avancer le temps, une **bulle autour de lui en simulation simplifiée**, le reste de l'agglomération en statistique (§2.4).

### 6.2 Plancher selon l'importance (validé)

Un personnage **important** ne descend jamais en simulation statistique : il reste au moins en simulation simplifiée, même loin des joueurs. Ses décisions qui pèsent sur le monde (affaires, bande, mairie) sont donc prises avec des recettes.

(décidé ; *pourquoi : ce sont ceux dont les décisions ou les suites ont le plus de chances d'être remarquées par les joueurs*) Sont importants :

- les personnages de palier élevé (palier 7 et plus **[à calibrer]**) et les hors catégorie ;
- les personnages impliqués dans un événement en cours avec un joueur (enquête, conflit, projet commun) ;
- les personnages récemment incarnés (pendant quelques jours de jeu **[à calibrer]**), pour que leurs suites restent crédibles ;
- les personnages en **repos de l'IA** après une incarnation (décidé ; E17) : selon la règle d'accès du socle ([04-joueur.md](../04-joueur.md), §4), un personnage de palier 5 ou plus n'est accessible à aucun joueur pendant min(D/2, 4 h) réelles après une incarnation de durée D, ni au même cercle pendant min(D, 8 h), ni à un même compte plus de 8 heures par jour réel **[à calibrer]** ; l'IA met ce temps à profit pour se rétracter ou réparer (§5.3). *Pourquoi : un personnage puissant ne doit pas devenir la marionnette permanente d'un groupe, et ses suites doivent rester crédibles.*

Pour une ville de 10 000 habitants, cela fait quelques centaines de personnages.

### 6.3 Transitions entre niveaux (décidé)

*Pourquoi : reconstruire à partir de l'agenda garantit qu'un personnage est toujours là où il devrait être, et l'hystérésis évite des allers-retours coûteux.*

- **Montée** (statistique → simplifié → complet) : la position et l'activité sont **reconstruites à partir du bloc d'agenda en cours** (« au bureau, en train de travailler », « sur le trajet, aux deux tiers »). Le résultat est toujours plausible.
- **Zones de bâtiment** (décidé ; P14, [revues/generation.md](../revues/generation.md), §8.5) : dans un grand bâtiment (usine, hôpital, centre commercial, lycée), la montée en complet se fait par **zone de bâtiment** : un joueur dans un atelier de l'usine met en complet sa zone et les zones contiguës visibles, pas les 300 postes de l'usine ; les autres zones restent en simplifié, avec les mêmes traces (§13.12). *Pourquoi : la charge du complet doit rester bornée par joueur, et un grand bâtiment d'un seul bloc la ferait exploser à chaque visite.*
- **Descente** : le plan en cours est abandonné et résumé dans l'état de l'agenda ; les souvenirs détaillés sont résumés.
- **Hystérésis** : la distance de montée est plus petite que la distance de descente, et un personnage reste un temps minimal à un niveau, pour éviter qu'il change sans arrêt **[à calibrer]**.
- Un joueur qui prend le contrôle d'un personnage éloigné crée une **nouvelle bulle** autour de lui (§7.2).
- (décidé, v0.8) Tout niveau laisse les **mêmes traces**, et une reconstruction respecte le **dernier constat** fait par un joueur (§13.12).

### 6.4 Ordres de grandeur du coût **[à calibrer]**

Pour une ville de 10 000 personnages en ligne :

| Niveau | Nombre | Décisions par seconde réelle |
|---|---|---|
| Complet | 50 à 150 par bulle de joueur | 100 à 300 par bulle |
| Simplifié | Quelques milliers | Quelques dizaines au total |
| Statistique | Le reste | Négligeable |

Les décisions elles-mêmes coûtent peu. Les postes coûteux sont la **perception** (qui voit qui) et la **recherche de chemin** en simulation complète, et les recherches GOAP. Leviers :

- grille spatiale pour la perception ;
- recherche de chemin hiérarchique, chemins partagés entre personnages qui vont au même endroit ;
- décisions **étalées** sur plusieurs pas de simulation au lieu d'être toutes prises en même temps, selon un **budget de décisions par pas** exprimé en unités de travail (nombre d'évaluations d'objectifs et d'opérations de planification **[à calibrer]**) ;
- plans de routine **mis en cache** par fiche de rôle ;
- **budget GOAP par pas** de simulation, exprimé en **nœuds explorés** **[à calibrer]**, au-delà duquel les recherches attendent le pas suivant, dans un ordre défini. Aucun budget n'est exprimé en millisecondes : un même monde, avec la même graine, prend ainsi les mêmes décisions quelle que soit la charge de la machine ([05-architecture.md](../05-architecture.md), §2.10 et §2.22).

Le coût des mécanismes d'intelligence ajoutés en v0.8 est récapitulé au §13.15.

Le coût en simulation complète grandit avec le **nombre de joueurs**, pas avec la taille de la ville : c'est ce qui justifie le découpage en secteurs répartissables entre serveurs ([05-architecture.md](../05-architecture.md), §2.7 ; [ville.md](ville.md), §8.1).

En solo accéléré (par exemple une nuit de 8 heures sautée en quelques secondes), 10 000 personnages en simulation statistique représentent quelques dizaines de milliers de résolutions de blocs : quelques secondes de calcul au plus.

### 6.5 Gestion automatique et rodage (décidé)

- **Automate de gestion** : un gérant d'entreprise en simulation statistique ou absent (à l'extérieur, personnage persistant déconnecté) ne prend pas de décisions avec ses recettes ; un automate prudent, modulé par son profil, assure la gestion courante, et les décisions lourdes attendent son retour en simplifié, sauf urgence ([economie.md](economie.md), §9.5). *Pourquoi : la gestion courante d'un commerce loin des joueurs n'a pas besoin de plus, et les décisions qui changent le monde restent prises par des recettes.*
- **Rodage** : après la génération, la population vit en simulation **statistique**, avec les règles normales de ce domaine, sans naissances, décès ni mouvements ([population.md](population.md), §1.4).
  - **Initialisation à l'équilibre** (décidé ; P22, [revues/generation.md](../revues/generation.md), §9.7) : avant ce rodage, les variables rapides de ce domaine sont tirées dans leurs **distributions stationnaires**, que ce domaine fournit, mesurées par `ts-sim` sur des simulations longues et rangées dans le paquet de contenus avec leur provenance : **besoins** ; **pensées** actives par type, dont l'**humeur** découle, pour qu'elle ait toujours une cause (§13.2) ; **avancement des projets** en cours. Le rodage devient une mise en mouvement et une vérification. *Pourquoi : partir de l'équilibre raccourcit le rodage et évite que tout le monde ait faim ou soit de bonne humeur le premier jour.*
  - **Durée par convergence** (décidé ; P22) : le rodage s'arrête quand les indicateurs de stabilité (emploi, faillites, faim, humeur, trésorerie, relations) varient de moins d'un seuil **[à calibrer]** sur deux fenêtres de 7 jours, avec un minimum de 14 jours et un plafond de 60. La durée reste déterministe : même graine, même durée.
  - **Rodage propre** (décidé ; E25, P23) : chaque monde, à sa création, après le tirage de sa couche cachée (§4.4), vit avec sa graine les **14 jours du calendrier qui précèdent sa date de départ**, avec sa météo et ses vacances scolaires, ce qui fait diverger agendas, relations et stocks. *Pourquoi : un monde lancé en juillet doit avoir des habitants en rythme de juillet.*
  - La simulation statistique doit donc rester **stable** sur plusieurs semaines sans aucun joueur. *Pourquoi : c'est ce qui donne aux habitants des besoins, des habitudes et des relations « en cours » avant l'arrivée des joueurs.*

---

## 7. L'état « au repos »

### 7.1 Calcul (décidé)

*Pourquoi : prendre le contrôle d'un personnage menacé ou entre deux actions contournerait l'esprit de la règle du socle.*

L'état « au repos » ([04-joueur.md](../04-joueur.md), §5) est un **composant** recalculé par un système à partir de :

1. **l'activité en cours** : chaque action et chaque activité du catalogue porte une étiquette « au repos » ou « pas au repos », d'après le tableau du socle ;
2. **les états du personnage** : panique ou stress au-delà d'un seuil, inconscience, blessure grave ;
3. **les interactions** : en conversation, en combat, poursuivi ou poursuivant, au volant ;
4. **l'absence de menace perçue** : un personnage qui voit quelqu'un s'approcher de lui de façon menaçante n'est pas au repos, même s'il est encore assis ;
5. **la stabilité** : le personnage doit être au repos depuis une durée minimale (quelques secondes **[à calibrer]**), pour éviter les prises de contrôle entre deux actions.

### 7.2 Personnages éloignés (décidé)

*Pourquoi : l'état ne peut être jugé précisément qu'en simulation complète, d'où la montée avant la vérification.*

En simulation simplifiée ou statistique, l'état est déduit du **bloc d'agenda en cours** (dormir, travail routinier : au repos ; trajet en voiture : pas au repos). Quand un joueur demande à prendre le contrôle, le personnage monte en simulation complète, une bulle est créée, puis l'état est vérifié à nouveau avec les règles du §7.1.

### 7.3 Réservation courte (validé)

Si le personnage n'est pas au repos, la demande est **mise en attente** quelques minutes **[à calibrer]** : dès qu'il est au repos (il se gare, finit sa conversation), le joueur en prend le contrôle. **L'IA ne change pas son comportement** pour se rendre disponible. Le joueur peut annuler. (décidé) La réservation tombe si une autre condition d'accès cesse d'être remplie ; la première réservation valide est servie en premier si plusieurs joueurs visent le même personnage. *Pourquoi : c'est la règle la plus simple et la plus équitable entre joueurs.*

(décidé ; E30) Un compte n'a qu'**une seule réservation active** ; après un refus du « Passer maintenant ? » ([interface.md](interface.md), §3.5) ou une expiration, le même compte attend **10 minutes** **[à calibrer]** avant de pouvoir réserver à nouveau le même personnage. *Pourquoi : une réservation doit servir à jouer un personnage, pas à le bloquer.*

### 7.4 Passage de relais (décidé)

*Pourquoi : suspendre le plan plutôt que l'effacer permet au joueur de continuer naturellement et à l'IA de reprendre là où elle en était.*

- Si le personnage est dans une **activité routinière** (cuisiner, ranger), le joueur hérite de l'activité en cours : il peut la continuer ou l'arrêter. L'IA la garde dans son plan pour la reprise.
- S'il **dort**, il se réveille, avec les effets de la dette de sommeil (socle).
- Le plan de l'IA est **suspendu**, pas effacé : il sert de point de départ au bilan de situation de la reprise (§5.3).

---

## 8. La vie des enfants

L'agglomération compte environ **1 800 enfants** (18 % des habitants, [ville.md](ville.md), §2.3). Ils ne sont **jamais jouables** ([02-personnages.md](../02-personnages.md), §3.4), mais ce sont des personnages complets, pilotés par la même IA.

### 8.1 Autonomie progressive par âge (validé)

Chaque tranche d'âge est une **fiche de rôle** (§0), avec son agenda type, ses recettes et ses actions permises **[âges à calibrer]** :

| Âge | Autonomie | Journée type |
|---|---|---|
| **0 à 2 ans** | Entièrement dépendant | Au foyer, à la crèche ou chez une assistante maternelle (habitante) ; beaucoup de sommeil ; ses besoins sont satisfaits par un adulte |
| **3 à 10 ans** | Accompagné | École maternelle puis élémentaire, accompagné à l'aller et au retour ; garderie ou centre de loisirs ; jeux au parc ou dans la cour de l'immeuble sous la surveillance d'un adulte |
| **11 à 14 ans** | Trajets seuls dans le quartier | Collège ; trajets seuls à pied, à vélo ou en bus ; activités (sport, musique) ; amis chez soi ou dehors en journée |
| **15 à 17 ans** | Presque autonome | Lycée ; sorties le soir avec des horaires fixés par les parents, plus ou moins respectés ; petits boulots, jobs d'été ; bêtises et premières transgressions selon le profil |

- **Horaires scolaires** (décidé) : l'école fixe le gros de l'agenda en semaine (environ 8 h 30 à 16 h 30 **[à calibrer]**), avec les vacances scolaires du calendrier du monde. Les écoles et leurs horaires sont fournis par la ville ([ville.md](ville.md), §5.2). *Pourquoi : c'est le rythme d'une école européenne ordinaire, qui structure aussi la journée des familles.*
- **Dépendance au foyer** (décidé) : un enfant accompagné ne peut pas partir seul. Si personne ne vient le chercher, il attend sur place (l'école prévient les parents, puis d'autres adultes), ce qui crée de vraies conséquences pour les parents. *Pourquoi : cela donne du poids aux obligations des parents, y compris quand un joueur les incarne.*
- **Sortir sans permission** (décidé) : un enfant de 11 ans ou plus peut s'écarter de son agenda (traîner après le collège, sécher un cours) selon son profil et son âge. *Pourquoi : une adolescence sans écarts ne serait pas crédible.*
- **Garde alternée** (décidé) : après une séparation, l'enfant a **deux domiciles** et son agenda suit le calendrier de garde ; les obligations d'accompagnement passent d'un parent à l'autre (§2.5 ; [population.md](population.md), §2.2). *Pourquoi : c'est la situation la plus courante après une séparation, et elle se décrit entièrement par l'agenda.*

### 8.2 Besoins et motivations adaptés à l'âge (décidé)

*Pourquoi : réutiliser les mêmes besoins avec d'autres vitesses évite un système propre aux enfants.*

- Mêmes besoins courts que les adultes (§1.1), avec des vitesses adaptées : plus de sommeil, une faim plus régulière, un besoin de **détente** et de **stimulation** qui prend la forme du **jeu**.
- Les besoins des plus jeunes sont satisfaits **par un adulte** : un bébé qui a faim produit une sollicitation (il pleure) que ses parents ou sa garde doivent traiter.
- Motivations longues réduites : **relations** (parents, fratrie, amis), **statut** (place dans le groupe d'amis, à l'école), **devoir** (école, règles de la maison). Pas de motivation **argent** avant l'adolescence (argent de poche, petits boulots ensuite).
- Le **sentiment de sécurité** dépend fortement de la présence d'un adulte de confiance.

### 8.3 Perception et mémoire : témoins à part entière (validé)

- Les enfants **perçoivent et mémorisent** comme les autres (§4), avec une perception et une mémoire qui dépendent de l'âge **[à calibrer]** (un enfant de 4 ans retient peu et mal ; un adolescent comme un adulte).
- Ils **racontent** ce qu'ils ont vu à leurs parents, à leurs amis ou à leurs enseignants : un enfant qui voit quelque chose peut être à l'origine d'une rumeur ou d'une enquête.
- (décidé) Les adultes jugent leur parole **moins fiable**, d'autant plus que l'enfant est jeune. Le poids exact d'un témoignage relève des domaines social et combat (police et justice). *Pourquoi : c'est réaliste, et cela évite qu'un enfant devienne un témoin infaillible.*

### 8.4 Les enfants dans la vie des parents (décidé)

*Pourquoi : les enfants sont la première source d'obligations et de motivations des parents ; les intégrer à leur agenda rend les foyers vivants.*

- **Obligations** : déposer et chercher les enfants, les faire manger, les coucher, rester à la maison quand ils sont malades. Elles entrent dans l'agenda des parents (§2.1) et deviennent des **objectifs du personnage** quand un joueur incarne un parent (barème du socle : obligation du jour).
- **Organisation du foyer** : les parents se répartissent les obligations selon leurs horaires de travail ; à défaut, ils font appel à une garde (grands-parents, voisin, assistante maternelle, centre de loisirs).
- **Motivations** : avoir des enfants renforce le poids de l'**argent** (subvenir aux besoins), du **devoir** et des **relations** ; le bien-être des enfants peut faire naître des projets (changer de quartier, trouver un emploi mieux payé).
- **Protection** : un parent (et plus largement un adulte bienveillant) a un **réflexe de protection** envers un enfant en danger, qui passe avant sa propre sécurité selon son profil.
- **Un joueur qui néglige l'enfant** du personnage qu'il incarne : il perd de la synchronisation si c'est contraire à la nature du parent, et l'enfant comme les autres adultes réagissent (souvenirs, inquiétude, signalement). La suite relève du domaine social.

### 8.6 Violence et mineurs : garanties par construction (décidé)

*Pourquoi : la garantie doit tenir pour l'IA comme pour le joueur, et le plus sûr est qu'aucune recette ni aucune intention correspondante n'existe ([combat.md](combat.md), §12.2).*

- L'IA n'a **aucune recette ni aucune intention violente d'un adulte envers un mineur** : face à un mineur, un adulte n'a que des gestes de contrôle (repousser, se dégager, retenir par le bras ; pour un policier, interpeller et menotter sans frapper). Les réflexes (§3.1) et la planification (y compris la recherche GOAP, qui ne peut pas assembler ce qui n'existe pas) respectent la même limite.
- **Aucun acte avec une arme** n'est disponible pour un mineur ; la violence entre mineurs se limite aux bagarres à mains nues.
- Les adolescents peuvent commettre de petits délits (vols, dégradations, guet) par les recettes ordinaires, avec les limites ci-dessus.

### 8.5 Grandir : un profil façonné par l'enfance (validé)

- Les enfants **naissent dans le monde** ([population.md](population.md), §7.2) : le projet « avoir un enfant » (§3.4) mène à une grossesse (§2.5), puis à un nourrisson qui hérite en partie du potentiel et du profil de ses parents.
- Les enfants **vieillissent** avec le calendrier du monde (en ligne, au rythme réel ; en solo, accélérations comprises).
- Leur **profil comportemental évolue** pendant l'enfance selon ce qu'ils vivent : climat familial, réussite ou difficultés à l'école, amis, événements marquants (violence subie ou vue, deuil, déménagement). (décidé) Le profil reste malléable jusqu'à l'adolescence, puis se stabilise vers 18 ans ; chaque souvenir marquant le déplace légèrement dans la direction des **effets** de ce souvenir, de γ × un taux **[à calibrer]** (décidé ; G13, par la même **révélation** qu'au §13.5). *Pourquoi : réutiliser les effets des souvenirs, avec les paramètres d'`appraise`, est la façon la plus simple de relier l'enfance au caractère adulte.*
- À **18 ans**, l'enfant devient un **adulte jouable** comme les autres, avec le profil que sa vie lui a donné, ses souvenirs et ses relations. Il change de fiche de rôle (étudiant, premier emploi, sans emploi).

---

## 9. Séjours à l'extérieur

Un habitant peut partir **à l'extérieur** de l'agglomération puis revenir ([ville.md](ville.md), §7.4). Pendant son absence, il **n'est pas jouable** ([02-personnages.md](../02-personnages.md), §3.4).

### 9.1 Décider d'un départ (décidé)

*Pourquoi : passer par la planification normale garde un seul mécanisme de décision pour tous les objectifs.*

Les départs passent par la planification normale (§3), comme n'importe quel objectif :

| Motif | Origine de la décision |
|---|---|
| **Vacances** | Projet annuel, planifié à l'avance selon l'argent, les congés et, pour les familles, les vacances scolaires. Le foyer part ensemble ou en partie |
| **Déplacement professionnel** | Fiche de rôle (commercial, chauffeur routier, cheminot, cadre) : obligation de l'agenda |
| **Visite** | Motivation « relations » envers des proches vivant à l'extérieur (relations abstraites, sans personnage simulé) ; aussi pour un événement (mariage, enterrement) ou pour **aider un proche malade** (projet, §3.4). Les visites vont **toujours de la ville vers l'extérieur** : les proches de l'extérieur ne viennent jamais en ville ([population.md](population.md), §8.2) |
| **Travail à l'extérieur** | Navetteur sortant (§2.5) : absence quotidienne pendant ses heures de travail |
| **Déménagement définitif** | Voir §9.4 |
| **Détention** | Décidée par la justice (domaine combat) ; l'IA ne fait que la subir |
| **Fuite d'un suspect** | Réaction à une interpellation imminente ou à une enquête qui se rapproche, selon son profil, la gravité de ce qu'il risque et ses chances ([combat.md](combat.md), §7.1). Il est alors **en fuite à l'extérieur** : arrêté au bout d'un délai tiré au sort **[à calibrer]**, il revient comme **détenu** puis suit le retour de détention, ou il ne revient jamais et le cas devient un départ définitif (§9.4) |

- Un départ annoncé crée des **engagements** et des **souvenirs** chez les proches (« il est en vacances jusqu'au 15 ») : les voisins savent qu'il est absent s'il l'a dit.
- Il entraîne des **obligations à régler** : poser des congés, faire garder les enfants qui restent, confier ses clés.
- Les **contacts à distance** (appels, messages) avec les proches de l'extérieur, résolus de façon abstraite, satisfont un peu le lien social et entretiennent la relation, sans départ ([population.md](population.md), §8.2) (décidé ; *pourquoi : ils évitent qu'une relation lointaine ne vive que par des voyages*).
- **Le départ est réel** : en simulation complète ou simplifiée, le personnage se rend vraiment au point de sortie (gare, route, ponton) avec le mode de transport prévu ; en statistique, il quitte directement son lieu.

### 9.2 Pendant l'absence : résumé statistique (validé)

Le personnage n'est pas simulé. Le cœur garde (décidé ; *pourquoi : c'est le minimum pour un retour cohérent sans simuler l'extérieur*) :

- le **motif**, la **date de départ** et la **date de retour prévue** (qui peut être prolongée ou avancée par un événement) ;
- les **personnes qui l'accompagnent** ;
- ses **obligations manquées** en ville (travail, engagements), qui ont leurs conséquences normales si l'absence n'était pas prévue.

À son retour, on applique un **résumé** cohérent avec le motif **[à calibrer]** :

| Motif | Effets au retour |
|---|---|
| Vacances | Argent dépensé ; besoins de détente et d'énergie satisfaits ; souvenirs résumés positifs (ou décevants) |
| Déplacement professionnel | Salaire et frais ; fatigue ; parfois une étape de projet professionnel (contrat signé) |
| Visite | Relations avec les proches de l'extérieur renforcées ; humeur selon l'événement (mariage, deuil) |
| Détention | Résumé fixé par le domaine combat ([combat.md](combat.md), §9.2) : **désentraînement** (la force pouvant être entretenue selon le profil), formation éventuelle, incidents, évolution d'une dépendance, contacts avec les autres détenus habitants, souvenir marquant sans changement du profil ; au retour, obligations de suivi (§2.5) et projets de réinsertion ou de retour au milieu ([combat.md](combat.md), §9.3) |

Rarement, une **nouvelle** est tirée au sort (une rencontre, une maladie, une offre d'emploi, une mauvaise nouvelle) et peut faire naître un projet au retour.

(décidé) Un personnage **important** absent ne prend pas de décision en ville : ses responsabilités sont assumées par un **suppléant**, désigné ou implicite, qui prend les décisions courantes, prend avec prudence les décisions lourdes urgentes et reporte les autres ([social.md](social.md), §6.8) ; s'il gère une entreprise, l'automate de gestion assure le courant (§6.5). *Pourquoi : son absence ne doit pas paralyser son organisation.*

### 9.3 Retour (décidé)

*Pourquoi : un retour par un vrai point de sortie et une reprise identique à celle d'une incarnation évitent toute apparition incohérente.*

- Le personnage **réapparaît au point de sortie** par lequel il est censé revenir (le train de 18 h 12, la rocade), à l'heure prévue, puis rentre chez lui.
- La reprise suit la même logique que la fin d'une incarnation (§5.3) : bilan de situation (ce qui a changé pendant son absence, ce qu'il apprend en rentrant), application du résumé, re-planification. Il reprend son agenda au bloc en cours.
- Il redevient jouable dès qu'il a repassé la limite de l'agglomération, aux conditions habituelles.

### 9.4 Déménagements définitifs : l'IA, régulée (validé)

- Un déménagement hors de l'agglomération naît des **projets** du personnage : un emploi ailleurs, un rapprochement familial, un quartier qui ne lui convient plus, la fuite après un crime, la retraite.
- Un **multiplicateur des projets de départ**, calculé par le régulateur démographique ([population.md](population.md), §7.6), rend ces projets plus ou moins probables pour garder une **population stable** (les arrivées de nouveaux habitants compensent les départs et les décès).
- Sauf fuite, un projet de départ dure plusieurs semaines et se voit (§3.4).
- Un personnage parti définitivement **quitte le monde** : il n'est plus simulé ni jouable ; les familiarités des joueurs avec lui deviennent inutiles (décidé, puisqu'elles ne pourront plus jamais servir). Ses proches gardent ses souvenirs et une relation abstraite « à l'extérieur ».

### 9.5 Un joueur qui atteint une sortie : fin d'incarnation (validé)

Quand un joueur conduit le personnage qu'il incarne jusqu'à une sortie de la carte (route, gare, ponton) :

1. une **confirmation** lui est demandée ;
2. s'il confirme, l'**incarnation se termine** (bilan normal, comme quand il rend le personnage) ;
3. l'IA choisit entre un **séjour court** cohérent avec la vie du personnage (une visite, une course, un rendez-vous), suivi d'un retour selon le §9.3, et le **demi-tour** : elle fait demi-tour si le personnage a une **obligation dans les 12 heures** **[à calibrer]** (service, garde, audience, convocation, scrutin, rendez-vous important, enfant à prendre) (décidé ; E18 ; *pourquoi : la sortie ne doit pas devenir un moyen de retirer un habitant de la ville au moment où il compte, et un séjour court limite l'indisponibilité causée par le joueur*).

S'il refuse, le personnage fait demi-tour.

Conduire le personnage loin d'une **obligation importante** (poste de garde, service de police ou de secours, audience) et la lui faire abandonner est un **acte dissonant** pesé par ses conséquences, soumis au bilan provisoire de l'incarnation (décidé ; E18, E4 ; *pourquoi : le poids d'un abandon de poste tient à ce qu'il permet, pas au trajet*). Pour le **personnage persistant**, une sortie de la carte est un lieu de retrait ([02-personnages.md](../02-personnages.md), §1.1).

---

## 10. Points ouverts

| Sujet | Question | Piste |
|---|---|---|
| Valeurs de calibrage | Vitesses des besoins, seuils, poids d'utilité, inertie, rayons des bulles, tailles de mémoire, âges des enfants, effets des séjours | Prototype du cœur ([05-architecture.md](../05-architecture.md), §5, étape 1) ; les enquêtes publiques d'emploi du temps peuvent servir de référence pour les routines |
| Évolution du profil des enfants | Comment chaque type d'événement déplace-t-il le profil, et à quelle vitesse ? | Prototype ; règles par axe de valeurs |
| Liste des fiches de rôle | Quels rôles pour le prototype ? | Une dizaine de rôles de palier 0 à 3 dans un quartier, à partir des rôles économiques du prototype ([economie.md](economie.md), §3.9) |
| Seuil de moyens désespérés | Jusqu'où un besoin critique peut-il pousser un personnage à agir contre sa nature ? | Seuil par axe de valeurs **[à calibrer]** |
| Imitation des joueurs | Apprendre des incarnations bien synchronisées | Écarté pour l'instant (validé) ; à reconsidérer après les premières compétences entraînées |
| GOAP en simulation simplifiée | Faut-il une recherche simplifiée hors bulle ? | Pour l'instant non : l'échec d'une recette mène à un abandon ou un report |
| Évolution des dépendances | Vitesse d'installation d'une dépendance, chances de sevrage | Prototype, avec le domaine économie (produits, prix) |
| Présomptions des états inconnus | Liste des présomptions par type d'objet et de lieu (§3.7) | Prototype, à partir des usages publics des lieux fournis par la ville |
| Règles de réflexion | Quel catalogue initial de règles et de biais d'attribution (§13.4) ? | Une trentaine de règles écrites à la main, puis poids calibrés hors ligne (§13.13) |
| Pensées, humeur, crises | Durées, cumuls, seuils de crise (§13.2) | Prototype ; références publiques sur le deuil, le stress et l'humeur |
| Délais humains | Distribution des délais de réaction et d'hésitation (§13.7) | Tirée de données publiées sur les temps de réaction humains, puis partagée avec le délai de formulation ; les mesures de testeurs d'une bêta pourront l'affiner, sans être requises (décidé ; A8) |
| Listes bornées | Tailles des pensées, faits sensibles, déclarations, valeurs apprises, sujets échangés (§13) | Mesure de mémoire et de coût dans `ts-sim` |

Réglés par les autres domaines : **naissances** ([population.md](population.md), §7.2), **projets collectifs** ([social.md](social.md), §6.9), **hygiène et réactions sociales** ([social.md](social.md), §5.5).

---

## 11. Impacts sur le socle et les autres domaines

Ces points ne sont **pas appliqués** ailleurs : ils sont à trancher par la session principale ou par la session du domaine concerné.

### Revues d'équilibrage et de sécurité

Modifications demandées par [revues/equilibrage.md](../revues/equilibrage.md), §6.9, et [revues/securite.md](../revues/securite.md), §9.13, intégrées en v0.7 avec les arbitrages de la session principale : validation en deux étages (§3.7 ; S4, A2), couche cachée par monde (§4.4, §6.5 ; S3, E25, A1), coût estimé fourni par l'observation (§5.1 ; E1, E35, E38), point de rupture (§5.3, §5.6 ; E1), engagements au profit du cercle (§5.3 ; E6), barème du travail et objectifs liés (§2.3, §5.4 ; E11, E13), repos de l'IA (§6.2 ; E17), réservations (§7.3 ; E30), sortie de la carte et obligations (§9.5 ; E18).

Reliquats signalés par le domaine social ([social.md](social.md), §12.9), intégrés en v0.8 : réponse de l'IA aux demandes d'actes intimes (§3.6 ; A6), bulletin opaque d'un électeur incarné (§5.7 ; E9), rétractation d'un mariage ou d'un pacs avec le cercle et réévaluation d'une dette ou d'une promesse au profit du cercle, sans le biais de la parole donnée (§5.3 ; E6, E31).

### Revues de clôture (v1.3)

Corrections des revues de clôture d'architecture (C-1, C-2, C-5, C-6, S-4), de sécurité (C1) et d'équilibrage (C5, C6, C7) qui touchent ce domaine, **intégrées en v1.3** :

- contrôle des gestes en deux temps (§2.6) ;
- tenue stockée à tous les niveaux et clés d'aléa de la tenue et du coiffeur (§2.6) ;
- sources des persistants éteintes à leur disparition, et bruit imputé selon les croyances (§2.6) ;
- vocalisation sur un flux dédié avec un indice de variante (§3.6) ;
- quatre dates par relation, mises à jour sur identification seulement (§4.1) ;
- raisons sans savoir opaque, niveau calculé sur la classe apparente, acte de coût supérieur à 1 non proposé (§5.6) ;
- panels disjoints, admission par les ancres, accord entre modèles différents, six profils d'adversaires, AUC du discriminateur, surfaces des sessions joueuses et des juges (§13.13, §14).

Le socle 05 est déjà à jour. Aucune demande n'est envoyée : ce qui reste ailleurs est noté **pour la phase 1**.

| Destinataire | Impact | État |
|---|---|---|
| [social.md](social.md), §1.1, §5.11 et §9.4 | Quatre dates par relation au lieu de deux ; imputation du bruit selon la présence crue de l'occupant ; sources d'un persistant éteintes à sa disparition ; la détection de l'acharnement compte aussi les actes de faible intensité (bruit imputé, suivre, frapper aux portes, plaintes) | Pour la phase 1 |
| [interface.md](interface.md), §2.3 et §4.7 | Niveau de réticence affiché sur la classe apparente d'une relation secrète ; aucune raison citant un savoir opaque ; acte de coût supérieur à 1 absent des listes ; portrait reconstitué groupe par groupe, à la date de chaque précision | Pour la phase 1 |
| [direction-artistique.md](direction-artistique.md), §3.6 et §6.9 | Indice de variante de la vocalisation, lu sans tirage ; `cargo xtask assets check` par famille, avec alerte par valeur sans animation propre | Pour la phase 1 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 | La phrase « le signe de réticence est alors toujours visible », pour un acte de coût supérieur à 1, est à lire avec C6 : un tel acte n'est plus proposé dans les listes, et l'indicateur ne se voit que sur la barre de combat | Corrigé dans le socle le 30/10 |
| Phase 0 (revue d'architecture, tour 2) | Le **test d'identification** (§4.1) se fait par un seul tirage par épisode de perception, sur le flux `identification` (T2-1) ; l'**« inconnu vu »** est un souvenir ordinaire, soumis à l'oubli (§4.3 ; T2-2) | Pour la phase 1 |
| `ts-sim` et outils | `ts-sim play` et `ts-sim report --observer` ; panels disjoints ; suivi mensuel de l'AUC du discriminateur sur les traces de production | Pour la phase 1 |

### Social v0.7 et interface v0.7 (v1.2)

Demandes de [social.md](social.md), §12.12 (v0.7), et de [interface.md](interface.md), §15.2 et §15.13 (v0.7), **intégrées en v1.2** :

- recettes de l'escalade du bruit et réponses de la source (§2.6) ;
- retrait des signes de bande (§2.6) ;
- tenue attendue et `opens: [J8]` déclarés par les pratiques (§13.9) ;
- se changer là où sont les vêtements, parapluie à la porte, actions de radio et volume en trois crans (§2.6) ;
- classe de la cible et codes fermés (§5.6) ;
- actes faits par le déplacement (§5.6).

Aucune contradiction relevée. Ce qui touche d'autres domaines est noté ici, sans demande, **pour la phase 1** :

| Destinataire | Impact | État |
|---|---|---|
| [interface.md](interface.md) | Geste court « ôter ou cacher un signe de bande », possible partout, distinct de « se changer » ; à offrir aussi au personnage piloté (§2.6) | Pour la phase 1 |
| [direction-artistique.md](direction-artistique.md) | Gestes au catalogue d'animations : ôter ou cacher un signe, sonner ou frapper au mur pour le bruit, baisser le volume (§2.6) | Pour la phase 1 |
| [combat.md](combat.md) | La recette « se plaindre à la police » pour un tapage nocturne n'aboutit qu'à une infraction du catalogue, si le catalogue la retient (§2.6) ; la pression policière perçue entre dans le retrait des signes (§2.6) | Pour la phase 1 |
| [ville.md](ville.md) | Isolation entre logements et heures calmes, lues par le cran choisi et par la gêne (§2.6) | Pour la phase 1 |
| `ts-sim` (outils) | Calibrage des pas sautés de l'escalade, des réponses de la source, de la durée de reconnaissance d'un suivi (§2.6, §5.6) | Pour la phase 1 |

### Interface v0.6 (v1.1.2)

Demandes de [interface.md](interface.md), §15.2 (v0.6), **intégrées en v1.1.2** : vocalisation de chaque réplique (§3.6), dernier contact et précision vue pour les portraits (§4.1), historique daté de la coiffure et de la tenue habituelle (§2.6).

| Destinataire | Impact |
|---|---|
| [05-architecture.md](../05-architecture.md), §2.5 | La vocalisation est une **catégorie** et une **intensité** d'une énumération stable, choisies par une règle du cœur à la résolution de l'acte, pour tout pilote ; par relation, quatre dates (dernier échange, dernière perception, dernière à « tenue » ou mieux, dernière à « visage »), mises à jour seulement sur identification (corrigé en v1.3, C-5 et sécurité C1) |
| [social.md](social.md), §1.1 | Le champ « dernier contact » se dédouble quand il le faut : dernier **échange** (pour la dérive, §1.4 de social) et dernière **perception** (pour le portrait) ; les deux sont gardés aussi dans la forme compacte |
| [population.md](population.md) | Historique daté des **signes distinctifs** (cicatrice, tatouage, lunettes), à côté de ceux de la coiffure et de la tenue habituelle tenus ici (§2.6) |
| [interface.md](interface.md) | Les deux demandes du §15.2 (v0.6) sont traitées |

### Direction artistique (v1.1)

Demandes de [direction-artistique.md](direction-artistique.md), §12.3, **intégrées en v1.1** au §2.6 : tenue du jour, se changer et parapluie, coiffeur, énumérations de gestes et d'activités clés du catalogue d'animations, activités musicales et musique entendue.

| Destinataire | Impact |
|---|---|
| [direction-artistique.md](direction-artistique.md) | Les quatre demandes du §12.3 sont traitées (§2.6). Les énumérations de gestes et d'activités ont un **identifiant stable** et une **famille**, sur laquelle le client se replie en attendant une animation propre ; une valeur sans identifiant stable ni famille est refusée par la construction du paquet, et le contrôle des assets refuse une famille sans animation (corrigé en v1.3, C-1). La **source musicale** d'un lieu ou d'un objet (station, sono, musicien avec instrument, style et qualité) et son volume sont des données du cœur ; les morceaux sont choisis par le client |
| [05-architecture.md](../05-architecture.md) | Composants **tenue portée** (pièces et état), **coiffure** (coupe, longueur) et **source musicale** ; énumérations de gestes et d'activités à identifiants stables et à famille, seuls contrôlés à la construction du paquet ; les animations sont vérifiées par `cargo xtask assets check`, côté client (§2.6 ; corrigé en v1.3.1, T2-5) ; tenue portée stockée à tous les niveaux, mise à jour en statistique au changement d'usage (corrigé en v1.3, C-2) |
| [population.md](population.md) | État initial : garde-robe abstraite et état des pièces, coiffure et longueur de départ, **goûts musicaux**, instruments possédés et compétence musicale |
| [ville.md](ville.md) | Salons de coiffure (§5.3 de ville) ; **vestiaires** des postes à uniforme ; laveries ; lieux où l'on joue de la musique (bars, église, salle des fêtes, rues) |
| [economie.md](economie.md) | Dépenses de vêtements, de coiffeur et de laverie ; musiciens payés ou au chapeau ; tâche « mettre de la musique » dans les postes des bars et des boutiques |
| [social.md](social.md) | Lecture de la tenue (§5.5 de social) y compris la tenue inadaptée à une occasion ; gêne due au bruit entre voisins (plainte, dispute, rancune) ; le coiffeur habituel comme relation |
| [interface.md](interface.md) | Choix de la tenue dans la garde-robe pour le personnage piloté ; choix de la coupe chez le coiffeur ; allumer une radio et choisir la station |

### Revues des signatures et de la génération (v1.0)

Modifications de [revues/signatures.md](../revues/signatures.md), §9.6, et de [revues/generation.md](../revues/generation.md), §13.7, intégrées en v1.0 : signature dérivée des effets par `appraise` (§0, §1.4), recettes sans signature écrite et coût de chemin de GOAP (§3.3 ; G8), verdict d'`appraise` et masque d'imputabilité (§3.7), souvenir dissonant et réparation par l'effet dominant (§4.5), épisode et coût marginal (§5.1 ; G9), planchers de rupture (§5.3 ; G7), réticence en quatre niveaux (§5.6 ; G24), enfance (§8.5 ; G13), blâme et admiration (§13.1 ; G28), révélation des traits supposés (§13.5 ; G13), profil normatif (§13.9 ; G14), calibrage des signatures avant les décisions (§13.13 ; G19, G20, G21), brouillons du modèle de langage (§13.14 ; G18), mesures et scénarios `SIG_*` (§14) ; zones de bâtiment (§6.1, §6.3 ; P14), rodage à l'équilibre, par convergence et propre (§6.5 ; P22, P23), mobilier mobile (§13.6 ; P11). Le socle (03, 04, 05) est déjà à jour.

| Destinataire | Impact |
|---|---|
| [05-architecture.md](../05-architecture.md) | `ts_ai` appelle `ts_conduct::appraise` pour le choix des moyens, GOAP, la réticence et les témoins ; composants **épisode** (≈ 100 octets) et **cache des verdicts** (≈ 130 octets) par personnage ; **distributions stationnaires** de ce domaine (besoins, pensées par type, avancement des projets) rangées dans le paquet de contenus avec leur provenance, produites par `ts-sim` |
| [population.md](population.md) | Le rodage de famille utilise l'initialisation à l'équilibre fournie par ce domaine et s'arrête par convergence ; valeurs apprises de départ sur le mobilier mobile, produit à l'emménagement (§6.5, §13.6) |
| [ville.md](ville.md) | **Zones de bâtiment** des grands bâtiments, unité de la montée en complet (§6.3) |
| [social.md](social.md) | Blâme et écart perçu par `appraise` sur les croyances du témoin ; profil normatif des groupes et des pratiques (§13.1, §13.9) |
| [combat.md](combat.md) | Les justifications (légitime défense, défense d'autrui, devoir de fonction) et les planchers de rupture remplacent les variantes de signature de l'escalade ; réparation « se rendre à la police » choisie par l'effet dominant (§4.5, §5.3) |
| [interface.md](interface.md) | Réticence en quatre niveaux, indicateur de rupture et code de raison, seules valeurs reçues du cœur (§5.6) |

### Revue de l'intelligence (v0.8)

Impacts des mécanismes du §13 et de la mesure du §14, à trancher par la session principale ou par les domaines concernés.

| Destinataire | Impact |
|---|---|
| [05-architecture.md](../05-architecture.md) (session principale) | **Nouveaux composants bornés** par personnage : pensées (16), humeur, conclusions et thèmes de vie, traits supposés, faits sensibles (8) et déclarations, valeurs apprises (32), sujets échangés, voix ; environ 2 Ko par personnage, soit environ 20 Mo pour 10 000 **[à mesurer]**. La **réflexion** est une tâche quotidienne de l'ordonnanceur (§2.22). **Budget** d'environ 500 opérations par personnage et par jour en statistique pour l'ensemble du §13, à tenir dans l'objectif d'un an en moins de 5 minutes (§13.15). Un **flux d'aléa à clé distinct** pour le naturel (§13.7). Les sorties d'un modèle de langage n'entrent dans le cœur que comme **entrées journalisées** (§13.14). **Dernier constat** par personnage (§13.12). Indicateurs dans `ts_metrics`, scénarios `ts-sim`, **calibrage hors ligne des poids** orchestré en Python (§13.13, §14). Outil de **tests à l'aveugle** : relecture avec pilotes masqués et discriminateur hors ligne (§14.3) |
| [04-joueur.md](../04-joueur.md), §7 (session principale) | Préciser que la **personnalité** (profil) ne change pas, mais que les **attitudes**, conclusions et valeurs apprises évoluent avec la vie du personnage, y compris après une incarnation (§13.4, §13.6) |
| [04-joueur.md](../04-joueur.md) et [interface.md](interface.md) | Observation : certains gestes propres aux joueurs (courir sans cesse, tourner sur soi, sauter, tout fouiller) trahissent un pilote humain ; à traiter par la signature et la synchronisation ou par l'interface, puisque le jeu promet qu'on ne distingue pas les pilotes (§14.3) |
| [social.md](social.md) | Format de relation étendu : **traits supposés**, émotion envers la personne, **sujets échangés**, compteurs de **lassitude** (§13.5, §13.10, §13.11). **Faits sensibles** et **registre des déclarations**, liés aux actes « mentir » et « calomnier » ; deux versions d'un même fait par la même source font naître un **soupçon** (§4.2 de social). **Pratiques sociales** en données, à articuler avec les délibérations et les groupes (§13.9). Lassitude étendue à la semaine et aux demandes, invitations et cadeaux, en plus des rendements décroissants du §1.4 de social. **Conclusions** de rancune articulées avec la rancune et le pardon (§1.5 de social). Le classement du texte libre rend aussi un **ton** et un **registre** (§13.10). Voix propre dans la formulation et le repli systémique. *Traité par le social v0.5 (§12.10) ; ia-pnj aligné en v0.9 : ton et registre, champs étendus des seules relations complètes, pardon accepté* |
| [combat.md](combat.md) | **Anomalies** (§13.8) qui déclenchent les recettes d'alerte, d'intervention ou de fuite (§4.2 de combat). **Faits sensibles** d'un auteur : peur d'être dénoncé, pression sur un témoin, aveu (§6.4 et §11.2 de combat). **Soupçon** et incohérences de versions comme indices des affaires (§6.2 de combat). **Attroupements** autour d'un incident, et leur dispersion par la police (§13.9). **Habituation** aux provocations et fausses alertes répétées, avec E29 (§13.11). Conclusions « lieu dangereux » qui nourrissent le besoin de sécurité. Tables du statistique recalibrées sur les nouveaux comportements (§13.12) |
| [interface.md](interface.md) | Signes visibles de l'**humeur** et des **crises** (posture, démarche, expression) ; animations du **naturel** (gestes au repos, regard, hésitations) ; **ton** et **registre** affichés à la confirmation d'un texte libre ; intuitions nourries par les pensées, thèmes de vie et valeurs apprises (§5.5) |
| Service de dialogue | Reçoit la **voix**, l'**humeur**, l'**état du discours**, les sujets précédents et les dernières formulations ; évite les répétitions ; **biographie formulée** mise en cache pour les personnages importants ; classement du texte libre avec registre ; production **hors ligne** de contenus relus (§13.10, §13.14) |
| [ville.md](ville.md) | Les **attentes** s'appuient sur les usages publics et les horaires des lieux (§13.8) ; lieux des pratiques sociales (cimetière, salle des fêtes, salle de réunion) (§13.9) |
| [population.md](population.md) | État initial des nouveaux composants (voix tirée de l'origine, du milieu et du métier ; pensées, conclusions et valeurs apprises données par la génération puis par le rodage) |
| [economie.md](economie.md) | Les **valeurs apprises** sur les commerces (qualité, prix, honnêteté face à l'arnaque, apprise en une fois et tenue aussi en statistique) alimentent le choix des commerces et la fidélité des clients (§13.6 ; [social.md](social.md), §5.9) |

### Socle

| Fichier | Impact | État |
|---|---|---|
| [01-concept.md](../01-concept.md), §2 | Vitesse du temps en solo hors accélération : une heure de jeu = une heure réelle | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §3 | Météo et saisons, reliées au besoin de confort et au choix des activités (§1.1, §2.2) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §2 | Ajouter un trait **chronotype** (lève-tôt ↔ couche-tard), tiré à la création et modulé par l'âge (§2.1). Les besoins détaillés (§1.1) s'ajoutent aux états temporaires : soif, vessie, hygiène, confort, sécurité, lien social, détente, stimulation | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4 et [04-joueur.md](../04-joueur.md), §5 | L'état « au repos » exige aussi **l'absence de menace perçue** et une **durée minimale de stabilité** (§7.1) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4 | Personnage à l'extérieur non jouable (§9) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md) et [04-joueur.md](../04-joueur.md), §2 | Les personnages **vieillissent** avec le calendrier du monde ; le **profil des enfants évolue** selon leur vie puis se fixe vers 18 ans, âge où ils deviennent jouables (§8.5) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1.1 | Barème des objectifs du personnage (§5.4) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1.1 | Règle « pas de bonus pour un objectif atteint par un moyen contraire à sa nature », objectifs de travail découpés en tâches à cause du temps réel (§2.3, §5.4) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §2 et §3 | La **signature comportementale** des actions sert aussi à l'IA pour choisir ses moyens (§1.4) : une seule définition pour les deux usages | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §5 | **Réservation courte** d'un personnage qui n'est pas au repos (§7.3) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §6 | **Intuitions** du personnage incarné ; la règle des rôles invisibles ne concerne que les personnages non incarnés (§5.5) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §7 | Souvenir complet avec **dissonance**, engagements tenus **selon le caractère** (§4.5, §5.3) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md) et [03-vie-synchronisation.md](../03-vie-synchronisation.md), §2 | Atteindre une sortie de la carte met fin à l'incarnation, après confirmation, avec un bilan normal (§9.5). Pour le **personnage persistant**, une sortie de la carte est un lieu de retrait ([02-personnages.md](../02-personnages.md), §1.1) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.3 | Détail du mode **observation** de l'IA pendant l'incarnation (§5.1) et de la reprise dans le même tick (§5.3) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.6 | Trois niveaux de détail (complet, simplifié, statistique), **plancher selon l'importance**, bulle simplifiée en solo accéléré, transitions (§6) ; partage avec la ville : la ville définit où, l'IA définit quoi | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2 et §4 | Planification **hybride HTN + GOAP** ; **rôles décrits en données** ; seules les compétences d'exécution seront entraînées, jamais les décisions | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), validation des intentions | **Deux étages** : admissibilité épistémique (liste des actions, signe de réticence, coût estimé, masques d'entraînement ; règles absolues et garde hors fiction pour un pilote `Joueur`), puis résolution réelle avec échec perceptible ; codes de rejet fermés (§3.7 ; S4) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1, et [05-architecture.md](../05-architecture.md) | **Couche cachée** tirée par monde avec sa graine (détenteurs des savoirs opaques, caches et coffres, informateurs, policiers corruptibles), suivie d'un rodage propre ; savoirs opaques toujours des références (§4.4, §6.5 ; S3, E25) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 (pénalité : [04-joueur.md](../04-joueur.md), §1.1) | **Point de rupture** : l'acte dont le coût estimé dépasse la synchronisation restante n'a pas lieu ; le personnage renonce par un geste naturel (§5.3, §5.6 ; E1) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1.1 | **Barème du travail** (1 à 5 XP par tâche, 30 par heure réelle) et **objectifs liés** (§5.4 ; E11, E13), qui sortent de la fourchette 10 à 200 XP pour les tâches | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §4 et §5 | **Repos de l'IA** après une incarnation (§6.2 ; E17) ; **une seule réservation active** par compte, 10 minutes d'attente après un refus (§7.3 ; E30) | Intégré au socle le 29/10 |

### Domaine ville

- Les lieux doivent être **typés avec les activités qu'ils offrent** (manger, dormir, travailler, se laver, se divertir), leurs **horaires d'ouverture** et leur **capacité**.
- Le **graphe des rues, des pièces et des lieux avec des durées** sert à la simulation simplifiée et statistique (§6.1 ; [ville.md](ville.md), §8.3).
- Les zones des trois niveaux de détail sont définies par la ville ([ville.md](ville.md), §8.2) ; la ville reprend le **plancher des personnages importants** et la **bulle simplifiée autour du joueur** en solo accéléré (§6.1).
- Chaque personnage a un **domicile** et un **lieu de travail** attribués.
- Des **lieux jugés dangereux** ou sûrs alimentent le besoin de sécurité.
- Des **lieux de nuit** (bars, services, transports) sont nécessaires pour que la nuit en ligne ne soit pas vide.
- **Enfants** : crèches, assistantes maternelles, garderies et centres de loisirs, en plus des écoles ; horaires scolaires ; aires de jeux (§8).
- **Points de sortie** : le personnage qui part ou revient passe réellement par la gare, la rocade ou un ponton (§9.1, §9.3).
- **Couche cachée** : la ville fournit les emplacements possibles des caches et les coffres ; leur contenu et l'emplacement exact sont tirés par monde (§4.4 ; S3, A1). Les **usages publics** des lieux (horaires, accès) servent aux présomptions des états inconnus (§3.7).

### Domaine population ([population.md](population.md))

Demandes de ce domaine ([population.md](population.md), §12.3) intégrées en v0.4 : situations de vie (§2.5), projets familiaux et projets de départ (§3.4), naissances et garde alternée (§8), visites et contacts à distance (§9.1), multiplicateur (§9.4), rodage (§6.5).

- **Création initiale** des habitants, ménages, emplois et logements, dont les enfants et leur profil de départ.
- **Réglage global** des déménagements définitifs, pour garder une population stable, et **arrivées** de nouveaux habitants (§9.4).
- **Naissances** : réglées, de vraies naissances ([population.md](population.md), §7.2).
- Les **proches à l'extérieur** (relations abstraites) servent de motif aux visites (§9.1).
- **Couche cachée** : à la création de chaque monde, le tirage des détenteurs des savoirs opaques, des informateurs et des policiers corruptibles, puis le rodage propre de 14 jours de jeu (§4.4, §6.5 ; E25, S3, A1).

### Domaine économie

- Les **métiers** fournissent les fiches de rôle : horaires, tâches (qui deviennent des objectifs, §2.3), salaires, recettes propres.
- Une part d'**emplois de nuit** est nécessaire (§2.3).
- Les **prix** influencent le choix des recettes (§3.3).
- La simulation statistique a besoin de **revenus et dépenses agrégés** par bloc d'agenda (§2.4).
- Demandes de ce domaine ([economie.md](economie.md), §14.3) intégrées en v0.4 : fiches de métier (§0), dépendances (§1.1), coût relatif (§3.2), recettes économiques (§3.3), projets économiques (§3.4), réparation d'une opération dissonante (§5.3), automate de gestion (§6.5).
- **Enfants** : frais de garde, argent de poche, petits boulots et jobs d'été des adolescents (§8).
- **Séjours** : congés payés, coût des vacances, frais de déplacement professionnel, salaire pendant un déplacement ou une détention (§9.2).
- **Revues** : délai de rétractation de 7 jours pour les contrats immédiats, promesses de paiement et dettes au profit du cercle (§5.3 ; E6) ; barème des tâches de travail, 1 à 5 XP, plafonné à 30 par heure réelle, sans les tâches créées par le cercle (§5.4 ; E11).

### Domaine social

- La **dynamique des relations** (affection, confiance, respect, crainte) et leur effet sur le graphe social et la composante « Réseau ».
- La **propagation des savoirs et des rumeurs**, y compris le transfert de savoirs opaques (§4.4).
- Les **conversations entre PNJ**, abstraites (§3.6).
- Les **groupes, organisations et projets collectifs** ([social.md](social.md), §6.9), dont la **délégation des responsabilités** d'un personnage important absent ([social.md](social.md), §6.8 ; §9.2).
- Demandes de ce domaine ([social.md](social.md), §12.2) intégrées en v0.4 : actes sociaux comme recettes, choix des interlocuteurs et des sujets (§3.3, §3.6), croyances et format des relations (§4.2, §4.3), s'expliquer et se rétracter à la reprise (§5.3), projets collectifs (§3.4), délégation (§9.2).
- La **réputation** et les réactions à l'hygiène ou aux actes dissonants.
- **Enfants** : fiabilité de leur parole (§8.3), amitiés d'école, réactions à un enfant négligé (§8.4).
- **Revues** : une reconnaissance de dette ou une promesse de paiement au profit du cercle ouvre un délai de rétractation (§5.3 ; E6).

### Domaine combat

- Les **réflexes** (fuir, se défendre, se figer) et les seuils de violence (§3.1).
- Les compétences de combat, fuite et poursuite, d'abord écrites à la main puis entraînées (§3.5).
- Les **témoins** : selon leur respect des règles et leur bienveillance, ils appellent la police, interviennent, fuient ou se taisent.
- Les routines et réactions des policiers relèvent des fiches de rôle.
- **Enfants** : réflexe de protection des adultes (§8.4), poids d'un témoignage d'enfant (§8.3), règles de violence envers les enfants.
- **Détention** : décision de la justice, durée, et effets au retour (désentraînement, casier, contacts) (§9.2).
- Demandes de ce domaine ([combat.md](combat.md), §16.3) intégrées en v0.5 : réflexes et panique de foule (§3.1), observation aux moments clés d'un affrontement (§5.1), recettes face à la violence, à la justice, crimes et métiers (§3.3), souvenir structuré des témoins (§4.2), garanties envers les mineurs (§8.6), bracelet, suivi, jurés et témoins (§2.5), détention (§9.2), fuite à l'extérieur (§9.1).
- **Revues** : le point de rupture rend l'éjection **toujours** antérieure à l'acte irréparable (§5.3 ; E1) ; abandonner un poste de garde ou de service en quittant la carte est un acte dissonant (§9.5 ; E18) ; informateurs et policiers corruptibles sont tirés par monde (§4.4 ; A1).

### Domaine interface

- Forme des **intuitions** (§5.5) et leur précision selon la synchronisation et la familiarité.
- Affichage de l'indisponibilité d'un personnage et de la **réservation** en cours (§7.3).
- Possibilité pour le joueur d'**empêcher un geste automatique** (§5.2).
- **Confirmation** avant de franchir une sortie de la carte avec un personnage incarné (§9.5).
- Affichage d'un personnage visé qui est **à l'extérieur** (indisponible jusqu'à son retour).
- Demandes de ce domaine ([interface.md](interface.md), §15.2) intégrées en v0.5 : intuitions structurées et touche Réfléchir, « ce qu'il dirait », signe de réticence, capacités en cinq degrés (§5.6), annonce des gestes automatiques (§5.2), interruptions de l'avance du temps (§2.4), plan de mémoire des étages (§4.2).
- **Revues** : la liste des actions possibles est celle de l'étage épistémique, et les refus utilisent des codes fermés (§3.7 ; S4) ; signe de réticence toujours visible pour un acte au point de rupture et intuition « Il ne peut pas s'y résoudre. » (§5.6 ; E1) ; une seule réservation active par compte (§7.3 ; E30) ; repos de l'IA parmi les indisponibilités (§6.2 ; E17).

### Service de dialogue

- Il ne reçoit **jamais la valeur d'un savoir opaque** (§4.4), seulement le fait que le personnage le connaît.
- Il reçoit l'**intention de réponse** décidée par le cœur, ainsi que les souvenirs et relations utiles, sous forme de faits structurés.
- Le classement du texte libre d'un joueur ne peut produire qu'un acte de la liste des actions possibles de l'étage épistémique (§3.7 ; S4).
- Il reçoit aussi, depuis la v0.8, la voix, l'humeur et l'état du discours du personnage (§13.10), et peut servir hors ligne à proposer des contenus (§13.14).

---

## 12. Revue de l'intelligence des PNJ

> Revue faite en v0.8 par la session de domaine, à la demande de l'utilisateur, du point de vue d'un spécialiste de l'IA de personnages de jeu et des agents crédibles. Les réponses sont au §13, la mesure au §14. Les §12 à §14 viennent après les impacts pour ne pas changer la numérotation à laquelle les autres fichiers renvoient.

### 12.1 Verdict

Le modèle est solide pour **faire vivre** 10 000 habitants : utilité pour choisir, recettes HTN avec un secours GOAP pour agir, rôles en données, une IA qui ne sait que ce que sait le personnage, et une même signature pour l'IA et pour la synchronisation. C'est l'état de l'art des jeux à grande échelle, et il tient dans les contraintes de l'architecture. Mais, jusqu'à la v0.7, il simulait une **vie** plus qu'un **esprit** : il manquait les couches qui relient ce qui arrive à ce que le personnage ressent, retient, conclut et suppose des autres, et la surface d'imperfection qui fait paraître humain. Ces couches s'ajoutent sans modèle de langage dans la boucle, de façon déterministe et dans le budget (§13).

### 12.2 Comparaison avec l'état de l'art

| Référence | Ce qu'on reprend | Ce qu'on évite |
|---|---|---|
| **The Sims** | Besoins qui montent, lieux et objets qui « annoncent » ce qu'ils satisfont (déjà : lieux typés par activité) | Personnages sans passé ni avenir, qui oscillent d'un besoin à l'autre, sans conséquence durable |
| **Dwarf Fortress** | Personnalité fine et valeurs ; souvenirs qui reviennent et raniment une émotion ; stress cumulé jusqu'à la crise ; histoires qui émergent | Cascades incontrôlées (crises en chaîne), comportements absurdes non testés, coût par agent sans borne, opacité |
| **RimWorld** | Humeur faite de **pensées** datées, cumulées avec plafond, explicables ; seuils de crise | Le « conteur » qui provoque des événements pour l'intrigue : ce monde ne triche pas |
| **Shadows of Doubt** | Chaque habitant a un domicile, un travail, une routine et des traces vérifiables | Habitants creux, répliques répétées, **indifférence à l'anormal** (un corps, une porte forcée) |
| **Crusader Kings** | Stress d'agir contre ses traits (notre dissonance) ; opinions faites de causes datées qui s'estompent ; complots secrets avec risque de découverte ; secrets qui donnent prise | Tout réduire à une opinion chiffrée ; formules connues que les joueurs optimisent ; aucune vie dans l'espace |
| **Versu, Comme il Faut (Prom Week)** | **Pratiques sociales** en données (rôles, actes attendus, normes) ; volonté faite de nombreuses petites règles | Un coût d'écriture qui croît avec chaque scène ; tout faire passer par le texte |
| **Radiant AI (Oblivion)** | Leçon négative : des buts libres sans prix social ni prévision des conséquences ont produit vols et meurtres absurdes, puis une IA bridée | Lâcher des buts sans filtre des moyens ; livrer sans tester à grande échelle. Nos parades : filtre des moyens par le profil, risque perçu, police, `ts-sim` |
| **Generative Agents (Stanford, 2023)** | Flux de souvenirs, **rappel** par récence, importance et pertinence ; **réflexion** qui tire des conclusions ; plan du jour décomposé et replanification | Un modèle de langage à chaque pas : coût prohibitif pour 10 000 habitants, non reproductible, hallucinations, dérive, injections des joueurs |
| **Théorie de l'esprit** (PsychSim et travaux voisins) | Modèle d'autrui et croyances emboîtées pour la ruse et le soupçon | La récursion sans borne : on s'arrête au second ordre et aux faits sensibles |
| **OCC** | Évaluer un événement selon les buts (joie, détresse, espoir, crainte), les normes (fierté, honte, reproche, admiration) et les goûts | Écrire les 22 émotions : on garde celles qui changent une décision |
| **PAD** | Humeur lente sur trois axes (plaisir, activation, dominance) qui colore tout | Une humeur sans cause : chaque valeur s'explique par des pensées |
| **Utilité contre planification** (utilité à axes multiples, GOAP de F.E.A.R., HTN de Killzone et d'Horizon, BDI) | Utilité réactive pour choisir, planification pour enchaîner, **engagement** envers l'intention choisie | GOAP partout (coût), utilité sans engagement (hésitation sans fin) |

### 12.3 Évaluation par critère

| Critère | Ce qui marche | Ce qui manquait | Gravité | Réponse |
|---|---|---|---|---|
| Cohérence sur des semaines et des mois | Quatre mémoires, bilans des relations, projets, oubli et consolidation | Aucun rappel des souvenirs au moment de décider ; aucune conclusion générale (« il me méprise ») ; rancunes réduites à des chiffres | Majeure | §13.3, §13.4 |
| Variété des journées | Agenda, chronotype, météo, régularité selon le profil | Choix identiques d'un jour à l'autre ; ni habitudes ni occasions | Moyenne | §13.6, §13.7 |
| Raisonnement social | Croyances avec source et certitude, mensonge, réputation, discrétion, témoins | Personne ne sait ce que l'autre sait ; un menteur ne se souvient pas de ce qu'il a dit ; pas de soupçon | Majeure | §13.5 |
| Émotions et humeur | États temporaires qui modulent le choix | Pas d'évaluation des événements, pas d'humeur lente, pas d'émotion envers une personne, pas de crise | Majeure | §13.1, §13.2 |
| Imprévu et improvisation | Sélection relancée par les événements, GOAP, réflexes, panique | Aucune notion d'**anomalie** : un inconnu chez soi ou une porte forcée ne surprennent personne | Majeure | §13.8 |
| Apprentissage par l'expérience | Relations, croyances corrigées, désentraînement | Rien n'apprend qu'un lieu est dangereux ou qu'un commerçant triche ; aucune opinion ne change | Majeure | §13.4, §13.6 |
| Groupes et foules | Panique par contagion ; groupes et projets collectifs du social | Ni groupes d'occasion (marcher ensemble, file, cercle), ni attroupements, ni scènes sociales normées | Moyenne | §13.9 |
| Crédibilité de la conversation | Le cœur décide, le modèle formule ; moindre fait ; repli systémique | Rien ne retient ce qui a été dit (répétitions, pas de suites) ; pas de voix propre ; ton du joueur ignoré | Majeure | §13.10 |
| Transitions entre niveaux de détail | Reconstruction depuis l'agenda, hystérésis | Un événement tiré en statistique peut ne laisser aucune trace ; une reconstruction peut contredire ce qu'un joueur a vu | Moyenne | §13.12 |
| Résistance aux manipulations | Filtre des moyens, signatures, validation épistémique, résolution par les attributs, rendements décroissants du social | Ni agacement au-delà de la journée, ni méfiance, ni habituation : questions, cadeaux, provocations et appâts répétés restent payants | Majeure | §13.11 |
| Paraître humain à côté des joueurs | Mêmes intentions, mêmes limites, délai de formulation commun | **Perfection** : réaction au premier tick, trajet optimal, aucun geste au repos, aucune hésitation, aucune erreur | **Critique** | §13.7, §13.10, §14.3 |

### 12.4 Les cinq lacunes principales et leurs réponses

1. **Émotions sans évaluation ni humeur** → évaluation des événements selon les buts, les normes et les goûts ; pensées datées ; humeur lente ; crises (§13.1, §13.2).
2. **Mémoire sans rappel ni réflexion** → rappel indexé par sujet et réflexion nocturne déterministe, qui produit des conclusions et des thèmes de vie (§13.3, §13.4).
3. **Pas de théorie de l'esprit** → modèle d'autrui borné : traits supposés, faits sensibles au second ordre, registre de ce qu'on a dit, soupçon (§13.5).
4. **Ni anomalies ni apprentissage** → attentes et réactions à l'anormal ; valeurs apprises, aversions et habitudes (§13.6, §13.8).
5. **Surface trop parfaite et manipulable** → naturel (délais humains, hésitations, gestes, oublis), lassitude, méfiance et habituation, voix propre, tests à l'aveugle (§13.7, §13.10, §13.11, §14.3).

### 12.5 Deux choix rediscutés

- **Décisions non entraînées : maintenu**, avec un calibrage hors ligne des poids (§13.13).
- **Modèle de langage cosmétique en jeu : maintenu**, avec trois usages permis, dont aucun ne décide en jeu (§13.14).

---

## 13. Mécanismes d'intelligence et de réalisme

Chaque mécanisme est **(décidé)**, avec sa raison, son coût et ses niveaux de détail.

**Unités de travail** : une **opération** est une lecture, une écriture ou un calcul élémentaire sur un composant (un score, une mise à jour de jauge), dénombrée comme les nœuds de GOAP (§6.4) ; aucun coût n'est exprimé en temps ([05-architecture.md](../05-architecture.md), §2.10). Tous les coûts sont **[à calibrer]**.

**Budget global** : un an pour 10 000 habitants en moins de 5 minutes laisse environ 80 microsecondes de calcul par personnage et par jour simulé ([05-architecture.md](../05-architecture.md), §2.22). L'ensemble du §13 doit tenir dans **500 opérations par personnage et par jour en statistique** **[à calibrer]**, mesurées par `ts-sim` (§14.2). En cas de dépassement, on simplifie d'abord la réflexion en statistique (une nuit sur deux, règles réduites), puis la fréquence des pensées.

### 13.1 Évaluation des événements (décidé)

Chaque événement perçu d'une importance suffisante **[seuil à calibrer]** (§4.1) est évalué une fois, selon les trois questions du modèle OCC :

- **Pour mes buts** : l'événement sert-il ou contrarie-t-il un besoin, un engagement, un projet ou une motivation (§1, §3.4) ? Cela donne **joie** ou **détresse** ; s'il annonce une issue future, **espoir** ou **crainte** ; quand l'issue arrive, **soulagement** ou **déception**.
- **Pour mes normes** : l'acte qui le cause est-il louable ou blâmable ? Pour soi : **fierté** ou **honte** (la dissonance du §4.5 vue de l'intérieur) ; pour un autre : **admiration** ou **reproche**. (décidé ; G28) Le **blâme** est le verdict d'`appraise` sur les **croyances du témoin** (ce qu'il a vu, ce qu'il sait de la cible et du contexte), avec **ses** valeurs : un coût élevé est blâmable ; l'**admiration** vient des effets d'aide (`Help`) sous sa tolérance. Un témoin qui n'a pas vu la provocation juge donc plus sévèrement qu'un témoin qui l'a vue. *Pourquoi : la même fonction sur d'autres croyances donne des jugements différents et réalistes, sans signature « vue de l'extérieur ».*
- **Pour mes goûts** : la chose ou la personne me plaît-elle ? Cela donne **attirance** ou **aversion**.

Combinaisons retenues :

- reproche et détresse donnent la **colère** envers l'auteur ;
- admiration et joie donnent la **gratitude** ;
- la détresse d'autrui donne la **compassion** chez un bienveillant, une **satisfaction maligne** chez un personnage peu bienveillant ou envers un ennemi.

Le résultat met à jour les états temporaires existants (§1.3), crée une **pensée** (§13.2) et, si l'événement vient d'une personne, une **émotion envers elle**, qui entre dans le bilan de la relation si c'est une **relation complète** ([social.md](social.md), §1.1). On ne garde que les émotions qui changent une décision ; les autres nuances relèvent de la formulation.

*Pourquoi : c'est le chaînon manquant entre ce qui arrive et ce que ressent le personnage, et il n'utilise que des buts, des signatures et des relations qui existent déjà.*

**Coût** : 10 à 30 opérations par événement évalué. **Niveaux** : complet et simplifié, pour chaque événement notable ; statistique, une évaluation par événement tiré au sort et une pour le résultat global de chaque bloc d'agenda.

### 13.2 Pensées datées, humeur et crises (décidé)

- **Pensées** : une pensée est la trace émotionnelle d'un événement : cause (le souvenir), type (deuil, humiliation, réussite, dispute, compliment, besoin négligé), valence, intensité et durée de vie.
  - Les pensées d'un même type se **cumulent** jusqu'à un plafond, puis s'**estompent** à une vitesse réglée par la régulation émotionnelle (attribut) et la stabilité émotionnelle (profil).
  - Au plus **16** pensées sont actives **[à calibrer]** ; les plus faibles s'effacent.
  - Un souvenir rappelé (§13.3) peut **ranimer** une pensée : revoir celui qui vous a humilié.
- **Humeur** : trois valeurs lentes, **plaisir**, **activation** et **dominance** (modèle PAD), calculées comme une somme pondérée des pensées actives, de l'état des besoins et du stress. Elle n'a jamais de valeur sans cause : on peut toujours dire pourquoi un personnage est d'humeur sombre.
- **Effets** sur les décisions (§3.2) :
  - plaisir bas : moins d'initiative sociale, plus de repli et de consolations (alcool, nourriture, écrans, selon le profil) ;
  - activation haute : impulsivité, réactions immédiates plutôt que plans (comme la colère, §1.3) ;
  - dominance basse : soumission et fuite parmi les réflexes ; dominance haute : confrontation.
  - L'humeur règle aussi le **ton** transmis au dialogue (§13.10) et les **signes visibles** (posture, démarche, expression), rendus par l'interface.
- **Crises** : quand le plaisir reste sous un seuil assez longtemps **[à calibrer]**, le personnage peut faire une crise, choisie selon son profil parmi des recettes réalistes :
  - pleurer, s'isoler, rester au lit, s'enivrer, exploser contre un proche, partir marcher, appeler un ami, consulter ;
  - une crise soulage (pensées allégées) et laisse des souvenirs chez les témoins ;
  - **pare-feu** : une crise ne peut pas en déclencher une autre chez un témoin dans la même journée **[à calibrer]**, et une crise violente reste filtrée par le profil et par les garanties envers les mineurs (§8.6).

*Pourquoi : une humeur faite de causes datées est stable, explicable et lisible, et ses débordements, réglés par le profil et bornés, donnent les moments marquants de Dwarf Fortress et de RimWorld sans leurs cascades.*

**Coût** : environ 5 opérations par pensée ajoutée ; environ 20 pour recalculer l'humeur, seulement quand une pensée change ou à chaque bloc d'agenda. **Niveaux** : tous ; en statistique, pensées et humeur sont mises à jour une fois par bloc.

### 13.3 Rappel des souvenirs (décidé)

Quand une décision porte sur une personne, un lieu ou un sujet (choisir à qui parler, où aller, que répondre, évaluer un événement), l'IA rappelle les **3 souvenirs** **[à calibrer]** les plus utiles, d'après le score `récence × importance × pertinence` des agents génératifs. Ce calcul porte sur des faits structurés :

- la **pertinence** est le nombre d'éléments communs (même personne, même lieu, même type d'acte), jamais une similarité de texte ;
- les souvenirs sont **indexés par sujet** (personne, lieu) : le rappel ne parcourt que la liste du sujet.

Un souvenir rappelé :

- pèse sur le score de l'objectif (la mauvaise soirée dans ce bar) ;
- ranime une pensée (§13.2) ;
- fournit un sujet de conversation (« alors, ton entretien ? », §13.10) ;
- voit son importance renforcée, ce qui ralentit son oubli (§4.3) : on se souvient de ce qu'on se rappelle.

*Pourquoi : c'est ce qui fait peser le passé sur le présent, et un index par sujet le rend borné, déterministe et bon marché.*

**Coût** : au plus 40 opérations par rappel. **Niveaux** : complet et simplifié ; en statistique, seuls le bilan des relations et les conclusions (§13.4) servent.

### 13.4 Réflexion nocturne, conclusions et thèmes de vie (décidé)

Une fois par jour de jeu, pendant le sommeil, chaque personnage fait une **réflexion**. C'est une tâche quotidienne de l'ordonnanceur. Un jeu de **règles écrites en données** parcourt les souvenirs et les pensées de la journée et en tire des **conclusions** : des croyances générales avec une certitude (§4.2).

| Ce que le personnage a vécu | Conclusion | Effet |
|---|---|---|
| Trois mauvais souvenirs avec X en 30 jours | « X m'en veut » ou « X est malhonnête » | Méfiance, évitement, refus |
| Agressé ou effrayé dans un lieu | « Ce lieu est dangereux » | Aversion (§13.6), besoin de sécurité |
| Aidé plusieurs fois par Y | « Je dois quelque chose à Y » | Engagement de rendre service |
| Deux échecs sur la même étape de projet | Découragement | Abandon ou changement de moyen (§3.4) |
| Humilié en public | **Rancune** contre l'auteur | Projet de réparation ou de vengeance, selon le profil |
| Mort d'un proche | **Deuil** | Pensée longue, projets de transmission |

- **Biais du profil** : les conclusions dépendent de qui conclut. Un méfiant conclut plus vite à la malveillance ; un bienveillant cherche une excuse (biais d'attribution). Elles peuvent donc être **fausses**, et les faits suivants les corrigent comme toute croyance. *Pourquoi : c'est ce qui rend deux personnages différents face aux mêmes faits.*
- **Thèmes de vie** : les conclusions fortes et durables (rancune, deuil, ambition contrariée, amour, honte d'un acte) deviennent des **thèmes**, au plus 3 **[à calibrer]**. Pendant des mois, ils orientent les projets (§3.4), les sujets de conversation et les intuitions du joueur incarné (§5.5). Une rancune s'éteint par le **pardon**, selon le profil et les réparations reçues ([social.md](social.md), §1.5) : un **pardon accepté** éteint la conclusion de rancune **et** le thème de vie qui va avec (décidé ; *pourquoi : on ne peut pas avoir pardonné et continuer à vivre sa vie autour de cette rancune*) ; il reste le souvenir de l'offense, qui s'oublie normalement.
- **Changer d'avis** : le profil ne change pas (socle, [04-joueur.md](../04-joueur.md), §7), mais les **attitudes** changent : l'opinion sur une personne, un lieu, un commerce ou un sujet public. Les conclusions, la persuasion ([social.md](social.md), §3.1) et les valeurs apprises (§13.6) les modifient.
- La réflexion prépare aussi le **lendemain** : rendez-vous, étapes de projet à faire avancer, personnes et lieux à éviter.

*Pourquoi : c'est la réflexion des agents génératifs sans modèle de langage : des règles sur des faits structurés produisent des conclusions qui changent la conduite pendant des semaines, de façon reproductible.*

**Coût** : au plus 200 opérations par personnage et par jour **[à calibrer]**, en une seule tâche quotidienne. **Niveaux** : tous ; en statistique, sur les résumés de la journée.

### 13.5 Modèle d'autrui : ce que A croit que B sait (décidé)

Trois éléments s'ajoutent à la relation, bornés pour rester bon marché.

**Relations complètes seulement** (décidé ; [social.md](social.md), §1.1) : les champs étendus des relations (traits supposés, ici ; émotion envers la personne, §13.1 ; sujets échangés, §13.10 ; compteurs de lassitude, §13.11) ne sont tenus que dans les **relations complètes** (intimes, proches, connaissances suivies). La forme compacte des visages et des noms ne les garde pas, et une relation qui y descend les **perd** ; si elle remonte, ils repartent de son impression générale. Les faits sensibles et le registre des déclarations, qui appartiennent au personnage et non à la relation, sont gardés. *Pourquoi : on ne prête un caractère, une émotion durable ou une conversation suivie qu'à ceux qu'on connaît vraiment, et c'est ce qui borne le coût de ces champs.*

1. **Traits supposés** : l'idée que A se fait du caractère de B sur cinq axes (honnêteté, violence, fiabilité, bienveillance, respect des règles), avec une certitude. A l'estime d'après les actes de B qu'il a vus ou appris, par **révélation** (décidé ; G13) : un acte de gravité γ sur un axe montre que B tolère au moins `T(γ)` ; la croyance de A se resserre vers cette borne, avec une certitude qui croît avec le poids `W(γ)` ([revues/signatures.md](../revues/signatures.md), §3.11). *Pourquoi : ce que coûte un acte et ce qu'il révèle de celui qui le fait sont la même information.* Elle sert à **prévoir** B : acceptera-t-il, me dénoncera-t-il, se vengera-t-il ? Elle peut être fausse : un menteur habile paraît honnête.
2. **Faits sensibles au second ordre** : pour au plus **8** faits **[à calibrer]** qui le compromettent, lui ou un proche (un crime, une liaison, une dette cachée, un mensonge, un secret), le personnage tient la liste de **qui, selon lui, le sait**, avec une certitude.
   - Sources : un témoin qu'il a vu le voir, quelqu'un à qui il l'a dit, quelqu'un qui a pu l'apprendre d'une personne connue.
   - On s'arrête au **second ordre** (« je crois que B sait ») ; au-delà, c'est une rumeur ordinaire.
   - Effets, selon le profil : peur d'être dénoncé, éviter ou ménager B, le menacer, l'acheter, préparer une version, avouer avant d'être découvert. Ces conduites passent par les recettes de [combat.md](combat.md) et de [social.md](social.md).
3. **Registre des déclarations** : pour ces faits, ce qu'il a dit et à qui (la version donnée). Un menteur **reprend sa version**. Il peut s'en écarter s'il l'a oubliée (mémoire, stress, ivresse) : c'est ainsi qu'il se trahit.

**Soupçon** : c'est une croyance sur les **intentions** d'autrui (« B veut me tromper, me voler, me nuire »).

- Il naît d'indices : deux versions d'un même fait par la même source, une version contraire à ce qu'on a vu, une anomalie (§13.8), une générosité hors des usages (§13.11), une mauvaise réputation, une approche menaçante.
- Il baisse la confiance et rend prudent : vérifier, se renseigner, refuser, prévenir. Il peut devenir une conclusion (§13.4).
- Les **affaires** de police, faites de croyances, reçoivent les mêmes indices ([combat.md](combat.md), §6.2).

*Pourquoi : savoir ce que l'autre sait est le cœur de la ruse, du soupçon et de la peur d'être découvert ; le borner aux faits sensibles et au second ordre évite l'explosion des croyances emboîtées.*

**Coût** :

- traits supposés : environ 5 opérations par acte observé ;
- faits sensibles : environ 10 opérations par propagation qui les touche ;
- déclarations : environ 3 opérations par acte.

**Niveaux** : complet et simplifié ; en statistique, les traits supposés sont mis à jour par bloc, et les faits sensibles par la propagation des rumeurs, déjà statistique ([social.md](social.md), §10.1).

### 13.6 Apprentissage par l'expérience (décidé)

- **Valeurs apprises** : chaque personnage tient une valeur apprise pour chaque chose qu'il fréquente :
  - lieux (sécurité, agrément) ;
  - commerces (qualité, prix, **honnêteté**) ; l'honnêteté mesure l'**arnaque** subie : surfacturation, monnaie mal rendue, produit défectueux, prestation payée mais mal faite ou non faite ([economie.md](economie.md), §4.4) ;
  - trajets (durée réelle) ;
  - moyens (recettes qui ont marché ou échoué pour lui) ;
  - activités (plaisir réel).
- **Mise à jour** : après chaque résultat, `valeur ← valeur + taux × (résultat − valeur)`.
  - Le taux monte avec l'intensité : une agression suffit à faire fuir un parc (apprentissage en une fois), un café médiocre ne se juge qu'à la longue.
  - **Arnaque découverte** (décidé ; [economie.md](economie.md), §4.4 et §14.3) : la valeur d'honnêteté s'apprend **en une fois**, avec un taux fort **[à calibrer]**, comme l'agression ; elle revient vers la valeur de départ plus lentement que les autres **[à calibrer]**. *Pourquoi : un client trompé une fois ne fait plus confiance, alors qu'un prix un peu élevé ne se juge qu'à la longue.*
  - Au plus **32** valeurs **[à calibrer]** ; les moins importantes s'effacent.
  - Elles reviennent lentement vers la valeur de départ : on finit par retourner au parc.
- **Effet sur les décisions** : elles entrent dans la **faisabilité** et l'**urgence** du score (§3.2). On évite ce qui a blessé, on revient à ce qui a plu.
- **Habitudes** : un choix répété et satisfaisant reçoit un bonus d'**habitude** (son café, son banc, son trajet). C'est ce qui donne à chacun des préférences propres, et à la ville ses habitués.
- **Valeurs de départ au domicile** (décidé ; P11, [revues/generation.md](../revues/generation.md), §4.5) : les valeurs apprises données à la génération portent aussi sur le **mobilier mobile** du ménage (son fauteuil, son côté du lit, sa place à table), produit à l'emménagement et emporté au déménagement ; elles suivent l'objet, pas la pièce. *Pourquoi : les habitudes d'un foyer tiennent à ses meubles, qui déménagent avec lui.*
- **Incarnations** : les valeurs apprises viennent de tout ce que le personnage a vécu, y compris pendant une incarnation (§4.5), et le joueur incarné les retrouve dans ses intuitions.
- Aucun entraînement : une moyenne mobile par chose, déterministe.

*Pourquoi : changer de conduite après une expérience est l'apprentissage le plus visible, et une moyenne mobile bornée ne coûte presque rien.*

**Coût** : 1 à 3 opérations par résultat, 1 lecture par candidat évalué. **Niveaux** : tous ; en statistique, sur les résultats des blocs et des événements tirés. Cela vaut aussi pour l'honnêteté des commerces (décidé ; [economie.md](economie.md), §9.3 et §14.3) : une arnaque tirée en statistique met à jour la valeur du client comme en simulation complète. *Pourquoi : sans cela, la clientèle d'un commerce malhonnête ne baisserait que là où un joueur regarde.*

### 13.7 Variété maîtrisée et naturel (décidé)

- **Choix parmi les meilleurs** : quand plusieurs objectifs ou recettes ont des scores proches **[écart à calibrer]**, l'IA tire parmi les **trois meilleurs**, avec des chances proportionnelles à leur score. Le tirage est tempéré par le profil (un personnage « Perception » varie plus, un « Jugement » moins) et par les habitudes (§13.6). Il utilise l'aléa à clé du personnage, donc il est reproductible. Quand les scores ne sont pas proches, le meilleur choix l'emporte, sans hasard.
- **Occasions** : une rencontre fortuite (un ami croisé, une vitrine, une affiche, une musique entendue) crée un objectif candidat : prendre un café, entrer, s'arrêter. Il est évalué comme les autres. C'est la principale source de variété réaliste.
- **Naturel** : en simulation complète, sur un **flux d'aléa à part**, pour que ces tirages ne déplacent pas les autres :
  - **délais de réaction** tirés d'une distribution humaine, selon le temps de réaction, l'attention, la fatigue et l'ivresse, la même distribution que celle des joueurs : aucune réaction parfaite au premier tick ;
  - **hésitations** quand l'enjeu est grand ou les scores proches : s'arrêter, regarder autour, revenir sur ses pas ;
  - **gestes au repos** : regarder son téléphone, s'étirer, ajuster ses vêtements, observer les passants ;
  - **regard** vers un bruit, vers l'interlocuteur, vers ce qui bouge ;
  - **imperfections** : se tromper de rue, oublier ses clés ou son parapluie, changer d'avis en chemin, heurter quelqu'un et s'excuser, selon la mémoire, le stress et l'humeur. Celles qui ont des conséquences sont aussi tirées en simplifié et en statistique, pour que tous les niveaux vivent la même vie ;
  - **allure** selon l'âge, l'humeur, la hâte et la météo.

*Pourquoi : on reconnaît une IA à sa perfection plus qu'à ses erreurs ; des imperfections tirées des attributs et de l'état la rendent indiscernable sans la rendre incompétente, et un tirage limité aux meilleures options donne de la variété sans incohérence.*

**Coût** :

- tirage parmi les meilleurs : environ 10 opérations ;
- occasions : environ 5 opérations par perception notable ;
- naturel : environ 3 opérations par personnage et par seconde en complet.

**Niveaux** : tirage et occasions à tous les niveaux (en statistique, occasions tirées par bloc selon le lieu) ; naturel en complet seulement, sauf les imperfections qui ont des suites.

### 13.8 Attentes et anomalies (décidé)

- **Attentes** : chaque personnage a des attentes sur ses lieux familiers. Elles sont tirées de ce qu'il sait déjà :
  - qui devrait s'y trouver (les agendas connus de son foyer, de ses collègues) ;
  - dans quel état (porte fermée, objets à leur place : plan de mémoire, §4.2) ;
  - quel bruit il devrait y avoir (le calme la nuit) ;
  - quand ses proches devraient revenir.
- **Anomalies** : la perception compare ce qu'elle voit à ces attentes. Un **écart** est une **anomalie**, avec une gravité **[à calibrer]** :
  - un inconnu dans son logement, une porte forcée, une vitre brisée ;
  - un cri, de la fumée, un corps ;
  - un objet disparu, un proche qui ne rentre pas, une voiture qui le suit.
- **Réponses** : elles passent par des recettes filtrées par le profil et l'état (un ivre ou un distrait remarque moins) :
  - **regarder** : se retourner, s'approcher ;
  - **s'enquérir** : demander, appeler le proche ;
  - **s'inquiéter** : pensée de crainte (§13.2) ;
  - **alerter** : police, secours, voisin ;
  - **fuir** ;
  - ou ne rien faire et s'en souvenir.
- Plus la gravité est haute, plus la réaction est probable et rapide. Une anomalie non résolue reste une préoccupation, qui revient à la réflexion (§13.4).

*Pourquoi : réagir à ce qui n'est pas normal distingue un habitant d'un décor, et les attentes existent déjà dans les agendas, les relations et le plan de mémoire.*

**Coût** : environ 5 opérations par entité nouvellement perçue dans un lieu familier, ou d'une catégorie saillante (cri, fumée, arme, corps). **Niveaux** : complet ; en simplifié, seules les anomalies graves, par des règles d'événement ; en statistique, réactions tirées avec l'événement.

### 13.9 Pratiques sociales, groupes d'occasion et attroupements (décidé)

- **Pratiques sociales** : elles sont décrites en données, à la manière de Versu : repas de famille, fête, enterrement, mariage, réunion de travail, file d'attente, cercle de conversation, visite à l'hôpital.
  - Chaque pratique a des **rôles**, des **phases**, des **actes attendus** et des **normes** (ce qui est déplacé).
  - Un participant reçoit des objectifs propres à son rôle.
  - **Données déclarées** (décidé ; [social.md](social.md), §3.1 et §5.5 ; [revues/signatures-contenu.md](../revues/signatures-contenu.md), J8) :
    - la **tenue attendue** (sombre et sobre, soignée, libre), lue par le choix de la tenue (§2.6) et par le jugement des participants ;
    - pour les pratiques qui l'ouvrent, `opens: [J8]` avec les effets couverts : la **politesse** (compliment de convenance, mensonge de politesse), les **usages envers les enfants** (Père Noël, fée des dents, contes), la **plaisanterie entre amis** (taquinerie, moquerie amicale, bourrade, entre personnages amis seulement).
    - Hors d'une pratique qui l'ouvre, `appraise` juge le même acte sans J8 : la moquerie d'un inconnu reste une humiliation.
    - *Pourquoi : un usage est un fait social que les données doivent dire, sinon tout acte pourrait se réclamer du jeu.*
  - Un manquement à une norme est évalué comme un acte blâmable par les autres (§13.1), par `appraise` avec le **profil normatif** de la pratique ou du groupe (décidé ; G14), et touche la réputation ([social.md](social.md), §5.4).
- **Groupes d'occasion** : aller quelque part ensemble (marcher au même pas, s'attendre), se placer en cercle pour parler, s'asseoir ensemble, suivre un proche, faire la queue. C'est une compétence d'exécution, alimentée par un objectif commun.
- **Attroupements** : un incident visible (accident, bagarre, malaise, arrestation) attire des **curieux** selon leur profil (stimulation, bienveillance), jusqu'à une densité **[à calibrer]**.
  - Certains aident, filment, commentent ou propagent la nouvelle ; la police les disperse.
  - Ils complètent la panique de foule (§3.1), qui reste prioritaire dès qu'une menace est perçue.

*Pourquoi : des normes décrites en données suffisent à rendre crédible une scène sociale en réutilisant rôles, recettes et signatures, et les groupes d'occasion sont ce qu'on voit le plus dans une rue.*

**Coût** : environ 5 opérations par participant et par phase d'une pratique ; environ 5 par passant qui perçoit un incident. **Niveaux** : complet ; en simplifié, les phases d'une pratique sont résolues en bloc ; en statistique, seul le résultat compte (souvenirs, relations).

### 13.10 Conversation : état, voix et suites (décidé)

- **État du discours** : pour chaque conversation, le cœur tient les sujets abordés, les questions sans réponse et les propositions en attente. Une question déjà posée est reconnue comme telle.
- **Ce qu'on s'est dit** : chaque relation complète (§13.5) garde les derniers sujets échangés, avec leur date **[nombre à calibrer]**.
  - On ne demande pas deux fois « comment va ta mère ? » dans la journée.
  - On **revient** sur ce qui a été dit (« alors, cet entretien ? »), grâce au rappel (§13.3).
- **Voix propre** : chaque personnage a des paramètres de langage, tirés à la création d'après son âge, son origine, son milieu, son métier et son profil : registre, longueur des phrases, politesse, humour, tics, vocabulaire de métier.
  - Ils sont transmis au service de dialogue avec chaque acte, avec l'humeur (§13.2), pour une voix stable d'une scène à l'autre.
  - Le repli systémique les utilise aussi ([social.md](social.md), §3.6).
- **Anti-répétition** : le service reçoit les dernières formulations du personnage pour cet auditeur et évite de les répéter. C'est cosmétique, côté service.
- **Ton et registre du joueur** (décidé ; [social.md](social.md), §3.3) : le classement du texte libre d'un joueur rend, avec l'acte et le sujet, un **ton** (amical, neutre, sec, menaçant, séducteur) et un **registre** (poli ou familier), que le joueur voit et confirme ; la roue d'actes propose les mêmes choix.
  - Le PNJ évalue le ton comme une partie de l'acte (§13.1) : menacer sur un ton aimable reste une menace ; le registre est jugé d'après la relation et les usages (un tutoiement déplacé agace un inconnu âgé, pas un ami).
  - Ton et registre entrent dans le journal avec l'intention, ce qui garde la reproductibilité.

*Pourquoi : une conversation paraît humaine quand elle se souvient de ce qui a été dit, garde une voix et réagit au ton, et tout cela reste décidé par le cœur sur des faits structurés.*

**Coût** : environ 10 opérations par acte, dans le cœur. **Niveaux** : complet, avec un joueur auditeur ou à portée d'écoute ; entre PNJ hors d'écoute, seuls les sujets échangés sont mis à jour (complet et simplifié).

### 13.11 Résistance aux manipulations (décidé)

- **Lassitude** : la répétition d'un même acte par la même personne (saluer, complimenter, demander, inviter, offrir, insister) a des effets **décroissants** sur la journée et sur la semaine **[à calibrer]**. Ces effets s'ajoutent aux rendements décroissants du social ([social.md](social.md), §1.4). L'acte devient ensuite **agaçant** : pensée négative, refus, « tu me l'as déjà demandé », évitement.
- **Méfiance** : une générosité, une flatterie ou une offre hors des usages, venant de quelqu'un de peu connu, fait naître un **soupçon** (« qu'est-ce qu'il me veut ? », §13.5) plutôt qu'une hausse d'affection.
- **Habituation** : une provocation ou une fausse alerte répétée fait de moins en moins sursauter, puis met en colère ou fait appeler la police. Un appât laissé deux fois au même endroit éveille la méfiance.
- **Obstruction** : un personnage bloqué dans une porte ou un couloir demande à passer, contourne, se plaint, puis appelle quelqu'un ; il ne reste jamais figé.
- **Imprévisibilité légitime** : les paramètres sont publics (le paquet de contenus l'est, [revues/securite.md](../revues/securite.md)). Mais l'état intérieur (humeur, pensées, conclusions, valeurs apprises) et l'aléa à clé ne le sont pas : on ne peut pas prédire exactement une réponse sans connaître la vie du personnage.
- **Mêmes réactions pour tous** : ces protections sont des réactions du monde, identiques quel que soit le pilote de celui qui insiste ([social.md](social.md), §0). Elles s'ajoutent aux garde-fous du méta-jeu (§5.4).
- **Agents adverses** : ceux de la revue d'équilibrage (E40) reçoivent un objectif de plus : maximiser une relation ou obtenir un acte d'un PNJ par répétition, flatterie, appât ou obstruction (§14.2).

*Pourquoi : un humain se lasse, se méfie et finit par réagir, et ces trois réactions ferment la plupart des boucles qu'un joueur cherche dans une IA.*

**Relation compacte** (décidé) : un inconnu ou une personne connue de vue n'a pas de compteur ; mais des actes répétés de la même personne dans la journée sont un vrai contact, qui fait **remonter** la relation en relation complète ([social.md](social.md), §1.2), et les compteurs s'appliquent dès lors. *Pourquoi : la lassitude ne doit pas pouvoir se contourner en restant un inconnu.*

**Coût** : un compteur par relation complète et par type d'acte, 1 à 2 opérations par acte. **Niveaux** : complet et simplifié.

### 13.12 Cohérence entre niveaux de détail (décidé)

- **Pas d'événement sans traces** : un événement résolu en simplifié ou en statistique écrit les mêmes faits que s'il avait été joué en complet : souvenirs des deux parties, pensées, relations, objets déplacés, traces matérielles, rumeurs. Une dispute tirée au sort est une vraie dispute, dont on parle.
- **Constat respecté** : pour chaque personnage, le cœur garde le **dernier constat** fait par un personnage piloté par un joueur (lieu, heure, activité). Une reconstruction (§6.3) ne peut pas le contredire : quelqu'un qu'un joueur a vu partir n'est pas reconstruit chez lui avant un retour plausible.
- **Mêmes lois** : les tables du statistique (événements, réactions, issues) sont **calibrées sur des simulations en complet** par `ts-sim`, comme les affrontements ([combat.md](combat.md), §13). Un test compare leurs distributions (§14.2).

*Pourquoi : un joueur qui revient voit toute incohérence de transition, et on ne la prévient qu'en garantissant que chaque niveau laisse les mêmes traces et suit les mêmes lois.*

**Coût** : 1 opération par observation d'un joueur pour le dernier constat ; les traces sont celles que le complet écrirait de toute façon. **Niveaux** : transitions entre tous les niveaux.

### 13.13 Décisions écrites à la main, poids calibrés hors ligne (décidé)

- **Maintenu : les décisions ne sont pas entraînées.**
  - Un réseau de décisions serait opaque : on ne pourrait plus dire pourquoi un personnage a agi.
  - Il serait difficile à garder fidèle au profil et à la signature qui juge le joueur.
  - Il faudrait le réentraîner à chaque changement de règle.
  - Il serait exploitable de façon imprévisible.
  - Les agents crédibles à grande échelle restent faits de règles et d'utilité.
- **Ajout : calibrage hors ligne des poids.** Les **poids** des décisions (utilité, pensées, humeur, règles de réflexion, seuils) sont des données.
  - Ils sont **calibrés hors ligne** par une optimisation en boîte noire sur `ts-sim` (par exemple CMA-ES ou optimisation bayésienne), orchestrée en Python ([05-architecture.md](../05-architecture.md), §2.23).
  - Les cibles sont les indicateurs (§14.1) et les résultats des tests à l'aveugle (§14.3).
  - Les poids obtenus sont versionnés dans le paquet de contenus, relus, puis testés par les scénarios avant livraison.
- **Les signatures d'abord** (décidé ; G19, G20) : les paramètres des signatures (≈ 220 : effets, justifications, tolérance et poids, planchers) sont calibrés **avant** les poids des décisions, par **comparaisons** d'un **panel de sessions évaluatrices neuves**, le panel de calibrage, disjoint du panel d'acceptation (§14.3) (décidé ; [05-architecture.md](../05-architecture.md), arbitrage A8 ; « lequel est le plus malhonnête ? », modèles de Bradley-Terry et logistique ordinal sur la fonction `appraise` elle-même, ancres du concepteur et planchers comme contraintes dures ; [revues/signatures.md](../revues/signatures.md), §4.5). Ils restent **fixes** pendant la boucle comportementale ; un indicateur inatteignable sans toucher aux signatures est **arbitré par le responsable conception** (décidé ; A8 ; *pourquoi : aucune décision n'attend une action de l'utilisateur*). Les actions des joueurs **détectent** les faiblesses des signatures, elles ne servent jamais à les ajuster (G21). *Pourquoi : ce que le panel a jugé ne doit pas être déformé pour atteindre un indicateur de simulation.*

*Pourquoi : on garde une structure explicable écrite à la main, et l'on confie à la machine ce qu'elle fait mieux qu'un humain, régler des dizaines de poids ensemble ; rien n'est appris en jeu, donc la reproductibilité est intacte.*

**Coût** : nul en jeu ; hors ligne, des milliers de simulations `ts-sim`. **Niveaux** : tous.

### 13.14 Place des modèles de langage (décidé)

- **En jeu, aucune décision** : le texte reste cosmétique ([05-architecture.md](../05-architecture.md), §2.9). Un modèle de langage dans la boucle de décision aurait quatre défauts :
  - il serait non reproductible ;
  - il serait trop coûteux pour 10 000 habitants : un agent génératif consomme des milliers de jetons par heure simulée ;
  - il serait exposé aux injections des joueurs ;
  - rien ne garantirait sa fidélité au profil.
- **Trois usages permis**, dont aucun ne décide en jeu :
  1. **Hors ligne, pour écrire des contenus** : variantes de recettes, catalogue de pensées, règles de réflexion candidates, pratiques sociales, paramètres de voix, gabarits du repli systémique, et, pour les signatures, annotations d'actes proposées et recoupées, vignettes d'annotation et gabarits d'explication, **jamais de valeurs numériques** (décidé ; G18). Tout est relu, testé par `ts-sim` et versionné dans le paquet de contenus : en jeu, ce ne sont que des données.
  2. **En jeu, comme entrée journalisée** : le classement du texte libre d'un joueur en acte, sujet, ton et registre (§13.10), confirmé par le joueur. Le résultat entre dans le journal comme une intention : la relecture n'appelle jamais le modèle.
  3. **En jeu, cosmétique et mis en cache** : la formulation des répliques, et, pour les personnages importants, une **biographie formulée** à partir de leurs souvenirs et conclusions. Elle est mise en cache, renouvelée rarement, et ne sert qu'à donner du contexte à la formulation.

*Pourquoi : le modèle de langage apporte le plus là où il ne décide pas (écrire, comprendre le joueur, formuler), et chaque usage retenu garde la simulation reproductible et son coût borné.*

**Coût** : nul pour le cœur. **Niveaux** : sans objet.

### 13.15 Récapitulatif des coûts **[à calibrer]**

| Mécanisme | Complet et simplifié | Statistique (par personnage et par jour) |
|---|---|---|
| Évaluation des événements (§13.1) | 10 à 30 opérations par événement | ≈ 40 |
| Pensées et humeur (§13.2) | ≈ 5 par pensée, ≈ 20 par recalcul | ≈ 60 |
| Rappel (§13.3) | ≤ 40 par rappel | 0 |
| Réflexion (§13.4) | ≤ 200 par jour | ≤ 200 |
| Modèle d'autrui (§13.5) | 3 à 10 par acte ou propagation | ≈ 30 |
| Apprentissage (§13.6) | 1 à 3 par résultat | ≈ 15 |
| Variété et occasions (§13.7) | ≈ 10 par choix ; naturel ≈ 3 par seconde (complet) | ≈ 50 |
| Attentes et anomalies (§13.8) | ≈ 5 par entité perçue (complet) | ≈ 5 |
| Pratiques et attroupements (§13.9) | ≈ 5 par participant et par phase | ≈ 10 |
| Conversation (§13.10) | ≈ 10 par acte | 0 |
| Manipulations (§13.11) | 1 à 2 par acte | 0 |
| Cohérence (§13.12) | 1 par observation d'un joueur | ≈ 0 |
| *Pour mémoire : tenue, coiffeur et musique (§2.6)* | *≈ 10 par changement de tenue* | *≈ 2, compté dans le budget de la décision* |
| *Pour mémoire : signatures (`appraise`, §1.4)* | *≈ 40 à 80 par acte évalué, avec cache par personnage* | *≈ 40, compté dans le budget de la décision et non dans les 500* |
| **Total** | | **≈ 410, budget 500** |

Pour un an et 10 000 habitants, 500 opérations par jour et par personnage font environ 1,8 milliard d'opérations : à mesurer contre l'objectif de 5 minutes, avec le reste de la simulation (§14.1).

---

## 14. Mesurer l'intelligence et le réalisme

### 14.1 Indicateurs (décidé)

*Pourquoi : un comportement crédible se mesure par des distributions et des corrélations sur des milliers de vies, et ces indicateurs alimentent aussi le calibrage hors ligne (§13.13).*

Les indicateurs sont définis une seule fois dans `ts_metrics` ([05-architecture.md](../05-architecture.md), §2.20). Cibles **[à calibrer]**.

| Critère | Indicateur | Cible |
|---|---|---|
| Cohérence | Revirements inexpliqués : actes contraires à une conclusion forte sans cause nouvelle | Proche de 0 |
| Cohérence | Durée des rancunes selon le profil ; part des promesses tenues selon l'honnêteté | Corrélations dans le sens attendu |
| Variété | Entropie des journées d'un personnage (lieux × heures) sur 30 jours | Dans une bande : ni robot (trop basse), ni hasard (trop haute) |
| Variété | Écart entre personnages d'un même rôle | Au-dessus d'un seuil |
| Lisibilité du caractère | Corrélation entre chaque axe du profil et la fréquence des **effets** correspondants (les honnêtes trompent moins) (G1). **Grille** (décidé ; [revues/signatures-contenu.md](../revues/signatures-contenu.md), §6) : les **12 archétypes** (A1 bon samaritain à A12 neutre) dans les trois contextes de référence (ordinaire, menacé, dans le besoin) ; chaque archétype est comparé aux autres sur les effets qu'il produit, en simulation et dans le rapport des coûts | Significative sur chaque axe |
| Signatures (décidé ; [revues/signatures.md](../revues/signatures.md), §4.5 et §4.6) | Accord du coût calculé avec les jugements du panel de sessions évaluatrices (A8 ; corrélation de rang de Kendall) ; aire sous la courbe ROC du test « fidèle ou non » (§14.3) ; part des actes de l'IA dont le coût dépasse son seuil hors désespoir ; part des choix sous désespoir par profil ; part de la perte de synchronisation due au tempérament | Kendall ≥ 0,8 ; AUC ≥ 0,85 ; ≈ 0 ; plausible ; petite |
| Raisonnement social | Part des croyances vraies ; part des mensonges découverts ; cohérence des versions d'un menteur selon sa mémoire | Dans les bandes de calibrage |
| Émotions | Distribution de l'humeur ; fréquence des crises selon la stabilité ; durée de récupération après un deuil | Proches des références publiques |
| Imprévu | Part des anomalies injectées qui reçoivent une réaction, et délai, selon la gravité | Plus de 90 % pour les anomalies graves |
| Imprévu | Personnages sans plan plus de N minutes ; même action échouée répétée | ≈ 0 |
| Apprentissage | Fréquentation d'un lieu avant et après une agression ; fidélité à un commerce après une arnaque | Baisse nette, puis retour lent |
| Groupes et foules | Densité et durée des attroupements ; temps d'évacuation | Proches des références |
| Niveaux de détail | Écart entre distributions du complet et du statistique (test de Kolmogorov-Smirnov) ; contradictions d'un dernier constat | Non significatif ; 0 |
| Manipulations | Gain de relation ou d'actes obtenus par heure de répétition | Plafonné |
| Paraître humain | Distribution des délais de réaction des PNJ comparée à la distribution de référence (§10), puis à celle des joueurs d'une bêta s'il y en a ; score des tests à l'aveugle par profil d'adversaire et AUC du discriminateur (§14.3) | Indiscernables ; au plus 60 % de bonnes réponses |
| Coût | Opérations par personnage et par jour, par niveau ; durée d'un an simulé | Au plus le budget ; moins de 5 minutes |

### 14.2 Scénarios `ts-sim` (décidé)

*Pourquoi : chaque comportement attendu devient un test reproductible par graine, qui empêche son retour en arrière, comme le catalogue d'exploits de la revue d'équilibrage.*

Scénarios déclaratifs en RON, joués sur 100 graines et plusieurs profils, puis ajoutés à l'intégration continue avec des bandes de tolérance.

| Scénario | Ce qu'il vérifie |
|---|---|
| `ia_annee_ville` : 10 000 habitants pendant un an, en statistique, avec des bulles complètes tirées au hasard | Coût, stabilité, indicateurs de cohérence et de variété |
| `ia_humiliation` : A humilie B en public ; B est suivi pendant 90 jours | Rappel, rancune, évitement, vengeance ou pardon selon le profil |
| `ia_intrus` : inconnu dans un logement, porte forcée, objet disparu, proche qui ne rentre pas | Réactions aux anomalies et délais |
| `ia_parc_agression`, `ia_arnaque` | Aversion apprise, puis retour lent |
| `ia_deux_versions` : A ment à B et à C avec deux versions, puis B et C se parlent | Soupçon, découverte du mensonge |
| `ia_temoin_crime` : A commet un délit vu par B, et sait que B l'a vu | Faits sensibles : peur, pression, aveu selon le profil |
| `ia_deuil` | Humeur, crise, récupération, thème de vie |
| `ia_fete_enterrement` | Pratiques sociales et normes |
| `ia_attroupement_incendie` | Attroupement, panique, évacuation |
| `ia_aller_retour_lod` : mêmes personnages alternés entre niveaux ; même semaine jouée en complet et en statistique | Traces, dernier constat, distributions |
| Scénarios `SIG_*` (décidé ; [revues/signatures.md](../revues/signatures.md), §4.8) : légitime défense provoquée, ivresse puis violence, ordre du complice, faim fabriquée, liasses multiples, escalade, témoin partiel, sondage du profil | Imputabilité, épisode et planchers de rupture, monotonie, jugement des témoins, résistance au sondage par la réticence ; avec les invariants `INV-SIG-*` |
| `ia_manipulation` : agents adverses (E40) qui répètent saluts, cadeaux, questions, provocations, appâts, obstructions | Lassitude, méfiance, habituation ; aucun gain illimité |
| `ia_variete_role` : 100 personnages d'un même rôle pendant 30 jours | Entropie, habitudes, différences entre personnages |

### 14.3 Tests à l'aveugle « joueur ou PNJ » (décidé)

*Pourquoi : la promesse qu'on ne distingue pas les pilotes ne se vérifie qu'en demandant à des juges qui ne savent rien du cœur de les distinguer.*

**Juges et joueurs** (décidé ; [05-architecture.md](../05-architecture.md), arbitrage A8) : aucun de ces tests ne demande d'action à l'utilisateur.
- Les **juges** sont un **panel de sessions évaluatrices neuves**, sans accès au code, aux paramètres ni aux jugements précédents, avec des **consignes de juge variées** (joueur aguerri, novice, observateur attentif au corps, attentif au langage, sceptique) pour imiter un panel divers. L'accord et la cohérence des juges se mesurent comme pour des annotateurs humains.
- Le côté « joueur » est tenu par des **sessions joueuses** qui pilotent des **comptes synthétiques** avec l'interface d'un joueur (mêmes intentions, mêmes délais, même vue filtrée), en jouant sincèrement leur personnage.
- **Règles du panel** (décidé ; revue d'équilibrage de clôture, C7) :
  - deux panels **disjoints**, l'un pour le **calibrage**, l'autre pour l'**acceptation**, qui se fait sur le panel tenu à l'écart ;
  - un juge n'est admis que s'il place correctement au moins **90 %** des **ancres** ([revues/signatures-contenu.md](../revues/signatures-contenu.md), §7) ;
  - l'accord se mesure seulement entre juges de **consignes** et, si possible, de **modèles** différents, avec un **test-retest** où l'ordre des paires est inversé (au moins 90 %) ;
  - ce que reçoivent les juges vient d'une **relecture du journal**, puis de la **vue de l'observateur** par `ts_view` ; le journal, qui contient les commandes de pilote, n'est jamais lu directement par les juges (décidé ; T2-3). Toute parole libre leur arrive comme une **citation filtrée**.
  - *Pourquoi : des paramètres optimisés contre les juges qui les valident apprennent leurs biais, et des copies d'un même modèle s'accordent par construction.*
- **Adversaires** : les sessions joueuses jouent les **six profils d'adversaires** de la revue d'équilibrage (optimiseur, pressé, inactif, fauteur de troubles, rôliste, complice), et la cible de 60 % vaut **pour chaque profil**, pas en moyenne. *Pourquoi : une session joueuse joue posément, comme un modèle ; sans ces profils, le test mesurerait « modèle ou PNJ ».*
- **Surface des sessions joueuses** (décidé ; revue d'architecture de clôture, C-6 ; [05-architecture.md](../05-architecture.md), §2.13) :
  - elles jouent dans `ts-sim play` ou `ts_py`, avec un **pilote `Joueur`** attaché à un compte synthétique de `ts_meta`, jamais `Externe`, pour exercer la garde hors fiction, la synchronisation et le bilan ;
  - elles reçoivent exactement les messages de la vue (entités perçues, actions possibles, niveaux de réticence, codes de refus), rendus en **texte** ;
  - elles répondent par des intentions prises dans la liste des actions possibles ; le temps est **en pause** pendant qu'elles décident, et chaque intention porte une condition d'arrêt (fin de l'action, perception nouvelle, durée maximale) ;
  - leurs choix entrent au journal des entrées, et la partie est rejouable au bit près.
- **Surface des juges** : une **transcription de la vue d'un personnage observateur**, produite par `ts-sim report --observer` à partir de `ts_view` seul. Elle ne peut contenir ni le pilote, type privé, ni aucun composant interne, paramètre ou identifiant durable. Des images du client pourront s'y ajouter à partir de la phase 3.
- **Portée** : ces tests portent sur les **décisions et les conduites**, sur quelques minutes à quelques jours. Les **temps de réaction** en sont exclus, car une session joueuse n'a pas un rythme humain. Ils sont couverts par construction, avec la même distribution de délais pour tous (§13.7), et par la télémétrie d'une bêta s'il y en a une.
- Des testeurs humains d'une bêta pourront compléter le panel, sans jamais être requis.
- *Pourquoi : c'est gratuit, reproductible et sans action de l'utilisateur ; les cibles chiffrées restent les mêmes.*

- **Principe** : dans un monde de test, les juges évaluent des personnages. La moitié sont incarnés par des sessions joueuses qui jouent sincèrement (synchronisées), la moitié sont pilotés par l'IA, et les juges ne savent pas lesquels.
- **Trois formes** :
  1. **Observation** : regarder un personnage vivre 10 minutes, en direct ou en relecture du journal, pilotes masqués ;
  2. **Interaction** : 5 à 10 minutes de conversation, d'échange ou d'affaire avec lui ;
  3. **Vie longue** : suivre une semaine de jeu d'un personnage (ses actes et ce qu'il dit), et noter sa vraisemblance de 1 à 5, contre de vraies semaines de joueurs.
- **Mesure** : part de bonnes réponses, avec son intervalle de confiance ; **cible au plus 60 %** **[à calibrer]** (50 % est le hasard), pour chacun des six profils d'adversaires ; notes de vraisemblance égales entre PNJ et joueurs.
- **Catalogue des indices** : il vient des **juges** et du **discriminateur** (ci-dessous). Chaque juge dit ce qui l'a décidé. Les indices récurrents (« il répond trop vite », « il ne regarde jamais son téléphone », « il refait le même trajet ») entrent dans un **catalogue des indices**, sur le modèle du catalogue d'exploits. Chacun devient un indicateur ou un scénario, puis il est corrigé et surveillé.
- **Discriminateur hors ligne** : entre deux campagnes, un classifieur hors ligne apprend à distinguer les pilotes sur les journaux de test. S'il y parvient, ses variables les plus discriminantes désignent les indices à corriger. Cible : **AUC ≤ 0,70** **[à calibrer]** sur les mêmes traces que les juges (décidé ; C7). En production, il est réentraîné chaque **mois** sur les traces étiquetées par le serveur, qui connaît le pilote, et son AUC est suivie. Il ne tourne jamais en jeu.
- **Test « fidèle ou non »** (décidé ; [revues/signatures.md](../revues/signatures.md), §4.6) : les juges du panel lisent le portrait d'un personnage, regardent des relectures et disent si chaque acte était « dans son caractère » ; on mesure l'aire sous la courbe ROC du coût calculé face à leur jugement (cible inchangée, ≥ 0,85), et l'on recalibre l'échelle de réticence sur les degrés perçus.
- **Quand** : à chaque version majeure de l'IA, et avant l'ouverture du jeu en ligne.

