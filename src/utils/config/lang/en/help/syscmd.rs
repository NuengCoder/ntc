use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.syscmd.any".into(),
        "<any command> ## NTC passes unknown commands to your system".into(),
    );

    m.insert(
        "help.syscmd.native".into(),
        "shell -- echo, git, dir/ls, ping, etc.".into(),
    );

    m.insert(
        "help.syscmd.native2".into(),
        "all run natively alongside NTC commands.".into(),
    );

    m.insert("help.syscmd.title".into(), "SYSTEM COMMANDS:".into());
}
