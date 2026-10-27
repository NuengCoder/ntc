use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.viewf.title".into(),
        "VIEWF COMMAND (files only):".into(),
    );

    m.insert(
        "help.viewf.tree".into(),
        "viewf [<dir|@tp|@tp/dir/>] ## Show a files-only tree at depth 1".into(),
    );

    m.insert("help.viewf.file_usage".into(), "viewf <file|@tp/file|@tp/dirs/file> [--head N|--tail N|--betw A:B] ## Display file content (same as txtf)".into());

    m.insert(
        "help.viewf.size".into(),
        "viewf -s, --size [min-size] ## Show file sizes; with value, hide smaller files".into(),
    );

    m.insert(
        "help.viewf.sort".into(),
        "viewf --sort size|char|ext ## Sort file entries".into(),
    );

    m.insert(
        "help.viewf.ext".into(),
        "viewf -e, --ext <ext...> ## Show only matching extensions".into(),
    );

    m.insert(
        "help.viewf.care".into(),
        "viewf -c, --care ## Count everything when sizing".into(),
    );

    m.insert(
        "help.viewf.time".into(),
        "viewf -t, --time ## Show execution time".into(),
    );

    m.insert(
        "help.viewf.copy".into(),
        "viewf -cp, --copy ## Copy the file list to clipboard".into(),
    );

    m.insert(
        "help.viewf.example1".into(),
        "viewf ## List files in the current directory".into(),
    );

    m.insert(
        "help.viewf.example2".into(),
        "viewf src ## List files directly inside src".into(),
    );

    m.insert(
        "help.viewf.example3".into(),
        "viewf -e rs,txt --sort size ## Files by extension, largest first".into(),
    );

    m.insert(
        "help.viewf.example4".into(),
        "viewf @docs/readme.md --head 20 ## Display a teleport file".into(),
    );
}
