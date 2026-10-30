# tools/dev/guard-main.ps1 : crochet PreToolUse (Edit|Write|NotebookEdit) de Claude Code.
# - SEC0-35 : refuse .claude/settings(.local).json de toute copie de travail, sauf sur une branche
#   claude/outillage-*, et les réglages utilisateur (~/.claude) toujours. Erreur ou doute : refus.
# - A19 : refuse l'écriture dans la copie principale hors de .claude/worktrees/. Garde-fou de commodité :
#   une erreur interne laisse passer (les barrières sont les crochets Git épinglés, la CI et la fusion).
# spec: revues/developpement-agents.md §4.4 A19, §6.2 ; carnet architecture-securite.md SEC0-35
. (Join-Path $PSScriptRoot 'guard-lib.ps1')

# Racine de la copie principale : parent du dossier Git commun ($null si introuvable).
function Get-MainRoot([string]$From) {
  $common = & git -C $From rev-parse --path-format=absolute --git-common-dir 2>$null
  if ($LASTEXITCODE -ne 0 -or -not $common) { return $null }
  return Split-Path (ConvertTo-FullPath "$common".Trim()) -Parent
}

# Motif du refus pour l'écriture de $Raw, ou $null si elle est permise.
function Get-EditVerdict([string]$Raw, [string]$MainRoot, [string]$Cwd) {
  try { $file = ConvertTo-CanonicalPath $Raw $Cwd } catch { return "chemin refusé par prudence : $($_.Exception.Message)" }
  if (-not $file) { return 'chemin absent ou illisible : refus par prudence' }
  try {
    if (Test-SettingsFile $file) {
      if (Test-UnderPath $file (Join-Path $HOME '.claude')) {
        return "Réglages utilisateur de Claude Code protégés (SEC0-35) : $file ne se modifie par aucune session."
      }
      if (-not (Test-ToolingBranch (Get-Branch (Split-Path $file -Parent)))) {
        return "Réglages de Claude Code protégés (SEC0-35) : $file ne se modifie que sur une branche claude/outillage-*, par une PR de chemin sensible."
      }
    }
  } catch { return "SEC0-35 : erreur d'analyse ($($_.Exception.Message)) ; refus par prudence" }
  try {
    if ($MainRoot -and (Test-UnderPath $file $MainRoot) -and
        -not (Test-UnderPath $file (Join-Path $MainRoot '.claude\worktrees'))) {
      return "Copie principale en lecture seule : travaillez dans un worktree (EnterWorktree). Fichier refusé : $file"
    }
  } catch { [Console]::Error.WriteLine("guard-main : erreur interne ($($_.Exception.Message)) ; écriture laissée passer.") }
  return $null
}

if ($MyInvocation.InvocationName -eq '.') { return }   # chargé par les tests

try {
  $in = Read-HookInput
  $ti = $in.tool_input
  $raw = if ($ti.file_path) { "$($ti.file_path)" } else { "$($ti.notebook_path)" }
} catch { Exit-Deny "guard-main : entrée illisible ($($_.Exception.Message)) ; refus par prudence (SEC0-35)." }
$main = try { Get-MainRoot $PSScriptRoot } catch { $null }
$verdict = Get-EditVerdict $raw $main "$($in.cwd)"
if ($verdict) { Exit-Deny $verdict }
exit 0
