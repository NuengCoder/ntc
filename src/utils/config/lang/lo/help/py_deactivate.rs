use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.py_deactivate.title".into(),
        "PY_DEACTIVATE ຄຳສັ່ງ:".into(),
    );

    m.insert(
        "help.py_deactivate.usage".into(),
        "py_deactivate ## ປິດໃຊ້ງານ virtual environment ປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.py_deactivate.example1".into(),
        "py_deactivate ## ປິດໃຊ້ງານ venv".into(),
    );
}
