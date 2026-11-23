use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tutorial.title".into(), "TUTORIAL COMMAND:".into());

    m.insert(
        "help.tutorial.usage".into(),
        "tutorial [lesson] ## Guided hands-on lessons in a sandbox (tutorial list shows them all)"
            .into(),
    );

    m.insert(
        "help.tutorial.example1".into(),
        "tutorial ## Start interactive tutorial".into(),
    );
}
