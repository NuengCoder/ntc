use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cvcode.formats".into(),
        "Formats: b2, b10, b64 (base64), hex, url".into(),
    );

    m.insert(
        "cvcode.missing_data".into(),
        "Missing data to convert".into(),
    );

    m.insert(
        "cvcode.missing_to".into(),
        "Missing target format after 'to'".into(),
    );

    m.insert(
        "cvcode.usage".into(),
        " cvcode <from> <data> to <to> [-o <file>]".into(),
    );

    m.insert(
        "cvcode.usage_detailed".into(),
        "Usage: cvcode <from> <data> to <to> [-o <file>]".into(),
    );
}
