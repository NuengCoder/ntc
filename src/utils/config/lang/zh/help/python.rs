use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.python.activate".into(),
        "py_activate ## 激活Python虚拟环境（自动检测）".into(),
    );

    m.insert(
        "help.python.activate_path".into(),
        "py_activate <路径> ## 激活指定venv路径".into(),
    );

    m.insert(
        "help.python.deactivate".into(),
        "py_deactivate ## 停用当前虚拟环境".into(),
    );

    m.insert("help.python.title".into(), "Python命令：".into());

    m.insert(
        "help.python.example1".into(),
        "py_activate ## 自动激活 Python 虚拟环境".into(),
    );

    m.insert(
        "help.python.example2".into(),
        "py_activate .venv ## 激活指定虚拟环境".into(),
    );

    m.insert(
        "help.python.example3".into(),
        "py_deactivate ## 停用虚拟环境".into(),
    );
}
