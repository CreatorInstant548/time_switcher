#!/usr/bin/env bash
# tools/ci/install-tools.sh : installation épinglée des outils de la CI (SEC0-30, G6). Mode d'emploi : docs/dev/ci.md.
#   install-tools.sh rust         chaîne Rust de rust-toolchain.toml, par la rustup de l'image
#   install-tools.sh gitleaks     binaire Linux vérifié (archive et exécutable), configuration d'origin/main (G5)
#   install-tools.sh gitleaks-selftest  l'installation trouve un faux secret, en dossier et en commit -diff (G4)
#   install-tools.sh cargo-deny   binaire officiel, archive vérifiée par son SHA-256
#   install-tools.sh cargo-audit  cargo install --locked --version, dans ~/ts-tools/cargo-audit (mis en cache)
#   install-tools.sh cargo-fuzz   cargo install --locked --version, avec la nightly datée FUZZ_TOOLCHAIN
#   install-tools.sh version <outil>  version de tools.lock, écrite dans GITHUB_OUTPUT (clé de cache)
# Les valeurs viennent de tools/dev/tools.lock du dépôt. Aucune action tierce, aucune réussite silencieuse.
# spec: carnet architecture-securite.md SEC0-06, SEC0-30 ; §9.6 G5, G6
set -euo pipefail

root=$(git rev-parse --show-toplevel)
lock="$root/tools/dev/tools.lock"
cache=${TS_CACHE:-}

fail() { printf 'ÉCHEC (install-tools) : %s\n' "$*" >&2; exit 1; }
need_cache() { [ -n "$cache" ] || fail "TS_CACHE non défini."; }

lock_value() { # lock_value <clé> : valeur non vide de tools.lock, sans guillemets
  local v
  v=$(sed -n "s/^$1 *= *\"\(.*\)\".*/\1/p" "$lock" | head -n 1)
  [ -n "$v" ] || fail "$1 absent ou vide dans tools/dev/tools.lock."
  printf '%s' "$v"
}

sha256_of() { sha256sum "$1" | cut -d ' ' -f 1; }

# download <url> <sha256 attendu> <fichier> : HTTPS seulement, depuis les publications officielles
# de l'outil (liste fermée : une PR ne peut pas désigner un autre dépôt).
download() {
  case $1 in
    https://github.com/gitleaks/gitleaks/releases/download/*) ;;
    https://github.com/EmbarkStudios/cargo-deny/releases/download/*) ;;
    *) fail "source refusée : $1 (publications officielles de gitleaks ou cargo-deny seulement)." ;;
  esac
  curl --proto '=https' --tlsv1.2 -fsSL --retry 3 -o "$3" "$1" || fail "téléchargement impossible : $1"
  local actual
  actual=$(sha256_of "$3")
  [ "$actual" = "$2" ] || fail "empreinte de l'archive inattendue ($actual, attendu $2) : $1"
}

install_rust() {
  local channel
  channel=$(sed -n 's/^channel *= *"\(.*\)".*/\1/p' "$root/rust-toolchain.toml" | head -n 1)
  [ -n "$channel" ] || fail "channel absent de rust-toolchain.toml."
  rustup toolchain install "$channel" --profile minimal --component rustfmt --component clippy --no-self-update
  rustup show active-toolchain
}

install_gitleaks() {
  local version src archive_sha rel expected dir tmp actual
  need_cache
  version=$(lock_value 'gitleaks\.linux\.version')
  [ "$version" = "$(lock_value 'gitleaks\.version')" ] || fail "gitleaks.linux.version différente de gitleaks.version (G6)."
  src=$(lock_value 'gitleaks\.linux\.source')
  archive_sha=$(lock_value 'gitleaks\.linux\.archive_sha256')
  rel=$(lock_value 'gitleaks\.linux\.path')
  case $rel in /*|*..*) fail "gitleaks.linux.path doit être relatif à TS_CACHE, sans .." ;; esac
  dir=$(dirname "$cache/$rel")
  tmp=$(mktemp -d)
  download "$src" "$archive_sha" "$tmp/gitleaks.tar.gz"
  tar -xzf "$tmp/gitleaks.tar.gz" -C "$tmp" gitleaks || fail "archive de gitleaks illisible."
  actual=$(sha256_of "$tmp/gitleaks")
  echo "gitleaks $version : empreinte de l'exécutable $actual"
  expected=$(lock_value 'gitleaks\.linux\.sha256')
  [ "$actual" = "$expected" ] || fail "empreinte de gitleaks inattendue ($actual, attendu $expected)."
  rm -rf "$dir" && mkdir -p "$dir"
  install -m 0555 "$tmp/gitleaks" "$cache/$rel"
  rm -rf "$tmp"
  # Configuration de main, jamais celle de la PR (G5) ; xtask vérifie qu'elle est identique.
  git -C "$root" show origin/main:tools/dev/hooks/gitleaks.toml > "$dir/gitleaks.toml" ||
    fail "tools/dev/hooks/gitleaks.toml illisible sur origin/main."
  chmod 0444 "$dir/gitleaks.toml"
  [ -z "${GITHUB_ENV:-}" ] || printf 'TS_GITLEAKS_DIR=%s\n' "$dir" >> "$GITHUB_ENV"
}

# Vérification de l'installation (G4) : gitleaks, avec la configuration installée et les options de
# xtask check, doit trouver un faux secret dans un dossier, puis dans un commit sous l'attribut -diff.
selftest_gitleaks() {
  local bin dir tmp token code
  need_cache
  bin="$cache/$(lock_value 'gitleaks\.linux\.path')"
  dir=$(dirname "$bin")
  tmp=$(mktemp -d)
  # Faux jeton fabriqué à l'exécution par concaténation : jamais commité, jamais affiché (--redact).
  token="gh""p_$(openssl rand -base64 48 | tr -dc 'A-Za-z0-9' | cut -c 1-36)"
  [ "${#token}" -eq 40 ] || fail "faux jeton de longueur inattendue."
  mkdir "$tmp/dir" "$tmp/repo"
  printf 'token = "%s"\n' "$token" > "$tmp/dir/config.txt"
  unset token
  run_selftest() { # run_selftest <mode> <cible> [options] : code 42 attendu (secret trouvé)
    local mode=$1 target=$2; shift 2
    set +e
    "$bin" "$mode" --config "$dir/gitleaks.toml" --gitleaks-ignore-path "$dir" --ignore-gitleaks-allow \
      --redact --no-banner --log-level warn --exit-code 42 "$@" "$target"
    code=$?
    set -e
    [ "$code" -eq 42 ] || fail "auto-test de gitleaks ($mode) : faux secret non trouvé (code $code)."
    echo "auto-test de gitleaks ($mode) : faux secret trouvé."
  }
  run_selftest dir "$tmp/dir"
  cp "$tmp/dir/config.txt" "$tmp/repo/"
  printf '* -diff\n' > "$tmp/repo/.gitattributes"
  local g=(git -C "$tmp/repo" -c core.hooksPath=/dev/null -c commit.gpgsign=false -c user.name=ci -c user.email=ci@localhost)
  "${g[@]}" init -q
  "${g[@]}" add -A
  "${g[@]}" commit -q -m "test: auto-test"
  run_selftest git "$tmp/repo" --log-opts="--text --no-ext-diff --no-textconv HEAD"
  rm -rf "$tmp"
}

install_cargo_deny() {
  local version src archive_sha dir tmp name
  need_cache
  version=$(lock_value 'cargo-deny\.linux\.version')
  [ "$version" = "$(lock_value 'cargo-deny\.version')" ] || fail "cargo-deny.linux.version différente de cargo-deny.version (G6)."
  src=$(lock_value 'cargo-deny\.linux\.source')
  archive_sha=$(lock_value 'cargo-deny\.linux\.archive_sha256')
  dir="$cache/ci/cargo-deny"
  tmp=$(mktemp -d)
  download "$src" "$archive_sha" "$tmp/cargo-deny.tar.gz"
  name=$(basename "$src" .tar.gz)
  tar -xzf "$tmp/cargo-deny.tar.gz" -C "$tmp" "$name/cargo-deny" || fail "archive de cargo-deny illisible."
  rm -rf "$dir" && mkdir -p "$dir"
  install -m 0555 "$tmp/$name/cargo-deny" "$dir/cargo-deny"
  rm -rf "$tmp"
  add_path "$dir"
  "$dir/cargo-deny" --version
}

install_cargo_audit() {
  local version dir
  version=$(lock_value 'cargo-audit\.version')
  dir="$HOME/ts-tools/cargo-audit"
  if [ -x "$dir/bin/cargo-audit" ] && [ "$("$dir/bin/cargo-audit" audit --version | awk '{print $NF}')" = "$version" ]; then
    echo "cargo-audit $version : copie en cache"
  else
    rm -rf "$dir"
    cargo install --locked --version "$version" --root "$dir" cargo-audit
  fi
  add_path "$dir/bin"
}

# cargo-fuzz, avec la nightly datée du fuzzing (FUZZ_TOOLCHAIN, G6).
install_cargo_fuzz() {
  local version dir toolchain=${FUZZ_TOOLCHAIN:?FUZZ_TOOLCHAIN non défini}
  version=$(lock_value 'cargo-fuzz\.version')
  dir="$HOME/ts-tools/cargo-fuzz"
  rustup toolchain install "$toolchain" --profile minimal --no-self-update
  cargo "+$toolchain" install --locked --version "$version" --root "$dir" cargo-fuzz
  add_path "$dir/bin"
}

add_path() {
  [ -z "${GITHUB_PATH:-}" ] || printf '%s\n' "$1" >> "$GITHUB_PATH"
  export PATH="$1:$PATH"
}

case ${1:-} in
  rust) install_rust ;;
  gitleaks) install_gitleaks ;;
  gitleaks-selftest) selftest_gitleaks ;;
  cargo-deny) install_cargo_deny ;;
  cargo-audit) install_cargo_audit ;;
  cargo-fuzz) install_cargo_fuzz ;;
  version)
    v=$(lock_value "$(printf '%s' "${2:?outil manquant}" | sed 's/\./\\./g')\.version")
    printf 'version=%s\n' "$v" >> "${GITHUB_OUTPUT:-/dev/stdout}" ;;
  *) fail "usage : install-tools.sh rust|gitleaks|gitleaks-selftest|cargo-deny|cargo-audit|cargo-fuzz|version <outil>" ;;
esac
