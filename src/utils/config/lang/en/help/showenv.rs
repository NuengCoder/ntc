use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showenv.title".into(), "SHOWENV COMMAND:".into());

    m.insert(
        "help.showenv.usage".into(),
        "showenv ## Show environment variables (first 5 PATH entries)".into(),
    );

    m.insert(
        "help.showenv.all".into(),
        "showenv --all ## Show all PATH entries".into(),
    );

    m.insert(
        "help.showenv.depth".into(),
        "showenv -d <n> ## Show first n PATH entries".into(),
    );

    m.insert(
        "help.showenv.example1".into(),
        "showenv --all ## Show every PATH entry".into(),
    );
}
