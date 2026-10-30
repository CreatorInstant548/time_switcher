# tools/dev/test-repo-settings.ps1 : tests de repo-settings.ps1, sans appel à GitHub.
# Invariants du fichier (S2), comparaison des réponses, motifs de ref (S3), arguments et préalables,
# forme -Check -SourceCommit <sha> (AS-38) et lecture du fichier dans ce commit.
# Lit tools/ci/repo-settings.json de la copie de travail et les workflows d'origin/main. Sortie 1 si un cas échoue.
$dev = $PSScriptRoot
. (Join-Path $dev 'repo-settings.ps1')
$ErrorActionPreference = 'Stop'
$root = Split-Path (Split-Path $dev -Parent) -Parent
$text = Get-Content -Raw (Join-Path $root 'tools/ci/repo-settings.json')
function New-Cfg { return ConvertFrom-Json $text -AsHashtable -Depth 50 }
function Get-Rule($C, [string]$Key, [string]$Type) { return @($C[$Key]['rules']) | Where-Object { $_['type'] -ceq $Type } }

$fail = 0; $n = 0
function Check([string]$Name, [bool]$Ok) { $script:n++; if (-not $Ok) { $script:fail++; "ÉCHEC $Name" } }

$base = Get-Invalid (New-Cfg)
Check "fichier livré valide ($($base -join ' ; '))" ($base.Count -eq 0)

# S2 : chaque affaiblissement est refusé.
$mutations = [ordered]@{
  'bypass_actors non vide'           = { param($c) $c.ruleset_main['bypass_actors'] = @(@{ actor_id = 5; actor_type = 'RepositoryRole'; bypass_mode = 'always' }) }
  'bypass_actors de pilotage'        = { param($c) $c.ruleset_pilotage['bypass_actors'] = @(@{ actor_id = 1; actor_type = 'OrganizationAdmin'; bypass_mode = 'always' }) }
  'enforcement evaluate'             = { param($c) $c.ruleset_main['enforcement'] = 'evaluate' }
  'autre branche visée'              = { param($c) $c.ruleset_main['conditions']['ref_name']['include'] = @('refs/heads/autre') }
  'main exclue'                      = { param($c) $c.ruleset_main['conditions']['ref_name']['exclude'] = @('refs/heads/main') }
  'sans pull_request'                = { param($c) $c.ruleset_main['rules'] = @(@($c.ruleset_main['rules']) | Where-Object { $_['type'] -ne 'pull_request' }) }
  'sans required_status_checks'      = { param($c) $c.ruleset_main['rules'] = @(@($c.ruleset_main['rules']) | Where-Object { $_['type'] -ne 'required_status_checks' }) }
  'sans required_linear_history'     = { param($c) $c.ruleset_main['rules'] = @(@($c.ruleset_main['rules']) | Where-Object { $_['type'] -ne 'required_linear_history' }) }
  'sans non_fast_forward'            = { param($c) $c.ruleset_main['rules'] = @(@($c.ruleset_main['rules']) | Where-Object { $_['type'] -ne 'non_fast_forward' }) }
  'sans deletion'                    = { param($c) $c.ruleset_main['rules'] = @(@($c.ruleset_main['rules']) | Where-Object { $_['type'] -ne 'deletion' }) }
  'pilotage sans non_fast_forward'   = { param($c) $c.ruleset_pilotage['rules'] = @(@($c.ruleset_pilotage['rules']) | Where-Object { $_['type'] -ne 'non_fast_forward' }) }
  'règle en double'                  = { param($c) $c.ruleset_main['rules'] = @($c.ruleset_main['rules']) + @(@{ type = 'deletion' }) }
  'pull_request sans parameters'     = { param($c) (Get-Rule $c 'ruleset_main' 'pull_request').Remove('parameters') }
  'strict faux'                      = { param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['strict_required_status_checks_policy'] = $false }
  'strict en texte'                  = { param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['strict_required_status_checks_policy'] = 'true' }
  'ci-ok sans integration_id'        = { param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['required_status_checks'] = @(@{ context = 'ci-ok' }) }
  'ci-ok d''une autre application'   = { param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['required_status_checks'] = @(@{ context = 'ci-ok'; integration_id = 1 }) }
  'integration_id en texte'          = { param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['required_status_checks'] = @(@{ context = 'ci-ok'; integration_id = '15368' }) }
  'ci-ok en double, dont un libre'   = { param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['required_status_checks'] = @(@{ context = 'ci-ok'; integration_id = 15368 }, @{ context = 'ci-ok' }) }
  'autre contrôle que ci-ok'         = { param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['required_status_checks'] = @(@{ context = 'build'; integration_id = 15368 }) }
  'fusion classique'                 = { param($c) $c.repo['allow_merge_commit'] = $true }
  'rebase'                           = { param($c) $c.repo['allow_rebase_merge'] = $true }
  'fusion automatique'               = { param($c) $c.repo['allow_auto_merge'] = $true }
  'repo : clé hors liste'            = { param($c) $c.repo['visibility'] = 'public' }
  'allowed_actions all'              = { param($c) $c.actions.permissions['allowed_actions'] = 'all' }
  'sans sha_pinning_required'        = { param($c) $c.actions.permissions.Remove('sha_pinning_required') }
  'motif hors liste'                 = { param($c) $c.actions.selected_actions['patterns_allowed'] = @($c.actions.selected_actions['patterns_allowed']) + 'evil/action@*' }
  'motif élargi'                     = { param($c) $c.actions.selected_actions['patterns_allowed'] = @('actions/*') }
  'actions de GitHub toutes permises' = { param($c) $c.actions.selected_actions['github_owned_allowed'] = $true }
  'actions vérifiées permises'       = { param($c) $c.actions.selected_actions['verified_allowed'] = $true }
  'jeton en écriture'                = { param($c) $c.actions.workflow['default_workflow_permissions'] = 'write' }
  'Actions qui approuvent'           = { param($c) $c.actions.workflow['can_approve_pull_request_reviews'] = $true }
  'workflows des forks'              = { param($c) $c.actions.fork_pr_workflows_private_repos['run_workflows_from_fork_pull_requests'] = $true }
  'section en plus'                  = { param($c) $c['autre'] = @{} }
  'section actions manquante'        = { param($c) $c.actions.Remove('workflow') }
  'motif retiré, action utilisée'    = { param($c) $c.actions.selected_actions['patterns_allowed'] = @('actions/checkout@*') }
}
foreach ($k in $mutations.Keys) {
  $c = New-Cfg; & $mutations[$k] $c
  Check "S2 refuse : $k" ((Get-Invalid $c).Count -gt 0)
}

# AS-41 : chaque champ figé, un par un : valeur affaiblie, valeur en texte, champ retiré ; plus un champ
# inconnu par objet. Les valeurs sûres sont celles du script ; le fichier livré les respecte (cas ci-dessus).
function Get-Weaker($V) {
  if ($V -is [bool]) { return -not $V }
  if ($V -is [int] -or $V -is [long]) { return $V + 1 }
  if ($V -is [string]) { return "$V-x" }
  return , @('merge')   # liste ou objet : remplacé par une autre liste
}
$pins = [ordered]@{
  ruleset_main           = @({ param($c) $c.ruleset_main }, (Get-RulesetPins 'main'))
  ruleset_pilotage       = @({ param($c) $c.ruleset_pilotage }, (Get-RulesetPins 'pilotage'))
  pull_request           = @({ param($c) (Get-Rule $c 'ruleset_main' 'pull_request')['parameters'] }, $PullRequestParameters)
  required_status_checks = @({ param($c) (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters'] }, $StatusCheckParameters)
  repo                   = @({ param($c) $c.repo }, $RepoValues)
}
foreach ($k in $ActionsValues.Keys) { $pins["actions.$k"] = @({ param($c) $c.actions[$k] }.GetNewClosure(), $ActionsValues[$k]) }
foreach ($s in $pins.Keys) {
  $get, $safe = $pins[$s]
  foreach ($f in @($safe.Keys)) {
    $c = New-Cfg; (& $get $c)[$f] = Get-Weaker $safe[$f]
    Check "AS-41 refuse $s.$f affaibli" ((Get-Invalid $c).Count -gt 0)
    if ($safe[$f] -is [bool] -or $safe[$f] -is [int]) {
      $c = New-Cfg; (& $get $c)[$f] = "$($safe[$f])".ToLowerInvariant()
      Check "AS-41 refuse $s.$f en texte" ((Get-Invalid $c).Count -gt 0)
    }
    $c = New-Cfg; (& $get $c).Remove($f)
    Check "AS-41 refuse $s.$f absent" ((Get-Invalid $c).Count -gt 0)
  }
  $c = New-Cfg; (& $get $c)['champ_inconnu'] = $false
  Check "AS-41 refuse un champ inconnu dans $s" ((Get-Invalid $c).Count -gt 0)
}
# Listes figées : autres formes affaiblies ; paramètres neutres seulement avec leur valeur neutre.
foreach ($m in @(@('squash', 'merge'), @('squash', 'rebase'), @(), @('Squash'))) {
  $c = New-Cfg; (Get-Rule $c 'ruleset_main' 'pull_request')['parameters']['allowed_merge_methods'] = $m
  Check "AS-41 refuse allowed_merge_methods $(J $m)" ((Get-Invalid $c).Count -gt 0)
}
foreach ($v in @(@{ x = @(@{ context = 'ci-ok'; integration_id = 15368 }, @{ context = 'autre' }) }, @{ x = @() })) {
  $c = New-Cfg; (Get-Rule $c 'ruleset_main' 'required_status_checks')['parameters']['required_status_checks'] = $v.x
  Check "AS-41 refuse required_status_checks $(J $v.x)" ((Get-Invalid $c).Count -gt 0)
}
$c = New-Cfg; (Get-Rule $c 'ruleset_main' 'pull_request')['parameters']['automatic_copilot_code_review_enabled'] = $false
Check 'AS-41 accepte un paramètre neutre à sa valeur neutre' ((Get-Invalid $c).Count -eq 0)
$c = New-Cfg; (Get-Rule $c 'ruleset_main' 'pull_request')['parameters']['required_reviewers'] = @(@{ minimum_approvals = 0 })
Check 'AS-41 refuse un paramètre neutre sans sa valeur neutre' ((Get-Invalid $c).Count -gt 0)
$c = New-Cfg; $c.ruleset_main['rules'] = @($c.ruleset_main['rules']) + @(@{ type = 'merge_queue' })
Check 'AS-41 refuse une règle hors liste' ((Get-Invalid $c).Count -gt 0)
$c = New-Cfg; (Get-Rule $c 'ruleset_pilotage' 'deletion')['parameters'] = @{}
Check 'AS-41 refuse des paramètres sur une règle qui n''en a pas' ((Get-Invalid $c).Count -gt 0)

# Comparaison : réponse de GitHub conforme, avec les champs ignorés ; puis écarts.
$want = (New-Cfg).ruleset_main
$got = ConvertFrom-Json (ConvertTo-Json -InputObject $want -Depth 30) -AsHashtable -Depth 50
$got['id'] = 9; $got['node_id'] = 'RRS_x'; $got['source'] = 'o/r'; $got['source_type'] = 'Repository'; $got['_links'] = @{ self = @{ href = 'u' } }
$got['created_at'] = 'd'; $got['updated_at'] = 'd'; $got['current_user_can_bypass'] = 'never'
[array]::Reverse($got['rules'])
($got['rules'] | Where-Object { $_['type'] -eq 'pull_request' })['parameters']['automatic_copilot_code_review_enabled'] = $false
$d = [Collections.Generic.List[string]]::new(); Add-Diff (ConvertTo-Comparable $want) (ConvertTo-Comparable $got) 'ruleset_main' $d
Check "réponse conforme, champs ignorés ($($d -join ' ; '))" ($d.Count -eq 0)
$got['bypass_actors'] = @(@{ actor_id = 5; actor_type = 'RepositoryRole'; bypass_mode = 'always' })
$got['enforcement'] = 'disabled'; $got['rules'] = @($got['rules']) + @(@{ type = 'deletion' })
$got['extra'] = 1
$d = [Collections.Generic.List[string]]::new(); Add-Diff (ConvertTo-Comparable $want) (ConvertTo-Comparable $got) 'ruleset_main' $d
Check "écarts relevés : contournement, enforcement, règle en double, champ en plus ($($d.Count))" ($d.Count -eq 4)
$d = [Collections.Generic.List[string]]::new(); Add-Diff @{ allow_auto_merge = $false } @{ allow_auto_merge = $true; name = 'x' } 'repo' $d
Check 'repo : seuls les champs du fichier comptent' ($d.Count -eq 1)
$d = [Collections.Generic.List[string]]::new(); Add-Diff @{ patterns_allowed = @('a@*', 'b@*') } @{ patterns_allowed = @('b@*', 'a@*') } 'actions.selected_actions' $d
Check 'listes comparées sans tenir compte de l''ordre' ($d.Count -eq 0)
# Paramètre de règle absent du fichier : écart, sauf valeur neutre de la liste fermée.
$sc = { param($p) @{ rules = [ordered]@{ required_status_checks = @{ type = 'required_status_checks'; parameters = $p } } } }
foreach ($c in @(@(@{ do_not_enforce_on_create = $true }, 1), @(@{ do_not_enforce_on_create = $false }, 1), @(@{ inconnu = 0 }, 1),
                 @(@{ automatic_copilot_code_review_enabled = $false }, 0), @(@{ automatic_copilot_code_review_enabled = $true }, 1),
                 @(@{ required_reviewers = @() }, 0), @(@{ required_reviewers = @(@{ minimum_approvals = 0 }) }, 1))) {
  $d = [Collections.Generic.List[string]]::new()
  Add-Diff (& $sc @{ strict_required_status_checks_policy = $true }) (& $sc (@{ strict_required_status_checks_policy = $true } + $c[0])) 'ruleset_main' $d
  Check "paramètre en plus $(J $c[0]) : $($c[1]) écart(s), lu $($d.Count)" ($d.Count -eq $c[1])
}
# Protection classique : seul le 404 « Branch not protected » vaut « aucune ».
Check '404 Branch not protected' (Test-NotProtected @{ Status = 404; Json = @{ message = 'Branch not protected' } })
Check '404 Not Found refusé' (-not (Test-NotProtected @{ Status = 404; Json = @{ message = 'Not Found' } }))
Check '404 illisible refusé' (-not (Test-NotProtected @{ Status = 404; Json = $null }))

# S3 : motifs de ref d'un ruleset inconnu.
foreach ($c in @(@('~ALL', 'pilotage', $true), @('~DEFAULT_BRANCH', 'main', $true), @('~DEFAULT_BRANCH', 'pilotage', $false),
                 @('refs/heads/ma*', 'main', $true), @('refs/heads/**/*', 'pilotage', $true), @('refs/heads/claude/*', 'main', $false), @('refs/heads/release[', 'main', $true))) {
  Check "motif $($c[0]) sur $($c[1])" ((Test-RefPattern $c[0] $c[1] 'main') -eq $c[2])
}

# Arguments et préalables S4, dans un dépôt jetable : copie « principale » clonée d'un dépôt nu local.
# Son origin n'est jamais github.com : le dernier préalable refuse avant tout appel à GitHub, d'où
# qu'on lance ce test. Chaque refus est reconnu à son message, pas au seul code 2.
$cache = if ($env:TS_CACHE) { $env:TS_CACHE } else { 'C:\Dev\ts-cache' }
$tmp = Join-Path $cache "tmp\repo-settings-$([guid]::NewGuid().ToString('N').Substring(0, 8))"
$bare = Join-Path $tmp 'origin.git'; $M = Join-Path $tmp 'main'
function G { & git -c user.name=essai -c user.email=essai@example.invalid -c core.hooksPath=NUL @args 2>&1 | Out-Null }
G init -q --bare -b main $bare
G clone -q $bare $M
New-Item -ItemType Directory -Force (Join-Path $M 'tools/dev'), (Join-Path $M 'tools/ci'), (Join-Path $M 'sub') | Out-Null
Copy-Item (Join-Path $dev 'repo-settings.ps1') (Join-Path $M 'tools/dev/')
Copy-Item (Join-Path $root 'tools/ci/repo-settings.json') (Join-Path $M 'tools/ci/')
Set-Content (Join-Path $M 'sub/.gitkeep') ''
G -C $M add -A; G -C $M commit -q -m 'docs(essai): base'; G -C $M push -q origin main
$S = Join-Path $M 'tools/dev/repo-settings.ps1'
function Run-Settings([string]$Dir, [string]$Script, [string[]]$A) {
  $out = & pwsh -NoProfile -NonInteractive -WorkingDirectory $Dir -File $Script @A 2>&1 | Out-String
  return @{ Code = $LASTEXITCODE; Out = $out }
}
function Expect([string]$Name, $R, [string]$Pattern) { Check "$Name (code $($R.Code) : $($R.Out.Trim()))" ($R.Code -eq 2 -and $R.Out -match $Pattern) }

foreach ($a in @(@(), @('-Check', '-Apply'), @('-check'), @('-Apply', 'x'), @('-Check:$true'), @('Check'), @('-Apply:$false'))) {
  Expect "arguments refusés : $($a -join ' ')" (Run-Settings $M $S $a) 'usage :'
}
# Cas positif : -Check et -Apply passent les arguments et tous les préalables, jusqu'au dernier (distant).
foreach ($mode in '-Check', '-Apply') { Expect "$mode atteint le dernier préalable" (Run-Settings $M $S @($mode)) 'distant origin hors de github\.com' }
Expect 'sous-dossier de la copie principale' (Run-Settings (Join-Path $M 'sub') $S @('-Check')) 'pas depuis la racine de la copie principale'
$W = Join-Path $tmp 'wt'; G -C $M worktree add -q -b essai/wt $W
Expect 'worktree de la copie principale' (Run-Settings $W (Join-Path $W 'tools/dev/repo-settings.ps1') @('-Apply')) 'pas depuis la racine de la copie principale'
$out = Join-Path $tmp 'ailleurs'; New-Item -ItemType Directory -Force $out | Out-Null; Copy-Item $S $out
Expect 'script hors de la copie principale' (Run-Settings $M (Join-Path $out 'repo-settings.ps1') @('-Apply')) 'script lancé hors de la copie principale'
G -C $M switch -q -c autre
Expect 'copie principale hors de main' (Run-Settings $M $S @('-Check')) 'n''est pas sur main'
G -C $M switch -q main
Set-Content (Join-Path $M 'non-suivi.txt') 'x'
Expect 'copie principale non propre' (Run-Settings $M $S @('-Apply')) 'n''est pas propre'
Remove-Item (Join-Path $M 'non-suivi.txt')
G -C $M commit -q --allow-empty -m 'docs(essai): en avance'
Expect 'copie principale en avance d''origin/main' (Run-Settings $M $S @('-Check')) 'n''est pas à jour d''origin/main'
G -C $M push -q origin main; G -C $M reset -q --hard HEAD~1
Expect 'copie principale en retard d''origin/main' (Run-Settings $M $S @('-Apply')) 'n''est pas à jour d''origin/main'
G -C $M pull -q --ff-only
# AS-38 : -Check -SourceCommit <sha>, forme réservée à cargo xtask merge ; les préalables S4 restent.
$head = "$(& git -C $M rev-parse HEAD)".Trim()
foreach ($a in @(@('-Check', '-SourceCommit'), @('-Apply', '-SourceCommit', $head), @('-SourceCommit', $head, '-Check'),
                 @('-Check', '-SourceCommit', $head, 'x'), @('-Check', '-sourcecommit', $head), @('-Check', '-Source', $head))) {
  Expect "arguments refusés : $($a -join ' ')" (Run-Settings $M $S $a) 'usage :'
}
foreach ($x in $head.Substring(0, 39), $head.ToUpperInvariant(), "${head}0", ('g' * 40), 'HEAD', 'origin/main') {
  Expect "-SourceCommit refusé : $x" (Run-Settings $M $S @('-Check', '-SourceCommit', $x)) '-SourceCommit : SHA complet'
}
Expect '-SourceCommit : commit absent' (Run-Settings $M $S @('-Check', '-SourceCommit', ('0' * 40))) 'introuvable après git fetch'
Expect '-SourceCommit atteint le dernier préalable' (Run-Settings $M $S @('-Check', '-SourceCommit', $head)) 'distant origin hors de github\.com'
Expect '-SourceCommit hors de la racine' (Run-Settings (Join-Path $M 'sub') $S @('-Check', '-SourceCommit', $head)) 'pas depuis la racine de la copie principale'
Expect '-SourceCommit depuis un worktree' (Run-Settings $W (Join-Path $W 'tools/dev/repo-settings.ps1') @('-Check', '-SourceCommit', $head)) 'pas depuis la racine de la copie principale'
# Le fichier est lu dans le commit source (la tête d'une PR), ni sur le disque ni sur origin/main.
G -C $M switch -q -c marque
Set-Content (Join-Path $M 'tools/ci/repo-settings.json') '{"marque": 1}'
G -C $M commit -q -am 'docs(essai): marque'
$mark = "$(& git -C $M rev-parse HEAD)".Trim()
G -C $M rm -q tools/ci/repo-settings.json; G -C $M commit -q -m 'docs(essai): sans fichier dans la branche'
$nofile = "$(& git -C $M rev-parse HEAD)".Trim()
G -C $M switch -q main
Set-Content (Join-Path $M 'tools/ci/repo-settings.json') '{"disque": 1}'
Push-Location $M
try { $r = Read-Config $mark; $o = Read-Config 'origin/main' } finally { Pop-Location }
Check 'Read-Config lit le commit source, pas le disque' ($r -is [Collections.IDictionary] -and $r['marque'] -eq 1 -and -not $r.Contains('disque'))
Check 'Read-Config d''origin/main' ($o -is [Collections.IDictionary] -and $o.Contains('ruleset_main') -and -not $o.Contains('marque'))
G -C $M checkout -q -- tools/ci/repo-settings.json
$r = Run-Settings $M $S @('-Check', '-SourceCommit', $mark)
Check "-SourceCommit : fichier du commit lu et refusé par S2 avant GitHub (code $($r.Code) : $($r.Out.Trim()))" ($r.Code -eq 1 -and $r.Out -match 'sections attendues' -and $r.Out -notmatch 'distant')
# AS-41 : un fichier affaibli dans la tête est refusé par S2, avec les invariants du script de la copie
# principale (égal à origin/main par S4), même si la tête modifie aussi le script (refusé par ailleurs par
# la condition 1 de merge).
G -C $M switch -q -c affaibli main
$weak = ConvertFrom-Json $text -AsHashtable -Depth 50; $weak.repo['allow_auto_merge'] = $true
Set-Content (Join-Path $M 'tools/ci/repo-settings.json') (ConvertTo-Json $weak -Depth 30)
G -C $M commit -q -am 'docs(essai): fichier affaibli'
$weakSha = "$(& git -C $M rev-parse HEAD)".Trim()
$lax = (Get-Content -Raw $S).Replace('function Get-Invalid($C) {', 'function Get-Invalid($C) { return , [Collections.Generic.List[string]]::new()')
Set-Content (Join-Path $M 'tools/dev/repo-settings.ps1') $lax -NoNewline
G -C $M commit -q -am 'docs(essai): script sans invariants'
$laxSha = "$(& git -C $M rev-parse HEAD)".Trim()
G -C $M switch -q main
foreach ($x in @(@('fichier affaibli', $weakSha), @('fichier affaibli et script de la tête sans invariants', $laxSha))) {
  $r = Run-Settings $M $S @('-Check', '-SourceCommit', $x[1])
  Check "AS-41 -SourceCommit : $($x[0]) refusé (code $($r.Code) : $($r.Out.Trim()))" ($r.Code -eq 1 -and $r.Out -match 'repo\.allow_auto_merge doit valoir false')
}
Check 'le script de la tête n''est pas celui de la copie principale' ((Get-Content -Raw $S) -cne $lax)
Expect '-SourceCommit d''un commit sans le fichier' (Run-Settings $M $S @('-Check', '-SourceCommit', $nofile)) "absent du commit $nofile"
G -C $M rm -q tools/ci/repo-settings.json; G -C $M commit -q -m 'docs(essai): sans fichier'; G -C $M push -q origin main
Expect 'fichier absent d''origin/main' (Run-Settings $M $S @('-Check')) 'absent d''origin/main'
Remove-Item -LiteralPath $tmp -Recurse -Force

"test-repo-settings : $($n - $fail)/$n cas conformes."
exit [int]($fail -gt 0)
