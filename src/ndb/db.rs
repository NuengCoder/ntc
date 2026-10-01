//! SQLite inspection layer for the `ndb` shell.
//!
//! Every connection is opened **read-only**: `ndb` is a viewer, never a
//! mutator. On top of `SQLITE_OPEN_READ_ONLY`, `sql` additionally rejects any
//! statement whose first keyword is not a read-only one (defence in depth).

use anyhow::{Context, Result};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags};
use std::path::{Path, PathBuf};

/// File extensions treated as SQLite database candidates (lowercase).
pub const DB_EXTENSIONS: &[&str] = &["db", "sqlite", "sqlite3"];

/// Default maximum rows rendered for a single query.
pub const DEFAULT_ROW_LIMIT: usize = 50;

/// A database file discovered in a directory.
#[derive(Debug, Clone)]
pub struct DbFile {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    /// `Some(n)` when the file is a readable SQLite database with `n` user
    /// tables; `None` when it is not a database (or cannot be read).
    pub table_count: Option<usize>,
}

impl DbFile {
    pub fn is_database(&self) -> bool {
        self.table_count.is_some()
    }
}

/// List database files directly inside `dir` (non-recursive), sorted by name.
pub fn list_db_files(dir: &Path) -> Vec<DbFile> {
    let mut out: Vec<DbFile> = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return out,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !DB_EXTENSIONS.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
            continue;
        }
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let table_count = count_user_tables(&path);
        out.push(DbFile {
            path,
            name,
            size,
            table_count,
        });
    }
    out.sort_by_cached_key(|f| f.name.to_lowercase());
    out
}

/// Open `path` read-only and count user tables (`sqlite_%` excluded).
/// Returns `None` when the file is not a valid SQLite database.
fn count_user_tables(path: &Path) -> Option<usize> {
    let conn = open_read_only(path).ok()?;
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master \
             WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
            [],
            |r| r.get(0),
        )
        .ok()?;
    Some(count as usize)
}

/// Open a SQLite database in read-only mode.
pub fn open_read_only(path: &Path) -> Result<Connection> {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
    )
    .with_context(|| format!("failed to open database: {}", path.display()))
}

/// One table or view in the database.
#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    /// `"table"` or `"view"`.
    pub kind: String,
    pub columns: usize,
    pub rows: usize,
}

impl TableInfo {
    pub fn is_view(&self) -> bool {
        self.kind == "view"
    }
}

/// One column of a table (`PRAGMA table_info`).
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub col_type: String,
    pub not_null: bool,
    pub primary_key: bool,
}

impl ColumnInfo {
    /// Human-readable constraint suffix (`PK`, `NOT NULL`, both, or empty).
    pub fn constraint(&self) -> String {
        match (self.primary_key, self.not_null) {
            (true, true) => "PK NOT NULL".to_string(),
            (true, false) => "PK".to_string(),
            (false, true) => "NOT NULL".to_string(),
            (false, false) => String::new(),
        }
    }
}

/// Result of running a read-only query.
#[derive(Debug, Clone, Default)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
    /// `true` when more rows were available than the requested limit.
    pub truncated: bool,
    /// Wall-clock time spent executing and collecting the query.
    pub elapsed_ms: u128,
}

/// One index on a table.
#[derive(Debug, Clone)]
pub struct IndexInfo {
    pub name: String,
    pub table: String,
    pub unique: bool,
    /// `CREATE INDEX` SQL (`None` for auto-indexes).
    pub sql: Option<String>,
}

/// One foreign-key clause of a table (`PRAGMA foreign_key_list`).
#[derive(Debug, Clone)]
pub struct ForeignKeyInfo {
    pub id: i64,
    pub seq: i64,
    pub table: String,
    pub from: String,
    pub to: String,
}

/// Summary counts for the `info` command.
#[derive(Debug, Clone, Default)]
pub struct DbInfo {
    pub tables: usize,
    pub views: usize,
    pub indexes: usize,
    pub triggers: usize,
    pub page_size: i64,
    pub page_count: i64,
    pub sqlite_version: String,
}

/// Quote a SQL identifier (table/column name) for safe interpolation.
pub fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Convert a SQLite value to a printable string (`None` == SQL NULL).
pub fn value_to_string(value: ValueRef<'_>) -> Option<String> {
    match value {
        ValueRef::Null => None,
        ValueRef::Integer(i) => Some(i.to_string()),
        ValueRef::Real(f) => Some(format!("{}", f)),
        ValueRef::Text(bytes) => Some(String::from_utf8_lossy(bytes).to_string()),
        ValueRef::Blob(bytes) => Some(format!("<BLOB {} B>", bytes.len())),
    }
}

/// Strip leading whitespace and SQL comments (`--` and `/* */`).
fn strip_leading_comments(mut s: &str) -> &str {
    loop {
        let t = s.trim_start();
        if let Some(rest) = t.strip_prefix("--") {
            match rest.find('\n') {
                Some(pos) => s = &rest[pos + 1..],
                None => return "",
            }
        } else if let Some(rest) = t.strip_prefix("/*") {
            match rest.find("*/") {
                Some(pos) => s = &rest[pos + 2..],
                None => return "",
            }
        } else {
            return t;
        }
    }
}

/// First alphabetic keyword of a statement, uppercased (empty when unknown).
pub fn first_keyword(sql: &str) -> String {
    strip_leading_comments(sql)
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_ascii_uppercase()
}

/// Whether `sql` is a statement `ndb` is willing to execute.
/// Writes are rejected here and again by the read-only connection.
pub fn is_read_only_sql(sql: &str) -> bool {
    matches!(
        first_keyword(sql).as_str(),
        "SELECT" | "PRAGMA" | "EXPLAIN" | "WITH" | "VALUES" | "TABLE"
    )
}

/// Whether `sql` is an explicit write `ndb` may execute via `write`
/// (only when the session enabled `edit on`). Kept deliberately narrow:
/// row-level DML only — no DDL (`CREATE/DROP/ALTER`), no `ATTACH`,
/// no `VACUUM`, no multi-statement smuggling (callers enforce single).
pub fn is_write_sql(sql: &str) -> bool {
    matches!(
        first_keyword(sql).as_str(),
        "INSERT" | "UPDATE" | "DELETE" | "REPLACE"
    )
}

/// Copy `path` next to itself as `<name>.bak.<YYYYMMDD-HHMMSS>`.
/// Returns the backup path. Used before the first write of a session.
pub fn backup_file(path: &Path) -> Result<PathBuf> {
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "database.db".to_string());
    let backup_name = format!("{}.bak.{}", file_name, stamp);
    let dest = path
        .parent()
        .map(|p| p.join(&backup_name))
        .unwrap_or_else(|| PathBuf::from(&backup_name));
    std::fs::copy(path, &dest)
        .with_context(|| format!("failed to back up database to {}", dest.display()))?;
    Ok(dest)
}

/// An open database together with its display name.
///
/// By default the connection is **read-only** (`edit off`). `edit on`
/// reopens it read-write in place via [`Database::make_writable`] so an
/// explicit `write` can commit row-level DML.
pub struct Database {
    pub path: PathBuf,
    pub name: String,
    conn: Connection,
    writable: bool,
    /// Session write gate (`edit on`). Fresh handles start locked.
    pub(crate) edit_on: bool,
    /// Whether the auto-backup copy was already taken for this handle.
    pub(crate) backed_up: bool,
}

impl Database {
    /// Open `path` read-only, verifying that it really is a SQLite database.
    pub fn open(path: &Path) -> Result<Self> {
        let conn = open_read_only(path)?;
        // Touch sqlite_master so non-database files fail here with a helpful
        // message instead of a confusing error later.
        conn.query_row("SELECT count(*) FROM sqlite_master", [], |r| {
            r.get::<_, i64>(0)
        })
        .context("not a readable SQLite database")?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        Ok(Self {
            path: path.to_path_buf(),
            name,
            conn,
            writable: false,
            edit_on: false,
            backed_up: false,
        })
    }

    /// Whether this handle was reopened for writes (`edit on`).
    pub fn is_writable(&self) -> bool {
        self.writable
    }

    /// Reopen the database read-write in place (no `CREATE`: the file must
    /// already exist and be a valid SQLite database).
    pub fn make_writable(&mut self) -> Result<()> {
        if self.writable {
            return Ok(());
        }
        let conn = Connection::open_with_flags(
            &self.path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_URI,
        )
        .with_context(|| format!("failed to open database for writing: {}", self.path.display()))?;
        conn.query_row("SELECT count(*) FROM sqlite_master", [], |r| {
            r.get::<_, i64>(0)
        })
        .context("not a readable SQLite database")?;
        self.conn = conn;
        self.writable = true;
        Ok(())
    }

    /// Execute one row-level write (`INSERT/UPDATE/DELETE/REPLACE`) inside
    /// an explicit transaction. Returns affected rows.
    pub fn exec_write(&self, sql: &str) -> Result<u64> {
        if !self.writable {
            anyhow::bail!("writes are disabled: run `edit on` first");
        }
        if !is_write_sql(sql) {
            anyhow::bail!("only INSERT, UPDATE, DELETE and REPLACE are allowed via `write`");
        }
        self.conn
            .execute_batch("BEGIN IMMEDIATE")
            .context("failed to begin write transaction")?;
        let outcome: Result<u64> = (|| {
            let n = self.conn.execute(sql, [])?;
            Ok(n as u64)
        })();
        match outcome {
            Ok(n) => {
                self.conn
                    .execute_batch("COMMIT")
                    .context("failed to commit write transaction")?;
                Ok(n)
            }
            Err(e) => {
                let _ = self.conn.execute_batch("ROLLBACK");
                Err(e).context("write failed (rolled back)")
            }
        }
    }

    /// Tables and views, sorted by name (case-insensitive).
    pub fn tables(&self) -> Result<Vec<TableInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, type FROM sqlite_master \
             WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%' \
             ORDER BY name COLLATE NOCASE",
        )?;
        let raw: Vec<(String, String)> = stmt
            .query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let mut out = Vec::with_capacity(raw.len());
        for (name, kind) in raw {
            let columns = self.columns(&name).map(|c| c.len()).unwrap_or(0);
            let rows = self.row_count(&name).unwrap_or(0);
            out.push(TableInfo {
                name,
                kind,
                columns,
                rows,
            });
        }
        Ok(out)
    }

    /// Column metadata for `table` (or a view).
    pub fn columns(&self, table: &str) -> Result<Vec<ColumnInfo>> {
        let sql = format!("PRAGMA table_info({})", quote_ident(table));
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([], |r| {
            Ok(ColumnInfo {
                name: r.get::<_, String>(1)?,
                col_type: r.get::<_, String>(2).unwrap_or_default(),
                not_null: r.get::<_, i64>(3).unwrap_or(0) != 0,
                primary_key: r.get::<_, i64>(5).unwrap_or(0) != 0,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    /// Number of rows in `table`.
    pub fn row_count(&self, table: &str) -> Result<usize> {
        let sql = format!("SELECT COUNT(*) FROM {}", quote_ident(table));
        let n: i64 = self.conn.query_row(&sql, [], |r| r.get(0))?;
        Ok(n.max(0) as usize)
    }

    /// `CREATE TABLE`/`CREATE VIEW` SQL for `name`, when available.
    pub fn create_sql(&self, name: &str) -> Option<String> {
        self.conn
            .query_row(
                "SELECT sql FROM sqlite_master WHERE name = ?1 AND type IN ('table', 'view')",
                [name],
                |r| r.get::<_, Option<String>>(0),
            )
            .ok()
            .flatten()
    }

    /// Run a read-only query, collecting at most `limit` rows.
    /// `limit == usize::MAX` collects every row.
    pub fn query(&self, sql: &str, limit: usize) -> Result<QueryResult> {
        let started = std::time::Instant::now();
        let mut stmt = self.conn.prepare(sql)?;
        let columns: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
        let cols = columns.len();
        let mut rows = stmt.query([])?;
        let mut out_rows: Vec<Vec<Option<String>>> = Vec::new();
        let mut truncated = false;
        while let Some(row) = rows.next()? {
            if out_rows.len() >= limit {
                truncated = true;
                break;
            }
            let mut vals = Vec::with_capacity(cols);
            for i in 0..cols {
                vals.push(value_to_string(row.get_ref(i)?));
            }
            out_rows.push(vals);
        }
        Ok(QueryResult {
            columns,
            rows: out_rows,
            truncated,
            elapsed_ms: started.elapsed().as_millis(),
        })
    }

    /// Single-integer `PRAGMA` value (`page_size`, `page_count`, ...).
    fn pragma_int(&self, key: &str) -> i64 {
        let sql = format!("PRAGMA {}", key);
        self.conn
            .query_row(&sql, [], |r| r.get::<_, i64>(0))
            .unwrap_or(0)
    }

    /// Count `sqlite_master` entries of one type (`table`, `view`, ...),
    /// excluding internal `sqlite_%` objects.
    fn count_master(&self, kind: &str) -> usize {
        self.conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = ?1 AND name NOT LIKE 'sqlite_%'",
                [kind],
                |r| r.get::<_, i64>(0),
            )
            .map(|n| n.max(0) as usize)
            .unwrap_or(0)
    }

    /// Number of user tables (excluding views and internal objects).
    pub fn table_count(&self) -> usize {
        self.count_master("table")
    }

    /// Number of views.
    pub fn view_count(&self) -> usize {
        self.count_master("view")
    }

    /// Number of indexes (including auto-indexes).
    pub fn index_count(&self) -> usize {
        self.count_master("index")
    }

    /// Number of triggers.
    pub fn trigger_count(&self) -> usize {
        self.count_master("trigger")
    }

    /// Summary counts for the `info` command.
    pub fn info(&self) -> DbInfo {
        DbInfo {
            tables: self.table_count(),
            views: self.view_count(),
            indexes: self.index_count(),
            triggers: self.trigger_count(),
            page_size: self.pragma_int("page_size"),
            page_count: self.pragma_int("page_count"),
            sqlite_version: self
                .conn
                .query_row("SELECT sqlite_version()", [], |r| {
                    r.get::<_, String>(0)
                })
                .unwrap_or_default(),
        }
    }

    /// Indexes on `table` (`PRAGMA index_list` joined with `sqlite_master`).
    pub fn indexes(&self, table: &str) -> Result<Vec<IndexInfo>> {
        let sql = format!("PRAGMA index_list({})", quote_ident(table));
        let mut stmt = self.conn.prepare(&sql)?;
        let names: Vec<(String, bool)> = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2).unwrap_or(0) != 0,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        drop(stmt);
        let mut out = Vec::with_capacity(names.len());
        for (name, unique) in names {
            let sql: Option<String> = self
                .conn
                .query_row(
                    "SELECT sql FROM sqlite_master WHERE type = 'index' AND name = ?1",
                    [&name],
                    |r| r.get::<_, Option<String>>(0),
                )
                .ok()
                .flatten();
            out.push(IndexInfo {
                name,
                table: table.to_string(),
                unique,
                sql,
            });
        }
        Ok(out)
    }

    /// Every index in the database, grouped query-free via `sqlite_master`.
    pub fn all_indexes(&self) -> Result<Vec<IndexInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, tbl_name, sql FROM sqlite_master \
             WHERE type = 'index' AND name NOT LIKE 'sqlite_%' \
             ORDER BY tbl_name COLLATE NOCASE, name COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, Option<String>>(2)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows.collect::<rusqlite::Result<Vec<_>>>()? {
            let (name, table, sql) = row;
            // `sqlite_master` does not record uniqueness; look it up.
            let unique = self
                .indexes(&table)
                .map(|list| list.iter().any(|i| i.name == name && i.unique))
                .unwrap_or(false);
            out.push(IndexInfo {
                name,
                table,
                unique,
                sql,
            });
        }
        Ok(out)
    }

    /// Foreign-key clauses of `table` (`PRAGMA foreign_key_list`).
    pub fn foreign_keys(&self, table: &str) -> Result<Vec<ForeignKeyInfo>> {
        let sql = format!("PRAGMA foreign_key_list({})", quote_ident(table));
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([], |r| {
            Ok(ForeignKeyInfo {
                id: r.get::<_, i64>(0).unwrap_or(0),
                seq: r.get::<_, i64>(1).unwrap_or(0),
                table: r.get::<_, String>(2).unwrap_or_default(),
                from: r.get::<_, String>(3).unwrap_or_default(),
                to: r.get::<_, String>(4).unwrap_or_default(),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    /// `CREATE TRIGGER` SQL for triggers on `table`.
    pub fn triggers(&self, table: &str) -> Vec<String> {
        let mut stmt = match self.conn.prepare(
            "SELECT sql FROM sqlite_master \
             WHERE type = 'trigger' AND tbl_name = ?1 ORDER BY name COLLATE NOCASE",
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        stmt.query_map([table], |r| r.get::<_, Option<String>>(0))
            .map(|rows| {
                rows
                    .filter_map(|r| r.ok().flatten())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_db(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, score REAL);
             INSERT INTO users (name, score) VALUES ('ann', 1.5), ('bob', NULL);
             CREATE TABLE notes (body TEXT);
             CREATE VIEW top AS SELECT name FROM users WHERE score IS NOT NULL;",
        )
        .unwrap();
        path
    }

    #[test]
    fn read_only_sql_accepts_queries() {
        assert!(is_read_only_sql("SELECT * FROM users"));
        assert!(is_read_only_sql("  select 1"));
        assert!(is_read_only_sql("PRAGMA table_info(users)"));
        assert!(is_read_only_sql("EXPLAIN SELECT 1"));
        assert!(is_read_only_sql("-- comment\nSELECT 1"));
        assert!(is_read_only_sql("/* block */ SELECT 1"));
        assert!(is_read_only_sql("WITH a AS (SELECT 1) SELECT * FROM a"));
        assert!(is_read_only_sql("VALUES (1)"));
    }

    #[test]
    fn read_only_sql_rejects_writes() {
        for sql in [
            "INSERT INTO users (name) VALUES ('x')",
            "UPDATE users SET name = 'x'",
            "DELETE FROM users",
            "DROP TABLE users",
            "ALTER TABLE users ADD COLUMN z TEXT",
            "CREATE TABLE t2 (a INTEGER)",
            "VACUUM",
            "ATTACH DATABASE 'other.db' AS other",
            "",
        ] {
            assert!(!is_read_only_sql(sql), "should be rejected: {sql}");
        }
    }

    #[test]
    fn first_keyword_handles_comments() {
        assert_eq!(first_keyword("-- hi\nselect 1"), "SELECT");
        assert_eq!(first_keyword("/* hi */ pragma foo"), "PRAGMA");
        assert_eq!(first_keyword("-- unterminated"), "");
    }

    #[test]
    fn value_to_string_covers_all_types() {
        assert_eq!(value_to_string(ValueRef::Null), None);
        assert_eq!(value_to_string(ValueRef::Integer(7)), Some("7".to_string()));
        assert_eq!(
            value_to_string(ValueRef::Real(2.25)),
            Some("2.25".to_string())
        );
        assert_eq!(
            value_to_string(ValueRef::Text(b"hi")),
            Some("hi".to_string())
        );
        assert_eq!(
            value_to_string(ValueRef::Blob(&[1, 2, 3])),
            Some("<BLOB 3 B>".to_string())
        );
    }

    #[test]
    fn quote_ident_escapes_quotes() {
        assert_eq!(quote_ident("users"), "\"users\"");
        assert_eq!(quote_ident("we\"ird"), "\"we\"\"ird\"");
    }

    fn make_rich_db(dir: &Path) -> PathBuf {
        let path = dir.join("rich.db");
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE authors (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
             CREATE TABLE books (
               id INTEGER PRIMARY KEY,
               author_id INTEGER NOT NULL REFERENCES authors (id),
               title TEXT
             );
             CREATE INDEX idx_books_author ON books (author_id);
             CREATE TRIGGER trg_books_insert AFTER INSERT ON books BEGIN
               UPDATE books SET title = NEW.title WHERE id = NEW.id;
             END;",
        )
        .unwrap();
        path
    }

    #[test]
    fn info_counts_objects() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_rich_db(tmp.path());
        let db = Database::open(&path).unwrap();
        let info = db.info();
        assert_eq!(info.tables, 2);
        assert_eq!(info.views, 0);
        assert_eq!(info.indexes, 1, "INTEGER PRIMARY KEY needs no auto-index");
        assert_eq!(info.triggers, 1);
        assert!(info.page_size > 0);
        assert!(info.page_count > 0);
        assert!(!info.sqlite_version.is_empty());
    }

    #[test]
    fn indexes_lists_explicit_and_auto() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_rich_db(tmp.path());
        let db = Database::open(&path).unwrap();

        let on_books = db.indexes("books").unwrap();
        assert!(on_books.iter().any(|i| i.name == "idx_books_author" && i.unique == false));
        assert!(
            on_books
                .iter()
                .find(|i| i.name == "idx_books_author")
                .and_then(|i| i.sql.clone())
                .unwrap_or_default()
                .contains("CREATE INDEX")
        );

        let all = db.all_indexes().unwrap();
        // No sqlite_% internals; user index present.
        assert!(all.iter().any(|i| i.name == "idx_books_author"));
        assert!(all.iter().all(|i| !i.name.starts_with("sqlite_")));
    }

    #[test]
    fn foreign_keys_and_triggers_are_reported() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_rich_db(tmp.path());
        let db = Database::open(&path).unwrap();

        let fks = db.foreign_keys("books").unwrap();
        assert_eq!(fks.len(), 1);
        assert_eq!(fks[0].table, "authors");
        assert_eq!(fks[0].from, "author_id");
        assert_eq!(fks[0].to, "id");
        assert!(db.foreign_keys("authors").unwrap().is_empty());

        let triggers = db.triggers("books");
        assert_eq!(triggers.len(), 1);
        assert!(triggers[0].contains("TRIGGER"));
        assert!(db.triggers("authors").is_empty());
    }

    #[test]
    fn lists_and_reads_a_real_database() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_db(tmp.path(), "app.db");
        std::fs::write(tmp.path().join("notes.txt"), b"not a db").unwrap();

        let files = list_db_files(tmp.path());
        assert_eq!(files.len(), 1, "only .db files should be listed");
        assert_eq!(files[0].name, "app.db");
        assert!(files[0].is_database());
        // `users` + `notes` (views are not counted as tables)
        assert_eq!(files[0].table_count, Some(2));

        let db = Database::open(&path).unwrap();
        assert_eq!(db.name, "app.db");
        let tables = db.tables().unwrap();
        let names: Vec<&str> = tables.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, vec!["notes", "top", "users"]);
        assert!(tables.iter().any(|t| t.name == "top" && t.is_view()));

        assert_eq!(db.row_count("users").unwrap(), 2);
        let cols = db.columns("users").unwrap();
        assert_eq!(cols.len(), 3);
        assert!(cols[0].primary_key);
        assert_eq!(cols[0].constraint(), "PK");
        assert!(cols[1].not_null);

        assert!(db.create_sql("users").unwrap().contains("CREATE TABLE"));
        assert!(db.create_sql("nope").is_none());
    }

    #[test]
    fn query_collects_rows_and_truncates() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_db(tmp.path(), "app.db");
        let db = Database::open(&path).unwrap();

        let all = db.query("SELECT id, name, score FROM users ORDER BY id", 10).unwrap();
        assert_eq!(all.columns, vec!["id", "name", "score"]);
        assert_eq!(all.rows.len(), 2);
        assert!(!all.truncated);
        assert_eq!(all.rows[1][2], None, "NULL should stay NULL");

        let limited = db.query("SELECT id FROM users", 1).unwrap();
        assert_eq!(limited.rows.len(), 1);
        assert!(limited.truncated);
    }

    #[test]
    fn non_database_file_is_reported_and_read_only() {
        let tmp = tempfile::tempdir().unwrap();
        let bogus = tmp.path().join("bogus.db");
        std::fs::write(&bogus, b"definitely not sqlite").unwrap();

        let files = list_db_files(tmp.path());
        assert_eq!(files.len(), 1);
        assert!(!files[0].is_database());
        assert_eq!(files[0].table_count, None);

        assert!(Database::open(&bogus).is_err());
    }

    #[test]
    fn write_statements_fail_on_read_only_connection() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_db(tmp.path(), "app.db");
        let db = Database::open(&path).unwrap();
        // Even if the keyword guard were bypassed, SQLite refuses the write.
        assert!(db.query("DELETE FROM users", 10).is_err());
        assert!(db.query("INSERT INTO users (name) VALUES ('x')", 10).is_err());
    }

    #[test]
    fn write_gate_allows_dml_only() {
        assert!(is_write_sql("INSERT INTO users (name) VALUES ('x')"));
        assert!(is_write_sql("  update users SET name='x'"));
        assert!(is_write_sql("DELETE FROM users"));
        assert!(is_write_sql("REPLACE INTO users (id) VALUES (1)"));
        assert!(is_write_sql("-- c\nDELETE FROM users"));
        for sql in [
            "SELECT 1",
            "DROP TABLE users",
            "ALTER TABLE users ADD COLUMN z TEXT",
            "CREATE TABLE t2 (a INTEGER)",
            "VACUUM",
            "ATTACH DATABASE 'o.db' AS o",
            "",
        ] {
            assert!(!is_write_sql(sql), "should be rejected: {sql}");
        }
    }

    #[test]
    fn exec_write_needs_writable_and_commits() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_db(tmp.path(), "app.db");
        let mut db = Database::open(&path).unwrap();
        assert!(!db.is_writable());
        assert!(db.exec_write("INSERT INTO users (name) VALUES ('x')").is_err());

        db.make_writable().unwrap();
        assert!(db.is_writable());
        // DDL still rejected even when writable.
        assert!(db.exec_write("DROP TABLE users").is_err());

        let n = db
            .exec_write("INSERT INTO users (name) VALUES ('zz')")
            .unwrap();
        assert_eq!(n, 1);
        assert_eq!(db.row_count("users").unwrap(), 3);

        let n = db.exec_write("UPDATE users SET name='q' WHERE name='zz'").unwrap();
        assert_eq!(n, 1);
        let n = db.exec_write("DELETE FROM users WHERE name='q'").unwrap();
        assert_eq!(n, 1);
        assert_eq!(db.row_count("users").unwrap(), 2);
    }

    #[test]
    fn backup_file_copies_next_to_original() {
        let tmp = tempfile::tempdir().unwrap();
        let path = make_db(tmp.path(), "app.db");
        let dest = backup_file(&path).unwrap();
        assert!(dest.exists());
        assert_eq!(
            dest.parent().unwrap(),
            path.parent().unwrap(),
            "backup must sit next to the original"
        );
        assert!(dest
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("app.db.bak."));
    }
}