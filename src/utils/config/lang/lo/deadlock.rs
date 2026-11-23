use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "deadlock.add_usage".into(),
        "ວິທີໃຊ້: add <pass-env> <real-password>".into(),
    );
    m.insert("deadlock.added".into(), "ບັນທຶກ pass-env '{}' ແລ້ວ.".into());
    m.insert("deadlock.bye".into(), "ປິດ deadlock ແລ້ວ.".into());
    m.insert(
        "deadlock.confirm_master_prompt".into(),
        "ຢືນຢັນລະຫັດຜ່ານຫຼວຍໃໝ່: ".into(),
    );
    m.insert(
        "deadlock.corrupt_aborted".into(),
        "ຕູ້ເກັບອ່ານບໍ່ໄດ້ ແລະ ຕ້ອງກູ້ຄືນກ່ອນ deadlock ຈະເປີດໄດ້.".into(),
    );
    m.insert(
        "deadlock.corrupt_cancelled".into(),
        "ຍົກເລີກການກູ້ຄືນແລ້ວ; ຕູ້ເກັບຍັງບໍ່ຖືກແກ້ໄຂ.".into(),
    );
    m.insert(
        "deadlock.corrupt_kept".into(),
        "ເກັບຕູ້ເກັບທີ່ອ່ານບໍ່ໄດ້ໄວ້ເປັນ {}.".into(),
    );
    m.insert(
        "deadlock.corrupt_notice".into(),
        "ໄຟລ໌ຕູ້ເກັບທີ່ {} ອ່ານບໍ່ໄດ້ (ບໍ່ແມ່ນຕູ້ເກັບ deadlock ທີ່ຖືກຕ້ອງ).".into(),
    );
    m.insert(
        "deadlock.corrupt_recreate_failed".into(),
        "ຍ້າຍຕູ້ເກັບທີ່ອ່ານບໍ່ໄດ້ອອກບໍ່ໄດ້: {}".into(),
    );
    m.insert(
        "deadlock.corrupt_recreate_prompt".into(),
        "ສ້າງຕູ້ເກັບຫວ່າງໃໝ່ຕອນນີ້ບໍ? ໄຟລ໌ທີ່ອ່ານບໍ່ໄດ້ຈະຖືກເກັບໄວ້ຂ້າງຕູ້ເກັບ. [y/N]".into(),
    );
    m.insert(
        "deadlock.corrupt_recreated".into(),
        "ສ້າງຕູ້ເກັບຫວ່າງໃໝ່ແລ້ວ.".into(),
    );
    m.insert(
        "deadlock.corrupt_restore_failed".into(),
        "ກູ້ຄືນສຳຮອງບໍ່ໄດ້: {}".into(),
    );
    m.insert(
        "deadlock.corrupt_restore_prompt".into(),
        "ມີສຳຮອງຂອງຕູ້ເກັບ. ກູ້ຄືນບໍ? [y/N]".into(),
    );
    m.insert(
        "deadlock.corrupt_restored".into(),
        "ກູ້ຄືນຕູ້ເກັບຈາກສຳຮອງແລ້ວ.".into(),
    );
    m.insert("deadlock.created".into(), "ສ້າງຕູ້ເກັບໃໝ່ທີ່ {} ແລ້ວ.".into());
    m.insert(
        "deadlock.default_master_hint".into(),
        "ລະຫັດຜ່ານຫຼວຍເລີ່ມຕົ້ນ: {} - ລະບົບຈະໃຫ້ທ່ານປ່ຽນ.".into(),
    );
    m.insert(
        "deadlock.default_master_rejected".into(),
        "ເລືອກລະຫັດຜ່ານຫຼວຍອື່ນທີ່ບໍ່ແມ່ນຄ່າເລີ່ມຕົ້ນ.".into(),
    );
    m.insert(
        "deadlock.destroy_cancelled".into(),
        "ຍົກເລີກການລຶບຕູ້ເກັບແລ້ວ.".into(),
    );
    m.insert(
        "deadlock.destroy_confirm".into(),
        "ລຶບຕູ້ເກັບ ແລະ ຄວາມລັບທັງໝົດຢ່າງຖາວອນບໍ?".into(),
    );
    m.insert("deadlock.destroy_failed".into(), "ລຶບຕູ້ເກັບບໍ່ໄດ້: {}".into());
    m.insert("deadlock.destroyed".into(), "ລຶບຕູ້ເກັບແລ້ວ.".into());
    m.insert("deadlock.editor_fail".into(), "ເລີ່ມຕົວແກ້ໄຂແຖວບໍ່ໄດ້: {}".into());
    m.insert("deadlock.empty_master".into(), "ລະຫັດຜ່ານຫຼວຍຕ້ອງບໍ່ຫວ່າງ.".into());
    m.insert("deadlock.empty_password".into(), "ລະຫັດຜ່ານຕ້ອງບໍ່ຫວ່າງ.".into());
    m.insert(
        "deadlock.force_change_notice".into(),
        "ຍັງໃຊ້ລະຫັດຜ່ານຫຼວຍເລີ່ມຕົ້ນຢູ່; ກະລຸນາຕັ້ງລະຫັດໃໝ່ຕອນນີ້.".into(),
    );
    m.insert(
        "deadlock.forget_done".into(),
        "ລຶບລະຫັດຜ່ານຫຼວຍອອກຈາກທີ່ເກັບຂໍ້ມູນຮັບຮູ້ຂອງລະບົບແລ້ວ.".into(),
    );
    m.insert(
        "deadlock.forget_failed".into(),
        "ລຶບລະຫັດຜ່ານຫຼວຍອອກຈາກທີ່ເກັບຂໍ້ມູນຮັບຮູ້ຂອງລະບົບບໍ່ໄດ້: {}".into(),
    );
    m.insert(
        "deadlock.hint_help".into(),
        "ພິມ 'help' ເພື່ອເບິ່ງຄຳສັ່ງ deadlock.".into(),
    );
    m.insert(
        "deadlock.invalid_name".into(),
        "ຊື່ pass-env '{}' ບໍ່ຖືກຕ້ອງ (ໃຊ້ໄດ້ແຕ່ຕົວອັກສອນ, ຕົວເລກ, _ - .).".into(),
    );
    m.insert(
        "deadlock.keychain_registered".into(),
        "ລົງທະບຽນລະຫັດຜ່ານຫຼວຍໃນທີ່ເກັບຂໍ້ມູນຮັບຮູ້ຂອງລະບົບແລ້ວ (nssh ສາມາດປົດລັອກຕູ້ເກັບໄດ້; ໃຊ້ 'forget' ເພື່ອລຶບ)."
            .into(),
    );
    m.insert(
        "deadlock.keychain_unavailable".into(),
        "ທີ່ເກັບຂໍ້ມູນຮັບຮູ້ຂອງລະບົບໃຊ້ບໍ່ໄດ້ - nssh ຈະຖາມລະຫັດຜ່ານຫຼວຍ.".into(),
    );
    m.insert(
        "deadlock.ls_empty".into(),
        "ຕູ້ເກັບຫວ່າງ. ເພີ່ມດ້ວຍ: add <pass-env> <real-password>".into(),
    );
    m.insert("deadlock.ls_title".into(), "PASS-ENV -> ລະຫັດຜ່ານຈິງ:".into());
    m.insert(
        "deadlock.master_changed".into(),
        "ປ່ຽນລະຫັດຜ່ານຫຼວຍແລ້ວ; ຕູ້ເກັບຖືກເຂົ້າລະຫັດໃໝ່.".into(),
    );
    m.insert(
        "deadlock.master_mismatch".into(),
        "ລະຫັດຜ່ານບໍ່ກົງກັນ; ລອງອີກຄັ້ງ.".into(),
    );
    m.insert("deadlock.master_prompt".into(), "master-pass: ".into());
    m.insert(
        "deadlock.new_master_prompt".into(),
        "master-pass ໃໝ່: ".into(),
    );
    m.insert(
        "deadlock.no_args".into(),
        "deadlock ເປີດ shell ຂອງຕົນເອງ ແລະ ບໍ່ຮັບອາກິວເມັນ (ໃຊ້ --help ເພື່ອເບິ່ງຄຳສັ່ງ).".into(),
    );
    m.insert("deadlock.not_found".into(), "ບໍ່ພົບ pass-env ຊື່ '{}'.".into());
    m.insert(
        "deadlock.old_master_prompt".into(),
        "master-pass ເກົ່າ: ".into(),
    );
    m.insert(
        "deadlock.os_protected".into(),
        "ໄຟລ໌ຕູ້ເກັບຖືກປົກປ້ອງຈາກການລຶບໂດຍລະບົບປະຕິບັດການ.".into(),
    );
    m.insert(
        "deadlock.os_unprotected".into(),
        "ການປົກປ້ອງການລຶບລະດັບລະບົບບໍ່ມີໃນແພລດຟອມ/ບັນຊີນີ້ (ສິດທິໄຟລ໌ຍັງໃຊ້ໄດ້).".into(),
    );
    m.insert("deadlock.removed".into(), "ລຶບ pass-env '{}' ແລ້ວ.".into());
    m.insert("deadlock.rmpw_usage".into(), "ວິທີໃຊ້: rmpw <pass-env>".into());
    m.insert(
        "deadlock.too_many_attempts".into(),
        "ໃສ່ລະຫັດຜ່ານຫຼວຍຜິດຫຼາຍເກີນໄປ - ຍົກເລີກ.".into(),
    );
    m.insert(
        "deadlock.unknown_cmd".into(),
        "ບໍ່ຮູ້ຈັກຄຳສັ່ງ deadlock '{}'.".into(),
    );
    m.insert("deadlock.unlocked".into(), "ປົດລັອກຕູ້ເກັບແລ້ວ.".into());
    m.insert("deadlock.updated".into(), "ອັບເດດ pass-env '{}' ແລ້ວ.".into());
    m.insert("deadlock.value_prompt".into(), "real-password: ".into());
    m.insert(
        "deadlock.vault_missing".into(),
        "ຍັງບໍ່ມີຕູ້ເກັບ deadlock - ຣັນ `deadlock` ໜຶ່ງຄັ້ງເພື່ອສ້າງ.".into(),
    );
    m.insert(
        "deadlock.wrong_master".into(),
        "ລະຫັດຜ່ານຫຼວຍຜິດ (ຄັ້ງທີ {}/3).".into(),
    );
    m.insert("deadlock.wrong_master_once".into(), "ລະຫັດຜ່ານຫຼວຍຜິດ.".into());
}
