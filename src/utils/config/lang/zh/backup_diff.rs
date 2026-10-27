use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup_diff.cancel_opt".into(), "0. 取消".into());

    m.insert("backup_diff.cancelled".into(), "比较已取消。".into());

    m.insert(
        "backup_diff.comparing".into(),
        "正在将当前项目与备份 #{} 进行比较...".into(),
    );

    m.insert("backup_diff.invalid_input".into(), "无效的输入：{}".into());

    m.insert(
        "backup_diff.invalid_selection".into(),
        "无效的选择：{}".into(),
    );

    m.insert(
        "backup_diff.none".into(),
        "未找到此项目的备份。使用 'bkup' 创建一个。".into(),
    );

    m.insert("backup_diff.not_found".into(), "备份 #{} 未找到。".into());

    m.insert(
        "backup_diff.open_manually".into(),
        "手动打开文件：{}".into(),
    );

    m.insert("backup_diff.saved_to".into(), "差异报告已保存到：{}".into());

    m.insert(
        "backup_diff.select_prompt".into(),
        "选择备份（1-{}）：".into(),
    );

    m.insert(
        "backup_diff.select_title".into(),
        "\u{1F4CA} 选择要与当前项目比较的备份".into(),
    );
}
