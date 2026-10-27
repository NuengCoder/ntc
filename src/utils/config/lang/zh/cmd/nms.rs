use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nms.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.nms.help_play".into(),
        "nms play <文件>                                 全屏声音播放器（视频仅播放音频）".into(),
    );

    m.insert(
        "cmd.nms.help_scan".into(),
        "nms scan [-d|--depth <n>] [-s|--size] [-l|--long <时长>] [--care]  列出音频 + 视频文件"
            .into(),
    );

    m.insert(
        "cmd.nms.help_help".into(),
        "nms help                                         显示此帮助".into(),
    );

    m.insert("cmd.nms.options_title".into(), "选项：".into());

    m.insert(
        "cmd.nms.help_depth".into(),
        "  -d, --depth <n>   最大递归深度（默认取自配置）".into(),
    );

    m.insert(
        "cmd.nms.help_size".into(),
        "  -s, --size        显示文件大小和总计".into(),
    );

    m.insert(
        "cmd.nms.help_long".into(),
        "  -l, --long <时长>  仅列出长于 <时长> 的媒体（如 25s、1mn、1h）".into(),
    );

    m.insert(
        "cmd.nms.help_care".into(),
        "  --care            忽略忽略规则（care 模式）".into(),
    );

    m.insert("cmd.nms.keys_title".into(), "播放器按键：".into());

    m.insert(
        "cmd.nms.help_keys".into(),
        "  空格 暂停/继续 · ←/→ 快进快退 5 秒 · ↑/↓ 音量 · a 重播 · q 退出".into(),
    );

    m.insert("cmd.nms.examples_title".into(), "示例：".into());

    m.insert("cmd.nms.example_play".into(), "nms play song.mp3".into());

    m.insert("cmd.nms.example_video".into(), "nms play movie.mp4".into());

    m.insert("cmd.nms.example_scan".into(), "nms scan -d 2 -s".into());

    m.insert("cmd.nms.example_long".into(), "nms scan -l 1mn".into());

    m.insert(
        "cmd.nms.play_missing_file".into(),
        "缺少文件。用法：nms play <文件>".into(),
    );

    m.insert("cmd.nms.file_not_found".into(), "未找到文件 '{}'".into());

    m.insert("cmd.nms.not_media".into(), "不是音频/视频文件：'{}'".into());

    m.insert(
        "cmd.nms.no_sound_device".into(),
        "此设备未找到声卡设备 —— 无法在此播放音频".into(),
    );

    m.insert("cmd.nms.play_failed".into(), "无法播放 '{}'：{}".into());

    m.insert("cmd.nms.unknown_sub".into(), "未知的 nms 子命令：{}".into());

    m.insert("cmd.nms.unknown_option".into(), "未知选项：{}".into());

    m.insert("cmd.nms.depth_usage".into(), "用法：--depth <n>".into());

    m.insert("cmd.nms.invalid_depth".into(), "无效深度：'{}'".into());

    m.insert(
        "cmd.nms.long_usage".into(),
        "用法：--long <时长>（如 25s、1mn、1h）".into(),
    );

    m.insert(
        "cmd.nms.invalid_long".into(),
        "无效时长：'{}'（如 25s、1mn、1h）".into(),
    );

    m.insert("cmd.nms.long_none".into(), "没有长于 {} 的媒体".into());

    m.insert("cmd.nms.no_files".into(), "未找到音频/视频文件".into());

    m.insert(
        "cmd.nms.list_title".into(),
        "媒体文件（音频 + 视频）：".into(),
    );

    m.insert("cmd.nms.total_files".into(), "{} 个文件".into());

    m.insert("cmd.nms.total_files_size".into(), "{} 个文件，共 {}".into());
}
