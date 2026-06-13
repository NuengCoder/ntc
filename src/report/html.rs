use crate::config::Config;
use crate::explorer::{generate_tree, format_tree, TreeNode};
use crate::output::cat_file_with_line_numbers;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct HtmlReportGenerator;

impl HtmlReportGenerator {
    pub fn generate(dir_path: &Path, output_path: &Path, depth: Option<usize>) -> Result<()> {
        let start = Instant::now();
        let dir_name = dir_path.file_name().map(|n| n.to_string_lossy()).unwrap_or_else(|| std::borrow::Cow::Borrowed("root"));
        eprintln!("📊 Generating HTML report for: {}", dir_name);

        // Generate tree (same as before)
        let tree = generate_tree(dir_path.to_string_lossy().as_ref(), depth, true, None);
        let tree_str = format_tree(&tree, "", true);

        // Collect files
        let (supported, unsupported) = collect_files(dir_path);
        let mut files_html = String::new();
        let show_lines = Config::global_get_show_line_numbers();

        for path in &supported {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            let content = cat_file_with_line_numbers(path, show_lines)
                .unwrap_or_else(|e| format!("Error reading file: {}", e));
            let content_html = content
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            let escaped_name = name
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;");
            files_html.push_str(&format!(
                r#"<div class="file"><div class="file-header">📄 {}</div><div class="file-content"><pre>{}</pre></div></div>"#,
                escaped_name, content_html
            ));
        }

        if !unsupported.is_empty() {
            files_html.push_str(r#"<div class="skipped"><strong>⚠️ Skipped files (unsupported format):</strong><ul>"#);
            for path in &unsupported {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                files_html.push_str(&format!("<li>{}</li>", name));
            }
            files_html.push_str("</ul></div>");
        }

        let total_files = supported.len() + unsupported.len();
        let total_dirs = count_dirs(&tree);
        let total_size = crate::explorer::calculate_total_size(dir_path);
        let total_size_str = crate::explorer::human_readable_size(total_size);

        let template = include_str!("html_template/template.html");
        let html_output = template
            .replace("{{title}}", &dir_name)
            .replace("{{version}}", env!("CARGO_PKG_VERSION"))
            .replace("{{date}}", &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
            .replace("{{total_files}}", &total_files.to_string())
            .replace("{{total_dirs}}", &total_dirs.to_string())
            .replace("{{total_size}}", &total_size_str)
            .replace("{{scan_time}}", &format!("{:.2}", start.elapsed().as_secs_f64()))
            .replace("{{tree}}", &tree_str)
            .replace("{{files}}", &files_html);

        std::fs::write(output_path, html_output)?;
        eprintln!("✅ HTML report saved to: {}", output_path.display());
        Ok(())
    }
}

fn collect_files(dir_path: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let max_depth = Config::global_get_max_depth();
    super::collect_report_files(dir_path, max_depth)
}

fn count_dirs(node: &TreeNode) -> u64 {
    let mut count = if node.is_dir && node.depth > 0 { 1 } else { 0 };
    for child in &node.children {
        count += count_dirs(child);
    }
    count
}

