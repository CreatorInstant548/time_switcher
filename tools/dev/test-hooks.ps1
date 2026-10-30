# tools/dev/test-hooks.ps1 : essais des crochets Git et de install-hooks.ps1, dans un dépôt jetable
# sous TS_CACHE\tmp\ (jamais dans le vrai dépôt). L'installation se fait dans un TS_CACHE jetable.
# Sortie 1 si un essai échoue. Le faux secret est fabriqué à l'exécution : aucun n'est commis.
$ErrorActionPreference = 'Stop'
$dev = $PSScriptRoot
$realCache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
$tmp = Join-Path $realCache "tmp\hooks-$([guid]::NewGuid().ToString('N').Substring(0, 8))"
$repo = Join-Path $tmp 'repo'; $remote = Join-Path $tmp 'remote.git'; $other = Join-Path $tmp 'autre'
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
function Verify { & pwsh -NoProfile -File (Join-Path $dev 'install-hooks.ps1') -Repo $repo -Verify 2>&1 | Out-String }
function MainSha { "$(G rev-parse main)".Trim() }

try {
  # Dépôt jetable dont le distant a déjà un main (amorcé avant l'installation des crochets).
  & git init -q --bare -b main $remote
  & git init -q -b main $repo
  G config user.name essai | Out-Null; G config user.email essai@example.invalid | Out-Null
  G remote add origin $remote | Out-Null
  Add 'README.md' (Text "essai`n"); G commit -q -m 'docs(essai): base' | Out-Null; G push -q origin main | Out-Null
  $inst = & pwsh -NoProfile -File (Join-Path $dev 'install-hooks.ps1') -Repo $repo 2>&1 | Out-String
  Check 'installation' ($LASTEXITCODE -eq 0) $inst
  $v = Verify; Check 'vérification de l''empreinte et de la valeur effective' ($LASTEXITCODE -eq 0) $v

  Add 'a.txt' (Text "a`n")
  Expect 'commit sur main refusé' $true { G commit -m 'docs(essai): sur main' } 'SEC0-13'
  G reset -q | Out-Null
  G switch -q -c claude/essai | Out-Null
  Add 'a.txt' (Text "a`n")
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

  # I3 : gitleaks ne se désactive pas depuis la copie de travail.
  $chars = [char[]]'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
  $fake = ('gh' + 'p_') + -join (1..36 | ForEach-Object { $chars | Get-Random })
  Add 'config.txt' (Text "token = `"$fake`"`n")
  Expect 'faux secret refusé par gitleaks' $true { G commit -m 'chore(essai): secret' } 'gitleaks a trouv'
  Set-Content (Join-Path $repo '.gitleaks.toml') "[extend]`nuseDefault = true`n[allowlist]`npaths = ['''config\.txt''']"
  Expect 'secret refusé malgré un .gitleaks.toml non suivi' $true { G commit -m 'chore(essai): secret' } 'gitleaks a trouv'
  Remove-Item (Join-Path $repo '.gitleaks.toml')
  Set-Content (Join-Path $repo '.gitleaksignore') "config.txt:github-pat:1"
  Expect 'secret refusé malgré un .gitleaksignore non suivi' $true { G commit -m 'chore(essai): secret' } '\.gitleaksignore'
  Remove-Item (Join-Path $repo '.gitleaksignore')
  $env:GITLEAKS_CONFIG_TOML = "[allowlist]`npaths = ['''.*''']"
  $env:GITLEAKS_CONFIG = Join-Path $tmp 'vide.toml'; Set-Content $env:GITLEAKS_CONFIG "[allowlist]`npaths = ['''.*''']"
  try { Expect 'secret refusé malgré GITLEAKS_CONFIG et GITLEAKS_CONFIG_TOML' $true { G commit -m 'chore(essai): secret' } 'gitleaks a trouv' }
  finally { Remove-Item Env:GITLEAKS_CONFIG_TOML, Env:GITLEAKS_CONFIG }
  Add 'config.txt' (Text "token = `"$fake`" # gitleaks:allow`n")
  Expect 'secret refusé malgré gitleaks:allow' $true { G commit -m 'chore(essai): secret' } 'gitleaks a trouv'
  # I3 bis : un attribut -diff ou binary ne rend pas gitleaks aveugle.
  Add 'config.txt' (Text "token = `"$fake`"`n")
  Add '.gitattributes' (Text "config.txt -diff`n")
  Expect 'secret refusé malgré .gitattributes -diff (pre-commit)' $true { G commit -m 'chore(essai): secret -diff' } 'gitleaks a trouv'
  G rm -q --cached .gitattributes | Out-Null; Remove-Item (Join-Path $repo '.gitattributes')
  $info = Join-Path "$(G rev-parse --git-common-dir)".Trim() 'info'
  if (-not [IO.Path]::IsPathRooted($info)) { $info = Join-Path $repo $info }
  New-Item -ItemType Directory -Force $info | Out-Null; Set-Content (Join-Path $info 'attributes') 'config.txt binary'
  Expect 'secret refusé malgré .git/info/attributes binary (pre-commit)' $true { G commit -m 'chore(essai): secret binary' } 'gitleaks a trouv'
  # pre-push : commit fait sans crochet (dossier de crochets vide), puis poussée avec les crochets épinglés.
  $none = Join-Path $tmp 'sans-crochets'; New-Item -ItemType Directory -Force $none | Out-Null
  G switch -q -c claude/cache | Out-Null
  & git -C $repo -c "core.hooksPath=$none" commit -q -m 'chore(essai): secret caché' 2>&1 | Out-Null
  Expect 'secret refusé malgré binary (pre-push, --text)' $true { G push origin claude/cache } 'gitleaks a trouv'
  Remove-Item (Join-Path $info 'attributes')
  G switch -q claude/essai | Out-Null; G branch -q -D claude/cache | Out-Null
  G rm -q --cached config.txt 2>&1 | Out-Null; Remove-Item (Join-Path $repo 'config.txt') -ErrorAction SilentlyContinue

  Expect 'poussée d''une branche de tâche acceptée' $false { G push -q origin claude/essai }
  Expect 'poussée vers main refusée' $true { G push origin HEAD:main } 'vers main interdite'
  Expect 'poussée vers heads/main refusée' $true { G push origin HEAD:heads/main } 'vers main interdite'
  Add 'b.txt' (Text "b`n"); G commit -q --amend -m 'docs(essai): réécrit' | Out-Null
  Expect 'poussée forcée refusée' $true { G push -f origin claude/essai } 'forc'

  # I2 : main ne bouge que vers origin/main (reference-transaction).
  G switch -q -c claude/autre | Out-Null; Add 'd.txt' (Text "d`n"); G commit -q -m 'docs(essai): d' | Out-Null
  G switch -q main | Out-Null
  $base = MainSha
  Expect 'fusion sur main refusée' $true { G merge --no-ff -m 'Merge claude/autre' claude/autre } 'SEC0-13'
  G merge --abort 2>&1 | Out-Null
  Expect 'avance rapide de main hors origin/main refusée' $true { G merge --ff-only claude/autre } 'main du distant'
  Expect 'git reset --hard vers une autre branche refusé' $true { G reset --hard claude/autre } 'main du distant'
  G reset -q --hard | Out-Null   # le reset refusé a déjà touché l'index et les fichiers : on les rétablit
  Expect 'git cherry-pick sur main refusé' $true { G cherry-pick claude/autre } 'main du distant'
  G cherry-pick --abort 2>&1 | Out-Null
  Expect 'git revert sur main refusé' $true { G revert --no-edit HEAD } 'main du distant'
  G revert --abort 2>&1 | Out-Null
  Expect 'git update-ref refs/heads/main refusé' $true { G update-ref refs/heads/main claude/autre } 'main du distant'
  G switch -q claude/essai | Out-Null
  Expect 'git branch -f main refusé' $true { G branch -f main claude/autre } 'main du distant'
  Expect 'suppression de main refusée' $true { G branch -D main } 'suppression de main'
  Check 'main inchangé après les refus' ((MainSha) -eq $base) "$(MainSha) / $base"

  # git pull --ff-only dans la copie qui suit main : permis.
  & git clone -q $remote $other
  & git -C $other -c user.name=essai -c user.email=essai@example.invalid commit -q --allow-empty -m 'docs(essai): distant'
  & git -C $other push -q origin main
  G switch -q main | Out-Null
  Expect 'git pull --ff-only accepté' $false { G pull -q --ff-only origin main }
  Check 'main avancé à origin/main' ((MainSha) -eq "$(G rev-parse origin/main)".Trim()) (G log --oneline -3)
  # I2 bis : origin/main déplacé en local ; seule la valeur du distant (git ls-remote) fait foi.
  G switch -q -c claude/nonrelu | Out-Null; Add 'n.txt' (Text "n`n"); G commit -q -m 'docs(essai): non relu' | Out-Null
  G switch -q main | Out-Null; G update-ref refs/remotes/origin/main claude/nonrelu | Out-Null
  Expect 'avance rapide vers un origin/main local déplacé refusée' $true { G merge --ff-only origin/main } 'main du distant'
  G fetch -q origin | Out-Null
  # Distant injoignable : refus.
  & git -C $other -c user.name=essai -c user.email=essai@example.invalid commit -q --allow-empty -m 'docs(essai): distant 2'
  & git -C $other push -q origin main
  G fetch -q origin | Out-Null
  G remote set-url origin (Join-Path $tmp 'absent.git') | Out-Null
  Expect 'distant injoignable : mise à jour de main refusée' $true { G merge --ff-only origin/main } 'injoignable'
  G remote set-url origin $remote | Out-Null
  Expect 'distant rétabli : git merge --ff-only origin/main accepté' $false { G merge --ff-only origin/main }
  # origin/main absente : toute mise à jour de main refusée.
  $head = MainSha
  G switch -q claude/essai | Out-Null; G update-ref -d refs/remotes/origin/main | Out-Null
  Expect 'mise à jour de main sans origin/main refusée' $true { G branch -f main $head } 'absente'
  G fetch -q origin | Out-Null

  # Essai réel de travail courant : chaque commande passe guard-git, puis s'exécute avec les crochets épinglés.
  . (Join-Path $dev 'guard-git.ps1')
  $gitBash = Join-Path (Split-Path (Split-Path (Get-Command git).Source)) 'bin\bash.exe'
  function Invoke-Real([string]$Tool, [string]$Cmd) {
    $label = $Cmd -replace "`n", ' / '
    $v = Get-CommandVerdict $Cmd $Tool $repo
    if ($v) { Check "essai réel, garde : $label" $false $v; return }
    Push-Location $repo
    try {
      $out = if ($Tool -eq 'Bash') { & $gitBash -c $Cmd 2>&1 | Out-String } else { & pwsh -NoProfile -Command $Cmd 2>&1 | Out-String }
      $code = $LASTEXITCODE
    } finally { Pop-Location }
    Check "essai réel : $label" ($code -eq 0) $out
  }
  G switch -q claude/essai | Out-Null
  Set-Content (Join-Path $repo 'msg.txt') "docs(essai): travail courant`n" -NoNewline
  Set-Content (Join-Path $repo 'travail.txt') 'travail'
  Invoke-Real 'PowerShell' 'git switch -c claude/travail'
  Invoke-Real 'PowerShell' 'git status'
  Invoke-Real 'PowerShell' 'git add travail.txt'
  Invoke-Real 'PowerShell' 'git commit -q -F msg.txt'
  Set-Content (Join-Path $repo 'travail.txt') 'travail 2'
  if (Test-Path $gitBash) { Invoke-Real 'Bash' "git commit -q -F - -- travail.txt <<'EOF'`ndocs(essai): corps cité`n`nCite .claude/settings.json et git push origin main, sans effet.`nEOF" }
  Invoke-Real 'PowerShell' 'git push -q -u origin claude/travail'
  Invoke-Real 'PowerShell' 'git fetch -q origin'
  Invoke-Real 'PowerShell' 'gh pr view 5 -R CreatorInstant548/time_switcher --json state'
  & git -C $other -c user.name=essai -c user.email=essai@example.invalid commit -q --allow-empty -m 'docs(essai): distant 3'
  & git -C $other push -q origin main
  G switch -q main | Out-Null; G branch -q -u origin/main main | Out-Null
  Invoke-Real 'PowerShell' 'git pull -q --ff-only'
  Check 'essai réel : main à jour du distant' ((MainSha) -eq "$(& git -C $remote rev-parse main)".Trim())
  G switch -q claude/essai | Out-Null

  # Empreinte de gitleaks fausse : copie du jeu installé, tools.lock modifié.
  $installed = (G config --get core.hooksPath).Trim()
  $bad = Join-Path $tmp 'bad-hooks'; Copy-Item -Recurse $installed $bad
  $lock = Join-Path $bad 'tools.lock'; Set-ItemProperty $lock IsReadOnly $false
  (Get-Content $lock -Raw) -replace 'gitleaks\.sha256 = "[0-9a-f]+"', ('gitleaks.sha256 = "' + ('0' * 64) + '"') | Set-Content $lock -NoNewline
  Add 'c.txt' (Text "c`n")
  Expect 'empreinte de gitleaks fausse refusée' $true { & git -C $repo -c "core.hooksPath=$bad" commit -m 'docs(essai): c' 2>&1 | Out-String } 'empreinte de gitleaks inattendue'
  G reset -q | Out-Null

  # m5 : surcharge de core.hooksPath dans un worktree (config.worktree) : -Verify échoue.
  $wt = Join-Path $tmp 'wt'; G worktree add -q -b claude/wt $wt | Out-Null
  & git -C $repo config extensions.worktreeConfig true
  & git -C $wt config --worktree core.hooksPath $bad
  $v = Verify; Check 'surcharge dans config.worktree détectée par -Verify' (($LASTEXITCODE -ne 0) -and $v -match 'wt') $v
  & git -C $wt config --worktree --unset core.hooksPath
  $v = Verify; Check '-Verify conforme après retrait de la surcharge' ($LASTEXITCODE -eq 0) $v

  # Jeu installé modifié : -Verify échoue.
  $f = Join-Path $installed 'commit-msg'; Set-ItemProperty $f IsReadOnly $false; Add-Content $f '# modifié'
  $v = Verify; Check 'jeu modifié détecté par -Verify' ($LASTEXITCODE -ne 0) $v
} finally {
  Remove-Item Env:GITLEAKS_CONFIG_TOML, Env:GITLEAKS_CONFIG -ErrorAction SilentlyContinue
  $env:TS_CACHE = $oldCache
  Get-ChildItem -LiteralPath $tmp -Recurse -File -Force | ForEach-Object { $_.IsReadOnly = $false }
  Remove-Item -LiteralPath $tmp -Recurse -Force
}
"test-hooks : $($n - $fail)/$n essais conformes."
exit [int]($fail -gt 0)
