use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("image2doc.ocr_disabled".into(), "OCR not enabled on this platform or build. Recompile with 'ocr' feature or install tesseract.".into());

    m.insert(
        "image2doc.ocr_missing".into(),
        "Tesseract not found. OCR is unavailable.".into(),
    );

    m.insert(
        "image2doc.ocr_install".into(),
        "Install it with: ntpkg install tesseract".into(),
    );

    m.insert("image2doc.ocr_path_win".into(), "Then add it to PATH: usrpath add <dir>, then 'mode admin' + 'syspath add <dir>', or open a new terminal.".into());

    m.insert(
        "image2doc.ocr_path_unix".into(),
        "Then add it to PATH, e.g.: export PATH=\"/<dir>:$PATH\"".into(),
    );

    m.insert(
        "image2doc.ocr_rerun".into(),
        "Then run the same command again.".into(),
    );

    m.insert(
        "image2doc.ocr_failed".into(),
        "Tesseract failed to process the image.".into(),
    );
}
