use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.image.convert".into(),
        "image <path> to <fmt> ## Convert and display output (json|md|html|txt|desc)".into(),
    );

    m.insert(
        "help.image.display".into(),
        "image display <path> ## Render image inline in the terminal".into(),
    );

    m.insert(
        "help.image.show".into(),
        "image show <path> ## Render image inline in the terminal".into(),
    );

    m.insert(
        "help.image.sixel".into(),
        "image display <path> --sixel ## Force sixel rendering".into(),
    );

    m.insert(
        "help.image.interactive".into(),
        "image ## Interactive image converter (pick from current dir)".into(),
    );

    m.insert(
        "help.image.layout".into(),
        "image <path> --layout ## Enable UI layout detection".into(),
    );

    m.insert(
        "help.image.ocr".into(),
        "image <path> --ocr ## Enable OCR text extraction".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <path> ## Open image in default viewer".into(),
    );

    m.insert(
        "help.image.save".into(),
        "image <path> out <fmt> ## Convert and save to setO path".into(),
    );

    m.insert("help.image.title".into(), "IMAGE COMMANDS:".into());

    m.insert(
        "help.image.example1".into(),
        "image screenshot.png to json ## Convert image to JSON for AI".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image photo.jpg --ocr ## Extract text from image via OCR".into(),
    );
}
