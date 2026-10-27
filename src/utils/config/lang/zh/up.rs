use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("up.help.examples".into(), "示例：".into());

    m.insert(
        "up.help.opt_data".into(),
        " -d, --data <主体> 上传内联数据".into(),
    );

    m.insert(
        "up.help.opt_header".into(),
        " -H, --header <\"Key: Value\"> 添加请求标头（可重复）".into(),
    );

    m.insert("up.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert(
        "up.help.opt_post".into(),
        " --post 使用 POST 代替 PUT".into(),
    );

    m.insert("up.help.options".into(), "选项：".into());

    m.insert("up.help.title".into(), "上传（up）".into());

    m.insert("up.help.usage".into(), "用法：".into());

    m.insert(
        "up.help.usage_data".into(),
        " up -d \"数据\" <url> 上传内联文本数据".into(),
    );

    m.insert(
        "up.help.usage_line".into(),
        " up <文件> <url> 通过 PUT 上传文件".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        " up <文件> <url> --post 通过 POST 上传".into(),
    );
}
