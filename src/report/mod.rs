mod txt;
mod html;
mod json;
mod md;
#[cfg(not(target_os = "android"))]
mod pdf;
#[cfg(not(target_os = "android"))]
mod docx;
#[cfg(not(target_os = "android"))]
mod xlsx;

pub use txt::generate_txt_report;
pub use html::HtmlReportGenerator;
pub use json::JsonReportGenerator;
pub use md::MarkdownReportGenerator;
#[cfg(not(target_os = "android"))]
pub use pdf::generate_pdf_report;
#[cfg(not(target_os = "android"))]
pub use docx::generate_docx_report;
#[cfg(not(target_os = "android"))]
pub use xlsx::generate_xlsx_report;

use crate::config::Config;
use crate::filetype::{is_supported_format_with_config, FormatConfig};
use anyhow::{bail, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportFormat {
    Txt,
    Html,
    Json,
    Md,
    Pdf,
    Docx,
    Xlsx,
}

impl ReportFormat {
    pub fn from_extension(filename: &str) -> Self {
        let path = Path::new(filename);
        match path.extension().and_then(|e| e.to_str()) {
            Some("html" | "htm") => ReportFormat::Html,
            Some("json") => ReportFormat::Json,
            Some("md" | "markdown") => ReportFormat::Md,
            Some("pdf") => ReportFormat::Pdf,
            Some("docx") => ReportFormat::Docx,
            Some("xlsx") => ReportFormat::Xlsx,
            _ => ReportFormat::Txt,
        }
    }
    
    pub fn extension(&self) -> &str {
        match self {
            ReportFormat::Txt => "txt",
            ReportFormat::Html => "html",
            ReportFormat::Json => "json",
            ReportFormat::Md => "md",
            ReportFormat::Pdf => "pdf",
            ReportFormat::Docx => "docx",
            ReportFormat::Xlsx => "xlsx",
        }
    }

    pub fn is_binary(&self) -> bool {
        #[cfg(not(target_os = "android"))]
        { matches!(self, ReportFormat::Pdf | ReportFormat::Docx | ReportFormat::Xlsx) }
        #[cfg(target_os = "android")]
        { false }
    }
}

/// Walk files in a directory, skipping ignored directories, and classify
/// each file as supported (text) or unsupported (binary/ignored).
/// Returns (supported_paths, unsupported_paths), both sorted.
pub(crate) fn collect_report_files(
    dir_path: &Path,
    max_depth: usize,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let ignored_dirs = Config::global_get_ignored_dirs();
    let fmt_cfg = FormatConfig::from_global();

    let walker = WalkDir::new(dir_path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 { return true; }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                return !ignored_dirs.contains(&name);
            }
            true
        });

    let mut supported = Vec::new();
    let mut unsupported = Vec::new();

    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if is_supported_format_with_config(&path, &fmt_cfg) {
                supported.push(path);
            } else {
                unsupported.push(path);
            }
        }
    }

    supported.sort();
    unsupported.sort();
    (supported, unsupported)
}

pub fn generate_report(dir_path: &Path, format: ReportFormat) -> Result<String> {
    let dir_name = dir_path.file_name().map(|n| n.to_string_lossy()).unwrap_or_else(|| std::borrow::Cow::Borrowed("root"));
    let output_filename = format!("{}.{}", dir_name, format.extension());
    let output_path = crate::output::build_output_path(&output_filename);
    
    generate_report_to(dir_path, format, &output_path.to_string_lossy())
}

pub fn generate_report_to(dir_path: &Path, format: ReportFormat, output_file: &str) -> Result<String> {
    let output_path = std::path::Path::new(output_file).to_path_buf();
    
    match format {
        ReportFormat::Txt => {
            generate_txt_report(dir_path, &output_path)?;
        }
        ReportFormat::Html => {
            HtmlReportGenerator::generate(dir_path, &output_path, None)?;
        }
        ReportFormat::Json => {
            JsonReportGenerator::generate(dir_path, &output_path, true, None)?;
        }
        ReportFormat::Md => {
            MarkdownReportGenerator::generate(dir_path, &output_path, None)?;
        }
        #[cfg(not(target_os = "android"))]
        ReportFormat::Pdf => {
            generate_pdf_report(dir_path, &output_path)?;
        }
        #[cfg(not(target_os = "android"))]
        ReportFormat::Docx => {
            generate_docx_report(dir_path, &output_path)?;
        }
        #[cfg(not(target_os = "android"))]
        ReportFormat::Xlsx => {
            generate_xlsx_report(dir_path, &output_path)?;
        }
        #[cfg(target_os = "android")]
        ReportFormat::Pdf | ReportFormat::Docx | ReportFormat::Xlsx => {
            bail!("{} report format is not supported on Android", format.extension().to_uppercase());
        }
    }
    
    println!("Report saved to: {}", output_path.display());
    Ok(output_path.to_string_lossy().to_string())
}

// Add this function to generate report as string (for clipboard)
pub fn generate_report_to_string(dir_path: &Path, format: ReportFormat) -> Result<String> {
    if format.is_binary() {
        bail!("{} report cannot be copied to clipboard (binary format)", format.extension().to_uppercase());
    }
    // Create a temporary file
    let temp_dir = std::env::temp_dir();
    let temp_filename = format!("ntc_temp_{}.{}", 
        std::process::id(), 
        format.extension()
    );
    let temp_path = temp_dir.join(temp_filename);
    
    // Generate report to temp file using existing function
    generate_report_to(dir_path, format, &temp_path.to_string_lossy())?;
    
    // Read the content
    let content = std::fs::read_to_string(&temp_path)?;
    
    // Clean up temp file
    let _ = std::fs::remove_file(temp_path);
    
    Ok(content)
}

