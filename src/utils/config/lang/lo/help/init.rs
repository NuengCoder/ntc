use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.init.title".into(), "INIT ຄຳສັ່ງ:".into());

    m.insert(
        "help.init.usage".into(),
        "init [all|ran|local|opencode|project] [-f|--force] ## ສ້າງໄຟລ໌ໂຄງການ (NTCRANFILE, ntconfig.toml, .opencode/, scaffold ໂຄງການ)".into(),
    );

    m.insert(
        "help.init.example1".into(),
        "init ran ## ສ້າງ NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.init.example2".into(),
        "init project my-app --no lc,oc,docker ## scaffold ໂຄງການໂດຍບໍ່ສ້າງ LICENSE, OpenCode, Docker"
            .into(),
    );
}
