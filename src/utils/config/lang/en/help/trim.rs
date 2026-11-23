use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trim.title".into(), "TRIM COMMAND:".into());

    m.insert(
        "help.trim.usage".into(),
        "trim <pattern> [text] ## Remove every occurrence of <pattern> (text or piped input)"
            .into(),
    );

    m.insert(
        "help.trim.example1".into(),
        "echo nice ... bro ... | trim \" ...\" ## Strip a pattern from piped text (inline: trim \" ...\" \"nice ... bro ...\")".into(),
    );
}
