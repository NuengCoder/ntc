//! Shared `report project` parsing and output naming.

use crate::shell::commands::builtin::report::report::ReportFormat;
use std::path::PathBuf;

use super::report_view::tokenize_spans;

/// `report project <fmt> [path] [--name <output>]` request.
#[derive(Debug)]
pub(crate) struct ProjectReport {
    pub format: ReportFormat,
    pub path: PathBuf,
    pub name: Option<String>,
}

/// Parse a report format name (case-insensitive).
fn parse_report_format(raw: &str) -> Option<ReportFormat> {
    match raw.to_lowercase().as_str() {
        "txt" | "text" => Some(ReportFormat::Txt),
        "html" | "htm" => Some(ReportFormat::Html),
        "json" => Some(ReportFormat::Json),
        "md" | "markdown" => Some(ReportFormat::Md),
        "pdf" => Some(ReportFormat::Pdf),
        "docx" => Some(ReportFormat::Docx),
        "xlsx" => Some(ReportFormat::Xlsx),
        _ => None,
    }
}

/// Parse `project <fmt> [path] [--name <output>]` (the part after
/// `report project`). `Err` is a lang key for the usage message.
pub(crate) fn parse_project_report(args: &str) -> Result<ProjectReport, &'static str> {
    let usage = "cmd.report.project_usage";
    let toks: Vec<String> = tokenize_spans(args)
        .into_iter()
        .map(|(s, _, _)| s)
        .collect();
    let mut format: Option<ReportFormat> = None;
    let mut path: Option<String> = None;
    let mut name: Option<String> = None;
    let mut i = 0;
    while i < toks.len() {
        match toks[i].as_str() {
            "--name" | "-n" | "-o" | "--output" => match toks.get(i + 1) {
                Some(v) if !v.trim().is_empty() => {
                    name = Some(v.clone());
                    i += 1;
                }
                _ => return Err(usage),
            },
            other if other.starts_with('-') => return Err(usage),
            other => {
                if format.is_none() {
                    format = Some(parse_report_format(other).ok_or(usage)?);
                } else if path.is_none() {
                    path = Some(other.to_string());
                } else {
                    return Err(usage);
                }
            }
        }
        i += 1;
    }
    Ok(ProjectReport {
        format: format.ok_or(usage)?,
        path: PathBuf::from(path.unwrap_or_else(|| ".".to_string())),
        name,
    })
}

/// Output path for `--name`: used as given, with the format extension
/// appended when the name has none. Parent directories are created.
pub(crate) fn project_output_path(name: &str, format: ReportFormat) -> PathBuf {
    let mut out = PathBuf::from(name);
    if out.extension().is_none() {
        out.set_extension(format.extension());
    }
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_report_args() {
        let p = parse_project_report("html").unwrap();
        assert_eq!(p.format, ReportFormat::Html);
        assert_eq!(p.path, PathBuf::from("."));
        assert!(p.name.is_none());

        let p = parse_project_report("md src --name out").unwrap();
        assert_eq!(p.format, ReportFormat::Md);
        assert_eq!(p.path, PathBuf::from("src"));
        assert_eq!(p.name.as_deref(), Some("out"));

        // Flags may come before the path.
        let p = parse_project_report("json --name r1 src").unwrap();
        assert_eq!(p.path, PathBuf::from("src"));
        assert_eq!(p.name.as_deref(), Some("r1"));

        // Formats are case-insensitive; quoted names keep their spaces.
        let p = parse_project_report("HTML -n \"my report\"").unwrap();
        assert_eq!(p.format, ReportFormat::Html);
        assert_eq!(p.name.as_deref(), Some("my report"));

        // Errors: missing/invalid format, extra target, dangling flag.
        assert!(parse_project_report("").is_err());
        assert!(parse_project_report("nope").is_err());
        assert!(parse_project_report("txt a b").is_err());
        assert!(parse_project_report("txt --name").is_err());
        assert!(parse_project_report("txt --bogus").is_err());
    }

    #[test]
    fn project_output_names_get_the_format_extension() {
        assert_eq!(
            project_output_path("out", ReportFormat::Html),
            PathBuf::from("out.html")
        );
        // An explicit extension wins.
        assert_eq!(
            project_output_path("out.txt", ReportFormat::Html),
            PathBuf::from("out.txt")
        );
        let dir = tempfile::tempdir().unwrap();
        let named = dir.path().join("nested/out");
        assert_eq!(
            project_output_path(&named.to_string_lossy(), ReportFormat::Md),
            dir.path().join("nested/out.md")
        );
        assert!(dir.path().join("nested").is_dir());
    }
}
