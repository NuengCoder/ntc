use std::collections::HashMap;

pub(crate) mod arc;
pub(crate) mod care;
pub(crate) mod clipboard;
pub(crate) mod config;
pub(crate) mod fake;
pub(crate) mod fmt;
pub(crate) mod gs;
pub(crate) mod http;
pub(crate) mod igcar;
pub(crate) mod igcare;
pub(crate) mod igcared;
pub(crate) mod ignore;
pub(crate) mod image;
pub(crate) mod ipcfg;
pub(crate) mod mode;
pub(crate) mod nav;
pub(crate) mod nms;
pub(crate) mod nsheet;
pub(crate) mod nssh;
pub(crate) mod pathman;
pub(crate) mod qr;
pub(crate) mod qsc;
pub(crate) mod report;
pub(crate) mod scan;
pub(crate) mod serve;
pub(crate) mod smtp;
pub(crate) mod spec;
pub(crate) mod target;
pub(crate) mod time;
pub(crate) mod tl;
pub(crate) mod tmprt;
pub(crate) mod todo;
pub(crate) mod upgrade;
pub(crate) mod views;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.completed".into(),
        "Command completed successfully.".into(),
    );

    m.insert(
        "cmd.image_disabled".into(),
        "Image conversion not available (feature 'image2doc' disabled).".into(),
    );

    m.insert(
        "cmd.terminated".into(),
        "Command terminated (Ctrl+C)".into(),
    );
}
