use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mv.title".into(), "MV COMMAND:".into());

    m.insert(
        "help.mv.usage".into(),
        "mv <src|@tp|pattern> <dst|@tp> [-f] ## Move/rename file or dir (pattern src needs a dir dst)".into(),
    );

    m.insert(
        "help.mv.example1".into(),
        "mv old.txt new.txt ## Rename a file".into(),
    );
}
