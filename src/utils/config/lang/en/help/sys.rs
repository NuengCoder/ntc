use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.sys.title".into(), "SYSTEM COMMANDS:".into());

    m.insert(
        "help.sys.monitor".into(),
        "sys, top [-l|--log] [-d|--delay [seconds:-1]] ## Live system monitor (CPU, memory, disk, processes)".into(),
    );

    m.insert(
        "help.sys.passthrough".into(),
        "<any command> ## NTC passes unknown commands to your system".into(),
    );

    m.insert(
        "help.sys.native".into(),
        "shell -- echo, git, dir/ls, ping, etc. all run natively alongside NTC commands.".into(),
    );

    m.insert(
        "help.sys.example1".into(),
        "sys ## Show system monitor".into(),
    );

    m.insert(
        "help.sys.example2".into(),
        "sys --log --delay 5 ## Append CPU/memory/disk stats every 5s (default: 15s) to log/ntc_sys.log"
            .into(),
    );

    m.insert(
        "help.sys.example3".into(),
        "sys -ld 5 ## Same as 'sys -l -d 5': -l/--log switches logging on, -d/--delay takes the seconds"
            .into(),
    );
}
