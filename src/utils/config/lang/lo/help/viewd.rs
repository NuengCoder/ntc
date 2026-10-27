use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.viewd.title".into(),
        "ຄຳສັ່ງ VIEWD (ສະເພາະ directory):".into(),
    );

    m.insert(
        "help.viewd.usage".into(),
        "viewd [<dir|@tp|@tp/dir/>] [-d N] ## ສະແດງ tree ສະເພາະ directory".into(),
    );

    m.insert(
        "help.viewd.depth".into(),
        "viewd -d, --depth <n> ## ສະແດງ tree ດ້ວຍ depth ກຳນົດເອງ".into(),
    );

    m.insert(
        "help.viewd.size".into(),
        "viewd -s, --size [min-size] ## ສະແດງຂະໜາດ directory; ໃສ່ຄ່າຈະເຊື່ອງທີ່ນ້ອຍກວ່າ".into(),
    );

    m.insert(
        "help.viewd.care".into(),
        "viewd -c, --care ## ນັບທຸກຢ່າງເມື່ອຄິດຂະໜາດ".into(),
    );

    m.insert(
        "help.viewd.sort".into(),
        "viewd --sort size|char|ext ## ຈັດລຳດັບລາຍການ directory".into(),
    );

    m.insert(
        "help.viewd.time".into(),
        "viewd -t, --time ## ສະແດງເວລາດຳເນີນການ".into(),
    );

    m.insert(
        "help.viewd.copy".into(),
        "viewd -cp, --copy ## ສຳເນົາ directory tree ໃສ່ clipboard".into(),
    );

    m.insert(
        "help.viewd.example1".into(),
        "viewd ## ສະແດງສະເພາະ directory ໃນ tree ປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.viewd.example2".into(),
        "viewd -d 2 --size ## ສອງຊັ້ນພ້ອມຂະໜາດ directory".into(),
    );

    m.insert(
        "help.viewd.example3".into(),
        "viewd --sort size ## directory ໃຫຍ່ສຸດກ່ອນ".into(),
    );

    m.insert(
        "help.viewd.example4".into(),
        "viewd @docs/ ## ສະແດງ directory ຂອງ teleport".into(),
    );
}
