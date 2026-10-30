# tools/dev/guard-git.ps1 : crochet PreToolUse de Claude Code pour les outils qui lancent une commande
# (Bash, PowerShell, Monitor, mcp__terminal__run_in_terminal).
# Refuse (code 2) les commandes de SEC0-14, les écritures sur main (locales ou côté GitHub) et
# l'écriture des réglages de Claude Code (SEC0-35). Deux lectures de la commande : l'une respecte
# guillemets et échappements, l'autre les ignore (bash -c "...", $(...)) ; un refus de l'une suffit.
# En cas de doute ou d'erreur interne : refus.
# spec: carnet architecture-securite.md SEC0-13, SEC0-14, SEC0-15, SEC0-35 ; revues/developpement-agents.md §4.4
. (Join-Path $PSScriptRoot 'guard-lib.ps1')

# Outils examinés et dialecte de leur champ command.
$CommandTools = @{ 'Bash' = 'Bash'; 'Monitor' = 'Bash'; 'PowerShell' = 'PowerShell'; 'mcp__terminal__run_in_terminal' = 'PowerShell' }

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
function Test-MainRef([string]$Ref) { return ($Ref -replace '/{2,}', '/') -match '^(?i)(refs/)?(heads/)?main$' }
function Test-Exe([string]$Tok, [string]$Name) { return $Tok -match "(?i)(^|[\\/])$Name(\.exe)?$" }
# Clés de configuration qui changent ce que Git exécute ou pousse (alias, inclusions, crochets, destinations).
function Test-ForbiddenKey([string]$KeyValue) {
  $k = ($KeyValue -split '=', 2)[0]
  return $k -match '^(?i)(alias\.|include\.|includeif\.|core\.hookspath$|remote\..+\.(push|mirror)$)'
}
$cdCmds = 'cd', 'chdir', 'pushd', 'sl', 'set-location', 'push-location'
$valueOpts = '-C', '-c', '--git-dir', '--work-tree', '--namespace'
$readCmds = 'cat', 'type', 'get-content', 'gc', 'less', 'more', 'head', 'tail', 'grep', 'rg', 'select-string', 'sls',
            'ls', 'dir', 'get-childitem', 'gci', 'test-path', 'get-filehash', 'sha256sum', 'wc'
$readGit = 'diff', 'show', 'log', 'status'
$escape = '--abort', '--quit'   # sortir d'une opération en cours reste permis sur main

# Analyse d'un appel git à partir de l'index $At du segment ; motif du refus ou $null.
function Get-GitVerdict([string[]]$T, [int]$At, [string]$Dir) {
  $seg = @($T | Select-Object -Skip ($At + 1))
  foreach ($x in $seg) { if (Test-Abbrev $x '--no-verify' 6) { return '--no-verify est interdit (SEC0-14)' } }
  $i = $At + 1; $gargs = @()
  while ($i -lt $T.Count -and $T[$i].StartsWith('-')) {
    $o = $T[$i]; $v = if ($i + 1 -lt $T.Count) { $T[$i + 1] } else { '' }
    $kv = if ($o -ceq '-c') { $v } elseif ($o -clike '-c?*') { $o.Substring(2) }
          elseif ($o -like '--config-env=*') { $o.Substring(13) } elseif ($o -eq '--config-env') { $v } else { $null }
    if (($kv -and (Test-ForbiddenKey $kv)) -or $o -match '(?i)hookspath') {
      return "-c/--config-env sur alias, include, core.hooksPath ou remote.*.push|mirror est interdit (SEC0-14)"
    }
    if ($o -ceq '-C') { $Dir = ConvertTo-FullPath $v $Dir }
    elseif ($o -in '--git-dir', '--work-tree') { $gargs += $o, $v }
    elseif ($o -like '--git-dir=*' -or $o -like '--work-tree=*') { $gargs += $o }
    $i += $(if ($o -cin $valueOpts -or $o -eq '--config-env') { 2 } else { 1 })
  }
  if ($i -ge $T.Count) { return $null }
  $sub = $T[$i].ToLowerInvariant(); $a = @($T | Select-Object -Skip ($i + 1))
  $onMain = { (Get-Branch $Dir $gargs) -ieq 'main' }
  $pos = @($a | Where-Object { -not $_.StartsWith('-') })
  switch ($sub) {
    'config' {
      $keys = @($a | Where-Object { (Test-ForbiddenKey $_) -or $_ -match '(?i)hookspath' })
      if ($keys -and -not ($a | Where-Object { $_ -in '--get', '--get-all', '--get-regexp', '-l', '--list', 'get', 'list' })) {
        return "git config en écriture sur $($keys[0]) est interdit (SEC0-14)"
      }
    }
    'commit' {
      foreach ($x in $a) {   # -n vaut --no-verify ; il peut être groupé (-an), avant une option à valeur
        if ($x -cmatch '^-[a-zA-Z]+$') { foreach ($ch in $x.Substring(1).ToCharArray()) {
          if ($ch -ceq 'n') { return 'git commit -n (--no-verify) est interdit (SEC0-14)' }
          if ('mCcFtSu'.Contains([string]$ch)) { break } } }
      }
      if (& $onMain) { return 'commit sur main interdit (SEC0-13) : travaillez sur une branche claude/<sujet>' }
    }
    'push' {
      $refs = @(); $skip = $false
      foreach ($x in $a) {
        if ($skip) { $skip = $false; continue }
        if ($x -in '--repo', '-o', '--push-option', '--receive-pack', '--exec') { $skip = $true; continue }
        if ((Test-Abbrev $x '--force' 4) -or (Test-Abbrev $x '--force-with-lease' 4) -or (Test-Abbrev $x '--force-if-includes' 4) -or
            ($x -cmatch '^-[a-zA-Z]*f')) { return 'poussée forcée interdite (SEC0-14)' }
        if ((Test-Abbrev $x '--mirror' 4) -or (Test-Abbrev $x '--all' 4)) { return 'git push --all/--mirror interdit : poussez une seule branche' }
        if (-not $x.StartsWith('-')) { $refs += $x }
      }
      foreach ($r in ($refs | Select-Object -Skip 1)) {
        if ($r.StartsWith('+')) { return 'destination préfixée par + (poussée forcée) interdite (SEC0-14)' }
        $dst = ($r -split ':')[-1]
        if (Test-MainRef $dst) { return 'poussée vers main interdite (SEC0-13) : passez par une PR' }
        if ($dst -match '^(?i)(refs/)?(heads/)?pilotage$' -and ($r.StartsWith(':') -or $a -contains '--delete' -or $a -contains '-d')) {
          return 'suppression de la branche pilotage interdite'
        }
        if ($r -ieq 'HEAD' -and (& $onMain)) { return 'poussée vers main interdite (SEC0-13)' }
      }
      if ($refs.Count -le 1 -and (& $onMain)) { return 'poussée depuis main interdite (SEC0-13)' }
    }
    { $_ -in 'send-pack', 'http-push' -or $_ -like 'remote-*' } { return "git $sub est interdit : il pousse sans crochet pre-push" }
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
    { $_ -in 'revert', 'cherry-pick', 'rebase', 'am' } {
      if (-not ($a | Where-Object { $_ -in $escape }) -and (& $onMain)) { return "git $sub sur main interdit (SEC0-13) : main ne bouge que par git pull --ff-only" }
    }
    'reset' {
      $target = @($a | ForEach-Object -Begin { $stop = $false } -Process { if ($_ -eq '--') { $stop = $true } elseif (-not $stop -and -not $_.StartsWith('-')) { $_ } })
      if ($target -and (& $onMain)) { return 'git reset vers un commit sur main interdit (SEC0-13) : main ne bouge que par git pull --ff-only' }
    }
    { $_ -in 'merge', 'pull' } {
      if (-not ($a | Where-Object { $_ -in @('--ff-only') + $escape }) -and (& $onMain)) { return "git $sub sans --ff-only sur main interdit (SEC0-13)" }
    }
  }
  return $null
}

# Analyse d'un appel gh à partir de l'index $At ; motif du refus ou $null.
function Get-GhVerdict([string[]]$T, [int]$At) {
  $a = @($T | Select-Object -Skip ($At + 1))
  $words = @($a | Where-Object { -not $_.StartsWith('-') })
  $pr = [array]::IndexOf($words, 'pr')
  if ($pr -ge 0 -and $pr + 1 -lt $words.Count -and $words[$pr + 1] -eq 'merge') {
    return 'gh pr merge est interdit : fusion par pwsh tools/dev/merge-pr.ps1 <PR> depuis la copie principale (SEC0-15)'
  }
  if ($a -notcontains 'api') { return $null }
  for ($k = 0; $k -lt $a.Count; $k++) {
    $x = $a[$k]; $next = if ($k + 1 -lt $a.Count) { $a[$k + 1] } else { '' }
    $method = if ($x -ceq '-X' -or $x -eq '--method') { $next } elseif ($x -clike '-X?*') { $x.Substring(2) } elseif ($x -like '--method=*') { $x.Substring(9) } else { $null }
    if ($method -and $method -ine 'GET') { return "gh api en écriture ($method) est interdit (SEC0-14)" }
    if ($x -cmatch '^-[fF]' -or $x -match '^--(field|raw-field|input)') { return 'gh api avec des champs (-f, -F, --input) est interdit (SEC0-14)' }
    if ($x -match '(?i)(contents/|merges|pulls/[^/]+/merge|(^|/)git/|(^|/)graphql$)') { return "gh api sur $x est interdit (SEC0-14)" }
  }
  return $null
}

# Mot qui désigne un fichier de réglages de Claude Code, un motif qui peut le désigner, ou le dossier .claude.
function Test-SettingsMention([string]$Tok) {
  return $Tok -match '(?i)settings(\.local)?\.json|sett[^\s\\/]*[\*\?\[\{]|settin~\d' -or
         $Tok -match '(?i)(^|[\\/])\.claude([\\/]+[^\\/]*[\*\?\[\{][^\\/]*)?[\\/]*$'
}

# Chemin désigné par un mot (variables du dossier personnel développées) ; $null s'il reste une variable.
function Resolve-TokenPath([string]$Tok, [string]$Dir) {
  $m = [regex]::Match($Tok, '[^\s''"(),;=]*?(\.claude|sett)[^\s''"(),;]*')
  $p = if ($m.Success) { $m.Value } else { $Tok }
  foreach ($v in '${HOME}', '$HOME', '$env:HOME', '$env:USERPROFILE', '${env:USERPROFILE}', '%USERPROFILE%') {
    $p = $p.Replace($v, $HOME, [StringComparison]::OrdinalIgnoreCase)
  }
  if ($p -match '[\$%]') { return $null }   # variable non développée : cible inconnue
  try { return ConvertTo-CanonicalPath ($p -replace '^~(?=[\\/]|$)', $HOME) $Dir } catch { return $null }
}

# Segment de pure lecture : commande de tête dans la liste blanche, et aucune redirection.
function Test-ReadOnlySegment([string[]]$S) {
  if ($S | Where-Object { $_ -in '>', '>>' -or $_ -like '--output*' }) { return $false }
  $h = @($S | Where-Object { $_ -notmatch '^\w+=' })
  if (-not $h) { return $false }
  $head = ($h[0] -replace '(?i)\.exe$', '' -split '[\\/]')[-1].ToLowerInvariant()
  if ($head -in $readCmds -or $head -in $cdCmds) { return $true }
  if ($head -eq 'git') {
    $sub = $h | Select-Object -Skip 1 | Where-Object { -not $_.StartsWith('-') } | Select-Object -First 1
    return $sub -in $readGit -and -not ($h | Where-Object { $_ -match '^-[cC]$|^--(git-dir|work-tree)' })
  }
  return $false
}

# Motif du refus pour toute la commande, ou $null.
function Get-CommandVerdict([string]$Command, [string]$Tool, [string]$Cwd) {
  if (-not $Cwd) { $Cwd = (Get-Location).Path }
  $flat = $Command -replace '[''"`+\s\\]', ''
  if (($Command -match '(?i)disableAllHooks' -or $flat -match '(?i)disableAllHooks') -and -not (Test-ToolingBranch (Get-Branch $Cwd))) {
    return 'disableAllHooks ne se mentionne que sur une branche claude/outillage-* (SEC0-35)'
  }
  $hasGit = $Command -match '(?i)git'
  if ($hasGit -and $Command -match '(?i)GIT_CONFIG') { return 'GIT_CONFIG_* est interdit dans une commande git (SEC0-14)' }
  $danger = $Command -match '(?i)\b(push|commit|update-ref|merge|reset|rebase|config|send-pack)\b'
  if ($danger -and $Command -match '(?i)GIT_(DIR|WORK_TREE|COMMON_DIR|INDEX_FILE)') { return 'GIT_DIR/GIT_WORK_TREE avec une écriture Git est interdit, par prudence' }
  # Substitution, évaluation, ou commande tirée d'une variable ou d'une expression (& (...), & $x, $x push).
  $subst = $Command -match '\$\(|\$''|\$\{|(?i)\biex\b|(?i)invoke-expression|(?i)\bcmd(\.exe)?\s+/[ck]|[&.]\s*[\(\$]|(^|[;&|\n])\s*\$\w' -or
           ($Tool -eq 'Bash' -and $Command.Contains('`')) -or ($Command -match '(?i)\bcmd\b' -and $Command.Contains('^'))
  if ($subst -and $danger) { return 'substitution ($(...), iex, cmd /c...) avec push, commit, merge, reset, rebase ou config : refus par prudence' }

  foreach ($naive in $false, $true) {
    $dir = $Cwd
    $segs = Split-Command $Command $Tool -Naive:$naive
    foreach ($s in $segs) {
      # SEC0-35 : un segment qui désigne les réglages doit être une pure lecture, sauf sur claude/outillage-*.
      $mentions = @($s | Where-Object { Test-SettingsMention $_ })
      if ($mentions -and -not (Test-ReadOnlySegment $s)) {
        foreach ($t in $mentions) {
          $p = Resolve-TokenPath $t $dir
          $user = $p -and (Test-UnderPath $p (Join-Path $HOME '.claude'))
          if (-not $p -or $user -or -not (Test-ToolingBranch (Get-Branch (Split-Path $p -Parent)))) {
            return "écriture possible de $t interdite (SEC0-35) : réglages de Claude Code en lecture seule (lecture permise par cat, Get-Content, grep, git diff...)"
          }
        }
      }
      # Configuration de Git (core.hooksPath, .git/config, config.worktree, crochets, ~/.gitconfig) : lecture seule hors de git.
      $isGit = [bool]($s | Select-Object -First 3 | Where-Object { Test-Exe $_ 'git' })
      if (-not $isGit -and -not (Test-ReadOnlySegment $s) -and
          ($s | Where-Object { $_ -match '(?i)hookspath|(^|[\\/])\.git[\\/]+(config|config\.worktree|hooks)([\\/]|$)|\.gitconfig$' })) {
        return 'écriture possible de la configuration de Git (core.hooksPath, .git/config, crochets) interdite (SEC0-14)'
      }
      if ($s[0] -in $cdCmds) {
        $arg = $s | Select-Object -Skip 1 | Where-Object { -not $_.StartsWith('-') } | Select-Object -First 1
        if ($arg) { $nd = ConvertTo-FullPath $arg $dir; if ($nd) { $dir = $nd } }
        continue
      }
      for ($k = 0; $k -lt $s.Count; $k++) {
        # Lecture stricte : git en position de commande seulement ; lecture naïve : partout.
        if (-not $naive -and $k -gt 0 -and -not ($s[$k - 1] -in 'env', 'command', 'exec', 'sudo', 'time', 'nohup', 'xargs', '.' -or $s[$k - 1] -match '^\w+=')) { continue }
        $v = if (Test-Exe $s[$k] 'git') { Get-GitVerdict $s $k $dir } elseif (Test-Exe $s[$k] 'gh') { Get-GhVerdict $s $k } else { $null }
        if ($v) { return $v }
      }
    }
  }
  return $null
}

if ($MyInvocation.InvocationName -eq '.') { return }   # chargé par les tests

try {
  $in = Read-HookInput
  $tool = "$($in.tool_name)"
  if (-not $CommandTools.ContainsKey($tool)) { exit 0 }
  $ti = $in.tool_input
  if ($tool -eq 'Monitor' -and $null -eq $ti.command -and $ti.ws) { exit 0 }   # WebSocket : aucune commande
  if ($ti.command -isnot [string]) { Exit-Deny "Commande refusée par guard-git : champ command absent ou illisible pour $tool, refus par prudence." }
  $cwd = "$($in.cwd)"
  if ($ti.cwd) { $cwd = ConvertTo-FullPath "$($ti.cwd)" $cwd }
  $v = Get-CommandVerdict $ti.command $CommandTools[$tool] $cwd
  if ($v) { Exit-Deny "Commande refusée par guard-git : $v." }
} catch {
  Exit-Deny "guard-git : erreur interne ($($_.Exception.Message)) ; commande refusée par prudence."
}
exit 0
