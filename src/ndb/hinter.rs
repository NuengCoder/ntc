//! Ghost-text hints and Tab cycling for the `ndb` shell.
//!
//! Mirrors `crate::shell::hinter`: completions are shown as ghost text and
//! `Tab` / `Shift-Tab` cycle through the candidates (`Right-arrow` accepts).
//! Because `ndb` has two levels, the candidate set follows the current state
//! via a shared [`NdbContext`]:
//!
//! * `ndb [<cwd>]>`     - root commands; `interface <TAB>` completes `.db` files
//! * `ndb [<file.db>]>` - database commands; `tablef`/`schema <TAB>` completes
//!   table names

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Cmd, ConditionalEventHandler, Context, Event, EventContext, RepeatCount};

use crate::config::Config;

/// Commands available at `ndb [<cwd>]>` (no database open).
pub(crate) const ROOT_COMMANDS: &[&str] = &[
    "help", "h", "view", "ls", "list", "interface", "use", "open", "in", "i", "sql", "query",
    "dbal", "clear", "cls", "exit", "quit", "q",
];

/// Commands available at `ndb [<file.db>]>` (database open).
pub(crate) const DB_COMMANDS: &[&str] = &[
    "help", "h", "viewf", "vf", "view", "v", "tables", "t", "ls", "list", "tablef", "tblf", "tf",
    "schema", "desc", "describe", "sql", "query", "edit", "write", "backup", "bak", "more",
    "next", "m", "indexes", "idx", "info", "dbal", "interface", "use", "open", "in", "i",
    "clear", "cls", "exit", "quit", "q",
];

/// `dbal` subcommands (mirrors `dbal::SUBCOMMANDS` without help aliases).
const DBAL_SUBCOMMANDS: &[&str] = &[
    "add", "edit", "rnm", "rename", "rm", "remove", "info", "list", "ls", "run", "cls", "clear",
];

/// `dbal` subcommands taking an existing alias name.
const DBAL_ALIAS_ARG_SUBS: &[&str] = &["rm", "remove", "info", "edit", "run"];

/// Commands whose first argument is a table name.
const TABLE_ARG_COMMANDS: &[&str] = &[
    "tablef", "tblf", "tf", "schema", "desc", "describe", "indexes", "idx",
];

/// SQL keywords after which a table name is expected.
const SQL_TABLE_KEYWORDS: &[&str] = &["from", "join"];

/// Commands whose first argument is a database file.
const OPEN_COMMANDS: &[&str] = &["interface", "use", "open", "in", "i"];

/// Identifiers that ask for help - ghost text is suppressed for them.
fn is_help_token(tok: &str) -> bool {
    matches!(tok.to_lowercase().as_str(), "--help" | "-h")
}

/// Mutable shell state the hinter reads on every keystroke.
#[derive(Debug)]
pub(crate) struct NdbContext {
    in_db: AtomicBool,
    tables: RwLock<Vec<String>>,
    db_files: RwLock<Vec<String>>,
    aliases: RwLock<Vec<String>>,
}

impl Default for NdbContext {
    fn default() -> Self {
        Self {
            in_db: AtomicBool::new(false),
            tables: RwLock::new(Vec::new()),
            db_files: RwLock::new(Vec::new()),
            aliases: RwLock::new(Vec::new()),
        }
    }
}

impl NdbContext {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Whether a database is currently open (`ndb [<file.db>]>`).
    pub(crate) fn in_db(&self) -> bool {
        self.in_db.load(Ordering::Relaxed)
    }

    pub(crate) fn set_in_db(&self, value: bool) {
        self.in_db.store(value, Ordering::Relaxed);
    }

    /// Table/view names of the open database (empty when none is open).
    pub(crate) fn set_tables(&self, tables: Vec<String>) {
        if let Ok(mut guard) = self.tables.write() {
            *guard = tables;
        }
    }

    /// `.db` file names found in the working directory.
    pub(crate) fn set_db_files(&self, files: Vec<String>) {
        if let Ok(mut guard) = self.db_files.write() {
            *guard = files;
        }
    }

    /// Saved `dbal` alias names.
    pub(crate) fn set_aliases(&self, aliases: Vec<String>) {
        if let Ok(mut guard) = self.aliases.write() {
            *guard = aliases;
        }
    }
}
/// Tab-cycle cursor: remembers the line the ghost was computed for and which
/// candidate index is currently shown. Shared between the hinter (which reads
/// it) and the Tab key handler (which advances it).
#[derive(Debug, Default)]
pub(crate) struct CycleState {
    line: String,
    pos: usize,
    index: isize,
}

/// A rustyline `Hinter` for the `ndb` shell.
pub(crate) struct NdbHinter {
    ctx: Arc<NdbContext>,
    cycle: Arc<Mutex<CycleState>>,
}

impl NdbHinter {
    pub(crate) fn new(ctx: Arc<NdbContext>) -> Self {
        NdbHinter {
            ctx,
            cycle: Arc::new(Mutex::new(CycleState::default())),
        }
    }

    /// Handle for the Tab key handler so it can advance the same cycle state.
    pub(crate) fn tab_handler(&self, delta: isize) -> NdbTabCycleHandler {
        NdbTabCycleHandler {
            cycle: Arc::clone(&self.cycle),
            delta,
        }
    }

    /// Command words valid at the current level.
    fn commands(&self) -> Vec<String> {
        let list = if self.ctx.in_db() {
            DB_COMMANDS
        } else {
            ROOT_COMMANDS
        };
        list.iter().map(|s| s.to_string()).collect()
    }

    fn tables(&self) -> Vec<String> {
        self.ctx
            .tables
            .read()
            .map(|t| t.clone())
            .unwrap_or_default()
    }

    fn db_files(&self) -> Vec<String> {
        self.ctx
            .db_files
            .read()
            .map(|f| f.clone())
            .unwrap_or_default()
    }

    /// Saved `dbal` alias names, excluding any that shadow a builtin
    /// command (builtins always win at runtime).
    fn aliases(&self) -> Vec<String> {
        let known: Vec<String> = self.commands();
        self.ctx
            .aliases
            .read()
            .map(|list| {
                list.iter()
                    .filter(|a| !known.iter().any(|c| c.eq_ignore_ascii_case(a)))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Full candidate words for the word being typed in `typed`
    /// (cursor assumed at end of `typed`).
    pub(crate) fn candidates_for(&self, typed: &str) -> Vec<String> {
        if typed.is_empty() {
            return vec![];
        }
        let trailing_space = typed.ends_with(' ') || typed.ends_with('\t');
        let parts: Vec<&str> = typed.split_whitespace().collect();
        if parts.is_empty() {
            return vec![];
        }
        // `--help` / `-h` switches to contextual help on Enter: stay quiet.
        if parts.iter().any(|p| is_help_token(p)) {
            return vec![];
        }

        // -- Level 1: the command word itself --
        // Bare `dbal` alias names execute, so they complete here too.
        if parts.len() == 1 && !trailing_space {
            let word = parts[0];
            if word.starts_with('-') {
                return filter_prefix(vec!["--help".to_string(), "-h".to_string()], word);
            }
            let mut out = filter_prefix(self.commands(), word);
            out.extend(filter_prefix(self.aliases(), word));
            return out;
        }

        // -- Argument completion --
        let cmd = parts[0].to_lowercase();

        // Inside `dbal`: subcommands first, then alias names for the
        // subcommands that take one.
        if cmd == "dbal" {
            // Completing the subcommand itself (`dbal ` / `dbal r`).
            if parts.len() == 1 || (parts.len() == 2 && !trailing_space) {
                let frag = if parts.len() == 2 { parts[1] } else { "" };
                return filter_prefix(
                    DBAL_SUBCOMMANDS.iter().map(|s| s.to_string()).collect(),
                    frag,
                );
            }
            // Completing the alias name after a subcommand that takes one
            // (`dbal run ` / `dbal run top`).
            let (sub, frag) = if trailing_space {
                (parts.last().copied().unwrap_or(""), "")
            } else if parts.len() >= 3 {
                (
                    parts[parts.len() - 2],
                    parts.last().copied().unwrap_or(""),
                )
            } else {
                ("", "")
            };
            if DBAL_ALIAS_ARG_SUBS.contains(&sub.to_lowercase().as_str()) {
                return filter_prefix(self.aliases(), frag);
            }
            return vec![];
        }

        // Inside `sql`, complete table names right after FROM/JOIN. A query
        // is free text, so this branch runs before the first-argument gate.
        if (cmd == "sql" || cmd == "query") && self.ctx.in_db() {
            let (prev, frag) = if trailing_space {
                (parts.last().copied().unwrap_or(""), "")
            } else if parts.len() >= 3 {
                (
                    parts[parts.len() - 2],
                    parts.last().copied().unwrap_or(""),
                )
            } else {
                ("", "")
            };
            let prev = prev.trim_end_matches([';', ',']).to_lowercase();
            if SQL_TABLE_KEYWORDS.contains(&prev.as_str()) {
                return filter_prefix(self.tables(), frag);
            }
            return vec![];
        }

        // Every other command only completes its first argument.
        let first_arg = if trailing_space {
            parts.len() == 1
        } else {
            parts.len() == 2
        };
        if !first_arg {
            return vec![];
        }
        let frag = if trailing_space {
            ""
        } else {
            parts.last().copied().unwrap_or("")
        };

        if TABLE_ARG_COMMANDS.contains(&cmd.as_str()) && self.ctx.in_db() {
            return filter_prefix(self.tables(), frag);
        }

        if OPEN_COMMANDS.contains(&cmd.as_str()) {
            let mut names = self.db_files();
            // List numbers are accepted by `interface` too.
            names.extend((1..=names.len()).map(|i| i.to_string()));
            return filter_prefix(names, frag);
        }

        vec![]
    }
}

impl Hinter for NdbHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if !Config::global_get_autosuggest_enabled() {
            return None;
        }
        let mut pos = pos.min(line.len());
        while pos > 0 && !line.is_char_boundary(pos) {
            pos -= 1;
        }
        let typed = &line[..pos];
        if typed.is_empty() {
            return None;
        }

        let cands = self.candidates_for(typed);
        if cands.is_empty() {
            return None;
        }

        // Show the currently-cycled candidate as ghost text. Typing resets to
        // the first candidate; Tab advances (see NdbTabCycleHandler).
        let pick: String = {
            let mut cyc = match self.cycle.lock() {
                Ok(c) => c,
                Err(_) => return None,
            };
            if cyc.line != typed || cyc.pos != pos {
                cyc.line = typed.to_string();
                cyc.pos = pos;
                cyc.index = 0;
            }
            cands[(cyc.index.rem_euclid(cands.len() as isize)) as usize].clone()
        };

        // The ghost is the remainder after the word already typed.
        let frag: &str = if typed.ends_with(' ') || typed.ends_with('\t') {
            ""
        } else {
            typed.split_whitespace().last().unwrap_or("")
        };
        match pick.get(frag.len()..) {
            Some(rest) if !rest.is_empty() => Some(rest.to_string()),
            _ => None,
        }
    }
}

/// Tab key handler: advances the ghost-text cycle index and repaints the line.
/// The buffer itself is never modified, so Tab stays on the same line and
/// Right-arrow accepts the ghost.
pub(crate) struct NdbTabCycleHandler {
    cycle: Arc<Mutex<CycleState>>,
    delta: isize,
}

impl ConditionalEventHandler for NdbTabCycleHandler {
    fn handle(
        &self,
        _evt: &Event,
        _n: RepeatCount,
        _positive: bool,
        ctx: &EventContext,
    ) -> Option<Cmd> {
        let line = ctx.line();
        let pos = ctx.pos();
        if line.is_empty() {
            return Some(Cmd::Noop);
        }
        if let Ok(mut c) = self.cycle.lock() {
            if c.line != line || c.pos != pos {
                c.line = line.to_string();
                c.pos = pos;
                c.index = 0;
            } else {
                c.index = c.index.wrapping_add(self.delta);
            }
        }
        Some(Cmd::Repaint)
    }
}

/// Keep candidates that start with `frag` (case-insensitive) and are longer
/// than what was typed. `names` are full candidate words.
pub(crate) fn filter_prefix(names: Vec<String>, frag: &str) -> Vec<String> {
    if frag.is_empty() {
        let mut out = names;
        out.retain(|n| !n.is_empty());
        return out;
    }
    let lower = frag.to_lowercase();
    names
        .into_iter()
        .filter(|n| n.to_lowercase().starts_with(&lower) && n.len() > frag.len())
        .collect()
}
impl Highlighter for NdbHelper {}
impl Completer for NdbHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        Ok((0, vec![]))
    }
}
impl Validator for NdbHelper {}
impl rustyline::Helper for NdbHelper {}

/// Combined rustyline helper wrapping an [`NdbHinter`].
pub(crate) struct NdbHelper {
    hinter: NdbHinter,
}

impl NdbHelper {
    pub(crate) fn new(ctx: Arc<NdbContext>) -> Self {
        NdbHelper {
            hinter: NdbHinter::new(ctx),
        }
    }

    pub(crate) fn tab_handler(&self, delta: isize) -> NdbTabCycleHandler {
        self.hinter.tab_handler(delta)
    }
}

impl Hinter for NdbHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_level_suggests_root_commands() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_db_files(vec!["app.db".to_string(), "shop.sqlite".to_string()]);
        let h = NdbHinter::new(ctx);
        let c = h.candidates_for("in");
        assert!(c.contains(&"interface".to_string()));
        assert!(!c.contains(&"tablef".to_string()));
        assert!(h.candidates_for("tablef").is_empty());
    }

    #[test]
    fn db_level_suggests_db_commands() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        ctx.set_tables(vec!["notes".to_string(), "users".to_string()]);
        let h = NdbHinter::new(ctx);
        let c = h.candidates_for("t");
        assert!(c.contains(&"tables".to_string()));
        assert!(c.contains(&"tablef".to_string()));
    }

    #[test]
    fn interface_completes_db_files_and_numbers() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_db_files(vec!["app.db".to_string(), "shop.sqlite".to_string()]);
        let h = NdbHinter::new(ctx);
        let c = h.candidates_for("interface ");
        assert!(c.contains(&"app.db".to_string()));
        assert!(c.contains(&"shop.sqlite".to_string()));
        assert_eq!(h.candidates_for("open ap"), vec!["app.db".to_string()]);
    }

    #[test]
    fn tablef_completes_table_names() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        ctx.set_tables(vec!["notes".to_string(), "users".to_string()]);
        let h = NdbHinter::new(ctx);
        assert_eq!(
            h.candidates_for("tablef "),
            vec!["notes".to_string(), "users".to_string()]
        );
        assert_eq!(h.candidates_for("schema us"), vec!["users".to_string()]);
        // A second argument is never completed.
        assert!(h.candidates_for("tablef users ").is_empty());
    }

    #[test]
    fn clear_is_offered_at_both_levels() {
        let ctx = Arc::new(NdbContext::new());
        let root = NdbHinter::new(ctx);
        assert!(root.candidates_for("cl").contains(&"clear".to_string()));

        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        let db = NdbHinter::new(ctx);
        assert!(db.candidates_for("cl").contains(&"clear".to_string()));
    }

    #[test]
    fn db_level_suggests_alias_commands() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        ctx.set_tables(vec!["users".to_string()]);
        let h = NdbHinter::new(ctx);
        // Short/open aliases are valid db-level commands.
        assert!(h.candidates_for("q").contains(&"query".to_string()));
        assert!(h.candidates_for("i").contains(&"interface".to_string()));
        // Table-argument aliases complete table names.
        assert_eq!(h.candidates_for("tf "), vec!["users".to_string()]);
        assert_eq!(h.candidates_for("describe "), vec!["users".to_string()]);
        // Tree/list aliases exist alongside the canonical names.
        assert!(h.candidates_for("v").contains(&"viewf".to_string()));
        assert!(h.candidates_for("l").contains(&"list".to_string()));
    }

    #[test]
    fn db_level_suggests_new_commands() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        let h = NdbHinter::new(ctx);
        assert!(h.candidates_for("inf").contains(&"info".to_string()));
        assert!(h.candidates_for("ind").contains(&"indexes".to_string()));
        assert!(h.candidates_for("mor").contains(&"more".to_string()));
    }

    #[test]
    fn db_level_suggests_write_commands() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        let h = NdbHinter::new(ctx);
        assert!(h.candidates_for("ed").contains(&"edit".to_string()));
        assert!(h.candidates_for("wr").contains(&"write".to_string()));
        assert!(h.candidates_for("bac").contains(&"backup".to_string()));
        // Write commands are db-level only, never at the root.
        let ctx = Arc::new(NdbContext::new());
        let root = NdbHinter::new(ctx);
        assert!(root.candidates_for("wr").is_empty());
        assert!(root.candidates_for("ed").is_empty());
    }

    #[test]
    fn sql_completes_tables_after_from_join() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        ctx.set_tables(vec!["notes".to_string(), "users".to_string()]);
        let h = NdbHinter::new(ctx);
        assert_eq!(
            h.candidates_for("sql SELECT * FROM "),
            vec!["notes".to_string(), "users".to_string()]
        );
        assert_eq!(
            h.candidates_for("sql select * from us"),
            vec!["users".to_string()]
        );
        assert_eq!(
            h.candidates_for("sql SELECT * FROM a JOIN "),
            vec!["notes".to_string(), "users".to_string()]
        );
        // No completion for other positions.
        assert!(h.candidates_for("sql SELECT ").is_empty());
        assert!(h.candidates_for("sql SELECT * WHERE ").is_empty());
    }

    #[test]
    fn dbal_suggests_subcommands_and_aliases() {
        let ctx = Arc::new(NdbContext::new());
        ctx.set_aliases(vec!["top_users".to_string()]);
        let h = NdbHinter::new(ctx);
        // `dbal` itself is a command at both levels.
        assert!(h.candidates_for("dba").contains(&"dbal".to_string()));
        // Subcommand completion.
        assert!(h.candidates_for("dbal ").contains(&"run".to_string()));
        let subs = h.candidates_for("dbal r");
        assert!(subs.contains(&"run".to_string()));
        assert!(subs.contains(&"rm".to_string()));
        // Alias-name completion for subcommands that take one.
        assert_eq!(
            h.candidates_for("dbal run "),
            vec!["top_users".to_string()]
        );
        assert_eq!(
            h.candidates_for("dbal run top"),
            vec!["top_users".to_string()]
        );
        // Bare alias names complete as first tokens ...
        assert_eq!(h.candidates_for("top"), vec!["top_users".to_string()]);
        // ... unless they shadow a builtin command.
        let ctx = Arc::new(NdbContext::new());
        ctx.set_in_db(true);
        ctx.set_aliases(vec!["tables".to_string()]);
        let h = NdbHinter::new(ctx);
        assert_eq!(
            h.candidates_for("tabl"),
            vec!["tables".to_string(), "tablef".to_string()]
        );
    }

    #[test]
    fn help_token_suppresses_ghost() {
        let ctx = Arc::new(NdbContext::new());
        let h = NdbHinter::new(ctx);
        assert!(h.candidates_for("help --help").is_empty());
        assert!(h.candidates_for("--").contains(&"--help".to_string()));
    }
}
