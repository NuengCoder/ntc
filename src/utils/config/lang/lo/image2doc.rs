use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "image2doc.ocr_disabled".into(),
        "OCR ບໍ່ໄດ້ເປີດໃຊ້ງານເທິງ platform ຫຼື build ນີ້. ຄອມໄພ້ໃໝ່ດ້ວຍ feature 'ocr' ຫຼື ຕິດຕັ້ງ tesseract.".into(),
    );

    m.insert(
        "image2doc.ocr_missing".into(),
        "ບໍ່ພົບ Tesseract. OCR ບໍ່ສາມາດໃຊ້ໄດ້.".into(),
    );

    m.insert(
        "image2doc.ocr_install".into(),
        "ຕິດຕັ້ງດ້ວຍ: ntpkg install tesseract".into(),
    );

    m.insert("image2doc.ocr_path_win".into(), "ຈາກນັ້ນເພີ່ມໃສ່ PATH: usrpath add <dir>, ຈາກນັ້ນ 'mode admin' + 'syspath add <dir>', ຫຼື ເປີດ terminal ໃໝ່.".into());

    m.insert(
        "image2doc.ocr_path_unix".into(),
        "ຈາກນັ້ນເພີ່ມໃສ່ PATH, ເຊັ່ນ: export PATH=\"/<dir>:$PATH\"".into(),
    );

    m.insert(
        "image2doc.ocr_rerun".into(),
        "ຈາກນັ້ນເຮັດວຽກຄຳສັ່ງດຽວກັນອີກຄັ້ງ.".into(),
    );

    m.insert(
        "image2doc.ocr_failed".into(),
        "Tesseract ປະມວນຜົນຮູບພາບບໍ່ສຳເລັດ.".into(),
    );
}
