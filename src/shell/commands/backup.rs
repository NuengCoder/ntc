//! Consolidated `backup` command — create, restore, undo, diff, and list
//! project backups through a single entry point with subcommands.

use crate::backup::BackupManager;
use crate::backup_diff::BackupDiff;
use crate::backup_manifest::display_path;
use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_info, print_success};
use crate::shell::helpers::show_tree;
use crate::t;
use crate::utils::interactive::{InteractiveMenu, MenuItem};

use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub fn cmd_backup(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "create" | "cr" => cmd_create(subargs, nav),
        "restore" | "rs" => cmd_restore(subargs, nav),
        "undo" | "ud" => cmd_undo(subargs, nav),
        "clear" | "cl" => cmd_clear(subargs, nav),
        "recreate" | "re" => cmd_recreate(subargs, nav),
        "diff" => cmd_diff(subargs, nav),
        "list" | "ls" => cmd_list(nav),
        _ => {
            println!("{}", t!("backup.help_title").cyan().bold());
            println!("{}", t!("backup.help_create"));
            println!("{}", t!("backup.help_list"));
            println!("{}", t!("backup.help_restore"));
            println!("{}", t!("backup.help_restore_choose"));
            println!("{}", t!("backup.help_restore_deleted"));
            println!("{}", t!("backup.help_restore_choose_deleted"));
            println!("{}", t!("backup.help_undo"));
            println!("{}", t!("backup.help_undo_cls"));
            println!("{}", t!("backup.help_undo_force"));
            println!("{}", t!("backup.help_clear"));
            println!("{}", t!("backup.help_clear_force"));
            println!("{}", t!("backup.help_recreate"));
            println!("{}", t!("backup.help_recreate_force"));
            println!("{}", t!("backup.help_diff"));
            println!();
            println!("{}", t!("backup.help_examples").green());
            println!("  backup create");
            println!("  backup restore 3");
            println!("  backup undo");
            println!("  backup clear");
            println!("  backup recreate --force");
            println!("  backup diff 1");
            Ok(false)
        }
    }
}

fn cmd_create(args: &str, nav: &mut Navigator) -> Result<bool> {
    match args {
        "--where" | "-w" => {
            BackupManager::show_backup_location(nav.current_path());
        }
        "--verify" | "-v" => {
            let exists = BackupManager::has_backups(nav.current_path());
            let project_hash = crate::backup::compute_project_hash(nav.current_path());
            let backup_dir =
                crate::backup_manifest::BackupIndex::get_project_backup_dir(&project_hash);
            if exists {
                let count = BackupManager::list_backups(nav.current_path())?.len();
                print_success(&t!("backup.exists", count, display_path(&backup_dir)));
            } else {
                print_info(&t!("backup.none"));
                println!("{}", t!("backup.create_hint").green());
            }
        }
        _ => {
            if !args.is_empty() {
                print_error(&t!("backup.unknown_create_option", args));
            } else {
                BackupManager::create_backup(nav.current_path())?;
            }
        }
    }
    Ok(false)
}

fn cmd_clear(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args == "--force" || args == "-f" {
        BackupManager::clear_backups(nav.current_path())?;
        print_success(&t!("backup.cleared"));
    } else if args.is_empty() {
        let backups = BackupManager::list_backups(nav.current_path())?;
        if backups.is_empty() {
            print_info(&t!("backup.none"));
            return Ok(false);
        }
        println!();
        println!("{}", t!("backup.warning_title").yellow().bold());
        println!("{}", t!("backup.warning_count", backups.len()).yellow());
        for (num, date, size, file_count) in backups.iter().take(10) {
            println!(
                "{}",
                t!("backup.entry_file_count", num, date, size, file_count)
            );
        }
        if backups.len() > 10 {
            println!("{}", t!("backup.and_more", backups.len() - 10));
        }
        println!();
        print!("{} ", t!("backup.confirm").red());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            BackupManager::clear_backups(nav.current_path())?;
            print_success(&t!("backup.cleared"));
        } else {
            println!("{}", t!("backup.cancelled").dimmed());
        }
    } else {
        print_error(&t!("backup.unknown_clear_option", args));
        println!("{}", t!("backup.usage_clear"));
    }
    Ok(false)
}

fn cmd_recreate(args: &str, nav: &mut Navigator) -> Result<bool> {
    let force = args == "--force" || args == "-f";

    if !force && !args.is_empty() {
        print_error(&t!("backup.unknown_recreate_option", args));
        println!("{}", t!("backup.usage_recreate"));
        return Ok(false);
    }

    if force || confirm_recreate(nav)? {
        BackupManager::clear_backups(nav.current_path())?;
        BackupManager::clear_undo_history(nav.current_path())?;
        BackupManager::create_backup(nav.current_path())?;
        print_success(&t!("backup.recreated"));
    }
    Ok(false)
}

fn confirm_recreate(nav: &mut Navigator) -> Result<bool> {
    let has_backups = BackupManager::has_backups(nav.current_path());

    if !has_backups {
        println!("{}", t!("backup.recreate_prompt").yellow());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        return Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"));
    }

    println!();
    println!("{}", t!("backup.recreate_will_title").yellow().bold());
    println!("{}", t!("backup.recreate_step1"));
    println!("{}", t!("backup.recreate_step2"));
    println!("{}", t!("backup.recreate_step3"));
    println!();
    print!("{} ", t!("backup.recreate_proceed").red());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn cmd_restore(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    let has_choose = tokens.contains(&"--choose");
    let has_deleted = tokens.contains(&"--deleted");
    let has_force = tokens.contains(&"--force") || tokens.contains(&"-f");

    // Extract explicit backup number if present
    let explicit_num = tokens.iter().find_map(|t| t.parse::<usize>().ok());

    let project_root = nav.current_path().to_path_buf();

    let backup_number = match explicit_num {
        Some(n) => n,
        None if has_choose || has_deleted => match pick_backup_interactive(&project_root)? {
            Some(n) => n,
            None => return Ok(false),
        },
        None if args.is_empty() => {
            // Original stdin-based interactive picker (backward compat)
            let backups = BackupManager::list_backups(&project_root)?;
            if backups.is_empty() {
                print_info(&t!("backup.none"));
                return Ok(false);
            }
            println!();
            println!(
                "{}",
                "==================================================".cyan()
            );
            println!("{}", t!("backup.available_title").cyan().bold());
            println!(
                "{}",
                "==================================================".cyan()
            );
            for (i, (num, date, size, file_count)) in backups.iter().enumerate() {
                println!(
                    "{}",
                    t!("backup.select_entry", i + 1, num, date, size, file_count)
                );
            }
            println!("{}", t!("backup.cancel_opt").red());
            println!();
            print!("{} ", t!("backup.select_prompt", backups.len()).green());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if input == "0" || input.is_empty() {
                println!("{}", t!("backup.restore_cancelled").dimmed());
                return Ok(false);
            }
            match input.parse::<usize>() {
                Ok(n) if n >= 1 && n <= backups.len() => backups[n - 1].0,
                _ => {
                    print_error(&t!("backup.invalid_selection", input));
                    return Ok(false);
                }
            }
        }
        _ => {
            print_error(&t!("backup.invalid_arg", args));
            println!("{}", t!("backup.usage_restore"));
            return Ok(false);
        }
    };

    let result = if has_deleted {
        BackupManager::restore_deleted_only(&project_root, backup_number, !has_force)
    } else {
        BackupManager::restore_backup(&project_root, backup_number, !has_force)
    };

    match result {
        Ok(()) => {
            clear_screen();
            show_tree(nav, Some(1), false, false, false, false, false, false);
        }
        Err(e) => print_error(&format!("{}", e)),
    }

    Ok(false)
}

/// Present an InteractiveMenu of available backups and return the chosen number.
fn pick_backup_interactive(project_root: &std::path::Path) -> Result<Option<usize>> {
    let backups = BackupManager::list_backups(project_root)?;
    if backups.is_empty() {
        print_info(&t!("backup.none"));
        return Ok(None);
    }

    let items: Vec<MenuItem> = backups
        .iter()
        .map(|(num, date, size, count)| {
            MenuItem::with_detail(
                t!("backup.menu_item", num),
                t!("backup.pick_detail", date, size, count),
            )
        })
        .collect();

    let title = t!("backup.pick_title");
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(t!("backup.pick_prompt", backups.len()));

    menu.run().map(|opt| opt.map(|i| backups[i].0))
}

fn cmd_undo(args: &str, nav: &mut Navigator) -> Result<bool> {
    match args {
        "--cls" | "--clear" => {
            println!();
            print!("{} ", t!("backup.clear_history_prompt").red());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                BackupManager::clear_undo_history(nav.current_path())?;
            } else {
                println!("{}", t!("backup.cancelled").dimmed());
            }
        }
        "--force" | "-f" => {
            BackupManager::clear_undo_history(nav.current_path())?;
        }
        _ => {
            if !args.is_empty() {
                print_error(&t!("backup.unknown_undo_option", args));
            } else {
                BackupManager::undo_last_restore(nav.current_path())?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false, false);
            }
        }
    }
    Ok(false)
}

fn cmd_diff(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        BackupDiff::run_diff_interactive(nav.current_path())?;
    } else {
        match args.parse::<usize>() {
            Ok(n) => BackupDiff::generate_diff(nav.current_path(), n)?,
            Err(_) => print_error(&t!("backup.invalid_diff_arg", args)),
        }
    }
    Ok(false)
}

fn cmd_list(nav: &mut Navigator) -> Result<bool> {
    let exists = BackupManager::has_backups(nav.current_path());
    let project_hash = crate::backup::compute_project_hash(nav.current_path());
    let backup_dir = crate::backup_manifest::BackupIndex::get_project_backup_dir(&project_hash);
    if exists {
        let backups = BackupManager::list_backups(nav.current_path())?;
        println!("{}", t!("backup.list_title", backups.len()).cyan().bold());
        for (num, date, size, file_count) in &backups {
            println!("{}", t!("backup.list_entry", num, date, size, file_count));
        }
        println!(
            "{}",
            t!("backup.list_location", display_path(&backup_dir)).dimmed()
        );
    } else {
        print_info(&t!("backup.none"));
    }
    Ok(false)
}
