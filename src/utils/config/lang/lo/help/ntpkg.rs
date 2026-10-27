use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ntpkg.available".into(),
        "ntpkg available [lang] ## ລາຍຊື່ພາສາ ຫຼື ເວີຊັນທີ່ດາວໂຫຼດໄດ້".into(),
    );

    m.insert(
        "help.ntpkg.help".into(),
        "ntpkg help ## ສະແດງການໃຊ້ງານ ntpkg".into(),
    );

    m.insert(
        "help.ntpkg.info".into(),
        "ntpkg info <lang> ## ສະແດງຂໍ້ມູນ package (version, location, binaries)".into(),
    );

    m.insert(
        "help.ntpkg.install".into(),
        "ntpkg install <lang> [ver] ## ດາວໂຫຼດ ແລະ ຕິດຕັ້ງ runtime".into(),
    );

    m.insert(
        "help.ntpkg.list".into(),
        "ntpkg list ## ສະແດງ runtimes ທີ່ຕິດຕັ້ງ".into(),
    );

    m.insert(
        "help.ntpkg.ntpx".into(),
        "ntpx <pkg> [bin] [args] ## ເຮັດວຽກ binary ຂອງ package ທີ່ຕິດຕັ້ງ".into(),
    );

    m.insert(
        "help.ntpkg.search".into(),
        "ntpkg search [keyword] ## ຄົ້ນຫາ packages ທີ່ມີ".into(),
    );

    m.insert(
        "help.ntpkg.title".into(),
        "PACKAGE MANAGER (ntpkg) + ntpx:".into(),
    );

    m.insert(
        "help.ntpkg.uninstall".into(),
        "ntpkg uninstall <lang> [ver] ## ຖອນ runtime ທີ່ຕິດຕັ້ງ".into(),
    );

    m.insert(
        "help.ntpkg.use".into(),
        "ntpkg use <lang> <ver> ## ເປີດໃຊ້ເວີຊັນສະເພາະ".into(),
    );

    m.insert(
        "help.ntpkg.which".into(),
        "ntpkg which <lang> ## ສະແດງ path ຂອງ runtime ທີ່ໃຊ້ຢູ່".into(),
    );

    m.insert(
        "help.ntpkg.example1".into(),
        "ntpkg install node 18 ## ຕິດຕັ້ງ Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example2".into(),
        "ntpkg list ## ສະແດງ runtimes ທີ່ຕິດຕັ້ງ".into(),
    );

    m.insert(
        "help.ntpkg.example3".into(),
        "ntpkg use node 18 ## ເປີດໃຊ້ Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example4".into(),
        "ntpkg which node ## ສະແດງ path ຂອງ Node.js".into(),
    );

    m.insert(
        "help.ntpkg.example5".into(),
        "ntpkg search python ## ຄົ້ນຫາ packages Python".into(),
    );

    m.insert(
        "help.ntpkg.example6".into(),
        "ntpkg info node ## ສະແດງຂໍ້ມູນ package Node.js".into(),
    );

    m.insert(
        "help.ntpkg.example7".into(),
        "ntpkg available ## ລາຍຊື່ພາສາທີ່ດາວໂຫຼດໄດ້".into(),
    );

    m.insert(
        "help.ntpkg.example8".into(),
        "ntpx node npm init ## ເຮັດວຽກ npm ຜ່ານ ntpx".into(),
    );

    m.insert(
        "help.ntpkg.example9".into(),
        "ntpkg help ## ສະແດງການໃຊ້ງານ ntpkg".into(),
    );
}
