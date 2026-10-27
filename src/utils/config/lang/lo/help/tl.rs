use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.tl.dir".into(),
        "tl dir <path> [-s <-n|-l|-r>] ## ສະແດງນາມສະກຸນທີ່ຮອງຮັບ ແລະ ຈຳນວນແຖວໃນໄດເຣັກທໍຣີ (sort ໄດ້)"
            .into(),
    );

    m.insert("help.tl.ext".into(), "tl ext <ext> [-s <-n|-l|-r>] ## ສະແດງໄຟລ໌ <ext> ທັງໝົດ ແລະ ຈຳນວນແຖວລວມ (ເຄົາລົບ igcare, sort ໄດ້)".into());

    m.insert(
        "help.tl.file".into(),
        "tl file <path> ## ນັບແຖວໃນໄຟລ໌ເດີ່ມ".into(),
    );

    m.insert("help.tl.sort".into(), "tl ext|dir <target> -s <-n|-l|-r> ## Sort: -n ປົກກະຕິ, -l ຕາມແຖວ (ນ້ອຍຫາໃຫຍ່), -r ກົງກັນຂ້າມ (ໃຫຍ່ຫານ້ອຍ)".into());

    m.insert("help.tl.title".into(), "ຄຳສັ່ງ TOTAL LINE:".into());

    m.insert(
        "help.tl.example1".into(),
        "tl file main.rs ## ນັບແຖວໃນ main.rs".into(),
    );

    m.insert(
        "help.tl.example2".into(),
        "tl ext .rs ## ນັບແຖວໃນໄຟລ໌ .rs ທັງໝົດ".into(),
    );

    m.insert(
        "help.tl.example3".into(),
        "tl dir . ## ນັບແຖວໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.tl.example4".into(),
        "tl ext .rs -s -r ## ລາຍຊື່ໄຟລ໌ .rs ຈັດຕາມແຖວ, ໃຫຍ່ຫານ້ອຍ".into(),
    );
}
