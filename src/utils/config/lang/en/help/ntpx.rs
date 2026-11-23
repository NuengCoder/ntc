use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ntpx.title".into(), "NTPX COMMAND:".into());

    m.insert(
        "help.ntpx.usage".into(),
        "ntpx <pkg> [bin] [args] ## Run an installed package's binary".into(),
    );

    m.insert(
        "help.ntpx.example1".into(),
        "ntpx node npm init ## Run npm via ntpx".into(),
    );
}
