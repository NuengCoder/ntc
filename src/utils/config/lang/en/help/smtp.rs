use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.smtp.title".into(), "SMTP COMMAND:".into());

    m.insert(
        "help.smtp.usage".into(),
        "smtp ## SMTP email client (--id, -i, -u, -pwd, -s, -b, -f, --to)".into(),
    );

    m.insert(
        "help.smtp.example1".into(),
        "smtp --id 1 ## Open SMTP client profile 1".into(),
    );
}
