# Le joueur : niveau, profil et accès aux personnages

> Fichier du **socle** : modifié uniquement par la session principale (voir [README](README.md)).

## 1. Niveau de compte

- Chaque joueur a un **niveau associé à son compte**, **distinct en solo et en ligne**.
- Le joueur peut **réinitialiser** son compte quand il le veut : il **perd tout**, y compris son niveau, et revient au point de départ de toutes ses caractéristiques. Dans le monde, son personnage persistant **déménage définitivement** et ses biens sont vendus. Un personnage persistant abandonné 12 mois réels est traité de même pour le monde, sans perte de niveau pour le joueur ([domaines/economie.md](domaines/economie.md), §11.6).
  - La réinitialisation, comme le changement d'e-mail et la suppression du compte, demande une **réauthentification** et ne prend effet qu'après un **délai d'annulation de 72 h** ; en ligne, elle est possible **une fois tous les 90 jours réels** (décidé, [revues/securite.md](revues/securite.md), S13 ; [revues/equilibrage.md](revues/equilibrage.md), E23).
  - La **note de conduite**, les sanctions et l'historique des abus **survivent à la réinitialisation**, qui ne remet à zéro que ce qui est dans le jeu (E23). *Pourquoi : la réinitialisation est un nouveau départ dans le monde, pas l'effacement d'un comportement de joueur.*
- **Période probatoire** (en ligne, décidé, E24) : pendant les 30 premiers jours réels d'un compte, après chaque réinitialisation, et au moins jusqu'au niveau 10, les flux reçus de personnages pilotés par d'autres joueurs sont plafonnés à 2 000 écus cumulés, et les seuils des détections d'abus sont divisés par deux. *Pourquoi : un compte neuf est le support idéal d'un complice ou d'un saboteur, et un vrai débutant n'a besoin d'aucune de ces possibilités.*
- Une **condamnation** n'enlève pas d'expérience en elle-même ; les crimes du personnage persistant modifient le profil du joueur comme ses autres actes ([domaines/combat.md](domaines/combat.md), §9.5 et §11.4).
- **L'argent n'est jamais une source d'expérience** ([domaines/economie.md](domaines/economie.md), §0).
- Le niveau progresse grâce à l'**expérience** (§1.1).

### 1.1 Montée en expérience (décidé)

**Principe** : on gagne de l'expérience en **jouant bien un personnage**, c'est-à-dire en restant fidèle à sa nature et en accomplissant ce qu'il cherche à accomplir. On n'en gagne pas en accumulant du temps ou des actions répétitives.

**Sources d'expérience**

| Source | Gain |
|---|---|
| **Temps d'incarnation actif** | 1 XP par minute **active** : une minute est active si le joueur a pris une **décision** dans les 5 minutes précédentes (nouvelle destination, acte social, action du menu, tâche, réaction à un événement ; continuer une activité ou marcher tout droit n'en sont pas). Une activité passive longue (écran, lecture, attente, trajet en transport) compte au plus 10 minutes. Une incarnation de **moins de 10 minutes** ne rapporte ni temps actif, ni bonus, ni prime de familiarité ([revues/equilibrage.md](revues/equilibrage.md), E12, E14) |
| **Objectifs du personnage** | Bonus quand le joueur fait avancer les objectifs propres du personnage, ceux que l'IA poursuivrait (finir sa journée de travail, conclure une affaire, aider un proche…) : de 10 à 200 XP selon l'importance (barème ci-dessous) |
| **Nouvelle familiarité** | 50 XP la première fois qu'une familiarité atteint le seuil permettant d'incarner un nouveau personnage, pour une cible de **palier 1 ou plus**, une fois par cible (E15) |
| **Personnage persistant** | Mêmes sources, à 50 % : il n'a pas de synchronisation, donc pas de mérite à « jouer un rôle » |

**Barème des objectifs du personnage** (validé ; valeurs [à calibrer]) :

| Type d'objectif | Exemples | XP |
|---|---|---|
| Besoin important ou geste de routine | Manger, dormir à une heure raisonnable, faire ses courses | 10 |
| Obligation du jour | Être à l'heure, finir sa journée ou sa mission, aller chercher les enfants | 20 à 50 |
| Tâche de travail | Servir un client, terminer une réparation, livrer un colis | 1 à 5, au plus 30 par heure réelle |
| Étape de projet | Décrocher un entretien, rembourser une partie d'une dette, renouer le contact | 50 à 100 |
| Aboutissement de projet, événement majeur | Obtenir une promotion, se réconcilier, conclure une grosse affaire | 100 à 200 |

- Les objectifs sont **générés par l'IA du personnage**, jamais choisis par le joueur ([domaines/ia-pnj.md](domaines/ia-pnj.md), §5.4).
- Un objectif atteint **par un moyen contraire à la nature du personnage** ne rapporte aucun bonus.
- Comme une journée de travail dure 8 heures réelles, le travail est découpé en **tâches** (servir un client, terminer une réparation, livrer un colis), qui rapportent chacune **1 à 5 XP**, dans la limite de **30 XP par heure réelle**, avec des rendements décroissants au-delà de 4 heures de travail dans la journée ; une tâche créée par un personnage du cercle du joueur (§8) ne compte pas (décidé, E11). *Pourquoi : une journée de travail bien faite doit rester au rythme de la courbe (≈ 100 XP par heure).*
- **Objectifs liés** (décidé, E13) : aucun bonus si la situation résolue ou l'événement qui accomplit l'objectif a été **causé par un personnage du cercle** du joueur dans les 30 jours (sauf le personnage incarné lui-même, dans la même incarnation, qui mène son propre travail ou projet) ; une promesse ou un rendez-vous que le joueur a lui-même fait prendre au personnage ne rapporte que le barème de routine ; les étapes et aboutissements de projet sont **proportionnels à la part du chemin** faite pendant les incarnations de ce joueur (au moins 20 %) ; un objectif accompli avec un personnage piloté par un autre joueur rapporte moitié, au plus un aboutissement par paire de comptes et par 30 jours. *Pourquoi : un objectif doit venir de la vie du personnage, pas d'une mise en scène du joueur ou de ses complices.*
- Les objectifs de routine ont un **plafond quotidien** ; provoquer un objectif artificiellement (affamer le personnage pour le faire manger) ne rapporte pas plus que ce plafond.

**Affichage** : le bilan donne la fidélité en mots, sans montrer le multiplicateur de palier ni la valeur de chaque objectif ; aucun compteur d'expérience n'est affiché pendant l'incarnation ([domaines/interface.md](domaines/interface.md), §3.8).

**Bilan d'une incarnation**, calculé quand le joueur quitte le personnage, en est éjecté ou quand le personnage meurt :

```
XP = (temps actif + bonus d'objectifs) × S_moyenne² × (1 + 0,15 × palier) − pénalités
```

- **S_moyenne²** : la synchronisation moyenne pendant l'incarnation, **pondérée par les décisions** du joueur plutôt que par le temps (une heure de lecture ne dilue pas un acte dissonant, E12), au carré. Une synchronisation de 0,9 garde 81 % du gain, une de 0,5 n'en garde que 25 %. Jouer à contre-emploi ne rapporte presque rien.
- **(1 + 0,15 × palier)** : un personnage de palier 0 rapporte ×1, un personnage de palier 11 rapporte ×2,65. Les personnages puissants sont plus exigeants, donc plus gratifiants.

**Pénalités**

| Événement | Pénalité |
|---|---|
| Éjection pour désynchronisation, point de rupture, ou personnage quitté sous le seuil d'alerte | Perte de toute l'expérience de l'incarnation, plus 10 % de l'expérience requise pour le niveau actuel (E1, E2) |
| Désynchronisation volontaire grave | En plus, 25 % de l'expérience du niveau, doublée à chaque récidive dans les 30 jours réels ; trois en 30 jours, ou trois refus par le point de rupture dans la journée, alimentent la note de conduite (E3) |
| Complicité par facilitation | Signature de complicité pondérée par le préjudice, appliquée au bilan provisoire (E4) |
| Mort du personnage incarné par imprudence ou à contre-emploi | Perte de l'expérience de l'incarnation ; deuxième dans les 30 jours réels : +10 % de l'expérience du niveau ; ensuite +25 % et signalement à la détection de l'abus du monde partagé ; double pendant la période probatoire (E28) |
| Mort du personnage incarné en restant fidèle à sa nature | Aucune pénalité : la mort fait partie de la vie du personnage |

L'expérience peut baisser, et **le niveau aussi**. Le niveau reste ainsi le reflet de la façon dont le joueur joue réellement.

**Garde-fous contre l'accumulation artificielle**

- Répéter la même activité rapporte de moins en moins (rendements décroissants par activité et par jour).
- En solo, **avancer le temps ne rapporte rien** : seul le temps joué compte.
- En ligne, l'expérience par heure diminue légèrement au-delà de quelques heures de jeu dans la journée **[à calibrer]**, pour limiter l'écart entre les joueurs qui jouent énormément et les autres.

**Courbe de niveau**

Expérience totale pour atteindre le niveau n : `XP_total(n) ≈ 21 × n^1,85`.

Temps de jeu correspondant, au rythme de base d'un personnage de palier 0 bien joué (≈ 100 XP par heure) :

| Niveau | Palier débloqué | Expérience totale | Temps au rythme de base |
|---|---|---|---|
| 3 | 1 — Ordinaire | ≈ 160 | ≈ 1 h 30 |
| 10 | 3 — Qualifié | ≈ 1 500 | ≈ 15 h |
| 21 | 5 — Respecté | ≈ 5 900 | ≈ 60 h |
| 36 | 7 — Influent | ≈ 15 900 | ≈ 160 h |
| 55 | 9 — Très puissant | ≈ 34 900 | ≈ 350 h |
| 80 | 11 — Sommet | ≈ 69 800 | ≈ 700 h |
| 100 | — | ≈ 105 000 | ≈ 1 050 h |

Les temps réels sont nettement plus courts, parce qu'un joueur qui progresse incarne des personnages de palier plus élevé, qui rapportent jusqu'à 2,65 fois plus, et accomplit leurs objectifs. En jouant à chaque fois le meilleur palier accessible, le niveau 80 demande plutôt de l'ordre de 300 à 400 heures **[à calibrer]**.

## 2. Profil comportemental (validé)

Joueurs et personnages sont décrits dans un même espace, ce qui permet de mesurer la distance entre eux.

**Le tempérament, sur le modèle des quatre dimensions MBTI**, en échelles **continues** :

| Dimension | Pôles |
|---|---|
| Orientation de l'énergie | Introversion (I) ↔ Extraversion (E) |
| Perception | Sensation (S) ↔ Intuition (N) |
| Décision | Pensée (T) ↔ Sentiment (F) |
| Organisation | Jugement (J) ↔ Perception (P) |

**Les valeurs et la conduite** :

| Axe | Pôles |
|---|---|
| Stabilité émotionnelle | Anxieux, impulsif ↔ calme, maître de soi |
| Bienveillance | Cruauté ↔ compassion |
| Respect des règles | Hors-la-loi ↔ légaliste |
| Honnêteté | Manipulation ↔ franchise |
| Violence | Pacifisme ↔ usage de la force |

- « Myers-Briggs » et « MBTI » sont des marques déposées : le jeu utilisera ses propres noms pour les dimensions et les types.
- Le **profil d'un personnage** découle de sa personnalité.
- Le **profil d'un joueur** se construit à partir de **ses actions** (moyenne glissante, les actions récentes pèsent plus). Un nouveau joueur commence avec un profil **neutre**. Le profil est **distinct en solo et en ligne** ; le poids d'une action est proportionnel à son **enjeu** (sa gravité au sens des signatures : montant rapporté au patrimoine, tort, coût pour le personnage), et elle déplace le profil vers ce qu'elle révèle (G13), avec des rendements décroissants pour les actions anodines répétées ; une catégorie ne se gagne ou ne se perd qu'après **10 heures** de jeu actif depuis le dernier changement (décidé, E16 : *le profil mesure une façon de jouer, pas une série de gestes faits pour déplacer une jauge*).
- Le joueur voit son propre profil sous la forme d'un **portrait en phrases**, accompagné des **noms de ses catégories** (décidé), sans aucune valeur. Pourquoi : les catégories sont le repère qui explique quels personnages lui sont accessibles, et elles ne révèlent rien des autres personnages.
- La **signature** d'une action est **dérivée de ses effets** (tort, tromperie, illégalité, prise ou sortie de valeur, contrainte, risque, aide, rupture d'engagement…), lus sur ce que le personnage **croit** ; le contexte agit par des **justifications générales** (légitime défense, protection d'autrui, provocation, nécessité, ordre, devoir de fonction, urgence, consentement, contrainte subie), avec un plafond de proportionnalité, et elles ne valent pas si l'acteur, ou le cercle du joueur, a créé la situation. Tuer en légitime défense n'est pas tuer de sang-froid, et provoquer pour se « défendre » ne l'est pas non plus (décidé, [revues/signatures.md](revues/signatures.md), G1, G5, G6). *Pourquoi : quelques dizaines d'effets et de justifications décrits une fois remplacent des milliers de valeurs écrites à la main, et sont cohérents par construction.*
- **Une seule définition de la signature comportementale** sert à deux usages : mesurer la synchronisation du joueur, et permettre à l'IA de choisir des moyens fidèles à la nature du personnage ([domaines/ia-pnj.md](domaines/ia-pnj.md), §1.4). Elle est calculée par **une seule fonction**, qui sert aussi aux témoins, aux souvenirs, au profil du joueur et aux traits supposés, et rend un verdict **explicable** (le coût et ses deux raisons principales, mises en mots) ; un acte composé a pour signature l'agrégat de ses étapes. L'humain n'écrit que l'exceptionnel (annotations d'une ligne pour 10 à 15 % des actes, dérogations bornées pour au plus 5 %, une quarantaine d'actes d'ancrage) ; les paramètres sont calibrés hors ligne par **comparaisons** d'un panel de sessions évaluatrices (A8, [05-architecture.md](05-architecture.md), en-tête) ([revues/signatures.md](revues/signatures.md), G8, G11, G13, G19). Les valeurs de départ, les 46 ancres, les 12 archétypes, les annotations (≈ 12 % des actes), les dérogations (≈ 1 %) et les gabarits sont écrits dans [revues/signatures-contenu.md](revues/signatures-contenu.md) (M1 à M18) ; ancres et archétypes sont tenus et arbitrés par le responsable conception, sans rien attendre de l'utilisateur (A8).
- Le profil des **enfants évolue** selon ce qu'ils vivent, puis se fixe vers 18 ans, âge où ils deviennent jouables ([domaines/ia-pnj.md](domaines/ia-pnj.md), §8.5).
- **Règles publiques** (décidé le 01/11, arbitrage A14 du [README](README.md), [#76](https://github.com/CreatorInstant548/time_switcher/issues/76)) : depuis que le dépôt est public, la fonction des signatures, ses paramètres calibrés, les planchers de rupture, les seuils d'accès et les invariants sont lisibles par tous. **Aucune règle de jeu ne repose sur le secret de sa formule.**
  - **Jouer contre la formule est voulu** : une signature est dérivée des effets réels de l'acte. Pour garder un profil, le joueur doit donc vraiment produire ces effets, c'est-à-dire jouer de cette façon, au moins 10 heures avant tout changement de catégorie (E16), avec des rendements décroissants pour les gestes anodins répétés. Rester juste sous un plancher de rupture ne fait que jouer un acte permis, puisque les planchers placent les actes extrêmes au-dessus de 1 par construction.
  - **Ce qui reste caché est l'état d'un monde, jamais une règle** : graines secrètes, population en ligne, couche cachée et part serveur tirées par monde, profil exact des habitants (S2, S3, P12 ; risque 5 de [revues/signatures.md](revues/signatures.md), §8).
  - **Pas de seuil de jeu tiré par monde ni réservé au serveur.** Les règles et leurs paramètres sont les mêmes en solo et en ligne, et livrés avec le jeu. *Pourquoi : le solo embarque le serveur ; ce qu'on apprend dans un monde doit valoir dans tous ; et un seuil caché ou tiré se devine en quelques semaines de sondage, sans rien protéger.*
  - **Seule exception, les détecteurs d'exploitation hors fiction** : détection de la triche et des robots, de l'acharnement, du rapprochement de comptes et de la mise en scène répétée. Ce ne sont pas des règles de jeu : leurs seuils peuvent vivre dans un paquet propre au serveur en ligne, hors du dépôt, par déploiement. C'est un ralentisseur : aucune protection ni aucune décision n'en dépend, et l'échelle des sanctions ([domaines/social.md](domaines/social.md), §9.4) reste publique.
  - **La chasse aux exploits suppose un adversaire qui connaît tout** : joueurs synthétiques et agents adverses reçoivent la formule et les paramètres exacts (attendu M38).

## 3. Catégories comportementales

Les **catégories** sont des zones de cet espace, avec un nom lisible combinant tempérament et conduite (par exemple « ESTP — Hors-la-loi »).

- Un joueur appartient à **toutes les catégories** dont son profil est assez proche.
- Un personnage jouable appartient de la même façon à une ou plusieurs catégories.
- La règle d'accès utilise directement la distance entre profils.
- Chaque action **trop éloignée** du profil du personnage incarné fait perdre des points de synchronisation, d'autant plus que l'écart et la gravité sont grands. Agir conformément au personnage en fait regagner progressivement. Le **tempérament** compte peu dans ce coût et ne mène jamais seul à l'éjection ; ce sont surtout les valeurs qui comptent ([revues/signatures.md](revues/signatures.md), G12).

## 4. Conditions pour prendre le contrôle d'un personnage jouable

1. Aucun autre joueur ne le contrôle.
2. Il n'est pas **hors catégorie** ([02-personnages.md](02-personnages.md), §3.4).
3. Le joueur s'est **familiarisé** avec lui (§6), sauf pour les personnages de départ.
4. Le **niveau de compte** du joueur est suffisant pour le **palier** du personnage.
5. Le **profil** du joueur n'est pas trop éloigné de celui du personnage (seuil **[à calibrer]**).
6. Le personnage est **au repos** (§5).
7. Il n'est pas en **repos de l'IA** (décidé, E17) : après une incarnation de durée D d'un personnage de **palier 5 ou plus**, aucun joueur ne le reprend pendant min(D/2, 4 h) réelles, le même cercle (§8) pendant min(D, 8 h), et un compte ne l'incarne pas plus de 8 h par jour réel. *Pourquoi : un personnage puissant ne doit jamais devenir la marionnette permanente d'un groupe, et l'IA a besoin de temps pour se rétracter ou réparer.*

## 5. Personnage « au repos »

Un joueur ne peut prendre le contrôle que d'un personnage engagé dans aucune action complexe ou intense, qui **ne perçoit aucune menace**, et qui est au repos **depuis quelques secondes** **[à calibrer]**, pour éviter les prises de contrôle entre deux actions (décidé ; [domaines/ia-pnj.md](domaines/ia-pnj.md), §7.1).

| Au repos (prise de contrôle possible) | Pas au repos (impossible) |
|---|---|
| Marcher tranquillement, attendre, s'asseoir | Combat, fuite, poursuite |
| Activité routinière (ranger, cuisiner, travail répétitif) | Conduite d'un véhicule |
| Se reposer, lire, regarder un écran | Conversation en cours, négociation |
| Dormir (voir ci-dessous) | Tâche délicate (chirurgie, réparation complexe, crochetage) |
| | Situation d'urgence, état de panique |

**Réservation courte** (validé) : si le personnage visé n'est pas au repos, la demande est mise en attente quelques minutes **[à calibrer]**. Dès qu'il est au repos (il se gare, finit sa conversation), le joueur en prend le contrôle. L'IA ne change pas son comportement pour se rendre disponible, et le joueur peut annuler ([domaines/ia-pnj.md](domaines/ia-pnj.md), §7.3). Une seule réservation active par compte ; après un refus ou une expiration, le même compte attend 10 minutes pour réserver le même personnage (E30).

**Prendre le contrôle d'un personnage qui dort** est possible : il se réveille. S'il n'a pas assez dormi, le joueur en **subit les effets**, selon les règles de la progression réaliste : dette de sommeil, réactions ralenties, perception et concentration dégradées, irritabilité, jusqu'à ce que le personnage récupère.

## 6. Départ et familiarité

**Les rôles ne sont pas visibles.** Le joueur ne voit ni le métier, ni le niveau, ni le palier, ni les catégories des personnages qu'il croise. Il les découvre comme dans la vie : en observant et en fréquentant les gens.

Cette règle concerne les personnages que le joueur **n'incarne pas**. Quand il incarne un personnage, il peut recevoir davantage d'indices sur sa vie, sous forme d'**intuitions** (« vous devriez être au travail », « votre frère vous manque »), dont la précision augmente avec la synchronisation et la familiarité ([domaines/ia-pnj.md](domaines/ia-pnj.md), §5.5). Les intuitions ne révèlent jamais un savoir opaque.

**Au départ**, le joueur choisit entre :

- son **personnage persistant**, qui arrive à Orvenne **par le train**, avec un petit pécule (≈ 1 200 écus sur un compte et 100 en espèces **[à calibrer]**) et trois nuits payées au foyer ([domaines/ville.md](domaines/ville.md), §2.5 ; [domaines/economie.md](domaines/economie.md), §11.1) ;
- une **sélection aléatoire d'adultes de palier 0**, jouables sans familiarité ; la population garantit qu'ils couvrent toutes les catégories comportementales ([domaines/population.md](domaines/population.md), §4.6). La sélection se renouvelle **5 fois par jour réel** au plus ; au-delà de 6 incarnations courtes dans la journée, chaque nouvelle prise de contrôle attend 10 minutes (E14 : *parcourir des centaines de vies en quelques secondes est un annuaire, que le socle refuse*).

**Pour prendre le contrôle de tout autre personnage**, le joueur doit s'être **familiarisé** avec lui, en le fréquentant avec son personnage persistant ou avec un autre personnage qu'il incarnait. On progresse ainsi de proche en proche dans le réseau social du monde : incarner A permet de connaître B, incarner B permet de connaître C…

**La familiarité est liée au personnage qu'incarne le joueur.** Si le joueur a commencé à se familiariser avec un personnage B en incarnant A, il doit continuer avec A : la progression obtenue avec A ne se cumule pas avec celle obtenue avec un autre personnage C.

Fonctionnement (validé) :

- La familiarité est une valeur propre à chaque triplet (compte du joueur, personnage incarné A, personnage visé B), **distincte en solo et en ligne**, et remise à zéro avec le compte.
- Le joueur peut incarner B dès que **l'une** de ses familiarités avec B atteint le seuil.
- Seules les interactions faites pendant que le joueur incarne A comptent. Quand l'IA pilote A, la relation entre A et B évolue dans le monde, mais la familiarité du joueur ne progresse pas.
- Si A meurt définitivement, la familiarité obtenue grâce à A est perdue. Le personnage persistant, qui ne meurt jamais définitivement, est donc le moyen le plus sûr de construire ses familiarités.
- Elle augmente avec les **interactions réelles** : conversations, travail en commun, activités partagées, temps passé ensemble, entraide. Les échanges superficiels (un salut, une question isolée) rapportent très peu, et répéter la même interaction rapporte de moins en moins.
- Elle **diminue lentement** sans contact.
- Le **seuil** de familiarité requis augmente avec le palier du personnage : on peut incarner un collègue après quelques jours de travail ensemble, mais il faut une vraie relation pour incarner un chef d'entreprise.
- La familiarité **révèle progressivement** des informations sur le personnage, en quatre étapes : une description, son nom, des traits de caractère, puis s'il serait jouable. Quand il ne l'est pas, le joueur apprend la **nature** de l'obstacle (expérience, connaissance, profil, hors catégorie pour une fonction publique ; un hors catégorie clandestin apparaît comme un obstacle d'expérience), jamais une quantité. L'indisponibilité (pas au repos, piloté par un autre joueur, état grave, repos de l'IA) **n'est plus affichée en continu** : le joueur ne l'apprend qu'en demandant à incarner, par une réservation qui expire sans motif après un délai variable ; un personnage à l'extérieur apparaît comme « Absent d'Orvenne ». Le personnage persistant d'un autre joueur ne se distingue donc pas d'un habitant souvent occupé ([domaines/interface.md](domaines/interface.md), §3.4 et §3.5 ; E26).
- Elle est calculée à partir des **événements d'interaction**, jamais à partir des valeurs de la relation entre les personnages ; la violence et la menace ne rapportent rien ; une interaction avec un personnage piloté par un autre joueur compte pour moitié. La table des gains est dans [domaines/social.md](domaines/social.md), §2.2.
- Pendant une incarnation, le joueur accède aux **connaissances sociales ordinaires** du personnage (noms, liens, métiers connus), qui ne lui restent pas après ; les secrets restent opaques ([domaines/social.md](domaines/social.md), §2.3).

## 7. Points délicats

| Sujet | Problème | Piste |
|---|---|---|
| **Espionnage par incarnation** | Un joueur incarne un personnage pour apprendre un secret, puis en profite avec un autre personnage | Voir §7.1 |
| **Reprise par l'IA** | Après un comportement inhabituel du joueur, l'IA garde-t-elle la personnalité d'origine ? | Réglé (validé) : la personnalité ne change pas. Le personnage se souvient de **tout** ce qu'il a fait pendant l'incarnation ; les actes contraires à sa nature sont des souvenirs **dissonants** (stress, regret ou justification). Les **engagements** pris par le joueur sont tenus **selon son caractère** ([domaines/ia-pnj.md](domaines/ia-pnj.md), §4.5 et §5.3). La personnalité ne change pas, mais les **attitudes**, les conclusions et les valeurs apprises évoluent avec la vie du personnage, y compris après une incarnation ([domaines/ia-pnj.md](domaines/ia-pnj.md), §13.4 et §13.6) |
| **Gestes trahissant un joueur** | Courir sans cesse, tourner sur soi, sauter, tout fouiller : des gestes qu'aucun habitant ne fait désignent un pilote humain | (décidé) Les commandes n'offrent **aucun geste gratuit** (pas de saut ni de geste libre hors des actions de contexte) ; courir sans raison apparente en public porte un effet d'emportement (légère dissonance pour un personnage calme, [revues/signatures.md](revues/signatures.md), §3.2) et fatigue ; fouiller est un acte avec signature et réaction des témoins. Les **actes excentriques** que la surface de jeu permet (suivre ou dévisager un inconnu, s'asseoir à la table d'inconnus, entrer par une porte ouverte, frapper aux portes, fouiller une poubelle, jeter un déchet) portent une annotation de signature ; ceux qui demanderaient un geste libre (danser seul, s'allonger sur la chaussée, uriner en public) **n'existent pas** en première version (décidé, arbitrage A7 du 29/10 sur [revues/signatures-contenu.md](revues/signatures-contenu.md), §5.2 ; *pourquoi : la règle « aucun geste gratuit » prime, et une annotation ne crée pas d'acte*). Les habitants ont de leur côté un **naturel** (hésitations, gestes au repos, regard, téléphone). *Pourquoi : le jeu promet qu'on ne distingue pas les pilotes, et la surface de jeu doit être la même des deux côtés* ([domaines/ia-pnj.md](domaines/ia-pnj.md), §13.7 et §14.3) |
| **Sabotage par désynchronisation** | Tuer un personnage important en agissant contre sa nature | Réglé : la désynchronisation ne touche que la barre de vie du joueur, et l'acte extrême pour le profil n'a pas lieu (point de rupture) ([03-vie-synchronisation.md](03-vie-synchronisation.md), §1) |
| **Accumulation de familiarité** | Saluer en boucle des dizaines de personnages pour pouvoir les incarner | Seules les interactions réelles comptent, avec des gains décroissants (§6) |
| **Enrichissement par l'incarnation** | Vider le compte d'un personnage incarné au profit d'un autre (persistant, ami) | Synchronisation pondérée par le montant, réparation par l'IA à la reprise, **flux interdits** entre personnages d'un même joueur, journalisation des flux entre joueurs ([domaines/economie.md](domaines/economie.md), §12) ; toute sortie de valeur sans contrepartie (abandonner, cacher, perdre au jeu, surpayer) est un **don** ; perdre volontairement au jeu se mesure par l'écart d'espérance entre la décision du joueur et celle de l'IA en observation, compté comme un don aux gagnants ; le cumul n'additionne que les décisions **moins bonnes** que celles de l'IA, sans déduire les meilleures (clôture R2 : sinon bien relancer puis se coucher avec la meilleure main réduit le don compté) ; les paris ne portent que sur des événements extérieurs au monde (clôture C2) ; céder ou jeter sa propre alliance ou un souvenir de famille porte un tort émotionnel envers soi (clôture C4) ; engagements pris pour le cercle soumis à rétractation ; échanges du persistant à ± 2 % ; point de rupture pour les dons extrêmes (E1, E5, E6, E7, §8) |
| **Décisions lourdes** | Vendre la maison d'un personnage, démissionner, faire un gros virement pendant une incarnation | Ces décisions passent par des **procédures avec délai**, pendant lequel l'IA peut se rétracter selon son caractère ([domaines/social.md](domaines/social.md), §8.4) |
| **Récusation et conflit d'intérêts** | Un joueur incarne le policier, le juge, l'agent administratif, la victime ou le témoin d'une affaire de son propre personnage ou d'un complice | **Conflit d'intérêts** étendu au **cercle** du joueur (§8), aux décisions administratives, au vote des élus, aux prêts et aux actes de victime et de témoin ; un aveu ou un témoignage ne porte que sur la mémoire réelle du personnage ([domaines/combat.md](domaines/combat.md), §11.8 ; E8) |
| **Mise en danger de soi** | Utiliser la mort d'un personnage incarné comme moyen | Aucune intention de mise en danger mortelle de soi n'est proposée ([domaines/combat.md](domaines/combat.md), §11.9) ; morts imprudentes répétées pénalisées (E28) |
| **Collusion et comptes multiples** | Un ami ou un second compte reçoit les dons, enquête sur l'affaire, harcèle à tour de rôle | Cercle du joueur (§8), période probatoire (§1), détection de collusion et comptes présumés liés, qui ne produisent que des alertes (E8, E24, E41 ; [revues/securite.md](revues/securite.md), S15) |
| **Élections** | Des joueurs incarnent des électeurs à tour de rôle pour choisir le maire | Le **bulletin** d'un électeur incarné est celui que son IA aurait émis (E9) |
| **Marionnette par roulement** | Un groupe garde en permanence un personnage puissant | Repos de l'IA (§4, E17) |
| **Client modifié, robots** | Lire ce que l'écran cache, automatiser le jeu | Serveur autoritaire, vue filtrée sans marge, validation épistémique des intentions, détection des robots hors du cœur ([revues/securite.md](revues/securite.md), §4.1, S4, S5, S16) |
| **Informations du joueur** | Le joueur fait dire au personnage un fait qu'il ne connaît pas | Le fait devient une **affirmation sans fondement** pour ceux qui l'entendent ([domaines/social.md](domaines/social.md), §3.3) |

### 7.1 Espionnage par incarnation

Les rôles invisibles et la familiarité (§6) rendent l'espionnage beaucoup plus coûteux : pour incarner l'employé de banque qui connaît le coffre, il faut d'abord savoir qu'il travaille à la banque, puis le fréquenter assez longtemps.

Deux risques demeurent :

- **en ligne, des complices** peuvent s'échanger ce qu'ils apprennent, hors du jeu ;
- un joueur peut **construire patiemment** une familiarité avec une cible.

Un troisième risque, l'**extraction des données du client**, est réglé par la population en ligne privée et la **couche cachée tirée par monde** ([01-concept.md](01-concept.md), §1.1 ; [revues/securite.md](revues/securite.md), S2, S3 ; [revues/equilibrage.md](revues/equilibrage.md), E25) : ce qu'on apprend en solo ne révèle pas qui détient quel secret sur un serveur.

Parade complémentaire (validée) : **les savoirs du personnage sont « opaques »**. Il peut s'en servir dans ses actions (ouvrir le coffre dont il connaît le code, appeler un contact), mais le joueur ne voit jamais le code, le mot de passe ou le secret lui-même, et ne peut donc pas le réutiliser avec un autre personnage. Les savoirs opaques sont des **références** sans valeur extractible, même dans les données du jeu.

## 8. Le cercle du joueur (décidé)

*Source : [revues/equilibrage.md](revues/equilibrage.md), E8 ; [revues/securite.md](revues/securite.md), S15. Pourquoi : une seule notion, testée une seule fois, couvre le joueur seul, ses amis et ses complices démasqués, au lieu de règles par compte qui ont chacune un trou.*

Le **cercle** d'un joueur est une donnée **hors fiction**, tenue par le service des comptes ; le monde l'ignore. Il réunit :

1. les **personnages liés** au joueur : son personnage persistant et les personnages qu'il a incarnés depuis 30 jours de jeu ;
2. les personnages liés à ses **comptes associés** : ses **amis mutuels**, et les comptes qu'une **session de modération**, distincte de la détection, a **rapprochés** sur dossier à partir des alertes de collusion (A8). Les comptes « présumés liés » calculés automatiquement ne produisent que des alertes ; seul un rapprochement décidé sur dossier a un effet. Le rapprochement est **qualifié** (décidé, [domaines/economie.md](domaines/economie.md), §12.3 et §12.4) : « **même personne** » fusionne les comptes, qui sont alors traités comme un seul joueur (personnages liés : aucun don, échanges bornés) ; « **complicité** » place les comptes dans le cercle associé. *Pourquoi : un second compte d'un même joueur n'a droit à rien de plus qu'un seul compte, alors qu'un complice reste un autre joueur.* Le rapprochement est **notifié** au joueur comme toute mesure lourde de modération (motifs, règle appliquée, usage de moyens automatisés, voie de recours), sans nommer l'autre compte ni l'origine du dossier ; il figure dans l'export de ses données et il est contestable (décidé, arbitrage A11 du 30/10 ; *pourquoi : la revue de sécurité le range parmi les mesures lourdes, et la lecture la plus protectrice du RGPD, A8, demande d'informer la personne d'une décision automatisée qui la touche*).

Deux règles s'appliquent, dans la **garde hors fiction** de la validation des intentions ([05-architecture.md](05-architecture.md), §2.3) :

- **Flux** : entre personnages liés, pas de don et échanges bornés ([domaines/economie.md](domaines/economie.md), §12.3) ; entre un personnage incarné et le cercle associé, au-delà de 2 000 écus cumulés par 30 jours, les flux passent par la confirmation bancaire avec délai et sont journalisés ; au-delà de ce seuil, **aucun don en espèces** vers le cercle associé, un don « au montant libre » (chapeau, pourboire) est borné à ≈ 20 écus **[à calibrer]**, et les flux nets vers le cercle associé sont plafonnés à 20 000 écus par 30 jours, comme entre personnages liés (décidé, clôture de l'équilibrage C1 ; *pourquoi : sinon les espèces contournent la confirmation bancaire*).
- **Conflit d'intérêts** : un personnage incarné ne fait **aucune décision institutionnelle ni aucun acte de procédure** dont un personnage du cercle est bénéficiaire, suspect ou partie : actes d'enquête et de justice, décisions administratives (permis, licence, subvention, marché public, logement social, recrutement public, contrôle), vote d'un élu sur un dossier qui le concerne, prêt bancaire, actes de victime et de témoin (retirer une plainte, refuser d'identifier, fournir un alibi, avouer). Le témoignage reste possible, construit à partir de la **mémoire réelle** du personnage. Les intentions ne sont pas proposées, et une intuition l'explique (« Vous êtes trop proche de cette affaire. ») ; dans le monde, le personnage se récuse.

