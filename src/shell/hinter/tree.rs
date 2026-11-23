//! Command-tree completion driven by the registry: the root command set,
//! static subcommands/flags, and pipeline/redirect/heredoc slot completion.

use super::files::list_path_candidates;
use super::filter_prefix;
use crate::config::Config;
use std::collections::HashSet;

/// True when `name` is one of the shell's built-in commands.
pub(crate) fn is_builtin_command(name: &str) -> bool {
    crate::shell::commands::registry_data::registry().contains(name)
}

/// Returns possible subcommand completions for a command path.
/// `parts` is the prefix path (the words before the one being typed).
///   - If `parts.len() == 1` (e.g. `["ral"]`), returns the subcommands of `ral`.
///   - If `parts.len() >= 2` (e.g. `["ral", "export"]`), drills deeper.
/// `-g` / `--global` hints.
///
/// Only offered inside a directory that has an `ntconfig.toml`: there the
/// default target of a write is the local file, so the flag is the explicit
/// way to reach the global config. Outside such a directory the default is
/// already the global config and the flag would be noise.
pub(crate) fn global_scope_flags() -> Vec<&'static str> {
    if Config::get_local_config_path().is_some() {
        vec!["-g", "--global"]
    } else {
        vec![]
    }
}

pub(crate) fn subcommands_for(parts: &[&str]) -> Vec<&'static str> {
    crate::shell::commands::registry_data::registry().subcommands(parts, &global_scope_flags())
}

/// Commands offered at the start of a piped stage (`cmd | `), in ghost
/// order. Only these are completed after a pipe — no other command falls
/// back to generic completion.
pub(crate) const PIPE_FILTER_COMMANDS: &[&str] = &[
    "head", "tail", "betw", "gs", "sort", "rev", "count", "trim", "line", "encode", "qur",
    "clip",
];

/// Pipe-stage command completion. `Some` means the stage is still in its
/// command slot (or names an unsupported command), so the caller must not
/// fall back to generic completion. `None` means one of the pipe filters
/// is already present and its own flags/files should complete normally.
pub(crate) fn pipe_stage_commands(tail: &str) -> Option<Vec<String>> {
    let trimmed = tail.trim();
    if trimmed.is_empty() {
        return Some(PIPE_FILTER_COMMANDS.iter().map(|s| s.to_string()).collect());
    }
    let first = trimmed.split_whitespace().next().unwrap_or("");
    let command_given =
        tail.ends_with(' ') || tail.ends_with('\t') || trimmed.split_whitespace().count() > 1;
    if !command_given {
        // Still typing the command name: prefix-match in declared order.
        return Some(filter_prefix(
            PIPE_FILTER_COMMANDS.iter().map(|s| s.to_string()).collect(),
            first,
        ));
    }
    if PIPE_FILTER_COMMANDS.contains(&first.to_lowercase().as_str()) {
        None
    } else {
        Some(vec![])
    }
}

/// Completion for `>>` (append) and `<` (input) targets. Readable files
/// only — binary, image, audio, video and archive files are hidden — plus
/// directories so `sub/<Tab>` traversal keeps working. Nothing once the
/// single target token is already present.
pub(crate) fn redirect_target_candidates(tail: &str) -> Vec<String> {
    let trimmed = tail.trim();
    let target_given = !trimmed.is_empty()
        && (tail.ends_with(' ') || tail.ends_with('\t') || trimmed.split_whitespace().count() > 1);
    if target_given {
        return vec![];
    }
    let (dir_part, file_part) = match trimmed.rfind('/') {
        Some(idx) => (&trimmed[..idx], &trimmed[idx + 1..]),
        None => ("", trimmed),
    };
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let base = if dir_part.is_empty() {
        cwd
    } else {
        let p = cwd.join(dir_part);
        if !p.is_dir() {
            return vec![];
        }
        p
    };
    list_path_candidates(&base, dir_part, file_part, false, false, None, true)
}

/// Completion for the `<<` heredoc delimiter: `EOF` only.
pub(crate) fn heredoc_delim_candidates(tail: &str) -> Vec<String> {
    let trimmed = tail.trim();
    if trimmed.is_empty() {
        return vec!["EOF".to_string()];
    }
    if tail.ends_with(' ') || tail.ends_with('\t') || trimmed.split_whitespace().count() > 1 {
        return vec![];
    }
    filter_prefix(vec!["EOF".to_string()], trimmed)
}

pub(crate) fn build_command_set() -> HashSet<String> {
    let mut cmds: HashSet<String> = crate::shell::commands::registry_data::registry()
        .names()
        .map(str::to_string)
        .collect();
    let aliases = Config::get_run_aliases_merged();
    for name in aliases.keys() {
        cmds.insert(name.clone());
    }
    // Curated, installed-only names join level-1 completion:
    // `config integration_shell on` adds foreign shells, `config code_tools
    // on` adds programming tools/compilers. Arbitrary PATH executables never
    // appear.
    if crate::shell::ext_complete::integration_enabled() {
        for name in crate::shell::ext_complete::shell_names() {
            cmds.insert(name);
        }
        // brew ships with the macOS zsh integration.
        if let Some(name) = crate::shell::ext_complete::package_manager_name() {
            cmds.insert(name.to_string());
        }
    }
    if crate::shell::ext_complete::code_tools_enabled() {
        for name in crate::shell::ext_complete::code_tool_names() {
            cmds.insert(name);
        }
    }
    cmds
}
