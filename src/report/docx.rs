use crate::config::Config;
use crate::explorer::{generate_tree, format_tree};
use crate::filetype::FormatConfig;
use crate::output::cat_file_with_line_numbers;
use anyhow::Result;
use docx_rs::*;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

const MONO_FONT: &str = "Courier New";

fn collect_files(dir_path: &Path, max_depth: usize) -> (Vec<std::path::PathBuf>, Vec<std::path::PathBuf>) {
    let mut supported = Vec::new();
    let mut unsupported = Vec::new();
    let ignored_dirs = Config::global_get_ignored_dirs();
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

fn mono_run(text: &str, size: usize) -> Run {
    Run::new()
        .add_text(text)
        .fonts(RunFonts::new().ascii(MONO_FONT))
        .size(size)
}

fn bold_run(text: &str, size: usize) -> Run {
    Run::new().add_text(text).size(size).bold()
}

fn normal_run(text: &str, size: usize) -> Run {
    Run::new().add_text(text).size(size)
}

pub fn generate_docx_report(dir_path: &Path, output_path: &Path) -> Result<()> {
    let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let show_lines = Config::global_get_show_line_numbers();
    let max_depth = Config::global_get_max_depth();
    let tree = generate_tree(dir_path.to_string_lossy().as_ref(), None, true, None);

    let mut doc = Docx::new();

    // Title
    doc = doc.add_paragraph(
        Paragraph::new()
            .add_run(bold_run(&format!("{} - Directory Report", dir_name), 52))
            .page_break_before(false),
    );

    // Date
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    doc = doc.add_paragraph(Paragraph::new().add_run(normal_run(&date, 24)));

    // Separator
    doc = doc.add_paragraph(Paragraph::new().add_run(mono_run(&"=".repeat(100), 20)));

    // Tree heading
    doc = doc.add_paragraph(Paragraph::new().add_run(bold_run("Directory Tree", 34)));

    // Tree content
    let tree_str = format_tree(&tree, "", true);
    for line in tree_str.lines() {
        doc = doc.add_paragraph(Paragraph::new().add_run(mono_run(line, 20)));
    }

    // Separator
    doc = doc.add_paragraph(Paragraph::new().add_run(mono_run(&"=".repeat(100), 20)));
    doc = doc.add_paragraph(Paragraph::new().add_run(bold_run(&dir_name, 34)));
    doc = doc.add_paragraph(Paragraph::new().add_run(mono_run(&"=".repeat(100), 20)));

    // File contents
    let (supported_files, unsupported_files) = collect_files(dir_path, max_depth);

    for file_path in &supported_files {
        let name = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        doc = doc.add_paragraph(Paragraph::new().add_run(mono_run(&format!("── {} ──", name), 20).bold()));

        match cat_file_with_line_numbers(file_path, show_lines) {
            Ok(content) => {
                for line in content.lines() {
                    doc = doc.add_paragraph(Paragraph::new().add_run(mono_run(line, 20)));
                }
            }
            Err(_) => {
                doc = doc.add_paragraph(Paragraph::new().add_run(mono_run("[Error reading file]", 20)));
            }
        }
    }

    if !unsupported_files.is_empty() {
        doc = doc.add_paragraph(Paragraph::new().add_run(mono_run("── Unsupported Files (skipped) ──", 20).bold()));
        for file_path in &unsupported_files {
            let name = file_path.file_name().unwrap_or_default().to_string_lossy();
            doc = doc.add_paragraph(Paragraph::new().add_run(mono_run(&format!("Skipped: {}", name), 20)));
        }
    }

    let file = File::create(output_path)?;
    doc.build().pack(file)?;
    Ok(())
}
