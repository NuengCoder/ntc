use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::helpers::{show_tree, ignoresc_loop, caresc_loop, igcare_export_all, igcare_export_select, igcare_import};

use anyhow::Result;
use colored::*;

pub fn cmd_ignored(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let dirs = Config::global_get_ignored_dirs();
    let fmt_cfg = FormatConfig::from_global();
    let is_local = Config::get_local_config_path().is_some();

    println!();
    if is_local {
        println!("{}", "📌 Ignore/Care Settings (from local ntconfig.toml)".cyan().bold());
        if let Some(path) = Config::get_local_config_path() {
            println!("{}", format!("   Config: {}", path.display()).dimmed());
        }
    } else {
        println!("{}", "📌 Ignore/Care Settings (global)".cyan().bold());
    }
    println!("{}", "==================================================".cyan());

    println!("{}", "Ignored directories:".yellow());
    for d in &dirs { println!("  - {}", d.red()); }
    println!("{}", "Ignored extensions:".yellow());
    for e in &fmt_cfg.ignored_extensions { println!("  - .{}", e.red()); }
    println!("{}", "Extra supported extensions:".yellow());
    for e in &fmt_cfg.extra_extensions { println!("  - .{}", e.green()); }
    println!("{}", "Ignored files:".yellow());
    for f in &fmt_cfg.ignored_files { println!("  - {}", f.red()); }
    println!("{}", "Extra supported files:".yellow());
    for f in &fmt_cfg.extra_files { println!("  - {}", f.green()); }

    if is_local {
        println!();
        println!("{}", "💡 Tip: These settings are project-specific (saved in ntconfig.toml)".dimmed());
        println!("{}", "   Global settings are hidden while in this directory".dimmed());
    }
    Ok(false)
}

pub fn cmd_ignore(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: ignore <name>[, <name>, ...]");
    } else {
        for name in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let _ = Config::local_add_ignored_dir(name);
        }
        Config::reload_global();
    }
    Ok(false)
}

pub fn cmd_cared(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: cared <name>[, <name>, ...]");
    } else {
        for name in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let _ = Config::local_remove_ignored_dir(name);
        }
        Config::reload_global();
    }
    Ok(false)
}

pub fn cmd_ignoref(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: ignoref <ext>[, <ext>, ...]");
    } else {
        for ext in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let _ = Config::local_add_ignored_extension(ext);
        }
        Config::reload_global();
    }
    Ok(false)
}

pub fn cmd_caref(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: caref <ext>[, <ext>, ...]");
    } else {
        for ext in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let _ = Config::local_add_extra_supported_extension(ext);
        }
        Config::reload_global();
    }
    Ok(false)
}

pub fn cmd_ignoren(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: ignoren <file>[, <file>, ...]");
    } else {
        for name in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let _ = Config::local_add_ignored_file(name);
        }
        Config::reload_global();
    }
    Ok(false)
}

pub fn cmd_caren(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: caren <file>[, <file>, ...]");
    } else {
        for name in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let _ = Config::local_add_extra_supported_file(name);
        }
        Config::reload_global();
    }
    Ok(false)
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
    println!();
    println!("{}", "ignores — Select directory to ignore".cyan().bold());
    println!("  {} {}", "0".yellow(), "exit".dimmed());
    if dirs.is_empty() {
        println!("  {}", "(no subdirectories)".dimmed());
    } else {
        for (i, name) in &dirs {
            println!("  {} {}", i.to_string().yellow(), name.blue());
        }
    }
    println!();
    print!("{} ", "Ignore which directory?".green());

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();
    if choice == "0" || choice.is_empty() {
        println!("{}", "Cancelled.".dimmed());
    } else if let Ok(num) = choice.parse::<usize>() {
        if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
            let _ = Config::local_add_ignored_dir(name);
            Config::reload_global();
        } else {
            print_error("Invalid number.");
        }
    } else {
        print_error("Invalid input.");
    }
    Ok(false)
}

pub fn cmd_ignoresc(nav: &mut Navigator) -> Result<bool> {
    ignoresc_loop(nav)?;
    show_tree(nav, Some(1), false, false, false, false, false);
    Ok(false)
}

pub fn cmd_cares(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let ignored = Config::global_get_ignored_dirs();
    let mut ignored_vec: Vec<String> = ignored.into_iter().collect();
    ignored_vec.sort_by_key(|a| a.to_lowercase());
    println!();
    println!("{}", "cares — Select ignored directory to un-ignore".cyan().bold());
    println!("  {} {}", "0".yellow(), "exit".dimmed());
    if ignored_vec.is_empty() {
        println!("  {}", "(no ignored directories)".dimmed());
    } else {
        for (i, name) in ignored_vec.iter().enumerate() {
            println!("  {} {}", (i + 1).to_string().yellow(), name.red());
        }
    }
    println!();
    print!("{} ", "Care about which directory?".green());
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();
    if choice == "0" || choice.is_empty() {
        println!("{}", "Cancelled.".dimmed());
    } else if let Ok(num) = choice.parse::<usize>() {
        if num >= 1 && num <= ignored_vec.len() {
            let name = &ignored_vec[num - 1];
            let _ = Config::local_remove_ignored_dir(name);
            Config::reload_global();
        } else {
            print_error("Invalid number.");
        }
    } else {
        print_error("Invalid input.");
    }
    Ok(false)
}

pub fn cmd_caresc(nav: &mut Navigator) -> Result<bool> {
    caresc_loop(nav)?;
    show_tree(nav, Some(1), false, false, false, false, false);
    Ok(false)
}
