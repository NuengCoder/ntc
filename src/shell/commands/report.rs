use crate::config::Config;
use crate::filetype::is_supported_format;
use crate::navigator::Navigator;
use crate::output::{cat_file_with_syntax, print_error, print_success, print_warning};
use crate::report::{generate_report, ReportFormat};
use crate::shell::helpers::show_tree;
use super::{list_supported_files, show_file_selection_menu};

use anyhow::Result;
use std::path::Path;

pub fn cmd_view(args: &str, nav: &mut Navigator) -> Result<bool> {
    let mut show_sizes = false;
    let mut depth_override: Option<usize> = None;
    let mut copy_to_clipboard = false;
    let mut care_sizes = false;

    let parts: Vec<&str> = args.split_whitespace().collect();
    let mut i = 0;
    while i < parts.len() {
        match parts[i] {
            "-s" | "--size" => show_sizes = true,
            "-c" | "--cp" => copy_to_clipboard = true,
            "--care" => care_sizes = true,
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
                println!("Usage: view [-s|--size] [-c|--cp] [--care] [-d|--depth <n>]");
                return Ok(false);
            }
        }
        i += 1;
    }

    let max_depth = depth_override.unwrap_or(Config::global_get_max_depth());
    show_tree(nav, Some(max_depth), true, true, show_sizes, copy_to_clipboard, care_sizes);
    Ok(false)
}

enum ReportKind {
    Text(ReportFormat, &'static str),
    DirOnly(ReportFormat, &'static str),
    DirOrFile(ReportFormat),
}

fn report_kind_for(cmd: &str) -> ReportKind {
    match cmd {
        "txt" => ReportKind::DirOrFile(ReportFormat::Txt),
        "txtc" | "txtf" => unreachable!(),
        "json" => ReportKind::Text(ReportFormat::Json, "JSON"),
        "md" => ReportKind::Text(ReportFormat::Md, "Markdown"),
        "html" => ReportKind::DirOrFile(ReportFormat::Html),
        "pdf" => ReportKind::DirOnly(ReportFormat::Pdf, "PDF"),
        "docx" => ReportKind::DirOnly(ReportFormat::Docx, "DOCX"),
        "xlsx" => ReportKind::DirOnly(ReportFormat::Xlsx, "XLSX"),
        _ => unreachable!(),
    }
}

pub fn cmd_report(cmd: &str, args: &str, nav: &mut Navigator) -> Result<bool> {
    let kind = report_kind_for(cmd);
    let parts: Vec<&str> = args.split_whitespace().collect();
    let copy_to_clipboard = parts.contains(&"--cp");
    let target_arg = if copy_to_clipboard {
        parts.iter().find(|&&p| p != "--cp").unwrap_or(&"").trim()
    } else {
        args
    };

    let target = if target_arg.is_empty() {
        nav.current_path()
    } else {
        Path::new(target_arg)
    };

    match kind {
        ReportKind::Text(format, label) => {
            if target.is_dir() {
                if copy_to_clipboard {
                    let content = crate::report::generate_report_to_string(target, format)?;
                    crate::output::copy_to_clipboard(&content, label)?;
                    print_success(&format!("{} report copied to clipboard!", label));
                } else {
                    generate_report(target, format)?;
                }
            } else {
                print_error(&format!("{} report only works on directories", label));
            }
        }
        ReportKind::DirOnly(format, label) => {
            if target.is_dir() {
                generate_report(target, format)?;
            } else {
                print_error(&format!("{} report only works on directories", label));
            }
        }
        ReportKind::DirOrFile(format) => {
            if target.is_dir() {
                if copy_to_clipboard {
                    let content = crate::report::generate_report_to_string(target, format)?;
                    crate::output::copy_to_clipboard(&content, "TXT")?;
                    print_success("Report copied to clipboard!");
                } else {
                    generate_report(target, format)?;
                }
            } else if target.is_file() {
                if is_supported_format(target) {
                    let show_lines = Config::global_get_show_line_numbers();
                    cat_file_with_syntax(target, show_lines)?;
                } else {
                    print_warning(&format!("Skipped (not support format): {}", target_arg));
                }
            }
        }
    }
    Ok(false)
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

pub fn cmd_txt(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_report("txt", args, nav)
}

pub fn cmd_html(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_report("html", args, nav)
}

pub fn cmd_json(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_report("json", args, nav)
}

pub fn cmd_md(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_report("md", args, nav)
}

pub fn cmd_pdf(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_report("pdf", args, nav)
}

pub fn cmd_docx(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_report("docx", args, nav)
}

pub fn cmd_xlsx(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_report("xlsx", args, nav)
}
