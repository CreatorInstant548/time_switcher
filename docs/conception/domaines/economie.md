# Domaine — économie

> Fichier de **domaine** : modifié uniquement par la session « Conception — économie » (voir [README](../README.md)).
> Version 0.1 — 29/10/2026 : première rédaction complète.
> Version 0.2 — 29/10/2026 : intégration des demandes de [population.md](population.md) (§12.4) et de [social.md](social.md) (§12.4) après la relecture de cohérence ; impacts sur le socle marqués intégrés.
> Version 0.3 — 29/10/2026 : intégration des demandes de [combat.md](combat.md) (§16.5 : mécanisme des sanctions au §7.8, police et justice, assurances, sécurité municipale) et d'[interface.md](interface.md) (§15.4 : prix affichés, fiabilité en mots, prêts en agence, plan papier, journal local).
> Version 0.4 — 29/10/2026 : intégration des revues d'[équilibrage](../revues/equilibrage.md) (§6.6 : E5, E6, E7, E8, E10, E11, E24, E31, E33) et de [sécurité](../revues/securite.md) (§9.12 : S15, S23, S30), avec les arbitrages de la session principale (cercle du joueur, barème du travail).
> Version 0.5 — 29/10/2026 : intégration de la revue de l'intelligence des PNJ ([ia-pnj.md](ia-pnj.md) v0.8, §11 et §13.6) : les valeurs apprises sur les commerces alimentent le choix des commerces et la fidélité (§4.4).
> Version 0.6 — 29/10/2026 : intégration des revues des [signatures](../revues/signatures.md) (§9.9 : signature dérivée du grand livre, don reconnu par son effet, cumul de 30 jours, écritures déclarées par les recettes) et de la [génération](../revues/generation.md) (§13.6 : aires de chalandise, taux d'effort, initialisation à l'équilibre) ; l'initialisation est numérotée §9.5 bis en v0.7 (le solo accéléré garde son §9.6).
> Version 0.7 — 29/10/2026 : intégration des demandes de [direction-artistique.md](direction-artistique.md) (§12.8 : garde-robe agrégée, véhicules sans marque), de [revues/signatures-contenu.md](../revues/signatures-contenu.md) (§11.4 : M3, M10, D13, D15, D16, A15) et d'[ia-pnj.md](ia-pnj.md) v1.1 (§2.6 : vêtements, coiffeur, laverie, musiciens, tâche « mettre de la musique ») ; application de la règle 6 du README (aucune renumérotation) : le §9.6 « Solo accéléré » retrouve son numéro, l'initialisation à l'équilibre devient le §9.5 bis. Application de l'arbitrage A8 (aucune action de l'utilisateur ni d'un humain) : la **session de modération** remplace le modérateur humain (§12.3, §12.4, §12.5) ; les annulations du grand livre sont exécutées par la session d'exploitation (§2.3, §12.4) ; le tableau de bord est lu par les sessions (§10.3).
> Version 0.8 — 30/10/2026 : intégration des corrections des revues de clôture (équilibrage C1, C2, C3, C4, C8 ; architecture S-7) : espèces et plafond de 20 000 écus vers le cercle associé, montant libre borné (§3.8, §12.4) ; perdre volontairement au jeu et paris extérieurs (§7.6) ; règles de la garde-robe (§5.13) ; A15 bis (§5.10) ; rapprochements sur actes réciproques et indices techniques, modération selon le socle (§12.3, §12.4) ; §14.11 de clôture.
> Version 0.8.1 — 30/10/2026 : arbitrage A11 : le rapprochement de comptes est notifié au joueur (motifs, règle, moyens automatisés, recours), sans nommer l'autre compte ni l'origine du dossier (§12.4).
> Version 0.8.2 — 30/10/2026 : correction R2 de l'équilibrage : l'écart d'espérance de la perte volontaire au jeu n'additionne que les décisions moins bonnes que celles de l'IA, sans déduire les meilleures (§7.6).
> Tous les choix de ce fichier sont pris par la session de domaine, sur consigne de l'utilisateur, et marqués **(décidé)** avec leur raison. Les valeurs marquées **[à calibrer]** seront ajustées par le prototypage. Ce qui touche le socle ou un autre domaine est listé au §14 et n'est appliqué nulle part ailleurs.

Ce domaine décrit l'argent du monde : la monnaie et les prix, les métiers, les entreprises, la propriété, le marché, l'économie parallèle, l'État et les communes, la façon dont tout cela est simulé aux trois niveaux de détail et reste stable dans un monde persistant, et enfin l'économie du personnage persistant et les garde-fous contre les abus des joueurs.

Il s'appuie sur le socle ([01-concept.md](../01-concept.md) à [05-architecture.md](../05-architecture.md)), sur [ville.md](ville.md) (lieux, bâtiments, quartiers, transports, extérieur) et sur [ia-pnj.md](ia-pnj.md) (fiches de rôle, agenda, décisions, niveaux de détail).

## Sommaire

0. [Principes](#0-principes)
1. [Monnaie et ordres de grandeur](#1-monnaie-et-ordres-de-grandeur)
2. [Comptes et circulation de l'argent](#2-comptes-et-circulation-de-largent)
3. [Métiers et travail](#3-métiers-et-travail)
4. [Entreprises et commerces](#4-entreprises-et-commerces)
5. [Propriété](#5-propriété)
6. [Marché et prix](#6-marché-et-prix)
7. [Économie parallèle](#7-économie-parallèle)
8. [État et collectivités](#8-état-et-collectivités)
9. [Simulation à plusieurs niveaux](#9-simulation-à-plusieurs-niveaux)
10. [Stabilité sur la durée](#10-stabilité-sur-la-durée)
11. [Le personnage persistant](#11-le-personnage-persistant)
12. [Garde-fous contre les abus](#12-garde-fous-contre-les-abus)
13. [Points ouverts](#13-points-ouverts)
14. [Impacts sur le socle et les autres domaines](#14-impacts-sur-le-socle-et-les-autres-domaines)

---

## 0. Principes

1. **Aucun argent ne naît de rien** (décidé). Chaque opération est une **écriture entre deux comptes** : ce que l'un paie, l'autre le reçoit. L'argent n'entre dans l'agglomération et n'en sort que par des comptes de l'**extérieur** (§2.2). *Pourquoi : c'est la seule façon de garantir qu'une économie persistante ne dérive pas en silence, et cela rend chaque fortune explicable.*
2. **Une petite économie ouverte, ancrée sur l'extérieur** (décidé). Orvenne importe presque tout ce qu'elle consomme et vend à l'extérieur une partie de ce qu'elle produit. Les prix locaux s'écartent d'un **prix de référence** extérieur dans une marge bornée (§6). *Pourquoi : c'est la réalité d'une ville de cette taille, et l'ancre empêche l'inflation ou l'effondrement des prix provoqués par quelques joueurs.*
3. **Pas de clientèle fictive** (décidé). Les recettes d'un commerce viennent des achats réels des habitants, quel que soit leur niveau de détail. Seuls quelques établissements « à rayonnement extérieur » reçoivent en plus une demande de l'extérieur, invisible (§4.4). *Pourquoi : c'est la traduction économique de la règle « aucun figurant » ; un commerce vide doit vraiment perdre de l'argent.*
4. **Mêmes règles pour tous** (décidé). PNJ, personnages incarnés et personnages persistants paient les mêmes prix, les mêmes impôts et obéissent aux mêmes contrats. Les seules règles propres aux joueurs sont les **garde-fous** du §12, qui empêchent de détourner l'incarnation. *Pourquoi : une économie à deux vitesses se verrait et casserait l'illusion du monde.*
5. **L'argent n'est jamais une source d'expérience** (décidé). On ne gagne pas d'expérience en s'enrichissant, seulement en accomplissant les objectifs du personnage ([04-joueur.md](../04-joueur.md), §1.1). *Pourquoi : sinon l'enrichissement deviendrait le but du jeu et l'incentive à abuser serait maximal.*
6. **Les comptes ne dépendent pas du niveau de détail** (décidé). Un salaire, un loyer ou un achat existent de la même façon en simulation complète, simplifiée ou statistique ; seule la finesse des transactions change (§9). *Pourquoi : c'est la condition pour qu'avancer le temps en solo ou s'éloigner d'un quartier ne crée ni ne détruise d'argent.*
7. **L'économie est une donnée** (décidé). Métiers, catégories de marchandises, prix de référence, barèmes d'impôts et d'aides sont décrits dans des **tables de données**, comme les fiches de rôle de l'IA ([ia-pnj.md](ia-pnj.md), §0). *Pourquoi : on calibre et on enrichit sans toucher au code, et on peut lancer de nombreuses simulations avec des réglages différents (contrainte de la revue d'architecture).*

---

## 1. Monnaie et ordres de grandeur

### 1.1 L'écu (décidé)

La monnaie du monde est l'**écu**, divisé en 100 **centimes**. On écrit « 12,50 écus ». Son pouvoir d'achat est **proche de celui de l'euro au milieu des années 2020**.

*Pourquoi : un nom ancien et francophone, sans être une monnaie réelle en circulation ; un pouvoir d'achat proche de l'euro donne aux joueurs européens des repères immédiats (« une baguette à 1,20 », « un studio à 420 »).*

Le monde n'a qu'une monnaie. Les transactions avec l'extérieur se font aussi en écus.

### 1.2 Deux formes d'argent (décidé)

| Forme | Où | Propriétés |
|---|---|---|
| **Compte bancaire** | Dans une banque d'Orvenne (§4.10) | Paiement par carte ou virement, prélèvements automatiques, **traçable** (chaque opération est enregistrée avec son émetteur et son destinataire) |
| **Espèces** | Objet physique : portefeuille, caisse d'un commerce, coffre, cache | Anonymes, **peuvent être volées ou perdues**, retirées et déposées au guichet ou au distributeur |

*Pourquoi : les espèces sont le support naturel de l'économie parallèle, du vol et du pourboire ; le compte rend les flux légaux traçables pour la police et les impôts.*

- Les espèces sont un objet unique « liasse » par contenant (une quantité), pas des billets individuels.
- Un commerce dépose sa caisse à la banque en fin de journée ou de semaine selon son gérant : la caisse pleine du soir est une cible.
- La plupart des PNJ paient par carte ; les plus âgés, les plus modestes et ceux qui vivent de l'économie parallèle utilisent davantage les espèces **[à calibrer]**.

### 1.3 Patrimoine médian et échelle de richesse (décidé)

La composante **Richesse** de l'influence se mesure en multiples du **patrimoine médian** du monde ([02-personnages.md](../02-personnages.md), §3.2). Ce domaine en précise le calcul :

- **Patrimoine net** d'un personnage = comptes + espèces + valeur de ses biens (logements, locaux, parcelles, véhicules, objets de valeur, parts d'entreprise) − dettes. Un bien commun d'un couple compte pour moitié à chacun.
- La **médiane** est calculée sur les **adultes habitants permanents** : ni les enfants (patrimoine presque nul, ils feraient tomber la médiane), ni les personnages persistants (exclus du classement, [ville.md](ville.md), §2.5).
- Un patrimoine négatif donne une richesse de 0.
- Le calcul est refait **une fois par mois de jeu**, et immédiatement pour un personnage dont le patrimoine change de plus de moitié (héritage, faillite, gros vol) **[à calibrer]**.

*Pourquoi : la médiane des adultes est la référence la plus parlante et la plus stable ; l'exclusion des persistants suit la règle de la ville.*

Valeur de départ : **patrimoine médian ≈ 50 000 écus** **[à calibrer]**, cohérent avec une agglomération où environ la moitié des ménages sont propriétaires et où les locataires du grand ensemble et de la commune populaire ont peu d'épargne.

| Richesse (socle) | Multiple de la médiane | Patrimoine | Exemple à Orvenne |
|---|---|---|---|
| 0 à 30 | ≤ 0,1 × | ≤ 5 000 | Locataire modeste, étudiant, endetté |
| 50 | 1 × | ≈ 50 000 | Employé propriétaire d'une petite voiture et d'une épargne, ou jeune propriétaire endetté |
| 70 | 10 × | ≈ 500 000 | Médecin propriétaire de sa maison aux Tilleuls, patron de PME |
| 80 | ≈ 100 × | ≈ 5 millions | Grande famille de Hautevigne, entrepreneur régional |
| 90 | 1 000 × | ≈ 50 millions | Propriétaire de l'usine principale (hors catégorie) |

### 1.4 Ordres de grandeur **[à calibrer]**

Les montants sont **mensuels et nets** (après cotisations sociales, avant impôt sur le revenu, §8.2), pour un temps plein, sauf mention contraire.

**Salaires**

| Emploi | Salaire net mensuel |
|---|---|
| Salaire minimum (35 h par semaine) | 1 450 (≈ 9,55 de l'heure) |
| Caissier, livreur salarié, agent d'entretien, serveur | 1 450 à 1 650 |
| Ouvrier qualifié, employé de bureau, chauffeur de bus | 1 700 à 2 100 |
| Artisan salarié, technicien, aide-soignant | 1 800 à 2 400 |
| Infirmier, enseignant, policier, agent de maîtrise | 2 100 à 2 900 |
| Cadre, ingénieur | 3 000 à 4 800 |
| Médecin généraliste (revenu d'activité) | 5 000 à 8 000 |
| Avocat, patron de PME (revenu tiré de l'activité) | 3 000 à 12 000, très variable |
| Directeur de l'usine, dirigeant régional | 8 000 à 15 000 |

Salaire médian : **≈ 1 950**. Coût pour l'employeur : **≈ 1,6 × le net** en moyenne, 1,35 × au salaire minimum (§8.2).

**Logement**

| Bien | Loyer mensuel (hors charges) | Prix d'achat |
|---|---|---|
| Chambre au foyer de travailleurs | 15 la nuit, 330 le mois | — |
| Chambre d'hôtel (quartier de la Gare) | 70 la nuit | — |
| Studio meublé | 420 à 520 | 50 000 à 80 000 |
| T2 | 480 à 650 | 80 000 à 140 000 |
| T3 (logement social aux Peupliers) | 380 à 450 | — |
| T3 (privé) | 620 à 850 | 120 000 à 220 000 |
| Maison de ville, pavillon des Forges | 800 à 1 100 | 160 000 à 260 000 |
| Villa de Hautevigne | 1 800 à 4 000 | 500 000 à 2 millions |

Prix au mètre carré, selon le quartier : de ≈ 1 600 (Peupliers, Forges) à ≈ 3 500 (Tilleuls, Cité) et ≈ 4 500 (Hautevigne) ; loyers de 7 à 14 le m² **[à calibrer]**.

**Prix courants**

| Article ou service | Prix |
|---|---|
| Baguette | 1,20 |
| Café au comptoir | 1,80 |
| Demi de bière en terrasse | 3,50 |
| Sandwich | 5,50 |
| Menu du midi au restaurant | 15 |
| Repas gastronomique (Hautevigne) | 80 |
| Paquet de cigarettes | 12 |
| Journal local (numéro / abonnement mensuel) | 1,50 / 22 |
| Plan papier de l'agglomération (tabac-presse) | 4 |
| Courses hebdomadaires pour deux personnes | 100 à 130 |
| Ticket de bus / carnet de 10 / abonnement mensuel | 1,60 / 14 / 40 (moitié prix pour les moins de 26 ans et les plus de 64 ans) |
| Navette fluviale | 2 |
| Billet de train vers l'extérieur | 12 à 40 selon la destination |
| Taxi | 4 + 1,90 par km |
| Litre de carburant | 1,85 |
| Coupe chez le coiffeur (homme / femme) | 20 / 35 |
| Laverie : lavage / séchage | 4 / 2 |
| Vêtements, par pièce (entrée de gamme / moyenne / haut de gamme) : t-shirt, pantalon, manteau, chaussures | 8 / 20 / 60 ; 20 / 45 / 130 ; 50 / 130 / 400 ; 30 / 70 / 220 (§5.13) |
| Cachet d'un musicien pour une soirée au bar | 80 à 150 (§3.8) |
| Entrée de cinéma / piscine | 10 / 4 |
| Électricité et chauffage d'un T3 | 90 l'été, 200 l'hiver |
| Téléphone et internet | 35 |
| Assurance habitation / auto | 15 / 50 |
| Vélo / scooter / voiture d'occasion / voiture neuve | 300 / 2 500 / 7 000 / 25 000 |
| Fusil de chasse à l'armurerie (permis requis) | 800 |

### 1.5 Budget type d'un ménage (décidé)

Le budget d'un ménage est la référence qui sert à l'IA (motivation **argent**, [ia-pnj.md](ia-pnj.md), §1.2) et à la simulation statistique (§9.3). Exemple : couple d'employés aux Forges, un enfant, 3 400 de revenus nets **[à calibrer]** :

| Poste | Montant mensuel |
|---|---|
| Loyer (T3 privé) | 700 |
| Énergie, téléphone, assurances | 220 |
| Alimentation | 550 |
| Transports (une voiture, carburant, assurance) | 280 |
| Impôt sur le revenu | 120 |
| Garde et cantine | 150 |
| Habillement, coiffeur, laverie (v0.7, §5.13) | 120 |
| Santé, divers | 180 |
| Loisirs, sorties | 250 |
| Épargne ou remboursement de dette | 830 |

*Pourquoi : un budget explicite, découpé en postes, est la base commune des décisions d'achat de l'IA, des agrégats statistiques et du calibrage.*

### 1.6 Montants de départ par classe (décidé)

La population attribue à chaque ménage une **classe de revenu** et une **classe de patrimoine** ([population.md](population.md), §10.1) ; ce domaine en fixe les montants. Les classes sont des **quintiles** de la population de départ, plus une classe « aisée » pour le haut de l'échelle **[toutes valeurs à calibrer]** :

| Classe | Revenu net mensuel du ménage (par unité de consommation*) | Patrimoine net par adulte | Ménages typiques |
|---|---|---|---|
| **R1 / P1** (20 % les plus modestes) | < 1 000 | < 3 000, souvent des dettes de consommation | Chômeurs, revenu minimum, étudiants, sans-abri, foyer, logement social |
| **R2 / P2** | 1 000 à 1 450 | 3 000 à 20 000 | Employés au salaire minimum, retraités modestes, familles monoparentales |
| **R3 / P3** (médiane) | 1 450 à 1 950 | 20 000 à 90 000 (médiane ≈ 50 000) | Ouvriers qualifiés, employés, jeunes propriétaires endettés |
| **R4 / P4** | 1 950 à 2 800 | 90 000 à 250 000 | Techniciens, enseignants, infirmiers, artisans, propriétaires de leur logement |
| **R5 / P5** | 2 800 à 6 000 | 250 000 à 1 million | Cadres, professions libérales, commerçants prospères |
| **Aisée** (≈ 1 % des ménages) | > 6 000 | > 1 million, jusqu'à ≈ 50 millions | Grandes familles de Hautevigne, dirigeants, propriétaire de l'usine |

\* Unité de consommation : 1 pour le premier adulte, 0,5 pour chaque autre personne de 14 ans ou plus, 0,3 par enfant plus jeune.

- Le **revenu** découle d'abord du métier et de l'ancienneté (§1.4), des pensions et des prestations ; la classe sert à tirer ce qui ne dépend pas du métier (revenus du patrimoine, aide familiale).
- Le **patrimoine** est réparti en logement (s'il est propriétaire, avec le prêt restant dû selon l'ancienneté d'achat), véhicules, épargne et parts d'entreprise, selon la classe et l'âge.
- Revenu et patrimoine sont **corrélés mais pas confondus** : un retraité modeste propriétaire de sa maison des Forges est R2 / P4 ; un jeune cadre locataire est R5 / P2.
- Contrôle à la génération : le patrimoine médian des adultes doit tomber à ≈ 50 000 (§1.3), et le budget de chaque ménage (§1.5) doit être soutenable (loyer ou mensualité ≤ 40 % du revenu, sauf quelques ménages en difficulté volontairement semés).

*Pourquoi : des quintiles se calibrent directement sur des statistiques réelles de revenus et de patrimoines, et le découplage entre les deux classes donne la diversité des situations réelles.*

---

## 2. Comptes et circulation de l'argent

### 2.1 Les titulaires de comptes (décidé)

Un **compte** appartient à un titulaire :

| Titulaire | Exemples |
|---|---|
| **Personnage** | Compte courant, livret d'épargne, espèces qu'il porte ou qu'il cache |
| **Ménage** | Compte joint éventuel ; les dépenses communes y sont prélevées |
| **Entreprise** | Trésorerie, caisse en espèces, dettes et créances (§4) |
| **Groupe** | Caisse d'une association, d'un club, d'une bande ; gérée par son trésorier ou son chef ([social.md](social.md), §6.1) |
| **Collectivité** | Budget de chaque commune, d'Orvenne Habitat (§8.1) |
| **Banque** | Réserves, prêts accordés, dépôts reçus |
| **Extérieur** | Comptes abstraits du §2.2 |

### 2.2 L'extérieur (décidé)

L'extérieur est représenté par **quelques comptes abstraits**, sans personnage :

| Compte extérieur | Ce qu'il reçoit | Ce qu'il verse |
|---|---|---|
| **Fournisseurs** | Achats de marchandises importées, énergie, carburant | — |
| **Clients** | — | Achats des produits exportés (usine, logistique, port) et de la demande extérieure abstraite (§4.4) |
| **Employeurs extérieurs** | — | Salaires des **≈ 150 navetteurs sortants**, qui travaillent hors de la carte ([population.md](population.md), §3.2) |
| **État** | Cotisations, impôts nationaux, TVA, amendes nationales | Salaires des services publics nationaux, retraites, chômage, aides, dotations aux communes (§8) |
| **Monde financier** | Intérêts des prêts refinancés, placements | Refinancement des banques, dividendes de placements extérieurs |
| **Ménages extérieurs** | Dépenses de vacances, envois aux proches, successions sans héritier local | **Héritages** des proches à l'extérieur (§5.11), aides familiales ponctuelles |

*Pourquoi : quelques comptes suffisent à boucler l'économie ; c'est par eux, et seulement par eux, qu'on règle la stabilité (§10).*

### 2.3 Le grand livre (décidé)

- Chaque opération est une **écriture** : date, montant, compte débité, compte crédité, motif (salaire, achat, loyer…), lieu, et indicateur **déclarée / non déclarée** (§7).
- Les écritures sont regroupées par **jour de jeu** et sauvegardées dans PostgreSQL avec le reste du monde ([05-architecture.md](../05-architecture.md), §2.8).
- En simulation complète, un achat donne une écriture ; en simulation statistique, un **bloc d'agenda** donne quelques écritures agrégées (§9). Le total est le même.
- Les écritures en **espèces** ne sont visibles que du cœur : ni la banque, ni la police, ni l'administration fiscale ne les voient, sauf au moment d'un dépôt ou d'un retrait.
- **Invariants vérifiés en production** (décidé, revue de sécurité S23, S30). Chaque jour, le serveur contrôle que :
  - la somme des écritures est nulle, comptes extérieurs compris ;
  - chaque objet unique n'existe qu'une fois.

  Si une violation est trouvée, les **flux concernés sont gelés automatiquement**. La **session d'exploitation** annule ensuite de façon ciblée les écritures fautives, sur le rapport de l'invariant et sans retour en arrière du monde ; si un joueur a exploité le bogue sciemment, la session de modération le décide sur dossier (décidé, A8). *Pourquoi : un bogue de reprise ou de transaction interrompue peut dupliquer de l'argent, et un joueur qui le découvre le répète ; il faut le voir le jour même et réparer sans toucher au reste.*

*Pourquoi : un journal unique sert à la fois au contrôle de la stabilité, aux enquêtes (traçabilité) et au calibrage.*

---

## 3. Métiers et travail

### 3.1 La fiche de métier (décidé)

Un métier est une **fiche de rôle** au sens de l'IA ([ia-pnj.md](ia-pnj.md), §0). Ce domaine en fournit la partie économique :

| Champ | Contenu |
|---|---|
| Intitulé, secteur | Boulanger, commerce de bouche |
| Employeurs possibles | Types de lieux qui ouvrent ce poste ([ville.md](ville.md), §9.1) |
| Contrats possibles | CDI, CDD, intérim, mission, indépendant, fonctionnaire, au noir (§3.2) |
| Horaires | Plages, jours, roulements, nuit (§3.4) |
| Compétences requises | Compétences et niveau minimal ; diplômes ou permis (permis de conduire, licence de débit de boissons) |
| Palier typique | Indicatif, pour la population ([02-personnages.md](../02-personnages.md), §3.4) |
| Salaire | Fourchette, progression avec l'ancienneté et la compétence (§3.5) |
| **Tâches** | Catalogue de tâches avec durée, conditions et valeur produite (§3.3) |
| Recettes et actions propres | Pour l'IA ; ce que le métier permet (ouvrir la caisse, prescrire, interpeller) |
| Savoirs et accès | Clés, codes (opaques), accès aux locaux et aux véhicules de l'entreprise |
| Évolution | Promotions possibles et conditions (§3.6) |

*Pourquoi : un format unique sert à l'IA, à l'embauche, à la paie et au personnage incarné ; ajouter un métier ne demande que des données.*

### 3.2 Les types de contrat (décidé)

| Contrat | Principe | Qui |
|---|---|---|
| **CDI** | Horaires fixes, salaire mensuel, congés payés, chômage en cas de licenciement | La majorité des salariés |
| **CDD** | Comme le CDI, avec une date de fin (remplacement, saison) | Jeunes, saisonniers |
| **Intérim** | L'agence d'intérim place un salarié chez une entreprise pour quelques jours ou semaines ; paie hebdomadaire | Chômeurs, jeunes, personnages persistants |
| **Mission à la tâche** | Une mission courte (15 min à 4 h) payée à l'acte, via le tableau des missions (§3.7) | Étudiants, retraités, chômeurs, personnages persistants |
| **Indépendant** | Artisan, commerçant, profession libérale, chauffeur de taxi : revenu = bénéfice de son activité | Paliers 2 à 6 |
| **Fonctionnaire** | Emploi public stable (mairie, hôpital, police, écoles) ; ne peut pas être licencié pour motif économique | Services publics (§8) |
| **Au noir** | Aucun contrat : paiement en espèces, ni cotisations ni protection (§7.2) | Économie parallèle |

Durée légale du travail : **35 heures par semaine** ; heures supplémentaires payées 25 % de plus ; **5 semaines de congés payés** par an **[à calibrer]**.

### 3.3 Les tâches (décidé)

Comme une journée de travail dure 8 heures réelles en ligne, le travail est découpé en **tâches** ([04-joueur.md](../04-joueur.md), §1.1 ; [ia-pnj.md](ia-pnj.md), §2.3).

- Une tâche dure **de quelques minutes à une heure** réelle **[à calibrer]**. Une journée en compte de 15 à 60.
- Elle a des **conditions** (un client attend, une commande est arrivée, une machine est en panne) et une **fin constatée par la simulation** (le client est servi, la voiture réparée, le colis livré).
- Elle **produit de la valeur** pour l'employeur : une vente, une prestation facturée, une production. C'est ce qui relie le travail au chiffre d'affaires de l'entreprise (§4.2).
- Sa **qualité** dépend des compétences et de l'état du personnage (fatigue, stress, blessures) : une tâche mal faite rapporte moins, crée un client mécontent ou un défaut.
- Le **rendement** d'une équipe dépend aussi de son **ambiance** ([social.md](social.md), §6.4) (décidé) : bonne ambiance, +5 % de tâches accomplies et moins d'erreurs ; mauvaise ambiance, jusqu'à −15 %, plus d'erreurs, d'absences et de démissions **[à calibrer]**. L'effet passe par le stress et la motivation des personnages, pas par une règle à part. *Pourquoi : l'asymétrie (une mauvaise ambiance coûte plus qu'une bonne ne rapporte) est réaliste et donne au patron une raison économique de soigner son équipe.*
- Il y a toujours des **tâches de fond** quand rien ne presse (ranger, nettoyer, faire l'inventaire, préparer), pour qu'un joueur ne reste pas inactif.
- **Barème du travail** (décidé, revue d'équilibrage E11) :
  - chaque tâche accomplie rapporte **1 à 5 XP** selon sa durée **[à calibrer]** ;
  - la somme des tâches est plafonnée à **30 XP par heure réelle** de travail, avec des **rendements décroissants au-delà de 4 heures** de travail dans la journée ;
  - les **obligations** du barème restent dues en plus : **être à l'heure** (20 XP) et **finir sa journée ou sa mission** (20 à 50 XP) ([04-joueur.md](../04-joueur.md), §1.1) ;
  - une tâche **créée par un personnage du cercle du joueur** (§12.4 ; le complice qui commande dix cafés) ne compte pas ;
  - avec les multiplicateurs, une journée bien faite reste au **rythme de la courbe du socle**, environ 100 XP par heure.

  *Pourquoi : 20 à 50 XP pour une tâche de deux minutes donnaient dix fois le rythme sur lequel la courbe est construite ; c'est la journée tenue, pas le nombre de gestes, que le barème récompense.* Les missions à la tâche (§3.7) suivent le même barème.
- Le joueur peut **rendre le personnage** à tout moment : l'IA termine la journée.

Exemples **[à calibrer]** :

| Métier | Tâches typiques (durée réelle) |
|---|---|
| Serveur de bar | Prendre une commande (2 min), servir une table (3 min), encaisser (1 min), débarrasser (3 min), changer un fût (10 min), **mettre de la musique** (1 min, selon l'heure et la clientèle), fermer le bar (30 min) |
| Vendeur en boutique | Conseiller un client (5 min), encaisser (1 min), réassortir un rayon (15 min), **mettre de la musique** (1 min, à l'ouverture et quand l'ambiance change), plier et ranger (10 min) |
| Boulanger | Pétrir une fournée (20 min), cuire (30 min), mettre en rayon (10 min), servir un client (2 min), nettoyer le fournil (30 min) |
| Mécanicien | Diagnostiquer une panne (15 min), changer des plaquettes (45 min), vidange (30 min), faire un devis (10 min) |
| Livreur | Charger une tournée (15 min), livrer un colis (5 à 15 min avec le trajet), faire signer, rapporter les retours |
| Agent de sécurité | Ronde (20 min), contrôle d'accès (2 min), rapport d'incident (10 min) |
| Infirmier | Soin à un patient (10 à 20 min), distribution des traitements (30 min), transmission à l'équipe suivante (15 min) |
| Employé de mairie | Traiter un dossier (15 à 30 min), accueillir un usager (5 min) |

### 3.4 Horaires et emplois de nuit (décidé)

Les horaires viennent de la fiche de métier et des **horaires d'ouverture** du lieu ([ville.md](ville.md), §9.1). Trois formes : horaires de jour, **roulements** (2×8, 3×8 dans l'usine, la logistique, l'hôpital, la police) et **horaires de nuit**.

Pour que la nuit en ligne ne soit pas vide ([ia-pnj.md](ia-pnj.md), §2.3), **environ 12 % des emplois** comportent un travail de nuit habituel ou en roulement, soit ≈ 500 emplois, et **≈ 150 à 250 personnes travaillent à 3 h du matin** **[à calibrer]** :

| Secteur | Emplois de nuit |
|---|---|
| Santé | Infirmiers, aides-soignants, médecins de garde, urgences (Beaulieu), pharmacie de garde |
| Sécurité | Police nationale et municipale en patrouille, pompiers, agents de sécurité et gardiens (usine, port, parking, hôpital), veilleurs de nuit (hôtel, foyer) |
| Industrie et logistique | Usine principale en 3×8, plateforme logistique (préparation et chargement de nuit), port (déchargement) |
| Restauration et loisirs | Bars (fermeture vers 1 h, 2 h le week-end), boîte de nuit (jusqu'à 5 h), restauration rapide tardive, épicerie de nuit |
| Alimentation | Boulangers (début vers 4 h), livraisons aux commerces au petit matin, marché (installation à 5 h) |
| Transports | Ligne de bus de nuit, taxis, dépanneurs |
| Propreté | Nettoyage des bureaux (tôt le matin ou le soir), collecte des déchets à l'aube, déneigement l'hiver |
| Parallèle | Revendeurs, receleurs, cambrioleurs (§7) |

Le travail de nuit est mieux payé (**+20 %** des heures de nuit) **[à calibrer]**, ce qui le rend attractif pour les personnages persistants qui jouent tard.

### 3.5 Rémunération et paie (décidé)

| Contrat | Moment de la paie | Forme |
|---|---|---|
| CDI, CDD, fonctionnaire | Dernier jour ouvré du mois | Virement |
| Intérim | Chaque fin de semaine | Virement |
| Mission | À la validation de la mission (§3.7) | Virement, ou espèces si non déclarée |
| Indépendant | Il se verse ce qu'il veut sur la trésorerie de son entreprise | Virement |
| Au noir | À la fin de la journée ou du chantier, comme convenu | Espèces |

- Le salaire progresse avec l'**ancienneté** (≈ +1 % par an) et avec la **compétence** (passage à l'échelon supérieur quand la compétence franchit un seuil) **[à calibrer]**.
- **Primes** possibles : fin d'année, résultats, pourboires (espèces, dans la restauration et les taxis).
- Les **absences non payées** (absence injustifiée, grève) sont retenues sur le salaire ; une absence pour maladie est payée (après un jour de carence) grâce à l'assurance maladie (État, §8.3).
- Le salaire est dû même si le personnage est incarné par un joueur ou piloté par l'IA : l'employeur ne voit qu'un employé.

### 3.6 Le marché du travail (décidé)

**Offres.** Une entreprise ouvre un poste quand elle manque de bras (départ, croissance, surcharge mesurée sur ses tâches en attente). L'offre est publiée à la **Maison de l'emploi** (service public, dans la Cité, antenne aux Forges), sur le **tableau des missions** pour les besoins courts, à l'agence d'intérim, ou par bouche-à-oreille (domaine social).

**Postes vacants au départ.** La population laisse **≈ 100 postes vacants** (2 à 3 %) à la création du monde ([population.md](population.md), §3.2) : ils sont publiés comme offres dès le premier jour, pour les jeunes qui entrent dans la vie active, les chômeurs, les nouveaux venus et les personnages persistants.

**Embauche.** Le recruteur (le gérant, piloté par l'IA ou par un joueur) compare les candidats sur : compétences et diplômes, expérience, **fiabilité** connue (§3.7), casier judiciaire s'il le demande, prétention salariale, et ce qu'il sait du candidat par sa **réputation** et ses **relations** (il se renseigne auprès de ceux qu'il connaît, [social.md](social.md), §5.7) (décidé). Un candidat recommandé par quelqu'un en qui le recruteur a confiance passe devant à compétence égale ; le **favoritisme** (embaucher un proche moins compétent) existe, d'autant plus que le recruteur respecte peu les règles. *Pourquoi : c'est ainsi que se pourvoient la plupart des emplois d'une petite ville, et cela donne une valeur économique au réseau du personnage.* Le domaine social fixe le déroulé de l'entretien.

**Recrutement extérieur** (décidé). Une offre qui reste sans candidat convenable pendant **4 semaines de jeu** (1 semaine pour un poste indispensable) **[à calibrer]** est signalée au **régulateur démographique** ([population.md](population.md), §7.4 et §7.6), qui fait venir un ménage dont un membre a le métier recherché. Le poste reste ouvert aux habitants jusqu'à l'arrivée. *Pourquoi : un poste rare (chirurgien, pilote de la navette) ne doit pas rester vide, et passer par une vraie arrivée respecte la règle « aucun figurant ».*

**Absences et renvoi** (règle de [ville.md](ville.md), §2.5) :

| Événement | Conséquence |
|---|---|
| Retard | Remarque ; répétés, ils comptent comme une absence **[à calibrer]** |
| 1ʳᵉ absence injustifiée | Avertissement oral, retenue sur salaire |
| 2ᵉ absence injustifiée dans les 30 jours | Avertissement écrit |
| 3ᵉ absence injustifiée dans les 30 jours, ou abandon de poste de 3 jours | **Renvoi** (licenciement pour faute, sans indemnité) |
| Faute grave (vol, violence, ivresse) | Renvoi immédiat |

Un congé posé à l'avance, un arrêt maladie ou un séjour annoncé ne sont pas des absences injustifiées. Ces règles s'appliquent à tous, et garantissent qu'un **poste indispensable n'est jamais bloqué longtemps** par un personnage persistant absent.

**Autres fins de contrat** : démission (pas de chômage ; **préavis** d'un mois, deux semaines pour un CDD ou un emploi de palier 0 à 1, pendant lequel elle peut être retirée d'un commun accord, §12.2), fin de CDD, **licenciement économique** (entreprise en difficulté, §4.6 : indemnité d'un mois de salaire par tranche de 4 ans d'ancienneté, chômage) **[à calibrer]**, départ en retraite.

**Promotion.** Quand un poste supérieur se libère, il est proposé en priorité à un employé de l'entreprise qui a les compétences, l'ancienneté et une bonne fiabilité. Une promotion peut faire changer de palier (composante **Autorité**, [02-personnages.md](../02-personnages.md), §3.2). Elle peut être l'aboutissement d'un projet du personnage ([ia-pnj.md](ia-pnj.md), §3.4). Comme pour l'embauche, le respect et l'affection du décideur envers le candidat comptent à côté de la compétence ([social.md](social.md), §6.4).

**Grève** (décidé). La grève naît comme **projet collectif** des salariés d'une entreprise ([social.md](social.md), §6.4 et §6.9). Ce domaine en fixe les effets :

- les grévistes **ne sont pas payés** pour les heures de grève ; une **caisse de grève** (compte de groupe du syndicat ou de l'association, §2.1) peut les aider ;
- l'entreprise perd la production des grévistes : ventes manquées, retards, service perturbé (bus, [ville.md](ville.md), §7.3) ; dans un **service public**, un **service minimum** est assuré par les non-grévistes quand c'est possible ;
- un salarié ne peut pas être sanctionné pour avoir fait grève : ce n'est **pas une absence injustifiée** ;
- la grève se termine par une **négociation** entre le gérant et les meneurs, dont l'issue dépend de la trésorerie de l'entreprise, de la durée de la grève, du profil du gérant et de la loyauté des grévistes : hausse de salaire ou prime (dans la fourchette du métier), amélioration des conditions, ou échec et reprise du travail.

*Pourquoi : des règles économiques simples (perte de salaire contre perte de production) suffisent à rendre la décision de faire grève et celle de céder crédibles des deux côtés.*

### 3.7 Les missions à la tâche (décidé)

Les emplois à la tâche conviennent le mieux à la présence irrégulière du personnage persistant ([ville.md](ville.md), §2.5). Ils passent par le **tableau des missions** : une application sur le téléphone de chaque personnage, et un panneau à l'agence d'intérim (quartier de la Gare) et à la Maison de l'emploi.

- **D'où viennent les missions** : de vrais besoins des entreprises et des habitants du monde, jamais d'un générateur de quêtes : un restaurant qui manque d'un serveur ce soir, la plateforme logistique en surcharge, un commerce qui reçoit une grosse livraison, un habitant qui déménage, un chantier en retard, un jardin à entretenir, un événement à surveiller.
- **Forme d'une mission** : donneur d'ordre, lieu, heure de début, durée estimée (15 min à 4 h), rémunération, compétences ou permis requis.
- **Déroulé** : accepter la mission crée un **engagement** ; se rendre sur place ; accomplir les tâches ; le donneur d'ordre **valide** selon la qualité ; paiement immédiat.
- **Rémunération** : **11 à 15 écus de l'heure** nets **[à calibrer]**, un peu plus que le salaire minimum horaire mais sans congés ni chômage, comme dans la réalité. Majoration de nuit.
- **Fiabilité** : chaque donneur d'ordre et l'agence notent la **fiabilité** (venu, à l'heure, travail bien fait). Une bonne fiabilité donne accès à des missions mieux payées et à des propositions d'embauche ; une mission acceptée puis non honorée la fait chuter. C'est une valeur interne de 0 à 100, que le personnage connaît **en mots**, comme une note de plateforme ([interface.md](interface.md), §8.3) (décidé) : « nouvelle » (moins de 5 missions), « fragile » (< 40), « correcte » (40 à 60), « bonne » (60 à 85), « excellente » (> 85) **[seuils à calibrer]**. *Pourquoi : une note en mots suffit à décider et évite d'afficher un chiffre interne.*
- **Les PNJ y ont accès aussi** (étudiants, chômeurs, retraités, adolescents de 16 ans et plus pour certaines missions) : les joueurs sont en concurrence avec eux, et une mission non prise par un joueur l'est par un PNJ.
- **Pas de garde d'enfants** proposée aux personnages persistants (décidé). *Pourquoi : confier un enfant à un personnage piloté par un inconnu va contre l'esprit de protection des mineurs du socle ([02-personnages.md](../02-personnages.md), §3.4).*

Un personnage persistant actif peut ainsi gagner **≈ 50 à 120 écus par jour de jeu** joué au début **[à calibrer]**, assez pour payer un studio en une à deux semaines de jeu régulier.

### 3.8 Catalogue indicatif des métiers **[à calibrer]**

| Secteur | Métiers (palier typique) | Contrat courant | Horaires |
|---|---|---|---|
| Commerce | Caissier (1), vendeur (1), épicier (2), boulanger (2), boucher (2), gérant de supérette (3), directeur de l'hypermarché (5) | CDI, indépendant | Jour, samedi, boulanger de nuit |
| Restauration, loisirs | Plongeur (0), serveur (1), barman (1), cuisinier (2), patron de bar (3), videur (2), patron de la boîte (5) | CDI, CDD, mission, au noir | Soir, nuit |
| Musique (v0.7) | Musicien de rue (0), musicien de bar ou de fête (1 à 2), organiste de l'église (2), professeur de musique (2 à 3), animateur de radio locale (3) | Au chapeau, cachet à la soirée, vacation, CDI, indépendant | Soir, week-end |
| Artisanat, bâtiment | Manœuvre (0), maçon (1), plombier (2), électricien (2), serrurier (2), vitrier (2), patron de l'entreprise de bâtiment (6) | CDI, indépendant, intérim, au noir | Jour |
| Automobile | Mécanicien (2), carrossier (2), dépanneur (2), garagiste (3), pompiste (1) | CDI, indépendant | Jour, astreinte |
| Industrie, logistique | Opérateur de production (1), cariste (1), préparateur de commandes (1), chef d'équipe (3), ingénieur (4), directeur de l'usine (8) | CDI, intérim | 3×8 |
| Transports | Chauffeur de bus (2), cheminot (2), pilote de la navette (2), taxi (2), livreur (1), chauffeur routier (1) | CDI, indépendant, mission | Roulements, nuit |
| Santé | Aide-soignant (2), infirmier (3), pharmacien (4), médecin généraliste (4), chirurgien (6), directeur de l'hôpital (7) | Fonctionnaire, indépendant | Roulements, nuit |
| Sécurité | Agent de surveillance de la voie publique (1), agent de sécurité (1), policier municipal (3), gardien de la paix (3), policier de la brigade anticriminalité (4), enquêteur, enquêteur financier (4 à 5), technicien de police scientifique (3 à 4), commissaire adjoint (6), commissaire central (hors catégorie), pompier (3) | Fonctionnaire, CDI | Roulements, nuit |
| Justice et contrôles | Greffier (3), conseiller d'insertion et de probation (3), contrôleur fiscal (4), inspecteur du travail (4), commissaire de justice (5), mandataire judiciaire (5), juges (6 à 7), substitut du procureur (6), procureur (hors catégorie) | Fonctionnaire ; commissaire de justice et mandataire indépendants | Jour, permanences |
| Éducation | Assistante maternelle (1), animateur (1), professeur des écoles (3), professeur (3), chef d'établissement (5) | Fonctionnaire, CDI | Jour |
| Bureau, services | Employé de bureau (1), comptable (3), agent immobilier (3), banquier (3 à 5), assureur (3), avocat (5), notaire (6), journaliste (3 à 6) | CDI, indépendant | Jour |
| Public | Agent municipal (1), jardinier (1), employé de mairie (2), secrétaire de mairie (4), conseiller municipal (7), maire (hors catégorie pour la ville-centre) | Fonctionnaire, élu | Jour, astreinte |
| Propreté | Agent d'entretien (0 à 1), éboueur (1) | CDI, mission, au noir | Tôt le matin, soir |
| Parallèle | Guetteur (0), revendeur (1 à 2), receleur (3), cambrioleur (2 à 4), chef de réseau (7 à 9) | Aucun | Variables |
| Sans emploi | Étudiant (0), chômeur (0), retraité (0 à 3), parent au foyer (0) | — | — |

Le domaine population répartit ces métiers dans les 4 100 emplois ([ville.md](ville.md), §2.3). Les fiches de police, de justice et de contrôle, avec leurs recettes et leurs effectifs (≈ 80 personnes), sont fournies par [combat.md](combat.md), §5 et §14 ; ce domaine leur ajoute la partie économique :

| Métier | Salaire net mensuel **[à calibrer]** | Employeur |
|---|---|---|
| Agent de surveillance de la voie publique | 1 550 | Commune |
| Policier municipal | 1 900 à 2 400 | Commune |
| Gardien de la paix, chef de poste | 2 100 à 2 800 | État |
| Enquêteur, policier de la brigade anticriminalité, technicien | 2 500 à 3 300 | État |
| Commissaire adjoint / commissaire central | 4 200 / 5 500 | État |
| Greffier, conseiller d'insertion et de probation | 1 900 à 2 600 | État |
| Contrôleur fiscal, inspecteur du travail | 2 400 à 3 300 | État |
| Juge, substitut | 3 500 à 5 500 | État |
| Procureur | 6 000 | État |
| Commissaire de justice, mandataire judiciaire | 4 000 à 8 000 (revenu d'indépendant : actes, frais de saisie et d'expulsion, honoraires de liquidation) | Indépendant |
| Juge consulaire (tribunal de commerce) | Bénévole, en plus de son métier | — |

**Musiciens et musique dans les commerces** (décidé, v0.7 ; [ia-pnj.md](ia-pnj.md) v1.1, §2.6 ; [direction-artistique.md](direction-artistique.md), §5.4). Jouer en public est une activité ou un poste, payé ou non.

| Situation | Rémunération **[à calibrer]** | Écriture |
|---|---|---|
| Musicien de bar, de fête ou de mariage | **Cachet** de 80 à 150 la soirée, selon sa compétence et sa réputation | Payé par l'établissement ou l'hôte, souvent en espèces et non déclaré |
| Musicien de rue | **Au chapeau** : chaque passant décide de donner ou non, de 0,50 à 2 écus. Cela fait ≈ 5 à 30 écus de l'heure selon le lieu, l'heure, l'affluence et la qualité | Petits dons en espèces des passants |
| Organiste, chorale | Vacation d'≈ 50 par office ; mariages et obsèques payés par les familles | Paroisse (association, §4.9) |
| Professeur de musique | ≈ 25 de l'heure en cours particulier, ou salarié de l'école de musique | Déclaré |

- **Au chapeau** : chaque don est un vrai petit don au sens du §12.1, toujours anodin à cette échelle. Un don « au montant libre » (chapeau, pourboire, quête) est **borné à ≈ 20 écus** **[à calibrer]** (décidé, clôture d'équilibrage C1) : on ne glisse pas une liasse dans un chapeau. Au-delà, c'est un don en main propre ordinaire, soumis à toutes les règles du §12. En statistique, la recette d'une séance est tirée selon les mêmes critères.
- **Musique dans un bar ou une boutique** : « mettre de la musique » est une **tâche du poste** (serveur, vendeur, patron, §3.3). Un établissement qui diffuse de la musique paie une **redevance de diffusion** de 20 à 60 par mois à un organisme fictif, sur le compte extérieur « Fournisseurs ». Une musique bien choisie améliore un peu l'accueil perçu ([social.md](social.md), §5.9) ; trop forte, elle gêne les voisins.

*Pourquoi : la musique jouée en ville vient de vrais habitants qui en vivent plus ou moins bien, et le cachet au noir comme le chapeau sont les formes réelles de ce petit marché.*

### 3.9 Rôles économiques du prototype (décidé)

Pour le prototype du faubourg Saint-Roch ([05-architecture.md](../05-architecture.md), §5), une douzaine de rôles de palier 0 à 3 suffisent ([ia-pnj.md](ia-pnj.md), §10) : **boulanger**, **serveur de bar**, **patron de bar**, **épicier**, **mécanicien**, **livreur à la mission**, **agent d'entretien**, **petit revendeur**, **retraité**, **chômeur**, **étudiant**, et le **personnage persistant** en missions. *Pourquoi : ils couvrent les contrats (CDI, indépendant, mission, au noir, sans emploi), le jour et la nuit, l'économie légale et parallèle, avec les lieux que le quartier du prototype contient déjà.*

---

## 4. Entreprises et commerces

### 4.1 Le modèle d'une entreprise (décidé)

Une **entreprise** est une entité du monde, distincte de ses lieux et de ses employés. Le même modèle sert aux commerces, artisans, usines, services publics, associations et réseaux criminels.

| Donnée | Contenu |
|---|---|
| Forme | Indépendant, société, service public, association, **clandestine** (§7) |
| Propriétaires | Personnages ou entreprises, avec leurs parts ; un **gérant** qui décide (souvent le propriétaire) |
| Établissements | Un ou plusieurs lieux ([ville.md](ville.md), §9.1), loués ou possédés |
| Personnel | Contrats de travail (§3.2) et **organigramme** (voir ci-dessous) |
| Trésorerie | Compte en banque et caisse en espèces |
| Stocks | Par catégorie de marchandises (§4.3) |
| Prix pratiqués | Par catégorie (§6.2) |
| Fournisseurs et clients | Habituels, avec délais de paiement |
| Dettes et créances | Prêts bancaires, factures impayées, loyers dus |
| Réputation | Qualité, prix, accueil, perçus par les clients (domaine social) |
| État | Active, en difficulté, en redressement, fermée (§4.6) |

*Pourquoi : un modèle unique permet de traiter toutes les organisations économiques avec les mêmes systèmes, y compris les trafics, et à un joueur de passer de l'une à l'autre.*

**Une seule entité pour l'économie et le social** (décidé). L'entreprise est aussi un **groupe** au sens du domaine social ([social.md](social.md), §6.1) : la même entité porte les composants économiques (ce paragraphe) et les composants sociaux (normes, loyauté, projets collectifs, réputation). *Pourquoi : deux entités pour la même organisation finiraient par se contredire (un employé licencié encore membre de l'équipe).*

**Organigramme** (décidé). C'est l'organisation formelle sur laquelle s'appuient la loyauté, les ordres et la délégation ([social.md](social.md), §6.7 et §6.8) :

| Élément | Contenu |
|---|---|
| **Poste** | Un métier (§3.1) dans cette entreprise, avec son titulaire (ou vacant), ses horaires et son salaire |
| **Rang** | Direction, encadrement, employé ; il fixe qui peut **ordonner** à qui et l'**autorité** du supérieur sur ses subordonnés |
| **Supérieur direct** | Le poste dont il dépend ; l'ensemble forme un arbre |
| **Suppléant** | Pour chaque poste à responsabilité : le poste ou la personne qui le remplace en cas d'absence, **désigné** par le titulaire ou, à défaut, implicite ([social.md](social.md), §6.8) |
| **Pouvoirs** | Ce que le poste permet de décider : dépenses jusqu'à un plafond, embauche, prix, signature des contrats, accès à la caisse et aux comptes |

- Un indépendant seul a un organigramme d'un seul poste ; l'usine en a plusieurs centaines.
- L'organigramme alimente la composante **Autorité** des dirigeants ([02-personnages.md](../02-personnages.md), §3.2) : rang × taille de l'organisation dirigée.
- Les **pouvoirs** bornent ce qu'un joueur peut faire en incarnant un employé : un caissier n'accède pas aux comptes de l'entreprise, un chef de rayon ne signe pas un bail.

### 4.2 Le fonctionnement quotidien (décidé)

- **Recettes** : ventes aux clients réels (§4.4), prestations facturées, exportations, subventions.
- **Charges fixes** (mensuelles) : loyer du local ou remboursement du prêt, salaires, énergie, assurances, taxe foncière ou cotisation foncière (§8.2).
- **Charges variables** : achats de marchandises et de matières, carburant, sous-traitance.
- **Clôture journalière** : chaque nuit de jeu, l'entreprise fait ses comptes : ventes du jour, état des stocks, commandes à passer, trésorerie. C'est à ce moment que le gérant (ou l'automate de gestion, §9.5) prend ses décisions courantes : réapprovisionner, ajuster un prix, demander un renfort, déposer la caisse.
- **Clôture mensuelle** : paie, loyers, impôts, remboursements ; décisions plus lourdes (embaucher, licencier, investir, chercher un local plus grand).
- **Bénéfice** : ce qui reste est conservé en trésorerie, versé aux propriétaires (dividendes ou revenu de l'indépendant) ou investi.

### 4.3 Stocks et approvisionnement (décidé)

- Les stocks sont tenus **par catégorie de marchandises** (pain, boissons, produits frais, épicerie, tabac, carburant, pièces automobiles, matériaux…) et en **quantités**, pas objet par objet.
- Les objets ne sont **instanciés** que quand ils sont visibles ou manipulés en simulation complète (le rayon de la boulangerie, la bouteille servie, la pièce montée sur la voiture). Un objet sorti du stock et emporté devient un objet du monde, avec un propriétaire (§5.10).
- **Réapprovisionnement** : le gérant commande quand le stock passe sous un seuil. La commande part chez un **grossiste** (plateforme logistique des Forges) ou directement chez un fournisseur extérieur ; elle est **livrée par un vrai camion** conduit par un chauffeur habitant ([ville.md](ville.md), §7.4), avec un délai (le lendemain matin en général) **[à calibrer]**.
- **Produits périssables** : le pain se perd le soir, les produits frais en quelques jours. Les invendus sont jetés, bradés ou donnés (selon le profil du gérant).
- **Rupture** : un rayon vide est visible ; le client va ailleurs (choix de recette de l'IA) et la réputation baisse.
- **Production locale** : boulangeries (farine → pain), restaurants (produits → repas), artisans (matériaux → prestation), usine (matières → produits exportés), maraîchers des environs au marché hebdomadaire.

*Pourquoi : des stocks agrégés coûtent peu à simuler pour 600 établissements, tout en gardant visibles les ruptures, les livraisons et les vols de marchandises.*

### 4.4 La clientèle (décidé)

- Les clients d'un commerce sont les **habitants qui y achètent**, selon les recettes de l'IA ([ia-pnj.md](ia-pnj.md), §3.3) : distance, habitude, prix, réputation, horaires, météo. En simulation statistique, le choix est tiré selon ces mêmes critères (§9.3).
- **Valeurs apprises et fidélité** (décidé, [ia-pnj.md](ia-pnj.md) v0.8, §13.6 ; [social.md](social.md), §5.9). Chaque client garde ses propres **valeurs apprises** sur les commerces qu'il fréquente, dans trois domaines :
  - la **qualité** : ce qu'il a reçu ;
  - le **prix** : ce qu'il a payé, comparé au prix de référence (§6.1) ;
  - l'**arnaque** : surfacturation, monnaie mal rendue, produit défectueux, prestation facturée mais mal faite ou non faite.

  Ces valeurs entrent dans le choix du commerce.
  - **Commerce connu** : la valeur apprise compte plus que la réputation.
  - **Commerce jamais essayé** : seule compte la réputation, qui est collective et nourrie par le bouche-à-oreille.
  - **Choix répété et satisfaisant** : il donne le bonus d'**habitude**, puis la **fidélité** (lien « client habituel », prix d'ami, ardoise, §6.2).
  - **Arnaque découverte** : elle s'apprend **en une fois**. Le client cesse de venir, le raconte, ce qui fait baisser la réputation, et peut réclamer ou porter plainte (§7.8).
  - **Retour du client** : les valeurs reviennent lentement vers la valeur de départ, et un ancien client finit par revenir.

  Un commerce qui triche gagne donc un peu tout de suite et perd ses habitués ensuite, que son gérant soit piloté par l'IA ou incarné par un joueur. La baisse de fréquentation se retrouve dans le facteur offre/demande du prix local (§6.2). Ce mécanisme existe à tous les niveaux de détail : en statistique, les valeurs sont mises à jour sur les résultats des blocs et sur les événements tirés (§9.3).

  *Pourquoi : la concurrence entre commerces doit naître de l'expérience de chaque client, sans règle de marché à part. La valeur apprise individuelle, la réputation collective et la fidélité couvrent ensemble le premier essai, l'habitude et la rupture.*
- **Établissements à rayonnement extérieur** : l'agglomération a les services d'une ville moyenne pour la population d'une petite ville ([ville.md](ville.md), §2.2). Quelques établissements reçoivent donc une **demande extérieure abstraite**, payée par le compte extérieur « Clients » :

| Établissement | Part extérieure de la clientèle **[à calibrer]** | Forme |
|---|---|---|
| Hôtel de la Gare | ≈ 70 % | Chambres réservées par des voyageurs d'affaires non simulés : marquées occupées et fermées à clé (« ne pas déranger ») ; personne n'y est visible |
| Hypermarché | ≈ 25 % | Commandes en ligne et retraits au drive pour les villages voisins |
| Restaurant gastronomique, club de golf | ≈ 30 % | Réservations de groupes extérieurs, comptées sur les jours où la salle a de la place |
| Plateforme logistique, usine, port | Presque tout | Commandes de clients extérieurs (§4.8) |

Personne d'inventé n'est jamais **visible** : la règle « aucun figurant » est respectée, comme pour les trains de marchandises en transit ([ville.md](ville.md), §7.4). *Pourquoi : sans cette demande, l'hôtel et quelques lieux emblématiques feraient faillite en solo, alors qu'ils sont nécessaires (arrivée du personnage persistant, lieux faits main).*

### 4.5 La création d'une entreprise (décidé)

Une création naît d'un **projet** ([ia-pnj.md](ia-pnj.md), §3.4) d'un personnage (motivation argent ou statut), ou de la décision d'un joueur. Étapes :

1. **Idée** : l'économie publie chaque mois des **opportunités** par quartier, calculées sur la demande non servie (pas de boulangerie à moins de 400 m, bars toujours pleins, longues attentes au garage), avec le calcul des aires de chalandise ci-dessous. Un PNJ ambitieux et compétent dans le domaine les « remarque » ; un joueur les devine en observant.
2. **Financement** : apport personnel (en général 20 à 30 % du besoin) et **prêt bancaire** si le dossier convainc la banque (§4.10).
3. **Local** : louer ou acheter un local libre compatible avec les règles d'urbanisme du quartier ([ville.md](ville.md), §6.2), ou reprendre un fonds de commerce existant.
4. **Autorisations** : immatriculation (immédiate), licence de débit de boissons pour un bar, permis d'aménagement si travaux, agrément pour certaines professions (pharmacie, armurerie, taxi) **[à calibrer]**.
5. **Aménagement** : travaux par l'entreprise de bâtiment (§5.7), achat du mobilier et du premier stock.
6. **Ouverture** : embauche, horaires publiés, enseigne allumée.

Ordres de grandeur du capital nécessaire **[à calibrer]** : 15 000 (petit commerce en location) à 80 000 (bar avec licence et travaux), 300 000 (garage équipé).

**Ratios d'équipement et aires de chalandise** (décidé, [revues/generation.md](../revues/generation.md), P9). Chaque type d'établissement a une **population de seuil** et un **rayon** de chalandise, et soit il attire ses semblables (**regroupement**), soit il les repousse (**répulsion**). Ces valeurs sont une table de données (principe 7).

Elles servent deux fois, **avec le même calcul** :
- **à la génération** : le Programme en déduit le nombre d'établissements par type, puis la ville les place là où sont leurs clients, ce qui les rend viables par construction ;
- **en jeu** : elles calculent chaque mois les opportunités de l'étape 1.

| Type | Population de seuil (habitants pour un établissement) | Rayon de chalandise | Regroupement ou répulsion |
|---|---|---|---|
| Boulangerie | ≈ 1 500 | 400 m | Répulsion (un concurrent à moins de 200 m est rare) |
| Épicerie, supérette | ≈ 1 500 | 400 m | Répulsion faible |
| Supermarché | ≈ 5 000 | 1,5 km | Répulsion forte |
| Hypermarché | Toute l'agglomération, plus la demande extérieure (§4.4) | Agglomération | Un seul, en périphérie |
| Pharmacie | ≈ 3 000, et au moins une par quartier habité | 600 m | Répulsion forte (règle de quota réaliste) |
| Médecin généraliste | ≈ 1 000 | 800 m | Regroupement en cabinet ou maison de santé |
| Tabac-presse | ≈ 2 500 | 600 m | Répulsion |
| Bar, café | ≈ 600 | 300 m | Regroupement (Cité, faubourg, gare) |
| Restaurant | ≈ 700 | 500 m, plus la clientèle des bureaux | Regroupement (centre, places) |
| Coiffeur | ≈ 1 200 | 500 m | Neutre |
| Garage automobile | ≈ 2 500 | 2 km | Regroupement en zone d'activité (Forges) |
| Agence bancaire | ≈ 2 500 | 1 km | Regroupement au centre |
| École élémentaire | Selon les enfants de 6 à 10 ans (places ≥ enfants) | 800 m | Répulsion (secteur scolaire) |

Toutes ces valeurs sont **[à calibrer]**. Les services dont la ville a décidé l'emplacement ([ville.md](ville.md), §5.2, §5.3) sont posés en premier et entrent dans le calcul comme les autres.

La **demande non servie** d'un type se calcule ainsi :
1. on compte la population dans le rayon ;
2. on la divise par la population de seuil ;
3. on retire les établissements du même type déjà présents dans le rayon.

En jeu, ce compte est **corrigé par la fréquentation observée** : files d'attente, ruptures, chiffre d'affaires des établissements existants (§4.4). Un excédent durable devient une opportunité. *Pourquoi : un seul calcul pour placer les commerces à la génération et pour faire naître les créations en jeu garantit que la ville de départ et la ville vivante obéissent à la même logique, et qu'un commerce créé en jeu ait des clients.*

### 4.6 Difficultés et faillite (décidé)

| Étape | Déclencheur | Ce qui se passe |
|---|---|---|
| **Difficulté** | Trésorerie négative ou pertes trois mois de suite **[à calibrer]** | Le gérant réagit selon son profil : baisse des prix, réduction des horaires, licenciements, découvert ou prêt, apport personnel, recours à l'argent liquide douteux (usurier, §7.6) |
| **Cessation de paiements** | Une dette exigible (salaires, loyer, fournisseur, banque) impayée depuis plus d'un mois | Le gérant doit déclarer la cessation au **tribunal** (la Cité) ; un créancier peut aussi la demander |
| **Redressement** | Décision du tribunal si l'activité semble viable | Plan de remboursement étalé ; l'entreprise continue sous surveillance pendant quelques mois |
| **Liquidation** | Échec du redressement ou activité non viable | Fermeture ; employés licenciés (chômage) ; stocks, mobilier et véhicules **vendus aux enchères** ; créanciers remboursés dans l'ordre (salaires, État, banque, fournisseurs) ; le local est libéré |
| **Reprise** | À tout moment avant la liquidation | Un repreneur (PNJ ou joueur) rachète l'entreprise ou son fonds |

Un **indépendant** répond de ses dettes sur son patrimoine personnel (son logement excepté) ; une **société** non. Les rôles du tribunal (juge, mandataire) sont des fiches de métier ; le domaine combat, police et justice pourra les compléter.

### 4.7 Ouverture, fermeture et changements d'usage (décidé)

C'est la mécanique économique des **changements d'usage** de la ville ([ville.md](ville.md), §6.1).

- **Fermeture** : par faillite, départ en retraite sans repreneur, décès sans héritier repreneur, déménagement, ou décision du propriétaire. Le local devient **vacant** ; l'enseigne s'éteint, les vitrines se vident.
- **Local vacant** : son propriétaire le met en location ou en vente ; le prix baisse avec la durée de vacance. Au-delà de quelques mois **[à calibrer]**, le local se dégrade (état d'entretien), et dans les quartiers peu surveillés il peut devenir un **squat** ou une planque (choix des domaines social et combat).
- **Nouvel usage** : un repreneur choisit l'usage selon les opportunités (§4.5) et les **règles d'urbanisme** du quartier. Un logement peut devenir un local et inversement, avec des travaux et, si besoin, l'accord de la mairie.
- Le changement est inscrit dans le **journal des modifications** de la ville ([05-architecture.md](../05-architecture.md), §2.11).
- **Équilibre** : l'économie vérifie chaque mois que chaque **service indispensable** existe à une distance raisonnable (au moins une boulangerie, une pharmacie, une épicerie par quartier habité, et pour toute l'agglomération un journal local, des pompes funèbres, un tabac-presse **[à calibrer]**). Si l'un manque longtemps, la valeur de l'opportunité augmente fortement, ce qui pousse un PNJ à ouvrir. *Pourquoi : on laisse le marché décider, mais un quartier ne doit pas rester des mois sans pain à cause d'une faillite.*

### 4.8 Grands employeurs et secteurs exportateurs (décidé)

| Employeur | Rôle économique |
|---|---|
| **Usine principale** (Forges) | Exporte sa production ; son carnet de commandes suit la **conjoncture** extérieure (§6.7). Embauches, intérim, chômage partiel ou plan social en découlent. Son propriétaire est hors catégorie |
| **Plateforme logistique** (Forges) | Grossiste de la ville (elle approvisionne les commerces) et entrepôt régional (exportation de service) |
| **Port** | Déchargement des péniches, entreposage ; activité liée aux matières de l'usine et aux matériaux du bâtiment |
| **Hôpital** (Beaulieu) | Service public national, financé par l'État (§8.5) |
| **Mairies** | Employeurs publics locaux (§8.4) |
| **Entreprise de bâtiment** | Constructions, rénovations et réparations (§5.7, §5.8) ; ses commandes viennent du monde |
| **Écoles** | Financées par l'État (enseignants) et les communes (bâtiments, cantine, agents) |

Les secteurs exportateurs sont la principale **entrée d'argent** privée dans l'agglomération : leur volume est l'un des leviers de stabilité (§10.2).

**Le journal local** (décidé, [interface.md](interface.md), §8.1 et §8.6). L'entreprise de presse de la ville (quartier de la Gare, [ville.md](ville.md), §5.3) publie un **quotidien** imprimé à l'aube et sa version sur le téléphone, et anime la radio locale. Son contenu est **produit par son activité**, à partir de ce qu'elle sait :

| Rubrique | Source | Recette |
|---|---|---|
| **Avis de décès**, faire-part de naissance et de mariage | Commandés par les familles, directement ou par les pompes funèbres et la maternité | Payants (≈ 80 à 150 l'avis **[à calibrer]**) |
| **Annonces légales** : créations, ventes de fonds, liquidations, ventes aux enchères, enquêtes publiques | Publiées **automatiquement** à partir du registre (§5.1) et des décisions du tribunal ; obligatoires | Payées par l'entreprise ou la collectivité concernée |
| **Ouvertures, fermetures**, vie économique | Annonces légales, plus ce qu'apprennent les journalistes (communiqués, passage devant un commerce) | Publicité des commerces |
| Faits divers, **chronique judiciaire**, politique municipale, résultats d'élections, sport | **Tâches** des journalistes (§3.3) : se renseigner, interroger, assister à une audience ou à un conseil, rédiger ; un journaliste ne publie que ce qu'il sait ou croit ([social.md](social.md), §4) | Ventes et abonnements |
| Petites annonces (emploi, logement, ventes entre particuliers) | Déposées par les habitants | Payantes |

- Les **annonces payées et légales** paraissent toujours, même si la rédaction est réduite ; les **articles** dépendent des journalistes présents et de leur compétence (un journal sans journaliste ne publie que des annonces).
- Les annonces légales, obligatoires et payées, assurent au journal une **recette stable** ; le journal local figure parmi les **services indispensables** de l'agglomération (§4.7) : s'il fermait, l'opportunité de le reprendre deviendrait maximale (décidé). *Pourquoi : l'interface s'appuie sur lui pour rendre visibles les événements de la ville ; il doit donc presque toujours exister, mais comme une vraie entreprise.*

### 4.9 Services publics et associations (décidé)

Ce sont des entreprises de forme « service public » ou « association » : elles ont des employés, des locaux, des charges et des recettes, mais leur budget vient surtout des **dotations** (État, commune) et non des ventes. Elles ne font **pas faillite** ; en cas de budget insuffisant, elles réduisent leurs services (horaires, postes non remplacés). Le **foyer de travailleurs** est géré par une association subventionnée par la commune d'Orvenne (décidé). *Pourquoi : il accueille les personnages persistants à leur arrivée ([04-joueur.md](../04-joueur.md), §6) et ne doit jamais disparaître.*

### 4.10 Banques et assurances (décidé)

**Banques** (deux ou trois, quartier de la Gare, la Cité, communes, [ville.md](ville.md), §5.2) :

- tiennent les comptes, les distributeurs et la caisse en espèces ;
- accordent des **prêts** selon des règles fixes appliquées par le conseiller (un PNJ, ou un joueur qui l'incarne, qui ne peut s'en écarter qu'en commettant une fraude visible dans le grand livre) :

| Prêt | Conditions **[à calibrer]** | Taux annuel |
|---|---|---|
| Découvert autorisé | Revenus réguliers | 15 % |
| Prêt à la consommation | Mensualités ≤ 33 % des revenus ; jusqu'à 5 ans | 7 % |
| Prêt immobilier | Apport ≥ 10 %, mensualités ≤ 35 % des revenus réguliers ; jusqu'à 25 ans | 3,5 % |
| Prêt professionnel | Dossier jugé viable, apport ≥ 20 % | 5 % |

- **Où** (décidé, [interface.md](interface.md), §8.2) : un prêt, un découvert autorisé ou une renégociation se demande **en agence**, sur rendez-vous, auprès du **conseiller** ; l'application bancaire du téléphone ne fait que des **simulations** (mensualité, durée, capacité d'emprunt). *Pourquoi : c'est la pratique réelle, et le conseiller est un personnage qu'un joueur peut incarner, borné par les règles fixes du tableau.*
- **Revenus réguliers** : ceux des trois derniers mois de jeu (salaires, pensions, loyers perçus, bénéfices déclarés).
- **Épargne** : livret rémunéré à 1 % par an **[à calibrer]**.
- **Défaut de paiement** : relances, frais, inscription au fichier des incidents (plus de crédit pendant un an **[à calibrer]**), puis saisie des biens donnés en garantie (logement acheté à crédit, véhicule).
- **Signalements** : un dépôt d'espèces important, des virements inhabituels ou répétés sous un seuil déclenchent un signalement (§7.7).
- Les banques se refinancent auprès du compte extérieur « Monde financier » quand leurs réserves ne suffisent pas.

**Assurances** (agence dans la Cité) : habitation (obligatoire pour un locataire), automobile (obligatoire), responsabilité professionnelle, multirisque des commerces. Elles remboursent les dégâts moins une **franchise** (§5.8) ; elles peuvent refuser en cas de fraude soupçonnée (incendie volontaire, faux vol) et le signaler.

---

## 5. Propriété

### 5.1 Le registre (décidé)

Le monde tient un **registre des titres**, l'équivalent du cadastre et des fichiers d'immatriculation :

| Bien | Titre | Ce qu'il contient |
|---|---|---|
| Parcelle | Titre de propriété | Propriétaire, surface, constructibilité |
| Bâtiment | En propriété unique ou en **copropriété** (§5.6) | Propriétaire unique ou syndicat des copropriétaires |
| Logement, local | Lot | Propriétaire, bail en cours, locataire |
| Véhicule | Certificat d'immatriculation | Propriétaire, assurance, signalement de vol |
| Arme | Déclaration (armes de chasse) | Détenteur, permis |
| Entreprise | Parts | Propriétaires et pourcentages |

- Les **clés** sont des objets du monde liés à un droit d'accès ([ville.md](ville.md), §5.1) ; le titre est la propriété juridique. On peut avoir la clé sans le titre (locataire, voleur, ami) et le titre sans la clé (propriétaire bailleur).
- **Qui loue** (décidé, en accord avec [population.md](population.md), §3.1) :

  | Bailleur | Part des logements loués **[à calibrer]** |
  |---|---|
  | **Orvenne Habitat** (logement social) | ≈ 35 % |
  | **Propriétaires bailleurs habitants** (≈ 10 % des ménages possèdent un autre logement que le leur) | ≈ 35 % |
  | **Propriétaires non résidents** (investisseurs extérieurs, gérés par l'agence immobilière ; loyers vers le compte extérieur) | ≈ 20 % |
  | **Entreprises** (sociétés foncières, logements de fonction) | ≈ 10 % |

  *Pourquoi : des bailleurs habitants donnent des relations propriétaire-locataire réelles (relances, conflits, arrangements) ; les non-résidents fournissent un stock de biens à vendre sans dépouiller les habitants.*
- **Logés par l'employeur** (gardien, concierge, certains employés d'Orvenne Habitat) : pas de loyer, c'est un avantage lié au poste ; le logement est perdu avec l'emploi, après un délai de trois mois **[à calibrer]**.
- **Orvenne Habitat**, office public rattaché à la mairie d'Orvenne, possède le logement social (essentiellement les Peupliers, une partie des Forges) (décidé). *Pourquoi : le grand ensemble a besoin d'un bailleur identifiable, responsable de l'entretien et des attributions.*

### 5.2 La valeur des biens (décidé)

- **Immobilier** : surface × **prix au m² du quartier** (§6.3) × coefficients (type, étage, état d'entretien, dégâts, nuisances) **[à calibrer]**.
- **Véhicules** : cote d'occasion selon le modèle, l'âge et l'état.
- **Entreprise** : actif net (trésorerie + biens + stocks − dettes) + un multiple du bénéfice annuel moyen **[à calibrer]**.
- **Objets** : valeur de catalogue, dépréciée selon l'état.

Ces valeurs servent au patrimoine (§1.3), aux prêts, aux assurances, aux successions et à la taxe foncière.

### 5.3 La location (décidé)

**Trouver un logement** : annonces de l'agence immobilière (vitrine, application), annonces de particuliers (panneaux, bouche-à-oreille), Orvenne Habitat pour le logement social (dossier, liste d'attente, priorité selon les revenus et la situation familiale).

**Dossier de location** (logement privé) :

| Condition | Règle **[à calibrer]** |
|---|---|
| Revenus | Revenus réguliers ≥ 3 × le loyer |
| À défaut | Un **garant** (proche dont les revenus suffisent), ou un **dépôt de garantie majoré** (3 mois au lieu d'un), ou un loyer payé d'avance |
| Dépôt de garantie | 1 mois de loyer, rendu au départ moins les dégâts |
| Bail | 3 ans vide, 1 an meublé, reconduit tacitement ; préavis de 1 à 3 mois |

Le propriétaire (PNJ) choisit entre plusieurs candidats selon ses critères (sûreté des revenus, impression laissée, profil) ; il peut être plus ou moins exigeant ou discriminant selon sa personnalité (domaine social).

**Meublés, foyer, hôtel** : sans condition de revenus, payés d'avance (à la nuit, à la semaine ou au mois). C'est la porte d'entrée normale du personnage persistant et des nouveaux venus.

**Logement réservé à distance** (décidé). Un ménage qui va arriver ([population.md](population.md), §7.4) loue ou achète **avant son arrivée**, par l'agence immobilière, Orvenne Habitat ou un propriétaire bailleur, avec les mêmes conditions de dossier (ses revenus futurs comptent s'il a une promesse d'embauche). Le bail commence à une date fixée ; le logement est marqué **réservé** et retiré des annonces, et le propriétaire est payé du premier loyer et du dépôt de garantie dès la signature (compte extérieur « Ménages extérieurs »). À défaut, le ménage réserve des nuits à l'hôtel ou au foyer et cherche sur place. *Pourquoi : c'est ainsi qu'on déménage réellement, et le logement réservé ne peut pas être pris entre-temps par un joueur.*

**Loyer** : prélevé automatiquement chaque mois sur le compte du locataire, ou payé en main propre. Charges en plus (eau, parties communes). Révision annuelle plafonnée à l'évolution de l'indice de référence (nulle par défaut, §6.1).

**Locaux professionnels** : bail commercial de 9 ans, loyer plus élevé, mêmes mécanismes.

**Colocation** et **sous-location** (autorisée ou non par le propriétaire) existent ; la sous-location non autorisée est un motif de résiliation.

### 5.4 L'achat et la vente (décidé)

- Un propriétaire met en vente quand un projet l'y pousse (déménagement, besoin d'argent, succession, retraite) ; il fixe un prix proche de la valeur estimée (§5.2), ajusté selon son profil et son urgence.
- L'acheteur visite (accès accompagné), négocie, puis signe chez le **notaire** (cabinet aux Tilleuls), **10 jours de jeu** après l'accord (procédure avec délai, §12.2) **[à calibrer]**.
- **Frais d'acquisition** : ≈ 8 % du prix, dont les **droits de mutation** (6 %) pour la commune et les honoraires du notaire **[à calibrer]**.
- **Financement** : comptant, ou prêt immobilier (§4.10). Un bien acheté à crédit est **hypothéqué** : il peut être saisi en cas de défaut.
- Les **joueurs peuvent acheter**, avec leur personnage persistant ou un personnage incarné, aux mêmes conditions (sous réserve des garde-fous du §12).
- Les **parcelles constructibles** se vendent de la même façon ; beaucoup appartiennent à la mairie, qui les vend ou les attribue à des projets qu'elle soutient.

### 5.5 Loyers impayés et expulsion (décidé)

Règle de [ville.md](ville.md), §2.5, détaillée ici. Les délais sont en **jours de jeu** **[à calibrer]** :

| Jour après l'échéance | Étape |
|---|---|
| J+5 | Relance du propriétaire (courrier dans la boîte aux lettres, appel) |
| J+15 | Seconde relance, frais de retard |
| J+30 | **Commandement de payer** par huissier ; le garant éventuel est appelé |
| J+60 | Le propriétaire saisit le **tribunal** |
| J+90 | Jugement : **expulsion** prononcée, sauf si la dette est réglée ou un échéancier accepté |
| J+100 | **Expulsion** par l'huissier, avec la police si besoin |

- **Trêve hivernale** : aucune expulsion d'une résidence principale entre le 1ᵉʳ novembre et le 31 mars ; la procédure continue mais l'exécution attend (décidé). *Pourquoi : c'est une règle européenne familière, qui évite de jeter un habitant à la rue en plein hiver et qui suit le calendrier réel du serveur.*
- À tout moment, payer la dette (avec les frais) arrête la procédure.
- **Après l'expulsion** : les serrures sont changées ; les clés de l'ancien locataire ne marchent plus ; ses affaires sont placées en **garde-meubles** (à ses frais) pendant 30 jours **[à calibrer]**, puis vendues ou jetées. Pour un personnage persistant, le logement cesse d'être un lieu de retrait : il réapparaît **à la gare** ([02-personnages.md](../02-personnages.md), §1.1).
- Une **dette locative** non réglée reste due ; elle peut être saisie sur les revenus futurs (saisie sur salaire par tranches, avec un reste à vivre protégé, §7.8).
- Un PNJ expulsé cherche un logement moins cher, se fait héberger par un proche, va au foyer, ou se retrouve **sans domicile** (centre social, squat) selon ses ressources et ses relations. Les aides (§8.3) et le logement social limitent ce cas.

### 5.6 Copropriété, charges et entretien (décidé)

- Dans un immeuble en copropriété, les **parties communes** (escaliers, toit, façade, cave, ascenseur) appartiennent au **syndicat des copropriétaires**, géré par un **syndic** (l'agence immobilière en général). Chaque propriétaire paie des **charges** selon sa part.
- Les gros travaux (ravalement, toiture, ascenseur) sont votés en assemblée annuelle, financés par un appel de fonds.
- L'**entretien** courant de chaque bien coûte ≈ 1 % de sa valeur par an **[à calibrer]**. Un propriétaire qui n'entretient pas laisse l'**état d'entretien** se dégrader ([ville.md](ville.md), §4.4 et §6.3), ce qui fait baisser la valeur et les loyers du bien et, à la longue, l'image du quartier.

### 5.7 Constructions et rénovations (décidé)

Le déroulé (projet, permis, chantier) est dans [ville.md](ville.md), §6.2. Ce domaine fixe le **financement** :

1. **Devis** de l'entreprise de bâtiment : ≈ 1 800 écus le m² en construction neuve, 600 à 1 200 en rénovation lourde **[à calibrer]**, selon le quartier et la typologie.
2. **Financement** du porteur : fonds propres, prêt bancaire, subvention de la mairie (rénovation de façades, logement social), ou capital extérieur pour un promoteur.
3. **Paiement par étapes** : 30 % à la commande, 40 % au gros œuvre, 30 % à la réception. Un porteur qui ne paie pas une étape arrête le chantier (palissade, chantier abandonné).
4. **Travail réel** : l'entreprise achète ses matériaux (livrés par camion ou par péniche au port) et paie ses ouvriers ; son chiffre d'affaires vient du chantier.
5. **Fin** : le bâtiment entre au registre ; le porteur le vend, le loue ou l'occupe.

**Construction de résidences quand la demande monte** ([ville.md](ville.md), §2.5) : quand le **taux de vacance des logements** de l'agglomération reste sous **2 %** pendant un mois de jeu, ou que l'indice des loyers monte de plus de **10 %** en trois mois **[à calibrer]**, un **promoteur** (une société de l'agglomération, Orvenne Habitat ou un investisseur extérieur) lance un projet de résidence sur une parcelle constructible : immeuble de studios et de T2, ou résidence meublée près de la gare. *Pourquoi : la demande des personnages persistants doit se traduire par plus de logements, pas par l'éviction des habitants.*

### 5.8 Dégâts, réparations et assurances (décidé)

Les dégâts persistent jusqu'à réparation ([ville.md](ville.md), §6.3). Qui paie :

| Situation | Qui paie |
|---|---|
| Usure, vétusté | Le propriétaire |
| Dégât causé par un locataire | Le locataire (retenu sur le dépôt de garantie au départ, ou facturé) |
| Vandalisme, cambriolage, incendie accidentel, dégât des eaux, crue | L'**assurance** du propriétaire ou de l'occupant, moins la franchise (≈ 150) ; sans assurance, le propriétaire |
| Auteur identifié | L'assureur se retourne contre lui ; la justice peut le condamner à rembourser |
| Espace public (lampadaire, mobilier urbain, voirie) | La commune ; elle peut poursuivre l'auteur |

Coûts indicatifs **[à calibrer]** : vitre 150, serrure 120, porte d'entrée 600, tag nettoyé 200, carrosserie légère 400, incendie d'un local 20 000 à 100 000.

Une réparation est une **commande** passée à un artisan (vitrier, serrurier, garagiste, entreprise de bâtiment) ; elle crée des tâches et du chiffre d'affaires. Un propriétaire sans argent ni assurance laisse le dégât en place.

**Primes d'assurance** (décidé, [combat.md](combat.md), §8.6) **[à calibrer]** :

```
prime = prime de base (§1.4, selon le bien ou le véhicule)
        × coefficient de quartier   (cambriolages, vols de véhicules, incendies et dégradations
                                     des 12 derniers mois rapportés à la moyenne de l'agglomération ;
                                     borné à 0,8 – 1,8 ; recalculé chaque année)
        × bonus-malus de l'assuré   (−5 % par année sans sinistre, +25 % par sinistre responsable ;
                                     borné à 0,5 – 2)
```

- Un quartier où la criminalité monte voit ses primes monter à l'échéance suivante, ce qui pèse sur les commerces et les ménages ; la baisse est tout aussi lente.
- Au-delà d'un malus de 2 ou après une fraude prouvée, l'assureur **résilie** : l'assuré doit trouver un autre contrat, plus cher.
- **Recours** : l'assureur qui indemnise devient **créancier de l'auteur** identifié et recouvre par le mécanisme du §7.8.

*Pourquoi : la criminalité d'un quartier doit coûter à tous ses habitants, comme dans la réalité, ce qui donne un intérêt économique à la sécurité.*

### 5.9 Les véhicules (décidé)

- Achetés neufs (concession aux Forges **[à calibrer]**) ou d'occasion (garages, particuliers). Immatriculation et assurance obligatoires.
- **Coûts d'usage** : carburant (§1.4), entretien et pannes (probabilité selon l'âge et l'entretien), réparations, stationnement (gratuit dans la plupart des rues, payant dans la Cité et le quartier de la Gare, abonnement au parking souterrain) **[à calibrer]**.
- **Amendes** de stationnement et **fourrière** (frais de garde par jour) : un véhicule abandonné finit vendu aux enchères.
- **Entreprises** : utilitaires, camions, taxis, bus (régie), véhicules de service.
- **Vol** : un véhicule volé est signalé au registre ; il est revendu tel quel, maquillé ou démonté pour les pièces (§7.4).
- **Modèles génériques par catégorie, sans marque** (décidé, [direction-artistique.md](direction-artistique.md), §1.4 et §12.8). Le prix d'un véhicule dépend de sa **catégorie**, de son **âge** et de son **état** ; la catégorie est l'une de celles-ci : vélo, scooter, moto, citadine, berline, break, monospace, utilitaire, camion. Il n'y a ni marque ni modèle.
  - Les **gammes** (entrée, moyenne, haut de gamme) jouent le rôle des marques pour le prix. Elles se voient seulement par l'état, la finition et les options que la direction artistique dessine.
  - La concession et les garages portent des **enseignes fictives** générées.

  Le prix neuf d'une citadine d'entrée de gamme est d'≈ 15 000, celui d'une berline haut de gamme d'≈ 45 000. La décote est d'≈ 15 % la première année, puis d'≈ 10 % par an **[à calibrer]**. *Pourquoi : la fiction n'a pas de marques réelles, et la catégorie, l'âge et l'état suffisent à exprimer la richesse d'un propriétaire et la valeur d'un vol.*

### 5.10 Les objets de valeur (décidé)

- Un objet d'une valeur supérieure à **50 écus** **[à calibrer]** porte un **propriétaire légitime** dans le cœur, et éventuellement un **signalement de vol** si le propriétaire a porté plainte.
- Les objets de moindre valeur n'ont pas de propriétaire suivi : celui qui les a les possède.
- La possession d'un objet volé n'est pas visible : seul celui qui vérifie (police, prêteur sur gages scrupuleux) peut constater qu'il est signalé.

*Pourquoi : on suit ce qui compte pour le vol et le recel sans enregistrer chaque fourchette.*

**Valeur affective** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), A15). Un objet peut porter l'étiquette **valeur affective** : une alliance, un souvenir de famille, une photo encadrée, une montre ou un bijou hérité, la médaille d'un parent.
- **Propriétaire suivi** : l'étiquette donne au cœur un propriétaire suivi, **quel que soit le prix** de l'objet, même sous le seuil de 50 écus.
- **Hors du grand livre** : l'étiquette ne change pas la valeur marchande. Un receleur paie le prix du marché ; un prêteur sur gages prête sur ce prix.
- **Signature** : voler ou détruire un tel objet ajoute un effet de **peine causée** (0,4) envers le propriétaire.
- **Réaction du propriétaire** : il porte plainte plus souvent, cherche plus longtemps, et peut racheter son objet bien au-dessus de sa valeur.
- **Origine de l'étiquette** : elle est posée à la génération pour les héritages et les cadeaux marquants ([population.md](population.md)), et en jeu par un événement de vie : alliance au mariage, objet reçu d'un défunt (§5.11).

*Pourquoi : la valeur affective n'est pas dans le grand livre, mais voler l'alliance d'une veuve n'est pas voler un objet de même prix ; l'étiquette porte sur le type d'objet et vaut pour tout acte qui le prend ou le détruit.*

**Céder son propre objet de valeur affective** (décidé, A15 bis, clôture d'équilibrage C4). Le propriétaire qui **cède ou jette** son propre objet de valeur affective reçoit un effet de **peine causée envers soi** de 0,4 **[à calibrer]**. Cela couvre la vente, la mise en gage chez le prêteur sur gages (§7.5), le don et l'abandon. La règle s'applique au même type d'objet que A15, pour l'IA comme pour le joueur. *Pourquoi : vendre son alliance au juste prix laisse le grand livre équilibré, mais ce n'est pas un acte neutre ; sans cette règle, un joueur pourrait brader les souvenirs d'un personnage sans aucun coût, et l'IA les céderait sans hésiter.*

### 5.11 Successions (décidé)

À la mort d'un habitant (mort définitive d'un personnage jouable, mort naturelle) :

- ses biens vont à ses **héritiers** (conjoint, enfants, puis famille), après **droits de succession** progressifs versés à l'État (exonération sous 100 000 par enfant, puis 20 à 45 %) **[à calibrer]** ;
- un enfant héritier mineur a ses biens gérés par son tuteur jusqu'à 18 ans ;
- sans héritier dans l'agglomération, les biens sont vendus et le produit part vers l'extérieur (héritiers à l'extérieur ou État) ;
- une **entreprise** est reprise par un héritier, vendue, ou fermée (§4.7) ;
- les **dettes** sont payées sur la succession avant le partage ;
- **indignité successorale** (décidé, revue d'équilibrage E31) : un héritier **condamné pour la mort du défunt**, ou pour **complicité**, est exclu de la succession ; sa part va aux héritiers suivants. Si la condamnation survient après le partage, il restitue ce qu'il a reçu (créance au rang des victimes, §7.8). *Pourquoi : c'est la règle de tous les droits européens, et sans elle tuer un parent riche, ou le faire tuer par un complice, serait un moyen de s'enrichir.*

**Frais d'obsèques** : ≈ 3 500 écus **[à calibrer]**, payés par la succession, à défaut par la famille ; la commune prend en charge des obsèques simples pour une personne sans ressources ni famille. Les pompes funèbres sont un commerce (fiche de métier : conseiller funéraire, porteur).

**Héritages venus de l'extérieur** (décidé). Quand un **proche à l'extérieur** meurt ([population.md](population.md), §8.2), l'habitant peut hériter : le montant est tiré selon le lien (parent, grand-parent, oncle sans enfant) et la **classe de patrimoine** de la famille (§1.6), et versé par le compte « Ménages extérieurs » quelques semaines après le décès **[à calibrer]**. Les héritages sont soumis aux mêmes droits de succession. *Pourquoi : la plupart des héritages d'une ville viennent de parents qui vivent ailleurs, et c'est une source réaliste d'enrichissement soudain et de projets.*

Les successions sont un moteur important de **projets** (héritage, conflits familiaux, reprise d'un commerce) et de changements de palier.

### 5.12 Séparation et divorce (décidé)

La rupture est décidée par le domaine social ([social.md](social.md), §6.2) ; ce domaine fixe le partage :

| Situation du couple | Partage des biens |
|---|---|
| **Mariés** (régime par défaut : communauté réduite aux acquêts) | Les biens acquis pendant le mariage sont partagés **par moitié** ; chacun garde ce qu'il possédait avant ou a reçu par héritage ou don |
| **Pacsés, concubins** | Chacun garde ses biens ; les biens achetés ensemble (en **indivision**) sont partagés selon la part payée par chacun |
| **Contrat de mariage** (séparation de biens, rare, plus fréquent chez les aisés) | Comme les pacsés |

- **Logement commun** : s'il est loué à deux noms, l'un reste et reprend le bail, l'autre part ; s'il est possédé, l'un **rachète** la part de l'autre ou le bien est **vendu** et le produit partagé.
- **Comptes joints** et **dettes communes** sont partagés par moitié ; un prêt immobilier commun suit le logement.
- **Enfants** : le parent qui n'a pas la garde principale verse une **pension alimentaire** d'≈ 10 % de son revenu net par enfant **[à calibrer]** ; en garde alternée, pas de pension sauf grand écart de revenus.
- **Durée** : un divorce par consentement mutuel prend ≈ 1 mois de jeu, un divorce contesté 3 à 6 mois **[à calibrer]** ; c'est une décision lourde avec délai (§12.2).
- Une entreprise qui appartient au couple est partagée comme un autre bien : rachat des parts, vente, ou association maintenue (rare).

*Pourquoi : quelques règles inspirées du droit européen courant suffisent à donner aux séparations leurs vraies conséquences matérielles (déménagement, appauvrissement, conflits), qui nourrissent les projets.*

### 5.13 Vêtements, coiffeur et laverie (décidé, v0.7)

La direction artistique donne à chaque habitant une **garde-robe abstraite** de pièces sémantiques ([direction-artistique.md](direction-artistique.md), §3.3 et §12.8). Ce domaine en fixe le coût.

**Choix : une garde-robe agrégée, achetée pièce par pièce en boutique** (décidé). Pour l'économie, la garde-robe est **une seule valeur** par personnage :
- sa **valeur de remplacement**, c'est-à-dire ce que coûterait toute la garde-robe neuve dans la gamme du personnage ;
- son **usure**, de 0 à 100 %.

Les pièces elles-mêmes, avec leur couleur et leur état visible, restent celles de la direction artistique.

**Règles de la garde-robe** (décidé, clôture d'équilibrage C3 et d'architecture S-7) :
- **Pièces de prix** : une pièce dont le prix dépasse le seuil des objets de valeur (50 écus, §5.10) est un **objet de valeur suivi**, avec propriétaire et prix (un manteau de laine, un costume, des chaussures de luxe). Elle peut être volée, revendue, mise en gage et donnée comme tout objet de valeur. Les autres pièces n'ont ni propriétaire suivi ni prix enregistré.
- **Vêtement offert** : offrir un vêtement est un **don de son prix** (§12.1), impossible entre personnages liés (§12.3) et compté dans les plafonds du cercle (§12.4).
- **Pièces abstraites** : elles **ne se revendent pas**. Seules les pièces suivies peuvent se revendre.
- **Ce qui fait foi** : c'est l'**usure agrégée**. L'état visible des pièces (propre, sale, mouillé, déchiré) n'est qu'un état **passager** de présentation : il ne compte ni dans le patrimoine ni dans une écriture. Seuls ses effets durables (déchirure, tache définitive) passent dans l'usure.
- **Achat sans besoin** : quand l'usure est à 0, un achat de plus par un personnage incarné compte comme un **excès envers soi** (§12.1) ; au-delà de la valeur de remplacement de sa garde-robe sur 30 jours, il n'est pas proposé.

*Pourquoi : une garde-robe aisée vaut 15 000 écus, et ses pièces chères ne doivent pas faire sortir de la valeur sans écriture ; ne suivre que ces pièces garde le coût de l'agrégat pour tout le reste.*
- **Usure** : elle avance avec le temps, d'≈ 30 % par an en moyenne **[à calibrer]**. Elle va plus vite avec le travail physique, les intempéries, les bagarres et les enfants qui grandissent. Une pièce déchirée ou tachée pour de bon augmente l'usure d'un coup.
- **Achat** : au-delà d'un seuil d'usure propre au personnage (son soin, son budget, son métier), le projet « acheter des vêtements » apparaît ([ia-pnj.md](ia-pnj.md), §2.6). En **simulation complète**, l'achat se fait **pièce par pièce** en boutique, au prix affiché de sa gamme (§1.4, §6.2) ; le joueur choisit ses pièces dans le catalogue de la boutique. Chaque achat réduit l'usure en proportion de sa valeur, et ajoute ou remplace des pièces de la garde-robe abstraite. En **statistique**, l'achat est un montant agrégé pris sur le poste « habillement » du bloc d'agenda (§9.3).
- **Valeurs de départ par classe** (valeur de remplacement par adulte) **[à calibrer]** :

  | Classe | Valeur de remplacement par adulte |
  |---|---|
  | R1 | ≈ 600 |
  | R3 | ≈ 1 500 |
  | R5 | ≈ 4 000 |
  | Aisée | ≈ 15 000 |

  Un enfant compte la moitié de la valeur d'un adulte de son ménage, et grandit plus vite que ses vêtements ne s'usent. Ces valeurs donnent une dépense de vêtements d'≈ 3 % du revenu ; avec le coiffeur, on retrouve le poste du budget type (§1.5).
- **Uniformes et tenues de travail** : ils sont fournis par l'employeur et comptés dans ses charges. Ils ne sortent pas du budget du ménage.
- **Coiffeur** : 20 à 35 la coupe, toutes les 4 à 10 semaines selon le personnage ([ia-pnj.md](ia-pnj.md), §2.6). Le salon est choisi comme tout commerce (§4.4). La coupe faite à la maison ne coûte rien.
- **Laverie** : un ménage sans lave-linge lave à la laverie, 4 le lavage et 2 le séchage, une à deux fois par semaine. C'est le cas des studios, du foyer de travailleurs et d'une partie du parc social. Un lave-linge coûte ≈ 400 et dure ≈ 8 ans **[à calibrer]**. Avoir ou non un lave-linge est tiré par classe à la génération.

*Pourquoi agrégée : tenir un prix et un propriétaire pour chacune des ≈ 300 000 pièces des 10 000 habitants ne servirait à rien ; seul le vol d'une pièce de valeur intéresse le cœur, et il passe par les objets de valeur (§5.10). Une valeur et une usure par personnage suffisent pour le budget, le projet d'achat et ce que les autres lisent de sa tenue. Pourquoi acheter par pièce en boutique : le passage en caisse est ce que le joueur voit et fait, et la boutique de vêtements doit vivre comme les autres commerces.*

---

## 6. Marché et prix

### 6.1 Le prix de référence (décidé)

Chaque **catégorie de marchandise ou de service** a un **prix de référence** : le prix « national », celui auquel les commerces achètent en gros (prix de gros de référence) et autour duquel ils vendent (prix de détail de référence). Les valeurs de départ sont celles du §1.4.

**Pas d'inflation tendancielle par défaut** (décidé) : les prix de référence ne dérivent pas avec les années ; ils varient seulement avec les **saisons**, la **conjoncture** et des **chocs** ponctuels (§6.5 à §6.7). Les salaires de référence non plus. *Pourquoi : une inflation réelle de 2 % par an n'apporte rien au jeu, brouille les repères des joueurs sur un monde qui dure des années, et compliquerait la stabilité. Un réglage permet de l'activer pour une simulation particulière.*

### 6.2 La formation des prix locaux (décidé)

Le prix d'un article dans un commerce est fixé par son **gérant**, à la clôture (§4.2) :

```
prix = prix de référence
       × coefficient de l'enseigne     (hypermarché 0,90 · supérette 1,00 · épicerie de nuit 1,20 · Hautevigne 1,15)
       × facteur offre/demande         (stock et ventes récentes du commerce, borné à 0,80 – 1,30)
       × facteur saison et météo       (§6.6)
       × style du gérant               (profil : généreux, âpre au gain ; borné à 0,95 – 1,10)
```

Toutes les bornes sont **[à calibrer]**. Conséquences :

- un commerce en rupture ou très demandé monte ses prix, dans la limite de la borne ; un commerce qui écoule mal brade ;
- la **concurrence** joue par les clients : un commerce trop cher perd ses clients au profit d'un autre (choix de recette de l'IA), ce qui fait baisser son facteur de demande ;
- comme les commerces achètent au **prix de gros de référence**, aucun prix local ne peut s'envoler durablement : si un prix reste haut, une opportunité apparaît (§4.5) et un concurrent ouvre.

Les **services** (coiffeur, garagiste, plombier) suivent la même formule sur un tarif horaire de référence. Les prix des **services publics** (bus, piscine, cantine) sont fixés par la commune (§8.4).

**Prix affichés** (décidé, [interface.md](interface.md), §8.5) :

- chaque article en rayon ou en vitrine porte une **étiquette** ; un bar ou un restaurant a une **carte** ou une ardoise ; un service a un **tarif** affiché au comptoir (prix à l'heure, forfaits courants) ;
- le prix calculé par la formule est **arrondi** à un prix d'affichage lisible (au 0,10 près sous 10 écus, à l'écu ou en « ,90 » au-dessus, selon le style du commerce) **[à calibrer]** ;
- le prix affiché ne change **qu'à la clôture** (§4.2), jamais en cours de journée, sauf démarque des invendus le soir, soldes annoncées et marchandage (marché, occasion) ;
- le client paie le prix affiché ; il reçoit un **ticket** (objet du monde, qui sert de preuve d'achat face à un soupçon de vol).

*Pourquoi : des prix stables dans la journée et arrondis comme dans un vrai commerce restent lisibles pour le joueur, sans renoncer à la formation des prix.*

Les **marchandages** (marché, occasion, économie parallèle) partent de ce prix ; leur résultat dépend des compétences de négociation et des profils (domaine social).

**Prix d'ami, fidélité et crédit** (décidé, [social.md](social.md), §5.7) :

- un commerçant peut faire une **remise** à un ami ou à un **habitué** (client venu souvent ces dernières semaines) : jusqu'à −15 % **[à calibrer]**, selon l'affection, la fréquence des visites et sa générosité ; ou offrir un verre, un pain ;
- les **habitudes** pèsent dans le choix du commerce par l'IA (§4.4) : un client fidèle ne part pas à la première hausse de prix ; un commerçant aimable et des prix d'ami entretiennent cette fidélité ;
- le **crédit chez le commerçant** (l'ardoise au bar, à l'épicerie) est accordé aux habitués en qui il a confiance, jusqu'à un plafond **[à calibrer]** ; c'est une dette, donc un **engagement** du client ([ia-pnj.md](ia-pnj.md), §4.2) ;
- les **prêts entre particuliers** se font selon la confiance et la réputation de l'emprunteur ; ils sont des engagements, avec ou sans reconnaissance de dette écrite.

*Pourquoi : ces petites faveurs sont la monnaie des relations de quartier et donnent une valeur économique à la réputation. Entre personnages liés à un même joueur, la borne du §12.3 s'applique toujours.*

Les **banques**, en revanche, appliquent des règles fixes (§4.10) : la réputation n'y compte qu'à travers l'historique bancaire. *Pourquoi : c'est le verrou contre l'abus d'un banquier incarné (§12.5).*

### 6.3 L'immobilier par quartier (décidé)

Chaque quartier a un **indice des prix au m²** (vente) et un **indice des loyers**, mis à jour **chaque mois de jeu** et lissés (variation de quelques pour cent par mois au plus **[à calibrer]**) à partir de :

| Facteur | Effet |
|---|---|
| Taux de vacance du quartier | Peu de logements libres → hausse |
| Transactions récentes | Prix réellement payés |
| État d'entretien moyen ([ville.md](ville.md), §6.3) | Façades dégradées, tags, déchets → baisse |
| Sécurité perçue ([ville.md](ville.md), §9.1) | Incidents récents (agressions, cambriolages, trafics visibles) → baisse |
| Services proches | Ouverture ou fermeture de commerces, d'une école → hausse ou baisse |
| Nuisances | Bar bruyant, chantier, friche → baisse locale |

*Pourquoi : l'immobilier devient le miroir de la vie du quartier, et ce que font les joueurs (violence, rénovations, ouverture d'un commerce) se lit dans les prix.*

**Valeurs de départ par quartier** (décidé ; *pourquoi : elles suivent l'identité de chaque quartier fixée par la ville, [ville.md](ville.md), §3.1*) **[à calibrer]** :

| Quartier | Prix de vente au m² | Loyer privé au m² par mois | Remarque |
|---|---|---|---|
| La Cité (vieux centre) | 3 300 | 13 | Petits logements anciens, très demandés |
| Quartier de la Gare | 2 800 | 12 | Surtout des bureaux ; studios et résidence près de la gare |
| Les Tilleuls (bourgeois) | 3 500 | 13 | Grands appartements anciens |
| Beaulieu (hôpital et campus) | 2 600 | 12,5 | Studios étudiants, chambres en résidence ≈ 350 par mois |
| Faubourg Saint-Roch | 2 100 | 10 | Petits immeubles, en lente montée |
| Les Quais | 1 500 | 8 | Peu de logements, entrepôts et friches bon marché |
| Les Peupliers (grand ensemble) | 1 400 | 8 (social : ≈ 6) | Surtout Orvenne Habitat |
| Hautevigne | 4 500 | 14 | Villas, terrains chers |
| Forges-sur-Ostrelle | 1 700 | 8,5 | Pavillons modestes, cités ouvrières, logement social |

Les **locaux commerciaux** se louent ≈ 1,5 × le loyer au m² d'habitation du quartier, les **parcelles constructibles** se vendent ≈ 30 % du prix au m² bâti du quartier par m² de terrain.

**Usage à la génération** (décidé, [revues/generation.md](../revues/generation.md), P19). La génération loge les ménages par un **flot à coût minimal** ; ces valeurs de départ y servent à calculer le **taux d'effort** de chaque affectation possible.
- **Calcul** : le taux d'effort est le loyer, ou la mensualité de prêt pour un propriétaire, divisé par le revenu net du ménage.
- **Taux cible** : ≈ 25 % pour un locataire du parc social, ≈ 30 % pour un locataire du parc privé, ≈ 30 % pour un propriétaire qui rembourse un prêt. Un propriétaire sans prêt paie seulement ses charges, et son taux d'effort compte peu.
- **Coût d'une affectation** : il croît avec l'écart à ce taux cible, fortement au-delà de **40 %**. C'est le plafond du contrôle de soutenabilité du §1.6, à part les quelques ménages en difficulté semés volontairement.

Toutes ces valeurs sont **[à calibrer]**. Le coût est calculé en **entiers** (centimes et points de pour mille), pour que l'affectation soit reproductible. *Pourquoi : les loyers de départ et les revenus des ménages doivent se correspondre dès la génération ; sinon, le premier mois de jeu produirait une vague d'impayés et d'expulsions.*

### 6.4 Les salaires (décidé)

Chaque métier a un **salaire de référence** (§1.4). L'employeur propose un salaire dans la fourchette selon la compétence du candidat et la **tension** du métier : un métier où les offres restent longtemps sans candidat voit son salaire proposé monter, jusqu'à **+15 %** **[à calibrer]** ; un métier où les candidats abondent reste au bas de la fourchette. Le salaire minimum est un plancher légal (sauf au noir).

### 6.5 Les marchandises venues de l'extérieur (décidé)

- Presque toutes les marchandises sont **importées** : elles arrivent par **camion** (plateforme logistique, livraisons directes), par **péniche** (port : matériaux, matières de l'usine, carburant) ou par **train de marchandises** (usine) ([ville.md](ville.md), §7.4). L'argent part vers le compte extérieur « Fournisseurs ».
- La **plateforme logistique** est le grossiste de la ville : elle stocke et redistribue ; une grève, une panne ou une absence massive de ses chauffeurs se traduit par des **retards de livraison** et des **ruptures** dans les commerces.
- **Perturbations** **[fréquences à calibrer]** : crue de l'Ostrelle (port et navette arrêtés, livraisons fluviales suspendues), neige et verglas (camions retardés), grève des transporteurs, pénurie de carburant. Elles font monter le facteur offre/demande des commerces touchés et peuvent provoquer des achats de précaution par les habitants.
- **Production locale** : pain, repas, prestations, et l'agriculture des environs (maraîchers au marché hebdomadaire, fruits en saison), à des prix proches de la référence.

### 6.6 Saisons et météo (décidé)

Chaque catégorie de marchandise et chaque activité a un **profil saisonnier** et une **sensibilité à la météo** **[à calibrer]**, comme les activités de l'IA ([ia-pnj.md](ia-pnj.md), §2.2). Ils modulent la **demande** (donc les ventes et, à la marge, les prix) et les **coûts**.

| Saison ou météo | Effets |
|---|---|
| **Hiver** | Chauffage : la facture d'énergie double ; demande de vêtements chauds ; terrasses fermées, bars plus fréquentés à l'intérieur ; déneigement et salage (emplois municipaux, missions) ; fêtes de fin d'année (renforts dans les commerces, CDD, ventes en hausse en décembre) |
| **Printemps** | Jardinage, travaux de rénovation, réouverture des terrasses |
| **Été** | Terrasses, glaces, boissons fraîches ; **jobs d'été** des adolescents et étudiants ; vacances (départs à l'extérieur, commerces à horaires réduits en août) ; fruits et légumes locaux moins chers |
| **Automne** | Rentrée scolaire (fournitures, inscriptions), reprise de l'activité |
| **Pluie** | Moins de clients dans les commerces de rue et les terrasses ; plus de bus et de taxis ; plus de livraisons à domicile |
| **Canicule** | Boissons, ventilateurs ; moins d'activité physique en journée ; malaises (pharmacie, hôpital) |
| **Gel, neige** | Accidents (garages, carrossiers, dépanneurs), canalisations gelées (plombiers), retards de livraison |
| **Crue** | Caves inondées (réparations), port à l'arrêt, chômage technique |

L'**énergie** a un prix unitaire fixe ; c'est la **consommation** qui varie avec la saison (décidé). Le **carburant** varie avec la conjoncture (±10 % par mois au plus **[à calibrer]**). *Pourquoi : c'est le comportement réel des factures des ménages, et cela reste simple.*

### 6.7 La conjoncture (décidé)

Une variable **conjoncture extérieure** suit une marche aléatoire lente et **bornée** (expansion ↔ récession), sur plusieurs mois réels **[à calibrer]**. Elle agit sur :

- les **commandes des secteurs exportateurs** (usine, logistique, port, hôtel) ;
- le prix du **carburant** et de certaines matières ;
- marginalement, le recrutement et les salaires proposés.

S'y ajoutent de rares **événements économiques** tirés au sort : perte d'un gros client de l'usine (chômage partiel, plan social), nouvelle commande importante (embauches), fermeture d'un fournisseur.

*Pourquoi : ces mouvements créent des histoires (licenciements, embauches, inquiétude) sans faire dériver l'économie, puisque la variable revient toujours vers son centre.*

### 6.8 Le budget du personnage dans les décisions de l'IA (décidé)

Les prix influencent le choix des recettes de l'IA ([ia-pnj.md](ia-pnj.md), §3.2 et §3.3). Ce domaine fournit :

- le **disponible** du personnage : solde des comptes et espèces, moins une **réserve** pour les charges fixes à venir d'ici le prochain revenu (loyer, factures) ;
- la **propension à dépenser** : selon le profil (un « Perception » impulsif dépense plus facilement qu'un « Jugement » prévoyant), la motivation argent et le revenu ;
- le **coût relatif** d'une recette : prix rapporté au disponible. Un restaurant à 15 écus est anodin pour un cadre, coûteux pour un chômeur en fin de mois.

Ce coût relatif entre dans la **faisabilité** du score d'utilité : un personnage à court d'argent cuisine chez lui, prend le bus plutôt que le taxi, et, si son profil le permet ou si son besoin devient critique, envisage un moyen illégal ([ia-pnj.md](ia-pnj.md), §1.4).

---

## 7. Économie parallèle

### 7.1 Principes (décidé)

- L'économie parallèle utilise **les mêmes mécanismes** que l'économie légale : entreprises de forme **clandestine** (un réseau a un chef, des employés, des stocks, une trésorerie en espèces, des clients), prix, stocks, livraisons.
- Ses écritures sont marquées **non déclarées** : elles passent en **espèces**, n'apparaissent ni à la banque ni aux impôts, et ne laissent de trace que dans la **mémoire des témoins** et des participants.
- Elle obéit aux mêmes contraintes de fidélité : un personnage n'y participe que si son profil le permet (respect des règles, honnêteté, violence), ou poussé par un besoin critique ([ia-pnj.md](ia-pnj.md), §1.4).
- La **répression** (enquêtes, arrestations, peines) relève du domaine **combat, police et justice** ; ce domaine-ci fournit les traces et les montants.

*Pourquoi : réutiliser le modèle d'entreprise évite un second système, et l'argent liquide rend naturellement ces flux invisibles aux institutions mais pas aux témoins.*

### 7.2 Travail au noir (décidé)

- **Où** : bâtiment, restauration, ménage, déménagement, jardinage, garages.
- **Pourquoi l'employeur le fait** : il économise les cotisations (le coût du travail passe de 1,6 × à 1 × le net) et la paperasse.
- **Pour le travailleur** : un paiement en espèces immédiat, souvent un peu au-dessus du net déclaré, mais **aucune protection** (pas de chômage, pas de retraite, pas d'assurance en cas d'accident) et aucun recours si l'employeur ne paie pas.
- **Risques** : contrôle de l'**inspection du travail** (fiche de métier, domaine combat), amende pour l'employeur, redressement des cotisations, dénonciation par un employé mécontent ou un concurrent.
- Le personnage persistant peut y avoir accès par relation (missions non déclarées proposées de bouche à oreille, jamais sur le tableau officiel).

### 7.3 Trafics (décidé)

| Trafic | Chaîne | Prix de rue indicatifs **[à calibrer]** |
|---|---|---|
| **Stupéfiants** | Fournisseur extérieur (compte « Fournisseurs », non déclaré) → chef de réseau (stock, trésorerie) → revendeurs aux **points de deal** → consommateurs | Cannabis 10 le gramme, cocaïne 70 le gramme |
| **Cigarettes de contrebande** | Extérieur → revendeurs de rue, épiceries complices | 6 le paquet (moitié du prix légal) |
| **Armes** | Extérieur, vols chez les chasseurs, armurerie complice → quelques intermédiaires | Pistolet 1 500 à 3 000, rare ; fusil de chasse volé 400 |
| **Pièces et véhicules volés** | Voleurs → garages complices (démontage, maquillage) → revente | 20 à 40 % de la valeur |

- **Degré de nocivité** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), D16). Chaque catégorie de produit vendu porte un **degré de nocivité**, une colonne de la table des marchandises (principe 7). Vendre ou fournir un produit nocif donne au vendeur un effet de **mise en danger** de l'acheteur, de magnitude égale à ce degré **[à calibrer]**. Les valeurs de départ sont :

  | Produit | Degré de nocivité |
  |---|---|
  | Tabac | 0,05 |
  | Cannabis | 0,15 |
  | Cocaïne | 0,4 |
  | Alcool vendu à un adulte sobre | 0 |
  | Alcool servi à un client déjà ivre | 0,1 |
  | Tous les autres produits | 0 |

  La **vente d'alcool ou de tabac à un mineur** n'est pas proposée : c'est une règle absolue. La signature vient de l'acte de vente lui-même ; elle est la même pour le buraliste, pour le serveur et pour le revendeur de rue. Ce qui distingue le revendeur est la **violation de la loi**, que la vente de stupéfiants ajoute. *Pourquoi : un dealer qui croit ne faire de tort à personne doit tout de même porter le poids de ce qu'il vend, et le même barème s'applique, à plus petite échelle, aux commerces légaux.*
- La **demande** vient de vrais consommateurs : des PNJ qui ont une **dépendance** (tabac, alcool, cannabis, cocaïne), une habitude festive, ou un besoin (une arme pour se protéger ou pour un projet). Les dépendances sont un ajout demandé au socle et à l'IA (§14).
- Les **points de deal** occupent des lieux peu surveillés fournis par la ville ([ville.md](ville.md), §5.3) ; leur choix et leur **territoire** relèvent du domaine social (bandes) ; les conflits pour un territoire relèvent du domaine combat.
- Le chiffre d'affaires d'un réseau est réel : il paie ses guetteurs et revendeurs en espèces, rembourse ses fournisseurs, et son chef s'enrichit (composantes **Moyens** et parfois **Richesse**, [02-personnages.md](../02-personnages.md), §3.2).

### 7.4 Vol et recel (décidé)

- **Ce qui se vole** : espèces, objets de valeur, marchandises (stock d'un commerce, camion), véhicules, pièces.
- **Recel** : le voleur revend à un **receleur** à **20 à 30 %** de la valeur ; le receleur revend à **40 à 60 %** à des acheteurs peu regardants, à un prêteur sur gages peu scrupuleux, ou à l'extérieur **[à calibrer]**.
- Un objet volé garde son **signalement** (§5.10) : le vendre à un commerce honnête, ou le porter devant un policier qui vérifie, est risqué.
- Les **marchandises sans signalement** (stocks agrégés, carburant) sont faciles à écouler mais difficiles à transporter en grande quantité.

### 7.5 Le prêteur sur gages (décidé)

Le prêteur sur gages est un **commerce légal** de la ville ([ville.md](ville.md), §5.3) :

- il prête **30 à 60 %** de la valeur estimée d'un objet déposé en gage, pour **3 mois** renouvelables, à un taux de **≈ 3 % par mois** **[à calibrer]** ;
- si l'emprunteur ne rembourse pas, l'objet est **vendu** (en boutique ou aux enchères) ;
- il **doit** vérifier l'identité du déposant et, s'il le veut, le signalement de l'objet. Un prêteur honnête refuse un objet signalé et prévient la police ; un prêteur peu scrupuleux le prend à prix cassé et devient un **receleur** de fait.

C'est un lieu clé pour les personnages en difficulté (fin de mois, dette urgente) et pour les voleurs.

### 7.6 Usure, racket et paiements occultes (décidé)

- **Usurier** : un prêteur illégal, en espèces, sans dossier, à des taux énormes (≈ 10 % par semaine **[à calibrer]**). Le remboursement est un **engagement** du débiteur ; le défaut entraîne des menaces, puis des violences (domaine combat), ou l'obligation de rendre un service.
- **Racket** : une bande impose à des commerces un paiement régulier « pour leur protection ». Pour le commerce, c'est une charge fixe non déclarée ; refuser expose à des dégâts (§5.8). Les décisions (qui racketter, combien, comment réagir) relèvent des domaines social et combat.
- **Paiements occultes** : acheter un secret ou corrompre quelqu'un ([social.md](social.md), §4.4) est un **transfert** de l'acheteur au détenteur, presque toujours **en espèces**. Le montant se négocie (domaine social) autour d'une **valeur de référence** : le risque pris par celui qui cède (perte d'emploi, prison), exprimé en mois de son revenu, de **1 à 3 mois** pour un service mineur (fermer les yeux sur un stationnement, une information banale) à **12 mois ou plus** pour un risque grave (code d'un coffre, faux témoignage, policier qui détourne une enquête) **[à calibrer]**. Le paiement peut être immédiat ou promis (un **engagement**, tenu ou non selon le caractère). Un paiement par virement laisse une trace dans le grand livre ; une dépense soudaine du bénéficiaire nourrit les soupçons sur son train de vie (§7.7).
- **Jeux d'argent** (décidé, v0.7). Les jeux **légaux** se vendent au tabac-presse : loterie, jeux de grattage, paris sportifs et hippiques. Les paris ne portent que sur des **événements extérieurs** au monde : ligues, matchs et courses hors de l'agglomération, tirés par le compte « État » (décidé, clôture d'équilibrage C2). *Pourquoi : un pari sur un événement de la ville permettrait d'incarner un de ses acteurs pour truquer le résultat au profit de son persistant.*
  - Les mises vont au compte extérieur « État », qui joue le rôle de l'opérateur national, et les gains en viennent.
  - Le tabac-presse touche une commission d'≈ 5 % des mises.
  - La part rendue aux joueurs est d'≈ 65 % pour la loterie et le grattage, et d'≈ 85 % pour les paris **[à calibrer]**.

  Les jeux **clandestins** se tiennent dans une arrière-salle : poker, cartes, paris entre habitués. Ils sont organisés par un patron de bar ou par une bande, qui prélève ≈ 5 à 10 % des mises en espèces (§7.3). Les tirages utilisent l'aléa à clé, donc ils sont reproductibles.
- **Signature du jeu** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), D15). Une mise n'a pas de signature écrite à la main ; elle est **dérivée** :
  - la **perte espérée**, c'est-à-dire la mise × (1 − part rendue aux joueurs), est un **don** à une contrepartie anonyme, dans le cumul de 30 jours du §12.1 ;
  - la mise rapportée au revenu mensuel donne un effet d'**excès envers soi**.

  Jouer quelques écus au loto est donc anodin ; jouer son salaire est une désynchronisation pour un personnage prudent.
- **Perdre volontairement** (décidé, clôture d'équilibrage C2) : à chaque décision de jeu d'un pilote `Joueur` (se coucher, suivre, relancer, miser), le cœur compare l'**espérance** de la décision prise à celle de la décision qu'aurait prise l'**IA en observation**, dans l'état connu du personnage (ses cartes, ce qu'il sait des autres). C'est le même calcul d'estimation que celui du point de rupture.
  - L'**écart d'espérance cumulé** est un **don aux gagnants** de la main, en proportion de leurs gains. Il n'additionne que les décisions **moins bonnes** que celles de l'IA en observation ; les décisions meilleures ne sont **pas déduites** (décidé, R2, v0.8.2). *Pourquoi : sinon, bien relancer puis se coucher avec la meilleure main réduirait le don compté.*
  - Ce don entre dans le cumul de 30 jours (§12.1), et il est soumis aux règles des liés (§12.3) et du cercle (§12.4).

  Se coucher avec une main gagnante devant le persistant d'un ami est donc un don, même si aucune écriture ne ressemble à un don. *Pourquoi : au jeu, la valeur passe par une décision, pas par une écriture ; seule la comparaison avec ce que le personnage aurait fait mesure le transfert.*
- **Caisse des bandes** : une bande ([social.md](social.md), §6.6) a une **caisse de groupe** en espèces (§2.1), alimentée par ses activités (trafics, racket, recel), qui paie ses membres, ses fournisseurs, ses avocats et ses dépenses de prestige. Une bande peut exploiter plusieurs activités clandestines (§7.3), chacune étant une entreprise clandestine dont la bande est propriétaire.

### 7.7 Blanchiment et traces (décidé)

L'argent illégal est en **espèces**. Pour le dépenser au grand jour (acheter un logement, une voiture, un commerce), il faut le faire entrer dans le circuit déclaré :

| Moyen | Mécanisme | Trace |
|---|---|---|
| Dépôt direct en banque | Simple | Au-delà de **10 000 écus par mois** **[à calibrer]**, ou par dépôts répétés juste en dessous, la banque fait un **signalement** |
| Commerce à recettes en espèces (bar, garage, laverie) | On ajoute de fausses ventes aux recettes déclarées | Écart entre les achats de marchandises et les ventes déclarées, visible lors d'un contrôle fiscal |
| Achats en espèces | Biens payés en liquide (voiture d'occasion, travaux au noir) | Témoins, vendeur |
| Prête-nom | Un proche achète à sa place | Relation connue ; revenus du prête-nom incohérents |

Un **train de vie** incohérent avec les revenus déclarés (voiture neuve d'un chômeur) est une **piste** pour la police et l'administration fiscale, perceptible aussi par l'entourage (rumeur, domaine social). *Pourquoi : quelques règles simples rendent l'enrichissement illégal possible mais risqué, et donnent des prises aux enquêtes du domaine combat.*

### 7.8 Sanctions économiques : amendes, saisies, confiscations, indemnisation (décidé)

**Partage** : le domaine combat, police et justice fixe les infractions, le **barème** des peines, l'ordre de recouvrement et les cas ([combat.md](combat.md), §8.4 à §8.6, §9.5) ; ce domaine fournit le **mécanisme** : comment une somme est calculée à partir des revenus, prélevée, saisie, vendue, et sur quel compte elle arrive. Pour le personnage persistant, les **conséquences équivalentes** d'une condamnation ([01-concept.md](../01-concept.md), §2) passent par ce même mécanisme. *Pourquoi : un seul mécanisme, sans création ni destruction d'argent, sert à tous les cas.*

**Revenu de référence** (décidé). Partout où une sanction dépend des revenus, on prend le **revenu net mensuel moyen des trois derniers mois** tel qu'il apparaît dans le grand livre (salaires, missions, pensions, prestations, loyers perçus, bénéfices déclarés), et au moins le **salaire minimum** ([combat.md](combat.md), §9.5). Les revenus **non déclarés** n'y entrent que s'ils ont été établis par l'enquête. *Pourquoi : c'est ce que la justice peut connaître ; un trafiquant sans revenu déclaré paie au moins sur la base du salaire minimum, et ses avoirs criminels sont confisqués à part.*

**Modulation des amendes par les revenus** **[à calibrer]** :

| Sanction | Calcul |
|---|---|
| **Contravention** (stationnement, tapage, ivresse, vitesse) | Montant **forfaitaire** du barème, non modulé ; majoré de 50 % s'il n'est pas payé dans les 45 jours |
| **Amende délictuelle** | Montant du barème × coefficient de revenu = revenu de référence ÷ salaire médian (§1.4), borné entre **0,5** et **5** |
| **Jours-amende** | Montant par jour = **revenu de référence ÷ 60** (environ la moitié du revenu journalier), au moins 5 écus ; nombre de jours fixé par le tribunal (360 au plus). Les jours non payés se transforment en détention (combat) |
| **Amende équivalente** du personnage persistant en ligne | Formule du domaine combat (`M × revenu de référence × 0,5`, [combat.md](combat.md), §9.5), avec le revenu de référence ci-dessus ; recouvrée comme une amende |
| **Caution** (contrôle judiciaire) | 1 à 3 × le revenu de référence, plus pour un patrimoine important ; somme **bloquée** sur un compte séquestre de la banque : elle reste au patrimoine du personnage mais n'est plus disponible ; rendue s'il a respecté ses obligations, sinon versée à l'État |

**Bénéficiaires** : les amendes vont à l'**État** (compte extérieur), sauf les contraventions constatées par la police municipale (stationnement, tapage, abords des écoles), qui vont à la **commune** (§8.4). Les dommages et intérêts vont à la **victime**.

**Recouvrement** (ordre fixé par le domaine combat, [combat.md](combat.md), §8.6 ; mécanique ici) :

1. **paiement volontaire** dans un délai, ou **échéancier** (mensualités ≤ 30 % du revenu de référence) ;
2. **prélèvement sur les comptes** : tout le solde disponible, hors caution bloquée et hors un **reste à vivre** égal au revenu minimum (§8.3) ;
3. **saisie sur salaire** (et sur pension, indemnités, revenus de missions) : par tranches, de 5 % de la part du revenu proche du salaire minimum à 100 % de la part qui dépasse 3 × le salaire minimum, sans jamais toucher le reste à vivre **[à calibrer]** ;
4. **saisie et vente aux enchères** par le commissaire de justice : véhicules, puis objets de valeur, puis parts d'entreprise, puis biens immobiliers autres que la **résidence principale**, qui est insaisissable sauf pour le prêteur qui l'a financée ;
5. le reste est une **dette** envers le créancier ; elle ne s'efface ni par la mort du personnage persistant ni par le temps ; elle passe à la succession d'un habitant décédé (§5.11).

**Rang entre créanciers** quand l'argent ne suffit pas pour tous (décidé) : pensions alimentaires, puis **victimes** (dommages et intérêts), puis salaires dus aux employés, puis **État et communes** (amendes, impôts, redressements, fonds de garantie), puis banques et créanciers garantis sur leur bien, puis tous les autres. *Pourquoi : on protège d'abord les plus fragiles, comme le droit européen courant.*

**Saisies pendant l'enquête** : espèces, véhicules et objets saisis sont **placés sous scellés** : ils restent la propriété du suspect mais deviennent indisponibles ; rendus en cas de relaxe, confisqués ou vendus au profit des créanciers en cas de condamnation. Les stupéfiants et les armes saisis sont **détruits** après jugement : leur valeur sort de l'économie comme un stock détruit, sans création ni destruction d'argent.

**Confiscation** des avoirs criminels : espèces confisquées versées au compte « État » ; biens (y compris acquis par un prête-nom, si c'est démontré) vendus aux enchères, produit versé à l'État.

**Fonds de garantie des victimes** : quand l'auteur de violences graves ou d'une mort violente est inconnu ou insolvable, le compte « État » indemnise la victime ou ses proches (repères : violences avec incapacité durable 5 000 à 50 000 ; décès ≈ 30 000 par proche direct **[à calibrer]**), puis détient une **créance** sur l'auteur, recouvrée par le même mécanisme s'il est retrouvé.

**Recours des assureurs** : l'assureur qui a indemnisé une victime devient créancier de l'auteur identifié pour la somme versée (§5.8), et la recouvre par le même mécanisme.

**Peines sans argent** : le **travail d'intérêt général** est un travail **non payé** pour la commune ou une association (§4.9), inscrit dans l'agenda comme un emploi, fait de tâches (§3.3), sans salaire ni cotisations ; il ne remplace aucun poste rémunéré. La **détention à domicile sous bracelet** laisse le condamné garder son emploi et ses revenus, dans les horaires imposés. Les **stages** de sensibilisation sont payés par le condamné (≈ 200 **[à calibrer]**).

**Détention et argent** (décidé, en accord avec [combat.md](combat.md), §9.2) :

- le salaire est **suspendu** dès l'incarcération ; au-delà d'**un mois**, le contrat est rompu (abandon de poste) ;
- un **fonctionnaire** est suspendu sans traitement pendant sa détention, puis **révoqué** si la peine ferme atteint un an ou si une interdiction d'exercer est prononcée **[à calibrer]** ;
- le chômage et le revenu minimum du détenu sont suspendus ; la retraite continue ; les prestations du **ménage** (aide au logement, allocations familiales) sont recalculées sur ses ressources restantes ;
- l'**échéancier** continue (loyer, crédits, assurances) ;
- le détenu peut donner **procuration** à un proche avant son départ (ou par courrier) : le mandataire peut payer, encaisser et gérer ses comptes et ses biens, dans les limites de la procuration ; c'est aussi possible pour toute absence longue (séjour, hospitalisation) ;
- son entreprise passe à son **suppléant** ou à l'automate de gestion (§9.5).

**Aide juridictionnelle et honoraires** :

| Défense | Coût **[à calibrer]** |
|---|---|
| Avocat **de permanence** (commis d'office) | Payé par l'État : **gratuit** si le revenu de référence est inférieur à ≈ 1 300 ; **partiel** (25 à 75 % à la charge du justiciable) jusqu'à ≈ 2 000 ; au-delà, honoraires du barème de l'aide, payés par le justiciable |
| Avocat **choisi** | Honoraires libres : ≈ 150 à 300 de l'heure ; forfaits indicatifs : garde à vue 500, comparution immédiate 1 000, audience correctionnelle 1 500 à 5 000, cour d'assises 10 000 à 40 000 |

Les avocats sont des indépendants (§3.2) : les honoraires font leur chiffre d'affaires ; l'aide juridictionnelle leur est versée par le compte « État ». Une bande qui paie l'avocat d'un membre le fait depuis sa caisse (§7.6).

---

## 8. État et collectivités

### 8.1 Les acteurs publics (décidé)

| Acteur | Nature | Rôle économique |
|---|---|---|
| **État** | Compte extérieur abstrait (§2.2), présent par ses services | Collecte cotisations, impôt sur le revenu, TVA, impôt sur les sociétés, droits de succession ; paie les services publics nationaux, retraites, chômage, aides, dotations |
| **Commune d'Orvenne** | Entité locale, maire hors catégorie | Budget propre (§8.4) ; régie des **bus** ; tutelle d'**Orvenne Habitat** |
| **Hautevigne**, **Forges-sur-Ostrelle** | Entités locales | Budget propre ; contribuent au réseau de bus par une participation |
| **Orvenne Habitat** | Office public | Logement social : loyers, entretien, attributions |

Il n'y a **pas d'intercommunalité** ni de département simulés (décidé). *Pourquoi : trois communes et l'État suffisent pour une agglomération de 10 000 habitants ; un échelon de plus compliquerait sans rien apporter au jeu.*

### 8.2 Les prélèvements (décidé)

Barèmes simplifiés, tous **[à calibrer]** :

| Prélèvement | Assiette | Taux | Bénéficiaire |
|---|---|---|---|
| **Cotisations sociales** | Salaires | Coût employeur ≈ 1,6 × le net (1,35 × au salaire minimum) ; le joueur ne voit que le net | État |
| **Impôt sur le revenu** | Revenu net du foyer, par part (1 par adulte, 0,5 par enfant) | Retenu à la source chaque mois : 0 % jusqu'à 1 000 par part, 11 % jusqu'à 2 400, 30 % jusqu'à 6 800, 41 % au-delà | État |
| **TVA** | Ventes | Incluse dans les prix affichés : 20 %, 5,5 % sur l'alimentation ; le commerce la reverse chaque mois | État |
| **Impôt sur les sociétés** | Bénéfice annuel des sociétés | 25 % | État |
| **Cotisations des indépendants** | Bénéfice ou chiffre d'affaires (micro-entreprise : 22 % du chiffre d'affaires) | — | État |
| **Taxe foncière** | Valeur locative des biens possédés | ≈ un mois de loyer théorique par an | Commune |
| **Cotisation foncière des entreprises** | Locaux professionnels | Forfait selon la surface | Commune |
| **Droits de mutation** | Ventes immobilières | 6 % | Commune |
| **Impôt sur les grandes fortunes immobilières** | Patrimoine immobilier net au-delà de **30 × la médiane** (≈ 1,5 million) | 0,5 % à 1,5 % par an | État |
| **Droits de succession** | Héritages (§5.11) | Progressifs | État |
| **Amendes** | Infractions | Barème du domaine combat ([combat.md](combat.md), §8.5), modulé par les revenus pour les délits (§7.8) | Commune (contraventions de la police municipale) ou État |

Les **missions** du tableau (§3.7) sont versées **nettes** : l'agence ou la plateforme règle les cotisations ; seul l'impôt sur le revenu reste dû. *Pourquoi : le personnage persistant doit pouvoir travailler sans gérer de déclarations.*

**Fraude** : un indépendant peut sous-déclarer ; un contrôle fiscal tiré au sort (plus probable en cas d'incohérence, §7.7) peut la révéler, avec redressement et pénalités.

### 8.3 Les prestations (décidé)

Barèmes simplifiés, tous **[à calibrer]**, versés par l'État :

| Prestation | Conditions | Montant |
|---|---|---|
| **Chômage** | Licenciement ou fin de contrat (pas la démission), au moins 6 mois travaillés sur les 24 derniers | ≈ 70 % du dernier salaire net, pendant une durée égale à la durée travaillée (18 mois au plus) |
| **Revenu minimum** | Adulte résident, ressources faibles, plus de chômage ou jamais travaillé | ≈ 600 par mois pour une personne seule, plus par personne à charge, moins les autres revenus |
| **Aide au logement** | Locataire à faibles revenus | Jusqu'à 40 % du loyer, dégressive avec le revenu |
| **Allocations familiales** | À partir du deuxième enfant | ≈ 140 par mois pour deux enfants, plus pour chaque enfant suivant |
| **Retraite** | À partir de **64 ans** et d'une durée minimale de travail | ≈ 50 à 75 % du salaire moyen de fin de carrière ; **minimum vieillesse** ≈ 1 000 |
| **Assurance maladie** | Tout habitant | Soins remboursés à 70 à 100 % ; hospitalisation publique couverte, sauf un forfait journalier (≈ 20) |
| **Bourses étudiantes** | Étudiant à faibles revenus familiaux | 150 à 600 par mois |
| **Congé de maternité** | Salariée ou indépendante enceinte ([population.md](population.md), §7.2) | 16 semaines (6 avant, 10 après la naissance), indemnisées à ≈ 100 % du salaire net dans la limite d'un plafond |
| **Congé de paternité** | Second parent | 4 semaines, indemnisées comme la maternité |
| **Congé parental** | Un parent d'enfant de moins de 3 ans, à sa demande | Jusqu'à 1 an, ≈ 450 par mois ; le poste est conservé |

- Les prestations sont **demandées** : un personnage doit faire la démarche (Maison de l'emploi, mairie, centre social). Un PNJ le fait selon son profil et ses savoirs ; un personnage incarné peut le faire ou l'oublier.
- **Le personnage persistant** y a droit aux mêmes conditions, ce qui empêche sa chute dans la misère s'il ne joue plus. Mais **aucune prestation ne s'accumule pendant ses absences** : le revenu minimum et l'aide au logement ne sont versés que pour les mois où il a été **présent au moins 10 heures connectées** **[à calibrer]** et **à jour de ses démarches de demandeur** : un **rendez-vous mensuel à la Maison de l'emploi**, inscrit dans son agenda (décidé, revue d'équilibrage E33). Sans l'un ou l'autre, le versement du mois est suspendu, comme pour un habitant qui ne fait pas ses démarches. *Pourquoi : sans cela, un compte abandonné accumulerait des aides sans fin, et quelques connexions par mois suffiraient à vivre des aides ; une aide sociale suppose une démarche, ce qui est à la fois réaliste et difficile à exploiter.*

Les **coûts des enfants** pour les ménages ([ia-pnj.md](ia-pnj.md), §8) : crèche ou assistante maternelle ≈ 300 par mois après aides, garderie et centre de loisirs ≈ 10 par jour, cantine ≈ 3,50 par repas, argent de poche ≈ 20 par mois de 11 à 14 ans et ≈ 40 de 15 à 17 ans, fournitures à la rentrée **[à calibrer]**. Les **adolescents de 16 et 17 ans** peuvent prendre des **jobs d'été**, des missions adaptées (manutention légère, aide en restauration, distribution de prospectus) et faire du baby-sitting entre PNJ.

### 8.4 Le budget des communes (décidé)

Chaque commune a un **budget annuel**, voté par son conseil municipal et suivi mois par mois.

| Recettes | Dépenses |
|---|---|
| Taxe foncière, cotisation foncière des entreprises | Salaires des agents municipaux et de la police municipale |
| Droits de mutation | Entretien des écoles primaires, cantine, centre de loisirs |
| **Dotation de l'État** (proportionnelle à la population) | Voirie, éclairage, propreté, espaces verts, déneigement |
| Tarifs (cantine, piscine, parking, bus pour Orvenne) | Réparations de l'espace public (§5.8) |
| Amendes municipales | Subventions (associations, foyer, centre social, fêtes) |
| Loyers et ventes de biens municipaux (parcelles) | Régie des bus (Orvenne), participation (autres communes) |
| Emprunts | Remboursement des emprunts ; investissements (rénovations, équipements) |

- Le **maire** (et son conseil) répartit le budget par **postes** et fixe les taux et tarifs, selon ses priorités et son profil (sécurité, propreté, social, investissement, baisse des impôts). Les débats et alliances relèvent du domaine social.
- **Sécurité municipale** (décidé, [combat.md](combat.md), §3.2 et §5.1) : le conseil fixe l'**effectif** de la police municipale (au départ 10 à Orvenne dont 2 agents de stationnement, 4 aux Forges, 3 à Hautevigne), son **armement** (aucun, non létal, ou armes de poing : un équipement et une formation d'≈ 1 500 par agent, puis ≈ 300 par an **[à calibrer]**) et le parc de **vidéoprotection** (≈ 12 000 par caméra installée, ≈ 800 par an d'entretien **[à calibrer]**). Ces choix viennent des priorités du maire et de la **sécurité perçue** par les habitants ; ils déterminent où se trouvent les caméras municipales pour le domaine combat. Un policier municipal coûte à la commune ≈ 1,6 × son net (§8.2), comme tout agent.
- **Effets visibles** : un poste sous-financé se voit (lampadaires non réparés, rues sales, horaires de la piscine réduits, moins de bus) et pèse sur l'état d'entretien et les prix immobiliers (§6.3).
- **Déficit** : la commune emprunte. Si l'endettement dépasse un seuil **[à calibrer]**, l'État (le préfet, à l'extérieur) impose une **hausse des taxes** et des économies (décidé). *Pourquoi : c'est réaliste et cela borne les dérives d'un maire dépensier, y compris si un joueur incarne un élu influent.*

### 8.5 Les services publics nationaux (décidé)

L'hôpital public, la police nationale, les collèges et lycées publics, l'antenne universitaire, le tribunal (juges, procureur, greffiers, conseillers d'insertion et de probation), l'aide juridictionnelle, les contrôles (inspection du travail, contrôle fiscal), la Maison de l'emploi et le train régional sont **financés par l'État** : leurs salaires et leur fonctionnement sont versés par le compte extérieur « État ». Leurs effectifs sont fixés par des **dotations** de postes, révisées rarement **[à calibrer]**. Ils n'ont pas de recettes à atteindre ; leurs dirigeants gèrent un budget de fonctionnement.

---

## 9. Simulation à plusieurs niveaux

### 9.1 Une seule comptabilité (décidé)

Les comptes, contrats, stocks et titres sont des **composants** des entités, comme les besoins et l'agenda ([ia-pnj.md](ia-pnj.md), §0, principe 5). Ils existent indépendamment du niveau de détail : changer de niveau ne change rien à ce qu'un personnage possède ou doit. Seule la **finesse des transactions** dépend du niveau.

### 9.2 Les transactions selon le niveau (décidé)

| Niveau | Transactions | Exemple |
|---|---|---|
| **Complet** | Une écriture par acte, au moment où il a lieu ; objets instanciés | Le client paie son café au comptoir, la tasse est posée |
| **Simplifié** | Une écriture par **activité** (à la fin de l'activité) | « Faire les courses à la supérette » : une écriture de 38 écus, le stock de la supérette baisse par catégorie |
| **Statistique** | Quelques écritures **par bloc d'agenda**, selon les barèmes du §9.3 | « Soirée au bar » : une écriture au prix moyen local d'une soirée, versée au bar où le personnage est censé être |

Dans les trois cas, l'argent va **au bon commerce** (celui où le personnage est, ou qu'il a choisi) : un commerce loin des joueurs encaisse quand même les achats de ses clients.

### 9.3 Les agrégats par bloc d'agenda (décidé)

En simulation statistique, et en solo accéléré, chaque **bloc d'agenda** résolu en une fois ([ia-pnj.md](ia-pnj.md), §2.4) produit des revenus et des dépenses tirés de **barèmes** par type de bloc, modulés par le budget du personnage (§6.8), les prix locaux, la saison et la météo **[à calibrer]** :

| Bloc d'agenda | Revenus | Dépenses | Effets sur l'entreprise |
|---|---|---|---|
| Travail (salarié) | Salaire **acquis** (versé à la paie) | Repas de midi selon l'habitude (gamelle, sandwich, restaurant) | Production de valeur = tâches moyennes de la fiche × qualité moyenne du personnage |
| Travail (indépendant, commerce) | Ventes aux clients présents dans les autres blocs | Achats, énergie | Stocks consommés, clôture normale |
| Travail (navetteur sortant) | Salaire acquis, versé par le compte « Employeurs extérieurs » | Transport (train, carburant) ; repas pris à l'extérieur, versé à l'extérieur | Aucun en ville |
| Mission | Paiement de la mission | Transport | Tâches du donneur d'ordre réalisées |
| Courses | — | Panier selon la taille du ménage et le budget | Ventes et baisse des stocks du commerce choisi |
| Repas ou sortie à l'extérieur | — | Prix moyen local de l'activité | Ventes du lieu |
| Trajet | — | Ticket, carburant, taxi selon le mode | Recettes de la régie, de la station-service |
| Loisir payant | — | Prix de l'activité | Ventes du lieu |
| Activité illégale (revente, vol) | Gains tirés selon les taux du lieu et les compétences | Achat de marchandise | Stocks et trésorerie du réseau ; événements tirés au sort (arrestation, conflit) |
| Séjour à l'extérieur | Selon le motif ([ia-pnj.md](ia-pnj.md), §9.2) | Coût du séjour (vacances ≈ 60 à 150 par jour et par personne selon le niveau de vie) **[à calibrer]**, versé à l'extérieur | — |

Les barèmes sont calculés **à partir des mêmes règles** qu'en simulation complète (même prix, même choix de commerce, mêmes taux de réussite), seule la résolution est plus grossière. *Pourquoi : c'est la règle de l'IA pour l'accélération (mêmes règles, résolution plus grossière), appliquée à l'argent.*

### 9.4 L'échéancier (décidé)

Certaines opérations ne dépendent d'aucun personnage et tournent selon le **calendrier**, à tous les niveaux et même pour les absents :

| Fréquence | Opérations |
|---|---|
| Chaque nuit | Clôture des entreprises (§4.2), intérêts du découvert, péremption des stocks |
| Chaque semaine | Paie de l'intérim, dépôts de caisse, marché hebdomadaire |
| Chaque mois | Salaires, loyers, prélèvements (énergie, téléphone, assurances, crédits), impôt à la source, prestations, indices immobiliers et patrimoine médian, budget des communes |
| Chaque année | Taxe foncière, impôt sur les sociétés, impôt sur les grandes fortunes, révision des baux, budget voté, assemblées de copropriété |

Les **séjours à l'extérieur** ([ia-pnj.md](ia-pnj.md), §9) : le salaire continue pendant les congés payés et les déplacements professionnels (frais remboursés par l'employeur) ; il est **suspendu** pendant une détention, et le contrat est rompu si la détention dépasse un mois **[à calibrer]** (détail au §7.8) ; loyers et prélèvements continuent dans tous les cas.

### 9.5 Les décisions des entreprises selon le niveau (décidé)

- Le **gérant** prend les décisions de son entreprise (§4.2). S'il est en simulation complète ou simplifiée, il les prend avec ses **recettes** d'IA (commander, fixer un prix, embaucher).
- S'il est **absent** (à l'extérieur, hospitalisé, personnage persistant déconnecté), son **suppléant** de l'organigramme (§4.1) décide à sa place, selon la règle de délégation du domaine social ([social.md](social.md), §6.8) : décisions courantes prises normalement, décisions lourdes et urgentes prises avec prudence, décisions lourdes non urgentes (investir, licencier, vendre, fermer) reportées à son retour.
- Si le décideur (titulaire ou suppléant) est en simulation **statistique**, ou s'il n'y a aucun suppléant, les décisions courantes sont prises par un **automate de gestion** : des règles simples et prudentes (réapprovisionner au même niveau, garder les prix, ne pas embaucher ni investir), modulées par le profil du décideur. Les décisions lourdes attendent qu'un décideur repasse en simplifié ou revienne, sauf urgence (cessation de paiements).
- Les gérants d'entreprises importantes sont des personnages **importants** et ne descendent jamais en statistique ([ia-pnj.md](ia-pnj.md), §6.2).

*Pourquoi : les décisions qui changent le monde sont prises par des recettes, comme le veut l'IA ; la gestion courante d'une boulangerie loin des joueurs n'a pas besoin de plus qu'un automate.*

### 9.5 bis Initialisation à l'équilibre (décidé, [revues/generation.md](../revues/generation.md), P22)

Au départ du **rodage**, les variables économiques rapides ne partent pas de zéro : elles sont tirées dans leurs **distributions stationnaires**.

| Variable | Distribution fournie |
|---|---|
| **Stocks d'un commerce** | Par type d'établissement et catégorie de marchandises : niveau du stock, rapporté au seuil de réapprovisionnement, au moment de la journée où le monde démarre |
| **Trésorerie d'une entreprise** | Par type et par taille d'établissement : trésorerie rapportée aux charges mensuelles, avec une part en difficulté conforme au taux de faillite cible (§10.3) |
| **Solde courant d'un ménage** | Par classe de revenu (§1.6) : solde rapporté au revenu mensuel, selon le jour du mois (avant ou après la paie, le loyer, les prélèvements) |
| **Provisions d'un ménage** | Par taille du ménage : nourriture et consommables, en jours de besoin |
| **Échéances en cours** | Retards de loyer et de remboursement, procédures déjà engagées (§5.5), en proportion des taux cibles |

Ces distributions sont **mesurées par `ts-sim`** sur des simulations longues du monde en accéléré. Elles sont rangées dans le paquet de contenus avec leur **provenance** (version du paquet, graines, durée), et sont remesurées quand une règle économique change.
- Le **patrimoine** et les **biens**, eux, viennent de la génération (§1.6) ; seules les variables qui changent en quelques jours sont concernées.
- Le rodage s'arrête quand les indicateurs se stabilisent. Les indicateurs économiques suivis sont la trésorerie des entreprises, les faillites, le chômage et les impayés (§10.3).

*Pourquoi : un monde qui démarre avec tous les stocks pleins et toutes les trésoreries au même niveau passe ses premières semaines à osciller (ruptures en série, faillites groupées) ; partir de l'équilibre fait du rodage une vérification et le raccourcit.*

### 9.6 Solo accéléré (décidé)

Quand le joueur fait avancer le temps, l'économie suit le niveau de chaque zone (statistique pour l'agglomération, simplifié autour du joueur), et l'**échéancier** (§9.4) s'exécute pour chaque jour sauté. Avancer d'un mois en solo produit donc un mois complet de salaires, loyers, ventes, faillites et ouvertures. Avancer le temps ne rapporte rien de spécial au joueur : son personnage vit simplement la période (salaire s'il travaille, loyer dans tous les cas).

---

## 10. Stabilité sur la durée

Un monde en ligne dure des années réelles. L'économie doit rester crédible sans intervention.

### 10.1 Entrées et sorties d'argent (décidé)

| Entrées dans l'agglomération | Sorties de l'agglomération |
|---|---|
| Exportations (usine, logistique, port) et demande extérieure abstraite (hôtel, hypermarché…) | Importations de marchandises, d'énergie, de carburant |
| Salaires des services publics nationaux et des ≈ 150 navetteurs sortants | Cotisations, impôt sur le revenu, TVA, impôt sur les sociétés |
| Retraites (≈ 2 100 retraités), chômage, bourses, aides, dotations aux communes | Loyers des propriétaires non résidents |
| Prêts des banques refinancés à l'extérieur | Intérêts et remboursements vers l'extérieur |
| Héritages venus de l'extérieur | Dépenses de vacances à l'extérieur, successions parties à l'extérieur |
| Fortune apportée par les nouveaux arrivants | Fortune emportée par ceux qui partent définitivement |

Il n'existe **aucune autre source** : pas d'argent trouvé sur les ennemis, pas de récompense de quête, pas de création par le jeu (décidé). *Pourquoi : c'est ce qui distingue une économie simulée d'une économie de jeu de rôle en ligne, où la création d'argent par les joueurs provoque une inflation continue.*

### 10.2 Les stabilisateurs (décidé)

1. **L'ancre des prix** (§6.1, §6.2) : les commerces achètent au prix de référence ; aucun prix local ne peut s'éloigner durablement de la référence.
2. **L'État** : quand les revenus baissent, les impôts baissent et les prestations montent (et inversement), ce qui amortit les chocs.
3. **La démographie** : le domaine population règle les arrivées et les départs selon les **emplois vacants** et les **logements libres** que ce domaine lui fournit ([ia-pnj.md](ia-pnj.md), §9.4).
4. **La concurrence** : les opportunités (§4.5) font ouvrir des commerces là où la demande n'est pas servie, et les faillites ferment ceux qui sont en trop.
5. **Le régulateur extérieur** : un système invisible observe chaque mois les indicateurs (§10.3) et, s'ils sortent de leur plage, ajuste **lentement** les flux extérieurs : volume des commandes extérieures (dans la limite de la conjoncture), dotation de l'État aux communes, nombre d'arrivants solvables. Il ne touche jamais directement un compte (décidé). *Pourquoi : tout ajustement passe par des flux qui ont un sens dans le monde (une commande, une dotation), donc rien n'apparaît ou ne disparaît inexplicablement.*

### 10.3 Les indicateurs (décidé)

Chaque monde tient un **tableau de bord économique**, lu par le régulateur (§10.2), par la chaîne de calibrage et par les sessions d'exploitation et de modération (décidé, A8) :

| Indicateur | Plage cible **[à calibrer]** |
|---|---|
| Argent détenu par les habitants et les entreprises (par habitant) | Stable à ± 15 % sur un an |
| Prix moyens locaux / prix de référence | 0,95 à 1,10 |
| Taux de chômage | ≈ 8 %, entre 6 et 10 % (même cible que le régulateur démographique, [population.md](population.md), §7.6) |
| Taux de vacance des logements | ≈ 4 à 5 %, entre 3 et 7 % ; sous 2 %, constructions (§5.7) |
| Nombre d'établissements par catégorie et par quartier | Services indispensables présents (§4.7) |
| Faillites (liquidations) par an | ≈ 3 à 5 % des établissements, compensées par les créations (ajout v0.6, utilisé par l'initialisation à l'équilibre, §9.5 bis) |
| Ménages locataires en impayé de loyer | ≈ 2 à 4 % (ajout v0.6, §9.5 bis) |
| Patrimoine médian | Stable à ± 10 % sur un an |
| Inégalités (part du patrimoine des 1 % les plus riches) | Dans une bande réaliste |
| Patrimoine et revenus des personnages persistants | Suivi, voir §10.4 |
| Balance des flux avec l'extérieur | Proche de l'équilibre sur un trimestre |

### 10.4 L'accumulation de richesse par les joueurs (décidé)

Les personnages persistants peuvent s'enrichir sans limite de principe, mais :

- les **impôts progressifs** (revenu, grandes fortunes immobilières, successions) et les **coûts de possession** (entretien, taxe foncière, assurances, dépréciation des véhicules) sont des **puits** naturels ;
- la **profondeur du marché** est limitée : acheter beaucoup fait monter les prix, et les propriétaires PNJ ne vendent que si un projet les y pousse ;
- les **garde-fous** du §12 empêchent l'enrichissement par l'incarnation ;
- les **plafonds de propriété** du §11.4 protègent le logement des habitants ;
- la fortune des persistants **n'entre pas** dans le patrimoine médian (§1.3), donc elle ne déforme pas l'échelle de richesse des habitants.

Comme l'argent ne rapporte pas d'expérience (principe 5), s'enrichir est un choix de vie du personnage, pas une course imposée.

---

## 11. Le personnage persistant

### 11.1 Arrivée (décidé)

Le personnage persistant arrive à la gare ([04-joueur.md](../04-joueur.md), §6) avec :

- **1 200 écus** sur un compte ouvert à son nom et **100 écus** en espèces **[à calibrer]** ;
- **trois nuits payées au foyer de travailleurs** (faubourg Saint-Roch) ; il peut préférer l'hôtel à ses frais ;
- un téléphone avec le tableau des missions (§3.7).

*Pourquoi : assez pour se nourrir et se loger une à deux semaines s'il travaille, pas assez pour se dispenser de travailler ; le foyer plutôt que l'hôtel, parce qu'il est le logement d'arrivée des travailleurs modestes et que l'association qui le gère ne peut pas disparaître (§4.9).*

Le pécule est versé **une seule fois par personnage persistant**. Une réinitialisation de compte en crée un nouveau (§11.6).

### 11.2 Revenus (décidé)

Il peut gagner sa vie comme n'importe qui : **missions** (§3.7), emploi salarié (§3.6, avec les règles d'absence), activité **indépendante** ou **entreprise** (§11.5), **loyers** de biens achetés, **économie parallèle** (§7). Tous les revenus sont soumis aux mêmes impôts.

### 11.3 Pendant ses absences (décidé)

Ses absences sont des séjours à l'extérieur ([ville.md](ville.md), §2.5). Pendant ce temps :

- l'**échéancier** continue (§9.4) : loyer, abonnements, assurances, remboursements de prêt sont prélevés sur son compte ; ses loyers perçus et son salaire de congé payé arrivent ;
- il **ne consomme rien** d'autre : pas de dépense de séjour (décidé). *Pourquoi : le loyer et les charges suffisent à donner un coût à l'absence ; faire payer en plus un « séjour » punirait le joueur qui s'éloigne quelques semaines du jeu ;*
- il n'accumule **aucune prestation** pour les mois d'absence (§8.3) ;
- son employeur le **renvoie** s'il manque ses heures sans congé posé (§3.6) ; un congé posé à l'avance est respecté ;
- s'il ne peut plus payer son loyer, la procédure du §5.5 suit son cours : une **longue absence se remarque** ([ville.md](ville.md), §2.5).

Le joueur peut **prévoir** son absence : poser des congés, payer d'avance, résilier son bail, confier son entreprise à un gérant (§11.5).

### 11.4 Logement et propriété (décidé)

- Il **loue** un seul logement à la fois comme **résidence principale** (lieu de retrait), ou il y est propriétaire. Il peut aussi rester au foyer ou à l'hôtel tant qu'il paie.
- Il peut **acheter** des biens pour les louer, dans la limite de **3 logements** et **2 locaux** en ligne **[à calibrer]** ; en solo, pas de limite (décidé). *Pourquoi : en ligne, des centaines de joueurs pourraient sinon accaparer les 4 700 logements et chasser les habitants ; en solo, le joueur ne gêne que son propre monde.* Les biens détenus par une **société qu'il contrôle** comptent dans ce plafond (décidé, revue d'équilibrage E10). *Pourquoi : sinon une société écran suffirait à contourner le plafond.*
- Il ne peut **pas acquérir le contrôle** d'une entité qui rendrait un personnage **hors catégorie** pour le contrôle de ressources critiques ([02-personnages.md](../02-personnages.md), §3.4, critère 2) : banques, usine principale, plateforme logistique, régie des bus, Orvenne Habitat, réseaux d'énergie et d'eau (décidé). *Pourquoi : c'est la même règle que celle qui interdit d'incarner ceux qui les contrôlent.*
- Ses biens loués à des PNJ sont gérés par l'**agence immobilière** (commission ≈ 7 % des loyers) s'il le souhaite, ce qui lui évite de gérer les relances lui-même.

### 11.5 L'entreprise d'un personnage persistant (décidé)

- Il peut **créer ou racheter** une entreprise (§4.5), en être le gérant et y employer des PNJ.
- Comme il s'absente, il a intérêt à désigner un **suppléant** dans l'organigramme (§4.1), par exemple un **gérant salarié** de confiance, qui décide à sa place pendant ses absences selon la règle de délégation (§9.5) ; sans suppléant, l'**automate de gestion** assure seulement la gestion courante.
- Les employés PNJ jugent leur patron absent comme tout autre (relations, domaine social) : un patron jamais là, qui paie mal, perd ses employés.
- Si la trésorerie s'épuise, l'entreprise suit le chemin normal des difficultés (§4.6).
- **Plafond de puissance** (décidé, revue d'équilibrage E10) : la puissance du persistant est calculée comme celle d'un habitant ([02-personnages.md](../02-personnages.md), §3.3), même s'il ne compte pas dans le classement. Elle ne peut pas dépasser le **seuil du palier 9** : une acquisition de parts, un rachat ou une prise de contrôle qui la dépasserait n'est pas proposé, et une intuition l'explique (« Ce serait trop pour vous. »). Il ne peut pas non plus être nommé à la direction d'une entité hors catégorie (port, Orvenne Habitat). *Pourquoi : la catégorie « hors catégorie » existe pour qu'aucun joueur ne tienne ces leviers, et rien d'autre n'arrête un persistant puisqu'il n'entre pas dans le classement.*

### 11.6 Mort, condamnation et réinitialisation (décidé)

- **Mort** : les **frais d'hôpital** sont couverts par l'assurance maladie à l'hôpital public (forfait journalier seulement) ; la **clinique privée** coûte ≈ 2 000 écus, choisie seulement s'il peut payer ([03-vie-synchronisation.md](../03-vie-synchronisation.md), §3) **[à calibrer]**. Ce qu'il portait peut être perdu ou volé (objets, espèces) ; ses comptes et biens sont intacts.
- **Condamnation** : les conséquences équivalentes ([01-concept.md](../01-concept.md), §2) passent par le mécanisme d'amende et de saisie (§7.8) ; la perte d'emploi suit les règles du §3.6.
- **Réinitialisation du compte** ([04-joueur.md](../04-joueur.md), §1) : le personnage persistant **déménage définitivement** à l'extérieur. Ses biens sont **vendus** (aux enchères ou via l'agence) et le produit part avec lui vers l'extérieur ; son bail est résilié ; ses entreprises sont vendues ou fermées ; ses dettes sont réglées sur ses avoirs, le reste est perdu. Rien ne passe au nouveau personnage (décidé). *Pourquoi : la réinitialisation « fait tout perdre » selon le socle, et les biens libérés retournent au monde.*
- **Abandon** (en ligne seulement, puisqu'en solo le monde est en pause) : un compte qui ne se connecte plus pendant **12 mois réels** **[à calibrer]** est traité comme une réinitialisation pour ce qui concerne le monde (biens vendus, bail résilié), mais le joueur garde son niveau ; à son retour, son personnage persistant revient à la gare avec le produit de la vente sur son compte (décidé). *Pourquoi : libérer les logements et commerces bloqués par des joueurs partis, sans punir définitivement un joueur qui revient.*

---

## 12. Garde-fous contre les abus

Le principal risque est qu'un joueur **détourne l'incarnation** pour enrichir son personnage persistant, un autre personnage qu'il compte incarner, ou un complice. Les garde-fous s'empilent (décidé ; *pourquoi : aucun ne suffit seul, mais ensemble ils rendent l'abus plus coûteux que le jeu honnête, sans interdire l'économie normale*).

### 12.1 Synchronisation (règle du socle appliquée à l'argent)

Toute action économique a une **signature comportementale** ([04-joueur.md](../04-joueur.md), §2) : donner, prêter sans garantie, vendre à perte, payer trop cher, dépenser une fortune, démissionner, vendre sa maison. Son écart avec le profil du personnage est **pondéré par le montant rapporté à son patrimoine** : donner 20 écus à un ami est anodin ; donner la moitié de ses économies à un inconnu est une désynchronisation massive, qui mène vite à l'**éjection** et à la perte de l'expérience de l'incarnation.

**La signature est dérivée de l'effet sur le grand livre** (décidé, [revues/signatures.md](../revues/signatures.md), G1, G8, G9). Aucune action économique n'a de signature écrite à la main. Ses écritures, réelles ou attendues, sont traduites en **effets élémentaires** ; le paquet de contenus en tire la signature, avec une seule fonction pour tout le jeu. Trois effets viennent de ce domaine :

| Effet | Ce qui le déclenche dans le grand livre | Magnitude |
|---|---|---|
| **Sortie de valeur sans contrepartie** (don) | Valeur nette sortie du patrimoine de l'acteur, c'est-à-dire montant versé ou bien cédé moins la contrepartie reçue, estimée au **prix de référence** (§6.1) | Montant net rapporté au patrimoine de l'acteur |
| **Prise du bien d'autrui** | Valeur entrée sans contrepartie, contre le gré du titulaire et **sans fondement en droit** (vol, escroquerie, détournement) | `max(0,01 ; v / min(P, 50 000))` (M3), où `v` est la valeur prise et `P` le patrimoine de la victime |
| **Profiter d'autrui** | Écart au prix de référence au détriment d'une cible vulnérable ou dépendante (usure, abus de faiblesse, loyer abusif) | `max(0,01 ; v / min(P, 50 000))` (M3), où `v` est l'écart au prix de référence |

**Magnitude des effets de valeur** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), M3). Pour la prise, le profit et le **tort matériel** (dégât sur le bien d'autrui, `v` étant alors le coût de remise en état), la magnitude vaut `max(0,01 ; v / min(P, 50 000 écus))`. Pour le don, elle reste `v / P`, avec `P` le patrimoine de l'acteur.
- **Plafond de 50 000** : le patrimoine d'une victime riche compte comme le **patrimoine médian** (§1.3), pas plus.
- **Plancher de 0,01** : toute prise compte au moins un peu.

*Pourquoi : voler 2 000 écus à un millionnaire n'est pas anodin, et le patrimoine médian donne l'échelle commune.*

**Un transfert fondé en droit n'est jamais une prise** (décidé, [revues/signatures-contenu.md](../revues/signatures-contenu.md), M10 et D13). Ces transferts ne portent **aucun effet** de prise ni de profit, même contre le gré de celui qui paie :
- une saisie ou une confiscation (§7.8) ;
- une amende ;
- un impôt ou une cotisation (§8.2) ;
- un loyer, une dette ou une facture dus ;
- une indemnité fixée par une décision de justice ;
- une expulsion exécutée (§5.5) ;
- une retenue sur salaire prévue au contrat.

Le grand livre porte le **motif** de chaque écriture (§2.3) ; c'est ce motif qui établit le fondement. Le policier qui saisit, l'huissier qui exécute, le propriétaire qui encaisse son loyer ou le percepteur ne se désynchronisent donc pas pour un acte légal. Ce qui reste écrit à la main pour ces actes, comme la peine causée par une expulsion, relève des annotations de la revue. *Pourquoi : sans cette règle, tout agent de recouvrement, tout bailleur et tout fonctionnaire des impôts porterait une signature de voleur.*

*Pourquoi : les écritures existent de toute façon pour tenir le grand livre, donc les lire suffit ; un acte nouveau reçoit sa signature sans qu'on ait à y penser, et deux gestes qui ont le même effet sur le patrimoine pèsent pareil.*

**Toute sortie de valeur sans contrepartie est un don** (décidé, revue d'équilibrage E5 ; reconnu par son effet, G9). Le don au **bénéficiaire probable** (celui qui ramasse, gagne ou encaisse) n'a **pas de liste d'actes** : il est reconnu par son effet. Ce qui suit n'est qu'une liste d'exemples :
- abandonner, jeter, détruire ;
- cacher hors de ses propres lieux, déposer là où un autre peut prendre ;
- perdre volontairement au jeu ;
- laisser un pourboire hors des usages ;
- payer un bien ou un service au-dessus de la **borne haute** de son prix (§6.2).

**Cumul sur 30 jours** (décidé, G9). Les dons, les prises et les profits sont additionnés par **contrepartie** sur **30 jours de jeu glissants** : c'est l'**épisode** du personnage, le même qu'il soit piloté par l'IA ou par un joueur.
- Les **bénéficiaires anonymes** (banc, poubelle, table de jeu, inconnu qui ramasse) forment **une seule contrepartie**.
- Chaque acte coûte son **coût marginal**, c'est-à-dire ce qu'il ajoute au cumul. Dix liasses de 500 écus posées sur dix bancs coûtent donc autant qu'un don de 5 000 écus.
- Le même cumul sert aux seuils du §12.2 (5 000 écus, ou 20 % du patrimoine liquide) et au plafond du cercle (§12.4).

*Pourquoi : c'est ce qui empêche de découper un don interdit en petits gestes anodins.*

Ce don est pesé par le montant rapporté au patrimoine, comme ci-dessus. Il suit aussi les seuils du §12.2 (5 000 écus ou 20 % du patrimoine liquide) et le **point de rupture** du socle : l'acte trop contraire au personnage n'a pas lieu. Un **service sans tarif affiché** ne peut pas être facturé à un personnage incarné au-delà de **1,3 fois** le tarif horaire de référence du métier **[à calibrer]**. À la reprise, l'IA cherche ce qu'elle a « égaré », et peut le déclarer perdu ou volé. *Pourquoi : le don est défini par son effet sur le patrimoine, pas par le geste choisi pour le faire ; sinon, poser une liasse sur un banc contournerait toutes les règles sur les dons.*

### 12.2 Décisions lourdes : procédures avec délai, puis réparation par l'IA

Les décisions économiques lourdes passent par une **procédure réaliste avec délai**, comme le demande le domaine social ([social.md](social.md), §8.4) (décidé) :

| Décision | Procédure | Délai avant effet **[à calibrer]** |
|---|---|---|
| Vendre ou acheter un logement, un local, une parcelle | Accord (compromis), puis signature chez le notaire | 10 jours de jeu ; l'acheteur peut se rétracter sans frais pendant ce délai, le vendeur seulement avec une indemnité (10 % du prix) |
| Vendre ou racheter une entreprise, des parts | Accord, puis acte de cession | 15 jours de jeu |
| Vendre un véhicule ou un objet de plus de 5 000 écus | Accord, puis remise des papiers | 2 jours de jeu |
| **Gros virement ou don** (plus de 5 000 écus, ou plus de 20 % du patrimoine liquide) | La banque demande une **confirmation**, exécutée le lendemain | 1 jour de jeu, annulable jusque-là |
| Prêter une grosse somme entre particuliers | Reconnaissance de dette, puis versement | 1 jour de jeu |
| Démissionner | Lettre, puis **préavis** (§3.6) | 2 à 4 semaines de jeu, retrait possible d'un commun accord |
| Résilier un bail | Lettre, puis **préavis** (§5.3) | 1 à 3 mois de jeu |
| Divorcer | Procédure (§5.12) | 1 à 6 mois de jeu |
| Signer un bail, un contrat de travail, un prêt bancaire, un abonnement, une commande de service de plus de 1 000 écus | Signature | Immédiat (ce sont des décisions ordinaires, protégées par les conditions du dossier) ; **exception** ci-dessous |

**Exception : engagements pris pour le cercle** (décidé, revue d'équilibrage E6). Pendant une incarnation, un contrat de la dernière ligne, une **promesse de paiement** ou une **reconnaissance de dette** peut être signé au profit d'un personnage du **cercle du joueur** (§12.4) ou d'un personnage piloté par un autre joueur. Il ouvre alors un **délai de rétractation de 7 jours de jeu** **[à calibrer]**. À la reprise, l'IA le réévalue comme une décision lourde et peut l'**annuler sans indemnité** s'il est dissonant. Dans le monde, c'est le droit de rétractation ordinaire.

Une **promesse d'argent** est pesée **au moment où elle est faite**, comme un don du même montant (§12.1), et non au moment où elle est tenue. Elle entre dans le **même cumul de 30 jours** que les dons envers ce bénéficiaire (décidé, [revues/signatures.md](../revues/signatures.md), G9). Quand elle est tenue, le versement ne compte pas une seconde fois. *Pourquoi : un contrat signé pour un complice transporte de la valeur dans le temps, et doit être traité comme le transfert qu'il prépare.*

- Pendant le délai, la décision est un **engagement** du personnage. Si elle a été prise par un joueur et qu'elle est **dissonante**, l'IA peut **se rétracter** à la reprise, selon son caractère ([social.md](social.md), §8.4) : elle annule le virement, ne se présente pas chez le notaire, retire sa démission. La rétractation coûte ce que prévoit la procédure (indemnité du vendeur) et la confiance de l'autre partie.
- Une décision **fidèle** à la nature du personnage est menée à son terme par l'IA.
- Les petits montants passent **immédiatement** : un café, un loyer, un plein, un cadeau ordinaire.

Après exécution, une opération **dissonante** importante reste un souvenir dissonant ([ia-pnj.md](ia-pnj.md), §4.5). Le personnage cherche à **réparer** selon son profil : réclamer l'argent, porter plainte pour escroquerie ou abus de faiblesse, éviter la personne. Le bénéficiaire en garde rarement le fruit en paix.

*Pourquoi : des délais qui existent dans la vraie vie protègent le personnage sans interdiction artificielle, et laissent à l'IA le temps de revenir sur ce qu'elle n'aurait jamais fait.*

### 12.3 Flux liés interdits (règle propre aux joueurs)

Une règle hors fiction, appliquée comme les règles du cercle (§12.4) par la garde hors fiction du pilote (revue d'équilibrage E35) : sont **liés** à un joueur son personnage persistant et tous les personnages qu'il a incarnés depuis **30 jours de jeu** **[à calibrer]**.

- Entre deux personnages liés au même joueur, les **dons** (argent, objets, biens) sont **impossibles** : l'intention n'est pas proposée, et une intuition l'explique (« il ne ferait pas ça »).
- Les **échanges** restent possibles (acheter, vendre, louer, embaucher) mais seulement **au prix de référence à ± 10 %** **[à calibrer]**, salaires compris (dans la fourchette du métier).
- **Échanges du persistant, estimation, plafond** (décidé, revue d'équilibrage E7) :
  - tout échange dont le **persistant** est partie se fait au prix de référence à **± 2 %** **[à calibrer]** ;
  - un bien **sans prix de référence** se vend au prix d'une **estimation** par un professionnel du monde, payée par le vendeur. Ce sont des biens comme un fonds de commerce, des parts, un objet de valeur unique ou un bien immobilier atypique, et l'estimation est faite par un vrai métier : le notaire pour l'immobilier, le comptable pour un fonds ou des parts, le commissaire de justice pour un objet de valeur ;
  - les échanges entre personnages liés sont plafonnés à **20 000 écus par période de 30 jours de jeu** **[à calibrer]**. Au-delà, l'intention n'est pas proposée (« Il ne ferait pas ça »).

  *Pourquoi : la bande de ± 10 % est un écart acceptable une fois, pas un robinet ; le persistant est le seul personnage que le joueur garde, c'est donc le seul qui compte.*
- **Liaison figée** (décidé, revue d'équilibrage E6) : un engagement pris pendant qu'une liaison existait **garde cette liaison jusqu'à son extinction**. Une échéance fixée après les 30 jours n'échappe donc pas aux flux liés.
- Les **prêts** entre personnages liés suivent les conditions bancaires (§4.10).
- **Comptes d'une même personne** (décidé, revues de sécurité S15 et d'équilibrage E8). La **session de modération** (arbitrage A8 : distincte de la détection, elle décide sur dossier) peut **rapprocher** des comptes et décider qu'ils appartiennent à une même personne : c'est le « lien avéré ».
  - Ces comptes sont traités comme **un seul joueur** pour cette règle : leurs personnages liés sont liés entre eux.
  - Les **comptes présumés liés**, calculés automatiquement, ne servent qu'aux alertes (§12.4) et ne déclenchent jamais cette règle.
  - Un rapprochement entre personnes distinctes (complicité) les place dans le **cercle associé** (§12.4), pas ici.
  - Un rapprochement « **même personne** » exige, en plus des flux, des **indices techniques** : appareil, réseau, horaires de connexion (S15 ; décidé, clôture d'équilibrage C8).

  *Pourquoi : la règle des liés est la seule juste pour un joueur qui a deux comptes, mais une détection automatique se trompe sur les familles et les colocataires, et ne doit donc jamais punir seule.*
- Un échange indirect (passer par un intermédiaire PNJ) est rattrapé par le §12.1 et le §12.2, puisque l'intermédiaire doit lui-même agir selon sa nature.

*Pourquoi : c'est le seul cas que la synchronisation ne bloque pas toujours : un personnage très généreux pourrait, sans se désynchroniser, donner sa fortune au persistant de son joueur.*

### 12.4 Le cercle du joueur et la collusion (en ligne)

Deux joueurs complices peuvent s'échanger les rôles : A incarne un riche PNJ et donne à B, puis l'inverse. Un même joueur peut aussi passer par des amis ou par un second compte. Une seule notion couvre ces cas : le **cercle du joueur**.

**Définition du cercle** (décidé, revue d'équilibrage E8). Le cercle est une donnée **hors fiction**, tenue par le service des comptes ; le monde l'ignore. Il réunit :
1. les **personnages liés** au joueur (§12.3) : son persistant et les personnages qu'il a incarnés depuis 30 jours de jeu ;
2. le **cercle associé**, c'est-à-dire les personnages liés à :
   - ses **amis mutuels** ;
   - les comptes que la **session de modération** a rapprochés du sien (A8).

**Ce qui déclenche une règle** (décidé). Les **comptes présumés liés**, calculés automatiquement, servent seulement à produire des alertes. Seul un **rapprochement décidé par la session de modération** fait appliquer une règle de jeu ; c'est ce que la revue de sécurité (S15) appelle le « lien avéré » (décidé, arbitrage A8).
- **Session de modération** : elle est distincte de la session qui calcule les alertes. Elle décide **sur dossier** : alertes, flux journalisés, graphe des comptes, extraits du grand livre. Sa décision est **journalisée** avec ses raisons. Elle suit les règles de modération du socle ([05-architecture.md](../05-architecture.md), en-tête, A8) (décidé, clôture d'équilibrage C8) :
  - le dossier est construit par un outil déterministe ;
  - la décision est choisie dans une liste fermée, motivée, et cite les événements du journal sur lesquels elle s'appuie ;
  - une mesure lourde demande deux sessions indépendantes ;
  - un échantillon de dossiers est rejugé chaque mois.
- **Actes réciproques** (décidé, C8) : un rapprochement pour **complicité** exige des **actes réciproques** entre les comptes (flux dans les deux sens, échanges de rôles, faveurs rendues). Un flux **à sens unique** ne pèse que sur celui qui l'envoie. *Pourquoi : sinon, un fauteur de troubles pourrait faire rapprocher une cible de lui en lui envoyant des dons qu'elle n'a pas demandés.*
- **Information du joueur** (décidé, arbitrage A11, v0.8.1) : le rapprochement est **notifié** au joueur, comme toute mesure lourde de modération. La notification donne :
  - les **motifs** ;
  - la **règle appliquée** (liés ou cercle associé) ;
  - l'usage de **moyens automatisés** dans la détection ;
  - la **voie de recours**.

  L'**autre compte n'est pas nommé**, et l'**origine du dossier** n'est pas dite. Le rapprochement figure toujours dans l'**export de ses données**. *Pourquoi : la sécurité range le rapprochement parmi les mesures lourdes, et la lecture la plus protectrice du RGPD (A8) demande d'informer la personne ; taire l'autre compte et l'origine du dossier protège les tiers et la détection.*
- **Contestation** (RGPD) : le joueur peut contester un rapprochement. Un **panel de trois sessions neuves** juge la contestation **à la majorité**. Il n'annule la décision que pour une **erreur de fait** ou un **élément nouveau**, désignés dans le dossier ; sinon, ce ne serait qu'un second tirage (décidé, C8). Les arguments du joueur arrivent sous une forme structurée, et son texte libre comme citation filtrée. La voie de contestation est fixée par l'exploitation avant l'ouverture en ligne.

*Pourquoi : un calcul automatique confond les familles, les colocataires et les cybercafés ; une sanction, même discrète, doit être décidée par un juge distinct du détecteur, pouvoir être expliquée et être contestée, sans que personne ait à intervenir à la main.*

Les règles du cercle (décidé, revue d'équilibrage E8 ; *pourquoi : une seule notion, testée une seule fois, couvre le joueur seul, ses amis et ses complices démasqués, au lieu de trois règles qui ont chacune un trou*) :

- **Flux entre liés** : la règle du §12.3.
- **Flux vers le cercle associé** : entre un personnage incarné et un personnage du cercle associé, les dons et les échanges restent possibles. Au-delà de **2 000 écus cumulés par 30 jours de jeu** **[à calibrer]**, ils passent par la **confirmation bancaire** du §12.2 : exécution le lendemain, rétractation possible par l'IA. Ils sont aussi journalisés pour la modération.
- **Espèces et plafond vers le cercle associé** (décidé, clôture d'équilibrage C1) :
  - au-delà de ces 2 000 écus, **aucun don en espèces** d'un personnage incarné vers le cercle associé n'est proposé (« Il ne ferait pas ça ») : la confirmation bancaire ne peut pas s'appliquer à des espèces ;
  - un don « au montant libre » (chapeau, pourboire) est borné à ≈ 20 écus (§3.8) ;
  - les sorties de valeur sans contrepartie d'un personnage incarné vers le cercle associé sont plafonnées à **20 000 écus nets par 30 jours de jeu** **[à calibrer]**, le même plafond que les échanges entre liés (§12.3, E7). Au-delà, l'intention n'est pas proposée.

  C'est une règle **hors fiction**, de la garde du pilote : elle ne change ni la magnitude M3 (§12.1) ni les décisions des PNJ. *Pourquoi : un don de 30 000 écus par un millionnaire pèse peu pour sa signature, et un profil généreux le ferait sans se désynchroniser ; le plafond ferme ce robinet vers les amis comme il est fermé entre liés.*
- **Conflit d'intérêts**, pour sa part économique : un personnage incarné ne prend **aucune décision institutionnelle** dont un personnage du cercle est bénéficiaire ou partie :
  - accorder un **prêt bancaire** ;
  - attribuer une **subvention**, un **marché public**, un **logement social** ;
  - délivrer une **licence**, un **permis** ou un **agrément** ;
  - **recruter** dans un service public ;
  - mener un **contrôle fiscal**.

  L'intention n'est pas proposée, et une intuition l'explique (« Vous êtes trop proche de cette affaire. »). Dans le monde, le personnage se récuse, ou laisse suivre son cours la décision que l'IA avait prise. La règle générale, avec les actes d'enquête, de justice et de témoin, relève du socle et du domaine combat.

**Période probatoire** (décidé, revue d'équilibrage E24). Elle couvre les **30 premiers jours réels** d'un compte en ligne, ou ceux qui suivent une réinitialisation, et dure au moins jusqu'au **niveau 10**. Pendant cette période, le compte ne peut pas recevoir plus de **2 000 écus cumulés** **[à calibrer]** de personnages pilotés par d'autres joueurs ; au-delà, l'intention de l'autre n'est pas proposée. *Pourquoi : un compte neuf est le support idéal d'un complice, et un vrai nouveau joueur n'a besoin d'aucune somme de cette taille.*

**Mariage et compte joint** (décidé, revue d'équilibrage E31). Un mariage ou un pacs peut unir un personnage incarné et un personnage du cercle, ou piloté par un autre joueur.
- Il suit la **procédure avec délai** ([social.md](social.md), §8.4). L'IA peut se rétracter, sans le biais « un personnage honnête tient parole ».
- Un **compte joint** peut verser de l'argent sur le compte personnel d'un persistant, quand le conjoint PNJ a été incarné par le cercle dans les 30 jours. Ces flux sont **journalisés** comme les flux entre joueurs.

*Pourquoi : un mariage arrangé entre complices ne doit pas devenir un compte commun hors des garde-fous.*

**Surveillance** :
- les §12.1 et §12.2 s'appliquent quel que soit le bénéficiaire ;
- les **flux importants entre personnages pilotés par des joueurs différents** (persistants ou incarnés) sont **journalisés** pour la session de modération, avec des alertes sur les schémas répétés (mêmes paires de comptes, dons croisés) **[seuils à calibrer]**. Ces alertes utilisent aussi les **comptes présumés liés** (décidé, revue de sécurité S15) ;
- le grand livre (§2.3) permet de remonter et d'annuler un abus avéré. L'abus est établi par la **session de modération**, sur dossier, et l'annulation est exécutée par la session d'exploitation (décidé, A8).

### 12.5 Autres abus et parades

| Abus | Parade |
|---|---|
| Incarner un banquier pour accorder un prêt à son persistant | Les règles de prêt sont fixes (§4.10) ; s'en écarter est une fraude visible dans le grand livre ; le bénéficiaire est lié (§12.3) ; un prêt au cercle n'est pas proposé (conflit d'intérêts, §12.4, E8) |
| Incarner un patron pour embaucher son persistant à un salaire énorme | Salaire borné à la fourchette du métier entre personnages liés (§12.3) ; signature dissonante pour le patron (§12.1) |
| Incarner un PNJ pour lui faire vendre sa maison à son persistant à bas prix | Prix borné (§12.3) ; délai de 10 jours avant la signature, pendant lequel l'IA se rétracte (§12.2) |
| Vider les comptes d'un PNJ incarné en achats de luxe, puis revendre au persistant | Signature dissonante (§12.1) ; revente entre personnages liés au prix de référence (§12.3), donc sans gain |
| Enrichir un PNJ (dons du persistant) pour l'incarner plus tard et rapatrier l'argent | Dons entre personnages liés impossibles (§12.3) ; un PNJ plus riche peut changer de palier et devenir inaccessible |
| Accumuler des prestations sur un persistant inactif | Prestation versée seulement avec 10 heures connectées dans le mois et le rendez-vous mensuel à la Maison de l'emploi (§8.3, E33) |
| Multiplier les personnages persistants pour toucher plusieurs pécules | Un seul persistant par compte et par serveur ; pécule unique (§11.1) |
| Arbitrage entre le solo et le serveur | Les économies sont totalement séparées ([04-joueur.md](../04-joueur.md), §1) |
| Accaparer les logements d'un serveur | Plafond de biens en ligne (§11.4), construction de résidences (§5.7) |
| Accaparer les missions la nuit | Les missions viennent de vrais besoins, en nombre limité ; les PNJ y ont accès (§3.7) |
| Se faire tuer pour éviter une dette | La dette reste due après le réveil à l'hôpital (§11.6) |
| Se déconnecter pour fuir une saisie ou une expulsion | L'échéancier et les procédures continuent pendant les absences (§11.3) |
| Poser une liasse, perdre au jeu, surpayer un service | Signature de don au bénéficiaire probable ; service sans tarif plafonné à 1,3 × le tarif de référence (§12.1, E5) |
| Bail, prêt ou dette signé pour un complice pendant une incarnation | Rétractation de 7 jours de jeu sans indemnité ; promesse pesée quand elle est faite (§12.2, E6) |
| Échelonner un transfert après la fin de la liaison | La liaison reste attachée à l'engagement jusqu'à son extinction (§12.3, E6) |
| Vendre au persistant un bien sans prix de référence, ou lui transférer par petits échanges | Estimation par un professionnel, ± 2 % pour le persistant, plafond de 20 000 écus par 30 jours (§12.3, E7) |
| Faire transiter l'argent par des amis ou un compte complice | Cercle associé : confirmation bancaire au-delà de 2 000 écus par 30 jours, conflit d'intérêts, journalisation (§12.4, E8) |
| Recevoir des fonds sur un compte neuf | Période probatoire : 2 000 écus au plus reçus d'autres joueurs (§12.4, E24) |
| Contourner le plafond de biens par une société | Biens de la société comptés ; plafond de puissance au palier 9 (§11.4, §11.5, E10) |
| Mariage de complaisance | Rétractation sans biais, journalisation du compte joint (§12.4, E31) |
| Tuer un parent riche, ou le faire tuer, pour hériter | Indignité successorale (§5.11, E31) |
| Comptes multiples | Connexion par la plateforme de distribution ; comptes présumés liés pour les alertes ; règles des liés appliquées sur lien avéré décidé par la session de modération (§12.3, §12.4, S15, A8) |
| Duplication d'argent ou d'objets par un bogue | Invariants du grand livre (somme nulle des écritures) et d'unicité des objets vérifiés chaque jour en production ; gel des flux concernés et annulation ciblée des écritures (§2.3, S23, S30) |
| Glisser une liasse dans le chapeau du persistant d'un ami, ou lui donner des espèces | Montant libre borné à ≈ 20 écus ; aucun don en espèces au cercle associé au-delà de 2 000 écus ; plafond de 20 000 écus nets par 30 jours (§3.8, §12.4, C1) |
| Se coucher avec une main gagnante, parier sur un événement qu'on peut truquer | Écart d'espérance avec la décision de l'IA compté comme don aux gagnants ; paris sur des événements extérieurs seulement (§7.6, C2) |
| Offrir un vêtement cher, revendre la garde-robe | Pièces chères suivies comme objets de valeur, vêtement offert = don, pièces abstraites invendables (§5.13, C3) |
| Brader l'alliance ou les souvenirs d'un personnage incarné | Peine causée envers soi (§5.10, A15 bis, C4) |
| Faire rapprocher une cible en lui envoyant des dons non sollicités | Complicité seulement sur actes réciproques ; un flux à sens unique ne pèse que sur l'envoyeur (§12.4, C8) |

---

## 13. Points ouverts

Aucune question de conception propre à l'économie ne reste ouverte. Restent :

- les valeurs **[à calibrer]** (montants, taux, seuils, délais, bornes de prix, parts de clientèle extérieure, plafonds de propriété), à ajuster par le prototype du cœur ([05-architecture.md](../05-architecture.md), §5) puis par des simulations longues du monde entier en accéléré, en suivant le tableau de bord du §10.3 ;
- les décisions qui doivent être confirmées par le socle ou par d'autres domaines (§14).

---

## 14. Impacts sur le socle et les autres domaines

Ces points ne sont **pas appliqués** ailleurs : ils sont à trancher par la session principale ou par la session du domaine concerné.

### 14.1 Socle

| Fichier | Impact | État |
|---|---|---|
| [01-concept.md](../01-concept.md), §1.1 | Préciser que l'économie est **fermée sur l'extérieur** par quelques comptes abstraits (§2.2) et qu'aucun argent n'est créé par le jeu (§10.1) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §1.1 (au lieu du README) | Monnaie du monde : l'**écu** (§1.1) | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §2 | Les conséquences équivalentes d'une condamnation (amende, saisie de biens) passent par le mécanisme du §7.8 | Intégré au socle le 29/10 |
| [01-concept.md](../01-concept.md), §3 | Effets économiques des saisons et de la météo : prix unitaire de l'énergie fixe, consommation saisonnière ; demande modulée par catégorie (§6.6) | Non repris : le socle les résume déjà ; le détail reste ici (§6.6) |
| [02-personnages.md](../02-personnages.md), §2 | Ajouter des **dépendances** possibles (tabac, alcool, drogues) comme besoins propres à certains personnages, nécessaires à la demande des trafics (§7.3) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.2 | Préciser la composante **Richesse** : patrimoine net (définition du §1.3), médiane calculée sur les **adultes habitants permanents** (ni enfants, ni persistants), patrimoine négatif = 0, recalcul mensuel ; patrimoine médian de départ ≈ 50 000 écus | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.4 et §1.1 | Critère 2 (ressources critiques) : liste des entités économiques concernées (banques, usine principale, plateforme logistique, régie des bus, Orvenne Habitat, énergie, eau), qu'un personnage persistant ne peut pas non plus contrôler (§11.4) | Intégré au socle le 29/10 |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §3.3 | Frais d'hôpital : forfait journalier à l'hôpital public (≈ 20), ≈ 2 000 à la clinique privée ; une dette n'est pas effacée par la mort (§11.6) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1 | Réinitialisation : le personnage persistant déménage définitivement, ses biens sont vendus vers l'extérieur ; abandon de 12 mois réels traité de même pour le monde, sans perte de niveau (§11.6) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §1 | Principe : **l'argent n'est jamais une source d'expérience** (§0) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §6 | Pécule d'arrivée : 1 200 écus sur compte, 100 en espèces, trois nuits au foyer (§11.1) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §7 | Nouveau point délicat « **enrichissement par l'incarnation** » : synchronisation pondérée par le montant, procédures avec délai et réparation par l'IA, **flux liés** interdits entre personnages d'un même joueur, journalisation des flux entre joueurs (§12) | Intégré au socle le 29/10 (les procédures avec délai du §12.2 sont un ajout de la v0.2, cohérent avec [social.md](social.md), §8.4) |
| [05-architecture.md](../05-architecture.md), §2 | Économie dans le cœur : **grand livre** en partie double sauvegardé par jour de jeu (§2.3), comptes et stocks comme composants indépendants du niveau de détail (§9.1), **échéancier** calendaire (§9.4), automate de gestion des entreprises (§9.5), tables de données économiques (§0, principe 7), tableau de bord et régulateur (§10.3) | Intégré au socle le 29/10 (§2.15) |
| [05-architecture.md](../05-architecture.md), §2 | Nouveau en v0.2 : l'**entreprise** et le **groupe** social sont **une seule entité** portant des composants économiques et sociaux (§4.1) | Intégré au socle le 29/10 (§2.17) |
| [04-joueur.md](../04-joueur.md), §1.1 | v0.4 : **barème du travail** : 1 à 5 XP par tâche, 30 XP par heure réelle au plus, rendements décroissants après 4 heures, obligations « à l'heure » et « finir sa journée » maintenues, tâches créées par le cercle exclues (§3.3, E11) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §7, et [05-architecture.md](../05-architecture.md) (service des comptes) | v0.4 : **cercle du joueur** (liés, amis mutuels, comptes rapprochés par la session de modération, A8), **période probatoire**, règles économiques du cercle (§12.3, §12.4, E8, E24, A3, S15) | Intégré au socle le 29/10 |
| [04-joueur.md](../04-joueur.md), §7 | v0.4, choix de ce domaine (décidé ; v0.7 : la session de modération remplace le modérateur, A8) : un rapprochement est **qualifié**. « Même personne » fusionne les comptes pour la règle des liés (§12.3, S15) ; « complicité » les place dans le cercle associé (§12.4, E8) | Intégré au socle le 29/10 |
| [02-personnages.md](../02-personnages.md), §3.3 | v0.4 : **plafond de puissance** du persistant au seuil du palier 9 ; biens des sociétés contrôlées comptés dans son plafond de biens (§11.4, §11.5, E10) | Intégré au socle le 29/10 |
| [05-architecture.md](../05-architecture.md), §2.8 | v0.4 : **invariants** du grand livre et d'unicité des objets vérifiés chaque jour en production, gel et annulation ciblée (§2.3, S23, S30) | Intégré au socle le 29/10 |

### 14.2 Domaine ville

Transmis à la session ville par la session principale le 29/10.

- Ajouter ou confirmer les lieux : **agence d'intérim** (quartier de la Gare), **Maison de l'emploi** (la Cité, antenne aux Forges), **notaire** (Tilleuls), **concession automobile** (Forges), **garde-meubles**, **fourrière**, **centre social** (déjà aux Peupliers), **siège d'Orvenne Habitat**, **pompes funèbres** (ajout v0.2, §5.11).
- Le registre des lieux ([ville.md](ville.md), §9.1) porte aussi : le **titre** (propriétaire, copropriété), le **bail** en cours, la **valeur** estimée et l'indicateur **vacant** (§5.1, §5.2, §4.7).
- Les **parcelles constructibles** ont un propriétaire (souvent la mairie) et un prix (§5.4).
- Les **chambres d'hôtel occupées hors champ** (§4.4) : fermées à clé, sans personne visible.
- Le **régulateur de vacance** déclenche des constructions de résidences (§5.7) : la ville doit garder assez de parcelles constructibles à proximité de la gare et des quartiers populaires.
- Les **opportunités** de commerce (§4.5) s'appuient sur les règles d'urbanisme par quartier.
- Les prix des transports en commun sont fixés au §1.4 (régie d'Orvenne, §8.1).

### 14.3 Domaine IA des PNJ

- Les **fiches de métier** (§3.1) complètent les fiches de rôle ; catalogue indicatif au §3.8 et rôles du prototype au §3.9.
- La **faisabilité** du score d'utilité utilise le **disponible**, la **propension à dépenser** et le **coût relatif** (§6.8).
- Recettes à écrire : chercher un emploi, demander une prestation, chercher un logement, gérer une entreprise (gérant), réapprovisionner, fixer un prix, prendre une mission, emprunter, mettre en gage, receler, blanchir, réclamer une dette, se rétracter pendant le délai d'une décision lourde (§12.2), faire grève (§3.6), désigner un suppléant (§4.1), faire crédit à un habitué (§6.2), payer ou accepter un paiement occulte (§7.6).
- v0.6 (décidé, [revues/signatures.md](../revues/signatures.md), G8) : chaque recette économique **déclare ses écritures attendues** : comptes débité et crédité, montant ou formule du montant, contrepartie, prix de référence.
  - **Pas de signature écrite à la main** : la signature de la recette est **compilée** à partir de ces écritures (§12.1).
  - **Contrôle par les tests** (`ts-sim`) : les écritures réelles d'une exécution doivent correspondre aux écritures déclarées. Un écart est une erreur du paquet.
- **Dépendances** comme besoins propres à certains personnages (§7.3), désormais dans le socle ([02-personnages.md](../02-personnages.md), §2).
- Les **projets** économiques (créer une entreprise, acheter un logement, économiser) s'appuient sur les **opportunités** (§4.5) et le crédit (§4.10).
- Le **suppléant** décide pour un gérant absent ; un **automate de gestion** ne prend que les décisions courantes quand le décideur est en statistique ou sans suppléant (§9.5).
- Le **rendement** au travail dépend de l'ambiance de l'équipe, à travers le stress et la motivation (§3.3).
- Réparation d'une opération **dissonante** après une incarnation (§12.2).
- v0.5 : la demande de [ia-pnj.md](ia-pnj.md) v0.8 (§11, revue de l'intelligence) est intégrée au §4.4. Les **valeurs apprises** sur les commerces (qualité, prix, arnaque, §13.6) entrent dans le choix du commerce, avec la réputation et la fidélité de [social.md](social.md), §5.9. Ce domaine attend de l'IA des PNJ deux choses :
  - que la valeur « arnaque » soit apprise en une fois ;
  - qu'elle soit mise à jour aussi en statistique (§9.3).

### 14.4 Domaine population

Les demandes de [population.md](population.md), §12.4 sont traitées en v0.2 : montants par **classe de revenu et de patrimoine** (§1.6) et prix des logements par quartier (§6.3) ; **revenus venus de l'extérieur** (navetteurs, compte « Employeurs extérieurs », §2.2 ; retraites, bourses, aides, §8.3) ; **≈ 100 postes vacants** et **recrutement extérieur** (§3.6) ; **congés** de maternité, de paternité et parental (§8.3) ; **frais d'obsèques** et **héritages venus de l'extérieur** (§5.11) ; **logement réservé à distance** (§5.3) ; **bailleur social** et **propriétaires bailleurs habitants** (§5.1).

Désaccord tranché par la session principale le 29/10 : l'hôtel garde sa **demande extérieure abstraite** (§4.4), sans visiteur visible ; population.md corrige sa formulation.

Restent attendus de la population :

- répartir les **métiers** (§3.8) dans les ≈ 4 100 postes avec ≈ 12 % d'emplois de nuit (§3.4) ;
- attribuer les **classes** du §1.6 en respectant les contrôles indiqués (médiane ≈ 50 000, budgets soutenables) ; la **propriété** selon les parts du §5.1 ; les **dettes** en cours (prêts immobiliers) ; les **entreprises**, leurs propriétaires et leur **organigramme** avec suppléants (§4.1) ;
- contrôle de viabilité à la génération : chaque entreprise doit avoir une marge attendue positive avec la clientèle réelle de son quartier et sa part extérieure (§4.4) ;
- le régulateur démographique reçoit les **offres longtemps non pourvues** (§3.6) ; un arrivant apporte un patrimoine tiré selon sa classe, un partant l'emporte (§10.1) ;
- les **liens familiaux** et les **proches à l'extérieur** désignent les héritiers (§5.11) ; le **régime matrimonial** de chaque couple marié (par défaut la communauté, §5.12).

### 14.5 Domaine social

Les demandes de [social.md](social.md), §12.4 sont traitées en v0.2 : **organigramme** des entreprises (postes, rangs, supérieurs, suppléants, pouvoirs) et entité unique entreprise-groupe (§4.1) ; **ambiance** et rendement (§3.3) ; **grève** (§3.6) ; **embauche** selon la réputation et les relations (§3.6), **prix d'ami**, fidélité, crédit chez le commerçant et prêts entre particuliers (§6.2), les banques restant à règles fixes ; **procédures avec délai** pour les décisions lourdes (§12.2) ; **caisse des groupes** (§2.1), caisse et activités des **bandes**, **paiements occultes** (§7.6) ; **séparation des biens** au divorce (§5.12).

Restent attendus du social :

- **Entretiens d'embauche**, négociation des prix, des salaires, des pots-de-vin et de l'issue d'une grève, choix des locataires par les propriétaires, recommandations (§3.6, §5.3, §6.2, §7.6).
- **Bandes et réseaux** : territoires des points de deal, racket, usure (§7.3, §7.6).
- **Rumeurs** sur le train de vie (§7.7) ; réactions de l'entourage à une expulsion, une faillite, un héritage.
- **Politique municipale** : conseil, priorités du maire, vote du budget (§8.4).
- **Dettes entre particuliers** comme engagements ([ia-pnj.md](ia-pnj.md), §4.2).

### 14.6 Domaine combat, police et justice

Les demandes de [combat.md](combat.md), §16.5 sont traitées en v0.3 :

- **barème** modulé par les revenus (coefficient de revenu pour les délits, contraventions forfaitaires), **jours-amende**, revenu de référence, bénéficiaires (§7.8, §8.2) ;
- **recouvrement** : échéancier, prélèvement avec reste à vivre, saisie sur salaire par tranches, saisie et vente aux enchères, résidence principale insaisissable, dette perpétuelle, **rang entre créanciers** (§7.8) ;
- **scellés**, **confiscation** au profit de l'État, destruction des stupéfiants et des armes (§7.8) ;
- **fonds de garantie** des victimes par le compte « État », avec créance sur l'auteur ; recours des assureurs (§7.8) ;
- **amende équivalente** du personnage persistant et **caution** sur compte séquestre (§7.8) ;
- **aide juridictionnelle** et honoraires d'avocats (§7.8) ;
- **travail d'intérêt général** non payé, bracelet compatible avec l'emploi, stages payants (§7.8) ;
- **détention** : suspension du salaire, rupture au-delà d'un mois, fonctionnaire suspendu puis révoqué, prestations du ménage recalculées, **procuration** (§7.8) ;
- **salaires** des métiers de police, de justice et de contrôle (§3.8) ; ces services sont financés par l'État (§8.5), la police municipale par les communes ;
- **budgets communaux** de sécurité : effectifs, armement, vidéoprotection (§8.4) ;
- **primes d'assurance** selon la criminalité du quartier et le bonus-malus (§5.8).

Restent attendus du domaine combat : les enquêtes à partir des **signalements bancaires**, du train de vie et du grand livre (§7.7) ; la poursuite de la **corruption** et de l'achat de secrets (§7.6) ; les plaintes pour escroquerie après une opération dissonante (§12.2) ; l'expulsion avec le concours de la police (§5.5) ; la fraude à l'assurance (§5.8).

### 14.7 Domaine interface

Les demandes de ce domaine ont été traitées par [interface.md](interface.md) (son §15.4). Ses demandes en retour sont traitées en v0.3 : **prix d'affichage** lisibles, arrondis, stables dans la journée, ticket (§6.2) ; **fiabilité** des missions en mots (§3.7) ; **prêts en agence**, simulations seulement dans l'application (§4.10) ; **plan papier** vendu au tabac-presse (§1.4) ; **journal local** dont le contenu (avis, annonces légales, ouvertures et fermetures, articles) est produit par l'activité de l'entreprise de presse (§4.8).

Rappel des demandes initiales :

- Affichage du **solde**, des espèces portées, des relevés bancaires et des **échéances** à venir.
- Application **tableau des missions** sur le téléphone (§3.7), annonces immobilières, offres d'emploi.
- **Courrier** (relances, factures, avis d'huissier) qui s'accumule pendant les absences (§5.5, [ville.md](ville.md), §2.5).
- Prix affichés en vitrine et en rayon ; ticket de caisse.
- Indication qu'une intention économique est **impossible** entre personnages liés, sous forme d'intuition (§12.3).
- Outils de **modération** et tableau de bord économique, lus par les sessions d'exploitation et de modération (§10.3, §12.4 ; A8).

### 14.8 Revues d'équilibrage et de sécurité

Les modifications de [revues/equilibrage.md](../revues/equilibrage.md), §6.6, et de [revues/securite.md](../revues/securite.md), §9.12, sont intégrées en v0.4, avec les arbitrages de la session principale :

| Renvoi | Où | Contenu |
|---|---|---|
| E5 | §12.1 | Toute sortie de valeur sans contrepartie a la signature d'un don ; service sans tarif plafonné à 1,3 × le tarif de référence |
| E6 | §12.2, §12.3 | Rétractation de 7 jours pour les engagements pris au profit du cercle ; promesse pesée quand elle est faite ; liaison figée |
| E7 | §12.3 | ± 2 % pour le persistant, estimation professionnelle, plafond de 20 000 écus par 30 jours |
| E8, A3 | §12.4 | Cercle du joueur ; confirmation bancaire au-delà de 2 000 écus vers le cercle associé ; conflit d'intérêts économique |
| E10 | §11.4, §11.5 | Biens des sociétés comptés ; plafond de puissance au palier 9 |
| E11 | §3.3 | Barème du travail |
| E24 | §12.4 | Période probatoire |
| E31 | §5.11, §12.4 | Indignité successorale ; mariage et compte joint |
| E33 | §8.3 | 10 heures connectées et rendez-vous mensuel à la Maison de l'emploi |
| S15 | §12.3, §12.4 | Comptes présumés liés pour les alertes ; règles sur lien avéré |
| S23, S30 | §2.3, §12.5 | Invariants quotidiens, gel et annulation ciblée |

Ces points touchent d'autres domaines :

- **IA des PNJ** : il faut des recettes pour :
  - chercher un bien « égaré » après une incarnation, puis le déclarer perdu ou volé (§12.1) ;
  - réévaluer un engagement pendant les 7 jours de rétractation (§12.2) ;
  - se récuser, ou laisser suivre son cours une décision antérieure, en cas de conflit d'intérêts (§12.4).
- **Social** : un mariage ou un pacs avec le cercle se fait sans le biais « un personnage honnête tient parole » ([social.md](social.md), §8.4 ; §12.4, E31).
- **Combat** :
  - l'indignité successorale suppose qu'une condamnation pour la mort du défunt, ou pour complicité, soit transmise à la succession, même après le partage (§5.11) ;
  - le conflit d'intérêts étend la récusation de [combat.md](combat.md), §11.8 (E8).
- **Population** : le persistant ne peut pas être nommé à une fonction hors catégorie ([population.md](population.md), §6.2 ; §11.5, E10).
- **Interface** :
  - les intuitions « Il ne ferait pas ça », « Ce serait trop pour vous » et « Vous êtes trop proche de cette affaire » ;
  - le rendez-vous mensuel à la Maison de l'emploi dans l'agenda du persistant (§8.3) ;
  - l'exportation des données du joueur, qui inclut les rapprochements décidés par la session de modération (§12.4).

### 14.9 Revues des signatures et de la génération

Les modifications de [revues/signatures.md](../revues/signatures.md), §9.9, et de [revues/generation.md](../revues/generation.md), §13.6, sont intégrées en v0.6 ; le socle est déjà à jour.

| Renvoi | Où | Contenu |
|---|---|---|
| G1, G8, G9 | §12.1 | Signature dérivée des écritures : effets don, prise et profit ; don reconnu par son effet ; cumul de 30 jours par contrepartie, bénéficiaires anonymes réunis, coût marginal |
| G9 | §12.2 | La promesse entre dans le même cumul, et n'est pas comptée une seconde fois quand elle est tenue |
| G8 | §14.3 | Les recettes déclarent leurs écritures attendues ; leur signature est compilée et vérifiée par `ts-sim` |
| P9 | §4.5 | Ratios d'équipement et aires de chalandise par type ; même calcul de demande non servie pour le Programme et pour les opportunités |
| P19 | §6.3 | Taux d'effort cibles et coût entier du flot ménages → logements |
| P22 | §9.5 bis, §10.3 | Distributions stationnaires des stocks, trésoreries, soldes, provisions et échéances ; indicateurs de faillites et d'impayés ajoutés |

Ces points touchent d'autres domaines :

- **Ville** : la table des aires de chalandise du §4.5 est l'entrée de l'affectation du programme. Les regroupements prévus (bars de la Cité et du faubourg, garages des Forges, banques au centre) doivent rester compatibles avec les règles d'urbanisme par quartier ([ville.md](ville.md), §6.2).
- **Population** :
  - les taux d'effort du §6.3 entrent dans le flot ménages → logements ;
  - les distributions stationnaires du §9.5 bis remplacent, pour les variables économiques rapides, les valeurs de départ tirées sans référence ;
  - le contrôle de viabilité ([population.md](population.md), §3.4) devient une vérification.
- **IA des PNJ** : les recettes économiques déclarent leurs écritures attendues (§14.3), et le gabarit d'effets en est déduit ; elles n'écrivent plus de signature.
- **Combat** : les effets prise et profit (§12.1) sont lus dans les mêmes écritures que les qualifications du vol, de l'escroquerie et de l'usure ([combat.md](combat.md), §8.1). Les deux domaines doivent garder le même prix de référence (§6.1) pour estimer un préjudice.

### 14.10 Direction artistique, contenu des signatures et IA des PNJ v1.1

| Demande | Où | État |
|---|---|---|
| Vêtements sur le poste « habillement » ; garde-robe qui s'use ([direction-artistique.md](direction-artistique.md), §12.8) | §5.13 (nouvelle), §1.4, §1.5 | Intégré en v0.7. **Choix : garde-robe agrégée** (valeur de remplacement et usure par personnage), achetée pièce par pièce en boutique en simulation complète |
| Véhicules génériques par catégorie, sans marque (§12.8) | §5.9 | Intégré en v0.7 |
| Transfert fondé en droit, jamais une prise (M10, D13) | §12.1 | Intégré en v0.7 |
| Magnitude des effets de valeur (M3) | §12.1 | Intégré en v0.7 |
| Degré de nocivité des produits (D16) | §7.3 | Intégré en v0.7 ; valeurs pour l'alcool décidées par ce domaine, ajustées ensuite par le calibrage des signatures comme les autres |
| Étiquette « valeur affective » (A15) | §5.10 | Intégré en v0.7 |
| Jeu d'argent dérivé par D15 | §7.6 | Intégré en v0.7, avec les jeux légaux et clandestins |
| Dépenses de vêtements, de coiffeur et de laverie ; musiciens ; tâche « mettre de la musique » ([ia-pnj.md](ia-pnj.md) v1.1, §2.6) | §5.13, §3.3, §3.8 | Intégré en v0.7 |
| Règle 6 du README : aucune renumérotation | §9.5 bis, §9.6 | Appliquée en v0.7 |
| Arbitrage A8 : aucune action de l'utilisateur ni d'un humain ; session de modération distincte de la détection, décidant sur dossier | §2.3, §10.3, §12.3, §12.4, §12.5, §14.1, §14.7, §14.8 | Appliqué en v0.7 ; les seules mentions de l'utilisateur sont les marques historiques de l'en-tête |

Ces ajouts touchent d'autres domaines :

- **Population** :
  - tirer à la génération la valeur et l'usure de la garde-robe de chaque personne, et l'équipement en lave-linge du ménage (§5.13) ;
  - poser l'étiquette « valeur affective » sur les objets hérités et les cadeaux marquants (§5.10).
- **IA des PNJ** :
  - le projet « acheter des vêtements » se déclenche sur le seuil d'usure (§5.13) ;
  - les recettes « jouer de l'argent » et « jouer au chapeau » déclarent leurs écritures (§7.6, §3.8) ;
  - la tâche « mettre de la musique » s'ajoute aux fiches de serveur et de vendeur (§3.3).
- **Direction artistique** :
  - les gammes de vêtements et de véhicules (entrée, moyenne, haut de gamme) se lisent seulement par l'état, la finition et les options (§5.9, §5.13) ;
  - l'organisme de redevance musicale porte un nom fictif (§3.8).
- **Social** : une musique bien choisie améliore un peu l'accueil perçu d'un commerce (§3.8).
- **Combat** : l'alcool servi à un client ivre et la vente à un mineur (§7.3) relèvent aussi du catalogue des infractions ; les jeux clandestins sont une activité de bande (§7.6).
- **Interface** : le prix par pièce s'affiche en boutique (§1.4) ; le chapeau du musicien de rue est un don volontaire du joueur.

### 14.11 Clôture de la conception (v0.8)

Les corrections des revues de clôture sont intégrées en v0.8 :

| Correction | Où | État |
|---|---|---|
| C1 : espèces, montant libre, plafond vers le cercle associé | §3.8, §12.4, §12.5 | Intégré en v0.8 |
| C2 : perdre volontairement au jeu, paris extérieurs | §7.6, §12.5 | Intégré en v0.8 |
| C3 et S-7 : règles de la garde-robe | §5.13, §12.5 | Intégré en v0.8 |
| C4 : A15 bis, céder son propre objet de valeur affective | §5.10, §12.5 | Intégré en v0.8 |
| C8 : rapprochements sur actes réciproques et indices techniques, modération selon le socle | §12.3, §12.4, §12.5 | Intégré en v0.8 (avant l'ouverture en ligne) |

Ce domaine n'envoie plus de demande aux autres domaines. Ce qui reste est du travail de construction, noté **pour la phase 1** :

| Reste | Où | État |
|---|---|---|
| Valeurs **[à calibrer]** (prix, salaires, seuils, plafonds, taux, délais), calibrées par `ts-sim` et le tableau de bord | Tout le fichier, §10.3, §13 | Pour la phase 1 |
| Tables de données économiques : métiers, marchandises avec degré de nocivité, prix de référence, aires de chalandise, barèmes | §0 principe 7, §3.8, §4.5, §7.3 | Pour la phase 1 |
| Écritures attendues déclarées par chaque recette économique, et leur contrôle par `ts-sim` | §14.3, §12.1 | Pour la phase 1 |
| Distributions stationnaires mesurées pour l'initialisation à l'équilibre | §9.5 bis | Pour la phase 1 |
| Invariants du grand livre et d'unicité des objets | §2.3 | Pour la phase 1 |
| Calcul de l'écart d'espérance au jeu, sur l'estimation du point de rupture | §7.6 | Pour la phase 1 |
| Garde hors fiction du cercle (plafonds, espèces, conflit d'intérêts, période probatoire) | §12.3, §12.4 | Pour la phase 1 (règles), ouverture en ligne (modération) |
| Rôles économiques du prototype | §3.9 | Pour la phase 1 |

Les demandes aux autres domaines notées aux §14.2 à §14.10 sont closes. Ce qui n'y a pas été repris reste ouvert pour la phase 1, sans nouvelle demande.
