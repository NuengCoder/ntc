use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.image.convert".into(),
        "image <path> to <fmt> ## ແປງ ແລະ ສະແດງຜົນລັບ (json|md|html|txt|desc)".into(),
    );

    m.insert(
        "help.image.display".into(),
        "image display <path> ## ສະແດງຮູບໃນ terminal ໂດຍກົງ".into(),
    );

    m.insert(
        "help.image.show".into(),
        "image show <path> ## ສະແດງຮູບໃນ terminal ໂດຍກົງ".into(),
    );

    m.insert(
        "help.image.sixel".into(),
        "image display <path> --sixel ## ບັງຄັບ render ແບບ sixel".into(),
    );

    m.insert(
        "help.image.interactive".into(),
        "image ## ເຄື່ອງມືແປງຮູບ interactive (ເລືອກຈາກໄດເຣັກທໍຣີປັດຈຸບັນ)".into(),
    );

    m.insert(
        "help.image.layout".into(),
        "image <path> --layout ## ເປີດການກວດຈັບ UI layout".into(),
    );

    m.insert(
        "help.image.ocr".into(),
        "image <path> --ocr ## ເປີດການສະກັດຂໍ້ຄວາມ OCR".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <path> ## ເປີດຮູບພາບໃນ viewer ເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "help.image.save".into(),
        "image <path> out <fmt> ## ແປງ ແລະ ບັນທຶກໄປທີ່ path ທີ່ຕັ້ງໄວ້".into(),
    );

    m.insert("help.image.title".into(), "ຄຳສັ່ງ IMAGE:".into());

    m.insert(
        "help.image.example1".into(),
        "image screenshot.png to json ## ແປງຮູບເປັນ JSON ສຳລັບ AI".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image photo.jpg --ocr ## ສະກັດຂໍ້ຄວາມຈາກຮູບຜ່ານ OCR".into(),
    );
}
