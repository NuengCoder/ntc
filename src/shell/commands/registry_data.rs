//! The process-wide command registry.
//!
//! `BUILTIN_SPECS` in [`super::builtin`] is the only command table: each
//! command module owns its own `SPEC`, so adding a command means adding a
//! module and one line to that list. Nothing here enumerates command names.

use super::core::registry::Registry;
use std::sync::OnceLock;

static REGISTRY: OnceLock<Registry> = OnceLock::new();

/// The process-wide command registry, built once from the built-in specs.
pub(crate) fn registry() -> &'static Registry {
    REGISTRY.get_or_init(|| Registry::new(super::builtin::BUILTIN_SPECS.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn registry_has_unique_names_and_aliases() {
        let registry = registry();
        let mut seen = HashSet::new();
        for name in registry.names() {
            assert!(seen.insert(name), "duplicate command name or alias: {name}");
            assert_eq!(name, &name.to_ascii_lowercase());
        }
        assert!(registry.specs().len() >= 100);
    }

    #[test]
    fn every_alias_resolves_to_its_canonical_spec() {
        let registry = registry();
        for spec in registry.specs() {
            assert_eq!(registry.resolve(spec.name).unwrap().name, spec.name);
            for alias in spec.aliases {
                assert_eq!(registry.resolve(alias).unwrap().name, spec.name);
            }
        }
    }

    #[test]
    fn completion_only_drift_is_not_registered() {
        let registry = registry();
        assert!(!registry.contains("ls"), "ls never had a dispatch arm");
        assert!(registry.contains("ui"), "ui is a real command");
    }

    #[test]
    fn config_and_its_alias_keep_distinct_contextual_help() {
        let registry = registry();
        assert!(registry.is_contextual("config"));
        assert!(!registry.is_contextual("cfg"));
    }

    #[test]
    fn native_help_policy_matches_the_old_flag_heavy_set() {
        let registry = registry();
        for name in [
            "http",
            "dl",
            "up",
            "ping",
            "trace",
            "tracert",
            "traceroute",
            "dns",
            "smtp",
            "serve",
        ] {
            assert!(registry.owns_help(name), "{name} should own -h");
        }
        for name in ["cd", "view", "config", "help", "image"] {
            assert!(!registry.owns_help(name), "{name} should use help topic");
        }
    }

    #[test]
    fn plain_contextual_commands_match_the_old_known_root_gaps() {
        let registry = registry();
        for name in ["head", "tail", "betw", "sort", "rev", "ui"] {
            assert!(!registry.is_contextual(name), "{name} was not a known root");
        }
        for name in ["cd", "view", "config", "where", "git"] {
            assert!(registry.is_contextual(name), "{name} was a known root");
        }
    }

    #[test]
    fn every_spec_is_builtin() {
        use crate::shell::commands::core::spec::CommandSource;
        let registry = registry();
        for spec in registry.specs() {
            assert_eq!(
                spec.source,
                CommandSource::Builtin,
                "'{}' must be a built-in in 2.4.9",
                spec.name
            );
        }
        // The plugin seam ships empty and registers no command: the
        // reserved namespace is not a built-in.
        assert!(crate::shell::commands::plugin::registry().is_empty());
        assert!(!registry.contains("plugin"));
    }
}
