use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.igcare.create_failed".into(), "ສ້າງໄຟລ໌ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "cmd.igcare.example_export_all".into(),
        "  igcare export --all myproject        # ສ້າງ myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.example_export_select".into(),
        "  igcare export --select myproject     # ເລືອກວ່າຈະສົ່ງອອກໝວດໝູ່ໃດ".into(),
    );

    m.insert(
        "cmd.igcare.example_import".into(),
        "  igcare import myproject.ntc.igcare   # ນຳເຂົ້າໄປໃນການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );

    m.insert("cmd.igcare.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.igcare.export_cmd".into(),
        "igcare export --all <name> ສົ່ງອອກການຕັ້ງຄ່າທັງໝົດໄປຍັງ <name>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_name_empty".into(),
        "ຊື່ສົ່ງອອກບໍ່ສາມາດຫວ່າງເປົ່າໄດ້.".into(),
    );

    m.insert(
        "cmd.igcare.export_select_cmd".into(),
        "igcare export --select <name> ເລືອກໝວດໝູ່ເພື່ອສົ່ງອອກໄປຍັງ <name>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_usage".into(),
        "ວິທີໃຊ້: igcare export --all <name> | igcare export --select <name>".into(),
    );

    m.insert("cmd.igcare.file_exists".into(), "ໄຟລ໌ມີຢູ່ແລ້ວ: {}".into());

    m.insert(
        "cmd.igcare.import_cmd".into(),
        "igcare import <file> ນຳເຂົ້າການຕັ້ງຄ່າຈາກໄຟລ໌ .ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.import_example".into(),
        "ຕົວຢ່າງ: igcare import myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.import_usage".into(),
        "ວິທີໃຊ້: igcare import <file>".into(),
    );

    m.insert("cmd.igcare.main_title".into(), "ຕົວຈັດການ Ignore/Care".into());

    m.insert(
        "cmd.igcare.template_created".into(),
        "ສ້າງ igcare template ແລ້ວ: {}".into(),
    );

    m.insert(
        "cmd.igcare.template_hint".into(),
        "ນີ້ແມ່ນໄຟລ໌ template ທ້ອງຖິ່ນ. ໃຊ້ 'igcare import' ເພື່ອນຳໃຊ້ການຕັ້ງຄ່າ.".into(),
    );

    m.insert(
        "cmd.igcare.type_help".into(),
        "ພິມ 'igcare' ເພື່ອເບິ່ງຄວາມຊ່ວຍເຫຼືອ.".into(),
    );

    m.insert(
        "cmd.igcare.unknown_flag".into(),
        "ແທັກສົ່ງອອກບໍ່ຮູ້ຈັກ: {}. ໃຊ້ --all ຫຼື --select".into(),
    );

    m.insert(
        "cmd.igcare.unknown_subcmd".into(),
        "ບໍ່ຮູ້ຈັກ igcare subcommand: {}".into(),
    );
}
