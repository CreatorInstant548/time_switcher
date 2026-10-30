# tools/dev/test-repo-settings.ps1 : tests de repo-settings.ps1, sans appel à GitHub.
# Invariants du fichier (S2), comparaison des réponses, motifs de ref (S3), arguments et préalables.
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

# S3 : motifs de ref d'un ruleset inconnu.
foreach ($c in @(@('~ALL', 'pilotage', $true), @('~DEFAULT_BRANCH', 'main', $true), @('~DEFAULT_BRANCH', 'pilotage', $false),
                 @('refs/heads/ma*', 'main', $true), @('refs/heads/**/*', 'pilotage', $true), @('refs/heads/claude/*', 'main', $false), @('refs/heads/release[', 'main', $true))) {
  Check "motif $($c[0]) sur $($c[1])" ((Test-RefPattern $c[0] $c[1] 'main') -eq $c[2])
}

# Arguments et préalables : code 2, sans appel à GitHub (lancé hors de la copie principale, ou fichier absent d'origin/main).
$script = Join-Path $dev 'repo-settings.ps1'
foreach ($a in @(@(), @('-Check', '-Apply'), @('-check'), @('-Apply', 'x'), @('-Check:$true'), @('Check'))) {
  & pwsh -NoProfile -NonInteractive -File $script @a 2>$null | Out-Null
  Check "arguments refusés : $($a -join ' ')" ($LASTEXITCODE -eq 2)
}
& pwsh -NoProfile -NonInteractive -File $script -Check 2>$null | Out-Null
Check 'hors de la copie principale ou avant la fusion : code 2' ($LASTEXITCODE -eq 2)

"test-repo-settings : $($n - $fail)/$n cas conformes."
exit [int]($fail -gt 0)
