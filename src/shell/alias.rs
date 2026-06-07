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
pub(super) fn parse_call_syntax(token: &str) -> (&str, Option<&str>) {
    if let Some(paren_start) = token.find('(') {
        if token.ends_with(')') {
            let base = &token[..paren_start];
            let arg  = &token[paren_start + 1..token.len() - 1];
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
    default_val: Option<String>,
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
            if chars[i] == ':' && i + 1 < chars.len() && chars[i + 1] == '-' && colon_pos.is_none() {
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
        if i >= chars.len() || !(chars[i].is_ascii_lowercase() || chars[i].is_ascii_digit() || chars[i] == '_') {
            return None;
        }
        let start = i;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
            i += 1;
        }
        let name: String = chars[start..i].iter().collect();
        (name, i, None)
    };

    // Check for array expansion suffix: []suffix
    if end + 1 < chars.len() && chars[end] == '[' && chars[end + 1] == ']' {
        let suffix_start = end + 2;
        let mut suffix_end = suffix_start;
        while suffix_end < chars.len()
            && !chars[suffix_end].is_whitespace()
            && chars[suffix_end] != '&'
            && chars[suffix_end] != ';'
        {
            suffix_end += 1;
        }
        let suffix: String = chars[suffix_start..suffix_end].iter().collect();
        Some(VarRef { name, end: suffix_end, array_suffix: Some(suffix), default_val })
    } else {
        Some(VarRef { name, end, array_suffix: None, default_val })
    }
}

fn substitute_params(template: &str, args: &[&str]) -> String {
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
                if let Some(ref suffix) = vr.array_suffix {
                    // Array expansion: $name[]suffix or ${name}[]suffix
                    if let Some(val) = map.get(vr.name.as_str()) {
                        let elements: Vec<&str> = if val.starts_with('[') && val.ends_with(']') && val.len() >= 2 {
                            val[1..val.len()-1].split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
                        } else {
                            vec![val]
                        };
                        let expanded: Vec<String> = elements.iter().map(|e| format!("{}{}", e, suffix)).collect();
                        result.push_str(&expanded.join(" "));
                    } else if let Some(ref default) = vr.default_val {
                        let elements: Vec<&str> = if default.starts_with('[') && default.ends_with(']') && default.len() >= 2 {
                            default[1..default.len()-1].split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
                        } else {
                            vec![default.as_str()]
                        };
                        let expanded: Vec<String> = elements.iter().map(|e| format!("{}{}", e, suffix)).collect();
                        result.push_str(&expanded.join(" "));
                    } else {
                        result.push('$');
                        result.push_str(&vr.name);
                        result.push_str("[]");
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
pub(super) fn extract_param_names(template: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some(ref vr) = parse_var_ref(&chars, i) {
                let base_name = if vr.array_suffix.is_some() {
                    format!("{}[]", vr.name)
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
    hint.split(',').filter_map(|part| {
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
    }).collect()
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
                        // Replace with ${name:-default}
                        result.push_str("${");
                        result.push_str(&vr.name);
                        result.push_str(":-");
                        result.push_str(def_val);
                        result.push('}');
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

/// Recursively expand aliases in a command (no && splitting).
/// Supports parameterised calls: `py(hello)` with template `python $x.py` -> `python hello.py`
#[allow(clippy::only_used_in_recursion)]
fn expand_aliases(cmd: &str, depth: usize) -> String {
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
        expand_aliases(&new_cmd, depth + 1)
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
        let expanded = expand_aliases(&part, 0);
        if contains_ampersands(&expanded) {
            result.extend(expand_command_line(&expanded));
        } else if !expanded.trim().is_empty() {
            result.push(expanded);
        }
    }
    result
}