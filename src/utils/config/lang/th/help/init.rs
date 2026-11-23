use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.init.title".into(), "INIT คำสั่ง:".into());

    m.insert(
        "help.init.usage".into(),
        "init [all|ran|local|opencode|project] [-f|--force] ## เริ่มต้นไฟล์โปรเจกต์ (NTCRANFILE, ntconfig.toml, .opencode/, scaffold โปรเจกต์)".into(),
    );

    m.insert(
        "help.init.example1".into(),
        "init ran ## เริ่มต้น NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.init.example2".into(),
        "init project my-app --no lc,oc,docker ## scaffold โปรเจกต์โดยไม่สร้าง LICENSE, OpenCode, Docker".into(),
    );
}
