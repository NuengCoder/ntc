use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fileops.cp".into(),
        "cp <src|@tp|pattern> <dst|@tp> [-f] ## Copy file or dir (pattern src needs a dir dst)"
            .into(),
    );

    m.insert(
        "help.fileops.mkd".into(),
        "mkd <dir>[, ...] ## Create a new directory".into(),
    );

    m.insert(
        "help.fileops.mkf".into(),
        "mkf <file>[, ...] ## Create a new file".into(),
    );

    m.insert(
        "help.fileops.rmd".into(),
        "rmd <dir>[, ...] ## Delete directories; wildcards delete every match after confirmation"
            .into(),
    );

    m.insert(
        "help.fileops.rmf".into(),
        "rmf <file>[, ...] ## Delete a file (with confirmation)".into(),
    );

    m.insert(
        "help.fileops.rnmd".into(),
        "rnmd <old> <new> ## Rename a directory".into(),
    );

    m.insert(
        "help.fileops.rnmf".into(),
        "rnmf <old> <new> ## Rename a file".into(),
    );

    m.insert("help.fileops.mv".into(), "mv <src|@tp|pattern> <dst|@tp> [-f] ## Move/rename file or dir (pattern src needs a dir dst)".into());

    m.insert("help.fileops.title".into(), "FILE OPERATIONS:".into());

    m.insert(
        "help.fileops.example1".into(),
        "mkf index.html ## Create a new file".into(),
    );

    m.insert(
        "help.fileops.example2".into(),
        "mkd src/components ## Create directories".into(),
    );

    m.insert(
        "help.fileops.example3".into(),
        "cp file.txt backup.txt ## Copy a file".into(),
    );
}
