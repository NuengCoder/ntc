use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "navigator.already_root".into(),
        "อยู่ที่รากแล้ว ไม่สามารถกลับไปต่อได้".into(),
    );

    m.insert(
        "navigator.cannot_navigate_to".into(),
        "ไม่สามารถนำทางไปที่: {}".into(),
    );

    m.insert(
        "navigator.cannot_read_dir".into(),
        "ไม่สามารถอ่านไดเรกทอรี: {}".into(),
    );

    m.insert(
        "navigator.drive_windows_only".into(),
        "การนำทางไดรฟ์รองรับเฉพาะบน Windows".into(),
    );

    m.insert("navigator.not_a_directory".into(), "{} ไม่ใช่ไดเรกทอรี".into());

    m.insert(
        "navigator.venv_activate_unix".into(),
        "   เรียก: source ./venv/bin/activate".into(),
    );

    m.insert(
        "navigator.venv_activate_win".into(),
        "   เรียก: .\\\\venv\\\\Scripts\\\\activate".into(),
    );

    m.insert(
        "navigator.venv_tip".into(),
        "   เรียก: source ./venv/bin/activate".into(),
    );
}
