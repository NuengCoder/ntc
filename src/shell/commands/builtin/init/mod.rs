//! `init` — create the run file, local config, and/or OpenCode scaffold.

pub(crate) mod help;

mod project;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use crate::shell::commands::builtin::ran::ranfile::parser;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::init_scope::parse_scope;
use super::super::support::local_config::LOCAL_CONFIG_TEMPLATE;
use super::super::support::opencode::scaffold_opencode;

pub(crate) fn cmd_init(args: &str, nav: &mut Navigator) -> Result<bool> {
    let trimmed = args.trim();

    // `init project` needs the raw, case-preserved project name, so route it
    // before the lowercase subcommand parser below.
    if let Some(first) = trimmed.split_whitespace().next() {
        if first.eq_ignore_ascii_case("project") {
            let rest = trimmed[first.len()..].trim();
            return project::cmd_init_project(rest, nav);
        }
    }

    // The old catch-all `init all` was removed: it silently scaffolded every
    // part, including `.opencode/`, for people who only wanted a run file.
    if trimmed
        .split_whitespace()
        .any(|token| token.eq_ignore_ascii_case("all"))
    {
        print_error(&t!("cmd.config.init.all_removed"));
        return Ok(false);
    }

    let scope = match parse_scope(trimmed) {
        Some(scope) => scope,
        None => {
            print_error(&t!("cmd.config.init.usage"));
            return Ok(false);
        }
    };

    if scope.ran {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&t!("cmd.config.init.create_failed", "current directory", e));
                return Ok(false);
            }
        };
        let fmt = parser::preferred_format();
        let path = cwd.join(fmt.file_name());
        if path.exists() {
            print_warning(&t!("cmd.config.init.exists_warn", fmt.file_name()));
        } else {
            // Another format may still be here; the preferred one is what
            // `ran` reads, so create that and name the file left behind.
            if let Some((other, _)) = parser::resolve(&cwd) {
                let other_name = other
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(parser::ranfile_names);
                println!("{}", t!("ran.fmt.old_kept", other_name, fmt.file_name()));
            }
            let created: anyhow::Result<()> = parser::template(fmt).and_then(|body| {
                std::fs::write(&path, body)?;
                Ok(())
            });
            match created {
                Ok(_) => print_success(&t!("cmd.config.init.created", path.display())),
                Err(e) => print_error(&t!("cmd.config.init.create_failed", path.display(), e)),
            }
        }
    }

    if scope.local {
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

    if scope.opencode {
        scaffold_opencode(nav.current_path(), scope.force);
    }

    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "init",
    aliases: &[],
    help_topic: Some("init"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_init),
};

#[cfg(test)]
fn cmd_init_opencode(force: bool) {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            print_error(&t!("cmd.config.init.create_failed", "current directory", e));
            return;
        }
    };
    scaffold_opencode(&cwd, force);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::commands::support::opencode::prune_opencode;

    #[test]
    fn init_all_is_rejected() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let mut nav = Navigator::new().unwrap();

        cmd_init("all", &mut nav).unwrap();
        assert!(!std::path::Path::new("ntconfig.toml").exists());
        assert!(!std::path::Path::new(".opencode").exists());
        assert!(!std::path::Path::new(parser::preferred_format().file_name()).exists());
    }

    #[test]
    fn init_positional_creates_only_requested_parts() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let mut nav = Navigator::new().unwrap();

        cmd_init("ran", &mut nav).unwrap();
        assert!(std::path::Path::new(parser::preferred_format().file_name()).exists());
        assert!(!std::path::Path::new("ntconfig.toml").exists());

        cmd_init("local", &mut nav).unwrap();
        assert!(std::path::Path::new("ntconfig.toml").exists());

        // The old flag syntax is rejected.
        cmd_init("--all", &mut nav).unwrap();
        assert!(!std::path::Path::new(".opencode").exists());
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
            include_str!("assets/opencode_template/opencode.jsonc")
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
            include_str!("assets/opencode_template/ntc.ts")
        );
    }

    #[test]
    fn opencode_deinit_prunes_only_scaffold() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let base = std::env::current_dir().unwrap().join(".opencode");

        // Scenario 1: a pure scaffold is removed completely.
        cmd_init_opencode(false);
        prune_opencode(&base);
        assert!(!base.exists(), "pure scaffold should be fully removed");

        // Scenario 2: a user file keeps its directory (and .opencode/) alive.
        cmd_init_opencode(false);
        std::fs::write(base.join("tools/mine.ts"), "x").unwrap();
        prune_opencode(&base);

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
