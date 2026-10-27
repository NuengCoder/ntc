use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.tl.dir_not_found".into(), "ບໍ່ພົບໄດເຣັກທໍຣີ: {}".into());

    m.insert("cmd.tl.dir_usage".into(), "ວິທີໃຊ້: tl dir <path>".into());

    m.insert(
        "cmd.tl.ext_not_supported".into(),
        ".{} ບໍ່ຮອງຮັບ — ຖືກບລັອກ".into(),
    );

    m.insert("cmd.tl.ext_usage".into(), "ວິທີໃຊ້: tl ext <extension>".into());

    m.insert("cmd.tl.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: {}".into());

    m.insert("cmd.tl.file_usage".into(), "ວິທີໃຊ້: tl file <path>".into());

    m.insert(
        "cmd.tl.help_dir".into(),
        "  tl dir <path>           ສະແດງນາມສະກຸນທີ່ຮອງຮັບ ແລະ ຈຳນວນແຖວທັງໝົດໃນໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "cmd.tl.help_ext".into(),
        "  tl ext <extension>      ສະແດງໄຟລ໌ທັງໝົດທີ່ມີນາມສະກຸນນັ້ນ ແລະ ຈຳນວນແຖວທັງໝົດ".into(),
    );

    m.insert(
        "cmd.tl.help_file".into(),
        "  tl file <path>          ນັບແຖວໃນໄຟລ໌ດຽວ".into(),
    );

    m.insert("cmd.tl.help_sort".into(), "  tl ext|dir <target> -s <-n|-l|-r>   ຈັດລຽງຜົນລັບ (-n ປົກກະຕິ, -l ຕາມແຖວໜ້ອຍ→ຫຼາຍ, -r ຕາມແຖວຫຼາຍ→ໜ້ອຍ)".into());

    m.insert("cmd.tl.highest".into(), "ສູງສຸດ: {}  {} ແຖວ".into());

    m.insert("cmd.tl.no_ext_files".into(), "ບໍ່ພົບໄຟລ໌ .{}".into());

    m.insert(
        "cmd.tl.no_supported_files".into(),
        "ບໍ່ພົບໄຟລ໌ທີ່ຮອງຮັບໃນ {}".into(),
    );

    m.insert("cmd.tl.not_a_file".into(), "ບໍ່ແມ່ນໄຟລ໌: {}".into());

    m.insert(
        "cmd.tl.sort_usage".into(),
        "ແທັກຈັດລຽງບໍ່ຖືກຕ້ອງ. ໃຊ້: -s -n (ປົກກະຕິ) | -s -l (ຕາມແຖວ, ໜ້ອຍ→ຫຼາຍ) | -s -r (ຍ້ອນຫຼັງ, ຫຼາຍ→ໜ້ອຍ)."
            .into(),
    );

    m.insert(
        "cmd.tl.total_files_lines".into(),
        "ທັງໝົດ: {} ໄຟລ໌, {} ແຖວ".into(),
    );

    m.insert("cmd.tl.unknown_sub".into(), "ບໍ່ຮູ້ຈັກ subcommand '{}'.".into());

    m.insert("cmd.tl.usage_title".into(), "ວິທີໃຊ້:".into());
}
