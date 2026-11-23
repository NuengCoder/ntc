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

    m.insert(
        "fmt.help_example10".into(),
        "  fmt data.csv to xlsx -o data.xlsx   # CSV เป็น Excel".into(),
    );
    m.insert(
        "fmt.help_example6".into(),
        "  fmt report.docx to md              # DOCX เป็น Markdown".into(),
    );
    m.insert(
        "fmt.help_example7".into(),
        "  fmt readme.md to html -o out.html".into(),
    );
    m.insert(
        "fmt.help_example8".into(),
        "  fmt notes.txt to pdf -o notes.pdf   # ข้อความเป็น PDF".into(),
    );
    m.insert(
        "fmt.help_example9".into(),
        "  fmt report.txt to docx -o report.docx".into(),
    );
}
