use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.deadlock.title".into(),
        "DEADLOCK - ຕູ້ເກັບລະຫັດຜ່ານທີ່ເຂົ້າລະຫັດ:".into(),
    );

    m.insert(
        "help.deadlock.launch".into(),
        "  deadlock                          ເປີດຕູ້ເກັບ (ຈະຖາມລະຫັດຜ່ານຫຼວຍ)".into(),
    );

    m.insert(
        "help.deadlock.ls".into(),
        "  ls                                ລາຍຊື່ pass-env ທຸກອັນພ້ອມລະຫັດຜ່ານຈິງ".into(),
    );

    m.insert(
        "help.deadlock.add".into(),
        "  add <pass-env> <real-password>    ບັນທຶກ ຫຼື ອັບເດດຄວາມລັບ (ລະຫັດຜ່ານສາມາດໃສ່ເຄື່ອງໝາຍຄຳ)".into(),
    );

    m.insert(
        "help.deadlock.rmpw".into(),
        "  rmpw <pass-env>                   ລຶບຄວາມລັບທີ່ໄດ້ບັນທຶກໄວ້".into(),
    );

    m.insert(
        "help.deadlock.master".into(),
        "  master-pass                       ປ່ຽນລະຫັດຜ່ານຫຼວຍ".into(),
    );

    m.insert(
        "help.deadlock.forget".into(),
        "  forget                            ລຶບລະຫັດຜ່ານຫຼວຍອອກຈາກທີ່ເກັບຂໍ້ມູນຮັບຮູ້ຂອງລະບົບປະຕິບັດການ".into(),
    );

    m.insert(
        "help.deadlock.destroy".into(),
        "  destroy                           ລຶບຕູ້ເກັບ ແລະ ຄວາມລັບທັງໝົດ".into(),
    );

    m.insert(
        "help.deadlock.exit".into(),
        "  exit                              ປິດຕູ້ເກັບແລ້ວກັບໄປ ntc shell".into(),
    );

    m.insert(
        "help.deadlock.note".into(),
        "ຄວາມລັບຖືກເຂົ້າລະຫັດດ້ວຍ AES-256-GCM ໂດຍໃຊ້ກະແຈທີ່ມາຈາກລະຫັດຜ່ານຫຼວຍຜ່ານ Argon2id. ລະຫັດຜ່ານຫຼວຍເລີ່ມຕົ້ນແມ່ນ 12345678 ແລະ deadlock ຈະໃຫ້ຕັ້ງລະຫັດຜ່ານໃໝ່ເມື່ອເຂົ້າສູ່ລະບົບຄັ້ງທຳອິດ. ໄຟລ໌ຕູ້ເກັບຖືກຈຳກັດໃຫ້ແຕ່ບັນຊີຂອງເຈົ້າ ແລະ ຖືກປົກປ້ອງຈາກການລຶບໂດຍລະບົບປະຕິບັດການ (Windows deny-DELETE ACL / macOS uchg; Linux ຕ້ອງໃຊ້ root). ລະຫັດຜ່ານຫຼວຍຖືກລົງທະບຽນໄວ້ໃນທີ່ເກັບຂໍ້ມູນຮັບຮູ້ຂອງລະບົບປະຕິບັດການເພື່ອໃຫ້ nssh ສາມາດແກ້ຊື່ pass-env ໄດ້ໂດຍບໍ່ຕ້ອງເປີດ shell ນີ້.".into(),
    );

    m.insert("help.deadlock.example1".into(), "  deadlock".into());

    m.insert(
        "help.deadlock.example2".into(),
        "  deadlock > add prod_pass CatLoveFish360@house".into(),
    );
}
