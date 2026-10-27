use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.scan.depth".into(),
        "scan ext <ext...> -d, --depth <n> ## Limit scan depth".into(),
    );

    m.insert(
        "help.scan.example1".into(),
        "scan ext rs ## List all .rs files".into(),
    );

    m.insert(
        "help.scan.example2".into(),
        "scan ext rs c -d 2 ## List .rs/.c files up to depth 2".into(),
    );

    m.insert(
        "help.scan.example3".into(),
        "scan ext rs c -s ## List with sizes + total".into(),
    );

    m.insert(
        "help.scan.example4".into(),
        "scan port 192.168.1.1 --top 10 ## Scan top 10 ports".into(),
    );

    m.insert(
        "help.scan.example5".into(),
        "scan ip 192.168.1.0/24 ## Sweep for live hosts".into(),
    );

    m.insert(
        "help.scan.example6".into(),
        "scan files *.rs ## List files by name pattern".into(),
    );

    m.insert(
        "help.scan.example7".into(),
        "scan dir k* ## List directories by name pattern".into(),
    );

    m.insert(
        "help.scan.ext".into(),
        "scan ext <ext...> ## List files by extension (no line counts)".into(),
    );

    m.insert(
        "help.scan.files".into(),
        "scan files [pattern] ## List files by name pattern (* = all)".into(),
    );

    m.insert(
        "help.scan.dir".into(),
        "scan dir [pattern] ## List directories by name pattern (* = all)".into(),
    );

    m.insert(
        "help.scan.port".into(),
        "scan port [host] [--top N | --ports <spec> | --all] ## TCP port scan".into(),
    );

    m.insert(
        "help.scan.ip".into(),
        "scan ip [cidr] [--top N | --ports <spec>] ## Live-host sweep + ARP".into(),
    );

    m.insert(
        "help.scan.size".into(),
        "scan ext <ext...> -s, --size ## Show sizes + total".into(),
    );

    m.insert(
        "help.scan.care".into(),
        "scan ext <ext...> --care ## Include everything (care mode)".into(),
    );

    m.insert("help.scan.title".into(), "SCAN COMMANDS:".into());
}
