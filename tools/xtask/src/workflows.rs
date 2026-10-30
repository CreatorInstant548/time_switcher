//! Contrôle des workflows GitHub Actions (SEC0-29 à 31, F10), par lignes et par indentation,
//! sans dépendance YAML. En cas de doute, la forme est refusée.

/// Actions permises (SEC0-30), épinglées par SHA de 40 caractères.
const ALLOWED_ACTIONS: [&str; 4] = [
    "actions/checkout",
    "actions/cache",
    "swatinem/rust-cache",
    "actions/upload-artifact",
];
/// Seul contexte permis dans une interpolation `${{ }}` d'un `run:` (SEC0-31). Tout le reste
/// passe par `env:` et une variable citée : `github`, `inputs`, `env`, `steps`, `needs`, les
/// fonctions, et aussi `matrix`, qui peut recopier `github.event`.
const RUN_SAFE_CONTEXTS: [&str; 1] = ["runner."];

/// Vrai si une ligne contient une interpolation hors des contextes permis.
fn tainted(line: &str) -> bool {
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
        !(plain && RUN_SAFE_CONTEXTS.iter().any(|c| expr.starts_with(c)))
    })
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

/// (indentation de la clé, clé, valeur) d'une ligne `clé: valeur` ou `- clé: valeur`.
fn key_of(line: &str) -> Option<(usize, &str, &str)> {
    let mut col = indent(line);
    let mut rest = &line[col..];
    if let Some(item) = rest.strip_prefix('-') {
        let item_rest = item.trim_start_matches(' ');
        col += 1 + item.len() - item_rest.len();
        rest = item_rest;
    }
    let (key, value) = rest.split_once(':')?;
    let key = key.trim_matches(['"', '\'']); // clé citée : même traitement
    let valid = !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_-".contains(c));
    (valid && (value.is_empty() || value.starts_with(' '))).then(|| (col, key, value.trim()))
}

fn strip_comment(value: &str) -> &str {
    value.find(" #").map_or(value, |i| &value[..i]).trim()
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

/// Écarts d'un workflow, chacun avec son numéro de ligne.
pub fn check_workflow(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut errors = Vec::new();
    let mut top_permissions = false;
    // (indentation de la clé run:, valeur en bloc littéral `|` ou `>`)
    let mut run_until: Option<(usize, bool)> = None;
    for (i, &line) in lines.iter().enumerate() {
        let n = i + 1;
        let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        let in_run = run_until.is_some_and(|(c, _)| line.trim().is_empty() || indent(line) > c);
        let in_block = in_run && run_until.is_some_and(|(_, block)| block);
        if line.contains('\t') {
            errors.push(format!("ligne {n} : tabulation refusée"));
        }
        // Hors d'un bloc littéral, YAML décode les échappements : `"u\x73es"` vaut `"uses"`.
        if line.contains('\\') && !in_block {
            errors.push(format!(
                "ligne {n} : barre oblique inverse hors d'un bloc run: |"
            ));
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
        let key = key_of(line);
        if tainted(line) && (in_run || key.is_some_and(|k| k.1 == "run")) {
            errors.push(format!(
                "ligne {n} : interpolation hors de runner. dans un run: (SEC0-31)"
            ));
        }
        if in_run {
            continue; // contenu d'un script : seules les interpolations y comptent
        }
        run_until = None;
        // uses: ou run: ailleurs qu'en clé de tête de ligne (collection en ligne, etc.).
        let hidden = ["uses", "run"].iter().any(|k| {
            ["", "\"", "'"]
                .iter()
                .any(|q| compact.contains(&format!("{k}{q}:")))
        });
        if hidden && !key.is_some_and(|k| k.1 == "uses" || k.1 == "run") {
            errors.push(format!("ligne {n} : forme de uses: ou run: non reconnue"));
        }
        let Some((col, k, value)) = key else {
            continue;
        };
        if value.starts_with(['&', '*']) {
            errors.push(format!("ligne {n} : ancre ou alias YAML refusé"));
        }
        // Collection en ligne : seules `{}`, `[]` et une liste de mots (`[push, pull_request]`).
        let v = strip_comment(value);
        if v.starts_with(['{', '[']) && v != "{}" && v != "[]" && v[1..].contains([':', '{', '[']) {
            errors.push(format!("ligne {n} : collection en ligne refusée"));
        }
        if k == "run" {
            run_until = Some((col, value.starts_with(['|', '>'])));
        }
        if k == "permissions" && col == 0 && !line.starts_with('-') {
            top_permissions = strip_comment(value) == "{}";
        }
        if k == "uses" {
            let v = strip_comment(value).trim_matches(['"', '\'']);
            let (repo, sha) = v.split_once('@').unwrap_or((v, ""));
            let owner_repo: Vec<&str> = repo.splitn(3, '/').take(2).collect();
            let allowed =
                ALLOWED_ACTIONS.contains(&owner_repo.join("/").to_ascii_lowercase().as_str());
            let pinned =
                sha.len() == 40 && sha.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
            if !allowed || !pinned {
                errors.push(format!(
                    "ligne {n} : action hors liste ou non épinglée par SHA (SEC0-30)"
                ));
            }
            let checkout = owner_repo
                .join("/")
                .eq_ignore_ascii_case("actions/checkout");
            // persist-credentials: false sous le with: de l'étape, pas ailleurs (env:, etc.).
            let mut under_with = false;
            let no_persist = !step_block(&lines, i, col).iter().any(|l| {
                let Some((c, k, v)) = key_of(l) else {
                    return false;
                };
                if c == col {
                    under_with = k == "with";
                }
                c > col && under_with && k == "persist-credentials" && strip_comment(v) == "false"
            });
            if checkout && no_persist {
                errors.push(format!(
                    "ligne {n} : actions/checkout sans persist-credentials: false"
                ));
            }
        }
    }
    if !top_permissions {
        errors.push("permissions: {} absent au niveau du workflow (SEC0-29)".to_owned());
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
        ];
        for (from, to) in cases {
            let text = good().replacen(from, to, 1);
            assert_ne!(text, good(), "{from}");
            assert!(!check_workflow(&text).is_empty(), "accepté à tort : {to}");
        }
        let inline = good().replace("run: |", "run: echo ${{ github.event.issue.title }}");
        assert!(!check_workflow(&inline).is_empty());
        assert!(!check_workflow(&good().replace("permissions: {}\n", "")).is_empty());
        let runner = good().replace(
            "echo \"$TITLE\"",
            "echo ${{ runner.os }} ${{runner.temp}} \\",
        );
        assert_eq!(check_workflow(&runner), Vec::<String>::new());
        let list = good().replace(
            "  pull_request:",
            "  push:\n    branches: [main, \"release\"]",
        );
        assert_eq!(check_workflow(&list), Vec::<String>::new());
    }
}
