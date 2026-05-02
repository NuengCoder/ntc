use crate::config::Config;
use crate::explorer::{generate_tree, format_tree, TreeNode};
use crate::filetype::is_supported_format;
use crate::output::{build_output_path, cat_file_with_line_numbers, format_separator, write_file};
use anyhow::Result;
use std::path::Path;

/// Supported report formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportFormat {
    Txt,
    Html,
}

impl ReportFormat {
    /// Get file extension for the format
    pub fn extension(&self) -> &str {
        match self {
            ReportFormat::Txt => "txt",
            ReportFormat::Html => "html",
        }
    }
}

/// Generate a report for a directory and save to the output path
pub fn generate_report(dir_path: &Path, format: ReportFormat) -> Result<String> {
    let dir_name = dir_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let output_filename = format!("{}.{}", dir_name, format.extension());
    let output_path = build_output_path(&output_filename);

    let content = build_report_content(dir_path, format)?;
    write_file(&output_path, &content)?;

    println!("Report saved to: {}", output_path.display());
    Ok(output_path.to_string_lossy().to_string())
}

/// Generate a report for a directory with custom output filename
pub fn generate_report_to(dir_path: &Path, format: ReportFormat, output_file: &str) -> Result<String> {
    let output_path = build_output_path(output_file);
    let content = build_report_content(dir_path, format)?;
    write_file(&output_path, &content)?;

    println!("Report saved to: {}", output_path.display());
    Ok(output_path.to_string_lossy().to_string())
}

/// Build the full report content as a string
fn build_report_content(dir_path: &Path, format: ReportFormat) -> Result<String> {
    let dir_name = dir_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let show_lines = Config::global_get_show_line_numbers();
    let tree = generate_tree(dir_path.to_string_lossy().as_ref(), None,true);

    match format {
        ReportFormat::Txt => build_txt_report(&dir_name, &tree, dir_path, show_lines),
        ReportFormat::Html => build_html_report(&dir_name, &tree, dir_path, show_lines),
    }
}

/// Build TXT report content
fn build_txt_report(
    dir_name: &str,
    tree: &TreeNode,
    dir_path: &Path,
    show_lines: bool,
) -> Result<String> {
    let mut content = String::new();

    // Header
    let header = format!("{} directory", dir_name);
    content.push_str(&"=".repeat(77));
    content.push('\n');
    let padding = (77usize.saturating_sub(header.len())) / 2;
    content.push_str(&" ".repeat(padding));
    content.push_str(&header);
    content.push('\n');
    content.push_str(&"=".repeat(77));
    content.push_str("\n\n");

    // Directory tree
    let tree_str = format_tree(tree, "", true);
    content.push_str(&tree_str);
    content.push('\n');

    // Divider
    content.push_str(&"=".repeat(77));
    content.push('\n');
    let dir_header = format!("{}", dir_name);
    let dir_padding = (77usize.saturating_sub(dir_header.len())) / 2;
    content.push_str(&" ".repeat(dir_padding));
    content.push_str(&dir_header);
    content.push('\n');
    content.push_str(&"=".repeat(77));
    content.push_str("\n\n");

    // Collect supported and unsupported files
    let (supported_files, unsupported_files) = collect_files(dir_path);

    // Supported files section
    for file_path in &supported_files {
        content.push_str(&format_separator(
            &file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        ));
        content.push('\n');

        match cat_file_with_line_numbers(file_path, show_lines) {
            Ok(file_content) => {
                content.push_str(&file_content);
                content.push('\n');
            }
            Err(e) => {
                content.push_str(&format!("[Error reading file: {}]\n\n", e));
            }
        }
    }

    // Unsupported files section
    if !unsupported_files.is_empty() {
        content.push_str(&format_separator("Unsupported Files (skipped)"));
        content.push('\n');
        for file_path in &unsupported_files {
            content.push_str(&format!(
                "Skipped (not support format): {}\n",
                file_path.file_name().unwrap_or_default().to_string_lossy()
            ));
        }
        content.push('\n');
    }

    Ok(content)
}

/// Build HTML report content
fn build_html_report(
    dir_name: &str,
    tree: &TreeNode,
    dir_path: &Path,
    show_lines: bool,
) -> Result<String> {
    let mut content = String::new();

    // HTML header
    content.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    content.push_str("<meta charset=\"UTF-8\">\n");
    content.push_str(&format!("<title>{} - Directory Report</title>\n", dir_name));
    content.push_str(
        "<style>\n\
         body { font-family: 'Consolas', 'Courier New', monospace; background: #1e1e1e; color: #d4d4d4; padding: 20px; }\n\
         h1, h2 { color: #569cd6; }\n\
         .tree { white-space: pre; background: #252526; padding: 15px; border-radius: 5px; }\n\
         .file-content { background: #252526; padding: 15px; margin: 10px 0; border-radius: 5px; white-space: pre-wrap; }\n\
         .file-header { background: #0e639c; color: white; padding: 8px 15px; border-radius: 3px; margin-top: 20px; font-weight: bold; }\n\
         .skipped { color: #ce9178; padding: 15px; }\n\
         .line-num { color: #858585; user-select: none; }\n\
         hr { border: 1px solid #3e3e3e; }\n\
         </style>\n");
    content.push_str("</head>\n<body>\n");

    // Title
    content.push_str(&format!("<h1>{} directory</h1>\n", dir_name));

    // Tree
    content.push_str("<h2>Directory Tree</h2>\n");
    content.push_str("<div class=\"tree\">\n");
    let tree_str = format_tree(tree, "", true);
    content.push_str(&html_escape(&tree_str));
    content.push_str("</div>\n\n");

    content.push_str("<hr>\n\n");

    // Collect supported and unsupported files
    let (supported_files, unsupported_files) = collect_files(dir_path);

    // Supported files
    content.push_str("<h2>Supported Files</h2>\n");
    for file_path in &supported_files {
        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        content.push_str(&format!("<div class=\"file-header\">{}</div>\n", file_name));
        content.push_str("<div class=\"file-content\">\n");

        match cat_file_with_line_numbers(file_path, show_lines) {
            Ok(file_content) => {
                content.push_str(&html_escape(&file_content));
            }
            Err(e) => {
                content.push_str(&format!("[Error reading file: {}]\n", e));
            }
        }
        content.push_str("</div>\n\n");
    }

    // Unsupported files
    if !unsupported_files.is_empty() {
        content.push_str("<h2>Unsupported Files (skipped)</h2>\n");
        content.push_str("<div class=\"skipped\">\n<ul>\n");
        for file_path in &unsupported_files {
            content.push_str(&format!(
                "<li>{}</li>\n",
                file_path.file_name().unwrap_or_default().to_string_lossy()
            ));
        }
        content.push_str("</ul>\n</div>\n");
    }

    content.push_str("</body>\n</html>\n");
    Ok(content)
}

/// Collect all supported and unsupported text files recursively

fn collect_files(dir_path: &Path) -> (Vec<std::path::PathBuf>, Vec<std::path::PathBuf>) {
    let mut supported = Vec::new();
    let mut unsupported = Vec::new();
    let ignored_dirs = Config::global_get_ignored_dirs();

    if let Ok(entries) = std::fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();
                // Skip ignored directories
                if ignored_dirs.contains(&dir_name) {
                    continue;
                }
                let (sub_supported, sub_unsupported) = collect_files(&path);
                supported.extend(sub_supported);
                unsupported.extend(sub_unsupported);
            } else if path.is_file() {
                if is_supported_format(&path) {
                    supported.push(path);
                } else {
                    unsupported.push(path);
                }
            }
        }
    }

    supported.sort();
    unsupported.sort();
    (supported, unsupported)
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_dir() -> TempDir {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join("subdir")).unwrap();
        fs::write(temp.path().join("file1.txt"), "hello world\nline 2\n").unwrap();
        fs::write(temp.path().join("file2.rs"), "fn main() {\n    println!(\"hi\");\n}\n").unwrap();
        fs::write(temp.path().join("subdir").join("nested.txt"), "nested content\n").unwrap();
        fs::write(temp.path().join("binary.bin"), b"\x00\x01\x02\x03").unwrap();
        temp
    }

    #[test]
    fn test_collect_files() {
        let temp = create_test_dir();
        let (supported, unsupported) = collect_files(temp.path());

        // file1.txt, file2.rs, nested.txt should be supported
        assert_eq!(supported.len(), 3, "Expected 3 supported text files");

        // binary.bin should be unsupported
        assert_eq!(unsupported.len(), 1, "Expected 1 unsupported binary file");
    }

    #[test]
    fn test_build_txt_report() -> Result<()> {
        let temp = create_test_dir();
        let tree = generate_tree(temp.path().to_string_lossy().as_ref(), None,true);
        let report = build_txt_report(
            &temp.path().file_name().unwrap().to_string_lossy().to_string(),
            &tree,
            temp.path(),
            false,
        )?;

        assert!(report.contains("file1.txt"));
        assert!(report.contains("hello world"));
        assert!(report.contains("file2.rs"));
        assert!(report.contains("Skipped"));
        Ok(())
    }

    #[test]
    fn test_build_html_report() -> Result<()> {
        let temp = create_test_dir();
        let tree = generate_tree(temp.path().to_string_lossy().as_ref(), None,true);
        let report = build_html_report(
            &temp.path().file_name().unwrap().to_string_lossy().to_string(),
            &tree,
            temp.path(),
            false,
        )?;

        assert!(report.contains("<!DOCTYPE html>"));
        assert!(report.contains("<html"));
        assert!(report.contains("file1.txt"));
        assert!(report.contains("hello world"));
        assert!(report.contains("</html>"));
        Ok(())
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>alert('xss')</script>"), 
                   "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("foo\"bar"), "foo&quot;bar");
    }

    #[test]
    fn test_report_format_extension() {
        assert_eq!(ReportFormat::Txt.extension(), "txt");
        assert_eq!(ReportFormat::Html.extension(), "html");
    }

    #[test]
    fn test_generate_report_creates_file() -> Result<()> {
        let temp = create_test_dir();
        let test_dir_name = temp.path().file_name().unwrap().to_string_lossy().to_string();
        
        // Temporarily set output to temp parent so file is created there
        let original = Config::global_get_output_path();
        Config::global_set_output_path(temp.path().parent().unwrap());

        let _output_path = generate_report(temp.path(), ReportFormat::Txt)?;
        
        let expected_path = temp.path().parent().unwrap().join(format!("{}.txt", test_dir_name));
        assert!(expected_path.exists(), "Report file should exist at {}", expected_path.display());
        
        // Clean up
        let _ = fs::remove_file(&expected_path);
        Config::global_set_output_path(&original);
        Ok(())
    }
}