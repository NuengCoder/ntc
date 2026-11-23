//! `resetcg` — reset the global config, optionally keeping a backup.

pub(crate) mod help;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_resetcg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
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

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "resetcg",
    aliases: &["reset-config"],
    help_topic: Some("resetcg"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_resetcg),
};
