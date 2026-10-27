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
}
