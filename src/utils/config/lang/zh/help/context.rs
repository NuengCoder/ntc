use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.context.cr".into(), "<cr> 按回车运行".into());

    m.insert(
        "help.context.qur.depth_value".into(),
        "qur -d <n>  （最大深度，0 = 仅当前目录）".into(),
    );

    m.insert(
        "help.context.qur.limit_value".into(),
        "qur -l <n>  （仅显示前 n 个匹配/文件）".into(),
    );

    m.insert(
        "help.context.qur.needs_glob".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.next_depth".into(),
        "qur <glob> -s \"<pattern>\" [-dp | -rp \"<new>\"] [-d <n>] [-l <n>]  （按 <cr> 运行）"
            .into(),
    );

    m.insert(
        "help.context.qur.next_display_replace".into(),
        "qur <glob> -s \"<pattern>\" -dp | -rp \"<new>\" [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.next_select".into(),
        "qur <glob> -s \"<pattern>\" | -d <n> | -l <n>".into(),
    );

    m.insert(
        "help.context.qur.replace_value".into(),
        "qur -rp <replacement>  （需要 -s \"<pattern>\"）".into(),
    );

    m.insert(
        "help.context.qur.select_value".into(),
        "qur -s <pattern>  （内容模式，不区分大小写）".into(),
    );
}
