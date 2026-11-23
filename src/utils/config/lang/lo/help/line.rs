use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.line.title".into(), "LINE ຄຳສັ່ງ:".into());

    m.insert(
        "help.line.usage".into(),
        "line [-A <ຄຳນຳໜ້າ>] [-B <ຕົວຄັ່ນ>] [file] ## ໃສ່ເລກແຕ່ລະແຖວ (ຄ່າເລີ່ມຕົ້ນ '1: ແຖວ')".into(),
    );

    m.insert(
        "help.line.example1".into(),
        "gs main | line -A 'line' -B ':' ## ເຕີມ line1: ... ໜ້າທຸກແຖວ".into(),
    );
}
