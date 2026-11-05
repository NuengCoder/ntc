use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.scan.depth_usage".into(),
        "Usage: scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.examples_title".into(), "Examples:".into());

    m.insert(
        "cmd.scan.ext_usage".into(),
        "Usage: scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.help_ext".into(), "  scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]  List files by extension (no line counts)".into());

    m.insert("cmd.scan.help_files".into(), "  scan files [pattern] [-d|--depth <n>] [-s|--size] [--care]  List files by name pattern (* matches all)".into());

    m.insert("cmd.scan.help_dir".into(), "  scan dir [pattern] [-d|--depth <n>] [-s|--size] [--care]  List directories by name pattern (* matches all)".into());

    m.insert("cmd.scan.help_port".into(), "  scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]  TCP port scan (default host 127.0.0.1, top 20)".into());

    m.insert("cmd.scan.help_ip".into(), "  scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]  Live-host sweep (default auto local /24)".into());

    m.insert("cmd.scan.ip_arp_only".into(), "(arp)".into());

    m.insert("cmd.scan.ip_done".into(), "Live: {} of {} swept.".into());

    m.insert("cmd.scan.ip_none".into(), "No live hosts in {}.".into());

    m.insert(
        "cmd.scan.ip_start".into(),
        "Sweeping {} ({} ports/host)...".into(),
    );

    m.insert("cmd.scan.ip_title".into(), "Live hosts in {}:".into());

    m.insert(
        "cmd.scan.missing_ports".into(),
        "Missing value after '--ports'.".into(),
    );

    m.insert(
        "cmd.scan.missing_timeout".into(),
        "Missing value after '--timeout'.".into(),
    );

    m.insert(
        "cmd.scan.missing_top".into(),
        "Missing value after '--top'.".into(),
    );

    m.insert("cmd.scan.invalid_depth".into(), "Invalid depth: {}".into());

    m.insert(
        "cmd.scan.invalid_timeout".into(),
        "Invalid --timeout value: '{}'.".into(),
    );

    m.insert(
        "cmd.scan.invalid_top".into(),
        "Invalid --top value: '{}'.".into(),
    );

    m.insert("cmd.scan.list_title".into(), "SCAN: {}".into());

    m.insert("cmd.scan.no_files".into(), "No {} files found".into());

    m.insert(
        "cmd.scan.port_done".into(),
        "Open: {} of {} scanned.".into(),
    );

    m.insert(
        "cmd.scan.files_usage".into(),
        "Usage: scan files [pattern] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert(
        "cmd.scan.dir_usage".into(),
        "Usage: scan dir [pattern] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.no_dirs".into(), "No {} directories found".into());

    m.insert("cmd.scan.total_dirs".into(), "Total: {} directories".into());

    m.insert(
        "cmd.scan.total_dirs_size".into(),
        "Total: {} directories ({})".into(),
    );

    m.insert("cmd.scan.port_none".into(), "No open ports on {}.".into());

    m.insert(
        "cmd.scan.port_start".into(),
        "Scanning {} ({} ports)...".into(),
    );

    m.insert("cmd.scan.port_title".into(), "Open ports on {}:".into());

    m.insert(
        "cmd.scan.progress".into(),
        "Scanning {} … {}/{} ({}%) · {} open".into(),
    );

    m.insert("cmd.scan.total_files".into(), "Total: {} files".into());

    m.insert(
        "cmd.scan.total_files_size".into(),
        "Total: {} files ({})".into(),
    );

    m.insert(
        "cmd.scan.unknown_option".into(),
        "Unknown scan option: {}".into(),
    );

    m.insert(
        "cmd.scan.unknown_sub".into(),
        "Unknown scan subcommand: '{}'".into(),
    );

    m.insert(
        "cmd.scan.unexpected_arg".into(),
        "Unexpected argument: '{}'.".into(),
    );

    m.insert("cmd.scan.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.scan.help_waep".into(),
        "  scan waep <host|url>    Enumerate web paths (endpoint discovery)".into(),
    );

    m.insert(
        "cmd.scan.waep_usage".into(),
        "Usage: scan waep <host|host:port|url[/base]> [flags]  (see: scan help)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_target".into(),
        "Invalid target: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_method".into(),
        "Invalid method: {} (use GET or HEAD)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_status".into(),
        "Invalid status list: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_depth".into(),
        "Invalid depth: {} (1-4)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_concurrency".into(),
        "Invalid concurrency: {} (1-64)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_timeout".into(),
        "Invalid timeout: {} (>=50 ms)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_header".into(),
        "Invalid header: {} (use 'Name: value')".into(),
    );

    m.insert(
        "cmd.scan.waep_start".into(),
        "Probing {} with {} path(s)...".into(),
    );

    m.insert(
        "cmd.scan.waep_none".into(),
        "No paths answered on {}".into(),
    );

    m.insert("cmd.scan.waep_summary".into(), "{} hit(s)".into());

    m.insert("cmd.scan.waep_elapsed".into(), "elapsed: {} ms".into());

    m.insert("cmd.scan.waep_saved".into(), "Saved: {}".into());

    m.insert(
        "cmd.scan.waep_save_failed".into(),
        "Cannot save {}: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_wordlist_failed".into(),
        "Cannot read wordlist {}: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_empty_wordlist".into(),
        "Wordlist {} has no usable entries".into(),
    );

    m.insert("cmd.scan.waep_flags_title".into(), "Flags:".into());

    m.insert(
        "cmd.scan.waep_flags".into(),
        "-w, --wordlist <file>      use a custom wordlist (one path per line)\n  -x, --ext <ext,...>        also probe word.ext for each extension\n  -m, --method GET|HEAD      HTTP method to use (default GET)\n  -s, --status <codes>       accepted status codes, comma separated\n      --all                  show every status, including 404\n  -r, --recursive            descend into discovered paths\n  -d, --depth <1-4>          recursion depth (default 1)\n  -c, --concurrency <1-64>   parallel probes (default 16)\n      --timeout <ms>         per-probe timeout in ms (default 4000)\n  -H, --header \"Name: value\" extra request header (repeatable)\n  -A, --user-agent <ua>      User-Agent header\n  -o, --output <file>        save hits to a file\n  -q, --quiet                only print the summary\n  -h, --help                 this help".into(),
    );

    m.insert("cmd.scan.waep_options_title".into(), "Options:".into());

    m.insert(
        "cmd.scan.waep_options".into(),
        "target        host, host:port, url or url/base (default scheme http)\n  paths         shown relative to the base: .../api reports /files, not /api/files\n  status 405    counts as a hit (the path exists, the method is not allowed)\n  recursion     only directory-like hits (no dot) are descended into".into(),
    );

    m.insert(
        "cmd.scan.waep_unreachable".into(),
        "No response from {} - check the host, port and scheme (DNS/connection/TLS failed)".into(),
    );

    m.insert(
        "cmd.scan.waep_some_failed".into(),
        "{} of {} probes got no response (timeout or connection refused)".into(),
    );
}
