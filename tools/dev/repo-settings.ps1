# tools/dev/repo-settings.ps1 : réglages GitHub du dépôt, décrits dans tools/ci/repo-settings.json (T3d).
# Mode d'emploi : docs/dev/outillage.md §6. Deux modes exclusifs, sans autre argument :
#   -Check : lecture seule (GET) ; tout écart avec le fichier d'origin/main est affiché (code 1) ;
#   -Apply : crée ou remplace les rulesets, règle le dépôt et les Actions selon ce fichier, puis -Check.
# Les valeurs sont lues sur origin/main (git show), jamais sur le disque : une branche ne peut pas faire
# appliquer les siennes. Lancé depuis la racine de la copie principale, propre, sur main, à jour
# d'origin/main, et le script lancé doit être celui de cette copie (S4). Les invariants sont codés ici
# (S2) : affaiblir les règles demande de modifier aussi ce script.
# Codes de sortie : 0 conforme ; 1 écart, fichier refusé ou écriture refusée ; 2 usage, préalable ou erreur.
# Conditions suivies : AS-30 R1 à R5, AS-31 E4 et E5 (et l'attestation qui remplace R6.4), §9.12 S1 à S5.
# spec: carnet architecture-securite.md SEC0-30 ; §9.12 S1, S2, S3, S4, S5
$ErrorActionPreference = 'Stop'
trap { [Console]::Error.WriteLine("repo-settings : erreur interne ($($_.Exception.Message))"); exit 2 }

$File = 'tools/ci/repo-settings.json'
$ActionsAppId = 15368   # application GitHub Actions : app.id des contrôles ci-ok, relevé le 30/10 (R1)
$Patterns = 'actions/checkout@*', 'actions/cache@*', 'actions/cache/restore@*', 'actions/cache/save@*',
            'Swatinem/rust-cache@*', 'actions/upload-artifact@*'   # liste fermée de SEC0-30
$ActionsApi = [ordered]@{ permissions = '/actions/permissions'; selected_actions = '/actions/permissions/selected-actions'
  workflow = '/actions/permissions/workflow'; fork_pr_workflows_private_repos = '/actions/permissions/fork-pr-workflows-private-repos' }
# Champs des réponses ignorés à la comparaison : identifiants, liens, dates, champs calculés ; tout le
# reste de l'objet du dépôt (seuls les champs du fichier y sont comparés).
$Ignored = '^ruleset_\w+\.(id|node_id|source|source_type|_links|created_at|updated_at|current_user_can_bypass)$',
           '^repo\.', '^actions\.permissions\.selected_actions_url$'
# Paramètre de règle absent du fichier : un écart, sauf ceux de cette liste fermée, et seulement avec
# leur valeur neutre (aucun effet sur la règle). GitHub peut les renvoyer sans qu'on les ait envoyés ;
# les accepter ici évite qu'un -Apply conforme laisse un écart qui suspendrait toutes les fusions.
# Toute autre valeur, ou tout autre paramètre (do_not_enforce_on_create: true…), est un écart.
$NeutralParameters = @{
  automatic_copilot_code_review_enabled = 'false'   # pull_request : relecture automatique par Copilot, désactivée
  required_reviewers                    = '[]'      # pull_request : relecteurs obligatoires par motif, aucun
}

function Stop-With([int]$Code, [string]$Message) { [Console]::Error.WriteLine("repo-settings : $Message"); exit $Code }
function Invoke-Git {
  $o = & git @args 2>$null
  if ($LASTEXITCODE) { Stop-With 2 "échec de git $($args -join ' ')" }
  return $o
}
function Test-True($V) { return $V -is [bool] -and $V }
function Test-False($V) { return $V -is [bool] -and -not $V }
function Test-SamePath([string]$A, [string]$B) {
  $n = { param($p) [IO.Path]::GetFullPath($p).TrimEnd('\', '/') }
  return (& $n $A).Equals((& $n $B), [StringComparison]::OrdinalIgnoreCase)
}

# JSON canonique : clés triées, éléments des listes triés.
function ConvertTo-Sorted($V) {
  if ($V -is [Collections.IDictionary]) {
    $o = [ordered]@{}; foreach ($k in @($V.Keys | Sort-Object)) { $o[$k] = ConvertTo-Sorted $V[$k] }; return $o
  }
  if ($V -is [Collections.IList]) {
    return , @(@($V | ForEach-Object { ConvertTo-Sorted $_ }) | Sort-Object { ConvertTo-Json -InputObject $_ -Compress -Depth 30 })
  }
  return $V
}
function J($V) { return ConvertTo-Json -InputObject (ConvertTo-Sorted $V) -Compress -Depth 30 }

# Écarts entre la valeur attendue et la valeur lue ; objets comparés clé par clé.
function Add-Diff($Want, $Got, [string]$Path, $Out) {
  if ($Want -isnot [Collections.IDictionary] -or $Got -isnot [Collections.IDictionary]) {
    if ((J $Want) -cne (J $Got)) { $Out.Add("$Path : attendu $(J $Want), lu $(J $Got)") }
    return
  }
  foreach ($k in $Want.Keys) {
    if ($Got.Contains($k)) { Add-Diff $Want[$k] $Got[$k] "$Path.$k" $Out } else { $Out.Add("$Path.$k : absent de GitHub, attendu $(J $Want[$k])") }
  }
  foreach ($k in $Got.Keys) {
    $p = "$Path.$k"
    if ($Want.Contains($k) -or ($Ignored | Where-Object { $p -match $_ })) { continue }
    if ($p -match '^ruleset_\w+\.rules\.\w+\.parameters\.\w+$' -and $k -cin @($NeutralParameters.Keys) -and (J $Got[$k]) -ceq $NeutralParameters[$k]) {
      Write-Host "  note : $p = $(J $Got[$k]) sur GitHub, valeur neutre absente du fichier"; continue
    }
    $Out.Add("$p : présent sur GitHub ($(J $Got[$k])), absent du fichier")
  }
}

# Attestation de GitHub, qui remplace l'essai de poussée R6.4 (AS-31, T3e-2) : le compte des sessions
# ne peut jamais contourner le ruleset main, et GitHub applique bien à main les cinq règles effectives.
$EffectiveMain = 'deletion', 'non_fast_forward', 'required_linear_history', 'pull_request', 'required_status_checks'
function Add-Attestation($Ruleset, $Effective, $Out) {
  $bypass = if ($Ruleset -is [Collections.IDictionary]) { $Ruleset['current_user_can_bypass'] } else { $null }
  if ($bypass -isnot [string] -or $bypass -cne 'never') { $Out.Add("ruleset_main.current_user_can_bypass : attendu ""never"", lu $(J $bypass) (AS-31, R6.4)") }
  $types = @(@($Effective) | Where-Object { $_ -is [Collections.IDictionary] } | ForEach-Object { "$($_['type'])" })
  foreach ($t in $EffectiveMain) { if ($t -cnotin $types) { $Out.Add("rules/branches/main : règle effective $t absente (AS-31, R6.4)") } }
}

# Ruleset comparable : règles indexées par type (une règle en double ressort comme écart).
function ConvertTo-Comparable($R) {
  $o = [ordered]@{}; foreach ($k in $R.Keys) { $o[$k] = $R[$k] }
  $m = [ordered]@{}
  foreach ($rule in @($R['rules'])) { $t = "$($rule['type'])"; while ($m.Contains($t)) { $t += '_double' }; $m[$t] = $rule }
  $o['rules'] = $m
  return $o
}

# gh api ; @{ Status = 200 si réussi, sinon le statut HTTP de la réponse (0 s'il est illisible) ; Json }.
function Invoke-Api([string]$Path, [string]$Method = 'GET', $Body = $null) {
  $a = @('api', '-H', 'Accept: application/vnd.github+json', '-H', 'X-GitHub-Api-Version: 2022-11-28')
  if ($Method -ne 'GET') { $a += @('-X', $Method, '--input', '-') }
  $a += "repos/$Slug$Path"
  $out = if ($Method -eq 'GET') { & gh @a 2>$null } else { ConvertTo-Json -InputObject $Body -Depth 30 | & gh @a 2>$null }
  $code = $LASTEXITCODE
  $text = "$($out -join "`n")".Trim()
  $json = $null; if ($text) { try { $json = ConvertFrom-Json $text -AsHashtable -Depth 50 -NoEnumerate } catch { $json = $null } }
  $status = if ($code -eq 0) { 200 } elseif ($json -is [Collections.IDictionary] -and "$($json['status'])" -match '^\d+$') { [int]$json['status'] } else { 0 }
  return @{ Status = $status; Json = $json }
}
function Get-Api([string]$Path, [int[]]$Allow = @()) {
  $r = Invoke-Api $Path
  if ($r.Status -ne 200 -and $r.Status -notin $Allow) { Stop-With 2 "GET repos/$Slug$Path : échec (HTTP $($r.Status))" }
  return $r
}

# Réponse 404 de branches/<b>/protection qui dit bien « aucune protection classique ».
function Test-NotProtected($R) {
  return $R.Status -eq 404 -and $R.Json -is [Collections.IDictionary] -and $R.Json['message'] -ceq 'Branch not protected'
}

# Le motif de ref d'un ruleset peut-il viser cette branche ? (prudent : * couvre aussi /)
function Test-RefPattern([string]$Pattern, [string]$Branch, [string]$Default) {
  if ($Pattern -eq '~ALL') { return $true }
  if ($Pattern -eq '~DEFAULT_BRANCH') { return $Branch -eq $Default }
  try { return "refs/heads/$Branch" -like ($Pattern -replace '\*\*/?', '*') } catch { return $true }   # motif illisible
}

# S2 : invariants du fichier ; motifs de refus (liste vide si le fichier est valide).
function Get-Invalid($C) {
  $bad = [Collections.Generic.List[string]]::new()
  if ($C -isnot [Collections.IDictionary] -or (J @($C.Keys)) -cne (J @('actions', 'repo', 'ruleset_main', 'ruleset_pilotage'))) {
    $bad.Add('sections attendues : ruleset_main, ruleset_pilotage, repo, actions'); return , $bad
  }
  $specs = @(@('ruleset_main', 'main', @('pull_request', 'required_status_checks', 'required_linear_history', 'non_fast_forward', 'deletion')),
             @('ruleset_pilotage', 'pilotage', @('non_fast_forward', 'deletion')))
  foreach ($s in $specs) {
    $key, $branch, $needed = $s; $r = $C[$key]
    if ($r -isnot [Collections.IDictionary]) { $bad.Add("$key : objet attendu"); continue }
    if (-not "$($r['name'])" -or $r['target'] -cne 'branch' -or $r['enforcement'] -cne 'active') { $bad.Add("$key : name, target branch et enforcement active exigés") }
    if ($r['bypass_actors'] -isnot [Collections.IList] -or @($r['bypass_actors']).Count -ne 0) { $bad.Add("$key : bypass_actors doit être une liste vide") }
    if ((J $r['conditions']) -cne (J @{ ref_name = @{ include = @("refs/heads/$branch"); exclude = @() } })) { $bad.Add("$key : conditions.ref_name doit viser refs/heads/$branch seulement") }
    $types = @(@($r['rules']) | ForEach-Object { "$($_['type'])" })
    foreach ($t in $needed) { if ($t -cnotin $types) { $bad.Add("$key : règle $t absente") } }
    if (@($types | Select-Object -Unique).Count -ne $types.Count) { $bad.Add("$key : règle en double") }
  }
  if ($bad.Count) { return , $bad }
  if ($C.ruleset_main['name'] -ceq $C.ruleset_pilotage['name']) { $bad.Add('les deux rulesets portent le même nom') }
  $rules = @($C.ruleset_main['rules'])
  $pr = ($rules | Where-Object { $_['type'] -ceq 'pull_request' })['parameters']
  if ($pr -isnot [Collections.IDictionary]) { $bad.Add('ruleset_main : pull_request sans parameters') }
  $sc = ($rules | Where-Object { $_['type'] -ceq 'required_status_checks' })['parameters']
  $checks = if ($sc -is [Collections.IDictionary]) { @($sc['required_status_checks']) } else { @() }
  $ci = @($checks | Where-Object { $_ -is [Collections.IDictionary] -and $_['context'] -ceq 'ci-ok' })
  if (-not ($sc -is [Collections.IDictionary] -and (Test-True $sc['strict_required_status_checks_policy']))) { $bad.Add('ruleset_main : strict_required_status_checks_policy doit valoir true') }
  if ($ci.Count -eq 0 -or ($ci | Where-Object { -not (($_['integration_id'] -is [int] -or $_['integration_id'] -is [long]) -and $_['integration_id'] -eq $ActionsAppId) })) {
    $bad.Add("ruleset_main : ci-ok requis, lié à GitHub Actions (integration_id $ActionsAppId, R1)")
  }
  $repo = $C.repo
  # Clés fermées : un PATCH du dépôt ne peut rien changer d'autre (visibilité, branche par défaut…).
  $repoKeys = 'allow_auto_merge', 'allow_merge_commit', 'allow_rebase_merge', 'allow_squash_merge', 'delete_branch_on_merge', 'squash_merge_commit_title'
  if ($repo -isnot [Collections.IDictionary] -or (J @($repo.Keys)) -cne (J @($repoKeys))) { $bad.Add("repo : clés attendues $($repoKeys -join ', ')"); return , $bad }
  if (-not ($repo -is [Collections.IDictionary] -and (Test-True $repo['allow_squash_merge']) -and (Test-False $repo['allow_merge_commit']) -and
            (Test-False $repo['allow_rebase_merge']) -and (Test-False $repo['allow_auto_merge']))) {
    $bad.Add('repo : squash seulement, sans fusion automatique (R2)')
  }
  $act = $C.actions
  if ($act -isnot [Collections.IDictionary] -or (J @($act.Keys)) -cne (J @($ActionsApi.Keys)) -or
      @($act.Values | Where-Object { $_ -isnot [Collections.IDictionary] }).Count) {
    $bad.Add("actions : objets attendus $(@($ActionsApi.Keys) -join ', ')"); return , $bad
  }
  $p = $act.permissions; $sel = $act.selected_actions; $wf = $act.workflow; $fork = $act.fork_pr_workflows_private_repos
  if ($p['allowed_actions'] -cne 'selected' -or -not (Test-True $p['sha_pinning_required'])) { $bad.Add('actions.permissions : allowed_actions selected et sha_pinning_required true exigés (R4)') }
  if (-not (Test-False $sel['github_owned_allowed']) -or -not (Test-False $sel['verified_allowed'])) { $bad.Add('actions.selected_actions : github_owned_allowed et verified_allowed doivent valoir false') }
  $pats = @($sel['patterns_allowed'])
  foreach ($x in $pats) { if ("$x" -cnotin $Patterns) { $bad.Add("actions.selected_actions : motif « $x » hors de la liste fermée de SEC0-30") } }
  if ($wf['default_workflow_permissions'] -cne 'read' -or -not (Test-False $wf['can_approve_pull_request_reviews'])) { $bad.Add('actions.workflow : jeton en lecture et aucune approbation par les Actions (R4)') }
  if (-not (Test-False $fork['run_workflows_from_fork_pull_requests'])) { $bad.Add('actions.fork_pr_workflows_private_repos : aucun workflow pour les PR d''un fork (S5)') }
  # Chaque action des workflows d'origin/main doit rester permise, sinon -Apply casserait la CI.
  foreach ($f in @(Invoke-Git ls-tree --name-only origin/main .github/workflows/) | Where-Object { $_ -match '\.ya?ml$' }) {
    foreach ($m in [regex]::Matches((@(Invoke-Git show "origin/main:$f") -join "`n"), '(?m)^[\s-]*uses:\s*[''"]?([^\s''"#]+)')) {
      $u = $m.Groups[1].Value
      if (-not $u.StartsWith('./') -and -not ($pats | Where-Object { $u -like $_ })) { $bad.Add("$f : $u hors des motifs permis") }
    }
  }
  return , $bad
}

# Écarts entre GitHub et le fichier (S3 compris).
function Get-Drift($C) {
  $out = [Collections.Generic.List[string]]::new()
  $repo = (Get-Api '').Json
  Add-Diff $C.repo $repo 'repo' $out
  $list = @((Get-Api '/rulesets?includes_parents=true&per_page=100').Json)
  if ($list.Count -ge 100) { Stop-With 2 'plus de 100 rulesets : pagination non gérée' }
  $ours = $C.ruleset_main['name'], $C.ruleset_pilotage['name']
  foreach ($key in 'ruleset_main', 'ruleset_pilotage') {
    $want = $C[$key]; $hits = @($list | Where-Object { $_['name'] -ceq $want['name'] })
    if ($hits.Count -ne 1) { $out.Add("$key : $($hits.Count) ruleset(s) nommé(s) « $($want['name']) » sur GitHub, attendu 1"); continue }
    $full = (Get-Api "/rulesets/$($hits[0]['id'])").Json
    Add-Diff (ConvertTo-Comparable $want) (ConvertTo-Comparable $full) $key $out
    if ($key -ceq 'ruleset_main') { Add-Attestation $full (Get-Api '/rules/branches/main?per_page=100').Json $out }
  }
  foreach ($r in $list | Where-Object { $_['name'] -cnotin $ours }) {
    $full = (Get-Api "/rulesets/$($r['id'])").Json
    if ($full['target'] -ne 'branch') { continue }
    $inc = @(); try { $inc = @($full['conditions']['ref_name']['include']) } catch { $inc = @('~ALL') }
    if (-not $inc) { $inc = @('~ALL') }   # illisible : tenu pour visant tout
    foreach ($b in 'main', 'pilotage') {
      if ($inc | Where-Object { Test-RefPattern "$_" $b $repo['default_branch'] }) { $out.Add("ruleset inconnu « $($r['name']) » (id $($r['id'])) qui vise $b (S3)") }
    }
  }
  # protected=true couvre aussi les branches visées par un ruleset : la protection classique se lit
  # branche par branche. Seul le 404 « Branch not protected » vaut « aucune » ; tout autre 404 est une erreur.
  $branches = @((Get-Api '/branches?protected=true&per_page=100').Json)
  if ($branches.Count -ge 100) { Stop-With 2 'plus de 100 branches protégées : pagination non gérée' }
  foreach ($b in $branches) {
    $path = "/branches/$([uri]::EscapeDataString($b['name']))/protection"
    $r = Get-Api $path -Allow 404
    if ($r.Status -eq 200) { $out.Add("protection de branche classique sur $($b['name']) (S3)") }
    elseif (-not (Test-NotProtected $r)) { Stop-With 2 "GET repos/$Slug$path : 404 inattendu ($(J $r.Json))" }
  }
  foreach ($k in $ActionsApi.Keys) {
    $r = Get-Api $ActionsApi[$k] -Allow 404, 409
    if ($r.Status -ne 200) { $out.Add("actions.$k : illisible ou sans objet sur GitHub (HTTP $($r.Status))"); continue }
    Add-Diff $C.actions[$k] $r.Json "actions.$k" $out
  }
  return , $out
}

function Invoke-Write([string]$Method, [string]$Path, $Body) {
  $r = Invoke-Api $Path $Method $Body
  if ($r.Status -ne 200) { Stop-With 1 "$Method repos/$Slug$Path refusé (HTTP $($r.Status)) : $(J $r.Json)" }
  Write-Host "  $Method repos/$Slug$Path : fait"
}

if ($MyInvocation.InvocationName -eq '.') { return }   # chargé par tools/dev/test-repo-settings.ps1

# --- Arguments et préalables ---
if ($args.Count -ne 1 -or $args[0] -cnotin '-Check', '-Apply') { Stop-With 2 'usage : pwsh -NoProfile -File tools/dev/repo-settings.ps1 -Check|-Apply' }
$Apply = $args[0] -ceq '-Apply'
# S4 d'abord, puis le fichier d'origin/main, puis le distant : chaque refus a son message (testés par
# tools/dev/test-repo-settings.ps1 dans un dépôt jetable dont origin est local, sans appel à GitHub).
$common = "$(Invoke-Git rev-parse --path-format=absolute --git-common-dir)".Trim()
$main = Split-Path ([IO.Path]::GetFullPath($common)) -Parent
$top = "$(Invoke-Git rev-parse --show-toplevel)".Trim()
if (-not (Test-SamePath (Get-Location).Path $main) -or -not (Test-SamePath $top $main)) { Stop-With 2 "lancé depuis $((Get-Location).Path), pas depuis la racine de la copie principale ($main)" }
if (-not (Test-SamePath $PSScriptRoot (Join-Path $main 'tools/dev'))) { Stop-With 2 "script lancé hors de la copie principale ($PSScriptRoot)" }
if ("$(Invoke-Git branch --show-current)".Trim() -cne 'main') { Stop-With 2 'la copie principale n''est pas sur main' }
if (@(Invoke-Git status --porcelain).Count) { Stop-With 2 'la copie principale n''est pas propre' }
Invoke-Git fetch --quiet origin | Out-Null
if ("$(Invoke-Git rev-parse HEAD)".Trim() -cne "$(Invoke-Git rev-parse origin/main)".Trim()) { Stop-With 2 'la copie principale n''est pas à jour d''origin/main : git pull --ff-only' }
& git cat-file -e "origin/main:$File" 2>$null
if ($LASTEXITCODE) { Stop-With 2 "$File absent d'origin/main : réglages pas encore fusionnés, rien à comparer ni à appliquer" }
$url = "$(Invoke-Git config --get remote.origin.url)".Trim()
if ($url -notmatch '^(https://github\.com/|git@github\.com:)([\w.-]+/[\w.-]+?)(\.git)?$') { Stop-With 2 "distant origin hors de github.com : $url" }
$Slug = $Matches[2]

$cfg = $null
try { $cfg = ConvertFrom-Json (@(Invoke-Git show "origin/main:$File") -join "`n") -AsHashtable -Depth 50 } catch { $cfg = $null }
$invalid = if ($null -eq $cfg) { [Collections.Generic.List[string]]@("$File : JSON illisible") } else { Get-Invalid $cfg }
foreach ($x in $invalid) { Write-Host "  REFUS du fichier : $x" }

if ($Apply) {
  if ($invalid.Count) { Stop-With 1 "$File d'origin/main refusé (S2) : rien n'est appliqué" }
  Write-Host "repo-settings -Apply ($Slug, $File d'origin/main)"
  $list = @((Get-Api '/rulesets?includes_parents=false&per_page=100').Json)
  foreach ($key in 'ruleset_main', 'ruleset_pilotage') {
    $want = $cfg[$key]; $hits = @($list | Where-Object { $_['name'] -ceq $want['name'] })
    if ($hits.Count -gt 1) { Stop-With 1 "$($hits.Count) rulesets nommés « $($want['name']) » : à dédoublonner avant -Apply" }
    if ($hits.Count -eq 1) { Invoke-Write 'PUT' "/rulesets/$($hits[0]['id'])" $want } else { Invoke-Write 'POST' '/rulesets' $want }
  }
  Invoke-Write 'PATCH' '' $cfg.repo
  foreach ($k in $ActionsApi.Keys) { Invoke-Write 'PUT' $ActionsApi[$k] $cfg.actions[$k] }
}

# Fichier refusé : c'est déjà un écart, et GitHub n'est pas comparé à des valeurs refusées.
$drift = if ($invalid.Count) { @($invalid) } else { @(Get-Drift $cfg) }
foreach ($d in $drift) { Write-Host "  ÉCART : $d" }
$mode = if ($Apply) { '-Apply puis -Check' } else { '-Check' }
if (@($drift).Count) { Write-Host "repo-settings $mode : $(@($drift).Count) écart(s) avec $File d'origin/main"; exit 1 }
Write-Host "repo-settings $mode : conforme à $File d'origin/main ($Slug)"
exit 0
