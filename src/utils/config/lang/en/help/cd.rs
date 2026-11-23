use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cd.title".into(), "CD COMMAND:".into());

    m.insert(
        "help.cd.usage".into(),
        "cd <path> ## Navigate to a directory".into(),
    );

    m.insert(
        "help.cd.cd_tp".into(),
        "cd to <tp_name> ## Teleport to a saved teleport point".into(),
    );

    m.insert(
        "help.cd.example1".into(),
        "cd src ## Navigate to the src directory".into(),
    );

    m.insert(
        "help.cd.example2".into(),
        "cd to web ## Teleport to 'web' savepoint".into(),
    );
}
