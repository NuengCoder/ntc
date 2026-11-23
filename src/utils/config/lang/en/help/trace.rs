use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trace.title".into(), "TRACE COMMAND:".into());

    m.insert(
        "help.trace.usage".into(),
        "trace <host> [-m N] [-w MS] ## Hop-by-hop route (tracert/traceroute, TTL-walk fallback)"
            .into(),
    );

    m.insert(
        "help.trace.example1".into(),
        "trace example.com ## Trace the route to example.com".into(),
    );
}
