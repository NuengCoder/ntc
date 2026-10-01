//! `dbal` — saved read-only SQL queries for the `ndb` shell.
//!
//! The third alias dimension: `qal` lives in the editor (session-only text
//! queries), `ral` lives in the ntc shell (system commands), `dbal` lives
//! here (named SQL, stored global + local `ntconfig.toml`, local wins).
//!
//! Bodies execute through [`run_sql`](super::run_sql), so the read-only gate
//! applies to aliases exactly as it does to typed queries.

use super::db::{self, Database};
use super::render::{self, format_box_table};
use super::{print_table_box, run_sql, Action, LastQuery, NdbCommand};
use crate::config::Config;
use crate::output::print_error;
use crate::shell::alias::{extract_param_names, parse_call_syntax, substitute_params};
use crate::t;
use crate::utils::terminal::asciify;

use colored::*;

/// `dbal` subcommands (also used for Tab completion).
pub(crate) const SUBCOMMANDS: &[&str] = &[
    "add", "edit", "rnm", "rename", "rm", "remove", "info", "list", "ls", "run", "cls", "clear",
    "help", "h",
];

/// Alias names must not collide with `ndb` builtins (bare names execute,
/// and builtins always win) nor contain whitespace/quotes.
fn validate_dbal_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    if name
        .chars()
        .any(|c| c.is_whitespace() || c == '"' || c == '\'')
    {
        return false;
    }
    if name.starts_with('-') || name.starts_with('@') || name.starts_with('#') {
        return false;
    }
    let lower = name.to_lowercase();
    for word in super::hinter::ROOT_COMMANDS
        .iter()
        .chain(super::hinter::DB_COMMANDS.iter())
        .chain(SUBCOMMANDS.iter())
    {
        if lower == *word {
            return false;
        }
    }
    true
}

/// Look up an alias body by name (case-insensitive), if one is saved.
pub(crate) fn lookup(name: &str) -> Option<String> {
    Config::get_db_aliases()
        .get(&name.to_lowercase())
        .cloned()
}

/// Split SQL on `;` outside single/double quotes (`''` is an escaped quote).
/// Returns the non-empty trimmed pieces.
fn split_statements(sql: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut in_single = false;
    let mut in_double = false;
    let chars: Vec<(usize, char)> = sql.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (pos, ch) = chars[i];
        match ch {
            '\'' if !in_double => {
                // `''` inside a string is an escaped quote, not a boundary.
                if in_single
                    && i + 1 < chars.len()
                    && chars[i + 1].1 == '\''
                {
                    i += 1;
                } else {
                    in_single = !in_single;
                }
            }
            '"' if !in_single => in_double = !in_double,
            ';' if !in_single && !in_double => {
                let piece = sql[start..pos].trim();
                if !piece.is_empty() {
                    parts.push(piece);
                }
                start = pos + 1;
            }
            _ => {}
        }
        i += 1;
    }
    let tail = sql[start..].trim();
    if !tail.is_empty() {
        parts.push(tail);
    }
    parts
}

/// Fill `$x` / `${x}` / `${x:-default}` in `template` (ral-style, positional).
/// Returns the final SQL, or the names of still-missing parameters.
/// Value-introduced `$` refs are ignored — only template-declared names count.
fn resolve_body(template: &str, args: &[&str]) -> Result<String, Vec<String>> {
    let filled = substitute_params(template, args);
    let declared = extract_param_names(template);
    if declared.is_empty() {
        return Ok(filled);
    }
    let leftover = extract_param_names(&filled);
    let missing: Vec<String> = declared
        .into_iter()
        .filter(|p| leftover.contains(p))
        .collect();
    if missing.is_empty() {
        Ok(filled)
    } else {
        Err(missing)
    }
}

/// Split a `name(a, b)` call suffix into values (commas separate them).
fn split_call_args(call: &str) -> Vec<String> {
    call.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Where an entry landed, for the `(local)` / `(global)` suffix.
fn scope_label() -> String {
    if Config::db_aliases_are_local() {
        t!("ndb.dbal_scope_local")
    } else {
        t!("ndb.dbal_scope_global")
    }
}

/// Strip one pair of surrounding double quotes (mirrors `ral add`).
fn unquote(s: &str) -> &str {
    let s = s.trim();
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// Print the `dbal` usage listing.
fn print_usage() {
    println!();
    println!("{}", t!("ndb.dbal_title").cyan().bold());
    for key in [
        "ndb.dbal_add",
        "ndb.dbal_edit",
        "ndb.dbal_rnm",
        "ndb.dbal_rm",
        "ndb.dbal_info",
        "ndb.dbal_list",
        "ndb.dbal_run",
        "ndb.dbal_cls",
        "ndb.dbal_exec",
    ] {
        println!("  {}", t!(key));
    }
    println!();
    println!("{}", t!("ndb.dbal_note").dimmed());
    println!();
}

/// Execute a saved alias body against the open database.
/// Bodies hold exactly one statement (`&&` sequences commands in the shell,
/// not inside an alias) — anything else is rejected before it can run.
/// Returns paging state for `more` on success.
fn exec_alias(
    sql: &str,
    db: Option<&Database>,
    limit: usize,
    page: &mut Option<LastQuery>,
) -> Action {
    let mut statements = split_statements(sql);
    if statements.len() != 1 {
        print_error(&t!("ndb.dbal_single_only"));
        return Action::Stay;
    }
    let stmt = statements.pop().unwrap_or("");
    match db {
        Some(d) => {
            *page = run_sql(d, stmt, limit);
            Action::Stay
        }
        None => {
            print_error(&t!("ndb.sql_needs_db"));
            Action::Stay
        }
    }
}

/// Resolve a template with `args` and execute it.
fn run_template(
    template: &str,
    args: &[&str],
    db: Option<&Database>,
    limit: usize,
    page: &mut Option<LastQuery>,
) -> Action {
    match resolve_body(template, args) {
        Err(missing) => {
            print_error(&t!("ndb.dbal_missing_args", missing.join(", ")));
            Action::Stay
        }
        Ok(sql) => exec_alias(&sql, db, limit, page),
    }
}

/// Try bare-name execution (`recent 2026-01-01`, `recent('2026-01-01')`).
/// Returns `None` when `cmd_word` names no saved alias.
pub(crate) fn try_bare(
    cmd_word: &str,
    extra_args: &[String],
    db: Option<&Database>,
    limit: usize,
    page: &mut Option<LastQuery>,
) -> Option<Action> {
    let (base, call) = parse_call_syntax(cmd_word);
    let template = lookup(base)?;
    let owned: Vec<String> = match call {
        Some(c) => split_call_args(c),
        None => extra_args.to_vec(),
    };
    let args: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
    Some(run_template(&template, &args, db, limit, page))
}

/// `dbal ...` — manage and run saved queries.
/// `db` is `None` at the root level (management works, execution needs a DB).
pub(crate) fn handle_dbal(
    command: &NdbCommand,
    db: Option<&Database>,
    page: &mut Option<LastQuery>,
) -> Action {
    let Some(sub) = command.args.first().map(|s| s.to_lowercase()) else {
        print_usage();
        return Action::Stay;
    };
    let rest = &command.args[1..];

    match sub.as_str() {
        "help" | "h" => {
            print_usage();
            Action::Stay
        }
        "add" => {
            if rest.len() < 2 {
                print_error(&t!("ndb.dbal_add_usage"));
                return Action::Stay;
            }
            let name = &rest[0];
            if !validate_dbal_name(name) {
                print_error(&t!("ndb.dbal_invalid_name", name));
                println!("{}", t!("ndb.dbal_name_rules").dimmed());
                return Action::Stay;
            }
            let joined = rest[1..].join(" ");
            let sql = unquote(&joined);
            if !db::is_read_only_sql(sql) {
                print_error(&t!("ndb.dbal_blocked"));
                return Action::Stay;
            }
            match Config::add_db_alias(name, sql) {
                Ok(_) => println!("{}", t!("ndb.dbal_added", name, scope_label()).green()),
                Err(e) => print_error(&t!("ndb.query_failed", e)),
            }
            Action::Stay
        }
        "edit" => {
            if rest.len() < 2 {
                print_error(&t!("ndb.dbal_edit_usage"));
                return Action::Stay;
            }
            let name = &rest[0];
            let joined = rest[1..].join(" ");
            let sql = unquote(&joined);
            if !db::is_read_only_sql(sql) {
                print_error(&t!("ndb.dbal_blocked"));
                return Action::Stay;
            }
            match Config::update_db_alias(name, sql) {
                Ok(true) => println!("{}", t!("ndb.dbal_updated", name).green()),
                Ok(false) => print_error(&t!("ndb.dbal_not_found", name)),
                Err(e) => print_error(&t!("ndb.query_failed", e)),
            }
            Action::Stay
        }
        "rnm" | "rename" => {
            if rest.len() != 3 || !rest[1].eq_ignore_ascii_case("to") {
                print_error(&t!("ndb.dbal_rnm_usage"));
                return Action::Stay;
            }
            if Config::rename_db_alias(&rest[0], &rest[2]) {
                println!("{}", t!("ndb.dbal_renamed", rest[0], rest[2]).green());
            } else {
                print_error(&t!("ndb.dbal_not_found", rest[0]));
            }
            Action::Stay
        }
        "rm" | "remove" => {
            let Some(name) = rest.first() else {
                print_error(&t!("ndb.dbal_rm_usage"));
                return Action::Stay;
            };
            match Config::remove_db_alias(name) {
                Ok(true) => println!("{}", t!("ndb.dbal_removed", name).green()),
                Ok(false) => print_error(&t!("ndb.dbal_not_found", name)),
                Err(e) => print_error(&t!("ndb.query_failed", e)),
            }
            Action::Stay
        }
        "info" => {
            let Some(name) = rest.first() else {
                print_error(&t!("ndb.dbal_info_usage"));
                return Action::Stay;
            };
            match lookup(name) {
                Some(sql) => {
                    println!("'{}' -> {}", name, sql.cyan());
                    let params = extract_param_names(&sql);
                    if !params.is_empty() {
                        println!("  {}", t!("ndb.dbal_params", params.join(", ")).dimmed());
                    }
                }
                None => print_error(&t!("ndb.dbal_not_found", name)),
            }
            Action::Stay
        }
        "list" | "ls" => {
            let mut aliases: Vec<(String, String)> =
                Config::get_db_aliases().into_iter().collect();
            aliases.sort_by(|a, b| a.0.cmp(&b.0));
            println!();
            if aliases.is_empty() {
                println!("{}", t!("ndb.dbal_no_aliases").yellow());
                return Action::Stay;
            }
            let headers = vec![t!("ndb.col_name"), t!("ndb.col_sql")];
            let rows: Vec<Vec<Option<String>>> = aliases
                .iter()
                .map(|(n, q)| vec![Some(n.clone()), Some(q.clone())])
                .collect();
            print_table_box(&asciify(&format_box_table(
                &headers,
                &rows,
                render::MAX_CELL_WIDTH,
            )));
            println!();
            Action::Stay
        }
        "run" => {
            let Some(first) = rest.first() else {
                print_error(&t!("ndb.dbal_run_usage"));
                return Action::Stay;
            };
            // `dbal run q('2026-01-01')` or `dbal run q 2026-01-01`.
            let (base, call) = parse_call_syntax(first);
            match lookup(base) {
                Some(template) => {
                    let owned: Vec<String> = match call {
                        Some(c) => split_call_args(c),
                        None => rest[1..].to_vec(),
                    };
                    let args: Vec<&str> =
                        owned.iter().map(|s| s.as_str()).collect();
                    run_template(&template, &args, db, command.row_limit(), page)
                }
                None => {
                    print_error(&t!("ndb.dbal_not_found", base));
                    Action::Stay
                }
            }
        }
        "cls" | "clear" => {
            let force = rest
                .iter()
                .any(|a| a == "--force" || a == "-f");
            if !force {
                print_error(&t!("ndb.dbal_cls_usage"));
                return Action::Stay;
            }
            match Config::clear_db_aliases() {
                Ok(n) => println!("{}", t!("ndb.dbal_cleared", n, scope_label()).green()),
                Err(e) => print_error(&t!("ndb.query_failed", e)),
            }
            Action::Stay
        }
        _ => {
            print_error(&t!("ndb.dbal_unknown_sub", sub));
            print_usage();
            Action::Stay
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{resolve_body, split_statements, validate_dbal_name};

    #[test]
    fn statements_split_outside_quotes_only() {
        assert_eq!(split_statements("SELECT 1"), vec!["SELECT 1"]);
        assert_eq!(split_statements("SELECT 1;"), vec!["SELECT 1"]);
        assert_eq!(
            split_statements("SELECT 1; SELECT 2;"),
            vec!["SELECT 1", "SELECT 2"]
        );
        // `;` inside string literals is not a boundary ...
        assert_eq!(
            split_statements("SELECT * FROM t WHERE a = 'x;y'"),
            vec!["SELECT * FROM t WHERE a = 'x;y'"]
        );
        // ... and `''` is an escaped quote, not a boundary toggle.
        assert_eq!(
            split_statements("SELECT 'it''s; ok'"),
            vec!["SELECT 'it''s; ok'"]
        );
        assert!(split_statements("  ;  ").is_empty());
    }

    #[test]
    fn params_fill_positionally_with_defaults() {
        // Plain scalar + braced forms.
        let out = resolve_body("SELECT * FROM t WHERE a = $a AND b = ${b}", &["1", "2"]);
        assert_eq!(
            out,
            Ok("SELECT * FROM t WHERE a = 1 AND b = 2".to_string())
        );
        // Defaults apply when no value is given ...
        let out = resolve_body("SELECT * FROM ${t:-users}", &[]);
        assert_eq!(out, Ok("SELECT * FROM users".to_string()));
        // ... and explicit values beat defaults.
        let out = resolve_body("SELECT * FROM ${t:-users}", &["logs"]);
        assert_eq!(out, Ok("SELECT * FROM logs".to_string()));
        // Missing values are reported by name.
        assert_eq!(
            resolve_body("SELECT $a, $b", &["1"]),
            Err(vec!["b".to_string()])
        );
        // `$` introduced by a *value* is not treated as a parameter.
        let out = resolve_body("SELECT * FROM t WHERE a = $a", &["$5"]);
        assert_eq!(out, Ok("SELECT * FROM t WHERE a = $5".to_string()));
        // Templates without params pass through untouched.
        let out = resolve_body("SELECT 1", &[]);
        assert_eq!(out, Ok("SELECT 1".to_string()));
    }

    #[test]
    fn names_must_not_shadow_builtins_or_contain_spaces() {
        assert!(validate_dbal_name("top_users"));
        assert!(validate_dbal_name("q1"));
        for bad in [
            "",
            "with space",
            "\"quoted\"",
            "-flag",
            "@x",
            "tables",
            "TABLEF",
            "sql",
            "dbal",
            "run",
            "exit",
            "clear",
            "info",
            "more",
            "edit",
            "write",
            "backup",
            "bak",
        ] {
            assert!(!validate_dbal_name(bad), "should be rejected: {bad}");
        }
    }
}
