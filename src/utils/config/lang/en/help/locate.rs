use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.locate.title".into(), "LOCATE COMMAND:".into());

    m.insert(
        "help.locate.usage".into(),
        "locate <pattern> [-d <n>] [--no-fuzzy] ## Search both files and directories".into(),
    );

    m.insert(
        "help.locate.example1".into(),
        "locate README ## Search files & dirs for 'README'".into(),
    );
}
