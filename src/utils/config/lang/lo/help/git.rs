use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.git.title".into(), "ຄຳສັ່ງ GIT INTEGRATION:".into());

    m.insert(
        "help.git.status".into(),
        "git status | st ## ສະແດງສະຖານະ repo ແບບອ່ານງ່າຍ".into(),
    );

    m.insert(
        "help.git.branch".into(),
        "git branch | b ## ລາຍຊື່ branch (ໄຮໄລ້ທີ່ກຳລັງໃຊ້)".into(),
    );

    m.insert(
        "help.git.log".into(),
        "git log | l ## ບັນທຶກ commit ຫຼ້າສຸດແບບມີສີ".into(),
    );

    m.insert(
        "help.git.diff".into(),
        "git diff ## ສະແດງ diff ໃນ working tree".into(),
    );

    m.insert(
        "help.git.passthrough".into(),
        "git <ໃດກໍໄດ້> ## ຄຳສັ່ງຍ່ອຍອື່ນສົ່ງຕໍ່ໄປ git ແທ້".into(),
    );

    m.insert(
        "help.git.example1".into(),
        "git status ## ສະແດງ branch + staged/unstaged/untracked".into(),
    );

    m.insert(
        "help.git.example2".into(),
        "git branch -a ## ລາຍຊື່ branch ທັງໝົດ (passthrough)".into(),
    );

    m.insert(
        "help.git.example3".into(),
        "git commit -am \"msg\" ## ໃຊ້ git ປົກກະຕິ (passthrough)".into(),
    );
}
