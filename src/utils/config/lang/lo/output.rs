use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("output.filename_header".into(), "{}:".into());

    m.insert("output.line_prefix".into(), "LINE {}>>".into());

    m.insert(
        "output.range_past_eof".into(),
        "(range ເກີນທ້າຍໄຟລ໌ — ບໍ່ມີຫຍັງສະແດງ)".into(),
    );

    m.insert("output.mkdir_failed".into(), "ສ້າງໄດເຣັກທໍຣີບໍ່ສຳເລັດ: {}".into());

    m.insert("output.mkfile_failed".into(), "ສ້າງໄຟລ໌ບໍ່ສຳເລັດ: {}".into());

    m.insert("output.print_error".into(), "\u{2717} {}".into());

    m.insert("output.print_info".into(), "\u{2139} {}".into());

    m.insert("output.print_success".into(), "\u{2713} {}".into());

    m.insert("output.print_warning".into(), "\u{26A0} {}".into());

    m.insert(
        "output.read_file_failed".into(),
        "ອ່ານໄຟລ໌ '{}' ບໍ່ສຳເລັດ".into(),
    );

    m.insert("output.write_failed".into(), "ຂຽນໄປຍັງໄຟລ໌ບໍ່ສຳເລັດ: {}".into());
}
