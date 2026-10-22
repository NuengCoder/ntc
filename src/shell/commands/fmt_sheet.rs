//! Spreadsheet-family conversions for the `fmt` command:
//! csv / tsv / json in, csv / tsv / xlsx / json / md / html out.

use super::fmt_doc::{self, Block, Document, Inline};
use crate::nsheet::{self, Sheet};
use anyhow::{bail, Result};
use std::path::Path;

fn source_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

/// Read a sheet source. `xlsx` is intentionally write-only.
pub fn read_sheet(path: &Path, ext: &str) -> Result<Sheet> {
    if ext == "xlsx" {
        bail!("{}", crate::t!("cmd.fmt.xlsx_source_unsupported"));
    }
    let raw = std::fs::read(path)?;
    let text = String::from_utf8_lossy(&raw).into_owned();
    let source = source_name(path);
    match ext {
        "csv" => nsheet::parse::parse_delimited(&text, Some(','), None, source)
            .map_err(anyhow::Error::msg),
        "tsv" => nsheet::parse::parse_delimited(&text, Some('\t'), None, source)
            .map_err(anyhow::Error::msg),
        "json" => nsheet::parse::parse_json(&text, source).map_err(anyhow::Error::msg),
        _ => bail!("unsupported sheet extension: {}", ext),
    }
}

fn escape_field(value: &str, delim: char) -> String {
    if value.contains(delim) || value.contains('"') || value.contains('\n') || value.contains('\r')
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn write_row(out: &mut String, cells: &[String], delim: char) {
    for (i, cell) in cells.iter().enumerate() {
        if i > 0 {
            out.push(delim);
        }
        out.push_str(&escape_field(cell, delim));
    }
    out.push('\n');
}

/// Serialize a sheet as delimiter-separated text (header row included).
pub fn to_delimited(sheet: &Sheet, delim: char) -> String {
    let mut out = String::new();
    write_row(&mut out, &sheet.headers, delim);
    for row in &sheet.rows {
        write_row(&mut out, row, delim);
    }
    out
}

/// Serialize a sheet as an array of row objects.
pub fn to_json(sheet: &Sheet) -> String {
    format!("{}\n", nsheet::to_json(&sheet.headers, &sheet.rows))
}

/// Render a sheet as a single document table (for md/html output).
pub fn to_document(sheet: &Sheet) -> Document {
    let header = Some(
        sheet
            .headers
            .iter()
            .map(|h| vec![Inline::Text(h.clone())])
            .collect(),
    );
    let rows = sheet
        .rows
        .iter()
        .map(|row| {
            (0..sheet.headers.len())
                .map(|i| {
                    vec![Inline::Text(
                        row.get(i).cloned().unwrap_or_default().to_string(),
                    )]
                })
                .collect()
        })
        .collect();
    Document {
        blocks: vec![Block::Table { header, rows }],
    }
}

/// Write a sheet as `.xlsx`. Numbers are written as numbers when the value
/// is canonical (so leading-zero IDs and long digit strings stay text).
#[cfg(not(target_os = "android"))]
pub fn write_xlsx(sheet: &Sheet, path: &Path) -> Result<()> {
    use rust_xlsxwriter::{Color, Format, Workbook};

    fn number_value(s: &str) -> Option<f64> {
        let t = s.trim();
        if t.is_empty() || t.len() > 15 || t.starts_with('+') {
            return None;
        }
        let digits = t.strip_prefix('-').unwrap_or(t);
        if digits.len() > 1 && digits.starts_with('0') && !digits.starts_with("0.") {
            return None;
        }
        t.parse::<f64>().ok()
    }

    let mut wb = Workbook::new();
    let header_fmt = Format::new()
        .set_bold()
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x2E75B6));
    let ws = wb.add_worksheet().set_name("Data")?;

    for (col, header) in sheet.headers.iter().enumerate() {
        ws.write_string_with_format(0, col as u16, header, &header_fmt)?;
    }
    for (row, values) in sheet.rows.iter().enumerate() {
        for (col, value) in values.iter().enumerate() {
            if value.is_empty() {
                continue;
            }
            match number_value(value) {
                Some(n) => ws.write_number((row + 1) as u32, col as u16, n)?,
                None => ws.write_string((row + 1) as u32, col as u16, value)?,
            };
        }
    }

    wb.save(path)?;
    Ok(())
}

#[cfg(target_os = "android")]
pub fn write_xlsx(_sheet: &Sheet, _path: &Path) -> Result<()> {
    bail!("XLSX output is not supported on Android")
}

/// Serialize a sheet to any text target.
pub fn serialize_text(sheet: &Sheet, target: &str) -> Result<String> {
    match target {
        "csv" => Ok(to_delimited(sheet, ',')),
        "tsv" => Ok(to_delimited(sheet, '\t')),
        "json" => Ok(to_json(sheet)),
        "md" | "html" => {
            let doc = to_document(sheet);
            fmt_doc::serialize_document_text(&doc, target)
        }
        _ => bail!("unsupported sheet target: {}", target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Sheet {
        Sheet {
            headers: vec!["name".into(), "note".into()],
            rows: vec![
                vec!["alice".into(), "a,b".into()],
                vec!["bob".into(), "say \"hi\"".into()],
            ],
            source: "sample.csv".into(),
        }
    }

    #[test]
    fn delimited_round_trip_quotes() {
        let text = to_delimited(&sample(), ',');
        let back =
            nsheet::parse::parse_delimited(&text, Some(','), Some(true), "t".into()).unwrap();
        assert_eq!(back.headers, vec!["name", "note"]);
        assert_eq!(back.rows[0], vec!["alice", "a,b"]);
        assert_eq!(back.rows[1], vec!["bob", "say \"hi\""]);
    }

    #[test]
    fn sheet_to_json_md_and_html() {
        let json = serialize_text(&sample(), "json").unwrap();
        assert!(json.contains("\"name\""), "got:\n{}", json);
        assert!(json.contains("alice"), "got:\n{}", json);
        let md = serialize_text(&sample(), "md").unwrap();
        assert!(md.contains("| name | note |"), "got:\n{}", md);
        let html = serialize_text(&sample(), "html").unwrap();
        assert!(html.contains("<table>"), "got:\n{}", html);
    }

    #[test]
    fn xlsx_source_is_rejected() {
        match read_sheet(Path::new("book.xlsx"), "xlsx") {
            Ok(_) => panic!("xlsx source must be rejected"),
            Err(err) => assert!(!err.to_string().is_empty()),
        }
    }

    #[cfg(not(target_os = "android"))]
    #[test]
    fn xlsx_write_smoke() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.xlsx");
        write_xlsx(&sample(), &path).unwrap();
        let len = std::fs::metadata(&path).unwrap().len();
        assert!(len > 100, "xlsx too small: {}", len);
    }
}
