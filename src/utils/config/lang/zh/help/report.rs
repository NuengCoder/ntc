use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.report.cp".into(),
        "report <格式> --cp ## 将报告复制到剪贴板而非保存".into(),
    );

    m.insert(
        "help.report.docx".into(),
        "report docx [路径] ## 生成DOCX报告".into(),
    );

    m.insert(
        "help.report.html".into(),
        "report html [路径] ## 生成HTML报告".into(),
    );

    m.insert(
        "help.report.json".into(),
        "report json [路径] ## 生成JSON报告".into(),
    );

    m.insert(
        "help.report.md".into(),
        "report md [路径] ## 生成Markdown报告".into(),
    );

    m.insert(
        "help.report.pdf".into(),
        "report pdf [路径] ## 生成PDF报告".into(),
    );

    m.insert("help.report.title".into(), "报告命令：".into());

    m.insert(
        "help.report.txt".into(),
        "report txt [路径] ## 生成TXT报告".into(),
    );

    m.insert(
        "help.report.txtc".into(),
        "txtc <文件> ## 将文件内容复制到剪贴板".into(),
    );

    m.insert("help.report.txtf".into(), "txtf [--head N|--tail N|--betw A:B] [文件|目录|@tp/文件|@tp/目录] ## 显示文件内容或从目录中选择文件（保留原始行号）".into());

    m.insert(
        "help.report.txtf_example1".into(),
        "txtf main.rs --head 30 ## 显示 main.rs 前 30 行".into(),
    );

    m.insert(
        "help.report.txtf_example2".into(),
        "txtf --betw 10:20 main.rs ## 显示 main.rs 第 10-20 行".into(),
    );

    m.insert(
        "help.report.txtf_example3".into(),
        "txtf @docs/notes.txt ## 显示传送点中的文件".into(),
    );

    m.insert(
        "help.report.txtf_example4".into(),
        "txtf src ## 从 src 目录中选择一个支持的文件显示".into(),
    );

    m.insert(
        "help.report.xlsx".into(),
        "report xlsx [路径] ## 生成XLSX报告（含依赖分析）".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report txt ## 生成 TXT 报告".into(),
    );

    m.insert(
        "help.report.example2".into(),
        "report html --cp ## 复制 HTML 报告到剪贴板".into(),
    );

    m.insert(
        "help.report.example3".into(),
        "report pdf ## 生成 PDF 报告".into(),
    );

    m.insert(
        "help.report.example4".into(),
        "txtc file.txt ## 复制文件内容到剪贴板".into(),
    );

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## 项目报告保存到 <file>".into(),
    );

    m.insert("help.report.project_example".into(), "report project html . --name docs/project ## 生成带可折叠目录树的 HTML 报告，保存为 docs/project.html".into());

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## 完整显示剪贴板历史条目".into(),
    );

    m.insert("help.report.clip_example".into(), "report clip 1".into());
}
