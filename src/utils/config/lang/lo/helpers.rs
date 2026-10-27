use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "helpers.building_tree".into(),
        "ກຳລັງສ້າງ directory tree...".into(),
    );

    m.insert(
        "helpers.calculating_sizes".into(),
        "ກຳລັງຄິດໄລ່ຂະໜາດໄດເຣັກທໍຣີ...".into(),
    );

    m.insert("helpers.export_cancelled".into(), "ການສົ່ງອອກຖືກຍົກເລີກ.".into());

    m.insert(
        "helpers.exported_all".into(),
        "ສົ່ງອອກ alias ທັງໝົດ {} ໄປຍັງ: {}".into(),
    );

    m.insert(
        "helpers.exported_categories".into(),
        "ສົ່ງອອກ {} ປະເພດໄປຍັງ: {}".into(),
    );

    m.insert(
        "helpers.exported_count".into(),
        "ສົ່ງອອກ alias {} ໄປຍັງ: {}".into(),
    );

    m.insert(
        "helpers.exported_igcare_all".into(),
        "ສົ່ງອອກການຕັ້ງຄ່າ ignore/care ທັງໝົດໄປຍັງ: {}".into(),
    );

    m.insert("helpers.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: {}".into());

    m.insert(
        "helpers.imported_igcare".into(),
        "ນຳເຂົ້າການຕັ້ງຄ່າ ignore/care ຈາກ: {}".into(),
    );

    m.insert(
        "helpers.imported_ral".into(),
        "ນຳເຂົ້າ run aliases ຈາກ: {}".into(),
    );

    m.insert(
        "helpers.invalid_back_count".into(),
        "ຈຳນວນ back ບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert("helpers.invalid_input".into(), "ຂໍ້ມູນເຂົ້າບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert("helpers.invalid_num".into(), "ຈຳນວນບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "helpers.invalid_num_range".into(),
        "ຈຳນວນບໍ່ຖືກຕ້ອງ: {}. ໃຊ້ 1-{} ຫຼື 0 ເພື່ອຈົບ.".into(),
    );

    m.insert(
        "helpers.no_editor".into(),
        "ບໍ່ພົບ external editor. ກຳລັງເປີດ built-in editor...".into(),
    );

    m.insert(
        "helpers.no_ral_to_export".into(),
        "ບໍ່ມີ run aliases ໃຫ້ສົ່ງອອກ.".into(),
    );
}
