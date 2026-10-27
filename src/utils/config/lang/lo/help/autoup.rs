use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.autoup.add".into(),
        "autoup add <name> \"<cmd>\" ## ເພີ່ມ startup entry ໃສ່ HKCU".into(),
    );

    m.insert(
        "help.autoup.disable".into(),
        "autoup disable <name> ## ປິດໃຊ້ງານ entry (ໃຊ້ StartupApproved)".into(),
    );

    m.insert(
        "help.autoup.enable".into(),
        "autoup enable <name> ## ເປີດໃຊ້ງານ entry ທີ່ຖືກປິດຄືນ".into(),
    );

    m.insert(
        "help.autoup.list".into(),
        "autoup list ## ລາຍຊື່ startup entries (CU/LM/SF sources)".into(),
    );

    m.insert(
        "help.autoup.rm".into(),
        "autoup rm <name> ## ລຶບ startup entry".into(),
    );

    m.insert(
        "help.autoup.sources".into(),
        "[CU]=HKCU [LM]=HKLM [SF]=Startup Folder".into(),
    );

    m.insert(
        "help.autoup.title".into(),
        "ການຈັດການ STARTUP (Windows):".into(),
    );

    m.insert(
        "help.autoup.example1".into(),
        "autoup list ## ລາຍຊື່ startup entries".into(),
    );

    m.insert(
        "help.autoup.example2".into(),
        "autoup add ntc \"C:\\ntc\\ntc.exe\" ## ເພີ່ມໃສ່ startup".into(),
    );

    m.insert(
        "help.autoup.example3".into(),
        "autoup rm ntc ## ລຶບ startup entry".into(),
    );

    m.insert(
        "help.autoup.example4".into(),
        "autoup disable ntc ## ປິດໃຊ້ງານ startup entry".into(),
    );
}
