use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.serve.title".into(), "SERVE COMMAND:".into());

    m.insert(
        "help.serve.service".into(),
        "serve <service> ## Pick a service (bare `serve` lists them)".into(),
    );

    m.insert(
        "help.serve.chess".into(),
        "serve chess [--online] ## Hotseat/AI/coach chess UI (--online: lobby + rooms)".into(),
    );

    m.insert(
        "help.serve.git".into(),
        "serve git [dir] ## Git GUI: status, diff, stage, commit, branches, tags, stash, push/pull"
            .into(),
    );

    m.insert(
        "help.serve.arc".into(),
        "serve arc [dir] ## Archive GUI: create, inspect and extract archives".into(),
    );

    m.insert(
        "help.serve.ndb".into(),
        "serve ndb ## SQLite browser: pick a database in this folder, browse schema/rows, optional --write".into(),
    );

    m.insert(
        "help.serve.theme".into(),
        "serve theme ## Theme GUI: edit, preview, save and apply themes".into(),
    );

    m.insert(
        "help.serve.fake".into(),
        "serve fake ## Fake-data GUI: types or saved groups, preview txt/json/csv/sql".into(),
    );

    m.insert(
        "help.serve.config".into(),
        "serve config ## Configuration GUI: settings and serve defaults (CLI editor stays at config serve)".into(),
    );

    m.insert(
        "help.serve.backup".into(),
        "serve backup [path] ## Backup GUI: create, diff, restore, undo, clear this folder".into(),
    );

    m.insert(
        "help.serve.example1".into(),
        "serve files --port 8080 ## Start files drive server on port 8080".into(),
    );
}
