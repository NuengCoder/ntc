use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "report.generating_html".into(),
        "Generating HTML report for: {}".into(),
    );

    m.insert(
        "report.generating_json".into(),
        "Generating JSON report for: {}".into(),
    );

    m.insert(
        "report.generating_md".into(),
        "Generating Markdown report for: {}".into(),
    );

    m.insert(
        "report.saved_html".into(),
        "HTML report saved to: {}".into(),
    );

    m.insert(
        "report.saved_json".into(),
        "JSON report saved to: {}".into(),
    );

    m.insert(
        "report.saved_md".into(),
        "Markdown report saved to: {}".into(),
    );

    m.insert("report.saved_to".into(), "Report saved to: {}".into());
}
