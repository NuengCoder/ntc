use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("trace.help.title".into(), "ຕິດຕາມເສັ້ນທາງ".into());

    m.insert("trace.help.usage".into(), "ການໃຊ້:".into());

    m.insert(
        "trace.help.usage_line".into(),
        " trace <ໂຮສ|IP> [ຕົວເລືອກ]".into(),
    );

    m.insert("trace.help.options".into(), "ຕົວເລືອກ:".into());

    m.insert(
        "trace.help.opt_max_hops".into(),
        " -m, --max-hops <n> ຈຳນວນ hop ສູງສຸດ (ຄ່າເລີ່ມຕົ້ນ 30)".into(),
    );

    m.insert(
        "trace.help.opt_timeout".into(),
        " -w, --timeout <ມິລລິວິນາທີ> ເວລາລໍຕໍ່ hop".into(),
    );

    m.insert(
        "trace.help.opt_queries".into(),
        " -q, --queries <n> ຈຳນວນການທົດສອບຕໍ່ hop (traceroute)".into(),
    );

    m.insert(
        "trace.help.opt_numeric".into(),
        " -n, --numeric ບໍ່ແປງຊື່ໂຮສ".into(),
    );

    m.insert(
        "trace.help.opt_family".into(),
        " -4, -6 ບັງຄັບ IPv4 / IPv6".into(),
    );

    m.insert(
        "trace.help.opt_no_fallback".into(),
        " --no-fallback ບໍ່ໃຊ້ວິທີ TTL ແຕ່ລະ hop".into(),
    );

    m.insert(
        "trace.help.opt_help".into(),
        " -h, --help ສະແດງການຊ່ວຍເຫຼືອ".into(),
    );

    m.insert("trace.help.examples".into(), "ຕົວຢ່າງ:".into());

    m.insert("netdiag.missing_value".into(), "ຂາດຄ່າຂອງ {}".into());

    m.insert("netdiag.invalid_value".into(), "ຄ່າບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert("netdiag.unknown_option".into(), "ຕົວເລືອກທີ່ບໍ່ຮູ້ຈັກ: {}".into());

    m.insert("netdiag.unexpected_arg".into(), "ອາກິວເມັນເກີນ: {}".into());

    m.insert("netdiag.host_required".into(), "ຕ້ອງລະບຸໂຮສ".into());

    m.insert(
        "netdiag.forever_or_count".into(),
        "ໃຊ້ --forever ຫຼື --count ຢ່າງໃດຢ່າງໜຶ່ງ".into(),
    );

    m.insert("netdiag.spawn_failed".into(), "ເອີ້ນ {} ບໍ່ໄດ້: {}".into());

    m.insert(
        "netdiag.flag_unsupported".into(),
        "ping ຂອງແພລດຟອມນີ້ບໍ່ຮອງຮັບ {} (ຂ້າມ)".into(),
    );

    m.insert(
        "netdiag.trace_missing".into(),
        "ບໍ່ພົບເຄື່ອງມື traceroute (tracert/traceroute/tracepath)".into(),
    );

    m.insert(
        "netdiag.trace_fallback".into(),
        "ໃຊ້ວິທີ TTL ແຕ່ລະ hop ແທນ (ping ຕໍ່ hop) ຊ້າກວ່າ ແລະ hop ທີ່ຖືກກັ່ນຕອງຈະສະແດງ *".into(),
    );

    m.insert(
        "netdiag.trace_title".into(),
        "ກຳລັງຕິດຕາມເສັ້ນທາງໄປ {} (ສູງສຸດ {} hop):".into(),
    );
}
