use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.views.usage".into(),
        "วิธีใช้: views [path] [-n] [--size [ขนาดขั้นต่ำ]] [--sort <name|ext|size>]".into(),
    );

    m.insert(
        "cmd.views.invalid_size".into(),
        "ขนาดไม่ถูกต้อง: {} (ใช้ 1024, 10KB, 5MB, ...)".into(),
    );

    m.insert("cmd.views.invalid_sort".into(), "ไม่มีค่าสำหรับ --sort".into());

    m.insert(
        "cmd.views.invalid_sort_value".into(),
        "ไม่รู้จักคีย์การเรียง: {} (ใช้ name, ext หรือ size)".into(),
    );

    m.insert("cmd.views.empty".into(), "(ไดเรกทอรีว่าง)".into());
}
