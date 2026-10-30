#!/usr/bin/env bash
# tools/ci/main-changed.sh : main a-t-il changé depuis la dernière exécution hebdomadaire réussie de
# nightly.yml (SEC0-33, G9) ? Lecture seule par l'API (permission actions: read).
# Exécution hebdomadaire : planifiée un dimanche, ou lancée à la main (workflow_dispatch).
# Entrées (environnement) : GH_TOKEN, GH_REPO, GITHUB_SHA, GITHUB_EVENT_NAME.
# Sortie : changed=true|false dans GITHUB_OUTPUT (sinon la sortie standard).
# Un lancement manuel vaut toujours changed=true : il sert aux essais.
# spec: carnet architecture-securite.md SEC0-33 ; §9.6 G9
set -euo pipefail

out=${GITHUB_OUTPUT:-/dev/stdout}
if [ "${GITHUB_EVENT_NAME:-}" = workflow_dispatch ]; then
  echo "Lancement manuel : travaux hebdomadaires lancés."
  echo "changed=true" >> "$out"
  exit 0
fi

# Jour 0 de gmtime : dimanche.
last=$(gh api "repos/$GH_REPO/actions/workflows/nightly.yml/runs?branch=main&status=success&per_page=100" \
  --jq '[.workflow_runs[] | select(.event == "workflow_dispatch"
          or (.event == "schedule" and ((.run_started_at | fromdateiso8601 | gmtime | .[6]) == 0)))][0].head_sha // ""')

if [ -n "$last" ] && [ "$last" = "$GITHUB_SHA" ]; then
  echo "main inchangée depuis la dernière exécution hebdomadaire réussie ($last) : travaux sautés."
  echo "changed=false" >> "$out"
else
  echo "main a changé (dernière exécution hebdomadaire réussie : ${last:-aucune})."
  echo "changed=true" >> "$out"
fi
