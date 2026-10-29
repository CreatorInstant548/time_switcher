# Time Switcher — Document de conception

> Version 0.8 — 28/10/2026 (objets paradoxaux, refroidissement par abandon, explosion en solo, exploits temporels)
> Statut : conception validée dans ses grandes lignes, aucun code écrit.
> Les valeurs marquées **[à calibrer]** sont des points de départ, à ajuster par le prototypage.
> Les points marqués **[proposé]** n'ont pas encore été validés explicitement.

## Sommaire

1. [Vision](#1-vision)
2. [Modèle temporel](#2-modèle-temporel)
3. [Monde : simulation macro et génération micro](#3-monde--simulation-macro-et-génération-micro)
4. [Personnages et progression](#4-personnages-et-progression)
5. [IA des PNJ](#5-ia-des-pnj)
6. [Multijoueur](#6-multijoueur)
7. [Architecture technique](#7-architecture-technique)
8. [Périmètre initial et feuille de route](#8-périmètre-initial-et-feuille-de-route)
9. [Questions ouvertes](#9-questions-ouvertes)
10. [Glossaire](#10-glossaire)

---

## 1. Vision

Un jeu de voyage dans le temps, **en solo et en monde persistant en ligne**, qui **mélange tous les genres** (action, aventure, infiltration, stratégie, survie, vie sociale et économique) au sein d'un même bac à sable systémique. L'univers est l'**histoire réelle**, et le jeu commence à **Roubaix**. Dans ce jeu :

- le joueur se déplace entre les époques et peut **modifier le passé** ;
- toute modification s'applique **instantanément**, sans aucun paradoxe ;
- les lignes temporelles sont **multiples** et **générées procéduralement** ;
- la progression du personnage est **réaliste** : pas de magie, des capacités physiques et mentales qui pèsent sur le gameplay ;
- les PNJ sont pilotés par des **IA capables de tenir n'importe quel rôle**, en partie entraînées par apprentissage ;
- le rendu commence en **2.5D**, puis passe en **3D**, sans toucher à la logique du jeu.

### Principes directeurs

1. **Cohérence physique d'abord** : chaque règle doit pouvoir se justifier, aucune n'est ajoutée seulement pour empêcher un paradoxe.
2. **Lisibilité** : le joueur doit toujours pouvoir comprendre pourquoi le monde a changé et pourquoi il a échoué.
3. **Un seul code de simulation** pour le solo, le serveur, le client et l'entraînement des IA.

### Contraintes explicitement rejetées

Ces mécanismes ont été écartés et ne doivent pas être réintroduits :

- les vagues de changement qui se propagent avec un délai ;
- les lois de conservation des objets entre époques (les doublons restent possibles ; ils ont seulement un coût, la température de la puce, §2.11) ;
- les événements immuables (points fixes).

---

## 2. Modèle temporel

### 2.1 Fondement physique

Le modèle s'appuie sur l'**interprétation d'Everett** (mondes multiples), appliquée au voyage temporel par **David Deutsch** (1991) : un voyageur qui remonte le temps n'arrive jamais dans *son* passé, mais dans celui d'une branche qui lui était identique jusque-là.

Conséquences :

| Paradoxe classique | Pourquoi il ne se produit pas |
|---|---|
| Grand-père | Le grand-père tué appartient à une autre branche que celle d'où vient le voyageur |
| Bootstrap (information ou objet sans origine) | Tout ce qu'apporte un voyageur a une origine dans la branche qu'il a quittée |
| Doublons d'objets | Deux exemplaires dans une même branche ne posent aucun problème : le bilan de matière est respecté à l'échelle du multivers |
| Prédestination | Aucun événement n'est fixé à l'avance |

### 2.2 Deux horloges

- **Temps historique** : la position sur la chronologie (1850, 1950…). **Chaque voyageur a son propre temps historique**, qui avance à son rythme (voir §2.10).
- **Méta-temps** : le temps vécu par les joueurs et le serveur. Il avance toujours et ne recule jamais.

Toute la causalité du jeu est ordonnée par le méta-temps.

### 2.3 Les branches

Une branche n'est pas un état stocké, c'est une **recette** :

```
Branche {
  id
  graine_racine          // héritée de la branche d'origine
  journal                // interventions des voyageurs, triées
  version                // incrémentée à chaque intervention
}

Monde(branche, t, région) = génération(graine, t, région) + interventions applicables
```

- Créer une branche ne coûte presque rien : un identifiant, une graine et une copie du journal (ou une référence au journal parent).
- Une branche que plus aucun voyageur n'occupe est supprimée.

### 2.4 Les voyageurs sont des fils

Un voyageur n'appartient à aucune branche : c'est un **fil** qui les traverse. Il emporte avec lui :

- son corps (capacités, états, âge biologique) ;
- son inventaire ;
- sa mémoire (savoirs, souvenirs, relations telles qu'il les a vécues).

Une modification de l'histoire ne l'affecte jamais directement. S'il supprime sa propre origine dans une branche, il devient une **anomalie** : il n'a plus d'origine dans cette branche, mais continue d'exister.

### 2.5 Les interventions

Une **intervention** est une action d'un voyageur sur le monde : apparaître, disparaître, tuer, sauver, donner, prendre, construire, détruire, convaincre…

```
Intervention {
  auteur                 // id du voyageur
  t_historique, lieu
  t_meta                 // pour départager deux interventions simultanées
  type, cible, paramètres
  préconditions          // ce qui doit exister pour qu'elle reste valide
  ampleur                // influence sur la divergence chaotique
}
```

- Le journal est trié par **temps historique**, puis par **méta-temps**.
- **L'arrivée et le départ d'un voyageur sont eux-mêmes des interventions** (apparition et disparition), avec une ampleur minime.
- Les actions courantes sans effet durable (marcher, regarder) ne sont pas journalisées. Seules le sont les actions qui modifient l'état persistant du monde.

### 2.6 Application instantanée d'une intervention

Quand une intervention est insérée dans le journal d'une branche, à une date t₁ :

1. **Insertion** dans le journal, et incrément de la version de la branche.
2. **Invalidation** des zones postérieures à t₁ situées dans le **cône causal** de l'intervention.
3. **Recalcul de la simulation macro** de t₁ jusqu'aux époques concernées (voir §3).
4. **Rebasage** : toutes les interventions postérieures à t₁, quel que soit leur auteur, sont rejouées dans l'ordre :
   - si leurs préconditions sont toujours remplies, elles sont réappliquées ;
   - sinon, elles deviennent **orphelines** : elles sont retirées et leurs conséquences disparaissent. Leur auteur s'en souvient.
5. **Régénération immédiate** des zones où se trouvent des voyageurs. Les autres zones sont régénérées à la demande (évaluation paresseuse).
6. **Réconciliation** des voyageurs présents :
   - leur position est conservée. S'ils se retrouvent dans de la matière solide, ils sont déplacés vers l'espace libre le plus proche ;
   - leur inventaire et leur mémoire sont conservés ;
   - un **journal des divergences** leur indique ce qui a changé autour d'eux.

#### Cône causal

Une intervention à (t₁, x₁) ne peut affecter une région x à la date t que si :

```
distance(x, x₁) ≤ v × (t − t₁)
```

avec v la vitesse maximale de propagation des influences **[à calibrer]**. En dehors du cône, la région est générée exactement comme avant l'intervention.

#### Divergence chaotique

À l'intérieur du cône, l'écart entre l'ancienne et la nouvelle histoire grandit avec le temps, comme dans un système chaotique (δ(t) ≈ δ₀ · e^(λt), λ étant l'exposant de Lyapunov **[à calibrer]**).

- Juste après une intervention, le monde est presque identique.
- Des décennies plus tard, il peut être méconnaissable.
- Concrètement, chaque élément généré a une **probabilité de dévier** de sa version précédente, qui croît avec le temps écoulé depuis l'intervention et avec son ampleur.

### 2.7 Les voyageurs suivent la ligne

> Quand une branche est modifiée, **tous les voyageurs présents dans cette branche suivent la nouvelle version**, quelle que soit leur époque.

Physiquement, chaque intervention crée une nouvelle branche (Deutsch). Comme tous les voyageurs y passent, l'ancienne n'a plus d'occupant et disparaît. Du point de vue des joueurs, la ligne est **réécrite sur place, instantanément**.

Implémentation : une branche possède un numéro de **version**. On ne crée de branche distincte qu'au moment d'un décrochage.

#### Le décrochage

Un voyageur peut **se décrocher** : il reste dans la version courante de la branche, qui devient une branche distincte, pendant que les autres suivent la modification.

Réglages possibles **[proposé]** :

- toujours suivre (par défaut) ;
- suivre seulement les modifications des membres de son groupe ;
- demander à chaque modification.

Le décrochage est la principale protection contre le sabotage, sans recourir à des événements immuables.

### 2.8 Le voyage

Un voyage se définit par une **destination** : (branche, date, lieu).

Dans le périmètre initial, les dates d'arrivée sont limitées à **5 époques : 1850, 1900, 1950, 2000 et 2020**. Ce n'est qu'une restriction des destinations : l'histoire, elle, est **continue**, et le temps de chaque voyageur avance ensuite librement (§2.10). Passer à des destinations arbitraires ne demandera aucun changement du modèle.

Les déplacements possibles avec la puce temporelle :

| Déplacement | Effet sur la ligne |
|---|---|
| **Bond vers le futur** | Le voyageur **reste sur la même ligne**. Il arrive dans le futur tel que l'histoire actuelle de la ligne le produit, y compris les conséquences de ses propres actions |
| **Bond vers le passé** | Le voyageur **change forcément de ligne** : une nouvelle branche est créée, identique à sa ligne jusqu'à la date visée, et il y arrive seul. Les autres voyageurs de sa ligne d'origine ne sont pas affectés |
| **Changement de ligne** | Le voyageur rejoint une ligne existante. Son apparition y est journalisée comme intervention, et les voyageurs situés plus loin dans le futur de cette ligne en voient les effets instantanément |
| **Synchronisation de puce** | Deux voyageurs synchronisent leurs puces pour se rejoindre : l'un arrive dans la ligne de l'autre, au même instant que lui |

**Retour sur une ligne déjà visitée** : si le voyageur arrive à un instant où sa propre version antérieure se trouve déjà dans cette ligne, il **remplace son ancienne version** à ce moment-là. Une ligne ne contient donc jamais deux corps du même voyageur au même instant.

- Les interventions de l'ancienne version postérieures à ce moment sont retirées du journal : le voyageur revit cette période à sa façon.
- L'ancienne version disparaît avec ce qu'elle portait ; le voyageur garde son propre corps, son inventaire et sa mémoire **[proposé]**.

Le voyage ne repose pas le corps : seul le **temps personnel** vécu compte (voir §4).

### 2.9 Cas limites

| Situation | Résultat |
|---|---|
| A, en 1850, modifie le passé ; B est en 2020 dans la même branche | B voit le changement instantanément (§2.6), ou se décroche |
| A arrive d'une autre branche dans le passé de la branche de B | Identique : l'origine du voyageur n'a aucune importance |
| B, en 2020, agit ; A est en 1850 | A n'est pas affecté : la causalité ne remonte pas |
| A agit en 1899 contre une action de B datée de 1900 | L'action de B devient orpheline ; B s'en souvient |
| Deux interventions au même instant historique | Départagées par le méta-temps |
| Un voyageur rencontre sa version native | Possible : dans une branche modifiée, son double natif a pu vivre une autre vie (allié, rival, miroir) |
| Un voyageur apporte un objet qui existe déjà à cette époque | Deux exemplaires coexistent, et la puce du voyageur chauffe (§2.11) |

### 2.10 Horloges individuelles

Chaque voyageur possède son **propre temps historique** t_i, qui avance selon son mode de jeu : vitesse normale en mode réaliste, accélérée en mode simulation (§6.2). Il n'y a pas d'horloge commune à une époque.

C'est possible sans incohérence parce que le monde est une **fonction** de (branche, t) : on peut calculer l'état de la branche à n'importe quelle date, quel que soit l'endroit où se trouvent les autres voyageurs.

#### La puce temporelle

Chaque voyageur porte une **puce implantée** qui lui permet de voyager dans le temps : bonds vers le futur ou le passé, changements de ligne, synchronisation avec la puce d'un autre voyageur (§2.8). Elle reste active quand le joueur est déconnecté (§6.2).

#### L'attraction temporelle (capture W)

Un voyageur qui **avance** (accélération, bond vers le futur, reconnexion) ou qui **arrive dans une ligne** est capté par l'instant du joueur le plus proche dans le temps, si l'écart est **inférieur à W**.

Cela garantit une propriété simple :

> Dans une ligne, deux voyageurs sont soit **au même instant**, soit séparés d'**au moins W**.

Pourquoi c'est nécessaire : un joueur situé juste derrière un autre (quelques minutes) crée une situation néfaste au jeu :

| Problème | Ce qui se passe |
|---|---|
| **Voyance** | Celui qui est derrière voit la trace de l'autre faire, sous ses yeux, ce que l'autre a fait il y a une minute. Il connaît donc toujours ses actions à l'avance : en combat, il esquive tout et tend des embuscades parfaites |
| **Harcèlement invisible** | Chaque action de celui qui est derrière réécrit la dernière minute de l'autre. L'autre voit son monde changer sans cesse (un PNJ meurt en pleine conversation, une porte se verrouille) sans jamais pouvoir atteindre le responsable, qui est dans son passé |
| **Duplication d'objets** | Celui qui est derrière ramasse un objet avant la trace de l'autre. L'action de l'autre devient orpheline, mais il garde l'objet (un voyageur conserve son inventaire) : l'objet existe désormais en deux exemplaires |
| **Instabilité visuelle** | Le monde de celui qui est devant est réécrit en permanence, ce qui le rend illisible |
| **Charge serveur** | Chaque action déclenche un rebasage de la minute écoulée |

Règles précises :

1. **Arrivée dans une ligne** à la date visée T : si un joueur de la ligne se trouve à moins de W de T (avant ou après), le voyageur arrive **à l'instant de ce joueur** (le plus proche s'il y en a plusieurs).
2. **Avancée** de t₀ vers T : le voyageur est capté par le **premier joueur rencontré** dont l'instant est compris entre t₀ et T + W. Autrement dit, il s'arrête à l'instant de ce joueur s'il le croise en chemin, ou s'il allait s'arrêter juste derrière lui. Il peut choisir de **traverser** cet instant, mais seulement pour s'arrêter au-delà de cet instant + W.
3. **Reconnexion** : la puce a décompté le temps réel (§6.2). Le point de réapparition est traité comme une arrivée (règle 1).
4. Le **bond vers le passé** n'est pas concerné : il crée une nouvelle branche, où le voyageur est seul.

Les écarts ne peuvent pas apparaître autrement : en mode réaliste, toutes les horloges avancent au même rythme (le temps réel), et un joueur déconnecté continue d'avancer en temps réel.

Au-delà de W, deux voyageurs sont vraiment dans des époques différentes : voir la trace de l'autre, c'est alors un vrai voyage dans le temps, pas une gêne entre voisins. Et si l'un des deux veut rejoindre l'autre en avançant, il est capté à son instant (règle 2) : la confrontation se fait face à face.

#### Valeur de W

**W = 24 heures de temps historique** (valeur retenue).

| Valeur | Avantages | Inconvénients |
|---|---|---|
| Quelques minutes ou heures | Arrivées précises | Les problèmes de voyance, de harcèlement et de réécriture du passé immédiat restent possibles à l'échelle d'une même journée |
| **24 heures** | Couvre l'échelle « tactique » (combat, conversation, plans de la journée) ; cohérent avec les blocs d'activité du mode simulation, qui découpent le temps en journées | Une arrivée peut être décalée d'au plus un jour par rapport à la date visée, ce qui est négligeable quand on vise une époque |
| Une semaine ou plus | Protection maximale | Les arrivées et les bonds deviennent imprécis, et les joueurs sont captés trop souvent, surtout dans une ligne très peuplée |

Ce qui reste possible au-delà de 24 h (agir sur la veille d'un autre joueur, par exemple) relève du **voyage dans le temps assumé**, qui est le cœur du jeu : l'autre le découvre dans le journal des divergences, comme n'importe quelle modification de l'histoire.

**Les traces** : quand un voyageur avance, son passage dans l'histoire (présence, déplacements, actions) est enregistré dans le journal. Un voyageur plus « en retard » qui traverse ces dates rencontre le corps de l'autre voyageur, rejoué à partir du journal : c'est sa **trace**.

Grâce à la capture W, les traces n'apparaissent qu'entre voyageurs **vraiment séparés dans le temps** (au moins W) : c'est la situation normale d'un voyage dans le passé, pas une gêne entre joueurs voisins.

- Une trace fait partie de l'histoire de la branche, comme n'importe quel PNJ. On peut interagir avec elle et la modifier.
- Si la trace est modifiée, les interventions suivantes du voyageur concerné sont rebasées comme n'importe quelles autres (§2.6).
- **Le voyageur lui-même n'est jamais affecté** (§2.4). Si sa trace meurt, il devient une **anomalie** dans la nouvelle version : on trouve son corps à une date, et on le croise vivant plus tard. Il conserve son corps, son inventaire et sa mémoire.

**Conséquence pour les joueurs en avance** : les actions d'un joueur vraiment en retard tombent dans leur passé. Elles leur parviennent donc **instantanément**, par le mécanisme normal de modification de l'histoire.

### 2.11 Paradoxes et température de la puce

Le modèle n'interdit rien : il n'y a **pas de loi de conservation**, et un objet peut exister en plusieurs exemplaires. Mais créer une incohérence temporelle a un **coût** : la puce du voyageur **chauffe**.

#### Identifiant des objets

Chaque objet possède un identifiant :

```
ObjetId {
  ligne_origine     // la ligne où l'objet a été créé
  numéro            // unique dans cette ligne
  version           // l'exemplaire : 1 pour l'original, puis une version par copie
}
```

- **L'identité** d'un objet, c'est (ligne d'origine, numéro). Deux exemplaires de la même identité sont « le même objet ».
- **Quand une branche est créée** (bond vers le passé), ses objets gardent l'identité de ceux de la ligne d'origine. Chaque exemplaire qui en diverge (ramassé, modifié, déplacé) reçoit une nouvelle **version**. C'est indispensable : sinon, un joueur qui fait un bond dans le passé pourrait ramasser une seconde fois un objet qu'il possède déjà sans que la puce le détecte.
- Un objet fabriqué en jeu reçoit une nouvelle identité, dans la ligne où il est fabriqué.
- Les identifiants sont **déterministes** : un objet généré procéduralement reçoit toujours le même identifiant à partir de la graine et de sa position dans la génération.

Le serveur tient un **registre des exemplaires** : pour chaque identité, où se trouve chaque exemplaire (dans quelle ligne, à quelle date, porté par qui).

#### Détection et avertissement

Il y a **paradoxe** quand un voyageur fait coexister deux exemplaires d'une même identité :

- il ramasse un objet dont un autre exemplaire est **déjà porté par un voyageur**, lui-même ou un autre.

  *Exemple : A, en 1920, a ramassé une montre. B arrive dans la même ligne en 1900 et veut ramasser cette même montre, qui est encore à sa place en 1900. La puce de B l'avertit que cet objet existe déjà ailleurs. S'il la ramasse quand même, sa puce chauffe : l'action de A devient orpheline, et deux exemplaires de la montre existent désormais, l'un porté par A, l'autre par B.*
- il apporte dans une ligne un objet dont un autre exemplaire y est déjà porté par un voyageur (voir aussi le pillage depuis une branche privée, §2.12) ;
- d'autres types de paradoxes sont listés au §2.12.

Avant toute action paradoxale, la **puce avertit** le joueur. Il reste libre d'agir.

#### Objets paradoxaux

- **Seule la version 1** d'un objet est normale et peut être utilisée normalement.
- Toutes les autres versions sont **marquées paradoxales jusqu'à leur destruction**.
- La version 1 est l'exemplaire **obtenu en premier en méta-temps**. Dans l'exemple de la montre, c'est celle de A : l'exemplaire paradoxal est toujours celui dont l'acquisition crée le doublon, et celui qui le crée est averti. Personne ne peut donc rendre paradoxaux les objets d'un autre joueur.
- Un objet paradoxal le reste même si la version 1 est détruite **[proposé]**. Sinon, il suffirait de détruire l'original pour « blanchir » la copie.
- Un objet paradoxal ne peut être ni utilisé, ni vendu, ni donné, ni consommé, ni transformé. On peut seulement le porter, l'abandonner ou le détruire **[proposé]**.
- Un objet fabriqué à partir de matériaux paradoxaux est lui-même paradoxal **[proposé]**. Sinon, fondre de l'or dupliqué en un nouveau lingot suffirait à le blanchir.
- La puce peut **dissoudre** un objet paradoxal qu'on porte, pour le détruire même s'il est indestructible par des moyens ordinaires (or, pierre…) **[proposé]**.
- Un objet paradoxal abandonné reste dans le monde. Quiconque le ramasse est averti par sa puce, et sa propre puce chauffe s'il le garde.

#### Température

| Zone | Effets **[seuils à calibrer]** |
|---|---|
| Normale | Aucun effet |
| Chaude | Gêne, sensation de brûlure : légère hausse du stress |
| Brûlante | Douleur, fièvre : stress élevé, motricité fine et concentration dégradées (§4.4) |
| Critique | Douleur intense, malaise ; avertissements répétés de la puce |
| Explosion | **Mort** (voir ci-dessous) |

- Tant que le voyageur porte des objets paradoxaux, la température **monte progressivement**, d'autant plus vite qu'il en porte beaucoup **[proposé]**.
- La puce **ne refroidit que lorsqu'on s'est débarrassé de tous les objets paradoxaux** (abandon ou destruction).
- La chaleur agit sur les états du personnage, conformément à la progression réaliste.

#### Explosion de la puce

Si le joueur accumule trop de paradoxes, sa puce **explose** et il meurt.

- **En ligne** : c'est une mort plus grave que les autres. **Le personnage disparaît tout simplement**, sans corps ni hôpital, même en mode permissif.
- **En solo** : le personnage ne disparaît pas. C'est une mort ordinaire, avec retour au dernier point de sauvegarde.

### 2.12 Exploits temporels identifiés **[à arbitrer]**

Ces abus n'ont pas tous la forme d'objets en double, mais ils profitent tous du voyage dans le temps.

| Exploit | Principe | Parade proposée |
|---|---|---|
| **Recommencer jusqu'à réussir** | Le joueur rate une négociation, perd un pari ou commet un crime, puis fait un bond dans le passé (nouvelle branche) ou revient sur la ligne avant l'événement (remplacement). Il réessaie en sachant ce qui va se passer | Un coût d'usage pour la puce : énergie qui se recharge en temps réel, ou fatigue et douleur après chaque bond vers le passé |
| **Fuite instantanée** | Sur le point de mourir ou de perdre un combat, le joueur active sa puce et disparaît. Le combat devient ingagnable, et la mort en mode hardcore évitable | Une activation de la puce qui demande plusieurs secondes de préparation, interrompue si le joueur est frappé ou en état de stress extrême |
| **Échange annulé** | A vend un objet à un marchand en 1920. Son complice B, en 1900, tue le marchand. La vente devient orpheline : A garde l'argent, et l'objet n'a jamais quitté ses mains dans la nouvelle histoire. A obtient de l'argent sans rien perdre | Quand un échange devient orphelin, il est **annulé des deux côtés** : chacun récupère ce qu'il a donné et perd ce qu'il a reçu. L'opération est neutre, pour une victime comme pour des complices |
| **Pillage depuis une branche privée** | Un bond dans le passé crée une branche où le joueur est seul. Il y ramasse les trésors avant tout le monde, puis les ramène dans une ligne partagée, où les exemplaires d'origine sont encore à leur place | Un objet amené d'une autre ligne devient paradoxal s'il s'y trouve déjà un exemplaire **que ses occupants ont déjà vu ou utilisé**. Les objets ordinaires transportés par un voyageur ne sont pas concernés |
| **Placements à travers le temps** | Déposer de l'argent en 1850, faire un bond vers 2020 dans la même ligne, retirer 170 ans d'intérêts, les convertir en or, faire un bond dans le passé, et recommencer | Le réalisme suffit, à condition de le simuler : inflation, faillites bancaires, guerres, changements de monnaie (nouveau franc, euro), comptes dormants saisis par l'État, et impossibilité de prouver son identité pour un compte vieux de 170 ans |
| **Connaissance de l'avenir** | Parier sur des résultats connus (bourse, courses, élections), investir dans les entreprises qui réussiront. Les joueurs connaissent en plus l'histoire réelle | La divergence chaotique : chaque gain modifie l'histoire, et plus on en profite, moins l'avenir connu reste fiable. Les petits gains restent possibles, c'est le plaisir du voyage dans le temps |
| **Commerce anachronique** | Apporter des antibiotiques en 1900, ou acheter de l'or bon marché à une époque pour le revendre à une autre | Le réalisme : poids transportable (force), méfiance des PNJ envers des produits inexplicables, demande limitée, attention des autorités |
| **Paradoxe comme arme** | Faire chauffer la puce d'un autre joueur pour le tuer | Déjà couvert : seul celui qui crée un doublon reçoit l'exemplaire paradoxal, et tout transfert d'objet exige l'acceptation du receveur, après l'avertissement de sa puce |

---

## 3. Monde : simulation macro et génération micro

Simuler des siècles en détail est impossible, et un simple bruit aléatoire rendrait les changements incompréhensibles. Le monde est donc simulé sur **deux niveaux**.

### 3.1 Niveau macro : simulé

Pour chaque **région** et chaque **pas de temps** (une année **[à calibrer]**), un ensemble réduit de variables :

- population, richesse, ressources ;
- factions et rapports de force ;
- niveau technologique ;
- infrastructures (routes, villes, fortifications) ;
- **personnages notables** (dirigeants, figures locales, lignées), qui sont des entités nommées.

La simulation macro est **entièrement déterministe** : elle utilise un générateur aléatoire maison, initialisé à partir de la graine de la branche. Elle doit pouvoir recalculer plusieurs siècles en quelques millisecondes.

Une intervention agit sur le niveau macro, soit en modifiant des variables (« la récolte de 1851 est détruite »), soit en modifiant une entité nommée (« l'industriel Aldric meurt en 1850 »).

### 3.2 Niveau micro : généré

Les détails (bâtiments, terrain fin, PNJ ordinaires, objets) sont **générés procéduralement à partir de l'état macro** et de la graine :

```
générer_zone(état_macro(région, t), graine, zone) → description de la zone
```

Le niveau micro produit une **description sémantique**, jamais des ressources graphiques. Par exemple : « maison en pierre, 2 étages, en (x, y), orientée nord, délabrée ». Chaque rendu (2.5D, 3D) traduit cette description avec sa propre table de correspondance.

### 3.3 Persistance des objets à travers le temps

Un objet laissé dans le passé (coffre enterré, lettre, bâtiment) n'atteint une époque ultérieure que s'il **survit** : sa probabilité de survie dépend de son matériau, de son emplacement et des événements macro (guerres, incendies, constructions).

C'est la base de la coopération entre époques : messages, caches, investissements.

### 3.4 Histoire réelle

La branche d'origine reproduit l'**histoire réelle**. La simulation macro calcule donc un **écart à l'histoire** plutôt qu'une histoire inventée :

```
état_macro(région, t) = histoire_réelle(région, t) + δ(région, t)
```

- Dans la branche d'origine, δ = 0 partout : on obtient exactement l'histoire réelle.
- Une intervention crée un écart δ, que la simulation fait évoluer : il peut s'amortir (une petite perturbation que la société absorbe) ou s'amplifier (divergence chaotique, §2.6).
- Aucun événement n'est imposé : l'histoire réelle n'est que l'état de référence. Une fois l'écart assez grand, la simulation suit sa propre dynamique.

**Données nécessaires** : séries de population, économie (usines, emplois, salaires, prix), cartes et plans successifs (cadastres, plans de ville), bâtiments notables, personnages notables, événements locaux et nationaux. Pour Roubaix, la période 1850–2020 est exceptionnellement riche : essor de la « ville aux mille cheminées » et de l'industrie textile, forte immigration belge, occupation allemande de 1914–1918, puis désindustrialisation et reconversion.

**Hors de la région simulée** : le reste du monde (guerres, politique nationale, économie mondiale) suit l'histoire réelle et agit comme une **influence extérieure** sur la région. C'est une **limite de périmètre**, pas un événement immuable : elle disparaîtra à mesure que la simulation s'étendra à d'autres régions. Dans le périmètre initial, une intervention à Roubaix ne peut donc pas empêcher la Première Guerre mondiale.

**Personnes et marques réelles** :

- les personnages historiques décédés peuvent apparaître comme entités nommées ;
- pour les époques récentes (2000, 2020), les personnages notables sont **fictifs**, pour ne pas mettre en scène des personnes vivantes ;
- les marques et entreprises encore existantes sont remplacées par des équivalents **fictifs**.

**Première région : Roubaix et ses environs.**

### 3.5 Sources historiques et méthode

#### Sources identifiées pour Roubaix

| Source | Contenu utile | Réutilisation |
|---|---|---|
| **Archives municipales de Roubaix** et leur bibliothèque numérique **BN-R** (bn-r.fr) | Délibérations du conseil municipal, plans, photographies, cartes postales, état civil, voirie ; plus de 590 000 pages numérisées | Conditions à vérifier |
| **Archives nationales du monde du travail** (ANMT), installées à Roubaix dans l'ancienne filature Motte-Bossut | Archives d'entreprises, de syndicats et d'associations (économie, techniques, vie ouvrière) | Conditions à vérifier |
| **Gallica** (BnF) | Presse ancienne (dont la presse locale), livres, annuaires, cartes | Gratuite pour un usage non commercial ; **licence payante pour un usage commercial** |
| **IGN — Remonter le temps** | Cartes d'état-major, cartes de 1950, photographies aériennes 1950–1965, comparables aux cartes actuelles | Licence à vérifier |
| **Cassini (EHESS)** et recensements | Séries de population de la commune | Données factuelles |
| **INSEE** | Statistiques récentes (2000, 2020) | Licence ouverte |
| **OpenStreetMap** | Géographie actuelle (rues, bâtiments) | Licence ODbL (attribution, partage à l'identique des données) |
| **Wikidata** | Personnages, bâtiments, entreprises, dates | Domaine public (CC0) |
| **Base Mérimée** (plateforme POP) | Patrimoine bâti protégé | Licence ouverte en général |
| Archives départementales du Nord, travaux d'historiens, associations d'histoire locale | Compléments, synthèses, validation | Variable |

#### Méthode

1. **Recherche et compilation** : Claude rassemble les données dans des fichiers structurés du dépôt (`data/history/roubaix/`) : séries chiffrées, entités nommées, événements, plans.
2. **Une source par donnée** : chaque valeur cite sa source et porte un **niveau de confiance**.
3. **Gestion des contradictions** : quand les sources divergent, toutes les valeurs sont conservées et la contradiction est signalée. Par exemple, les tableaux de population donnent environ 124 000 habitants en 1901, mais un article en donne environ 220 000 (probablement une agglomération plus large).
4. **Précision** : pour l'instant, une précision approximative suffit. Une relecture par un historien local pourra venir plus tard.
5. **Contact avec les institutions** : les archives municipales et les ANMT sont à Roubaix même. Un projet de jeu qui met leur ville en valeur peut ouvrir un partenariat (accès aux fonds, conseils, droits de réutilisation).

#### Règles de droits

- Les **faits** (dates, chiffres, noms, événements) ne sont pas protégés : on peut les utiliser librement dans la simulation.
- Les **documents** (photos, cartes, scans) ne sont **pas intégrés tels quels** dans le jeu. Ils servent de **référence** pour créer nos propres ressources graphiques (bâtiments, rues, costumes).
- Toute intégration directe d'un document passe d'abord par une vérification de sa licence, et par une licence commerciale si nécessaire.

Exemple de premières données (à vérifier sur Cassini avant usage) : environ 34 700 habitants en 1851, 124 661 en 1896, 124 365 en 1901.

---

## 4. Personnages et progression

Aucune magie. La performance d'un personnage à un instant donné vaut :

```
performance = capacité × compétence × modificateur d'état
```

### 4.1 Capacités physiques

| Capacité | Effets en jeu |
|---|---|
| Force maximale | Charge portable, corps à corps, armes lourdes, forcer un passage |
| Puissance (explosivité) | Sauts, départs de sprint, impact des coups |
| Endurance aérobie | Durée d'effort, récupération du souffle, longues marches |
| Endurance anaérobie | Enchaînements de combat, sprints répétés |
| Mobilité | Escalade, passages étroits, risque de blessure |
| Coordination et équilibre | Précision des gestes, équilibre (poutres, chevaux, navires) |
| Composition corporelle | Poids (fatigue, chutes, nage), réserves d'énergie, résistance au froid |

### 4.2 Capacités mentales

Elles agissent directement sur ce que le joueur voit et contrôle.

| Capacité | Effets en jeu |
|---|---|
| Attention et perception | Rayon de détection, détails visibles (indices, menaces) |
| Mémoire de travail | Quantité d'informations affichées à l'interface, durée de mémorisation de la carte |
| Mémoire à long terme | Rétention des savoirs, vitesse d'apprentissage |
| Temps de réaction | Délai entre la commande et l'action du personnage, fenêtres de parade et d'esquive |
| Régulation émotionnelle | Vitesse de montée et de retombée du stress |
| Raisonnement | Déductions proposées, compréhension des mécanismes |
| Intelligence sociale | Lecture des émotions et des intentions des PNJ |

### 4.3 Compétences et savoirs

Appris par la pratique : techniques (escrime, équitation, chirurgie, forge…), langues, savoirs culturels par époque (monnaie, coutumes, codes sociaux), savoirs anachroniques (médecine, chimie, histoire).

- Un voyageur qui ignore les codes d'une époque éveille la **suspicion** des PNJ.
- Un savoir anachronique n'est utile que si l'on dispose des compétences et des ressources pour l'appliquer à l'époque visitée.

### 4.4 États temporaires

| État | Effets |
|---|---|
| Stress | Courbe de Yerkes-Dodson : un peu améliore la performance, trop la dégrade (vision en tunnel, tremblements, motricité fine et mémoire de travail réduites) |
| Fatigue aiguë | Due à l'effort, se récupère en minutes |
| Fatigue accumulée (sommeil) | Se récupère en jours ; réactions ralenties, perception dégradée, micro-sommeils |
| Faim, hydratation | La déshydratation dégrade la cognition avant la force ; le jeûne épuise les réserves |
| Douleur et blessures | Localisées (un bras blessé dégrade la visée et l'escalade), guérison réaliste, séquelles possibles |
| Température | Le froid dégrade la dextérité puis la cognition ; la chaleur réduit l'endurance |
| Immunité | Le voyageur n'est pas immunisé contre les maladies d'une époque, et peut en transmettre |

Règle de lisibilité : tout malus doit être **visible et explicable** pour le joueur.

### 4.5 Moteur de progression

**Adaptation avec plafond génétique** (pour les capacités) :

```
gain = taux × stimulus × récupération × (P_max − niveau) / P_max
```

**Loi puissance de l'apprentissage** (pour les compétences) :

```
performance = P_max − (P_max − P_0) × N^(−α)       // N = nombre de pratiques
```

**Surcompensation** : l'entraînement fatigue d'abord, les gains n'arrivent qu'après la récupération. Sans sommeil suffisant : stagnation, régression, blessures (surentraînement).

**Désentraînement** : ce qui n'est pas utilisé décline, à des vitesses différentes :

| Élément | Vitesse de perte |
|---|---|
| Endurance | Rapide (quelques semaines) |
| Force | Plus lente |
| Compétences motrices bien ancrées | Très lente |
| Savoirs déclaratifs (langues, faits) | Progressive sans pratique |

**Âge biologique** : gains rapides avant ~25 ans, plateau, puis déclin progressif (puissance et temps de réaction déclinent tôt, savoirs et jugement tard).

### 4.6 Le temps personnel, vraie ressource

- Le corps du voyageur vieillit selon son **temps personnel** vécu.
- Passer un an à s'entraîner en 1900 coûte un an d'âge biologique.
- Sauter dans le temps ne repose pas et ne fait pas progresser : on ne peut pas tricher avec le voyage.
- Le mode simulation (§6.2) accélère le temps du joueur, pas les lois du corps : un an simulé, c'est un an de vieillissement, de progression et d'usure.

### 4.7 Création de personnage

Deux méthodes sont disponibles :

- **budget de points** : le joueur répartit les plafonds génétiques ;
- **tirage aléatoire** : les plafonds sont tirés selon des distributions réalistes.

Dans les deux cas, on vise des profils complémentaires plutôt qu'un personnage excellent partout.

### 4.8 Blessures critiques et mort

| Mode | Conséquence |
|---|---|
| **Solo** | Retour au dernier point de sauvegarde |
| **En ligne, hardcore** | La mort est définitive : le joueur doit créer un nouveau personnage |
| **En ligne, permissif** | Le personnage ne meurt pas : il est retrouvé et soigné. Il se **réveille à l'hôpital un an plus tard**, dans son temps historique |
| **Explosion de la puce** (tous modes en ligne) | Le personnage **disparaît** : pas de corps, pas d'hôpital. Le joueur doit créer un nouveau personnage (§2.11). En solo, c'est une mort ordinaire |

**Point de sauvegarde (solo)** : un instantané composé de la version de la branche (graine + journal) et de l'état du fil du voyageur. C'est très léger.

**Mode permissif, sans magie** : l'année d'hospitalisation a des effets réalistes :

- le personnage vieillit d'un an ;
- il subit un **désentraînement** massif (fonte musculaire, perte d'endurance), à compenser par une rééducation ;
- des séquelles sont possibles selon la blessure ;
- pendant cette année, son corps est à l'hôpital dans l'histoire (journalisé) : le monde a évolué, ses biens et ses relations aussi.

**Mort d'une trace** : seule la mort du corps présent du voyageur compte. Si sa trace est tuée dans le passé par un autre joueur, il devient une anomalie (§2.10), il ne meurt pas.

---

## 5. IA des PNJ

### 5.1 Objectif

Les PNJ doivent pouvoir tenir **n'importe quel rôle**, y compris ceux que la génération procédurale invente.

Ce n'est pas possible avec un modèle unique qui déciderait de tout : il serait trop coûteux à entraîner et impossible à contrôler. La généralité vient de la **composition** : un agent générique combiné à des rôles décrits en données.

### 5.2 Architecture en couches

```
RÔLE (données) : objectifs, emploi du temps, savoirs, relations, actions autorisées, attributs
        │
        ▼
1. MOTIVATIONS        besoins (faim, sécurité, argent, statut), personnalité, émotions
2. PLANIFICATION      GOAP / HTN : enchaîne des actions pour atteindre un objectif
3. COMPÉTENCES        apprises : combat, poursuite, fuite, discrétion, déplacement en foule
                      scriptées : forger, vendre, cultiver, soigner…
4. SOCIAL / DIALOGUE  optionnel : modèle de langage
────────────────────
MÉMOIRE               ce que le PNJ a vu, sait et ressent envers chacun
```

**Un nouveau rôle, ce sont de nouvelles données, pas une nouvelle IA.**

### 5.3 Ce qui est entraîné

Seules les **compétences** (couche 3) sont apprises, et elles sont **partagées entre tous les rôles** :

- une politique par compétence, **conditionnée par le personnage** : elle reçoit ses capacités, ses états et son niveau de compétence. Un vétéran reposé et un paysan terrifié se comportent différemment avec le même modèle ;
- des politiques **conditionnées par l'objectif** (atteindre X sans être vu, poursuivre Y, tenir Z à distance) ;
- du **self-play** et des populations d'adversaires pour le combat ;
- un **apprentissage progressif** : duels simples, puis groupes, terrains variés, états dégradés.

Les modèles entraînés sont exportés au format **ONNX** et exécutés par le cœur.

### 5.4 Dialogue

Le dialogue utilise un **modèle de langage**, au choix du joueur :

| Choix | Fonctionnement | Coût |
|---|---|---|
| **Modèle local** | Un modèle ouvert téléchargé, exécuté sur la machine du joueur (llama.cpp via LLamaSharp). Le joueur choisit parmi une liste de modèles selon sa carte graphique | Gratuit, mais il consomme de la mémoire graphique au détriment du rendu |
| **Modèle en ligne** | **Uniquement via notre service hébergé**. Le joueur choisit le modèle parmi ceux que le service propose (par exemple Claude Haiku 5.5 ou Claude Sonnet 5.5). Pas de clé d'API personnelle | À notre charge (voir l'estimation ci-dessous) |
| **Sans modèle** | Dialogue systémique par modèles de phrases (ci-dessous) | Gratuit |

**Architecture** : une interface unique de **fournisseur de dialogue** côté client C#, avec une implémentation par type (local, chaque API en ligne, systémique). Ajouter un fournisseur ou un modèle ne touche rien d'autre.

**Le modèle de langage n'a aucune autorité sur le jeu.** Il a deux rôles :

1. **Comprendre** : traduire la phrase libre tapée par le joueur en une **intention structurée** (demander, marchander, menacer, mentir…, avec son sujet) ;
2. **Exprimer** : formuler la réponse du PNJ.

Entre les deux, c'est **le cœur** qui décide : les couches 1 et 2 de l'IA choisissent l'intention de réponse, et les règles tranchent les résultats (une tentative de persuasion réussit selon les capacités et les relations, pas selon l'éloquence du modèle). Conséquences :

- choisir tel ou tel modèle ne donne **aucun avantage** en jeu, et ne permet pas de tricher en ligne ;
- la **mémoire des PNJ** enregistre des faits et des intentions, pas du texte. Elle reste cohérente quel que soit le modèle utilisé par chaque joueur ;
- si aucun modèle n'est disponible (pas de connexion, machine trop faible), le jeu bascule automatiquement sur le **dialogue systémique**.

#### Dialogue systémique (repli sans modèle)

- Le joueur choisit une **intention** dans un menu contextuel : saluer, demander une information sur un sujet, marchander, convaincre, menacer, mentir, séduire, recruter…
- Le PNJ répond selon ses motivations, sa mémoire et ses relations, avec une **intention de réponse**.
- La réponse est rédigée à partir de **modèles de phrases** combinés et remplis avec les connaissances du PNJ, variés selon l'époque, le milieu social, le caractère et l'émotion. Les variantes régionales (picard, flamand des immigrés belges) sont possibles.
- Avantages : gratuit, hors ligne, déterministe, entièrement contrôlé.
- Limites : plus répétitif, et le joueur ne peut pas taper librement ses propres phrases.

#### Règles pour le modèle de langage

- il **exprime**, il ne décide pas : les couches 1 et 2 choisissent l'intention (refuser, se méfier, négocier), le modèle la formule ;
- sa sortie est **structurée** et validée par les règles du jeu ;
- il ne reçoit que la mémoire du PNJ et les **savoirs de son époque** (un PNJ de 1850 ne connaît ni l'avion ni Internet) ;
- seuls les PNJ en conversation avec un joueur l'utilisent. Les conversations entre PNJ utilisent des modèles de phrases ou restent abstraites, sinon le coût explose.

#### Estimation du coût (tarifs de l'API Anthropic au 06/10/2026)

Hypothèses par réplique d'un PNJ : environ 2 000 jetons en entrée (personnalité, mémoire, savoirs de l'époque, historique de la conversation) et environ 150 en sortie (réponse structurée).

| Modèle | Tarif (entrée / sortie, par million de jetons) | Coût par réplique | Coût par heure de dialogue intense (60 répliques) |
|---|---|---|---|
| Claude Haiku 5.5 | 0,10 $ / 0,50 $ | ≈ 0,0003 $ | ≈ 0,02 $ |
| Claude Sonnet 5.5 | 2 $ / 10 $ | ≈ 0,0055 $ | ≈ 0,33 $ |

Ordres de grandeur pour un monde persistant, avec Haiku 5.5 et 20 répliques par heure de jeu en moyenne : ≈ 0,006 $ par joueur et par heure, soit ≈ 0,25 $ par joueur pour 40 heures de jeu par mois. Pour 10 000 joueurs actifs, ≈ 2 500 $ par mois.

Options :

- **Solo** : un petit modèle local sur la machine du joueur (gratuit pour nous, mais il demande de la mémoire graphique et concurrence le rendu), ou l'API.
- **En ligne** : l'API (coût proportionnel à l'usage), ou un modèle hébergé sur nos propres serveurs GPU (coût fixe, rentable à partir d'un certain volume).
- **Leviers de réduction** : mise en cache des parties fixes du contexte, résumés de mémoire compacts, quota de répliques par joueur, modèles de phrases pour les échanges banals (saluer, marchander simplement).

### 5.5 Niveau de détail de l'IA

| Situation du PNJ | IA utilisée |
|---|---|
| En interaction avec un joueur | Les 4 couches, compétences apprises, dialogue |
| Visible mais pas en interaction | Motivations, planification, compétences simplifiées |
| Hors de vue, dans la zone chargée | Planification grossière, emploi du temps |
| Hors zone | Simulation macro uniquement |

Un PNJ peut changer de niveau à tout moment sans incohérence, parce que son état et sa mémoire sont stockés dans le cœur, pas dans l'IA.

### 5.6 Cohérence avec le modèle temporel

- La mémoire des PNJ appartient à la branche : elle est régénérée avec le monde. Seuls les voyageurs se souviennent des versions précédentes.
- La simulation macro n'utilise **aucun réseau de neurones**, pour rester strictement déterministe et rapide. Les réseaux de neurones ne servent qu'en temps réel, dans les zones de joueurs, sous l'autorité du serveur.

### 5.7 Le personnage du joueur en mode simulation

En mode simulation (§6.2), le personnage du joueur est piloté par **la même IA que les PNJ**. Les instructions du joueur jouent le rôle de la fiche de rôle : routine, objectifs, limites (« ne pas se battre », « ne pas dépenser plus de X »), conditions d'interruption.

Un seul système d'IA sert donc aux PNJ, au personnage en simulation et, plus tard, aux traces des voyageurs (§2.10).

---

## 6. Multijoueur

### 6.1 Principes

- Le jeu en ligne est un **monde persistant**.
- **Serveur autoritaire** : il possède les branches, leurs journaux, et ordonne toutes les interventions.
- Les clients peuvent exécuter le même cœur pour générer localement les zones (génération déterministe) et anticiper les actions ; le serveur vérifie les résultats importants.
- Un **identifiant de branche** suffit pour rejoindre, partager ou comparer des lignes temporelles.

### 6.2 Modes réaliste et simulation

Le temps historique de chaque voyageur avance selon son mode (§2.10). Ce fonctionnement est le même en solo et en ligne.

**Mode réaliste** : le joueur contrôle directement son personnage, et le temps s'écoule à **vitesse normale**.

**Mode simulation** :

1. le joueur donne des **instructions** à son personnage : emploi du temps, objectifs, limites, conditions d'interruption ;
2. il **accélère le temps** (multiplicateur au choix, plafonné **[à calibrer]**) ;
3. le personnage vit sa vie, piloté par l'IA (§5.7) ;
4. le jeu **repasse automatiquement en mode réaliste** dès qu'un événement important arrive au personnage.

Événements qui interrompent la simulation (liste configurable en partie par le joueur) :

- blessure, maladie, agression, danger imminent ;
- un autre voyageur arrive au même instant que le personnage et interagit avec lui ;
- une modification de l'histoire change directement la situation du personnage (son logement disparaît, un proche n'existe plus…) ;
- une condition définie par le joueur (« préviens-moi si quelqu'un parle de X »).

**Résolution accélérée** : en mode simulation, le personnage n'est pas simulé tick par tick. Ses journées sont découpées en **blocs d'activité** (travailler, s'entraîner, dormir, voyager) dont les résultats sont calculés directement par les règles du cœur : progression, fatigue, argent, relations, et tirages de risques. Quand un tirage produit un événement, la simulation détaillée reprend à cet instant précis. C'est ce qui rend l'accélération possible sans coût serveur démesuré.

**Joueur déconnecté** : son personnage **disparaît** de l'histoire à cette date (disparition journalisée). Sa puce reste active et **décompte le temps en heures réelles** : à la reconnexion, il réapparaît dans la même branche, à la date de déconnexion plus la durée réelle de son absence. Il revient donc dans un présent qui a avancé.

- Il réapparaît dans la version actuelle de la branche : si d'autres joueurs ont modifié l'histoire entre-temps, il le découvre dans le journal des divergences, et sa position est réconciliée si nécessaire (§2.6).
- Le point de réapparition est soumis à la capture W (§2.10) : si un joueur de la ligne se trouve à moins de W, il réapparaît à son instant.
- Pendant l'absence, le corps est en suspension : pas de vieillissement, pas de faim ni de fatigue **[proposé]**.

### 6.3 Accélération en groupe

- Chaque membre d'un groupe peut **proposer** une accélération vers une date cible.
- Chacun est **libre de suivre** ou non.
- Un membre peut suivre **en retard**, après le départ du groupe.

Les arrivées décalées sont réglées par les mécanismes du §2.10, pour un groupe comme pour des joueurs qui ne se connaissent pas :

- **changement de ligne** : la capture W place les suivants au même instant que les premiers arrivés ;
- **accélération dans la même ligne** : la capture W arrête celui qui accélère à l'instant des premiers arrivés ;
- **rejoindre un joueur précis** : synchronisation de puce (§2.8).

Dans les deux cas, les premiers voient les suivants arriver en direct.
### 6.4 Interactions entre époques

- **Coopération** : messages et caches laissés dans le passé, investissements (fonder, planter, placer), demandes de « correction » du passé par un allié.
- **Compétition** : les joueurs du passé ont une grande influence, mais sur un résultat chaotique et difficile à viser ; ceux du futur disposent de plus d'informations, mais n'agissent que sur leur époque et après.
- **Protection** : le décrochage (§2.7).

---

## 7. Architecture technique

### 7.1 Couches

```
┌───────────────────────────────────────────────────────────────┐
│ CLIENT GODOT (C#) : rendu 2.5D puis 3D, interface, entrées,    │
│ audio, dialogue (modèle de langage)                            │
│ OUTILS (C#) : éditeur de rôles, visualiseur de branches        │
└──────────────┬────────────────────────────────────────────────┘
               │ interface C (csbindgen) : commandes ↓  état et évènements ↑
┌──────────────▼────────────────────────────────────────────────┐
│ CŒUR (Rust) : modèle temporel, simulation macro, génération   │
│ micro, règles, personnages, IA (couches 1 à 3, inférence ONNX),│
│ mémoire des PNJ                                                │
└──────┬─────────────────────────────────────┬──────────────────┘
       │                                     │ PyO3 (API par lots)
┌──────▼───────────────┐          ┌──────────▼─────────────────┐
│ SERVEUR (Rust)       │          │ ENTRAÎNEMENT (Python)       │
│ réseau + cœur        │          │ PyTorch → export ONNX       │
└──────────────────────┘          └────────────────────────────┘
```

### 7.2 Répartition des langages

| Partie | Langage | Raison |
|---|---|---|
| Cœur de simulation | Rust | Performance, régularité, sûreté, intégration Python |
| Serveur | Rust | Essentiellement le cœur avec du réseau, sans changement de langage |
| Entraînement des IA | Python | Écosystème d'apprentissage automatique (PyTorch) |
| Client de jeu (Godot 4) | C# | Prise en charge officielle par Godot, code d'interface simple |
| Outils | C# | Développement rapide |

**Règle de partage** : si la simulation en a besoin pour produire le même résultat sans affichage, c'est dans le cœur Rust. Si ça ne fait qu'afficher, transmettre ou outiller, c'est en C#.

### 7.3 Règles de la frontière du cœur

1. Le cœur **ne dépend d'aucun moteur graphique** et ne sait pas qu'il est affiché.
2. Le monde du cœur est **en 3D** (x, y, z, hauteurs, étages), même quand le rendu est en 2.5D.
3. Déplacements, collisions, lignes de vue et chutes sont calculés **dans le cœur**. La physique du moteur graphique n'est que décorative.
4. Les timings de gameplay (impact d'un coup, fenêtre de parade) sont des **données du cœur** ; les animations s'y synchronisent.
5. Aucune règle de jeu dans le C#.

### 7.4 Interface entre langages

- **Commandes en lot** par tick (C# → Rust) ; **évènements et état** (Rust → C#) dans des tampons mémoire partagés, lus sans copie.
- Jamais un appel par entité ou par propriété.
- C# ne manipule que des **identifiants** (`EntityId`, `BranchId`…) ; les données restent dans Rust.
- Format : structs `#[repr(C)]` pour les données de taille fixe, format binaire (FlatBuffers ou équivalent) pour les données variables.
- L'API d'entraînement (Python) suit le même principe : `step_batch(actions[N × agents]) → observations, récompenses`, avec des tableaux numpy partagés sans copie.

### 7.5 Déterminisme

Indispensable pour les branches, le serveur et l'entraînement :

- générateur aléatoire maison (PCG ou xorshift), jamais celui du langage ou du moteur ;
- simulation par **ticks**, sans dépendre de l'heure réelle ;
- **entiers ou virgule fixe** dans la simulation macro, pour un résultat identique sur toutes les machines ;
- itérations dans un ordre défini (pas de parcours de table de hachage non ordonnée) ;
- tests automatiques de déterminisme : même graine et même journal donnent un état identique, octet pour octet.

### 7.6 Rendu : de la 2.5D à la 3D

La 2.5D est réalisée **en vraie 3D** dans Godot :

- une scène 3D avec une **caméra orthographique** à angle fixe (vue isométrique ou de trois quarts) ;
- des **modèles 3D simples** (low poly) plutôt que des sprites dessinés ;
- le même éclairage, les mêmes collisions et la même scène que la future version 3D.

Pour passer à la 3D, il suffira d'ajouter une **caméra libre en perspective** et des modèles plus détaillés. Il n'y aura aucune ressource graphique à refaire de zéro, contrairement à une 2.5D en sprites.

### 7.7 Risques de performance

Le monde persistant concentre le principal risque technique : les modifications du passé sont fréquentes, et chacune déclenche un rebasage.

- **Coût du rebasage** : il croît avec le nombre d'interventions postérieures. Parade : suivre les **dépendances** entre interventions et entités, pour ne revalider que celles qui touchent ce qui a réellement changé (dans le cône causal).
- **Coût du recalcul macro** : des **instantanés** de l'état macro à intervalles réguliers permettent de reprendre le calcul au plus près de la modification.
- **Modifications proches du présent** : ce sont les plus nombreuses, et elles sont bon marché (peu de temps à recalculer).
- **Fragmentation en branches** (décrochages) : les branches sans voyageur sont supprimées ; un nombre maximal de branches par serveur pourra être fixé si nécessaire.

### 7.8 Structure du dépôt **[proposé]**

```
time_switcher/
├── docs/                      # documentation de conception
├── core/                      # workspace Cargo (Rust)
│   └── crates/
│       ├── sim/               # le cœur
│       ├── ffi/               # interface C pour le C# (csbindgen)
│       ├── py/                # module Python pour l'entraînement (PyO3)
│       ├── server/            # serveur en ligne
│       └── cli/               # outil console de test et de démonstration
├── training/                  # Python : environnements, entraînement, export ONNX
├── models/                    # modèles ONNX entraînés
└── game/                      # C# (.NET)
    ├── TimeSwitcher.Bridge/   # enveloppe C# autour de l'interface native
    ├── TimeSwitcher.Tools/    # éditeur de rôles, visualiseur de branches
    └── client-godot/          # client Godot 4 (C#)
```

---

## 8. Périmètre initial et feuille de route

### 8.1 Périmètre initial

| Version complète | Périmètre initial |
|---|---|
| Destinations de voyage libres | **5 époques d'arrivée** : 1850, 1900, 1950, 2000, 2020 (l'histoire et les horloges restent continues) |
| Monde entier | **Roubaix et ses environs**, le reste du monde suivant l'histoire réelle |
| Tous les fournisseurs de dialogue | **Un modèle local et un modèle en ligne**, avec le dialogue systémique en repli |
| Interventions libres | **Un catalogue** : tuer, sauver, donner, prendre, construire, détruire, convaincre |
| En ligne à grande échelle | **Solo**, puis coopération à 2–4 joueurs |
| Divergence chaotique fine | **Règles de propagation simples** sur les variables macro |

### 8.2 Étapes

1. **Prototype du moteur temporel** (Rust, console) : simulation macro de 1850 à 2020 ; une intervention en 1850 ; vérification que 1950 et 2020 changent de façon **cohérente et lisible**. *C'est le test qui valide le concept.*
2. **Branches et voyage** : création, versions, rebasage, interventions orphelines, décrochage ; tests de déterminisme.
3. **Génération micro** : description sémantique des zones à partir de l'état macro.
4. **Personnage** : capacités, états, progression de base ; déplacements et collisions dans le cœur ; horloge individuelle et mode simulation par blocs d'activité.
5. **Client Godot 2.5D** (3D à caméra orthographique) : se promener en 1850 puis en 2020 et voir la différence.
6. **IA des PNJ** : motivations et planification, puis une première compétence apprise (combat) avec l'API par lots.
7. **Coopération en ligne** : serveur Rust, 2 joueurs à 2 époques.
8. **Extension** : plus d'époques, de régions, d'interventions, de rôles ; dialogue ; passage à la 3D.

---

## 9. Questions ouvertes

| Sujet | Question |
|---|---|
| Capture W | La capture s'applique-t-elle à toute la ligne, ou seulement aux joueurs proches dans l'espace ? |
| Lieu d'arrivée | Lors d'une capture, le lieu reste-t-il celui choisi par le voyageur, ou devient-il celui du joueur qui capte ? |
| Exploits temporels | Arbitrer les parades proposées au §2.12 |
| Traces d'un personnage disparu | Après une explosion, ses traces passées restent-elles dans l'histoire, ou sont-elles effacées ? |
| Autres voyageurs dans une branche créée | Quand un bond vers le passé crée une branche, les autres joueurs présents à cette date y ont un corps. Que devient-il après la date du bond (piloté par l'IA comme un double, ou absent) ? |
| Joueur déconnecté | Le corps vieillit-il pendant l'absence (proposé : non) ? |
| Traces | Faut-il des protections contre le harcèlement via les traces (lois et justice en jeu, serveurs aux règles différentes) ? |
| Financement du dialogue | Le service hébergé est à notre charge : abonnement, modèles premium payants, quota ? |
| Vitesse du cône causal et chaos | Valeurs de v et λ, à trouver par prototypage |
| Accélération maximale | Plafond du multiplicateur en mode simulation |

---

## 10. Glossaire

| Terme | Définition |
|---|---|
| **Branche** | Une ligne temporelle : graine + journal d'interventions |
| **Version de branche** | Numéro incrémenté à chaque intervention ; une version ne devient une branche distincte qu'en cas de décrochage |
| **Voyageur / fil** | Personnage extérieur aux branches, qui conserve corps, inventaire et mémoire |
| **Intervention** | Action d'un voyageur qui modifie durablement le monde, enregistrée dans le journal |
| **Journal** | Liste des interventions d'une branche, triée par temps historique puis méta-temps |
| **Rebasage** | Rejeu des interventions postérieures à une modification, sur la nouvelle histoire |
| **Intervention orpheline** | Intervention dont les préconditions ne sont plus remplies après un rebasage |
| **Décrochage** | Choix d'un voyageur de rester dans la version actuelle d'une branche |
| **Cône causal** | Zone de l'espace-temps qu'une intervention peut influencer |
| **Divergence chaotique** | Écart croissant avec le temps entre l'ancienne et la nouvelle histoire |
| **Anomalie** | Voyageur dont l'origine n'existe plus dans la branche où il se trouve |
| **Méta-temps** | Temps réel des joueurs et du serveur, qui avance toujours |
| **Horloge individuelle** | Temps historique propre à chaque voyageur, qui avance selon son mode de jeu |
| **Trace** | Passage d'un voyageur dans l'histoire, rejoué pour ceux qui traversent ces dates plus tard |
| **Mode réaliste / mode simulation** | Contrôle direct à vitesse normale / vie pilotée par l'IA selon des instructions, à vitesse accélérée |
| **Puce temporelle** | Implant qui permet au voyageur de voyager dans le temps |
| **Bond** | Déplacement dans le temps sans changer de ligne |
| **Capture W** | Attraction d'un voyageur qui avance ou arrive dans une ligne vers l'instant du joueur le plus proche, si l'écart est inférieur à W (24 h) |
| **Synchronisation de puce** | Action de deux voyageurs qui se rejoignent dans la même ligne, au même instant |
| **Identifiant d'objet** | (ligne d'origine, numéro, version) ; l'identité est (ligne d'origine, numéro), la version distingue les exemplaires |
| **Paradoxe** | Action qui fait coexister deux exemplaires d'une même identité |
| **Objet paradoxal** | Tout exemplaire autre que la version 1 ; il le reste jusqu'à sa destruction |
| **Température de la puce** | Jauge qui monte tant qu'on porte des objets paradoxaux et ne redescend qu'une fois qu'on s'en est débarrassé ; à saturation, la puce explose |
| **Remplacement** | Un voyageur qui revient dans une ligne prend la place de sa version antérieure à ce moment-là |
| **Écart à l'histoire (δ)** | Différence entre l'état d'une branche et l'histoire réelle ; nulle dans la branche d'origine |
| **Bloc d'activité** | Unité de résolution accélérée en mode simulation (une journée de travail, une nuit de sommeil…) |
| **Temps personnel** | Temps vécu par un voyageur ; détermine son vieillissement |
| **Simulation macro** | Simulation déterministe et grossière de l'histoire (régions × décennies) |
| **Génération micro** | Génération procédurale des détails à partir de l'état macro |
| **Journal des divergences** | Interface indiquant au joueur ce qui a changé après une modification |
