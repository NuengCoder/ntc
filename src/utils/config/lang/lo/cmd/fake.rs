use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.fake.usage_title".into(),
        "fake - ສ້າງຂໍ້ມູນຈຳລອງ/ສຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.usage_gen".into(),
        "  fake gen <ຊະນິດ> [-n N] [-f ຮູບແບບ] [-s ຊີດ] [-o ໄຟລ] [-g ກຸ່ມ] [-l ພາສາ] [-u [ຖັນ]]".into(),
    );

    m.insert(
        "cmd.fake.usage_types".into(),
        "  fake types [-s] [ຮູບແບບ]       ສະແດງຊະນິດທັງໝົດ (ຄົ້ນຫາແບບ fuzzy ດ້ວຍ -s)".into(),
    );

    m.insert(
        "cmd.fake.usage_locales".into(),
        "  fake locales                   ສະແດງລະຫັດ --locale ທີ່ຮອງຮັບ".into(),
    );

    m.insert(
        "cmd.fake.usage_group".into(),
        "  fake group <ຄຳສັ່ງຍ່ອຍ>        ຈັດການກຸ່ມຖັນທີ່ບັນທຶກໄວ້".into(),
    );

    m.insert(
        "cmd.fake.usage_help".into(),
        "  fake help                      ສະແດງຄວາມຊ່ວຍເຫຼືອນີ້".into(),
    );

    m.insert("cmd.fake.options_title".into(), "ຕົວເລືອກ (fake gen):".into());

    m.insert(
        "cmd.fake.opt_count".into(),
        "  -n, --count <N>       ຈຳນວນແຖວທີ່ຈະສ້າງ (ຄ່າເລີ່ມຕົ້ນ 1)".into(),
    );

    m.insert(
        "cmd.fake.opt_format".into(),
        "  -f, --format <ຮູບແບບ> json | txt | sql | csv | xlsx (ຄ່າເລີ່ມຕົ້ນ txt)".into(),
    );

    m.insert(
        "cmd.fake.opt_seed".into(),
        "  -s, --seed <N>        ຊີດສຳລັບຜົນລັບທີ່ເຮັດຊ້ຳໄດ້".into(),
    );

    m.insert(
        "cmd.fake.opt_output".into(),
        "  -o, --output <ໄຟລ>   ຂຽນລົງໄຟລ (ນາມສະກຸນກຳນົດຮູບແບບ)".into(),
    );

    m.insert(
        "cmd.fake.opt_group".into(),
        "  -g, --group <ຊື່>    ສ້າງຈາກກຸ່ມທີ່ບັນທຶກໄວ້ແທນ <ຊະນິດ>".into(),
    );

    m.insert(
        "cmd.fake.opt_locale".into(),
        "  -l, --locale <ລະຫັດ> ຂໍ້ມູນຕາມພາສາ: en_US en_GB zh_CN th_TH lo_LA".into(),
    );

    m.insert(
        "cmd.fake.opt_unique".into(),
        "  -u, --unique [ຖັນ]     ເຮັດໃຫ້ທັງແຖວ (ຫຼືຖັນທີ່ລະບຸ) ບໍ່ຊ້ຳ".into(),
    );

    m.insert("cmd.fake.examples_header".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.fake.example1".into(),
        "  fake gen name,email,city -n 3".into(),
    );

    m.insert(
        "cmd.fake.example2".into(),
        "  fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "cmd.fake.example3".into(),
        "  fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.example4".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.unknown_sub".into(),
        "ບໍ່ຮູ້ຈັກຄຳສັ່ງຍ່ອຍ fake: '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.wrote".into(),
        "ຂຽນແລ້ວ {} ແຖວໄປທີ່ {} ({})".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_ext".into(),
        "ບໍ່ຮູ້ຈັກນາມສະກຸນໄຟລຜົນລັບ '{}' (ໃຊ້ -f ເພື່ອກຳນົດຮູບແບບ)".into(),
    );

    m.insert(
        "cmd.fake.gen.xlsx_needs_output".into(),
        "ຜົນລັບ xlsx ຕ້ອງໃຊ້ -o <ໄຟລ.xlsx> (ບໍ່ສາມາດພິມໄຟລຖານຂໍ້ມູນໄບນາລີໄປຢັງເທີມິນັນໄດ້)".into(),
    );

    m.insert(
        "cmd.fake.gen.count_usage".into(),
        "fake gen: -n/--count ຕ້ອງມີຕົວເລກ".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_count".into(),
        "fake gen: ຈຳນວນບໍ່ຖືກຕ້ອງ '{}' (ຕ້ອງເປັນຈຳນວນເຕັມບວກ)".into(),
    );

    m.insert(
        "cmd.fake.gen.format_usage".into(),
        "fake gen: -f/--format ຕ້ອງມີຄ່າ".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_format".into(),
        "fake gen: ບໍ່ຮູ້ຈັກຮູບແບບ '{}' (json|txt|sql|csv|xlsx)".into(),
    );

    m.insert(
        "cmd.fake.gen.seed_usage".into(),
        "fake gen: -s/--seed ຕ້ອງມີຕົວເລກ".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_seed".into(),
        "fake gen: ຊີດບໍ່ຖືກຕ້ອງ '{}' (ຕ້ອງເປັນຈຳນວນເຕັມ)".into(),
    );

    m.insert(
        "cmd.fake.gen.output_usage".into(),
        "fake gen: -o/--output ຕ້ອງມີເສັ້ນທາງໄຟລ".into(),
    );

    m.insert(
        "cmd.fake.gen.group_usage".into(),
        "fake gen: -g/--group ຕ້ອງມີຊື່ກຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_flag".into(),
        "fake gen: ຕົວເລືອກທີ່ບໍ່ຮູ້ຈັກ '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.group_and_types".into(),
        "fake gen: <ຊະນິດ> ແລະ -g/--group ໃຊ້ຮ່ວມກັນບໍ່ໄດ້".into(),
    );

    m.insert(
        "cmd.fake.gen.need_input".into(),
        "fake gen: ໃຫ້ຊະນິດຢ່າງໜ້ອຍໜຶ່ງ ຫຼືໃຊ້ -g/--group".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_type".into(),
        "fake gen: ບໍ່ຮູ້ຈັກຊະນິດ '{}' (ໂຫຼດ 'fake types' ເພື່ອເບິ່ງທັງໝົດ)".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_with_group".into(),
        "fake gen: ໃຊ້ --unique ກັບ -g/--group ຮ່ວມກັນບໍ່ໄດ້ (ໃຫ້ໃສ່ '!' ທ້າຍຖັນໃນກຸ່ມແທນ)".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_unknown_col".into(),
        "fake gen: --unique ລະບຸຖັນທີ່ບໍ່ຮູ້ຈັກ '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_exhausted".into(),
        "fake gen: ສ້າງຄ່າທີ່ບໍ່ຊ້ຳສຳລັບ '{}' ບໍ່ໄດ້ (ພື້ນທີ່ຄ່າໜ້ອຍເກີນໄປ)".into(),
    );

    m.insert(
        "cmd.fake.gen.locale_usage".into(),
        "fake gen: -l/--locale ຕ້ອງມີລະຫັດພາສາ (en_US en_GB zh_CN th_TH lo_LA)".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_locale".into(),
        "fake gen: ບໍ່ຮູ້ຈັກພາສາ '{}' (en_US en_GB zh_CN th_TH lo_LA)".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_cycle".into(),
        "fake gen: ການອ້າງອີງກຸ່ມເປັນວົງຈອນຜ່ານ '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_unknown_col".into(),
        "fake gen: ບໍ່ພົບຖັນທີ່ອ້າງອີງ '{}' ໃນກຸ່ມ '{}'".into(),
    );

    m.insert("cmd.fake.types.title".into(), "ຊະນິດ fake ທີ່ໃຊ້ໄດ້ ({})".into());

    m.insert("cmd.fake.types.none".into(), "ບໍ່ມີຊະນິດທີ່ກົງກັບ '{}'".into());

    m.insert(
        "cmd.fake.types.matches".into(),
        "ຊະນິດທີ່ກົງກັບ '{}' ({}):".into(),
    );

    m.insert("cmd.fake.locales.title".into(), "ໂລເຄລທີ່ຮອງຮັບ ({}):".into());

    m.insert(
        "cmd.fake.locales.note".into(),
        "ໃຊ້ກັບ: fake gen <ຊະນິດ> --locale <ລະຫັດ>".into(),
    );

    m.insert(
        "cmd.fake.group.usage_title".into(),
        "fake group - ຈັດການກຸ່ມຖັນທີ່ບັນທຶກໄວ້:".into(),
    );

    m.insert(
        "cmd.fake.group.help_add".into(),
        "  fake group add <ຊື່> [ນາມແຝງ as ຊະນິດ, ...]    ສ້າງກຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.group.help_edit".into(),
        "  fake group edit <ຊື່> [ນາມແຝງ as ຊະນິດ, ...]   ແທນຖັນຂອງກຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.group.help_rm".into(),
        "  fake group rm <ຊື່>                            ລຶບກຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.group.help_rnm".into(),
        "  fake group rnm <ເກົ່າ> <ໃໝ່>                    ປ່ຽນຊື່ກຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.group.help_ls".into(),
        "  fake group ls [ຮູບແບບ]                          ສະແດງກຸ່ມ (wildcard ເຊັ່ນ u*)".into(),
    );

    m.insert(
        "cmd.fake.group.help_info".into(),
        "  fake group info <ຊື່>                          ສະແດງຖັນຂອງກຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.group.help_clear".into(),
        "  fake group clear [-f]                           ລຶບທຸກກຸ່ມ".into(),
    );

    m.insert(
        "cmd.fake.group.help_export".into(),
        "  fake group export <ຊື່>|--all [-o ໄຟລ]        ສົ່ງອອກກຸ່ມເປັນ .ntc_fake".into(),
    );

    m.insert(
        "cmd.fake.group.help_import".into(),
        "  fake group import <ໄຟລ> [--force] [--prefix p]  ນຳເຂົ້າກຸ່ມຈາກໄຟລ".into(),
    );

    m.insert(
        "cmd.fake.group.example1".into(),
        "  fake group add users [username as name, email, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.example2".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_sub".into(),
        "ບໍ່ຮູ້ຈັກຄຳສັ່ງຍ່ອຍ 'fake group': '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.no_columns".into(),
        "ໃຫ້ຢ່າງໜ້ອຍໜຶ່ງຖັນ ເຊັ່ນ [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_column".into(),
        "ບໍ່ສາມາດແຍກຖັນ '{}' (ຕ້ອງເປັນ 'ນາມແຝງ as ຊະນິດ')".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_type".into(),
        "ບໍ່ຮູ້ຈັກຊະນິດ '{}' (ໂຫຼດ 'fake types' ເພື່ອເບິ່ງທັງໝົດ)".into(),
    );

    m.insert(
        "cmd.fake.group.add_usage".into(),
        "ວິທີໃຊ້: fake group add <ຊື່> [ນາມແຝງ as ຊະນິດ, ...]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_name".into(),
        "ຊື່ກຸ່ມບໍ່ຖືກຕ້ອງ '{}' (ໃຊ້ຕົວອັກສອນ ຕົວເລກ '_' ຫຼື '-')".into(),
    );

    m.insert(
        "cmd.fake.group.not_found".into(),
        "ບໍ່ພົບກຸ່ມ fake ຊື່ '{}'".into(),
    );

    m.insert("cmd.fake.group.exists".into(), "ມີກຸ່ມ fake '{}' ແລ້ວ".into());

    m.insert("cmd.fake.group.scope_local".into(), "ທ້ອງຖິ່ນ".into());

    m.insert("cmd.fake.group.scope_global".into(), "ສ່ວນກາງ".into());

    m.insert(
        "cmd.fake.group.added".into(),
        "ເພີ່ມກຸ່ມ '{}' ພ້ອມ {} ຖັນໄປຍັງການຕັ້ງຄ່າ{}ແລ້ວ".into(),
    );

    m.insert(
        "cmd.fake.group.updated".into(),
        "ອັບເດດກຸ່ມ '{}' ພ້ອມ {} ຖັນໃນການຕັ້ງຄ່າ{}ແລ້ວ".into(),
    );

    m.insert(
        "cmd.fake.group.rm_usage".into(),
        "ວິທີໃຊ້: fake group rm <ຊື່>".into(),
    );

    m.insert("cmd.fake.group.removed".into(), "ລຶບກຸ່ມ '{}' ແລ້ວ".into());

    m.insert(
        "cmd.fake.group.rnm_usage".into(),
        "ວິທີໃຊ້: fake group rnm <ເກົ່າ> <ໃໝ່>".into(),
    );

    m.insert(
        "cmd.fake.group.renamed".into(),
        "ປ່ຽນຊື່ກຸ່ມ '{}' ເປັນ '{}' ແລ້ວ".into(),
    );

    m.insert(
        "cmd.fake.group.ls_empty".into(),
        "ຍັງບໍ່ມີກຸ່ມ fake ທີ່ບັນທຶກໄວ້".into(),
    );

    m.insert("cmd.fake.group.ls_none".into(), "ບໍ່ມີກຸ່ມທີ່ກົງກັບ '{}'".into());

    m.insert(
        "cmd.fake.group.info_usage".into(),
        "ວິທີໃຊ້: fake group info <ຊື່>".into(),
    );

    m.insert("cmd.fake.group.info_title".into(), "ກຸ່ມ '{}'".into());

    m.insert("cmd.fake.group.info_count".into(), "{} ຖັນ".into());

    m.insert(
        "cmd.fake.group.clear_empty".into(),
        "ບໍ່ມີກຸ່ມ fake ທີ່ຈະລຶບ".into(),
    );

    m.insert(
        "cmd.fake.group.clear_prompt".into(),
        "ລຶບ {} ກຸ່ມ fake ຫຼືບໍ່? [y/N] ".into(),
    );

    m.insert("cmd.fake.group.clear_cancelled".into(), "ຍົກເລິກແລ້ວ".into());

    m.insert("cmd.fake.group.cleared".into(), "ລຶບ {} ກຸ່ມ fake ແລ້ວ".into());

    m.insert(
        "cmd.fake.group.bad_ref".into(),
        "ການອ້າງອີງບໍ່ຖືກຕ້ອງ '{}' (ຕ້ອງເປັນ ref:<ກຸ່ມ>.<ຖັນ>)".into(),
    );

    m.insert(
        "cmd.fake.group.ref_col_missing".into(),
        "ບໍ່ພົບຖັນທີ່ອ້າງອີງ '{}' ໃນກຸ່ມ '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.export_usage".into(),
        "ວິທີໃຊ້: fake group export <ຊື່>|--all [-o ໄຟລ]".into(),
    );

    m.insert(
        "cmd.fake.group.import_usage".into(),
        "ວິທີໃຊ້: fake group import <ໄຟລ> [--force] [--prefix p]".into(),
    );

    m.insert(
        "cmd.fake.group.exported".into(),
        "ສົ່ງອອກ {} ກຸ່ມໄປຍັງ {} ແລ້ວ".into(),
    );

    m.insert("cmd.fake.group.import_ok".into(), "ນຳເຂົ້າ {} ກຸ່ມແລ້ວ".into());

    m.insert(
        "cmd.fake.group.import_exists".into(),
        "ມີກຸ່ມຢູ່ແລ້ວ: {} (ໃຊ້ --force ເພື່ອຂຽນທັບ)".into(),
    );

    m.insert(
        "cmd.fake.group.import_bad_file".into(),
        "ນຳເຂົ້າກຸ່ມບໍ່ສຳເລັດ: {}".into(),
    );
}
