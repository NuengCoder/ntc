use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtc.title".into(), "TXTC COMMAND:".into());

    m.insert(
        "help.txtc.usage".into(),
        "txtc [--line|--no-line] <file> ## Copy file content to clipboard (line numbers on/off)"
            .into(),
    );

    m.insert(
        "help.txtc.example1".into(),
        "txtc main.rs ## Copy main.rs to the clipboard".into(),
    );
}
