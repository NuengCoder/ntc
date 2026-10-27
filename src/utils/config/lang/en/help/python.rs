use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.python.activate".into(),
        "py_activate ## Activate Python virtual environment (auto-detect)".into(),
    );

    m.insert(
        "help.python.activate_path".into(),
        "py_activate <path> ## Activate specific venv path".into(),
    );

    m.insert(
        "help.python.deactivate".into(),
        "py_deactivate ## Deactivate current virtual environment".into(),
    );

    m.insert("help.python.title".into(), "PYTHON COMMANDS:".into());

    m.insert(
        "help.python.example1".into(),
        "py_activate ## Auto-activate Python venv".into(),
    );

    m.insert(
        "help.python.example2".into(),
        "py_activate .venv ## Activate specific venv".into(),
    );

    m.insert(
        "help.python.example3".into(),
        "py_deactivate ## Deactivate venv".into(),
    );
}
