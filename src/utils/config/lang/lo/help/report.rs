use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.report.cp".into(),
        "report <fmt> --cp ## ສຳເນົາບົດລາຍງານໃສ່ clipboard ແທນການບັນທຶກ".into(),
    );

    m.insert(
        "help.report.docx".into(),
        "report docx [path] ## ສ້າງບົດລາຍງານ DOCX".into(),
    );

    m.insert(
        "help.report.html".into(),
        "report html [path] ## ສ້າງບົດລາຍງານ HTML".into(),
    );

    m.insert(
        "help.report.json".into(),
        "report json [path] ## ສ້າງບົດລາຍງານ JSON".into(),
    );

    m.insert(
        "help.report.md".into(),
        "report md [path] ## ສ້າງບົດລາຍງານ Markdown".into(),
    );

    m.insert(
        "help.report.pdf".into(),
        "report pdf [path] ## ສ້າງບົດລາຍງານ PDF".into(),
    );

    m.insert("help.report.title".into(), "ຄຳສັ່ງ REPORT:".into());

    m.insert(
        "help.report.txt".into(),
        "report txt [path] ## ສ້າງບົດລາຍງານ TXT".into(),
    );

    m.insert(
        "help.report.txtc".into(),
        "txtc <file> ## ສຳເນົາເນື້ອໃນໄຟລ໌ໃສ່ clipboard".into(),
    );

    m.insert("help.report.txtf".into(), "txtf [--head N|--tail N|--betw A:B] [file|dir|@tp/file|@tp/dir] ## ສະແດງເນື້ອໃນໄຟລ໌ ຫຼື ເລືອກໄຟລ໌ຈາກ directory (ເລກແຖວເດີມ)".into());

    m.insert(
        "help.report.txtf_example1".into(),
        "txtf main.rs --head 30 ## 30 ແຖວທຳອິດຂອງ main.rs".into(),
    );

    m.insert(
        "help.report.txtf_example2".into(),
        "txtf --betw 10:20 main.rs ## ແຖວ 10-20 ຂອງ main.rs".into(),
    );

    m.insert(
        "help.report.txtf_example3".into(),
        "txtf @docs/notes.txt ## ສະແດງໄຟລ໌ໃນ teleport".into(),
    );

    m.insert(
        "help.report.txtf_example4".into(),
        "txtf src ## ເລືອກໄຟລ໌ທີ່ຮອງຮັບຈາກ directory src ເພື່ອສະແດງ".into(),
    );

    m.insert(
        "help.report.xlsx".into(),
        "report xlsx [path] ## ສ້າງບົດລາຍງານ XLSX (ພ້ອມການວິເຄາະ dependency)".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report txt ## ສ້າງບົດລາຍງານ TXT".into(),
    );

    m.insert(
        "help.report.example2".into(),
        "report html --cp ## ສຳເນົາບົດລາຍງານ HTML ໃສ່ clipboard".into(),
    );

    m.insert(
        "help.report.example3".into(),
        "report pdf ## ສ້າງບົດລາຍງານ PDF".into(),
    );

    m.insert(
        "help.report.example4".into(),
        "txtc file.txt ## ສຳເນົາເນື້ອໃນໄຟລ໌ໃສ່ clipboard".into(),
    );

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## ບັນທຶກລາຍງານໂຄງການໄວ້ທີ <file>".into(),
    );

    m.insert("help.report.project_example".into(), "report project html . --name docs/project ## ລາຍງານ HTML ມີຕົ້ນໄມ້ພັບໄດ້ ບັນທຶກເປັນ docs/project.html".into());

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## ສະແດງບັນທຶກປະຫວາດ clipboard ເຕັມ".into(),
    );

    m.insert("help.report.clip_example".into(), "report clip 1".into());
}
