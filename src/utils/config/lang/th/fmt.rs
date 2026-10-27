use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "fmt.help_example1".into(),
        "  fmt package.json to yaml".into(),
    );

    m.insert(
        "fmt.help_example2".into(),
        "  fmt config.json to toml -o config.toml".into(),
    );

    m.insert(
        "fmt.help_example3".into(),
        "  fmt config.yaml to json".into(),
    );

    m.insert(
        "fmt.help_example4".into(),
        "  fmt Cargo.toml to json".into(),
    );

    m.insert(
        "fmt.help_example5".into(),
        "  fmt data.json                      # จัดรูปแบบตามรูปแบบเดิม".into(),
    );

    m.insert("fmt.help_examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "fmt.help_no_target".into(),
        "  fmt <file>                         จัดรูปแบบ/ทำให้เป็นมาตรฐานตามรูปแบบเดิม".into(),
    );

    m.insert("fmt.help_usage".into(), "การใช้งาน:".into());
}
