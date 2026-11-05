//! `ran check` — read a NTCRANFILE the way `ran` does and report every
//! problem with the line that has to change, instead of bailing on the first
//! one. The two mistakes that break a ranfile most often get their own
//! dedicated diagnosis: a single `\` where TOML wants `\\`, and random text
//! sitting where a `key = "value"` line belongs.

use crate::ranfile::parser::RESERVED_TARGET_NAMES;
use crate::ranfile::types::{RanFormat, Ranfile};
use std::collections::{HashMap, HashSet};

/// One problem found while interpreting a NTCRANFILE.
pub(crate) struct Problem {
    /// 1-based line number; 0 when the problem is not tied to a line.
    pub(crate) line: usize,
    /// 1-based column number; 0 when unknown.
    pub(crate) col: usize,
    pub(crate) message: String,
    pub(crate) hint: Option<String>,
}

impl Problem {
    fn at(line: usize, col: usize, message: impl Into<String>, hint: Option<String>) -> Self {
        Self {
            line,
            col,
            message: message.into(),
            hint,
        }
    }
}

/// Interpret a TOML ranfile and return every problem, ordered by position.
pub(crate) fn check_text(content: &str) -> Vec<Problem> {
    check_text_as(content, RanFormat::Toml)
}

/// Interpret a ranfile in `fmt`.
///
/// TOML keeps the full diagnosis (escapes, stray text, tables). JSON and YAML
/// borrow their parser's own error position and then run the same structural
/// checks, so a converted file is still validated end to end.
pub(crate) fn check_text_as(content: &str, fmt: RanFormat) -> Vec<Problem> {
    if fmt == RanFormat::Toml {
        return check_toml(content);
    }
    let mut problems = Vec::new();
    match parse_foreign(content, fmt) {
        Ok(ranfile) => problems.extend(semantic_problems(&ranfile, content, fmt)),
        Err((line, col, message)) => problems.push(Problem::at(
            line,
            col,
            format!("invalid {}: {}", fmt.ext().to_uppercase(), message.trim()),
            None,
        )),
    }
    problems.sort_by_key(|p| (p.line, p.col));
    problems.dedup_by(|a, b| a.line == b.line && a.col == b.col && a.message == b.message);
    problems
}

/// Parse a JSON/YAML ranfile, keeping the parser's 1-based position.
fn parse_foreign(content: &str, fmt: RanFormat) -> Result<Ranfile, (usize, usize, String)> {
    match fmt {
        RanFormat::Json => {
            serde_json::from_str(content).map_err(|e| (e.line(), e.column(), e.to_string()))
        }
        RanFormat::Yaml => serde_yaml::from_str(content).map_err(|e| {
            let location = e.location();
            (
                location.as_ref().map(|l| l.line()).unwrap_or(0),
                location.as_ref().map(|l| l.column()).unwrap_or(0),
                e.to_string(),
            )
        }),
        RanFormat::Toml => unreachable!("TOML goes through check_toml"),
    }
}

/// The TOML-only interpreter.
fn check_toml(content: &str) -> Vec<Problem> {
    let mut problems = escape_problems(content);
    problems.extend(control_char_problems(content));
    problems.extend(stray_text_problems(content));

    match toml::from_str::<Ranfile>(content) {
        Ok(ranfile) => problems.extend(semantic_problems(&ranfile, content, RanFormat::Toml)),
        Err(err) => {
            let (line, col) = err
                .span()
                .map(|span| line_col(content, span.start))
                .unwrap_or((0, 0));
            let message = err.message().trim().to_string();
            let hint = if message.contains("escape") {
                Some(
                    "a literal backslash is written as \\\\ (two backslashes) inside a \"…\" string, \
                     or use a 'single quoted' string where backslashes are literal"
                        .to_string(),
                )
            } else if message.contains("expected") || message.contains("unexpected") {
                Some(
                    "only `key = \"value\"` lines and `[table]` headers belong in this file"
                        .to_string(),
                )
            } else if message.contains("duplicate key") {
                Some("each key may only appear once per table".to_string())
            } else {
                None
            };
            // A line that already carries its own diagnosis (bad escape,
            // stray text) does not need TOML's generic complaint on top.
            if line == 0 || !problems.iter().any(|p| p.line == line) {
                problems.push(Problem::at(
                    line,
                    col,
                    format!("invalid TOML: {}", message),
                    hint,
                ));
            }
        }
    }

    problems.sort_by_key(|p| (p.line, p.col));
    problems.dedup_by(|a, b| a.line == b.line && a.col == b.col && a.message == b.message);
    problems
}

/// Characters TOML accepts after a backslash inside a basic string.
const VALID_ESCAPES: &[char] = &['b', 't', 'n', 'f', 'r', '"', '\\', 'u', 'U'];

/// Legal escapes that turn into a control character. In a ranfile they are
/// almost always a Windows path typed with one backslash (`packaging\build.cmd`
/// parses fine but silently becomes `packaging` + a backspace).
const CONTROL_ESCAPES: &[char] = &['b', 't', 'n', 'f', 'r'];

/// `\` escapes that TOML rejects — the single most common ranfile breakage
/// (`cmd = "packaging\build.cmd"` instead of `packaging\\build.cmd`).
fn escape_problems(content: &str) -> Vec<Problem> {
    let mut out = Vec::new();
    let mut multiline_basic = false;
    let mut multiline_literal = false;

    for (idx, line) in content.lines().enumerate() {
        let line_no = idx + 1;

        if multiline_literal {
            if line.matches("'''").count() % 2 == 1 {
                multiline_literal = false;
            }
            continue;
        }
        if !multiline_basic {
            if line.matches("\"\"\"").count() % 2 == 1 {
                multiline_basic = true;
            }
            if line.matches("'''").count() % 2 == 1 {
                multiline_literal = true;
                continue;
            }
        } else if line.matches("\"\"\"").count() % 2 == 1 {
            multiline_basic = false;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        let mut in_basic = multiline_basic;
        let mut in_literal = false;
        while i < chars.len() {
            let c = chars[i];
            if in_literal {
                if c == '\'' {
                    in_literal = false;
                }
                i += 1;
                continue;
            }
            if in_basic {
                if c == '\\' {
                    match chars.get(i + 1).copied() {
                        Some(next) if VALID_ESCAPES.contains(&next) => {
                            if CONTROL_ESCAPES.contains(&next) {
                                out.push(Problem::at(
                                    line_no,
                                    i + 1,
                                    format!(
                                        "escape '\\{}' becomes a control character, not a path separator",
                                        next
                                    ),
                                    Some(format!("use a doubled backslash: {}", doubled(&chars, i))),
                                ));
                            }
                            i += 2;
                            continue;
                        }
                        Some(next) => {
                            out.push(Problem::at(
                                line_no,
                                i + 1,
                                format!("invalid escape '\\{}' in a string", next),
                                Some(format!("use a doubled backslash: {}", doubled(&chars, i))),
                            ));
                            i += 2;
                            continue;
                        }
                        None => {
                            i += 1;
                            continue;
                        }
                    }
                }
                if c == '"' {
                    in_basic = false;
                }
                i += 1;
                continue;
            }
            match c {
                '"' => in_basic = true,
                '\'' => in_literal = true,
                // `#` starts a comment only outside a string.
                '#' => break,
                _ => {}
            }
            i += 1;
        }
    }

    out
}

/// Raw control characters: what a single `\b`/`\t`/`\n` collapses into when
/// the file was written by a tool that interpreted the escape itself.
fn control_char_problems(content: &str) -> Vec<Problem> {
    let mut out = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_control() && ch != '\t' && ch != '\r' {
                out.push(Problem::at(
                    idx + 1,
                    col + 1,
                    format!("raw control character U+{:04X} in the file", ch as u32),
                    Some(
                        "a single \\b / \\t / \\n collapsed into this — write the backslash twice (\\\\)"
                            .to_string(),
                    ),
                ));
                break;
            }
        }
    }
    out
}

/// The line with the backslash at `at` doubled — the fix to copy and paste.
fn doubled(chars: &[char], at: usize) -> String {
    let mut fixed: String = chars[..at].iter().collect();
    fixed.push_str("\\\\");
    fixed.extend(&chars[at + 1..]);
    fixed.trim().to_string()
}

/// Lines that are neither a comment, a `[table]` header nor `key = value`:
/// the "random text in the wrong place" that makes ran unusable.
fn stray_text_problems(content: &str) -> Vec<Problem> {
    let mut out = Vec::new();
    let mut array_depth: i32 = 0;
    let mut in_multiline = false;

    for (idx, line) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_multiline {
            let quotes = trimmed.matches("\"\"\"").count() + trimmed.matches("'''").count();
            if quotes % 2 == 1 {
                in_multiline = false;
            }
            continue;
        }
        if array_depth > 0 {
            array_depth += trimmed.matches('[').count() as i32;
            array_depth -= trimmed.matches(']').count() as i32;
            array_depth = array_depth.max(0);
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
            continue;
        }
        if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
            let quotes = trimmed.matches("\"\"\"").count() + trimmed.matches("'''").count();
            if quotes % 2 == 1 {
                in_multiline = true;
            }
            continue;
        }

        let Some(eq) = line.find('=') else {
            out.push(Problem::at(
                line_no,
                line.len() - line.trim_start().len() + 1,
                "stray text where a `key = \"value\"` line was expected",
                Some(
                    "remove it, or write it as `key = \"value\"` under [vars] or [targets.<name>]"
                        .to_string(),
                ),
            ));
            continue;
        };

        // An array that opens on this line and closes on a later one.
        let rest = &line[eq + 1..];
        if rest.contains('[') && !rest.contains(']') {
            array_depth = 1;
        }
    }

    out
}

/// Structural checks over the parsed file: reserved names, missing
/// dependencies, cycles, empty targets and `$(VAR)` references that do not
/// exist under `[vars]` (they silently expand to nothing at run time).
/// Where a target header appears, per format.
fn header_needle(fmt: RanFormat, name: &str) -> String {
    match fmt {
        RanFormat::Toml => format!("[targets.{}]", name),
        RanFormat::Json => format!("\"{}\"", name),
        RanFormat::Yaml => format!("{}:", name),
    }
}

/// Where a `[vars]` entry appears, per format.
fn var_needle(fmt: RanFormat, name: &str) -> String {
    match fmt {
        RanFormat::Toml => format!("{} =", name),
        RanFormat::Json => format!("\"{}\":", name),
        RanFormat::Yaml => format!("{}:", name),
    }
}

fn semantic_problems(ranfile: &Ranfile, content: &str, fmt: RanFormat) -> Vec<Problem> {
    let mut out = Vec::new();
    let reserved = RESERVED_TARGET_NAMES
        .iter()
        .map(|n| *n)
        .collect::<Vec<&str>>()
        .join(", ");
    let defined: HashSet<&str> = ranfile.vars.keys().map(String::as_str).collect();

    for (name, target) in &ranfile.targets {
        let line = find_line(content, &header_needle(fmt, name)).unwrap_or(0);

        if RESERVED_TARGET_NAMES.contains(&name.as_str()) {
            out.push(Problem::at(
                line,
                0,
                format!("'{}' is reserved and cannot be a target name", name),
                Some(format!("reserved names: {}", reserved)),
            ));
        }

        for dep in &target.deps {
            if !ranfile.targets.contains_key(dep) {
                out.push(Problem::at(
                    line,
                    0,
                    format!(
                        "target '{}' depends on '{}', which is not defined",
                        name, dep
                    ),
                    Some(format!("add [targets.{}] or drop it from deps", dep)),
                ));
            }
        }

        if target.cmd.trim().is_empty() && target.deps.is_empty() {
            out.push(Problem::at(
                line,
                0,
                format!("target '{}' has no cmd and no deps", name),
                Some("give it a cmd = \"…\" or at least one dependency".to_string()),
            ));
        }

        let cmd_line = find_line(content, target.cmd.trim()).unwrap_or(line);
        for var in var_refs(&target.cmd) {
            if !defined.contains(var.as_str()) {
                out.push(Problem::at(
                    cmd_line,
                    0,
                    format!("$({}) is not defined under [vars]", var),
                    Some(
                        "add it to [vars] or fix the spelling — unknown vars expand to nothing"
                            .to_string(),
                    ),
                ));
            }
        }
    }

    // Variable values can reference other variables too.
    let var_map: HashMap<&str, &str> = ranfile
        .vars
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    for (name, value) in &var_map {
        let line = find_line(content, &var_needle(fmt, name)).unwrap_or(0);
        for var in var_refs(value) {
            if !defined.contains(var.as_str()) {
                out.push(Problem::at(
                    line,
                    0,
                    format!("variable '{}' references undefined $({})", name, var),
                    Some("define it under [vars] before it is used".to_string()),
                ));
            }
        }
    }

    for name in ranfile.targets.keys() {
        let mut visited = HashSet::new();
        if let Some(cycle) = find_cycle(name, ranfile, &mut visited) {
            let line = find_line(content, &header_needle(fmt, &cycle)).unwrap_or(0);
            out.push(Problem::at(
                line,
                0,
                format!("circular dependency: {} depends on itself", cycle),
                Some("remove one of the deps that closes the loop".to_string()),
            ));
        }
    }

    out
}

/// First cycle reachable from `target`, if any.
fn find_cycle(target: &str, ranfile: &Ranfile, visited: &mut HashSet<String>) -> Option<String> {
    if !visited.insert(target.to_string()) {
        return Some(target.to_string());
    }
    if let Some(t) = ranfile.targets.get(target) {
        for dep in &t.deps {
            if let Some(found) = find_cycle(dep, ranfile, visited) {
                return Some(found);
            }
        }
    }
    visited.remove(target);
    None
}

/// `$(NAME)` references in a string, in order of appearance.
fn var_refs(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i] == '$' && chars[i + 1] == '(' {
            let mut name = String::new();
            let mut j = i + 2;
            let mut closed = false;
            while j < chars.len() {
                if chars[j] == ')' {
                    closed = true;
                    break;
                }
                name.push(chars[j]);
                j += 1;
            }
            if closed {
                if !name.is_empty() {
                    out.push(name);
                }
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// 1-based `(line, column)` of a byte offset.
fn line_col(content: &str, offset: usize) -> (usize, usize) {
    let offset = offset.min(content.len());
    let before = &content[..offset];
    let line = before.matches('\n').count() + 1;
    let col = before
        .rsplit('\n')
        .next()
        .map(|s| s.chars().count())
        .unwrap_or(0)
        + 1;
    (line, col)
}

/// 1-based line number of the first line containing `needle`.
fn find_line(content: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    content
        .lines()
        .position(|line| line.contains(needle))
        .map(|idx| idx + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn messages(content: &str) -> Vec<String> {
        check_text(content)
            .into_iter()
            .map(|p| format!("{}:{} {}", p.line, p.col, p.message))
            .collect()
    }

    const GOOD: &str = r#"
[vars]
ver = "1.0"
arc = "app-$(ver).zip"

[targets.standalone]
deps = []
cmd = "echo $(arc)"
"#;

    #[test]
    fn clean_file_has_no_problems() {
        assert!(check_text(GOOD).is_empty(), "{:?}", messages(GOOD));
    }

    #[test]
    fn single_backslash_is_flagged_with_the_fixed_line() {
        let content = "[targets.standalone]\ndeps = []\ncmd = \"packaging\\build.cmd\"\n";
        let problems = check_text(content);
        let escape = problems
            .iter()
            .find(|p| p.message.contains("'\\b'"))
            .expect("escape problem");
        assert_eq!(escape.line, 3);
        let hint = escape.hint.clone().unwrap_or_default();
        assert!(hint.contains("packaging\\\\build.cmd"), "{hint}");
    }

    #[test]
    fn invalid_escape_is_reported_as_an_error() {
        let content = "[targets.standalone]\ncmd = \"C:\\Program Files\\x\"\n";
        let all = messages(content).join("\n");
        assert!(all.contains("invalid escape '\\P'"), "{all}");
    }

    #[test]
    fn raw_control_character_is_reported() {
        // What `printf 'cmd = "a\bb"'` (or a shell heredoc) leaves behind.
        let content = "[targets.standalone]\ncmd = \"a\u{8}b\"\n";
        let all = messages(content).join("\n");
        assert!(all.contains("raw control character U+0008"), "{all}");
    }

    #[test]
    fn doubled_backslash_is_accepted() {
        let content = "[targets.standalone]\ndeps = []\ncmd = \"packaging\\\\build.cmd\"\n";
        assert!(check_text(content).is_empty(), "{:?}", messages(content));
    }

    #[test]
    fn backslash_in_a_literal_string_is_not_an_escape() {
        let content = "[targets.standalone]\ndeps = []\ncmd = 'packaging\\build.cmd'\n";
        assert!(check_text(content).is_empty(), "{:?}", messages(content));
    }

    #[test]
    fn stray_text_is_reported_at_its_line() {
        let content = "[targets.standalone]\ndeps = []\ncmd = \"echo hi\"\noops stray words\n";
        let problems = check_text(content);
        let stray = problems
            .iter()
            .find(|p| p.message.contains("stray text"))
            .expect("stray text problem");
        assert_eq!(stray.line, 4);
        assert!(stray.hint.is_some());
    }

    #[test]
    fn arrays_spanning_lines_are_not_stray_text() {
        let content = "[targets.all]\ndeps = [\n  \"a\",\n  \"b\"\n]\ncmd = \"\"\n\n[targets.a]\ncmd = \"x\"\n\n[targets.b]\ncmd = \"y\"\n";
        let stray: Vec<String> = messages(content)
            .into_iter()
            .filter(|m| m.contains("stray text"))
            .collect();
        assert!(stray.is_empty(), "{stray:?}");
    }

    #[test]
    fn missing_dependency_and_cycle_are_reported() {
        let content = "[targets.a]\ndeps = [\"b\"]\ncmd = \"x\"\n\n[targets.b]\ndeps = [\"a\"]\ncmd = \"y\"\n";
        let all = messages(content).join("\n");
        assert!(all.contains("circular dependency"), "{all}");

        let missing = "[targets.a]\ndeps = [\"nope\"]\ncmd = \"x\"\n";
        let all = messages(missing).join("\n");
        assert!(all.contains("'nope', which is not defined"), "{all}");
    }

    #[test]
    fn reserved_target_name_is_reported() {
        let content = "[targets.check]\ncmd = \"echo hi\"\n";
        let all = messages(content).join("\n");
        assert!(all.contains("reserved"), "{all}");
    }

    #[test]
    fn undefined_variable_is_reported() {
        let content = "[vars]\nver = \"1.0\"\n\n[targets.a]\ncmd = \"echo $(nope) $(ver)\"\n";
        let all = messages(content).join("\n");
        assert!(all.contains("$(nope) is not defined"), "{all}");
        assert!(!all.contains("$(ver) is not defined"), "{all}");
    }

    #[test]
    fn broken_json_and_yaml_report_their_parser_position() {
        let json = check_text_as("{\n  \"targets\": {,}\n}", RanFormat::Json);
        assert!(
            json.iter().any(|p| p.message.starts_with("invalid JSON")),
            "{:?}",
            messages_of(json)
        );

        let yaml = check_text_as("targets: [", RanFormat::Yaml);
        assert!(
            yaml.iter().any(|p| p.message.starts_with("invalid YAML")),
            "{:?}",
            messages_of(yaml)
        );
    }

    #[test]
    fn json_and_yaml_still_get_the_structural_checks() {
        let json = r#"{"vars": {}, "targets": {"a": {"deps": ["nope"], "cmd": "x"}}}"#;
        let problems = messages_of(check_text_as(json, RanFormat::Json));
        assert!(
            problems.iter().any(|m| m.contains("which is not defined")),
            "{problems:?}"
        );

        let yaml = "vars: {}\ntargets:\n  check:\n    deps: []\n    cmd: \"x\"\n";
        let problems = messages_of(check_text_as(yaml, RanFormat::Yaml));
        assert!(
            problems.iter().any(|m| m.contains("reserved")),
            "{problems:?}"
        );
    }

    fn messages_of(problems: Vec<Problem>) -> Vec<String> {
        problems
            .into_iter()
            .map(|p| format!("{}:{} {}", p.line, p.col, p.message))
            .collect()
    }

    #[test]
    fn var_refs_finds_each_reference() {
        assert_eq!(var_refs("$(a) x $(b)"), vec!["a", "b"]);
        assert_eq!(var_refs("$(unclosed"), Vec::<String>::new());
        assert_eq!(var_refs("plain"), Vec::<String>::new());
    }
}
