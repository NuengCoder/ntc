use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.py_deactivate.title".into(),
        "PY_DEACTIVATE COMMAND:".into(),
    );

    m.insert(
        "help.py_deactivate.usage".into(),
        "py_deactivate ## Deactivate current virtual environment".into(),
    );

    m.insert(
        "help.py_deactivate.example1".into(),
        "py_deactivate ## Deactivate venv".into(),
    );
}
