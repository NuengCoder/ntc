//! VSCode-style completion list for MQL4/MQL5 (`.mq4/.mq5/.mqh`).
//!
//! Two fixed boxes: Box1 (candidates, 12 rows, kind tags) left, Box2
//! (detail of the current selection) right, one line below the cursor.
//! After accepting `name(`, a Box2-only signature-help floats one line
//! above the cursor with the active parameter highlighted.
//!
//! Candidate priority (round-robin passes, two per kind max out at 12):
//! `F`unction > `V`ar/type > `A`rray > `K`eyword/constant > `O`bject(class)
//! > `S`truct > `E`num > `M`acro. User symbols sort before builtins.

use crate::mql::{
    builtin_function_names, constant_names, dialect_for_ext, extract_class_ranges, extract_classes,
    extract_classes_full, extract_comment_box, extract_custom_typed_variables, extract_defines,
    extract_enums, extract_structs, extract_user_functions, extract_variables, hover_info,
    hover_info_detailed, in_class_body, keyword_names, type_names, visible_in_dialect, Dialect,
};
use crate::syntax::SyntaxLanguage;

use crate::editor::{Editor, Mode};

/// Visible rows in Box1 and column widths (fixed: never resize).
pub const COMPLETE_ROWS: usize = 12;
pub const BOX1_WIDTH: usize = 34;
pub const BOX2_WIDTH: usize = 46;

/// Candidate kind with its `[X]` tag and fill priority (lower first).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Function,
    Var,
    Array,
    Keyword,
    Object,
    Struct,
    Enum,
    Macro,
}

impl Kind {
    pub fn tag(self) -> &'static str {
        match self {
            Kind::Function => "[F]",
            Kind::Var => "[V]",
            Kind::Array => "[A]",
            Kind::Keyword => "[K]",
            Kind::Object => "[O]",
            Kind::Struct => "[S]",
            Kind::Enum => "[E]",
            Kind::Macro => "[M]",
        }
    }

    fn priority(self) -> u8 {
        match self {
            Kind::Function => 0,
            Kind::Var => 1,
            Kind::Array => 2,
            Kind::Keyword => 3,
            Kind::Object => 4,
            Kind::Struct => 5,
            Kind::Enum => 6,
            Kind::Macro => 7,
        }
    }
}

/// One rendered candidate row.
#[derive(Debug, Clone)]
pub struct Item {
    /// Display label (`fun()`, `arr[]`, `foo`).
    pub label: String,
    /// Bare name for insertion filtering.
    pub name: String,
    pub kind: Kind,
    /// One-line detail for Box2.
    pub detail: String,
    /// Extra doc lines for Box2 (may be empty).
    pub doc: String,
    /// Signature params for wrapped display (functions only).
    pub sig_params: Vec<String>,
    /// Return type for wrapped display (user functions/methods only).
    pub return_type: String,
    /// True when accepting should leave `()` + open signature help.
    pub opens_paren: bool,
    /// User code (sorts before builtins inside a kind).
    pub user: bool,
}

/// Live completion session state (kept on the Editor).
#[derive(Debug, Default)]
pub struct CompletionState {
    pub items: Vec<Item>,
    pub selected: usize,
    pub prefix: String,
}

/// Live signature-help state.
#[derive(Debug, Default, PartialEq)]
pub struct SigHelpState {
    pub label: String,
    pub ret: String,
    pub params: Vec<String>,
    pub active: usize,
    pub desc: String,
}

/// One wrapped signature row: text runs with per-segment highlight flags.
#[derive(Debug, PartialEq)]
pub struct SigRow {
    /// (text, highlighted) segments; concatenation is the row content.
    pub segments: Vec<(String, bool)>,
}

/// Wrap `ret label(params)` to `inner_w` columns over at most `max_rows` rows.
/// `ret` is the return type (may be empty for builtins); when present the
/// header reads `void fun(`, otherwise `fun(`. Overflow continues on lines
/// starting with `,` (`,int g` style); overlong single params truncate with
/// `…`. `active` (when `Some` and in range) is the highlighted parameter
/// index.
pub fn wrap_sig(
    label: &str,
    ret: &str,
    params: &[String],
    active: Option<usize>,
    inner_w: usize,
    max_rows: usize,
) -> Vec<SigRow> {
    let inner_w = inner_w.max(8);
    let max_rows = max_rows.max(1);
    // Clamp over-long indices to the last param (variadic tail belongs
    // to it); empty lists never highlight.
    let active = match active {
        Some(a) if !params.is_empty() => Some(a.min(params.len() - 1)),
        _ => None,
    };
    let width = |s: &str| s.chars().count();
    let mut rows: Vec<SigRow> = vec![SigRow { segments: vec![] }];
    let mut cur_w = 0usize;
    // Elide the tail of the current row and finish.
    let mut done = false;
    // Place one chunk. `brk` chunks may start a fresh row (`,int g`
    // style); others elide when out of room. Row-leading text is
    // trimmed so continuations read `,int g`, not `, int g`.
    //
    // Implemented inline (not a closure) to keep the borrow checker
    // happy while mutating `rows`/`cur_w` across calls.
    macro_rules! place {
        ($text:expr, $hi:expr, $brk:expr) => {{
            let text: String = $text;
            let row_empty = rows.last().is_none_or(|r: &SigRow| r.segments.is_empty());
            if !row_empty && cur_w + width(&text) > inner_w {
                if !$brk || rows.len() >= max_rows {
                    if let Some(last) = rows.last_mut() {
                        last.segments.push(("…".to_string(), false));
                    }
                    done = true;
                } else {
                    rows.push(SigRow { segments: vec![] });
                    cur_w = 0;
                }
            }
            if !done {
                let text = if cur_w == 0 {
                    text.trim_start().to_string()
                } else {
                    text
                };
                cur_w += width(&text);
                rows.last_mut().unwrap().segments.push((text, $hi));
            }
        }};
    }
    let header = if ret.trim().is_empty() {
        format!("{}(", label)
    } else {
        format!("{} {}(", ret.trim(), label)
    };
    place!(header, false, false);
    for (i, p) in params.iter().enumerate() {
        if done {
            break;
        }
        let p = if p.chars().count() > 28 {
            let cut = p.floor_char_boundary(27);
            format!("{}…", &p[..cut])
        } else {
            p.clone()
        };
        // Later params travel with their comma.
        // when it fits, else the comma leads the next row (`,int g`).
        // Params carry no trailing space; the separator owns spacing.
        let hi = Some(i) == active;
        if i == 0 {
            // First param attaches directly (or elides when hopeless).
            place!(p.clone(), hi, false);
            if done {
                break;
            }
            continue;
        }
        let row_empty = rows.last().is_none_or(|r: &SigRow| r.segments.is_empty());
        if !row_empty && cur_w + 2 + width(&p) > inner_w {
            if rows.len() >= max_rows {
                place!(", ".to_string(), false, true);
                break;
            }
            rows.push(SigRow { segments: vec![] });
            cur_w = 0;
            place!(",".to_string(), false, true);
            if done {
                break;
            }
            place!(p.clone(), hi, false);
            if done {
                break;
            }
            continue;
        }
        place!(", ".to_string(), false, true);
        if done {
            break;
        }
        place!(p.clone(), hi, false);
        if done {
            break;
        }
    }
    if !done {
        // Closing paren rides along, or takes its own row when rows remain.
        place!(")".to_string(), false, true);
    }
    let _ = cur_w;
    rows
}

fn starts_with_ci(name: &str, prefix: &str) -> bool {
    name.len() >= prefix.len() && name[..prefix.len()].eq_ignore_ascii_case(prefix)
}

/// Split a parameter list on top-level commas (nesting-aware).
fn split_params(params: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut cur = String::new();
    for ch in params.chars() {
        match ch {
            '(' | '[' | '{' => {
                depth += 1;
                cur.push(ch);
            }
            ')' | ']' | '}' => {
                depth = depth.saturating_sub(1);
                cur.push(ch);
            }
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out.into_iter().filter(|p| !p.is_empty()).collect()
}

/// Parse a `hover_info` one-liner `Name(p1, p2) → desc`.
fn parse_hover_sig(line: &str) -> Option<(Vec<String>, String)> {
    let (sig, desc) = line
        .split_once(" \u{2192} ")
        .or_else(|| line.split_once(" -> "))?;
    let open = sig.find('(')?;
    let close = sig.rfind(')')?;
    if close < open {
        return None;
    }
    Some((split_params(&sig[open + 1..close]), desc.trim().to_string()))
}

/// Build the full candidate list for `prefix` (case-insensitive),
/// pure priority sort: kind order, then user symbols, then alphabetical.
/// Member functions declared inside class bodies never leak here.
pub fn build_items(lines: &[String], prefix: &str, dialect: Option<Dialect>) -> Vec<Item> {
    if prefix.is_empty() {
        return Vec::new();
    }
    let ranges = extract_class_ranges(lines);
    let mut all: Vec<Item> = Vec::new();
    let mut push = |kind: Kind,
                    name: &str,
                    label: String,
                    detail: String,
                    doc: String,
                    sig: Vec<String>,
                    ret: String,
                    paren: bool,
                    user: bool| {
        if starts_with_ci(name, prefix) {
            all.push(Item {
                label,
                name: name.to_string(),
                kind,
                detail,
                doc,
                sig_params: sig,
                return_type: ret,
                opens_paren: paren,
                user,
            });
        }
    };

    // User functions (outside class bodies) + builtins.
    for f in extract_user_functions(lines) {
        if in_class_body(&ranges, f.line) {
            continue;
        }
        let sig = if f.return_type.is_empty() {
            format!("{}({})", f.name, f.params)
        } else {
            format!("{} {}({})", f.return_type, f.name, f.params)
        };
        let doc = user_doc(lines, f.line);
        push(
            Kind::Function,
            &f.name,
            format!("{}()", f.name),
            sig,
            doc,
            split_params(&f.params),
            f.return_type.clone(),
            true,
            true,
        );
    }
    for name in builtin_function_names() {
        if !visible_in_dialect(name, dialect) {
            continue;
        }
        let (detail, doc, sigp) = match hover_info(name) {
            Some(line) => (
                line.to_string(),
                hover_info_detailed(name).unwrap_or_default(),
                parse_hover_sig(line).map(|(p, _)| p).unwrap_or_default(),
            ),
            None => (format!("{}()", name), String::new(), Vec::new()),
        };
        push(
            Kind::Function,
            name,
            format!("{}()", name),
            detail,
            doc,
            sigp,
            String::new(),
            true,
            false,
        );
    }
    // Variables, arrays, types (outside class bodies).
    for v in extract_variables(lines) {
        if in_class_body(&ranges, v.line) {
            continue;
        }
        if v.type_name.ends_with("[]") {
            push(
                Kind::Array,
                &v.name,
                format!("{}[]", v.name),
                format!("{} {}", v.type_name, v.name),
                String::new(),
                Vec::new(),
                String::new(),
                false,
                true,
            );
        } else {
            push(
                Kind::Var,
                &v.name,
                v.name.clone(),
                format!("{} {}", v.type_name, v.name),
                String::new(),
                Vec::new(),
                String::new(),
                false,
                true,
            );
        }
    }
    // Variables of user class/struct/enum type (`Object object1();`):
    // these never appear in `extract_variables` (builtin types only),
    // so offer them here or `o` completes to nothing.
    {
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
        for v in extract_custom_typed_variables(lines, &user_types) {
            push(
                Kind::Var,
                &v.name,
                v.name.clone(),
                format!("{} {}", v.type_name, v.name),
                String::new(),
                Vec::new(),
                String::new(),
                false,
                true,
            );
        }
    }
    for t in type_names() {
        push(
            Kind::Var,
            t,
            t.to_string(),
            format!("type {}", t),
            String::new(),
            Vec::new(),
            String::new(),
            false,
            false,
        );
    }
    // Keywords + named constants share [K] (per spec).
    for k in keyword_names() {
        push(
            Kind::Keyword,
            k,
            k.to_string(),
            format!("keyword {}", k),
            String::new(),
            Vec::new(),
            String::new(),
            false,
            false,
        );
    }
    for c in constant_names() {
        push(
            Kind::Keyword,
            c,
            c.to_string(),
            format!("{} · constant", c),
            String::new(),
            Vec::new(),
            String::new(),
            false,
            false,
        );
    }
    // Classes, structs, enums (+members), defines.
    for (name, line) in extract_classes(lines) {
        let doc = user_doc(lines, line);
        push(
            Kind::Object,
            &name,
            name.clone(),
            format!("class {}", name),
            doc,
            Vec::new(),
            String::new(),
            false,
            true,
        );
    }
    for (name, line) in extract_structs(lines) {
        let doc = user_doc(lines, line);
        push(
            Kind::Struct,
            &name,
            name.clone(),
            format!("struct {}", name),
            doc,
            Vec::new(),
            String::new(),
            false,
            true,
        );
    }
    for e in extract_enums(lines) {
        if !e.name.is_empty() {
            push(
                Kind::Enum,
                &e.name,
                e.name.clone(),
                format!("enum {}", e.name),
                String::new(),
                Vec::new(),
                String::new(),
                false,
                true,
            );
        }
        for m in &e.members {
            push(
                Kind::Enum,
                m,
                m.clone(),
                if e.name.is_empty() {
                    "enum member".to_string()
                } else {
                    format!("{} member ({})", m, e.name)
                },
                String::new(),
                Vec::new(),
                String::new(),
                false,
                true,
            );
        }
    }
    for d in extract_defines(lines) {
        let detail = if d.value.is_empty() {
            format!("#define {}", d.name)
        } else {
            format!("#define {} {}", d.name, d.value)
        };
        push(
            Kind::Macro,
            &d.name,
            d.name.clone(),
            detail,
            String::new(),
            Vec::new(),
            String::new(),
            false,
            true,
        );
    }

    // Pure priority sort: kind order, user symbols first, alphabetical.
    // No cap, no round-robin: the UI windows 12 rows into this list.
    all.sort_by(|a, b| {
        a.kind
            .priority()
            .cmp(&b.kind.priority())
            .then_with(|| b.user.cmp(&a.user))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    all
}

/// Build the member list for `obj.` where `obj` has user type `class_name`.
/// Methods first (constructors included, private/protected excluded),
/// then member variables. Empty prefix shows everything.
pub fn build_member_items(lines: &[String], class_name: &str, prefix: &str) -> Vec<Item> {
    let mut out = Vec::new();
    for info in extract_classes_full(lines) {
        if info.name != class_name {
            continue;
        }
        for m in info
            .methods
            .iter()
            .filter(|m| !m.is_private || m.name == class_name)
        {
            if !prefix.is_empty() && !starts_with_ci(&m.name, prefix) {
                continue;
            }
            out.push(Item {
                label: format!("{}()", m.name),
                name: m.name.clone(),
                kind: Kind::Function,
                detail: m.detail.clone(),
                doc: user_doc(lines, m.line),
                sig_params: split_params(&m.params),
                return_type: m.return_type.clone(),
                opens_paren: true,
                user: true,
            });
        }
        for v in info.vars.iter().filter(|v| !v.is_private) {
            if !prefix.is_empty() && !starts_with_ci(&v.name, prefix) {
                continue;
            }
            out.push(Item {
                label: v.name.clone(),
                name: v.name.clone(),
                kind: Kind::Var,
                detail: v.detail.clone(),
                doc: String::new(),
                sig_params: Vec::new(),
                return_type: String::new(),
                opens_paren: false,
                user: true,
            });
        }
    }
    out.sort_by(|a, b| {
        a.kind
            .priority()
            .cmp(&b.kind.priority())
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    out
}

/// Plain `//` comment lines directly above `line` (fallback when no
/// `//+` doc box exists). Joined top-down, markers stripped.
fn preceding_comments(lines: &[String], line: usize) -> String {
    if line == 0 || line > lines.len() {
        return String::new();
    }
    let mut out = Vec::new();
    let mut i = line;
    while i > 0 {
        i -= 1;
        let t = lines[i].trim();
        if !t.starts_with("//") {
            break;
        }
        let body = t[2..].trim().trim_matches(|c| c == '+' || c == '|').trim();
        if !body.is_empty() {
            out.push(body.to_string());
        }
    }
    out.reverse();
    out.join("\n")
}

fn user_doc(lines: &[String], line: usize) -> String {
    extract_comment_box(lines, line)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| preceding_comments(lines, line))
}
/// Identifier characters for prefix extraction.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Word immediately before (`y`, `byte`); empty when on whitespace/punct.
pub fn word_before(lines: &[String], y: usize, byte: usize) -> String {
    let line = match lines.get(y) {
        Some(l) => l,
        None => return String::new(),
    };
    let byte = byte.min(line.len());
    let mut start = byte;
    while start > 0 {
        match line[..start].chars().next_back() {
            Some(c) if is_word_char(c) => {
                start -= c.len_utf8();
            }
            _ => break,
        }
    }
    line[start..byte].to_string()
}

/// Resolve signature (return type, params, description) for `name`:
/// user functions, then class/struct methods, then builtins.
pub fn signature_for(
    name: &str,
    lines: &[String],
    dialect: Option<Dialect>,
) -> Option<(String, Vec<String>, String)> {
    let lower = name.to_lowercase();
    for f in extract_user_functions(lines) {
        if f.name.eq_ignore_ascii_case(&lower) {
            let desc = user_doc(lines, f.line);
            let first = if desc.lines().count() > 3 {
                desc.lines().take(3).collect::<Vec<_>>().join(" ")
            } else {
                desc
            };
            return Some((f.return_type.clone(), split_params(&f.params), first));
        }
    }
    for info in extract_classes_full(lines) {
        for m in info
            .methods
            .iter()
            .filter(|m| !m.is_private || m.name == info.name)
        {
            if m.name.eq_ignore_ascii_case(&lower) {
                let doc = user_doc(lines, m.line);
                return Some((m.return_type.clone(), split_params(&m.params), doc));
            }
        }
    }
    for b in builtin_function_names() {
        if b.eq_ignore_ascii_case(&lower) && visible_in_dialect(b, dialect) {
            if let Some(line) = hover_info(b) {
                if let Some((params, desc)) = parse_hover_sig(line) {
                    let doc = hover_info_detailed(b).unwrap_or_default();
                    let desc = if doc.is_empty() { desc } else { doc };
                    return Some((String::new(), params, desc));
                }
            }
            return Some((String::new(), Vec::new(), String::new()));
        }
    }
    None
}

/// Control-flow names that never take signature help.
const NO_SIG_KEYWORDS: &[&str] = &[
    "if", "for", "while", "switch", "catch", "return", "sizeof", "else", "do",
];

/// A located enclosing call: callee `name`, position of `(`, active index.
#[derive(Debug, PartialEq)]
pub struct CallSite {
    pub name: String,
    /// Byte offset of `(` within `lines[open_line]`.
    pub open_line: usize,
    pub open_byte: usize,
    pub active: usize,
}

/// Strip `"…"` strings and `//` comments (replaced by spaces to keep
/// byte offsets stable).
fn mask_line(line: &str) -> String {
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

/// Find the enclosing call whose `(` precedes (`y`, `byte`).
/// Scans back at most 15 lines; returns `None` outside any parens,
/// past a depth-0 `)`, or for control-flow callees.
pub fn enclosing_call(lines: &[String], y: usize, byte: usize) -> Option<CallSite> {
    if lines.is_empty() {
        return None;
    }
    let y = y.min(lines.len() - 1);
    let start_line = y.saturating_sub(15);
    // Innermost unmatched `(` before the cursor, tracked as a stack so
    // nested calls resolve to the inner callee.
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut dead = false;
    for (li, line) in lines.iter().enumerate().take(y + 1).skip(start_line) {
        let masked = mask_line(line);
        let end = if li == y {
            byte.min(masked.len())
        } else {
            masked.len()
        };
        // Byte-safe walk over the masked prefix.
        let mut bi = 0usize;
        for ch in masked[..end].chars() {
            let w = ch.len_utf8();
            if ch == '(' {
                stack.push((li, bi));
            } else if ch == ')' && stack.pop().is_none() {
                // Unbalanced close on this path: outside any call.
                dead = true;
                break;
            }
            bi += w;
        }
        if dead {
            break;
        }
    }
    if dead {
        return None;
    }
    let (oli, ob) = *stack.last()?;
    // Callee = identifier immediately before `(`.
    let line = &lines[oli];
    let mut start = ob;
    while start > 0 {
        match line[..start].chars().next_back() {
            Some(c) if is_word_char(c) => {
                start -= c.len_utf8();
            }
            _ => break,
        }
    }
    let name = line[start..ob].to_string();
    if name.is_empty() || NO_SIG_KEYWORDS.contains(&name.as_str()) {
        return None;
    }
    // Active index = top-level commas between `(` and the cursor.
    let mut commas = 0usize;
    let mut d = 0usize;
    for (li, text) in lines.iter().enumerate().skip(oli).take(y - oli + 1) {
        let from = if li == oli { ob + 1 } else { 0 };
        let to = if li == y {
            byte.min(text.len())
        } else {
            text.len()
        };
        if from >= to {
            continue;
        }
        // Depth relative to the open paren (strings masked).
        for ch in mask_line(&text[from..to]).chars() {
            match ch {
                '(' | '[' | '{' => d += 1,
                ')' | ']' | '}' => {
                    d = d.saturating_sub(1);
                }
                ',' if d == 0 => commas += 1,
                _ => {}
            }
        }
    }
    Some(CallSite {
        name,
        open_line: oli,
        open_byte: ob,
        active: commas,
    })
}

/// Dialect from a file extension (`mq4`/`mq5`, else union).
pub fn dialect_for_path(path: &std::path::Path) -> Option<Dialect> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    dialect_for_ext(ext)
}

/// Member context at (`y`, `byte`): the cursor sits after `obj[.prefix]`
/// where `obj` has a user class/struct type. Returns (class, prefix).
/// Byte-safe; `None` for plain positions.
pub fn member_context(lines: &[String], y: usize, byte: usize) -> Option<(String, String)> {
    let line = lines.get(y)?;
    let byte = byte.min(line.len());
    let word = word_before(lines, y, byte);
    // The dot sits right before the word (modulo spaces).
    let before = line[..byte.saturating_sub(word.len())].trim_end();
    if !before.ends_with('.') {
        return None;
    }
    let base = before[..before.len() - 1].trim_end();
    let mut start = base.len();
    while start > 0 {
        match base[..start].chars().next_back() {
            Some(c) if is_word_char(c) => {
                start -= c.len_utf8();
            }
            _ => break,
        }
    }
    let name = base[start..].to_string();
    if name.is_empty() {
        return None;
    }
    // Known user types: classes, structs, enums.
    let mut types: Vec<String> = Vec::new();
    for (n, _) in extract_classes(lines) {
        types.push(n);
    }
    for (n, _) in extract_structs(lines) {
        types.push(n);
    }
    for e in extract_enums(lines) {
        if !e.name.is_empty() {
            types.push(e.name);
        }
    }
    for v in extract_custom_typed_variables(lines, &types) {
        if v.name == name {
            return Some((v.type_name, word));
        }
    }
    None
}

impl Editor {
    /// Completion/sig-help live only in single-cursor MQL insert mode.
    fn mql_active(&self) -> bool {
        self.mode == Mode::Insert
            && !self.has_multiple_cursors()
            && self.syntax.language == Some(SyntaxLanguage::Mql)
    }

    pub(crate) fn dismiss_popups(&mut self) {
        self.completion = None;
        self.sighelp = None;
    }

    /// Rebuild (or hide) the candidate list from the word at the cursor.
    /// Member context (`obj.` prefix) rebuilds the member list instead.
    /// Preserves the selection by name when the prefix only grew.
    /// Repaints whenever visibility flips so no ghost box remains.
    pub(crate) fn maybe_trigger_completion(&mut self) {
        if !self.mql_active() {
            return;
        }
        let was_visible = self
            .completion
            .as_ref()
            .is_some_and(|s| !s.items.is_empty());
        if let Some((class, _)) = member_context(&self.lines, self.cursor.y, self.cursor.byte) {
            let word = word_before(&self.lines, self.cursor.y, self.cursor.byte);
            let items = build_member_items(&self.lines, &class, &word);
            if items.is_empty() {
                self.completion = None;
            } else {
                let keep = match &self.completion {
                    Some(st) if word.starts_with(&st.prefix) => {
                        st.items.get(st.selected).map(|i| i.name.clone())
                    }
                    _ => None,
                };
                let selected = keep
                    .and_then(|name| items.iter().position(|i| i.name == name))
                    .unwrap_or(0);
                self.completion = Some(CompletionState {
                    items,
                    selected,
                    prefix: word,
                });
            }
            if self
                .completion
                .as_ref()
                .is_some_and(|s| !s.items.is_empty())
                || was_visible
            {
                self.mark_all_dirty();
            }
            return;
        }
        let word = word_before(&self.lines, self.cursor.y, self.cursor.byte);
        if word.is_empty() {
            self.completion = None;
        } else {
            let items = build_items(&self.lines, &word, dialect_for_path(&self.path));
            if items.is_empty() {
                self.completion = None;
            } else {
                let keep = match &self.completion {
                    Some(st) if word.starts_with(&st.prefix) => {
                        st.items.get(st.selected).map(|i| i.name.clone())
                    }
                    _ => None,
                };
                let selected = keep
                    .and_then(|name| items.iter().position(|i| i.name == name))
                    .unwrap_or(0);
                self.completion = Some(CompletionState {
                    items,
                    selected,
                    prefix: word,
                });
            }
        }
        let now_visible = self
            .completion
            .as_ref()
            .is_some_and(|s| !s.items.is_empty());
        if now_visible || was_visible {
            self.mark_all_dirty();
        }
    }

    /// Recompute signature help from the enclosing call at the cursor.
    /// Repaints on any state change so dismissed boxes never linger.
    pub(crate) fn refresh_sighelp(&mut self) {
        if !self.mql_active() {
            if self.sighelp.is_some() {
                self.sighelp = None;
                self.mark_all_dirty();
            }
            return;
        }
        let next = match enclosing_call(&self.lines, self.cursor.y, self.cursor.byte) {
            Some(call) => {
                match signature_for(&call.name, &self.lines, dialect_for_path(&self.path)) {
                    Some((ret, params, desc)) => Some(SigHelpState {
                        label: call.name,
                        ret,
                        params,
                        active: call.active,
                        desc,
                    }),
                    None => None,
                }
            }
            None => None,
        };
        if next != self.sighelp {
            self.sighelp = next;
            self.mark_all_dirty();
        }
    }

    /// Accept the selected candidate: replace the current word. Function
    /// labels end with `()` — the cursor lands inside and sig-help opens.
    pub(crate) fn accept_completion(&mut self) {
        let (label, paren) = match &self.completion {
            Some(st) if !st.items.is_empty() => {
                let it = &st.items[st.selected % st.items.len()];
                (it.label.clone(), it.opens_paren)
            }
            _ => return,
        };
        if self.has_selection() {
            self.delete_selection();
        }
        let word = word_before(&self.lines, self.cursor.y, self.cursor.byte);
        if word.is_empty() {
            self.completion = None;
            return;
        }
        let start = self.cursor.byte - word.len();
        self.snapshot();
        self.lines[self.cursor.y].replace_range(start..self.cursor.byte, &label);
        self.cursor.byte = start + label.len();
        self.completion = None;
        self.mark_all_dirty();
        if paren {
            // Step inside the parens; the post-key funnel refresh opens
            // signature help from the new cursor position.
            self.cursor.byte = self.cursor.byte.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sl(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn fixture() -> Vec<String> {
        sl(&[
            "//it's so fun here",
            "void fun(int a , int b){ implemented;}",
            "//some_object_comment",
            "class func{ private int x; };",
            "int foo;",
            "double arr[];",
            "enum Color { RED, GREEN };",
            "#define MAXLOT 10",
            "int bar;",
        ])
    }

    #[test]
    fn twelve_row_priority_fill() {
        let items = build_items(&fixture(), "f", None);
        assert!(!items.is_empty());
        // Pure priority sort: kind order never decreases down the list.
        let mut last_prio = 0u8;
        for i in &items {
            assert!(i.kind.priority() >= last_prio);
            last_prio = i.kind.priority();
        }
        // User function leads the Function block.
        assert_eq!(items[0].name, "fun");
        // Labels carry kind-correct suffixes.
        for i in &items {
            match i.kind {
                Kind::Function => assert!(i.label.ends_with("()"), "{}", i.label),
                Kind::Array => assert!(i.label.ends_with("[]"), "{}", i.label),
                _ => {}
            }
        }
    }

    #[test]
    fn pure_sort_groups_kinds() {
        // Same-initial names across kinds: F block, then V, A, O, S, E, M.
        let lines = sl(&[
            "void afun(){ }",
            "int avar;",
            "int aarr[];",
            "class acls { };",
            "struct astr { };",
            "enum aenu { AM };",
            "#define AMACRO 1",
        ]);
        let items = build_items(&lines, "a", None);
        let kinds: Vec<Kind> = items.iter().map(|i| i.kind).collect();
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names[0], "afun", "user function leads F: {:?}", names);
        let first_v = kinds.iter().position(|k| *k == Kind::Var).unwrap();
        let first_a = kinds.iter().position(|k| *k == Kind::Array).unwrap();
        let first_o = kinds.iter().position(|k| *k == Kind::Object).unwrap();
        let first_s = kinds.iter().position(|k| *k == Kind::Struct).unwrap();
        let first_e = kinds.iter().position(|k| *k == Kind::Enum).unwrap();
        let first_m = kinds.iter().position(|k| *k == Kind::Macro).unwrap();
        assert!(first_v > 0 && first_a > first_v && first_o > first_a);
        assert!(first_s > first_o && first_e > first_s && first_m > first_e);
        assert!(names.contains(&"aarr"));
        assert!(names.contains(&"AMACRO"));
    }

    #[test]
    fn prefix_case_insensitive() {
        let items = build_items(&fixture(), "F", None);
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"fun"));
        assert!(names.contains(&"false"));
        assert!(names.contains(&"func"));
        assert!(names.contains(&"foo"));
    }

    #[test]
    fn user_detail_shapes() {
        let items = build_items(&fixture(), "fu", None);
        let fun = items.iter().find(|i| i.name == "fun").unwrap();
        assert_eq!(fun.kind, Kind::Function);
        assert_eq!(fun.label, "fun()");
        assert!(
            fun.detail.contains("void fun(int a , int b)"),
            "{}",
            fun.detail
        );
        assert!(fun.doc.contains("it's so fun here"), "{}", fun.doc);
        assert!(fun.opens_paren);
        let cls = build_items(&fixture(), "func", None)
            .into_iter()
            .find(|i| i.kind == Kind::Object)
            .unwrap();
        assert!(cls.detail.contains("class func"), "{}", cls.detail);
        assert!(cls.doc.contains("some_object_comment"), "{}", cls.doc);
        let foo = build_items(&fixture(), "foo", None)
            .into_iter()
            .find(|i| i.name == "foo")
            .unwrap();
        assert_eq!(foo.kind, Kind::Var);
        assert!(foo.detail.contains("int foo"), "{}", foo.detail);
    }

    #[test]
    fn builtin_signature_parse() {
        let (ret, params, desc) = signature_for("OrderSend", &[], None).unwrap();
        assert!(ret.is_empty());
        assert!(params.iter().any(|p| p.contains("symbol")), "{:?}", params);
        assert!(params.iter().any(|p| p.contains("volume")), "{:?}", params);
        assert!(!desc.is_empty());
        assert!(signature_for("NoSuchFunction", &[], None).is_none());
    }

    #[test]
    fn user_signature_carries_return_type() {
        let lines = sl(&["void fun(int a , int b){ implemented;}"]);
        let (ret, params, _) = signature_for("fun", &lines, None).unwrap();
        assert_eq!(ret, "void");
        assert_eq!(params, vec!["int a", "int b"]);
        // Rendered header keeps the return type: `void fun(`.
        let rows = wrap_sig("fun", &ret, &params, Some(0), 44, 6);
        let flat: Vec<String> = rows
            .iter()
            .map(|r| r.segments.iter().map(|(s, _)| s.clone()).collect())
            .collect();
        assert!(flat[0].starts_with("void fun(int a"), "{:?}", flat);
    }

    #[test]
    fn enclosing_call_basics() {
        let lines = sl(&["OrderSend(symbol, cmd, volume);"]);
        let call = enclosing_call(&lines, 0, 18).unwrap();
        assert_eq!(call.name, "OrderSend");
        assert_eq!(call.active, 1);
        // Past the closing paren: outside.
        assert!(enclosing_call(&lines, 0, 30).is_none());
        // Before the paren: outside.
        assert!(enclosing_call(&lines, 0, 5).is_none());
        // Control flow excluded.
        let lines = sl(&["if(a > 0) {"]);
        assert!(enclosing_call(&lines, 0, 6).is_none());
    }

    #[test]
    fn enclosing_call_nesting_strings_comments() {
        let lines = sl(&["foo(bar(1, 2), x); // f( not"]);
        // Byte 12 sits inside bar(...): innermost wins.
        let call = enclosing_call(&lines, 0, 12).unwrap();
        assert_eq!(call.name, "bar");
        assert_eq!(call.active, 1);
        // Byte 15 is past bar's close: back to foo.
        let call = enclosing_call(&lines, 0, 15).unwrap();
        assert_eq!(call.name, "foo");
        assert_eq!(call.active, 1);
        let call = enclosing_call(&lines, 0, 8).unwrap();
        assert_eq!(call.name, "bar");
        // String parens don't count.
        let lines = sl(&["Print(\"(not, a, call\");"]);
        let call = enclosing_call(&lines, 0, 14).unwrap();
        assert_eq!(call.name, "Print");
        assert_eq!(call.active, 0);
    }

    #[test]
    fn word_before_edges() {
        let lines = sl(&["  OrderSen", ""]);
        assert_eq!(word_before(&lines, 0, 10), "OrderSen");
        assert_eq!(word_before(&lines, 1, 0), "");
        assert_eq!(word_before(&lines, 0, 2), "");
    }

    #[test]
    fn empty_prefix_shows_nothing() {
        assert!(build_items(&fixture(), "", None).is_empty());
        assert!(build_items(&fixture(), "zzz_no_match", None).is_empty());
    }

    fn flat_rows(rows: &[SigRow]) -> Vec<String> {
        rows.iter()
            .map(|r| {
                r.segments
                    .iter()
                    .map(|(s, hi)| if *hi { format!("[{}]", s) } else { s.clone() })
                    .collect::<String>()
            })
            .collect()
    }

    #[test]
    fn wrap_sig_splits_and_highlights() {
        // 26 params console: active #6 lands wherever it fits.
        let params: Vec<String> = (b'a'..=b'z')
            .map(|c| format!("int {}", c as char))
            .collect();
        let rows = wrap_sig("fun", "", &params, Some(6), 44, 6);
        assert!(rows.len() > 1 && rows.len() <= 6);
        let flat = flat_rows(&rows);
        assert!(flat[0].starts_with("fun(int a"), "{}", flat[0]);
        // Active param g highlighted exactly once across all rows.
        let joined = flat.join("\n");
        assert_eq!(joined.matches("[int g]").count(), 1, "{}", joined);
        // Continuation rows lead with a comma.
        assert!(flat[1].starts_with(','), "{:?}", flat);
        // Everything still present (params + close paren).
        assert!(joined.contains("int z"), "{}", joined);
        assert!(joined.contains(')'), "{}", joined);
    }

    #[test]
    fn wrap_sig_empty_and_truncation() {
        let rows = wrap_sig("f", "", &[], None, 44, 6);
        assert_eq!(flat_rows(&rows), vec!["f()".to_string()]);
        // Tiny width + row cap: tail elided, rows bounded.
        let params = vec!["int a".to_string(); 30];
        let rows = wrap_sig("fun", "", &params, Some(0), 20, 2);
        assert_eq!(rows.len(), 2);
        let joined = flat_rows(&rows).join("\n");
        assert!(joined.contains('…'), "{}", joined);
    }

    #[test]
    fn member_items_and_method_signatures() {
        let lines = sl(&[
            "class O {",
            "public:",
            "  O(int a, double b);",
            "  void run(int x);",
            "  int hidden;",
            "  int secret;",
            "private:",
            "  int priv;",
            "};",
            "O o;",
        ]);
        let items = build_member_items(&lines, "O", "");
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        // Constructor + public method + public vars; private excluded.
        assert!(names.contains(&"O"));
        assert!(names.contains(&"run"));
        assert!(names.contains(&"hidden"));
        assert!(!names.contains(&"priv"));
        let run = items.iter().find(|i| i.name == "run").unwrap();
        assert_eq!(run.kind, Kind::Function);
        assert_eq!(run.label, "run()");
        assert!(run.detail.contains("O::run(int x)"), "{}", run.detail);
        // Prefix filters members.
        let items = build_member_items(&lines, "O", "ru");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "run");
        // Unknown class: nothing.
        assert!(build_member_items(&lines, "Nope", "").is_empty());
        // Method signatures resolve for sig-help.
        let (ret, params, _) = signature_for("run", &lines, None).unwrap();
        assert_eq!(ret, "void");
        assert_eq!(params, vec!["int x"]);
    }

    #[test]
    fn class_members_excluded_from_globals() {
        let lines = sl(&["class O {", "  void run(int x);", "};", "int foo;"]);
        let items = build_items(&lines, "ru", None);
        assert!(
            !items.iter().any(|i| i.name == "run"),
            "in-class method must not leak globally"
        );
        let items = build_items(&lines, "fo", None);
        assert!(items.iter().any(|i| i.name == "foo"));
    }

    #[test]
    fn custom_typed_variables_complete_globally() {
        // `Object object1();` must be offered for prefix `o`.
        let lines = sl(&[
            "class Object {",
            "public:",
            "  Object();",
            "};",
            "Object object1();",
        ]);
        let items = build_items(&lines, "o", None);
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"object1"), "{:?}", names);
        let obj = items.iter().find(|i| i.name == "object1").unwrap();
        assert_eq!(obj.kind, Kind::Var);
        assert!(obj.detail.contains("Object object1"), "{}", obj.detail);
    }

    #[test]
    fn member_completion_excludes_outside_functions() {
        // `object1.` offers members only — never `other_outside_class_fun`,
        // even though it is called inside a member body.
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
            "Object object1();",
            "object1.",
        ]);
        let ctx = member_context(&lines, 10, lines[10].len());
        assert_eq!(ctx, Some(("Object".to_string(), String::new())));
        let items = build_member_items(&lines, "Object", "");
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"sub_fun"), "{:?}", names);
        assert!(
            !names.contains(&"other_outside_class_fun"),
            "outside fun leaked: {:?}",
            names
        );
        for i in items.iter().filter(|i| i.kind == Kind::Function) {
            assert!(
                ["Object", "sub_fun", "fun"].contains(&i.name.as_str()),
                "unexpected member fn: {}",
                i.name
            );
        }
    }
}
