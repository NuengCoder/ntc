// src/fake/format.rs
//! Output serializers for generated `fake` rows.

use std::path::Path;

use anyhow::Result;

/// A generated result set: the (optional) table/group name, ordered column
/// names and the generated rows (each row has one cell per column).
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fmt {
    Txt,
    Json,
    Sql,
    Csv,
    Xlsx,
}

impl Fmt {
    /// Parse `json|txt|text|sql|csv|xlsx` (case-insensitive).
    pub fn parse(s: &str) -> Option<Fmt> {
        match s.trim().to_lowercase().as_str() {
            "txt" | "text" | "plain" => Some(Fmt::Txt),
            "json" => Some(Fmt::Json),
            "sql" => Some(Fmt::Sql),
            "csv" => Some(Fmt::Csv),
            "xlsx" | "excel" => Some(Fmt::Xlsx),
            _ => None,
        }
    }

    /// Infer the format from a file extension.
    pub fn from_ext(ext: &str) -> Option<Fmt> {
        Fmt::parse(ext)
    }

    pub fn ext(self) -> &'static str {
        match self {
            Fmt::Txt => "txt",
            Fmt::Json => "json",
            Fmt::Sql => "sql",
            Fmt::Csv => "csv",
            Fmt::Xlsx => "xlsx",
        }
    }
}

/// Render a text-based format. Returns `None` for `xlsx` (binary writer).
pub fn to_text(fmt: Fmt, table: &Table) -> Option<String> {
    match fmt {
        Fmt::Txt => Some(render_txt(table)),
        Fmt::Json => Some(render_json(table)),
        Fmt::Sql => Some(render_sql(table)),
        Fmt::Csv => Some(render_csv(table)),
        Fmt::Xlsx => None,
    }
}

fn render_txt(table: &Table) -> String {
    if table.rows.is_empty() {
        return String::new();
    }
    let mut widths: Vec<usize> = table.columns.iter().map(|c| c.len()).collect();
    for row in &table.rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }
    let mut out = String::new();
    let header: Vec<String> = table
        .columns
        .iter()
        .enumerate()
        .map(|(i, c)| format!("{:<w$}", c, w = widths[i]))
        .collect();
    out.push_str(&header.join("  "));
    out.push('\n');
    let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
    out.push_str(&sep.join("  "));
    out.push('\n');
    for row in &table.rows {
        let line: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:<w$}", c, w = widths.get(i).copied().unwrap_or(c.len())))
            .collect();
        out.push_str(&line.join("  "));
        out.push('\n');
    }
    out
}

fn render_json(table: &Table) -> String {
    let mut out = String::from("[\n");
    for (ri, row) in table.rows.iter().enumerate() {
        out.push_str("  {");
        for (ci, col) in table.columns.iter().enumerate() {
            let val = row.get(ci).map(String::as_str).unwrap_or("");
            out.push_str(&format!(
                "\"{}\": \"{}\"",
                json_escape(col),
                json_escape(val)
            ));
            if ci + 1 < table.columns.len() {
                out.push_str(", ");
            }
        }
        out.push('}');
        if ri + 1 < table.rows.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("]\n");
    out
}

fn render_csv(table: &Table) -> String {
    let mut out = String::new();
    out.push_str(
        &table
            .columns
            .iter()
            .map(|c| csv_escape(c))
            .collect::<Vec<_>>()
            .join(","),
    );
    out.push('\n');
    for row in &table.rows {
        out.push_str(
            &row.iter()
                .map(|c| csv_escape(c))
                .collect::<Vec<_>>()
                .join(","),
        );
        out.push('\n');
    }
    out
}

fn render_sql(table: &Table) -> String {
    let name = sanitize_ident(&table.name);
    let cols: Vec<String> = table.columns.iter().map(|c| sanitize_ident(c)).collect();
    let col_list = cols.join(", ");
    let mut out = String::new();
    out.push_str(&format!("INSERT INTO {} ({}) VALUES\n", name, col_list));
    let n = table.rows.len();
    for (ri, row) in table.rows.iter().enumerate() {
        let vals: Vec<String> = row.iter().map(|v| sql_value(v)).collect();
        let suffix = if ri + 1 < n { "," } else { ";" };
        out.push_str(&format!("  ({}){}\n", vals.join(", "), suffix));
    }
    out
}

/// Write an `.xlsx` workbook with a single sheet named after the group.
#[cfg(not(target_os = "android"))]
pub fn write_xlsx(table: &Table, path: &Path) -> Result<()> {
    use rust_xlsxwriter::{Color, Format, Workbook};

    let mut wb = Workbook::new();
    let header_fmt = Format::new()
        .set_bold()
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x2E75B6));
    let sheet_name = sanitize_sheet_name(&table.name);

    let ws = wb.add_worksheet().set_name(sheet_name.as_str())?;
    for (ci, col) in table.columns.iter().enumerate() {
        ws.write_string_with_format(0, ci as u16, col.as_str(), &header_fmt)?;
    }
    for (ri, row) in table.rows.iter().enumerate() {
        for (ci, cell) in row.iter().enumerate() {
            ws.write_string((ri + 1) as u32, ci as u16, cell.as_str())?;
        }
    }
    wb.save(path)
        .map_err(|e| anyhow::anyhow!("failed to save xlsx: {e}"))?;
    Ok(())
}

#[cfg(target_os = "android")]
pub fn write_xlsx(_table: &Table, _path: &Path) -> Result<()> {
    anyhow::bail!("xlsx output is not available on this platform")
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
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

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Bare when the value is numeric or a boolean; string-quoted otherwise.
fn sql_value(v: &str) -> String {
    let numeric = !(v.len() > 1 && v.starts_with('0') && !v.starts_with("0."))
        && (v.parse::<i64>().is_ok() || v.parse::<f64>().is_ok());
    if numeric || v == "true" || v == "false" {
        return v.to_string();
    }
    format!("'{}'", v.replace('\'', "''"))
}

/// Quote an identifier only when it is not a simple bare word.
fn sanitize_ident(name: &str) -> String {
    let bare = !name.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !name.chars().next().is_some_and(|c| c.is_ascii_digit());
    if bare {
        name.to_string()
    } else {
        format!("\"{}\"", name.replace('"', "\"\""))
    }
}

/// Excel sheet names: ≤31 chars, no `[]:*?/\`.
#[cfg(not(target_os = "android"))]
fn sanitize_sheet_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if "[]:*?/\\".contains(c) { '_' } else { c })
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "fake".to_string()
    } else {
        trimmed.chars().take(31).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Table {
        Table {
            name: "users".into(),
            columns: vec!["username".into(), "age".into()],
            rows: vec![
                vec!["amy".into(), "41".into()],
                vec!["bob, jr".into(), "52".into()],
            ],
        }
    }

    #[test]
    fn sql_quotes_strings_and_leaves_numbers_bare() {
        let sql = render_sql(&sample());
        assert!(sql.starts_with("INSERT INTO users (username, age) VALUES"));
        assert!(sql.contains("('amy', 41)"));
        assert!(sql.contains("('bob, jr', 52)"));
        assert!(sql.trim_end().ends_with(';'));
    }

    #[test]
    fn csv_escapes_commas() {
        let csv = render_csv(&sample());
        assert!(csv.contains("\"bob, jr\""));
    }

    #[test]
    fn json_preserves_column_order() {
        let json = render_json(&sample());
        let user = json.find("username").unwrap();
        let age = json.find("age").unwrap();
        assert!(user < age);
    }

    #[test]
    fn sql_escapes_quotes() {
        let t = Table {
            name: "t".into(),
            columns: vec!["c".into()],
            rows: vec![vec!["O'Brien".into()]],
        };
        assert!(render_sql(&t).contains("'O''Brien'"));
    }
}
