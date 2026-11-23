//! Presentation helpers for the `ndb` shell: box tables and schema trees.
//!
//! Everything is rendered as plain text so the result can pass through
//! `crate::utils::terminal::asciify` for Termux / ASCII-only terminals.

use crate::t;
use std::fmt::Write as _;

use super::db::{ColumnInfo, QueryResult, TableInfo};

/// Maximum characters shown for a single cell before it is elided.
pub const MAX_CELL_WIDTH: usize = 40;

/// Approximate display width (character count).
fn display_width(s: &str) -> usize {
    s.chars().count()
}

/// Collapse control characters and summarize `NULL` values.
fn cell_text(value: Option<&str>) -> String {
    match value {
        None => t!("ndb.null"),
        Some(v) => v.replace(['\n', '\r', '\t'], " ").trim().to_string(),
    }
}

/// Clamp a cell to `max` characters, appending an ellipsis when elided.
fn elide(s: &str, max: usize) -> String {
    if max == 0 || display_width(s) <= max {
        return s.to_string();
    }
    let keep = max.saturating_sub(1);
    let mut out: String = s.chars().take(keep).collect();
    out.push('…');
    out
}

/// Pad `s` on the right to `width` characters.
fn pad(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - w))
    }
}

fn border(left: char, mid: char, right: char, widths: &[usize]) -> String {
    let mut out = String::new();
    out.push(left);
    for (i, w) in widths.iter().enumerate() {
        if i > 0 {
            out.push(mid);
        }
        out.push_str(&"-".repeat(w + 2));
    }
    out.push(right);
    out.push('\n');
    out
}

fn row_line(cells: &[String], widths: &[usize], max_cell: usize) -> String {
    let mut out = String::new();
    out.push('|');
    for (i, w) in widths.iter().enumerate() {
        let raw = cells.get(i).map(|s| s.as_str()).unwrap_or("");
        out.push(' ');
        out.push_str(&pad(&elide(raw, max_cell), *w));
        out.push(' ');
        out.push('|');
    }
    out.push('\n');
    out
}

/// Render a result set as a box-drawn table.
pub fn format_box_table(
    headers: &[String],
    rows: &[Vec<Option<String>>],
    max_cell: usize,
) -> String {
    if headers.is_empty() {
        return String::new();
    }
    let cols = headers.len();
    let mut widths = vec![1usize; cols];
    for (i, h) in headers.iter().enumerate() {
        widths[i] = display_width(&elide(h, max_cell)).max(1);
    }
    for row in rows {
        for i in 0..cols {
            let text = cell_text(row.get(i).and_then(|v| v.as_deref()));
            widths[i] = widths[i].max(display_width(&elide(&text, max_cell)).max(1));
        }
    }

    let mut out = String::new();
    out.push_str(&border('┌', '┬', '┐', &widths));
    out.push_str(&row_line(headers, &widths, max_cell));
    out.push_str(&border('├', '┼', '┤', &widths));
    for row in rows {
        let cells: Vec<String> = (0..cols)
            .map(|i| cell_text(row.get(i).and_then(|v| v.as_deref())))
            .collect();
        out.push_str(&row_line(&cells, &widths, max_cell));
    }
    out.push_str(&border('└', '┴', '┘', &widths));
    out
}

/// Render a [`QueryResult`] plus a row-count/truncation footer.
pub fn format_query_result(result: &QueryResult, limit: usize) -> String {
    if result.columns.is_empty() {
        return t!("ndb.no_columns");
    }
    let mut out = format_box_table(&result.columns, &result.rows, MAX_CELL_WIDTH);
    if result.rows.is_empty() {
        let _ = writeln!(out, "{}", t!("ndb.rows_empty"));
    } else {
        let _ = writeln!(out, "{}", t!("ndb.rows_shown", result.rows.len()));
        if result.truncated {
            let _ = writeln!(out, "{}", t!("ndb.rows_truncated", limit));
        }
    }
    let _ = writeln!(out, "{}", t!("ndb.query_time", result.elapsed_ms));
    out
}

/// Detail suffix for a table entry, e.g. `(5 columns, 1200 rows)`.
pub fn table_detail(info: &TableInfo) -> String {
    let kind = if info.is_view() {
        format!("{} ", t!("ndb.kind_view"))
    } else {
        String::new()
    };
    format!(
        "({}{}, {})",
        kind,
        t!("ndb.columns_count", info.columns),
        t!("ndb.rows_count", info.rows)
    )
}

/// `show_tree`-style listing of table names under the database name.
pub fn format_table_list(db_name: &str, tables: &[TableInfo]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}", db_name);
    if tables.is_empty() {
        let _ = writeln!(out, "{}", t!("ndb.no_tables"));
        return out;
    }
    let last = tables.len() - 1;
    for (i, info) in tables.iter().enumerate() {
        let branch = if i == last { "└──" } else { "├──" };
        let _ = writeln!(out, "{} {}  {}", branch, info.name, table_detail(info));
    }
    out
}

/// A table paired with its columns, for the `viewf` schema tree.
pub struct TableSchema<'a> {
    pub info: &'a TableInfo,
    pub columns: &'a [ColumnInfo],
}

/// `show_tree`-style schema tree: tables, then their columns beneath them.
pub fn format_schema_tree(db_name: &str, schemas: &[TableSchema<'_>]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}", db_name);
    if schemas.is_empty() {
        let _ = writeln!(out, "{}", t!("ndb.no_tables"));
        return out;
    }
    let last = schemas.len() - 1;
    for (i, schema) in schemas.iter().enumerate() {
        let branch = if i == last { "└──" } else { "├──" };
        let _ = writeln!(
            out,
            "{} {}  {}",
            branch,
            schema.info.name,
            table_detail(schema.info)
        );
        let indent = if i == last { "    " } else { "│   " };
        let cols = schema.columns;
        for (j, col) in cols.iter().enumerate() {
            let col_branch = if j + 1 == cols.len() {
                "└──"
            } else {
                "├──"
            };
            let ty = if col.col_type.is_empty() {
                "ANY".to_string()
            } else {
                col.col_type.clone()
            };
            let extra = col.constraint();
            if extra.is_empty() {
                let _ = writeln!(out, "{}{} {}  {}", indent, col_branch, col.name, ty);
            } else {
                let _ = writeln!(
                    out,
                    "{}{} {}  {}  {}",
                    indent, col_branch, col.name, ty, extra
                );
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_table_pads_and_borders() {
        let headers = vec!["id".to_string(), "name".to_string()];
        let rows = vec![
            vec![Some("1".to_string()), Some("ann".to_string())],
            vec![Some("2".to_string()), None],
        ];
        let table = format_box_table(&headers, &rows, MAX_CELL_WIDTH);
        let lines: Vec<&str> = table.lines().collect();
        // top + header + separator + 2 rows + bottom
        assert_eq!(lines.len(), 6, "unexpected number of lines");
        assert!(lines[0].starts_with('┌') && lines[0].ends_with('┐'));
        assert!(lines[2].starts_with('├') && lines[2].ends_with('┤'));
        assert!(lines[5].starts_with('└') && lines[5].ends_with('┘'));
        // Every line must share the same character width.
        let width = lines[0].chars().count();
        for line in &lines {
            assert_eq!(line.chars().count(), width, "ragged line: {line}");
        }
        assert!(table.contains("NULL"), "NULL values must be labelled");
    }

    #[test]
    fn query_result_shows_timing_footer() {
        let result = QueryResult {
            columns: vec!["id".to_string()],
            rows: vec![vec![Some("1".to_string())]],
            truncated: false,
            elapsed_ms: 7,
        };
        let out = format_query_result(&result, 50);
        assert!(out.contains("7 ms"), "timing footer missing: {out}");
    }

    #[test]
    fn elide_clamps_long_cells() {
        let long = "x".repeat(100);
        let out = elide(&long, 10);
        assert_eq!(out.chars().count(), 10);
        assert!(out.ends_with('…'));
        assert_eq!(elide("short", 10), "short");
    }

    #[test]
    fn cell_text_flattens_newlines() {
        assert_eq!(cell_text(Some("a\nb\tc")), "a b c");
        assert_eq!(cell_text(None), t!("ndb.null"));
    }

    #[test]
    fn table_list_uses_tree_branches() {
        let tables = vec![
            TableInfo {
                name: "users".into(),
                kind: "table".into(),
                columns: 3,
                rows: 10,
            },
            TableInfo {
                name: "top".into(),
                kind: "view".into(),
                columns: 1,
                rows: 2,
            },
        ];
        let tree = format_table_list("app.db", &tables);
        let lines: Vec<&str> = tree.lines().collect();
        assert_eq!(lines[0], "app.db");
        assert!(lines[1].starts_with("├── users"));
        assert!(lines[2].starts_with("└── top"));
    }

    #[test]
    fn schema_tree_nests_columns() {
        let info = TableInfo {
            name: "users".into(),
            kind: "table".into(),
            columns: 2,
            rows: 1,
        };
        let columns = vec![
            ColumnInfo {
                name: "id".into(),
                col_type: "INTEGER".into(),
                not_null: false,
                primary_key: true,
            },
            ColumnInfo {
                name: "name".into(),
                col_type: "TEXT".into(),
                not_null: true,
                primary_key: false,
            },
        ];
        let schemas = vec![TableSchema {
            info: &info,
            columns: &columns,
        }];
        let tree = format_schema_tree("app.db", &schemas);
        assert!(tree.contains("── users"));
        assert!(tree.contains("├── id  INTEGER  PK"));
        assert!(tree.contains("└── name  TEXT  NOT NULL"));
    }
}
