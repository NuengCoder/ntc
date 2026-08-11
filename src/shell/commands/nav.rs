//! Navigation commands: `cd`, `back`, `godrive`, `gos`, `gosc`, `fgo`, `where`.

use super::parse_search_args;
use crate::explorer::human_readable_size;
use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success};
use crate::shell::helpers::{gosc_loop, show_tree};
use crate::t;
use crate::teleport::TeleportManager;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn cmd_cd(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("cmd.nav.cd.usage1"));
        println!("{}", t!("cmd.nav.cd.usage2"));
        println!("{}", t!("cmd.nav.cd.example1"));
        println!("{}", t!("cmd.nav.cd.example2"));
        println!("{}", t!("cmd.nav.cd.example3"));
    } else {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.len() >= 2 && parts[0].to_lowercase() == "to" {
            let tp_name = parts[1];
            if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
                TeleportManager::jump_by_name(nav, tp_name)?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false, false);
            } else {
                print_error(&t!("cmd.nav.cd.tp_not_found", tp_name));
                println!("{}", t!("cmd.nav.cd.tp_list"));
            }
        } else {
            nav.go_to(Path::new(args))?;
            clear_screen();
            print_success(&t!("cmd.nav.cd.navigated", nav.display_path()));
            show_tree(nav, Some(1), false, false, false, false, false, false);
        }
    }
    Ok(false)
}

pub fn cmd_godrive(args: &str, nav: &mut Navigator) -> Result<bool> {
    #[cfg(not(windows))]
    {
        let _ = (args, nav);
        print_error(&t!("cmd.nav.drive_windows_only"));
    }
    #[cfg(windows)]
    if args.is_empty() {
        let drives = Navigator::list_drives();
        println!("{}", t!("nav.available_drives").cyan().bold());
        for (i, d) in drives.iter().enumerate() {
            println!("  {}: {}:\\", i + 1, d);
        }
        println!();
        print!("{}", t!("nav.drive_prompt"));

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;
        let choice = choice.trim().to_lowercase();

        if choice == "cancel" || choice.is_empty() {
            println!("{}", t!("nav.cancelled"));
        } else if choice.len() == 1 && choice.chars().next().unwrap().is_alphabetic() {
            nav.go_drive(choice.chars().next().unwrap())?;
            clear_screen();
            show_tree(nav, Some(1), false, false, false, false, false, false);
        } else if let Ok(num) = choice.parse::<usize>() {
            if num > 0 && num <= drives.len() {
                nav.go_drive(drives[num - 1])?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false, false);
            } else {
                print_error(&t!("cmd.nav.invalid_drive"));
            }
        } else {
            print_error(&t!("cmd.nav.invalid_input"));
        }
    } else {
        let letter = args.chars().next().unwrap_or('C');
        nav.go_drive(letter)?;
        clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false, false);
    }
    Ok(false)
}

pub fn cmd_back(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        match nav.go_back() {
            Ok(()) => {
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false, false);
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
                                print_error(&t!("cmd.nav.back.invalid_step", i + 1));
                            }
                            success = false;
                            break;
                        }
                    }
                }
                if success {
                    clear_screen();
                    show_tree(nav, Some(1), false, false, false, false, false, false);
                }
            }
            _ => print_error(&t!("cmd.nav.back.invalid_number", args)),
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

    let a = |s: &str| crate::utils::terminal::asciify(s);
    println!();
    println!(
        "{}",
        a("╔══════════════════════════════════════════════════════════════════╗").cyan()
    );
    println!(
        "{}",
        a(&format!("{:^76}", t!("cmd.nav.where.title"))).cyan()
    );
    println!(
        "{}",
        a("╚══════════════════════════════════════════════════════════════════╝").cyan()
    );
    println!();
    println!(
        "  {} {}",
        t!("cmd.nav.where.executable").green().bold(),
        exe.display().to_string().cyan()
    );
    println!(
        "  {} {}",
        t!("cmd.nav.where.config_file").yellow().bold(),
        config_path.display().to_string().cyan()
    );
    println!(
        "  {} {}",
        t!("cmd.nav.where.current_dir").blue().bold(),
        nav.display_path().cyan()
    );
    println!();

    if config_path.exists() {
        println!("{}", t!("nav.config_exists").green());
        if let Ok(metadata) = std::fs::metadata(&config_path) {
            println!(
                "  {} {}",
                t!("cmd.nav.where.config_size").dimmed(),
                human_readable_size(metadata.len()).dimmed()
            );
        }
    } else {
        println!("  {}", t!("cmd.nav.where.config_missing").yellow());
    }
    println!();
    Ok(false)
}

pub fn cmd_gos(nav: &mut Navigator) -> Result<bool> {
    let dirs = nav.list_subdirs()?;
    println!();
    println!("{}", t!("nav.gos_where").cyan().bold());
    println!("  {} {}", "0".yellow(), t!("cmd.nav.gos.exit").dimmed());
    if dirs.is_empty() {
        println!("  {}", t!("cmd.nav.gos.empty").dimmed());
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
        println!("{}", t!("cmd.nav.gos.staying").dimmed());
    } else if let Ok(num) = choice.parse::<usize>() {
        if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
            let new_path = nav.current_path().join(name);
            nav.go_to(&new_path)?;
            clear_screen();
            print_success(&t!("cmd.nav.cd.navigated", nav.display_path()));
            show_tree(nav, Some(1), false, false, false, false, false, false);
        } else {
            print_error(&t!("cmd.nav.invalid_number"));
        }
    } else {
        print_error(&t!("cmd.nav.invalid_input"));
    }
    Ok(false)
}

pub fn cmd_gosc(nav: &mut Navigator) -> Result<bool> {
    gosc_loop(nav)?;
    show_tree(nav, Some(1), false, false, false, false, false, false);
    Ok(false)
}

pub fn cmd_fgo(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("cmd.nav.fgo_usage"));
        println!("{}", t!("cmd.nav.fgo.example1"));
        println!("{}", t!("cmd.nav.fgo.example2"));
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
    println!(
        "{}",
        t!("cmd.nav.fgo.found", results.len(), pattern)
            .cyan()
            .bold()
    );
    for (i, r) in results.iter().enumerate() {
        let path_str = r.full_path.to_string_lossy();
        println!("  {}. {}", (i + 1).to_string().yellow(), path_str.dimmed());
    }
    println!("  {}", t!("cmd.nav.fgo.cancel").red());
    println!();
    print!("{} ", t!("cmd.nav.fgo.select").green());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    if input == "0" || input.is_empty() {
        println!("{}", t!("cmd.nav.fgo.cancelled").dimmed());
        return Ok(false);
    }
    if let Ok(n) = input.parse::<usize>() {
        if n >= 1 && n <= results.len() {
            let target = &results[n - 1].full_path;
            let parent = target.parent().unwrap_or(target);
            nav.go_to(parent)?;
            clear_screen();
            print_success(&t!("cmd.nav.cd.navigated", nav.display_path()));
            show_tree(nav, Some(1), false, false, false, false, false, false);
        } else {
            print_error(&t!("cmd.nav.invalid_selection"));
        }
    } else {
        print_error(&t!("cmd.nav.invalid_input"));
    }
    Ok(false)
}
