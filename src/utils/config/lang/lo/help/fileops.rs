use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fileops.cp".into(),
        "cp <src|@tp> <dst|@tp> [-f] ## ສຳເນົາໄຟລ໌ຫຼືໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.fileops.mkd".into(),
        "mkd <dir>[, ...] ## ສ້າງໄດເຣັກທໍຣີໃໝ່".into(),
    );

    m.insert(
        "help.fileops.mkf".into(),
        "mkf <file>[, ...] ## ສ້າງໄຟລ໌ໃໝ່".into(),
    );

    m.insert(
        "help.fileops.rmd".into(),
        "rmd <dir>[, ...] ## ລຶບໄດເຣັກທໍຣີ; wildcard ລຶບທຸກລາຍການທີ່ກົງກັນຫຼັງຢືນຢັນ".into(),
    );

    m.insert(
        "help.fileops.rmf".into(),
        "rmf <file>[, ...] ## ລຶບໄຟລ໌ (ພ້ອມຢືນຢັນ)".into(),
    );

    m.insert(
        "help.fileops.rnmd".into(),
        "rnmd <old> <new> ## ປ່ຽນຊື່ໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.fileops.rnmf".into(),
        "rnmf <old> <new> ## ປ່ຽນຊື່ໄຟລ໌".into(),
    );

    m.insert(
        "help.fileops.mv".into(),
        "mv <src|@tp> <dst|@tp> [-f] ## ຍ້າຍ/ປ່ຽນຊື່ໄຟລ໌ຫຼືໄດເຣັກທໍຣີ".into(),
    );

    m.insert("help.fileops.title".into(), "ການດຳເນີນການ FILE:".into());

    m.insert(
        "help.fileops.example1".into(),
        "mkf index.html ## ສ້າງໄຟລ໌ໃໝ່".into(),
    );

    m.insert(
        "help.fileops.example2".into(),
        "mkd src/components ## ສ້າງໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.fileops.example3".into(),
        "cp file.txt backup.txt ## ສຳເນົາໄຟລ໌".into(),
    );
}
