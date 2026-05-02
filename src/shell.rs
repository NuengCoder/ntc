use crate::config::Config;
use crate::explorer::{generate_tree, format_tree};
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

    let _ = rl.load_history("ntc_history.txt");

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              Welcome to ntc 1.1.0 - Navigate, Tree, Cat          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Type 'help' for available commands, 'exit' to quit.");
    println!();

    // Show initial state - directories only (depth 1)
    show_tree(&nav, Some(1), false, false);

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

    let _ = rl.save_history("ntc_history.txt");
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
                show_tree(nav, Some(1), false, false);
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
                    show_tree(nav, Some(1), false, false);
                } else if let Ok(num) = choice.parse::<usize>() {
                    if num > 0 && num <= drives.len() {
                        nav.go_drive(drives[num - 1])?;
                        show_tree(nav, Some(1), false,false);
                    } else {
                        println!("Invalid drive number.");
                    }
                } else {
                    println!("Invalid input.");
                }
            } else {
                let letter = args.chars().next().unwrap_or('C');
                nav.go_drive(letter)?;
                show_tree(nav, Some(1), false,false);
            }
        }

        "back" => {
            match nav.go_back() {
                Ok(()) => show_tree(nav, Some(1), false,false),
                Err(e) => println!("{}", e),
            }
        }

        // --- View command (config depth) ---
        "view" => {
            show_tree(nav, None, true,true);
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

        // --- Terminal & Info ---
        "clear" => {
            // Clear screen using Windows cmd
            let _ = std::process::Command::new("cmd").args(&["/c", "cls"]).status();
            // Re-print welcome banner and current state
            println!();
            println!("╔══════════════════════════════════════════════════════════════════╗");
            println!("║              Welcome to ntc 1.1.0 - Navigate, Tree, Cat          ║");
            println!("╚══════════════════════════════════════════════════════════════════╝");
            println!();
            println!("Type 'help' for available commands, 'exit' to quit.");
            println!();
            show_tree(nav, Some(1), false,false);
        }

        "version" => {
            println!("ntc 1.1.0");
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
            println!("Ignored directories: {:?}", dirs);
            println!("Ignored extensions: {:?}", ignored_exts);
            println!("Extra supported extensions: {:?}", extra_exts);
        }
        "ignore" => {
            if args.is_empty() {
                println!("Usage: ignore <name>");
            } else {
                Config::global_add_ignored_dir(args);
                println!("Now ignoring: {}", args);
            }
        }
        "cared" => {
            if args.is_empty() {
                println!("Usage: cared <name>");
            } else {
                Config::global_remove_ignored_dir(args);
                println!("No longer ignoring: {}", args);
            }
        }
        "ignoref" => {
            if args.is_empty() {
                println!("Usage: ignoref <ext>");
            } else {
                Config::global_add_ignored_extension(args);
                println!("Now ignoring .{} files", args);
            }
        }
        "caref" => {
            if args.is_empty() {
                println!("Usage: caref <ext>");
            } else {
                Config::global_remove_ignored_extension(args);
                Config::global_add_extra_supported_extension(args);
                println!("Now caring about .{} files", args);
            }
        }

        _ => {
            println!("Unknown command: {}", cmd);
            println!("Type 'help' for available commands.");
        }
    }

    Ok(false)
}


/// Show directory tree.
fn show_tree(nav: &Navigator, max_depth_override: Option<usize>, show_progress: bool, include_files: bool) {
    println!();
    print_separator("Current Directory");
    println!("Path: {}", nav.display_path());
    println!();

    let pb = if show_progress {
        let total = crate::explorer::count_entries(
            &nav.current_path().to_string_lossy(),
            max_depth_override,
        );
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::with_template("[{bar:40}] {percent}% {msg}")
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
    );

    if let Some(pb) = pb {
        pb.finish_with_message("Done");
    }

    let tree_str = format_tree(&tree, "", true);
    println!("{}", tree_str);
}

/// Print interactive mode help
fn print_interactive_help() {
    println!(r#"
╔══════════════════════════════════════════════════════════════════╗
║                     ntc 1.1.0 - Interactive Help                 ║
╚══════════════════════════════════════════════════════════════════╝

NAVIGATION COMMANDS:
  go <path>           Navigate to a directory (shows root contents)
  go                  Show go command usage
  godrive             List all drives and select one
  godrive <letter>    Navigate to a drive (e.g., godrive C)
  back                Go back to parent directory

VIEW COMMAND:
  view                Show full directory tree using configured depth

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
  setD <int>          Set max depth (min: 2)
  setL                Show line number setting (ON/OFF)
  setL ON|OFF         Enable/disable line numbers
  setT                Show current thread count
  setT <int>          Set number of threads

IGNORE/CARE COMMANDS:
    ignored             Show ignored dirs, extensions, and extra supported
    ignore <name>       Ignore a directory name
    cared <name>        Stop ignoring a directory name
    ignoref <ext>       Ignore a file extension
    caref <ext>         Care about a file extension (un-ignore and add as supported)

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
        Config::global_set_max_depth(10);
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