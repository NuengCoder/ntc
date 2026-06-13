// src/utils/teleport.rs
use crate::config::Config;
use crate::navigator::{Navigator, clear_screen};
use crate::output::{print_error, print_success, print_info};
use crate::session::SessionState;
use crate::shell::show_tree;
use anyhow::Result;
use colored::*;
use std::collections::VecDeque;
use std::path::{PathBuf};
use std::io::{self, Write};

const MAX_STACK_SIZE: usize = 50;

/// Reserved command names that cannot be used as teleport names
const RESERVED_NAMES: &[&str] = &["add", "jump", "list", "rm", "cls", "help", "tp", "back", "history", "clear"];

/// Teleport manager for handling savepoints
pub struct TeleportManager;

impl TeleportManager {
    /// Push current directory onto navigation stack before teleporting
    fn push_to_stack(path: PathBuf) {
        let mut session = SessionState::write_global();
        let mut deque: VecDeque<PathBuf> = session.nav_stack.drain(..).collect();
        // Don't push duplicate consecutive entries
        if deque.back() != Some(&path) {
            deque.push_back(path);
            while deque.len() > MAX_STACK_SIZE {
                deque.pop_front();
            }
        }
        session.nav_stack = deque.into();
        drop(session);
        SessionState::save_global();
    }

    /// Pop from navigation stack to get previous directory
    fn pop_from_stack() -> Option<PathBuf> {
        let mut session = SessionState::write_global();
        let mut deque: VecDeque<PathBuf> = session.nav_stack.drain(..).collect();
        let popped = deque.pop_back();
        session.nav_stack = deque.into();
        drop(session);
        SessionState::save_global();
        popped
    }

    /// Get the current stack without modifying it
    pub fn get_stack() -> Vec<PathBuf> {
        SessionState::read_global().nav_stack.clone()
    }

    /// Clear the navigation stack
    pub fn clear_stack() {
        let mut session = SessionState::write_global();
        session.nav_stack.clear();
        drop(session);
        SessionState::save_global();
        print_success("Teleport history cleared.");
    }

    /// Validate if a name can be used as a teleport savepoint
    pub fn validate_name(name: &str) -> bool {
        let name_lower = name.to_lowercase();
        !RESERVED_NAMES.contains(&name_lower.as_str()) && !name_lower.is_empty()
    }

    /// Get all teleports from config (always global only)
    pub fn get_all() -> std::collections::HashMap<String, PathBuf> {
        Config::read_global().teleports.clone()
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
        
        let mut cfg = Config::write_global();
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

    /// Jump to a teleport savepoint by name (pushes current location to stack)
    pub fn jump_by_name(nav: &mut Navigator, name: &str) -> Result<()> {
        let name_lower = name.to_lowercase();
        let teleports = Self::get_all();
        
        if let Some(path) = teleports.get(&name_lower) {
            // Push current location to stack before teleporting
            Self::push_to_stack(nav.current_path().to_path_buf());
            
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
        
        // Push current location to stack before teleporting
        Self::push_to_stack(nav.current_path().to_path_buf());
        
        nav.go_to(path)?;
        clear_screen();
        print_success(&format!("Teleported to '{}' -> {}", name, nav.display_path()));
        Self::show_current_tree(nav);
        Ok(())
    }

    /// Teleport back to previous location (undo last teleport)
    pub fn teleport_back(nav: &mut Navigator) -> Result<()> {
        if let Some(prev_path) = Self::pop_from_stack() {
            if prev_path.exists() {
                nav.go_to(&prev_path)?;
                clear_screen();
                print_success(&format!("Teleported back to: {}", nav.display_path()));
                
                // Push the current location onto stack again? No, that would create a loop.
                // The user can use `tpb` again to go back further in history.
                Self::show_current_tree(nav);
            } else {
                print_error(&format!("Previous location no longer exists: {}", prev_path.display()));
                print_info("Removing from history. Use 'tp history' to see remaining entries.");
                // Try again with next entry
                Self::teleport_back(nav)?;
            }
        } else {
            print_info("No teleport history. Use 'tp jump <name>' first to create history.");
        }
        Ok(())
    }

    /// Show teleport history (navigation stack)
    pub fn show_history() -> Result<()> {
        let stack = Self::get_stack();
        
        if stack.is_empty() {
            print_info("No teleport history. Use 'tp jump <name>' to start building history.");
            return Ok(());
        }
        
        println!();
        println!("{}", "==================================================".cyan());
        println!("{}", "📜 Teleport History (most recent last)".cyan().bold());
        println!("{}", "==================================================".cyan());
        
        for (i, path) in stack.iter().enumerate() {
            let marker = if i == stack.len() - 1 { " ← current?" } else { "" };
            println!("  {}. {}{}", 
                (i + 1).to_string().yellow(), 
                path.display().to_string().dimmed(),
                marker.dimmed());
        }
        println!();
        println!("{}", "Use 'tpb' to go back to the previous location.".green());
        println!("{}", "Use 'tp clear' to clear history.".dimmed());
        
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
        println!("{}", "Use 'tp jump <name>' to teleport.".green());
        println!("{}", "Use 'tpb' to return to previous location.".dimmed());
        
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
        println!("  {}", "b. Teleport Back".cyan());
        println!("  {}", "h. Show History".cyan());
        println!();
        print!("{} ", format!("Enter number to teleport (1-{}) or b/h/0: ", sorted.len()).green());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "0" || input.is_empty() {
            println!("{}", "Teleport cancelled.".dimmed());
            return Ok(());
        }
        
        if input == "b" || input == "B" {
            return Self::teleport_back(nav);
        }
        
        if input == "h" || input == "H" {
            return Self::show_history();
        }
        
        match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= sorted.len() => {
                let (name, path) = &sorted[num - 1];
                Self::push_to_stack(nav.current_path().to_path_buf());
                nav.go_to(path)?;
                clear_screen();
                print_success(&format!("Teleported to '{}' -> {}", name, nav.display_path()));
                Self::show_current_tree(nav);
            }
            Ok(_) => {
                print_error(&format!("Invalid number: {}. Choose 1-{}", input, sorted.len()));
            }
            Err(_) => {
                print_error(&format!("Invalid input: {}. Enter a number, 'b', or 'h'.", input));
            }
        }
        
        Ok(())
    }

    /// Remove a teleport savepoint by name
    pub fn remove_by_name(name: &str) -> Result<()> {
        let name_lower = name.to_lowercase();
        let mut cfg = Config::write_global();
        
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
        
        let mut cfg = Config::write_global();
        
        if !cfg.teleports.contains_key(&old_lower) {
            print_error(&format!("Savepoint not found: '{}'", old_name));
            return Ok(());
        }
        
        if cfg.teleports.contains_key(&new_lower) {
            print_error(&format!("Cannot rename: '{}' already exists as a savepoint", new_name));
            println!("{}", "Use 'tp list' to see existing savepoints.".dimmed());
            return Ok(());
        }
        
        let Some(path) = cfg.teleports.remove(&old_lower) else {
            print_error(&format!("Savepoint not found: '{}'", old_name));
            return Ok(());
        };
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
            let mut cfg = Config::write_global();
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