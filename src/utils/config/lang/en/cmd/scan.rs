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
}
