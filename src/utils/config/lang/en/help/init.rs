use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.init.title".into(), "INIT COMMAND:".into());

    m.insert(
        "help.init.usage".into(),
        "init [all|ran|local|opencode|project] [-f|--force] ## Initialize project files (NTCRANFILE, ntconfig.toml, .opencode/, project scaffold)".into(),
    );

    m.insert(
        "help.init.example1".into(),
        "init ran ## Initialize NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.init.example2".into(),
        "init project my-app --no lc,oc,docker ## Scaffold without LICENSE, OpenCode, Docker"
            .into(),
    );
}
