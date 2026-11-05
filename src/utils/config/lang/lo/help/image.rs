use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.image.title".into(), "ຄຳສັ່ງ IMAGE:".into());

    m.insert(
        "help.image.interactive".into(),
        "image ## ເລືອກຮູບຈາກໄດເຣັກທໍຣີປັດຈຸບັນແລ້ວສະແດງໃນ terminal".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <path> ## ສະແດງຮູບໃນ terminal (ເໝືອນ display)".into(),
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
        "help.image.example1".into(),
        "image screenshot.png ## ສະແດງຮູບໃນ terminal".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image display photo.jpg --sixel ## ບັງຄັບ render ແບບ sixel".into(),
    );
}
