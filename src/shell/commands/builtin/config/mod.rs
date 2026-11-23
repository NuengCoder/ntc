//! `config` — settings dispatcher and management subcommands.

pub(crate) mod completion;
pub(crate) mod help;

mod env;
mod runtime;
mod serve;
mod settings;

use crate::navigator::Navigator;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use env::{config_env, config_rmenv};
use runtime::{config_history, config_lang, config_qsc, config_threads, config_watch};
use serve::config_serve;
use settings::{
    config_autosuggest, config_code_tools, config_color, config_depth, config_dir_view,
    config_file_view, config_git, config_idle_hint, config_integration_shell, config_intro_view,
    config_linenumbers, config_output, config_size, config_todo, config_update_check,
};

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
        "idle" | "idle-hint" | "ghost_typing" | "ghost-typing" | "gt" => config_idle_hint(subargs),
        "threads" | "t" => config_threads(subargs),
        "history" | "h" => config_history(subargs),
        "watch" | "w" => config_watch(subargs),
        "git" => config_git(subargs),
        "dir_view" | "dv" => config_dir_view(subargs),
        "file_view" | "fv" => config_file_view(subargs),
        "todo" | "td" => config_todo(subargs),
        "size" | "s" => config_size(subargs),
        "update_check" | "uc" => config_update_check(subargs),
        "intro_view" | "iv" => config_intro_view(subargs),
        "integration_shell" | "int_shell" | "is" => config_integration_shell(subargs),
        "code_tools" | "ct" => config_code_tools(subargs),
        "qsc" => config_qsc(subargs),
        "serve" => config_serve(subargs),
        "lang" | "language" => config_lang(subargs),
        "env" | "vars" => config_env(subargs),
        "rmenv" | "envrm" | "unsetenv" => config_rmenv(subargs),
        _ => {
            println!("{}", t!("cmd.config.title").cyan().bold());
            println!();
            println!("{}", t!("cmd.config.settings_header").yellow());
            println!("  {}", t!("help.config.output"));
            println!("  {}", t!("help.config.depth"));
            println!("  {}", t!("help.config.linenumbers"));
            println!("  {}", t!("help.config.color"));
            println!("  {}", t!("help.config.autosuggest"));
            println!("  {}", t!("help.config.idle"));
            println!("  {}", t!("help.config.git"));
            println!("  {}", t!("help.config.dir_view"));
            println!("  {}", t!("help.config.file_view"));
            println!("  {}", t!("help.config.todo"));
            println!("  {}", t!("help.config.size"));
            println!("  {}", t!("help.config.update_check"));
            println!("  {}", t!("help.config.intro_view"));
            println!("  {}", t!("help.config.integration_shell"));
            println!("  {}", t!("help.config.code_tools"));
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
            println!("  {}", t!("help.showcg.usage"));
            println!("  {}", t!("help.opencg.usage"));
            println!("  {}", t!("help.resetcg.usage"));
            println!("  {}", t!("help.restorecg.usage"));
            println!("  {}", t!("help.local.init"));
            println!("  {}", t!("help.local.deinit"));
            println!("  {}", t!("help.local.help"));
            println!("  {}", t!("help.showenv.usage"));
            println!("  {}", t!("help.showenv.all"));
            println!("  {}", t!("help.showenv.depth"));
            println!("  {}", t!("help.config.env"));
            println!("  {}", t!("help.config.env_os"));
            println!("  {}", t!("help.config.rmenv"));
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

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "config",
    aliases: &["cfg"],
    help_topic: Some("config"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &["cfg"],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_config),
};

#[cfg(test)]
mod tests {
    use super::serve::config_serve;
    use super::settings::{parse_size_action, SizeAction, SizeTarget};
    use super::*;
    use crate::config::{Config, ServeService};
    use std::path::PathBuf;

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
    fn parse_size_show_all_when_empty() {
        assert_eq!(parse_size_action(""), Some(SizeAction::ShowAll));
        assert_eq!(parse_size_action("   "), Some(SizeAction::ShowAll));
    }

    #[test]
    fn parse_size_set_all_on_off() {
        assert_eq!(
            parse_size_action("on"),
            Some(SizeAction::Set {
                target: SizeTarget::All,
                state: true
            })
        );
        assert_eq!(
            parse_size_action("off"),
            Some(SizeAction::Set {
                target: SizeTarget::All,
                state: false
            })
        );
        assert_eq!(
            parse_size_action("ON"),
            Some(SizeAction::Set {
                target: SizeTarget::All,
                state: true
            })
        );
    }

    #[test]
    fn parse_size_set_target() {
        assert_eq!(
            parse_size_action("file on"),
            Some(SizeAction::Set {
                target: SizeTarget::Files,
                state: true
            })
        );
        assert_eq!(
            parse_size_action("dirs off"),
            Some(SizeAction::Set {
                target: SizeTarget::Dirs,
                state: false
            })
        );
        assert_eq!(
            parse_size_action("all off"),
            Some(SizeAction::Set {
                target: SizeTarget::All,
                state: false
            })
        );
    }

    #[test]
    fn parse_size_show_target() {
        assert_eq!(
            parse_size_action("file"),
            Some(SizeAction::ShowTarget(SizeTarget::Files))
        );
        assert_eq!(
            parse_size_action("dir"),
            Some(SizeAction::ShowTarget(SizeTarget::Dirs))
        );
        assert_eq!(
            parse_size_action("all"),
            Some(SizeAction::ShowTarget(SizeTarget::All))
        );
    }

    #[test]
    fn parse_size_rejects_invalid() {
        assert_eq!(parse_size_action("bogus"), None);
        assert_eq!(parse_size_action("file maybe"), None);
        assert_eq!(parse_size_action("all on extra"), None);
    }

    /// `config gt` / `config ghost_typing` are second names for the idle ghost
    /// typing switch, so they must reach the same setting as `config idle`.
    #[test]
    fn ghost_typing_names_toggle_the_idle_switch() {
        let mut nav = Navigator::new().unwrap();
        Config::with_test_config(Config::new(), || {
            cmd_config("gt off", &mut nav).unwrap();
            assert!(!Config::global_get_idle_hint_enabled());
            cmd_config("ghost_typing on", &mut nav).unwrap();
            assert!(Config::global_get_idle_hint_enabled());
            cmd_config("idle off", &mut nav).unwrap();
            assert!(!Config::global_get_idle_hint_enabled());
            // A bare name only reports, an unknown value is refused.
            assert!(cmd_config("gt", &mut nav).is_ok());
            assert!(cmd_config("gt maybe", &mut nav).is_ok());
            assert!(!Config::global_get_idle_hint_enabled());
        });
    }
}
