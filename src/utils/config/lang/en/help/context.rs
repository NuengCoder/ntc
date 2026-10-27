use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.context.cr".into(),
        "<cr> press enter to run it".into(),
    );

    m.insert(
        "help.context.qur.needs_glob".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.select_value".into(),
        "qur -s <pattern>  (content pattern, case-insensitive)".into(),
    );

    m.insert(
        "help.context.qur.replace_value".into(),
        "qur -rp <replacement>  (needs -s \"<pattern>\")".into(),
    );

    m.insert(
        "help.context.qur.depth_value".into(),
        "qur -d <n>  (max depth, 0 = current dir only)".into(),
    );

    m.insert(
        "help.context.qur.limit_value".into(),
        "qur -l <n>  (show only first n matches/files)".into(),
    );

    m.insert(
        "help.context.qur.next_select".into(),
        "qur <glob> -s \"<pattern>\" | -d <n> | -l <n>".into(),
    );

    m.insert(
        "help.context.qur.next_display_replace".into(),
        "qur <glob> -s \"<pattern>\" -dp | -rp \"<new>\" [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.next_depth".into(),
        "qur <glob> -s \"<pattern>\" [-dp | -rp \"<new>\"] [-d <n>] [-l <n>]  (<cr> to run)".into(),
    );
}
