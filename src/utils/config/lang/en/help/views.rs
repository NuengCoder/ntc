use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.views.title".into(),
        "VIEWS COMMAND (short, flat listing):".into(),
    );

    m.insert(
        "help.views.usage".into(),
        "views [<dir|@tp|@tp/dir/>] ## List one directory level, flat like ls".into(),
    );

    m.insert(
        "help.views.size".into(),
        "views -s, --size [min-size] ## Show sizes; with value (1024, 10KB, 5MB), hide smaller"
            .into(),
    );

    m.insert(
        "help.views.sort".into(),
        "views --sort <name|ext|size> ## Sort entries (default: name)".into(),
    );

    m.insert(
        "help.views.newline".into(),
        "views -n ## One entry per line instead of the flowing groups".into(),
    );

    m.insert("help.views.note".into(), "No [Directory]/[ignored] tags - colours show the kind. Depth is fixed at 1 and sizes are always computed with --care semantics. Use --size to display sizes.".into());

    m.insert(
        "help.views.example1".into(),
        "views ## Flat listing of the current directory".into(),
    );

    m.insert(
        "help.views.example2".into(),
        "views src --size 1KB --sort size ## Entries >= 1KB, largest first, sizes shown".into(),
    );
}
