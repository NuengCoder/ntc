use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "Cancelled.".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. Cancel".into());

    m.insert("cmd.image.enter_number".into(), "Enter the number: ".into());

    m.insert(
        "cmd.image.no_ansi".into(),
        "This terminal has no ANSI colors; cannot display '{}' inline.".into(),
    );

    m.insert(
        "cmd.image.no_files".into(),
        "No image files found in current directory.".into(),
    );

    m.insert(
        "cmd.image.displaying".into(),
        "Displaying {} ({}x{}) via {}".into(),
    );

    m.insert("cmd.image.select_image".into(), "Select image:".into());

    m.insert(
        "cmd.image.usage".into(),
        "Usage: image [display|show] [--sixel] [<path>]".into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "Forcing sixel output (detected: {}) — this terminal may show garbage.".into(),
    );
}
