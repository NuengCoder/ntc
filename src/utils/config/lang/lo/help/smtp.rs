use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.smtp.title".into(), "SMTP ຄຳສັ່ງ:".into());

    m.insert(
        "help.smtp.usage".into(),
        "smtp ## ໂປຣແກຣມສົ່ງອີເມວ SMTP (--id, -i, -u, -pwd, -s, -b, -f, --to)".into(),
    );

    m.insert(
        "help.smtp.example1".into(),
        "smtp --id 1 ## ເປີດໂປຣໄຟລ໌ SMTP 1".into(),
    );
}
