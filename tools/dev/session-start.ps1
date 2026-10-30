# tools/dev/session-start.ps1 : crochet SessionStart de Claude Code ; idempotent, ne bloque jamais (sortie 0).
# Attribue un emplacement de compilation (TS_CACHE\target\main pour la copie principale, w1 à w3 par bail
# pour les worktrees), crée la jonction target, et alerte sous 40 Go libres. Sa sortie entre dans le contexte.
# spec: revues/developpement-agents.md §4.2 A9, §4.6 A34, §6
param([string]$Root)   # copie de travail ; par défaut, celle du dossier du projet (ou du dossier courant)
$ErrorActionPreference = 'Stop'
$Grace = [TimeSpan]::FromMinutes(10)   # un bail plus jeune est tenu, même sans jonction (attribution en cours)

function Get-JunctionTarget([string]$Path) {
  $i = Get-Item -LiteralPath $Path -Force -ErrorAction SilentlyContinue
  if ($i -and $i.LinkType -eq 'Junction') { return ("$($i.Target)" -replace '^\\\\\?\\|^\\\?\?\\', '').TrimEnd('\') }
}
function Test-SamePath([string]$A, [string]$B) { return $A -and $B -and $A.TrimEnd('\').Equals($B.TrimEnd('\'), [StringComparison]::OrdinalIgnoreCase) }

# Bail tenu : récent, ou son worktree existe encore (fichier .git) et sa jonction target pointe vers
# l'emplacement. git worktree remove laisse le dossier et la jonction, mais retire .git.
function Test-LeaseHeld([string]$Lease, [string]$Slot) {
  $f = Get-Item -LiteralPath $Lease -Force -ErrorAction SilentlyContinue
  if (-not $f) { return $false }
  if ([DateTime]::UtcNow - $f.LastWriteTimeUtc -lt $Grace) { return $true }
  $owner = "$(Get-Content -LiteralPath $Lease -Raw -ErrorAction SilentlyContinue)".Trim()
  return [bool]($owner -and (Test-Path -LiteralPath (Join-Path $owner '.git')) -and
                (Test-SamePath (Get-JunctionTarget (Join-Path $owner 'target')) $Slot))
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

try {
  $cache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
  $slots = Join-Path $cache 'target'
  if (-not $Root) { $Root = if ($env:CLAUDE_PROJECT_DIR) { $env:CLAUDE_PROJECT_DIR } else { (Get-Location).Path } }
  $top = "$(& git -C $Root rev-parse --show-toplevel 2>$null)".Trim()
  if (-not $top) { "session-start : $Root n'est pas une copie de travail Git ; aucun emplacement attribué."; exit 0 }
  $top = [IO.Path]::GetFullPath($top).TrimEnd('\')
  $link = Join-Path $top 'target'
  $isMain = Test-Path -LiteralPath (Join-Path $top '.git') -PathType Container   # .git est un fichier dans un worktree
  New-Item -ItemType Directory -Force $slots | Out-Null

  $current = Get-JunctionTarget $link
  if ($current) {
    $name = Split-Path $current -Leaf
    $lease = Join-Path $slots "$name.lease"
    if (-not $isMain -and $name -match '^w[1-3]$' -and -not (Test-Path -LiteralPath $lease)) { [void](New-Lease $lease $top) }   # bail perdu : repris
    "Emplacement de compilation : $current"
  } elseif (Test-Path -LiteralPath $link) {
    'AVERTISSEMENT : target/ est un dossier local, hors emplacement. Le supprimer puis relancer tools/dev/session-start.ps1.'
  } else {
    $slot = $null
    if ($isMain) { $slot = Join-Path $slots 'main' }
    else {
      foreach ($n in 'w1', 'w2', 'w3') {   # bail déjà au nom de ce worktree (jonction retirée par git clean -xdf) : repris
        $lease = Join-Path $slots "$n.lease"
        if ((Test-Path -LiteralPath $lease) -and (Test-SamePath "$(Get-Content -LiteralPath $lease -Raw)".Trim() $top)) {
          (Get-Item -LiteralPath $lease).LastWriteTimeUtc = [DateTime]::UtcNow; $slot = Join-Path $slots $n; break
        }
      }
    }
    if (-not $slot -and -not $isMain) {
      foreach ($n in 'w1', 'w2', 'w3') {
        $lease = Join-Path $slots "$n.lease"; $dir = Join-Path $slots $n
        if (Test-Path -LiteralPath $lease) {
          if (Test-LeaseHeld $lease $dir) { continue }
          Remove-OrphanLease $lease
        }
        if (New-Lease $lease $top) { $slot = $dir; break }
      }
    }
    if ($slot) {
      New-Item -ItemType Directory -Force $slot | Out-Null
      New-Item -ItemType Junction -Path $link -Target $slot | Out-Null
      "Emplacement de compilation attribué : $slot"
    } else {
      'AVERTISSEMENT : les 3 emplacements sont pris. Ne pas compiler ici : attendre, ou passer la tâche au cloud.'
    }
  }
  # Repli de §6.3 point 4 : au-delà de 100 caractères de racine, link.exe dépasse MAX_PATH par la jonction
  # (LNK1104 constaté) ; les commandes Bash de la session reçoivent alors CARGO_TARGET_DIR vers l'emplacement.
  $target = Get-JunctionTarget $link
  if ($target -and $top.Length -gt 100) {
    if ($env:CLAUDE_ENV_FILE) { Add-Content -LiteralPath $env:CLAUDE_ENV_FILE "export CARGO_TARGET_DIR='$target'" }
    "Chemin long ($($top.Length) car.) : compiler avec CARGO_TARGET_DIR=$target (déjà réglé pour Bash ; en PowerShell : `$env:CARGO_TARGET_DIR='$target')."
  }
  $root = [IO.Path]::GetPathRoot([IO.Path]::GetFullPath($cache))
  $free = [math]::Floor([IO.DriveInfo]::new($root).AvailableFreeSpace / 1GB)
  if ($free -lt 40) { "AVERTISSEMENT : $free Go libres (< 40). Lancer cargo xtask disk --clean ; ne créer aucun worktree." }
} catch { "session-start : $($_.Exception.Message)" }
exit 0
