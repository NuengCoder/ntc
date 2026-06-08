use crate::config::Config;
use crate::explorer::{format_tree, format_tree_with_sizes, generate_tree};
use crate::navigator::Navigator;
use crate::output::{print_error, print_separator, print_success, print_warning};

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

// ============================================================================
// Export/Import helpers for ral and igcare
// ============================================================================

fn export_path(name: &str, ext: &str) -> std::path::PathBuf {
    let out = Config::global_get_output_path();
    out.join(format!("{}.{}", name, ext))
}

pub(crate) fn ral_export_all(name: &str) -> Result<()> {
    let aliases = Config::global_get_run_aliases();
    if aliases.is_empty() {
        print_warning("No run aliases to export.");
        return Ok(());
    }
    let path = export_path(name, "ntc.ral");
    write_ral_file(&path, &aliases, None)?;
    Ok(())
}

pub(crate) fn ral_export_select(nav: &mut Navigator, name: &str) -> Result<()> {
    let aliases = Config::global_get_run_aliases();
    if aliases.is_empty() {
        print_warning("No run aliases to export.");
        return Ok(());
    }

    let mut sorted: Vec<(&String, &String)> = aliases.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    let mut selected: std::collections::HashSet<usize> = std::collections::HashSet::new();

    loop {
        crate::navigator::clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false);

        println!();
        println!("╔══════════════════════════════════════════════════╗");
        println!("║{:^50}║", "ral export — Select Aliases to Export".cyan().bold());
        println!("╠══════════════════════════════════════════════════╣");
        println!("║ {} select/deselect by number     ", "<n>".yellow());
        println!("║ {} select all                    ", "a".yellow());
        println!("║ {} finish export ({})           ", "0".red(), if selected.is_empty() { "cancel".red() } else { format!("{} selected", selected.len()).green() });
        println!("╠──────────────────────────────────────────────────╣");

        for (i, (aname, cmd)) in sorted.iter().enumerate() {
            let idx = i + 1;
            let mark = if selected.contains(&idx) {
                " (selected)".green().to_string()
            } else {
                String::new()
            };
            println!("║ {}. {} -> {}{}", idx.to_string().yellow(), aname.blue(), cmd.dimmed(), mark);
        }

        println!("╚══════════════════════════════════════════════════╝");
        println!();
        print!("{} ", "ral-export>".green().bold());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            continue;
        }

        if input == "0" {
            if selected.is_empty() {
                println!("{}", "Export cancelled.".dimmed());
                return Ok(());
            }
            // Export selected aliases
            let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            let mut sel_indices: Vec<usize> = selected.iter().cloned().collect();
            sel_indices.sort();
            for idx in sel_indices {
                let (aname, cmd) = sorted[idx - 1];
                map.insert(aname.clone(), cmd.clone());
            }
            let path = export_path(name, "ntc.ral");
            write_ral_file(&path, &map, Some(map.len()))?;
            return Ok(());
        }

        if input == "a" {
            let path = export_path(name, "ntc.ral");
            write_ral_file(&path, &aliases, None)?;
            return Ok(());
        }

        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= sorted.len() {
                if selected.contains(&num) {
                    selected.remove(&num);
                } else {
                    selected.insert(num);
                }
            } else {
                print_error(&format!("Invalid number: {}. Use 1-{} or 0 to finish.", num, sorted.len()));
            }
        } else {
            print_error(&format!("Invalid input: {}", input));
        }
    }
}

fn write_ral_file(
    path: &std::path::Path,
    aliases: &std::collections::HashMap<String, String>,
    count: Option<usize>,
) -> Result<()> {
    let mut content = String::from("# ntc run aliases export\n");
    content.push_str("# Import with: ral import <file>\n\n");
    content.push_str("[run_aliases]\n");

    let mut sorted: Vec<(&String, &String)> = aliases.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    for (name, cmd) in &sorted {
        content.push_str(&format!("{} = {:?}\n", name, cmd));
    }

    std::fs::write(path, &content)?;

    if let Some(cnt) = count {
        print_success(&format!("Exported {} alias(es) to: {}", cnt, path.display()));
    } else {
        print_success(&format!("Exported all {} alias(es) to: {}", sorted.len(), path.display()));
    }
    Ok(())
}

pub(crate) fn ral_import(path_str: &str) -> Result<()> {
    let path = std::path::Path::new(path_str);
    if !path.exists() {
        print_error(&format!("File not found: {}", path.display()));
        return Ok(());
    }
    Config::import_run_aliases_from_file(path)
        .map_err(|e| anyhow::anyhow!("Failed to import: {}", e))?;
    Config::reload_global();
    print_success(&format!("Imported run aliases from: {}", path.display()));
    Ok(())
}

pub(crate) fn igcare_export_all(name: &str) -> Result<()> {
    let path = export_path(name, "ntc.igcare");
    let cfg = Config::global().read().unwrap();
    write_igcare_file(&path, &cfg)?;
    drop(cfg);
    print_success(&format!("Exported all ignore/care settings to: {}", path.display()));
    Ok(())
}

pub(crate) fn igcare_export_select(name: &str) -> Result<()> {
    let cfg = Config::global().read().unwrap();

    let category_labels = [
        "Ignored directories",
        "Ignored extensions",
        "Extra supported extensions",
        "Ignored files",
        "Extra supported files",
    ];

    let category_fields: [&std::collections::HashSet<String>; 5] = [
        &cfg.ignored_directory_names,
        &cfg.ignored_extensions,
        &cfg.extra_supported_extensions,
        &cfg.ignored_files,
        &cfg.extra_supported_files,
    ];

    let total = category_fields.len();
    let mut selected: std::collections::HashSet<usize> = std::collections::HashSet::new();

    loop {
        crate::navigator::clear_screen();

        println!();
        println!("╔══════════════════════════════════════════════════╗");
        println!("║{:^50}║", "igcare export — Select Categories".cyan().bold());
        println!("╠══════════════════════════════════════════════════╣");
        println!("║ {} select/deselect by number     ", "<n>".yellow());
        println!("║ {} select all                    ", "a".yellow());
        println!("║ {} finish export ({})           ", "0".red(), if selected.is_empty() { "cancel".red() } else { format!("{} selected", selected.len()).green() });
        println!("╠──────────────────────────────────────────────────╣");

        for (i, label) in category_labels.iter().enumerate() {
            let idx = i + 1;
            let count = category_fields[i].len();
            let count_str = format!(" ({} item(s))", count).yellow();
            let mark = if selected.contains(&idx) {
                " (selected)".green().to_string()
            } else {
                String::new()
            };
            println!("║ {}. {}{}{}", idx.to_string().yellow(), label.cyan(), count_str, mark);
        }

        println!("╚══════════════════════════════════════════════════╝");
        println!();
        print!("{} ", "igcare-export>".green().bold());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            continue;
        }

        if input == "0" {
            if selected.is_empty() {
                println!("{}", "Export cancelled.".dimmed());
                return Ok(());
            }
            let mut sel: Vec<usize> = selected.iter().cloned().collect();
            sel.sort();
            let path = export_path(name, "ntc.igcare");
            write_igcare_file_selective(&path, &cfg, &sel)?;
            print_success(&format!("Exported {} category(ies) to: {}", sel.len(), path.display()));
            return Ok(());
        }

        if input == "a" {
            let path = export_path(name, "ntc.igcare");
            write_igcare_file(&path, &cfg)?;
            print_success(&format!("Exported all ignore/care settings to: {}", path.display()));
            return Ok(());
        }

        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= total {
                if selected.contains(&num) {
                    selected.remove(&num);
                } else {
                    selected.insert(num);
                }
            } else {
                print_error(&format!("Invalid number: {}. Use 1-{} or 0 to finish.", num, total));
            }
        } else {
            print_error(&format!("Invalid input: {}", input));
        }
    }
}

fn write_igcare_file(path: &std::path::Path, cfg: &Config) -> Result<()> {
    fn format_set(items: &std::collections::HashSet<String>) -> String {
        if items.is_empty() {
            return "[]".to_string();
        }
        let mut sorted: Vec<&String> = items.iter().collect();
        sorted.sort();
        let mut result = String::from("[\n");
        for item in sorted {
            result.push_str(&format!("    {:?},\n", item));
        }
        result.push(']');
        result
    }

    let mut content = String::from("# ntc ignore/care settings export\n");
    content.push_str("# Import with: igcare import <file>\n\n");
    content.push_str(&format!("ignored_directory_names = {}\n\n", format_set(&cfg.ignored_directory_names)));
    content.push_str(&format!("ignored_extensions = {}\n\n", format_set(&cfg.ignored_extensions)));
    content.push_str(&format!("extra_supported_extensions = {}\n\n", format_set(&cfg.extra_supported_extensions)));
    content.push_str(&format!("ignored_files = {}\n\n", format_set(&cfg.ignored_files)));
    content.push_str(&format!("extra_supported_files = {}\n", format_set(&cfg.extra_supported_files)));

    std::fs::write(path, &content)?;
    Ok(())
}

fn write_igcare_file_selective(
    path: &std::path::Path,
    cfg: &Config,
    selections: &[usize],
) -> Result<()> {
    fn format_set(items: &std::collections::HashSet<String>) -> String {
        if items.is_empty() {
            return "[]".to_string();
        }
        let mut sorted: Vec<&String> = items.iter().collect();
        sorted.sort();
        let mut result = String::from("[\n");
        for item in sorted {
            result.push_str(&format!("    {:?},\n", item));
        }
        result.push(']');
        result
    }

    let category_fields: Vec<(&str, &std::collections::HashSet<String>)> = vec![
        ("ignored_directory_names", &cfg.ignored_directory_names),
        ("ignored_extensions", &cfg.ignored_extensions),
        ("extra_supported_extensions", &cfg.extra_supported_extensions),
        ("ignored_files", &cfg.ignored_files),
        ("extra_supported_files", &cfg.extra_supported_files),
    ];

    let mut content = String::from("# ntc ignore/care settings export (selected categories)\n");
    content.push_str("# Import with: igcare import <file>\n\n");

    for sel in selections {
        if *sel >= 1 && *sel <= category_fields.len() {
            let (field_name, items) = category_fields[*sel - 1];
            content.push_str(&format!("{} = {}\n\n", field_name, format_set(items)));
        }
    }

    std::fs::write(path, &content)?;
    Ok(())
}

pub(crate) fn igcare_import(path_str: &str) -> Result<()> {
    let path = std::path::Path::new(path_str);
    if !path.exists() {
        print_error(&format!("File not found: {}", path.display()));
        return Ok(());
    }
    Config::import_igcare_from_file(path)
        .map_err(|e| anyhow::anyhow!("Failed to import: {}", e))?;
    Config::reload_global();
    print_success(&format!("Imported ignore/care settings from: {}", path.display()));
    Ok(())
}