//! Shell commands for directory tree display (`view`) and report generation
//! (`report` with subcommands: txt, json, md, html, pdf, docx, xlsx).
//! Also provides `txtc` (copy file) and `txtf` (display file).

use crate::config::Config;
use crate::filetype::is_supported_format;
use crate::navigator::Navigator;
use crate::output::{cat_file_with_syntax, print_error, print_success, print_warning};
use crate::report::{generate_report, ReportFormat};
use crate::shell::helpers::show_tree;
use super::{list_supported_files, show_file_selection_menu};

use colored::*;

use anyhow::Result;
use std::path::Path;

pub fn cmd_view(args: &str, nav: &mut Navigator) -> Result<bool> {
    let mut show_sizes = false;
    let mut depth_override: Option<usize> = None;
    let mut copy_to_clipboard = false;
    let mut care_sizes = false;
    let mut show_time = false;

    let parts: Vec<&str> = args.split_whitespace().collect();
    let mut i = 0;
    while i < parts.len() {
        match parts[i] {
            "-s" | "--size" => show_sizes = true,
            "-c" | "--cp" => copy_to_clipboard = true,
            "--care" => care_sizes = true,
            "--time" | "-t" => show_time = true,
            "-d" | "--depth" => {
                if i + 1 < parts.len() {
                    if let Ok(depth) = parts[i + 1].parse::<usize>() {
                        depth_override = Some(depth);
                        i += 1;
                    } else {
                        print_error(&format!("Invalid depth: {}", parts[i + 1]));
                    }
                }
            }
            _ => {
                print_error(&format!("Unknown view option: {}", parts[i]));
                println!("Usage: view [-s|--size] [-c|--cp] [--care] [-t|--time] [-d|--depth <n>]");
                return Ok(false);
            }
        }
        i += 1;
    }

    let max_depth = depth_override.unwrap_or(Config::global_get_max_depth());
    let start = std::time::Instant::now();
    show_tree(nav, Some(max_depth), true, true, show_sizes, copy_to_clipboard, care_sizes);
    if show_time {
        let elapsed = crate::explorer::format_duration(start.elapsed());
        let w = elapsed.len() + 16;
        println!("┌{}┐", "─".repeat(w));
        println!("│ Time:  {:<w$} │", elapsed.cyan(), w = w - 9);
        println!("└{}┘", "─".repeat(w));
    }
    Ok(false)
}

pub fn cmd_report(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let format = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match format {
        "txt" => generate_report_file(Path::new(subargs), ReportFormat::Txt)?,
        "json" => generate_report_dir(Path::new(subargs), ReportFormat::Json, "JSON")?,
        "md" => generate_report_dir(Path::new(subargs), ReportFormat::Md, "Markdown")?,
        "html" => generate_report_file(Path::new(subargs), ReportFormat::Html)?,
        "pdf" => generate_report_dir_only(Path::new(subargs), ReportFormat::Pdf, "PDF")?,
        "docx" => generate_report_dir_only(Path::new(subargs), ReportFormat::Docx, "DOCX")?,
        "xlsx" => generate_report_dir_only(Path::new(subargs), ReportFormat::Xlsx, "XLSX")?,
        _ => {
            println!("{}", "report — Generate project reports".cyan().bold());
            println!("  report txt [path]      Generate TXT report");
            println!("  report json [path]     Generate JSON report");
            println!("  report md [path]       Generate Markdown report");
            println!("  report html [path]     Generate HTML report");
            println!("  report pdf [path]      Generate PDF report");
            println!("  report docx [path]     Generate DOCX report");
            println!("  report xlsx [path]     Generate XLSX report");
            println!();
            println!("{}", "Examples:".green());
            println!("  report txt             Report on current directory");
            println!("  report json ..         Report on parent directory");
        }
    }
    Ok(false)
}

fn generate_report_file(target: &Path, format: ReportFormat) -> Result<()> {
    let target = if target.as_os_str().is_empty() { Path::new(".") } else { target };
    if target.is_dir() {
        generate_report(target, format)?;
    } else if target.is_file() {
        if is_supported_format(target) {
            let show_lines = Config::global_get_show_line_numbers();
            cat_file_with_syntax(target, show_lines)?;
        } else {
            print_warning(&format!("Skipped (not support format): {}", target.display()));
        }
    }
    Ok(())
}

fn generate_report_dir(target: &Path, format: ReportFormat, label: &str) -> Result<()> {
    let target = if target.as_os_str().is_empty() { Path::new(".") } else { target };
    if target.is_dir() {
        generate_report(target, format)?;
    } else {
        print_error(&format!("{} report only works on directories", label));
    }
    Ok(())
}

fn generate_report_dir_only(target: &Path, format: ReportFormat, label: &str) -> Result<()> {
    let target = if target.as_os_str().is_empty() { Path::new(".") } else { target };
    if target.is_dir() {
        generate_report(target, format)?;
    } else {
        print_error(&format!("{} report only works on directories", label));
    }
    Ok(())
}

pub fn cmd_txtc(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let files = list_supported_files(nav)?;
        show_file_selection_menu(nav, files, true)?;
    } else {
        let target = Path::new(args);
        if target.is_file() {
            if is_supported_format(target) {
                let show_lines = Config::global_get_show_line_numbers();
                let content = crate::output::cat_file_with_line_numbers(target, show_lines)?;

                #[cfg(target_os = "android")]
                {
                    use crate::output::print_info;
                    print_info(&format!("Copying '{}' to clipboard...", target.display()));
                    match crate::output::copy_to_clipboard(&content, "TXT") {
                        Ok(()) => {}
                        Err(e) => {
                            print_error(&format!("Failed to copy: {}", e));
                            let output_file = crate::output::build_output_path(&format!("copied_{}.txt",
                                target.file_name().unwrap_or_default().to_string_lossy()));
                            if let Ok(()) = crate::output::write_file(&output_file, &content) {
                                print_success(&format!("File content saved to: {}", output_file.display()));
                                print_info(&format!("You can open this in Neovim with: :edit {}", output_file.display()));
                            }
                        }
                    }
                }

                #[cfg(not(target_os = "android"))]
                {
                    crate::output::copy_to_clipboard(&content, "TXT")?;
                    print_success(&format!("File '{}' copied to clipboard!", target.display()));
                }
            } else {
                print_warning(&format!("Skipped (not support format): {}", args));
            }
        } else {
            print_error(&format!("File not found: {}", args));
        }
    }
    Ok(false)
}

pub fn cmd_txtf(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let files = list_supported_files(nav)?;
        show_file_selection_menu(nav, files, false)?;
    } else {
        let target = Path::new(args);
        if target.is_file() {
            if is_supported_format(target) {
                let show_lines = Config::global_get_show_line_numbers();
                cat_file_with_syntax(target, show_lines)?;
            } else {
                print_warning(&format!("Skipped (not support format): {}", args));
            }
        } else {
            print_error(&format!("File not found: {}", args));
        }
    }
    Ok(false)
}
