use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::t;
use anyhow::{bail, Result};
use colored::*;
use std::path::{Path, PathBuf};

use super::fmt_doc as doc_fmt;
use super::fmt_sheet as sheet_fmt;

// ─── format registry ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Data,
    Doc,
    Sheet,
}

/// Map a raw extension to a canonical fmt format.
/// Data family: json, yaml, toml. Document family: txt, md, html, xml, docx,
/// pdf. Sheet family: csv, tsv, xlsx.
fn normalize_ext(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "json" => Some("json"),
        "yml" | "yaml" => Some("yaml"),
        "toml" => Some("toml"),
        "md" | "markdown" => Some("md"),
        "html" | "htm" => Some("html"),
        "xml" => Some("xml"),
        "docx" => Some("docx"),
        "txt" | "text" => Some("txt"),
        "pdf" => Some("pdf"),
        "csv" => Some("csv"),
        "tsv" => Some("tsv"),
        "xlsx" => Some("xlsx"),
        _ => None,
    }
}

fn native_family(fmt: &str) -> Family {
    match fmt {
        "json" | "yaml" | "toml" => Family::Data,
        "csv" | "tsv" | "xlsx" => Family::Sheet,
        _ => Family::Doc,
    }
}

fn family_label(family: Family) -> &'static str {
    match family {
        Family::Data => "data",
        Family::Doc => "doc",
        Family::Sheet => "sheet",
    }
}

/// Pick the pipeline family. `json` is deliberately contextual: from a data
/// file it stays data JSON, from a document file it is document JSON, and from
/// a sheet file it is an array of row objects.
fn resolve_family(from: &str, to: Option<&str>) -> std::result::Result<Family, (Family, Family)> {
    let from_kind = native_family(from);
    let Some(target) = to else {
        return Ok(from_kind);
    };
    // Every family can emit/parse its own JSON meaning.
    if target == "json" {
        return Ok(from_kind);
    }
    let target_kind = native_family(target);
    // A JSON source can be reinterpreted according to the requested target.
    if from == "json" {
        return Ok(target_kind);
    }
    if target_kind == from_kind {
        return Ok(from_kind);
    }
    // Sheets render naturally as document tables.
    if from_kind == Family::Sheet && target_kind == Family::Doc && matches!(target, "md" | "html") {
        return Ok(Family::Sheet);
    }
    Err((from_kind, target_kind))
}

// ─── argument parsing ──────────────────────────────────────────────────

struct FmtArgs {
    from: &'static str,
    file: PathBuf,
    to: Option<&'static str>,
    output: Option<PathBuf>,
}

fn parse_fmt_args(args: &str, cwd: &Path) -> Result<FmtArgs> {
    let args = args.trim();
    if args.is_empty() {
        bail!("{}", t!("cmd.fmt.usage"));
    }

    // Legacy syntax guard: `fmt json file.json ...` (bare format token first,
    // no dot in it) used to be valid; error out with an explanatory hint.
    let first_token = args.split_whitespace().next().unwrap_or("");
    if !first_token.contains('.')
        && first_token.to_lowercase() != "to"
        && normalize_ext(first_token).is_some()
    {
        bail!("{}", t!("cmd.fmt.legacy_syntax"));
    }

    // Extract -o / --output first (from the end).
    let (rest_clean, out_path) = {
        let r = args;
        if let Some(pos) = r.rfind(" -o ") {
            let out = r[pos + 4..].trim();
            if !out.is_empty() && !out.starts_with('-') {
                (r[..pos].trim(), Some(cwd.join(out)))
            } else {
                (r, None)
            }
        } else if let Some(pos) = r.rfind(" --output ") {
            let out = r[pos + 10..].trim();
            if !out.is_empty() && !out.starts_with('-') {
                (r[..pos].trim(), Some(cwd.join(out)))
            } else {
                (r, None)
            }
        } else {
            (r, None)
        }
    };

    // `to <fmt>`: the remainder after " to " must be a single supported ext.
    let (file_str, to_fmt) = match find_to_target(rest_clean) {
        Some((file, target)) => (file, normalize_ext(&target)),
        None => (rest_clean.to_string(), None),
    };

    if file_str.is_empty() {
        bail!("{}", t!("cmd.fmt.missing_file"));
    }

    // Resolve the source file from the cwd.
    let file_path = cwd.join(&file_str);
    if !file_path.exists() {
        bail!("{}", t!("cmd.fmt.file_not_found", file_str));
    }

    let file_ext = file_path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();
    let from = match normalize_ext(&file_ext) {
        Some(f) => f,
        None => bail!("{}", t!("cmd.fmt.unknown_format", file_ext)),
    };

    // If no explicit target was given, fall back to the -o output extension.
    let to = match to_fmt {
        Some(t) => Some(t),
        None => out_path
            .as_ref()
            .map(|p| p.extension().map(|e| e.to_string_lossy().to_string()))
            .flatten()
            .and_then(|e| normalize_ext(&e)),
    };

    Ok(FmtArgs {
        from,
        file: file_path,
        to,
        output: out_path,
    })
}

/// Find " to <ext>" where the remainder is exactly one supported ext token.
/// Also handles the common "file name with spaces to yaml" case.
fn find_to_target(rest: &str) -> Option<(String, String)> {
    let lower = rest.to_lowercase();
    let mut search_from = 0usize;
    while let Some(pos) = lower[search_from..].find(" to ") {
        let abs = search_from + pos;
        let file = rest[..abs].trim();
        let target = rest[abs + 4..].trim();
        if !file.is_empty() && target.split_whitespace().count() == 1 {
            // Must be a valid format, otherwise it's just an " to " inside
            // some other keyword; keep looking.
            let candidate = target
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches(|c| c == '"' || c == '\'');
            if let Some(norm) = normalize_ext(&candidate.to_lowercase()) {
                return Some((file.to_string(), norm.to_string()));
            }
        }
        search_from = abs + 4;
    }
    None
}

// ─── data-family conversion (json / yaml / toml) ───────────────────────

fn parse_as_json(content: &str) -> Result<serde_json::Value> {
    serde_json::from_str(content).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.invalid_json", e)))
}

fn parse_as_yaml(content: &str) -> Result<serde_json::Value> {
    serde_yaml::from_str(content).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.invalid_yaml", e)))
}

fn parse_as_toml(content: &str) -> Result<serde_json::Value> {
    let value: toml::Value = toml::from_str(content)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.invalid_toml", e)))?;
    serde_json::to_value(value).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.toml_conv_err", e)))
}

fn serialize_as_json(value: &serde_json::Value) -> Result<String> {
    serde_json::to_string_pretty(value)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.json_ser_err", e)))
}

fn serialize_as_yaml(value: &serde_json::Value) -> Result<String> {
    serde_yaml::to_string(value).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.yaml_ser_err", e)))
}

fn serialize_as_toml(value: &serde_json::Value) -> Result<String> {
    let toml_value = toml::Value::try_from(value.clone())
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.toml_ser_err", e)))?;
    toml::to_string_pretty(&toml_value)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.toml_ser_err", e)))
}

fn parse_data(content: &str, fmt: &str) -> Result<serde_json::Value> {
    match fmt {
        "json" => parse_as_json(content),
        "yaml" => parse_as_yaml(content),
        "toml" => parse_as_toml(content),
        _ => bail!("{}", t!("cmd.fmt.unknown_format", fmt)),
    }
}

fn serialize_data(value: &serde_json::Value, fmt: &str) -> Result<String> {
    match fmt {
        "json" => serialize_as_json(value),
        "yaml" => serialize_as_yaml(value),
        "toml" => serialize_as_toml(value),
        _ => bail!("{}", t!("cmd.fmt.unknown_format", fmt)),
    }
}

fn print_usage() {
    println!("{}", t!("fmt.help_usage").cyan().bold());
    println!("  {}", t!("cmd.fmt.usage"));
    println!();
    println!("{}", t!("fmt.help_no_target").green());
    println!();
    println!("{}", t!("fmt.help_examples").green());
    println!("  {}", t!("fmt.help_example1"));
    println!("  {}", t!("fmt.help_example2"));
    println!("  {}", t!("fmt.help_example3"));
    println!("  {}", t!("fmt.help_example4"));
    println!("  {}", t!("fmt.help_example5"));
    println!("  {}", t!("fmt.help_example6"));
    println!("  {}", t!("fmt.help_example7"));
    println!("  {}", t!("fmt.help_example8"));
    println!("  {}", t!("fmt.help_example9"));
    println!("  {}", t!("fmt.help_example10"));
}

// ─── command entry ─────────────────────────────────────────────────────

pub fn cmd_fmt(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let cwd = _nav.current_path().to_path_buf();

    let fmt_args = match parse_fmt_args(args, &cwd) {
        Ok(a) => a,
        Err(e) => {
            print_error(&e.to_string());
            println!();
            print_usage();
            return Ok(false);
        }
    };

    // Cross-family check.
    let family = match resolve_family(fmt_args.from, fmt_args.to) {
        Ok(family) => family,
        Err((from_kind, to_kind)) => {
            print_error(&t!(
                "cmd.fmt.cross_family",
                fmt_args.from,
                fmt_args.to.unwrap_or(""),
                family_label(from_kind),
                family_label(to_kind)
            ));
            return Ok(false);
        }
    };

    match family {
        Family::Data => run_data_conversion(&fmt_args),
        Family::Doc => run_doc_conversion(&fmt_args),
        Family::Sheet => run_sheet_conversion(&fmt_args),
    }
}

fn run_data_conversion(args: &FmtArgs) -> Result<bool> {
    let content_raw = match std::fs::read_to_string(&args.file) {
        Ok(c) => c,
        Err(e) => {
            print_error(&t!("cmd.fmt.read_fail", e));
            return Ok(false);
        }
    };
    let content = content_raw.trim_start_matches('\u{feff}').to_string();

    if content.contains('\0') {
        print_error(&t!("cmd.fmt.binary_file"));
        return Ok(false);
    }

    let value = match parse_data(&content, args.from) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };

    let target_fmt = args.to.unwrap_or(args.from);
    let output = match serialize_data(&value, target_fmt) {
        Ok(s) => s,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };

    emit_output(&output, args.output.as_ref(), target_fmt)
}

fn emit_output(text: &str, out: Option<&PathBuf>, fmt: &str) -> Result<bool> {
    match out {
        Some(path) => {
            std::fs::write(path, text)?;
            print_success(&t!("cmd.fmt.written_to", path.display()));
        }
        // Stdout gets syntax colors (gated on `setc` ON + ANSI support);
        // only the trailing newline of the serialized text is dropped so
        // the shell prompt stays on its own line.
        None => crate::output::print_syntax_text(text.trim_end(), Some(fmt)),
    }
    Ok(false)
}

fn run_doc_conversion(args: &FmtArgs) -> Result<bool> {
    if args.from == "pdf" {
        print_error(&t!("cmd.fmt.pdf_source_unsupported"));
        return Ok(false);
    }

    let doc = doc_fmt::read_document(&args.file, args.from)?;

    // Binary outputs need a file path.
    let to = args.to.unwrap_or(args.from);
    if matches!(to, "docx" | "pdf") && args.output.is_none() {
        if to == "pdf" {
            print_error(&t!("cmd.fmt.pdf_needs_output"));
        } else {
            print_error(&t!("cmd.fmt.docx_needs_output"));
        }
        return Ok(false);
    }

    match to {
        "docx" => {
            let path = args.output.as_ref().unwrap();
            if let Err(e) = doc_fmt::write_docx(&doc, path) {
                print_error(&e.to_string());
                return Ok(false);
            }
            print_success(&t!("cmd.fmt.written_to", path.display()));
            Ok(false)
        }
        #[cfg(not(target_os = "android"))]
        "pdf" => {
            let path = args.output.as_ref().unwrap();
            if let Err(e) = doc_fmt::write_pdf(&doc, path) {
                print_error(&e.to_string());
                return Ok(false);
            }
            print_success(&t!("cmd.fmt.written_to", path.display()));
            Ok(false)
        }
        #[cfg(target_os = "android")]
        "pdf" => {
            print_error(&t!("cmd.fmt.pdf_unsupported"));
            Ok(false)
        }
        _ => {
            let output = doc_fmt::serialize_document_text(&doc, to)
                .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.doc_ser_err", to, e)))?;
            emit_output(&output, args.output.as_ref(), to)
        }
    }
}

fn run_sheet_conversion(args: &FmtArgs) -> Result<bool> {
    let sheet = match sheet_fmt::read_sheet(&args.file, args.from) {
        Ok(sheet) => sheet,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };

    let target = args.to.unwrap_or(args.from);
    if target == "xlsx" && args.output.is_none() {
        print_error(&t!("cmd.fmt.xlsx_needs_output"));
        return Ok(false);
    }

    if target == "xlsx" {
        let path = args.output.as_ref().unwrap();
        if let Err(e) = sheet_fmt::write_xlsx(&sheet, path) {
            print_error(&e.to_string());
            return Ok(false);
        }
        print_success(&t!("cmd.fmt.written_to", path.display()));
        return Ok(false);
    }

    let output = match sheet_fmt::serialize_text(&sheet, target) {
        Ok(output) => output,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    emit_output(&output, args.output.as_ref(), target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_sources_use_the_doc_family() {
        assert_eq!(resolve_family("txt", None), Ok(Family::Doc));
        assert_eq!(resolve_family("txt", Some("json")), Ok(Family::Doc));
        assert_eq!(resolve_family("txt", Some("pdf")), Ok(Family::Doc));
        assert_eq!(resolve_family("docx", Some("txt")), Ok(Family::Doc));
    }

    #[test]
    fn sheet_sources_use_the_sheet_family() {
        assert_eq!(resolve_family("csv", None), Ok(Family::Sheet));
        assert_eq!(resolve_family("csv", Some("json")), Ok(Family::Sheet));
        assert_eq!(resolve_family("csv", Some("xlsx")), Ok(Family::Sheet));
        assert_eq!(resolve_family("tsv", Some("xlsx")), Ok(Family::Sheet));
        // Sheets may render as tables.
        assert_eq!(resolve_family("csv", Some("md")), Ok(Family::Sheet));
        assert_eq!(resolve_family("csv", Some("html")), Ok(Family::Sheet));
    }

    #[test]
    fn json_sources_follow_the_requested_target() {
        assert_eq!(resolve_family("json", None), Ok(Family::Data));
        assert_eq!(resolve_family("json", Some("yaml")), Ok(Family::Data));
        assert_eq!(resolve_family("json", Some("json")), Ok(Family::Data));
        assert_eq!(resolve_family("json", Some("txt")), Ok(Family::Doc));
        assert_eq!(resolve_family("json", Some("csv")), Ok(Family::Sheet));
    }

    #[test]
    fn incompatible_families_are_rejected() {
        assert!(resolve_family("csv", Some("pdf")).is_err());
        assert!(resolve_family("docx", Some("csv")).is_err());
        assert!(resolve_family("yaml", Some("html")).is_err());
    }
}
