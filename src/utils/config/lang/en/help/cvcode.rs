use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cvcode.title".into(), "CVCODE COMMAND:".into());

    m.insert(
        "help.cvcode.usage".into(),
        "cvcode <from> <data> to <to> ## Convert between formats".into(),
    );

    m.insert(
        "help.cvcode.example1".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## Convert between formats".into(),
    );

    m.insert(
        "help.cvcode.example2".into(),
        "cvcode b64 \"aGVsbG8=\" to plaintext ## Decode to readable text".into(),
    );
}
