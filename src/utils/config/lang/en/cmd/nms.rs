use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nms.usage_title".into(), "Usage:".into());

    m.insert("cmd.nms.help_play".into(), "nms play <file>                                  Fullscreen sound player (video plays audio only)".into());

    m.insert("cmd.nms.help_scan".into(), "nms scan [-d|--depth <n>] [-s|--size] [-l|--long <dur>] [--care]  List audio + video files".into());

    m.insert(
        "cmd.nms.help_help".into(),
        "nms help                                         Show this help".into(),
    );

    m.insert("cmd.nms.options_title".into(), "Options:".into());

    m.insert(
        "cmd.nms.help_depth".into(),
        "  -d, --depth <n>   Max recursion depth (default: from config)".into(),
    );

    m.insert(
        "cmd.nms.help_size".into(),
        "  -s, --size        Show file sizes and a total".into(),
    );

    m.insert(
        "cmd.nms.help_long".into(),
        "  -l, --long <dur>  Only list media longer than <dur> (e.g. 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "cmd.nms.help_care".into(),
        "  --care            Ignore the ignore-rules (care mode)".into(),
    );

    m.insert("cmd.nms.keys_title".into(), "Player keys:".into());

    m.insert(
        "cmd.nms.help_keys".into(),
        "  space pause/resume · ←/→ seek 5s · ↑/↓ volume · a replay · q quit".into(),
    );

    m.insert("cmd.nms.examples_title".into(), "Examples:".into());

    m.insert("cmd.nms.example_play".into(), "nms play song.mp3".into());

    m.insert("cmd.nms.example_video".into(), "nms play movie.mp4".into());

    m.insert("cmd.nms.example_scan".into(), "nms scan -d 2 -s".into());

    m.insert("cmd.nms.example_long".into(), "nms scan -l 1mn".into());

    m.insert(
        "cmd.nms.play_missing_file".into(),
        "Missing file. Usage: nms play <file>".into(),
    );

    m.insert(
        "cmd.nms.file_not_found".into(),
        "File '{}' not found".into(),
    );

    m.insert(
        "cmd.nms.not_media".into(),
        "Not an audio/video file: '{}'".into(),
    );

    m.insert(
        "cmd.nms.no_sound_device".into(),
        "No sound device found on this machine — cannot play audio here".into(),
    );

    m.insert("cmd.nms.play_failed".into(), "Cannot play '{}': {}".into());

    m.insert(
        "cmd.nms.unknown_sub".into(),
        "Unknown nms subcommand: {}".into(),
    );

    m.insert("cmd.nms.unknown_option".into(), "Unknown option: {}".into());

    m.insert("cmd.nms.depth_usage".into(), "Usage: --depth <n>".into());

    m.insert("cmd.nms.invalid_depth".into(), "Invalid depth: '{}'".into());

    m.insert(
        "cmd.nms.long_usage".into(),
        "Usage: --long <duration> (e.g. 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "cmd.nms.invalid_long".into(),
        "Invalid duration: '{}' (use e.g. 25s, 1mn, 1h)".into(),
    );

    m.insert("cmd.nms.long_none".into(), "No media longer than {}".into());

    m.insert(
        "cmd.nms.no_files".into(),
        "No audio/video files found".into(),
    );

    m.insert(
        "cmd.nms.list_title".into(),
        "Media files (audio + video):".into(),
    );

    m.insert("cmd.nms.total_files".into(), "{} file(s)".into());

    m.insert(
        "cmd.nms.total_files_size".into(),
        "{} file(s), {} total".into(),
    );
}
