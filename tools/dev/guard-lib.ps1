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
