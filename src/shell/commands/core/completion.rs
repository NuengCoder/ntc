//! Static completion trees owned by each command spec.
//!
//! `CompletionSpec` covers subcommands and flags; dynamic values (teleport
//! names, themes, aliases, …) stay in the shell's hinter providers. The
//! lookup is pure: config-dependent scope flags are passed in by the caller.

/// A command's static completion data.
#[derive(Clone, Copy)]
pub(crate) struct CompletionSpec {
    /// Tokens offered right after the command root.
    pub root: &'static [&'static str],
    /// Offer `root` at every depth (flat subcommand lists like `tp`).
    pub flat: bool,
    /// Deeper nodes; the longest matching path wins.
    pub nodes: &'static [CompletionNode],
}

/// Commands without subcommands or flags share this empty spec.
pub(crate) const EMPTY: CompletionSpec = CompletionSpec {
    root: &[],
    flat: false,
    nodes: &[],
};

/// One node of a command's completion tree.
#[derive(Clone, Copy)]
pub(crate) struct CompletionNode {
    /// Path below the root; each element lists the names that match it.
    /// `"*"` matches any token. An empty path matches any prefix deeper
    /// than the root.
    pub path: &'static [&'static [&'static str]],
    /// Match path elements case-sensitively (a few historical arms do).
    pub case_sensitive: bool,
    /// Tokens when the prefix ends exactly at this path.
    pub tokens: &'static [&'static str],
    /// Tokens one level past the path; `None` repeats `tokens`.
    pub deeper: Option<&'static [&'static str]>,
    /// Tokens two or more levels past the path; `None` repeats `deeper`.
    pub beyond: Option<&'static [&'static str]>,
    /// Last-token value lists, applied strictly past the path.
    pub value_slots: &'static [ValueSlot],
    /// Last tokens that suppress all completions (value-taking flags).
    pub suppress_after: &'static [&'static str],
    /// Append the caller-supplied scope flags (`-g`/`--global`).
    pub scope_flags: bool,
}

impl CompletionNode {
    /// Case-insensitive node with the same tokens at every depth.
    pub(crate) const fn at(
        path: &'static [&'static [&'static str]],
        tokens: &'static [&'static str],
    ) -> Self {
        Self {
            path,
            case_sensitive: false,
            tokens,
            deeper: None,
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: false,
        }
    }

    /// Like [`Self::at`], but path elements match case-sensitively.
    pub(crate) const fn at_cs(
        path: &'static [&'static [&'static str]],
        tokens: &'static [&'static str],
    ) -> Self {
        Self {
            path,
            case_sensitive: true,
            tokens,
            deeper: None,
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: false,
        }
    }

    /// Node offering only the config-dependent scope flags.
    pub(crate) const fn scoped(path: &'static [&'static [&'static str]]) -> Self {
        Self {
            path,
            case_sensitive: false,
            tokens: &[],
            deeper: None,
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: true,
        }
    }

    /// Like [`Self::scoped`], but path elements match case-sensitively.
    pub(crate) const fn scoped_cs(path: &'static [&'static [&'static str]]) -> Self {
        Self {
            path,
            case_sensitive: true,
            tokens: &[],
            deeper: None,
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: true,
        }
    }
}

/// A last-token value list: after one of `after`, offer `values`.
#[derive(Clone, Copy)]
pub(crate) struct ValueSlot {
    pub after: &'static [&'static str],
    pub values: &'static [&'static str],
}

/// Resolve the static completions for `parts` (root + typed tokens).
/// `scope_flags` is appended for nodes that ask for it.
pub(crate) fn lookup(
    spec: &CompletionSpec,
    parts: &[&str],
    scope_flags: &[&'static str],
) -> Vec<&'static str> {
    if parts.is_empty() {
        return vec![];
    }
    if parts.len() == 1 || spec.flat {
        return spec.root.to_vec();
    }
    let rest = &parts[1..];
    let mut best: Option<(&CompletionNode, usize)> = None;
    for node in spec.nodes {
        if node.path.len() > rest.len() {
            continue;
        }
        let matches = node.path.iter().zip(rest).all(|(names, part)| {
            names.iter().any(|name| {
                if *name == "*" {
                    true
                } else if node.case_sensitive {
                    name == part
                } else {
                    name.eq_ignore_ascii_case(part)
                }
            })
        });
        if matches && best.is_none_or(|(_, len)| node.path.len() > len) {
            best = Some((node, node.path.len()));
        }
    }
    let Some((node, path_len)) = best else {
        return vec![];
    };
    let depth = rest.len();
    let mut out: Vec<&'static str> = if depth == path_len {
        node.tokens.to_vec()
    } else if depth == path_len + 1 {
        node.deeper.unwrap_or(node.tokens).to_vec()
    } else {
        node.beyond.or(node.deeper).unwrap_or(node.tokens).to_vec()
    };
    if depth > path_len {
        let last = parts[parts.len() - 1].to_lowercase();
        for slot in node.value_slots {
            if slot.after.iter().any(|a| a.eq_ignore_ascii_case(&last)) {
                return slot.values.to_vec();
            }
        }
        if node
            .suppress_after
            .iter()
            .any(|s| s.eq_ignore_ascii_case(&last))
        {
            return vec![];
        }
    }
    if node.scope_flags {
        out.extend(scope_flags.iter().copied());
    }
    out
}
