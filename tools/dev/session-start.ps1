# tools/dev/session-start.ps1 : emplacements de compilation (TS_CACHE\target\main|w1|w2|w3) ; ne bloque jamais (sortie 0).
# - Sans option (crochet SessionStart) : la copie principale reçoit main ; un worktree ne prend rien et voit
#   seulement l'état des emplacements. Alerte sous 40 Go libres. La sortie entre dans le contexte de la session.
# - -Acquire : un worktree prend un bail sur w1, w2 ou w3 et crée la jonction target. À lancer par une tâche
#   qui compile, avant son premier cargo : les sessions de documentation et les sous-agents ne prennent rien.
# - -Release : libère le bail de ce worktree et retire la jonction seule (le contenu de l'emplacement reste).
# spec: revues/developpement-agents.md §4.2 A9, §4.6 A34, §6
param([string]$Root, [switch]$Acquire, [switch]$Release)
$ErrorActionPreference = 'Stop'
$Grace = [TimeSpan]::FromMinutes(10)   # un bail plus jeune est tenu, même si son worktree n'est pas encore visible

function Get-JunctionTarget([string]$Path) {
  $i = Get-Item -LiteralPath $Path -Force -ErrorAction SilentlyContinue
  if ($i -and $i.LinkType -eq 'Junction') { return ("$($i.Target)" -replace '^\\\\\?\\|^\\\?\?\\', '').TrimEnd('\') }
}
function Test-SamePath([string]$A, [string]$B) { return $A -and $B -and $A.TrimEnd('\').Equals($B.TrimEnd('\'), [StringComparison]::OrdinalIgnoreCase) }
function Get-LeaseOwner([string]$Lease) { "$(Get-Content -LiteralPath $Lease -Raw -ErrorAction SilentlyContinue)".Trim() }

# Bail tenu : récent, ou son worktree existe encore (fichier .git), même si la jonction a disparu
# (git clean -xdf). git worktree remove retire .git : le bail devient alors orphelin.
function Test-LeaseHeld([string]$Lease) {
  $f = Get-Item -LiteralPath $Lease -Force -ErrorAction SilentlyContinue
  if (-not $f) { return $false }
  if ([DateTime]::UtcNow - $f.LastWriteTimeUtc -lt $Grace) { return $true }
  $owner = Get-LeaseOwner $Lease
  return [bool]($owner -and (Test-Path -LiteralPath (Join-Path $owner '.git')))
}

# Création exclusive du bail : un seul gagnant, sans course.
function New-Lease([string]$Lease, [string]$Owner) {
  try {
    $fs = [IO.File]::Open($Lease, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write)
    try { $b = [Text.Encoding]::UTF8.GetBytes($Owner); $fs.Write($b, 0, $b.Length) } finally { $fs.Dispose() }
    return $true
  } catch [IO.IOException] { return $false }
}

# Libère un bail orphelin par renommage (atomique : un seul gagnant), puis le supprime.
function Remove-OrphanLease([string]$Lease) {
  $tmp = "$Lease.stale-$([guid]::NewGuid().ToString('N'))"
  try { [IO.File]::Move($Lease, $tmp) } catch { return }
  if ([DateTime]::UtcNow - (Get-Item -LiteralPath $tmp -Force).LastWriteTimeUtc -lt $Grace) {
    try { [IO.File]::Move($tmp, $Lease) } catch { Remove-Item -LiteralPath $tmp -Force }   # bail neuf pris par erreur : rendu
  } else { Remove-Item -LiteralPath $tmp -Force }
}

function New-Junction([string]$Link, [string]$Slot) {
  New-Item -ItemType Directory -Force $Slot | Out-Null
  New-Item -ItemType Junction -Path $Link -Target $Slot | Out-Null
}

try {
  $cache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
  $slots = Join-Path $cache 'target'
  if (-not $Root) { $Root = if ($env:CLAUDE_PROJECT_DIR -and -not ($Acquire -or $Release)) { $env:CLAUDE_PROJECT_DIR } else { (Get-Location).Path } }
  $top = "$(& git -C $Root rev-parse --show-toplevel 2>$null)".Trim()
  if (-not $top) { "session-start : $Root n'est pas une copie de travail Git ; aucun emplacement."; exit 0 }
  $top = [IO.Path]::GetFullPath($top).TrimEnd('\')
  $link = Join-Path $top 'target'
  $isMain = Test-Path -LiteralPath (Join-Path $top '.git') -PathType Container   # .git est un fichier dans un worktree
  New-Item -ItemType Directory -Force $slots | Out-Null
  $mine = @('w1', 'w2', 'w3') | Where-Object { Test-SamePath (Get-LeaseOwner (Join-Path $slots "$_.lease")) $top } | Select-Object -First 1
  $current = Get-JunctionTarget $link

  if ($Release) {
    if ($current) { [IO.Directory]::Delete($link) }   # la jonction seule, sans suivre le lien
    if ($mine) { Remove-Item -LiteralPath (Join-Path $slots "$mine.lease") -Force; "Emplacement $mine libéré." }
    else { 'Aucun bail au nom de ce worktree.' }
  } elseif ($current) {
    "Emplacement de compilation : $current"
  } elseif (Test-Path -LiteralPath $link) {
    'AVERTISSEMENT : target/ est un dossier local, hors emplacement. Le supprimer, puis relancer tools/dev/session-start.ps1 -Acquire.'
  } elseif ($isMain) {
    New-Junction $link (Join-Path $slots 'main'); "Emplacement de compilation attribué : $(Join-Path $slots 'main')"
  } elseif (-not $Acquire) {
    $free = @('w1', 'w2', 'w3') | Where-Object { -not (Test-LeaseHeld (Join-Path $slots "$_.lease")) }
    $state = if ($mine) { "bail $mine déjà à ce nom (jonction absente)" } else { "$(@($free).Count) emplacement(s) libre(s) sur 3" }
    "Aucun emplacement de compilation pris ($state). Avant le premier cargo : pwsh -NoProfile -File tools/dev/session-start.ps1 -Acquire ; à la fin : -Release."
  } else {
    $slot = if ($mine) { (Get-Item -LiteralPath (Join-Path $slots "$mine.lease")).LastWriteTimeUtc = [DateTime]::UtcNow; Join-Path $slots $mine }
    if (-not $slot) {
      foreach ($n in 'w1', 'w2', 'w3') {
        $lease = Join-Path $slots "$n.lease"
        if (Test-Path -LiteralPath $lease) {
          if (Test-LeaseHeld $lease) { continue }
          Remove-OrphanLease $lease
        }
        if (New-Lease $lease $top) { $slot = Join-Path $slots $n; break }
      }
    }
    if ($slot) { New-Junction $link $slot; "Emplacement de compilation attribué : $slot" }
    else { 'AVERTISSEMENT : les 3 emplacements sont pris. Ne pas compiler ici : attendre, ou passer la tâche au cloud.' }
  }

  # Repli de §6.3 point 4 : au-delà de 100 caractères de racine, link.exe dépasse MAX_PATH par la jonction
  # (LNK1104 constaté) ; la compilation passe alors par CARGO_TARGET_DIR vers l'emplacement.
  $target = Get-JunctionTarget $link
  if ($target -and $top.Length -gt 100) {
    $quoted = "'" + ($target -replace "'", "'\''") + "'"
    if ($env:CLAUDE_ENV_FILE) { Add-Content -LiteralPath $env:CLAUDE_ENV_FILE "export CARGO_TARGET_DIR=$quoted" }
    "Chemin long ($($top.Length) car.) : compiler avec CARGO_TARGET_DIR=$target (Bash : export CARGO_TARGET_DIR=$quoted ; PowerShell : `$env:CARGO_TARGET_DIR = '$($target -replace "'", "''")')."
  }
  $root = [IO.Path]::GetPathRoot([IO.Path]::GetFullPath($cache))
  $free = [math]::Floor([IO.DriveInfo]::new($root).AvailableFreeSpace / 1GB)
  if ($free -lt 40) { "AVERTISSEMENT : $free Go libres (< 40). Lancer cargo xtask disk --clean ; ne créer aucun worktree." }
} catch { "session-start : $($_.Exception.Message)" }
exit 0
