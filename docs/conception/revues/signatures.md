# Revue des signatures comportementales — des signatures dérivées, composées et calibrées

> Version 1.0 — 29/10/2026. Document **à part**, comme les revues d'[architecture](architecture.md), d'[équilibrage](equilibrage.md) et de [sécurité](securite.md) : la session principale le relit et intègre ce qu'elle retient dans le socle et les domaines. Aucun autre fichier n'a été modifié.
> Les choix marqués **(décidé)** sont ceux de cette revue, avec leur raison. Les valeurs marquées **[à calibrer]** sont des points de départ : elles seront fixées par la chaîne de calibrage du §4.5. Les nombres des exemples sont **illustratifs**.
> Complété le 29/10/2026 par [signatures-contenu.md](signatures-contenu.md) : valeurs de départ de la grammaire, des justifications et des planchers, dériveurs, annotations et dérogations, archétypes, ancres, échelles et gabarits d'explication, ainsi que les précisions M1 à M18, qui remplacent les passages concernés de ce document.
> Périmètre : la définition, le calcul, l'écriture, le contrôle et le calibrage des **signatures comportementales** et de leur **coût** pour un profil. Les règles qui s'en servent (synchronisation, point de rupture, choix des moyens, bilan, témoins) ne changent pas de sens ; seule leur source change.

## Sommaire

0. [Résumé](#0-résumé)
1. [Le système actuel et ses limites](#1-le-système-actuel-et-ses-limites)
2. [Comparaison des pistes](#2-comparaison-des-pistes)
3. [Le système retenu](#3-le-système-retenu)
4. [La chaîne d'outils](#4-la-chaîne-doutils)
5. [Explicabilité](#5-explicabilité)
6. [Décisions](#6-décisions)
7. [Coûts](#7-coûts)
8. [Risques et mesure](#8-risques-et-mesure)
9. [Modifications proposées au socle et aux domaines](#9-modifications-proposées-au-socle-et-aux-domaines)

---

## 0. Résumé

Aujourd'hui, chaque façon d'agir (recette, acte social, geste de combat, opération économique, infraction) doit porter une **signature écrite dans les données**, et chaque contexte qui la change (légitime défense, faim, protection d'un proche) une variante. Avec plusieurs centaines d'actes, plusieurs contextes chacun et neuf axes, cela fait de l'ordre de **20 000 nombres** écrits à la main, sans définition commune de ce qu'ils mesurent, ni moyen de vérifier qu'ils sont cohérents entre eux. La conception a d'ailleurs déjà commencé à s'en écarter : le don (E5) et la complicité (E4) sont définis **par leur effet**, pas par le geste.

Le système retenu généralise ce mouvement :

1. **La signature est dérivée des effets.** Un catalogue fermé de **22 effets élémentaires** (tort physique, prise du bien d'autrui, tromperie, sortie de valeur sans contrepartie, contrainte, violation de la loi, rupture d'engagement, aide, risque pour soi…) décrit une fois pour toutes comment chaque effet pèse sur les neuf axes du profil. Les effets d'un acte sont **lus dans ses données mécaniques**, que les domaines écrivent déjà pour le résoudre (dégâts attendus, écritures du grand livre, croyance créée chez l'auditeur, infraction qualifiée).
2. **Le contexte est une poignée de règles générales.** Douze **justifications** (légitime défense, protection d'autrui, provocation, nécessité, ordre, devoir de fonction, urgence, consentement, contrainte subie…) réduisent la gravité de certains effets, avec un **plafond de proportionnalité**. Elles ne valent pas si l'acteur a lui-même créé la situation (**imputabilité**), ce que la garde hors fiction étend au cercle du joueur.
3. **Les actes composés se calculent.** Un plan, une recette HTN ou un projet a pour signature l'agrégat des effets de ses étapes ; un **épisode** cumule les actes répétés envers la même contrepartie, ce qui rend le découpage d'un acte en petits gestes inutile et garantit que le joueur paie, geste après geste, exactement ce que l'IA paie pour le plan entier.
4. **Le coût pour un profil** suit un modèle à **tolérance et poids** par axe : plus l'effet est grave, moins de profils le tolèrent et plus le dépassement coûte. Quatre **planchers de rupture**, évalués en dernier sur le profil de base, garantissent par construction que les actes extrêmes du point de rupture (E1) coûtent plus que 1.
5. **L'humain n'écrit que l'exceptionnel** : la grammaire des effets et des justifications (≈ 220 paramètres), une annotation d'une ligne pour ≈ 10 à 15 % des actes, une **dérogation bornée** et justifiée pour au plus ≈ 5 %. Les paramètres sont **calibrés hors ligne** à partir de **comparaisons humaines** (« lequel est le plus malhonnête ? »), avec des ancres fixées par le concepteur ; un modèle de langage n'écrit que des brouillons, relus et recoupés.
6. **Une seule fonction**, `appraise`, dans une crate de bibliothèque `ts_conduct`, sert à l'IA, au joueur, aux témoins, au profil du joueur et aux souvenirs. Elle rend un **verdict explicable** : le coût et ses deux raisons principales, codées, que des gabarits mettent en mots (« Elle ne ment pas à ses proches. »).

Le coût en jeu reste faible (≈ 40 à 80 opérations par acte évalué, avec cache par personnage), la reproductibilité est intacte (virgule fixe, ordre canonique, rien n'est appris en jeu), et la revue ajoute des invariants à la chasse aux exploits (anti-découpage, mise en scène des justifications, monotonie, garantie de rupture). **30 décisions** (G1 à G30).

---

## 1. Le système actuel et ses limites

### 1.1 Ce qui existe

| Source | Ce qu'elle dit de la signature |
|---|---|
| [04-joueur.md](../04-joueur.md), §2 et §3 | Une signature par action, qui dépend du contexte ; une seule définition pour la synchronisation et le choix des moyens ; chaque action trop éloignée du profil fait perdre de la synchronisation, « d'autant plus que l'écart et la gravité sont grands » |
| [03-vie-synchronisation.md](../03-vie-synchronisation.md), §1 | Point de rupture : « le barème des signatures garantit » que les actes extrêmes coûtent plus que 1 |
| [ia-pnj.md](../domaines/ia-pnj.md), §1.4 et §3.3 | Chaque recette a une signature ; son coût croît avec la distance au profil ; au-delà d'un seuil, elle est écartée sauf désespoir |
| ia-pnj §3.7, §5.1, §5.6 | Le coût estimé est calculé à l'étage épistémique pour chaque action possible ; il donne le signe de réticence et le point de rupture |
| ia-pnj §4.5, §13.1, §13.5 | Souvenirs dissonants ; évaluation des événements selon les normes ; traits supposés estimés d'après les signatures des actes vus |
| [social.md](../domaines/social.md), §3.1, §3.3, §5.4, §6.1, §6.11 | Chaque acte social a sa signature ; le ton compte ; écart perçu par les témoins ; normes d'un groupe en « signature attendue » ; programme électoral « résumé par une signature de valeurs » |
| [combat.md](../domaines/combat.md), §1.7, §8.1, §11.1 à §11.3 | Chaque intention de combat a une signature dont le poids dépend du contexte ; chaque infraction porte une signature (axes et poids selon la gravité) ; escalade de signatures ; riposte, protection et acharnement distingués |
| [economie.md](../domaines/economie.md), §12.1 | Signature des actions économiques pondérée par le montant rapporté au patrimoine ; toute sortie de valeur sans contrepartie a la signature d'un don au bénéficiaire probable (E5) |
| [equilibrage.md](equilibrage.md), E1, E4, E5, E16, E35, E38 | Point de rupture, complicité par facilitation pondérée par le préjudice, don défini par l'effet, poids des actions dans le profil du joueur selon l'enjeu, garde hors fiction, journal des coûts |
| [interface.md](../domaines/interface.md), §2.3 et §3.8 | Réticence graduée selon la précision, signe de rupture toujours visible, bilan en mots |

### 1.2 Une signature, huit usages

| Usage | Moment | Point de vue | Unité jugée | Ce qu'il demande |
|---|---|---|---|---|
| Choix des moyens par l'IA (ia-pnj §1.4) | Avant d'agir | L'acteur, sur ses croyances | La recette ou le plan | Un coût comparable entre recettes, bon marché, mis en cache |
| Synchronisation du joueur (04 §3) | À chaque intention | L'acteur, sur ses croyances | L'intention élémentaire | Le même coût que pour l'IA, additif sans double compte |
| Réticence et point de rupture (ia-pnj §5.6, E1) | Avant chaque action possible | L'acteur | L'intention | Un coût exact, une garantie pour les actes extrêmes |
| Souvenirs dissonants (ia-pnj §4.5) | Après l'acte | L'acteur | L'acte | Une raison, pour choisir la réparation |
| Évaluation selon les normes, traits supposés (ia-pnj §13.1, §13.5 ; social §5.4) | Quand l'acte est perçu | Le **témoin**, sur ses croyances | L'acte perçu | Le même calcul, avec les valeurs ou les traits supposés du témoin |
| Complicité par facilitation (E4) | Après coup, dans l'heure | Méta-jeu | Le geste et ce qu'il a permis | Un effet différé, pondéré par le préjudice |
| Sorties de valeur assimilées à un don (E5) | Avant d'agir | L'acteur | L'opération | Un effet défini par le patrimoine, pas par le geste |
| Poids des actions dans le profil du joueur (E16) | Après l'acte | Méta-jeu | L'acte | Un enjeu, et une direction dans l'espace du profil |

### 1.3 Les limites

- **L1 — Aucune définition formelle.** Le socle parle d'« écart » et de « gravité », mais ne dit pas ce qu'est une signature : un point de l'espace du profil ? une direction ? des axes seulement concernés ? Un point complet pénaliserait un menteur qui aide un ami (l'axe d'honnêteté n'est pas en jeu), une direction ne dit pas à partir de quand un acte devient dissonant.
- **L2 — L'explosion combinatoire.** Quelques centaines de recettes HTN, ≈ 45 actes sociaux avec cinq tons, ≈ 35 intentions de combat, ≈ 70 infractions, des opérations économiques, des normes de pratiques : 600 à 800 éléments, chacun avec trois à cinq contextes, sur neuf axes. Ces nombres seraient écrits par des sessions différentes, à des moments différents.
- **L3 — Des échelles différentes selon les domaines.** Le combat pondère par l'escalade, l'économie par le montant rapporté au patrimoine, le social par l'acte et le ton, la justice par la peine. Rien ne garantit qu'un vol de 50 écus et une insulte en public se situent l'un par rapport à l'autre comme un humain les situerait.
- **L4 — Deux grains de jugement.** L'IA juge une **recette** (« voler de la nourriture »), le joueur une suite d'**intentions** (entrer, prendre, sortir). Si la signature de la recette n'est pas celle de ses étapes, la promesse « une seule définition » est fausse, et un acte découpé en gestes anodins échappe au jugement (c'est exactement la faille F10, traitée au cas par cas par E4).
- **L5 — Le contexte en variantes.** « Frapper pour protéger son enfant » et « frapper un passant » sont deux signatures ; « frapper un homme à terre » une troisième ; « frapper sur ordre », « frapper par erreur » d'autres encore. Chaque nouveau contexte multiplie le travail, et chaque contexte oublié devient un trou.
- **L6 — La garantie de rupture repose sur chaque auteur.** E1 suppose que tout acte extrême porte un barème assez lourd. Une recette nouvelle mal écrite (un « empoisonner » qui n'existe pas, un « pousser du pont ») passerait sous le seuil sans qu'aucun test le voie.
- **L7 — Le point de vue n'est pas précisé.** Le témoin juge sur ce qu'il a vu (social §5.4), pas sur ce qui s'est passé : s'il n'a pas vu la provocation, il juge plus sévèrement. Une signature écrite d'avance ne peut pas le refléter.
- **L8 — Le tempérament est flou.** Les axes de tempérament font partie du profil, mais la conception dit aussi qu'un écart de tempérament n'entraîne « jamais une mauvaise opinion » (social §5.4). On ne sait pas combien il coûte en synchronisation, ni s'il peut mener à la rupture.
- **L9 — Le mot est surchargé.** « Signature » désigne aussi la norme d'un groupe (social §6.1), le programme d'un candidat (social §6.11) et les signatures cryptographiques des artefacts (sécurité S27).
- **L10 — Rien ne se vérifie.** Il n'existe ni test de cohérence (un acte plus grave coûte-t-il plus ?), ni rapport de ce qui change quand on modifie une valeur, ni lien avec un jugement humain.

---

## 2. Comparaison des pistes

### 2.1 Critères

Effort humain par acte nouveau ; cohérence entre actes et domaines ; **justesse** (accord avec le jugement humain) ; explicabilité ; coût en jeu ; reproductibilité ; résistance aux exploits (revue d'équilibrage) et à la lecture du client (sécurité §1) ; couverture automatique du contenu nouveau.

### 2.2 Les pistes

**A — Statu quo : tout à la main.** Juste là où l'auteur a bien pensé, incohérent ailleurs ; aucune garantie, coût humain maximal. Sert de référence.

**B — Signature dérivée des effets.** Un acte est décrit par ce qu'il fait (tort, tromperie, illégalité, transfert, risque, contrainte, aide, rupture d'engagement) ; une table unique dit comment chaque effet pèse sur les axes. C'est la piste la plus structurante : elle couvre tout acte nouveau dont les effets sont connus, rend les domaines comparables (un vol et une insulte passent par la même échelle) et rend les garanties vérifiables (tout acte qui cause une blessure mortelle attendue tombe sous la même règle). Limite : les actes dont le sens n'est pas dans leurs effets mécaniques (rire à un enterrement, courir sans raison en public) demandent une annotation ; et la table des effets doit être juste, d'où le calibrage.

**C — Composition.** Un plan tire sa signature de ses étapes et de son but. Indispensable pour L4 ; inutile seule (il faut bien que les étapes aient une signature) ; naturelle avec B, puisque des effets s'additionnent alors que des signatures écrites ne se composent pas.

**D — Modificateurs de contexte en règles générales.** Une douzaine de règles remplacent des centaines de variantes. Le risque propre à D est la **mise en scène** : provoquer l'agression pour tuer en légitime défense, affamer le personnage pour voler par nécessité, se faire donner l'ordre par un complice. Il se traite par l'imputabilité (§3.7), qu'une règle générale permet justement d'appliquer partout.

**E — Génération hors ligne par un modèle de langage.** Permise par ia-pnj §13.14. Bonne pour écrire des brouillons (annotations des cas que les dériveurs ne couvrent pas, vignettes d'annotation, gabarits d'explication) ; mauvaise comme source de **nombres** : non calibrée, instable d'une version à l'autre, biaisée, et sans garantie de monotonie. Retenue comme outil d'écriture, jamais comme source de vérité.

**F — Calibrage hors ligne par annotations humaines comparatives.** Les humains sont fiables pour **comparer** (« lequel est le plus malhonnête ? ») et peu fiables pour noter dans l'absolu ; un modèle ordinal (Bradley-Terry) transforme des milliers de comparaisons en paramètres. C'est la seule source de **justesse** au sens du jeu (« ce personnage ne ferait pas ça »), et elle se combine parfaitement avec B : on calibre ≈ 220 paramètres de la grammaire, pas 20 000 signatures.

**G — Apprentissage sur les données de jeu.** Rejeté comme vérité de terrain : les joueurs ne se comportent pas comme les personnages, et ce qu'ils font souvent n'est ni naturel ni moral ; apprendre d'eux récompenserait les exploits (un geste que les joueurs font beaucoup parce qu'il coûte peu deviendrait « normal »). Retenu comme **détecteur** : un acte beaucoup plus fréquent chez les joueurs que chez l'IA de profils semblables, à faible coût, est un candidat à la sous-évaluation (§4.7).

**H — Modèle appris opaque** (réseau, plongements de texte qui prédisent la signature d'après la description de l'acte). Rejeté : inexplicable, non monotone, sensible à la formulation, exploitable de façon imprévisible ; c'est la raison même pour laquelle les décisions des PNJ ne sont pas apprises (ia-pnj §13.13).

**I — Exemples et plus proches voisins** (la signature d'un acte nouveau est celle des actes annotés les plus semblables). Utile comme **contrôle** (un acte dont la signature dérivée s'écarte beaucoup de celle de ses voisins est signalé), insuffisant comme méthode : la similarité de deux actes est justement ce qu'on cherche à définir.

**J — Couche intermédiaire de « fondations morales »** (soin, équité, loyauté, autorité, pureté, liberté). Séduisante en théorie, rejetée : le socle a fixé neuf axes, une couche de plus doublerait les paramètres sans rien expliquer de plus au joueur ; les effets se projettent directement sur les axes du socle.

### 2.3 Tableau comparatif

| Piste | Effort par acte nouveau | Cohérence | Justesse | Explicabilité | Coût en jeu | Exploits | Retenue |
|---|---|---|---|---|---|---|---|
| A Tout à la main | Très élevé | Faible | Variable | Moyenne (un nombre par acte) | Minimal | Trous par oubli | Non (sauf dérogations bornées) |
| B Dérivée des effets | Nul à faible | Forte | Bonne une fois calibrée | Forte (effet → raison) | Faible | Garanties vérifiables | **Oui, structure** |
| C Composition | Nul | Forte | Bonne | Forte | Faible | Ferme le découpage | **Oui** |
| D Règles de contexte | Nul | Forte | Bonne | Forte (« il se défendait ») | Faible | Mise en scène, à traiter | **Oui, avec imputabilité** |
| E Modèle de langage hors ligne | Faible | Moyenne | Inconnue | Moyenne | Nul | Risque d'erreur silencieuse | **Oui, brouillons relus** |
| F Comparaisons humaines | Ponctuel | Forte | **La référence** | Forte | Nul | Nul | **Oui, réglage** |
| G Données de jeu | Nul | — | Fausse | — | Nul | Apprend les exploits | **Détecteur seulement** |
| H Modèle opaque | Nul | Moyenne | Inconnue | Nulle | Moyen | Imprévisibles | Non |
| I Plus proches voisins | Faible | Moyenne | Moyenne | Moyenne | Faible | — | **Contrôle seulement** |
| J Fondations morales | Faible | Forte | Bonne | Moins bonne | Faible | — | Non |

### 2.4 Conclusion

Le système retenu combine **B, C et D comme structure**, **F comme réglage**, **E comme outil d'écriture**, **G et I comme détecteurs**. C'est la répartition déjà choisie pour les décisions des PNJ (ia-pnj §13.13) : une structure explicable écrite à la main, des poids réglés hors ligne contre des jugements humains, rien d'appris en jeu.

Les jeux qui ont affronté ce problème vont dans le même sens. Les **systèmes à jauge unique** (karma, réputation morale) sont trop pauvres pour neuf axes. Les jeux qui écrivent une réaction de personnalité **par décision** (stress des traits dans les jeux de dynastie) paient un coût de contenu qui croît avec chaque décision ajoutée. Ceux qui font réagir des valeurs ou des préceptes à des **types d'événements** génériques plutôt qu'à des actes (systèmes d'idéologies et de valeurs des simulations de colonie) obtiennent une couverture large pour peu d'écriture : c'est l'idée des effets. Les modèles de jeux sociaux en données (Versu, Comme il faut) montrent enfin que des **règles générales sur des faits structurés** suffisent à juger une scène, à condition de disposer de bonnes exceptions.

---

## 3. Le système retenu

### 3.1 Vue d'ensemble

```
INTENTION ou RECETTE (données RON du domaine)
   │  dériveur statique (ts-data build) : gabarit d'effets, emplacements de paramètres
   ▼
EFFETS ÉLÉMENTAIRES  ← descripteur du domaine à l'exécution (croyances de l'acteur) :
   │                     montant, blessure attendue, cible, relation, propriété, loi
   │  circonstances (calculées par le cœur, depuis les croyances)
   ▼
JUSTIFICATIONS (12 règles, plafond de proportionnalité, imputabilité)
   ▼
ÉPISODE (cumul par type d'effet et contrepartie : anti-découpage, coût marginal)
   ▼
AXES (projection de chaque effet, agrégation « ou bruité » par axe et par sens)
   ▼
COÛT POUR UN PROFIL (tolérance et poids par axe ; tempérament plafonné)
   ▼
PLANCHERS DE RUPTURE (4 règles, sur le profil de base)
   ▼
VERDICT : coût · niveau de réticence (0 à 4) · rupture oui/non · deux raisons codées
```

Tout cela est **une seule fonction pure**, `ts_conduct::appraise`, qui reçoit des effets, des circonstances, un profil, des paramètres et un mode, et rend un verdict (§3.11). Elle ne lit ni le monde ni le pilote.

### 3.2 Les effets élémentaires (décidé)

Le catalogue est **fermé** : ajouter un type d'effet est une décision de conception, rare, versionnée avec le paquet de contenus. Chaque effet a un **type**, une **magnitude** de 0 à 1, une **classe de cible** et, si besoin, une **contrepartie** (`StableId` de la victime, du bénéficiaire, du bien, ou « anonyme »).

| N° | Effet (identifiant) | Magnitude : d'où elle vient | Axes touchés (pôle vers lequel l'acte tire) |
|---|---|---|---|
| 1 | Tort physique à autrui (`HarmPhysical`) | Gravité **attendue** de la blessure d'après le modèle de blessure (combat §2) : douleur 0,05, blessure légère 0,2, incapacité 0,45, grave 0,7, mortelle 1 | Violence (force), bienveillance (cruauté) |
| 2 | Usage de la force sans blessure attendue (`Force`) | Pousser 0,2, saisir 0,3, immobiliser 0,4 | Violence (force) |
| 3 | Contrainte (`Coercion`) | Intensité de la menace (verbale, geste, arme montrée, arme braquée) et de ce qui est exigé | Violence (force), bienveillance (cruauté) |
| 4 | Mise en danger d'autrui (`Endangerment`) | Probabilité × gravité du dommage possible, selon les croyances (conduite en état d'ivresse, tir près de passants) | Bienveillance (cruauté), stabilité (impulsif) |
| 5 | Atteinte à la dignité (`Humiliation`) | Insulter, se moquer, humilier ; × 1,5 en public **[à calibrer]** | Bienveillance (cruauté), stabilité (impulsif) |
| 6 | Tort matériel (`PropertyDamage`) | Dommage rapporté au patrimoine de la victime, avec un plancher en valeur absolue | Bienveillance (cruauté), violence (force) |
| 7 | Prise du bien d'autrui (`Appropriation`) | Valeur rapportée au patrimoine de la victime, avec un plancher en valeur absolue | Honnêteté (manipulation), bienveillance (cruauté) |
| 8 | Tromperie (`Deception`) | Enjeu pour la cible de ce qu'on lui fait croire (rien, un détail, une décision, de l'argent, sa sécurité) | Honnêteté (manipulation) |
| 9 | Trahison de confiance (`BreachOfTrust`) | Révéler une confidence, agir contre son employeur ou son groupe, infidélité, abus d'une position | Honnêteté (manipulation), bienveillance (cruauté) |
| 10 | Manquement à une obligation (`CommitmentBroken`) | Obligation de l'agenda, rendez-vous, promesse, devoir de garde, selon son importance | Organisation (perception), respect des règles (hors-la-loi) |
| 11 | Violation de la loi (`LawViolation`) | Qualification de l'infraction par le catalogue de la justice (combat §8.1) : contravention 0,1 ; délit de 0,3 à 0,7 selon la peine type ; crime 0,9 à 1 | Respect des règles (hors-la-loi) |
| 12 | Manquement à une norme (`NormViolation`) | Poids de la norme dans la pratique ou le groupe (ia-pnj §13.9, social §6.1) | Respect des règles (hors-la-loi), sentiment (pensée) |
| 13 | Sortie de valeur sans contrepartie (`GiftOut`) | Montant net (valeur sortie moins contrepartie au prix de référence) rapporté au **patrimoine** de l'acteur (E5) | Bienveillance (compassion), organisation (perception) |
| 14 | Profiter d'autrui (`Exploitation`) | Écart au prix de référence au détriment d'une cible vulnérable ou dépendante (usure, abus de faiblesse) | Bienveillance (cruauté), honnêteté (manipulation) |
| 15 | Facilitation (`Facilitation`) | Ouvrir ou laisser un accès protégé, remettre une clé, un code (par référence), couper une alarme, annoncer une absence, quitter une garde (E4) ; reprend la gravité du tort rendu possible, × 0,7 **[à calibrer]** | Ceux du tort rendu possible, plus honnêteté (trahison) si le bien protégé est celui de l'employeur, d'un proche ou de soi |
| 16 | Instigation (`Instigation`) | Ordonner, commanditer, recruter pour un tort ; reprend la gravité du tort, × 0,8 **[à calibrer]** | Ceux du tort commandé |
| 17 | Aide (`Help`) | Soigner, secourir, protéger, rendre service, consoler, selon le coût pour soi et le bénéfice pour l'autre | Bienveillance (compassion) |
| 18 | Risque pour soi (`SelfRisk`) | Probabilité × gravité attendue pour soi, selon les croyances | Stabilité (calme : l'anxieux s'en écarte), violence (force), organisation (perception) |
| 19 | Emportement (`Outburst`) | Crier, éclater, geste brusque, courir sans raison apparente en public (interface J§7) | Stabilité (impulsif) |
| 20 | Peine causée sans tort (`EmotionalImpact`) | Refuser durement, rompre, critiquer, licencier : la détresse prévue chez l'autre | Décision (pensée), bienveillance (cruauté), faible |
| 21 | Penchant de tempérament (`TemperamentLean`) | Lu dans les étiquettes d'activité et d'acte qui existent déjà (ia-pnj §1.4) : seul ou en groupe, calme ou animé, connu ou nouveau, planifié ou imprévu, efficacité ou égards | Un des quatre axes de tempérament, vers un pôle |
| 22 | Excès envers soi (`SelfIndulgence`) | S'enivrer, consommer un produit, dépenser sans compter pour soi, selon l'ampleur | Stabilité (impulsif), organisation (perception) |

Un acte réel porte rarement plus de **quatre effets** : « voler un portefeuille » donne `Appropriation` et `LawViolation` ; « frapper avec une barre » donne `HarmPhysical` et `LawViolation` ; « mentir à sa sœur sur une dette » donne `Deception` (et `BreachOfTrust` si la dette lui était due).

*Pourquoi un catalogue fermé et court : un humain peut le relire en entier, chaque effet a une phrase d'explication, et chaque garantie (rupture, monotonie) s'écrit une fois par effet au lieu d'une fois par acte.*

### 3.3 Magnitude, classe de cible, gravité (décidé)

La **gravité brute** d'un effet est :

```
g = borne[0,1]( G_t(m) × R_t(c) )
```

- `G_t` est une **courbe de gravité** propre au type (monotone, trois paramètres) qui transforme la magnitude en gravité : un vol de 1 % du patrimoine de la victime est anodin, de 30 % très grave.
- `R_t(c)` est le **facteur de cible** : la même tromperie pèse plus envers un intime, la même prise moins envers une grande enseigne. Table unique, écrite une fois :

| Classe de cible | `HarmPhysical` | `Deception`, `BreachOfTrust` | `Appropriation`, `PropertyDamage` | `GiftOut` | `Help` |
|---|---|---|---|---|---|
| Intime (conjoint, enfant, parent) | 1,4 | 1,3 | 1,3 | 0,3 | 0,5 |
| Proche (ami, famille) | 1,2 | 1,2 | 1,2 | 0,6 | 0,7 |
| Connaissance, collègue | 1,0 | 1,0 | 1,0 | 0,9 | 1,0 |
| Inconnu | 1,0 | 0,8 | 1,0 | 1,0 | 1,0 |
| Ennemi déclaré, auteur d'un tort récent | 0,7 | 0,6 | 0,7 | 1,2 | 1,3 |
| Personne vulnérable (âgée, malade, à terre, sans défense) | 1,3 | 1,2 | 1,3 | 1,0 | 0,8 |
| Organisation anonyme (enseigne, administration) | — | 0,7 | 0,6 | 1,0 | 1,0 |
| Bénéficiaire probable inconnu (banc, poubelle, jeu) | — | — | — | 1,0 | — |

Toutes les valeurs sont **[à calibrer]** (§4.5). La classe vient des **croyances** de l'acteur : il ne sait pas que l'inconnu est un policier en civil.

*Pourquoi des facteurs par classe plutôt que par acte : « tromper un intime est pire que tromper un inconnu » est vrai de toutes les tromperies, et s'écrit donc une fois.*

### 3.4 Des effets aux axes (décidé)

Chaque type d'effet a une **projection** sur au plus trois axes : un **pôle** (le côté vers lequel l'acte tire) et un **coefficient** `π` de 0 à 1 (par exemple `HarmPhysical` : violence 1,0 ; bienveillance 0,6). Pour chaque axe `k` et chaque sens `s`, les effets se combinent en **« ou bruité »** :

```
γ_ks = 1 − Π_e (1 − π_tks × g_e)
```

- l'agrégat est borné par 1, croît avec chaque effet ajouté, ne dépend pas de l'ordre, et deux petits effets pèsent moins qu'un grand ;
- **l'ordre canonique** des effets (type, puis contrepartie) est fixé avant le calcul, parce que l'arrondi en virgule fixe n'est pas associatif (§3.11).

*Pourquoi le « ou bruité » : l'addition ferait qu'une poignée de gestes anodins dépasse un acte grave, le maximum ignorerait qu'un acte qui ment et vole est pire qu'un acte qui ment seulement.*

### 3.5 Du profil au coût : tolérance et poids (décidé)

Le profil a neuf axes notés de −1 à +1 (stockés en millièmes). Pour un axe `k` et un sens `s`, on mesure la **position du profil vers le pôle opposé** à celui où tire l'acte, `q_ks` : pour un acte qui tire vers la manipulation, `q` est l'honnêteté du personnage (+1 = très franc).

```
Δ_ks = max(0, q_ks − T_ks(γ_ks))          dépassement de la tolérance
c_ks = W_ks(γ_ks) × Δ_ks                   coût sur l'axe
coût = Σ (axes de valeurs) c_ks  +  min(plafond_T, Σ (axes de tempérament) c_ks)
```

- **`T_ks(γ)`, la tolérance** : la position au-delà de laquelle l'acte devient dissonant. Elle **décroît** avec la gravité : un petit mensonge de politesse n'est dissonant que pour les profils très francs (T proche de +0,8) ; une escroquerie l'est pour presque tous (T proche de −0,6). Courbe monotone à trois paramètres par axe et par sens.
- **`W_ks(γ)`, le poids** : ce que coûte chaque unité de dépassement. Il **croît** avec la gravité, plus vite que linéairement : être un peu trop franc pour un petit mensonge ne coûte presque rien, beaucoup trop doux pour un coup de couteau coûte plus que toute la synchronisation.
- Les **sens** comptent : les dix-huit couples (axe, sens) ont leurs propres courbes. Faire du tort quand on est bienveillant coûte beaucoup ; aider quand on est cruel coûte un peu ; donner une fortune quand on n'est pas un saint coûte beaucoup (c'est `GiftOut`, dont la tolérance est très haute pour les grands montants).
- **Le tempérament** est plafonné (`plafond_T` ≈ 0,03 par acte et ≈ 0,10 par heure de jeu **[à calibrer]**) et n'entre dans aucune règle de rupture (§3.8 ; G12).
- **États passagers** : ivresse, colère, panique décalent le **profil effectif** comme aujourd'hui (le seuil de violence baisse avec l'ivresse, combat §1.6) ; ils ne touchent pas la signature. Pour un pilote `Joueur`, les décalages créés pendant l'incarnation ne comptent pas (imputabilité, §3.7).
- **Unité** : le coût est exprimé en **points de synchronisation** (S va de 1 à 0). C'est le « écart × enjeu » du socle, rendu calculable.

Repères **[à calibrer]** : petit mensonge d'un personnage franc ≈ 0,01 à 0,03 ; mensonge qui engage de l'argent ≈ 0,1 ; gifler un inconnu pour un pacifiste ≈ 0,3 ; acte extrême > 1.

*Pourquoi la tolérance plutôt qu'une distance à un point : un acte n'exige pas un profil précis, il exige seulement de ne pas être trop loin du côté qui le refuse ; un menteur n'est pas dissonant quand il dit la vérité, un honnête homme l'est quand il ment, et d'autant plus que le mensonge est grave.*

### 3.6 Circonstances et justifications (décidé)

Les **circonstances** sont des faits calculés par le cœur, une fois par tick et par personnage en simulation complète (par bloc ailleurs), à partir de ses **croyances** : menace actuelle contre soi ou contre un proche et sa gravité, offense subie récente, besoin critique sans moyen fidèle connu, ordre reçu d'une autorité, rôle en service, danger grave pour un tiers, consentement de la cible, contrainte subie. Ce sont des drapeaux et quelques valeurs, pas du texte.

Une **justification** est une règle générale : si une circonstance tient, la gravité de certains effets baisse d'un facteur `f`, éventuellement jusqu'à un **plafond** seulement (la proportionnalité) :

```
g' = (1 − f) × min(g, plafond) + max(0, g − plafond)
```

| N° | Justification | Condition (croyances de l'acteur) | Effets concernés, facteur `f` **[à calibrer]** | Plafond |
|---|---|---|---|---|
| J1 | Légitime défense | Agression actuelle contre soi | `HarmPhysical`, `Force`, `Coercion` : 0,85 ; `LawViolation` effacée pour la part proportionnée | Gravité de la menace |
| J2 | Défense d'autrui | Agression actuelle contre un tiers | Mêmes effets : 0,9 pour un intime ou un enfant (réflexe de protection), 0,7 pour un inconnu | Gravité de la menace |
| J3 | Provocation | Offense ou humiliation subie de la même personne, depuis peu | `Humiliation`, `Outburst`, `Coercion` : 0,4 ; `HarmPhysical` : 0,2 (atténue, ne justifie pas) | — |
| J4 | Nécessité | Besoin critique et aucun moyen fidèle connu | `Appropriation`, `LawViolation` : 0,5, seulement si la cible n'est pas vulnérable et la magnitude faible | Magnitude de ce qui suffit au besoin |
| J5 | Ordre d'une autorité | Ordre reçu d'un supérieur légitime (contrat, rang) | `CommitmentBroken`, `LawViolation`, `NormViolation` : 0,2 à 0,6 selon le respect des règles de l'acteur ; `HarmPhysical` : au plus 0,3 | — |
| J6 | Devoir de fonction | Le rôle en service autorise l'acte (interpeller, retenir, saisir, soigner) | `LawViolation` effacée ; `Force`, `Coercion` : 0,7 ; `HarmPhysical` : 0,6 | Gravité de la résistance |
| J7 | Urgence, sauvetage | Danger grave pour un tiers que l'acte réduit | `PropertyDamage`, `LawViolation` (mineure), `SelfRisk` : 0,8 | Gravité du danger |
| J8 | Consentement, jeu, usage | La cible consent, ou la pratique l'attend (sport, bluff au poker, marchandage, plaisanterie entre amis) | `Deception`, `Humiliation`, `Force`, `HarmPhysical` léger : 0,9 | Ce que couvre le consentement |
| J9 | Contrainte subie | L'acteur agit sous la menace | Tous sauf `HarmPhysical` grave envers un tiers : 0,6 | — |
| J10 | Riposte proportionnée à un tort | Tort matériel récent subi de la même personne | `PropertyDamage`, `Humiliation` : 0,3 | Gravité du tort subi |
| J11 | Réparation | L'acte répare un tort de l'acteur (rendre, rembourser) | `GiftOut` : 0,9 envers la victime | Montant du tort |
| J12 | Devoir envers un proche dépendant | Nourrir, loger, soigner un enfant ou un parent à charge | `GiftOut`, `SelfRisk` : 0,8 | Ce que demande le besoin |

- **Une seule justification par effet, la plus favorable** : elles ne se cumulent pas (décidé). *Pourquoi : sinon provocation, nécessité et ordre réunis effaceraient n'importe quel acte.*
- **L'ignorance n'est pas une règle** : comme les effets sont lus sur les **croyances** de l'acteur, prendre un objet qu'on croit à soi n'est pas une prise du bien d'autrui, et pousser quelqu'un qu'on ne sait pas cardiaque n'est pas un tort mortel. La **négligence** reste : un risque que l'acteur pouvait connaître garde un `Endangerment` léger.
- **Ce qui n'est pas une justification** : la vengeance, l'intérêt, la loyauté envers une bande. Elles ne réduisent rien ; ce sont les **profils** qui les rendent naturelles (un violent tolère la vengeance, un hors-la-loi le vol).
- La justice du monde utilise les **mêmes** catégories (légitime défense, provocation, contrainte, urgence) pour ses circonstances (combat §8.3), mais sur **son dossier**, pas sur les croyances de l'acteur : un joueur qui s'est vraiment défendu sans témoin ne perd rien en synchronisation et peut être poursuivi quand même.

*Pourquoi des règles générales : une douzaine de règles couvrent toutes les combinaisons acte × contexte, se testent une fois, et donnent une explication (« vous défendiez votre fils ») au lieu d'une variante muette.*

### 3.7 Imputabilité : pas de justification mise en scène (décidé)

Une justification **ne vaut pas** si sa circonstance a été **causée par l'acteur** dans le même épisode : celui qui a provoqué la bagarre n'est pas en légitime défense (comme en droit), celui qui a refusé les moyens fidèles qu'il connaissait n'est pas dans la nécessité, celui qui a mis son proche en danger ne le « protège » pas. Le cœur connaît déjà ces liens de cause (ia-pnj §5.4, E38), sur deux maillons au plus.

Pour un pilote `Joueur`, la **garde hors fiction** (E35) étend la règle, sans que le monde le sache :

- une justification dont la circonstance a été causée par un personnage du **cercle du joueur** dans les 30 jours de jeu (l'ordre donné par le patron qu'incarne un complice, la provocation d'un ami) est neutralisée ;
- les **décalages d'état** créés pendant l'incarnation (ivresse, colère entretenue, faim laissée monter) ne sont pas appliqués au profil effectif ;
- concrètement, l'IA en observation calcule le verdict en recevant de la garde un **masque d'imputabilité** (bits de justifications et d'états à ignorer). La fonction reste la même ; seul un argument change.

*Pourquoi : des règles de contexte générales sont exactement ce qu'un joueur cherche à fabriquer ; l'imputabilité ferme la mise en scène une fois pour toutes, au lieu d'une parade par contexte.*

### 3.8 Planchers de rupture : la garantie de E1 (décidé)

Après tout le reste, quatre règles écrites à la main imposent `coût ≥ 1 + ε` (« impossible même à synchronisation parfaite »). Elles sont évaluées sur le **profil de base** (jamais sur le profil effectif), avec les seules justifications **non imputables**, et elles portent sur des **effets**, donc sur tous les actes présents et futurs :

| Règle | Effet et seuil **[à calibrer]** | Profils concernés (« à contre-emploi ») |
|---|---|---|
| R1 Violence grave | `HarmPhysical` de gravité ≥ 0,7 (blessure grave ou mort attendue) après justifications | Violence ≤ 0,3 **ou** bienveillance ≥ 0,5 ; et, pour une victime inconnue sans aucune justification, tous les profils sauf la cruauté extrême (bienveillance ≤ −0,8) |
| R2 Don massif | `GiftOut` cumulé ≥ 0,5 du patrimoine vers une contrepartie qui n'est pas un intime (bénéficiaires anonymes réunis) | Tous sauf une bienveillance ≥ 0,9 |
| R3 Facilitation contre les siens | `Facilitation` reconnue d'avance (le personnage croit le destinataire non autorisé) contre soi, son employeur ou un intime | Honnêteté ≥ 0 **ou** respect des règles ≥ 0 |
| R4 Mise en danger mortelle de soi | `SelfRisk` avec probabilité de mort attendue ≥ 0,2, alors qu'une issue sans risque est connue | Personnage **prudent** : stabilité ≤ −0,2 **ou** organisation (jugement) ≥ 0,4 |

- La facilitation qui n'est **pas** reconnue d'avance (le gardien ne sait pas qui attend dehors) reste traitée après coup par E4, dans le bilan et dans le souvenir (§3.10).
- Les **règles absolues** (mineurs, mise en danger mortelle de soi non proposée) ne sont pas des coûts : elles restent des refus de la validation, quel que soit le pilote.
- Un acte qui tombe sous un plancher n'est **pas proposé** dans les listes à synchronisation pleine (interface §2.3) ; ce n'est plus à l'auteur de l'acte d'y penser.

*Pourquoi des planchers sur les effets : la garantie de E1 devient une propriété du calcul, vérifiable exactement (§4.8), au lieu de dépendre de la vigilance de chaque auteur de recette.*

### 3.9 Composition : étapes, recettes, plans, épisodes (décidé)

- **Les effets appartiennent aux actions élémentaires.** « Entrer dans la boutique » n'a pas d'effet ; « prendre l'objet » porte la prise et l'infraction. Une recette n'ajoute **aucun effet** propre ; un projet non plus.
- **La signature d'une méthode HTN** est l'agrégat des gabarits d'effets de ses étapes, **calculé à la compilation** du paquet (`ts-data build`), avec les paramètres laissés en emplacements (montant, cible). La signature d'une recette pour un choix donné est celle de la méthode retenue ; celle d'un **projet** est celle de sa méthode la plus probable, ce qui permet de filtrer « se venger » par le profil sans rien écrire.
- **GOAP** utilise le coût de l'agrégat comme coût de chemin ; comme l'agrégat ne fait que croître quand on ajoute une action, la recherche bornée reste correcte. Un plan GOAP promu en recette (ia-pnj §3.3) a sa signature d'office.
- **L'épisode** est la mémoire de la conduite récente du personnage : pour chaque couple (type d'effet, contrepartie), la **somme des magnitudes** de ses actes, tant que moins d'une heure de jeu sépare deux actes ; pour `GiftOut`, `Appropriation` et `Exploitation`, sur **30 jours de jeu glissants**, comme les seuils de l'économie (economie §12.2 et §12.4). Les bénéficiaires anonymes forment une seule contrepartie.
- **Coût marginal** : un acte coûte `coût(épisode + acte) − coût(épisode)`. La somme des coûts des gestes d'un plan est donc **exactement** le coût du plan entier, et dix vols de 100 écus coûtent autant qu'un vol de 1 000 (la gravité est recalculée sur la somme des magnitudes, pas combinée en « ou bruité » quand c'est la même contrepartie).
- L'épisode est un **composant du personnage** (au plus 8 entrées), le même pour l'IA et le joueur : c'est la conduite du personnage, pas celle du pilote.

*Pourquoi : c'est ce qui rend vraie la promesse « une seule définition » malgré les deux grains de jugement (L4), et ce qui ferme par construction le découpage d'un acte en gestes anodins (F10, F12) au lieu de le rattraper au cas par cas.*

### 3.10 Point de vue et moment (décidé)

| Moment | Point de vue | Effets lus sur | Sert à |
|---|---|---|---|
| **Avant l'acte** (étage épistémique) | Acteur | Ses croyances | Choix des moyens, réticence, point de rupture, perte de synchronisation, poids dans le profil du joueur |
| **Quand l'acte est perçu** | Témoin | Les croyances **du témoin** (ce qu'il a vu, ce qu'il sait de la cible), évaluées avec ses valeurs (blâme, §13.1) ou avec les traits qu'il prête à l'acteur (écart perçu, social §5.4) | Émotions, traits supposés, réputation |
| **Après coup** | Acteur | Les conséquences réelles qu'il apprend (la porte a servi à un cambriolage, l'homme poussé est mort) | Réévaluation du **souvenir dissonant** : la culpabilité peut grandir |
| **Après coup** (méta-jeu) | Bilan | Les liens de cause du journal (E38) | Complicité par facilitation (E4), dans le bilan provisoire |

- La synchronisation n'est **jamais** redébitée après coup : un joueur ne perd pas de synchronisation pour une conséquence que le personnage ne pouvait pas prévoir. La sanction d'une facilitation exploitée passe par le bilan (E4), qui est du méta-jeu.
- Un témoin qui n'a pas vu la provocation juge l'acte **sans** la justification J3 ; un témoin qui n'a vu qu'une partie de l'épisode juge la partie vue. C'est la même fonction, avec d'autres entrées.

*Pourquoi : la même règle sur des croyances différentes produit naturellement des jugements différents et réalistes, sans signature « vue de l'extérieur » à écrire.*

### 3.11 Une fonction pour tous les usages (décidé)

```rust
// ts_conduct — bibliothèque pure, sans ECS, sans E/S
pub fn appraise(
    effects: &[MoralEffect],        // effets de l'acte (ou du plan), sur les croyances du point de vue
    circ: &Circumstances,           // drapeaux et valeurs, calculés par le cœur
    profile: &Profile,              // profil de base ou effectif, ou traits supposés, ou profil normatif
    episode: Option<&Episode>,      // conduite récente de l'acteur (coût marginal)
    mask: ImputabilityMask,         // justifications et états à ignorer (garde hors fiction)
    params: &ConductParams,         // grammaire calibrée, issue du paquet de contenus
) -> Verdict;                       // coût (millièmes), niveau 0..4, rupture, 2 raisons codées
```

| Usage | Appel |
|---|---|
| Choix des moyens par l'IA | `appraise(effets de la méthode, circ, profil effectif, épisode, aucun masque)` ; seuil de choix inchangé (ia-pnj §1.4) |
| Réticence, coût estimé, point de rupture | L'IA en observation, pour chaque action possible : même appel, avec le **masque** de la garde pour un pilote `Joueur` |
| Synchronisation | Coût du verdict de l'intention exécutée, retiré de S |
| Souvenir dissonant | Le verdict (raisons comprises) est gardé avec le souvenir ; réévalué si des conséquences réelles sont apprises |
| Blâme et admiration (ia-pnj §13.1) | `appraise` avec les croyances du témoin et **son** profil : coût élevé = blâmable ; effets `Help` sous tolérance = louable |
| Écart perçu (social §5.4) | `appraise` avec les croyances du témoin et les **traits supposés** de l'acteur |
| Traits supposés (ia-pnj §13.5) | **Révélation** : un acte de gravité γ sur un axe montre que l'acteur tolère au moins `T(γ)` ; la croyance du témoin se resserre vers cette borne, avec une certitude qui croît avec `W(γ)` |
| Profil du joueur (E16) | Même révélation, poids proportionnel à γ (l'enjeu) ; rendements décroissants inchangés |
| Profil des enfants, biographie (ia-pnj §8.5, population) | Un souvenir marquant déplace le profil vers le pôle de ses effets, de γ × taux **[à calibrer]** |
| Normes d'un groupe, manquement (social §6.1, ia-pnj §13.9) | `appraise` avec le **profil normatif** du groupe : coût = gravité du manquement aux yeux du groupe |
| Complicité (E4) | Effet `Facilitation` créé après coup par le lien de cause, évalué par `ts_meta` avec la même fonction |

Calcul en **virgule fixe** (millièmes pour les profils et les coûts, Q16 pour les gravités), effets triés dans un ordre canonique. *Pourquoi : la comparaison au point de rupture se fait à la frontière ; un calcul entier donne le même résultat au bit près dans le cœur, dans `ts_meta`, dans Python (`ts_py` appelle la même fonction) et dans les tests, et permet la vérification exacte du §4.8.*

### 3.12 Ce qui reste écrit à la main (décidé)

| Quoi | Volume | Qui l'écrit | Fréquence |
|---|---|---|---|
| **Grammaire** : 22 effets, projections, courbes de gravité, tolérance et poids, facteurs de cible | ≈ 180 paramètres | Claude (structure) ; valeurs calibrées (§4.5) | Une fois, puis recalibrage |
| **Justifications** (12) et **planchers de rupture** (4) | ≈ 40 paramètres | Claude, relu par le concepteur | Rarement |
| **Dériveurs** : de l'effet mécanique à l'effet élémentaire, par domaine | ≈ 30 règles de code | Claude | Quand un domaine ajoute un type d'effet mécanique |
| **Annotations** d'acte : un ou deux effets que les données mécaniques ne montrent pas (une norme, un geste excentrique, un sens symbolique) | ≈ 10 à 15 % des actes, une ligne chacun | Auteur du contenu, brouillon possible par modèle de langage | À la création de l'acte |
| **Dérogations bornées** : correction d'au plus ± 0,15 sur un axe, avec une phrase de justification obligatoire | Cible ≤ 5 % des actes ; alerte de la CI au-delà | Auteur, relue par le concepteur | Exceptionnel |
| **Ancres** : ≈ 40 actes types dont le concepteur fixe la place (« jamais pour un doux », « anodin pour presque tous ») | ≈ 40 | **Le concepteur** | Une fois, puis à chaque grand ajout |
| **Gabarits d'explication** : une phrase par (effet, axe, sens), variantes par classe de cible et par précision | ≈ 60 à 80 | Brouillon par modèle de langage, relu | Une fois |

Une dérogation ne peut **jamais** faire passer un acte sous un plancher de rupture, ni inverser l'ordre d'une échelle déclarée (§4.3).

### 3.13 Exemples (valeurs illustratives)

**1. Mentir à sa sœur sur une dette.** Lucie, honnêteté +0,6. Effet : `Deception`, magnitude 0,3 (une décision d'argent), cible intime (× 1,3) → g = 0,39 ; honnêteté γ = 0,39 ; tolérance T(0,39) ≈ +0,1 ; dépassement 0,5 ; poids W ≈ 0,25 → **coût ≈ 0,125**. Réticence de niveau 2 (« elle hésite »). Raison : « Elle ne ment pas à ses proches. »

**2. Frapper son agresseur, puis le frapper à terre.** Marc, violence −0,6, bienveillance +0,5. Coup de poing : `HarmPhysical` 0,2, `LawViolation`. Pendant l'agression (menace 0,45) : J1, plafond 0,45 ≥ 0,2 → g ≈ 0,03, infraction effacée → **coût ≈ 0,01**. L'agresseur tombe et renonce : la menace est nulle, J1 ne s'applique plus, la cible est vulnérable (× 1,3) → g ≈ 0,26, plus l'infraction → **coût ≈ 0,3** (« s'acharner »). Une seule intention, aucune variante écrite : c'est combat §11.1, calculé.

**3. La liasse sur le banc, en cinq fois.** Paul, patrimoine 9 000 écus. Poser 980 écus sur un banc : `GiftOut` vers le bénéficiaire anonyme, magnitude 0,11. Les poses suivantes, sur d'autres bancs, s'ajoutent à la **même** contrepartie anonyme de l'épisode (30 jours). À la cinquième, le cumul atteint 0,54 : R2 s'applique, l'acte **n'a pas lieu**, Paul hésite et remet la liasse dans sa poche. Ni le banc ni le découpage ne changent rien (E5, F12).

**4. Le policier.** Interpeller avec force un suspect qui résiste : `Force`, `Coercion` ; J6 (devoir de fonction), infraction effacée → coût faible pour un policier légaliste. Frapper le même suspect menotté : résistance nulle, plafond de J6 dépassé, cible vulnérable, et la justice qualifie « violences par une personne dépositaire de l'autorité publique » → coût fort, rupture pour un policier doux (R1 si la blessure attendue est grave).

**5. L'honnête homme affamé.** Recette « voler de la nourriture » pour l'IA : méthode = entrer (rien), prendre (`Appropriation` 0,02 sur une grande enseigne × 0,6, `LawViolation` 0,1), sortir (rien). Faim critique sans moyen fidèle connu : J4, facteur 0,5. Coût sous le seuil des moyens désespérés : l'IA le choisit, avec stress et souvenir dissonant (ia-pnj §1.4). Un joueur qui incarne le même homme et fait les mêmes gestes perd exactement ce coût ; s'il a refusé un repas offert un peu avant, J4 est imputable et ne compte pas.

---

## 4. La chaîne d'outils

### 4.1 Place dans les crates (décidé)

| Crate | Rôle |
|---|---|
| `ts_model` | Types partagés : `MoralEffect`, `EffectKind`, `TargetClass`, `Circumstances`, `Verdict`, `ReasonCode` ; trait `ConductDescriber` (un par domaine) |
| **`ts_conduct`** (nouvelle) | Bibliothèque **pure**, au niveau de `ts_data` : `appraise`, agrégation, justifications, planchers, révélation, épisode ; aucun système ECS, aucune E/S |
| `ts_data` | Compilation des **gabarits** de signature (dériveurs statiques), contrôle de cohérence, rapport de différences |
| Greffons (`ts_combat`, `ts_economy`, `ts_social`, `ts_justice`, `ts_ai`, `ts_world`, `ts_body`) | Chacun fournit son **descripteur** (`ConductDescriber`) qui remplit les paramètres à l'exécution : blessure attendue, montant net, croyance créée, qualification de l'infraction, norme de la pratique ; branchés par `ts_game` comme `PriceOracle` (revue d'architecture §5.2) |
| `ts_ai` | Choix des moyens, IA en observation, circonstances, épisode |
| `ts_pilot` | Garde hors fiction : masque d'imputabilité, comparaison au point de rupture |
| `ts_view` | Niveau de réticence quantifié et code de raison vers l'interface |
| `ts_meta` | Complicité (E4) et poids du profil du joueur (E16), avec `appraise` |
| `ts_runner` (`ts-sim`), `ts_py`, `python/ts_gym` | Invariants, scénarios, calibrage, rapports |

*Pourquoi une crate de bibliothèque à part : les greffons ne dépendent jamais les uns des autres (revue d'architecture §5.2), or cinq domaines ont besoin du même calcul ; le placer sous les greffons, comme `ts_data`, respecte la règle et garantit une seule implémentation. Pourquoi ce nom : « signature » désigne déjà les signatures cryptographiques des artefacts (S27), et deux sens du même mot dans le code finiraient par se confondre.*

### 4.2 Compilation du paquet : dériveurs et gabarits (décidé)

1. Chaque domaine décrit déjà en RON les **effets mécaniques** de ses actes, pour les résoudre : dégâts, écritures du grand livre, croyance transmise et sa vérité pour l'orateur, engagement créé ou rompu, infraction, norme de pratique, étiquettes d'activité.
2. `ts-data build` passe chaque acte dans les **dériveurs statiques** de son domaine, qui produisent un **gabarit d'effets** : la liste des effets élémentaires, avec les magnitudes fixes et les **emplacements** des paramètres connus seulement à l'exécution.

| Effet mécanique (exemple) | Effet élémentaire dérivé |
|---|---|
| `Damage { weapon, zone, force }` (combat) | `HarmPhysical` (gravité attendue par le modèle de blessure) ; `Force` si la gravité attendue est nulle |
| `Transfer { from, to, amount, counterpart }` (économie) | `GiftOut` si la valeur nette sort sans contrepartie au prix de référence ; `Appropriation` si `from` n'est pas l'acteur et ne consent pas ; `Exploitation` si l'écart joue contre une cible vulnérable |
| `AssertBelief { believed_true: false }` (social) | `Deception` (enjeu = ce que la cible en décidera) |
| `RevealFact { discretion_commitment }` (social) | `BreachOfTrust` |
| Ton menaçant, `Threaten`, arme braquée (social, combat) | `Coercion` |
| `Insult`, `Humiliate` (social) | `Humiliation` (× public) |
| `OpenAccess { protected_party }` (ville, économie) | `Facilitation` si le destinataire n'est pas autorisé selon les croyances |
| `MissObligation` (agenda) | `CommitmentBroken` |
| Qualification par le catalogue des infractions (justice) | `LawViolation` |
| Norme de la pratique ou du groupe (IA, social) | `NormViolation` |
| Étiquettes d'activité (seul / groupe, connu / nouveau, planifié / imprévu) | `TemperamentLean` |
| `Care`, `Rescue`, `Protect`, `Console` | `Help` |
| Risque estimé pour soi ou pour autrui | `SelfRisk`, `Endangerment` |

3. Les **annotations** et **dérogations** de l'acte (§3.12) sont appliquées, puis les signatures des méthodes HTN sont agrégées (§3.9).
4. Le paquet contient les gabarits, les paramètres de `ConductParams` et les gabarits d'explication ; son empreinte couvre le tout (revue d'architecture §5.4).

À l'exécution, un descripteur n'a qu'à **remplir** les emplacements (montant, cible, classe de relation, blessure attendue selon l'arme et la distance) à partir des croyances. *Pourquoi : l'essentiel du travail se fait une fois, à la compilation, et le calcul en jeu se réduit à quelques multiplications.*

### 4.3 Contrôle de cohérence (`ts-data check`) (décidé)

Bloquant dans l'intégration continue, avec des messages en français :

1. **Couverture** : tout effet mécanique a un dériveur ; tout acte qui crée un tort, une croyance, un transfert ou une infraction a au moins un effet élémentaire ; un acte sans aucun effet est signalé (« Cet acte est neutre pour tous les profils : est-ce voulu ? »).
2. **Échelles déclarées** : les chaînes d'ordre écrites une fois (escalade du combat : insulter < menacer < frapper < prendre une arme < frapper avec < viser < tirer < s'acharner ; vol simple < vol avec violence ; politesse < mensonge d'intérêt < escroquerie) doivent rester **monotones** pour tous les profils d'une grille de 3⁹ points.
3. **Planchers** : pour chaque règle R1 à R4, aucun acte qui la déclenche ne coûte moins que 1 + ε dans la zone « à contre-emploi » (vérification **exacte** : le coût est linéaire par morceaux en chaque axe, donc il suffit de tester les sommets de la zone).
4. **Faisabilité des rôles** : pour chaque fiche de rôle, les recettes de cœur de métier (le policier interpelle, le revendeur écoule, le boucher découpe) coûtent moins que le seuil de choix pour les profils que le générateur de population tire pour ce rôle ([population.md](../domaines/population.md)) ; sinon le rôle serait injouable par sa propre IA.
5. **Moyens pour chaque catégorie** : pour chaque catégorie comportementale (04 §3) et chaque besoin important, au moins deux moyens fidèles existent.
6. **Doublons** : deux actes aux effets identiques ont la même signature ; s'ils doivent différer, une annotation le dit.
7. **Dérogations** : bornées (± 0,15), justifiées, jamais sous un plancher, part totale ≤ 5 % ; chaque dérogation apparaît dans le rapport.
8. **Voisinage** (piste I) : un acte dont le coût dérivé s'écarte fortement de celui de ses voisins dans l'espace des effets est signalé pour relecture.
9. **Ensemble de référence** : ≈ 200 actes × 12 profils archétypes × 3 contextes, avec des **bandes attendues** tirées des ancres du concepteur et des comparaisons humaines (§4.5) ; un résultat hors bande bloque.

`ts-data signatures` produit en plus un **rapport** lisible par le concepteur : pour chaque acte, son coût pour les 12 archétypes, ses raisons, et, à chaque changement de contenu ou de paramètres, la **liste des actes dont le coût a changé** de plus de 0,02 **[à calibrer]** pour un archétype. *Pourquoi : un changement de grammaire touche toute la ville ; il doit se relire comme un changement de règle.*

### 4.4 Génération assistée hors ligne (décidé)

Usage permis par ia-pnj §13.14 (écrire hors ligne des contenus relus et versionnés). Le modèle de langage écrit des **brouillons**, jamais des nombres du paquet :

- **Annotations d'actes nouveaux** : à partir de la description et des données mécaniques d'un acte, il propose les effets que les dériveurs n'ont pas trouvés, avec une phrase de raison. Un recoupement automatique **rejette** toute proposition qui contredit les données mécaniques (un « tort physique » sur un acte sans dégâts, une « tromperie » sur un acte qui ne transmet aucune croyance) ; le reste est relu et entre par une modification ordinaire des données.
- **Vignettes d'annotation** : il formule les petites scènes des campagnes de comparaison (§4.5) à partir de gabarits structurés (acte, cible, contexte, portrait du personnage) ; chaque vignette est relue.
- **Gabarits d'explication** et **intuitions** : phrases par (effet, axe, sens, classe de cible, précision).
- **Juge de tri** : pour choisir les comparaisons à soumettre aux humains, on cherche les paires où le modèle calibré et un modèle de langage sont en désaccord ; le jugement du modèle de langage n'entre **jamais** dans l'ajustement.

*Pourquoi : un modèle de langage écrit vite et bien des descriptions, mais ses nombres ne sont ni calibrés ni stables d'une version à l'autre ; le cantonner aux brouillons et au tri garde son apport sans son risque.*

### 4.5 Calibrage par comparaisons humaines (décidé)

**Ce qu'on demande aux annotateurs**, sur des vignettes :

1. **Comparaison sur un axe**, sans personnage : « Lequel de ces deux actes est le plus malhonnête ? le plus violent ? le plus contraire aux règles ? » Cela règle les courbes de gravité, les projections et les facteurs de cible.
2. **Comparaison pour un personnage** décrit par un court portrait en mots (jamais des nombres) : « Pour Lucie, lequel serait le plus contraire à sa nature ? » Cela règle la tolérance et les poids.
3. **Jugement absolu en trois degrés** : « Ce personnage pourrait-il faire cela ? sans difficulté / à contrecœur / jamais ». Cela règle les seuils de réticence et vérifie les planchers.
4. **Justifications** : la même vignette avec et sans le contexte (« … alors qu'il venait d'être frappé »). Cela règle les facteurs `f` et les plafonds.

**Le modèle** : le coût calculé par `appraise` est la variable latente d'un modèle **Bradley-Terry** (comparaisons) et d'un modèle **logistique ordinal** (degrés), ajustés par maximum de vraisemblance avec une régularisation vers les valeurs de départ du concepteur. La **monotonie** est garantie par la paramétrisation (courbes construites comme des sommes de termes positifs), et les **planchers** et les **ancres** sont des contraintes **dures** : un ajustement qui les viole est refusé, pas corrigé en silence. L'ajustement appelle la fonction Rust elle-même à travers `ts_py` : il n'y a pas de seconde implémentation à garder fidèle.

**L'échantillonnage** est **actif** : on soumet d'abord les paires où le modèle est le plus incertain, celles où annotateurs et modèle ont divergé, et celles du tri du §4.4.

**Volumes** **[à calibrer]** : une campagne initiale de ≈ 3 000 à 5 000 comparaisons et ≈ 600 jugements absolus, par 5 à 10 annotateurs (testeurs, concepteur), à raison de 15 à 30 secondes par comparaison ; puis ≈ 100 à 300 comparaisons par ajout important de contenu.

**Qualité** :

- **accord entre annotateurs** mesuré sur 10 % de paires répétées : il fixe le plafond de justesse atteignable ;
- **cohérence de chaque annotateur** (paires répétées, transitivité) : un annotateur incohérent est écarté ;
- **diversité** du panel (âges, milieux), pour limiter le biais d'une seule sensibilité ;
- **le concepteur tranche** : ses ancres priment, et un désaccord persistant entre son ancre et le panel lui est soumis avec les vignettes en cause.

**Validation** : sur des paires tenues à part, le modèle doit retrouver le choix majoritaire des annotateurs dans **au moins 85 %** des cas où ceux-ci s'accordent à plus de 80 % **[à calibrer]** ; corrélation de rang (Kendall) ≥ 0,8 sur l'ensemble de référence.

Les paramètres obtenus sont **versionnés** dans le paquet, relus avec le rapport de différences (§4.3), puis testés par les scénarios (§4.8) avant livraison, exactement comme les poids des décisions (ia-pnj §13.13).

*Pourquoi des comparaisons : les humains classent bien et notent mal ; un modèle ordinal sur 200 paramètres extrait d'eux ce qu'aucun auteur ne saurait écrire pour 20 000 nombres, et le concepteur, qui ne code pas, conçoit en jugeant des scènes.*

### 4.6 Validation comportementale et tests à l'aveugle (décidé)

- **Ensuite seulement**, les poids des **décisions** (seuil de choix des moyens, inertie, tirages) sont calibrés sur les indicateurs de `ts-sim` (ia-pnj §13.13, §14.1), en gardant **fixes** les paramètres de signature : la boucle comportementale ne peut pas déformer ce que les humains ont jugé. Si un indicateur est inatteignable sans toucher aux signatures, c'est signalé au concepteur, qui arbitre.
- **Indicateurs ajoutés** à ia-pnj §14.1 : corrélation entre chaque axe du profil et la fréquence des **effets** correspondants (et non plus des actes) ; part des actes de l'IA dont le coût dépasse son seuil (≈ 0 hors désespoir) ; part des choix sous désespoir par profil.
- **Test à l'aveugle « fidèle ou non »** : dans le cadre des tests d'ia-pnj §14.3, des testeurs regardent des relectures et disent si un acte était « dans le caractère » du personnage, dont ils ont lu le portrait ; on mesure l'aire sous la courbe ROC du coût calculé face à leur jugement (cible ≥ 0,85 **[à calibrer]**), et l'on recalibre l'échelle de réticence sur les degrés perçus.

### 4.7 Les données de jeu : un détecteur, pas une vérité (décidé)

Les actions des joueurs ne servent **jamais** à ajuster les paramètres. Elles alimentent trois indicateurs d'alerte, hors du cœur, dont les seuils sont dans le **paquet d'exploitation** (S29) :

- **Sous-évaluation** : un acte (ou un type d'effet) beaucoup plus fréquent chez les pilotes `Joueur` que chez l'IA de profils semblables, avec une faible perte de synchronisation, est soumis à relecture et à une campagne de comparaisons ciblée ;
- **Lacunes** : les combinaisons acte × circonstances jouées par les joueurs et jamais vues dans l'ensemble de référence entrent dans la file d'annotation ;
- **Mise en scène** : la fréquence des justifications neutralisées par l'imputabilité, par compte, alimente la détection d'abus (E3) ; une forte proportion désigne un joueur qui cherche les contextes.

*Pourquoi : les joueurs ne vivent pas comme les habitants ; ce qu'ils font souvent montre où le système est faible, pas ce qui est naturel.*

### 4.8 Invariants et scénarios `ts-sim` (décidé)

S'ajoutent au catalogue de la revue d'équilibrage (§5.3) :

| Invariant | Énoncé |
|---|---|
| `INV-SIG-RUPTURE` | Pour chaque plancher R1 à R4 et chaque acte du paquet qui le déclenche, le coût est > 1 dans toute la zone à contre-emploi (vérification exacte aux sommets) |
| `INV-SIG-MONOTONE` | Les échelles déclarées restent monotones pour tout profil ; ajouter un effet à un acte n'en baisse jamais le coût |
| `INV-SIG-SALAMI` | Découper un acte en N actes envers la même contrepartie, dans l'épisode, ne baisse pas le coût total de plus de ε |
| `INV-SIG-SAME-FN` | Pour toute intention d'un joueur synthétique, le coût calculé par l'IA en observation, celui lu par la garde et celui que `ts_ai` attribuerait à la même méthode sont identiques au bit près |
| `INV-SIG-PLAN` | La somme des coûts marginaux des étapes d'une méthode exécutée par un joueur égale le coût de la méthode pour l'IA |
| `INV-SIG-STAGED` | Aucune justification dont la circonstance a été causée par l'acteur dans l'épisode, ou par son cercle dans les 30 jours, ne réduit un coût d'un pilote `Joueur` |
| `INV-SIG-NO-CLIENT-COST` | Aucun coût, aucune gravité ni aucun paramètre de profil ne part vers le client ; seuls le niveau quantifié (0 à 4), l'indicateur de rupture et un code de raison y vont |
| `INV-SIG-AI-BOUND` | Hors désespoir, l'IA ne choisit aucune méthode dont le coût dépasse son seuil |

Scénarios ajoutés au répertoire `scenarios/exploits/` et aux scénarios de l'IA :

| Scénario | Ce qu'il vérifie |
|---|---|
| `SIG_legitime_defense_provoquee` | Insulter un violent jusqu'à ce qu'il frappe, puis le tuer « en se défendant » : J1 imputable, R1 s'applique |
| `SIG_ivresse_puis_violence` | Faire boire un doux, puis le faire frapper : le décalage d'état n'est pas appliqué pour le joueur |
| `SIG_ordre_du_complice` | Un complice incarne le patron et ordonne : J5 neutralisée par la garde |
| `SIG_faim_fabriquee` | Refuser les repas puis voler « par nécessité » : J4 imputable |
| `SIG_liasses_multiples` | Dons anonymes répétés sur 30 jours : R2 au cumul |
| `SIG_escalade` | Chaque marche de l'escalade, sur 12 profils et 3 contextes : monotonie, rupture au bon endroit |
| `SIG_temoin_partiel` | Un témoin qui n'a pas vu la provocation juge plus sévèrement qu'un témoin qui l'a vue |
| `SIG_sondage_profil` | Un agent adverse qui cherche à estimer le profil d'un personnage incarné par la réticence (§8, risque 5) : nombre de sondages nécessaires |

Les **agents adverses** (E40) reçoivent un objectif de plus : **minimiser la perte de synchronisation pour un résultat nuisible donné** (une valeur transférée au cercle, un tort causé). Toute stratégie qui y parvient mieux que le jeu honnête est réduite et ajoutée au catalogue.

### 4.9 Ajouter un acte : le flux de travail (décidé)

1. L'auteur (une session de domaine, ou Claude pour le concepteur) écrit l'acte en RON, avec ses effets **mécaniques**, comme aujourd'hui. Il n'écrit **pas** de signature.
2. `ts-data check` dérive les effets, vérifie la cohérence et affiche le coût de l'acte pour les 12 archétypes, avec ses raisons.
3. Si un sens de l'acte manque (norme, symbole, geste excentrique), l'auteur ajoute une **annotation** d'une ligne ; un brouillon peut venir du modèle de langage (§4.4).
4. Si l'acte reste mal placé après annotation, et seulement alors, une **dérogation bornée** avec sa justification.
5. Le rapport de différences est relu ; pour un acte important ou nouveau dans sa famille, quelques vignettes entrent dans la file de comparaisons.
6. Les invariants et les scénarios tournent dans l'intégration continue.

Pour ≈ 85 % des actes, les étapes 3 à 5 sont vides.

### 4.10 Ordre de mise en œuvre (décidé)

| Phase ([05-architecture.md](../05-architecture.md), §5) | Ajout |
|---|---|
| 0. Fondations | Virgule fixe et tri canonique dans `ts_kernel` ; emplacement de `ts_conduct` dans l'espace de travail et règle de dépendance |
| 1. Prototype du cœur | `ts_conduct` complet ; grammaire avec les **valeurs de départ** du concepteur ; dériveurs du combat à mains nues, de l'économie minimale et des actes sociaux ; ancres ; `ts-data check` (couverture, échelles, planchers) ; `INV-SIG-RUPTURE`, `INV-SIG-MONOTONE`, `INV-SIG-SALAMI` |
| 2. Échange de pilote | Masque d'imputabilité dans la garde ; `INV-SIG-SAME-FN`, `INV-SIG-PLAN`, `INV-SIG-STAGED` ; scénarios `SIG_*` ; objectif des agents adverses |
| 3. Client solo | Réticence quantifiée et codes de raison ; gabarits d'explication ; bilan « ce qui lui a coûté » ; `INV-SIG-NO-CLIENT-COST` |
| 4. En ligne | Indicateurs du §4.7 et seuils dans le paquet d'exploitation |
| 5. Extension | Campagne de comparaisons et calibrage ; brouillons par modèle de langage ; test à l'aveugle « fidèle ou non » avant l'ouverture publique |

---

## 5. Explicabilité

### 5.1 Le verdict porte ses raisons (décidé)

`appraise` rend, avec le coût, les **deux contributions principales** sous forme codée : (type d'effet, axe, sens, classe de cible, justification appliquée ou neutralisée). Elles coûtent quatre octets chacune. Aucune explication n'est calculée à part : c'est la décomposition du calcul lui-même, donc elle ne peut pas mentir.

### 5.2 Les mots (décidé)

Des **gabarits** par (effet, axe, sens), avec des variantes par classe de cible et par degré de précision, mettent les raisons en mots :

| Raison codée | Précision « claire » | Précision « précise » |
|---|---|---|
| `Deception`, honnêteté, intime | « Ça ne vous ressemble pas de mentir. » | « Elle ne ment pas à ses proches. » |
| `HarmPhysical`, violence, vulnérable | « Il n'est pas du genre à frapper. » | « Frapper un homme à terre ? Jamais. » |
| `GiftOut`, bienveillance, anonyme | « Cet argent compte pour lui. » | « Il ne laisserait pas ses économies au premier venu. » |
| J2 appliquée | — | « Vous protégiez votre fils. » |
| J1 neutralisée par l'imputabilité | — | « C'est vous qui l'avez cherché. » |

### 5.3 Où le joueur les voit (décidé)

- **Réticence** : le signe garde ses degrés (interface §2.3), désormais **quatre niveaux** quantifiés (aucun, léger, net, fort) plus le signe de **rupture** ; la phrase de raison s'affiche au survol à partir de la précision « claire » (interface §4.1).
- **Intuition immédiate** après un acte qui fait baisser S : la raison principale (« Ça ne vous ressemble pas : vous ne mentez pas à vos proches. »).
- **Rupture** : « Il ne peut pas s'y résoudre. » suivi, à précision « précise », de la raison (« … frapper un homme à terre »).
- **Bilan** (interface §3.8) : deux lignes de plus, en mots, sans chiffres : **« Ce qui lui a coûté »** (au plus trois raisons, agrégées sur l'incarnation, pondérées par le coût) et **« Ce qui l'excusait »** (les justifications qui ont joué). *Pourquoi : comprendre pourquoi un acte était dissonant est la meilleure leçon pour jouer juste, et une justification reconnue apprend au joueur que le jeu tient compte du contexte.*

### 5.4 Ce que les explications ne révèlent pas (décidé)

- Aucun **nombre** : ni coût, ni gravité, ni position sur un axe. Le client reçoit un niveau (0 à 4), un indicateur de rupture et un code de raison, jamais le verdict complet (`INV-SIG-NO-CLIENT-COST`).
- Aucune **croyance d'autrui** : les raisons portent sur les croyances du personnage incarné, que le joueur partage pendant l'incarnation.
- Les raisons des **témoins** (blâme, écart perçu) ne sont jamais montrées au joueur ; elles se voient dans le monde (reproche, « ça ne te ressemble pas »).
- Les développeurs voient la **trace complète** (effets, circonstances, justifications, coût par axe) dans la vue de conception, compilée seulement en développement (05 §2.20).

### 5.5 L'IA s'en sert aussi (décidé)

Le souvenir dissonant garde ses raisons ; la **réparation** se choisit d'après le type d'effet dominant, sans table par acte :

| Effet dominant | Réparations proposées par l'IA (ia-pnj §5.3) |
|---|---|
| `Deception`, `BreachOfTrust` | Avouer, s'expliquer, s'excuser |
| `Appropriation`, `GiftOut` | Rendre, réclamer, faire opposition, déclarer perdu |
| `HarmPhysical`, `Force`, `Humiliation` | S'excuser, éviter la personne, se rendre à la police (si légaliste et honnête) |
| `CommitmentBroken`, `NormViolation` | S'excuser, rattraper l'obligation |
| `Facilitation` | Prévenir, dire à la police, changer la serrure |

---

## 6. Décisions

Chaque décision est marquée (décidé) avec sa raison en une phrase. Les valeurs sont **[à calibrer]** sauf mention contraire.

**G1 — Une signature dérivée des effets (décidé).** La signature d'un acte n'est plus écrite : elle est calculée à partir de ses effets élémentaires, pris dans un catalogue fermé de 22 types (§3.2).
*Raison : ce qu'un acte fait est ce qui le rend dissonant, et le décrire une fois par effet couvre tous les actes, présents et futurs.*

**G2 — Coût par tolérance et poids, par axe et par sens, en virgule fixe (décidé).** Formules du §3.4 et du §3.5 ; profils en millièmes, gravités en Q16, ordre canonique des effets.
*Raison : un acte n'exige pas un profil précis mais refuse les profils trop éloignés de lui, et le calcul entier rend la frontière du point de rupture identique partout.*

**G3 — Les effets sont lus dans les données mécaniques (décidé).** Dériveurs statiques à la compilation, descripteurs de domaine à l'exécution, sur les croyances de l'acteur (§4.2).
*Raison : les domaines écrivent déjà ces effets pour résoudre les actes ; les réécrire en signatures serait du travail en double qui finirait par diverger.*

**G4 — Classes de cible comme facteur de gravité (décidé).** Une table unique (§3.3) remplace les variantes « envers un proche », « envers un inconnu ».
*Raison : la relation à la victime change la gravité de la même façon pour tous les actes d'une famille.*

**G5 — Douze justifications générales avec plafond de proportionnalité (décidé).** Une seule par effet, la plus favorable (§3.6).
*Raison : une douzaine de règles couvrent toutes les combinaisons acte × contexte, et l'absence de cumul empêche d'effacer un acte en empilant les excuses.*

**G6 — Imputabilité (décidé).** Une justification ne vaut pas si l'acteur a causé sa circonstance dans l'épisode ; pour un pilote `Joueur`, la garde neutralise aussi celles causées par son cercle (30 jours) et les décalages d'état créés pendant l'incarnation (§3.7).
*Raison : des règles de contexte générales sont exactement ce qu'un joueur cherche à fabriquer.*

**G7 — Quatre planchers de rupture sur les effets, évalués en dernier sur le profil de base (décidé).** R1 violence grave, R2 don massif, R3 facilitation contre les siens, R4 mise en danger mortelle de soi (§3.8).
*Raison : la garantie de E1 devient une propriété du calcul, vérifiable exactement, au lieu de dépendre de chaque auteur.*

**G8 — Composition par agrégation des effets des étapes (décidé).** Les recettes, méthodes, plans GOAP et projets n'ajoutent aucun effet ; leur signature est calculée à la compilation (§3.9).
*Raison : c'est la seule façon que l'IA, qui juge des plans, et le joueur, qui fait des gestes, utilisent vraiment la même définition.*

**G9 — Épisode et coût marginal (décidé).** Cumul des magnitudes par (type d'effet, contrepartie), sur une heure ou sur 30 jours pour les valeurs ; bénéficiaires anonymes réunis ; chaque acte coûte sa contribution marginale (§3.9).
*Raison : un acte découpé en gestes coûte alors exactement autant que l'acte entier.*

**G10 — Avant l'acte pour décider, après coup pour se souvenir et pour le bilan (décidé).** La synchronisation n'est jamais redébitée après coup ; les souvenirs sont réévalués ; E4 passe par le bilan (§3.10).
*Raison : on ne punit pas en jeu ce que le personnage ne pouvait pas prévoir, mais il peut le regretter et le joueur en répondre au bilan.*

**G11 — Une fonction `appraise` dans une crate `ts_conduct` (décidé).** Bibliothèque pure sous les greffons ; ce nom plutôt que « signature » (§4.1).
*Raison : cinq domaines ont besoin du même calcul sans pouvoir dépendre les uns des autres, et « signature » désigne déjà les signatures cryptographiques des artefacts.*

**G12 — Le tempérament compte peu et ne rompt jamais (décidé).** Effets de penchant lus dans les étiquettes existantes, coût plafonné par acte et par heure, aucun plancher de rupture.
*Raison : un introverti qui bavarde surprend ses proches sans les choquer (social §5.4), et un écart de tempérament ne doit pas éjecter un joueur.*

**G13 — La révélation sert au profil du joueur, aux traits supposés et à l'enfance (décidé).** Un acte de gravité γ montre qu'on tolère au moins `T(γ)` ; mêmes paramètres pour E16, ia-pnj §13.5 et §8.5 (§3.11).
*Raison : ce que coûte un acte et ce qu'il révèle de celui qui le fait sont la même information.*

**G14 — « Profil normatif » et « profil de valeurs » remplacent « signature attendue » et « signature de valeurs » (décidé).** Les normes d'un groupe et le programme d'un candidat sont des points ou des zones de l'espace du profil ; un manquement est évalué par `appraise` avec le profil normatif.
*Raison : un seul mot pour une seule chose évite les contresens entre sessions.*

**G15 — L'humain n'écrit que l'exceptionnel (décidé).** Grammaire, justifications, planchers, ancres et gabarits ; annotations d'une ligne pour ≈ 10 à 15 % des actes ; dérogations bornées (± 0,15) et justifiées pour au plus 5 % (§3.12).
*Raison : l'effort humain va là où il est irremplaçable, le jugement des cas qui échappent à la règle.*

**G16 — Contrôle de cohérence bloquant dans `ts-data check` (décidé).** Couverture, échelles, planchers, faisabilité des rôles, moyens par catégorie, doublons, dérogations, voisinage, ensemble de référence (§4.3).
*Raison : une incohérence de signature change le comportement de toute une catégorie d'habitants, elle doit être arrêtée avant de compiler.*

**G17 — Rapport de différences à chaque changement (décidé).** `ts-data signatures` liste les actes dont le coût change pour un archétype.
*Raison : modifier la grammaire est modifier une règle, et doit se relire comme telle.*

**G18 — Le modèle de langage écrit des brouillons, jamais des nombres (décidé).** Annotations proposées et recoupées, vignettes, gabarits, tri des comparaisons ; son jugement n'entre pas dans l'ajustement (§4.4).
*Raison : il écrit bien mais n'est ni calibré ni stable, et ia-pnj §13.14 ne lui permet que d'écrire des contenus relus.*

**G19 — Calibrage par comparaisons humaines, avec ancres et contraintes dures (décidé).** Bradley-Terry et logistique ordinale sur la fonction Rust elle-même, échantillonnage actif, monotonie par construction, planchers et ancres inviolables (§4.5).
*Raison : les humains comparent mieux qu'ils ne notent, et 200 paramètres se règlent sur quelques milliers de comparaisons.*

**G20 — La boucle comportementale ne touche pas aux signatures (décidé).** Les poids des décisions sont calibrés ensuite, signatures fixes ; un conflit remonte au concepteur (§4.6).
*Raison : ce que les humains ont jugé ne doit pas être déformé pour atteindre un indicateur de simulation.*

**G21 — Les données de jeu détectent, elles n'enseignent pas (décidé).** Sous-évaluation, lacunes et mise en scène, en alertes hors du cœur, seuils dans le paquet d'exploitation (§4.7).
*Raison : les joueurs ne se conduisent pas comme les personnages, et apprendre d'eux récompenserait les exploits.*

**G22 — Huit invariants et huit scénarios dans `ts-sim` (décidé).** `INV-SIG-*` et `SIG_*` (§4.8), plus un objectif des agents adverses.
*Raison : chaque garantie de cette revue doit être une assertion exécutée à chaque modification, comme les autres invariants d'équilibrage.*

**G23 — Le verdict porte deux raisons codées (décidé).** Décomposition du calcul, quatre octets par raison (§5.1).
*Raison : une explication tirée du calcul lui-même ne peut pas contredire le coût.*

**G24 — Réticence en quatre niveaux, aucun nombre vers le client (décidé).** Niveau, indicateur de rupture et code de raison seulement (§5.3, §5.4).
*Raison : tout ce qui part vers le client est public, et un coût exact permettrait de reconstituer un profil.*

**G25 — Le bilan dit « ce qui lui a coûté » et « ce qui l'excusait » (décidé).** En mots, au plus trois raisons (§5.3).
*Raison : savoir pourquoi on a perdu le fil apprend à jouer juste mieux que n'importe quel chiffre.*

**G26 — La réparation se choisit par l'effet dominant (décidé).** Table du §5.5, sans réparation écrite par acte.
*Raison : on répare un mensonge par un aveu et un vol par une restitution, quel que soit l'acte précis.*

**G27 — Atelier d'annotation hors ligne, séparé de la console (décidé).** Une page statique générée par `ts_gym`, qui ne contient que des vignettes et exporte les réponses en fichiers versionnés.
*Raison : l'annotation n'a besoin d'aucune donnée de monde, et la mettre dans la console d'administration ouvrirait une surface d'attaque pour rien.*

**G28 — Le témoin juge sur ses propres croyances (décidé).** Même fonction, croyances et valeurs du témoin (§3.10).
*Raison : c'est ce qui rend les jugements des témoins différents et réalistes sans signature « vue de l'extérieur ».*

**G29 — Paramètres dans le paquet de contenus, surchargeables par scénario (décidé).** Ils font partie des règles publiques ; `ts-sim` peut les balayer pour des analyses de sensibilité.
*Raison : c'est le statut de tous les paramètres de jeu (revue d'architecture §5.4), et le balayage mesure ce que chaque paramètre change dans la ville.*

**G30 — Le classement du texte libre ne fournit que des énumérations fermées (décidé).** Acte, sujet, ton et registre, confirmés par le joueur ; aucune valeur libre n'entre dans les effets (S18).
*Raison : un texte de joueur ne doit pas pouvoir régler la gravité de son propre acte.*

---

## 7. Coûts

### 7.1 En jeu **[à mesurer]**

| Calcul | Opérations | Remarque |
|---|---|---|
| Remplir un gabarit (descripteur) | 5 à 15 par effet | Montant, cible, blessure attendue |
| Justifications | ≈ 3 par effet | Indexées par type d'effet ; masque de circonstances calculé une fois par tick |
| Agrégation par axe et coût | ≈ 25 | Au plus 9 axes non nuls, souvent 2 ou 3 |
| Planchers, épisode | ≈ 10 | |
| **Un acte évalué** | **≈ 40 à 80** | |
| Liste d'actions d'un personnage incarné | ≈ 30 actions → ≈ 1 500 à 2 500 par rafraîchissement | Seulement quand la liste change ; un personnage par joueur |
| Choix des moyens par l'IA | ≈ 30 par recette, avec cache | Cache par (gabarit, masque de circonstances), invalidé quand le profil effectif change |
| Statistique | ≈ +40 par personnage et par jour par rapport à une lecture de signature écrite | Une vingtaine d'évaluations par jour, surtout en cache |

Le surcoût en statistique (≈ 40 opérations par jour) relève du budget de la décision (ia-pnj §3, §6.4), pas des 500 opérations du §13 ; il reste petit devant eux. **Mémoire** : gabarits ≈ 50 Ko dans le paquet (≈ 800 actes × 64 octets) ; par personnage, l'épisode (≈ 100 octets) et le cache (≈ 130 octets), soit ≈ 2,5 Mo pour 10 000 habitants ; deux raisons de 4 octets par souvenir dissonant.

### 7.2 Hors ligne

| Poste | Ordre de grandeur |
|---|---|
| Écrire `ts_conduct`, les dériveurs et les contrôles | ≈ 2 à 3 semaines de développement, réparties sur les phases 0 à 3 |
| Valeurs de départ et ancres du concepteur | ≈ 3 à 4 heures de relecture de vignettes |
| Campagne initiale de comparaisons | ≈ 25 à 40 heures d'annotateurs au total (5 000 comparaisons à 20 s, 600 jugements, répétitions) |
| Ajustement du modèle | Quelques minutes de calcul par campagne |
| Contrôles `ts-data check` | Quelques secondes par compilation (grille de 3⁹ profils sur ≈ 800 actes) |
| Brouillons par modèle de langage | Quelques euros par campagne |
| Ajout d'un acte | 0 minute pour ≈ 85 % des actes ; 5 à 15 minutes pour une annotation ; relecture du rapport ≈ 2 minutes |

### 7.3 Comparé au statu quo

| | À la main | Système retenu |
|---|---|---|
| Nombres écrits par des humains | ≈ 20 000 (800 actes × ≈ 3 contextes × 9 axes) | ≈ 220 paramètres calibrés, ≈ 100 annotations d'une ligne, ≤ 40 dérogations |
| Garantie de rupture | Par relecture | Par construction, vérifiée exactement |
| Cohérence entre domaines | Non mesurée | Une échelle, mesurée contre les jugements humains |
| Coût d'un contexte nouveau | Une variante par acte concerné | Une règle |

---

## 8. Risques et mesure

| N° | Risque | Comment il se manifeste | Mesure | Parade |
|---|---|---|---|---|
| 1 | **Sens manquant** : un acte dont la gravité n'est pas dans ses effets (symbole, norme implicite) | Coût trop faible, joueurs qui l'exploitent, testeurs qui le jugent « pas dans le caractère » | Test à l'aveugle « fidèle ou non » (AUC) ; détecteur de sous-évaluation (§4.7) ; voisinage (§4.3) | Annotation, au besoin nouvel effet du catalogue |
| 2 | **Lissage** : tous les actes se ressemblent, la personnalité ne se lit plus | Indicateur « lisibilité du caractère » qui baisse ; IA de profils différents qui agissent pareil | Kendall ≥ 0,8 contre les humains ; corrélations profil × effets (ia-pnj §14.1) ; entropie par rôle | Recalibrer les poids `W`, enrichir les projections |
| 3 | **Mise en scène des contextes** | Joueurs qui provoquent, affament, enivrent, font donner l'ordre | `INV-SIG-STAGED`, scénarios `SIG_*`, agents adverses, taux de justifications neutralisées par compte | Imputabilité (G6) |
| 4 | **Découpage et cumul** | Valeur sortie par petits gestes, coups répétés | `INV-SIG-SALAMI`, `SIG_liasses_multiples` | Épisode (G9), seuils de l'économie inchangés |
| 5 | **Sondage du profil** : la grammaire est publique, la réticence aussi ; un joueur estime le profil d'un habitant en sondant des actes calibrés | Wikis de profils des habitants d'un serveur | `SIG_sondage_profil` : nombre de sondages pour estimer un axe à ± 0,2 ; cible ≥ 30 sondages dans des situations différentes **[à calibrer]** | Quatre niveaux seulement, fiabilité liée à la précision (interface §2.3) ; population en ligne privée et couche cachée par monde (S2, S3) ; aucun nombre vers le client |
| 6 | **Dérive du calibrage** : une nouvelle campagne change toute la ville | Comportements qui changent entre deux versions | Rapport de différences (G17), indicateurs de `ts-sim` avant et après, ensemble de référence | Paramètres versionnés, relus, déployés avec le paquet ; un monde garde l'empreinte de ses règles |
| 7 | **Biais des annotateurs** | Une sensibilité unique imposée à tous les profils | Accord et cohérence par annotateur ; écart entre sous-groupes du panel | Panel divers, ancres du concepteur, arbitrage |
| 8 | **Erreur silencieuse du modèle de langage** | Une annotation plausible mais fausse | Taux de rejet par recoupement ; part des annotations corrigées en relecture | Recoupement automatique, relecture obligatoire (G18) |
| 9 | **Divergence entre l'IA et le joueur** | Un même geste qui coûte différemment selon le chemin | `INV-SIG-SAME-FN`, `INV-SIG-PLAN` | Une seule fonction, un seul composant d'épisode |
| 10 | **Garantie de rupture cassée** par un recalibrage ou une dérogation | Un acte extrême redevenu possible | `INV-SIG-RUPTURE` exact ; contrainte dure du calibrage | Planchers hors de portée des dérogations (G15) |
| 11 | **Coût de calcul** plus élevé que prévu | Budget de la décision dépassé en statistique | Compteurs d'opérations de `ts-sim` par niveau | Cache par gabarit et masque ; descripteurs simplifiés en statistique |
| 12 | **Explications incomprises ou répétitives** | Joueurs qui ne comprennent pas la réticence | Tests d'interface ; part des actes à réticence forte joués malgré tout | Gabarits révisés, variantes |
| 13 | **Tempérament surpondéré** | Éjections pour des écarts de goût | Part de la perte de synchronisation due au tempérament (cible < 15 % **[à calibrer]**) | Plafonds (G12) |
| 14 | **Annotations qui deviennent des signatures écrites** | La part annotée ou dérogée grimpe | Part des actes annotés (> 15 %) ou dérogés (> 5 %) dans le rapport de la CI | Revoir les dériveurs ou ajouter un effet au catalogue |

---

## 9. Modifications proposées au socle et aux domaines

La session principale décide de ce qu'elle retient ; les domaines appliquent eux-mêmes leurs modifications. Les revues précédentes ne sont pas modifiées.

### 9.1 [CLAUDE.md](../../../CLAUDE.md), section « Architecture »

| Ajout |
|---|
| Une puce : « **Signatures comportementales dérivées des effets** (`ts_conduct`, fonction unique `appraise`) : effets élémentaires lus dans les données des actes, justifications générales avec imputabilité, planchers de rupture ; seul l'exceptionnel est écrit à la main ; paramètres calibrés hors ligne par comparaisons humaines (docs/conception/revues/signatures.md). » |

### 9.2 [README.md](../README.md)

| Section | Modification |
|---|---|
| Déroulement, point 5 | Ajouter la **revue des signatures** ([revues/signatures.md](signatures.md)) aux revues relues puis intégrées par la session principale |
| Relectures de cohérence | Ligne à ajouter par la session principale après intégration |

### 9.3 [03-vie-synchronisation.md](../03-vie-synchronisation.md)

| Section | Modification |
|---|---|
| §1, paragraphe « Point de rupture » | Remplacer la justification implicite « le barème des signatures » par : « Quatre **planchers de rupture**, posés sur les effets des actes et évalués sur le profil de base du personnage, garantissent que ces actes coûtent plus que 1 ; une justification que le joueur a lui-même provoquée ne les lève pas ([revues/signatures.md](revues/signatures.md), G7, G6). » |
| §1, tableau, ligne « Le joueur agit contre la nature du personnage » | Ajouter : « le coût est celui de la fonction unique `appraise` (revues/signatures.md) ; un acte découpé en gestes coûte autant que l'acte entier (G9). » |

### 9.4 [04-joueur.md](../04-joueur.md)

| Section | Modification |
|---|---|
| §2, puce « La signature d'une action dépend de son contexte » | Remplacer par : « La **signature** d'une action est **dérivée de ses effets** (tort, tromperie, illégalité, transfert, contrainte, risque, aide, rupture d'engagement…), lus sur ce que le personnage **croit** ; le contexte agit par des **justifications générales** (légitime défense, protection d'autrui, provocation, nécessité, ordre, devoir de fonction, urgence, consentement, contrainte subie), qui ne valent pas si l'acteur, ou le cercle du joueur, a créé la situation. Tuer en légitime défense n'est pas tuer de sang-froid, et provoquer pour se « défendre » ne l'est pas non plus ([revues/signatures.md](revues/signatures.md), G1, G5, G6). » |
| §2, puce « Une seule définition de la signature comportementale » | Compléter : « … calculée par une seule fonction, qui sert aussi aux témoins, aux souvenirs, au profil du joueur et aux traits supposés ; un acte composé a pour signature l'agrégat de ses étapes (G8, G11, G13). » |
| §2, puce sur le profil du joueur (E16) | Préciser : « le poids d'une action est sa **gravité** au sens des signatures, et elle déplace le profil vers ce qu'elle révèle (G13). » |
| §2, puce « Le profil des enfants évolue » | Ajouter « … dans la direction des effets des souvenirs marquants (G13). » |
| §3, puce « Chaque action trop éloignée… » | Ajouter : « Le tempérament compte peu et ne mène jamais seul à l'éjection (G12). » |
| §7, ligne « Gestes trahissant un joueur » | Remplacer « a une signature » par « porte un effet d'emportement (revues/signatures.md, §3.2) » |

### 9.5 [05-architecture.md](../05-architecture.md)

| Section | Modification |
|---|---|
| En-tête | Ajouter : « Complété par la [revue des signatures](revues/signatures.md) (G1 à G30). » |
| §2.2 | Ajouter : « Une bibliothèque **`ts_conduct`**, au niveau de `ts_data`, porte la fonction unique `appraise` des signatures et de leur coût ; les greffons l'utilisent sans dépendre les uns des autres ; chaque domaine fournit un **descripteur** de ses actes, branché par `ts_game`. Les signatures sont des **gabarits** compilés dans le paquet de contenus à partir des effets mécaniques des actes (revues/signatures.md, G3, G11). » |
| §2.3, « Garde hors fiction » | Ajouter « l'**imputabilité** (masque des justifications causées par le cercle et des états créés pendant l'incarnation, G6) » à la liste de ce que la garde applique |
| §2.5 | Ajouter : « Le client ne reçoit, pour chaque action possible, qu'un **niveau de réticence** (0 à 4), un indicateur de rupture et un code de raison ; jamais un coût (G24). » |
| §2.13, « Décisions écrites à la main » | Ajouter : « Les **signatures** suivent le même principe : une grammaire d'effets écrite à la main, des paramètres calibrés hors ligne par **comparaisons humaines** (Bradley-Terry), avant les poids des décisions et sans que ceux-ci les modifient (G19, G20). » |
| §2.24 | Ajouter les invariants `INV-SIG-*` et l'objectif des agents adverses « minimiser la perte de synchronisation pour un résultat nuisible » (G22). |
| §5, phases 0 à 5 | Ajouter les éléments du §4.10 de la revue ; en phase 1, remplacer « signatures et coût estimé des actes » par « `ts_conduct`, grammaire des effets avec valeurs de départ, dériveurs du prototype, contrôles de cohérence et invariants de rupture, de monotonie et de découpage » |

### 9.6 [domaines/ia-pnj.md](../domaines/ia-pnj.md)

| Section | Modification |
|---|---|
| §0, principe 6 | Ajouter : « La signature est **dérivée des effets** de l'acte et calculée par la fonction unique `appraise` (revues/signatures.md). » |
| §1.4, « Règle de choix des moyens » | Remplacer « chaque recette a une signature » par « la signature d'une recette est l'agrégat des effets de ses étapes ; son coût pour un personnage suit le modèle de tolérance et de poids (revues/signatures.md, §3.5) » ; le seuil et les moyens désespérés sont inchangés ; la nécessité y devient la justification J4 |
| §3.3, recettes | Préciser que les recettes **n'écrivent pas** de signature : leurs étapes portent des effets mécaniques, la signature est compilée (G8) ; GOAP utilise le coût de l'agrégat comme coût de chemin |
| §3.7 | Ajouter que le coût estimé est le verdict d'`appraise` sur les croyances, avec le masque d'imputabilité pour un pilote `Joueur` |
| §4.5 | Ajouter : « le souvenir dissonant garde les deux raisons du verdict ; il est réévalué si le personnage apprend des conséquences réelles ; la réparation est choisie d'après l'effet dominant (revues/signatures.md, §5.5) » |
| §5.1 | Ajouter l'**épisode** (conduite récente du personnage) et le coût **marginal** (G9) |
| §5.3, « Éjection par le point de rupture » | Remplacer « Le barème des signatures garantit » par « Les planchers de rupture garantissent (G7) » |
| §5.6, « Signe de réticence » | Quatre niveaux quantifiés et une raison en mots (G24) ; aucune autre valeur vers le client |
| §8.5 | « … dans la direction des effets de ses souvenirs marquants (G13) » |
| §13.1, « Pour mes normes » | Préciser : le blâme est le verdict d'`appraise` sur les **croyances du témoin**, avec **ses** valeurs ; l'admiration vient des effets d'aide (G28) |
| §13.5, traits supposés | Préciser la **révélation** (G13) : un acte de gravité γ montre une tolérance au moins égale à `T(γ)` |
| §13.13 | Ajouter que les paramètres des signatures sont calibrés **avant** les poids des décisions, par comparaisons humaines, et restent fixes pendant la boucle comportementale (G19, G20) |
| §13.14, usage 1 | Ajouter aux contenus écrits hors ligne : annotations d'actes proposées et recoupées, vignettes d'annotation, gabarits d'explication ; jamais de valeurs numériques (G18) |
| §14.1 | Ajouter : corrélation profil × **effets** ; accord avec les jugements humains (Kendall) ; AUC du test « fidèle ou non » ; part de la perte de synchronisation due au tempérament |
| §14.2, §14.3 | Ajouter les scénarios `SIG_*` et le test à l'aveugle « fidèle ou non » (§4.6, §4.8) |

### 9.7 [domaines/social.md](../domaines/social.md)

| Section | Modification |
|---|---|
| §0, principe 5 | « … chacun avec sa signature, **dérivée de ses effets** (revues/signatures.md). » |
| §3.1 | Les actes déclarent leurs **effets mécaniques** (croyance transmise et sa vérité pour l'orateur, engagement créé, secret révélé, humiliation), d'où la signature est dérivée ; plus de signature écrite par acte |
| §3.3, « Synchronisation » | « Le ton compte : un ton menaçant ajoute un effet de **contrainte**, quel que soit l'acte (G1) ; le classement du texte libre ne fournit que des énumérations fermées (G30). » |
| §5.4 | L'écart perçu est le verdict d'`appraise` sur les **croyances du témoin** et les traits qu'il prête à l'acteur (G28) ; un témoin qui n'a pas vu la provocation juge sans elle |
| §6.1, « Normes » | Remplacer « signature attendue » par « **profil normatif** » (G14) ; un manquement est évalué avec ce profil |
| §6.7, ordres | Le coût de l'ordre pour le profil du subordonné est le verdict d'`appraise` avec la justification J5 ; l'ordre est lui-même un acte d'**instigation** pour celui qui le donne |
| §6.11, vote | Remplacer « résumé par une signature de valeurs » par « résumé par un **profil de valeurs** » (G14) |

### 9.8 [domaines/combat.md](../domaines/combat.md)

| Section | Modification |
|---|---|
| §1.7, puce sur la signature | « La signature d'une intention est dérivée de la **blessure attendue** (modèle du §2), de la contrainte et de l'infraction qualifiée ; le contexte agit par les justifications générales (revues/signatures.md, §3.6). » |
| §8.1 | Les infractions ne portent plus de signature écrite : le catalogue fournit la **qualification** et la peine, qui donnent l'effet de violation de la loi ; le reste de la signature vient des effets de l'acte |
| §10.1, ligne « Profil » | « Le coût de la recette est celui de l'agrégat de ses étapes (G8). » |
| §11.1 | Riposte, protection, acharnement et poursuite d'un agresseur qui fuit découlent des justifications J1 et J2 avec **plafond de proportionnalité** et de la classe de cible « vulnérable » ; aucune variante écrite |
| §11.2, « Escalade de signatures » | L'escalade est une **échelle déclarée**, monotone par contrôle automatique ; la rupture relève du plancher R1 ; ajouter : « provoquer pour pouvoir se défendre ne justifie rien (imputabilité, G6) » |
| §11.3 | Le critère de mise en danger mortelle d'un personnage prudent devient le plancher **R4** |

### 9.9 [domaines/economie.md](../domaines/economie.md)

| Section | Modification |
|---|---|
| §12.1 | « La signature d'une action économique est dérivée de son **effet sur le grand livre** : valeur nette sortie sans contrepartie au prix de référence (don), prise, profit sur une cible vulnérable ; le don au bénéficiaire probable (E5) n'a plus de liste d'actes : il est reconnu par son effet, et les bénéficiaires anonymes sont réunis dans un même cumul de 30 jours (revues/signatures.md, G9). » |
| §12.2, promesse d'argent | Préciser : une promesse est un effet de sortie de valeur au moment où elle est faite (inchangé), dans le même cumul |
| §14.3, recettes économiques | Les recettes déclarent leurs écritures attendues ; leur signature est compilée |

### 9.10 [domaines/interface.md](../domaines/interface.md)

| Section | Modification |
|---|---|
| §2.3, « Réticence avant d'agir » | Quatre niveaux (aucun, léger, net, fort) plus le signe de rupture ; une phrase de raison au survol à partir de la précision « claire » (revues/signatures.md, §5.3) |
| §2.3, « Point de rupture » | À précision « précise », ajouter la raison après « Il ne peut pas s'y résoudre. » |
| §3.8, bilan | Ajouter deux rubriques en mots : « Ce qui lui a coûté » (au plus trois raisons) et « Ce qui l'excusait » (G25) |
| §4.1 | Les gabarits d'explication suivent les degrés de précision des intuitions |

### 9.11 [domaines/population.md](../domaines/population.md)

| Section | Modification |
|---|---|
| §4.1, puce « Biographie » (« déplacent légèrement le profil dans la direction de leur signature ») | « … dans la direction des effets de ces événements (G13). » |
| §4.6 (couverture des catégories) | Ajouter que `ts-data check` vérifie, pour chaque catégorie, au moins deux moyens fidèles par besoin important, et, pour chaque rôle, que ses recettes de cœur de métier sont fidèles aux profils que le générateur tire pour lui (§4.3) |

### 9.12 [domaines/ville.md](../domaines/ville.md)

Aucune modification nécessaire. Les accès protégés (serrures, alarmes, codes) et leurs responsables, que la facilitation utilise, sont déjà des données du serveur (S33) et de l'économie (pouvoirs d'un poste).

### 9.13 Revues précédentes

Non modifiées. Les invariants `INV-SIG-*` complètent ceux de la [revue d'équilibrage](equilibrage.md), §5.3, et l'indicateur de sous-évaluation rejoint ses indicateurs de production (§5.7) ; la règle « aucun coût vers le client » met en œuvre le renvoi de l'équilibrage à la sécurité (« vérifier qu'aucune donnée de coût estimé des actes ne part vers le client »).
