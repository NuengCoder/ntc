use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "NMS COMMANDS (NTC Music):".into());

    m.insert(
        "help.nms.play".into(),
        "nms play [-sf <dur>] <file> ## Play sound in the terminal (Ctrl+C stops; -sf starts from an offset)".into(),
    );

    m.insert("help.nms.scan".into(), "nms scan [-d <n>] [-s] [-l <dur>] [--care] ## List audio + video files (dur: 25s, 1mn, 1h)".into());

    m.insert(
        "help.nms.long".into(),
        "nms scan -l <dur> ## Only list media longer than <dur> (e.g. 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "help.nms.keys".into(),
        "Plays in the terminal with a live position/remaining countdown · video plays audio only (through ffmpeg) · Ctrl+C stops".into(),
    );

    m.insert(
        "help.nms.example1".into(),
        "nms play -sf 30 song.mp3 ## Start an audio file at 0:30".into(),
    );

    m.insert(
        "help.nms.example2".into(),
        "nms play movie.mp4 ## Play a video's audio track".into(),
    );

    m.insert(
        "help.nms.example3".into(),
        "nms scan -d 2 -s ## List media with sizes".into(),
    );

    m.insert(
        "help.nms.example4".into(),
        "nms scan -l 1mn ## List media longer than 1 minute".into(),
    );
}
