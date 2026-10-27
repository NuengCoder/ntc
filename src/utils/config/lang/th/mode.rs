use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "mode.help_admin".into(),
        "  mode admin               ออกและเปิดใหม่ในฐานะ Administrator".into(),
    );

    m.insert(
        "mode.help_example_admin".into(),
        "  mode admin                (UAC prompt จะปรากฏ)".into(),
    );

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "mode.help_normal".into(),
        "  mode normal              ออกและเปิดใหม่ในโหมดปกติ".into(),
    );

    m.insert("mode.help_usage".into(), "การใช้งาน:".into());
}
