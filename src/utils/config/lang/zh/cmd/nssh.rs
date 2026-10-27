use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nssh.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.nssh.help_add".into(),
        "nssh add <名称> --host <主机> [--user 用户] [--port 端口] [--key 密钥] [--pass-env 变量]"
            .into(),
    );

    m.insert(
        "cmd.nssh.help_ssh".into(),
        "nssh ssh [名称] [--cmd \"命令\"] [--trust] [--ask-pass] [--timeout 秒]".into(),
    );

    m.insert(
        "cmd.nssh.help_cp".into(),
        "nssh cp <本地> <名称>:<远端> | <名称>:<远端> <本地>".into(),
    );

    m.insert(
        "cmd.nssh.help_scan".into(),
        "nssh scan <主机> [--ports 端口| --top 数量] [--timeout 毫秒]".into(),
    );

    m.insert(
        "cmd.nssh.help_manage".into(),
        "nssh ls | nssh info <名称> | nssh rm <名称> | nssh edit <名称> [选项]".into(),
    );

    m.insert(
        "cmd.nssh.help_keygen".into(),
        "nssh keygen [参数...]   通过 ssh-keygen 生成密钥".into(),
    );

    m.insert(
        "cmd.nssh.help_help".into(),
        "nssh help   显示此帮助（无参数时选择配置）".into(),
    );

    m.insert("cmd.nssh.options_title".into(), "选项：".into());

    m.insert(
        "cmd.nssh.help_host".into(),
        "  --host <主机>           服务器地址或 IP（add 必填）".into(),
    );

    m.insert(
        "cmd.nssh.help_user".into(),
        "  --user, -u <用户>       登录用户（默认当前用户）".into(),
    );

    m.insert(
        "cmd.nssh.help_port".into(),
        "  --port, -p <端口>       SSH 端口（默认 22）".into(),
    );

    m.insert(
        "cmd.nssh.help_key".into(),
        "  --key <路径>            私钥文件（默认 ~/.ssh/id_ed25519、id_rsa）".into(),
    );

    m.insert(
        "cmd.nssh.help_pass_env".into(),
        "  --pass-env <变量>      从环境变量读取密码（永不存储）".into(),
    );

    m.insert(
        "cmd.nssh.help_cmd".into(),
        "  --cmd \"命令\"            运行单条命令而非交互式 shell".into(),
    );

    m.insert(
        "cmd.nssh.help_trust".into(),
        "  --trust               接受并记录未知主机密钥（默认拒绝）".into(),
    );

    m.insert(
        "cmd.nssh.help_ask_pass".into(),
        "  --ask-pass            提示输入密码（隐藏输入）".into(),
    );

    m.insert(
        "cmd.nssh.help_timeout".into(),
        "  --timeout <数字>      连接超时秒数（ssh/cp）或毫秒（scan）".into(),
    );

    m.insert(
        "cmd.nssh.help_top".into(),
        "  --top <数量>            扫描常用端口前 N 个（默认 20）".into(),
    );

    m.insert(
        "cmd.nssh.help_scan_ports".into(),
        "  --ports <规格>        如 22,80,8000-8010（每段最多 2000 个）".into(),
    );

    m.insert(
        "cmd.nssh.help_yes".into(),
        "  --yes, -y             跳过删除确认".into(),
    );

    m.insert("cmd.nssh.examples_title".into(), "示例：".into());

    m.insert(
        "cmd.nssh.example_add".into(),
        "nssh add prod --host 203.0.113.10 --user root --key ~/.ssh/id_rsa".into(),
    );

    m.insert(
        "cmd.nssh.example_ssh".into(),
        "nssh ssh prod --cmd \"uptime\"".into(),
    );

    m.insert(
        "cmd.nssh.example_cp".into(),
        "nssh cp ./app.tar.gz prod:/tmp/".into(),
    );

    m.insert(
        "cmd.nssh.example_scan".into(),
        "nssh scan example.com --top 20".into(),
    );

    m.insert("cmd.nssh.missing_host".into(), "--host 后缺少值".into());

    m.insert("cmd.nssh.missing_user".into(), "--user 后缺少值".into());

    m.insert("cmd.nssh.missing_port".into(), "--port 后缺少值".into());

    m.insert("cmd.nssh.missing_key".into(), "--key 后缺少值".into());

    m.insert(
        "cmd.nssh.missing_pass_env".into(),
        "--pass-env 后缺少值".into(),
    );

    m.insert("cmd.nssh.missing_cmd".into(), "--cmd 后缺少值".into());

    m.insert(
        "cmd.nssh.missing_timeout".into(),
        "--timeout 后缺少值".into(),
    );

    m.insert("cmd.nssh.missing_ports".into(), "--ports 后缺少值".into());

    m.insert("cmd.nssh.missing_top".into(), "--top 后缺少值".into());

    m.insert("cmd.nssh.invalid_port".into(), "无效的端口：'{}'".into());

    m.insert("cmd.nssh.invalid_timeout".into(), "无效的超时：'{}'".into());

    m.insert(
        "cmd.nssh.invalid_top".into(),
        "无效的 --top 值：'{}'".into(),
    );

    m.insert(
        "cmd.nssh.invalid_name".into(),
        "无效的配置名 '{}'（仅限字母数字 _ - .）".into(),
    );

    m.insert("cmd.nssh.unknown_option".into(), "未知：{}".into());

    m.insert("cmd.nssh.add_usage".into(), "用法：nssh add <名称> --host <主机> [--user 用户] [--port 端口] [--key 密钥] [--pass-env 变量]".into());

    m.insert(
        "cmd.nssh.profile_exists".into(),
        "配置 '{}' 已存在（使用 nssh edit 修改）".into(),
    );

    m.insert("cmd.nssh.profile_saved".into(), "已保存配置：{}".into());

    m.insert(
        "cmd.nssh.profile_not_found".into(),
        "未找到配置 '{}'".into(),
    );

    m.insert(
        "cmd.nssh.key_not_found".into(),
        "密钥文件 '{}' 不存在（仍已存储）".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_missing".into(),
        "环境变量 '{}' 未设置（仍已存储）".into(),
    );

    m.insert(
        "cmd.nssh.ls_empty".into(),
        "暂无配置。添加：nssh add <名称> --host <主机>".into(),
    );

    m.insert("cmd.nssh.ls_title".into(), "SSH 配置：".into());

    m.insert("cmd.nssh.info_title".into(), "配置 '{}'：".into());

    m.insert(
        "cmd.nssh.info_usage".into(),
        "用法：nssh info <名称>".into(),
    );

    m.insert("cmd.nssh.info_host".into(), "主机：".into());

    m.insert("cmd.nssh.info_user".into(), "用户：".into());

    m.insert("cmd.nssh.info_port".into(), "端口：".into());

    m.insert("cmd.nssh.info_key".into(), "密钥：".into());

    m.insert("cmd.nssh.info_pass".into(), "密码变量：".into());

    m.insert(
        "cmd.nssh.rm_usage".into(),
        "用法：nssh rm <名称> [--yes]".into(),
    );

    m.insert("cmd.nssh.rm_confirm".into(), "删除配置 '{}'？".into());

    m.insert("cmd.nssh.rm_deleted".into(), "已删除配置：{}".into());

    m.insert("cmd.nssh.rm_cancelled".into(), "已取消。".into());

    m.insert("cmd.nssh.edit_usage".into(), "用法：nssh edit <名称> [--host 主机] [--user 用户] [--port 端口] [--key 密钥|--no-key] [--pass-env 变量|--no-pass-env]".into());

    m.insert("cmd.nssh.edit_saved".into(), "已更新配置：{}".into());

    m.insert("cmd.nssh.pick_profile".into(), "选择 SSH 配置".into());

    m.insert("cmd.nssh.ask_pass".into(), "{}@{} 的密码：".into());

    m.insert(
        "cmd.nssh.pass_env_unset".into(),
        "环境变量 '{}' 为空或未设置 —— 仅在使用 --ask-pass 时提示".into(),
    );

    m.insert(
        "cmd.nssh.ssh_connecting".into(),
        "正在连接 {}@{}:{}……".into(),
    );

    m.insert("cmd.nssh.cmd_exit".into(), "远端命令退出码 {}".into());

    m.insert(
        "cmd.nssh.cp_usage".into(),
        "用法：nssh cp <本地> <名称>:<远端> | nssh cp <名称>:<远端> <本地>".into(),
    );

    m.insert(
        "cmd.nssh.cp_no_remote".into(),
        "必须有一端是 <配置>:<路径>".into(),
    );

    m.insert(
        "cmd.nssh.cp_two_sides".into(),
        "两端都是远端 —— 每次只复制一端".into(),
    );

    m.insert(
        "cmd.nssh.cp_local_missing".into(),
        "未找到本地文件 '{}'".into(),
    );

    m.insert(
        "cmd.nssh.cp_read_fail".into(),
        "无法读取本地文件：{}".into(),
    );

    m.insert(
        "cmd.nssh.cp_too_large".into(),
        "文件超过 256MB 传输限制".into(),
    );

    m.insert(
        "cmd.nssh.cp_uploading".into(),
        "正在上传 {} 到 {}:{}……".into(),
    );

    m.insert(
        "cmd.nssh.cp_downloading".into(),
        "正在下载 {}:{} 到 {}……".into(),
    );

    m.insert("cmd.nssh.cp_done".into(), "完成（{} 字节）".into());

    m.insert(
        "cmd.nssh.scan_usage".into(),
        "用法：nssh scan <主机> [--ports 规格 | --top 数量] [--timeout 毫秒]".into(),
    );

    m.insert(
        "cmd.nssh.scan_start".into(),
        "正在扫描 {}（{} 个端口）……".into(),
    );

    m.insert("cmd.nssh.scan_title".into(), "{} 开放的端口：".into());

    m.insert("cmd.nssh.scan_none".into(), "{} 没有开放端口".into());

    m.insert("cmd.nssh.scan_done".into(), "探测 {} 个，开放 {} 个".into());

    m.insert(
        "cmd.nssh.keygen_missing".into(),
        "未找到 ssh-keygen：{}".into(),
    );

    m.insert("cmd.nssh.keygen_exit".into(), "ssh-keygen 退出码 {}".into());

    m.insert(
        "cmd.nssh.shell_hint".into(),
        "交互式 shell —— 回车 ~ . 强制断开".into(),
    );

    m.insert("cmd.nssh.shell_back".into(), "已回到 ntc shell".into());

    m.insert("cmd.nssh.shell_forced".into(), "已强制断开（~.）".into());
}
