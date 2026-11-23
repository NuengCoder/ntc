use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.sort.title".into(), "SORT COMMAND:".into());

    m.insert(
        "help.sort.usage".into(),
        "sort [-r] [-n] [-u] [-f] [file] ## Sort lines (reverse/numeric/unique/ignore-case)".into(),
    );

    m.insert(
        "help.sort.example1".into(),
        "gs error | sort -u | head 10 ## Unique sorted matches, first 10".into(),
    );
}
