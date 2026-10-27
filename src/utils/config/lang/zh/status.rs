use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("status.none".into(), "无".into());

    m.insert("status.off".into(), "关".into());

    m.insert("status.on".into(), "开".into());
}
