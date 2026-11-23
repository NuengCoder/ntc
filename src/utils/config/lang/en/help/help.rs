use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.help.title".into(), "HELP COMMAND:".into());

    m.insert("help.help.usage".into(), "help ## Show this help".into());

    m.insert(
        "help.help.example1".into(),
        "help cd ## Show one command's help".into(),
    );
    m.insert(
        "help.help.command".into(),
        "help <command> ## Show one command's section".into(),
    );
    m.insert(
        "help.help.all".into(),
        "help --all ## Print every command section".into(),
    );
}
