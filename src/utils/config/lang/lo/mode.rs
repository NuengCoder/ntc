use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "mode.help_admin".into(),
        "  mode admin               ອອກ ແລະ ເລີ່ມໃໝ່ເປັນ Administrator".into(),
    );

    m.insert(
        "mode.help_example_admin".into(),
        "  mode admin                (UAC prompt ຈະປາກົດ)".into(),
    );

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_examples".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "mode.help_normal".into(),
        "  mode normal              ອອກ ແລະ ເລີ່ມໃໝ່ຕາມປົກກະຕິ".into(),
    );

    m.insert("mode.help_usage".into(), "Usage:".into());
}
