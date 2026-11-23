use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.http.title".into(), "HTTP 命令：".into());

    m.insert(
        "help.http.usage".into(),
        "http <方法> <url> [选项] ## 发送请求（默认方法为 GET）".into(),
    );

    m.insert(
        "help.http.usage2".into(),
        "http GET|POST|PUT|PATCH|DELETE|HEAD|QUERY <url> ## 支持的方法".into(),
    );

    m.insert(
        "help.http.note".into(),
        "选项：-H \"Key: Value\" 请求头，-d 文本请求体，--json JSON 请求体（自动设置 Content-Type），-q key=value 查询参数，-a user:pass Basic 认证，--bearer token，-o 文件 保存响应体，-s 只输出响应体，-v 显示请求，-t 秒 超时（默认 30）。JSON 响应会自动格式化，除非使用 --no-pretty。".into(),
    );

    m.insert(
        "help.http.example1".into(),
        "http GET https://api.example.com/users ## 简单 GET 请求".into(),
    );

    m.insert(
        "help.http.example2".into(),
        "http POST https://api.example.com/users --json '{\"name\":\"ada\"}' ## 发送 JSON 请求体".into(),
    );

    m.insert(
        "help.http.example3".into(),
        "http GET https://api.github.com/repos/rust-lang/rust -H \"Accept: application/vnd.github+json\" -q per_page=1 ## 请求头 + 查询参数".into(),
    );

    m.insert(
        "help.http.example4".into(),
        "http GET https://api.example.com/me -a user:pass -o me.json ## Basic 认证并保存响应体".into(),
    );

    m.insert(
        "help.http.example5".into(),
        "http GET https://api.example.com/users -s | qur -s name ## 将原始响应体传给其他命令".into(),
    );
}
