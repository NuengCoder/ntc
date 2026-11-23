use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fsc.title".into(), "FSC COMMAND:".into());

    m.insert(
        "help.fsc.usage".into(),
        "fsc <pattern> [-d <n>] [--no-fuzzy] ## Search files, pick one to display & optionally copy".into(),
    );

    m.insert(
        "help.fsc.example1".into(),
        "fsc config ## Find a matching file and display it".into(),
    );
}
