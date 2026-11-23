//! `opencg` — create the config file if needed and open it in the editor.

pub(crate) mod help;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use anyhow::Result;
use std::path::PathBuf;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::ne::launch_ne_editor;

pub(crate) fn cmd_opencg(_args: &str, _nav: &mut Navigator) -> Result<bool> {
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

    // The built-in editor is the supported way to edit config now.
    // Refuse nesting inside the editor's embedded ntc shell.
    if std::env::var("NTC_IN_EDITOR").is_ok() {
        print_error(&t!("misc.ne.already_in_editor"));
        return Ok(false);
    }
    print_success(&t!("cmd.config.opencg.opening", config_path.display()));
    launch_ne_editor(config_path, None);
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "opencg",
    aliases: &["editcfg", "editcg"],
    help_topic: Some("opencg"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_opencg),
};
