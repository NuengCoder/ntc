use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.pathman.syspath_add".into(),
        "syspath add <path> ## ເພີ່ມໄດເຣັກທໍຣີໃສ່ system PATH".into(),
    );

    m.insert(
        "help.pathman.syspath_add_cwd".into(),
        "syspath add --cwd | --here ## ເພີ່ມໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ system PATH".into(),
    );

    m.insert(
        "help.pathman.syspath_move".into(),
        "syspath move <n> <m> ## ຍ້າຍ entry #n ໄປຕຳແໜ່ງ #m".into(),
    );

    m.insert(
        "help.pathman.syspath_rm".into(),
        "syspath rm <path-or-number> ## ລຶບຈາກ system PATH (ຕາມ path ຫຼື index)".into(),
    );

    m.insert(
        "help.pathman.syspath_show".into(),
        "syspath show ## ສະແດງ PATH entries ລະດັບລະບົບ".into(),
    );

    m.insert(
        "help.pathman.title".into(),
        "PATH MANAGER (Windows):".into(),
    );

    m.insert(
        "help.pathman.usrpath_add".into(),
        "usrpath add <path> ## ເພີ່ມໄດເຣັກທໍຣີໃສ່ user PATH".into(),
    );

    m.insert(
        "help.pathman.usrpath_add_cwd".into(),
        "usrpath add --cwd | --here ## ເພີ່ມໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ user PATH".into(),
    );

    m.insert(
        "help.pathman.usrpath_move".into(),
        "usrpath move <n> <m> ## ຍ້າຍ entry #n ໄປຕຳແໜ່ງ #m".into(),
    );

    m.insert(
        "help.pathman.usrpath_rm".into(),
        "usrpath rm <path-or-number> ## ລຶບຈາກ user PATH (ຕາມ path ຫຼື index)".into(),
    );

    m.insert(
        "help.pathman.usrpath_show".into(),
        "usrpath show ## ສະແດງ user-level PATH entries".into(),
    );

    m.insert(
        "help.pathman.example1".into(),
        "usrpath show ## ສະແດງ user PATH entries".into(),
    );

    m.insert(
        "help.pathman.example2".into(),
        "usrpath add C:\\tools ## ເພີ່ມໃສ່ user PATH".into(),
    );

    m.insert(
        "help.pathman.example3".into(),
        "usrpath rm 2 ## ລຶບ entry #2 ຈາກ user PATH".into(),
    );

    m.insert(
        "help.pathman.example4".into(),
        "syspath show ## ສະແດງ system PATH entries".into(),
    );

    m.insert(
        "help.pathman.example5".into(),
        "syspath add C:\\bin ## ເພີ່ມໃສ່ system PATH".into(),
    );

    m.insert(
        "help.pathman.example6".into(),
        "syspath move 3 1 ## ຍ້າຍ entry ໄປຕຳແໜ່ງ 1".into(),
    );
}
