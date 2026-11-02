#!/usr/bin/env bash
# tools/ci/install-tools.sh: pinned installation of the CI tools (SEC0-30, G6). Usage: docs/dev/ci.md.
#   install-tools.sh rust         Rust toolchain of rust-toolchain.toml, through the image's rustup
#   install-tools.sh gitleaks     checked Linux binary (archive and executable), configuration of origin/main (G5)
#   install-tools.sh gitleaks-selftest  the installation finds a fake secret, in a folder and in a -diff commit (G4)
#   install-tools.sh cargo-deny   official binary, archive checked by its SHA-256
#   install-tools.sh cargo-audit  cargo install --locked --version, into ~/ts-tools/cargo-audit (cached)
#   install-tools.sh cargo-fuzz   cargo install --locked --version, with the dated nightly FUZZ_TOOLCHAIN
#   install-tools.sh version <tool>  version from tools.lock, written to GITHUB_OUTPUT (cache key)
#   install-tools.sh check <subcommand> [<tool>]  reads every tools.lock value the subcommand needs, with
#                                 the same reader, and stops before any download, install or write (#95)
# The values come from the repository's tools/dev/tools.lock. No third-party action, no silent success.
# spec: carnet architecture-securite.md SEC0-06, SEC0-30 ; §9.6 G5, G6
set -euo pipefail

root=$(git rev-parse --show-toplevel)
lock="$root/tools/dev/tools.lock"
cache=${TS_CACHE:-}
mode=install
if [ "${1:-}" = check ]; then
  mode=check
  shift
fi

fail() { printf 'FAILED (install-tools): %s\n' "$*" >&2; exit 1; }
need_cache() { [ -n "$cache" ] || fail "TS_CACHE is not set."; }
# check mode: every subcommand reads all its values first, then returns here (cargo xtask check,
# workflows step: a pin missing from tools.lock fails the PR, not the run, #75).
checking() { [ "$mode" = check ]; }

lock_value() { # lock_value <key>: non-empty value of tools.lock, without quotes
  local v
  v=$(sed -n "s/^$1 *= *\"\(.*\)\".*/\1/p" "$lock" | head -n 1)
  [ -n "$v" ] || fail "${1//\\./.} missing or empty in tools/dev/tools.lock."
  printf '%s' "$v"
}

sha256_of() { sha256sum "$1" | cut -d ' ' -f 1; }

# check_source <url>: HTTPS only, from the official releases of the tool (closed list: a PR cannot
# name another repository), in canonical form: no `..`, no `//` after the host, only the
# characters [A-Za-z0-9._/-] (#17 m3; the SHA-256 stays the real protection).
check_source() {
  case $1 in
    https://github.com/gitleaks/gitleaks/releases/download/*) ;;
    https://github.com/EmbarkStudios/cargo-deny/releases/download/*) ;;
    *) fail "source refused: $1 (official releases of gitleaks or cargo-deny only)." ;;
  esac
  local rest=${1#https://}
  case $rest in
    *[!A-Za-z0-9._/-]* | *..* | *//*) fail "source refused: $1 (not in canonical form)." ;;
  esac
}

# download <url> <expected sha256> <file>
download() {
  check_source "$1"
  curl --proto '=https' --tlsv1.2 -fsSL --retry 3 -o "$3" "$1" || fail "download failed: $1"
  local actual
  actual=$(sha256_of "$3")
  [ "$actual" = "$2" ] || fail "unexpected archive SHA-256 ($actual, expected $2): $1"
}

install_rust() {
  local channel
  channel=$(sed -n 's/^channel *= *"\(.*\)".*/\1/p' "$root/rust-toolchain.toml" | head -n 1)
  [ -n "$channel" ] || fail "channel missing from rust-toolchain.toml."
  checking && return 0
  rustup toolchain install "$channel" --profile minimal --component rustfmt --component clippy --no-self-update
  rustup show active-toolchain
}

install_gitleaks() {
  local version src archive_sha rel expected dir tmp actual
  version=$(lock_value 'gitleaks\.linux\.version')
  [ "$version" = "$(lock_value 'gitleaks\.version')" ] || fail "gitleaks.linux.version differs from gitleaks.version (G6)."
  src=$(lock_value 'gitleaks\.linux\.source')
  archive_sha=$(lock_value 'gitleaks\.linux\.archive_sha256')
  rel=$(lock_value 'gitleaks\.linux\.path')
  expected=$(lock_value 'gitleaks\.linux\.sha256')
  case $rel in /*|*..*) fail "gitleaks.linux.path must be relative to TS_CACHE, without .." ;; esac
  check_source "$src"
  checking && return 0
  need_cache
  dir=$(dirname "$cache/$rel")
  tmp=$(mktemp -d)
  download "$src" "$archive_sha" "$tmp/gitleaks.tar.gz"
  tar -xzf "$tmp/gitleaks.tar.gz" -C "$tmp" gitleaks || fail "gitleaks archive unreadable."
  actual=$(sha256_of "$tmp/gitleaks")
  echo "gitleaks $version: executable SHA-256 $actual"
  [ "$actual" = "$expected" ] || fail "unexpected gitleaks SHA-256 ($actual, expected $expected)."
  rm -rf "$dir" && mkdir -p "$dir"
  install -m 0555 "$tmp/gitleaks" "$cache/$rel"
  rm -rf "$tmp"
  # Configuration of main, never the PR's (G5); xtask checks that it is identical.
  git -C "$root" show origin/main:tools/dev/hooks/gitleaks.toml > "$dir/gitleaks.toml" ||
    fail "tools/dev/hooks/gitleaks.toml unreadable on origin/main."
  chmod 0444 "$dir/gitleaks.toml"
  [ -z "${GITHUB_ENV:-}" ] || printf 'TS_GITLEAKS_DIR=%s\n' "$dir" >> "$GITHUB_ENV"
}

# Check of the installation (G4): gitleaks, with the installed configuration and the options of
# xtask check, must find a fake secret in a folder, then in a commit under the -diff attribute.
selftest_gitleaks() {
  local rel bin dir tmp token code
  rel=$(lock_value 'gitleaks\.linux\.path')
  checking && return 0
  need_cache
  bin="$cache/$rel"
  dir=$(dirname "$bin")
  tmp=$(mktemp -d)
  # Fake token built at run time by concatenation: never committed, never shown (--redact).
  token="gh""p_$(openssl rand -base64 48 | tr -dc 'A-Za-z0-9' | cut -c 1-36)"
  [ "${#token}" -eq 40 ] || fail "fake token of unexpected length."
  mkdir "$tmp/dir" "$tmp/repo"
  printf 'token = "%s"\n' "$token" > "$tmp/dir/config.txt"
  unset token
  run_selftest() { # run_selftest <mode> <target> [options]: exit code 42 expected (secret found)
    local mode=$1 target=$2; shift 2
    set +e
    "$bin" "$mode" --config "$dir/gitleaks.toml" --gitleaks-ignore-path "$dir" --ignore-gitleaks-allow \
      --redact --no-banner --log-level warn --exit-code 42 "$@" "$target"
    code=$?
    set -e
    [ "$code" -eq 42 ] || fail "gitleaks self-test ($mode): fake secret not found (exit code $code)."
    echo "gitleaks self-test ($mode): fake secret found."
  }
  run_selftest dir "$tmp/dir"
  cp "$tmp/dir/config.txt" "$tmp/repo/"
  printf '* -diff\n' > "$tmp/repo/.gitattributes"
  local g=(git -C "$tmp/repo" -c core.hooksPath=/dev/null -c commit.gpgsign=false -c user.name=ci -c user.email=ci@localhost)
  "${g[@]}" init -q
  "${g[@]}" add -A
  "${g[@]}" commit -q -m "test: self-test"
  run_selftest git "$tmp/repo" --log-opts="--text --no-ext-diff --no-textconv HEAD"
  rm -rf "$tmp"
}

install_cargo_deny() {
  local version src archive_sha dir tmp name
  version=$(lock_value 'cargo-deny\.linux\.version')
  [ "$version" = "$(lock_value 'cargo-deny\.version')" ] || fail "cargo-deny.linux.version differs from cargo-deny.version (G6)."
  src=$(lock_value 'cargo-deny\.linux\.source')
  archive_sha=$(lock_value 'cargo-deny\.linux\.archive_sha256')
  check_source "$src"
  checking && return 0
  need_cache
  dir="$cache/ci/cargo-deny"
  tmp=$(mktemp -d)
  download "$src" "$archive_sha" "$tmp/cargo-deny.tar.gz"
  name=$(basename "$src" .tar.gz)
  tar -xzf "$tmp/cargo-deny.tar.gz" -C "$tmp" "$name/cargo-deny" || fail "cargo-deny archive unreadable."
  rm -rf "$dir" && mkdir -p "$dir"
  install -m 0555 "$tmp/$name/cargo-deny" "$dir/cargo-deny"
  rm -rf "$tmp"
  add_path "$dir"
  "$dir/cargo-deny" --version
}

install_cargo_audit() {
  local version dir
  version=$(lock_value 'cargo-audit\.version')
  checking && return 0
  dir="$HOME/ts-tools/cargo-audit"
  if [ -x "$dir/bin/cargo-audit" ] && [ "$("$dir/bin/cargo-audit" audit --version | awk '{print $NF}')" = "$version" ]; then
    echo "cargo-audit $version: cached copy"
  else
    rm -rf "$dir"
    cargo install --locked --version "$version" --root "$dir" cargo-audit
  fi
  add_path "$dir/bin"
}

# cargo-fuzz, with the dated nightly of the fuzzing (FUZZ_TOOLCHAIN, G6).
install_cargo_fuzz() {
  local version dir toolchain
  version=$(lock_value 'cargo-fuzz\.version')
  checking && return 0
  toolchain=${FUZZ_TOOLCHAIN:?FUZZ_TOOLCHAIN is not set}
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
    v=$(lock_value "$(printf '%s' "${2:?tool missing}" | sed 's/\./\\./g')\.version")
    checking || printf 'version=%s\n' "$v" >> "${GITHUB_OUTPUT:-/dev/stdout}" ;;
  *) fail "usage: install-tools.sh [check] rust|gitleaks|gitleaks-selftest|cargo-deny|cargo-audit|cargo-fuzz|version <tool>" ;;
esac
