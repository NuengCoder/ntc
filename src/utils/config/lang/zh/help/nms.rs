use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "NMS 命令（NTC 音乐）：".into());

    m.insert(
        "help.nms.play".into(),
        "nms play <文件> ## 全屏声音播放器（空格暂停 · ←/→ 快进 · ↑/↓ 音量 · a 重播）".into(),
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
        "播放器界面：实时波形 + 高低电平表 · 视频仅播放音频 · 播放结束会定格，按 a 重播".into(),
    );

    m.insert(
        "help.nms.example1".into(),
        "nms play song.mp3 ## 播放音频文件".into(),
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
