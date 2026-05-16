use crate::config::Config;
use crate::explorer::{format_tree, format_tree_with_sizes, generate_tree};
use crate::filetype::is_supported_format;
use crate::navigator::{Navigator, clear_screen};
use crate::output::{cat_file, print_error, print_info, print_separator, print_success, print_warning};
use crate::report::{generate_report, ReportFormat};
use crate::watcher;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Launch the interactive shell
pub fn run_shell() -> Result<()> {
    let mut nav = Navigator::new()?;
    let mut rl = DefaultEditor::new().expect("Failed to create line editor");

    let history_path = Config::global().read().unwrap().resolve_history_path();
    if Config::global_get_history_enabled() && !history_path.as_os_str().is_empty() {
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
            if changed.load(Ordering::SeqCst) {
                changed.store(false, Ordering::SeqCst);
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
                        break; // Stop on first error
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
        let history_path = Config::global().read().unwrap().resolve_history_path();
        if !history_path.as_os_str().is_empty() {
            let _ = rl.save_history(&history_path);
        }
    }

    Ok(())
}

/// Execute a single interactive command
fn execute_command(input: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").trim();

    match cmd.as_str() {
        "go" => {
            if args.is_empty() {
                println!("Usage: go <directory_path>");
                println!("Example: go C:\\Users");
                println!("         go subdir");
            } else {
                nav.go_to(Path::new(args))?;
                print_success(&format!("Navigated to: {}", nav.display_path()));
                show_tree(nav, Some(1), false, false, false);
            }
        }

        "godrive" => {
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
                    show_tree(nav, Some(1), false, false, false);
                } else if let Ok(num) = choice.parse::<usize>() {
                    if num > 0 && num <= drives.len() {
                        nav.go_drive(drives[num - 1])?;
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
                show_tree(nav, Some(1), false, false, false);
            }
        }

        "back" => {
            if args.is_empty() {
                match nav.go_back() {
                    Ok(()) => show_tree(nav, Some(1), false, false, false),
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
            if args.is_empty() {
                generate_report(nav.current_path(), ReportFormat::Txt)?;
            } else {
                let target = Path::new(args);
                if target.is_dir() {
                    generate_report(target, ReportFormat::Txt)?;
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
            println!("ntc executable: {}", exe.display().to_string().cyan());
            println!("Current directory: {}", nav.display_path().cyan());
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
                    print_success(&format!("Navigated to: {}", nav.display_path()));
                    show_tree(nav, Some(1), false, false, false);
                } else {
                    print_error("Invalid number.");
                }
            } else {
                print_error("Invalid input.");
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
            let ignored_exts = Config::global_get_ignored_extensions();
            let extra_exts = Config::global_get_extra_supported_extensions();
            let ignored_files = Config::global_get_ignored_files();
            let extra_files = Config::global_get_extra_supported_files();
            println!("{}", "Ignored directories:".yellow());
            for d in &dirs { println!("  - {}", d.red()); }
            println!("{}", "Ignored extensions:".yellow());
            for e in &ignored_exts { println!("  - .{}", e.red()); }
            println!("{}", "Extra supported extensions:".yellow());
            for e in &extra_exts { println!("  - .{}", e.green()); }
            println!("{}", "Ignored files:".yellow());
            for f in &ignored_files { println!("  - {}", f.red()); }
            println!("{}", "Extra supported files:".yellow());
            for f in &extra_files { println!("  - {}", f.green()); }
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

        _ => {
            print_error(&format!("Unknown command: {}", cmd));
            println!("{}", "Type 'help' for available commands.".dimmed());
        }
    }

    Ok(false)
}

fn show_tree(
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

    let tree = generate_tree(
        &nav.current_path().to_string_lossy(),
        max_depth_override,
        include_files,
        tree_pb.as_ref(),
    );

    if let Some(pb) = tree_pb {
        pb.finish_with_message("Done");
    }

    let tree_str = if show_sizes {
        let dir_count = crate::explorer::count_dirs_in_tree(&tree);
        let scan_pb = ProgressBar::new(dir_count);
        scan_pb.set_style(
            ProgressStyle::with_template("ScanB  [{bar:30}] {percent}% {msg}")
                .unwrap()
                .progress_chars("=> "),
        );
        scan_pb.set_message("Calculating sizes...");

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
    println!("  gos                 List subdirectories and pick one to navigate");
    println!("  godrive             List all drives and select one");
    println!("  godrive <letter>    Navigate to a drive (e.g., godrive C)");
    println!("  back                Go back to parent directory");
    println!("  back <n>            Go back n parent directories");

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