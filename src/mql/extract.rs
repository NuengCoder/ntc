use crate::mql::type_names;

pub(crate) fn extract_comment_box(lines: &[String], func_line: usize) -> Option<String> {
    if func_line == 0 || func_line >= lines.len() {
        return None;
    }
    let mut i = func_line;
    while i > 0 {
        i -= 1;
        let trimmed = lines[i].trim();
        if trimmed.is_empty() || trimmed == "{" {
            continue;
        }
        break;
    }
    if !lines[i].trim_start().starts_with("//+") {
        return None;
    }
    let mut content: Vec<String> = Vec::new();
    while i > 0 {
        i -= 1;
        let trimmed = lines[i].trim();
        if trimmed.starts_with("//|") {
            let rest = &trimmed[3..];
            if let Some(end) = rest.rfind('|') {
                let raw = rest[..end].trim();
                if !raw.is_empty() {
                    content.push(raw.to_string());
                }
            }
        } else if trimmed.starts_with("//+") {
            break;
        } else {
            return None;
        }
    }
    if content.is_empty() {
        return None;
    }
    content.reverse();
    Some(content.join("\n"))
}

pub(crate) struct UserFunction {
    pub(crate) name: String,
    pub(crate) return_type: String,
    pub(crate) params: String,
    pub(crate) line: usize,
}

pub(crate) fn extract_user_functions(lines: &[String]) -> Vec<UserFunction> {
    let skip_keywords: [&str; 16] = [
        "if", "while", "for", "switch", "return",
        "case", "break", "continue", "default",
        "catch", "throw", "try",
        "sizeof", "delete", "new", "dynamic_cast",
    ];
    let skip_modifiers: [&str; 8] = [
        "static", "virtual", "override", "const",
        "extern", "private", "protected", "public",
    ];
    let mut funcs = Vec::new();
    for (li, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.ends_with(';') {
            continue;
        }
        let code = if let Some(pos) = trimmed.find("//") {
            &trimmed[..pos]
        } else {
            trimmed
        };
        let code = code.trim();
        if let Some(paren) = code.find('(') {
            if !code[paren..].contains(')') {
                continue;
            }
            let before_paren = code[..paren].trim();
            let name = before_paren.split_whitespace().last().unwrap_or("");
            if name.is_empty()
                || !name.starts_with(|c: char| c.is_alphabetic() || c == '_')
                || skip_keywords.contains(&name)
            {
                continue;
            }
            let after_paren = &code[paren..];
            let close = after_paren.find(')').unwrap_or(after_paren.len());
            let raw_params = after_paren[1..close].trim().to_string();
            // Extract return type: everything before the name
            let return_tokens: Vec<&str> = before_paren.split_whitespace().collect();
            let ret_type = if return_tokens.len() > 1 {
                return_tokens[..return_tokens.len() - 1]
                    .iter()
                    .filter(|t| !skip_modifiers.contains(t))
                    .copied()
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                String::new()
            };
            if !funcs.iter().any(|f: &UserFunction| f.name == name) {
                funcs.push(UserFunction {
                    name: name.to_string(),
                    return_type: ret_type,
                    params: raw_params,
                    line: li,
                });
            }
        }
    }
    funcs
}

pub(crate) struct VariableInfo {
    pub(crate) name: String,
    pub(crate) type_name: String,
}

pub(crate) fn extract_variables(lines: &[String]) -> Vec<VariableInfo> {
    let types = type_names();
    let mut vars = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }

        let code = if let Some(pos) = trimmed.find("//") {
            &trimmed[..pos]
        } else {
            trimmed
        };
        let code = code.trim();
        if code.is_empty() || !code.ends_with(';') {
            continue;
        }

        let code = if let Some(rest) = code.strip_prefix("input ") { rest }
            else if let Some(rest) = code.strip_prefix("static ") { rest }
            else if let Some(rest) = code.strip_prefix("const ") { rest }
            else { code };

        let code = code.trim();
        if !code.ends_with(';') { continue; }

        let first = match code.split_whitespace().next() {
            Some(f) => f,
            None => continue,
        };

        let type_name = match types.iter().find(|t| t.eq_ignore_ascii_case(first)) {
            Some(t) => t,
            None => continue,
        };

        let after_type = code[first.len()..].trim().trim_end_matches(';').trim();

        for part in after_type.split(',') {
            let part = part.trim();
            let decl_part = part.split('=').next().unwrap_or("").trim();
            let bracket_dims = decl_part.matches("[]").count();
            let var_name = decl_part
                .split(|c: char| c == '[' || c == '=' || c == ',' || c == ';')
                .next()
                .map(|s| s.trim())
                .unwrap_or("");

            if !var_name.is_empty()
                && var_name.starts_with(|c: char| c.is_alphabetic() || c == '_')
                && !vars.iter().any(|v: &VariableInfo| v.name == var_name)
            {
                let display_type = if bracket_dims > 0 {
                    format!("{}{}", type_name, "[]".repeat(bracket_dims))
                } else {
                    type_name.to_string()
                };
                vars.push(VariableInfo {
                    name: var_name.to_string(),
                    type_name: display_type,
                });
            }
        }
    }

    vars
}

pub(crate) fn extract_classes(lines: &[String]) -> Vec<(String, usize)> {
    let mut classes: Vec<(String, usize)> = Vec::new();
    for (li, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("class ") {
            let name = rest
                .split_whitespace()
                .next()
                .map(|s| s.trim_end_matches(&['{', ':'][..]))
                .unwrap_or("");
            if !name.is_empty() && !classes.iter().any(|(n, _)| n == name) {
                classes.push((name.to_string(), li));
            }
        }
    }
    classes
}

pub(crate) fn extract_structs(lines: &[String]) -> Vec<(String, usize)> {
    let mut structs: Vec<(String, usize)> = Vec::new();
    for (li, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("struct ") {
            let name = rest
                .split_whitespace()
                .next()
                .map(|s| s.trim_end_matches(&['{', ':'][..]))
                .unwrap_or("");
            if !name.is_empty() && !structs.iter().any(|(n, _)| n == name) {
                structs.push((name.to_string(), li));
            }
        }
    }
    structs
}

pub(crate) fn extract_includes(lines: &[String]) -> Vec<String> {
    let mut includes = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        let path = if let Some(rest) = trimmed.strip_prefix("#include ") {
            rest.trim()
        } else {
            continue;
        };
        let name = if (path.starts_with('"') || path.starts_with('<'))
            && (path.ends_with('"') || path.ends_with('>'))
        {
            let inner = &path[1..path.len() - 1];
            let stem = if let Some(dot) = inner.rfind('.') {
                &inner[..dot]
            } else {
                inner
            };
            stem.to_string()
        } else {
            continue;
        };
        if !name.is_empty() && !includes.contains(&name) {
            includes.push(name);
        }
    }
    includes
}
