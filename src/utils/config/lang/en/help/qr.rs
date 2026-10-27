use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qr.decode".into(),
        "qr decode <image> ## Decode QR code from image".into(),
    );

    m.insert(
        "help.qr.encode".into(),
        "qr <text> ## Encode text and display QR in terminal".into(),
    );

    m.insert(
        "help.qr.save".into(),
        "qr encode <text> -o <file> ## Save QR as PNG image".into(),
    );

    m.insert(
        "help.qr.image".into(),
        "qr encode <image> -o <file> ## Encode an image into a QR (scan to view image)".into(),
    );

    m.insert("help.qr.title".into(), "QR CODE COMMANDS:".into());

    m.insert(
        "help.qr.example1".into(),
        "qr \"Hello World\" ## Display QR code in terminal".into(),
    );

    m.insert(
        "help.qr.example2".into(),
        "qr encode \"Hello\" -o hello.png ## Save QR as PNG".into(),
    );

    m.insert(
        "help.qr.example3".into(),
        "qr decode qr.png ## Decode QR from image".into(),
    );

    m.insert(
        "help.qr.example4".into(),
        "qr encode image.png -o imageqrcode.png ## Encode image so scanning shows it".into(),
    );
}
