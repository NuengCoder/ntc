use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "mode.help_admin".into(),
        "  mode admin              退出并以管理员身份重新启动".into(),
    );

    m.insert(
        "mode.help_example_admin".into(),
        "  mode admin               （将出现 UAC 提示）".into(),
    );

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_examples".into(), "示例：".into());

    m.insert(
        "mode.help_normal".into(),
        "  mode normal             退出并以普通身份重新启动".into(),
    );

    m.insert("mode.help_usage".into(), "用法：".into());
}
