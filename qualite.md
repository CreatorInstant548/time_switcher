# Carnet du responsable qualité et essais

> Tenu par cette session seule, lancée par l'orchestrateur le 01/11/2026, en fin de phase 0. Règles : [organisation.md](../time_switcher/docs/pilotage/organisation.md) §1, §3, §3.1 et §5.
> Indépendant du développement : je ne corrige jamais le code. Les défauts vont en `role:dev`, les failles en `role:architecture`, les règles à revoir en `role:design` avec `deviation`.
> Mis à jour le 02/11/2026.

## 1. État

| Point de §5 | État |
|---|---|
| 2. Campagnes : déterminisme | **Validé** le 01/11 (§3.1), puis de nouveau le 02/11 après le correctif de #67 (§3.2) |
| 2. Campagnes : performances | **Validé** le 02/11 (§4.2) : tous les budgets de la phase 0 sont tenus, et [#67](https://github.com/CreatorInstant548/time_switcher/issues/67) est corrigée (PR #92). Avec 4 ou 8 fils, l'exécution n'est plus plus lente qu'avec 1 fil |
| 2. Campagnes : invariants d'équilibrage | Sans objet en phase 0 : [equilibrage.md](../time_switcher/docs/conception/revues/equilibrage.md) §5.9 ne prévoit rien pour cette phase |
| 4. Version jouable | Sans objet : la phase 0 n'est pas jouable (constaté) |

**Verdict du 02/11 : point 2 de §5 validé.**
- Les campagnes ont été refaites sur `main` à `1ba1406`, après la fusion de la PR #92 :
  - le déterminisme est vert sur 25 processus et 605 allers-retours d'instantané, sans aucun écart avec le 01/11 ;
  - en multi-fils, l'exécution va maintenant aussi vite qu'avec 1 fil, ou un peu plus vite.
- La condition posée le 01/11 est remplie (ci-dessous).
- **Restent ouvertes, hors de mon verdict** : l'issue [#94](https://github.com/CreatorInstant548/time_switcher/issues/94), le calibrage des seuils de l'exécuteur, prévu en phase 1 (mesures ajoutées), et l'issue [#68](https://github.com/CreatorInstant548/time_switcher/issues/68), des durcissements mineurs des tests de `ts_toy`. Toutes deux sont en `role:dev`.
  - Depuis la règle du 01/11 (organisation §5 point 5), toute *issue* ouverte doit être résolue avant de clore une phase ; c'est à l'orchestrateur de les arbitrer.
  - Aucune des deux ne remet en cause les mesures.

**Verdict précédent, du 01/11 : point 2 de §5 non validé, à cause de l'issue #67.**
- Le déterminisme est entièrement validé, et aucun budget n'est dépassé.
- **(Décidé)** #67 bloque mon point 2. *Pourquoi :* depuis le 01/11, organisation §5 point 5 interdit de reporter un défaut. Il faudra de toute façon refaire les deux campagnes après le correctif, puisque celui-ci touche à l'exécuteur, donc au déterminisme.
- **Pour lever le blocage** : #67 fusionnée, puis les campagnes du §3 et du §4 refaites avec le même banc ; le point 2 est validé si tout est d'accord et si le mode à 4 fils n'est pas plus lent que celui à 1 fil.

## 2. Méthode (décidé)

- **Banc hors du dépôt**, dans `C:\Dev\ts-cache\qa\` :
  - `harness/` : une crate `qa_harness` qui lit `ts_kernel`, `ts_persist` et `ts_toy` de la copie principale par chemin ; son `Cargo.lock` est copié du dépôt, compilé `--offline`, donc avec les mêmes versions ;
  - `run-matrix.ps1` : un processus par configuration, avec la mémoire de pointe relevée toutes les 50 ms ;
  - `matrix-det.txt` et `matrix-perf.txt` ; résultats bruts dans `runs/`.
  - *Pourquoi :* les tests de `ts_toy` ne connaissent que la graine 1, 1 ou 4 fils et un seul point d'instantané. Il faut varier les graines, les fils et les reprises sans toucher un fichier suivi.
- **Construction** : profil `sim` recopié du `Cargo.toml` racine (`lto = "thin"`, `codegen-units = 16`, `overflow-checks`), dans l'emplacement `main`. `ts-sim` est construit par `cargo build -p ts_runner --profile sim --locked`.
- **Machine** : Windows 10, i5-8300H (4 cœurs, 8 fils), 32 Go ; rustc 1.99.1.
- **Copie principale** : `main` à `043ed05`, puis à `91049e7` après la fusion de la #70 (`ts-sim run`), puis à `1ba1406` après la #92 (02/11). Aucun fichier suivi n'a été modifié.
- **02/11** : le banc accepte l'option `exec=default|always|N`, qui choisit `ParallelPhases`, l'option ajoutée par la #92. Les matrices refaites sont `matrix-det2.txt` et `matrix-perf2.txt` ; les résultats bruts sont dans `runs2/`, et le script d'enchaînement est `campaign2.sh`.
  - J'ai trouvé un dossier `target/` local dans la copie principale, hors de l'emplacement de compilation. Il pesait 156 Mo et contenait un `xtask.exe` d'une autre session.
  - **(Décidé)** Je l'ai supprimé, puis j'ai pris l'emplacement avec `session-start -Acquire`, comme le demande l'avertissement du crochet. *Pourquoi :* c'est un cache régénérable, et aucune compilation ne tournait.
- **Linux** : il n'y a pas de Rust sous WSL. **(Décidé)** je n'installe aucune chaîne sur la machine. *Pourquoi :* ce serait une modification hors du projet, et 05 §2.10 n'exige pas le déterminisme entre plateformes. La comparaison passe par la CI, pour la graine 1.

## 3. Campagne de déterminisme

### 3.1 Avant le correctif de #67 (01/11)

**Monde-jouet de T9a** (`ts_toy`, 1 000 résidents, grille de 32 × 32), un an simulé (365 jours) en saut de temps, profil `sim`.

**Matrice** : 16 processus.
- Graines publiques : 1, 2, 3, 42 et 2⁶⁴ − 1.
- Fils : 1, 4 et 8, chaque fois dans un processus distinct. La graine 2 tourne deux fois sur 4 fils.

**Chaque processus vérifie :**
1. le générateur exécuté sur le groupe de fils et en ligne ;
2. une exécution d'une traite (un seul `run_until`) ;
3. une exécution découpée, avec l'empreinte relevée tous les 30 jours ;
4. 3 ou 4 instantanés isolés, chacun encodé en `zstd` et en `Stored`, décodé, restauré, réencodé, puis mené jusqu'à la fin. Les points choisis : promenade du jour 182 (avec des entités en `LodComplete`), jours 7 et 14, minuit pile, 08:00:00.050, 23:59:59.950, jour 1 + 1 ms, etc. ;
5. des reprises enchaînées : une tous les 30 jours, ou tous les 7 jours pour la graine 42, soit 52 reprises de suite ;
6. un instantané de l'état final.

**Résultats : aucun écart.**
- **Par graine, les 3 ou 4 processus donnent les mêmes valeurs, quel que soit le nombre de fils**, sur 79 à 91 clés comparées : empreinte finale, 12 points de contrôle, empreinte des octets encodés de chaque instantané, générateur, nombre de ticks et de ticks sautés.
- **Le découpage de l'exécution ne change pas l'empreinte**, et chaque reprise donne l'empreinte attendue, juste après la restauration comme à la fin. Au total, 379 allers-retours d'instantané sont verts.
- Le réencodage d'un monde restauré redonne les mêmes octets. Le codec `Stored` restaure le même état que `zstd`.
- Des graines différentes donnent des mondes différents.
- **Valeurs dorées** (`data/golden/ts_toy.json`) retrouvées pour la graine 1 : `year.world` `c5217b31…`, 207 994 ticks exécutés et 630 512 006 sautés ; version courte `b47cd9a9…` ; générateur `d6906127…` ; règles `1d52dc6c…`.

| Graine | Empreinte de l'an | Ticks exécutés |
|---|---|---|
| 1 | `c5217b31157265f2…` | 207 994 |
| 2 | `76b3666826f13464…` | 206 929 |
| 3 | `8cbd3dbd2c360d5f…` | 207 434 |
| 42 | `672cd967b5fea7bf…` | 207 372 |
| 2⁶⁴ − 1 | `03b3dee0618d7ca7…` | 206 884 |

**CI Linux** : tâche de nuit du 01/11 ([exécution 41271957098](https://github.com/CreatorInstant548/time_switcher/actions/runs/41271957098), `cargo xtask determinism --full`, en debug, sur le commit `7365327`). Elle donne la même empreinte d'an, `c5217b31…`, sur 1 et 4 fils. Windows en `sim` et Linux en debug sont donc identiques pour la graine 1.

**Avec `ts-sim run` (PR #70, fusionnée)** : `data/scenarios/toy_year.ron`, avec les 5 graines, deux exécutions chacune.
- L'empreinte d'état est identique à celle du banc pour chaque graine, alors que `ts-sim` avance par demi-journées.
- Pour une même graine, `metrics.parquet` et `manifest.ron` sont identiques à l'octet près entre les deux exécutions.
- Limite : `ts-sim` tourne toujours sur 1 fil (`SimConfig::default()`). Le cas N fils n'est couvert que par le banc.

### 3.2 Après le correctif de #67 (02/11, `main` à `1ba1406`)

**Ce qui a changé** : la #92 choisit l'exécuteur phase par phase.
- Par défaut, `FromWork(256)` ne prend l'exécuteur parallèle que pour une phase qui compte au moins deux systèmes et assez de travail.
- Or le monde-jouet n'a qu'un système par phase : avec ce réglage, il ne passe donc jamais par l'exécuteur parallèle.
- **(Décidé)** Chaque configuration à 4 ou 8 fils tourne deux fois : une fois avec `FromWork(256)`, une fois avec `Always`.
- *Pourquoi :* `FromWork(256)` est ce que le jeu utilisera, et `Always` est le seul moyen d'exercer encore l'exécuteur parallèle sur ce monde. Les preuves de déterminisme du dépôt font de même.

**Matrice** : les mêmes graines et les mêmes points d'instantané qu'au §3.1. Pour chaque graine, 5 processus : 1 fil, puis 4 et 8 fils, chacun avec `FromWork(256)` et avec `Always`. Au total, 25 processus.

**Résultats : aucun écart.**
- **Entre les processus d'une même graine**, les 58 à 66 clés hors durées sont identiques :
  - l'empreinte finale et les 12 points de contrôle ;
  - l'empreinte des octets encodés de chaque instantané et de l'état final ;
  - le générateur, ainsi que les nombres de ticks exécutés et sautés.
- **Avec le §3.1**, ces mêmes clés sont identiques aussi : le correctif ne change aucun résultat.
- **Reprises** : 605 allers-retours d'instantané, tous verts (instantanés isolés, reprises enchaînées et état final). Le réencodage et le codec `Stored` sont toujours d'accord.
- **`ts-sim run`**, avec 5 graines et deux exécutions chacune :
  - les empreintes sont les mêmes qu'au §3.1 ;
  - `metrics.parquet` et `manifest.ron` sont identiques à l'octet près entre les deux exécutions, et avec ceux du 01/11.
- **Banc du dépôt** (`cargo bench -p ts_runner --bench time_skip --profile sim`, 1, 2, 4 et 8 fils, 3 exécutions chacun) : tous les processus donnent la même empreinte, `c5217b31…`.
- **CI Linux** : la CI de la fusion sur `main` ([exécution 41361057716](https://github.com/CreatorInstant548/time_switcher/actions/runs/41361057716)) passe `determinism` en version courte, avec 1 et 4 fils d'accord avec `data/golden/ts_toy.json`.
  - Les valeurs dorées n'ont pas changé, et la version d'un an sur Linux était déjà égale au 01/11.
  - La prochaine version d'un an tournera dans la tâche de nuit du dimanche.

## 4. Campagne de performances

### 4.1 Avant le correctif de #67 (01/11)

Profil `sim`, graine 1. Chaque configuration tourne seule, dans son propre processus.

| Monde | Fils | Durée d'un an | Mémoire de pointe (ensemble de travail) |
|---|---|---|---|
| 1 000 résidents | 1 | 2,6 s et 2,8 s (deux processus) | 5,4 Mo |
| 1 000 | 2 | 28,0 s | 5,7 Mo |
| 1 000 | 4 | 28,0 s | 5,7 Mo |
| 1 000 | 8 | 28,9 s | 5,9 Mo |
| 10 000 (grille de 100 × 100, 454 565 ticks) | 1 | 37,4 s | 9,9 Mo (33 Mo avec les instantanés) |
| 10 000 | 4 | 139,5 s | 10,2 Mo (34 Mo avec les instantanés) |
| `ts-sim run`, 1 000 | 1 | 3,8 à 5,2 s (processus complet, Parquet compris) | 5,6 à 6,2 Mo |

**Instantanés** (état complet) :

| Monde | `Stored` | `zstd` | Taux | Copie | Encodage `zstd` | Décodage | Restauration |
|---|---|---|---|---|---|---|---|
| 1 000 | 85,6 ko | 31 à 32 ko | ≈ 2,7 | — | 6 à 28 ms | 1 à 4 ms | 4 à 8 ms |
| 10 000 | 851 ko | 317 à 333 ko | ≈ 2,6 | 11 à 47 ms | 47 à 85 ms | 22 à 27 ms | 53 à 81 ms |

**Comparaison aux budgets** (attendus-mesurables, 05) :

| Attendu | Cible | Mesure | Verdict |
|---|---|---|---|
| M1 | Même empreinte au bit, sur 1 fil comme sur N | Identique sur 1, 4 et 8 fils, 5 graines | **Tenu** |
| M2 | Monde-jouet de 1 000 entités, un an, même empreinte | 2,6 s, même empreinte à chaque exécution, et égale aux valeurs dorées | **Tenu** |
| M7 | Assertions de taille des composants à la compilation (phase 0) | La macro `assert_size!` existe dans `ts_kernel` (par exemple `StableId` sur 8 octets) | **Tenu** pour la phase 0. Le monde-jouet n'a pas les composants réels, et sa mémoire (10 Mo pour 10 000) n'indique rien sur 0,55 à 0,85 Go |
| M3 (phase 1, à titre indicatif) | Un an pour 10 000 habitants en moins de 5 min | Monde-jouet vide : 37 s sur 1 fil, 140 s sur 4 fils | Pas encore applicable. Le mode multi-fils consomme déjà 47 % du budget sans aucune règle : c'est le défaut #67 |
| Architecture §12.3 | Taux de compression de 2,1 à 6,7 ; 60 à 120 Mo compressés pour 10 000 habitants | Taux de 2,6 à 2,7 | Taux **tenu**. La taille n'est pas applicable au monde-jouet |

**Constats :**
1. **[#67](https://github.com/CreatorInstant548/time_switcher/issues/67) confirmée en profil `sim`** ([commentaire](https://github.com/CreatorInstant548/time_switcher/issues/67#issuecomment-6563850990)).
   - Environ 120 µs par tick exécuté sur 2 à 8 fils, contre 13 µs sur 1 fil. C'est un coût fixe par passage d'ordonnancement, sans contention.
   - Les durées sur 4 ou 8 fils varient aussi beaucoup d'un processus à l'autre : de 24 à 57 s pour le même an.
   - J'ai ajouté l'étiquette `role:qa` à l'issue, puisque je referai la mesure après le correctif.
2. **À surveiller en phase 1, sans issue (décidé)** : la copie d'un instantané complet de 10 000 entités-jouets prend 11 à 47 ms. L'architecture §12.3 vise « quelques millisecondes », mais pour la copie des morceaux modifiés d'un instantané incrémental, qui n'existe pas encore. *Pourquoi pas d'issue :* aucun budget de la phase 0 n'est en cause, et il n'y a rien à corriger aujourd'hui. Je mesurerai en phase 1, sur les composants réels.
3. Hors de mon périmètre, noté pour mémoire : le fuzzing de la tâche de nuit du 01/11 est en échec ([#75](https://github.com/CreatorInstant548/time_switcher/issues/75), `nightly-failure`). Il est suivi par le développement et par l'architecture et la sécurité.

### 4.2 Après le correctif de #67 (02/11, `main` à `1ba1406`)

Mêmes conditions qu'au §4.1, mais d'autres sessions partageaient la machine : les durées mesurées au banc d'essai sont bruitées. Elles montrent parfois une durée totale près de deux fois supérieure au temps de calcul. Les chiffres les plus stables sont ceux du banc `time_skip`, qui garde l'exécution la plus rapide sur 3.

**Banc `time_skip`** (1 000 résidents, 365 jours, graine 1, `FromWork(256)`) :

| Fils | Plus rapide | µs par tick | Médiane |
|---|---|---|---|
| 1 | 1 493 ms | 7 | 1 848 ms |
| 2 | 1 502 ms | 7 | 1 550 ms |
| 4 | 1 520 ms | 7 | 1 613 ms |
| 8 | 1 486 ms | 7 | 1 542 ms |

**Banc d'essai** (un an d'une traite, mémoire de pointe en ensemble de travail) :

| Monde | 1 fil | 4 fils, `FromWork(256)` | 8 fils, `FromWork(256)` | 4 fils, `Always` |
|---|---|---|---|---|
| 1 000 résidents | 4,1 s (deux processus) ; 5,4 Mo | 3,7 s (deux processus) ; 5,5 Mo | 3,5 s ; 5,7 Mo | 21,9 s |
| 10 000 résidents (454 565 ticks) | 34,3 s ; 10,1 Mo | 29,7 s ; 10,3 Mo | 31,0 s ; 10,0 Mo | 86,8 s |
| `ts-sim run`, 1 000 | 2,0 à 2,7 s (processus complet, Parquet compris) | — | — | — |

Avant le correctif, il fallait 28 s pour 1 000 résidents et 139,5 s pour 10 000, sur 4 fils.

**Instantanés** : les tailles sont inchangées.
- 1 000 résidents : 85 599 octets en `Stored`, 31 066 en `zstd`.
- 10 000 résidents : 850 599 octets en `Stored`, 316 563 à 332 971 en `zstd`.
- Durées à 10 000 résidents : copie 7 à 11 ms ; encodage `zstd` 38 à 59 ms ; décodage 16 à 22 ms ; restauration 41 à 60 ms.

**Comparaison aux budgets** : les mêmes verdicts qu'au §4.1, avec deux changements.
- **M3** (indicatif en phase 0) : le monde-jouet de 10 000 habitants prend maintenant environ 30 s sur 4 fils, contre 140 s auparavant, soit environ 10 % du budget de 5 minutes, sans règles.
- **#67** : le défaut est levé. Le mode multi-fils ne coûte plus rien sur le monde-jouet, et le mode à 4 fils n'est jamais plus lent qu'à 1 fil. La condition posée au §1 est remplie.

**Constats :**
1. **L'exécuteur parallèle (`Always`) coûte toujours environ 85 µs par tick** pour 1 000 résidents, et environ 115 µs pour 10 000, sur un monde où chaque phase n'a qu'un système.
   - C'est le coût propre à `bevy_ecs` qu'a identifié la #92. Le réglage par défaut l'évite, ce n'est donc pas un défaut.
   - Ce coût fixe est la référence du calibrage des seuils, en phase 1 : un seuil n'est bon que si le travail parallèle le rembourse sur les vrais systèmes.
   - Mesures déposées dans [#94](https://github.com/CreatorInstant548/time_switcher/issues/94#issuecomment-6565827242). Je les referai sur les systèmes de la phase 1.
2. **La copie d'un instantané complet** de 10 000 entités-jouets prend maintenant 7 à 11 ms, sur une machine moins chargée qu'au 01/11. La surveillance en phase 1 est maintenue (§4.1, constat 2).

## 5. Demandes envoyées

| Date | Vers | Objet | État |
|---|---|---|---|
| 01/11 | Développement, [#67](https://github.com/CreatorInstant548/time_switcher/issues/67) | Mesures en profil `sim` ; #67 bloque le point 2 | Corrigée par la PR #92. Correctif vérifié le 02/11 ([commentaire](https://github.com/CreatorInstant548/time_switcher/issues/67#issuecomment-6565826902)) |
| 01/11 | Orchestrateur (message) | Verdict : point 2 non validé, à cause de #67 | Envoyée |
| 02/11 | Développement, [#94](https://github.com/CreatorInstant548/time_switcher/issues/94) | Coût de `Always` en profil `sim`, données pour le calibrage | Déposée ([commentaire](https://github.com/CreatorInstant548/time_switcher/issues/94#issuecomment-6565827242)) |
| 02/11 | Orchestrateur (message) | Verdict : point 2 validé | Envoyée |

## 6. Demandes reçues

| Date | De | Objet | État |
|---|---|---|---|
| 01/11 | Orchestrateur | Points 2 et 4 de §5 pour la phase 0 | Rendue le 01/11, puis le 02/11 (§1) |
| 02/11 | Développement (message) | #67 fusionnée (PR #92) : refaire les campagnes | Faite le 02/11 (§3.2, §4.2) |

## 7. Pour la relève

- Pour reproduire : `cargo build --profile sim --offline` dans `C:\Dev\ts-cache\qa\harness`, avec `CARGO_TARGET_DIR=C:\Dev\ts-cache\target\main`, puis `pwsh -File C:\Dev\ts-cache\qa\run-matrix.ps1 -Matrix <matrice> -OutDir <dossier>`.
- Mettre à jour le `Cargo.lock` du banc depuis le dépôt quand ce dernier change.
- Pour toute la campagne d'un coup : `sh C:/Dev/ts-cache/qa/campaign2.sh`, qui enchaîne le déterminisme, les performances, les 10 exécutions de `ts-sim` et le banc `time_skip`, soit environ une heure. Comparer ensuite au §3.2 et au §4.2.
- En phase 1, sur les vrais systèmes :
  - refaire la mesure de `Always` contre `FromWork` (#94) ;
  - mesurer la copie des instantanés (§4.1, constat 2) ;
  - mesurer le budget M3.
