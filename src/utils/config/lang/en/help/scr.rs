use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.scr.continue".into(),
        "scr -c <file.ntc> [args...] ## Run script, continue after errors".into(),
    );

    m.insert(
        "help.scr.example1".into(),
        "scr deploy.ntc ## Run deploy.ntc".into(),
    );

    m.insert(
        "help.scr.example2".into(),
        "scr build.ntc dev prod ## Pass args $1=dev, $2=prod".into(),
    );

    m.insert(
        "help.scr.example3".into(),
        "scr -c setup.ntc ## Run and continue after errors".into(),
    );

    m.insert("help.scr.help".into(), "scr help ## Show scr usage".into());

    m.insert(
        "help.scr.init".into(),
        "scr init [name] ## Create a script template".into(),
    );

    m.insert(
        "help.scr.ls".into(),
        "scr ls ## List *.ntc scripts in the current directory".into(),
    );

    m.insert(
        "help.scr.run".into(),
        "scr <file.ntc> [args...] ## Run an automation script".into(),
    );

    m.insert(
        "help.scr.title".into(),
        "SCR / AUTOMATION SCRIPT COMMANDS:".into(),
    );
}
