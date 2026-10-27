use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.views.usage".into(),
        "ວິທີໃຊ້: views [path] [-n] [--size [ຂະໜາດຂັ້ນຕ່ຳ]] [--sort <name|ext|size>]".into(),
    );

    m.insert(
        "cmd.views.invalid_size".into(),
        "ຂະໜາດບໍ່ຖືກຕ້ອງ: {} (ໃຊ້ 1024, 10KB, 5MB, ...)".into(),
    );

    m.insert("cmd.views.invalid_sort".into(), "ບໍ່ມີຄ່າສຳລັບ --sort".into());

    m.insert(
        "cmd.views.invalid_sort_value".into(),
        "ບໍ່ຮູ້ຈັກຄີການຈັດລຳດັບ: {} (ໃຊ້ name, ext ຫຼື size)".into(),
    );

    m.insert("cmd.views.empty".into(), "(directory ຫວ່າງເປົ່າ)".into());
}
