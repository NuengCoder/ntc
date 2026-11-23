//! `ral` — run-alias manager.
//!
//! The verb handlers live in [`execute`], `ral ls` rendering in [`render`] and
//! named hint lists in [`hints`]; this module only wires the registry entry.

pub(crate) mod completion;
pub(crate) mod help;

mod execute;
mod hints;
mod render;

pub(crate) use execute::cmd_ral;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ral",
    aliases: &[],
    help_topic: Some("ral"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ral),
};

#[cfg(test)]
mod tests {
    use super::render::render_alias_section;
    use crate::config::Config;
    use crate::t;
    use crate::utils::test_support::TempCwd;
    use std::collections::HashMap;

    /// `ral ls` shows a `[global]` and a `[local]` section, each with its own
    /// aliases.
    #[test]
    fn ral_ls_lists_global_and_local_sections() {
        let _cwd = TempCwd::with_fixture(
            &[("ntconfig.toml", "[run_aliases]\nllocal = \"echo local\"\n")],
            &[],
        );
        let mut cfg = Config::default();
        cfg.run_aliases
            .insert("gglobal".into(), "echo global".into());
        Config::with_test_config(cfg, || {
            let (global_aliases, local_aliases) = Config::get_run_aliases_by_source();
            let hint_specs = Config::get_run_alias_hints_merged();
            let global_section = render_alias_section(
                &t!("alias.ral.section_global"),
                &global_aliases,
                &local_aliases,
                &hint_specs,
                None,
                &t!("alias.ral.none"),
            );
            let local_section = render_alias_section(
                &t!("alias.ral.section_local"),
                &local_aliases,
                &HashMap::new(),
                &hint_specs,
                None,
                &t!("alias.ral.none"),
            );
            assert!(
                global_section.contains("[global]"),
                "got:\n{}",
                global_section
            );
            assert!(local_section.contains("[local]"), "got:\n{}", local_section);
            assert!(
                global_section.contains("gglobal"),
                "got:\n{}",
                global_section
            );
            assert!(local_section.contains("llocal"), "got:\n{}", local_section);
        });
    }

    /// The same name in both stores is listed twice and the global entry is
    /// marked as shadowed.
    #[test]
    fn ral_ls_marks_shadowed_global_entries() {
        let _cwd = TempCwd::with_fixture(
            &[("ntconfig.toml", "[run_aliases]\nll = \"echo local\"\n")],
            &[],
        );
        let mut cfg = Config::default();
        cfg.run_aliases.insert("ll".into(), "echo global".into());
        Config::with_test_config(cfg, || {
            let (global_aliases, local_aliases) = Config::get_run_aliases_by_source();
            let hint_specs = Config::get_run_alias_hints_merged();
            let global_section = render_alias_section(
                &t!("alias.ral.section_global"),
                &global_aliases,
                &local_aliases,
                &hint_specs,
                None,
                &t!("alias.ral.none"),
            );
            assert!(
                global_section.contains("echo global"),
                "got:\n{}",
                global_section
            );
            assert!(
                global_section.contains("shadowed"),
                "got:\n{}",
                global_section
            );
        });
    }

    /// Without a local config the `[local]` section says so instead of
    /// pretending the folder has local aliases.
    #[test]
    fn ral_ls_reports_a_missing_local_config() {
        let _cwd = TempCwd::with_fixture(&[], &[]);
        let mut cfg = Config::default();
        cfg.run_aliases
            .insert("gglobal".into(), "echo global".into());
        Config::with_test_config(cfg, || {
            let (_global_aliases, local_aliases) = Config::get_run_aliases_by_source();
            let hint_specs = Config::get_run_alias_hints_merged();
            let local_section = render_alias_section(
                &t!("alias.ral.section_local"),
                &local_aliases,
                &HashMap::new(),
                &hint_specs,
                None,
                &t!("alias.ral.no_local"),
            );
            assert!(local_section.contains("[local]"), "got:\n{}", local_section);
            assert!(
                local_section.contains("no local config"),
                "got:\n{}",
                local_section
            );
        });
    }
}
