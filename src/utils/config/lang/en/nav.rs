use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("nav.available_drives".into(), "Available drives:".into());

    m.insert("nav.cancelled".into(), "Cancelled.".into());

    m.insert(
        "nav.config_exists".into(),
        " \u{2713} Config file exists".into(),
    );

    m.insert(
        "nav.drive_prompt".into(),
        "Enter drive letter or number (or 'cancel'): ".into(),
    );

    m.insert("nav.gos_where".into(), "gos where?".into());
}
