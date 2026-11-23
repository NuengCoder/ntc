use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.doctor.title".into(), "DOCTOR COMMANDS:".into());

    m.insert(
        "help.doctor.usage".into(),
        "doctor ## Check config, PATH, ranfile and ntpkg for problems".into(),
    );

    m.insert(
        "help.doctor.example1".into(),
        "doctor ## Read-only report; nothing is changed".into(),
    );
}
