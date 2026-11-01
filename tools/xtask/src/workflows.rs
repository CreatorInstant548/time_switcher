//! Contrôle des workflows GitHub Actions (SEC0-29 à 31, F10), par lignes et par indentation,
//! sans dépendance YAML. **Toute ligne non reconnue est refusée** (I1) : on n'accepte que
//! `clé: valeur`, `- clé: valeur`, `- scalaire`, les commentaires et le contenu des blocs `|`/`>`.

/// Actions permises (SEC0-30), épinglées par SHA de 40 caractères.
const ALLOWED_ACTIONS: [&str; 4] = [
    "actions/checkout",
    "actions/cache",
    "swatinem/rust-cache",
    "actions/upload-artifact",
];
/// Seul contexte sûr dans une interpolation `${{ }}`. Tout autre (`github`, `inputs`, `matrix`,
/// `env`, `steps`, `needs`, les fonctions) n'est permis que sous `env:`, sous `with:` et dans
/// `if:` ; un `run:` le reçoit par `env:` et une variable citée (SEC0-31).
const SAFE_CONTEXTS: [&str; 1] = ["runner."];
/// Clés dont les valeurs descendantes peuvent porter une interpolation non sûre.
const DATA_KEYS: [&str; 2] = ["env", "with"];
/// Premiers caractères d'un scalaire de liste refusés : clé explicite, flux, ancre, alias,
/// balise, bloc, directive, réservés.
const ODD_SCALAR_START: &str = "?:{[&*!|>%@`";

/// Vrai si une ligne contient une interpolation hors des contextes sûrs.
fn unsafe_interpolation(line: &str) -> bool {
    line.split("${{").skip(1).any(|expr| {
        let expr: String = expr
            .split("}}")
            .next()
            .unwrap_or_default()
            .split_whitespace()
            .collect();
        let expr = expr.to_ascii_lowercase();
        let plain = expr
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b));
        !(plain && SAFE_CONTEXTS.iter().any(|c| expr.starts_with(c)))
    })
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

/// Ligne structurée reconnue.
struct Line<'a> {
    /// Élément de liste (`- …`), dont le tiret est à la colonne `dash`.
    item: bool,
    dash: usize,
    /// Colonne de la clé (ou du scalaire, après le tiret).
    col: usize,
    /// Clé en minuscules (m4) ; `None` pour un scalaire de liste.
    key: Option<String>,
    value: &'a str,
}

/// `clé: valeur`, `- clé: valeur` ou `- scalaire` ; `None` pour toute autre forme.
fn parse(line: &str) -> Option<Line<'_>> {
    let dash = indent(line);
    let mut col = dash;
    let mut rest = &line[dash..];
    let item = rest == "-" || rest.starts_with("- ");
    if item {
        let r = rest[1..].trim_start_matches(' ');
        col += rest.len() - r.len();
        rest = r;
    }
    let pair = rest
        .split_once(':')
        .filter(|(_, v)| v.is_empty() || v.starts_with(' '));
    match pair {
        Some((key, value)) => {
            let key = key.trim_matches(['"', '\'']); // clé citée : même traitement
            let valid = !key.is_empty()
                && key
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "_-".contains(c));
            let key = Some(key.to_ascii_lowercase());
            let value = value.trim();
            valid.then_some(Line {
                item,
                dash,
                col,
                key,
                value,
            })
        }
        None if item && !rest.is_empty() && !rest.starts_with(|c| ODD_SCALAR_START.contains(c)) => {
            let value = rest.trim();
            Some(Line {
                item,
                dash,
                col,
                key: None,
                value,
            })
        }
        None => None,
    }
}

fn strip_comment(value: &str) -> &str {
    value.find(" #").map_or(value, |i| &value[..i]).trim()
}

/// Niveau ouvert de la pile d'indentation : clé, ou élément de liste (`key` = `-`).
struct Level {
    col: usize,
    key: String,
    /// La clé attend des enfants (valeur vide).
    open: bool,
    /// Colonne de ses enfants, fixée par le premier.
    child: Option<usize>,
}

/// Place la ligne dans la pile ; `false` si son indentation est incohérente.
fn place(stack: &mut Vec<Level>, l: &Line) -> bool {
    let fits = |parent: &mut Level, col: usize| {
        let ok = parent.open && parent.child.is_none_or(|c| c == col);
        parent.child = Some(col);
        ok
    };
    if l.item {
        while stack
            .last()
            .is_some_and(|t| t.col > l.dash || (t.col == l.dash && t.key == "-"))
        {
            stack.pop();
        }
        let Some(parent) = stack.last_mut() else {
            return false;
        };
        let ok = parent.col <= l.dash && fits(parent, l.dash);
        let open = l.key.is_some();
        stack.push(Level {
            col: l.dash,
            key: "-".into(),
            open,
            child: Some(l.col),
        });
        ok
    } else {
        while stack.last().is_some_and(|t| t.col > l.col) {
            stack.pop();
        }
        if stack.last().is_some_and(|t| t.col == l.col && t.key != "-") {
            stack.pop();
        }
        match stack.last_mut() {
            None => l.col == 0,
            Some(p) if p.key == "-" => p.child == Some(l.col),
            Some(p) => p.col < l.col && fits(p, l.col),
        }
    }
}

/// Lignes de l'élément de liste (étape) qui contient la ligne `i`, de clé à l'indentation `col`.
fn step_block<'b, 'a>(lines: &'b [&'a str], i: usize, col: usize) -> &'b [&'a str] {
    let blank = |l: &str| l.trim().is_empty();
    let mut start = i;
    while !lines[start].trim_start().starts_with('-') && start > 0 {
        let prev = lines[start - 1];
        if !blank(prev) && indent(prev) < col && !prev.trim_start().starts_with('-') {
            break;
        }
        start -= 1;
    }
    let mut end = i + 1;
    while end < lines.len() && (blank(lines[end]) || indent(lines[end]) >= col) {
        end += 1;
    }
    &lines[start..end]
}

/// `uses:` d'une étape : action de la liste, épinglée ; `persist-credentials: false` sous le
/// `with:` d'un `actions/checkout`.
fn check_uses(lines: &[&str], i: usize, l: &Line, errors: &mut Vec<String>) {
    let n = i + 1;
    let v = strip_comment(l.value).trim_matches(['"', '\'']);
    let (repo, sha) = v.split_once('@').unwrap_or((v, ""));
    let owner_repo = repo.splitn(3, '/').take(2).collect::<Vec<_>>().join("/");
    let owner_repo = owner_repo.to_ascii_lowercase();
    let pinned = sha.len() == 40 && sha.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
    if !ALLOWED_ACTIONS.contains(&owner_repo.as_str()) || !pinned {
        errors.push(format!(
            "ligne {n} : action hors liste ou non épinglée par SHA (SEC0-30)"
        ));
    }
    let mut under_with = false;
    let persist = step_block(lines, i, l.col).iter().any(|s| {
        let Some(s) = parse(s) else { return false };
        let key = s.key.as_deref().unwrap_or_default();
        if s.col == l.col {
            under_with = key == "with";
        }
        s.col > l.col
            && under_with
            && key == "persist-credentials"
            && strip_comment(s.value) == "false"
    });
    if owner_repo == "actions/checkout" && !persist {
        errors.push(format!(
            "ligne {n} : actions/checkout sans persist-credentials: false sous with:"
        ));
    }
}

/// Action épinglée d'une ligne `uses:` : dépôt `propriétaire/nom`, SHA et étiquette du commentaire.
#[derive(Debug, PartialEq, Eq)]
pub struct Pin {
    pub line: usize,
    pub repo: String,
    pub sha: String,
    pub tag: Option<String>,
}

/// Lignes `uses:` d'un workflow déjà accepté par [`check_workflow`] (T3-b, J8). L'étiquette est
/// le premier mot du commentaire (`# v7.0.1`), en caractères sûrs pour une adresse de l'API.
pub fn pinned_uses(text: &str) -> Vec<Pin> {
    let safe = |s: &str| {
        !s.is_empty()
            && !s.starts_with(['.', '-'])
            && !s.contains("..")
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
    };
    let mut out = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let Some(l) = parse(raw).filter(|l| l.key.as_deref() == Some("uses")) else {
            continue;
        };
        let v = strip_comment(l.value).trim_matches(['"', '\'']);
        let (path, sha) = v.split_once('@').unwrap_or((v, ""));
        let repo = path.splitn(3, '/').take(2).collect::<Vec<_>>().join("/");
        let tag = l
            .value
            .find(" #")
            .and_then(|c| l.value[c + 2..].split_whitespace().next());
        out.push(Pin {
            line: i + 1,
            repo,
            sha: sha.to_owned(),
            tag: tag.filter(|t| safe(t)).map(str::to_owned),
        });
    }
    out
}

/// Écarts d'un workflow, chacun avec son numéro de ligne.
pub fn check_workflow(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut errors = Vec::new();
    let mut top_permissions = false;
    let mut stack: Vec<Level> = Vec::new();
    // Bloc `|` ou `>` ouvert : (colonne de sa clé, interpolations non sûres permises).
    let mut block: Option<(usize, bool)> = None;
    for (i, &raw) in lines.iter().enumerate() {
        let n = i + 1;
        let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
        let compact = compact.to_ascii_lowercase();
        if raw.contains('\t') {
            errors.push(format!("ligne {n} : tabulation refusée"));
        }
        for bad in [
            "pull_request_target",
            "workflow_run",
            "secrets:inherit",
            "<<:",
        ] {
            if compact.contains(bad) {
                errors.push(format!("ligne {n} : {bad} interdit"));
            }
        }
        if let Some((col, data)) = block {
            if raw.trim().is_empty() || indent(raw) > col {
                if unsafe_interpolation(raw) && !data {
                    errors.push(format!(
                        "ligne {n} : interpolation non sûre hors de env:, with: et if: (SEC0-31)"
                    ));
                }
                continue; // contenu d'un bloc : seules les interpolations y comptent
            }
            block = None;
        }
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        // Hors d'un bloc, YAML décode les échappements : `"u\x73es"` vaut `"uses"`.
        if raw.contains('\\') {
            errors.push(format!(
                "ligne {n} : barre oblique inverse hors d'un bloc | ou >"
            ));
        }
        let Some(l) = parse(raw) else {
            errors.push(format!(
                "ligne {n} : forme non reconnue (clé explicite, flux, ancre, balise…)"
            ));
            continue;
        };
        if !place(&mut stack, &l) {
            errors.push(format!("ligne {n} : indentation incohérente"));
        }
        let key = l.key.as_deref().unwrap_or_default();
        // Les ancêtres sont en pile ; la ligne elle-même n'y est que si c'est une clé d'élément.
        let data = key == "if"
            || stack
                .iter()
                .any(|lv| DATA_KEYS.contains(&lv.key.as_str()) && lv.col < l.col);
        let data = data && key != "run";
        if unsafe_interpolation(raw) && !data {
            errors.push(format!(
                "ligne {n} : interpolation non sûre hors de env:, with: et if: (SEC0-31)"
            ));
        }
        let hidden = ["uses", "run"].iter().any(|k| {
            ["", "\"", "'"]
                .iter()
                .any(|q| compact.contains(&format!("{k}{q}:")))
        });
        if hidden && key != "uses" && key != "run" {
            errors.push(format!("ligne {n} : forme de uses: ou run: non reconnue"));
        }
        let v = strip_comment(l.value);
        if v.starts_with(['&', '*', '!']) {
            errors.push(format!("ligne {n} : ancre, alias ou balise YAML refusé"));
        }
        // Collection en ligne : seules `{}`, `[]` et une liste de mots (`[push, pull_request]`).
        if v.starts_with(['{', '[']) && v != "{}" && v != "[]" && v[1..].contains([':', '{', '[']) {
            errors.push(format!("ligne {n} : collection en ligne refusée"));
        }
        if v.starts_with(['|', '>']) {
            block = Some((l.col, data));
        }
        if l.key.is_some() {
            let open = v.is_empty();
            stack.push(Level {
                col: l.col,
                key: key.to_owned(),
                open,
                child: None,
            });
        }
        if key == "permissions" && !l.item && l.col == 0 {
            top_permissions = v == "{}";
        }
        if key == "uses" {
            check_uses(&lines, i, &l, &mut errors);
        }
    }
    if !top_permissions {
        errors.push("permissions: {} absent au niveau du workflow (SEC0-29)".to_owned());
    }
    errors
}

/// Pinned installer of the CI tools; its keys live in [`TOOLS_LOCK`] (SEC0-06, G6).
pub const INSTALL_SCRIPT: &str = "tools/ci/install-tools.sh";
pub const TOOLS_LOCK: &str = "tools/dev/tools.lock";

/// Value of `key` in `tools.lock`, read exactly as `lock_value` of [`INSTALL_SCRIPT`] reads it:
/// first line matching `^<key> *= *"(.*)".*` (greedy, up to the last quote), `None` when absent
/// or empty. A looser reader would accept a line the script then refuses at run time (#75).
fn sh_lock_value<'a>(lock: &'a str, key: &str) -> Option<&'a str> {
    lock.lines()
        .find_map(|line| {
            let rest = line.strip_prefix(key)?.trim_start_matches(' ');
            let rest = rest.strip_prefix('=')?.trim_start_matches(' ');
            let rest = rest.strip_prefix('"')?;
            Some(&rest[..rest.rfind('"')?])
        })
        .filter(|v| !v.is_empty())
}

/// `tools.lock` keys read by each subcommand of the installer: its `case` line `name) function ;;`
/// gives the function, whose body (up to the first line `}`) gives the `lock_value '<key>'` calls.
fn installer_keys(script: &str) -> Vec<(String, Vec<String>)> {
    let ident = |s: &str| {
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
    };
    let mut bodies: Vec<(String, Vec<String>)> = Vec::new();
    let mut current: Option<(String, Vec<String>)> = None;
    for line in script.lines() {
        if let Some(name) = line.strip_suffix("() {").filter(|n| ident(n)) {
            current = Some((name.to_owned(), Vec::new()));
        } else if line == "}" {
            bodies.extend(current.take());
        } else if let Some((_, keys)) = current.as_mut() {
            for call in line.split("lock_value '").skip(1) {
                if let Some((key, _)) = call.split_once('\'') {
                    keys.push(key.replace("\\.", "."));
                }
            }
        }
    }
    let mut out = Vec::new();
    for line in script.lines() {
        let Some((name, rest)) = line.trim().split_once(") ") else {
            continue;
        };
        let Some(function) = rest.strip_suffix(";;").map(str::trim) else {
            continue;
        };
        if !(ident(name) && ident(function)) {
            continue;
        }
        let keys = bodies
            .iter()
            .find(|(f, _)| f == function)
            .map(|(_, k)| k.clone())
            .unwrap_or_default();
        out.push((name.to_owned(), keys));
    }
    out
}

/// Every call of the installer in a workflow must name a known subcommand whose `tools.lock`
/// keys are all present and non-empty (#75: `cargo-fuzz` was called by nightly.yml with no
/// pinned version, so the weekly fuzzing failed on its first run). `workflows`: (path, text).
pub fn tool_lock_errors(workflows: &[(String, String)], script: &str, lock: &str) -> Vec<String> {
    let known = installer_keys(script);
    let mut errors = Vec::new();
    for (path, text) in workflows {
        for (i, line) in text.lines().enumerate() {
            let n = i + 1;
            if line.trim_start().starts_with('#') {
                continue; // YAML or shell comment: nothing runs
            }
            // Split on the file name: any path or quoting of the script is still a call.
            for call in line.split("install-tools.sh").skip(1) {
                let mut words = call
                    .split_whitespace()
                    .map(|w| w.trim_matches(['"', '\'']))
                    .filter(|w| !w.is_empty());
                let keys = match words.next() {
                    Some("version") => match words.next() {
                        Some(tool) if !tool.is_empty() && !tool.contains('$') => {
                            vec![format!("{tool}.version")]
                        }
                        _ => {
                            errors.push(format!(
                                "{path} : line {n} : install-tools.sh version without a literal tool name"
                            ));
                            continue;
                        }
                    },
                    Some(sub) => match known.iter().find(|(name, _)| name == sub) {
                        Some((_, keys)) => keys.clone(),
                        None => {
                            errors.push(format!(
                                "{path} : line {n} : unknown install-tools.sh subcommand `{sub}`"
                            ));
                            continue;
                        }
                    },
                    None => {
                        errors.push(format!(
                            "{path} : line {n} : install-tools.sh without a subcommand"
                        ));
                        continue;
                    }
                };
                for key in keys {
                    if sh_lock_value(lock, &key).is_none() {
                        errors.push(format!(
                            "{path} : line {n} : {key} missing or empty in {TOOLS_LOCK} (SEC0-06)"
                        ));
                    }
                }
            }
        }
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA: &str = "11bd71901bbe5b1630ceea73d27597364c9af683";

    fn good() -> String {
        format!(
            "on:\n  pull_request:\npermissions: {{}}\njobs:\n  a:\n    permissions:\n      contents: read\n    \
             steps:\n      - uses: actions/checkout@{SHA} # v4\n        with:\n          persist-credentials: false\n      \
             - name: t\n        env:\n          TITLE: ${{{{ github.event.pull_request.title }}}}\n        run: |\n          \
             echo \"$TITLE\"\n          cargo xtask check --only workflows\n"
        )
    }

    #[test]
    fn accepts_a_conforming_workflow() {
        assert_eq!(check_workflow(&good()), Vec::<String>::new());
    }

    #[test]
    fn refuses_each_violation() {
        let cases = [
            ("permissions: {}", "permissions: write-all"),
            ("pull_request:", "pull_request_target:"),
            ("persist-credentials: false", "persist-credentials: true"),
            (&format!("@{SHA}") as &str, "@v4"),
            ("actions/checkout@", "evil/checkout@"),
            ("echo \"$TITLE\"", "echo \"${{ github.head_ref }}\""),
            ("cargo xtask", "echo ${{inputs.x}} && cargo xtask"),
            (
                "      - name: t",
                "      - uses: docker://alpine\n      - name: t",
            ),
            (
                "      - name: t",
                "      - {uses: actions/cache@x}\n      - name: t",
            ),
            ("TITLE: ${{", "TITLE: *alias\n          X: ${{"),
            (
                "echo \"$TITLE\"",
                "echo \"${{ GITHUB.EVENT.issue.title }}\"",
            ),
            ("echo \"$TITLE\"", "echo '${{ toJSON(github) }}'"),
            ("echo \"$TITLE\"", "echo ${{ env.TITLE }}"),
            (
                "      - name: t",
                "      - \"uses\": evil/a@main\n      - name: t",
            ),
            (
                "        run: |",
                "        'run': echo ${{ github.head_ref }} |",
            ),
            // Contournements relevés par la relecture de sécurité.
            ("echo \"$TITLE\"", "echo ${{ matrix.t }}"),
            ("    steps:", "    steps: [{uses: evil/x@main}]\n    other:"),
            (
                "    steps:",
                "    steps: [{run: \"echo ${{ github.head_ref }}\"}]\n    other:",
            ),
            (
                "      - name: t",
                "      - \"u\\x73es\": evil/x@main\n      - name: t",
            ),
            ("  pull_request:", "  \"pull_request_t\\x61rget\":"),
            ("        with:\n", "        env:\n"),
            // Relecture neuve, I1 : clés explicites, balises, flux, indentation, interpolations.
            (
                "        run: |",
                "        ? run\n        : echo \"${{ github.head_ref }}\"\n        x: |",
            ),
            (
                "      - name: t",
                "      - name: k\n        ? uses\n        : evil/x@main\n      - name: t",
            ),
            ("      - name: t", "      - ? uses\n      - name: t"),
            ("TITLE: ${{", "TITLE: !!str x\n          X: ${{"),
            ("TITLE: ${{", "TITLE: &a x\n          X: ${{"),
            ("        env:", "         env:"),
            (
                "      contents: read",
                "      contents: read\n     actions: read",
            ),
            ("  a:\n", "  name: ${{ github.event.issue.title }}\n  a:\n"),
            ("  a:\n", "  a:\n    runs-on: ${{ matrix.os }}\n"),
            ("permissions: {}", "permissions: {}\n---"),
            // m4 : clés comparées sans tenir compte de la casse.
            (
                "      - name: t",
                "      - USES: evil/x@main\n      - name: t",
            ),
            (
                "        run: |",
                "        Run: echo ${{ github.head_ref }} |",
            ),
            ("persist-credentials: false", "Persist-Credentials: true"),
        ];
        for (from, to) in cases {
            let text = good().replacen(from, to, 1);
            assert_ne!(text, good(), "{from}");
            assert!(!check_workflow(&text).is_empty(), "accepté à tort : {to}");
        }
        let inline = good().replace("run: |", "run: echo ${{ github.event.issue.title }}");
        assert!(!check_workflow(&inline).is_empty());
        assert!(!check_workflow(&good().replace("permissions: {}\n", "")).is_empty());
    }

    #[test]
    fn reads_pins() {
        let text = good().replace(
            "      - name: t",
            &format!("      - uses: actions/cache/restore@{SHA}\n      - uses: \"Swatinem/rust-cache@{SHA}\" # ../x\n      - name: t"),
        );
        let pins = pinned_uses(&text);
        assert_eq!(pins.len(), 3);
        assert_eq!(
            pins[0],
            Pin {
                line: 9,
                repo: "actions/checkout".into(),
                sha: SHA.into(),
                tag: Some("v4".into())
            }
        );
        assert_eq!(
            (pins[1].repo.as_str(), pins[1].tag.as_deref()),
            ("actions/cache", None)
        );
        assert_eq!(
            (pins[2].repo.as_str(), pins[2].tag.as_deref()),
            ("Swatinem/rust-cache", None)
        );
    }

    #[test]
    fn accepts_safe_forms() {
        let runner = good().replace(
            "echo \"$TITLE\"",
            "echo ${{ runner.os }} ${{runner.temp}} \\",
        );
        assert_eq!(check_workflow(&runner), Vec::<String>::new());
        let list = good().replace(
            "  pull_request:",
            "  push:\n    branches: [main, \"release\"]\n    tags:\n      - v1\n      - 'v2'",
        );
        assert_eq!(check_workflow(&list), Vec::<String>::new());
        let data = good().replace(
            "    steps:",
            "    if: ${{ github.event_name == 'push' }}\n    env:\n      M: ${{ matrix.os }}\n    steps:",
        );
        assert_eq!(check_workflow(&data), Vec::<String>::new());
        let with = good().replace(
            "          persist-credentials: false",
            "          persist-credentials: false\n          ref: ${{ github.head_ref }}",
        );
        assert_eq!(check_workflow(&with), Vec::<String>::new());
        let flush = good().replace("      - uses", "    - uses").replace(
            "        with:\n          persist",
            "      with:\n        persist",
        );
        let flush = flush.replace(
            "      - name: t\n        env:\n          TITLE",
            "    - name: t\n      env:\n        TITLE",
        );
        let flush = flush.replace(
            "        run: |\n          echo \"$TITLE\"\n          cargo",
            "      run: |\n        echo \"$TITLE\"\n        cargo",
        );
        assert_eq!(check_workflow(&flush), Vec::<String>::new());
    }

    const SCRIPT: &str = include_str!("../../ci/install-tools.sh");
    const LOCK: &str = include_str!("../../dev/tools.lock");

    fn repo_workflows() -> Vec<(String, String)> {
        vec![
            (
                "ci.yml".into(),
                include_str!("../../../.github/workflows/ci.yml").into(),
            ),
            (
                "nightly.yml".into(),
                include_str!("../../../.github/workflows/nightly.yml").into(),
            ),
        ]
    }

    #[test]
    fn reads_lock_values_like_the_installer() {
        let lock = "# a.version = \"1\"\n a.version = \"2\"\na.versionx = \"3\"\nb.version = \"\"\n\
                    c.version=\"4\"\nc.version = \"5\"\nd.version = 6\ne.version  =  \"7\" # \"x\"\n";
        assert_eq!(
            sh_lock_value(lock, "a.version"),
            None,
            "comment, indent, prefix"
        );
        assert_eq!(sh_lock_value(lock, "b.version"), None, "empty");
        assert_eq!(
            sh_lock_value(lock, "c.version"),
            Some("4"),
            "first line wins"
        );
        assert_eq!(sh_lock_value(lock, "d.version"), None, "unquoted");
        assert_eq!(
            sh_lock_value(lock, "e.version"),
            Some("7\" # \"x"),
            "greedy, as sed"
        );
        assert_eq!(sh_lock_value(LOCK, "cargo-fuzz.version"), Some("0.13.2"));
    }

    #[test]
    fn maps_installer_subcommands_to_their_keys() {
        let keys = installer_keys(SCRIPT);
        let of = |sub: &str| keys.iter().find(|(s, _)| s == sub).map(|(_, k)| k.clone());
        assert_eq!(of("rust"), Some(vec![]));
        assert_eq!(
            of("cargo-fuzz"),
            Some(vec!["cargo-fuzz.version".to_owned()])
        );
        assert_eq!(
            of("cargo-audit"),
            Some(vec!["cargo-audit.version".to_owned()])
        );
        let gitleaks = of("gitleaks").unwrap();
        assert!(gitleaks.contains(&"gitleaks.linux.sha256".to_owned()));
        assert!(gitleaks.contains(&"gitleaks.version".to_owned()));
        assert_eq!(
            of("gitleaks-selftest"),
            Some(vec!["gitleaks.linux.path".to_owned()])
        );
        assert_eq!(
            of("version"),
            None,
            "handled apart: its key depends on the call"
        );
    }

    /// #75: nightly.yml called `install-tools.sh cargo-fuzz` while tools.lock had no
    /// `cargo-fuzz.version`; the weekly fuzzing failed at run time instead of in the PR.
    #[test]
    fn refuses_an_installer_call_without_its_pin() {
        let flows = repo_workflows();
        assert_eq!(tool_lock_errors(&flows, SCRIPT, LOCK), Vec::<String>::new());
        let unpinned: String = LOCK
            .lines()
            .filter(|l| !l.starts_with("cargo-fuzz.version"))
            .map(|l| format!("{l}\n"))
            .collect();
        let errors = tool_lock_errors(&flows, SCRIPT, &unpinned);
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].starts_with("nightly.yml : line "), "{errors:?}");
        assert!(
            errors[0].contains("cargo-fuzz.version missing or empty"),
            "{errors:?}"
        );
        let empty = LOCK.replace(
            "cargo-fuzz.version = \"0.13.2\"",
            "cargo-fuzz.version = \"\"",
        );
        assert_eq!(tool_lock_errors(&flows, SCRIPT, &empty).len(), 1);
        for (call, what) in [
            ("bash tools/ci/install-tools.sh maturin", "unknown"),
            ("bash ./tools/ci/install-tools.sh \"maturin\"", "unknown"),
            (
                "bash tools/ci/install-tools.sh version maturin",
                "maturin.version missing",
            ),
            (
                "bash tools/ci/install-tools.sh version \"$T\"",
                "literal tool name",
            ),
            ("bash tools/ci/install-tools.sh", "without a subcommand"),
        ] {
            let flow = vec![(
                "w.yml".to_owned(),
                format!("    steps:\n      - run: {call}\n"),
            )];
            let errors = tool_lock_errors(&flow, SCRIPT, LOCK);
            assert!(
                errors.len() == 1 && errors[0].contains(what),
                "{call}: {errors:?}"
            );
        }
        let comment = vec![(
            "w.yml".to_owned(),
            "# install-tools.sh maturin\n      - run: |\n          # bash install-tools.sh maturin\n"
                .to_owned(),
        )];
        assert_eq!(
            tool_lock_errors(&comment, SCRIPT, LOCK),
            Vec::<String>::new()
        );
    }
}
