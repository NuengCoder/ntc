use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("nav.available_drives".into(), "可用驱动器：".into());

    m.insert("nav.cancelled".into(), "已取消。".into());

    m.insert("nav.config_exists".into(), " \u{2713} 配置文件存在".into());

    m.insert(
        "nav.drive_prompt".into(),
        "输入驱动器字母或编号（或 'cancel'）：".into(),
    );

    m.insert("nav.gos_where".into(), "gos 哪里？".into());
}
