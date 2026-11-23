use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.report.title".into(), "报告命令：".into());

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## 项目报告保存到 <file>".into(),
    );

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## 完整显示剪贴板历史条目".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report project html . --name docs/project ## 生成带可折叠目录树的 HTML 报告，保存为 docs/project.html".into(),
    );

    m.insert("help.report.example2".into(), "report clip 1".into());
}
