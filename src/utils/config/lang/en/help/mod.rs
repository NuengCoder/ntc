use std::collections::HashMap;

pub(crate) mod alias;
pub(crate) mod arc;
pub(crate) mod autoup;
pub(crate) mod backup;
pub(crate) mod clipboard;
pub(crate) mod config;
pub(crate) mod context;
pub(crate) mod deadlock;
pub(crate) mod editor;
pub(crate) mod encdec;
pub(crate) mod explorer;
pub(crate) mod fake;
pub(crate) mod fileops;
pub(crate) mod fmt;
pub(crate) mod git;
pub(crate) mod igcare;
pub(crate) mod image;
pub(crate) mod math;
pub(crate) mod mode;
pub(crate) mod nav;
pub(crate) mod ndb;
pub(crate) mod network;
pub(crate) mod nms;
pub(crate) mod nsheet;
pub(crate) mod nssh;
pub(crate) mod ntpkg;
pub(crate) mod other;
pub(crate) mod pathman;
pub(crate) mod pipe;
pub(crate) mod python;
pub(crate) mod qr;
pub(crate) mod qsc;
pub(crate) mod ran;
pub(crate) mod report;
pub(crate) mod scan;
pub(crate) mod scr;
pub(crate) mod search;
pub(crate) mod size;
pub(crate) mod syscmd;
pub(crate) mod teleport;
pub(crate) mod theme;
pub(crate) mod time;
pub(crate) mod tl;
pub(crate) mod tmprt;
pub(crate) mod todo;
pub(crate) mod view;
pub(crate) mod viewd;
pub(crate) mod viewf;
pub(crate) mod views;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.examples_header".into(), "Examples:".into());

    m.insert(
        "help.unknown_topic".into(),
        "Unknown help topic: '{}'. Run 'help' for all topics.".into(),
    );
}
