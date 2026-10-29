# Vie, synchronisation, bilan et mort

> Fichier du **socle** : modifié uniquement par la session principale (voir [README](README.md)).

## 1. Deux barres de vie

On distingue :

- les **points de vie réels** du personnage : sa santé dans le monde, qui ne baisse qu'avec les blessures, maladies et accidents ;
- les **points de vie du joueur** : ceux qui comptent pour le joueur pendant l'incarnation.

Plus le joueur se désynchronise, plus sa barre de vie **s'écarte** de celle du personnage. Formule (validée) :

```
synchronisation S : de 1 (parfaite) à 0 (totale désynchronisation)
PV du joueur = PV réels × S
```

| Événement | Effet |
|---|---|
| Le personnage est blessé | Les deux barres baissent |
| Le joueur agit contre la nature du personnage | Seule la barre du joueur baisse (S diminue), du coût calculé par la fonction unique des signatures ; un acte découpé en petits gestes coûte autant que l'acte entier ([revues/signatures.md](revues/signatures.md), G9) |
| Le joueur agit conformément au personnage | S remonte progressivement, la barre du joueur se rapproche de la barre réelle |
| La barre du joueur tombe à zéro | **Le joueur est éjecté** ; le personnage garde ses PV réels et l'IA reprend le contrôle |
| Le joueur demande un acte dont le coût estimé dépasse la synchronisation restante | **L'acte n'a pas lieu** (point de rupture) : le personnage hésite puis renonce par un geste naturel, le joueur est éjecté et l'IA reprend dans le même tick |
| Les PV réels tombent à zéro | Le personnage meurt (§3 et §4) |

**Point de rupture** (décidé, [revues/equilibrage.md](revues/equilibrage.md), E1) : les actes **extrêmes pour le profil** d'un personnage (homicide ou violence grave à contre-emploi, don de plus de la moitié de son patrimoine à quelqu'un qui n'est pas un intime, facilitation d'un crime contre soi, son employeur ou ses proches, mise en danger mortelle évitable pour un personnage prudent) ont un coût supérieur à 1 : ils sont **impossibles pour lui**, même à synchronisation parfaite. Quatre **planchers de rupture**, posés sur les effets des actes et évalués sur le profil de base du personnage, garantissent par construction que ces actes coûtent plus que 1 ; une justification que le joueur ou son cercle a lui-même provoquée ne les lève pas ([revues/signatures.md](revues/signatures.md), G6, G7). Un tel acte n'est **pas proposé** dans les listes d'actions ; seuls les gestes de la barre de combat, qui restent disponibles, portent alors l'**indicateur de rupture**, toujours visible. Un acte qui ne dépasse que la synchronisation restante reste proposé, avec ce même indicateur (décidé, clôture de l'équilibrage C6 ; [revues/signatures-contenu.md](revues/signatures-contenu.md), M8). **Cible liée par un secret** (décidé, clôture de l'équilibrage R1) : la **classe apparente** de la relation, celle que le joueur peut connaître, sert au niveau affiché, à l'indicateur affiché et au choix de proposer l'acte ; le vrai coût ne s'applique qu'à l'exécution, où l'acte n'a pas lieu et le joueur est éjecté s'il dépasse la synchronisation restante ou 1 (E1). Cette éjection ne compte ni pour la récidive (E3) ni dans les refus répétés du point de rupture. *Pourquoi : sinon l'absence d'un acte dans la liste révèle gratuitement un lien caché.* *Pourquoi : ce qu'une pénalité ne peut pas réparer doit être empêché ; c'est l'escalade du combat généralisée à tous les actes.*

Un joueur totalement désynchronisé est donc éjecté alors qu'il laisse beaucoup de vie au personnage. La désynchronisation **ne blesse jamais** le personnage : il est impossible de tuer un personnage en agissant contre sa nature.

L'interface affiche les deux barres, pour que le joueur voie l'écart se creuser : barres empilées, écart hachuré, sans chiffres par défaut, avec une **alerte avant l'éjection** ([domaines/interface.md](domaines/interface.md), §2.2). Quitter le personnage sous le seuil d'alerte, ou dans les 5 minutes de jeu qui suivent un acte dissonant grave, **compte comme une éjection** ([revues/equilibrage.md](revues/equilibrage.md), E2 : *l'alerte doit servir à se reprendre, pas à fuir la pénalité*).

Les **PV réels** sont calculés à partir des blessures localisées, de la perte de sang et des maladies, avec des seuils de choc, d'état critique et de mort ([domaines/combat.md](domaines/combat.md), §2.2).

Le personnage persistant n'a pas de synchronisation : sa barre de vie est sa vie réelle.

## 2. Bilan du joueur

Le **bilan du compte** est alimenté par :

- chaque **incarnation** d'un personnage jouable : points de synchronisation et comportement pendant l'incarnation, évalués quand le joueur quitte le personnage, en est éjecté, ou quand le personnage meurt ;
- les **actions du personnage persistant**.

**Fin d'incarnation par une sortie de la carte** : quand un joueur conduit le personnage qu'il incarne jusqu'à une sortie de l'agglomération, une confirmation lui est demandée ; s'il confirme, l'incarnation se termine avec un bilan normal, et l'IA choisit entre un séjour court et cohérent à l'extérieur et un **demi-tour**, qu'elle fait si le personnage a une obligation dans les 12 heures (service, garde, audience, scrutin, enfant à prendre) ([domaines/ia-pnj.md](domaines/ia-pnj.md), §9.5 ; [revues/equilibrage.md](revues/equilibrage.md), E18).

Une désynchronisation volontaire et répétée pèse lourdement sur le bilan. Un homicide ou une violence grave **à contre-emploi** compte comme une désynchronisation volontaire grave. La mort d'un personnage incarné est jugée **imprudente** si elle diffère du réflexe et de la décision que l'IA en observation aurait choisis ; Le bilan reste **provisoire une heure de jeu** après la fin de l'incarnation : une mort survenue dans les 10 minutes après une déconnexion en plein affrontement y est comptée ([domaines/combat.md](domaines/combat.md), §11), et une **facilitation** (porte ouverte, clé ou code remis, alarme coupée, absence annoncée, obligation de garde abandonnée) exploitée par un crime pendant cette heure y est comptée comme **complicité** ([revues/equilibrage.md](revues/equilibrage.md), E4 : *un acte se juge sur ce qu'il a permis, sinon tout crime se découpe en gestes anodins*). Une désynchronisation volontaire grave coûte 25 % de l'expérience du niveau, doublée à chaque récidive dans les 30 jours (E3). Le bilan se traduit en expérience ([04-joueur.md](04-joueur.md), §1.1).

## 3. Mort du personnage persistant

Il se réveille **à l'hôpital public**, ou **à la clinique privée s'il en a les moyens**, avec un **malus qui dépend de la cause de sa mort** (validé ; valeurs exactes à calibrer). S'il était **interpellé, poursuivi par la police ou recherché** au moment de sa mort, il se réveille à l'hôpital public **sous la garde de la police**, et la garde à vue accélérée commence à sa reconnexion. En **état critique**, il ne disparaît jamais à la déconnexion : son sort est simulé jusqu'au bout ([revues/equilibrage.md](revues/equilibrage.md), E21 : *mourir ne doit être ni une évasion, ni une façon d'éviter la mort qui vient*).

Dans le monde, le personnage persistant « mort » est en réalité **réanimé**. En ligne, une **blessure grave non mortelle** suit la même règle que la convalescence théorique (sans malus psychologique ni compte dans les morts répétées) ; les blessures légères guérissent en temps réel, connecté ou non ([domaines/combat.md](domaines/combat.md), §2.4 et §2.5).

### 3.1 Malus selon la cause

Les durées sont en **jours de jeu**. Les malus physiques diminuent progressivement pendant la convalescence ; les malus psychologiques diminuent avec le repos et l'absence de nouvel événement traumatisant.

| Cause | Malus physiques | Malus psychologiques | Convalescence |
|---|---|---|---|
| **Coups, agression** | Force −20 %, endurance −20 % | Stress accru en présence de l'agresseur ou de personnes qui lui ressemblent | 7 jours |
| **Arme blanche** | Force et endurance −30 %, saignement à la reprise d'effort intense | Méfiance accrue envers les inconnus proches | 14 jours |
| **Arme à feu** | Mobilité −40 %, endurance −40 %, douleur à l'effort | Sursaut et stress aux bruits forts | 30 jours |
| **Chute, accident de la route** | Fracture : mobilité −50 % (jambe) ou coordination −50 % (bras), selon la zone touchée | Appréhension de la cause (conduite, hauteur) | 30 à 45 jours |
| **Brûlure** | Dextérité −30 %, sensibilité à la chaleur | Peur du feu | 21 jours |
| **Noyade, asphyxie** | Endurance −30 % | Peur de l'eau ou des espaces confinés | 10 jours |
| **Intoxication** (poison, produit dangereux) | Endurance −30 %, nausées, réactions ralenties | Méfiance envers la nourriture d'inconnus | 7 jours |
| **Maladie** | Endurance −40 %, fatigue accrue | — | Selon la maladie |

### 3.2 Solo et en ligne

| Mode | Fonctionnement |
|---|---|
| **Solo** | Convalescence normale : les malus diminuent au fil des jours de jeu, et le joueur peut faire avancer le temps |
| **En ligne** | **Pas de convalescence** : le personnage sort de l'hôpital immédiatement, ses blessures sont considérées comme soignées. En contrepartie, ses attributs baissent **d'autant que pendant la convalescence théorique** : la perte de forme (désentraînement) qu'auraient causée 30 jours d'immobilisation s'applique d'un coup. Le joueur doit ensuite regagner ces attributs en s'entraînant. Les **malus psychologiques** s'appliquent quand même, pendant la durée théorique de la convalescence, en temps réel |

Le calcul de la baisse en ligne utilise le même modèle de désentraînement que la progression réaliste, appliqué à la durée théorique de convalescence et à la gravité de la blessure (une fracture de la jambe fait plus perdre en mobilité et en endurance qu'en force des bras).

### 3.3 Règles communes

- **Désentraînement** : la convalescence fait perdre de la forme physique, comme dans la progression réaliste. En ligne, cette perte est appliquée immédiatement.
- **Rééducation (solo)** : une activité adaptée (kinésithérapie, marche) raccourcit la convalescence ; un effort trop intense la rallonge ou aggrave la blessure.
- **Morts répétées** : chaque nouvelle mort dans un délai court (par exemple 30 jours) allonge la convalescence de 50 % (en ligne : augmente la baisse d'attributs de 50 %) et peut laisser une **séquelle permanente** légère (cicatrice, douleur chronique, légère perte d'une capacité).
- **Conséquences sociales** : frais d'hôpital (forfait journalier modeste à l'hôpital public, nettement plus cher à la clinique privée ; une dette n'est pas effacée par la mort, [domaines/economie.md](domaines/economie.md), §11.6), absence au travail, et, si la mort est liée à un crime, une possible visite de la police.
- **Objets** : ce que le personnage portait peut avoir été perdu, volé, ou saisi comme pièce à conviction selon les circonstances.

## 4. Mort d'un personnage jouable

C'est **toujours une mort définitive** dans le monde.
