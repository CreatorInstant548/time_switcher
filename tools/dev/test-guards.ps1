# tools/dev/test-guards.ps1 : tests de guard-git.ps1 et guard-main.ps1 (tables de cas refusés et permis).
# Dépôts jetables sous TS_CACHE\tmp\ ; aucun effet sur le vrai dépôt. Sortie 1 si un cas échoue.
$ErrorActionPreference = 'Stop'
$dev = $PSScriptRoot
. (Join-Path $dev 'guard-git.ps1')
. (Join-Path $dev 'guard-main.ps1')

$cache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
$tmp = Join-Path $cache "tmp\guards-$([guid]::NewGuid().ToString('N').Substring(0, 8))"
$repos = @{}
foreach ($r in @(@('main', 'main'), @('feat', 'claude/x'), @('tool', 'claude/outillage-essai'), @('pil', 'pilotage'))) {
  $d = Join-Path $tmp $r[0]
  New-Item -ItemType Directory -Force (Join-Path $d '.claude') | Out-Null
  & git init -q -b $r[1] $d
  $repos[$r[0]] = $d
}
$M = $repos.main; $F = $repos.feat; $T = $repos.tool

# Chaque cas : outil, copie courante, commande, verdict attendu (D = refus, A = permis).
$cases = @(
  # SEC0-13, SEC0-14 : commit sur main
  @('Bash', 'main', 'git commit -m "x"', 'D'),
  @('Bash', 'feat', "git -C '$M' commit -m x", 'D'),
  @('Bash', 'feat', "cd '$M' && git commit -am x", 'D'),
  @('Bash', 'main', 'git   commit    -m x', 'D'),
  @('PowerShell', 'feat', "git -C `"$M`" commit -m x", 'D'),
  # poussée vers main, forcée, ou depuis main
  @('Bash', 'feat', 'git push origin main', 'D'),
  @('Bash', 'feat', 'git push origin HEAD:main', 'D'),
  @('Bash', 'feat', 'git push origin HEAD:refs/heads/main', 'D'),
  @('Bash', 'feat', 'git push origin claude/x:main', 'D'),
  @('Bash', 'feat', 'git push origin :main', 'D'),
  @('Bash', 'feat', 'git push origin +claude/x', 'D'),
  @('Bash', 'feat', 'git push --force origin claude/x', 'D'),
  @('Bash', 'feat', 'git push -f origin claude/x', 'D'),
  @('Bash', 'feat', 'git push -uf origin claude/x', 'D'),
  @('Bash', 'feat', 'git push --force-with-lease origin claude/x', 'D'),
  @('Bash', 'feat', 'git push --force-with-lease=claude/x:abc origin claude/x', 'D'),
  @('Bash', 'feat', 'git push --forc origin claude/x', 'D'),
  @('Bash', 'feat', 'git push --force-if-includes origin claude/x', 'D'),
  @('Bash', 'feat', 'git push --mirror origin', 'D'),
  @('Bash', 'feat', 'git push --all origin', 'D'),
  @('Bash', 'feat', 'git push origin --delete pilotage', 'D'),
  @('Bash', 'main', 'git push', 'D'),
  @('Bash', 'main', 'git push origin', 'D'),
  @('Bash', 'main', 'git push origin HEAD', 'D'),
  # variantes : chaînage, guillemets, chemins, sous-shells
  @('Bash', 'feat', 'git status && git push origin main', 'D'),
  @('Bash', 'feat', 'git status; git push origin main', 'D'),
  @('Bash', 'feat', 'echo ok | git push --force', 'D'),
  @('Bash', 'feat', 'git push origin "main"', 'D'),
  @('Bash', 'feat', "git push origin 'HEAD:main'", 'D'),
  @('Bash', 'feat', 'git push origin ma\in', 'D'),
  @('Bash', 'feat', 'bash -c "git push origin main"', 'D'),
  @('Bash', 'feat', 'echo "$(git push -f)"', 'D'),
  @('Bash', 'feat', '/usr/bin/git push origin main', 'D'),
  @('Bash', 'feat', 'git.exe push origin main', 'D'),
  @('Bash', 'feat', 'GIT_TRACE=1 git push origin main', 'D'),
  @('Bash', 'feat', 'git -C . push origin main', 'D'),
  @('PowerShell', 'feat', 'git push origin main; Write-Host ok', 'D'),
  @('PowerShell', 'feat', '& git push --force origin claude/x', 'D'),
  @('PowerShell', 'feat', '& "C:\Program Files\Git\cmd\git.exe" push origin main', 'D'),
  @('PowerShell', 'feat', 'git push origin ma`in', 'D'),
  @('PowerShell', 'feat', 'pwsh -Command "git push -f origin claude/x"', 'D'),
  # --no-verify et core.hooksPath
  @('Bash', 'feat', 'git commit --no-verify -m x', 'D'),
  @('Bash', 'feat', 'git commit --no-verif -m x', 'D'),
  @('Bash', 'feat', 'git commit -n -m x', 'D'),
  @('Bash', 'feat', 'git commit -anm x', 'D'),
  @('Bash', 'feat', 'git push --no-verify origin claude/x', 'D'),
  @('Bash', 'feat', 'git merge --no-verify claude/y', 'D'),
  @('Bash', 'feat', 'git -c core.hooksPath=/dev/null commit -m x', 'D'),
  @('Bash', 'feat', 'git -c core.hookspath=x push origin claude/x', 'D'),
  @('Bash', 'feat', 'git config core.hooksPath /tmp/h', 'D'),
  @('Bash', 'feat', 'git config --local core.hooksPath x', 'D'),
  @('Bash', 'feat', 'git config core.hooksPath', 'D'),
  @('Bash', 'feat', 'git config --unset core.hooksPath', 'D'),
  @('Bash', 'feat', 'GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=core.hooksPath GIT_CONFIG_VALUE_0=x git commit -m x', 'D'),
  # update-ref, branch, checkout/switch, gh api
  @('Bash', 'feat', 'git update-ref refs/heads/main HEAD', 'D'),
  @('Bash', 'feat', 'git update-ref -d refs/heads/main', 'D'),
  @('Bash', 'feat', 'git update-ref --stdin', 'D'),
  @('Bash', 'feat', 'git branch -f main HEAD', 'D'),
  @('Bash', 'feat', 'git branch --force main', 'D'),
  @('Bash', 'feat', 'git branch -D main', 'D'),
  @('Bash', 'feat', 'git branch -m main', 'D'),
  @('Bash', 'feat', 'git checkout -B main', 'D'),
  @('Bash', 'feat', 'git switch -C main origin/main', 'D'),
  @('Bash', 'feat', 'gh api repos/o/r/git/refs/heads/main -X PATCH -f sha=abc', 'D'),
  @('Bash', 'feat', 'gh api -X DELETE repos/o/r/git/refs/heads/x', 'D'),
  # SEC0-35 : réglages de Claude Code
  @('Bash', 'feat', 'echo {} > .claude/settings.json', 'D'),
  @('Bash', 'feat', 'echo x >> .claude/settings.local.json', 'D'),
  @('Bash', 'feat', 'cp /tmp/x .claude/settings.json', 'D'),
  @('Bash', 'feat', 'mv x.json .claude/settings.local.json', 'D'),
  @('Bash', 'feat', "sed -i 's/a/b/' .claude/settings.json", 'D'),
  @('Bash', 'feat', 'tee .claude/settings.json < x', 'D'),
  @('Bash', 'feat', 'rm .claude/settings.json', 'D'),
  @('Bash', 'feat', 'cd .claude && echo {} > settings.json', 'D'),
  @('Bash', 'feat', 'git checkout HEAD~1 -- .claude/settings.json', 'D'),
  @('Bash', 'tool', "cp x '$M/.claude/settings.json'", 'D'),
  @('Bash', 'feat', "echo '{`"disableAllHooks`": true}' > x.json", 'D'),
  @('Bash', 'feat', 'grep -r disableAllHooks .', 'D'),
  @('PowerShell', 'feat', "Set-Content .claude/settings.json '{}'", 'D'),
  @('PowerShell', 'feat', "'{}' | Out-File -FilePath .claude\settings.local.json", 'D'),
  @('PowerShell', 'feat', 'Copy-Item x.json .claude\settings.json', 'D'),
  @('PowerShell', 'feat', "[IO.File]::WriteAllText('.claude/settings.json', '{}')", 'D'),
  @('PowerShell', 'feat', 'Remove-Item .claude\settings.local.json', 'D'),

  # Permis
  @('Bash', 'feat', 'git status', 'A'),
  @('Bash', 'feat', 'git log --oneline -5', 'A'),
  @('Bash', 'feat', 'git commit -m "feat(x): y"', 'A'),
  @('Bash', 'feat', 'git commit -m "docs(x): la branche main et push --force"', 'A'),
  @('Bash', 'main', "git -C '$F' commit -m x", 'A'),
  @('Bash', 'feat', 'git push -u origin claude/x', 'A'),
  @('Bash', 'feat', 'git push origin claude/x:claude/x', 'A'),
  @('Bash', 'feat', 'git push origin HEAD', 'A'),
  @('Bash', 'feat', 'git push', 'A'),
  @('Bash', 'feat', 'git push --follow-tags origin claude/x', 'A'),
  @('Bash', 'feat', 'git push origin --delete claude/vieux', 'A'),
  @('Bash', 'pil', 'git commit -m "docs(pilotage): carnet"', 'A'),
  @('Bash', 'pil', 'git push origin pilotage', 'A'),
  @('Bash', 'feat', 'git fetch origin && git rebase origin/main', 'A'),
  @('Bash', 'main', 'git pull --ff-only', 'A'),
  @('Bash', 'main', 'git switch main', 'A'),
  @('Bash', 'feat', 'git config --get core.hooksPath', 'A'),
  @('Bash', 'feat', 'git branch -d claude/vieux', 'A'),
  @('Bash', 'feat', 'git branch --merged main', 'A'),
  @('Bash', 'feat', 'git diff main...HEAD', 'A'),
  @('Bash', 'feat', 'git log origin/main..HEAD 2>&1 | head', 'A'),
  @('Bash', 'feat', 'gh pr create --base main --title t --body-file f.md', 'A'),
  @('Bash', 'feat', 'gh api repos/o/r/pulls', 'A'),
  @('Bash', 'feat', 'cat .claude/settings.json', 'A'),
  @('Bash', 'feat', 'git diff -- .claude/settings.json 2>&1', 'A'),
  @('Bash', 'tool', 'echo {} > .claude/settings.json', 'A'),
  @('Bash', 'tool', 'grep -r disableAllHooks docs', 'A'),
  @('Bash', 'feat', 'cargo build -p ts_kernel', 'A'),
  @('Bash', 'feat', 'echo main > notes.txt', 'A'),
  @('PowerShell', 'feat', 'git commit -m "x" 2>&1', 'A'),
  @('PowerShell', 'feat', 'Get-Content .claude\settings.json', 'A'),
  @('PowerShell', 'tool', "Set-Content .claude/settings.json '{}'", 'A')
)

$fail = 0; $n = 0
foreach ($c in $cases) {
  $n++
  $v = Get-CommandVerdict $c[2] $c[0] $repos[$c[1]]
  $got = if ($v) { 'D' } else { 'A' }
  if ($got -ne $c[3]) { $fail++; "ÉCHEC guard-git [$($c[0]) @ $($c[1])] $($c[2]) : attendu $($c[3]), obtenu $got $v" }
}

# guard-main : chemin, copie principale = dépôt jetable « main ».
$edits = @(
  @("$M\src\lib.rs", 'D'),
  @("$M\.git\config", 'D'),
  @("$M\.claude\worktrees\wt\src\lib.rs", 'A'),
  @("$M-pilotage\carnet.md", 'A'),
  @("$HOME\notes\x.md", 'A'),
  @("$HOME\.claude\settings.json", 'D'),
  @("$M\.claude\settings.json", 'D'),
  @("$F\.claude\settings.json", 'D'),
  @("$F\.claude\settings.local.json", 'D'),
  @("$F\crates\x\settings.json", 'A'),
  @("$F\src\lib.rs", 'A'),
  @("$T\.claude\settings.json", 'A'),
  @("$T\.claude\settings.local.json", 'A')
)
foreach ($e in $edits) {
  $n++
  $v = Get-EditVerdict (ConvertTo-FullPath $e[0]) $M
  $got = if ($v) { 'D' } else { 'A' }
  if ($got -ne $e[1]) { $fail++; "ÉCHEC guard-main $($e[0]) : attendu $($e[1]), obtenu $got $v" }
}

# Bout en bout : JSON sur l'entrée standard, code de sortie 2 ou 0.
function Invoke-Hook([string]$Script, [hashtable]$Payload) {
  $Payload | ConvertTo-Json -Compress | & pwsh -NoProfile -File (Join-Path $dev $Script) 2>$null | Out-Null
  return $LASTEXITCODE
}
$e2e = @(
  @('guard-git.ps1', @{ tool_name = 'Bash'; cwd = $F; tool_input = @{ command = 'git push --force origin claude/x' } }, 2),
  @('guard-git.ps1', @{ tool_name = 'PowerShell'; cwd = $F; tool_input = @{ command = 'git status' } }, 0),
  @('guard-main.ps1', @{ tool_name = 'Write'; cwd = $F; tool_input = @{ file_path = "$F\.claude\settings.json" } }, 2),
  @('guard-main.ps1', @{ tool_name = 'Edit'; cwd = $F; tool_input = @{ file_path = "$F\src\lib.rs" } }, 0)
)
foreach ($e in $e2e) {
  $n++
  $code = Invoke-Hook $e[0] $e[1]
  if ($code -ne $e[2]) { $fail++; "ÉCHEC bout en bout $($e[0]) $($e[1].tool_input | ConvertTo-Json -Compress) : code $code, attendu $($e[2])" }
}

Remove-Item -LiteralPath $tmp -Recurse -Force
"test-guards : $($n - $fail)/$n cas conformes."
exit [int]($fail -gt 0)
