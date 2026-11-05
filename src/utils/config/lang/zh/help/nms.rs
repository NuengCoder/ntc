use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "NMS 命令（NTC 音乐）：".into());

    m.insert(
        "help.nms.play".into(),
        "nms play [-sf <时长>] <文件> ## 在终端播放声音（Ctrl+C 停止；-sf 指定开始位置）".into(),
    );

    m.insert(
        "help.nms.scan".into(),
        "nms scan [-d <n>] [-s] [-l <时长>] [--care] ## 列出音频 + 视频文件（时长：25s、1mn、1h）"
            .into(),
    );

    m.insert(
        "help.nms.long".into(),
        "nms scan -l <时长> ## 仅列出长于 <时长> 的媒体（如 25s、1mn、1h）".into(),
    );

    m.insert(
        "help.nms.keys".into(),
        "在终端中播放并显示实时位置 / 剩余时间倒计时 · 视频仅播放音频（通过 ffmpeg） · Ctrl+C 停止".into(),
    );

    m.insert(
        "help.nms.example1".into(),
        "nms play -sf 30 song.mp3 ## 从 0:30 开始播放音频".into(),
    );

    m.insert(
        "help.nms.example2".into(),
        "nms play movie.mp4 ## 播放视频的音轨".into(),
    );

    m.insert(
        "help.nms.example3".into(),
        "nms scan -d 2 -s ## 列出媒体文件及大小".into(),
    );

    m.insert(
        "help.nms.example4".into(),
        "nms scan -l 1mn ## 列出长于 1 分钟的媒体".into(),
    );
}
