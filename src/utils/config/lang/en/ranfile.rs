use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "ranfile.available_title".into(),
        "Available targets:".into(),
    );

    m.insert("ranfile.depends_on".into(), " ─▶ depends on: ".into());

    m.insert(
        "ranfile.runner.hint".into(),
        "Run {} to see available targets.".into(),
    );
}
