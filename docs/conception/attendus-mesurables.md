# Attendus mesurables

> Fichier tenu par le responsable conception. Créé le 30/10/2026, à la demande de l'orchestrateur ([../pilotage/organisation.md](../pilotage/organisation.md), §3.1).
> Il rassemble les **valeurs cibles** écrites dans la conception, pour que le responsable qualité et essais les suive. La source fait foi : en cas d'écart, c'est le fichier cité qui a raison, et ce tableau doit être corrigé.
> Les valeurs marquées **[à mesurer]** ou **[à calibrer]** dans leur source sont des objectifs de départ. Un objectif manqué remonte au responsable conception comme une remontée `role:design` de type `deviation` ; s'il touche à la technique, il va au responsable architecture et sécurité.
> Règle 6 des « Règles pour les sessions de domaine » du [README](README.md) : les lignes ne sont jamais renumérotées. Une nouvelle ligne prend le numéro suivant, et une ligne retirée garde son numéro avec la mention « supprimée ». Une famille de bornes de même source partage un numéro, et chaque borne y prend une lettre (M39a, M39b…).

## 1. Simulation et performances

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M1 | Reproductibilité par graine : un même binaire et une même plateforme donnent la même empreinte, sur 1 fil comme sur N fils | Identique au bit | [05](05-architecture.md) §2.10 | Test de déterminisme (`cargo xtask check`) | 0 |
| M2 | Monde-jouet de 1 000 entités, un an simulé en saut de temps | Même empreinte à chaque exécution | 05 §5, phase 0 | Critère de fin de phase 0 | 0 |
| M3 | Un an simulé pour 10 000 habitants | Moins de 5 min **[à mesurer]** | 05 §2.22 | `ts-sim`, chronométré | 1 |
| M4 | Coût de l'esprit des PNJ en statistique | ≈ 500 opérations par personnage et par jour **[à mesurer]** | 05 §2.13 ; [ia-pnj](domaines/ia-pnj.md) §13.15 | Compteurs d'unités de travail | 1 |
| M5 | Pas de la simulation complète | 20 Hz, sans ralentir le temps en ligne | 05 §2.22 | Indicateurs du serveur | 1 |
| M6 | Équivalence entre niveaux de détail (même règle, mêmes paramètres) | Test d'équivalence vert pour chaque règle, tenue comprise | 05 §2.2, §2.6 | Tests automatiques | 1 |

## 2. Mémoire

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M7 | Mémoire d'un monde de 10 000 habitants | 0,55 à 0,85 Go | Clôture de l'architecture, S-1 | Assertions de taille des composants à la compilation, puis mesure | 0 |
| M8 | Composants de l'esprit | ≈ 2 Ko par personnage, ≈ 20 Mo en tout **[à mesurer]** | ia-pnj §13 | Idem | 1 |
| M9 | Génome d'apparence ; tenue portée ; dates par relation | ≈ 64 o ; ≈ 30 o ; 4 × 2 o, soit ≈ 52 Mo (`SimDay` signé sur 16 bits, ±89 ans autour du jour de départ, saturé au-delà) | 05 §2.2, §2.5 ; clôture de l'architecture, T2-6 ; [README](README.md), A12 | Idem | 1 |

## 3. Génération

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M10 | Création d'un monde solo | ≤ 10 s sur 4 cœurs, plafond 20 s, dont ≤ 5 s de rodage propre **[à mesurer]** | 05 §2.14 ; [population](domaines/population.md) §1.6 | Chronométrage `ts-worldbuild` | 1 |
| M11 | Chaîne complète d'une famille | ≤ 2 min sur 8 cœurs, rodage de famille ≤ 60 s **[à mesurer]** | 05 §2.14 ; [revues/generation.md](revues/generation.md) | Idem | 5 |
| M12 | Mémoire de la génération | ≤ 4 Go | population §9.4 | Mesure | 5 |
| M13 | Fidélité de la population aux tables cibles | Écart aux marges ≤ 3 % ; ≥ 80 % de chaque rôle dans sa fourchette de palier | population §1.3 | Validateurs de `ts-worldbuild` | 1 |
| M14 | Réseau social de départ | Plages cibles du [social](domaines/social.md) §10.2, dont ≥ 98 % des adultes et ≥ 90 % des liens au-dessus de « connaissance » | social §10.2 ; population §5.3 | Validateur du réseau, puis `ts-sim` au fil des années | 1 |
| M15 | Robustesse des plans directeurs | 95 % de réussite sur 32 × 32 graines | [ville](domaines/ville.md) §4.6 (P37) | Campagne de graines | 5 |
| M37 | Part serveur tirée par monde (W2) : pose réelle des caméras, coffres, caches et alarmes | ≤ 1/3 des candidats de chaque sorte, par secteur **[à calibrer]** | [README](README.md), A13 (C2) ; [revues/generation.md](revues/generation.md) V12, W2 | Validateur bloquant de la création du monde | 1 |

## 4. Indistinguabilité et esprit des PNJ

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M16 | Tests à l'aveugle « joueur ou PNJ » | ≤ 60 % de bonnes réponses, **par profil d'adversaire** (six profils) | 05, en-tête (A8, clôture) ; ia-pnj §14.3 | Sessions évaluatrices face à des sessions joueuses | 3 |
| M17 | Discriminateur hors ligne | AUC ≤ 0,70 **[à calibrer]** ; en production, mesurée chaque mois sur les traces étiquetées par le serveur | ia-pnj §14.3 | Discriminateur | 3 |
| M18 | Test « fidèle ou non » (les signatures face au jugement) | AUC ≥ 0,85 | ia-pnj §14 ; [revues/signatures.md](revues/signatures.md) | Sessions évaluatrices | 5 |
| M19 | Qualité du panel de juges | Un juge doit placer ≥ 90 % des ancres ; test-retest à ordre inversé ≥ 90 % ; accord mesuré entre consignes et modèles différents | 05, en-tête ; ia-pnj §13.13, §14.3 | Contrôles du panel | 3 |
| M20 | Aucune différence de protocole selon le pilote (message, champ, cadence, délai) | Test automatique vert | 05 §2.5 | Test de vue | 3 |

## 5. Signatures comportementales

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M21 | Part des actes écrits à la main | Annotations 10 à 15 %, dérogations ≤ 5 % (alerte de la CI à 5 %) | [04](04-joueur.md) §2 ; [revues/signatures-contenu.md](revues/signatures-contenu.md) §5 | Rapport `ts-data signatures` | 1 |
| M22 | Ancres, échelles, planchers | Les 46 ancres dans leur bande pour les 12 archétypes ; échelles monotones ; planchers qui donnent bien la rupture | signatures-contenu | `ts-data check` et invariants `INV-SIG-*` | 1 |
| M23 | Accord du modèle avec le choix majoritaire des juges | ≥ 85 % là où les juges s'accordent à plus de 80 % | revues/signatures.md §4 | Calibrage | 5 |

## 6. Vie privée et sécurité du rendu

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M24 | Test de parenté, à la précision « visage », naissances et fratries comprises | AUC ≤ 0,70 **[à calibrer]** | [direction-artistique](domaines/direction-artistique.md) §3.4 ; population §1.3 | Classificateur sur les apparences perçues | 1 |
| M25 | Test de réidentification d'une personne masquée ou en silhouette | AUC ≤ 0,70 **[à calibrer]** | 05 §2.5 | Idem | 1 |
| M26 | Vue identique quand l'identification échoue | Écrans, vue et journal de conversation identiques | 05 §2.5 | Test automatique | 1 |
| M27 | Aucune valeur envoyée au client n'est une clé, une graine de flux ou un état de l'aléa du cœur | Test automatique vert | 05, en-tête (clôture, C3) | Test | 0 |
| M28 | Profil `public` sans `dev-commands`, `dev-introspection`, `external-pilot`, pause ni banc de pilotage | Erreur de compilation, artefact vérifié, refus de démarrer | 05 §2.3, §5 | CI et vérification de l'artefact | 0 |

## 7. Méta-jeu, équilibrage et modération

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M29 | Rythme de progression | ≈ 100 XP par heure au palier 0 bien joué ; tâches de travail ≤ 30 XP par heure réelle ; niveau 80 en ≈ 300 à 400 h au meilleur palier | 04 §1.1 | Joueurs synthétiques (`ts_meta`, `ts-sim`) | 2 |
| M30 | Invariants d'équilibrage et catalogue d'exploits | Tous verts dans la CI, y compris « sonde de la classe cachée » et « relance puis abandon » | 05 §2.24 ; clôture de l'équilibrage, P2 | CI et agents adverses | 2 |
| M31 | Constance de la modération par session | ≥ 90 % sur l'échantillon rejugé chaque mois **[à calibrer]** | social §9.4 | Rejugement mensuel | 5 |
| M38 | Exploits face à un adversaire qui connaît la formule | Invariants d'équilibrage et des signatures verts face à des agents adverses qui reçoivent la formule et les paramètres exacts | [04](04-joueur.md) §2 ; [README](README.md), A14 | CI et agents adverses | 2 |

## 8. Contenus et client

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M32 | Mémoire vidéo | ≤ 2,5 Go en 2.5D, ≤ 6 Go en 3D en qualité maximale **[à calibrer]** | direction-artistique §6 | Scène étalon | 3 |
| M33 | Poids des assets | LFS ≤ 4 Go en 2.5D, historique compris ; texture ≤ 16 Mo ; morceau de musique ≤ 30 Mo | direction-artistique §9.3 | `cargo xtask assets check` | 3 |
| M34 | Accessibilité visuelle | ≤ 3 éclats par seconde, toujours | direction-artistique §4.5 ; [interface](domaines/interface.md) §11.1 | Contrôle des assets et des effets | 3 |
| M35 | Couverture des animations | Au moins une animation par famille de gestes | 05 §2.2 | `cargo xtask assets check` | 3 |

## 9. Outillage

| N° | Attendu | Cible | Source | Mesure | Dès la phase |
|---|---|---|---|---|---|
| M36 | Minutes de CI | ≤ 3 000 par mois (GitHub Pro) ; alerte et mode minimal au-delà de 80 %, soit 2 400 | 05, en-tête ; clôture de l'architecture, S-6 ; orchestrateur, passage à GitHub Pro | Relevé mensuel | 0 |

## 10. Bornes et formats des données (décidé le 01/11)

Valeurs de départ pour les remontées [#43](https://github.com/CreatorInstant548/time_switcher/issues/43), [#44](https://github.com/CreatorInstant548/time_switcher/issues/44) et [#45](https://github.com/CreatorInstant548/time_switcher/issues/45), avec leur **méthode de calibrage**. Une borne n'est pas une cible : c'est un plafond de sécurité, choisi assez large pour ne jamais gêner un contenu ou un monde légitime. Chaque borne a sa propre lettre (M39a, M39b…), pour être relevée ou retirée seule.

**Méthode commune** (décidé) : chaque borne a sa **mesure** (outil cité ci-dessous), qui rapporte la part utilisée de la borne. La CI **alerte à 50 %**. On relève alors la borne par une PR de conception, à **4 fois** la plus grande valeur mesurée, arrondie à la puissance de 2 supérieure. Une borne ne baisse jamais en dessous de 4 fois le maximum mesuré. *Pourquoi : une alerte à mi-chemin laisse le temps de décider ; une marge de 4 absorbe une phase de contenus sans rouvrir la question à chaque PR.* Les bornes du décodage des instantanés (§10.3) ont leur propre calibrage, lié à la population.

### 10.1 Chargement des contenus (`ts_data`, #43)

| N° | Borne | Valeur | Raison |
|---|---|---|---|
| M39a | Taille d'un fichier `.ron` | 1 Mio | Un fichier écrit à la main reste très en dessous ; les grandes tables (prénoms, noms, taux) se découpent par thème |
| M39b | Taille totale d'un paquet | 64 Mio | Borne la mémoire d'un chargement |
| M39c | Nombre de fichiers d'un paquet | 4 096 | Un fichier par nature, par rôle ou par famille de pièces |
| M39d | Nombre de fiches d'un paquet | 65 536 | Catalogues prévus jusqu'en phase 3 : quelques centaines d'actes et de rôles, quelques milliers de pièces de garde-robe, de recettes et de lignes de tables |
| M39e | Profondeur d'imbrication RON, en **niveaux de récursion du lecteur RON** (`with_recursion_limit`) | 32 (relevé de 16 le 02/11, arbitrage A18 du README) | Les fiches s'imbriquent sur 3 à 6 niveaux visibles ; un niveau visible en coûte environ 2 au lecteur (une fiche à 3 niveaux en mesure 7, une à 6 en mesurerait 13, soit 41 %). Une seule unité, celle que le chargeur applique et que `ts-data check` rapporte |
| M39f | Profondeur des sous-dossiers | 8 | `data/content/<domaine>/<famille>/…` en utilise 3 ou 4 |

Mesure : `ts-data check` rapporte, pour chaque borne, la plus grande valeur du paquet. Dès la phase 0.

### 10.2 Identifiants et noms des fiches (`ts_data`, #44, #137)

| N° | Règle | Valeur |
|---|---|---|
| M40a | Grammaire d'un identifiant | Segments `[a-z][a-z0-9_]*` séparés par un point, **au moins deux** ; 1 à 64 octets ; en anglais, comme le code |
| M40b | Premier segment | **Espace de noms de la sorte de fiche** : le nom de la sorte en `snake_case` (`Activity` donne `activity`, `SoundEvent` donne `sound_event`). La liste est celle des sortes de fiches de `ts_data` : aujourd'hui `activity` et `item` ; à venir, par exemple `role`, `recipe`, `garment` ou `table`. Elle s'allonge quand une PR de conception ajoute une sorte. L'espace de noms suit la **sorte**, jamais le dossier : un dossier peut regrouper plusieurs sortes, et les fichiers peuvent rester à plat (`data/content/items.ron`) |
| M40c | Unicité | Unique dans tout le paquet, toutes sortes confondues |
| M40d | Stabilité | Un identifiant n'est **jamais renommé ni réutilisé**. Le registre est **un seul fichier**, `data/content/ids.lock`, en ajout seul, pour toutes les sortes : c'est celui que prévoit [direction-artistique](domaines/direction-artistique.md) pour les pièces de vêtement et les familles. Chaque ligne donne un identifiant et son état, actif ou retiré. `ts-data check` refuse un identifiant du paquet absent du registre, un identifiant retiré qui réapparaît, et une ligne supprimée ou modifiée autrement que par le passage d'actif à retiré. Le registre ne contient **que des identifiants de fiches** (décidé le 02/11, arbitrage A17 du README) : une ligne active sans fiche du même identifiant est refusée ; une ligne retirée n'a plus de fiche. Ce qui doit garder un identifiant stable sans être une pièce ou un acte, comme une famille de vêtements, de gestes ou d'activités ([direction-artistique](domaines/direction-artistique.md) §3.6 et §3.10), est une fiche de sa propre sorte (par exemple `garment_family.top` ou `gesture_family.walk`), avec son nom affiché. Il en va de même des valeurs des listes fermées à identifiant stable rangées avec les données ([ia-pnj](domaines/ia-pnj.md) §2.6) : gestes, postures et vocalisations sont des fiches de leurs sortes, comme les activités le sont déjà. C'est l'équivalent, pour les fiches, de la règle 6 des « Règles pour les sessions de domaine » du [README](README.md), qui interdit de renuméroter les sections |
| M40e | Nom affiché | 1 à 256 octets d'UTF-8, en **forme normalisée NFC**, sans espace en tête ou en fin ; en français, comme toute la fiction. **Caractères refusés** (décidé le 02/11, A17) : contrôles (Cc), caractères de format (Cf : inversions du sens d'écriture, caractères de largeur nulle…), usage privé (Co), points de code non attribués (Cn), séparateurs de ligne et de paragraphe (Zl, Zp), et tout espace autre que l'espace simple (U+0020), l'espace insécable (U+00A0) et l'espace fine insécable (U+202F) de la typographie française ; au plus 2 marques combinantes de suite (catégorie M : Mn, Mc, Me, sélecteurs de variante compris). La table des catégories est une dépendance épinglée, à la même version d'Unicode que celle de la normalisation NFC : refuser Cn garantit alors que la forme NFC d'un nom accepté ne changera pas |

*Pourquoi :*
- les identifiants entrent dans l'empreinte du paquet, et l'espace de noms en tête évite les collisions entre domaines sans préfixe numérique ;
- lier l'espace de noms à la sorte plutôt qu'au dossier laisse chaque domaine ranger ses fichiers comme il veut, sans que le contrôle dépende de ce rangement ;
- un registre unique évite que deux fichiers gardent le même invariant ;
- un registre limité aux fiches garde une seule règle, sans identifiant orphelin, et toute fiche a son nom affiché ;
- la forme NFC garantit qu'un même nom donne une même empreinte, quel que soit l'éditeur ;
- les caractères invisibles permettent de faire passer un nom pour un autre à l'écran ; le français n'en a pas besoin, sauf ses deux espaces insécables, et deux marques combinantes suffisent après NFC ;
- la longueur d'affichage relève de l'interface : un nom long est abrégé à l'écran, jamais refusé au chargement.

Mesure : `ts-data check`, dès la phase 0. Le registre M40d naît avec la PR qui ajoute son contrôle, en phase 0 ; il reprend alors tous les identifiants existants, à l'état actif.

### 10.3 Décodage des instantanés (`ts_persist`, #45)

Ces bornes portent sur des **octets et des nombres d'éléments en entrée** du décodeur, plus le pic de mémoire du décodage (M41h) et la mémoire de l'état décodé par le noyau (M41i). Les bornes des valeurs décodées (compteurs, dates, `DayOrigin`, durée du tick, dans `ts_kernel`) sont fixées par construction dans le code et ne relèvent pas de M41.

| N° | Borne | Valeur de départ |
|---|---|---|
| M41a | Enveloppe stockée (fichier entier, compressé) | 256 Mio |
| M41b | Total décompressé, toutes parts | 1 Gio |
| M41c | Taille décompressée d'une part | 512 Mio |
| M41d | Manifeste | 4 Mio |
| M41e | Nombre de parts | 65 536 |
| M41f | Nom d'une part | 128 octets |
| M41g | Fenêtre `zstd` | 8 Mio |
| M41h | **Pic de mémoire du décodage entier** (tampons d'entrée, parts décompressées et état décodé par le noyau, au plus haut) | ≤ 2 Gio au départ ; après calibrage, ≤ 3 fois la taille en mémoire du monde au plafond **[à calibrer]** |
| M41i | **Mémoire de l'état décodé** par le décodeur canonique du noyau (`ts_kernel`), en-tête compris (noms des greffons) : octets demandés à l'allocateur, comptés par un majorant, croissance des collections comprise (voir ci-dessous) ; les octets canoniques de la part d'état n'y sont pas comptés, ils font partie de la mémoire déjà tenue | 512 Mio au départ (décidé le 02/11, arbitrage A16 du README) **[à calibrer]** |

Les valeurs de départ de M41a à M41g sont celles du code. Elles laissent de la marge à un monde de 10 000 habitants, sur l'estimation haute de [revues/architecture.md](revues/architecture.md) §12.3 (120 Mo compressés et 400 Mo en mémoire, cette dernière prise comme majorant du décompressé) : l'enveloppe tient jusqu'à ≈ 21 000 habitants, le total décompressé jusqu'à ≈ 26 000. Avec ces bornes, l'étape des parts (enveloppe, parts décompressées, tampon de la part en cours, fenêtre `zstd`, manifeste) est majorée par 1 957 429 248 octets (≈ 1,82 Gio, `Limits::decode_peak_bound`), et non 1,5 Gio comme l'estimait [#50](https://github.com/CreatorInstant548/time_switcher/issues/50), m5.

**Termes de l'étape des parts** (décidé le 02/11, arbitrage A19 du README, remontée [#150](https://github.com/CreatorInstant548/time_switcher/issues/150)) : la mémoire du décodeur `zstd` dépend des **blocs forgés**, pas des trames ordinaires. Quelques octets d'entrée peuvent lui faire prendre bien plus qu'un fichier normal. Chaque terme du majorant se justifie donc sur le pire bloc valide, jamais sur une mesure de fichiers normaux. Les valeurs de #150 sont confirmées : 3 fois (fenêtre + 1,25 Mio) pour la fenêtre, l'ancien et le nouveau tampon étant tenus ensemble pendant qu'il grandit, et 6 Mio pour le reste du décodeur. Les termes internes du majorant de l'étape des parts appartiennent au code (`Limits::decode_peak_bound` fait foi pour le chiffre ci-dessus) : les corriger ne remonte pas à la conception tant que le majorant reste sous M41h et que la contrainte de l'étape du noyau tient, puisque le décodeur `zstd` est déjà libéré à cette étape. Un terme que la mesure ne voit pas, comme l'ancien bloc d'un `Vec` qui grandit, repose sur une analyse écrite dans le code. *Pourquoi : M41h est la promesse faite à l'hôte ; le détail du décodeur peut changer avec une mise à jour de la dépendance, sans rouvrir une décision de jeu.*

**Étape du noyau** (décidé le 02/11, arbitrage A16 du README, remontée [#125](https://github.com/CreatorInstant548/time_switcher/issues/125)) : la taille en octets d'une part ne borne pas la mémoire de l'état décodé, car l'amplification dépend des types de l'état (×3,53 mesuré sur des composants d'un octet, octets canoniques compris, bien plus pour un `Vec<Option<Large>>`). Le décodeur canonique du noyau tient donc son propre budget M41i :
- chaque allocation est **imputée avant d'être faite**, et l'instantané est refusé dès que le total dépasserait M41i ;
- une collection qui croît impute sa nouvelle capacité avant chaque réallocation, l'ancienne restant comptée tant qu'elle est tenue ; rien n'est réservé d'après une longueur lue dans l'entrée (pas de `with_capacity`, règle déjà suivie par le décodeur canonique) ;
- la mesure est celle de l'allocateur, comme pour l'étape des parts (`Limits::decode_peak_bound` et l'allocateur plafonné des tests de `ts_persist`).

M41h couvre alors **tout** le décodage : la mémoire encore tenue quand l'étape du noyau commence, plus M41i, doit rester sous M41h, et le code le vérifie à la compilation comme pour l'étape des parts. Les valeurs de départ y satisfont : à cette étape, l'enveloppe (M41a, 256 Mio, tenue par l'appelant) et les parts décompressées (au plus M41b, 1 Gio) sont tenues, les tampons de décompression déjà libérés ; avec M41i, cela fait 1 792 Mio, sous 2 Gio. Les petits frais fixes de cette étape (vecteur des parts et leurs noms, dispositions des types de l'état) sont comptés dans la vérification faite à la compilation. La mémoire de la reconstruction du monde à partir de l'état décodé (`Sim::restore`) est hors de M41h : elle croît avec l'état décodé, donc reste bornée indirectement par M41i, et relève de la mémoire de l'hôte, dimensionnée sur la taille en mémoire du monde au plafond. *Pourquoi : seul le décodeur connaît la taille en mémoire de ce qu'il construit ; une borne plus petite sur la part d'état devrait supposer un pire type d'état, que les règles peuvent toujours dépasser. 512 Mio est une valeur de départ prudente : la forme décodée n'a pas la taille du monde en mémoire (chaque valeur de composant y devient un identifiant et une boîte, 24 octets plus une allocation), et la mesure de la phase 1 dira si elle suffit pour 10 000 habitants.*

**Calibrage** (décidé) :
- **Deux mesures distinctes**, en phase 1, sur un monde réel au **plafond de calcul** fixé pour un serveur ([population](domaines/population.md) §7.6) :
  - la **taille en octets** de l'instantané : enveloppe compressée, total décompressé (forme canonique), plus grande part et manifeste ;
  - la **taille en mémoire après décodage**, la **mémoire de l'état décodé** (octets demandés à l'allocateur par le décodeur du noyau, pour M41i) et le **pic de mémoire** du décodage. Elles donnent le **taux d'amplification** (un `Option<Large>` à `None` tient en un octet d'entrée) et dimensionnent la mémoire de l'hôte. Les entrées du fuzzing qui amplifient le plus servent à vérifier ce taux.
- **Bornes d'octets (M41a à M41d)** : chacune vaut 2 fois sa propre mesure en octets au plafond, arrondie à la puissance de 2 supérieure. Elles croissent donc avec la population. M41e à M41g ne changent que si le découpage en parts ou la compression change.
- **Mémoire de l'état décodé (M41i)** : 2 fois la mémoire de l'état décodé mesurée au plafond (octets demandés à l'allocateur), arrondie à la puissance de 2 supérieure.
- **Pic de mémoire (M41h)** : il doit rester sous 3 fois la taille en mémoire du monde au plafond, sur le plus gros instantané attendu comme sur les pires entrées du fuzzing, et la contrainte de l'étape du noyau doit tenir. S'il dépasse, ou si les règles de calibrage de M41a à M41d et de M41i donnent une somme trop grande, on applique **un seul ordre de remèdes** :
  1. libérer avant l'étape du noyau ce qui n'est plus lu (parts déjà consommées, enveloppe si l'hôte n'en a plus besoin) ;
  2. changer le décodage (décoder part par part, libérer chaque tampon décompressé dès qu'il est lu) ;
  3. réduire la marge de M41i sous 2 fois la mesure, sans descendre sous 1,25 fois ;
  4. réduire de même la marge des bornes d'octets M41a à M41d, sans descendre sous 1,25 fois ;
  5. en dernier recours seulement, relever M41h par une PR de conception.

  En phase 0, les valeurs de départ de M41a à M41g ne changent pas : elles ne bougent qu'au calibrage de la phase 1. *Pourquoi : un petit fichier ne doit jamais pouvoir réclamer plusieurs gigaoctets à un serveur ou à la machine d'un joueur.*
- **Mêmes bornes en solo et en ligne.** Les mondes y ont la même taille, et une sauvegarde solo reçue d'un autre joueur doit être décodée avec la même prudence (S24). Un hôte peut toujours passer des bornes plus strictes.

Mesure : `ts-sim` (tailles des instantanés, mémoire de l'état décodé et pic de mémoire au plafond) et fuzzing de `ts_persist`. Valeurs de départ dès la phase 0, calibrage en phase 1.
