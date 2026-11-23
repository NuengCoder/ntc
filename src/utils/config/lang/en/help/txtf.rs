use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtf.title".into(), "TXTF COMMAND:".into());

    m.insert(
        "help.txtf.usage".into(),
        "txtf [--head N|--tail N|--betw A:B] [--line|--no-line] [file|dir|@tp/file|@tp/dir] ## Display file content or pick a file from a directory (--line/--no-line override the line-number setting)".into(),
    );

    m.insert(
        "help.txtf.example1".into(),
        "txtf main.rs --head 30 ## First 30 lines of main.rs".into(),
    );

    m.insert(
        "help.txtf.example2".into(),
        "txtf --betw 10:20 main.rs ## Lines 10-20 of main.rs".into(),
    );

    m.insert(
        "help.txtf.example3".into(),
        "txtf @docs/notes.txt ## Display a teleport savepoint file".into(),
    );

    m.insert(
        "help.txtf.example4".into(),
        "txtf src ## Pick a supported file from src to display".into(),
    );
}
