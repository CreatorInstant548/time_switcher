# tools/dev/test-hooks.ps1 : essais des crochets Git et de install-hooks.ps1, dans un dépôt jetable
# sous TS_CACHE\tmp\ (jamais dans le vrai dépôt). L'installation se fait dans un TS_CACHE jetable.
# Sortie 1 si un essai échoue. Le faux secret est fabriqué à l'exécution : aucun n'est commis.
$ErrorActionPreference = 'Stop'
$dev = $PSScriptRoot
$realCache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
$tmp = Join-Path $realCache "tmp\hooks-$([guid]::NewGuid().ToString('N').Substring(0, 8))"
$repo = Join-Path $tmp 'repo'; $remote = Join-Path $tmp 'remote.git'
New-Item -ItemType Directory -Force $repo | Out-Null
$oldCache = $env:TS_CACHE; $env:TS_CACHE = Join-Path $tmp 'cache'
$fail = 0; $n = 0

function G { & git -C $repo @args 2>&1 | Out-String }
function Check([string]$Name, [bool]$Ok, [string]$Detail = '') {
  $script:n++
  if ($Ok) { "  ok  $Name" } else { $script:fail++; "ÉCHEC $Name $Detail" }
}
function Expect([string]$Name, [bool]$Refused, [scriptblock]$Do, [string]$Why = '') {
  $out = & $Do | Out-String; $code = $LASTEXITCODE   # $Why : motif attendu dans le message de refus
  Check $Name ((($code -ne 0) -eq $Refused) -and (-not $Refused -or $out -match $Why)) "(code $code) $out"
}
function Add([string]$Path, [byte[]]$Bytes) {
  $p = Join-Path $repo $Path; New-Item -ItemType Directory -Force (Split-Path $p) | Out-Null
  [IO.File]::WriteAllBytes($p, $Bytes); G add -- $Path | Out-Null
}
function Text([string]$s) { [Text.Encoding]::UTF8.GetBytes($s) }

try {
  & git init -q --bare -b main $remote
  & git init -q -b main $repo
  G config user.name essai | Out-Null; G config user.email essai@example.invalid | Out-Null
  G remote add origin $remote | Out-Null
  $inst = & pwsh -NoProfile -File (Join-Path $dev 'install-hooks.ps1') -Repo $repo 2>&1 | Out-String
  Check 'installation' ($LASTEXITCODE -eq 0) $inst
  $verify = & pwsh -NoProfile -File (Join-Path $dev 'install-hooks.ps1') -Repo $repo -Verify 2>&1 | Out-String
  Check 'vérification de l''empreinte' ($LASTEXITCODE -eq 0) $verify

  Add 'README.md' (Text "essai`n")
  Expect 'commit sur main refusé' $true { G commit -m 'docs(essai): premier' } 'SEC0-13'
  G switch -q -c claude/essai | Out-Null
  Expect 'message mal formé refusé' $true { G commit -m 'premier commit' } 'format attendu'
  Expect 'message conforme accepté' $false { G commit -m 'docs(essai): premier' }

  Add 'big.bin' ([byte[]]::new(5MB + 1))
  Expect 'fichier de plus de 5 Mo refusé' $true { G commit -m 'chore(essai): gros' } '> 5 Mo'
  G rm -q --cached big.bin | Out-Null
  Add 'art/logo.PNG' ([byte[]]::new(100))
  Expect 'extension .png hors LFS refusée' $true { G commit -m 'chore(essai): png' } 'hors LFS'
  G rm -q --cached art/logo.PNG | Out-Null
  Add 'crates/a/tests/fixtures/petit.png' ([byte[]]::new(1000))
  Expect 'fixture de 1 Ko acceptée' $false { G commit -m 'test(essai): fixture' }
  Add 'crates/a/tests/fixtures/gros.png' ([byte[]]::new(300KB))
  Expect 'fixture de 300 Ko refusée' $true { G commit -m 'test(essai): grosse fixture' } '256 Ko'
  G rm -q --cached crates/a/tests/fixtures/gros.png | Out-Null

  $chars = [char[]]'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
  $fake = ('gh' + 'p_') + -join (1..36 | ForEach-Object { $chars | Get-Random })
  Add 'config.txt' (Text "token = `"$fake`"`n")
  Expect 'faux secret refusé par gitleaks' $true { G commit -m 'chore(essai): secret' } 'gitleaks a trouv'
  G rm -q --cached config.txt | Out-Null

  Expect 'poussée d''une branche de tâche acceptée' $false { G push -q origin claude/essai }
  Expect 'poussée vers main refusée' $true { G push origin HEAD:main } 'vers main interdite'
  Add 'b.txt' (Text "b`n"); G commit -q --amend -m 'docs(essai): réécrit' | Out-Null
  Expect 'poussée forcée refusée' $true { G push -f origin claude/essai } 'forc'

  G switch -q -c claude/autre | Out-Null; Add 'd.txt' (Text "d`n"); G commit -q -m 'docs(essai): d' | Out-Null
  G branch main claude/essai | Out-Null; G switch -q main | Out-Null
  Expect 'fusion sur main refusée (pre-merge-commit)' $true { G merge --no-ff -m 'Merge claude/autre' claude/autre } 'SEC0-13'
  G merge --abort 2>&1 | Out-Null; G switch -q claude/essai | Out-Null

  # Empreinte de gitleaks fausse : copie du jeu installé, tools.lock modifié.
  $installed = (G config --get core.hooksPath).Trim()
  $bad = Join-Path $tmp 'bad-hooks'; Copy-Item -Recurse $installed $bad
  $lock = Join-Path $bad 'tools.lock'; Set-ItemProperty $lock IsReadOnly $false
  (Get-Content $lock -Raw) -replace 'gitleaks\.sha256 = "[0-9a-f]+"', ('gitleaks.sha256 = "' + ('0' * 64) + '"') | Set-Content $lock -NoNewline
  Add 'c.txt' (Text "c`n")
  Expect 'empreinte de gitleaks fausse refusée' $true { & git -C $repo -c "core.hooksPath=$bad" commit -m 'docs(essai): c' 2>&1 | Out-String } 'empreinte de gitleaks inattendue'
  G reset -q | Out-Null

  # Jeu installé modifié : -Verify échoue.
  $f = Join-Path $installed 'commit-msg'; Set-ItemProperty $f IsReadOnly $false; Add-Content $f '# modifié'
  $verify = & pwsh -NoProfile -File (Join-Path $dev 'install-hooks.ps1') -Repo $repo -Verify 2>&1 | Out-String
  Check 'jeu modifié détecté par -Verify' ($LASTEXITCODE -ne 0) $verify
} finally {
  $env:TS_CACHE = $oldCache
  Get-ChildItem -LiteralPath $tmp -Recurse -File -Force | ForEach-Object { $_.IsReadOnly = $false }
  Remove-Item -LiteralPath $tmp -Recurse -Force
}
"test-hooks : $($n - $fail)/$n essais conformes."
exit [int]($fail -gt 0)
