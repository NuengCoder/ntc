use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.py_activate.title".into(), "PYTHON COMMANDS:".into());

    m.insert(
        "help.py_activate.activate".into(),
        "py_activate ## Activate Python virtual environment (auto-detect)".into(),
    );

    m.insert(
        "help.py_activate.activate_path".into(),
        "py_activate <path> ## Activate specific venv path".into(),
    );

    m.insert(
        "help.py_activate.example1".into(),
        "py_activate ## Auto-activate Python venv".into(),
    );

    m.insert(
        "help.py_activate.example2".into(),
        "py_activate .venv ## Activate specific venv".into(),
    );
}
