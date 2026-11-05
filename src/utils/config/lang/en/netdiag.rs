use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("trace.help.title".into(), "Trace".into());

    m.insert("trace.help.usage".into(), "Usage:".into());

    m.insert(
        "trace.help.usage_line".into(),
        " trace <host|ip> [options]".into(),
    );

    m.insert("trace.help.options".into(), "Options:".into());

    m.insert(
        "trace.help.opt_max_hops".into(),
        " -m, --max-hops <n> Maximum hops (default 30)".into(),
    );

    m.insert(
        "trace.help.opt_timeout".into(),
        " -w, --timeout <ms> Per-hop timeout in milliseconds".into(),
    );

    m.insert(
        "trace.help.opt_queries".into(),
        " -q, --queries <n> Probes per hop (traceroute)".into(),
    );

    m.insert(
        "trace.help.opt_numeric".into(),
        " -n, --numeric Do not resolve host names".into(),
    );

    m.insert(
        "trace.help.opt_family".into(),
        " -4, -6 Force IPv4 / IPv6".into(),
    );

    m.insert(
        "trace.help.opt_no_fallback".into(),
        " --no-fallback Do not fall back to a TTL walk".into(),
    );

    m.insert(
        "trace.help.opt_help".into(),
        " -h, --help Show this help".into(),
    );

    m.insert("trace.help.examples".into(), "Examples:".into());

    m.insert(
        "netdiag.missing_value".into(),
        "Missing value for {}".into(),
    );

    m.insert("netdiag.invalid_value".into(), "Invalid value: {}".into());

    m.insert("netdiag.unknown_option".into(), "Unknown option: {}".into());

    m.insert(
        "netdiag.unexpected_arg".into(),
        "Unexpected argument: {}".into(),
    );

    m.insert("netdiag.host_required".into(), "A host is required".into());

    m.insert(
        "netdiag.forever_or_count".into(),
        "Use either --forever or --count, not both".into(),
    );

    m.insert("netdiag.spawn_failed".into(), "Cannot run {}: {}".into());

    m.insert(
        "netdiag.flag_unsupported".into(),
        "{} is not supported by this platform's ping; ignored".into(),
    );

    m.insert(
        "netdiag.trace_missing".into(),
        "No traceroute tool found (tracert/traceroute/tracepath)".into(),
    );

    m.insert(
        "netdiag.trace_fallback".into(),
        "Falling back to a TTL walk (one ping per hop); it is slower and filtered hops show *"
            .into(),
    );

    m.insert(
        "netdiag.trace_title".into(),
        "Tracing route to {} (max {} hops):".into(),
    );
}
