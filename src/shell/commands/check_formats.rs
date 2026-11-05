//! Validators behind the `<cmd> check` subcommands.
//!
//! Each format is interpreted the way its command will read it, and every
//! problem is returned with the line that has to change. The reporters live
//! in [`super::check_common`]; this module only produces diagnostics, which
//! keeps the validators unit-testable without touching stdout.

use super::check_common::{find_line, line_col, Diagnostic};
use std::collections::{HashMap, HashSet};

// ============================================================================
// Shared TOML helpers
// ============================================================================

/// Turn a `toml` parse error into a diagnostic with the broken position and a
/// hint naming the keys the file actually accepts.
fn schema_error(content: &str, err: toml::de::Error, expected: &str) -> Diagnostic {
    let (line, col) = err
        .span()
        .map(|span| line_col(content, span.start))
        .unwrap_or((0, 0));
    let message = format!("invalid TOML: {}", err.message().trim());
    let hint = if message.contains("unknown field") || message.contains("unknown key") {
        Some(format!("this file only takes {}", expected))
    } else if message.contains("expected") || message.contains("unexpected") {
        Some(format!("expected {}", expected))
    } else if message.contains("duplicate") {
        Some("each key may only appear once per table".to_string())
    } else if message.contains("escape") {
        Some(
            "a literal backslash is written as \\\\ inside a \"…\" string, or use 'single quotes'"
                .to_string(),
        )
    } else {
        None
    };
    Diagnostic::at(line, col, message, hint)
}

// ============================================================================
// ral — run aliases (`*.ntc.ral`)
// ============================================================================

/// `run_aliases` / `run_alias_hints` / `run_hint_lists`, matching the schema
/// `ral import` deserializes. Unknown tables are rejected so a typo shows up
/// here instead of silently importing nothing.
pub(crate) fn ral(content: &str) -> Vec<Diagnostic> {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct RalFile {
        run_aliases: Option<HashMap<String, String>>,
        run_alias_hints: Option<HashMap<String, String>>,
        run_hint_lists: Option<HashMap<String, Vec<String>>>,
    }

    let parsed: RalFile = match toml::from_str(content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return vec![schema_error(
                content,
                err,
                "the [run_aliases], [run_alias_hints] and [run_hint_lists] tables",
            )]
        }
    };

    let mut out = Vec::new();
    let aliases = parsed.run_aliases.unwrap_or_default();

    if aliases.is_empty() {
        out.push(Diagnostic::line(
            find_line(content, "run_aliases").unwrap_or(0),
            "no [run_aliases] entries to import",
            Some("add a [run_aliases] table with `name = \"command\"` lines".to_string()),
        ));
    }

    let mut ordered: Vec<(&String, &String)> = aliases.iter().collect();
    ordered.sort_by_key(|(name, _)| name.to_lowercase());

    let mut seen: HashMap<String, String> = HashMap::new();
    for (name, command) in ordered {
        let line = alias_line(content, name);
        out.extend(alias_problems(name, command, line));

        let lower = name.to_lowercase();
        if let Some(previous) = seen.insert(lower, name.clone()) {
            if previous != *name {
                out.push(Diagnostic::line(
                    line,
                    format!(
                        "'{}' and '{}' differ only by case, so one would overwrite the other",
                        previous, name
                    ),
                    Some("aliases are stored lowercased — keep a single spelling".to_string()),
                ));
            }
        }
    }

    let hints = parsed.run_alias_hints.unwrap_or_default();
    for name in hints.keys() {
        if !aliases.keys().any(|a| a.eq_ignore_ascii_case(name)) {
            out.push(Diagnostic::line(
                hint_line(content, name),
                format!("hint spec for '{}' but no such alias", name),
                Some("add the alias under [run_aliases] or drop the hint".to_string()),
            ));
        }
    }

    let lists = parsed.run_hint_lists.unwrap_or_default();
    for (name, values) in &lists {
        let line = hint_line(content, name);
        if values.is_empty() {
            out.push(Diagnostic::line(
                line,
                format!("hint list '{}' is empty", name),
                Some("list at least one value, or remove the list".to_string()),
            ));
        }
        let mut seen_values = HashSet::new();
        for value in values {
            if value.trim().is_empty() {
                out.push(Diagnostic::line(
                    line,
                    format!("hint list '{}' has an empty value", name),
                    None,
                ));
            } else if !seen_values.insert(value.to_lowercase()) {
                out.push(Diagnostic::line(
                    line,
                    format!("duplicate value '{}' in hint list '{}'", value, name),
                    None,
                ));
            }
        }
    }

    out.sort_by_key(|d| (d.line, d.col));
    out
}

/// Problems with one alias name/command pair.
fn alias_problems(name: &str, command: &str, line: usize) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    if !crate::config::validate_alias_name(name) {
        out.push(Diagnostic::line(
            line,
            format!("'{}' is not a valid alias name", name),
            Some(
                "names are lowercase letters, digits, '_' and '-', and cannot be a reserved word"
                    .to_string(),
            ),
        ));
    }

    if command.trim().is_empty() {
        out.push(Diagnostic::line(
            line,
            format!("alias '{}' has an empty command", name),
            Some("give it the command line to run".to_string()),
        ));
        return out;
    }

    if let Some((display, separator)) = crate::shell::alias::find_invalid_array_sep(command) {
        out.push(Diagnostic::line(
            line,
            format!(
                "array separator in {} must be one character, not '{}'",
                display, separator
            ),
            Some(format!(
                "write {}…{} with a single separator character",
                display, display
            )),
        ));
    }

    if let Some(col) = unclosed_placeholder(command) {
        out.push(Diagnostic::at(
            line,
            col,
            "unclosed '${' placeholder",
            Some("close it with '}'".to_string()),
        ));
    }

    let bare = crate::shell::alias::bare_env_refs(command);
    if !bare.is_empty() {
        out.push(Diagnostic::line(
            line,
            format!(
                "{} reads environment variables at run time, not alias parameters",
                bare.iter()
                    .map(|n| format!("${}", n))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Some("write ${NAME} for an alias parameter, or keep $NAME for the env var".to_string()),
        ));
    }

    out
}

/// Line where an alias is defined (accepts both TOML quote styles).
fn alias_line(content: &str, name: &str) -> usize {
    find_line(content, &format!("\"{}\"", name))
        .or_else(|| find_line(content, &format!("'{}'", name)))
        .or_else(|| find_line(content, name))
        .unwrap_or(0)
}

/// Line where a hint/list entry is defined.
fn hint_line(content: &str, name: &str) -> usize {
    alias_line(content, name)
}

/// Byte offset of a `${` that never closes.
fn unclosed_placeholder(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' && bytes[i + 1] == b'{' {
            if !s[i + 2..].contains('}') {
                return Some(i + 1);
            }
        }
        i += 1;
    }
    None
}

// ============================================================================
// math — one expression per line (`*.ntc.math`)
// ============================================================================

pub(crate) fn math(content: &str) -> Vec<Diagnostic> {
    crate::math::check_text(content)
        .into_iter()
        .map(|(line, col, message)| Diagnostic::at(line, col, message, None))
        .collect()
}

// ============================================================================
// scr — `*.ntc` automation scripts
// ============================================================================

pub(crate) fn script(content: &str) -> Vec<Diagnostic> {
    crate::script::check::check_text(content)
}

// ============================================================================
// ran — `NTCRANFILE.toml`
// ============================================================================

pub(crate) fn ran(content: &str) -> Vec<Diagnostic> {
    crate::ranfile::check::check_text(content)
        .into_iter()
        .map(|problem| Diagnostic::at(problem.line, problem.col, problem.message, problem.hint))
        .collect()
}

/// `ran check` on an `NTCRANFILE.json`.
pub(crate) fn ran_json(content: &str) -> Vec<Diagnostic> {
    ran_as(content, crate::ranfile::types::RanFormat::Json)
}

/// `ran check` on an `NTCRANFILE.yaml`.
pub(crate) fn ran_yaml(content: &str) -> Vec<Diagnostic> {
    ran_as(content, crate::ranfile::types::RanFormat::Yaml)
}

fn ran_as(content: &str, fmt: crate::ranfile::types::RanFormat) -> Vec<Diagnostic> {
    crate::ranfile::check::check_text_as(content, fmt)
        .into_iter()
        .map(|problem| Diagnostic::at(problem.line, problem.col, problem.message, problem.hint))
        .collect()
}

// ============================================================================
// igcare — `*.ntc.igcare`
// ============================================================================

pub(crate) fn igcare(content: &str) -> Vec<Diagnostic> {
    #[derive(serde::Deserialize, Default)]
    #[serde(deny_unknown_fields)]
    struct IgcareFile {
        ignored_directory_names: Option<Vec<String>>,
        ignored_extensions: Option<Vec<String>>,
        extra_supported_extensions: Option<Vec<String>>,
        ignored_files: Option<Vec<String>>,
        extra_supported_files: Option<Vec<String>>,
        cared_directory_names: Option<Vec<String>>,
        cared_extensions: Option<Vec<String>>,
        cared_files: Option<Vec<String>>,
    }

    let parsed: IgcareFile = match toml::from_str(content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return vec![schema_error(
                content,
                err,
                "ignored_*, extra_supported_* and cared_* string arrays",
            )]
        }
    };

    // (key, values, is_extension)
    let lists: [(&str, &Option<Vec<String>>, bool); 8] = [
        (
            "ignored_directory_names",
            &parsed.ignored_directory_names,
            false,
        ),
        ("ignored_extensions", &parsed.ignored_extensions, true),
        (
            "extra_supported_extensions",
            &parsed.extra_supported_extensions,
            true,
        ),
        ("ignored_files", &parsed.ignored_files, false),
        (
            "extra_supported_files",
            &parsed.extra_supported_files,
            false,
        ),
        (
            "cared_directory_names",
            &parsed.cared_directory_names,
            false,
        ),
        ("cared_extensions", &parsed.cared_extensions, true),
        ("cared_files", &parsed.cared_files, false),
    ];

    let mut out = Vec::new();
    let mut any_value = false;

    for (key, values, is_extension) in lists {
        let Some(values) = values else { continue };
        let key_line = find_line(content, key).unwrap_or(0);
        any_value |= !values.is_empty();

        let mut seen = HashSet::new();
        for item in values {
            let line = find_line(content, &format!("\"{}\"", item))
                .or_else(|| find_line(content, &format!("\"{}\"", item.to_lowercase())))
                .unwrap_or(key_line);
            let trimmed = item.trim();

            if trimmed.is_empty() {
                out.push(Diagnostic::line(
                    line,
                    format!("empty entry in {}", key),
                    Some("remove it or give it a value".to_string()),
                ));
                continue;
            }
            if is_extension && trimmed.starts_with('.') {
                out.push(Diagnostic::line(
                    line,
                    format!("extension '{}' is written with a leading dot", trimmed),
                    Some(format!("write \"{}\"", trimmed.trim_start_matches('.'))),
                ));
            }
            if is_extension && !trimmed.starts_with('.') && trimmed.contains('.') {
                out.push(Diagnostic::line(
                    line,
                    format!(
                        "'{}' contains a dot, so it never matches an extension",
                        trimmed
                    ),
                    Some("the matcher compares the part after the last dot".to_string()),
                ));
            }
            if !is_extension && (trimmed.contains('/') || trimmed.contains('\\')) {
                out.push(Diagnostic::line(
                    line,
                    format!("'{}' looks like a path", trimmed),
                    Some("these lists match a single name, not a path".to_string()),
                ));
            }
            if !seen.insert(trimmed.to_lowercase()) {
                out.push(Diagnostic::line(
                    line,
                    format!("duplicate entry '{}' in {}", trimmed, key),
                    None,
                ));
            }
        }
    }

    if !any_value {
        out.push(Diagnostic::line(
            0,
            "this file sets no ignore/care values",
            Some(
                "fill at least one of the ignored_*, extra_supported_* or cared_* lists"
                    .to_string(),
            ),
        ));
    }

    out.sort_by_key(|d| (d.line, d.col));
    out
}

// ============================================================================
// theme — `*.ntc_theme`
// ============================================================================

pub(crate) fn theme(content: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    match toml::from_str::<crate::utils::theme::Theme>(content) {
        Ok(theme) => {
            if theme.name.trim().is_empty() {
                out.push(Diagnostic::line(
                    find_line(content, "name").unwrap_or(0),
                    "the theme has no name",
                    Some("add name = \"my-theme\"".to_string()),
                ));
            }
        }
        Err(err) => {
            out.push(schema_error(
                content,
                err,
                "name, author, description, [syntax.*], [shell.*] and [editor.*] colour keys",
            ));
        }
    }

    // `Theme` ignores unknown top-level keys; a typo there silently does
    // nothing, so point it out.
    if let Ok(value) = content.parse::<toml::Value>() {
        if let Some(table) = value.as_table() {
            const ALLOWED: &[&str] =
                &["name", "author", "description", "syntax", "shell", "editor"];
            for key in table.keys() {
                if !ALLOWED.contains(&key.as_str()) {
                    out.push(Diagnostic::line(
                        find_line(content, key).unwrap_or(0),
                        format!("unknown theme key '{}'", key),
                        Some(format!("expected one of: {}", ALLOWED.join(", "))),
                    ));
                }
            }
        }
    }

    out.sort_by_key(|d| (d.line, d.col));
    out
}

// ============================================================================
// fake — `*.ntc_fake` bundles (TOML, or JSON for `*.json`)
// ============================================================================

pub(crate) fn fake(content: &str) -> Vec<Diagnostic> {
    #[derive(serde::Deserialize, Default)]
    struct FakeFile {
        #[serde(default)]
        version: Option<u32>,
        #[serde(default)]
        groups: HashMap<String, Vec<crate::config::FakeColumn>>,
    }

    let mut out = Vec::new();

    if let Some(version) = &toml_or_json_version(content) {
        if *version == 0 {
            out.push(Diagnostic::line(
                0,
                "bundle version 0 is not readable",
                None,
            ));
        }
    }

    let parsed: FakeFile = if content.trim_start().starts_with('{') {
        match serde_json::from_str(content) {
            Ok(parsed) => parsed,
            Err(err) => {
                return vec![Diagnostic::at(
                    err.line(),
                    err.column(),
                    format!("invalid JSON: {}", err),
                    None,
                )]
            }
        }
    } else {
        match toml::from_str(content) {
            Ok(parsed) => parsed,
            Err(err) => {
                return vec![schema_error(
                    content,
                    err,
                    "version = 1 and [groups.<name>] column tables",
                )]
            }
        }
    };

    if let Some(version) = parsed.version {
        if version != 1 {
            out.push(Diagnostic::line(
                find_line(content, "version").unwrap_or(0),
                format!("unknown bundle version {}", version),
                Some("this build reads version 1 bundles".to_string()),
            ));
        }
    }

    if parsed.groups.is_empty() {
        out.push(Diagnostic::line(
            0,
            "the bundle contains no groups",
            Some("add [groups.<name>] with one `column as type` entry per field".to_string()),
        ));
    }

    for (group, columns) in &parsed.groups {
        let header = find_line(content, group).unwrap_or(0);
        if group.trim().is_empty() {
            out.push(Diagnostic::line(header, "group name is empty", None));
        } else if group.chars().any(char::is_whitespace) {
            out.push(Diagnostic::line(
                header,
                format!("group '{}' contains whitespace", group),
                Some("use letters, digits, '_' and '-'".to_string()),
            ));
        }

        if columns.is_empty() {
            out.push(Diagnostic::line(
                header,
                format!("group '{}' has no columns", group),
                Some("add at least one `alias as type` column".to_string()),
            ));
            continue;
        }

        let mut seen = HashSet::new();
        for column in columns {
            let line = find_line(content, &column.alias).unwrap_or(header);
            if column.alias.trim().is_empty() {
                out.push(Diagnostic::line(line, "column alias is empty", None));
            } else if !seen.insert(column.alias.to_lowercase()) {
                out.push(Diagnostic::line(
                    line,
                    format!("duplicate column '{}' in group '{}'", column.alias, group),
                    Some("give each column a distinct alias".to_string()),
                ));
            }

            let type_name = column.type_name.trim();
            // Point at the `type` line, not at the alias that happens to
            // repeat the same text.
            let type_line = find_line(content, type_name).unwrap_or(line);
            if let Some(rest) = type_name.strip_prefix("ref:") {
                match rest.split_once('.') {
                    Some((target, field))
                        if !target.trim().is_empty() && !field.trim().is_empty() =>
                    {
                        if !parsed
                            .groups
                            .keys()
                            .any(|g| g.eq_ignore_ascii_case(target.trim()))
                        {
                            out.push(Diagnostic::line(
                                type_line,
                                format!(
                                    "ref target group '{}' is not in this bundle",
                                    target.trim()
                                ),
                                Some(
                                    "export the referenced group in the same file, or create it first"
                                        .to_string(),
                                ),
                            ));
                        }
                    }
                    _ => out.push(Diagnostic::line(
                        type_line,
                        format!("bad reference '{}'", type_name),
                        Some("write ref:<group>.<column>".to_string()),
                    )),
                }
            } else if !crate::fake::types::TYPES
                .iter()
                .any(|t| t.name.eq_ignore_ascii_case(type_name))
            {
                out.push(Diagnostic::line(
                    type_line,
                    format!("unknown fake type '{}'", type_name),
                    Some("run `fake types -s <text>` to search the type list".to_string()),
                ));
            }
        }
    }

    out.sort_by_key(|d| (d.line, d.col));
    out
}

/// `version` from either encoding, when the file parses far enough to have one.
fn toml_or_json_version(content: &str) -> Option<u32> {
    if content.trim_start().starts_with('{') {
        return serde_json::from_str::<serde_json::Value>(content)
            .ok()
            .and_then(|v| v.get("version").and_then(|v| v.as_u64()))
            .map(|v| v as u32);
    }
    content
        .lines()
        .find_map(|line| line.trim().strip_prefix("version"))
        .and_then(|rest| rest.trim_start().strip_prefix('='))
        .and_then(|rest| rest.trim().split_whitespace().next())
        .and_then(|v| v.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn messages(diags: &[Diagnostic]) -> String {
        diags
            .iter()
            .map(|d| format!("{}:{} {}", d.line, d.col, d.message))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // ---- ral ----

    const RAL_OK: &str = "[run_aliases]\nbuild = \"cargo build\"\ntest = \"cargo test\"\n";

    #[test]
    fn ral_clean_file_has_no_problems() {
        assert!(ral(RAL_OK).is_empty(), "{}", messages(&ral(RAL_OK)));
    }

    #[test]
    fn ral_bare_env_ref_is_flagged() {
        // A bare lowercase `$name` is an environment variable at run time;
        // `ral add` warns about exactly these.
        let content = "[run_aliases]\nproj = \"cd $proj\"\n";
        let all = messages(&ral(content));
        assert!(all.contains("environment variables"), "{all}");
    }

    #[test]
    fn ral_braced_param_is_accepted() {
        let content = "[run_aliases]\nproj = \"cd ${proj}\"\n";
        let all = messages(&ral(content));
        assert!(!all.contains("environment variables"), "{all}");
    }

    #[test]
    fn ral_bad_array_separator_is_reported() {
        let content = "[run_aliases]\nrunc = \"gcc $x[ab].c\"\n";
        let all = messages(&ral(content));
        assert!(all.contains("array separator"), "{all}");
    }

    #[test]
    fn ral_unknown_table_is_reported() {
        let content = "[run_alias]\nbuild = \"x\"\n";
        let all = messages(&ral(content));
        assert!(all.contains("invalid TOML"), "{all}");
    }

    #[test]
    fn ral_hint_for_missing_alias_is_reported() {
        let content =
            "[run_aliases]\nbuild = \"cargo build\"\n\n[run_alias_hints]\nnope = \"#x\"\n";
        let all = messages(&ral(content));
        assert!(all.contains("hint spec for 'nope'"), "{all}");
    }

    #[test]
    fn ral_unclosed_placeholder_is_reported() {
        let content = "[run_aliases]\nbuild = \"echo ${x\"\n";
        let all = messages(&ral(content));
        assert!(all.contains("unclosed '${'"), "{all}");
    }

    // ---- igcare ----

    #[test]
    fn igcare_accepts_canonical_file() {
        let content =
            "ignored_extensions = [\"rs\", \"toml\"]\ncared_directory_names = [\"src\"]\n";
        assert!(igcare(content).is_empty(), "{}", messages(&igcare(content)));
    }

    #[test]
    fn igcare_flags_a_leading_dot_extension() {
        let content = "ignored_extensions = [\".rs\"]\n";
        let all = messages(&igcare(content));
        assert!(all.contains("leading dot"), "{all}");
    }

    #[test]
    fn igcare_flags_a_path_in_a_name_list() {
        let content = "ignored_files = [\"build/out\"]\n";
        let all = messages(&igcare(content));
        assert!(all.contains("looks like a path"), "{all}");
    }

    #[test]
    fn igcare_flags_unknown_keys() {
        let content = "ignored_extension = [\"rs\"]\n";
        let all = messages(&igcare(content));
        assert!(all.contains("invalid TOML"), "{all}");
    }

    // ---- theme ----

    #[test]
    fn theme_clean_file_has_no_problems() {
        let content = "name = \"mine\"\n\n[syntax]\nkeyword = { r = 1, g = 2, b = 3 }\n";
        assert!(theme(content).is_empty(), "{}", messages(&theme(content)));
    }

    #[test]
    fn theme_flags_unknown_top_level_key() {
        let content = "name = \"mine\"\ncolour = 1\n";
        let all = messages(&theme(content));
        assert!(all.contains("unknown theme key 'colour'"), "{all}");
    }

    #[test]
    fn theme_reports_bad_colour_value() {
        let content = "name = \"mine\"\n\n[syntax]\nkeyword = \"nope\"\n";
        let all = messages(&theme(content));
        assert!(all.contains("invalid TOML"), "{all}");
    }

    // ---- fake ----

    const FAKE_OK: &str = r#"
version = 1

[groups]
users = []
"#;

    #[test]
    fn fake_flags_empty_group() {
        let all = messages(&fake(FAKE_OK));
        assert!(all.contains("has no columns"), "{all}");
    }

    #[test]
    fn fake_accepts_a_full_bundle() {
        let content = r#"
version = 1

[[groups.users]]
alias = "email"
type = "email"

[[groups.users]]
alias = "name"
type = "name"
"#;
        assert!(fake(content).is_empty(), "{}", messages(&fake(content)));
    }

    #[test]
    fn fake_flags_unknown_type() {
        let content = "version = 1\n\n[[groups.users]]\nalias = \"x\"\ntype = \"not_a_type\"\n";
        let all = messages(&fake(content));
        assert!(all.contains("unknown fake type"), "{all}");
    }

    #[test]
    fn fake_flags_bad_reference() {
        let content = "version = 1\n\n[[groups.users]]\nalias = \"x\"\ntype = \"ref:nope\"\n";
        let all = messages(&fake(content));
        assert!(all.contains("bad reference"), "{all}");
    }

    #[test]
    fn fake_accepts_json_bundle() {
        let content = r#"{"version":1,"groups":{"users":[{"alias":"email","type":"email"}]}}"#;
        assert!(fake(content).is_empty(), "{}", messages(&fake(content)));
    }

    #[test]
    fn fake_reports_json_syntax_error() {
        let all = messages(&fake("{\"version\":1,"));
        assert!(all.contains("invalid JSON"), "{all}");
    }

    // ---- math / scr ----

    #[test]
    fn math_reports_the_broken_line() {
        let all = messages(&math("1+1\n1+\n"));
        assert!(all.contains("2:"), "{all}");
    }

    #[test]
    fn math_skips_comments_and_blank_lines() {
        assert!(math("# hi\n\n1+1\n// also ok\n").is_empty());
    }

    #[test]
    fn script_reports_unterminated_quote() {
        let all = messages(&script("say \"hello\n"));
        assert!(all.contains("unterminated"), "{all}");
    }

    // ---- ran ----

    #[test]
    fn ran_reports_problems_through_the_shared_type() {
        let content = "[targets.check]\ncmd = \"echo hi\"\n";
        let diags = ran(content);
        assert!(!diags.is_empty());
        assert!(diags.iter().any(|d| d.line == 1));
    }
}
