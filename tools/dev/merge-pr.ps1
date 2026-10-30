# tools/dev/merge-pr.ps1 <PR> : PROVISOIRE, À SUPPRIMER PAR T3 (remplacé par cargo xtask merge : un seul
# chemin de fusion). Chemin sensible. Seule fusion permise par guard-git, qui refuse gh pr merge.
# Lancé depuis la copie principale, propre et sur main ; vérifie la PR (ouverte, base main, fusionnable,
# à jour de main), les approbations liées au SHA de tête pour un chemin sensible, ci-ok, puis fusionne par
# squash avec --match-head-commit et met la copie principale à jour par git pull --ff-only.
# Revérifie l'empreinte des crochets installés (install-hooks.ps1 -Verify, SEC0-12) avant toute fusion.
# Le contrôle de l'origine des commits de main (origin-check, SEC0-17) reste à T3.
# -DryRun : tous les contrôles en lecture, sans fusion ni mise à jour (permis depuis un worktree).
# spec: carnet architecture-securite.md SEC0-15 (révisé), §5 (chemins sensibles), §9.3
param([Parameter(Mandatory)][int]$Pr, [switch]$DryRun)
$ErrorActionPreference = 'Stop'

function Fail([string]$Message) { [Console]::Error.WriteLine("merge-pr : REFUS : $Message"); exit 1 }
function Invoke-Git { $out = & git -C $script:main @args 2>&1; if ($LASTEXITCODE -ne 0) { Fail "git $($args -join ' ') : $out" }; $out }
$sensitive = '^(\.claude/|\.github/|\.cargo/|tools/|fuzz/|deny\.toml$|\.gitleaks\.toml$|rust-toolchain\.toml$|' +
             'crates/(ts_persist|ts_net|ts_pilot|ts_client_native|ts_accounts|ts_admin|ts_protocol|ts_view)/)'

# 1. Copie principale propre, sur main, à jour par --ff-only. Le script lancé doit être le sien (pas une copie modifiée).
$common = "$(& git -C $PSScriptRoot rev-parse --path-format=absolute --git-common-dir)".Trim()
$script:main = Split-Path ([IO.Path]::GetFullPath($common)) -Parent
$here = [IO.Path]::GetFullPath("$(& git -C $PSScriptRoot rev-parse --show-toplevel)".Trim())
if (-not $DryRun -and -not $here.Equals($main, [StringComparison]::OrdinalIgnoreCase)) {
  Fail "lancer depuis la copie principale ($main\tools\dev\merge-pr.ps1), pas depuis $here."
}
if ("$(Invoke-Git branch --show-current)".Trim() -ne 'main') { Fail 'la copie principale n''est pas sur main.' }
if (Invoke-Git status --porcelain) { Fail 'la copie principale n''est pas propre.' }
if (-not $DryRun) { Invoke-Git pull --ff-only --quiet | Out-Null } else { Invoke-Git fetch --quiet origin | Out-Null }
$hooks = & pwsh -NoProfile -File (Join-Path $PSScriptRoot 'install-hooks.ps1') -Repo $main -Verify 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { Fail "crochets non conformes (install-hooks.ps1 -Verify) : $($hooks.Trim())" }

# 2. La PR.
$view = & gh pr view $Pr --json state,baseRefName,headRefOid,isDraft,mergeable,changedFiles,comments,url 2>&1
if ($LASTEXITCODE -ne 0) { Fail "gh pr view $Pr : $view" }
$p = $view | ConvertFrom-Json
$sha = $p.headRefOid
if ($p.state -ne 'OPEN') { Fail "PR $Pr à l'état $($p.state)." }
if ($p.baseRefName -ne 'main') { Fail "base $($p.baseRefName), attendu main." }
if ($p.isDraft) { Fail 'PR en brouillon.' }
if ($p.mergeable -ne 'MERGEABLE') { Fail "PR non fusionnable ($($p.mergeable))." }
$behind = & gh api "repos/{owner}/{repo}/compare/main...$sha" --jq .behind_by 2>&1
if ($LASTEXITCODE -ne 0) { Fail "comparaison avec main : $behind" }
if ([int]"$behind".Trim() -ne 0) { Fail "branche en retard de $behind commit(s) sur main : la mettre à jour." }

# 3. Chemin sensible : review-ok et archsec-ok sur le SHA de tête. La liste complète des fichiers vient de
# gh pr diff --name-only (gh pr view --json files s'arrête à 100) et doit compter changedFiles fichiers.
$paths = @(& gh pr diff $Pr --name-only 2>&1 | Where-Object { $_ })
if ($LASTEXITCODE -ne 0) { Fail "gh pr diff --name-only : $paths" }
if ($paths.Count -ne [int]$p.changedFiles) { Fail "liste des fichiers incomplète ($($paths.Count) lus, $($p.changedFiles) annoncés)." }
$isSensitive = [bool]($paths | Where-Object { $_ -match $sensitive })
if (-not $isSensitive -and ($paths | Where-Object { $_ -match '(^|/)Cargo\.(toml|lock)$' })) {
  $diff = & gh pr diff $Pr 2>&1 | Out-String
  if ($LASTEXITCODE -ne 0) { Fail "gh pr diff : $diff" }
  foreach ($chunk in ($diff -split '(?m)^(?=diff --git )')) {
    # [features] et [profile.*] d'un Cargo.toml ; crate ajoutée dans Cargo.lock.
    if ($chunk -match '^diff --git a/\S*Cargo\.toml ' -and $chunk -match '(?m)^[ +\-@].*\[(features|profile[.\]])') { $isSensitive = $true }
    if ($chunk -match '^diff --git a/\S*Cargo\.lock ' -and $chunk -match '(?m)^\+name = "') { $isSensitive = $true }
  }
}
if ($isSensitive) {
  $bodies = @($p.comments | ForEach-Object body)
  foreach ($tag in 'review-ok', 'archsec-ok') {
    if (-not ($bodies | Where-Object { $_ -match "(?m)^\s*$tag`:\s*$sha\s*$" })) { Fail "chemin sensible : commentaire « $tag`: $sha » absent." }
  }
}

# 4. ci-ok sur le SHA de tête : exigé dès que .github/workflows/ci.yml existe sur origin/main.
& git -C $main cat-file -e 'origin/main:.github/workflows/ci.yml' 2>$null
$ciExists = $LASTEXITCODE -eq 0
$ci = & gh api "repos/{owner}/{repo}/commits/$sha/check-runs" --jq '.check_runs[] | select(.name == "ci-ok") | .conclusion' 2>&1
if ($LASTEXITCODE -ne 0) { Fail "lecture des contrôles : $ci" }
$ci = "$ci".Trim()
if ($ciExists -and $ci -ne 'success') { Fail "ci-ok n'est pas vert sur $sha ($(if ($ci) { $ci } else { 'absent' }))." }
if (-not $ciExists) {
  if ($ci -and $ci -ne 'success') { Fail "ci-ok à l'état $ci sur $sha." }
  'AVERTISSEMENT : pas encore de CI (.github/workflows/ci.yml absent de origin/main) : noter la sortie de cargo xtask check et des tests dans le carnet du développement.'
}

if ($DryRun) { "Contrôles réussis pour la PR $Pr ($sha, chemin sensible : $isSensitive) ; aucune fusion (-DryRun)."; exit 0 }
& gh pr merge $Pr --squash --delete-branch --match-head-commit $sha
if ($LASTEXITCODE -ne 0) { Fail "gh pr merge a échoué." }
Invoke-Git pull --ff-only --quiet | Out-Null
"PR $Pr fusionnée ($sha) ; copie principale à jour : $(Invoke-Git log --oneline -1)"
