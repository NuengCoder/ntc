use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcare.title".into(), "IGCARE COMMANDS:".into());

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## Export all settings to <name>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## Select categories to export".into(),
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
        "help.igcare.example1".into(),
        "igcare make ## Create an igcare template".into(),
    );
}
