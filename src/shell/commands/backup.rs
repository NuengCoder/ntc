//! Consolidated `backup` command — create, restore, undo, diff, and list
//! project backups through a single entry point with subcommands.

use crate::backup::BackupManager;
use crate::backup_diff::BackupDiff;
use crate::backup_manifest::display_path;
use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success, print_info};
use crate::shell::helpers::show_tree;

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
            println!("{}", "backup — Manage project backups".cyan().bold());
            println!("  backup create             Create a new backup");
            println!("  backup list               List all backups");
            println!("  backup restore [num]      Restore a backup (interactive if no num)");
            println!("  backup undo               Undo last restore");
            println!("  backup undo --cls         Clear undo history");
            println!("  backup undo --force       Clear undo history (no confirm)");
            println!("  backup clear              Delete all backups (with confirmation)");
            println!("  backup clear --force      Delete all backups (no confirmation)");
            println!("  backup recreate           Clear all + undo history + create fresh");
            println!("  backup recreate --force   Same, skip all confirmations");
            println!("  backup diff [num]         Diff current project against a backup");
            println!();
            println!("{}", "Examples:".green());
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
            let backup_dir = crate::backup_manifest::BackupIndex::get_project_backup_dir(&project_hash);
            if exists {
                let count = BackupManager::list_backups(nav.current_path())?.len();
                print_success(&format!("Backups exist: {} backup(s) at {}", count, display_path(&backup_dir)));
            } else {
                print_info("No backups found for this project.");
                println!("{}", "Use 'backup create' to create your first backup.".green());
            }
        }
        _ => {
            if !args.is_empty() {
                print_error(&format!("Unknown backup create option: {}", args));
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
        print_success("All backups deleted.");
    } else if args.is_empty() {
        let backups = BackupManager::list_backups(nav.current_path())?;
        if backups.is_empty() {
            print_info("No backups found for this project.");
            return Ok(false);
        }
        println!();
        println!("{}", "⚠️  WARNING: This will delete ALL backups for this project!".yellow().bold());
        println!("{}", format!("You have {} backup(s):", backups.len()).yellow());
        for (num, date, size, file_count) in backups.iter().take(10) {
            println!("  Backup #{} — {} — {} — {} files", num, date, size, file_count);
        }
        if backups.len() > 10 {
            println!("  ... and {} more", backups.len() - 10);
        }
        println!();
        print!("{} ", "Type 'yes' to confirm: ".red());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            BackupManager::clear_backups(nav.current_path())?;
            print_success("All backups deleted.");
        } else {
            println!("{}", "Clear cancelled.".dimmed());
        }
    } else {
        print_error(&format!("Unknown backup clear option: {}", args));
        println!("Usage: backup clear [--force]");
    }
    Ok(false)
}

fn cmd_recreate(args: &str, nav: &mut Navigator) -> Result<bool> {
    let force = args == "--force" || args == "-f";

    if !force && !args.is_empty() {
        print_error(&format!("Unknown backup recreate option: {}", args));
        println!("Usage: backup recreate [--force]");
        return Ok(false);
    }

    if force || confirm_recreate(nav)? {
        BackupManager::clear_backups(nav.current_path())?;
        BackupManager::clear_undo_history(nav.current_path())?;
        BackupManager::create_backup(nav.current_path())?;
        print_success("Project recreated: old backups cleared, fresh backup created.");
    }
    Ok(false)
}

fn confirm_recreate(nav: &mut Navigator) -> Result<bool> {
    let has_backups = BackupManager::has_backups(nav.current_path());

    if !has_backups {
        println!("{}", "No existing backups found. Creating fresh backup anyway? (y/N): ".yellow());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        return Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"));
    }

    println!();
    println!("{}", "⚠️  RECREATE will:".yellow().bold());
    println!("  1. Delete ALL existing backups");
    println!("  2. Clear undo history");
    println!("  3. Create a fresh backup of the current state");
    println!();
    print!("{} ", "Proceed? (y/N): ".red());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn cmd_restore(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let backups = BackupManager::list_backups(nav.current_path())?;
        if backups.is_empty() {
            print_info("No backups found for this project. Use 'backup create' to make one.");
            return Ok(false);
        }
        println!();
        println!("{}", "==================================================".cyan());
        println!("{}", "📦 Available Backups (newest first)".cyan().bold());
        println!("{}", "==================================================".cyan());
        for (i, (num, date, size, file_count)) in backups.iter().enumerate() {
            println!(
                "  {}. Backup #{} — {} — {} — {} files",
                i + 1, num, date, size, file_count
            );
        }
        println!("  {}", "0. Cancel".red());
        println!();
        print!("{} ", format!("Select backup to restore (1-{}): ", backups.len()).green());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "0" || input.is_empty() {
            println!("{}", "Restore cancelled.".dimmed());
            return Ok(false);
        }
        match input.parse::<usize>() {
            Ok(n) if n >= 1 && n <= backups.len() => {
                let backup_num = backups[n - 1].0;
                BackupManager::restore_backup(nav.current_path(), backup_num, true)?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false);
            }
            Ok(_)  => print_error(&format!("Invalid selection: {}", input)),
            Err(_) => print_error(&format!("Invalid input: {}", input)),
        }
    } else if let Ok(num) = args.parse::<usize>() {
        BackupManager::restore_backup(nav.current_path(), num, true)?;
        clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false);
    } else {
        print_error(&format!("Invalid argument: {}", args));
        println!("Usage: backup restore [number]");
    }
    Ok(false)
}

fn cmd_undo(args: &str, nav: &mut Navigator) -> Result<bool> {
    match args {
        "--cls" | "--clear" => {
            println!();
            print!("{} ", "⚠ Clear undo history? This cannot be undone. (y/N):".red());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                BackupManager::clear_undo_history(nav.current_path())?;
            } else {
                println!("{}", "Clear cancelled.".dimmed());
            }
        }
        "--force" | "-f" => {
            BackupManager::clear_undo_history(nav.current_path())?;
        }
        _ => {
            if !args.is_empty() {
                print_error(&format!("Unknown backup undo option: {}", args));
            } else {
                BackupManager::undo_last_restore(nav.current_path())?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false, false, false);
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
            Err(_) => print_error(&format!("Invalid argument: {}. Usage: backup diff <number>", args)),
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
        println!("{}", format!("Backups for this project: {} found", backups.len()).cyan().bold());
        for (num, date, size, file_count) in &backups {
            println!("  #{} — {} — {} — {} files", num, date, size, file_count);
        }
        println!("{}", format!("Location: {}", display_path(&backup_dir)).dimmed());
    } else {
        print_info("No backups found for this project.");
    }
    Ok(false)
}