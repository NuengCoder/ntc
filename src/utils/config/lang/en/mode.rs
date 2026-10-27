use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "mode.help_admin".into(),
        "  mode admin               Exit and re-launch as Administrator".into(),
    );

    m.insert(
        "mode.help_example_admin".into(),
        "  mode admin                (UAC prompt will appear)".into(),
    );

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_examples".into(), "Examples:".into());

    m.insert(
        "mode.help_normal".into(),
        "  mode normal              Exit and re-launch normally".into(),
    );

    m.insert("mode.help_usage".into(), "Usage:".into());
}
