use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.igcare.care_dir".into(),
        "care dir <name>[, ...] ## Stop ignoring directories".into(),
    );

    m.insert(
        "help.igcare.care_ext".into(),
        "care ext <ext>[, ...] ## Add extra supported extensions".into(),
    );

    m.insert(
        "help.igcare.care_file".into(),
        "care file <name>[, ...] ## Add extra supported files".into(),
    );

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## Export all settings to <name>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## Select categories to export".into(),
    );

    m.insert(
        "help.igcare.ignore_dir".into(),
        "ignore dir <name>[, ...] ## Ignore directories by name".into(),
    );

    m.insert(
        "help.igcare.ignore_ext".into(),
        "ignore ext <ext>[, ...] ## Ignore file extensions".into(),
    );

    m.insert(
        "help.igcare.ignore_file".into(),
        "ignore file <name>[, ...] ## Ignore specific files".into(),
    );

    m.insert(
        "help.igcare.import".into(),
        "igcare import <file> ## Import settings from .ntc.igcare file".into(),
    );

    m.insert(
        "help.igcare.make".into(),
        "igcare make [name] ## Create igcare template in CWD".into(),
    );

    m.insert(
        "help.igcare.show".into(),
        "igcared ## Show all ignore/care settings".into(),
    );

    m.insert("help.igcare.title".into(), "IGNORE / CARE COMMANDS:".into());

    m.insert(
        "help.igcare.example1".into(),
        "ignore dir node_modules ## Ignore node_modules".into(),
    );

    m.insert(
        "help.igcare.example2".into(),
        "ignore ext .log ## Ignore .log files".into(),
    );

    m.insert(
        "help.igcare.example3".into(),
        "care dir src ## Un-ignore src directory".into(),
    );

    m.insert(
        "help.igcare.example4".into(),
        "igcared ## Show all ignore/care settings".into(),
    );
}
