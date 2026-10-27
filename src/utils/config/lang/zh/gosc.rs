use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("gosc.calculating_size".into(), "正在计算目录大小...".into());

    m.insert("gosc.exiting".into(), "正在退出 gosc...".into());

    m.insert(
        "gosc.output_zone".into(),
        "─── 输出区 ───────────────────────────────────".into(),
    );

    m.insert(
        "gosc.output_zone_end".into(),
        "──────────────────────────────────────────────────".into(),
    );

    m.insert("gosc.panel.back_1".into(), "返回 1 级".into());

    m.insert("gosc.panel.back_2".into(), "返回 2 级".into());

    m.insert("gosc.panel.back_n".into(), "返回 n 级".into());

    m.insert("gosc.panel.empty".into(), "（无子目录）".into());

    m.insert("gosc.panel.exit".into(), "退出 gosc".into());

    m.insert(
        "gosc.panel.size_care".into(),
        "大小 --care（总大小）".into(),
    );

    m.insert("gosc.panel.title".into(), "gosc — 持续导航".into());

    m.insert(
        "gosc.panel.view_care".into(),
        "查看 --care -d 1（文件 + 关注）".into(),
    );

    m.insert(
        "gosc.panel.view_size_care".into(),
        "查看 -s --care -d 1（文件 + 大小 + 关注）".into(),
    );

    m.insert("gosc.press_enter".into(), "按回车继续...".into());

    m.insert("gosc.prompt".into(), "gosc>".into());

    m.insert("gosc.size_bytes".into(), "字节：".into());

    m.insert("gosc.size_human".into(), "可读：".into());

    m.insert("gosc.size_title".into(), "当前目录大小".into());
}
