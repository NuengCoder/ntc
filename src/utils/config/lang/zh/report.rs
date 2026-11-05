use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "report.generating_html".into(),
        "正在生成 {} 的 HTML 报告".into(),
    );

    m.insert(
        "report.generating_json".into(),
        "正在生成 {} 的 JSON 报告".into(),
    );

    m.insert(
        "report.generating_md".into(),
        "正在生成 {} 的 Markdown 报告".into(),
    );

    m.insert("report.saved_html".into(), "HTML 报告已保存到：{}".into());

    m.insert("report.saved_json".into(), "JSON 报告已保存到：{}".into());

    m.insert("report.saved_md".into(), "Markdown 报告已保存到：{}".into());

    m.insert("report.saved_to".into(), "报告已保存到：{}".into());
}
