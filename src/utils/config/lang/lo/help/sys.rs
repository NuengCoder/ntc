use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.sys.title".into(), "ຄຳສັ່ງ SYSTEM:".into());

    m.insert(
        "help.sys.monitor".into(),
        "sys, top [-l|--log] [-d|--delay [ວິນາທີ:-1]] ## ຕິດຕາມລະບົບແບບສົດ (CPU, memory, disk, processes)".into(),
    );

    m.insert(
        "help.sys.passthrough".into(),
        "<ຄຳສັ່ງໃດໆ> ## NTC ສົ່ງຄຳສັ່ງທີ່ບໍ່ຮູ້ຈັກໄປຍັງລະບົບຂອງທ່ານ".into(),
    );

    m.insert(
        "help.sys.native".into(),
        "shell -- echo, git, dir/ls, ping, ອື່ນໆ. all ເຮັດວຽກ native ພ້ອມກັບຄຳສັ່ງ NTC.".into(),
    );

    m.insert(
        "help.sys.example1".into(),
        "sys ## ສະແດງ system monitor".into(),
    );

    m.insert(
        "help.sys.example2".into(),
        "sys --log --delay 5 ## ເພີ່ມການໃຊ້ CPU/ຫນ່ວຍຄວາມຈຳ/ດິສກ໌ ທຸກໆ 5 ວິນາທີ (ຄ່າເລີ່ມຕົ້ນ 15 ວິນາທີ) ຕໍ່ທ້າຍ log/ntc_sys.log".into(),
    );

    m.insert(
        "help.sys.example3".into(),
        "sys -ld 5 ## ຄືກັນກັບ 'sys -l -d 5': -l/--log ເປີດການບັນທຶກ, -d/--delay ກຳນົດວິນາທີ".into(),
    );
}
