use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "report.generating_html".into(),
        "\u{1F4CA} Generating HTML report for: {}".into(),
    );

    m.insert(
        "report.generating_json".into(),
        "\u{1F4CA} Generating JSON report for: {}".into(),
    );

    m.insert(
        "report.generating_md".into(),
        "\u{1F4CA} Generating Markdown report for: {}".into(),
    );

    m.insert(
        "report.saved_html".into(),
        "\u{2705} HTML report saved to: {}".into(),
    );

    m.insert(
        "report.saved_json".into(),
        "\u{2705} JSON report saved to: {}".into(),
    );

    m.insert(
        "report.saved_md".into(),
        "\u{2705} Markdown report saved to: {}".into(),
    );

    m.insert("report.saved_to".into(), "Report saved to: {}".into());
}
