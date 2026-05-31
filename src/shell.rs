use crate::config::{Config};
use crate::explorer::{count_dirs_in_tree, format_tree, format_tree_with_sizes, generate_tree};
use crate::filetype::{FormatConfig, is_supported_format};
use crate::navigator::{Navigator, clear_screen};
use crate::output::{cat_file, print_error, print_info, print_separator, print_success, print_warning};
use crate::report::{generate_report, ReportFormat};
use crate::teleport::TeleportManager;
use crate::watcher;
use anyhow::{Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::Arc;

// ============================================================================
// Helper functions for alias expansion and pipeline processing
// ============================================================================

/// Split a line on `&&` that are not inside double quotes.
fn split_on_ampersands(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '"' => {
                current.push('"');
                in_quotes = !in_quotes;
                i += 1;
            }
            '&' if !in_quotes && i + 1 < chars.len() && chars[i + 1] == '&' => {
                if !current.trim().is_empty() {
                    result.push(current.trim().to_string());
                }
                current.clear();
                i += 2;
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                }
                continue;
            }
            _ => {
                current.push(chars[i]);
                i += 1;
            }
        }
    }
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}

/// Check if a string contains && outside quotes
fn contains_ampersands(s: &str) -> bool {
    split_on_ampersands(s).len() > 1
}

/// Parse a call token like `py(hello)` into `("py", Some("hello"))`.
/// Returns `(token, None)` if there are no parentheses.
fn parse_call_syntax(token: &str) -> (&str, Option<&str>) {
    if let Some(paren_start) = token.find('(') {
        if token.ends_with(')') {
            let base = &token[..paren_start];
            let arg  = &token[paren_start + 1..token.len() - 1];
            return (base, Some(arg));
        }
    }
    (token, None)
}

// ── shared arg parser for fs / ds ─────────────────────────────────────────
// Accepts:  <pattern> [-d <depth>]   (depth flag must be at the end)
// Returns:  (pattern, max_depth)
fn parse_search_args(args: &str) -> (String, usize) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let default_depth = Config::global_get_max_depth();

    // Look for -d <n> at the end: ["pattern", ..., "-d", "3"]
    if parts.len() >= 3 {
        let n = parts.len();
        if parts[n - 2] == "-d" {
            if let Ok(depth) = parts[n - 1].parse::<usize>() {
                let pattern = parts[..n - 2].join(" ");
                return (pattern, depth);
            }
        }
    }

    (args.to_string(), default_depth)
}

/// Replace `$identifier` placeholders in `template` with positional `args`.
/// Placeholders are matched to args by their order of first appearance in the template.
/// e.g. template = "pytest $x --cov=$y",  args = ["mymodule", "src"]
///   -> first  $identifier seen: $x -> "mymodule"
///   -> second $identifier seen: $y -> "src"
///   -> result: "pytest mymodule --cov=src"
///
/// If fewer args than placeholders, unmatched $names are left as-is.
/// If more args than placeholders, extras are ignored.
fn substitute_params(template: &str, args: &[&str]) -> String {
    // 1. Collect placeholder names in order of first appearance.
    let mut param_order: Vec<String> = Vec::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            let start = i + 1;
            let mut end = start;
            while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                end += 1;
            }
            if end > start {
                let name: String = chars[start..end].iter().collect();
                if !param_order.contains(&name) {
                    param_order.push(name);
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }

    // 2. Build a name->value map from param_order + args (positional).
    let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    for (idx, name) in param_order.iter().enumerate() {
        if let Some(val) = args.get(idx) {
            map.insert(name.as_str(), val);
        }
    }

    // 3. Walk the template again and substitute.
    let mut result = String::with_capacity(template.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            let start = i + 1;
            let mut end = start;
            while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                end += 1;
            }
            if end > start {
                let name: String = chars[start..end].iter().collect();
                if let Some(val) = map.get(name.as_str()) {
                    result.push_str(val);
                } else {
                    // No mapping — keep original $name
                    result.push('$');
                    result.push_str(&name);
                }
                i = end;
            } else {
                result.push('$');
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Recursively expand aliases in a command (no && splitting).
/// Supports parameterised calls: `py(hello)` with template `python $x.py` -> `python hello.py`
fn expand_aliases(cmd: &str, depth: usize) -> String {
    const MAX_DEPTH: usize = 5;
    if depth > MAX_DEPTH {
        return cmd.to_string();
    }

    let aliases = Config::global_get_run_aliases();
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    if parts.is_empty() {
        return cmd.to_string();
    }

    let first_token = parts[0];
    let rest = if parts.len() > 1 {
        &cmd[first_token.len()..].trim_start()
    } else {
        ""
    };

    // Parse potential `name(arg)` call syntax
    let (base_name, call_arg) = parse_call_syntax(first_token);
    let lookup_key = base_name.to_lowercase();

    if let Some(template) = aliases.get(&lookup_key) {
        // If called with an argument, substitute every `$word` placeholder in the template.
        let expanded_template = if let Some(args_str) = call_arg {
            let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
            substitute_params(template, &args)
        } else {
            template.clone()
        };

        let new_cmd = if rest.is_empty() {
            expanded_template
        } else {
            format!("{} {}", expanded_template, rest)
        };
        expand_aliases(&new_cmd, depth + 1)
    } else {
        cmd.to_string()
    }
}

/// Fully expand a command line, handling && properly
fn expand_command_line(input: &str) -> Vec<String> {
    if input.trim().is_empty() {
        return vec![];
    }
    
    let parts = split_on_ampersands(input);
    let mut result = Vec::new();
    for part in parts {
        let expanded = expand_aliases(&part, 0);
        if contains_ampersands(&expanded) {
            result.extend(expand_command_line(&expanded));
        } else if !expanded.trim().is_empty() {
            result.push(expanded);
        }
    }
    result
}

/// Execute a system command
fn execute_system_command(cmd: &str, cwd: &Path) -> Result<bool> {
    print_info(&format!("Executing: {}", cmd));
    println!();
    
    let status = run_system_command(cmd, cwd);
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
            Ok(false)
        }
        Err(e) => {
            print_error(&format!("Failed to execute command: {}", e));
            Ok(false)
        }
    }
}

// ============================================================================
// Main shell entry points
// ============================================================================

pub fn run_shell() -> Result<()> {
    let nav = Navigator::new()?;
    run_shell_with_nav(nav)
}

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
    
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║{}║", format!("              Welcome to ntc {} - Navigate, Tree, Cat          ", env!("CARGO_PKG_VERSION")).cyan().bold());
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", "Type 'help' for available commands, 'exit' to quit.".dimmed());
    show_tree(&nav, Some(1), false, false, false);
    
    loop {
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
        
        // Expand all aliases in the entire line
        let expanded_commands = expand_command_line(line);
        
        // Execute each expanded command in sequence
        let mut should_exit = false;
        for cmd in expanded_commands {
            match execute_command(&cmd, &mut nav) {
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
    }
    
    if Config::global_get_history_enabled() {
        if let Some(history_path) = Config::global().read().unwrap().resolve_history_path() {
            let _ = rl.save_history(&history_path);
        }
    }
    
    Ok(())
}

// ============================================================================
// Command execution
// ============================================================================

fn validate_alias_name(name: &str) -> bool {
    let reserved_commands = [
        "go", "cd", "godrive", "god", "back", "b", "view", "txt", "html", "json", "md",
        "seto", "setd", "setl", "sett", "seth", "watch", "clear", "version", "where",
        "gos", "gosc", "ral", "run", "r", "showcg", "help", "exit", "quit", "ignored",
        "ignore", "cared", "ignoref", "caref", "ignoren", "caren", "size", "tp", 
        "opencg", "resetcg", "restorecg", "gencg", "esc" , "bkup" , "pldw" , "unpd" , 
        "fs" , "ds"
    ];
    
    if name.contains('@') || name.contains('#') {
        return false;
    }
    
    if reserved_commands.contains(&name) {
        return false;
    }
    
    true
}

/// Execute a single command (already fully expanded)
fn execute_command(input: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").trim();

    match cmd.as_str() {
        "go" | "cd" => {
            if args.is_empty() {
                println!("Usage: go <directory_path>");
                println!("       go to <tp_name>      Teleport to savepoint");
                println!("Example: go C:\\Users");
                println!("         go subdir");
                println!("         go to work         # Teleport to 'work' savepoint");
            } else {
                // Check for "go to <tp_name>" syntax
                let parts: Vec<&str> = args.split_whitespace().collect();
                if parts.len() >= 2 && parts[0].to_lowercase() == "to" {
                    // go to <tp_name> - teleport
                    let tp_name = parts[1];
                    if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
                        TeleportManager::jump_by_name(nav, tp_name)?;
                        clear_screen();
                        show_tree(nav, Some(1), false, false, false);
                    } else {
                        print_error(&format!("Teleport point not found: '{}'", tp_name));
                        println!("Use 'tp list' to see all savepoints.");
                    }
                } else {
                    // Normal directory navigation
                    nav.go_to(Path::new(args))?;
                    clear_screen();
                    print_success(&format!("Navigated to: {}", nav.display_path()));
                    show_tree(nav, Some(1), false, false, false);
                }
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
            let mut show_sizes = false;
            let mut depth_override: Option<usize> = None;
            
            let parts: Vec<&str> = args.split_whitespace().collect();
            let mut i = 0;
            while i < parts.len() {
                match parts[i] {
                    "-s" | "--size" => show_sizes = true,
                    "-d" | "--depth" => {
                        if i + 1 < parts.len() {
                            if let Ok(depth) = parts[i + 1].parse::<usize>() {
                                depth_override = Some(depth);
                                i += 1; // skip the value
                            } else {
                                print_error(&format!("Invalid depth: {}", parts[i + 1]));
                            }
                        }
                    }
                    _ => {
                        print_error(&format!("Unknown view option: {}", parts[i]));
                        println!("Usage: view [-s|--size] [-d|--depth <n>]");
                        return Ok(false);
                    }
                }
                i += 1;
            }
            
            let max_depth = depth_override.unwrap_or_else(|| Config::global_get_max_depth());
            show_tree(nav, Some(max_depth), true, true, show_sizes);
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
            }
        }

        "txtc" => {
            if args.is_empty() {
                show_file_selection_menu(nav, true)?;
            } else {
                let target = Path::new(args);
                if target.is_file() {
                    if is_supported_format(target) {
                        let show_lines = Config::global_get_show_line_numbers();
                        let content = crate::output::cat_file_with_line_numbers(target, show_lines)?;
                        
                        #[cfg(target_os = "android")]
                        {
                            // On Android, show file path for Neovim integration
                            print_info(&format!("Copying '{}' to clipboard...", target.display()));
                            match crate::output::copy_to_clipboard(&content, "TXT") {
                                Ok(()) => {
                                    // Success message already printed by copy_to_clipboard
                                }
                                Err(e) => {
                                    print_error(&format!("Failed to copy: {}", e));
                                    // Alternative: save to file
                                    let output_file = crate::output::build_output_path(&format!("copied_{}.txt", 
                                        target.file_name().unwrap_or_default().to_string_lossy()));
                                    if let Ok(()) = crate::output::write_file(&output_file, &content) {
                                        print_success(&format!("File content saved to: {}", output_file.display()));
                                        print_info(&format!("You can open this in Neovim with: :edit {}", output_file.display()));
                                    }
                                }
                            }
                        }
                        
                        #[cfg(not(target_os = "android"))]
                        {
                            crate::output::copy_to_clipboard(&content, "TXT")?;
                            print_success(&format!("File '{}' copied to clipboard!", target.display()));
                        }
                    } else {
                        print_warning(&format!("Skipped (not support format): {}", args));
                    }
                } else {
                    print_error(&format!("File not found: {}", args));
                }
            }
        }

        "txtf" => {
            if args.is_empty() {
                show_file_selection_menu(nav, false)?;
            } else {
                let target = Path::new(args);
                if target.is_file() {
                    if is_supported_format(target) {
                        let show_lines = Config::global_get_show_line_numbers();
                        cat_file(target, show_lines)?;
                    } else {
                        print_warning(&format!("Skipped (not support format): {}", args));
                    }
                } else {
                    print_error(&format!("File not found: {}", args));
                }
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
            
            if config_path.exists() {
                println!("  {}", "✓ Config file exists".green());
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
            show_tree(nav, Some(1), false, false, false);
        }

        "ral" => {
            if args.is_empty() {
                println!("{}", "Run Alias (ral) Commands:".cyan().bold());
                println!("  ral add <name> <command>          Create a new run alias");
                println!("  ral add <name>(x) <command>       Create a parameterised alias (use $x in command)");
                println!("  ral edit <name> <command>         Update an existing alias");
                println!("  ral rnm <old> to <new>            Rename an alias");
                println!("  ral rm <name>                     Remove an alias");
                println!("  ral list                          Show all aliases");
                println!("  ral cls                           Clear ALL aliases (asks confirmation)");
                println!();
                println!("{}", "Examples:".green());
                println!("  ral add btr \"cargo build --release\"");
                println!("  ral rnm btr to build");
                println!("  ral add py \"python test.py\"");
                println!("  ral edit py \"python main.py\"");
                println!("  ral add run_file(x) \"python $x.py\"");
                println!("  ral list");
                println!("  ral rm py");
                println!();
                println!("{}", "Usage with run:".green());
                println!("  run btr              # Executes: cargo build --release");
                println!("  run py               # Executes: python test.py");
                println!("  run_file(hello)      # Executes: python hello.py");
            } else {
                let parts: Vec<&str> = args.splitn(2, ' ').collect();
                let subcmd = parts[0].to_lowercase();
                let subargs = parts.get(1).unwrap_or(&"").trim();
                
                match subcmd.as_str() {
                    "add" => {
                        if subargs.is_empty() {
                            print_error("Usage: ral add <name> <command>");
                            println!("Example: ral add btr \"cargo build --release\"");
                            println!("Example: ral add py(x) \"python $x.py\"");
                        } else {
                            let add_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                            if add_parts.len() < 2 {
                                print_error("Usage: ral add <name> <command>");
                                println!("Example: ral add btr \"cargo build --release\"");
                                println!("Example: ral add py(x) \"python $x.py\"");
                            } else {
                                let raw_name = add_parts[0];
                                let mut command = add_parts[1].to_string();
                                if command.starts_with('"') && command.ends_with('"') && command.len() >= 2 {
                                    command = command[1..command.len()-1].to_string();
                                }
                                // Strip any (param) signature from the alias name — store only the base name.
                                // e.g. `py(x)` -> stored as `py`, command keeps `$x` as the placeholder.
                                let (base_name, param_hint) = parse_call_syntax(raw_name);
                                let name = base_name;
                                if !validate_alias_name(name) {
                                    print_error(&format!("Invalid alias name: '{}'", name));
                                    println!("Alias names cannot:");
                                    println!("  - Start with @ or #");
                                    println!("  - Be a reserved command (go, view, txt, etc.)");
                                    return Ok(false);
                                }
                                let _ = Config::local_add_run_alias(name, &command);
                                Config::reload_global();
                                if param_hint.is_some() {
                                    println!("  Now you can run: {}({})", name.green(), "<arg>".cyan());
                                } else {
                                    println!("  Now you can run: {}", name.green());
                                }
                            }
                        }
                    }
                    "edit" => {
                        if subargs.is_empty() {
                            print_error("Usage: ral edit <name> <new_command>");
                            println!("Example: ral edit py \"python main.py\"");
                            println!("Example: ral edit py(x) \"python $x.py\"");
                        } else {
                            let edit_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                            if edit_parts.len() < 2 {
                                print_error("Usage: ral edit <name> <new_command>");
                                println!("Example: ral edit py \"python main.py\"");
                                println!("Example: ral edit py(x) \"python $x.py\"");
                            } else {
                                // Strip (param) from name just like `add`
                                let (base_name, _) = parse_call_syntax(edit_parts[0]);
                                let name = base_name;
                                let mut command = edit_parts[1].to_string();
                                if command.starts_with('"') && command.ends_with('"') && command.len() >= 2 {
                                    command = command[1..command.len()-1].to_string();
                                }
                                if !validate_alias_name(name) {
                                    print_error(&format!("Invalid alias name: '{}'", name));
                                    return Ok(false);
                                }
                                let _ = Config::local_update_run_alias(name, &command);
                                Config::reload_global();
                            }
                        }
                    }
                    "rm" | "remove" => {
                        if subargs.is_empty() {
                            print_error("Usage: ral rm <name>");
                            println!("Example: ral rm py");
                        } else {
                            let name = subargs.trim();
                            let _ = Config::local_remove_run_alias(name);
                            Config::reload_global();
                        }
                    }
                    "cls" | "clear" => {
                        let aliases = Config::global_get_run_aliases();
                        if aliases.is_empty() {
                            print_info("No run aliases to clear.");
                            return Ok(false);
                        }
                        
                        // Check for --force flag
                        let force = subargs == "--force" || subargs == "-f";
                        
                        if !force {
                            // Show warning and ask for confirmation
                            println!();
                            println!("{}", "⚠️  WARNING: This will delete ALL run aliases!".yellow().bold());
                            println!("{}", format!("You have {} alias(es) defined:", aliases.len()).yellow());
                            
                            let is_local = Config::get_local_config_path().is_some();
                            if is_local {
                                println!("{}", "  (local config only - global aliases are safe)".dimmed());
                            }
                            
                            let mut sorted: Vec<_> = aliases.keys().collect();
                            sorted.sort();
                            for (i, name) in sorted.iter().take(10).enumerate() {
                                println!("  {}. {}", i + 1, name);
                            }
                            if aliases.len() > 10 {
                                println!("  ... and {} more", aliases.len() - 10);
                            }
                            
                            println!();
                            print!("{} ", "Are you sure? Type 'yes' to confirm: ".red());
                            io::stdout().flush()?;
                            
                            let mut confirm = String::new();
                            io::stdin().read_line(&mut confirm)?;
                            let confirm = confirm.trim().to_lowercase();
                            
                            if confirm != "yes" && confirm != "y" {
                                println!("{}", "Clear cancelled.".dimmed());
                                return Ok(false);
                            }
                        }
                        
                        let _ = Config::local_clear_run_aliases();
                        Config::reload_global();
                    }
                    "list" | "ls" => {
                        let (aliases, is_local) = Config::get_run_aliases_with_source();
                        if aliases.is_empty() {
                            print_info("No run aliases defined. Use 'ral add <name> <command>'");
                        } else {
                            println!();
                            println!("{}", "==================================================".cyan());
                            if is_local {
                                println!("{}", "📌 Run Aliases (from local ntconfig.toml)".cyan().bold());
                                if let Some(path) = Config::get_local_config_path() {
                                    println!("{}", format!("   Config: {}", path.display()).dimmed());
                                }
                            } else {
                                println!("{}", "📌 Run Aliases (global)".cyan().bold());
                            }
                            println!("{}", "==================================================".cyan());
                            let mut sorted: Vec<_> = aliases.iter().collect();
                            sorted.sort_by(|a, b| a.0.cmp(b.0));
                            for (i, (name, cmd)) in sorted.iter().enumerate() {
                                let is_valid = validate_alias_name(name);
                                let name_display = if is_valid {
                                    name.blue()
                                } else {
                                    format!("{} (INVALID)", name).red()
                                };
                                // Show whether the alias has param placeholders
                                let has_params = cmd.contains('$');
                                let cmd_display = if has_params {
                                    format!("{} {}", cmd.dimmed(), "(parameterised)".cyan())
                                } else {
                                    cmd.dimmed().to_string()
                                };
                                println!("  {}. {} -> {}", 
                                    (i + 1).to_string().yellow(), 
                                    name_display, 
                                    cmd_display);
                            }
                            println!();
                            println!("{}", "Usage: <alias>  or  <alias>(<arg>) for parameterised aliases".green());
                            if is_local {
                                println!("{}", "💡 Tip: These aliases are project-specific (saved in ntconfig.toml)".dimmed());
                            }
                        }
                    }
                    "rnm" | "rename" => {
                        if subargs.is_empty() {
                            print_error("Usage: ral rnm <old_name> to <new_name>");
                            println!("Example: ral rnm btr to build");
                        } else {
                            let rnm_parts: Vec<&str> = subargs.splitn(3, ' ').collect();
                            if rnm_parts.len() < 3 || rnm_parts[1].to_lowercase() != "to" {
                                print_error("Usage: ral rnm <old_name> to <new_name>");
                                println!("Example: ral rnm btr to build");
                            } else {
                                let old_name = rnm_parts[0];
                                let new_name = rnm_parts[2];
                                if !validate_alias_name(new_name) {
                                    print_error(&format!("Invalid alias name: '{}'", new_name));
                                    return Ok(false);
                                }
                                let aliases = Config::global_get_run_aliases();
                                if let Some(command) = aliases.get(&old_name.to_lowercase()) {
                                    let command = command.clone();
                                    let _ = Config::local_remove_run_alias(old_name);
                                    let _ = Config::local_add_run_alias(new_name, &command);
                                    Config::reload_global();
                                    print_success(&format!("Renamed alias '{}' to '{}'", old_name, new_name));
                                } else {
                                    print_error(&format!("Alias '{}' not found", old_name));
                                }
                            }
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
                println!("  run btr                     # Run alias");
                println!("  run py test.py              # Run alias with args");
                println!("  run_file(hello)             # Run parameterised alias");
            } else {
                let expanded_parts = expand_command_line(args);
                for cmd in expanded_parts {
                    execute_system_command(&cmd, nav.current_path())?;
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

        "opencg" | "editcfg" => {
            let config_path = dirs::config_dir()
                .map(|d| d.join("ntc").join("config.toml"))
                .unwrap_or_else(|| PathBuf::from("ntc_config.toml"));
            
            // Create config if it doesn't exist
            if !config_path.exists() {
                if let Some(parent) = config_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let default_config = Config::new();
                if let Ok(toml) = toml::to_string_pretty(&default_config) {
                    let _ = std::fs::write(&config_path, toml);
                }
                print_info(&format!("Created config file at: {}", config_path.display()));
            }
            
            // Try to use $EDITOR environment variable first
            let editor = std::env::var("EDITOR").or_else(|_| std::env::var("VISUAL"));
            
            match editor {
                Ok(editor_cmd) => {
                    let parts: Vec<&str> = editor_cmd.split_whitespace().collect();
                    if parts.is_empty() {
                        open_with_fallback(&config_path);
                    } else {
                        let mut cmd = std::process::Command::new(parts[0]);
                        for arg in &parts[1..] {
                            cmd.arg(arg);
                        }
                        cmd.arg(&config_path);
                        
                        match cmd.status() {
                            Ok(_) => print_success(&format!("Opening config: {}", config_path.display())),
                            Err(_) => open_with_fallback(&config_path),
                        }
                    }
                }
                Err(_) => open_with_fallback(&config_path),
            }
        }

        "resetcg" | "reset-config" => {
            let config_path = dirs::config_dir()
                .map(|d| d.join("ntc").join("config.toml"))
                .unwrap_or_else(|| PathBuf::from("ntc_config.toml"));
            
            // Check if config exists
            if !config_path.exists() {
                print_warning("Config file not found. Nothing to reset.");
                return Ok(false);
            }
            
            // Show current config location and warn user
            println!();
            println!("{}", "⚠️  CONFIG RESET WARNING".red().bold());
            println!("{}", "═".repeat(50).red());
            println!("Config file: {}", config_path.display().to_string().yellow());
            println!();
            println!("{}", "This will RESET your configuration to DEFAULT values:".yellow());
            println!("  • Output path     → Desktop");
            println!("  • Max depth       → 2");
            println!("  • Line numbers    → OFF");
            println!("  • Threads         → 4");
            println!("  • History         → OFF");
            println!("  • File watcher    → OFF");
            println!("  • Teleports       → Cleared");
            println!("  • Run aliases     → Cleared");
            println!("  • Ignored dirs    → target, build, venv, node_modules, installer, logs, .git");
            println!("  • Custom ignores  → Removed");
            println!();
            
            // Backup option
            println!("{}", "Options:".cyan().bold());
            println!("  • Type 'yes'     - Reset config (no backup)");
            println!("  • Type 'backup'  - Create backup before reset");
            println!("  • Type 'no'      - Cancel");
            println!();
            print!("{} ", "Your choice: ".green());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "backup" => {
                    // Create backup with timestamp
                    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                    let backup_path = config_path.with_file_name(format!(
                        "config_{}.toml.bak",
                        timestamp
                    ));
                    
                    match std::fs::copy(&config_path, &backup_path) {
                        Ok(_) => print_success(&format!("Backup created: {}", backup_path.display())),
                        Err(e) => {
                            print_error(&format!("Failed to create backup: {}", e));
                            return Ok(false);
                        }
                    }
                    
                    // Now reset
                    let default_config = Config::new();
                    if let Ok(toml) = toml::to_string_pretty(&default_config) {
                        match std::fs::write(&config_path, toml) {
                            Ok(_) => {
                                print_success("Config reset to defaults!");
                                print_info("Backup saved. Use 'restorecg' to restore if needed.");
                                
                                // Reload config in memory
                                let mut cfg = Config::global().write().unwrap();
                                *cfg = Config::load();
                                drop(cfg);
                            }
                            Err(e) => print_error(&format!("Failed to write config: {}", e)),
                        }
                    } else {
                        print_error("Failed to serialize default config");
                    }
                }
                "yes" | "y" => {
                    // Reset without backup
                    let default_config = Config::new();
                    if let Ok(toml) = toml::to_string_pretty(&default_config) {
                        match std::fs::write(&config_path, toml) {
                            Ok(_) => {
                                print_success("Config reset to defaults!");
                                
                                // Reload config in memory
                                let mut cfg = Config::global().write().unwrap();
                                *cfg = Config::load();
                                drop(cfg);
                            }
                            Err(e) => print_error(&format!("Failed to write config: {}", e)),
                        }
                    } else {
                        print_error("Failed to serialize default config");
                    }
                }
                _ => {
                    println!("{}", "Reset cancelled.".dimmed());
                }
            }
        }

        "restorecg" | "restore-config" => {
            let config_dir = dirs::config_dir()
                .map(|d| d.join("ntc"))
                .unwrap_or_else(|| PathBuf::from("."));
            
            // Find all backup files
            let mut backups: Vec<PathBuf> = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&config_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("config_") && name.ends_with(".toml.bak") {
                            backups.push(path);
                        }
                    }
                }
            }
            
            if backups.is_empty() {
                print_info("No backup files found.");
                return Ok(false);
            }
            
            // Sort by modified time (newest first)
            backups.sort_by(|a, b| {
                let a_time = std::fs::metadata(a).and_then(|m| m.modified()).ok();
                let b_time = std::fs::metadata(b).and_then(|m| m.modified()).ok();
                b_time.cmp(&a_time)
            });
            
            println!();
            println!("{}", "📋 Available Backups:".cyan().bold());
            for (i, backup) in backups.iter().enumerate() {
                if let Ok(metadata) = std::fs::metadata(backup) {
                    if let Ok(time) = metadata.modified() {
                        let datetime: chrono::DateTime<chrono::Local> = time.into();
                        println!("  {}. {} ({})", 
                            i + 1,
                            backup.file_name().unwrap_or_default().to_string_lossy(),
                            datetime.format("%Y-%m-%d %H:%M:%S")
                        );
                    } else {
                        println!("  {}. {}", i + 1, backup.file_name().unwrap_or_default().to_string_lossy());
                    }
                }
            }
            println!("  {}", "0. Cancel".red());
            println!();
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input == "0" || input.is_empty() {
                println!("{}", "Restore cancelled.".dimmed());
                return Ok(false);
            }
            
            match input.parse::<usize>() {
                Ok(num) if num > 0 && num <= backups.len() => {
                    let backup_path = &backups[num - 1];
                    let config_path = config_dir.join("config.toml");
                    
                    // Create a backup of current config before restoring (just in case)
                    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                    let current_backup = config_dir.join(format!("config_before_restore_{}.toml.bak", timestamp));
                    let _ = std::fs::copy(&config_path, &current_backup);
                    
                    // Restore from selected backup
                    match std::fs::copy(backup_path, &config_path) {
                        Ok(_) => {
                            print_success(&format!("Restored config from: {}", backup_path.file_name().unwrap_or_default().to_string_lossy()));
                            print_info(&format!("Current config backed up to: {}", current_backup.file_name().unwrap_or_default().to_string_lossy()));
                            
                            // Reload config in memory
                            let mut cfg = Config::global().write().unwrap();
                            *cfg = Config::load();
                            drop(cfg);
                            
                            print_success("Config reloaded! Run 'showcg' to see the restored settings.");
                        }
                        Err(e) => print_error(&format!("Failed to restore: {}", e)),
                    }
                }
                _ => {
                    print_error("Invalid selection.");
                }
            }
        }

        "gencg" | "gen-config" | "gen-ntconfig" => {
            let current_dir = nav.current_path();
            let ntconfig_path = current_dir.join("ntconfig.toml");
            
            // Check for --all or -a flag
            let export_all = args == "--all" || args == "-a";
            
            if ntconfig_path.exists() {
                println!();
                println!("{}", "⚠️  ntconfig.toml already exists!".yellow().bold());
                println!("Path: {}", ntconfig_path.display().to_string().cyan());
                println!();
                print!("{} ", "Overwrite? (y/N): ".red());
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim().to_lowercase();
                
                if input != "y" && input != "yes" {
                    println!("{}", "Generation cancelled.".dimmed());
                    return Ok(false);
                }
            }
            
            let toml_content = if export_all {
                // Export ALL current settings as active config with proper TOML syntax
                let current_cfg = Config::global().read().unwrap();
                
                // Helper function to format HashSet as TOML array
                fn format_toml_array(items: &std::collections::HashSet<String>) -> String {
                    if items.is_empty() {
                        return "[]".to_string();
                    }
                    let mut sorted: Vec<&String> = items.iter().collect();
                    sorted.sort();
                    let mut result = String::new();
                    result.push_str("[\n");
                    for (i, item) in sorted.iter().enumerate() {
                        let comma = if i == sorted.len() - 1 { "" } else { "," };
                        result.push_str(&format!("    \"{}\"{}\n", item, comma));
                    }
                    result.push_str("]");
                    result
                }
                
                // Format ignored_directory_names
                let ignored_dirs_str = format_toml_array(&current_cfg.ignored_directory_names);
                
                // Format ignored_extensions
                let ignored_exts_str = format_toml_array(&current_cfg.ignored_extensions);
                
                // Format extra_supported_extensions
                let extra_exts_str = format_toml_array(&current_cfg.extra_supported_extensions);
                
                // Format ignored_files
                let ignored_files_str = format_toml_array(&current_cfg.ignored_files);
                
                // Format extra_supported_files
                let extra_files_str = format_toml_array(&current_cfg.extra_supported_files);
                
                // Build run_aliases TOML section
                let mut run_aliases_toml = String::new();
                if !current_cfg.run_aliases.is_empty() {
                    run_aliases_toml.push_str("\n# Run aliases for this project\n");
                    run_aliases_toml.push_str("[run_aliases]\n");
                    let mut sorted_aliases: Vec<_> = current_cfg.run_aliases.iter().collect();
                    sorted_aliases.sort_by(|a, b| a.0.cmp(b.0));
                    for (name, cmd) in sorted_aliases {
                        run_aliases_toml.push_str(&format!("{} = \"{}\"\n", name, cmd.replace('"', "\\\"")));
                    }
                } else {
                    run_aliases_toml.push_str("\n# Run aliases for this project\n");
                    run_aliases_toml.push_str("# [run_aliases]\n");
                    run_aliases_toml.push_str("# test = \"cargo test\"\n");
                    run_aliases_toml.push_str("# build = \"cargo build --release\"\n");
                }
                
                format!(
                    r#"# ntc local configuration file
# This file overrides global ignore/care and run alias settings for this directory only
# Remove or comment out any line to use the global/default value

# Ignored directories (case-insensitive)
ignored_directory_names = {}

# Ignored file extensions
ignored_extensions = {}

# Extra supported extensions (treat as text files)
extra_supported_extensions = {}

# Ignored specific files
ignored_files = {}

# Extra supported specific files
extra_supported_files = {}
{}"#,
                    ignored_dirs_str,
                    ignored_exts_str,
                    extra_exts_str,
                    ignored_files_str,
                    extra_files_str,
                    run_aliases_toml
                )
            } else {
                // Generate a commented template (guide only, no active settings)
                r#"# ntc local configuration file
# Place this file in any directory to override global ignore/care and run alias settings
# 
# INSTRUCTIONS:
# 1. Remove the '#' from lines you want to enable
# 2. Add your project-specific values
# 3. Run 'ntc' in this directory to activate
#
# For quick setup, run: gencg --all  (copies your current global settings)

# Ignored directories (case-insensitive)
# ignored_directory_names = [
#     "target",
#     "node_modules",
#     ".git",
# ]

# Ignored file extensions
# ignored_extensions = [
#     "log",
#     "tmp",
#     "bak",
# ]

# Extra supported extensions (treat as text files)
# extra_supported_extensions = [
#     "myext",
#     "custom",
# ]

# Ignored specific files
# ignored_files = [
#     "Cargo.lock",
#     "package-lock.json",
# ]

# Extra supported specific files
# extra_supported_files = [
#     ".env",
#     "Dockerfile",
# ]

# Run aliases for this project only
# [run_aliases]
# test = "cargo test"
# build = "cargo build --release"
# fb = "flutter clean && flutter pub get"
# py = "python $x.py"   # parameterised: call as py(filename)
"#
                .to_string()
            };
            
            match std::fs::write(&ntconfig_path, toml_content) {
                Ok(_) => {
                    if export_all {
                        print_success(&format!("Created ntconfig.toml with current settings in: {}", current_dir.display()));
                        println!();
                        println!("{}", "✅ Active configuration exported with valid TOML syntax:".green());
                        println!("  • Ignored directories/extensions/files");
                        println!("  • Extra supported files/extensions");
                        println!("  • Run aliases");
                        Config::reload_global();
                    } else {
                        print_success(&format!("Created ntconfig.toml template in: {}", current_dir.display()));
                        println!();
                        println!("{}", "📝 This is a COMMENTED TEMPLATE - no active settings yet".yellow());
                        println!("{}", "   Edit the file and remove '#' from lines you want to enable".dimmed());
                        println!("{}", "   Or run: gencg --all  to export your current settings".dimmed());
                    }
                    println!();
                    println!("{}", "💡 Tip: Global settings (teleports, output path, depth) stay global".dimmed());
                }
                Err(e) => print_error(&format!("Failed to create ntconfig.toml: {}", e)),
            }
            
            return Ok(false)
        }

        "help" => {
            print_interactive_help();
        }

        "exit" | "quit" | "esc" => {
            return Ok(true);
        }

        "ignored" => {
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
        }
        
        "ignore" => {
            if args.is_empty() {
                println!("Usage: ignore <directory_name>");
            } else {
                let _ = Config::local_add_ignored_dir(args);
                // Reload config to reflect changes
                Config::reload_global();
            }
        }

        "cared" => {
            if args.is_empty() {
                println!("Usage: cared <directory_name>");
            } else {
                let _ = Config::local_remove_ignored_dir(args);
                Config::reload_global();
            }
        }

        "ignoref" => {
            if args.is_empty() {
                println!("Usage: ignoref <extension>");
            } else {
                let _ = Config::local_add_ignored_extension(args);
                Config::reload_global();
            }
        }

        "caref" => {
            if args.is_empty() {
                println!("Usage: caref <extension>");
            } else {
                let _ = Config::local_add_extra_supported_extension(args);
                Config::reload_global();
            }
        }

        "ignoren" => {
            if args.is_empty() {
                println!("Usage: ignoren <filename>");
            } else {
                let _ = Config::local_add_ignored_file(args);
                Config::reload_global();
            }
        }

        "caren" => {
            if args.is_empty() {
                println!("Usage: caren <filename>");
            } else {
                let _ = Config::local_add_extra_supported_file(args);
                Config::reload_global();
            }
        }

        "size" => {
            let total = crate::explorer::calculate_dir_size(nav.current_path());
            let bytes_str = format!("{}", total);
            let human_str = crate::explorer::human_readable_size(total);
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
        }

        "tp" => {
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
                    "list" | "ls" => {
                        TeleportManager::list()?;
                    }
                    "rm" => {
                        if subargs.is_empty() {
                            println!("Usage: tp rm <name|number>");
                        } else if let Ok(num) = subargs.parse::<usize>() {
                            TeleportManager::remove_by_index(num)?;
                        } else {
                            TeleportManager::remove_by_name(subargs)?;
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
        }

        // ============================================================================
        // Backup commands — paste these arms into execute_command() in shell.rs
        // Also add "bkup", "pldw", "unpd" to:
        //   1. validate_alias_name() reserved_commands array
        //   2. The `_ =>` fallthrough ntc-command match list
        // ============================================================================

        "bkup" => {
            match args {
                "--where" | "-w" => {
                    crate::backup::BackupManager::show_backup_location(nav.current_path());
                }

                "--cls" | "--clear" => {
                    // Show how many backups exist before asking for confirmation
                    let backups = crate::backup::BackupManager::list_backups(nav.current_path())?;
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
                        crate::backup::BackupManager::clear_backups(nav.current_path())?;
                    } else {
                        println!("{}", "Clear cancelled.".dimmed());
                    }
                }

                "--force" | "-f" => {
                    // Non-interactive clear (for scripting via ral aliases)
                    crate::backup::BackupManager::clear_backups(nav.current_path())?;
                }

                "" => {
                    crate::backup::BackupManager::create_backup(nav.current_path())?;
                }

                _ => {
                    print_error(&format!("Unknown bkup option: {}", args));
                    println!("Usage:");
                    println!("  bkup              Create a new backup");
                    println!("  bkup --where      Show backup storage location");
                    println!("  bkup --cls        Delete all backups (asks confirmation)");
                    println!("  bkup --force      Delete all backups (no confirmation)");
                }
            }
        }

        "pldw" => {
            if args.is_empty() {
                // Interactive restore menu
                let backups = crate::backup::BackupManager::list_backups(nav.current_path())?;
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
                        crate::backup::BackupManager::restore_backup(
                            nav.current_path(), backup_num, true
                        )?;
                        clear_screen();
                        show_tree(nav, Some(1), false, false, false);
                    }
                    Ok(_)  => print_error(&format!("Invalid selection: {}", input)),
                    Err(_) => print_error(&format!("Invalid input: {}", input)),
                }
            } else if let Ok(num) = args.parse::<usize>() {
                // Direct restore by backup number (non-interactive confirmation still shown)
                crate::backup::BackupManager::restore_backup(nav.current_path(), num, true)?;
                clear_screen();
                show_tree(nav, Some(1), false, false, false);
            } else {
                print_error(&format!("Invalid argument: {}", args));
                println!("Usage:");
                println!("  pldw              Interactive restore menu");
                println!("  pldw <number>     Restore backup by number");
            }
        }

        "unpd" => {
            match args {
                "--cls" | "--clear" => {
                    println!();
                    print!("{} ", "⚠ Clear undo history? This cannot be undone. (y/N):".red());
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                        crate::backup::BackupManager::clear_undo_history(nav.current_path())?;
                    } else {
                        println!("{}", "Clear cancelled.".dimmed());
                    }
                }

                "--force" | "-f" => {
                    // Non-interactive clear (consistent with bkup --force / ral cls --force)
                    crate::backup::BackupManager::clear_undo_history(nav.current_path())?;
                }

                "" => {
                    crate::backup::BackupManager::undo_last_restore(nav.current_path())?;
                    clear_screen();
                    show_tree(nav, Some(1), false, false, false);
                }

                _ => {
                    print_error(&format!("Unknown unpd option: {}", args));
                    println!("Usage:");
                    println!("  unpd              Undo the last restore");
                    println!("  unpd --cls        Clear undo history (asks confirmation)");
                    println!("  unpd --force      Clear undo history (no confirmation)");
                }
            }
        }

        // File search command
        "fs" => {
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
        }

        // Directory search command
        "ds" => {
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
        }

        _ => {
            // Check for @teleport shortcut
            if cmd.starts_with('@') && cmd.len() > 1 {
                let tp_name = &cmd[1..];
                if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
                    TeleportManager::jump_by_name(nav, tp_name)?;
                    return Ok(false);
                }
            }
            
            // Not an ntc command – but we still need to expand aliases recursively
            // Get the fully expanded command line (handles && and nested aliases)
            let expanded_parts = expand_command_line(input);
            
            // Execute each part as a system command
            for cmd_part in expanded_parts {
                // Check if this part is actually an ntc command after expansion
                let parts: Vec<&str> = cmd_part.splitn(2, ' ').collect();
                let first_word = parts[0].to_lowercase();
                
                // If it's an ntc command, recurse back into execute_command
                match first_word.as_str() {
                       "go"     | "cd"      | "godrive" | "god"     | "back"      | "b"       | "view" 
                    | "txt"     | "txtc"    | "txtf"    | "html"    | "json"      | "md" 
                    | "seto"    | "setd"    | "setl"    | "sett"    | "seth"      | "watch" 
                    | "clear"   | "version" | "where" 
                    | "gos"     | "gosc"    | "ral"     | "run"     | "r" 
                    | "showcg"  | "help"    | "exit"    | "quit" 
                    | "ignored" | "ignore"  | "cared"   | "ignoref" | "caref"     | "ignoren" | "caren" 
                    | "size"    | "tp"      | "opencg"  | "resetcg" | "restorecg" | "gencg" 
                    | "bkup"    | "pldw"    | "unpd"    | "fs"      | "ds" => {
                        // This is an ntc command – execute it recursively
                        if let Err(e) = execute_command(&cmd_part, nav) {
                            print_error(&format!("{}", e));
                            return Ok(false);
                        }
                    }
                    _ => {
                        // Regular system command
                        if let Err(e) = execute_system_command(&cmd_part, nav.current_path()) {
                            print_error(&format!("{}", e));
                            return Ok(false);
                        }
                    }
                }
            }
            return Ok(false)
        }
    }

    Ok(false)
}

// ============================================================================
// Helper functions (gosc, show_tree, file menus, system command)
// ============================================================================

fn gosc_loop(nav: &mut Navigator) -> Result<()> {
    loop {
        let dirs = nav.list_subdirs()?;
        clear_screen();
        show_tree(nav, Some(1), false, false, false);
        
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
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "0" {
            println!("{}", "Exiting gosc...".dimmed());
            break Ok(());
        }
        
        if input.starts_with('-') {
            let back_str = &input[1..];
            match back_str.parse::<usize>() {
                Ok(n) if n > 0 => {
                    for _ in 0..n {
                        let _ = nav.go_back();
                    }
                }
                Err(_) => {
                    print_error(&format!("Invalid back count: {}", back_str));
                }
                Ok(_) => {}
            }
            continue;
        }
        
        match input.parse::<usize>() {
            Ok(num) => {
                if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
                    let new_path = nav.current_path().join(name);
                    let _ = nav.go_to(&new_path);
                } else {
                    print_error(&format!("Invalid number: {}", num));
                }
            }
            Err(_) => {
                print_error(&format!("Invalid input: {}", input));
            }
        }
    }
}

fn open_with_fallback(config_path: &Path) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", config_path.to_str().unwrap_or("")])
            .status();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(config_path).status();
    }
    #[cfg(target_os = "linux")]
    {
        if std::process::Command::new("xdg-open")
            .arg(config_path)
            .status()
            .is_err()
        {
            for editor in &["vim", "nano","vi"] {
                if std::process::Command::new(editor)
                    .arg(config_path)
                    .status()
                    .is_ok()
                {
                    break;
                }
            }
        }
    }
    print_info(&format!("Config path: {}", config_path.display()));
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
        let total_dirs = count_dirs_in_tree(&tree);
        let scan_pb = ProgressBar::new(total_dirs);
        scan_pb.set_style(
            ProgressStyle::with_template("ScanB  [{bar:30}] {percent}% {msg}")
                .unwrap()
                .progress_chars("=> "),
        );
        scan_pb.set_message("Calculating sizes...");
        crate::explorer::compute_tree_sizes(&mut tree, Some(&scan_pb));  
        let result = format_tree_with_sizes(&tree, "", true, true, Some(&scan_pb));
        scan_pb.finish_and_clear();
        result
    } else {
        format_tree(&tree, "", true)
    };

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

fn show_file_selection_menu(nav: &Navigator, copy_mode: bool) -> Result<()> {
    let files = list_supported_files(nav)?;
    
    println!();
    println!("{}", "==================================================".cyan());
    if copy_mode {
        println!("{}", "📋 Select a file to COPY to clipboard".cyan().bold());
    } else {
        println!("{}", "📄 Select a file to DISPLAY".cyan().bold());
    }
    println!("{}", "==================================================".cyan());
    
    if files.is_empty() {
        println!("  {}", "(no supported files)".dimmed());
    } else {
        for (i, (name, path)) in files.iter().enumerate() {
            let size_str = get_file_size(path);
            println!("  {}. {} {}", 
                (i + 1).to_string().yellow(), 
                name.blue(),
                size_str.dimmed());
        }
    }
    
    println!("  {}", "0. Cancel".red());
    println!();
    print!("{} ", format!("Enter number (1-{}) or 0: ", files.len()).green());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    
    if input == "0" || input.is_empty() {
        println!("{}", "Cancelled.".dimmed());
        return Ok(());
    }
    
    match input.parse::<usize>() {
        Ok(num) if num > 0 && num <= files.len() => {
            let (name, path) = &files[num - 1];
            if copy_mode {
                println!();
                print_info(&format!("Copying '{}' to clipboard...", name));
                let show_lines = Config::global_get_show_line_numbers();
                let content = crate::output::cat_file_with_line_numbers(path, show_lines)?;
                crate::output::copy_to_clipboard(&content, "TXT")?;
                print_success(&format!("File '{}' copied to clipboard!", name));
            } else {
                let show_lines = Config::global_get_show_line_numbers();
                println!();
                cat_file(path, show_lines)?;
            }
        }
        Ok(_) => {
            print_error(&format!("Invalid number: {}", input));
        }
        Err(_) => {
            print_error(&format!("Invalid input: {}", input));
        }
    }
    
    Ok(())
}

fn list_supported_files(nav: &Navigator) -> Result<Vec<(String, PathBuf)>> {
    let fmt_cfg = FormatConfig::from_global();
    let mut files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(nav.current_path()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if crate::filetype::is_supported_format_with_config(&path, &fmt_cfg) {
                        files.push((name.to_string(), path));
                    }
                }
            }
        }
    }
    
    files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    Ok(files)
}

fn get_file_size(path: &Path) -> String {
    if let Ok(metadata) = std::fs::metadata(path) {
        let size = metadata.len();
        if size < 1024 {
            format!("({} B)", size)
        } else if size < 1024 * 1024 {
            format!("({:.1} KB)", size as f64 / 1024.0)
        } else {
            format!("({:.1} MB)", size as f64 / (1024.0 * 1024.0))
        }
    } else {
        String::new()
    }
}

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
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                     ntc {} - Interactive Help                 ║", env!("CARGO_PKG_VERSION"));
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!("{}", "NAVIGATION COMMANDS:".cyan().bold());
    println!("  go <path>           Navigate to a directory");
    println!("  go to <tp_name>     Teleport to a saved teleport point");
    println!("  cd <path>           Same as go");
    println!("  cd to <tp_name>     Teleport to a saved teleport point");
    println!("  gos                 List subdirectories and pick one");
    println!("  gosc                Continuous navigation (0 to exit)");
    println!("  godrive             List and select Windows drives");
    println!("  back                Go back to parent directory");
    println!("  back <n>            Go back n parent directories");

    println!("{}", "CONFIGURATION COMMANDS:".cyan().bold());
    println!("  setO                Show current output path");
    println!("  setO <path>         Set output path");
    println!("  setD                Show current max depth");
    println!("  setD <int>          Set max depth (min: 1, max: 20)");
    println!("  setL                Show line number setting (ON/OFF)");
    println!("  setL ON|OFF         Enable/disable line numbers");
    println!("  setT                Show current thread count");
    println!("  setT <int>          Set number of threads");
    println!("  setH                Show history settings");
    println!("  setH ON|OFF         Enable/disable history");
    println!("  setH <path>         Set custom history file path");
    println!("  setH default        Reset history to default location");
    println!("  showcg              Show current configuration overview");
    println!("  opencg              Open config.toml in default editor");        
    println!("  resetcg             Reset config to defaults (with backup option)");  
    println!("  restorecg           Restore config from backup");              
    println!("  gencg               Create ntconfig.toml template (commented, for manual editing)");
    println!("  gencg --all         Export current settings to ntconfig.toml (active config)");
    println!("  watch ON|OFF        Enable/disable file watcher");

    println!();
    println!("{}", "TELEPORT COMMANDS:".cyan().bold());
    println!("  tp add <name>       Save current location");
    println!("  tp jump <name>      Teleport to saved location");
    println!("  tp list             List all teleport points");
    println!("  tp rm <name>        Remove teleport point");
    println!("  @<name>             Quick teleport shortcut");

    println!();
    println!("{}", "RUN ALIAS COMMANDS:".cyan().bold());
    println!("  ral add <name> \"<command>\"         Create alias");
    println!("  ral add <name>(x) \"<cmd $x>\"       Create parameterised alias");
    println!("  ral edit <name> \"<command>\"        Update alias");
    println!("  ral rnm <old> to <new>             Rename alias");
    println!("  ral rm <name>                      Remove alias");
    println!("  ral list                           Show all aliases");
    println!("  ral cls                            Clear ALL aliases (with confirmation)");
    println!("  <alias>                            Execute alias directly");
    println!("  <alias>(<arg>)                     Execute parameterised alias");
    println!("  run <alias>                        Execute alias with 'run'");
    println!();
    println!("{}", "Examples:".green());
    println!("  ral add dal \"dart analyze lib/\"");
    println!("  ral add frr \"flutter run --release -d RFCW71EGWDW\"");
    println!("  ral add fb \"dal && frr\"");
    println!("  fb                                 # Runs both commands");
    println!("  ral add py(x) \"python $x.py\"");
    println!("  py(hello)                          # Runs: python hello.py");

    println!();
    println!("{}", "VIEW COMMAND:".cyan().bold());
    println!("  view                Show directory tree (uses config depth)");
    println!("  view -s             Show tree with sizes");
    println!("  view --size         Same as -s");
    println!("  view -d <n>         Show tree with custom depth (overrides config)");
    println!("  view --depth <n>    Same as -d");
    println!("  view -s -d <n>      Show tree with sizes and custom depth");

    println!();
    println!("{}", "REPORT COMMANDS:".cyan().bold());
    println!("  txt                 Generate TXT report");
    println!("  json                Generate JSON report");
    println!("  md                  Generate Markdown report");
    println!("  html                Generate HTML report");
    println!("  txtc                Copy file to clipboard");
    println!("  txtf                Display file");

    println!("{}", "BACKUP COMMANDS:".cyan().bold());
    println!("  bkup                Create a backup of current project");
    println!("  bkup --where        Show backup storage location");
    println!("  bkup --cls          Delete ALL backups for this project (asks confirmation)");
    println!("  bkup --force        Delete ALL backups (no confirmation)");
    println!("  pldw                Interactive restore menu");
    println!("  pldw <number>       Restore backup by number");
    println!("  unpd                Undo the last restore");
    println!("  unpd --cls          Clear undo history (asks confirmation)");
    println!("  unpd --force        Clear undo history (no confirmation)");

    println!();
    println!("{}", "OTHER COMMANDS:".cyan().bold());
    println!("  clear               Clear screen");
    println!("  version             Show version");
    println!("  where               Show locations");
    println!("  help                Show this help");
    println!("  exit, quit          Exit ntc");
    println!();
}

fn print_tp_help() {
    println!();
    println!("{}", "Teleport (tp) Commands:".cyan().bold());
    println!("  tp                 Interactive menu");
    println!("  tp add <name>      Save current directory");
    println!("  tp add <name> <path>  Save specific path");
    println!("  tp jump <name>     Teleport by name");
    println!("  tp jump <number>   Teleport by number");
    println!("  tp list            Show all savepoints");
    println!("  tp rm <name>       Remove by name");
    println!("  tp rm <number>     Remove by number");
    println!("  tp rnm <old> to <new>  Rename savepoint");
    println!("  tp cls             Clear ALL savepoints");
    println!("  @<name>            Quick teleport shortcut");
    println!();
}