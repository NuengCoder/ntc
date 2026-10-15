//! `ndb` — the NTC Database shell ("crack" open a `.db` file and browse it).
//!
//! `ndb` is a nested interactive shell launched from the ntc shell:
//!
//! ```text
//! ntc [project]> ndb          # ndb [project]>  (lists every .db in the cwd)
//! ndb [project]> interface   # ndb [app.db]>    (auto-shows tables)
//! ndb [app.db]> tablef users
//! ndb [app.db]> exit         # ndb [project]>
//! ndb [project]> exit        # back to the ntc shell
//! ```
//!
//! Databases are always opened **read-only** — `ndb` never writes.

pub mod db;
pub mod dbal;
pub mod hinter;
pub mod render;

use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success, print_warning};
use crate::t;
use crate::utils::interactive::{InteractiveMenu, MenuItem};
use crate::utils::terminal::asciify;

use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{Config as RlConfig, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use hinter::{NdbContext, NdbHelper};

use db::{Database, DEFAULT_ROW_LIMIT};
use render::{format_box_table, format_query_result, TableSchema};

// ============================================================================
// Command parsing
// ============================================================================

/// A parsed `ndb` command line.
pub(crate) struct NdbCommand {
    /// Lowercased command word (empty for a blank line).
    cmd: String,
    /// Raw query text — only meaningful for `sql`/`query` (kept verbatim).
    query: String,
    /// Positional arguments (non-flag tokens, surrounding quotes removed).
    args: Vec<String>,
    /// `-l/--limit <n>` value, when given.
    limit: Option<usize>,
    /// `--all` requested every row.
    all: bool,
}

impl NdbCommand {
    /// Effective row limit (`usize::MAX` means "no limit").
    fn row_limit(&self) -> usize {
        if self.all {
            usize::MAX
        } else {
            self.limit.unwrap_or(DEFAULT_ROW_LIMIT)
        }
    }

    /// First positional argument, if any.
    fn arg(&self) -> Option<String> {
        self.args.first().cloned()
    }
}

/// Split `line` into its first whitespace-delimited word and the remainder.
fn split_head(line: &str) -> (&str, &str) {
    let trimmed = line.trim();
    match trimmed.find(char::is_whitespace) {
        Some(pos) => (&trimmed[..pos], trimmed[pos..].trim()),
        None => (trimmed, ""),
    }
}

/// Tokenize args, honoring double quotes (quotes are stripped from output).
fn tokenize(args: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// Consume leading `-l <n>` / `--limit <n>` / `--all` flags, returning the rest.
fn take_leading_flags(rest: &str) -> (Option<usize>, bool, String) {
    let mut limit: Option<usize> = None;
    let mut all = false;
    let mut remaining = rest.trim_start();
    loop {
        let (head, tail) = split_head(remaining);
        match head {
            "-l" | "--limit" => {
                let (num, after) = split_head(tail);
                match num.parse::<usize>() {
                    Ok(n) => {
                        limit = Some(n);
                        remaining = after;
                    }
                    Err(_) => break,
                }
            }
            "-a" | "--all" => {
                all = true;
                remaining = tail;
            }
            _ => break,
        }
    }
    (limit, all, remaining.to_string())
}

/// Split a line on `&&` outside single/double quotes, dropping empties.
/// SQL strings are single-quoted, so both quote kinds hide the separator
/// (`''` is an escaped quote inside a string, not a boundary).
fn split_chain(line: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut in_single = false;
    let mut in_double = false;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (pos, ch) = chars[i];
        match ch {
            '\'' if !in_double => {
                if in_single && i + 1 < chars.len() && chars[i + 1].1 == '\'' {
                    i += 1;
                } else {
                    in_single = !in_single;
                }
            }
            '"' if !in_single => in_double = !in_double,
            '&' if !in_single && !in_double && i + 1 < chars.len() && chars[i + 1].1 == '&' => {
                let piece = line[start..pos].trim();
                if !piece.is_empty() {
                    parts.push(piece);
                }
                i += 1; // skip the second `&`
                start = chars[i].0 + 1;
            }
            _ => {}
        }
        i += 1;
    }
    let tail = line[start..].trim();
    if !tail.is_empty() {
        parts.push(tail);
    }
    parts
}

/// Parse a raw `ndb` line into a command.
///
/// `sql`/`query` keep their remainder verbatim (flags are only pulled from the
/// front) so the query text is never rewritten. Every other command accepts
/// `-l/--limit <n>` and `-a/--all` anywhere on the line.
fn parse_command(line: &str) -> NdbCommand {
    let (head, rest) = split_head(line);
    let cmd = head.to_lowercase();

    // `write` keeps its remainder verbatim like `sql` so string literals
    // survive untouched (`write YES INSERT INTO t VALUES ('a b')`).
    if cmd == "sql" || cmd == "query" || cmd == "write" {
        let (limit, all, query) = take_leading_flags(rest);
        return NdbCommand {
            cmd,
            query,
            args: Vec::new(),
            limit,
            all,
        };
    }

    let mut limit: Option<usize> = None;
    let mut all = false;
    let mut args: Vec<String> = Vec::new();
    let mut tokens = tokenize(rest).into_iter();
    while let Some(tok) = tokens.next() {
        match tok.as_str() {
            "-l" | "--limit" => {
                if let Some(n) = tokens.next().and_then(|v| v.parse::<usize>().ok()) {
                    limit = Some(n);
                }
            }
            "-a" | "--all" => all = true,
            _ => args.push(tok),
        }
    }

    NdbCommand {
        cmd,
        query: String::new(),
        args,
        limit,
        all,
    }
}

/// What the shell should do after handling a command.
pub(crate) enum Action {
    /// Stay in the current level.
    Stay,
    /// Leave the open database, returning to `ndb [<cwd>]>`.
    Close,
    /// Replace/open the current database.
    Opened(Box<Database>),
    /// Leave the `ndb` shell entirely.
    Exit,
}

/// Current shell level.
enum NdbState {
    Root,
    Db(Box<Database>),
}

// ============================================================================
// Display helpers
// ============================================================================

/// Print the `ndb` banner with the working directory.
fn print_banner(cwd: &Path) {
    println!();
    println!(
        "{}",
        asciify("╔══════════════════════════════════════════════════════════════════╗")
    );
    println!(
        "{}",
        format!("{:^66}", t!("ndb.banner", env!("CARGO_PKG_VERSION")))
            .cyan()
            .bold()
    );
    println!(
        "{}",
        asciify("╚══════════════════════════════════════════════════════════════════╝")
    );
    println!(
        "{}",
        t!("ndb.cwd_line", crate::filetype::strip_unc_prefix(cwd)).dimmed()
    );
}

/// Print every `.db` file found in `cwd` (this is `ndb view`).
fn list_databases(cwd: &Path) {
    let files = db::list_db_files(cwd);
    println!();
    if files.is_empty() {
        println!("{}", t!("ndb.no_db_files").yellow());
        println!("{}", t!("ndb.no_db_files_hint").dimmed());
        println!();
        return;
    }
    println!("{}", t!("ndb.db_files_header", files.len()).cyan().bold());
    for (i, file) in files.iter().enumerate() {
        let size = crate::explorer::human_readable_size(file.size);
        let detail = match file.table_count {
            Some(n) => t!("ndb.tables_count", n),
            None => t!("ndb.not_a_db").to_string(),
        };
        println!(
            "  {} {} {}",
            format!("[{}]", i + 1).yellow(),
            file.name.blue(),
            format!("({}, {})", size, detail).dimmed()
        );
    }
    println!();
}

/// Render a tree produced by the render helpers, coloring branch lines.
fn print_tree(tree: &str) {
    #[cfg(not(target_os = "android"))]
    {
        let theme = crate::utils::theme::ThemeManager::current();
        for line in tree.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("├──") || trimmed.starts_with("└──") {
                println!("{}", line.color(theme.shell.tree_branch.to_colored()));
            } else {
                println!("{}", line);
            }
        }
    }
    #[cfg(target_os = "android")]
    {
        for line in tree.lines() {
            println!("{}", line);
        }
    }
}

/// Resolve `interface` args: a list number or a file name/path.
fn resolve_db_spec(spec: &str, cwd: &Path) -> Option<PathBuf> {
    let spec = spec.trim();
    if spec.is_empty() {
        return None;
    }
    if let Ok(idx) = spec.parse::<usize>() {
        if idx >= 1 {
            if let Some(file) = db::list_db_files(cwd).get(idx - 1) {
                return Some(file.path.clone());
            }
        }
        return None;
    }
    let path = PathBuf::from(spec);
    let candidate = if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    };
    if candidate.is_file() {
        Some(candidate)
    } else {
        None
    }
}

/// Open a database read-only, reporting failures to the user.
fn open_database(path: &Path) -> Option<Box<Database>> {
    match Database::open(path) {
        Ok(db) => Some(Box::new(db)),
        Err(e) => {
            print_error(&t!("ndb.open_failed", path.display(), e));
            None
        }
    }
}

/// Show the table tree of a freshly opened database.
fn print_db_overview(db: &Database) {
    match db.tables() {
        Ok(tables) => {
            println!();
            println!("{}", t!("ndb.opened", db.name).green().bold());
            println!();
            print_tree(&render::format_table_list(&db.name, &tables));
            println!();
            println!("{}", t!("ndb.hint_next").dimmed());
        }
        Err(e) => print_error(&t!("ndb.query_failed", e)),
    }
}

/// Re-show the current context after `clear`: table tree without the
/// "Opened database" header (the db is already open).
fn redraw_db_context(db: &Database) {
    match db.tables() {
        Ok(tables) => {
            println!();
            print_tree(&render::format_table_list(&db.name, &tables));
            println!();
            println!("{}", t!("ndb.hint_next").dimmed());
        }
        Err(e) => print_error(&t!("ndb.query_failed", e)),
    }
}

/// Re-show the root context after `clear`: banner + `.db` file list.
fn redraw_root_context(cwd: &Path) {
    print_banner(cwd);
    list_databases(cwd);
}

/// `clear`/`cls` at the root level: wipe and re-show banner + file list.
fn clear_root(cwd: &Path) {
    clear_screen();
    redraw_root_context(cwd);
}

/// `clear`/`cls` with a database open: wipe and re-show the table tree.
fn clear_db(db: &Database) {
    clear_screen();
    redraw_db_context(db);
}

/// Interactive picker over the `.db` files in `cwd`.
fn pick_db_file(cwd: &Path) -> Option<PathBuf> {
    let files = db::list_db_files(cwd);
    if files.is_empty() {
        print_warning(&t!("ndb.no_db_files"));
        return None;
    }
    let items: Vec<MenuItem> = files
        .iter()
        .map(|f| {
            let size = crate::explorer::human_readable_size(f.size);
            let detail = match f.table_count {
                Some(n) => format!("({}, {})", size, t!("ndb.tables_count", n)),
                None => format!("({}, {})", size, t!("ndb.not_a_db")),
            };
            MenuItem::with_detail(f.name.clone(), detail)
        })
        .collect();
    let title = t!("ndb.pick_db");
    let prompt = t!("ndb.pick_prompt", files.len());
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(prompt)
        .cancel_label("Cancel");
    match menu.run() {
        Ok(Some(idx)) => files.get(idx).map(|f| f.path.clone()),
        _ => None,
    }
}

/// Interactive picker over the tables of an open database.
fn pick_table(db: &Database) -> Option<String> {
    let tables = match db.tables() {
        Ok(t) => t,
        Err(e) => {
            print_error(&t!("ndb.query_failed", e));
            return None;
        }
    };
    if tables.is_empty() {
        print_warning(&t!("ndb.no_tables"));
        return None;
    }
    let items: Vec<MenuItem> = tables
        .iter()
        .map(|t| MenuItem::with_detail(t.name.clone(), render::table_detail(t)))
        .collect();
    let title = t!("ndb.pick_table");
    let prompt = t!("ndb.pick_prompt", tables.len());
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(prompt)
        .cancel_label("Cancel");
    match menu.run() {
        Ok(Some(idx)) => tables.get(idx).map(|t| t.name.clone()),
        _ => None,
    }
}

// ============================================================================
// Database commands
// ============================================================================

/// Print a rendered box table, highlighting its header row.
fn print_table_box(boxed: &str) {
    #[cfg(not(target_os = "android"))]
    {
        for (i, line) in boxed.lines().enumerate() {
            if i == 1 {
                println!("{}", line.cyan().bold());
            } else {
                println!("{}", line);
            }
        }
    }
    #[cfg(target_os = "android")]
    {
        for line in boxed.lines() {
            println!("{}", line);
        }
    }
}

/// `tables [pattern]` — flat list of table/view names with column and row
/// counts, optionally filtered by a case-insensitive substring.
fn show_table_list(db: &Database, filter: Option<&str>) {
    match db.tables() {
        Ok(tables) => {
            println!();
            let tables: Vec<&db::TableInfo> = match filter {
                Some(pat) => {
                    let needle = pat.to_lowercase();
                    tables
                        .iter()
                        .filter(|t| t.name.to_lowercase().contains(&needle))
                        .collect()
                }
                None => tables.iter().collect(),
            };
            if tables.is_empty() {
                println!("{}", t!("ndb.no_tables").yellow());
                return;
            }
            let headers = vec![
                t!("ndb.col_name"),
                t!("ndb.col_kind"),
                t!("ndb.col_columns"),
                t!("ndb.col_rows"),
            ];
            let rows: Vec<Vec<Option<String>>> = tables
                .iter()
                .map(|t| {
                    let kind = if t.is_view() {
                        t!("ndb.kind_view")
                    } else {
                        t!("ndb.kind_table")
                    };
                    vec![
                        Some(t.name.clone()),
                        Some(kind),
                        Some(t.columns.to_string()),
                        Some(t.rows.to_string()),
                    ]
                })
                .collect();
            print_table_box(&asciify(&format_box_table(
                &headers,
                &rows,
                render::MAX_CELL_WIDTH,
            )));
            println!();
        }
        Err(e) => print_error(&t!("ndb.query_failed", e)),
    }
}

/// `viewf` — schema tree of tables and their columns.
fn show_schema_tree(db: &Database) {
    match db.tables() {
        Ok(tables) => {
            let mut cols_per_table = Vec::with_capacity(tables.len());
            for info in &tables {
                cols_per_table.push(db.columns(&info.name).unwrap_or_default());
            }
            let schemas: Vec<TableSchema<'_>> = tables
                .iter()
                .zip(cols_per_table.iter())
                .map(|(info, columns)| TableSchema { info, columns })
                .collect();
            println!();
            print_tree(&render::format_schema_tree(&db.name, &schemas));
            println!();
        }
        Err(e) => print_error(&t!("ndb.query_failed", e)),
    }
}

/// `schema` — `CREATE` SQL plus a column table.
fn show_table_schema(db: &Database, table: &str) {
    let create = match db.create_sql(table) {
        Some(sql) => sql,
        None => {
            print_error(&t!("ndb.table_not_found", table));
            return;
        }
    };
    println!();
    println!("{}", t!("ndb.schema_header", table).cyan().bold());
    println!("{}", create.cyan());

    let columns = db.columns(table).unwrap_or_default();
    if !columns.is_empty() {
        let headers = vec![
            t!("ndb.col_name"),
            t!("ndb.col_type"),
            t!("ndb.col_constraints"),
        ];
        let rows: Vec<Vec<Option<String>>> = columns
            .iter()
            .map(|c| {
                let col_type = if c.col_type.is_empty() {
                    "ANY".to_string()
                } else {
                    c.col_type.clone()
                };
                vec![Some(c.name.clone()), Some(col_type), Some(c.constraint())]
            })
            .collect();
        println!();
        print_table_box(&asciify(&format_box_table(
            &headers,
            &rows,
            render::MAX_CELL_WIDTH,
        )));
    }

    // Indexes on this table (skipped when there are none).
    if let Ok(indexes) = db.indexes(table) {
        if !indexes.is_empty() {
            let yes = t!("ndb.yes");
            let no = t!("ndb.no");
            println!();
            println!("{}", t!("ndb.indexes_title", indexes.len()).cyan().bold());
            let headers = vec![t!("ndb.col_name"), t!("ndb.col_unique")];
            let rows: Vec<Vec<Option<String>>> = indexes
                .iter()
                .map(|i| {
                    vec![
                        Some(i.name.clone()),
                        Some(if i.unique { yes.clone() } else { no.clone() }),
                    ]
                })
                .collect();
            print_table_box(&asciify(&format_box_table(
                &headers,
                &rows,
                render::MAX_CELL_WIDTH,
            )));
            for index in &indexes {
                if let Some(sql) = index.sql.as_deref() {
                    println!("  {}", sql.dimmed());
                }
            }
        }
    }

    // Foreign keys (skipped when there are none).
    if let Ok(fkeys) = db.foreign_keys(table) {
        if !fkeys.is_empty() {
            println!();
            println!("{}", t!("ndb.fkeys_title").cyan().bold());
            let headers = vec![t!("ndb.col_from"), t!("ndb.col_to")];
            let rows: Vec<Vec<Option<String>>> = fkeys
                .iter()
                .map(|fk| {
                    vec![
                        Some(fk.from.clone()),
                        Some(format!("{}.{}", fk.table, fk.to)),
                    ]
                })
                .collect();
            print_table_box(&asciify(&format_box_table(
                &headers,
                &rows,
                render::MAX_CELL_WIDTH,
            )));
        }
    }

    // Triggers on this table (skipped when there are none).
    let triggers = db.triggers(table);
    if !triggers.is_empty() {
        println!();
        println!("{}", t!("ndb.triggers_title").cyan().bold());
        for trigger in &triggers {
            println!("{}", trigger.cyan());
        }
    }
    println!();
}

/// Run a read-only query and render the result.
/// The last paged query, so `more` can fetch the next chunk.
/// `shown` counts rows already displayed; the base SQL is re-run with a
/// larger collection limit and the leading `shown` rows are skipped
/// client-side (keeps working even when the SQL has its own `LIMIT`).
pub(crate) struct LastQuery {
    base: String,
    limit: usize,
    shown: usize,
}

/// Run `base_sql`, skipping the first `skip` rows and showing at most
/// `limit` after that. Returns the new paging state on success.
fn run_paged(db: &Database, base_sql: &str, limit: usize, skip: usize) -> Option<LastQuery> {
    let take = if limit == usize::MAX {
        usize::MAX
    } else {
        skip.saturating_add(limit)
    };
    match db.query(base_sql, take) {
        Ok(result) => {
            let available = result.rows.len();
            let page_rows: Vec<Vec<Option<String>>> = result.rows.into_iter().skip(skip).collect();
            let shown = page_rows.len();
            if shown == 0 {
                println!("{}", t!("ndb.more_empty").yellow());
                return None;
            }
            // Still truncated when the collection hit the take cap, or when
            // the reported truncation survives slicing.
            let truncated = available >= take || (result.truncated && shown > 0);
            let page = db::QueryResult {
                columns: result.columns,
                rows: page_rows,
                truncated,
                elapsed_ms: result.elapsed_ms,
            };
            println!();
            print_table_box(&asciify(&format_query_result(&page, limit)));
            println!();
            Some(LastQuery {
                base: base_sql.to_string(),
                limit,
                shown: skip.saturating_add(shown),
            })
        }
        Err(e) => {
            print_error(&t!("ndb.sql_error", e));
            None
        }
    }
}

/// Run a read-only query and render the first page.
/// Returns paging state for `more` on success.
fn run_sql(db: &Database, sql: &str, limit: usize) -> Option<LastQuery> {
    let sql = sql.trim();
    if sql.is_empty() {
        print_error(&t!("ndb.sql_usage"));
        return None;
    }
    if !db::is_read_only_sql(sql) {
        print_error(&t!("ndb.sql_blocked"));
        return None;
    }
    run_paged(db, sql, limit, 0)
}

/// `tablef` — display the rows of a single table.
/// Returns paging state for `more` on success.
fn show_table_rows(db: &Database, table: &str, limit: usize) -> Option<LastQuery> {
    let exists = db
        .tables()
        .map(|t| t.iter().any(|x| x.name == table))
        .unwrap_or(false);
    if !exists {
        print_error(&t!("ndb.table_not_found", table));
        return None;
    }
    let base = format!("SELECT * FROM {}", db::quote_ident(table));
    println!("{}", t!("ndb.showing_table", table).dimmed());
    run_sql(db, &base, limit)
}

/// `more` — show the next page of the last query.
fn show_more(db: &Database, page: &mut Option<LastQuery>) {
    let last = match page {
        Some(p) => p,
        None => {
            print_error(&t!("ndb.no_query_yet"));
            return;
        }
    };
    if last.limit == usize::MAX {
        println!("{}", t!("ndb.more_empty").yellow());
        return;
    }
    if let Some(next) = run_paged(db, &last.base.clone(), last.limit, last.shown) {
        *page = Some(next);
    }
}

/// `info` — database file, engine and object counts.
fn show_db_info(db: &Database) {
    let info = db.info();
    let size = std::fs::metadata(&db.path)
        .map(|m| crate::explorer::human_readable_size(m.len()))
        .unwrap_or_else(|_| "-".to_string());
    println!();
    println!("{}", t!("ndb.info_title", db.name).cyan().bold());
    let headers = vec![t!("ndb.col_property"), t!("ndb.col_value")];
    let rows: Vec<Vec<Option<String>>> = vec![
        (t!("ndb.info_file"), db.path.display().to_string()),
        (t!("ndb.info_size"), size),
        (t!("ndb.info_sqlite"), info.sqlite_version),
        (t!("ndb.info_page"), info.page_size.to_string()),
        (t!("ndb.info_pages"), info.page_count.to_string()),
        (t!("ndb.info_tables"), info.tables.to_string()),
        (t!("ndb.info_views"), info.views.to_string()),
        (t!("ndb.info_indexes"), info.indexes.to_string()),
        (t!("ndb.info_triggers"), info.triggers.to_string()),
    ]
    .into_iter()
    .map(|(k, v)| vec![Some(k), Some(v)])
    .collect();
    print_table_box(&asciify(&format_box_table(
        &headers,
        &rows,
        render::MAX_CELL_WIDTH,
    )));
    println!();
}

/// `indexes` — every index, or only those of one table.
fn show_indexes(db: &Database, table: Option<&str>) {
    let indexes = match db.all_indexes() {
        Ok(list) => list,
        Err(e) => {
            print_error(&t!("ndb.query_failed", e));
            return;
        }
    };
    let filtered: Vec<&db::IndexInfo> = match table {
        Some(name) => indexes
            .iter()
            .filter(|i| i.table.eq_ignore_ascii_case(name))
            .collect(),
        None => indexes.iter().collect(),
    };
    println!();
    if filtered.is_empty() {
        println!("{}", t!("ndb.no_indexes").yellow());
        return;
    }
    let yes = t!("ndb.yes");
    let no = t!("ndb.no");
    println!("{}", t!("ndb.indexes_title", filtered.len()).cyan().bold());
    let headers = vec![
        t!("ndb.col_name"),
        t!("ndb.col_table"),
        t!("ndb.col_unique"),
    ];
    let rows: Vec<Vec<Option<String>>> = filtered
        .iter()
        .map(|i| {
            vec![
                Some(i.name.clone()),
                Some(i.table.clone()),
                Some(if i.unique { yes.clone() } else { no.clone() }),
            ]
        })
        .collect();
    print_table_box(&asciify(&format_box_table(
        &headers,
        &rows,
        render::MAX_CELL_WIDTH,
    )));
    println!();
}

// ============================================================================
// Safe-write helpers (`edit` / `write` / `backup`)
// ============================================================================

/// Copy the database file next to itself once per open handle, before the
/// first write. Later writes skip the copy (`backed_up` flag on the handle).
fn ensure_backup(db: &mut Database) -> bool {
    if db.backed_up {
        return true;
    }
    match db::backup_file(&db.path) {
        Ok(dest) => {
            db.backed_up = true;
            println!("{}", t!("ndb.backup_done", dest.display()).green());
            true
        }
        Err(e) => {
            print_error(&t!("ndb.backup_failed", e));
            false
        }
    }
}

/// `edit on|off|status` — flip the session write gate. Turning `on`
/// reopens the handle read-write in place; turning `off` keeps the handle
/// (SQLite has no downgrade) but blocks further `write` commands.
fn cmd_edit(db: &mut Database, arg: Option<&str>) {
    match arg.map(str::to_lowercase).as_deref() {
        Some("on") => match db.make_writable() {
            Ok(()) => {
                db.edit_on = true;
                println!("{}", t!("ndb.edit_on", db.name).yellow());
                println!("{}", t!("ndb.edit_warn").dimmed());
            }
            Err(e) => print_error(&t!("ndb.edit_failed", e)),
        },
        Some("off") => {
            db.edit_on = false;
            println!("{}", t!("ndb.edit_off").dimmed());
        }
        _ => {
            let state = if db.edit_on {
                t!("ndb.edit_state_on")
            } else {
                t!("ndb.edit_state_off")
            };
            println!("{}", t!("ndb.edit_status", state));
            println!("{}", t!("ndb.edit_usage").dimmed());
        }
    }
}

/// Strip an explicit confirmation prefix (`YES`, `-y`, `--yes`) from a
/// `write` body. Returns `(confirmed, sql)`.
fn split_write_confirm(body: &str) -> (bool, &str) {
    let trimmed = body.trim_start();
    for prefix in ["YES ", "YES\t", "YES\n", "-y ", "--yes "] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return (true, rest.trim_start());
        }
    }
    // Bare `YES` with nothing after it: confirmed but empty (usage error).
    if trimmed == "YES" || trimmed == "-y" || trimmed == "--yes" {
        return (true, "");
    }
    (false, body)
}

/// `write YES <INSERT|UPDATE|DELETE|REPLACE ...>` — one row-level write
/// inside a transaction. Without the `YES` prefix only the confirm hint
/// is shown, so a stray paste can never mutate the file.
fn cmd_write(db: &mut Database, body: &str, page: &mut Option<LastQuery>) {
    if !db.edit_on {
        print_error(&t!("ndb.write_needs_edit"));
        return;
    }
    let (confirmed, sql) = split_write_confirm(body);
    let sql = sql.trim().trim_end_matches(';').trim();
    if sql.is_empty() {
        print_error(&t!("ndb.write_usage"));
        return;
    }
    if !db::is_write_sql(sql) {
        print_error(&t!("ndb.write_blocked"));
        return;
    }
    if !confirmed {
        println!("{}", t!("ndb.write_confirm", sql).yellow());
        println!("{}", t!("ndb.write_confirm_hint").dimmed());
        return;
    }
    if !ensure_backup(db) {
        return;
    }
    match db.exec_write(sql) {
        Ok(n) => {
            *page = None;
            print_success(&t!("ndb.write_done", n));
        }
        Err(e) => print_error(&t!("ndb.write_failed", e)),
    }
}

/// `backup` — manual copy of the database file next to itself.
fn cmd_backup(db: &mut Database) {
    match db::backup_file(&db.path) {
        Ok(dest) => {
            db.backed_up = true;
            print_success(&t!("ndb.backup_done", dest.display()));
        }
        Err(e) => print_error(&t!("ndb.backup_failed", e)),
    }
}

// ============================================================================
// Level handlers
// ============================================================================

/// Help listing for the `ndb [<cwd>]>` level.
fn print_root_help() {
    println!();
    println!("{}", t!("ndb.help_root_title").cyan().bold());
    for key in [
        "ndb.help_root_help",
        "ndb.help_root_view",
        "ndb.help_root_interface",
        "ndb.help_root_sql",
        "ndb.help_dbal",
        "ndb.help_root_clear",
        "ndb.help_root_exit",
    ] {
        println!("  {}", t!(key));
    }
    println!();
    println!("{}", t!("ndb.help_note").dimmed());
    println!();
}

/// Help listing for the `ndb [<file.db>]>` level.
fn print_db_help() {
    println!();
    println!("{}", t!("ndb.help_db_title").cyan().bold());
    for key in [
        "ndb.help_db_help",
        "ndb.help_db_viewf",
        "ndb.help_db_tables",
        "ndb.help_db_tablef",
        "ndb.help_db_schema",
        "ndb.help_db_sql",
        "ndb.help_db_write",
        "ndb.help_db_edit",
        "ndb.help_db_backup",
        "ndb.help_db_more",
        "ndb.help_db_indexes",
        "ndb.help_db_info",
        "ndb.help_dbal",
        "ndb.help_db_interface",
        "ndb.help_db_clear",
        "ndb.help_db_exit",
    ] {
        println!("  {}", t!(key));
    }
    println!();
    println!("{}", t!("ndb.help_readonly").dimmed());
    println!("{}", t!("ndb.help_limit").dimmed());
    println!();
}

/// Commands available at `ndb [<cwd>]>`.
fn handle_root(command: &NdbCommand, cwd: &Path) -> Action {
    match command.cmd.as_str() {
        "help" | "h" | "?" => {
            print_root_help();
            Action::Stay
        }
        "view" | "ls" | "list" | "show" => {
            list_databases(cwd);
            Action::Stay
        }
        "interface" | "use" | "open" | "in" | "i" => {
            let target = match command.arg() {
                Some(spec) => resolve_db_spec(&spec, cwd),
                None => pick_db_file(cwd),
            };
            match target.and_then(|path| open_database(&path)) {
                Some(db) => {
                    print_db_overview(&db);
                    Action::Opened(db)
                }
                None => {
                    if command.arg().is_some() {
                        print_error(&t!("ndb.interface_usage"));
                    }
                    Action::Stay
                }
            }
        }
        "sql" | "query" | "info" | "indexes" | "more" | "next" => {
            print_error(&t!("ndb.sql_needs_db"));
            Action::Stay
        }
        "dbal" => {
            let mut sink = None;
            dbal::handle_dbal(command, None, &mut sink)
        }
        "clear" | "cls" => {
            clear_root(cwd);
            Action::Stay
        }
        "exit" | "quit" | "q" => Action::Exit,
        _ => {
            // Bare `dbal` alias names execute (with `$params` from the
            // trailing words) — but they need an open DB.
            match dbal::try_bare(
                &command.cmd,
                &command.args,
                None,
                command.row_limit(),
                &mut None,
            ) {
                Some(action) => action,
                None => {
                    print_error(&t!("ndb.unknown_cmd", command.cmd));
                    println!("{}", t!("ndb.hint_help").dimmed());
                    Action::Stay
                }
            }
        }
    }
}

/// Commands available at `ndb [<file.db>]>`.
/// The write gate + backup flag live on the `Database` handle itself,
/// so fresh `interface` opens always start locked (`edit off`).
fn handle_db(
    command: &NdbCommand,
    db: &mut Database,
    cwd: &Path,
    page: &mut Option<LastQuery>,
) -> Action {
    match command.cmd.as_str() {
        "help" | "h" | "?" => {
            print_db_help();
            Action::Stay
        }
        // `viewf` == schema tree of the whole database.
        "viewf" | "vf" | "view" | "v" => {
            show_schema_tree(db);
            Action::Stay
        }
        "tables" | "t" | "ls" | "list" => {
            let filter = command.arg();
            show_table_list(db, filter.as_deref());
            Action::Stay
        }
        // `tablef` == `txtf` for a table.
        "tablef" | "tblf" | "tf" => {
            let table = match command.arg() {
                Some(name) => Some(name),
                None => pick_table(db),
            };
            if let Some(name) = table {
                *page = show_table_rows(db, &name, command.row_limit());
            }
            Action::Stay
        }
        "schema" | "desc" | "describe" => match command.arg() {
            Some(name) => {
                show_table_schema(db, &name);
                Action::Stay
            }
            None => {
                print_error(&t!("ndb.schema_usage"));
                Action::Stay
            }
        },
        "sql" | "query" => {
            *page = run_sql(db, &command.query, command.row_limit());
            Action::Stay
        }
        // `edit` flips the session write gate; `write` runs one
        // confirmed row-level statement; `backup` snapshots the file.
        // `sql` itself stays read-only forever.
        "edit" => {
            let arg = command.arg();
            cmd_edit(db, arg.as_deref());
            Action::Stay
        }
        "write" => {
            cmd_write(db, &command.query, page);
            Action::Stay
        }
        "backup" | "bak" => {
            cmd_backup(db);
            Action::Stay
        }
        "more" | "next" | "m" => {
            show_more(db, page);
            Action::Stay
        }
        "indexes" | "idx" => {
            let table = command.arg();
            show_indexes(db, table.as_deref());
            Action::Stay
        }
        "info" => {
            show_db_info(db);
            Action::Stay
        }
        "interface" | "use" | "open" | "in" | "i" => {
            let target = match command.arg() {
                Some(spec) => resolve_db_spec(&spec, cwd),
                None => pick_db_file(cwd),
            };
            match target.and_then(|path| open_database(&path)) {
                Some(next) => {
                    print_db_overview(&next);
                    Action::Opened(next)
                }
                None => {
                    if command.arg().is_some() {
                        print_error(&t!("ndb.interface_usage"));
                    }
                    Action::Stay
                }
            }
        }
        "dbal" => dbal::handle_dbal(command, Some(&*db), page),
        "clear" | "cls" => {
            clear_db(db);
            Action::Stay
        }
        "exit" | "quit" | "q" => Action::Close,
        _ => {
            // Bare `dbal` alias names run directly (builtins always win,
            // since this branch only handles unknown commands).
            match dbal::try_bare(
                &command.cmd,
                &command.args,
                Some(&*db),
                command.row_limit(),
                page,
            ) {
                Some(action) => action,
                None => {
                    print_error(&t!("ndb.unknown_cmd", command.cmd));
                    println!("{}", t!("ndb.hint_help").dimmed());
                    Action::Stay
                }
            }
        }
    }
}

// ============================================================================
// Shell loop
// ============================================================================

/// `ntc_history.txt` -> `ntc_history.txt.ndb` (ndb keeps its own history).
fn ndb_history_path(base: &Path) -> PathBuf {
    let mut path = base.to_path_buf();
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "ntc_history".to_string());
    path.set_file_name(format!("{}.ndb", name));
    path
}

/// Refresh the hinter context from the current shell state.
/// `tables` is empty at the root level; `db_files` always mirrors `cwd`.
fn sync_hinter_context(ctx: &NdbContext, cwd: &Path, state: &NdbState) {
    let files: Vec<String> = db::list_db_files(cwd).into_iter().map(|f| f.name).collect();
    ctx.set_db_files(files);
    let mut aliases: Vec<String> = crate::config::Config::get_db_aliases()
        .into_keys()
        .collect();
    aliases.sort();
    ctx.set_aliases(aliases);
    match state {
        NdbState::Root => {
            ctx.set_in_db(false);
            ctx.set_tables(Vec::new());
        }
        NdbState::Db(db) => {
            ctx.set_in_db(true);
            let tables: Vec<String> = db
                .tables()
                .map(|t| t.into_iter().map(|info| info.name).collect())
                .unwrap_or_default();
            ctx.set_tables(tables);
        }
    }
}

/// Read continuation lines until one ends with `;`, joining them into a
/// single statement. Returns `None` when the user aborts (Ctrl-C / Ctrl-D).
fn read_multiline_sql(rl: &mut Editor<NdbHelper, FileHistory>) -> Option<String> {
    println!("{}", t!("ndb.multiline_hint").dimmed());
    let mut buf = String::new();
    loop {
        match rl.readline("...+> ") {
            Ok(line) => {
                rl.add_history_entry(&line).ok();
                if !buf.is_empty() {
                    buf.push('\n');
                }
                buf.push_str(&line);
                if line.trim_end().ends_with(';') {
                    return Some(buf);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                return None;
            }
            Err(_) => return None,
        }
    }
}

/// Run the interactive `ndb` shell until the user exits it.
pub fn run_ndb_shell(nav: &mut Navigator) -> Result<()> {
    let cwd = nav.current_path().to_path_buf();
    let dir_label = nav.current_dir_name();

    let ctx = Arc::new(NdbContext::new());
    let mut rl = Editor::<NdbHelper, FileHistory>::with_config(RlConfig::default())
        .map_err(|e| anyhow::anyhow!("{}", t!("ndb.editor_fail", e)))?;
    let helper = NdbHelper::new(Arc::clone(&ctx));
    let tab_next = helper.tab_handler(1);
    let tab_prev = helper.tab_handler(-1);
    rl.set_helper(Some(helper));
    // Tab / Shift-Tab cycle ghost-text candidates in place (no newline);
    // Right-arrow (rustyline default) accepts the ghost.
    rl.bind_sequence(
        KeyEvent(KeyCode::Tab, Modifiers::NONE),
        EventHandler::Conditional(Box::new(tab_next)),
    );
    rl.bind_sequence(
        KeyEvent(KeyCode::BackTab, Modifiers::NONE),
        EventHandler::Conditional(Box::new(tab_prev)),
    );

    let history_path = crate::config::Config::read_global()
        .resolve_history_path()
        .map(|p| ndb_history_path(&p));
    if let Some(ref hp) = history_path {
        let _ = rl.load_history(hp);
    }

    print_banner(&cwd);
    list_databases(&cwd);
    println!("{}", t!("ndb.start_hint").dimmed());

    let mut state = NdbState::Root;
    let mut page: Option<LastQuery> = None;
    sync_hinter_context(&ctx, &cwd, &state);
    loop {
        // Prompts must stay plain text: ANSI codes break rustyline's cursor
        // positioning (same constraint as the main ntc shell prompt).
        let prompt = match &state {
            NdbState::Root => format!("ndb [{}]> ", dir_label),
            NdbState::Db(db) => format!("ndb [{}]> ", db.name),
        };

        let line = match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(&line).ok();
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D steps out one level, like `exit`.
                if matches!(state, NdbState::Db(_)) {
                    println!("{}", t!("ndb.closed_db"));
                    state = NdbState::Root;
                    sync_hinter_context(&ctx, &cwd, &state);
                    continue;
                }
                break;
            }
            Err(_) => break,
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // `&&` sequences commands: every segment runs in order with state
        // updates (open/close) applied between segments. Only `exit`
        // short-circuits the rest of the chain.
        let segments = split_chain(trimmed);
        let single = segments.len() == 1;
        let mut shell_done = false;
        for segment in segments {
            let mut command = parse_command(segment);
            if command.cmd.is_empty() {
                continue;
            }

            // Bare `sql` opens multi-line mode: keep reading `...>` lines
            // until one ends with `;` (Ctrl-C aborts). Prompting mid-chain
            // would stall the sequence, so chains require inline SQL.
            if (command.cmd == "sql" || command.cmd == "query") && command.query.is_empty() {
                if !single {
                    print_error(&t!("ndb.sql_usage"));
                    continue;
                }
                match read_multiline_sql(&mut rl) {
                    Some(collected) => command.query = collected,
                    None => continue,
                }
            }

            let action = match &mut state {
                NdbState::Root => handle_root(&command, &cwd),
                NdbState::Db(db) => handle_db(&command, db, &cwd, &mut page),
            };

            match action {
                Action::Stay => {}
                Action::Close => {
                    println!("{}", t!("ndb.closed_db"));
                    state = NdbState::Root;
                    page = None;
                }
                Action::Opened(db) => {
                    state = NdbState::Db(db);
                    page = None;
                }
                Action::Exit => {
                    shell_done = true;
                    break;
                }
            }
            sync_hinter_context(&ctx, &cwd, &state);
        }
        if shell_done {
            break;
        }
    }

    if let Some(hp) = history_path {
        let _ = rl.save_history(&hp);
    }
    println!("{}", t!("ndb.bye").dimmed());
    Ok(())
}

/// Print the `ndb` help topic (`help ndb` / `ndb --help` in the ntc shell).
pub fn print_help_topic() {
    crate::shell::help::print_ndb_help();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `with_test_config` swaps the process-global singleton, so tests that
    /// use it must not run concurrently with each other.
    static CONFIG_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn parse_command_reads_flags_at_any_depth() {
        let c = parse_command("TABLEF users --all");
        assert_eq!(c.cmd, "tablef");
        assert_eq!(c.arg().as_deref(), Some("users"));
        assert!(c.all);
        assert_eq!(c.row_limit(), usize::MAX);

        let c = parse_command("tablef -l 5 users");
        assert_eq!(c.limit, Some(5));
        assert_eq!(c.row_limit(), 5);
        assert_eq!(c.arg().as_deref(), Some("users"));

        let c = parse_command("tables");
        assert_eq!(c.cmd, "tables");
        assert_eq!(c.arg(), None);
        assert_eq!(c.row_limit(), DEFAULT_ROW_LIMIT);
    }

    #[test]
    fn sql_keeps_the_query_verbatim() {
        let c = parse_command("sql SELECT * FROM users WHERE name = 'a b'");
        assert_eq!(c.cmd, "sql");
        assert_eq!(c.query, "SELECT * FROM users WHERE name = 'a b'");
        assert_eq!(c.row_limit(), DEFAULT_ROW_LIMIT);

        // Leading flags are pulled off, the query itself is untouched.
        let c = parse_command("sql -l 3 SELECT 1");
        assert_eq!(c.limit, Some(3));
        assert_eq!(c.query, "SELECT 1");
    }

    #[test]
    fn quoted_arguments_stay_one_token() {
        let c = parse_command("tablef \"my table\"");
        assert_eq!(c.arg().as_deref(), Some("my table"));
    }

    #[test]
    fn blank_lines_are_ignored() {
        assert!(parse_command("   ").cmd.is_empty());
    }

    #[test]
    fn resolve_db_spec_accepts_index_and_name() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = rusqlite::Connection::open(tmp.path().join("a.db")).unwrap();
        conn.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
        drop(conn);
        std::fs::write(tmp.path().join("b.txt"), b"x").unwrap();

        assert_eq!(
            resolve_db_spec("1", tmp.path()),
            Some(tmp.path().join("a.db"))
        );
        assert_eq!(
            resolve_db_spec("a.db", tmp.path()),
            Some(tmp.path().join("a.db"))
        );
        assert_eq!(resolve_db_spec("missing.db", tmp.path()), None);
        assert_eq!(resolve_db_spec("9", tmp.path()), None);
        assert_eq!(resolve_db_spec("", tmp.path()), None);
    }

    #[test]
    fn history_path_appends_marker() {
        let p = ndb_history_path(Path::new("/tmp/ntc_history.txt"));
        assert_eq!(
            p.file_name().unwrap().to_string_lossy(),
            "ntc_history.txt.ndb"
        );
    }

    #[test]
    fn clear_stays_at_the_current_level() {
        let tmp = tempfile::tempdir().unwrap();
        let cmd = parse_command("clear");
        assert!(matches!(handle_root(&cmd, tmp.path()), Action::Stay));
        let cls = parse_command("cls");
        assert!(matches!(handle_root(&cls, tmp.path()), Action::Stay));

        let db_path = tmp.path().join("app.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
        drop(conn);
        let mut db = Database::open(&db_path).unwrap();
        let mut page = None;
        assert!(matches!(
            handle_db(&cmd, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert!(matches!(
            handle_db(&cls, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
    }

    #[test]
    fn new_commands_stay_and_page() {
        let tmp = tempfile::tempdir().unwrap();
        let db_path = tmp.path().join("app.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
             INSERT INTO users (name) VALUES ('a'), ('b'), ('c');
             CREATE INDEX idx_users_name ON users (name);",
        )
        .unwrap();
        drop(conn);
        let mut db = Database::open(&db_path).unwrap();
        let mut page = None;

        // `tables` with a pattern that matches nothing still stays.
        let c = parse_command("tables zzz");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        // `info` / `indexes` stay and don't touch paging state.
        for line in ["info", "indexes", "indexes users"] {
            let c = parse_command(line);
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
        }
        assert!(page.is_none());

        // First page of two, then `more` finishes the query.
        let c = parse_command("sql -l 2 SELECT id FROM users ORDER BY id");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(page.as_ref().map(|p| p.shown), Some(2));
        let c = parse_command("more");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(page.as_ref().map(|p| p.shown), Some(3));

        // `more` with no prior query errors but stays.
        let mut empty: Option<LastQuery> = None;
        let c = parse_command("more");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut empty),
            Action::Stay
        ));
        assert!(empty.is_none());
    }

    #[test]
    fn dbal_run_and_bare_name_execute_saved_queries() {
        let _guard = CONFIG_GUARD.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let db_path = tmp.path().join("app.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER);
             INSERT INTO users VALUES (1), (2);",
        )
        .unwrap();
        drop(conn);
        let mut db = Database::open(&db_path).unwrap();

        let mut seed = crate::config::Config::new();
        seed.db_aliases.insert(
            "all_users".to_string(),
            "SELECT id FROM users ORDER BY id".to_string(),
        );
        crate::config::Config::with_test_config(seed, || {
            // Explicit run with a limit, then `more` is armed.
            let mut page = None;
            let c = parse_command("dbal run all_users -l 1");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
            assert_eq!(page.as_ref().map(|p| p.shown), Some(1));

            // Bare alias names run too (case-insensitive), full table here.
            let c = parse_command("ALL_USERS");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
            assert_eq!(page.as_ref().map(|p| p.shown), Some(2));

            // Unknown names still error without touching paging state.
            let mut empty: Option<LastQuery> = None;
            let c = parse_command("nope_missing");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut empty),
                Action::Stay
            ));
            assert!(empty.is_none());
        });
    }

    #[test]
    fn chain_splits_outside_quotes_only() {
        assert_eq!(split_chain("tables"), vec!["tables"]);
        assert_eq!(
            split_chain("tables && schema users"),
            vec!["tables", "schema users"]
        );
        assert_eq!(split_chain("tables&&tables"), vec!["tables", "tables"]);
        // `&&` inside string literals is not a separator ...
        assert_eq!(
            split_chain("sql SELECT 'a && b'"),
            vec!["sql SELECT 'a && b'"]
        );
        assert_eq!(
            split_chain("sql SELECT \"a && b\""),
            vec!["sql SELECT \"a && b\""]
        );
        // ... and empties are dropped.
        assert_eq!(
            split_chain("tables &&  && tables"),
            vec!["tables", "tables"]
        );
        assert!(split_chain("  &&  ").is_empty());
    }

    #[test]
    fn dbal_params_execute_and_injection_is_blocked() {
        let _guard = CONFIG_GUARD.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let db_path = tmp.path().join("app.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER, name TEXT);
             INSERT INTO users VALUES (1, 'ann'), (2, 'bob');",
        )
        .unwrap();
        drop(conn);
        let mut db = Database::open(&db_path).unwrap();

        let mut seed = crate::config::Config::new();
        seed.db_aliases.insert(
            "by_name".to_string(),
            "SELECT id FROM users WHERE name = $name".to_string(),
        );
        seed.db_aliases.insert(
            "recent".to_string(),
            "SELECT id FROM ${t:-users} ORDER BY id".to_string(),
        );
        crate::config::Config::with_test_config(seed, || {
            // Positional value (quoted for SQL strings).
            let mut page = None;
            let c = parse_command("dbal run by_name \"'ann'\"");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
            assert_eq!(page.as_ref().map(|p| p.shown), Some(1));

            // Call syntax + bare-name fallback share the same resolver.
            let mut page = None;
            let c = parse_command("by_name('bob')");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
            assert_eq!(page.as_ref().map(|p| p.shown), Some(1));

            // Defaults apply with no args.
            let mut page = None;
            let c = parse_command("recent");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
            assert_eq!(page.as_ref().map(|p| p.shown), Some(2));

            // Missing values error without running.
            let mut page = None;
            let c = parse_command("dbal run by_name");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
            assert!(page.is_none());

            // `;`-smuggled writes are rejected before execution.
            let mut page = None;
            let c = parse_command("dbal run by_name \"'x'; DELETE FROM users\"");
            assert!(matches!(
                handle_db(&c, &mut db, tmp.path(), &mut page),
                Action::Stay
            ));
            assert!(page.is_none());
            assert_eq!(db.row_count("users").unwrap(), 2);
        });
    }

    #[test]
    fn dbal_rejects_write_bodies_without_storing() {
        let _guard = CONFIG_GUARD.lock().unwrap();
        let seed = crate::config::Config::new();
        crate::config::Config::with_test_config(seed, || {
            let tmp = tempfile::tempdir().unwrap();
            let c = parse_command("dbal add evil \"DELETE FROM users\"");
            assert!(matches!(handle_root(&c, tmp.path()), Action::Stay));
            assert!(dbal::lookup("evil").is_none());

            // Invalid names are rejected the same way.
            let c = parse_command("dbal add tables \"SELECT 1\"");
            assert!(matches!(handle_root(&c, tmp.path()), Action::Stay));
            assert!(dbal::lookup("tables").is_none());
        });
    }

    #[test]
    fn hinter_context_follows_shell_state() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = NdbContext::new();
        let root = NdbState::Root;
        sync_hinter_context(&ctx, tmp.path(), &root);
        assert!(!ctx.in_db());

        let db_path = tmp.path().join("app.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch("CREATE TABLE users (id INTEGER)")
            .unwrap();
        drop(conn);
        let db = Database::open(&db_path).unwrap();
        let state = NdbState::Db(Box::new(db));
        sync_hinter_context(&ctx, tmp.path(), &state);
        assert!(ctx.in_db());
    }

    #[test]
    fn write_keeps_query_verbatim() {
        let c = parse_command("write YES INSERT INTO t VALUES ('a b')");
        assert_eq!(c.cmd, "write");
        assert_eq!(c.query, "YES INSERT INTO t VALUES ('a b')");
    }

    #[test]
    fn write_confirm_prefix_is_detected() {
        assert_eq!(
            split_write_confirm("YES INSERT INTO t VALUES (1)"),
            (true, "INSERT INTO t VALUES (1)")
        );
        assert_eq!(
            split_write_confirm("--yes DELETE FROM t"),
            (true, "DELETE FROM t")
        );
        assert_eq!(
            split_write_confirm("INSERT INTO t VALUES (1)"),
            (false, "INSERT INTO t VALUES (1)")
        );
        assert_eq!(split_write_confirm("YES"), (true, ""));
    }

    #[test]
    fn edit_write_backup_flow() {
        let tmp = tempfile::tempdir().unwrap();
        let db_path = tmp.path().join("app.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
             INSERT INTO users (name) VALUES ('a');",
        )
        .unwrap();
        drop(conn);
        let mut db = Database::open(&db_path).unwrap();
        let mut page = None;

        // Locked by default: `write` refuses without `edit on`, no backup made.
        let c = parse_command("write YES INSERT INTO users (name) VALUES ('b')");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(db.row_count("users").unwrap(), 1);
        assert!(!db.backed_up);

        // `edit on` unlocks; unconfirmed `write` only previews.
        let c = parse_command("edit on");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert!(db.edit_on);
        let c = parse_command("write INSERT INTO users (name) VALUES ('b')");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(db.row_count("users").unwrap(), 1);
        assert!(!db.backed_up, "preview must not back up");

        // Confirmed write commits + auto-backs-up exactly once.
        let c = parse_command("write YES INSERT INTO users (name) VALUES ('b')");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(db.row_count("users").unwrap(), 2);
        assert!(db.backed_up);
        let backups: Vec<_> = std::fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("app.db.bak."))
            .collect();
        assert_eq!(backups.len(), 1);

        // DDL stays blocked even when unlocked.
        let c = parse_command("write YES DROP TABLE users");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(db.row_count("users").unwrap(), 2);

        // `sql` can never write, even with `edit on`.
        let c = parse_command("sql DELETE FROM users");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(db.row_count("users").unwrap(), 2);

        // Second write reuses the single backup.
        let c = parse_command("write YES DELETE FROM users WHERE name = 'b'");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(db.row_count("users").unwrap(), 1);
        let backups: Vec<_> = std::fs::read_dir(tmp.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("app.db.bak."))
            .collect();
        assert_eq!(backups.len(), 1);

        // `edit off` relocks.
        let c = parse_command("edit off");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert!(!db.edit_on);
        let c = parse_command("write YES DELETE FROM users");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
        assert_eq!(db.row_count("users").unwrap(), 1);

        // Manual `backup` works while locked.
        let c = parse_command("backup");
        assert!(matches!(
            handle_db(&c, &mut db, tmp.path(), &mut page),
            Action::Stay
        ));
    }

    #[test]
    fn fresh_handle_starts_locked() {
        let tmp = tempfile::tempdir().unwrap();
        let db_path = tmp.path().join("app.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
        drop(conn);
        let db = Database::open(&db_path).unwrap();
        assert!(!db.edit_on);
        assert!(!db.backed_up);
        assert!(!db.is_writable());
    }
}
