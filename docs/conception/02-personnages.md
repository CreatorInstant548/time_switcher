# Personnages, attributs et puissance

> Fichier du **socle** : modifié uniquement par la session principale (voir [README](README.md)).

## 1. Les deux types de personnages

| | Personnage persistant | Personnage jouable du monde |
|---|---|---|
| Qui le contrôle | Uniquement son joueur | N'importe quel joueur qui remplit les conditions, un seul à la fois |
| Quand le joueur le quitte ou en est éjecté | — | **L'IA reprend le contrôle** |
| À la déconnexion | **Le personnage disparaît** du monde | **L'IA reprend le contrôle** |
| Existence hors connexion | Aucune | Il vit en permanence, piloté par l'IA |
| Niveau | **Son propre niveau**, distinct du niveau du compte | Calculé à partir de ses attributs (§3.1) |
| Synchronisation | **Aucune** : sa barre de vie est sa vie réelle | Oui ([03-vie-synchronisation.md](03-vie-synchronisation.md), §1) |
| Effet sur le bilan du compte | **Ses actions affectent le bilan du compte** | Le bilan d'incarnation affecte le bilan du compte ([03-vie-synchronisation.md](03-vie-synchronisation.md), §2) |
| À sa mort | Il se réveille **à l'hôpital**, avec un malus selon la cause de sa mort | **Mort définitive** |

Pour le moteur, tous les personnages sont du même type (attributs, mémoire, relations, objectifs) ; seul le pilote change.

### 1.1 Le personnage persistant dans le monde

Le détail est dans [domaines/ville.md](domaines/ville.md), §2.5. En résumé :

- C'est un **habitant qui s'absente souvent** : il loue ou achète un vrai logement, peut prendre de vrais emplois (de préférence à la tâche), et ses absences sont vues par les autres comme des séjours à l'extérieur.
- **Lieux de retrait** : il disparaît et réapparaît dans son logement, dans sa chambre d'hôtel ou de foyer, ou en montant dans un transport qui part (et réapparaît en en descendant). S'il se déconnecte ailleurs, il reste **environ une minute** sur place, puis rejoint le lieu de retrait le plus proche : on ne fuit pas un combat ou une arrestation en se déconnectant. **Tant qu'il est menacé** (affrontement, poursuite, interpellation), **aucun lieu, même de retrait, ne le fait disparaître**, dans la limite d'un plafond ; **en état critique, il ne disparaît jamais** ; un véhicule qui ne lui appartient pas ne quitte pas la carte avec lui. En disparaissant chez lui, il y **dépose** ce qu'il porte, sauf ses effets personnels (décidé, [revues/equilibrage.md](revues/equilibrage.md), E19 à E21 ; *pourquoi : un lieu de retrait est l'endroit où l'on rentre chez soi, pas une porte de secours ni une cachette hors du monde*). Sans lieu de retrait valide, il réapparaît à la gare.
- **Sortie de la carte** (décidé) : une sortie de l'agglomération (route, voie ferrée, ponton) est aussi un lieu de retrait ; il y disparaît et y réapparaît. Pourquoi : c'est la règle la plus simple, cohérente avec le départ en train.
- Il **ne compte pas** dans les 10 000 habitants ni dans le classement des paliers, qui mesure la puissance relative des habitants permanents. Il compte en revanche dans le **graphe social**, avec un facteur de disponibilité, et ses relations évoluent normalement pendant ses absences ([domaines/social.md](domaines/social.md), §1.7 et §5.6).
- À la reconnexion, le joueur peut choisir de réapparaître **à la gare** plutôt qu'à son lieu de retrait habituel, pour échapper à un acharnement ([domaines/social.md](domaines/social.md), §9.4).
- Il ne peut jamais contrôler une entité économique classée « ressource critique » (§3.4).
- **Fonctions et puissance** (décidé, [revues/equilibrage.md](revues/equilibrage.md), E10) : il ne peut **ni être candidat ni être nommé** à une fonction hors catégorie, et sa puissance, calculée comme celle d'un habitant (§3.3), ne peut dépasser le **seuil du palier 9** : une action qui la dépasserait n'est pas proposée. Les biens des sociétés qu'il contrôle comptent dans ses plafonds de biens. *Pourquoi : il n'entre pas dans le classement, et rien d'autre ne l'empêcherait de tenir ce que le jeu refuse à tout joueur.*
- **Création** (décidé) : le joueur choisit l'apparence, le nom et l'âge ; les attributs se fixent par **budget de points** ou par **tirage aléatoire** (qui répartit au hasard le même budget, [revues/equilibrage.md](revues/equilibrage.md), E34), dans des bornes réalistes modulées par l'âge choisi (la moyenne de la population vaut 50). Pourquoi : c'est le choix fait pour la progression réaliste reprise de l'ancien projet, et il laisse au joueur le choix entre maîtrise et surprise. Les **compétences** de départ se limitent aux compétences de base de son âge (conduite, cuisine, usage du téléphone), sans compétence de métier : il les acquiert en jouant (décidé ; [domaines/interface.md](domaines/interface.md), §3.2). L'**éditeur d'apparence** n'offre que l'espace que le générateur de population peut produire (mêmes plages, même catalogue), et il n'existe **aucun objet d'apparence propre aux joueurs** : ni tenue exclusive, ni cosmétique payant, ni récompense visible (décidé, [domaines/direction-artistique.md](domaines/direction-artistique.md), §3.7). *Pourquoi : une apparence que seul un joueur peut avoir le désignerait au premier coup d'œil.*

### 1.2 Âge (décidé)

Tous les personnages **vieillissent avec le calendrier du monde** : au rythme réel en ligne, avec les accélérations en solo. Les enfants deviennent des adultes jouables à 18 ans. Le monde a de vraies **naissances**, et des **décès naturels** annoncés par le déclin de la santé ([domaines/population.md](domaines/population.md), §7.2 et §7.3). Pourquoi : c'est la conséquence directe du temps réel et de la progression réaliste par attributs (l'âge pèse sur les capacités).

## 2. Attributs

Les personnages sont décrits par des **attributs réalistes**, physiques et mentaux, repris de l'ancien projet (voir [l'archive](../archives/conception-voyage-temporel.md), §4) : force, endurance, coordination, perception, mémoire, temps de réaction, régulation émotionnelle, intelligence sociale, compétences et savoirs, états temporaires (fatigue, stress, blessures).

Compléments (décidé, venant de [domaines/ia-pnj.md](domaines/ia-pnj.md), §1.1 et §2.1) :

- un trait **chronotype** (lève-tôt ↔ couche-tard), tiré à la création et modulé par l'âge, qui décale le sommeil et les loisirs ;
- des **besoins** détaillés, qui s'ajoutent aux états temporaires : énergie, faim, soif, vessie, hygiène, confort, santé, sécurité, lien social, détente, stimulation ;
- des **dépendances** possibles (tabac, alcool, drogues), besoins propres à certains personnages ([domaines/economie.md](domaines/economie.md), §7.3).

## 3. Niveau, influence et puissance

La puissance d'un personnage a deux composantes : ce qu'il **sait faire** (son niveau) et ce qu'il **peut faire faire au monde** (son influence). Un champion d'arts martiaux sans le sou et un maire âgé n'ont pas le même genre de puissance, mais les deux comptent.

Toutes les échelles vont de **0 à 100**. Le modèle est validé ; les valeurs et pondérations seront **[à calibrer]** par le prototypage.

### 3.1 Niveau personnel (NP)

Ce que le personnage sait faire, à partir de ses attributs :

| Indice | Contenu | Poids |
|---|---|---|
| Physique | Force, endurance, coordination, vitesse, santé générale | 30 % |
| Mental | Perception, mémoire, temps de réaction, régulation émotionnelle, raisonnement | 30 % |
| Compétences | Moyenne des **3 meilleures compétences**, pondérée par leur utilité (le tir ou la négociation comptent plus que la couture) | 40 % |

Chaque attribut est exprimé par rapport à la population du monde : 50 = la moyenne, 100 = le meilleur niveau humain réaliste. On ne prend que les 3 meilleures compétences, parce qu'un expert dans un domaine est puissant même s'il ne sait rien faire d'autre.

### 3.2 Influence (I)

Ce que le personnage peut faire faire au monde. Cinq composantes, chacune sur 0–100 :

| Composante | Mesure | Repères |
|---|---|---|
| **Autorité** | Rang hiérarchique × taille de l'organisation dirigée, sur une échelle logarithmique | 0 : aucun subordonné · 30 : chef d'équipe (5–10 personnes) · 60 : directeur (≈ 200 personnes) · 90 : dirigeant d'une grande institution |
| **Richesse** | Patrimoine net en multiples du patrimoine médian du monde, sur une échelle logarithmique (indépendante de la monnaie fictive) | 50 : patrimoine médian · 70 : 10 × la médiane · 90 : 1 000 × la médiane |
| **Réseau** | Centralité dans le graphe social : on compte les relations, pondérées par l'influence de chaque relation (connaître le préfet compte plus que connaître dix voisins) | Calculé automatiquement sur le graphe des relations |
| **Moyens** | Ce qu'il peut mobiliser physiquement : personnes à ses ordres, armes, véhicules, locaux | 0 : rien · 50 : une petite équipe équipée · 90 : une organisation armée |
| **Notoriété** | Part de la population qui le connaît, sur une échelle logarithmique | 0 : inconnu · 50 : connu dans son quartier · 90 : connu de toute la ville |

**Calcul** : `I = 0,6 × (meilleure composante) + 0,4 × (moyenne des cinq)`.

Précisions (décidé, venant des domaines) :

- **Richesse** : patrimoine net ; la médiane de référence est calculée sur les **adultes habitants permanents** (ni enfants, ni personnages persistants), un patrimoine négatif vaut 0, et le calcul est refait chaque mois. Patrimoine médian de départ ≈ 50 000 écus **[à calibrer]** ([domaines/economie.md](domaines/economie.md), §1.3).
- **Réseau** : calculé sur les relations **entrantes** (ce que les autres feraient pour le personnage), pondérées par leur poids de mobilisation, mis à jour chaque jour ; repères : 30 = médiane d'un adulte, 60 = 10 ×, 90 = 100 × ([domaines/social.md](domaines/social.md), §1.7).
- **Notoriété** : nombre de personnages pour qui il est au moins « connu de nom », rumeur comprise ([domaines/social.md](domaines/social.md), §1.7).

On donne un poids important à la meilleure composante, car on peut être très influent par un seul levier. Un chef de bande riche en hommes et en armes, mais inconnu du public et sans patrimoine officiel, doit quand même compter comme puissant.

### 3.3 Puissance

```
Puissance = max(NP, I) + 0,25 × min(NP, I)      (plafonnée à 100)
```

- La composante dominante compte pleinement : un champion de combat sans le sou et un notable âgé sont tous deux puissants.
- L'autre composante ajoute un bonus : un chef de gang qui est aussi un combattant d'élite est plus puissant qu'un chef de gang ordinaire.

### 3.4 Paliers d'accès

Les paliers sont définis par **rang dans la population** plutôt que par des seuils fixes. Le monde reste ainsi équilibré à mesure qu'il évolue : il y a toujours à peu près la même proportion de personnages puissants.

Le rang est calculé sur **toute la population, enfants compris** (validé). Les enfants, environ 18 % des habitants, se trouvent presque tous au palier 0 ; les adultes sont donc un peu plus nombreux, en proportion, dans les paliers supérieurs.

Il y a **12 paliers jouables** (0 à 11), plus les personnages **hors catégorie**, jamais jouables. Le niveau de compte va de 0 à 100 **[à calibrer]**.

| Palier | Nom | Part de la population | Part cumulée | Exemples | Niveau de compte requis |
|---|---|---|---|---|---|
| 0 | Anonyme | 40 % | 40 % | Étudiant, chômeur, retraité modeste, employé débutant | 0 |
| 1 | Ordinaire | 20 % | 60 % | Caissier, livreur, ouvrier, employé de bureau | 3 |
| 2 | Établi | 13 % | 73 % | Artisan, commerçant, employé expérimenté, petit revendeur | 6 |
| 3 | Qualifié | 9 % | 82 % | Infirmier, technicien spécialisé, enseignant, jeune policier, sportif amateur de bon niveau | 10 |
| 4 | Confirmé | 6,5 % | 88,5 % | Policier expérimenté, cadre, médecin généraliste, cambrioleur aguerri, agent de sécurité ancien militaire | 15 |
| 5 | Respecté | 4,5 % | 93 % | Chef de service, avocat, patron de PME, gros revendeur de quartier, champion régional | 21 |
| 6 | Notable | 3 % | 96 % | Médecin réputé, journaliste connu, lieutenant d'une bande, commissaire de quartier, entrepreneur prospère | 28 |
| 7 | Influent | 2 % | 98 % | Directeur d'usine, conseiller municipal, chef de bande, boxeur professionnel | 36 |
| 8 | Puissant | 1 % | 99 % | Dirigeant d'une entreprise régionale, commissaire divisionnaire, chef de gang, athlète d'élite | 45 |
| 9 | Très puissant | 0,5 % | 99,5 % | Grand patron, adjoint au maire influent, chef d'un réseau criminel, célébrité locale | 55 |
| 10 | Élite | 0,25 % | 99,75 % | PDG d'une grande entreprise, haut fonctionnaire, baron du crime | 67 |
| 11 | Sommet | 0,15 % | 99,9 % | Grande fortune, figure médiatique nationale, champion du monde | 80 |
| — | **Hors catégorie** | 0,1 % | 100 % | Voir ci-dessous | **Jamais jouable** |

Pour une ville de 10 000 habitants, cela fait environ 4 000 personnages de palier 0, une centaine de palier 8, une quinzaine de palier 11 et une dizaine hors catégorie.

Exemples de calcul (indicatifs : le palier réel dépend du rang dans la population) :

| Personnage | NP | I | Puissance | Palier |
|---|---|---|---|---|
| Caissière de supermarché | 35 | 10 | 38 | 1 |
| Policier expérimenté | 55 | 30 | 63 | 4 |
| Boxeur professionnel sans fortune | 80 | 20 | 85 | 7 |
| Chef de bande | 50 | 70 | 83 | 7 |
| Maire de la ville | 45 | 95 | 100 | Hors catégorie |

Ces paliers sont internes au jeu : **ni les rôles, ni les niveaux, ni les paliers des personnages ne sont affichés** ([04-joueur.md](04-joueur.md), §6).

**Critères qui rendent un personnage non jouable** (en plus du rang de puissance) :

1. **Influence trop grande** : il pourrait bouleverser le monde à lui seul, en une seule session (maire d'une grande ville, chef de la police, parrain du crime organisé, dirigeant d'une entreprise qui fait vivre la ville).
2. **Contrôle de ressources critiques** : infrastructures (énergie, eau, transports), forces armées, systèmes financiers. Entités concernées à Orvenne : les banques, l'usine principale, la plateforme logistique, la régie des bus, le bailleur social Orvenne Habitat, l'énergie et l'eau ([domaines/economie.md](domaines/economie.md), §11.4).
3. **Rôle indispensable au monde** : personnages dont la disparition casserait des pans entiers du monde ou de son histoire.
4. **Enfants** : pour des raisons évidentes, un joueur ne peut pas incarner un mineur. Règle absolue, pour l'IA comme pour les joueurs : **aucun acte de séduction ni relation de couple entre un adulte et un mineur** ; la vie intime des adultes reste abstraite ([domaines/social.md](domaines/social.md), §1.6).

Règles absolues sur les mineurs, en plus (décidé, [domaines/combat.md](domaines/combat.md), §12.2) : **aucun geste violent d'un adulte envers un mineur**, aucune arme ne vise un mineur, et **aucun tir, lancer ou coup n'est proposé** si sa trajectoire passerait près d'un mineur ; **aucun enfant ne meurt du fait d'un personnage** ; entre mineurs, seulement des bagarres sans arme. Se servir de mineurs comme couverture ou les faire agir pour un délit est impossible pour un joueur et signalé à la modération ([revues/equilibrage.md](revues/equilibrage.md), E27 : *les règles absolues protègent les mineurs ; elles ne doivent jamais protéger un adulte qui se sert d'eux*).

Les personnages hors catégorie sont **faits à la main** au départ (une dizaine de fonctions) ; les critères ci-dessus les gardent non jouables même si leur rang de puissance baisse ([domaines/population.md](domaines/population.md), §6.2).

**Non jouable temporairement :**

- s'il n'est pas **au repos** ([04-joueur.md](04-joueur.md), §5) ;
- s'il est déjà contrôlé par un joueur ;
- pendant le **repos de l'IA** qui suit une incarnation, pour un personnage de palier 5 ou plus ([04-joueur.md](04-joueur.md), §4) ;
- s'il est dans un état particulier qui empêche une prise de contrôle sereine (inconscient, hospitalisé en état grave) ;
- s'il est **en garde à vue** ou en détention provisoire (un joueur qui l'incarnait déjà continue) ; une personne sous bracelet électronique reste jouable ([domaines/combat.md](domaines/combat.md), §7.2 et §8.4) ;
- s'il est **à l'extérieur** de l'agglomération (vacances, déplacement, détention : la prison est hors de la carte), jusqu'à son retour ([domaines/ville.md](domaines/ville.md), §7.4).

La puissance d'un personnage évolue avec le monde : un employé promu directeur peut changer de palier, voire devenir hors catégorie.
