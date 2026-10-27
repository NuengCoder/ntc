use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("nfe.binary_file".into(), "(binary / ຮູບແບບທີ່ບໍ່ຮອງຮັບ)".into());

    m.insert("nfe.binary_file_preview".into(), "(binary file)".into());

    m.insert("nfe.empty_dir".into(), "(ໄດເຣັກທໍຣີວ່າງ)".into());

    m.insert("nfe.file_truncated".into(), "... (ໄຟລ໌ຖືກຕັດ)".into());

    m.insert(
        "nfe.hint_bar".into(),
        "h help | b ກັບ | f find | / filter | q ອອກ".into(),
    );

    m.insert("nfe.no_matches".into(), "(ບໍ່ມີການກົງ)".into());

    m.insert("nfe.preview_empty".into(), "(ວ່າງ)".into());

    m.insert("nfe.preview_error".into(), "(ອ່ານບໍ່ໄດ້)".into());

    m.insert("nfe.preview_more".into(), "... (ເພີ່ມ)".into());

    m.insert("nfe.sort_date".into(), "Date".into());

    m.insert("nfe.sort_name".into(), "Name".into());

    m.insert("nfe.sort_size".into(), "Size".into());

    m.insert("nfe.sort_type".into(), "Type".into());

    m.insert(
        "nfe.status_bar".into(),
        "{} ລາຍການ | {} dirs | {} ໄຟລ໌ | Sort: {} {}".into(),
    );
}
