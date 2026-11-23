//! `showcg` — show the effective global configuration.

pub(crate) mod help;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::config_display::{on_off, print_serve_row};

pub(crate) fn cmd_showcg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let w = 65;
    println!();
    println!("{}", t!("msg.showcg.box_top", "─".repeat(w)));
    println!("│{:^w$}│", t!("msg.showcg.title").cyan().bold(), w = w);
    println!("{}", t!("msg.showcg.box_divider", "─".repeat(w)));
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_output_path"),
        Config::global_get_output_path()
            .display()
            .to_string()
            .green()
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_max_depth"),
        Config::global_get_max_depth().to_string().yellow()
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_line_numbers"),
        if Config::global_get_show_line_numbers() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_threads"),
        Config::global_get_num_threads().to_string().yellow()
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_history"),
        if Config::global_get_history_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_watcher"),
        if Config::global_get_file_watcher_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_watch_limit"),
        Config::global_get_watch_limit().to_string().yellow()
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_color"),
        if Config::global_get_color_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_dir_view"),
        if Config::global_get_dir_view_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_file_view"),
        if Config::global_get_file_view_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_auto_size"),
        t!(
            "msg.showcg.auto_size_value",
            on_off(Config::global_get_auto_size_files()),
            on_off(Config::global_get_auto_size_dirs())
        )
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_update_check"),
        if Config::global_get_update_check_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_intro_view"),
        if Config::global_get_intro_view_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_integration_shell"),
        if Config::global_get_integration_shell_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
        t!("msg.showcg.label_code_tools"),
        if Config::global_get_code_tools_enabled() {
            t!("status.on").green()
        } else {
            t!("status.off").red()
        }
    );
    println!(
        "│ {:<20} {:<42}",
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
        "│ {:<20} {:<42}",
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
    // One row per serve service, read from the shared service table so a new
    // service never needs a second list here.
    for row in super::serve::service::SERVICES {
        print_serve_row(&t!(row.label_key), row.service);
    }
    println!("{}", t!("msg.showcg.box_bottom", "─".repeat(w)));
    println!();
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "showcg",
    aliases: &[],
    help_topic: Some("showcg"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_showcg),
};
