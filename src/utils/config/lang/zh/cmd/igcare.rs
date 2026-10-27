use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.igcare.create_failed".into(), "创建文件失败：{}".into());

    m.insert(
        "cmd.igcare.example_export_all".into(),
        "  igcare export --all myproject        # Creates myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.example_export_select".into(),
        "  igcare export --select myproject     # Pick which categories to export".into(),
    );

    m.insert(
        "cmd.igcare.example_import".into(),
        "  igcare import myproject.ntc.igcare   # Import into current config".into(),
    );

    m.insert("cmd.igcare.examples_title".into(), "示例：".into());

    m.insert(
        "cmd.igcare.export_cmd".into(),
        "igcare export --all <名称> 将所有设置导出到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_name_empty".into(),
        "导出名称不能为空。".into(),
    );

    m.insert(
        "cmd.igcare.export_select_cmd".into(),
        "igcare export --select <名称> 选择类别导出到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_usage".into(),
        "用法：igcare export --all <名称> | igcare export --select <名称>".into(),
    );

    m.insert("cmd.igcare.file_exists".into(), "文件已存在：{}".into());

    m.insert(
        "cmd.igcare.import_cmd".into(),
        "igcare import <文件> 从 .ntc.igcare 文件导入设置".into(),
    );

    m.insert(
        "cmd.igcare.import_example".into(),
        "示例：igcare import myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.import_usage".into(),
        "用法：igcare import <文件>".into(),
    );

    m.insert("cmd.igcare.main_title".into(), "忽略/关注管理器".into());

    m.insert(
        "cmd.igcare.template_created".into(),
        "已创建 igcare 模板：{}".into(),
    );

    m.insert(
        "cmd.igcare.template_hint".into(),
        "这是一个本地模板文件。使用 'igcare import' 应用设置。".into(),
    );

    m.insert(
        "cmd.igcare.type_help".into(),
        "输入 'igcare' 获取帮助。".into(),
    );

    m.insert(
        "cmd.igcare.unknown_flag".into(),
        "未知的导出标志：{}。使用 --all 或 --select".into(),
    );

    m.insert(
        "cmd.igcare.unknown_subcmd".into(),
        "未知的 igcare 子命令：{}".into(),
    );
}
