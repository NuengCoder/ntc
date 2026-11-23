use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cp.title".into(), "CP COMMAND:".into());

    m.insert(
        "help.cp.usage".into(),
        "cp <src|@tp|pattern> <dst|@tp> [-f] ## Copy file or dir (pattern src needs a dir dst)"
            .into(),
    );

    m.insert(
        "help.cp.example1".into(),
        "cp file.txt backup.txt ## Copy a file".into(),
    );
}
