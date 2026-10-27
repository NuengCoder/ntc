use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "NMS COMMANDS (NTC Music):".into());

    m.insert("help.nms.play".into(), "nms play <file> ## Fullscreen sound player (space pause · ←/→ seek · ↑/↓ volume · a replay)".into());

    m.insert("help.nms.scan".into(), "nms scan [-d <n>] [-s] [-l <dur>] [--care] ## List audio + video files (dur: 25s, 1mn, 1h)".into());

    m.insert(
        "help.nms.long".into(),
        "nms scan -l <dur> ## Only list media longer than <dur> (e.g. 25s, 1mn, 1h)".into(),
    );

    m.insert("help.nms.keys".into(), "Player TUI: live waveform + high/low level meter · video plays audio only · freezes when finished, press a to replay".into());

    m.insert(
        "help.nms.example1".into(),
        "nms play song.mp3 ## Play an audio file".into(),
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
