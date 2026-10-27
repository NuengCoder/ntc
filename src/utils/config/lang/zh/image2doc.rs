use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "image2doc.ocr_disabled".into(),
        "此平台或构建未启用 OCR。使用 'ocr' 功能重新编译或安装 tesseract。".into(),
    );

    m.insert(
        "image2doc.ocr_missing".into(),
        "未找到 Tesseract。OCR 不可用。".into(),
    );

    m.insert(
        "image2doc.ocr_install".into(),
        "通过以下命令安装：ntpkg install tesseract".into(),
    );

    m.insert("image2doc.ocr_path_win".into(), "然后将其添加到 PATH：usrpath add <目录>，接着执行 'mode admin' + 'syspath add <目录>'，或重新打开终端。".into());

    m.insert(
        "image2doc.ocr_path_unix".into(),
        "然后将其添加到 PATH，例如：export PATH=\"/<目录>:$PATH\"".into(),
    );

    m.insert(
        "image2doc.ocr_rerun".into(),
        "然后再次运行相同的命令。".into(),
    );

    m.insert(
        "image2doc.ocr_failed".into(),
        "Tesseract 处理图像失败。".into(),
    );
}
