# tools/dev/guard-git.ps1 : crochet PreToolUse (Bash|PowerShell) de Claude Code.
# Refuse (code 2) les commandes de SEC0-14 et l'écriture des réglages de Claude Code (SEC0-35).
# Deux lectures de la commande : l'une respecte guillemets et échappements, l'autre les ignore
# (bash -c "...", $(...) entre guillemets) ; un refus de l'une suffit. En cas de doute : refus.
# spec: carnet architecture-securite.md SEC0-13, SEC0-14, SEC0-35 ; revues/developpement-agents.md §4.4
. (Join-Path $PSScriptRoot 'guard-lib.ps1')

# Découpe en segments (séparés par ; & | ( ) { } et fin de ligne), chacun une liste de mots.
# Les redirections deviennent des mots '>', '>>', '>&' ; -Naive ignore guillemets et échappements.
function Split-Command([string]$Text, [string]$Tool, [switch]$Naive) {
  $segs = [Collections.Generic.List[object]]::new()
  $cur = [Collections.Generic.List[string]]::new()
  $sb = [Text.StringBuilder]::new(); $has = $false; $q = $null
  $esc = if ($Tool -eq 'PowerShell') { '`' } else { '\' }
  $flush = { if ($has) { $cur.Add($sb.ToString()) }; [void]$sb.Clear(); Set-Variable has $false -Scope 1 }
  for ($i = 0; $i -lt $Text.Length; $i++) {
    $c = [string]$Text[$i]
    if ($Naive) {
      if ('''"`'.Contains($c)) { $c = ' ' }
    } elseif ($q -eq "'") {
      if ($c -eq "'") { $q = $null } else { [void]$sb.Append($c) }; continue
    } elseif ($c -eq $esc -and $i + 1 -lt $Text.Length -and
              ($null -eq $q -or $Tool -eq 'PowerShell' -or '$`"\'.Contains([string]$Text[$i + 1]))) {
      $i++; [void]$sb.Append($Text[$i]); $has = $true; continue
    } elseif ($q -eq '"') {
      if ($c -eq '"') { $q = $null } else { [void]$sb.Append($c) }; continue
    } elseif ($c -eq "'" -or $c -eq '"') { $q = $c; $has = $true; continue }
    if (" `t`r".Contains($c)) { & $flush; continue }
    if ($c -eq '>') {
      & $flush; $r = '>'
      if ($i + 1 -lt $Text.Length -and $Text[$i + 1] -eq '>') { $r = '>>'; $i++ }
      if ($i + 1 -lt $Text.Length -and $Text[$i + 1] -eq '&') { $r = '>&'; $i++ }
      $cur.Add($r); continue
    }
    if (";&|`n(){}".Contains($c)) {
      & $flush; if ($cur.Count) { $segs.Add($cur.ToArray()) }; $cur.Clear(); continue
    }
    [void]$sb.Append($c); $has = $true
  }
  & $flush; if ($cur.Count) { $segs.Add($cur.ToArray()) }
  return , $segs.ToArray()
}

function Test-Abbrev([string]$Tok, [string]$Full, [int]$Min) {   # abréviation d'option longue admise par Git
  $t = ($Tok -split '=', 2)[0]
  return $t.Length -ge $Min -and $Full.StartsWith($t, [StringComparison]::OrdinalIgnoreCase)
}
function Test-MainRef([string]$Ref) { return $Ref -match '^(?i)(refs/heads/)?main$' }
function Test-Exe([string]$Tok, [string]$Name) { return $Tok -match "(?i)(^|[\\/])$Name(\.exe)?$" }
$cdCmds = 'cd', 'chdir', 'pushd', 'sl', 'set-location', 'push-location'
$valueOpts = '-C', '-c', '--git-dir', '--work-tree', '--namespace'

# Analyse d'un appel git à partir de l'index $At du segment ; motif du refus ou $null.
function Get-GitVerdict([string[]]$T, [int]$At, [string]$Dir) {
  $i = $At + 1; $gargs = @()
  while ($i -lt $T.Count -and $T[$i].StartsWith('-')) {
    $o = $T[$i]; $v = if ($i + 1 -lt $T.Count) { $T[$i + 1] } else { '' }
    if ($o -match '(?i)hookspath' -or ($o -ceq '-c' -and $v -match '(?i)hookspath')) {
      return 'modifier core.hooksPath est interdit (SEC0-14)'
    }
    if ($o -ceq '-C') { $Dir = ConvertTo-FullPath $v $Dir }
    elseif ($o -in '--git-dir', '--work-tree') { $gargs += $o, $v }
    elseif ($o -like '--git-dir=*' -or $o -like '--work-tree=*') { $gargs += $o }
    $i += $(if ($o -cin $valueOpts) { 2 } else { 1 })
  }
  if ($i -ge $T.Count) { return $null }
  $sub = $T[$i].ToLowerInvariant(); $a = @($T | Select-Object -Skip ($i + 1))
  foreach ($x in $a) { if (Test-Abbrev $x '--no-verify' 6) { return '--no-verify est interdit (SEC0-14)' } }
  switch ($sub) {
    'config' {
      if (($a -match '(?i)hookspath') -and -not ($a | Where-Object { $_ -in '--get', '--get-all', '--get-regexp', '-l', '--list', 'get', 'list' })) {
        return 'git config core.hooksPath est interdit (SEC0-14)'
      }
    }
    'commit' {
      foreach ($x in $a) {   # -n vaut --no-verify ; il peut être groupé (-an), avant une option à valeur
        if ($x -cmatch '^-[a-zA-Z]+$') { foreach ($ch in $x.Substring(1).ToCharArray()) {
          if ($ch -ceq 'n') { return 'git commit -n (--no-verify) est interdit (SEC0-14)' }
          if ('mCcFtSu'.Contains([string]$ch)) { break } } }
      }
      if ((Get-Branch $Dir $gargs) -ieq 'main') { return 'commit sur main interdit (SEC0-13) : travaillez sur une branche claude/<sujet>' }
    }
    'push' {
      $pos = @(); $skip = $false
      foreach ($x in $a) {
        if ($skip) { $skip = $false; continue }
        if ($x -in '--repo', '-o', '--push-option', '--receive-pack', '--exec') { $skip = $true; continue }
        if ((Test-Abbrev $x '--force' 4) -or (Test-Abbrev $x '--force-with-lease' 4) -or (Test-Abbrev $x '--force-if-includes' 4) -or
            ($x -cmatch '^-[a-zA-Z]*f')) { return 'poussée forcée interdite (SEC0-14)' }
        if ((Test-Abbrev $x '--mirror' 4) -or (Test-Abbrev $x '--all' 4)) { return 'git push --all/--mirror interdit : poussez une seule branche' }
        if (-not $x.StartsWith('-')) { $pos += $x }
      }
      foreach ($r in ($pos | Select-Object -Skip 1)) {
        if ($r.StartsWith('+')) { return 'destination préfixée par + (poussée forcée) interdite (SEC0-14)' }
        $dst = ($r -split ':')[-1]
        if (Test-MainRef $dst) { return 'poussée vers main interdite (SEC0-13) : passez par une PR' }
        if ($dst -match '^(?i)(refs/heads/)?pilotage$' -and ($r.StartsWith(':') -or $a -contains '--delete' -or $a -contains '-d')) {
          return 'suppression de la branche pilotage interdite'
        }
        if ($r -ieq 'HEAD' -and (Get-Branch $Dir $gargs) -ieq 'main') { return 'poussée vers main interdite (SEC0-13)' }
      }
      if ($pos.Count -le 1 -and (Get-Branch $Dir $gargs) -ieq 'main') { return 'poussée depuis main interdite (SEC0-13)' }
    }
    'update-ref' {
      if ($a -contains '--stdin' -or ($a | Where-Object { Test-MainRef $_ })) { return 'git update-ref sur main interdit (SEC0-14)' }
    }
    'branch' {
      $dangerous = $a | Where-Object { $_ -in '--force', '--delete', '--move', '--copy' -or $_ -cmatch '^-[a-zA-Z]*[fDdMmCc]' }
      if ($dangerous -and ($a | Where-Object { Test-MainRef $_ })) { return 'git branch -f/-d/-m main interdit (SEC0-14)' }
    }
    { $_ -in 'checkout', 'switch' } {
      for ($k = 0; $k -lt $a.Count - 1; $k++) {
        if ($a[$k] -cin '-B', '-C', '--force-create' -and (Test-MainRef $a[$k + 1])) { return 'recréer main est interdit (SEC0-14)' }
      }
    }
  }
  return $null
}

# Motif du refus pour toute la commande, ou $null.
function Get-CommandVerdict([string]$Command, [string]$Tool, [string]$Cwd) {
  if (-not $Cwd) { $Cwd = (Get-Location).Path }
  if ($Command -match '(?i)disableAllHooks' -and -not (Test-ToolingBranch (Get-Branch $Cwd))) {
    return 'disableAllHooks ne se mentionne que sur une branche claude/outillage-* (SEC0-35)'
  }
  if ($Command -match '(?i)GIT_CONFIG' -and $Command -match '(?i)hookspath') { return 'modifier core.hooksPath est interdit (SEC0-14)' }
  foreach ($naive in $false, $true) {
    $dir = $Cwd
    $segs = Split-Command $Command $Tool -Naive:$naive
    $all = @($segs | ForEach-Object { $_ })
    # SEC0-35 : écriture des réglages de Claude Code (redirection, commande ou méthode qui écrit).
    $targets = @($all | Where-Object { Test-SettingsFile $_ -Loose })
    if ($targets) {
      $writes = $Command -match '(?i)(Set-Content|Add-Content|Out-File|Clear-Content|WriteAll|::Write|::Copy|::Move|::Delete|\.Save\()'
      foreach ($s in $segs) {
        for ($k = 0; $k -lt $s.Count; $k++) {
          $w = $s[$k]
          if ($w -in '>', '>>' -and $k + 1 -lt $s.Count -and (Test-SettingsFile $s[$k + 1] -Loose)) { $writes = $true }
          if ($w -in 'tee', 'tee-object', 'cp', 'copy', 'copy-item', 'cpi', 'mv', 'move', 'move-item', 'mi', 'rm', 'del', 'erase',
              'remove-item', 'ri', 'rename-item', 'ren', 'rni', 'new-item', 'ni', 'sc', 'ac', 'clc', 'install', 'dd', 'truncate',
              'touch', 'ln', 'mklink', 'rsync', 'checkout', 'restore', 'apply', 'stash') { $writes = $true }
          if ($w -match '(?i)(^|[\\/])(sed|perl|ruby)(\.exe)?$' -and ($s | Where-Object { $_ -clike '-i*' -or $_ -like '--in-place*' })) { $writes = $true }
        }
      }
      if ($writes) {
        foreach ($t in $targets) {
          $p = ConvertTo-FullPath $t $dir
          if (-not (Test-ToolingBranch (Get-Branch (Split-Path $p -Parent)))) {
            return "écriture de $t interdite (SEC0-35) : réglages de Claude Code modifiables seulement sur une branche claude/outillage-*"
          }
        }
      }
    }
    foreach ($s in $segs) {
      if ($s[0] -in $cdCmds) {
        $arg = $s | Select-Object -Skip 1 | Where-Object { -not $_.StartsWith('-') } | Select-Object -First 1
        if ($arg) { $nd = ConvertTo-FullPath $arg $dir; if ($nd) { $dir = $nd } }
        continue
      }
      for ($k = 0; $k -lt $s.Count; $k++) {
        # Lecture stricte : git en position de commande seulement ; lecture naïve : partout.
        if (-not $naive -and $k -gt 0 -and -not ($s[$k - 1] -in 'env', 'command', 'exec', 'sudo', 'time', 'nohup', 'xargs', '.' -or $s[$k - 1] -match '^\w+=')) { continue }
        if (Test-Exe $s[$k] 'git') {
          $v = Get-GitVerdict $s $k $dir
          if ($v) { return $v }
        } elseif ((Test-Exe $s[$k] 'gh') -and ($s -contains 'api') -and ($s | Where-Object { $_ -match '(?i)git(/|%2F)refs' })) {
          return 'gh api sur git/refs est interdit (SEC0-14)'
        }
      }
    }
  }
  return $null
}

if ($MyInvocation.InvocationName -eq '.') { return }   # chargé par les tests

try {
  $in = Read-HookInput
  $cmd = "$($in.tool_input.command)"
  if ($cmd -match '(?i)git|gh|settings|disableAllHooks') {
    $v = Get-CommandVerdict $cmd "$($in.tool_name)" "$($in.cwd)"
    if ($v) { Exit-Deny "Commande refusée par guard-git : $v." }
  }
} catch {
  # Barrière : une erreur interne refuse, par prudence.
  Exit-Deny "guard-git : erreur interne ($($_.Exception.Message)) ; commande refusée par prudence."
}
exit 0
