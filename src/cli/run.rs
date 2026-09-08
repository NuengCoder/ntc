use crate::cli::helpers::{preprocess_args, print_help, print_logo};
use crate::config::Config;
use crate::filetype::{is_supported_format, strip_unc_prefix, FormatConfig};
use crate::navigator::Navigator;
use crate::output::{cat_file, print_error, print_success, print_warning};
use crate::report::{generate_report, generate_report_to, ReportFormat};
use crate::shell::commands::execute_command;
use crate::t;
use anyhow::{bail, Result};
use clap::{Arg, ArgAction, Command};
use colored::*;
use std::io::Write;
use std::path::Path;

fn handle_teleport_at(raw_args: &[String]) -> Result<bool> {
    if raw_args.len() != 2 {
        return Ok(true);
    }
    let arg = &raw_args[1];
    if (!arg.starts_with('@') && !arg.starts_with('#')) || arg.len() <= 1 {
        return Ok(true);
    }
    let tp_name = &arg[1..];
    use crate::navigator::Navigator;
    use crate::teleport::TeleportManager;

    if let Some(path) = TeleportManager::get_path(tp_name) {
        let clean_path = path.to_string_lossy().to_string();
        let clean_path =
            Path::new(clean_path.strip_prefix(r"\\?\").unwrap_or(&clean_path)).to_path_buf();

        let mut nav = Navigator::new()?;
        match nav.go_to(&clean_path) {
            Ok(()) => {
                println!(
                    "{}",
                    t!("msg.teleport.success", tp_name, clean_path.display())
                );
                println!();
                print_logo();
                crate::shell::run_shell_with_nav(nav)?;
                Ok(false)
            }
            Err(e) => {
                print_error(&t!("msg.teleport.error", tp_name, e));
                Ok(false)
            }
        }
    } else {
        print_error(&t!("msg.teleport.not_found", tp_name));
        println!("{}", t!("msg.teleport.hint_list"));
        Ok(false)
    }
}

/// Resolve the `--path` value for `--size` / `--view`.
/// Returns `Ok(None)` when no path is given, `Ok(Some(p))` when resolved, and
/// `Err` (error already printed) when resolution fails.
fn resolve_cli_path(
    matches: &clap::ArgMatches,
    nav: &Navigator,
) -> Result<Option<std::path::PathBuf>> {
    let Some(spec) = matches.get_one::<String>("path") else {
        return Ok(None);
    };
    let spec = spec.trim();
    if spec.is_empty() {
        return Ok(None);
    }
    match crate::shell::helpers::resolve_target_spec(spec, nav) {
        Ok(p) => Ok(Some(p)),
        Err(_) => Err(anyhow::anyhow!("target resolution failed")),
    }
}

fn handle_simple_flags(matches: &clap::ArgMatches, known_flags: &[String]) -> Result<bool> {
    if matches.get_flag("version") {
        println!(
            "{}",
            t!("msg.version.format", env!("CARGO_PKG_VERSION"))
                .green()
                .bold()
        );
        return Ok(false);
    }

    if matches.get_flag("upgrade") {
        return crate::shell::commands::upgrade::cmd_upgrade();
    }

    if matches.get_flag("where_cli") {
        let exe = std::env::current_exe().unwrap_or_default();
        let cwd = std::env::current_dir().unwrap_or_default();
        println!("{}", t!("msg.where.executable", exe.display()));
        println!("{}", t!("msg.where.current_dir", cwd.display()));
        return Ok(false);
    }

    if matches.get_flag("clear") {
        print!("\x1b[2J\x1b[1;1H");
        let _ = std::io::stdout().flush();
        return Ok(false);
    }

    if matches.get_flag("list") {
        println!(
            "{}",
            t!("msg.list.title", env!("CARGO_PKG_VERSION"))
                .green()
                .bold()
        );
        for flag in known_flags {
            println!("{}", t!("msg.list.entry_format", flag));
        }
        return Ok(false);
    }

    if matches.get_flag("help_extra") {
        print_help();
        return Ok(false);
    }

    if matches.get_flag("showcg") {
        let w = 65;
        println!();
        println!("{}", t!("msg.showcg.box_top", "─".repeat(w)));
        println!("│{:^w$}│", t!("msg.showcg.title"), w = w);
        println!("{}", t!("msg.showcg.box_divider", "─".repeat(w)));
        println!(
            "│ {:<20} {:<42} │",
            t!("msg.showcg.label_output_path"),
            Config::global_get_output_path().display().to_string()
        );
        println!(
            "│ {:<20} {:<42} │",
            t!("msg.showcg.label_max_depth"),
            Config::global_get_max_depth().to_string()
        );
        println!(
            "│ {:<20} {:<42} │",
            t!("msg.showcg.label_line_numbers"),
            if Config::global_get_show_line_numbers() {
                t!("status.on")
            } else {
                t!("status.off")
            }
        );
        println!(
            "│ {:<20} {:<42} │",
            t!("msg.showcg.label_threads"),
            Config::global_get_num_threads().to_string()
        );
        println!(
            "│ {:<20} {:<42} │",
            t!("msg.showcg.label_history"),
            if Config::global_get_history_enabled() {
                t!("status.on")
            } else {
                t!("status.off")
            }
        );
        println!(
            "│ {:<20} {:<42} │",
            t!("msg.showcg.label_watcher"),
            if Config::global_get_file_watcher_enabled() {
                t!("status.on")
            } else {
                t!("status.off")
            }
        );
        println!(
            "│ {:<20} {:<42} │",
            t!("msg.showcg.label_color"),
            if Config::global_get_color_enabled() {
                t!("status.on")
            } else {
                t!("status.off")
            }
        );
        println!("{}", t!("msg.showcg.box_bottom", "─".repeat(w)));
        println!();
        return Ok(false);
    }

    if matches.get_flag("ignored") {
        let dirs = Config::get_ignored_dirs_combined();
        let fmt_cfg = FormatConfig::from_both();
        println!("{}", t!("msg.ignored.dirs", &format!("{:?}", dirs)));
        println!(
            "{}",
            t!(
                "msg.ignored.extensions",
                &format!("{:?}", fmt_cfg.ignored_extensions)
            )
        );
        println!(
            "{}",
            t!(
                "msg.ignored.extra_extensions",
                &format!("{:?}", fmt_cfg.extra_extensions)
            )
        );
        println!(
            "{}",
            t!("msg.ignored.files", &format!("{:?}", fmt_cfg.ignored_files))
        );
        println!(
            "{}",
            t!(
                "msg.ignored.extra_files",
                &format!("{:?}", fmt_cfg.extra_files)
            )
        );
        return Ok(false);
    }

    if matches.get_flag("size") {
        let show_view = matches.get_flag("view_cli");
        let show_time = matches.get_flag("time");
        let start = std::time::Instant::now();
        use crate::navigator::Navigator;
        let nav = Navigator::new()?;
        let target = match resolve_cli_path(&matches, &nav) {
            Ok(t) => t.unwrap_or_else(|| nav.current_path().to_path_buf()),
            Err(_) => return Ok(false),
        };
        let total = crate::explorer::calculate_dir_size(&target);
        println!("{}", t!("msg.size.path_label", strip_unc_prefix(&target)));
        println!("{}", "┌─────────────────────────────────────────┐");
        println!(
            "│ {} ",
            t!("msg.size.title").trim().trim_matches('│').trim()
        );
        println!(
            "│ {:<20} {:>20} ",
            t!("msg.size.bytes_label"),
            format!("{}", total)
        );
        println!(
            "│ {:<20} {:>20} ",
            t!("msg.size.human_label"),
            crate::explorer::human_readable_size(total)
        );
        if show_time {
            println!("├─────────────────────────────────────────┤");
            println!(
                "│ {:<20} {:>20} ",
                t!("msg.size.time_label"),
                crate::explorer::format_duration(start.elapsed())
            );
        }
        println!("{}", "└─────────────────────────────────────────┘");
        if show_view {
            println!();
            let mut tree = crate::explorer::generate_tree(
                &target.to_string_lossy(),
                Some(1),
                true,
                None,
            );
            crate::explorer::compute_tree_sizes(&mut tree, None, false);
            let tree_str =
                crate::explorer::format_tree_with_sizes(&tree, "", true, true, true, false, None);
            println!("{}", tree_str);
        }
        return Ok(false);
    }

    if matches.get_flag("view_cli") && !matches.get_flag("size") {
        let show_time = matches.get_flag("time");
        let start = std::time::Instant::now();
        use crate::navigator::Navigator;
        let nav = Navigator::new()?;
        let target = match resolve_cli_path(&matches, &nav) {
            Ok(t) => t.unwrap_or_else(|| nav.current_path().to_path_buf()),
            Err(_) => return Ok(false),
        };
        let tree = crate::explorer::generate_tree(
            &target.to_string_lossy(),
            Some(1),
            true,
            None,
        );
        let tree_str =
            crate::explorer::format_tree_with_sizes(&tree, "", true, false, false, false, None);
        if show_time {
            println!("{}", tree_str);
            println!("┌─────────────────────────────────────────┐");
            println!(
                "│ {:<20} {:>20} ",
                t!("msg.size.time_label"),
                crate::explorer::format_duration(start.elapsed())
            );
            println!("└─────────────────────────────────────────┘");
        } else {
            println!("{}", tree_str);
        }
        return Ok(false);
    }

    if let Some(text) = matches.get_one::<String>("say") {
        println!("{}", text.green());
        return Ok(false);
    }

    if matches.get_flag("dino") {
        crate::game::run()?;
        return Ok(false);
    }

    if let Some(exprs) = matches.get_many::<String>("math") {
        let input: Vec<&str> = exprs.map(|s| s.as_str()).collect();
        let joined = input.join(" ");
        crate::math::run(&joined)?;
        return Ok(false);
    }

    if let Some(shell) = matches.get_one::<String>("generate_completions") {
        crate::cli::helpers::generate_completions(shell);
        return Ok(false);
    }

    if matches.get_flag("lsp") {
        let log_path = matches.get_one::<String>("lsp_log").map(Path::new);
        crate::lsp::run_server_with_logger(log_path)?;
        return Ok(false);
    }

    // Handle image to document conversion
    if let Some(image_path) = matches.get_one::<String>("image") {
        use crate::image2doc::{convert_image, save_conversion, ConversionOptions, OutputFormat};
        use std::path::Path;

        let img_path = Path::new(image_path);
        if !img_path.exists() {
            print_error(&t!("msg.image.not_found", image_path));
            return Ok(false);
        }

        let fmt = matches
            .get_one::<String>("image-to")
            .and_then(|s| s.parse().ok())
            .unwrap_or(OutputFormat::Markdown);

        let output_path = matches.get_one::<String>("output").map(|s| {
            let p = Path::new(s);
            if p.extension().is_none() {
                p.with_extension(fmt.extension())
            } else {
                p.to_path_buf()
            }
        });

        let options = ConversionOptions {
            output_format: fmt,
            output_path: output_path.clone(),
            #[cfg(all(feature = "ocr", not(target_os = "android")))]
            extract_text: true,
            ..Default::default()
        };

        let result = convert_image(img_path, options)?;

        if let Some(out) = &output_path {
            save_conversion(&result, out)?;
            print_success(&t!("msg.image.converted", out.display()));
        } else {
            println!("{}", result);
        }

        return Ok(false);
    }

    Ok(true)
}

fn handle_watch_flag(matches: &clap::ArgMatches) -> Result<bool> {
    if let Some(val) = matches.get_one::<String>("watch") {
        if val.is_empty() {
            let enabled = Config::global_get_file_watcher_enabled();
            println!(
                "{}",
                t!("msg.watch.status", if enabled { "ON" } else { "OFF" })
            );
        } else {
            let upper = val.to_uppercase();
            if upper == "ON" {
                Config::global_set_file_watcher_enabled(true);
                print_success(&t!("msg.watch.on"));
            } else if upper == "OFF" {
                Config::global_set_file_watcher_enabled(false);
                print_warning(&t!("msg.watch.off"));
            } else {
                print_error(&t!("msg.watch.invalid"));
            }
        }
        return Ok(false);
    } else if matches.contains_id("watch") {
        let enabled = Config::global_get_file_watcher_enabled();
        println!(
            "{}",
            t!("msg.watch.status", if enabled { "ON" } else { "OFF" })
        );
        return Ok(false);
    }
    Ok(true)
}

fn handle_ignore_care(matches: &clap::ArgMatches) -> Result<bool> {
    if let Some(names) = matches.get_many::<String>("ignore") {
        for name in names {
            Config::global_add_ignored_dir(name);
            print_success(&t!("msg.ignore.dir_added", name));
        }
        return Ok(false);
    }

    if let Some(names) = matches.get_many::<String>("cared") {
        for name in names {
            Config::global_remove_ignored_dir(name);
            print_success(&t!("msg.ignore.dir_removed", name));
        }
        return Ok(false);
    }

    if let Some(exts) = matches.get_many::<String>("ignoref") {
        for ext in exts {
            Config::global_add_ignored_extension(ext);
            print_success(&t!("msg.ignore.ext_added", ext));
        }
        return Ok(false);
    }

    if let Some(exts) = matches.get_many::<String>("caref") {
        for ext in exts {
            Config::global_add_extra_supported_extension(ext);
            print_success(&t!("msg.ignore.ext_removed", ext));
        }
        return Ok(false);
    }

    if let Some(files) = matches.get_many::<String>("ignoren") {
        for file in files {
            Config::global_add_ignored_file(file);
            print_success(&t!("msg.ignore.file_added", file));
        }
        return Ok(false);
    }

    if let Some(files) = matches.get_many::<String>("caren") {
        for file in files {
            Config::global_add_extra_supported_file(file);
            print_success(&t!("msg.ignore.file_removed", file));
        }
        return Ok(false);
    }

    Ok(true)
}

fn handle_setters(matches: &clap::ArgMatches) -> Result<bool> {
    if let Some(val) = matches.get_one::<String>("setH") {
        if val.is_empty() {
            let enabled = Config::global_get_history_enabled();
            let path = Config::global_get_history_path();
            println!(
                "{}",
                t!("msg.setH.status", if enabled { "ON" } else { "OFF" })
            );
            match path {
                Some(p) => println!("{}", t!("msg.setH.path", p.display())),
                None => println!("{}", t!("msg.setH.path_default")),
            }
        } else {
            let upper = val.to_uppercase();
            if upper == "ON" {
                Config::global_set_history_enabled(true);
                print_success(&t!("msg.setH.on"));
            } else if upper == "OFF" {
                Config::global_set_history_enabled(false);
                print_warning(&t!("msg.setH.off"));
            } else if val == "default" {
                Config::global_set_history_path(None);
                print_success(&t!("msg.setH.path_reset"));
            } else {
                let p = Path::new(val);
                Config::global_set_history_path(Some(p.to_path_buf()));
                print_success(&t!("msg.setH.path_set", p.display()));
            }
        }
        return Ok(false);
    } else if matches.contains_id("setH") {
        let enabled = Config::global_get_history_enabled();
        let path = Config::global_get_history_path();
        println!(
            "{}",
            t!("msg.setH.status", if enabled { "ON" } else { "OFF" })
        );
        match path {
            Some(p) => println!("{}", t!("msg.setH.path", p.display())),
            None => println!("{}", t!("msg.setH.path_default")),
        }
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setO") {
        if val.is_empty() {
            println!(
                "{}",
                t!(
                    "msg.setO.status",
                    Config::global_get_output_path().display()
                )
            );
        } else {
            Config::global_set_output_path(Path::new(val));
            print_success(&t!("msg.setO.success", val));
        }
        return Ok(false);
    } else if matches.contains_id("setO") {
        println!(
            "{}",
            t!(
                "msg.setO.status",
                Config::global_get_output_path().display()
            )
        );
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setD") {
        if val.is_empty() {
            println!("{}", t!("msg.setD.status", Config::global_get_max_depth()));
        } else {
            match val.parse::<usize>() {
                Ok(depth) => {
                    if !(1..=20).contains(&depth) {
                        bail!(t!("msg.setD.error_out_of_range", val));
                    }
                    Config::global_set_max_depth(depth);
                    print_success(&t!("msg.setD.success", Config::global_get_max_depth()));
                }
                Err(_) => bail!(t!("msg.setD.error_parse", val)),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setD") {
        println!("{}", t!("msg.setD.status", Config::global_get_max_depth()));
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setL") {
        if val.is_empty() {
            let state = if Config::global_get_show_line_numbers() {
                "ON"
            } else {
                "OFF"
            };
            println!("{}", t!("msg.setL.status", state));
        } else {
            match Config::parse_on_off(val) {
                Some(state) => {
                    Config::global_set_show_line_numbers(state);
                    print_success(&t!("msg.setL.success", if state { "ON" } else { "OFF" }));
                }
                None => bail!(t!("msg.setL.error", val)),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setL") {
        let state = if Config::global_get_show_line_numbers() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setL.status", state));
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setT") {
        if val.is_empty() {
            println!(
                "{}",
                t!("msg.setT.status", Config::global_get_num_threads())
            );
        } else {
            match Config::parse_num_threads(val) {
                Some(threads) => {
                    if !(1..=64).contains(&threads) {
                        bail!(t!("msg.setT.error_out_of_range", val));
                    }
                    Config::global_set_num_threads(threads);
                    print_success(&t!("msg.setT.success", Config::global_get_num_threads()));
                }
                None => bail!(t!("msg.setT.error_parse", val)),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setT") {
        println!(
            "{}",
            t!("msg.setT.status", Config::global_get_num_threads())
        );
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setC") {
        if val.is_empty() {
            let state = if Config::global_get_color_enabled() {
                "ON"
            } else {
                "OFF"
            };
            println!("{}", t!("msg.setC.status", state));
        } else {
            match Config::parse_on_off(val) {
                Some(state) => {
                    Config::global_set_color_enabled(state);
                    print_success(&t!("msg.setC.success", if state { "ON" } else { "OFF" }));
                }
                None => bail!(t!("msg.setC.error", val)),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setC") {
        let state = if Config::global_get_color_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setC.status", state));
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setA") {
        if val.is_empty() {
            let state = if Config::global_get_autosuggest_enabled() {
                "ON"
            } else {
                "OFF"
            };
            println!("{}", t!("msg.setA.status", state));
        } else {
            match Config::parse_on_off(val) {
                Some(state) => {
                    Config::global_set_autosuggest_enabled(state);
                    if state {
                        print_success(&t!("msg.setA.on"));
                    } else {
                        print_warning(&t!("msg.setA.off"));
                    }
                }
                None => bail!(t!("msg.setA.error", val)),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setA") {
        let state = if Config::global_get_autosuggest_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setA.status", state));
        return Ok(false);
    }

    Ok(true)
}

fn handle_input(matches: &clap::ArgMatches) -> Result<bool> {
    if let Some(input_path) = matches.get_one::<String>("input") {
        let path = Path::new(input_path);
        let copy_to_clipboard = matches.get_flag("copy");

        if path.is_dir() {
            let output_file = matches.get_one::<String>("output");
            let format_str = matches
                .get_one::<String>("format")
                .map(|s| s.as_str())
                .unwrap_or("txt");

            let format = match format_str {
                "html" => ReportFormat::Html,
                "json" => ReportFormat::Json,
                "md" => ReportFormat::Md,
                "pdf" => ReportFormat::Pdf,
                "docx" => ReportFormat::Docx,
                "xlsx" => ReportFormat::Xlsx,
                _ => ReportFormat::Txt,
            };

            if copy_to_clipboard {
                if format.is_binary() {
                    print_warning(&t!(
                        "msg.report.binary_clipboard_warning",
                        format_str.to_uppercase()
                    ));
                } else {
                    let content = crate::report::generate_report_to_string(path, format)?;
                    crate::output::copy_to_clipboard(&content, format_str)?;
                    print_success(&t!(
                        "msg.report.copied_to_clipboard",
                        format_str.to_uppercase()
                    ));
                }
            } else if let Some(output) = output_file {
                let output = crate::output::build_output_path(output);
                generate_report_to(path, format, &output.to_string_lossy())?;
            } else {
                generate_report(path, format)?;
            }
        } else if path.is_file() {
            if is_supported_format(path) {
                let show_lines = Config::global_get_show_line_numbers();
                let output_file = matches.get_one::<String>("output");
                if let Some(output) = output_file {
                    let content = crate::output::cat_file_with_line_numbers(path, show_lines)?;
                    let output_path = crate::output::build_output_path(output);
                    crate::output::write_file(&output_path, &content)?;
                    print_success(&t!("msg.report.file_saved", output_path.display()));
                } else {
                    cat_file(path, show_lines)?;
                }
            } else {
                print_warning(&t!("msg.report.skipped_unsupported", input_path));
            }
        } else {
            bail!(t!("msg.report.path_not_found", input_path));
        }
        return Ok(false);
    }
    Ok(true)
}

fn handle_teleport_cmds(matches: &clap::ArgMatches) -> Result<bool> {
    if matches.get_flag("tp_list") {
        crate::teleport::TeleportManager::list()?;
        return Ok(false);
    }

    if let Some(name) = matches.get_one::<String>("tp_add") {
        let current_dir = std::env::current_dir()?;
        crate::teleport::TeleportManager::add(name, current_dir)?;
        return Ok(false);
    }

    if let Some(name) = matches.get_one::<String>("tp_rm") {
        crate::teleport::TeleportManager::remove_by_name(name)?;
        return Ok(false);
    }

    Ok(true)
}

fn handle_edit_init(matches: &clap::ArgMatches) -> Result<bool> {
    if let Some(path) = matches.get_one::<String>("init") {
        let p = std::path::Path::new(path);
        let created = crate::editor::init_file(p)?;
        crate::editor::edit_file(p)?;
        if created {
            eprintln!("{}", t!("msg.init.template_created", p.display()));
        }
        return Ok(false);
    }

    if let Some(path) = matches.get_one::<String>("edit") {
        crate::editor::edit_file(std::path::Path::new(path))?;
        return Ok(false);
    }

    Ok(true)
}

fn handle_hyper(matches: &clap::ArgMatches) -> Result<bool> {
    if let Some(name) = matches.get_one::<String>("hyper") {
        let lang = matches
            .get_one::<String>("lang")
            .map(|s| s.as_str())
            .unwrap_or("c");
        crate::editor::init_hyper_project(name, lang)?;
        return Ok(false);
    }
    Ok(true)
}

fn handle_ral_igcare(matches: &clap::ArgMatches) -> Result<bool> {
    if let Some(name) = matches.get_one::<String>("ral_export_all") {
        crate::shell::helpers::ral_export_all(name)?;
        return Ok(false);
    }

    if let Some(name) = matches.get_one::<String>("ral_export_select") {
        use crate::navigator::Navigator;
        let mut nav = Navigator::new()?;
        crate::shell::helpers::ral_export_select(&mut nav, name)?;
        return Ok(false);
    }

    if let Some(file) = matches.get_one::<String>("ral_import") {
        crate::shell::helpers::ral_import(file)?;
        return Ok(false);
    }

    if let Some(name) = matches.get_one::<String>("igcare_export_all") {
        crate::shell::helpers::igcare_export_all(name)?;
        return Ok(false);
    }

    if let Some(name) = matches.get_one::<String>("igcare_export_select") {
        crate::shell::helpers::igcare_export_select(name)?;
        return Ok(false);
    }

    if let Some(file) = matches.get_one::<String>("igcare_import") {
        crate::shell::helpers::igcare_import(file)?;
        return Ok(false);
    }

    Ok(true)
}

/// Parse command-line arguments and execute the appropriate action.
pub fn run_cli() -> Result<bool> {
    let raw_args: Vec<String> = std::env::args().collect();

    // --- Direct dispatch: `ntc qsc daemon` → enter hidden daemon loop without shell ---
    if raw_args.len() >= 3
        && raw_args[1].to_lowercase() == "qsc"
        && raw_args[2].to_lowercase() == "daemon"
    {
        crate::utils::qsc::run();
    }

    // --- Handle @teleport shortcut from command line ---
    if !handle_teleport_at(&raw_args)? {
        return Ok(false);
    }

    let args = preprocess_args(raw_args);

    let known_flags: Vec<String> = vec![
        t!("cli.known_flag.input"),
        t!("cli.known_flag.generate_completions"),
        t!("cli.known_flag.output"),
        t!("cli.known_flag.cp"),
        t!("cli.known_flag.setO"),
        t!("cli.known_flag.setD"),
        t!("cli.known_flag.setL"),
        t!("cli.known_flag.setC"),
        t!("cli.known_flag.setT"),
        t!("cli.known_flag.setH"),
        t!("cli.known_flag.showcg"),
        t!("cli.known_flag.watch"),
        t!("cli.known_flag.say"),
        t!("cli.known_flag.size"),
        t!("cli.known_flag.view"),
        t!("cli.known_flag.path"),
        t!("cli.known_flag.view_size"),
        t!("cli.known_flag.ignored"),
        t!("cli.known_flag.ignore"),
        t!("cli.known_flag.cared"),
        t!("cli.known_flag.ignoref"),
        t!("cli.known_flag.caref"),
        t!("cli.known_flag.ignoren"),
        t!("cli.known_flag.caren"),
        t!("cli.known_flag.clear"),
        t!("cli.known_flag.version"),
        t!("cli.known_flag.upgrade"),
        t!("cli.known_flag.where"),
        t!("cli.known_flag.ral_export_all"),
        t!("cli.known_flag.ral_export_select"),
        t!("cli.known_flag.ral_import"),
        t!("cli.known_flag.igcare_export_all"),
        t!("cli.known_flag.igcare_export_select"),
        t!("cli.known_flag.igcare_import"),
        t!("cli.known_flag.explorer"),
        t!("cli.known_flag.dino"),
        t!("cli.known_flag.math"),
        t!("cli.known_flag.list"),
        t!("cli.known_flag.help_flag"),
        t!("cli.known_flag.tp_add"),
        t!("cli.known_flag.tp_list"),
        t!("cli.known_flag.tp_rm"),
        t!("cli.known_flag.teleport_at"),
        t!("cli.known_flag.no_args"),
    ];

    let matches = Command::new("ntc")
        .disable_help_flag(true)
        .author("NuengCoder")
        .about(t!("cli.run.about"))
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("PATH")
                .help(t!("cli.run.help_input"))
                .num_args(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help(t!("cli.run.help_output"))
                .num_args(1),
        )
        .arg(
            Arg::new("setO")
                .long("setO")
                .value_name("PATH")
                .help(t!("cli.run.help_seto"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setD")
                .long("setD")
                .value_name("DEPTH")
                .help(t!("cli.run.help_setd"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setL")
                .long("setL")
                .value_name("STATE")
                .help(t!("cli.run.help_setl"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setT")
                .long("setT")
                .value_name("THREADS")
                .help(t!("cli.run.help_sett"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setC")
                .long("setC")
                .value_name("STATE")
                .help(t!("cli.run.help_setc"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setA")
                .long("setA")
                .value_name("STATE")
                .help(t!("cli.run.help_seta"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setH")
                .long("setH")
                .value_name("VALUE")
                .help(t!("cli.run.help_seth"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("showcg")
                .long("showcg")
                .help(t!("cli.run.help_showcg"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("watch")
                .long("watch")
                .value_name("STATE")
                .help(t!("cli.run.help_watch"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("say")
                .short('s')
                .long("say")
                .visible_alias("print")
                .value_name("TEXT")
                .help(t!("cli.run.help_say"))
                .num_args(1),
        )
        .arg(
            Arg::new("clear")
                .long("clear")
                .help(t!("cli.run.help_clear"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("version")
                .long("version")
                .help(t!("cli.run.help_version"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("upgrade")
                .long("upgrade")
                .help(t!("cli.run.help_upgrade"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .visible_alias("fun")
                .help(t!("cli.run.help_list"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("help_extra")
                .short('h')
                .long("help")
                .help(t!("cli.run.help_helpextra"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignored")
                .long("ignored")
                .help(t!("cli.run.help_ignored"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore")
                .long("ignore")
                .value_name("NAME")
                .help(t!("cli.run.help_ignore"))
                .num_args(1..),
        )
        .arg(
            Arg::new("cared")
                .long("cared")
                .value_name("NAME")
                .help(t!("cli.run.help_cared"))
                .num_args(1..),
        )
        .arg(
            Arg::new("ignoref")
                .long("ignoref")
                .value_name("EXT")
                .help(t!("cli.run.help_ignoref"))
                .num_args(1..),
        )
        .arg(
            Arg::new("caref")
                .long("caref")
                .value_name("EXT")
                .help(t!("cli.run.help_caref"))
                .num_args(1..),
        )
        .arg(
            Arg::new("ignoren")
                .long("ignoren")
                .value_name("FILE")
                .help(t!("cli.run.help_ignoren"))
                .num_args(1..),
        )
        .arg(
            Arg::new("caren")
                .long("caren")
                .value_name("FILE")
                .help(t!("cli.run.help_caren"))
                .num_args(1..),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help(t!("cli.run.help_path"))
                .num_args(1),
        )
        .arg(
            Arg::new("size")
                .long("size")
                .help(t!("cli.run.help_size"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("view_cli")
                .long("view")
                .help(t!("cli.run.help_view"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("time")
                .long("time")
                .short('t')
                .help(t!("cli.run.help_time"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("where_cli")
                .long("where")
                .help(t!("cli.run.help_where"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("tp_add")
                .long("tp-add")
                .value_name("NAME")
                .help(t!("cli.run.help_tp_add"))
                .num_args(1),
        )
        .arg(
            Arg::new("tp_list")
                .long("tp-list")
                .help(t!("cli.run.help_tp_list"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("tp_rm")
                .long("tp-rm")
                .value_name("NAME")
                .help(t!("cli.run.help_tp_rm"))
                .num_args(1),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help(t!("cli.run.help_format"))
                .num_args(1)
                .value_parser(["txt", "html", "json", "md", "pdf", "docx", "xlsx"]),
        )
        .arg(
            Arg::new("copy")
                .long("cp")
                .help(t!("cli.run.help_copy"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .value_name("FILE")
                .help(t!("cli.run.help_edit"))
                .num_args(1),
        )
        .arg(
            Arg::new("init")
                .long("init")
                .value_name("FILE")
                .help(t!("cli.run.help_init"))
                .num_args(1),
        )
        .arg(
            Arg::new("hyper")
                .long("hyper")
                .value_name("PROJECT")
                .help(t!("cli.run.help_hyper"))
                .num_args(1),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .value_name("LANG")
                .help(t!("cli.run.help_lang"))
                .num_args(1)
                .value_parser(["c"]),
        )
        .arg(
            Arg::new("ral_export_all")
                .long("ral-export-all")
                .value_name("NAME")
                .help(t!("cli.run.help_ral_export_all"))
                .num_args(1),
        )
        .arg(
            Arg::new("ral_export_select")
                .long("ral-export-select")
                .value_name("NAME")
                .help(t!("cli.run.help_ral_export_select"))
                .num_args(1),
        )
        .arg(
            Arg::new("ral_import")
                .long("ral-import")
                .value_name("FILE")
                .help(t!("cli.run.help_ral_import"))
                .num_args(1),
        )
        .arg(
            Arg::new("igcare_export_all")
                .long("igcare-export-all")
                .value_name("NAME")
                .help(t!("cli.run.help_igcare_export_all"))
                .num_args(1),
        )
        .arg(
            Arg::new("igcare_export_select")
                .long("igcare-export-select")
                .value_name("NAME")
                .help(t!("cli.run.help_igcare_export_select"))
                .num_args(1),
        )
        .arg(
            Arg::new("igcare_import")
                .long("igcare-import")
                .value_name("FILE")
                .help(t!("cli.run.help_igcare_import"))
                .num_args(1),
        )
        .arg(
            Arg::new("dino")
                .long("dino")
                .help(t!("cli.run.help_dino"))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("math")
                .long("math")
                .value_name("EXPR")
                .help(t!("cli.run.help_math"))
                .num_args(1..),
        )
        .arg(
            Arg::new("lsp")
                .long("lsp")
                .help(t!("cli.run.help_lsp"))
                .action(ArgAction::SetTrue)
                .hide(true),
        )
        .arg(
            Arg::new("lsp_log")
                .long("lsp-log")
                .value_name("FILE")
                .help(t!("cli.run.help_lsp_log"))
                .num_args(1)
                .hide(true),
        )
        .arg(
            Arg::new("explorer")
                .short('x')
                .long("explorer")
                .value_name("PATH")
                .help(t!("cli.run.help_explorer"))
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("image")
                .long("image")
                .value_name("IMAGE")
                .help(t!("cli.run.help_image"))
                .num_args(1),
        )
        .arg(
            Arg::new("image-to")
                .long("image-to")
                .value_name("FORMAT")
                .help(t!("cli.run.help_image_to"))
                .num_args(1)
                .value_parser(["json", "md", "markdown", "html", "txt", "text"])
                .hide(true),
        )
        .arg(
            Arg::new("generate_completions")
                .long("generate-completions")
                .value_name("SHELL")
                .help(t!("cli.run.help_generate_completions"))
                .num_args(1)
                .value_parser(["bash", "zsh", "fish", "powershell"]),
        )
        .try_get_matches_from(args);

    let matches = match matches {
        Ok(m) => m,
        Err(e) => {
            // Unknown args → dispatch directly via shell (e.g. "ntc pathcp")
            if e.kind() == clap::error::ErrorKind::UnknownArgument
                || e.kind() == clap::error::ErrorKind::InvalidSubcommand
            {
                let raw: Vec<String> = std::env::args().skip(1).collect();
                if !raw.is_empty() {
                    let mut nav = Navigator::new()?;
                    execute_command(&raw.join(" "), &mut nav, &None)?;
                    return Ok(false);
                }
                return Ok(true);
            }
            return Err(e.into());
        }
    };

    // Dispatch to handler functions in priority order
    if !handle_simple_flags(&matches, &known_flags)? {
        return Ok(false);
    }
    if !handle_watch_flag(&matches)? {
        return Ok(false);
    }
    if !handle_ignore_care(&matches)? {
        return Ok(false);
    }
    if !handle_setters(&matches)? {
        return Ok(false);
    }
    if !handle_input(&matches)? {
        return Ok(false);
    }
    if !handle_teleport_cmds(&matches)? {
        return Ok(false);
    }
    if !handle_hyper(&matches)? {
        return Ok(false);
    }
    if !handle_edit_init(&matches)? {
        return Ok(false);
    }
    if !handle_ral_igcare(&matches)? {
        return Ok(false);
    }

    // --- Handle explorer TUI ---
    if matches.contains_id("explorer") {
        let path = matches
            .get_one::<String>("explorer")
            .map(|s| std::path::Path::new(s));
        crate::nfe::run_explorer(path)?;
        return Ok(false);
    }

    // --- No arguments: Launch interactive mode ---
    Ok(true)
}
