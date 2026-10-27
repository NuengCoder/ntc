use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.arc.create_usage".into(),
        "用法：arc create <out.zip|tar.gz|tgz|tar> <文件...>".into(),
    );

    m.insert(
        "cmd.arc.extract_usage".into(),
        "用法：arc extract <压缩包> [目标目录]".into(),
    );

    m.insert(
        "cmd.arc.list_usage".into(),
        "用法：arc list <压缩包>".into(),
    );

    m.insert(
        "cmd.arc.bad_ext".into(),
        "不支持的压缩包类型（请使用 .zip、.tar.gz/.tgz、.tar）".into(),
    );

    m.insert("cmd.arc.not_found".into(), "未找到：{}".into());

    m.insert("cmd.arc.no_inputs".into(), "没有要打包的输入文件。".into());

    m.insert(
        "cmd.arc.created".into(),
        "已创建压缩包：{}（{} 个条目）".into(),
    );

    m.insert("cmd.arc.extracted".into(), "已解压 {} 个文件到 {}".into());

    m.insert("cmd.arc.list_title".into(), "压缩包：{}".into());

    m.insert("cmd.arc.total".into(), "共计：{} 个条目".into());

    m.insert(
        "cmd.arc.unknown_sub".into(),
        "未知的 arc 子命令：'{}'".into(),
    );

    m.insert("cmd.arc.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.arc.help_create".into(),
        "  arc create <输出> <文件...>  创建压缩包（.zip、.tar.gz、.tar）".into(),
    );

    m.insert(
        "cmd.arc.help_extract".into(),
        "  arc extract <文件> [目录]   解压压缩包".into(),
    );

    m.insert(
        "cmd.arc.help_list".into(),
        "  arc list <文件>              列出压缩包内容".into(),
    );

    m.insert("cmd.arc.examples_title".into(), "示例：".into());
}
