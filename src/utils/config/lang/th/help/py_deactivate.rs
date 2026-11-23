use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.py_deactivate.title".into(),
        "PY_DEACTIVATE คำสั่ง:".into(),
    );

    m.insert(
        "help.py_deactivate.usage".into(),
        "py_deactivate ## ปิดใช้งานสภาพแวดล้อม virtual ปัจจุบัน".into(),
    );

    m.insert(
        "help.py_deactivate.example1".into(),
        "py_deactivate ## ปิดใช้งาน venv".into(),
    );
}
