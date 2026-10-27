use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("output.filename_header".into(), "{}：".into());

    m.insert("output.line_prefix".into(), "行 {} >>".into());

    m.insert(
        "output.range_past_eof".into(),
        "（范围超出文件末尾 — 无内容可显示）".into(),
    );

    m.insert("output.mkdir_failed".into(), "创建目录失败：{}".into());

    m.insert("output.mkfile_failed".into(), "创建文件失败：{}".into());

    m.insert("output.print_error".into(), "\u{2717} {}".into());

    m.insert("output.print_info".into(), "\u{2139} {}".into());

    m.insert("output.print_success".into(), "\u{2713} {}".into());

    m.insert("output.print_warning".into(), "\u{26A0} {}".into());

    m.insert(
        "output.read_file_failed".into(),
        "读取文件 '{}' 失败".into(),
    );

    m.insert("output.write_failed".into(), "写入文件失败：{}".into());
}
