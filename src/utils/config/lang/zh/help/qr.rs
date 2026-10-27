use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qr.decode".into(),
        "qr decode <图像> ## 从图像解码QR码".into(),
    );

    m.insert(
        "help.qr.encode".into(),
        "qr <文本> ## 编码文本并在终端中显示QR码".into(),
    );

    m.insert(
        "help.qr.save".into(),
        "qr encode <文本> -o <文件> ## 将QR码保存为PNG图像".into(),
    );

    m.insert(
        "help.qr.image".into(),
        "qr encode <图像> -o <文件> ## 将图像编码为二维码（扫描即可查看图像）".into(),
    );

    m.insert("help.qr.title".into(), "QR码命令：".into());

    m.insert(
        "help.qr.example1".into(),
        "qr \"Hello World\" ## 在终端显示二维码".into(),
    );

    m.insert(
        "help.qr.example2".into(),
        "qr encode \"Hello\" -o hello.png ## 保存二维码为 PNG".into(),
    );

    m.insert(
        "help.qr.example3".into(),
        "qr decode qr.png ## 从图片解码二维码".into(),
    );

    m.insert(
        "help.qr.example4".into(),
        "qr encode image.png -o imageqrcode.png ## 将图像编码为二维码，扫描即显示".into(),
    );
}
