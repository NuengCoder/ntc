use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.usrpath.title".into(),
        "PATH MANAGER:".into(),
    );

    m.insert(
        "help.usrpath.show".into(),
        "usrpath show ## Show user-level PATH entries".into(),
    );

    m.insert(
        "help.usrpath.add".into(),
        "usrpath add <path> ## Add directory to user PATH".into(),
    );

    m.insert(
        "help.usrpath.add_cwd".into(),
        "usrpath add --cwd | --here ## Add current directory to user PATH".into(),
    );

    m.insert(
        "help.usrpath.rm".into(),
        "usrpath rm <path-or-number> ## Remove from user PATH (by path or index)".into(),
    );

    m.insert(
        "help.usrpath.move".into(),
        "usrpath move <n> <m> ## Move entry #n to position #m".into(),
    );

    m.insert(
        "help.usrpath.example1".into(),
        "usrpath show ## Show user PATH entries".into(),
    );

    m.insert(
        "help.usrpath.example2".into(),
        "usrpath add ~/bin ## Add to user PATH".into(),
    );

    m.insert(
        "help.usrpath.example3".into(),
        "usrpath rm 2 ## Remove entry #2 from user PATH".into(),
    );
}
