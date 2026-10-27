use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("interactive.cancelled".into(), "Cancelled.".into());

    m.insert("interactive.invalid_input".into(), "Invalid input.".into());

    m.insert(
        "interactive.invalid_selection".into(),
        "Invalid selection.".into(),
    );

    m.insert("interactive.no_items".into(), "(no items)".into());
}
