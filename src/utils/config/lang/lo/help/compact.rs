use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "ພາບລວມຄຳສັ່ງ".into());

    m.insert(
        "help.compact.footer".into(),
        "ພິມ 'help <ຄຳສັ່ງ>' ເພື່ອເບິ່ງລາຍລະອຽດ, 'help --all' ເພື່ອເບິ່ງທັງໝົດ.".into(),
    );

    m.insert("help.compact.arc".into(), "ສ້າງໄຟລ໌ບີບอัด".into());

    m.insert(
        "help.compact.autoup".into(),
        "ລາຍຊື່ startup entries (CU/LM/SF sources) [Windows Only]".into(),
    );

    m.insert(
        "help.compact.back".into(),
        "ກັບໄປໄດເຣັກທໍຣີແມ່ (ຫຼື n ລະດັບ)".into(),
    );

    m.insert(
        "help.compact.backup".into(),
        "ສ້າງ backup ຂອງໂຄງການປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.compact.betw".into(),
        "ສະແດງແຖວ A ຫາ B (ເລີ່ມທີ່ 1 ລວມປາຍ)".into(),
    );

    m.insert("help.compact.care".into(), "ຢຸດການບໍ່ສົນໃຈໄດເຣັກທໍຣີ".into());

    m.insert("help.compact.cd".into(), "ນຳທາງໄປໄດເຣັກທໍຣີ".into());

    m.insert("help.compact.clear".into(), "ລ້າງໜ້າຈໍ".into());

    m.insert("help.compact.clip".into(), "ສະແດງ clipboard history".into());

    m.insert("help.compact.config".into(), "ສະແດງ/ຕັ້ງ path ຜົນລັບ".into());

    m.insert(
        "help.compact.count".into(),
        "ນັບຈຳນວນຄັ້ງທີ່ <needle> ປາກົດ".into(),
    );

    m.insert("help.compact.cp".into(), "ສຳເນົາໄຟລ໌ຫຼືໄດເຣັກທໍຣີ".into());

    m.insert("help.compact.cvcode".into(), "ແປງລະຫວ່າງຮູບແບບ".into());

    m.insert(
        "help.compact.deadlock".into(),
        "  deadlock                          ເປີດຕູ້ເກັບ (ຈະຖາມລະຫັດຜ່ານຫຼວຍ)".into(),
    );

    m.insert(
        "help.compact.decode".into(),
        "ຖອດລະຫັດ hex, base64, URL, binary".into(),
    );

    m.insert(
        "help.compact.deinit".into(),
        "ລຶບໄຟລ໌ໂຄງການ (ຖາມຢືນຢັນກ່ອນລຶບທຸກຄັ້ງ)".into(),
    );

    m.insert("help.compact.dl".into(), "ດາວໂຫຼດໄຟລ໌".into());

    m.insert("help.compact.dns".into(), "ຄົ້ນຫາ DNS".into());

    m.insert(
        "help.compact.doctor".into(),
        "ກວດ config, PATH, ranfile ແລະ ntpkg".into(),
    );

    m.insert("help.compact.ds".into(), "ຄົ້ນຫາໄດເຣັກທໍຣີຕາມຊື່".into());

    m.insert(
        "help.compact.encode".into(),
        "ເຂົ້າລະຫັດ hex, base64, URL, binary".into(),
    );

    m.insert("help.compact.exit".into(), "ອອກຈາກ ntc".into());

    m.insert(
        "help.compact.explorer".into(),
        "ເປີດ terminal file explorer ເຕັມຈໍ".into(),
    );

    m.insert(
        "help.compact.fake".into(),
        "  fake gen <ຊະນິດ> [-n N] [-f ຮູບແບບ] [-s ຊີດ] [-o ໄຟລ]   ສ້າງແຖວ".into(),
    );

    m.insert(
        "help.compact.fgo".into(),
        "ຊອກຫາໄຟລ໌, ນຳທາງໄປ parent dir ຂອງໄຟລ໌ທີ່ເລືອກ".into(),
    );

    m.insert("help.compact.fmt".into(), "ແປງ JSON ເປັນ YAML ຫຼື TOML".into());

    m.insert("help.compact.fs".into(), "ຄົ້ນຫາໄຟລ໌ຕາມຊື່".into());

    m.insert(
        "help.compact.fsc".into(),
        "ຄົ້ນຫາໄຟລ໌, ເລືອກເພື່ອສະແດງ ແລະ ສຳເນົາໄດ້".into(),
    );

    m.insert(
        "help.compact.git".into(),
        "ສະແດງສະຖານະ repo ແບບອ່ານງ່າຍ".into(),
    );

    m.insert(
        "help.compact.godrive".into(),
        "ສະຫຼັບ drive ຫຼື mount /mnt".into(),
    );

    m.insert(
        "help.compact.gos".into(),
        "ລາຍຊື່ subdirectories ແລະ ເລືອກ".into(),
    );

    m.insert("help.compact.gosc".into(), "ນຳທາງຕໍ່ເນື່ອງ".into());

    m.insert("help.compact.gs".into(), "ຄົ້ນຫາເນື້ອໃນໄຟລ໌ (grep ງ່າຍໆ)".into());

    m.insert(
        "help.compact.head".into(),
        "ສະແດງ N ແຖວທຳອິດ (ຄ່າເລີ່ມຕົ້ນ 10)".into(),
    );

    m.insert("help.compact.help".into(), "ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into());

    m.insert("help.compact.http".into(), "ສົ່ງຄຳຮ້ອງ HTTP".into());

    m.insert(
        "help.compact.igcare".into(),
        "ສົ່ງອອກການຕັ້ງຄ່າທັງໝົດໄປຍັງ <name>.ntc.igcare".into(),
    );

    m.insert(
        "help.compact.igcared".into(),
        "ສະແດງການຕັ້ງຄ່າ ignore/care ທັງໝົດ".into(),
    );

    m.insert("help.compact.ignore".into(), "ບໍ່ສົນໃຈໄດເຣັກທໍຣີຕາມຊື່".into());

    m.insert(
        "help.compact.image".into(),
        "ເລືອກຮູບຈາກໄດເຣັກທໍຣີປັດຈຸບັນແລ້ວສະແດງໃນ terminal".into(),
    );

    m.insert("help.compact.init".into(), "ສ້າງໄຟລ໌ໂຄງການ".into());

    m.insert(
        "help.compact.ipcfg".into(),
        "ສະແດງ adapters ແບບ ipconfig/ifconfig".into(),
    );

    m.insert(
        "help.compact.local".into(),
        "ສ້າງ ntconfig.toml template (commented)".into(),
    );

    m.insert("help.compact.locate".into(), "ຄົ້ນຫາທັງໄຟລ໌ ແລະ ໄດເຣັກທໍຣີ".into());

    m.insert("help.compact.math".into(), "ຄຳນວນພົດການຄິດໄລ່".into());

    m.insert("help.compact.mkd".into(), "ສ້າງໄດເຣັກທໍຣີໃໝ່".into());

    m.insert("help.compact.mkf".into(), "ສ້າງໄຟລ໌ໃໝ່".into());

    m.insert(
        "help.compact.mode".into(),
        "ອອກ ແລະ ເລີ່ມ ntc ໃໝ່ເປັນ Administrator".into(),
    );

    m.insert("help.compact.mv".into(), "ຍ້າຍ/ປ່ຽນຊື່ໄຟລ໌ຫຼືໄດເຣັກທໍຣີ".into());

    m.insert(
        "help.compact.ndb".into(),
        "ເປີດ ndb database shell (.db browser) ໃນໄດເຣັກທຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.compact.ne".into(),
        "ເປີດໄຟລ໌ທີ່ມີຢູ່ແລ້ວໃນ text editor ໃນຕົວ".into(),
    );

    m.insert("help.compact.nms".into(), "ຫຼິ້ນສຽງໃນ terminal".into());

    m.insert(
        "help.compact.nsheet".into(),
        "ເບິ່ງຕາຕະລາງແບບເຕັມຈໍ (CSV/TSV/JSON)".into(),
    );

    m.insert("help.compact.nssh".into(), "ບັນທຶກໂປຣໄຟລ໌ SSH".into());

    m.insert(
        "help.compact.ntpkg".into(),
        "ດາວໂຫຼດ ແລະ ຕິດຕັ້ງ runtime".into(),
    );

    m.insert(
        "help.compact.ntpx".into(),
        "ເຮັດວຽກ binary ຂອງ package ທີ່ຕິດຕັ້ງ".into(),
    );

    m.insert(
        "help.compact.opencg".into(),
        "ເປີດ config.toml ໃນ editor".into(),
    );

    m.insert(
        "help.compact.pathcp".into(),
        "ສຳເນົາ path ໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ clipboard".into(),
    );

    m.insert("help.compact.ping".into(), "Ping ໂຮສ".into());

    m.insert(
        "help.compact.pipe".into(),
        "ສົ່ງ output ຂອງ a ເປັນ input ຂອງ b [NON-COMMAND]".into(),
    );

    m.insert(
        "help.compact.py_activate".into(),
        "ເປີດໃຊ້ Python virtual environment".into(),
    );

    m.insert(
        "help.compact.py_deactivate".into(),
        "ປິດໃຊ້ງານ virtual environment ປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.compact.qr".into(),
        "ເຂົ້າລະຫັດຂໍ້ຄວາມ ແລະ ສະແດງ QR ໃນ terminal".into(),
    );

    m.insert(
        "help.compact.qsc".into(),
        "ເລີ່ມ QSC daemon (background process)".into(),
    );

    m.insert("help.compact.qur".into(), "ຄົ້ນຫາໄຟລ໌, ເລືອກເນື້ອໃນ, ແທນທີ່".into());

    m.insert("help.compact.ral".into(), "ສ້າງ alias".into());

    m.insert(
        "help.compact.ran".into(),
        "ເຮັດວຽກ target 'standalone' (default)".into(),
    );

    m.insert(
        "help.compact.report".into(),
        "ບັນທຶກລາຍງານໂຄງການໄວ້ທີ <file>".into(),
    );

    m.insert(
        "help.compact.resetcg".into(),
        "ຣີເຊັດ config ເປັນຄ່າເລີ່ມຕົ້ນ (ພ້ອມຕົວເລືອກ backup)".into(),
    );

    m.insert(
        "help.compact.restorecg".into(),
        "ກູ້ຄືນ config ຈາກ backup".into(),
    );

    m.insert("help.compact.rev".into(), "ຍ້ອນຕົວອັກສອນຂອງແຕ່ລະແຖວ".into());

    m.insert(
        "help.compact.rmd".into(),
        "ລຶບໄດເຣັກທໍຣີ; wildcard ລຶບທຸກລາຍການທີ່ກົງກັນຫຼັງຢືນຢັນ".into(),
    );

    m.insert("help.compact.rmf".into(), "ລຶບໄຟລ໌ (ພ້ອມຢືນຢັນ)".into());

    m.insert("help.compact.rnmd".into(), "ປ່ຽນຊື່ໄດເຣັກທໍຣີ".into());

    m.insert("help.compact.rnmf".into(), "ປ່ຽນຊື່ໄຟລ໌".into());

    m.insert("help.compact.run".into(), "ເຮັດວຽກ alias ດ້ວຍ 'run'".into());

    m.insert("help.compact.say".into(), "ພິມຂໍ້ຄວາມ (ບໍ່ redirect ໄຟລ໌)".into());

    m.insert(
        "help.compact.nprintf".into(),
        "ຜົນອອກຈັດຮູບແບບສະຕາຍ Bash".into(),
    );

    m.insert(
        "help.compact.scan".into(),
        "ສະແດງໄຟລ໌ຕາມນາມສະກຸນ (ບໍ່ນັບແຖວ)".into(),
    );

    m.insert("help.compact.scr".into(), "ແລ່ນສະຄຣິບອັດຕະໂນມັດ".into());

    m.insert(
        "help.compact.serve".into(),
        "ເລືອກບໍລິການ (`serve` ດຽວຈະສະແດງລາຍການ)".into(),
    );

    m.insert(
        "help.compact.showcg".into(),
        "ສະແດງພາບລວມການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.compact.showenv".into(),
        "ສະແດງ environment variables (PATH entries ທຳອິດ 5)".into(),
    );

    m.insert("help.compact.size".into(), "ສະແດງຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into());

    m.insert(
        "help.compact.skill".into(),
        "ດາວໂຫຼດ skill ຈາກ GitHub".into(),
    );

    m.insert("help.compact.smtp".into(), "ສົ່ງອີເມວ (SMTP client)".into());

    m.insert("help.compact.sort".into(), "ຈັດລຳດັບແຖວ".into());

    m.insert("help.compact.spec".into(), "ສະແດງສະເປັກຄອມພິວເຕີ/ລະບົບ".into());

    m.insert("help.compact.sys".into(), "ຕິດຕາມລະບົບແບບສົດ".into());

    m.insert(
        "help.compact.syspath".into(),
        "ສະແດງ PATH entries ລະດັບລະບົບ".into(),
    );

    m.insert(
        "help.compact.tail".into(),
        "ສະແດງ N ແຖວສຸດທ້າຍ (ຄ່າເລີ່ມຕົ້ນ 10)".into(),
    );

    m.insert(
        "help.compact.theme".into(),
        "ຕົວຈັດການ theme interactive (ເມນູ)".into(),
    );

    m.insert(
        "help.compact.time".into(),
        "  time run <ຄຳສັ່ງ>                     ຮັບໃຊ້ຄຳສັ່ງ NTC, alias ຫຼື ຄຳສັ່ງພາຍນອກ ແລະ ລາຍງານເວລາ"
            .into(),
    );

    m.insert("help.compact.tl".into(), "ນັບແຖວໃນໄຟລ໌ເດີ່ມ".into());

    m.insert(
        "help.compact.tmprt".into(),
        "ຕິດຕາມອຸນຫະພູມ hardware ແບບສົດ".into(),
    );

    m.insert("help.compact.todo".into(), "ສະແດງວຽກທັງໝົດ".into());

    m.insert("help.compact.tp".into(), "ເມນູ interactive".into());

    m.insert(
        "help.compact.tpb".into(),
        "Teleport ກັບໄປຕຳແໜ່ງກ່ອນໜ້າ (undo tp)".into(),
    );

    m.insert("help.compact.trace".into(), "ຕິດຕາມເສັ້ນທາງແຕ່ລະ hop".into());

    m.insert("help.compact.trim".into(), "ລຶບທຸກຄັ້ງທີ່ພົບ <pattern>".into());

    m.insert(
        "help.compact.tutorial".into(),
        "ບົດຮຽນປະຕິບັດໃນ sandbox".into(),
    );

    m.insert(
        "help.compact.txtc".into(),
        "ສຳເນົາເນື້ອໃນໃສ່ clipboard (ເປີດ/ປິດເລກແຖວ)".into(),
    );

    m.insert(
        "help.compact.txtf".into(),
        "ສະແດງເນື້ອໃນໄຟລ໌ ຫຼື ເລືອກໄຟລ໌ຈາກ directory (ເລກແຖວເດີມ)".into(),
    );

    m.insert("help.compact.ui".into(), "ສະແດງໂໝດ UI ປັດຈຸບັນ".into());

    m.insert("help.compact.up".into(), "ອັບໂຫຼດໄຟລ໌".into());

    m.insert(
        "help.compact.usrpath".into(),
        "ສະແດງ user-level PATH entries".into(),
    );

    m.insert("help.compact.version".into(), "ສະແດງເວີຊັນ".into());

    m.insert("help.compact.view".into(), "ສະແດງ tree ຫຼື ເນື້ອໃນໄຟລ໌".into());

    m.insert(
        "help.compact.viewd".into(),
        "ສະແດງ tree ສະເພາະ directory".into(),
    );

    m.insert(
        "help.compact.viewf".into(),
        "ສະແດງ tree ສະເພາະໄຟລ໌ທີ່ depth 1".into(),
    );

    m.insert(
        "help.compact.views".into(),
        "ສະແດງໜຶ່ງລະດັບຂອງ directory ແບບແບນຄື ls".into(),
    );

    m.insert(
        "help.compact.where".into(),
        "ສະແດງ path ຂອງ ntc ຫຼື ຄົ້ນຫາໂປຣແກຣມໃນ PATH".into(),
    );

    m.insert(
        "help.compact.wi".into(),
        "ຟีดການປ່ຽນແປງໄຟລ໌ແບບສົດ (q=ອອກ)".into(),
    );

    m.insert(
        "help.compact.line".into(),
        "ໃສ່ເລກແຕ່ລະແຖວ".into(),
    );
}
