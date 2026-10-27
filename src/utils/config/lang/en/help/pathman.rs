use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.pathman.syspath_add".into(),
        "syspath add <path> ## Add directory to system PATH".into(),
    );

    m.insert(
        "help.pathman.syspath_add_cwd".into(),
        "syspath add --cwd | --here ## Add current directory to system PATH".into(),
    );

    m.insert(
        "help.pathman.syspath_move".into(),
        "syspath move <n> <m> ## Move entry #n to position #m".into(),
    );

    m.insert(
        "help.pathman.syspath_rm".into(),
        "syspath rm <path-or-number> ## Remove from system PATH (by path or index)".into(),
    );

    m.insert(
        "help.pathman.syspath_show".into(),
        "syspath show ## Show system-level PATH entries".into(),
    );

    m.insert(
        "help.pathman.title".into(),
        "PATH MANAGER (Windows):".into(),
    );

    m.insert(
        "help.pathman.usrpath_add".into(),
        "usrpath add <path> ## Add directory to user PATH".into(),
    );

    m.insert(
        "help.pathman.usrpath_add_cwd".into(),
        "usrpath add --cwd | --here ## Add current directory to user PATH".into(),
    );

    m.insert(
        "help.pathman.usrpath_move".into(),
        "usrpath move <n> <m> ## Move entry #n to position #m".into(),
    );

    m.insert(
        "help.pathman.usrpath_rm".into(),
        "usrpath rm <path-or-number> ## Remove from user PATH (by path or index)".into(),
    );

    m.insert(
        "help.pathman.usrpath_show".into(),
        "usrpath show ## Show user-level PATH entries".into(),
    );

    m.insert(
        "help.pathman.example1".into(),
        "usrpath show ## Show user PATH entries".into(),
    );

    m.insert(
        "help.pathman.example2".into(),
        "usrpath add C:\\tools ## Add to user PATH".into(),
    );

    m.insert(
        "help.pathman.example3".into(),
        "usrpath rm 2 ## Remove entry #2 from user PATH".into(),
    );

    m.insert(
        "help.pathman.example4".into(),
        "syspath show ## Show system PATH entries".into(),
    );

    m.insert(
        "help.pathman.example5".into(),
        "syspath add C:\\bin ## Add to system PATH".into(),
    );

    m.insert(
        "help.pathman.example6".into(),
        "syspath move 3 1 ## Move entry to position 1".into(),
    );
}
