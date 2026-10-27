use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.teleport.add".into(),
        "tp add <name> ## Save current directory".into(),
    );

    m.insert(
        "help.teleport.add_path".into(),
        "tp add <name> <path> ## Save specific path".into(),
    );

    m.insert(
        "help.teleport.at".into(),
        "@<name> ## Quick teleport shortcut".into(),
    );

    m.insert(
        "help.teleport.cls".into(),
        "tp cls ## Clear ALL savepoints".into(),
    );

    m.insert(
        "help.teleport.info".into(),
        "tp info <name> ## Show savepoint details".into(),
    );

    m.insert(
        "help.teleport.jump".into(),
        "tp jump <name> ## Teleport to saved location".into(),
    );

    m.insert(
        "help.teleport.jump_num".into(),
        "tp jump <number> ## Teleport by number".into(),
    );

    m.insert(
        "help.teleport.list".into(),
        "tp list ## List all teleport points".into(),
    );

    m.insert("help.teleport.menu".into(), "tp ## Interactive menu".into());

    m.insert(
        "help.teleport.rm".into(),
        "tp rm <name> ## Remove teleport point".into(),
    );

    m.insert(
        "help.teleport.rnm".into(),
        "tp rnm <old> to <new> ## Rename a savepoint".into(),
    );

    m.insert("help.teleport.title".into(), "TELEPORT COMMANDS:".into());

    m.insert(
        "help.teleport.tpb".into(),
        "tpb ## Teleport back to previous location (undo tp)".into(),
    );

    m.insert(
        "help.teleport.tpb_clear".into(),
        "tpb clear ## Clear teleport history".into(),
    );

    m.insert(
        "help.teleport.tpb_history".into(),
        "tpb history ## Show teleport navigation history".into(),
    );

    m.insert(
        "help.teleport.example1".into(),
        "tp add web ## Save current directory as 'web'".into(),
    );

    m.insert(
        "help.teleport.example2".into(),
        "tp add docs ~/Documents ## Save specific path".into(),
    );

    m.insert(
        "help.teleport.example3".into(),
        "tp jump web ## Teleport to 'web'".into(),
    );

    m.insert(
        "help.teleport.example4".into(),
        "tp list ## List all savepoints".into(),
    );

    m.insert(
        "help.teleport.example5".into(),
        "@web ## Quick teleport shortcut".into(),
    );
}
