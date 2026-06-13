use crate::config::Config;
use crate::explorer::{human_readable_size, calculate_dir_size, calculate_total_size};
use crate::filetype::is_supported_format;
use crate::navigator::{clear_screen, Navigator};
use crate::output::{cat_file, print_error, print_success, print_warning, print_info};
use crate::search;
use crate::session::SessionState;
use crate::shell::helpers::{show_tree, run_with_spinner};
use crate::shell::help::{print_interactive_help, print_tp_help};
use crate::teleport::TeleportManager;
use crate::watcher;
use super::{execute_system_command, parse_search_args};

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::sync::Arc;

pub fn cmd_clear(_args: &str, nav: &mut Navigator) -> Result<bool> {
    clear_screen();
    println!();
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        let version = env!("CARGO_PKG_VERSION");
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║{}║", format!("        Welcome to ntc {} - Navigate, Toolkit, Center          ", version).color(theme.shell.help_header.to_colored()).bold());
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
    #[cfg(target_os = "android")]
    {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║{}║", format!("        Welcome to ntc {} - Navigate, Toolkit, Center          ", env!("CARGO_PKG_VERSION")));
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
    println!("{}", "Type 'help' for available commands, 'exit' to quit.".dimmed());
    show_tree(nav, Some(1), false, false, false, false, false);
    Ok(false)
}

pub fn cmd_dino(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    crate::game::run()?;
    Ok(false)
}

pub fn cmd_math(args: &str, _nav: &mut Navigator) -> Result<bool> {
    crate::math::run(args)?;
    Ok(false)
}

pub fn cmd_version(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        println!("ntc {}", env!("CARGO_PKG_VERSION").color(theme.shell.success.to_colored()).bold());
    }
    #[cfg(target_os = "android")]
    {
        println!("ntc {}", env!("CARGO_PKG_VERSION"));
    }
    Ok(false)
}

pub fn cmd_help(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    print_interactive_help();
    Ok(false)
}

pub fn cmd_tutorial(_args: &str, nav: &mut Navigator) -> Result<bool> {
    crate::tutorial::run_tutorial(nav)?;
    Ok(false)
}

pub fn cmd_exit(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    Ok(true)
}

pub fn cmd_ne(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let raw_path = if let Some(init_path) = args.strip_prefix("--init ") {
        let p = std::path::PathBuf::from(init_path.trim());
        let _ = crate::editor::init_file(&p);
        eprintln!("Created template: {}", p.display());
        p
    } else if args.is_empty() || args == "." {
        let cwd = nav.current_path();
        let scratch = cwd.join(".scratch");
        if !scratch.exists() {
            let _ = std::fs::write(&scratch, "");
        }
        print_info(&format!("Opened scratch buffer in {}", nav.current_path().display()));
        scratch
    } else {
        std::path::PathBuf::from(args.trim())
    };
    let path = if raw_path.is_dir() {
        let scratch = raw_path.join(".scratch");
        if !scratch.exists() {
            let _ = std::fs::write(&scratch, "");
        }
        scratch
    } else {
        raw_path
    };
    let restored = SessionState::read_global().editor_session.clone();
    match crate::editor::edit_file_with_session(&path, restored) {
        Ok((_, captured)) => {
            SessionState::write_global().editor_session = captured;
            SessionState::save_global();
        }
        Err(e) => print_error(&format!("Editor error: {}", e)),
    }
    Ok(false)
}

pub fn cmd_size(args: &str, nav: &mut Navigator) -> Result<bool> {
    let care = args.trim() == "--care";
    let total = if care {
        run_with_spinner("Calculating directory size...", || {
            calculate_total_size(nav.current_path())
        })
    } else {
        run_with_spinner("Calculating directory size...", || {
            calculate_dir_size(nav.current_path())
        })
    };
    let bytes_str = format!("{}", total);
    let human_str = human_readable_size(total);
    let max_label = 8;
    let max_value = bytes_str.len().max(human_str.len());
    let w = max_label + max_value + 10;

    println!();
    println!("┌{}┐", "─".repeat(w));
    println!("│{:^w$}│", "Current Directory Size".cyan().bold(), w = w);
    println!("├{}┤", "─".repeat(w));
    println!("│ Bytes: {:<w$} │", bytes_str.yellow(), w = w - 9);
    println!("│ Human: {:<w$} │", human_str.green().bold(), w = w - 9);
    println!("└{}┘", "─".repeat(w));
    Ok(false)
}

pub fn cmd_tp(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        TeleportManager::interactive_menu(nav)?;
    } else {
        let tp_parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = tp_parts[0].to_lowercase();
        let subargs = tp_parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "add" => {
                if subargs.is_empty() {
                    println!("Usage: tp add <name> [path]");
                } else {
                    let add_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    let name = add_parts[0];
                    if add_parts.len() > 1 {
                        let path = std::path::Path::new(add_parts[1]);
                        TeleportManager::add(name, path.to_path_buf())?;
                    } else {
                        TeleportManager::add_current(nav, name)?;
                    }
                }
            }
            "jump" | "to" => {
                if subargs.is_empty() {
                    println!("Usage: tp jump <name|number>");
                } else if let Ok(num) = subargs.parse::<usize>() {
                    TeleportManager::jump_by_index(nav, num)?;
                } else {
                    TeleportManager::jump_by_name(nav, subargs)?;
                }
            }
            "info" => {
                if subargs.is_empty() {
                    print_error("Usage: tp info <name>");
                    println!("Example: tp info my_project");
                } else {
                    let teleports = TeleportManager::get_all();
                    let name_lower = subargs.to_lowercase();
                    if let Some(path) = teleports.get(&name_lower) {
                        println!();
                        println!("{}", "==================================================".cyan());
                        println!("{}", "📌 Teleport Savepoint Info".cyan().bold());
                        println!("{}", "==================================================".cyan());
                        println!("  {}: {}", "Name".yellow(), name_lower.blue().bold());
                        println!("  {}: {}", "Path".yellow(), path.display().to_string().dimmed());
                        let exists = if path.exists() { "yes".green() } else { "no".red() };
                        println!("  {}: {}", "Exists".yellow(), exists);
                        if let Ok(canonical) = std::fs::canonicalize(path) {
                            println!("  {}: {}", "Canonical".yellow(), canonical.display().to_string().dimmed());
                        }
                        let total = teleports.len();
                        let sorted: Vec<&String> = teleports.keys().collect();
                        if let Some(pos) = sorted.iter().position(|k| *k == &name_lower) {
                            println!("  {}: {} / {}", "Index".yellow(), (pos + 1).to_string().yellow(), total.to_string().yellow());
                        }
                        println!("  {}: {}", "Source".yellow(), "global".green());
                        println!();
                    } else {
                        print_error(&format!("Savepoint '{}' not found", subargs));
                    }
                }
            }
            "list" | "ls" => {
                TeleportManager::list()?;
            }
            "rm" => {
                if subargs.is_empty() {
                    println!("Usage: tp rm <name|number>[, <name|number>, ...]");
                } else {
                    for arg in subargs.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                        if let Ok(num) = arg.parse::<usize>() {
                            TeleportManager::remove_by_index(num)?;
                        } else {
                            TeleportManager::remove_by_name(arg)?;
                        }
                    }
                }
            }
            "rnm" | "rename" => {
                if subargs.is_empty() {
                    println!("Usage: tp rnm <old_name> to <new_name>");
                } else {
                    let parts: Vec<&str> = subargs.splitn(4, ' ').collect();
                    if parts.len() >= 3 && parts[1].to_lowercase() == "to" {
                        TeleportManager::rename(parts[0], parts[2])?;
                    } else {
                        print_error("Invalid format. Use: tp rnm <old> to <new>");
                    }
                }
            }
            "cls" => {
                TeleportManager::clear_all()?;
            }
            "help" => {
                print_tp_help();
            }
            _ => {
                print_error(&format!("Unknown tp subcommand: {}", subcmd));
                println!("Type 'tp help' for usage.");
            }
        }
    }
    Ok(false)
}

pub fn cmd_gs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: gs <pattern> [-d <depth>]");
        println!("  gs main             # search file contents using config depth");
        println!("  gs main -d 5        # search up to 5 levels deep");
        println!("  gs fn main          # search for 'fn' in source code");
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_content(nav.current_path(), &pattern, max_depth);
    let output  = crate::search::format_content_results(&results, &pattern, max_depth);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_fs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: fs <pattern> [-d <depth>]");
        println!("  fs main.c          # search using config depth");
        println!("  fs main.c -d 5     # search up to 5 levels deep");
        println!("  fs main            # partial match: finds main_helper.c etc.");
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_files(nav.current_path(), &pattern, max_depth);
    let output  = crate::search::format_search_results(&results, &pattern, max_depth, true);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_ds(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: ds <pattern> [-d <depth>]");
        println!("  ds src             # search using config depth");
        println!("  ds src -d 3        # search up to 3 levels deep");
        println!("  ds test            # partial match: finds test_utils/ etc.");
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_directories(nav.current_path(), &pattern, max_depth);
    let output  = crate::search::format_search_results(&results, &pattern, max_depth, false);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_fsc(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: fsc <pattern> [-d <depth>]");
        println!("  fsc main.rs        # search then pick file to display");
        println!("  fsc main.rs -d 5   # search up to 5 levels deep");
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_files(nav.current_path(), &pattern, max_depth);
    if results.is_empty() {
        let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
        print!("{}", output);
        return Ok(false);
    }
    loop {
        println!();
        println!("{}", format!("🔍 Found {} file(s) for \"{}\":", results.len(), pattern).cyan().bold());
        for (i, r) in results.iter().enumerate() {
            let path_str = r.full_path.to_string_lossy();
            println!("  {}. {}", (i + 1).to_string().yellow(), path_str.dimmed());
        }
        println!("  {}", "0. Done".red());
        println!();
        print!("{} ", "Select file to display: ".green());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "0" || input.is_empty() {
            println!("{}", "Done.".dimmed());
            return Ok(false);
        }
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= results.len() {
                let target = &results[n - 1].full_path;
                if target.is_file() {
                    if is_supported_format(target) {
                        let show_lines = Config::global_get_show_line_numbers();
                        cat_file(target, show_lines)?;
                        println!();
                        print!("{} ", "Copy to clipboard? (y/N): ".yellow());
                        io::stdout().flush()?;
                        let mut copy_choice = String::new();
                        io::stdin().read_line(&mut copy_choice)?;
                        let copy_choice = copy_choice.trim().to_lowercase();
                        if copy_choice == "y" || copy_choice == "yes" {
                            let content = crate::output::cat_file_with_line_numbers(target, show_lines)?;
                            crate::output::copy_to_clipboard(&content, "TXT")?;
                            print_success(&format!("Copied '{}' to clipboard!", target.display()));
                        }
                    } else {
                        print_warning(&format!("Skipped (not support format): {}", target.display()));
                    }
                } else {
                    print_warning("Selected path is not a file.");
                }
            } else {
                print_error("Invalid selection.");
            }
        } else {
            print_error("Invalid input.");
        }
        println!();
    }
}

pub fn cmd_locate(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: locate <pattern> [-d <depth>]");
        println!("  locate main.c       # search files and dirs (config depth)");
        println!("  locate main.c -d 5  # search up to 5 levels deep");
        println!("  locate main         # partial match: finds main.c, main_helper/ etc.");
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = search::search_all(nav.current_path(), &pattern, max_depth);
    if results.is_empty() {
        println!("{}", format!("\n🔍 No results for \"{}\"\n", pattern).dimmed());
    } else {
        let has_exact = results.iter().any(|r| r.match_kind == search::MatchKind::Exact);
        let has_partial = results.iter().any(|r| r.match_kind == search::MatchKind::Partial);
        let label = if has_exact { "Exact matches" } else if has_partial { "Partial matches" } else { "Did you mean?" };
        println!();
        println!("{}", format!("🔍 {} for \"{}\" ({} result(s)):", label, pattern, results.len()).cyan().bold());
        for (i, r) in results.iter().enumerate() {
            let icon = if r.full_path.is_dir() { "📁" } else { "📄" };
            if r.match_kind == search::MatchKind::Fuzzy {
                println!("  {}. {} {} ({:.0}% match)", (i + 1).to_string().yellow(), icon, r.full_path.to_string_lossy().dimmed(), r.score * 100.0);
            } else {
                println!("  {}. {} {}", (i + 1).to_string().yellow(), icon, r.full_path.to_string_lossy().dimmed());
            }
        }
        println!();
    }
    Ok(false)
}

pub fn cmd_tpb(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        TeleportManager::teleport_back(nav)?;
    } else {
        let args_lower = args.to_lowercase();
        match args_lower.as_str() {
            "history" | "hist" | "h" => {
                TeleportManager::show_history()?;
            }
            "clear" | "cls" => {
                TeleportManager::clear_stack();
            }
            _ => {
                print_error(&format!("Unknown tpb option: {}", args));
                println!("Usage:");
                println!("  tpb                 - Teleport back to previous location");
                println!("  tpb history         - Show teleport history");
                println!("  tpb clear           - Clear teleport history");
            }
        }
    }
    Ok(false)
}

pub fn cmd_mkf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error("mkf: missing operand");
    } else {
        let path = std::path::Path::new(args);
        if !path.exists() {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    let _ = std::fs::create_dir_all(parent);
                }
            }
            match std::fs::write(path, "") {
                Ok(()) => print_success(&format!("Created file: {}", args)),
                Err(e) => print_error(&format!("mkf: {}", e)),
            }
        } else {
            print_warning(&format!("Already exists: {}", args));
        }
    }
    Ok(false)
}

pub fn cmd_mkd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error("mkd: missing operand");
    } else if let Err(e) = std::fs::create_dir_all(std::path::Path::new(args)) {
        print_error(&format!("mkd: {}", e));
    } else {
        print_success(&format!("Created directory: {}", args));
    }
    Ok(false)
}

pub fn cmd_rmd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error("rmd: missing operand");
    } else {
        let path = std::path::Path::new(args);
        if !path.exists() {
            print_error(&format!("rmd: '{}' does not exist", args));
        } else if !path.is_dir() {
            print_error(&format!("rmd: '{}' is not a directory", args));
        } else if let Err(e) = std::fs::remove_dir_all(path) {
            print_error(&format!("rmd: {}", e));
        } else {
            print_success(&format!("Removed directory: {}", args));
        }
    }
    Ok(false)
}

pub fn cmd_rmf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error("rmf: missing operand");
    } else {
        let path = std::path::Path::new(args);
        if !path.exists() {
            print_error(&format!("rmf: '{}' does not exist", args));
        } else if path.is_dir() {
            print_error(&format!("rmf: '{}' is a directory (use rmd instead)", args));
        } else {
            match std::fs::remove_file(path) {
                Ok(()) => print_success(&format!("Removed: {}", args)),
                Err(e) => print_error(&format!("rmf: {}", e)),
            }
        }
    }
    Ok(false)
}

pub fn cmd_ui(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    println!("The modern UI was removed. Only the classic shell is available.");
    Ok(false)
}

// Add this function to misc.rs

pub fn cmd_theme(args: &str, _nav: &mut Navigator) -> Result<bool> {
    use crate::utils::theme::ThemeManager;
    
    if args.is_empty() {
        // Interactive menu
        loop {
            let current = ThemeManager::current();
            let themes = ThemeManager::list_themes();
            
            println!();
            println!("╔══════════════════════════════════════════════════════════════════╗");
            println!("║{:^65}║", "🎨 ntc Theme Manager".cyan().bold());
            println!("╠══════════════════════════════════════════════════════════════════╣");
            println!("║  Current theme: {:48} ║", current.name.green().bold());
            println!("╠══════════════════════════════════════════════════════════════════╣");
            
            for (i, name) in themes.iter().enumerate() {
                let marker = if name == &current.name { " ✓" } else { "  " };
                println!("║  {}. {}{:<53} ", 
                    (i + 1).to_string().yellow(),
                    name,
                    marker
                );
            }
            println!("╠══════════════════════════════════════════════════════════════════╣");
            println!("║  Commands:                                                       ║");
            println!("║    n <number>         - Switch to theme by number                ║");
            println!("║    add                - Add new theme (opens editor)             ║");
            println!("║    rm <name>          - Remove theme                             ║");
            println!("║    edit <name>        - Edit theme in editor                     ║");
            println!("║    info <name>        - Show theme info                          ║");
            println!("║    export <name>      - Export theme to file                     ║");
            println!("║    import <file>      - Import theme from file                   ║");
            println!("║    rnm <old> to <new> - Rename theme                             ║");
            println!("║    0                  - Exit                                     ║");
            println!("╚══════════════════════════════════════════════════════════════════╝");
            println!();
            print!("{} ", "theme>".green().bold());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input == "0" || input.is_empty() {
                println!("{}", "Exiting theme manager.".dimmed());
                break;
            }
            
            // Drop the read guard on current before set_theme, which needs a write lock
            drop(current);
            
            // Parse number
            if let Ok(num) = input.parse::<usize>() {
                if num >= 1 && num <= themes.len() {
                    let theme_name = &themes[num - 1];
                    if ThemeManager::set_theme(theme_name) {
                        print_success(&format!("Switched to theme: {}", theme_name));
                    } else {
                        print_error(&format!("Failed to switch to theme: {}", theme_name));
                    }
                } else {
                    print_error(&format!("Invalid number: {}. Use 1-{}", num, themes.len()));
                }
                continue;
            }
            
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0].to_lowercase().as_str() {
                "a" | "add" => {
                    print!("{} ", "Theme name: ".cyan());
                    io::stdout().flush()?;
                    let mut name = String::new();
                    io::stdin().read_line(&mut name)?;
                    let name = name.trim();
                    
                    if name.is_empty() {
                        print_error("Theme name cannot be empty");
                    } else if let Err(e) = ThemeManager::create_theme(name) {
                        print_error(&format!("Failed to create theme: {}", e));
                    } else {
                        print_success(&format!("Created theme '{}' (opened in editor)", name));
                        print_info("Edit the file, save, then run 'theme reload' to apply changes");
                    }
                }
                "r" | "rm" | "remove" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme rm <name>");
                    } else {
                        let name = parts[1];
                        if let Err(e) = ThemeManager::remove_theme(name) {
                            print_error(&format!("Failed to remove theme: {}", e));
                        } else {
                            print_success(&format!("Removed theme: {}", name));
                        }
                    }
                }
                "e" | "edit" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme edit <name>");
                    } else {
                        let name = parts[1];
                        if ThemeManager::theme_exists(name) {
                            let themes_dir = dirs::config_dir()
                                .map(|d| d.join("ntc").join("themes"));
                            if let Some(dir) = themes_dir {
                                let theme_path = dir.join(format!("{}.ntc_theme", name));
                                let _ = crate::editor::edit_file(&theme_path);
                                print_info(&format!("Edit '{}', save, then run 'theme reload'", name));
                            }
                        } else {
                            print_error(&format!("Theme '{}' not found", name));
                        }
                    }
                }
                "i" | "info" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme info <name>");
                    } else {
                        let name = parts[1];
                        if let Some(theme) = ThemeManager::get_theme_info(name) {
                            println!();
                            println!("{}", "════════════════════════════════════════════════════════════".cyan());
                            println!("{}", format!("🎨 Theme: {}", theme.name).cyan().bold());
                            println!("{}", "════════════════════════════════════════════════════════════".cyan());
                            if let Some(author) = theme.author {
                                println!("  {}: {}", "Author".yellow(), author);
                            }
                            if let Some(desc) = theme.description {
                                println!("  {}: {}", "Description".yellow(), desc);
                            }
                            println!("  {}: {}", "Syntax colors".yellow(), "✓ defined".green());
                            println!("  {}: {}", "Shell colors".yellow(), "✓ defined".green());
                            println!("  {}: {}", "Editor colors".yellow(), "✓ defined".green());
                            println!();
                        } else {
                            print_error(&format!("Theme '{}' not found", name));
                        }
                    }
                }
                "x" | "export" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme export <name> [output_file]");
                    } else {
                        let name = parts[1];
                        let output = if parts.len() >= 3 {
                            std::path::PathBuf::from(parts[2])
                        } else {
                            let current_dir = std::env::current_dir().unwrap_or_default();
                            current_dir.join(format!("{}.ntc_theme", name))
                        };
                        
                        if let Err(e) = ThemeManager::export_theme(name, &output) {
                            print_error(&format!("Failed to export theme: {}", e));
                        } else {
                            print_success(&format!("Exported theme '{}' to: {}", name, output.display()));
                        }
                    }
                }
                "m" | "import" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme import <file.ntc_theme>");
                    } else {
                        let path = std::path::Path::new(parts[1]);
                        if !path.exists() {
                            print_error(&format!("File not found: {}", path.display()));
                        } else if let Err(e) = ThemeManager::import_theme(path) {
                            print_error(&format!("Failed to import theme: {}", e));
                        } else {
                            print_success(&format!("Imported theme from: {}", path.display()));
                            print_info("Run 'theme list' to see available themes");
                        }
                    }
                }
                "rnm" | "rename" => {
                    if parts.len() < 4 || parts[2].to_lowercase() != "to" {
                        print_error("Usage: theme rnm <old_name> to <new_name>");
                        println!("Example: theme rnm mytheme to my_new_theme");
                    } else {
                        let old_name = parts[1];
                        let new_name = parts[3];
                        if let Err(e) = ThemeManager::rename_theme(old_name, new_name) {
                            print_error(&format!("Failed to rename theme: {}", e));
                        } else {
                            print_success(&format!("Renamed theme '{}' -> '{}'", old_name, new_name));
                        }
                    }
                }
                "reload" => {
                    ThemeManager::reload_themes();
                    print_success("Themes reloaded from disk");
                }
                _ => {
                    print_error(&format!("Unknown command: {}", parts[0]));
                    println!("Type 'theme' for interactive menu");
                }
            }
        }
        Ok(false)
    } else {
        // Direct command mode (non-interactive)
        let parts: Vec<&str> = args.split_whitespace().collect();
        match parts[0].to_lowercase().as_str() {
            "list" | "ls" => {
                let current = ThemeManager::current();
                for name in ThemeManager::list_themes() {
                    if name == current.name {
                        println!("  ✓ {}", name.green());
                    } else {
                        println!("    {}", name);
                    }
                }
            }
            "current" => {
                let current = ThemeManager::current();
                println!("Current theme: {}", current.name.green().bold());
                if let Some(author) = &current.author {
                    println!("Author: {}", author);
                }
                if let Some(desc) = &current.description {
                    println!("Description: {}", desc);
                }
            }
            "reload" => {
                ThemeManager::reload_themes();
                print_success("Themes reloaded from disk");
            }
            "add" => {
                if parts.len() < 2 {
                    print_error("Usage: theme add <name>");
                } else if let Err(e) = ThemeManager::create_theme(parts[1]) {
                    print_error(&format!("Failed to create theme: {}", e));
                } else {
                    print_success(&format!("Created theme '{}' (opened in editor)", parts[1]));
                }
            }
            "rm" => {
                if parts.len() < 2 {
                    print_error("Usage: theme rm <name>");
                } else if let Err(e) = ThemeManager::remove_theme(parts[1]) {
                    print_error(&format!("Failed to remove theme: {}", e));
                } else {
                    print_success(&format!("Removed theme: {}", parts[1]));
                }
            }
            "edit" => {
                if parts.len() < 2 {
                    print_error("Usage: theme edit <name>");
                } else if ThemeManager::theme_exists(parts[1]) {
                    let themes_dir = dirs::config_dir()
                        .map(|d| d.join("ntc").join("themes"));
                    if let Some(dir) = themes_dir {
                        let theme_path = dir.join(format!("{}.ntc_theme", parts[1]));
                        let _ = crate::editor::edit_file(&theme_path);
                    }
                } else {
                    print_error(&format!("Theme '{}' not found", parts[1]));
                }
            }
            "info" => {
                if parts.len() < 2 {
                    print_error("Usage: theme info <name>");
                } else if let Some(theme) = ThemeManager::get_theme_info(parts[1]) {
                    println!("Theme: {}", theme.name.cyan().bold());
                    if let Some(author) = theme.author {
                        println!("Author: {}", author);
                    }
                    if let Some(desc) = theme.description {
                        println!("Description: {}", desc);
                    }
                } else {
                    print_error(&format!("Theme '{}' not found", parts[1]));
                }
            }
            "export" => {
                if parts.len() < 2 {
                    print_error("Usage: theme export <name> [output_file]");
                } else {
                    let name = parts[1];
                    let output = if parts.len() >= 3 {
                        std::path::PathBuf::from(parts[2])
                    } else {
                        std::env::current_dir().unwrap_or_default().join(format!("{}.ntc_theme", name))
                    };
                    if let Err(e) = ThemeManager::export_theme(name, &output) {
                        print_error(&format!("Failed to export: {}", e));
                    } else {
                        print_success(&format!("Exported to: {}", output.display()));
                    }
                }
            }
            "import" => {
                if parts.len() < 2 {
                    print_error("Usage: theme import <file.ntc_theme>");
                } else {
                    let path = std::path::Path::new(parts[1]);
                    if !path.exists() {
                        print_error(&format!("File not found: {}", path.display()));
                    } else if let Err(e) = ThemeManager::import_theme(path) {
                        print_error(&format!("Failed to import: {}", e));
                    } else {
                        print_success(&format!("Imported theme from: {}", path.display()));
                    }
                }
            }
            "rename" | "rnm" => {
                if parts.len() < 4 || parts[2].to_lowercase() != "to" {
                    print_error("Usage: theme rename <old_name> to <new_name>");
                } else if let Err(e) = ThemeManager::rename_theme(parts[1], parts[3]) {
                    print_error(&format!("Failed to rename: {}", e));
                } else {
                    print_success(&format!("Renamed '{}' -> '{}'", parts[1], parts[3]));
                }
            }
            _ => {
                // Try to switch to theme by name, or by index if it's a number
                let themes = ThemeManager::list_themes();
                if let Ok(num) = parts[0].parse::<usize>() {
                    if num >= 1 && num <= themes.len() {
                        let theme_name = &themes[num - 1];
                        if ThemeManager::set_theme(theme_name) {
                            print_success(&format!("Switched to theme: {}", theme_name));
                        } else {
                            print_error(&format!("Failed to switch to theme: {}", theme_name));
                        }
                    } else {
                        print_error(&format!("Invalid number: {}. Use 1-{}", num, themes.len()));
                    }
                } else if ThemeManager::set_theme(parts[0]) {
                    print_success(&format!("Switched to theme: {}", parts[0]));
                } else {
                    print_error(&format!("Theme not found: {}. Use 'theme list' to see available themes.", parts[0]));
                }
            }
        }
        Ok(false)
    }
}

pub fn cmd_fallback(
    input: &str,
    nav: &mut Navigator,
    _watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();

    if cmd.starts_with('@') && cmd.len() > 1 {
        let tp_name = &cmd[1..];
        if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
            TeleportManager::jump_by_name(nav, tp_name)?;
            return Ok(false);
        }
    }

    let cmd_part = input.trim();
    if !cmd_part.is_empty() {
        if let Err(e) = execute_system_command(cmd_part, nav.current_path()) {
            print_error(&format!("{}", e));
            return Ok(false);
        }
    }
    Ok(false)
}