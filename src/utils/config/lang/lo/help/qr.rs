use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qr.decode".into(),
        "qr decode <image> ## ຖອດລະຫັດ QR ຈາກຮູບພາບ".into(),
    );

    m.insert(
        "help.qr.encode".into(),
        "qr <text> ## ເຂົ້າລະຫັດຂໍ້ຄວາມ ແລະ ສະແດງ QR ໃນ terminal".into(),
    );

    m.insert(
        "help.qr.save".into(),
        "qr encode <text> -o <file> ## ບັນທຶກ QR ເປັນຮູບ PNG".into(),
    );

    m.insert(
        "help.qr.image".into(),
        "qr encode <image> -o <file> ## ເຂົ້າລະຫັດຮູບພາບໃສ່ QR (scan ເພື່ອເບິ່ງຮູບ)".into(),
    );

    m.insert("help.qr.title".into(), "ຄຳສັ່ງ QR CODE:".into());

    m.insert(
        "help.qr.example1".into(),
        "qr \"Hello World\" ## ສະແດງ QR code ໃນ terminal".into(),
    );

    m.insert(
        "help.qr.example2".into(),
        "qr encode \"Hello\" -o hello.png ## ບັນທຶກ QR ເປັນ PNG".into(),
    );

    m.insert(
        "help.qr.example3".into(),
        "qr decode qr.png ## ຖອດລະຫັດ QR ຈາກຮູບ".into(),
    );

    m.insert(
        "help.qr.example4".into(),
        "qr encode image.png -o imageqrcode.png ## ເຂົ້າລະຫັດຮູບເພື່ອໃຫ້ scan ເຫັນມັນ".into(),
    );
}
