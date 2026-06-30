use crate::config::Config;
use crate::explorer::{human_readable_size, format_duration, calculate_dir_size, calculate_total_size};
use crate::filetype::is_supported_format;
use crate::navigator::{clear_screen, Navigator};
use crate::output::{cat_file, print_error, print_success, print_warning, print_info};
use crate::search;
use crate::session::SessionState;
use crate::shell::helpers::{show_tree, run_with_spinner};
use crate::shell::help::{print_interactive_help, print_tp_help};
use crate::shell::venv::{clear_venv_environment};
use crate::teleport::TeleportManager;
use crate::watcher;
use super::{execute_system_command, parse_search_args};

use anyhow::Result;
use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::Path;
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
    let raw_path = if let Some(hyper_args) = args.strip_prefix("--hyper ") {
        let mut parts = hyper_args.trim().splitn(3, ' ');
        let name = parts.next().unwrap_or("").trim();
        let mut lang = "c";
        if let Some(p) = parts.next() {
            if p == "--lang" {
                if let Some(l) = parts.next() { lang = l.trim(); }
            }
        }
        if name.is_empty() || name.contains('.') {
            print_error("Usage: ne --hyper <name> --lang <lang>  (name must not contain extension)");
            return Ok(false);
        }
        match crate::editor::init_hyper_project(name, lang) {
            Ok(_) => eprintln!("Created hyper project: {}", name),
            Err(e) => print_error(&format!("Hyper project error: {}", e)),
        }
        return Ok(false);
    } else if let Some(super_path) = args.strip_prefix("--super ") {
        let p = std::path::PathBuf::from(super_path.trim());
        let _ = crate::editor::init_super_file(&p);
        eprintln!("Created super template: {}", p.display());
        p
    } else if let Some(init_path) = args.strip_prefix("--init ") {
        let p = std::path::PathBuf::from(init_path.trim());
        let _ = crate::editor::init_file(&p);
        eprintln!("Created template: {}", p.display());
        p
    } else if args.is_empty() || args == "." {
        let cwd = nav.current_path();
        let welcome = cwd.join("welcome.ntcEditor");
        if !welcome.exists() {
            let content = crate::editor::generate_welcome_content();
            let _ = std::fs::write(&welcome, content);
        }
        print_info(&format!("Opened welcome screen in {}", nav.current_path().display()));
        welcome
    } else {
        // Support "file:line" syntax (e.g. main.rs:42, src/main.rs:42)
        let trimmed = args.trim();
        if let Some((file_part, line_part)) = trimmed.rsplit_once(':') {
            if let Ok(line) = line_part.parse::<usize>() {
                if line > 0 {
                    let p = std::path::PathBuf::from(file_part);
                    crate::editor::GOTO_LINE.store(line, std::sync::atomic::Ordering::SeqCst);
                    p
                } else {
                    std::path::PathBuf::from(trimmed)
                }
            } else {
                std::path::PathBuf::from(trimmed)
            }
        } else {
            std::path::PathBuf::from(trimmed)
        }
    };
    let path = if raw_path.is_dir() {
        let welcome = raw_path.join("welcome.ntcEditor");
        if !welcome.exists() {
            let content = crate::editor::generate_welcome_content();
            let _ = std::fs::write(&welcome, content);
        }
        welcome
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
    let mut care = false;
    let mut show_time = false;
    for arg in args.split_whitespace() {
        match arg {
            "--care" => care = true,
            "--time" | "-t" => show_time = true,
            _ => {}
        }
    }
    let start = std::time::Instant::now();
    let total = if care {
        run_with_spinner("Calculating directory size...", || {
            calculate_total_size(nav.current_path())
        })
    } else {
        run_with_spinner("Calculating directory size...", || {
            calculate_dir_size(nav.current_path())
        })
    };
    let elapsed = start.elapsed();
    let bytes_str = format!("{}", total);
    let human_str = human_readable_size(total);
    let time_str = format_duration(elapsed);
    let max_label = 8;
    let max_value = bytes_str.len().max(human_str.len()).max(time_str.len());
    let w = max_label + max_value + 10;

    println!();
    println!("┌{}┐", "─".repeat(w));
    println!("│{:^w$}│", "Current Directory Size".cyan().bold(), w = w);
    println!("├{}┤", "─".repeat(w));
    println!("│ Bytes: {:<w$} │", bytes_str.yellow(), w = w - 9);
    println!("│ Human: {:<w$} │", human_str.green().bold(), w = w - 9);
    if show_time {
        println!("├{}┤", "─".repeat(w));
        println!("│ Time:  {:<w$} │", time_str.cyan(), w = w - 9);
    }
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
        print_error("Usage: gs <pattern> [-d <depth>] [-i]");
        println!("  gs main             # search file contents using config depth");
        println!("  gs main -d 5        # search up to 5 levels deep");
        println!("  gs -i main          # interactive mode (browse results, Enter to open)");
        println!("  gs -i main -d 3     # interactive mode with depth limit");
        return Ok(false);
    }
    let interactive = args.split_whitespace().any(|s| s == "-i");
    let clean_args = if interactive {
        args.split_whitespace()
            .filter(|s| *s != "-i")
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        args.to_string()
    };
    let (pattern, max_depth) = parse_search_args(&clean_args);
    let results = crate::search::search_content(nav.current_path(), &pattern, max_depth);
    if interactive {
        return super::gs_interactive::run_gs_interactive(results, nav.current_path());
    }
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

pub fn cmd_rnmf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error("rnmf: missing operand");
        println!("  Usage: rnmf <old_name> <new_name>");
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error("rnmf: missing operand");
        println!("  Usage: rnmf <old_name> <new_name>");
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&format!("rnmf: '{}' does not exist", old));
    } else if old_path.is_dir() {
        print_error(&format!("rnmf: '{}' is a directory (use rnmd instead)", old));
    } else if new_path.exists() {
        print_error(&format!("rnmf: '{}' already exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&format!("Renamed '{}' -> '{}'", old, new)),
            Err(e) => print_error(&format!("rnmf: {}", e)),
        }
    }
    Ok(false)
}

pub fn cmd_rnmd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error("rnmd: missing operand");
        println!("  Usage: rnmd <old_name> <new_name>");
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error("rnmd: missing operand");
        println!("  Usage: rnmd <old_name> <new_name>");
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&format!("rnmd: '{}' does not exist", old));
    } else if !old_path.is_dir() {
        print_error(&format!("rnmd: '{}' is not a directory", old));
    } else if new_path.exists() {
        print_error(&format!("rnmd: '{}' already exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&format!("Renamed directory '{}' -> '{}'", old, new)),
            Err(e) => print_error(&format!("rnmd: {}", e)),
        }
    }
    Ok(false)
}

pub fn cmd_ui(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    println!("The modern UI was removed. Only the classic shell is available.");
    Ok(false)
}



// In src/shell/commands/misc.rs - Replace the cmd_venva function:

pub fn cmd_venva(args: &str, nav: &mut Navigator) -> Result<bool> {
    use crate::shell::venv::*;
    
    // Check if already in a venv
    if env::var("VIRTUAL_ENV").is_ok() {
        print_warning("Already in a virtual environment. Use 'deactivate' to exit first.");
        return Ok(false);
    }
    
    let current_path = nav.current_path().to_path_buf();
    
    let venv_path = if args.is_empty() {
        // Auto-detect venv in current directory
        let venv_names = &["venv", ".venv", "env"];
        let mut found = None;
        
        for name in venv_names {
            let venv_dir = current_path.join(name);
            if venv_dir.exists() {
                found = Some(venv_dir);
                break;
            }
        }
        
        if found.is_none() {
            print_error("No virtual environment found. Usage: python_activate <venv_name>");
            return Ok(false);
        }
        found.unwrap()
    } else {
        let path = Path::new(args);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_path.join(args)
        }
    };
    
    // Find the activate script
    let activate_script = get_activate_script_path(&venv_path);
    let activate_path = match activate_script {
        Some(p) => p,
        None => {
            print_error(&format!("No activate script found in: {}", venv_path.display()));
            return Ok(false);
        }
    };
    
    print_info(&format!("Activating virtual environment: {}", venv_path.display()));
    
    // Parse and apply the environment
    #[cfg(windows)]
    let changes = parse_batch_activate(&activate_path);
    #[cfg(not(windows))]
    let changes = parse_bash_activate(&activate_path);
    
    match changes {
        Ok(env_changes) => {
            apply_venv_environment(env_changes);
            
            // Update the navigator's venv name
            let new_venv_name = detect_active_venv();
            nav.update_venv_name(new_venv_name);
            
            // Verify activation by checking if pip is in PATH
            check_pip_in_path(&venv_path);
            
            print_success(&format!("Activated: {}", venv_path.display()));
            print_info("Virtual environment is now active. pip and python commands will use this environment.");
        }
        Err(e) => {
            print_error(&format!("Failed to activate: {}", e));
        }
    }
    
    Ok(false)
}

// In src/shell/commands/misc.rs - Add/enhance this function:

pub fn cmd_show_env(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let show_all = args == "--all" || args == "-a";
    let depth: Option<usize> = if let Some(depth_str) = args.strip_prefix("-d ") {
        depth_str.parse::<usize>().ok()
    } else if let Some(depth_str) = args.strip_prefix("--depth ") {
        depth_str.parse::<usize>().ok()
    } else {
        None
    };
    
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                         Environment Variables                    ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Core venv variables (always shown)
    println!("{}", "🐍 Python Virtual Environment:".green().bold());
    match std::env::var("VIRTUAL_ENV") {
        Ok(val) => println!("  VIRTUAL_ENV = {}", val.cyan()),
        Err(_) => println!("  VIRTUAL_ENV = {}", "(not set)".dimmed()),
    }
    println!();
    
    // PATH is always useful to see
    println!("{}", "📂 PATH (first few entries):".yellow().bold());
    if let Ok(path) = std::env::var("PATH") {
        let entries: Vec<&str> = path.split(if cfg!(windows) { ';' } else { ':' }).collect();
        let max_display = depth.unwrap_or(5);
        
        for (i, entry) in entries.iter().enumerate().take(max_display) {
            let marker = if entry.contains("venv") || entry.contains(".venv") || entry.contains("env") {
                "🐍"
            } else {
                "  "
            };
            println!("  {}. {} {}", (i + 1).to_string().yellow(), marker, entry.dimmed());
        }
        
        if entries.len() > max_display {
            let remaining = entries.len() - max_display;
            println!("  {} ... and {} more", "   ".to_string().dimmed(), remaining.to_string().yellow());
        }
        
        if show_all {
            println!();
            println!("{}", "📂 PATH (full):".yellow().bold());
            for (i, entry) in entries.iter().enumerate() {
                let marker = if entry.contains("venv") || entry.contains(".venv") || entry.contains("env") {
                    "🐍"
                } else {
                    "  "
                };
                println!("  {}. {} {}", (i + 1).to_string().yellow(), marker, entry);
            }
        }
    }
    println!();
    
    // System information
    println!("{}", "💻 System Information:".blue().bold());
    println!("  OS: {}", std::env::consts::OS.cyan());
    println!("  Shell: ntc v{}", env!("CARGO_PKG_VERSION").cyan());
    println!("  Current directory: {}", _nav.display_path().dimmed());
    println!();
    
    // Optional: Show other venv-related variables if they exist
    let other_vars = [
        "PYTHONHOME",
        "PYTHONPATH",
        "_OLD_VIRTUAL_PATH",
        "_OLD_VIRTUAL_PYTHONHOME",
        "CONDA_DEFAULT_ENV",
        "CONDA_PREFIX",
    ];
    
    let mut has_others = false;
    for var in &other_vars {
        if let Ok(val) = std::env::var(var) {
            if !has_others {
                println!("{}", "🔧 Other Python/Venv Variables:".magenta().bold());
                has_others = true;
            }
            println!("  {} = {}", var.cyan(), val.dimmed());
        }
    }
    
    if has_others {
        println!();
    }
    
    // Summary
    println!("{}", "📊 Summary:".cyan().bold());
    if std::env::var("VIRTUAL_ENV").is_ok() {
        println!("  ✅ Virtual environment is {}", "ACTIVE".green().bold());
        println!("  💡 Run 'deactivate' to exit the virtual environment");
    } else {
        println!("  ❌ Virtual environment is {}", "NOT ACTIVE".red().bold());
        println!("  💡 Run 'python_activate' to activate a virtual environment");
    }
    println!();
    
    Ok(false)
}

// Helper function to verify pip is accessible
fn check_pip_in_path(venv_path: &Path) {
    #[cfg(windows)]
    let pip_path = venv_path.join("Scripts").join("pip.exe");
    #[cfg(not(windows))]
    let pip_path = venv_path.join("bin").join("pip");
    
    if pip_path.exists() {
        print_info(&format!("✓ pip found at: {}", pip_path.display()));
    } else {
        print_warning("pip not found in venv/Scripts directory");
    }
}

pub fn cmd_venvd(_args: &str, nav: &mut Navigator) -> Result<bool> {
    if env::var("VIRTUAL_ENV").is_err() {
        print_info("Not in a virtual environment");
        return Ok(false);
    }
    clear_venv_environment();
    let current_path = nav.current_path().to_path_buf();
    nav.go_to(&current_path)?; // Refresh display
    print_success("Deactivated virtual environment");
    Ok(false)
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