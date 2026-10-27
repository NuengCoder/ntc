use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.backup.clear".into(),
        "backup clear ## ລຶບ backup ທັງໝົດ (ຖາມຢືນຢັນ)".into(),
    );

    m.insert(
        "help.backup.clear_force".into(),
        "backup clear --force ## ລຶບ backup ທັງໝົດ (ບໍ່ຢືນຢັນ)".into(),
    );

    m.insert(
        "help.backup.create".into(),
        "backup create ## ສ້າງ backup ຂອງໂຄງການປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.backup.diff".into(),
        "backup diff [num] ## ປຽບທຽບໂຄງການປັດຈຸບັນກັບ backup".into(),
    );

    m.insert(
        "help.backup.list".into(),
        "backup list ## ລາຍຊື່ backups ທັງໝົດ".into(),
    );

    m.insert(
        "help.backup.recreate".into(),
        "backup recreate ## ລຶບທັງໝົດ + undo + ສ້າງ backup ສົດ".into(),
    );

    m.insert(
        "help.backup.restore".into(),
        "backup restore [num] ## ກູ້ຄືນ backup (interactive ຖ້າບໍ່ມີ num)".into(),
    );

    m.insert(
        "help.backup.restore_choose".into(),
        "backup restore --choose ## ເລືອກ backup ຈາກເມນູ interactive".into(),
    );

    m.insert(
        "help.backup.restore_deleted".into(),
        "backup restore --deleted ## ກູ້ຄືນພຽງໄຟລ໌ທີ່ຖືກລຶບ".into(),
    );

    m.insert("help.backup.title".into(), "ຄຳສັ່ງ BACKUP:".into());

    m.insert(
        "help.backup.undo".into(),
        "backup undo ## ຍົກເລີກການກູ້ຄືນລ່າສຸດ".into(),
    );

    m.insert(
        "help.backup.undo_cls".into(),
        "backup undo --cls ## ລຶບ undo history (ຖາມຢືນຢັນ)".into(),
    );

    m.insert(
        "help.backup.undo_force".into(),
        "backup undo --force ## ລຶບ undo history (ບໍ່ຢືນຢັນ)".into(),
    );

    m.insert(
        "help.backup.example1".into(),
        "backup create ## ສ້າງ backup ໃໝ່".into(),
    );

    m.insert(
        "help.backup.example2".into(),
        "backup list ## ລາຍຊື່ backups ທັ່ງໝົດ".into(),
    );

    m.insert(
        "help.backup.example3".into(),
        "backup restore 1 ## ກູ້ຄືນ backup #1".into(),
    );

    m.insert(
        "help.backup.example4".into(),
        "backup diff 1 ## ປຽບທຽບກັບ backup #1".into(),
    );
}
