# `cargo xtask spec-map`

Mode d'emploi de l'outil qui relie le code à la conception (tâche T4 de la phase 0). Sources : [organisation](../pilotage/organisation.md) §3.1 point 3, et le §7 du carnet du responsable architecture et sécurité, avec ses corrections H1 à H8 (§9.7).

`tools/xtask/` est un **chemin sensible** : toute modification passe par `/security-review`, un relecteur neuf et l'avis d'archi-sécu. L'outil est en lecture seule, sans réseau ni dépendance nouvelle (les titres Markdown sont analysés à la main, `serde_json` produit le JSON).

## 1. Commandes

```
cargo xtask spec-map check                          # appelé par cargo xtask check (étape spec-map)
cargo xtask spec-map report [--json] [--file <nom>] # code vers sections, sections vers code, trous, PROVISOIRE
cargo xtask spec-map diff <révision>                # code qui cite les sections modifiées depuis la révision
```

- `check` rend 0 s'il n'y a aucun écart, 1 sinon. Il **échoue** sur un renvoi vers un fichier, une section ou un identifiant qui n'existe pas, sur un nom court ambigu, sur un renvoi illisible, sur une marque provisoire sans numéro, sur un élément de renvoi fait seulement de parenthèses, sur un type de fichier suivi ni analysé ni classé (§2), et sur une entrée de `known_bad.txt` périmée ou absente d'`origin/main` (§4). Sans la conception, ou sans la branche `origin/pilotage` (message « git fetch origin pilotage »), il échoue aussi : jamais de réussite silencieuse.
- `report` écrit en Markdown, ou en JSON avec `--json` (§3). `--file` limite le rapport à un fichier de conception, désigné comme dans un renvoi. Le filtre par phase est reporté (H8).
- `diff` compare la conception de la révision à celle de l'arbre de travail et liste, pour chaque section ou identifiant modifié, ajouté ou supprimé, le code qui le cite. Un renvoi à une section couvre ses sous-sections. Seul `docs/conception/` est comparé, pas les carnets. La révision est vérifiée d'abord (`git rev-parse --verify --end-of-options <rév>^{commit}`) ; une valeur qui commence par `-` est refusée, et aucun argument n'est passé à Git sans `--end-of-options` (H4).

Toute commande lit les fichiers par `Ctx::files()` (fichiers ordinaires de l'arbre, aucun lien suivi) et n'écrit que des chemins relatifs avec `/`, jamais un chemin absolu (SEC0-26, H5). La sortie est triée : même entrée, même sortie.

## 2. Ce qui est lu

**Code analysé.** Les fichiers de l'arbre, sauf `*.md` et tout `docs/` (archives comprises). Les types connus sont lus ; un type que ni le tableau ci-dessous ni la liste `NOT_SCANNED` de `refs.rs` (données, binaires, fichiers générés : `.lock`, `.json`, `.txt`, images, `.parquet`…) ne classe est un **écart** de `check`, pour qu'un nouveau type soit classé au lieu d'être sauté en silence.

| Marque de commentaire | Types de fichier |
|---|---|
| `//`, `//!`, `///`, `/* … */` | `.rs`, `.cs`, `.ron` (RON se commente par `//`, H7), `.ts`, `.tsx`, `.js`, `.mjs`, shaders (`.glsl`, `.wgsl`, `.hlsl`, `.gdshader`, `.frag`, `.vert`, `.shader`) |
| `#` | `.py`, `.toml`, `.yml`, `.yaml`, `.sh`, `.ps1`, `.psm1`, `.psd1`, `.gd`, fichiers sans extension |
| `--` | `.sql` |

Un renvoi est un commentaire de ligne **seul sur sa ligne** dont le texte commence par `spec:` (dans un commentaire de bloc `/* */`, c'est une forme refusée). Une marque provisoire est un commentaire, seul, en fin de ligne ou de bloc, qui contient le mot `PROVISIONAL` ou `PROVISOIRE` : elle doit porter un numéro de remontée (§3). Un commentaire dans une chaîne de caractères n'en est pas un, et une apostrophe au milieu d'un mot (`l'étape`) n'ouvre pas une chaîne. Le fichier est lu en UTF-8 (un BOM en tête est ignoré), en UTF-16 avec marque d'ordre des octets, ou, à défaut, avec remplacement des octets illisibles (un `.ps1` en Windows-1252) ; un fichier à octets nuls est sauté s'il n'a pas d'extension, et c'est un écart sinon.

**Documents cibles.**
- `docs/conception/**/*.md`, désignés par un nom relatif à `docs/conception/` (`domaines/combat.md`) ou par un **nom court** résolu de façon unique (`combat.md`, `securite.md`, `05-architecture.md`). Un nom court qui désigne deux fichiers est une erreur ; le nom relatif lève l'ambiguïté.
- Un carnet : `carnet <rôle>.md …` se résout dans `origin/pilotage` par `git show --end-of-options origin/pilotage:<rôle>.md`. Le nom doit être un nom de fichier simple. Ces renvois seront réécrits vers les revues à la consolidation de fin de phase 0.

**Sections.** Un titre Markdown (hors blocs de code) dont le premier mot est un numéro `n(.m)*`, avec un point final facultatif et un suffixe facultatif (`bis`, `ter` ou une lettre minuscule) : `## 2. Titre` donne `2`, `### 5.3 bis Titre` donne `5.3 bis`. Un titre sans numéro (`### Phase 0 — …`) reste dans la section numérotée au-dessus.

**Points numérotés (AS-19).** La conception cite les points d'une liste comme des sous-sections : `05-architecture.md §2.3` est le point `3. ` de la section `## 2`. Un élément de liste numérotée de **premier niveau** (`N. ` en début de ligne, sans retrait), placé directement sous un titre de numéro `k` (un titre sans numéro compte pour le dernier titre numéroté), définit la section `§k.N`. Les lignes en retrait qui suivent appartiennent au point ; une liste en retrait n'en crée pas. Si le titre a déjà un vrai sous-titre `k.N`, le sous-titre prime, sans doublon. Si deux listes sous le même titre définissent le même `§k.N`, un renvoi vers `§k.N` est **ambigu** et `check` échoue. Un titre dont la clé porte un suffixe (`5.3 bis`) ne porte pas de points.

**Identifiants.** Forme `[A-Z][A-Z0-9]*(-[A-Z0-9]+)*-?\d+` (`S8`, `M28`, `SEC0-35`, `AS-12`, `R-ARCH-04`, `T0-06`, `F3`). Un titre ou un identifiant placé dans un commentaire HTML `<!-- … -->` ne compte pas. Un identifiant est **défini dans le fichier cible** s'il est la première cellule d'une ligne de tableau, **ou la deuxième cellule quand elle n'est faite que de lui** (éventuellement entre `**` ou accents graves : tableau des décisions qui commence par la date, `| 30/10 | AS-33 | … |`), ou en gras en tête d'élément de liste (`- **X**`, `- **X.**`, `- **X — titre**`, `- [ ] **X**`, `1. **X**`) ou de ligne. Si le renvoi donne une section avant l'identifiant, celui-ci doit être défini dans cette section ou dans une de ses sous-sections.

## 3. Grammaire d'un renvoi

```
// spec: <fichier> <élément>, <élément> ; <fichier> <élément> ; <élément>
```

- `;` sépare les fichiers ; `,` sépare les éléments d'un même fichier. Un segment sans nom de fichier reprend celui du précédent (`… ; §9.5 F1 à F10`).
- Un élément est `§n(.m)*` avec `bis`, `ter` ou une lettre en option (`§3.10 b`), un identifiant, une plage `X à Y` de même préfixe (`SEC0-29 à 31`, `F1 à F10`), ou une section qui situe un identifiant (`§9.5 F1 à F10`, `§4.4 A19`).
- Un nombre seul reprend le préfixe de l'identifiant précédent (`SEC0-03, 06`).
- Un texte entre parenthèses est ignoré (`(révisé)`), mais un élément qui n'est fait que de parenthèses (`combat.md, (§3)`) est refusé : il serait ignoré sans être vérifié.
- Un renvoi sans élément (`// spec: combat.md`) cite le fichier seul.
- `PROVISIONAL(design #<n>)` : marque une valeur provisoire, `<n>` étant la remontée `role:design` ; l'ancienne forme `PROVISOIRE(conception #<n>)` reste reconnue jusqu'à sa traduction. Le mot seul, sans cette forme, est une marque sans numéro, donc un écart.

Les bornes d'une plage doivent être définies ; les identifiants définis entre les deux sont comptés comme cités.

## 4. `known_bad.txt` : la liste fermée (H3)

`tools/xtask/src/spec_map/known_bad.txt` recense les renvois faux qui se trouvent hors de `tools/xtask/` (un renvoi faux dans `tools/xtask/` se corrige sur place). Une entrée par ligne :

```
fichier | ligne | texte du renvoi | raison | #issue
```

L'issue est une issue `role:dev`. `check` ne tolère un renvoi faux que s'il correspond exactement à une entrée (fichier, ligne et texte), et il échoue sur toute entrée **périmée** (renvoi corrigé, déplacé ou disparu) : la liste ne peut que diminuer, et elle doit être vide à la fin de la phase 0. `check` refuse aussi toute entrée **absente de la version d'`origin/main`** (ligne comparée sans blancs de bord) : ajouter ou modifier une entrée ne passe plus que par la correction du renvoi. Il échoue s'il ne peut pas lire le fichier d'`origin/main`.

## 5. Schéma de `report --json`

Version 1. Les tableaux sont triés (fichier puis ligne ; documents puis sections en ordre naturel : `2` avant `10`) ; les clés d'un objet sont triées.

```
{
  "schema": 1,
  "references":  [ { "file": "…", "line": 1,
                     "targets": [ { "doc": "…", "section": "3.1" | null, "id": "S8" | null } ] } ],
  "sections":    [ { "doc": "…", "section": "3.1", "title": "…" | null,
                     "code": [ { "file": "…", "line": 1 } ] } ],
  "identifiers": [ { "doc": "…", "id": "S8", "code": [ { "file": "…", "line": 1 } ] } ],
  "sections_without_code": [ { "doc": "…", "section": "3.1", "title": "…" } ],
  "identifiers_without_code": 123,
  "identifiers_defined_twice": [ { "doc": "…", "id": "S8", "sections": [ "4.3", "6" ] } ],
  "provisional": [ { "file": "…", "line": 1, "issue": 12 | null, "text": "…" } ]
}
```

- `doc` est un chemin relatif du dépôt (`docs/conception/revues/securite.md`) ou `origin/pilotage:<rôle>.md` pour un carnet. `title` est le titre sans son numéro.
- `references` : un renvoi vers un fichier seul a `section` et `id` nuls. Un renvoi à un identifiant situé dans une section compte pour l'identifiant **et** pour la section.
- `sections_without_code` ne couvre que `docs/conception/` ; une section est « avec code » si un renvoi la cite directement (pas par un parent ni un enfant).
- `provisional` : `issue` est nul si la marque n'a pas de numéro (`check` échoue alors).
- `identifiers_defined_twice` signale (AS-20) les identifiants définis plusieurs fois dans un même fichier, avec les sections concernées : ils restent valides et `check` reste vert.
- `--file` filtre `references`, `sections`, `identifiers`, `sections_without_code`, `identifiers_without_code` et `identifiers_defined_twice` ; `provisional` reste complet.

Le tableau du responsable qualité (organisation §3.1 point 4) lit ce JSON.
