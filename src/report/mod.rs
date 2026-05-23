mod txt;
mod html;
mod json;
mod md;

pub use txt::generate_txt_report;
pub use html::HtmlReportGenerator;
pub use json::JsonReportGenerator;
pub use md::MarkdownReportGenerator;

use anyhow::{Result};
use std::path::Path;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportFormat {
    Txt,
    Html,
    Json,
    Md,
}

impl ReportFormat {
    pub fn from_extension(filename: &str) -> Self {
        let path = Path::new(filename);
        match path.extension().and_then(|e| e.to_str()) {
            Some("html" | "htm") => ReportFormat::Html,
            Some("json") => ReportFormat::Json,
            Some("md" | "markdown") => ReportFormat::Md,
            _ => ReportFormat::Txt,
        }
    }
    
    pub fn extension(&self) -> &str {
        match self {
            ReportFormat::Txt => "txt",
            ReportFormat::Html => "html",
            ReportFormat::Json => "json",
            ReportFormat::Md => "md",
        }
    }
}

pub fn generate_report(dir_path: &Path, format: ReportFormat) -> Result<String> {
    let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy();
    let output_filename = format!("{}.{}", dir_name, format.extension());
    let output_path = crate::output::build_output_path(&output_filename);
    
    generate_report_to(dir_path, format, &output_path.to_string_lossy())
}

pub fn generate_report_to(dir_path: &Path, format: ReportFormat, output_file: &str) -> Result<String> {
    let output_path = crate::output::build_output_path(output_file);
    
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
    }
    
    println!("Report saved to: {}", output_path.display());
    Ok(output_path.to_string_lossy().to_string())
}

// Add this function to generate report as string (for clipboard)
pub fn generate_report_to_string(dir_path: &Path, format: ReportFormat) -> Result<String> {
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

