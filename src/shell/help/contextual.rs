//! Cisco-style contextual help for `<prefix> --help`.
//!
//! Command-specific next-step handlers live in the owning command module
//! (`commands::builtin::<cmd>::help`); this module keeps the generic engine
//! used when a root has no special case.

use super::print_cr;

/// Strip a trailing contextual-help token (`--help` / `-h`).
/// Returns the prefix before it, or None when no help token is present.
pub(crate) fn strip_help_suffix(input: &str) -> Option<String> {
    crate::shell::hinter::strip_help_suffix(input)
}

/// Cisco-style contextual help for `<prefix> --help`.
/// Returns true when the prefix is a known command and help was printed.
pub(crate) fn print_contextual_help(prefix_line: &str) -> bool {
    let trimmed = prefix_line.trim();
    if trimmed.is_empty() {
        return false;
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return false;
    }
    let lowers: Vec<String> = parts.iter().map(|s| s.to_lowercase()).collect();
    let root = lowers[0].as_str();

    match crate::shell::commands::core::help::contextual_for(root) {
        Some(render) => render(&parts, &lowers),
        None => print_generic_help(&parts, &lowers, root),
    }
}

fn print_generic_help(parts: &[&str], lowers: &[String], _root: &str) -> bool {
    use crate::shell::hinter::subcommands_for;
    let root = lowers[0].as_str();

    // Known value-complete patterns: value already supplied -> <cr>.
    if is_generic_complete(parts, lowers) {
        print_cr();
        return true;
    }

    // Next-step candidates (named only; fs paths become <path> templates).
    let mut next = named_next_sanitized(parts);
    if !next.is_empty() {
        next.sort();
        next.dedup();
        for n in next {
            outln!("{}", n);
        }
        return true;
    }

    if parts.len() == 1 {
        // Bare command with no further subcommands (e.g. `where`, `back`,
        // `gos`): complete, Enter runs it. Unknown roots fall through to
        // the caller (returns false) so system fallback still works.
        if crate::shell::commands::registry_data::registry().is_contextual(root) {
            print_cr();
            return true;
        }
        return false;
    }

    // No next candidates at depth >= 2.
    // If the last token is a known subcommand of its parent, a value is
    // still missing -> show `prefix <value>`; otherwise a value was already
    // supplied -> <cr>.
    let parent = &parts[..parts.len() - 1];
    let parent_subs: Vec<String> = subcommands_for(parent)
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();
    let last_lower = parts[parts.len() - 1].to_lowercase();
    if parent_subs.contains(&last_lower) {
        outln!("{} <value>", parts.join(" "));
        return true;
    }
    // If parent itself expected an enumerable value (alias/tp/theme/ran…),
    // the current token is that value -> complete.
    print_cr();
    true
}

/// Next candidates for help printing: named sources + subcommands, but
/// suppress repeated top-level subcommand lists at depth >= 2 (e.g. `tp
/// jump --help` must not re-list `add/jump/to…`).
fn named_next_sanitized(parts: &[&str]) -> Vec<String> {
    use crate::shell::hinter::{
        alias_names_for_command, ntpkg_names_for_command, ran_target_names_for_command,
        scr_scripts_for_command, subcommands_for, teleport_names_for_command,
        theme_names_for_command,
    };
    if parts.is_empty() {
        return vec![];
    }
    // `cd to` is handled by its own printer; keep generic free of cd noise.
    if parts.len() >= 2 && parts[0].to_lowercase() == "cd" {
        return vec![];
    }
    let mut out: Vec<String> = Vec::new();
    out.extend(teleport_names_for_command(parts));
    out.extend(alias_names_for_command(parts));
    out.extend(ran_target_names_for_command(parts));
    out.extend(scr_scripts_for_command(parts));
    out.extend(ntpkg_names_for_command(parts));
    out.extend(theme_names_for_command(parts));
    if parts.len() == 1 && matches!(parts[0].to_lowercase().as_str(), "godrive" | "god") {
        out.extend(
            crate::navigator::Navigator::drive_roots()
                .into_iter()
                .map(|r| r.label)
                .collect::<Vec<_>>(),
        );
    }
    let cur = subcommands_for(parts);
    if parts.len() == 1 {
        out.extend(cur.into_iter().map(|s| s.to_string()));
    } else {
        // Keep deeper valid drill-downs (e.g. `ral export`, `config watch`,
        // `tl ext`), skip pure repetitions of the top-level list.
        let top = subcommands_for(&[parts[0]]);
        if cur != top {
            out.extend(cur.into_iter().map(|s| s.to_string()));
        }
    }
    out.sort();
    out.dedup();
    out
}

/// True when `parts` already contains a user value for a value-taking
/// subcommand, i.e. help should show `<cr>` instead of listing again.
fn is_generic_complete(parts: &[&str], lowers: &[String]) -> bool {
    if parts.len() < 3 {
        return false;
    }
    let root = lowers[0].as_str();
    let sub = lowers[1].as_str();
    match root {
        "ral" => match sub {
            "rm" | "remove" | "info" | "edit" => parts.len() >= 3,
            "rnm" | "rename" => parts.len() >= 5,
            _ => false,
        },
        "theme" => match sub {
            "info" | "edit" | "rm" | "remove" | "rnm" | "rename" | "export" => parts.len() >= 3,
            _ => false,
        },
        "tpb" => parts.len() >= 2,
        "ran" | "scr" | "script" => {
            // `ran init` etc. are subcommands; anything else with a value
            // is a target/script -> complete.
            !matches!(sub, "init" | "deinit" | "list" | "ls" | "help")
        }
        _ => false,
    }
}
