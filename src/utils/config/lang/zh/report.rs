use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "report.generating_html".into(),
        "\u{1F4CA} 正在生成 {} 的 HTML 报告".into(),
    );

    m.insert(
        "report.generating_json".into(),
        "\u{1F4CA} 正在生成 {} 的 JSON 报告".into(),
    );

    m.insert(
        "report.generating_md".into(),
        "\u{1F4CA} 正在生成 {} 的 Markdown 报告".into(),
    );

    m.insert(
        "report.saved_html".into(),
        "\u{2705} HTML 报告已保存到：{}".into(),
    );

    m.insert(
        "report.saved_json".into(),
        "\u{2705} JSON 报告已保存到：{}".into(),
    );

    m.insert(
        "report.saved_md".into(),
        "\u{2705} Markdown 报告已保存到：{}".into(),
    );

    m.insert("report.saved_to".into(), "报告已保存到：{}".into());
}
