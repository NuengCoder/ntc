use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.care.title".into(), "CARE 命令：".into());

    m.insert(
        "help.care.care_dir".into(),
        "care dir <名称>[, ...] ## 停止忽略目录".into(),
    );

    m.insert(
        "help.care.care_ext".into(),
        "care ext <扩展名>[, ...] ## 添加额外支持的扩展名".into(),
    );

    m.insert(
        "help.care.care_file".into(),
        "care file <名称>[, ...] ## 添加额外支持的文件".into(),
    );

    m.insert(
        "help.care.example1".into(),
        "care dir src ## 取消忽略 src 目录".into(),
    );
}
