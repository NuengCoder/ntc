//! Help-section metadata and lookups.
//!
//! Command modules own their help text; this module only holds the shared
//! table types and the lookup helpers used by `shell::help` and the registry.

/// One `help <topic>` section, rendered by the command module that owns it.
#[derive(Clone, Copy)]
pub(crate) struct HelpTopic {
    /// Canonical topic name (`help nav`).
    pub topic: &'static str,
    /// Extra names accepted by `help <name>` (`help cd`, `help cfg`, …).
    pub aliases: &'static [&'static str],
    /// Renderer owned by the command module.
    pub render: fn(),
}

/// Contextual-help renderer: prefix tokens plus their lowercased forms.
pub(crate) type ContextualRender = fn(&[&str], &[String]) -> bool;

/// One contextual-help special case, keyed by command root.
#[derive(Clone, Copy)]
pub(crate) struct ContextualHelp {
    /// Command root the handler answers for (`cd`, `tp`, …).
    pub root: &'static str,
    /// Prints the next-step help for a prefix beginning with `root`.
    pub render: ContextualRender,
}

/// One line of the compact index: `(topic, label, description key)`.
pub(crate) type CompactEntry = (&'static str, &'static str, &'static str);

fn topics() -> &'static [HelpTopic] {
    super::super::builtin::HELP_TOPICS
}

fn order() -> &'static [&'static str] {
    super::super::builtin::HELP_ORDER
}

fn compact() -> &'static [CompactEntry] {
    super::super::builtin::COMPACT_HELP
}

fn contextual() -> &'static [ContextualHelp] {
    super::super::builtin::CONTEXTUAL_HELP
}

/// Find a topic by canonical name or alias, case-insensitively.
pub(crate) fn find_topic(name: &str) -> Option<&'static HelpTopic> {
    let name = name.trim();
    topics().iter().find(|entry| {
        entry.topic.eq_ignore_ascii_case(name)
            || entry
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(name))
    })
}

/// Topics in full-reference order (`help --all`).
pub(crate) fn topics_in_order() -> impl Iterator<Item = &'static HelpTopic> {
    order()
        .iter()
        .filter_map(|name| topics().iter().find(|entry| entry.topic == *name))
}

/// Compact index entries in print order.
pub(crate) fn compact_entries() -> impl Iterator<Item = CompactEntry> {
    compact().iter().copied()
}

/// Contextual-help handler for a command root, when the command has one.
pub(crate) fn contextual_for(root: &str) -> Option<ContextualRender> {
    contextual()
        .iter()
        .find(|entry| entry.root.eq_ignore_ascii_case(root))
        .map(|entry| entry.render)
}
