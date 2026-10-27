use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tl.dir".into(), "tl dir <path> [-s <-n|-l|-r>] ## Show supported extensions and total lines in directory (sortable)".into());

    m.insert("help.tl.ext".into(), "tl ext <ext> [-s <-n|-l|-r>] ## Show all <ext> files and their total lines (respects igcare, sortable)".into());

    m.insert(
        "help.tl.file".into(),
        "tl file <path> ## Count lines in a single file".into(),
    );

    m.insert("help.tl.sort".into(), "tl ext|dir <target> -s <-n|-l|-r> ## Sort: -n normal, -l by line (small to big), -r reverse (big to small)".into());

    m.insert("help.tl.title".into(), "TOTAL LINE COMMANDS:".into());

    m.insert(
        "help.tl.example1".into(),
        "tl file main.rs ## Count lines in main.rs".into(),
    );

    m.insert(
        "help.tl.example2".into(),
        "tl ext .rs ## Count lines in all .rs files".into(),
    );

    m.insert(
        "help.tl.example3".into(),
        "tl dir . ## Count lines in current directory".into(),
    );

    m.insert(
        "help.tl.example4".into(),
        "tl ext .rs -s -r ## List .rs files sorted by lines, big to small".into(),
    );
}
