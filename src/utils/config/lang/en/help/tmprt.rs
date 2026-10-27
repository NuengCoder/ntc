use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.tmprt.title".into(),
        "TEMPERATURE & WEATHER COMMANDS:".into(),
    );

    m.insert(
        "help.tmprt.com".into(),
        "tmprt com [secs] ## Live hardware temperature monitor (q=quit s=freeze c='oC/'oF)".into(),
    );

    m.insert(
        "help.tmprt.weather".into(),
        "tmprt weather [city] ## Current weather via OpenWeather API".into(),
    );

    m.insert(
        "help.tmprt.key".into(),
        "tmprt key [API_KEY] ## Save/display OpenWeather API key".into(),
    );

    m.insert(
        "help.tmprt.city".into(),
        "tmprt city [name] ## Save/display default city for weather".into(),
    );

    m.insert(
        "help.tmprt.sensors".into(),
        "tmprt sensors <status|install|uninstall|debug> ## Manage hardware sensor backends (LHM)"
            .into(),
    );

    m.insert(
        "help.tmprt.example1".into(),
        "tmprt com 5 ## Live view refreshing every 5 seconds".into(),
    );

    m.insert(
        "help.tmprt.example2".into(),
        "tmprt weather Bangkok ## Weather for Bangkok".into(),
    );

    m.insert(
        "help.tmprt.example3".into(),
        "tmprt key 1234567890abcdef ## Save API key".into(),
    );

    m.insert(
        "help.tmprt.example4".into(),
        "tmprt city Bangkok ## Set default city".into(),
    );
}
