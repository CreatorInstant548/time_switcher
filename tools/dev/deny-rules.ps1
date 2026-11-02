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
  # #119: merge through the API (pulls/<n>/merge, and the merges endpoint), and the access to the
  # repository (collaborators, deploy keys, webhooks, environments).
  '/merge', 'collaborators', 'keys', 'hooks', 'environments'
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
    'git -C * push * +*'
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
    'gh api graphql*BranchProtectionRule*', 'gh api graphql*updateRepository*'
  )) { $Base.Add($p) }
# Settings and secrets of the repository.
foreach ($p in @('gh repo edit:*', 'gh repo delete:*', 'gh secret:*', 'gh workflow disable:*', 'gh ruleset:*')) {
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
  $missing = @($rules | Where-Object { $actual -cnotcontains $_ })
  $extra = @($actual | Where-Object { $rules -cnotcontains $_ })
  $sameOrder = ($actual.Count -eq $rules.Count) -and -not (Compare-Object $actual $rules -SyncWindow 0 -CaseSensitive)
  if ($missing.Count -eq 0 -and $extra.Count -eq 0 -and $sameOrder) {
    Write-Output "deny rules: $($rules.Count) rules ($($Base.Count) base patterns x 5 forms), as generated"
    exit 0
  }
  $missing | ForEach-Object { Write-Output "missing: $_" }
  $extra | ForEach-Object { Write-Output "not generated: $_" }
  if ($missing.Count -eq 0 -and $extra.Count -eq 0) { Write-Output 'same rules, other order' }
  Write-Output 'deny rules differ from tools/dev/deny-rules.ps1: run it with -Write, then review the diff'
  exit 1
}

# -Write: only the deny array is replaced, the rest of the file is kept byte for byte.
$pattern = '(?s)("deny"\s*:\s*\[)(.*?)(\n(\s*)\])'
$m = [regex]::Match($text, $pattern)
if (-not $m.Success) { Write-Output "ERROR: no deny array in $Settings"; exit 2 }
$indent = $m.Groups[4].Value + '  '
$items = ($rules | ForEach-Object { $indent + ($_ | ConvertTo-Json) }) -join ",`n"
$new = $text.Substring(0, $m.Index) + $m.Groups[1].Value + "`n" + $items + $m.Groups[3].Value + $text.Substring($m.Index + $m.Length)
$null = $new | ConvertFrom-Json
[System.IO.File]::WriteAllText($Settings, $new, [System.Text.UTF8Encoding]::new($false))
Write-Output "deny rules written: $($rules.Count) rules in $Settings"
