use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rev.title".into(), "REV COMMAND:".into());

    m.insert(
        "help.rev.usage".into(),
        "rev [-u] [file] ## Reverse each line's characters, or flip the line order with -u".into(),
    );

    m.insert(
        "help.rev.example1".into(),
        "rev notes.txt ## Reverse characters on each line".into(),
    );

    m.insert(
        "help.rev.example2".into(),
        "rev -u notes.txt ## Upside down: last line first, first line last".into(),
    );
}
