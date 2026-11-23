use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.smtp.title".into(), "SMTP 命令：".into());

    m.insert(
        "help.smtp.usage".into(),
        "smtp ## SMTP邮件客户端（--id, -i, -u, -pwd, -s, -b, -f, --to）".into(),
    );

    m.insert(
        "help.smtp.example1".into(),
        "smtp --id 1 ## 打开 SMTP 客户端配置 1".into(),
    );
}
