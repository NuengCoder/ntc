use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.fmt.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: {}".into());

    m.insert(
        "cmd.fmt.binary_file".into(),
        "ພົບໄຟລ໌ binary ຫຼືທີ່ບີບອັດ fmt ໃຊ້ໄດ້ກັບໄຟລ໌ຂໍ້ຄວາມເທົ່ານັ້ນ".into(),
    );

    m.insert(
        "cmd.fmt.cross_family".into(),
        "Cannot convert '{}' to '{}': different format families ({} vs {}).".into(),
    );

    m.insert(
        "cmd.fmt.doc_ser_err".into(),
        "ຍືດຫຍັງເອກະສານເປັນ '{}' ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.fmt.docx_needs_output".into(),
        "DOCX output ຕ້ອງກຳນົດຝນອກົດແຟ້ມ (-o <file>).".into(),
    );

    m.insert(
        "cmd.fmt.pdf_needs_output".into(),
        "PDF output ຕ້ອງກຳນົດຝນອກົດແຟ້ມ (-o <file>).".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_needs_output".into(),
        "XLSX output ຕ້ອງກຳນົດຝນອກົດແຟ້ມ (-o <file>).".into(),
    );

    m.insert(
        "cmd.fmt.pdf_source_unsupported".into(),
        "ບໍ່ຮອງຮັບ PDF ເປັນ input; fmt ຂຽນ PDF ໄດ້ຢ່າງດຽວ (-o <file.pdf>).".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_source_unsupported".into(),
        "ບໍ່ຮອງຮັບ XLSX ເປັນ input; fmt ຂຽນ XLSX ໄດ້ຢ່າງດຽວ (-o <file.xlsx>).".into(),
    );

    m.insert(
        "cmd.fmt.pdf_unsupported".into(),
        "ແພລດຟອມນີ້ບໍ່ຮອງຮັບ PDF output.".into(),
    );

    m.insert("cmd.fmt.invalid_json".into(), "JSON ບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert("cmd.fmt.invalid_toml".into(), "TOML ບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert("cmd.fmt.invalid_yaml".into(), "YAML ບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "cmd.fmt.json_ser_err".into(),
        "ຜິດພາດ JSON serialization: {}".into(),
    );

    m.insert("cmd.fmt.missing_file".into(), "ຂາດ path ໄຟລ໌".into());

    m.insert("cmd.fmt.read_fail".into(), "ອ່ານໄຟລ໌ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "cmd.fmt.toml_conv_err".into(),
        "ຜິດພາດການແປງ TOML: {}".into(),
    );

    m.insert(
        "cmd.fmt.toml_ser_err".into(),
        "ຜິດພາດ TOML serialization: {}".into(),
    );

    m.insert("cmd.fmt.unknown_format".into(), "ບໍ່ສະໜັບສະໜູນປະເພດໄຟລ໌ '{}' ສຳລັບ fmt. ໃຊ້ໄຟລ໌ທີ່ມີແຕ່ລະນາມສະກຸນ: json, yaml, toml, md, html, xml, docx, txt, pdf, csv, tsv, ຫຼື xlsx.".into());

    m.insert("cmd.fmt.unknown_target".into(), "ຮູບແບບເປົ້າໝາຍບໍ່ຮູ້ຈັກ: '{}'. ໃຊ້ json, yaml, toml, md, html, xml, docx, txt, pdf, csv, tsv, ຫຼື xlsx.".into());

    m.insert(
        "cmd.fmt.legacy_syntax".into(),
        "ຮູບແບບວິທີໃຊ້ປັບໃໝ່: ສູດຕົ້ນທາງຈາກນາມສະກຸນໄຟລ໌. ໃຊ້: fmt <file> [to <format>] [-o <output>]".into(),
    );

    m.insert(
        "cmd.fmt.usage".into(),
        "ວິທີໃຊ້: fmt <file> [to json|yaml|toml|md|html|xml|docx|txt|pdf|csv|tsv|xlsx] [-o <output>]"
            .into(),
    );

    m.insert("cmd.fmt.written_to".into(), "ຂຽນໄປທີ່: {}".into());

    m.insert(
        "cmd.fmt.yaml_ser_err".into(),
        "ຜິດພາດ YAML serialization: {}".into(),
    );
}
