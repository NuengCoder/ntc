use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.python.activate".into(),
        "py_activate ## ເປີດໃຊ້ Python virtual environment (auto-detect)".into(),
    );

    m.insert(
        "help.python.activate_path".into(),
        "py_activate <path> ## ເປີດໃຊ້ venv path ສະເພາະ".into(),
    );

    m.insert(
        "help.python.deactivate".into(),
        "py_deactivate ## ປິດໃຊ້ງານ virtual environment ປັດຈຸບັນ".into(),
    );

    m.insert("help.python.title".into(), "ຄຳສັ່ງ PYTHON:".into());

    m.insert(
        "help.python.example1".into(),
        "py_activate ## ເປີດໃຊ້ Python venv ອັດຕະໂນມັດ".into(),
    );

    m.insert(
        "help.python.example2".into(),
        "py_activate .venv ## ເປີດໃຊ້ venv ສະເພາະ".into(),
    );

    m.insert(
        "help.python.example3".into(),
        "py_deactivate ## ປິດໃຊ້ງານ venv".into(),
    );
}
