use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.network.dl".into(),
        "dl <url> ## Download file (--progress, --resume, --stop, --cancel)".into(),
    );

    m.insert(
        "help.network.dns".into(),
        "dns <host> ## DNS lookup (-t record type: A, AAAA, ALL, MX, NS, etc)".into(),
    );

    m.insert(
        "help.network.http".into(),
        "http <method> <url> ## HTTP API client (-H, -d, --json, -q, -a, --bearer, -s, -o, -v, -t)"
            .into(),
    );

    m.insert(
        "help.network.ping".into(),
        "ping <host> ## ICMP ping (-c count)".into(),
    );

    m.insert(
        "help.network.ipcfg".into(),
        "ipcfg [name] ## Show adapters like ipconfig/ifconfig".into(),
    );

    m.insert("help.network.serve".into(), "serve files|http-server|config ## Services: file-drive UI, static CWD server, per-service defaults".into());

    m.insert(
        "help.network.smtp".into(),
        "smtp ## SMTP email client (--id, -i, -u, -pwd, -s, -b, -f, --to)".into(),
    );

    m.insert(
        "help.network.spec".into(),
        "spec ## Show computer/system specifications (CPU, GPU, RAM, storage, OS, serial)".into(),
    );

    m.insert(
        "help.network.sys".into(),
        "sys, top ## Live system monitor (CPU, memory, disk, processes)".into(),
    );

    m.insert(
        "help.network.title".into(),
        "SYSTEM & NETWORK COMMANDS:".into(),
    );

    m.insert(
        "help.network.up".into(),
        "up <file> <url> ## Upload file (--post, -H, -d)".into(),
    );

    m.insert(
        "help.network.example1".into(),
        "sys ## Show system monitor".into(),
    );

    m.insert(
        "help.network.example2".into(),
        "http GET https://api.example.com ## Simple GET request".into(),
    );

    m.insert(
        "help.network.example3".into(),
        "dl https://example.com/file.zip --progress ## Download with progress".into(),
    );

    m.insert(
        "help.network.example4".into(),
        "ping google.com ## Ping google.com".into(),
    );

    m.insert(
        "help.network.example5".into(),
        "dns example.com -t MX ## DNS MX record lookup".into(),
    );

    m.insert(
        "help.network.example6".into(),
        "serve files --port 8080 ## Start files drive server on port 8080".into(),
    );
}
