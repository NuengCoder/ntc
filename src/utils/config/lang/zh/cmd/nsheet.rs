use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nsheet.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.nsheet.help_open".into(),
        "nsheet open <文件> [选项]   全屏浏览器（CSV/TSV/JSON/JSONL）".into(),
    );

    m.insert(
        "cmd.nsheet.help_view".into(),
        "nsheet view <文件> [选项]   打印表格（默认 50 行，--all 显示全部）".into(),
    );

    m.insert("cmd.nsheet.options_title".into(), "选项：".into());

    m.insert(
        "cmd.nsheet.help_delimiter".into(),
        "  -d, --delimiter <字符|auto>  字段分隔符（默认自动检测 , ; tab |）".into(),
    );

    m.insert(
        "cmd.nsheet.help_header".into(),
        "  --header / --no-header        强制有/无表头（默认自动检测）".into(),
    );

    m.insert(
        "cmd.nsheet.help_limit".into(),
        "  -l, --limit <行数> / --all     view 模式打印行数".into(),
    );

    m.insert(
        "cmd.nsheet.help_sort".into(),
        "  --sort <列> [--desc]            按列名或从 1 起的序号排序".into(),
    );

    m.insert(
        "cmd.nsheet.help_filter".into(),
        "  --filter <表达式>             例如 age>30、city~york、name=bob（!=,>=,<=,&&,||）".into(),
    );

    m.insert(
        "cmd.nsheet.help_export".into(),
        "  --export <json|md>             导出而不是打印".into(),
    );

    m.insert(
        "cmd.nsheet.help_output".into(),
        "  -o, --output <文件>            导出目标（默认标准输出）".into(),
    );

    m.insert(
        "cmd.nsheet.help_help".into(),
        "  --help, -h                     显示此帮助".into(),
    );

    m.insert("cmd.nsheet.examples_title".into(), "示例：".into());

    m.insert(
        "cmd.nsheet.example_open".into(),
        "nsheet open users.csv".into(),
    );

    m.insert(
        "cmd.nsheet.example_view".into(),
        "nsheet view users.csv -l 20 --sort age --desc".into(),
    );

    m.insert(
        "cmd.nsheet.example_filter".into(),
        "nsheet view data.json --filter \"age>30 && city~york\"".into(),
    );

    m.insert(
        "cmd.nsheet.example_export".into(),
        "nsheet view a.csv --export json -o out.json".into(),
    );

    m.insert(
        "cmd.nsheet.missing_file".into(),
        "缺少文件。用法：nsheet open <文件> | nsheet view <文件>".into(),
    );

    m.insert("cmd.nsheet.file_not_found".into(), "未找到文件 '{}'".into());

    m.insert("cmd.nsheet.load_fail".into(), "无法加载表格：{}".into());

    m.insert("cmd.nsheet.empty_sheet".into(), "空表格（未找到列）".into());

    m.insert("cmd.nsheet.no_rows".into(), "没有匹配的行。".into());

    m.insert("cmd.nsheet.unknown_option".into(), "未知：{}".into());

    m.insert("cmd.nsheet.unknown_column".into(), "未知列：'{}'".into());

    m.insert("cmd.nsheet.invalid_limit".into(), "无效的行数：'{}'".into());

    m.insert(
        "cmd.nsheet.invalid_delimiter".into(),
        "无效的分隔符：'{}'（使用单个字符或 auto）".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_export".into(),
        "无效的导出格式：'{}'（使用 json 或 md）".into(),
    );

    m.insert(
        "cmd.nsheet.delimiter_usage".into(),
        "用法：--delimiter <字符|auto>".into(),
    );

    m.insert(
        "cmd.nsheet.limit_usage".into(),
        "用法：--limit <行数>".into(),
    );

    m.insert("cmd.nsheet.sort_usage".into(), "用法：--sort <列>".into());

    m.insert(
        "cmd.nsheet.filter_usage".into(),
        "用法：--filter <表达式>（例如 age>30）".into(),
    );

    m.insert(
        "cmd.nsheet.export_usage".into(),
        "用法：--export <json|md>".into(),
    );

    m.insert(
        "cmd.nsheet.output_usage".into(),
        "用法：--output <文件>".into(),
    );

    m.insert("cmd.nsheet.filter_fail".into(), "筛选表达式错误：{}".into());

    m.insert("cmd.nsheet.export_fail".into(), "导出失败：{}".into());

    m.insert("cmd.nsheet.exported".into(), "已导出：{}".into());

    m.insert(
        "cmd.nsheet.rows_shown".into(),
        "显示 {} / {} 行（{} 列）".into(),
    );

    m.insert(
        "cmd.nsheet.rows_truncated".into(),
        "已截断为 {} 行（使用 --all 或 -l <行数>）".into(),
    );

    m.insert("cmd.nsheet.tui_hint".into(), "方向键/hjkl 移动 · / 搜索 · n 下一个 · s 排序 · f 筛选 · c 清除 · e 导出 · Enter 查看 · q 退出".into());

    m.insert(
        "cmd.nsheet.tui_search_prompt".into(),
        "搜索（留空清除）".into(),
    );

    m.insert(
        "cmd.nsheet.tui_sort_prompt".into(),
        "按列排序（! 后缀 = 降序，留空清除）".into(),
    );

    m.insert(
        "cmd.nsheet.tui_filter_prompt".into(),
        "筛选（留空清除）".into(),
    );

    m.insert(
        "cmd.nsheet.tui_export_prompt".into(),
        "导出到（.json/.md 路径）".into(),
    );

    m.insert(
        "cmd.nsheet.tui_cell_title".into(),
        "单元格（按任意键关闭）".into(),
    );

    m.insert("cmd.nsheet.tui_popup_hint".into(), "按任意键关闭".into());

    m.insert("cmd.nsheet.tui_no_matches".into(), "无匹配".into());

    m.insert(
        "cmd.nsheet.tui_large".into(),
        "大表格（{} 行）—— 导航可能较慢".into(),
    );
}
