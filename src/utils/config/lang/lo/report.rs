use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "report.generating_html".into(),
        "\u{1F4CA} ກຳລັງສ້າງບົດລາຍງານ HTML ສຳລັບ: {}".into(),
    );

    m.insert(
        "report.generating_json".into(),
        "\u{1F4CA} ກຳລັງສ້າງບົດລາຍງານ JSON ສຳລັບ: {}".into(),
    );

    m.insert(
        "report.generating_md".into(),
        "\u{1F4CA} ກຳລັງສ້າງບົດລາຍງານ Markdown ສຳລັບ: {}".into(),
    );

    m.insert(
        "report.saved_html".into(),
        "\u{2705} ບັນທຶກບົດລາຍງານ HTML ໄປທີ່: {}".into(),
    );

    m.insert(
        "report.saved_json".into(),
        "\u{2705} ບັນທຶກບົດລາຍງານ JSON ໄປທີ່: {}".into(),
    );

    m.insert(
        "report.saved_md".into(),
        "\u{2705} ບັນທຶກບົດລາຍງານ Markdown ໄປທີ່: {}".into(),
    );

    m.insert("report.saved_to".into(), "ບັນທຶກບົດລາຍງານໄປທີ່: {}".into());
}
