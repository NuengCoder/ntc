use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.fmt.file_not_found".into(), "ไม่พบไฟล์: {}".into());

    m.insert(
        "cmd.fmt.binary_file".into(),
        "ตรวจพบไฟล์ไบนารีหรือไฟล์บีบอัด fmt ใช้ได้เฉพาะไฟล์ข้อความเท่านั้น".into(),
    );

    m.insert(
        "cmd.fmt.cross_family".into(),
        "แปลง '{}' เป็น '{}' ไม่ได้: คนละตระกูลรูปแบบ ({} กับ {})".into(),
    );

    m.insert(
        "cmd.fmt.doc_ser_err".into(),
        "แปลงเอกสารเป็น '{}' ไม่สำเร็จ: {}".into(),
    );

    m.insert(
        "cmd.fmt.docx_needs_output".into(),
        "การปล่อย DOCX ต้องระบุไฟล์ปลายทาง (-o <ไฟล์>)".into(),
    );

    m.insert(
        "cmd.fmt.pdf_needs_output".into(),
        "การปล่อย PDF ต้องระบุไฟล์ปลายทาง (-o <ไฟล์>)".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_needs_output".into(),
        "การปล่อย XLSX ต้องระบุไฟล์ปลายทาง (-o <ไฟล์>)".into(),
    );

    m.insert(
        "cmd.fmt.pdf_source_unsupported".into(),
        "ไม่รองรับการอ่าน PDF; fmt ปล่อย PDF ได้เท่านั้น (-o <ไฟล์.pdf>)".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_source_unsupported".into(),
        "ไม่รองรับการอ่าน XLSX; fmt ปล่อย XLSX ได้เท่านั้น (-o <ไฟล์.xlsx>)".into(),
    );

    m.insert(
        "cmd.fmt.pdf_unsupported".into(),
        "แพลตฟอร์มนี้ไม่รองรับการปล่อย PDF".into(),
    );

    m.insert("cmd.fmt.invalid_json".into(), "JSON ไม่ถูกต้อง: {}".into());

    m.insert("cmd.fmt.invalid_toml".into(), "TOML ไม่ถูกต้อง: {}".into());

    m.insert("cmd.fmt.invalid_yaml".into(), "YAML ไม่ถูกต้อง: {}".into());

    m.insert(
        "cmd.fmt.json_ser_err".into(),
        "ข้อผิดพลาดการแปลง JSON เป็นอนุกรม: {}".into(),
    );

    m.insert("cmd.fmt.missing_file".into(), "ไม่พบเส้นทางไฟล์".into());

    m.insert("cmd.fmt.read_fail".into(), "ไม่สามารถอ่านไฟล์: {}".into());

    m.insert(
        "cmd.fmt.toml_conv_err".into(),
        "ข้อผิดพลาดการแปลง TOML: {}".into(),
    );

    m.insert(
        "cmd.fmt.toml_ser_err".into(),
        "ข้อผิดพลาดการแปลง TOML เป็นอนุกรม: {}".into(),
    );

    m.insert("cmd.fmt.unknown_format".into(), "ไม่รองรับไฟล์ชนิด '{}' สำหรับ fmt ใช้ไฟล์ที่มีนามสกุล: json, yaml, toml, md, html, xml, docx, txt, pdf, csv, tsv หรือ xlsx".into());

    m.insert("cmd.fmt.unknown_target".into(), "รูปแบบเป้าหมายไม่รู้จัก: '{}'. ใช้ json, yaml, toml, md, html, xml, docx, txt, pdf, csv, tsv หรือ xlsx.".into());

    m.insert(
        "cmd.fmt.legacy_syntax".into(),
        "รูปแบบคำสั่งเปลี่ยนไป: รูปแบบต้นทางใช้จากนามสกุลไฟล์ ใช้: fmt <ไฟล์> [to <รูปแบบ>] [-o <ผลลัพธ์>]".into(),
    );

    m.insert("cmd.fmt.usage".into(), "การใช้งาน: fmt <ไฟล์> [to json|yaml|toml|md|html|xml|docx|txt|pdf|csv|tsv|xlsx] [-o <ผลลัพธ์>]".into());

    m.insert("cmd.fmt.written_to".into(), "เขียนไปที่: {}".into());

    m.insert(
        "cmd.fmt.yaml_ser_err".into(),
        "ข้อผิดพลาดการแปลง YAML เป็นอนุกรม: {}".into(),
    );
}
