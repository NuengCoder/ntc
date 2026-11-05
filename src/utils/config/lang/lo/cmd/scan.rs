use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.scan.depth_usage".into(),
        "ວິທີໃຊ້: scan ext <ນາມສະກຸນ...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.scan.ext_usage".into(),
        "ວິທີໃຊ້: scan ext <ນາມສະກຸນ...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.help_ext".into(), "  scan ext <ນາມສະກຸນ...> [-d|--depth <n>] [-s|--size] [--care]  ສະແດງໄຟລ໌ຕາມນາມສະກຸນ (ບໍ່ນັບແຖວ)".into());

    m.insert(
        "cmd.scan.help_files".into(),
        "  scan files [ແບບ] [-d|--depth <n>] [-s|--size] [--care]  ສະແດງໄຟລ໌ຕາມຊື່ (* = ທັງໝົດ)".into(),
    );

    m.insert(
        "cmd.scan.help_dir".into(),
        "  scan dir [ແບບ] [-d|--depth <n>] [-s|--size] [--care]  ສະແດງໄດເຣັກທໍຣີຕາມຊື່ (* = ທັງໝົດ)"
            .into(),
    );

    m.insert("cmd.scan.help_port".into(), "  scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]  ສະແກນ TCP port (ຄ່າເລີ່ມຕົ້ນ localhost, 20 ports ແລກ)".into());

    m.insert("cmd.scan.help_ip".into(), "  scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]  sweep ຫາ hosts ທີ່ alive (ຄ່າເລີ່ມຕົ້ນ /24 ອັດຕະໂນມັດ)".into());

    m.insert("cmd.scan.ip_arp_only".into(), "(arp)".into());

    m.insert("cmd.scan.ip_done".into(), "alive: {} ຈາກ {} ທີ່ sweep".into());

    m.insert("cmd.scan.ip_none".into(), "ບໍ່ພົບ hosts ທີ່ alive ໃນ {}".into());

    m.insert(
        "cmd.scan.ip_start".into(),
        "ກຳລັງ sweep {} ({} ports/host)...".into(),
    );

    m.insert("cmd.scan.ip_title".into(), "hosts ທີ່ alive ໃນ {}:".into());

    m.insert("cmd.scan.missing_ports".into(), "ຂາດຄ່າຫຼັງ '--ports'.".into());

    m.insert(
        "cmd.scan.missing_timeout".into(),
        "ຂາດຄ່າຫຼັງ '--timeout'.".into(),
    );

    m.insert("cmd.scan.missing_top".into(), "ຂາດຄ່າຫຼັງ '--top'.".into());

    m.insert("cmd.scan.invalid_depth".into(), "ຄວາມເລິກບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "cmd.scan.invalid_timeout".into(),
        "ຄ່າ --timeout ບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert(
        "cmd.scan.invalid_top".into(),
        "ຄ່າ --top ບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert("cmd.scan.list_title".into(), "ຜົນສະແກນ: {}".into());

    m.insert("cmd.scan.no_files".into(), "ບໍ່ພົບໄຟລ໌ {}".into());

    m.insert("cmd.scan.port_done".into(), "ເປີດ: {} ຈາກ {} ທີ່ສະແກນ".into());

    m.insert(
        "cmd.scan.files_usage".into(),
        "ວິທີໃຊ້: scan files [ແບບ] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert(
        "cmd.scan.dir_usage".into(),
        "ວິທີໃຊ້: scan dir [ແບບ] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.no_dirs".into(), "ບໍ່ພົບໄດເຣັກທໍຣີ {}".into());

    m.insert("cmd.scan.total_dirs".into(), "ທັງໝົດ: {} ໄດເຣັກທໍຣີ".into());

    m.insert(
        "cmd.scan.total_dirs_size".into(),
        "ທັງໝົດ: {} ໄດເຣັກທໍຣີ ({})".into(),
    );

    m.insert("cmd.scan.port_none".into(), "ບໍ່ມີ port ເປີດຢູ່ {}".into());

    m.insert(
        "cmd.scan.port_start".into(),
        "ກຳລັງສະແກນ {} ({} ports)...".into(),
    );

    m.insert("cmd.scan.port_title".into(), "ports ທີ່ເປີດຢູ່ {}:".into());

    m.insert(
        "cmd.scan.progress".into(),
        "ກຳລັງສະແກນ {} … {}/{} ({}%) · ເປີດ {}".into(),
    );

    m.insert("cmd.scan.total_files".into(), "ທັງໝົດ: {} ໄຟລ໌".into());

    m.insert(
        "cmd.scan.total_files_size".into(),
        "ທັງໝົດ: {} ໄຟລ໌ ({})".into(),
    );

    m.insert(
        "cmd.scan.unknown_option".into(),
        "ບໍ່ຮູ້ຈັກ scan option: {}".into(),
    );

    m.insert(
        "cmd.scan.unknown_sub".into(),
        "ບໍ່ຮູ້ຈັກ scan subcommand: '{}'".into(),
    );

    m.insert(
        "cmd.scan.unexpected_arg".into(),
        "argument ບໍ່ຄາດຄິດ: '{}'".into(),
    );

    m.insert("cmd.scan.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "cmd.scan.help_waep".into(),
        "  scan waep <ໂຮສ|URL>   ຄົ້ນຫາເສັ້ນທາງເວັບ (endpoint discovery)".into(),
    );

    m.insert(
        "cmd.scan.waep_usage".into(),
        "ການໃຊ້: scan waep <ໂຮສ|ໂຮສ:ພອດ|URL[/base]> [ຕົວເລືອກ] (ເບິ່ງ: scan help)".into(),
    );

    m.insert("cmd.scan.waep_bad_target".into(), "ເປົ້າໝາຍບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "cmd.scan.waep_bad_method".into(),
        "method ບໍ່ຖືກຕ້ອງ: {} (ໃຊ້ GET ຫຼື HEAD)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_status".into(),
        "ລາຍການ status ບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_depth".into(),
        "ຄວາມເລິກບໍ່ຖືກຕ້ອງ: {} (1-4)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_concurrency".into(),
        "ຈຳນວນພ້ອມກັນບໍ່ຖືກຕ້ອງ: {} (1-64)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_timeout".into(),
        "ເວລາໝົດບໍ່ຖືກຕ້ອງ: {} (>=50 ms)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_header".into(),
        "header ບໍ່ຖືກຕ້ອງ: {} (ຮູບແບບ 'Name: value')".into(),
    );

    m.insert(
        "cmd.scan.waep_start".into(),
        "ກຳລັງກວດ {} ດ້ວຍ {} ເສັ້ນທາງ...".into(),
    );

    m.insert(
        "cmd.scan.waep_none".into(),
        "ບໍ່ມີເສັ້ນທາງໃດຕອບສະໜອງໃນ {}".into(),
    );

    m.insert("cmd.scan.waep_summary".into(), "ພົບ {} ລາຍການ".into());

    m.insert("cmd.scan.waep_elapsed".into(), "ໃຊ້ເວລາ: {} ms".into());

    m.insert("cmd.scan.waep_saved".into(), "ບັນທຶກແລ້ວ: {}".into());

    m.insert("cmd.scan.waep_save_failed".into(), "ບັນທຶກ {} ບໍ່ໄດ້: {}".into());

    m.insert(
        "cmd.scan.waep_wordlist_failed".into(),
        "ອ່ານ wordlist {} ບໍ່ໄດ້: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_empty_wordlist".into(),
        "wordlist {} ບໍ່ມີລາຍການທີ່ໃຊ້ໄດ້".into(),
    );

    m.insert("cmd.scan.waep_flags_title".into(), "ຕົວເລືອກ:".into());

    m.insert(
        "cmd.scan.waep_flags".into(),
        "-w, --wordlist <ຟາຍ>      ໃຊ້ wordlist ຂອງຕົນເອງ (ໜຶ່ງເສັ້ນທາງຕໍ່ແຖວ)\n  -x, --ext <ນາມສະກຸນ,...>  ທົດສອບ word.ext ສຳລັບແຕ່ລະນາມສະກຸນ\n  -m, --method GET|HEAD      HTTP method (ຄ່າເລີ່ມຕົ້ນ GET)\n  -s, --status <ລະຫັດ,...>   ລະຫັດສະຖານະທີ່ຖືວ່າພົບ, ຄັ່ນດ້ວຍຈຸນພາກ\n      --all                  ສະແດງທຸກສະຖານະ ລວມ 404\n  -r, --recursive            ລົງໄປໃນເສັ້ນທາງທີ່ພົບ\n  -d, --depth <1-4>          ຄວາມເລິກ (ຄ່າເລີ່ມຕົ້ນ 1)\n  -c, --concurrency <1-64>   ຈຳນວນ probe ພ້ອມກັນ (ຄ່າເລີ່ມຕົ້ນ 16)\n      --timeout <ms>         ເວລາລໍຕໍ່ probe (ຄ່າເລີ່ມຕົ້ນ 4000)\n  -H, --header ຊື່:ຄ່າ      ເພີ່ມ header (ໃຊ້ຊ້ຳໄດ້)\n  -A, --user-agent <ua>      User-Agent header\n  -o, --output <ຟາຍ>        ບັນທຶກຜົນລົງຟາຍ\n  -q, --quiet                ສະແດງສະເພາະສະຫຼຸບ\n  -h, --help                 ສະແດງຄວາມຊ່ວຍເຫຼືອນີ້".into(),
    );

    m.insert("cmd.scan.waep_options_title".into(), "ຄຳອະທິບາຍ:".into());

    m.insert(
        "cmd.scan.waep_options".into(),
        "target        host, host:port, url ຫຼື url/base (scheme ເລີ່ມຕົ້ນ http)\n  paths         ສະແດງທຽບກັບ base: .../api ຈະສະແດງ /files ບໍ່ແມ່ນ /api/files\n  ສະຖານະ 405  ຖືວ່າພົບ (ເສັ້ນທາງມີຢູ່ ແຕ່ method ນີ້ບໍ່ຮອງຮັບ)\n  recursion     ລົງໄປສະເພາະທີ່ເບິ່ງຄືໂຟນເດີ (ບໍ່ມີຈຸດ)".into(),
    );

    m.insert(
        "cmd.scan.waep_unreachable".into(),
        "ບໍ່ມີການຕອບສະໜອງຈາກ {} - ກວດເບິ່ງໂຮສ, ພອດ ແລະ scheme (DNS/ການເຊື່ອມຕໍ່/TLS ລົ້ມເຫຼວ)".into(),
    );

    m.insert(
        "cmd.scan.waep_some_failed".into(),
        "{} ຈາກ {} probe ບໍ່ມີການຕອບສະໜອງ (ໝົດເວລາ ຫຼື ຖືກປະຕິເສດການເຊື່ອມຕໍ່)".into(),
    );
}
