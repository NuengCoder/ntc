use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.clip.title".into(), "CLIPBOARD COMMANDS:".into());

    m.insert(
        "help.clip.show".into(),
        "clip ## Show clipboard history".into(),
    );

    m.insert(
        "help.clip.list".into(),
        "clip list ## Show clipboard history".into(),
    );

    m.insert(
        "help.clip.select".into(),
        "clip select <N> ## Copy entry N back to clipboard".into(),
    );

    m.insert(
        "help.clip.clear".into(),
        "clip clear ## Clear clipboard history".into(),
    );

    m.insert(
        "help.clip.example1".into(),
        "clip ## Show clipboard history".into(),
    );

    m.insert(
        "help.clip.example2".into(),
        "clip select 1 ## Copy entry #1 back to clipboard".into(),
    );
}
