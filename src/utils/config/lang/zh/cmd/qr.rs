use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qr.decode_fail".into(),
        "解码二维码 #{} 失败：{}".into(),
    );

    m.insert(
        "cmd.qr.decode_requires".into(),
        "二维码解码需要 'image2doc' 功能（默认启用）。使用 --features image2doc 运行。".into(),
    );

    m.insert(
        "cmd.qr.decode_usage".into(),
        "用法：qr decode <图像文件>".into(),
    );

    m.insert(
        "cmd.qr.encode_usage".into(),
        "用法：qr encode <文本|图像> [-o <文件>]".into(),
    );

    m.insert("cmd.qr.example1".into(), "  qr \"你好，世界！\"".into());

    m.insert(
        "cmd.qr.example2".into(),
        "  qr encode \"hello world\" -o qr.png".into(),
    );

    m.insert("cmd.qr.example3".into(), "  qr decode qr.png".into());

    m.insert(
        "cmd.qr.example_image".into(),
        "  qr encode image.png -o imageqrcode.png".into(),
    );

    m.insert("cmd.qr.examples_title".into(), "示例：".into());

    m.insert("cmd.qr.file_not_found".into(), "未找到文件：{}".into());

    m.insert("cmd.qr.gen_fail".into(), "生成二维码失败：{}".into());

    m.insert(
        "cmd.qr.help_decode".into(),
        "  qr decode <图像文件>            从图像解码二维码".into(),
    );

    m.insert(
        "cmd.qr.help_encode".into(),
        "  qr <文本>                        编码文本并在终端中显示二维码".into(),
    );

    m.insert(
        "cmd.qr.help_same_as_above".into(),
        "  qr encode <文本>                 同上".into(),
    );

    m.insert(
        "cmd.qr.help_save_png".into(),
        "  qr encode <文本> -o <文件>       将二维码保存为 PNG 图像".into(),
    );

    m.insert(
        "cmd.qr.help_encode_image".into(),
        "  qr encode <图像> -o <文件>       将图像编码为二维码（自动缩小）".into(),
    );

    m.insert(
        "cmd.qr.image_save_requires".into(),
        "图像保存需要 'image2doc' 功能（默认启用）".into(),
    );

    m.insert(
        "cmd.qr.image_requires_out".into(),
        "编码图像需要指定输出文件：qr encode <图像> -o <文件>.png".into(),
    );

    m.insert(
        "cmd.qr.image_too_large".into(),
        "图像即使缩小到最小尺寸仍无法放入二维码。".into(),
    );

    m.insert(
        "cmd.qr.no_qr_detected".into(),
        "图像中未检测到二维码。".into(),
    );

    m.insert("cmd.qr.no_text".into(), "没有要编码的文本".into());

    m.insert("cmd.qr.open_image_fail".into(), "打开图像失败：{}".into());

    m.insert("cmd.qr.qr_label".into(), "{} {}".into());

    m.insert("cmd.qr.saved".into(), "二维码已保存到：{}".into());

    m.insert("cmd.qr.usage_title".into(), "二维码（QR Code）：".into());
}
