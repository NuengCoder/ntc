use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::helpers::{show_tree, ignoresc_loop, caresc_loop, igcare_export_all, igcare_export_select, igcare_import};
use crate::utils::interactive::{InteractiveMenu, MenuItem};

use anyhow::Result;
use colored::*;

fn apply_igcare(args: &str, usage: &str, action: fn(&str) -> std::result::Result<(), Box<dyn std::error::Error>>, action_desc: &str) -> Result<bool> {
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
            print_success(&format!("{} {} item(s)", action_desc, count));
            print_info("Run 'ignored' to see current ignore/care settings");
        }
    }
    Ok(false)
}

pub fn cmd_ignored(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let is_local = Config::get_local_config_path().is_some();

    println!();
    if is_local {
        println!("{}", "📌 Ignore/Care Settings (global + local)".cyan().bold());
        if let Some(path) = Config::get_local_config_path() {
            println!("{}", format!("   Local config: {}", path.display()).dimmed());
        }
        println!("{}", "   Items from both sources are combined below.".dimmed());
    } else {
        println!("{}", "📌 Ignore/Care Settings (global)".cyan().bold());
    }
    println!("{}", "==================================================".cyan());

    let combined_dirs = Config::get_ignored_dirs_combined();
    let combined_fmt = FormatConfig::from_both();

    println!("{}", "Ignored directories:".yellow());
    for d in &combined_dirs { println!("  - {}", d.red()); }
    println!("{}", "Ignored extensions:".yellow());
    for e in &combined_fmt.ignored_extensions { println!("  - .{}", e.red()); }
    println!("{}", "Extra supported extensions:".yellow());
    for e in &combined_fmt.extra_extensions { println!("  - .{}", e.green()); }
    println!("{}", "Ignored files:".yellow());
    for f in &combined_fmt.ignored_files { println!("  - {}", f.red()); }
    println!("{}", "Extra supported files:".yellow());
    for f in &combined_fmt.extra_files { println!("  - {}", f.green()); }

    if is_local {
        println!();
        println!("{}", "💡 Tip: Local settings are from ntconfig.toml. Delete that file".dimmed());
        println!("{}", "   or run 'local deinit' to revert to global-only filtering.".dimmed());
    }
    Ok(false)
}

pub fn cmd_ignore(args: &str, _nav: &mut Navigator) -> Result<bool> {
    apply_igcare(args, "Usage: ignore <name>[, <name>, ...]", Config::local_add_ignored_dir, "Ignored directories:")
}

pub fn cmd_cared(args: &str, _nav: &mut Navigator) -> Result<bool> {
    apply_igcare(args, "Usage: cared <name>[, <name>, ...]", Config::local_remove_ignored_dir, "Un-ignored directories:")
}

pub fn cmd_ignoref(args: &str, _nav: &mut Navigator) -> Result<bool> {
    apply_igcare(args, "Usage: ignoref <ext>[, <ext>, ...]", Config::local_add_ignored_extension, "Ignored extensions:")
}

pub fn cmd_caref(args: &str, _nav: &mut Navigator) -> Result<bool> {
    apply_igcare(args, "Usage: caref <ext>[, <ext>, ...]", Config::local_add_extra_supported_extension, "Extra supported extensions:")
}

pub fn cmd_ignoren(args: &str, _nav: &mut Navigator) -> Result<bool> {
    apply_igcare(args, "Usage: ignoren <file>[, <file>, ...]", Config::local_add_ignored_file, "Ignored files:")
}

pub fn cmd_caren(args: &str, _nav: &mut Navigator) -> Result<bool> {
    apply_igcare(args, "Usage: caren <file>[, <file>, ...]", Config::local_add_extra_supported_file, "Extra supported files:")
}

pub fn cmd_igcare(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", "Ignore/Care (igcare) Commands:".cyan().bold());
        println!("  igcare export --all <name>           Export all settings to <name>.ntc.igcare");
        println!("  igcare export --select <name>        Select categories to export to <name>.ntc.igcare");
        println!("  igcare import <file>                 Import settings from a .ntc.igcare file");
        println!();
        println!("{}", "Examples:".green());
        println!("  igcare export --all myproject        # Creates myproject.ntc.igcare");
        println!("  igcare export --select myproject     # Pick which categories to export");
        println!("  igcare import myproject.ntc.igcare   # Import into current config");
    } else {
        let ig_parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = ig_parts[0].to_lowercase();
        let subargs = ig_parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "export" => {
                if subargs.is_empty() {
                    print_error("Usage: igcare export --all <name> | igcare export --select <name>");
                } else {
                    let export_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if export_parts.len() < 2 {
                        print_error("Usage: igcare export --all <name> | igcare export --select <name>");
                    } else {
                        let flag = export_parts[0].to_lowercase();
                        let export_name = export_parts[1];
                        if export_name.is_empty() {
                            print_error("Export name cannot be empty.");
                        } else if flag == "--all" || flag == "-a" {
                            igcare_export_all(export_name)?;
                        } else if flag == "--select" || flag == "-s" {
                            igcare_export_select(export_name)?;
                        } else {
                            print_error(&format!("Unknown export flag: {}. Use --all or --select", flag));
                        }
                    }
                }
            }
            "import" => {
                if subargs.is_empty() {
                    print_error("Usage: igcare import <file>");
                    println!("Example: igcare import myproject.ntc.igcare");
                } else {
                    igcare_import(subargs)?;
                }
            }
            "make" | "mk" => {
                let name = if !subargs.is_empty() { subargs } else { "example" };
                let path = std::env::current_dir().unwrap_or_default().join(format!("{}.ntc.igcare", name));
                if path.exists() {
                    print_error(&format!("File already exists: {}", path.display()));
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
                            print_success(&format!("Created igcare template: {}", path.display()));
                            print_info("This is a local template file. Use 'igcare import' to apply settings.");
                        }
                        Err(e) => print_error(&format!("Failed to create file: {}", e)),
                    }
                }
            }
            _ => {
                print_error(&format!("Unknown igcare subcommand: {}", subcmd));
                println!("Type 'igcare' for help.");
            }
        }
    }
    Ok(false)
}

pub fn cmd_ignores(_args: &str, nav: &mut Navigator) -> Result<bool> {
    let dirs = nav.list_subdirs()?;
    let items: Vec<MenuItem> = dirs.iter().map(|(_, name)| MenuItem::simple(name)).collect();

    let menu = InteractiveMenu::new("ignores — Select directory to ignore")
        .items(items)
        .prompt("Ignore which directory?");

    if let Some(idx) = menu.run()? {
        if let Some((_, name)) = dirs.get(idx) {
            let _ = Config::local_add_ignored_dir(name);
            Config::reload_global();
            print_success(&format!("Ignored directory: {}", name));
            print_info("Run 'ignored' to see all ignore/care settings");
        }
    }
    Ok(false)
}

pub fn cmd_ignoresc(nav: &mut Navigator) -> Result<bool> {
    ignoresc_loop(nav)?;
    show_tree(nav, Some(1), false, false, false, false, false);
    Ok(false)
}

pub fn cmd_cares(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let ignored = Config::get_ignored_dirs_combined();
    let mut ignored_vec: Vec<String> = ignored.into_iter().collect();
    ignored_vec.sort_by_key(|a| a.to_lowercase());

    let items: Vec<MenuItem> = ignored_vec.iter().map(MenuItem::simple).collect();
    let menu = InteractiveMenu::new("cares — Select ignored directory to un-ignore")
        .items(items)
        .prompt("Care about which directory?");

    if let Some(idx) = menu.run()? {
        let name = &ignored_vec[idx];
        let _ = Config::local_remove_ignored_dir(name);
        Config::reload_global();
        print_success(&format!("Un-ignored directory: {}", name));
        print_info("Run 'ignored' to see all ignore/care settings");
    }
    Ok(false)
}

pub fn cmd_caresc(nav: &mut Navigator) -> Result<bool> {
    caresc_loop(nav)?;
    show_tree(nav, Some(1), false, false, false, false, false);
    Ok(false)
}
