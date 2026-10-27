use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("helpers.building_tree".into(), "正在构建目录树...".into());

    m.insert(
        "helpers.calculating_sizes".into(),
        "正在计算目录大小...".into(),
    );

    m.insert("helpers.export_cancelled".into(), "导出已取消。".into());

    m.insert(
        "helpers.exported_all".into(),
        "已导出所有 {} 个别名到：{}".into(),
    );

    m.insert(
        "helpers.exported_categories".into(),
        "已导出 {} 个类别到：{}".into(),
    );

    m.insert(
        "helpers.exported_count".into(),
        "已导出 {} 个别名到：{}".into(),
    );

    m.insert(
        "helpers.exported_igcare_all".into(),
        "已导出所有忽略/关注设置到：{}".into(),
    );

    m.insert("helpers.file_not_found".into(), "未找到文件：{}".into());

    m.insert(
        "helpers.imported_igcare".into(),
        "已从以下位置导入忽略/关注设置：{}".into(),
    );

    m.insert(
        "helpers.imported_ral".into(),
        "已从以下位置导入运行别名：{}".into(),
    );

    m.insert(
        "helpers.invalid_back_count".into(),
        "无效的返回计数：{}".into(),
    );

    m.insert("helpers.invalid_input".into(), "无效的输入：{}".into());

    m.insert("helpers.invalid_num".into(), "无效的数字：{}".into());

    m.insert(
        "helpers.invalid_num_range".into(),
        "无效的数字：{}。使用 1-{} 或 0 完成。".into(),
    );

    m.insert(
        "helpers.no_editor".into(),
        "未找到外部编辑器。打开内置编辑器...".into(),
    );

    m.insert(
        "helpers.no_ral_to_export".into(),
        "没有要导出的运行别名。".into(),
    );
}
