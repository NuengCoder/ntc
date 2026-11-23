use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.run.title".into(), "RUN COMMAND:".into());

    m.insert(
        "help.run.usage".into(),
        "run <alias> ## Execute alias with 'run'".into(),
    );

    m.insert(
        "help.run.direct".into(),
        "<alias> ## Execute alias directly".into(),
    );

    m.insert(
        "help.run.param".into(),
        "<alias>(<arg>) ## Execute parameterised alias".into(),
    );

    m.insert(
        "help.run.example1".into(),
        "build ## Run alias directly".into(),
    );
}
