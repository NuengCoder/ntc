use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.image.convert".into(),
        "image <路径> to <格式> ## 转换并显示输出（json|md|html|txt|desc）".into(),
    );

    m.insert(
        "help.image.display".into(),
        "image display <路径> ## 在终端内直接渲染图像".into(),
    );

    m.insert(
        "help.image.show".into(),
        "image show <路径> ## 在终端内直接渲染图像".into(),
    );

    m.insert(
        "help.image.sixel".into(),
        "image display <路径> --sixel ## 强制使用 sixel 渲染".into(),
    );

    m.insert(
        "help.image.interactive".into(),
        "image ## 交互式图像转换器（从当前目录选取）".into(),
    );

    m.insert(
        "help.image.layout".into(),
        "image <路径> --layout ## 启用UI布局检测".into(),
    );

    m.insert(
        "help.image.ocr".into(),
        "image <路径> --ocr ## 启用OCR文本提取".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <路径> ## 在默认查看器中打开图像".into(),
    );

    m.insert(
        "help.image.save".into(),
        "image <路径> out <格式> ## 转换并保存到输出目录".into(),
    );

    m.insert("help.image.title".into(), "图像命令：".into());

    m.insert(
        "help.image.example1".into(),
        "image screenshot.png to json ## 将图片转换为 JSON 供 AI 使用".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image photo.jpg --ocr ## 通过 OCR 从图片提取文本".into(),
    );
}
