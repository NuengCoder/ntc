//! `nsheet` — terminal sheet viewer for CSV/TSV/JSON/JSONL.
//!
//! ```text
//! ntc [project]> nsheet open users.csv     # fullscreen TUI
//! ntc [project]> nsheet view users.csv -l 20 --sort age --desc
//! ```
//!
//! Parsing lives in [`parse`]; this module holds the view model shared by
//! the non-interactive `view` printer and the fullscreen TUI: column
//! resolution, filtering, numeric-aware sorting and JSON/Markdown export.

pub mod parse;

pub use parse::{load_file, Sheet, SHEET_EXTENSIONS};

/// Comparison operator for one filter condition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CmpOp {
    Contains,
    Eq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
}

/// One `column<op>value` filter condition.
#[derive(Debug, Clone)]
pub struct Condition {
    pub col: usize,
    pub op: CmpOp,
    pub value: String,
}

/// How multiple conditions combine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Logic {
    And,
    Or,
}

/// Resolve a column spec: header name (case-insensitive) or 1-based index.
pub fn resolve_col(spec: &str, headers: &[String]) -> Option<usize> {
    let t = spec.trim();
    if let Ok(n) = t.parse::<usize>() {
        if n >= 1 && n <= headers.len() {
            return Some(n - 1);
        }
        return None;
    }
    headers.iter().position(|h| h.eq_ignore_ascii_case(t))
}

/// Display name for a column index (used in footers and the TUI title).
pub fn col_name(headers: &[String], idx: usize) -> String {
    headers.get(idx).cloned().unwrap_or_else(|| format!("col{}", idx + 1))
}

/// Split a filter expression into conditions plus combining logic.
/// Operators (checked longest-first): `!=`, `>=`, `<=`, `~`, `=`, `>`, `<`.
/// Multiple conditions join with `&&` (default) or `||` — mixing is rejected.
pub fn parse_filter(expr: &str, headers: &[String]) -> Result<(Vec<Condition>, Logic), String> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Err("empty filter".to_string());
    }
    let (logic, parts) = if expr.contains("&&") && expr.contains("||") {
        return Err("cannot mix && and || in one filter".to_string());
    } else if expr.contains("&&") {
        (Logic::And, expr.split("&&").collect::<Vec<_>>())
    } else if expr.contains("||") {
        (Logic::Or, expr.split("||").collect::<Vec<_>>())
    } else {
        (Logic::And, vec![expr])
    };
    let mut conds = Vec::new();
    for part in parts {
        let part = part.trim();
        let mut found: Option<(usize, CmpOp)> = None;
        // Longest operators first so `!=` wins over `=`.
        for (op_str, op) in [
            ("!=", CmpOp::NotEq),
            (">=", CmpOp::Gte),
            ("<=", CmpOp::Lte),
            ("~", CmpOp::Contains),
            ("=", CmpOp::Eq),
            (">", CmpOp::Gt),
            ("<", CmpOp::Lt),
        ] {
            if let Some(pos) = part.find(op_str) {
                found = Some((pos, op));
                // Keep the found operator string length for splitting.
                let left = part[..pos].trim();
                let right = part[pos + op_str.len()..].trim();
                let col = resolve_col(left, headers)
                    .ok_or_else(|| format!("unknown column '{}'", left))?;
                let value = strip_quotes(right);
                if value.is_empty() && !matches!(found.as_ref().unwrap().1, CmpOp::Eq | CmpOp::NotEq) {
                    return Err(format!("missing value in '{}'", part));
                }
                conds.push(Condition { col, op, value });
                break;
            }
        }
        if found.is_none() {
            return Err(format!(
                "bad condition '{}' (use col~text, col=text, col!=text, col>n, col<n)",
                part
            ));
        }
    }
    Ok((conds, logic))
}

fn strip_quotes(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

fn cmp_values(cell: &str, op: &CmpOp, value: &str) -> bool {
    match op {
        CmpOp::Contains => cell.to_lowercase().contains(&value.to_lowercase()),
        CmpOp::Eq => cell == value,
        CmpOp::NotEq => cell != value,
        CmpOp::Gt | CmpOp::Lt | CmpOp::Gte | CmpOp::Lte => {
            match (cell.trim().parse::<f64>(), value.trim().parse::<f64>()) {
                (Ok(a), Ok(b)) => match op {
                    CmpOp::Gt => a > b,
                    CmpOp::Lt => a < b,
                    CmpOp::Gte => a >= b,
                    CmpOp::Lte => a <= b,
                    _ => false,
                },
                _ => match op {
                    CmpOp::Gt => cell > value,
                    CmpOp::Lt => cell < value,
                    CmpOp::Gte => cell >= value,
                    CmpOp::Lte => cell <= value,
                    _ => false,
                },
            }
        }
    }
}

/// True when `row` satisfies the conditions under `logic`.
pub fn row_matches(row: &[String], conds: &[Condition], logic: Logic) -> bool {
    let cell = |c: &Condition| {
        row.get(c.col).map(|s| s.as_str()).unwrap_or("")
    };
    match logic {
        Logic::And => conds.iter().all(|c| cmp_values(cell(c), &c.op, &c.value)),
        Logic::Or => conds.iter().any(|c| cmp_values(cell(c), &c.op, &c.value)),
    }
}

/// Numeric-aware sort: both-numeric compares as f64, otherwise ordinal.
pub fn sort_rows(rows: &mut [Vec<String>], col: usize, desc: bool) {
    rows.sort_by(|a, b| {
        let av = a.get(col).map(|s| s.as_str()).unwrap_or("");
        let bv = b.get(col).map(|s| s.as_str()).unwrap_or("");
        let ord = match (av.trim().parse::<f64>(), bv.trim().parse::<f64>()) {
            (Ok(x), Ok(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
            _ => av.cmp(bv),
        };
        if desc { ord.reverse() } else { ord }
    });
}

/// View rows as `Option` cells for [`crate::ndb::render::format_box_table`].
pub fn to_opt_rows(rows: &[Vec<String>], width: usize) -> Vec<Vec<Option<String>>> {
    rows.iter()
        .map(|r| {
            (0..width)
                .map(|i| {
                    let s = r.get(i).map(|s| s.as_str()).unwrap_or("");
                    if s.is_empty() { None } else { Some(s.to_string()) }
                })
                .collect()
        })
        .collect()
}

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Export rows as a JSON array of objects.
pub fn to_json(headers: &[String], rows: &[Vec<String>]) -> String {
    let mut out = String::from("[\n");
    for (ri, row) in rows.iter().enumerate() {
        out.push_str("  {");
        for (i, h) in headers.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            let v = row.get(i).map(|s| s.as_str()).unwrap_or("");
            out.push_str(&format!("\"{}\": \"{}\"", escape_json(h), escape_json(v)));
        }
        out.push('}');
        if ri + 1 < rows.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push(']');
    out
}

/// Export rows as a GitHub-style Markdown table.
pub fn to_markdown(headers: &[String], rows: &[Vec<String>]) -> String {
    let cell = |s: &str| s.replace('|', "\\|").replace('\n', " ");
    let mut out = String::new();
    out.push_str(&format!(
        "| {} |\n",
        headers.iter().map(|h| cell(h)).collect::<Vec<_>>().join(" | ")
    ));
    out.push_str(&format!("|{}|\n", headers.iter().map(|_| "---").collect::<Vec<_>>().join("|")));
    for row in rows {
        let line: Vec<String> = (0..headers.len())
            .map(|i| cell(row.get(i).map(|s| s.as_str()).unwrap_or("")))
            .collect();
        out.push_str(&format!("| {} |\n", line.join(" | ")));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers() -> Vec<String> {
        vec!["name".into(), "age".into(), "city".into()]
    }

    #[test]
    fn resolve_col_by_name_and_index() {
        let h = headers();
        assert_eq!(resolve_col("name", &h), Some(0));
        assert_eq!(resolve_col("AGE", &h), Some(1));
        assert_eq!(resolve_col("3", &h), Some(2));
        assert_eq!(resolve_col("0", &h), None);
        assert_eq!(resolve_col("zzz", &h), None);
    }

    #[test]
    fn filter_parse_and_match() {
        let h = headers();
        let (conds, logic) = parse_filter("age>30 && city~york", &h).unwrap();
        assert_eq!(logic, Logic::And);
        assert!(row_matches(&["a".into(), "35".into(), "New York".into()], &conds, logic));
        assert!(!row_matches(&["a".into(), "25".into(), "New York".into()], &conds, logic));
        let (or_conds, or_logic) = parse_filter("age<20 || name=bob", &h).unwrap();
        assert_eq!(or_logic, Logic::Or);
        assert!(row_matches(&["bob".into(), "50".into(), "x".into()], &or_conds, or_logic));
        assert!(parse_filter("age>1 && name~a || city=x", &h).is_err());
        assert!(parse_filter("zzz=1", &h).is_err());
    }

    #[test]
    fn sort_numeric_aware() {
        let mut rows = vec![
            vec!["b".into(), "30".into()],
            vec!["a".into(), "9".into()],
            vec!["c".into(), "100".into()],
        ];
        sort_rows(&mut rows, 1, false);
        assert_eq!(rows[0][1], "9");
        assert_eq!(rows[2][1], "100");
        sort_rows(&mut rows, 0, true);
        assert_eq!(rows[0][0], "c");
    }

    #[test]
    fn export_shapes() {
        let h = headers();
        let rows = vec![vec!["a\"b".into(), "1".into(), "x|y".into()]];
        let j = to_json(&h, &rows);
        assert!(j.contains("\\\""));
        let m = to_markdown(&h, &rows);
        assert!(m.contains("\\|"));
    }
}
