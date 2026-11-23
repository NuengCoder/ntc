use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.py_deactivate.title".into(),
        "PY_DEACTIVATE 命令：".into(),
    );

    m.insert(
        "help.py_deactivate.usage".into(),
        "py_deactivate ## 停用当前虚拟环境".into(),
    );

    m.insert(
        "help.py_deactivate.example1".into(),
        "py_deactivate ## 停用虚拟环境".into(),
    );
}
