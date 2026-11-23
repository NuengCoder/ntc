use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.py_activate.title".into(), "Python命令：".into());

    m.insert(
        "help.py_activate.activate".into(),
        "py_activate ## 激活Python虚拟环境（自动检测）".into(),
    );

    m.insert(
        "help.py_activate.activate_path".into(),
        "py_activate <路径> ## 激活指定venv路径".into(),
    );

    m.insert(
        "help.py_activate.example1".into(),
        "py_activate ## 自动激活 Python 虚拟环境".into(),
    );

    m.insert(
        "help.py_activate.example2".into(),
        "py_activate .venv ## 激活指定虚拟环境".into(),
    );
}
