use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cvcode.formats".into(),
        "格式：b2, b10, b64（base64）, hex, url".into(),
    );

    m.insert("cvcode.missing_data".into(), "缺少要转换的数据".into());

    m.insert("cvcode.missing_to".into(), "'to' 后缺少目标格式".into());

    m.insert(
        "cvcode.usage".into(),
        " cvcode <来源> <数据> to <目标> [-o <文件>]".into(),
    );

    m.insert(
        "cvcode.usage_detailed".into(),
        "用法：cvcode <来源> <数据> to <目标> [-o <文件>]".into(),
    );
}
