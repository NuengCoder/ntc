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
    m.insert(
        "help.scan.waep".into(),
        "scan waep <host|url> ## Enumerate web paths (/api, /admin, /upload, ...)".into(),
    );

    m.insert(
        "help.scan.waep_base".into(),
        "scan waep example.com/api -r -d 2 ## Recurse under a base path".into(),
    );

    m.insert(
        "help.scan.waep_flags".into(),
        "scan waep <target> [-w <file>] [-x <ext,...>] [-m GET|HEAD] [-s <codes>] [--all] ## Wordlist, suffixes, method, accepted statuses".into(),
    );

    m.insert(
        "help.scan.waep_recursive".into(),
        "scan waep <target> [-r] [-d 1-4] [-c 1-64] [--timeout <ms>] [-H 'Name: value'] [-A <ua>] [-o <file>] [-q] ## Recursion, speed, headers, output".into(),
    );

    m.insert(
        "help.scan.example8".into(),
        "scan waep http://127.0.0.1:55555/api ## Discover what an ntc serve API exposes".into(),
    );

    m.insert(
        "help.scan.example9".into(),
        "scan waep example.com -x php,html --all ## Show every response plus .php/.html variants"
            .into(),
    );

    m.insert(
        "help.scan.example10".into(),
        "scan waep example.com/api -r -d 2 -c 32 ## Recurse two levels with 32 parallel probes"
            .into(),
    );
}
