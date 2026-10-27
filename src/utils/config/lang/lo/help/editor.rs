use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.editor.ne".into(),
        "ne, ntceditor <file> ## ເປີດໄຟລ໌ທີ່ມີຢູ່ແລ້ວໃນ text editor ໃນຕົວ".into(),
    );

    m.insert(
        "help.editor.ne_hyper".into(),
        "ne --hyper <name> --lang <l> [-I] ## ສ້າງ C/C++ hyper project (-I ເປີດໄດເຣັກທໍຣີໂຄງການ)".into(),
    );

    m.insert(
        "help.editor.ne_init".into(),
        "ne --init <file> [<file> ...] [-I <file>] ## ສ້າງໄຟລ໌ template; -I ເປີດໜຶ່ງໄຟລ໌".into(),
    );

    m.insert(
        "help.editor.ne_line".into(),
        "ne, ntceditor <file>:<n> ## ເປີດໄຟລ໌ທີ່ແຖວ <n>".into(),
    );

    m.insert(
        "help.editor.ne_super".into(),
        "ne --super <file> [<file> ...] [-I <file>] ## ສ້າງໄຟລ໌ template super; -I ເປີດໜຶ່ງໄຟລ໌".into(),
    );

    m.insert(
        "help.editor.ne_welcome".into(),
        "ne, ntceditor ## ເປີດໜ້າຈໍ welcome ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert("help.editor.title".into(), "ຄຳສັ່ງ EDITOR:".into());

    m.insert(
        "help.editor.example1".into(),
        "ne main.rs ## ເປີດ main.rs ໃນ editor".into(),
    );

    m.insert(
        "help.editor.example2".into(),
        "ne src/main.rs:42 ## ເປີດ main.rs ທີ່ແຖວ 42".into(),
    );

    m.insert(
        "help.editor.example3".into(),
        "ne --init a.py b.html -I a.py ## ສ້າງ template ແລະເປີດ a.py".into(),
    );

    m.insert(
        "help.editor.example4".into(),
        "ne --hyper myproject --lang c ## ສ້າງ C hyper project".into(),
    );
}
