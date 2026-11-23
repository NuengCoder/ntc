use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.py_activate.title".into(),
        "คำสั่ง Python (PYTHON):".into(),
    );

    m.insert(
        "help.py_activate.activate".into(),
        "py_activate ## เปิดใช้งานสภาพแวดล้อม virtual Python (ตรวจจับอัตโนมัติ)".into(),
    );

    m.insert(
        "help.py_activate.activate_path".into(),
        "py_activate <path> ## เปิดใช้งานเส้นทาง venv เฉพาะ".into(),
    );

    m.insert(
        "help.py_activate.example1".into(),
        "py_activate ## เปิดใช้งาน Python venv อัตโนมัติ".into(),
    );

    m.insert(
        "help.py_activate.example2".into(),
        "py_activate .venv ## เปิดใช้งาน venv เฉพาะ".into(),
    );
}
