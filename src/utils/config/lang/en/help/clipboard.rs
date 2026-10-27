use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.clipboard.clear".into(),
        "clip clear ## Clear clipboard history".into(),
    );

    m.insert(
        "help.clipboard.list".into(),
        "clip list ## Show clipboard history".into(),
    );

    m.insert(
        "help.clipboard.pathcp".into(),
        "pathcp ## Copy current directory path to clipboard".into(),
    );

    m.insert(
        "help.clipboard.select".into(),
        "clip select <N> ## Copy entry N back to clipboard".into(),
    );

    m.insert(
        "help.clipboard.show".into(),
        "clip ## Show clipboard history".into(),
    );

    m.insert("help.clipboard.title".into(), "CLIPBOARD COMMANDS:".into());

    m.insert(
        "help.clipboard.example1".into(),
        "clip ## Show clipboard history".into(),
    );

    m.insert(
        "help.clipboard.example2".into(),
        "clip select 1 ## Copy entry #1 back to clipboard".into(),
    );

    m.insert(
        "help.clipboard.example3".into(),
        "pathcp ## Copy current directory path".into(),
    );
}
