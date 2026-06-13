// src/report/txt.rs
use crate::config::Config;
use crate::explorer::{generate_tree, format_tree, TreeNode};
use crate::output::{cat_file_with_line_numbers, format_separator, write_file};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn generate_txt_report(dir_path: &Path, output_path: &Path) -> Result<()> {
    let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy();
    let show_lines = Config::global_get_show_line_numbers();
    let max_depth = Config::global_get_max_depth();
    let tree = generate_tree(dir_path.to_string_lossy().as_ref(), None, true, None);
    
    let content = build_txt_content(&dir_name, &tree, dir_path, show_lines, max_depth)?;
    write_file(output_path, &content)?;
    
    Ok(())
}

fn build_txt_content(
    dir_name: &str,
    tree: &TreeNode,
    dir_path: &Path,
    show_lines: bool,
    max_depth: usize,
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
    let dir_padding = (77usize.saturating_sub(dir_name.len())) / 2;
    content.push_str(&" ".repeat(dir_padding));
    content.push_str(dir_name);
    content.push('\n');
    content.push_str(&"=".repeat(77));
    content.push_str("\n\n");

    // Collect files
    let (supported_files, unsupported_files) = collect_files(dir_path, max_depth);

    // Supported files section
    for file_path in &supported_files {
        content.push_str(&format_separator(
            &file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy(),
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

fn collect_files(dir_path: &Path, max_depth: usize) -> (Vec<PathBuf>, Vec<PathBuf>) {
    super::collect_report_files(dir_path, max_depth)
}