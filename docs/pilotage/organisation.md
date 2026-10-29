# Organisation des sessions

> Fichier tenu **uniquement par l'orchestrateur** (décidé le 29/10). Les autres sessions le lisent et ne le modifient pas.
> *Pourquoi : avant Git, deux sessions qui modifient le même fichier peuvent écraser le travail l'une de l'autre ; ce texte a donc quitté le README de la conception, tenu par le responsable conception.*

## 1. Hiérarchie

| Niveau | Session | Rôle | Quand |
|---|---|---|---|
| 1 | **Orchestrateur** (session de départ) | Reçoit les demandes de l'utilisateur et les confie au bon responsable ; tient la feuille de route et décide du passage d'une phase à la suivante (§5) ; arbitre entre responsables ; surveille la consommation (§4) ; garde les tâches d'exploitation du dépôt prévues pour la « session principale » dans [revues/developpement-agents.md](../conception/revues/developpement-agents.md) (§9, §10) | En place |
| 2 | **Responsable conception** | Est la « **session principale** » des documents de conception : socle 01 à 04, domaines et README de la conception ; relectures de cohérence, arbitrages, intégration des revues, retours aux domaines ; répond aux questions de conception venues des autres responsables | En place |
| 3 | Sessions de domaine et de revue | Chacune ne modifie que son fichier ; nouveau domaine **direction artistique et sonore** | En place |
| 2 | **Responsable architecture et sécurité** | Gardien technique, indépendant du développement qui est pressé de livrer :<br>- propriétaire de [05-architecture.md](../conception/05-architecture.md) et des revues techniques (architecture, sécurité, génération, développement par agents) ;<br>- tient le modèle de menaces et les exigences de sécurité bloquantes de chaque phase ;<br>- valide le découpage de chaque phase avant le lancement des tâches, ainsi que toute tâche qui change une frontière de crate, un format, le protocole ou un chemin sensible ;<br>- donne le dernier avis, bloquant, sur les PR des chemins sensibles ;<br>- fait l'audit de sécurité de fin de phase et suit l'audit des dépendances.<br>Il écrit des décisions, pas de code | Phase 0, avant le responsable développement |
| 3 | Revues et audits techniques (éphémères, lecture seule) | Revue d'un choix d'architecture, audit de sécurité de fin de phase, étude ciblée (cryptographie, réseau, persistance) : chaque fois une session experte neuve | Phase 0 |
| 2 | **Responsable développement** | Chef technique : découpe le périmètre de [05-architecture.md](../conception/05-architecture.md) §5 en tâches courtes, fait lui-même les PR courtes sur les fichiers partagés, lance les sessions de tâche, répartit les 3 emplacements de compilation, vérifie et fusionne les PR | Phase 0 |
| 3 | Sessions de tâche (éphémères) | Une tâche = une branche = un worktree = une PR, archivée après fusion. Zones : noyau et règles ; réseau, serveur et persistance (chemins sensibles : revue de sécurité obligatoire) ; client Godot (une seule à la fois ; elle fait aussi la **lecture des animations dans le jeu** : machine d'états, mélanges, placement des pieds, regard, cadence réduite au loin, repli sur l'animation de la famille) ; génération ; Python et entraînement ; services annexes (dialogue, comptes, console d'administration) | Phase 0 |
| 2 | **Responsable qualité et essais** | Indépendant du développement, pour qu'aucun code ne soit jugé par celui qui l'a écrit :<br>- travaille dans la copie principale (compilations optimisées, profil `sim`) ;<br>- campagnes `ts-sim`, déterminisme, budgets de performance, chasse aux exploits (`ts_meta`), invariants d'équilibrage, puis tests à l'aveugle « joueur ou PNJ » ;<br>- prépare les versions jouables pour les essais de l'utilisateur et transforme ses retours en tickets.<br>Il ne corrige rien : les défauts vont au développement, les règles à changer à la conception, les failles à l'architecture | Fin de la phase 0 |
| 3 | Campagnes et audits (éphémères, lecture seule) | Analyses de simulations ; audit d'équilibrage de fin de phase, par une session experte neuve | Fin de la phase 0 |
| 2 | **Responsable contenus** | Produit les assets selon la direction artistique : modèles 3D, textures, sons et **animations**, à partir de sources libres ou sous licence et de génération procédurale ; outils de génération ; registre des licences et règles Git LFS.<br>**Animations** ([direction-artistique.md](../conception/domaines/direction-artistique.md) §3.6) :<br>- choix des captures de mouvement libres ;<br>- script de reciblage sur le squelette commun ;<br>- catalogue de gestes sémantiques relié aux énumérations du cœur, avec une animation au moins par famille ;<br>- niveaux de détail d'animation fournis avec chaque clip | Début de la phase 3 (premier client), avec un premier catalogue d'environ 40 animations |
| 2 | *Responsable exploitation* (plus tard) | Mise en ligne : hébergement, PostgreSQL, sauvegardes, supervision, outils de modération | Avant l'ouverture en ligne |

Pas de session permanente par crate, ni de relecteur permanent, ni de chef de projet ou de responsable de feuille de route à part (décidé). La feuille de route est partagée en deux :
- l'orchestrateur tient `feuille-de-route.md` (branche `pilotage`, §3) : jalons, priorités, état, rôles à lancer ;
- le responsable développement tient le plan détaillé à l'intérieur d'une phase. *Pourquoi :*
- les sessions permanentes accumulent du contexte, coûtent cher et se marchent dessus ;
- la relecture se fait par `/code-review` et par un sous-agent neuf, à chaque PR ;
- la coordination revient à l'orchestrateur ;
- un responsable de feuille de route n'aurait aucun pouvoir sur le travail. Il recopierait l'état tenu par les autres et ajouterait un intermédiaire entre l'utilisateur et les décisions de priorité.

Partage des décisions : les règles de jeu reviennent à la conception, les décisions techniques à l'architecture, sans recouvrement. Un choix technique qui change le jeu passe d'abord par la conception.

## 2. Sessions et barre latérale

- **Chaque responsable lance lui-même ses sous-sessions** : elles s'affichent sous lui et lui rendent compte. *Pourquoi : les comptes rendus automatiques vont à la session qui a lancé, et la barre latérale ne permet pas de changer le parent d'une session.*
  - **Repli quand l'application refuse** (constaté le 30/10 : une session lancée par une autre ne peut pas toujours lancer de sessions) : le responsable confie ses tâches à des **sous-agents isolés en worktree**. Chacun a sa branche `claude/<sujet>` et son worktree, pousse sa branche et ouvre sa PR, avec les mêmes règles qu'une session de tâche : propriété des fichiers, emplacements de compilation, vérifications avant PR. Le résultat revient au responsable, qui le note dans son carnet.
  - *Pourquoi : la règle « une tâche = une branche = un worktree = une PR » est conservée ; seul le contenant change. Le responsable reste seul à fusionner.*
- Les 13 sessions de domaine et de revue lancées avant le responsable conception restent affichées sous l'orchestrateur. C'est le responsable conception qui les pilote, par message ; leurs comptes rendus automatiques arrivent aussi chez l'orchestrateur, qui n'agit pas dessus. Les détacher est irréversible et coupe leurs comptes rendus : on ne le fait pas.
- **Même mode de permission pour toutes les sessions** (`auto`). *Pourquoi : une session dans un autre mode retient les messages des autres sessions jusqu'à l'accord de l'utilisateur, ce qui bloque la chaîne sans prévenir.*
- **Les responsables ne compilent pas** : seules les sessions de tâche compilent (3 au plus). Le responsable qualité utilise l'emplacement de la copie principale.
- **Les règles communes vivent dans `CLAUDE.md` et dans `docs/pilotage/`, jamais seulement dans la mémoire d'une session.** *Pourquoi : la mémoire automatique dépend du dossier de la session, et une session dans un worktree ne la voit pas.*

## 3. Communication et traces

- **Les messages entre sessions ne sont qu'une sonnette.** Ils ne sont pas garantis : un message peut être retenu ou perdu, et rien ne confirme sa lecture. La trace fait foi :
  - **avant Git** : l'expéditeur note la demande, avec son état, dans la section « Demandes envoyées » de son carnet ; le destinataire la reporte dans « Demandes reçues » du sien. Sans réponse à son tour suivant, l'expéditeur relance ;
  - **dès le dépôt GitHub** : toute demande entre rôles est une *issue*, avec une étiquette par rôle (`role:conception`, `role:architecture`, `role:dev`, `role:qualite`, `role:contenus`) et un tableau GitHub Project ; le message ne contient que le lien.
- **Chaque responsable tient un carnet** court, `<rôle>.md`, sur la branche `pilotage` : état, décisions en cours, demandes envoyées et reçues, sessions actives. L'orchestrateur tient le sien, `orchestrateur.md`.
- **Branche `pilotage` (décidé le 30/10)**.
  - **Contenu** : les fichiers vivants de pilotage, c'est-à-dire les carnets, `feuille-de-route.md` et `a-relire.md`. Ce fichier-ci reste sur `main`.
  - **Emplacement** : une branche orpheline, extraite dans le worktree permanent `C:\Dev\Claude\time_switcher-pilotage\`.
  - **Écriture** : chaque session ne modifie que son propre fichier, puis fait `git pull --rebase`, `git commit` et `git push`. Pas de PR, et la CI ne tourne pas sur cette branche.
  - *Pourquoi :*
    - les carnets changent plusieurs fois par jour. Les faire passer par des PR userait les 2 000 minutes de CI et obligerait le responsable développement à fusionner les carnets des autres ;
    - hors de Git, ils n'auraient ni historique ni copie de sauvegarde.
- **Relève d'une session** : quand une session de niveau 1 ou 2 a été résumée deux fois, ou qu'elle s'embrouille, elle met son carnet à jour, puis une session neuve prend le relais à partir du carnet. L'orchestrateur suit la même règle.
- **Un seul rédacteur par fichier.** Avant Git, deux sessions ne modifient jamais le même fichier ; après, chaque branche ne touche que les fichiers de sa tâche.
- **Circulation** :
  - une question de conception va au responsable conception, qui décide dans les documents ; le développement applique ensuite ;
  - une question technique (crates, formats, protocole, sécurité) va au responsable architecture et sécurité ;
  - la qualité envoie les défauts au développement, les failles et les écarts à l'architecture, et les règles à revoir à la conception ;
  - un désaccord entre responsables est tranché par l'orchestrateur.

### 3.1 Remontées vers la conception (décidé)

Une session peut voir un problème de conception sans le reconnaître comme tel. Le risque est qu'elle le règle en silence dans le code, et la conception ne correspondrait plus au jeu réel. Les remontées ne dépendent donc pas du jugement de chacun : elles sont déclenchées par des signaux précis, aidées par des outils et vérifiées à heure fixe.

**1. Signaux qui obligent à remonter.** Toute session qui rencontre l'un de ces signaux ouvre une remontée vers la conception, avec l'étiquette `role:conception` et le type `ecart`. Avant Git, la remontée passait par la section « Demandes envoyées » du carnet ; depuis le dépôt GitHub, c'est une *issue*.

| Signal | Qui le voit d'habitude |
|---|---|
| Une règle écrite est inapplicable, ambiguë ou contredit une autre | Développement |
| Le code a besoin d'une règle de jeu que la conception ne donne pas (un trou) | Développement |
| Une valeur **[à calibrer]** se révèle mauvaise à la mesure | Qualité |
| Un budget de performance, de mémoire ou de temps oblige à simplifier une règle | Développement, qualité |
| Un invariant d'équilibrage échoue, ou un exploit est trouvé | Qualité |
| La simulation produit un comportement que la conception n'avait pas prévu (économie qui s'emballe, habitants absurdes, etc.) | Qualité |
| Une contrainte de sécurité ou d'architecture oblige à changer le jeu | Architecture et sécurité |
| Un asset ou un style impossible à obtenir dans les limites fixées | Contenus |
| Un retour de l'utilisateur après un essai | Qualité |

**2. Aucune règle de jeu inventée en silence.** Une session de code qui a besoin d'une règle absente ne la crée pas comme si elle était définitive. Elle procède ainsi :
- elle prend la valeur la plus prudente, c'est-à-dire la plus simple et la plus facile à changer ;
- elle la marque `// PROVISOIRE(conception #<issue>)` dans le code ;
- elle ouvre la remontée et continue son travail sans attendre.

Le responsable développement refuse toute PR qui ajoute une règle de jeu sans référence à la conception ni marque `PROVISOIRE`.

**3. Traçabilité entre la conception et le code.**
- Chaque système de règles porte en commentaire la section de conception qu'il applique, par exemple `// spec: combat.md §5.4`.
- `cargo xtask spec-map` produit la correspondance dans les deux sens : quel code applique telle section, et quelles sections n'ont pas encore de code.
- Quand une section change, le responsable conception voit le code touché et ouvre la tâche d'adaptation.
- Pour que ces renvois restent valides, la conception **ne renumérote jamais** ses sections : une nouvelle section prend un numéro libre ou un suffixe, par exemple §5.4 bis.
- L'outil est construit en phase 0 par le responsable architecture et sécurité.

**4. Les attendus de conception sont vérifiés en continu.**
- La qualité tient un tableau des valeurs cibles écrites dans la conception, par exemple un taux de réussite au test « joueur ou PNJ » d'au plus 60 %, ou les budgets par personnage.
- Ses campagnes les comparent à la mesure.
- Tout écart ouvre automatiquement une remontée.

**5. Questions systématiques dans les PR.** Le modèle de PR pose deux questions :
- « Cette PR s'écarte-t-elle de la conception, ou comble-t-elle un trou ? »
- « Si oui, quelle remontée ? »

**6. Rendez-vous fixes.**
- Chaque semaine de travail, le responsable conception traite les remontées ouvertes et parcourt les PR fusionnées qui touchent aux règles. Il décide, met à jour les documents et répond.
- À chaque fin de phase, il vérifie la conformité, comme prévu au §5.

**7. Remontées urgentes.** Une remontée qui bloque une tâche, ou qui remet en cause une décision du socle, est signalée tout de suite par message au responsable conception, en plus de la trace écrite. S'il n'a pas répondu à la fin de son tour suivant, l'orchestrateur tranche provisoirement.

## 4. Consommation (décidé)

| Travail | Modèle | *Pourquoi* |
|---|---|---|
| Orchestrateur et responsables | Opus | Jugement, arbitrages, cohérence d'ensemble |
| Tâches du noyau, des chemins sensibles ou qui changent l'architecture | Opus | Une erreur y coûte cher |
| Tâches courantes : tests, documentation, scripts, outils d'assets, corrections simples | Sonnet | Assez bon, bien moins cher |
| Campagnes et analyses en lecture seule | Sonnet | Lire et résumer |
| Recherches dans le code (sous-agents) | Haiku | Rapide et économique |

- Au plus **5 sessions qui travaillent en même temps**, toutes confondues ; les sessions inactives ne comptent pas.
- L'orchestrateur relève la consommation chaque jour de travail. Si elle approche de la limite du forfait, il ralentit : moins de sessions en parallèle et plus de Sonnet.
- Une session éphémère est archivée dès que son travail est fusionné ou rendu.

## 5. Fin de phase

Une phase est terminée quand :
1. le responsable architecture et sécurité a validé l'audit de sécurité et les exigences bloquantes de la phase ;
2. le responsable qualité a validé ses campagnes : déterminisme, performances, invariants d'équilibrage ;
3. le responsable conception a vérifié que ce qui est construit respecte la conception ;
4. pour une phase jouable, le responsable qualité a joué la version et l'a fait jouer par des robots `Joueur`, et ses constats sont traités ou planifiés. La version est mise à la disposition de l'utilisateur ; ses retours, s'il en donne, sont traités comme les autres, mais la phase ne les attend pas.

L'orchestrateur constate ces quatre points dans son carnet et ouvre la phase suivante.

## 6. L'utilisateur ne fait aucune action (décidé, demande de l'utilisateur du 29/10)

**Aucune session ne pose de question à l'utilisateur, ni ne lui demande une action**, qu'il s'agisse d'un feu vert, d'une validation, d'un essai, d'une vérification ou d'un réglage. Chaque session applique ce qu'elle juge le mieux, le marque **(décidé)** avec sa raison et continue.
- **Les feux verts sont donnés par l'orchestrateur** : lancement d'une phase, création du dépôt, envoi sur GitHub, lancement des responsables.
- **Ce qu'aucune session ne peut faire elle-même**, à cause des règles de sécurité de Claude, est contourné par une solution qui s'en passe : payer, créer un compte, saisir un mot de passe, modifier un réglage de sécurité du système. Pour un asset payant, on prend une source libre ; pour un service payant, une offre gratuite ou une solution locale. Si aucun contournement n'existe, l'orchestrateur le note comme contrainte dans son carnet et réorganise le travail autour. Il le signale à l'utilisateur comme une simple information, jamais comme une demande.
- **Rien d'irréversible n'est fait sur ce qui appartient à l'utilisateur hors du projet** : ses autres fichiers, ses comptes, ses autres dépôts.

**Droit de veto, sans obligation** : l'orchestrateur tient `a-relire.md` (branche `pilotage`, §3), une liste courte des décisions importantes récentes, une ligne et un lien chacune. L'utilisateur n'a pas à la lire ; s'il revient sur une décision, le responsable concerné la reprend.

## 7. Dépendance à la machine

Les sessions s'arrêtent quand l'ordinateur se met en veille.
- Pour une campagne longue, l'orchestrateur demande le maintien en éveil.
- Les tâches de nuit passent par la CI GitHub.
- Une tâche qui doit tourner sans l'ordinateur peut aller dans une session en cloud, une fois le dépôt sur GitHub.
