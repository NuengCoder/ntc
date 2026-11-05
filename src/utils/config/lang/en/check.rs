use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "check.help".into(),
        " check [file] - interpret {} and report every problem".into(),
    );

    m.insert("check.hint".into(), "hint".into());

    m.insert("check.no_files".into(), "No {} file in {}".into());

    m.insert("check.not_found".into(), "{} not found".into());

    m.insert("check.ok".into(), "{}: no problem".into());

    m.insert("check.read_fail".into(), "Cannot read {}: {}".into());

    m.insert("check.summary".into(), "{} problem(s) in {}".into());

    m.insert("check.title".into(), "Check {}".into());
}
