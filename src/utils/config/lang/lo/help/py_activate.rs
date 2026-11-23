use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.py_activate.title".into(), "ຄຳສັ່ງ PYTHON:".into());

    m.insert(
        "help.py_activate.activate".into(),
        "py_activate ## ເປີດໃຊ້ Python virtual environment (auto-detect)".into(),
    );

    m.insert(
        "help.py_activate.activate_path".into(),
        "py_activate <path> ## ເປີດໃຊ້ venv path ສະເພາະ".into(),
    );

    m.insert(
        "help.py_activate.example1".into(),
        "py_activate ## ເປີດໃຊ້ Python venv ອັດຕະໂນມັດ".into(),
    );

    m.insert(
        "help.py_activate.example2".into(),
        "py_activate .venv ## ເປີດໃຊ້ venv ສະເພາະ".into(),
    );
}
