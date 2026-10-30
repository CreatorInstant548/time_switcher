# tools/dev/guard-main.ps1 : crochet PreToolUse (Edit|Write|NotebookEdit) de Claude Code.
# Refuse (code 2) l'écriture dans la copie principale hors de .claude/worktrees/ (A19), et sur
# .claude/settings(.local).json de toute copie de travail, sauf sur une branche claude/outillage-* (SEC0-35).
# Garde-fou de commodité : les barrières sont les crochets Git épinglés, la CI et xtask merge.
# spec: revues/developpement-agents.md §4.4 A19, §6.2 ; carnet architecture-securite.md SEC0-35
. (Join-Path $PSScriptRoot 'guard-lib.ps1')

# Racine de la copie principale : parent du dossier Git commun ($null si introuvable).
function Get-MainRoot([string]$From) {
  $common = & git -C $From rev-parse --path-format=absolute --git-common-dir 2>$null
  if ($LASTEXITCODE -ne 0 -or -not $common) { return $null }
  return Split-Path (ConvertTo-FullPath "$common".Trim()) -Parent
}

# Motif du refus, ou $null si l'écriture est permise.
function Get-EditVerdict([string]$File, [string]$MainRoot) {
  if (-not $File) { return $null }
  if (Test-SettingsFile $File) {
    $branch = Get-Branch (Split-Path $File -Parent)
    if (-not (Test-ToolingBranch $branch)) {
      return "Réglages de Claude Code protégés (SEC0-35) : $File ne se modifie que sur une branche claude/outillage-*, par une PR de chemin sensible."
    }
  }
  if ($MainRoot -and (Test-UnderPath $File $MainRoot) -and
      -not (Test-UnderPath $File (Join-Path $MainRoot '.claude\worktrees'))) {
    return "Copie principale en lecture seule : travaillez dans un worktree (EnterWorktree). Fichier refusé : $File"
  }
  return $null
}

if ($MyInvocation.InvocationName -eq '.') { return }   # chargé par les tests

try {
  $in = Read-HookInput
  $ti = $in.tool_input
  $raw = if ($ti.file_path) { $ti.file_path } else { $ti.notebook_path }
  $file = ConvertTo-FullPath $raw $in.cwd
  $verdict = Get-EditVerdict $file (Get-MainRoot $PSScriptRoot)
  if ($verdict) { Exit-Deny $verdict }
} catch {
  # Garde-fou de commodité : une erreur interne laisse passer, avec un avertissement.
  [Console]::Error.WriteLine("guard-main : erreur interne ($($_.Exception.Message)) ; écriture laissée passer.")
}
exit 0
