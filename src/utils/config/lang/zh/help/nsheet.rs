use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nsheet.title".into(), "表格命令：".into());

    m.insert(
        "help.nsheet.open".into(),
        "nsheet open <文件> [选项] ## 全屏表格浏览器（CSV/TSV/JSON）".into(),
    );

    m.insert(
        "help.nsheet.view".into(),
        "nsheet view <文件> [选项] ## 打印表格（默认 50 行）".into(),
    );

    m.insert(
        "help.nsheet.sort".into(),
        "nsheet view <文件> --sort <列> [--desc] ## 按列排序".into(),
    );

    m.insert(
        "help.nsheet.filter".into(),
        "nsheet view <文件> --filter <表达式> ## 筛选行（col~文本、col=值、col>数值）".into(),
    );

    m.insert(
        "help.nsheet.export".into(),
        "nsheet view <文件> --export <json|md> [-o <文件>] ## 导出".into(),
    );

    m.insert("help.nsheet.tui".into(), "全屏按键：方向键/hjkl 移动 · / 搜索 · n 下一个 · s 排序 · f 筛选 · e 导出 · Enter 查看 · q 退出".into());

    m.insert(
        "help.nsheet.example1".into(),
        "nsheet open users.csv ## 浏览表格".into(),
    );

    m.insert(
        "help.nsheet.example2".into(),
        "nsheet view data.json --filter \"age>30\" --sort age ## 筛选并排序".into(),
    );

    m.insert(
        "help.nsheet.example3".into(),
        "nsheet view a.csv --export md -o out.md ## 导出 Markdown".into(),
    );
}
