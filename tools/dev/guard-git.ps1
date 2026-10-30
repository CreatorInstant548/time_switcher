# tools/dev/guard-git.ps1 : crochet PreToolUse de Claude Code pour les outils qui lancent une commande
# (Bash, PowerShell, Monitor, mcp__terminal__run_in_terminal). Refuse (code 2) par LISTE BLANCHE :
# - git : options globales -C et --no-pager seulement ; sous-commandes permises, chacune avec ses règles ;
#   toute autre sous-commande, tout alias, tout préfixe d'environnement : refus (SEC0-13, SEC0-14) ;
# - gh : sous-commandes permises, api en lecture seule ; gh pr merge refusé (fusion par cargo xtask merge, SEC0-15) ;
# - cargo xtask merge <PR> et cargo xtask hooks install : seulement depuis la racine de la copie principale ;
#   merge --dry-run partout ; toute autre forme (cargo run -p xtask … merge, binaire xtask, Start-Process,
#   chaîne) refusée, ainsi que toute variable de cargo ou rustc modifiée à côté (J2, I1, m4) ;
# - pwsh [-NoProfile -File] tools/dev/repo-settings.ps1 -Check|-Apply : forme exacte, seule, depuis la racine
#   de la copie principale (AS-30 R5, §9.12 S4) ;
# - réglages de Claude Code et configuration de Git : pure lecture seulement (SEC0-35) ;
# - substitution ou évaluation avec un mot d'écriture Git : refus par prudence.
# Les corps de heredoc cités (<<'EOF') et de here-string @'...'@ sont des données ; bash -c, pwsh -Command
# et cmd /c sont relus récursivement. En cas de doute ou d'erreur interne : refus.
# spec: carnet architecture-securite.md SEC0-13, SEC0-14, SEC0-15, SEC0-35 ; revues/developpement-agents.md §4.4
. (Join-Path $PSScriptRoot 'guard-lib.ps1')

$CommandTools = @{ 'Bash' = 'Bash'; 'Monitor' = 'Bash'; 'PowerShell' = 'PowerShell'; 'mcp__terminal__run_in_terminal' = 'PowerShell' }
$HintCommit = 'pour un message : git commit -F <fichier> ; pour gh : --body-file <fichier>'

# Découpe en segments (séparés par ; & | ( ) { } et fin de ligne), chacun une liste de mots sans guillemets.
# Les redirections deviennent des mots '>', '>>', '>&'.
function Split-Command([string]$Text, [string]$Tool) {
  $segs = [Collections.Generic.List[object]]::new()
  $cur = [Collections.Generic.List[string]]::new()
  $sb = [Text.StringBuilder]::new(); $has = $false; $q = $null
  $esc = if ($Tool -eq 'PowerShell') { '`' } else { '\' }
  $braces = 0   # accolades ouvertes dans le mot courant : repos/{owner}/{repo}, HEAD@{0} restent des mots entiers
  $flush = { if ($has) { $cur.Add($sb.ToString()) }; [void]$sb.Clear(); Set-Variable has $false -Scope 1; Set-Variable braces 0 -Scope 1 }
  for ($i = 0; $i -lt $Text.Length; $i++) {
    $c = [string]$Text[$i]
    if ($q -eq "'") { if ($c -eq "'") { $q = $null } else { [void]$sb.Append($c) }; continue }
    if ($c -eq $esc -and $i + 1 -lt $Text.Length -and
        ($null -eq $q -or $Tool -eq 'PowerShell' -or '$`"\'.Contains([string]$Text[$i + 1]))) {
      $i++; [void]$sb.Append($Text[$i]); $has = $true; continue
    }
    if ($q -eq '"') { if ($c -eq '"') { $q = $null } else { [void]$sb.Append($c) }; continue }
    if ($c -eq "'" -or $c -eq '"') { $q = $c; $has = $true; continue }
    if (" `t`r".Contains($c)) { & $flush; continue }
    if ($c -eq '>') {
      # Descripteur numérique collé (2>, 1>>, 2>&1) : il fait partie de la redirection, pas de la commande.
      if ($has -and $sb.ToString() -match '^\d+$') { [void]$sb.Clear(); $has = $false; $braces = 0 }
      & $flush; $r = '>'
      if ($i + 1 -lt $Text.Length -and $Text[$i + 1] -eq '>') { $r = '>>'; $i++ }
      if ($i + 1 -lt $Text.Length -and $Text[$i + 1] -eq '&') { $r = '>&'; $i++ }
      $cur.Add($r); continue
    }
    # « { » ne coupe qu'en début de mot ; la « } » qui la ferme dans le même mot reste littérale.
    if ($c -eq '{' -and $has) { $braces++; [void]$sb.Append($c); continue }
    if ($c -eq '}' -and $braces -gt 0) { $braces--; [void]$sb.Append($c); continue }
    if (";&|`n(){}".Contains($c)) { & $flush; if ($cur.Count) { $segs.Add($cur.ToArray()) }; $cur.Clear(); continue }
    [void]$sb.Append($c); $has = $true
  }
  & $flush; if ($cur.Count) { $segs.Add($cur.ToArray()) }
  return , $segs.ToArray()
}

# Retire les corps de heredoc à délimiteur cité (Bash) et de here-string littérale @'...'@ (PowerShell) : ce sont des données.
function Remove-LiteralBodies([string]$Text) {
  $out = [Collections.Generic.List[string]]::new(); $end = $null; $dash = $false; $ps = $false
  foreach ($l in ($Text -split "`n")) {
    $t = $l.TrimEnd("`r")
    if ($ps) { if ($t -match "^'@(.*)$") { $ps = $false; $out.Add($Matches[1]) }; continue }
    if ($end) { if ($(if ($dash) { $t.TrimStart("`t") } else { $t }) -eq $end) { $end = $null }; continue }
    $out.Add($l)
    if ($t -match "@'\s*$") { $ps = $true }
    elseif ($t -match "<<(-?)\s*(?:'([^']+)'|`"([^`"]+)`"|\\(\w+))") { $dash = $Matches[1] -eq '-'; $end = "$($Matches[2])$($Matches[3])$($Matches[4])" }
  }
  return $out -join "`n"
}

function Test-Abbrev([string]$Tok, [string]$Full, [int]$Min) {   # abréviation d'option longue admise par Git
  $t = ($Tok -split '=', 2)[0]
  return $t.Length -ge $Min -and $Full.StartsWith($t, [StringComparison]::OrdinalIgnoreCase)
}
function Test-MainRef([string]$Ref) { return ($Ref -replace '/{2,}', '/') -match '^(?i)(refs/)?(heads/)?main$' }
# main exactement, casse comprise : seule forme que garde le ruleset main de GitHub (AS-31).
function Test-MainRefExact([string]$Ref) { return ($Ref -replace '/{2,}', '/') -cmatch '^(refs/)?(heads/)?main$' }
function Test-OriginMain([string]$Ref) { return $Ref -match '^(?i)(refs/remotes/)?origin/main$' }
function Test-Exe([string]$Tok, [string]$Name) { return $Tok -match "(?i)(^|[\\/])$Name(\.exe)?$" }
function Get-Head([string[]]$S) { $h = @($S | Where-Object { $_ -notmatch '^\w+=' })[0]; return (("$h" -replace '(?i)\.exe$', '') -split '[\\/]')[-1].ToLowerInvariant() }

$cdCmds = 'cd', 'chdir', 'pushd', 'sl', 'set-location', 'push-location'
$readCmds = 'cat', 'type', 'get-content', 'gc', 'less', 'more', 'head', 'tail', 'grep', 'rg', 'select-string', 'sls',
            'ls', 'dir', 'get-childitem', 'gci', 'test-path', 'get-filehash', 'sha256sum', 'wc'
$readGit = 'diff', 'show', 'log', 'status', 'blame', 'ls-files', 'grep', 'cat-file'
# Commandes dont les arguments sont des données : un mot « git » y est du texte, pas un appel.
$dataCmds = $readCmds + 'echo', 'printf', 'write-host', 'write-output', 'write-error', 'findstr', 'sed', 'awk', 'jq', 'sort', 'git', 'gh'
$prefixCmds = 'command', 'exec', 'sudo', 'time', 'nohup', 'xargs', 'nice', 'timeout', 'watch', '-exec', '-execdir', '.'
$shells = 'bash', 'sh', 'zsh', 'dash', 'ksh', 'pwsh', 'powershell', 'cmd', 'eval'
$escape = '--abort', '--quit'
$gitRead = 'status', 'log', 'show', 'diff', 'blame', 'grep', 'describe', 'rev-parse', 'ls-files', 'ls-remote', 'cat-file',
           'merge-base', 'shortlog', 'rev-list', 'show-ref', 'for-each-ref', 'name-rev', 'check-ignore', 'check-attr',
           'count-objects', 'version', 'help', 'add', 'rm', 'mv', 'restore', 'clean'

# Arguments sans les redirections (>, >>, >&) ni leur cible.
function Remove-Redirections([string[]]$A) {
  $out = [Collections.Generic.List[string]]::new()
  for ($k = 0; $k -lt $A.Count; $k++) { if ($A[$k] -in '>', '>>', '>&') { $k++ } else { $out.Add($A[$k]) } }
  return , $out.ToArray()
}

# Analyse d'un appel git à l'index $At du segment ; motif du refus ou $null.
function Get-GitVerdict([string[]]$T, [int]$At, [string]$Dir) {
  for ($j = 0; $j -lt $At; $j++) {
    if ($T[$j] -match '^\w+=' -or $T[$j] -eq 'env') { return "préfixe d'environnement ($($T[$j])) sur un appel git interdit (SEC0-14)" }
  }
  $seg = @($T | Select-Object -Skip ($At + 1))
  foreach ($x in $seg) {
    if (Test-Abbrev $x '--no-verify' 6) { return '--no-verify est interdit (SEC0-14)' }
    if ($x -like '--output*') { return 'git --output est interdit : rediriger la lecture autrement' }
  }
  $i = $At + 1
  while ($i -lt $T.Count -and $T[$i].StartsWith('-')) {
    $o = $T[$i]
    if ($o -ceq '-C') { $Dir = ConvertTo-FullPath $T[$i + 1] $Dir; $i += 2; continue }
    if ($o -in '--no-pager', '-P', '--version', '--help') { $i++; continue }
    return "option globale de git « $o » interdite : seules -C <dossier> et --no-pager sont permises (SEC0-14)"
  }
  if ($i -ge $T.Count) { return $null }
  $sub = $T[$i].ToLowerInvariant(); $a = Remove-Redirections @($T | Select-Object -Skip ($i + 1))
  $pos = @($a | Where-Object { -not $_.StartsWith('-') })
  $onMain = { (Get-Branch $Dir) -ieq 'main' }
  $hasEscape = [bool]($a | Where-Object { $_ -in $escape })
  switch ($sub) {
    { $_ -in $gitRead } { return $null }
    'commit' {
      foreach ($x in $a) {   # -n vaut --no-verify ; il peut être groupé (-an), avant une option à valeur
        if ($x -cmatch '^-[a-zA-Z]+$') { foreach ($ch in $x.Substring(1).ToCharArray()) {
          if ($ch -ceq 'n') { return 'git commit -n (--no-verify) est interdit (SEC0-14)' }
          if ('mCcFtSu'.Contains([string]$ch)) { break } } }
      }
      if (& $onMain) { return 'commit sur main interdit (SEC0-13) : travaillez sur une branche claude/<sujet>' }
      return $null
    }
    { $_ -in 'switch', 'checkout' } {
      for ($k = 0; $k -lt $a.Count - 1; $k++) {
        if ($a[$k] -cin '-B', '-C', '-b', '-c', '--force-create', '--create' -and (Test-MainRef $a[$k + 1])) { return 'créer ou recréer main est interdit (SEC0-14)' }
      }
      return $null
    }
    'branch' {
      $vals = '--merged', '--no-merged', '--contains', '--no-contains', '--points-at', '-u', '--set-upstream-to', '--sort', '--format'
      $targets = @(); for ($k = 0; $k -lt $a.Count; $k++) {
        if ($a[$k] -in $vals) { $k++; continue }
        if (-not $a[$k].StartsWith('-')) { $targets += $a[$k] }
      }
      if ($targets | Where-Object { Test-MainRef $_ }) {
        $list = $a | Where-Object { $_ -in '-l', '--list', '-a', '-r', '--all', '--remotes' }
        if (-not $list) { return 'git branch sur main (création, -f, -d, -m, -c) interdit (SEC0-14)' }
      }
      return $null
    }
    'fetch' {
      if ($pos.Count -gt 1 -or ($a | Where-Object { $_ -match ':|(?i)refs/' })) {
        return 'git fetch avec une refspec explicite est interdit (refs/remotes/ ne bouge que par git fetch <distant>)'
      }
      return $null
    }
    'pull' {
      if ($a | Where-Object { $_ -match ':|(?i)refs/' }) { return 'git pull avec une refspec explicite est interdit' }
      if ($a -notcontains '--ff-only' -and (& $onMain)) { return 'sur main, git pull sans --ff-only est interdit : git pull --ff-only' }
      return $null
    }
    'push' {
      # AS-31, T3e-1 : la poussée directe et la poussée forcée vers main ne sont plus refusées ici,
      # le ruleset main (26213563) de GitHub les refuse sans contournement. Seulement vers le distant
      # origin, nommé : le ruleset ne garde que lui. La poussée forcée reste refusée vers toute autre
      # destination, et vers une destination implicite (sans refspec, HEAD) ; main s'écrit en
      # minuscules, seule forme que garde le ruleset (Main, MAIN : poussée forcée refusée).
      $refs = @(); $skip = $false; $force = $false; $repoOpt = $false; $delete = $false; $tags = $false
      foreach ($x in $a) {
        if ($skip) { $skip = $false; continue }
        if ($x -in '--repo', '-o', '--push-option', '--receive-pack', '--exec') { $repoOpt = $repoOpt -or $x -eq '--repo'; $skip = $true; continue }
        if ((Test-Abbrev $x '--delete' 4) -or $x -cmatch '^-[a-zA-Z]*d') { $delete = $true }
        if ((Test-Abbrev $x '--tags' 4) -or (Test-Abbrev $x '--follow-tags' 4)) { $tags = $true }
        if ((Test-Abbrev $x '--force' 4) -or (Test-Abbrev $x '--force-with-lease' 4) -or (Test-Abbrev $x '--force-if-includes' 4) -or
            ($x -cmatch '^-[a-zA-Z]*f')) { $force = $true; continue }
        if ((Test-Abbrev $x '--mirror' 4) -or (Test-Abbrev $x '--all' 4) -or (Test-Abbrev $x '--branches' 4)) {
          return 'git push --all/--branches/--mirror interdit : poussez une seule branche'
        }
        if ($x -like '--repo=*' -or $x -like '--receive-pack=*' -or $x -like '--exec=*') { return "git push $x interdit" }
        if (-not $x.StartsWith('-')) { $refs += $x }
      }
      if ($refs.Count -ge 1 -and $refs[0] -eq '.') { return 'git push vers le dépôt local (.) interdit' }
      if ($force -and $refs.Count -le 1) { return 'poussée forcée sans destination explicite interdite (SEC0-14) : seule main, nommée, peut être visée' }
      if ($force -and $tags) { return 'poussée forcée avec --tags ou --follow-tags interdite (SEC0-14) : les étiquettes seraient forcées aussi' }
      $viaOrigin = $refs.Count -ge 1 -and $refs[0] -ceq 'origin' -and -not $repoOpt
      $mainHint = 'poussée vers main permise seulement vers le distant origin, nommé, que garde le ruleset de GitHub (SEC0-13, AS-31)'
      if ($refs.Count -le 1 -and -not $viaOrigin -and (& $onMain)) { return $mainHint }
      foreach ($r in ($refs | Select-Object -Skip 1)) {
        $plus = $r.StartsWith('+'); $spec = if ($plus) { $r.Substring(1) } else { $r }
        if ($spec -eq ':' -or $spec.Contains('*')) { return 'refspec de correspondance (:) ou à motif (*) interdite : poussez une seule branche' }
        $dst = ($spec -split ':')[-1]
        $toMain = Test-MainRef $dst
        if ($toMain -and ($spec.StartsWith(':') -or $delete)) { return 'suppression de la branche main interdite (SEC0-13)' }
        if (($toMain -or ($dst -in 'HEAD', '@' -and (& $onMain))) -and -not $viaOrigin) { return $mainHint }
        if (($plus -or $force) -and -not (Test-MainRefExact $dst)) { return 'poussée forcée interdite hors de main (SEC0-14)' }
        if ($dst -match '(?i)^refs/remotes/') { return 'poussée vers refs/remotes/ interdite' }
        if ($dst -match '^(?i)(refs/)?(heads/)?pilotage$' -and ($spec.StartsWith(':') -or $delete)) {
          return 'suppression de la branche pilotage interdite'
        }
      }
      return $null
    }
    'merge' {
      if ($hasEscape -or -not (& $onMain)) { return $null }
      if ($a -contains '--ff-only' -and $pos.Count -eq 1 -and (Test-OriginMain $pos[0])) { return $null }
      return 'sur main, seul git merge --ff-only origin/main est permis (SEC0-13) ; sinon git pull --ff-only'
    }
    { $_ -in 'rebase', 'revert', 'cherry-pick' } {
      if ($hasEscape) { return $null }
      if ($sub -eq 'rebase' -and ($pos | Where-Object { Test-MainRef $_ })) { return 'git rebase qui réécrit main interdit (SEC0-13)' }
      if (& $onMain) { return "git $sub sur main interdit (SEC0-13) : main ne bouge que par git pull --ff-only" }
      return $null
    }
    'reset' {
      $target = @($a | ForEach-Object -Begin { $stop = $false } -Process {
        if ($_ -eq '--') { $stop = $true } elseif (-not $stop -and -not $_.StartsWith('-') -and $_ -ine 'HEAD') { $_ } })
      if (-not $target -or -not (& $onMain)) { return $null }
      if ($target.Count -eq 1 -and (Test-OriginMain $target[0]) -and ($a | Where-Object { $_ -in '--hard', '--keep' })) { return $null }
      return 'sur main, seul git reset --hard origin/main est permis (SEC0-13)'
    }
    'stash' {
      if (-not $pos -or $pos[0] -in 'push', 'list', 'apply', 'drop', 'show') { return $null }
      return "git stash $($pos[0]) interdit : push, list, apply, drop et show seulement (pile partagée entre sessions)"
    }
    'worktree' {
      if ($pos.Count -eq 0 -or $pos[0] -notin 'list', 'add', 'remove', 'prune') { return 'git worktree : list, add, remove et prune seulement' }
      for ($k = 0; $k -lt $a.Count - 1; $k++) { if ($a[$k] -cin '-B', '-b' -and (Test-MainRef $a[$k + 1])) { return 'créer main par git worktree est interdit' } }
      if ($a -ccontains '-B') { return 'git worktree add -B interdit : -b <nouvelle branche>' }
      return $null
    }
    'remote' {
      if ($a.Count -eq 0 -or ($a.Count -eq 1 -and $a[0] -in '-v', '--verbose')) { return $null }
      return 'git remote : lecture seule (git remote -v)'
    }
    'config' {
      $read = $a | Where-Object { $_ -in '--get', '--get-all', '--get-regexp', '-l', '--list', 'get', 'list' }
      $write = $a | Where-Object { $_ -match '^--(unset|add|replace-all|rename-section|remove-section|edit)|^-e$|^(set|unset|edit|rename-section|remove-section)$' }
      if ($read -and -not $write) { return $null }
      return 'git config en écriture est interdit : lecture seule (--get, --get-all, --list)'
    }
    'tag' {
      if ($a.Count -eq 0 -or (($a | Where-Object { $_ -in '-l', '--list' }) -and -not ($a | Where-Object { $_ -cmatch '^-[dfasmuF]|^--(delete|force|annotate|sign|message|file)' }))) { return $null }
      return 'git tag : lecture seule (git tag -l)'
    }
    'lfs' {
      if ($pos.Count -ge 1 -and $pos[0] -in 'install', 'uninstall', 'update') { return "git lfs $($pos[0]) interdit : il touche aux crochets" }
      return $null
    }
  }
  return "sous-commande git « $sub » non permise (alias ou commande hors liste blanche, SEC0-14)"
}

# Analyse d'un appel gh à l'index $At ; motif du refus ou $null.
function Get-GhVerdict([string[]]$T, [int]$At) {
  $a = Remove-Redirections @($T | Select-Object -Skip ($At + 1)); $words = @()
  for ($k = 0; $k -lt $a.Count; $k++) {
    if ($a[$k] -in '-R', '--repo') { $k++; continue }
    if (-not $a[$k].StartsWith('-')) { $words += $a[$k] }
  }
  $top = "$($words[0])".ToLowerInvariant(); $sub = "$($words[1])".ToLowerInvariant()
  $allowed = @{ 'pr' = 'create', 'view', 'list', 'diff', 'checks', 'comment', 'status', 'close', 'edit', 'ready'
                'issue' = 'create', 'view', 'list', 'comment', 'edit', 'close', 'reopen'
                'run' = 'list', 'view', 'watch', 'rerun'; 'workflow' = 'list', 'view'; 'repo' = @('view'); 'auth' = @('status')
                'label' = 'list', 'create', 'edit' }
  if (-not $top -and ($a | Where-Object { $_ -in '--version', '--help', '-h' })) { return $null }
  if ($top -in 'version', 'help') { return $null }
  if ($allowed.ContainsKey($top)) {
    if ($top -eq 'pr' -and $sub -eq 'edit' -and ($a | Where-Object { $_ -eq '--base' -or $_ -like '--base=*' -or $_ -ceq '-B' })) {
      return 'gh pr edit --base est interdit : la base d''une PR reste main'
    }
    if ($sub -in $allowed[$top]) { return $null }
    if ($top -eq 'pr' -and $sub -eq 'merge') { return 'gh pr merge est interdit : fusion par cargo xtask merge <PR> depuis la racine de la copie principale (SEC0-15)' }
    return "gh $top $sub non permis (liste blanche de gh)"
  }
  if ($top -ne 'api') { return "gh $top non permis (alias ou commande hors liste blanche de gh)" }
  return Get-GhApiArgsVerdict $a
}

# Mots d'un segment hors valeurs de message de git et gh (--title, --body, -m…) : un titre de PR
# peut citer « cargo xtask merge » sans rien lancer.
function Get-ActiveWords([string[]]$S) {
  $head = Get-Head $S
  $msg = '--body', '--title', '-m', '--message', '-b', '-t'
  return @(for ($k = 0; $k -lt $S.Count; $k++) {
    if ($head -in 'git', 'gh' -and $k -gt 0 -and $S[$k - 1] -in $msg) { continue }
    if ($head -in 'git', 'gh' -and $S[$k] -match '^--(body|title|message)=') { continue }
    $S[$k]
  })
}

# J2, I1, m4 : la fusion (cargo xtask merge <PR>) et l'installation des crochets (cargo xtask hooks
# install) ne se lancent que sous leur forme exacte, depuis la racine de la copie principale, qui
# compile le xtask de main ; merge --dry-run (aucune écriture) partout. Dès qu'une commande cite
# xtask, tout segment qui cite merge ou install doit être une de ces formes exactes, ou une
# pure lecture (cat, grep, git diff…) : Start-Process, -ArgumentList, iex et les chaînes sont refusés.
function Get-XtaskVerdict([string[]]$S, [string]$Dir, [bool]$XtaskInCommand) {
  if (-not $XtaskInCommand -or (Test-ReadOnlySegment $S)) { return $null }
  $w = Get-ActiveWords $S
  if (-not ($w | Where-Object { $_ -match '(?i)merge|install' })) { return $null }
  $S = Remove-Redirections $S   # une sortie redirigée ne change pas ce qui est lancé
  $merge = $S.Count -in 4, 5 -and $S[0] -ceq 'cargo' -and $S[1] -ceq 'xtask' -and $S[2] -ceq 'merge' -and $S[3] -match '^\d+$' -and
           ($S.Count -eq 4 -or $S[4] -ceq '--dry-run')
  $install = $S.Count -eq 4 -and $S[0] -ceq 'cargo' -and $S[1] -ceq 'xtask' -and $S[2] -ceq 'hooks' -and $S[3] -ceq 'install'
  if (-not $merge -and -not $install) {
    return 'seules les formes « cargo xtask merge <PR> [--dry-run] » et « cargo xtask hooks install » lancent la fusion ou l''installation des crochets (J2)'
  }
  if ($merge -and $S.Count -eq 5) { return $null }
  $main = Get-MainCopyRootAt $Dir
  if (-not $main) { return "cargo xtask $($S[2]) se lance seulement depuis la racine de la copie principale, pas depuis $Dir (J2)" }
  return $null
}

# Racine de la copie principale (parent du dossier Git commun) si $Dir est cette racine, sinon ''.
function Get-MainCopyRootAt([string]$Dir) {
  $common = "$(& git -C $Dir rev-parse --path-format=absolute --git-common-dir 2>$null)".Trim()
  $main = if ($LASTEXITCODE -eq 0 -and $common) { Split-Path ([IO.Path]::GetFullPath($common)) -Parent } else { '' }
  $here = ConvertTo-FullPath $Dir $Dir
  if ($main -and $here -and $here.Equals($main, [StringComparison]::OrdinalIgnoreCase)) { return $main }
  return ''
}

# AS-30 R5, §9.12 S4 : tools/dev/repo-settings.ps1 (seul à écrire les réglages GitHub par gh api) ne se
# lance que sous une forme exacte, seule dans la commande, depuis la racine de la copie principale.
# Un segment de lecture ou de données (cat, grep, git, gh, echo…) peut le citer ; tout autre segment
# qui le cite (autre argument, autre chemin, Start-Process, &, bash -c, pwsh -Command, motif) : refus.
$RepoSettingsForms = 'pwsh tools/dev/repo-settings.ps1 -Check', 'pwsh tools/dev/repo-settings.ps1 -Apply',
                     'pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Check', 'pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Apply'
# Mot qui désigne le script : son nom ; son nom court 8.3 (REPO-S~1.PS1) ; un motif dans tools/dev/ ; un
# motif dans tools/ qui contient repo-s, sett ou ~<chiffre> (tools/d?v/repo-setting?.ps1).
function Test-RepoSettingsMention([string]$Tok) {
  return $Tok -match '(?i)(^|[\\/\*\?\]])repo-settings(?!\.json$)|repo-s[^\\/\s]*~\d|[\\/]dev[\\/]+[^\\/\s]*[\*\?\[\{]' -or
         $Tok -match '(?i)^(?=.*tools[\\/])(?=.*[\*\?\[])(?=.*(repo-s|sett|~\d))'
}
function Get-RepoSettingsVerdict($Segments, [string]$Dir, [string]$Text) {
  # Nom fabriqué par concaténation ('repo-'+'settings.ps1', ("re"+"po-settings.ps1")) : refus.
  $flat = $Text -replace '[''"`+\s()]', ''
  $named = $Segments | ForEach-Object { $_ } | Where-Object { $_ -match '(?i)(?<![\w-])repo-settings' }
  if ($Text -match '(?i)repo-[''"]|[''"]settings\.ps1' -or ($flat -match '(?i)(?<![\w-])repo-settings(?!\.json)' -and -not $named)) {
    return 'nom de repo-settings.ps1 fabriqué par morceaux : refus (AS-30 R5, S4)'
  }
  $data = $readCmds + $cdCmds + 'echo', 'printf', 'write-host', 'write-output', 'git', 'gh'
  $calls = @($Segments | Where-Object { (Get-Head $_) -notin $data -and (Get-ActiveWords $_ | Where-Object { Test-RepoSettingsMention $_ }) })
  if (-not $calls) { return $null }
  if ($Segments.Count -ne 1 -or ($Segments[0] -join ' ') -cnotin $RepoSettingsForms) {
    return 'repo-settings.ps1 ne se lance que seul, sous la forme exacte « pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Check » (ou -Apply) (AS-30 R5, S4)'
  }
  if (-not (Get-MainCopyRootAt $Dir)) { return "repo-settings.ps1 se lance seulement depuis la racine de la copie principale, pas depuis $Dir (S4)" }
  return $null
}

# Arguments de gh api : lecture seule (GET, sans champs), hors chemins d'écriture ; motif du refus ou $null.
function Get-GhApiArgsVerdict([string[]]$A) {
  for ($k = 0; $k -lt $A.Count; $k++) {
    $x = $A[$k]; $next = if ($k + 1 -lt $A.Count) { $A[$k + 1] } else { '' }
    $method = if ($x -ceq '-X' -or $x -eq '--method') { $next } elseif ($x -clike '-X?*') { $x.Substring(2) } elseif ($x -like '--method=*') { $x.Substring(9) } else { $null }
    if ($method -and $method -ine 'GET') { return "gh api en écriture ($method) est interdit (SEC0-14)" }
    if ($x -cmatch '^-[fF]' -or $x -match '^--(field|raw-field|input)') { return 'gh api avec des champs (-f, -F, --input) est interdit (SEC0-14)' }
    if ($x -match '(?i)(contents/|merges|pulls/[^/\s]+/merge|(^|/)git/|(^|/)graphql$)') { return "gh api sur $x est interdit (SEC0-14)" }
  }
  return $null
}

# gh api relu sur tout le reste de la ligne, jusqu'à un | ; & hors guillemets : des parenthèses
# (gh api ("…") -X PUT) ou un découpage du chemin ne cachent ni la méthode, ni les champs, ni le chemin.
function Get-GhApiLineVerdict([string]$Text) {
  foreach ($m in [regex]::Matches($Text, '(?i)(?<![\w.-])gh(\.exe)?[''"]?((\s+-R|\s+--repo)\s+\S+)*\s+api\b(?<rest>[^\n]*)')) {
    $sb = [Text.StringBuilder]::new(); $q = $null
    foreach ($ch in $m.Groups['rest'].Value.ToCharArray()) {
      if ($q) { if ($ch -eq $q) { $q = $null } else { [void]$sb.Append($ch) }; continue }
      if ($ch -eq [char]"'" -or $ch -eq [char]'"') { $q = $ch; continue }
      if ('|;&'.Contains([string]$ch)) { break }
      [void]$sb.Append($(if ('()'.Contains([string]$ch)) { ' ' } else { $ch }))
    }
    $v = Get-GhApiArgsVerdict @($sb.ToString() -split '\s+' | Where-Object { $_ })
    if ($v) { return $v }
  }
  return $null
}

# Mot qui désigne un fichier de réglages de Claude Code, un motif qui peut le désigner, ou le dossier .claude.
function Test-SettingsMention([string]$Tok) {
  return $Tok -match '(?i)settings(\.local)?\.json|sett[^\s\\/]*[\*\?\[\{]|settin~\d' -or
         $Tok -match '(?i)(^|[\\/])\.claude([\\/]+[^\\/]*[\*\?\[\{][^\\/]*)?[\\/]*$'
}
# Mot qui désigne la configuration de Git (fichiers de configuration, attributs, crochets).
function Test-GitConfigMention([string]$Tok) {
  $t = $Tok -replace '[\\/]\.(?=[\\/])', '' -replace '[\\/]{2,}', '/'
  return $t -match '(?i)hookspath|config\.worktree|\.gitconfig$|(^|[\\/])\.config[\\/]git([\\/]|$)|(^|[\\/])\.git[\\/](config|hooks|info[\\/]attributes)([\\/]|$)'
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
  $head = Get-Head $S
  if ($head -in $readCmds -or $head -in $cdCmds) { return $true }
  if ($head -eq 'git') {
    $h = @($S | Where-Object { $_ -notmatch '^\w+=' })
    $sub = $h | Select-Object -Skip 1 | Where-Object { -not $_.StartsWith('-') } | Select-Object -First 1
    return $sub -in $readGit -and -not ($h | Where-Object { $_ -match '^-[cC]$|^--(git-dir|work-tree)' })
  }
  return $false
}

# Motif du refus pour toute la commande, ou $null.
function Get-CommandVerdict([string]$Command, [string]$Tool, [string]$Cwd, [int]$Depth = 0) {
  if ($Depth -gt 3) { return 'imbrication de shells trop profonde : refus par prudence' }
  if (-not $Cwd) { $Cwd = (Get-Location).Path }
  $text = Remove-LiteralBodies $Command
  $flat = $text -replace '[''"`+\s\\]', ''
  if (($text -match '(?i)disableAllHooks' -or $flat -match '(?i)disableAllHooks') -and -not (Test-ToolingBranch (Get-Branch $Cwd))) {
    return 'disableAllHooks ne se mentionne que sur une branche claude/outillage-* (SEC0-35)'
  }
  $hasGit = $text -match '(?i)(^|[^\w.-])git(\.exe)?([^\w.-]|$)'
  if ($hasGit -and $text -match '(?i)GIT_CONFIG|set-item\s+(-path\s+)?env:|new-item\s+(-path\s+)?env:|\$env:\w+\s*=|SetEnvironmentVariable|\bexport\s+\w+=|\bset\s+\w+=') {
    return "variable d'environnement modifiée dans une commande qui appelle git : refus (SEC0-14) ; lancer git dans une commande séparée"
  }
  if ($hasGit -and $text -match '(?i)\b(set-alias|new-alias|sal|nal)\b|\bfunction\s+[\w-]+|\balias\s+[\w-]+=') {
    return 'alias ou fonction définis dans une commande qui appelle git : refus (SEC0-14)'
  }
  if ($text -match '(?i)\b(pwsh|powershell)(\.exe)?\b.*\s-(e|ec|enc|encodedcommand)\s') { return 'pwsh -EncodedCommand est interdit : commande illisible' }
  # Substitution, évaluation, ou commande tirée d'une variable (seuls $(git rev-parse ...) et $(pwd) sont permis),
  # dans une commande qui appelle git ou gh, avec un mot d'écriture placé après git ou gh.
  # Une affectation en tête ($x = …, $env:X = …) n'est pas une commande tirée d'une variable.
  $scan = $text -replace '\$\(\s*(git\s+rev-parse(\s+--?[\w-]+)*|pwd)\s*\)', 'X'
  $subst = $scan -match '\$\(|\$''|\$\{|(?i)\biex\b|(?i)invoke-expression|(?i)\bcmd(\.exe)?\s+/[ck]|[&.]\s*[\(\$]|(^|[;&|\n])\s*\$[\w:]+(?![\w:]|\s*=)' -or
           ($Tool -eq 'Bash' -and $scan.Contains('`')) -or ($scan -match '(?i)\bcmd\b' -and $scan.Contains('^'))
  $danger = '\b(push|commit|update-ref|merge|reset|rebase|config|send-pack)\b'
  # Mot d'écriture placé : après git ou gh ; après une commande fabriquée en position de commande
  # (g${x}it, $'\x67it', g^it, ('gi'+'t')) ; ou n'importe où avec iex, Invoke-Expression, eval ou cmd /c.
  $hidden = $scan -match "(?is)(?<![\w.-])(git|gh)(\.exe)?\b.*$danger" -or
            $scan -match "(?im)(^|[;&|\n])\s*&?\s*\S*[\`$``^+'`"\\{}()]\S*\s+$danger" -or
            (($scan -match '(?i)\biex\b|invoke-expression|\beval\b|\bcmd(\.exe)?\s+/[ck]' -or ($Tool -eq 'Bash' -and $scan.Contains('`'))) -and
             $scan -match "(?i)$danger")
  if ($subst -and $hidden) {
    return "substitution (`$(...), iex, cmd /c...) avec une écriture Git : refus par prudence ; $HintCommit"
  }
  $v = Get-GhApiLineVerdict $text
  if ($v) { return $v }

  # I1 : dans une commande qui cite cargo ou xtask, aucune variable de cargo ni de rustc n'est
  # modifiée (alias, dossier cible, compilateur, chaîne d'outils).
  # Sans xtask dans la commande, CARGO_TARGET_DIR reste permis : c'est le repli des chemins longs de
  # session-start.ps1 ; un binaire xtask construit ainsi est de toute façon refusé par son empreinte.
  $toolVars = if ($text -match '(?i)xtask') { '(CARGO_\w*|RUSTC\w*|RUSTUP_\w*|RUSTFLAGS)' }
              else { '(CARGO_(?!TARGET_DIR\b)\w*|RUSTC\w*|RUSTUP_\w*|RUSTFLAGS)' }
  if ($text -match '(?i)cargo|xtask' -and (
      $text -match "(?i)(^|[\s;&|(])(\`$env:|\`$\{env:)?$toolVars\}?\s*=" -or
      $text -match "(?i)\b(set-item|new-item|si|ni|sv|set-variable)\b[^;|&\n]*env:$toolVars" -or
      $text -match "(?i)SetEnvironmentVariable\s*\(\s*['`"]$toolVars" -or
      $text -match "(?i)\b(export|set|setx)\s+(-\w+\s+)*$toolVars")) {
    return 'variable de cargo, rustc ou rustup modifiée dans une commande qui lance cargo ou xtask : refus (J2, I1)'
  }
  $segments = Split-Command $text $Tool
  $xtaskInCommand = [bool]($segments | ForEach-Object { Get-ActiveWords $_ } | Where-Object { $_ -match '(?i)xtask' })
  $v = Get-RepoSettingsVerdict $segments $Cwd $text
  if ($v) { return $v }
  $dir = $Cwd
  foreach ($s in $segments) {
    $head = Get-Head $s
    # SEC0-35 : un segment qui désigne les réglages doit être une pure lecture, sauf sur claude/outillage-*.
    # Les valeurs de --body, --title et -m de git ou gh sont des messages, pas des cibles.
    $mentions = @(); for ($k = 0; $k -lt $s.Count; $k++) {
      if ($head -in 'git', 'gh' -and $k -gt 0 -and $s[$k - 1] -in '--body', '--title', '-m', '--message', '-b', '-t') { continue }
      if ($head -in 'git', 'gh' -and $s[$k] -match '^--(body|title|message)=') { continue }
      if (Test-SettingsMention $s[$k]) { $mentions += $s[$k] }
    }
    if ($mentions -and -not (Test-ReadOnlySegment $s)) {
      foreach ($t in $mentions) {
        $p = Resolve-TokenPath $t $dir
        $user = $p -and (Test-UnderPath $p (Join-Path $HOME '.claude'))
        if (-not $p -or $user -or -not (Test-ToolingBranch (Get-Branch (Split-Path $p -Parent)))) {
          return "écriture possible de $t interdite (SEC0-35) : réglages de Claude Code en lecture seule (lecture permise par cat, Get-Content, grep, git diff...)"
        }
      }
    }
    # Un appel git du segment est jugé par sa liste blanche ; une redirection vers ces fichiers est refusée partout.
    $redirect = $(for ($k = 0; $k -lt $s.Count - 1; $k++) { if ($s[$k] -in '>', '>>' -and (Test-GitConfigMention $s[$k + 1])) { $true } })
    $callsGit = [bool]$(for ($k = 0; $k -lt $s.Count; $k++) {
      if ((Test-Exe $s[$k] 'git') -and ($k -eq 0 -or $s[$k - 1] -in $prefixCmds + '=' -or $s[$k - 1] -match '^\$[\w:]+=?$')) { $true } })
    if ($redirect -or (-not $callsGit -and -not (Test-ReadOnlySegment $s) -and ($s | Where-Object { Test-GitConfigMention $_ }))) {
      return 'écriture possible de la configuration de Git (.git/config, config.worktree, attributs, crochets, ~/.gitconfig) interdite (SEC0-14)'
    }
    if ($s[0] -in $cdCmds) {
      $arg = $s | Select-Object -Skip 1 | Where-Object { -not $_.StartsWith('-') } | Select-Object -First 1
      if ($arg -eq 'X') { continue }
      if ($arg) { $nd = ConvertTo-FullPath $arg $dir; if ($nd) { $dir = $nd } }
      continue
    }
    # Shells imbriqués : leur commande est relue.
    if ($head -in $shells) {
      $h = @($s | Where-Object { $_ -notmatch '^\w+=' })
      $idx = -1; for ($k = 1; $k -lt $h.Count; $k++) { if ($h[$k] -match '^(?i)(-c|-command|-com\w*|/c|/k)$') { $idx = $k; break } }
      if ($head -eq 'eval') { $idx = 0 }
      if ($idx -ge 0 -and $idx + 1 -lt $h.Count) {
        $inner = ($h | Select-Object -Skip ($idx + 1)) -join ' '
        $dialect = if ($head -in 'pwsh', 'powershell', 'cmd') { 'PowerShell' } else { 'Bash' }
        $v = Get-CommandVerdict $inner $dialect $dir ($Depth + 1)
        if ($v) { return $v }
      }
    }
    $v = Get-XtaskVerdict $s $dir $xtaskInCommand
    if ($v) { return $v }
    for ($k = 0; $k -lt $s.Count; $k++) {
      # git ou gh en position de commande, ou n'importe où hors des commandes dont les arguments sont des données.
      $atCmd = $k -eq 0 -or $s[$k - 1] -in $prefixCmds -or $s[$k - 1] -match '^\w+=' -or $s[$k - 1] -eq 'env'
      if (-not $atCmd -and $head -in $dataCmds) { continue }
      $v = if (Test-Exe $s[$k] 'git') { Get-GitVerdict $s $k $dir } elseif (Test-Exe $s[$k] 'gh') { Get-GhVerdict $s $k } else { $null }
      if ($v) { return $v }
      if ((Test-Exe $s[$k] 'git') -or (Test-Exe $s[$k] 'gh')) { break }   # le reste du segment appartient à cet appel
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
