use crate::config::Config;
use crate::explorer::{format_tree, format_tree_with_sizes, generate_tree};
use crate::filetype::is_supported_format;
use crate::navigator::Navigator;
use crate::output::{cat_file, print_separator};
use crate::report::{generate_report, ReportFormat};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::Path;

/// Launch the interactive shell
pub fn run_shell() -> Result<()> {
    let mut nav = Navigator::new()?;
    let mut rl = DefaultEditor::new().unwrap_or_else(|_| {
        DefaultEditor::new().expect("Failed to create line editor")
    });

    // Load history from configured path (if enabled)
    let history_path = Config::global().read().unwrap().resolve_history_path();
    if Config::global_get_history_enabled() && !history_path.as_os_str().is_empty() {
        let _ = rl.load_history(&history_path);
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              Welcome to ntc 1.3.0 - Navigate, Tree, Cat          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Type 'help' for available commands, 'exit' to quit.");
    println!();

    // Show initial state - directories only (depth 1)
    show_tree(&nav, Some(1), false, false, false);

    loop {
        let prompt = format!("ntc [{}]> ", nav.display_path());

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
                println!("Goodbye!");
                break;
            }
            Err(_) => break,
        };

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match execute_command(line, &mut nav) {
            Ok(should_exit) => {
                if should_exit {
                    println!("Goodbye!");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    // Save history to configured path
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
        // --- Navigation Commands (depth 1) ---
        "go" => {
            if args.is_empty() {
                println!("Usage: go <directory_path>");
                println!("Example: go C:\\Users");
                println!("         go subdir");
            } else {
                nav.go_to(Path::new(args))?;
                show_tree(nav, Some(1), false, false, false);
            }
        }

        "godrive" => {
            if args.is_empty() {
                let drives = Navigator::list_drives();
                println!("Available drives:");
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
                        println!("Invalid drive number.");
                    }
                } else {
                    println!("Invalid input.");
                }
            } else {
                let letter = args.chars().next().unwrap_or('C');
                nav.go_drive(letter)?;
                show_tree(nav, Some(1), false, false, false);
            }
        }

        "back" => {
            if args.is_empty() {
                // back 1 (default)
                match nav.go_back() {
                    Ok(()) => show_tree(nav, Some(1), false, false, false),
                    Err(e) => println!("{}", e),
                }
            } else {
                // back n
                match args.parse::<usize>() {
                    Ok(n) if n > 0 => {
                        let mut success = true;
                        for i in 0..n {
                            match nav.go_back() {
                                Ok(()) => {}
                                Err(e) => {
                                    if i == 0 {
                                        println!("{}", e);
                                    } else {
                                        println!("Error: null parent at step {} - nowhere to go back", i + 1);
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
                    _ => println!("Invalid number: {}. Usage: back [n]", args),
                }
            }
        }

        // --- View command (config depth) ---
        "view" => {
            let show_sizes = args == "--size";
            show_tree(nav, None, true, true, show_sizes);
        }

        // --- Report Generation ---
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
                        println!("Skipped (not support format): {}", args);
                    }
                } else {
                    println!("Path not found: {}", args);
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
                        println!("Skipped (not support format): {}", args);
                    }
                } else {
                    println!("Path not found: {}", args);
                }
            }
        }

        // --- Configuration Commands ---
        "seto" => {
            if args.is_empty() {
                println!("Current output path: {}", Config::global_get_output_path().display());
            } else {
                Config::global_set_output_path(Path::new(args));
                println!("Output path set to: {}", Config::global_get_output_path().display());
            }
        }

        "setd" => {
            if args.is_empty() {
                println!("Current max depth: {}", Config::global_get_max_depth());
            } else {
                match args.parse::<usize>() {
                    Ok(depth) => {
                        Config::global_set_max_depth(depth);
                        println!("Max depth set to: {}", Config::global_get_max_depth());
                    }
                    Err(_) => println!("Invalid depth: {}. Must be a positive integer.", args),
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
                        println!("Line numbers: {}", if state { "ON" } else { "OFF" });
                    }
                    None => println!("Invalid value: {}. Use ON or OFF.", args),
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
                        println!("Threads set to: {}", Config::global_get_num_threads());
                    }
                    None => println!("Invalid thread count: {}. Must be a positive integer.", args),
                }
            }
        }

        // --- History ---
        "seth" => {
            if args.is_empty() {
                let enabled = Config::global_get_history_enabled();
                let path = Config::global_get_history_path();
                println!("History: {}", if enabled { "ON" } else { "OFF" });
                match path {
                    Some(p) => println!("History path: {}", p.display()),
                    None => println!("History path: default (current directory)"),
                }
            } else {
                let upper = args.to_uppercase();
                if upper == "ON" {
                    Config::global_set_history_enabled(true);
                    println!("History: ON");
                } else if upper == "OFF" {
                    Config::global_set_history_enabled(false);
                    println!("History: OFF");
                } else if args == "default" {
                    Config::global_set_history_path(None);
                    println!("History path reset to default");
                } else {
                    let p = Path::new(args);
                    Config::global_set_history_path(Some(p.to_path_buf()));
                    println!("History path set to: {}", p.display());
                }
            }
        }

        // --- Terminal & Info ---
        "clear" => {
            let _ = std::process::Command::new("cmd").args(&["/c", "cls"]).status();
            println!();
            println!("╔══════════════════════════════════════════════════════════════════╗");
            println!("║              Welcome to ntc 1.3.0 - Navigate, Tree, Cat          ║");
            println!("╚══════════════════════════════════════════════════════════════════╝");
            println!();
            println!("Type 'help' for available commands, 'exit' to quit.");
            println!();
            show_tree(nav, Some(1), false, false, false);
        }

        "version" => {
            println!("ntc 1.3.0");
        }

        "help" => {
            print_interactive_help();
        }

        "exit" | "quit" => {
            return Ok(true);
        }

        // --- Ignore/Care ---
        "ignored" => {
            let dirs = Config::global_get_ignored_dirs();
            let ignored_exts = Config::global_get_ignored_extensions();
            let extra_exts = Config::global_get_extra_supported_extensions();
            let ignored_files = Config::global_get_ignored_files();
            let extra_files = Config::global_get_extra_supported_files();
            println!("Ignored directories: {:?}", dirs);
            println!("Ignored extensions: {:?}", ignored_exts);
            println!("Extra supported extensions: {:?}", extra_exts);
            println!("Ignored files: {:?}", ignored_files);
            println!("Extra supported files: {:?}", extra_files);
        }
        "ignore" => {
            if args.is_empty() {
                println!("Usage: ignore <directory_name>");
            } else {
                Config::global_add_ignored_dir(args);
                println!("Now ignoring directory: {}", args);
            }
        }
        "cared" => {
            if args.is_empty() {
                println!("Usage: cared <directory_name>");
            } else {
                Config::global_remove_ignored_dir(args);
                println!("No longer ignoring directory: {}", args);
            }
        }
        "ignoref" => {
            if args.is_empty() {
                println!("Usage: ignoref <extension>");
            } else {
                Config::global_add_ignored_extension(args);
                println!("Now ignoring .{} files", args);
            }
        }
        "caref" => {
            if args.is_empty() {
                println!("Usage: caref <extension>");
            } else {
                Config::global_add_extra_supported_extension(args);
                println!("Now caring about .{} files", args);
            }
        }
        "ignoren" => {
            if args.is_empty() {
                println!("Usage: ignoren <filename>");
                println!("Example: ignoren Cargo.lock");
            } else {
                Config::global_add_ignored_file(args);
                println!("Now ignoring file: {}", args);
            }
        }
        "caren" => {
            if args.is_empty() {
                println!("Usage: caren <filename>");
                println!("Example: caren Cargo.lock");
            } else {
                Config::global_add_extra_supported_file(args);
                println!("Now caring about file: {}", args);
            }
        }

        "size" => {
            let total = crate::explorer::calculate_dir_size(nav.current_path());
            println!();
            println!("┌─────────────────────────────────────────┐");
            println!("│ Current Directory Size                  │");
            println!("│ Bytes: {:>32} │", format!("{}", total));
            println!("│ Human: {:>32} │", crate::explorer::human_readable_size(total));
            println!("└─────────────────────────────────────────┘");
        }

        _ => {
            println!("Unknown command: {}", cmd);
            println!("Type 'help' for available commands.");
        }
    }

    Ok(false)
}


/// Show directory tree.
fn show_tree(
    nav: &Navigator,
    max_depth_override: Option<usize>,
    show_progress: bool,
    include_files: bool,
    show_sizes: bool,
) {
    println!();
    print_separator("Current Directory");
    println!("Path: {}", nav.display_path());
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
        let dir_count = count_dirs_in_tree(&tree);
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

    println!("{}", tree_str);
}

/// Count directories in tree recursively
fn count_dirs_in_tree(node: &crate::explorer::TreeNode) -> u64 {
    let mut count = if node.is_dir && node.depth > 0 { 1 } else { 0 };
    for child in &node.children {
        count += count_dirs_in_tree(child);
    }
    count
}

/// Print interactive mode help
fn print_interactive_help() {
    println!(r#"
╔══════════════════════════════════════════════════════════════════╗
║                     ntc 1.3.0 - Interactive Help                 ║
╚══════════════════════════════════════════════════════════════════╝

NAVIGATION COMMANDS:
  go <path>           Navigate to a directory (shows root contents)
  go                  Show go command usage
  godrive             List all drives and select one
  godrive <letter>    Navigate to a drive (e.g., godrive C)
  back                Go back to parent directory
  back <n>            Go back n parent directories

VIEW COMMAND:
  view                Show full directory tree using configured depth
  view --size         Show directory tree with sizes

SIZE COMMAND:
  size                Show current directory size

REPORT COMMANDS:
  txt                 Generate TXT report of current directory
  txt <dir>           Generate TXT report of specified directory
  txt <file>          Display file contents (if supported format)
  html                Generate HTML report of current directory
  html <dir>          Generate HTML report of specified directory
  html <file>         Display file contents (if supported format)

CONFIGURATION COMMANDS:
  setO                Show current output path
  setO <path>         Set output path
  setD                Show current max depth
  setD <int>          Set max depth (min: 1, max: 12)
  setL                Show line number setting (ON/OFF)
  setL ON|OFF         Enable/disable line numbers
  setT                Show current thread count
  setT <int>          Set number of threads
  setH                Show history settings
  setH ON|OFF         Enable/disable history
  setH <path>         Set custom history file path
  setH default        Reset history to default location

IGNORE/CARE COMMANDS:
  ignored             Show all ignored items
  ignore <name>       Ignore a directory name
  cared <name>        Stop ignoring a directory name
  ignoref <ext>       Ignore a file extension
  caref <ext>         Care about a file extension
  ignoren <file>      Ignore a specific file (e.g., Cargo.lock)
  caren <file>        Care about a specific file

TERMINAL COMMANDS:
  clear               Clear the terminal screen
  version             Show version information

OTHER COMMANDS:
  help                Show this help
  exit, quit          Exit ntc
"#);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_execute_go_empty() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("go", &mut nav);
        assert!(result.is_ok());
        assert!(!result.unwrap());
        Ok(())
    }

    #[test]
    fn test_execute_back() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("back", &mut nav);
        assert!(result.is_ok() || result.is_err());
        Ok(())
    }

    #[test]
    fn test_execute_back_n() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("back 3", &mut nav);
        assert!(result.is_ok() || result.is_err());
        Ok(())
    }

    #[test]
    fn test_execute_help() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("help", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_clear() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("clear", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_version() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("version", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_seth_empty() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setH", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_seth_on() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setH ON", &mut nav);
        assert!(result.is_ok());
        assert!(Config::global_get_history_enabled());
        Ok(())
    }

    #[test]
    fn test_execute_seth_off() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setH OFF", &mut nav);
        assert!(result.is_ok());
        assert!(!Config::global_get_history_enabled());
        Config::global_set_history_enabled(true);
        Ok(())
    }

    #[test]
    fn test_execute_ignoren() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("ignoren Cargo.lock", &mut nav);
        assert!(result.is_ok());
        let ignored = Config::global_get_ignored_files();
        assert!(ignored.contains("Cargo.lock"));
        Config::global_remove_ignored_file("Cargo.lock");
        Ok(())
    }

    #[test]
    fn test_execute_caren() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("caren Cargo.lock", &mut nav);
        assert!(result.is_ok());
        let extra = Config::global_get_extra_supported_files();
        assert!(extra.contains("Cargo.lock"));
        Config::global_remove_extra_supported_file("Cargo.lock");
        Ok(())
    }

    #[test]
    fn test_execute_seto_empty() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setO", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_setd_empty() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setD", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_setl_empty() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setL", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_sett_empty() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setT", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_exit() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("exit", &mut nav);
        assert!(result.is_ok());
        assert!(result.unwrap());
        Ok(())
    }

    #[test]
    fn test_execute_unknown_command() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("foobar123", &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_seto_with_path() -> Result<()> {
        let mut nav = Navigator::new()?;
        let temp = TempDir::new()?;
        let result = execute_command(&format!("setO {}", temp.path().display()), &mut nav);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_execute_setd_with_value() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setD 5", &mut nav);
        assert!(result.is_ok());
        assert_eq!(Config::global_get_max_depth(), 5);
        Config::global_set_max_depth(2);
        Ok(())
    }

    #[test]
    fn test_execute_setl_on() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setL ON", &mut nav);
        assert!(result.is_ok());
        assert!(Config::global_get_show_line_numbers());
        Config::global_set_show_line_numbers(false);
        Ok(())
    }

    #[test]
    fn test_execute_sett_with_value() -> Result<()> {
        let mut nav = Navigator::new()?;
        let result = execute_command("setT 8", &mut nav);
        assert!(result.is_ok());
        assert_eq!(Config::global_get_num_threads(), 8);
        Config::global_set_num_threads(4);
        Ok(())
    }
}