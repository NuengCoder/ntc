use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qr.decode_fail".into(),
        "Failed to decode QR #{}: {}".into(),
    );

    m.insert("cmd.qr.decode_requires".into(), "QR decode requires feature 'image2doc' (enabled by default). Run with --features image2doc.".into());

    m.insert(
        "cmd.qr.decode_usage".into(),
        "Usage: qr decode <image_file>".into(),
    );

    m.insert(
        "cmd.qr.encode_usage".into(),
        "Usage: qr encode <text|image> [-o <file>]".into(),
    );

    m.insert("cmd.qr.example1".into(), "  qr \"Hello, World!\"".into());

    m.insert(
        "cmd.qr.example2".into(),
        "  qr encode \\\"hello world\\\" -o qr.png".into(),
    );

    m.insert("cmd.qr.example3".into(), "  qr decode qr.png".into());

    m.insert(
        "cmd.qr.example_image".into(),
        "  qr encode image.png -o imageqrcode.png".into(),
    );

    m.insert("cmd.qr.examples_title".into(), "Examples:".into());

    m.insert("cmd.qr.file_not_found".into(), "File not found: {}".into());

    m.insert(
        "cmd.qr.gen_fail".into(),
        "Failed to generate QR code: {}".into(),
    );

    m.insert(
        "cmd.qr.help_decode".into(),
        "  qr decode <image_file>           Decode QR code from image".into(),
    );

    m.insert(
        "cmd.qr.help_encode".into(),
        "  qr <text>                        Encode and display QR code in terminal".into(),
    );

    m.insert(
        "cmd.qr.help_same_as_above".into(),
        "  qr encode <text>                 Same as above".into(),
    );

    m.insert(
        "cmd.qr.help_save_png".into(),
        "  qr encode <text> -o <file>       Save QR as PNG image".into(),
    );

    m.insert(
        "cmd.qr.help_encode_image".into(),
        "  qr encode <image> -o <file>       Encode an image into a QR code (auto-downscaled)"
            .into(),
    );

    m.insert(
        "cmd.qr.image_save_requires".into(),
        "Image saving requires feature 'image2doc' (enabled by default)".into(),
    );

    m.insert(
        "cmd.qr.image_requires_out".into(),
        "Encoding an image requires an output file: qr encode <image> -o <file>.png".into(),
    );

    m.insert(
        "cmd.qr.image_too_large".into(),
        "Image is too large to fit in a QR code even at minimum size.".into(),
    );

    m.insert(
        "cmd.qr.no_qr_detected".into(),
        "No QR code detected in the image.".into(),
    );

    m.insert("cmd.qr.no_text".into(), "No text to encode".into());

    m.insert(
        "cmd.qr.open_image_fail".into(),
        "Failed to open image: {}".into(),
    );

    m.insert("cmd.qr.qr_label".into(), "{} {}".into());

    m.insert("cmd.qr.saved".into(), "QR code saved to: {}".into());

    m.insert("cmd.qr.usage_title".into(), "QR Code:".into());
}
