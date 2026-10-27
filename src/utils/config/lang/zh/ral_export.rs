use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ral_export.all".into(), "全选".into());

    m.insert("ral_export.cancel".into(), "取消".into());

    m.insert("ral_export.export_cancelled".into(), "导出已取消。".into());

    m.insert("ral_export.finish".into(), "完成导出".into());

    m.insert("ral_export.prompt".into(), "ral-导出>".into());

    m.insert("ral_export.select".into(), "按数字选择/取消选择".into());

    m.insert("ral_export.selected".into(), "已选择".into());

    m.insert("ral_export.selected_mark".into(), "（已选择）".into());

    m.insert(
        "ral_export.title".into(),
        "ral 导出 — 选择要导出的别名".into(),
    );
}
