use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.igcared.section_global".into(), "[global]".into());

    m.insert("cmd.igcared.section_local".into(), "[local]".into());

    m.insert("cmd.igcared.cared_dirs".into(), "ไดเรกทอรีที่ดูแล:".into());

    m.insert("cmd.igcared.cared_exts".into(), "นามสกุลที่ดูแล:".into());

    m.insert("cmd.igcared.cared_files".into(), "ไฟล์ที่ดูแล:".into());

    m.insert("cmd.igcared.none".into(), "(ไม่มี)".into());

    m.insert(
        "cmd.igcared.no_local".into(),
        "(ไม่มี config local ในไดเรกทอรีนี้)".into(),
    );

    m.insert(
        "cmd.igcared.scope_help".into(),
        "รายการ local มีความสำคัญกว่า รายการ global ยังทำงานต่อ และ config global จะไม่ถูกแก้ไข".into(),
    );

    m.insert("cmd.igcared.extra_exts".into(), "นามสกุลเพิ่มเติม:".into());

    m.insert("cmd.igcared.extra_files".into(), "ไฟล์เพิ่มเติม:".into());

    m.insert(
        "cmd.igcared.ignored_dirs".into(),
        "ไดเรกทอรีที่ถูกละเว้น:".into(),
    );

    m.insert("cmd.igcared.ignored_exts".into(), "นามสกุลที่ถูกละเว้น:".into());

    m.insert("cmd.igcared.ignored_files".into(), "ไฟล์ที่ถูกละเว้น:".into());

    m.insert(
        "cmd.igcared.local_config".into(),
        "   การกำหนดค่าในเครื่อง: {}".into(),
    );
}
