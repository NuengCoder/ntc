use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.report.cp".into(),
        "report <fmt> --cp ## Copy report to clipboard instead of saving".into(),
    );

    m.insert(
        "help.report.docx".into(),
        "report docx [path] ## Generate DOCX report".into(),
    );

    m.insert(
        "help.report.html".into(),
        "report html [path] ## Generate HTML report".into(),
    );

    m.insert(
        "help.report.json".into(),
        "report json [path] ## Generate JSON report".into(),
    );

    m.insert(
        "help.report.md".into(),
        "report md [path] ## Generate Markdown report".into(),
    );

    m.insert(
        "help.report.pdf".into(),
        "report pdf [path] ## Generate PDF report".into(),
    );

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## Project report saved under <file>".into(),
    );

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## Show clipboard history entry in full".into(),
    );

    m.insert("help.report.clip_example".into(), "report clip 1".into());

    m.insert("help.report.project_example".into(), "report project html . --name docs/project ## HTML report with a collapsible tree saved as docs/project.html".into());

    m.insert("help.report.title".into(), "REPORT COMMANDS:".into());

    m.insert(
        "help.report.txt".into(),
        "report txt [path] ## Generate TXT report".into(),
    );

    m.insert(
        "help.report.txtc".into(),
        "txtc <file> ## Copy file content to clipboard".into(),
    );

    m.insert("help.report.txtf".into(), "txtf [--head N|--tail N|--betw A:B] [--line|--no-line] [file|dir|@tp/file|@tp/dir] ## Display file content or pick a file from a directory (--line/--no-line override the line-number setting)".into());

    m.insert(
        "help.report.xlsx".into(),
        "report xlsx [path] ## Generate XLSX report (with dependency analysis)".into(),
    );

    m.insert(
        "help.report.txtf_example1".into(),
        "txtf main.rs --head 30 ## First 30 lines of main.rs".into(),
    );

    m.insert(
        "help.report.txtf_example2".into(),
        "txtf --betw 10:20 main.rs ## Lines 10-20 of main.rs".into(),
    );

    m.insert(
        "help.report.txtf_example3".into(),
        "txtf @docs/notes.txt ## Display a teleport savepoint file".into(),
    );

    m.insert(
        "help.report.txtf_example4".into(),
        "txtf src ## Pick a supported file from src to display".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report txt ## Generate TXT report".into(),
    );

    m.insert(
        "help.report.example2".into(),
        "report html --cp ## Copy HTML report to clipboard".into(),
    );

    m.insert(
        "help.report.example3".into(),
        "report pdf ## Generate PDF report".into(),
    );

    m.insert(
        "help.report.example4".into(),
        "txtc file.txt ## Copy file content to clipboard".into(),
    );
}
