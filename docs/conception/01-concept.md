# Concept et temps

> Fichier du **socle** : modifié uniquement par la session principale (voir [README](README.md)).

## 1. Concept

Un **monde ouvert contemporain et fictif**, en **solo** et **en ligne**, où chacun vit librement.

Pour jouer, un joueur peut :

- **jouer son personnage persistant**, qui lui est réservé ;
- ou **prendre le contrôle d'un personnage jouable** du monde, s'il remplit les conditions d'accès ([04-joueur.md](04-joueur.md), §4).

**Public adulte** : le jeu vise une classification 18 ans ; ses limites de contenu sont fixées dans [domaines/combat.md](domaines/combat.md), §12.

**Quasiment tous les personnages du monde sont jouables**, sauf ceux qui sont trop puissants ou exclus pour d'autres raisons ([02-personnages.md](02-personnages.md), §3.4).

### 1.1 Le monde

- L'**agglomération d'Orvenne**, ville moyenne européenne francophone, traversée par un fleuve, **l'Ostrelle** : une ville-centre et deux communes (Hautevigne, aisée, en amont ; Forges-sur-Ostrelle, populaire et industrielle, en aval). Le détail est dans [domaines/ville.md](domaines/ville.md).
- **10 000 habitants**, extensible, tous des personnages complets : **aucun figurant**. Toute personne visible est un vrai habitant.
- Tous les mondes partent de **la même carte initiale**. Les parties solo partent de la **population initiale solo**, livrée avec le jeu ; les serveurs en ligne partent de la **population initiale en ligne**, produite par la même chaîne mais **jamais distribuée** (décidé). À sa création, chaque monde tire avec sa propre graine sa **couche cachée** (détenteurs des savoirs opaques, caches et leur contenu, alarmes, informateurs, policiers corruptibles, secrets des intrigues de départ), puis fait son propre rodage. Les mondes évoluent ensuite chacun de leur côté ; les dates sont relatives au jour de départ du monde ([domaines/population.md](domaines/population.md), §9 ; [revues/securite.md](revues/securite.md), S2, S3 ; [revues/equilibrage.md](revues/equilibrage.md), E25). *Pourquoi : tout ce qui est livré au client est lu par les joueurs ; ce qu'on apprend en solo ne doit révéler les secrets d'aucun serveur.*
- « Aucun figurant » veut dire **aucune personne inventée visible** : les proches à l'extérieur ne viennent jamais en ville, les nouveaux venus arrivent pour s'installer. Une **demande extérieure abstraite** reste possible sans présence visible (chambres d'hôtel réservées et fermées, commandes de l'usine), comme les trains de marchandises en transit ([domaines/economie.md](domaines/economie.md), §4.4). La règle vaut **aussi au son** (décidé) : toute voix, tout pas, tout moteur entendu a une source réelle perçue, et le fond sonore reste impersonnel (vent, pluie, oiseaux, rumeur lointaine) ([domaines/direction-artistique.md](domaines/direction-artistique.md), §0.4, §5.1).
- **Une fiction sans référence réelle** (décidé) : aucune marque, aucun insigne d'État et aucune personne réels dans le monde ; enseignes, uniformes, véhicules et médias sont fictifs. **Aucun animal visible** dans la première version : les animaux ne sont pas simulés, et les oiseaux n'existent qu'au son ([domaines/direction-artistique.md](domaines/direction-artistique.md), §1.4, §3.9). *Pourquoi : le monde est fictif, et tout ce qui est visible doit venir du monde simulé, identique pour tous.*
- « Extensible » veut dire que la population suit la capacité d'accueil de la ville, dans la limite d'un plafond de calcul par serveur ([domaines/population.md](domaines/population.md), §7.6). Le monde a de vraies **naissances** et de vrais **décès**.
- **Économie fermée** : la monnaie est l'**écu** ; l'argent circule entre comptes, et l'extérieur n'est représenté que par quelques comptes abstraits. Le jeu ne crée jamais d'argent ([domaines/economie.md](domaines/economie.md), §1, §2.2, §10.1).
- **Le monde ne connaît pas les joueurs** : rien dans le monde ne signale qu'un personnage est piloté par un joueur. Le pilote n'est jamais affiché, sauf entre amis mutuels qui y consentent ([domaines/social.md](domaines/social.md), §0 et §9.1).

## 2. Le temps

| Mode | Écoulement du temps |
|---|---|
| **Solo** | **Une heure de jeu = une heure réelle**, comme en ligne. Le joueur peut en plus **faire avancer le temps** comme il le veut (attendre, dormir, sauter des heures ou des jours). Le monde est alors simulé en accéléré. Quand le jeu est fermé, le monde est **en pause** : le joueur reprend exactement là où il s'était arrêté (décidé, [domaines/ville.md](domaines/ville.md), §2.5). Chaque monde solo a **une seule chronologie** (journal écrit en continu : après une fermeture brutale, le jeu reprend exactement au même instant ; les sauvegardes de secours ne servent qu'en cas de corruption, [revues/equilibrage.md](revues/equilibrage.md), E32) ; le joueur peut avoir **plusieurs mondes solo**, qui partagent son niveau solo. Consulter le téléphone ou le plan ne met pas le monde en pause, sauf option d'accessibilité ([domaines/interface.md](domaines/interface.md), §9) |
| **En ligne** | **Temps réel** : une journée de jeu dure 24 heures réelles |

**Pas de voyage rapide en ligne** : on se déplace à pied, en véhicule ou en transport en commun, et le temps de trajet fait partie de la vie du personnage. Pour jouer ailleurs, le joueur peut aussi incarner un personnage qui s'y trouve déjà. En solo, faire avancer le temps pendant un trajet en tient lieu.

Conséquences en ligne :

- **Pas de convalescence en ligne** : la mort du personnage persistant se traduit par une baisse immédiate de ses attributs, équivalente à la convalescence théorique ([03-vie-synchronisation.md](03-vie-synchronisation.md), §3).
- **Le sommeil du personnage persistant n'est pas géré** : il disparaît à chaque déconnexion. La fatigue liée à l'effort s'applique toujours.
- **Pas de peine de prison en temps réel pour le personnage persistant** (décidé) : comme pour la convalescence, une condamnation se traduit immédiatement par des **conséquences équivalentes** (casier judiciaire, amende ou saisie de biens, perte d'emploi, réputation, éventuelle interdiction de séjour dans certains lieux), puis le personnage est libre. Raison : immobiliser un personnage persistant des mois réels reviendrait à interdire au joueur de le jouer, ce que la règle de convalescence évite déjà. Les personnages jouables purgent leur peine normalement (« à l'extérieur »), et en solo la peine s'écoule avec le temps de jeu. Amendes et saisies passent par le mécanisme économique de [domaines/economie.md](domaines/economie.md), §7.8 ; le reste relève du domaine combat, police et justice. En pratique ([domaines/combat.md](domaines/combat.md), §7.2 et §9.5) : garde à vue accélérée, plaider-coupable ou audience, **jugement par défaut** après 14 jours réels sans se présenter ; chaque mois de prison est converti en amende équivalente, désentraînement plafonné, perte d'emploi et des postes de groupe, interdictions et délai d'épreuve en temps réel. Une peine convertie de **6 mois ou plus** s'accompagne d'une **surveillance électronique** : pendant min(M, 3) mois réels, quand le persistant est connecté, il doit être dans son lieu de retrait de 22 h à 6 h et déclarer ses sorties de l'agglomération. Le jugement par défaut se fait sans aménagement favorable et avec mandat, et l'amende équivalente tient compte du patrimoine connu ([revues/equilibrage.md](revues/equilibrage.md), E22). Dans le monde, la peine est perçue comme aménagée. **En solo**, le personnage persistant est réellement détenu à l'extérieur ; le joueur incarne d'autres personnages ou fait avancer le temps.
- **Un serveur par fuseau horaire** (Europe, Amériques, Asie…) : chaque serveur suit l'heure de son fuseau, pour que chacun joue à des heures cohérentes avec sa propre journée. Chaque serveur a donc son propre monde persistant.

## 3. Météo et saisons

La météo et les saisons font partie du socle, parce qu'elles touchent tous les domaines.

- **Saisons** : elles suivent le **calendrier** du monde. En ligne, c'est le calendrier réel du fuseau du serveur ; en solo, le calendrier avance avec le temps de jeu, accélérations comprises.
- **Climat** : tempéré océanique, cohérent avec une ville européenne (hivers frais et humides, étés doux, épisodes de chaleur ou de gel).
- **Météo** : générée jour après jour selon la saison (pluie, vent, brouillard, neige, canicule, orage), identique pour tout le monde dans un même monde.

Effets (le détail relève de chaque domaine) :

| Domaine | Effets |
|---|---|
| Personnages | Confort (froid, chaleur), santé (rhume, coup de chaleur, hypothermie), fatigue, humeur ; tenue adaptée ou non |
| IA des PNJ | Choix des activités et des trajets (moins de sorties sous la pluie, terrasses l'été) |
| Ville | Éclairage (jours courts en hiver), état des routes (verglas, inondation du fleuve possible), végétation |
| Déplacements | Vitesse, visibilité, accidents, fréquentation des transports |
| Économie | Activités saisonnières, prix de l'énergie et de certains produits |
| Combat | Visibilité, adhérence, bruit couvert par la pluie |
