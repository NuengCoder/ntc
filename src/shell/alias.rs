use crate::config::Config;
use std::collections::HashMap;

// ============================================================================
// Helper functions for alias expansion and pipeline processing
// ============================================================================

/// Split a line on `&&` that are not inside double quotes.
fn split_on_ampersands(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '"' => {
                current.push('"');
                in_quotes = !in_quotes;
                i += 1;
            }
            '&' if !in_quotes && i + 1 < chars.len() && chars[i + 1] == '&' => {
                if !current.trim().is_empty() {
                    result.push(current.trim().to_string());
                }
                current.clear();
                i += 2;
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                }
                continue;
            }
            _ => {
                current.push(chars[i]);
                i += 1;
            }
        }
    }
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}

/// Check if a string contains && outside quotes
pub(super) fn contains_ampersands(s: &str) -> bool {
    split_on_ampersands(s).len() > 1
}

/// Parse a call token like `py(hello)` into `("py", Some("hello"))`.
/// Returns `(token, None)` if there are no parentheses.
/// Strips trailing whitespace from the base name to handle `py (hello)`.
/// Shared with the `ndb` shell (`dbal run name(args)`).
pub(crate) fn parse_call_syntax(token: &str) -> (&str, Option<&str>) {
    if let Some(paren_start) = token.find('(') {
        if token.ends_with(')') {
            let base = token[..paren_start].trim_end();
            let arg = &token[paren_start + 1..token.len() - 1];
            return (base, Some(arg));
        }
    }
    (token, None)
}

/// Tokenize a string on whitespace, respecting `[...]` as single tokens.
/// e.g. `"[add,minus,mul] math"` → `["[add,minus,mul]", "math"]`
fn tokenize_args(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_bracket = false;
    for ch in input.chars() {
        match ch {
            '[' if !in_bracket => {
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                }
                current.clear();
                current.push(ch);
                in_bracket = true;
            }
            ']' if in_bracket => {
                current.push(ch);
                in_bracket = false;
                tokens.push(current.trim().to_string());
                current.clear();
            }
            ' ' | '\t' if !in_bracket => {
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    tokens
}

/// Parsed variable reference.
struct VarRef {
    name: String,
    end: usize,
    array_suffix: Option<String>,
    /// `Some(c)` for `$x[c]` / `${x}[c]` (single-char join separator).
    array_sep: Option<char>,
    default_val: Option<String>,
}

/// Read an array suffix starting at `from` (first char after `]`):
/// runs until whitespace, `&` or `;`.
fn read_array_suffix(chars: &[char], from: usize) -> (String, usize) {
    let mut end = from;
    while end < chars.len()
        && !chars[end].is_whitespace()
        && chars[end] != '&'
        && chars[end] != ';'
    {
        end += 1;
    }
    (chars[from..end].iter().collect(), end)
}

/// Parse a variable reference starting at `chars[i]` where `chars[i] == '$'`.
/// Returns `Some(VarRef)` on success.
/// Supports:
///   - `$name`          (name: [a-z][a-zA-Z0-9_]*)
///   - `${name}`        (braced)
///   - `${name:-val}`   (braced with default)
///   - `$name[]suffix`  / `${name}[]suffix`  (array expansion)
fn parse_var_ref(chars: &[char], mut i: usize) -> Option<VarRef> {
    // chars[i] is known to be '$'
    i += 1;
    let (name, end, default_val) = if i < chars.len() && chars[i] == '{' {
        // ${name} or ${name:-default} syntax
        i += 1;
        let start = i;
        let mut colon_pos = None;
        while i < chars.len() && chars[i] != '}' {
            if chars[i] == ':' && i + 1 < chars.len() && chars[i + 1] == '-' && colon_pos.is_none()
            {
                colon_pos = Some(i);
            }
            i += 1;
        }
        if i <= start || i >= chars.len() {
            return None;
        }
        let name: String = if let Some(cp) = colon_pos {
            chars[start..cp].iter().collect()
        } else {
            chars[start..i].iter().collect()
        };
        let default_val = colon_pos.map(|cp| {
            let val_start = cp + 2; // skip ':-'
            chars[val_start..i].iter().collect::<String>()
        });
        i += 1; // skip '}'
        (name, i, default_val)
    } else {
        // $name syntax — first char must be [a-z0-9_]
        if i >= chars.len()
            || !(chars[i].is_ascii_lowercase() || chars[i].is_ascii_digit() || chars[i] == '_')
        {
            return None;
        }
        let start = i;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
            i += 1;
        }
        let name: String = chars[start..i].iter().collect();
        (name, i, None)
    };

    // Check for array expansion: `[]suffix` (legacy space join) or
    // `[c]suffix` (single-char separator join). Anything else in the
    // brackets (e.g. `$x[ab]`) is NOT array syntax — fall through to a
    // plain reference so the text survives literally (`ral add`/`edit`
    // reject such templates up front with a helpful error).
    if end < chars.len() && chars[end] == '[' {
        let mut close = end + 1;
        while close < chars.len() && chars[close] != ']' {
            close += 1;
        }
        if close < chars.len() {
            let content: String = chars[end + 1..close].iter().collect();
            if content.is_empty() {
                let (suffix, suffix_end) = read_array_suffix(chars, close + 1);
                return Some(VarRef {
                    name,
                    end: suffix_end,
                    array_suffix: Some(suffix),
                    array_sep: None,
                    default_val,
                });
            } else if content.chars().count() == 1 {
                let sep = content.chars().next().unwrap();
                let (suffix, suffix_end) = read_array_suffix(chars, close + 1);
                return Some(VarRef {
                    name,
                    end: suffix_end,
                    array_suffix: Some(suffix),
                    array_sep: Some(sep),
                    default_val,
                });
            }
        }
    }
    Some(VarRef {
        name,
        end,
        array_suffix: None,
        array_sep: None,
        default_val,
    })
}

/// Scan a template for multi-char array brackets (`$x[ab]`, `${o}[ab]`)
/// that array syntax rejects. Returns `(display, content)`, e.g.
/// `("$x[ab]", "ab")`, for the first offense. `ral add`/`edit` refuse
/// such templates; expansion itself leaves them literal.
pub(crate) fn find_invalid_array_sep(template: &str) -> Option<(String, String)> {
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '$' {
            i += 1;
            continue;
        }
        let start = i;
        i += 1;
        // Skip the name part exactly like `parse_var_ref`.
        if i < chars.len() && chars[i] == '{' {
            i += 1;
            let mut colon_pos = None;
            while i < chars.len() && chars[i] != '}' {
                if chars[i] == ':' && i + 1 < chars.len() && chars[i + 1] == '-' && colon_pos.is_none() {
                    colon_pos = Some(i);
                }
                i += 1;
            }
            if i >= chars.len() {
                // Unclosed `${`: skip past `$` and keep scanning.
                i = start + 1;
                continue;
            }
            i += 1; // skip '}'
        } else {
            if i >= chars.len()
                || !(chars[i].is_ascii_lowercase() || chars[i].is_ascii_digit() || chars[i] == '_')
            {
                i = start + 1;
                continue;
            }
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
        }
        if i < chars.len() && chars[i] == '[' {
            let mut close = i + 1;
            while close < chars.len() && chars[close] != ']' {
                close += 1;
            }
            if close < chars.len() {
                let content: String = chars[i + 1..close].iter().collect();
                if content.chars().count() > 1 {
                    let display: String = chars[start..=close].iter().collect();
                    return Some((display, content));
                }
                i = close + 1;
                continue;
            }
        }
    }
    None
}

/// Shared with the `ndb` shell (`dbal` aliases reuse ral-style params).
pub(crate) fn substitute_params(template: &str, args: &[&str]) -> String {
    // 1. Collect placeholder names in order of first appearance.
    let mut param_order: Vec<String> = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some(ref vr) = parse_var_ref(&chars, i) {
                if !param_order.contains(&vr.name) {
                    param_order.push(vr.name.clone());
                }
                i = vr.end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    // 2. Build a name->value map from param_order + args (positional).
    let mut map: HashMap<&str, &str> = HashMap::new();
    for (idx, name) in param_order.iter().enumerate() {
        if let Some(val) = args.get(idx) {
            map.insert(name.as_str(), val);
        }
    }

    // 3. Walk the template again and substitute.
    let mut result = String::with_capacity(template.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some(ref vr) = parse_var_ref(&chars, i) {
                if vr.array_suffix.is_some() || vr.array_sep.is_some() {
                    // Array expansion: `$name[]suffix` (space join) or
                    // `$name[sep]suffix` (joined with ` sep `).
                    let suffix = vr.array_suffix.as_deref().unwrap_or("");
                    let joiner = match vr.array_sep {
                        Some(c) => format!(" {} ", c),
                        None => " ".to_string(),
                    };
                    if let Some(val) = map.get(vr.name.as_str()) {
                        let elements: Vec<&str> =
                            if val.starts_with('[') && val.ends_with(']') && val.len() >= 2 {
                                val[1..val.len() - 1]
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                    .collect()
                            } else {
                                vec![val]
                            };
                        let expanded: Vec<String> = elements
                            .iter()
                            .map(|e| format!("{}{}", e, suffix))
                            .collect();
                        result.push_str(&expanded.join(&joiner));
                    } else if let Some(ref default) = vr.default_val {
                        let elements: Vec<&str> = if default.starts_with('[')
                            && default.ends_with(']')
                            && default.len() >= 2
                        {
                            default[1..default.len() - 1]
                                .split(',')
                                .map(|s| s.trim())
                                .filter(|s| !s.is_empty())
                                .collect()
                        } else {
                            vec![default.as_str()]
                        };
                        let expanded: Vec<String> = elements
                            .iter()
                            .map(|e| format!("{}{}", e, suffix))
                            .collect();
                        result.push_str(&expanded.join(&joiner));
                    } else {
                        result.push('$');
                        result.push_str(&vr.name);
                        result.push('[');
                        if let Some(c) = vr.array_sep {
                            result.push(c);
                        }
                        result.push(']');
                        result.push_str(suffix);
                    }
                } else if let Some(val) = map.get(vr.name.as_str()) {
                    result.push_str(val);
                } else if let Some(ref default) = vr.default_val {
                    result.push_str(default);
                } else {
                    result.push('$');
                    result.push_str(&vr.name);
                }
                i = vr.end;
            } else {
                // Bare $ not followed by a valid name — emit as-is
                result.push('$');
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Extract parameter names from a command template (e.g. `$x`, `${x}`).
/// Returns names in order of first appearance.
/// Shared with the `ndb` shell (`dbal` usage hints).
pub(crate) fn extract_param_names(template: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some(ref vr) = parse_var_ref(&chars, i) {
                let base_name = if vr.array_suffix.is_some() || vr.array_sep.is_some() {
                    match vr.array_sep {
                        Some(c) => format!("{}[{}]", vr.name, c),
                        None => format!("{}[]", vr.name),
                    }
                } else {
                    vr.name.clone()
                };
                let display_name = if let Some(ref default) = vr.default_val {
                    format!("{}={}", base_name, default)
                } else {
                    base_name
                };
                if !names.contains(&display_name) {
                    names.push(display_name);
                }
                i = vr.end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    names
}

/// Parse a parameter hint like `"x=a,e=txt"` into `[("x", "a"), ("e", "txt")]`.
/// A param without `=` has no default (e.g. `"x,y"`).
pub(super) fn parse_param_defaults(hint: &str) -> Vec<(String, String)> {
    hint.split(',')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            if let Some(eq) = part.find('=') {
                let name = part[..eq].trim().to_string();
                let val = part[eq + 1..].trim().to_string();
                if !name.is_empty() {
                    Some((name, val))
                } else {
                    None
                }
            } else {
                None // no default
            }
        })
        .collect()
}

/// Walk a template and replace bare `$name` / `${name}` references with `{name:-default}`
/// for any param that has a default value.
pub(super) fn inject_template_defaults(template: &str, defaults: &[(String, String)]) -> String {
    let mut result = String::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some(ref vr) = parse_var_ref(&chars, i) {
                // Only inject default if this name has no existing default and we have one
                if vr.default_val.is_none() {
                    if let Some((_, def_val)) = defaults.iter().find(|(n, _)| *n == vr.name) {
                        // Replace with ${name:-default}, preserving any
                        // array tail (`[]`, `[c]`, suffix) verbatim.
                        result.push_str("${");
                        result.push_str(&vr.name);
                        result.push_str(":-");
                        result.push_str(def_val);
                        result.push('}');
                        if vr.array_suffix.is_some() || vr.array_sep.is_some() {
                            result.push('[');
                            if let Some(c) = vr.array_sep {
                                result.push(c);
                            }
                            result.push(']');
                            result.push_str(vr.array_suffix.as_deref().unwrap_or(""));
                        }
                        i = vr.end;
                        continue;
                    }
                }
                // No default to inject — emit original chars
                let end = vr.end;
                while i < end {
                    result.push(chars[i]);
                    i += 1;
                }
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// One `#param --hint-*` / `#param --select-*` annotation group for alias completion.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParamHint {
    /// Placeholder name without `#` / brackets.
    pub param: String,
    /// True when written `#name[]` / `#name[c]` (targets array placeholders).
    pub is_array: bool,
    /// Separator from `#name[c]` form (round-tripped verbatim).
    pub array_sep: Option<char>,
    /// Hint kinds in first-seen order (deduped).
    pub kinds: Vec<ParamHintKind>,
    /// Interactive select kinds in first-seen order (deduped).
    /// Prompted when the alias runs with missing args (like `gos`/`txtf`).
    pub selects: Vec<ParamSelectKind>,
}

/// Completion source for one alias parameter.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParamHintKind {
    Tp,
    /// Extensions without dots, lowercased; empty = all files.
    Ext(Vec<String>),
    Spf,
    File,
    Dir,
    Fd,
    All,
    /// Inline literal values: `--hint [a,b]`.
    Values(Vec<String>),
    /// Named value list: `--hint-n <name>` (see `ral hintn`).
    Named(String),
}

/// Interactive-select source for one alias parameter.
/// Mirrors the requested `#param --select-*` flags plus the inline
/// `--select [...]` and named `--select-n <name>` forms.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParamSelectKind {
    Tp,
    /// Extensions without dots, lowercased; empty = all files.
    Ext(Vec<String>),
    File,
    Dir,
    Fd,
    /// Inline literal values: `--select [a,b]`.
    Values(Vec<String>),
    /// Named value list: `--select-n <name>` (see `ral hintn`).
    Named(String),
}

/// Tokenize on whitespace respecting double quotes (quotes stripped).
fn tokenize_hint_args(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut has_token = false;
    for ch in input.chars() {
        if ch == '"' {
            in_quotes = !in_quotes;
            has_token = true;
            continue;
        }
        if ch.is_whitespace() && !in_quotes {
            if has_token {
                out.push(std::mem::take(&mut cur));
                has_token = false;
            }
            continue;
        }
        cur.push(ch);
        has_token = true;
    }
    if has_token {
        out.push(cur);
    }
    out
}

/// Split `#name` / `#name[]` / `#name[c]` into `(name, is_array, sep)`.
/// Multi-char bracket content is not annotation syntax (`None`).
pub(crate) fn parse_hint_param(tok: &str) -> Option<(String, bool, Option<char>)> {
    let body = tok.strip_prefix('#')?;
    if body.is_empty() {
        return None;
    }
    let (name, is_array, sep) = if let Some(inner) = body
        .strip_suffix(']')
        .and_then(|b| b.split_once('['))
    {
        let (n, content) = inner;
        if content.is_empty() {
            (n, true, None)
        } else if content.chars().count() == 1 {
            (n, true, content.chars().next())
        } else {
            return None;
        }
    } else {
        (body, false, None)
    };
    if name.is_empty() {
        return None;
    }
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((name.to_string(), is_array, sep))
}

/// Placeholder idents of a template: `(name, is_array)` in first-appearance
/// order (same walk as [`substitute_params`]).
pub(crate) fn placeholder_idents(template: &str) -> Vec<(String, bool)> {
    let mut out: Vec<(String, bool)> = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some(ref vr) = parse_var_ref(&chars, i) {
                // One entry per name: array-ness from first appearance.
                if !out.iter().any(|(n, _)| n == &vr.name) {
                    out.push((
                        vr.name.clone(),
                        vr.array_suffix.is_some() || vr.array_sep.is_some(),
                    ));
                }
                i = vr.end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Parse one `--hint-*` flag, consuming its value tokens.
/// Returns the kind, or `Err(())` for an unknown hint flag.
fn parse_hint_flag(flag: &str, values: &[String]) -> Result<ParamHintKind, ()> {
    match flag {
        "--hint-tp" => Ok(ParamHintKind::Tp),
        "--hint-spf" => Ok(ParamHintKind::Spf),
        "--hint-file" => Ok(ParamHintKind::File),
        "--hint-dir" => Ok(ParamHintKind::Dir),
        "--hint-fd" => Ok(ParamHintKind::Fd),
        "--hint-all" => Ok(ParamHintKind::All),
        "--hint-ext" => Ok(ParamHintKind::Ext(parse_ext_values(values))),
        "--hint" => Ok(ParamHintKind::Values(parse_inline_values(values))),
        "--hint-n" => Ok(ParamHintKind::Named(clean_hint_list_name(values))),
        _ => Err(()),
    }
}

/// Shared `--hint-ext` / `--select-ext` value parsing.
/// Values may be `py`, `py,c` or `(py, c)` across tokens.
fn parse_ext_values(values: &[String]) -> Vec<String> {
    let mut exts: Vec<String> = Vec::new();
    let joined = values.join(" ");
    for part in joined.split([',', ' ', '\t']) {
        let mut e = part.trim().to_lowercase();
        e = e.trim_matches(|c| c == '(' || c == ')').trim().to_string();
        e = e.strip_prefix('.').unwrap_or(&e).to_string();
        if !e.is_empty() && !exts.contains(&e) {
            exts.push(e);
        }
    }
    exts
}

/// Parse one `--select-*` flag, consuming its value tokens.
fn parse_select_flag(flag: &str, values: &[String]) -> Result<ParamSelectKind, ()> {
    match flag {
        "--select-tp" => Ok(ParamSelectKind::Tp),
        "--select-file" => Ok(ParamSelectKind::File),
        "--select-dir" => Ok(ParamSelectKind::Dir),
        "--select-fd" => Ok(ParamSelectKind::Fd),
        "--select-ext" => Ok(ParamSelectKind::Ext(parse_ext_values(values))),
        "--select" => Ok(ParamSelectKind::Values(parse_inline_values(values))),
        "--select-n" => Ok(ParamSelectKind::Named(clean_hint_list_name(values))),
        _ => Err(()),
    }
}

/// Normalize a `--hint-n` / `--select-n` name token (lowercased).
/// The caller guarantees exactly one token.
fn clean_hint_list_name(values: &[String]) -> String {
    values
        .first()
        .map(|s| clean_list_item(s).to_lowercase())
        .unwrap_or_default()
}

/// Strip one pair of surrounding `"`/`'` quotes and trim whitespace.
/// Used for value-list items and hint-list names.
fn clean_list_item(raw: &str) -> String {
    let t = raw.trim();
    let bytes = t.as_bytes();
    if t.len() >= 2
        && ((bytes[0] == b'"' && bytes[t.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[t.len() - 1] == b'\''))
    {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

/// Parse an inline `--hint [...]` / `--select [...]` value span.
/// `values` are the whitespace tokens of the `[...]` span (quotes that
/// grouped spaces were already stripped by tokenizing, so commas always
/// separate — use a named `hintn` list when a value needs a literal
/// comma). Empty items are dropped; an empty span yields an empty vec
/// (rejected by the caller with `bad_group`).
fn parse_inline_values(values: &[String]) -> Vec<String> {
    let joined = values.join(" ");
    // Guard: an odd `"` count means tokenizing swallowed a following
    // token (e.g. `#y`) into this span — reject rather than corrupt.
    if joined.matches('"').count() % 2 == 1 {
        return Vec::new();
    }
    let t = joined.trim();
    let inner = match (t.find('['), t.rfind(']')) {
        (Some(s), Some(e)) if e > s => &t[s + 1..e],
        _ => t,
    };
    let mut out: Vec<String> = Vec::new();
    for part in inner.split(',') {
        let item = clean_list_item(part);
        if !item.is_empty() && !out.contains(&item) {
            out.push(item);
        }
    }
    out
}

/// Parse a raw `[...]` value list (for `ral hintn add`), quote-aware so
/// `"a,b"` stays one value. Requires balanced brackets, ≥1 value, and
/// nothing but whitespace after the closing `]`.
pub(crate) fn parse_value_list_raw(input: &str) -> Result<Vec<String>, ()> {
    let t = input.trim();
    let start_byte = t.find('[').ok_or(())?;
    // Convert the byte index to a char index (`chars` and byte
    // positions diverge on multibyte input).
    let start = t[..start_byte].chars().count();
    let chars: Vec<char> = t.chars().collect();
    let mut i = start;
    let mut depth = 0i32;
    let mut quote: Option<char> = None;
    let mut end: Option<usize> = None;
    while i < chars.len() {
        let c = chars[i];
        if let Some(q) = quote {
            if c == q {
                quote = None;
            }
        } else if c == '"' || c == '\'' {
            quote = Some(c);
        } else if c == '[' {
            depth += 1;
        } else if c == ']' {
            depth -= 1;
            if depth == 0 {
                end = Some(i);
                break;
            }
            if depth < 0 {
                return Err(());
            }
        }
        i += 1;
    }
    let end = end.ok_or(())?;
    let inner: String = chars[start + 1..end].iter().collect();
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut q: Option<char> = None;
    for c in inner.chars() {
        if let Some(qq) = q {
            if c == qq {
                q = None;
            } else {
                cur.push(c);
            }
        } else if c == '"' || c == '\'' {
            q = Some(c);
        } else if c == ',' {
            let item = clean_list_item(&cur);
            if !item.is_empty() && !out.contains(&item) {
                out.push(item);
            }
            cur.clear();
        } else {
            cur.push(c);
        }
    }
    if q.is_some() {
        return Err(());
    }
    let item = clean_list_item(&cur);
    if !item.is_empty() && !out.contains(&item) {
        out.push(item);
    }
    if out.is_empty() {
        return Err(());
    }
    // Trailing garbage after `]` is a usage error.
    let rest: String = chars[end + 1..].iter().collect();
    if !rest.trim().is_empty() {
        return Err(());
    }
    Ok(out)
}

/// Strictly parse a token tail as hint/select annotations.
/// Every `#param` group must carry ≥1 flag (a flagless group — mid-tail
/// or trailing — is malformed), every flag must be known, and every name
/// must occur in `idents`. Returns the groups, or `Err((code, idx))`
/// with `idx` the offending token index: `"unknown_hint"` /
/// `"unknown_param"` / `"bad_group"`. Same-param repeats merge.
fn parse_hint_tail(
    toks: &[String],
    idents: &[(String, bool)],
) -> Result<Vec<ParamHint>, (&'static str, usize)> {
    fn flush(
        cur: &mut Option<(usize, ParamHint)>,
        groups: &mut Vec<ParamHint>,
    ) -> Result<(), (&'static str, usize)> {
        if let Some((s, g)) = cur.take() {
            if g.kinds.is_empty() && g.selects.is_empty() {
                return Err(("bad_group", s));
            }
            if let Some(prev) = groups.iter_mut().find(|p| p.param == g.param) {
                for k in g.kinds {
                    if !prev.kinds.contains(&k) {
                        prev.kinds.push(k);
                    }
                }
                for k in g.selects {
                    if !prev.selects.contains(&k) {
                        prev.selects.push(k);
                    }
                }
            } else {
                groups.push(g);
            }
        }
        Ok(())
    }
    let mut groups: Vec<ParamHint> = Vec::new();
    let mut i = 0;
    let mut cur: Option<(usize, ParamHint)> = None;
    while i < toks.len() {
        let tok = &toks[i];
        if let Some((name, is_array, array_sep)) = parse_hint_param(tok) {
            flush(&mut cur, &mut groups)?;
            if !idents.iter().any(|(n, _)| n == &name) {
                return Err(("unknown_param", i));
            }
            cur = Some((
                i,
                ParamHint {
                    param: name,
                    is_array,
                    array_sep,
                    kinds: Vec::new(),
                    selects: Vec::new(),
                },
            ));
            i += 1;
        } else if tok == "--hint"
            || tok == "--hint-n"
            || tok == "--select"
            || tok == "--select-n"
            || tok.starts_with("--hint-")
            || tok.starts_with("--select-")
        {
            let Some((_, ref mut g)) = cur else {
                return Err(("bad_group", i));
            };
            let is_hint_ext = tok == "--hint-ext";
            let is_select_ext = tok == "--select-ext";
            let is_inline = tok == "--hint" || tok == "--select";
            let is_named = tok == "--hint-n" || tok == "--select-n";
            // `--hint-ext` / `--select-ext` take space/comma values;
            // `--hint` / `--select` take one `[...]` span; `--hint-n` /
            // `--select-n` take one name. Other flags take none, so a
            // following bare token falls through to the `bad_group` arm.
            let mut values: Vec<String> = Vec::new();
            let mut j = i + 1;
            if is_hint_ext || is_select_ext {
                while j < toks.len()
                    && !toks[j].starts_with('#')
                    && !toks[j].starts_with("--hint-")
                    && !toks[j].starts_with("--select-")
                    && !toks[j].starts_with('-')
                {
                    values.push(toks[j].clone());
                    j += 1;
                }
            } else if is_inline {
                // Bracket span: join tokens until brackets balance.
                // Stops at a new group/flag; an unclosed span is malformed.
                let mut depth = 0i32;
                while j < toks.len()
                    && !toks[j].starts_with('#')
                    && !is_annotation_flag(&toks[j])
                {
                    for ch in toks[j].chars() {
                        if ch == '[' {
                            depth += 1;
                        } else if ch == ']' {
                            depth -= 1;
                        }
                    }
                    values.push(toks[j].clone());
                    j += 1;
                    if depth == 0 {
                        break;
                    }
                }
                if values.is_empty()
                    || depth != 0
                    || !values[0].trim_start().starts_with('[')
                    || parse_inline_values(&values).is_empty()
                {
                    return Err(("bad_group", i));
                }
            } else if is_named {
                if j < toks.len()
                    && !toks[j].starts_with('#')
                    && !is_annotation_flag(&toks[j])
                    && !toks[j].starts_with('-')
                {
                    values.push(toks[j].clone());
                    j += 1;
                }
                if values.len() != 1 || clean_hint_list_name(&values).is_empty() {
                    return Err(("bad_group", i));
                }
            }
            if tok == "--hint" || tok == "--hint-n" || tok.starts_with("--hint-") {
                match parse_hint_flag(tok.as_str(), &values) {
                    Ok(kind) => {
                        if !g.kinds.contains(&kind) {
                            g.kinds.push(kind);
                        }
                    }
                    Err(()) => return Err(("unknown_hint", i)),
                }
            } else {
                match parse_select_flag(tok.as_str(), &values) {
                    Ok(kind) => {
                        if !g.selects.contains(&kind) {
                            g.selects.push(kind);
                        }
                    }
                    Err(()) => return Err(("unknown_hint", i)),
                }
            }
            i = j;
        } else {
            // Bare values only belong to `--hint-ext` / `--select-ext`;
            // anything else (flags, stray text) breaks the annotation shape.
            return Err(("bad_group", i));
        }
    }
    flush(&mut cur, &mut groups)?;
    if groups.is_empty() {
        return Err(("bad_group", 0));
    }
    Ok(groups)
}

/// True for annotation flag tokens: bare `--hint` / `--select` and any
/// `--hint-*` / `--select-*` form (including `--hint-n` / `--select-n`).
fn is_annotation_flag(tok: &str) -> bool {
    tok == "--hint"
        || tok == "--select"
        || tok.starts_with("--hint-")
        || tok.starts_with("--select-")
}

/// Split a `ral add`/`edit` command into its executable template and
/// trailing `#param --hint-*` / `#param --select-*` annotations.
///
/// Candidate regions start at each `#param`-shaped token, tried left to
/// right; the first tail that strictly parses wins (so `echo #a $x
/// #x --hint-dir` keeps `echo #a $x` as command text). Failures are
/// remembered, not fatal: with no parseable region, a tail that
/// mentioned `--hint-*` / `--select-*` returns the first failure code,
/// otherwise the whole input is plain command text.
pub(crate) fn split_hint_annotations(input: &str) -> Result<(String, Vec<ParamHint>), &'static str> {
    let toks = tokenize_hint_args(input);
    let mut first_err: Option<&'static str> = None;
    let mut mentioned = false;
    for start in 0..toks.len() {
        if parse_hint_param(&toks[start]).is_none() {
            continue;
        }
        let tail = &toks[start..];
        if tail.iter().any(|t| {
            t == "--hint" || t == "--select" || t.starts_with("--hint-") || t.starts_with("--select-")
        }) {
            mentioned = true;
        }
        let idents = placeholder_idents(&toks[..start].join(" "));
        match parse_hint_tail(tail, &idents) {
            Ok(groups) => {
                let cmd = toks[..start].join(" ");
                return Ok((cmd, groups));
            }
            Err((code, _)) => {
                if first_err.is_none() {
                    first_err = Some(code);
                }
            }
        }
    }
    match first_err {
        Some(code) if mentioned => Err(code),
        _ => Ok((input.trim().to_string(), Vec::new())),
    }
}

/// Parse a stored hint spec (serialized form) back into groups.
/// Lenient: anything unparseable is skipped (specs are validated at
/// `ral add`/`edit` time, so this is just tamper-proofing).
pub(crate) fn parse_hint_spec(spec: &str) -> Vec<ParamHint> {
    let toks = tokenize_hint_args(spec);
    let idents: Vec<(String, bool)> = toks
        .iter()
        .filter_map(|t| parse_hint_param(t).map(|(n, a, _)| (n, a)))
        .collect();
    let mut idents = idents;
    idents.dedup();
    parse_hint_tail(&toks, &idents).unwrap_or_default()
}

/// Serialize hints back to canonical `#x --hint-dir …` form
/// (ext values comma-joined, bare `--hint-ext` / `--select-ext` when empty).
pub(crate) fn serialize_hint_spec(hints: &[ParamHint]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for h in hints {
        let mut p = format!(
            "#{}{}",
            h.param,
            match (h.is_array, h.array_sep) {
                (false, _) => String::new(),
                (true, Some(c)) => format!("[{}]", c),
                (true, None) => "[]".to_string(),
            }
        );
        for k in &h.kinds {
            match k {
                ParamHintKind::Tp => p.push_str(" --hint-tp"),
                ParamHintKind::Spf => p.push_str(" --hint-spf"),
                ParamHintKind::File => p.push_str(" --hint-file"),
                ParamHintKind::Dir => p.push_str(" --hint-dir"),
                ParamHintKind::Fd => p.push_str(" --hint-fd"),
                ParamHintKind::All => p.push_str(" --hint-all"),
                ParamHintKind::Ext(exts) if exts.is_empty() => p.push_str(" --hint-ext"),
                ParamHintKind::Ext(exts) => {
                    p.push_str(&format!(" --hint-ext {}", exts.join(",")));
                }
                ParamHintKind::Values(vals) => {
                    p.push_str(&format!(" --hint [{}]", serialize_values(vals)));
                }
                ParamHintKind::Named(name) => {
                    p.push_str(&format!(" --hint-n {}", name));
                }
            }
        }
        for k in &h.selects {
            match k {
                ParamSelectKind::Tp => p.push_str(" --select-tp"),
                ParamSelectKind::File => p.push_str(" --select-file"),
                ParamSelectKind::Dir => p.push_str(" --select-dir"),
                ParamSelectKind::Fd => p.push_str(" --select-fd"),
                ParamSelectKind::Ext(exts) if exts.is_empty() => p.push_str(" --select-ext"),
                ParamSelectKind::Ext(exts) => {
                    p.push_str(&format!(" --select-ext {}", exts.join(",")));
                }
                ParamSelectKind::Values(vals) => {
                    p.push_str(&format!(" --select [{}]", serialize_values(vals)));
                }
                ParamSelectKind::Named(name) => {
                    p.push_str(&format!(" --select-n {}", name));
                }
            }
        }
        parts.push(p);
    }
    parts.join(" ")
}

/// Serialize inline values, quoting items that need it to re-parse
/// (spaces, commas, brackets, `#`, leading `-`). Items containing `"`
/// are single-quoted; items with both quote kinds fall back to
/// double-quotes (documented limitation).
fn serialize_values(vals: &[String]) -> String {
    vals.iter()
        .map(|v| {
            let needs_quote = v.chars().any(|c| {
                c.is_whitespace() || c == ',' || c == '[' || c == ']' || c == '#'
            }) || v.starts_with('-')
                || v.contains('"')
                || v.contains('\'');
            if !needs_quote {
                v.clone()
            } else if !v.contains('"') {
                format!("\"{}\"", v)
            } else if !v.contains('\'') {
                format!("'{}'", v)
            } else {
                format!("\"{}\"", v)
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// `--hint-n` / `--select-n` list names referenced by parsed groups
/// (lowercased, deduped, first-seen order).
pub(crate) fn named_refs_in_groups(groups: &[ParamHint]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for g in groups {
        for k in &g.kinds {
            if let ParamHintKind::Named(n) = k {
                if !out.contains(n) {
                    out.push(n.clone());
                }
            }
        }
        for k in &g.selects {
            if let ParamSelectKind::Named(n) = k {
                if !out.contains(n) {
                    out.push(n.clone());
                }
            }
        }
    }
    out
}

/// `--hint-n` / `--select-n` names referenced by a spec string.
pub(crate) fn spec_referenced_lists(spec: &str) -> Vec<String> {
    let toks = tokenize_hint_args(spec);
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        if (toks[i] == "--hint-n" || toks[i] == "--select-n") && i + 1 < toks.len() {
            let name = clean_list_item(&toks[i + 1]).to_lowercase();
            if !name.is_empty() && !out.contains(&name) {
                out.push(name);
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

/// True when a spec string references hint list `name`.
pub(crate) fn spec_references_named(spec: &str, name: &str) -> bool {
    spec_referenced_lists(spec)
        .iter()
        .any(|n| n == &name.to_lowercase())
}

/// Rewrite `--hint-n` / `--select-n` references `old` → `new` in a spec
/// string. Operates on the same token stream the parser sees; inline
/// `[...]` spans re-join to identical items on re-parse.
pub(crate) fn rename_named_in_spec(spec: &str, old: &str, new: &str) -> String {
    let toks = tokenize_hint_args(spec);
    let old_lower = old.to_lowercase();
    let mut out = toks;
    let mut i = 0;
    while i < out.len() {
        if (out[i] == "--hint-n" || out[i] == "--select-n") && i + 1 < out.len() {
            if clean_list_item(&out[i + 1]).to_lowercase() == old_lower {
                out[i + 1] = new.to_lowercase();
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out.join(" ")
}

/// Values of a named hint list (empty when missing).
pub(crate) fn named_list_values(name: &str) -> Vec<String> {
    Config::get_run_hint_lists_merged()
        .get(&name.to_lowercase())
        .cloned()
        .unwrap_or_default()
}

/// Public entry point: expand aliases in a command (no && splitting).
pub(super) fn expand_aliases(cmd: &str) -> String {
    expand_aliases_rec(cmd, 0)
}

/// Recursively expand aliases in a command (no && splitting).
/// Supports parameterised calls: `py(hello)` with template `python $x.py` -> `python hello.py`
fn expand_aliases_rec(cmd: &str, depth: usize) -> String {
    const MAX_DEPTH: usize = 5;
    if depth > MAX_DEPTH {
        return cmd.to_string();
    }

    let aliases = Config::global_get_run_aliases();
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    if parts.is_empty() {
        return cmd.to_string();
    }

    let first_token = parts[0];
    let rest = if parts.len() > 1 {
        cmd[first_token.len()..].trim_start()
    } else {
        ""
    };

    // Parse potential `name(arg)` call syntax
    let (base_name, call_arg) = parse_call_syntax(first_token);
    let lookup_key = base_name.to_lowercase();

    if let Some(template) = aliases.get(&lookup_key) {
        let has_placeholders = template.contains('$');
        let (expanded_template, consumed_rest) = if let Some(args_str) = call_arg {
            // Parenthesized args: py(hello) or runc([add,minus], math)
            let args: Vec<&str> = if args_str.is_empty() {
                Vec::new()
            } else {
                args_str.split(',').map(|s| s.trim()).collect()
            };
            (substitute_params(template, &args), false)
        } else if !rest.is_empty() && has_placeholders {
            // Positional args: runc [add,minus,mul] math
            let tokens = tokenize_args(rest);
            let token_refs: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
            (substitute_params(template, &token_refs), true)
        } else {
            // Even with no args, substitute so that ${name:-default} gets resolved
            (substitute_params(template, &[]), false)
        };

        let new_cmd = if consumed_rest || rest.is_empty() {
            expanded_template
        } else {
            format!("{} {}", expanded_template, rest)
        };
        expand_aliases_rec(&new_cmd, depth + 1)
    } else {
        cmd.to_string()
    }
}

/// Fully expand a command line, handling && properly
pub(super) fn expand_command_line(input: &str) -> Vec<String> {
    if input.trim().is_empty() {
        return vec![];
    }

    let parts = split_on_ampersands(input);
    let mut result = Vec::new();
    for part in parts {
        let expanded = expand_aliases(&part);
        if contains_ampersands(&expanded) {
            result.extend(expand_command_line(&expanded));
        } else if !expanded.trim().is_empty() {
            result.push(expanded);
        }
    }
    result
}

/// Ordered placeholders with their first-seen defaults (if any).
fn placeholder_defaults(template: &str) -> Vec<(String, bool, Option<String>)> {
    let mut out: Vec<(String, bool, Option<String>)> = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some(ref vr) = parse_var_ref(&chars, i) {
                if !out.iter().any(|(n, _, _)| n == &vr.name) {
                    out.push((
                        vr.name.clone(),
                        vr.array_suffix.is_some() || vr.array_sep.is_some(),
                        vr.default_val.clone(),
                    ));
                }
                i = vr.end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    out
}

/// List plain file names in `cwd`, optionally filtered.
fn select_cwd_files(cwd: &std::path::Path, matching: impl Fn(&str) -> bool) -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(cwd) {
        for entry in entries.flatten() {
            let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
            if !is_file {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                if matching(name) {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out
}

/// List subdirectory names in `cwd`, sorted.
fn select_cwd_dirs(cwd: &std::path::Path) -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(cwd) {
        for entry in entries.flatten() {
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if !is_dir {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out
}

/// Union candidates for one param's `--select-*` kinds (sorted, deduped).
fn select_kind_candidates(
    kinds: &[ParamSelectKind],
    cwd: &std::path::Path,
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for kind in kinds {
        match kind {
            ParamSelectKind::Tp => {
                out.extend(crate::teleport::TeleportManager::get_all().into_keys());
            }
            ParamSelectKind::Ext(exts) if !exts.is_empty() => {
                out.extend(select_cwd_files(cwd, |n| {
                    let lower = n.to_lowercase();
                    exts.iter()
                        .any(|e| lower == *e || lower.ends_with(&format!(".{}", e)))
                }));
            }
            ParamSelectKind::Ext(_) | ParamSelectKind::File => {
                out.extend(select_cwd_files(cwd, |_| true));
            }
            ParamSelectKind::Dir => {
                out.push(".".to_string());
                out.extend(select_cwd_dirs(cwd));
            }
            ParamSelectKind::Fd => {
                out.push(".".to_string());
                out.extend(select_cwd_dirs(cwd));
                out.extend(select_cwd_files(cwd, |_| true));
            }
            ParamSelectKind::Values(vals) => {
                out.extend(vals.iter().cloned());
            }
            ParamSelectKind::Named(name) => {
                out.extend(named_list_values(name));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Prompt for one missing param with an `InteractiveMenu` (like `gos`/`txtf`).
/// Cancel (`0`/empty/Esc/invalid) uses `default` when present, else `None` (abort).
fn prompt_select_value(
    param: &str,
    selects: &[ParamSelectKind],
    default: Option<&str>,
    cwd: &std::path::Path,
) -> anyhow::Result<Option<String>> {
    use crate::utils::interactive::{InteractiveMenu, MenuItem};
    let candidates = select_kind_candidates(selects, cwd);
    if candidates.is_empty() {
        if let Some(d) = default {
            crate::output::print_info(&format!(
                "No matches for '{}'; using default '{}'.",
                param, d
            ));
            return Ok(Some(d.to_string()));
        }
        crate::output::print_info(&format!("No matches for '{}'; cancelled.", param));
        return Ok(None);
    }
    let title = match default {
        Some(d) => format!("Select {} (0 = default '{}'):", param, d),
        None => format!("Select {}:", param),
    };
    let items: Vec<MenuItem> = candidates
        .iter()
        .map(|c| MenuItem::simple(c))
        .collect();
    let prompt = format!("Enter number (1-{}) or 0: ", candidates.len());
    // InteractiveMenu owns its title str; build an owned string first.
    let title_owned = title.clone();
    let menu = InteractiveMenu::new(title_owned.as_str())
        .items(items)
        .prompt(prompt);
    match menu.run()? {
        Some(idx) => Ok(candidates.get(idx).cloned()),
        None => match default {
            Some(d) => {
                crate::output::print_info(&format!(
                    "Cancelled; using default '{}' for '{}'.",
                    d, param
                ));
                Ok(Some(d.to_string()))
            }
            None => Ok(None),
        },
    }
}

/// Try to fill one `&&`-part's missing alias args via `--select-*` prompts.
/// Returns `Ok(None)` when the user cancelled a param without default (abort
/// the whole line), else `Ok(Some((invocation, prompted)))` where invocation
/// is the (possibly rewritten) alias call and `prompted` reports whether any
/// menu was shown.
fn fill_invocation_once(
    part: &str,
    nav: &crate::navigator::Navigator,
) -> anyhow::Result<Option<(String, bool)>> {
    let trimmed = part.trim();
    if trimmed.is_empty() {
        return Ok(Some((part.to_string(), false)));
    }
    let split: Vec<&str> = trimmed.splitn(2, ' ').collect();
    let first_token = split[0];
    let rest = if split.len() > 1 {
        trimmed[first_token.len()..].trim_start()
    } else {
        ""
    };
    let (base_name, call_arg) = parse_call_syntax(first_token);
    let lookup_key = base_name.to_lowercase();
    let aliases = Config::global_get_run_aliases();
    let Some(template) = aliases.get(&lookup_key) else {
        return Ok(Some((part.to_string(), false)));
    };
    let template = template.clone();
    let order = placeholder_defaults(&template);
    if order.is_empty() {
        return Ok(Some((part.to_string(), false)));
    }
    let is_call_form = call_arg.is_some();
    let base_original = base_name.to_string();
    let trailing = if is_call_form { rest.to_string() } else { String::new() };
    let mut provided: Vec<String> = if let Some(args_str) = call_arg {
        if args_str.trim().is_empty() {
            Vec::new()
        } else {
            args_str.split(',').map(|s| s.trim().to_string()).collect()
        }
    } else if rest.is_empty() {
        Vec::new()
    } else {
        tokenize_args(rest)
    };
    // If every missing slot has no select spec, let the classic path handle it.
    let hints = Config::global_get_run_alias_hints();
    let groups: Vec<ParamHint> = hints
        .get(&lookup_key)
        .map(|spec| parse_hint_spec(spec))
        .unwrap_or_default();
    let mut missing_has_select = false;
    // Determine missing range: positional = provided.len()..
    // call-form = first empty slot onwards (plus trailing missing).
    let start_idx: usize = if is_call_form {
        // Number of leading non-empty call args.
        let mut n = 0;
        for a in &provided {
            if a.trim().is_empty() {
                break;
            }
            n += 1;
        }
        // If all provided slots non-empty but fewer than order, missing starts there.
        if n == provided.len() {
            provided.len()
        } else {
            n
        }
    } else {
        provided.len()
    };
    if start_idx >= order.len() {
        return Ok(Some((part.to_string(), false)));
    }
    for idx in start_idx..order.len() {
        // For call-form gaps beyond the first empty, every slot counts as missing.
        let (pname, _, _) = &order[idx];
        if let Some(g) = groups.iter().find(|g| &g.param == pname) {
            if !g.selects.is_empty() {
                missing_has_select = true;
                break;
            }
        }
    }
    if !missing_has_select {
        return Ok(Some((part.to_string(), false)));
    }
    let cwd = nav.current_path().to_path_buf();
    // Build full positional args, prompting for select slots.
    let mut full_args: Vec<String> = Vec::new();
    // Keep leading provided (for call-form, keep up to start_idx).
    let keep = if is_call_form { start_idx.min(provided.len()) } else { provided.len() };
    for a in provided.iter().take(keep) {
        full_args.push(a.clone());
    }
    // For positional, provided already in full_args via keep.
    for idx in full_args.len()..order.len() {
        let (pname, is_array, default) = &order[idx];
        // Call-form gap value if present and non-empty beyond start? For
        // `a,,c` the later non-empty values shift; simplest: ignore later
        // call values once a gap appeared (user will re-pick sequentially).
        let group = groups.iter().find(|g| &g.param == pname);
        let selects: &[ParamSelectKind] = group.map(|g| g.selects.as_slice()).unwrap_or(&[]);
        if selects.is_empty() {
            if let Some(d) = default {
                full_args.push(d.clone());
            } else if *is_array {
                full_args.push(format!("${}[]", pname));
            } else {
                full_args.push(format!("${}", pname));
            }
            continue;
        }
        match prompt_select_value(pname, selects, default.as_deref(), &cwd)? {
            Some(v) => full_args.push(v),
            None => return Ok(None),
        }
    }
    // Reconstruct invocation in its original form.
    let filled = if is_call_form {
        let inner = full_args.join(",");
        if trailing.is_empty() {
            format!("{}({})", base_original, inner)
        } else {
            format!("{}({}) {}", base_original, inner, trailing)
        }
    } else {
        // Positional: rest is consumed as args.
        let _ = std::mem::take(&mut provided);
        format!("{} {}", first_token, full_args.join(" "))
    };
    Ok(Some((filled, true)))
}

/// Expand aliases, prompting `--select-*` menus for missing args.
/// Returns `Ok(None)` when the user cancelled (execute nothing).
pub(crate) fn expand_command_line_interactive(
    input: &str,
    nav: &mut crate::navigator::Navigator,
) -> anyhow::Result<Option<Vec<String>>> {
    if input.trim().is_empty() {
        return Ok(Some(vec![]));
    }
    let mut current = input.trim().to_string();
    for _ in 0..5 {
        let parts = split_on_ampersands(&current);
        let mut any_prompted = false;
        let mut filled_parts: Vec<String> = Vec::new();
        for part in &parts {
            match fill_invocation_once(part, nav)? {
                None => return Ok(None),
                Some((filled, prompted)) => {
                    any_prompted |= prompted;
                    filled_parts.push(filled);
                }
            }
        }
        let joined = filled_parts.join(" && ");
        let expanded = expand_command_line(&joined);
        let expanded_joined = expanded.join(" && ");
        if !any_prompted && expanded_joined == joined {
            return Ok(Some(expanded));
        }
        if !any_prompted && expanded_joined == current {
            return Ok(Some(expanded));
        }
        // If nothing more to expand and nothing prompted, done.
        if expanded_joined == current && !any_prompted {
            return Ok(Some(expanded));
        }
        current = expanded_joined;
        if !any_prompted {
            // No prompts this round but expansion progressed (chained
            // alias); loop again to catch deeper `--select-*` roots.
            continue;
        }
        // Prompted: loop again in case expansion revealed another alias
        // needing selection; otherwise next round exits with no prompts.
        continue;
    }
    Ok(Some(expand_command_line(&current)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_on_ampersands_simple() {
        let parts = split_on_ampersands("echo a && echo b");
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "echo a");
        assert_eq!(parts[1], "echo b");
    }

    #[test]
    fn test_split_on_ampersands_no_ampersands() {
        let parts = split_on_ampersands("echo hello");
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0], "echo hello");
    }

    #[test]
    fn test_split_on_ampersands_in_quotes() {
        let parts = split_on_ampersands("echo \"a && b\"");
        assert_eq!(parts.len(), 1);
        assert!(parts[0].contains("a && b"));
    }

    #[test]
    fn test_split_on_ampersands_empty_middle_skipped() {
        let parts = split_on_ampersands("echo a &&  && echo b");
        // Middle segment is whitespace-only, so it's skipped
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "echo a");
        assert_eq!(parts[1], "echo b");
    }

    #[test]
    fn test_parse_call_syntax_no_parens() {
        let (name, arg) = parse_call_syntax("hello");
        assert_eq!(name, "hello");
        assert!(arg.is_none());
    }

    #[test]
    fn test_parse_call_syntax_with_parens() {
        let (name, arg) = parse_call_syntax("py(hello)");
        assert_eq!(name, "py");
        assert_eq!(arg, Some("hello"));
    }

    #[test]
    fn test_parse_call_syntax_empty_arg() {
        let (name, arg) = parse_call_syntax("run()");
        assert_eq!(name, "run");
        assert_eq!(arg, Some(""));
    }

    #[test]
    fn test_parse_call_syntax_no_closing_paren() {
        let (name, arg) = parse_call_syntax("run(");
        assert_eq!(name, "run(");
        assert!(arg.is_none());
    }

    #[test]
    fn test_contains_ampersands_true() {
        assert!(contains_ampersands("a && b"));
    }

    #[test]
    fn test_contains_ampersands_false() {
        assert!(!contains_ampersands("echo hello"));
    }

    #[test]
    fn test_contains_ampersands_in_quotes() {
        assert!(!contains_ampersands("echo \"a && b\""));
    }

    #[test]
    fn hint_annotations_all_kinds() {
        let (cmd, hints) =
            split_hint_annotations("tl dir ${x:-.} #x --hint-dir").unwrap();
        assert_eq!(cmd, "tl dir ${x:-.}");
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].param, "x");
        assert!(!hints[0].is_array);
        assert_eq!(hints[0].kinds, vec![ParamHintKind::Dir]);
    }

    #[test]
    fn hint_annotations_multi_flag_union_and_ext_forms() {
        let (cmd, hints) =
            split_hint_annotations("cp ${x} ${y} #x --hint-all #y --hint-tp --hint-dir")
                .unwrap();
        assert_eq!(cmd, "cp ${x} ${y}");
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].kinds, vec![ParamHintKind::All]);
        assert_eq!(
            hints[1].kinds,
            vec![ParamHintKind::Tp, ParamHintKind::Dir]
        );
        // Ext value forms.
        for (tail, want) in [
            ("#x --hint-ext py", vec!["py"]),
            ("#x --hint-ext (py, c)", vec!["py", "c"]),
            ("#x --hint-ext py,c", vec!["py", "c"]),
            ("#x --hint-ext .PY", vec!["py"]),
        ] {
            let (_, hints) =
                split_hint_annotations(&format!("python ${{x:-test}} {}", tail)).unwrap();
            let want: Vec<String> = want.into_iter().map(str::to_string).collect();
            assert_eq!(hints[0].kinds, vec![ParamHintKind::Ext(want)], "tail: {}", tail);
        }
        // Bare --hint-ext with no values = all files.
        let (_, hints) =
            split_hint_annotations("run $x #x --hint-ext").unwrap();
        assert_eq!(hints[0].kinds, vec![ParamHintKind::Ext(vec![])]);
        // Array placeholder form.
        let (_, hints) =
            split_hint_annotations("gcc -o ${o:-a}.exe $x[] #x[] --hint-ext c").unwrap();
        assert!(hints[0].is_array);
    }

    #[test]
    fn hint_annotations_plain_commands_untouched() {
        // No #param tail at all.
        let (cmd, hints) = split_hint_annotations("cargo build --release").unwrap();
        assert_eq!(cmd, "cargo build --release");
        assert!(hints.is_empty());
        // Lone #word without flags stays command text.
        let (cmd, hints) = split_hint_annotations("echo #notahint").unwrap();
        assert_eq!(cmd, "echo #notahint");
        assert!(hints.is_empty());
        // Maximal command wins when an earlier #word exists.
        let (cmd, hints) = split_hint_annotations("echo #a $x #x --hint-dir").unwrap();
        assert_eq!(cmd, "echo #a $x");
        assert_eq!(hints.len(), 1);
    }

    #[test]
    fn hint_annotations_errors_save_nothing() {
        // Unknown hint flag.
        assert_eq!(
            split_hint_annotations("run $x #x --hint-bogus").unwrap_err(),
            "unknown_hint"
        );
        // Unknown param.
        assert_eq!(
            split_hint_annotations("run $x #y --hint-dir").unwrap_err(),
            "unknown_param"
        );
        // Trailing flagless group.
        assert_eq!(
            split_hint_annotations("run $x #x --hint-dir #y").unwrap_err(),
            "unknown_param"
        );
        // Bare value where a group is expected.
        assert_eq!(
            split_hint_annotations("run $x #x --hint-dir oops").unwrap_err(),
            "bad_group"
        );
    }

    #[test]
    fn hint_spec_roundtrips() {
        let (_, hints) =
            split_hint_annotations("cp ${x} ${y} #x --hint-all #y --hint-tp --hint-dir")
                .unwrap();
        assert_eq!(
            serialize_hint_spec(&hints),
            "#x --hint-all #y --hint-tp --hint-dir"
        );
        let (_, hints) =
            split_hint_annotations("run $x #x --hint-ext (py, c)").unwrap();
        assert_eq!(serialize_hint_spec(&hints), "#x --hint-ext py,c");
    }

    #[test]
    fn select_annotations_all_kinds() {
        for (tail, check) in [
            ("#x --select-dir", "dir"),
            ("#x --select-file", "file"),
            ("#x --select-fd", "fd"),
            ("#x --select-tp", "tp"),
        ] {
            let (cmd, hints) =
                split_hint_annotations(&format!("run $x {}", tail)).unwrap();
            assert_eq!(cmd, "run $x", "tail: {}", tail);
            assert_eq!(hints.len(), 1);
            assert_eq!(hints[0].param, "x");
            assert!(hints[0].kinds.is_empty());
            assert_eq!(hints[0].selects.len(), 1, "tail: {}", tail);
            let _ = check;
        }
        let (_, hints) = split_hint_annotations("tl dir ${x:-.} #x --select-dir").unwrap();
        assert_eq!(hints[0].selects, vec![ParamSelectKind::Dir]);
        // Bare --select-ext = all files.
        let (_, hints) = split_hint_annotations("run $x #x --select-ext").unwrap();
        assert_eq!(hints[0].selects, vec![ParamSelectKind::Ext(vec![])]);
    }

    #[test]
    fn select_ext_value_forms() {
        for (tail, want) in [
            ("#x --select-ext py", vec!["py"]),
            ("#x --select-ext (py, c)", vec!["py", "c"]),
            ("#x --select-ext py,c", vec!["py", "c"]),
            ("#x --select-ext .PY", vec!["py"]),
        ] {
            let (_, hints) =
                split_hint_annotations(&format!("run $x {}", tail)).unwrap();
            let want: Vec<String> = want.into_iter().map(str::to_string).collect();
            assert_eq!(
                hints[0].selects,
                vec![ParamSelectKind::Ext(want)],
                "tail: {}",
                tail
            );
        }
    }

    #[test]
    fn select_mixed_hint_and_union() {
        let (cmd, hints) =
            split_hint_annotations("cp ${x} ${y} #x --hint-dir --select-dir #y --select-tp --select-dir")
                .unwrap();
        assert_eq!(cmd, "cp ${x} ${y}");
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].kinds, vec![ParamHintKind::Dir]);
        assert_eq!(hints[0].selects, vec![ParamSelectKind::Dir]);
        assert_eq!(
            hints[1].selects,
            vec![ParamSelectKind::Tp, ParamSelectKind::Dir]
        );
        // Same-param repeats merge.
        let (_, hints) =
            split_hint_annotations("run $x #x --select-dir #x --select-tp").unwrap();
        assert_eq!(
            hints[0].selects,
            vec![ParamSelectKind::Dir, ParamSelectKind::Tp]
        );
        // Roundtrip keeps both.
        assert_eq!(
            serialize_hint_spec(&hints),
            "#x --select-dir --select-tp"
        );
        let (_, hints) =
            split_hint_annotations("run $x #x --hint-dir --select-dir --select-ext py,c").unwrap();
        assert_eq!(
            serialize_hint_spec(&hints),
            "#x --hint-dir --select-dir --select-ext py,c"
        );
    }

    #[test]
    fn select_errors() {
        // Unknown select flag.
        assert_eq!(
            split_hint_annotations("run $x #x --select-bogus").unwrap_err(),
            "unknown_hint"
        );
        // Unknown param.
        assert_eq!(
            split_hint_annotations("run $x #y --select-dir").unwrap_err(),
            "unknown_param"
        );
        // Bare value after non-ext select.
        assert_eq!(
            split_hint_annotations("run $x #x --select-dir oops").unwrap_err(),
            "bad_group"
        );
        // Flagless group still bad.
        assert_eq!(
            split_hint_annotations("run $x #x --select-dir #y").unwrap_err(),
            "unknown_param"
        );
    }

    #[test]
    fn placeholder_defaults_captured() {
        let order = placeholder_defaults("cp ${x} ${y:-def} $x[]");
        assert_eq!(order.len(), 2);
        assert_eq!(order[0].0, "x");
        assert!(order[0].2.is_none());
        assert_eq!(order[1].0, "y");
        assert_eq!(order[1].2.as_deref(), Some("def"));
    }

    #[test]
    fn inline_values_parse_and_serialize() {
        // Bare --hint / --select with bracket lists.
        let (cmd, hints) =
            split_hint_annotations("ssh $p #p --hint [8080,38533,55555]").unwrap();
        assert_eq!(cmd, "ssh $p");
        assert_eq!(
            hints[0].kinds,
            vec![ParamHintKind::Values(vec![
                "8080".to_string(),
                "38533".to_string(),
                "55555".to_string()
            ])]
        );
        assert_eq!(
            serialize_hint_spec(&hints),
            "#p --hint [8080,38533,55555]"
        );
        // Quoted phrases with spaces survive.
        let (_, hints) =
            split_hint_annotations("echo $w #w --select [\"what is this\",omg]").unwrap();
        assert_eq!(
            hints[0].selects,
            vec![ParamSelectKind::Values(vec![
                "what is this".to_string(),
                "omg".to_string()
            ])]
        );
        assert_eq!(
            serialize_hint_spec(&hints),
            "#w --select [\"what is this\",omg]"
        );
        // Union with other kinds on the same param.
        let (_, hints) =
            split_hint_annotations("run $x #x --hint-dir --hint [build,dist]").unwrap();
        assert_eq!(hints[0].kinds.len(), 2);
        // Bare flag without a bracket span is malformed.
        assert_eq!(
            split_hint_annotations("run $x #x --hint").unwrap_err(),
            "bad_group"
        );
        assert_eq!(
            split_hint_annotations("run $x #x --hint dir").unwrap_err(),
            "bad_group"
        );
        assert_eq!(
            split_hint_annotations("run $x #x --select []").unwrap_err(),
            "bad_group"
        );
    }

    #[test]
    fn named_refs_parse_and_helpers() {
        let (cmd, hints) =
            split_hint_annotations("ssh $p #p --hint-n port").unwrap();
        assert_eq!(cmd, "ssh $p");
        assert_eq!(hints[0].kinds, vec![ParamHintKind::Named("port".to_string())]);
        assert_eq!(serialize_hint_spec(&hints), "#p --hint-n port");
        let (_, hints) =
            split_hint_annotations("run $x #x --select-dir --select-n port").unwrap();
        assert_eq!(
            hints[0].selects,
            vec![ParamSelectKind::Dir, ParamSelectKind::Named("port".to_string())]
        );
        // Names normalize to lowercase.
        let (_, hints) =
            split_hint_annotations("run $x #x --hint-n PORT").unwrap();
        assert_eq!(hints[0].kinds, vec![ParamHintKind::Named("port".to_string())]);
        // Missing name is malformed.
        assert_eq!(
            split_hint_annotations("run $x #x --hint-n").unwrap_err(),
            "bad_group"
        );
        // Spec helpers.
        assert_eq!(
            named_refs_in_groups(&hints),
            vec!["port".to_string()]
        );
        assert!(spec_references_named("#p --hint-n port", "PORT"));
        assert!(!spec_references_named("#p --hint-dir", "port"));
        assert_eq!(
            spec_referenced_lists("#x --hint-n port #y --select-n hosts --select-n port"),
            vec!["port".to_string(), "hosts".to_string()]
        );
        assert_eq!(
            rename_named_in_spec("#x --hint-n port #y --select-n port", "port", "ports"),
            "#x --hint-n ports #y --select-n ports"
        );
    }

    #[test]
    fn raw_value_lists_parse() {
        assert_eq!(
            parse_value_list_raw("[8080,5050,5000]").unwrap(),
            vec!["8080", "5050", "5000"]
        );
        assert_eq!(
            parse_value_list_raw("[8080, 5050 , 5000]").unwrap(),
            vec!["8080", "5050", "5000"]
        );
        // Quoted commas stay inside one value; quotes are stripped.
        assert_eq!(
            parse_value_list_raw("[\"that's great\", \"omg,nice\", ok]").unwrap(),
            vec!["that's great", "omg,nice", "ok"]
        );
        assert_eq!(
            parse_value_list_raw("['a b',c]").unwrap(),
            vec!["a b", "c"]
        );
        // Dedup, drop empties.
        assert_eq!(
            parse_value_list_raw("[a,,a, b]").unwrap(),
            vec!["a", "b"]
        );
        // Failures: no brackets, unbalanced, empty, trailing garbage.
        assert!(parse_value_list_raw("8080,5050").is_err());
        assert!(parse_value_list_raw("[8080,5050").is_err());
        assert!(parse_value_list_raw("[]").is_err());
        assert!(parse_value_list_raw("[a] extra").is_err());
    }

    #[test]
    fn sep_joins_with_spaces_around() {
        // $x[,] over [math.c, cat.c] -> "math.c , cat.c".
        assert_eq!(
            substitute_params("run $x[,]", &["[math.c,cat.c]"]),
            "run math.c , cat.c"
        );
        assert_eq!(
            substitute_params("run $x[;]", &["[math.c,cat.c]"]),
            "run math.c ; cat.c"
        );
        assert_eq!(
            substitute_params("run $x[.]", &["[math.c,cat.c]"]),
            "run math.c . cat.c"
        );
        // Legacy [] still space-joins.
        assert_eq!(
            substitute_params("run $x[]", &["[math.c,cat.c]"]),
            "run math.c cat.c"
        );
    }

    #[test]
    fn sep_applies_suffix_per_element_then_joins() {
        assert_eq!(
            substitute_params("gcc $x[,].c", &["[a,b]"]),
            "gcc a.c , b.c"
        );
        // Single (non-list) value passes through with suffix, no join.
        assert_eq!(substitute_params("run $x[,] ", &["solo"]), "run solo ");
        // Bracketed defaults split on commas too (plain defaults
        // stay one value, like legacy `[]`).
        assert_eq!(
            substitute_params("run ${x:-[a,b]}[,]", &[]),
            "run a , b"
        );
        assert_eq!(substitute_params("run ${x:-a,b}[,]", &[]), "run a,b");
        // Braced sep form.
        assert_eq!(
            substitute_params("run ${x}[,]", &["[a,b]"]),
            "run a , b"
        );
    }

    #[test]
    fn sep_multichar_stays_literal() {
        // `$x[ab]` is not array syntax: plain $x substitution (missing
        // here), brackets survive literally. `ral add` rejects these.
        assert_eq!(substitute_params("run $x[ab]", &[]), "run $x[ab]");
        assert_eq!(
            find_invalid_array_sep("run $x[ab]").unwrap(),
            ("$x[ab]".to_string(), "ab".to_string())
        );
        assert_eq!(
            find_invalid_array_sep("run ${o}[abc] $x[,] ").unwrap(),
            ("${o}[abc]".to_string(), "abc".to_string())
        );
        assert!(find_invalid_array_sep("run $x[,] $y[] ${z}").is_none());
        assert!(find_invalid_array_sep("run $x[] plain").is_none());
        assert!(find_invalid_array_sep("no placeholders").is_none());
    }

    #[test]
    fn sep_extract_display_and_inject() {
        // extract_param_names shows the sep form.
        let names = extract_param_names("run $x[,]");
        assert_eq!(names, vec!["x[,]".to_string()]);
        // inject preserves the array tail through default rewriting.
        let out = inject_template_defaults("run $x[,]", &[("x".into(), "a,b".into())]);
        assert_eq!(out, "run ${x:-a,b}[,]");
        let out = inject_template_defaults("run $x[].c", &[("x".into(), "a".into())]);
        assert_eq!(out, "run ${x:-a}[].c");
    }

    #[test]
    fn test_expand_command_line_empty() {
        let result = expand_command_line("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_expand_command_line_whitespace() {
        let result = expand_command_line("   ");
        assert!(result.is_empty());
    }
}
