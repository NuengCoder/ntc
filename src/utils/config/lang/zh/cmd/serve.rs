use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.serve.db_title".into(), "数据库：".into());

    m.insert(
        "cmd.serve.example_default".into(),
        "  serve files           HTTP 运行于 8080 端口".into(),
    );

    m.insert(
        "cmd.serve.example_port".into(),
        "  serve files 3000       HTTP 运行于 3000 端口".into(),
    );

    m.insert(
        "cmd.serve.example_spa".into(),
        "  serve files --spa      SPA 模式（React Router 回退）".into(),
    );

    m.insert(
        "cmd.serve.example_tunnel".into(),
        "  serve files --tunnel   HTTP + cloudflared 隧道".into(),
    );

    m.insert("cmd.serve.examples_title".into(), "示例：".into());

    m.insert(
        "cmd.serve.files_stored_title".into(),
        "文件存储位置：".into(),
    );

    m.insert(
        "cmd.serve.help_bind".into(),
        "  --bind, -b <地址>    绑定地址（默认：0.0.0.0）".into(),
    );

    m.insert(
        "cmd.serve.help_help".into(),
        "  --help, -h           显示此帮助".into(),
    );

    m.insert(
        "cmd.serve.help_port".into(),
        "  --port, -p <端口>    监听端口（默认：8080）".into(),
    );

    m.insert(
        "cmd.serve.help_spa".into(),
        "  --spa                SPA 模式（回退到 index.html）".into(),
    );

    m.insert("cmd.serve.help_title".into(), "HTTP 文件服务器".into());

    m.insert(
        "cmd.serve.help_tunnel".into(),
        "  --tunnel, -t         通过 cloudflared 隧道暴露".into(),
    );

    m.insert(
        "cmd.serve.help_share".into(),
        "  --share [路径]        检查网络并显示局域网 URL + 二维码".into(),
    );

    m.insert(
        "cmd.serve.help_qr".into(),
        "  --qr                  强制显示二维码（含 --share 效果）".into(),
    );

    m.insert(
        "cmd.serve.help_no_qr".into(),
        "  --no-qr               不显示二维码".into(),
    );

    m.insert(
        "cmd.serve.example_share".into(),
        "  serve files 8080 --share  检查网络并打印局域网 URL + 二维码".into(),
    );

    m.insert(
        "cmd.serve.share_checking".into(),
        "正在检查网络连接……".into(),
    );

    m.insert(
        "cmd.serve.share_online".into(),
        "网络正常 —— 分享模式".into(),
    );

    m.insert(
        "cmd.serve.share_offline".into(),
        "无网络连接 —— 仅显示本地地址。".into(),
    );

    m.insert("cmd.serve.share_lan_title".into(), "局域网：".into());

    m.insert("cmd.serve.share_path_title".into(), "分享：".into());

    m.insert(
        "cmd.serve.share_no_lan".into(),
        "未找到局域网地址 —— 仅显示本地地址。".into(),
    );

    m.insert(
        "cmd.serve.share_bound_loopback".into(),
        "已绑定到回环地址：局域网设备无法访问（请使用默认 0.0.0.0）。".into(),
    );

    m.insert(
        "cmd.serve.share_not_private".into(),
        "检测到的地址 {} 不是私有局域网段。".into(),
    );

    m.insert(
        "cmd.serve.share_not_private_hint".into(),
        "手机分享需要 10.x.x.x、172.16-31.x.x 或 192.168.x.x。".into(),
    );

    m.insert(
        "cmd.serve.share_qr_fail".into(),
        "二维码渲染失败：{}".into(),
    );

    m.insert(
        "cmd.serve.help_usage".into(),
        "serve files [端口] [选项]".into(),
    );

    m.insert(
        "cmd.serve.help_web_dir".into(),
        "  --web-dir, -w <目录> 使用自定义前端目录".into(),
    );

    m.insert(
        "cmd.serve.help_auth".into(),
        "  --auth <用户名:密码>  访问 /api/* 需要登录（例如 --auth admin:secret）".into(),
    );

    m.insert(
        "cmd.serve.example_auth".into(),
        "  serve files 8080 --auth admin:secret   8080 端口 HTTP，API 需要登录".into(),
    );

    m.insert(
        "cmd.serve.invalid_auth".into(),
        "无效的 --auth（用法：--auth <用户名:密码>）".into(),
    );

    m.insert("cmd.serve.missing_auth".into(), "缺少 --auth 的值".into());

    m.insert(
        "cmd.serve.auth_on".into(),
        "已为用户 '{}' 启用 API 登录验证".into(),
    );

    m.insert(
        "cmd.serve.help_lock".into(),
        "  --lock                只读：允许下载/列出，禁止上传/编辑/创建/删除".into(),
    );

    m.insert(
        "cmd.serve.lock_on".into(),
        "只读模式 (--lock)：已阻止上传、编辑、创建和删除".into(),
    );

    m.insert("cmd.serve.invalid_port".into(), "无效的端口".into());

    m.insert("cmd.serve.missing_bind".into(), "缺少绑定地址".into());

    m.insert("cmd.serve.missing_dir".into(), "缺少目录".into());

    m.insert("cmd.serve.missing_port".into(), "缺少端口".into());

    m.insert("cmd.serve.options_title".into(), "选项：".into());

    m.insert("cmd.serve.stopped".into(), "服务器已停止。".into());

    m.insert("cmd.serve.unknown_option".into(), "未知：{}".into());

    m.insert("cmd.serve.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.serve.cloudflared_starting".into(),
        "cloudflared 隧道正在启动（PID {}）".into(),
    );

    m.insert(
        "cmd.serve.cloudflared_missing".into(),
        "未找到 cloudflared：{}。安装：{}".into(),
    );

    m.insert(
        "cmd.serve.chess_soon".into(),
        "`serve chess` 即将推出（在线多人）".into(),
    );

    m.insert(
        "cmd.serve.services_title".into(),
        "Serve — 请选择服务".into(),
    );

    m.insert(
        "cmd.serve.services_usage".into(),
        "serve <服务> [选项]（裸 `serve` 不运行任何服务）".into(),
    );

    m.insert("cmd.serve.services_header".into(), "服务：".into());

    m.insert(
        "cmd.serve.services_row_files".into(),
        "  files         文件网盘 Web 界面（浏览器上传/下载）".into(),
    );

    m.insert(
        "cmd.serve.services_row_http".into(),
        "  http-server   当前目录的静态服务器".into(),
    );

    m.insert(
        "cmd.serve.services_row_chess".into(),
        "  chess         双人热座象棋（浏览器对弈）".into(),
    );

    m.insert(
        "cmd.serve.services_row_config".into(),
        "  config        查看/设置各服务默认值（端口、绑定）".into(),
    );

    m.insert(
        "cmd.serve.services_example_files".into(),
        "  serve files --port 9000".into(),
    );

    m.insert(
        "cmd.serve.services_example_http".into(),
        "  serve http-server ./dist".into(),
    );

    m.insert(
        "cmd.serve.services_example_config".into(),
        "  serve config files --port-default 9000".into(),
    );

    m.insert(
        "cmd.serve.http_title".into(),
        "HTTP 静态服务器（当前目录）".into(),
    );

    m.insert(
        "cmd.serve.http_usage".into(),
        "serve http-server [目录] [--port 8000] [选项]".into(),
    );

    m.insert(
        "cmd.serve.example_http_default".into(),
        "  serve http-server            在 8080 上托管当前目录".into(),
    );

    m.insert(
        "cmd.serve.example_http_dir".into(),
        "  serve http-server ./dist --port 8001   在 8001 上托管 ./dist".into(),
    );

    m.insert(
        "cmd.serve.example_http_auth".into(),
        "  serve http-server --auth admin:secret  需要登录".into(),
    );

    m.insert(
        "cmd.serve.help_auth_http".into(),
        "  --auth <user:pass>     所有路由都需要登录（例如 --auth admin:secret）".into(),
    );

    m.insert("cmd.serve.http_serving".into(), "正在托管".into());

    m.insert("cmd.serve.http_no_dir".into(), "目录 '{}' 不存在".into());

    m.insert(
        "cmd.serve.current_defaults".into(),
        "已配置默认值（serve config）：端口 {}，绑定 {}".into(),
    );

    m.insert(
        "cmd.serve.chess_no_auth".into(),
        "`serve chess` 不支持 --auth（热座模式无需登录）".into(),
    );

    m.insert("cmd.serve.chess_serving".into(), "象棋服务器运行于".into());

    m.insert("cmd.serve.chess_title".into(), "象棋（双人热座）".into());

    m.insert(
        "cmd.serve.chess_usage".into(),
        "serve chess [--port 8000] [选项]".into(),
    );

    m.insert(
        "cmd.serve.example_chess_default".into(),
        "  serve chess              在 8080 上运行象棋".into(),
    );

    m.insert(
        "cmd.serve.example_chess_port".into(),
        "  serve chess --port 9000  在 9000 上运行象棋".into(),
    );
}
