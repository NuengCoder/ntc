use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.syspath.title".into(), "SYSPATH ຄຳສັ່ງ:".into());

    m.insert(
        "help.syspath.show".into(),
        "syspath show ## ສະແດງ PATH entries ລະດັບລະບົບ".into(),
    );

    m.insert(
        "help.syspath.add".into(),
        "syspath add <path> ## ເພີ່ມໄດເຣັກທໍຣີໃສ່ system PATH".into(),
    );

    m.insert(
        "help.syspath.add_cwd".into(),
        "syspath add --cwd | --here ## ເພີ່ມໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ system PATH".into(),
    );

    m.insert(
        "help.syspath.rm".into(),
        "syspath rm <path-or-number> ## ລຶບຈາກ system PATH (ຕາມ path ຫຼື index)".into(),
    );

    m.insert(
        "help.syspath.move".into(),
        "syspath move <n> <m> ## ຍ້າຍ entry #n ໄປຕຳແໜ່ງ #m".into(),
    );

    m.insert(
        "help.syspath.example1".into(),
        "syspath show ## ສະແດງ system PATH entries".into(),
    );

    m.insert(
        "help.syspath.example2".into(),
        "syspath add /usr/local/bin ## ເພີ່ມໃສ່ system PATH".into(),
    );

    m.insert(
        "help.syspath.example3".into(),
        "syspath move 3 1 ## ຍ້າຍ entry ໄປຕຳແໜ່ງ 1".into(),
    );
}
