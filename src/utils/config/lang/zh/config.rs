use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "config.general.autosuggest".into(),
        " config autosuggest on|off 显示/设置自动建议".into(),
    );

    m.insert(
        "config.general.color".into(),
        " config color on|off 显示/设置颜色".into(),
    );

    m.insert(
        "config.general.depth".into(),
        " config depth [n] 显示/设置最大深度（1-20）".into(),
    );

    m.insert("config.general.examples".into(), "示例：".into());

    m.insert(
        "config.general.history".into(),
        " config history [on|off|路径] 显示/设置历史记录".into(),
    );

    m.insert(
        "config.general.lang".into(),
        " config lang [语言] 显示/设置界面语言".into(),
    );

    m.insert(
        "config.general.linenumbers".into(),
        " config linenumbers on|off 显示/设置行号".into(),
    );

    m.insert(
        "config.general.output".into(),
        " config output [路径] 显示/设置输出路径".into(),
    );

    m.insert(
        "config.general.qsc".into(),
        " config qsc \"#c 0\" 显示/设置QSC全局热键".into(),
    );

    m.insert("config.general.settings".into(), "设置：".into());

    m.insert(
        "config.general.threads".into(),
        " config threads [n] 显示/设置线程数".into(),
    );

    m.insert(
        "config.general.title".into(),
        "config \u{2014} 查看或更改配置设置".into(),
    );

    m.insert(
        "config.general.watch".into(),
        " config watch on|off 显示/设置文件监视器".into(),
    );

    m.insert("config.lang.set".into(), "语言已设置为：{}".into());
}
