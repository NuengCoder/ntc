use crate::explorer::human_readable_size;
use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success};
use crate::shell::helpers::{show_tree, gosc_loop};
use crate::teleport::TeleportManager;
use super::parse_search_args;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn cmd_go(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: go <directory_path>");
        println!("       go to <tp_name>      Teleport to savepoint");
        println!("Example: go C:\\Users");
        println!("         go subdir");
        println!("         go to work         # Teleport to 'work' savepoint");
    } else {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.len() >= 2 && parts[0].to_lowercase() == "to" {
            let tp_name = parts[1];
            if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
                TeleportManager::jump_by_name(nav, tp_name)?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false);
            } else {
                print_error(&format!("Teleport point not found: '{}'", tp_name));
                println!("Use 'tp list' to see all savepoints.");
            }
        } else {
            nav.go_to(Path::new(args))?;
            clear_screen();
            print_success(&format!("Navigated to: {}", nav.display_path()));
            show_tree(nav, Some(1), false, false, false, false, false);
        }
    }
    Ok(false)
}

pub fn cmd_godrive(args: &str, nav: &mut Navigator) -> Result<bool> {
    #[cfg(not(windows))]
    {
        let _ = (args, nav);
        print_error("Drive navigation is only supported on Windows.");
    }
    #[cfg(windows)]
    if args.is_empty() {
        let drives = Navigator::list_drives();
        println!("{}", "Available drives:".cyan().bold());
        for (i, d) in drives.iter().enumerate() {
            println!("  {}: {}:\\", i + 1, d);
        }
        println!();
        print!("Enter drive letter or number (or 'cancel'): ");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;
        let choice = choice.trim().to_lowercase();

        if choice == "cancel" || choice.is_empty() {
            println!("Cancelled.");
        } else if choice.len() == 1 && choice.chars().next().unwrap().is_alphabetic() {
            nav.go_drive(choice.chars().next().unwrap())?;
            clear_screen();
            show_tree(nav, Some(1), false, false, false, false, false);
        } else if let Ok(num) = choice.parse::<usize>() {
            if num > 0 && num <= drives.len() {
                nav.go_drive(drives[num - 1])?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false);
            } else {
                print_error("Invalid drive number.");
            }
        } else {
            print_error("Invalid input.");
        }
    } else {
        let letter = args.chars().next().unwrap_or('C');
        nav.go_drive(letter)?;
        clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false);
    }
    Ok(false)
}

pub fn cmd_back(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        match nav.go_back() {
            Ok(()) => {
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false);
            }
            Err(e) => print_error(&format!("{}", e)),
        }
    } else {
        match args.parse::<usize>() {
            Ok(n) if n > 0 => {
                let mut success = true;
                for i in 0..n {
                    match nav.go_back() {
                        Ok(()) => {}
                        Err(e) => {
                            if i == 0 {
                                print_error(&format!("{}", e));
                            } else {
                                print_error(&format!("Null parent at step {} - nowhere to go back", i + 1));
                            }
                            success = false;
                            break;
                        }
                    }
                }
                if success {
                    clear_screen();
                    show_tree(nav, Some(1), false, false, false, false, false);
                }
            }
            _ => print_error(&format!("Invalid number: {}. Usage: back [n]", args)),
        }
    }
    Ok(false)
}

pub fn cmd_where(nav: &mut Navigator) -> Result<bool> {
    let exe = std::env::current_exe().unwrap_or_default();
    let config_path = dirs::config_dir()
        .map(|d| d.join("ntc").join("config.toml"))
        .filter(|p| p.exists())
        .unwrap_or_else(|| {
            dirs::config_dir()
                .map(|d| d.join("ntc").join("config.toml"))
                .unwrap_or_else(|| PathBuf::from("Not found"))
        });

    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                         ntc Location Info                        ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    println!("  {} {}", "📁 Executable:".green().bold(), exe.display().to_string().cyan());
    println!("  {} {}", "⚙️  Config file:".yellow().bold(), config_path.display().to_string().cyan());
    println!("  {} {}", "📂 Current dir:".blue().bold(), nav.display_path().cyan());
    println!();

    if config_path.exists() {
        println!("  {}", "✓ Config file exists".green());
        if let Ok(metadata) = std::fs::metadata(&config_path) {
            println!("  {} {}", "📏 Config size:".dimmed(), human_readable_size(metadata.len()).dimmed());
        }
    } else {
        println!("  {}", "⚠ Config file not found (will be created on first save)".yellow());
    }
    println!();
    Ok(false)
}

pub fn cmd_gos(nav: &mut Navigator) -> Result<bool> {
    let dirs = nav.list_subdirs()?;
    println!();
    println!("{}", "gos where?".cyan().bold());
    println!("  {} {}", "0".yellow(), "exit".dimmed());
    if dirs.is_empty() {
        println!("  {}", "(no subdirectories)".dimmed());
    } else {
        for (i, name) in &dirs {
            println!("  {} {}", i.to_string().yellow(), name.bright_magenta());
        }
    }
    println!();
    print!("{} ", ">".green());

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    if choice == "0" || choice.is_empty() {
        println!("{}", "Staying here.".dimmed());
    } else if let Ok(num) = choice.parse::<usize>() {
        if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
            let new_path = nav.current_path().join(name);
            nav.go_to(&new_path)?;
            clear_screen();
            print_success(&format!("Navigated to: {}", nav.display_path()));
            show_tree(nav, Some(1), false, false, false, false, false);
        } else {
            print_error("Invalid number.");
        }
    } else {
        print_error("Invalid input.");
    }
    Ok(false)
}

pub fn cmd_gosc(nav: &mut Navigator) -> Result<bool> {
    gosc_loop(nav)?;
    show_tree(nav, Some(1), false, false, false, false, false);
    Ok(false)
}

pub fn cmd_fgo(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: fgo <pattern> [-d <depth>]");
        println!("  fgo main.rs        # search then pick file to navigate to its parent");
        println!("  fgo main.rs -d 5   # search up to 5 levels deep");
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_files(nav.current_path(), &pattern, max_depth);
    if results.is_empty() {
        let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
        print!("{}", output);
        return Ok(false);
    }
    println!();
    println!("{}", format!("🔍 Found {} file(s) for \"{}\":", results.len(), pattern).cyan().bold());
    for (i, r) in results.iter().enumerate() {
        let path_str = r.full_path.to_string_lossy();
        println!("  {}. {}", (i + 1).to_string().yellow(), path_str.dimmed());
    }
    println!("  {}", "0. Cancel".red());
    println!();
    print!("{} ", "Select file to navigate to its parent directory: ".green());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    if input == "0" || input.is_empty() {
        println!("{}", "Cancelled.".dimmed());
        return Ok(false);
    }
    if let Ok(n) = input.parse::<usize>() {
        if n >= 1 && n <= results.len() {
            let target = &results[n - 1].full_path;
            let parent = target.parent().unwrap_or(target);
            nav.go_to(parent)?;
            clear_screen();
            print_success(&format!("Navigated to: {}", nav.display_path()));
            show_tree(nav, Some(1), false, false, false, false, false);
        } else {
            print_error("Invalid selection.");
        }
    } else {
        print_error("Invalid input.");
    }
    Ok(false)
}
