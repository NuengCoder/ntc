use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("theme.create_failed".into(), "创建主题失败：{}".into());

    m.insert("theme.create_file_failed".into(), "创建文件失败：{}".into());

    m.insert(
        "theme.create_first".into(),
        "运行 'theme add <名称>' 创建您的第一个主题".into(),
    );

    m.insert(
        "theme.created".into(),
        "已创建主题 '{}'（在编辑器中打开）".into(),
    );

    m.insert(
        "theme.created_hint".into(),
        "编辑文件，保存，然后运行 'theme reload' 应用更改".into(),
    );

    m.insert("theme.current_title".into(), "当前主题：{}".into());

    m.insert("theme.dir_title".into(), "主题目录：".into());

    m.insert(
        "theme.edit_hint".into(),
        "编辑 '{}'，保存，然后运行 'theme reload'".into(),
    );

    m.insert("theme.export_failed".into(), "导出主题失败：{}".into());

    m.insert("theme.export_failed_simple".into(), "导出失败：{}".into());

    m.insert("theme.exported_simple".into(), "已导出到：{}".into());

    m.insert("theme.exported_to".into(), "已导出主题 '{}' 到：{}".into());

    m.insert("theme.file_exists".into(), "文件已存在：{}".into());

    m.insert("theme.file_not_found".into(), "未找到文件：{}".into());

    m.insert("theme.files_title".into(), "主题文件：".into());

    m.insert("theme.import_failed".into(), "导入主题失败：{}".into());

    m.insert("theme.import_failed_simple".into(), "导入失败：{}".into());

    m.insert(
        "theme.import_hint".into(),
        "运行 'theme list' 查看可用主题".into(),
    );

    m.insert("theme.imported".into(), "已从以下位置导入主题：{}".into());

    m.insert("theme.info_author".into(), "作者".into());

    m.insert("theme.info_defined".into(), "\u{2713} 已定义".into());

    m.insert("theme.info_description".into(), "描述".into());

    m.insert("theme.info_editor".into(), "编辑器颜色".into());

    m.insert("theme.info_shell".into(), "Shell 颜色".into());

    m.insert("theme.info_syntax".into(), "语法颜色".into());

    m.insert("theme.info_title".into(), "\u{1F3A8} 主题：{}".into());

    m.insert(
        "theme.invalid_number".into(),
        "无效的编号：{}。使用 1-{}".into(),
    );

    m.insert("theme.name_empty".into(), "主题名称不能为空".into());

    m.insert("theme.no_dir".into(), "无法确定主题目录".into());

    m.insert("theme.no_dir_yet".into(), "（目录尚不存在）".into());

    m.insert("theme.no_files".into(), "（无主题文件）".into());

    m.insert("theme.not_found".into(), "未找到主题 '{}'".into());

    m.insert(
        "theme.not_found_hint".into(),
        "未找到主题：{}。使用 'theme list' 查看可用主题。".into(),
    );

    m.insert("theme.opened_dir".into(), "已打开主题目录：{}".into());

    m.insert("theme.reloaded".into(), "主题已从磁盘重新加载".into());

    m.insert("theme.remove_failed".into(), "删除主题失败：{}".into());

    m.insert("theme.removed".into(), "已删除主题：{}".into());

    m.insert("theme.rename_failed".into(), "重命名主题失败：{}".into());

    m.insert("theme.rename_failed_simple".into(), "重命名失败：{}".into());

    m.insert("theme.renamed".into(), "已重命名主题 '{}' -> '{}'".into());

    m.insert(
        "theme.renamed_simple".into(),
        "已重命名 '{}' -> '{}'".into(),
    );

    m.insert("theme.switch_failed".into(), "切换主题失败：{}".into());

    m.insert(
        "theme.switch_failed_simple".into(),
        "切换主题失败：{}".into(),
    );

    m.insert("theme.switched".into(), "已切换到主题：{}".into());

    m.insert("theme.template_created".into(), "已创建主题模板：{}".into());

    m.insert(
        "theme.template_info".into(),
        "这是一个本地模板文件。使用 'theme import' 将其添加到主题列表。".into(),
    );

    m.insert(
        "theme.type_interactive".into(),
        "输入 'theme' 进入交互菜单".into(),
    );

    m.insert("theme.ui.current".into(), "当前主题".into());

    m.insert("theme.ui.exit".into(), "退出主题管理器。".into());

    m.insert("theme.ui.name_prompt".into(), "主题名称：".into());

    m.insert("theme.ui.theme_prompt".into(), "theme>".into());

    m.insert("theme.ui.title".into(), "\u{1F3A8} ntc 主题管理器".into());

    m.insert("theme.unknown_cmd".into(), "未知命令：{}".into());

    m.insert("theme.usage_add".into(), "用法：theme add <名称>".into());

    m.insert(
        "theme.usage_export".into(),
        "用法：theme export <名称> [输出文件]".into(),
    );

    m.insert(
        "theme.usage_import".into(),
        "用法：theme import <文件.ntc_theme>".into(),
    );

    m.insert("theme.usage_info".into(), "用法：theme info <名称>".into());

    m.insert(
        "theme.usage_rename".into(),
        "用法：theme rename <旧名> to <新名>".into(),
    );

    m.insert("theme.usage_rm".into(), "用法：theme rm <名称>".into());

    m.insert(
        "theme.usage_rnm".into(),
        "用法：theme rnm <旧名> to <新名>".into(),
    );
}
