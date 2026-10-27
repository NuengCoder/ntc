use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.spec.other".into(), "ອື່ນໆ".into());

    m.insert("cmd.spec.title".into(), "ສະເປັກລະບົບ".into());

    m.insert("cmd.spec.cpu".into(), "ຊີພີຢູ".into());

    m.insert("cmd.spec.vendor".into(), "ຜູ້ຜະລິດ".into());

    m.insert("cmd.spec.generation".into(), "ເຈັນ".into());

    m.insert("cmd.spec.model".into(), "ຮຸ່ນ".into());

    m.insert("cmd.spec.cores".into(), "ຄໍ / ທຣີດ".into());

    m.insert("cmd.spec.gpu".into(), "ກາດຈໍ".into());

    m.insert("cmd.spec.ram".into(), "ແຣມ".into());

    m.insert("cmd.spec.storage".into(), "ບ່ອນເກັບຂໍ້ມູນ".into());

    m.insert("cmd.spec.total".into(), "ລວມ".into());

    m.insert("cmd.spec.os".into(), "ລະບົບປະຕິບັດການ".into());

    m.insert("cmd.spec.manufacturer".into(), "ຜູ້ຜະລິດ".into());

    m.insert("cmd.spec.computer_model".into(), "ຮຸ່ນຄອມພິວເຕີ".into());

    m.insert("cmd.spec.serial".into(), "ເລກຊີຣຽວ".into());

    m.insert("cmd.spec.hostname".into(), "ຊື່ໂຮສ".into());

    m.insert("cmd.spec.kernel".into(), "ເຄີເນລ".into());

    m.insert("cmd.spec.uptime".into(), "ເວລາເຮັດວຽກ".into());

    m.insert("cmd.spec.unknown".into(), "ບໍ່ຮູ້".into());

    m.insert("cmd.spec.custom".into(), "ປະກອບເອງ".into());
}
