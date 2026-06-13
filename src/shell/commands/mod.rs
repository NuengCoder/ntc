use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::watcher;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

// ── submodules ──────────────────────────────────────────────────────────
mod alias_commands;
mod backup;
mod config_commands;
mod ignore;
mod misc;
mod nav;
mod ran_commands;
mod report;


// ============================================================================
// Command execution
// ============================================================================

/// Execute a single command (already fully expanded)
#[allow(clippy::only_used_in_recursion)]
pub(crate) fn execute_command(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").trim();

    match cmd.as_str() {
        "go" | "cd" => nav::cmd_go(args, nav),
        "godrive" | "god" => nav::cmd_godrive(args, nav),
        "back" | "b" => nav::cmd_back(args, nav),
        "view" => report::cmd_view(args, nav),
        "txt" => report::cmd_txt(args, nav),
        "txtc" => report::cmd_txtc(args, nav),
        "txtf" => report::cmd_txtf(args, nav),
        "html" => report::cmd_html(args, nav),
        "json" => report::cmd_json(args, nav),
        "md" => report::cmd_md(args, nav),
        "pdf" => report::cmd_pdf(args, nav),
        "docx" => report::cmd_docx(args, nav),
        "xlsx" => report::cmd_xlsx(args, nav),
        "seto" => config_commands::cmd_seto(args, nav),
        "setd" => config_commands::cmd_setd(args, nav),
        "setl" => config_commands::cmd_setl(args, nav),
        "setc" => config_commands::cmd_setc(args, nav),
        "seta" => config_commands::cmd_seta(args, nav),
        "sett" => config_commands::cmd_sett(args, nav),
        "seth" => config_commands::cmd_seth(args, nav),
        "watch" => config_commands::cmd_watch(args, nav),
        "clear" => misc::cmd_clear(args, nav),
        "dino" => misc::cmd_dino(args, nav),
        "math" => misc::cmd_math(args, nav),
        "version" => misc::cmd_version(args, nav),
        "where" => nav::cmd_where(nav),
        "gos" => nav::cmd_gos(nav),
        "gosc" => nav::cmd_gosc(nav),
        "ignores" => ignore::cmd_ignores(args, nav),
        "ignoresc" => ignore::cmd_ignoresc(nav),
        "cares" => ignore::cmd_cares(args, nav),
        "caresc" => ignore::cmd_caresc(nav),
        "ral" => alias_commands::cmd_ral(args, nav),
        "ran" => ran_commands::cmd_ran(args, nav),
        "run" | "r" => alias_commands::cmd_run(args, nav),
        "showcg" => config_commands::cmd_showcg(args, nav),
        "opencg" | "editcfg" => config_commands::cmd_opencg(args, nav),
        "ne" | "ntceditor" => misc::cmd_ne(args, nav),
        "resetcg" | "reset-config" => config_commands::cmd_resetcg(args, nav),
        "restorecg" | "restore-config" => config_commands::cmd_restorecg(args, nav),
        "local" => config_commands::cmd_local(args, nav),
        "init" => config_commands::cmd_init(args, nav),
        "deinit" => config_commands::cmd_deinit(args, nav),
        "help" => misc::cmd_help(args, nav),
        "tutorial" | "guide" => misc::cmd_tutorial(args, nav),
        "exit" | "quit" | "esc" => misc::cmd_exit(args, nav),
        "ignored" => ignore::cmd_ignored(args, nav),
        "ignore" => ignore::cmd_ignore(args, nav),
        "cared" => ignore::cmd_cared(args, nav),
        "ignoref" => ignore::cmd_ignoref(args, nav),
        "caref" => ignore::cmd_caref(args, nav),
        "ignoren" => ignore::cmd_ignoren(args, nav),
        "caren" => ignore::cmd_caren(args, nav),
        "igcare" => ignore::cmd_igcare(args, nav),
        "size" => misc::cmd_size(args, nav),
        "tp" => misc::cmd_tp(args, nav),
        "tpb" => misc::cmd_tpb(args, nav),
        "ui" => misc::cmd_ui(args, nav),
        "bkup" => backup::cmd_bkup(args, nav),
        "pldw" => backup::cmd_pldw(args, nav),
        "unpd" => backup::cmd_unpd(args, nav),
        "diff" => backup::cmd_diff(args, nav),
        "fs" => misc::cmd_fs(args, nav),
        "ds" => misc::cmd_ds(args, nav),
        "gs" => misc::cmd_gs(args, nav),
        "fgo" => nav::cmd_fgo(args, nav),
        "fsc" => misc::cmd_fsc(args, nav),
        "locate" => misc::cmd_locate(args, nav),
        "mkf" => misc::cmd_mkf(args, nav),
        "mkd" => misc::cmd_mkd(args, nav),
        "rmd" => misc::cmd_rmd(args, nav),
        "rmf" => misc::cmd_rmf(args, nav),
        "theme" => misc::cmd_theme(args, nav),
        _ => misc::cmd_fallback(input, nav, watcher_handle),
    }
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

static CHILD_PID: AtomicU32 = AtomicU32::new(0);

fn init_ctrlc_handler() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let _ = ctrlc::set_handler(move || {
            let pid = CHILD_PID.load(Ordering::SeqCst);
            if pid > 0 {
                #[cfg(windows)]
                {
                    let _ = std::process::Command::new("taskkill")
                        .args(["/F", "/T", "/PID", &pid.to_string()])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                }
                #[cfg(not(windows))]
                {
                    let _ = std::process::Command::new("kill")
                        .args(["-TERM", &pid.to_string()])
                        .status();
                }
            }
        });
    });
}

fn run_system_command(args: &str, cwd: &Path) -> Result<std::process::ExitStatus> {
    init_ctrlc_handler();

    #[cfg(windows)]
    {
        let mut child = std::process::Command::new("cmd")
            .args(["/C", args])
            .current_dir(cwd)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;
        
        CHILD_PID.store(child.id(), Ordering::SeqCst);
        let status = child.wait()?;
        CHILD_PID.store(0, Ordering::SeqCst);
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
        
        CHILD_PID.store(child.id(), Ordering::SeqCst);
        let status = child.wait()?;
        CHILD_PID.store(0, Ordering::SeqCst);
        Ok(status)
    }
}

/// ── shared arg parser for fs / ds ─────────────────────────────────────────
/// Accepts:  <pattern> [-d <depth>]   (depth flag must be at the end)
/// Returns:  (pattern, max_depth)
/// Handles double-quoted patterns: fs "my file.rs" -d 3
fn parse_search_args(args: &str) -> (String, usize) {
    let default_depth = Config::global_get_max_depth();

    // Tokenize respecting double quotes
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    // Look for -d <n> at the end: ["pattern", ..., "-d", "3"]
    let n = parts.len();
    if n >= 3 && parts[n - 2] == "-d" {
        if let Ok(depth) = parts[n - 1].parse::<usize>() {
            let pattern = parts[..n - 2].join(" ");
            return (pattern, depth);
        }
    }

    (args.to_string(), default_depth)
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
    
    files.sort_by_cached_key(|a| a.0.to_lowercase());
    Ok(files)
}

fn show_file_selection_menu(_nav: &Navigator, files: Vec<(String, PathBuf)>, copy_mode: bool) -> Result<()> {
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
                crate::output::cat_file_with_syntax(path, show_lines)?;
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
