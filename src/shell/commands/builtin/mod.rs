//! Built-in command modules and their static registry entries.
//!
//! Every built-in command lives in its own module here and exports one static
//! [`CommandSpec`]. [`BUILTIN_SPECS`] is the only command table; dispatch,
//! help and completion all read it through `registry_data::registry()`.

pub(crate) mod arc;
pub(crate) mod autoup;
pub(crate) mod back;
pub(crate) mod backup;
pub(crate) mod betw;
pub(crate) mod care;
pub(crate) mod cd;
pub(crate) mod clear;
pub(crate) mod clip;
pub(crate) mod config;
pub(crate) mod count;
pub(crate) mod cp;
pub(crate) mod cvcode;
pub(crate) mod deadlock;
pub(crate) mod decode;
pub(crate) mod deinit;
pub(crate) mod dl;
pub(crate) mod dns;
pub(crate) mod doctor;
pub(crate) mod ds;
pub(crate) mod encode;
pub(crate) mod exit;
pub(crate) mod fake;
pub(crate) mod fgo;
pub(crate) mod fmt;
pub(crate) mod fs;
pub(crate) mod fsc;
pub(crate) mod git;
pub(crate) mod godrive;
pub(crate) mod gos;
pub(crate) mod gosc;
pub(crate) mod gs;
pub(crate) mod head;
pub(crate) mod help;
pub(crate) mod http;
pub(crate) mod igcare;
pub(crate) mod igcared;
pub(crate) mod ignore;
pub(crate) mod image;
pub(crate) mod init;
pub(crate) mod ipcfg;
pub(crate) mod line;
pub(crate) mod local;
pub(crate) mod locate;
pub(crate) mod math;
pub(crate) mod mkd;
pub(crate) mod mkf;
pub(crate) mod mode;
pub(crate) mod mv;
pub(crate) mod ndb;
pub(crate) mod ne;
pub(crate) mod nfe;
pub(crate) mod nms;
pub(crate) mod nprintf;
pub(crate) mod nsheet;
pub(crate) mod nssh;
pub(crate) mod ntpkg;
pub(crate) mod ntpx;
pub(crate) mod opencg;
pub(crate) mod pathcp;
pub(crate) mod ping;
pub(crate) mod py_activate;
pub(crate) mod py_deactivate;
pub(crate) mod qr;
pub(crate) mod qsc;
pub(crate) mod qur;
pub(crate) mod ral;
pub(crate) mod ran;
pub(crate) mod report;
pub(crate) mod resetcg;
pub(crate) mod restorecg;
pub(crate) mod rev;
pub(crate) mod rmd;
pub(crate) mod rmf;
pub(crate) mod rnmd;
pub(crate) mod rnmf;
pub(crate) mod run;
pub(crate) mod say;
pub(crate) mod scan;
pub(crate) mod scr;
pub(crate) mod serve;
pub(crate) mod showcg;
pub(crate) mod showenv;
pub(crate) mod size;
pub(crate) mod skill;
pub(crate) mod smtp;
pub(crate) mod sort;
pub(crate) mod spec;
pub(crate) mod sys;
pub(crate) mod syspath;
pub(crate) mod tail;
pub(crate) mod theme;
pub(crate) mod time;
pub(crate) mod tl;
pub(crate) mod tmprt;
pub(crate) mod todo;
pub(crate) mod tp;
pub(crate) mod tpb;
pub(crate) mod trace;
pub(crate) mod trim;
pub(crate) mod tutorial;
pub(crate) mod txtc;
pub(crate) mod txtf;
pub(crate) mod ui;
pub(crate) mod up;
pub(crate) mod usrpath;
pub(crate) mod version;
pub(crate) mod view;
pub(crate) mod viewd;
pub(crate) mod viewf;
pub(crate) mod views;
pub(crate) mod r#where;
pub(crate) mod wi;

use super::core::help::{CompactEntry, ContextualHelp, HelpTopic};
use super::core::spec::CommandSpec;

/// Specs owned by modules already moved under `builtin/`.
pub(crate) static BUILTIN_SPECS: &[&CommandSpec] = &[
    &clear::SPEC,
    &version::SPEC,
    &help::SPEC,
    &tutorial::SPEC,
    &exit::SPEC,
    &cd::SPEC,
    &godrive::SPEC,
    &back::SPEC,
    &r#where::SPEC,
    &gos::SPEC,
    &gosc::SPEC,
    &fgo::SPEC,
    &gs::SPEC,
    &fs::SPEC,
    &ds::SPEC,
    &fsc::SPEC,
    &locate::SPEC,
    &math::SPEC,
    &head::SPEC,
    &mkf::SPEC,
    &mkd::SPEC,
    &mv::SPEC,
    &cp::SPEC,
    &size::SPEC,
    &ne::SPEC,
    &py_activate::SPEC,
    &py_deactivate::SPEC,
    &showenv::SPEC,
    &say::SPEC,
    &ui::SPEC,
    &rmf::SPEC,
    &rmd::SPEC,
    &rnmf::SPEC,
    &rnmd::SPEC,
    &pathcp::SPEC,
    &tail::SPEC,
    &betw::SPEC,
    &sort::SPEC,
    &rev::SPEC,
    &count::SPEC,
    &trim::SPEC,
    &line::SPEC,
    &qur::SPEC,
    &qr::SPEC,
    &qsc::SPEC,
    &clip::SPEC,
    &config::SPEC,
    &tl::SPEC,
    &mode::SPEC,
    &autoup::SPEC,
    &tmprt::SPEC,
    &time::SPEC,
    &git::SPEC,
    &fake::SPEC,
    &fmt::SPEC,
    &todo::SPEC,
    &scan::SPEC,
    &scr::SPEC,
    &smtp::SPEC,
    &ndb::SPEC,
    &nms::SPEC,
    &nprintf::SPEC,
    &nssh::SPEC,
    &nsheet::SPEC,
    &tp::SPEC,
    &tpb::SPEC,
    &deadlock::SPEC,
    &showcg::SPEC,
    &opencg::SPEC,
    &resetcg::SPEC,
    &restorecg::SPEC,
    &local::SPEC,
    &init::SPEC,
    &deinit::SPEC,
    &view::SPEC,
    &viewf::SPEC,
    &viewd::SPEC,
    &views::SPEC,
    &report::SPEC,
    &txtc::SPEC,
    &txtf::SPEC,
    &dl::SPEC,
    &up::SPEC,
    &dns::SPEC,
    &doctor::SPEC,
    &encode::SPEC,
    &decode::SPEC,
    &cvcode::SPEC,
    &ignore::SPEC,
    &care::SPEC,
    &igcare::SPEC,
    &igcared::SPEC,
    &serve::SPEC,
    &arc::SPEC,
    &backup::SPEC,
    &http::SPEC,
    &ipcfg::SPEC,
    &ral::SPEC,
    &ran::SPEC,
    &run::SPEC,
    &skill::SPEC,
    &spec::SPEC,
    &sys::SPEC,
    &theme::SPEC,
    &wi::SPEC,
    &usrpath::SPEC,
    &syspath::SPEC,
    &ping::SPEC,
    &trace::SPEC,
    &image::SPEC,
    &nfe::SPEC,
    &ntpkg::SPEC,
    &ntpx::SPEC,
];

/// Every `help <topic>` section: one entry per canonical command (plus the
/// `pipe` operator topic), with the aliases it answers to and the command
/// module that renders it. Order is alphabetical.
pub(crate) static HELP_TOPICS: &[HelpTopic] = &[
    HelpTopic {
        topic: "arc",
        aliases: &[],
        render: arc::help::print_arc_help_section,
    },
    HelpTopic {
        topic: "autoup",
        aliases: &[],
        render: autoup::help::print_autoup_help_section,
    },
    HelpTopic {
        topic: "back",
        aliases: &["b"],
        render: back::help::print_back_help_section,
    },
    HelpTopic {
        topic: "backup",
        aliases: &[],
        render: backup::help::print_backup_help_section,
    },
    HelpTopic {
        topic: "betw",
        aliases: &[],
        render: betw::help::print_betw_help_section,
    },
    HelpTopic {
        topic: "care",
        aliases: &[],
        render: care::help::print_care_help_section,
    },
    HelpTopic {
        topic: "cd",
        aliases: &[],
        render: cd::help::print_cd_help_section,
    },
    HelpTopic {
        topic: "clear",
        aliases: &[],
        render: clear::help::print_clear_help_section,
    },
    HelpTopic {
        topic: "clip",
        aliases: &[],
        render: clip::help::print_clip_help_section,
    },
    HelpTopic {
        topic: "config",
        aliases: &["cfg"],
        render: config::help::print_config_help_section,
    },
    HelpTopic {
        topic: "count",
        aliases: &[],
        render: count::help::print_count_help_section,
    },
    HelpTopic {
        topic: "cp",
        aliases: &[],
        render: cp::help::print_cp_help_section,
    },
    HelpTopic {
        topic: "cvcode",
        aliases: &[],
        render: cvcode::help::print_cvcode_help_section,
    },
    HelpTopic {
        topic: "deadlock",
        aliases: &[],
        render: deadlock::help::print_deadlock_help_section,
    },
    HelpTopic {
        topic: "decode",
        aliases: &[],
        render: decode::help::print_decode_help_section,
    },
    HelpTopic {
        topic: "deinit",
        aliases: &[],
        render: deinit::help::print_deinit_help_section,
    },
    HelpTopic {
        topic: "dl",
        aliases: &[],
        render: dl::help::print_dl_help_section,
    },
    HelpTopic {
        topic: "dns",
        aliases: &[],
        render: dns::help::print_dns_help_section,
    },
    HelpTopic {
        topic: "ds",
        aliases: &[],
        render: ds::help::print_ds_help_section,
    },
    HelpTopic {
        topic: "doctor",
        aliases: &[],
        render: doctor::help::print_doctor_help_section,
    },
    HelpTopic {
        topic: "encode",
        aliases: &[],
        render: encode::help::print_encode_help_section,
    },
    HelpTopic {
        topic: "exit",
        aliases: &["quit", "esc"],
        render: exit::help::print_exit_help_section,
    },
    HelpTopic {
        topic: "explorer",
        aliases: &["nfe"],
        render: nfe::help::print_explorer_help_section,
    },
    HelpTopic {
        topic: "fake",
        aliases: &[],
        render: fake::help::print_fake_help_section,
    },
    HelpTopic {
        topic: "fgo",
        aliases: &[],
        render: fgo::help::print_fgo_help_section,
    },
    HelpTopic {
        topic: "fmt",
        aliases: &[],
        render: fmt::help::print_fmt_help_section,
    },
    HelpTopic {
        topic: "fs",
        aliases: &[],
        render: fs::help::print_fs_help_section,
    },
    HelpTopic {
        topic: "fsc",
        aliases: &[],
        render: fsc::help::print_fsc_help_section,
    },
    HelpTopic {
        topic: "git",
        aliases: &[],
        render: git::help::print_git_help_section,
    },
    HelpTopic {
        topic: "godrive",
        aliases: &["god"],
        render: godrive::help::print_godrive_help_section,
    },
    HelpTopic {
        topic: "gos",
        aliases: &[],
        render: gos::help::print_gos_help_section,
    },
    HelpTopic {
        topic: "gosc",
        aliases: &[],
        render: gosc::help::print_gosc_help_section,
    },
    HelpTopic {
        topic: "gs",
        aliases: &[],
        render: gs::help::print_gs_help_section,
    },
    HelpTopic {
        topic: "head",
        aliases: &[],
        render: head::help::print_head_help_section,
    },
    HelpTopic {
        topic: "help",
        aliases: &[],
        render: help::help::print_help_help_section,
    },
    HelpTopic {
        topic: "http",
        aliases: &[],
        render: http::help::print_http_help_section,
    },
    HelpTopic {
        topic: "igcare",
        aliases: &[],
        render: igcare::help::print_igcare_help_section,
    },
    HelpTopic {
        topic: "igcared",
        aliases: &[],
        render: igcared::help::print_igcared_help_section,
    },
    HelpTopic {
        topic: "ignore",
        aliases: &[],
        render: ignore::help::print_ignore_help_section,
    },
    HelpTopic {
        topic: "image",
        aliases: &["img"],
        render: image::help::print_image_help_section,
    },
    HelpTopic {
        topic: "init",
        aliases: &[],
        render: init::help::print_init_help_section,
    },
    HelpTopic {
        topic: "ipcfg",
        aliases: &[],
        render: ipcfg::help::print_ipcfg_help_section,
    },
    HelpTopic {
        topic: "line",
        aliases: &[],
        render: line::help::print_line_help_section,
    },
    HelpTopic {
        topic: "local",
        aliases: &[],
        render: local::help::print_local_help_section,
    },
    HelpTopic {
        topic: "locate",
        aliases: &[],
        render: locate::help::print_locate_help_section,
    },
    HelpTopic {
        topic: "math",
        aliases: &[],
        render: math::help::print_math_help_section,
    },
    HelpTopic {
        topic: "mkd",
        aliases: &[],
        render: mkd::help::print_mkd_help_section,
    },
    HelpTopic {
        topic: "mkf",
        aliases: &[],
        render: mkf::help::print_mkf_help_section,
    },
    HelpTopic {
        topic: "mode",
        aliases: &[],
        render: mode::help::print_mode_help_section,
    },
    HelpTopic {
        topic: "mv",
        aliases: &["move"],
        render: mv::help::print_mv_help_section,
    },
    HelpTopic {
        topic: "ndb",
        aliases: &[],
        render: ndb::help::print_ndb_help_section,
    },
    HelpTopic {
        topic: "ne",
        aliases: &["ntceditor"],
        render: ne::help::print_ne_help_section,
    },
    HelpTopic {
        topic: "nms",
        aliases: &[],
        render: nms::help::print_nms_help_section,
    },
    HelpTopic {
        topic: "nprintf",
        aliases: &[],
        render: nprintf::help::print_nprintf_help_section,
    },
    HelpTopic {
        topic: "nsheet",
        aliases: &[],
        render: nsheet::help::print_nsheet_help_section,
    },
    HelpTopic {
        topic: "nssh",
        aliases: &[],
        render: nssh::help::print_nssh_help_section,
    },
    HelpTopic {
        topic: "ntpkg",
        aliases: &[],
        render: ntpkg::help::print_ntpkg_help_section,
    },
    HelpTopic {
        topic: "ntpx",
        aliases: &[],
        render: ntpx::help::print_ntpx_help_section,
    },
    HelpTopic {
        topic: "opencg",
        aliases: &["editcfg", "editcg"],
        render: opencg::help::print_opencg_help_section,
    },
    HelpTopic {
        topic: "pathcp",
        aliases: &[],
        render: pathcp::help::print_pathcp_help_section,
    },
    HelpTopic {
        topic: "ping",
        aliases: &[],
        render: ping::help::print_ping_help_section,
    },
    HelpTopic {
        topic: "pipe",
        aliases: &["pipes", "redirect", "redirection", "pipeline", "operator"],
        render: betw::help::print_pipe_help_section,
    },
    HelpTopic {
        topic: "py_activate",
        aliases: &[],
        render: py_activate::help::print_py_activate_help_section,
    },
    HelpTopic {
        topic: "py_deactivate",
        aliases: &[],
        render: py_deactivate::help::print_py_deactivate_help_section,
    },
    HelpTopic {
        topic: "qr",
        aliases: &[],
        render: qr::help::print_qr_help_section,
    },
    HelpTopic {
        topic: "qsc",
        aliases: &[],
        render: qsc::help::print_qsc_help_section,
    },
    HelpTopic {
        topic: "qur",
        aliases: &[],
        render: qur::help::print_qur_help_section,
    },
    HelpTopic {
        topic: "ral",
        aliases: &[],
        render: ral::help::print_ral_help_section,
    },
    HelpTopic {
        topic: "ran",
        aliases: &[],
        render: ran::help::print_ran_help_section,
    },
    HelpTopic {
        topic: "report",
        aliases: &[],
        render: report::help::print_report_help_section,
    },
    HelpTopic {
        topic: "resetcg",
        aliases: &["reset-config"],
        render: resetcg::help::print_resetcg_help_section,
    },
    HelpTopic {
        topic: "restorecg",
        aliases: &["restore-config"],
        render: restorecg::help::print_restorecg_help_section,
    },
    HelpTopic {
        topic: "rev",
        aliases: &[],
        render: rev::help::print_rev_help_section,
    },
    HelpTopic {
        topic: "rmd",
        aliases: &[],
        render: rmd::help::print_rmd_help_section,
    },
    HelpTopic {
        topic: "rmf",
        aliases: &[],
        render: rmf::help::print_rmf_help_section,
    },
    HelpTopic {
        topic: "rnmd",
        aliases: &[],
        render: rnmd::help::print_rnmd_help_section,
    },
    HelpTopic {
        topic: "rnmf",
        aliases: &[],
        render: rnmf::help::print_rnmf_help_section,
    },
    HelpTopic {
        topic: "run",
        aliases: &["r"],
        render: run::help::print_run_help_section,
    },
    HelpTopic {
        topic: "say",
        aliases: &[],
        render: say::help::print_say_help_section,
    },
    HelpTopic {
        topic: "scan",
        aliases: &[],
        render: scan::help::print_scan_help_section,
    },
    HelpTopic {
        topic: "scr",
        aliases: &["script"],
        render: scr::help::print_scr_help_section,
    },
    HelpTopic {
        topic: "serve",
        aliases: &[],
        render: serve::help::print_serve_help_section,
    },
    HelpTopic {
        topic: "showcg",
        aliases: &[],
        render: showcg::help::print_showcg_help_section,
    },
    HelpTopic {
        topic: "showenv",
        aliases: &[],
        render: showenv::help::print_showenv_help_section,
    },
    HelpTopic {
        topic: "size",
        aliases: &[],
        render: size::help::print_size_help_section,
    },
    HelpTopic {
        topic: "skill",
        aliases: &[],
        render: skill::help::print_skill_help_section,
    },
    HelpTopic {
        topic: "smtp",
        aliases: &[],
        render: smtp::help::print_smtp_help_section,
    },
    HelpTopic {
        topic: "sort",
        aliases: &[],
        render: sort::help::print_sort_help_section,
    },
    HelpTopic {
        topic: "spec",
        aliases: &[],
        render: spec::help::print_spec_help_section,
    },
    HelpTopic {
        topic: "sys",
        aliases: &["top"],
        render: sys::help::print_sys_help_section,
    },
    HelpTopic {
        topic: "syspath",
        aliases: &[],
        render: syspath::help::print_syspath_help_section,
    },
    HelpTopic {
        topic: "tail",
        aliases: &[],
        render: tail::help::print_tail_help_section,
    },
    HelpTopic {
        topic: "theme",
        aliases: &[],
        render: theme::help::print_theme_help_section,
    },
    HelpTopic {
        topic: "time",
        aliases: &[],
        render: time::help::print_time_help_section,
    },
    HelpTopic {
        topic: "tl",
        aliases: &[],
        render: tl::help::print_tl_help_section,
    },
    HelpTopic {
        topic: "tmprt",
        aliases: &[],
        render: tmprt::help::print_tmprt_help_section,
    },
    HelpTopic {
        topic: "todo",
        aliases: &["td"],
        render: todo::help::print_todo_help_section,
    },
    HelpTopic {
        topic: "tp",
        aliases: &[],
        render: tp::help::print_tp_help_section,
    },
    HelpTopic {
        topic: "tpb",
        aliases: &[],
        render: tpb::help::print_tpb_help_section,
    },
    HelpTopic {
        topic: "trace",
        aliases: &["tracert", "traceroute"],
        render: trace::help::print_trace_help_section,
    },
    HelpTopic {
        topic: "trim",
        aliases: &[],
        render: trim::help::print_trim_help_section,
    },
    HelpTopic {
        topic: "tutorial",
        aliases: &["guide"],
        render: tutorial::help::print_tutorial_help_section,
    },
    HelpTopic {
        topic: "txtc",
        aliases: &[],
        render: txtc::help::print_txtc_help_section,
    },
    HelpTopic {
        topic: "txtf",
        aliases: &[],
        render: txtf::help::print_txtf_help_section,
    },
    HelpTopic {
        topic: "ui",
        aliases: &[],
        render: ui::help::print_ui_help_section,
    },
    HelpTopic {
        topic: "up",
        aliases: &[],
        render: up::help::print_up_help_section,
    },
    HelpTopic {
        topic: "usrpath",
        aliases: &[],
        render: usrpath::help::print_usrpath_help_section,
    },
    HelpTopic {
        topic: "version",
        aliases: &[],
        render: version::help::print_version_help_section,
    },
    HelpTopic {
        topic: "view",
        aliases: &[],
        render: view::help::print_view_help_section,
    },
    HelpTopic {
        topic: "viewd",
        aliases: &[],
        render: viewd::help::print_viewd_help_section,
    },
    HelpTopic {
        topic: "viewf",
        aliases: &[],
        render: viewf::help::print_viewf_help_section,
    },
    HelpTopic {
        topic: "views",
        aliases: &[],
        render: views::help::print_views_help_section,
    },
    HelpTopic {
        topic: "where",
        aliases: &[],
        render: r#where::help::print_where_help_section,
    },
    HelpTopic {
        topic: "wi",
        aliases: &[],
        render: wi::help::print_wi_help_section,
    },
];

pub(crate) static HELP_ORDER: &[&str] = &[
    "arc",
    "autoup",
    "back",
    "backup",
    "betw",
    "care",
    "cd",
    "clear",
    "clip",
    "config",
    "count",
    "cp",
    "cvcode",
    "deadlock",
    "decode",
    "deinit",
    "dl",
    "dns",
    "doctor",
    "ds",
    "encode",
    "exit",
    "explorer",
    "fake",
    "fgo",
    "fmt",
    "fs",
    "fsc",
    "git",
    "godrive",
    "gos",
    "gosc",
    "gs",
    "head",
    "help",
    "http",
    "igcare",
    "igcared",
    "ignore",
    "image",
    "init",
    "ipcfg",
    "line",
    "local",
    "locate",
    "math",
    "mkd",
    "mkf",
    "mode",
    "mv",
    "ndb",
    "ne",
    "nms",
    "nprintf",
    "nsheet",
    "nssh",
    "ntpkg",
    "ntpx",
    "opencg",
    "pathcp",
    "ping",
    "pipe",
    "py_activate",
    "py_deactivate",
    "qr",
    "qsc",
    "qur",
    "ral",
    "ran",
    "report",
    "resetcg",
    "restorecg",
    "rev",
    "rmd",
    "rmf",
    "rnmd",
    "rnmf",
    "run",
    "say",
    "scan",
    "scr",
    "serve",
    "showcg",
    "showenv",
    "size",
    "skill",
    "smtp",
    "sort",
    "spec",
    "sys",
    "syspath",
    "tail",
    "theme",
    "time",
    "tl",
    "tmprt",
    "todo",
    "tp",
    "tpb",
    "trace",
    "trim",
    "tutorial",
    "txtc",
    "txtf",
    "ui",
    "up",
    "usrpath",
    "version",
    "view",
    "viewd",
    "viewf",
    "views",
    "where",
    "wi",
];

pub(crate) static COMPACT_HELP: &[CompactEntry] = &[
    ("arc", "arc", "help.compact.arc"),
    ("autoup", "autoup", "help.compact.autoup"),
    ("back", "back", "help.compact.back"),
    ("backup", "backup", "help.compact.backup"),
    ("betw", "betw", "help.compact.betw"),
    ("care", "care", "help.compact.care"),
    ("cd", "cd", "help.compact.cd"),
    ("clear", "clear", "help.compact.clear"),
    ("clip", "clip", "help.compact.clip"),
    ("config", "config", "help.compact.config"),
    ("count", "count", "help.compact.count"),
    ("cp", "cp", "help.compact.cp"),
    ("cvcode", "cvcode", "help.compact.cvcode"),
    ("deadlock", "deadlock", "help.compact.deadlock"),
    ("decode", "decode", "help.compact.decode"),
    ("deinit", "deinit", "help.compact.deinit"),
    ("dl", "dl", "help.compact.dl"),
    ("dns", "dns", "help.compact.dns"),
    ("doctor", "doctor", "help.compact.doctor"),
    ("ds", "ds", "help.compact.ds"),
    ("encode", "encode", "help.compact.encode"),
    ("exit", "exit", "help.compact.exit"),
    ("explorer", "explorer", "help.compact.explorer"),
    ("fake", "fake", "help.compact.fake"),
    ("fgo", "fgo", "help.compact.fgo"),
    ("fmt", "fmt", "help.compact.fmt"),
    ("fs", "fs", "help.compact.fs"),
    ("fsc", "fsc", "help.compact.fsc"),
    ("git", "git", "help.compact.git"),
    ("godrive", "godrive", "help.compact.godrive"),
    ("gos", "gos", "help.compact.gos"),
    ("gosc", "gosc", "help.compact.gosc"),
    ("gs", "gs", "help.compact.gs"),
    ("head", "head", "help.compact.head"),
    ("help", "help", "help.compact.help"),
    ("http", "http", "help.compact.http"),
    ("igcare", "igcare", "help.compact.igcare"),
    ("igcared", "igcared", "help.compact.igcared"),
    ("ignore", "ignore", "help.compact.ignore"),
    ("image", "image", "help.compact.image"),
    ("init", "init", "help.compact.init"),
    ("ipcfg", "ipcfg", "help.compact.ipcfg"),
    ("line", "line", "help.compact.line"),
    ("local", "local", "help.compact.local"),
    ("locate", "locate", "help.compact.locate"),
    ("math", "math", "help.compact.math"),
    ("mkd", "mkd", "help.compact.mkd"),
    ("mkf", "mkf", "help.compact.mkf"),
    ("mode", "mode", "help.compact.mode"),
    ("mv", "mv", "help.compact.mv"),
    ("ndb", "ndb", "help.compact.ndb"),
    ("ne", "ne", "help.compact.ne"),
    ("nms", "nms", "help.compact.nms"),
    ("nprintf", "nprintf", "help.compact.nprintf"),
    ("nsheet", "nsheet", "help.compact.nsheet"),
    ("nssh", "nssh", "help.compact.nssh"),
    ("ntpkg", "ntpkg", "help.compact.ntpkg"),
    ("ntpx", "ntpx", "help.compact.ntpx"),
    ("opencg", "opencg", "help.compact.opencg"),
    ("pathcp", "pathcp", "help.compact.pathcp"),
    ("ping", "ping", "help.compact.ping"),
    ("pipe", "pipe", "help.compact.pipe"),
    ("py_activate", "py_activate", "help.compact.py_activate"),
    (
        "py_deactivate",
        "py_deactivate",
        "help.compact.py_deactivate",
    ),
    ("qr", "qr", "help.compact.qr"),
    ("qsc", "qsc", "help.compact.qsc"),
    ("qur", "qur", "help.compact.qur"),
    ("ral", "ral", "help.compact.ral"),
    ("ran", "ran", "help.compact.ran"),
    ("report", "report", "help.compact.report"),
    ("resetcg", "resetcg", "help.compact.resetcg"),
    ("restorecg", "restorecg", "help.compact.restorecg"),
    ("rev", "rev", "help.compact.rev"),
    ("rmd", "rmd", "help.compact.rmd"),
    ("rmf", "rmf", "help.compact.rmf"),
    ("rnmd", "rnmd", "help.compact.rnmd"),
    ("rnmf", "rnmf", "help.compact.rnmf"),
    ("run", "run", "help.compact.run"),
    ("say", "say", "help.compact.say"),
    ("scan", "scan", "help.compact.scan"),
    ("scr", "scr", "help.compact.scr"),
    ("serve", "serve", "help.compact.serve"),
    ("showcg", "showcg", "help.compact.showcg"),
    ("showenv", "showenv", "help.compact.showenv"),
    ("size", "size", "help.compact.size"),
    ("skill", "skill", "help.compact.skill"),
    ("smtp", "smtp", "help.compact.smtp"),
    ("sort", "sort", "help.compact.sort"),
    ("spec", "spec", "help.compact.spec"),
    ("sys", "sys", "help.compact.sys"),
    ("syspath", "syspath", "help.compact.syspath"),
    ("tail", "tail", "help.compact.tail"),
    ("theme", "theme", "help.compact.theme"),
    ("time", "time", "help.compact.time"),
    ("tl", "tl", "help.compact.tl"),
    ("tmprt", "tmprt", "help.compact.tmprt"),
    ("todo", "todo", "help.compact.todo"),
    ("tp", "tp", "help.compact.tp"),
    ("tpb", "tpb", "help.compact.tpb"),
    ("trace", "trace", "help.compact.trace"),
    ("trim", "trim", "help.compact.trim"),
    ("tutorial", "tutorial", "help.compact.tutorial"),
    ("txtc", "txtc", "help.compact.txtc"),
    ("txtf", "txtf", "help.compact.txtf"),
    ("ui", "ui", "help.compact.ui"),
    ("up", "up", "help.compact.up"),
    ("usrpath", "usrpath", "help.compact.usrpath"),
    ("version", "version", "help.compact.version"),
    ("view", "view", "help.compact.view"),
    ("viewd", "viewd", "help.compact.viewd"),
    ("viewf", "viewf", "help.compact.viewf"),
    ("views", "views", "help.compact.views"),
    ("where", "where", "help.compact.where"),
    ("wi", "wi", "help.compact.wi"),
];

/// Command roots whose contextual help is a command-specific handler rather
/// than the generic completion-driven engine.
pub(crate) static CONTEXTUAL_HELP: &[ContextualHelp] = &[
    ContextualHelp {
        root: "cd",
        render: cd::help::print_cd_help,
    },
    ContextualHelp {
        root: "ignore",
        render: ignore::help::print_ignore_care_help,
    },
    ContextualHelp {
        root: "care",
        render: ignore::help::print_ignore_care_help,
    },
    ContextualHelp {
        root: "tp",
        render: tp::help::print_tp_context_help,
    },
    ContextualHelp {
        root: "arc",
        render: arc::help::print_arc_context_help,
    },
    ContextualHelp {
        root: "scan",
        render: scan::help::print_scan_context_help,
    },
    ContextualHelp {
        root: "qur",
        render: qur::help::print_qur_context_help,
    },
    ContextualHelp {
        root: "time",
        render: time::help::print_time_context_help,
    },
];

#[cfg(test)]
mod tests {
    use super::super::support::file_ops::{tp_pair_config, tp_pair_fixture};
    use super::{mkd::cmd_mkd, mkf::cmd_mkf, rmd::cmd_rmd, rmf::cmd_rmf};
    use crate::navigator::Navigator;

    #[test]
    fn mkf_mkd_rmf_rmd_accept_teleport_savepoints() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            // mkd / mkf create inside the savepoint.
            assert!(cmd_mkd("@tp2/build", &mut nav).is_ok());
            assert!(root.join("tp2").join("build").is_dir());
            assert!(cmd_mkf("@tp2/build/notes.md", &mut nav).is_ok());
            assert!(root.join("tp2").join("build").join("notes.md").is_file());
            // rmf removes a savepoint file and expands a `*` pattern there.
            std::fs::write(root.join("tp2").join("build").join("a.log"), "a").unwrap();
            assert!(cmd_rmf("@tp2/build/*.log", &mut nav).is_ok());
            assert!(!root.join("tp2").join("build").join("a.log").exists());
            assert!(cmd_rmf("@tp2/build/notes.md", &mut nav).is_ok());
            assert!(!root.join("tp2").join("build").join("notes.md").exists());
            // rmd removes the directory inside the savepoint.
            assert!(cmd_rmd("@tp2/build", &mut nav).is_ok());
            assert!(!root.join("tp2").join("build").exists());
        });
    }
}
