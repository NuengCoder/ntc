//! Hinter tests: ghost text, tab cycling, providers and the path engine.

use super::*;
use crate::navigator::Navigator;
use crate::utils::test_support::TempCwd;

/// `math ` ghosts the format file in the working directory before any
/// subcommand, and `<cmd> check ` ghosts nothing else.
#[test]
fn format_files_lead_the_hints() {
    let _cwd = TempCwd::with_fixture(&[("calc.ntc.math", "1+1\n"), ("notes.txt", "x\n")], &[]);
    let h = NtcHinter::new();

    let math = h.candidates_for("math ");
    assert_eq!(
        math.first().map(String::as_str),
        Some("calc.ntc.math"),
        "{math:?}"
    );
    assert!(math.contains(&"check".to_string()), "{math:?}");

    let check = h.candidates_for("math check ");
    assert_eq!(check, vec!["calc.ntc.math".to_string()], "{check:?}");

    // A fragment that matches nothing leaves the slot empty instead of
    // falling back to the whole directory.
    assert!(h.candidates_for("math check zzz").is_empty());
}

#[test]
fn ran_check_offers_the_ranfile() {
    let _cwd = TempCwd::with_fixture(
        &[(
            "NTCRANFILE.toml",
            "[targets.standalone]\ndeps = []\ncmd = \"echo hi\"\n",
        )],
        &[],
    );
    let h = NtcHinter::new();
    assert_eq!(
        h.candidates_for("ran check "),
        vec!["NTCRANFILE.toml".to_string()]
    );
}

#[test]
fn import_slots_offer_format_files() {
    let _cwd = TempCwd::with_fixture(
        &[("mine.ntc.ral", "[run_aliases]\n"), ("x.ntc.igcare", "")],
        &[],
    );
    assert_eq!(
        format_file_candidates(&["ral", "import"], ""),
        vec!["mine.ntc.ral".to_string()]
    );
    assert_eq!(
        format_file_candidates(&["igcare", "import"], "x"),
        vec!["x.ntc.igcare".to_string()]
    );
    // Slots that take free text never offer a format file.
    assert!(format_file_candidates(&["ral", "add"], "").is_empty());
    assert!(check_only_candidates(&["math"], "").is_none());
}

#[test]
fn import_slots_hint_format_files_through_the_hinter() {
    let _cwd = TempCwd::with_fixture(
        &[("mine.ntc.ral", "[run_aliases]\n"), ("x.ntc.igcare", "")],
        &[],
    );
    let h = NtcHinter::new();
    assert_eq!(
        h.candidates_for("ral import "),
        vec!["mine.ntc.ral".to_string()]
    );
    assert_eq!(
        h.candidates_for("igcare import x"),
        vec!["x.ntc.igcare".to_string()]
    );
}

#[test]
fn nprintf_percent_completes_conversions() {
    let h = NtcHinter::new();
    let quoted = h.candidates_for("nprintf \"%");
    assert_eq!(quoted.first().map(String::as_str), Some("%s"), "{quoted:?}");
    for conv in ["%d", "%x", "%f", "%b", "%q", "%%"] {
        assert!(
            quoted.contains(&conv.to_string()),
            "missing {conv}: {quoted:?}"
        );
    }

    // Flags/width/precision stay in the candidate, so the ghost only adds the
    // conversion character after they have been typed.
    let flagged = h.candidates_for("nprintf \"%-8");
    assert!(flagged.contains(&"%-8d".to_string()), "{flagged:?}");
    assert!(flagged.contains(&"%-8s".to_string()), "{flagged:?}");

    // A finished `%%` and a closed format are not conversion slots.
    assert!(nprintf_format_candidates("nprintf \"100%% done").is_none());
    assert!(nprintf_format_candidates("nprintf \"%s\"").is_none());
}

#[test]
fn nprintf_backslash_completes_escapes() {
    let h = NtcHinter::new();
    let escapes = h.candidates_for("nprintf \"line\\");
    assert_eq!(
        escapes.first().map(String::as_str),
        Some("\\n"),
        "{escapes:?}"
    );
    for escape in ["\\t", "\\r", "\\\\", "\\\"", "\\c"] {
        assert!(
            escapes.contains(&escape.to_string()),
            "missing {escape}: {escapes:?}"
        );
    }

    // An escaped backslash is complete; only an odd trailing run is open.
    assert!(nprintf_format_candidates("nprintf \"done\\\\").is_none());
}

#[test]
fn nprintf_ghost_remainder_skips_quotes_and_literal_text() {
    assert_eq!(
        nprintf_ghost_remainder("nprintf \"hello %", "%s"),
        Some("s".to_string())
    );
    assert_eq!(
        nprintf_ghost_remainder("nprintf \"%-8", "%-8d"),
        Some("d".to_string())
    );
    assert_eq!(
        nprintf_ghost_remainder("nprintf \"line\\", "\\n"),
        Some("n".to_string())
    );
    assert_eq!(
        nprintf_ghost_remainder("nprintf hello%", "%q"),
        Some("q".to_string())
    );
}

#[test]
fn local_config_created_while_the_shell_is_open_refreshes_aliases() {
    let _cwd = TempCwd::with_fixture(&[], &[]);
    let h = NtcHinter::new();
    let typed = "ntc_local_hint_fix";
    assert!(!h
        .candidates_for(typed)
        .contains(&"ntc_local_hint_fixture".to_string()));

    std::fs::write(
        "ntconfig.toml",
        "[run_aliases]\nntc_local_hint_fixture = \"echo local\"\n",
    )
    .unwrap();

    let candidates = h.candidates_for(typed);
    assert!(
        candidates.contains(&"ntc_local_hint_fixture".to_string()),
        "{candidates:?}"
    );
}

#[test]
fn local_aliases_refresh_after_cd() {
    let _cwd = TempCwd::with_fixture(&[], &["project"]);
    std::fs::write(
        "project/ntconfig.toml",
        "[run_aliases]\nntc_local_hint_fixture = \"echo local\"\n",
    )
    .unwrap();
    let h = NtcHinter::new();
    let typed = "ntc_local_hint_fix";
    assert!(!h
        .candidates_for(typed)
        .contains(&"ntc_local_hint_fixture".to_string()));

    let mut nav = Navigator::new().unwrap();
    nav.go_to(std::path::Path::new("project")).unwrap();

    let candidates = h.candidates_for(typed);
    assert!(
        candidates.contains(&"ntc_local_hint_fixture".to_string()),
        "{candidates:?}"
    );
}

#[test]
fn dollar_fragment_hints_env_vars_from_every_store() {
    std::env::set_var("NTC_HINT_OS_ONLY", "1");
    crate::utils::paths::set_session_var("NTC_HINT_SESSION_ONLY", "2");
    let h = NtcHinter::new();
    let c = h.candidates_for("print $NTC_HINT_");
    assert!(c.contains(&"$NTC_HINT_OS_ONLY".to_string()), "{c:?}");
    assert!(c.contains(&"$NTC_HINT_SESSION_ONLY".to_string()), "{c:?}");
    // A shorter prefix filters, and a bare `$` lists everything.
    assert!(h
        .candidates_for("print $NTC_HINT_O")
        .contains(&"$NTC_HINT_OS_ONLY".to_string()));
    assert!(h
        .candidates_for("print $")
        .contains(&"$NTC_HINT_OS_ONLY".to_string()));
    // The braced form keeps its shape so `${…}` stays valid.
    assert!(h
        .candidates_for("print ${NTC_HINT_O")
        .contains(&"${NTC_HINT_OS_ONLY}".to_string()));
    crate::utils::paths::remove_session_var("NTC_HINT_SESSION_ONLY");
    std::env::remove_var("NTC_HINT_OS_ONLY");
}

#[test]
fn l1_prefix_matches_commands() {
    let h = NtcHinter::new();
    let c = h.candidates_for("t");
    assert!(c.contains(&"tl".to_string()));
    assert!(c.contains(&"theme".to_string()));
    assert!(c.contains(&"todo".to_string()));
    // Exact-length matches are excluded (ghost only shows the remainder).
    assert!(!c.iter().any(|x| x == "t"));
}

#[test]
fn root_completion_contains_every_registry_name() {
    let registry = crate::shell::commands::registry_data::registry();
    let cmds = build_command_set();
    for name in registry.names() {
        assert!(cmds.contains(name), "root completion missing '{name}'");
    }
}

#[test]
fn code_tools_delegates_git_subcommands() {
    // `invalidate()` clears process-global caches shared with the
    // ext_complete tests (PS mode, line cache), so serialize with them.
    let _lock = crate::utils::test_support::lock_globals();
    let mut cfg = hint_test_config();
    cfg.code_tools_enabled = true;
    Config::with_test_config(cfg, || {
        crate::shell::ext_complete::invalidate();
        let h = NtcHinter::new();
        let (cands, base) = h.ghost_candidates("git ch", 6);
        assert_eq!(base, 0, "base offset");
        assert!(cands.contains(&"git checkout".to_string()), "{cands:?}");
        // Pipe stage: the base is the tail (which keeps the leading space).
        let (cands, base) = h.ghost_candidates("ls | git ch", 11);
        assert_eq!(base, 4, "base offset after `|`");
        assert!(cands.contains(&" git checkout".to_string()), "{cands:?}");
        // Unknown commands keep ntc's own (filesystem) candidates.
        let (_, base) = h.ghost_candidates("view Cargo", 10);
        assert_eq!(base, usize::MAX);
    });
}

#[test]
fn integration_shell_only_adds_installed_shell_names() {
    // Shares the ext_complete caches with the PS/host-mode tests.
    let _lock = crate::utils::test_support::lock_globals();
    let mut cfg = hint_test_config();
    cfg.integration_shell_enabled = true;
    Config::with_test_config(cfg, || {
        crate::shell::ext_complete::invalidate();
        let cmds = build_command_set();
        // `git` is a builtin, so it is always present; compilers are not
        // added by the shell gate.
        assert!(!cmds.contains("g++"), "code tools leaked into shells");
        assert!(!cmds.contains("ntcfixture"));
        for name in crate::shell::ext_complete::shell_names() {
            assert!(cmds.contains(&name), "missing installed shell {name}");
        }
    });
}

#[test]
fn code_tools_adds_only_installed_curated_names() {
    // Shares the ext_complete caches with the PS/host-mode tests.
    let _lock = crate::utils::test_support::lock_globals();
    let mut cfg = hint_test_config();
    cfg.code_tools_enabled = true;
    Config::with_test_config(cfg, || {
        crate::shell::ext_complete::invalidate();
        let cmds = build_command_set();
        for name in crate::shell::ext_complete::code_tool_names() {
            assert!(cmds.contains(&name), "missing installed tool {name}");
        }
        // Shells stay behind their own gate.
        for name in ["bash", "zsh", "fish"] {
            if !is_builtin_command(name) {
                assert!(!cmds.contains(name), "shell leaked into code tools");
            }
        }
        assert!(!cmds.contains("ntcfixture"));
    });
}

#[test]
fn config_code_tools_hints_on_off() {
    let flags: Vec<String> = subcommands_for(&["config", "code_tools"])
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(flags, vec!["ON", "OFF"]);
    assert_eq!(
        subcommands_for(&["config", "ct"]),
        vec!["ON", "OFF"],
        "short alias"
    );
}

#[test]
fn config_color_hints_on_off() {
    assert_eq!(subcommands_for(&["config", "color"]), vec!["ON", "OFF"]);
    assert_eq!(
        subcommands_for(&["config", "c"]),
        vec!["ON", "OFF"],
        "short alias"
    );
    assert_eq!(
        subcommands_for(&["cfg", "color"]),
        vec!["ON", "OFF"],
        "command alias"
    );

    // The ghost-text path sees the same candidates for both spellings.
    let h = NtcHinter::new();
    for line in ["config color ", "cfg color "] {
        let cands = h.candidates_for(line);
        assert!(cands.contains(&"ON".to_string()), "{line}: {cands:?}");
        assert!(cands.contains(&"OFF".to_string()), "{line}: {cands:?}");
    }
}

#[test]
fn full_line_ghost_is_case_insensitive() {
    assert_eq!(strip_prefix_ci("Get-Content", "get-c"), Some("ontent"));
    assert_eq!(strip_prefix_ci("Select-Object", "sel"), Some("ect-Object"));
    assert_eq!(strip_prefix_ci("Get-Content", "git"), None);
}

#[test]
fn gs_flags_are_suggested() {
    let h = NtcHinter::new();
    let c = h.candidates_for("gs -");
    for flag in [
        "-p", "--path", "-s", "--sort", "-d", "-l", "--limit", "-I", "-x", "-h", "-g", "-w", "-z",
    ] {
        assert!(c.contains(&flag.to_string()), "missing {flag}: {c:?}");
    }
    // The pager flag moved to `-I`; lowercase is not offered any more.
    assert!(!c.contains(&"-i".to_string()), "-i still offered: {c:?}");
}

/// `-g` / `--global` is only offered inside a folder that has an
/// `ntconfig.toml` (there the default target is the local file).
#[test]
fn global_scope_flag_is_hinted_only_inside_a_local_dir() {
    {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let flags = subcommands_for(&["ral", "add"]);
        assert!(!flags.contains(&"-g"), "{flags:?}");
        assert!(!flags.contains(&"--global"), "{flags:?}");
        let flags = subcommands_for(&["ignore", "dir"]);
        assert!(!flags.contains(&"-g"), "{flags:?}");
    }
    {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[("ntconfig.toml", "")], &[]);
        let flags = subcommands_for(&["ral", "add"]);
        assert!(flags.contains(&"-g"), "{flags:?}");
        assert!(flags.contains(&"--global"), "{flags:?}");
        let flags = subcommands_for(&["ral", "rm"]);
        assert!(flags.contains(&"-g"), "{flags:?}");
        let flags = subcommands_for(&["ral", "hintn", "add"]);
        assert!(flags.contains(&"-g"), "{flags:?}");
        let flags = subcommands_for(&["ignore", "dir"]);
        assert!(flags.contains(&"-g"), "{flags:?}");
        let flags = subcommands_for(&["care", "ext"]);
        assert!(flags.contains(&"-g"), "{flags:?}");
    }
}

#[test]
fn tl_cycles_all_subcommands() {
    let h = NtcHinter::new();
    assert_eq!(h.candidates_for("tl "), vec!["dir", "ext", "file"]);
    assert_eq!(h.candidates_for("tl d"), vec!["dir"]);
    assert_eq!(h.candidates_for("tl e"), vec!["ext"]);
}

#[test]
fn view_flag_fragments_ghost_size_flags() {
    let h = NtcHinter::new();
    // `view -d` ghosts `-ds`; the exact `-d` itself is excluded.
    let c = h.candidates_for("view -d");
    assert!(c.contains(&"-ds".to_string()), "got: {:?}", c);
    assert!(!c.iter().any(|x| x == "-d"));
    // `view --d` ghosts `--depth` and `--dirsize`.
    let c = h.candidates_for("view --d");
    assert!(c.contains(&"--depth".to_string()), "got: {:?}", c);
    assert!(c.contains(&"--dirsize".to_string()), "got: {:?}", c);
    // A second flag is still suggested after the first one.
    let c = h.candidates_for("view -ds --");
    assert!(c.contains(&"--depth".to_string()), "got: {:?}", c);
    // Dash-only still offers the universal help flag.
    let c = h.candidates_for("view -");
    assert!(c.contains(&"--help".to_string()), "got: {:?}", c);
    // New display flags ghost too.
    let c = h.candidates_for("view --s");
    assert!(c.contains(&"--sort".to_string()), "got: {:?}", c);
    assert!(c.contains(&"--size".to_string()), "got: {:?}", c);
    let c = h.candidates_for("view --h");
    assert!(c.contains(&"--head".to_string()), "got: {:?}", c);
    // A value-taking flag awaiting its value offers no subcommands.
    assert!(subcommands_for(&["view", "--head"]).is_empty());
    assert!(subcommands_for(&["view", "--sort"]).is_empty());
    assert!(!subcommands_for(&["view", "-s"]).is_empty());
}

#[test]
fn view_path_taken_skips_flags_and_values() {
    // `src` must resolve to a real dir for the `-ext` value-stop check,
    // so run inside an isolated repo-like fixture (also serializes the
    // process-global CWD against parallel tests on CI).
    let _cwd = TempCwd::with_repo_fixture();
    assert!(!view_path_taken(&[]));
    assert!(!view_path_taken(&["-ds"]));
    assert!(!view_path_taken(&["-d", "2"]));
    assert!(!view_path_taken(&["-p", "src"]));
    assert!(!view_path_taken(&["--dirsize", "--depth", "3"]));
    assert!(!view_path_taken(&["-ext", "rs", "c"]));
    assert!(!view_path_taken(&["-ext"]));
    assert!(view_path_taken(&["src"]));
    assert!(view_path_taken(&["-ds", "src"]));
    assert!(view_path_taken(&["src", "-ds"]));
    assert!(view_path_taken(&["-ext", "rs", "src", "-ds"]));
    // Optional size values are skipped; bare size flags are not.
    assert!(!view_path_taken(&["-s", "10MB"]));
    assert!(!view_path_taken(&["--head", "5"]));
    assert!(view_path_taken(&["-s", "src"]));
    assert!(!view_path_taken(&["-s"]));
    assert!(!view_path_taken(&["-s", "10MB", "-d", "2"]));
}

#[test]
fn ext_slot_detection() {
    assert!(in_ext_slot(&["scan", "ext"]));
    assert!(in_ext_slot(&["scan", "ext", "rs"]));
    assert!(in_ext_slot(&["tl", "ext"]));
    assert!(in_ext_slot(&["view", "-ext"]));
    assert!(in_ext_slot(&["view", "src", "--ext", "rs"]));
    assert!(!in_ext_slot(&["scan"]));
    assert!(!in_ext_slot(&["scan", "ext", "rs", "-d"]));
    assert!(!in_ext_slot(&["view", "src"]));
    assert!(!in_ext_slot(&["view", "-ds"]));
    assert!(!in_ext_slot(&["tl", "file"]));
}

#[test]
fn ext_names_complete_and_comma_aware() {
    let h = NtcHinter::new();
    // `scan ext r` suggests `rs` (and friends), not files.
    let c = h.candidates_for("scan ext r");
    assert!(c.contains(&"rs".to_string()), "got: {:?}", c);
    // After a complete ext, the next slot still completes.
    let c = h.candidates_for("scan ext rs c");
    assert!(c.contains(&"cs".to_string()), "got: {:?}", c);
    assert!(c.contains(&"cpp".to_string()), "got: {:?}", c);
    // Comma form completes after the comma with full replacements.
    let c = h.candidates_for("scan ext rs,c");
    assert!(c.contains(&"rs,cs".to_string()), "got: {:?}", c);
    assert!(!c.iter().any(|x| x == "cs"));
    // `view -ext` and the `tl ext` bonus behave the same.
    let c = h.candidates_for("view -ext p");
    assert!(c.contains(&"py".to_string()), "got: {:?}", c);
    let c = h.candidates_for("tl ext j");
    assert!(c.contains(&"js".to_string()), "got: {:?}", c);
    // Bare slots stay quiet (flags/subcommands cover them).
    assert!(ext_names_for_command(&["scan", "ext"], "").is_empty());
}

#[test]
fn mv_cp_tp_slash_lists_savepoint_contents_in_src_only() {
    let _cwd = TempCwd::with_fixture(
        &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
        &["tp1", "tp1/sub", "tp2"],
    );
    let root = std::env::current_dir().unwrap();
    let mut cfg = hint_test_config();
    cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
    cfg.teleports.insert("tp2".to_string(), root.join("tp2"));
    Config::with_test_config(cfg, || {
        let h = NtcHinter::new();
        // `@tp/` lists files plain and dirs with trailing `/`.
        let c = h.candidates_for("cp @tp1/");
        assert!(c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
        assert!(c.contains(&"@tp1/sub/".to_string()), "got: {:?}", c);
        assert!(
            !c.contains(&"@tp1/sub/deep.txt".to_string()),
            "got: {:?}",
            c
        );
        // Partial name filters.
        let c = h.candidates_for("cp @tp1/f");
        assert!(c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
        assert!(!c.iter().any(|s| s == "@tp1/sub/"), "got: {:?}", c);
        // Nested traversal.
        let c = h.candidates_for("cp @tp1/sub/d");
        assert!(c.contains(&"@tp1/sub/deep.txt".to_string()), "got: {:?}", c);
        // Flags don't consume the src slot.
        let c = h.candidates_for("cp -f @tp1/");
        assert!(c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
        // Dst slots keep existing behavior (nothing).
        assert!(h.candidates_for("cp file @tp2/").is_empty());
        let c = h.candidates_for("mv @tp1/ ");
        assert!(c.is_empty(), "got: {:?}", c);
        // Unknown savepoints and other commands are untouched.
        assert!(h.candidates_for("cp @ghost/").is_empty());
        assert!(h.candidates_for("size @tp1/").is_empty());
    });
}

#[test]
fn view_and_txtf_tp_subpaths_respect_entry_modes() {
    let _cwd = TempCwd::with_fixture(
        &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
        &["tp1", "tp1/sub"],
    );
    let root = std::env::current_dir().unwrap();
    let mut cfg = hint_test_config();
    cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
    Config::with_test_config(cfg, || {
        let h = NtcHinter::new();
        // viewf / txtf offer files (dirs stay traversable).
        for line in ["viewf @tp1/", "txtf @tp1/", "txtf --head 5 @tp1/"] {
            let c = h.candidates_for(line);
            assert!(
                c.contains(&"@tp1/file.txt".to_string()),
                "got: {:?} for {}",
                c,
                line
            );
        }
        // Directory traversal stays available for txtf's picker mode.
        let c = h.candidates_for("txtf @tp1/");
        assert!(c.contains(&"@tp1/sub/".to_string()), "got: {:?}", c);
        let c = fs_candidates_for(&["txtf"], "tp1/");
        assert!(c.contains(&"tp1/sub/".to_string()), "got: {:?}", c);
        assert!(c.contains(&"tp1/file.txt".to_string()), "got: {:?}", c);
        // viewd hides files but keeps directories.
        let c = h.candidates_for("viewd @tp1/");
        assert!(c.contains(&"@tp1/sub/".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
        // Filled slots offer only flags, never more paths.
        for line in ["viewf @tp1/file.txt ", "txtf @tp1/file.txt "] {
            let c = h.candidates_for(line);
            assert!(
                !c.iter().any(|s| s.starts_with("@tp1/")),
                "got: {:?} for {}",
                c,
                line
            );
        }
    });
}

#[test]
fn view_variants_offer_their_flags() {
    let h = NtcHinter::new();
    for line in ["viewf --", "viewd --"] {
        let c = h.candidates_for(line);
        assert!(
            c.contains(&"--care".to_string()),
            "got: {:?} for {}",
            c,
            line
        );
        assert!(
            c.contains(&"--copy".to_string()),
            "got: {:?} for {}",
            c,
            line
        );
    }
    assert!(h
        .candidates_for("view --")
        .contains(&"--directory".to_string()));
}

#[test]
fn mv_cp_at_savepoints_and_first_slot() {
    let _cwd = TempCwd::with_repo_fixture();
    let mut cfg = hint_test_config();
    cfg.teleports
        .insert("docs".to_string(), std::path::PathBuf::from("C:/docs"));
    cfg.teleports
        .insert("dl".to_string(), std::path::PathBuf::from("C:/dl"));
    Config::with_test_config(cfg, || {
        let h = NtcHinter::new();
        // `@` completes savepoints for mv/cp operands...
        let c = h.candidates_for("cp @");
        assert!(c.contains(&"@docs".to_string()), "got: {:?}", c);
        assert!(c.contains(&"@dl".to_string()), "got: {:?}", c);
        let c = h.candidates_for("mv @do");
        assert!(c.contains(&"@docs".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"@dl".to_string()), "got: {:?}", c);
        // ...and the first plain slot lists files and dirs.
        let c = h.candidates_for("cp ");
        assert!(c.contains(&"notes.txt".to_string()), "got: {:?}", c);
        assert!(c.contains(&"src/".to_string()), "got: {:?}", c);
        let c = h.candidates_for("mv ");
        assert!(c.contains(&"Cargo.toml".to_string()), "got: {:?}", c);
        assert!(c.contains(&"docs/".to_string()), "got: {:?}", c);
    });
}

#[test]
fn scan_files_dir_subcommands_and_flags() {
    let h = NtcHinter::new();
    // New subcommands complete after `scan`.
    let c = h.candidates_for("scan ");
    assert!(c.contains(&"files".to_string()), "got: {:?}", c);
    assert!(c.contains(&"dir".to_string()), "got: {:?}", c);
    assert!(c.contains(&"ext".to_string()), "got: {:?}", c);
    // Same flag set as `scan ext`.
    for cmd in ["scan files --", "scan dir --", "scan ext --"] {
        let c = h.candidates_for(cmd);
        assert!(
            c.contains(&"--care".to_string()),
            "got: {:?} for {}",
            c,
            cmd
        );
        assert!(
            c.contains(&"--depth".to_string()),
            "got: {:?} for {}",
            c,
            cmd
        );
        assert!(
            c.contains(&"--size".to_string()),
            "got: {:?} for {}",
            c,
            cmd
        );
    }
    // Pattern slots do NOT get ext-name ghosts.
    assert!(!in_ext_slot(&["scan", "files"]));
    assert!(!in_ext_slot(&["scan", "dir"]));
    assert!(ext_names_for_command(&["scan", "files"], "r").is_empty());
}

#[test]
fn view_after_flags_still_completes_paths() {
    // Path slot still completes after flags (fixture dir is non-empty).
    let _cwd = TempCwd::with_repo_fixture();
    assert!(!fs_candidates_for(&["view", "-ds"], "").is_empty());
    assert!(!fs_candidates_for(&["view", "-d", "2", "--dirsize"], "").is_empty());
    // Once the path slot is filled, no more path hints.
    assert!(fs_candidates_for(&["view", "-ds", "src"], "").is_empty());
}

#[test]
fn arc_output_slot_offers_no_hint() {
    let h = NtcHinter::new();
    assert!(h.candidates_for("arc create ").is_empty());
    assert!(fs_candidates_for(&["arc", "create"], "").is_empty());
}

#[test]
fn arc_list_extract_filter_to_archives() {
    for parent in [&["arc", "list"][..], &["arc", "extract"][..]] {
        for c in fs_candidates_for(parent, "") {
            if c.ends_with('/') {
                continue; // directories pass through for traversal
            }
            let l = c.to_lowercase();
            assert!(
                l.ends_with(".zip")
                    || l.ends_with(".tar.gz")
                    || l.ends_with(".tgz")
                    || l.ends_with(".tar"),
                "non-archive offered: {}",
                c
            );
        }
    }
    // Destination slot exists and only completes directories.
    for c in fs_candidates_for(&["arc", "extract", "some.zip"], "") {
        assert!(c.ends_with('/'), "non-dir dest offered: {}", c);
    }
}

#[test]
fn cd_to_suggests_teleports_not_dirs() {
    let h = NtcHinter::new();
    // `cd to ` is a teleport jump — filesystem entries end with '/'.
    for c in h.candidates_for("cd to ") {
        assert!(!c.ends_with('/'), "unexpected fs entry: {}", c);
    }
}

#[test]
fn cd_bare_lists_dirs_only() {
    let h = NtcHinter::new();
    // Bare `cd` completes subdirectories only — every candidate ends
    // with '/' and no bookmark names appear.
    for c in h.candidates_for("cd ") {
        assert!(c.ends_with('/'), "unexpected non-dir entry: {}", c);
    }
}

#[test]
fn cd_at_names_are_not_completed() {
    let h = NtcHinter::new();
    // `cd @name` is not a teleport jump, so no @ candidates after `cd`.
    assert!(h.candidates_for("cd @").is_empty());
    assert!(h.candidates_for("cd @work").is_empty());
}

#[test]
fn ds_lists_dirs_only() {
    // Every candidate ends with `/` (fixture always has `src/`).
    let _cwd = TempCwd::with_repo_fixture();
    let c = fs_candidates_for(&["ds"], "");
    assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
    assert!(
        c.iter().all(|x| x.ends_with('/')),
        "non-dir offered: {:?}",
        c
    );
}

#[test]
fn fs_and_mkf_list_files_only() {
    // No directory entries at all; regular files show up.
    let _cwd = TempCwd::with_repo_fixture();
    for parent in [&["fs"][..], &["fsc"][..], &["mkf"][..], &["rmf"][..]] {
        let c = fs_candidates_for(parent, "");
        assert!(
            !c.iter().any(|x| x.ends_with('/')),
            "dir offered for {:?}: {:?}",
            parent,
            c
        );
    }
    let c = fs_candidates_for(&["fs"], "");
    assert!(
        c.iter().any(|x| x == "Cargo.toml"),
        "expected files, got: {:?}",
        c
    );
}

#[test]
fn txtc_suppresses_binaries_but_keeps_dirs() {
    // Dirs pass for `dir/<Tab>` traversal; known binaries never show.
    let _cwd = TempCwd::with_repo_fixture();
    let c = fs_candidates_for(&["txtc"], "");
    assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
    for x in &c {
        if x.ends_with('/') {
            continue;
        }
        assert!(
            !crate::filetype::is_finder_hidden(std::path::Path::new(x)),
            "binary offered: {}",
            x
        );
    }
    // Same for the `txtf` twin.
    let c = fs_candidates_for(&["txtf"], "");
    assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
}

#[test]
fn non_image_anchors_survive_own_filters() {
    // Guards the earlier regression where every command inherited the
    // image extension filter: non-image anchors must show up.
    let _cwd = TempCwd::with_repo_fixture();
    assert!(fs_candidates_for(&["fs"], "").contains(&"Cargo.toml".to_string()));
    assert!(fs_candidates_for(&["txtc"], "").contains(&"Cargo.toml".to_string()));
    assert!(fs_candidates_for(&["mkf"], "").contains(&"Cargo.toml".to_string()));
    // …while the image slot itself still filters to images + dirs.
    for x in fs_candidates_for(&["image"], "") {
        if x.ends_with('/') {
            continue;
        }
        let l = x.to_lowercase();
        assert!(
            IMAGE_EXTS.iter().any(|e| l.ends_with(e)),
            "non-image offered: {}",
            x
        );
    }
}

#[test]
fn fileops_complete_every_operand_slot() {
    // `rmf`/`mkf` take comma-separated file lists — slots past the
    // first complete files too (`rmf a.txt, b<Tab>`).
    let _cwd = TempCwd::with_repo_fixture();
    let c = fs_candidates_for(&["rmf", "a.txt,"], "");
    assert!(!c.iter().any(|x| x.ends_with('/')), "dir offered: {:?}", c);
    let c = fs_candidates_for(&["mkf", "a.txt", "b.txt,"], "");
    assert!(!c.iter().any(|x| x.ends_with('/')), "dir offered: {:?}", c);
    // `mkd`/`rmd` take comma-separated dir lists — dirs at any depth.
    let c = fs_candidates_for(&["rmd", "old,"], "");
    assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
    assert!(c.iter().all(|x| x.ends_with('/')), "got: {:?}", c);
}

#[test]
fn igcare_values_complete_bare_names() {
    // `ignore`/`care` store plain names, so no `src/` trailing slash.
    let _cwd = TempCwd::with_fixture(
        &[
            ("Cargo.toml", "[package]\nname = \"fixture\"\n"),
            ("app.toml", "x = 1\n"),
            ("notes.txt", "hello\n"),
            ("photo.png", "fakepng"),
            ("src/main.rs", "fn main() {}\n"),
        ],
        &["src", "docs", "target"],
    );
    let mut cfg = Config::default();
    cfg.ignored_extensions.insert("toml".into());
    cfg.ignored_files.insert("notes.txt".into());
    Config::with_test_config(cfg, || {
        let h = NtcHinter::new();
        let c = h.candidates_for("ignore dir s");
        assert!(c.contains(&"src".to_string()), "got: {:?}", c);
        assert!(!c.iter().any(|x| x == "src/"), "got: {:?}", c);
        // Bare slots list everything that is not ignored yet.
        let c = h.candidates_for("ignore dir ");
        assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"target".to_string()), "got: {:?}", c);
        // `care` offers only the already-ignored items.
        let c = h.candidates_for("care dir ");
        assert!(c.contains(&"target".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"src".to_string()), "got: {:?}", c);
        // Files and extensions follow the same split.
        let c = h.candidates_for("ignore file ");
        assert!(c.contains(&"Cargo.toml".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"notes.txt".to_string()), "got: {:?}", c);
        let c = h.candidates_for("care file no");
        assert!(c.contains(&"notes.txt".to_string()), "got: {:?}", c);
        let c = h.candidates_for("ignore ext t");
        assert!(c.contains(&"txt".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"toml".to_string()), "got: {:?}", c);
        let c = h.candidates_for("care ext t");
        assert!(c.contains(&"toml".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"txt".to_string()), "got: {:?}", c);
        // Comma-separated slots keep the typed prefix.
        let c = h.candidates_for("ignore dir src,d");
        assert!(c.contains(&"src,docs".to_string()), "got: {:?}", c);
        // Non-value slots stay untouched.
        assert!(igcare_value_candidates_for(&["ignore"], "").is_none());
        assert!(igcare_value_candidates_for(&["ignore", "foo"], "").is_none());
        assert!(igcare_value_candidates_for(&["fs"], "").is_none());
    });
}

#[test]
fn fileops_tp_subpaths_respect_entry_modes() {
    let _cwd = TempCwd::with_fixture(
        &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
        &["tp1", "tp1/sub"],
    );
    let root = std::env::current_dir().unwrap();
    let mut cfg = hint_test_config();
    cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
    Config::with_test_config(cfg, || {
        let h = NtcHinter::new();
        // mkf / rmf offer files; directories stay traversable.
        for line in ["mkf @tp1/", "rmf @tp1/", "mkf a.txt, @tp1/"] {
            let c = h.candidates_for(line);
            assert!(
                c.contains(&"@tp1/file.txt".to_string()),
                "got: {:?} for {}",
                c,
                line
            );
        }
        // mkd / rmd offer directories only.
        for line in ["mkd @tp1/", "rmd @tp1/"] {
            let c = h.candidates_for(line);
            assert!(
                c.contains(&"@tp1/sub/".to_string()),
                "got: {:?} for {}",
                c,
                line
            );
            assert!(
                !c.contains(&"@tp1/file.txt".to_string()),
                "got: {:?} for {}",
                c,
                line
            );
        }
        // Nested traversal.
        let c = h.candidates_for("rmf @tp1/sub/d");
        assert!(c.contains(&"@tp1/sub/deep.txt".to_string()), "got: {:?}", c);
    });
}

#[test]
fn rmf_star_dot_completes_extensions() {
    // `rmf *.<Tab>` offers `*.<ext>` for extensions present in the dir
    // (fixture has `.toml`, `.rs`, …); a typed stub filters them.
    let _cwd = TempCwd::with_repo_fixture();
    let c = fs_candidates_for(&["rmf"], "*.");
    assert!(!c.is_empty(), "no *-candidates offered");
    assert!(
        c.iter().all(|x| x.starts_with("*.")),
        "non-pattern offered: {:?}",
        c
    );
    assert!(c.contains(&"*.toml".to_string()), "got: {:?}", c);
    let c2 = fs_candidates_for(&["rmf"], "*.t");
    assert!(!c2.is_empty(), "stub filtered everything");
    assert!(c2.iter().all(|x| x.starts_with("*.")), "got: {:?}", c2);
    assert!(c2.contains(&"*.toml".to_string()), "got: {:?}", c2);
    assert!(c2.len() <= c.len());
    // A fully-typed pattern ghosts nothing.
    assert!(fs_candidates_for(&["rmf"], "*.toml").is_empty());
}

#[test]
fn mv_cp_star_completes_extensions() {
    // `cp *` offers `*` itself plus every `*.<ext>` present, so Tab
    // cycles `*` -> `*.toml` -> `*.txt` -> ….
    let _cwd = TempCwd::with_repo_fixture();
    let c = fs_candidates_for(&["cp"], "*");
    assert!(c.contains(&"*".to_string()), "got: {:?}", c);
    assert!(c.contains(&"*.toml".to_string()), "got: {:?}", c);
    assert!(c.contains(&"*.txt".to_string()), "got: {:?}", c);
    assert!(c.contains(&"*.png".to_string()), "got: {:?}", c);
    // A typed stub filters and drops the bare `*`.
    let c2 = fs_candidates_for(&["mv"], "*.t");
    assert!(!c2.contains(&"*".to_string()), "got: {:?}", c2);
    assert!(c2.contains(&"*.toml".to_string()), "got: {:?}", c2);
    assert!(c2.contains(&"*.txt".to_string()), "got: {:?}", c2);
    // Fully-typed pattern, dst slot and non-pattern forms stay quiet.
    assert!(fs_candidates_for(&["cp"], "*.toml").is_empty());
    assert!(fs_candidates_for(&["cp", "a.txt"], "*.py").is_empty());
    assert!(fs_candidates_for(&["cp"], "*a").is_empty());
    // End-to-end through the hinter, with a leading flag.
    let h = NtcHinter::new();
    let c = h.candidates_for("cp -f *");
    assert!(c.contains(&"*".to_string()), "got: {:?}", c);
    assert!(c.contains(&"*.toml".to_string()), "got: {:?}", c);
}

#[test]
fn stage_tail_splits_top_level_operators() {
    assert_eq!(NtcHinter::stage_tail("ran reset && tl ").0, " tl ");
    assert_eq!(NtcHinter::stage_tail("ran reset && tl").0, " tl");
    assert_eq!(NtcHinter::stage_tail("tl").0, "tl");
    // Pipes and redirects split too; the operator is reported.
    assert_eq!(NtcHinter::stage_tail("gs main | sort").0, " sort");
    assert_eq!(NtcHinter::stage_tail("gs main | sort").1, Some("|"));
    assert_eq!(NtcHinter::stage_tail("a || b").1, Some("||"));
    assert_eq!(NtcHinter::stage_tail("a > out.txt").1, Some(">"));
    assert_eq!(NtcHinter::stage_tail("a >> out.txt").1, Some(">>"));
    assert_eq!(NtcHinter::stage_tail("a < in.txt").1, Some("<"));
    assert_eq!(NtcHinter::stage_tail("a <<EOF").0, "EOF");
    assert_eq!(NtcHinter::stage_tail("a <<< hi").1, Some("<<<"));
    // Single `&` is literal, quoted operators are literal, unclosed
    // quotes swallow the rest — all mean "no operator".
    assert_eq!(NtcHinter::stage_tail("a & b").0, "a & b");
    assert_eq!(NtcHinter::stage_tail("say \"a && b").0, "say \"a && b");
    assert_eq!(NtcHinter::stage_tail("say \"a && b\" && tl ").0, " tl ");
    assert_eq!(NtcHinter::stage_tail("&&").0, "");
    // Unspaced operators are data, exactly like the executor.
    assert_eq!(NtcHinter::stage_tail("math a|b").0, "math a|b");
    assert_eq!(
        NtcHinter::stage_tail("nsheet x --filter age>30").0,
        "nsheet x --filter age>30"
    );
}

#[test]
fn chained_segments_complete_like_single_lines() {
    let h = NtcHinter::new();
    // Second segment behaves as if typed alone (any command).
    assert_eq!(
        h.candidates_for("cd / && tl "),
        vec!["dir".to_string(), "ext".to_string(), "file".to_string()]
    );
    assert_eq!(h.candidates_for("tl d && tl e"), vec!["ext".to_string()]);
    // Depth-sensitive logic (ext slot) works past `&&` too.
    assert!(h
        .candidates_for("tl ext && scan ext r")
        .contains(&"rs".to_string()));
    // `--help` suppression is per-segment now.
    assert_eq!(
        h.candidates_for("view --help && tl "),
        vec!["dir".to_string(), "ext".to_string(), "file".to_string()]
    );
    // Empty tail stays quiet, like an empty line.
    assert!(h.candidates_for("ran reset &&").is_empty());
    assert!(h.candidates_for("ran reset && ").is_empty());
}

#[test]
fn at_fragment_filters_teleport_names() {
    // Regression: `@s` never suggested `@shop` — candidates carry
    // the `@` but were filtered by the stripped stub.
    let mut cfg = Config::default();
    cfg.teleports
        .insert("shop".to_string(), std::path::PathBuf::from("/tmp/shop"));
    cfg.teleports
        .insert("work".to_string(), std::path::PathBuf::from("/tmp/work"));
    Config::with_test_config(cfg, || {
        let h = NtcHinter::new();
        // The reported case: `@s` ghosts `@shop`.
        assert_eq!(h.candidates_for("@s"), vec!["@shop".to_string()]);
        // Bare `@` lists everything; fully-typed ghosts nothing.
        let all = h.candidates_for("@");
        assert!(all.contains(&"@shop".to_string()), "got: {:?}", all);
        assert!(all.contains(&"@work".to_string()), "got: {:?}", all);
        assert!(h.candidates_for("@shop").is_empty());
        // Case-insensitive, both positions.
        assert_eq!(h.candidates_for("@S"), vec!["@shop".to_string()]);
        assert_eq!(h.candidates_for("tp jump @w"), vec!["@work".to_string()]);
        // `cd @…` stays quiet (bare cd takes subdirs only).
        assert!(h.candidates_for("cd @s").is_empty());
    });
}

#[test]
fn bare_at_sign_lists_teleports() {
    let h = NtcHinter::new();
    // Whatever the store holds, every candidate is an `@name`.
    // (Empty store → empty list; never filesystem noise.)
    for c in h.candidates_for("@") {
        assert!(c.starts_with('@'), "unexpected: {}", c);
    }
    // `cd @` stays quiet (bare cd takes subdirs only).
    assert!(h.candidates_for("cd @").is_empty());
}

#[test]
fn filter_prefix_is_case_insensitive() {
    let out = filter_prefix(vec!["Downloads".to_string(), "docs".to_string()], "d");
    assert_eq!(out, vec!["Downloads", "docs"]);
    let out = filter_prefix(vec!["dir".to_string(), "ext".to_string()], "");
    assert_eq!(out, vec!["dir", "ext"]);
}

#[test]
fn help_suffix_strips_at_any_depth() {
    assert_eq!(strip_help_suffix("cd --help"), Some("cd".to_string()));
    assert_eq!(
        strip_help_suffix("ignore dir --help"),
        Some("ignore dir".to_string())
    );
    assert_eq!(
        strip_help_suffix("ignore dir foo --help"),
        Some("ignore dir foo".to_string())
    );
    assert_eq!(
        strip_help_suffix("cd to foo -h"),
        Some("cd to foo".to_string())
    );
    assert_eq!(strip_help_suffix("cd"), None);
    assert_eq!(strip_help_suffix("ignore dir foo"), None);
}

#[test]
fn help_flag_is_suggested_for_dash_frag() {
    let h = NtcHinter::new();
    assert!(h
        .candidates_for("ignore --")
        .contains(&"--help".to_string()));
    assert_eq!(h.candidates_for("ignore --h"), vec!["--help".to_string()]);
    assert!(h.candidates_for("cd -").contains(&"--help".to_string()));
}

#[test]
fn help_token_suppresses_ghost() {
    let h = NtcHinter::new();
    assert!(h.candidates_for("ignore --help").is_empty());
    assert!(h.candidates_for("ignore dir --help").is_empty());
    assert!(h.candidates_for("cd to foo --help").is_empty());
    assert!(h.candidates_for("ignore --help ").is_empty());
}

#[test]
fn serve_flags_include_auth() {
    let h = NtcHinter::new();
    // `serve --a` ghosts `--auth` (and `--auth` alone lists flags).
    assert!(h
        .candidates_for("serve --a")
        .contains(&"--auth".to_string()));
    // Fully typed flags ghost nothing (ghost is the untyped remainder).
    assert!(h.candidates_for("serve --auth").is_empty());
    assert!(h.candidates_for("serve --").contains(&"--auth".to_string()));
    // Sibling flags keep working alongside it.
    assert!(h
        .candidates_for("serve --t")
        .contains(&"--tunnel".to_string()));
    // Share flags (internet-first LAN + QR).
    assert!(h
        .candidates_for("serve --s")
        .contains(&"--share".to_string()));
    assert!(h
        .candidates_for("serve --")
        .contains(&"--share".to_string()));
    assert!(h.candidates_for("serve --").contains(&"--qr".to_string()));
    assert!(h
        .candidates_for("serve --")
        .contains(&"--no-qr".to_string()));
    // Never-implemented flags must not be suggested (`--https` used to
    // ghost here while cmd_serve rejected it as unknown).
    assert!(!h
        .candidates_for("serve --")
        .contains(&"--https".to_string()));
    assert!(h.candidates_for("serve --ht").is_empty());
    // `--lock` (serve files read-only) completes on both arms.
    assert!(h.candidates_for("serve --").contains(&"--lock".to_string()));
    assert!(h
        .candidates_for("serve files --")
        .contains(&"--lock".to_string()));
    assert!(h
        .candidates_for("serve files --l")
        .contains(&"--lock".to_string()));
}

#[test]
fn serve_config_subcommands_complete() {
    let h = NtcHinter::new();
    // `serve c` suggests the config subcommand alongside others.
    assert!(h.candidates_for("serve c").contains(&"config".to_string()));
    // `serve config` is the GUI now: flags only, no service arguments.
    assert!(h
        .candidates_for("serve config --")
        .contains(&"--port".to_string()));
    assert!(h
        .candidates_for("serve config --")
        .contains(&"--bind".to_string()));
    assert!(h
        .candidates_for("serve fake --")
        .contains(&"--port".to_string()));
    // Same from the `config serve` alias side.
    assert!(h.candidates_for("config s").contains(&"serve".to_string()));
    assert!(h
        .candidates_for("config serve ")
        .contains(&"files".to_string()));
    assert!(h
        .candidates_for("config serve ")
        .contains(&"git".to_string()));
    assert!(h
        .candidates_for("config serve ")
        .contains(&"arc".to_string()));
    assert!(h
        .candidates_for("serve git --")
        .contains(&"--port".to_string()));
    assert!(h
        .candidates_for("serve arc --")
        .contains(&"--bind".to_string()));
    assert!(h
        .candidates_for("serve theme --")
        .contains(&"--port".to_string()));
    // `serve ndb` is the database browser: flags only, loopback by default.
    assert!(h
        .candidates_for("serve ndb --")
        .contains(&"--port".to_string()));
    assert!(h
        .candidates_for("serve ndb --")
        .contains(&"--auth".to_string()));
    assert!(h
        .candidates_for("serve ndb --")
        .contains(&"--write".to_string()));
    // The database is picked in the page: no path completion.
    assert!(fs_candidates_for(&["serve", "ndb"], "app").is_empty());
    assert!(h
        .candidates_for("config serve ")
        .contains(&"ndb".to_string()));
    assert!(h
        .candidates_for("config serve ")
        .contains(&"fake".to_string()));
    assert!(h
        .candidates_for("config serve ")
        .contains(&"config".to_string()));
    assert!(h
        .candidates_for("serve chess --")
        .contains(&"--online".to_string()));
}

#[test]
fn serve_chess_flags_exclude_auth() {
    let h = NtcHinter::new();
    // `serve chess` rejects --auth/--spa: never suggested.
    assert!(h
        .candidates_for("serve chess --")
        .contains(&"--port".to_string()));
    assert!(h
        .candidates_for("serve chess --")
        .contains(&"--web-dir".to_string()));
    assert!(!h
        .candidates_for("serve chess --")
        .contains(&"--auth".to_string()));
    assert!(!h
        .candidates_for("serve chess --")
        .contains(&"--spa".to_string()));
    // `serve http-server` rejects --spa/--web-dir: never suggested.
    assert!(h
        .candidates_for("serve http-server --")
        .contains(&"--auth".to_string()));
    assert!(!h
        .candidates_for("serve http-server --")
        .contains(&"--spa".to_string()));
    assert!(!h
        .candidates_for("serve http-server --")
        .contains(&"--web-dir".to_string()));
}

#[test]
fn nsheet_flags_include_open_view() {
    let h = NtcHinter::new();
    assert!(h.candidates_for("nsheet o").contains(&"open".to_string()));
    assert!(h
        .candidates_for("nsheet --f")
        .contains(&"--filter".to_string()));
    assert!(h
        .candidates_for("nsheet --")
        .contains(&"--sort".to_string()));
    assert!(h
        .candidates_for("nsheet --")
        .contains(&"--export".to_string()));
    assert!(h.candidates_for("nsheet --xyz").is_empty());
}

#[test]
fn nssh_flags_include_subcommands() {
    let h = NtcHinter::new();
    assert!(h.candidates_for("nssh ").contains(&"ssh".to_string()));
    assert!(h.candidates_for("nssh s").contains(&"ssh".to_string()));
    assert!(h
        .candidates_for("nssh ssh --")
        .contains(&"--cmd".to_string()));
    assert!(h
        .candidates_for("nssh ssh --c")
        .contains(&"--cmd".to_string()));
    assert!(h
        .candidates_for("nssh add --h")
        .contains(&"--host".to_string()));
    assert!(h
        .candidates_for("nssh scan --t")
        .contains(&"--top".to_string()));
    assert!(h.candidates_for("nssh scan --xyz").is_empty());
}

fn hint_test_config() -> Config {
    let mut cfg = Config::default();
    cfg.run_aliases
        .insert("tld".to_string(), "tl dir ${x:-.}".to_string());
    cfg.run_alias_hints
        .insert("tld".to_string(), "#x --hint-dir".to_string());
    cfg.run_aliases
        .insert("pyc".to_string(), "python ${x:-test}".to_string());
    cfg.run_alias_hints
        .insert("pyc".to_string(), "#x --hint-ext py,toml".to_string());
    cfg.run_aliases
        .insert("cop".to_string(), "cp ${x} ${y}".to_string());
    cfg.run_alias_hints.insert(
        "cop".to_string(),
        "#x --hint-all #y --hint-tp --hint-dir".to_string(),
    );
    cfg.run_aliases
        .insert("plain".to_string(), "echo ${x}".to_string());
    cfg
}

#[test]
fn alias_hint_dir_lists_dot_and_dirs() {
    let _cwd = TempCwd::with_repo_fixture();
    Config::with_test_config(hint_test_config(), || {
        let c = alias_arg_candidates_for(&["tld"], "", true).unwrap();
        assert!(c.contains(&".".to_string()), "got: {:?}", c);
        assert!(c.contains(&"src".to_string()), "got: {:?}", c);
        assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
        // Dir hint: no files.
        assert!(!c.contains(&"notes.txt".to_string()), "got: {:?}", c);
        // End-to-end ghost path.
        let h = NtcHinter::new();
        assert!(h.candidates_for("tld ").contains(&"src".to_string()));
        assert!(h.candidates_for("tld s").contains(&"src".to_string()));
    });
}

#[test]
fn alias_hint_ext_filters_files() {
    let _cwd = TempCwd::with_repo_fixture();
    Config::with_test_config(hint_test_config(), || {
        let c = alias_arg_candidates_for(&["pyc"], "", true).unwrap();
        assert!(c.contains(&"app.toml".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"notes.txt".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"photo.png".to_string()), "got: {:?}", c);
    });
}

#[test]
fn alias_hint_positional_and_call_form_index() {
    let _cwd = TempCwd::with_repo_fixture();
    Config::with_test_config(hint_test_config(), || {
        // Second positional slot → y's hints (tp + dir).
        let c = alias_arg_candidates_for(&["cop", "a.txt"], "", true).unwrap();
        assert!(c.contains(&".".to_string()), "got: {:?}", c);
        assert!(c.contains(&"src".to_string()), "got: {:?}", c);
        // Call form `cop(a, b` → same second slot (the `b`
        // fragment lives outside parent parts, like candidates_for).
        let c = alias_arg_candidates_for(&["cop(a,"], "b", false).unwrap();
        assert!(c.contains(&".".to_string()), "got: {:?}", c);
        let h = NtcHinter::new();
        assert!(h.candidates_for("cop(a, ").contains(&".".to_string()));
        // First slot → x's hints (all: files present).
        let c = alias_arg_candidates_for(&["cop"], "", true).unwrap();
        assert!(c.contains(&"notes.txt".to_string()), "got: {:?}", c);
        // Out-of-range slot → None (default behavior).
        assert!(alias_arg_candidates_for(&["cop", "a", "b", "c"], "", true).is_none());
    });
}

#[test]
fn alias_hint_bracket_elements_complete() {
    let _cwd = TempCwd::with_repo_fixture();
    let mut cfg = hint_test_config();
    cfg.run_aliases
        .insert("runc".to_string(), "gcc -o ${o}.exe $x[]".to_string());
    cfg.run_alias_hints
        .insert("runc".to_string(), "#x[] --hint-ext c".into());
    // tld's dir hints observe brackets too (elements = dirs).
    Config::with_test_config(cfg, || {
        // `runc o [ma` — element slot of the array param, head kept
        // so the ghost preserves the bracket.
        let c = alias_arg_candidates_for(&["runc", "o"], "[ma", false).unwrap();
        assert!(c.iter().all(|s| s.starts_with('[')), "got: {:?}", c);
        // `tld [s` — dir elements with head.
        let c = alias_arg_candidates_for(&["tld"], "[s", false).unwrap();
        assert!(c.contains(&"[src".to_string()), "got: {:?}", c);
        // Multi-element group `tld [src, d` — still slot 0; the
        // bare frag completes bare elements (delimiters live in
        // parent tokens, so they must not leak into the head).
        let c = alias_arg_candidates_for(&["tld", "[src,"], "d", false).unwrap();
        assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
        assert!(!c.iter().any(|s| s.starts_with('[')), "got: {:?}", c);
        // Same, but the bracket lives in the frag itself.
        let c = alias_arg_candidates_for(&["tld"], "[src, d", false).unwrap();
        assert!(c.contains(&"[src, docs".to_string()), "got: {:?}", c);
        // Trailing space inside brackets → bare elements.
        let c = alias_arg_candidates_for(&["tld", "[src,"], "", true).unwrap();
        assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
        assert!(!c.iter().any(|s| s.starts_with('[')), "got: {:?}", c);
        // Closed bracket ends the slot → past-the-end, no hints.
        assert!(alias_arg_candidates_for(&["runc", "o", "[a.c]"], "", true).is_none());
        // End-to-end: ghost keeps the bracket text.
        let h = NtcHinter::new();
        let c = h.candidates_for("tld [");
        assert!(!c.is_empty(), "expected bracket-headed ghosts");
        assert!(c.iter().all(|s| s.starts_with('[')), "got: {:?}", c);
    });
}

#[test]
fn ral_hint_suggests_values_named_and_hintn() {
    let h = NtcHinter::new();
    // Bare --hint / --select and their -n forms are suggested.
    let c = h.candidates_for("ral add tld \"tl dir ${x}\" #x --sel");
    assert!(c.contains(&"--select-dir".to_string()), "got: {:?}", c);
    assert!(c.contains(&"--select-n".to_string()), "got: {:?}", c);
    assert!(c.contains(&"--select".to_string()), "got: {:?}", c);
    let c = h.candidates_for("ral add tld \"tl dir ${x}\" #x --hin");
    assert!(c.contains(&"--hint-n".to_string()), "got: {:?}", c);
    assert!(c.contains(&"--hint".to_string()), "got: {:?}", c);
    // hintn subcommands complete.
    let c = h.candidates_for("ral ");
    assert!(c.contains(&"hintn".to_string()), "got: {:?}", c);
    let c = h.candidates_for("ral hintn ");
    assert!(c.contains(&"add".to_string()), "got: {:?}", c);
    assert!(c.contains(&"ls".to_string()), "got: {:?}", c);
    // After --hint-n / --select-n, list names complete.
    let mut cfg = hint_test_config();
    cfg.run_hint_lists.insert(
        "port".to_string(),
        vec!["8080".to_string(), "5050".to_string()],
    );
    Config::with_test_config(cfg, || {
        let h = NtcHinter::new();
        let c = h.candidates_for("ral add srv \"ssh ${p}\" #p --hint-n ");
        assert!(c.contains(&"port".to_string()), "got: {:?}", c);
        let c = h.candidates_for("ral add srv \"ssh ${p}\" #p --hint-n p");
        assert!(c.contains(&"port".to_string()), "got: {:?}", c);
        // hintn rm/info complete list names too.
        let c = h.candidates_for("ral hintn rm ");
        assert!(c.contains(&"port".to_string()), "got: {:?}", c);
        let c = h.candidates_for("ral hintn info p");
        assert!(c.contains(&"port".to_string()), "got: {:?}", c);
    });
}

#[test]
fn alias_hint_values_and_named_complete() {
    let _cwd = TempCwd::with_repo_fixture();
    let mut cfg = hint_test_config();
    cfg.run_aliases
        .insert("srv".to_string(), "ssh ${p}".to_string());
    cfg.run_alias_hints
        .insert("srv".to_string(), "#p --hint [8080,5050]".into());
    cfg.run_aliases
        .insert("srvn".to_string(), "ssh ${p}".to_string());
    cfg.run_alias_hints
        .insert("srvn".to_string(), "#p --hint-n port".into());
    cfg.run_aliases
        .insert("srvboth".to_string(), "ssh ${p}".to_string());
    cfg.run_alias_hints.insert(
        "srvboth".to_string(),
        "#p --hint [8080] --select-n port".into(),
    );
    cfg.run_hint_lists.insert(
        "port".to_string(),
        vec!["8080".to_string(), "5050".to_string()],
    );
    Config::with_test_config(cfg, || {
        // Inline values complete verbatim.
        let c = alias_arg_candidates_for(&["srv"], "", true).unwrap();
        assert!(c.contains(&"8080".to_string()), "got: {:?}", c);
        assert!(c.contains(&"5050".to_string()), "got: {:?}", c);
        // Named refs resolve through the store.
        let c = alias_arg_candidates_for(&["srvn"], "", true).unwrap();
        assert!(c.contains(&"8080".to_string()), "got: {:?}", c);
        assert!(c.contains(&"5050".to_string()), "got: {:?}", c);
        // Mixed inline + named union.
        let c = alias_arg_candidates_for(&["srvboth"], "", true).unwrap();
        assert!(c.contains(&"8080".to_string()), "got: {:?}", c);
        assert!(c.contains(&"5050".to_string()), "got: {:?}", c);
    });
    // Dangling named refs complete nothing (no fallback leak).
    let mut cfg2 = hint_test_config();
    cfg2.run_aliases
        .insert("dangle".to_string(), "ssh ${p}".to_string());
    cfg2.run_alias_hints
        .insert("dangle".to_string(), "#p --hint-n ghost".into());
    Config::with_test_config(cfg2, || {
        let c = alias_arg_candidates_for(&["dangle"], "", true).unwrap();
        assert!(c.is_empty(), "got: {:?}", c);
    });
}

#[test]
fn alias_hint_missing_falls_back() {
    let _cwd = TempCwd::with_repo_fixture();
    Config::with_test_config(hint_test_config(), || {
        // Unaliased root → None.
        assert!(alias_arg_candidates_for(&["nosuchalias"], "", true).is_none());
        assert!(alias_arg_candidates_for(&["nosuchalias", "x"], "", true).is_none());
        // Alias without hints → None (today's empty behavior kept).
        assert!(alias_arg_candidates_for(&["plain"], "", true).is_none());
        // Builtins never route here even if named alike (reserved).
        assert!(alias_arg_candidates_for(&["view"], "", true).is_none());
    });
}

#[test]
fn ral_hint_suggests_params_then_flags() {
    let h = NtcHinter::new();
    // `#` after a template with placeholders → #param names.
    let c = h.candidates_for("ral add tld \"tl dir ${x:-.}\" #");
    assert!(c.contains(&"#x".to_string()), "got: {:?}", c);
    // Array placeholders keep their brackets.
    let c = h.candidates_for("ral edit runc \"gcc $x[]\" #");
    assert!(c.contains(&"#x[]".to_string()), "got: {:?}", c);
    // `--hint` prefix → the hint flags.
    let c = h.candidates_for("ral add tld \"tl dir ${x}\" #x --hint-");
    for f in [
        "--hint-tp",
        "--hint-ext",
        "--hint-ext-name",
        "--hint-spf",
        "--hint-file",
        "--hint-file-name",
        "--hint-dir",
        "--hint-fd",
        "--hint-all",
    ] {
        assert!(c.contains(&f.to_string()), "missing {} in {:?}", f, c);
    }
    // `--select` prefix → the select flags.
    let c = h.candidates_for("ral add tld \"tl dir ${x}\" #x --select-");
    for f in [
        "--select-dir",
        "--select-ext",
        "--select-ext-name",
        "--select-file",
        "--select-file-name",
        "--select-fd",
        "--select-tp",
    ] {
        assert!(c.contains(&f.to_string()), "missing {} in {:?}", f, c);
    }
    // Just-finished #param → flags as well.
    let c = h.candidates_for("ral add tld \"tl dir ${x}\" #x ");
    assert!(c.contains(&"--hint-dir".to_string()), "got: {:?}", c);
    // No placeholders → nothing extra.
    let c = h.candidates_for("ral add btr \"cargo build\" #");
    assert!(!c.iter().any(|s| s.starts_with('#')), "got: {:?}", c);
    // Non add/edit roots stay quiet.
    let c = h.candidates_for("ral list #");
    assert!(!c.iter().any(|s| s.starts_with('#')), "got: {:?}", c);
}

#[test]
fn hint_name_kinds_yield_file_stems() {
    use crate::shell::alias::ParamHintKind as K;
    let _cwd = TempCwd::with_repo_fixture();
    // `--hint-file-name` = stems of every CWD file.
    let c = hint_kind_candidates(&[K::FileName]);
    assert_eq!(c, vec!["Cargo", "app", "notes", "photo"]);
    // `--hint-ext-name toml` = stems of matching files only.
    let c = hint_kind_candidates(&[K::ExtName(vec!["toml".into()])]);
    assert_eq!(c, vec!["Cargo", "app"]);
    // Multi-extension union, sorted/deduped.
    let c = hint_kind_candidates(&[K::ExtName(vec!["txt".into(), "png".into()])]);
    assert_eq!(c, vec!["notes", "photo"]);
    // Non-matching extension → empty.
    assert!(hint_kind_candidates(&[K::ExtName(vec!["rs".into()])]).is_empty());
    // Bare `--hint-ext-name` behaves like `--hint-file-name`.
    assert_eq!(
        hint_kind_candidates(&[K::ExtName(vec![])]),
        vec!["Cargo", "app", "notes", "photo"]
    );
}

#[test]
fn image_slots_filter_to_images_plus_dirs() {
    // Every non-dir candidate must be an image file; dirs pass through
    // for `dir/<Tab>` traversal.
    let _cwd = TempCwd::with_repo_fixture();
    for parent in [
        &["image"][..],
        &["img"][..],
        &["image", "display"][..],
        &["image", "show"][..],
        &["img", "display"][..],
    ] {
        for c in fs_candidates_for(parent, "") {
            if c.ends_with('/') {
                continue;
            }
            let l = c.to_lowercase();
            assert!(
                IMAGE_EXTS.iter().any(|e| l.ends_with(e)),
                "non-image offered for {:?}: {}",
                parent,
                c
            );
        }
    }
    // Fixture has non-image files (Cargo.toml) that must be suppressed,
    // and `src/` dir that must still pass through.
    let c = fs_candidates_for(&["image"], "");
    assert!(
        !c.iter().any(|x| x == "Cargo.toml"),
        "non-image file offered: {:?}",
        c
    );
    assert!(
        c.iter().any(|x| x == "src/"),
        "dirs must pass image filter: {:?}",
        c
    );
    // Deeper positions (flags/values) complete nothing.
    assert!(fs_candidates_for(&["image", "a.png", "to"], "").is_empty());
    assert!(fs_candidates_for(&["image", "display", "a.png"], "").is_empty());
}

#[test]
fn fmt_source_slot_offers_convertible_files_only() {
    // Directories plus fmt-supported files only.
    let _cwd = TempCwd::with_fixture(
        &[
            ("README.md", "# hi\n"),
            ("Cargo.toml", "[package]\n"),
            ("notes.txt", "hello\n"),
            ("data.docx", "fake"),
            ("book.pdf", "fake"),
            ("book.xlsx", "fake"),
            ("image.png", "fake"),
            ("docs/raw.bin", "fake"),
        ],
        &["docs"],
    );
    let c = fs_candidates_for(&["fmt"], "");
    assert!(
        c.iter().any(|x| x == "README.md"),
        "md file must be offered: {:?}",
        c
    );
    assert!(
        c.iter().any(|x| x == "Cargo.toml"),
        "toml file must be offered: {:?}",
        c
    );
    assert!(
        c.iter().any(|x| x == "data.docx"),
        "docx file must be offered: {:?}",
        c
    );
    assert!(
        c.iter().any(|x| x == "notes.txt"),
        "txt file must be offered: {:?}",
        c
    );
    assert!(
        !c.iter().any(|x| x == "image.png"),
        "unsupported ext must be suppressed: {:?}",
        c
    );
    assert!(
        !c.iter().any(|x| x == "book.pdf" || x == "book.xlsx"),
        "write-only formats must not be offered as sources: {:?}",
        c
    );
    assert!(
        c.iter().any(|x| x == "docs/"),
        "dirs must pass for traversal: {:?}",
        c
    );
    // `fmt docs/` traversal filters inside the directory too.
    let c = fs_candidates_for(&["fmt"], "docs/");
    assert!(
        !c.iter().any(|x| x.ends_with(".bin")),
        "unsupported file listed inside traversal: {:?}",
        c
    );
    // Deeper positions (keyword/flag slots) complete no filesystem paths.
    assert!(fs_candidates_for(&["fmt", "x.md", "to"], "").is_empty());
    assert!(fs_candidates_for(&["fmt", "x.md", "-o"], "").is_empty());
}

#[test]
fn fmt_to_keyword_and_target_slots() {
    let h = NtcHinter::new();
    // After the source file: the keyword/flag slots.
    let c = h.candidates_for("fmt README.md ");
    assert!(c.contains(&"to".to_string()), "got: {:?}", c);
    assert!(c.contains(&"-o".to_string()), "got: {:?}", c);
    assert!(c.contains(&"--output".to_string()), "got: {:?}", c);
    // While typing the keyword.
    assert!(h
        .candidates_for("fmt README.md t")
        .contains(&"to".to_string()));
    // The `to` target slot lists every conversion format…
    for fmt_ in [
        "json", "yaml", "toml", "md", "html", "xml", "docx", "txt", "pdf", "csv", "tsv", "xlsx",
    ] {
        assert!(
            h.candidates_for("fmt README.md to ")
                .contains(&fmt_.to_string()),
            "missing {fmt_}: got {:?}",
            h.candidates_for("fmt README.md to ")
        );
    }
    // …with prefix filtering.
    let c = h.candidates_for("fmt README.md to j");
    assert_eq!(c, vec!["json".to_string()]);
    // No filesystem paths in the `to` slot.
    assert!(!h
        .candidates_for("fmt README.md to ")
        .contains(&"README.md".to_string()));
}

#[test]
fn fmt_output_slot_completes_full_names() {
    let h = NtcHinter::new();
    // Bare `-o ` suggests `out.<ext>` filenames.
    let c = h.candidates_for("fmt x.md to json -o ");
    assert!(c.contains(&"out.json".to_string()), "got: {:?}", c);
    assert!(c.contains(&"out.docx".to_string()), "got: {:?}", c);
    assert!(c.contains(&"out.pdf".to_string()), "got: {:?}", c);
    assert!(c.contains(&"out.xlsx".to_string()), "got: {:?}", c);
    // A dotless stub grows a dot with the whole palette.
    let c = h.candidates_for("fmt x.md to json -o j");
    assert!(c.contains(&"j.json".to_string()), "got: {:?}", c);
    // Partial extension keeps the typed basename.
    let c = h.candidates_for("fmt x.md to json -o o.j");
    assert!(c.contains(&"o.json".to_string()), "got: {:?}", c);
    assert!(!c.contains(&"o.html".to_string()), "got: {:?}", c);
    // `md` prefix matches both `.md` and `.markdown` (case-insensitive).
    let c = h.candidates_for("fmt x.md to json --output n.m");
    assert!(c.contains(&"n.md".to_string()), "got: {:?}", c);
    assert!(c.contains(&"n.markdown".to_string()), "got: {:?}", c);
    // Slotted after other words? Never outside the fmt root or the
    // flag slot.
    assert!(h.candidates_for("scan src -o ").is_empty());
}

#[test]
fn image_sixel_flag_ghosts_after_path() {
    let h = NtcHinter::new();
    // Flag-first and flag-after-path both offer --sixel…
    assert!(h
        .candidates_for("image display --")
        .contains(&"--sixel".to_string()));
    assert!(h
        .candidates_for("image display a.png --")
        .contains(&"--sixel".to_string()));
    assert!(h
        .candidates_for("image show a.png --six")
        .contains(&"--sixel".to_string()));
    // …but other subcommands never do.
    assert!(!h
        .candidates_for("image foo --")
        .contains(&"--sixel".to_string()));
    // Top level still lists it alongside the other flags.
    assert!(h
        .candidates_for("image --")
        .contains(&"--sixel".to_string()));
}

#[test]
fn qur_flags_include_select_replace_display() {
    let h = NtcHinter::new();
    // Flag fragments ghost qur flags (plus universal --help/-h).
    assert!(h
        .candidates_for("qur --s")
        .contains(&"--select".to_string()));
    assert!(h
        .candidates_for("qur --")
        .contains(&"--display".to_string()));
    assert!(h
        .candidates_for("qur --")
        .contains(&"--replace".to_string()));
    assert!(h.candidates_for("qur -").contains(&"-s".to_string()));
    assert!(h.candidates_for("qur -").contains(&"-rp".to_string()));
    assert!(h.candidates_for("qur -").contains(&"-dp".to_string()));
    assert!(h.candidates_for("qur --xyz").is_empty());
    // Value-taking flag awaiting its value offers no subcommands.
    assert!(subcommands_for(&["qur", "-s"]).is_empty());
    assert!(subcommands_for(&["qur", "-d"]).is_empty());
    assert!(!subcommands_for(&["qur"]).is_empty());
}

#[test]
fn qur_glob_slot_only_before_content_flags() {
    // Glob slot: files complete.
    assert!(qur_glob_slot(&["qur"]));
    assert!(qur_glob_slot(&["qur", "*.rs"]));
    assert!(qur_glob_slot(&["qur", "*.rs", "-d", "3"]));
    // Value expected: no files.
    assert!(!qur_glob_slot(&["qur", "-s"]));
    assert!(!qur_glob_slot(&["qur", "*.rs", "-s"]));
    assert!(!qur_glob_slot(&["qur", "*.rs", "-rp"]));
    assert!(!qur_glob_slot(&["qur", "*.rs", "-d"]));
    // Content flag present: glob phase over.
    assert!(!qur_glob_slot(&["qur", "*.rs", "-s", "foo"]));
    assert!(!qur_glob_slot(&["qur", "*.rs", "-s", "foo", "-dp"]));
    // fs candidates mirror the slot.
    assert!(fs_candidates_for(&["qur", "-s"], "").is_empty());
    assert!(fs_candidates_for(&["qur", "*.rs", "-s", "foo"], "").is_empty());
}

#[test]
fn txtf_flags_include_head_tail_betw() {
    let h = NtcHinter::new();
    assert!(h.candidates_for("txtf --h").contains(&"--head".to_string()));
    assert!(h.candidates_for("txtf --").contains(&"--betw".to_string()));
    assert!(h.candidates_for("txtf --xyz").is_empty());
    // Value-taking flag awaiting its value offers no subcommands.
    assert!(subcommands_for(&["txtf", "--head"]).is_empty());
    assert!(!subcommands_for(&["txtf"]).is_empty());
    // view --betw ghosts like the other valued flags.
    assert!(h.candidates_for("view --b").contains(&"--betw".to_string()));
    assert!(subcommands_for(&["view", "--betw"]).is_empty());
}

#[test]
fn help_completes_canonical_topics_only() {
    let h = NtcHinter::new();
    let c = h.candidates_for("help ");
    for t in [
        "cd", "config", "image", "arc", "scan", "tp", "nssh", "nsheet", "nms", "pipe", "fake",
        "time",
    ] {
        assert!(c.contains(&t.to_string()), "missing topic {}: {:?}", t, c);
    }
    // Group labels and aliases are not suggested — canonical names only.
    assert!(!c.contains(&"nav".to_string()), "got: {:?}", c);
    assert!(!c.contains(&"network".to_string()), "got: {:?}", c);
    assert!(!c.contains(&"img".to_string()), "got: {:?}", c);
    assert!(!c.contains(&"pipes".to_string()), "got: {:?}", c);
    // Prefix + case-insensitive filtering.
    assert_eq!(h.candidates_for("help im"), vec!["image".to_string()]);
    assert_eq!(h.candidates_for("help IM"), vec!["image".to_string()]);
    assert_eq!(h.candidates_for("help ar"), vec!["arc".to_string()]);
    assert_eq!(
        h.candidates_for("help pi"),
        vec!["ping".to_string(), "pipe".to_string()]
    );
    assert_eq!(
        h.candidates_for("help PI"),
        vec!["ping".to_string(), "pipe".to_string()]
    );
    assert_eq!(h.candidates_for("help fa"), vec!["fake".to_string()]);
    // Topic filled: no further hints.
    assert!(h.candidates_for("help image ").is_empty());
    assert!(h.candidates_for("help pipe ").is_empty());
}

#[test]
fn time_completions_offer_flags() {
    let h = NtcHinter::new();
    for candidate in ["--count", "--bench", "--compare", "--help"] {
        assert!(
            h.candidates_for("time ").contains(&candidate.to_string()),
            "missing {candidate}"
        );
    }
    assert!(h.candidates_for("time --co").contains(&"--count".to_string()));
    // The old subcommands are gone.
    assert!(!h.candidates_for("time ").contains(&"run".to_string()));
}

#[test]
fn fake_completions_offer_subcommands_and_types() {
    let subs = subcommands_for(&["fake"]);
    for s in ["gen", "types", "locales", "group", "help"] {
        assert!(subs.contains(&s), "fake missing subcommand {s}: {subs:?}");
    }
    let gen_flags = subcommands_for(&["fake", "gen"]);
    assert!(gen_flags.contains(&"-n"));
    assert!(gen_flags.contains(&"--group"));
    assert!(gen_flags.contains(&"--unique"));
    assert!(gen_flags.contains(&"--locale"));

    let group_subs = subcommands_for(&["fake", "group"]);
    assert!(group_subs.contains(&"export"));
    assert!(group_subs.contains(&"import"));

    let types = fake_names_for_command(&["fake", "gen"], "");
    assert!(types.iter().any(|t| t == "name"), "got: {types:?}");
    assert!(types.iter().any(|t| t == "email"), "got: {types:?}");

    let locales = fake_names_for_command(&["fake", "gen", "-l"], "");
    assert!(locales.iter().any(|l| l == "en_US"), "got: {locales:?}");
    assert!(locales.iter().any(|l| l == "th_TH"), "got: {locales:?}");

    // `fake gen -g <TAB>` / `fake group rm <TAB>` suggest saved group
    // names; the count depends on config, so only assert they don't panic.
    let _ = fake_names_for_command(&["fake", "gen", "-g"], "");
    let _ = fake_names_for_command(&["fake", "group", "rm"], "");
}

#[test]
fn pipe_stage_offers_only_the_pipe_filters() {
    let h = NtcHinter::new();
    let filters = vec![
        "head".to_string(),
        "tail".to_string(),
        "betw".to_string(),
        "gs".to_string(),
        "sort".to_string(),
        "rev".to_string(),
        "count".to_string(),
        "trim".to_string(),
        "line".to_string(),
        "encode".to_string(),
        "qur".to_string(),
        "clip".to_string(),
    ];
    assert_eq!(h.candidates_for("gs main | "), filters);
    // Cursor immediately after the pipe (no trailing space) still lists them.
    assert_eq!(h.candidates_for("gs main |"), filters);
    assert_eq!(h.candidates_for("gs main | s"), vec!["sort".to_string()]);
    assert_eq!(h.candidates_for("gs main | r"), vec!["rev".to_string()]);
    assert_eq!(h.candidates_for("gs main | be"), vec!["betw".to_string()]);
    assert_eq!(h.candidates_for("gs main | tr"), vec!["trim".to_string()]);
    assert_eq!(h.candidates_for("gs main | li"), vec!["line".to_string()]);
    assert_eq!(h.candidates_for("gs main | e"), vec!["encode".to_string()]);
    assert_eq!(h.candidates_for("gs main | q"), vec!["qur".to_string()]);
    // No fallback for other commands (they never complete in a pipe).
    assert!(h.candidates_for("gs main | say ").is_empty());
    assert!(h.candidates_for("gs main | sa").is_empty());
    assert!(h.candidates_for("gs main | view a").is_empty());
    assert!(h.candidates_for("gs main | -").is_empty());
}

#[test]
fn pipe_stage_filters_complete_their_flags_and_files() {
    let h = NtcHinter::new();
    for flag in [
        "-r",
        "--reverse",
        "-n",
        "--numeric",
        "-u",
        "--unique",
        "-f",
        "--ignore-case",
    ] {
        assert!(
            h.candidates_for("gs main | sort -")
                .contains(&flag.to_string()),
            "sort missing {flag}"
        );
    }
    for flag in [
        "-p", "--path", "-s", "--sort", "-d", "-l", "--limit", "-I", "-x", "-h", "-g", "-w", "-z",
    ] {
        assert!(
            h.candidates_for("gs main | gs -")
                .contains(&flag.to_string()),
            "gs missing {flag}"
        );
    }
    for flag in [
        "-s",
        "--select",
        "-rp",
        "--replace",
        "-dp",
        "--display",
        "-d",
        "--depth",
        "-l",
        "--limit",
    ] {
        assert!(
            h.candidates_for("gs main | qur -")
                .contains(&flag.to_string()),
            "qur missing {flag}"
        );
    }
    for codec in ["b2", "b10", "b64", "base64", "hex", "url"] {
        assert!(
            h.candidates_for("gs main | encode ")
                .contains(&codec.to_string()),
            "encode missing {codec}"
        );
    }
    assert!(h
        .candidates_for("gs main | encode b64 -")
        .contains(&"-o".to_string()));
    assert!(h
        .candidates_for("gs main | encode b64 -")
        .contains(&"--output".to_string()));
    // Head/tail/betw/sort take files; `head`/`tail`/`betw` have no flags.
    assert!(h
        .candidates_for("gs main | head -")
        .contains(&"--help".to_string()));
    assert!(!h
        .candidates_for("gs main | head --")
        .contains(&"--tail".to_string()));
}

#[test]
fn pipe_stage_files_use_existing_command_completion() {
    let _cwd = TempCwd::with_repo_fixture();
    let h = NtcHinter::new();
    assert!(h
        .candidates_for("gs main | head ")
        .contains(&"notes.txt".to_string()));
    assert!(h
        .candidates_for("gs main | sort ")
        .contains(&"Cargo.toml".to_string()));
    assert!(h
        .candidates_for("gs main | qur ")
        .contains(&"Cargo.toml".to_string()));
}

#[test]
fn output_redirect_never_hints() {
    let _cwd = TempCwd::with_repo_fixture();
    let h = NtcHinter::new();
    assert!(h.candidates_for("gs main > ").is_empty());
    assert!(h.candidates_for("gs main > out").is_empty());
    assert!(h.candidates_for("gs main >").is_empty());
}

#[test]
fn append_and_input_redirects_hint_text_files_only() {
    let _cwd = TempCwd::with_repo_fixture();
    let h = NtcHinter::new();
    for prefix in ["gs main >> ", "gs main < ", "gs main >>", "gs main <"] {
        let c = h.candidates_for(prefix);
        assert!(
            c.contains(&"Cargo.toml".to_string()),
            "{prefix} missing text file: {c:?}"
        );
        assert!(
            c.contains(&"docs/".to_string()),
            "{prefix} missing traversal dir: {c:?}"
        );
        assert!(
            !c.iter().any(|x| x == "photo.png"),
            "{prefix} offered an image: {c:?}"
        );
    }
    // Prefix filtering and nested traversal.
    assert_eq!(
        h.candidates_for("gs main >> no"),
        vec!["notes.txt".to_string()]
    );
    assert!(h
        .candidates_for("gs main < src/")
        .contains(&"src/main.rs".to_string()));
    // Slot filled: nothing more to hint.
    assert!(h.candidates_for("gs main >> notes.txt ").is_empty());
    assert!(h.candidates_for("gs main < notes.txt ").is_empty());
}

#[test]
fn heredoc_delimiter_is_always_eof() {
    let h = NtcHinter::new();
    assert_eq!(h.candidates_for("say << "), vec!["EOF".to_string()]);
    assert_eq!(h.candidates_for("say <<"), vec!["EOF".to_string()]);
    assert_eq!(h.candidates_for("say <<E"), vec!["EOF".to_string()]);
    assert_eq!(h.candidates_for("say <<EOF"), Vec::<String>::new());
    assert_eq!(h.candidates_for("say <<EOF "), Vec::<String>::new());
    assert!(h.candidates_for("say <<XYZ").is_empty());
    // Here-strings take free text: no hint.
    assert!(h.candidates_for("say <<< ").is_empty());
    assert!(h.candidates_for("say <<< hi").is_empty());
}

#[test]
fn hyphen_stages_grow_one_boundary_at_a_time() {
    assert_eq!(stage_rest("et-Acl", "Get-Acl"), "et");
    assert_eq!(stage_rest("-Acl", "Get-Acl"), "-");
    assert_eq!(stage_rest("Acl", "Get-Acl"), "Acl");
    // Flags and paths keep their full remainder.
    assert_eq!(stage_rest("epth", "--depth"), "epth");
    assert_eq!(stage_rest("-file.txt", "my-file.txt"), "-file.txt");
    assert_eq!(stage_rest("ildItem", "Get-ChildItem"), "ildItem");
}

#[test]
fn ghost_remainder_adds_a_space_after_a_bare_operator() {
    assert_eq!(
        ghost_remainder("gs main |", "head"),
        Some(" head".to_string())
    );
    assert_eq!(
        ghost_remainder("gs main | ", "head"),
        Some("head".to_string())
    );
    assert_eq!(
        ghost_remainder("gs main | s", "sort"),
        Some("ort".to_string())
    );
    assert_eq!(ghost_remainder("say <<", "EOF"), Some(" EOF".to_string()));
    assert_eq!(
        ghost_remainder("ran reset &&", "tl"),
        Some(" tl".to_string())
    );
}

#[test]
fn double_pipe_segments_complete_commands() {
    let h = NtcHinter::new();
    assert!(h
        .candidates_for("gs main || tl ")
        .contains(&"ext".to_string()));
    assert_eq!(h.candidates_for("gs main || tl d"), vec!["dir".to_string()]);
}
