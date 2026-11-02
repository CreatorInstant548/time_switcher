# tools/dev/test-slots.ps1 : essais de session-start.ps1 et vérifications bloquantes de dev-agents §6.3.
# 1. Logique des baux, dans un TS_CACHE jetable : main automatique, -Acquire, -Release, idempotence,
#    3 emplacements pris, bail orphelin, bail tenu sans jonction, repli CARGO_TARGET_DIR, et invalidation des
#    empreintes des crates de l'espace de travail quand un emplacement change de worktree (#56).
# 2. Avec -Real : dans les vrais emplacements (TS_CACHE\target), un worktree jetable d'un dépôt jetable
#    reçoit un emplacement, compile un projet Cargo jetable à travers la jonction (chemin long), puis
#    git clean -xdf et git worktree remove ne doivent pas abîmer l'emplacement. Le bail est libéré à la fin.
param([switch]$Real)
$ErrorActionPreference = 'Stop'
$dev = $PSScriptRoot
$cache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
$tmp = Join-Path $cache "tmp\slots-$([guid]::NewGuid().ToString('N').Substring(0, 8))"
$fail = 0; $n = 0
function Check([string]$Name, [bool]$Ok, [string]$Detail = '') {
  $script:n++; if ($Ok) { "  ok  $Name" } else { $script:fail++; "ÉCHEC $Name $Detail" }
}
function Start-Session([string]$Root, [string]$Cache, [switch]$Acquire, [switch]$Release) {
  $old = $env:TS_CACHE; $env:TS_CACHE = $Cache
  $opts = @(); if ($Acquire) { $opts += '-Acquire' }; if ($Release) { $opts += '-Release' }
  try { & pwsh -NoProfile -File (Join-Path $dev 'session-start.ps1') -Root $Root @opts 2>&1 | Out-String } finally { $env:TS_CACHE = $old }
}
function Get-Link([string]$Path) { $i = Get-Item -LiteralPath $Path -Force -ErrorAction SilentlyContinue; if ($i) { "$($i.LinkType)|$($i.Target)" } }
function New-Repo([string]$Dir) {
  & git init -q -b claude/base $Dir
  & git -C $Dir config user.name essai; & git -C $Dir config user.email essai@example.invalid
  Set-Content (Join-Path $Dir '.gitignore') "/target`n" -NoNewline
  & git -C $Dir add .gitignore; & git -C $Dir commit -q -m 'chore(essai): base'
}

try {
  # --- 1. Logique des baux (cache jetable) ---
  $fc = Join-Path $tmp 'cache'; $repo = Join-Path $tmp 'repo'
  New-Item -ItemType Directory -Force $repo | Out-Null; New-Repo $repo
  $wts = 1..4 | ForEach-Object { $w = Join-Path $tmp "wt$_"; & git -C $repo worktree add -q -b "claude/w$_" $w 2>&1 | Out-Null; $w }
  $o = Start-Session $repo $fc
  Check 'copie principale : emplacement main, sans option' ((Get-Link "$repo\target") -like "Junction|*\target\main") $o
  $o = Start-Session $wts[0] $fc
  Check 'worktree sans -Acquire : rien pris, rappel de la commande' (($o -match '-Acquire') -and ($o -match '3 emplacement') -and -not (Test-Path "$($wts[0])\target")) $o
  foreach ($k in 0..2) {
    $o = Start-Session $wts[$k] $fc -Acquire
    Check "worktree $($k + 1) : -Acquire donne w$($k + 1)" ((Get-Link "$($wts[$k])\target") -like "Junction|*\target\w$($k + 1)") $o
  }
  $o = Start-Session $wts[0] $fc
  Check 'idempotent (2e démarrage)' ($o -match 'Emplacement de compilation : .*w1') $o
  $o = Start-Session $wts[3] $fc -Acquire
  Check '3 emplacements pris : avertissement, aucune jonction' (($o -match 'les 3 emplacements sont pris') -and -not (Test-Path "$($wts[3])\target")) $o
  # Bail orphelin : worktree 2 retiré (git laisse le dossier et la jonction, sans .git), bail vieilli.
  & git -C $repo worktree remove --force $wts[1] 2>&1 | Out-Null
  (Get-Item "$fc\target\w2.lease").LastWriteTimeUtc = [DateTime]::UtcNow.AddHours(-1)
  $o = Start-Session $wts[3] $fc -Acquire
  Check 'bail orphelin libéré et repris' (((Get-Link "$($wts[3])\target") -like '*\w2') -and ((Get-Content "$fc\target\w2.lease" -Raw) -eq $wts[3])) $o
  # Bail ancien dont le worktree existe, jonction retirée (git clean -xdf) : toujours tenu.
  [IO.Directory]::Delete("$($wts[0])\target")
  (Get-Item "$fc\target\w1.lease").LastWriteTimeUtc = [DateTime]::UtcNow.AddHours(-1)
  $w5 = Join-Path $tmp 'wt5'; & git -C $repo worktree add -q -b claude/w5 $w5 2>&1 | Out-Null
  $o = Start-Session $w5 $fc -Acquire
  Check 'bail tenu sans jonction (worktree présent) : non repris' ((Get-Content "$fc\target\w1.lease" -Raw) -eq $wts[0] -and -not (Test-Path "$w5\target")) $o
  $o = Start-Session $wts[0] $fc -Acquire
  Check 'le worktree reprend son propre bail' ((Get-Link "$($wts[0])\target") -like '*\w1') $o
  Set-Content "$fc\target\w1\garde.txt" 'x'
  $o = Start-Session $wts[0] $fc -Release
  Check '-Release : bail libéré, jonction retirée, contenu intact' ((-not (Test-Path "$fc\target\w1.lease")) -and -not (Test-Path "$($wts[0])\target") -and (Test-Path "$fc\target\w1\garde.txt")) $o
  # #56: when a slot changes worktree, the Cargo fingerprints of the workspace crates (and only those) are deleted;
  # a junction is never followed; the same worktree loses nothing.
  $fps = @("$fc\target\w1\debug\.fingerprint", "$fc\target\w1\fuzz-ws\x86_64-pc-windows-msvc\debug\.fingerprint")
  $kept = @('essai_ws_extra-0123456789abcdef', 'essai_ws-short', 'serde-0123456789abcdef')
  $mine = @('essai_ws-0123456789abcdef', 'essai_ws-fedcba9876543210')
  function Seed-Fingerprints([string[]]$Names) {
    foreach ($d in $fps) { foreach ($n in $Names) { New-Item -ItemType Directory -Force "$d\$n" | Out-Null; Set-Content "$d\$n\unit.json" 'x' } }
  }
  function Count-Fingerprints([string[]]$Names) { @($fps | ForEach-Object { $d = $_; $Names | Where-Object { Test-Path "$d\$_" } }).Count }
  New-Item -ItemType Directory -Force "$($wts[0])\crates\a" | Out-Null
  Set-Content "$($wts[0])\Cargo.toml" "[workspace]`nmembers = [`"crates/a`"]`n"
  Set-Content "$($wts[0])\crates\a\Cargo.toml" "[package]`nkeywords = [`"x`"]`nname = `"essai_ws`"`nversion = `"0.0.0`"`n[dependencies]`nname = `"pas_un_paquet`"`n"
  Seed-Fingerprints ($mine + $kept)
  $outside = Join-Path $tmp 'outside-fingerprints'; New-Item -ItemType Directory -Force $outside | Out-Null; Set-Content "$outside\sentinel.txt" 'x'
  New-Item -ItemType Junction -Path "$($fps[0])\essai_ws-aaaaaaaaaaaaaaaa" -Target $outside | Out-Null
  Set-Content "$fc\target\w1\last-worktree.txt" $wts[3] -NoNewline   # the last to build here: another worktree
  $o = Start-Session $wts[0] $fc -Acquire
  Check '#56 worktree change: fingerprints of the worktree crates deleted, the rest kept' (
    (Count-Fingerprints $mine) -eq 0 -and (Count-Fingerprints $kept) -eq $kept.Count * $fps.Count -and -not (Test-Path "$($fps[0])\essai_ws-aaaaaaaaaaaaaaaa")) $o
  Check '#56 a junction in .fingerprint is removed, not followed' (Test-Path "$outside\sentinel.txt") $o
  Check '#56 the last worktree is recorded in the slot' ((Get-Content "$fc\target\w1\last-worktree.txt" -Raw) -eq $wts[0]) $o
  Seed-Fingerprints $mine
  $o = Start-Session $wts[0] $fc -Release
  $o = Start-Session $wts[0] $fc -Acquire
  Check '#56 same worktree: nothing deleted' ((Count-Fingerprints $mine) -eq $mine.Count * $fps.Count -and $o -notmatch 'removed') $o
  $o = Start-Session $wts[0] $fc -Release
  $w6 = Join-Path $tmp 'wt6'; & git -C $repo worktree add -q -b claude/w6 $w6 2>&1 | Out-Null
  Set-Content "$w6\Cargo.toml" "[package]`nname = `"essai_ws6`"`nversion = `"0.0.0`"`n"
  Seed-Fingerprints @('essai_ws6-0123456789abcdef')
  $o = Start-Session $w6 $fc -Acquire
  Check '#56 other worktree: its crates are invalidated, dependencies stay' (
    (Count-Fingerprints @('essai_ws6-0123456789abcdef')) -eq 0 -and (Count-Fingerprints $kept) -eq $kept.Count * $fps.Count -and (Get-Content "$fc\target\w1\last-worktree.txt" -Raw) -eq $w6) $o
  $o = Start-Session $w6 $fc -Release

  # Repli CARGO_TARGET_DIR (racine > 100 car.) : guillemet simple échappé dans CLAUDE_ENV_FILE.
  $qc = Join-Path $tmp "ca'che"; $long = Join-Path $tmp ('l' * (101 - $tmp.Length))
  & git -C $repo worktree add -q -b claude/long $long 2>&1 | Out-Null
  $envFile = Join-Path $tmp 'env.sh'; $env:CLAUDE_ENV_FILE = $envFile
  try { $o = Start-Session $long $qc -Acquire } finally { Remove-Item Env:CLAUDE_ENV_FILE }
  $line = "$(Get-Content $envFile -Raw)".Trim()
  $want = "export CARGO_TARGET_DIR='" + ((Join-Path $qc 'target\w1') -replace "'", "'\''") + "'"
  Check 'repli CARGO_TARGET_DIR : guillemet échappé' ($line -eq $want) "$line / $want $o"

  # --- 2. Vérifications bloquantes de §6.3, dans les vrais emplacements ---
  if ($Real) {
    # Racine de 90 caractères : celle des worktrees de l'application (.claude\worktrees\agent-<id>) plus 20 de marge.
    $wt = Join-Path $tmp ('w' * (90 - $tmp.Length - 1))
    & git -C $repo worktree add -q -b claude/essai-long $wt 2>&1 | Out-Null
    $o = Start-Session $wt $cache -Acquire
    $slot = ("$((Get-Item -LiteralPath "$wt\target" -Force).Target)" -replace '^\\\\\?\\', '').TrimEnd('\')
    Check "attribution d'un emplacement réel ($slot)" ($slot -match '\\target\\w[1-3]$') $o
    $lease = "$slot.lease"
    New-Item -ItemType Directory -Force "$wt\src\$('m' * 30)" | Out-Null
    Set-Content "$wt\Cargo.toml" "[package]`nname = `"essai_jonction_chemins_longs_$('a' * 20)`"`nversion = `"0.0.0`"`nedition = `"2021`"`n[workspace]`n"
    Set-Content "$wt\src\main.rs" "mod $('m' * 30);`nfn main() { println!(`"{}`", $('m' * 30)::f()); }`n"
    Set-Content "$wt\src\$('m' * 30)\mod.rs" "pub fn f() -> u32 { 42 }`n#[cfg(test)] mod tests { #[test] fn t() { assert_eq!(super::f(), 42); } }`n"
    & git -C $wt add -A; & git -C $wt commit -q -m 'chore(essai): projet'
    $before = @(Get-ChildItem -LiteralPath $slot -Force | ForEach-Object Name)   # contenu antérieur, laissé en place
    $sentinel = Join-Path $slot 'sentinelle-essai.txt'; Set-Content $sentinel 'essai'
    function Build([string]$Dir, [string]$TargetDir) {
      Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
      if ($TargetDir) { $env:CARGO_TARGET_DIR = $TargetDir }
      Push-Location $Dir
      try { $out = & cargo test --quiet 2>&1 | Out-String; $code = $LASTEXITCODE }
      finally { Pop-Location; Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue }
      $longest = (Get-ChildItem -LiteralPath "$Dir\target" -Recurse -Force -ErrorAction SilentlyContinue | ForEach-Object { $_.FullName.Length } | Measure-Object -Maximum).Maximum
      return @($code, $longest, $out)
    }
    $b = Build $wt $null
    Check "compilation et tests à travers la jonction (chemin le plus long : $($b[1]) car.)" ($b[0] -eq 0) $b[2]
    $artefacts = (Get-ChildItem -LiteralPath $slot -Recurse -File -Force | Measure-Object).Count
    Check "artefacts dans l'emplacement ($artefacts fichiers)" ($artefacts -gt 2)

    # Chemin très long (> 260 car.) : constat par la jonction, puis repli CARGO_TARGET_DIR vers l'emplacement.
    $deep = Join-Path $wt ('x' * 60 + '\' + 'y' * 60 + '\' + 'z' * 40)
    New-Item -ItemType Directory -Force $deep | Out-Null
    Copy-Item -Recurse "$wt\src", "$wt\Cargo.toml" $deep
    (Get-Content "$deep\Cargo.toml" -Raw) -replace 'name = "essai_', 'name = "profond_' | Set-Content "$deep\Cargo.toml"   # édition de liens neuve
    New-Item -ItemType Junction -Path "$deep\target" -Target $slot | Out-Null
    $b = Build $deep $null
    "  --  chemin très long par la jonction ($($b[1]) car.) : $(if ($b[0] -eq 0) { 'compile' } elseif ($b[2] -match 'LNK1104') { 'échec de link.exe (LNK1104, MAX_PATH)' } else { 'échec' })"
    [IO.Directory]::Delete("$deep\target")
    $b = Build $deep $slot
    Check 'chemin très long : repli CARGO_TARGET_DIR compile' ($b[0] -eq 0) $b[2]
    Remove-Item -LiteralPath (Join-Path $wt ('x' * 60)) -Recurse -Force
    $artefacts = (Get-ChildItem -LiteralPath $slot -Recurse -File -Force | Measure-Object).Count

    $out = & git -C $wt clean -xdf 2>&1 | Out-String
    $after = (Get-ChildItem -LiteralPath $slot -Recurse -File -Force | Measure-Object).Count
    Check "git clean -xdf : emplacement intact ($after/$artefacts fichiers, jonction : $(if (Test-Path "$wt\target") { 'restée' } else { 'retirée' }))" (
      (Test-Path $sentinel) -and $after -eq $artefacts) $out
    $o = Start-Session $wt $cache -Acquire
    Check 'après git clean, le même emplacement est repris' ((Get-Link "$wt\target") -like "Junction|$slot") $o

    $out = & git -C $repo worktree remove $wt 2>&1 | Out-String
    $how = 'git worktree remove'
    if ($LASTEXITCODE -ne 0) { $out += & git -C $repo worktree remove --force $wt 2>&1 | Out-String; $how += ' --force' }
    $left = (Get-Item -LiteralPath "$wt\target" -Force -ErrorAction SilentlyContinue).LinkType -eq 'Junction'
    $after = (Get-ChildItem -LiteralPath $slot -Recurse -File -Force | Measure-Object).Count
    $state = if ($left) { 'retiré ; dossier et jonction laissés' } elseif (Test-Path $wt) { 'resté' } else { 'supprimé' }
    Check "$how : emplacement intact ($after/$artefacts fichiers, worktree $state)" (
      (Test-Path $sentinel) -and $after -eq $artefacts -and -not (Test-Path "$wt\.git")) $out
    if ($left) { [IO.Directory]::Delete("$wt\target") }   # la jonction seule, sans suivre le lien

    # Nettoyage de l'emplacement essayé (fichiers de l'essai seulement), puis libération du bail : dans cet ordre,
    # pour ne jamais supprimer les fichiers d'une session qui aurait repris le bail entre-temps.
    if ((Get-Content -LiteralPath $lease -Raw).Trim() -eq [IO.Path]::GetFullPath($wt)) {
      Get-ChildItem -LiteralPath $slot -Force | Where-Object { $_.Name -notin $before } | Remove-Item -Recurse -Force
      Remove-Item -LiteralPath $lease -Force
    }
    Check 'bail libéré' (-not (Test-Path $lease))
  }
} finally {
  Remove-Item -LiteralPath $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
"test-slots : $($n - $fail)/$n essais conformes."
exit [int]($fail -gt 0)
