use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("interactive.cancelled".into(), "已取消。".into());

    m.insert("interactive.invalid_input".into(), "无效的输入。".into());

    m.insert(
        "interactive.invalid_selection".into(),
        "无效的选择。".into(),
    );

    m.insert("interactive.no_items".into(), "（无项目）".into());
}
