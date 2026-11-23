use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.usrpath.title".into(),
        "ຕົວຈັດການ PATH:".into(),
    );

    m.insert(
        "help.usrpath.show".into(),
        "usrpath show ## ສະແດງ user-level PATH entries".into(),
    );

    m.insert(
        "help.usrpath.add".into(),
        "usrpath add <path> ## ເພີ່ມໄດເຣັກທໍຣີໃສ່ user PATH".into(),
    );

    m.insert(
        "help.usrpath.add_cwd".into(),
        "usrpath add --cwd | --here ## ເພີ່ມໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ user PATH".into(),
    );

    m.insert(
        "help.usrpath.rm".into(),
        "usrpath rm <path-or-number> ## ລຶບຈາກ user PATH (ຕາມ path ຫຼື index)".into(),
    );

    m.insert(
        "help.usrpath.move".into(),
        "usrpath move <n> <m> ## ຍ້າຍ entry #n ໄປຕຳແໜ່ງ #m".into(),
    );

    m.insert(
        "help.usrpath.example1".into(),
        "usrpath show ## ສະແດງ user PATH entries".into(),
    );

    m.insert(
        "help.usrpath.example2".into(),
        "usrpath add ~/bin ## ເພີ່ມໃສ່ user PATH".into(),
    );

    m.insert(
        "help.usrpath.example3".into(),
        "usrpath rm 2 ## ລຶບ entry #2 ຈາກ user PATH".into(),
    );
}
