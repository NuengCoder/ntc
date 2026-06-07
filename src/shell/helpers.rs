use crate::config::Config;
use crate::explorer::{format_tree, format_tree_with_sizes, generate_tree};
use crate::navigator::Navigator;
use crate::output::{print_error, print_separator};

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ============================================================================
// Helper functions (gosc, show_tree, file menus, spinner, system command)
// ============================================================================

pub(crate) fn show_tree(
    nav: &Navigator,
    max_depth_override: Option<usize>,
    show_progress: bool,
    include_files: bool,
    show_sizes: bool,
    copy_to_clipboard: bool,
    care_sizes: bool,
) {
    println!();
    print_separator("Current Directory");
    println!("Path: {}", nav.display_path().cyan());
    println!();

    let mut tree = if show_progress {
        run_with_spinner("Building directory tree...", || {
            generate_tree(
                &nav.current_path().to_string_lossy(),
                max_depth_override,
                include_files,
                None,
            )
        })
    } else {
        generate_tree(
            &nav.current_path().to_string_lossy(),
            max_depth_override,
            include_files,
            None,
        )
    };

    let tree_str = if show_sizes {
        if show_progress {
            run_with_spinner("Calculating directory sizes...", || {
                crate::explorer::compute_tree_sizes(&mut tree, None, care_sizes);
            });
        } else {
            crate::explorer::compute_tree_sizes(&mut tree, None, care_sizes);
        }
        format_tree_with_sizes(&tree, "", true, true, care_sizes, None)
    } else {
        format_tree(&tree, "", true)
    };

    if copy_to_clipboard {
        let _ = crate::output::copy_to_clipboard(&tree_str, "Tree");
    }

    for line in tree_str.lines() {
        if line.contains("[ignored]") {
            println!("{}", line.red().dimmed());
        } else if line.contains("[Directory]") {
            println!("{}", line.blue());
        } else if line.trim().starts_with("├──") || line.trim().starts_with("└──") {
            println!("{}", line.green());
        } else {
            println!("{}", line);
        }
    }
}

pub(super) fn gosc_loop(nav: &mut Navigator) -> Result<()> {
    loop {
        let dirs = nav.list_subdirs()?;
        crate::navigator::clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false);
        
        println!();
        println!("╔══════════════════════════════════════════════════╗");
        println!("║{:^50}║", "gosc — Navigate Continuously".cyan().bold());
        println!("╠══════════════════════════════════════════════════╣");
        println!("║ {} go back 1 level", "-1".yellow());
        println!("║ {} go back 2 levels", "-2".yellow());
        println!("║ {} go back n levels", "-n".yellow());
        println!("║ {} exit gosc", "0".red());
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
        
        if let Some(back_str) = input.strip_prefix('-') {
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

pub(super) fn ignoresc_loop(nav: &mut Navigator) -> Result<()> {
    loop {
        let dirs = nav.list_subdirs()?;
        crate::navigator::clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false);

        println!();
        println!("╔══════════════════════════════════════════════════╗");
        println!("║{:^50}║", "ignoresc — Continuously Ignore Directories".cyan().bold());
        println!("╠══════════════════════════════════════════════════╣");
        println!("║ {} exit ignoresc", "0".red());
        println!("╠──────────────────────────────────────────────────╣");

        if dirs.is_empty() {
            println!("║ {}", "(no subdirectories)".dimmed());
        } else {
            for (i, name) in &dirs {
                println!("║ {}. {}", i.to_string().yellow(), name.blue());
            }
        }

        println!("╚══════════════════════════════════════════════════╝");
        println!();
        print!("{} ", "ignoresc>".green().bold());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "0" {
            println!("{}", "Exiting ignoresc...".dimmed());
            break Ok(());
        }

        match input.parse::<usize>() {
            Ok(num) => {
                if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
                    let _ = Config::local_add_ignored_dir(name);
                    Config::reload_global();
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

pub(super) fn caresc_loop(nav: &mut Navigator) -> Result<()> {
    loop {
        let ignored = Config::global_get_ignored_dirs();
        let mut ignored_vec: Vec<String> = ignored.into_iter().collect();
        ignored_vec.sort_by_key(|a| a.to_lowercase());
        crate::navigator::clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false);

        println!();
        println!("╔══════════════════════════════════════════════════╗");
        println!("║{:^50}║", "caresc — Continuously Un-ignore Directories".cyan().bold());
        println!("╠══════════════════════════════════════════════════╣");
        println!("║ {} exit caresc", "0".red());
        println!("╠──────────────────────────────────────────────────╣");

        if ignored_vec.is_empty() {
            println!("║ {}", "(no ignored directories)".dimmed());
        } else {
            for (i, name) in ignored_vec.iter().enumerate() {
                println!("║ {}. {}", (i + 1).to_string().yellow(), name.red());
            }
        }

        println!("╚══════════════════════════════════════════════════╝");
        println!();
        print!("{} ", "caresc>".green().bold());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "0" {
            println!("{}", "Exiting caresc...".dimmed());
            break Ok(());
        }

        match input.parse::<usize>() {
            Ok(num) if num >= 1 && num <= ignored_vec.len() => {
                let name = &ignored_vec[num - 1];
                let _ = Config::local_remove_ignored_dir(name);
                Config::reload_global();
            }
            Ok(num) => {
                print_error(&format!("Invalid number: {}", num));
            }
            Err(_) => {
                print_error(&format!("Invalid input: {}", input));
            }
        }
    }
}

pub(super) fn open_with_fallback(config_path: &std::path::Path) {
    #[cfg(windows)]
    {
        let status = std::process::Command::new("cmd")
            .args(["start", "/C","", config_path.to_str().unwrap_or("")])
            .status();
        if status.is_ok() { return; }
    }
    #[cfg(target_os = "macos")]
    {
        let status = std::process::Command::new("open").arg(config_path).status();
        if status.is_ok() { return; }
    }
    #[cfg(target_os = "linux")]
    {
        if std::process::Command::new("xdg-open")
            .arg(config_path)
            .status()
            .is_ok()
        {
            return;
        }
        for editor in &["vim", "nano", "nvim","emacs"] {
            if std::process::Command::new(editor)
                .arg(config_path)
                .status()
                .is_ok()
            {
                return;
            }
        }
    }
    crate::output::print_info("No external editor found. Opening built-in editor...");
    let _ = crate::editor::edit_file(config_path);
}

/// Run a closure with an animated spinner in a background thread.
/// The spinner is cleared when `f` returns.
pub(super) fn run_with_spinner<F, T>(msg: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();
    let msg = msg.to_string();

    let msg_clone = msg.clone();
    let handle = thread::spawn(move || {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut i = 0;
        while !done_clone.load(Ordering::Relaxed) {
            print!("\r{} {} ", frames[i], msg_clone);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_millis(80));
            i = (i + 1) % frames.len();
        }
    });

    let result = f();

    done.store(true, Ordering::Relaxed);
    let _ = handle.join();
    // Clear spinner line with spaces then go back to start
    let clear_len = msg.len() + 4; // spinner char + space + msg + space
    print!("\r{:1$}\r", "", clear_len);
    let _ = io::stdout().flush();

    result
}