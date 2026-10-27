use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("igcare_export.all".into(), "全选".into());

    m.insert("igcare_export.cancel".into(), "取消".into());

    m.insert(
        "igcare_export.extra_supported_extensions".into(),
        "额外支持的扩展名".into(),
    );

    m.insert(
        "igcare_export.extra_supported_files".into(),
        "额外支持的文件".into(),
    );

    m.insert("igcare_export.finish".into(), "完成导出".into());

    m.insert("igcare_export.ignored_dirs".into(), "忽略的目录".into());

    m.insert(
        "igcare_export.ignored_extensions".into(),
        "忽略的扩展名".into(),
    );

    m.insert("igcare_export.ignored_files".into(), "忽略的文件".into());

    m.insert("igcare_export.items".into(), "{} 个项目".into());

    m.insert("igcare_export.prompt".into(), "igcare-导出>".into());

    m.insert("igcare_export.select".into(), "按数字选择/取消选择".into());

    m.insert("igcare_export.selected".into(), "已选择".into());

    m.insert("igcare_export.selected_mark".into(), "（已选择）".into());

    m.insert(
        "igcare_export.title".into(),
        "igcare 导出 — 选择类别".into(),
    );

    m.insert("igcare_export.cared_dirs".into(), "关注的目录".into());

    m.insert(
        "igcare_export.cared_extensions".into(),
        "关注的扩展名".into(),
    );

    m.insert("igcare_export.cared_files".into(), "关注的文件".into());
}
