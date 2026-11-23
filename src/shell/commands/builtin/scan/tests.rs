use super::files::*;
use super::net::*;
use super::pid::{
    filter_and_sort_rows, is_local_host, parse_pid_target, parse_process_rows, powershell_commands,
    remote_commands, PidSort,
};
use super::tokenize;
use crate::config::Config;
use crate::filetype::{make_ignored_dir_filter, matches_any_extension};
use crate::shell::commands::builtin::nssh::nssh::scan as portscan;
use crate::shell::commands::builtin::nssh::nssh::store::{save_profile, Profile};
use walkdir::WalkDir;

fn scan_dir(
    root: &std::path::Path,
    exts: &[String],
    max_depth: usize,
    care: bool,
) -> Vec<(String, u64)> {
    let mut entries: Vec<(String, u64)> = Vec::new();
    if care {
        for entry in WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches_any_extension(&name, exts) {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            entries.push((rel, size));
        }
    } else {
        let walker = WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|e| {
                // Tests run with the real global config; mirror cmd_scan filtering.
                (make_ignored_dir_filter(&Config::get_ignored_dirs_combined()))(e)
            });
        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches_any_extension(&name, exts) {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            entries.push((rel, size));
        }
    }
    entries.sort_by_key(|(rel, _)| rel.to_lowercase());
    entries
}

#[test]
fn parse_exts_and_flags_in_any_order() {
    let p = parse_scan_args(&tokenize("rs c -d 3 -s")).unwrap();
    assert_eq!(p.exts, vec!["rs", "c"]);
    assert_eq!(p.max_depth, Some(3));
    assert!(p.show_size);
    assert!(!p.care);

    let p = parse_scan_args(&tokenize("-s --depth 2 .RS, c")).unwrap();
    assert_eq!(p.exts, vec!["rs", "c"]);
    assert_eq!(p.max_depth, Some(2));
    assert!(p.show_size);
    assert!(!p.care);

    let p = parse_scan_args(&tokenize("rs --care -s")).unwrap();
    assert_eq!(p.exts, vec!["rs"]);
    assert!(p.show_size);
    assert!(p.care);

    let p = parse_scan_args(&tokenize("--care rs c -d 2")).unwrap();
    assert_eq!(p.exts, vec!["rs", "c"]);
    assert_eq!(p.max_depth, Some(2));
    assert!(p.care);
}

#[test]
fn parse_rejects_missing_exts_and_bad_depth() {
    assert!(parse_scan_args(&tokenize("-s")).is_err());
    assert!(parse_scan_args(&tokenize("rs -d abc")).is_err());
    assert!(parse_scan_args(&tokenize("rs --bogus")).is_err());
}

#[test]
fn pid_target_defaults_to_localhost() {
    let _home = crate::utils::test_support::IsolatedConfigHome::new();
    let target = parse_pid_target(&tokenize("")).unwrap();
    assert_eq!(target.host, "localhost");
    assert_eq!(target.user, None);
    assert_eq!(target.port, 22);
    assert!(!target.trust);
    assert_eq!(target.sort, PidSort::Memory);
    assert!(!target.desc);
    assert_eq!(target.filter, None);
    assert_eq!(target.min_memory, None);
    assert_eq!(target.max_memory, None);
    assert_eq!(target.offset, 0);
    assert_eq!(target.limit, None);

    let target = parse_pid_target(&tokenize("host:localhost")).unwrap();
    assert_eq!(target.host, "localhost");

    let target = parse_pid_target(&tokenize("192.0.2.10")).unwrap();
    assert_eq!(target.host, "192.0.2.10");
    assert_eq!(target.user, None);
}

#[test]
fn pid_target_parses_ssh_syntax_and_flags() {
    let _home = crate::utils::test_support::IsolatedConfigHome::new();
    let target = parse_pid_target(&tokenize("alice@example.com:2222")).unwrap();
    assert_eq!(target.user.as_deref(), Some("alice"));
    assert_eq!(target.host, "example.com");
    assert_eq!(target.port, 2222);

    let target = parse_pid_target(&tokenize("[::1]:2200")).unwrap();
    assert_eq!(target.host, "::1");
    assert_eq!(target.port, 2200);

    let target = parse_pid_target(&tokenize("::1")).unwrap();
    assert_eq!(target.host, "::1");
    assert_eq!(target.port, 22);

    let target = parse_pid_target(&tokenize(
        "deploy@example.com --user root --port 2200 --key ~/.ssh/id_ed25519 \
         --pass-env PROD_PASS --trust --timeout 5",
    ))
    .unwrap();
    // Explicit flags override the user/host shorthand.
    assert_eq!(target.user.as_deref(), Some("root"));
    assert_eq!(target.port, 2200);
    assert_eq!(target.key.as_deref(), Some("~/.ssh/id_ed25519"));
    assert_eq!(target.pass_env.as_deref(), Some("PROD_PASS"));
    assert!(target.trust);
    assert_eq!(target.timeout, std::time::Duration::from_secs(5));

    let target =
        parse_pid_target(&tokenize("--sort pid --desc --filter CHROME example.com")).unwrap();
    assert_eq!(target.sort, PidSort::Pid);
    assert!(target.desc);
    assert_eq!(target.filter.as_deref(), Some("CHROME"));
    assert_eq!(target.host, "example.com");

    let target = parse_pid_target(&tokenize(
        "prod --min-memory 100MB --max-memory 2GB --offset 10 --limit 3",
    ))
    .unwrap();
    assert_eq!(target.min_memory, Some(100 * 1024 * 1024));
    assert_eq!(target.max_memory, Some(2 * 1024 * 1024 * 1024));
    assert_eq!(target.offset, 10);
    assert_eq!(target.limit, Some(3));
    let target = parse_pid_target(&tokenize("prod --limit 3")).unwrap();
    assert_eq!(target.limit, Some(3));
    let target = parse_pid_target(&tokenize("--head 5")).unwrap();
    assert_eq!(target.limit, Some(5));
    let target = parse_pid_target(&tokenize("-l 2")).unwrap();
    assert_eq!(target.limit, Some(2));

    assert_eq!(
        parse_pid_target(&tokenize("--sort size")).unwrap().sort,
        PidSort::Memory
    );
    assert_eq!(
        parse_pid_target(&tokenize("--sort NAME")).unwrap().sort,
        PidSort::Name
    );
}

#[test]
fn pid_filter_and_sort_rows() {
    let rows = || {
        vec![
            (10, "chrome.exe".to_string(), 300),
            (2, "Code.exe".to_string(), 500),
            (33, "Web Content".to_string(), 100),
            (4, "chrome renderer".to_string(), 300),
        ]
    };
    let pids = |rows: Vec<(u32, String, u64)>| rows.into_iter().map(|r| r.0).collect::<Vec<_>>();

    // Default: largest memory first, PID breaks ties.
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            None,
            None,
            PidSort::Memory,
            false
        )),
        vec![2, 4, 10, 33]
    );
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            None,
            None,
            PidSort::Memory,
            true
        )),
        vec![33, 4, 10, 2]
    );
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            None,
            None,
            PidSort::Pid,
            false
        )),
        vec![2, 4, 10, 33]
    );
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            None,
            None,
            PidSort::Pid,
            true
        )),
        vec![33, 10, 4, 2]
    );

    // Name ordering is case-insensitive; descending flips it.
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            None,
            None,
            PidSort::Name,
            false
        )),
        vec![4, 10, 2, 33]
    );
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            None,
            None,
            PidSort::Name,
            true
        )),
        vec![33, 2, 10, 4]
    );

    // Filter is a case-insensitive substring match.
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            Some("CHROME"),
            None,
            None,
            PidSort::Memory,
            false
        )),
        vec![4, 10]
    );
    assert!(
        filter_and_sort_rows(rows(), Some("firefox"), None, None, PidSort::Memory, false)
            .is_empty()
    );

    // Minimum memory keeps rows at or above the threshold.
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            Some(300),
            None,
            PidSort::Memory,
            false
        )),
        vec![2, 4, 10]
    );
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            Some(500),
            None,
            PidSort::Memory,
            false
        )),
        vec![2]
    );

    // Maximum memory keeps rows at or below the threshold.
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            None,
            Some(300),
            PidSort::Memory,
            false
        )),
        vec![4, 10, 33]
    );

    // Bounds are inclusive and combine.
    assert_eq!(
        pids(filter_and_sort_rows(
            rows(),
            None,
            Some(300),
            Some(500),
            PidSort::Memory,
            false
        )),
        vec![2, 4, 10]
    );
    assert!(
        filter_and_sort_rows(rows(), None, Some(301), Some(499), PidSort::Memory, false).is_empty()
    );
}

#[test]
fn pid_target_rejects_bad_input() {
    let _home = crate::utils::test_support::IsolatedConfigHome::new();
    for args in [
        "a b",
        "--bogus",
        "host:",
        "user@",
        "@host",
        "example.com:abc",
        "example.com:0",
        "example.com:70000",
        "[::1",
        "--port",
        "--port abc",
        "--timeout 0",
        "--timeout abc",
        "--user",
        "--sort",
        "--sort bogus",
        "--filter",
        "--filter \" \"",
        "--limit",
        "--limit 0",
        "--limit abc",
        "--head",
        "-l -1",
        "--min-memory",
        "--min-memory abc",
        "--max-memory",
        "--max-memory abc",
        "--min-memory 2GB --max-memory 1GB",
        "--offset",
        "--offset abc",
        "--offset -1",
    ] {
        assert!(parse_pid_target(&tokenize(args)).is_err(), "args: {args}");
    }
}

#[test]
fn pid_parses_posix_ps_rows() {
    let rows = parse_process_rows(
        "  PID   RSS COMMAND\n  123  1024 chrome\n    9  2048 Web Content\nnot a row\n",
        1024,
    );
    assert_eq!(
        rows,
        vec![
            (123, "chrome".to_string(), 1024 * 1024),
            (9, "Web Content".to_string(), 2048 * 1024),
        ]
    );
    assert!(parse_process_rows("", 1024).is_empty());
    assert!(parse_process_rows("  PID   RSS COMMAND\n", 1024).is_empty());

    // PowerShell emits WorkingSet64 in bytes, so the scale is 1.
    let rows = parse_process_rows("123 14077952 chrome\r\n9 1024 svchost\r\n", 1);
    assert_eq!(
        rows,
        vec![
            (123, "chrome".to_string(), 14077952),
            (9, "svchost".to_string(), 1024),
        ]
    );
}

#[test]
fn pid_resolves_nssh_profile_and_allows_overrides() {
    let _home = crate::utils::test_support::IsolatedConfigHome::new();
    save_profile(
        "prod",
        &Profile {
            host: "203.0.113.10".to_string(),
            user: "deploy".to_string(),
            port: 2222,
            key: Some("~/.ssh/id_ed25519".to_string()),
            pass_env: Some("PROD_PASS".to_string()),
        },
    )
    .unwrap();

    let target = parse_pid_target(&tokenize("prod")).unwrap();
    assert_eq!(target.host, "203.0.113.10");
    assert_eq!(target.user.as_deref(), Some("deploy"));
    assert_eq!(target.port, 2222);
    assert_eq!(target.key.as_deref(), Some("~/.ssh/id_ed25519"));
    assert_eq!(target.pass_env.as_deref(), Some("PROD_PASS"));
    assert_eq!(target.profile.as_deref(), Some("prod"));

    // CLI flags still override profile fields.
    let target = parse_pid_target(&tokenize(
        "prod --user root --port 2200 --key /tmp/k --pass-env OTHER",
    ))
    .unwrap();
    assert_eq!(target.host, "203.0.113.10");
    assert_eq!(target.user.as_deref(), Some("root"));
    assert_eq!(target.port, 2200);
    assert_eq!(target.key.as_deref(), Some("/tmp/k"));
    assert_eq!(target.pass_env.as_deref(), Some("OTHER"));

    // Explicit raw forms bypass profile resolution entirely.
    let target = parse_pid_target(&tokenize("alice@prod")).unwrap();
    assert_eq!(target.host, "prod");
    assert_eq!(target.user.as_deref(), Some("alice"));
    assert_eq!(target.profile, None);
    assert_eq!(target.key, None);

    let target = parse_pid_target(&tokenize("prod:2200")).unwrap();
    assert_eq!(target.host, "prod");
    assert_eq!(target.port, 2200);
    assert_eq!(target.profile, None);

    let target = parse_pid_target(&tokenize("host:prod")).unwrap();
    assert_eq!(target.host, "prod");
    assert_eq!(target.profile, None);

    // An unknown bare name is still a host.
    let target = parse_pid_target(&tokenize("not-a-saved-profile")).unwrap();
    assert_eq!(target.host, "not-a-saved-profile");
    assert_eq!(target.profile, None);
}

#[test]
fn pid_remote_commands_include_toybox_and_units() {
    let commands = remote_commands();
    assert_eq!(commands[0].0, "ps -A -o pid= -o rss= -o comm=");
    assert_eq!(commands[1].0, "ps -eo pid=,rss=,comm=");
    // Toybox (Android/Termux) reports RSS in KiB, like procps.
    assert_eq!(commands[2].0, "ps -A -o PID,RSS,NAME");
    assert_eq!(commands[2].1, 1024);
    // PowerShell reports WorkingSet64 in bytes.
    assert!(commands[3].0.starts_with("powershell "));
    assert_eq!(commands[3].1, 1);
    assert!(commands[4].0.starts_with("pwsh "));
    assert_eq!(commands[4].1, 1);
}

#[test]
fn pid_powershell_command_is_encoded_for_windows() {
    let commands = powershell_commands();
    assert_eq!(commands.len(), 2);
    assert!(commands[0].starts_with("powershell "));
    assert!(commands[1].starts_with("pwsh "));

    for command in commands {
        let encoded = command
            .split_once(" -EncodedCommand ")
            .expect("encoded command")
            .1;
        let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)
            .expect("valid base64");
        assert_eq!(bytes.len() % 2, 0, "UTF-16LE is byte-paired");
        let utf16: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();
        let script = String::from_utf16(&utf16).expect("valid UTF-16LE");
        assert!(script.contains("Get-Process"));
        assert!(script.contains("WorkingSet64"));
    }
}

#[test]
fn pid_detects_local_hosts() {
    assert!(is_local_host("localhost"));
    assert!(is_local_host("127.0.0.1"));
    assert!(is_local_host("::1"));
    // TEST-NET-1 can never be a local interface address.
    assert!(!is_local_host("192.0.2.1"));
}

#[test]
fn parse_net_flags_top_ports_all_timeout() {
    use std::time::Duration;
    let (ports, to, given) =
        parse_net_flags(&tokenize("--top 3"), portscan::TOP_PORTS, 800, true).unwrap();
    assert_eq!(ports, portscan::TOP_PORTS[..3].to_vec());
    assert_eq!(to, Duration::from_millis(800));
    assert!(!given);

    let (ports, _, _) = parse_net_flags(
        &tokenize("--ports 22,80-81"),
        portscan::TOP_PORTS,
        800,
        true,
    )
    .unwrap();
    assert_eq!(ports, vec![22, 80, 81]);

    let (ports, _, _) =
        parse_net_flags(&tokenize("--all"), portscan::TOP_PORTS, 800, true).unwrap();
    assert_eq!(ports.len(), 65535);
    assert_eq!((ports[0], ports[65534]), (1, 65535));

    // --all rejected where not allowed (scan ip).
    assert!(parse_net_flags(&tokenize("--all"), portscan::TOP_PORTS, 400, false).is_err());

    let (_, to, given) =
        parse_net_flags(&tokenize("--timeout 10"), portscan::TOP_PORTS, 800, true).unwrap();
    assert_eq!(to, Duration::from_millis(50)); // floored
    assert!(given);

    assert!(parse_net_flags(&tokenize("--top abc"), portscan::TOP_PORTS, 800, true).is_err());
    assert!(parse_net_flags(&tokenize("--timeout"), portscan::TOP_PORTS, 800, true).is_err());
    assert!(parse_net_flags(&tokenize("--bogus"), portscan::TOP_PORTS, 800, true).is_err());
    assert!(parse_net_flags(&tokenize("oops"), portscan::TOP_PORTS, 800, true).is_err());
}

#[test]
fn walk_filters_by_extension_case_insensitively() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.path().join("notes.TXT"), "hi").unwrap();
    std::fs::write(dir.path().join("photo.png"), "fake").unwrap();
    let sub = dir.path().join("sub");
    std::fs::create_dir(&sub).unwrap();
    std::fs::write(sub.join("lib.RS"), "x").unwrap();

    let exts = vec!["rs".to_string()];
    let found = scan_dir(dir.path(), &exts, 10, false);
    let names: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
    assert_eq!(names, vec!["main.rs", "sub/lib.RS"]);

    // Depth limit excludes the nested file (depth 2).
    let shallow = scan_dir(dir.path(), &exts, 1, false);
    assert_eq!(shallow.len(), 1);

    // Multi-ext + no match.
    let multi = scan_dir(
        dir.path(),
        &["rs".to_string(), "txt".to_string()],
        10,
        false,
    );
    assert_eq!(multi.len(), 3);
    let none = scan_dir(dir.path(), &["zzz".to_string()], 10, false);
    assert!(none.is_empty());

    // Care mode returns the same set when nothing is ignored.
    let cared = scan_dir(dir.path(), &exts, 10, true);
    assert_eq!(cared.len(), 2);
}

#[test]
fn wildcard_matches_segments_case_insensitively() {
    assert!(matches_wildcard("main.rs", "*.rs"));
    assert!(matches_wildcard("LIB.RS", "*.rs"));
    assert!(matches_wildcard("keep.ext", "k*.ext"));
    assert!(matches_wildcard("theend.ext", "*end.ext"));
    assert!(matches_wildcard("anything", "*"));
    assert!(matches_wildcard("main.rs", "main.rs"));
    assert!(matches_wildcard("MAIN.RS", "main.rs"));
    assert!(!matches_wildcard("main.rs", "main.py"));
    assert!(!matches_wildcard("main.rs", "k*"));
    assert!(!matches_wildcard("lib.rs", "*end.rs"));
    // Middle segments must appear in order.
    assert!(matches_wildcard("aXbYc", "a*b*c"));
    assert!(!matches_wildcard("aXc", "a*b*c"));
    // `*` matches empty runs.
    assert!(matches_wildcard("k", "k*"));
    assert!(matches_wildcard("end", "*end"));
    // Non-`*` chars stay literal.
    assert!(!matches_wildcard("kbc", "k?c"));
    assert!(!matches_wildcard("keep", "keep2"));
}

#[test]
fn parse_name_args_pattern_and_flags() {
    let p = parse_scan_name_args(&tokenize("*.rs -d 3 -s"), "cmd.scan.files_usage", false).unwrap();
    assert_eq!(p.pattern.as_deref(), Some("*.rs"));
    assert_eq!(p.max_depth, Some(3));
    assert!(p.show_size);
    assert!(!p.care);

    let p = parse_scan_name_args(
        &tokenize("--care k* --depth 2"),
        "cmd.scan.files_usage",
        false,
    )
    .unwrap();
    assert_eq!(p.pattern.as_deref(), Some("k*"));
    assert_eq!(p.max_depth, Some(2));
    assert!(p.care);

    // Bare / `*` / `*.*` for files all mean match-all.
    for args in ["", "*", "*.*"] {
        let p = parse_scan_name_args(&tokenize(args), "cmd.scan.files_usage", false).unwrap();
        assert!(p.pattern.is_none(), "args: {:?}", args);
    }
    // `*` for dir means match-all, but `*.*` is rejected.
    let p = parse_scan_name_args(&tokenize("*"), "cmd.scan.dir_usage", true).unwrap();
    assert!(p.pattern.is_none());
    assert!(parse_scan_name_args(&tokenize("*.*"), "cmd.scan.dir_usage", true).is_err());

    // Second bare token, bad depth, unknown flag all fail.
    assert!(parse_scan_name_args(&tokenize("a* b*"), "cmd.scan.files_usage", false).is_err());
    assert!(parse_scan_name_args(&tokenize("*.rs -d abc"), "cmd.scan.files_usage", false).is_err());
    assert!(parse_scan_name_args(&tokenize("--bogus"), "cmd.scan.files_usage", false).is_err());
}

#[test]
fn walk_matching_files_dirs_care_and_sizes() {
    let _cwd = crate::utils::test_support::TempCwd::with_fixture(
        &[
            ("keep.rs", "12345678"),
            ("skip.txt", "hi"),
            ("sub/deep.rs", "xyz"),
            ("target/secret.rs", "q"),
        ],
        &["sub", "target", "kbox", "backend"],
    );
    let mut cfg = Config::default();
    cfg.ignored_directory_names.insert("target".to_string());
    Config::with_test_config(cfg, || {
        let root = std::env::current_dir().unwrap();
        // Files: `*.rs` without care skips the ignored `target/`.
        let found = walk_matching(
            &root,
            10,
            false,
            false,
            &(|n: &str| matches_wildcard(n, "*.rs")),
        );
        let mut rels: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
        rels.sort();
        assert_eq!(rels, vec!["keep.rs", "sub/deep.rs"]);
        // Care mode includes everything.
        let found = walk_matching(&root, 10, true, false, &(|_: &str| true));
        assert_eq!(found.len(), 4);
        // Dirs: `k*` matches only `kbox`; the root itself is never listed.
        let found = walk_matching(
            &root,
            10,
            false,
            true,
            &(|n: &str| matches_wildcard(n, "k*")),
        );
        let rels: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
        assert_eq!(rels, vec!["kbox"]);
        // Dirs: match-all skips ignored `target/` without care ...
        let found = walk_matching(&root, 10, false, true, &(|_: &str| true));
        let mut rels: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
        rels.sort();
        assert_eq!(rels, vec!["backend", "kbox", "sub"]);
        // ... and includes it with care.
        let found = walk_matching(&root, 10, true, true, &(|_: &str| true));
        assert_eq!(found.len(), 4);
        // Sizes aggregate descendant file bytes (`sub` holds 3 bytes).
        let found = walk_matching(&root, 10, false, true, &(|_: &str| true));
        let size_of = |d: &str| {
            found
                .iter()
                .find(|(r, _)| r == d)
                .map(|(_, s)| *s)
                .unwrap_or(u64::MAX)
        };
        assert_eq!(size_of("sub"), 3);
        assert_eq!(size_of("backend"), 0);
        // Depth limit excludes nested entries (depth 2).
        let shallow = walk_matching(&root, 1, false, false, &(|_: &str| true));
        assert_eq!(shallow.len(), 2);
        let shallow_dirs = walk_matching(&root, 1, false, true, &(|_: &str| true));
        assert_eq!(shallow_dirs.len(), 3);
    });
}
