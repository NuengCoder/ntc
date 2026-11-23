use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ntpx.title".into(), "NTPX ຄຳສັ່ງ:".into());

    m.insert(
        "help.ntpx.usage".into(),
        "ntpx <pkg> [bin] [args] ## ເຮັດວຽກ binary ຂອງ package ທີ່ຕິດຕັ້ງ".into(),
    );

    m.insert(
        "help.ntpx.example1".into(),
        "ntpx node npm init ## ເຮັດວຽກ npm ຜ່ານ ntpx".into(),
    );
}
