use std::collections::HashMap;

use super::spec::{CommandSource, CommandSpec, FlagPolicy};

/// Immutable command name/alias index built once from the static specs.
pub(crate) struct Registry {
    specs: Vec<&'static CommandSpec>,
    by_name: HashMap<&'static str, &'static CommandSpec>,
}

impl Registry {
    /// Build the index. A duplicate name or alias is a programming error in
    /// the static table, so it fails loudly instead of silently shadowing.
    pub(crate) fn new(specs: Vec<&'static CommandSpec>) -> Self {
        let mut by_name: HashMap<&'static str, &'static CommandSpec> = HashMap::new();
        for &spec in &specs {
            assert_eq!(
                spec.source,
                CommandSource::Builtin,
                "the built-in registry only accepts built-in specs: '{}'",
                spec.name
            );
            assert_eq!(
                spec.name,
                spec.name.to_ascii_lowercase(),
                "command names must be lowercase: '{}'",
                spec.name
            );
            for name in std::iter::once(spec.name).chain(spec.aliases.iter().copied()) {
                if let Some(previous) = by_name.insert(name, spec) {
                    panic!(
                        "duplicate command name or alias '{}' ({} and {})",
                        name, previous.name, spec.name
                    );
                }
            }
        }
        Self { specs, by_name }
    }

    pub(crate) fn specs(&self) -> &[&'static CommandSpec] {
        &self.specs
    }

    /// Resolve a canonical name or alias, case-insensitively.
    pub(crate) fn resolve(&self, name: &str) -> Option<&'static CommandSpec> {
        let name = name.to_ascii_lowercase();
        self.by_name.get(name.as_str()).copied()
    }

    pub(crate) fn contains(&self, name: &str) -> bool {
        self.resolve(name).is_some()
    }

    /// True when `name` would dispatch to a built-in command. Alias
    /// validation uses this so a run alias can never shadow a name the
    /// shell resolves, matching dispatch's case-insensitive lookup.
    pub(crate) fn is_reserved(&self, name: &str) -> bool {
        self.contains(name)
    }

    /// Every canonical name and alias, in spec order.
    pub(crate) fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.specs()
            .iter()
            .flat_map(|spec| std::iter::once(spec.name).chain(spec.aliases.iter().copied()))
    }

    pub(crate) fn help_topic(&self, name: &str) -> Option<&'static str> {
        self.resolve(name).and_then(|spec| spec.help_topic)
    }

    pub(crate) fn is_contextual(&self, name: &str) -> bool {
        let Some(spec) = self.resolve(name) else {
            return false;
        };
        if spec.name.eq_ignore_ascii_case(name) {
            return spec.contextual;
        }
        !spec
            .non_contextual_aliases
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(name))
            && spec.contextual
    }

    /// True when the command handles `-h`/`--help` itself.
    pub(crate) fn owns_help(&self, name: &str) -> bool {
        self.resolve(name)
            .is_some_and(|spec| spec.flags == FlagPolicy::CommandOwnsHelp)
    }

    /// Static subcommand/flag completions for a typed prefix. The caller
    /// supplies config-dependent scope flags so the core stays pure.
    pub(crate) fn subcommands(
        &self,
        parts: &[&str],
        scope_flags: &[&'static str],
    ) -> Vec<&'static str> {
        let Some(spec) = parts.first().and_then(|root| self.resolve(root)) else {
            return vec![];
        };
        super::completion::lookup(spec.complete, parts, scope_flags)
    }
}
