use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.report.title".into(), "ຄຳສັ່ງ REPORT:".into());

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## ບັນທຶກລາຍງານໂຄງການໄວ້ທີ <file>".into(),
    );

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## ສະແດງບັນທຶກປະຫວາດ clipboard ເຕັມ".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report project html . --name docs/project ## ລາຍງານ HTML ມີຕົ້ນໄມ້ພັບໄດ້ ບັນທຶກເປັນ docs/project.html".into(),
    );

    m.insert("help.report.example2".into(), "report clip 1".into());
}
