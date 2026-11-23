use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmf.title".into(), "RMF ຄຳສັ່ງ:".into());

    m.insert(
        "help.rmf.usage".into(),
        "rmf <file>[, ...] ## ລຶບໄຟລ໌ (ພ້ອມຢືນຢັນ)".into(),
    );

    m.insert("help.rmf.example1".into(), "rmf old.log ## ລຶບໄຟລ໌".into());
}
