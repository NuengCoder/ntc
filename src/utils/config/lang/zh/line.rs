use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.line.usage".into(),
        "用法：line [-A <前缀>] [-B <分隔符>] [文件]  （为每行编号；没有文件时读取管道输入）".into(),
    );

    m.insert(
        "misc.line.bad_arg".into(),
        "line：意外的参数 '{}'（用法：line [-A <前缀>] [-B <分隔符>] [文件]）".into(),
    );
}
