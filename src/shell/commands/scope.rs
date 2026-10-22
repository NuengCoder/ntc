//! Config scope flag (`-g` / `--global`) parsing.
//!
//! Inside a directory that has an `ntconfig.toml`, the default target of a
//! write is the local file; the global config is only reachable through the
//! explicit `-g` / `--global` flag. This keeps a project-local command from
//! ever modifying the machine-wide config by accident.

use crate::config::ConfigScope;

/// Split a `-g` / `--global` scope flag off an argument string.
///
/// The flag may be the first or the last standalone token; the remainder is
/// returned unchanged. Quoted command bodies are safe — `"ls -g"` ends with a
/// quote, not with a standalone `-g` token.
pub(crate) fn split_scope_flag(args: &str) -> (ConfigScope, &str) {
    let trimmed = args.trim();

    if trimmed == "-g" || trimmed == "--global" {
        return (ConfigScope::Global, "");
    }

    for flag in ["-g", "--global"] {
        if let Some(rest) = trimmed.strip_prefix(flag) {
            if rest.is_empty() || rest.starts_with(char::is_whitespace) {
                return (ConfigScope::Global, rest.trim_start());
            }
        }
    }

    for flag in ["-g", "--global"] {
        let suffix = format!(" {}", flag);
        if trimmed.ends_with(&suffix) {
            // The space in front of the flag makes it a standalone token.
            let head = &trimmed[..trimmed.len() - suffix.len()];
            return (ConfigScope::Global, head.trim_end());
        }
    }

    (ConfigScope::Auto, trimmed)
}

/// Split a `-g` / `--global` scope flag off the *front* of an argument
/// string.
///
/// Used for `ral add <name> -g "<command>"`, where the flag follows the alias
/// name. Only an unquoted standalone token counts — `"-g"` (a quoted command)
/// is left alone.
pub(crate) fn split_scope_flag_prefix(args: &str) -> (ConfigScope, &str) {
    let trimmed = args.trim_start();
    for flag in ["-g", "--global"] {
        if let Some(rest) = trimmed.strip_prefix(flag) {
            if rest.starts_with(char::is_whitespace) {
                return (ConfigScope::Global, rest.trim_start());
            }
        }
    }
    (ConfigScope::Auto, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_flag_is_split_off() {
        let (scope, rest) = split_scope_flag("-g ll \"echo hi\"");
        assert_eq!(scope, ConfigScope::Global);
        assert_eq!(rest, "ll \"echo hi\"");
    }

    #[test]
    fn trailing_flag_is_split_off() {
        let (scope, rest) = split_scope_flag("ll \"echo hi\" -g");
        assert_eq!(scope, ConfigScope::Global);
        assert_eq!(rest, "ll \"echo hi\"");
    }

    #[test]
    fn quoted_flag_is_kept() {
        let (scope, rest) = split_scope_flag("ll \"ls -g\"");
        assert_eq!(scope, ConfigScope::Auto);
        assert_eq!(rest, "ll \"ls -g\"");
    }

    #[test]
    fn no_flag_keeps_args() {
        let (scope, rest) = split_scope_flag("ll \"echo hi\"");
        assert_eq!(scope, ConfigScope::Auto);
        assert_eq!(rest, "ll \"echo hi\"");
    }

    #[test]
    fn prefix_flag_is_split_off() {
        let (scope, rest) = split_scope_flag_prefix("-g \"echo hi\"");
        assert_eq!(scope, ConfigScope::Global);
        assert_eq!(rest, "\"echo hi\"");
    }

    #[test]
    fn prefix_flag_ignores_quoted_commands() {
        let (scope, rest) = split_scope_flag_prefix("\"-g foo\"");
        assert_eq!(scope, ConfigScope::Auto);
        assert_eq!(rest, "\"-g foo\"");
    }
}
