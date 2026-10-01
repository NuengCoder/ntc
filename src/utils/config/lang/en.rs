use std::collections::HashMap;



pub fn build_map() -> HashMap<String, String> {

    let mut m = HashMap::new();

    // ========== ndb (database shell) ==========
    m.insert("ndb.banner".into(), "ndb {} - NTC Database shell".into());
    m.insert("ndb.cwd_line".into(), "cwd: {}".into());
    m.insert("ndb.no_db_files".into(), "No .db files found in the current directory.".into());
    m.insert("ndb.no_db_files_hint".into(), "Add a database file here, or cd into a folder that has one.".into());
    m.insert("ndb.db_files_header".into(), "Available databases ({}):".into());
    m.insert("ndb.tables_count".into(), "{} tables".into());
    m.insert("ndb.not_a_db".into(), "not a database".into());
    m.insert("ndb.pick_db".into(), "Select a database to OPEN".into());
    m.insert("ndb.pick_table".into(), "Select a table to DISPLAY".into());
    m.insert("ndb.pick_prompt".into(), "Enter number (1-{}) or 0: ".into());
    m.insert("ndb.open_failed".into(), "Failed to open '{}': {}".into());
    m.insert("ndb.opened".into(), "Opened database: {} (read-only)".into());
    m.insert("ndb.hint_next".into(), "Use 'tables', 'viewf', 'tablef <table>', 'schema <table>', 'sql <query>', 'exit'.".into());
    m.insert("ndb.no_tables".into(), "This database has no tables.".into());
    m.insert("ndb.query_failed".into(), "Query failed: {}".into());
    m.insert("ndb.unknown_cmd".into(), "Unknown ndb command: '{}'".into());
    m.insert("ndb.hint_help".into(), "Type 'help' to list every ndb command.".into());
    m.insert("ndb.interface_usage".into(), "Usage: interface <file.db>  (or 'view' to list databases)".into());
    m.insert("ndb.sql_needs_db".into(), "Open a database first: interface <file.db>".into());
    m.insert("ndb.sql_usage".into(), "Usage: sql <query>   e.g. sql SELECT * FROM users".into());
    m.insert("ndb.sql_blocked".into(), "ndb is read-only: only SELECT, PRAGMA, EXPLAIN, WITH and VALUES are allowed.".into());
    m.insert("ndb.sql_error".into(), "SQL error: {}".into());
    m.insert("ndb.table_not_found".into(), "Table not found: {}".into());
    m.insert("ndb.schema_header".into(), "Schema for '{}':".into());
    m.insert("ndb.schema_usage".into(), "Usage: schema <table>".into());
    m.insert("ndb.showing_table".into(), "Table: {} (read-only)".into());
    m.insert("ndb.editor_fail".into(), "Failed to create line editor: {}".into());
    m.insert("ndb.start_hint".into(), "Type 'help' for commands, 'exit' to return to the ntc shell.".into());
    m.insert("ndb.closed_db".into(), "Database closed.".into());
    m.insert("ndb.bye".into(), "Left ndb shell.".into());
    m.insert("ndb.no_args".into(), "Usage: ndb   (opens the ndb database shell)".into());
    m.insert("ndb.null".into(), "NULL".into());
    m.insert("ndb.kind_view".into(), "view".into());
    m.insert("ndb.kind_table".into(), "table".into());
    m.insert("ndb.columns_count".into(), "{} columns".into());
    m.insert("ndb.rows_count".into(), "{} rows".into());
    m.insert("ndb.rows_empty".into(), "0 rows".into());
    m.insert("ndb.rows_shown".into(), "{} row(s) shown".into());
    m.insert("ndb.rows_truncated".into(), "Showing the first {} row(s) - use --all or -l <n> for more.".into());
    m.insert("ndb.no_columns".into(), "Statement returned no columns.".into());
    m.insert("ndb.col_name".into(), "NAME".into());
    m.insert("ndb.col_kind".into(), "KIND".into());
    m.insert("ndb.col_columns".into(), "COLUMNS".into());
    m.insert("ndb.col_rows".into(), "ROWS".into());
    m.insert("ndb.col_type".into(), "TYPE".into());
    m.insert("ndb.col_constraints".into(), "CONSTRAINTS".into());

    // ========== ndb in-shell help ==========
    m.insert("ndb.help_root_title".into(), "NDB - DATABASE SHELL (no database open):".into());
    m.insert("ndb.help_root_help".into(), "help | h | ?               Show this command list".into());
    m.insert("ndb.help_root_view".into(), "view | ls                  List .db files in the current directory".into());
    m.insert("ndb.help_root_interface".into(), "interface <file.db>       Open a database (prompt becomes ndb [file.db]>)".into());
    m.insert("ndb.help_root_sql".into(), "sql                        Run queries once a database is open (interface first)".into());
    m.insert("ndb.help_root_clear".into(), "clear | cls                Clear screen and re-show databases".into());
    m.insert("ndb.help_root_exit".into(), "exit | q                   Return to the ntc shell".into());
    m.insert("ndb.help_db_title".into(), "NDB - DATABASE SHELL (read-only):".into());
    m.insert("ndb.help_db_help".into(), "help | h | ?               Show this command list".into());
    m.insert("ndb.help_db_viewf".into(), "viewf | v                  Show the schema tree (tables + columns)".into());
    m.insert("ndb.help_db_tables".into(), "tables [pattern]          List table and view names (optional filter)".into());
    m.insert("ndb.help_db_tablef".into(), "tablef <table>             Display rows of a table (like txtf)".into());
    m.insert("ndb.help_db_schema".into(), "schema <table>             Show CREATE SQL, columns, indexes, keys".into());
    m.insert("ndb.help_db_sql".into(), "sql <query>                Run a read-only query (-l N, --all)".into());
    m.insert("ndb.help_db_write".into(), "write YES <stmt>           Run one write (INSERT/UPDATE/DELETE, needs edit on)".into());
    m.insert("ndb.help_db_edit".into(), "edit on|off               Allow or block writes for this database".into());
    m.insert("ndb.help_db_backup".into(), "backup                     Copy the .db file next to itself".into());
    m.insert("ndb.help_db_more".into(), "more | next              Show the next page of the last query".into());
    m.insert("ndb.help_db_indexes".into(), "indexes [table]          List indexes (all, or for one table)".into());
    m.insert("ndb.help_db_info".into(), "info                     Show file, engine and object counts".into());
    m.insert("ndb.help_db_interface".into(), "interface <file.db>       Switch to another database".into());
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
    m.insert("ndb.no_query_yet".into(), "Run 'sql <query>' or 'tablef <table>' first, then 'more'.".into());
    m.insert("ndb.multiline_hint".into(), "Enter SQL ending with ';' (Ctrl-C aborts):".into());
    m.insert("ndb.yes".into(), "yes".into());
    m.insert("ndb.no".into(), "no".into());
    m.insert("ndb.help_db_clear".into(), "clear | cls                Clear screen and re-show tables".into());
    m.insert("ndb.help_db_exit".into(), "exit | q                   Close the database, back to ndb [cwd]>".into());
    m.insert("ndb.help_dbal".into(), "dbal                          Manage saved queries (db aliases)".into());
    m.insert("ndb.dbal_title".into(), "DBAL - DATABASE ALIASES (saved read-only queries):".into());
    m.insert("ndb.dbal_add".into(), "dbal add <name> \"<SELECT ...>\"   Save a query".into());
    m.insert("ndb.dbal_edit".into(), "dbal edit <name> \"<SELECT ...>\"  Update a saved query".into());
    m.insert("ndb.dbal_rnm".into(), "dbal rnm <old> to <new>          Rename an alias".into());
    m.insert("ndb.dbal_rm".into(), "dbal rm <name>                  Remove an alias".into());
    m.insert("ndb.dbal_info".into(), "dbal info <name>                Show an alias body".into());
    m.insert("ndb.dbal_list".into(), "dbal list                     List saved queries".into());
    m.insert("ndb.dbal_run".into(), "dbal run <name> [args] [-l N|--all]  Run a saved query ($params)".into());
    m.insert("ndb.dbal_cls".into(), "dbal cls --force               Remove ALL aliases".into());
    m.insert("ndb.dbal_exec".into(), "<name>                          Run a saved query directly".into());
    m.insert("ndb.dbal_note".into(), "Aliases store read-only queries only; writes are rejected. Type an alias name to run it. Values substitute literally — quote strings yourself: dbal run q \"'2026-01-01'\".".into());
    m.insert("ndb.dbal_params".into(), "params: {}".into());
    m.insert("ndb.dbal_missing_args".into(), "Missing value for parameter(s): {}".into());
    m.insert("ndb.dbal_single_only".into(), "dbal aliases hold a single statement — use && to run several.".into());
    m.insert("ndb.dbal_add_usage".into(), "Usage: dbal add <name> \"<SELECT ...>\"".into());
    m.insert("ndb.dbal_edit_usage".into(), "Usage: dbal edit <name> \"<SELECT ...>\"".into());
    m.insert("ndb.dbal_rnm_usage".into(), "Usage: dbal rnm <old> to <new>".into());
    m.insert("ndb.dbal_rm_usage".into(), "Usage: dbal rm <name>".into());
    m.insert("ndb.dbal_info_usage".into(), "Usage: dbal info <name>".into());
    m.insert("ndb.dbal_run_usage".into(), "Usage: dbal run <name> [-l N | --all]".into());
    m.insert("ndb.dbal_cls_usage".into(), "Usage: dbal cls --force   (removes ALL db aliases)".into());
    m.insert("ndb.dbal_added".into(), "Alias '{}' added ({})".into());
    m.insert("ndb.dbal_updated".into(), "Alias '{}' updated".into());
    m.insert("ndb.dbal_removed".into(), "Alias '{}' removed".into());
    m.insert("ndb.dbal_renamed".into(), "Renamed '{}' to '{}'".into());
    m.insert("ndb.dbal_cleared".into(), "Cleared {} db alias(es) ({})".into());
    m.insert("ndb.dbal_not_found".into(), "Alias '{}' not found".into());
    m.insert("ndb.dbal_invalid_name".into(), "Invalid alias name: '{}'".into());
    m.insert("ndb.dbal_name_rules".into(), "Names must not be ndb commands and must not contain spaces or quotes.".into());
    m.insert("ndb.dbal_blocked".into(), "dbal stores read-only queries only: only SELECT, PRAGMA, EXPLAIN, WITH and VALUES are allowed.".into());
    m.insert("ndb.dbal_unknown_sub".into(), "Unknown dbal command: '{}'".into());
    m.insert("ndb.dbal_no_aliases".into(), "No database aliases yet. Add one: dbal add <name> \"<SELECT ...>\"".into());
    m.insert("ndb.dbal_scope_local".into(), "local".into());
    m.insert("ndb.dbal_scope_global".into(), "global".into());
    m.insert("ndb.col_sql".into(), "SQL".into());
    m.insert("ndb.help_note".into(), "Every database is opened read-only; ndb never modifies your files.".into());
    m.insert("ndb.help_readonly".into(), "Read-only: only SELECT, PRAGMA, EXPLAIN, WITH and VALUES are executed.".into());
    m.insert("ndb.help_limit".into(), "Row limit: 50 by default (-l <n> to change, --all for every row).".into());
    m.insert("ndb.edit_on".into(), "Edit mode ON for {} (writes enabled)".into());
    m.insert("ndb.edit_warn".into(), "Writes need explicit confirm: write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into());
    m.insert("ndb.edit_failed".into(), "Failed to enable edit mode: {}".into());
    m.insert("ndb.edit_off".into(), "Edit mode OFF (read-only)".into());
    m.insert("ndb.edit_status".into(), "Edit mode: {}".into());
    m.insert("ndb.edit_state_on".into(), "ON".into());
    m.insert("ndb.edit_state_off".into(), "OFF".into());
    m.insert("ndb.edit_usage".into(), "Usage: edit on|off".into());
    m.insert("ndb.write_needs_edit".into(), "Writes are disabled: run `edit on` first".into());
    m.insert("ndb.write_usage".into(), "Usage: write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into());
    m.insert("ndb.write_blocked".into(), "Only INSERT, UPDATE, DELETE and REPLACE are allowed via `write`".into());
    m.insert("ndb.write_confirm".into(), "About to run: {}".into());
    m.insert("ndb.write_confirm_hint".into(), "Re-run with YES to confirm: write YES <statement>".into());
    m.insert("ndb.write_done".into(), "Write applied: {} row(s) affected".into());
    m.insert("ndb.write_failed".into(), "Write failed: {}".into());
    m.insert("ndb.backup_done".into(), "Backup saved: {}".into());
    m.insert("ndb.backup_failed".into(), "Backup failed: {}".into());

    // ========== help.ndb (ntc shell help section) ==========
    m.insert("help.ndb.title".into(), "NDB (DATABASE) COMMANDS:".into());
    m.insert("help.ndb.launch".into(), "ndb ## Open the ndb database shell (.db browser) in the current directory".into());
    m.insert("help.ndb.root_help".into(), "help ## Show every ndb command (inside ndb)".into());
    m.insert("help.ndb.root_view".into(), "view | ls ## List available .db files (inside ndb)".into());
    m.insert("help.ndb.root_interface".into(), "interface <file.db> ## Open a database: prompt becomes ndb [file.db]>".into());
    m.insert("help.ndb.root_sql".into(), "sql <query> ## Run a read-only query once a database is open".into());
    m.insert("help.ndb.root_exit".into(), "exit ## Close the database, then leave ndb".into());
    m.insert("help.ndb.db_viewf".into(), "viewf | v ## Show the schema tree of tables and columns".into());
    m.insert("help.ndb.db_tables".into(), "tables | t ## List table and view names".into());
    m.insert("help.ndb.db_tablef".into(), "tablef <table> ## Display a table's rows (like txtf)".into());
    m.insert("help.ndb.db_schema".into(), "schema <table> ## Show CREATE TABLE SQL and column details".into());
    m.insert("help.ndb.db_sql".into(), "sql <query> ## Run a read-only query (-l N rows, --all)".into());
    m.insert("help.ndb.db_write".into(), "write YES <stmt> ## Run one write, needs edit on (inside ndb)".into());
    m.insert("help.ndb.db_edit".into(), "edit on|off ## Allow or block writes (inside ndb)".into());
    m.insert("help.ndb.db_backup".into(), "backup ## Copy the .db file next to itself (inside ndb)".into());
    m.insert("help.ndb.more".into(), "more ## Show the next page of the last query (inside ndb)".into());
    m.insert("help.ndb.indexes".into(), "indexes [table] ## List indexes (inside ndb)".into());
    m.insert("help.ndb.info".into(), "info ## Show database file, engine and object counts (inside ndb)".into());
    m.insert("help.ndb.dbal".into(), "dbal ## Manage saved read-only queries: add|run|list|rm (inside ndb)".into());
    m.insert("help.ndb.db_exit".into(), "exit ## Close the database and return to ndb [cwd]>".into());
    m.insert("help.ndb.readonly".into(), "read-only ## writes are blocked".into());
    m.insert("help.ndb.example1".into(), "ndb ## then: interface users.db".into());
    m.insert("help.ndb.example2".into(), "ndb ## then: tablef users --all".into());

    m.insert("alias.ral.add_cmd".into(), " ral add <name> <command> Create a new run alias".into());

    m.insert("alias.ral.alias_count".into(), "You have {} alias(es) defined:".into());

    m.insert("alias.ral.and_more".into(), "... and {} more".into());

    m.insert("alias.ral.array_cmd".into(), " ral add <name>(x[],y) <cmd> Create alias with array params (use $x[].ext or ${{x}}[].ext)".into());

    m.insert("alias.ral.cls_cancelled".into(), "Clear cancelled.".into());

    m.insert("alias.ral.cls_cmd".into(), " ral cls Clear ALL aliases (asks confirmation)".into());

    m.insert("alias.ral.cls_prompt".into(), "Are you sure? Type 'yes' to confirm: ".into());

    m.insert("alias.ral.cls_warning".into(), "\u{26A0}\u{FE0F} WARNING: This will delete ALL run aliases!".into());

    m.insert("alias.ral.config_path".into(), " Config: {}".into());

    m.insert("alias.ral.create_failed".into(), "Failed to create file: {}".into());

    m.insert("alias.ral.default_cmd".into(), " ral add <name>({{x=a,e=txt}}) <cmd> Create alias with default values".into());

    m.insert("alias.ral.edit_cmd".into(), " ral edit <name> <command> Update an existing alias".into());

    m.insert("alias.ral.empty".into(), "No run aliases defined. Use 'ral add <name> <command>'".into());

    m.insert("alias.ral.examples_header".into(), "Examples:".into());

    m.insert("alias.ral.export_all_cmd".into(), " ral export --all <name> Export all aliases to <name>.ntc.ral".into());

    m.insert("alias.ral.export_import_header".into(), "Export/Import:".into());

    m.insert("alias.ral.export_name_empty".into(), "Export name cannot be empty.".into());

    m.insert("alias.ral.export_select_cmd".into(), " ral export --select <name> Select aliases to export to <name>.ntc.ral".into());

    m.insert("alias.ral.file_exists".into(), "File already exists: {}".into());

    m.insert("alias.ral.import_cmd".into(), " ral import <file> Import aliases from a .ntc.ral file".into());

    m.insert("alias.ral.info_cmd".into(), " ral info <name> [--edit] Show alias details (--edit copies the edit command to clipboard)".into());

    m.insert("alias.ral.info_command".into(), "Command".into());

    m.insert("alias.ral.info_name".into(), "Name".into());

    m.insert("alias.ral.info_none".into(), "none".into());

    m.insert("alias.ral.info_params".into(), "Parameters".into());

    m.insert("alias.ral.info_source".into(), "Source".into());

    m.insert("alias.ral.info_source_global".into(), "global".into());

    m.insert("alias.ral.info_source_local".into(), "local (ntconfig.toml)".into());

    m.insert("alias.ral.info_title".into(), "\u{1F4CC} Alias Info".into());

    m.insert("alias.ral.info_edit_copied".into(), "Edit command copied to clipboard:".into());

    m.insert("alias.ral.info_edit_hint".into(), "Paste it, tweak the command, then press Enter.".into());

    m.insert("alias.ral.invalid_name".into(), "Invalid alias name: '{}'".into());

    m.insert("alias.ral.invalid_suffix".into(), "{} (INVALID)".into());

    m.insert("alias.ral.list_cmd".into(), " ral list Show all aliases".into());

    m.insert("alias.ral.list_title_global".into(), "\u{1F4CC} Run Aliases (global)".into());

    m.insert("alias.ral.list_title_local".into(), "\u{1F4CC} Run Aliases (from local ntconfig.toml)".into());

    m.insert("alias.ral.local_config_only".into(), " (local config only - global aliases are safe)".into());

    m.insert("alias.ral.make_cmd".into(), " ral make [name] Create alias template in CWD".into());

    m.insert("alias.ral.name_rule_at".into(), " - Start with @ or #".into());

    m.insert("alias.ral.name_rule_reserved".into(), " - Be a reserved command (go, view, txt, etc.)".into());

    m.insert("alias.ral.name_rules".into(), "Alias names cannot:".into());

    m.insert("alias.ral.no_aliases".into(), "No run aliases to clear.".into());

    m.insert("alias.ral.not_found".into(), "Alias '{}' not found".into());

    m.insert("alias.ral.now_run".into(), "Now you can run: {}".into());

    m.insert("alias.ral.now_run_arg".into(), "Now you can run: {}({})".into());

    m.insert("alias.ral.now_run_defaults".into(), "Now you can run: {}(<arg>) (defaults: {})".into());

    m.insert("alias.ral.param_cmd".into(), " ral add <name>(x) <command> Create parameterised alias (use $x or ${{x}} in command)".into());

    m.insert("alias.ral.renamed".into(), "Renamed alias '{}' to '{}'".into());

    m.insert("alias.ral.rm_cmd".into(), " ral rm <name>[, <name>, ...] Remove alias(es)".into());

    m.insert("alias.ral.rnm_cmd".into(), " ral rnm <old> to <new> Rename an alias".into());

    m.insert("alias.ral.template_created".into(), "Created ral template: {}".into());

    m.insert("alias.ral.template_info".into(), "This is a local template file. Use 'ral import' to add aliases to your config.".into());

    m.insert("alias.ral.tip_local".into(), "\u{1F4A1} Tip: These aliases are project-specific (saved in ntconfig.toml)".into());

    m.insert("alias.ral.title".into(), "Run Alias (ral) Commands:".into());

    m.insert("alias.ral.type_ral_help".into(), "Type 'ral' for help.".into());

    m.insert("alias.ral.unknown_flag".into(), "Unknown export flag: {}. Use --all or --select".into());

    m.insert("alias.ral.unknown_subcmd".into(), "Unknown ral subcommand: {}".into());

    m.insert("alias.ral.usage_add".into(), "Usage: ral add <name> <command>".into());

    m.insert("alias.ral.usage_edit".into(), "Usage: ral edit <name> <new_command>".into());

    m.insert("alias.ral.usage_export".into(), "Usage: ral export --all <name> | ral export --select <name>".into());

    m.insert("alias.ral.usage_hint".into(), "Usage: <alias> or <alias>(<arg>) for parameterised aliases".into());

    m.insert("alias.ral.usage_import".into(), "Usage: ral import <file>".into());

    m.insert("alias.ral.usage_info".into(), "Usage: ral info <alias> [--edit]".into());

    m.insert("alias.ral.usage_rm".into(), "Usage: ral rm <name>[, <name>, ...]".into());

    m.insert("alias.ral.usage_rnm".into(), "Usage: ral rnm <old_name> to <new_name>".into());

    m.insert("alias.ral.usage_with_run".into(), "Usage with run:".into());

    m.insert("alias.ral.run_example1".into(), "  run btr              # Executes: cargo build --release".into());

    m.insert("alias.ral.run_example2".into(), "  run py               # Executes: python test.py".into());

    m.insert("alias.ral.run_example3".into(), "  run_file(hello)      # Executes: python hello.py".into());

    m.insert("alias.ral.run_example4".into(), "  runc [add,minus,mul] math  # Executes: cls && gcc -o math.exe add.c minus.c mul.c && ...".into());

    m.insert("alias.ral.export_example1".into(), "  ral export --all myaliases       # Creates myaliases.ntc.ral".into());

    m.insert("alias.ral.export_example2".into(), "  ral export --select myaliases    # Interactive pick & export".into());

    m.insert("alias.ral.export_example3".into(), "  ral import myaliases.ntc.ral     # Import into current config".into());

    m.insert("alias.run.examples_header".into(), "Examples:".into());

    m.insert("alias.run.example_alias".into(), "  run btr                     # Run alias".into());

    m.insert("alias.run.example_alias_args".into(), "  run py test.py              # Run alias with args".into());

    m.insert("alias.run.example_param".into(), "  run_file(hello)             # Run parameterised alias".into());

    m.insert("alias.run.example_real".into(), "  run python --version        # Run real command".into());

    m.insert("alias.run.usage".into(), "Usage: run <command|alias> [args...]".into());

    m.insert("autoup.added".into(), "Added startup entry: {} \u{2192} {}".into());

    m.insert("autoup.already_disabled".into(), "Entry '{}' is already disabled.".into());

    m.insert("autoup.already_enabled".into(), "Entry '{}' is already enabled.".into());

    m.insert("autoup.already_exists".into(), "Entry '{}' already exists in user startup.".into());

    m.insert("autoup.cmd_empty".into(), "Command value cannot be empty.".into());

    m.insert("autoup.disabled".into(), "Disabled startup entry: {}".into());

    m.insert("autoup.disabled_system".into(), "Disabled system startup entry: {}".into());

    m.insert("autoup.enabled".into(), "Enabled startup entry: {}".into());

    m.insert("autoup.enabled_system".into(), "Enabled system startup entry: {}".into());

    m.insert("autoup.help_add".into(), " autoup add <name> \"<command>\" Add a startup entry (HKCU)".into());

    m.insert("autoup.help_disable".into(), " autoup disable <name> Disable a startup entry".into());

    m.insert("autoup.help_enable".into(), " autoup enable <name> Re-enable a disabled entry".into());

    m.insert("autoup.help_help".into(), " autoup help Show this help".into());

    m.insert("autoup.help_list".into(), " autoup list List all startup entries from all sources".into());

    m.insert("autoup.help_rm".into(), " autoup rm <name> Remove a startup entry".into());

    m.insert("autoup.hklm_hint".into(), "'{}' is a system entry (HKLM). Run 'mode admin' first, then retry.".into());

    m.insert("autoup.hklm_hint_short".into(), "'{}' is a system entry. Run 'mode admin' first, then retry.".into());

    m.insert("autoup.list_empty".into(), "(No startup entries found)".into());

    m.insert("autoup.list_title".into(), "Windows Startup Entries".into());

    m.insert("autoup.list_total".into(), "Total: {} [A]:{} [D]:{} (CU:{} LM:{} SF:{})".into());

    m.insert("autoup.not_found".into(), "Startup entry not found: {}".into());

    m.insert("autoup.reg_delete_fail".into(), "Failed to delete registry entry.".into());

    m.insert("autoup.reg_disable_fail".into(), "Failed to set disabled state.".into());

    m.insert("autoup.reg_exec_fail".into(), "Failed to run reg.exe: {}".into());

    m.insert("autoup.reg_query_fail".into(), "Failed to query registry: {}".into());

    m.insert("autoup.reg_write_fail".into(), "Failed to write registry entry. Try running as Administrator.".into());

    m.insert("autoup.removed".into(), "Removed startup entry: {}".into());

    m.insert("autoup.removed_system".into(), "Removed system startup entry: {}".into());

    m.insert("autoup.sf_disable_hint".into(), "'{}' is in the Startup folder. Move the shortcut out to disable it.".into());

    m.insert("autoup.sf_rm_hint".into(), "'{}' is in the Startup folder. Delete the shortcut file manually.".into());

    m.insert("autoup.source_cu".into(), " [CU] Current User (HKCU\\...\\Run)".into());

    m.insert("autoup.source_lm".into(), " [LM] Local Machine (HKLM\\...\\Run) \u{2014} requires admin (mode admin)".into());

    m.insert("autoup.source_sf".into(), " [SF] Startup Folder (%APPDATA%\\...\\Startup) \u{2014} read-only".into());

    m.insert("autoup.source_tags".into(), "Source tags:".into());

    m.insert("autoup.usage_add".into(), "Usage: autoup add <name> \"<command>\"".into());

    m.insert("autoup.usage_disable".into(), "Usage: autoup disable <name>".into());

    m.insert("autoup.usage_enable".into(), "Usage: autoup enable <name>".into());

    m.insert("autoup.usage_examples".into(), "Examples:".into());

    m.insert("autoup.usage_rm".into(), "Usage: autoup rm <name>".into());

    m.insert("autoup.usage_title".into(), "Usage:".into());

    m.insert("autoup.use_rm_first".into(), "Use 'autoup rm' first if you want to replace it.".into());

    m.insert("autoup.will_run".into(), "The entry will run on your next login.".into());

    m.insert("autoup.windows_only".into(), "autoup is a Windows-only feature.".into());

    m.insert("backup.and_more".into(), " ... and {} more".into());

    m.insert("backup.available_title".into(), " Available Backups (newest first)".into());

    m.insert("backup.cancel_opt".into(), " 0. Cancel".into());

    m.insert("backup.cancelled".into(), "Clear cancelled.".into());

    m.insert("backup.clear_history_prompt".into(), "\u{26A0} Clear undo history? This cannot be undone. (y/N):".into());

    m.insert("backup.cleared".into(), "All backups deleted.".into());

    m.insert("backup.cleared_count".into(), "Cleared {} backup(s) for this project.".into());

    m.insert("backup.confirm".into(), "Type 'yes' to confirm: ".into());

    m.insert("backup.create_hint".into(), "Use 'backup create' to create your first backup.".into());

    m.insert("backup.created".into(), "Backup created: #{} \u{2014} {} \u{2014} {} files".into());

    m.insert("backup.created_detail".into(), "Backup #{} created: {} ({} files, {})".into());

    m.insert("backup.creating".into(), "Creating backup...".into());

    m.insert("backup.delete_extra_failed".into(), "Failed to delete extra file {}: {}".into());

    m.insert("backup.deleted_since".into(), "{} has been deleted since the backup.".into());

    m.insert("backup.entry_file_count".into(), " Backup #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into());

    m.insert("backup.exists".into(), "Backups exist: {} backup(s) at {}".into());

    m.insert("backup.failed_to_backup".into(), "Failed to backup {}: {}".into());

    m.insert("backup.file_deleted".into(), "{} was deleted after the backup.".into());

    m.insert("backup.file_modified".into(), "{} has changed since the last save.".into());

    m.insert("backup.help_clear".into(), " backup clear Delete all backups (with confirmation)".into());

    m.insert("backup.help_clear_force".into(), " backup clear --force Delete all backups (no confirmation)".into());

    m.insert("backup.help_create".into(), " backup create Create a new backup".into());

    m.insert("backup.help_diff".into(), " backup diff [num] Diff current project against a backup".into());

    m.insert("backup.help_examples".into(), "Examples:".into());

    m.insert("backup.help_list".into(), " backup list List all backups".into());

    m.insert("backup.help_recreate".into(), " backup recreate Clear all + undo history + create fresh".into());

    m.insert("backup.help_recreate_force".into(), " backup recreate --force Same, skip all confirmations".into());

    m.insert("backup.help_restore".into(), " backup restore [num] Restore a backup (interactive if no num)".into());

    m.insert("backup.help_restore_choose".into(), " backup restore --choose Pick a backup from an interactive menu".into());

    m.insert("backup.help_restore_choose_deleted".into(), " backup restore --choose --deleted Pick + restore-deleted".into());

    m.insert("backup.help_restore_deleted".into(), " backup restore --deleted Restore only files deleted since the backup".into());

    m.insert("backup.help_title".into(), "backup \u{2014} Manage project backups".into());

    m.insert("backup.help_undo".into(), " backup undo Undo last restore".into());

    m.insert("backup.help_undo_cls".into(), " backup undo --cls Clear undo history".into());

    m.insert("backup.help_undo_force".into(), " backup undo --force Clear undo history (no confirm)".into());

    m.insert("backup.invalid_arg".into(), "Invalid argument: {}".into());

    m.insert("backup.invalid_diff_arg".into(), "Invalid argument: {}. Usage: backup diff <number>".into());

    m.insert("backup.invalid_selection".into(), "Invalid selection: {}".into());

    m.insert("backup.list_entry".into(), " #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into());

    m.insert("backup.list_location".into(), "Location: {}".into());

    m.insert("backup.list_title".into(), "Backups for this project: {} found".into());

    m.insert("backup.menu_item".into(), "Backup #{}".into());

    m.insert("backup.modified_since".into(), "{} has been modified since the backup.".into());

    m.insert("backup.no_deleted_files".into(), "No deleted files to restore \u{2014} project already matches the backup snapshot.".into());

    m.insert("backup.none".into(), "No backups found for this project.".into());

    m.insert("backup.not_found_num".into(), "Backup #{} not found.".into());

    m.insert("backup.pick_detail".into(), "{} \u{2014} {} \u{2014} {} files".into());

    m.insert("backup.pick_prompt".into(), "Enter number (1-{}) or 0: ".into());

    m.insert("backup.pick_title".into(), "Select a backup to restore from".into());

    m.insert("backup.previous_state_saved".into(), "Previous state saved. Use 'unpd' to undo this restore.".into());

    m.insert("backup.recreate_proceed".into(), "Proceed? (y/N): ".into());

    m.insert("backup.recreate_prompt".into(), "No existing backups found. Creating fresh backup anyway? (y/N): ".into());

    m.insert("backup.recreate_step1".into(), " 1. Delete ALL existing backups".into());

    m.insert("backup.recreate_step2".into(), " 2. Clear undo history".into());

    m.insert("backup.recreate_step3".into(), " 3. Create a fresh backup of the current state".into());

    m.insert("backup.recreate_will_title".into(), "\u{26A0}\u{FE0F} RECREATE will:".into());

    m.insert("backup.recreated".into(), "Project recreated: old backups cleared, fresh backup created.".into());

    m.insert("backup.restore_add_count".into(), "{} file(s) will be restored.".into());

    m.insert("backup.restore_cancelled".into(), "Restore cancelled.".into());

    m.insert("backup.restore_confirm".into(), "Proceed with restore? (y/N): ".into());

    m.insert("backup.restore_create_title".into(), "\u{26A0} The following files will be CREATED ({}):".into());

    m.insert("backup.restore_delete_title".into(), "\u{26A0} The following files will be DELETED (not in backup) ({}):".into());

    m.insert("backup.restore_deleted_confirm".into(), "Continue with restore-deleted? (y/N): ".into());

    m.insert("backup.restore_deleted_title".into(), " {} files will be restored (were present in backup but are gone now):".into());

    m.insert("backup.restore_failed".into(), "Failed to restore {}: {}".into());

    m.insert("backup.restore_overwrite_count".into(), "{} file(s) will be overwritten.".into());

    m.insert("backup.restore_overwrite_title".into(), "\u{26A0} The following files will be OVERWRITTEN ({}):".into());

    m.insert("backup.restore_preview_title".into(), "Restore Preview".into());

    m.insert("backup.restored_backup".into(), "Restore complete: Backup #{} \u{2014} {} files restored".into());

    m.insert("backup.restored_date".into(), "Restore was performed on: {}".into());

    m.insert("backup.restored_deleted".into(), "Restored deleted files from Backup #{}: {} file(s) restored".into());

    m.insert("backup.restored_result".into(), "Restored backup #{} ({})".into());

    m.insert("backup.select_entry".into(), " {}. Backup #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into());

    m.insert("backup.select_prompt".into(), "Select backup to restore (1-{}): ".into());

    m.insert("backup.skip_large".into(), "Skipped {} ({} MB exceeds 50 MB limit)".into());

    m.insert("backup.skip_metadata".into(), "Skipping {} (metadata error: {})".into());

    m.insert("backup.skip_rel_path".into(), "Skipping {} (could not resolve relative path)".into());

    m.insert("backup.skip_symlink".into(), "Skipping {} (symlink \u{2014} not following)".into());

    m.insert("backup.skipped_count".into(), "Skipped {} file(s) \u{2014} see ignore.txt for details".into());

    m.insert("backup.skipped_error".into(), "Skipped {} ({})".into());

    m.insert("backup.skipped_large".into(), "Skipped {} ({} \u{2014} too large)".into());

    m.insert("backup.skipped_unsupported".into(), "Skipped {} (unsupported file type)".into());

    m.insert("backup.snapshot_failed".into(), "Could not snapshot {} for undo ({}), continuing".into());

    m.insert("backup.some_restore_failed".into(), "Some files could not be restored. Undo storage has been kept.".into());

    m.insert("backup.stored_in".into(), "Backups stored in: {}".into());

    m.insert("backup.undo_cleared".into(), "Undo history cleared (freed {})".into());

    m.insert("backup.undo_complete".into(), "Undo complete: Restored {} file(s) from before last restore".into());

    m.insert("backup.undo_complete_detail".into(), "Undid restore of backup #{} ({} restored, {} deleted, {} re-restored)".into());

    m.insert("backup.undo_delete_failed".into(), "Could not delete restored file {}: {}".into());

    m.insert("backup.undo_missing_original".into(), "Could not restore {} (original missing from undo storage)".into());

    m.insert("backup.undo_missing_snapshot".into(), "Could not re-restore {} (snapshot missing from undo storage)".into());

    m.insert("backup.undo_no_history".into(), "No undo history found.".into());

    m.insert("backup.undo_none".into(), "No restore operation to undo.".into());

    m.insert("backup.undo_rerestore_failed".into(), "Could not re-restore deleted file {}: {}".into());

    m.insert("backup.unknown_clear_option".into(), "Unknown backup clear option: {}".into());

    m.insert("backup.unknown_create_option".into(), "Unknown backup create option: {}".into());

    m.insert("backup.unknown_recreate_option".into(), "Unknown backup recreate option: {}".into());

    m.insert("backup.unknown_undo_option".into(), "Unknown backup undo option: {}".into());

    m.insert("backup.usage_clear".into(), "Usage: backup clear [--force]".into());

    m.insert("backup.usage_recreate".into(), "Usage: backup recreate [--force]".into());

    m.insert("backup.usage_restore".into(), "Usage: backup restore [--choose] [--deleted] [num]".into());

    m.insert("backup.warning_count".into(), "You have {} backup(s):".into());

    m.insert("backup.warning_title".into(), "\u{26A0}\u{FE0F} WARNING: This will delete ALL backups for this project!".into());

    m.insert("backup.would_overwrite".into(), "{} would be overwritten (use --force to bypass)".into());

    m.insert("backup.would_overwrite_deleted".into(), "Restoring deleted file: {}".into());

    m.insert("backup_diff.cancel_opt".into(), "0. Cancel".into());

    m.insert("backup_diff.cancelled".into(), "Diff cancelled.".into());

    m.insert("backup_diff.comparing".into(), "Comparing current project with backup #{}...".into());

    m.insert("backup_diff.invalid_input".into(), "Invalid input: {}".into());

    m.insert("backup_diff.invalid_selection".into(), "Invalid selection: {}".into());

    m.insert("backup_diff.none".into(), "No backups found for this project. Use 'bkup' to create one.".into());

    m.insert("backup_diff.not_found".into(), "Backup #{} not found.".into());

    m.insert("backup_diff.open_manually".into(), "Open the file manually: {}".into());

    m.insert("backup_diff.saved_to".into(), "Diff report saved to: {}".into());

    m.insert("backup_diff.select_prompt".into(), "Select backup (1-{}): ".into());

    m.insert("backup_diff.select_title".into(), "📊 Select a backup to diff against current project".into());

    m.insert("clap.arg.cared.help".into(), "Stop ignoring one or more directories".into());

    m.insert("clap.arg.caref.help".into(), "Care about one or more file extensions".into());

    m.insert("clap.arg.caren.help".into(), "Care about one or more specific files".into());

    m.insert("clap.arg.clear.help".into(), "Clear the terminal screen".into());

    m.insert("clap.arg.copy.help".into(), "Copy report to clipboard instead of saving to file".into());


    m.insert("clap.arg.edit.help".into(), "Open file in built-in text editor".into());

    m.insert("clap.arg.explorer.help".into(), "Launch NTC File Explorer TUI (optional starting path)".into());

    m.insert("clap.arg.format_cli.help".into(), "Output format: txt, html, json, md".into());

    m.insert("clap.arg.generate_completions.help".into(), "Generate shell completion script (bash|zsh|fish|powershell)".into());

    m.insert("clap.arg.help_arg.help".into(), "Show detailed help".into());

    m.insert("clap.arg.hyper.help".into(), "Create hyper project scaffold (DSA library)".into());

    m.insert("clap.arg.igcare_export_all.help".into(), "Export all ignore/care settings to <name>.ntc.igcare".into());

    m.insert("clap.arg.igcare_export_select.help".into(), "Select categories to export to <name>.ntc.igcare".into());

    m.insert("clap.arg.igcare_import.help".into(), "Import ignore/care settings from .ntc.igcare file".into());

    m.insert("clap.arg.ignore.help".into(), "Ignore one or more directory names".into());

    m.insert("clap.arg.ignored.help".into(), "Show ignored items".into());

    m.insert("clap.arg.ignoref.help".into(), "Ignore one or more file extensions".into());

    m.insert("clap.arg.ignoren.help".into(), "Ignore one or more specific files".into());

    m.insert("clap.arg.image.help".into(), "Convert image to document (JSON, MD, HTML, TXT)".into());

    m.insert("clap.arg.image_to.help".into(), "Output format for image conversion: json, md, html, txt".into());

    m.insert("clap.arg.init.help".into(), "Create starter file from template and open in editor".into());

    m.insert("clap.arg.input.help".into(), "Input file or directory path".into());

    m.insert("clap.arg.lang_cli.help".into(), "Language for hyper project".into());

    m.insert("clap.arg.list.help".into(), "List all command-line functions".into());

    m.insert("clap.arg.lsp.help".into(), "Start the ntc-math LSP server over stdio".into());

    m.insert("clap.arg.lsp_log.help".into(), "Path to write LSP debug log".into());

    m.insert("clap.arg.math.help".into(), "Evaluate a math expression".into());

    m.insert("clap.arg.output.help".into(), "Output filename for report".into());

    m.insert("clap.arg.ral_export_all.help".into(), "Export all run aliases to <name>.ntc.ral".into());

    m.insert("clap.arg.ral_export_select.help".into(), "Select run aliases to export to <name>.ntc.ral".into());

    m.insert("clap.arg.ral_import.help".into(), "Import run aliases from .ntc.ral file".into());

    m.insert("clap.arg.say.help".into(), "Print text to stdout".into());

    m.insert("clap.arg.setA.help".into(), "Show or set autosuggest (ghost text) (ON/OFF)".into());

    m.insert("clap.arg.setC.help".into(), "Show or set color output (ON/OFF)".into());

    m.insert("clap.arg.setD.help".into(), "Show or set max recursion depth".into());

    m.insert("clap.arg.setH.help".into(), "Show or set history (path/ON/OFF/default)".into());

    m.insert("clap.arg.setL.help".into(), "Show or set line number display (ON/OFF)".into());

    m.insert("clap.arg.setO.help".into(), "Show or set the default output directory".into());

    m.insert("clap.arg.setT.help".into(), "Show or set number of threads".into());

    m.insert("clap.arg.showcg.help".into(), "Show current configuration overview".into());

    m.insert("clap.arg.size_cli.help".into(), "Show current directory size".into());

    m.insert("clap.arg.time.help".into(), "Show elapsed time (use with --size or --view)".into());

    m.insert("clap.arg.tp_add.help".into(), "Save current directory as teleport point".into());

    m.insert("clap.arg.tp_list.help".into(), "List all teleport points".into());

    m.insert("clap.arg.tp_rm.help".into(), "Remove teleport point".into());

    m.insert("clap.arg.upgrade.help".into(), "Check for updates and upgrade ntc to the latest version".into());

    m.insert("clap.arg.version.help".into(), "Show version information".into());

    m.insert("clap.arg.view_cli.help".into(), "Quick view of current directory tree".into());

    m.insert("clap.arg.watch_cli.help".into(), "Show or set file watcher (ON/OFF)".into());

    m.insert("clap.arg.where_cli.help".into(), "Show ntc executable location".into());

    m.insert("clap.value_name.depth".into(), "DEPTH".into());

    m.insert("clap.value_name.expr".into(), "EXPR".into());

    m.insert("clap.value_name.ext".into(), "EXT".into());

    m.insert("clap.value_name.file".into(), "FILE".into());

    m.insert("clap.value_name.format_cli".into(), "FORMAT".into());

    m.insert("clap.value_name.image".into(), "IMAGE".into());

    m.insert("clap.value_name.lang".into(), "LANG".into());

    m.insert("clap.value_name.name".into(), "NAME".into());

    m.insert("clap.value_name.path".into(), "PATH".into());

    m.insert("clap.value_name.project".into(), "PROJECT".into());

    m.insert("clap.value_name.shell".into(), "SHELL".into());

    m.insert("clap.value_name.state".into(), "STATE".into());

    m.insert("clap.value_name.text".into(), "TEXT".into());

    m.insert("clap.value_name.threads".into(), "THREADS".into());

    m.insert("clap.value_name.value".into(), "VALUE".into());

    m.insert("cli.completions.shell.bash".into(), "bash".into());

    m.insert("cli.completions.shell.fish".into(), "fish".into());

    m.insert("cli.completions.shell.powershell".into(), "powershell".into());

    m.insert("cli.completions.shell.zsh".into(), "zsh".into());

    m.insert("cli.example.cat_file".into(), " ntc -i file.txt Display file contents".into());

    m.insert("cli.example.image".into(), " ntc -image screenshot.png to json Convert image to JSON for AI\n".into());

    m.insert("cli.example.interactive".into(), " ntc Launch interactive shell".into());

    m.insert("cli.example.math_arithmetic".into(), " ntc --math \"3+4*5\" Evaluate math expression".into());

    m.insert("cli.example.math_func".into(), " ntc --math \"sin(PI/2)\" Math with functions and constants".into());

    m.insert("cli.example.report_cp".into(), " ntc -i src --cp Copy directory tree to clipboard".into());

    m.insert("cli.example.report_docx".into(), " ntc -i src -f docx Generate DOCX report from src".into());

    m.insert("cli.example.report_json_cp".into(), " ntc -i src -f json --cp Copy JSON report to clipboard".into());

    m.insert("cli.example.report_pdf".into(), " ntc -i src -f pdf Generate PDF report from src".into());

    m.insert("cli.example.report_txt".into(), " ntc -i src Generate TXT report of src directory".into());

    m.insert("cli.example.report_xlsx".into(), " ntc -i src -f xlsx Generate XLSX report (with dependency analysis)".into());

    m.insert("cli.example.say".into(), " ntc -say \"Hello World\" Print Hello World".into());

    m.insert("cli.example.setL".into(), " ntc --setL ON Enable line numbers".into());

    m.insert("cli.example.showcg".into(), " ntc --showcg Show configuration".into());

    m.insert("cli.example.teleport_at".into(), " ntc @web Launch and teleport to 'web' savepoint".into());

    m.insert("cli.example.watch".into(), " ntc --watch ON Enable file watcher".into());

    m.insert("cli.flag.cared".into(), " --cared <name> Stop ignoring a directory name".into());

    m.insert("cli.flag.caref".into(), " --caref <ext> Care about a file extension".into());

    m.insert("cli.flag.caren".into(), " --caren <file> Care about a specific file\n".into());

    m.insert("cli.flag.clear".into(), " --clear Clear the terminal screen".into());

    m.insert("cli.flag.copy".into(), " --cp Copy report to clipboard instead of saving to file".into());


    m.insert("cli.flag.edit".into(), " -e, --edit <file> Open file in built-in text editor".into());

    m.insert("cli.flag.explorer".into(), " -x, --explorer [PATH] Launch NTC File Explorer TUI".into());

    m.insert("cli.flag.format".into(), " -f, --format <FORMAT> Output format: txt, html, json, md, pdf, docx, xlsx (default: txt)".into());

    m.insert("cli.flag.generate_completions".into(), " --generate-completions <SHELL> Generate completions (bash|zsh|fish|powershell)\n".into());

    m.insert("cli.flag.help_flag".into(), " --help Show this help message".into());

    m.insert("cli.flag.hyper".into(), " --hyper <name> --lang c Create C/C++ hyper project scaffold\n".into());

    m.insert("cli.flag.igcare_export_all".into(), " --igcare-export-all <name> Export all ignore/care settings".into());

    m.insert("cli.flag.igcare_export_select".into(), " --igcare-export-select <name> Select categories to export".into());

    m.insert("cli.flag.igcare_import".into(), " --igcare-import <file> Import settings from .ntc.igcare file\n".into());

    m.insert("cli.flag.ignore".into(), " --ignore <name> Ignore a directory name".into());

    m.insert("cli.flag.ignored".into(), " --ignored Show currently ignored items".into());

    m.insert("cli.flag.ignoref".into(), " --ignoref <ext> Ignore a file extension".into());

    m.insert("cli.flag.ignoren".into(), " --ignoren <file> Ignore a specific file (e.g., Cargo.lock)".into());

    m.insert("cli.flag.image".into(), " -image <path> to <fmt> Convert image to document for AI (json|md|html|txt)".into());

    m.insert("cli.flag.init".into(), " --init <file> Create starter template and open in editor".into());

    m.insert("cli.flag.input".into(), " -i, --input <path> Process a file or directory".into());

    m.insert("cli.flag.list".into(), " --list, --fun List all command-line functions".into());

    m.insert("cli.flag.math".into(), " --math <EXPR> Evaluate a math expression (e.g. --math \"3+4*5\")".into());

    m.insert("cli.flag.output".into(), " -o, --output <file> Save output to specified file".into());

    m.insert("cli.flag.ral_export_all".into(), " --ral-export-all <name> Export all run aliases to <name>.ntc.ral".into());

    m.insert("cli.flag.ral_export_select".into(), " --ral-export-select <name> Select aliases to export".into());

    m.insert("cli.flag.ral_import".into(), " --ral-import <file> Import aliases from .ntc.ral file".into());

    m.insert("cli.flag.say".into(), " -say, -print <text> Print text to stdout".into());

    m.insert("cli.flag.setA".into(), " --setA [ON|OFF] Show/set autosuggest (ghost text)".into());

    m.insert("cli.flag.setC".into(), " --setC [ON|OFF] Show or toggle color output".into());

    m.insert("cli.flag.setD".into(), " --setD [depth] Show or set max recursion depth (min: 1, max: 20)".into());

    m.insert("cli.flag.setH".into(), " --setH [path|ON|OFF] Show/set history path or enable/disable".into());

    m.insert("cli.flag.setL".into(), " --setL [ON|OFF] Show or toggle line numbers for file display".into());

    m.insert("cli.flag.setO".into(), " --setO [path] Show or set the output directory (default: Desktop)".into());

    m.insert("cli.flag.setT".into(), " --setT [threads] Show or set number of threads (default: 4)".into());

    m.insert("cli.flag.showcg".into(), " --showcg Show current configuration overview".into());

    m.insert("cli.flag.size".into(), " --size Show current directory size".into());

    m.insert("cli.flag.tp_add".into(), " --tp-add <name> Save current directory as teleport point".into());

    m.insert("cli.flag.tp_list".into(), " --tp-list List all teleport points".into());

    m.insert("cli.flag.tp_rm".into(), " --tp-rm <name> Remove a teleport point\n".into());

    m.insert("cli.flag.version".into(), " --version Show version information".into());

    m.insert("cli.flag.view".into(), " --view Quick view of current directory tree".into());

    m.insert("cli.flag.view_size".into(), " --view --size Quick view with directory sizes\n".into());

    m.insert("cli.flag.watch_flag".into(), " --watch [ON|OFF] Show/set file watcher state".into());

    m.insert("cli.flag.where".into(), " --where Show ntc executable and config location\n".into());

    m.insert("cli.help.description".into(), "A combine of Navigation, Project Toolkit Center hub.\n".into());

    m.insert("cli.help.title".into(), "ntc <version> \u{2014} Navigate, Toolkit, Center".into());

    m.insert("cli.helpers.basic_usage".into(), " ntc [OPTIONS]\n ntc -i <path> [-o <output>]\n ntc -i <path> --cp Copy report to clipboard\n".into());

    m.insert("cli.helpers.cli_examples".into(), " ntc Launch interactive shell\n ntc @web Launch and teleport to 'web' savepoint\n ntc --math \"3+4*5\" Evaluate math expression\n ntc --math \"sin(PI/2)\" Math with functions and constants\n ntc -i src Generate TXT report of src directory\n ntc -i src --cp Copy directory tree to clipboard\n ntc -i src -f json --cp Copy JSON report to clipboard\n ntc -i src -f pdf Generate PDF report from src\n ntc -i src -f docx Generate DOCX report from src\n ntc -i src -f xlsx Generate XLSX report (with dependency analysis)\n ntc -i file.txt Display file contents\n ntc --setL ON Enable line numbers\n ntc --showcg Show configuration\n ntc -say \"Hello World\" Print Hello World\n ntc --watch ON Enable file watcher\n ntc -image screenshot.png to json Convert image to JSON for AI\n".into());

    m.insert("cli.helpers.config_header".into(), "CONFIGURATION OPTIONS:".into());

    m.insert("cli.helpers.config_options".into(), " --setO [path] Show or set the output directory (default: Desktop)\n --setD [depth] Show or set max recursion depth (min: 1, max: 20)\n --setL [ON|OFF] Show or toggle line numbers for file display\n --setC [ON|OFF] Show or toggle color output\n --setT [threads] Show or set number of threads (default: 4)\n --setH [path|ON|OFF] Show/set history path or enable/disable\n --setA [ON|OFF] Show/set autosuggest (ghost text)\n --showcg Show current configuration overview\n --watch [ON|OFF] Show/set file watcher state\n --where Show ntc executable and config location\n".into());

    m.insert("cli.helpers.editor_header".into(), "EDITOR OPTIONS:".into());

    m.insert("cli.helpers.editor_options".into(), " -e, --edit <file> Open file in built-in text editor\n --init <file> Create starter template and open in editor\n --hyper <name> --lang c Create C/C++ hyper project scaffold\n".into());

    m.insert("cli.helpers.examples_header".into(), "EXAMPLES:".into());

    m.insert("cli.helpers.export_header".into(), "ALIAS / IGNORE/CARE EXPORT:".into());

    m.insert("cli.helpers.export_options".into(), " --ral-export-all <name> Export all run aliases to <name>.ntc.ral\n --ral-export-select <name> Select aliases to export\n --ral-import <file> Import aliases from .ntc.ral file\n --igcare-export-all <name> Export all ignore/care settings\n --igcare-export-select <name> Select categories to export\n --igcare-import <file> Import settings from .ntc.igcare file\n".into());

    m.insert("cli.helpers.igcare_options".into(), " --ignored Show currently ignored items\n --ignore <name> Ignore a directory name\n --cared <name> Stop ignoring a directory name\n --ignoref <ext> Ignore a file extension\n --caref <ext> Care about a file extension\n --ignoren <file> Ignore a specific file (e.g., Cargo.lock)\n --caren <file> Care about a specific file\n".into());

    m.insert("cli.helpers.ignore_header".into(), "IGNORE/CARE OPTIONS:".into());

    m.insert("cli.helpers.interactive_note".into(), "INTERACTIVE-ONLY (run 'ntc' with no args):\n Navigation: cd, back, gos, gosc\n Teleport: tp jump, tp to, @name\n Tutorial: tutorial \u{2014} Start interactive step-by-step tutorial\n".into());

    m.insert("cli.helpers.other_header".into(), "OTHER OPTIONS:".into());

    m.insert("cli.helpers.other_options".into(), " -say, -print <text> Print text to stdout\n -image <path> to <fmt> Convert image to document for AI (json|md|html|txt)\n -x, --explorer [PATH] Launch NTC File Explorer TUI\n --clear Clear the terminal screen\n --version Show version information\n --math <EXPR> Evaluate a math expression (e.g. --math \"3+4*5\")\n --list, --fun List all command-line functions\n --help Show this help message\n --generate-completions <SHELL> Generate completions (bash|zsh|fish|powershell)\n".into());

    m.insert("cli.helpers.report_header".into(), "REPORT OPTIONS:".into());

    m.insert("cli.helpers.report_options".into(), " -i, --input <path> Process a file or directory\n -o, --output <file> Save output to specified file\n -f, --format <FORMAT> Output format: txt, html, json, md, pdf, docx, xlsx (default: txt)\n --cp Copy report to clipboard instead of saving to file\n --size Show current directory size\n --view Quick view of current directory tree\n --view --size Quick view with directory sizes\n".into());

    m.insert("cli.helpers.shell_commands".into(), " ntc pathcp Copy path to clipboard\n ntc say \"hello\" --loop 3 Print text with loop\n ntc tl dir . Count lines in directory\n ntc ne Open text editor\n ntc size Show directory size\n ntc view Show directory tree\n ntc serve 8080 Start HTTP server on port 8080\n ntc dl https://example.com/file.zip --progress Download with progress\n ntc encode b64 hello Base64 encode a string\n ntc qr \"text\" Generate QR code in terminal\n ntc math 3+4*5 Evaluate expression\n ntc ral add btr \"cargo build --release\" Add alias from CLI\n".into());

    m.insert("cli.helpers.shell_header".into(), "SHELL COMMANDS (also work from CLI):".into());

    m.insert("cli.helpers.tagline".into(), "A combine of Navigation, Project Toolkit Center hub.\n".into());

    m.insert("cli.helpers.tp_header".into(), "TELEPORT SAVE POINTS:".into());

    m.insert("cli.helpers.tp_options".into(), " --tp-add <name> Save current directory as teleport point\n --tp-list List all teleport points\n --tp-rm <name> Remove a teleport point\n".into());

    m.insert("cli.helpers.usage_header".into(), "USAGE:".into());

    m.insert("cli.interactive.navigation".into(), " Navigation: cd, back, gos, gosc".into());

    m.insert("cli.interactive.teleport".into(), " Teleport: tp jump, tp to, @name".into());

    m.insert("cli.interactive.tutorial".into(), " Tutorial: tutorial \u{2014} Start interactive step-by-step tutorial\n".into());

    m.insert("cli.known_flag.cared".into(), "--cared <name> Stop ignoring a directory".into());

    m.insert("cli.known_flag.caref".into(), "--caref <ext> Care about a file extension".into());

    m.insert("cli.known_flag.caren".into(), "--caren <file> Care about a specific file".into());

    m.insert("cli.known_flag.clear".into(), "--clear Clear the terminal screen".into());

    m.insert("cli.known_flag.cp".into(), "--cp Copy report to clipboard".into());


    m.insert("cli.known_flag.explorer".into(), "-x, --explorer [PATH] Launch NTC File Explorer TUI".into());

    m.insert("cli.known_flag.generate_completions".into(), "--generate-completions <SHELL> Generate shell completions (bash|zsh|fish|powershell)".into());

    m.insert("cli.known_flag.help_flag".into(), "--help Show help".into());

    m.insert("cli.known_flag.igcare_export_all".into(), "--igcare-export-all <name> Export all ignore/care settings to <name>.ntc.igcare".into());

    m.insert("cli.known_flag.igcare_export_select".into(), "--igcare-export-select <name> Select categories to export to <name>.ntc.igcare".into());

    m.insert("cli.known_flag.igcare_import".into(), "--igcare-import <file> Import ignore/care settings from .ntc.igcare file".into());

    m.insert("cli.known_flag.ignore".into(), "--ignore <name> Ignore a directory name".into());

    m.insert("cli.known_flag.ignored".into(), "--ignored Show ignored items".into());

    m.insert("cli.known_flag.ignoref".into(), "--ignoref <ext> Ignore a file extension".into());

    m.insert("cli.known_flag.ignoren".into(), "--ignoren <file> Ignore a specific file".into());

    m.insert("cli.known_flag.input".into(), "-i, --input <path> Input file or directory".into());

    m.insert("cli.known_flag.list".into(), "--list, --fun List all command-line functions".into());

    m.insert("cli.known_flag.math".into(), "--math <EXPR> Evaluate a math expression (e.g. --math \"3+4*5\")".into());

    m.insert("cli.known_flag.no_args".into(), "(no args) Launch interactive mode".into());

    m.insert("cli.known_flag.output".into(), "-o, --output <file> Output filename".into());

    m.insert("cli.known_flag.ral_export_all".into(), "--ral-export-all <name> Export all run aliases to <name>.ntc.ral".into());

    m.insert("cli.known_flag.ral_export_select".into(), "--ral-export-select <name> Select run aliases to export to <name>.ntc.ral".into());

    m.insert("cli.known_flag.ral_import".into(), "--ral-import <file> Import run aliases from .ntc.ral file".into());

    m.insert("cli.known_flag.say".into(), "-say, -print <text> Print text to stdout".into());

    m.insert("cli.known_flag.setC".into(), "--setC [ON|OFF] Show or set color output".into());

    m.insert("cli.known_flag.setD".into(), "--setD [depth] Show or set max depth (1-20)".into());

    m.insert("cli.known_flag.setH".into(), "--setH [path|ON|OFF] Show or set history path/state".into());

    m.insert("cli.known_flag.setL".into(), "--setL [ON|OFF] Show or set line numbers".into());

    m.insert("cli.known_flag.setO".into(), "--setO [path] Show or set output directory".into());

    m.insert("cli.known_flag.setT".into(), "--setT [threads] Show or set thread count".into());

    m.insert("cli.known_flag.showcg".into(), "--showcg Show current configuration overview".into());

    m.insert("cli.known_flag.size".into(), "--size Show current directory size".into());

    m.insert("cli.known_flag.path".into(), "--path <path|@tp> Target path or teleport savepoint (use with --size or --view)".into());

    m.insert("cli.known_flag.teleport_at".into(), "ntc @<name> Launch and teleport to savepoint".into());

    m.insert("cli.known_flag.tp_add".into(), "--tp-add <name> Save current directory as teleport point".into());

    m.insert("cli.known_flag.tp_list".into(), "--tp-list List all teleport points".into());

    m.insert("cli.known_flag.tp_rm".into(), "--tp-rm <name> Remove teleport point".into());

    m.insert("cli.known_flag.upgrade".into(), "--upgrade Check for updates and upgrade ntc".into());

    m.insert("cli.known_flag.version".into(), "--version Show version information".into());

    m.insert("cli.known_flag.view".into(), "--view Quick view of current directory tree".into());

    m.insert("cli.known_flag.view_size".into(), "--view --size Quick view with directory sizes".into());

    m.insert("cli.known_flag.watch".into(), "--watch [ON|OFF] Show or set file watcher state".into());

    m.insert("cli.known_flag.where".into(), "--where Show ntc executable and config location".into());

    m.insert("cli.logo.c".into(), "C".into());

    m.insert("cli.logo.center".into(), "Center".into());

    m.insert("cli.logo.letters_format".into(), " {} {} {}".into());

    m.insert("cli.logo.n".into(), "N".into());

    m.insert("cli.logo.navigate".into(), "Navigate".into());

    m.insert("cli.logo.subtitle_format".into(), " {} {} {}".into());

    m.insert("cli.logo.t".into(), "T".into());

    m.insert("cli.logo.toolkit".into(), "Toolkit".into());

    m.insert("cli.run.about".into(), "Navigate, Tree, Cat - Directory tree viewer and file concatenator".into());

    m.insert("cli.run.help_cared".into(), "cared".into());

    m.insert("cli.run.help_cared".into(), "cared".into());

    m.insert("cli.run.help_caref".into(), "caref".into());

    m.insert("cli.run.help_caref".into(), "caref".into());

    m.insert("cli.run.help_caren".into(), "caren".into());

    m.insert("cli.run.help_caren".into(), "caren".into());

    m.insert("cli.run.help_clear".into(), "Clear the terminal screen".into());

    m.insert("cli.run.help_copy".into(), "Copy report to clipboard instead of saving to file".into());


    m.insert("cli.run.help_edit".into(), "Open file in built-in text editor".into());

    m.insert("cli.run.help_explorer".into(), "Launch NTC File Explorer TUI (optional starting path)".into());

    m.insert("cli.run.help_format".into(), "Output format: txt, html, json, md".into());

    m.insert("cli.run.help_generate_completions".into(), "Generate shell completion script (bash|zsh|fish|powershell)".into());

    m.insert("cli.run.help_helpextra".into(), "Show extended help for subcommands".into());

    m.insert("cli.run.help_helpextra".into(), "Show extended help for subcommands".into());

    m.insert("cli.run.help_hyper".into(), "Create hyper project scaffold (DSA library)".into());

    m.insert("cli.run.help_igcare_export_all".into(), "Export all ignore/care settings to <name>.ntc.igcare".into());

    m.insert("cli.run.help_igcare_export_select".into(), "Select categories to export to <name>.ntc.igcare".into());

    m.insert("cli.run.help_igcare_import".into(), "Import ignore/care settings from .ntc.igcare file".into());

    m.insert("cli.run.help_ignore".into(), "Show ntc executable location".into());

    m.insert("cli.run.help_ignore".into(), "Show ntc executable location".into());

    m.insert("cli.run.help_ignored".into(), "ignoren".into());

    m.insert("cli.run.help_ignored".into(), "ignoren".into());

    m.insert("cli.run.help_ignoref".into(), "ignoref".into());

    m.insert("cli.run.help_ignoren".into(), "ignoren".into());

    m.insert("cli.run.help_image".into(), "Convert image to document (JSON, MD, HTML, TXT)".into());

    m.insert("cli.run.help_image_to".into(), "Output format for image conversion: json, md, html, txt".into());

    m.insert("cli.run.help_init".into(), "Create starter file from template and open in editor".into());

    m.insert("cli.run.help_input".into(), "Input file or directory path".into());

    m.insert("cli.run.help_lang".into(), "Language for hyper project".into());

    m.insert("cli.run.help_list".into(), "List all command-line functions".into());

    m.insert("cli.run.help_lsp".into(), "Start the ntc-math LSP server over stdio".into());

    m.insert("cli.run.help_lsp_log".into(), "Path to write LSP debug log".into());

    m.insert("cli.run.help_math".into(), "Evaluate a math expression".into());

    m.insert("cli.run.help_mode".into(), "Switch between normal and admin mode".into());

    m.insert("cli.run.help_mode".into(), "Switch between normal and admin mode".into());

    m.insert("cli.run.help_modn".into(), "modn".into());

    m.insert("cli.run.help_modn".into(), "modn".into());

    m.insert("cli.run.help_output".into(), "Output filename for report".into());

    m.insert("cli.run.help_path".into(), "Target path or teleport savepoint (use with --size or --view)".into());

    m.insert("cli.run.help_ral_export_all".into(), "Export all run aliases to <name>.ntc.ral".into());

    m.insert("cli.run.help_ral_export_select".into(), "Select run aliases to export to <name>.ntc.ral".into());

    m.insert("cli.run.help_ral_import".into(), "Import run aliases from .ntc.ral file".into());

    m.insert("cli.run.help_say".into(), "Print text to stdout".into());

    m.insert("cli.run.help_seta".into(), "Show or set autosuggest (ghost text) (ON/OFF)".into());

    m.insert("cli.run.help_setc".into(), "Show or set color output (ON/OFF)".into());

    m.insert("cli.run.help_setd".into(), "Show or set max recursion depth".into());

    m.insert("cli.run.help_seth".into(), "Show or set history (path/ON/OFF/default)".into());

    m.insert("cli.run.help_setl".into(), "Show or set line number display (ON/OFF)".into());

    m.insert("cli.run.help_seto".into(), "Show or set the default output directory".into());

    m.insert("cli.run.help_sett".into(), "Show or set number of threads".into());

    m.insert("cli.run.help_showcg".into(), "Show current configuration overview".into());

    m.insert("cli.run.help_size".into(), "Show current directory size".into());

    m.insert("cli.run.help_time".into(), "Show elapsed time (use with --size or --view)".into());

    m.insert("cli.run.help_tp_add".into(), "Save current directory as teleport point".into());

    m.insert("cli.run.help_tp_list".into(), "List all teleport points".into());

    m.insert("cli.run.help_tp_rm".into(), "Remove teleport point".into());

    m.insert("cli.run.help_upgrade".into(), "Check for updates and upgrade ntc to the latest version".into());

    m.insert("cli.run.help_version".into(), "Show version information".into());

    m.insert("cli.run.help_view".into(), "Quick view of current directory tree".into());

    m.insert("cli.run.help_watch".into(), "Show or set file watcher (ON/OFF)".into());

    m.insert("cli.run.help_where".into(), "Show ntc executable location".into());

    m.insert("cli.section.config".into(), "CONFIGURATION OPTIONS:".into());

    m.insert("cli.section.editor".into(), "EDITOR OPTIONS:".into());

    m.insert("cli.section.examples".into(), "EXAMPLES:".into());

    m.insert("cli.section.export".into(), "ALIAS / IGNORE/CARE EXPORT:".into());

    m.insert("cli.section.igcare".into(), "IGNORE/CARE OPTIONS:".into());

    m.insert("cli.section.interactive_only".into(), "INTERACTIVE-ONLY (run 'ntc' with no args):".into());

    m.insert("cli.section.other".into(), "OTHER OPTIONS:".into());

    m.insert("cli.section.report".into(), "REPORT OPTIONS:".into());

    m.insert("cli.section.shell_commands".into(), "SHELL COMMANDS (also work from CLI):".into());

    m.insert("cli.section.teleport".into(), "TELEPORT SAVE POINTS:".into());

    m.insert("cli.shell_cmd.dl".into(), " ntc dl https://example.com/file.zip --progress Download with progress".into());

    m.insert("cli.shell_cmd.encode".into(), " ntc encode b64 hello Base64 encode a string".into());

    m.insert("cli.shell_cmd.math".into(), " ntc math 3+4*5 Evaluate expression".into());

    m.insert("cli.shell_cmd.ne".into(), " ntc ne Open text editor".into());

    m.insert("cli.shell_cmd.pathcp".into(), " ntc pathcp Copy path to clipboard".into());

    m.insert("cli.shell_cmd.qr".into(), " ntc qr \"text\" Generate QR code in terminal".into());

    m.insert("cli.shell_cmd.ral_add".into(), " ntc ral add btr \"cargo build --release\" Add alias from CLI\n".into());

    m.insert("cli.shell_cmd.say_loop".into(), " ntc say \"hello\" --loop 3 Print text with loop".into());

    m.insert("cli.shell_cmd.serve".into(), " ntc serve 8080 Start HTTP server on port 8080".into());

    m.insert("cli.shell_cmd.size".into(), " ntc size Show directory size".into());

    m.insert("cli.shell_cmd.tl".into(), " ntc tl dir . Count lines in directory".into());

    m.insert("cli.shell_cmd.view".into(), " ntc view Show directory tree".into());

    m.insert("cli.usage.basic".into(), " ntc [OPTIONS]".into());

    m.insert("cli.usage.clipboard".into(), " ntc -i <path> --cp Copy report to clipboard\n".into());

    m.insert("cli.usage.header".into(), "USAGE:".into());

    m.insert("cli.usage.input".into(), " ntc -i <path> [-o <output>]".into());

    m.insert("cmd.care.dir_desc".into(), "care dir <name>[, ...] Stop ignoring directories".into());

    m.insert("cmd.care.dir_usage".into(), "Usage: care dir <name>[, <name>, ...]".into());

    m.insert("cmd.care.ext_desc".into(), "care ext <ext>[, ...] Add extra supported extensions".into());

    m.insert("cmd.care.ext_usage".into(), "Usage: care ext <ext>[, <ext>, ...]".into());

    m.insert("cmd.care.file_desc".into(), "care file <name>[, ...] Add extra supported files".into());

    m.insert("cmd.care.file_usage".into(), "Usage: care file <name>[, <name>, ...]".into());

    m.insert("cmd.care.title".into(), "care \u{2014} Manage cared (un-ignored) items".into());

    m.insert("cmd.clipboard.access_fail".into(), "Failed to access clipboard: {}".into());

    m.insert("cmd.clipboard.already_empty".into(), "Clipboard history is already empty.".into());

    m.insert("cmd.clipboard.clear_cancelled".into(), "Clear cancelled.".into());

    m.insert("cmd.clipboard.clear_prompt".into(), "Clear cancelled.".into());

    m.insert("cmd.clipboard.clear_prompt".into(), "Clear cancelled.".into());

    m.insert("cmd.clipboard.cleared".into(), "Clipboard history cleared.".into());

    m.insert("cmd.clipboard.copied_entry".into(), "Copied entry #{} ({}) to clipboard.".into());

    m.insert("cmd.clipboard.empty".into(), "Clipboard history is empty.".into());

    m.insert("cmd.clipboard.entry_not_found".into(), "Entry #{} not found (have {} entries).".into());

    m.insert("cmd.clipboard.help_clear".into(), "  clip clear         Clear clipboard history".into());

    m.insert("cmd.clipboard.help_list".into(), "  clip               Show clipboard history".into());

    m.insert("cmd.clipboard.help_select".into(), "  clip select <N>    Copy entry N back to clipboard".into());

    m.insert("cmd.clipboard.history_header".into(), "  Clipboard History (last {}):".into());

    m.insert("cmd.clipboard.select_usage".into(), "Usage: clip select <N>".into());

    m.insert("cmd.clipboard.set_fail".into(), "Failed to set clipboard: {}".into());

    m.insert("cmd.clipboard.termux_unavailable".into(), "termux-clipboard-set not available: {}".into());

    m.insert("cmd.clipboard.unknown_sub".into(), "Unknown clip subcommand: {}".into());

    m.insert("cmd.completed".into(), "Command completed successfully.".into());

    m.insert("cmd.config.deinit.confirm".into(), "Delete {}? (y/N): ".into());

    m.insert("cmd.config.deinit.local_not_found".into(), "No ntconfig.toml found, skipping.".into());

    m.insert("cmd.config.deinit.local_remove_failed".into(), "Failed to remove ntconfig.toml: {}".into());

    m.insert("cmd.config.deinit.local_removed".into(), "ntconfig.toml removed.".into());

    m.insert("cmd.config.deinit.not_found".into(), "No {} found, skipping.".into());

    m.insert("cmd.config.deinit.remove_failed".into(), "Failed to remove {}: {}".into());

    m.insert("cmd.config.deinit.removed".into(), "Removed {}".into());

    m.insert("cmd.config.deinit.skipped".into(), "Skipped.".into());

    m.insert("cmd.config.deinit.usage".into(), "Usage: deinit [--all | --ran | --local]".into());

    m.insert("cmd.config.example_depth".into(), " config depth 4  # Set max tree depth to 4".into());

    m.insert("cmd.config.example_history".into(), " config history off  # Disable history".into());

    m.insert("cmd.config.example_linenumbers".into(), " config linenumbers ON  # Enable line numbers in file display".into());

    m.insert("cmd.config.example_output".into(), " config output C:\\reports  # Set output path".into());

    m.insert("cmd.config.example_watch".into(), " config watch ON  # Enable file watcher".into());

    m.insert("cmd.config.examples_header".into(), "Examples:".into());

    m.insert("cmd.config.init.create_failed".into(), "Failed to create {}: {}".into());

    m.insert("cmd.config.init.created".into(), "Created {}".into());

    m.insert("cmd.config.init.exists_warn".into(), "{} already exists, skipping.".into());

    m.insert("cmd.config.init.local_exists".into(), "ntconfig.toml already exists, skipping.".into());

    m.insert("cmd.config.init.usage".into(), "Usage: init [--all | --ran | --local]".into());

    m.insert("cmd.config.lang.active".into(), "{} (active)".into());

    m.insert("cmd.config.lang.available".into(), "Available languages:".into());

    m.insert("cmd.config.lang.coming_soon".into(), "{} (set, coming soon)".into());

    m.insert("cmd.config.lang.set".into(), "Language set to: {}".into());

    m.insert("cmd.config.lang.set_coming_soon".into(), "Language set to: {} (coming soon)".into());

    m.insert("cmd.config.lang.unsupported".into(), "Unsupported language '{}'. Available: {}".into());

    m.insert("cmd.config.local.create_failed".into(), "Failed to create ntconfig.toml: {}".into());

    m.insert("cmd.config.local.created_export".into(), "Created ntconfig.toml with current settings in: {}".into());

    m.insert("cmd.config.local.created_template".into(), "Created ntconfig.toml template in: {}".into());

    m.insert("cmd.config.local.exists_warning".into(), "\u{26a0}\u{fe0f} ntconfig.toml already exists!".into());

    m.insert("cmd.config.local.export_aliases".into(), " \u{2022} Run aliases".into());

    m.insert("cmd.config.local.export_extras".into(), " \u{2022} Extra supported files/extensions".into());

    m.insert("cmd.config.local.export_ignores".into(), " \u{2022} Ignored directories/extensions/files".into());

    m.insert("cmd.config.local.export_valid".into(), "\u{2705} Active configuration exported with valid TOML syntax:".into());

    m.insert("cmd.config.local.init_cancelled".into(), "Initialisation cancelled.".into());

    m.insert("cmd.config.local.overwrite_prompt".into(), "Overwrite? (y/N): ".into());

    m.insert("cmd.config.local.template_all_hint".into(), " Or run: local init --all to export your current settings".into());

    m.insert("cmd.config.local.template_edit_hint".into(), " Edit the file and remove '#' from lines you want to enable".into());

    m.insert("cmd.config.local.template_hint".into(), "\u{1f4dd} This is a COMMENTED TEMPLATE - no active settings yet".into());

    m.insert("cmd.config.local.tip_global".into(), "\u{1f4a1} Tip: Global settings (teleports, output path, depth) stay global".into());

    m.insert("cmd.config.local_deinit.cancelled".into(), "Deinitialisation cancelled.".into());

    m.insert("cmd.config.local_deinit.confirm".into(), "Delete ntconfig.toml? (y/N): ".into());

    m.insert("cmd.config.local_deinit.done".into(), "ntconfig.toml removed. Global config is now the only active config.".into());

    m.insert("cmd.config.local_deinit.global_safe".into(), "Global config will NOT be affected.".into());

    m.insert("cmd.config.local_deinit.lose_settings".into(), "Local ignore/care and run alias settings will be lost.".into());

    m.insert("cmd.config.local_deinit.not_found".into(), "No ntconfig.toml found in the current directory.".into());

    m.insert("cmd.config.local_deinit.remove_failed".into(), "Failed to remove ntconfig.toml: {}".into());

    m.insert("cmd.config.local_deinit.title".into(), "\u{26A0}\u{FE0F} LOCAL CONFIG REMOVAL".into());

    m.insert("cmd.config.local_deinit.will_delete".into(), "This will delete: {}".into());

    m.insert("cmd.config.opencg.created".into(), "Created config file at: {}".into());

    m.insert("cmd.config.opencg.opening".into(), "Opening config: {}".into());

    m.insert("cmd.config.qsc.disabled".into(), "QSC hotkey disabled.".into());

    m.insert("cmd.config.qsc.hotkey_set".into(), "QSC hotkey set to: {}".into());

    m.insert("cmd.config.qsc.hotkey_status".into(), "QSC hotkey: {}".into());

    m.insert("cmd.config.qsc.invalid_format".into(), "Invalid QSC format: '{}'. Use '#c #s 0' style.".into());

    m.insert("cmd.config.qsc.not_configured".into(), "not configured".into());

    m.insert("cmd.config.qsc.usage1".into(), "Usage: config qsc \"#c 0\"".into());

    m.insert("cmd.config.qsc.usage2".into(), " config qsc \"#c #s a\"".into());

    m.insert("cmd.config.qsc.usage3".into(), " config qsc \"#c #s #a F1\"".into());

    m.insert("cmd.config.qsc.usage4".into(), " config qsc off".into());

    m.insert("cmd.config.resetcg.backup_created".into(), "Backup created: {}".into());

    m.insert("cmd.config.resetcg.backup_failed".into(), "Failed to create backup: {}".into());

    m.insert("cmd.config.resetcg.backup_info".into(), "Backup saved. Use 'restorecg' to restore if needed.".into());

    m.insert("cmd.config.resetcg.cancelled".into(), "Reset cancelled.".into());

    m.insert("cmd.config.resetcg.config_file_label".into(), "Config file:".into());

    m.insert("cmd.config.resetcg.default_aliases".into(), " \u{2022} Run aliases \u{2192} Cleared".into());

    m.insert("cmd.config.resetcg.default_custom_ignores".into(), " \u{2022} Custom ignores \u{2192} Removed".into());

    m.insert("cmd.config.resetcg.default_depth".into(), " \u{2022} Max depth \u{2192} 2".into());

    m.insert("cmd.config.resetcg.default_history".into(), " \u{2022} History \u{2192} OFF".into());

    m.insert("cmd.config.resetcg.default_ignored_dirs".into(), " \u{2022} Ignored dirs \u{2192} target, build, venv, node_modules, installer, logs, .git".into());

    m.insert("cmd.config.resetcg.default_linenumbers".into(), " \u{2022} Line numbers \u{2192} OFF".into());

    m.insert("cmd.config.resetcg.default_output".into(), " \u{2022} Output path \u{2192} Desktop".into());

    m.insert("cmd.config.resetcg.default_teleports".into(), " \u{2022} Teleports \u{2192} Cleared".into());

    m.insert("cmd.config.resetcg.default_threads".into(), " \u{2022} Threads \u{2192} 4".into());

    m.insert("cmd.config.resetcg.default_watcher".into(), " \u{2022} File watcher \u{2192} OFF".into());

    m.insert("cmd.config.resetcg.not_found".into(), "Config file not found. Nothing to reset.".into());

    m.insert("cmd.config.resetcg.opt_backup".into(), " \u{2022} Type 'backup' - Create backup before reset".into());

    m.insert("cmd.config.resetcg.opt_no".into(), " \u{2022} Type 'no' - Cancel".into());

    m.insert("cmd.config.resetcg.opt_yes".into(), " \u{2022} Type 'yes' - Reset config (no backup)".into());

    m.insert("cmd.config.resetcg.options_label".into(), "Options:".into());

    m.insert("cmd.config.resetcg.reset_done".into(), "Config reset to defaults!".into());

    m.insert("cmd.config.resetcg.serialize_failed".into(), "Failed to serialize default config".into());

    m.insert("cmd.config.resetcg.warning_desc".into(), "This will RESET your configuration to DEFAULT values:".into());

    m.insert("cmd.config.resetcg.warning_title".into(), "\u{26A0}\u{FE0F} CONFIG RESET WARNING".into());

    m.insert("cmd.config.resetcg.write_failed".into(), "Failed to write config: {}".into());

    m.insert("cmd.config.resetcg.your_choice".into(), "Your choice: ".into());

    m.insert("cmd.config.restorecg.backed_up".into(), "Current config backed up to: {}".into());

    m.insert("cmd.config.restorecg.cancel_opt".into(), "0. Cancel".into());

    m.insert("cmd.config.restorecg.cancelled".into(), "Restore cancelled.".into());

    m.insert("cmd.config.restorecg.failed".into(), "Failed to restore: {}".into());

    m.insert("cmd.config.restorecg.invalid".into(), "Invalid selection.".into());

    m.insert("cmd.config.restorecg.none".into(), "No backup files found.".into());

    m.insert("cmd.config.restorecg.reloaded".into(), "Config reloaded! Run 'showcg' to see the restored settings.".into());

    m.insert("cmd.config.restorecg.restored".into(), "Restored config from: {}".into());

    m.insert("cmd.config.restorecg.title".into(), "\u{1F4CB} Available Backups:".into());

    m.insert("cmd.config.mgmt_header".into(), "Management:".into());

    m.insert("cmd.config.settings_header".into(), "Settings:".into());

    m.insert("cmd.config.showcg.lang_coming_soon".into(), "{} (coming soon)".into());

    m.insert("cmd.config.showcg.lang_label".into(), "Language:".into());

    m.insert("cmd.config.showcg.qsc_label".into(), "QSC Hotkey:".into());

    m.insert("cmd.config.title".into(), "config \u{2014} View or change configuration settings".into());

    m.insert("cmd.config.watch.alias_not_found".into(), "Alias '{}' not found. Use 'ral list' to see aliases.".into());

    m.insert("cmd.config.watch.trigger_cleared".into(), "Watch trigger alias cleared.".into());

    m.insert("cmd.config.watch.trigger_label".into(), "Trigger alias:".into());

    m.insert("cmd.config.watch.trigger_off_usage".into(), "config watch trigger off Disable auto-run".into());

    m.insert("cmd.config.watch.trigger_set".into(), "Watch trigger set to alias: '{}'".into());

    m.insert("cmd.config.watch.trigger_usage".into(), "config watch trigger <alias> Auto-run alias on change".into());

    m.insert("cmd.config.watch.limit_usage".into(), "config watch limit <n> Rows shown in change box".into());

    m.insert("cmd.config.watch.usage".into(), "Use: config watch ON | config watch OFF | config watch trigger <alias> | config watch trigger off | config watch limit <n>".into());

    m.insert("cmd.config.watch.usage_line".into(), "Usage: config watch ON|OFF".into());

    m.insert("cmd.fmt.file_not_found".into(), "File not found: {}".into());

    m.insert("cmd.fmt.invalid_json".into(), "Invalid JSON: {}".into());

    m.insert("cmd.fmt.invalid_toml".into(), "Invalid TOML: {}".into());

    m.insert("cmd.fmt.invalid_yaml".into(), "Invalid YAML: {}".into());

    m.insert("cmd.fmt.json_ser_err".into(), "JSON serialization error: {}".into());

    m.insert("cmd.fmt.json_ser_err".into(), "JSON serialization error: {}".into());

    m.insert("cmd.fmt.missing_file".into(), "Missing file path".into());

    m.insert("cmd.fmt.read_fail".into(), "Failed to read file: {}".into());

    m.insert("cmd.fmt.toml_conv_err".into(), "TOML conversion error: {}".into());

    m.insert("cmd.fmt.toml_ser_err".into(), "TOML serialization error: {}".into());

    m.insert("cmd.fmt.unknown_format".into(), "Unknown format: '{}'. Use json, yaml, or toml.".into());

    m.insert("cmd.fmt.unknown_target".into(), "Unknown target format: '{}'. Use json, yaml, or toml.".into());

    m.insert("cmd.fmt.usage".into(), "Usage: fmt json|yaml|toml <file> [to json|yaml|toml] [-o <file>]".into());

    m.insert("cmd.fmt.written_to".into(), "Written to: {}".into());

    m.insert("cmd.fmt.yaml_ser_err".into(), "YAML serialization error: {}".into());

    m.insert("cmd.gs.no_matches".into(), "No matches found.".into());

    m.insert("cmd.http.authorization_label".into(), "Authorization:".into());

    m.insert("cmd.http.authorization_label".into(), "Authorization:".into());

    m.insert("cmd.http.basic_placeholder".into(), "Basic <encoded>".into());

    m.insert("cmd.http.basic_placeholder".into(), "Basic <encoded>".into());

    m.insert("cmd.http.bearer_placeholder".into(), "Bearer <token>".into());

    m.insert("cmd.http.bearer_placeholder".into(), "Bearer <token>".into());

    m.insert("cmd.http.byte_count".into(), "{} bytes".into());

    m.insert("cmd.http.byte_count".into(), "{} bytes".into());

    m.insert("cmd.http.help_auth".into(), "  -a, --auth <user:pass>         Basic authentication".into());

    m.insert("cmd.http.help_bearer".into(), "  --bearer <token>               Bearer token authentication".into());

    m.insert("cmd.http.help_body".into(), "  -d, --data <body>              Request body".into());

    m.insert("cmd.http.help_example1".into(), "  http GET https://api.github.com/users/octocat".into());

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert("cmd.http.help_example3".into(), "  http GET https://api.example.com/search -q q=hello -q page=1 -s > results.json".into());

    m.insert("cmd.http.help_example4".into(), "  http GET https://api.example.com/data --bearer mytoken -o response.json".into());

    m.insert("cmd.http.help_example5".into(), "  http GET https://api.example.com/health -s -t 5".into());

    m.insert("cmd.http.help_example6".into(), "  http GET https://api.example.com/users -a admin:secret123".into());

    m.insert("cmd.http.help_examples_title".into(), "Examples:".into());

    m.insert("cmd.http.help_examples_title".into(), "Examples:".into());

    m.insert("cmd.http.help_header".into(), "  -H, --header <\\\"Key: Value\\\">     Add request header (repeatable)".into());

    m.insert("cmd.http.help_help".into(), "  -h, --help                     Show this help".into());

    m.insert("cmd.http.help_json".into(), "  --json <data>                  Send as JSON (auto-sets Content-Type)".into());

    m.insert("cmd.http.help_methods".into(), "  GET, POST, PUT, DELETE, PATCH, HEAD".into());

    m.insert("cmd.http.help_methods_title".into(), "Methods:".into());

    m.insert("cmd.http.help_methods_title".into(), "Methods:".into());

    m.insert("cmd.http.help_no_pretty".into(), "  --no-pretty                    Disable JSON pretty-printing".into());

    m.insert("cmd.http.help_options_title".into(), "Options:".into());

    m.insert("cmd.http.help_options_title".into(), "Options:".into());

    m.insert("cmd.http.help_output".into(), "  -o, --output <file>            Save body to file (raw)".into());

    m.insert("cmd.http.help_query".into(), "  -q, --query <key=value>        URL query parameter (repeatable)".into());

    m.insert("cmd.http.help_silent".into(), "  -s, --silent                   Show body only (no status/headers)".into());

    m.insert("cmd.http.help_timeout".into(), "  -t, --timeout <secs>           Request timeout (default 30)".into());

    m.insert("cmd.http.help_title".into(), "HTTP API Client".into());

    m.insert("cmd.http.help_title".into(), "HTTP API Client".into());

    m.insert("cmd.http.help_usage_default".into(), "  <url>                          (GET by default if URL is first argument)".into());

    m.insert("cmd.http.help_usage_line".into(), "  http <method> <url> [options]".into());

    m.insert("cmd.http.help_usage_title".into(), "Usage:".into());

    m.insert("cmd.http.help_usage_title".into(), "Usage:".into());

    m.insert("cmd.http.help_verbose".into(), "  -v, --verbose                  Show outgoing request details".into());

    m.insert("cmd.http.invalid_body".into(), "Invalid body: {}".into());

    m.insert("cmd.http.invalid_body".into(), "Invalid body: {}".into());

    m.insert("cmd.http.invalid_header".into(), "Invalid header format, expected \"Key: Value\", got: {}".into());

    m.insert("cmd.http.invalid_header".into(), "Invalid header format, expected \"Key: Value\", got: {}".into());

    m.insert("cmd.http.invalid_query".into(), "Invalid query format, expected key=value, got: {}".into());

    m.insert("cmd.http.invalid_query".into(), "Invalid query format, expected key=value, got: {}".into());

    m.insert("cmd.http.invalid_request".into(), "Invalid request: {}".into());

    m.insert("cmd.http.invalid_request".into(), "Invalid request: {}".into());

    m.insert("cmd.http.invalid_timeout".into(), "Invalid timeout: {}".into());

    m.insert("cmd.http.invalid_timeout".into(), "Invalid timeout: {}".into());

    m.insert("cmd.http.missing_auth".into(), "Missing user:pass after -a".into());

    m.insert("cmd.http.missing_auth".into(), "Missing user:pass after -a".into());

    m.insert("cmd.http.missing_bearer".into(), "Missing token after --bearer".into());

    m.insert("cmd.http.missing_bearer".into(), "Missing token after --bearer".into());

    m.insert("cmd.http.missing_body_data".into(), "Missing body data after -d".into());

    m.insert("cmd.http.missing_body_data".into(), "Missing body data after -d".into());

    m.insert("cmd.http.missing_filename".into(), "Missing filename after -o".into());

    m.insert("cmd.http.missing_filename".into(), "Missing filename after -o".into());

    m.insert("cmd.http.missing_header".into(), "Missing header value after -H".into());

    m.insert("cmd.http.missing_header".into(), "Missing header value after -H".into());

    m.insert("cmd.http.missing_json".into(), "Missing JSON data after --json".into());

    m.insert("cmd.http.missing_json".into(), "Missing JSON data after --json".into());

    m.insert("cmd.http.missing_query".into(), "Missing key=value after -q".into());

    m.insert("cmd.http.missing_query".into(), "Missing key=value after -q".into());

    m.insert("cmd.http.missing_timeout".into(), "Missing seconds after -t".into());

    m.insert("cmd.http.missing_timeout".into(), "Missing seconds after -t".into());

    m.insert("cmd.http.request_failed".into(), "{} {} failed".into());

    m.insert("cmd.http.request_failed".into(), "{} {} failed".into());

    m.insert("cmd.http.saved_bytes".into(), "Saved {} bytes to {}".into());

    m.insert("cmd.http.saved_bytes".into(), "Saved {} bytes to {}".into());

    m.insert("cmd.http.status_line".into(), "{} {} {}  ".into());

    m.insert("cmd.http.status_line".into(), "{} {} {}  ".into());

    m.insert("cmd.http.unknown_status".into(), "Unknown".into());

    m.insert("cmd.http.unknown_status".into(), "Unknown".into());

    m.insert("cmd.http.url_required".into(), "URL is required. Usage: http <method> <url>".into());

    m.insert("cmd.http.url_required".into(), "URL is required. Usage: http <method> <url>".into());

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.write_fail".into(), "Failed to write to {}".into());

    m.insert("cmd.http.write_fail".into(), "Failed to write to {}".into());

    m.insert("cmd.igcar.main_title".into(), "Ignore/Care (igcare) Commands:".into());

    m.insert("cmd.igcare.create_failed".into(), "Failed to create file: {}".into());

    m.insert("cmd.igcare.example_export_all".into(), "  igcare export --all myproject        # Creates myproject.ntc.igcare".into());

    m.insert("cmd.igcare.example_export_select".into(), "  igcare export --select myproject     # Pick which categories to export".into());

    m.insert("cmd.igcare.example_import".into(), "  igcare import myproject.ntc.igcare   # Import into current config".into());

    m.insert("cmd.igcare.examples_title".into(), "Examples:".into());

    m.insert("cmd.igcare.examples_title".into(), "Examples:".into());

    m.insert("cmd.igcare.export_cmd".into(), "igcare export --all <name> Export all settings to <name>.ntc.igcare".into());

    m.insert("cmd.igcare.export_name_empty".into(), "Export name cannot be empty.".into());

    m.insert("cmd.igcare.export_select_cmd".into(), "igcare export --select <name> Select categories to export to <name>.ntc.igcare".into());

    m.insert("cmd.igcare.export_usage".into(), "Usage: igcare export --all <name> | igcare export --select <name>".into());

    m.insert("cmd.igcare.file_exists".into(), "File already exists: {}".into());

    m.insert("cmd.igcare.import_cmd".into(), "igcare import <file> Import settings from a .ntc.igcare file".into());

    m.insert("cmd.igcare.import_example".into(), "Example: igcare import myproject.ntc.igcare".into());

    m.insert("cmd.igcare.import_usage".into(), "Usage: igcare import <file>".into());

    m.insert("cmd.igcare.main_title".into(), "Ignore/Care Manager".into());

    m.insert("cmd.igcare.main_title".into(), "Ignore/Care Manager".into());

    m.insert("cmd.igcare.template_created".into(), "Created igcare template: {}".into());

    m.insert("cmd.igcare.template_hint".into(), "This is a local template file. Use 'igcare import' to apply settings.".into());

    m.insert("cmd.igcare.type_help".into(), "Type 'igcare' for help.".into());

    m.insert("cmd.igcare.unknown_flag".into(), "Unknown export flag: {}. Use --all or --select".into());

    m.insert("cmd.igcare.unknown_subcmd".into(), "Unknown igcare subcommand: {}".into());

    m.insert("cmd.igcared.combined_hint".into(), "Global + Local settings combined:".into());

    m.insert("cmd.igcared.combined_hint".into(), "Global + Local settings combined:".into());

    m.insert("cmd.igcared.extra_exts".into(), "Extra extensions: {:?}".into());

    m.insert("cmd.igcared.extra_exts".into(), "Extra extensions: {:?}".into());

    m.insert("cmd.igcared.extra_files".into(), "Extra files: {:?}".into());

    m.insert("cmd.igcared.extra_files".into(), "Extra files: {:?}".into());

    m.insert("cmd.igcared.ignored_dirs".into(), "Ignored directories: {:?}".into());

    m.insert("cmd.igcared.ignored_dirs".into(), "Ignored directories: {:?}".into());

    m.insert("cmd.igcared.ignored_exts".into(), "Ignored extensions: {:?}".into());

    m.insert("cmd.igcared.ignored_exts".into(), "Ignored extensions: {:?}".into());

    m.insert("cmd.igcared.ignored_files".into(), "Ignored files: {:?}".into());

    m.insert("cmd.igcared.ignored_files".into(), "Ignored files: {:?}".into());

    m.insert("cmd.igcared.local_config".into(), "   Local config: {}".into());

    m.insert("cmd.igcared.local_config".into(), "   Local config: {}".into());

    m.insert("cmd.igcared.local_tip".into(), "Run 'igcared' to view combined settings".into());

    m.insert("cmd.igcared.local_tip2".into(), "Use 'igcared' to see current state".into());

    m.insert("cmd.igcared.separator".into(), "─".into());

    m.insert("cmd.igcared.separator".into(), "─".into());

    m.insert("cmd.igcared.title_combined".into(), "Combined Settings".into());

    m.insert("cmd.igcared.title_combined".into(), "Combined Settings".into());

    m.insert("cmd.igcared.title_global".into(), "Global Settings".into());

    m.insert("cmd.igcared.title_global".into(), "Global Settings".into());

    m.insert("cmd.ignore.applied".into(), "{} {} item(s)".into());

    m.insert("cmd.ignore.care_example_dir".into(), "  care dir build,dist".into());

    m.insert("cmd.ignore.care_example_ext".into(), "  care ext txt,md,json".into());

    m.insert("cmd.ignore.care_example_ext".into(), "  care ext txt,md,json".into());

    m.insert("cmd.ignore.care_example_file".into(), "  care file .env".into());

    m.insert("cmd.ignore.care_examples_title".into(), "Care examples:".into());

    m.insert("cmd.ignore.care_examples_title".into(), "Care examples:".into());

    m.insert("cmd.ignore.check_hint".into(), "Run 'igcared' to see current ignore/care settings".into());

    m.insert("cmd.ignore.dir_desc".into(), "ignore dir <name>[, ...] Ignore directories".into());

    m.insert("cmd.ignore.dir_usage".into(), "Usage: ignore dir <name>[, <name>, ...]".into());

    m.insert("cmd.ignore.example_dir".into(), "  ignore dir temp,logs".into());

    m.insert("cmd.ignore.example_ext".into(), "  ignore ext log,tmp,bak".into());

    m.insert("cmd.ignore.example_file".into(), "  ignore file Cargo.lock".into());

    m.insert("cmd.ignore.examples_title".into(), "Examples:".into());

    m.insert("cmd.ignore.examples_title".into(), "Examples:".into());

    m.insert("cmd.ignore.ext_desc".into(), "ignore ext <ext>[, ...] Ignore file extensions".into());

    m.insert("cmd.ignore.ext_usage".into(), "Usage: ignore ext <ext>[, <ext>, ...]".into());

    m.insert("cmd.ignore.file_desc".into(), "ignore file <name>[, ...] Ignore specific files".into());

    m.insert("cmd.ignore.file_usage".into(), "Usage: ignore file <name>[, <name>, ...]".into());

    m.insert("cmd.ignore.title".into(), "ignore \u{2014} Manage ignored items".into());

    m.insert("cmd.image.cancelled".into(), "Cancelled.".into());

    m.insert("cmd.image.cancelled".into(), "Cancelled.".into());

    m.insert("cmd.image.default_label".into(), "Default".into());

    m.insert("cmd.image.default_label".into(), "Default".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. Cancel".into());

    m.insert("cmd.image.desc_html".into(), "  {}. HTML".into());

    m.insert("cmd.image.desc_json".into(), "  {}. JSON".into());

    m.insert("cmd.image.desc_markdown".into(), "  {}. Markdown (default)".into());

    m.insert("cmd.image.desc_narrative".into(), "  {}. Narrative description (for AI)".into());

    m.insert("cmd.image.desc_txt".into(), "  {}. TXT".into());

    m.insert("cmd.image.enter_number".into(), "Enter the number: ".into());

    m.insert("cmd.image.enter_number".into(), "Enter the number: ".into());

    m.insert("cmd.image.missing_out_format".into(), "Missing format after 'out'.".into());

    m.insert("cmd.image.missing_to_format".into(), "Missing format after 'to'.".into());

    m.insert("cmd.image.no_files".into(), "No image files found in current directory.".into());

    m.insert("cmd.image.displaying".into(), "Displaying {} ({}x{}) via {}".into());

    m.insert("cmd.image.select_image".into(), "Select image:".into());

    m.insert("cmd.image.select_image".into(), "Select image:".into());

    m.insert("cmd.image.usage".into(), "Usage: image <path> [display|show|to|out <format>] [--ocr] [--layout]".into());

    m.insert("cmd.ipcfg.dns_label".into(), "DNS ......... :".into());

    m.insert("cmd.ipcfg.no_adapters".into(), "No network adapters found.".into());

    m.insert("cmd.ipcfg.no_such_adapter".into(), "No such adapter: '{}'.".into());

    m.insert("cmd.ipcfg.title".into(), "Network adapters:".into());

    m.insert("cmd.image_disabled".into(), "Image conversion not available (feature 'image2doc' disabled).".into());

    m.insert("cmd.mode.admin_relaunch".into(), "Re-launching ntc as Administrator...".into());

    m.insert("cmd.mode.elevate_fail".into(), "Failed to launch elevated process: {}".into());

    m.insert("cmd.mode.exe_fail".into(), "Failed to get executable path: {}".into());

    m.insert("cmd.mode.launch_fail".into(), "Failed to launch process: {}".into());

    m.insert("cmd.mode.normal_relaunch".into(), "Re-launching ntc in normal mode...".into());

    m.insert("cmd.mode.windows_only".into(), "mode is a Windows-only feature.".into());

    m.insert("cmd.nav.back.invalid_number".into(), "Invalid number: {}. Usage: back [n]".into());

    m.insert("cmd.nav.back.invalid_step".into(), "Null parent at step {} - nowhere to go back".into());

    m.insert("cmd.nav.cd.example1".into(), "Example: cd C:\\Users".into());

    m.insert("cmd.nav.cd.example2".into(), " cd subdir".into());

    m.insert("cmd.nav.cd.example3".into(), " cd to work # Teleport to 'work' savepoint".into());

    m.insert("cmd.nav.cd.navigated".into(), "Navigated to: {}".into());

    m.insert("cmd.nav.cd.tp_list".into(), "Use 'tp list' to see all savepoints.".into());

    m.insert("cmd.nav.cd.tp_not_found".into(), "Teleport point not found: '{}'".into());

    m.insert("cmd.nav.cd.usage1".into(), "Usage: cd <directory_path>".into());

    m.insert("cmd.nav.cd.usage2".into(), " cd to <tp_name> Teleport to savepoint".into());

    m.insert("cmd.nav.drive_windows_only".into(), "Drive navigation is only supported on Windows.".into());

    m.insert("cmd.nav.fgo.cancel".into(), "0. Cancel".into());

    m.insert("cmd.nav.fgo.cancelled".into(), "Cancelled.".into());

    m.insert("cmd.nav.fgo.example1".into(), " fgo main.rs # search then pick file to navigate to its parent".into());

    m.insert("cmd.nav.fgo.example2".into(), " fgo main.rs -d 5 # search up to 5 levels deep".into());

    m.insert("cmd.nav.fgo.found".into(), "\u{1f50d} Found {} file(s) for \"{}\":".into());

    m.insert("cmd.nav.fgo.select".into(), "Select file to navigate to its parent directory: ".into());

    m.insert("cmd.nav.fgo_usage".into(), "Usage: fgo <pattern> [-d <depth>]".into());

    m.insert("cmd.nav.gos.empty".into(), "(no subdirectories)".into());

    m.insert("cmd.nav.gos.exit".into(), "exit".into());

    m.insert("cmd.nav.gos.staying".into(), "Staying here.".into());

    m.insert("cmd.nav.invalid_drive".into(), "Invalid drive number.".into());

    m.insert("cmd.nav.invalid_input".into(), "Invalid input.".into());

    m.insert("cmd.nav.invalid_number".into(), "Invalid number.".into());

    m.insert("cmd.nav.invalid_selection".into(), "Invalid selection.".into());

    m.insert("cmd.nav.where.config_file".into(), "\u{2699}\u{fe0f} Config file:".into());

    m.insert("cmd.nav.where.config_missing".into(), "\u{26a0} Config file not found (will be created on first save)".into());

    m.insert("cmd.nav.where.config_size".into(), "\u{1f4cf} Config size:".into());

    m.insert("cmd.nav.where.current_dir".into(), "\u{1f4c2} Current dir:".into());

    m.insert("cmd.nav.where.executable".into(), "\u{1f4c1} Executable:".into());

    m.insert("cmd.nav.where.title".into(), "ntc Location Info".into());

    m.insert("cmd.pathman.access_denied".into(), "Access denied — administrator privileges required.".into());

    m.insert("cmd.pathman.added_to_path".into(), "Added to {}: {}".into());

    m.insert("cmd.pathman.already_in_path".into(), "Already in PATH: {}".into());

    m.insert("cmd.pathman.empty_path".into(), "║  {}".into());

    m.insert("cmd.pathman.example_add".into(), "  {} add C:\\Tools".into());

    m.insert("cmd.pathman.example_add_cwd".into(), "  {} add --cwd".into());

    m.insert("cmd.pathman.example_move".into(), "  {} move 3 1               Move entry #3 to the top".into());

    m.insert("cmd.pathman.example_rm_idx".into(), "  {} rm 3                    Remove entry #3 from the list".into());

    m.insert("cmd.pathman.example_rm_path".into(), "  {} rm C:\\OldTools".into());

    m.insert("cmd.pathman.example_show".into(), "  {} show".into());

    m.insert("cmd.pathman.examples_title".into(), "Examples:".into());

    m.insert("cmd.pathman.examples_title".into(), "Examples:".into());

    m.insert("cmd.pathman.help_add".into(), "  {} add <path>               Add a directory to PATH".into());

    m.insert("cmd.pathman.help_add_cwd".into(), "  {} add --cwd | --here       Add current directory to PATH".into());

    m.insert("cmd.pathman.help_move".into(), "  {} move <n> <m>             Move entry #n to position #m".into());

    m.insert("cmd.pathman.help_rm".into(), "  {} rm <path-or-number>      Remove from PATH (by path or index)".into());

    m.insert("cmd.pathman.help_show".into(), "  {} show                     Show PATH entries with indices".into());

    m.insert("cmd.pathman.idx_out_of_range".into(), "Index out of range: {} (1-{})".into());

    m.insert("cmd.pathman.indices_numbers".into(), "Indices must be numbers".into());

    m.insert("cmd.pathman.mode_admin_hint".into(), "then run the command again. Or use: mode admin".into());

    m.insert("cmd.pathman.move_usage".into(), "Usage: {} move <n> <m>".into());

    m.insert("cmd.pathman.moved".into(), "Moved to position #{} in {}: {}".into());

    m.insert("cmd.pathman.new_terminal".into(), "Changes will apply to new terminal windows.".into());

    m.insert("cmd.pathman.not_found_in_path".into(), "Not found in PATH: {}".into());

    m.insert("cmd.pathman.path_update_fail".into(), "Failed to update PATH: {}".into());

    m.insert("cmd.pathman.path_update_fail".into(), "Failed to update PATH: {}".into());

    m.insert("cmd.pathman.reg_query_fail".into(), "Failed to query registry: {}".into());

    m.insert("cmd.pathman.reg_write_fail".into(), "Failed to write registry: {}".into());

    m.insert("cmd.pathman.removed_from_path".into(), "Removed from {}: {}".into());

    m.insert("cmd.pathman.rm_usage".into(), "Usage: usrpath rm <path-or-number>".into());

    m.insert("cmd.pathman.run_as_admin".into(), "Run your terminal as Administrator (right-click → \\\"Run as administrator\\\"),".into());

    m.insert("cmd.pathman.same_source_target".into(), "Source and target are the same \u{2014} nothing to move".into());

    m.insert("cmd.pathman.system_level".into(), "System".into());

    m.insert("cmd.pathman.system_level".into(), "System".into());

    m.insert("cmd.pathman.total_entries".into(), "  Total: {} entries".into());

    m.insert("cmd.pathman.usage_title".into(), "Usage:".into());

    m.insert("cmd.pathman.usage_title".into(), "Usage:".into());

    m.insert("cmd.pathman.user_level".into(), "User".into());

    m.insert("cmd.pathman.user_level".into(), "User".into());

    m.insert("cmd.pathman.windows_only".into(), "usrpath/syspath is a Windows-only feature.".into());

    m.insert("cmd.qr.decode_fail".into(), "Failed to decode QR #{}: {}".into());

    m.insert("cmd.qr.decode_requires".into(), "QR decode requires feature 'image2doc' (enabled by default). Run with --features image2doc.".into());

    m.insert("cmd.qr.decode_usage".into(), "Usage: qr decode <image_file>".into());

    m.insert("cmd.qr.encode_usage".into(), "Usage: qr encode <text|image> [-o <file>]".into());

    m.insert("cmd.qr.example1".into(), "  qr \"Hello, World!\"".into());

    m.insert("cmd.qr.example2".into(), "  qr encode \\\"hello world\\\" -o qr.png".into());

    m.insert("cmd.qr.example3".into(), "  qr decode qr.png".into());

    m.insert("cmd.qr.example_image".into(), "  qr encode image.png -o imageqrcode.png".into());

    m.insert("cmd.qr.examples_title".into(), "Examples:".into());

    m.insert("cmd.qr.file_not_found".into(), "File not found: {}".into());

    m.insert("cmd.qr.gen_fail".into(), "Failed to generate QR code: {}".into());

    m.insert("cmd.qr.help_decode".into(), "  qr decode <image_file>           Decode QR code from image".into());

    m.insert("cmd.qr.help_encode".into(), "  qr <text>                        Encode and display QR code in terminal".into());

    m.insert("cmd.qr.help_same_as_above".into(), "  qr encode <text>                 Same as above".into());

    m.insert("cmd.qr.help_save_png".into(), "  qr encode <text> -o <file>       Save QR as PNG image".into());

    m.insert("cmd.qr.help_encode_image".into(), "  qr encode <image> -o <file>       Encode an image into a QR code (auto-downscaled)".into());

    m.insert("cmd.qr.image_save_requires".into(), "Image saving requires feature 'image2doc' (enabled by default)".into());

    m.insert("cmd.qr.image_requires_out".into(), "Encoding an image requires an output file: qr encode <image> -o <file>.png".into());

    m.insert("cmd.qr.image_too_large".into(), "Image is too large to fit in a QR code even at minimum size.".into());

    m.insert("cmd.qr.no_qr_detected".into(), "No QR code detected in the image.".into());

    m.insert("cmd.qr.no_text".into(), "No text to encode".into());

    m.insert("cmd.qr.open_image_fail".into(), "Failed to open image: {}".into());

    m.insert("cmd.qr.qr_label".into(), "{} {}".into());

    m.insert("cmd.qr.saved".into(), "QR code saved to: {}".into());

    m.insert("cmd.qr.usage_title".into(), "QR Code:".into());

    m.insert("cmd.qsc.already_running".into(), "QSC daemon is already running. Use 'qsc stop' first.".into());

    m.insert("cmd.qsc.current_setting".into(), "  Current: {}".into());

    m.insert("cmd.qsc.current_setting".into(), "  Current: {}".into());

    m.insert("cmd.qsc.daemon_running".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_stopped".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_unknown".into(), "  {} ({})".into());

    m.insert("cmd.qsc.daemon_unknown_err".into(), "QSC daemon: unknown ({})".into());

    m.insert("cmd.qsc.help_conf_ex1".into(), "    config qsc \\\"#c 0\\\"        Set hotkey to Ctrl+0".into());

    m.insert("cmd.qsc.help_conf_ex2".into(), "    config qsc \\\"#c #s a\\\"     Set hotkey to Ctrl+Shift+A".into());

    m.insert("cmd.qsc.help_conf_ex3".into(), "    config qsc \\\"#c #s #a F1\\\"  Set hotkey to Ctrl+Shift+Alt+F1".into());

    m.insert("cmd.qsc.help_conf_off".into(), "    config qsc off            Disable QSC hotkey".into());

    m.insert("cmd.qsc.help_configure".into(), "  First configure the hotkey:".into());

    m.insert("cmd.qsc.help_listen".into(), "  qsc listen              Start the QSC global hotkey daemon".into());

    m.insert("cmd.qsc.help_listen".into(), "  qsc listen              Start the QSC global hotkey daemon".into());

    m.insert("cmd.qsc.help_status".into(), "  qsc status              Check if QSC daemon is running".into());

    m.insert("cmd.qsc.help_status".into(), "  qsc status              Check if QSC daemon is running".into());

    m.insert("cmd.qsc.help_stop".into(), "  qsc stop                Stop the QSC global hotkey daemon".into());

    m.insert("cmd.qsc.help_stop".into(), "  qsc stop                Stop the QSC global hotkey daemon".into());

    m.insert("cmd.qsc.hotkey_label".into(), "  Hotkey: {}".into());

    m.insert("cmd.qsc.hotkey_not_configured".into(), "  {}".into());

    m.insert("cmd.qsc.not_configured".into(), "No QSC hotkey configured. Use 'config qsc \"#c 0\"' first.".into());

    m.insert("cmd.qsc.not_running".into(), "QSC daemon is not running.".into());

    m.insert("cmd.qsc.started".into(), "QSC daemon started.".into());

    m.insert("cmd.qsc.startup_registered".into(), "QSC daemon registered as a startup app.".into());

    m.insert("cmd.qsc.startup_removed".into(), "QSC daemon removed from startup apps.".into());

    m.insert("cmd.qsc.startup_usage".into(), "Usage: qsc startup on|off".into());

    m.insert("cmd.qsc.status_running".into(), "QSC daemon: running".into());

    m.insert("cmd.qsc.status_stopped".into(), "QSC daemon: stopped".into());

    m.insert("cmd.qsc.stopped".into(), "QSC daemon stopped.".into());

    m.insert("cmd.qsc.usage_title".into(), "QSC Global Hotkey:".into());

    m.insert("cmd.qsc.usage_title".into(), "QSC Global Hotkey:".into());

    m.insert("cmd.report.content_saved".into(), "File content saved to: {}".into());

    m.insert("cmd.report.copied_clipboard".into(), "File '{}' copied to clipboard!".into());

    m.insert("cmd.report.copy_failed".into(), "Failed to copy: {}".into());

    m.insert("cmd.report.copy_failed".into(), "Failed to copy: {}".into());

    m.insert("cmd.report.copying_clipboard".into(), "Copying '{}' to clipboard...".into());

    m.insert("cmd.report.dir_only".into(), "{} report only works on directories".into());

    m.insert("cmd.report.example_json".into(), "  report json ..         Report on parent directory".into());

    m.insert("cmd.report.example_txt".into(), "  report txt             Report on current directory".into());

    m.insert("cmd.report.examples".into(), "Examples:".into());

    m.insert("cmd.report.examples".into(), "Examples:".into());

    m.insert("cmd.report.file_not_found".into(), "File not found: {}".into());

    m.insert("cmd.report.help_docx".into(), "  report docx [path]     Generate DOCX report".into());

    m.insert("cmd.report.help_html".into(), "  report html [path]     Generate HTML report".into());

    m.insert("cmd.report.help_json".into(), "  report json [path]     Generate JSON report".into());

    m.insert("cmd.report.help_md".into(), "  report md [path]       Generate Markdown report".into());

    m.insert("cmd.report.help_pdf".into(), "  report pdf [path]      Generate PDF report".into());

    m.insert("cmd.report.help_txt".into(), "  report txt [path]      Generate TXT report".into());

    m.insert("cmd.report.help_xlsx".into(), "  report xlsx [path]     Generate XLSX report".into());

    m.insert("cmd.report.invalid_depth".into(), "Invalid depth: {}".into());

    m.insert("cmd.report.neovim_hint".into(), "You can open this in Neovim with: :edit {}".into());

    m.insert("cmd.report.skipped_unsupported".into(), "Skipped (not support format): {}".into());

    m.insert("cmd.report.ext_usage".into(), "Usage: view -ext <ext...>  (e.g. view -ext rs c)".into());

    m.insert("cmd.report.time_label".into(), "Time:".into());

    m.insert("cmd.report.title".into(), "report — Generate project reports".into());

    m.insert("cmd.report.unknown_option".into(), "Unknown view option: {}".into());

    m.insert("cmd.scan.depth_usage".into(), "Usage: scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]".into());
    m.insert("cmd.scan.examples_title".into(), "Examples:".into());
    m.insert("cmd.scan.ext_usage".into(), "Usage: scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]".into());
    m.insert("cmd.scan.help_ext".into(), "  scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]  List files by extension (no line counts)".into());

    m.insert("cmd.scan.help_port".into(), "  scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]  TCP port scan (default host 127.0.0.1, top 20)".into());

    m.insert("cmd.scan.help_ip".into(), "  scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]  Live-host sweep (default auto local /24)".into());

    m.insert("cmd.scan.ip_arp_only".into(), "(arp)".into());

    m.insert("cmd.scan.ip_done".into(), "Live: {} of {} swept.".into());

    m.insert("cmd.scan.ip_none".into(), "No live hosts in {}.".into());

    m.insert("cmd.scan.ip_start".into(), "Sweeping {} ({} ports/host)...".into());

    m.insert("cmd.scan.ip_title".into(), "Live hosts in {}:".into());

    m.insert("cmd.scan.missing_ports".into(), "Missing value after '--ports'.".into());

    m.insert("cmd.scan.missing_timeout".into(), "Missing value after '--timeout'.".into());

    m.insert("cmd.scan.missing_top".into(), "Missing value after '--top'.".into());
    m.insert("cmd.scan.invalid_depth".into(), "Invalid depth: {}".into());

    m.insert("cmd.scan.invalid_timeout".into(), "Invalid --timeout value: '{}'.".into());

    m.insert("cmd.scan.invalid_top".into(), "Invalid --top value: '{}'.".into());
    m.insert("cmd.scan.list_title".into(), "SCAN: {}".into());
    m.insert("cmd.scan.no_files".into(), "No {} files found".into());

    m.insert("cmd.scan.port_done".into(), "Open: {} of {} scanned.".into());

    m.insert("cmd.scan.port_none".into(), "No open ports on {}.".into());

    m.insert("cmd.scan.port_start".into(), "Scanning {} ({} ports)...".into());

    m.insert("cmd.scan.port_title".into(), "Open ports on {}:".into());

    m.insert("cmd.scan.progress".into(), "Scanning {} … {}/{} ({}%) · {} open".into());
    m.insert("cmd.scan.total_files".into(), "Total: {} files".into());
    m.insert("cmd.scan.total_files_size".into(), "Total: {} files ({})".into());
    m.insert("cmd.scan.unknown_option".into(), "Unknown scan option: {}".into());
    m.insert("cmd.scan.unknown_sub".into(), "Unknown scan subcommand: '{}'".into());

    m.insert("cmd.scan.unexpected_arg".into(), "Unexpected argument: '{}'.".into());
    m.insert("cmd.scan.usage_title".into(), "Usage:".into());

    m.insert("cmd.report.view_usage".into(), "Usage: view [-s|--size] [-c|--cp] [--care] [-t|--time] [-d|--depth <n>] [-p|--path <path|@tp>] [-ext|--ext <ext...>]".into());

    m.insert("cmd.serve.db_title".into(), "Database:".into());

    m.insert("cmd.target.not_dir".into(), "'{}' is not a directory".into());

    m.insert("cmd.target.not_found".into(), "Path '{}' not found".into());

    m.insert("cmd.target.tp_not_found".into(), "Savepoint '{}' not found".into());

    m.insert("cmd.serve.db_title".into(), "Database:".into());

    m.insert("cmd.serve.example_default".into(), "  serve                  HTTP on port 8080".into());

    m.insert("cmd.serve.example_default".into(), "  serve                  HTTP on port 8080".into());

    m.insert("cmd.serve.example_port".into(), "  serve 3000             HTTP on port 3000".into());

    m.insert("cmd.serve.example_port".into(), "  serve 3000             HTTP on port 3000".into());

    m.insert("cmd.serve.example_spa".into(), "  serve --spa            SPA fallback for React Router".into());

    m.insert("cmd.serve.example_spa".into(), "  serve --spa            SPA fallback for React Router".into());

    m.insert("cmd.serve.example_tunnel".into(), "  serve --tunnel         HTTP + cloudflared tunnel".into());

    m.insert("cmd.serve.example_tunnel".into(), "  serve --tunnel         HTTP + cloudflared tunnel".into());

    m.insert("cmd.serve.examples_title".into(), "Examples:".into());

    m.insert("cmd.serve.examples_title".into(), "Examples:".into());

    m.insert("cmd.serve.files_stored_title".into(), "Files stored in:".into());

    m.insert("cmd.serve.files_stored_title".into(), "Files stored in:".into());

    m.insert("cmd.serve.help_bind".into(), "  --bind, -b <addr>     Bind address (default: 0.0.0.0)".into());

    m.insert("cmd.serve.help_bind".into(), "  --bind, -b <addr>     Bind address (default: 0.0.0.0)".into());

    m.insert("cmd.serve.help_help".into(), "  --help, -h            Show this help".into());

    m.insert("cmd.serve.help_help".into(), "  --help, -h            Show this help".into());

    m.insert("cmd.serve.help_port".into(), "  --port, -p <port>     Port to listen on (default: 8080)".into());

    m.insert("cmd.serve.help_port".into(), "  --port, -p <port>     Port to listen on (default: 8080)".into());

    m.insert("cmd.serve.help_spa".into(), "  --spa                 SPA mode (fallback to index.html)".into());

    m.insert("cmd.serve.help_spa".into(), "  --spa                 SPA mode (fallback to index.html)".into());

    m.insert("cmd.serve.help_title".into(), "HTTP File Server".into());

    m.insert("cmd.serve.help_title".into(), "HTTP File Server".into());

    m.insert("cmd.serve.help_tunnel".into(), "  --tunnel, -t          Expose via cloudflared tunnel".into());

    m.insert("cmd.serve.help_tunnel".into(), "  --tunnel, -t          Expose via cloudflared tunnel".into());

    m.insert("cmd.serve.help_share".into(), "  --share [path]        Check internet, show LAN URL + QR for phones".into());

    m.insert("cmd.serve.help_qr".into(), "  --qr                  Force QR display (implies --share)".into());

    m.insert("cmd.serve.help_no_qr".into(), "  --no-qr               Suppress QR display".into());

    m.insert("cmd.serve.example_share".into(), "  serve 8080 --share      Check internet, print LAN URL + QR".into());

    m.insert("cmd.serve.share_checking".into(), "Checking internet connection...".into());

    m.insert("cmd.serve.share_online".into(), "Internet OK — share mode".into());

    m.insert("cmd.serve.share_offline".into(), "No internet connection — showing localhost only.".into());

    m.insert("cmd.serve.share_lan_title".into(), "LAN:".into());

    m.insert("cmd.serve.share_path_title".into(), "Share:".into());

    m.insert("cmd.serve.share_no_lan".into(), "No LAN address found — showing localhost only.".into());

    m.insert("cmd.serve.share_bound_loopback".into(), "Bound to loopback: LAN devices cannot reach this server (use default 0.0.0.0).".into());

    m.insert("cmd.serve.share_not_private".into(), "Detected address {} is not a private LAN range.".into());

    m.insert("cmd.serve.share_not_private_hint".into(), "Expected 10.x.x.x, 172.16-31.x.x or 192.168.x.x for phone sharing.".into());

    m.insert("cmd.serve.share_qr_fail".into(), "QR render failed: {}".into());

    m.insert("cmd.serve.help_usage".into(), "serve [port] [options]".into());

    m.insert("cmd.serve.help_usage".into(), "serve [port] [options]".into());

    m.insert("cmd.serve.help_web_dir".into(), "  --web-dir, -w <dir>   Serve custom frontend directory".into());

    m.insert("cmd.serve.help_web_dir".into(), "  --web-dir, -w <dir>   Serve custom frontend directory".into());

    m.insert("cmd.serve.help_auth".into(), "  --auth <user:pass>    Require login for /api/* (e.g. --auth admin:secret)".into());

    m.insert("cmd.serve.example_auth".into(), "  serve 8080 --auth admin:secret   HTTP on 8080, API needs login".into());

    m.insert("cmd.serve.invalid_auth".into(), "Invalid --auth (use --auth <user:pass>)".into());

    m.insert("cmd.serve.missing_auth".into(), "Missing value for --auth".into());

    m.insert("cmd.serve.auth_on".into(), "API auth enabled for user '{}'".into());

    m.insert("cmd.nssh.usage_title".into(), "Usage:".into());
    m.insert("cmd.nssh.help_add".into(), "nssh add <name> --host <h> [--user u] [--port p] [--key path] [--pass-env VAR]".into());
    m.insert("cmd.nssh.help_ssh".into(), "nssh ssh [name] [--cmd \"...\"] [--trust] [--ask-pass] [--timeout s]".into());
    m.insert("cmd.nssh.help_cp".into(), "nssh cp <local> <name>:<remote> | <name>:<remote> <local>".into());
    m.insert("cmd.nssh.help_scan".into(), "nssh scan <host> [--ports 22,80,8000-8010 | --top N] [--timeout ms]".into());
    m.insert("cmd.nssh.help_manage".into(), "nssh ls | nssh info <name> | nssh rm <name> | nssh edit <name> [flags]".into());
    m.insert("cmd.nssh.help_keygen".into(), "nssh keygen [args...]   Generate keys via ssh-keygen".into());
    m.insert("cmd.nssh.help_help".into(), "nssh help   Show this help (bare nssh picks a profile)".into());
    m.insert("cmd.nssh.options_title".into(), "Options:".into());
    m.insert("cmd.nssh.help_host".into(), "  --host <h>            Server hostname or IP (required for add)".into());
    m.insert("cmd.nssh.help_user".into(), "  --user, -u <u>        Login user (default: current user)".into());
    m.insert("cmd.nssh.help_port".into(), "  --port, -p <p>        SSH port (default: 22)".into());
    m.insert("cmd.nssh.help_key".into(), "  --key <path>          Private key file (default: ~/.ssh/id_ed25519, id_rsa)".into());
    m.insert("cmd.nssh.help_pass_env".into(), "  --pass-env <VAR>      Read the password from env var VAR (never stored)".into());
    m.insert("cmd.nssh.help_cmd".into(), "  --cmd \"...\"           Run one command instead of an interactive shell".into());
    m.insert("cmd.nssh.help_trust".into(), "  --trust               Accept + record unknown host keys (default: fail closed)".into());
    m.insert("cmd.nssh.help_ask_pass".into(), "  --ask-pass            Prompt for the password (hidden input)".into());
    m.insert("cmd.nssh.help_timeout".into(), "  --timeout <n>         Connect timeout in seconds (ssh/cp) or ms (scan)".into());
    m.insert("cmd.nssh.help_top".into(), "  --top <n>             Scan the top-N common ports (default: 20)".into());
    m.insert("cmd.nssh.help_scan_ports".into(), "  --ports <spec>        Ports like 22,80,8000-8010 (max 2000 per range)".into());
    m.insert("cmd.nssh.help_yes".into(), "  --yes, -y             Skip the delete confirmation".into());
    m.insert("cmd.nssh.examples_title".into(), "Examples:".into());
    m.insert("cmd.nssh.example_add".into(), "nssh add prod --host 203.0.113.10 --user root --key ~/.ssh/id_rsa".into());
    m.insert("cmd.nssh.example_ssh".into(), "nssh ssh prod --cmd \"uptime\"".into());
    m.insert("cmd.nssh.example_cp".into(), "nssh cp ./app.tar.gz prod:/tmp/".into());
    m.insert("cmd.nssh.example_scan".into(), "nssh scan example.com --top 20".into());
    m.insert("cmd.nssh.missing_host".into(), "Missing value for --host".into());
    m.insert("cmd.nssh.missing_user".into(), "Missing value for --user".into());
    m.insert("cmd.nssh.missing_port".into(), "Missing value for --port".into());
    m.insert("cmd.nssh.missing_key".into(), "Missing value for --key".into());
    m.insert("cmd.nssh.missing_pass_env".into(), "Missing value for --pass-env".into());
    m.insert("cmd.nssh.missing_cmd".into(), "Missing value for --cmd".into());
    m.insert("cmd.nssh.missing_timeout".into(), "Missing value for --timeout".into());
    m.insert("cmd.nssh.missing_ports".into(), "Missing value for --ports".into());
    m.insert("cmd.nssh.missing_top".into(), "Missing value for --top".into());
    m.insert("cmd.nssh.invalid_port".into(), "Invalid port: '{}'".into());
    m.insert("cmd.nssh.invalid_timeout".into(), "Invalid timeout: '{}'".into());
    m.insert("cmd.nssh.invalid_top".into(), "Invalid --top value: '{}'".into());
    m.insert("cmd.nssh.invalid_name".into(), "Invalid profile name '{}' (letters, digits, _ - . only)".into());
    m.insert("cmd.nssh.unknown_option".into(), "Unknown: {}".into());
    m.insert("cmd.nssh.add_usage".into(), "Usage: nssh add <name> --host <h> [--user u] [--port p] [--key path] [--pass-env VAR]".into());
    m.insert("cmd.nssh.profile_exists".into(), "Profile '{}' exists (use nssh edit to change it)".into());
    m.insert("cmd.nssh.profile_saved".into(), "Profile saved: {}".into());
    m.insert("cmd.nssh.profile_not_found".into(), "Profile '{}' not found".into());
    m.insert("cmd.nssh.key_not_found".into(), "Key file '{}' does not exist (stored anyway)".into());
    m.insert("cmd.nssh.pass_env_missing".into(), "Env var '{}' is not set (stored anyway)".into());
    m.insert("cmd.nssh.ls_empty".into(), "No profiles yet. Add one: nssh add <name> --host <h>".into());
    m.insert("cmd.nssh.ls_title".into(), "SSH profiles:".into());
    m.insert("cmd.nssh.info_title".into(), "Profile '{}':".into());
    m.insert("cmd.nssh.info_usage".into(), "Usage: nssh info <name>".into());
    m.insert("cmd.nssh.info_host".into(), "host:".into());
    m.insert("cmd.nssh.info_user".into(), "user:".into());
    m.insert("cmd.nssh.info_port".into(), "port:".into());
    m.insert("cmd.nssh.info_key".into(), "key:".into());
    m.insert("cmd.nssh.info_pass".into(), "pass_env:".into());
    m.insert("cmd.nssh.rm_usage".into(), "Usage: nssh rm <name> [--yes]".into());
    m.insert("cmd.nssh.rm_confirm".into(), "Delete profile '{}'?".into());
    m.insert("cmd.nssh.rm_deleted".into(), "Deleted profile: {}".into());
    m.insert("cmd.nssh.rm_cancelled".into(), "Cancelled.".into());
    m.insert("cmd.nssh.edit_usage".into(), "Usage: nssh edit <name> [--host h] [--user u] [--port p] [--key path|--no-key] [--pass-env VAR|--no-pass-env]".into());
    m.insert("cmd.nssh.edit_saved".into(), "Profile updated: {}".into());
    m.insert("cmd.nssh.pick_profile".into(), "Pick an SSH profile".into());
    m.insert("cmd.nssh.ask_pass".into(), "Password for {}@{}: ".into());
    m.insert("cmd.nssh.pass_env_unset".into(), "Env var '{}' is empty or unset — will prompt if --ask-pass".into());
    m.insert("cmd.nssh.ssh_connecting".into(), "Connecting to {}@{}:{}...".into());
    m.insert("cmd.nssh.cmd_exit".into(), "Remote command exited with code {}".into());
    m.insert("cmd.nssh.cp_usage".into(), "Usage: nssh cp <local> <name>:<remote> | nssh cp <name>:<remote> <local>".into());
    m.insert("cmd.nssh.cp_no_remote".into(), "Exactly one side must be <profile>:<path>".into());
    m.insert("cmd.nssh.cp_two_sides".into(), "Both sides are remote — copy one side at a time".into());
    m.insert("cmd.nssh.cp_local_missing".into(), "Local file '{}' not found".into());
    m.insert("cmd.nssh.cp_read_fail".into(), "Cannot read local file: {}".into());
    m.insert("cmd.nssh.cp_too_large".into(), "File exceeds the 256MB v1 transfer guard".into());
    m.insert("cmd.nssh.cp_uploading".into(), "Uploading {} to {}:{}...".into());
    m.insert("cmd.nssh.cp_downloading".into(), "Downloading {}:{} to {}...".into());
    m.insert("cmd.nssh.cp_done".into(), "Done ({} bytes)".into());
    m.insert("cmd.nssh.scan_usage".into(), "Usage: nssh scan <host> [--ports <spec> | --top <n>] [--timeout <ms>]".into());
    m.insert("cmd.nssh.scan_start".into(), "Scanning {} ({} ports)...".into());
    m.insert("cmd.nssh.scan_title".into(), "Open ports on {}:".into());
    m.insert("cmd.nssh.scan_none".into(), "No open ports on {}".into());
    m.insert("cmd.nssh.scan_done".into(), "{} open of {} probed".into());
    m.insert("cmd.nssh.keygen_missing".into(), "ssh-keygen not found: {}".into());
    m.insert("cmd.nssh.keygen_exit".into(), "ssh-keygen exited with code {}".into());
    m.insert("cmd.nssh.shell_hint".into(), "Interactive shell — Enter ~ . force-disconnects".into());
    m.insert("cmd.nssh.shell_back".into(), "Back in ntc shell".into());
    m.insert("cmd.nssh.shell_forced".into(), "Force-disconnected (~.)".into());

    m.insert("cmd.nsheet.usage_title".into(), "Usage:".into());
    m.insert("cmd.nsheet.help_open".into(), "nsheet open <file> [options]   Fullscreen viewer (CSV/TSV/JSON/JSONL)".into());
    m.insert("cmd.nsheet.help_view".into(), "nsheet view <file> [options]   Print table (default 50 rows, --all for all)".into());
    m.insert("cmd.nsheet.options_title".into(), "Options:".into());
    m.insert("cmd.nsheet.help_delimiter".into(), "  -d, --delimiter <c|auto>  Field separator (default: auto-detect , ; tab |)".into());
    m.insert("cmd.nsheet.help_header".into(), "  --header / --no-header      Force header row on/off (default: auto-detect)".into());
    m.insert("cmd.nsheet.help_limit".into(), "  -l, --limit <n> / --all      Rows to print in view mode".into());
    m.insert("cmd.nsheet.help_sort".into(), "  --sort <col> [--desc]         Sort by column name or 1-based index".into());
    m.insert("cmd.nsheet.help_filter".into(), "  --filter <expr>              e.g. age>30, city~york, name=bob (!=,>=,<=,&&,||)".into());
    m.insert("cmd.nsheet.help_export".into(), "  --export <json|md>           Export instead of printing".into());
    m.insert("cmd.nsheet.help_output".into(), "  -o, --output <file>          Export destination (default: stdout)".into());
    m.insert("cmd.nsheet.help_help".into(), "  --help, -h                   Show this help".into());
    m.insert("cmd.nsheet.examples_title".into(), "Examples:".into());
    m.insert("cmd.nsheet.example_open".into(), "nsheet open users.csv".into());
    m.insert("cmd.nsheet.example_view".into(), "nsheet view users.csv -l 20 --sort age --desc".into());
    m.insert("cmd.nsheet.example_filter".into(), "nsheet view data.json --filter \"age>30 && city~york\"".into());
    m.insert("cmd.nsheet.example_export".into(), "nsheet view a.csv --export json -o out.json".into());
    m.insert("cmd.nsheet.missing_file".into(), "Missing file. Usage: nsheet open <file> | nsheet view <file>".into());
    m.insert("cmd.nsheet.file_not_found".into(), "File '{}' not found".into());
    m.insert("cmd.nsheet.load_fail".into(), "Cannot load sheet: {}".into());
    m.insert("cmd.nsheet.empty_sheet".into(), "Empty sheet (no columns found)".into());
    m.insert("cmd.nsheet.no_rows".into(), "No rows match.".into());
    m.insert("cmd.nsheet.unknown_option".into(), "Unknown: {}".into());
    m.insert("cmd.nsheet.unknown_column".into(), "Unknown column: '{}'".into());
    m.insert("cmd.nsheet.invalid_limit".into(), "Invalid limit: '{}'".into());
    m.insert("cmd.nsheet.invalid_delimiter".into(), "Invalid delimiter: '{}' (use a single char or auto)".into());
    m.insert("cmd.nsheet.invalid_export".into(), "Invalid export format: '{}' (use json or md)".into());
    m.insert("cmd.nsheet.delimiter_usage".into(), "Usage: --delimiter <char|auto>".into());
    m.insert("cmd.nsheet.limit_usage".into(), "Usage: --limit <n>".into());
    m.insert("cmd.nsheet.sort_usage".into(), "Usage: --sort <column>".into());
    m.insert("cmd.nsheet.filter_usage".into(), "Usage: --filter <expr> (e.g. age>30)".into());
    m.insert("cmd.nsheet.export_usage".into(), "Usage: --export <json|md>".into());
    m.insert("cmd.nsheet.output_usage".into(), "Usage: --output <file>".into());
    m.insert("cmd.nsheet.filter_fail".into(), "Bad filter: {}".into());
    m.insert("cmd.nsheet.export_fail".into(), "Export failed: {}".into());
    m.insert("cmd.nsheet.exported".into(), "Exported: {}".into());
    m.insert("cmd.nsheet.rows_shown".into(), "Showing {} of {} rows ({} cols)".into());
    m.insert("cmd.nsheet.rows_truncated".into(), "Truncated to {} rows (use --all or -l <n>)".into());
    m.insert("cmd.nsheet.tui_hint".into(), "arrows/hjkl move · / search · n next · s sort · f filter · c clear · e export · Enter view · q quit".into());
    m.insert("cmd.nsheet.tui_search_prompt".into(), "search (empty clears)".into());
    m.insert("cmd.nsheet.tui_sort_prompt".into(), "sort by col (! suffix = desc, empty clears)".into());
    m.insert("cmd.nsheet.tui_filter_prompt".into(), "filter (empty clears)".into());
    m.insert("cmd.nsheet.tui_export_prompt".into(), "export to (.json/.md path)".into());
    m.insert("cmd.nsheet.tui_cell_title".into(), "Cell (any key to close)".into());
    m.insert("cmd.nsheet.tui_popup_hint".into(), "any key to close".into());
    m.insert("cmd.nsheet.tui_no_matches".into(), "No matches".into());
    m.insert("cmd.nsheet.tui_large".into(), "Large sheet ({} rows) — navigation may be slow".into());

    m.insert("cmd.serve.invalid_port".into(), "Invalid port".into());

    m.insert("cmd.serve.invalid_port".into(), "Invalid port".into());

    m.insert("cmd.serve.missing_bind".into(), "Missing bind".into());

    m.insert("cmd.serve.missing_bind".into(), "Missing bind".into());

    m.insert("cmd.serve.missing_dir".into(), "Missing dir".into());

    m.insert("cmd.serve.missing_dir".into(), "Missing dir".into());

    m.insert("cmd.serve.missing_port".into(), "Missing port".into());

    m.insert("cmd.serve.missing_port".into(), "Missing port".into());

    m.insert("cmd.serve.options_title".into(), "Options:".into());

    m.insert("cmd.serve.options_title".into(), "Options:".into());

    m.insert("cmd.serve.stopped".into(), "Server stopped.".into());

    m.insert("cmd.serve.unknown_option".into(), "Unknown: {}".into());

    m.insert("cmd.serve.unknown_option".into(), "Unknown: {}".into());

    m.insert("cmd.serve.usage_title".into(), "Usage:".into());

    m.insert("cmd.serve.usage_title".into(), "Usage:".into());

    m.insert("cmd.serve.cloudflared_starting".into(), "Cloudflared tunnel starting (PID {})".into());

    m.insert("cmd.serve.cloudflared_missing".into(), "cloudflared not found: {}. Install: {}".into());

    m.insert("cmd.smtp.build_fail".into(), "Failed to build email message".into());

    m.insert("cmd.smtp.create_one_hint".into(), "  Create one: {} -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.create_syntax".into(), "smtp -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.create_syntax".into(), "smtp -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.create_usage".into(), "Server, username, and password are required. Usage: smtp -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.created_label".into(), "Created:".into());

    m.insert("cmd.smtp.created_label".into(), "Created:".into());

    m.insert("cmd.smtp.decode_failed".into(), "Failed to decode stored password".into());

    m.insert("cmd.smtp.decode_failed".into(), "Failed to decode stored password".into());

    m.insert("cmd.smtp.help_attachments_desc".into(), "  -f accepts comma-separated file paths and directories (files are expanded recursively)".into());

    m.insert("cmd.smtp.help_attachments_example".into(), "  Example: smtp --id a1b2 --to user@test.com -s \\\"Report\\\" -b \\\"See attached\\\" -f \\\"./report.pdf,./images/\\\"".into());

    m.insert("cmd.smtp.help_attachments_title".into(), "Attachments:".into());

    m.insert("cmd.smtp.help_attachments_title".into(), "Attachments:".into());

    m.insert("cmd.smtp.help_create_example".into(), "  Example: smtp -i smtp.gmail.com -p 587 -u me@gmail.com -pwd app123".into());

    m.insert("cmd.smtp.help_create_syntax".into(), "  smtp -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.help_create_title".into(), "Create a profile:".into());

    m.insert("cmd.smtp.help_create_title".into(), "Create a profile:".into());

    m.insert("cmd.smtp.help_info_syntax".into(), "  smtp --id-info <id>".into());

    m.insert("cmd.smtp.help_info_title".into(), "Show profile info:".into());

    m.insert("cmd.smtp.help_info_title".into(), "Show profile info:".into());

    m.insert("cmd.smtp.help_list_syntax".into(), "  smtp --id-ls".into());

    m.insert("cmd.smtp.help_list_title".into(), "List profiles:".into());

    m.insert("cmd.smtp.help_list_title".into(), "List profiles:".into());

    m.insert("cmd.smtp.help_port_25".into(), "  25   Plain (if supported by server)".into());

    m.insert("cmd.smtp.help_port_465".into(), "  465  TLS (SMTPS)".into());

    m.insert("cmd.smtp.help_port_587".into(), "  587  STARTTLS (default)".into());

    m.insert("cmd.smtp.help_ports_title".into(), "Ports:".into());

    m.insert("cmd.smtp.help_ports_title".into(), "Ports:".into());

    m.insert("cmd.smtp.help_rm_syntax".into(), "  smtp --id-rm <id>".into());

    m.insert("cmd.smtp.help_rm_title".into(), "Delete a profile:".into());

    m.insert("cmd.smtp.help_rm_title".into(), "Delete a profile:".into());

    m.insert("cmd.smtp.help_send_syntax".into(), "  smtp --id <id> --to <email> [--cc <email>] [--bcc <email>] -s <subject> -b <body> [-f <files>]".into());

    m.insert("cmd.smtp.help_send_title".into(), "Send email:".into());

    m.insert("cmd.smtp.help_send_title".into(), "Send email:".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP Email Client".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP Email Client".into());

    m.insert("cmd.smtp.id_label".into(), "ID:".into());

    m.insert("cmd.smtp.id_label".into(), "ID:".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "Invalid BCC: {}".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "Invalid BCC: {}".into());

    m.insert("cmd.smtp.invalid_cc".into(), "Invalid CC: {}".into());

    m.insert("cmd.smtp.invalid_port".into(), "Invalid port number".into());

    m.insert("cmd.smtp.invalid_pwd_encoding".into(), "Invalid password encoding: {}".into());

    m.insert("cmd.smtp.invalid_pwd_encoding".into(), "Invalid password encoding: {}".into());

    m.insert("cmd.smtp.invalid_recipient".into(), "Invalid recipient: {}".into());

    m.insert("cmd.smtp.invalid_sender".into(), "Invalid sender email address".into());

    m.insert("cmd.smtp.invalid_sender".into(), "Invalid sender email address".into());

    m.insert("cmd.smtp.missing_body".into(), "Missing body after -b".into());

    m.insert("cmd.smtp.missing_body".into(), "Missing body after -b".into());

    m.insert("cmd.smtp.missing_id".into(), "Missing ID after --id".into());

    m.insert("cmd.smtp.missing_id".into(), "Missing ID after --id".into());

    m.insert("cmd.smtp.missing_id_info".into(), "Missing ID after --id-info".into());

    m.insert("cmd.smtp.missing_id_info".into(), "Missing ID after --id-info".into());

    m.insert("cmd.smtp.missing_id_rm".into(), "Missing ID after --id-rm".into());

    m.insert("cmd.smtp.missing_id_rm".into(), "Missing ID after --id-rm".into());

    m.insert("cmd.smtp.missing_password".into(), "Missing password after -pwd".into());

    m.insert("cmd.smtp.missing_password".into(), "Missing password after -pwd".into());

    m.insert("cmd.smtp.missing_path".into(), "Missing path after -f".into());

    m.insert("cmd.smtp.missing_port".into(), "Missing port after -p".into());

    m.insert("cmd.smtp.missing_port".into(), "Missing port after -p".into());

    m.insert("cmd.smtp.missing_recipient_bcc".into(), "Missing recipient after --bcc".into());

    m.insert("cmd.smtp.missing_recipient_bcc".into(), "Missing recipient after --bcc".into());

    m.insert("cmd.smtp.missing_recipient_cc".into(), "Missing recipient after --cc".into());

    m.insert("cmd.smtp.missing_recipient_cc".into(), "Missing recipient after --cc".into());

    m.insert("cmd.smtp.missing_recipient_to".into(), "Missing recipient after --to".into());

    m.insert("cmd.smtp.missing_recipient_to".into(), "Missing recipient after --to".into());

    m.insert("cmd.smtp.missing_server".into(), "Missing server after -i".into());

    m.insert("cmd.smtp.missing_server".into(), "Missing server after -i".into());

    m.insert("cmd.smtp.missing_subject".into(), "Missing subject after -s".into());

    m.insert("cmd.smtp.missing_subject".into(), "Missing subject after -s".into());

    m.insert("cmd.smtp.missing_user".into(), "Missing username after -u".into());

    m.insert("cmd.smtp.missing_user".into(), "Missing username after -u".into());

    m.insert("cmd.smtp.need_recipient".into(), "Recipient required. Use --to <email>".into());

    m.insert("cmd.smtp.need_recipient".into(), "Recipient required. Use --to <email>".into());

    m.insert("cmd.smtp.need_subject".into(), "Subject required. Use -s <subject>".into());

    m.insert("cmd.smtp.no_files_in_dir".into(), "No files found in directory: {}".into());

    m.insert("cmd.smtp.no_profiles".into(), "  {}  No SMTP profiles found.".into());

    m.insert("cmd.smtp.password_label".into(), "Password:".into());

    m.insert("cmd.smtp.password_label".into(), "Password:".into());

    m.insert("cmd.smtp.path_not_found".into(), "Path not found: {}".into());

    m.insert("cmd.smtp.port_label".into(), "Port:".into());

    m.insert("cmd.smtp.port_label".into(), "Port:".into());

    m.insert("cmd.smtp.profile_label".into(), "SMTP Profile:".into());

    m.insert("cmd.smtp.profile_label".into(), "SMTP Profile:".into());

    m.insert("cmd.smtp.profile_not_found".into(), "Profile not found: {}".into());

    m.insert("cmd.smtp.profile_saved".into(), "SMTP profile saved: {}".into());

    m.insert("cmd.smtp.profiles_title".into(), "SMTP Profiles:".into());

    m.insert("cmd.smtp.read_attach_fail".into(), "Failed to read attachment: {}".into());

    m.insert("cmd.smtp.removed_profile".into(), "Removed SMTP profile: {}".into());

    m.insert("cmd.smtp.send_fail".into(), "Failed to send email: {}".into());

    m.insert("cmd.smtp.send_ok".into(), "Email sent successfully{}".into());

    m.insert("cmd.smtp.sending".into(), "Sending email to {}...".into());

    m.insert("cmd.smtp.server_label".into(), "Server:".into());

    m.insert("cmd.smtp.server_label".into(), "Server:".into());

    m.insert("cmd.smtp.skipping_exec".into(), "Skipping executable file: {}".into());

    m.insert("cmd.smtp.skipping_not_file".into(), "Skipping (not a file or directory): {}".into());

    m.insert("cmd.smtp.transport_fail".into(), "Failed to configure SMTP transport: {}".into());

    m.insert("cmd.smtp.unknown_option".into(), "Unknown option: {}".into());

    m.insert("cmd.smtp.unknown_option".into(), "Unknown option: {}".into());

    m.insert("cmd.smtp.user_label".into(), "User:".into());

    m.insert("cmd.smtp.user_label".into(), "User:".into());

    m.insert("cmd.smtp.with_attachments".into(), " ({} attachment{})".into());

    m.insert("cmd.terminated".into(), "Command terminated (Ctrl+C)".into());

    m.insert("cmd.tl.dir_not_found".into(), "Directory not found: {}".into());

    m.insert("cmd.tl.dir_usage".into(), "Usage: tl dir <path>".into());

    m.insert("cmd.tl.ext_not_supported".into(), ".{} not supported — blocked".into());

    m.insert("cmd.tl.ext_usage".into(), "Usage: tl ext <extension>".into());

    m.insert("cmd.tl.file_not_found".into(), "File not found: {}".into());

    m.insert("cmd.tl.file_usage".into(), "Usage: tl file <path>".into());

    m.insert("cmd.tl.help_dir".into(), "  tl dir <path>           Show supported extensions and total lines in directory".into());

    m.insert("cmd.tl.help_ext".into(), "  tl ext <extension>      Show all files with extension and total lines".into());

    m.insert("cmd.tl.help_file".into(), "  tl file <path>          Count lines in a single file".into());

    m.insert("cmd.tl.help_sort".into(), "  tl ext|dir <target> -s <-n|-l|-r>   Sort output (-n normal, -l by line asc, -r by line desc)".into());

    m.insert("cmd.tl.highest".into(), "highest: {}  {} lines".into());

    m.insert("cmd.tl.no_ext_files".into(), "No .{} files found".into());

    m.insert("cmd.tl.no_supported_files".into(), "No supported files found in {}".into());

    m.insert("cmd.tl.not_a_file".into(), "Not a file: {}".into());

    m.insert("cmd.tl.sort_usage".into(), "Invalid sort flags. Use: -s -n (normal) | -s -l (by line, small to big) | -s -r (reverse, big to small).".into());

    m.insert("cmd.tl.total_files_lines".into(), "total: {} files, {} lines".into());

    m.insert("cmd.tl.unknown_sub".into(), "Unknown subcommand '{}'.".into());

    m.insert("cmd.tl.usage_title".into(), "Usage:".into());

    m.insert("cmd.todo.add_usage".into(), "Usage: todo add \"task name\" [-> N]".into());

    m.insert("cmd.todo.all_cleared".into(), "All tasks cleared.".into());

    m.insert("cmd.todo.cancelled".into(), "Cancelled.".into());

    m.insert("cmd.todo.clear_prompt".into(), "Clear all {} tasks? [y/N] ".into());

    m.insert("cmd.todo.empty_name".into(), "Task name cannot be empty".into());

    m.insert("cmd.todo.help_add".into(), "  todo add \\\"name\\\" [-> N]  Add task (optionally at position N)".into());

    m.insert("cmd.todo.help_clear".into(), "  todo clear              Clear all tasks (with confirmation)".into());

    m.insert("cmd.todo.help_info".into(), "  todo info [N]           Show all tasks or task N".into());

    m.insert("cmd.todo.help_rm".into(), "  todo rm <N>             Remove task N".into());

    m.insert("cmd.todo.help_show".into(), "  todo                    Show all tasks".into());

    m.insert("cmd.todo.info_usage".into(), "Usage: todo info <N>  — N must be a number".into());

    m.insert("cmd.todo.info_usage".into(), "Usage: todo info <N>  — N must be a number".into());

    m.insert("cmd.todo.invalid_position".into(), "Invalid position: '{}' must be a number".into());

    m.insert("cmd.todo.list_title".into(), "TODO LIST:".into());

    m.insert("cmd.todo.list_title".into(), "TODO LIST:".into());

    m.insert("cmd.todo.missing_position".into(), "Missing position number after '->'".into());

    m.insert("cmd.todo.missing_quote".into(), "Missing closing quote".into());

    m.insert("cmd.todo.must_be_quoted".into(), "Task name must be in quotes".into());

    m.insert("cmd.todo.no_tasks".into(), "  {}".into());

    m.insert("cmd.todo.none_to_clear".into(), "No tasks to clear.".into());

    m.insert("cmd.todo.one_based".into(), "Task numbers are 1-based (minimum 1)".into());

    m.insert("cmd.todo.position_one_based".into(), "Position must be 1-based (minimum 1)".into());

    m.insert("cmd.todo.rm_usage".into(), "Usage: todo rm <N>".into());

    m.insert("cmd.todo.rm_usage_num".into(), "Usage: todo rm <N>  — N must be a number".into());

    m.insert("cmd.todo.task_added".into(), "Task added: {}".into());

    m.insert("cmd.todo.task_not_found".into(), "Task #{} not found (have {} tasks)".into());

    m.insert("cmd.todo.task_removed".into(), "Task removed: {}".into());

    m.insert("cmd.todo.unexpected_text".into(), "Unexpected text after task name: '{}'".into());

    m.insert("cmd.todo.unknown_sub".into(), "Unknown todo subcommand: '{}'".into());

    m.insert("cmd.todo.usage_title".into(), "Usage:".into());

    m.insert("cmd.todo.usage_title".into(), "Usage:".into());

    m.insert("cmd.todo.help_done".into(), "  todo done <N>           Mark task N as done".into());
    m.insert("cmd.todo.help_undo".into(), "  todo undo <N>           Mark task N as not done".into());
    m.insert("cmd.todo.help_edit".into(), "  todo edit <N> \"name\"    Rename task N".into());
    m.insert("cmd.todo.done_usage".into(), "Usage: todo done <N>".into());
    m.insert("cmd.todo.undo_usage".into(), "Usage: todo undo <N>".into());
    m.insert("cmd.todo.edit_usage".into(), "Usage: todo edit <N> \"new name\"".into());
    m.insert("cmd.todo.task_done".into(), "Task done: {}".into());
    m.insert("cmd.todo.task_undone".into(), "Task reopened: {}".into());
    m.insert("cmd.todo.task_edited".into(), "Task updated: {}".into());
    m.insert("cmd.todo.already_done".into(), "Task #{} is already done".into());
    m.insert("cmd.todo.already_open".into(), "Task #{} is already open".into());

    m.insert("cmd.arc.create_usage".into(), "Usage: arc create <out.zip|tar.gz|tgz|tar> <files...>".into());
    m.insert("cmd.arc.extract_usage".into(), "Usage: arc extract <archive> [dest]".into());
    m.insert("cmd.arc.list_usage".into(), "Usage: arc list <archive>".into());
    m.insert("cmd.arc.bad_ext".into(), "Unsupported archive type (use .zip, .tar.gz/.tgz, .tar)".into());
    m.insert("cmd.arc.not_found".into(), "Not found: {}".into());
    m.insert("cmd.arc.no_inputs".into(), "No input files to archive.".into());
    m.insert("cmd.arc.created".into(), "Archive created: {} ({} entries)".into());
    m.insert("cmd.arc.extracted".into(), "Extracted {} files to {}".into());
    m.insert("cmd.arc.list_title".into(), "ARCHIVE: {}".into());
    m.insert("cmd.arc.total".into(), "Total: {} entries".into());
    m.insert("cmd.arc.unknown_sub".into(), "Unknown arc subcommand: '{}'".into());
    m.insert("cmd.arc.usage_title".into(), "Usage:".into());
    m.insert("cmd.arc.help_create".into(), "  arc create <out> <files...>  Create archive (.zip, .tar.gz, .tar)".into());
    m.insert("cmd.arc.help_extract".into(), "  arc extract <file> [dest]   Extract archive".into());
    m.insert("cmd.arc.help_list".into(), "  arc list <file>              List archive contents".into());
    m.insert("cmd.arc.examples_title".into(), "Examples:".into());

    m.insert("cmd.upgrade.after_restart".into(), "The new version will be ready after restart.".into());

    m.insert("cmd.upgrade.ahead".into(), "Local version ({}) is ahead of latest release ({}).".into());
    m.insert("cmd.upgrade.already_latest".into(), "ntc {} is already the latest version.".into());
    m.insert("cmd.upgrade.api_parse_fail".into(), "Failed to parse GitHub API response".into());

    m.insert("cmd.upgrade.api_read_fail".into(), "Failed to read GitHub API response".into());

    m.insert("cmd.upgrade.api_request_fail".into(), "GitHub API request failed: {}".into());

    m.insert("cmd.upgrade.check_fail".into(), "Failed to check for updates: {}".into());

    m.insert("cmd.upgrade.checking".into(), "Checking for updates (current version: {})...".into());

    m.insert("cmd.upgrade.copy_fail".into(), "Failed to copy binary: {}".into());

    m.insert("cmd.upgrade.copy_fail_android".into(), "Failed to copy binary. Check disk space or permissions.".into());

    m.insert("cmd.upgrade.dev_build".into(), "You may be running a development build.".into());

    m.insert("cmd.upgrade.download_complete".into(), "Download complete. Installing update...".into());

    m.insert("cmd.upgrade.download_fail".into(), "Download failed: {}".into());

    m.insert("cmd.upgrade.downloading".into(), "Downloading update...".into());

    m.insert("cmd.upgrade.downloading_url".into(), "Downloading: {}".into());

    m.insert("cmd.upgrade.exe_path_fail".into(), "Failed to get executable path: {}".into());

    m.insert("cmd.upgrade.install_fail".into(), "Installation failed: {}".into());

    m.insert("cmd.upgrade.missing_tag".into(), "Missing 'tag_name' in GitHub release response".into());

    m.insert("cmd.upgrade.new_version".into(), "New version available: ntc {}".into());

    m.insert("cmd.upgrade.no_assets_release".into(), "No assets found in release v{}".into());

    m.insert("cmd.upgrade.perm_fail".into(), "Failed to set executable permissions: {}".into());

    m.insert("cmd.upgrade.progress_bytes".into(), "Downloaded: {} bytes".into());

    m.insert("cmd.upgrade.progress_pct".into(), "Progress: {}%".into());

    m.insert("cmd.upgrade.restart".into(), "Restart ntc to use the new version.".into());

    m.insert("cmd.upgrade.stream_fail".into(), "Failed to read download stream".into());

    m.insert("cmd.upgrade.sudo_hint".into(), "Run with sudo/administrator privileges".into());

    m.insert("cmd.upgrade.time_remaining".into(), "Time remaining: {:>3} seconds  ".into());

    m.insert("cmd.upgrade.tmpdir_fail".into(), "Failed to create temp directory: {}".into());

    m.insert("cmd.upgrade.unsupported_platform".into(), "Unsupported platform: {}".into());

    m.insert("cmd.upgrade.updated".into(), "ntc has been updated to v{}!".into());

    m.insert("cmd.upgrade.write_fail".into(), "Failed to write downloaded file".into());

    m.insert("cmd.upgrade.no_arch_asset".into(), "No prebuilt ntc binary for this architecture ({}). Build from source: cargo build --release".into());

    m.insert("cmd.upgrade.no_asset_match".into(), "No release asset matches platform key '{}'".into());

    m.insert("cmd.upgrade.extracting".into(), "Extracting binary...".into());

    m.insert("cmd.upgrade.extract_fail".into(), "Failed to read/parse downloaded archive: {}".into());

    m.insert("cmd.upgrade.no_file_in_archive".into(), "Could not find '{}' inside the downloaded archive".into());

    m.insert("cmd.upgrade.no_installer_asset".into(), "Could not find a Windows installer asset to update ntc".into());

    m.insert("cmd.upgrade.installer_launch".into(), "Launching the ntc installer (silent)...".into());

    m.insert("cmd.upgrade.installer_launch_fail".into(), "Failed to launch the installer: {}".into());

    m.insert("cmd.upgrade.home_fail".into(), "Could not determine your home directory".into());

    m.insert("cmd.upgrade.installed_user_dir".into(), "ntc installed to {} (user install, no admin/sudo required).".into());

    m.insert("cmd.upgrade.path_exported".into(), "PATH updated in {}".into());

    m.insert("config.general.autosuggest".into(), " config autosuggest ON|OFF Show/set autosuggest".into());

    m.insert("config.general.color".into(), " config color ON|OFF Show/set color".into());

    m.insert("config.general.depth".into(), " config depth [n] Show/set max depth (1-20)".into());

    m.insert("config.general.examples".into(), "Examples:".into());

    m.insert("config.general.history".into(), " config history [on|off|path] Show/set history".into());

    m.insert("config.general.lang".into(), " config lang [language] Show/set interface language".into());

    m.insert("config.general.linenumbers".into(), " config linenumbers ON|OFF Show/set line numbers".into());

    m.insert("config.general.output".into(), " config output [path] Show/set output path".into());

    m.insert("config.general.qsc".into(), " config qsc \"#c 0\" Show/set QSC global hotkey".into());

    m.insert("config.general.settings".into(), "Settings:".into());

    m.insert("config.general.threads".into(), " config threads [n] Show/set thread count".into());

    m.insert("config.general.title".into(), "config \u{2014} View or change configuration settings".into());

    m.insert("config.general.watch".into(), " config watch ON|OFF Show/set file watcher".into());

    m.insert("config.lang.set".into(), "Language set to: {}".into());

    m.insert("config.lang.set".into(), "Language set to: {}".into());

    m.insert("cvcode.formats".into(), "Formats: b2, b10, b64 (base64), hex, url".into());

    m.insert("cvcode.missing_data".into(), "Missing data to convert".into());

    m.insert("cvcode.missing_to".into(), "Missing target format after 'to'".into());

    m.insert("cvcode.usage".into(), " cvcode <from> <data> to <to> [-o <file>]".into());

    m.insert("cvcode.usage_detailed".into(), "Usage: cvcode <from> <data> to <to> [-o <file>]".into());

    m.insert("decode.unknown_format".into(), "Unknown decode format: '{}'. Use b2, b10, b64, hex, or url.".into());

    m.insert("decode.usage_cmd".into(), "Usage: decode {} <target> [-o <file>]".into());

    m.insert("decode.usage_decode_b2".into(), " decode b2 <target> [-o <file>] Binary decode".into());

    m.insert("decode.usage_decode_b10".into(), " decode b10 <target> [-o <file>] Decimal decode".into());

    m.insert("decode.usage_decode_b64".into(), " decode b64 <target> [-o <file>] Base64 decode".into());

    m.insert("decode.usage_decode_hex".into(), " decode hex <target> [-o <file>] Hex decode".into());

    m.insert("decode.usage_decode_url".into(), " decode url <target> [-o <file>] URL percent-decode".into());

    m.insert("dl.cancel_hint".into(), "Use 'dl --cancel <number|url>' to remove a record.".into());

    m.insert("dl.completed".into(), "Downloaded {} ({} in {}, {})".into());

    m.insert("dl.dns_host_required".into(), "Host is required. Usage: dns <host>".into());

    m.insert("dl.dns_looking_up".into(), "Looking up {} {} records via system tools...".into());

    m.insert("dl.dns_lookup_failed".into(), "DNS lookup failed (exit code: {})".into());

    m.insert("dl.dns_missing_type".into(), "Missing record type after -t".into());

    m.insert("dl.dns_no_records".into(), " {} No {} records found".into());

    m.insert("dl.dns_tool_failed".into(), "Failed to run DNS tool: {}. Install dig or nslookup for {} record support.".into());

    m.insert("dl.downloading".into(), "Downloading {}".into());

    m.insert("dl.failed_append".into(), "Failed to open {} for append".into());

    m.insert("dl.failed_create".into(), "Failed to create {}".into());

    m.insert("dl.failed_download".into(), "Failed to download {}".into());

    m.insert("dl.failed_read".into(), "Failed to read {}".into());

    m.insert("dl.file_still_exists".into(), "File '{}' still exists. Remove manually if desired.".into());

    m.insert("dl.help.examples".into(), "Examples:".into());

    m.insert("dl.help.opt_cancel".into(), " --cancel <number|url> Remove download record".into());

    m.insert("dl.help.opt_help".into(), " -h, --help Show this help".into());

    m.insert("dl.help.opt_o".into(), " -o, --output <file> Save as filename".into());

    m.insert("dl.help.opt_progress".into(), " --progress Show download progress bar".into());

    m.insert("dl.help.opt_resume".into(), " --resume Resume partial download".into());

    m.insert("dl.help.opt_stop".into(), " --stop Show download history".into());

    m.insert("dl.help.options".into(), "Options:".into());

    m.insert("dl.help.title".into(), "Download (dl)".into());

    m.insert("dl.help.usage".into(), "Usage:".into());

    m.insert("dl.help.usage_line".into(), " dl <url> [options]".into());

    m.insert("dl.history_title".into(), "Download History:".into());

    m.insert("dl.http_error".into(), "HTTP {}: {}".into());

    m.insert("dl.invalid_header".into(), "Invalid header format, expected \"Key: Value\", got: {}".into());

    m.insert("dl.invalid_index".into(), "Invalid index".into());

    m.insert("dl.invalid_request".into(), "Invalid request: {}".into());

    m.insert("dl.missing_cancel".into(), "Missing URL/name after --cancel".into());

    m.insert("dl.missing_data".into(), "Missing data after -d".into());

    m.insert("dl.missing_header".into(), "Missing header value after -H".into());

    m.insert("dl.missing_o".into(), "Missing filename after -o".into());

    m.insert("dl.no_records".into(), "No downloads recorded yet.".into());

    m.insert("dl.not_found".into(), "No download record found for: {}".into());

    m.insert("dl.not_found_404".into(), "URL not found (404): {}".into());

    m.insert("dl.ping_exit_code".into(), "ping exited with code: {}".into());

    m.insert("dl.ping_failed".into(), "Failed to run ping: {}".into());

    m.insert("dl.ping_host_required".into(), "Host is required. Usage: ping <host>".into());

    m.insert("dl.ping_invalid_count".into(), "Invalid count".into());

    m.insert("dl.ping_missing_count".into(), "Missing count after -c".into());

    m.insert("dl.removed_record".into(), "Removed download record: {}".into());

    m.insert("dl.request_failed".into(), "Request to {} failed".into());

    m.insert("dl.up_url_required".into(), "URL is required. Usage: up <file> <url>".into());

    m.insert("dl.upload_failed".into(), "Upload to {} failed".into());

    m.insert("dl.uploading".into(), "Uploading {} ({})".into());

    m.insert("dl.url_required".into(), "URL is required. Usage: dl <url>".into());

    m.insert("dns.help.examples".into(), "Examples:".into());

    m.insert("dns.help.opt_help".into(), " -h, --help Show this help".into());

    m.insert("dns.help.opt_type".into(), " -t, --type <type> Record type: A, AAAA, ALL, MX, NS, CNAME etc (default A)".into());

    m.insert("dns.help.opt_type_sub".into(), " MX/NS/CNAME/etc use system dig/nslookup".into());

    m.insert("dns.help.options".into(), "Options:".into());

    m.insert("dns.help.title".into(), "DNS Lookup".into());

    m.insert("dns.help.usage".into(), "Usage:".into());

    m.insert("dns.help.usage_line".into(), " dns <host> [options]".into());

    m.insert("editor.already_root".into(), "Already at root".into());

    m.insert("editor.auto_save_off".into(), "Auto-save on exit: OFF".into());

    m.insert("editor.auto_save_on".into(), "Auto-save on exit: ON".into());

    m.insert("editor.cleared_cursors".into(), "Cleared extra cursors".into());

    m.insert("editor.clip_copied_chars".into(), "Copied {} chars".into());

    m.insert("editor.clip_copied_chars".into(), "Copied {} chars".into());

    m.insert("editor.clip_copied_line".into(), "Copied line".into());

    m.insert("editor.clip_copied_line".into(), "Copied line".into());

    m.insert("editor.clip_cut_chars".into(), "Cut {} chars".into());

    m.insert("editor.clip_cut_chars".into(), "Cut {} chars".into());

    m.insert("editor.clip_cut_line".into(), "Cut line".into());

    m.insert("editor.clip_cut_line".into(), "Cut line".into());

    m.insert("editor.clip_empty".into(), "Clipboard is empty".into());

    m.insert("editor.clip_empty".into(), "Clipboard is empty".into());

    m.insert("editor.clip_yanked".into(), "Line yanked".into());

    m.insert("editor.clip_yanked".into(), "Line yanked".into());

    m.insert("editor.clipboard_empty".into(), "Clipboard is empty".into());

    m.insert("editor.close_btn".into(), "\u{2715}".into());

    m.insert("editor.cmd_already_at_root".into(), "Already at root".into());

    m.insert("editor.cmd_auto_off".into(), "Auto-save on exit: OFF".into());

    m.insert("editor.cmd_auto_on".into(), "Auto-save on exit: ON".into());

    m.insert("editor.cmd_cd_prefix".into(), "cd:".into());

    m.insert("editor.cmd_color_off".into(), "Color: OFF".into());

    m.insert("editor.cmd_color_on".into(), "Color: ON".into());

    m.insert("editor.cmd_created_dir".into(), "Created directory:".into());

    m.insert("editor.cmd_current_theme".into(), "Current theme:".into());

    m.insert("editor.cmd_exited_hex".into(), "Exited hex mode (changes discarded)".into());

    m.insert("editor.cmd_gos_only".into(), "gos: only {} subdirectories".into());

    m.insert("editor.cmd_gos_usage".into(), "gos: usage gos <N>".into());

    m.insert("editor.cmd_goto_usage".into(), "goto: usage goto <addr> or goto 0x<hex>".into());

    m.insert("editor.cmd_hex_save_failed".into(), "Hex save failed:".into());

    m.insert("editor.cmd_hex_saved".into(), "Hex saved".into());

    m.insert("editor.cmd_jumped_to_line".into(), "Jumped to line".into());

    m.insert("editor.cmd_jumped_to_offset".into(), "Jumped to offset".into());

    m.insert("editor.cmd_mkd_failed".into(), "mkd:".into());

    m.insert("editor.cmd_mkd_missing".into(), "mkd: missing operand".into());

    m.insert("editor.cmd_mkdir_failed".into(), "mkdir:".into());

    m.insert("editor.cmd_mkdir_missing".into(), "mkdir: missing operand".into());

    m.insert("editor.cmd_mkf_failed".into(), "mkf:".into());

    m.insert("editor.cmd_mkf_missing".into(), "mkf: missing operand".into());

    m.insert("editor.cmd_no_cwd".into(), "Cannot get current directory".into());

    m.insert("editor.cmd_no_subdirs".into(), "No subdirectories".into());

    m.insert("editor.cmd_no_tp".into(), "No teleport savepoints".into());

    m.insert("editor.cmd_not_dir".into(), "is not a directory".into());

    m.insert("editor.cmd_not_exist".into(), "does not exist".into());

    m.insert("editor.cmd_removed".into(), "Removed:".into());

    m.insert("editor.cmd_removed_dir".into(), "Removed directory:".into());

    m.insert("editor.cmd_rmd".into(), "rmd:".into());

    m.insert("editor.cmd_rmd_missing".into(), "rmd: missing operand".into());

    m.insert("editor.cmd_rmf".into(), "rmf:".into());

    m.insert("editor.cmd_rmf_failed".into(), "rmf:".into());

    m.insert("editor.cmd_rmf_missing".into(), "rmf: missing operand".into());

    m.insert("editor.cmd_run_missing".into(), "run: missing command".into());

    m.insert("editor.cmd_save_failed".into(), "Save failed:".into());

    m.insert("editor.cmd_saved".into(), "Saved".into());

    m.insert("editor.cmd_syntax_off".into(), "Syntax highlighting: OFF".into());

    m.insert("editor.cmd_syntax_on".into(), "Syntax highlighting: ON".into());

    m.insert("editor.cmd_theme_list_hint".into(), "Use 'theme list' to see available themes.".into());

    m.insert("editor.cmd_theme_not_found".into(), "Theme not found".into());

    m.insert("editor.cmd_theme_switched".into(), "Switched to theme".into());

    m.insert("editor.cmd_themes".into(), "Themes:".into());

    m.insert("editor.cmd_touch_failed".into(), "touch:".into());

    m.insert("editor.cmd_touch_missing".into(), "touch: missing operand".into());

    m.insert("editor.cmd_tp_add_error".into(), "tp-add:".into());

    m.insert("editor.cmd_tp_add_missing".into(), "tp-add: missing name".into());

    m.insert("editor.cmd_tp_added".into(), "Savepoint created".into());

    m.insert("editor.cmd_tp_error".into(), "tp:".into());

    m.insert("editor.cmd_tp_jumped".into(), "Teleported to".into());

    m.insert("editor.cmd_tp_list".into(), "Teleports:".into());

    m.insert("editor.cmd_tp_not_found".into(), "Savepoint not found:".into());

    m.insert("editor.cmd_unknown".into(), "Unknown command:".into());

    m.insert("editor.cmd_visual_hint".into(), "Visual mode arrow keys to select, Esc to exit".into());

    m.insert("editor.color_off".into(), "Color: OFF".into());

    m.insert("editor.color_on".into(), "Color: ON".into());

    m.insert("editor.compl_builtin".into(), "[built-in]".into());

    m.insert("editor.compl_builtin".into(), "[built-in]".into());

    m.insert("editor.compl_math_abs".into(), "absolute value".into());

    m.insert("editor.compl_math_abs".into(), "absolute value".into());

    m.insert("editor.compl_math_acos".into(), "arccosine (radians)".into());

    m.insert("editor.compl_math_acos".into(), "arccosine (radians)".into());

    m.insert("editor.compl_math_acot".into(), "arccotangent (radians)".into());

    m.insert("editor.compl_math_acot".into(), "arccotangent (radians)".into());

    m.insert("editor.compl_math_acsc".into(), "arccosecant (radians)".into());

    m.insert("editor.compl_math_acsc".into(), "arccosecant (radians)".into());

    m.insert("editor.compl_math_asec".into(), "arcsecant (radians)".into());

    m.insert("editor.compl_math_asec".into(), "arcsecant (radians)".into());

    m.insert("editor.compl_math_asin".into(), "arcsine (radians)".into());

    m.insert("editor.compl_math_asin".into(), "arcsine (radians)".into());

    m.insert("editor.compl_math_atan".into(), "arctangent (radians)".into());

    m.insert("editor.compl_math_atan".into(), "arctangent (radians)".into());

    m.insert("editor.compl_math_avg".into(), "average of values".into());

    m.insert("editor.compl_math_avg".into(), "average of values".into());

    m.insert("editor.compl_math_ceil".into(), "ceiling (round up)".into());

    m.insert("editor.compl_math_ceil".into(), "ceiling (round up)".into());

    m.insert("editor.compl_math_cos".into(), "cosine (radians)".into());

    m.insert("editor.compl_math_cos".into(), "cosine (radians)".into());

    m.insert("editor.compl_math_cot".into(), "cotangent (radians)".into());

    m.insert("editor.compl_math_cot".into(), "cotangent (radians)".into());

    m.insert("editor.compl_math_csc".into(), "cosecant (radians)".into());

    m.insert("editor.compl_math_csc".into(), "cosecant (radians)".into());

    m.insert("editor.compl_math_floor".into(), "floor (round down)".into());

    m.insert("editor.compl_math_floor".into(), "floor (round down)".into());

    m.insert("editor.compl_math_ln".into(), "natural logarithm".into());

    m.insert("editor.compl_math_ln".into(), "natural logarithm".into());

    m.insert("editor.compl_math_log10".into(), "base-10 logarithm".into());

    m.insert("editor.compl_math_log10".into(), "base-10 logarithm".into());

    m.insert("editor.compl_math_log2".into(), "base-2 logarithm".into());

    m.insert("editor.compl_math_log2".into(), "base-2 logarithm".into());

    m.insert("editor.compl_math_max".into(), "maximum of values".into());

    m.insert("editor.compl_math_max".into(), "maximum of values".into());

    m.insert("editor.compl_math_min".into(), "minimum of values".into());

    m.insert("editor.compl_math_min".into(), "minimum of values".into());

    m.insert("editor.compl_math_pow".into(), "power (x^y)".into());

    m.insert("editor.compl_math_pow".into(), "power (x^y)".into());

    m.insert("editor.compl_math_rand".into(), "random number".into());

    m.insert("editor.compl_math_rand".into(), "random number".into());

    m.insert("editor.compl_math_round".into(), "round to nearest".into());

    m.insert("editor.compl_math_round".into(), "round to nearest".into());

    m.insert("editor.compl_math_sec".into(), "secant (radians)".into());

    m.insert("editor.compl_math_sec".into(), "secant (radians)".into());

    m.insert("editor.compl_math_sin".into(), "sine (radians)".into());

    m.insert("editor.compl_math_sin".into(), "sine (radians)".into());

    m.insert("editor.compl_math_sqrt".into(), "square root".into());

    m.insert("editor.compl_math_sqrt".into(), "square root".into());

    m.insert("editor.compl_math_sum".into(), "sum of values".into());

    m.insert("editor.compl_math_sum".into(), "sum of values".into());

    m.insert("editor.compl_math_tan".into(), "tangent (radians)".into());

    m.insert("editor.compl_math_tan".into(), "tangent (radians)".into());

    m.insert("editor.compl_math_tobinary".into(), "convert to binary".into());

    m.insert("editor.compl_math_tobinary".into(), "convert to binary".into());

    m.insert("editor.compl_math_todecimal".into(), "convert to decimal".into());

    m.insert("editor.compl_math_todecimal".into(), "convert to decimal".into());

    m.insert("editor.compl_math_tohb".into(), "convert to human-readable bytes".into());

    m.insert("editor.compl_math_tohb".into(), "convert to human-readable bytes".into());

    m.insert("editor.compl_math_tohex".into(), "convert to hex".into());

    m.insert("editor.compl_math_tohex".into(), "convert to hex".into());

    m.insert("editor.compl_math_tooctal".into(), "convert to octal".into());

    m.insert("editor.compl_math_tooctal".into(), "convert to octal".into());

    m.insert("editor.compl_other".into(), "[other]".into());

    m.insert("editor.compl_other".into(), "[other]".into());

    m.insert("editor.compl_print_values".into(), "print value(s)".into());

    m.insert("editor.compl_print_values".into(), "print value(s)".into());

    m.insert("editor.compl_return_value".into(), "return a value".into());

    m.insert("editor.compl_return_value".into(), "return a value".into());

    m.insert("editor.completion_title".into(), "completions".into());

    m.insert("editor.copied_line".into(), "Copied line".into());

    m.insert("editor.cut_line".into(), "Cut line".into());

    m.insert("editor.edit_added_cursor".into(), "Added cursor ({} total)".into());

    m.insert("editor.edit_added_cursor".into(), "Added cursor ({} total)".into());

    m.insert("editor.edit_already_selected".into(), "Already selected".into());

    m.insert("editor.edit_already_selected".into(), "Already selected".into());

    m.insert("editor.edit_no_more_occurrences".into(), "No more occurrences".into());

    m.insert("editor.edit_no_more_occurrences".into(), "No more occurrences".into());

    m.insert("editor.edit_wrap".into(), "wrap".into());

    m.insert("editor.edit_wrap".into(), "wrap".into());

    m.insert("editor.exited_hex".into(), "Exited hex mode (changes discarded)".into());

    m.insert("editor.exited_multicursor".into(), "Exited multi-cursor mode".into());

    m.insert("editor.ff_prompt_find_empty".into(), "Find: Type to search...".into());

    m.insert("editor.ff_prompt_find_prefix".into(), "Find:".into());

    m.insert("editor.ff_prompt_gs_prefix".into(), "gs:".into());

    m.insert("editor.file_changed_reload".into(), "File changed externally \u{2014} reloaded".into());

    m.insert("editor.gosc_exit_hint".into(), "0=exit ".into());

    m.insert("editor.gosc_hint".into(), " Enter=go -N=back N Esc=close ".into());

    m.insert("editor.gosc_title".into(), " gosc \u{2014} Navigate Continuously ".into());

    m.insert("editor.gs_loading".into(), " finding... {} pls wait".into());

    m.insert("editor.help_screen".into(), " NORMAL MODE (vim-style verbs + motion)\n i=insert v=visual q=query n=save c=copy e=nfe f=find b=files t=term u=encoding g=gmode :=cmd ?=help 0=exit(dirty-checked)\n hjkl/awsd/arrows move \u{B7} Home/End line \u{B7} PgUp/PgDn page \u{B7} ^J=jump\n\n GMODE (press g, type, Enter)\n t/T=next/prev tab qt=close tab aqt=close all N=goto line q=exit Esc=cancel\n\n NFE SIDEBAR (press e)\n hjkl/arrows/wasd move \u{B7} Enter=dir-cd/file-open(stay) \u{B7} Esc=back\n\n INSERT MODE (editing)\n type to insert \u{B7} arrows move \u{B7} Esc=normal ^S=save ^Z=undo ^R=redo ^D=multi ^F=find ^N=files ^P=paste ^T=term ^J=jump ^E=cmd \u{B7} Home/End line \u{B7} PgUp/PgDn page \u{B7} F3/Sh+F3 find next/prev\n\n VISUAL MODE (selection)\n hjkl/arrows move (Shift selects) \u{B7} a=select-all toggle c=copy x=cut y=yank p=paste :=cmd e=nfe f=find g=gmode \u{B7} Esc=exit\n\n COMMAND MODE\n :<number> go to line number\n :l N or :line N go to line N\n :q or :quit quit editor (with save prompt)\n :wq save and quit\n :w or :write save file\n :auto on enable auto-save\n :auto off disable auto-save\n :syntax on enable syntax highlighting\n :syntax off disable syntax highlighting\n :color on|off toggle color output\n :cd <path> change directory (Navigator)\n :back [N] go to parent directory (N levels)\n :gos <N> navigate to Nth subdirectory\n :gosc list numbered subdirectories\n :mkdir <dir> create directory\n :mkd <dir> create directory (alias for :mkdir)\n :touch <file> create new file\n :mkf <file> create new file (alias for :touch)\n :rmd <target> recursively delete directory\n :rmf <target> force delete file or directory\n :rnmf <src> <dst> rename file\n :rnmd <src> <dst> rename directory\n :ne <file> navigate & edit (create if missing)\n :ne . show CWD in sidebar\n :theme <name> switch to theme by name\n :theme list list available themes\n :theme show current theme\n :git diff or :gd show git diff of current file\n :git blame or :blame show blame of current line\n :tp <name> jump to teleport savepoint\n :tp-add <name> save CWD as teleport\n :gs <pattern> search file contents across project\n :run <command> execute command in persistent run panel\n Esc cancel\n\n QUERY MODE (q in Normal, Ctrl+Alt+Q in Insert, :x in Command — leading / optional)\n /s 'text' select next occurrence after cursor (wraps); bare /s selects word under cursor\n /l N go to line N (cursor to line start)\n /sa 'text' select all occurrences in buffer\n /sal select all lines\n /sl N select line N; /sl A B range A..B (any order); /sl A , B , … discrete lines\n /rp <find> <replace> replace in selection (selection required; quote multi-word: /rp 'a b' 'c')\n /add <text> to #end|#start|#hm append/prepend on selected lines (selection required)\n /set <pat> to #end|#start|#hm align first occurrence of pat (selection required)\n /c | /cs clear selection\n /upper uppercase selection (selection required)\n /lower lowercase selection (selection required)\n /qal add <n> <q> add query alias (e.g. /qal sw /rp ${1} ${2})\n /qal rm <n> remove alias\n /qal info <n> show alias\n /qal edit <n> <q> update alias\n /qal rnm <a> to <b> rename alias\n /qal list list aliases\n <name> [args] run alias directly (no / needed)\n /undo undo\n /redo redo\n /exit /quit /Esc exit Query Mode\n ; chain multiple commands\n ${1} ${2} alias positional params\n ${1:-default} alias param with default\n ${1[]} alias array param (all remaining args)\n\n TERMINAL / RUN PANEL\n Ctrl+T toggle terminal panel\n :run <command> execute command, opens persistent run panel\n mouse click panel to focus, click editor to return\n Esc (in panel) run: close | terminal: return to editor\n drag \u{2500}\u{2500}\u{2500} row resize panel height".into());

    m.insert("editor.hex_cannot_read".into(), "hex: cannot read file: {}".into());

    m.insert("editor.hex_cannot_read".into(), "hex: cannot read file: {}".into());

    m.insert("editor.hex_mode_hint".into(), "Hex mode — 0-9/a-f edit bytes  u=undo  Ctrl+R=redo  :=command  Esc=exit".into());

    m.insert("editor.hex_mode_hint".into(), "Hex mode — 0-9/a-f edit bytes  u=undo  Ctrl+R=redo  :=command  Esc=exit".into());

    m.insert("editor.hex_saved".into(), "Hex saved".into());

    m.insert("editor.hex_unsaved".into(), "Unsaved hex changes. Use :w to save, :q! to discard.".into());

    m.insert("editor.hex_unsaved_warning".into(), "Unsaved hex changes. Use :w to save, :q! to discard.".into());

    m.insert("editor.hex_unsaved_warning".into(), "Unsaved hex changes. Use :w to save, :q! to discard.".into());

    m.insert("editor.hint_command_actions".into(), "Enter=exec Esc=cancel".into());

    m.insert("editor.hint_command_nav".into(), "cd|back|gos|gs|mkdir|touch|ne|tp|tp-add".into());

    m.insert("editor.hint_file_finder_results".into(), "results \u{2191}\u{2193}=nav Enter=open Esc=close".into());

    m.insert("editor.hint_find".into(), "Find: {}".into());

    m.insert("editor.hint_find".into(), "Find: {}".into());

    m.insert("editor.hint_find_prefix".into(), "Find \u{2014}".into());

    m.insert("editor.hint_gosc".into(), "Go to: {}".into());

    m.insert("editor.hint_gosc".into(), "Go to: {}".into());

    m.insert("editor.hint_gosc_close".into(), "Esc=close".into());

    m.insert("editor.hint_gosc_nav".into(), "dirs type number+Enter=nav 0=exit".into());

    m.insert("editor.hint_gosc_prefix".into(), "gosc \u{2014}".into());

    m.insert("editor.hint_gs_close".into(), "Esc=close".into());

    m.insert("editor.hint_gs_done".into(), "Done: {} matches".into());

    m.insert("editor.hint_gs_done".into(), "Done: {} matches".into());

    m.insert("editor.hint_gs_done_prefix".into(), "gs \u{2014}".into());

    m.insert("editor.hint_gs_loading".into(), "gs \u{2014} searching...".into());

    m.insert("editor.hint_help".into(), " Help \u{2014} Esc/q=close \u{2191}\u{2193}=scroll".into());

    m.insert("editor.hint_hex_actions".into(), "0-9/a-f=edit u=undo ^R=redo :=cmd :w=save Esc=exit".into());

    m.insert("editor.hint_hex_edit".into(), "Hex edit — type hex bytes".into());

    m.insert("editor.hint_hex_edit".into(), "Hex edit — type hex bytes".into());

    m.insert("editor.hint_insert".into(), " Insert mode \u{2014} Esc=normal ^S=save ^Z=undo ^R=redo ^D=multi ^F=find ^N=files ^P=paste ^T=term ^J=jump".into());

    m.insert("editor.hint_insert_msg".into(), " Esc=normal".into());

    m.insert("editor.hint_insert_multi".into(), " Insert* \u{2014} Esc=exit multi-cursor ^D=nextOcc".into());

    m.insert("editor.hint_insert_multi_msg".into(), " Esc=exit multi-cursor".into());

    m.insert("editor.hint_jump".into(), "Go to line \u{2014}".into());

    m.insert("editor.hint_jump_actions".into(), "Enter=jump Esc=cancel".into());

    m.insert("editor.hint_gmode".into(), "g".into());

    m.insert("editor.hint_gmode_actions".into(), "t/T=tabs qt/aqt=close N=goto q=exit Enter=run Esc=cancel".into());

    m.insert("editor.gmode_unknown".into(), "Unknown gmode command: '{}'".into());

    m.insert("editor.hint_normal".into(), " i=insert v=visual f=find b=files e=nfe t=term g=gmode n=save c=copy Sh=select :cmd ?=help 0=exit".into());

    m.insert("editor.hint_normal_msg".into(), " i=insert v=visual :cmd ?=help 0=exit".into());

    m.insert("editor.hint_query".into(), "Query: {}".into());

    m.insert("editor.hint_query".into(), "Query: {}".into());

    m.insert("editor.hint_query_actions".into(), "Actions: /s next, /l goto, /sa all, /sal lines, /sl line, /rp replace, /add, /set, /upper, /lower, /qal, /c clear".into());

    m.insert("editor.hint_query_actions".into(), "Actions: /s next, /l goto, /sa all, /sal lines, /sl line, /rp replace, /add, /set, /upper, /lower, /qal, /c clear".into());

    m.insert("editor.hint_query_prefix".into(), "/{} — {}".into());

    m.insert("editor.hint_query_prefix".into(), "/{} — {}".into());

    m.insert("editor.hint_results".into(), "Results: {}".into());

    m.insert("editor.hint_results".into(), "Results: {}".into());

    m.insert("editor.hint_results_so_far".into(), "Results: {} so far".into());

    m.insert("editor.hint_results_so_far".into(), "Results: {} so far".into());

    m.insert("editor.hint_run_empty".into(), "Run output \u{2014} (empty)".into());

    m.insert("editor.hint_run_entry".into(), "Enter=exec Esc=cancel".into());

    m.insert("editor.hint_run_output".into(), "Run output \u{2014} lines".into());

    m.insert("editor.hint_run_scroll".into(), ":=cmd \u{2191}\u{2193}/j/k=scroll Esc=close".into());

    m.insert("editor.hint_search".into(), "Search:".into());

    m.insert("editor.hint_search_actions".into(), "Enter=next Shift+Enter=prev Esc=close".into());

    m.insert("editor.hint_term_actions".into(), "lines PgUp/PgDn=scroll Ctrl+Shift+C=copy Ctrl+C=interrupt".into());

    m.insert("editor.hint_term_close".into(), "Esc/q=close".into());

    m.insert("editor.hint_term_exited".into(), "Terminal exited".into());

    m.insert("editor.hint_term_lines".into(), "Terminal \u{2014}".into());

    m.insert("editor.hint_terminal".into(), "Terminal — {} — {}".into());

    m.insert("editor.hint_terminal".into(), "Terminal — {} — {}".into());

    m.insert("editor.hint_visual".into(), " hjkl=move Sh=select a=all c=copy x=cut y=yank p=paste Esc=exit".into());

    m.insert("editor.hint_visual_no_msg".into(), " Visual mode \u{2014} hjkl/arrows move \u{00B7} Shift selects \u{00B7} a=all c=copy x=cut y=yank p=paste".into());

    m.insert("editor.hover_title".into(), " info ".into());

    m.insert("editor.hyper_created".into(), "Hyper project '{}' created successfully!".into());

    m.insert("editor.hyper_created".into(), "Hyper project '{}' created successfully!".into());

    m.insert("editor.hyper_creating".into(), "Creating hyper project '{}' (C)...".into());

    m.insert("editor.hyper_creating".into(), "Creating hyper project '{}' (C)...".into());

    m.insert("editor.hyper_dir_exists".into(), "Directory '{}' already exists.".into());

    m.insert("editor.hyper_dir_exists".into(), "Directory '{}' already exists.".into());

    m.insert("editor.hyper_name_with_ext".into(), "Project name '{}' contains an extension. Use a name without dots.".into());

    m.insert("editor.hyper_name_with_ext".into(), "Project name '{}' contains an extension. Use a name without dots.".into());

    m.insert("editor.hyper_unsupported_lang".into(), "Unsupported language '{}'. Only 'c' is supported.".into());

    m.insert("editor.hyper_unsupported_lang".into(), "Unsupported language '{}'. Only 'c' is supported.".into());

    m.insert("editor.line_deleted".into(), "Line deleted".into());

    m.insert("editor.line_selected".into(), "Line selected".into());

    m.insert("editor.line_yanked".into(), "Line yanked".into());

    m.insert("editor.max_tabs".into(), "Max tabs ({}) reached".into());

    m.insert("editor.mkd_missing".into(), "mkd: missing operand".into());

    m.insert("editor.mkdir_missing".into(), "mkdir: missing operand".into());

    m.insert("editor.mkf_missing".into(), "mkf: missing operand".into());

    m.insert("editor.mode.command".into(), "[Command]".into());

    m.insert("editor.mode.finder".into(), "[Finder]".into());

    m.insert("editor.mode.gosc".into(), "[Gosc]".into());

    m.insert("editor.mode.help".into(), "[Help]".into());

    m.insert("editor.mode.hex".into(), "[HEX]".into());

    m.insert("editor.mode.insert".into(), "[Insert]".into());

    m.insert("editor.mode.insert_star".into(), "[Insert *]".into());

    m.insert("editor.mode.jump".into(), "[Jump]".into());

    m.insert("editor.mode.normal".into(), "[Normal]".into());

    m.insert("editor.mode.query".into(), "[Query]".into());

    m.insert("editor.mode.run".into(), "[Run]".into());

    m.insert("editor.mode.search".into(), "[Search]".into());

    m.insert("editor.mode.term".into(), "[Term]".into());

    m.insert("editor.mode.visual".into(), "[Visual]".into());

    m.insert("editor.mode_cleared_cursors".into(), "Cleared extra cursors".into());

    m.insert("editor.mode_cleared_cursors".into(), "Cleared extra cursors".into());

    m.insert("editor.mode_command".into(), "[Command]".into());

    m.insert("editor.mode_exited_multi".into(), "Exited multi-cursor mode".into());

    m.insert("editor.mode_exited_multi".into(), "Exited multi-cursor mode".into());

    m.insert("editor.mode_finder".into(), "[Finder]".into());

    m.insert("editor.mode_encoding".into(), "[Encoding]".into());

    m.insert("editor.mode_git_diff".into(), "[GitDiff]".into());
    m.insert("editor.git_no_diff".into(), "git: no diff for this file".into());
    m.insert("editor.git_no_blame".into(), "git: no blame info for this line".into());
    m.insert("editor.git_disabled".into(), "git integration is disabled (config git ON)".into());
    m.insert("editor.git_quit_hint".into(), "Esc/q to close".into());

    m.insert("editor.mode_gosc".into(), "[Gosc]".into());

    m.insert("editor.mode_help".into(), "[Help]".into());

    m.insert("editor.mode_hex".into(), "[HEX]".into());

    m.insert("editor.mode_insert".into(), "[Insert]".into());

    m.insert("editor.mode_insert_multi".into(), "[Insert *]".into());

    m.insert("editor.mode_jump".into(), "[Jump]".into());

    m.insert("editor.mode_jump_range".into(), "Line {} out of range (1-{})".into());

    m.insert("editor.mode_jump_range".into(), "Line {} out of range (1-{})".into());

    m.insert("editor.mode_normal".into(), "[Normal]".into());

    m.insert("editor.mode_gmode".into(), "[Gmode]".into());

    m.insert("editor.mode_nfe".into(), "[Nfe]".into());

    m.insert("editor.hint_nfe".into(), " hjkl/wasd/arrows move · Enter open · :=size · Esc back".into());

    m.insert("editor.hint_nfe_cmd_actions".into(), "Enter=apply Esc=cancel".into());

    m.insert("editor.nfe_unknown".into(), "Unknown nfe command: '{}' (try size full|def|N%)".into());

    m.insert("editor.nfe_size_usage".into(), "Usage: size full|def|N% (got '{}')".into());

    m.insert("editor.nfe_size_set".into(), "Sidebar width: {}".into());

    m.insert("editor.nfe_size_default".into(), "default (30 cols)".into());

    m.insert("editor.nfe_size_full".into(), "full".into());

    m.insert("editor.mode_query".into(), "[Query]".into());

    m.insert("editor.mode_run".into(), "[Run]".into());

    m.insert("editor.mode_run_err".into(), "run: {}".into());

    m.insert("editor.mode_run_err".into(), "run: {}".into());

    m.insert("editor.mode_search".into(), "[Search]".into());

    m.insert("editor.mode_term_copied".into(), "Copied {} chars".into());

    m.insert("editor.mode_term_copied".into(), "Copied {} chars".into());

    m.insert("editor.mode_terminal".into(), "[Term]".into());

    m.insert("editor.mode_terminal_err".into(), "Terminal error: {}".into());

    m.insert("editor.mode_terminal_err".into(), "Terminal error: {}".into());

    m.insert("editor.mode_visual".into(), "[Visual]".into());

    m.insert("editor.mode_visual_start".into(), "Visual mode — arrow keys to select, Esc to exit".into());

    m.insert("editor.mode_visual_start".into(), "Visual mode — arrow keys to select, Esc to exit".into());

    m.insert("editor.mode_zoom_hint".into(), "Zoom: Ctrl++ / Ctrl+- in terminal emulator".into());

    m.insert("editor.mode_zoom_hint".into(), "Zoom: Ctrl++ / Ctrl+- in terminal emulator".into());

    m.insert("editor.modified_marker".into(), " \u{25CF}".into());

    m.insert("editor.mouse_closed_all".into(), "Closed all tabs".into());

    m.insert("editor.mouse_closed_all".into(), "Closed all tabs".into());

    m.insert("editor.nav_line_selected".into(), "Line selected".into());

    m.insert("editor.nav_line_selected".into(), "Line selected".into());

    m.insert("editor.nav_selected_all".into(), "Selected all".into());

    m.insert("editor.nav_selected_all".into(), "Selected all".into());

    m.insert("editor.nav_split_cursor".into(), "Split cursor ({} total)".into());

    m.insert("editor.nav_split_cursor".into(), "Split cursor ({} total)".into());

    m.insert("editor.no_subdirs".into(), "No subdirectories".into());

    m.insert("editor.no_teleports".into(), "No teleport savepoints".into());

    m.insert("editor.nothing_redo".into(), "Nothing to redo".into());

    m.insert("editor.nothing_undo".into(), "Nothing to undo".into());

    m.insert("editor.opened_file".into(), "Opened {}".into());

    m.insert("editor.panel_executing".into(), "EXECUTING:".into());

    m.insert("editor.panel_lines".into(), "lines".into());

    m.insert("editor.panel_run".into(), "RUN:".into());

    m.insert("editor.panel_run_hint".into(), "j/k scroll, Esc/q close".into());

    m.insert("editor.panel_run_output".into(), "RUN OUTPUT".into());

    m.insert("editor.panel_terminal".into(), "TERMINAL".into());

    m.insert("editor.pattern_not_found".into(), "Pattern not found: {}".into());

    m.insert("editor.query_add_format".into(), "Format: /add <text> to #end|#start|#hm".into());

    m.insert("editor.query_add_format".into(), "Format: /add <text> to #end|#start|#hm".into());

    m.insert("editor.query_add_position".into(), "Position must be #end, #start, or #hm".into());

    m.insert("editor.query_add_position".into(), "Position must be #end, #start, or #hm".into());

    m.insert("editor.query_add_usage".into(), "/add <text> to #end|#start|#hm".into());

    m.insert("editor.query_add_usage".into(), "/add <text> to #end|#start|#hm".into());

    m.insert("editor.query_added_text".into(), "Added text to {} line(s)".into());

    m.insert("editor.query_added_text".into(), "Added text to {} line(s)".into());

    m.insert("editor.query_alias_not_found".into(), "Alias not found: {}".into());

    m.insert("editor.query_alias_not_found".into(), "Alias not found: {}".into());

    m.insert("editor.query_empty_pattern".into(), "Empty find pattern".into());

    m.insert("editor.query_empty_pattern".into(), "Empty find pattern".into());

    m.insert("editor.query_lowercased".into(), "Lowercased selection".into());

    m.insert("editor.query_lowercased".into(), "Lowercased selection".into());

    m.insert("editor.query_no_matches".into(), "No matches for '{}'".into());

    m.insert("editor.query_no_matches".into(), "No matches for '{}'".into());

    m.insert("editor.query_no_pattern_matches".into(), "No lines contain '{}'".into());

    m.insert("editor.query_no_pattern_matches".into(), "No lines contain '{}'".into());

    m.insert("editor.query_qal_add_usage".into(), "/qal add <name> <query>".into());

    m.insert("editor.query_qal_add_usage".into(), "/qal add <name> <query>".into());

    m.insert("editor.query_qal_added".into(), "Alias '{}' added".into());

    m.insert("editor.query_qal_added".into(), "Alias '{}' added".into());

    m.insert("editor.query_qal_edit_usage".into(), "/qal edit <name> <new_query>".into());

    m.insert("editor.query_qal_edit_usage".into(), "/qal edit <name> <new_query>".into());

    m.insert("editor.query_qal_help".into(), "/qal: add|rm|info|edit|rnm|list|run".into());

    m.insert("editor.query_qal_help".into(), "/qal: add|rm|info|edit|rnm|list|run".into());

    m.insert("editor.query_qal_info_usage".into(), "/qal info <name>".into());

    m.insert("editor.query_qal_info_usage".into(), "/qal info <name>".into());

    m.insert("editor.query_qal_list".into(), "Aliases: {}".into());

    m.insert("editor.query_qal_list".into(), "Aliases: {}".into());

    m.insert("editor.query_qal_no_aliases".into(), "No query aliases".into());

    m.insert("editor.query_qal_no_aliases".into(), "No query aliases".into());

    m.insert("editor.query_qal_not_found".into(), "Alias '{}' not found".into());

    m.insert("editor.query_qal_not_found".into(), "Alias '{}' not found".into());

    m.insert("editor.query_qal_removed".into(), "Alias '{}' removed".into());

    m.insert("editor.query_qal_removed".into(), "Alias '{}' removed".into());

    m.insert("editor.query_qal_renamed".into(), "Renamed '{}' to '{}'".into());

    m.insert("editor.query_qal_renamed".into(), "Renamed '{}' to '{}'".into());

    m.insert("editor.query_qal_rm_usage".into(), "/qal rm <name>".into());

    m.insert("editor.query_qal_rm_usage".into(), "/qal rm <name>".into());

    m.insert("editor.query_qal_rnm_usage".into(), "/qal rnm <old> to <new>".into());

    m.insert("editor.query_qal_rnm_usage".into(), "/qal rnm <old> to <new>".into());

    m.insert("editor.query_qal_updated".into(), "Alias '{}' updated".into());

    m.insert("editor.query_qal_updated".into(), "Alias '{}' updated".into());

    m.insert("editor.query_l_usage".into(), "/l <line> — go to line".into());

    m.insert("editor.query_l_usage".into(), "/l <line> — go to line".into());

    m.insert("editor.query_qal_exists".into(), "Alias '{}' already exists (use /qal edit)".into());

    m.insert("editor.query_qal_exists".into(), "Alias '{}' already exists (use /qal edit)".into());

    m.insert("editor.query_qal_reserved".into(), "Reserved name: '{}'".into());

    m.insert("editor.query_qal_reserved".into(), "Reserved name: '{}'".into());

    m.insert("editor.query_replace_format".into(), "Format: /p <find> to <replace>".into());

    m.insert("editor.query_replace_format".into(), "Format: /p <find> to <replace>".into());

    m.insert("editor.query_rp_usage".into(), "/rp <find> <replace> — replace in selection".into());

    m.insert("editor.query_rp_usage".into(), "/rp <find> <replace> — replace in selection".into());

    m.insert("editor.query_s_usage".into(), "/s \"text\" — select next occurrence (bare /s = word under cursor)".into());

    m.insert("editor.query_s_usage".into(), "/s \"text\" — select next occurrence (bare /s = word under cursor)".into());

    m.insert("editor.query_sa_usage".into(), "/sa \"text\" — select all occurrences".into());

    m.insert("editor.query_sa_usage".into(), "/sa \"text\" — select all occurrences".into());

    m.insert("editor.query_selected_lines".into(), "Selected {} line(s)".into());

    m.insert("editor.query_selected_lines".into(), "Selected {} line(s)".into());

    m.insert("editor.query_selected_next".into(), "Selected next '{}'".into());

    m.insert("editor.query_selected_next".into(), "Selected next '{}'".into());

    m.insert("editor.query_selected_occurrences".into(), "Selected {} occurrence(s) of '{}'".into());

    m.insert("editor.query_selected_occurrences".into(), "Selected {} occurrence(s) of '{}'".into());

    m.insert("editor.query_selected_word".into(), "Selected word".into());

    m.insert("editor.query_selected_word".into(), "Selected word".into());

    m.insert("editor.query_selection_required".into(), "Selection required (select with /s, /sa, /sl or /sal first)".into());

    m.insert("editor.query_selection_required".into(), "Selection required (select with /s, /sa, /sl or /sal first)".into());

    m.insert("editor.query_sl_usage".into(), "/sl <line> [line] | <line> , ... — select lines".into());

    m.insert("editor.query_sl_usage".into(), "/sl <line> [line] | <line> , ... — select lines".into());

    m.insert("editor.query_replace_usage".into(), "/p <find> to <replace>".into());

    m.insert("editor.query_replace_usage".into(), "/p <find> to <replace>".into());

    m.insert("editor.query_replaced_count".into(), "Replaced {} occurrence(s)".into());

    m.insert("editor.query_replaced_count".into(), "Replaced {} occurrence(s)".into());

    m.insert("editor.query_selected_all_lines".into(), "Selected all lines".into());

    m.insert("editor.query_selected_all_lines".into(), "Selected all lines".into());

    m.insert("editor.query_selected_line".into(), "Selected line {}".into());

    m.insert("editor.query_selected_line".into(), "Selected line {}".into());

    m.insert("editor.query_selected_lines_range".into(), "Selected lines {}-{}".into());

    m.insert("editor.query_selected_lines_range".into(), "Selected lines {}-{}".into());

    m.insert("editor.query_selection_cleared".into(), "Selection cleared".into());

    m.insert("editor.query_selection_cleared".into(), "Selection cleared".into());

    m.insert("editor.query_set_alignment".into(), "Set alignment on {} line(s)".into());

    m.insert("editor.query_set_alignment".into(), "Set alignment on {} line(s)".into());

    m.insert("editor.query_set_format".into(), "Format: /set <pattern> to #end|#start|#hm".into());

    m.insert("editor.query_set_format".into(), "Format: /set <pattern> to #end|#start|#hm".into());

    m.insert("editor.query_set_usage".into(), "/set <pattern> to #end|#start|#hm".into());

    m.insert("editor.query_set_usage".into(), "/set <pattern> to #end|#start|#hm".into());

    m.insert("editor.query_unknown".into(), "Unknown query: {}".into());

    m.insert("editor.query_unknown".into(), "Unknown query: {}".into());

    m.insert("editor.query_uppercased".into(), "Uppercased selection".into());

    m.insert("editor.query_uppercased".into(), "Uppercased selection".into());

    m.insert("editor.redo".into(), "Redo".into());

    m.insert("editor.rmd_missing".into(), "rmd: missing operand".into());

    m.insert("editor.rmf_missing".into(), "rmf: missing operand".into());

    m.insert("editor.run_exited".into(), "Process exited with code {}".into());

    m.insert("editor.run_exited".into(), "Process exited with code {}".into());

    m.insert("editor.run_missing".into(), "run: missing command".into());

    m.insert("editor.run_terminated".into(), "Process terminated".into());

    m.insert("editor.run_terminated".into(), "Process terminated".into());

    m.insert("editor.save_auto_error".into(), "Auto-save error: {}".into());

    m.insert("editor.save_auto_error".into(), "Auto-save error: {}".into());

    m.insert("editor.save_failed".into(), "Save failed: {}".into());

    m.insert("editor.encoding_changed".into(), "Encoding changed".into());

    m.insert("editor.enc_title".into(), "Encoding".into());
    m.insert("editor.enc_hint".into(), "↑↓ select · Enter confirm · Esc cancel".into());
    m.insert("editor.hint_encoding".into(), "Encoding:".into());
    m.insert("editor.encoding_reload_error".into(), "Reload failed: {}".into());
    m.insert(
        "editor.encoding_applied_on_save".into(),
        "applied on next save".into(),
    );

    m.insert("editor.save_prompt".into(), "Save file [Y|N]?".into());

    m.insert("editor.save_prompt".into(), "Save file [Y|N]?".into());

    m.insert("editor.saved".into(), "Saved \u{2713}".into());

    m.insert("editor.saved_full".into(), " Saved ✓".into());

    m.insert("editor.saved_full".into(), " Saved ✓".into());

    m.insert("editor.saved_marker".into(), " \u{2713}".into());

    m.insert("editor.scratch".into(), "[scratch]".into());

    m.insert("editor.scratch_label".into(), "[scratch]".into());

    m.insert("editor.scratch_label".into(), "[scratch]".into());

    m.insert("editor.search_pattern_not_found".into(), "Pattern not found: {}".into());

    m.insert("editor.search_pattern_not_found".into(), "Pattern not found: {}".into());

    m.insert("editor.selected_all".into(), "Selected all".into());

    m.insert("editor.status_modified".into(), "modified".into());

    m.insert("editor.status_modified_tag".into(), " \u{25cf}".into());

    m.insert("editor.status_saved".into(), "saved".into());

    m.insert("editor.status_saved_tag".into(), " \u{2713}".into());

    m.insert("editor.syntax_off".into(), "Syntax highlighting: OFF".into());

    m.insert("editor.syntax_on".into(), "Syntax highlighting: ON".into());

    m.insert("editor.tab_cannot_open".into(), "Cannot open {}".into());

    m.insert("editor.tab_cannot_open".into(), "Cannot open {}".into());

    m.insert("editor.tab_error_reading".into(), "Error reading {}: {}".into());

    m.insert("editor.tab_error_reading".into(), "Error reading {}: {}".into());

    m.insert("editor.tab_max_reached".into(), "Max tabs ({}) reached".into());

    m.insert("editor.tab_max_reached".into(), "Max tabs ({}) reached".into());

    m.insert("editor.tab_opened".into(), "Opened {}".into());

    m.insert("editor.tab_opened".into(), "Opened {}".into());

    m.insert("editor.term_title".into(), "Terminal".into());

    m.insert("editor.term_title".into(), "Terminal".into());

    m.insert("editor.tmpl_last_saved".into(), "Last saved: {}".into());

    m.insert("editor.tmpl_last_saved".into(), "Last saved: {}".into());

    m.insert("editor.tmpl_recover_prompt".into(), "Recover unsaved changes? [Y/N]: ".into());

    m.insert("editor.tmpl_recover_prompt".into(), "Recover unsaved changes? [Y/N]: ".into());

    m.insert("editor.tmpl_recovery_found".into(), "Crash recovery file found for: {}".into());

    m.insert("editor.tmpl_recovery_found".into(), "Crash recovery file found for: {}".into());

    m.insert("editor.touch_missing".into(), "touch: missing operand".into());

    m.insert("editor.tp_add_missing".into(), "tp-add: missing name".into());

    m.insert("editor.undo".into(), "Undo".into());

    m.insert("editor.unknown_cmd".into(), "Unknown command: {}".into());

    m.insert("editor.untitled".into(), "untitled".into());

    m.insert("editor.untitled_label".into(), "untitled".into());

    m.insert("editor.untitled_label".into(), "untitled".into());

    m.insert("editor.visual_hint".into(), "Visual mode \u{2014} arrow keys to select, Esc to exit".into());

    m.insert("editor.welcome.title".into(), "A fast, terminal-based code editor built in Rust.".into());

    m.insert("editor.zoom_hint".into(), "Zoom: Ctrl++ / Ctrl+- in terminal emulator".into());

    m.insert("encode.binary_hint".into(), "Use -o <file> to save the binary output.".into());

    m.insert("encode.binary_warning".into(), "Output is binary data. Displaying hex preview:".into());

    m.insert("encode.hex_even".into(), "Hex string length must be even".into());

    m.insert("encode.input_must_be_text".into(), "Input data must be text".into());

    m.insert("encode.input_not_text".into(), "{} input must be text (not binary)".into());

    m.insert("encode.invalid_b2".into(), "Invalid binary string: '{}'".into());

    m.insert("encode.invalid_b10".into(), "Invalid decimal byte: '{}'".into());

    m.insert("encode.invalid_b64".into(), "Invalid base64 input: {}".into());

    m.insert("encode.invalid_hex_pct".into(), "Invalid hex in percent-encoding: %{}".into());

    m.insert("encode.invalid_hex_pos".into(), "Invalid hex character at position {}".into());

    m.insert("encode.invalid_pct".into(), "Invalid percent-encoding".into());

    m.insert("encode.truncated_pct".into(), "Truncated percent-encoding".into());

    m.insert("encode.unknown_format".into(), "Unknown encode format: '{}'. Use b2, b10, b64, hex, or url.".into());

    m.insert("encode.unknown_format_fmt".into(), "(unknown format: {})".into());

    m.insert("encode.unknown_format_generic".into(), "Unknown format: '{}'. Use b2, b10, b64, hex, or url.".into());

    m.insert("encode.usage_cmd".into(), "Usage: encode {} <target> [-o <file>]".into());

    m.insert("encode.usage_encode_b2".into(), " encode b2 <target> [-o <file>] Binary encode (0101...)".into());

    m.insert("encode.usage_encode_b10".into(), " encode b10 <target> [-o <file>] Decimal encode (0-255...)".into());

    m.insert("encode.usage_encode_b64".into(), " encode b64 <target> [-o <file>] Base64 encode".into());

    m.insert("encode.usage_encode_hex".into(), " encode hex <target> [-o <file>] Hex encode".into());

    m.insert("encode.usage_encode_url".into(), " encode url <target> [-o <file>] URL percent-encode".into());

    m.insert("encode.usage_examples".into(), "Examples:".into());

    m.insert("encode.usage_title".into(), "Usage:".into());

    m.insert("encode.written_to".into(), "Written to: {}".into());

    m.insert("entry.goodbye".into(), "Goodbye!".into());

    m.insert("entry.ntpkg_active".into(), "ntpkg: {} runtime{} active in PATH".into());

    m.insert("entry.startup_hint".into(), "Type 'help' for available commands, 'exit' to quit.".into());

    m.insert("entry.watch_trigger".into(), "watch trigger: {}".into());

    m.insert("entry.watcher_failed".into(), "Watcher failed: {}".into());

    m.insert("entry.watcher_started".into(), "File watcher started (recursive)".into());

    m.insert("entry.welcome_fmt".into(), " Welcome to ntc {} - Navigate, Toolkit, Center ".into());

    m.insert("error.failed_write_completions".into(), "Failed to write completions: {}".into());

    m.insert("error.unsupported_shell".into(), "Unsupported shell: {}. Use: bash, zsh, fish, powershell".into());

    m.insert("fmt.help_example1".into(), "  fmt json package.json to yaml".into());

    m.insert("fmt.help_example2".into(), "  fmt json config.json to toml -o config.toml".into());

    m.insert("fmt.help_example3".into(), "  fmt yaml config.yaml to json".into());

    m.insert("fmt.help_example4".into(), "  fmt toml Cargo.toml to json".into());

    m.insert("fmt.help_example5".into(), "  fmt json data.json                      # pretty-print JSON".into());

    m.insert("fmt.help_examples".into(), "Examples:".into());

    m.insert("fmt.help_json_line".into(), "  fmt json <file> [to yaml|toml] [-o <file>]    Convert JSON to YAML or TOML".into());

    m.insert("fmt.help_no_target".into(), "  fmt json <file>                          Pretty-print JSON".into());

    m.insert("fmt.help_no_target".into(), "  fmt json <file>                          Pretty-print JSON".into());

    m.insert("fmt.help_toml_line".into(), "  fmt toml <file> [to json|yaml] [-o <file>]    Convert TOML to JSON or YAML".into());

    m.insert("fmt.help_usage".into(), "Usage:".into());

    m.insert("fmt.help_yaml_line".into(), "  fmt yaml <file> [to json|toml] [-o <file>]    Convert YAML to JSON or TOML".into());












    m.insert("gosc.calculating_size".into(), "Calculating directory size...".into());

    m.insert("gosc.exiting".into(), "Exiting gosc...".into());

    m.insert("gosc.output_zone".into(), "─── Output Zone ───────────────────────────────────".into());

    m.insert("gosc.output_zone_end".into(), "──────────────────────────────────────────────────".into());

    m.insert("gosc.panel.back_1".into(), "go back 1 level".into());

    m.insert("gosc.panel.back_2".into(), "go back 2 levels".into());

    m.insert("gosc.panel.back_n".into(), "go back n levels".into());

    m.insert("gosc.panel.empty".into(), "(no subdirectories)".into());

    m.insert("gosc.panel.exit".into(), "exit gosc".into());

    m.insert("gosc.panel.size_care".into(), "size --care (total size)".into());

    m.insert("gosc.panel.title".into(), "gosc — Navigate Continuously".into());

    m.insert("gosc.panel.view_care".into(), "view --care -d 1 (files + care)".into());

    m.insert("gosc.panel.view_size_care".into(), "view -s --care -d 1 (files + sizes + care)".into());

    m.insert("gosc.press_enter".into(), "Press Enter to continue...".into());

    m.insert("gosc.prompt".into(), "gosc>".into());

    m.insert("gosc.size_bytes".into(), "Bytes:".into());

    m.insert("gosc.size_human".into(), "Human:".into());

    m.insert("gosc.size_title".into(), "Current Directory Size".into());

    m.insert("help.examples_header".into(), "Examples:".into());

    m.insert("help.context.cr".into(), "<cr> press enter to run it".into());

    m.insert("help.alias.add".into(), "ral add <name> \"<cmd>\" ## Create alias".into());

    m.insert("help.alias.add_array".into(), "ral add <name>(x[],y) \"<cmd>\" ## Alias with array params".into());

    m.insert("help.alias.add_defaults".into(), "ral add <name>({x=a,e=txt}) \"<cmd>\" ## Alias with default values".into());

    m.insert("help.alias.add_param".into(), "ral add <name>(x) \"<cmd>\" ## Create parameterised alias with $x".into());

    m.insert("help.alias.cls".into(), "ral cls ## Clear ALL aliases (with confirmation)".into());

    m.insert("help.alias.edit".into(), "ral edit <name> \"<cmd>\" ## Update alias".into());

    m.insert("help.alias.export_all".into(), "ral export --all <name> ## Export all aliases to <name>.ntc.ral".into());

    m.insert("help.alias.export_select".into(), "ral export --select <name> ## Select aliases to export".into());

    m.insert("help.alias.import".into(), "ral import <file> ## Import aliases from .ntc.ral file".into());

    m.insert("help.alias.info".into(), "ral info <name> [--edit] ## Show alias details (--edit: copy edit command to clipboard)".into());

    m.insert("help.alias.list".into(), "ral list ## Show all aliases".into());

    m.insert("help.alias.make".into(), "ral make [name] ## Create alias template in CWD".into());

    m.insert("help.alias.rm".into(), "ral rm <name>[, ...] ## Remove alias(es)".into());

    m.insert("help.alias.rnm".into(), "ral rnm <old> to <new> ## Rename alias".into());

    m.insert("help.alias.run".into(), "run <alias> ## Execute alias with 'run'".into());

    m.insert("help.alias.run_direct".into(), "<alias> ## Execute alias directly".into());

    m.insert("help.alias.run_param".into(), "<alias>(<arg>) ## Execute parameterised alias".into());

    m.insert("help.alias.title".into(), "RUN ALIAS COMMANDS:".into());

    m.insert("help.autoup.add".into(), "autoup add <name> \"<cmd>\" ## Add a startup entry to HKCU".into());

    m.insert("help.autoup.disable".into(), "autoup disable <name> ## Disable entry (uses StartupApproved)".into());

    m.insert("help.autoup.enable".into(), "autoup enable <name> ## Re-enable a disabled entry".into());

    m.insert("help.autoup.list".into(), "autoup list ## List all startup entries (CU/LM/SF sources)".into());

    m.insert("help.autoup.rm".into(), "autoup rm <name> ## Remove a startup entry".into());

    m.insert("help.autoup.sources".into(), "[CU]=HKCU [LM]=HKLM [SF]=Startup Folder".into());

    m.insert("help.autoup.title".into(), "STARTUP MANAGER (Windows):".into());

    m.insert("help.backup.clear".into(), "backup clear ## Delete ALL backups (asks confirmation)".into());

    m.insert("help.backup.clear_force".into(), "backup clear --force ## Delete ALL backups (no confirmation)".into());

    m.insert("help.backup.create".into(), "backup create ## Create a backup of current project".into());

    m.insert("help.backup.diff".into(), "backup diff [num] ## Diff current project against a backup".into());

    m.insert("help.backup.list".into(), "backup list ## List all backups".into());

    m.insert("help.backup.recreate".into(), "backup recreate ## Clear all + undo + create fresh backup".into());

    m.insert("help.backup.restore".into(), "backup restore [num] ## Restore a backup (interactive if no num)".into());

    m.insert("help.backup.restore_choose".into(), "backup restore --choose ## Pick a backup from an interactive menu".into());

    m.insert("help.backup.restore_deleted".into(), "backup restore --deleted ## Restore only files that were deleted".into());

    m.insert("help.backup.title".into(), "BACKUP COMMANDS:".into());

    m.insert("help.backup.undo".into(), "backup undo ## Undo the last restore".into());

    m.insert("help.backup.undo_cls".into(), "backup undo --cls ## Clear undo history (asks confirmation)".into());

    m.insert("help.backup.undo_force".into(), "backup undo --force ## Clear undo history (no confirmation)".into());

    m.insert("help.clipboard.clear".into(), "clip clear ## Clear clipboard history".into());

    m.insert("help.clipboard.list".into(), "clip list ## Show clipboard history".into());

    m.insert("help.clipboard.pathcp".into(), "pathcp ## Copy current directory path to clipboard".into());

    m.insert("help.clipboard.select".into(), "clip select <N> ## Copy entry N back to clipboard".into());

    m.insert("help.clipboard.show".into(), "clip ## Show clipboard history".into());

    m.insert("help.clipboard.title".into(), "CLIPBOARD COMMANDS:".into());

    m.insert("help.config.autosuggest".into(), "config autosuggest ON|OFF ## Show/set autosuggest (ghost text)".into());

    m.insert("help.config.color".into(), "config color ON|OFF ## Show/set color output".into());

    m.insert("help.config.depth".into(), "config depth [n] ## Show/set max depth (1-20)".into());

    m.insert("help.config.history".into(), "config history [on|off|path] ## Show/set history".into());

    m.insert("help.config.lang".into(), "config lang [language] ## Show/set interface language".into());

    m.insert("help.config.linenumbers".into(), "config linenumbers ON|OFF ## Show/set line numbers".into());

    m.insert("help.config.local_deinit".into(), "local deinit ## Remove ntconfig.toml from current directory".into());

    m.insert("help.config.local_help".into(), "local help ## Show local command usage".into());

    m.insert("help.config.local_init".into(), "local init ## Create ntconfig.toml template (commented)".into());

    m.insert("help.config.local_init_all".into(), "local init --all ## Export current settings to ntconfig.toml".into());

    m.insert("help.config.opencg".into(), "opencg ## Open config.toml in editor".into());

    m.insert("help.config.output".into(), "config output [path] ## Show/set output path".into());

    m.insert("help.config.qsc".into(), "config qsc \"#c 0\" ## Set QSC global hotkey".into());

    m.insert("help.config.qsc_off".into(), "config qsc off ## Disable QSC hotkey".into());

    m.insert("help.config.resetcg".into(), "resetcg ## Reset config to defaults (with backup option)".into());

    m.insert("help.config.restorecg".into(), "restorecg ## Restore config from backup".into());

    m.insert("help.config.showcg".into(), "showcg ## Show current configuration overview".into());

    m.insert("help.config.showenv".into(), "showenv ## Show environment variables (first 5 PATH entries)".into());

    m.insert("help.config.showenv_all".into(), "showenv --all ## Show all PATH entries".into());

    m.insert("help.config.showenv_d".into(), "showenv -d <n> ## Show first n PATH entries".into());

    m.insert("help.config.threads".into(), "config threads [n] ## Show/set thread count".into());

    m.insert("help.config.title".into(), "CONFIGURATION COMMANDS:".into());

    m.insert("help.config.watch".into(), "config watch ON|OFF ## Enable/disable file watcher".into());

    m.insert("help.config.watch_trigger".into(), "config watch trigger <a> ## Auto-run alias when watcher fires".into());

    m.insert("help.config.watch_trigger_off".into(), "config watch trigger off ## Disable auto-run".into());

    m.insert("help.config.watch_limit".into(), "config watch limit <n> ## Rows shown in watcher change box".into());

    m.insert("help.config.wi".into(), "wi [path] ## Live file-change feed (q=quit)".into());




    m.insert("help.editor.ne".into(), "ne, ntceditor <file> ## Open file in built-in text editor".into());

    m.insert("help.editor.ne_hyper".into(), "ne --hyper <name> --lang <l> ## Create C/C++ hyper project".into());

    m.insert("help.editor.ne_init".into(), "ne --init <file> ## Create an empty file template".into());

    m.insert("help.editor.ne_line".into(), "ne, ntceditor <file>:<n> ## Open file at line <n>".into());

    m.insert("help.editor.ne_super".into(), "ne --super <file> ## Create a super file template".into());

    m.insert("help.editor.ne_welcome".into(), "ne, ntceditor ## Open welcome screen in current directory".into());

    m.insert("help.editor.title".into(), "EDITOR COMMANDS:".into());

    m.insert("help.encdec.cvcode".into(), "cvcode <from> <data> to <to> ## Convert between formats".into());

    m.insert("help.encdec.decode_b2".into(), "decode b2 <target> [-o <file>] ## Binary decode".into());

    m.insert("help.encdec.decode_b10".into(), "decode b10 <target> [-o <file>] ## Decimal decode".into());

    m.insert("help.encdec.decode_b64".into(), "decode b64 <target> [-o <file>] ## Base64 decode".into());

    m.insert("help.encdec.decode_hex".into(), "decode hex <target> [-o <file>] ## Hex decode".into());

    m.insert("help.encdec.decode_url".into(), "decode url <target> [-o <file>] ## URL percent-decode".into());

    m.insert("help.encdec.encode_b2".into(), "encode b2 <target> [-o <file>] ## Binary encode (0101...)".into());

    m.insert("help.encdec.encode_b10".into(), "encode b10 <target> [-o <file>] ## Decimal encode (0-255...)".into());

    m.insert("help.encdec.encode_b64".into(), "encode b64 <target> [-o <file>] ## Base64 encode (file or raw text)".into());

    m.insert("help.encdec.encode_hex".into(), "encode hex <target> [-o <file>] ## Hex encode".into());

    m.insert("help.encdec.encode_url".into(), "encode url <target> [-o <file>] ## URL percent-encode".into());

    m.insert("help.encdec.title".into(), "ENCODE/DECODE COMMANDS:".into());

    m.insert("help.explorer.hint1".into(), "h=help, l=go to line, b=parent dir, f=find".into());

    m.insert("help.explorer.hint2".into(), "/=filter, e=editor, r=rename, d=delete".into());

    m.insert("help.explorer.hint3".into(), "n=new file, m=new dir, i=hidden toggle".into());

    m.insert("help.explorer.hint4".into(), "s/sort toggle, S=sort reverse, F5=refresh".into());

    m.insert("help.explorer.hint5".into(), "q/Esc=quit, mouse=click/scroll".into());

    m.insert("help.explorer.launch".into(), "explorer, nfe [PATH] ## Launch the full-screen terminal file explorer".into());

    m.insert("help.explorer.title".into(), "FILE EXPLORER (NFE):".into());

    m.insert("help.fileops.cp".into(), "cp <src> <dst|@tp> [-f] ## Copy file or dir".into());

    m.insert("help.fileops.mkd".into(), "mkd <dir>[, ...] ## Create a new directory".into());

    m.insert("help.fileops.mkf".into(), "mkf <file>[, ...] ## Create a new file".into());

    m.insert("help.fileops.rmd".into(), "rmd <dir>[, ...] ## Delete a directory (with confirmation)".into());

    m.insert("help.fileops.rmf".into(), "rmf <file>[, ...] ## Delete a file (with confirmation)".into());

    m.insert("help.fileops.rnmd".into(), "rnmd <old> <new> ## Rename a directory".into());

    m.insert("help.fileops.rnmf".into(), "rnmf <old> <new> ## Rename a file".into());

    m.insert("help.fileops.mv".into(), "mv <src> <dst|@tp> [-f] ## Move/rename file or dir".into());

    m.insert("help.fileops.title".into(), "FILE OPERATIONS:".into());

    m.insert("help.fmt.json".into(), "fmt json <file> [to yaml|toml] ## Convert JSON to YAML or TOML".into());

    m.insert("help.fmt.output".into(), "(add -o <file> to save output)".into());

    m.insert("help.fmt.title".into(), "FORMAT CONVERTER:".into());

    m.insert("help.fmt.toml".into(), "fmt toml <file> [to json|yaml] ## Convert TOML to JSON or YAML".into());

    m.insert("help.fmt.yaml".into(), "fmt yaml <file> [to json|toml] ## Convert YAML to JSON or TOML".into());

    m.insert("help.igcare.care_dir".into(), "care dir <name>[, ...] ## Stop ignoring directories".into());

    m.insert("help.igcare.care_ext".into(), "care ext <ext>[, ...] ## Add extra supported extensions".into());

    m.insert("help.igcare.care_file".into(), "care file <name>[, ...] ## Add extra supported files".into());

    m.insert("help.igcare.export_all".into(), "igcare export --all <n> ## Export all settings to <name>.ntc.igcare".into());

    m.insert("help.igcare.export_select".into(), "igcare export --select <n> ## Select categories to export".into());

    m.insert("help.igcare.ignore_dir".into(), "ignore dir <name>[, ...] ## Ignore directories by name".into());

    m.insert("help.igcare.ignore_ext".into(), "ignore ext <ext>[, ...] ## Ignore file extensions".into());

    m.insert("help.igcare.ignore_file".into(), "ignore file <name>[, ...] ## Ignore specific files".into());

    m.insert("help.igcare.import".into(), "igcare import <file> ## Import settings from .ntc.igcare file".into());

    m.insert("help.igcare.make".into(), "igcare make [name] ## Create igcare template in CWD".into());

    m.insert("help.igcare.show".into(), "igcared ## Show all ignore/care settings".into());

    m.insert("help.igcare.title".into(), "IGNORE / CARE COMMANDS:".into());

    m.insert("help.image.convert".into(), "image <path> to <fmt> ## Convert and display output (json|md|html|txt|desc)".into());

    m.insert("help.image.display".into(), "image display <path> ## Render image inline in the terminal".into());

    m.insert("help.image.show".into(), "image show <path> ## Render image inline in the terminal".into());

    m.insert("help.image.interactive".into(), "image ## Interactive image converter (pick from current dir)".into());

    m.insert("help.image.layout".into(), "image <path> --layout ## Enable UI layout detection".into());

    m.insert("help.image.ocr".into(), "image <path> --ocr ## Enable OCR text extraction".into());

    m.insert("help.image.open".into(), "image <path> ## Open image in default viewer".into());

    m.insert("help.image.save".into(), "image <path> out <fmt> ## Convert and save to setO path".into());

    m.insert("help.image.title".into(), "IMAGE COMMANDS:".into());

    m.insert("help.math.eval".into(), "math <expr> ## Evaluate a math expression".into());

    m.insert("help.math.file".into(), "math file.ntc.math ## Compile and run a math file".into());

    m.insert("help.math.fun".into(), "math fun add/rm/edit/info/ls ## Manage custom math functions".into());

    m.insert("help.math.timer".into(), "math timer [sec] ## Lap timer or countdown with alarm".into());

    m.insert("help.math.title".into(), "MATH COMMANDS:".into());

    m.insert("help.mode.admin".into(), "mode admin ## Exit and re-launch ntc as Administrator".into());

    m.insert("help.mode.normal".into(), "mode normal ## Exit and re-launch ntc without elevation".into());

    m.insert("help.mode.title".into(), "WINDOWS MODE (Windows):".into());

    m.insert("help.nav.back".into(), "back [n] ## Go back to parent directory (or n levels)".into());

    m.insert("help.nav.cd".into(), "cd <path> ## Navigate to a directory".into());

    m.insert("help.nav.cd_tp".into(), "cd to <tp_name> ## Teleport to a saved teleport point".into());

    m.insert("help.nav.fgo".into(), "fgo <pattern> [-d <n>] ## Find files, navigate to selected file's parent dir".into());

    m.insert("help.nav.godrive".into(), "godrive ## List and select Windows drives".into());

    m.insert("help.nav.gos".into(), "gos ## List subdirectories and pick one".into());

    m.insert("help.nav.gosc".into(), "gosc ## Continuous navigation (0 to exit, v/s/vs for view)".into());

    m.insert("help.nav.title".into(), "NAVIGATION COMMANDS:".into());

    m.insert("help.nav.where".into(), "where ## Show ntc executable and config locations".into());

    m.insert("help.nav.example1".into(), "cd src ## Navigate to the src directory".into());

    m.insert("help.nav.example2".into(), "back ## Go back to parent directory".into());

    m.insert("help.nav.example3".into(), "gos ## Interactive subdirectory selection".into());

    m.insert("help.nav.example4".into(), "cd to web ## Teleport to 'web' savepoint".into());

    m.insert("help.network.dl".into(), "dl <url> ## Download file (--progress, --resume, --stop, --cancel)".into());

    m.insert("help.network.dns".into(), "dns <host> ## DNS lookup (-t record type: A, AAAA, ALL, MX, NS, etc)".into());

    m.insert("help.network.http".into(), "http <method> <url> ## HTTP API client (-H, -d, --json, -q, -a, --bearer, -s, -o, -v, -t)".into());

    m.insert("help.network.ping".into(), "ping <host> ## ICMP ping (-c count)".into());

    m.insert("help.network.ipcfg".into(), "ipcfg [name] ## Show adapters like ipconfig/ifconfig".into());

    m.insert("help.network.serve".into(), "serve [port] ## HTTP file server (--auth, --tunnel, --share, --spa, --web-dir)".into());

    m.insert("help.network.smtp".into(), "smtp ## SMTP email client (--id, -i, -u, -pwd, -s, -b, -f, --to)".into());

    m.insert("help.network.spec".into(), "spec ## Show computer/system specifications (CPU, GPU, RAM, storage, OS, serial)".into());

    m.insert("help.network.sys".into(), "sys, top ## Live system monitor (CPU, memory, disk, processes)".into());

    m.insert("help.network.title".into(), "SYSTEM & NETWORK COMMANDS:".into());

    m.insert("help.network.up".into(), "up <file> <url> ## Upload file (--post, -H, -d)".into());

    m.insert("help.ntpkg.available".into(), "ntpkg available [lang] ## List languages or downloadable versions".into());

    m.insert("help.ntpkg.help".into(), "ntpkg help ## Show ntpkg usage".into());

    m.insert("help.ntpkg.info".into(), "ntpkg info <lang> ## Show package info (version, location, binaries)".into());

    m.insert("help.ntpkg.install".into(), "ntpkg install <lang> [ver] ## Download & install a runtime".into());

    m.insert("help.ntpkg.list".into(), "ntpkg list ## Show installed runtimes".into());

    m.insert("help.ntpkg.ntpx".into(), "ntpx <pkg> [bin] [args] ## Run an installed package's binary".into());

    m.insert("help.ntpkg.search".into(), "ntpkg search [keyword] ## Search available packages".into());

    m.insert("help.ntpkg.title".into(), "PACKAGE MANAGER (ntpkg) + ntpx:".into());

    m.insert("help.ntpkg.uninstall".into(), "ntpkg uninstall <lang> [ver] ## Remove an installed runtime".into());

    m.insert("help.ntpkg.use".into(), "ntpkg use <lang> <ver> ## Activate a specific version".into());

    m.insert("help.ntpkg.which".into(), "ntpkg which <lang> ## Show path to active runtime".into());

    m.insert("help.other.clear".into(), "clear ## Clear screen".into());

    m.insert("help.other.deinit".into(), "deinit [--ran|--local] ## Remove project files".into());

    m.insert("help.other.exit".into(), "exit, quit ## Exit ntc".into());

    m.insert("help.other.help".into(), "help ## Show this help".into());

    m.insert("help.other.init".into(), "init [--ran|--local] ## Initialize project files (NTCRANFILE.toml + ntconfig.toml)".into());

    m.insert("help.other.print".into(), "print <text> [--loop N] [-n|-t] ## Alias for say".into());

    m.insert("help.other.say".into(), "say <text> [--loop N] [-n|-t] ## Print text (no file redirect)".into());

    m.insert("help.other.title".into(), "OTHER COMMANDS:".into());

    m.insert("help.other.tutorial".into(), "tutorial ## Start the interactive step-by-step tutorial".into());
    m.insert("help.unknown_topic".into(), "Unknown help topic: '{}'. Run 'help' for all topics.".into());

    m.insert("help.other.ui".into(), "ui ## Show current UI mode".into());

    m.insert("help.other.version".into(), "version ## Show version".into());

    m.insert("help.pathman.syspath_add".into(), "syspath add <path> ## Add directory to system PATH".into());

    m.insert("help.pathman.syspath_add_cwd".into(), "syspath add --cwd | --here ## Add current directory to system PATH".into());

    m.insert("help.pathman.syspath_move".into(), "syspath move <n> <m> ## Move entry #n to position #m".into());

    m.insert("help.pathman.syspath_rm".into(), "syspath rm <path-or-number> ## Remove from system PATH (by path or index)".into());

    m.insert("help.pathman.syspath_show".into(), "syspath show ## Show system-level PATH entries".into());

    m.insert("help.pathman.title".into(), "PATH MANAGER (Windows):".into());

    m.insert("help.pathman.usrpath_add".into(), "usrpath add <path> ## Add directory to user PATH".into());

    m.insert("help.pathman.usrpath_add_cwd".into(), "usrpath add --cwd | --here ## Add current directory to user PATH".into());

    m.insert("help.pathman.usrpath_move".into(), "usrpath move <n> <m> ## Move entry #n to position #m".into());

    m.insert("help.pathman.usrpath_rm".into(), "usrpath rm <path-or-number> ## Remove from user PATH (by path or index)".into());

    m.insert("help.pathman.usrpath_show".into(), "usrpath show ## Show user-level PATH entries".into());

    m.insert("help.python.activate".into(), "py_activate ## Activate Python virtual environment (auto-detect)".into());

    m.insert("help.python.activate_path".into(), "py_activate <path> ## Activate specific venv path".into());

    m.insert("help.python.deactivate".into(), "py_deactivate ## Deactivate current virtual environment".into());

    m.insert("help.python.title".into(), "PYTHON COMMANDS:".into());

    m.insert("help.qr.decode".into(), "qr decode <image> ## Decode QR code from image".into());

    m.insert("help.qr.encode".into(), "qr <text> ## Encode text and display QR in terminal".into());

    m.insert("help.qr.save".into(), "qr encode <text> -o <file> ## Save QR as PNG image".into());

    m.insert("help.qr.image".into(), "qr encode <image> -o <file> ## Encode an image into a QR (scan to view image)".into());

    m.insert("help.qr.title".into(), "QR CODE COMMANDS:".into());

    m.insert("help.qsc.listen".into(), "qsc listen ## Start QSC daemon (background process)".into());

    m.insert("help.qsc.off".into(), "config qsc off ## Disable QSC hotkey".into());

    m.insert("help.qsc.set".into(), "config qsc \"#c 0\" ## Set hotkey (Ctrl+0)".into());

    m.insert("help.qsc.set_cs".into(), "config qsc \"#c #s a\" ## Set hotkey (Ctrl+Shift+A)".into());

    m.insert("help.qsc.startup".into(), "qsc startup on|off ## Register/unregister as startup app".into());

    m.insert("help.qsc.status".into(), "qsc status ## Check if QSC daemon is running".into());

    m.insert("help.qsc.stop".into(), "qsc stop ## Stop QSC daemon".into());

    m.insert("help.qsc.title".into(), "QSC (GLOBAL HOTKEY) COMMANDS:".into());

    m.insert("help.ran.deinit".into(), "ran deinit ## Remove NTCRANFILE.toml".into());

    m.insert("help.ran.help".into(), "ran help ## Show ran usage".into());

    m.insert("help.ran.init".into(), "ran init ## Create a NTCRANFILE.toml template".into());

    m.insert("help.ran.list".into(), "ran list ## List all targets in NTCRANFILE.toml".into());

    m.insert("help.ran.run".into(), "ran ## Run the 'standalone' target (default)".into());

    m.insert("help.ran.target".into(), "ran <target> ## Run a specific target from NTCRANFILE.toml".into());

    m.insert("help.ran.title".into(), "RAN / TASK RUNNER COMMANDS:".into());

    m.insert("help.report.cp".into(), "report <fmt> --cp ## Copy report to clipboard instead of saving".into());

    m.insert("help.report.docx".into(), "report docx [path] ## Generate DOCX report".into());

    m.insert("help.report.html".into(), "report html [path] ## Generate HTML report".into());

    m.insert("help.report.json".into(), "report json [path] ## Generate JSON report".into());

    m.insert("help.report.md".into(), "report md [path] ## Generate Markdown report".into());

    m.insert("help.report.pdf".into(), "report pdf [path] ## Generate PDF report".into());

    m.insert("help.report.title".into(), "REPORT COMMANDS:".into());

    m.insert("help.report.txt".into(), "report txt [path] ## Generate TXT report".into());

    m.insert("help.report.txtc".into(), "txtc <file> ## Copy file content to clipboard".into());

    m.insert("help.report.txtf".into(), "txtf <file> ## Display file content".into());

    m.insert("help.report.xlsx".into(), "report xlsx [path] ## Generate XLSX report (with dependency analysis)".into());

    m.insert("help.scr.continue".into(), "scr -c <file.ntc> [args...] ## Run script, continue after errors".into());

    m.insert("help.scr.example1".into(), "scr deploy.ntc ## Run deploy.ntc".into());

    m.insert("help.scr.example2".into(), "scr build.ntc dev prod ## Pass args $1=dev, $2=prod".into());

    m.insert("help.scr.example3".into(), "scr -c setup.ntc ## Run and continue after errors".into());

    m.insert("help.scr.help".into(), "scr help ## Show scr usage".into());

    m.insert("help.scr.init".into(), "scr init [name] ## Create a script template".into());

    m.insert("help.scr.ls".into(), "scr ls ## List *.ntc scripts in the current directory".into());

    m.insert("help.scr.run".into(), "scr <file.ntc> [args...] ## Run an automation script".into());

    m.insert("help.scr.title".into(), "SCR / AUTOMATION SCRIPT COMMANDS:".into());

    m.insert("help.search.ds".into(), "ds <pattern> [-d <n>] ## Search directories by name".into());

    m.insert("help.search.fs".into(), "fs <pattern> [-d <n>] ## Search files by name (fuzzy fallback)".into());

    m.insert("help.search.fsc".into(), "fsc <pattern> [-d <n>] ## Search files, pick one to display & optionally copy".into());

    m.insert("help.search.gs".into(), "gs <pattern> [-d <n>] [-l <n>] ## Search file contents (simple grep)".into());

    m.insert("help.search.gs_interactive".into(), "gs -i <pattern> [-d <n>] ## Interactive grep (10/page, Enter to open)".into());

    m.insert("help.search.locate".into(), "locate <pattern> [-d <n>] ## Search both files and directories".into());

    m.insert("help.search.title".into(), "SEARCH COMMANDS:".into());

    m.insert("help.size.care".into(), "size --care ## Calculate size including everything (care mode)".into());

    m.insert("help.size.path".into(), "size -p, --path <path|@tp> ## Show size of a specific path or teleport savepoint".into());

    m.insert("help.size.size".into(), "size ## Show current directory size".into());

    m.insert("help.size.time".into(), "size -t, --time ## Show elapsed time".into());

    m.insert("help.size.title".into(), "SIZE COMMAND:".into());

    m.insert("help.syscmd.any".into(), "<any command> ## NTC passes unknown commands to your system".into());

    m.insert("help.syscmd.native".into(), "shell -- echo, git, dir/ls, ping, etc.".into());

    m.insert("help.syscmd.native2".into(), "all run natively alongside NTC commands.".into());

    m.insert("help.syscmd.title".into(), "SYSTEM COMMANDS:".into());

    m.insert("help.teleport.add".into(), "tp add <name> ## Save current directory".into());

    m.insert("help.teleport.add_path".into(), "tp add <name> <path> ## Save specific path".into());

    m.insert("help.teleport.at".into(), "@<name> ## Quick teleport shortcut".into());

    m.insert("help.teleport.cls".into(), "tp cls ## Clear ALL savepoints".into());

    m.insert("help.teleport.info".into(), "tp info <name> ## Show savepoint details".into());

    m.insert("help.teleport.jump".into(), "tp jump <name> ## Teleport to saved location".into());

    m.insert("help.teleport.jump_num".into(), "tp jump <number> ## Teleport by number".into());

    m.insert("help.teleport.list".into(), "tp list ## List all teleport points".into());

    m.insert("help.teleport.menu".into(), "tp ## Interactive menu".into());

    m.insert("help.teleport.rm".into(), "tp rm <name> ## Remove teleport point".into());

    m.insert("help.teleport.rnm".into(), "tp rnm <old> to <new> ## Rename a savepoint".into());

    m.insert("help.teleport.title".into(), "TELEPORT COMMANDS:".into());

    m.insert("help.teleport.tpb".into(), "tpb ## Teleport back to previous location (undo tp)".into());

    m.insert("help.teleport.tpb_clear".into(), "tpb clear ## Clear teleport history".into());

    m.insert("help.teleport.tpb_history".into(), "tpb history ## Show teleport navigation history".into());

    m.insert("help.theme.add".into(), "theme add <name> ## Create a new theme (opens in editor)".into());

    m.insert("help.theme.current".into(), "theme current ## Show current theme name and details".into());

    m.insert("help.theme.edit".into(), "theme edit [name] ## Edit a theme (or open theme directory)".into());

    m.insert("help.theme.export".into(), "theme export <name> ## Export theme to .ntc_theme file".into());

    m.insert("help.theme.import".into(), "theme import <file> ## Import theme from .ntc_theme file".into());

    m.insert("help.theme.info".into(), "theme info <name> ## Show theme metadata and color counts".into());

    m.insert("help.theme.interactive".into(), "theme ## Interactive theme manager (menu)".into());

    m.insert("help.theme.list".into(), "theme list ## List all available themes".into());

    m.insert("help.theme.make".into(), "theme make [name] ## Create theme template in CWD".into());

    m.insert("help.theme.reload".into(), "theme reload ## Reload themes from disk".into());

    m.insert("help.theme.rm".into(), "theme rm <name> ## Remove a theme".into());

    m.insert("help.theme.rnm".into(), "theme rnm <old> to <new> ## Rename a theme".into());

    m.insert("help.theme.switch".into(), "theme <name> ## Switch to a theme by name".into());

    m.insert("help.theme.title".into(), "THEME COMMANDS:".into());

    m.insert("help.theme.where".into(), "theme where ## Show theme directory location".into());

    m.insert("help.tl.dir".into(), "tl dir <path> [-s <-n|-l|-r>] ## Show supported extensions and total lines in directory (sortable)".into());

    m.insert("help.tl.ext".into(), "tl ext <ext> [-s <-n|-l|-r>] ## Show all <ext> files and their total lines (respects igcare, sortable)".into());

    m.insert("help.tl.file".into(), "tl file <path> ## Count lines in a single file".into());

    m.insert("help.tl.sort".into(), "tl ext|dir <target> -s <-n|-l|-r> ## Sort: -n normal, -l by line (small to big), -r reverse (big to small)".into());

    m.insert("help.tl.title".into(), "TOTAL LINE COMMANDS:".into());

    m.insert("help.todo.add".into(), "todo add \"name\" ## Append a new task".into());

    m.insert("help.todo.add_at".into(), "todo add \"name\" -> N ## Insert task at position N".into());

    m.insert("help.todo.clear".into(), "todo clear ## Clear ALL tasks (with confirmation)".into());

    m.insert("help.todo.info".into(), "todo info [N] ## Show all tasks or details of task N".into());

    m.insert("help.todo.rm".into(), "todo rm <N> ## Remove task N".into());

    m.insert("help.todo.show".into(), "todo ## Show all tasks (from future_todo.txt in CWD)".into());

    m.insert("help.todo.title".into(), "TODO COMMANDS:".into());
    m.insert("help.todo.done".into(), "todo done <N> ## Mark task N as done".into());
    m.insert("help.todo.undo".into(), "todo undo <N> ## Reopen task N".into());
    m.insert("help.todo.edit".into(), "todo edit <N> \"name\" ## Rename task N".into());

    m.insert("help.arc.title".into(), "ARCHIVE COMMANDS:".into());
    m.insert("help.arc.create".into(), "arc create <out.zip|tar.gz|tar> <files...> ## Create archive".into());
    m.insert("help.arc.extract".into(), "arc extract <file> [dest] ## Extract archive".into());
    m.insert("help.arc.list".into(), "arc list <file> ## List archive contents".into());
    m.insert("help.arc.example1".into(), "arc create backup.zip src Cargo.toml ## Create zip".into());
    m.insert("help.arc.example2".into(), "arc list backup.zip ## List contents".into());
    m.insert("help.arc.example3".into(), "arc extract backup.zip restored ## Extract".into());

    m.insert("help.scan.depth".into(), "scan ext <ext...> -d, --depth <n> ## Limit scan depth".into());
    m.insert("help.scan.example1".into(), "scan ext rs ## List all .rs files".into());
    m.insert("help.scan.example2".into(), "scan ext rs c -d 2 ## List .rs/.c files up to depth 2".into());
    m.insert("help.scan.example3".into(), "scan ext rs c -s ## List with sizes + total".into());

    m.insert("help.scan.example4".into(), "scan port 192.168.1.1 --top 10 ## Scan top 10 ports".into());

    m.insert("help.scan.example5".into(), "scan ip 192.168.1.0/24 ## Sweep for live hosts".into());
    m.insert("help.scan.ext".into(), "scan ext <ext...> ## List files by extension (no line counts)".into());

    m.insert("help.scan.port".into(), "scan port [host] [--top N | --ports <spec> | --all] ## TCP port scan".into());

    m.insert("help.scan.ip".into(), "scan ip [cidr] [--top N | --ports <spec>] ## Live-host sweep + ARP".into());
    m.insert("help.scan.size".into(), "scan ext <ext...> -s, --size ## Show sizes + total".into());
    m.insert("help.scan.care".into(), "scan ext <ext...> --care ## Include everything (care mode)".into());
    m.insert("help.scan.title".into(), "SCAN COMMANDS:".into());

    m.insert("help.nssh.title".into(), "NSSH COMMANDS:".into());
    m.insert("help.nssh.add".into(), "nssh add <name> --host <h> ## Save an SSH profile".into());
    m.insert("help.nssh.ssh".into(), "nssh ssh [name] [--cmd \"...\"] ## Shell or one remote command".into());
    m.insert("help.nssh.cp".into(), "nssh cp <local> <name>:<remote> ## SFTP upload/download".into());
    m.insert("help.nssh.scan".into(), "nssh scan <host> [--top N] ## TCP port scan".into());
    m.insert("help.nssh.manage".into(), "nssh ls|info|rm|edit ## Manage profiles".into());
    m.insert("help.nssh.keygen".into(), "nssh keygen ## Generate keys via ssh-keygen".into());
    m.insert("help.nssh.example1".into(), "nssh add prod --host 203.0.113.10 --user root ## Save profile".into());
    m.insert("help.nssh.example2".into(), "nssh ssh prod --cmd \"uptime\" ## Run remote command".into());
    m.insert("help.nssh.example3".into(), "nssh cp ./app.tar.gz prod:/tmp/ ## Upload file".into());

    m.insert("help.nsheet.title".into(), "NSHEET COMMANDS:".into());
    m.insert("help.nsheet.open".into(), "nsheet open <file> [options] ## Fullscreen sheet viewer (CSV/TSV/JSON)".into());
    m.insert("help.nsheet.view".into(), "nsheet view <file> [options] ## Print sheet as a table (default 50 rows)".into());
    m.insert("help.nsheet.sort".into(), "nsheet view <file> --sort <col> [--desc] ## Sort by column".into());
    m.insert("help.nsheet.filter".into(), "nsheet view <file> --filter <expr> ## Filter rows (col~text, col=n, col>n)".into());
    m.insert("help.nsheet.export".into(), "nsheet view <file> --export <json|md> [-o <file>] ## Export rows".into());
    m.insert("help.nsheet.tui".into(), "TUI keys: arrows/hjkl move · / search · n next · s sort · f filter · e export · Enter view · q quit".into());
    m.insert("help.nsheet.example1".into(), "nsheet open users.csv ## Browse the sheet".into());
    m.insert("help.nsheet.example2".into(), "nsheet view data.json --filter \"age>30\" --sort age ## Filter + sort".into());
    m.insert("help.nsheet.example3".into(), "nsheet view a.csv --export md -o out.md ## Export Markdown".into());

    m.insert("help.view.care".into(), "view --care ## Show tree with care everything".into());

    m.insert("help.view.cp".into(), "view -c, --cp ## Show tree and copy to clipboard".into());

    m.insert("help.view.depth".into(), "view -d, --depth <n> ## Show tree with custom depth".into());

    m.insert("help.view.dirsize".into(), "view -ds, --dirsize ## Show tree with directory sizes only".into());

    m.insert("help.view.ext".into(), "view -ext, --ext <ext...> ## Show tree with only matching extensions".into());

    m.insert("help.view.filesize".into(), "view -fs, --filesize ## Show tree with file sizes only".into());

    m.insert("help.view.path".into(), "view -p, --path <path|@tp> ## Show tree of a specific path or teleport savepoint".into());

    m.insert("help.view.size".into(), "view -s, --size ## Show tree with dir and file sizes".into());

    m.insert("help.view.time".into(), "view --time, -t ## Show tree with execution time".into());

    m.insert("help.view.title".into(), "VIEW COMMAND:".into());

    m.insert("help.view.tree".into(), "view ## Show directory tree (uses config depth)".into());

    m.insert("help.config.example1".into(), "config output ~/Desktop ## Set output path to Desktop".into());

    m.insert("help.config.example2".into(), "config depth 3 ## Set max depth to 3".into());

    m.insert("help.config.example3".into(), "config color ON ## Enable color output".into());

    m.insert("help.config.example4".into(), "config threads 8 ## Set thread count to 8".into());

    m.insert("help.config.example5".into(), "showcg ## Show current configuration overview".into());

    m.insert("help.editor.example1".into(), "ne main.rs ## Open main.rs in editor".into());

    m.insert("help.editor.example2".into(), "ne src/main.rs:42 ## Open main.rs at line 42".into());

    m.insert("help.editor.example3".into(), "ne --init newfile.md ## Create a template file".into());

    m.insert("help.editor.example4".into(), "ne --hyper myproject --lang c ## Create C hyper project".into());

    m.insert("help.theme.example1".into(), "theme list ## List all available themes".into());

    m.insert("help.theme.example2".into(), "theme dark ## Switch to 'dark' theme".into());

    m.insert("help.theme.example3".into(), "theme current ## Show current theme info".into());

    m.insert("help.theme.example4".into(), "theme add mytheme ## Create a new theme".into());

    m.insert("help.teleport.example1".into(), "tp add web ## Save current directory as 'web'".into());

    m.insert("help.teleport.example2".into(), "tp add docs ~/Documents ## Save specific path".into());

    m.insert("help.teleport.example3".into(), "tp jump web ## Teleport to 'web'".into());

    m.insert("help.teleport.example4".into(), "tp list ## List all savepoints".into());

    m.insert("help.teleport.example5".into(), "@web ## Quick teleport shortcut".into());

    m.insert("help.ran.example1".into(), "ran ## Run default standalone target".into());

    m.insert("help.ran.example2".into(), "ran build ## Run 'build' target".into());

    m.insert("help.ran.example3".into(), "ran list ## List all targets".into());

    m.insert("help.alias.example1".into(), "ral add build \"cargo build\" ## Create simple alias".into());

    m.insert("help.alias.example2".into(), "ral add greet(x) \"echo Hello $x\" ## Parameterised alias".into());

    m.insert("help.alias.example3".into(), "ral add copy(x) \"cp $x ${{x}}.bak\" ## Alias with defaults".into());

    m.insert("help.alias.example4".into(), "ral list ## Show all aliases".into());

    m.insert("help.alias.example5".into(), "ral export --all myaliases ## Export aliases".into());

    m.insert("help.alias.example6".into(), "build ## Run alias directly".into());

    m.insert("help.view.example1".into(), "view ## Show directory tree".into());

    m.insert("help.view.example2".into(), "view --size ## Show tree with sizes".into());

    m.insert("help.view.example3".into(), "view -d 2 ## Show tree with depth 2".into());

    m.insert("help.view.example4".into(), "view --cp ## Show tree and copy to clipboard".into());

    m.insert("help.view.example5".into(), "view --time ## Show tree with execution time".into());

    m.insert("help.view.example6".into(), "view -p C:\\code\\project ## Show tree of a specific path".into());

    m.insert("help.view.example7".into(), "view @web ## Show tree of a teleport savepoint".into());

    m.insert("help.size.example1".into(), "size ## Show current directory size".into());

    m.insert("help.size.example2".into(), "size ../src ## Show size of another directory".into());

    m.insert("help.size.example3".into(), "size -p C:\\code\\project ## Show size of a specific path".into());

    m.insert("help.size.example4".into(), "size @web --care -t ## Care mode with elapsed time for a savepoint".into());

    m.insert("help.report.example1".into(), "report txt ## Generate TXT report".into());

    m.insert("help.report.example2".into(), "report html --cp ## Copy HTML report to clipboard".into());

    m.insert("help.report.example3".into(), "report pdf ## Generate PDF report".into());

    m.insert("help.report.example4".into(), "txtc file.txt ## Copy file content to clipboard".into());

    m.insert("help.backup.example1".into(), "backup create ## Create a new backup".into());

    m.insert("help.backup.example2".into(), "backup list ## List all backups".into());

    m.insert("help.backup.example3".into(), "backup restore 1 ## Restore backup #1".into());

    m.insert("help.backup.example4".into(), "backup diff 1 ## Diff against backup #1".into());

    m.insert("help.search.example1".into(), "gs fn main ## Search for 'fn main' in files".into());

    m.insert("help.search.example2".into(), "gs -i config ## Interactive search results".into());

    m.insert("help.search.example3".into(), "fs *.rs ## Find .rs files".into());

    m.insert("help.search.example4".into(), "ds src ## Find directories named 'src'".into());

    m.insert("help.search.example5".into(), "locate README ## Search files & dirs for 'README'".into());

    m.insert("help.igcare.example1".into(), "ignore dir node_modules ## Ignore node_modules".into());

    m.insert("help.igcare.example2".into(), "ignore ext .log ## Ignore .log files".into());

    m.insert("help.igcare.example3".into(), "care dir src ## Un-ignore src directory".into());

    m.insert("help.igcare.example4".into(), "igcared ## Show all ignore/care settings".into());

    m.insert("help.network.example1".into(), "sys ## Show system monitor".into());

    m.insert("help.network.example2".into(), "http GET https://api.example.com ## Simple GET request".into());

    m.insert("help.network.example3".into(), "dl https://example.com/file.zip --progress ## Download with progress".into());

    m.insert("help.network.example4".into(), "ping google.com ## Ping google.com".into());

    m.insert("help.network.example5".into(), "dns example.com -t MX ## DNS MX record lookup".into());

    m.insert("help.network.example6".into(), "serve 8080 ## Start HTTP server on port 8080".into());

    m.insert("help.clipboard.example1".into(), "clip ## Show clipboard history".into());

    m.insert("help.clipboard.example2".into(), "clip select 1 ## Copy entry #1 back to clipboard".into());

    m.insert("help.clipboard.example3".into(), "pathcp ## Copy current directory path".into());

    m.insert("help.qsc.example1".into(), "config qsc \"#c 0\" ## Set Ctrl+0 hotkey".into());

    m.insert("help.qsc.example2".into(), "qsc listen ## Start QSC daemon".into());

    m.insert("help.qsc.example3".into(), "qsc status ## Check daemon status".into());

    m.insert("help.todo.example1".into(), "todo ## Show all tasks".into());

    m.insert("help.todo.example2".into(), "todo add \"Fix bug\" ## Add a new task".into());

    m.insert("help.todo.example3".into(), "todo rm 1 ## Remove task #1".into());

    m.insert("help.other.example1".into(), "init --ran ## Initialize NTCRANFILE.toml".into());

    m.insert("help.other.example2".into(), "clear ## Clear the screen".into());

    m.insert("help.other.example3".into(), "version ## Show version info".into());

    m.insert("help.other.example4".into(), "tutorial ## Start interactive tutorial".into());

    m.insert("help.fileops.example1".into(), "mkf index.html ## Create a new file".into());

    m.insert("help.fileops.example2".into(), "mkd src/components ## Create directories".into());

    m.insert("help.fileops.example3".into(), "cp file.txt backup.txt ## Copy a file".into());

    m.insert("help.tl.example1".into(), "tl file main.rs ## Count lines in main.rs".into());

    m.insert("help.tl.example2".into(), "tl ext .rs ## Count lines in all .rs files".into());

    m.insert("help.tl.example3".into(), "tl dir . ## Count lines in current directory".into());

    m.insert("help.tl.example4".into(), "tl ext .rs -s -r ## List .rs files sorted by lines, big to small".into());

    m.insert("help.ntpkg.example1".into(), "ntpkg install node 18 ## Install Node.js 18".into());

    m.insert("help.ntpkg.example2".into(), "ntpkg list ## Show installed runtimes".into());

    m.insert("help.ntpkg.example3".into(), "ntpkg use node 18 ## Activate Node.js 18".into());

    m.insert("help.ntpkg.example4".into(), "ntpkg which node ## Show path to Node.js".into());

    m.insert("help.ntpkg.example5".into(), "ntpkg search python ## Search for Python packages".into());

    m.insert("help.ntpkg.example6".into(), "ntpkg info node ## Show Node.js package info".into());

    m.insert("help.ntpkg.example7".into(), "ntpkg available ## List downloadable languages".into());

    m.insert("help.ntpkg.example8".into(), "ntpx node npm init ## Run npm via ntpx".into());

    m.insert("help.ntpkg.example9".into(), "ntpkg help ## Show ntpkg usage".into());

    m.insert("help.qr.example1".into(), "qr \"Hello World\" ## Display QR code in terminal".into());

    m.insert("help.qr.example2".into(), "qr encode \"Hello\" -o hello.png ## Save QR as PNG".into());

    m.insert("help.qr.example3".into(), "qr decode qr.png ## Decode QR from image".into());

    m.insert("help.qr.example4".into(), "qr encode image.png -o imageqrcode.png ## Encode image so scanning shows it".into());

    m.insert("help.pathman.example1".into(), "usrpath show ## Show user PATH entries".into());

    m.insert("help.pathman.example2".into(), "usrpath add C:\\tools ## Add to user PATH".into());

    m.insert("help.pathman.example3".into(), "usrpath rm 2 ## Remove entry #2 from user PATH".into());

    m.insert("help.pathman.example4".into(), "syspath show ## Show system PATH entries".into());

    m.insert("help.pathman.example5".into(), "syspath add C:\\bin ## Add to system PATH".into());

    m.insert("help.pathman.example6".into(), "syspath move 3 1 ## Move entry to position 1".into());

    m.insert("help.autoup.example1".into(), "autoup list ## List startup entries".into());

    m.insert("help.autoup.example2".into(), "autoup add ntc \"C:\\ntc\\ntc.exe\" ## Add to startup".into());

    m.insert("help.autoup.example3".into(), "autoup rm ntc ## Remove startup entry".into());

    m.insert("help.autoup.example4".into(), "autoup disable ntc ## Disable startup entry".into());

    m.insert("help.mode.example1".into(), "mode admin ## Relaunch as Administrator".into());

    m.insert("help.explorer.example1".into(), "explorer ## Launch file explorer in current dir".into());

    m.insert("help.explorer.example2".into(), "nfe ~/projects ## Open explorer in ~/projects".into());

    m.insert("help.math.example1".into(), "math 3+4*5 ## Evaluate arithmetic expression".into());

    m.insert("help.math.example2".into(), "math sin(PI/2) ## Math with functions".into());

    m.insert("help.math.example3".into(), "math timer 30 ## Start 30-second countdown".into());

    m.insert("help.math.example4".into(), "math fun add myfn \"x+1\" ## Add custom function".into());

    m.insert("help.math.example5".into(), "math fun ls ## List custom functions".into());

    m.insert("help.math.example6".into(), "math calc.ntc.math ## Run math file".into());

    m.insert("help.python.example1".into(), "py_activate ## Auto-activate Python venv".into());

    m.insert("help.python.example2".into(), "py_activate .venv ## Activate specific venv".into());

    m.insert("help.python.example3".into(), "py_deactivate ## Deactivate venv".into());

    m.insert("help.encdec.example1".into(), "encode b64 \"hello\" ## Base64 encode a string".into());

    m.insert("help.encdec.example2".into(), "encode hex file.bin ## Hex encode a file".into());

    m.insert("help.encdec.example3".into(), "decode b64 aGVsbG8= ## Base64 decode".into());

    m.insert("help.encdec.example4".into(), "cvcode b64 \"aGVsbG8=\" to hex ## Convert between formats".into());

    m.insert("help.fmt.example1".into(), "fmt json config.json to yaml ## Convert JSON to YAML".into());

    m.insert("help.fmt.example2".into(), "fmt toml Cargo.toml to json -o output.json ## Convert TOML to JSON".into());

    m.insert("help.image.example1".into(), "image screenshot.png to json ## Convert image to JSON for AI".into());

    m.insert("help.image.example2".into(), "image photo.jpg --ocr ## Extract text from image via OCR".into());

    m.insert("helpers.building_tree".into(), "Building directory tree...".into());

    m.insert("helpers.calculating_sizes".into(), "Calculating directory sizes...".into());

    m.insert("helpers.export_cancelled".into(), "Export cancelled.".into());

    m.insert("helpers.exported_all".into(), "Exported all {} alias(es) to: {}".into());

    m.insert("helpers.exported_categories".into(), "Exported {} category(ies) to: {}".into());

    m.insert("helpers.exported_count".into(), "Exported {} alias(es) to: {}".into());

    m.insert("helpers.exported_igcare_all".into(), "Exported all ignore/care settings to: {}".into());

    m.insert("helpers.file_not_found".into(), "File not found: {}".into());

    m.insert("helpers.imported_igcare".into(), "Imported ignore/care settings from: {}".into());

    m.insert("helpers.imported_ral".into(), "Imported run aliases from: {}".into());

    m.insert("helpers.invalid_back_count".into(), "Invalid back count: {}".into());

    m.insert("helpers.invalid_input".into(), "Invalid input: {}".into());

    m.insert("helpers.invalid_num".into(), "Invalid number: {}".into());

    m.insert("helpers.invalid_num_range".into(), "Invalid number: {}. Use 1-{} or 0 to finish.".into());

    m.insert("helpers.no_editor".into(), "No external editor found. Opening built-in editor...".into());

    m.insert("helpers.no_ral_to_export".into(), "No run aliases to export.".into());

    m.insert("igcare_export.all".into(), "select all".into());

    m.insert("igcare_export.cancel".into(), "cancel".into());

    m.insert("igcare_export.extra_supported_extensions".into(), "Extra supported extensions".into());

    m.insert("igcare_export.extra_supported_files".into(), "Extra supported files".into());

    m.insert("igcare_export.finish".into(), "finish export".into());

    m.insert("igcare_export.ignored_dirs".into(), "Ignored directories".into());

    m.insert("igcare_export.ignored_extensions".into(), "Ignored extensions".into());

    m.insert("igcare_export.ignored_files".into(), "Ignored files".into());

    m.insert("igcare_export.items".into(), "{} item(s)".into());

    m.insert("igcare_export.prompt".into(), "igcare-export>".into());

    m.insert("igcare_export.select".into(), "select/deselect by number".into());

    m.insert("igcare_export.selected".into(), "selected".into());

    m.insert("igcare_export.selected_mark".into(), "(selected)".into());

    m.insert("igcare_export.title".into(), "igcare export — Select Categories".into());

    m.insert("image2doc.ocr_disabled".into(), "OCR not enabled on this platform or build. Recompile with 'ocr' feature or install tesseract.".into());

    m.insert("image2doc.ocr_missing".into(), "Tesseract not found. OCR is unavailable.".into());

    m.insert("image2doc.ocr_install".into(), "Install it with: ntpkg install tesseract".into());

    m.insert("image2doc.ocr_path_win".into(), "Then add it to PATH: usrpath add <dir>, then 'mode admin' + 'syspath add <dir>', or open a new terminal.".into());

    m.insert("image2doc.ocr_path_unix".into(), "Then add it to PATH, e.g.: export PATH=\"/<dir>:$PATH\"".into());

    m.insert("image2doc.ocr_rerun".into(), "Then run the same command again.".into());

    m.insert("image2doc.ocr_failed".into(), "Tesseract failed to process the image.".into());

    m.insert("interactive.cancelled".into(), "Cancelled.".into());

    m.insert("interactive.invalid_input".into(), "Invalid input.".into());

    m.insert("interactive.invalid_selection".into(), "Invalid selection.".into());

    m.insert("interactive.no_items".into(), "(no items)".into());

    m.insert("math.builtin_agg".into(), "  agg:  sum, min, max, avg".into());

    m.insert("math.builtin_constants".into(), "  constants: PI, E, PHI, TAU".into());

    m.insert("math.builtin_conv".into(), "  conv: toBinary, toHex, to8/toOctal, toDecimal, toHB".into());

    m.insert("math.builtin_header".into(), "Built-in functions:".into());

    m.insert("math.builtin_inv_trig".into(), "  inv:  arcsin/asin, arccos/acos, arctan/atan, arccot/acot, arcsec/asec, arccsc/acsc".into());

    m.insert("math.builtin_math".into(), "  math: sqrt, pow(x,y), abs, floor, ceil, round, ln/log, log2, log10".into());

    m.insert("math.builtin_print".into(), "  print: print values (like Python)".into());

    m.insert("math.builtin_rand".into(), "  rand: rand(min,max)".into());

    m.insert("math.builtin_trig".into(), "  trig: sin, cos, tan, cot, sec, csc".into());

    m.insert("math.compiled_header".into(), "Compiled: {}".into());

    m.insert("math.compiled_header".into(), "Compiled: {}".into());

    m.insert("math.countdown".into(), "Countdown: {} seconds".into());

    m.insert("math.error".into(), "  {} at line {}: {}".into());

    m.insert("math.example_add".into(), "  math 3+4*5                           → 23".into());

    m.insert("math.example_fun_call".into(), "  math square(5)                        → 25".into());

    m.insert("math.example_fun_def".into(), "  math fun add square(x) = x^2".into());

    m.insert("math.example_hexbin".into(), "  math 0xFF + 0b1010                   → 265".into());

    m.insert("math.example_rand".into(), "  math rand(1,100)                      → 42".into());

    m.insert("math.example_sin".into(), "  math sin(PI/2)                       → 1".into());

    m.insert("math.example_sqrt".into(), "  math sqrt(144)                        → 12".into());

    m.insert("math.example_timer".into(), "  math timer 10                        → countdown 10s".into());

    m.insert("math.example_tobinary".into(), "  math toBinary(255)                    → 11111111".into());

    m.insert("math.example_tohb".into(), "  math toHB(1048576)                    → 1.00 MB".into());

    m.insert("math.example_tohex".into(), "  math toHex(255)                       → FF".into());

    m.insert("math.examples_header".into(), "Examples:".into());

    m.insert("math.exec_end".into(), "\u{255A}\u{2550} End: {} \u{2550}\u{255D}".into());

    m.insert("math.failed_read".into(), "Failed to read '{}': {}".into());

    m.insert("math.file_not_found".into(), "File not found: '{}'".into());

    m.insert("math.fun_commands_header".into(), "math fun commands:".into());

    m.insert("math.fun_list_header".into(), "User-defined math functions:".into());

    m.insert("math.fun_none".into(), " No user-defined functions. Use 'math fun add <name>(<params>) = <body>'".into());

    m.insert("math.fun_removed".into(), " Removed: {}".into());

    m.insert("math.help_expr".into(), "  math <expression>       Evaluate a math expression".into());

    m.insert("math.help_file".into(), "  math <file>.ntc.math    Compile and run a math file".into());

    m.insert("math.help_fun_add".into(), "  math fun add <name>(<params>) = <body>   Define a function".into());

    m.insert("math.help_fun_edit".into(), "  math fun edit <name> = <body>  Update a function".into());

    m.insert("math.help_fun_info".into(), "  math fun info <name>    Show function details".into());

    m.insert("math.help_fun_ls".into(), "  math fun ls             List all user-defined functions".into());

    m.insert("math.help_fun_rm".into(), "  math fun rm <name>      Remove a function".into());

    m.insert("math.help_func_call".into(), "  math <func>(<args>)     Call a built-in or user-defined function".into());

    m.insert("math.help_timer".into(), "  math timer [seconds]    Lap timer or countdown with alarm".into());

    m.insert("math.help_title".into(), "Math Module - Usage:".into());

    m.insert("math.is_directory".into(), "'{}' is a directory, not a file".into());

    m.insert("math.lap_time".into(), "Lap: {} seconds".into());

    m.insert("math.time_remaining".into(), "Time remaining: {}".into());

    m.insert("math.timer_started".into(), "Timer started...".into());

    m.insert("math.timer_usage".into(), "Usage: math timer <seconds>".into());
    m.insert("math.times_up".into(), "Time's up!".into());

    m.insert("misc.clear.type_help".into(), "Type 'help' for available commands, 'exit' to quit.".into());

    m.insert("misc.clear.welcome".into(), "Welcome to ntc {} - Navigate, Toolkit, Center".into());

    m.insert("misc.ds.example1".into(), " ds src # search using config depth".into());

    m.insert("misc.ds.example2".into(), " ds src -d 3 # search up to 3 levels deep".into());

    m.insert("misc.ds.example3".into(), " ds test # partial match: finds test_utils/ etc.".into());

    m.insert("misc.ds.usage".into(), "Usage: ds <pattern> [-d <depth>]".into());

    m.insert("misc.fs.example1".into(), " fs main.c # search using config depth".into());

    m.insert("misc.fs.example2".into(), " fs main.c -d 5 # search up to 5 levels deep".into());

    m.insert("misc.fs.example3".into(), " fs main # partial match: finds main_helper.c etc.".into());

    m.insert("misc.fs.usage".into(), "Usage: fs <pattern> [-d <depth>]".into());

    m.insert("misc.fsc.copied".into(), "Copied '{}' to clipboard!".into());

    m.insert("misc.fsc.copy_prompt".into(), "Copy to clipboard? (y/N): ".into());

    m.insert("misc.fsc.done_opt".into(), "0. Done".into());

    m.insert("misc.fsc.done_str".into(), "Done.".into());

    m.insert("misc.fsc.example1".into(), " fsc main.rs # search then pick file to display".into());

    m.insert("misc.fsc.example2".into(), " fsc main.rs -d 5 # search up to 5 levels deep".into());

    m.insert("misc.fsc.found".into(), "\u{1F50D} Found {} file(s) for \"{}\":".into());

    m.insert("misc.fsc.invalid_input".into(), "Invalid input.".into());

    m.insert("misc.fsc.invalid_selection".into(), "Invalid selection.".into());

    m.insert("misc.fsc.not_file".into(), "Selected path is not a file.".into());

    m.insert("misc.fsc.select_prompt".into(), "Select file to display: ".into());

    m.insert("misc.fsc.skipped".into(), "Skipped (not support format): {}".into());

    m.insert("misc.fsc.usage".into(), "Usage: fsc <pattern> [-d <depth>]".into());

    m.insert("misc.gs.example1".into(), " gs main # search file contents using config depth".into());

    m.insert("misc.gs.example2".into(), " gs main -d 5 # search up to 5 levels deep".into());

    m.insert("misc.gs.example3".into(), " gs -i main # interactive mode (browse results, Enter to open)".into());

    m.insert("misc.gs.example4".into(), " gs -i main -d 3 # interactive mode with depth limit".into());

    m.insert("misc.gs.example5".into(), " gs main -d 5 -l 10 # show only first 10 matches".into());

    m.insert("misc.gs.searching".into(), "Searching contents...".into());

    m.insert("misc.gs.many_results".into(), "gs: {} matches found, showing first 20,000. Narrow your search.".into());

    m.insert("misc.gs.usage".into(), "Usage: gs <pattern> [-d <depth>] [-l <limit>] [-i]".into());

    m.insert("misc.wi.not_dir".into(), "wi: not a directory: '{}'".into());

    m.insert("misc.wi.start_error".into(), "wi: cannot watch: {}".into());

    m.insert("misc.wi.usage".into(), "Usage: wi [path]".into());

    m.insert("misc.locate.example1".into(), " locate main.c # search files and dirs (config depth)".into());

    m.insert("misc.locate.example2".into(), " locate main.c -d 5 # search up to 5 levels deep".into());

    m.insert("misc.locate.example3".into(), " locate main # partial match: finds main.c, main_helper/ etc.".into());

    m.insert("misc.locate.header".into(), "\u{1F50D} {} for \"{}\" ({} result(s)):".into());

    m.insert("misc.locate.label_didyoumean".into(), "Did you mean?".into());

    m.insert("misc.locate.label_exact".into(), "Exact matches".into());

    m.insert("misc.locate.label_partial".into(), "Partial matches".into());

    m.insert("misc.locate.no_results".into(), "\n\u{1F50D} No results for \"{}\"\n".into());

    m.insert("misc.locate.usage".into(), "Usage: locate <pattern> [-d <depth>]".into());

    m.insert("misc.mkd.created".into(), "Created directory: {}".into());

    m.insert("misc.mkd.error".into(), "mkd: {}: {}".into());

    m.insert("misc.mkd.missing".into(), "mkd: missing operand".into());

    m.insert("misc.mkf.created".into(), "Created file: {}".into());

    m.insert("misc.mkf.error".into(), "mkf: {}: {}".into());

    m.insert("misc.mkf.exists".into(), "Already exists: {}".into());

    m.insert("misc.mkf.missing".into(), "mkf: missing operand".into());

    m.insert("misc.ne.created_hyper".into(), "Created hyper project: {}".into());

    m.insert("misc.ne.created_super".into(), "Created super template: {}".into());

    m.insert("misc.ne.created_template".into(), "Created template: {}".into());

    m.insert("misc.ne.editor_error".into(), "Editor error: {}".into());

    m.insert("misc.ne.hyper_error".into(), "Hyper project error: {}".into());

    m.insert("misc.ne.opened_welcome".into(), "Opened welcome screen in {}".into());

    m.insert("misc.ne.usage_hyper".into(), "Usage: ne --hyper <name> --lang <lang> (name must not contain extension)".into());

    m.insert("misc.pathcp.copied".into(), "Path copied to clipboard: {}".into());

    m.insert("misc.pip.found".into(), "\u{2713} pip found at: {}".into());

    m.insert("misc.pip.not_found".into(), "pip not found in venv/Scripts directory".into());

    m.insert("misc.rmd.error".into(), "rmd: {}: {}".into());

    m.insert("misc.rmd.missing".into(), "rmd: missing operand".into());

    m.insert("misc.rmd.not_dir".into(), "rmd: '{}' is not a directory".into());

    m.insert("misc.rmd.not_exist".into(), "rmd: '{}' does not exist".into());

    m.insert("misc.rmd.removed".into(), "Removed directory: {}".into());

    m.insert("misc.rmf.error".into(), "rmf: {}: {}".into());

    m.insert("misc.rmf.is_dir".into(), "rmf: '{}' is a directory (use rmd instead)".into());

    m.insert("misc.rmf.missing".into(), "rmf: missing operand".into());

    m.insert("misc.rmf.not_exist".into(), "rmf: '{}' does not exist".into());

    m.insert("misc.rmf.no_match".into(), "rmf: no files match '{}'".into());

    m.insert("misc.rmf.pattern_matches".into(), "{} file(s) match '{}':".into());

    m.insert("misc.rmf.confirm_delete".into(), "Delete {} file(s)?".into());

    m.insert("misc.rmf.cancelled".into(), "Cancelled.".into());

    m.insert("misc.rmf.removed".into(), "Removed: {}".into());

    m.insert("misc.rnmd.error".into(), "rnmd: {}".into());

    m.insert("misc.rnmd.exists".into(), "rnmd: '{}' already exists".into());

    m.insert("misc.rnmd.missing".into(), "rnmd: missing operand".into());

    m.insert("misc.rnmd.not_dir".into(), "rnmd: '{}' is not a directory".into());

    m.insert("misc.rnmd.not_exist".into(), "rnmd: '{}' does not exist".into());

    m.insert("misc.rnmd.renamed".into(), "Renamed directory '{}' -> '{}'".into());

    m.insert("misc.rnmd.usage".into(), " Usage: rnmd <old_name> <new_name>".into());

    m.insert("misc.rnmf.error".into(), "rnmf: {}".into());

    m.insert("misc.rnmf.exists".into(), "rnmf: '{}' already exists".into());

    m.insert("misc.rnmf.is_dir".into(), "rnmf: '{}' is a directory (use rnmd instead)".into());

    m.insert("misc.rnmf.missing".into(), "rnmf: missing operand".into());

    m.insert("misc.rnmf.not_exist".into(), "rnmf: '{}' does not exist".into());

    m.insert("misc.rnmf.renamed".into(), "Renamed '{}' -> '{}'".into());

    m.insert("misc.rnmf.usage".into(), " Usage: rnmf <old_name> <new_name>".into());

    m.insert("misc.mv.error".into(), "mv: {}".into());

    m.insert("misc.mv.exists".into(), "mv: target exists: '{}' (use -f to overwrite)".into());

    m.insert("misc.mv.missing".into(), "mv: missing operand".into());

    m.insert("misc.mv.moved".into(), "Moved '{}' -> '{}'".into());

    m.insert("misc.mv.not_exist".into(), "mv: source not found: '{}'".into());

    m.insert("misc.mv.usage".into(), "Usage: mv <src> <dst|@tp> [-f]".into());

    m.insert("misc.cp.error".into(), "cp: {}".into());

    m.insert("misc.cp.exists".into(), "cp: target exists: '{}' (use -f to overwrite)".into());

    m.insert("misc.cp.missing".into(), "cp: missing operand".into());

    m.insert("misc.cp.copied".into(), "Copied '{}' -> '{}'".into());

    m.insert("misc.cp.not_exist".into(), "cp: source not found: '{}'".into());

    m.insert("misc.cp.same".into(), "cp: source and target are the same file: '{}'".into());

    m.insert("misc.cp.usage".into(), "Usage: cp <src> <dst|@tp> [-f]".into());

    m.insert("misc.say.example1".into(), " say \"hello world\" --loop 5 -n # 5 lines".into());

    m.insert("misc.say.example2".into(), " say \"hello world\" --loop 3 -t # 1 line, 3 times".into());

    m.insert("misc.say.loop_exceed".into(), "--loop value exceeds max ({}), clamping".into());

    m.insert("misc.say.loop_invalid".into(), "Invalid --loop value '{}', using 1".into());

    m.insert("misc.say.loop_requires".into(), "--loop requires a number, using 1".into());

    m.insert("misc.say.loop_warning".into(), "--loop value cannot be less than 1, using 1".into());

    m.insert("misc.say.usage".into(), "Usage: say <text> [--loop N] [-n|-t]".into());

    m.insert("misc.show_env.activate_hint".into(), "Run 'python_activate' to activate a virtual environment".into());

    m.insert("misc.show_env.active".into(), "Virtual environment is".into());

    m.insert("misc.show_env.and_more".into(), "... and {} more".into());

    m.insert("misc.show_env.cwd_label".into(), "Current directory:".into());

    m.insert("misc.show_env.deactivate_hint".into(), "Run 'deactivate' to exit the virtual environment".into());

    m.insert("misc.show_env.not_active".into(), "Virtual environment is".into());

    m.insert("misc.show_env.not_set".into(), "(not set)".into());

    m.insert("misc.show_env.os_label".into(), "OS:".into());

    m.insert("misc.show_env.other_header".into(), "\u{1F527} Other Python/Venv Variables:".into());

    m.insert("misc.show_env.path_full".into(), "\u{1F4C2} PATH (full):".into());

    m.insert("misc.show_env.path_header".into(), "\u{1F4C2} PATH (first few entries):".into());

    m.insert("misc.show_env.shell_label".into(), "Shell:".into());

    m.insert("misc.show_env.summary_header".into(), "\u{1F4CA} Summary:".into());

    m.insert("misc.show_env.sys_header".into(), "\u{1F4BB} System Information:".into());

    m.insert("misc.show_env.venv_header".into(), "\u{1F40D} Python Virtual Environment:".into());

    m.insert("misc.size.bytes_label".into(), "Bytes:".into());

    m.insert("misc.size.calculating".into(), "Calculating directory size...".into());

    m.insert("misc.size.human_label".into(), "Human:".into());

    m.insert("misc.size.path_label".into(), "Path:".into());

    m.insert("misc.size.time_label".into(), "Time:".into());

    m.insert("misc.size.title".into(), "Current Directory Size".into());

    m.insert("misc.tp.example_info".into(), "Example: tp info my_project".into());

    m.insert("misc.tp.info_canonical".into(), "Canonical".into());

    m.insert("misc.tp.info_exists".into(), "Exists".into());

    m.insert("misc.tp.info_global".into(), "global".into());

    m.insert("misc.tp.info_index".into(), "Index".into());

    m.insert("misc.tp.info_name".into(), "Name".into());

    m.insert("misc.tp.info_path".into(), "Path".into());

    m.insert("misc.tp.info_source".into(), "Source".into());

    m.insert("misc.tp.info_title".into(), "\u{1F4CC} Teleport Savepoint Info".into());

    m.insert("misc.tp.invalid_format".into(), "Invalid format. Use: tp rnm <old> to <new>".into());

    m.insert("misc.tp.not_found".into(), "Savepoint '{}' not found".into());

    m.insert("misc.tp.type_help".into(), "Type 'tp help' for usage.".into());

    m.insert("misc.tp.unknown_subcmd".into(), "Unknown tp subcommand: {}".into());

    m.insert("misc.tp.usage_add".into(), "Usage: tp add <name> [path]".into());

    m.insert("misc.tp.usage_info".into(), "Usage: tp info <name>".into());

    m.insert("misc.tp.usage_jump".into(), "Usage: tp jump <name|number>".into());

    m.insert("misc.tp.usage_rm".into(), "Usage: tp rm <name|number>[, <name|number>, ...]".into());

    m.insert("misc.tp.usage_rnm".into(), "Usage: tp rnm <old_name> to <new_name>".into());

    m.insert("misc.tpb.unknown".into(), "Unknown tpb option: {}".into());

    m.insert("misc.tpb.usage".into(), "Usage:".into());

    m.insert("misc.tpb.usage_basic".into(), " tpb - Teleport back to previous location".into());

    m.insert("misc.tpb.usage_clear".into(), " tpb clear - Clear teleport history".into());

    m.insert("misc.tpb.usage_history".into(), " tpb history - Show teleport history".into());

    m.insert("misc.ui.removed".into(), "The modern UI was removed. Only the classic shell is available.".into());

    m.insert("misc.venva.activated".into(), "Activated: {}".into());

    m.insert("misc.venva.activating".into(), "Activating virtual environment: {}".into());

    m.insert("misc.venva.active_info".into(), "Virtual environment is now active. pip and python commands will use this environment.".into());

    m.insert("misc.venva.already_in".into(), "Already in a virtual environment. Use 'deactivate' to exit first.".into());

    m.insert("misc.venva.failed".into(), "Failed to activate: {}".into());

    m.insert("misc.venva.no_script".into(), "No activate script found in: {}".into());

    m.insert("misc.venva.not_found".into(), "No virtual environment found. Usage: python_activate <venv_name>".into());

    m.insert("misc.venvd.deactivated".into(), "Deactivated virtual environment".into());

    m.insert("misc.venvd.not_in_venv".into(), "Not in a virtual environment".into());

    m.insert("mode.help_admin".into(), "  mode admin               Exit and re-launch as Administrator".into());

    m.insert("mode.help_admin".into(), "  mode admin               Exit and re-launch as Administrator".into());

    m.insert("mode.help_example_admin".into(), "  mode admin                (UAC prompt will appear)".into());

    m.insert("mode.help_example_admin".into(), "  mode admin                (UAC prompt will appear)".into());

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_examples".into(), "Examples:".into());

    m.insert("mode.help_normal".into(), "  mode normal              Exit and re-launch normally".into());

    m.insert("mode.help_normal".into(), "  mode normal              Exit and re-launch normally".into());

    m.insert("mode.help_usage".into(), "Usage:".into());

    m.insert("msg.ignore.dir_added".into(), "Now ignoring directory: {}".into());

    m.insert("msg.ignore.dir_removed".into(), "No longer ignoring directory: {}".into());

    m.insert("msg.ignore.ext_added".into(), "Now ignoring .{} files".into());

    m.insert("msg.ignore.ext_removed".into(), "Now caring about .{} files".into());

    m.insert("msg.ignore.file_added".into(), "Now ignoring file: {}".into());

    m.insert("msg.ignore.file_removed".into(), "Now caring about file: {}".into());

    m.insert("msg.ignored.dirs".into(), "Ignored directories: {}".into());

    m.insert("msg.ignored.extensions".into(), "Ignored extensions: {}".into());

    m.insert("msg.ignored.extra_extensions".into(), "Extra supported extensions: {}".into());

    m.insert("msg.ignored.extra_files".into(), "Extra supported files: {}".into());

    m.insert("msg.ignored.files".into(), "Ignored files: {}".into());

    m.insert("msg.image.converted".into(), "Image converted and saved to: {}".into());

    m.insert("msg.image.not_found".into(), "Image not found: {}".into());

    m.insert("msg.init.template_created".into(), "Created template: {}".into());

    m.insert("msg.list.entry_format".into(), " {}".into());

    m.insert("msg.list.title".into(), "ntc {} - Available command-line functions:\n".into());

    m.insert("msg.report.binary_clipboard_warning".into(), "{} report cannot be copied to clipboard (binary format)".into());

    m.insert("msg.report.copied_to_clipboard".into(), "{} report copied to clipboard!".into());

    m.insert("msg.report.file_saved".into(), "File saved to: {}".into());

    m.insert("msg.report.path_not_found".into(), "Path not found: {}".into());

    m.insert("msg.report.skipped_unsupported".into(), "Skipped (not support format): {}".into());

    m.insert("msg.setA.error".into(), "Invalid value for setA: {}. Use ON or OFF.".into());

    m.insert("msg.setA.off".into(), "Autosuggest: OFF".into());

    m.insert("msg.setA.on".into(), "Autosuggest: ON".into());

    m.insert("msg.setA.status".into(), "Autosuggest (ghost text): {}".into());

    m.insert("msg.setC.error".into(), "Invalid value for setC: {}. Use ON or OFF.".into());

    m.insert("msg.setC.status".into(), "Color output: {}".into());

    m.insert("msg.setC.success".into(), "Color: {}".into());

    m.insert("msg.setD.error_out_of_range".into(), "Invalid depth value: {}. Must be between 1 and 20.".into());

    m.insert("msg.setD.error_parse".into(), "Invalid depth value: {}. Must be a positive integer between 1 and 20.".into());

    m.insert("msg.setD.status".into(), "Current max depth: {}".into());

    m.insert("msg.setD.success".into(), "Max depth set to: {}".into());

    m.insert("msg.setH.off".into(), "History: OFF".into());

    m.insert("msg.setH.on".into(), "History: ON".into());

    m.insert("msg.setH.path".into(), "History path: {}".into());

    m.insert("msg.setH.path_default".into(), "History path: default".into());

    m.insert("msg.setH.path_reset".into(), "History path reset to default".into());

    m.insert("msg.setH.path_set".into(), "History path set to: {}".into());

    m.insert("msg.setH.status".into(), "History: {}".into());

    m.insert("msg.setL.error".into(), "Invalid value for setL: {}. Use ON or OFF.".into());

    m.insert("msg.setL.status".into(), "Line numbers: {}".into());

    m.insert("msg.setL.success".into(), "Line numbers: {}".into());

    m.insert("msg.setO.status".into(), "Current output path: {}".into());

    m.insert("msg.setO.success".into(), "Output path set to: {}".into());

    m.insert("msg.setT.error_out_of_range".into(), "Invalid thread count: {}. Must be between 1 and 64.".into());

    m.insert("msg.setT.error_parse".into(), "Invalid thread count: {}. Must be a positive integer between 1 and 64.".into());

    m.insert("msg.setT.status".into(), "Current threads: {}".into());

    m.insert("msg.setT.success".into(), "Threads set to: {}".into());

    m.insert("msg.showcg.box_bottom".into(), "\u{2514}{}\u{2518}".into());

    m.insert("msg.showcg.box_divider".into(), "\u{251C}{}\u{2524}".into());

    m.insert("msg.showcg.box_top".into(), "\u{250C}{}\u{2510}".into());

    m.insert("msg.showcg.label_color".into(), "Color:".into());

    m.insert("msg.showcg.label_history".into(), "History:".into());

    m.insert("msg.showcg.label_line_numbers".into(), "Line Numbers:".into());

    m.insert("msg.showcg.label_max_depth".into(), "Max Depth:".into());

    m.insert("msg.showcg.label_output_path".into(), "Output Path:".into());

    m.insert("msg.showcg.label_threads".into(), "Threads:".into());

    m.insert("msg.showcg.label_watcher".into(), "Watcher:".into());
    m.insert("msg.showcg.label_watch_limit".into(), "Watch limit:".into());

    m.insert("msg.showcg.label_dir_view".into(), "Dir View:".into());

    m.insert("msg.showcg.title".into(), "Current Configuration".into());

    m.insert("msg.size.bytes_label".into(), "Bytes:".into());

    m.insert("msg.size.human_label".into(), "Human:".into());

    m.insert("msg.size.path_label".into(), "Path: {}".into());

    m.insert("msg.size.time_label".into(), "Time:".into());

    m.insert("msg.size.title".into(), "Current Directory Size".into());

    m.insert("msg.teleport.error".into(), "Failed to teleport to '{}': {}".into());

    m.insert("msg.teleport.hint_list".into(), "Use 'ntc --tp-list' to see all savepoints.".into());

    m.insert("msg.teleport.not_found".into(), "Teleport point not found: '{}'".into());

    m.insert("msg.teleport.success".into(), "\u{2713} Teleported to '{}' -> {}".into());

    m.insert("msg.version.format".into(), "ntc {}".into());

    m.insert("msg.watch.invalid".into(), "Use --watch ON or --watch OFF".into());

    m.insert("msg.watch.off".into(), "File watcher: OFF (restart ntc to deactivate)".into());

    m.insert("msg.watch.on".into(), "File watcher: ON (restart ntc to activate)".into());

    m.insert("msg.watch.status".into(), "File watcher: {}".into());

    m.insert("msg.watch.limit_status".into(), "Watch display limit: {}".into());

    m.insert("msg.watch.limit_success".into(), "Watch display limit set to {}".into());

    m.insert("msg.watch.limit_error".into(), "Invalid watch limit: '{}'. Use a positive number.".into());

    m.insert("msg.where.current_dir".into(), "Current directory: {}".into());

    m.insert("msg.where.executable".into(), "ntc executable: {}".into());

    m.insert("nav.available_drives".into(), "Available drives:".into());

    m.insert("nav.cancelled".into(), "Cancelled.".into());

    m.insert("nav.config_exists".into(), " \u{2713} Config file exists".into());

    m.insert("nav.drive_prompt".into(), "Enter drive letter or number (or 'cancel'): ".into());

    m.insert("nav.gos_where".into(), "gos where?".into());

    m.insert("navigator.already_root".into(), "Already at root, cannot go back further".into());

    m.insert("navigator.cannot_navigate_to".into(), "Cannot navigate to: {}".into());

    m.insert("navigator.cannot_navigate_to".into(), "Cannot navigate to: {}".into());

    m.insert("navigator.cannot_read_dir".into(), "Cannot read directory: {}".into());

    m.insert("navigator.cannot_read_dir".into(), "Cannot read directory: {}".into());

    m.insert("navigator.drive_windows_only".into(), "Drive navigation is only supported on Windows".into());

    m.insert("navigator.not_a_directory".into(), "{} is not a directory".into());

    m.insert("navigator.venv_activate_unix".into(), "   Run: source ./venv/bin/activate".into());

    m.insert("navigator.venv_activate_win".into(), "   Run: .\\\\\\\\venv\\\\\\\\Scripts\\\\\\\\activate".into());

    m.insert("navigator.venv_tip".into(), "   Run: source ./venv/bin/activate".into());

    m.insert("navigator.venv_tip".into(), "   Run: source ./venv/bin/activate".into());

    m.insert("nfe.binary_file".into(), "(binary / unsupported format)".into());

    m.insert("nfe.binary_file_preview".into(), "(binary file)".into());

    m.insert("nfe.empty_dir".into(), "(empty directory)".into());

    m.insert("nfe.file_truncated".into(), "... (file truncated)".into());

    m.insert("nfe.hint_bar".into(), "h help | b back | f find | / filter | q quit".into());

    m.insert("nfe.no_matches".into(), "(no matches)".into());

    m.insert("nfe.preview_empty".into(), "(empty)".into());

    m.insert("nfe.preview_error".into(), "(cannot read)".into());

    m.insert("nfe.preview_more".into(), "... (more)".into());

    m.insert("nfe.sort_date".into(), "Date".into());

    m.insert("nfe.sort_name".into(), "Name".into());

    m.insert("nfe.sort_size".into(), "Size".into());

    m.insert("nfe.sort_type".into(), "Type".into());

    m.insert("nfe.status_bar".into(), "{} items | {} dirs | {} files | Sort: {} {}".into());

    m.insert("ntpkg.add_to_path".into(), "Add to your PATH:".into());

    m.insert("ntpkg.add_to_sys_path".into(), "Add to your system PATH (requires admin):".into());

    m.insert("ntpkg.add_to_user_path".into(), "Add to your user PATH:".into());

    m.insert("ntpkg.already_installed".into(), "{} {} is already installed at {}".into());

    m.insert("ntpkg.available_hint".into(), " Use 'ntpkg available <language>' to list versions.".into());

    m.insert("ntpkg.configuring".into(), "Configuring (./configure)...".into());

    m.insert("ntpkg.downloading".into(), "Downloading {}".into());

    m.insert("ntpkg.enter_number".into(), "Enter number (1-{}) or 0: ".into());

    m.insert("ntpkg.extracting".into(), "Extracting...".into());

    m.insert("ntpkg.extracting_indent".into(), " Extracting...".into());

    m.insert("ntpkg.extracting_source".into(), "Extracting source tarball...".into());

    m.insert("ntpkg.filter_label".into(), "(filter: \"{}\")".into());

    m.insert("ntpkg.git_not_found".into(), "Git not found on PATH. Downloading MinGit...".into());

    m.insert("ntpkg.help_usage".into(), "ntpkg \u{2014} ntc package manager for language runtimes".into());

    m.insert("ntpkg.help_usage_title".into(), "  Usage:".into());

    m.insert("ntpkg.help_install".into(), "    ntpkg install <language> [version]   Download & install a runtime".into());

    m.insert("ntpkg.help_uninstall".into(), "    ntpkg uninstall <language> [version]  Remove an installed runtime".into());

    m.insert("ntpkg.help_list".into(), "    ntpkg list                           Show installed runtimes".into());

    m.insert("ntpkg.help_use".into(), "    ntpkg use <language> <version>        Activate a specific version".into());

    m.insert("ntpkg.help_which".into(), "    ntpkg which <language>                Show path to active runtime".into());

    m.insert("ntpkg.help_available".into(), "    ntpkg available [language]             List languages or downloadable versions".into());

    m.insert("ntpkg.help_help".into(), "    ntpkg help                            Show this help".into());

    m.insert("ntpkg.ntpx_header".into(), "  ntpx \u{2014} run any installed package's binary".into());

    m.insert("ntpkg.ntpx_usage_line".into(), "    ntpx <package> [binary] [args...]".into());

    m.insert("ntpkg.help_examples_title".into(), "  Examples:".into());

    m.insert("ntpkg.help_ex_ntpx_go".into(), "    ntpx go version              # run 'go version'".into());

    m.insert("ntpkg.help_ex_ntpx_py".into(), "    ntpx python --version        # run 'python --version'".into());

    m.insert("ntpkg.help_ex_ntpx_msys2".into(), "    ntpx msys2 gcc --version     # run 'gcc --version' (explicit binary)".into());

    m.insert("ntpkg.available_title".into(), "Available packages".into());

    m.insert("ntpkg.package_info_title".into(), "Package info: {}".into());

    m.insert("ntpkg.active_label".into(), "Active:".into());

    m.insert("ntpkg.installed_label".into(), "Installed:".into());

    m.insert("ntpkg.location_label".into(), "Location:".into());

    m.insert("ntpkg.binaries_label".into(), "Binaries:".into());

    m.insert("ntpkg.not_set".into(), "(not set)".into());

    m.insert("ntpkg.none".into(), "(none)".into());

    m.insert("ntpkg.active_marker".into(), " (active)".into());

    m.insert("ntpkg.info_usage".into(), "Usage: ntpkg info <language>".into());

    m.insert("ntpkg.install_failed".into(), "Install failed: {}".into());

    m.insert("ntpkg.install_loc".into(), "Install location: {}".into());

    m.insert("ntpkg.install_usage".into(), "Usage: ntpkg install <language> [version]".into());

    m.insert("ntpkg.installed_msg".into(), "{} {} installed!".into());

    m.insert("ntpkg.installing".into(), "Installing (make install)...".into());

    m.insert("ntpkg.latest_suffix".into(), "(latest)".into());

    m.insert("ntpkg.launch_installer".into(), "Launching installer. Complete the setup, then close the installer window...".into());

    m.insert("ntpkg.magick_macos".into(), "ImageMagick on macOS:".into());

    m.insert("ntpkg.magick_manual_reg".into(), "If installed, register manually: ntpkg use imagemagick <version>".into());

    m.insert("ntpkg.magick_not_found".into(), "Could not auto-detect magick.exe after install.".into());

    m.insert("ntpkg.msys2_gcc_incomplete".into(), "GCC installation may not have completed. You can install it later via pacman.".into());

    m.insert("ntpkg.msys2_gcc_not_found".into(), "GCC not found yet. Run pacman to install it:".into());

    m.insert("ntpkg.msys2_installed".into(), "MSYS2 installed! Install optional packages via pacman:".into());

    m.insert("ntpkg.msys2_linux_gcc".into(), "On this platform, install GCC with your package manager:".into());

    m.insert("ntpkg.msys2_running_pacman".into(), "Running: pacman -S mingw-w64-ucrt-x86_64-gcc".into());

    m.insert("ntpkg.no_active_version".into(), "No active version for '{}' and no version specified".into());

    m.insert("ntpkg.no_match".into(), "No packages match '{}'".into());

    m.insert("ntpkg.no_packages".into(), "No packages installed. Use 'ntpkg install <language>'".into());

    m.insert("ntpkg.no_versions".into(), "No versions tracked for '{}'".into());

    m.insert("ntpkg.no_versions_for".into(), "No versions available for '{}'".into());

    m.insert("ntpkg.not_installed".into(), "{} {} is not installed".into());

    m.insert("ntpkg.now_using".into(), "Now using {} {}".into());

    m.insert("ntpkg.npm_bin_in_path".into(), "npm global bin is in PATH".into());

    m.insert("ntpkg.npm_detected".into(), "Node.js and npm detected".into());

    m.insert("ntpkg.npm_not_detected".into(), "Node.js/npm not detected.".into());

    m.insert("ntpkg.npm_section_node_first".into(), "  ── Install Node.js first ──".into());

    m.insert("ntpkg.npm_node_found_at".into(), "  ✓ Node.js found at: {}".into());

    m.insert("ntpkg.npm_node_not_found".into(), "  ✗ Node.js not found".into());

    m.insert("ntpkg.npm_install_via_pkg".into(), "  Install via pkg:".into());

    m.insert("ntpkg.npm_install_via_ntpkg".into(), "  Install via ntpkg:".into());

    m.insert("ntpkg.npm_ntpkg_cmd".into(), "    ntpkg install {}".into());

    m.insert("ntpkg.npm_cmd".into(), "    npm i -g {}".into());

    m.insert("ntpkg.npm_or_download_nodejs".into(), "  Or download from: https://nodejs.org".into());

    m.insert("ntpkg.npm_or_fnm_nvm_volta".into(), "  Or install via fnm/nvm/volta".into());

    m.insert("ntpkg.npm_found_at".into(), "  ✓ npm found at: {}".into());

    m.insert("ntpkg.npm_not_found".into(), "  ✗ npm not found".into());

    m.insert("ntpkg.npm_then_run".into(), "  Then run:".into());

    m.insert("ntpkg.npm_then_rerun".into(), "  Then re-run:".into());

    m.insert("ntpkg.npm_bin_not_in_path".into(), "npm global bin not found in PATH ({})".into());

    m.insert("ntpkg.path_add_label".into(), "  Add to PATH:".into());

    m.insert("ntpkg.npm_install_section".into(), "  ── Install {} ──".into());

    m.insert("ntpkg.npm_run_cmd_label".into(), "  Run this command:".into());

    m.insert("ntpkg.ntpx_usage".into(), "Usage: ntpx <package> [binary] [args...]".into());

    m.insert("ntpkg.package_count".into(), "{} packages".into());

    m.insert("ntpkg.php_skip_register".into(), "Skipped registering PHP with ntpkg.".into());

    m.insert("ntpkg.removed_msg".into(), "{} {} removed".into());

    m.insert("ntpkg.select_jdk_version".into(), "Select JDK {} version".into());

    m.insert("ntpkg.select_version".into(), "Select {} version".into());

    m.insert("ntpkg.skip_register".into(), "Skipped registering {} with ntpkg.".into());

    m.insert("ntpkg.termux_use_pkg".into(), "On Termux, install via pkg instead of ntpkg:".into());

    m.insert("ntpkg.termux_cmd".into(), "    pkg install {}".into());

    m.insert("ntpkg.termux_no_pkg".into(), "No Termux pkg package available for '{}'.".into());

    m.insert("ntpkg.src_act".into(), "act - Run GitHub Actions locally".into());

    m.insert("ntpkg.src_act".into(), "act - Run GitHub Actions locally".into());

    m.insert("ntpkg.src_cloudflare".into(), "cloudflared - Cloudflare Tunnel client".into());

    m.insert("ntpkg.src_cloudflare".into(), "cloudflared - Cloudflare Tunnel client".into());

    m.insert("ntpkg.src_devops_tool".into(), "Azure DevOps CLI tool".into());

    m.insert("ntpkg.src_devops_tool".into(), "Azure DevOps CLI tool".into());

    m.insert("ntpkg.src_docker".into(), "Docker - Container runtime".into());

    m.insert("ntpkg.src_docker".into(), "Docker - Container runtime".into());

    m.insert("ntpkg.src_docker_compose".into(), "Docker Compose - Multi-container orchestration".into());

    m.insert("ntpkg.src_docker_compose".into(), "Docker Compose - Multi-container orchestration".into());

    m.insert("ntpkg.src_dotnet".into(), ".NET SDK - Cross-platform development".into());

    m.insert("ntpkg.src_dotnet".into(), ".NET SDK - Cross-platform development".into());

    m.insert("ntpkg.src_ffmpeg".into(), "FFmpeg - Media converter".into());

    m.insert("ntpkg.src_ffmpeg".into(), "FFmpeg - Media converter".into());

    m.insert("ntpkg.src_freebuff".into(), "Freebuff - Free AI coding agent".into());

    m.insert("ntpkg.src_freebuff".into(), "Freebuff - Free AI coding agent".into());

    m.insert("ntpkg.src_flutter".into(), "Flutter - UI toolkit".into());

    m.insert("ntpkg.src_flutter".into(), "Flutter - UI toolkit".into());

    m.insert("ntpkg.src_gh".into(), "GitHub CLI - GitHub from terminal".into());

    m.insert("ntpkg.src_gh".into(), "GitHub CLI - GitHub from terminal".into());

    m.insert("ntpkg.src_git".into(), "Git - Version control".into());

    m.insert("ntpkg.src_git".into(), "Git - Version control".into());

    m.insert("ntpkg.src_go".into(), "Go - Programming language".into());

    m.insert("ntpkg.src_go".into(), "Go - Programming language".into());

    m.insert("ntpkg.src_helm".into(), "Helm - Kubernetes package manager".into());

    m.insert("ntpkg.src_helm".into(), "Helm - Kubernetes package manager".into());

    m.insert("ntpkg.src_imagemagick".into(), "ImageMagick - Image manipulation".into());

    m.insert("ntpkg.src_imagemagick".into(), "ImageMagick - Image manipulation".into());

    m.insert("ntpkg.src_java".into(), "Java - Runtime environment".into());

    m.insert("ntpkg.src_java".into(), "Java - Runtime environment".into());

    m.insert("ntpkg.src_kilocode".into(), "Kiloc - AI code assistant".into());

    m.insert("ntpkg.src_kilocode".into(), "Kiloc - AI code assistant".into());

    m.insert("ntpkg.src_kind".into(), "kind - Kubernetes in Docker".into());

    m.insert("ntpkg.src_kind".into(), "kind - Kubernetes in Docker".into());

    m.insert("ntpkg.src_kotlin".into(), "Kotlin - Programming language".into());

    m.insert("ntpkg.src_kotlin".into(), "Kotlin - Programming language".into());

    m.insert("ntpkg.src_kubectl".into(), "kubectl - Kubernetes CLI".into());

    m.insert("ntpkg.src_kubectl".into(), "kubectl - Kubernetes CLI".into());

    m.insert("ntpkg.src_kustomize".into(), "Kustomize - Kubernetes config customization".into());

    m.insert("ntpkg.src_kustomize".into(), "Kustomize - Kubernetes config customization".into());

    m.insert("ntpkg.src_lua".into(), "Lua - Scripting language".into());

    m.insert("ntpkg.src_lua".into(), "Lua - Scripting language".into());

    m.insert("ntpkg.src_mimocode".into(), "Mimic - AI coding agent".into());

    m.insert("ntpkg.src_mimocode".into(), "Mimic - AI coding agent".into());

    m.insert("ntpkg.src_msys2".into(), "MSYS2 - Software distribution and building platform".into());

    m.insert("ntpkg.src_msys2".into(), "MSYS2 - Software distribution and building platform".into());

    m.insert("ntpkg.src_nim".into(), "Nim - Programming language".into());

    m.insert("ntpkg.src_nim".into(), "Nim - Programming language".into());

    m.insert("ntpkg.src_node".into(), "Node.js - JavaScript runtime".into());

    m.insert("ntpkg.src_node".into(), "Node.js - JavaScript runtime".into());

    m.insert("ntpkg.src_omniroute".into(), "OmniRoute - Visualization tool".into());

    m.insert("ntpkg.src_omniroute".into(), "OmniRoute - Visualization tool".into());

    m.insert("ntpkg.src_opencode".into(), "opencode - CLI coding agent".into());

    m.insert("ntpkg.src_opencode".into(), "opencode - CLI coding agent".into());

    m.insert("ntpkg.src_php".into(), "PHP - Scripting language".into());

    m.insert("ntpkg.src_php".into(), "PHP - Scripting language".into());

    m.insert("ntpkg.src_python".into(), "Python - Programming language".into());

    m.insert("ntpkg.src_python".into(), "Python - Programming language".into());

    m.insert("ntpkg.src_ruby".into(), "Ruby - Programming language".into());

    m.insert("ntpkg.src_ruby".into(), "Ruby - Programming language".into());

    m.insert("ntpkg.src_rust".into(), "Rust - Systems programming language".into());

    m.insert("ntpkg.src_rust".into(), "Rust - Systems programming language".into());

    m.insert("ntpkg.src_scala".into(), "Scala - Programming language".into());

    m.insert("ntpkg.src_scala".into(), "Scala - Programming language".into());

    m.insert("ntpkg.src_sqlite".into(), "SQLite - Embedded database".into());

    m.insert("ntpkg.src_sqlite".into(), "SQLite - Embedded database".into());

    m.insert("ntpkg.src_sqlitestudio".into(), "SQLiteStudio - SQLite manager".into());

    m.insert("ntpkg.src_sqlitestudio".into(), "SQLiteStudio - SQLite manager".into());

    m.insert("ntpkg.src_swift".into(), "Swift - Programming language".into());

    m.insert("ntpkg.src_swift".into(), "Swift - Programming language".into());

    m.insert("ntpkg.src_vscode".into(), "VS Code - Code editor".into());

    m.insert("ntpkg.src_vscode".into(), "VS Code - Code editor".into());

    m.insert("ntpkg.src_zig".into(), "Zig - Programming language".into());

    m.insert("ntpkg.src_zig".into(), "Zig - Programming language".into());

    m.insert("ntpkg.src_tesseract".into(), "Tesseract - OCR engine".into());

    m.insert("ntpkg.tesseract_running_installer".into(), "Running Tesseract installer silently (elevated)...".into());

    m.insert("ntpkg.tesseract_not_found".into(), "Could not auto-detect tesseract.exe after install. Add its folder to PATH with: usrpath add <dir> (then 'mode admin' + 'syspath add <dir>').".into());

    m.insert("ntpkg.tesseract_linux".into(), "Install Tesseract on Linux via your package manager:".into());

    m.insert("ntpkg.tesseract_macos".into(), "Install Tesseract on macOS via Homebrew:".into());

    m.insert("ntpkg.tesseract_unsupported".into(), "Tesseract installation is not supported on this platform.".into());

    m.insert("ntpkg.src_nginx".into(), "nginx - Web server / reverse proxy".into());

    m.insert("ntpkg.nginx_unsupported".into(), "nginx installation is not supported on this platform.".into());

    m.insert("ntpkg.nginx_system_hint_linux".into(), "Install nginx on Linux via your package manager:".into());

    m.insert("ntpkg.nginx_system_hint_macos".into(), "Install nginx on macOS via Homebrew:".into());

    m.insert("ntpkg.nginx_termux".into(), "Install nginx on Termux via its package manager:".into());

    m.insert("ntpkg.nginx_choose_method".into(), "No official prebuilt Linux/macOS binary. Choose an install method:".into());

    m.insert("ntpkg.nginx_option_source".into(), "Build from source (downloads nginx source tarball)".into());

    m.insert("ntpkg.nginx_option_system".into(), "Install via system package manager".into());

    m.insert("ntpkg.nginx_run_commands_system".into(), "Install nginx with the package-manager commands shown above (not tracked by ntpkg).".into());

    m.insert("ntpkg.nginx_build_confirm".into(), "Build nginx {} from source into the ntpkg packages dir? Requires make/gcc + PCRE/Zlib/OpenSSL dev packages.".into());

    m.insert("ntpkg.nginx_build_deps".into(), "Build deps hint: build-essential libpcre2-dev zlib1g-dev libssl-dev (Debian/Ubuntu).".into());

    m.insert("ntpkg.nginx_configuring".into(), "Running ./configure...".into());

    m.insert("ntpkg.nginx_compiling".into(), "Compiling...".into());

    m.insert("ntpkg.supported_langs".into(), " Supported languages:".into());

    m.insert("ntpkg.uninstall_failed".into(), "Uninstall failed: {}".into());

    m.insert("ntpkg.uninstall_usage".into(), "Usage: ntpkg uninstall <language> [version]".into());

    m.insert("ntpkg.unknown_subcmd".into(), "Unknown ntpkg subcommand: '{}'. Use 'ntpkg help'".into());

    m.insert("ntpkg.use_usage".into(), "Usage: ntpkg use <language> <version>".into());

    m.insert("ntpkg.which_usage".into(), "Usage: ntpkg which <language> [version]".into());

    m.insert("output.filename_header".into(), "{}:".into());

    m.insert("output.line_prefix".into(), "LINE {}>>".into());

    m.insert("output.mkdir_failed".into(), "Failed to create directory: {}".into());

    m.insert("output.mkfile_failed".into(), "Failed to create file: {}".into());

    m.insert("output.print_error".into(), "\u{2717} {}".into());

    m.insert("output.print_info".into(), "\u{2139} {}".into());

    m.insert("output.print_success".into(), "\u{2713} {}".into());

    m.insert("output.print_warning".into(), "\u{26A0} {}".into());

    m.insert("output.read_file_failed".into(), "Failed to read file '{}'".into());

    m.insert("output.write_failed".into(), "Failed to write to file: {}".into());

    m.insert("ping.help.examples".into(), "Examples:".into());

    m.insert("ping.help.opt_count".into(), " -c, --count <n> Number of pings (default 4)".into());

    m.insert("ping.help.opt_help".into(), " -h, --help Show this help".into());

    m.insert("ping.help.options".into(), "Options:".into());

    m.insert("ping.help.title".into(), "Ping".into());

    m.insert("ping.help.usage".into(), "Usage:".into());

    m.insert("ping.help.usage_line".into(), " ping <host> [options]".into());

    m.insert("qsc.macos_listen_hint".into(), "QSC: Run 'qsc listen' once support is added.".into());

    m.insert("qsc.macos_not_implemented".into(), "QSC: macOS global hotkey support not yet implemented.".into());

    m.insert("qsc.unsupported_platform".into(), "QSC: unsupported platform".into());

    m.insert("ral_export.all".into(), "select all".into());

    m.insert("ral_export.cancel".into(), "cancel".into());

    m.insert("ral_export.export_cancelled".into(), "Export cancelled.".into());

    m.insert("ral_export.finish".into(), "finish export".into());

    m.insert("ral_export.prompt".into(), "ral-export>".into());

    m.insert("ral_export.select".into(), "select/deselect by number".into());

    m.insert("ral_export.selected".into(), "selected".into());

    m.insert("ral_export.selected_mark".into(), "(selected)".into());

    m.insert("ral_export.title".into(), "ral export — Select Aliases to Export".into());

    m.insert("ran.already_exists".into(), "{} already exists in {}".into());

    m.insert("ran.calling_not_allowed".into(), "Calling 'ran' from within a NTCRANFILE target is not allowed".into());

    m.insert("ran.completed".into(), "Command completed successfully.".into());

    m.insert("ran.created".into(), "Created {}".into());

    m.insert("ran.desc_aliases".into(), " Commands are expanded through ntc's run-alias system, so aliases work too.".into());

    m.insert("ran.desc_aliases2".into(), " Prefix with 'run ' to bypass alias expansion and run directly as a system command.".into());

    m.insert("ran.desc_deps".into(), " Targets can depend on other targets. Dependencies run first, in order.".into());

    m.insert("ran.desc_note".into(), " Calling 'ran' from within a NTCRANFILE target command is not allowed.".into());

    m.insert("ran.desc_reserved".into(), " init, deinit, help, list, ls \u{2014} these cannot be used as target names.".into());

    m.insert("ran.desc_vars".into(), " Define variables under [vars] and reference them with $(VAR_NAME) in commands.".into());

    m.insert("ran.desc_vars2".into(), " They work like Make variables but use $(...) syntax.".into());

    m.insert("ran.error_prefix".into(), "ran: {}".into());

    m.insert("ran.exec_failed".into(), "Failed to execute command: {}".into());

    m.insert("ran.executing".into(), "Executing: {}".into());

    m.insert("ran.exit_code".into(), "Command exited with code: {}".into());

    m.insert("ran.get_cwd_failed".into(), "Failed to get current directory: {}".into());

    m.insert("ran.help_deinit_fmt".into(), " ran deinit Remove {} from the current directory".into());

    m.insert("ran.help_help".into(), " ran help Show this help".into());

    m.insert("ran.help_init_fmt".into(), " ran init Create a {} in the current directory".into());

    m.insert("ran.help_list".into(), " ran list List all available targets".into());

    m.insert("ran.help_standalone".into(), " ran Run the 'standalone' target (default)".into());

    m.insert("ran.help_target".into(), " ran <target> Run a specific target".into());

    m.insert("ran.help_title".into(), "ran \u{2014} ntc task runner (like Make but ntc-native)".into());

    m.insert("ran.help_usage".into(), "Usage:".into());

    m.insert("ran.not_found".into(), "No {} found in {}".into());

    m.insert("ran.remove_failed".into(), "Failed to remove {}: {}".into());

    m.insert("ran.removed".into(), "Removed {}".into());

    m.insert("ran.section_aliases".into(), "Run aliases:".into());

    m.insert("ran.section_deps".into(), "Dependencies:".into());

    m.insert("ran.section_note".into(), "Note:".into());

    m.insert("ran.section_reserved".into(), "Reserved target names:".into());

    m.insert("ran.section_vars".into(), "Variables:".into());

    m.insert("ran.terminated".into(), "Command terminated (Ctrl+C)".into());

    m.insert("ranfile.available_title".into(), "Available targets:".into());

    m.insert("ranfile.has_command".into(), " (has command)".into());

    m.insert("ranfile.depends_on".into(), " ─▶ depends on: ".into());

    m.insert("ranfile.runner.hint".into(), "Run {} to see available targets.".into());

    m.insert("report.generating_html".into(), "\u{1F4CA} Generating HTML report for: {}".into());

    m.insert("report.generating_json".into(), "\u{1F4CA} Generating JSON report for: {}".into());

    m.insert("report.generating_md".into(), "\u{1F4CA} Generating Markdown report for: {}".into());

    m.insert("report.saved_html".into(), "\u{2705} HTML report saved to: {}".into());

    m.insert("report.saved_json".into(), "\u{2705} JSON report saved to: {}".into());

    m.insert("report.saved_md".into(), "\u{2705} Markdown report saved to: {}".into());

    m.insert("report.saved_to".into(), "Report saved to: {}".into());

    m.insert("scr.calling_not_allowed".into(), "Nested scr calls are not allowed inside a script".into());

    m.insert("scr.cmd_failed".into(), "command failed".into());

    m.insert("scr.completed".into(), "Script '{}' completed successfully ({} commands)".into());

    m.insert("scr.continue_warn".into(), "Continuing after error at line {}".into());

    m.insert("scr.desc_args".into(), " $1..$9, $@ Positional script arguments".into());

    m.insert("scr.desc_force".into(), " @ <cmd> Force a system command (bypass ntc builtins)".into());

    m.insert("scr.desc_interp".into(), " $var / ${var} / ${var:-default} Variable interpolation".into());

    m.insert("scr.desc_set".into(), " set NAME = value Define a variable".into());

    m.insert("scr.done_errors".into(), "Script '{}' finished with {} error(s)".into());

    m.insert("scr.error_at".into(), "Error at {}:{} \u{2014} {}".into());

    m.insert("scr.executing".into(), "[{}] Executing: {}".into());

    m.insert("scr.failed_abort".into(), "Script '{}' aborted at line {} ({} error(s))".into());

    m.insert("scr.help_continue".into(), " scr -c <file.ntc> [args...] Continue after errors".into());

    m.insert("scr.help_help".into(), " scr help Show this help".into());

    m.insert("scr.help_init".into(), " scr init [name] Create a script template".into());

    m.insert("scr.help_ls".into(), " scr ls List *.ntc scripts in the current directory".into());

    m.insert("scr.help_run".into(), " scr <file.ntc> [args...] Run an automation script".into());

    m.insert("scr.help_title".into(), "scr \u{2014} ntc automation script runner".into());

    m.insert("scr.help_usage".into(), "Usage:".into());

    m.insert("scr.init_created".into(), "Created template {}".into());

    m.insert("scr.init_exists".into(), "{} already exists".into());

    m.insert("scr.ls_empty".into(), "No *.ntc scripts found in {}".into());

    m.insert("scr.ls_title".into(), "AUTOMATION SCRIPTS:".into());

    m.insert("scr.no_commands".into(), "{} contains no commands to run".into());

    m.insert("scr.not_found".into(), "Script not found: {} ({})".into());

    m.insert("scr.start".into(), "Running script {}...".into());

    m.insert("scr.syscmd".into(), "Executing: {}".into());

    m.insert("scr.syscmd_exit".into(), "Command exited with code: {}".into());

    m.insert("scr.syscmd_terminated".into(), "Command terminated (Ctrl+C)".into());

    m.insert("status.none".into(), "none".into());

    m.insert("status.off".into(), "OFF".into());

    m.insert("status.on".into(), "ON".into());

    m.insert("teleport.added".into(), "Savepoint '{}' created -> {}".into());

    m.insert("teleport.all_cleared".into(), "All savepoints cleared.".into());

    m.insert("teleport.back_hint".into(), "Use 'tpb' to return to previous location.".into());

    m.insert("teleport.back_opt".into(), "b. Teleport Back".into());

    m.insert("teleport.cancel_opt".into(), "0. Cancel".into());

    m.insert("teleport.choose_prompt".into(), "Enter number to teleport (1-{}) or b/h/0: ".into());

    m.insert("teleport.clear_cancelled".into(), "Clear cancelled.".into());

    m.insert("teleport.clear_confirm".into(), "\u{26A0} Are you sure you want to delete ALL savepoints? (y/N): ".into());

    m.insert("teleport.clear_hint".into(), "Use 'tp clear' to clear history.".into());

    m.insert("teleport.fs_failed".into(), "Failed to access file system: {}".into());

    m.insert("teleport.history_cleared".into(), "Teleport history cleared.".into());

    m.insert("teleport.history_opt".into(), "h. Show History".into());

    m.insert("teleport.history_title".into(), "\u{1F4DC} Teleport History (most recent last)".into());

    m.insert("teleport.invalid_char".into(), "Invalid character in name: '{}'".into());

    m.insert("teleport.invalid_index".into(), "Invalid index: {}. Use 1-{}".into());

    m.insert("teleport.invalid_input_interactive".into(), "Invalid input: {}. Enter a number, 'b', or 'h'.".into());

    m.insert("teleport.invalid_name".into(), "Invalid name: '{}' is reserved or empty".into());

    m.insert("teleport.jump_failed".into(), "Could not teleport: '{}'".into());

    m.insert("teleport.jump_hint".into(), "Use 'tp jump <name>' to teleport.".into());

    m.insert("teleport.jumped".into(), "Teleported to '{}' -> {}".into());

    m.insert("teleport.list_title".into(), "\u{1F4CC} Your Teleport Savepoints (global)".into());

    m.insert("teleport.menu_title".into(), "\u{1F4CC} Teleport - Your Savepoints (global)".into());

    m.insert("teleport.name_exists".into(), "Cannot rename: '{}' already exists as a savepoint".into());

    m.insert("teleport.name_taken".into(), "Savepoint '{}' already exists. Use 'tp edit' to update it.".into());

    m.insert("teleport.name_taken_edit".into(), "Savepoint '{}' already exists".into());

    m.insert("teleport.no_history_first".into(), "No teleport history. Use 'tp jump <name>' first to create history.".into());

    m.insert("teleport.no_history_start".into(), "No teleport history. Use 'tp jump <name>' to start building history.".into());

    m.insert("teleport.no_savepoints_to_clear".into(), "No savepoints to clear.".into());

    m.insert("teleport.no_savepoints_yet".into(), "No savepoints yet. Use 'tp add <name>' to create one.".into());

    m.insert("teleport.not_found".into(), "Savepoint not found: '{}'".into());

    m.insert("teleport.overwrite_cancelled".into(), "Overwrite cancelled.".into());

    m.insert("teleport.overwrite_prompt".into(), "Savepoint '{}' already exists at a different location. Overwrite? (y/N): ".into());

    m.insert("teleport.prev_gone".into(), "Previous location no longer exists: {}".into());

    m.insert("teleport.removed".into(), "Removed savepoint: '{}'".into());

    m.insert("teleport.removing_from_history".into(), "Removing from history. Use 'tp history' to see remaining entries.".into());

    m.insert("teleport.rename_failed".into(), "Failed to rename savepoint: {}".into());

    m.insert("teleport.rename_same".into(), "New name is the same as the old name".into());

    m.insert("teleport.renamed".into(), "Renamed savepoint '{}' \u{2192} '{}'".into());

    m.insert("teleport.renamed_with_path".into(), "Renamed savepoint '{}' -> '{}' (-> {})".into());

    m.insert("teleport.teleport_cancelled".into(), "Teleport cancelled.".into());

    m.insert("teleport.teleported_back".into(), "Teleported back to: {}".into());

    m.insert("teleport.tpb_hint".into(), "Use 'tpb' to go back to the previous location.".into());

    m.insert("teleport.updated".into(), "Savepoint '{}' updated: {} -> {}".into());

    m.insert("theme.create_failed".into(), "Failed to create theme: {}".into());

    m.insert("theme.create_file_failed".into(), "Failed to create file: {}".into());

    m.insert("theme.create_first".into(), "Run 'theme add <name>' to create your first theme".into());

    m.insert("theme.created".into(), "Created theme '{}' (opened in editor)".into());

    m.insert("theme.created_hint".into(), "Edit the file, save, then run 'theme reload' to apply changes".into());

    m.insert("theme.current_title".into(), "Current theme: {}".into());

    m.insert("theme.dir_title".into(), "Theme directory:".into());

    m.insert("theme.edit_hint".into(), "Edit '{}', save, then run 'theme reload'".into());

    m.insert("theme.export_failed".into(), "Failed to export theme: {}".into());

    m.insert("theme.export_failed_simple".into(), "Failed to export: {}".into());

    m.insert("theme.exported_simple".into(), "Exported to: {}".into());

    m.insert("theme.exported_to".into(), "Exported theme '{}' to: {}".into());

    m.insert("theme.file_exists".into(), "File already exists: {}".into());

    m.insert("theme.file_not_found".into(), "File not found: {}".into());

    m.insert("theme.files_title".into(), "Theme files:".into());

    m.insert("theme.import_failed".into(), "Failed to import theme: {}".into());

    m.insert("theme.import_failed_simple".into(), "Failed to import: {}".into());

    m.insert("theme.import_hint".into(), "Run 'theme list' to see available themes".into());

    m.insert("theme.imported".into(), "Imported theme from: {}".into());

    m.insert("theme.info_author".into(), "Author".into());

    m.insert("theme.info_defined".into(), "\u{2713} defined".into());

    m.insert("theme.info_description".into(), "Description".into());

    m.insert("theme.info_editor".into(), "Editor colors".into());

    m.insert("theme.info_shell".into(), "Shell colors".into());

    m.insert("theme.info_syntax".into(), "Syntax colors".into());

    m.insert("theme.info_title".into(), "\u{1F3A8} Theme: {}".into());

    m.insert("theme.invalid_number".into(), "Invalid number: {}. Use 1-{}".into());

    m.insert("theme.name_empty".into(), "Theme name cannot be empty".into());

    m.insert("theme.no_dir".into(), "Could not determine theme directory".into());

    m.insert("theme.no_dir_yet".into(), "(directory does not exist yet)".into());

    m.insert("theme.no_files".into(), "(no theme files)".into());

    m.insert("theme.not_found".into(), "Theme '{}' not found".into());

    m.insert("theme.not_found_hint".into(), "Theme not found: {}. Use 'theme list' to see available themes.".into());

    m.insert("theme.opened_dir".into(), "Opened theme directory: {}".into());

    m.insert("theme.reloaded".into(), "Themes reloaded from disk".into());

    m.insert("theme.remove_failed".into(), "Failed to remove theme: {}".into());

    m.insert("theme.removed".into(), "Removed theme: {}".into());

    m.insert("theme.rename_failed".into(), "Failed to rename theme: {}".into());

    m.insert("theme.rename_failed_simple".into(), "Failed to rename: {}".into());

    m.insert("theme.renamed".into(), "Renamed theme '{}' -> '{}'".into());

    m.insert("theme.renamed_simple".into(), "Renamed '{}' -> '{}'".into());

    m.insert("theme.switch_failed".into(), "Failed to switch to theme: {}".into());

    m.insert("theme.switch_failed_simple".into(), "Failed to switch to theme: {}".into());

    m.insert("theme.switched".into(), "Switched to theme: {}".into());

    m.insert("theme.template_created".into(), "Created theme template: {}".into());

    m.insert("theme.template_info".into(), "This is a local template file. Use 'theme import' to add it to your theme list.".into());

    m.insert("theme.type_interactive".into(), "Type 'theme' for interactive menu".into());

    m.insert("theme.ui.current".into(), "Current theme".into());

    m.insert("theme.ui.exit".into(), "Exiting theme manager.".into());

    m.insert("theme.ui.name_prompt".into(), "Theme name: ".into());

    m.insert("theme.ui.theme_prompt".into(), "theme>".into());

    m.insert("theme.ui.title".into(), "\u{1F3A8} ntc Theme Manager".into());

    m.insert("theme.unknown_cmd".into(), "Unknown command: {}".into());

    m.insert("theme.usage_add".into(), "Usage: theme add <name>".into());

    m.insert("theme.usage_export".into(), "Usage: theme export <name> [output_file]".into());

    m.insert("theme.usage_import".into(), "Usage: theme import <file.ntc_theme>".into());

    m.insert("theme.usage_info".into(), "Usage: theme info <name>".into());

    m.insert("theme.usage_rename".into(), "Usage: theme rename <old_name> to <new_name>".into());

    m.insert("theme.usage_rm".into(), "Usage: theme rm <name>".into());

    m.insert("theme.usage_rnm".into(), "Usage: theme rnm <old_name> to <new_name>".into());

    m.insert("tutorial.complete_msg".into(), "You now know the essentials of NTC:".into());

    m.insert("tutorial.complete_title".into(), " Tutorial Complete! ".into());

    m.insert("tutorial.demonstrating".into(), "Demonstrating:".into());

    m.insert("tutorial.exit_hint".into(), "Type 'exit' to quit NTC.".into());

    m.insert("tutorial.help_hint".into(), "Type 'help' for the full command reference.".into());

    m.insert("tutorial.prompt_continue".into(), "Press Enter to continue".into());

    m.insert("tutorial.prompt_exit_label".into(), "exit".into());

    m.insert("tutorial.prompt_skip_label".into(), "skip".into());

    m.insert("tutorial.return_hint".into(), "Press Enter to return to the shell".into());

    m.insert("tutorial.step_title".into(), " NTC Tutorial \u{2014} Step {}/{} ".into());

    m.insert("tutorial.tip_label".into(), "\u{1F4A1} Tip:".into());

    m.insert("up.help.examples".into(), "Examples:".into());

    m.insert("up.help.opt_data".into(), " -d, --data <body> Upload inline data".into());

    m.insert("up.help.opt_header".into(), " -H, --header <\"Key: Value\"> Add request header (repeatable)".into());

    m.insert("up.help.opt_help".into(), " -h, --help Show this help".into());

    m.insert("up.help.opt_post".into(), " --post Use POST instead of PUT".into());

    m.insert("up.help.options".into(), "Options:".into());

    m.insert("up.help.title".into(), "Upload (up)".into());

    m.insert("up.help.usage".into(), "Usage:".into());

    m.insert("up.help.usage_data".into(), " up -d \"data\" <url> Upload inline text data".into());

    m.insert("up.help.usage_line".into(), " up <file> <url> Upload file via PUT".into());

    m.insert("up.help.usage_post".into(), " up <file> <url> --post Upload via POST".into());

    m.insert("watcher.file_changed".into(), "\u{2190} file watcher detected".into());

    m.insert("watcher.more_events".into(), "\u{2026} and {} more".into());

    m.insert("watcher.tree_refreshed".into(), "watcher \u{2192} tree refreshed".into());

    m.insert("help.tmprt.title".into(), "TEMPERATURE & WEATHER COMMANDS:".into());

    m.insert("help.tmprt.com".into(), "tmprt com [secs] ## Live hardware temperature monitor (q=quit s=freeze c='oC/'oF)".into());

    m.insert("help.tmprt.weather".into(), "tmprt weather [city] ## Current weather via OpenWeather API".into());

    m.insert("help.tmprt.key".into(), "tmprt key [API_KEY] ## Save/display OpenWeather API key".into());

    m.insert("help.tmprt.city".into(), "tmprt city [name] ## Save/display default city for weather".into());

    m.insert("help.tmprt.sensors".into(), "tmprt sensors <status|install|uninstall|debug> ## Manage hardware sensor backends (LHM)".into());

    m.insert("help.tmprt.example1".into(), "tmprt com 5 ## Live view refreshing every 5 seconds".into());

    m.insert("help.tmprt.example2".into(), "tmprt weather Bangkok ## Weather for Bangkok".into());

    m.insert("help.tmprt.example3".into(), "tmprt key 1234567890abcdef ## Save API key".into());

    m.insert("help.tmprt.example4".into(), "tmprt city Bangkok ## Set default city".into());

    m.insert("help.git.title".into(), "GIT INTEGRATION COMMANDS:".into());

    m.insert("help.git.status".into(), "git status | st ## Pretty-printed repo status".into());

    m.insert("help.git.branch".into(), "git branch | b ## List branches (current highlighted)".into());

    m.insert("help.git.log".into(), "git log | l ## Colored recent commit log".into());

    m.insert("help.git.diff".into(), "git diff ## Show working tree diff".into());

    m.insert("help.git.passthrough".into(), "git <any> ## All other git subcommands pass through".into());

    m.insert("help.git.example1".into(), "git status ## Show branch + staged/unstaged/untracked".into());

    m.insert("help.git.example2".into(), "git branch -a ## List all branches (passthrough)".into());

    m.insert("help.git.example3".into(), "git commit -am \"msg\" ## Normal git flow (passthrough)".into());

    m.insert("help.config.git".into(), "config git ON|OFF ## Enable/disable git integration (prompt + symbols)".into());

    m.insert("help.config.dir_view".into(), "config dir_view ON|OFF ## Show/hide directory tree in shell".into());

    m.insert("git.no_git_binary".into(), "Git is not installed. Run `ntpkg install git` to install it.".into());

    m.insert("git.not_repo".into(), "Not a git repository (or any of the parent directories).".into());

    m.insert("git.executing".into(), "Executing: {}".into());

    m.insert("git.completed".into(), "git command completed.".into());

    m.insert("git.exit_code".into(), "Command exited with code: {}".into());

    m.insert("git.terminated".into(), "Command terminated.".into());

    m.insert("git.failed".into(), "Failed to execute command: {}".into());

    m.insert("git.checking_status".into(), "Checking status".into());

    m.insert("git.status_failed".into(), "Could not read git status.".into());

    m.insert("git.branch_header".into(), "On branch".into());

    m.insert("git.upstream".into(), "tracking {}".into());

    m.insert("git.ahead".into(), "ahead {}".into());

    m.insert("git.behind".into(), "behind {}".into());

    m.insert("git.detached_header".into(), "Detached HEAD".into());

    m.insert("git.section_conflicts".into(), "Unmerged paths:".into());

    m.insert("git.section_staged".into(), "Changes to be committed:".into());

    m.insert("git.section_unstaged".into(), "Changes not staged for commit:".into());

    m.insert("git.section_untracked".into(), "Untracked files:".into());

    m.insert("git.clean".into(), "Working tree clean.".into());

    m.insert("git.branch_failed".into(), "Could not list branches.".into());

    m.insert("git.log_failed".into(), "Could not read git log.".into());

    m.insert("git.log_empty".into(), "No commits found.".into());

    m.insert("git.help_title".into(), "GIT INTEGRATION:".into());

    m.insert("git.help_status".into(), "git status ## Show branch + staged/unstaged/untracked".into());

    m.insert("git.help_branch".into(), "git branch ## List branches".into());

    m.insert("git.help_log".into(), "git log ## Recent colored commits".into());

    m.insert("git.help_diff".into(), "git diff ## Working tree diff".into());

    m.insert("git.help_passthrough".into(), "git <any> ## Everything else passes to real git".into());

    m.insert("git.examples_header".into(), "EXAMPLES:".into());

    m.insert("git.help_example1".into(), "git status".into());

    m.insert("git.help_example2".into(), "git checkout main".into());

    m.insert("msg.setGit.status".into(), "Git integration is: {}".into());

    m.insert("msg.setGit.on".into(), "Git integration enabled.".into());

    m.insert("msg.setGit.off".into(), "Git integration disabled.".into());

    m.insert("msg.setGit.error".into(), "Usage: config git ON|OFF".into());

    m.insert("msg.setDV.status".into(), "Dir view is: {}".into());

    m.insert("msg.setDV.on".into(), "Dir view enabled.".into());

    m.insert("msg.setDV.off".into(), "Dir view disabled.".into());

    m.insert("msg.setDV.error".into(), "Usage: config dir_view ON|OFF".into());

    m.insert("cmd.tmprt.unknown_sub".into(), "Unknown tmprt subcommand: {}".into());

    m.insert("cmd.spec.other".into(), "Other".into());

    m.insert("cmd.spec.title".into(), "SYSTEM SPECIFICATIONS".into());

    m.insert("cmd.spec.cpu".into(), "CPU".into());

    m.insert("cmd.spec.vendor".into(), "Vendor".into());

    m.insert("cmd.spec.generation".into(), "Generation".into());

    m.insert("cmd.spec.model".into(), "Model".into());

    m.insert("cmd.spec.cores".into(), "Cores / Threads".into());

    m.insert("cmd.spec.gpu".into(), "GPU".into());

    m.insert("cmd.spec.ram".into(), "RAM".into());

    m.insert("cmd.spec.storage".into(), "Storage".into());

    m.insert("cmd.spec.total".into(), "Total".into());

    m.insert("cmd.spec.os".into(), "OS".into());

    m.insert("cmd.spec.manufacturer".into(), "Manufacturer".into());

    m.insert("cmd.spec.computer_model".into(), "Computer Model".into());

    m.insert("cmd.spec.serial".into(), "Serial Number".into());

    m.insert("cmd.spec.hostname".into(), "Hostname".into());

    m.insert("cmd.spec.kernel".into(), "Kernel".into());

    m.insert("cmd.spec.uptime".into(), "Uptime".into());

    m.insert("cmd.spec.unknown".into(), "Unknown".into());

    m.insert("cmd.spec.custom".into(), "Custom".into());

    m.insert("cmd.tmprt.usage_title".into(), "Usage:".into());

    m.insert("cmd.tmprt.usage_com".into(), "  tmprt com [secs]        Live hardware temperature monitor".into());

    m.insert("cmd.tmprt.usage_weather".into(), "  tmprt weather [city]    Current weather via OpenWeather API".into());

    m.insert("cmd.tmprt.usage_key".into(), "  tmprt key [API_KEY]     Save/display the OpenWeather API key".into());

    m.insert("cmd.tmprt.usage_city".into(), "  tmprt city [name]       Save/display the default weather city".into());

    m.insert("cmd.tmprt.usage_sensors".into(), "  tmprt sensors <sub>     Manage hardware sensor backends".into());

    m.insert("cmd.tmprt.examples_title".into(), "Examples:".into());

    m.insert("cmd.tmprt.example1".into(), "tmprt com".into());

    m.insert("cmd.tmprt.example2".into(), "tmprt com 5".into());

    m.insert("cmd.tmprt.example3".into(), "tmprt weather Bangkok".into());

    m.insert("cmd.tmprt.example4".into(), "tmprt key 1234567890abcdef".into());

    m.insert("cmd.tmprt.com.no_sensors".into(), "No temperature sensors detected.".into());

    m.insert("cmd.tmprt.com.no_sensors_hint".into(), "On Windows this may require administrator rights; on Linux a hwmon/thermal sensor must be present.".into());

    m.insert("cmd.tmprt.com.footer".into(), " Press q to quit".into());

    m.insert("cmd.tmprt.com.lhm_missing".into(), "No CPU temperature backend found (ACPI exposes no sensor on this machine).".into());

    m.insert("cmd.tmprt.com.lhm_prompt".into(), "Install LibreHardwareMonitorLib (auto-downloaded, ~7 MB) for CPU/GPU/board/storage temperatures?".into());

    m.insert("cmd.tmprt.com.lhm_confirm".into(), "Install now? [Y/n]".into());

    m.insert("cmd.tmprt.com.lhm_installing".into(), "Downloading and installing LibreHardwareMonitorLib...".into());

    m.insert("cmd.tmprt.com.lhm_installed".into(), "LibreHardwareMonitorLib installed.".into());

    m.insert("cmd.tmprt.com.lhm_install_failed".into(), "LibreHardwareMonitorLib install failed: {}".into());

    m.insert("cmd.tmprt.com.admin_hint".into(), "CPU temps need the PawnIO driver and an elevated process. Run 'mode admin' to relaunch ntc elevated.".into());
    m.insert("cmd.tmprt.com.pawnio_hint".into(), "CPU temps need the PawnIO driver. Run 'tmprt sensors install' (needs Administrator).".into());

    m.insert("cmd.tmprt.weather.no_key".into(), "No OpenWeather API key found.".into());

    m.insert("cmd.tmprt.weather.no_key_hint".into(), "Set one with: tmprt key <API_KEY>  (or export OPENWEATHER_API_KEY)".into());

    m.insert("cmd.tmprt.weather.no_city".into(), "No city given and no default city configured.".into());

    m.insert("cmd.tmprt.weather.no_city_hint".into(), "Usage: tmprt weather <city>   or set a default: tmprt city <name>".into());

    m.insert("cmd.tmprt.weather.request_failed".into(), "Weather request failed: {}".into());

    m.insert("cmd.tmprt.weather.read_failed".into(), "Failed to read weather response: {}".into());

    m.insert("cmd.tmprt.weather.bad_key".into(), "OpenWeather rejected the API key (HTTP 401).".into());

    m.insert("cmd.tmprt.weather.city_not_found".into(), "City not found: {}  (HTTP 404)".into());

    m.insert("cmd.tmprt.weather.http_error".into(), "OpenWeather error (HTTP {}): {}".into());

    m.insert("cmd.tmprt.weather.parse_failed".into(), "Failed to parse weather data: {}".into());

    m.insert("cmd.tmprt.weather.title".into(), "Weather:".into());

    m.insert("cmd.tmprt.weather.condition".into(), "Condition".into());

    m.insert("cmd.tmprt.weather.temp".into(), "Temperature".into());

    m.insert("cmd.tmprt.weather.feels".into(), "feels like".into());

    m.insert("cmd.tmprt.weather.range".into(), "Min / Max".into());

    m.insert("cmd.tmprt.weather.humidity".into(), "Humidity".into());

    m.insert("cmd.tmprt.weather.pressure".into(), "Pressure".into());

    m.insert("cmd.tmprt.weather.wind".into(), "Wind".into());

    m.insert("cmd.tmprt.weather.visibility".into(), "Visibility".into());

    m.insert("cmd.tmprt.weather.clouds".into(), "Clouds".into());

    m.insert("cmd.tmprt.weather.sunrise".into(), "Sunrise".into());

    m.insert("cmd.tmprt.weather.sunset".into(), "Sunset".into());

    m.insert("cmd.tmprt.weather.coords".into(), "Coords".into());

    m.insert("cmd.tmprt.key.env_source".into(), "OpenWeather API key (from OPENWEATHER_API_KEY env): {}".into());

    m.insert("cmd.tmprt.key.status".into(), "OpenWeather API key: {}".into());

    m.insert("cmd.tmprt.key.not_set".into(), "No OpenWeather API key is configured.".into());

    m.insert("cmd.tmprt.key.saved".into(), "OpenWeather API key saved: {}".into());

    m.insert("cmd.tmprt.city.status".into(), "Default weather city: {}".into());

    m.insert("cmd.tmprt.city.not_set".into(), "No default weather city is configured.".into());

    m.insert("cmd.tmprt.city.saved".into(), "Default weather city saved: {}".into());
    m.insert("cmd.tmprt.sensors.help_title".into(), "tmprt sensors subcommands:".into());
    m.insert("cmd.tmprt.sensors.help_status".into(), "  tmprt sensors status        Show which sensor backends are available".into());
    m.insert("cmd.tmprt.sensors.help_install".into(), "  tmprt sensors install       Download LibreHardwareMonitorLib (Windows)".into());
    m.insert("cmd.tmprt.sensors.help_uninstall".into(), "  tmprt sensors uninstall     Remove the downloaded LibreHardwareMonitorLib files".into());
    m.insert("cmd.tmprt.sensors.status_title".into(), "Hardware sensor backends:".into());
    m.insert("cmd.tmprt.sensors.src_acpi".into(), "ACPI/hwmon (sysinfo)".into());
    m.insert("cmd.tmprt.sensors.src_nvidia".into(), "NVIDIA (nvidia-smi)".into());
    m.insert("cmd.tmprt.sensors.src_lhm".into(), "LibreHardwareMonitor".into());
    m.insert("cmd.tmprt.sensors.src_unavailable".into(), "unavailable".into());
    m.insert("cmd.tmprt.sensors.src_installed".into(), "installed".into());
    m.insert("cmd.tmprt.sensors.src_not_installed".into(), "not installed".into());
    m.insert("cmd.tmprt.sensors.status_hint".into(), "Run 'tmprt sensors install' to enable LibreHardwareMonitor-based sensors (Windows).".into());
    m.insert("cmd.tmprt.sensors.installed".into(), "LibreHardwareMonitorLib installed.".into());
    m.insert("cmd.tmprt.sensors.install_failed".into(), "LibreHardwareMonitorLib install failed: {}".into());
    m.insert("cmd.tmprt.sensors.uninstalled".into(), "LibreHardwareMonitorLib files removed.".into());
    m.insert("cmd.tmprt.sensors.uninstall_failed".into(), "Failed to remove LibreHardwareMonitorLib files: {}".into());
    m.insert("cmd.tmprt.sensors.not_installed".into(), "LibreHardwareMonitorLib is not installed.".into());
    m.insert("cmd.tmprt.sensors.windows_only".into(), "LibreHardwareMonitor is only supported on Windows.".into());
    m.insert("cmd.tmprt.sensors.downloading".into(), "Downloading LibreHardwareMonitorLib".into());

    m.insert("cmd.tmprt.sensors.elevation".into(), "Elevation".into());

    m.insert("cmd.tmprt.sensors.admin_yes".into(), "Administrator".into());

    m.insert("cmd.tmprt.sensors.admin_no".into(), "standard user".into());

    m.insert("cmd.tmprt.sensors.need_admin".into(), "CPU temps via LHM need an elevated process (PawnIO driver). Run 'mode admin' to relaunch ntc elevated.".into());
    m.insert("cmd.tmprt.sensors.src_pawnio".into(), "PawnIO driver".into());
    m.insert("cmd.tmprt.sensors.pawnio_missing".into(), "The PawnIO driver is not installed - CPU temps will stay '--'. Run 'tmprt sensors install' to install it (needs Administrator).".into());
    m.insert("cmd.tmprt.sensors.pawnio_present".into(), "PawnIO driver is already installed.".into());
    m.insert("cmd.tmprt.sensors.pawnio_elevating".into(), "Elevating to install the PawnIO driver (accept the UAC prompt)...".into());
    m.insert("cmd.tmprt.sensors.pawnio_downloading".into(), "Downloading PawnIO driver".into());
    m.insert("cmd.tmprt.sensors.pawnio_installed".into(), "PawnIO driver installed.".into());
    m.insert("cmd.tmprt.sensors.pawnio_reboot".into(), "PawnIO driver installed - restart your PC to activate it.".into());
    m.insert("cmd.tmprt.sensors.pawnio_failed".into(), "PawnIO driver install failed: {}".into());

    m.insert("cmd.tmprt.sensors.help_debug".into(), "  tmprt sensors debug         Dump detected hardware + sensor values (diagnostics)".into());

    m.insert("cmd.tmprt.sensors.debug_title".into(), "Sensor diagnostics (LibreHardwareMonitor):".into());

    m.insert("cmd.tmprt.sensors.debug_none".into(), "LibreHardwareMonitor is not installed - run 'tmprt sensors install' first.".into());

    m

}

