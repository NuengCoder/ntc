use crate::config::Config;
use crate::explorer::{generate_tree, format_tree, TreeNode};
use crate::filetype::{is_supported_format_with_config, FormatConfig};
use crate::output::cat_file_with_line_numbers;
use anyhow::Result;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

pub struct HtmlReportGenerator;

impl HtmlReportGenerator {
    pub fn generate(dir_path: &Path, output_path: &Path, depth: Option<usize>) -> Result<()> {
        let start = Instant::now();
        let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy();
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
                .replace("&", "&amp;")
                .replace("<", "&lt;")
                .replace(">", "&gt;");
            files_html.push_str(&format!(
                r#"<div class="file"><div class="file-header">📄 {}</div><div class="file-content"><pre>{}</pre></div></div>"#,
                name, content_html
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
        let total_size = calculate_total_size(&tree);
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

fn collect_files(dir_path: &Path) -> (Vec<std::path::PathBuf>, Vec<std::path::PathBuf>) {
    let mut supported = Vec::new();
    let mut unsupported = Vec::new();
    let ignored_dirs = Config::global_get_ignored_dirs();
    let max_depth = Config::global_get_max_depth();
    let fmt_cfg = FormatConfig::from_global();

    let walker = WalkDir::new(dir_path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 { return true; }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) { return false; }
            }
            true
        });

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

fn count_dirs(node: &TreeNode) -> u64 {
    let mut count = if node.is_dir && node.depth > 0 { 1 } else { 0 };
    for child in &node.children {
        count += count_dirs(child);
    }
    count
}

fn calculate_total_size(node: &TreeNode) -> u64 {
    let mut total = node.size.unwrap_or(0);
    for child in &node.children {
        total += calculate_total_size(child);
    }
    total
}