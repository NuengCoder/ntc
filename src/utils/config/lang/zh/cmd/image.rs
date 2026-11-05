use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "已取消。".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. 取消".into());

    m.insert("cmd.image.enter_number".into(), "请输入编号：".into());

    m.insert(
        "cmd.image.no_ansi".into(),
        "此终端不支持 ANSI 颜色，无法在终端内显示 '{}'。".into(),
    );

    m.insert(
        "cmd.image.no_files".into(),
        "当前目录中未找到图像文件。".into(),
    );

    m.insert("cmd.image.select_image".into(), "选择图片：".into());

    m.insert("cmd.image.displaying".into(), "显示 {} ({}x{}) · {}".into());

    m.insert(
        "cmd.image.usage".into(),
        "用法：image [display|show] [--sixel] [<路径>]".into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "正在强制 sixel 输出（检测到：{}）——该终端可能显示乱码。".into(),
    );
}
