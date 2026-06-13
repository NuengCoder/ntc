use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning, print_info};
use crate::shell::helpers::open_with_fallback;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn cmd_seto(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Current output path: {}", Config::global_get_output_path().display());
    } else {
        Config::global_set_output_path(Path::new(args));
        print_success(&format!("Output path set to: {}", Config::global_get_output_path().display()));
    }
    Ok(false)
}

pub fn cmd_setd(args: &str, _nav: &mut Navigator) -> Result<bool> {
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
    Ok(false)
}

pub fn cmd_setl(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_show_line_numbers() { "ON" } else { "OFF" };
        println!("Line numbers: {}", state);
    } else {
        match Config::parse_on_off(args) {
            Some(state) => {
                Config::global_set_show_line_numbers(state);
                print_success(&format!("Line numbers: {}", if state { "ON" } else { "OFF" }));
            }
            None => print_error(&format!("Invalid value: {}. Use ON or OFF.", args)),
        }
    }
    Ok(false)
}

pub fn cmd_setc(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_color_enabled() { "ON" } else { "OFF" };
        println!("Color output: {}", state);
    } else {
        match Config::parse_on_off(args) {
            Some(state) => {
                Config::global_set_color_enabled(state);
                print_success(&format!("Color: {}", if state { "ON" } else { "OFF" }));
            }
            None => print_error(&format!("Invalid value: {}. Use ON or OFF.", args)),
        }
    }
    Ok(false)
}

pub fn cmd_seta(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_autosuggest_enabled() { "ON" } else { "OFF" };
        println!("Autosuggest (ghost text): {}", state);
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_autosuggest_enabled(true);
                print_success("Autosuggest: ON");
            }
            "OFF" => {
                Config::global_set_autosuggest_enabled(false);
                print_warning("Autosuggest: OFF");
            }
            _ => print_error(&format!("Invalid value: {}. Use ON or OFF.", args)),
        }
    }
    Ok(false)
}

pub fn cmd_sett(args: &str, _nav: &mut Navigator) -> Result<bool> {
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
    Ok(false)
}

pub fn cmd_seth(args: &str, _nav: &mut Navigator) -> Result<bool> {
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
    Ok(false)
}

pub fn cmd_watch(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let enabled = Config::global_get_file_watcher_enabled();
        let trigger = Config::global_get_watch_trigger_alias();
        println!("File watcher: {}", if enabled { "ON".green() } else { "OFF".red() });
        match trigger {
            Some(ref a) => println!("Trigger alias: {}", a.cyan()),
            None        => println!("Trigger alias: {}", "none".dimmed()),
        }
        println!("Usage: watch ON|OFF");
        println!("       watch trigger <alias>   Auto-run alias on change");
        println!("       watch trigger off        Disable auto-run");
    } else {
        let upper = args.to_uppercase();
        if upper == "ON" {
            Config::global_set_file_watcher_enabled(true);
            print_success("File watcher: ON (restart ntc to activate)");
        } else if upper == "OFF" {
            Config::global_set_file_watcher_enabled(false);
            print_warning("File watcher: OFF (restart ntc to deactivate)");
        } else {
            let parts: Vec<&str> = args.splitn(2, ' ').collect();
            if parts[0].to_lowercase() == "trigger" {
                let target = parts.get(1).unwrap_or(&"").trim();
                if target.is_empty() || target.to_lowercase() == "off" {
                    Config::global_set_watch_trigger_alias(None);
                    print_success("Watch trigger alias cleared.");
                } else {
                    let aliases = Config::global_get_run_aliases();
                    if aliases.contains_key(&target.to_lowercase()) {
                        Config::global_set_watch_trigger_alias(Some(target.to_lowercase()));
                        print_success(&format!("Watch trigger set to alias: '{}'", target));
                    } else {
                        print_error(&format!(
                            "Alias '{}' not found. Use 'ral list' to see aliases.",
                            target
                        ));
                    }
                }
            } else {
                print_error("Use: watch ON | watch OFF | watch trigger <alias> | watch trigger off");
            }
        }
    }
    Ok(false)
}

pub fn cmd_showcg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
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
    println!("│ {:<20} {:<42} │", "Color:", if Config::global_get_color_enabled() { "ON".green() } else { "OFF".red() });
    println!("└{}┘", "─".repeat(w));
    println!();
    Ok(false)
}

pub fn cmd_opencg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let config_path = dirs::config_dir()
        .map(|d| d.join("ntc").join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("ntc_config.toml"));

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
    Ok(false)
}

pub fn cmd_resetcg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let config_path = dirs::config_dir()
        .map(|d| d.join("ntc").join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("ntc_config.toml"));

    if !config_path.exists() {
        print_warning("Config file not found. Nothing to reset.");
        return Ok(false);
    }

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

            let default_config = Config::new();
            if let Ok(toml) = toml::to_string_pretty(&default_config) {
                match std::fs::write(&config_path, toml) {
                    Ok(_) => {
                        print_success("Config reset to defaults!");
                        print_info("Backup saved. Use 'restorecg' to restore if needed.");

                        let mut cfg = Config::write_global();
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
            let default_config = Config::new();
            if let Ok(toml) = toml::to_string_pretty(&default_config) {
                match std::fs::write(&config_path, toml) {
                    Ok(_) => {
                        print_success("Config reset to defaults!");

                        let mut cfg = Config::write_global();
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
    Ok(false)
}

pub fn cmd_restorecg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let config_dir = dirs::config_dir()
        .map(|d| d.join("ntc"))
        .unwrap_or_else(|| PathBuf::from("."));

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

            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let current_backup = config_dir.join(format!("config_before_restore_{}.toml.bak", timestamp));
            let _ = std::fs::copy(&config_path, &current_backup);

            match std::fs::copy(backup_path, &config_path) {
                Ok(_) => {
                    print_success(&format!("Restored config from: {}", backup_path.file_name().unwrap_or_default().to_string_lossy()));
                    print_info(&format!("Current config backed up to: {}", current_backup.file_name().unwrap_or_default().to_string_lossy()));

                    let mut cfg = Config::write_global();
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
    Ok(false)
}

pub fn cmd_local(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let subcmd = parts.first().map(|s| *s).unwrap_or("");
    let subargs = parts.get(1..).unwrap_or(&[]).join(" ");

    match subcmd {
        "init" => cmd_local_init(&subargs, nav),
        "deinit" => cmd_local_deinit(nav),
        "help" | "-h" | "--help" => { cmd_local_help(); Ok(false) }
        _ => {
            if subcmd.is_empty() {
                cmd_local_help();
            } else {
                print_error(&format!("Unknown local subcommand: '{}'. Use 'local help' for usage.", subcmd));
            }
            Ok(false)
        }
    }
}

fn cmd_local_init(args: &str, nav: &mut Navigator) -> Result<bool> {
    let current_dir = nav.current_path();
    let ntconfig_path = current_dir.join("ntconfig.toml");
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
            println!("{}", "Initialisation cancelled.".dimmed());
            return Ok(false);
        }
    }

    let toml_content = if export_all {
        let current_cfg = Config::read_global();

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
            result.push(']');
            result
        }

        let ignored_dirs_str = format_toml_array(&current_cfg.ignored_directory_names);
        let ignored_exts_str = format_toml_array(&current_cfg.ignored_extensions);
        let extra_exts_str = format_toml_array(&current_cfg.extra_supported_extensions);
        let ignored_files_str = format_toml_array(&current_cfg.ignored_files);
        let extra_files_str = format_toml_array(&current_cfg.extra_supported_files);

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
        r#"# ntc local configuration file
# Place this file in any directory to override global ignore/care and run alias settings
# 
# INSTRUCTIONS:
# 1. Remove the '#' from lines you want to enable
# 2. Add your project-specific values
# 3. Run 'ntc' in this directory to activate
#
# For quick setup, run: local init --all  (copies your current global settings)

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
                println!("{}", "   Or run: local init --all  to export your current settings".dimmed());
            }
            println!();
            println!("{}", "💡 Tip: Global settings (teleports, output path, depth) stay global".dimmed());
        }
        Err(e) => print_error(&format!("Failed to create ntconfig.toml: {}", e)),
    }

    Ok(false)
}

fn cmd_local_deinit(nav: &mut Navigator) -> Result<bool> {
    let ntconfig_path = nav.current_path().join("ntconfig.toml");

    if !ntconfig_path.exists() {
        print_warning("No ntconfig.toml found in the current directory.");
        return Ok(false);
    }

    println!();
    println!("{}", "⚠️  LOCAL CONFIG REMOVAL".red().bold());
    println!("{}", "═".repeat(50).red());
    println!("This will delete: {}", ntconfig_path.display().to_string().yellow());
    println!("Local ignore/care and run alias settings will be lost.");
    println!("Global config will NOT be affected.");
    println!();
    print!("{} ", "Delete ntconfig.toml? (y/N): ".red());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("{}", "Deinitialisation cancelled.".dimmed());
        return Ok(false);
    }

    match std::fs::remove_file(&ntconfig_path) {
        Ok(_) => {
            print_success("ntconfig.toml removed. Global config is now the only active config.");
            Config::reload_global();
        }
        Err(e) => print_error(&format!("Failed to remove ntconfig.toml: {}", e)),
    }

    Ok(false)
}

fn cmd_local_help() {
    println!();
    println!("{}", "local — Manage local ntconfig.toml configuration".cyan().bold());
    println!();
    println!("  local init                 Create a commented ntconfig.toml template");
    println!("  local init --all           Export current global settings to ntconfig.toml");
    println!("  local deinit               Remove ntconfig.toml from current directory");
    println!("  local help                 Show this help");
    println!();
    println!("{}", "The local ntconfig.toml overrides global ignore/care and run alias settings");
    println!("for the current directory only. It does NOT affect teleports, output path,");
    println!("max depth, or other global settings.");
    println!();
}

// ── top-level init: create both NTCRANFILE.toml and ntconfig.toml ─────────

use crate::ranfile::parser::{RANFILE_NAME, RANFILE_TEMPLATE};

pub fn cmd_init(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim().to_lowercase();
    let do_ran = args.is_empty() || args == "--all" || args == "-a" || args == "--ran";
    let do_local = args.is_empty() || args == "--all" || args == "-a" || args == "--local";

    if !do_ran && !do_local {
        print_error("Usage: init [--all | --ran | --local]");
        return Ok(false);
    }

    if do_ran {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&format!("Failed to get current directory: {}", e));
                return Ok(false);
            }
        };
        let path = cwd.join(RANFILE_NAME);
        if path.exists() {
            print_warning(&format!("{} already exists, skipping.", RANFILE_NAME));
        } else {
            match std::fs::write(&path, RANFILE_TEMPLATE) {
                Ok(_) => print_success(&format!("Created {}", path.display())),
                Err(e) => print_error(&format!("Failed to create {}: {}", path.display(), e)),
            }
        }
    }

    if do_local {
        let current_dir = nav.current_path();
        let ntconfig_path = current_dir.join("ntconfig.toml");

        if ntconfig_path.exists() {
            print_warning("ntconfig.toml already exists, skipping.");
        } else {
            let toml_content = r#"# ntc local configuration file
# Place this file in any directory to override global ignore/care and run alias settings
#
# INSTRUCTIONS:
# 1. Remove the '#' from lines you want to enable
# 2. Add your project-specific values
#
# For quick setup, run: local init --all  (copies your current global settings)

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
"#;

            match std::fs::write(&ntconfig_path, toml_content) {
                Ok(_) => print_success(&format!("Created {}", ntconfig_path.display())),
                Err(e) => print_error(&format!("Failed to create ntconfig.toml: {}", e)),
            }
        }
    }

    Ok(false)
}

// ── top-level deinit: remove both NTCRANFILE.toml and ntconfig.toml ──────

pub fn cmd_deinit(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim().to_lowercase();
    let do_ran = args.is_empty() || args == "--all" || args == "-a" || args == "--ran";
    let do_local = args.is_empty() || args == "--all" || args == "-a" || args == "--local";

    if !do_ran && !do_local {
        print_error("Usage: deinit [--all | --ran | --local]");
        return Ok(false);
    }

    if do_ran {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&format!("Failed to get current directory: {}", e));
                return Ok(false);
            }
        };
        let path = cwd.join(RANFILE_NAME);
        if !path.exists() {
            print_warning(&format!("No {} found, skipping.", RANFILE_NAME));
        } else {
            print!("{} Delete {}? (y/N): ", "⚠".red().bold(), path.display().to_string().yellow());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                match std::fs::remove_file(&path) {
                    Ok(_) => print_success(&format!("Removed {}", path.display())),
                    Err(e) => print_error(&format!("Failed to remove {}: {}", path.display(), e)),
                }
            } else {
                println!("{}", "Skipped.".dimmed());
            }
        }
    }

    if do_local {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&format!("Failed to get current directory: {}", e));
                return Ok(false);
            }
        };
        let path = cwd.join("ntconfig.toml");
        if !path.exists() {
            print_warning("No ntconfig.toml found, skipping.");
        } else {
            print!("{} Delete {}? (y/N): ", "⚠".red().bold(), path.display().to_string().yellow());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                match std::fs::remove_file(&path) {
                    Ok(_) => {
                        print_success("ntconfig.toml removed.");
                        Config::reload_global();
                    }
                    Err(e) => print_error(&format!("Failed to remove ntconfig.toml: {}", e)),
                }
            } else {
                println!("{}", "Skipped.".dimmed());
            }
        }
    }

    Ok(false)
}
