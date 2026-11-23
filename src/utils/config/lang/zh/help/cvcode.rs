use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cvcode.title".into(), "CVCODE 命令：".into());

    m.insert(
        "help.cvcode.usage".into(),
        "cvcode <来源> <数据> to <目标> ## 在格式之间转换".into(),
    );

    m.insert(
        "help.cvcode.example1".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## 在格式间转换".into(),
    );

    m.insert(
        "help.cvcode.example2".into(),
        "cvcode b64 \"aGVsbG8=\" to plaintext ## 解码为可读文本".into(),
    );
}
