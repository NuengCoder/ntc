use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.help.title".into(), "HELP ຄຳສັ່ງ:".into());

    m.insert("help.help.usage".into(), "help ## ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into());

    m.insert(
        "help.help.example1".into(),
        "help cd ## ສະແດງວິທີໃຊ້ຂອງແຕ່ລະຄຳສັ່ງ".into(),
    );
    m.insert(
        "help.help.command".into(),
        "help <ຄຳສັ່ງ> ## ສະແດງຄຳອະທິບາຍຂອງແຕ່ລະຄຳສັ່ງ".into(),
    );
    m.insert(
        "help.help.all".into(),
        "help --all ## ສະແດງຄຳອະທິບາຍຂອງທຸກຄຳສັ່ງ".into(),
    );
}
