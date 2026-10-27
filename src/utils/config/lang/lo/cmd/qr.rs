use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qr.decode_fail".into(),
        "ຖອດລະຫັດ QR #{} ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.qr.decode_requires".into(),
        "ການຖອດລະຫັດ QR ຕ້ອງການ feature 'image2doc' (ເປີດໂດຍຄ່າເລີ່ມຕົ້ນ). ແລ່ນດ້ວຍ --features image2doc."
            .into(),
    );

    m.insert(
        "cmd.qr.decode_usage".into(),
        "ວິທີໃຊ້: qr decode <image_file>".into(),
    );

    m.insert(
        "cmd.qr.encode_usage".into(),
        "ວິທີໃຊ້: qr encode <text|image> [-o <file>]".into(),
    );

    m.insert("cmd.qr.example1".into(), "  qr \"Hello, World!\"".into());

    m.insert(
        "cmd.qr.example2".into(),
        "  qr encode \"hello world\" -o qr.png".into(),
    );

    m.insert("cmd.qr.example3".into(), "  qr decode qr.png".into());

    m.insert(
        "cmd.qr.example_image".into(),
        "  qr encode image.png -o imageqrcode.png".into(),
    );

    m.insert("cmd.qr.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert("cmd.qr.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: {}".into());

    m.insert("cmd.qr.gen_fail".into(), "ສ້າງ QR code ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "cmd.qr.help_decode".into(),
        "  qr decode <image_file>           ຖອດລະຫັດ QR code ຈາກຮູບພາບ".into(),
    );

    m.insert(
        "cmd.qr.help_encode".into(),
        "  qr <text>                        ສ້າງ ແລະ ສະແດງ QR code ໃນ terminal".into(),
    );

    m.insert(
        "cmd.qr.help_same_as_above".into(),
        "  qr encode <text>                 ຄືກັບຂ້າງເທິງ".into(),
    );

    m.insert(
        "cmd.qr.help_save_png".into(),
        "  qr encode <text> -o <file>       ບັນທຶກ QR ເປັນຮູບ PNG".into(),
    );

    m.insert(
        "cmd.qr.help_encode_image".into(),
        "  qr encode <image> -o <file>      ເຂົ້າລະຫັດຮູບພາບເປັນ QR code (auto-downscaled)".into(),
    );

    m.insert(
        "cmd.qr.image_save_requires".into(),
        "ການບັນທຶກຮູບພາບຕ້ອງການ feature 'image2doc' (ເປີດໂດຍຄ່າເລີ່ມຕົ້ນ)".into(),
    );

    m.insert(
        "cmd.qr.image_requires_out".into(),
        "ການເຂົ້າລະຫັດຮູບພາບຕ້ອງການໄຟລ໌ຜົນລັບ: qr encode <image> -o <file>.png".into(),
    );

    m.insert(
        "cmd.qr.image_too_large".into(),
        "ຮູບພາບໃຫຍ່ເກີນໄປຈົນໃສ່ໃນ QR code ບໍ່ໄດ້ ເຖິງແມ່ນຂະໜາດຕ່ຳສຸດ.".into(),
    );

    m.insert(
        "cmd.qr.no_qr_detected".into(),
        "ບໍ່ພົບ QR code ໃນຮູບພາບ.".into(),
    );

    m.insert("cmd.qr.no_text".into(), "ບໍ່ມີຂໍ້ຄວາມໃຫ້ເຂົ້າລະຫັດ".into());

    m.insert("cmd.qr.open_image_fail".into(), "ເປີດຮູບພາບບໍ່ສຳເລັດ: {}".into());

    m.insert("cmd.qr.qr_label".into(), "{} {}".into());

    m.insert("cmd.qr.saved".into(), "ບັນທຶກ QR code ໄປທີ່: {}".into());

    m.insert("cmd.qr.usage_title".into(), "QR Code:".into());
}
