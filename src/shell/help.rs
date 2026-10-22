use colored::*;

use crate::t;

fn section_header(key: &str) {
    println!();
    println!("{}", t!(key).cyan().bold());
}

fn usage(key: &str) {
    let val = t!(key);
    if let Some(pos) = val.find(" ## ") {
        println!("  {:<38} {}", &val[..pos], &val[pos + 4..]);
    } else {
        println!("  {}", val);
    }
}

fn example_row(key: &str) {
    let val = t!(key);
    if let Some(pos) = val.find(" ## ") {
        println!("    {:<38} # {}", &val[..pos], &val[pos + 4..]);
    } else {
        println!("    {}", val);
    }
}

pub(super) fn print_interactive_help() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!(
        "║                     ntc {} - Interactive Help                 ║",
        env!("CARGO_PKG_VERSION")
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");

    print_nav_help();
    print_config_help();
    print_editor_help();
    print_theme_help();
    print_teleport_help();
    print_ran_help();
    print_scr_help();
    print_alias_help();
    print_view_help();
    print_size_help();
    print_report_help();
    print_backup_help();
    print_search_help();
    print_igcare_help();
    print_network_help();
    print_tmprt_help();
    print_git_help();
    print_clipboard_help();
    print_qsc_help();
    print_todo_help();
    print_arc_help();
    print_scan_help();
    print_other_help();
    print_syscmd_help();
    print_pipe_help();
    print_fileops_help();
    print_tl_help();
    print_ntpkg_help();
    print_qr_help();
    print_pathman_help();
    print_autoup_help();
    print_mode_help();
    print_explorer_help();
    print_math_help();
    print_python_help();
    print_encdec_help();
    print_fmt_help();
    print_image_help();
    print_ndb_help();
    print_nsheet_help();
    print_nssh_help();
    print_nms_help();
    print_fake_help();
}

/// Print a single help section for `help <topic>` / `<cmd> -h`.
/// Returns false when the topic is unknown.
pub(super) fn print_topic_help(topic: &str) -> bool {
    match topic.trim().to_lowercase().as_str() {
        "nav" | "cd" | "back" | "b" | "where" | "gos" | "gosc" | "godrive" | "god" | "fgo" => {
            print_nav_help()
        }
        "config" | "watch" | "w" | "wi" => print_config_help(),
        "editor" | "ne" | "ntceditor" => print_editor_help(),
        "theme" => print_theme_help(),
        "teleport" | "tp" | "tpb" => print_teleport_help(),
        "ran" => print_ran_help(),
        "scr" | "script" => print_scr_help(),
        "alias" | "ral" | "run" | "r" => print_alias_help(),
        "view" | "viewf" | "viewd" | "txtc" | "txtf" => print_view_help(),
        "size" => print_size_help(),
        "report" => print_report_help(),
        "backup" => print_backup_help(),
        "search" | "gs" | "fs" | "ds" | "locate" | "fsc" | "qur" => print_search_help(),
        "igcare" | "ignore" | "care" | "igcared" => print_igcare_help(),
        "network" | "http" | "dl" | "up" | "ping" | "dns" | "smtp" | "serve" | "ipcfg" => {
            print_network_help()
        }
        "tmprt" => print_tmprt_help(),
        "git" => print_git_help(),
        "clipboard" | "clip" => print_clipboard_help(),
        "qsc" => print_qsc_help(),
        "todo" | "td" => print_todo_help(),
        "arc" => print_arc_help(),
        "scan" => print_scan_help(),
        "other" | "clear" | "version" | "say" | "print" | "tutorial" | "ui" | "exit" | "quit"
        | "esc" | "help" => print_other_help(),
        "syscmd" | "sys" | "top" => print_syscmd_help(),
        "pipe" | "pipes" | "redirect" | "redirection" | "pipeline" | "operator" => {
            print_pipe_help()
        }
        "fileops" | "mkf" | "mkd" | "rmf" | "rmd" | "rnmf" | "rnmd" | "mv" | "move" | "cp" => {
            print_fileops_help()
        }
        "tl" => print_tl_help(),
        "ntpkg" | "ntpx" => print_ntpkg_help(),
        "qr" => print_qr_help(),
        "pathman" | "usrpath" | "syspath" => print_pathman_help(),
        "autoup" => print_autoup_help(),
        "mode" => print_mode_help(),
        "explorer" | "nfe" => print_explorer_help(),
        "math" => print_math_help(),
        "python" | "py_activate" | "py_deactivate" | "showenv" => print_python_help(),
        "encdec" | "encode" | "decode" | "cvcode" => print_encdec_help(),
        "fmt" => print_fmt_help(),
        "image" | "img" => print_image_help(),
        "ndb" => print_ndb_help(),
        "nsheet" => print_nsheet_help(),
        "nssh" => print_nssh_help(),
        "nms" => print_nms_help(),
        "fake" => print_fake_help(),
        _ => return false,
    }
    true
}

fn print_nav_help() {
    section_header("help.nav.title");
    usage("help.nav.cd");
    usage("help.nav.cd_tp");
    usage("help.nav.gos");
    usage("help.nav.gosc");
    usage("help.nav.godrive");
    usage("help.nav.back");
    usage("help.nav.fgo");
    usage("help.nav.where");
    println!("{}", t!("help.examples_header").green());
    example_row("help.nav.example1");
    example_row("help.nav.example2");
    example_row("help.nav.example3");
    example_row("help.nav.example4");
    example_row("help.nav.example5");
}

fn print_config_help() {
    section_header("help.config.title");
    usage("help.config.output");
    usage("help.config.depth");
    usage("help.config.linenumbers");
    usage("help.config.color");
    usage("help.config.threads");
    usage("help.config.history");
    usage("help.config.autosuggest");
    usage("help.config.git");
    usage("help.config.dir_view");
    usage("help.config.watch");
    usage("help.config.watch_trigger");
    usage("help.config.watch_trigger_off");
    usage("help.config.watch_limit");
    usage("help.config.wi");
    usage("help.config.qsc");
    usage("help.config.qsc_off");
    usage("help.config.lang");
    usage("help.config.showcg");
    usage("help.config.opencg");
    usage("help.config.resetcg");
    usage("help.config.restorecg");
    usage("help.config.local_init");
    usage("help.config.local_deinit");
    usage("help.config.local_help");
    usage("help.config.showenv");
    usage("help.config.showenv_all");
    usage("help.config.showenv_d");
    println!("{}", t!("help.examples_header").green());
    example_row("help.config.example1");
    example_row("help.config.example2");
    example_row("help.config.example3");
    example_row("help.config.example4");
    example_row("help.config.example5");
}

fn print_editor_help() {
    section_header("help.editor.title");
    usage("help.editor.ne");
    usage("help.editor.ne_line");
    usage("help.editor.ne_welcome");
    usage("help.editor.ne_init");
    usage("help.editor.ne_super");
    usage("help.editor.ne_hyper");
    println!("{}", t!("help.examples_header").green());
    example_row("help.editor.example1");
    example_row("help.editor.example2");
    example_row("help.editor.example3");
    example_row("help.editor.example4");
}

fn print_theme_help() {
    section_header("help.theme.title");
    usage("help.theme.interactive");
    usage("help.theme.list");
    usage("help.theme.current");
    usage("help.theme.switch");
    usage("help.theme.add");
    usage("help.theme.rm");
    usage("help.theme.edit");
    usage("help.theme.where");
    usage("help.theme.info");
    usage("help.theme.export");
    usage("help.theme.import");
    usage("help.theme.rnm");
    usage("help.theme.reload");
    usage("help.theme.make");
    println!("{}", t!("help.examples_header").green());
    example_row("help.theme.example1");
    example_row("help.theme.example2");
    example_row("help.theme.example3");
    example_row("help.theme.example4");
}

fn print_teleport_help() {
    section_header("help.teleport.title");
    usage("help.teleport.menu");
    usage("help.teleport.add");
    usage("help.teleport.add_path");
    usage("help.teleport.jump");
    usage("help.teleport.jump_num");
    usage("help.teleport.list");
    usage("help.teleport.info");
    usage("help.teleport.rm");
    usage("help.teleport.rnm");
    usage("help.teleport.cls");
    usage("help.teleport.at");
    usage("help.teleport.tpb");
    usage("help.teleport.tpb_history");
    usage("help.teleport.tpb_clear");
    println!("{}", t!("help.examples_header").green());
    example_row("help.teleport.example1");
    example_row("help.teleport.example2");
    example_row("help.teleport.example3");
    example_row("help.teleport.example4");
    example_row("help.teleport.example5");
}

fn print_ran_help() {
    section_header("help.ran.title");
    usage("help.ran.run");
    usage("help.ran.target");
    usage("help.ran.init");
    usage("help.ran.deinit");
    usage("help.ran.list");
    usage("help.ran.help");
    println!("{}", t!("help.examples_header").green());
    example_row("help.ran.example1");
    example_row("help.ran.example2");
    example_row("help.ran.example3");
}

fn print_scr_help() {
    section_header("help.scr.title");
    usage("help.scr.run");
    usage("help.scr.continue");
    usage("help.scr.init");
    usage("help.scr.ls");
    usage("help.scr.help");
    println!("{}", t!("help.examples_header").green());
    example_row("help.scr.example1");
    example_row("help.scr.example2");
    example_row("help.scr.example3");
}

fn print_alias_help() {
    section_header("help.alias.title");
    usage("help.alias.add");
    usage("help.alias.add_param");
    usage("help.alias.add_defaults");
    usage("help.alias.add_array");
    usage("help.alias.add_rest");
    usage("help.alias.edit");
    usage("help.alias.rnm");
    usage("help.alias.rm");
    usage("help.alias.list");
    usage("help.alias.info");
    usage("help.alias.cls");
    usage("help.alias.export_all");
    usage("help.alias.export_select");
    usage("help.alias.import");
    usage("help.alias.make");
    usage("help.alias.hintn");
    usage("help.alias.run");
    usage("help.alias.run_direct");
    usage("help.alias.run_param");
    println!("{}", t!("help.examples_header").green());
    example_row("help.alias.example1");
    example_row("help.alias.example2");
    example_row("help.alias.example3");
    example_row("help.alias.example4");
    example_row("help.alias.example5");
    example_row("help.alias.example6");
    example_row("help.alias.example7");
}

fn print_scan_help() {
    section_header("help.scan.title");
    usage("help.scan.ext");
    usage("help.scan.files");
    usage("help.scan.dir");
    usage("help.scan.depth");
    usage("help.scan.size");
    usage("help.scan.care");
    usage("help.scan.port");
    usage("help.scan.ip");
    println!("{}", t!("help.examples_header").green());
    example_row("help.scan.example1");
    example_row("help.scan.example2");
    example_row("help.scan.example3");
    example_row("help.scan.example4");
    example_row("help.scan.example5");
    example_row("help.scan.example6");
    example_row("help.scan.example7");
}

fn print_view_help() {
    section_header("help.view.title");
    usage("help.view.tree");
    usage("help.view.size");
    usage("help.view.filesize");
    usage("help.view.dirsize");
    usage("help.view.sort");
    usage("help.view.head");
    usage("help.view.tail");
    usage("help.view.file_usage");
    usage("help.view.file_only");
    usage("help.view.dir_only");
    usage("help.view.ext");
    usage("help.view.cp");
    usage("help.view.depth");
    usage("help.view.care");
    usage("help.view.time");
    usage("help.view.path");
    println!("{}", t!("help.examples_header").green());
    example_row("help.view.example1");
    example_row("help.view.example2");
    example_row("help.view.example3");
    example_row("help.view.example4");
    example_row("help.view.example5");
    example_row("help.view.example6");
    example_row("help.view.example7");
    example_row("help.view.example8");
    example_row("help.view.example9");
    example_row("help.view.example10");
    example_row("help.view.example11");
    example_row("help.view.example12");

    section_header("help.viewf.title");
    usage("help.viewf.tree");
    usage("help.viewf.file_usage");
    usage("help.viewf.size");
    usage("help.viewf.sort");
    usage("help.viewf.ext");
    usage("help.viewf.care");
    usage("help.viewf.time");
    usage("help.viewf.copy");
    println!("{}", t!("help.examples_header").green());
    example_row("help.viewf.example1");
    example_row("help.viewf.example2");
    example_row("help.viewf.example3");
    example_row("help.viewf.example4");

    section_header("help.viewd.title");
    usage("help.viewd.usage");
    usage("help.viewd.depth");
    usage("help.viewd.size");
    usage("help.viewd.care");
    usage("help.viewd.sort");
    usage("help.viewd.time");
    usage("help.viewd.copy");
    println!("{}", t!("help.examples_header").green());
    example_row("help.viewd.example1");
    example_row("help.viewd.example2");
    example_row("help.viewd.example3");
    example_row("help.viewd.example4");
}

fn print_size_help() {
    section_header("help.size.title");
    usage("help.size.size");
    usage("help.size.path");
    usage("help.size.care");
    usage("help.size.time");
    println!("{}", t!("help.examples_header").green());
    example_row("help.size.example1");
    example_row("help.size.example2");
    example_row("help.size.example3");
    example_row("help.size.example4");
}

fn print_report_help() {
    section_header("help.report.title");
    usage("help.report.txt");
    usage("help.report.json");
    usage("help.report.md");
    usage("help.report.html");
    usage("help.report.pdf");
    usage("help.report.docx");
    usage("help.report.xlsx");
    usage("help.report.cp");
    usage("help.report.txtc");
    usage("help.report.txtf");
    println!("{}", t!("help.examples_header").green());
    example_row("help.report.example1");
    example_row("help.report.example2");
    example_row("help.report.example3");
    example_row("help.report.example4");
    example_row("help.report.txtf_example1");
    example_row("help.report.txtf_example2");
    example_row("help.report.txtf_example3");
    example_row("help.report.txtf_example4");
}

fn print_backup_help() {
    section_header("help.backup.title");
    usage("help.backup.create");
    usage("help.backup.list");
    usage("help.backup.restore");
    usage("help.backup.restore_choose");
    usage("help.backup.restore_deleted");
    usage("help.backup.undo");
    usage("help.backup.undo_cls");
    usage("help.backup.undo_force");
    usage("help.backup.clear");
    usage("help.backup.clear_force");
    usage("help.backup.recreate");
    usage("help.backup.diff");
    println!("{}", t!("help.examples_header").green());
    example_row("help.backup.example1");
    example_row("help.backup.example2");
    example_row("help.backup.example3");
    example_row("help.backup.example4");
}

fn print_search_help() {
    section_header("help.search.title");
    usage("help.search.gs");
    usage("help.search.gs_interactive");
    usage("help.search.qur");
    usage("help.search.fs");
    usage("help.search.ds");
    usage("help.search.locate");
    usage("help.search.fsc");
    usage("help.search.pipe");
    println!("{}", t!("help.examples_header").green());
    example_row("help.search.example1");
    example_row("help.search.example2");
    example_row("help.search.example3");
    example_row("help.search.example4");
    example_row("help.search.example5");
    example_row("help.search.example6");
    example_row("help.search.example7");
    example_row("help.search.example8");
}

fn print_igcare_help() {
    section_header("help.igcare.title");
    usage("help.igcare.ignore_dir");
    usage("help.igcare.ignore_ext");
    usage("help.igcare.ignore_file");
    usage("help.igcare.care_dir");
    usage("help.igcare.care_ext");
    usage("help.igcare.care_file");
    usage("help.igcare.show");
    usage("help.igcare.export_all");
    usage("help.igcare.export_select");
    usage("help.igcare.import");
    usage("help.igcare.make");
    println!("{}", t!("help.examples_header").green());
    example_row("help.igcare.example1");
    example_row("help.igcare.example2");
    example_row("help.igcare.example3");
    example_row("help.igcare.example4");
}

fn print_network_help() {
    section_header("help.network.title");
    usage("help.network.sys");
    usage("help.network.http");
    usage("help.network.dl");
    usage("help.network.up");
    usage("help.network.ping");
    usage("help.network.dns");
    usage("help.network.smtp");
    usage("help.network.spec");
    usage("help.network.serve");
    usage("help.network.ipcfg");
    println!("{}", t!("help.examples_header").green());
    example_row("help.network.example1");
    example_row("help.network.example2");
    example_row("help.network.example3");
    example_row("help.network.example4");
    example_row("help.network.example5");
    example_row("help.network.example6");
}

fn print_tmprt_help() {
    section_header("help.tmprt.title");
    usage("help.tmprt.com");
    usage("help.tmprt.weather");
    usage("help.tmprt.key");
    usage("help.tmprt.city");
    usage("help.tmprt.sensors");
    println!("{}", t!("help.examples_header").green());
    example_row("help.tmprt.example1");
    example_row("help.tmprt.example2");
    example_row("help.tmprt.example3");
    example_row("help.tmprt.example4");
}

fn print_git_help() {
    section_header("help.git.title");
    usage("help.git.status");
    usage("help.git.branch");
    usage("help.git.log");
    usage("help.git.diff");
    usage("help.git.passthrough");
    println!("{}", t!("help.examples_header").green());
    example_row("help.git.example1");
    example_row("help.git.example2");
    example_row("help.git.example3");
}

fn print_clipboard_help() {
    section_header("help.clipboard.title");
    usage("help.clipboard.show");
    usage("help.clipboard.list");
    usage("help.clipboard.select");
    usage("help.clipboard.clear");
    usage("help.clipboard.pathcp");
    println!("{}", t!("help.examples_header").green());
    example_row("help.clipboard.example1");
    example_row("help.clipboard.example2");
    example_row("help.clipboard.example3");
}

fn print_qsc_help() {
    section_header("help.qsc.title");
    usage("help.qsc.set");
    usage("help.qsc.set_cs");
    usage("help.qsc.off");
    usage("help.qsc.listen");
    usage("help.qsc.stop");
    usage("help.qsc.status");
    usage("help.qsc.startup");
    println!("{}", t!("help.examples_header").green());
    example_row("help.qsc.example1");
    example_row("help.qsc.example2");
    example_row("help.qsc.example3");
}

fn print_todo_help() {
    section_header("help.todo.title");
    usage("help.todo.show");
    usage("help.todo.info");
    usage("help.todo.add");
    usage("help.todo.add_at");
    usage("help.todo.done");
    usage("help.todo.undo");
    usage("help.todo.edit");
    usage("help.todo.rm");
    usage("help.todo.clear");
    println!("{}", t!("help.examples_header").green());
    example_row("help.todo.example1");
    example_row("help.todo.example2");
    example_row("help.todo.example3");
}

fn print_arc_help() {
    section_header("help.arc.title");
    usage("help.arc.create");
    usage("help.arc.extract");
    usage("help.arc.list");
    println!("{}", t!("help.examples_header").green());
    example_row("help.arc.example1");
    example_row("help.arc.example2");
    example_row("help.arc.example3");
}

fn print_other_help() {
    section_header("help.other.title");
    usage("help.other.init");
    usage("help.other.deinit");
    usage("help.other.clear");
    usage("help.other.version");
    usage("help.other.help");
    usage("help.other.say");
    usage("help.other.print");
    usage("help.other.tutorial");
    usage("help.other.ui");
    usage("help.other.exit");
    println!("{}", t!("help.examples_header").green());
    example_row("help.other.example1");
    example_row("help.other.example2");
    example_row("help.other.example3");
    example_row("help.other.example4");
}

fn print_syscmd_help() {
    section_header("help.syscmd.title");
    usage("help.syscmd.any");
    usage("help.syscmd.native");
    usage("help.syscmd.native2");
}

fn print_pipe_help() {
    section_header("help.pipe.title");
    usage("help.pipe.pipe");
    usage("help.pipe.and");
    usage("help.pipe.or");
    usage("help.pipe.out");
    usage("help.pipe.append");
    usage("help.pipe.in");
    usage("help.pipe.heredoc");
    usage("help.pipe.herestring");
    usage("help.pipe.head");
    usage("help.pipe.tail");
    usage("help.pipe.betw");
    usage("help.pipe.sort");
    usage("help.pipe.rev");
    usage("help.pipe.filters");
    usable_note("help.pipe.note");
    usable_note("help.pipe.hint");
    println!("{}", t!("help.examples_header").green());
    example_row("help.pipe.example1");
    example_row("help.pipe.example2");
    example_row("help.pipe.example3");
    example_row("help.pipe.example4");
    example_row("help.pipe.example5");
}

/// Print an indented note row (no `##` split).
fn usable_note(key: &str) {
    println!("  {}", t!(key));
}

fn print_fileops_help() {
    section_header("help.fileops.title");
    usage("help.fileops.mkf");
    usage("help.fileops.mkd");
    usage("help.fileops.rmf");
    usage("help.fileops.rmd");
    usage("help.fileops.rnmf");
    usage("help.fileops.rnmd");
    usage("help.fileops.mv");
    usage("help.fileops.cp");
    println!("{}", t!("help.examples_header").green());
    example_row("help.fileops.example1");
    example_row("help.fileops.example2");
    example_row("help.fileops.example3");
}

fn print_tl_help() {
    section_header("help.tl.title");
    usage("help.tl.file");
    usage("help.tl.ext");
    usage("help.tl.dir");
    usage("help.tl.sort");
    println!("{}", t!("help.examples_header").green());
    example_row("help.tl.example1");
    example_row("help.tl.example2");
    example_row("help.tl.example3");
    example_row("help.tl.example4");
}

fn print_ntpkg_help() {
    section_header("help.ntpkg.title");
    usage("help.ntpkg.install");
    usage("help.ntpkg.uninstall");
    usage("help.ntpkg.list");
    usage("help.ntpkg.use");
    usage("help.ntpkg.which");
    usage("help.ntpkg.search");
    usage("help.ntpkg.info");
    usage("help.ntpkg.available");
    usage("help.ntpkg.help");
    usage("help.ntpkg.ntpx");
    println!("{}", t!("help.examples_header").green());
    example_row("help.ntpkg.example1");
    example_row("help.ntpkg.example2");
    example_row("help.ntpkg.example3");
    example_row("help.ntpkg.example4");
    example_row("help.ntpkg.example5");
    example_row("help.ntpkg.example6");
    example_row("help.ntpkg.example7");
    example_row("help.ntpkg.example8");
    example_row("help.ntpkg.example9");
}

fn print_qr_help() {
    section_header("help.qr.title");
    usage("help.qr.encode");
    usage("help.qr.save");
    usage("help.qr.image");
    usage("help.qr.decode");
    println!("{}", t!("help.examples_header").green());
    example_row("help.qr.example1");
    example_row("help.qr.example2");
    example_row("help.qr.example4");
    example_row("help.qr.example3");
}

fn print_pathman_help() {
    section_header("help.pathman.title");
    usage("help.pathman.usrpath_show");
    usage("help.pathman.usrpath_add");
    usage("help.pathman.usrpath_add_cwd");
    usage("help.pathman.usrpath_rm");
    usage("help.pathman.usrpath_move");
    usage("help.pathman.syspath_show");
    usage("help.pathman.syspath_add");
    usage("help.pathman.syspath_add_cwd");
    usage("help.pathman.syspath_rm");
    usage("help.pathman.syspath_move");
    println!("{}", t!("help.examples_header").green());
    example_row("help.pathman.example1");
    example_row("help.pathman.example2");
    example_row("help.pathman.example3");
    example_row("help.pathman.example4");
    example_row("help.pathman.example5");
    example_row("help.pathman.example6");
}

fn print_autoup_help() {
    section_header("help.autoup.title");
    usage("help.autoup.list");
    usage("help.autoup.add");
    usage("help.autoup.rm");
    usage("help.autoup.disable");
    usage("help.autoup.enable");
    usage("help.autoup.sources");
    println!("{}", t!("help.examples_header").green());
    example_row("help.autoup.example1");
    example_row("help.autoup.example2");
    example_row("help.autoup.example3");
    example_row("help.autoup.example4");
}

fn print_mode_help() {
    section_header("help.mode.title");
    usage("help.mode.admin");
    usage("help.mode.normal");
    println!("{}", t!("help.examples_header").green());
    example_row("help.mode.example1");
}

fn print_explorer_help() {
    section_header("help.explorer.title");
    usage("help.explorer.launch");
    usage("help.explorer.hint1");
    usage("help.explorer.hint2");
    usage("help.explorer.hint3");
    usage("help.explorer.hint4");
    usage("help.explorer.hint5");
    println!("{}", t!("help.examples_header").green());
    example_row("help.explorer.example1");
    example_row("help.explorer.example2");
}

fn print_math_help() {
    section_header("help.math.title");
    usage("help.math.eval");
    usage("help.math.timer");
    usage("help.math.fun");
    usage("help.math.file");
    println!("{}", t!("help.examples_header").green());
    example_row("help.math.example1");
    example_row("help.math.example2");
    example_row("help.math.example3");
    example_row("help.math.example4");
    example_row("help.math.example5");
    example_row("help.math.example6");
}

fn print_python_help() {
    section_header("help.python.title");
    usage("help.python.activate");
    usage("help.python.activate_path");
    usage("help.python.deactivate");
    println!("{}", t!("help.examples_header").green());
    example_row("help.python.example1");
    example_row("help.python.example2");
    example_row("help.python.example3");
}

fn print_encdec_help() {
    section_header("help.encdec.title");
    usage("help.encdec.encode_b2");
    usage("help.encdec.encode_b10");
    usage("help.encdec.encode_b64");
    usage("help.encdec.encode_hex");
    usage("help.encdec.encode_url");
    usage("help.encdec.decode_b2");
    usage("help.encdec.decode_b10");
    usage("help.encdec.decode_b64");
    usage("help.encdec.decode_hex");
    usage("help.encdec.decode_url");
    usage("help.encdec.cvcode");
    println!("{}", t!("help.examples_header").green());
    example_row("help.encdec.example1");
    example_row("help.encdec.example2");
    example_row("help.encdec.example3");
    example_row("help.encdec.example4");
}

fn print_fmt_help() {
    section_header("help.fmt.title");
    usage("cmd.fmt.usage");
    println!("{}", t!("help.examples_header").green());
    example_row("fmt.help_example1");
    example_row("fmt.help_example2");
    example_row("fmt.help_example4");
    example_row("fmt.help_example5");
    example_row("fmt.help_example6");
    example_row("fmt.help_example7");
    example_row("fmt.help_example8");
    example_row("fmt.help_example9");
    example_row("fmt.help_example10");
}

fn print_image_help() {
    section_header("help.image.title");
    usage("help.image.interactive");
    usage("help.image.open");
    usage("help.image.display");
    usage("help.image.show");
    usage("help.image.sixel");
    usage("help.image.convert");
    usage("help.image.save");
    usage("help.image.ocr");
    usage("help.image.layout");
    println!("{}", t!("help.examples_header").green());
    example_row("help.image.example1");
    example_row("help.image.example2");
}

/// The `ndb` section of the interactive help.
pub(crate) fn print_ndb_help() {
    section_header("help.ndb.title");
    usage("help.ndb.launch");
    usage("help.ndb.root_help");
    usage("help.ndb.root_view");
    usage("help.ndb.root_interface");
    usage("help.ndb.root_sql");
    usage("help.ndb.root_exit");
    usage("help.ndb.db_viewf");
    usage("help.ndb.db_tables");
    usage("help.ndb.db_tablef");
    usage("help.ndb.db_schema");
    usage("help.ndb.db_sql");
    usage("help.ndb.db_write");
    usage("help.ndb.db_edit");
    usage("help.ndb.db_backup");
    usage("help.ndb.more");
    usage("help.ndb.indexes");
    usage("help.ndb.info");
    usage("help.ndb.dbal");
    usage("help.ndb.db_exit");
    usage("help.ndb.readonly");
    println!("{}", t!("help.examples_header").green());
    example_row("help.ndb.example1");
    example_row("help.ndb.example2");
}

pub(crate) fn print_nssh_help() {
    section_header("help.nssh.title");
    usage("help.nssh.add");
    usage("help.nssh.ssh");
    usage("help.nssh.cp");
    usage("help.nssh.scan");
    usage("help.nssh.manage");
    usage("help.nssh.keygen");
    println!("{}", t!("help.examples_header").green());
    example_row("help.nssh.example1");
    example_row("help.nssh.example2");
    example_row("help.nssh.example3");
}

pub(crate) fn print_nms_help() {
    section_header("help.nms.title");
    usage("help.nms.play");
    usage("help.nms.scan");
    usage("help.nms.long");
    usage("help.nms.keys");
    println!("{}", t!("help.examples_header").green());
    example_row("help.nms.example1");
    example_row("help.nms.example2");
    example_row("help.nms.example3");
    example_row("help.nms.example4");
}

pub(crate) fn print_nsheet_help() {
    section_header("help.nsheet.title");
    usage("help.nsheet.open");
    usage("help.nsheet.view");
    usage("help.nsheet.sort");
    usage("help.nsheet.filter");
    usage("help.nsheet.export");
    usage("help.nsheet.tui");
    println!("{}", t!("help.examples_header").green());
    example_row("help.nsheet.example1");
    example_row("help.nsheet.example2");
    example_row("help.nsheet.example3");
}

pub(crate) fn print_fake_help() {
    section_header("help.fake.title");
    usage("help.fake.gen");
    usage("help.fake.gen_options");
    usage("help.fake.gen_group");
    usage("help.fake.gen_unique");
    usage("help.fake.gen_locale");
    usage("help.fake.types");
    usage("help.fake.types_search");
    usage("help.fake.locales");
    usage("help.fake.group_add");
    usage("help.fake.group_edit");
    usage("help.fake.group_rm");
    usage("help.fake.group_rnm");
    usage("help.fake.group_ls");
    usage("help.fake.group_info");
    usage("help.fake.group_clear");
    usage("help.fake.group_export");
    usage("help.fake.group_import");
    usage("help.fake.group_ref");
    usage("help.fake.help");
    println!("{}", t!("help.examples_header").green());
    example_row("help.fake.example1");
    example_row("help.fake.example2");
    example_row("help.fake.example3");
    example_row("help.fake.example4");
}

pub(super) fn print_tp_help() {
    print_teleport_help();
}

/// Strip a trailing contextual-help token (`--help` / `-h`).
/// Returns the prefix before it, or None when no help token is present.
pub(crate) fn strip_help_suffix(input: &str) -> Option<String> {
    super::hinter::strip_help_suffix(input)
}

fn print_cr() {
    println!("{}", t!("help.context.cr"));
}

/// Cisco-style contextual help for `<prefix> --help`.
/// Returns true when the prefix is a known command and help was printed.
pub(crate) fn print_contextual_help(prefix_line: &str) -> bool {
    let trimmed = prefix_line.trim();
    if trimmed.is_empty() {
        return false;
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return false;
    }
    let lowers: Vec<String> = parts.iter().map(|s| s.to_lowercase()).collect();
    let root = lowers[0].as_str();

    match root {
        "cd" => print_cd_help(&parts, &lowers),
        "ignore" | "care" => print_ignore_care_help(&parts, &lowers, root),
        "tp" => print_tp_context_help(&parts, &lowers),
        "arc" => print_arc_context_help(&parts, &lowers),
        "scan" => print_scan_context_help(&parts, &lowers),
        "qur" => print_qur_context_help(&parts, &lowers),
        _ => print_generic_help(&parts, &lowers, root),
    }
}

fn print_cd_help(parts: &[&str], lowers: &[String]) -> bool {
    if parts.len() == 1 {
        println!("cd <path>");
        println!("cd to <tp-alias>");
        return true;
    }
    if parts.len() == 2 {
        if lowers[1] == "to" {
            let names = super::hinter::teleport_names_for_command(&["cd"]);
            if names.is_empty() {
                println!("cd to <tp-alias>");
            } else {
                let mut sorted = names;
                sorted.sort();
                for n in sorted {
                    println!("{}", n);
                }
            }
            return true;
        }
        // `cd <path>` already complete (path may contain spaces; full args
        // are treated as one path by cmd_cd).
        print_cr();
        return true;
    }
    // `cd to <name>` (or `cd <path...>`): complete, Enter runs it.
    print_cr();
    true
}

fn print_ignore_care_help(parts: &[&str], _lowers: &[String], root: &str) -> bool {
    if parts.len() == 1 {
        for s in super::hinter::subcommands_for(&[parts[0]]) {
            println!("{}", s);
        }
        return true;
    }
    if parts.len() == 2 {
        match parts[1].to_lowercase().as_str() {
            "dir" => println!("{} dir <dir_name>", root),
            "ext" => println!("{} ext <ext>", root),
            "file" => println!("{} file <file_name>", root),
            _ => {
                // Unknown subcommand: re-list valid choices (corrective).
                for s in super::hinter::subcommands_for(&[parts[0]]) {
                    println!("{}", s);
                }
            }
        }
        return true;
    }
    // Value already supplied (`ignore dir foo`, possibly comma-separated
    // with extra words): complete.
    print_cr();
    true
}

fn print_tp_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use super::hinter::{subcommands_for, teleport_names_for_command};
    if parts.len() == 1 {
        for s in subcommands_for(&[parts[0]]) {
            println!("{}", s);
        }
        return true;
    }
    let sub = lowers[1].as_str();
    if parts.len() == 2 {
        match sub {
            "jump" | "to" | "info" | "rm" => {
                let names = teleport_names_for_command(parts);
                if names.is_empty() {
                    println!("tp {} <tp-alias>", sub);
                } else {
                    let mut sorted = names;
                    sorted.sort();
                    for n in sorted {
                        println!("{}", n);
                    }
                }
            }
            "add" => println!("tp add <name> [<path>]"),
            "rnm" | "rename" => println!("tp rnm <old> to <new>"),
            "list" | "ls" | "cls" | "clear" | "help" => print_cr(),
            _ => {
                for s in subcommands_for(&[parts[0]]) {
                    println!("{}", s);
                }
            }
        }
        return true;
    }
    // len >= 3: value-taking subcommands are complete once a value is given.
    match sub {
        "jump" | "to" | "info" | "rm" | "add" => print_cr(),
        "rnm" | "rename" => {
            // `tp rnm <old> to <new>` needs 5 tokens; fewer means still
            // incomplete — show usage, otherwise <cr>.
            if parts.len() >= 5 {
                print_cr();
            } else {
                println!("tp rnm <old> to <new>");
            }
        }
        _ => print_cr(),
    }
    true
}

fn print_arc_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use super::hinter::subcommands_for;
    if parts.len() == 1 {
        for s in subcommands_for(&[parts[0]]) {
            println!("{}", s);
        }
        return true;
    }
    match lowers[1].as_str() {
        "create" | "c" | "mk" => {
            println!("arc create <out.zip|tar.gz|tgz|tar> <files...>");
            println!("  e.g. arc create backup.zip src Cargo.toml");
        }
        "list" | "ls" | "l" => println!("arc list <archive>"),
        "extract" | "x" | "unzip" => println!("arc extract <archive> [dest]"),
        _ => {
            // Unknown subcommand: re-list valid choices (corrective).
            for s in subcommands_for(&[parts[0]]) {
                println!("{}", s);
            }
        }
    }
    true
}

fn print_scan_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use super::hinter::subcommands_for;
    if parts.len() == 1 {
        for s in subcommands_for(&[parts[0]]) {
            println!("{}", s);
        }
        return true;
    }
    match lowers[1].as_str() {
        "ext" => println!("scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]"),
        "port" => println!("scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]"),
        "ip" => println!("scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]"),
        _ => {
            // Unknown subcommand: re-list valid choices (corrective).
            for s in subcommands_for(&[parts[0]]) {
                println!("{}", s);
            }
        }
    }
    true
}

fn print_qur_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use super::hinter::subcommands_for;
    // Bare `qur`: needs a file glob first.
    if parts.len() == 1 {
        println!("{}", t!("help.context.qur.needs_glob"));
        for s in subcommands_for(&[parts[0]]) {
            println!("{}", s);
        }
        return true;
    }
    let has = |flags: &[&str]| {
        lowers
            .iter()
            .any(|t| flags.contains(&t.as_str()) || t.starts_with("--limit="))
    };
    let has_select = has(&["-s", "--select"]);
    let has_replace = has(&["-rp", "--replace"]);
    let has_display = has(&["-dp", "--display"]);
    // Value-taking flag awaiting its value (`-s`, `-rp`, `-d`, `-l`) → show slot.
    if let Some(last) = lowers.last().map(|s| s.as_str()) {
        match last {
            "-s" | "--select" => {
                println!("{}", t!("help.context.qur.select_value"));
                return true;
            }
            "-rp" | "--replace" => {
                println!("{}", t!("help.context.qur.replace_value"));
                return true;
            }
            "-d" | "--depth" => {
                println!("{}", t!("help.context.qur.depth_value"));
                return true;
            }
            "-l" | "--limit" => {
                println!("{}", t!("help.context.qur.limit_value"));
                return true;
            }
            _ => {}
        }
    }
    if !has_select {
        // Glob given, no `-s` yet: next step is select or depth/limit.
        println!("{}", t!("help.context.qur.next_select"));
        return true;
    }
    if has_replace || has_display {
        // Select + display/replace present: only depth/limit (or Enter) remain.
        println!("{}", t!("help.context.qur.next_depth"));
        return true;
    }
    // Select present without display/replace: suggest both next steps.
    println!("{}", t!("help.context.qur.next_display_replace"));
    true
}

fn print_generic_help(parts: &[&str], lowers: &[String], _root: &str) -> bool {
    use super::hinter::{
        alias_names_for_command, dedup_sorted, ntpkg_names_for_command,
        ran_target_names_for_command, scr_scripts_for_command, subcommands_for,
        theme_names_for_command,
    };
    let root = lowers[0].as_str();

    // Known value-complete patterns: value already supplied -> <cr>.
    if is_generic_complete(parts, lowers) {
        print_cr();
        return true;
    }

    // Next-step candidates (named only; fs paths become <path> templates).
    let mut next = named_next_sanitized(parts);
    // `ran`/`scr` dynamic names are already inside named_next; keep lists
    // stable for tests.
    let _ = (
        &alias_names_for_command,
        &ntpkg_names_for_command,
        &ran_target_names_for_command,
        &scr_scripts_for_command,
        &theme_names_for_command,
        &dedup_sorted,
    );
    if !next.is_empty() {
        next.sort();
        next.dedup();
        for n in next {
            println!("{}", n);
        }
        return true;
    }

    if parts.len() == 1 {
        // Bare command with no further subcommands (e.g. `where`, `back`,
        // `gos`): complete, Enter runs it. Unknown roots fall through to
        // the caller (returns false) so system fallback still works.
        if is_known_root(root) {
            print_cr();
            return true;
        }
        return false;
    }

    // No next candidates at depth >= 2.
    // If the last token is a known subcommand of its parent, a value is
    // still missing -> show `prefix <value>`; otherwise a value was already
    // supplied -> <cr>.
    let parent = &parts[..parts.len() - 1];
    let parent_subs: Vec<String> = subcommands_for(parent)
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();
    let last_lower = parts[parts.len() - 1].to_lowercase();
    if parent_subs.contains(&last_lower) {
        println!("{} <value>", parts.join(" "));
        return true;
    }
    // If parent itself expected an enumerable value (alias/tp/theme/ran…),
    // the current token is that value -> complete.
    print_cr();
    true
}

/// Next candidates for help printing: named sources + subcommands, but
/// suppress repeated top-level subcommand lists at depth >= 2 (e.g. `tp
/// jump --help` must not re-list `add/jump/to…`).
fn named_next_sanitized(parts: &[&str]) -> Vec<String> {
    use super::hinter::{
        alias_names_for_command, ntpkg_names_for_command, ran_target_names_for_command,
        scr_scripts_for_command, subcommands_for, teleport_names_for_command,
        theme_names_for_command,
    };
    if parts.is_empty() {
        return vec![];
    }
    // `cd to` is handled by its own printer; keep generic free of cd noise.
    if parts.len() >= 2 && parts[0].to_lowercase() == "cd" {
        return vec![];
    }
    let mut out: Vec<String> = Vec::new();
    out.extend(teleport_names_for_command(parts));
    out.extend(alias_names_for_command(parts));
    out.extend(ran_target_names_for_command(parts));
    out.extend(scr_scripts_for_command(parts));
    out.extend(ntpkg_names_for_command(parts));
    out.extend(theme_names_for_command(parts));
    if parts.len() == 1 && matches!(parts[0].to_lowercase().as_str(), "godrive" | "god") {
        out.extend(
            crate::navigator::Navigator::list_drives()
                .into_iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>(),
        );
    }
    let cur = subcommands_for(parts);
    if parts.len() == 1 {
        out.extend(cur.into_iter().map(|s| s.to_string()));
    } else {
        // Keep deeper valid drill-downs (e.g. `ral export`, `config watch`,
        // `tl ext`), skip pure repetitions of the top-level list.
        let top = subcommands_for(&[parts[0]]);
        if cur != top {
            out.extend(cur.into_iter().map(|s| s.to_string()));
        }
    }
    out.sort();
    out.dedup();
    out
}

/// True when `parts` already contains a user value for a value-taking
/// subcommand, i.e. help should show `<cr>` instead of listing again.
fn is_generic_complete(parts: &[&str], lowers: &[String]) -> bool {
    if parts.len() < 3 {
        return false;
    }
    let root = lowers[0].as_str();
    let sub = lowers[1].as_str();
    match root {
        "ral" => match sub {
            "rm" | "remove" | "info" | "edit" => parts.len() >= 3,
            "rnm" | "rename" => parts.len() >= 5,
            _ => false,
        },
        "theme" => match sub {
            "info" | "edit" | "rm" | "remove" | "rnm" | "rename" | "export" => parts.len() >= 3,
            _ => false,
        },
        "tpb" => parts.len() >= 2,
        "ran" | "scr" | "script" => {
            // `ran init` etc. are subcommands; anything else with a value
            // is a target/script -> complete.
            !matches!(sub, "init" | "deinit" | "list" | "ls" | "help")
        }
        _ => false,
    }
}

fn is_known_root(root: &str) -> bool {
    matches!(
        root,
        "cd" | "godrive"
            | "god"
            | "back"
            | "b"
            | "view"
            | "viewf"
            | "viewd"
            | "ls"
            | "report"
            | "txtc"
            | "txtf"
            | "config"
            | "clear"
            | "version"
            | "where"
            | "gos"
            | "gosc"
            | "ral"
            | "ran"
            | "scr"
            | "script"
            | "run"
            | "r"
            | "showcg"
            | "help"
            | "exit"
            | "quit"
            | "ignore"
            | "care"
            | "igcared"
            | "igcare"
            | "size"
            | "tp"
            | "tpb"
            | "backup"
            | "gs"
            | "qur"
            | "wi"
            | "fs"
            | "ds"
            | "fgo"
            | "fsc"
            | "locate"
            | "theme"
            | "todo"
            | "td"
            | "arc"
            | "scan"
            | "tl"
            | "qr"
            | "mode"
            | "ne"
            | "ntceditor"
            | "sys"
            | "top"
            | "git"
            | "mkf"
            | "mkd"
            | "rmf"
            | "rmd"
            | "rnmf"
            | "rnmd"
            | "mv"
            | "move"
            | "cp"
            | "clip"
            | "qsc"
            | "encode"
            | "decode"
            | "cvcode"
            | "fmt"
            | "usrpath"
            | "syspath"
            | "autoup"
            | "say"
            | "print"
            | "tmprt"
            | "math"
            | "image"
            | "img"
            | "ntpkg"
            | "ntpx"
            | "explorer"
            | "nfe"
            | "local"
            | "init"
            | "deinit"
            | "showenv"
            | "py_activate"
            | "py_deactivate"
            | "http"
            | "dl"
            | "up"
            | "ping"
            | "dns"
            | "smtp"
            | "serve"
            | "spec"
            | "pathcp"
            | "tutorial"
            | "guide"
            | "ndb"
            | "nms"
            | "nsheet"
            | "nssh"
            | "fake"
            | "esc"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contextual_help_handles_issue_examples() {
        // These print to stdout during tests; assert they are handled.
        assert!(print_contextual_help("cd"));
        assert!(print_contextual_help("cd to"));
        assert!(print_contextual_help("cd to some_tp"));
        assert!(print_contextual_help("cd src"));
        assert!(print_contextual_help("ignore"));
        assert!(print_contextual_help("ignore dir"));
        assert!(print_contextual_help("ignore dir foo"));
        assert!(print_contextual_help("care ext"));
        assert!(print_contextual_help("tp"));
        assert!(print_contextual_help("tp jump"));
        assert!(print_contextual_help("tp jump foo"));
        assert!(print_contextual_help("arc"));
        assert!(print_contextual_help("arc create"));
        assert!(print_contextual_help("arc list"));
        assert!(print_contextual_help("arc extract"));
        assert!(print_contextual_help("scan"));
        assert!(print_contextual_help("scan ext"));
        assert!(print_contextual_help("qur"));
        assert!(print_contextual_help("qur *.*"));
        assert!(print_contextual_help("qur *.* -s foo"));
        assert!(print_contextual_help("qur *.* -s foo -dp"));
        assert!(print_contextual_help("where"));
        assert!(!print_contextual_help(""));
        assert!(!print_contextual_help("definitely_not_a_cmd_xyz"));
    }
}
