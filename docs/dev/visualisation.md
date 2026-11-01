# Visualisation minimale (`ts_viz`)

Mode d'emploi de la visualisation de développement (tâche T11, objectif V0-10 de la phase 0). Sources : le carnet du responsable architecture et sécurité, §9.16 (VZ4, VZ5 et **VZ7**) et la décision **AS-48** ; Y1, Y3 et Y4 du §9.15.

## 1. Principe

```
ts-viz positions  →  positions.parquet  →  positions_to_rrd.py  →  positions.rrd  →  visualiseur Rerun
   (Rust, ts_viz)      (sous TS_CACHE)       (Python, optionnel)    (sous TS_CACHE)    (logiciel à part)
```

- **`rerun` n'est pas dans l'arbre Rust** (AS-48) : toutes ses variantes tirent `tokio`, `tonic`, `hyper`, `rustls`, `ring` (du C) et `re_analytics`. `ts_viz` n'ajoute **aucune dépendance** : il écrit un Parquet comme `ts_runner` écrit `metrics.parquet` (crate `parquet` sans `arrow` et sans compression, Y1).
- La conversion en `.rrd` est un **script Python optionnel** (groupe `viz` de `python/`). Ce groupe n'est installé ni par `cargo xtask py` ni par la CI.
- Le visualiseur Rerun est un logiciel de bureau tiers, installé à part, hors du dépôt.

## 2. Produire `positions.parquet`

```
pwsh -NoProfile -File tools/dev/session-start.ps1 -Acquire      # dans un worktree, avant de compiler
cargo run -p ts_viz -- positions data/scenarios/toy_short.ron --seed 1 --out <TS_CACHE>/viz/toy_short
```

- Même scénario RON, même graine **publique de test** (Y2) et mêmes règles de `--out` que `ts-sim run` (Y3) : strictement sous `TS_CACHE`, aucun lien suivi, fichier créé en `create_new`. `--allow-outside-cache` existe comme pour `ts-sim`, mais jamais dans une copie de travail Git.
- Le monde est le monde-jouet de `ts_toy` (1 000 résidents). Chaque jour, à 08:00, chaque résident donne une ligne. Un groupe de lignes par jour, par `StableId` croissant.
- Colonnes, toutes obligatoires : `day`, `tick`, `id` (`StableId`), `parcel`, `x`, `y` (entiers 64 bits) et `place` (texte : `home`, `to_work`, `work`, `to_home`). En chemin, la parcelle est celle que le résident a quittée.
- Le pied de fichier porte des métadonnées `ts_viz.*` : format, **identifiant d'enregistrement**, scénario et son empreinte, graine, empreinte des règles, empreinte d'état finale, taille de la grille.

**Bornes (VZ5, Y4)** : 1 à 3 660 jours (borne du scénario, vérifiée aussi par `encode`) et les résidents du monde-jouet, soit 3,66 millions de lignes au plus (une soixantaine de Mo sur le disque, une centaine en mémoire pendant l'écriture).

**Reproductibilité (VZ4)** :
- L'identifiant d'enregistrement est dérivé (BLAKE3) de la graine et de l'empreinte du fichier de scénario : aucun UUID aléatoire.
- Les données sont posées sur la ligne de temps simulée (`day`, `tick`).
- Le fichier ne contient ni chemin local, ni nom d'utilisateur, ni nom de machine, ni heure murale. Même scénario et même graine donnent les **mêmes octets**, sur toute machine. L'empreinte d'état finale est celle des valeurs dorées de `data/golden/ts_toy.json` (`short.world`, `year.world`).
- Tests (`cargo test -p ts_viz`) : données décodées de deux exécutions identiques, recherche de `TS_CACHE`, `HOME`, du dossier de cache, du nom d'utilisateur et du nom d'hôte dans le fichier, jours bornés, règles de `--out`.

## 3. Installer le groupe `viz` (à la main, une fois)

Dans `python/`, avec le `uv` épinglé de `tools/dev/tools.lock` (le projet impose sa version) :

```
cd python
uv sync --locked --group viz
```

- `pyproject.toml` impose les roues seulement (`no-build`), l'interpréteur installé (3.14.0) et les paquets publiés au moins deux semaines avant le verrou ; `uv.lock` porte les empreintes.
- Le groupe contient `rerun-sdk==0.38.1` seul. Il tire `pyarrow` (qui lit le Parquet : rien d'autre n'est ajouté pour cela), `attrs`, `pillow`, `psutil` et `typing-extensions`.
- `uv sync` rend l'environnement exact : la commande ci-dessus retire `ts-py` (construit par maturin), et `cargo xtask py`, qui lance `uv sync --locked` sans le groupe, **retire** les paquets de `viz` et reconstruit `ts-py`. Selon l'outil dont on a besoin, refaire l'une ou l'autre commande.

## 4. Convertir en `.rrd`

```
cd python
uv run --no-sync python -I viz/positions_to_rrd.py <TS_CACHE>/viz/toy_short/positions.parquet --out <TS_CACHE>/viz/toy_short/positions.rrd
```

Le script ne fait que lire le Parquet et appeler `rr.save(<fichier>)` :
- l'entrée et la sortie sont **strictement sous `TS_CACHE`** (sans `..`, sans lien), la sortie est un nouveau fichier `.rrd` ;
- l'entrée est traitée comme non fiable : taille (512 Mo), schéma exact, nombre de lignes, métadonnées et valeurs sont contrôlés avant tout usage ;
- jamais `spawn`, `connect_grpc`, `serve` ni `show` : le script n'ouvre pas le visualiseur et n'ouvre aucune connexion (un test lit son code pour le vérifier) ;
- l'enregistrement a l'identifiant du fichier, deux lignes de temps (`day`, `tick`), les résidents en points 2D colorés par lieu, et les effectifs par lieu en séries. Ni heure murale (`log_time` coupée), ni propriétés d'enregistrement (`send_properties=False`).

**Télémétrie coupée.** La documentation de Rerun (README de `re_analytics`) donne `rerun analytics disable` pour refuser toute collecte, qui ne concerne que les événements du visualiseur. Avant d'importer `rerun`, le script :
1. retire de son environnement `RERUN*`, `FORCE_RERUN_ANALYTICS`, `OTEL_*` et `TELEMETRY_*` (ils pourraient rediriger le CLI ou les points d'envoi) ;
2. lance cette commande par le CLI livré dans la roue (`python -I -m rerun_cli analytics disable`) : elle écrit seulement le refus dans la configuration d'analyse de l'utilisateur (`analytics.json`), n'ouvre rien et n'envoie rien. Si elle échoue, le script s'arrête.

**Ce qui reste non reproductible dans le `.rrd`** (VZ4) : Rerun donne à chaque lot de données (*chunk*) un identifiant, et à chaque ligne un `RowId`, tous deux fondés sur l'heure et l'aléa. Deux conversions du même Parquet ne donnent donc pas les mêmes octets, mais les **mêmes données décodées**. Le fichier ne contient aucun chemin, nom d'utilisateur ni nom de machine. Les tests du groupe (`uv run --no-sync pytest viz -p no:cacheprovider`, hors de `cargo xtask py` et de la CI) comparent les données décodées.

## 5. Ouvrir le `.rrd`

Installer le visualiseur Rerun **à part**, par une version de son choix (téléchargement depuis le site de Rerun, ou `cargo install rerun-cli` hors de l'espace de travail). Ne pas l'installer dans `python/.venv` ni ajouter `rerun` à un `Cargo.toml` du dépôt. Avant son premier lancement, refuser sa télémétrie : `rerun analytics disable`. Puis :

```
rerun <TS_CACHE>/viz/toy_short/positions.rrd
```

Faire défiler la ligne de temps `day` (ou `tick`) pour voir les résidents se répartir entre leur maison, leur travail et la route à 08:00, jour après jour.

Ne jamais ajouter de `.rrd` ni de Parquet au dépôt (artefacts régénérables, A6).
