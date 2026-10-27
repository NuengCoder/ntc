use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.report.content_saved".into(),
        "文件内容已保存到：{}".into(),
    );

    m.insert(
        "cmd.report.copied_clipboard".into(),
        "文件 '{}' 已复制到剪贴板！".into(),
    );

    m.insert("cmd.report.copy_failed".into(), "复制失败：{}".into());

    m.insert(
        "cmd.report.copying_clipboard".into(),
        "正在将 '{}' 复制到剪贴板...".into(),
    );

    m.insert("cmd.report.dir_only".into(), "{} 报告只能用于目录".into());

    m.insert(
        "cmd.report.example_json".into(),
        "  report json ..         对父目录生成报告".into(),
    );

    m.insert(
        "cmd.report.example_txt".into(),
        "  report txt             对当前目录生成报告".into(),
    );

    m.insert("cmd.report.examples".into(), "示例：".into());

    m.insert("cmd.report.file_not_found".into(), "文件未找到：{}".into());

    m.insert(
        "cmd.report.no_files_in_dir".into(),
        "该目录中没有可显示的支持文件：{}".into(),
    );

    m.insert(
        "cmd.report.help_docx".into(),
        "  report docx [路径]      生成 DOCX 报告".into(),
    );

    m.insert(
        "cmd.report.help_html".into(),
        "  report html [路径]      生成 HTML 报告".into(),
    );

    m.insert(
        "cmd.report.help_json".into(),
        "  report json [路径]      生成 JSON 报告".into(),
    );

    m.insert(
        "cmd.report.help_md".into(),
        "  report md [路径]        生成 Markdown 报告".into(),
    );

    m.insert(
        "cmd.report.help_pdf".into(),
        "  report pdf [路径]       生成 PDF 报告".into(),
    );

    m.insert(
        "cmd.report.help_txt".into(),
        "  report txt [路径]       生成 TXT 报告".into(),
    );

    m.insert(
        "cmd.report.help_xlsx".into(),
        "  report xlsx [路径]      生成 XLSX 报告".into(),
    );

    m.insert("cmd.report.invalid_depth".into(), "无效的深度：{}".into());

    m.insert(
        "cmd.report.neovim_hint".into(),
        "可以用 Neovim 打开：:edit {}".into(),
    );

    m.insert(
        "cmd.report.skipped_unsupported".into(),
        "已跳过（不支持的格式）：{}".into(),
    );

    m.insert(
        "cmd.report.ext_usage".into(),
        "用法：view -ext <扩展名...>（例如 view -ext rs c）".into(),
    );

    m.insert("cmd.report.time_label".into(), "耗时：".into());

    m.insert("cmd.report.title".into(), "report — 生成项目报告".into());

    m.insert(
        "cmd.report.unknown_option".into(),
        "未知的 view 选项：{}".into(),
    );

    m.insert("cmd.report.view_usage".into(), "用法：view [-s|--size [最小值]] [-fs|--filesize [最小值]] [-ds|--dirsize [最小值]] [--sort size|char|ext] [--head <n>|--tail <n>] [-c|--cp] [--care] [-t|--time] [-d|--depth <n>] [-p|--path <路径|@tp>] [-ext|--ext <扩展名...>]".into());

    m.insert(
        "cmd.report.invalid_sort".into(),
        "无效的排序键：'{}'（size|char|ext）".into(),
    );

    m.insert(
        "cmd.report.invalid_count".into(),
        "无效的行数：'{}'（必须大于 0）".into(),
    );

    m.insert(
        "cmd.report.invalid_range".into(),
        "无效的范围：'{}'（应为 n:m，如 --betw 10:20）".into(),
    );

    m.insert("cmd.report.clip_header".into(), "剪贴板条目 #{}:".into());

    m.insert("cmd.report.clip_meta_date".into(), "日期".into());

    m.insert("cmd.report.clip_meta_lines".into(), "行数".into());

    m.insert("cmd.report.clip_meta_chars".into(), "字符数".into());
}
