use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.smtp.build_fail".into(), "构建电子邮件消息失败".into());

    m.insert(
        "cmd.smtp.create_one_hint".into(),
        "  创建：{} -i <服务器> -p <端口> -u <用户> -pwd <密码>".into(),
    );

    m.insert(
        "cmd.smtp.create_syntax".into(),
        "smtp -i <服务器> -p <端口> -u <用户> -pwd <密码>".into(),
    );

    m.insert(
        "cmd.smtp.create_usage".into(),
        "需要服务器、用户名和密码。用法：smtp -i <服务器> -p <端口> -u <用户> -pwd <密码>".into(),
    );

    m.insert("cmd.smtp.created_label".into(), "已创建：".into());

    m.insert("cmd.smtp.decode_failed".into(), "解码存储的密码失败".into());

    m.insert(
        "cmd.smtp.body_file_missing".into(),
        "找不到正文文件：{}".into(),
    );

    m.insert(
        "cmd.smtp.body_html_from_file".into(),
        "HTML 正文来自文件：{}".into(),
    );

    m.insert(
        "cmd.smtp.body_md_from_file".into(),
        "Markdown 正文来自文件：{}（已渲染为 HTML）".into(),
    );

    m.insert(
        "cmd.smtp.body_read_fail".into(),
        "读取正文文件 {} 失败：{}".into(),
    );

    m.insert(
        "cmd.smtp.body_stdin_empty".into(),
        "未从标准输入收到正文".into(),
    );

    m.insert(
        "cmd.smtp.body_text_from_file".into(),
        "纯文本正文来自文件：{}".into(),
    );

    m.insert(
        "cmd.smtp.help_body_file".into(),
        "  -b letter.html | letter.md       从文件读取正文（Markdown 会渲染为 HTML）".into(),
    );

    m.insert(
        "cmd.smtp.help_body_format".into(),
        "  --body-format auto|text|html|md  强制指定 -b 的解析方式".into(),
    );

    m.insert(
        "cmd.smtp.help_body_inline".into(),
        "  -b '<p style=text-align:center>你好</p>'   自动识别内联 HTML".into(),
    );

    m.insert(
        "cmd.smtp.help_body_stdin".into(),
        "  -b -                            从标准输入读取正文".into(),
    );

    m.insert("cmd.smtp.help_body_title".into(), "邮件正文：".into());

    m.insert(
        "cmd.smtp.invalid_body_format".into(),
        "未知的正文格式 '{}'（可用 auto、text、html、md）".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_file".into(),
        "--body-file 后缺少路径".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_format".into(),
        "--body-format 后缺少值".into(),
    );

    m.insert(
        "cmd.smtp.help_attachments_desc".into(),
        "  -f 接受逗号分隔的文件路径和目录（目录中的文件会递归展开）".into(),
    );

    m.insert("cmd.smtp.help_attachments_example".into(), "  示例：smtp --id a1b2 --to user@test.com -s \"报告\" -b \"见附件\" -f \"./report.pdf,./images/\"".into());

    m.insert("cmd.smtp.help_attachments_title".into(), "附件：".into());

    m.insert(
        "cmd.smtp.help_create_example".into(),
        "  示例：smtp -i smtp.gmail.com -p 587 -u me@gmail.com -pwd app123".into(),
    );

    m.insert(
        "cmd.smtp.help_create_syntax".into(),
        "  smtp -i <服务器> -p <端口> -u <用户> -pwd <密码>".into(),
    );

    m.insert("cmd.smtp.help_create_title".into(), "创建配置：".into());

    m.insert(
        "cmd.smtp.help_info_syntax".into(),
        "  smtp --id-info <id>".into(),
    );

    m.insert("cmd.smtp.help_info_title".into(), "显示配置信息：".into());

    m.insert("cmd.smtp.help_list_syntax".into(), "  smtp --id-ls".into());

    m.insert("cmd.smtp.help_list_title".into(), "列出配置：".into());

    m.insert(
        "cmd.smtp.help_port_25".into(),
        "   25   明文（若服务器支持）".into(),
    );

    m.insert(
        "cmd.smtp.help_port_465".into(),
        "   465  TLS（SMTPS）".into(),
    );

    m.insert(
        "cmd.smtp.help_port_587".into(),
        "   587  STARTTLS（默认）".into(),
    );

    m.insert("cmd.smtp.help_ports_title".into(), "端口：".into());

    m.insert(
        "cmd.smtp.help_rm_syntax".into(),
        "  smtp --id-rm <id>".into(),
    );

    m.insert("cmd.smtp.help_rm_title".into(), "删除配置：".into());

    m.insert(
        "cmd.smtp.help_send_syntax".into(),
        "  smtp --id <id> --to <邮箱> [--cc <邮箱>] [--bcc <邮箱>] -s <主题> -b <正文> [-f <文件>]"
            .into(),
    );

    m.insert("cmd.smtp.help_send_title".into(), "发送邮件：".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP 邮件客户端".into());

    m.insert("cmd.smtp.id_label".into(), "ID：".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "无效的 BCC：{}".into());

    m.insert("cmd.smtp.invalid_cc".into(), "无效的 CC：{}".into());

    m.insert("cmd.smtp.invalid_port".into(), "无效的端口号".into());

    m.insert(
        "cmd.smtp.invalid_pwd_encoding".into(),
        "无效的密码编码：{}".into(),
    );

    m.insert(
        "cmd.smtp.invalid_recipient".into(),
        "无效的收件人：{}".into(),
    );

    m.insert(
        "cmd.smtp.invalid_sender".into(),
        "无效的发件人邮箱地址".into(),
    );

    m.insert("cmd.smtp.missing_body".into(), "-b 后缺少正文".into());

    m.insert("cmd.smtp.missing_id".into(), "--id 后缺少 ID".into());

    m.insert(
        "cmd.smtp.missing_id_info".into(),
        "--id-info 后缺少 ID".into(),
    );

    m.insert("cmd.smtp.missing_id_rm".into(), "--id-rm 后缺少 ID".into());

    m.insert("cmd.smtp.missing_password".into(), "-pwd 后缺少密码".into());

    m.insert("cmd.smtp.missing_path".into(), "-f 后缺少路径".into());

    m.insert("cmd.smtp.missing_port".into(), "-p 后缺少端口".into());

    m.insert(
        "cmd.smtp.missing_recipient_bcc".into(),
        "--bcc 后缺少收件人".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_cc".into(),
        "--cc 后缺少收件人".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_to".into(),
        "--to 后缺少收件人".into(),
    );

    m.insert("cmd.smtp.missing_server".into(), "-i 后缺少服务器".into());

    m.insert("cmd.smtp.missing_subject".into(), "-s 后缺少主题".into());

    m.insert("cmd.smtp.missing_user".into(), "-u 后缺少用户名".into());

    m.insert(
        "cmd.smtp.need_recipient".into(),
        "需要收件人。使用 --to <邮箱>".into(),
    );

    m.insert(
        "cmd.smtp.need_subject".into(),
        "需要主题。使用 -s <主题>".into(),
    );

    m.insert(
        "cmd.smtp.no_files_in_dir".into(),
        "目录中未找到文件：{}".into(),
    );

    m.insert(
        "cmd.smtp.no_profiles".into(),
        "  {}  未找到 SMTP 配置。".into(),
    );

    m.insert("cmd.smtp.password_label".into(), "密码：".into());

    m.insert("cmd.smtp.path_not_found".into(), "路径未找到：{}".into());

    m.insert("cmd.smtp.port_label".into(), "端口：".into());

    m.insert("cmd.smtp.profile_label".into(), "SMTP 配置：".into());

    m.insert("cmd.smtp.profile_not_found".into(), "未找到配置：{}".into());

    m.insert(
        "cmd.smtp.profile_saved".into(),
        "SMTP 配置已保存：{}".into(),
    );

    m.insert("cmd.smtp.profiles_title".into(), "SMTP 配置：".into());

    m.insert(
        "cmd.smtp.read_attach_fail".into(),
        "读取附件失败：{}".into(),
    );

    m.insert(
        "cmd.smtp.removed_profile".into(),
        "已移除 SMTP 配置：{}".into(),
    );

    m.insert("cmd.smtp.send_fail".into(), "发送邮件失败：{}".into());

    m.insert("cmd.smtp.send_ok".into(), "邮件发送成功{}".into());

    m.insert("cmd.smtp.sending".into(), "正在向 {} 发送邮件...".into());

    m.insert("cmd.smtp.server_label".into(), "服务器：".into());

    m.insert("cmd.smtp.skipping_exec".into(), "跳过可执行文件：{}".into());

    m.insert(
        "cmd.smtp.skipping_not_file".into(),
        "已跳过（不是文件或目录）：{}".into(),
    );

    m.insert(
        "cmd.smtp.transport_fail".into(),
        "配置 SMTP 传输失败：{}".into(),
    );

    m.insert("cmd.smtp.unknown_option".into(), "未知选项：{}".into());

    m.insert("cmd.smtp.user_label".into(), "用户：".into());

    m.insert(
        "cmd.smtp.with_attachments".into(),
        " （{} 个附件{}）".into(),
    );
}
