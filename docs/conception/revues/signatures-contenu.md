# Signatures : ce qui reste écrit à la main

> Version 1.0.1, 29/10/2026 : arbitrage **A7** de la session principale (décidé). Il n'existe ni geste libre, ni danse, ni émote (« aucun geste gratuit », 04 §7, interface §5.1 et §5.2) : la famille des « gestes libres » devient celle des « actes excentriques permis par la surface de jeu (interface §5.2) ». A18 (danser seul), A19 (s'allonger sur la chaussée) et A25 (uriner en public) sont **sans objet en première version** et sortent du décompte, qui passe à 235 actes et 25 annotations (10,6 %). *Pourquoi : la règle « aucun geste gratuit » protège l'indistinguabilité des pilotes, et une annotation ne doit pas faire croire qu'un acte existe.*
> Version 1.0, 29/10/2026. Complément de la [revue des signatures](signatures.md), écrit par la même session à la demande de l'utilisateur : « réalise tous les actes qui sont à faire à la main ». Ce document remplit chaque ligne du tableau « Ce qui reste écrit à la main » (signatures.md, §3.12). Comme les autres revues, c'est un document **à part** : la session principale le relit et l'intègre. Aucun autre fichier n'a été modifié, sauf une ligne de renvoi en tête de signatures.md.
> Toutes les valeurs sont des **valeurs de départ** au sens du §4.5 de la revue. Le calibrage par comparaisons humaines part de ces valeurs et reste près d'elles ; les ancres et les planchers sont des contraintes dures. Elles sont donc toutes **[à calibrer]**, sauf mention contraire.
> Les choix nouveaux ou précisés sont marqués **(décidé)** et numérotés M1 à M18 (§1.3), chacun avec sa raison.
> Ces valeurs ont été vérifiées avec un modèle de calcul jetable, hors du dépôt, qui suit les formules de la revue :
> - les 46 ancres tombent dans leur bande pour les 12 archétypes ;
> - les 6 échelles déclarées sont monotones pour les 19 683 profils de la grille 3⁹ ;
> - les planchers donnent bien la rupture.
>
> À la phase 1, `ts-data check` refera ces vérifications sur la vraie fonction `appraise`.

## Sommaire

0. [Ce que contient ce document](#0-ce-que-contient-ce-document)
1. [Conventions et précisions apportées à la revue](#1-conventions-et-précisions-apportées-à-la-revue)
2. [La grammaire des effets : valeurs de départ](#2-la-grammaire-des-effets--valeurs-de-départ)
3. [Justifications et planchers : valeurs de départ](#3-justifications-et-planchers--valeurs-de-départ)
4. [Les dériveurs](#4-les-dériveurs)
5. [Annotations et dérogations des actes](#5-annotations-et-dérogations-des-actes)
6. [Les archétypes et les contextes de référence](#6-les-archétypes-et-les-contextes-de-référence)
7. [Les ancres](#7-les-ancres)
8. [Les échelles déclarées](#8-les-échelles-déclarées)
9. [Les gabarits d'explication](#9-les-gabarits-dexplication)
10. [Ce qui reste à faire par des humains](#10-ce-qui-reste-à-faire-par-des-humains)
11. [Modifications proposées au socle, aux domaines et à la revue](#11-modifications-proposées-au-socle-aux-domaines-et-à-la-revue)

---

## 0. Ce que contient ce document

| Ligne du §3.12 de la revue | Section | Volume écrit ici | Fichier de contenu prévu (phase 1) |
|---|---|---|---|
| Grammaire : effets, projections, courbes de gravité, tolérance et poids, facteurs de cible | §2 | 199 paramètres | `data/content/conduct/grammar.ron` |
| Justifications (12) et planchers de rupture (4) | §3 | 53 paramètres | `data/content/conduct/justifications.ron`, `floors.ron` |
| Dériveurs, de l'effet mécanique à l'effet élémentaire | §4 | 32 règles | Code des descripteurs de chaque greffon et dériveurs statiques de `ts_data` |
| Annotations d'acte | §5.2 | 25 actes sur les 235 recensés (10,6 %) | Dans le fichier RON de l'acte lui-même (champ `conduct`) |
| Dérogations bornées | §5.3 | 3 actes (1,3 %) | Idem |
| Archétypes et contextes de référence | §6 | 12 profils, 3 contextes | `data/content/conduct/archetypes.ron` |
| Ancres du concepteur | §7 | 46 actes × 12 archétypes | `data/content/conduct/anchors.ron` |
| Échelles déclarées | §8 | 6 échelles, 24 marches | `data/content/conduct/scales.ron` |
| Gabarits d'explication | §9 | 66 entrées, 3 formes chacune | `data/content/conduct/explain.fr.ron` |

Au total, environ **250 paramètres** de grammaire, de justifications et de planchers sont écrits à la main. La revue en annonçait ≈ 220 ; l'écart vient de la table des facteurs de cible, complétée pour les 22 effets (§2.3). Viennent s'y ajouter :
- 32 règles de dérivation ;
- 25 annotations d'une ligne ;
- 3 dérogations ;
- 46 ancres ;
- 66 gabarits.

Il n'y a **aucune signature complète** écrite à la main.

*Pourquoi des fichiers séparés par nature : les ancres et les archétypes appartiennent au concepteur ; la grammaire, au calibrage ; les gabarits, à la traduction. Chacun se relit et se versionne à son rythme.*

---

## 1. Conventions et précisions apportées à la revue

### 1.1 Les axes et leur sens

Chaque axe va de −1 à +1 (stocké en millièmes). Le pôle −1 est **toujours le premier** des pôles nommés au §2 de [04-joueur.md](../04-joueur.md).

| Code | Axe | −1 | +1 |
|---|---|---|---|
| `EI` | Orientation de l'énergie | Introversion | Extraversion |
| `SN` | Perception | Sensation | Intuition |
| `TF` | Décision | Pensée | Sentiment |
| `JP` | Organisation | Jugement | Perception |
| `STAB` | Stabilité émotionnelle | Anxieux, impulsif | Calme, maître de soi |
| `BIEN` | Bienveillance | Cruauté | Compassion |
| `REGL` | Respect des règles | Hors-la-loi | Légaliste |
| `HON` | Honnêteté | Manipulation | Franchise |
| `VIOL` | Violence | Pacifisme | Usage de la force |

Un effet **tire** un axe vers l'un de ses pôles ; on note le sens par le signe (`VIOL+` tire vers la force, `HON−` vers la manipulation). La position qui compte pour le coût, `q`, est la position du personnage vers le pôle **opposé** : pour `HON−`, `q` est l'honnêteté ; pour `VIOL+`, c'est l'opposé de la violence (un pacifiste à −0,8 a `q` = 0,8).

### 1.2 Les courbes (décidé, M1)

Toutes les courbes de la grammaire (gravité, tolérance, poids) sont **linéaires par morceaux, à trois nœuds**, dont les **positions sont fixes** et les **valeurs calibrées**. Entre deux nœuds, la valeur est interpolée en ligne droite.
- La gravité vaut 0 en 0.
- La tolérance est constante sous son premier nœud.
- Le poids vaut 0 en 0.
- Au-delà du dernier nœud, chaque courbe garde sa dernière valeur.

*Pourquoi :*
- *le calcul reste exact en virgule fixe ;*
- *la monotonie se réduit à une contrainte d'ordre entre trois nombres ;*
- *le concepteur peut lire la courbe (« à 10 %, 40 % et 100 % de magnitude, la gravité vaut… »).*

### 1.3 Précisions et corrections de la revue (décidé)

Le travail de saisie et les vérifications ont fait apparaître ce qui suit. Ces précisions remplacent les passages correspondants de la revue : la §11.1 les reprend fichier par fichier.

| N° | Décision | Raison |
|---|---|---|
| **M1** | Courbes linéaires par morceaux à trois nœuds fixes (§1.2). | Exactes en virgule fixe, monotones par construction, lisibles. |
| **M2** | La magnitude de `LawViolation` suit la **peine type** du barème (combat §8.5), et non la peine maximale (barème au §2.2). Les circonstances aggravantes **propres à l'auteur** (arme, réunion, préméditation, masque, qualité d'agent public) ajoutent 0,05 chacune, au plus 0,15. Celles qui tiennent à la **victime** (mineure, vulnérable, conjoint, policier) ne s'ajoutent pas : elles passent déjà par la classe de cible. | La peine maximale rapprochait trop un vol simple d'un cambriolage. Compter deux fois la vulnérabilité de la victime aurait doublé son poids. |
| **M3** | Pour les effets de valeur (`Appropriation`, `PropertyDamage`, `Exploitation`), la magnitude est `max(0,01 ; v / min(P, 50 000 écus))`, où `v` est la valeur et `P` le patrimoine de la victime. Pour `GiftOut`, elle reste `v / P` de l'acteur. | C'est le « plancher en valeur absolue » de la revue, rendu précis. Voler un riche n'est pas anodin : son patrimoine compte comme la médiane. Toute prise compte au moins un peu. |
| **M4** | La seconde clause de **R1** (« tous les profils sauf la cruauté extrême ») s'applique à toute victime qui **n'est pas un ennemi déclaré**, et non aux seuls inconnus. | Sans cela, s'acharner sur une victime vulnérable coûtait moins que tirer sur un inconnu pour certains profils violents. Le contrôle des échelles l'a montré sur 729 profils de la grille. |
| **M5** | **R4** concerne le personnage prudent : jugement ≥ 0,4 (`JP` ≤ −0,4), **ou** stabilité ≤ −0,2 **avec** violence ≤ 0. | L'axe de stabilité réunit l'anxieux et l'impulsif. Seul l'anxieux est prudent, et une faible stabilité sans goût de la force en est le meilleur signe. Sinon, un bagarreur impulsif n'aurait jamais pu prendre de risque. |
| **M6** | **J1** et **J2** atténuent aussi la `Deception` envers l'agresseur (facteur 0,9). **J4** couvre aussi le besoin critique d'un **proche dépendant** (enfant, parent à charge). **J7** atténue aussi la `Deception` et le `BreachOfTrust` qui réduisent un danger grave pour un tiers (0,8). **J6** atténue aussi l'humiliation d'une fouille ou d'une palpation faite en service (0,8). | Mentir à un agresseur pour cacher sa victime, voler un médicament pour son enfant et dénoncer un proche qui prépare un meurtre sont des cas généraux. Ils relèvent d'une règle, pas d'une dérogation par acte. |
| **M7** | Le facteur de **J5** vaut `f = 0,2 + 0,4 × max(0, respect des règles)`, et au plus 0,3 pour `HarmPhysical`. | C'est la formule de « 0,2 à 0,6 selon le respect des règles ». |
| **M8** | Réticence : **0** aucune (coût < 0,01), **1** légère (< 0,05), **2** nette (< 0,20), **3** forte (≤ 1). La **rupture** (coût > S restante, ou > 1) est un indicateur à part. Le « niveau 0 à 4 » du §3.1 de la revue compte la rupture comme cinquième état. | Ces seuils placent l'exemple de Lucie (≈ 0,16) en « net », comme le dit la revue, et la gifle d'un pacifiste (≈ 0,25) en « fort ». |
| **M9** | Une **facilitation non reconnue** (le gardien ne sait pas qui il fait entrer) ne porte **aucun** effet de trahison avant l'acte. Elle porte seulement un `CommitmentBroken` si une consigne existait. | La trahison suppose de savoir. Le reste passe par E4 dans le bilan (revue, §3.8). |
| **M10** | Un transfert **fondé en droit** n'est jamais une `Appropriation` : saisie, amende, impôt, loyer ou dette dus, décision de justice, expulsion exécutée. De même, une obligation **suspendue en droit** (grève déclarée, congé, arrêt maladie) n'est pas manquée. | Sans cette règle, le policier qui saisit, l'huissier qui exécute ou le gréviste se désynchroniseraient pour un acte légal. |
| **M11** | Une échelle déclarée exige que chaque marche **domine** la précédente sur chaque couple (axe, sens), après dérivation. La monotonie du coût s'ensuit pour tout profil. « Insulter » sort de l'escalade du combat et rejoint l'échelle de la parole. | Insulter et menacer tirent des axes différents : aucune règle honnête ne rend l'une toujours pire que l'autre. La dominance, elle, est ce qu'un dériveur peut casser, donc ce qu'il faut contrôler. |
| **M12** | `SelfRisk` tire la stabilité vers le calme avec un coefficient faible (0,3). | Même raison que M5 : sur l'axe de stabilité, le risque ne distingue pas l'anxieux de l'impulsif. |
| **M13** | Aucun effet ne tire vers la franchise, la légalité ou le pacifisme. Les couples `HON+`, `REGL+` et `VIOL−` restent donc **sans objet**, avec des paramètres neutres, et ne sont pas calibrés. | Être honnête, légal ou paisible n'est jamais dissonant en soi. Le hors-la-loi qui dénonce sa bande paie la trahison (`BreachOfTrust`), pas la légalité. |
| **M14** | `CommitmentBroken` projette 0,5 sur le respect des règles, au lieu de 0,3. | La part de tempérament (`JP`) est plafonnée : sans ce relèvement, manquer une obligation ne coûtait presque rien, même à un légaliste rigide. |
| **M15** | Annotations et dérogations sont écrites **dans le fichier RON de l'acte**, dans un champ `conduct` : `add` (effets ajoutés), `opens` (justification d'usage ouverte) et `override` (dérogation). | L'auteur de l'acte les voit en l'écrivant, et le rapport de la CI les compte acte par acte. |
| **M16** | Une **dérogation** est une correction additive de γ sur un seul couple (axe, sens), de ±0,15 au plus, appliquée après l'agrégation et avant le coût. Elle ne s'applique jamais à un acte qui déclenche un plancher. | C'est la forme la plus étroite qui permette de corriger un placement. Elle reste vérifiable par les invariants. |
| **M17** | Une **ancre** est une **bande de niveaux** de réticence (0, 1, 2, 3, R) par archétype, et non un coût. | Le concepteur juge des degrés (« jamais », « à contrecœur »), pas des nombres. Le calibrage doit seulement rester dans ces degrés. |
| **M18** | La **classe de cible** retenue est celle dont le facteur est le plus élevé parmi les classes qui s'appliquent. La classe « ennemi » ne s'applique que si aucune classe intime, proche ou vulnérable ne s'applique. | Un ennemi blessé à terre est vulnérable avant d'être un ennemi, et un conjoint devenu hostile reste un intime. |

---

## 2. La grammaire des effets : valeurs de départ

### 2.1 Courbes de gravité

La gravité brute d'un effet est `g = G(m) × R(classe)`, bornée à 1 (revue, §3.3). Deux jeux de nœuds sont utilisés :
- **effets de valeur** : nœuds aux magnitudes 0,02, 0,10 et 0,50, c'est-à-dire en fraction de patrimoine ;
- **autres effets** : nœuds à 0,10, 0,40 et 1,00.

| N° | Effet | Nœuds | G au 1er nœud | G au 2e | G au 3e |
|---|---|---|---|---|---|
| 1 | `HarmPhysical` | 0,1 / 0,4 / 1 | 0,08 | 0,40 | 1,00 |
| 2 | `Force` | 0,1 / 0,4 / 1 | 0,04 | 0,20 | 0,50 |
| 3 | `Coercion` | 0,1 / 0,4 / 1 | 0,08 | 0,40 | 0,90 |
| 4 | `Endangerment` | 0,1 / 0,4 / 1 | 0,08 | 0,40 | 1,00 |
| 5 | `Humiliation` | 0,1 / 0,4 / 1 | 0,06 | 0,30 | 0,70 |
| 6 | `PropertyDamage` | 0,02 / 0,1 / 0,5 | 0,10 | 0,35 | 0,80 |
| 7 | `Appropriation` | 0,02 / 0,1 / 0,5 | 0,15 | 0,45 | 0,95 |
| 8 | `Deception` | 0,1 / 0,4 / 1 | 0,08 | 0,35 | 0,90 |
| 9 | `BreachOfTrust` | 0,1 / 0,4 / 1 | 0,10 | 0,40 | 0,90 |
| 10 | `CommitmentBroken` | 0,1 / 0,4 / 1 | 0,05 | 0,25 | 0,60 |
| 11 | `LawViolation` | 0,1 / 0,4 / 1 | 0,10 | 0,40 | 1,00 |
| 12 | `NormViolation` | 0,1 / 0,4 / 1 | 0,05 | 0,25 | 0,60 |
| 13 | `GiftOut` | 0,02 / 0,1 / 0,5 | 0,08 | 0,40 | 0,95 |
| 14 | `Exploitation` | 0,02 / 0,1 / 0,5 | 0,15 | 0,45 | 0,95 |
| 15 | `Facilitation` | — | Gravité du tort rendu possible × **0,7** | | |
| 16 | `Instigation` | — | Gravité du tort commandé × **0,8** | | |
| 17 | `Help` | 0,1 / 0,4 / 1 | 0,05 | 0,25 | 0,60 |
| 18 | `SelfRisk` | 0,1 / 0,4 / 1 | 0,08 | 0,40 | 1,00 |
| 19 | `Outburst` | 0,1 / 0,4 / 1 | 0,05 | 0,25 | 0,50 |
| 20 | `EmotionalImpact` | 0,1 / 0,4 / 1 | 0,05 | 0,25 | 0,60 |
| 21 | `TemperamentLean` | 0,1 / 0,4 / 1 | 0,10 | 0,40 | 1,00 |
| 22 | `SelfIndulgence` | 0,1 / 0,4 / 1 | 0,05 | 0,25 | 0,60 |

Lecture : un effet dont le plafond reste sous 1 ne peut pas, à lui seul, atteindre la gravité d'un tort grave. C'est le cas de `Force`, `Humiliation`, `CommitmentBroken`, `NormViolation`, `Help`, `Outburst`, `EmotionalImpact` et `SelfIndulgence`. Pousser, humilier ou manquer un rendez-vous ne vaut jamais un coup de couteau.

### 2.2 Barèmes de magnitude (ce que remplissent les descripteurs)

| Effet | Barème |
|---|---|
| `HarmPhysical` | Blessure attendue (modèle de blessure, combat §2.1) : douleur 0,05 ; blessure légère 0,2 ; incapacité 0,45 ; grave 0,7 ; mortelle 1. Avec une arme : espérance sur la distance et la zone visée. |
| `Force` | Pousser 0,2 ; saisir 0,3 ; immobiliser, maintenir au sol, menotter 0,4. |
| `Coercion` | Injonction 0,1 ; menace verbale 0,15 ; geste menaçant 0,25 ; arme montrée 0,4 ; arme braquée 0,7. Exiger un bien ou un acte ajoute 0,1. |
| `Endangerment` | Probabilité × gravité du dommage possible, selon les croyances de l'acteur. Repères : conduire ivre en ville 0,3 ; tir près de passants 0,6 ; feu dans un local habité 0,8. |
| `Humiliation` | Se moquer 0,1 ; insulter 0,3 ; cracher sur quelqu'un, humilier 0,5 ; humilier gravement 0,7. Multipliée par **1,5** en public, au plus 1. |
| `PropertyDamage`, `Appropriation`, `Exploitation` | M3 : `max(0,01 ; v / min(P, 50 000))`. Pour `Exploitation`, `v` est l'écart au prix de référence. |
| `Deception` | Enjeu, pour la cible, de ce qu'on lui fait croire : rien 0,02 ; un détail 0,05 ; une opinion qu'elle se fera 0,1 ; une décision qui l'engage 0,3 ; son argent, selon M3 et au moins 0,3 ; sa sécurité 0,8. |
| `BreachOfTrust` | Confidence anodine révélée 0,1 ; secret intime 0,4 ; secret dangereux pour son détenteur 0,7. Agir contre son employeur ou son groupe : 0,3 (négligence voulue) à 0,7 (le livrer). Infidélité 0,6. |
| `CommitmentBroken` | Rendez-vous manqué 0,1 ; retard au travail 0,2 ; journée de travail manquée, promesse rompue 0,3 ; abandon de poste 0,5 ; devoir de garde d'un enfant abandonné 0,8. |
| `LawViolation` | M2, d'après la peine type du barème (combat §8.5) : contravention 0,10 ; délit puni d'une amende ou d'un rappel à la loi 0,25 ; délit puni d'un sursis de 6 mois au plus 0,35 ; sursis plus long ou ferme de 6 mois au plus 0,45 ; ferme de 6 mois à 2 ans 0,55 ; ferme de plus de 2 ans 0,70 ; crime 0,85 ; meurtre 0,95 ; assassinat 1. S'y ajoutent les aggravations propres à l'auteur, +0,05 chacune, au plus +0,15. |
| `NormViolation` | Poids de la norme dans la pratique ou le groupe (ia-pnj §13.9, social §6.1), de 0,1 (incivilité) à 0,6 (manquement grave, comme ne pas venir aux obsèques d'un proche). |
| `GiftOut` | Valeur nette sortie divisée par le patrimoine de l'acteur, cumulée par contrepartie sur 30 jours (G9). Un prêt sans garantie compte pour le montant × la probabilité de défaut que croit l'acteur (0,3 par défaut). Une promesse d'argent compte au moment où elle est faite (economie §12.2). |
| `Help` | Consoler 0,1 ; rendre service 0,15 ; soigner 0,3 ; secourir 0,4 ; protéger d'un danger 0,6 ; risquer sa vie pour autrui 1. |
| `SelfRisk` | Probabilité × gravité attendue pour soi. La **probabilité de mort** et la **connaissance d'une issue sans risque** sont transmises à part, pour R4. |
| `Outburst` | Hausser le ton 0,1 ; crier 0,2 ; courir sans raison apparente, geste brusque 0,3 ; éclat prolongé 0,4. Toujours en public et sans raison que le personnage perçoive. |
| `EmotionalImpact` | Refus sec 0,1 ; critique, reproche 0,2 ; licenciement 0,3 ; expulsion d'un logement 0,5 ; rupture 0,6. |
| `TemperamentLean` | Penchant léger 0,2 ; net 0,5 ; fort 0,8, lu dans les étiquettes d'activité. |
| `SelfIndulgence` | Un verre 0,05 ; une dépense pour soi au-delà du budget, en mois de revenu disponible, au plus 1 ; ivresse 0,4 ; ivresse forte 0,6 ; produit illégal +0,1. |

### 2.3 Facteurs de cible

La table complète la revue (§3.3) pour les 22 effets. Les effets absents de la table n'ont pas de facteur de cible (facteur 1). Ce sont `LawViolation`, `NormViolation`, `SelfRisk`, `Outburst`, `TemperamentLean` et `SelfIndulgence`.

| Classe de cible | Corps : `HarmPhysical`, `Force`, `Coercion`, `Endangerment` | Émotion : `Humiliation`, `EmotionalImpact` | Confiance : `Deception`, `BreachOfTrust`, `CommitmentBroken` | Biens : `Appropriation`, `PropertyDamage`, `Exploitation` | `GiftOut` | `Help` |
|---|---|---|---|---|---|---|
| Intime | 1,4 | 1,4 | 1,3 | 1,3 | 0,3 | 0,5 |
| Proche | 1,2 | 1,2 | 1,2 | 1,2 | 0,6 | 0,7 |
| Connaissance, collègue | 1,0 | 1,0 | 1,0 | 1,0 | 0,9 | 1,0 |
| Inconnu | 1,0 | 0,8 | 0,8 | 1,0 | 1,0 | 1,0 |
| Ennemi déclaré, auteur d'un tort récent | 0,7 | 0,6 | 0,6 | 0,7 | 1,2 | 1,3 |
| Personne vulnérable | 1,3 | 1,3 | 1,2 | 1,3 | 0,8 | 0,8 |
| Organisation anonyme | 1,0 | 0,5 | 0,7 | 0,6 | 1,0 | 1,0 |
| Bénéficiaire anonyme | 1,0 | 1,0 | 1,0 | 1,0 | 1,0 | 1,0 |

`Facilitation` et `Instigation` prennent la classe de la victime du tort rendu possible ou commandé. La classe est choisie selon M18.

### 2.4 Projections : quels axes chaque effet tire, et de combien

| Effet | Axes tirés (sens) et coefficient π |
|---|---|
| `HarmPhysical` | `VIOL+` 1,0 ; `BIEN−` 0,7 |
| `Force` | `VIOL+` 0,8 ; `BIEN−` 0,2 |
| `Coercion` | `VIOL+` 0,7 ; `BIEN−` 0,5 |
| `Endangerment` | `BIEN−` 0,6 ; `STAB−` 0,4 |
| `Humiliation` | `BIEN−` 0,7 ; `STAB−` 0,3 |
| `PropertyDamage` | `VIOL+` 0,5 ; `BIEN−` 0,4 |
| `Appropriation` | `HON−` 0,8 ; `BIEN−` 0,4 |
| `Deception` | `HON−` 1,0 |
| `BreachOfTrust` | `HON−` 0,9 ; `BIEN−` 0,4 |
| `CommitmentBroken` | `JP+` 0,7 ; `REGL−` 0,5 (M14) |
| `LawViolation` | `REGL−` 1,0 |
| `NormViolation` | `REGL−` 0,6 ; `TF−` 0,3 |
| `GiftOut` | `BIEN+` 1,0 ; `JP+` 0,3 |
| `Exploitation` | `BIEN−` 0,8 ; `HON−` 0,6 |
| `Facilitation` | Projections du tort rendu possible ; plus `BreachOfTrust` envers le bien protégé s'il est celui de l'acteur, de son employeur ou d'un intime, et si la facilitation est reconnue (M9) |
| `Instigation` | Projections du tort commandé |
| `Help` | `BIEN+` 0,6 |
| `SelfRisk` | `STAB+` 0,3 (M12) ; `VIOL+` 0,3 ; `JP+` 0,4 |
| `Outburst` | `STAB−` 1,0 |
| `EmotionalImpact` | `TF−` 0,5 ; `BIEN−` 0,3 |
| `TemperamentLean` | L'axe et le sens de l'étiquette, 1,0 |
| `SelfIndulgence` | `STAB−` 0,6 ; `JP+` 0,5 |

### 2.5 Tolérance et poids

Les nœuds sont aux gravités agrégées γ = 0,05, 0,40 et 1,00.
- La **tolérance** `T` est la position `q` au-delà de laquelle l'acte devient dissonant. Elle décroît : plus l'acte est grave, plus de profils le refusent.
- Le **poids** `W` est le coût par unité de dépassement. Il croît.
- Coût sur un couple : `W(γ) × max(0, q − T(γ))`.

| Couple | Ce qu'il mesure | T (0,05 / 0,40 / 1,00) | W (0,05 / 0,40 / 1,00) |
|---|---|---|---|
| `VIOL+` | Usage de la force par qui y répugne | 0,40 / −0,35 / −0,75 | 0,03 / 0,45 / 2,00 |
| `BIEN−` | Faire du tort, quand on est bon | 0,30 / −0,25 / −0,70 | 0,06 / 0,40 / 1,60 |
| `BIEN+` | Donner ou aider, quand on est dur | 0,60 / −0,40 / −0,90 | 0,04 / 0,35 / 1,80 |
| `HON−` | Tromper, prendre, trahir, quand on est franc | 0,40 / 0,00 / −0,60 | 0,06 / 0,35 / 1,40 |
| `REGL−` | Enfreindre, quand on est légaliste | 0,35 / −0,35 / −0,75 | 0,04 / 0,25 / 1,20 |
| `STAB−` | S'emporter, se laisser aller, quand on est calme | 0,20 / −0,20 / −0,60 | 0,05 / 0,20 / 0,60 |
| `STAB+` | Prendre un risque, quand on est anxieux | 0,50 / 0,00 / −0,40 | 0,04 / 0,30 / 1,40 |
| Tempérament (les 8 couples, mêmes valeurs) | Agir contre son penchant | 0,50 / 0,25 / 0,00 | 0,02 / 0,06 / 0,12 |
| `HON+`, `REGL+`, `VIOL−` | Sans objet (M13) | 0,95 / 0,90 / 0,80 | 0,01 / 0,03 / 0,06 |

Lecture, sur trois exemples :
- un mensonge anodin (γ ≈ 0,04) ne touche que les personnages dont l'honnêteté dépasse 0,40, et à peine ;
- un mensonge à un intime sur une dette (γ ≈ 0,34) est dissonant dès une honnêteté d'environ 0,1 ;
- une escroquerie (γ proche de 1) l'est pour tous ceux dont l'honnêteté dépasse −0,60, avec un poids qui fait vite dépasser 1.

### 2.6 Autres paramètres

| Paramètre | Valeur | Rôle |
|---|---|---|
| Plafond du tempérament par acte | 0,03 | G12 |
| Plafond du tempérament par heure de jeu | 0,10 | G12 |
| Majoration publique de `Humiliation` | × 1,5 | §2.2 |
| Facteur de `Facilitation` | × 0,7 | E4 |
| Facteur d'`Instigation` | × 0,8 | Social §6.7 |
| Seuils de réticence | 0,01 / 0,05 / 0,20 | M8 |
| Épisode : écart maximal entre deux actes | 1 h de jeu | G9, **fixé** (règle, pas calibrage) |
| Épisode : fenêtre des valeurs (`GiftOut`, `Appropriation`, `Exploitation`) | 30 jours de jeu glissants | G9, **fixé**, aligné sur economie §12 |
| Épisode : nombre d'entrées | 8 | **fixé** |

Compte des paramètres calibrés de la grammaire :
- 60 nœuds de gravité ;
- 35 coefficients de projection ;
- 48 facteurs de cible ;
- 48 tolérances et poids ;
- 8 scalaires.

Soit **199** au total.

---

## 3. Justifications et planchers : valeurs de départ

### 3.1 Justifications

La formule est celle de la revue : `g' = (1 − f) × min(g, plafond) + max(0, g − plafond)`. Une seule justification s'applique par effet, la plus favorable. Une justification imputable (revue §3.7) ne compte pas.

| N° | Justification | Effets et facteur f | Plafond |
|---|---|---|---|
| J1 | Légitime défense | `HarmPhysical`, `Force`, `Coercion` 0,85 ; `Deception` envers l'agresseur 0,9 (M6) ; `LawViolation` 1 (effacée pour la part proportionnée) | Gravité de la menace (barème `HarmPhysical`) |
| J2 | Défense d'autrui | Intime ou enfant : mêmes effets à 0,9 ; autre tiers : 0,7 (`Deception` 0,9, `LawViolation` 1) | Gravité de la menace |
| J3 | Provocation | `Humiliation`, `Outburst`, `Coercion` 0,4 ; `HarmPhysical` 0,2 | — |
| J4 | Nécessité | `Appropriation`, `LawViolation` 0,5, si la cible n'est pas vulnérable ; besoin critique de l'acteur **ou d'un proche dépendant** (M6) | Magnitude 0,4, ce qui suffit au besoin |
| J5 | Ordre d'une autorité | `CommitmentBroken`, `LawViolation`, `NormViolation` : `0,2 + 0,4 × max(0, REGL)` ; `HarmPhysical` : le même, au plus 0,3 (M7) | — |
| J6 | Devoir de fonction | `LawViolation` 1 ; `Force`, `Coercion` 0,7 ; `HarmPhysical` 0,6 ; fouille et palpation (`Humiliation` du dériveur D31) 0,8 | Gravité de la résistance (0,5 par défaut pour une résistance active) |
| J7 | Urgence, sauvetage | `PropertyDamage`, `LawViolation` (jusqu'au délit à 0,45), `SelfRisk` 0,8 ; `Deception`, `BreachOfTrust` 0,8 (M6) | Gravité du danger évité |
| J8 | Consentement, jeu, usage | `Deception`, `Humiliation`, `Force` 0,9 ; `HarmPhysical` 0,9 jusqu'à 0,2 | Ce que couvre le consentement ou l'usage |
| J9 | Contrainte subie | Tous les effets sauf `HarmPhysical` grave envers un tiers : 0,6 | — |
| J10 | Riposte proportionnée à un tort | `PropertyDamage`, `Humiliation` 0,3 | Gravité du tort subi |
| J11 | Réparation | `GiftOut` envers la victime 0,9 | Montant du tort |
| J12 | Devoir envers un proche dépendant | `GiftOut`, `SelfRisk` 0,8 | Ce que demande le besoin |

### 3.2 Planchers de rupture

Ils sont évalués en dernier, sur le **profil de base**, avec les seules justifications non imputables. Ils imposent un coût ≥ 1 + ε, avec ε = 0,001.

| Règle | Effet et seuil | Profils « à contre-emploi » |
|---|---|---|
| R1 Violence grave | `HarmPhysical` de gravité ≥ 0,70 après justifications | Violence ≤ 0,3 **ou** bienveillance ≥ 0,5. **En plus**, s'il n'y a aucune justification et que la victime n'est pas un ennemi déclaré (M4) : tous les profils sauf bienveillance ≤ −0,8. |
| R2 Don massif | `GiftOut` cumulé ≥ 0,5 vers une contrepartie qui n'est pas un intime | Bienveillance < 0,9 |
| R3 Facilitation contre les siens | `Facilitation` reconnue contre soi, son employeur ou un intime | Honnêteté ≥ 0 **ou** respect des règles ≥ 0 |
| R4 Mise en danger mortelle de soi | `SelfRisk` avec probabilité de mort ≥ 0,2, alors qu'une issue sans risque est connue | `JP` ≤ −0,4, **ou** stabilité ≤ −0,2 avec violence ≤ 0 (M5) |

Compte des paramètres :
- 41 facteurs et plafonds de justification ;
- 12 seuils de plancher.

Soit **53** au total.

---

## 4. Les dériveurs

Les dériveurs traduisent les effets **mécaniques** que chaque domaine écrit déjà pour résoudre ses actes en effets **élémentaires** (revue, §4.2). Il y a deux sortes de travail :
- la partie **statique** (le gabarit et ses emplacements) tourne dans `ts-data build` ;
- la partie **à l'exécution** (remplir la magnitude et la classe d'après les croyances) est le descripteur `ConductDescriber` du greffon.

Tout est évalué sur les **croyances** de l'acteur. Il y a **32 règles** (décidé).

| N° | Greffon | Effet mécanique | Condition | Effet élémentaire | Magnitude (§2.2) | Contrepartie |
|---|---|---|---|---|---|---|
| D01 | `ts_combat` | `Damage { weapon, zone, force }` | Blessure attendue > 0 | `HarmPhysical` | Barème de blessure, en espérance | Victime |
| D02 | `ts_combat` | `Damage` ou `Grab`, `Push`, `Pin`, `Cuff` | Blessure attendue nulle | `Force` | 0,2 / 0,3 / 0,4 | Victime |
| D03 | `ts_combat`, `ts_social` | `Threaten { level }`, `Brandish`, `Aim`, ton menaçant | Une cible le perçoit | `Coercion` | Barème, + 0,1 si exigence | Cible |
| D04 | Tous | `RiskToOthers { p, severity }` | p × gravité > 0,02 | `Endangerment` | p × gravité | Personnes exposées (anonyme si plusieurs) |
| D05 | `ts_social` | `Mock`, `Insult`, `Humiliate`, `SpitOn` | — | `Humiliation` | Barème, × 1,5 en public | Cible |
| D06 | `ts_world` | `DamageState { object, from, to }` | Le bien n'est pas à l'acteur | `PropertyDamage` | M3, sur le coût de remise en état | Propriétaire |
| D07 | `ts_economy` | `Transfer { from, to, amount }` | `from` n'est pas l'acteur, ne consent pas, **sans fondement en droit** (M10) | `Appropriation` | M3 | Titulaire |
| D08 | `ts_economy` | Idem | Le titulaire est l'employeur, le groupe ou un intime de l'acteur | En plus : `BreachOfTrust` | 0,4 + magnitude de la prise, au plus 0,7 | Titulaire |
| D09 | `ts_economy` | `Transfer` depuis l'acteur | Valeur nette sortie > 0 au prix de référence | `GiftOut` | Net / patrimoine de l'acteur | Bénéficiaire, ou anonyme |
| D10 | `ts_economy` | `Loan { guarantee: false }` | — | `GiftOut` | Montant × défaut cru (0,3) / patrimoine | Emprunteur |
| D11 | `ts_economy` | `Promise { amount }`, reconnaissance de dette | — | `GiftOut` au moment de la promesse ; rien au versement | Montant / patrimoine | Bénéficiaire |
| D12 | `ts_economy` | Prix convenu | Écart défavorable à une cible vulnérable ou dépendante | `Exploitation` | Écart, M3 | Cible |
| D13 | `ts_economy` | Transfert fondé en droit (saisie, amende, impôt, dette due, décision de justice) | — | **Aucun** (M10) | — | — |
| D14 | `ts_economy` | `Purchase { for_self }` | Au-delà du budget disponible du mois | `SelfIndulgence` | Montant / revenu mensuel disponible | — |
| D15 | `ts_economy` | `Bet { stake }` | — | `GiftOut` (perte espérée) et `SelfIndulgence` (mise / revenu mensuel) | Barème | Anonyme |
| D16 | `ts_economy` | `Supply { product }` d'un produit nocif (stupéfiant, alcool à un mineur exclu par règle absolue) | — | `Endangerment` envers l'acheteur | Cannabis 0,15 ; tabac 0,05 ; cocaïne 0,4 | Acheteur |
| D17 | `ts_social` | `AssertBelief { believed_true: false }` | — | `Deception` | Enjeu pour l'auditeur | Auditeur |
| D18 | `ts_social` | `RevealFact { discretion_commitment }` | Engagement de discrétion envers un tiers | `BreachOfTrust` | Barème | Détenteur du secret |
| D19 | Tous | Acte marqué `against: Employer \| Group \| Partner` (détourner, livrer, être infidèle, abuser de sa fonction) | — | `BreachOfTrust` | 0,3 à 0,7, infidélité 0,6 | Employeur, groupe ou partenaire |
| D20 | `ts_ai` (agenda) | `MissObligation { importance }` | Obligation non suspendue en droit (M10) | `CommitmentBroken` | Barème | Bénéficiaire de l'obligation |
| D21 | `ts_justice` | Qualification par le catalogue (combat §8.1) | Infraction qualifiée **d'après les croyances** de l'acteur | `LawViolation` | M2 | Anonyme |
| D22 | `ts_ai`, `ts_social` | Norme d'une pratique ou d'un groupe | Acte déclaré déplacé par la pratique | `NormViolation` | Poids de la norme | Groupe |
| D23 | `ts_world`, `ts_economy` | `OpenAccess { protected_party }`, remise de clé ou de code, coupure d'alarme, annonce d'absence | Destinataire cru non autorisé | `Facilitation`, sur le gabarit du tort que l'acteur croit rendre possible ; drapeau « reconnue » | × 0,7 | Partie protégée |
| D24 | `ts_social` | `Order`, `Commission`, `Recruit { act }` | L'acte commandé porte un tort | `Instigation`, sur le gabarit de l'acte | × 0,8 | Victime de l'acte |
| D25 | `ts_ai`, `ts_body`, `ts_combat` | `Care`, `Rescue`, `Protect`, `Console`, `Serve` | — | `Help` | Barème | Bénéficiaire |
| D26 | Tous | `RiskToSelf { p_death, severity, safe_exit_known }` | — | `SelfRisk` (plus les drapeaux de R4) | p × gravité | — |
| D27 | `ts_ai` | Étiquettes `shout`, `run`, `abrupt_gesture` | En public, sans raison perçue | `Outburst` | Barème | — |
| D28 | `ts_social`, `ts_economy` | `RefuseHarshly`, `Criticize`, `Reproach`, `Fire`, `Evict`, `BreakUp` | Annoncer une mauvaise nouvelle n'en fait pas partie | `EmotionalImpact` | Barème | Cible |
| D29 | `ts_ai` | Étiquettes d'activité (seul ou en groupe, calme ou animé, connu ou nouveau, planifié ou imprévu, efficacité ou égards) | — | `TemperamentLean` | 0,2 / 0,5 / 0,8 | — |
| D30 | `ts_body` | `Consume { substance, dose }` | — | `SelfIndulgence` | Barème | — |
| D31 | `ts_combat` | `Search`, `PatDown { target }` (fouiller une personne, la palper) | — | `Humiliation` 0,1 et `Coercion` 0,1 | Fixe | Cible |
| D32 | Tous | Résolution de la **classe de cible** | — | — | M18 | — |

*Pourquoi ces 32 règles : elles couvrent tous les effets mécaniques des sept domaines. Un effet mécanique nouveau sans dériveur bloque la CI (revue §4.3, couverture), si bien que cette liste ne peut pas vieillir en silence.*

---

## 5. Annotations et dérogations des actes

### 5.1 Inventaire

J'ai recensé, dans les sept domaines, **235 actes** au grain où les domaines les décrivent. Le paquet en comptera davantage, ≈ 800 avec les variantes par rôle et par recette ; les proportions devraient tenir, car les variantes héritent des annotations de leur acte. Chaque acte a été classé ainsi :
- **dérivé** : ses effets mécaniques disent tout ;
- **annoté** : un sens manque dans ses effets ;
- **dérogé** : il est mal placé même après annotation.

| Domaine | Familles recensées | Actes | Dérivés | Annotés | Dérogés |
|---|---|---|---|---|---|
| Combat : intentions | Posture, attaque, défense, arme, issue, après (combat §1.7) | 35 | 34 | 1 | 0 |
| Combat : effractions et dégâts | Clé, crochetage, forçage, vitre, ruse, coffre, véhicule, dégradation, tag, incendie (§3) | 12 | 10 | 2 | 0 |
| Combat : témoins et victimes | Fuir, se figer, appeler, intervenir, secourir, filmer, encourager, se taire, plainte (§4) | 10 | 8 | 2 | 0 |
| Police et justice | Contrôle, palpation, fouille, interpellation, garde à vue, perquisition, saisie, interrogatoire, corruption, informateur, témoignage, aveu, jury (§5 à §8) | 20 | 18 | 2 | 0 |
| Crimes (recettes) | Vols, cambriolage, racket, usure, trafic, recel, fraudes, corruption, secrets, blanchiment, expéditions (§10) | 25 | 25 | 0 | 0 |
| Social | Les 45 actes du catalogue, la négociation et la délibération (social §3) | 57 | 49 | 6 | 2 |
| Économie | Tâches, contrats, prix, achats, ventes, location, expulsion, prêts, dons, jeu, prestations (economie §3 à §7) | 30 | 25 | 4 | 1 |
| Vie courante (IA) | Besoins, loisirs, enfants, vote, consommation, conduite, transports, projets (ia-pnj §1 à §9) | 30 | 30 | 0 | 0 |
| Actes excentriques permis par la surface de jeu | Actes qu'une action de contexte ou le déplacement permet (interface §5.2) ; aucun geste libre (A7) | 12 | 4 | 8 | 0 |
| Ville | Accès, clés, alarmes, portes (ville §5.1 et §5.5) | 4 | 4 | 0 | 0 |
| **Total** | | **235** | **207 (88,1 %)** | **25 (10,6 %)** | **3 (1,3 %)** |

Les trois catégories sont exclusives : un acte dérogé n'est pas compté parmi les annotés, et « dérivé » veut dire « ni annoté ni dérogé ».

### 5.2 Annotations (25)

Chaque ligne s'écrit dans le champ `conduct` de l'acte (M15). Exemple :

```ron
conduct: (add: [(kind: NormViolation, magnitude: 0.4, target: Owner)])
```

Il n'y a pas de magnitude libre : chaque valeur vient des barèmes du §2.2.

| N° | Domaine | Acte | Annotation | Sens qui manque aux effets mécaniques |
|---|---|---|---|---|
| A01 | Combat | Recouvrir le tag d'une bande rivale | `Humiliation` 0,3 envers le groupe rival (classe ennemi) | Un affront symbolique, sans dégât notable |
| A02 | Combat | Profaner une tombe, dégrader un monument aux morts | `NormViolation` 0,6 | Le sacré n'a pas de prix dans le grand livre |
| A03 | Combat (après) | Fouiller les poches d'un blessé ou d'un mort, hors fonction | `NormViolation` 0,5 | Le respect dû au corps d'autrui |
| A04 | Témoins | Encourager un agresseur | `Instigation` à 0,4 de la gravité du tort en cours | Encourager n'est pas ordonner, mais y contribue |
| A05 | Témoins | Se taire et partir quand la victime est en danger grave et qu'on pouvait appeler sans risque | `LawViolation` (non-assistance à personne en danger, délit 0,45) et `NormViolation` 0,2 | Une omission ne laisse aucun effet mécanique |
| A06 | Police et justice | Se taire devant la police quand on sait | `NormViolation` 0,2 (norme civique) | Le silence n'a pas d'effet mécanique |
| A07 | Police et justice | Informer la police sur son propre groupe | `BreachOfTrust` 0,6 envers le groupe | La loyauté au groupe n'est pas un engagement de discrétion écrit |
| A08 | Social | Mensonge de politesse ou de compassion (« ton plat est délicieux ») | `opens: [J8]` (usage de politesse) | L'usage l'attend ; la tromperie est réelle mais attendue |
| A09 | Social | Mentir à un enfant dans un usage (fêtes, contes) | `opens: [J8]` (usage) | Idem |
| A10 | Social | Séduire une personne qu'on sait en couple | `NormViolation` 0,3 | Briser un couple, au-delà de l'infidélité éventuelle de l'acteur, déjà dérivée par D19 |
| A11 | Social | Refuser de serrer la main tendue | `Humiliation` 0,2 | Un affront purement symbolique |
| A12 | Social | Brûler ou souiller l'emblème d'un groupe | `Humiliation` 0,4 envers le groupe | Symbole, valeur matérielle nulle |
| A13 | Social | Cracher sur quelqu'un | `Humiliation` 0,5 | Aucun dégât ni blessure |
| A14 | Économie | Expulser son locataire (procédure légale) | `EmotionalImpact` 0,5 | La procédure est légale (M10) et le grand livre ne montre pas la détresse |
| A15 | Économie | Voler ou détruire un objet de valeur affective (alliance, souvenir de famille) | `EmotionalImpact` 0,4 envers le propriétaire | La valeur affective n'est pas dans le grand livre. L'annotation porte sur le **type d'objet** et vaut pour tout acte qui le prend ou le détruit |
| A16 | Économie | Mendier | `NormViolation` 0,15, modulée par le milieu (social §5.5) | La honte sociale |
| A17 | Économie | Accepter un travail non déclaré (salarié) | `NormViolation` 0,2 | Le salarié ne commet aucune infraction du catalogue, mais un légaliste s'en écarte |
| ~~A18~~ | — | ~~Danser seul dans la rue~~ | **Sans objet en première version** (A7) : l'acte n'existe pas | — |
| ~~A19~~ | — | ~~S'allonger sur la chaussée~~ | **Sans objet en première version** (A7) : l'acte n'existe pas | — |
| A20 | Actes excentriques | Dévisager longuement un inconnu | `NormViolation` 0,15 | Geste excentrique, légèrement menaçant |
| A21 | Actes excentriques | Suivre un inconnu dans la rue | `NormViolation` 0,2 ; `Coercion` 0,1 s'il s'en aperçoit | Inquiéter sans menacer |
| A22 | Actes excentriques | S'asseoir à la table d'inconnus sans y être invité | `NormViolation` 0,2 | Geste excentrique |
| A23 | Actes excentriques | Entrer chez autrui par une porte ouverte, sans y être invité | `NormViolation` 0,4, et `LawViolation` (violation de domicile) si le combat l'ajoute au catalogue (§11.3) | L'intrusion n'a pas d'effet mécanique sans vol ni dégât |
| A24 | Actes excentriques | Fouiller une poubelle | `NormViolation` 0,1, modulée par le milieu | Geste excentrique |
| ~~A25~~ | — | ~~Uriner en public~~ | **Sans objet en première version** (A7) : l'acte n'existe pas | — |
| A26 | Actes excentriques | Jeter un déchet par terre | `NormViolation` 0,1 | Incivilité |
| A27 | Actes excentriques | Frapper aux portes au hasard | `NormViolation` 0,15 | Geste excentrique |
| A28 | Actes excentriques | Manger dans l'assiette d'un inconnu | `NormViolation` 0,3 (la prise est dérivée par D07) | Geste excentrique, au-delà de la valeur prise |

Les numéros A18, A19 et A25 sont gardés barrés pour que les renvois restent stables ; ils ne comptent pas parmi les 25 annotations.

*Pourquoi près d'un tiers des annotations portent sur les actes excentriques : ce sont des actes que la surface de jeu permet (une action de contexte, un déplacement) sans qu'aucun domaine les ait écrits pour être résolus. Leur sens est entièrement social. C'est aussi là que le détecteur de lacunes (revue §4.7) trouvera les suivantes.*

### 5.3 Dérogations (3)

| N° | Acte | Dérogation (M16) | Justification obligatoire |
|---|---|---|---|
| O1 | Couvrir un proche devant la police, par un mensonge simple, hors serment (variante de « mentir à la police » quand le bénéficiaire est intime) | γ `HON−` : −0,10 | La loyauté familiale est jugée comme une circonstance atténuante d'un mensonge aux autorités, au-delà de la classe de cible, qui pèse dans l'autre sens. Le faux témoignage sous serment n'est pas concerné. |
| O2 | Garder un objet trouvé | γ `REGL−` : −0,10 | Trouver n'est pas prendre. Les gens jugent garder un portefeuille trouvé moins grave que voler à l'étalage, alors que la qualification et la prise sont les mêmes. |
| O3 | Mentir à un mourant ou à un malade grave pour l'apaiser | γ `HON−` : −0,15 | Le mensonge de compassion envers un intime vulnérable est jugé bon par presque tous, alors que la classe de cible le rend très grave. |

Avec O2, l'ancre 19 reste dans ses bandes et passe sous l'ancre 20 (voler à l'étalage) pour tous les archétypes, comme le veut sa justification. O1 et O3 ne touchent aucune ancre.

Ces trois dérogations sont **prévisionnelles**. Le rapport de la phase 1 (`ts-data signatures`) dira si elles restent nécessaires une fois le paquet réel compilé, et en fera sans doute apparaître d'autres. L'alerte de la CI à 5 % s'applique dès ce moment-là.

---

## 6. Les archétypes et les contextes de référence

Ces douze profils servent à trois contrôles :
- le rapport `ts-data signatures` (coût de chaque acte pour chacun) ;
- l'ensemble de référence (revue §4.3, contrôle 9) ;
- les ancres (§7).

Ils couvrent les catégories courantes et les extrêmes. Le concepteur peut en changer le **portrait**, qui est ce qu'on montre aux annotateurs (revue §4.5).

| N° | Nom | Portrait | `EI` | `SN` | `TF` | `JP` | `STAB` | `BIEN` | `REGL` | `HON` | `VIOL` |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A1 | Bon samaritain | D'une bonté rare, franc, respectueux des règles, incapable de violence | 0 | 0,2 | 0,6 | 0 | 0,5 | 0,9 | 0,6 | 0,8 | −0,8 |
| A2 | Honnête ordinaire | Gentil sans plus, honnête et respectueux de la loi, évite les conflits | 0 | 0 | 0,2 | −0,2 | 0,2 | 0,3 | 0,5 | 0,5 | −0,3 |
| A3 | Doux anxieux | Inquiet, réservé, bienveillant, a horreur de la violence | −0,6 | 0 | 0 | 0 | −0,6 | 0,5 | 0,4 | 0,4 | −0,8 |
| A4 | Légaliste rigide | Organisé, calme, très attaché à la loi, franc, prêt à user de la force si la loi le permet | 0 | 0 | −0,3 | −0,7 | 0,5 | 0 | 0,9 | 0,6 | 0,2 |
| A5 | Débrouillard | Arrange les règles à sa façon, pas méchant, improvise | 0 | 0 | 0 | 0,5 | 0,1 | 0,2 | −0,3 | −0,2 | −0,1 |
| A6 | Petit voyou | Sociable, impulsif, peu scrupuleux, aime la bagarre | 0,4 | 0 | 0 | 0,6 | −0,4 | −0,3 | −0,7 | −0,4 | 0,5 |
| A7 | Caïd froid | Calme, calculateur, dur, hors-la-loi, violent quand il le faut | 0 | 0 | −0,6 | −0,4 | 0,6 | −0,7 | −0,9 | −0,6 | 0,7 |
| A8 | Beau parleur | Charmeur, sociable, menteur, évite la violence | 0,7 | 0 | 0 | 0 | 0,4 | −0,2 | 0 | −0,8 | −0,4 |
| A9 | Bagarreur impulsif | S'emporte vite, cogne facilement, ni menteur ni franc | 0 | 0 | 0 | 0 | −0,8 | −0,4 | −0,3 | 0 | 0,8 |
| A10 | Généreux bohème | Généreux, sensible, désordonné, insouciant | 0 | 0 | 0,7 | 0,7 | −0,2 | 0,7 | 0 | 0,3 | −0,4 |
| A11 | Avare calculateur | Regardant, froid, organisé, assez respectueux des règles | 0 | 0 | −0,7 | −0,6 | 0,3 | −0,4 | 0,3 | 0,1 | −0,2 |
| A12 | Neutre | Au centre de tous les axes ; c'est aussi le profil de départ d'un nouveau joueur (04 §2) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Les **trois contextes** de référence sont :

| N° | Contexte | Circonstances |
|---|---|---|
| C0 | Ordinaire | Aucune |
| C1 | Menacé | Agression actuelle contre soi, gravité de la menace 0,45 (incapacité) ; ouvre J1 |
| C2 | Dans le besoin | Besoin critique sans moyen fidèle connu ; ouvre J4, plafond 0,4 |

---

## 7. Les ancres

Une ancre est un acte type dont le concepteur fixe la **place** pour chaque archétype, en **bandes de niveaux** de réticence (M17) :
- **0** aucune ;
- **1** légère ;
- **2** nette ;
- **3** forte ;
- **R** impossible, même à synchronisation parfaite.

« 0–1 » signifie « aucune ou légère ». Le calibrage ne peut jamais sortir une ancre de sa bande (revue §4.5).

Je les ai écrites pour le concepteur, à sa place, puisque la conception ne lui pose pas de question. Chacune se lit comme une phrase (« Ce que dit l'ancre »), et il peut la changer d'un mot. Avec les valeurs de départ des §2 et §3, **les 46 ancres sont respectées pour les 12 archétypes**.

| N° | Acte | Effets (magnitude, cible, justification) | Ce que dit l'ancre | A1 | A2 | A3 | A4 | A5 | A6 | A7 | A8 | A9 | A10 | A11 | A12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 01 | Dire un mensonge de politesse à un collègue (« j'adore ton pull ») | `Deception` 0,05, connaissance | Anodin pour presque tous | 0–1 | 0 | 0 | 0–1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 02 | Mentir à sa sœur sur une dette | `Deception` 0,3, intime | Coûteux pour les francs, rien pour les menteurs | 3 | 2 | 2 | 2 | 0–1 | 0 | 0 | 0 | 0–1 | 1–2 | 0–1 | 0–1 |
| 03 | Escroquer une personne âgée de 30 % de son patrimoine | `Deception` 0,8, `Appropriation` 0,3, `Exploitation` 0,3, vulnérable ; délit 0,45 | Jamais pour qui n'est ni malhonnête ni dur | R | R | R | R | 3–R | 2–3 | 0–1 | 2–3 | 3–R | R | 3–R | 3–R |
| 04 | Révéler la confidence d'un ami pour s'en moquer | `BreachOfTrust` 0,4, `Humiliation` 0,2, proche | Fort pour les loyaux et les bons | 3 | 2–3 | 2–3 | 2–3 | 1–2 | 0–1 | 0–1 | 0–1 | 0–1 | 2–3 | 1–2 | 1–2 |
| 05 | Bluffer au poker entre amis | `Deception` 0,2, proche ; J8 | Anodin pour tous | 0–1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 06 | Faire un faux témoignage pour couvrir un ami | `Deception` 0,5 envers l'institution ; délit 0,35 | Fort pour les légalistes et les francs | 3 | 3 | 3 | 3–R | 0–1 | 0–1 | 0 | 1–2 | 0–1 | 2 | 2–3 | 2 |
| 07 | Insulter un inconnu qui vous bouscule | `Humiliation` 0,2 en public, `Outburst` 0,3 | Net pour le doux, rien pour le bagarreur | 2 | 1–2 | 1 | 1–2 | 0–1 | 0 | 1–2 | 1–2 | 0 | 1–2 | 0–1 | 0–1 |
| 08 | Gifler un inconnu | `HarmPhysical` 0,05, `Humiliation` 0,3 en public ; délit 0,35 | ≈ 0,3 pour un pacifiste (repère du §3.5 de la revue) | 3 | 2–3 | 3 | 3 | 1–2 | 0–1 | 0–1 | 2 | 0 | 2–3 | 2 | 1–2 |
| 09 | Frapper son agresseur pendant l'agression | `HarmPhysical` 0,2 ; délit 0,35 ; J1 (menace 0,45) | Presque rien pour tous | 1–2 | 0–1 | 1–2 | 0–1 | 0–1 | 0 | 0 | 0–1 | 0 | 0–1 | 0–1 | 0 |
| 10 | Frapper l'agresseur à terre qui renonce | `HarmPhysical` 0,2, vulnérable ; délit 0,35 | S'acharner : fort pour les doux | 3 | 3 | 3 | 3 | 1–2 | 0–1 | 0–1 | 2 | 0–1 | 3 | 2–3 | 2 |
| 11 | Tuer un inconnu d'un coup de couteau, sans raison | `HarmPhysical` 1, inconnu ; meurtre 0,95 | Impossible pour tous (R1) | R | R | R | R | R | R | R | R | R | R | R | R |
| 12 | Blesser gravement un rival de bande au couteau | `HarmPhysical` 0,7, ennemi ; délit 0,5 | Naturel pour les violents, impossible pour les doux | R | 3–R | R | 3–R | 3 | 0–1 | 0–1 | 3 | 0–1 | 3–R | 3 | 3 |
| 13 | Pousser quelqu'un qui bloque le passage | `Force` 0,2 | Léger au plus | 1 | 0–1 | 1–2 | 0 | 0 | 0 | 0 | 0–1 | 0 | 0–1 | 0 | 0 |
| 14 | Menacer un commerçant pour le racket | `Coercion` 0,4 ; délit 0,55 | Fort pour les honnêtes, rien pour le caïd | 3–R | 3 | 3 | 3 | 2 | 0–1 | 0 | 2–3 | 0–2 | 3 | 2–3 | 2–3 |
| 15 | Braquer une arme sur un caissier | `Coercion` 0,9 ; crime 0,85 | Impossible pour les honnêtes et les doux | R | R | R | R | 3–R | 0–2 | 0–1 | 3–R | 2–3 | R | 3–R | 3–R |
| 16 | Tirer sur un policier qui vous poursuit | `HarmPhysical` 0,9 ; crime 0,95 | Impossible pour tous (R1) | R | R | R | R | R | R | R | R | R | R | R | R |
| 17 | Policier : maîtriser un suspect qui résiste | `Force` 0,4, `Coercion` 0,4 ; délit 0,35 ; J6 | Faisable pour tout policier | 1–2 | 0–1 | 1–2 | 0 | 0 | 0 | 0 | 0–1 | 0 | 0–2 | 0 | 0 |
| 18 | Policier : frapper un suspect menotté | `HarmPhysical` 0,2, vulnérable ; délit 0,4 | Fort pour un policier légaliste | 3 | 3 | 3 | 3 | 2 | 0–1 | 0–1 | 2–3 | 0–1 | 3 | 2–3 | 2 |
| 19 | Garder un portefeuille trouvé (50 écus) | `Appropriation` 0,01 ; délit 0,25 ; dérogation O2 | Léger ou net pour les honnêtes, toujours moins que l'ancre 20 | 2 | 1–2 | 1–2 | 1–2 | 0 | 0 | 0 | 0 | 0 | 0–1 | 0–2 | 0–1 |
| 20 | Voler à l'étalage dans une grande enseigne | `Appropriation` 0,01, organisation ; délit 0,25 | Net ou fort pour les honnêtes | 2–3 | 2 | 2 | 2–3 | 0–1 | 0 | 0 | 0–1 | 0–1 | 0–1 | 2 | 0–1 |
| 21 | Voler de la nourriture quand on a faim | Comme l'ancre 20 ; J4 | Possible pour un honnête affamé | 1–2 | 1–2 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0–1 | 0–1 | 0–1 |
| 22 | Cambrioler la maison d'un voisin absent | `Appropriation` 0,05, `PropertyDamage` 0,005 ; délit 0,45 | Fort pour les honnêtes | 3–R | 3 | 3 | 3–R | 1 | 0–1 | 0–1 | 2 | 0–1 | 3 | 3 | 2 |
| 23 | Voler un inconnu à la tire (100 écus) | `Appropriation` 0,01 ; délit 0,25 | Net ou fort pour les honnêtes | 2–3 | 2–3 | 2 | 2–3 | 0–1 | 0 | 0 | 0–1 | 0–1 | 1–2 | 2 | 0–1 |
| 24 | Arracher le sac d'une vieille dame, qui tombe | `Appropriation` 0,01, `HarmPhysical` 0,2, vulnérable ; délit 0,45 | Fort pour presque tous | 3–R | 3 | 3 | 3 | 2–3 | 0–1 | 0–1 | 3 | 0–1 | 3–R | 3 | 2–3 |
| 25 | Taguer un mur | `PropertyDamage` 0,001 ; délit 0,25 | Net pour les légalistes | 2 | 1–2 | 1–2 | 2 | 0–1 | 0 | 0 | 0–1 | 0 | 0–1 | 1–2 | 0–1 |
| 26 | Incendier le local vide d'un commerçant | `PropertyDamage` 0,6, `Endangerment` 0,3 ; délit 0,55 | Impossible pour les bons et les honnêtes | R | R | R | 3–R | 3 | 0–1 | 0–1 | 3 | 2 | R | 3–R | 3 |
| 27 | Receler un téléphone volé | Délit 0,35 | Net pour les honnêtes | 2–3 | 2–3 | 2–3 | 3 | 0–1 | 0 | 0 | 1–2 | 0–1 | 1–2 | 1–2 | 1–2 |
| 28 | Donner 2 écus à un mendiant | `GiftOut` 0,0001, anonyme | Anodin pour tous | 0 | 0 | 0 | 0 | 0 | 0 | 0–1 | 0 | 0 | 0 | 0–1 | 0 |
| 29 | Donner 10 % de son patrimoine à une association | `GiftOut` 0,1, organisation | Naturel pour les généreux, fort pour les durs | 0 | 1–2 | 0–1 | 2 | 2 | 2–3 | 3 | 2–3 | 2–3 | 0–1 | 3 | 2 |
| 30 | Laisser la moitié de son patrimoine sur un banc | `GiftOut` 0,5, anonyme | Impossible sauf pour un saint (R2) | 0–1 | R | R | R | R | R | R | R | R | R | R | R |
| 31 | Prêter 500 écus sans garantie à un ami | `GiftOut` 0,003, proche | Anodin pour presque tous | 0 | 0 | 0 | 0 | 0 | 0 | 0–1 | 0 | 0 | 0 | 0–1 | 0 |
| 32 | Payer son loyer | Aucun effet | Neutre pour tous | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 33 | Prêter à usure à un débiteur aux abois | `Exploitation` 0,15, vulnérable ; délit 0,55 | Impossible pour les bons | R | 3–R | R | 3–R | 2–3 | 1 | 0 | 2–3 | 0–2 | R | 2–3 | 2–3 |
| 34 | Policier : se laisser corrompre pour couvrir une enquête | `BreachOfTrust` 0,7, `Deception` 0,4, organisation ; délit 0,55 | Impossible pour un policier légaliste | R | 3–R | 3–R | R | 2–3 | 0–1 | 0–1 | 2–3 | 2 | 3 | 3 | 3 |
| 35 | Licencier un employé fautif | `EmotionalImpact` 0,3 | Léger au plus | 1 | 0–1 | 0–1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
| 36 | Rompre avec son conjoint par message | `EmotionalImpact` 0,6, `CommitmentBroken` 0,3, intime | Net pour les sensibles | 2–3 | 1–2 | 1–2 | 1–2 | 1–2 | 0–1 | 0–1 | 0–1 | 0–1 | 2 | 0–1 | 0–1 |
| 37 | Arriver une heure en retard sans prévenir | `CommitmentBroken` 0,3 | Léger pour les organisés | 1 | 0–1 | 0–1 | 1–2 | 0 | 0 | 0–1 | 0–1 | 0 | 0 | 0–1 | 0–1 |
| 38 | Ne pas aller aux obsèques d'un proche | `NormViolation` 0,6, `CommitmentBroken` 0,4 | Net pour la plupart | 2–3 | 1–2 | 1–2 | 2–3 | 0–1 | 0–1 | 0–1 | 0–1 | 0–1 | 1–2 | 1–2 | 0–1 |
| 39 | Crier et courir dans la rue sans raison | `Outburst` 0,4 | Net pour les calmes | 1–2 | 1 | 0–1 | 2 | 0–1 | 0 | 2 | 1–2 | 0 | 0–1 | 1 | 0–1 |
| 40 | Passer la soirée seul au lieu d'aller à la fête | `TemperamentLean` vers l'introversion 0,5 | Léger au plus (G12) | 0 | 0 | 0 | 0 | 0 | 0–1 | 0 | 1 | 0 | 0 | 0 | 0 |
| 41 | S'enivrer jusqu'à tituber en public | `SelfIndulgence` 0,6, `NormViolation` 0,2 ; contravention 0,1 | Net pour les rangés | 2 | 2 | 1 | 2–3 | 1 | 0 | 1–2 | 1 | 0 | 0–1 | 2 | 0–1 |
| 42 | Traverser un toit glissant pour fuir, alors qu'un escalier est connu | `SelfRisk` 0,5, mort possible 0,25 | Impossible pour les prudents (R4) | 2–3 | 1–2 | R | R | 0–1 | 0–2 | R | 1 | 1–3 | R | R | 0–1 |
| 43 | Gardien : ouvrir à un faux livreur (sans le savoir) | `CommitmentBroken` 0,2 (consigne) | Léger au plus : la tromperie n'est pas reconnue | 0–1 | 0–1 | 0–1 | 0–1 | 0 | 0 | 0 | 0 | 0 | 0–1 | 0 | 0 |
| 44 | Donner le code du coffre de son employeur à un complice | `Facilitation` reconnue, contre l'employeur | Impossible pour les honnêtes ou les légalistes (R3) | R | R | R | R | 2–3 | 0–1 | 0–1 | 3–R | R | R | R | R |
| 45 | Secourir un blessé dans la rue | `Help` 0,4, `SelfRisk` 0,05 | Rien, sauf un peu pour les durs | 0 | 0 | 0 | 0 | 0 | 0–1 | 1–2 | 0–1 | 0–1 | 0 | 1 | 0 |
| 46 | S'interposer pour protéger son enfant | `Help` 0,6, intime, `SelfRisk` 0,3, `Force` 0,3 ; J2, J12 | Presque rien pour tous | 0 | 0 | 0–2 | 0 | 0 | 0 | 0–1 | 0 | 0–1 | 0 | 0–1 | 0 |

Ce que les ancres fixent, en résumé :
- **Les actes extrêmes sont impossibles par construction** :
  - tuer sans raison (11, 16) ;
  - donner la moitié de ses biens (30) ;
  - livrer son employeur (44) ;
  - risquer sa vie sans nécessité (42), pour les prudents.

  C'est la garantie de E1.
- **Le contexte compte** : frapper son agresseur ne coûte presque rien (09), s'acharner sur lui à terre coûte cher (10). Voler pour manger reste possible pour un honnête homme (21), et le policier maîtrise sans peine celui qui résiste (17).
- **Les profils se distinguent nettement** :
  - le caïd ne paie presque rien pour le racket (14), le bon samaritain ne peut pas s'y résoudre ;
  - le généreux donne sans peine (29), l'avare non ;
  - le beau parleur ment sans peine (02), le franc hésite.
- **Le tempérament ne pèse presque rien** (40) : jamais plus qu'une réticence légère.

---

## 8. Les échelles déclarées

Chaque marche **domine** la précédente sur chaque couple (axe, sens) (M11). Le coût est donc monotone pour tout profil, ce que le modèle de vérification a confirmé sur les 19 683 profils de la grille 3⁹, sans exception. `ts-data check` contrôle la dominance après dérivation. Un dériveur qui la casse bloque la CI.

| Échelle | Marches et effets dérivés |
|---|---|
| **Escalade** (combat §11.2, sans « insulter », M11) | Menacer (`Coercion` 0,15 ; délit 0,35) < frapper (`HarmPhysical` 0,2 ; délit 0,35) < brandir une arme (`Coercion` 0,4 ; délit 0,4) < frapper avec une arme (`HarmPhysical` 0,45 ; délit 0,5) < viser (`Coercion` 0,7, `HarmPhysical` attendu 0,45 ; délit 0,5) < tirer (`HarmPhysical` 0,85, `Coercion` 0,7 ; crime 0,85) < s'acharner (`HarmPhysical` 1 sur une victime vulnérable, `Coercion` 0,7 ; meurtre 0,95) |
| **Parole** | Se moquer (`Humiliation` 0,1) < insulter (0,3) < humilier en public (0,5 × 1,5) |
| **Vol** | Voler à l'étalage (`Appropriation` 0,01, organisation ; délit 0,25) < voler à la tire (`Appropriation` 0,01, inconnu ; délit 0,25) < vol avec violence (`Appropriation` 0,01, `HarmPhysical` 0,2 ; délit 0,45) |
| **Mensonge** | Mensonge de politesse (`Deception` 0,05) < mensonge d'intérêt (0,3) < escroquerie (`Deception` 0,6, `Appropriation` 0,1 ; délit 0,4) |
| **Dons** | Pourboire (`GiftOut` 0,001) < cadeau (0,01) < don important (0,1) < don massif (0,5) |
| **Engagements** | Retard (`CommitmentBroken` 0,2) < rendez-vous manqué (0,3) < abandon de poste (0,5) < abandon d'un enfant à garder (`CommitmentBroken` 0,8 envers un intime, `Endangerment` 0,3 envers une personne vulnérable) |

Pour « viser », la blessure attendue est celle d'un coup de feu si l'arme part. C'est elle qui place « viser » au-dessus de « frapper avec une arme ».

---

## 9. Les gabarits d'explication

### 9.1 Forme (décidé)

Chaque raison codée du verdict (revue §5.1) a **trois formes**, parce que l'interface parle de deux façons (interface §2.7 et §4.1) :

| Forme | Sert à | Personne | Exemple |
|---|---|---|---|
| **Claire** | Réticence au survol, intuition immédiate à précision « claire » | 2e (voix intérieure) | « Ça ne vous ressemble pas de mentir. » |
| **Précise** | Les mêmes, à précision « précise » | 2e, avec la cible | « Vous ne mentez pas à vos proches. » |
| **Nominale** | Bilan (« Ce qui lui a coûté »), raison d'une rupture, vue de conception | Infinitif, 3e personne | « mentir à sa sœur » |

Variables, remplies par l'interface selon la classe de cible, la précision et la personne :
- `{cible}` : « votre sœur » ou « sa sœur » ; à précision claire, la classe seule (« un proche », « un inconnu », « quelqu'un de fragile », « une enseigne ») ;
- `{à_cible}` et `{de_cible}` : la même cible avec sa préposition ;
- `{Cible}` : la cible en début de phrase ;
- `{groupe}` : la pratique ou le groupe (« la famille », « vos collègues ») ;
- `{infraction}` : le nom de l'infraction du catalogue (« un vol ») ;
- `{besoin}` : le besoin critique (« vous avez faim ») ;
- `{e}` : l'accord au féminin.

Aucun gabarit ne contient de nombre (G24) ni d'affirmation sur la vie passée du personnage, qui pourrait être fausse.

*Pourquoi trois formes plutôt qu'une phrase à conjuguer : le français accorde les verbes et les adjectifs ; trois formes courtes écrites à la main sont plus sûres qu'un moteur de conjugaison, et restent traduisibles.*

### 9.2 Raisons tirées des effets (46)

| N° | Effet, axe | Claire | Précise | Nominale |
|---|---|---|---|---|
| E01 | `HarmPhysical`, violence | Vous n'êtes pas du genre à frapper. | Faire mal {à_cible} ? Ce n'est pas vous. | faire mal {à_cible} |
| E02 | `HarmPhysical`, bienveillance | Faire souffrir, ça ne vous ressemble pas. | Vous ne supporteriez pas de faire souffrir {cible}. | faire souffrir {cible} |
| E03 | `Force`, violence | Vous n'aimez pas bousculer les gens. | Porter la main sur {cible}, même sans frapper, vous répugne. | porter la main sur {cible} |
| E04 | `Force`, bienveillance | Vous n'aimez pas brusquer les autres. | Brusquer {cible} ? Pas vous. | brusquer {cible} |
| E05 | `Coercion`, violence | Menacer n'est pas votre manière. | Faire peur {à_cible} pour obtenir quelque chose ? Vous ne vous y voyez pas. | menacer {cible} |
| E06 | `Coercion`, bienveillance | Vous n'aimez pas faire peur. | Terroriser {cible} ? Jamais. | terroriser {cible} |
| E07 | `Endangerment`, bienveillance | Vous ne mettez pas les autres en danger. | Faire courir un risque {à_cible}, vous ne vous le pardonneriez pas. | mettre {cible} en danger |
| E08 | `Endangerment`, stabilité | C'est bien trop imprudent pour vous. | Jouer avec la sécurité des autres ? Vous êtes trop posé{e} pour ça. | jouer avec la sécurité des autres |
| E09 | `Humiliation`, bienveillance | Vous n'aimez pas blesser par les mots. | Humilier {cible}, et devant tout le monde ? Jamais. | humilier {cible} |
| E10 | `Humiliation`, stabilité | D'habitude, vous gardez votre calme. | Vous emporter contre {cible} comme ça ? Ça ne vous ressemble pas. | s'emporter contre {cible} |
| E11 | `PropertyDamage`, bienveillance | Abîmer ce qui appartient aux autres, ce n'est pas vous. | Casser ce qui appartient {à_cible} ? Vous n'y pensez même pas. | abîmer le bien {de_cible} |
| E12 | `PropertyDamage`, violence | Vous n'êtes pas du genre à tout casser. | Tout détruire, ce n'est pas votre façon de régler les choses. | tout casser |
| E13 | `Appropriation`, honnêteté | Vous ne prenez pas ce qui n'est pas à vous. | Voler {cible} ? Ce n'est pas vous. | voler {cible} |
| E14 | `Appropriation`, bienveillance | Priver quelqu'un de ce qu'il a vous met mal à l'aise. | Prendre {à_cible} ce dont on a besoin ? Non. | dépouiller {cible} |
| E15 | `Deception`, honnêteté | Ça ne vous ressemble pas de mentir. | Vous ne mentez pas {à_cible}. | mentir {à_cible} |
| E16 | `BreachOfTrust`, honnêteté | Vous tenez parole et vous gardez les secrets. | Trahir la confiance {de_cible} ? Pas vous. | trahir {cible} |
| E17 | `BreachOfTrust`, bienveillance | Vous ne laissez pas tomber ceux qui comptent sur vous. | {Cible} compte sur vous. | laisser tomber {cible} |
| E18 | `CommitmentBroken`, organisation | Vous tenez à vos engagements. | Vous l'aviez promis {à_cible}. | manquer à sa parole |
| E19 | `CommitmentBroken`, règles | Ce qui est prévu doit être fait. | Manquer à ce que vous devez {à_cible} vous dérange. | manquer à ses obligations |
| E20 | `LawViolation`, règles | Vous respectez la loi. | Pour vous, la loi, c'est la loi : {infraction}, c'est non. | enfreindre la loi ({infraction}) |
| E21 | `NormViolation`, règles | Ça ne se fait pas, et vous le savez. | Devant {groupe}, ce serait déplacé. | manquer aux usages |
| E22 | `NormViolation`, décision | Vous tenez à ce que pensent les autres. | Vous ne voudriez pas choquer {groupe}. | choquer son entourage |
| E23 | `GiftOut`, bienveillance | Cet argent compte pour vous. | Vous ne laisseriez pas vos économies au premier venu. Pour une personne : Même pour {cible}, c'est beaucoup. | se défaire de son argent |
| E24 | `GiftOut`, organisation | Vous faites attention à votre argent. | Dépenser autant d'un coup ? Vous tenez vos comptes. | dépenser sans compter |
| E25 | `Exploitation`, bienveillance | Profiter de la faiblesse des autres, ce n'est pas vous. | Profiter de la détresse {de_cible} ? Jamais. | profiter de la détresse {de_cible} |
| E26 | `Exploitation`, honnêteté | Vous êtes honnête en affaires. | Faire payer ce prix-là {à_cible}, vous trouveriez ça malhonnête. | abuser {de_cible} en affaires |
| E27 | `Facilitation` (tort rendu possible) | Vous ne laissez pas entrer n'importe qui. | Ouvrir à quelqu'un qui veut du mal {à_cible} ? Jamais. | ouvrir la porte à un mauvais coup |
| E28 | `Facilitation` (trahison des siens) | Vous ne trahissez pas ceux qui vous font confiance. | Livrer {cible} ? Pas vous. | livrer {cible} |
| E29 | `Instigation` | Faire faire le sale travail par d'autres, ce n'est pas vous. | Ordonner qu'on s'en prenne {à_cible} ? Vous en seriez incapable. | faire s'en prendre {à_cible} |
| E30 | `Help`, bienveillance (chez un dur) | Aider les autres, ce n'est pas trop votre genre. | Vous mettre en quatre pour {cible} ? Ça ne vous ressemble pas. | se mettre en quatre pour {cible} |
| E31 | `SelfRisk`, stabilité | C'est trop risqué pour vous. | Prendre un tel risque ? Vous êtes bien trop prudent{e}. | prendre un tel risque |
| E32 | `SelfRisk`, violence | Vous n'aimez pas l'affrontement. | Vous jeter dans la mêlée ? Pas vous. | se jeter dans la mêlée |
| E33 | `SelfRisk`, organisation | Vous réfléchissez avant d'agir. | Foncer sans réfléchir ? Vous préférez assurer vos arrières. | foncer sans réfléchir |
| E34 | `Outburst`, stabilité | Vous gardez votre sang-froid. | Crier comme ça en pleine rue ? Ça ne vous ressemble pas. | perdre son sang-froid en public |
| E35 | `EmotionalImpact`, décision | Vous ménagez les sentiments des autres. | Vous y mettriez plus de formes avec {cible}. | blesser {cible} sans ménagement |
| E36 | `EmotionalImpact`, bienveillance | Vous n'aimez pas faire de la peine. | Faire de la peine {à_cible} ? Vous vous en voudriez. | faire de la peine {à_cible} |
| E37 | Penchant vers l'introversion | Vous aimez être entouré{e}. | Rester seul{e} ce soir ? Vous vous ennuieriez. | s'isoler |
| E38 | Penchant vers l'extraversion | Vous préférez le calme. | Toute cette agitation vous fatigue. | se mêler à la foule |
| E39 | Penchant vers le connu | Vous aimez la nouveauté. | Encore la même chose ? Vous avez besoin de changement. | s'en tenir à l'habitude |
| E40 | Penchant vers la nouveauté | Vous préférez ce que vous connaissez. | Un endroit inconnu ? Vous n'y êtes pas à l'aise. | se lancer dans l'inconnu |
| E41 | Penchant vers l'efficacité | Vous pensez d'abord aux autres. | Faire passer l'efficacité avant {cible} ? Pas vous. | faire passer l'efficacité avant les gens |
| E42 | Penchant vers les égards | Vous allez droit au but. | Tant de précautions, ça ne vous ressemble pas. | prendre des gants |
| E43 | Penchant vers le plan | Vous aimez improviser. | Tout prévoir à l'avance ? Ça vous ennuie. | tout planifier |
| E44 | Penchant vers l'imprévu | Vous aimez que les choses soient prévues. | Changer de programme au dernier moment ? Ça vous contrarie. | improviser |
| E45 | `SelfIndulgence`, stabilité | Vous savez vous tenir. | Boire autant ? Vous n'aimez pas perdre le contrôle. | se laisser aller |
| E46 | `SelfIndulgence`, organisation | Vous êtes raisonnable. | Vous le regretteriez demain. | faire des excès |

### 9.3 Justifications appliquées (12)

À précision « précise », la justification s'ajoute à la raison sous forme de proposition : « Vous ne mentez pas à vos proches, **mais vous voulez la protéger**. » Au bilan, elle s'écrit sous forme nominale, dans « Ce qui l'excusait ».

| N° | Proposition (voix intérieure) | Nominale (bilan) |
|---|---|---|
| J1 | mais on vous attaque | légitime défense |
| J2 | mais vous protégez {cible} | protéger {cible} |
| J3 | mais on vous a provoqué{e} | provocation subie |
| J4 | mais vous n'avez pas le choix : {besoin} | nécessité |
| J5 | mais c'est un ordre | ordre reçu |
| J6 | mais c'est votre métier | devoir de fonction |
| J7 | mais il y a urgence | urgence |
| J8 | mais c'est l'usage | usage ou consentement |
| J9 | mais on vous y force | contrainte subie |
| J10 | mais {cible} vous a fait du tort | riposte à un tort |
| J11 | mais vous réparez un tort | réparation |
| J12 | mais {cible} dépend de vous | devoir envers un proche |

### 9.4 Justifications neutralisées par l'imputabilité (4)

| Cas | Voix intérieure (précise) |
|---|---|
| J1, J3, J10 causées par l'acteur | C'est vous qui l'avez cherché. |
| J4 imputable (moyen fidèle refusé) | Vous aviez le choix. |
| J5 neutralisée (ordre venu du cercle du joueur) | Cet ordre ne vous excuse pas. |
| État passager créé pendant l'incarnation (ivresse, colère entretenue) | Ce n'est pas l'alcool : c'est vous. (Pour la colère : Ce n'est pas la colère : c'est vous.) |

Ces phrases ne disent jamais **pourquoi** l'excuse ne vaut pas en termes de cercle ou de garde. Le joueur sait qui est son complice, et le monde n'en sait rien (revue §3.7).

### 9.5 Planchers de rupture (4)

À précision « précise », la rupture affiche « Il ne peut pas s'y résoudre : » suivi de la forme nominale (interface §2.3).

| Plancher | Nominale |
|---|---|
| R1 | blesser gravement {cible} |
| R2 | se défaire de presque tout ce qu'il a |
| R3 | livrer {cible} |
| R4 | risquer sa vie alors qu'il y a une autre issue |

Total : 46 + 12 + 4 + 4 = **66 entrées**, chacune en deux ou trois formes.

---

## 10. Ce qui reste à faire par des humains

Ce que ce document ne peut pas faire, et qui le sera à la phase indiquée (revue §4.10) :

| Tâche | Qui | Phase | Ce que ce document fournit |
|---|---|---|---|
| Relire les ancres et les archétypes, et changer ce qui ne lui convient pas | Le concepteur, s'il le souhaite : sans réponse, les ancres valent telles quelles | Avant la phase 1 | Les ancres en phrases (§7), les portraits (§6) |
| Campagne initiale de comparaisons (≈ 3 000 à 5 000 paires, ≈ 600 jugements absolus) | 5 à 10 annotateurs | 5 | Les valeurs de départ, point de régularisation ; les ancres et planchers, contraintes dures |
| Test à l'aveugle « fidèle ou non » | Testeurs | 5 | Les archétypes et les portraits |
| Confirmer les annotations et les dérogations sur le paquet réel | Le rapport `ts-data signatures`, relu par Claude, puis par le concepteur pour les dérogations | 1 | Les listes du §5 |
| Relire les gabarits en situation et en écrire les variantes | Claude (brouillons possibles par modèle de langage, G18), relu | 3 | Les 66 gabarits du §9 |

---

## 11. Modifications proposées au socle, aux domaines et à la revue

La session principale décide de ce qu'elle retient. Les domaines appliquent eux-mêmes leurs modifications.

### 11.1 [revues/signatures.md](signatures.md)

Une ligne de renvoi a été ajoutée en tête, sans autre changement. Les précisions M1 à M18 (§1.3) remplacent les passages suivants :

| Section | Passage | Remplacé par |
|---|---|---|
| §3.1 | « niveau de réticence (0 à 4) » | Niveaux 0 à 3 et indicateur de rupture (M8) |
| §3.2 | Magnitude de `LawViolation` « selon la peine type » | Barème M2 (§2.2) |
| §3.3 | Table des facteurs de cible (5 colonnes) | Table complète (§2.3) ; règle M18 |
| §3.6 | J1, J2, J4, J5, J6, J7 | M6, M7 (§3.1) |
| §3.8 | R1 (« victime inconnue »), R4 (« organisation (jugement) ≥ 0,4 ») | M4, M5 (§3.2) |
| §3.12 | « ≈ 180 paramètres » et « ≈ 40 paramètres » | 199 et 53 |
| §4.3, point 2 | Escalade commençant par « insulter » | Échelles du §8 et M11 |

### 11.2 Socle

- **[04-joueur.md](../04-joueur.md), §2** : aucune modification de fond. Le passage « une quarantaine d'actes d'ancrage » reste juste (46).
- **[05-architecture.md](../05-architecture.md), §2.2 ou arborescence** : ajouter le dossier `data/content/conduct/`. Ses fichiers sont listés au §0 de ce document : grammaire, justifications, planchers, archétypes, ancres, échelles, gabarits.
- **[README.md](../README.md)** : citer ce document à côté de la revue des signatures, dans le point 5 du déroulement et dans la relecture de cohérence.

### 11.3 [domaines/combat.md](../domaines/combat.md)

- **§8.1** : ajouter la **violation de domicile** au catalogue (délit, peine type sursis de 3 mois, maximum 1 an). *Raison : entrer chez autrui sans y être invité est une intrusion réelle ; sans infraction, seule une annotation (A23) la rend dissonante.*
- **§8.5** : la magnitude de la violation de la loi suit la peine type (M2). Les aggravations propres à l'auteur ajoutent 0,05 chacune ; celles qui tiennent à la victime passent par la classe de cible.
- **§11.2** : l'échelle déclarée de l'escalade commence à « menacer » (M11). « Insulter » relève de l'échelle de la parole.
- **§1.7, intention « Fouiller »** : dérivée par D31 (`Humiliation` et `Coercion` 0,1, J6 pour la police).

### 11.4 [domaines/economie.md](../domaines/economie.md)

- **§12.1** : un transfert fondé en droit n'est jamais une prise du bien d'autrui (M10, D13).
- **§12.1** : les magnitudes de valeur suivent M3.
- **§7.3** : chaque produit vendu porte un **degré de nocivité** (D16), qui donne la mise en danger de l'acheteur.
- **§5.10** : un objet peut porter l'étiquette **valeur affective** (A15).
- **§7.6** : le jeu d'argent est dérivé par D15 (perte espérée en don, mise en excès envers soi).

### 11.5 [domaines/social.md](../domaines/social.md)

- **§3.1** : annotations A08 à A13 ; dérogations O1 et O3.
- **§6.7** : le facteur de J5 suit M7.
- **§3.1** : les pratiques qui ouvrent J8 (politesse, usages envers les enfants, plaisanterie entre amis) le déclarent dans leurs données.

### 11.6 [domaines/ia-pnj.md](../domaines/ia-pnj.md)

- **§1.4** : J4 couvre le besoin critique d'un proche dépendant (M6).
- **§5.3** : R4 suit M5.
- **§14.1** : les 12 archétypes du §6 servent de grille aux indicateurs « corrélation profil × effets ».

### 11.7 [domaines/interface.md](../domaines/interface.md)

- **§2.3** : seuils de réticence de M8.
- **§4.1** : gabarits du §9, en trois formes (claire, précise, nominale), avec les variables `{cible}`, `{à_cible}`, `{de_cible}`, `{Cible}`, `{groupe}`, `{infraction}`, `{besoin}` et `{e}`.
- **§5.2** : les actes excentriques permis par la surface de jeu et annotés (A20 à A24, A26 à A28) portent leur signe de réticence comme les autres actes ; ils restent des actions de contexte ou des déplacements, sans aucun geste libre (A7).

### 11.8 [domaines/population.md](../domaines/population.md)

- **§4.1** : le contrôle de faisabilité des rôles (revue §4.3, point 4) utilise les profils que le générateur tire pour chaque fiche de rôle. Les 12 archétypes servent au rapport, pas au tirage. Aucune modification de fond.

### 11.9 [domaines/ville.md](../domaines/ville.md)

Aucune modification.
