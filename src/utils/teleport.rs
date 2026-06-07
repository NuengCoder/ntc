// src/teleport.rs
use crate::config::Config;
use crate::navigator::{Navigator,clear_screen};
use crate::output::{print_error, print_success, print_info};
use crate::shell::show_tree;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::path::{PathBuf};
use std::io::{self, Write};

/// Reserved command names that cannot be used as teleport names
const RESERVED_NAMES: &[&str] = &["add", "jump", "list", "rm", "cls", "help", "tp"];

/// Teleport manager for handling savepoints
pub struct TeleportManager;

impl TeleportManager {
    /// Validate if a name can be used as a teleport savepoint
    pub fn validate_name(name: &str) -> bool {
        let name_lower = name.to_lowercase();
        !RESERVED_NAMES.contains(&name_lower.as_str()) && !name_lower.is_empty()
    }

    /// Get all teleports from config (always global only)
    pub fn get_all() -> HashMap<String, PathBuf> {
        Config::global().read().unwrap().teleports.clone()
    }

    pub fn get_path(name: &str) -> Option<PathBuf> {
        let teleports = Self::get_all();
        teleports.get(&name.to_lowercase()).cloned()
    }
    
    pub fn show_current_tree(nav: &Navigator) {
        show_tree(nav, Some(1), false, false, false, false, false);
    }

    /// Add or update a teleport savepoint (always global)
    pub fn add(name: &str, path: PathBuf) -> Result<()> {
        let name_lower = name.to_lowercase();
        
        if !Self::validate_name(&name_lower) {
            print_error(&format!("Invalid name: '{}' is reserved or empty", name));
            return Ok(());
        }
        
        let mut cfg = Config::global().write().unwrap();
        let old_path = cfg.teleports.insert(name_lower.clone(), path.clone());
        cfg.save();
        
        if let Some(old) = old_path {
            print_success(&format!("Savepoint '{}' updated: {} -> {}", 
                name_lower, old.display(), path.display()));
        } else {
            print_success(&format!("Savepoint '{}' created -> {}", name_lower, path.display()));
        }
        
        Ok(())
    }

    /// Add current directory as teleport
    pub fn add_current(nav: &Navigator, name: &str) -> Result<()> {
        let path = nav.current_path().to_path_buf();
        Self::add(name, path)
    }

    /// Jump to a teleport savepoint by name
    pub fn jump_by_name(nav: &mut Navigator, name: &str) -> Result<()> {
        let name_lower = name.to_lowercase();
        let teleports = Self::get_all();
        
        if let Some(path) = teleports.get(&name_lower) {
            nav.go_to(path)?;
            clear_screen();
            print_success(&format!("Teleported to '{}' -> {}", name_lower, nav.display_path()));
            Self::show_current_tree(nav);
            Ok(())
        } else {
            print_error(&format!("Savepoint not found: '{}'", name));
            Ok(())
        }
    }

    /// Jump to a teleport savepoint by index (1-based)
    pub fn jump_by_index(nav: &mut Navigator, index: usize) -> Result<()> {
        let teleports = Self::get_all();
        let mut teleports_vec: Vec<(String, PathBuf)> = teleports.into_iter().collect();
        teleports_vec.sort_by(|a, b| a.0.cmp(&b.0));
        
        if index == 0 || index > teleports_vec.len() {
            print_error(&format!("Invalid index: {}. Use 1-{}", index, teleports_vec.len()));
            return Ok(());
        }
        
        let (name, path) = &teleports_vec[index - 1];
        nav.go_to(path)?;
        clear_screen();
        print_success(&format!("Teleported to '{}' -> {}", name, nav.display_path()));
        Self::show_current_tree(nav);
        Ok(())
    }

    /// List all teleport savepoints (global only)
    pub fn list() -> Result<()> {
        let teleports = Self::get_all();
        
        if teleports.is_empty() {
            print_info("No savepoints yet. Use 'tp add <name>' to create one.");
            return Ok(());
        }
        
        println!();
        println!("{}", "==================================================".cyan());
        println!("{}", "📌 Your Teleport Savepoints (global)".cyan().bold());
        println!("{}", "==================================================".cyan());
        
        let mut sorted: Vec<(String, PathBuf)> = teleports.into_iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (i, (name, path)) in sorted.iter().enumerate() {
            println!("  {}. {} -> {}", 
                (i + 1).to_string().yellow(), 
                name.blue(), 
                path.display().to_string().dimmed());
        }
        println!();
        
        Ok(())
    }

    /// Interactive menu (global only)
    pub fn interactive_menu(nav: &mut Navigator) -> Result<()> {
        let teleports = Self::get_all();
        
        if teleports.is_empty() {
            print_info("No savepoints yet. Use 'tp add <name>' to create one.");
            return Ok(());
        }
        
        println!();
        println!("{}", "==================================================".cyan());
        println!("{}", "📌 Teleport - Your Savepoints (global)".cyan().bold());
        println!("{}", "==================================================".cyan());
        
        let mut sorted: Vec<(String, PathBuf)> = teleports.into_iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (i, (name, path)) in sorted.iter().enumerate() {
            println!("  {}. {} -> {}", 
                (i + 1).to_string().yellow(), 
                name.blue(), 
                path.display().to_string().dimmed());
        }
        
        println!("  {}", "0. Cancel".red());
        println!();
        print!("{} ", format!("Enter number to teleport (1-{}) or 0: ", sorted.len()).green());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "0" || input.is_empty() {
            println!("{}", "Teleport cancelled.".dimmed());
            return Ok(());
        }
        
        match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= sorted.len() => {
                let (name, path) = &sorted[num - 1];
                nav.go_to(path)?;
                clear_screen();
                print_success(&format!("Teleported to '{}' -> {}", name, nav.display_path()));
                Self::show_current_tree(nav);
            }
            Ok(_) => {
                print_error(&format!("Invalid number: {}. Choose 1-{}", input, sorted.len()));
            }
            Err(_) => {
                print_error(&format!("Invalid input: {}. Enter a number.", input));
            }
        }
        
        Ok(())
    }

    /// Remove a teleport savepoint by name
    pub fn remove_by_name(name: &str) -> Result<()> {
        let name_lower = name.to_lowercase();
        let mut cfg = Config::global().write().unwrap();
        
        if cfg.teleports.remove(&name_lower).is_some() {
            cfg.save();
            print_success(&format!("Removed savepoint: '{}'", name_lower));
        } else {
            print_error(&format!("Savepoint not found: '{}'", name));
        }
        
        Ok(())
    }

    /// Remove a teleport savepoint by index (1-based)
    pub fn remove_by_index(index: usize) -> Result<()> {
        let teleports = Self::get_all();
        let mut teleports_vec: Vec<(String, PathBuf)> = teleports.into_iter().collect();
        teleports_vec.sort_by(|a, b| a.0.cmp(&b.0));
        
        if index == 0 || index > teleports_vec.len() {
            print_error(&format!("Invalid index: {}. Use 1-{}", index, teleports_vec.len()));
            return Ok(());
        }
        
        let (name, _) = &teleports_vec[index - 1];
        Self::remove_by_name(name)
    }

    /// Rename a teleport savepoint
    pub fn rename(old_name: &str, new_name: &str) -> Result<()> {
        let old_lower = old_name.to_lowercase();
        let new_lower = new_name.to_lowercase();
        
        if !Self::validate_name(&new_lower) {
            print_error(&format!("Invalid name: '{}' is reserved or empty", new_name));
            return Ok(());
        }
        
        let mut cfg = Config::global().write().unwrap();
        
        if !cfg.teleports.contains_key(&old_lower) {
            print_error(&format!("Savepoint not found: '{}'", old_name));
            return Ok(());
        }
        
        if cfg.teleports.contains_key(&new_lower) {
            print_error(&format!("Cannot rename: '{}' already exists as a savepoint", new_name));
            println!("{}", "Use 'tp list' to see existing savepoints.".dimmed());
            return Ok(());
        }
        
        let path = cfg.teleports.remove(&old_lower).unwrap();
        cfg.teleports.insert(new_lower.clone(), path.clone());
        cfg.save();
        
        print_success(&format!("Renamed savepoint '{}' -> '{}' (-> {})", 
            old_lower, new_lower, path.display()));
        
        Ok(())
    }

    /// Clear all teleport savepoints (with confirmation)
    pub fn clear_all() -> Result<()> {
        let teleports = Self::get_all();
        
        if teleports.is_empty() {
            print_info("No savepoints to clear.");
            return Ok(());
        }
        
        println!();
        print!("{} ", "⚠ Are you sure you want to delete ALL savepoints? (y/N): ".yellow());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();
        
        if input == "y" || input == "yes" {
            let mut cfg = Config::global().write().unwrap();
            cfg.teleports.clear();
            cfg.save();
            print_success("All savepoints cleared.");
        } else {
            println!("{}", "Clear cancelled.".dimmed());
        }
        
        Ok(())
    }

    /// Get count of savepoints
    pub fn count() -> usize {
        Self::get_all().len()
    }
}

/// Handle @name shortcut
pub fn handle_teleport_shortcut(nav: &mut Navigator, name: &str) -> Result<bool> {
    let name = name.trim();
    if name.is_empty() {
        return Ok(false);
    }
    
    let teleports = TeleportManager::get_all();
    if teleports.contains_key(&name.to_lowercase()) {
        TeleportManager::jump_by_name(nav, name)?;
        Ok(true)
    } else {
        Ok(false)
    }
}