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
        "if",
        "while",
        "for",
        "switch",
        "return",
        "case",
        "break",
        "continue",
        "default",
        "catch",
        "throw",
        "try",
        "sizeof",
        "delete",
        "new",
        "dynamic_cast",
    ];
    let skip_modifiers: [&str; 8] = [
        "static",
        "virtual",
        "override",
        "const",
        "extern",
        "private",
        "protected",
        "public",
    ];
    // Join a possibly multi-line signature starting at `li`: keeps
    // appending comment-stripped lines until the parens opened on the
    // first line balance (cap: 12 lines so a missing `)` can't swallow
    // the file). Returns the joined code, or `None` when unbalanced.
    fn join_signature(lines: &[String], li: usize) -> Option<String> {
        let first = lines[li].trim();
        let first = match first.split_once("//") {
            Some((c, _)) => c.trim(),
            None => first,
        };
        let mut code = first.to_string();
        let mut depth: i32 = 0;
        for ch in first.chars() {
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
            }
        }
        if depth <= 0 {
            return Some(code);
        }
        for next in lines.iter().skip(li + 1).take(11) {
            let t = next.trim();
            let t = match t.split_once("//") {
                Some((c, _)) => c.trim(),
                None => t,
            };
            if t.is_empty() {
                continue;
            }
            code.push(' ');
            code.push_str(t);
            for ch in t.chars() {
                if ch == '(' {
                    depth += 1;
                } else if ch == ')' {
                    depth -= 1;
                }
            }
            if depth <= 0 {
                return Some(code);
            }
        }
        None
    }
    // A `Name(...);` line is a prototype declaration only when it looks
    // like one: `Ret Name(...)` (return type present), no assignment or
    // member access before the paren (excludes call sites like
    // `x = fun(a);`, `o.run(x);`, `fun(a);`). Prototypes lose to real
    // definitions but beat having no signature at all.
    fn as_prototype(code: &str, paren: usize) -> bool {
        if !code.trim_end().ends_with(';') {
            return false;
        }
        let before = code[..paren].trim();
        if before.contains(['=', '.', '>', '<', '+', '-', '*', '/', '%', '!', '&', '|', '?', ':']) {
            return false;
        }
        before.split_whitespace().count() >= 2
    }
    let mut funcs: Vec<UserFunction> = Vec::new();
    // Names whose stored entry came from a prototype line: a later real
    // definition always replaces them.
    let mut proto_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    // (name, is_prototype): prototypes merge into / lose to definitions.
    let mut add = |name: &str, ret_type: String, raw_params: String, li: usize, proto: bool| {
        if let Some(f) = funcs.iter_mut().find(|f| f.name == name) {
            let stored_proto = proto_names.contains(name);
            // Preference: definition > prototype; then non-empty params.
            let want_new = match (stored_proto, proto) {
                (true, false) => true,
                (false, true) => false,
                _ => f.params.is_empty() && !raw_params.is_empty(),
            };
            if want_new {
                f.return_type = ret_type;
                f.params = raw_params;
                f.line = li;
                if proto {
                    proto_names.insert(name.to_string());
                } else {
                    proto_names.remove(name);
                }
            }
        } else {
            funcs.push(UserFunction {
                name: name.to_string(),
                return_type: ret_type,
                params: raw_params,
                line: li,
            });
            if proto {
                proto_names.insert(name.to_string());
            }
        }
    };
    for (li, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        let code = if let Some(pos) = trimmed.find("//") {
            &trimmed[..pos]
        } else {
            trimmed
        };
        let code = code.trim();
        let Some(paren) = code.find('(') else {
            continue;
        };
        let proto = as_prototype(code, paren);
        if !proto && trimmed.ends_with(';') {
            // Bare call site (`fun(a);`, `o.run(x);`, `x = fun(a);`) —
            // never a definition.
            continue;
        }
        // Definitions must balance on this line or within the following
        // lines (prototypes always balance here).
        let joined;
        let code: &str = if !proto && !code[paren..].contains(')') {
            match join_signature(lines, li) {
                Some(j) => {
                    joined = j;
                    &joined
                }
                None => continue,
            }
        } else {
            code
        };
        let paren = code.find('(').unwrap_or(0);
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
        if proto && ret_type.is_empty() {
            continue;
        }
        add(name, ret_type, raw_params, li, proto);
    }
    // In-class declarations (`O(int a);` inside `class O {…}`) are members,
    // never globals: drop them here so no consumer can leak them.
    // Likewise a prototype that collides with a same-named custom variable
    // of the same type (`Object object1();`) is an object construction,
    // not a function — the variable entry wins.
    if !funcs.is_empty() {
        let ranges = extract_class_ranges(lines);
        let mut user_types: Vec<String> = Vec::new();
        for (n, _) in extract_classes(lines) {
            user_types.push(n);
        }
        for (n, _) in extract_structs(lines) {
            user_types.push(n);
        }
        for e in extract_enums(lines) {
            if !e.name.is_empty() {
                user_types.push(e.name);
            }
        }
        let custom_vars = extract_custom_typed_variables(lines, &user_types);
        funcs.retain(|f| {
            if in_class_body(&ranges, f.line) {
                return false;
            }
            if proto_names.contains(&f.name)
                && custom_vars
                    .iter()
                    .any(|v| v.name == f.name && v.type_name == f.return_type)
            {
                return false;
            }
            true
        });
    }
    funcs
}

pub(crate) struct VariableInfo {
    pub(crate) name: String,
    pub(crate) type_name: String,
    pub(crate) line: usize,
}

pub(crate) fn extract_variables(lines: &[String]) -> Vec<VariableInfo> {
    let types = type_names();
    let mut vars = Vec::new();

    for (li, line) in lines.iter().enumerate() {
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

        let code = if let Some(rest) = code.strip_prefix("input ") {
            rest
        } else if let Some(rest) = code.strip_prefix("static ") {
            rest
        } else if let Some(rest) = code.strip_prefix("const ") {
            rest
        } else {
            code
        };

        let code = code.trim();
        if !code.ends_with(';') {
            continue;
        }

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
                    line: li,
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

/// A brace-matched `(name, first_line, last_line)` class/struct body range.
fn body_ranges(lines: &[String], keyword: &str) -> Vec<(String, usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            i += 1;
            continue;
        }
        // Callers pass the keyword with its trailing space, so a match
        // here already guarantees a real declaration header.
        let rest = match trimmed.strip_prefix(keyword) {
            Some(r) => r.trim(),
            _ => {
                i += 1;
                continue;
            }
        };
        let name = rest
            .split(|c: char| c.is_whitespace() || c == '{' || c == ':' || c == ';')
            .next()
            .unwrap_or("")
            .to_string();
        if name.is_empty()
            || !name.starts_with(|c: char| c.is_alphabetic() || c == '_')
        {
            i += 1;
            continue;
        }
        // Scan to the matching close brace (strings masked, // cut).
        let mut depth = 0usize;
        let mut opened = false;
        let mut j = i;
        let mut end = i;
        let mut closed = false;
        while j < lines.len() {
            let masked = mask_code_line(&lines[j]);
            for ch in masked.chars() {
                if ch == '{' {
                    depth += 1;
                    opened = true;
                } else if ch == '}' {
                    depth = depth.saturating_sub(1);
                    if opened && depth == 0 {
                        end = j;
                        closed = true;
                        break;
                    }
                }
            }
            if closed {
                break;
            }
            j += 1;
            // Unbalanced bodies (still being typed) run to EOF: still usable.
            if j >= lines.len() {
                end = lines.len() - 1;
                break;
            }
        }
        if opened {
            out.push((name, i, end));
            i = end + 1;
        } else {
            // Forward declaration (`class Foo;`): no body to offer.
            i += 1;
        }
    }
    out
}

/// Mask `"…"` strings and cut `//` comments (offsets preserved).
fn mask_code_line(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    let mut in_str = false;
    while let Some(ch) = chars.next() {
        if in_str {
            if ch == '\\' {
                out.push(' ');
                if chars.next().is_some() {
                    out.push(' ');
                }
            } else if ch == '"' {
                in_str = false;
                out.push(' ');
            } else {
                out.push(' ');
            }
        } else if ch == '"' {
            in_str = true;
            out.push(' ');
        } else if ch == '/' && chars.peek() == Some(&'/') {
            break;
        } else {
            out.push(ch);
        }
    }
    out
}

/// Line ranges of every class/struct body: `(name, first, last)` inclusive.
pub(crate) fn extract_class_ranges(lines: &[String]) -> Vec<(String, usize, usize)> {
    let mut ranges = body_ranges(lines, "class ");
    ranges.extend(body_ranges(lines, "struct "));
    ranges
}

/// True when `line` sits inside any class/struct body.
pub fn in_class_body(ranges: &[(String, usize, usize)], line: usize) -> bool {
    ranges.iter().any(|(_, s, e)| line >= *s && line <= *e)
}

/// One member of a class/struct: method or variable.
#[derive(Debug, Clone, PartialEq)]
pub struct MemberInfo {
    pub(crate) name: String,
    pub(crate) detail: String,
    pub(crate) params: String,
    pub(crate) return_type: String,
    pub(crate) is_method: bool,
    pub(crate) is_private: bool,
    pub(crate) line: usize,
}

/// Full class/struct info with parsed members.
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub(crate) name: String,
    pub(crate) methods: Vec<MemberInfo>,
    pub(crate) vars: Vec<MemberInfo>,
}

/// Parse member declarations inside one body range.
fn parse_body_members(
    lines: &[String],
    start: usize,
    end: usize,
    class_name: &str,
    methods: &mut Vec<MemberInfo>,
    vars: &mut Vec<MemberInfo>,
) {
    // Access sections: MQL defaults to private before any label.
    let mut private = true;
    // Brace nesting relative to the class body: lines inside a member
    // function body (e.g. a call like `other();` within `fun(){...}`)
    // are statements, not member declarations, and must be skipped so
    // they never leak into `object.` member completion.
    let mut nest = 0usize;
    // True after a member declaration with no `{` on its line
    // (`void run(int x);` or K&R `void fun()`): a `{`-leading line next
    // opens that member's body rather than the class body itself.
    let mut pending_fn_brace = false;
    let mut i = start;
    while i <= end.min(lines.len().saturating_sub(1)) {
        let trimmed = lines[i].trim();
        let masked = mask_code_line(&lines[i]);
        let opens = masked.chars().filter(|&c| c == '{').count();
        let closes = masked.chars().filter(|&c| c == '}').count();
        let idx = i;
        i += 1;
        if nest > 0 {
            nest = nest.saturating_add(opens).saturating_sub(closes);
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            nest = nest.saturating_add(opens).saturating_sub(closes);
            continue;
        }
        if idx == start {
            // Class/struct header line (`class O {`): its braces delimit
            // the body itself, not a nested scope.
            continue;
        }
        if masked.trim_start().starts_with('{') {
            // Lone `{` at class level: K&R member-body open when a
            // braceless member declaration precedes it, else the class
            // body open (or stray) — never a nested scope by itself.
            if pending_fn_brace {
                nest = nest.saturating_add(opens).saturating_sub(closes);
                pending_fn_brace = false;
            }
            continue;
        }
        let low = trimmed.to_lowercase();
        if low == "private:" {
            private = true;
            pending_fn_brace = false;
            continue;
        }
        if low == "public:" || low == "protected:" {
            private = false;
            pending_fn_brace = false;
            continue;
        }
        let code = trimmed;
        let code = match code.split_once("//") {
            Some((c, _)) => c.trim(),
            None => code,
        };
        if code.is_empty() {
            nest = nest.saturating_add(opens).saturating_sub(closes);
            continue;
        }
        if let Some(paren) = code.find('(') {
            // Method declaration or inline definition.
            if !code[paren..].contains(')') {
                nest = nest.saturating_add(opens).saturating_sub(closes);
                continue; // multi-line signature: out of scope v1
            }
            let before = code[..paren].trim();
            let mname = before.split_whitespace().last().unwrap_or("");
            if mname.is_empty()
                || !mname.starts_with(|c: char| c.is_alphabetic() || c == '_')
            {
                nest = nest.saturating_add(opens).saturating_sub(closes);
                continue;
            }
            // Skip constructor-ish noise that isn't a name (e.g. `if (`).
            if ["if", "for", "while", "switch", "return"].contains(&mname) {
                nest = nest.saturating_add(opens).saturating_sub(closes);
                continue;
            }
            let after = &code[paren..];
            let close = after.find(')').unwrap_or(after.len());
            let params = after[1..close].trim().to_string();
            // Return type: tokens before the name, minus modifiers.
            let ret_tokens: Vec<&str> = before
                .split_whitespace()
                .rev()
                .skip(1)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .filter(|t| {
                    ![
                        "static", "virtual", "override", "const", "extern", "private",
                        "protected", "public",
                    ]
                    .contains(t)
                })
                .collect();
            let ret = ret_tokens.join(" ");
            let detail = if ret.is_empty() {
                format!("{}::{}({})", class_name, mname, params)
            } else {
                format!("{} {}::{}({})", ret, class_name, mname, params)
            };
            if !methods.iter().any(|m: &MemberInfo| m.name == mname) {
                methods.push(MemberInfo {
                    name: mname.to_string(),
                    detail,
                    params,
                    return_type: ret.clone(),
                    is_method: true,
                    is_private: private,
                    line: i,
                });
            }
            if opens > 0 {
                // Inline body open on the declaration line itself
                // (`void fun(){...`): track the nested scope.
                nest = nest.saturating_add(opens).saturating_sub(closes);
                pending_fn_brace = false;
            } else {
                // Braceless declaration (`void run(int x);`) or K&R
                // definition (`void fun()` with `{` on the next line).
                pending_fn_brace = true;
            }
            continue;
        } else if code.ends_with(';') {
            // Member variable(s): `type a, b = 1;`
            let first = code.split_whitespace().next().unwrap_or("");
            if first.is_empty() {
                nest = nest.saturating_add(opens).saturating_sub(closes);
                continue;
            }
            let rest = code[first.len()..].trim().trim_end_matches(';').trim();
            for part in rest.split(',') {
                let decl = part.split('=').next().unwrap_or("").trim();
                let vname = decl
                    .split(|c: char| c == '[' || c.is_whitespace())
                    .next()
                    .unwrap_or("")
                    .trim();
                if vname.is_empty()
                    || !vname.starts_with(|c: char| c.is_alphabetic() || c == '_')
                    || !vname.chars().all(|c| c.is_alphanumeric() || c == '_')
                {
                    continue;
                }
                if !vars.iter().any(|v: &MemberInfo| v.name == vname) {
                    vars.push(MemberInfo {
                        name: vname.to_string(),
                        detail: format!("{} {}::{}", first, class_name, vname),
                        params: String::new(),
                        return_type: String::new(),
                        is_method: false,
                        is_private: private,
                        line: i,
                    });
                }
            }
        }
        // Member-variable and fall-through lines (`};`, stray text):
        // a substantive line ends any pending K&R expectation.
        pending_fn_brace = false;
        nest = nest.saturating_add(opens).saturating_sub(closes);
    }
}

/// Extract classes AND structs with their members, plus out-of-body
/// `Ret Class::Method(params)` definitions merged in.
pub(crate) fn extract_classes_full(lines: &[String]) -> Vec<ClassInfo> {
    let mut infos: Vec<ClassInfo> = Vec::new();
    for (name, start, end) in extract_class_ranges(lines) {
        let mut methods = Vec::new();
        let mut vars = Vec::new();
        parse_body_members(lines, start, end, &name, &mut methods, &mut vars);
        if !infos.iter().any(|c: &ClassInfo| c.name == name) {
            infos.push(ClassInfo {
                name,
                methods,
                vars,
            });
        }
    }
    // Out-of-body definitions: `Ret Class::Method(params) ...`.
    for (li, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        let Some(dc) = trimmed.find("::") else {
            continue;
        };
        let left = trimmed[..dc].trim();
        let class_name = left.split_whitespace().last().unwrap_or("");
        if class_name.is_empty() {
            continue;
        }
        let after = trimmed[dc + 2..].trim();
        let Some(paren) = after.find('(') else {
            continue;
        };
        if !after[paren..].contains(')') {
            continue;
        }
        let mname = after[..paren].trim();
        if mname.is_empty()
            || !mname.starts_with(|c: char| c.is_alphabetic() || c == '_')
        {
            continue;
        }
        let close = after[paren..].find(')').unwrap_or(0);
        let params = after[paren + 1..paren + close].trim().to_string();
        if let Some(info) = infos.iter_mut().find(|c| c.name == class_name) {
            if !info.methods.iter().any(|m| m.name == mname) {
                let ret = left[..left.len() - class_name.len()].trim().to_string();
                info.methods.push(MemberInfo {
                    name: mname.to_string(),
                    detail: if ret.is_empty() {
                        format!("{}::{}({})", class_name, mname, params)
                    } else {
                        format!("{} {}::{}({})", ret, class_name, mname, params)
                    },
                    params,
                    return_type: ret,
                    is_method: true,
                    is_private: false,
                    line: li,
                });
            }
        }
    }
    infos
}

/// Variables declared with a user-defined (class/struct/enum) type:
/// `MyClass obj;` or `MyClass obj(args);`.
pub(crate) fn extract_custom_typed_variables(
    lines: &[String],
    type_names: &[String],
) -> Vec<VariableInfo> {
    let mut vars = Vec::new();
    for (li, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        let code = match trimmed.split_once("//") {
            Some((c, _)) => c.trim(),
            None => trimmed,
        };
        if code.is_empty() {
            continue;
        }
        let first = code.split_whitespace().next().unwrap_or("");
        if !type_names.iter().any(|t| t == first) {
            continue;
        }
        // Remainder: `name;`, `name(args);` or `a, b;`.
        let mut rest = code[first.len()..].trim();
        rest = rest.trim_end_matches(';').trim();
        if let Some(paren) = rest.find('(') {
            // Constructor-style `obj(args)`: keep the name part.
            let before = rest[..paren].trim();
            if before.contains([' ', ',', '=']) {
                continue;
            }
            rest = before;
        }
        for part in rest.split(',') {
            let decl = part.split('=').next().unwrap_or("").trim();
            let decl = decl.split('[').next().unwrap_or("").trim();
            // A declaration is exactly one identifier; anything with
            // inner whitespace (`bad name`) is not a variable.
            if decl.is_empty() || decl.contains(char::is_whitespace) {
                continue;
            }
            let vname = decl;
            if vname.is_empty()
                || !vname.starts_with(|c: char| c.is_alphabetic() || c == '_')
                || !vname.chars().all(|c| c.is_alphanumeric() || c == '_')
            {
                continue;
            }
            if !vars.iter().any(|v: &VariableInfo| v.name == vname) {
                vars.push(VariableInfo {
                    name: vname.to_string(),
                    type_name: first.to_string(),
                    line: li,
                });
            }
        }
    }
    vars
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

/// One `enum Name { A, B = 1, ... };` declaration with its members.
pub(crate) struct EnumInfo {
    pub(crate) name: String,
    pub(crate) members: Vec<String>,
}

fn clean_enum_member(raw: &str) -> Option<String> {
    // Strip trailing comment, assigned value and separators.
    let no_comment = raw.split("//").next().unwrap_or("").trim();
    let no_value = no_comment.split('=').next().unwrap_or("").trim();
    let name = no_value
        .trim_matches(|c: char| c == ',' || c == ';' || c == '{' || c == '}')
        .trim();
    if name.is_empty() || !name.starts_with(|c: char| c.is_alphabetic() || c == '_') {
        return None;
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    Some(name.to_string())
}

/// Extract `enum` declarations (names + members). Bodies may span lines.
pub(crate) fn extract_enums(lines: &[String]) -> Vec<EnumInfo> {
    let mut enums = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            i += 1;
            continue;
        }
        let rest = match trimmed.strip_prefix("enum ") {
            Some(r) => r.trim(),
            None => {
                i += 1;
                continue;
            }
        };
        // Name is the first token unless the brace opens immediately.
        let name = rest
            .split(|c: char| c.is_whitespace() || c == '{' || c == ';')
            .next()
            .unwrap_or("")
            .trim_end_matches(&['{', ':'][..])
            .to_string();
        // Collect members until the closing brace. A declaration with
        // no brace at all (`enum Foo;`) is a forward declaration: skip.
        // A brace on the next non-empty line (K&R style) still counts.
        let mut members: Vec<String> = Vec::new();
        let mut j = i;
        let mut brace_seen = rest.contains('{');
        // Same-line body after `{`, if any.
        let mut body = if brace_seen {
            let pos = rest.find('{').unwrap_or(0);
            rest[pos + 1..].to_string()
        } else {
            String::new()
        };
        if !brace_seen {
            let mut k = i + 1;
            while k < lines.len() && lines[k].trim().is_empty() {
                k += 1;
            }
            if k < lines.len() && lines[k].trim_start().starts_with('{') {
                brace_seen = true;
                j = k;
                body = lines[k].trim().to_string();
            }
        }
        loop {
            for part in body.split(',') {
                if let Some(m) = clean_enum_member(part) {
                    if !members.contains(&m) {
                        members.push(m);
                    }
                }
            }
            if body.contains('{') {
                brace_seen = true;
            }
            if body.contains('}') {
                break;
            }
            j += 1;
            if j >= lines.len() || !brace_seen {
                if !brace_seen {
                    j = i; // rewind: nothing consumed
                }
                break;
            }
            body = lines[j].trim().to_string();
        }
        if !brace_seen {
            i += 1;
            continue;
        }
        if !name.is_empty() && !enums.iter().any(|e: &EnumInfo| e.name == name) {
            enums.push(EnumInfo { name, members });
        } else if name.is_empty() {
            // Anonymous enum: still offer its members under no name.
            if !members.is_empty()
                && !enums.iter().any(|e: &EnumInfo| e.name.is_empty())
            {
                enums.push(EnumInfo {
                    name: String::new(),
                    members,
                });
            }
        }
        i = j + 1;
    }
    enums
}

/// One `#define NAME [value]` macro definition.
pub(crate) struct DefineInfo {
    pub(crate) name: String,
    pub(crate) value: String,
}

/// Extract `#define` macros (plain and function-like `NAME(...)`).
pub(crate) fn extract_defines(lines: &[String]) -> Vec<DefineInfo> {
    let mut defines = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        let rest = match trimmed.strip_prefix("#define") {
            Some(r) => r.trim(),
            None => continue,
        };
        if rest.is_empty() {
            continue;
        }
        // Name ends at whitespace or `(` (function-like macro).
        let name: String = rest
            .chars()
            .take_while(|c| !c.is_whitespace() && *c != '(')
            .collect();
        if name.is_empty()
            || !name.starts_with(|c: char| c.is_alphabetic() || c == '_')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        // Value is whatever follows the name (and its parameter list).
        let mut value = rest[name.len()..].trim();
        if value.starts_with('(') {
            if let Some(end) = value.find(')') {
                value = value[end + 1..].trim();
            } else {
                value = "";
            }
        }
        if let Some((code, _)) = value.split_once("//") {
            value = code.trim();
        }
        if !defines.iter().any(|d: &DefineInfo| d.name == name) {
            defines.push(DefineInfo {
                name,
                value: value.to_string(),
            });
        }
    }
    defines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sl(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn enums_simple_and_multiline() {
        let lines = sl(&[
            "enum Color { RED, GREEN = 2, BLUE };",
            "enum Timeframe {",
            "  M1, // one minute",
            "  H1",
            "};",
            "// enum Fake { NOPE };",
        ]);
        let enums = extract_enums(&lines);
        assert_eq!(enums.len(), 2);
        assert_eq!(enums[0].name, "Color");
        assert_eq!(enums[0].members, vec!["RED", "GREEN", "BLUE"]);
        assert_eq!(enums[1].name, "Timeframe");
        assert_eq!(enums[1].members, vec!["M1", "H1"]);
    }

    #[test]
    fn defines_plain_valued_and_funclike() {       let lines = sl(&[
            "#define MAXLOT 10",
            "#define WINDOW \"main\" // main window",
            "#define SQR(x) ((x)*(x))",
            "#define BARE",
            "#define BARE // dup",
            "#define 9BAD 1",
        ]);
        let defines = extract_defines(&lines);
        assert_eq!(defines.len(), 4);
        assert_eq!(defines[0].name, "MAXLOT");
        assert_eq!(defines[0].value, "10");
        assert_eq!(defines[1].value, "\"main\"");
        assert_eq!(defines[2].name, "SQR");
        assert_eq!(defines[2].value, "((x)*(x))");
        assert_eq!(defines[3].name, "BARE");
        assert_eq!(defines[3].value, "");
    }

    #[test]
    fn class_ranges_members_and_access() {
        let lines = sl(&[
            "class O {",
            "  int hidden;",
            "public:",
            "  O(int a, double b);",
            "  void run(int x);",
            "  string name;",
            "};",
            "void O::run(int x) { }",
            "int foo;",
        ]);
        let ranges = extract_class_ranges(&lines);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].0, "O");
        assert!(in_class_body(&ranges, 3));
        assert!(!in_class_body(&ranges, 8));
        let infos = extract_classes_full(&lines);
        assert_eq!(infos.len(), 1);
        // constructor + run (in-body decl merged, not duplicated by O:: def)
        let methods: Vec<&str> = infos[0].methods.iter().map(|m| m.name.as_str()).collect();
        assert!(methods.contains(&"O"));
        assert!(methods.contains(&"run"));
        assert_eq!(methods.iter().filter(|m| **m == "run").count(), 1);
        let run = infos[0].methods.iter().find(|m| m.name == "run").unwrap();
        assert_eq!(run.params, "int x");
        assert!(!run.is_private);
        let hidden = infos[0].vars.iter().find(|v| v.name == "hidden").unwrap();
        assert!(hidden.is_private);
        // in-class functions must not leak to the global pool
        let ranges = extract_class_ranges(&lines);
        let globals = extract_user_functions(&lines);
        assert!(globals.iter().all(|f| !in_class_body(&ranges, f.line)));
    }

    #[test]
    fn custom_typed_variables() {
        let lines = sl(&["O o;", "O p(1, 2);", "int x;", "O bad name;"]);
        let vars = extract_custom_typed_variables(&lines, &["O".to_string()]);
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].name, "o");
        assert_eq!(vars[0].type_name, "O");
        assert_eq!(vars[1].name, "p");
    }

    #[test]
    fn body_statements_are_not_members() {
        // A call statement inside a member body must not become a member:
        // `object.` completion must not offer `other_outside_class_fun`.
        let lines = sl(&[
            "void other_outside_class_fun() {}",
            "class Object {",
            "public:",
            "  Object();",
            "  void sub_fun(int x);",
            "  void fun(){",
            "    other_outside_class_fun();",
            "  }",
            "};",
        ]);
        let infos = extract_classes_full(&lines);
        assert_eq!(infos.len(), 1);
        let methods: Vec<&str> = infos[0].methods.iter().map(|m| m.name.as_str()).collect();
        assert!(methods.contains(&"Object"), "{:?}", methods);
        assert!(methods.contains(&"sub_fun"), "{:?}", methods);
        assert!(methods.contains(&"fun"), "{:?}", methods);
        assert!(
            !methods.contains(&"other_outside_class_fun"),
            "in-body call leaked: {:?}",
            methods
        );
    }

    #[test]
    fn user_functions_prefer_param_definitions() {
        // An early no-param stub must not shadow the real definition.
        let lines = sl(&[
            "void fun(){ implemented; }",
            "void fun(int a, int b){ implemented; }",
        ]);
        let funcs = extract_user_functions(&lines);
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "fun");
        assert_eq!(funcs[0].params, "int a, int b");
    }

    #[test]
    fn user_functions_multiline_signatures() {
        let lines = sl(&[
            "void fun(int a ,",
            "         int b)",
            "{ implemented; }",
            "fun(a, b);",
        ]);
        let funcs = extract_user_functions(&lines);
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "fun");
        assert_eq!(funcs[0].return_type, "void");
        assert_eq!(funcs[0].params, "int a , int b");
    }

    #[test]
    fn user_functions_prototypes_merge_but_calls_do_not() {
        let lines = sl(&[
            "void proto(int q);",
            "proto(1);",
            "x = proto(2);",
            "o.run(3);",
            "void proto(int q) { implemented; }",
        ]);
        let funcs = extract_user_functions(&lines);
        // Prototype + definition merge into one entry; call sites
        // (`proto(1);`, `x = proto(2);`, `o.run(3);`) add nothing.
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "proto");
        assert_eq!(funcs[0].return_type, "void");
        assert_eq!(funcs[0].params, "int q");
    }

    #[test]
    fn user_functions_prototype_only_still_resolves() {
        let lines = sl(&["int only_proto(double d);", "only_proto(1.5);"]);
        let funcs = extract_user_functions(&lines);
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].return_type, "int");
        assert_eq!(funcs[0].params, "double d");
    }
}
