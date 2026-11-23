use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tp.title".into(), "TELEPORT COMMANDS:".into());

    m.insert("help.tp.menu".into(), "tp ## Interactive menu".into());

    m.insert(
        "help.tp.add".into(),
        "tp add <name> ## Save current directory".into(),
    );

    m.insert(
        "help.tp.add_path".into(),
        "tp add <name> <path> ## Save specific path".into(),
    );

    m.insert(
        "help.tp.jump".into(),
        "tp jump <name> ## Teleport to saved location".into(),
    );

    m.insert(
        "help.tp.jump_num".into(),
        "tp jump <number> ## Teleport by number".into(),
    );

    m.insert(
        "help.tp.list".into(),
        "tp list ## List all teleport points".into(),
    );

    m.insert(
        "help.tp.info".into(),
        "tp info <name> ## Show savepoint details".into(),
    );

    m.insert(
        "help.tp.rm".into(),
        "tp rm <name> ## Remove teleport point".into(),
    );

    m.insert(
        "help.tp.rnm".into(),
        "tp rnm <old> to <new> ## Rename a savepoint".into(),
    );

    m.insert(
        "help.tp.cls".into(),
        "tp cls ## Clear ALL savepoints".into(),
    );

    m.insert(
        "help.tp.at".into(),
        "@<name> ## Quick teleport shortcut".into(),
    );

    m.insert(
        "help.tp.example1".into(),
        "tp add web ## Save current directory as 'web'".into(),
    );

    m.insert(
        "help.tp.example2".into(),
        "tp add docs ~/Documents ## Save specific path".into(),
    );

    m.insert(
        "help.tp.example3".into(),
        "tp jump web ## Teleport to 'web'".into(),
    );

    m.insert(
        "help.tp.example4".into(),
        "tp list ## List all savepoints".into(),
    );

    m.insert(
        "help.tp.example5".into(),
        "@web ## Quick teleport shortcut".into(),
    );
}
