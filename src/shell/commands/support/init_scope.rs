//! Shared parsing for `init` / `deinit` project scopes.

/// Which parts of a project a top-level `init`/`deinit` should touch.
#[derive(Default)]
pub(crate) struct Scope {
    pub(crate) ran: bool,
    pub(crate) local: bool,
    pub(crate) opencode: bool,
    pub(crate) force: bool,
}

/// Parse `ran|local|opencode` subcommands (plus `-f`/`--force`).
///
/// No subcommand means `ran local`, matching the historical default. `all` is
/// rejected by the command with its own message before this runs.
pub(crate) fn parse_scope(args: &str) -> Option<Scope> {
    let mut scope = Scope::default();
    let mut any = false;
    for token in args.split_whitespace() {
        match token.to_ascii_lowercase().as_str() {
            "-f" | "--force" => scope.force = true,
            "ran" => {
                scope.ran = true;
                any = true;
            }
            "local" => {
                scope.local = true;
                any = true;
            }
            "opencode" | "oc" => {
                scope.opencode = true;
                any = true;
            }
            _ => return None,
        }
    }
    if !any {
        scope.ran = true;
        scope.local = true;
    }
    Some(scope)
}

/// Parse `ran|local|opencode` for `deinit`.
///
/// `all`, `project` and `-f`/`--force` are rejected: a removal cannot be
/// rolled back, so every removal has to be confirmed at the prompt.
pub(crate) fn parse_deinit_scope(args: &str) -> Option<Scope> {
    let mut scope = Scope::default();
    let mut any = false;
    for token in args.split_whitespace() {
        match token.to_ascii_lowercase().as_str() {
            "ran" => {
                scope.ran = true;
                any = true;
            }
            "local" => {
                scope.local = true;
                any = true;
            }
            "opencode" | "oc" => {
                scope.opencode = true;
                any = true;
            }
            _ => return None,
        }
    }
    if !any {
        scope.ran = true;
        scope.local = true;
    }
    Some(scope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_scope_accepts_positional_subcommands() {
        // `init` handles `all` itself (with a dedicated message); the shared
        // parser only knows the individual parts.
        assert!(parse_scope("all -f").is_none());

        let empty = parse_scope("").unwrap();
        assert!(empty.ran && empty.local && !empty.opencode && !empty.force);

        let ran = parse_scope("ran").unwrap();
        assert!(ran.ran && !ran.local && !ran.opencode);

        assert!(parse_scope("--all").is_none());
        assert!(parse_scope("nope").is_none());
    }

    #[test]
    fn parse_deinit_scope_rejects_all_project_and_force() {
        let empty = parse_deinit_scope("").unwrap();
        assert!(empty.ran && empty.local && !empty.opencode);

        let ran = parse_deinit_scope("ran").unwrap();
        assert!(ran.ran && !ran.local && !ran.opencode);

        let oc = parse_deinit_scope("opencode").unwrap();
        assert!(oc.opencode && !oc.ran && !oc.local);

        assert!(parse_deinit_scope("all").is_none());
        assert!(parse_deinit_scope("project").is_none());
        assert!(parse_deinit_scope("ran -f").is_none());
        assert!(parse_deinit_scope("--force").is_none());
    }
}
