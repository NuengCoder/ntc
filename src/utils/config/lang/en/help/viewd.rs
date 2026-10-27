use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.viewd.title".into(),
        "VIEWD COMMAND (directories only):".into(),
    );

    m.insert(
        "help.viewd.usage".into(),
        "viewd [<dir|@tp|@tp/dir/>] [-d N] ## Show a directories-only tree".into(),
    );

    m.insert(
        "help.viewd.depth".into(),
        "viewd -d, --depth <n> ## Show tree with custom depth".into(),
    );

    m.insert(
        "help.viewd.size".into(),
        "viewd -s, --size [min-size] ## Show directory sizes; with value, hide smaller".into(),
    );

    m.insert(
        "help.viewd.care".into(),
        "viewd -c, --care ## Count everything when sizing".into(),
    );

    m.insert(
        "help.viewd.sort".into(),
        "viewd --sort size|char|ext ## Sort directory entries".into(),
    );

    m.insert(
        "help.viewd.time".into(),
        "viewd -t, --time ## Show execution time".into(),
    );

    m.insert(
        "help.viewd.copy".into(),
        "viewd -cp, --copy ## Copy the directory tree to clipboard".into(),
    );

    m.insert(
        "help.viewd.example1".into(),
        "viewd ## Show only directories in the current tree".into(),
    );

    m.insert(
        "help.viewd.example2".into(),
        "viewd -d 2 --size ## Two levels deep with directory sizes".into(),
    );

    m.insert(
        "help.viewd.example3".into(),
        "viewd --sort size ## Largest directories first".into(),
    );

    m.insert(
        "help.viewd.example4".into(),
        "viewd @docs/ ## Show a teleport savepoint's directories".into(),
    );
}
