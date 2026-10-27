use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "git.no_git_binary".into(),
        "ຍັງບໍ່ໄດ້ຕິດຕັ້ງ Git. ແລ່ນ `ntpkg install git` ເພື່ອຕິດຕັ້ງ".into(),
    );

    m.insert(
        "git.not_repo".into(),
        "ບໍ່ແມ່ນ git repository (ຫຼືໂຟນເດີແມ່)".into(),
    );

    m.insert("git.executing".into(), "ກຳລັງດຳເນີນການ: {}".into());

    m.insert("git.completed".into(), "ຄຳສັ່ງ git ສຳເລັດ".into());

    m.insert("git.exit_code".into(), "ຄຳສັ່ງອອກດ້ວຍລະຫັດ: {}".into());

    m.insert("git.terminated".into(), "ຄຳສັ່ງຖືກຍົກເລີກ".into());

    m.insert("git.failed".into(), "ບໍ່ສາມາດດຳເນີນການຄຳສັ່ງໄດ້: {}".into());

    m.insert("git.checking_status".into(), "ກຳລັງກວດສະຖານະ".into());

    m.insert("git.status_failed".into(), "ບໍ່ສາມາດອ່ານສະຖານະ git ໄດ້".into());

    m.insert("git.branch_header".into(), "ຢູ່ໃນ branch".into());

    m.insert("git.upstream".into(), "ຕິດຕາມ {}".into());

    m.insert("git.ahead".into(), "ເໜືອກວ່າ {}".into());

    m.insert("git.behind".into(), "ຕ່ຳກວ່າ {}".into());

    m.insert("git.detached_header".into(), "Detached HEAD".into());

    m.insert("git.section_conflicts".into(), "ເສັ້ນທາງທີ່ຍັງບໍ່ໄດ້ merge:".into());

    m.insert("git.section_staged".into(), "ການປ່ຽນແປງທີ່ພ້ອມ commit:".into());

    m.insert(
        "git.section_unstaged".into(),
        "ການປ່ຽນແປງທີ່ຍັງບໍ່ staged:".into(),
    );

    m.insert("git.section_untracked".into(), "ໄຟລ໌ທີ່ບໍ່ໄດ້ຕິດຕາມ:".into());

    m.insert("git.clean".into(), "working tree ສະອາດ".into());

    m.insert("git.branch_failed".into(), "ບໍ່ສາມາດລາຍຊື່ branch ໄດ້".into());

    m.insert("git.log_failed".into(), "ບໍ່ສາມາດອ່ານ git log ໄດ້".into());

    m.insert("git.log_empty".into(), "ບໍ່ພົບ commit".into());

    m.insert("git.help_title".into(), "GIT INTEGRATION:".into());

    m.insert(
        "git.help_status".into(),
        "git status ## ສະແດງ branch + staged/unstaged/untracked".into(),
    );

    m.insert("git.help_branch".into(), "git branch ## ລາຍຊື່ branch".into());

    m.insert("git.help_log".into(), "git log ## commit ບໍ່ສຳເລັດ".into());

    m.insert(
        "git.help_diff".into(),
        "git diff ## diff ຂອງ working tree".into(),
    );

    m.insert(
        "git.help_passthrough".into(),
        "git <ໃດກໍໄດ້> ## ທີ່ເຫຼືອສົ່ງຕໍ່ git ແທ້".into(),
    );

    m.insert("git.examples_header".into(), "ຕົວຢ່າງ:".into());

    m.insert("git.help_example1".into(), "git status".into());

    m.insert("git.help_example2".into(), "git checkout main".into());
}
