use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("theme.create_failed".into(), "ສ້າງ theme ບໍ່ສຳເລັດ: {}".into());

    m.insert("theme.create_file_failed".into(), "ສ້າງໄຟລ໌ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "theme.create_first".into(),
        "ເຮັດວຽກ 'theme add <name>' ເພື່ອສ້າງ theme ທຳອິດຂອງທ່ານ".into(),
    );

    m.insert(
        "theme.created".into(),
        "ສ້າງ theme '{}' ແລ້ວ (ເປີດໃນ editor)".into(),
    );

    m.insert(
        "theme.created_hint".into(),
        "ແກ້ໄຂໄຟລ໌, ບັນທຶກ, ຈາກນັ້ນເຮັດວຽກ 'theme reload' ເພື່ອໃຊ້ການປ່ຽນແປງ".into(),
    );

    m.insert("theme.current_title".into(), "theme ປັດຈຸບັນ: {}".into());

    m.insert("theme.dir_title".into(), "ໄດເຣັກທໍຣີ theme:".into());

    m.insert(
        "theme.edit_hint".into(),
        "ແກ້ໄຂ '{}', ບັນທຶກ, ຈາກນັ້ນເຮັດວຽກ 'theme reload'".into(),
    );

    m.insert(
        "theme.export_failed".into(),
        "ສົ່ງອອກ theme ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "theme.export_failed_simple".into(),
        "ສົ່ງອອກບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("theme.exported_simple".into(), "ສົ່ງອອກໄປທີ່: {}".into());

    m.insert(
        "theme.exported_to".into(),
        "ສົ່ງອອກ theme '{}' ໄປທີ່: {}".into(),
    );

    m.insert("theme.file_exists".into(), "ໄຟລ໌ມີຢູ່ແລ້ວ: {}".into());

    m.insert("theme.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: {}".into());

    m.insert("theme.files_title".into(), "ໄຟລ໌ theme:".into());

    m.insert(
        "theme.import_failed".into(),
        "ນຳເຂົ້າ theme ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "theme.import_failed_simple".into(),
        "ນຳເຂົ້າບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "theme.import_hint".into(),
        "ເຮັດວຽກ 'theme list' ເພື່ອເບິ່ງ themes ທີ່ມີ".into(),
    );

    m.insert("theme.imported".into(), "ນຳເຂົ້າ theme ຈາກ: {}".into());

    m.insert("theme.info_author".into(), "Author".into());

    m.insert("theme.info_defined".into(), "ກຳນົດແລ້ວ".into());

    m.insert("theme.info_description".into(), "Description".into());

    m.insert("theme.info_editor".into(), "Editor colors".into());

    m.insert("theme.info_shell".into(), "Shell colors".into());

    m.insert("theme.info_syntax".into(), "Syntax colors".into());

    m.insert("theme.info_title".into(), "Theme: {}".into());

    m.insert(
        "theme.invalid_number".into(),
        "ຈຳນວນບໍ່ຖືກຕ້ອງ: {}. ໃຊ້ 1-{}".into(),
    );

    m.insert("theme.name_empty".into(), "ຊື່ theme ບໍ່ສາມາດວ່າງເປົ່າ".into());

    m.insert("theme.no_dir".into(), "ກຳນົດໄດເຣັກທໍຣີ theme ບໍ່ໄດ້".into());

    m.insert("theme.no_dir_yet".into(), "(ໄດເຣັກທໍຣີຍັງບໍ່ມີ)".into());

    m.insert("theme.no_files".into(), "(ບໍ່ມີໄຟລ໌ theme)".into());

    m.insert("theme.not_found".into(), "ບໍ່ພົບ Theme '{}'".into());

    m.insert(
        "theme.not_found_hint".into(),
        "ບໍ່ພົບ theme: {}. ໃຊ້ 'theme list' ເພື່ອເບິ່ງ themes ທີ່ມີ.".into(),
    );

    m.insert("theme.opened_dir".into(), "ເປີດໄດເຣັກທໍຣີ theme: {}".into());

    m.insert(
        "theme.reloaded".into(),
        "ໂຫຼດ themes ຄືນໃໝ່ຈາກ disk ແລ້ວ".into(),
    );

    m.insert("theme.remove_failed".into(), "ລຶບ theme ບໍ່ສຳເລັດ: {}".into());

    m.insert("theme.removed".into(), "ລຶບ theme: {} ແລ້ວ".into());

    m.insert("theme.rename_failed".into(), "ປ່ຽນຊື່ theme ບໍ່ສຳເລັດ: {}".into());

    m.insert("theme.rename_failed_simple".into(), "ປ່ຽນຊື່ບໍ່ສຳເລັດ: {}".into());

    m.insert("theme.renamed".into(), "ປ່ຽນຊື່ theme '{}' -> '{}'".into());

    m.insert("theme.renamed_simple".into(), "ປ່ຽນຊື່ '{}' -> '{}'".into());

    m.insert(
        "theme.switch_failed".into(),
        "ສະຫຼັບໄປ theme ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "theme.switch_failed_simple".into(),
        "ສະຫຼັບໄປ theme ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("theme.switched".into(), "ສະຫຼັບໄປ theme: {} ແລ້ວ".into());

    m.insert(
        "theme.template_created".into(),
        "ສ້າງ theme template: {}".into(),
    );

    m.insert(
        "theme.template_info".into(),
        "ນີ້ແມ່ນໄຟລ໌ template ທ້ອງຖິ່ນ. ໃຊ້ 'theme import' ເພື່ອເພີ່ມໃສ່ລາຍຊື່ theme ຂອງທ່ານ.".into(),
    );

    m.insert(
        "theme.type_interactive".into(),
        "ພິມ 'theme' ສຳລັບເມນູ interactive".into(),
    );

    m.insert("theme.ui.current".into(), "theme ປັດຈຸບັນ".into());

    m.insert("theme.ui.exit".into(), "ກຳລັງອອກຈາກ theme manager.".into());

    m.insert("theme.ui.name_prompt".into(), "ຊື່ theme: ".into());

    m.insert("theme.ui.theme_prompt".into(), "theme>".into());

    m.insert("theme.ui.title".into(), "ntc Theme Manager".into());

    m.insert("theme.unknown_cmd".into(), "ຄຳສັ່ງບໍ່ຮູ້ຈັກ: {}".into());

    m.insert("theme.usage_add".into(), "Usage: theme add <name>".into());

    m.insert(
        "theme.usage_update".into(),
        "Usage: theme update <name>".into(),
    );

    m.insert(
        "theme.updated".into(),
        "ອັບເດດ theme '{}' ແລ້ວ: ເພີ່ມ {} ຟິລດ໌ທີ່ຂາດ".into(),
    );

    m.insert("theme.up_to_date".into(), "theme '{}' ມີທຸກຟິລດ໌ແລ້ວ".into());

    m.insert("theme.update_failed".into(), "ອັບເດດບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "theme.usage_export".into(),
        "Usage: theme export <name> [output_file]".into(),
    );

    m.insert(
        "theme.usage_import".into(),
        "Usage: theme import <file.ntc_theme>".into(),
    );

    m.insert("theme.usage_info".into(), "Usage: theme info <name>".into());

    m.insert(
        "theme.usage_rename".into(),
        "Usage: theme rename <old_name> to <new_name>".into(),
    );

    m.insert(
        "theme.usage_rm".into(),
        "ການໃຊ້: theme rm <ຊື່> [<ຊື່> ...] [-f|--force]".into(),
    );

    m.insert("theme.usage_use".into(), "ການໃຊ້: theme use <ຊື່>".into());

    m.insert("theme.remove_confirm".into(), "ລຶບ {} ທີມ ({})? [y/N]".into());

    m.insert("theme.remove_cancelled".into(), "ຍົກເລີກແລ້ວ.".into());

    m.insert(
        "theme.remove_current".into(),
        "ບໍ່ສາມາດລຶບທີມທີ່ໃຊ້ຢູ່ '{}' ໄດ້; ກະລຸນາປ່ຽນໄປທີມອື່ນກ່ອນ".into(),
    );

    m.insert(
        "theme.remove_builtin".into(),
        "ບໍ່ສາມາດລຶບທີມໃນຕົວ '{}' ໄດ້".into(),
    );

    m.insert(
        "theme.usage_rnm".into(),
        "Usage: theme rnm <old_name> to <new_name>".into(),
    );

    m.insert(
        "theme.install_downloading".into(),
        "ກຳລັງດາວໂຫຼດ {}...".into(),
    );

    m.insert("theme.install_failed".into(), "ຕິດຕັ້ງທີມບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "theme.install_fetching".into(),
        "ກຳລັງດຶງໄຟລ໌ .ntc_theme ຈາກ GitHub...".into(),
    );

    m.insert(
        "theme.install_none".into(),
        "ບໍ່ພົບໄຟລ໌ .ntc_theme ທີ່ຕິດຕັ້ງໄດ້".into(),
    );

    m.insert("theme.install_prompt".into(), "ເລືອກທີມ (0 ເພື່ອຍົກເລີກ):".into());

    m.insert("theme.install_title".into(), "ຕິດຕັ້ງທີມ".into());

    m.insert("theme.installed".into(), "ຕິດຕັ້ງທີມ '{}' ແລ້ວ".into());

    m.insert(
        "theme.installed_active".into(),
        "ຕິດຕັ້ງ ແລະ ສະຫຼັບໄປທີມ '{}' ແລ້ວ".into(),
    );
}
