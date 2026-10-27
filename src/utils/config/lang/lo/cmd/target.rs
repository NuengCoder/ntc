use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.target.not_dir".into(), "'{}' ບໍ່ແມ່ນໄດເຣັກທໍຣີ".into());

    m.insert("cmd.target.not_found".into(), "ບໍ່ພົບເສັ້ນທາງ '{}'".into());

    m.insert("cmd.target.tp_not_found".into(), "ບໍ່ພົບຈຸດບັນທຶກ '{}'".into());
}
