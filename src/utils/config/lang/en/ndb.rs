use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ndb.banner".into(), "ndb {} - NTC Database shell".into());

    m.insert("ndb.cwd_line".into(), "cwd: {}".into());

    m.insert(
        "ndb.no_db_files".into(),
        "No .db files found in the current directory.".into(),
    );

    m.insert(
        "ndb.no_db_files_hint".into(),
        "Add a database file here, or cd into a folder that has one.".into(),
    );

    m.insert(
        "ndb.db_files_header".into(),
        "Available databases ({}):".into(),
    );

    m.insert("ndb.tables_count".into(), "{} tables".into());

    m.insert("ndb.not_a_db".into(), "not a database".into());

    m.insert("ndb.pick_db".into(), "Select a database to OPEN".into());

    m.insert("ndb.pick_table".into(), "Select a table to DISPLAY".into());

    m.insert(
        "ndb.pick_prompt".into(),
        "Enter number (1-{}) or 0: ".into(),
    );

    m.insert("ndb.open_failed".into(), "Failed to open '{}': {}".into());

    m.insert(
        "ndb.opened".into(),
        "Opened database: {} (read-only)".into(),
    );

    m.insert(
        "ndb.hint_next".into(),
        "Use 'tables', 'viewf', 'tablef <table>', 'schema <table>', 'sql <query>', 'exit'.".into(),
    );

    m.insert(
        "ndb.no_tables".into(),
        "This database has no tables.".into(),
    );

    m.insert("ndb.query_failed".into(), "Query failed: {}".into());

    m.insert("ndb.unknown_cmd".into(), "Unknown ndb command: '{}'".into());

    m.insert(
        "ndb.hint_help".into(),
        "Type 'help' to list every ndb command.".into(),
    );

    m.insert(
        "ndb.interface_usage".into(),
        "Usage: interface <file.db>  (or 'view' to list databases)".into(),
    );

    m.insert(
        "ndb.sql_needs_db".into(),
        "Open a database first: interface <file.db>".into(),
    );

    m.insert(
        "ndb.sql_usage".into(),
        "Usage: sql <query>   e.g. sql SELECT * FROM users".into(),
    );

    m.insert(
        "ndb.sql_blocked".into(),
        "ndb is read-only: only SELECT, PRAGMA, EXPLAIN, WITH and VALUES are allowed.".into(),
    );

    m.insert("ndb.sql_error".into(), "SQL error: {}".into());

    m.insert("ndb.table_not_found".into(), "Table not found: {}".into());

    m.insert("ndb.schema_header".into(), "Schema for '{}':".into());

    m.insert("ndb.schema_usage".into(), "Usage: schema <table>".into());

    m.insert("ndb.showing_table".into(), "Table: {} (read-only)".into());

    m.insert(
        "ndb.editor_fail".into(),
        "Failed to create line editor: {}".into(),
    );

    m.insert(
        "ndb.start_hint".into(),
        "Type 'help' for commands, 'exit' to return to the ntc shell.".into(),
    );

    m.insert("ndb.closed_db".into(), "Database closed.".into());

    m.insert("ndb.bye".into(), "Left ndb shell.".into());

    m.insert(
        "ndb.no_args".into(),
        "Usage: ndb   (opens the ndb database shell)".into(),
    );

    m.insert("ndb.null".into(), "NULL".into());

    m.insert("ndb.kind_view".into(), "view".into());

    m.insert("ndb.kind_table".into(), "table".into());

    m.insert("ndb.columns_count".into(), "{} columns".into());

    m.insert("ndb.rows_count".into(), "{} rows".into());

    m.insert("ndb.rows_empty".into(), "0 rows".into());

    m.insert("ndb.rows_shown".into(), "{} row(s) shown".into());

    m.insert(
        "ndb.rows_truncated".into(),
        "Showing the first {} row(s) - use --all or -l <n> for more.".into(),
    );

    m.insert(
        "ndb.no_columns".into(),
        "Statement returned no columns.".into(),
    );

    m.insert("ndb.col_name".into(), "NAME".into());

    m.insert("ndb.col_kind".into(), "KIND".into());

    m.insert("ndb.col_columns".into(), "COLUMNS".into());

    m.insert("ndb.col_rows".into(), "ROWS".into());

    m.insert("ndb.col_type".into(), "TYPE".into());

    m.insert("ndb.col_constraints".into(), "CONSTRAINTS".into());

    m.insert(
        "ndb.help_root_title".into(),
        "NDB - DATABASE SHELL (no database open):".into(),
    );

    m.insert(
        "ndb.help_root_help".into(),
        "help | h | ?               Show this command list".into(),
    );

    m.insert(
        "ndb.help_root_view".into(),
        "view | ls                  List .db files in the current directory".into(),
    );

    m.insert(
        "ndb.help_root_interface".into(),
        "interface <file.db>       Open a database (prompt becomes ndb [file.db]>)".into(),
    );

    m.insert(
        "ndb.help_root_sql".into(),
        "sql                        Run queries once a database is open (interface first)".into(),
    );

    m.insert(
        "ndb.help_root_clear".into(),
        "clear | cls                Clear screen and re-show databases".into(),
    );

    m.insert(
        "ndb.help_root_exit".into(),
        "exit | q                   Return to the ntc shell".into(),
    );

    m.insert(
        "ndb.help_db_title".into(),
        "NDB - DATABASE SHELL (read-only):".into(),
    );

    m.insert(
        "ndb.help_db_help".into(),
        "help | h | ?               Show this command list".into(),
    );

    m.insert(
        "ndb.help_db_viewf".into(),
        "viewf | v                  Show the schema tree (tables + columns)".into(),
    );

    m.insert(
        "ndb.help_db_tables".into(),
        "tables [pattern]          List table and view names (optional filter)".into(),
    );

    m.insert(
        "ndb.help_db_tablef".into(),
        "tablef <table>             Display rows of a table (like txtf)".into(),
    );

    m.insert(
        "ndb.help_db_schema".into(),
        "schema <table>             Show CREATE SQL, columns, indexes, keys".into(),
    );

    m.insert(
        "ndb.help_db_sql".into(),
        "sql <query>                Run a read-only query (-l N, --all)".into(),
    );

    m.insert(
        "ndb.help_db_write".into(),
        "write YES <stmt>           Run one write (INSERT/UPDATE/DELETE, needs edit on)".into(),
    );

    m.insert(
        "ndb.help_db_edit".into(),
        "edit on|off               Allow or block writes for this database".into(),
    );

    m.insert(
        "ndb.help_db_backup".into(),
        "backup                     Copy the .db file next to itself".into(),
    );

    m.insert(
        "ndb.help_db_more".into(),
        "more | next              Show the next page of the last query".into(),
    );

    m.insert(
        "ndb.help_db_indexes".into(),
        "indexes [table]          List indexes (all, or for one table)".into(),
    );

    m.insert(
        "ndb.help_db_info".into(),
        "info                     Show file, engine and object counts".into(),
    );

    m.insert(
        "ndb.help_db_interface".into(),
        "interface <file.db>       Switch to another database".into(),
    );

    m.insert("ndb.info_title".into(), "Database info: {}".into());

    m.insert("ndb.col_property".into(), "PROPERTY".into());

    m.insert("ndb.col_value".into(), "VALUE".into());

    m.insert("ndb.info_file".into(), "File".into());

    m.insert("ndb.info_size".into(), "Size".into());

    m.insert("ndb.info_sqlite".into(), "SQLite".into());

    m.insert("ndb.info_page".into(), "Page size".into());

    m.insert("ndb.info_pages".into(), "Pages".into());

    m.insert("ndb.info_tables".into(), "Tables".into());

    m.insert("ndb.info_views".into(), "Views".into());

    m.insert("ndb.info_indexes".into(), "Indexes".into());

    m.insert("ndb.info_triggers".into(), "Triggers".into());

    m.insert("ndb.indexes_title".into(), "Indexes ({}):".into());

    m.insert("ndb.no_indexes".into(), "No indexes found.".into());

    m.insert("ndb.col_table".into(), "TABLE".into());

    m.insert("ndb.col_unique".into(), "UNIQUE".into());

    m.insert("ndb.col_from".into(), "FROM".into());

    m.insert("ndb.col_to".into(), "TO".into());

    m.insert("ndb.fkeys_title".into(), "Foreign keys:".into());

    m.insert("ndb.triggers_title".into(), "Triggers:".into());

    m.insert("ndb.query_time".into(), "query took {} ms".into());

    m.insert("ndb.more_empty".into(), "No more rows.".into());

    m.insert(
        "ndb.no_query_yet".into(),
        "Run 'sql <query>' or 'tablef <table>' first, then 'more'.".into(),
    );

    m.insert(
        "ndb.multiline_hint".into(),
        "Enter SQL ending with ';' (Ctrl-C aborts):".into(),
    );

    m.insert("ndb.yes".into(), "yes".into());

    m.insert("ndb.no".into(), "no".into());

    m.insert(
        "ndb.help_db_clear".into(),
        "clear | cls                Clear screen and re-show tables".into(),
    );

    m.insert(
        "ndb.help_db_exit".into(),
        "exit | q                   Close the database, back to ndb [cwd]>".into(),
    );

    m.insert(
        "ndb.help_dbal".into(),
        "dbal                          Manage saved queries (db aliases)".into(),
    );

    m.insert(
        "ndb.dbal_title".into(),
        "DBAL - DATABASE ALIASES (saved read-only queries):".into(),
    );

    m.insert(
        "ndb.dbal_add".into(),
        "dbal add <name> \"<SELECT ...>\"   Save a query".into(),
    );

    m.insert(
        "ndb.dbal_edit".into(),
        "dbal edit <name> \"<SELECT ...>\"  Update a saved query".into(),
    );

    m.insert(
        "ndb.dbal_rnm".into(),
        "dbal rnm <old> to <new>          Rename an alias".into(),
    );

    m.insert(
        "ndb.dbal_rm".into(),
        "dbal rm <name>                  Remove an alias".into(),
    );

    m.insert(
        "ndb.dbal_info".into(),
        "dbal info <name>                Show an alias body".into(),
    );

    m.insert(
        "ndb.dbal_list".into(),
        "dbal list                     List saved queries".into(),
    );

    m.insert(
        "ndb.dbal_run".into(),
        "dbal run <name> [args] [-l N|--all]  Run a saved query ($params)".into(),
    );

    m.insert(
        "ndb.dbal_cls".into(),
        "dbal cls --force               Remove ALL aliases".into(),
    );

    m.insert(
        "ndb.dbal_exec".into(),
        "<name>                          Run a saved query directly".into(),
    );

    m.insert("ndb.dbal_note".into(), "Aliases store read-only queries only; writes are rejected. Type an alias name to run it. Values substitute literally — quote strings yourself: dbal run q \"'2026-01-01'\".".into());

    m.insert("ndb.dbal_params".into(), "params: {}".into());

    m.insert(
        "ndb.dbal_missing_args".into(),
        "Missing value for parameter(s): {}".into(),
    );

    m.insert(
        "ndb.dbal_single_only".into(),
        "dbal aliases hold a single statement — use && to run several.".into(),
    );

    m.insert(
        "ndb.dbal_add_usage".into(),
        "Usage: dbal add <name> \"<SELECT ...>\"".into(),
    );

    m.insert(
        "ndb.dbal_edit_usage".into(),
        "Usage: dbal edit <name> \"<SELECT ...>\"".into(),
    );

    m.insert(
        "ndb.dbal_rnm_usage".into(),
        "Usage: dbal rnm <old> to <new>".into(),
    );

    m.insert("ndb.dbal_rm_usage".into(), "Usage: dbal rm <name>".into());

    m.insert(
        "ndb.dbal_info_usage".into(),
        "Usage: dbal info <name>".into(),
    );

    m.insert(
        "ndb.dbal_run_usage".into(),
        "Usage: dbal run <name> [-l N | --all]".into(),
    );

    m.insert(
        "ndb.dbal_cls_usage".into(),
        "Usage: dbal cls --force   (removes ALL db aliases)".into(),
    );

    m.insert("ndb.dbal_added".into(), "Alias '{}' added ({})".into());

    m.insert("ndb.dbal_updated".into(), "Alias '{}' updated".into());

    m.insert("ndb.dbal_removed".into(), "Alias '{}' removed".into());

    m.insert("ndb.dbal_renamed".into(), "Renamed '{}' to '{}'".into());

    m.insert(
        "ndb.dbal_cleared".into(),
        "Cleared {} db alias(es) ({})".into(),
    );

    m.insert("ndb.dbal_not_found".into(), "Alias '{}' not found".into());

    m.insert(
        "ndb.dbal_invalid_name".into(),
        "Invalid alias name: '{}'".into(),
    );

    m.insert(
        "ndb.dbal_name_rules".into(),
        "Names must not be ndb commands and must not contain spaces or quotes.".into(),
    );

    m.insert("ndb.dbal_blocked".into(), "dbal stores read-only queries only: only SELECT, PRAGMA, EXPLAIN, WITH and VALUES are allowed.".into());

    m.insert(
        "ndb.dbal_unknown_sub".into(),
        "Unknown dbal command: '{}'".into(),
    );

    m.insert(
        "ndb.dbal_no_aliases".into(),
        "No database aliases yet. Add one: dbal add <name> \"<SELECT ...>\"".into(),
    );

    m.insert("ndb.dbal_scope_local".into(), "local".into());

    m.insert("ndb.dbal_scope_global".into(), "global".into());

    m.insert("ndb.col_sql".into(), "SQL".into());

    m.insert(
        "ndb.help_note".into(),
        "Every database is opened read-only; ndb never modifies your files.".into(),
    );

    m.insert(
        "ndb.help_readonly".into(),
        "Read-only: only SELECT, PRAGMA, EXPLAIN, WITH and VALUES are executed.".into(),
    );

    m.insert(
        "ndb.help_limit".into(),
        "Row limit: 50 by default (-l <n> to change, --all for every row).".into(),
    );

    m.insert(
        "ndb.edit_on".into(),
        "Edit mode ON for {} (writes enabled)".into(),
    );

    m.insert(
        "ndb.edit_warn".into(),
        "Writes need explicit confirm: write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into(),
    );

    m.insert(
        "ndb.edit_failed".into(),
        "Failed to enable edit mode: {}".into(),
    );

    m.insert("ndb.edit_off".into(), "Edit mode OFF (read-only)".into());

    m.insert("ndb.edit_status".into(), "Edit mode: {}".into());

    m.insert("ndb.edit_state_on".into(), "ON".into());

    m.insert("ndb.edit_state_off".into(), "OFF".into());

    m.insert("ndb.edit_usage".into(), "Usage: edit on|off".into());

    m.insert(
        "ndb.write_needs_edit".into(),
        "Writes are disabled: run `edit on` first".into(),
    );

    m.insert(
        "ndb.write_usage".into(),
        "Usage: write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into(),
    );

    m.insert(
        "ndb.write_blocked".into(),
        "Only INSERT, UPDATE, DELETE and REPLACE are allowed via `write`".into(),
    );

    m.insert("ndb.write_confirm".into(), "About to run: {}".into());

    m.insert(
        "ndb.write_confirm_hint".into(),
        "Re-run with YES to confirm: write YES <statement>".into(),
    );

    m.insert(
        "ndb.write_done".into(),
        "Write applied: {} row(s) affected".into(),
    );

    m.insert("ndb.write_failed".into(), "Write failed: {}".into());

    m.insert("ndb.backup_done".into(), "Backup saved: {}".into());

    m.insert("ndb.backup_failed".into(), "Backup failed: {}".into());
}
