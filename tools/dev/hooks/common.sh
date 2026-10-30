# tools/dev/hooks/common.sh : fonctions communes des crochets Git (chargé par « . »).
# Les crochets s'exécutent depuis leur copie épinglée TS_CACHE/hooks/<sha256>/ (SEC0-12),
# à côté de tools.lock, qui fixe le chemin et l'empreinte de gitleaks (SEC0-06).
# spec: carnet architecture-securite.md SEC0-06, SEC0-08, SEC0-12

HOOK_DIR=$(cd "$(dirname "$0")" && pwd)

fail() {
  printf 'REFUS (%s) : %s\n' "$(basename "$0")" "$*" >&2
  exit 1
}

lock_value() { # lock_value <clé> : valeur de tools.lock, sans guillemets
  sed -n "s/^$1 *= *\"\(.*\)\".*/\1/p" "$HOOK_DIR/tools.lock" | head -n 1
}

# Vérifie gitleaks avant usage ; refuse s'il manque ou si son empreinte diffère. Jamais de réussite silencieuse.
gitleaks_checked() {
  [ -f "$HOOK_DIR/tools.lock" ] || fail "tools.lock absent de $HOOK_DIR : réinstaller les crochets (tools/dev/install-hooks.ps1)."
  GITLEAKS=$(lock_value 'gitleaks\.path')
  expected=$(lock_value 'gitleaks\.sha256')
  [ -n "$GITLEAKS" ] && [ -n "$expected" ] || fail "gitleaks.path ou gitleaks.sha256 manquant dans tools.lock."
  [ -f "$GITLEAKS" ] || fail "gitleaks introuvable ($GITLEAKS)."
  command -v sha256sum >/dev/null 2>&1 || fail "sha256sum introuvable : impossible de vérifier gitleaks."
  actual=$(sha256sum "$GITLEAKS" | cut -d ' ' -f 1 | sed 's/^\\//')
  [ "$actual" = "$expected" ] || fail "empreinte de gitleaks inattendue ($actual, attendu $expected)."
}

# gitleaks avec la seule configuration épinglée du jeu de crochets : ni .gitleaks.toml ni .gitleaksignore
# de la copie de travail, ni variable d'environnement, ni commentaire gitleaks:allow (SEC0-08, T0-08).
gitleaks_run() {
  unset GITLEAKS_CONFIG GITLEAKS_CONFIG_TOML
  [ -f "$HOOK_DIR/gitleaks.toml" ] || fail "gitleaks.toml absent de $HOOK_DIR : réinstaller les crochets."
  # gitleaks lit aussi le .gitleaksignore de la racine, quel que soit --gitleaks-ignore-path (constaté en 8.30.1).
  [ -e "$(git rev-parse --show-toplevel)/.gitleaksignore" ] &&
    fail "un .gitleaksignore à la racine de la copie de travail est refusé (SEC0-08) : une exception passe par tools/dev/hooks/gitleaks.toml."
  dir=$(cd "$HOOK_DIR" && { pwd -W 2>/dev/null || pwd; })
  "$GITLEAKS" git --config "$dir/gitleaks.toml" --gitleaks-ignore-path "$dir" --ignore-gitleaks-allow \
    --redact --no-banner --log-level warn "$@" .
}

current_branch() {
  git symbolic-ref --short -q HEAD 2>/dev/null | tr 'A-Z' 'a-z'
}
