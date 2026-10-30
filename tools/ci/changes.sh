#!/usr/bin/env bash
# tools/ci/changes.sh : zones touchées par une plage de commits (G7, G8). Mode d'emploi : docs/dev/ci.md §2.
# Entrées, par l'environnement (jamais par ${{ }} dans un run:, SEC0-31) :
#   BASE_SHA, HEAD_SHA : plage lue depuis leur base commune (git diff base...tête).
# Si la base est absente, nulle ou inconnue, toutes les zones valent true (choix sûr).
# Sortie : une ligne <zone>=true|false par zone, dans GITHUB_OUTPUT (sinon la sortie standard).
# --summary : écrit aussi le tableau des zones et les chemins sensibles touchés dans GITHUB_STEP_SUMMARY.
# spec: carnet architecture-securite.md §5, SEC0-32 ; §9.6 G7, G8
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"
ZERO=0000000000000000000000000000000000000000
ZONES="rust python web client ci docs sensitive lock gitleaks_config"
summary=0
[ "${1:-}" = --summary ] && summary=1

fail() { printf 'ÉCHEC (changes) : %s\n' "$*" >&2; exit 1; }
commit_ok() { [ -n "$1" ] && [ "$1" != "$ZERO" ] && git cat-file -e "$1^{commit}" 2>/dev/null; }

base=${BASE_SHA:-}
head=${HEAD_SHA:-}
commit_ok "$head" || fail "HEAD_SHA absent ou inconnu."
for z in $ZONES; do printf -v "z_$z" false; done

all=0
reason=""
files=()
if commit_ok "$base" && mb=$(git merge-base "$base" "$head"); then
  while IFS= read -r -d '' f; do files+=("$f"); done < <(git diff --name-only -z --no-renames "$mb" "$head")
else
  all=1
  reason="base absente ou inconnue : toutes les zones sont retenues"
  mb=""
fi

# Liste des chemins sensibles : union de celle d'origin/main et de celle de la copie de travail.
# Une PR ne peut donc pas se retirer d'elle-même de la liste (même raison que G5).
entries=$({ git show origin/main:tools/ci/sensitive-paths.txt 2>/dev/null || true
            cat tools/ci/sensitive-paths.txt; } |
          sed 's/#.*//; s/^[[:space:]]*//; s/[[:space:]]*$//; /^$/d' | sort -u)

prefixes=(); exact=(); regex=""; first=""; lock_rule=0; toml_sources=0; lock_source=0
while IFS= read -r e; do
  case $e in
    section:*)
      # « profile.* » vaut aussi « [profile] » nu (clés pointées dessous) : motif ^profile([.].+)?$.
      p=${e#section:}
      case $p in *.\*) r="${p%.\*}"; r="${r//./[.]}([.].+)?" ;; *) r=${p//./[.]}; r=${r//\*/.+} ;; esac
      regex="${regex:+$regex|}$r"; first="${first:+$first|}${p%%.*}" ;;
    cargo-lock:new-crate) lock_rule=1 ;;
    cargo-toml:sources) toml_sources=1 ;;
    cargo-lock:source-change) lock_source=1 ;;
    # B1 : chemins comparés en minuscules (Windows confond TOOLS/ et tools/).
    */) prefixes+=("${e,,}") ;;
    *) exact+=("${e,,}") ;;
  esac
done <<< "$entries"

# Lignes des sections retenues d'un Cargo.toml, en-têtes normalisés ; à la racine du fichier,
# les clés pointées ou en ligne de même premier segment (features.x = …, profile = { … }).
sections_of() { # sections_of <révision> <fichier>
  [ -n "$regex" ] || return 0
  { git show "$1:$2" 2>/dev/null || true; } | awk -v re="^($regex)\$" -v re1="^($first)\$" '
    function name(l) { sub(/^[[:space:]]*\[+[[:space:]]*/, "", l); sub(/[[:space:]]*\]+.*$/, "", l)
                       gsub(/[[:space:]"'\'']/, "", l); return l }
    BEGIN { root = 1 }
    /^[[:space:]]*\[/ { cur = name($0); root = 0; keep = (cur ~ re); if (keep) print "[" cur "]"; next }
    root { k = $0; gsub(/[[:space:]"'\'']/, "", k); split(k, a, /[.=]/); if (a[1] ~ re1) print "racine:" $0; next }
    keep { print }'
}

crate_names() { # crate_names <révision>
  { git show "$1:Cargo.lock" 2>/dev/null || true; } | sed -n 's/^name = "\(.*\)"$/\1/p' | sort -u
}

# Couples « nom source » des [[package]] d'un Cargo.lock.
crate_sources() { # crate_sources <révision>
  { git show "$1:Cargo.lock" 2>/dev/null || true; } | awk '
    /^\[\[package\]\]/ { if (n != "") print n " " s; n = ""; s = ""; next }
    /^name = "/ { n = $3; gsub(/"/, "", n) } /^source = "/ { s = $3; gsub(/"/, "", s) }
    END { if (n != "") print n " " s }' | sort -u
}

# Lignes ajoutées ou retirées d'un Cargo.toml (espaces retirés) qui touchent à [patch, [replace
# ou à une source hors crates.io (AS-23 point 4).
toml_source_lines() { # toml_source_lines <base> <tête> <fichier>
  # diff sort en 1 quand les fichiers diffèrent : ce code ne doit pas compter (pipefail).
  { diff <({ git show "$1:$3" 2>/dev/null || true; } | tr -d ' \t\r' | tr 'A-Z' 'a-z' | sort) \
         <({ git show "$2:$3" 2>/dev/null || true; } | tr -d ' \t\r' | tr 'A-Z' 'a-z' | sort) || true; } |
    grep -E '^[<>]' | grep -qE '\[patch|\[replace|git=|registry=|branch=|tag=|rev='
}

sensitive_hits=()
is_sensitive() { # is_sensitive <fichier>
  local f=$1 l=${1,,} e
  l=${l#./}
  for e in "${prefixes[@]}"; do [[ $l == "$e"* ]] && return 0; done
  for e in "${exact[@]}"; do [[ $l == "$e" ]] && return 0; done
  case $l in
    cargo.toml|*/cargo.toml)
      [ "$(sections_of "$mb" "$f")" != "$(sections_of "$head" "$f")" ] && return 0
      [ "$toml_sources" = 1 ] && toml_source_lines "$mb" "$head" "$f" && return 0 ;;
    cargo.lock)
      [ "$lock_rule" = 1 ] && [ -n "$(comm -13 <(crate_names "$mb") <(crate_names "$head"))" ] && return 0
      if [ "$lock_source" = 1 ]; then
        # Une crate de la base apparaît avec une source nouvelle.
        while read -r name _; do
          crate_sources "$mb" | awk -v n="$name" '$1 == n { found = 1 } END { exit !found }' && return 0
        done < <(comm -13 <(crate_sources "$mb") <(crate_sources "$head"))
      fi ;;
  esac
  return 1
}

# B1 : l'arbre de tête ne doit avoir ni deux chemins égaux à la casse près, ni chemin non ASCII
# sous un chemin sensible. Sinon : échec (Windows écraserait un fichier suivi au git pull).
tree=$(git ls-tree -r -z --name-only --full-tree "$head" | tr '\0' '\n')
dups=$(printf '%s\n' "$tree" | tr 'A-Z' 'a-z' | sort | uniq -d)
[ -z "$dups" ] || fail "chemins égaux à la casse près dans la tête : $(printf '%s ' $dups)"
# Windows 8.3 short names (CARGO~1 = .cargo): refused on every path component.
short=$(printf '%s\n' "$tree" | grep -E '(^|/)[^/]*~[0-9]' || true)
[ -z "$short" ] || fail "path component looks like a Windows 8.3 short name (~<digit>): $(printf '%s ' $short)"
while IFS= read -r f; do
  [ -n "$f" ] || continue
  if printf '%s' "$f" | LC_ALL=C grep -q '[^ -~]' && is_sensitive "$f"; then
    fail "chemin non ASCII sous un chemin sensible : $f"
  fi
done < <(printf '%s\n' "$tree" | LC_ALL=C grep '[^ -~]' || true)

for f in "${files[@]}"; do
  case $f in
    crates/*|tools/xtask/*|fuzz/*|.cargo/*|Cargo.toml|Cargo.lock|rust-toolchain.toml|clippy.toml|rustfmt.toml|deny.toml)
      z_rust=true ;;
  esac
  case $f in python/*|*.py|pyproject.toml|uv.lock) z_python=true ;; esac
  case $f in admin-web/*|package.json|pnpm-lock.yaml|pnpm-workspace.yaml) z_web=true ;; esac
  case $f in client/*) z_client=true ;; esac
  # Code ou configuration que la CI exécute pour se contrôler elle-même.
  case $f in .github/*|tools/ci/*|tools/xtask/*|tools/dev/tools.lock|tools/dev/hooks/gitleaks.toml) z_ci=true ;; esac
  case $f in docs/*|*.md) z_docs=true ;; esac
  case $f in Cargo.lock) z_lock=true ;; esac
  case $f in tools/dev/hooks/gitleaks.toml|tools/dev/tools.lock) z_gitleaks_config=true ;; esac
  if is_sensitive "$f"; then z_sensitive=true; sensitive_hits+=("$f"); fi
done
if [ "$all" = 1 ]; then
  for z in $ZONES; do printf -v "z_$z" true; done
fi

result=$(for z in $ZONES; do v="z_$z"; printf '%s=%s\n' "$z" "${!v}"; done)
printf '%s\n' "$result"
[ -z "${GITHUB_OUTPUT:-}" ] || printf '%s\n' "$result" >> "$GITHUB_OUTPUT"

if [ "$summary" = 1 ] && [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
  {
    echo "### Zones touchées"
    echo
    [ -n "$reason" ] && { echo "$reason."; echo; }
    echo "${#files[@]} fichier(s) dans la plage."
    echo
    echo "| Zone | Touchée |"
    echo "|---|---|"
    for z in $ZONES; do v="z_$z"; echo "| $z | ${!v} |"; done
    if [ "${#sensitive_hits[@]}" -gt 0 ]; then
      echo
      echo "**Chemins sensibles touchés** : /security-review, relecteur neuf et archsec-ok requis."
      for f in "${sensitive_hits[@]}"; do printf -- '- `%s`\n' "$f"; done
    fi
  } >> "$GITHUB_STEP_SUMMARY"
fi
