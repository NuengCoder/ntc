//! File parsing for `nsheet`: CSV/TSV (delimiter-separated) and JSON/JSONL.
//!
//! Deliberately dependency-free (no `csv` crate): the parser handles
//! RFC-4180-style quoting (`"a,b"`, `""` escapes) plus delimiter and header
//! auto-detection so it works on Windows, Linux, macOS and Termux alike.

use std::path::Path;

/// A loaded sheet: uniform string grid plus the source label for titles.
pub struct Sheet {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub source: String,
}

/// Delimiter candidates considered during auto-detection, in priority order.
const DELIM_CANDIDATES: &[char] = &[',', ';', '\t', '|'];

/// Strip a UTF-8 BOM if present.
fn strip_bom(s: &str) -> &str {
    s.strip_prefix('\u{FEFF}').unwrap_or(s)
}

/// Split one delimiter-separated line, honouring `"quoted"` fields where
/// `""` is an escaped quote. Returns `None` on an unterminated quote.
fn split_line(line: &str, delim: char) -> Option<Vec<String>> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;
    let mut any_quote = false;
    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                    any_quote = true;
                }
            } else {
                cur.push(ch);
            }
        } else if ch == '"' && cur.is_empty() && !any_quote {
            in_quotes = true;
        } else if ch == delim {
            fields.push(std::mem::take(&mut cur));
            any_quote = false;
        } else {
            cur.push(ch);
        }
    }
    if in_quotes {
        return None;
    }
    fields.push(cur);
    Some(fields)
}

/// Score a delimiter: consistent field counts across sampled lines win.
/// Returns (delimiter, consistency score); higher is better.
fn detect_delimiter(lines: &[&str]) -> char {
    let sample: Vec<&str> = lines.iter().copied().take(20).collect();
    let mut best = (',', 0usize, 0usize); // (delim, consistent_lines, total_fields)
    for &d in DELIM_CANDIDATES {
        let mut counts: Vec<usize> = Vec::new();
        let mut ok = true;
        for line in &sample {
            match split_line(line, d) {
                Some(f) => counts.push(f.len()),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok || counts.is_empty() {
            continue;
        }
        let first = counts[0];
        if first < 2 {
            continue; // a delimiter must actually split
        }
        let consistent = counts.iter().filter(|&&c| c == first).count();
        let total: usize = counts.iter().sum();
        if consistent > best.1 || (consistent == best.1 && total > best.2) {
            best = (d, consistent, total);
        }
    }
    best.0
}

fn is_number_cell(s: &str) -> bool {
    let t = s.trim();
    !t.is_empty() && t.parse::<f64>().is_ok()
}

/// Header heuristic: the first row looks like a header when it contains no
/// numeric cells while the second row contains at least one.
fn looks_like_header(rows: &[Vec<String>]) -> bool {
    if rows.len() < 2 {
        return true; // single line under-determined: treat as header
    }
    let first_numeric = rows[0].iter().any(|c| is_number_cell(c));
    let second_numeric = rows[1].iter().any(|c| is_number_cell(c));
    !first_numeric && (second_numeric || rows[0].iter().any(|c| c.trim().is_empty()))
}

/// Parse delimiter-separated text into a [`Sheet`].
pub fn parse_delimited(
    text: &str,
    delim_opt: Option<char>,
    header_opt: Option<bool>,
    source: String,
) -> Result<Sheet, String> {
    let text = strip_bom(text);
    // Collect non-empty lines; tolerate a trailing newline.
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return Err("empty file".to_string());
    }
    let delim = delim_opt.unwrap_or_else(|| detect_delimiter(&lines));
    let mut rows: Vec<Vec<String>> = Vec::with_capacity(lines.len());
    for (idx, line) in lines.iter().enumerate() {
        match split_line(line, delim) {
            Some(mut f) => {
                for c in f.iter_mut() {
                    *c = c.trim().to_string();
                }
                rows.push(f);
            }
            None => return Err(format!("line {}: unterminated quote", idx + 1)),
        }
    }
    // Normalize ragged rows to the widest width.
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    for r in rows.iter_mut() {
        r.resize(width, String::new());
    }
    let use_header = header_opt.unwrap_or_else(|| looks_like_header(&rows));
    let (headers, data) = if use_header {
        let h = rows.remove(0);
        let headers: Vec<String> = h
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                let t = c.trim().to_string();
                if t.is_empty() {
                    format!("col{}", i + 1)
                } else {
                    t
                }
            })
            .collect();
        (headers, rows)
    } else {
        let headers: Vec<String> = (1..=width).map(|i| format!("col{}", i)).collect();
        (headers, rows)
    };
    Ok(Sheet { headers, rows: data, source })
}

fn json_value_to_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        _ => serde_json::to_string(v).unwrap_or_default(),
    }
}

/// Parse a JSON array of objects / JSONL object-per-line into a [`Sheet`].
/// Column order is first-seen key order; missing keys become empty cells.
pub fn parse_json(text: &str, source: String) -> Result<Sheet, String> {
    let text = strip_bom(text).trim();
    if text.is_empty() {
        return Err("empty file".to_string());
    }
    let mut objects: Vec<serde_json::Value> = Vec::new();
    if text.starts_with('[') {
        let v: serde_json::Value =
            serde_json::from_str(text).map_err(|e| format!("invalid JSON: {}", e))?;
        match v {
            serde_json::Value::Array(items) => objects = items,
            _ => return Err("top-level JSON must be an array or object-per-line".to_string()),
        }
    } else {
        for (idx, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let v: serde_json::Value =
                serde_json::from_str(line).map_err(|e| format!("line {}: {}", idx + 1, e))?;
            objects.push(v);
        }
    }
    if objects.is_empty() {
        return Err("no JSON values found".to_string());
    }
    // Scalar array (e.g. [1,2,3]) → single "value" column.
    if objects.iter().all(|v| !v.is_object()) {
        return Ok(Sheet {
            headers: vec!["value".to_string()],
            rows: objects.into_iter().map(|v| vec![json_value_to_string(&v)]).collect(),
            source,
        });
    }
    let mut headers: Vec<String> = Vec::new();
    for v in &objects {
        if let serde_json::Value::Object(map) = v {
            for k in map.keys() {
                if !headers.contains(k) {
                    headers.push(k.clone());
                }
            }
        }
    }
    if headers.is_empty() {
        return Err("no JSON objects found".to_string());
    }
    let mut rows = Vec::with_capacity(objects.len());
    for v in &objects {
        let mut row = vec![String::new(); headers.len()];
        if let serde_json::Value::Object(map) = v {
            for (i, h) in headers.iter().enumerate() {
                if let Some(val) = map.get(h) {
                    row[i] = json_value_to_string(val);
                }
            }
        }
        rows.push(row);
    }
    Ok(Sheet { headers, rows, source })
}

fn ext_of(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

/// Load any supported file. `delim_opt`/`header_opt` only apply to
/// delimiter-separated input; JSON detection is by extension (`.json`,
/// `.jsonl`, `.ndjson`) with a content sniff fallback.
pub fn load_file(
    path: &Path,
    delim_opt: Option<char>,
    header_opt: Option<bool>,
) -> Result<Sheet, String> {
    let raw = std::fs::read(path).map_err(|e| format!("cannot read file: {}", e))?;
    let text = String::from_utf8_lossy(&raw).into_owned();
    let source = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    let ext = ext_of(path);
    if matches!(ext.as_str(), "json" | "jsonl" | "ndjson") {
        return parse_json(&text, source);
    }
    if ext == "tsv" && delim_opt.is_none() {
        return parse_delimited(&text, Some('\t'), header_opt, source);
    }
    // Content sniff: JSON array/object-per-line without a JSON extension.
    let trimmed = strip_bom(&text).trim_start();
    if trimmed.starts_with('[') || trimmed.starts_with('{') {
        if let Ok(sheet) = parse_json(&text, source.clone()) {
            return Ok(sheet);
        }
    }
    parse_delimited(&text, delim_opt, header_opt, source)
}

/// Supported extensions for completion and validation messages.
pub const SHEET_EXTENSIONS: &[&str] = &[".csv", ".tsv", ".json", ".jsonl", ".ndjson", ".txt"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_line_handles_quotes() {
        assert_eq!(
            split_line(r#"a,"b,c",d"#, ',').unwrap(),
            vec!["a", "b,c", "d"]
        );
        assert_eq!(
            split_line(r#"a,"b""c",d"#, ',').unwrap(),
            vec!["a", r#"b"c"#, "d"]
        );
        assert!(split_line(r#"a,"bc,d"#, ',').is_none());
    }

    #[test]
    fn delimiter_detection_picks_semicolon() {
        let lines = vec!["a;b;c", "1;2;3", "4;5;6"];
        assert_eq!(detect_delimiter(&lines), ';');
    }

    #[test]
    fn header_heuristic() {
        let sheet =
            parse_delimited("name,age\nnueng,30\nmeli,25", None, None, "t".into()).unwrap();
        assert_eq!(sheet.headers, vec!["name", "age"]);
        assert_eq!(sheet.rows.len(), 2);
        let no_header =
            parse_delimited("1,2\n3,4", None, None, "t".into()).unwrap();
        assert_eq!(no_header.headers, vec!["col1", "col2"]);
        assert_eq!(no_header.rows.len(), 2);
    }

    #[test]
    fn ragged_rows_normalized() {
        let sheet = parse_delimited("a,b,c\n1,2", None, Some(true), "t".into()).unwrap();
        assert_eq!(sheet.rows[0], vec!["1", "2", ""]);
    }

    #[test]
    fn json_array_of_objects() {
        let sheet = parse_json(
            r#"[{"a":1,"b":"x"},{"b":"y","c":true}]"#,
            "t".into(),
        )
        .unwrap();
        assert_eq!(sheet.headers, vec!["a", "b", "c"]);
        assert_eq!(sheet.rows[0], vec!["1", "x", ""]);
        assert_eq!(sheet.rows[1], vec!["", "y", "true"]);
    }

    #[test]
    fn jsonl_and_scalars() {
        let sheet =
            parse_json("{\"a\":1}\n{\"a\":2}\n", "t".into()).unwrap();
        assert_eq!(sheet.rows.len(), 2);
        let scalars = parse_json("[1,2,3]", "t".into()).unwrap();
        assert_eq!(scalars.headers, vec!["value"]);
    }
}
