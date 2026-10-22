//! Config management commands: `config` (settings subcommands), `showcg`,
//! `opencg`, `resetcg`, `restorecg`, `local`, `init`, `deinit`.

use crate::config::Config;
use crate::config::ServeService;
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
        "git" => config_git(subargs),
        "dir_view" | "dv" => config_dir_view(subargs),
        "update_check" | "uc" => config_update_check(subargs),
        "qsc" => config_qsc(subargs),
        "serve" => config_serve(subargs),
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
            println!("  {}", t!("help.config.git"));
            println!("  {}", t!("help.config.dir_view"));
            println!("  {}", t!("help.config.update_check"));
            println!("  {}", t!("help.config.threads"));
            println!("  {}", t!("help.config.history"));
            println!("  {}", t!("help.config.watch"));
            println!("  {}", t!("help.config.qsc"));
            println!("  {}", t!("help.config.qsc_off"));
            println!("  {}", t!("help.config.serve"));
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

fn config_git(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_git_integration_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setGit.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_git_integration_enabled(true);
                print_success(&t!("msg.setGit.on"));
            }
            "OFF" => {
                Config::global_set_git_integration_enabled(false);
                print_warning(&t!("msg.setGit.off"));
            }
            _ => print_error(&t!("msg.setGit.error", args)),
        }
    }
    Ok(false)
}

fn config_dir_view(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_dir_view_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setDV.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_dir_view_enabled(true);
                print_success(&t!("msg.setDV.on"));
            }
            "OFF" => {
                Config::global_set_dir_view_enabled(false);
                print_warning(&t!("msg.setDV.off"));
            }
            _ => print_error(&t!("msg.setDV.error")),
        }
    }
    Ok(false)
}

fn config_update_check(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_update_check_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setUC.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_update_check_enabled(true);
                print_success(&t!("msg.setUC.on"));
            }
            "OFF" => {
                Config::global_set_update_check_enabled(false);
                print_warning(&t!("msg.setUC.off"));
            }
            _ => print_error(&t!("msg.setUC.error")),
        }
    }
    Ok(false)
}

/// `serve config [service] [--port-default N] [--bind-default ADDR]`
/// (alias: `config serve …` — both route here). Bare call shows all
/// services; a bare service name shows that service; flags save new
/// per-service defaults (port + bind) used when `--port`/`--bind`
/// are absent at serve startup.
pub(crate) fn config_serve(args: &str) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() {
        print_serve_config_all();
        return Ok(false);
    }
    if matches!(tokens[0].to_lowercase().as_str(), "help" | "--help" | "-h") {
        print_serve_config_help();
        return Ok(false);
    }
    let Some(svc) = ServeService::parse(tokens[0]) else {
        print_error(&t!("cmd.config.serve.invalid_service", tokens[0]));
        return Ok(false);
    };

    let mut port: Option<u16> = None;
    let mut bind: Option<String> = None;
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].to_lowercase().as_str() {
            "--port-default" => {
                i += 1;
                let v = tokens.get(i).copied().unwrap_or("");
                match v.parse::<u32>() {
                    Ok(n) if (1..=65535).contains(&n) => port = Some(n as u16),
                    _ => {
                        print_error(&t!("cmd.config.serve.invalid_port", v));
                        return Ok(false);
                    }
                }
            }
            "--bind-default" => {
                i += 1;
                let v = tokens.get(i).copied().unwrap_or("");
                if v.is_empty() {
                    print_error(&t!("cmd.config.serve.empty_bind"));
                    return Ok(false);
                }
                bind = Some(v.to_string());
            }
            other => {
                print_error(&t!("cmd.config.serve.unknown_option", other));
                return Ok(false);
            }
        }
        i += 1;
    }

    if port.is_none() && bind.is_none() {
        print_serve_config_one(svc);
    } else {
        if let Some(p) = port {
            Config::global_set_serve_port(svc, p);
        }
        if let Some(b) = bind {
            Config::global_set_serve_bind(svc, &b);
        }
        print_success(&t!(
            "cmd.config.serve.saved",
            svc.name(),
            Config::global_get_serve_bind(svc),
            Config::global_get_serve_port(svc)
        ));
    }
    Ok(false)
}

fn print_serve_config_one(svc: ServeService) {
    println!(
        "{}",
        t!(
            "cmd.config.serve.status",
            svc.name(),
            Config::global_get_serve_bind(svc),
            Config::global_get_serve_port(svc)
        )
    );
}

fn print_serve_config_all() {
    println!("{}", t!("cmd.config.serve.title").cyan().bold());
    print_serve_config_one(ServeService::Files);
    print_serve_config_one(ServeService::Http);
    print_serve_config_one(ServeService::Chess);
}

fn print_serve_config_help() {
    println!("{}", t!("cmd.config.serve.title").cyan().bold());
    println!();
    println!("{}", t!("cmd.config.serve.usage"));
    println!("  {}", t!("cmd.config.serve.example_show"));
    println!("  {}", t!("cmd.config.serve.example_show_one"));
    println!("  {}", t!("cmd.config.serve.example_set"));
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
        println!(
            "{}",
            t!("msg.watch.limit_status", Config::global_get_watch_limit())
        );
        println!("{}", t!("cmd.config.watch.usage_line"));
        println!("       {}", t!("cmd.config.watch.trigger_usage"));
        println!("       {}", t!("cmd.config.watch.trigger_off_usage"));
        println!("       {}", t!("cmd.config.watch.limit_usage"));
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
                    let aliases = Config::get_run_aliases_merged();
                    if aliases.contains_key(&target.to_lowercase()) {
                        Config::global_set_watch_trigger_alias(Some(target.to_lowercase()));
                        print_success(&t!("cmd.config.watch.trigger_set", target));
                    } else {
                        print_error(&t!("cmd.config.watch.alias_not_found", target));
                    }
                }
            } else if parts[0].to_lowercase() == "limit" {
                let value = parts.get(1).unwrap_or(&"").trim();
                if value.is_empty() {
                    println!(
                        "{}",
                        t!("msg.watch.limit_status", Config::global_get_watch_limit())
                    );
                } else {
                    match value.parse::<usize>().ok().filter(|&n| n >= 1) {
                        Some(n) => {
                            Config::global_set_watch_limit(n);
                            print_success(&t!(
                                "msg.watch.limit_success",
                                Config::global_get_watch_limit()
                            ));
                        }
                        None => print_error(&t!("msg.watch.limit_error", value)),
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
        t!("msg.showcg.label_watch_limit"),
        Config::global_get_watch_limit().to_string().yellow()
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
        t!("msg.showcg.label_dir_view"),
        if Config::global_get_dir_view_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_update_check"),
        if Config::global_get_update_check_enabled() {
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
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_serve_files"),
        format!(
            "{}:{}",
            Config::global_get_serve_bind(ServeService::Files),
            Config::global_get_serve_port(ServeService::Files)
        )
        .cyan()
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_serve_http"),
        format!(
            "{}:{}",
            Config::global_get_serve_bind(ServeService::Http),
            Config::global_get_serve_port(ServeService::Http)
        )
        .cyan()
    );
    println!(
        "│ {:<20} {:<42} │",
        t!("msg.showcg.label_serve_chess"),
        format!(
            "{}:{}",
            Config::global_get_serve_bind(ServeService::Chess),
            Config::global_get_serve_port(ServeService::Chess)
        )
        .cyan()
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

/// Commented template written by `local init` (and `init --local`).
///
/// The local file is a thin layer on top of the global config: local entries
/// take priority, the global entries keep working for everything the local
/// file does not mention, and the global config is never modified.
const LOCAL_CONFIG_TEMPLATE: &str = r#"# ntc local configuration file
# Place this file in any directory to add project-specific ignore/care and
# run alias settings.
#
# INSTRUCTIONS:
# 1. Remove the '#' from lines you want to enable
# 2. Add your project-specific values
# 3. Run 'ntc' in this directory to activate
#
# PRECEDENCE:
# - Local entries win over the global ones with the same name
# - Global entries keep working for everything not listed here
# - The global config is never changed by this file
#
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

# Cared names are never ignored here, even when the global config ignores
# them (the global config itself stays untouched).
# cared_directory_names = [
#     "target",
# ]
#
# cared_extensions = [
#     "log",
# ]
#
# cared_files = [
#     "Cargo.lock",
# ]

# Run aliases for this project only
# [run_aliases]
# test = "cargo test"
# build = "cargo build --release"
# fb = "flutter clean && flutter pub get"
# py = "python $x.py"   # parameterised: call as py(filename)
"#;

fn cmd_local_init(args: &str, nav: &mut Navigator) -> Result<bool> {
    let current_dir = nav.current_path();
    let ntconfig_path = current_dir.join("ntconfig.toml");

    // `--all` used to copy the global settings into the local file, which
    // replaced the global behaviour with a stale copy. It is gone: a local
    // config can no longer shadow the global one wholesale.
    if args == "--all" || args == "-a" {
        print_error(&t!("cmd.config.local.init_all_removed"));
        return Ok(false);
    }

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

    match std::fs::write(&ntconfig_path, LOCAL_CONFIG_TEMPLATE) {
        Ok(_) => {
            print_success(&t!(
                "cmd.config.local.created_template",
                current_dir.display()
            ));
            println!();
            println!("{}", t!("cmd.config.local.template_hint").yellow());
            println!("{}", t!("cmd.config.local.template_edit_hint").dimmed());
            println!("{}", t!("cmd.config.local.precedence_hint").dimmed());
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
    println!("  local deinit               Remove ntconfig.toml from current directory");
    println!("  local help                 Show this help");
    println!();
    println!("The local ntconfig.toml adds project-specific ignore/care and run alias");
    println!("settings for the current directory only. Local entries take priority;");
    println!("global entries keep working for everything the local file does not");
    println!("define, and the global config is never modified.");
    println!();
    println!("Use '-g' / '--global' on 'ral', 'ignore', 'care' and 'igcare import' to");
    println!("write to the global config from inside a project directory.");
    println!();
}

// ── OpenCode scaffold (`init --opencode` / `deinit --opencode`) ───────────
//
// Template files live under `src/shell/commands/opencode_template/` and are
// embedded into the binary, so `ntc init --opencode` can scaffold a project
// (`.opencode/` + tools/agents/opencode.jsonc) without any external files.

/// Files written by `init --opencode`, as `(path relative to .opencode/, contents)`.
const OPENCODE_TEMPLATES: &[(&str, &str)] = &[
    ("tools/ntc.ts", include_str!("opencode_template/ntc.ts")),
    (
        "agents/ntc-plan.md",
        include_str!("opencode_template/ntc-plan.md"),
    ),
    (
        "agents/ntc-impl.md",
        include_str!("opencode_template/ntc-impl.md"),
    ),
    (
        "opencode.jsonc",
        include_str!("opencode_template/opencode.jsonc"),
    ),
];

/// Prompt for destructive confirmation unless `force` is set. Returns `true`
/// when the caller may proceed.
fn confirm_delete(path: &Path, force: bool) -> Result<bool> {
    if force {
        return Ok(true);
    }
    print!(
        "{} {}",
        "⚠".red().bold(),
        t!("cmd.config.deinit.confirm", path.display()).yellow()
    );
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes")
}

/// Create `.opencode/` with `tools/`, `agents/` and the bundled templates.
/// Existing files are skipped unless `force` is set (which overwrites only the
/// template files, never the directory or any unrelated file).
fn cmd_init_opencode(force: bool) {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            print_error(&t!("cmd.config.init.create_failed", "current directory", e));
            return;
        }
    };
    let base = cwd.join(".opencode");

    for (rel, _) in OPENCODE_TEMPLATES {
        let path = base.join(rel);
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                print_error(&t!("cmd.config.init.create_failed", parent.display(), e));
            }
        }
    }

    for (rel, contents) in OPENCODE_TEMPLATES {
        let path = base.join(rel);
        if path.exists() && !force {
            print_warning(&t!("cmd.config.init.exists_warn", path.display()));
            continue;
        }
        match std::fs::write(&path, contents) {
            Ok(_) => print_success(&t!("cmd.config.init.created", path.display())),
            Err(e) => print_error(&t!("cmd.config.init.create_failed", path.display(), e)),
        }
    }

    print_success(&t!("cmd.config.init.opencode_ready", base.display()));
}

/// Remove the bundled OpenCode scaffold files, then prune the `tools/`,
/// `agents/` and `.opencode/` directories only if they are empty. Files added
/// by the user (and their directories) are left untouched.
fn cmd_deinit_opencode(force: bool) -> Result<()> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            print_error(&t!(
                "cmd.config.deinit.remove_failed",
                "current directory",
                e
            ));
            return Ok(());
        }
    };
    let base = cwd.join(".opencode");
    if !base.exists() {
        print_warning(&t!("cmd.config.deinit.opencode_not_found"));
        return Ok(());
    }

    if !confirm_delete(&base, force)? {
        println!("{}", t!("cmd.config.deinit.skipped").dimmed());
        return Ok(());
    }

    for (rel, _) in OPENCODE_TEMPLATES {
        let path = base.join(rel);
        if !path.exists() {
            continue;
        }
        match std::fs::remove_file(&path) {
            Ok(_) => print_success(&t!("cmd.config.deinit.removed", path.display())),
            Err(e) => print_error(&t!("cmd.config.deinit.remove_failed", path.display(), e)),
        }
    }

    // `remove_dir` only succeeds when the directory is empty, so user files
    // survive and keep their parent directory in place.
    for rel in ["tools", "agents", ""] {
        let dir = if rel.is_empty() {
            base.clone()
        } else {
            base.join(rel)
        };
        if dir.is_dir() {
            let _ = std::fs::remove_dir(&dir);
        }
    }

    Ok(())
}

// ── top-level init: create NTCRANFILE.toml, ntconfig.toml, .opencode/ ─────

use crate::ranfile::parser::{RANFILE_NAME, RANFILE_TEMPLATE};

pub fn cmd_init(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<String> = args.split_whitespace().map(|s| s.to_lowercase()).collect();
    let empty = tokens.is_empty();
    let has = |flag: &str| tokens.iter().any(|t| t == flag);
    let force = has("--force") || has("-f");
    let do_ran = empty || has("--all") || has("-a") || has("--ran");
    let do_local = empty || has("--all") || has("-a") || has("--local");
    let do_opencode = has("--all") || has("-a") || has("--opencode") || has("-oc");

    if !do_ran && !do_local && !do_opencode {
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
            match std::fs::write(&ntconfig_path, LOCAL_CONFIG_TEMPLATE) {
                Ok(_) => print_success(&format!("Created {}", ntconfig_path.display())),
                Err(e) => print_error(&format!("Failed to create ntconfig.toml: {}", e)),
            }
        }
    }

    if do_opencode {
        cmd_init_opencode(force);
    }

    Ok(false)
}

// ── top-level deinit: remove both NTCRANFILE.toml and ntconfig.toml ──────

pub fn cmd_deinit(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<String> = args.split_whitespace().map(|s| s.to_lowercase()).collect();
    let empty = tokens.is_empty();
    let has = |flag: &str| tokens.iter().any(|t| t == flag);
    let force = has("--force") || has("-f");
    let do_ran = empty || has("--all") || has("-a") || has("--ran");
    let do_local = empty || has("--all") || has("-a") || has("--local");
    let do_opencode = has("--all") || has("-a") || has("--opencode") || has("-oc");

    if !do_ran && !do_local && !do_opencode {
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
        } else if confirm_delete(&path, force)? {
            match std::fs::remove_file(&path) {
                Ok(_) => print_success(&t!("cmd.config.deinit.removed", path.display())),
                Err(e) => print_error(&t!("cmd.config.deinit.remove_failed", path.display(), e)),
            }
        } else {
            println!("{}", t!("cmd.config.deinit.skipped").dimmed());
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
        } else if confirm_delete(&path, force)? {
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

    if do_opencode {
        cmd_deinit_opencode(force)?;
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `local init` writes the commented template; `local init --all` is
    /// rejected and writes nothing.
    #[test]
    fn local_init_writes_template_and_rejects_all() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let mut nav = Navigator::new().unwrap();

        // The removed flag must not create a file.
        cmd_local_init("--all", &mut nav).unwrap();
        assert!(!std::path::Path::new("ntconfig.toml").exists());

        cmd_local_init("", &mut nav).unwrap();
        let content = std::fs::read_to_string("ntconfig.toml").unwrap();
        assert!(content.contains("run_aliases"), "got:\n{}", content);
        // The template stays fully commented out.
        assert!(
            !content
                .lines()
                .any(|l| l.starts_with("ignored_directory_names")),
            "got:\n{}",
            content
        );
        assert!(
            !content
                .lines()
                .any(|l| l.starts_with("cared_directory_names")),
            "got:\n{}",
            content
        );
        // The old "copy the global settings" hint is gone.
        assert!(!content.contains("local init --all"), "got:\n{}", content);
    }

    #[test]
    fn serve_config_rejects_unknown_service() {
        // Error paths return before touching the global singleton.
        assert!(config_serve("nope").is_ok());
        assert!(config_serve("files --port-default 0").is_ok());
        assert!(config_serve("files --port-default 99999").is_ok());
        assert!(config_serve("files --port-default abc").is_ok());
        assert!(config_serve("files --bind-default").is_ok());
        assert!(config_serve("files --nope").is_ok());
    }

    #[test]
    fn serve_config_set_then_show_roundtrip() {
        // The setter path persists via save() to the test config file
        // (tests never write the real config.toml). Back it up and restore
        // afterwards so the suite never leaves residue behind. The file
        // restore runs INSIDE the serial guard so a parallel test's save()
        // can never land after our restore (a stale-default write was once
        // observed clobbering a live [serve.chess] table).
        let cfg_path = crate::config::test_config_file();
        let backup: Option<(PathBuf, Option<Vec<u8>>)> =
            Some((cfg_path.clone(), std::fs::read(&cfg_path).ok()));
        fn restore(backup: &Option<(PathBuf, Option<Vec<u8>>)>) {
            if let Some((p, prev)) = backup {
                match prev {
                    Some(bytes) => {
                        let _ = std::fs::write(p, bytes);
                    }
                    None => {
                        let _ = std::fs::remove_file(p);
                    }
                }
            }
        }
        // Swap in a clean config so global reads are deterministic.
        Config::with_test_config(Config::new(), || {
            assert!(config_serve("files --port-default 9000").is_ok());
            assert_eq!(Config::global_get_serve_port(ServeService::Files), 9000);
            assert!(config_serve("http-server --bind-default 127.0.0.1").is_ok());
            assert_eq!(
                Config::global_get_serve_bind(ServeService::Http),
                "127.0.0.1"
            );
            // Untouched service keeps defaults.
            assert_eq!(Config::global_get_serve_port(ServeService::Http), 8080);
            // Show paths don't mutate.
            assert!(config_serve("").is_ok());
            assert!(config_serve("files").is_ok());
            assert!(
                config_serve("http-server --port-default 9001 --bind-default 127.0.0.2").is_ok()
            );
            assert_eq!(Config::global_get_serve_port(ServeService::Http), 9001);
            // chess is a real service now.
            assert!(config_serve("chess").is_ok());
            assert!(config_serve("chess --port-default 9100").is_ok());
            assert_eq!(Config::global_get_serve_port(ServeService::Chess), 9100);
            assert!(config_serve("help").is_ok());
            restore(&backup);
        });
        // Best-effort outer restore (covers a panic inside the guard).
        restore(&backup);
    }

    #[test]
    fn opencode_scaffold_create_skip_and_force() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let base = std::env::current_dir().unwrap().join(".opencode");

        cmd_init_opencode(false);
        assert!(base.join("tools/ntc.ts").is_file());
        assert!(base.join("agents/ntc-plan.md").is_file());
        assert!(base.join("agents/ntc-impl.md").is_file());
        assert!(base.join("opencode.jsonc").is_file());
        assert_eq!(
            std::fs::read_to_string(base.join("opencode.jsonc")).unwrap(),
            include_str!("opencode_template/opencode.jsonc")
        );

        // User edits survive a re-run without --force.
        std::fs::write(base.join("tools/ntc.ts"), "custom").unwrap();
        cmd_init_opencode(false);
        assert_eq!(
            std::fs::read_to_string(base.join("tools/ntc.ts")).unwrap(),
            "custom"
        );

        // --force restores the bundled template (files only).
        cmd_init_opencode(true);
        assert_eq!(
            std::fs::read_to_string(base.join("tools/ntc.ts")).unwrap(),
            include_str!("opencode_template/ntc.ts")
        );
    }

    #[test]
    fn opencode_deinit_prunes_only_scaffold() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let base = std::env::current_dir().unwrap().join(".opencode");

        // Scenario 1: a pure scaffold is removed completely.
        cmd_init_opencode(false);
        cmd_deinit_opencode(true).unwrap();
        assert!(!base.exists(), "pure scaffold should be fully removed");

        // Scenario 2: a user file keeps its directory (and .opencode/) alive.
        cmd_init_opencode(false);
        std::fs::write(base.join("tools/mine.ts"), "x").unwrap();
        cmd_deinit_opencode(true).unwrap();

        assert!(!base.join("agents/ntc-plan.md").exists());
        assert!(
            !base.join("agents").exists(),
            "empty agents/ should be pruned"
        );
        assert!(!base.join("opencode.jsonc").exists());
        assert!(
            base.join("tools/mine.ts").is_file(),
            "user file must survive"
        );
        assert!(base.join("tools").is_dir(), "dir with a user file stays");
        assert!(base.exists(), ".opencode/ stays while a user file remains");
    }
}
