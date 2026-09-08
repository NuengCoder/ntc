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
    print_other_help();
    print_syscmd_help();
    print_fileops_help();
    print_tl_help();
    print_ntpkg_help();
    print_qr_help();
    print_pathman_help();
    print_autoup_help();
    print_mode_help();
    print_explorer_help();
    print_dino_help();
    print_math_help();
    print_python_help();
    print_encdec_help();
    print_fmt_help();
    print_image_help();
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
    usage("help.config.qsc");
    usage("help.config.qsc_off");
    usage("help.config.lang");
    usage("help.config.showcg");
    usage("help.config.opencg");
    usage("help.config.resetcg");
    usage("help.config.restorecg");
    usage("help.config.local_init");
    usage("help.config.local_init_all");
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
}

fn print_view_help() {
    section_header("help.view.title");
    usage("help.view.tree");
    usage("help.view.size");
    usage("help.view.filesize");
    usage("help.view.dirsize");
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
    usage("help.search.fs");
    usage("help.search.ds");
    usage("help.search.locate");
    usage("help.search.fsc");
    println!("{}", t!("help.examples_header").green());
    example_row("help.search.example1");
    example_row("help.search.example2");
    example_row("help.search.example3");
    example_row("help.search.example4");
    example_row("help.search.example5");
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
    usage("help.todo.rm");
    usage("help.todo.clear");
    println!("{}", t!("help.examples_header").green());
    example_row("help.todo.example1");
    example_row("help.todo.example2");
    example_row("help.todo.example3");
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

fn print_fileops_help() {
    section_header("help.fileops.title");
    usage("help.fileops.mkf");
    usage("help.fileops.mkd");
    usage("help.fileops.rmf");
    usage("help.fileops.rmd");
    usage("help.fileops.rnmf");
    usage("help.fileops.rnmd");
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

fn print_dino_help() {
    section_header("help.dino.title");
    usage("help.dino.play");
    usage("help.dino.hint");
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
    usage("help.encdec.encode_b64");
    usage("help.encdec.encode_hex");
    usage("help.encdec.encode_url");
    usage("help.encdec.decode_b2");
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
    usage("help.fmt.json");
    usage("help.fmt.yaml");
    usage("help.fmt.toml");
    usage("help.fmt.output");
    println!("{}", t!("help.examples_header").green());
    example_row("help.fmt.example1");
    example_row("help.fmt.example2");
}

fn print_image_help() {
    section_header("help.image.title");
    usage("help.image.interactive");
    usage("help.image.open");
    usage("help.image.convert");
    usage("help.image.save");
    usage("help.image.ocr");
    usage("help.image.layout");
    println!("{}", t!("help.examples_header").green());
    example_row("help.image.example1");
    example_row("help.image.example2");
}

pub(super) fn print_tp_help() {
    print_teleport_help();
}
