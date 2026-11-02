# tools/dev/deny-rules.ps1: the native deny rules of Claude Code (`permissions.deny` of
# .claude/settings.json), generated from one list of base patterns (AS-50 AU1, issue #119).
#   pwsh -NoProfile -File tools/dev/deny-rules.ps1 -Check   compare the file with the list (code 1 if it differs)
#   pwsh -NoProfile -File tools/dev/deny-rules.ps1 -Write   rewrite the deny array of the file from the list
#   pwsh -NoProfile -File tools/dev/deny-rules.ps1 -List    print the generated rules, one per line
# Each base pattern is written under five forms: Bash(p), PowerShell(p), PowerShell(& p),
# Bash(cmd /c p) and PowerShell(cmd /c p). Usage and limits: docs/dev/outillage.md §3.1, §3.2.
# These rules are a belt, not a lock: the server rulesets of main and pilotage are the lock (§6.4).
param([switch]$Check, [switch]$Write, [switch]$List, [string]$Settings)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

# Resources whose writes through `gh api` are refused, with -X or --method before or after them,
# whatever the method (four patterns each).
$Resources = @(
  'rulesets', '/protection', 'actions/permissions', 'interaction-limits',
  'private-vulnerability-reporting',
  # #119: merge through the API (pulls/<n>/merge, and the merges endpoint), the access to the
  # repository (collaborators, deploy keys, webhooks, environments), what gh secret, gh variable
  # and gh workflow disable do, and the transfer of the repository. A leading `/` keeps the word
  # out of prose (`-f body='the hooks fail'`).
  '/merge', '/collaborators', '/keys', '/hooks', '/environments', 'actions/workflows',
  'actions/secrets', 'actions/variables', '/transfer'
)

$Base = [System.Collections.Generic.List[string]]::new()
# Merge: through cargo xtask merge only (§3.3).
$Base.Add('gh pr merge:*')
# Forced push, or push to main (all refused by the main ruleset too).
foreach ($p in @(
    'git push --force:*', 'git push -f:*', 'git push --force-with-lease:*',
    'git push * --force*', 'git push * -f *', 'git push * -f', 'git push * main',
    'git push * main:*', 'git push * HEAD:main*', 'git push * +*',
    # #119: options glued to their value, grouped short options, other spellings of main.
    'git push --force*', 'git push -fu*', 'git push -uf*', 'git push * -fu*', 'git push * -uf*',
    'git push * main *', 'git push *:main*', 'git push * refs/heads/main*',
    'git push *:refs/heads/main*',
    # #119: git -C <dir> push, forced or to main.
    'git -C * push *--force*', 'git -C * push -f*', 'git -C * push * -f', 'git -C * push * -f *',
    'git -C * push -uf*', 'git -C * push * -fu*', 'git -C * push * -uf*', 'git -C * push * main',
    'git -C * push * main *', 'git -C * push *:main*', 'git -C * push * refs/heads/main*',
    'git -C * push * +*', 'git -C * push *:refs/heads/main*',
    # #119: --mirror forces and deletes.
    'git push --mirror*', 'git push * --mirror*', 'git -C * push *--mirror*'
  )) { $Base.Add($p) }
# Writes through the API of gh.
$Base.Add('gh api *-X DELETE*')
$Base.Add('gh api *--method DELETE*')
foreach ($r in $Resources) {
  $Base.Add("gh api *-X *$r*")
  $Base.Add("gh api *$r* -X *")
  $Base.Add("gh api *--method *$r*")
  $Base.Add("gh api *$r* --method *")
}
foreach ($p in @(
    # #119: PATCH of the repository (= gh repo edit), and of anything else: gh has its own
    # subcommands for the legitimate edits (issue, pr, label), and PATCH git/refs moves a branch.
    'gh api *-X PATCH*', 'gh api *--method PATCH*',
    # #119: the method glued to its option, or after `=`, whatever the method or resource.
    'gh api *-XDELETE*', 'gh api *-XPUT*', 'gh api *-XPATCH*', 'gh api *-XPOST*',
    'gh api *--method=*', 'gh api *-X=*',
    # #119: GraphQL mutations that merge, or change rulesets, branch protection or the repository.
    'gh api graphql*mergePullRequest*', 'gh api graphql*RepositoryRuleset*',
    'gh api graphql*BranchProtectionRule*', 'gh api graphql*updateRepository*',
    'gh api graphql*enablePullRequestAutoMerge*', 'gh api graphql*mergeBranch*',
    'gh api graphql*updateRef*', 'gh api graphql*deleteRef*', 'gh api graphql*createCommitOnBranch*',
    # #119: a query read from a file hides the mutation name from these patterns.
    'gh api graphql*=@*', 'gh api graphql*--input*'
  )) { $Base.Add($p) }
# Settings and secrets of the repository.
foreach ($p in @('gh repo edit:*', 'gh repo delete:*', 'gh secret:*', 'gh workflow disable:*', 'gh ruleset:*',
    'gh repo archive:*', 'gh repo rename:*', 'gh variable:*')) {
  $Base.Add($p)
}

function Get-Rules {
  $rules = [System.Collections.Generic.List[string]]::new()
  foreach ($p in $Base) { $rules.Add("Bash($p)") }
  foreach ($p in $Base) { $rules.Add("PowerShell($p)") }
  foreach ($p in $Base) {
    $rules.Add("PowerShell(& $p)")
    $rules.Add("Bash(cmd /c $p)")
    $rules.Add("PowerShell(cmd /c $p)")
  }
  return , $rules
}

$dupes = $Base | Group-Object | Where-Object Count -GT 1
if ($dupes) { Write-Output "ERROR: base pattern listed twice: $($dupes.Name -join ', ')"; exit 2 }
if (@($Check, $Write, $List).Where({ $_ }).Count -ne 1) {
  Write-Output 'usage: pwsh -NoProfile -File tools/dev/deny-rules.ps1 -Check|-Write|-List [-Settings <file>]'
  exit 2
}
$rules = Get-Rules
if ($List) { $rules; exit 0 }

if (-not $Settings) { $Settings = Join-Path (Split-Path (Split-Path $PSScriptRoot)) '.claude/settings.json' }
$text = [System.IO.File]::ReadAllText($Settings)
$json = $text | ConvertFrom-Json
$actual = @($json.permissions.deny)

if ($Check) {
  $twice = @($actual | Group-Object -CaseSensitive | Where-Object Count -GT 1 | ForEach-Object Name)
  $missing = @($rules | Where-Object { $actual -cnotcontains $_ })
  $extra = @($actual | Where-Object { $rules -cnotcontains $_ })
  $sameOrder = ($actual.Count -eq $rules.Count) -and -not (Compare-Object $actual $rules -SyncWindow 0 -CaseSensitive)
  if ($missing.Count -eq 0 -and $extra.Count -eq 0 -and $twice.Count -eq 0 -and $sameOrder) {
    Write-Output "deny rules: $($rules.Count) rules ($($Base.Count) base patterns x 5 forms), as generated"
    exit 0
  }
  $missing | ForEach-Object { Write-Output "missing: $_" }
  $extra | ForEach-Object { Write-Output "not generated: $_" }
  $twice | ForEach-Object { Write-Output "listed twice: $_" }
  if ($missing.Count -eq 0 -and $extra.Count -eq 0 -and $twice.Count -eq 0) { Write-Output 'same rules, other order' }
  Write-Output 'deny rules differ from tools/dev/deny-rules.ps1: run it with -Write, then review the diff'
  exit 1
}

# -Write: only the deny array is replaced, the rest of the file is kept byte for byte. The array is
# found by its key, then its closing bracket by a scan that skips strings, whatever its layout.
$key = [regex]::Match($text, '"deny"\s*:\s*\[')
if (-not $key.Success -or ([regex]::Matches($text, '"deny"\s*:')).Count -ne 1) {
  Write-Output "ERROR: expected exactly one deny array in $Settings"; exit 2
}
$open = $key.Index + $key.Length - 1
$close = -1; $inString = $false; $depth = 0
for ($i = $open; $i -lt $text.Length; $i++) {
  $c = $text[$i]
  if ($inString) {
    if ($c -eq [char]92) { $i++ } elseif ($c -eq '"') { $inString = $false }
  } elseif ($c -eq '"') { $inString = $true
  } elseif ($c -eq '[') { $depth++
  } elseif ($c -eq ']') { $depth--; if ($depth -eq 0) { $close = $i; break } }
}
if ($close -lt 0) { Write-Output "ERROR: unterminated deny array in $Settings"; exit 2 }
$line = $text.LastIndexOf("`n", $key.Index) + 1
$indent = ($text.Substring($line, $key.Index - $line) -replace '\S.*$', '')
$items = ($rules | ForEach-Object { "$indent  " + ($_ | ConvertTo-Json) }) -join ",`n"
$new = $text.Substring(0, $open + 1) + "`n" + $items + "`n" + $indent + $text.Substring($close)
$parsed = $new | ConvertFrom-Json
if (@($parsed.permissions.deny).Count -ne $rules.Count) { Write-Output 'ERROR: rewritten file does not hold the rules'; exit 2 }
[System.IO.File]::WriteAllText($Settings, $new, [System.Text.UTF8Encoding]::new($false))
Write-Output "deny rules written: $($rules.Count) rules in $Settings"
