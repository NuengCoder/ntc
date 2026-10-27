use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.view.care".into(),
        "view -c, --care ## Show tree with care everything".into(),
    );

    m.insert(
        "help.view.cp".into(),
        "view -cp, --copy ## Show tree and copy to clipboard".into(),
    );

    m.insert(
        "help.view.file_only".into(),
        "view -f, --file ## Show files only (directories pruned)".into(),
    );

    m.insert(
        "help.view.dir_only".into(),
        "view -dir, --directory ## Show directories only (files pruned)".into(),
    );

    m.insert(
        "help.view.depth".into(),
        "view -d, --depth <n> ## Show tree with custom depth".into(),
    );

    m.insert(
        "help.view.dirsize".into(),
        "view -ds, --dirsize [min-size] ## Show dir sizes; with value, hide smaller dirs".into(),
    );

    m.insert(
        "help.view.head".into(),
        "view --head <n> ## Show only first n tree lines".into(),
    );

    m.insert(
        "help.view.tail".into(),
        "view --tail <n> ## Show only last n tree lines".into(),
    );

    m.insert(
        "help.view.sort".into(),
        "view --sort size|char|ext ## Sort children (dirs stay first)".into(),
    );

    m.insert(
        "help.view.ext".into(),
        "view -ext, --ext <ext...> ## Show tree with only matching extensions".into(),
    );

    m.insert(
        "help.view.filesize".into(),
        "view -fs, --filesize [min-size] ## Show file sizes; with value, hide smaller files".into(),
    );

    m.insert(
        "help.view.path".into(),
        "view -p, --path <path|@tp|@tp/sub> ## Show tree of a path or teleport path".into(),
    );

    m.insert(
        "help.view.size".into(),
        "view -s, --size [min-size] ## Show sizes; with value (10KB, 1.5MB), hide smaller".into(),
    );

    m.insert(
        "help.view.time".into(),
        "view --time, -t ## Show tree with execution time".into(),
    );

    m.insert("help.view.title".into(), "VIEW COMMAND:".into());

    m.insert(
        "help.view.tree".into(),
        "view [<dir|file|@tp|@tp/file|@tp/dir/>] ## Show tree or file content".into(),
    );

    m.insert(
        "help.view.example1".into(),
        "view ## Show directory tree".into(),
    );

    m.insert(
        "help.view.example2".into(),
        "view --size ## Show tree with sizes".into(),
    );

    m.insert(
        "help.view.example3".into(),
        "view -d 2 ## Show tree with depth 2".into(),
    );

    m.insert(
        "help.view.example4".into(),
        "view -cp ## Show tree and copy to clipboard".into(),
    );

    m.insert(
        "help.view.example5".into(),
        "view --time ## Show tree with execution time".into(),
    );

    m.insert(
        "help.view.example6".into(),
        "view -p C:\\code\\project ## Show tree of a specific path".into(),
    );

    m.insert(
        "help.view.example7".into(),
        "view @web ## Show tree of a teleport savepoint".into(),
    );

    m.insert(
        "help.view.example8".into(),
        "view -s 10MB ## Only files/dirs at least 10MB".into(),
    );

    m.insert(
        "help.view.example9".into(),
        "view --sort size --head 20 ## 20 largest entries first".into(),
    );

    m.insert("help.view.file_usage".into(), "view <file|@tp/file|@tp/dirs/file> [--head N|--tail N|--betw A:B] ## Display file content (same as txtf)".into());

    m.insert(
        "help.view.example10".into(),
        "view main.rs --tail 30 ## Last 30 lines of main.rs".into(),
    );

    m.insert(
        "help.view.example11".into(),
        "view main.rs --betw 10:20 ## Lines 10-20 of main.rs".into(),
    );

    m.insert(
        "help.view.example12".into(),
        "view @docs/readme.md ## Display a teleport savepoint file".into(),
    );
}
