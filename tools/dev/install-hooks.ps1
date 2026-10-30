# tools/dev/install-hooks.ps1 : installation provisoire des crochets Git (remplacée en T3 par cargo xtask hooks install).
# Copie tools/dev/hooks/* et tools.lock dans TS_CACHE\hooks\<sha256 du jeu>\, règle core.hooksPath dans la
# configuration commune du dépôt et écrit l'empreinte attendue dans TS_CACHE\hooks\current.sha256.
# -Verify contrôle seulement l'installation (code 1 en cas d'écart). Refuse si gitleaks manque ou diffère.
# spec: carnet architecture-securite.md SEC0-06, SEC0-12
param([string]$Repo = (Join-Path $PSScriptRoot '..\..'), [switch]$Verify)
$ErrorActionPreference = 'Stop'

function Get-Sha256([string]$Path) { (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant() }

# Empreinte du jeu de crochets : SHA-256 de la liste « nom NUL sha256 LF », triée par nom (ordre ordinal).
function Get-SetHash([string]$Dir) {
  $names = [string[]]@(Get-ChildItem -LiteralPath $Dir -File | ForEach-Object Name)
  [Array]::Sort($names, [StringComparer]::Ordinal)
  $lines = $names | ForEach-Object { "$_`0$(Get-Sha256 (Join-Path $Dir $_))`n" }
  $sha = [Security.Cryptography.SHA256]::Create()
  return -join ($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes(-join $lines)) | ForEach-Object { $_.ToString('x2') })
}

function Get-LockValue([string]$Lock, [string]$Key) {
  $m = Select-String -LiteralPath $Lock -Pattern ('^' + [regex]::Escape($Key) + '\s*=\s*"(.*)"') | Select-Object -First 1
  if ($m) { $m.Matches[0].Groups[1].Value }
}

try {
  $cache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
  $hooksRoot = Join-Path $cache 'hooks'
  $Repo = (Resolve-Path -LiteralPath $Repo).Path
  $lock = Join-Path $PSScriptRoot 'tools.lock'

  # gitleaks présent et conforme, sinon refus : un contrôle ne réussit jamais en silence (SEC0-06).
  $gl = Get-LockValue $lock 'gitleaks.path'; $expected = Get-LockValue $lock 'gitleaks.sha256'
  if (-not $gl -or -not (Test-Path -LiteralPath $gl)) { throw "gitleaks introuvable ($gl)." }
  if ((Get-Sha256 $gl) -ne $expected) { throw "empreinte de gitleaks inattendue ($(Get-Sha256 $gl), attendu $expected)." }

  if ($Verify) {
    $want = (Get-Content -LiteralPath (Join-Path $hooksRoot 'current.sha256') -Raw).Trim()
    $path = "$(& git -C $Repo config --local --get core.hooksPath)".Trim()
    $dir = Join-Path $hooksRoot $want
    if (-not (Test-Path -LiteralPath $dir)) { throw "copie épinglée absente : $dir" }
    if ([IO.Path]::GetFullPath($path) -ne [IO.Path]::GetFullPath($dir)) { throw "core.hooksPath = '$path', attendu $dir" }
    if ((Get-SetHash $dir) -ne $want) { throw "contenu de $dir modifié (empreinte $(Get-SetHash $dir))" }
    "Crochets conformes : $dir"
    exit 0
  }

  $stage = Join-Path $hooksRoot "staging-$([guid]::NewGuid().ToString('N'))"
  New-Item -ItemType Directory -Force $stage | Out-Null
  Copy-Item -Path (Join-Path $PSScriptRoot 'hooks\*') -Destination $stage
  Copy-Item -LiteralPath $lock -Destination $stage
  $hash = Get-SetHash $stage
  $dest = Join-Path $hooksRoot $hash
  if (Test-Path -LiteralPath $dest) {
    Remove-Item -LiteralPath $stage -Recurse -Force
    if ((Get-SetHash $dest) -ne $hash) { throw "$dest existe avec un contenu différent : à examiner, rien n'est remplacé." }
  } else {
    Move-Item -LiteralPath $stage -Destination $dest
    Get-ChildItem -LiteralPath $dest -File | ForEach-Object { $_.IsReadOnly = $true }
  }
  & git -C $Repo config --local core.hooksPath ($dest -replace '\\', '/')
  if ($LASTEXITCODE -ne 0) { throw 'git config core.hooksPath a échoué.' }
  Set-Content -LiteralPath (Join-Path $hooksRoot 'current.sha256') -Value $hash -NoNewline
  "Crochets installés : $dest (core.hooksPath de $Repo)"
} catch {
  [Console]::Error.WriteLine("install-hooks : $($_.Exception.Message)")
  exit 1
}
