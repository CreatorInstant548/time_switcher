# Carnet du responsable conception

> Tenu uniquement par le responsable conception (`local_5b3dcf29-8fbb-426c-b63a-80ab0302c1a6`), qui est la « session principale » des fichiers de conception. Règles de travail : [organisation.md](../time_switcher/docs/pilotage/organisation.md), tenu par l'orchestrateur. Court par principe. Les messages ne sont qu'une sonnette : c'est ce carnet qui fait foi.

## Mes fichiers

- Socle 01 à 04, domaines et README de la conception.
- 05-architecture.md et les revues techniques (architecture, sécurité, génération, développement par agents), **jusqu'à l'arrivée du responsable architecture et sécurité en phase 0**, qui les reprendra. Ensuite, toute décision technique qui change le jeu passe par moi.

## État au 30/10/2026

- **Conception terminée** (README v3.0) : boucle de clôture satisfaite, tableau des attendus mesurables créé, aucune action attendue de l'utilisateur. Mon rôle en phase 0 : traiter chaque semaine les remontées `role:conception` de type `ecart` et les PR fusionnées qui touchent aux règles ; vérifier en fin de phase que ce qui est construit respecte la conception (organisation §5).

## État au 29/10/2026

- Conception cohérente : 8 domaines (direction artistique comprise), 6 revues et le complément `signatures-contenu.md`, plus la revue de l'intelligence des PNJ (ia-pnj §12 à §14), tous intégrés au socle.
- Prise de fonction : aucun impact sur le socle n'était en attente. Socle complété : 05 §2.9 (parole des joueurs, A4, A6) et 05 §2.14 (W1 à W8).
- Direction artistique v0.1.1 intégrée : 01 §1.1, 02 §1.1, 05 §2.2, §2.5, §2.9, §3, §4 et §5.
- signatures-contenu (M1 à M18) intégré : 04 §2 et §7, 05 §2.2 et §2.5. **Arbitrage A7** : « aucun geste gratuit » prime, donc pas d'acte de geste libre (A18, A19, A25 sans objet).
- GitHub en offre gratuite : 05 adapté (en-tête et phase 0) ; aucun autre de mes documents ne supposait GitHub Pro. Complément chiffré de revue-dev-agents non demandé (décidé) : `CLAUDE.md` fixe déjà la règle, et le budget de minutes se mesurera sur la vraie CI en phase 0.
- README : renvoi à [organisation.md](../time_switcher/docs/pilotage/organisation.md) au lieu de la hiérarchie des sessions.

## Décisions et travaux en cours

| Sujet | État |
|---|---|
| **Boucle de clôture** (README, point 6) | **Satisfaite le 30/10** : sécurité (tour 2), architecture (tour 2), équilibrage (tour 3). **Conception terminée** (README v3.0) |
| Transmission des demandes croisées de la direction artistique et de signatures-contenu | Par vagues, au plus 5 sessions actives (organisation §4) |
| Futur **responsable architecture et sécurité** | Arrive en phase 0 ; reprendra 05 et les revues techniques |

## Demandes envoyées

| Date | À | Demande | État |
|---|---|---|---|
| 29/10 | conception-combat | Aucun décès d'enfant ; indignité successorale | Fait, vérifié (v0.4.2) |
| 29/10 | conception-social | Profil normatif type dans la fiche de groupe | Fait, vérifié (v0.6.1) |
| 29/10 | conception-ia-pnj | Honnêteté des commerces ; états périmés ; renvoi du point de rupture | Fait, vérifié (v1.0.1) |
| 29/10 | conception-ville | Année de construction ; ligne périmée | Fait, vérifié (v0.9.1) |
| 29/10 | conception-population | États périmés | Fait, vérifié (v0.7.1) |
| 29/10 | conception-direction-artistique | Rédiger le domaine, puis mettre à jour les états de son §12.1 | Fait, vérifié (v0.1.1) |
| 29/10 | conception-population | Direction artistique §12.2 (génome, garde-robe, anti-sosies, test de parenté, vieillissement) | Fait (v0.8) |
| 29/10 | conception-population | Reliquats de ia-pnj v1.1 (garde-robe, coiffure, goûts musicaux, instruments), interface v0.6 §15.5 (historique daté), combat v0.5 (cicatrices de départ) ; règle 6 | Envoyé |
| 29/10 | conception-ia-pnj | Direction artistique §12.3 (tenue du jour, coiffeur, gestes, musique) | Fait (v1.1) ; impacts sur 05 intégrés (§2.2) |
| 29/10 | conception-ia-pnj | Règle 6 ; interface v0.6 §15.2 (choix de la vocalisation, date du dernier contact par relation) | À envoyer |
| 29/10 | conception-ia-pnj | signatures-contenu §11.6 (J4, R4, archétypes ; réticence 0 à 3) | Fait (v1.1.1, §5.6 aligné sur M8) |
| 29/10 | conception-interface | Direction artistique §12.4 | Fait (v0.6) ; ligne de socle des portraits intégrée (05 §2.5) |
| 29/10 | conception-interface | signatures-contenu §11.7 (seuils M8, gabarits du §9, A7) ; ia-pnj v1.1 (choix de tenue, de coupe, de station) ; règle 6 | Envoyé |
| 29/10 | conception-combat | Direction artistique §12.7 et signatures-contenu §11.3 | Fait (v0.5) |
| 29/10 | conception-combat | Règle 6 | À envoyer |
| 29/10 | revue-signatures | Arbitrage A7 à reporter dans signatures-contenu (v1.0.1) | Fait (v1.0.1 : 235 actes, 25 annotés) |
| 29/10 | conception-direction-artistique | Retours de ia-pnj v1.1, population v0.8, interface v0.6 §15.12 et combat v0.5 §16.9 ; règle 6 | Envoyé |
| 29/10 | conception-ville | Direction artistique §12.5 ; ia-pnj v1.1 (coiffeurs, vestiaires, laveries, lieux de musique) ; règle 6 | Fait (v0.10) |
| 29/10 | conception-social | Direction artistique §12.6 ; signatures-contenu §11.5 ; ia-pnj v1.1 et population v0.8 (tenue, bruit, coiffeur, attirance) ; règle 6 | Envoyé; session inactive, fichier inchangé : message peut-être retenu, à relancer |
| 29/10 | conception-economie | Direction artistique §12.8 ; signatures-contenu §11.4 ; ia-pnj v1.1 (dépenses, musiciens) ; règle 6 | Envoyé; session inactive, fichier inchangé : message peut-être retenu, à relancer |
| 29/10 | conception-population | signatures-contenu §11.8 | Rien à demander (aucune modification de fond) |
| 29/10 | Orchestrateur | Responsable contenus au **début de la phase 3** (direction-artistique §10.3), à reporter dans organisation.md ; ancres et archétypes des signatures à mettre dans a-relire.md | Envoyé dans le compte rendu |
| 29/10 | Futur responsable architecture et sécurité | Direction-artistique §12.9 : avis sécurité sur l'apparence graduée et l'hérédité limitée, arborescence et LFS, synthèse vocale par `tract`, déclaration des contenus produits par IA | En attente de la phase 0 |
| 29/10 | conception-direction-artistique, conception-ia-pnj, conception-interface, conception-economie | Règle A8 (aucune action de l'utilisateur) appliquée à leur fichier | Envoyé (ajouté à leur tâche en cours) |
| 29/10 | conception-social, conception-population, conception-ville, conception-combat | Règle A8 (et règle 6 pour combat) : parcourir leur fichier | À envoyer quand des places se libèrent |
| 29/10 | Les 6 revues (architecture, équilibrage, sécurité, signatures, génération, développement par agents) | Règle A8, pour information : ne pas modifier leur revue, l'en-tête de 05 prime | À envoyer quand des places se libèrent |
| 29-30/10 | Retours reçus | population v0.8.1, social v0.7, ia-pnj v1.1.2 et v1.1.3 (A8), économie v0.7, interface v0.7, direction artistique v0.2 | Faits ; socle complété (05 §2.2 : `data/content/appearance/` ; 05 §2.5 : vocalisation, dates par relation) |
| 30/10 | Arbitrage A9 | Les cicatrices suivent le combat (≈ 30 % graves, ≈ 70 % critiques) ; la population s'aligne | Envoyé (population v0.9) |
| 30/10 | Dernière vague (clôture : plus aucune demande croisée, impacts restants notés « pour la phase 1 », seules les contradictions remontent) | combat (social, économie, A8, règle 6), ia-pnj (social, interface), population (A9, social, économie, A8), direction artistique (social, interface, économie) | Envoyé |
| 30/10 | Dernière vague, suite | ville (heures calmes et isolation ; A8), social (dernier contact dédoublé ; musique et accueil ; A8), interface (trois niveaux d'écoute, bulle par volume ; prix par pièce, chapeau) | Envoyé |
| 30/10 | revue-securite, revue-equilibrage, revue-architecture | **Revue de clôture** (décidé) : vérifier le delta depuis leur revue (direction artistique, nouveaux composants, signatures-contenu et A7, GitHub gratuit, A8), en lecture seule, avec une note `<revue>-cloture.md` dans le dossier partagé (bloquant, à corriger, à suivre). Dossier : `cloture-pour-revues-securite-equilibrage-architecture.md` | Tour 1 envoyé aux trois revues le 30/10 |
| 30/10 | revue-signatures, revue-generation, revue-dev-agents | A8 et règle 6, pour information | À envoyer avec les revues de clôture |
| 30/10 | Retours | économie v0.7 (A8 compris) ; interface v0.7.1 (A8) ; ia-pnj v1.1.3 (A8) | Faits |
| 30/10 | Futur responsable architecture et sécurité ; exploitation | Points de l'interface v0.7.1 §15.14 : accès des sessions à la console malgré FIDO2 (S21), en gardant le réseau privé, l'identité forte et l'audit chaîné ; examen humain sur contestation (RGPD art. 22) par du personnel d'exploitation, jamais par l'utilisateur, avant l'ouverture publique | En attente de la phase 0 et de l'exploitation |
| 30/10 | Dernière vague, retours | combat v0.6, social v0.8, ville v0.11, population v0.9, ia-pnj v1.2, interface v0.8, direction artistique v0.3 : aucune contradiction, sauf bulles DA/interface → **A10** | Faits ; A10 appliqué (direction artistique v0.3.1) |
| 30/10 | Orchestrateur | Boucle de clôture (README, point 6) : la phase 0 attend les trois verdicts « satisfaite » (3 tours au plus) ; à reporter dans organisation.md §5 | Envoyé dans le compte rendu |
| 30/10 | Domaines, après les trois verdicts | Corrections de clôture de l'architecture : ia-pnj §2.6 (C-1 contrôle des familles, C-2 tenue stockée), §4.1 (C-5 trois dates), §3.6 (S-4 flux de vocalisation et indice de variante) ; social §1.1 (C-5) ; interface §4.7 (C-5) ; direction artistique §2.2 (C-3 `solar_state`), §3.5 (C-4 graine de forme à la précision « visage »), S-5 (générateurs du client purs) | À grouper avec l'équilibrage. Sécurité : ia-pnj §4.1, social §1.1, interface §4.7 (C1 : mise à jour seulement à l'identification) ; direction artistique §3.5, §5.5, §3.10 d (C2 : résolution par précision, classe de voix grossière, test de réidentification), §8.3 (C3 : domaine cosmétique), §7.2, §8.2, §10.1 (C9 : formats de données seulement, quarantaine, greffons épinglés) ; interface §10.4 (C7 : dossier déterministe, deux sessions pour une mesure lourde, notification) |
| 30/10 | Phase 0 (carnet ; futur responsable architecture et sécurité) | Architecture S-1 à S-8 : budget de 0,55 à 0,85 Go par monde avec assertions de taille ; `SimDay(u16)` ; `data/content/ids.lock` ; vocalisation sans tirage client ; générateurs du client purs ; **CI estimée à 3 000–7 000 min/mois > 2 000** (contrôle local, pas de CI en brouillon, fusion par trains, pas de passage sur push, mode minimal à 80 %, `lfs: false`) ; usure agrégée faisant foi ; pause et banc de pilotage hors profil `public` | À suivre en phase 0 |
| 30/10 | Phase 0 (futur responsable architecture et sécurité ; orchestrateur pour `CLAUDE.md` et les réglages utilisateur) | Sécurité, à suivre : jetons *fine-grained* (fusion réservée à `xtask merge`), test des barrières, fuzzing surtout local, domaine cosmétique et `PlayerId` dès la phase 0, inventaire des clés dans le dépôt de déploiement, audits par sessions complétés par des outils, dialogue en ligne local (S19 sans objet), test de parenté étendu ; C4 touche `CLAUDE.md` et `~/.claude/settings.json` | À suivre ; signalé à l'orchestrateur |
| 30/10 | ia-pnj, social, interface, direction artistique | Vague de clôture groupée (corrections des trois revues, règle de clôture) | Envoyé |
| 30/10 | conception-economie | Équilibrage C1 (espèces vers le cercle associé), C2 (perte volontaire au jeu = écart d'espérance ; paris extérieurs), C3 (pièces chères = objets suivis ; vêtement offert = don ; pièces abstraites non revendables), C4 (A15 bis envers soi), C8 (complicité sur actes réciproques ; « même personne » sur indices techniques) | À envoyer dès qu'une place se libère |
| 30/10 | Phase 0 | Équilibrage P1 à P6 (AUC mensuelle en production, scénarios d'exploits et INV-OPAQUE-REASON, une action continue = une décision, campagne « fortune », RGPD art. 22 avant exclusion et fusion) | À suivre |
| 30/10 | économie, social, interface | Arbitrage A11 (rapprochement notifié, sans nommer l'autre compte) | Fait (v0.8.1, v0.9.1, v0.9.1) |
| 30/10 | Tour 2, sécurité | **Satisfaite** : rien de bloquant ni à corriger. À suivre : CLAUDE.md et organisation.md §7 alignés sur le bloc « Clôture » de 05 (orchestrateur) ; en phase 5, la notification d'un rapprochement ne cite que les actes du joueur ; une bêta de volontaires exige les préalables de l'ouverture publique | Noté |
| 30/10 | Tour 2, architecture | **Satisfaite** : rien de bloquant ni à corriger. Retouches de texte T2-3, T2-4 (`cosmetic/vocalization`, décidé) et T2-5 envoyées à ia-pnj (v1.3.1) et à la direction artistique (v0.4.1). Phase 0 : T2-1 (un tirage d'identification par épisode, flux `identification`), T2-2 (« inconnu vu » = souvenir ordinaire), T2-6 (`SimDay` sur 16 bits, ≈ 52 Mo de dates) | Retouches envoyées |
| 30/10 | Tour 2, équilibrage | **Non satisfaite** : R1 (cible liée par un secret : classe apparente pour l'affichage et le choix de proposer, vrai coût à l'exécution, éjection hors récidive) et R2 (cumul de la perte au jeu sans déduire les meilleures décisions). Socle corrigé (03 §1, 05 §2.5, 04 §7) ; envoyé à interface §2.3, ia-pnj §5.6, economie §7.6. **Tour 3 de l'équilibrage seul** à lancer après. Phase 0 : P1 (notification des rapprochements à date fixe), P2 (scénarios « sonde de la classe cachée » et « relance puis abandon ») | Corrections envoyées |
| 30/10 | Retouches et corrections du tour 2 | direction artistique v0.4.1 (T2-4, T2-5), ia-pnj v1.3.1 (T2-3 à T2-5, R1), interface v0.9.2 (R1), économie v0.8.2 (R2) | Faits, vérifiés |
| 30/10 | revue-equilibrage | **Tour 3** (dernier) : vérifier R1 et R2 | Envoyé |
| 30/10 | Tour 3, équilibrage | **Satisfaite** ; R1 et R2 vérifiés | Fait |
| 30/10 | revue-signatures, revue-generation, revue-dev-agents | Information : clôture, A8, règle 6, GitHub gratuit (sans modifier leurs revues) | Envoyé |
| 30/10 | Orchestrateur | Conception terminée : la phase 0 peut être lancée. À faire de son côté : `CLAUDE.md` et organisation.md (§5 boucle de clôture, §7) alignés sur le bloc « Clôture » de 05 (crochet Claude Code dans les réglages utilisateur, `gitleaks` en pre-push, contrôle d'origine hors CI) ; responsable contenus au début de la phase 3 ; ancres et archétypes dans a-relire.md | Envoyé dans le compte rendu |

## Demandes reçues

| Date | De | Demande | État |
|---|---|---|---|
| 29/10 | Orchestrateur | Prendre la main : état des lieux, impacts en attente, compte rendu | Fait |
| 29/10 | Orchestrateur | Lancer le domaine direction artistique, l'ajouter au README, faire la relecture de cohérence | Fait ; demandes croisées en cours |
| 29/10 | Orchestrateur | Créer et tenir ce carnet | Fait |
| 29/10 | Orchestrateur | Noter le futur responsable architecture et sécurité | Fait |
| 29/10 | Orchestrateur | Remplacer la hiérarchie des sessions du README par un renvoi à organisation.md | Fait |
| 29/10 | Orchestrateur | GitHub en offre gratuite : adapter 05 (§5, phase 0, en-tête) et mes documents | Fait |
| 29/10 | Orchestrateur | Relire et intégrer `revues/signatures-contenu.md` | Fait (A7) ; transmissions en cours |
| 29/10 | Orchestrateur | Remontées (organisation §3.1) : (1) ne jamais renuméroter une section, règle à ajouter au README et à transmettre aux domaines ; (2) dès la phase 0, chaque semaine, traiter les remontées `role:conception` de type `ecart` et les PR fusionnées qui touchent aux règles ; (3) avant la fin de la conception, tableau « Attendus mesurables » | (1) README fait (règle 6), transmission en cours ; (2) à partir de la phase 0 ; (3) à faire |
| 29/10 | Orchestrateur | Règle renforcée : aucune question ni action demandée à l'utilisateur ; nettoyer mes documents (dont la revue du développement par agents §11, par un complément dans 05) ; transmettre aux 14 sessions ; prévenir en une ligne quand la conception est terminée | Socle fait (arbitrage A8 : en-tête de 05, 04 §2 et §8, 05 §2.13, §2.16 et §5) ; transmission en cours |
| 30/10 | Utilisateur (dans cette session) | « Relancer les auditeurs de sécurité, d'équilibrage et d'architecture à la fin pour tout valider ? » | Oui (décidé) : revues de clôture sur le delta, après la dernière vague |
| 30/10 | Utilisateur (dans cette session) | Boucle : tant que les trois revues ne sont pas satisfaites, la conception n'est pas finie et la phase 0 ne commence pas | Oui (décidé), avec une borne de 3 tours ; README, point 6 |
| 30/10 | Orchestrateur | Tableau « Attendus mesurables » avant la fin de la conception | Fait : `docs/conception/attendus-mesurables.md` (M1 à M36) |

## Sessions pilotées

| Session | Identifiant | État |
|---|---|---|
| conception-ia-pnj | `local_a593b537-91ca-4432-a6c5-1ab95667ddfc` | Demandes en cours |
| conception-ville | `local_61470357-9a07-427a-8192-0ab1cd48ba0c` | Demande envoyée |
| conception-population | `local_9b96337a-8a77-4dff-80a8-b72448537fb9` | Demande en cours |
| conception-economie | `local_ebf54edd-044f-4a8c-b7b4-105f5166c9f6` | Demande envoyée |
| conception-social | `local_3d995761-52cc-4e30-bac3-c85b1e5b003a` | Demande envoyée |
| conception-combat | `local_647f2555-cab6-490c-b03d-d68fe536ddbe` | Demande en cours |
| conception-interface | `local_98f3a2ad-99c0-4bc4-8645-d2179f2a18d6` | Demande en cours |
| conception-direction-artistique | `local_d27a53f4-3489-40db-b5e8-71e23e687d7a` | Terminé (v0.1.1) ; lancée par moi |
| revue-architecture | `local_b948122b-f414-445a-a707-1a21a6b82108` | Intégrée |
| revue-equilibrage | `local_d727b095-a526-4a8c-9980-9247105a495e` | Intégrée |
| revue-securite | `local_aa55b44e-9947-4027-905d-f58c08076615` | Intégrée |
| revue-signatures | `local_42656a8f-c3d6-4376-b994-93fdeca0bf24` | signatures-contenu intégré ; A7 en cours |
| revue-generation | `local_675581bc-e305-4971-8190-d85ac4ead0bf` | Intégrée |
| revue-dev-agents | `local_e606f8c3-8697-459e-b4b0-804577816e28` | Intégrée |

Les 13 sessions d'origine ont été lancées par l'orchestrateur. Je les pilote par message, et je vérifie moi-même leurs fichiers.
