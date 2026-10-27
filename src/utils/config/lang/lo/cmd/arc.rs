use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.arc.create_usage".into(),
        "ວິທີໃຊ້: arc create <out.zip|tar.gz|tgz|tar> <ໄຟລ໌...>".into(),
    );

    m.insert(
        "cmd.arc.extract_usage".into(),
        "ວິທີໃຊ້: arc extract <ໄຟລ໌> [ປາຍທາງ]".into(),
    );

    m.insert("cmd.arc.list_usage".into(), "ວິທີໃຊ້: arc list <ໄຟລ໌>".into());

    m.insert(
        "cmd.arc.bad_ext".into(),
        "ປະເພດໄຟລ໌ບໍ່ຮອງຮັບ (ໃຊ້ .zip, .tar.gz/.tgz, .tar)".into(),
    );

    m.insert("cmd.arc.not_found".into(), "ບໍ່ພົບ: {}".into());

    m.insert("cmd.arc.no_inputs".into(), "ບໍ່ມີໄຟລ໌ນຳເຂົ້າສຳລັບບີบอัด.".into());

    m.insert("cmd.arc.created".into(), "ສ້າງໄຟລ໌ແລ້ວ: {} ({} ລາຍການ)".into());

    m.insert("cmd.arc.extracted".into(), "ແຕກໄຟລ໌ {} ໄຟລ໌ໄປ {}".into());

    m.insert("cmd.arc.list_title".into(), "ໄຟລ໌ບີບอัด: {}".into());

    m.insert("cmd.arc.total".into(), "ທັງໝົດ: {} ລາຍການ".into());

    m.insert(
        "cmd.arc.unknown_sub".into(),
        "ບໍ່ຮູ້ຈັກ arc subcommand: '{}'".into(),
    );

    m.insert("cmd.arc.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "cmd.arc.help_create".into(),
        "  arc create <ອອກ> <ໄຟລ໌...>  ສ້າງໄຟລ໌ບີບอัด (.zip, .tar.gz, .tar)".into(),
    );

    m.insert(
        "cmd.arc.help_extract".into(),
        "  arc extract <ໄຟລ໌> [ປາຍທາງ]   ແຕກໄຟລ໌".into(),
    );

    m.insert(
        "cmd.arc.help_list".into(),
        "  arc list <ໄຟລ໌>              ສະແດງເນື້ອໃນ".into(),
    );

    m.insert("cmd.arc.examples_title".into(), "ຕົວຢ່າງ:".into());
}
