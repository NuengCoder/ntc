use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.image.title".into(), "IMAGE COMMANDS:".into());

    m.insert(
        "help.image.interactive".into(),
        "image ## Pick an image from the current dir and show it inline".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <path> ## Show the image inline (same as display)".into(),
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
        "help.image.example1".into(),
        "image screenshot.png ## Show an image inline".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image display photo.jpg --sixel ## Force sixel rendering".into(),
    );
}
