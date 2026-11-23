//! Pull the symbols out of a `.ntc` buffer.
//!
//! A script is line-oriented, so this is a light scan rather than a parser: it
//! finds `func` definitions (with the comment block above them), `set`
//! variables, `for` loop variables and `input` targets. That is exactly what
//! completion and hover need, and it keeps working on half-typed files.

/// A `func NAME(params)` definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NtcFunc {
    pub name: String,
    pub params: Vec<String>,
    /// 0-based line of the `func` keyword.
    pub line: usize,
    /// Comment lines directly above the definition, without the `#`.
    pub doc: String,
}

impl NtcFunc {
    /// `func greet(who, other)` — the form shown in the detail column.
    pub fn signature(&self) -> String {
        format!("func {}({})", self.name, self.params.join(", "))
    }
}

/// A variable the script defines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NtcVar {
    pub name: String,
    /// The expression as written (empty for `input` targets).
    pub value: String,
    /// How it came to exist: `set`, `for` or `input`.
    pub origin: &'static str,
    pub line: usize,
}

/// Split a `func` signature into its name and parameter list.
fn parse_signature(rest: &str) -> Option<(String, Vec<String>)> {
    let rest = rest.trim();
    let (name, tail) = match rest.find('(') {
        Some(open) => (rest[..open].trim(), Some(&rest[open + 1..])),
        None => (rest.split_whitespace().next().unwrap_or(""), None),
    };
    if name.is_empty() {
        return None;
    }
    let params = tail
        .and_then(|tail| {
            tail.rsplit_once(')')
                .map(|(before, _)| before)
                .or(Some(tail))
        })
        .map(|inner| {
            inner
                .split(',')
                .map(|param| param.trim().to_string())
                .filter(|param| !param.is_empty())
                .collect()
        })
        .unwrap_or_default();
    Some((name.to_string(), params))
}

/// Comment lines immediately above `line`, stripped of the marker.
fn doc_above(lines: &[String], line: usize) -> String {
    let mut out: Vec<&str> = Vec::new();
    let mut index = line;
    while index > 0 {
        index -= 1;
        let text = lines[index].trim();
        if text.is_empty() {
            break;
        }
        let Some(comment) = text.strip_prefix('#') else {
            break;
        };
        out.push(comment.trim());
    }
    out.reverse();
    out.join("\n")
}

/// Every `func` definition in the buffer, in file order.
pub(crate) fn functions(lines: &[String]) -> Vec<NtcFunc> {
    let mut out = Vec::new();
    for (index, raw) in lines.iter().enumerate() {
        let text = raw.trim();
        let rest = match text.split_once(char::is_whitespace) {
            Some((word, rest))
                if matches!(
                    word.to_ascii_lowercase().as_str(),
                    "func" | "function" | "fn"
                ) =>
            {
                rest
            }
            _ => continue,
        };
        if let Some((name, params)) = parse_signature(rest) {
            out.push(NtcFunc {
                name,
                params,
                line: index,
                doc: doc_above(lines, index),
            });
        }
    }
    out
}

/// Every variable the buffer defines, in file order and de-duplicated by name
/// (the first definition wins, which is where a reader looks).
pub(crate) fn variables(lines: &[String]) -> Vec<NtcVar> {
    let mut out: Vec<NtcVar> = Vec::new();
    let mut push = |var: NtcVar| {
        if !out.iter().any(|existing| existing.name == var.name) {
            out.push(var);
        }
    };

    for (index, raw) in lines.iter().enumerate() {
        let text = raw.trim();
        let Some((word, rest)) = text.split_once(char::is_whitespace) else {
            continue;
        };
        match word.to_ascii_lowercase().as_str() {
            "set" => {
                // `set NAME = value`
                if let Some((name, value)) = rest.split_once('=') {
                    let name = name.trim();
                    if !name.is_empty() {
                        push(NtcVar {
                            name: name.to_string(),
                            value: value.trim().to_string(),
                            origin: "set",
                            line: index,
                        });
                    }
                }
            }
            "for" => {
                // `for VAR in ...`
                let mut parts = rest.splitn(2, char::is_whitespace);
                let name = parts.next().unwrap_or("").trim();
                if !name.is_empty() {
                    push(NtcVar {
                        name: name.to_string(),
                        value: parts.next().unwrap_or("").trim().to_string(),
                        origin: "for",
                        line: index,
                    });
                }
            }
            "input" => {
                // `input NAME [prompt]`
                let name = rest.split_whitespace().next().unwrap_or("");
                if !name.is_empty() {
                    push(NtcVar {
                        name: name.to_string(),
                        value: String::new(),
                        origin: "input",
                        line: index,
                    });
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sl(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn functions_are_found_with_their_params() {
        let lines = sl(&[
            "# greet someone",
            "# second line",
            "func greet(who, other)",
            "  say $who",
            "end",
            "func plain()",
            "end",
            "fn short",
            "end",
        ]);
        let funcs = functions(&lines);
        assert_eq!(funcs.len(), 3);
        assert_eq!(funcs[0].name, "greet");
        assert_eq!(funcs[0].params, vec!["who", "other"]);
        assert_eq!(funcs[0].line, 2);
        assert_eq!(funcs[0].doc, "greet someone\nsecond line");
        assert_eq!(funcs[1].params.len(), 0);
        assert_eq!(funcs[2].name, "short");
        assert_eq!(funcs[0].signature(), "func greet(who, other)");
    }

    #[test]
    fn variables_cover_set_for_and_input() {
        let lines = sl(&[
            "set name = \"world\"",
            "set count = 1 + 2",
            "for f in a b c",
            "input answer \"Continue? \"",
            "set name = shadowed",
        ]);
        let vars = variables(&lines);
        let names: Vec<&str> = vars.iter().map(|v| v.name.as_str()).collect();
        assert_eq!(names, vec!["name", "count", "f", "answer"]);
        assert_eq!(vars[0].value, "\"world\"");
        assert_eq!(vars[0].origin, "set");
        assert_eq!(vars[2].origin, "for");
        assert_eq!(vars[3].origin, "input");
    }

    #[test]
    fn a_half_typed_file_does_not_panic() {
        let lines = sl(&["func", "set", "for", "input", "func broken(", "set x ="]);
        let _ = functions(&lines);
        let _ = variables(&lines);
    }

    #[test]
    fn comments_stop_at_a_blank_line() {
        let lines = sl(&["# far away", "", "# near", "func f()", "end"]);
        assert_eq!(functions(&lines)[0].doc, "near");
    }
}
