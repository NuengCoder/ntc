use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.run.title".into(), "RUN คำสั่ง:".into());

    m.insert(
        "help.run.usage".into(),
        "run <alias> ## เรียก alias ด้วย 'run'".into(),
    );

    m.insert(
        "help.run.direct".into(),
        "<alias> ## เรียก alias ตรง ๆ".into(),
    );

    m.insert(
        "help.run.param".into(),
        "<alias>(<arg>) ## เรียก alias แบบมีพารามิเตอร์".into(),
    );

    m.insert(
        "help.run.example1".into(),
        "build ## เรียก alias ตรง ๆ".into(),
    );
}
