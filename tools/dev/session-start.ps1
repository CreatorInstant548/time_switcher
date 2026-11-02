# tools/dev/session-start.ps1 : emplacements de compilation (TS_CACHE\target\main|w1|w2|w3) ; ne bloque jamais (sortie 0).
# - Sans option (crochet SessionStart) : la copie principale reçoit main ; un worktree ne prend rien et voit
#   seulement l'état des emplacements. Alerte sous 40 Go libres. La sortie entre dans le contexte de la session.
# - -Acquire : un worktree prend un bail sur w1, w2 ou w3 et crée la jonction target. À lancer par une tâche
#   qui compile, avant son premier cargo : les sessions de documentation et les sous-agents ne prennent rien.
#   If the last worktree that built in the slot is not this one (or is unknown), the Cargo fingerprints of the
#   workspace crates are deleted (#56, see Clear-WorkspaceFingerprints), so that no output of another worktree is reused.
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

# ---- #56 : artefacts périmés quand un emplacement change de worktree ------------------------------------------
# A slot is shared by successive worktrees. Cargo names the artefacts of workspace crates by their path relative
# to the workspace root, so two worktrees produce the same file names, and a fingerprint is only an mtime check:
# an output written from another worktree can look fresh. It then gets reused with the other worktree's sources
# (wrong rlib in a doctest), or with that worktree's absolute path baked in (env!("CARGO_MANIFEST_DIR"),
# env!("CARGO_BIN_EXE_<bin>"), file!()) in a test binary. Third-party crates do not depend on the worktree.
# Fix: the slot records the last worktree that used it; when another one takes it, the `.fingerprint` entries of
# the workspace crates (and only those) are deleted, so Cargo rebuilds them and leaves dependencies cached. Done
# here rather than by `cargo clean -p`: no Cargo run, no lock, and only this slot, which the caller holds.
$OwnerMarker = 'last-worktree.txt'   # in the slot itself, so that wiping the slot (xtask disk --clean) resets it

# Package names of every Cargo.toml of the worktree (tracked or not yet added, never ignored: no target/).
function Get-WorkspacePackages([string]$Top) {
  $names = @{}
  foreach ($f in @(& git -C $Top ls-files --cached --others --exclude-standard -- '*Cargo.toml' 2>$null)) {
    $text = Get-Content -LiteralPath (Join-Path $Top "$f") -Raw -ErrorAction SilentlyContinue
    if ($text -match '(?ms)^\[package\][ \t]*\r?\n(.*?)(?=^\[|\z)' -and $Matches[1] -match '(?m)^name[ \t]*=[ \t]*"([^"]+)"') { $names[$Matches[1]] = $true }
  }
  return @($names.Keys)
}

# `.fingerprint` directories of a slot: <slot>/<profile>/, <slot>/<triple>/<profile>/, <slot>/<tool>/... (depth 4).
# Links are never entered.
function Get-FingerprintDirs([string]$Dir, [int]$Depth = 4) {
  foreach ($d in Get-ChildItem -LiteralPath $Dir -Directory -Force -ErrorAction SilentlyContinue) {
    if ($d.Attributes -band [IO.FileAttributes]::ReparsePoint) { continue }
    if ($d.Name -eq '.fingerprint') { $d.FullName }
    elseif ($Depth -gt 1 -and $d.Name -notin 'deps', 'build', 'incremental', 'examples', 'doc') { Get-FingerprintDirs $d.FullName ($Depth - 1) }
  }
}

# Deletes <package>-<16 hex digits> entries of every .fingerprint directory of the slot; returns the number deleted,
# or -1 if one could not be (the caller then keeps the slot marked as unknown, so that the next -Acquire retries).
function Clear-WorkspaceFingerprints([string]$Slot, [string[]]$Packages) {
  if (-not $Packages) { return 0 }
  $re = '^(?:' + (($Packages | ForEach-Object { [regex]::Escape($_) }) -join '|') + ')-[0-9a-f]{16}$'
  $count = 0; $failed = $false
  foreach ($fp in @(Get-FingerprintDirs $Slot)) {
    foreach ($e in Get-ChildItem -LiteralPath $fp -Directory -Force -ErrorAction SilentlyContinue | Where-Object { $_.Name -match $re }) {
      try {
        # A link is removed itself, never followed.
        if ($e.Attributes -band [IO.FileAttributes]::ReparsePoint) { [IO.Directory]::Delete($e.FullName) } else { [IO.Directory]::Delete($e.FullName, $true) }
        $count++
      } catch { $failed = $true }
    }
  }
  if ($failed) { return -1 }
  return $count
}

# Called after -Acquire, once the slot is ours.
function Sync-SlotOwner([string]$Slot, [string]$Top) {
  $marker = Join-Path $Slot $OwnerMarker
  if (Test-SamePath (Get-LeaseOwner $marker) $Top) { return }
  $n = Clear-WorkspaceFingerprints $Slot (Get-WorkspacePackages $Top)
  if ($n -lt 0) { 'WARNING: stale workspace artefacts could not all be removed from the slot (file in use?). Retry -Acquire, or touch the source file of a failing test (#56).'; return }
  [IO.File]::WriteAllText($marker, $Top)
  "Slot last used by another worktree (or unknown): $n workspace-crate fingerprint(s) removed, those crates will rebuild; dependencies stay cached."
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
  if ($Acquire -and -not $isMain -and $target -and (Test-SamePath (Get-LeaseOwner (Join-Path $slots "$(Split-Path $target -Leaf).lease")) $top)) {
    Sync-SlotOwner $target $top   # only a slot whose lease is in this worktree's name
  }
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
