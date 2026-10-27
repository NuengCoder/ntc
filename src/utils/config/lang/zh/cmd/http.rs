use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.http.authorization_label".into(), "授权：".into());

    m.insert("cmd.http.basic_placeholder".into(), "Basic <编码>".into());

    m.insert("cmd.http.bearer_placeholder".into(), "Bearer <令牌>".into());

    m.insert("cmd.http.byte_count".into(), "{} 字节".into());

    m.insert(
        "cmd.http.help_auth".into(),
        "  -a, --auth <用户:密码>       基本认证".into(),
    );

    m.insert(
        "cmd.http.help_bearer".into(),
        "  --bearer <令牌>             Bearer 令牌认证".into(),
    );

    m.insert(
        "cmd.http.help_body".into(),
        "  -d, --data <请求体>             请求体".into(),
    );

    m.insert(
        "cmd.http.help_example1".into(),
        "  http GET https://api.github.com/users/octocat".into(),
    );

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert(
        "cmd.http.help_example3".into(),
        "  http GET https://api.example.com/search -q q=hello -q page=1 -s > results.json".into(),
    );

    m.insert(
        "cmd.http.help_example4".into(),
        "  http GET https://api.example.com/data --bearer mytoken -o response.json".into(),
    );

    m.insert(
        "cmd.http.help_example5".into(),
        "  http GET https://api.example.com/health -s -t 5".into(),
    );

    m.insert(
        "cmd.http.help_example6".into(),
        "  http GET https://api.example.com/users -a admin:secret123".into(),
    );

    m.insert("cmd.http.help_examples_title".into(), "示例：".into());

    m.insert(
        "cmd.http.help_header".into(),
        "  -H, --header <\"键: 值\">    添加请求头（可重复）".into(),
    );

    m.insert(
        "cmd.http.help_help".into(),
        "  -h, --help                     显示此帮助".into(),
    );

    m.insert(
        "cmd.http.help_json".into(),
        "  --json <数据>                  以 JSON 发送（自动设置 Content-Type）".into(),
    );

    m.insert(
        "cmd.http.help_methods".into(),
        "  GET, POST, PUT, DELETE, PATCH, HEAD".into(),
    );

    m.insert("cmd.http.help_methods_title".into(), "方法：".into());

    m.insert(
        "cmd.http.help_no_pretty".into(),
        "  --no-pretty                    禁用 JSON 美化输出".into(),
    );

    m.insert("cmd.http.help_options_title".into(), "选项：".into());

    m.insert(
        "cmd.http.help_output".into(),
        "  -o, --output <文件>            将响应体保存到文件（原始）".into(),
    );

    m.insert(
        "cmd.http.help_query".into(),
        "  -q, --query <键=值>             URL 查询参数（可重复）".into(),
    );

    m.insert(
        "cmd.http.help_silent".into(),
        "  -s, --silent                   只显示响应体（不显示状态/请求头）".into(),
    );

    m.insert(
        "cmd.http.help_timeout".into(),
        "  -t, --timeout <秒>             请求超时（默认 30）".into(),
    );

    m.insert("cmd.http.help_title".into(), "HTTP API 客户端".into());

    m.insert(
        "cmd.http.help_usage_default".into(),
        "  <url>                          （如果第一个参数是 URL 则默认为 GET）".into(),
    );

    m.insert(
        "cmd.http.help_usage_line".into(),
        "  http <方法> <url> [选项]".into(),
    );

    m.insert("cmd.http.help_usage_title".into(), "用法：".into());

    m.insert(
        "cmd.http.help_verbose".into(),
        "  -v, --verbose                 显示传出请求详情".into(),
    );

    m.insert("cmd.http.invalid_body".into(), "无效的请求体：{}".into());

    m.insert(
        "cmd.http.invalid_header".into(),
        "无效的请求头格式，应为 \"键: 值\"，实际为：{}".into(),
    );

    m.insert(
        "cmd.http.invalid_query".into(),
        "无效的查询格式，应为 键=值，实际为：{}".into(),
    );

    m.insert("cmd.http.invalid_request".into(), "无效的请求：{}".into());

    m.insert("cmd.http.invalid_timeout".into(), "无效的超时：{}".into());

    m.insert("cmd.http.missing_auth".into(), "-a 后缺少 用户:密码".into());

    m.insert(
        "cmd.http.missing_bearer".into(),
        "--bearer 后缺少令牌".into(),
    );

    m.insert(
        "cmd.http.missing_body_data".into(),
        "-d 后缺少请求体数据".into(),
    );

    m.insert("cmd.http.missing_filename".into(), "-o 后缺少文件名".into());

    m.insert("cmd.http.missing_header".into(), "-H 后缺少请求头值".into());

    m.insert(
        "cmd.http.missing_json".into(),
        "--json 后缺少 JSON 数据".into(),
    );

    m.insert("cmd.http.missing_query".into(), "-q 后缺少 键=值".into());

    m.insert("cmd.http.missing_timeout".into(), "-t 后缺少秒数".into());

    m.insert("cmd.http.request_failed".into(), "{} {} 失败".into());

    m.insert(
        "cmd.http.saved_bytes".into(),
        "已将 {} 字节保存到 {}".into(),
    );

    m.insert("cmd.http.status_line".into(), "{} {} {}  ".into());

    m.insert("cmd.http.unknown_status".into(), "未知".into());

    m.insert(
        "cmd.http.url_required".into(),
        "需要 URL。用法：http <方法> <url>".into(),
    );

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.write_fail".into(), "写入 {} 失败".into());
}
