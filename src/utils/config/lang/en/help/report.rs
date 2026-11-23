use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.report.title".into(), "REPORT COMMANDS:".into());

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## Project report saved under <file>".into(),
    );

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## Show clipboard history entry in full".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report project html . --name docs/project ## HTML report with a collapsible tree saved as docs/project.html".into(),
    );

    m.insert("help.report.example2".into(), "report clip 1".into());
}
