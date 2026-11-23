use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.spec.title".into(), "SPEC COMMAND:".into());

    m.insert(
        "help.spec.usage".into(),
        "spec ## Show computer/system specifications (CPU, GPU, RAM, storage, OS, serial)".into(),
    );

    m.insert(
        "help.spec.example1".into(),
        "spec ## Show system specifications".into(),
    );
}
