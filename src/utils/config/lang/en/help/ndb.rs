use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ndb.title".into(), "NDB (DATABASE) COMMANDS:".into());

    m.insert(
        "help.ndb.launch".into(),
        "ndb ## Open the ndb database shell (.db browser) in the current directory".into(),
    );

    m.insert(
        "help.ndb.root_help".into(),
        "help ## Show every ndb command (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.root_view".into(),
        "view | ls ## List available .db files (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.root_interface".into(),
        "interface <file.db> ## Open a database: prompt becomes ndb [file.db]>".into(),
    );

    m.insert(
        "help.ndb.root_sql".into(),
        "sql <query> ## Run a read-only query once a database is open".into(),
    );

    m.insert(
        "help.ndb.root_exit".into(),
        "exit ## Close the database, then leave ndb".into(),
    );

    m.insert(
        "help.ndb.db_viewf".into(),
        "viewf | v ## Show the schema tree of tables and columns".into(),
    );

    m.insert(
        "help.ndb.db_tables".into(),
        "tables | t ## List table and view names".into(),
    );

    m.insert(
        "help.ndb.db_tablef".into(),
        "tablef <table> ## Display a table's rows (like txtf)".into(),
    );

    m.insert(
        "help.ndb.db_schema".into(),
        "schema <table> ## Show CREATE TABLE SQL and column details".into(),
    );

    m.insert(
        "help.ndb.db_sql".into(),
        "sql <query> ## Run a read-only query (-l N rows, --all)".into(),
    );

    m.insert(
        "help.ndb.db_write".into(),
        "write YES <stmt> ## Run one write, needs edit on (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.db_edit".into(),
        "edit on|off ## Allow or block writes (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.db_backup".into(),
        "backup ## Copy the .db file next to itself (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.more".into(),
        "more ## Show the next page of the last query (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.indexes".into(),
        "indexes [table] ## List indexes (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.info".into(),
        "info ## Show database file, engine and object counts (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.dbal".into(),
        "dbal ## Manage saved read-only queries: add|run|list|rm (inside ndb)".into(),
    );

    m.insert(
        "help.ndb.db_exit".into(),
        "exit ## Close the database and return to ndb [cwd]>".into(),
    );

    m.insert(
        "help.ndb.readonly".into(),
        "read-only ## writes are blocked".into(),
    );

    m.insert(
        "help.ndb.example1".into(),
        "ndb ## then: interface users.db".into(),
    );

    m.insert(
        "help.ndb.example2".into(),
        "ndb ## then: tablef users --all".into(),
    );
}
