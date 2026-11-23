//! `restorecg` — restore the global config from a timestamped backup.

pub(crate) mod help;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_restorecg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
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

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "restorecg",
    aliases: &["restore-config"],
    help_topic: Some("restorecg"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_restorecg),
};
