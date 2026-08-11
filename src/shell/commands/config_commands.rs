//! Config management commands: `config` (settings subcommands), `showcg`,
//! `opencg`, `resetcg`, `restorecg`, `local`, `init`, `deinit`.

use crate::config::Config;
use crate::config::VALID_LANGUAGES;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::helpers::open_with_fallback;
use crate::t;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn cmd_config(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "output" | "out" => config_output(subargs),
        "depth" | "d" => config_depth(subargs),
        "linenumbers" | "ln" | "line" => config_linenumbers(subargs),
        "color" | "c" => config_color(subargs),
        "autosuggest" | "as" => config_autosuggest(subargs),
        "threads" | "t" => config_threads(subargs),
        "history" | "h" => config_history(subargs),
        "watch" | "w" => config_watch(subargs),
        "qsc" => config_qsc(subargs),
        "lang" | "language" => config_lang(subargs),
        _ => {
            println!("{}", t!("cmd.config.title").cyan().bold());
            println!();
            println!("{}", t!("cmd.config.settings_header").yellow());
            println!("  {}", t!("help.config.output"));
            println!("  {}", t!("help.config.depth"));
            println!("  {}", t!("help.config.linenumbers"));
            println!("  {}", t!("help.config.color"));
            println!("  {}", t!("help.config.autosuggest"));
            println!("  {}", t!("help.config.threads"));
            println!("  {}", t!("help.config.history"));
            println!("  {}", t!("help.config.watch"));
            println!("  {}", t!("help.config.qsc"));
            println!("  {}", t!("help.config.qsc_off"));
            println!("  {}", t!("help.config.watch_trigger"));
            println!("  {}", t!("help.config.watch_trigger_off"));
            println!("  {}", t!("help.config.lang"));
            println!();
            println!("{}", t!("cmd.config.mgmt_header").yellow());
            println!("  {}", t!("help.config.showcg"));
            println!("  {}", t!("help.config.opencg"));
            println!("  {}", t!("help.config.resetcg"));
            println!("  {}", t!("help.config.restorecg"));
            println!("  {}", t!("help.config.local_init"));
            println!("  {}", t!("help.config.local_init_all"));
            println!("  {}", t!("help.config.local_deinit"));
            println!("  {}", t!("help.config.local_help"));
            println!("  {}", t!("help.config.showenv"));
            println!("  {}", t!("help.config.showenv_all"));
            println!("  {}", t!("help.config.showenv_d"));
            println!();
            println!("{}", t!("cmd.config.examples_header").green());
            println!("  {}", t!("cmd.config.example_output"));
            println!("  {}", t!("cmd.config.example_depth"));
            println!("  {}", t!("cmd.config.example_linenumbers"));
            println!("  {}", t!("cmd.config.example_history"));
            println!("  {}", t!("cmd.config.example_watch"));
            Ok(false)
        }
    }
}

fn config_output(args: &str) -> Result<bool> {
    if args.is_empty() {
        println!(
            "{}",
            t!(
                "msg.setO.status",
                Config::global_get_output_path().display()
            )
        );
    } else {
        Config::global_set_output_path(Path::new(args));
        print_success(&t!(
            "msg.setO.success",
            Config::global_get_output_path().display()
        ));
    }
    Ok(false)
}

fn config_depth(args: &str) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("msg.setD.status", Config::global_get_max_depth()));
    } else {
        match args.parse::<usize>() {
            Ok(depth) => {
                Config::global_set_max_depth(depth);
                print_success(&t!("msg.setD.success", Config::global_get_max_depth()));
            }
            Err(_) => print_error(&t!("msg.setD.error_parse", args)),
        }
    }
    Ok(false)
}

fn config_linenumbers(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_show_line_numbers() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setL.status", state));
    } else {
        match Config::parse_on_off(args) {
            Some(state) => {
                Config::global_set_show_line_numbers(state);
                print_success(&t!("msg.setL.success", if state { "ON" } else { "OFF" }));
            }
            None => print_error(&t!("msg.setL.error", args)),
        }
    }
    Ok(false)
}

fn config_color(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_color_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setC.status", state));
    } else {
        match Config::parse_on_off(args) {
            Some(state) => {
                Config::global_set_color_enabled(state);
                print_success(&t!("msg.setC.success", if state { "ON" } else { "OFF" }));
            }
            None => print_error(&t!("msg.setC.error", args)),
        }
    }
    Ok(false)
}

fn config_autosuggest(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_autosuggest_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setA.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_autosuggest_enabled(true);
                print_success(&t!("msg.setA.on"));
            }
            "OFF" => {
                Config::global_set_autosuggest_enabled(false);
                print_warning(&t!("msg.setA.off"));
            }
            _ => print_error(&t!("msg.setA.error", args)),
        }
    }
    Ok(false)
}

fn config_threads(args: &str) -> Result<bool> {
    if args.is_empty() {
        println!(
            "{}",
            t!("msg.setT.status", Config::global_get_num_threads())
        );
    } else {
        match Config::parse_num_threads(args) {
            Some(threads) => {
                Config::global_set_num_threads(threads);
                print_success(&t!("msg.setT.success", Config::global_get_num_threads()));
            }
            None => print_error(&t!("msg.setT.error_parse", args)),
        }
    }
    Ok(false)
}

fn config_history(args: &str) -> Result<bool> {
    if args.is_empty() {
        let enabled = Config::global_get_history_enabled();
        let path = Config::global_get_history_path();
        println!(
            "{}",
            t!("msg.setH.status", if enabled { "ON" } else { "OFF" })
        );
        match path {
            Some(p) => println!("{}", t!("msg.setH.path", p.display())),
            None => println!("{}", t!("msg.setH.path_default")),
        }
    } else {
        let upper = args.to_uppercase();
        if upper == "ON" {
            Config::global_set_history_enabled(true);
            print_success(&t!("msg.setH.on"));
        } else if upper == "OFF" {
            Config::global_set_history_enabled(false);
            print_warning(&t!("msg.setH.off"));
        } else if args == "default" {
            Config::global_set_history_path(None);
            print_success(&t!("msg.setH.path_reset"));
        } else {
            let p = Path::new(args);
            Config::global_set_history_path(Some(p.to_path_buf()));
            print_success(&t!("msg.setH.path_set", p.display()));
        }
    }
    Ok(false)
}

fn config_watch(args: &str) -> Result<bool> {
    if args.is_empty() {
        let enabled = Config::global_get_file_watcher_enabled();
        let trigger = Config::global_get_watch_trigger_alias();
        println!(
            "{}",
            t!("msg.watch.status", if enabled { "ON" } else { "OFF" })
        );
        match trigger {
            Some(ref a) => println!("{} {}", t!("cmd.config.watch.trigger_label"), a.cyan()),
            None => println!(
                "{} {}",
                t!("cmd.config.watch.trigger_label"),
                t!("status.none").dimmed()
            ),
        }
        println!("{}", t!("cmd.config.watch.usage_line"));
        println!("       {}", t!("cmd.config.watch.trigger_usage"));
        println!("       {}", t!("cmd.config.watch.trigger_off_usage"));
    } else {
        let upper = args.to_uppercase();
        if upper == "ON" {
            Config::global_set_file_watcher_enabled(true);
            print_success(&t!("msg.watch.on"));
        } else if upper == "OFF" {
            Config::global_set_file_watcher_enabled(false);
            print_warning(&t!("msg.watch.off"));
        } else {
            let parts: Vec<&str> = args.splitn(2, ' ').collect();
            if parts[0].to_lowercase() == "trigger" {
                let target = parts.get(1).unwrap_or(&"").trim();
                if target.is_empty() || target.to_lowercase() == "off" {
                    Config::global_set_watch_trigger_alias(None);
                    print_success(&t!("cmd.config.watch.trigger_cleared"));
                } else {
                    let aliases = Config::global_get_run_aliases();
                    if aliases.contains_key(&target.to_lowercase()) {
                        Config::global_set_watch_trigger_alias(Some(target.to_lowercase()));
                        print_success(&t!("cmd.config.watch.trigger_set", target));
                    } else {
                        print_error(&t!("cmd.config.watch.alias_not_found", target));
                    }
                }
            } else {
                print_error(&t!("cmd.config.watch.usage"));
            }
        }
    }
    Ok(false)
}

fn config_qsc(args: &str) -> Result<bool> {
    if args.is_empty() {
        match Config::global_get_qsc() {
            Some(ref hk) => {
                let mut parts = Vec::new();
                if hk.ctrl {
                    parts.push("Ctrl".to_string());
                }
                if hk.shift {
                    parts.push("Shift".to_string());
                }
                if hk.alt {
                    parts.push("Alt".to_string());
                }
                parts.push(hk.key.clone());
                println!(
                    "{} {}",
                    t!("cmd.config.qsc.hotkey_status"),
                    parts.join("+").cyan()
                );
            }
            None => println!(
                "{} {}",
                t!("cmd.config.qsc.hotkey_status"),
                t!("cmd.config.qsc.not_configured").dimmed()
            ),
        }
        println!("{}", t!("cmd.config.qsc.usage1"));
        println!("{}", t!("cmd.config.qsc.usage2"));
        println!("{}", t!("cmd.config.qsc.usage3"));
        println!("{}", t!("cmd.config.qsc.usage4"));
    } else {
        match Config::parse_qsc(args) {
            Some(hk) => {
                Config::global_set_qsc(Some(hk.clone()));
                let mut parts = Vec::new();
                if hk.ctrl {
                    parts.push("Ctrl".to_string());
                }
                if hk.shift {
                    parts.push("Shift".to_string());
                }
                if hk.alt {
                    parts.push("Alt".to_string());
                }
                parts.push(hk.key.clone());
                print_success(&t!("cmd.config.qsc.hotkey_set", parts.join("+")));
            }
            None if args.trim().eq_ignore_ascii_case("off") => {
                Config::global_set_qsc(None);
                print_success(&t!("cmd.config.qsc.disabled"));
            }
            None => {
                print_error(&t!("cmd.config.qsc.invalid_format", args));
            }
        }
    }
    Ok(false)
}

fn config_lang(args: &str) -> Result<bool> {
    if args.is_empty() {
        let current = Config::global_get_language();
        let active = Config::global_get_language_active();
        println!("{}", t!("cmd.config.lang.available").cyan().bold());
        for lang in VALID_LANGUAGES {
            if *lang == current {
                if active {
                    println!("  {}", t!("cmd.config.lang.active", lang).green());
                } else {
                    println!("  {}", t!("cmd.config.lang.coming_soon", lang).yellow());
                }
            } else {
                println!("  {}", lang);
            }
        }
    } else {
        let lang = args.trim().to_lowercase();
        if !VALID_LANGUAGES.contains(&lang.as_str()) {
            print_error(&t!(
                "cmd.config.lang.unsupported",
                args,
                VALID_LANGUAGES.join(", ")
            ));
            return Ok(false);
        }
        Config::global_set_language(&lang);
        crate::utils::config::lang::reload();
        Config::global_set_language_active(true);
        print_success(&t!("cmd.config.lang.set", lang));
    }
    Ok(false)
}

pub fn cmd_showcg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let w = 65;
    println!();
    println!("{}", t!("msg.showcg.box_top", "─".repeat(w)));
    println!("│{:^w$}│", t!("msg.showcg.title").cyan().bold(), w = w);
    println!("{}", t!("msg.showcg.box_divider", "─".repeat(w)));
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_output_path"),
        Config::global_get_output_path()
            .display()
            .to_string()
            .green()
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_max_depth"),
        Config::global_get_max_depth().to_string().yellow()
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_line_numbers"),
        if Config::global_get_show_line_numbers() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_threads"),
        Config::global_get_num_threads().to_string().yellow()
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_history"),
        if Config::global_get_history_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_watcher"),
        if Config::global_get_file_watcher_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_color"),
        if Config::global_get_color_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("cmd.config.showcg.qsc_label"),
        match Config::global_get_qsc() {
            Some(ref hk) => {
                let mut parts = Vec::new();
                if hk.ctrl {
                    parts.push("Ctrl");
                }
                if hk.shift {
                    parts.push("Shift");
                }
                if hk.alt {
                    parts.push("Alt");
                }
                parts.push(&hk.key);
                parts.join("+").cyan().to_string()
            }
            None => t!("status.off").dimmed().to_string(),
        }
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("cmd.config.showcg.lang_label"),
        if Config::global_get_language_active() {
            Config::global_get_language().green().to_string()
        } else {
            t!(
                "cmd.config.showcg.lang_coming_soon",
                Config::global_get_language()
            )
            .yellow()
            .to_string()
        }
    );
    println!("{}", t!("msg.showcg.box_bottom", "─".repeat(w)));
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
        print_info(&t!("cmd.config.opencg.created", config_path.display()));
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
                    Ok(_) => print_success(&t!("cmd.config.opencg.opening", config_path.display())),
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
        print_warning(&t!("cmd.config.resetcg.not_found"));
        return Ok(false);
    }

    println!();
    println!("{}", t!("cmd.config.resetcg.warning_title").red().bold());
    println!("{}", crate::utils::terminal::asciify(&"═".repeat(50)).red());
    println!(
        "{} {}",
        t!("cmd.config.resetcg.config_file_label"),
        config_path.display().to_string().yellow()
    );
    println!();
    println!("{}", t!("cmd.config.resetcg.warning_desc").yellow());
    println!("  {}", t!("cmd.config.resetcg.default_output"));
    println!("  {}", t!("cmd.config.resetcg.default_depth"));
    println!("  {}", t!("cmd.config.resetcg.default_linenumbers"));
    println!("  {}", t!("cmd.config.resetcg.default_threads"));
    println!("  {}", t!("cmd.config.resetcg.default_history"));
    println!("  {}", t!("cmd.config.resetcg.default_watcher"));
    println!("  {}", t!("cmd.config.resetcg.default_teleports"));
    println!("  {}", t!("cmd.config.resetcg.default_aliases"));
    println!("  {}", t!("cmd.config.resetcg.default_ignored_dirs"));
    println!("  {}", t!("cmd.config.resetcg.default_custom_ignores"));
    println!();
    println!("{}", t!("cmd.config.resetcg.options_label").cyan().bold());
    println!("{}", t!("cmd.config.resetcg.opt_yes"));
    println!("{}", t!("cmd.config.resetcg.opt_backup"));
    println!("{}", t!("cmd.config.resetcg.opt_no"));
    println!();
    print!("{} ", t!("cmd.config.resetcg.your_choice").green());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "backup" => {
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let backup_path = config_path.with_file_name(format!("config_{}.toml.bak", timestamp));

            match std::fs::copy(&config_path, &backup_path) {
                Ok(_) => print_success(&t!(
                    "cmd.config.resetcg.backup_created",
                    backup_path.display()
                )),
                Err(e) => {
                    print_error(&t!("cmd.config.resetcg.backup_failed", e));
                    return Ok(false);
                }
            }

            let default_config = Config::new();
            if let Ok(toml) = toml::to_string_pretty(&default_config) {
                match std::fs::write(&config_path, toml) {
                    Ok(_) => {
                        print_success(&t!("cmd.config.resetcg.reset_done"));
                        print_info(&t!("cmd.config.resetcg.backup_info"));

                        let mut cfg = Config::write_global();
                        *cfg = Config::load();
                        drop(cfg);
                    }
                    Err(e) => print_error(&t!("cmd.config.resetcg.write_failed", e)),
                }
            } else {
                print_error(&t!("cmd.config.resetcg.serialize_failed"));
            }
        }
        "yes" | "y" => {
            let default_config = Config::new();
            if let Ok(toml) = toml::to_string_pretty(&default_config) {
                match std::fs::write(&config_path, toml) {
                    Ok(_) => {
                        print_success(&t!("cmd.config.resetcg.reset_done"));

                        let mut cfg = Config::write_global();
                        *cfg = Config::load();
                        drop(cfg);
                    }
                    Err(e) => print_error(&t!("cmd.config.resetcg.write_failed", e)),
                }
            } else {
                print_error(&t!("cmd.config.resetcg.serialize_failed"));
            }
        }
        _ => {
            println!("{}", t!("cmd.config.resetcg.cancelled").dimmed());
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
        print_info(&t!("cmd.config.restorecg.none"));
        return Ok(false);
    }

    backups.sort_by(|a, b| {
        let a_time = std::fs::metadata(a).and_then(|m| m.modified()).ok();
        let b_time = std::fs::metadata(b).and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    println!();
    println!("{}", t!("cmd.config.restorecg.title").cyan().bold());
    for (i, backup) in backups.iter().enumerate() {
        if let Ok(metadata) = std::fs::metadata(backup) {
            if let Ok(time) = metadata.modified() {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    backup.file_name().unwrap_or_default().to_string_lossy(),
                    datetime.format("%Y-%m-%d %H:%M:%S")
                );
            } else {
                println!(
                    "  {}. {}",
                    i + 1,
                    backup.file_name().unwrap_or_default().to_string_lossy()
                );
            }
        }
    }
    println!("  {}", t!("cmd.config.restorecg.cancel_opt").red());
    println!();
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "0" || input.is_empty() {
        println!("{}", t!("cmd.config.restorecg.cancelled").dimmed());
        return Ok(false);
    }

    match input.parse::<usize>() {
        Ok(num) if num > 0 && num <= backups.len() => {
            let backup_path = &backups[num - 1];
            let config_path = config_dir.join("config.toml");

            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let current_backup =
                config_dir.join(format!("config_before_restore_{}.toml.bak", timestamp));
            let _ = std::fs::copy(&config_path, &current_backup);

            match std::fs::copy(backup_path, &config_path) {
                Ok(_) => {
                    print_success(&t!(
                        "cmd.config.restorecg.restored",
                        backup_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                    ));
                    print_info(&t!(
                        "cmd.config.restorecg.backed_up",
                        current_backup
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                    ));

                    let mut cfg = Config::write_global();
                    *cfg = Config::load();
                    drop(cfg);

                    print_success(&t!("cmd.config.restorecg.reloaded"));
                }
                Err(e) => print_error(&t!("cmd.config.restorecg.failed", e)),
            }
        }
        _ => {
            print_error(&t!("cmd.config.restorecg.invalid"));
        }
    }
    Ok(false)
}

pub fn cmd_local(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1..).unwrap_or(&[]).join(" ");

    match subcmd {
        "init" => cmd_local_init(&subargs, nav),
        "deinit" => cmd_local_deinit(nav),
        "help" | "-h" | "--help" => {
            cmd_local_help();
            Ok(false)
        }
        _ => {
            if subcmd.is_empty() {
                cmd_local_help();
            } else {
                print_error(&format!(
                    "Unknown local subcommand: '{}'. Use 'local help' for usage.",
                    subcmd
                ));
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
        println!("{}", t!("cmd.config.local.exists_warning").yellow().bold());
        println!("Path: {}", ntconfig_path.display().to_string().cyan());
        println!();
        print!("{} ", t!("cmd.config.local.overwrite_prompt").red());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("{}", t!("cmd.config.local.init_cancelled").dimmed());
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
                run_aliases_toml.push_str(&format!(
                    "{} = \"{}\"\n",
                    name,
                    cmd.replace('"', "\\\"")
                ));
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
                print_success(&t!(
                    "cmd.config.local.created_export",
                    current_dir.display()
                ));
                println!();
                println!("{}", t!("cmd.config.local.export_valid").green());
                println!("  {}", t!("cmd.config.local.export_ignores"));
                println!("  {}", t!("cmd.config.local.export_extras"));
                println!("  {}", t!("cmd.config.local.export_aliases"));
                Config::reload_global();
            } else {
                print_success(&t!(
                    "cmd.config.local.created_template",
                    current_dir.display()
                ));
                println!();
                println!("{}", t!("cmd.config.local.template_hint").yellow());
                println!("{}", t!("cmd.config.local.template_edit_hint").dimmed());
                println!("{}", t!("cmd.config.local.template_all_hint").dimmed());
            }
            println!();
            println!("{}", t!("cmd.config.local.tip_global").dimmed());
        }
        Err(e) => print_error(&t!("cmd.config.local.create_failed", e)),
    }

    Ok(false)
}

fn cmd_local_deinit(nav: &mut Navigator) -> Result<bool> {
    let ntconfig_path = nav.current_path().join("ntconfig.toml");

    if !ntconfig_path.exists() {
        print_warning(&t!("cmd.config.local_deinit.not_found"));
        return Ok(false);
    }

    println!();
    println!("{}", t!("cmd.config.local_deinit.title").red().bold());
    println!("{}", crate::utils::terminal::asciify(&"═".repeat(50)).red());
    println!(
        "{}",
        t!(
            "cmd.config.local_deinit.will_delete",
            ntconfig_path.display()
        )
    );
    println!("{}", t!("cmd.config.local_deinit.lose_settings"));
    println!("{}", t!("cmd.config.local_deinit.global_safe"));
    println!();
    print!("{} ", t!("cmd.config.local_deinit.confirm").red());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("{}", t!("cmd.config.local_deinit.cancelled").dimmed());
        return Ok(false);
    }

    match std::fs::remove_file(&ntconfig_path) {
        Ok(_) => {
            print_success(&t!("cmd.config.local_deinit.done"));
            Config::reload_global();
        }
        Err(e) => print_error(&t!("cmd.config.local_deinit.remove_failed", e)),
    }

    Ok(false)
}

fn cmd_local_help() {
    println!();
    println!(
        "{}",
        "local — Manage local ntconfig.toml configuration"
            .cyan()
            .bold()
    );
    println!();
    println!("  local init                 Create a commented ntconfig.toml template");
    println!("  local init --all           Export current global settings to ntconfig.toml");
    println!("  local deinit               Remove ntconfig.toml from current directory");
    println!("  local help                 Show this help");
    println!();
    println!("The local ntconfig.toml overrides global ignore/care and run alias settings");
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
        print_error(&t!("cmd.config.init.usage"));
        return Ok(false);
    }

    if do_ran {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&t!("cmd.config.init.create_failed", "current directory", e));
                return Ok(false);
            }
        };
        let path = cwd.join(RANFILE_NAME);
        if path.exists() {
            print_warning(&t!("cmd.config.init.exists_warn", RANFILE_NAME));
        } else {
            match std::fs::write(&path, RANFILE_TEMPLATE) {
                Ok(_) => print_success(&t!("cmd.config.init.created", path.display())),
                Err(e) => print_error(&t!("cmd.config.init.create_failed", path.display(), e)),
            }
        }
    }

    if do_local {
        let current_dir = nav.current_path();
        let ntconfig_path = current_dir.join("ntconfig.toml");

        if ntconfig_path.exists() {
            print_warning(&t!("cmd.config.init.local_exists"));
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
        print_error(&t!("cmd.config.deinit.usage"));
        return Ok(false);
    }

    if do_ran {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&t!(
                    "cmd.config.deinit.remove_failed",
                    "current directory",
                    e
                ));
                return Ok(false);
            }
        };
        let path = cwd.join(RANFILE_NAME);
        if !path.exists() {
            print_warning(&t!("cmd.config.deinit.not_found", RANFILE_NAME));
        } else {
            print!(
                "{} {}",
                "⚠".red().bold(),
                t!("cmd.config.deinit.confirm", path.display()).yellow()
            );
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                match std::fs::remove_file(&path) {
                    Ok(_) => print_success(&t!("cmd.config.deinit.removed", path.display())),
                    Err(e) => {
                        print_error(&t!("cmd.config.deinit.remove_failed", path.display(), e))
                    }
                }
            } else {
                println!("{}", t!("cmd.config.deinit.skipped").dimmed());
            }
        }
    }

    if do_local {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&t!(
                    "cmd.config.deinit.remove_failed",
                    "current directory",
                    e
                ));
                return Ok(false);
            }
        };
        let path = cwd.join("ntconfig.toml");
        if !path.exists() {
            print_warning(&t!("cmd.config.deinit.local_not_found"));
        } else {
            print!(
                "{} {}",
                "⚠".red().bold(),
                t!("cmd.config.deinit.confirm", path.display()).yellow()
            );
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                match std::fs::remove_file(&path) {
                    Ok(_) => {
                        print_success(&t!("cmd.config.deinit.local_removed"));
                        Config::reload_global();
                    }
                    Err(e) => print_error(&t!("cmd.config.deinit.local_remove_failed", e)),
                }
            } else {
                println!("{}", t!("cmd.config.deinit.skipped").dimmed());
            }
        }
    }

    Ok(false)
}
