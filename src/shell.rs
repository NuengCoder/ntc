use crate::config::Config;
use crate::explorer::{format_tree, format_tree_with_sizes, generate_tree};
use crate::filetype::{FormatConfig, is_supported_format};
use crate::navigator::{Navigator, clear_screen};
use crate::output::{cat_file, print_error, print_info, print_separator, print_success, print_warning};
use crate::report::{generate_report, ReportFormat};
use crate::teleport::TeleportManager;
use crate::watcher;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Launch the interactive shell 
pub fn run_shell() -> Result<()> {
    let nav = Navigator::new()?;
    run_shell_with_nav(nav)
}

/// Launch the interactive shell with an existing Navigator (for @ shortcuts)
pub fn run_shell_with_nav(mut nav: Navigator) -> Result<()> {
    if std::env::var("NTC_SHELL").is_ok() {
        print_warning("Already inside an ntc shell — nested shells are not supported.");
        return Ok(());
    }
    std::env::set_var("NTC_SHELL", "1");
    
    let mut rl = DefaultEditor::new().expect("Failed to create line editor");
    
    if let Some(history_path) = Config::global().read().unwrap().resolve_history_path() {
        let _ = rl.load_history(&history_path);
    }
    
    // File watcher
    let mut watcher_handle: Option<(notify::RecommendedWatcher, Arc<std::sync::atomic::AtomicBool>)> = None;
    if Config::global_get_file_watcher_enabled() {
        match watcher::start_watcher(nav.current_path()) {
            Ok(w) => {
                watcher_handle = Some(w);
                print_info("File watcher started");
            }
            Err(e) => print_warning(&format!("Watcher failed: {}", e)),
        }
    }
    
    // Welcome banner
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║{}║", format!("              Welcome to ntc {} - Navigate, Tree, Cat          ", env!("CARGO_PKG_VERSION")).cyan().bold());
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", "Type 'help' for available commands, 'exit' to quit.".dimmed());
    show_tree(&nav, Some(1), false, false, false);
    
    loop {
        // Check file watcher
        if let Some((_, ref changed)) = watcher_handle {
            if changed.load(Ordering::Acquire) {
                changed.store(false, Ordering::Relaxed);
                println!();
                print_info("Directory changed — refreshing...");
                show_tree(&nav, Some(1), false, false, false);
            }
        }
        
        let display_path = nav.display_path();
        let prompt = format!("ntc [{}]> ", display_path);
        
        let line = match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(&line).ok();
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".green());
                break;
            }
            Err(_) => break,
        };
        
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Check for piped commands (&&)
        if line.contains("&&") {
            let commands: Vec<&str> = line.split("&&").map(|s| s.trim()).collect();
            let mut should_exit = false;
            for cmd in commands {
                match execute_command(cmd, &mut nav) {
                    Ok(exit) => {
                        if exit {
                            should_exit = true;
                            break;
                        }
                    }
                    Err(e) => {
                        print_error(&format!("{}", e));
                        break;
                    }
                }
            }
            if should_exit {
                println!("{}", "Goodbye!".green());
                break;
            }
            continue;
        }
        
        match execute_command(line, &mut nav) {
            Ok(should_exit) => {
                if should_exit {
                    println!("{}", "Goodbye!".green());
                    break;
                }
            }
            Err(e) => {
                print_error(&format!("{}", e));
            }
        }
    }
    
    if Config::global_get_history_enabled() {
        if let Some(history_path) = Config::global().read().unwrap().resolve_history_path() {
            let _ = rl.save_history(&history_path);
        }
    }
    
    Ok(())
}

// Add this function before execute_command or near the top of shell.rs
fn validate_alias_name(name: &str) -> bool {
    // Reserved command names that would conflict with ntc commands
    let reserved_commands = [
        "go", "cd", "godrive", "god", "back", "b", "view", "txt", "html", "json", "md",
        "seto", "setd", "setl", "sett", "seth", "watch", "clear", "version", "where",
        "gos", "gosc", "ral", "run", "r", "showcg", "help", "exit", "quit", "ignored",
        "ignore", "cared", "ignoref", "caref", "ignoren", "caren", "size", "tp",
    ];
    
    // Check for forbidden characters
    if name.contains('@') || name.contains('#') {
        return false;
    }
    
    // Check if it's a reserved command name
    if reserved_commands.contains(&name) {
        return false;
    }
    
    true
}

/// Execute a single interactive command
fn execute_command(input: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").trim();

    // Check if cmd is a run alias FIRST
    let aliases = Config::global_get_run_aliases();
    if aliases.contains_key(&cmd) && validate_alias_name(&cmd) {
        // Execute as alias (same as "run <alias>")
        let full_cmd = if args.is_empty() {
            aliases[&cmd].clone()
        } else {
            format!("{} {}", aliases[&cmd], args)
        };
        print_info(&format!("Running: {}", full_cmd));
        println!();
        let status = run_system_command(&full_cmd, nav.current_path());
        println!();
        match status {
            Ok(exit_status) => {
                if exit_status.success() {
                    print_success("Command completed successfully.");
                } else {
                    match exit_status.code() {
                        Some(code) => print_error(&format!("Command exited with code: {}", code)),
                        None => print_warning("Command terminated (Ctrl+C)"),
                    }
                }
            }
            Err(e) => print_error(&format!("Failed to execute command: {}", e)),
        }
        return Ok(false);
    }

    match cmd.as_str() {
        "go" | "cd" => {
            if args.is_empty() {
                println!("Usage: go <directory_path>");
                println!("Example: go C:\\Users");
                println!("         go subdir");
                println!("  cd works the same as go");
            } else {
                nav.go_to(Path::new(args))?;
                clear_screen();
                print_success(&format!("Navigated to: {}", nav.display_path()));
                show_tree(nav, Some(1), false, false, false);
            }
        }

        "godrive" | "god" => {
            #[cfg(not(windows))]
            {
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
                    show_tree(nav, Some(1), false, false, false);
                } else if let Ok(num) = choice.parse::<usize>() {
                    if num > 0 && num <= drives.len() {
                        nav.go_drive(drives[num - 1])?;
                        clear_screen();
                        show_tree(nav, Some(1), false, false, false);
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
                show_tree(nav, Some(1), false, false, false);
            }
        }

        "back" | "b" => {
            if args.is_empty() {
                match nav.go_back() {
                    Ok(()) => {
                        clear_screen();
                        show_tree(nav, Some(1), false, false, false);
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
                            show_tree(nav, Some(1), false, false, false);
                        }
                    }
                    _ => print_error(&format!("Invalid number: {}. Usage: back [n]", args)),
                }
            }
        }

        "view" => {
            let show_sizes = args == "--size";
            show_tree(nav, None, true, true, show_sizes);
        }

        "txt" => {
            let parts: Vec<&str> = args.split_whitespace().collect();
            let copy_to_clipboard = parts.contains(&"--cp");
            let target_arg = if copy_to_clipboard {
                parts.iter().find(|&&p| p != "--cp").unwrap_or(&"").trim()
            } else {
                args
            };
            
            let target = if target_arg.is_empty() {
                nav.current_path()
            } else {
                Path::new(target_arg)
            };
            
            if target.is_dir() {
                if copy_to_clipboard {
                    let content = crate::report::generate_report_to_string(target, ReportFormat::Txt)?;
                    crate::output::copy_to_clipboard(&content, "TXT")?;
                    print_success("Directory tree copied to clipboard!");
                } else {
                    generate_report(target, ReportFormat::Txt)?;
                }
            } else if target.is_file() {
                if is_supported_format(target) {
                    let show_lines = Config::global_get_show_line_numbers();
                    cat_file(target, show_lines)?;
                } else {
                    print_warning(&format!("Skipped (not support format): {}", target_arg));
                }
            } else {
                print_error(&format!("Path not found: {}", target_arg));
            }
        }

        "html" => {
            if args.is_empty() {
                generate_report(nav.current_path(), ReportFormat::Html)?;
            } else {
                let target = Path::new(args);
                if target.is_dir() {
                    generate_report(target, ReportFormat::Html)?;
                } else if target.is_file() {
                    if is_supported_format(target) {
                        let show_lines = Config::global_get_show_line_numbers();
                        cat_file(target, show_lines)?;
                    } else {
                        print_warning(&format!("Skipped (not support format): {}", args));
                    }
                } else {
                    print_error(&format!("Path not found: {}", args));
                }
            }
        }

        "json" => {
            let parts: Vec<&str> = args.split_whitespace().collect();
            let copy_to_clipboard = parts.contains(&"--cp");
            let target_arg = if copy_to_clipboard {
                parts.iter().find(|&&p| p != "--cp").unwrap_or(&"").trim()
            } else {
                args
            };
            
            let target = if target_arg.is_empty() {
                nav.current_path()
            } else {
                Path::new(target_arg)
            };
            
            if target.is_dir() {
                if copy_to_clipboard {
                    let content = crate::report::generate_report_to_string(target, ReportFormat::Json)?;
                    crate::output::copy_to_clipboard(&content, "JSON")?;
                    print_success("JSON report copied to clipboard!");
                } else {
                    generate_report(target, ReportFormat::Json)?;
                }
            } else {
                print_error("JSON report only works on directories");
            }
        }
        "md" => {
            let parts: Vec<&str> = args.split_whitespace().collect();
            let copy_to_clipboard = parts.contains(&"--cp");
            let target_arg = if copy_to_clipboard {
                parts.iter().find(|&&p| p != "--cp").unwrap_or(&"").trim()
            } else {
                args
            };
            
            let target = if target_arg.is_empty() {
                nav.current_path()
            } else {
                Path::new(target_arg)
            };
            
            if target.is_dir() {
                if copy_to_clipboard {
                    let content = crate::report::generate_report_to_string(target, ReportFormat::Md)?;
                    crate::output::copy_to_clipboard(&content, "Markdown")?;
                    print_success("Markdown report copied to clipboard!");
                } else {
                    generate_report(target, ReportFormat::Md)?;
                }
            } else {
                print_error("Markdown report only works on directories");
            }
        }

        "seto" => {
            if args.is_empty() {
                println!("Current output path: {}", Config::global_get_output_path().display());
            } else {
                Config::global_set_output_path(Path::new(args));
                print_success(&format!("Output path set to: {}", Config::global_get_output_path().display()));
            }
        }

        "setd" => {
            if args.is_empty() {
                println!("Current max depth: {}", Config::global_get_max_depth());
            } else {
                match args.parse::<usize>() {
                    Ok(depth) => {
                        Config::global_set_max_depth(depth);
                        print_success(&format!("Max depth set to: {}", Config::global_get_max_depth()));
                    }
                    Err(_) => print_error(&format!("Invalid depth: {}. Must be a positive integer.", args)),
                }
            }
        }

        "setl" => {
            if args.is_empty() {
                let state = if Config::global_get_show_line_numbers() { "ON" } else { "OFF" };
                println!("Line numbers: {}", state);
            } else {
                match Config::parse_line_numbers_state(args) {
                    Some(state) => {
                        Config::global_set_show_line_numbers(state);
                        print_success(&format!("Line numbers: {}", if state { "ON" } else { "OFF" }));
                    }
                    None => print_error(&format!("Invalid value: {}. Use ON or OFF.", args)),
                }
            }
        }

        "sett" => {
            if args.is_empty() {
                println!("Current threads: {}", Config::global_get_num_threads());
            } else {
                match Config::parse_num_threads(args) {
                    Some(threads) => {
                        Config::global_set_num_threads(threads);
                        print_success(&format!("Threads set to: {}", Config::global_get_num_threads()));
                    }
                    None => print_error(&format!("Invalid thread count: {}. Must be a positive integer.", args)),
                }
            }
        }

        "seth" => {
            if args.is_empty() {
                let enabled = Config::global_get_history_enabled();
                let path = Config::global_get_history_path();
                println!("History: {}", if enabled { "ON".green() } else { "OFF".red() });
                match path {
                    Some(p) => println!("History path: {}", p.display()),
                    None => println!("History path: default"),
                }
            } else {
                let upper = args.to_uppercase();
                if upper == "ON" {
                    Config::global_set_history_enabled(true);
                    print_success("History: ON");
                } else if upper == "OFF" {
                    Config::global_set_history_enabled(false);
                    print_warning("History: OFF");
                } else if args == "default" {
                    Config::global_set_history_path(None);
                    print_success("History path reset to default");
                } else {
                    let p = Path::new(args);
                    Config::global_set_history_path(Some(p.to_path_buf()));
                    print_success(&format!("History path set to: {}", p.display()));
                }
            }
        }

        "watch" => {
            if args.is_empty() {
                let enabled = Config::global_get_file_watcher_enabled();
                println!("File watcher: {}", if enabled { "ON".green() } else { "OFF".red() });
                println!("Usage: watch ON|OFF");
            } else {
                let upper = args.to_uppercase();
                if upper == "ON" {
                    Config::global_set_file_watcher_enabled(true);
                    print_success("File watcher: ON (restart ntc to activate)");
                } else if upper == "OFF" {
                    Config::global_set_file_watcher_enabled(false);
                    print_warning("File watcher: OFF (restart ntc to deactivate)");
                } else {
                    print_error("Use watch ON or watch OFF");
                }
            }
        }

        "clear" => {
            clear_screen();
            println!();
            println!("╔══════════════════════════════════════════════════════════════════╗");
            println!("║{}║", format!("              Welcome to ntc {} - Navigate, Tree, Cat          ", env!("CARGO_PKG_VERSION")).cyan().bold());
            println!("╚══════════════════════════════════════════════════════════════════╝");
            println!("{}", "Type 'help' for available commands, 'exit' to quit.".dimmed());
            show_tree(nav, Some(1), false, false, false);
        }

        "version" => {
            println!("ntc {}", env!("CARGO_PKG_VERSION").green().bold());
        }

        "where" => {
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
            
            // Check if config file exists
            if config_path.exists() {
                println!("  {}", "✓ Config file exists".green());
                
                // Show config file size on Linux/WSL
                #[cfg(not(windows))]
                if let Ok(metadata) = std::fs::metadata(&config_path) {
                    println!("  {} {}", "📏 Config size:".dimmed(), crate::explorer::human_readable_size(metadata.len()).dimmed());
                }
                
                #[cfg(windows)]
                if let Ok(metadata) = std::fs::metadata(&config_path) {
                    println!("  {} {}", "📏 Config size:".dimmed(), crate::explorer::human_readable_size(metadata.len()).dimmed());
                }
            } else {
                println!("  {}", "⚠ Config file not found (will be created on first save)".yellow());
            }
            println!();
        }

        "gos" => {
            let dirs = nav.list_subdirs()?;
            println!();
            println!("{}", "gos where?".cyan().bold());
            println!("  {} {}", "0".yellow(), "exit".dimmed());
            if dirs.is_empty() {
                println!("  {}", "(no subdirectories)".dimmed());
            } else {
                for (i, name) in &dirs {
                    println!("  {} {}", i.to_string().yellow(), name.blue());
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
                    show_tree(nav, Some(1), false, false, false);
                } else {
                    print_error("Invalid number.");
                }
            } else {
                print_error("Invalid input.");
            }
        }

        "gosc" => {
            gosc_loop(nav)?;
            // After exiting gosc, show current directory tree
            show_tree(nav, Some(1), false, false, false);
        }

        "ral" => {
            if args.is_empty() {
                // Show help
                println!("{}", "Run Alias (ral) Commands:".cyan().bold());
                println!("  ral add <name> <command>    Create a new run alias");
                println!("  ral edit <name> <command>   Update an existing alias");
                println!("  ral rm <name>               Remove an alias");
                println!("  ral list                    Show all aliases");
                println!();
                println!("{}", "Examples:".green());
                println!("  ral add btr \"cargo build --release\"");
                println!("  ral add py \"python test.py\"");
                println!("  ral edit py \"python main.py\"");
                println!("  ral list");
                println!("  ral rm py");
                println!();
                println!("{}", "Usage with run:".green());
                println!("  run btr     # Executes: cargo build --release");
                println!("  run py      # Executes: python test.py");
            } else {
                let parts: Vec<&str> = args.splitn(3, ' ').collect();
                let subcmd = parts[0].to_lowercase();
                
                match subcmd.as_str() {
                    "add" => {
                        if parts.len() < 3 {
                            print_error("Usage: ral add <name> <command>");
                            println!("Example: ral add btr \"cargo build --release\"");
                        } else {
                            let name = parts[1];
                            let command = parts[2];
                            
                            // Validate alias name
                            if !validate_alias_name(name) {
                                print_error(&format!("Invalid alias name: '{}'", name));
                                println!("Alias names cannot:");
                                println!("  - Start with @ or #");
                                println!("  - Be a reserved command (go, view, txt, etc.)");
                                println!("  - Contain special characters");
                                return Ok(false);
                            }
                            
                            Config::global_add_run_alias(name, command);
                            print_success(&format!("Alias '{}' -> '{}'", name, command));
                            println!("  Now you can run: {}", name.green());
                        }
                    }
                    "edit" => {
                        if parts.len() < 3 {
                            print_error("Usage: ral edit <name> <new_command>");
                            println!("Example: ral edit py \"python main.py\"");
                        } else {
                            let name = parts[1];
                            let command = parts[2];
                            
                            // Validate alias name
                            if !validate_alias_name(name) {
                                print_error(&format!("Invalid alias name: '{}'", name));
                                println!("Alias names cannot:");
                                println!("  - Start with @ or #");
                                println!("  - Be a reserved command (go, view, txt, etc.)");
                                return Ok(false);
                            }
                            
                            if Config::global_update_run_alias(name, command) {
                                print_success(&format!("Updated alias '{}' -> '{}'", name, command));
                            } else {
                                print_error(&format!("Alias '{}' not found. Use 'ral add' to create it.", name));
                            }
                        }
                    }
                    "rm" | "remove" => {
                        if parts.len() < 2 {
                            print_error("Usage: ral rm <name>");
                            println!("Example: ral rm py");
                        } else {
                            let name = parts[1];
                            Config::global_remove_run_alias(name);
                            print_success(&format!("Removed alias '{}'", name));
                        }
                    }
                    "list" | "ls" => {
                        let aliases = Config::global_get_run_aliases();
                        if aliases.is_empty() {
                            print_info("No run aliases defined. Use 'ral add <name> <command>'");
                        } else {
                            println!();
                            println!("{}", "==================================================".cyan());
                            println!("{}", "📌 Run Aliases (ral)".cyan().bold());
                            println!("{}", "==================================================".cyan());
                            let mut sorted: Vec<_> = aliases.iter().collect();
                            sorted.sort_by(|a, b| a.0.cmp(b.0));
                            for (i, (name, cmd)) in sorted.iter().enumerate() {
                                // Check if alias name is valid
                                let is_valid = validate_alias_name(name);
                                let name_display = if is_valid {
                                    name.blue()
                                } else {
                                    format!("{} (INVALID - contains @/# or reserved)", name).red()
                                };
                                println!("  {}. {} -> {}", 
                                    (i + 1).to_string().yellow(), 
                                    name_display, 
                                    cmd.dimmed());
                            }
                            println!();
                            println!("{}", "Usage: <alias> [args]".green());
                            println!("{}", "  (just type the alias name, no 'run' needed)".dimmed());
                        }
                    }
                    _ => {
                        print_error(&format!("Unknown ral subcommand: {}", subcmd));
                        println!("Type 'ral' for help.");
                    }
                }
            }
        }

        "run" | "r" => {
            if args.is_empty() {
                println!("Usage: run <command|alias> [args...]");
                println!();
                println!("{}", "Examples:".green());
                println!("  run python --version        # Run real command");
                println!("  run btr                     # Run alias (cargo build --release)");
                println!("  run py test.py              # Run alias with extra args");
                println!();
                println!("{}", "Manage aliases with: ral add/list/rm/edit".cyan());
            } else {
                // Parse first word to check if it's an alias
                let parts: Vec<&str> = args.splitn(2, ' ').collect();
                let first_word = parts[0];
                let remaining_args = parts.get(1).unwrap_or(&"").trim();
                
                let aliases = Config::global_get_run_aliases();
                
                if let Some(aliased_cmd) = aliases.get(first_word) {
                    // Execute alias with remaining args appended
                    let full_cmd = if remaining_args.is_empty() {
                        aliased_cmd.clone()
                    } else {
                        format!("{} {}", aliased_cmd, remaining_args)
                    };
                    print_info(&format!("Running: {}", full_cmd));
                    println!();
                    
                    let status = run_system_command(&full_cmd, nav.current_path());
                    
                    println!();
                    match status {
                        Ok(exit_status) => {
                            if exit_status.success() {
                                print_success("Command completed successfully.");
                            } else {
                                match exit_status.code() {
                                    Some(code) => print_error(&format!("Command exited with code: {}", code)),
                                    None => print_warning("Command terminated (Ctrl+C)"),
                                }
                            }
                        }
                        Err(e) => {
                            print_error(&format!("Failed to execute command: {}", e));
                        }
                    }
                } else {
                    // Not an alias, run as-is
                    print_info(&format!("Running: {}", args));
                    println!();
                    
                    let status = run_system_command(args, nav.current_path());
                    
                    println!();
                    match status {
                        Ok(exit_status) => {
                            if exit_status.success() {
                                print_success("Command completed successfully.");
                            } else {
                                match exit_status.code() {
                                    Some(code) => print_error(&format!("Command exited with code: {}", code)),
                                    None => print_warning("Command terminated (Ctrl+C)"),
                                }
                            }
                        }
                        Err(e) => {
                            print_error(&format!("Failed to execute command: {}", e));
                        }
                    }
                }
            }
        }

        "showcg" => {
            let w = 65;
            println!();
            println!("┌{}┐", "─".repeat(w));
            println!("│{:^w$}│", "Current Configuration".cyan().bold(), w = w);
            println!("├{}┤", "─".repeat(w));
            println!("│ {:<20} {:<42} │", "Output Path:", Config::global_get_output_path().display().to_string().green());
            println!("│ {:<20} {:<42} │", "Max Depth:", Config::global_get_max_depth().to_string().yellow());
            println!("│ {:<20} {:<42} │", "Line Numbers:", if Config::global_get_show_line_numbers() { "ON".green() } else { "OFF".red() });
            println!("│ {:<20} {:<42} │", "Threads:", Config::global_get_num_threads().to_string().yellow());
            println!("│ {:<20} {:<42} │", "History:", if Config::global_get_history_enabled() { "ON".green() } else { "OFF".red() });
            println!("│ {:<20} {:<42} │", "Watcher:", if Config::global_get_file_watcher_enabled() { "ON".green() } else { "OFF".red() });
            println!("└{}┘", "─".repeat(w));
            println!();
        }

        "help" => {
            print_interactive_help();
        }

        "exit" | "quit" => {
            return Ok(true);
        }

        "ignored" => {
            let dirs = Config::global_get_ignored_dirs();
            let fmt_cfg = FormatConfig::from_global();
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
        }
        "ignore" => {
            if args.is_empty() {
                println!("Usage: ignore <directory_name>");
            } else {
                Config::global_add_ignored_dir(args);
                print_success(&format!("Now ignoring directory: {}", args));
            }
        }
        "cared" => {
            if args.is_empty() {
                println!("Usage: cared <directory_name>");
            } else {
                Config::global_remove_ignored_dir(args);
                print_success(&format!("No longer ignoring directory: {}", args));
            }
        }
        "ignoref" => {
            if args.is_empty() {
                println!("Usage: ignoref <extension>");
            } else {
                Config::global_add_ignored_extension(args);
                print_success(&format!("Now ignoring .{} files", args));
            }
        }
        "caref" => {
            if args.is_empty() {
                println!("Usage: caref <extension>");
            } else {
                Config::global_add_extra_supported_extension(args);
                print_success(&format!("Now caring about .{} files", args));
            }
        }
        "ignoren" => {
            if args.is_empty() {
                println!("Usage: ignoren <filename>");
            } else {
                Config::global_add_ignored_file(args);
                print_success(&format!("Now ignoring file: {}", args));
            }
        }
        "caren" => {
            if args.is_empty() {
                println!("Usage: caren <filename>");
            } else {
                Config::global_add_extra_supported_file(args);
                print_success(&format!("Now caring about file: {}", args));
            }
        }

        "size" => {
            let total = crate::explorer::calculate_dir_size(nav.current_path());
            let bytes_str = format!("{}", total);
            let human_str = crate::explorer::human_readable_size(total);
            let max_label = 8; // "Bytes: " = 7 chars + margin
            let max_value = bytes_str.len().max(human_str.len());
            let w = max_label + max_value + 10; // labels + values + padding
            
            println!();
            println!("┌{}┐", "─".repeat(w));
            println!("│{:^w$}│", "Current Directory Size".cyan().bold(), w = w);
            println!("├{}┤", "─".repeat(w));
            println!("│ Bytes: {:<w$} │", bytes_str.yellow(), w = w - 9);
            println!("│ Human: {:<w$} │", human_str.green().bold(), w = w - 9);
            println!("└{}┘", "─".repeat(w));
        }

        "tp" => {
            if args.is_empty() {
                // Interactive menu
                TeleportManager::interactive_menu(nav)?;
            } else {
                let tp_parts: Vec<&str> = args.splitn(2, ' ').collect();
                let subcmd = tp_parts[0].to_lowercase();
                let subargs = tp_parts.get(1).unwrap_or(&"").trim();
                
                match subcmd.as_str() {
                    "add" => {
                        if subargs.is_empty() {
                            println!("Usage: tp add <name> [path]");
                            println!("  tp add work         # Save current directory as 'work'");
                            println!("  tp add work D:\\Work # Save specific path as 'work'");
                        } else {
                            let add_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                            let name = add_parts[0];
                            
                            if add_parts.len() > 1 {
                                // Specific path provided
                                let path = std::path::Path::new(add_parts[1]);
                                TeleportManager::add(name, path.to_path_buf())?;
                            } else {
                                // Use current directory
                                TeleportManager::add_current(nav, name)?;
                            }
                        }
                    }
                    "jump" | "to" => {
                        if subargs.is_empty() {
                            println!("Usage: tp jump <name|number>");
                            println!("  tp jump work  # Jump by name");
                            println!("  tp jump 1     # Jump by number (from tp list)");
                        } else if let Ok(num) = subargs.parse::<usize>() {
                            TeleportManager::jump_by_index(nav, num)?;
                        } else {
                            TeleportManager::jump_by_name(nav, subargs)?;
                        }
                    }
                    "list" | "ls" => {
                        TeleportManager::list()?;
                    }
                    "rm" => {
                        if subargs.is_empty() {
                            println!("Usage: tp rm <name|number>");
                            println!("  tp rm work  # Remove by name");
                            println!("  tp rm 1     # Remove by number (from tp list)");
                        } else if let Ok(num) = subargs.parse::<usize>() {
                            TeleportManager::remove_by_index(num)?;
                        } else {
                            TeleportManager::remove_by_name(subargs)?;
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
        }

        // Handle @name shortcut (comes before unknown command check)
        // Add this BEFORE the default _ case
        _ => {
            // Check if it's an @teleport shortcut
            if cmd.starts_with('@') && cmd.len() > 1 {
                let tp_name = &cmd[1..];
                if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
                    TeleportManager::jump_by_name(nav, tp_name)?;
                    return Ok(false);
                }
            }
            
            print_error(&format!("Unknown command: {}", cmd));
            println!("{}", "Type 'help' for available commands.".dimmed());
        }
    }

    Ok(false)
}

/// Continuous directory selection loop — never ends until user inputs 0
fn gosc_loop(nav: &mut Navigator) -> Result<()> {
    loop {
        // 1. List subdirectories of current path
        let dirs = nav.list_subdirs()?;
        
        // 2. Clear screen for fresh display
        clear_screen();
        
        // 3. Show current directory tree
        show_tree(nav, Some(1), false, false, false);
        
        // 4. Display gosc menu
        println!();
        println!("╔══════════════════════════════════════════════════╗");
        println!("║{:^50}║", "gosc — Navigate Continuously".cyan().bold());
        println!("╠══════════════════════════════════════════════════╣");
        println!("║ {}", format!("{} go back 1 level", "-1".yellow()));
        println!("║ {}", format!("{} go back 2 levels", "-2".yellow()));
        println!("║ {}", format!("{} go back n levels", "-n".yellow()));
        println!("║ {}", format!("{} exit gosc", "0".red()));
        println!("╠──────────────────────────────────────────────────╣");
        
        if dirs.is_empty() {
            println!("║ {}", "(no subdirectories)".dimmed());
        } else {
            for (i, name) in &dirs {
                println!("║ {}. {}", i.to_string().yellow(), name.blue());
            }
        }
        
        println!("╚──────────────────────────────────────────────────╝");
        println!();
        print!("{} ", "gosc>".green().bold());
        
        // 5. Read input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        // 6. Process input
        if input.is_empty() {
            continue;
        }
        
        // Exit condition
        if input == "0" {
            println!("{}", "Exiting gosc...".dimmed());
            break;
        }
        
        // Back command: -1, -2, -n
        if input.starts_with('-') {
            let back_str = &input[1..];
            match back_str.parse::<usize>() {
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
                    if !success {
                        println!();
                        print!("Press Enter to continue...");
                        let mut pause = String::new();
                        std::io::stdin().read_line(&mut pause).ok();
                    }
                }
                Err(_) => {
                    print_error(&format!("Invalid back count: {}. Use -1, -2, -n", back_str));
                    println!();
                    print!("Press Enter to continue...");
                    let mut pause = String::new();
                    std::io::stdin().read_line(&mut pause).ok();
                }
                Ok(_) => {},
            }
            continue;
        }
        
        // Number selection
        match input.parse::<usize>() {
            Ok(num) => {
                if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
                    let new_path = nav.current_path().join(name);
                    match nav.go_to(&new_path) {
                        Ok(()) => {}
                        Err(e) => {
                            print_error(&format!("{}", e));
                            println!();
                            print!("Press Enter to continue...");
                            let mut pause = String::new();
                            std::io::stdin().read_line(&mut pause).ok();
                        }
                    }
                } else {
                    print_error(&format!("Invalid number: {}. Choose 1-{}", num, dirs.len()));
                    println!();
                    print!("Press Enter to continue...");
                    let mut pause = String::new();
                    std::io::stdin().read_line(&mut pause).ok();
                }
            }
            Err(_) => {
                print_error(&format!("Invalid input: {}. Use 0 to exit, -n to go back, or a number", input));
                println!();
                print!("Press Enter to continue...");
                let mut pause = String::new();
                std::io::stdin().read_line(&mut pause).ok();
            }
        }
    }
    
    Ok(())
}

pub(crate) fn show_tree(
    nav: &Navigator,
    max_depth_override: Option<usize>,
    show_progress: bool,
    include_files: bool,
    show_sizes: bool,
) {
    println!();
    print_separator("Current Directory");
    println!("Path: {}", nav.display_path().cyan());
    println!();

    let tree_pb = if show_progress {
        let total = crate::explorer::count_entries(
            &nav.current_path().to_string_lossy(),
            max_depth_override,
        );
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::with_template("ViewD  [{bar:30}] {percent}% {msg}")
                .unwrap()
                .progress_chars("=> "),
        );
        pb.set_message("Building tree...");
        Some(pb)
    } else {
        None
    };

    let mut tree = generate_tree(
        &nav.current_path().to_string_lossy(),
        max_depth_override,
        include_files,
        tree_pb.as_ref(),
    );

    if let Some(pb) = tree_pb {
        pb.finish_with_message("Done");
    }

    let tree_str = if show_sizes {
        // PRE-COMPUTE SIZES (one pass instead of N passes)
        let dir_count = crate::explorer::count_dirs_in_tree(&tree);
        let scan_pb = ProgressBar::new(dir_count);
        scan_pb.set_style(
            ProgressStyle::with_template("ScanB  [{bar:30}] {percent}% {msg}")
                .unwrap()
                .progress_chars("=> "),
        );
        scan_pb.set_message("Calculating sizes...");

        crate::explorer::compute_tree_sizes(&mut tree, Some(&scan_pb));  // ← ADD THIS LINE

        let result = format_tree_with_sizes(&tree, "", true, true, Some(&scan_pb));
        scan_pb.finish_with_message("Done");
        result
    } else {
        format_tree(&tree, "", true)
    };

    // Color the output:
    //   [Directory] lines → blue (regardless of connector prefix)
    //   file connector lines (├── / └──) → green
    //   everything else (root name, blank lines) → default
    for line in tree_str.lines() {
        if line.contains("[Directory]") {
            println!("{}", line.blue());
        } else if line.trim().starts_with("├──") || line.trim().starts_with("└──") {
            println!("{}", line.green());
        } else {
            println!("{}", line);
        }
    }
}

/// Run a system command, handling Ctrl+C properly
fn run_system_command(args: &str, cwd: &Path) -> Result<std::process::ExitStatus> {
    #[cfg(windows)]
    {
        let mut child = std::process::Command::new("cmd")
            .args(["/C", args])
            .current_dir(cwd)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;
        
        let child_id = child.id();
        
        // On Ctrl+C, kill the child process tree
        let _ = ctrlc::set_handler(move || {
            let _ = std::process::Command::new("taskkill")
                .args(["/F", "/T", "/PID", &child_id.to_string()])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        });
        
        let status = child.wait()?;
        
        Ok(status)
    }
    
    #[cfg(not(windows))]
    {
        let mut child = std::process::Command::new("sh")
            .args(["-c", args])
            .current_dir(cwd)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;
        
        let child_id = child.id();
        
        let _ = ctrlc::set_handler(move || {
            let _ = std::process::Command::new("kill")
                .args(["-TERM", &child_id.to_string()])
                .status();
        });
        
        let status = child.wait()?;
        
        Ok(status)
    }
}

fn print_interactive_help() {
    println!(
        "╔══════════════════════════════════════════════════════════════════╗"
    );
    println!(
        "║                     ntc {} - Interactive Help                 ║",
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "╚══════════════════════════════════════════════════════════════════╝"
    );
    println!("{}", "NAVIGATION COMMANDS:".cyan().bold());
    println!("  go <path>           Navigate to a directory (shows root contents)");
    println!("  go                  Show go command usage");
    println!("  cd <path>           Change directory (same as go)");
    println!("  cd                  Show cd command usage");
    println!("  gos                 List subdirectories and pick one to navigate");
    println!("  gosc                Continuous directory navigation (0 to exit)");
    println!("  godrive             List all drives and select one");
    println!("  godrive <letter>    Navigate to a drive (e.g., godrive C)");
    println!("  back                Go back to parent directory");
    println!("  back <n>            Go back n parent directories");

    println!();
    println!("{}", "TELEPORT COMMANDS:".cyan().bold());
    println!("  tp                  Show interactive teleport menu");
    println!("  tp add <name>       Save current location as teleport point");
    println!("  tp add <name> <path> Save specific path as teleport point");
    println!("  tp jump <name>      Teleport to saved location");
    println!("  tp to <name>        Teleport to saved location (alias)");
    println!("  tp list             List all teleport points");
    println!("  tp rm <name>        Remove teleport point");
    println!("  tp cls              Clear all teleport points");
    println!("  @<name>             Quick teleport shortcut");

    println!();
    println!("{}", "RUN ALIAS COMMANDS:".cyan().bold());
    println!("  ral add <name> <command>    Create a run alias");
    println!("  ral edit <name> <command>   Update an existing alias");
    println!("  ral rm <name>               Remove an alias");
    println!("  ral list                    Show all aliases");
    println!("  <alias>                     Execute alias directly (no 'run' needed)");
    println!("  run <alias> [args]          Execute alias with arguments");
    println!("  r <alias> [args]            Shortcut for run");
    println!();
    println!("{}", "Run Alias Rules:".yellow());
    println!("  • Cannot start with @ or #");
    println!("  • Cannot be a reserved command (go, view, txt, etc.)");
    println!("  • Use letters, numbers, hyphens, and underscores only");
    println!();
    println!("{}", "Run Alias Examples:".green());
    println!("  ral add btr \"cargo build --release\"");
    println!("  ral add py \"python test.py\"");
    println!("  btr                         # Runs: cargo build --release");
    println!("  py --verbose                # Runs: python test.py --verbose");

    println!();
    println!("{}", "VIEW COMMAND:".cyan().bold());
    println!("  view                Show full directory tree using configured depth");
    println!("  view --size         Show directory tree with sizes");

    println!();
    println!("{}", "SIZE COMMAND:".cyan().bold());
    println!("  size                Show current directory size");

    println!();
    println!("{}", "REPORT COMMANDS:".cyan().bold());
    println!("  txt                 Generate TXT report of current directory");
    println!("  txt <dir>           Generate TXT report of specified directory");
    println!("  txt <file>          Display file contents (if supported format)");
    println!("  txt --cp            Copy directory tree to clipboard");
    println!("  json                Generate JSON report of current directory");
    println!("  json <dir>          Generate JSON report of specified directory");
    println!("  json --cp           Copy JSON report to clipboard");
    println!("  md                  Generate Markdown report of current directory");
    println!("  md <dir>            Generate Markdown report of specified directory");
    println!("  md --cp             Copy Markdown report to clipboard");
    println!("  html                Generate HTML report of current directory");
    println!("  html <dir>          Generate HTML report of specified directory");
    println!("  html <file>         Display file contents (if supported format)");

    println!();
    println!("{}", "CONFIGURATION COMMANDS:".cyan().bold());
    println!("  setO                Show current output path");
    println!("  setO <path>         Set output path");
    println!("  setD                Show current max depth");
    println!("  setD <int>          Set max depth (min: 1, max: 12)");
    println!("  setL                Show line number setting (ON/OFF)");
    println!("  setL ON|OFF         Enable/disable line numbers");
    println!("  setT                Show current thread count");
    println!("  setT <int>          Set number of threads");
    println!("  setH                Show history settings");
    println!("  setH ON|OFF         Enable/disable history");
    println!("  setH <path>         Set custom history file path");
    println!("  setH default        Reset history to default location");
    println!("  showcg              Show current configuration overview");
    println!("  watch ON|OFF        Enable/disable file watcher");

    println!();
    println!("{}", "IGNORE/CARE COMMANDS:".cyan().bold());
    println!("  ignored             Show all ignored items");
    println!("  ignore <name>       Ignore a directory name");
    println!("  cared <name>        Stop ignoring a directory name");
    println!("  ignoref <ext>       Ignore a file extension");
    println!("  caref <ext>         Care about a file extension");
    println!("  ignoren <file>      Ignore a specific file");
    println!("  caren <file>        Care about a specific file");

    println!();
    println!("{}", "PIPING:".cyan().bold());
    println!("  Use && to chain commands: go src && txt && back");

    println!();
    println!("{}", "EXECUTION COMMAND:".cyan().bold());
    println!("  run <command>       Execute a system command from ntc shell");
    println!("                       Example: run python test.py");
    println!("                       Example: run cargo build");
    println!("                       Example: run git status");
    println!("  r <command>         Shortcut for run");

    println!();
    println!("{}", "TERMINAL COMMANDS:".cyan().bold());
    println!("  clear               Clear the terminal screen");
    println!("  version             Show version information");
    println!("  where               Show ntc executable and current directory");

    println!();
    println!("{}", "OTHER COMMANDS:".cyan().bold());
    println!("  help                Show this help");
    println!("  exit, quit          Exit ntc");
    println!();
}

// Add this function near print_interactive_help() or anywhere in shell.rs
fn print_tp_help() {
    println!();
    println!("{}", "Teleport (tp) Commands:".cyan().bold());
    println!("  tp                 Interactive menu (list + select)");
    println!("  tp add <name>      Save current directory as <name>");
    println!("  tp add <name> <path>  Save specific path as <name>");
    println!("  tp jump <name>     Teleport to savepoint by name");
    println!("  tp jump <number>   Teleport to savepoint by number");
    println!("  tp to <name>       Teleport to savepoint by name");
    println!("  tp to <number>     Teleport to savepoint by number");
    println!("  tp list            Show all savepoints (non-interactive)");
    println!("  tp rm <name>       Remove savepoint by name");
    println!("  tp rm <number>     Remove savepoint by number");
    println!("  tp cls             Clear ALL savepoints (asks confirmation)");
    println!("  @<name>            Shortcut to teleport to <name>");
    println!();
    println!("{}", "Examples:".green());
    println!("  tp add work                    # Save current dir as 'work'");
    println!("  tp add projects D:\\Projects   # Save specific path");
    println!("  tp jump work                  # Teleport to work");
    println!("  @work                         # Same as above (shorter)");
    println!("  tp                            # Show interactive menu");
    println!("  tp rm work                    # Remove savepoint");
}