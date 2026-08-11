//! Shell commands for managing ignore/care filtering:
//! `ignore`, `care`, `igcared`, `igcare` (export/import).

use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::helpers::{igcare_export_all, igcare_export_select, igcare_import};
use crate::t;

use anyhow::Result;
use colored::*;

fn apply_igcare(
    args: &str,
    usage: &str,
    action: fn(&str) -> std::result::Result<(), Box<dyn std::error::Error>>,
    action_desc: &str,
) -> Result<bool> {
    if args.is_empty() {
        println!("{}", usage);
    } else {
        let mut count = 0;
        for item in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if action(item).is_ok() {
                count += 1;
            }
        }
        Config::reload_global();
        if count > 0 {
            print_success(&t!("cmd.ignore.applied", action_desc, count));
            print_info(&t!("cmd.ignore.check_hint"));
        }
    }
    Ok(false)
}

pub fn cmd_ignore(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "dir" => apply_igcare(
            subargs,
            &t!("cmd.ignore.dir_usage"),
            Config::local_add_ignored_dir,
            "Ignored directories:",
        ),
        "ext" => apply_igcare(
            subargs,
            &t!("cmd.ignore.ext_usage"),
            Config::local_add_ignored_extension,
            "Ignored extensions:",
        ),
        "file" => apply_igcare(
            subargs,
            &t!("cmd.ignore.file_usage"),
            Config::local_add_ignored_file,
            "Ignored files:",
        ),
        _ => {
            println!("{}", t!("cmd.ignore.title").cyan().bold());
            println!("  {}", t!("cmd.ignore.dir_desc"));
            println!("  {}", t!("cmd.ignore.ext_desc"));
            println!("  {}", t!("cmd.ignore.file_desc"));
            println!();
            println!("{}", t!("cmd.ignore.examples_title").green());
            println!("  {}", t!("cmd.ignore.example_dir"));
            println!("  {}", t!("cmd.ignore.example_ext"));
            println!("  {}", t!("cmd.ignore.example_file"));
            Ok(false)
        }
    }
}

pub fn cmd_care(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "dir" => apply_igcare(
            subargs,
            &t!("cmd.care.dir_usage"),
            Config::local_remove_ignored_dir,
            "Un-ignored directories:",
        ),
        "ext" => apply_igcare(
            subargs,
            &t!("cmd.care.ext_usage"),
            Config::local_add_extra_supported_extension,
            "Extra supported extensions:",
        ),
        "file" => apply_igcare(
            subargs,
            &t!("cmd.care.file_usage"),
            Config::local_add_extra_supported_file,
            "Extra supported files:",
        ),
        _ => {
            println!("{}", t!("cmd.care.title").cyan().bold());
            println!("  {}", t!("cmd.care.dir_desc"));
            println!("  {}", t!("cmd.care.ext_desc"));
            println!("  {}", t!("cmd.care.file_desc"));
            println!();
            println!("{}", t!("cmd.ignore.care_examples_title").green());
            println!("  {}", t!("cmd.ignore.care_example_dir"));
            println!("  {}", t!("cmd.ignore.care_example_ext"));
            println!("  {}", t!("cmd.ignore.care_example_file"));
            Ok(false)
        }
    }
}

pub fn cmd_igcared(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let is_local = Config::get_local_config_path().is_some();

    println!();
    if is_local {
        println!("{}", t!("cmd.igcared.title_combined").cyan().bold());
        if let Some(path) = Config::get_local_config_path() {
            println!(
                "{}",
                t!("cmd.igcared.local_config", path.display()).dimmed()
            );
        }
        println!("{}", t!("cmd.igcared.combined_hint").dimmed());
    } else {
        println!("{}", t!("cmd.igcared.title_global").cyan().bold());
    }
    println!("{}", t!("cmd.igcared.separator").cyan());

    let combined_dirs = Config::get_ignored_dirs_combined();
    let combined_fmt = FormatConfig::from_both();

    println!("{}", t!("cmd.igcared.ignored_dirs").yellow());
    for d in &combined_dirs {
        println!("  - {}", d.red());
    }
    println!("{}", t!("cmd.igcared.ignored_exts").yellow());
    for e in &combined_fmt.ignored_extensions {
        println!("  - .{}", e.red());
    }
    println!("{}", t!("cmd.igcared.extra_exts").yellow());
    for e in &combined_fmt.extra_extensions {
        println!("  - .{}", e.green());
    }
    println!("{}", t!("cmd.igcared.ignored_files").yellow());
    for f in &combined_fmt.ignored_files {
        println!("  - {}", f.red());
    }
    println!("{}", t!("cmd.igcared.extra_files").yellow());
    for f in &combined_fmt.extra_files {
        println!("  - {}", f.green());
    }

    if is_local {
        println!();
        println!("{}", t!("cmd.igcared.local_tip").dimmed());
        println!("{}", t!("cmd.igcared.local_tip2").dimmed());
    }
    Ok(false)
}

pub fn cmd_igcare(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("cmd.igcare.main_title").cyan().bold());
        println!("  {}", t!("cmd.igcare.export_cmd"));
        println!("  {}", t!("cmd.igcare.export_select_cmd"));
        println!("  {}", t!("cmd.igcare.import_cmd"));
        println!();
        println!("{}", t!("cmd.igcare.examples_title").green());
        println!("  {}", t!("cmd.igcare.example_export_all"));
        println!("  {}", t!("cmd.igcare.example_export_select"));
        println!("  {}", t!("cmd.igcare.example_import"));
    } else {
        let ig_parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = ig_parts[0].to_lowercase();
        let subargs = ig_parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "export" => {
                if subargs.is_empty() {
                    print_error(&t!("cmd.igcare.export_usage"));
                } else {
                    let export_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if export_parts.len() < 2 {
                        print_error(&t!("cmd.igcare.export_usage"));
                    } else {
                        let flag = export_parts[0].to_lowercase();
                        let export_name = export_parts[1];
                        if export_name.is_empty() {
                            print_error(&t!("cmd.igcare.export_name_empty"));
                        } else if flag == "--all" || flag == "-a" {
                            igcare_export_all(export_name)?;
                        } else if flag == "--select" || flag == "-s" {
                            igcare_export_select(export_name)?;
                        } else {
                            print_error(&t!("cmd.igcare.unknown_flag", flag));
                        }
                    }
                }
            }
            "import" => {
                if subargs.is_empty() {
                    print_error(&t!("cmd.igcare.import_usage"));
                    println!("{}", t!("cmd.igcare.import_example"));
                } else {
                    igcare_import(subargs)?;
                }
            }
            "make" | "mk" => {
                let name = if !subargs.is_empty() {
                    subargs
                } else {
                    "example"
                };
                let path = std::env::current_dir()
                    .unwrap_or_default()
                    .join(format!("{}.ntc.igcare", name));
                if path.exists() {
                    print_error(&t!("cmd.igcare.file_exists", path.display()));
                } else {
                    let content = r#"# ntc ignore/care settings template
# Import with: igcare import <file>

ignored_directory_names = []
ignored_extensions = []
extra_supported_extensions = []
ignored_files = []
extra_supported_files = []
"#;
                    match std::fs::write(&path, content) {
                        Ok(()) => {
                            print_success(&t!("cmd.igcare.template_created", path.display()));
                            print_info(&t!("cmd.igcare.template_hint"));
                        }
                        Err(e) => print_error(&t!("cmd.igcare.create_failed", e)),
                    }
                }
            }
            _ => {
                print_error(&t!("cmd.igcare.unknown_subcmd", subcmd));
                println!("{}", t!("cmd.igcare.type_help"));
            }
        }
    }
    Ok(false)
}
