use crate::backup::BackupManager;
use crate::backup_diff::BackupDiff;
use crate::backup_manifest::display_path;
use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success, print_info};
use crate::shell::helpers::show_tree;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub fn cmd_bkup(args: &str, nav: &mut Navigator) -> Result<bool> {
    match args {
        "--where" | "-w" => {
            BackupManager::show_backup_location(nav.current_path());
        }

        "--cls" | "--clear" => {
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
            } else {
                println!("{}", "Clear cancelled.".dimmed());
            }
        }

        "--force" | "-f" => {
            BackupManager::clear_backups(nav.current_path())?;
        }

        "" => {
            BackupManager::create_backup(nav.current_path())?;
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
                println!("{}", "Use 'bkup' to create your first backup.".green());
            }
        }

        _ => {
            print_error(&format!("Unknown bkup option: {}", args));
            println!("Usage:");
            println!("  bkup              Create a new backup");
            println!("  bkup --verify     Check if backups exist");
            println!("  bkup --where      Show backup storage location");
            println!("  bkup --cls        Delete all backups (asks confirmation)");
            println!("  bkup --force      Delete all backups (no confirmation)");
        }
    }
    Ok(false)
}

pub fn cmd_pldw(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let backups = BackupManager::list_backups(nav.current_path())?;
        if backups.is_empty() {
            print_info("No backups found for this project. Use 'bkup' to create one.");
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
                BackupManager::restore_backup(
                    nav.current_path(), backup_num, true
                )?;
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
        println!("Usage:");
        println!("  pldw              Interactive restore menu");
        println!("  pldw <number>     Restore backup by number");
    }
    Ok(false)
}

pub fn cmd_unpd(args: &str, nav: &mut Navigator) -> Result<bool> {
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

        "" => {
            BackupManager::undo_last_restore(nav.current_path())?;
            clear_screen();
            show_tree(nav, Some(1), false, false, false, false, false);
        }

        _ => {
            print_error(&format!("Unknown unpd option: {}", args));
            println!("Usage:");
            println!("  unpd              Undo the last restore");
            println!("  unpd --cls        Clear undo history (asks confirmation)");
            println!("  unpd --force      Clear undo history (no confirmation)");
        }
    }
    Ok(false)
}

pub fn cmd_diff(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        BackupDiff::run_diff_interactive(nav.current_path())?;
    } else {
        match args.parse::<usize>() {
            Ok(n) => {
                BackupDiff::generate_diff(nav.current_path(), n)?;
            }
            Err(_) => {
                print_error(&format!("Invalid argument: {}. Usage: diff <backup_number>", args));
            }
        }
    }
    Ok(false)
}
