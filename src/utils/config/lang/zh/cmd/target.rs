use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.target.not_dir".into(), "“{}”不是目录".into());

    m.insert("cmd.target.not_found".into(), "路径“{}”不存在".into());

    m.insert("cmd.target.tp_not_found".into(), "传送点“{}”不存在".into());
}
