use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.image.title".into(), "图像命令：".into());

    m.insert(
        "help.image.interactive".into(),
        "image ## 从当前目录选取图片并在终端内显示".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <路径> ## 在终端内显示图片（与 display 相同）".into(),
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
        "help.image.example1".into(),
        "image screenshot.png ## 在终端内显示图片".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image display photo.jpg --sixel ## 强制使用 sixel 渲染".into(),
    );
}
