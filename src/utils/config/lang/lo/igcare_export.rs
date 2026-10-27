use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("igcare_export.cared_dirs".into(), "ໄດເຣັກທໍຣີທີ່ດູແລ".into());

    m.insert(
        "igcare_export.cared_extensions".into(),
        "ນາມສະກຸນທີ່ດູແລ".into(),
    );

    m.insert("igcare_export.cared_files".into(), "ໄຟລ໌ທີ່ດູແລ".into());

    m.insert("igcare_export.all".into(), "ເລືອກທັງໝົດ".into());

    m.insert("igcare_export.cancel".into(), "ຍົກເລີກ".into());

    m.insert(
        "igcare_export.extra_supported_extensions".into(),
        "ນາມສະກຸນທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert(
        "igcare_export.extra_supported_files".into(),
        "ໄຟລ໌ທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert("igcare_export.finish".into(), "ສຳເລັດການສົ່ງອອກ".into());

    m.insert("igcare_export.ignored_dirs".into(), "ໄດເຣັກທໍຣີທີ່ບໍ່ສົນໃຈ".into());

    m.insert(
        "igcare_export.ignored_extensions".into(),
        "ນາມສະກຸນທີ່ບໍ່ສົນໃຈ".into(),
    );

    m.insert("igcare_export.ignored_files".into(), "ໄຟລ໌ທີ່ບໍ່ສົນໃຈ".into());

    m.insert("igcare_export.items".into(), "{} ລາຍການ".into());

    m.insert("igcare_export.prompt".into(), "igcare-export>".into());

    m.insert("igcare_export.select".into(), "ເລືອກ/ຍົກເລີກຕາມເບີ".into());

    m.insert("igcare_export.selected".into(), "ຖືກເລືອກ".into());

    m.insert("igcare_export.selected_mark".into(), "(ຖືກເລືອກ)".into());

    m.insert(
        "igcare_export.title".into(),
        "igcare export — ເລືອກປະເພດ".into(),
    );
}
