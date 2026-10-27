use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.syscmd.any".into(),
        "<ຄຳສັ່ງໃດໆ> ## NTC ສົ່ງຄຳສັ່ງທີ່ບໍ່ຮູ້ຈັກໄປຍັງລະບົບຂອງທ່ານ".into(),
    );

    m.insert(
        "help.syscmd.native".into(),
        "shell -- echo, git, dir/ls, ping, ອື່ນໆ.".into(),
    );

    m.insert(
        "help.syscmd.native2".into(),
        "all ເຮັດວຽກ native ພ້ອມກັບຄຳສັ່ງ NTC.".into(),
    );

    m.insert("help.syscmd.title".into(), "ຄຳສັ່ງ SYSTEM:".into());
}
