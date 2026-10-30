# tools/dev/guard-lib.ps1 : fonctions communes à guard-main.ps1 et guard-git.ps1 (chargé par dot-sourcing).
# spec: revues/developpement-agents.md §6.2 ; carnet architecture-securite.md SEC0-14, SEC0-35

# Chemin Windows absolu à partir d'un chemin saisi (relatif, ~, /c/... de Git Bash, barres obliques).
function ConvertTo-FullPath([string]$Path, [string]$Base) {
  if ([string]::IsNullOrWhiteSpace($Path)) { return $null }
  $p = $Path.Trim()
  if ($p -match '^/([a-zA-Z])(/|$)') { $p = $Matches[1] + ':\' + $p.Substring(2).TrimStart('/') }
  if ($p -eq '~' -or $p.StartsWith('~/') -or $p.StartsWith('~\')) { $p = $HOME + $p.Substring(1) }
  if (-not $Base) { $Base = (Get-Location).Path }
  try { return [IO.Path]::GetFullPath(($p -replace '/', '\'), $Base).TrimEnd('\') } catch { return $null }
}

# Chemin canonique pour les gardes : variables du dossier personnel développées, préfixes \\?\ et \\.\
# retirés, flux NTFS (« : » après le lecteur) retirés, points et espaces finaux retirés, noms 8.3 résolus.
# Lève une exception pour un partage UNC local ou administratif, ou pour un nom 8.3 irrésoluble.
function ConvertTo-CanonicalPath([string]$Path, [string]$Base) {
  if ([string]::IsNullOrWhiteSpace($Path)) { return $null }
  $p = $Path.Trim()
  foreach ($v in '${HOME}', '$HOME', '$env:HOME', '$env:USERPROFILE', '${env:USERPROFILE}', '%USERPROFILE%') {
    $p = $p.Replace($v, $HOME, [StringComparison]::OrdinalIgnoreCase)
  }
  $p = $p -replace '/', '\'
  $p = $p -replace '^\\\\[?.]\\UNC\\', '\\' -replace '^\\\\[?.]\\', ''
  if ($p -match '^\\\\([^\\]+)\\([^\\]+)') {
    $hostRe = '^(?i)(localhost|127\.\d+\.\d+\.\d+|\[?::1\]?|' + [regex]::Escape("$env:COMPUTERNAME") + ')$'
    if ($Matches[1] -match $hostRe -or $Matches[2] -match '^[a-zA-Z]\$$') { throw "partage UNC local ou administratif : $Path" }
  }
  $p = ConvertTo-FullPath $p $Base
  if (-not $p) { throw "chemin illisible : $Path" }
  $colon = $p.IndexOf(':', 2)
  if ($colon -ge 0) { $p = $p.Substring(0, $colon) }   # flux NTFS : fichier::$DATA, fichier:flux
  $p = (($p -split '\\') | ForEach-Object { if ($_ -match '^[a-zA-Z]:$') { $_ } else { $_.TrimEnd('. ') } }) -join '\'
  if ($p -match '~\d') {   # nom court 8.3 : résolu par le système, sinon refus
    if (-not ('TsGuard.Native' -as [type])) {
      Add-Type -Namespace TsGuard -Name Native -MemberDefinition '[DllImport("kernel32.dll", CharSet = CharSet.Unicode)] public static extern uint GetLongPathName(string s, System.Text.StringBuilder l, uint n);'
    }
    $known = if (Test-Path -LiteralPath $p) { $p } else { Get-ExistingDir $p }
    $sb = [Text.StringBuilder]::new(32768)
    if ($known -and [TsGuard.Native]::GetLongPathName($known, $sb, 32768) -gt 0) { $p = $sb.ToString() + $p.Substring($known.Length) }
    if ($p -match '~\d') { throw "nom court (8.3) irrésoluble : $Path" }
  }
  return $p.TrimEnd('\')
}

# Vrai si $Path est $Root ou se trouve dessous (comparaison sans casse, séparateur exigé).
function Test-UnderPath([string]$Path, [string]$Root) {
  if (-not $Path -or -not $Root) { return $false }
  $r = $Root.TrimEnd('\')
  return $Path.Equals($r, [StringComparison]::OrdinalIgnoreCase) -or
         $Path.StartsWith($r + '\', [StringComparison]::OrdinalIgnoreCase)
}

# Premier dossier existant en remontant depuis $Path.
function Get-ExistingDir([string]$Path) {
  $d = $Path
  while ($d -and -not (Test-Path -LiteralPath $d -PathType Container)) { $d = Split-Path $d -Parent }
  return $d
}

# Branche courante de la copie de travail qui contient $Dir ('' si détachée, $null hors dépôt).
function Get-Branch([string]$Dir, [string[]]$GitArgs = @()) {
  $d = Get-ExistingDir $Dir
  if (-not $d) { return $null }
  $out = & git -C $d @GitArgs branch --show-current 2>$null
  if ($LASTEXITCODE -ne 0) { return $null }
  return "$out".Trim()
}

# Fichiers de réglages de Claude Code protégés par SEC0-35 : .claude/settings(.local).json ;
# avec -Loose (commandes shell), aussi le nom seul, sans dossier (cd .claude puis écriture).
function Test-SettingsFile([string]$Path, [switch]$Loose) {
  if ($Path -match '(?i)(^|[\\/])\.claude[\\/]+settings(\.local)?\.json$') { return $true }
  return $Loose -and $Path -match '(?i)^settings(\.local)?\.json$'
}

# Exception de SEC0-35 : seule une branche claude/outillage-* peut toucher aux réglages.
function Test-ToolingBranch([string]$Branch) {
  return [bool]($Branch -and $Branch -like 'claude/outillage-*')
}

function Read-HookInput {
  $raw = [Console]::In.ReadToEnd()
  if ([string]::IsNullOrWhiteSpace($raw)) { throw 'entrée du crochet vide' }
  return $raw | ConvertFrom-Json
}

function Exit-Deny([string]$Message) {
  [Console]::Error.WriteLine($Message)
  exit 2
}
