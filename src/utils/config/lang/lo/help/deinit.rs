use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.deinit.title".into(), "DEINIT ຄຳສັ່ງ:".into());

    m.insert(
        "help.deinit.usage".into(),
        "deinit [ran|local|opencode] ## ລຶບໄຟລ໌ໂຄງການ (ຖາມຢືນຢັນກ່ອນລຶບທຸກຄັ້ງ)".into(),
    );

    m.insert(
        "help.deinit.example1".into(),
        "deinit ran ## ລຶບ NTCRANFILE.toml".into(),
    );
}
