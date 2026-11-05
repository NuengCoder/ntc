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
        " --post 使用 POST（上传的默认方式）".into(),
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
        " up <文件> <url> 以 multipart POST 上传文件".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        " up <文件> <url> --put --raw 以 PUT 发送原始字节".into(),
    );

    m.insert(
        "up.missing_auth".into(),
        "-a 之后缺少凭据（格式：user:pass）".into(),
    );

    m.insert("up.missing_bearer".into(), "--bearer 之后缺少令牌".into());

    m.insert(
        "up.missing_form".into(),
        "-F 之后缺少字段（格式：name=value）".into(),
    );

    m.insert(
        "up.invalid_form".into(),
        "无效的表单字段：{}（格式：name=value）".into(),
    );

    m.insert("up.missing_timeout".into(), "--timeout 之后缺少秒数".into());

    m.insert("up.invalid_timeout".into(), "无效的超时时间：{}".into());

    m.insert("up.conflict".into(), "文件与 -d 数据只能二选一".into());

    m.insert(
        "up.fallback_retry".into(),
        "服务器返回 {}，改用 {} 重试".into(),
    );

    m.insert(
        "up.help.opt_auth".into(),
        " -a, --auth <user:pass> Basic 认证凭据".into(),
    );

    m.insert(
        "up.help.opt_bearer".into(),
        " --bearer <token> Bearer 令牌".into(),
    );

    m.insert(
        "up.help.opt_form".into(),
        " -F, --form <name=value> 附加 multipart 字段（可重复）".into(),
    );

    m.insert(
        "up.help.opt_raw".into(),
        " --raw 直接发送文件字节作为主体".into(),
    );

    m.insert(
        "up.help.opt_put".into(),
        " --put 使用原始 PUT 而非 multipart POST".into(),
    );

    m.insert(
        "up.help.opt_timeout".into(),
        " --timeout <秒> 请求超时（默认 300）".into(),
    );

    m.insert(
        "up.help.opt_no_fallback".into(),
        " --no-fallback 不切换到另一种传输方式重试".into(),
    );
}
