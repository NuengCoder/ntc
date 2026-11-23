use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::teleport::TeleportManager;
use crate::watcher;

use anyhow::Result;
use std::sync::Arc;

// ── submodules ──────────────────────────────────────────────────────────
pub(crate) mod builtin;
pub(crate) mod core;
pub(crate) mod plugin;
pub(crate) mod registry_data;
pub(crate) mod support;

// Re-exported so call sites keep using `commands::…` for shell plumbing.
pub(crate) use support::system::{
    capture_system_command, execute_system_command, init_ctrlc_handler, register_shutdown_flag,
    run_system_command, unregister_shutdown_flag,
};

// ============================================================================
// Command execution
// ============================================================================

/// Execute a full command line, handling pipelines (`|`), logical
/// operators (`&&` / `||` / `;`) and redirections (`>` `>>` `<` `<<`). Falls
/// back to a single-command dispatch when no operator is present.
pub(crate) fn execute_command(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    execute_command_with_status(input, nav, watcher_handle).map(|(exit_shell, _)| exit_shell)
}

/// Execute a full command line and report both whether the shell should exit
/// and whether the final command/pipeline segment succeeded.
pub(crate) fn execute_command_with_status(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<(bool, bool)> {
    if crate::shell::pipeline::has_operators(input) {
        return crate::shell::pipeline::run(input, nav, watcher_handle);
    }
    crate::shell::pipeline::reset_failed();
    let exit_shell = execute_single_command(input, nav, watcher_handle)?;
    let success = !crate::shell::pipeline::take_failed();
    Ok((exit_shell, success))
}

/// Execute a single command (already fully expanded, no operators).
///
/// Resolution order for a typed line: alias expansion (shell entry) →
/// tutorial guard → built-in registry → fallback (`@teleport`, math,
/// system command). Expansion runs first, so alias names are validated
/// against the registry (`validate_alias_name`) to prevent shadowing.
pub(crate) fn execute_single_command(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").trim();

    // Tutorial mode is a guided lab: only lesson commands run inside the
    // workspace, and leaving the folder blocks everything but the way back.
    if let Some(message) =
        crate::shell::commands::builtin::tutorial::tutorial::restriction(&cmd, nav)
    {
        print_error(&message);
        crate::shell::pipeline::mark_failed();
        return Ok(false);
    }
    crate::shell::commands::builtin::tutorial::tutorial::note_command(&cmd, args);

    // `time` owns a full help screen; handle its help forms before generic
    // contextual help prints a shell completion hint for the same suffix.
    if cmd == "time" && matches!(args, "help" | "--help" | "-h") {
        return builtin::time::cmd_time(args, nav);
    }

    // Cisco-style contextual `<prefix> --help` at any depth (concise
    // next-step hints, `<cr>` when complete). Runs before dispatch so
    // `ignore dir --help` never creates a dir named `--help`.
    // `-h` is only intercepted here when it is NOT a data flag for the
    // command; flag-heavy commands keep their own `-h` handling.
    {
        let trimmed = input.trim();
        if let Some(last) = trimmed.split_whitespace().last() {
            let last_lower = last.to_lowercase();
            if last_lower == "--help" || last_lower == "-h" {
                let is_long = last_lower == "--help";
                if let Some(prefix) = crate::shell::help::strip_help_suffix(input) {
                    let prefix_trimmed = prefix.trim();
                    let root_of_prefix = prefix_trimmed
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_lowercase();
                    let prefix_len = if prefix_trimmed.is_empty() {
                        0
                    } else {
                        prefix_trimmed.split_whitespace().count()
                    };
                    // Bare `<flag-heavy-cmd> --help/-h` keeps its detailed
                    // built-in help; deeper `<cmd> <sub> --help` uses concise.
                    let flag_heavy = registry_data::registry().owns_help(&root_of_prefix);
                    let keep_native = flag_heavy && prefix_len <= 1;
                    // Short `-h` for flag-heavy commands is data/flag: never intercept.
                    let short_blocked = !is_long && flag_heavy;
                    if !short_blocked && !keep_native {
                        if crate::shell::help::print_contextual_help(&prefix) {
                            return Ok(false);
                        }
                    }
                }
            }
        }
    }

    // Legacy sole-arg `<cmd> -h | --help` full-topic fallback (only when
    // contextual help did not handle it). Opt-in via the registry's
    // `help_topic`: commands where `-h` is data (gs search text, http
    // header flag, …) or that self-handle it (dl/up/serve/…) return None.
    if args == "-h" || args == "--help" {
        if let Some(topic) = registry_data::registry().help_topic(&cmd) {
            crate::shell::help::print_topic_help(topic);
            return Ok(false);
        }
    }

    // The editor's embedded terminal already runs an ntc shell; starting a
    // second one would stack shells on the same PTY.
    if crate::shell::entry::embedded_in_editor() && is_ntc_shell_invocation(&cmd) {
        print_error(&t!("misc.exit.no_nested_ntc"));
        return Ok(false);
    }

    if let Some(spec) = registry_data::registry().resolve(&cmd) {
        return spec.run.run(args, nav);
    }

    // A typed `*.ntc` file name runs the script, so a script is one word
    // away from the prompt (`deploy.ntc prod`) without the `scr` prefix.
    if crate::shell::commands::builtin::scr::looks_like_script(input, nav.current_path()) {
        return crate::shell::commands::builtin::scr::run_direct(input.trim(), nav);
    }

    cmd_fallback(input, nav, watcher_handle)
}

/// Fallback dispatch for unknown commands: teleport `@name`, math
/// expressions and system commands.
fn cmd_fallback(
    input: &str,
    nav: &mut Navigator,
    _watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();

    if cmd.starts_with('@') && cmd.len() > 1 {
        let tp_name = &cmd[1..];
        if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
            TeleportManager::jump_by_name(nav, tp_name)?;
            return Ok(false);
        }
    }

    // Try math expression evaluation for inputs that look like math
    let trimmed = input.trim();
    if crate::shell::commands::builtin::math::math::is_math_expression(trimmed) {
        if let Err(e) = crate::shell::commands::builtin::math::math::run(trimmed) {
            print_error(&format!("{}", e));
        }
        return Ok(false);
    }

    let cmd_part = input.trim();
    if !cmd_part.is_empty() {
        if let Err(e) = execute_system_command(cmd_part, nav.current_path()) {
            print_error(&format!("{}", e));
            return Ok(false);
        }
    }
    Ok(false)
}

/// True when `cmd` names the ntc binary itself (`ntc`, `ntc.exe`, or a
/// path ending in either), as opposed to a command that merely contains
/// the letters (`ntceditor`).
fn is_ntc_shell_invocation(cmd: &str) -> bool {
    let trimmed = cmd.trim().trim_matches('"').replace('\\', "/");
    let last = trimmed
        .rsplit('/')
        .next()
        .unwrap_or(&trimmed)
        .to_lowercase();
    last == "ntc" || last == "ntc.exe"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_registry_covers_safe_commands() {
        let registry = registry_data::registry();
        assert_eq!(registry.help_topic("mv"), Some("mv"));
        assert_eq!(registry.help_topic("move"), Some("mv"));
        assert_eq!(registry.help_topic("cp"), Some("cp"));
        assert_eq!(registry.help_topic("gs"), Some("gs"));
        assert_eq!(registry.help_topic("wi"), Some("wi"));
        assert_eq!(registry.help_topic("ne"), Some("ne"));
        assert_eq!(registry.help_topic("time"), Some("time"));
    }

    #[test]
    fn help_registry_skips_data_flag_commands() {
        let registry = registry_data::registry();
        // `-h` is data or self-handled here: must NOT be intercepted.
        assert_eq!(registry.help_topic("http"), None);
        assert_eq!(registry.help_topic("serve"), None);
        assert_eq!(registry.help_topic("dl"), None);
        assert_eq!(registry.help_topic("tmprt"), Some("tmprt"));
        assert_eq!(registry.help_topic("nope"), None);
    }

    #[test]
    fn plugin_namespace_is_reserved_but_never_loaded() {
        // 2.4.9 ships the seam empty: no plugin command is registered and
        // the process-wide plugin registry holds nothing.
        assert!(plugin::registry().is_empty());
        assert!(!registry_data::registry().contains("plugin"));
        // The namespace is reserved against alias shadowing in any case.
        assert!(!crate::config::validate_alias_name("plugin"));
        assert!(!crate::config::validate_alias_name("PLUGIN"));
    }

    #[test]
    fn unknown_and_plugin_commands_reach_fallback() {
        let registry = registry_data::registry();
        // Neither the reserved namespace nor an arbitrary unknown token
        // resolves to a built-in, so dispatch falls through to cmd_fallback.
        assert!(registry.resolve("plugin").is_none());
        assert!(registry.resolve("no_such_command_xyz").is_none());
        assert!(plugin::registry().is_empty());
        // The fallback is live: a math expression (not a built-in) is handled
        // without error and keeps the shell open.
        let mut nav = Navigator::for_test(std::path::Path::new("."));
        assert!(!cmd_fallback("2+2", &mut nav, &None).unwrap());
    }

    #[test]
    fn empty_plugin_registry_leaves_help_and_completion_unchanged() {
        assert!(plugin::registry().is_empty());
        let registry = registry_data::registry();
        // Help: the reserved namespace has no topic and is not a contextual
        // root; a real built-in's help is untouched.
        assert_eq!(registry.help_topic("plugin"), None);
        assert!(!registry.is_contextual("plugin"));
        assert_eq!(registry.help_topic("mv"), Some("mv"));
        // Completion: the registry offers nothing for the plugin namespace
        // and the root command set does not gain "plugin".
        assert!(registry.subcommands(&["plugin"], &[]).is_empty());
        crate::config::Config::with_test_config(crate::config::Config::new(), || {
            assert!(!crate::shell::hinter::build_command_set().contains("plugin"));
        });
    }
}
