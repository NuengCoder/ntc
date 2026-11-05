use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ping.help.examples".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "ping.help.opt_count".into(),
        " -c, --count <n> ຈຳນວນ ping (default 4)".into(),
    );

    m.insert(
        "ping.help.opt_help".into(),
        " -h, --help ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into(),
    );

    m.insert("ping.help.options".into(), "Options:".into());

    m.insert("ping.help.title".into(), "Ping".into());

    m.insert("ping.help.usage".into(), "Usage:".into());

    m.insert(
        "ping.help.usage_line".into(),
        " ping <host> [options]".into(),
    );

    m.insert(
        "ping.help.opt_forever".into(),
        " --forever ping ຕໍ່ເນື່ອງຈົນຢຸດ (Ctrl+C)".into(),
    );

    m.insert(
        "ping.help.opt_interval".into(),
        " -i, --interval <ມິລລິວິນາທີ> ໄລຍະຫ່າງລະຫວ່າງ ping (Unix ເທົ່ານັ້ນ)".into(),
    );

    m.insert(
        "ping.help.opt_timeout".into(),
        " -w, --timeout <ມິລລິວິນາທີ> ເວລາລໍຄຳຕອບ".into(),
    );

    m.insert(
        "ping.help.opt_size".into(),
        " -s, --size <ໄບຕ໌> ຂະໜາດ payload".into(),
    );

    m.insert(
        "ping.help.opt_ttl".into(),
        " -t, --ttl <n> ເວລາມີຊີວິດ (TTL)".into(),
    );

    m.insert(
        "ping.help.opt_family".into(),
        " -4, -6 ບັງຄັບ IPv4 / IPv6".into(),
    );

    m.insert(
        "ping.help.opt_quiet".into(),
        " -q, --quiet ສະແດງຜົນແບບງຽບ (Unix ເທົ່ານັ້ນ)".into(),
    );
}
