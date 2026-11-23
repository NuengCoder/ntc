use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmf.title".into(), "RMF COMMAND:".into());

    m.insert(
        "help.rmf.usage".into(),
        "rmf <file>[, ...] ## Delete a file (with confirmation)".into(),
    );

    m.insert(
        "help.rmf.example1".into(),
        "rmf old.log ## Delete a file".into(),
    );
}
