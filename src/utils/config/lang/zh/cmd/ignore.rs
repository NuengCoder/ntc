use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.ignore.applied".into(), "{} {} 个项目".into());

    m.insert(
        "cmd.ignore.care_example_dir".into(),
        "  care dir build,dist".into(),
    );

    m.insert(
        "cmd.ignore.care_example_ext".into(),
        "  care ext txt,md,json".into(),
    );

    m.insert(
        "cmd.ignore.care_example_file".into(),
        "  care file .env".into(),
    );

    m.insert("cmd.ignore.care_examples_title".into(), "关注示例：".into());

    m.insert(
        "cmd.ignore.check_hint".into(),
        "运行 'igcared' 查看当前忽略/关注设置".into(),
    );

    m.insert(
        "cmd.ignore.dir_desc".into(),
        "ignore dir <名称>[, ...] 忽略目录".into(),
    );

    m.insert(
        "cmd.ignore.dir_usage".into(),
        "用法：ignore dir <名称>[, <名称>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.example_dir".into(),
        "  ignore dir temp,logs".into(),
    );

    m.insert(
        "cmd.ignore.example_ext".into(),
        "  ignore ext log,tmp,bak".into(),
    );

    m.insert(
        "cmd.ignore.example_file".into(),
        "  ignore file Cargo.lock".into(),
    );

    m.insert("cmd.ignore.examples_title".into(), "示例：".into());

    m.insert(
        "cmd.ignore.ext_desc".into(),
        "ignore ext <扩展名>[, ...] 忽略文件扩展名".into(),
    );

    m.insert(
        "cmd.ignore.ext_usage".into(),
        "用法：ignore ext <扩展名>[, <扩展名>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.file_desc".into(),
        "ignore file <名称>[, ...] 忽略特定文件".into(),
    );

    m.insert(
        "cmd.ignore.file_usage".into(),
        "用法：ignore file <名称>[, <名称>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.title".into(),
        "ignore \u{2014} 管理已忽略的项目".into(),
    );

    m.insert("cmd.ignore.scope_local".into(), "本地配置".into());

    m.insert("cmd.ignore.scope_global".into(), "全局配置".into());

    m.insert(
        "cmd.ignore.scope_help".into(),
        "使用 '-g' / '--global' 写入全局配置（仅在含 ntconfig.toml 的目录中）。".into(),
    );

    m.insert("cmd.ignore.scope_target".into(), "目标：{}".into());
}
