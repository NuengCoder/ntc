use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.scr.continue".into(),
        "scr -c <file.ntc> [args...] ## ແລ່ນສະຄຣິບ ສືບຕໍ່ຫຼັງມີຂໍ້ຜິດພາດ".into(),
    );

    m.insert(
        "help.scr.example1".into(),
        "scr deploy.ntc ## ແລ່ນ deploy.ntc".into(),
    );

    m.insert(
        "help.scr.example2".into(),
        "scr build.ntc dev prod ## ສົ່ງ args $1=dev, $2=prod".into(),
    );

    m.insert(
        "help.scr.example3".into(),
        "scr -c setup.ntc ## ແລ່ນແລະສືບຕໍ່ຫຼັງຂໍ້ຜິດພາດ".into(),
    );

    m.insert(
        "help.scr.help".into(),
        "scr help ## ສະແດງການໃຊ້ງານ scr".into(),
    );

    m.insert(
        "help.scr.init".into(),
        "scr init [name] ## ສ້າງ template ສະຄຣິບ".into(),
    );

    m.insert(
        "help.scr.ls".into(),
        "scr ls ## ລາຍຊື່ *.ntc ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.scr.run".into(),
        "scr <file.ntc> [args...] ## ແລ່ນສະຄຣິບອັດຕະໂນມັດ".into(),
    );

    m.insert("help.scr.title".into(), "ຄຳສັ່ງ SCR / ສະຄຣິບອັດຕະໂນມັດ:".into());
}
