use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "Cancelled.".into());

    m.insert("cmd.image.default_label".into(), "Default".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. Cancel".into());

    m.insert("cmd.image.desc_html".into(), "  {}. HTML".into());

    m.insert("cmd.image.desc_json".into(), "  {}. JSON".into());

    m.insert(
        "cmd.image.desc_markdown".into(),
        "  {}. Markdown (default)".into(),
    );

    m.insert(
        "cmd.image.desc_narrative".into(),
        "  {}. Narrative description (for AI)".into(),
    );

    m.insert("cmd.image.desc_txt".into(), "  {}. TXT".into());

    m.insert("cmd.image.enter_number".into(), "Enter the number: ".into());

    m.insert(
        "cmd.image.missing_out_format".into(),
        "Missing format after 'out'.".into(),
    );

    m.insert(
        "cmd.image.missing_to_format".into(),
        "Missing format after 'to'.".into(),
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
        "Usage: image <path> [display|show [--sixel]|to|out <format>] [--ocr] [--layout]".into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "Forcing sixel output (detected: {}) — this terminal may show garbage.".into(),
    );
}
