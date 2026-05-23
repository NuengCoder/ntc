// src/report/txt.rs
use crate::config::Config;
use crate::explorer::{generate_tree, format_tree, TreeNode};
use crate::filetype::{FormatConfig};
use crate::output::{cat_file_with_line_numbers, format_separator, write_file};
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

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

fn collect_files(dir_path: &Path, max_depth: usize) -> (Vec<std::path::PathBuf>, Vec<std::path::PathBuf>) {
    let mut supported = Vec::new();
    let mut unsupported = Vec::new();
    let ignored_dirs = crate::config::Config::global_get_ignored_dirs();
    let fmt_cfg = FormatConfig::from_global();

    let walker = WalkDir::new(dir_path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) {
                    return false;
                }
            }
            true
        });

    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if crate::filetype::is_supported_format_with_config(&path, &fmt_cfg) {
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