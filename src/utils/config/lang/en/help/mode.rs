use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.mode.admin".into(),
        "mode admin ## Exit and re-launch ntc as Administrator".into(),
    );

    m.insert(
        "help.mode.normal".into(),
        "mode normal ## Exit and re-launch ntc without elevation".into(),
    );

    m.insert("help.mode.title".into(), "WINDOWS MODE (Windows):".into());

    m.insert(
        "help.mode.example1".into(),
        "mode admin ## Relaunch as Administrator".into(),
    );
}
