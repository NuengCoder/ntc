use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.where.title".into(), "WHERE COMMAND:".into());

    m.insert(
        "help.where.usage".into(),
        "where [-a|--all] [<binary>] ## Show the ntc executable, or find a binary on PATH".into(),
    );

    m.insert(
        "help.where.example1".into(),
        "where -a ## Show every ntc config, session, package and vault path".into(),
    );

    m.insert(
        "help.where.example2".into(),
        "where git ## Show the first git on PATH (-a lists every match)".into(),
    );
}
