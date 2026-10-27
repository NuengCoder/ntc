use crate::config::Config;
use crate::explorer::{generate_tree, TreeNode};
use crate::output::cat_file_with_line_numbers;
use crate::t;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct HtmlReportGenerator;

impl HtmlReportGenerator {
    pub fn generate(dir_path: &Path, output_path: &Path, depth: Option<usize>) -> Result<()> {
        let start = Instant::now();
        let dir_name = dir_path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| std::borrow::Cow::Borrowed("root"));
        eprintln!("{}", t!("report.generating_html", dir_name));

        // Generate tree. The HTML report renders it as a collapsible widget
        // (first level open, everything below folded away) instead of an
        // ASCII `<pre>` block.
        let tree = generate_tree(dir_path.to_string_lossy().as_ref(), depth, true, None);
        let tree_str = tree_widget(&tree, &dir_name);

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
            .replace(
                "{{date}}",
                &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            )
            .replace("{{total_files}}", &total_files.to_string())
            .replace("{{total_dirs}}", &total_dirs.to_string())
            .replace("{{total_size}}", &total_size_str)
            .replace(
                "{{scan_time}}",
                &format!("{:.2}", start.elapsed().as_secs_f64()),
            )
            .replace("{{tree}}", &tree_str)
            .replace("{{files}}", &files_html);

        std::fs::write(output_path, html_output)?;
        eprintln!("{}", t!("report.saved_html", output_path.display()));
        Ok(())
    }
}

/// Minimal HTML escaping for names embedded in the tree widget.
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Render the tree as a `<div>` widget: the root header plus one nested
/// list per directory, with each directory a `<details>` that starts
/// collapsed so only the first depth is visible until the user expands it.
fn tree_widget(node: &TreeNode, root_name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "<div class=\"ntree-root\">\u{1F4C1} {}</div>",
        escape_html(root_name)
    ));
    out.push_str(&tree_list(node));
    out
}

fn tree_list(node: &TreeNode) -> String {
    if node.children.is_empty() {
        return String::new();
    }
    let mut out = String::from("<ul class=\"ntree\">");
    for child in &node.children {
        if child.is_dir {
            out.push_str("<li class=\"ntree-dir\"><details><summary>");
            out.push_str(&format!("\u{1F4C1} {}", escape_html(&child.name)));
            out.push_str("</summary>");
            out.push_str(&tree_list(child));
            out.push_str("</details></li>");
        } else {
            let icon = if child.is_supported == Some(false) {
                "\u{1F5CE}"
            } else {
                "\u{1F4C4}"
            };
            out.push_str(&format!(
                "<li class=\"ntree-file\">{} {}</li>",
                icon,
                escape_html(&child.name)
            ));
        }
    }
    out.push_str("</ul>");
    out
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

#[cfg(test)]
mod tests {
    use super::*;

    fn node(name: &str, is_dir: bool, depth: usize, children: Vec<TreeNode>) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            path: name.to_string(),
            is_dir,
            is_ignored: false,
            is_supported: if is_dir { None } else { Some(true) },
            children,
            depth,
            size: None,
        }
    }

    #[test]
    fn tree_widget_folds_everything_below_the_first_depth() {
        let tree = node(
            "proj",
            true,
            0,
            vec![
                node("src", true, 1, vec![node("main.rs", false, 2, Vec::new())]),
                node("README.md", false, 1, Vec::new()),
            ],
        );
        let html = tree_widget(&tree, "proj");
        // Root header + one collapsed <details> per directory.
        assert!(html.contains("ntree-root"));
        assert!(html.contains("proj"));
        assert!(html.contains("<details><summary>"));
        assert!(!html.contains("<details open>"));
        // Deeper entries are present but folded away.
        assert!(html.contains("main.rs"));
        assert!(html.contains("ntree-file"));
        assert_eq!(html.matches("<ul class=\"ntree\">").count(), 2);
    }

    #[test]
    fn tree_widget_escapes_names_and_marks_unsupported_files() {
        let mut bin = node("a<b>&c.bin", false, 1, Vec::new());
        bin.is_supported = Some(false);
        let tree = node("root", true, 0, vec![bin]);
        let html = tree_widget(&tree, "root & co");
        assert!(html.contains("a&lt;b&gt;&amp;c.bin"));
        assert!(html.contains("root &amp; co"));
        assert!(html.contains("\u{1F5CE}"));
        assert!(!html.contains("a<b>"));
    }
}
