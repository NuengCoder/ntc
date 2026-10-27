use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "已取消。".into());

    m.insert("cmd.image.default_label".into(), "默认".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. 取消".into());

    m.insert("cmd.image.desc_html".into(), "  {}. HTML".into());

    m.insert("cmd.image.desc_json".into(), "  {}. JSON".into());

    m.insert(
        "cmd.image.desc_markdown".into(),
        "  {}. Markdown（默认）".into(),
    );

    m.insert(
        "cmd.image.desc_narrative".into(),
        "  {}. 叙述性描述（供 AI 使用）".into(),
    );

    m.insert("cmd.image.desc_txt".into(), "  {}. TXT".into());

    m.insert("cmd.image.enter_number".into(), "请输入编号：".into());

    m.insert(
        "cmd.image.missing_out_format".into(),
        "'out' 后缺少格式。".into(),
    );

    m.insert(
        "cmd.image.missing_to_format".into(),
        "'to' 后缺少格式。".into(),
    );

    m.insert(
        "cmd.image.no_files".into(),
        "当前目录中未找到图像文件。".into(),
    );

    m.insert("cmd.image.select_image".into(), "选择图片：".into());

    m.insert("cmd.image.displaying".into(), "显示 {} ({}x{}) · {}".into());

    m.insert(
        "cmd.image.usage".into(),
        "用法：image <路径> [display|show [--sixel]|to|out <格式>] [--ocr] [--layout]".into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "正在强制 sixel 输出（检测到：{}）——该终端可能显示乱码。".into(),
    );
}
