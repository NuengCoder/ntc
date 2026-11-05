use std::collections::HashMap;

/// ຄຳອະທິບາຍແຖວດຽວສຳລັບ help ແບບຫຍໍ້ (`help --all` ຍັງສະແດງແບບເຕັມ)
pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "ພາບລວມຄຳສັ່ງ".into());
    m.insert(
        "help.compact.footer".into(),
        "ພິມ 'help <ຄຳສັ່ງ>' ເພື່ອເບິ່ງລາຍລະອຽດ, 'help --all' ເພື່ອເບິ່ງທັງໝົດ.".into(),
    );

    m.insert("help.compact.nav".into(), "ປ່ຽນໄດເຣັກທໍຣີ, ກັບຄືນ, ກະໂດດ".into());
    m.insert("help.compact.config".into(), "ເບິ່ງ ຫຼື ປ່ຽນການຕັ້ງຄ່າ ntc".into());
    m.insert("help.compact.editor".into(), "ເປີດ editor ໃນຕົວ".into());
    m.insert("help.compact.theme".into(), "ສະຫຼັບ, ເພີ່ມ, ສົ່ງອອກ theme".into());
    m.insert(
        "help.compact.teleport".into(),
        "path ທີ່ບັນທຶກໄວ້ (tp jump <name>)".into(),
    );
    m.insert(
        "help.compact.ran".into(),
        "ເອີ້ນ task ຈາກ NTCRANFILE.toml".into(),
    );
    m.insert("help.compact.scr".into(), "ເອີ້ນໄຟລ໌ script ທີ່ບັນທຶກໄວ້".into());
    m.insert(
        "help.compact.alias".into(),
        "ເອີ້ນ alias (ral add <name> ...)".into(),
    );
    m.insert("help.compact.view".into(), "ຕົ້ນໄມ້ໄດເຣັກທໍຣີ ແລະ ໄຟລ໌".into());
    m.insert("help.compact.size".into(), "ຂະໜາດໄດເຣັກທໍຣີ".into());
    m.insert(
        "help.compact.report".into(),
        "ລາຍງານໂປຣເຈັກ (report project)".into(),
    );
    m.insert("help.compact.backup".into(), "ສຳຮອງ ແລະ ກູ້ຄືນໂປຣເຈັກ".into());
    m.insert(
        "help.compact.search".into(),
        "ຄົ້ນຫາຂໍ້ຄວາມ, ໄຟລ໌ ແລະ ໂຟນເດີ".into(),
    );
    m.insert(
        "help.compact.igcare".into(),
        "ກົດ ignore/care ສຳລັບການສະແກນ".into(),
    );
    m.insert(
        "help.compact.network".into(),
        "dl, up, http, serve, ping, dns, smtp".into(),
    );
    m.insert("help.compact.tmprt".into(), "ອຸນຫະພູມ ແລະ ອາກາດ".into());
    m.insert("help.compact.time".into(), "ຈັບເວລາ ແລະ benchmark".into());
    m.insert(
        "help.compact.git".into(),
        "ຄຳສັ່ງ Git ລັດ (status, branch, log)".into(),
    );
    m.insert(
        "help.compact.clipboard".into(),
        "ປະຫວັດ clipboard (clip list)".into(),
    );
    m.insert("help.compact.qsc".into(), "ຕົວຟັງຄຳສັ່ງດ່ວນ".into());
    m.insert("help.compact.todo".into(), "ລາຍການວຽກຂອງໂປຣເຈັກ".into());
    m.insert("help.compact.arc".into(), "ສ້າງ ແລະ ແຕກໄຟລ໌ບີບອັດ".into());
    m.insert("help.compact.scan".into(), "ສະແກນນາມສະກຸນ, port, IP".into());
    m.insert("help.compact.tl".into(), "ນັບແຖວໃນໄຟລ໌".into());
    m.insert(
        "help.compact.ntpkg".into(),
        "ຕິດຕັ້ງ ແລະ ຈັດການເຄື່ອງມື (ntpx)".into(),
    );
    m.insert("help.compact.qr".into(), "ສ້າງ ແລະ ອ່ານ QR code".into());
    m.insert(
        "help.compact.pathman".into(),
        "ຈັດການລາຍການ PATH (usrpath, syspath)".into(),
    );
    m.insert("help.compact.autoup".into(), "ຈັດການໂປຣແກຣມເລີ່ມຕົ້ນ".into());
    m.insert("help.compact.mode".into(), "ໂໝດ admin / ປົກກະຕິ".into());
    m.insert("help.compact.explorer".into(), "ຕົວເບິ່ງໄຟລ໌ (nfe)".into());
    m.insert("help.compact.math".into(), "ຄິດໄລ່ນິພົດ".into());
    m.insert("help.compact.python".into(), "ເປີດ Python venv".into());
    m.insert(
        "help.compact.encdec".into(),
        "encode, decode, cvcode".into(),
    );
    m.insert("help.compact.fmt".into(), "ຈັດຮູບແບບໄຟລ໌".into());
    m.insert("help.compact.image".into(), "ສະແດງຮູບໃນ terminal".into());
    m.insert("help.compact.ndb".into(), "ເບິ່ງຖານຂໍ້ມູນ SQLite".into());
    m.insert("help.compact.nsheet".into(), "ເບິ່ງ spreadsheet".into());
    m.insert(
        "help.compact.nssh".into(),
        "ໂປຣໄຟລ໌ SSH ແລະ ການສຳເນົາໄຟລ໌".into(),
    );
    m.insert("help.compact.nms".into(), "ເຄື່ອງຫຼິ້ນເພງໃນ terminal".into());
    m.insert("help.compact.fake".into(), "ສ້າງຂໍ້ມູນປອມ".into());
    m.insert("help.compact.deadlock".into(), "ຕູ້ລະຫັດຜ່ານແບບເຂົ້າລະຫັດ".into());
    m.insert("help.compact.syscmd".into(), "ເອີ້ນຄຳສັ່ງລະບົບໃດໆ".into());
    m.insert(
        "help.compact.pipe".into(),
        "pipe, ການປ່ຽນທາງ ແລະ filter".into(),
    );
    m.insert(
        "help.compact.fileops".into(),
        "mkf, mkd, rmf, rmd, mv, cp".into(),
    );
    m.insert(
        "help.compact.other".into(),
        "clear, version, say, ui, exit, help".into(),
    );
}
