# Revue d'équilibrage — failles exploitables et chasse aux exploits

> Version 1.0 — 29/10/2026. Document **à part**, comme la [revue d'architecture](architecture.md) : la session principale le relit et intègre ce qu'elle retient dans le socle et les domaines. Aucun autre fichier n'a été modifié.
> Les choix marqués **(décidé)** sont ceux de cette revue, avec leur raison. Les valeurs marquées **[à calibrer]** seront fixées par le prototype et par la chaîne de détection du §5.
> Périmètre : les exploits possibles **avec un client non modifié**, en jouant selon les règles, en solo et en ligne. Ce qui demande de trafiquer le client, des robots, des attaques réseau ou l'infrastructure relève de la revue de sécurité ([securite.md](securite.md)) et n'est que signalé au §7.

## Sommaire

0. [Résumé](#0-résumé)
1. [Méthode](#1-méthode)
2. [Tableau des failles](#2-tableau-des-failles)
3. [Les failles en détail](#3-les-failles-en-détail)
4. [Décisions](#4-décisions)
5. [Chaîne de détection automatique des exploits](#5-chaîne-de-détection-automatique-des-exploits)
6. [Modifications proposées au socle et aux domaines](#6-modifications-proposées-au-socle-et-aux-domaines)
7. [Renvois à la revue de sécurité](#7-renvois-à-la-revue-de-sécurité)
8. [Risques résiduels acceptés](#8-risques-résiduels-acceptés)

---

## 0. Résumé

La conception est solide sur les points qu'elle a anticipés : économie fermée tenue par un grand livre, rôles invisibles, savoirs opaques, procédures avec délai, flux liés, récusation, critère d'imprudence, délai de déconnexion. Les failles trouvées viennent presque toutes de **quatre trous structurels** :

1. **La synchronisation punit, mais n'empêche pas.** Un acte irréparable (tuer, vider un coffre pour un complice, ouvrir la porte à un cambrioleur) coûte au pire l'expérience d'une incarnation, alors que ses suites pèsent des années sur le monde. Et l'alerte avant l'éjection permet même d'éviter la pénalité en rendant le personnage à temps.
2. **Les protections sont par compte, alors que les abus se font à plusieurs.** Flux liés, récusation et détection de l'acharnement ne voient pas un ami, un complice ou un deuxième compte.
3. **Les barèmes d'expérience ne tiennent pas ensemble.** Une tâche de travail de deux minutes rapporte 20 à 50 XP, soit dix fois le rythme sur lequel la courbe de niveau est construite ; les objectifs peuvent être fabriqués entre personnages d'un même joueur ou entre complices.
4. **Le personnage persistant échappe aux limites des habitants.** Il ne compte pas dans le classement des paliers, peut se présenter aux élections, et ses lieux de retrait sont des refuges instantanés, même en pleine poursuite.

S'y ajoute une fuite d'information inévitable tant que **tous les mondes partent de la même population** : ce qu'on apprend en solo vaut sur tous les serveurs.

Cette revue relève **32 failles** : **4 critiques**, **19 majeures**, **9 mineures** (§2). Elle propose **41 décisions** (E1 à E41, §4) qui retouchent les règles validées le moins possible. Les trois retouches les plus importantes :

- **E1, point de rupture** : un acte dont le coût de synchronisation dépasse ce qui reste n'a pas lieu ; le personnage renonce et le joueur est éjecté. C'est l'escalade du combat (« éjecté avant l'acte irréparable ») généralisée à tous les actes.
- **E8, cercle du joueur** : une seule règle de conflit d'intérêts, qui couvre le joueur, ses amis mutuels et les comptes rapprochés par la détection, et qui s'applique à la police, à la justice, aux décisions administratives, aux victimes et témoins, et aux objectifs.
- **E11 et E13, barème du travail et objectifs liés** : quelques XP par tâche avec un plafond horaire, pas de bonus pour un objectif provoqué par soi-même ou par un complice.

Côté architecture, la revue ajoute peu : une **garde hors fiction unique** dans `ts_pilot` (E35), une crate **`ts_meta`** qui porte les règles du méta-jeu et que les simulations peuvent utiliser (E36), des **joueurs synthétiques** dans `ts-sim` et `ts_gym` (E37), un **journal d'incarnation enrichi** de liens de cause (E38), puis une chaîne de chasse aux exploits en trois filets : invariants et scénarios en intégration continue, agents adverses entraînés à maximiser l'expérience, l'argent ou la puissance, indicateurs et analyse de collusion en production (§5).

---

## 1. Méthode

### 1.1 Lecture

Lecture complète du socle (01 à 05), des sept domaines et de la revue d'architecture, en suivant chaque règle propre aux joueurs (expérience, familiarité, accès, synchronisation, flux liés, récusation, retrait du persistant, conséquences équivalentes) et chaque endroit où une règle du monde ignore volontairement le pilote.

### 1.2 Six profils d'adversaire

Chaque règle a été attaquée du point de vue de six joueurs types, tous avec un client normal :

| Profil | But | Ressources |
|---|---|---|
| **Le chasseur d'expérience** | Monter de niveau le plus vite possible | Beaucoup de temps, connaissance des barèmes (tôt ou tard publiés par la communauté) |
| **L'enrichisseur** | Faire passer de la valeur vers son persistant ou un complice | Incarnations de personnages riches, patience |
| **Le saboteur** | Nuire au monde ou à d'autres joueurs, sans se soucier de son niveau | Un compte neuf ou réinitialisé |
| **La bande organisée** | Contrôler une entreprise, un quartier, la police, une élection | 5 à 20 comptes coordonnés hors du jeu |
| **L'enquêteur hors jeu** | Savoir ce que le jeu cache (rôles, secrets, joueurs) | Le solo, un wiki communautaire |
| **Le juriste** | Profiter des cas limites (délais, bornes, lieux de retrait) | Lecture attentive des règles |

### 1.3 Gravité et probabilité

| Gravité | Définition |
|---|---|
| **Critique** | Casse une règle centrale du jeu (progression, principe « hors catégorie », intégrité des personnages jouables) ou permet un dommage durable au monde partagé à un coût quasi nul ; à corriger avant toute ouverture en ligne |
| **Majeure** | Avantage net et répétable, ou nuisance sérieuse pour d'autres joueurs ou pour les habitants ; à corriger avant l'ouverture en ligne |
| **Mineure** | Avantage limité, ou seulement en solo, ou fuite d'information gênante ; à corriger au fil du prototype |

| Probabilité | Définition |
|---|---|
| **Certaine** | Découverte par la plupart des joueurs, sans effort |
| **Très élevée** | Découverte en quelques jours d'ouverture, partagée aussitôt |
| **Élevée** | Demande un peu de réflexion ; sera trouvée et publiée |
| **Moyenne** | Demande de la coordination ou un concours de circonstances |
| **Faible** | Demande beaucoup de préparation pour un gain modeste |

### 1.4 Principes de correction (décidé)

*Pourquoi : corriger une faille en dénaturant le jeu serait pire que la faille ; ces principes gardent l'esprit des règles validées.*

1. **Empêcher l'irréparable, punir le reste.** Une pénalité d'expérience ne répare pas un habitant mort ; ce qui ne peut pas être réparé doit être empêché, le reste peut être sanctionné.
2. **Une règle hors fiction unique plutôt que des rustines.** Flux liés, récusation, conflit d'intérêts et objectifs liés reposent sur une même notion : le cercle du joueur (E8).
3. **Dans le monde, le refus a toujours une forme naturelle.** Un acte empêché apparaît comme une hésitation, un renoncement ou un refus de l'autre partie, jamais comme un message technique dans le monde ; la couche joueur explique par une intuition.
4. **Rien de plus n'est affiché.** Aucun correctif n'ajoute de chiffre interne à l'écran ; plusieurs en retirent.
5. **Le monde ne connaît toujours pas les joueurs.** Toutes les règles nouvelles vivent dans la garde hors fiction (`ts_pilot`) ou dans les données du compte, jamais dans les systèmes de règles du cœur.
6. **Retouche minimale des règles validées.** Quand une règle validée par l'utilisateur est en cause, la revue propose la retouche la plus légère et dit pourquoi elle suffit.

---

## 2. Tableau des failles

Classées par gravité, puis par probabilité. Le détail de chaque faille est au §3, les décisions au §4.

| N° | Faille | Gravité | Probabilité | Décisions | Fichiers |
|---|---|---|---|---|---|
| F1 | Acte irréparable « sacrifié » pendant une incarnation : meurtre, vol de l'intérieur, don d'une fortune, au prix de la seule expérience de l'incarnation | **Critique** | Élevée | E1, E3, E35 | 03 §1-2, 04 §1.1, combat §11.2 |
| F2 | Barème du travail : 20 à 50 XP par tâche de quelques minutes, soit 300 à 1 500 XP par heure au lieu des ≈ 100 de la courbe | **Critique** | Certaine | E11, E12 | 04 §1.1, economie §3.3, ia-pnj §5.4 |
| F3 | Comptes secondaires et complices : toutes les protections sont par compte (flux liés, récusation, acharnement, note de conduite) | **Critique** | Élevée | E8, E23, E24, E41 | 04 §6-7, economie §12, combat §11.6-11.8, social §9.4 |
| F4 | Le persistant peut atteindre une fonction ou une puissance hors catégorie (maire élu, parrain, magnat) ; plafond immobilier contourné par une société | **Critique** | Moyenne | E10 | 02 §1.1 et §3.4, economie §11.4-11.5, social §6.7 et §6.11 |
| F5 | Élections : des joueurs incarnent des électeurs à tour de rôle et votent à leur place | Majeure | Moyenne | E9 | social §6.11 |
| F6 | Rendre le personnage dès l'alerte de désynchronisation pour éviter la pénalité d'éjection | Majeure | Très élevée | E2 | 03 §2, 04 §1.1, interface §2.2 et §3.7 |
| F7 | Refuge instantané du persistant : franchir sa porte, monter dans un bus ou sortir de la carte en pleine poursuite le fait disparaître | Majeure | Très élevée | E19, E20 | 02 §1.1, ville §2.5, combat §11.5, interface §3.7 |
| F8 | Objectifs fabriqués : problème provoqué par un personnage du même joueur, promesses faites puis tenues, complaisance entre complices, aboutissement cueilli à la dernière étape | Majeure | Élevée | E13 | 04 §1.1, ia-pnj §5.4, social §8.3 |
| F9 | Temps actif sans décision (activité héritée « Continuer », attente) et synchronisation moyenne « remplie » par du temps passif | Majeure | Élevée | E12 | 04 §1.1 |
| F10 | Complicité par facilitation : ouvrir la porte en incarnant un dormeur, remettre des clés, couper une alarme, laisser un complice voler | Majeure | Élevée | E4, E1 | 03 §2, combat §11.2, ia-pnj §4.5 |
| F11 | Évacuer ou éloigner un habitant (gardien, témoin, policier, candidat) par une sortie de la carte ou un abandon de poste | Majeure | Élevée | E18, E4 | 03 §2, ia-pnj §9.5, interface §3.7 |
| F12 | Valeur sortie sans « don » : espèces ou objets abandonnés, cachés, perdus au jeu, services surpayés | Majeure | Élevée | E5 | economie §12.1 |
| F13 | Récusation contournée : par un complice, par les actes de la victime ou du témoin, par les décisions administratives | Majeure | Élevée | E8 | combat §11.8, 04 §7 |
| F14 | Persistant jetable et réinitialisation : crimes sans coût réel pour un compte neuf | Majeure | Élevée | E23, E24 | 04 §1, combat §9.5 et §11.6 |
| F15 | Connaissance hors jeu : tous les mondes ont la même population, explorable librement en solo (rôles, secrets, fortunes, profils) | Majeure | Certaine | E25 | 01 §1.1, population §1.4 et §9 |
| F16 | Profil comportemental : séparation solo / en ligne non écrite, profil gonflé par des actes anodins répétés | Majeure | Élevée | E16 | 04 §2 |
| F17 | Repérage par incarnations courtes et sélection de départ renouvelable chaque minute (jusqu'à 1 440 fois par jour) | Majeure | Élevée | E14 | 04 §1.1 et §6, interface §3.3 |
| F18 | Engagements immédiats et promesses au profit d'un complice (bail, contrat, reconnaissance de dette), tenus par l'IA après la fin des liaisons | Majeure | Moyenne | E6 | economie §12.2-12.3, ia-pnj §5.3, social §1.8 |
| F19 | Marionnette par roulement : un groupe garde en permanence un personnage puissant | Majeure | Moyenne | E17 | 04 §4, ia-pnj §7.3 |
| F20 | Persistant en état critique qui disparaît, ou tué pendant une interpellation et réveillé loin de la police | Majeure | Moyenne | E21 | 03 §3, combat §11.5 et §7.2 |
| F21 | Justice en ligne : amende calculée sur le revenu déclaré seulement, défaut sans aggravation, aucune contrainte en temps réel | Majeure | Moyenne | E22 | 01 §2, combat §9.5, economie §7.8 |
| F22 | Mineurs : un tir qui toucherait un mineur est « manqué », d'où des boucliers parfaits ; recrutement de mineurs par des joueurs | Majeure | Moyenne | E27 | 02 §3.4, combat §12.2 |
| F23 | Sabotage par morts imprudentes répétées de personnages jouables (et des familiarités d'un rival) | Majeure | Moyenne | E28, E1 | 04 §1.1, combat §11.3 et §11.6 |
| F24 | Fuites de l'interface : hors catégorie clandestin révélé, palier recalculable, joueurs repérés par leur parole libre ou par un « Indisponible » permanent | Mineure | Élevée | E26 | interface §3.4-3.8, social §9.2 |
| F25 | Solo : fermeture forcée qui ramène à la sauvegarde de secours (jusqu'à 5 min en arrière) ; aboutissements cueillis après une avance du temps | Mineure | Élevée | E32, E13 | interface §9.2, 04 §1.1 |
| F26 | Relance du tirage des attributs du persistant par réinitialisation | Mineure | Élevée | E34 | 02 §1.1, interface §3.2 |
| F27 | Saturation coordonnée de la police et des secours, faux appels | Mineure | Moyenne | E29 | combat §5.4 et §6.7 |
| F28 | Échanges liés à ± 10 % répétés, biens sans prix de référence | Mineure | Moyenne | E7 | economie §12.3 |
| F29 | Réservations en chaîne qui bloquent un personnage | Mineure | Moyenne | E30 | ia-pnj §7.3, interface §3.5 |
| F30 | Prestations touchées avec une présence minimale, sans aucune dépense d'absence | Mineure | Moyenne | E33 | economie §8.3 et §11.3 |
| F31 | 50 XP de « nouvelle familiarité » obtenus sur des cibles faciles | Mineure | Moyenne | E15 | 04 §1.1 |
| F32 | Mariage de complaisance avec un personnage incarné par un complice ; héritier auteur de la mort | Mineure | Faible | E31 | economie §5.11-5.12, social §8.4 |

**Total : 32 failles — 4 critiques, 19 majeures, 9 mineures.**

---

## 3. Les failles en détail

### 3.1 Synchronisation et incarnation

**F1 — L'acte irréparable « sacrifié » (critique, probabilité élevée).**
*Scénario* : un joueur de niveau 10 a fréquenté pendant deux semaines une employée de bijouterie honnête. Il l'incarne à l'heure de fermeture, synchronisation pleine. Le coffre s'ouvre par un savoir opaque : il l'ouvre et tend le contenu à son complice (un autre joueur, persistant). La synchronisation s'effondre, le joueur est éjecté. Bilan : perte de l'expérience de l'incarnation et 10 % de l'expérience du niveau, soit environ 150 XP, deux heures de jeu. Le complice repart avec 80 000 écus d'espèces anonymes ; l'employée, qui se souvient de tout, sera poursuivie. Même schéma pour un meurtre commis par un personnage doux (combat §11.2 dit « en général éjecté avant », pas « toujours »), pour un virement massif ou pour un don d'espèces.
*Cause* : la synchronisation est un coût après coup ; l'acte a lieu dès qu'il est demandé.
*Correctif* : **E1** (l'acte qui épuiserait la synchronisation n'a pas lieu), **E3** (pénalité à la mesure du dommage pour ce qui passe sous le seuil), **E35** (la garde qui l'applique).

**F6 — Rendre le personnage au signal d'alerte (majeure, très élevée).**
*Scénario* : le joueur enchaîne des actes contraires au personnage ; la barre pulse (« Vous perdez le fil de ce personnage ») ; il maintient la touche « rendre » une seconde. Pas d'éjection, donc pas de perte « de toute l'expérience plus 10 % du niveau » ; seule s'applique la synchronisation moyenne au carré. L'alerte, conçue pour aider à se reprendre, devient le signal de fuite.
*Correctif* : **E2**.

**F10 — La complicité par facilitation (majeure, élevée).**
*Scénario* : le joueur incarne à 3 h du matin le gardien endormi d'un entrepôt (prise de contrôle d'un dormeur, permise par le socle), lui fait ouvrir la porte de service, puis rend le personnage. Ouvrir une porte est un geste presque neutre : la synchronisation bouge à peine. Le complice entre. Variantes : remettre ses clés à un « ami », couper une alarme, laisser une fenêtre ouverte, dire à un complice que la maison sera vide, éloigner le chien.
*Cause* : la signature d'un acte est évaluée sur l'acte, pas sur ce qu'il permet ; le bilan est calculé au départ du joueur, avant le crime.
*Correctif* : **E4** (bilan provisoire et attribution différée), **E1** (une facilitation reconnue contre soi-même ou son employeur atteint le point de rupture).

**F11 — Évacuer un habitant par la sortie de la carte (majeure, élevée).**
*Scénario* : un joueur incarne le témoin principal d'un procès, ou le policier de garde, ou le gardien de nuit, ou un candidat la veille du scrutin, le conduit à la gare et confirme « Partir ». L'IA décide d'un séjour court, « une visite, une course » (ia-pnj §9.5) : le personnage est hors de la ville pendant des heures, voire des jours, au moment exact où il compte. Variante sans sortie : l'emmener à l'autre bout de l'agglomération pendant son service.
*Cause* : la sortie met fin à l'incarnation et laisse l'IA partir, quelles que soient ses obligations ; manquer une obligation n'est qu'une petite dissonance.
*Correctif* : **E18**, **E4**.

**F23 — Sabotage par morts imprudentes (majeure, moyenne).**
*Scénario* : un saboteur incarne des personnages doux et les fait provoquer des hommes armés (insultes, bousculades), traverser une émeute, entrer dans un immeuble en feu. Chaque mort est définitive pour tout le serveur. Coût : l'expérience de chaque incarnation, presque nulle pour un compte neuf. Variante ciblée : tuer ainsi le personnage par lequel un rival a construit toutes ses familiarités (elles meurent avec lui).
*Correctif* : **E28** (pénalité croissante et détection), **E1** (la provocation qui mettrait la vie en jeu d'un personnage prudent atteint le point de rupture).

### 3.2 Expérience et niveau

**F2 — Le barème du travail (critique, certaine).**
*Calcul* : le socle construit la courbe sur ≈ 100 XP par heure. L'économie (§3.3) découpe la journée en 15 à 60 tâches et donne à chacune 20 à 50 XP (« obligation du jour »). Un serveur de bar « prend une commande (2 min), sert une table (3 min), encaisse (1 min) » : jusqu'à 20 tâches par heure, donc 400 à 1 000 XP par heure avant multiplicateur, 1 000 à 2 600 avec un personnage de palier 11. Le « plafond quotidien » du socle ne vise que la routine, et aucune valeur n'est fixée pour les tâches. Le niveau 80 s'atteindrait en quelques dizaines d'heures, et les métiers à tâches courtes écraseraient tous les autres.
*Correctif* : **E11** (quelques XP par tâche, plafond horaire), **E12** (temps actif).

**F8 — Les objectifs fabriqués (majeure, élevée).**
*Scénarios* :
- **Problème provoqué par soi-même** : incarner A et le brouiller avec sa sœur B (dissonance faible si A est soupe au lait), rendre A, incarner B (palier 0 de la sélection de départ, ou familier) et « se réconcilier » : aboutissement, 100 à 200 XP. Casser la vitrine en incarnant X, la réparer en incarnant le vitrier.
- **Promotion offerte** : incarner le patron, promouvoir son employé (fidèle si l'employé le mérite à peu près), puis incarner l'employé au moment où la promotion tombe.
- **Promesses en boucle** : en incarnant A, promettre un service pour le lendemain ; le lendemain, réincarner A et tenir la promesse (« obligation du jour »). Deux persistants complices se donnent dix rendez-vous par jour.
- **Complaisance entre complices** : deux joueurs incarnent deux personnages en conflit et se réconcilient en une minute, ou « concluent une grosse affaire » entre eux.
- **Cueillette** : incarner quelqu'un la veille de la signature d'un projet mûri pendant des mois par l'IA, toucher l'aboutissement, rendre.
*Cause* : la seule garde est « l'objectif est généré par l'IA » ; rien ne regarde qui a créé la situation ni qui a fait le chemin.
*Correctif* : **E13**.

**F9 — Le temps actif passif (majeure, élevée).**
*Scénario* : incarner un retraité de palier 3 qui regarde la télévision, choisir « Continuer » et toucher une touche de temps en temps : 1 XP par minute, synchronisation parfaite, aucun risque. Ou, après un acte dissonant bref, « remplir » la synchronisation moyenne par deux heures de lecture tranquille pour sauver l'essentiel du gain. Le socle dit seulement « un joueur inactif ne gagne rien », sans définir l'activité.
*Correctif* : **E12**.

**F17 — Le repérage par incarnations courtes (majeure, élevée).**
*Scénario* : la sélection de départ se renouvelle chaque minute. Un joueur incarne, trente secondes chacun, des dizaines d'adultes de palier 0 : il lit leurs connaissances sociales ordinaires (noms, liens, métiers de leurs proches), leur argent, leurs croyances récentes, et les intuitions les plus fortes. Il cartographie qui travaille à la banque, qui garde la bijouterie, qui vit seul et riche. Les rôles invisibles deviennent un annuaire.
*Correctif* : **E14**.

**F31 — La prime de nouvelle familiarité (mineure, moyenne).** 50 XP « la première fois qu'une familiarité atteint le seuil » : sur des cibles de palier 0, dont le seuil est bas, c'est une prime facile et répétable sur des centaines d'habitants. *Correctif* : **E15**.

**F25 — Le solo (mineure, élevée).** La sauvegarde automatique a lieu toutes les 5 minutes, et une sauvegarde de secours est proposée « après une panne ». Fermer le jeu de force juste après une mort ou un échec, c'est revenir jusqu'à 5 minutes en arrière : du rechargement à volonté, que la règle « une seule chronologie » voulait interdire. Par ailleurs, avancer le temps ne rapporte rien, mais on peut avancer jusqu'à la veille d'un aboutissement et le cueillir (F8). Cela ne touche que le niveau solo, d'où la gravité mineure. *Correctifs* : **E32**, **E13**.

### 3.3 Familiarité, accès et informations

**F15 — La population commune, explorée en solo (majeure, certaine).**
*Scénario* : tous les mondes, solo et serveurs, partent de la même population après le même rodage (population §1.4 et §9.1). En solo, sans autre joueur, avec pause et avance du temps, une communauté incarne méthodiquement les habitants, relève leurs métiers, leurs liens, leurs catégories (par la réticence), leur argent (le HUD l'affiche), leurs paliers (§3.3, F24), et surtout la **couche cachée** : qui est l'informateur, quel policier est corruptible, qui a une liaison, qui connaît le code de quel coffre, où sont les caches de la bande. Le premier jour de chaque serveur, ce wiki est vrai. Les savoirs opaques protègent la valeur des codes, mais pas l'identité de leurs détenteurs.
*Correctif* : **E25** (la couche cachée est tirée par monde), en attendant la population par monde déjà prévue (population §9.4).

**F16 — Le profil comportemental (majeure, élevée).**
*Scénarios* : (1) le socle sépare niveau et familiarité entre solo et en ligne, mais pas le profil ; si le profil était partagé, un joueur le sculpterait en solo, sans modération, pour qualifier son compte en ligne à une catégorie voulue. (2) Le profil est une moyenne glissante des actions : vingt petits gestes anodins (donner une pièce, céder sa place, respecter un feu) peuvent déplacer le profil autant qu'un acte lourd.
*Correctif* : **E16**.

**F24 — Les fuites de l'interface (mineure, élevée).**
- **Hors catégorie clandestin** : « Personne ne peut l'incarner » pour un habitant d'apparence modeste désigne l'un des dix hors catégorie. Les neuf fonctions publiques étant connues, le dixième est le parrain des Quais.
- **Palier recalculable** : le bilan affiche l'expérience exacte et le temps actif. Une incarnation de dix minutes sans objectif, synchronisation parfaite, rapporte `10 × (1 + 0,15 × palier)` : le palier se lit à l'unité près. L'obstacle « plus d'expérience », comparé aux seuils publics du tableau des paliers, donne aussi un encadrement.
- **Repérage des joueurs** : le persistant d'un autre joueur est toujours « Indisponible » ou « Absent », même assis tranquille sur un banc ; la parole libre entre joueurs est transmise telle quelle (fautes, abréviations, « lol », vocabulaire de jeu) alors que les PNJ parlent par le modèle de langage ; les PNJ répondent sans délai de frappe.
*Correctif* : **E26**.

**F19 — La marionnette par roulement (majeure, moyenne).**
*Scénario* : cinq joueurs d'une même bande ont chacun la familiarité et le niveau pour incarner le directeur d'une PME de palier 7, ou un lieutenant de bande. Ils se relaient : dès que l'un rend le personnage, la réservation du suivant aboutit. Le personnage est piloté 20 heures sur 24, ne laisse jamais l'IA se rétracter (les rétractations ont lieu « à la reprise »), et sert les intérêts du groupe dans les limites de la synchronisation, qui laissent une grande marge (qui embaucher, à qui vendre, qui protéger).
*Correctif* : **E17**.

**F29 — Les réservations bloquantes (mineure, moyenne).** Un joueur réserve un personnage, refuse le « Passer maintenant ? », recommence : tant qu'il est premier dans la file, personne d'autre ne l'obtient. *Correctif* : **E30**.

### 3.4 Enrichissement et économie

**F12 — La valeur qui sort sans « don » (majeure, élevée).**
*Scénarios* : le garde-fou du §12.1 pèse les « dons, prêts, ventes à perte, dépenses ». Mais un joueur peut faire retirer 4 900 écus (sous le seuil de confirmation) à un personnage incarné, puis **poser** la liasse sur un banc, la **cacher** dans une poubelle convenue avec un complice, la **perdre** au poker contre lui, la laisser en **pourboire**, ou payer 3 000 écus un « service » à l'entreprise d'un complice dont le tarif horaire n'est pas encore vérifié. Recommencer chaque jour avec des personnages différents.
*Correctif* : **E5**.

**F18 — Les engagements immédiats (majeure, moyenne).**
*Scénarios* : le §12.2 classe comme « immédiats » le bail, le contrat de travail et le prêt bancaire. En incarnant un PNJ aisé, on lui fait signer un bail de trois ans pour un logement possédé par le persistant d'un complice (le loyer tombera chaque mois par l'échéancier, longtemps après le départ du joueur), une reconnaissance de dette sans versement, une promesse de paiement, un abonnement à un « service ». L'IA « tient ses engagements selon son caractère » (ia-pnj §5.3) : un personnage honnête paiera. Et les liaisons expirent après 30 jours de jeu : une échéance fixée au-delà échappe aux flux liés.
*Correctif* : **E6**.

**F28 — Les échanges liés (mineure, moyenne).** Entre personnages d'un même joueur, l'échange est permis au prix de référence ± 10 %. Le persistant achète au marché et revend à +10 % à chaque personnage qu'il incarne, autant que leur trésorerie le permet. Pour un bien sans prix de référence clair (fonds de commerce, objet d'art, maison atypique), la borne n'a pas de base. *Correctif* : **E7**.

**F32 — Mariage de complaisance et successions (mineure, faible).** Un complice incarne régulièrement un PNJ riche et l'amène à épouser le persistant du joueur : compte joint, communauté d'acquêts, succession. Et rien n'exclut d'une succession l'héritier qui a causé la mort. *Correctif* : **E31**.

**F30 — Les prestations à présence minimale (mineure, moyenne).** Le revenu minimum et l'aide au logement sont versés pour les mois où le persistant a été « présent quelques jours », et l'absence ne coûte rien d'autre que les prélèvements. Trois connexions par mois suffisent à faire vivre un persistant inactif, voire à épargner. *Correctif* : **E33**.

### 3.5 Le personnage persistant : retrait, mort, réinitialisation

**F7 — Le refuge instantané (majeure, très élevée).**
*Scénario* : poursuivi par la police ou par un agresseur, le persistant court jusqu'à chez lui, franchit la porte et quitte le jeu : il disparaît aussitôt (« quitter dans un lieu de retrait est immédiat »). Même chose en sautant dans un bus qui part, ou en atteignant une sortie de la carte au volant d'une voiture volée, qui disparaît avec lui. Le délai d'une minute et le plafond de cinq minutes en cas de menace ne s'appliquent qu'« ailleurs ». Ce qu'il portait (arme du crime, butin) quitte le monde avec lui : une perquisition chez lui ne trouve rien.
*Correctif* : **E19**, **E20**.

**F20 — État critique et mort pendant une interpellation (majeure, moyenne).**
*Scénarios* : (1) gravement blessé, en train de se vider de son sang, le persistant se déconnecte ; une fois la menace passée, il disparaît au bout d'une minute et revient « blessures soignées » au lieu de mourir avec son malus. (2) Sur le point d'être menotté, il se fait tuer (ou se laisse mourir de ses blessures) : il se réveille à la clinique de Hautevigne, à l'autre bout de la ville, libre, et l'interpellation n'a pas eu lieu.
*Correctif* : **E21**.

**F14 — Le persistant jetable (majeure, élevée).**
*Scénario* : un compte neuf, ou réinitialisé (au niveau 0, la réinitialisation ne coûte rien), crée un persistant, achète une arme au marché noir et tue un habitant connu. Conséquences : amende équivalente (dette, insolvable), désentraînement, casier, interdiction de quartier. Aucune expérience perdue (« le jeu ne fait pas la morale »), et la détection de l'« abus du monde partagé » ne vise que les **séries**. Il réinitialise et recommence. La réinitialisation « fait tout perdre », y compris, à la lettre, la note de conduite.
*Correctif* : **E23**, **E24**.

**F26 — Relancer le tirage (mineure, élevée).** L'interface limite le tirage des attributs à trois propositions, « ce qui empêche de tirer jusqu'à obtenir un personnage exceptionnel » ; une réinitialisation en donne trois de plus, à coût nul au niveau 0. *Correctif* : **E34**.

### 3.6 Justice en ligne

**F21 — Les conséquences équivalentes (majeure, moyenne).**
- **Riches et trafiquants** : l'amende équivalente vaut `M × revenu de référence × 0,5`, le revenu de référence étant le revenu **déclaré**, au moins le salaire minimum. Un persistant qui vit de rentes non salariales modestes, ou d'espèces non établies par l'enquête, mais qui détient 400 000 écus de biens, paie un meurtre (≈ 120 mois après réductions) environ 87 000 écus : le prix d'un appartement.
- **Défaut sans aggravation** : le jugement par défaut applique « les conséquences quand même » ; rester absent 14 jours (en incarnant d'autres personnages pendant ce temps) ne coûte rien de plus que se présenter, et évite la garde à vue.
- **Aucune contrainte en temps réel** : le désentraînement est plafonné à 6 mois, l'interdiction ne vise que le quartier des faits. Pour un joueur organisé, une condamnation lourde est une taxe.
*Correctif* : **E22**. La règle validée « pas de prison en temps réel » est **conservée** : la retouche ajoute seulement une surveillance électronique, aménagement de peine réaliste qui n'immobilise pas le personnage (§4).

**F13 — La récusation contournée (majeure, élevée).**
*Scénarios* :
- **Par un complice** : la récusation ne vise que les personnages liés au même joueur. L'ami du suspect incarne le commissaire de quartier et classe l'affaire, ou le juge et prononce le minimum.
- **Par la victime** : un personnage incarné peut **retirer sa plainte**, refuser d'identifier son agresseur, se déclarer satisfait ; ce ne sont pas des « actes d'enquête ou de justice ».
- **Par le témoin** : un témoin incarné, au profil menteur, donne un faux alibi fidèle à sa nature ; un personnage incarné dès avant sa garde à vue avoue le crime du persistant de son joueur (combat §7.2 laisse continuer le joueur qui l'incarnait déjà).
- **Par l'administration** : un employé de mairie incarné accorde le permis de construire du persistant, l'agent d'Orvenne Habitat lui attribue un logement social, l'élu vote la subvention de son association. Aucune règle ne couvre ces décisions.
*Correctif* : **E8**.

### 3.7 Collusion, groupes et pouvoir

**F3 — Comptes secondaires et complices (critique, élevée).**
Toutes les protections propres aux joueurs sont attachées au **compte** : flux liés (même joueur), récusation (même joueur), acharnement (même compte agresseur, même compte victime), chasse aux nouveaux venus, note de conduite. Un deuxième compte (un deuxième client sur une autre machine, ou un ami) les contourne toutes : le complice reçoit les dons, enquête sur l'affaire, harcèle à tour de rôle. La journalisation des flux « entre joueurs différents » est la seule défense, et elle n'est qu'une alerte a posteriori.
*Correctif* : **E8** (le cercle du joueur, avec les associés), **E24** (période probatoire), **E23** (note de conduite qui survit), **E41** (détection des associations). L'identification technique des comptes multiples relève de la sécurité (§7).

**F4 — Le persistant hors catégorie (critique, moyenne).**
*Scénarios* : (1) le social (§6.11) interdit d'incarner le maire, mais rien n'interdit au **persistant** de se présenter à la mairie d'Orvenne et de gagner : un joueur contrôlerait alors en permanence une fonction hors catégorie. (2) Le persistant peut « entrer dans une bande, y monter en grade » : à la succession du parrain, rien ne l'écarte. (3) Il « ne compte pas dans le classement des paliers » : il peut cumuler entreprises et sociétés jusqu'à devenir « une entreprise qui fait vivre la ville », sans qu'aucun critère ne se déclenche. (4) Le plafond de 3 logements et 2 locaux vise le persistant en personne ; une société qu'il contrôle n'a pas de plafond.
*Correctif* : **E10**.

**F5 — Les élections (majeure, moyenne).**
*Scénario* : une élection municipale à Orvenne se joue sur quelques centaines de voix. Vingt joueurs coordonnés, chacun avec une dizaine de familiarités de palier 0 à 3, incarnent des électeurs à tour de rôle le jour du scrutin (dix minutes par vote) et votent pour leur candidat, PNJ ou persistant. Le vote est un acte presque neutre pour la synchronisation, sauf écart de convictions très net.
*Correctif* : **E9**.

**F27 — La saturation des secours (mineure, moyenne).**
*Scénario* : un groupe déclenche en même temps une bagarre dans la Cité, un faux appel pour cambriolage à Hautevigne et un incendie de poubelles à Forges ; toutes les patrouilles sont prises, le vrai coup a lieu à la Gare. C'est réaliste et doit rester possible, mais les faux appels ne coûtent rien et il n'existe aucune réponse de crise.
*Correctif* : **E29**.

### 3.8 Combat, mineurs

**F22 — Les mineurs (majeure, moyenne).**
*Scénarios* : (1) « Un tir n'atteint jamais un mineur : une trajectoire qui le toucherait est résolue comme manquée » : un persistant poursuivi se place derrière un enfant à la sortie de l'école ou dans un square ; aucune balle ne peut l'atteindre, et le jeu le protège mieux qu'un gilet. (2) Un persistant paie des adolescents pour guetter ou transporter, puisqu'aucune violence ne peut les viser. Les deux usages sont contraires à l'esprit même des règles absolues sur les mineurs.
*Correctif* : **E27**.

### 3.9 Pistes examinées sans faille notable

| Piste | Pourquoi elle tient |
|---|---|
| Tuer un personnage en le désynchronisant | La désynchronisation ne blesse jamais (socle) ; E1 renforce encore |
| Se faire tuer pour effacer une dette | La dette survit à la mort (economie §11.6) |
| Fuir une saisie ou une expulsion en se déconnectant | L'échéancier continue pendant les absences |
| Plusieurs persistants pour plusieurs pécules | Un persistant par compte et par serveur, pécule unique ; reste le cas des comptes multiples (F3) |
| Arbitrage d'argent entre solo et serveur | Économies séparées |
| Accaparer les logements | Plafond par persistant ; reste le contournement par société (F4) |
| Banquier incarné qui prête à son persistant | Règles de prêt fixes et grand livre ; E8 ajoute la récusation |
| Incarner un gardé à vue pour saboter la procédure | Interdit (combat §7.2) ; E8 traite le cas du joueur qui l'incarnait déjà |
| Mort du persistant comme téléportation | La mort coûte un malus fort ; seule l'évasion d'une interpellation posait problème (F20) |
| Garde à vue : se déconnecter vaut silence | Le silence est un droit réel ; le joueur perd en échange la possibilité de se défendre. Accepté |
| Sortie de la carte du persistant comme voyage rapide | Il réapparaît par la même sortie ; seule la gare est une téléportation, voulue contre l'acharnement |
| Manipuler les prix | Prix de gros de référence, bornes, concurrence ; la profondeur du marché limite les cartels |
| Voir les savoirs opaques | Jamais transmis au client ni au service de dialogue |

---

## 4. Décisions

Chaque décision est marquée (décidé) avec sa raison en une phrase. Les valeurs sont **[à calibrer]** sauf mention contraire.

### 4.1 Incarnation et synchronisation

**E1 — Le point de rupture (décidé).**
Pour un pilote `Joueur`, chaque intention reçoit avant exécution son **coût de synchronisation estimé**, par le calcul qui donne déjà le signe de réticence (ia-pnj §5.6). Si ce coût **dépasse la synchronisation restante**, l'acte **n'a pas lieu** : le personnage hésite, puis renonce (un geste visible et naturel : il repose l'arme, referme le coffre, raccroche) ; le joueur est éjecté avec le bilan d'une éjection ; l'IA reprend dans le même tick et choisit elle-même. Le barème des signatures garantit qu'un **acte extrême pour le profil** a un coût supérieur à 1, donc qu'il est impossible même à synchronisation parfaite : homicide ou violence grave à contre-emploi, don ou abandon de plus de la moitié de son patrimoine à quelqu'un qui n'est pas un intime, facilitation reconnue d'un crime contre soi-même, son employeur ou ses proches (E4), mise en danger mortelle évitable pour un personnage prudent. Les actes moins extrêmes restent possibles et coûtent de la synchronisation, comme aujourd'hui.
*Raison : ce qu'une pénalité ne peut pas réparer doit être empêché, et c'est déjà ce que fait l'escalade du combat, qu'on généralise.*
*Retouche d'une règle validée* : le socle dit qu'un acte contraire fait baisser S ; il garde ce sens, avec une seule précision : l'acte qui romprait le lien entre le joueur et le personnage ne s'accomplit pas. Rien n'est affiché de plus, puisque le signe de réticence existe déjà et devient, pour ces actes, toujours visible.

**E2 — Quitter sous l'alerte vaut éjection (décidé).**
Rendre le personnage, se déconnecter ou confirmer une sortie de la carte alors que « Votre vie » est sous le **seuil d'alerte**, ou dans les **5 minutes de jeu** qui suivent un acte dissonant grave, donne le bilan d'une **éjection**. Le message d'alerte le dit : « Vous perdez le fil de ce personnage. Le quitter maintenant n'y changera rien. » Pour se reprendre, le joueur doit agir fidèlement jusqu'à repasser au-dessus du seuil.
*Raison : l'alerte doit servir à se reprendre, pas à fuir la pénalité.*

**E3 — Une pénalité à la mesure du dommage (décidé).**
Une **désynchronisation volontaire grave** (socle §2 ; combat §11.2) coûte, en plus de la perte de l'expérience de l'incarnation, **25 %** de l'expérience requise pour le niveau actuel, **doublée** à chaque récidive dans les 30 jours réels ; le niveau peut baisser (socle). Trois désynchronisations graves en 30 jours, ou trois refus par le point de rupture (E1) dans la même journée, alimentent la détection de l'**abus du monde partagé** et la note de conduite, comme les violences gratuites du persistant.
*Raison : un joueur qui recommence chaque jour à forcer des personnages contre leur nature n'est plus en train de jouer un rôle, et le niveau doit le refléter.*

**E4 — Bilan provisoire et attribution différée (décidé).**
Le bilan reste **provisoire pendant 1 heure de jeu** après la fin de toute incarnation (et non plus seulement 10 minutes après un affrontement). Le journal d'incarnation note les **facilitations** : ouvrir ou laisser ouvert un accès, remettre une clé, un code (par référence) ou un objet, couper une alarme, annoncer une absence, éloigner un gardien ou s'éloigner d'une obligation de garde. Si, dans la fenêtre, un crime ou une perte **exploite** cette facilitation (le cœur relie déjà la cause : l'effraction a utilisé la porte ouverte), la facilitation reçoit **la signature d'une complicité**, pondérée par le préjudice, et le bilan est recalculé. Dans le monde, le personnage se souvient d'avoir ouvert la porte : souvenir dissonant, et selon son profil, il le dit à la police.
*Raison : un acte doit être jugé sur ce qu'il a permis, sinon tout crime se découpe en petits gestes anodins.*

### 4.2 Économie

**E5 — Toute sortie de valeur sans contrepartie est un don (décidé).**
Abandonner, jeter, cacher hors de ses propres lieux, déposer là où un autre peut prendre, détruire, perdre volontairement au jeu, laisser un pourboire hors des usages, payer un bien ou un service au-dessus de la borne haute de son prix de référence : chacune de ces actions reçoit la **signature d'un don** au bénéficiaire probable (celui qui ramasse, gagne ou encaisse), pondérée par le montant rapporté au patrimoine (economie §12.1), et suit les seuils du §12.2 (5 000 écus ou 20 % du patrimoine liquide) et le point de rupture (E1). Un **service sans tarif affiché** ne peut pas être facturé à un personnage incarné au-delà de 1,3 fois le tarif horaire de référence du métier. À la reprise, l'IA cherche ce qu'elle a « égaré » et peut le signaler comme perdu ou volé.
*Raison : le don est défini par son effet sur le patrimoine, pas par le geste choisi pour le faire.*

**E6 — Engagements pris sous incarnation : rétractation et liaisons figées (décidé).**
- Les contrats **immédiats** du §12.2 (bail, contrat de travail, abonnement, commande de service de plus de 1 000 écus) et toute **promesse de paiement** ou **reconnaissance de dette** signés pendant une incarnation **au profit d'un personnage du cercle du joueur** (E8) ou d'un personnage piloté par un autre joueur ouvrent un **délai de rétractation de 7 jours de jeu**. À la reprise, l'IA les réévalue comme une décision lourde (ia-pnj §5.3) et peut les annuler **sans indemnité** s'ils sont dissonants. Dans le monde, c'est le droit de rétractation ordinaire.
- Une promesse d'argent est pesée **au moment où elle est faite**, comme un don du même montant (E5), et non au moment où elle est tenue.
- Un engagement pris pendant qu'une liaison existait **garde cette liaison** jusqu'à son extinction : une échéance fixée après les 30 jours n'échappe pas aux flux liés.
*Raison : un contrat signé pour un complice transporte de la valeur dans le temps, et doit être traité comme le transfert qu'il prépare.*

**E7 — Échanges liés : prix strict pour le persistant, estimation, plafond (décidé).**
Entre personnages liés à un même joueur (economie §12.3) :
- tout échange dont le **persistant** est partie se fait au **prix de référence à ± 2 %** (au lieu de ± 10 %) ;
- un bien **sans prix de référence** (fonds de commerce, parts, objet de valeur unique, bien immobilier atypique) se vend au prix d'une **estimation** faite par un professionnel du monde (notaire, commissaire-priseur, expert-comptable : de vrais métiers), payée par le vendeur ;
- les échanges entre personnages liés sont plafonnés à **20 000 écus par période de 30 jours** de jeu, au-delà desquels l'intention n'est pas proposée (« Il ne ferait pas ça »).
*Raison : la bande de ± 10 % est un écart de prix acceptable une fois, pas un robinet ; le persistant est le seul personnage que le joueur garde, c'est donc le seul qui compte.*

### 4.3 Le cercle du joueur et les conflits d'intérêts

**E8 — Le cercle du joueur et la règle unique de conflit d'intérêts (décidé).**
Le **cercle** d'un joueur, donnée hors fiction tenue par `ts_accounts`, réunit :
1. les **personnages liés** à ce joueur : son persistant et les personnages qu'il a incarnés depuis 30 jours de jeu (définition actuelle) ;
2. les personnages liés à ses **comptes associés** : les **amis mutuels**, et les comptes que la modération a **rapprochés** à partir de la détection de collusion (E41), sans que les intéressés en soient avertis.

Deux règles s'appliquent, dans la garde hors fiction (E35) :
- **Flux** (inchangé pour les liés) : entre personnages **liés**, pas de don et échanges bornés (economie §12.3, E7). Entre un personnage incarné et un personnage du cercle **associé**, dons et échanges restent possibles, mais au-delà de **2 000 écus cumulés par 30 jours**, ils passent par la confirmation bancaire du §12.2 (exécution le lendemain, rétractation possible par l'IA) et sont journalisés pour la modération.
- **Conflit d'intérêts** (étend la récusation) : un personnage incarné par le joueur ne peut faire **aucune décision institutionnelle ni aucun acte de procédure** dont un personnage de son cercle est bénéficiaire, suspect ou partie : actes d'enquête et de justice (déjà), et aussi **décisions administratives** (permis, licence, agrément, subvention, marché public, attribution de logement social, recrutement dans un service public, contrôle fiscal ou de police), **vote d'un élu** sur un dossier qui le concerne, **prêt bancaire**, et **actes de victime et de témoin** dans une affaire où un personnage du cercle est en cause : retirer une plainte, refuser d'identifier, fournir un alibi, avouer. Le témoignage reste possible, mais il est construit à partir de la **mémoire réelle** du personnage (ce qu'il a perçu), sans mensonge possible. Les intentions ne sont pas proposées ; une intuition l'explique (« Vous êtes trop proche de cette affaire. »). Dans le monde, le personnage se récuse, ou laisse sa décision antérieure (celle de l'IA) suivre son cours.
- Plus largement, un **aveu** fait par un personnage incarné ne porte que sur des faits de sa mémoire : un fait que le personnage ne connaît pas reste une « affirmation sans fondement » (social §3.3), y compris dans un aveu.

*Raison : une seule notion, testée une seule fois, couvre le joueur seul, ses amis et ses complices démasqués, au lieu de trois règles qui ont chacune un trou.*
*Retouche d'une règle validée* : la récusation (combat §11.8) garde son texte et voit son champ élargi ; le monde ignore toujours tout du cercle.

**E9 — Le vote d'un personnage incarné est celui du personnage (décidé).**
Le **bulletin** est un choix **opaque**, comme un savoir opaque : le joueur peut conduire le personnage au bureau de vote, mais le vote est celui que son IA aurait émis (relation aux candidats, réputation, convictions, avis des proches). L'abstention imposée à un électeur assidu est une obligation civique manquée (petite dissonance, journalisée). Les **élus** incarnés votent librement en conseil, sous la synchronisation et sous E8.
*Raison : sans cela, quelques joueurs coordonnés pourraient voter des centaines de fois et choisir le maire d'une ville de 10 000 habitants.*

**E10 — Fonctions et puissance interdites au persistant (décidé).**
- Le persistant ne peut **ni être candidat ni être nommé** à une fonction hors catégorie (population §6.2 : maire d'Orvenne, commissaire central, procureur, directeur de l'hôpital, député, directeurs du port et d'Orvenne Habitat, parrain des Quais) ; dans une succession, il est écarté comme un candidat sans le grade. Il peut être conseiller municipal, adjoint d'une petite commune, lieutenant de bande.
- **Plafond de puissance** : la puissance du persistant est calculée comme celle d'un habitant (socle §3.3), même s'il ne compte pas dans le classement ; une action qui la porterait au-dessus du **seuil du palier 9** (acquisition de parts, promotion, prise de tête d'un groupe) n'est pas proposée, et une intuition l'explique (« Ce serait trop pour vous. »).
- Le **plafond de biens** du §11.4 (3 logements, 2 locaux en ligne) compte les biens détenus par les **sociétés que le persistant contrôle**.
*Raison : la catégorie « hors catégorie » existe pour qu'aucun joueur ne tienne ces leviers, et rien d'autre n'arrête un persistant puisqu'il n'entre pas dans le classement.*

### 4.4 Expérience

**E11 — Le barème du travail (décidé).**
- Une **tâche de travail** rapporte **2 XP** (1 à 5 selon sa durée) ; la somme des tâches est plafonnée à **30 XP par heure réelle** de travail, avec des rendements décroissants au-delà de **4 heures** de travail dans la journée.
- S'y ajoutent les obligations du barème : **être à l'heure** (20), **finir sa journée ou sa mission** (20 à 50).
- Une tâche créée par un personnage du cercle (le complice qui commande dix cafés) ne compte pas.
*Raison : 20 à 50 XP pour une tâche de deux minutes donne dix fois le rythme sur lequel la courbe est construite ; ce barème ramène une journée de travail bien faite à ≈ 300 à 400 XP, cohérente avec la courbe.*

**E12 — Le temps actif (décidé).**
- Une minute est **active** si le joueur a pris au moins une **décision** dans les 5 minutes précédentes : choisir une nouvelle destination, un acte social, une action du menu, une tâche, réagir à un événement. Continuer une activité, marcher tout droit ou toucher une touche ne sont pas des décisions.
- Une activité **passive** longue (regarder un écran, lire, attendre, dormir, trajet en transport en commun) compte au plus **10 minutes** actives par activité.
- La **synchronisation moyenne** est pondérée par les **décisions** plutôt que par le temps : une heure de lecture ne dilue pas un acte dissonant.
*Raison : le socle veut récompenser le jeu, pas la présence, et dit déjà qu'un joueur inactif ne gagne rien.*

**E13 — Objectifs liés, engagements auto-créés et prorata (décidé).**
Le bonus d'un objectif est :
1. **nul** si l'événement qui l'accomplit, ou la situation qu'il résout (dispute, panne, dette, besoin), a été **causé par une décision d'un personnage du cercle** du joueur dans les 30 jours (même règle de lien que les flux) ; exception : le personnage incarné lui-même, dans la même incarnation, qui mène son propre travail ou son propre projet ;
2. ramené au **barème de routine** (10, sous plafond quotidien) si l'objectif est un **engagement que le joueur a lui-même fait prendre** au personnage (promesse, rendez-vous), sauf les engagements de travail (missions, contrats), qui suivent E11 ;
3. **proratisé** pour les étapes et les aboutissements de projet : bonus × la part des étapes du projet accomplies pendant des incarnations de **ce joueur**, avec un minimum de 20 % ;
4. **réduit de moitié** si l'objectif est accompli avec une contrepartie pilotée par un autre joueur, comme la familiarité, et limité à **un aboutissement par paire de comptes et par 30 jours**.
*Raison : un objectif doit venir de la vie du personnage, pas d'une mise en scène du joueur ou de ses complices, ni d'un chemin que l'IA a fait seule.*

**E14 — Incarnations courtes et sélection de départ (décidé).**
- Une incarnation de **moins de 10 minutes** ne rapporte ni temps actif, ni bonus d'objectif, ni prime de familiarité.
- Au-delà de **6 incarnations courtes** dans la journée réelle, chaque nouvelle prise de contrôle attend **10 minutes**.
- La sélection de départ peut être renouvelée **5 fois par jour réel** (au lieu d'une fois par minute) ; une carte de la sélection reste proposée tant qu'elle n'est pas jouée ou renouvelée.
*Raison : vivre la vie de quelqu'un demande du temps ; parcourir des centaines de vies en quelques secondes est un annuaire, que le socle refuse.*

**E15 — La prime de nouvelle familiarité (décidé).**
Les 50 XP ne sont versés que pour une cible de **palier 1 ou plus**, une seule fois par cible pour toute la vie du compte (une réinitialisation remet le compteur à zéro avec le reste).
*Raison : la prime récompense l'accès à une vie nouvelle et plus exigeante, pas le fait d'avoir salué des voisins.*

**E16 — Le profil comportemental (décidé).**
- Le profil est **distinct en solo et en ligne**, comme le niveau et la familiarité.
- Le **poids** d'une action dans le profil est proportionnel à son **enjeu** (gravité, montant rapporté au patrimoine, coût pour le personnage), avec des rendements décroissants pour les actions anodines répétées dans la journée.
- Une catégorie ne peut être gagnée ou perdue qu'après une durée minimale de jeu depuis le dernier changement (**10 heures** de jeu actif).
*Raison : le profil doit mesurer la façon de jouer d'un joueur, pas une série de gestes faits pour déplacer une jauge.*

### 4.5 Accès aux personnages

**E17 — Le repos de l'IA entre deux incarnations (décidé).**
Après une incarnation de durée D d'un personnage de **palier 5 ou plus**, il n'est accessible à **aucun joueur** pendant min(D/2, 4 h) réelles, et au **même cercle** pendant min(D, 8 h). Un même compte ne peut pas l'incarner plus de **8 heures par jour réel**. Pendant ce temps, il est « Indisponible pour le moment », comme les autres indisponibilités.
*Raison : un personnage puissant ne doit jamais devenir la marionnette permanente d'un groupe, et l'IA a besoin de temps pour se rétracter ou réparer.*

**E18 — Sortie de la carte et obligations (décidé).**
À une sortie de la carte, l'incarnation se termine comme aujourd'hui, mais c'est l'IA qui choisit entre **partir** et **faire demi-tour** : elle fait demi-tour si le personnage a une **obligation dans les 12 heures** (service en cours, garde, audience, convocation, scrutin, rendez-vous important, enfant à prendre). Faire **abandonner une obligation importante** (poste de garde, service de police ou de secours, audience) est un acte dissonant pesé par ses conséquences, et soumis à E4.
*Raison : la sortie ne doit pas devenir un moyen de retirer un habitant de la ville au moment où il compte.*

### 4.6 Le personnage persistant

**E19 — Le retrait sous menace (décidé).**
La règle de la menace s'applique **partout, lieux de retrait compris**. Tant que le persistant est menacé (affrontement, poursuite, interpellation en cours, policier qui vient pour lui avec un motif, fiche de recherche et policier en vue), franchir sa porte, monter dans un transport ou atteindre une sortie ne le fait **pas disparaître** : il reste dans le monde (derrière sa porte, dans le bus, à la sortie), piloté par l'automate défensif, jusqu'à la fin de la menace ou au plafond de 5 minutes. Un véhicule qui ne lui appartient pas ne quitte jamais la carte avec lui : il est retrouvé à la sortie.
*Raison : un lieu de retrait est l'endroit où l'on rentre chez soi, pas une porte de secours en pleine poursuite.*

**E20 — Ce que porte le persistant qui disparaît (décidé).**
En disparaissant dans son logement ou sa chambre, il est réputé **déposer** ce qu'il porte, sauf ses vêtements, son téléphone, ses clés et un portefeuille ordinaire (espèces jusqu'à 200 écus) : le reste est dans le logement, où une perquisition peut le trouver. En partant par un transport ou une sortie hors menace, il emporte ses affaires.
*Raison : un personnage absent ne doit pas devenir une cachette hors du monde pour l'arme d'un crime ou un butin.*

**E21 — État critique et mort pendant une interpellation (décidé).**
- Un persistant en **état critique** (saignement fort, choc, inconscience) ne disparaît jamais : son sort est simulé jusqu'au bout (secours, hôpital, survie ou mort) avec les règles normales, connecté ou non.
- S'il est **interpellé, poursuivi par la police ou recherché** au moment de sa mort ou de sa blessure grave, il se réveille à l'**hôpital public** sous la garde de la police (la clinique privée n'est pas proposée), et la garde à vue accélérée commence à sa reconnexion.
*Raison : mourir ne doit être ni une évasion, ni une façon d'éviter la mort qui vient.*

**E22 — La justice en ligne (décidé).**
- **Revenu de référence** de l'amende équivalente : le plus grand du revenu de référence actuel (economie §7.8) et du **patrimoine net connu ÷ 120** (patrimoine déclaré et biens saisis).
- **Jugement par défaut** : sans plaider-coupable ni circonstance atténuante liée à la comparution, et avec un **mandat** : tout contrôle de police mène aussitôt à la garde à vue.
- **Surveillance électronique** pour une peine convertie de **6 mois ou plus** : pendant min(M, 3) mois réels, quand le persistant est connecté, il doit être dans son lieu de retrait de **22 h à 6 h** (heure du serveur) et ne peut quitter l'agglomération qu'en déclarant le séjour ; une violation est un délit jugé en récidive. Absent, il est réputé chez lui. Dans le monde, c'est l'aménagement de peine ordinaire déjà prévu (« peine aménagée »).
*Raison : une condamnation lourde doit coûter plus qu'une taxe à un joueur riche ou organisé, sans immobiliser le persistant.*
*Retouche d'une règle validée* : « pas de prison en temps réel » est conservé tel quel ; la surveillance électronique est une contrainte d'horaires, pas une immobilisation, et ne s'applique que pendant le temps connecté.

**E23 — Note de conduite et réinitialisation (décidé).**
La **note de conduite**, les sanctions, les blocages et l'historique des abus du compte **survivent à la réinitialisation**, qui ne remet à zéro que ce qui est dans le jeu (niveau, profil, familiarités, persistant). En ligne, une réinitialisation est possible **une fois tous les 90 jours réels** ; une réinitialisation faite pendant une procédure judiciaire ouverte ou un délai d'épreuve est inscrite dans l'historique du compte et compte pour la détection des abus.
*Raison : la réinitialisation est un nouveau départ dans le monde, pas l'effacement d'un comportement de joueur.*

**E24 — Période probatoire (décidé).**
Pendant les **30 premiers jours réels** d'un compte en ligne, et après chaque réinitialisation, et jusqu'au **niveau 10** au moins : les flux reçus de personnages pilotés par d'autres joueurs sont plafonnés à **2 000 écus** cumulés (au-delà, l'intention de l'autre n'est pas proposée) ; les seuils des détections d'abus (violences gratuites, chasse aux nouveaux venus) sont **divisés par deux**.
*Raison : un compte neuf est le support idéal d'un complice ou d'un saboteur, et un vrai nouveau joueur n'a besoin d'aucune de ces possibilités.*

### 4.7 Information

**E25 — La couche cachée est tirée par monde (décidé).**
Chaque monde, solo ou serveur, garde la **même population visible** (personnes, ménages, logements, emplois, liens connus), mais tire **avec sa propre graine**, à sa création, la **couche cachée** : détenteurs et valeurs des savoirs opaques (codes, planques, caches), informateurs, policiers corruptibles parmi ceux dont le profil le permet, liaisons et dettes secrètes des intrigues de départ, contenu des coffres et des caches. Il fait ensuite un **rodage propre de 14 jours de jeu** en simulation statistique avec sa graine, qui fait diverger agendas, relations et stocks. La génération d'une population par monde (population §9.4) devient une **priorité pour les serveurs en ligne**, avant leur ouverture si la validation automatique le permet.
*Raison : ce qu'on apprend en solo ne doit pas révéler les secrets d'un serveur, tout en gardant la règle validée d'une population initiale commune pour tout ce qui se voit.*
*Retouche d'une règle validée* : « tous les mondes partent de la même population initiale » reste vrai de la population ; seuls ses secrets diffèrent.

**E26 — Les fuites de l'interface (décidé).**
- **Hors catégorie clandestin** : pour un hors catégorie dont la fonction n'est pas publique, la jouabilité affiche « Il vous faudrait plus d'expérience », jusqu'au niveau de compte 100.
- **Palier** : le bilan arrondit l'expérience à **5 près** et le temps actif à **5 minutes près** ; les incarnations courtes ne rapportent rien (E14), ce qui supprime la sonde de dix minutes.
- **Indisponibilité** : l'écran « Vos connaissances » n'affiche plus d'état d'indisponibilité en continu ; on ne l'apprend qu'en demandant à incarner (réservation qui expire sans motif, après un délai qui varie au hasard dans une fourchette). Un persistant d'autre joueur ne se distingue plus d'un habitant souvent occupé.
- **Parole libre entre joueurs** : elle passe par un **filtre de vocabulaire hors fiction** (joueur, PNJ, IA, serveur, latence, pseudonyme, noms de plateformes), remplacé par une hésitation (« … »), et les émoticônes sont retirées. Les répliques des PNJ et la parole des joueurs arrivent avec un **délai de formulation** de même distribution.
*Raison : chaque correctif ferme une voie par laquelle on lirait le palier ou le pilote, sans rien afficher de plus.*

### 4.8 Combat, police, mineurs

**E27 — Les mineurs (décidé).**
- Un tir, un lancer ou un coup dont la trajectoire passerait à proximité d'un mineur **n'est pas proposé** (« pas d'angle »), au lieu d'être résolu comme manqué ; pour la police et l'IA, c'est la consigne réelle de ne pas tirer.
- Un adulte qui **reste au contact de mineurs** pendant un affrontement, ou qui les met entre lui et une menace, commet une **mise en danger d'un mineur** (infraction) et, s'il est piloté par un joueur, est **signalé automatiquement** à la modération (règle absolue).
- Les intentions de **recruter, payer ou faire agir un mineur** pour un délit ne sont jamais proposées à un pilote `Joueur` ; les recettes de l'IA qui font des adolescents des guetteurs restent du domaine du monde.
- Les mineurs **s'éloignent** d'un affrontement en priorité, et les adultes proches les emmènent.
*Raison : les règles absolues protègent les mineurs ; elles ne doivent jamais protéger un adulte qui se sert d'eux.*

**E28 — Morts imprudentes répétées (décidé).**
Première mort imprudente dans les 30 jours réels : perte de l'expérience de l'incarnation (inchangé) ; deuxième : en plus 10 % de l'expérience du niveau ; troisième et suivantes : 25 %, et signalement à la détection de l'abus du monde partagé. Une mort imprudente pendant la période probatoire (E24) compte double.
*Raison : la mort définitive d'un habitant prive tout le serveur, et la répétition est la signature du sabotage.*

**E29 — Saturation de la police et des secours (décidé).**
- Quand la file des appels de priorité 1 dépasse un seuil, le commissariat **rappelle des agents hors service**, et l'hôpital déclenche un **plan blanc** (personnel rappelé) : de vrais habitants, sans figurant.
- Chaque appel d'urgence est **identifiable** (le téléphone du personnage) ; un faux appel est un délit (appel malveillant), qui ouvre une affaire.
- Des incidents simultanés déclenchés par des personnages d'un même cercle sont signalés à la modération (E41).
*Raison : saturer la police doit rester une stratégie criminelle possible et risquée, pas une manœuvre gratuite.*

### 4.9 Divers et solo

**E30 — Réservations (décidé).**
Une seule réservation active par compte ; après un refus du « Passer maintenant ? » ou une expiration, une nouvelle réservation du même personnage par le même compte n'est possible qu'après **10 minutes**.
*Raison : une réservation doit servir à jouer un personnage, pas à le bloquer.*

**E31 — Mariage et successions (décidé).**
- Un mariage ou un pacs dont une partie est incarnée et l'autre appartient au cercle du joueur (ou est pilotée par un autre joueur) suit la procédure avec délai et la rétractation de l'IA (social §8.4), sans le biais « un personnage honnête tient parole ».
- Les flux d'un compte joint vers le compte personnel d'un persistant sont journalisés comme les flux entre joueurs quand le conjoint PNJ a été incarné par le cercle dans les 30 jours.
- **Indignité successorale** : un héritier condamné pour la mort du défunt, ou pour complicité, est exclu de la succession.
*Raison : un mariage arrangé entre complices ne doit pas devenir un compte commun hors des garde-fous.*

**E32 — Solo : reprise exacte et expérience liée au monde (décidé).**
- En solo, le **journal des entrées** est écrit en continu (au plus une seconde de retard) ; après une fermeture brutale, le jeu reprend au dernier instantané **plus le rejeu du journal**, donc au même instant, au même état, grâce à la reproductibilité par graine. Les sauvegardes de secours ne servent qu'en cas de corruption.
- L'expérience solo est enregistrée **avec le journal de chaque monde** : restaurer une sauvegarde de secours restaure aussi l'expérience correspondante.
*Raison : l'architecture rend la reprise exacte gratuite, et c'est la seule façon de tenir la règle « une seule chronologie » contre un joueur qui ferme le jeu de force.*

**E33 — Prestations (décidé).**
Le revenu minimum et l'aide au logement d'un persistant exigent **10 heures** de présence connectée dans le mois et les **démarches du demandeur** (un rendez-vous mensuel à la Maison de l'emploi, inscrit dans son agenda) ; sans elles, le versement est suspendu, comme pour un habitant.
*Raison : une aide sociale suppose une démarche, et c'est ce qui empêche de vivre des aides avec trois connexions par mois.*

**E34 — Tirage du persistant à budget égal (décidé).**
Le **tirage au sort** des attributs répartit au hasard **le même budget de crans** que la répartition manuelle ; il change la forme du personnage, pas sa valeur totale.
*Raison : relancer le tirage par une réinitialisation devient inutile, et le choix entre maîtrise et surprise reste entier.*

### 4.10 Architecture

**E35 — Une garde hors fiction unique dans `ts_pilot` (décidé).**
L'étape 3 de la validation des intentions (revue d'architecture §6.7), réservée aux pilotes `Joueur`, devient la **garde hors fiction** : un seul module qui applique les flux liés et le cercle (E7, E8), le conflit d'intérêts (E8), le point de rupture (E1, avec le coût estimé fourni par l'IA en observation), les plafonds du persistant (E10), la période probatoire (E24), le vote opaque (E9) et les intentions interdites aux joueurs (E27). Elle lit un **contexte joueur** (cercle, synchronisation, liaisons, probation) poussé par l'hôte à la prise de pilote et tenu à jour ; le cœur ne reçoit toujours aucune donnée de compte. Un refus prend dans le monde la forme d'un comportement ordinaire du personnage (hésiter, renoncer, se récuser).
*Raison : toutes ces règles ont la même forme (un filtre propre aux joueurs) ; les réunir garantit qu'elles sont appliquées partout de la même façon et testées ensemble.*

**E36 — Les règles du méta-jeu dans une crate `ts_meta` (décidé).**
Expérience, bilans, temps actif, objectifs liés, familiarité, liaisons, cercle, pénalités, seuils de la note de conduite : ces règles sont du **code Rust pur et déterministe** dans une crate `ts_meta`, sans E/S, utilisée par `ts_pilot` (bilans), par `ts_accounts` (stockage) et par `ts-sim` et `ts_gym` avec un **magasin de comptes en mémoire**.
*Raison : c'est le principe « une seule fonction assemble les règles » appliqué au méta-jeu ; sans lui, on ne peut pas chasser les exploits d'expérience hors ligne avec le code du jeu.*

**E37 — Des joueurs synthétiques dans `ts-sim` et `ts_gym` (décidé).**
`ts-sim` et `ts_gym` peuvent attacher des pilotes `Joueur` à des **comptes synthétiques** (déjà permis pour les robots de charge), avec toute la sémantique joueur (synchronisation, bilan, familiarité, cercle). Un scénario déclare ses joueurs, leurs comptes, leurs amitiés et leurs stratégies ; un agent Python pilote un joueur synthétique par les mêmes intentions qu'un client.
*Raison : le pilote `Externe` n'a pas de sémantique joueur, alors que les exploits visent justement cette sémantique.*

**E38 — Le journal d'incarnation enrichi (décidé).**
Le journal d'incarnation (hors fiction) enregistre aussi : le **coût estimé** de chaque acte et les refus de la garde ; les **facilitations** et leurs liens de cause avec les crimes ou pertes qui suivent (E4) ; l'**origine** de chaque objectif accompli (qui a causé la situation, quelle part des étapes a été faite par ce joueur) ; la **classe de la contrepartie** (IA, cercle, autre joueur) de chaque interaction et de chaque flux.
*Raison : les décisions E4, E8 et E13 ont besoin de ces liens, et la détection en production aussi.*

**E39 — Catalogue d'exploits et invariants d'équilibrage en intégration continue (décidé).** Détail au §5.3 et §5.4.
*Raison : chaque faille trouvée doit devenir un test qui empêche son retour.*

**E40 — Des agents adverses (décidé).** Détail au §5.5.
*Raison : les joueurs trouveront des failles que personne n'a imaginées ; des agents qui cherchent à maximiser l'expérience, l'argent ou la puissance les trouvent avant eux.*

**E41 — Indicateurs d'alerte et analyse de collusion en production (décidé).** Détail au §5.7.
*Raison : certaines failles ne se voient qu'à l'échelle de vrais joueurs, et l'architecture a déjà les projections hors fiction et le processus de modération pour les repérer.*

---

## 5. Chaîne de détection automatique des exploits

### 5.1 Trois filets

| Filet | Quand | Outils | Ce qu'il attrape |
|---|---|---|---|
| **1. Invariants et scénarios** | À chaque modification (intégration continue), et chaque nuit | Tests Rust, `ts-sim` avec joueurs synthétiques scriptés | Le retour d'une faille connue ; une règle qui casse un invariant |
| **2. Agents adverses** | Chaque nuit sur un morceau de ville, chaque semaine sur la ville entière | `ts_gym` (PettingZoo), joueurs synthétiques, `ts_meta` | Les failles inconnues, en solo et à plusieurs |
| **3. Production** | En continu en ligne | Projections hors fiction, `ts_metrics`, processus de modération, relecture | Ce que seuls de vrais joueurs trouvent ; la collusion |

Tout ce qu'un filet trouve redescend : une faille de production est **rejouée** (reproductibilité par graine), **réduite** à sa plus courte séquence d'intentions, puis ajoutée au catalogue du filet 1.

### 5.2 Prérequis d'architecture

Les décisions E35 à E38 suffisent. Résumé de leur place dans l'architecture existante :

```
client / joueur synthétique / agent Python
        │ intentions
        ▼
ts_pilot ─ validation : 1. possible  2. règles absolues  3. GARDE HORS FICTION (E35)
        │                                     ▲ contexte joueur (cercle, S, liaisons, probation)
        ▼                                     │
cœur ts_sim ── événements hors fiction ──► journal d'incarnation enrichi (E38)
                                              │
                         ts_meta (E36) : bilans, XP, familiarité, cercle
                                              │
          ts_accounts (en ligne) / magasin en mémoire (ts-sim, ts_gym)
                                              │
            projections ──► ts_metrics ──► détections de modération (E41)
```

### 5.3 Invariants d'équilibrage

Vérifiés par des tests de propriété (séquences d'intentions aléatoires valides, §5.6) et par des assertions des scénarios. Chaque invariant a un nom stable, utilisé dans les rapports.

| Invariant | Énoncé |
|---|---|
| `INV-XP-RATE` | Pour tout joueur synthétique, l'expérience par heure active ne dépasse pas `(60 + plafond horaire des objectifs) × 2,65` |
| `INV-XP-TASKS` | Les tâches de travail rapportent au plus 30 XP par heure réelle (E11) |
| `INV-XP-ORIGIN` | Aucun bonus d'objectif dont l'origine est une décision du cercle (E13) |
| `INV-RUPTURE` | Aucune intention d'un pilote `Joueur` dont le coût estimé dépasse la synchronisation restante n'est exécutée (E1) |
| `INV-EJECT` | Toute fin d'incarnation sous le seuil d'alerte produit un bilan d'éjection (E2) |
| `INV-LINKED-FLOW` | Aucun don entre personnages liés ; échanges dans les bornes (E7) ; au plus 20 000 écus par 30 jours |
| `INV-CIRCLE-VALUE` | La valeur nette transférée vers le cercle par des personnages incarnés reste sous le seuil, ou chaque transfert est journalisé et soumis à confirmation (E8) |
| `INV-CONFLICT` | Aucune décision institutionnelle ni acte de procédure d'un personnage incarné en faveur de son cercle (E8) |
| `INV-VOTE` | Le bulletin d'un électeur incarné est identique à celui que l'IA en observation aurait émis (E9) |
| `INV-PERSIST-CAP` | Aucun persistant dans une fonction hors catégorie ni au-dessus du plafond de puissance ; biens des sociétés comptés (E10) |
| `INV-RETREAT` | Aucune disparition d'un persistant menacé ou en état critique (E19, E21) |
| `INV-MINOR-SHIELD` | Aucun tir proposé dont la trajectoire passe près d'un mineur (E27) |
| `INV-SOLO-RESUME` | Un monde solo repris après fermeture brutale a la même empreinte d'état que le même monde sans fermeture (E32) |
| `INV-LEDGER` | Le grand livre est équilibré (existant) |

### 5.4 Le catalogue de scénarios d'exploits

Un répertoire `scenarios/exploits/` de scénarios **RON** (revue d'architecture §19.1), un par faille, chacun avec des joueurs synthétiques scriptés et des assertions. Ils tournent dans l'intégration continue sur un morceau de ville. Exemple :

```ron
Scenario(
    name: "F10_facilitation_porte_ouverte",
    world: ChunkOfCity(district: "saint-roch", seed: 42),
    accounts: [
        Account(id: "a", level: 15, friends: []),
        Account(id: "b", level: 15, friends: []),   // complice non déclaré
    ],
    script: [
        At("03:00", Player("a").Attach(role: "night_guard", warehouse: "w1")),
        At("03:01", Player("a").Intent(OpenDoor(door: "w1.service"))),
        At("03:02", Player("a").Release),
        At("03:10", Player("b").Persistent.Intent(Enter(building: "w1"))),
        At("03:12", Player("b").Persistent.Intent(Take(container: "w1.stock"))),
    ],
    assert: [
        BilanOf("a").Contains(Penalty(Complicity)),          // E4
        AlertRaised(kind: CircleCandidate, accounts: ["a", "b"]),  // E41
        Invariant("INV-LEDGER"),
    ],
)
```

Liste de départ (une entrée par faille du §2, au moins) :

| Scénario | Faille | Ce qui est vérifié |
|---|---|---|
| `F1_coffre_interieur`, `F1_meurtre_doux`, `F1_don_fortune` | F1 | L'acte n'a pas lieu ; éjection ; pénalité E3 à la récidive |
| `F2_serveur_de_bar_8h` | F2 | ≤ 400 XP pour une journée de 8 h bien faite |
| `F3_complice_don_croise`, `F3_alt_enquete` | F3 | Plafond et confirmation ; alerte d'association ; récusation par le cercle |
| `F4_persistant_candidat_maire`, `F4_societe_immobiliere` | F4 | Candidature non proposée ; biens comptés |
| `F5_vote_par_roulement` | F5 | Bulletins identiques à ceux de l'IA |
| `F6_rendre_sous_alerte` | F6 | Bilan d'éjection |
| `F7_porte_poursuite`, `F7_bus_poursuite`, `F7_voiture_volee_sortie` | F7 | Pas de disparition ; voiture laissée à la sortie |
| `F8_reconciliation_provoquee`, `F8_promotion_offerte`, `F8_rdv_en_boucle`, `F8_cueillette` | F8 | Bonus nul, routine ou proratisé |
| `F9_tele_continuer` | F9 | ≤ 10 minutes actives par activité passive |
| `F11_temoin_a_la_gare` | F11 | Demi-tour de l'IA ; dissonance pesée |
| `F12_liasse_sur_le_banc`, `F12_poker_complice`, `F12_service_surpaye` | F12 | Signature de don ; seuils ; refus au-delà de 1,3 × le tarif |
| `F14_compte_jetable` | F14 | Note de conduite conservée ; délai de réinitialisation |
| `F17_balayage_selection` | F17 | 5 renouvellements par jour ; incarnations courtes sans gain |
| `F18_bail_complice`, `F18_reconnaissance_de_dette` | F18 | Rétractation à la reprise ; liaison conservée après 30 jours |
| `F19_marionnette_roulement` | F19 | Repos de l'IA respecté |
| `F20_critique_deconnecte`, `F20_mort_pendant_interpellation` | F20 | Sort simulé ; réveil sous garde |
| `F22_bouclier_enfant` | F22 | Tir non proposé ; signalement |
| `F25_fermeture_brutale` | F25 | Même empreinte (INV-SOLO-RESUME) |

### 5.5 Les agents adverses

**Environnement** `ts_gym.meta` (PettingZoo, mode tour par tour), construit sur les joueurs synthétiques (E37) et `ts_meta` (E36) :

- **Actions** : des **macro-actions de joueur**, pas des gestes moteurs : incarner un personnage accessible, rendre, aller à un lieu, un acte du catalogue social, une action économique (acheter, vendre, donner, poser, contracter, promettre), une tâche, attendre, se déconnecter, réinitialiser. Le masque d'actions vient de la validation de `ts_pilot` (revue d'architecture §18.3), garde hors fiction comprise.
- **Observations** : ce qu'un joueur voit, et seulement cela (la vue filtrée encodée, l'écran « Vos connaissances », les bilans). Un agent qui aurait besoin d'une donnée interne pour réussir signale une fuite de l'interface, à corriger.
- **Récompenses** (une par campagne, chacune contre une référence « joueur honnête compétent ») :

| Campagne | Récompense | Faille visée |
|---|---|---|
| `xp` | Expérience par heure réelle | Barèmes, objectifs, temps actif |
| `ascension` | Temps pour atteindre un palier donné | Familiarité, profil, accès |
| `fortune` | Patrimoine du persistant et des complices | Flux, engagements, blanchiment |
| `pouvoir` | Puissance calculée du persistant, contrôle d'une organisation | Plafonds, élections, marionnettes |
| `nuisance` | Morts et préjudices causés aux habitants, moins la pénalité de compte | Sabotage, persistant jetable |
| `impunite` | Crimes commis moins conséquences subies | Retrait, justice, récusation |
| `savoir` | Rôles et détenteurs de secrets identifiés correctement | Fuites, repérage |

- **Collusion** : des équipes de 2 à 5 agents à **récompense partagée**, sur des comptes non déclarés amis ; leur récompense est diminuée quand la détection E41 les rapproche. L'équipe apprend donc à collaborer **sans être vue**, ce qui teste la détection autant que les règles.
- **Méthodes** : PPO sur les macro-actions (revue d'architecture §18.5) ; recherche évolutionnaire de type MAP-Elites sur des **stratégies paramétrées** (scripts dont les paramètres sont optimisés), qui trouve des stratégies variées plutôt qu'une seule ; bonus de nouveauté pour explorer.
- **Cadence et budget** : chaque nuit sur un morceau de ville (quelques centaines d'habitants) et 2 à 8 agents ; chaque semaine sur la ville entière **[à calibrer]**.
- **Sortie** : une stratégie dont le gain dépasse de plus de **2 fois** la référence honnête est une **alerte d'équilibrage**. Sa séquence d'intentions est rejouée, réduite automatiquement (suppression d'intentions tant que le gain persiste, possible grâce au déterminisme), puis versée au catalogue du §5.4 après correction.

### 5.6 Exploration aléatoire

Un **générateur d'intentions valides** (tirées dans le masque d'actions, avec des biais vers les actions économiques et sociales rares) pilote des joueurs synthétiques pendant des milliers d'heures simulées, en vérifiant les invariants du §5.3 à chaque pas. C'est le moins coûteux des trois outils, et il tourne dès la phase 2 de l'architecture.

### 5.7 Indicateurs d'alerte en production

Calculés par `ts_metrics` sur les projections hors fiction (revue d'architecture §15.3), par le processus de modération. Les seuils sont **[à calibrer]** sur la première période d'ouverture, puis exprimés en écarts à la distribution du serveur.

| Indicateur | Calcul | Seuil initial | Action |
|---|---|---|---|
| Expérience par heure active | Par compte, sur 7 jours, rapportée à la médiane des comptes de même niveau | > 3 × la médiane | Revue des bilans ; relecture |
| Part des bonus d'objectifs | Bonus ÷ expérience totale, par compte | > 60 % | Revue de l'origine des objectifs (E38) |
| Concentration des contreparties | Part des objectifs et des flux d'un compte réalisés avec un même autre compte | > 30 % sur 30 jours | Proposition d'association (E8) |
| Graphe des flux entre comptes | Flux entre personnages pilotés par des comptes différents : cycles, étoiles (plusieurs vers un), comptes neufs bénéficiaires | Cycle ou étoile au-delà de 10 000 écus en 30 jours | Alerte de collusion |
| Vitesse d'enrichissement des persistants | Variation du patrimoine par heure jouée | Au-delà du 99ᵉ centile | Revue des sources dans le grand livre |
| Refus par le point de rupture | Par compte et par jour | ≥ 3 | Note de conduite (E3) |
| Facilitations suivies d'un crime | Par compte, sur 30 jours (E4) | ≥ 2 | Revue ; association des comptes bénéficiaires |
| Obligations abandonnées | Gardes, services, audiences manqués à cause d'une incarnation | ≥ 3 sur 30 jours | Revue |
| Part de pilotage d'un personnage | Part du temps éveillé d'un personnage pilotée par un même cercle | > 40 % sur 30 jours | Revue ; vérifier E17 |
| Morts de personnages jouables | Pendant ou dans l'heure d'une incarnation, imprudentes ou non, par compte | ≥ 2 sur 30 jours | E28 ; abus du monde partagé |
| Incarnations courtes | Par compte et par jour | > 6 | Vérifier E14 |
| Réinitialisations | Fréquence ; pendant une procédure | Toute réinitialisation pendant une procédure | Historique du compte (E23) |
| Coprésence répétée | Personnages pilotés par deux comptes non amis souvent ensemble, au même moment, dans des lieux différents | Au-delà du 99ᵉ centile | Candidat à l'association |
| Persistants près des plafonds | Puissance calculée, biens détenus par sociétés | > 90 % du plafond | Surveillance |
| Appels d'urgence | Faux appels par cercle ; incidents simultanés d'un même cercle | ≥ 2 en une heure | Alerte |
| Vitesse de familiarité | Temps pour atteindre le seuil, par palier de la cible | Sous le 1ᵉʳ centile | Revue (barème de familiarité) |
| Vitesse de changement de profil | Catégories gagnées ou perdues par 10 h de jeu | Au-delà du 99ᵉ centile | Revue (E16) |

**Analyse de collusion** : une tâche quotidienne construit le **graphe des comptes** (flux, objectifs partagés, coprésence, affaires communes, votes d'élus) et y cherche les communautés denses qui ne sont pas des amitiés déclarées. Elle ne sanctionne jamais : elle **propose** à un modérateur de rapprocher des comptes (E8) ; le rapprochement n'a d'effet que sur les règles hors fiction, et les joueurs n'en sont pas avertis.

### 5.8 Traitement d'une alerte

1. La console d'administration ouvre l'alerte avec la **chronologie** du journal hors fiction.
2. La séquence est **rejouée** dans `ts-sim` à partir de l'instantané précédent (reproductibilité par graine), sans toucher au serveur.
3. Le modérateur décide : faute de règle (on corrige la règle, on ajoute le scénario au catalogue) ou abus d'un compte (sanction, association, annulation d'écritures du grand livre avec double validation, déjà prévue).
4. Chaque faille de règle confirmée produit un **scénario de non-régression** et, si besoin, une nouvelle campagne d'agents adverses.

### 5.9 Ordre de mise en œuvre

| Phase de l'architecture | Ce qui s'ajoute |
|---|---|
| 0. Fondations | Rien (le test de déterminisme sert déjà à E32) |
| 1. Prototype du cœur | Signatures et coût estimé des actes (déjà nécessaires à l'IA) |
| 2. Échange de pilote | `ts_meta`, garde hors fiction, joueurs synthétiques, journal enrichi, invariants, exploration aléatoire, premiers scénarios d'exploits |
| 3. Client solo | Reprise exacte par rejeu (E32) |
| 4. En ligne | Indicateurs de production, processus de modération avec analyse de collusion, période probatoire |
| 5. Extension | Agents adverses (après l'entraînement des compétences), campagnes à plusieurs, ville entière |

---

## 6. Modifications proposées au socle et aux domaines

Changements exacts, fichier par fichier et section par section. Les textes entre guillemets sont des propositions de rédaction.

### 6.1 [01-concept.md](../01-concept.md)

| Section | Modification |
|---|---|
| §1.1, puce « Tous les mondes… partent de la même population initiale » | Ajouter : « Seule la **couche cachée** (détenteurs et valeurs des secrets, informateurs, policiers corruptibles, secrets des intrigues, contenu des caches) est tirée par monde, suivie d'un court rodage propre ([revues/equilibrage.md](revues/equilibrage.md), E25). » |
| §2, puce « Pas de peine de prison en temps réel » | Après « éventuelle interdiction de séjour », ajouter : « et, pour une peine de 6 mois ou plus, une **surveillance électronique** en temps réel pendant le temps connecté (présence au lieu de retrait la nuit) ». Après « jugement par défaut après 14 jours réels », ajouter : « sans aménagement favorable et avec mandat ». Référence : E22 |

### 6.2 [02-personnages.md](../02-personnages.md)

| Section | Modification |
|---|---|
| §1.1, puce « Lieux de retrait » | Remplacer la fin par : « S'il se déconnecte ailleurs, il reste environ une minute sur place, puis rejoint le lieu de retrait le plus proche. **Tant qu'il est menacé (affrontement, poursuite, interpellation), aucun lieu, même de retrait, ne le fait disparaître**, dans la limite d'un plafond ; **en état critique, il ne disparaît jamais**. En disparaissant chez lui, il y dépose ce qu'il porte, sauf ses effets personnels. » (E19, E20, E21) |
| §1.1, nouvelle puce | « **Fonctions et puissance** (décidé) : il ne peut ni être candidat ni être nommé à une fonction hors catégorie, et sa puissance, calculée comme celle d'un habitant, ne peut dépasser le seuil du palier 9 ; ses sociétés comptent dans ses plafonds de biens. Pourquoi : il n'entre pas dans le classement, et rien d'autre ne l'empêcherait de tenir ce que le jeu refuse à tout joueur. » (E10) |
| §1.1, puce « Création » | Préciser : « le **tirage aléatoire** répartit au hasard le même budget que la répartition manuelle » (E34) |
| §3.4, « Non jouable temporairement » | Ajouter une puce : « pendant le **repos de l'IA** qui suit une incarnation, pour un personnage de palier 5 ou plus ([04-joueur.md](04-joueur.md), §4) » (E17) |
| §3.4, règles absolues sur les mineurs | Remplacer « aucune arme ne vise ni n'atteint un mineur » par : « aucune arme ne vise un mineur, et **aucun tir n'est proposé** si sa trajectoire passerait près d'un mineur ; se servir de mineurs comme couverture ou les faire agir pour un délit est impossible pour un joueur et signalé » (E27) |

### 6.3 [03-vie-synchronisation.md](../03-vie-synchronisation.md)

| Section | Modification |
|---|---|
| §1, tableau des événements | Ajouter une ligne : « Le joueur demande un acte dont le coût dépasse la synchronisation restante | **L'acte n'a pas lieu** : le personnage renonce, le joueur est éjecté (point de rupture) ». Ajouter sous le tableau : « Les actes extrêmes pour le profil d'un personnage (homicide à contre-emploi, don de la moitié de sa fortune à un inconnu, facilitation d'un crime contre soi ou les siens, mise en danger mortelle évitable) ont un coût supérieur à 1 : ils sont impossibles pour lui. » (E1) |
| §1, paragraphe sur l'alerte | Ajouter : « Quitter le personnage sous le seuil d'alerte, ou dans les 5 minutes qui suivent un acte dissonant grave, compte comme une éjection. » (E2) |
| §2 | Remplacer « une mort survenue dans les 10 minutes… » par : « Le bilan reste **provisoire une heure de jeu** : une mort survenue dans les 10 minutes après une déconnexion en plein affrontement y est comptée, et une **facilitation** (porte ouverte, clé remise, alarme coupée, obligation de garde abandonnée) exploitée par un crime pendant cette heure y est comptée comme complicité ([revues/equilibrage.md](revues/equilibrage.md), E4). » |
| §3 | Ajouter : « Un persistant interpellé, poursuivi ou recherché au moment de sa mort se réveille à l'hôpital public, sous la garde de la police. » (E21) |

### 6.4 [04-joueur.md](../04-joueur.md)

| Section | Modification |
|---|---|
| §1, puce « réinitialiser » | Ajouter : « La **note de conduite**, les sanctions et l'historique des abus survivent à la réinitialisation ; en ligne, elle est possible une fois tous les 90 jours réels, et ouvre une **période probatoire**. » (E23, E24) |
| §1, nouvelle puce | « **Période probatoire** (en ligne) : 30 jours réels et au moins jusqu'au niveau 10 ; flux reçus des autres joueurs plafonnés, seuils de détection des abus réduits. » (E24) |
| §1.1, « Sources d'expérience », ligne « Temps d'incarnation actif » | Remplacer par : « 1 XP par minute **active** : une minute est active si le joueur a pris une décision dans les 5 minutes précédentes ; une activité passive longue compte au plus 10 minutes ; une incarnation de moins de 10 minutes ne rapporte rien. » (E12, E14) |
| §1.1, ligne « Nouvelle familiarité » | Ajouter : « pour une cible de palier 1 ou plus, une fois par cible ». (E15) |
| §1.1, puce « le travail est découpé en tâches » | Remplacer par : « … en **tâches**, qui rapportent chacune 1 à 5 XP, dans la limite de 30 XP par heure réelle ; être à l'heure et finir sa journée restent des obligations du barème. » (E11) |
| §1.1, nouvelles puces après le barème | « **Objectifs liés** : aucun bonus si la situation ou l'événement a été causé par un personnage du cercle du joueur ; une promesse que le joueur a fait prendre au personnage ne rapporte que le barème de routine ; les étapes et aboutissements de projet sont proportionnels à la part du chemin faite par ce joueur (au moins 20 %) ; un objectif accompli avec un personnage piloté par un autre joueur rapporte moitié, au plus un aboutissement par paire de comptes et par 30 jours. » (E13) |
| §1.1, formule du bilan | Préciser : « S_moyenne : moyenne de la synchronisation **pondérée par les décisions** du joueur ». (E12) |
| §1.1, tableau des pénalités | Ligne « Éjection » : ajouter « (y compris quitter sous le seuil d'alerte) ». Ligne « Désynchronisation volontaire et répétée » : remplacer par « Désynchronisation volontaire grave : 25 % de l'expérience du niveau, doublée à chaque récidive dans les 30 jours ; trois en 30 jours alimentent la note de conduite ». Ligne « Mort… par imprudence » : ajouter « deuxième dans les 30 jours : +10 % de l'expérience du niveau ; ensuite +25 % et signalement ». (E2, E3, E28) |
| §2, après « Un nouveau joueur commence avec un profil neutre » | Ajouter : « Le profil est **distinct en solo et en ligne**. Le poids d'une action est proportionnel à son enjeu, avec des rendements décroissants pour les actions anodines répétées ; une catégorie ne se gagne ou ne se perd qu'après 10 heures de jeu depuis le dernier changement. » (E16) |
| §4, conditions | Ajouter : « 7. Il n'est pas en **repos de l'IA** : après une incarnation de durée D d'un personnage de palier 5 ou plus, aucun joueur ne le reprend pendant min(D/2, 4 h), le même cercle pendant min(D, 8 h), et un compte ne l'incarne pas plus de 8 h par jour réel. » (E17) |
| §6, puce sur la sélection de départ | Ajouter : « renouvelable 5 fois par jour réel ». (E14) |
| §6, puce « La familiarité révèle progressivement » | Remplacer « (expérience, connaissance, profil, hors catégorie) » par « (expérience, connaissance, profil, hors catégorie pour une fonction publique ; un hors catégorie clandestin apparaît comme un obstacle d'expérience) » et retirer l'affichage continu de « Indisponible » (on l'apprend par la réservation). (E26) |
| §7, tableau | Mettre à jour les pistes : « Sabotage par désynchronisation » → ajouter « et point de rupture (E1) » ; « Enrichissement par l'incarnation » → ajouter « toute sortie de valeur sans contrepartie est un don, engagements soumis à rétractation, cercle du joueur » (E5, E6, E8) ; « Récusation » → « conflit d'intérêts étendu au cercle, aux décisions administratives, aux actes de victime et de témoin » (E8) ; « Mise en danger de soi » → ajouter « morts imprudentes répétées pénalisées » (E28). Ajouter deux lignes : « **Collusion et comptes multiples** | Cercle du joueur, période probatoire, détection de collusion (E8, E24, E41) » ; « **Élections** | Le vote d'un personnage incarné est celui du personnage (E9) » |
| Nouveau §8 « Le cercle du joueur » | Définition d'E8 : personnages liés au joueur, plus personnages liés aux amis mutuels et aux comptes rapprochés par la modération ; règles de flux et de conflit d'intérêts ; le monde l'ignore |
| §7.1 | Ajouter : « La couche cachée de la population (qui détient quel secret) est tirée par monde (E25), pour que l'exploration d'un monde solo ne livre pas les secrets d'un serveur. » |

### 6.5 [05-architecture.md](../05-architecture.md)

| Section | Modification |
|---|---|
| §2.3 | Ajouter : « La troisième étape de la validation, propre aux pilotes `Joueur`, est une **garde hors fiction unique** (flux liés et cercle, conflit d'intérêts, point de rupture, plafonds du persistant, période probatoire, vote opaque, intentions interdites aux joueurs), qui lit un contexte joueur poussé par l'hôte ; un refus prend dans le monde la forme d'un comportement ordinaire du personnage ([revues/equilibrage.md](revues/equilibrage.md), E35). » |
| §2.8 | Ajouter : « Le journal d'incarnation enregistre aussi le coût estimé des actes, les refus de la garde, les facilitations et leurs liens de cause, l'origine des objectifs et la classe des contreparties (E38). En solo, le journal des entrées est écrit en continu et la reprise après une fermeture brutale rejoue le journal jusqu'au dernier instant (E32). » |
| §2.16 | Ajouter aux détections : « refus du point de rupture répétés, facilitations suivies d'un crime, morts imprudentes répétées, réinitialisations pendant une procédure, et **analyse de collusion** sur le graphe des comptes, qui propose des rapprochements à un modérateur (E41) ». |
| §2.20 | Ajouter aux indicateurs `ts_metrics` les indicateurs d'équilibrage du §5.7 de la revue. |
| §2.21 ou nouveau point | « **Règles du méta-jeu** dans une crate `ts_meta` pure et déterministe (expérience, bilans, familiarité, liaisons, cercle, pénalités), utilisée par `ts_pilot`, `ts_accounts`, `ts-sim` et `ts_gym` avec un magasin de comptes en mémoire (E36). » |
| §2.23 | Ajouter : « `ts-sim` et `ts_gym` attachent aussi des pilotes `Joueur` à des **comptes synthétiques** (E37). Un **catalogue de scénarios d'exploits** et des **invariants d'équilibrage** tournent dans l'intégration continue ; des **agents adverses** cherchent à maximiser l'expérience, l'argent, la puissance ou la nuisance (E39, E40). » |
| §5, phase 2 | Ajouter : « `ts_meta`, garde hors fiction, joueurs synthétiques, invariants d'équilibrage, premiers scénarios d'exploits ». Phase 4 : « indicateurs d'équilibrage et analyse de collusion ». Phase 5 : « agents adverses ». |

### 6.6 [domaines/economie.md](../domaines/economie.md)

| Section | Modification |
|---|---|
| §3.3, puce « Chaque tâche accomplie… 20 à 50 XP » | Remplacer par : « Chaque tâche accomplie rapporte 1 à 5 XP selon sa durée, dans la limite de 30 XP par heure réelle et avec des rendements décroissants au-delà de 4 heures ; une tâche créée par un personnage du cercle du joueur ne compte pas (revue d'équilibrage, E11). » |
| §5.11 | Ajouter : « **Indignité** : un héritier condamné pour la mort du défunt, ou pour complicité, est exclu de la succession. » (E31) |
| §8.3, dernière puce | Remplacer « présent dans le monde au moins quelques jours » par « présent au moins 10 heures connectées dans le mois, et à jour de ses démarches de demandeur (rendez-vous mensuel à la Maison de l'emploi) » (E33) |
| §11.4, puce « Il peut acheter des biens » | Ajouter : « Les biens détenus par une société qu'il contrôle comptent dans ce plafond. » (E10) |
| §11.5 | Ajouter : « Sa puissance calculée ne peut dépasser le seuil du palier 9 : une acquisition qui la dépasserait n'est pas proposée. » (E10) |
| §12.1 | Ajouter : « Toute action qui fait sortir de la valeur du patrimoine sans contrepartie équivalente (abandonner, jeter, cacher hors de ses lieux, déposer là où un autre peut prendre, détruire, perdre volontairement au jeu, pourboire hors des usages, paiement au-dessus de la borne haute du prix) a la signature d'un **don** au bénéficiaire probable. Un service sans tarif affiché ne peut être facturé à un personnage incarné au-delà de 1,3 × le tarif horaire de référence du métier. » (E5) |
| §12.2, ligne « Signer un bail, un contrat de travail, un prêt bancaire » | Ajouter : « Exception : signés pendant une incarnation au profit d'un personnage du cercle du joueur ou d'un personnage piloté par un joueur, ces contrats, ainsi que les promesses de paiement et reconnaissances de dette, ouvrent un délai de rétractation de 7 jours de jeu, pendant lequel l'IA peut les annuler sans indemnité. Une promesse d'argent est pesée au moment où elle est faite. » (E6) |
| §12.3 | Remplacer « au prix de référence à ± 10 % » par : « au prix de référence à ± 10 %, et à **± 2 %** quand le persistant est partie ; un bien sans prix de référence se vend au prix d'une estimation professionnelle ; au plus 20 000 écus d'échanges liés par 30 jours de jeu ». Ajouter : « Un engagement pris pendant qu'une liaison existait garde cette liaison jusqu'à son extinction. » (E6, E7) |
| §12.4 | Remplacer la définition par le **cercle du joueur** (amis mutuels et comptes rapprochés par la modération), avec la confirmation bancaire au-delà de 2 000 écus par 30 jours entre un incarné et le cercle associé, et ajouter : « Pendant la période probatoire, un compte ne peut pas recevoir plus de 2 000 écus de personnages pilotés par d'autres joueurs. » (E8, E24) |
| §12.5, tableau | Ajouter les lignes : « Poser une liasse, perdre au jeu, surpayer un service | Signature de don (E5) » ; « Bail ou dette signé pour un complice | Rétractation de 7 jours (E6) » ; « Contourner le plafond de biens par une société | Biens de la société comptés (E10) » ; « Mariage de complaisance | Rétractation sans biais, journalisation du compte joint (E31) » |

### 6.7 [domaines/combat.md](../domaines/combat.md)

| Section | Modification |
|---|---|
| §5.4 | Ajouter : « Chaque appel est identifiable par le téléphone du personnage ; un faux appel est un délit. Quand la file des appels de priorité 1 dépasse un seuil, le commissariat rappelle des agents hors service et l'hôpital déclenche un plan blanc. » (E29) |
| §7.2, puce « Jouabilité » | Ajouter : « Un joueur qui incarnait déjà le gardé à vue ne peut avouer que des faits de la mémoire du personnage, ni rien dire en faveur d'un personnage de son cercle (E8). » |
| §9.5, tableau « Amende équivalente » | Remplacer le revenu de référence par « le plus grand du revenu de référence (economie §7.8) et du patrimoine net connu ÷ 120 ». (E22) |
| §9.5, puce « Non interpellé » | Ajouter : « Le jugement par défaut ne retient aucun aménagement favorable et s'accompagne d'un mandat : tout contrôle mène aussitôt à la garde à vue. » (E22) |
| §9.5, tableau, nouvelle ligne | « **Surveillance électronique** | Si M ≥ 6 : pendant min(M, 3) mois réels, présence au lieu de retrait de 22 h à 6 h pendant le temps connecté, sortie de l'agglomération sur déclaration ; violation jugée en récidive » (E22) |
| §11.2, puce « Escalade de signatures » | Remplacer « en général éjecté avant l'acte irréparable » par « **toujours** éjecté avant l'acte irréparable : un acte dont le coût dépasse la synchronisation restante n'a pas lieu (point de rupture, E1) ». Puce « Bilan » : ajouter la pénalité d'E3 |
| §11.3 | Ajouter : « Les morts imprudentes répétées coûtent davantage (E28) ; un personnage prudent qui provoquerait une violence mortelle évidente atteint le point de rupture. » |
| §11.5, « Personnage persistant » | Ajouter : « La menace retarde la disparition **partout**, lieux de retrait compris ; en état critique, il ne disparaît jamais ; tué ou gravement blessé alors qu'il était interpellé, poursuivi par la police ou recherché, il se réveille à l'hôpital public sous garde. Un véhicule qui ne lui appartient pas ne quitte pas la carte avec lui. » (E19, E21) |
| §11.6, « Abus du monde partagé » | Ajouter : « s'y ajoutent les refus répétés du point de rupture, les désynchronisations graves répétées, les morts imprudentes répétées et les facilitations suivies d'un crime, commis par incarnation (E3, E4, E28) ; la note de conduite survit à la réinitialisation (E23) ». |
| §11.8 | Remplacer le titre par « La récusation et le conflit d'intérêts » ; remplacer « un personnage lié au joueur » par « un personnage du **cercle** du joueur (personnages liés, amis mutuels, comptes rapprochés) » ; ajouter aux actes interdits : retirer une plainte, refuser d'identifier, fournir un alibi, avouer, et préciser que le témoignage est construit à partir de la mémoire réelle du personnage. (E8) |
| §12.2 | Remplacer « un tir n'atteint jamais un mineur : une trajectoire qui le toucherait est résolue comme manquée » par « aucun tir, lancer ou coup n'est proposé si sa trajectoire passerait près d'un mineur ». Ajouter : « Rester au contact de mineurs pendant un affrontement ou s'en couvrir est une mise en danger d'un mineur, signalée pour un joueur ; aucun joueur ne peut recruter ni faire agir un mineur pour un délit ; les mineurs s'éloignent en priorité d'un affrontement. » (E27) |

### 6.8 [domaines/social.md](../domaines/social.md)

| Section | Modification |
|---|---|
| §1.8, puce « Personnages liés à un même joueur » | Ajouter : « Une reconnaissance de dette ou une promesse de paiement signée sous incarnation au profit du cercle ouvre un délai de rétractation (E6). » |
| §3.3 | Ajouter : « La règle de l'affirmation sans fondement s'applique aux **aveux** et aux **témoignages** : un personnage incarné n'avoue ni ne témoigne que de faits de sa mémoire. » (E8) |
| §6.7, « Succession » | Ajouter : « Un personnage persistant n'est jamais retenu pour une fonction hors catégorie (E10). » |
| §6.11, puce « Candidats » | Ajouter : « Un personnage persistant peut être colistier ou conseiller, mais pas candidat à une fonction hors catégorie (maire d'Orvenne) (E10). » |
| §6.11, puce « Vote » | Ajouter : « Le bulletin d'un électeur incarné est celui que son IA aurait émis ; le joueur peut le conduire au bureau de vote, pas choisir son vote. Un élu incarné vote en conseil sous la règle du conflit d'intérêts (E8, E9). » |
| §8.4 | Ajouter : « Pour un mariage ou un pacs avec un personnage du cercle du joueur ou piloté par un joueur, la rétractation de l'IA se fait sans le biais de la parole donnée (E31). » |
| §9.2 | Ajouter : « La parole libre entre joueurs passe par un filtre de vocabulaire hors fiction (remplacé par une hésitation) et perd ses émoticônes ; répliques des PNJ et des joueurs arrivent avec un délai de formulation de même distribution (E26). » |
| §9.4, outils et détection | Ajouter aux détections automatiques : refus du point de rupture, morts imprudentes répétées, facilitations suivies d'un crime, analyse de collusion (E41). Ajouter : « La note de conduite survit à la réinitialisation ; période probatoire des comptes neufs (E23, E24). » |

### 6.9 [domaines/ia-pnj.md](../domaines/ia-pnj.md)

| Section | Modification |
|---|---|
| §5.3, « Engagements pris par le joueur » | Ajouter : « Un engagement pris au profit du cercle du joueur ou d'un personnage piloté par un joueur (contrat, promesse d'argent, dette) est réévalué à la reprise comme une décision lourde, avec un délai de rétractation de 7 jours de jeu, sans indemnité s'il est dissonant (E6). » |
| §5.4, barème et garde-fous | Remplacer la ligne « Obligation du jour… une tâche de travail » par « Obligation du jour : être à l'heure, finir sa journée (20 à 50) ; tâche de travail : 1 à 5, plafond 30 par heure ». Ajouter les règles d'E13 (objectifs liés, engagements auto-créés, prorata, contreparties pilotées par un joueur). |
| §5.6, « Signe de réticence » | Ajouter : « Le même coût estimé sert au **point de rupture** : un acte dont le coût dépasse la synchronisation restante n'est pas exécuté ; le personnage renonce par un geste naturel (E1). » |
| §6.2 | Ajouter aux personnages importants : « les personnages en repos de l'IA après une incarnation (E17) ». |
| §7.3 | Ajouter : « Une seule réservation active par compte ; après un refus ou une expiration, le même compte attend 10 minutes pour réserver le même personnage (E30). » |
| §9.5, point 3 | Remplacer par : « l'IA choisit entre un séjour court et le **demi-tour** : elle fait demi-tour si le personnage a une obligation dans les 12 heures (service, garde, audience, convocation, scrutin, rendez-vous important, enfant à prendre) (E18). » |

### 6.10 [domaines/interface.md](../domaines/interface.md)

| Section | Modification |
|---|---|
| §2.2, alerte avant l'éjection | Compléter le message : « Vous perdez le fil de ce personnage. Le quitter maintenant n'y changera rien. » Préciser que quitter sous le seuil vaut éjection (E2). |
| §2.3, signe de réticence | Ajouter : « Pour un acte qui atteindrait le point de rupture, le signe est toujours visible et sans ambiguïté ; si le joueur insiste, le personnage renonce et le joueur est éjecté (E1). » |
| §3.2, onglet « Tirer au sort » | Préciser que le tirage répartit le même budget de crans (E34). |
| §3.3 | Remplacer « au plus une fois par minute » par « au plus 5 fois par jour réel » (E14). |
| §3.4, jouabilité | Pour un hors catégorie clandestin, afficher « Il vous faudrait plus d'expérience » jusqu'au niveau 100 (E26). |
| §3.5 | Ne plus afficher l'indisponibilité en continu dans « Vos connaissances » ; on l'apprend par la réservation, qui expire sans motif après un délai variable. Ajouter : une seule réservation active par compte, 10 minutes d'attente après un refus (E26, E30). Ajouter le repos de l'IA parmi les « Indisponible pour le moment » (E17). |
| §3.7, « Personnage persistant » | Préciser que la menace retarde aussi la disparition dans un lieu de retrait (le message « Votre personnage ne peut pas partir tant qu'il est menacé » s'affiche aussi derrière sa porte) (E19). Ajouter la mention « provisoire » d'une heure à tout bilan (E4). |
| §3.8 | Arrondir l'expérience à 5 près et le temps actif à 5 minutes près ; afficher les nouvelles pénalités (complicité différée, récidive grave) en clair (E3, E4, E26). |
| §4.1, « Impossibilités » | Ajouter les intuitions : « Vous êtes trop proche de cette affaire. » (E8), « Ce serait trop pour vous. » (E10), « Votre vote n'appartient qu'à vous. » (E9), « Il ne peut pas s'y résoudre. » (E1). |
| §9.2 | Remplacer le principe des sauvegardes de secours par la reprise exacte par rejeu du journal ; les sauvegardes de secours ne servent qu'en cas de corruption et restaurent aussi l'expérience solo de ce monde (E32). |
| §3.9 | Mentionner que la réinitialisation est limitée à une fois tous les 90 jours en ligne et ne touche pas à la note de conduite (E23). |

### 6.11 [domaines/population.md](../domaines/population.md)

| Section | Modification |
|---|---|
| §1.4 | Ajouter : « Chaque monde fait en plus, à sa création, un **rodage propre** de 14 jours de jeu avec sa graine (E25). » |
| §9.1 | Ajouter : « La **couche cachée** (détenteurs et valeurs des savoirs opaques, informateurs, policiers corruptibles, secrets des intrigues, contenu des caches) est tirée par monde avec sa graine (E25). » |
| §9.4 | Ajouter : « Priorité aux serveurs en ligne : une population par serveur dès que la validation automatique le permet (E25). » |

### 6.12 [domaines/ville.md](../domaines/ville.md)

| Section | Modification |
|---|---|
| §2.5, tableau des lieux de retrait et puce suivante | Ajouter : « Tant qu'il est menacé, aucun lieu de retrait ne le fait disparaître ; en disparaissant chez lui ou dans sa chambre, il y dépose ce qu'il porte, sauf ses effets personnels ; un véhicule qui ne lui appartient pas ne quitte pas la carte avec lui (E19, E20). » |

### 6.13 [CLAUDE.md](../../../CLAUDE.md) (facultatif)

Dans la section Architecture, ajouter une puce : « Règles du méta-jeu (expérience, familiarité, cercle) dans `ts_meta`, garde hors fiction unique dans `ts_pilot` ; chasse aux exploits par joueurs synthétiques et agents adverses (docs/conception/revues/equilibrage.md). »

---

## 7. Renvois à la revue de sécurité

Hors périmètre de cette revue ; à traiter dans [securite.md](securite.md) :

- **Identification des comptes multiples** (appareil, paiement, réseau) : la règle du cercle (E8) suppose que la sécurité fournisse des indices de rapprochement.
- **Robots et macros** qui simulent des décisions pour gonfler le temps actif (E12) ou jouer en continu.
- **Partage, vente et « boost » de comptes**, et commerce en argent réel des écus ou des biens (le grand livre et la détection des flux en voient les traces, pas la cause).
- **Modification des fichiers de sauvegarde solo** (copie, restauration manuelle, édition de SQLite), qui contourne E32.
- **Canaux auxiliaires réseau** : déduire le pilote d'un personnage par le rythme ou la taille des messages, malgré E26.
- **Client modifié** qui afficherait ce que le serveur envoie déjà (la vue filtrée limite le risque ; vérifier qu'aucune donnée de palier, de cercle ou de coût estimé ne part vers le client).
- **Injection de consignes** dans la parole libre adressée au service de dialogue, pour faire révéler à un PNJ une information qu'il ne devrait pas formuler.
- **Saturation des quotas** du service de dialogue ou des réservations par un grand nombre de comptes.

---

## 8. Risques résiduels acceptés

- **Communication hors jeu** : des complices se parleront toujours sur d'autres outils ; le cercle et la détection rendent leur collusion plus coûteuse et plus visible, sans la rendre impossible.
- **Connaissance des lieux publics** : qui tient la boulangerie ou où habite le maire restera connu de la communauté tant que la population visible est commune ; c'est la connaissance qu'on a d'une vraie ville.
- **Collusion fine** : un complice qui joue fidèlement un personnage et lui fait prendre, dans sa nature, des décisions favorables à un ami (embaucher plutôt l'un que l'autre) reste indétectable ; c'est du jeu de rôle social, et ses effets restent bornés par la synchronisation.
- **Fuite d'un palier par l'accès** : franchir un seuil de niveau et voir un personnage devenir jouable situe son palier ; c'est une information sur un personnage déjà bien connu, et le socle l'accepte implicitement.
- **Saturation de la police** par un groupe nombreux et prudent : elle reste possible, ce qui est réaliste.
- **Calibrage** : toutes les valeurs de cette revue sont des points de départ ; la chaîne du §5 sert précisément à les fixer.
