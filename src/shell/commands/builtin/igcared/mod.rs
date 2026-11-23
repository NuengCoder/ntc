//! `igcared` — show ignore/care settings from global and local stores.

pub(crate) mod help;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

/// `igcared` — show the ignore/care settings, split by source
/// (`[global]` and `[local]`).
pub fn cmd_igcared(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let global = Config::global_igcare_sets();
    let local = Config::local_igcare_sets();
    let local_path = Config::get_local_config_path();

    println!();
    println!("{}", t!("cmd.igcared.section_global").cyan().bold());
    println!();
    print!("{}", render_igcare_section(&global, false));

    println!();
    println!("{}", t!("cmd.igcared.section_local").cyan().bold());
    if let Some(ref path) = local_path {
        println!(
            "{}",
            t!("cmd.igcared.local_config", path.display()).dimmed()
        );
        println!();
        print!("{}", render_igcare_section(&local, true));
    } else {
        println!();
        println!("  {}", t!("cmd.igcared.no_local").dimmed());
    }
    println!();
    println!("{}", t!("cmd.igcared.scope_help").dimmed());
    Ok(false)
}

/// Render the ignore/care rows of one store.
fn render_igcare_section(sets: &crate::config::IgcareSets, with_cared: bool) -> String {
    let mut out = String::new();
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.ignored_dirs"),
        &sets.ignored_directory_names,
        false,
    ));
    if with_cared {
        out.push_str(&render_igcare_row(
            &t!("cmd.igcared.cared_dirs"),
            &sets.cared_directory_names,
            false,
        ));
    }
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.ignored_exts"),
        &sets.ignored_extensions,
        true,
    ));
    if with_cared {
        out.push_str(&render_igcare_row(
            &t!("cmd.igcared.cared_exts"),
            &sets.cared_extensions,
            true,
        ));
    }
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.extra_exts"),
        &sets.extra_supported_extensions,
        true,
    ));
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.ignored_files"),
        &sets.ignored_files,
        false,
    ));
    if with_cared {
        out.push_str(&render_igcare_row(
            &t!("cmd.igcared.cared_files"),
            &sets.cared_files,
            false,
        ));
    }
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.extra_files"),
        &sets.extra_supported_files,
        false,
    ));
    out
}

/// Render one ignore/care row: label + one item per line.
fn render_igcare_row(
    label: &str,
    values: &std::collections::HashSet<String>,
    dotted: bool,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", label.yellow()));
    if values.is_empty() {
        out.push_str(&format!("  {}\n", t!("cmd.igcared.none").dimmed()));
        return out;
    }
    let mut sorted: Vec<&String> = values.iter().collect();
    sorted.sort();
    for item in sorted {
        if dotted {
            out.push_str(&format!("  - .{}\n", item.green()));
        } else {
            out.push_str(&format!("  - {}\n", item.green()));
        }
    }
    out
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "igcared",
    aliases: &[],
    help_topic: Some("igcared"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_igcared),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_support::TempCwd;

    /// `igcared` shows a `[global]` and a `[local]` section.
    #[test]
    fn igcared_lists_global_and_local_sections() {
        let _cwd = TempCwd::with_fixture(
            &[(
                "ntconfig.toml",
                "ignored_directory_names = [\"dist\"]\ncared_directory_names = [\"target\"]\n",
            )],
            &[],
        );
        let mut cfg = Config::default();
        cfg.ignored_directory_names.insert("build".into());
        Config::with_test_config(cfg, || {
            let global = Config::global_igcare_sets();
            let local = Config::local_igcare_sets();
            let global_section = render_igcare_section(&global, false);
            let local_section = render_igcare_section(&local, true);
            assert!(global_section.contains("build"), "got:\n{}", global_section);
            assert!(local_section.contains("dist"), "got:\n{}", local_section);
            // The local "cared" list is shown as its own row.
            assert!(local_section.contains("target"), "got:\n{}", local_section);
        });
    }

    /// Without a local config the `[local]` section says so.
    #[test]
    fn igcared_reports_a_missing_local_config() {
        let _cwd = TempCwd::with_fixture(&[], &[]);
        let local = Config::local_igcare_sets();
        assert!(local.ignored_directory_names.is_empty());
        assert!(local.cared_directory_names.is_empty());
    }
}
