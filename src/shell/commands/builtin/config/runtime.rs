//! `config` runtime settings: threads, history, watcher, qsc, and language.

use crate::config::{Config, VALID_LANGUAGES};
use crate::output::{print_error, print_success, print_warning};
use crate::t;
use crate::utils::paths;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::path::Path;

pub(super) fn config_threads(args: &str) -> Result<bool> {
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

pub(super) fn config_history(args: &str) -> Result<bool> {
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
            let expanded = paths::expand_path_str(args);
            let p = Path::new(&expanded);
            Config::global_set_history_path(Some(p.to_path_buf()));
            print_success(&t!("msg.setH.path_set", p.display()));
        }
    }
    Ok(false)
}

pub(super) fn config_watch(args: &str) -> Result<bool> {
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

pub(super) fn config_qsc(args: &str) -> Result<bool> {
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

pub(super) fn config_lang(args: &str) -> Result<bool> {
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
