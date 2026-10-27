use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.care.dir_desc".into(),
        "care dir <名称>[, ...] 停止忽略目录".into(),
    );

    m.insert(
        "cmd.care.dir_usage".into(),
        "用法：care dir <名称>[, <名称>, ...]".into(),
    );

    m.insert(
        "cmd.care.ext_desc".into(),
        "care ext <扩展名>[, ...] 添加额外支持的扩展名".into(),
    );

    m.insert(
        "cmd.care.ext_usage".into(),
        "用法：care ext <扩展名>[, <扩展名>, ...]".into(),
    );

    m.insert(
        "cmd.care.file_desc".into(),
        "care file <名称>[, ...] 添加额外支持的文件".into(),
    );

    m.insert(
        "cmd.care.file_usage".into(),
        "用法：care file <名称>[, <名称>, ...]".into(),
    );

    m.insert(
        "cmd.care.title".into(),
        "care \u{2014} 管理关注（取消忽略）的项目".into(),
    );

    m.insert(
        "cmd.care.scope_help".into(),
        "使用 '-g' / '--global' 写入全局配置（仅在含 ntconfig.toml 的目录中）。".into(),
    );
}
