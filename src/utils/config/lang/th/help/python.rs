use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.python.activate".into(),
        "py_activate ## เปิดใช้งานสภาพแวดล้อม virtual Python (ตรวจจับอัตโนมัติ)".into(),
    );

    m.insert(
        "help.python.activate_path".into(),
        "py_activate <path> ## เปิดใช้งานเส้นทาง venv เฉพาะ".into(),
    );

    m.insert(
        "help.python.deactivate".into(),
        "py_deactivate ## ปิดใช้งานสภาพแวดล้อม virtual ปัจจุบัน".into(),
    );

    m.insert("help.python.title".into(), "คำสั่ง Python (PYTHON):".into());

    m.insert(
        "help.python.example1".into(),
        "py_activate ## เปิดใช้งาน Python venv อัตโนมัติ".into(),
    );

    m.insert(
        "help.python.example2".into(),
        "py_activate .venv ## เปิดใช้งาน venv เฉพาะ".into(),
    );

    m.insert(
        "help.python.example3".into(),
        "py_deactivate ## ปิดใช้งาน venv".into(),
    );
}
