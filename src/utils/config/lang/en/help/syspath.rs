use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.syspath.title".into(), "SYSPATH COMMAND:".into());

    m.insert(
        "help.syspath.show".into(),
        "syspath show ## Show system-level PATH entries".into(),
    );

    m.insert(
        "help.syspath.add".into(),
        "syspath add <path> ## Add directory to system PATH".into(),
    );

    m.insert(
        "help.syspath.add_cwd".into(),
        "syspath add --cwd | --here ## Add current directory to system PATH".into(),
    );

    m.insert(
        "help.syspath.rm".into(),
        "syspath rm <path-or-number> ## Remove from system PATH (by path or index)".into(),
    );

    m.insert(
        "help.syspath.move".into(),
        "syspath move <n> <m> ## Move entry #n to position #m".into(),
    );

    m.insert(
        "help.syspath.example1".into(),
        "syspath show ## Show system PATH entries".into(),
    );

    m.insert(
        "help.syspath.example2".into(),
        "syspath add /usr/local/bin ## Add to system PATH".into(),
    );

    m.insert(
        "help.syspath.example3".into(),
        "syspath move 3 1 ## Move entry to position 1".into(),
    );
}
