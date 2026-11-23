//! Help rendering tests: the contextual dispatcher and byte-identical goldens.

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

#[cfg(test)]
mod golden_tests {
    use super::*;
    use std::path::PathBuf;

    /// Every canonical command topic, plus the `pipe` operator topic.
    const TOPICS: &[&str] = &[
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

    /// Command-specific and generic contextual-help paths.
    const CONTEXTUAL: &[&str] = &[
        "cd",
        "cd to",
        "cd to some_tp",
        "cd src",
        "ignore",
        "ignore dir",
        "ignore dir foo",
        "care ext",
        "tp",
        "tp jump",
        "tp jump foo",
        "arc",
        "arc create",
        "scan",
        "scan ext",
        "qur",
        "qur *.*",
        "qur *.* -s foo",
        "time",
        "where",
    ];

    fn golden_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("shell")
            .join("help_golden")
            .join(format!("{name}.golden"))
    }

    fn normalized(out: String) -> String {
        crate::shell::pipeline::strip_ansi_text(&out).replace("\r\n", "\n")
    }

    fn assert_golden(name: &str, actual: &str) {
        let path = golden_path(name);
        if std::env::var_os("NTC_UPDATE_HELP_GOLDEN").is_some() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(&path, actual).unwrap();
            return;
        }

        let expected = std::fs::read_to_string(&path).unwrap_or_else(|_| {
            panic!(
                "missing help golden '{}'; run `NTC_UPDATE_HELP_GOLDEN=1 cargo test help_golden`",
                path.display()
            )
        });
        if expected != actual {
            let expected_lines: Vec<&str> = expected.lines().collect();
            let actual_lines: Vec<&str> = actual.lines().collect();
            let at = expected_lines
                .iter()
                .zip(actual_lines.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(expected_lines.len().min(actual_lines.len()));
            panic!(
                "help golden '{}' changed at line {}\n  expected: {:?}\n  actual:   {:?}\nRun `NTC_UPDATE_HELP_GOLDEN=1 cargo test help_golden` to refresh.",
                name,
                at + 1,
                expected_lines.get(at),
                actual_lines.get(at),
            );
        }
    }

    #[test]
    fn help_compact_matches_golden() {
        let ((), out) = capture_help(print_compact_help);
        assert_golden("compact", &normalized(out));
    }

    #[test]
    fn help_full_matches_golden() {
        let ((), out) = capture_help(print_interactive_help);
        assert_golden("full", &normalized(out));
    }

    #[test]
    fn help_topics_match_golden() {
        let mut actual = String::new();
        for topic in TOPICS {
            actual.push_str(&format!("=== {topic} ===\n"));
            let (handled, out) = capture_help(|| print_topic_help(topic));
            assert!(handled, "unknown help topic: {topic}");
            actual.push_str(&normalized(out));
        }
        assert_golden("topics", &actual);
    }

    #[test]
    fn help_contextual_matches_golden() {
        let mut actual = String::new();
        for prefix in CONTEXTUAL {
            actual.push_str(&format!("=== {prefix} ===\n"));
            let (handled, out) = capture_help(|| print_contextual_help(prefix));
            assert!(handled, "unhandled contextual help: {prefix}");
            actual.push_str(&normalized(out));
        }
        assert_golden("contextual", &actual);
    }

    #[test]
    fn help_topic_aliases_match_their_canonical_topic() {
        let aliases: &[(&str, &str)] = &[
            ("b", "back"),
            ("cfg", "config"),
            ("quit", "exit"),
            ("esc", "exit"),
            ("god", "godrive"),
            ("img", "image"),
            ("move", "mv"),
            ("ntceditor", "ne"),
            ("nfe", "explorer"),
            ("editcfg", "opencg"),
            ("reset-config", "resetcg"),
            ("restore-config", "restorecg"),
            ("r", "run"),
            ("script", "scr"),
            ("top", "sys"),
            ("td", "todo"),
            ("tracert", "trace"),
            ("traceroute", "trace"),
            ("guide", "tutorial"),
            ("pipes", "pipe"),
            ("redirect", "pipe"),
            ("redirection", "pipe"),
            ("pipeline", "pipe"),
            ("operator", "pipe"),
        ];
        for (alias, canonical) in aliases {
            let (alias_handled, alias_out) = capture_help(|| print_topic_help(alias));
            let (canonical_handled, canonical_out) = capture_help(|| print_topic_help(canonical));
            assert!(alias_handled, "unknown help alias: {alias}");
            assert!(canonical_handled, "unknown help topic: {canonical}");
            assert_eq!(
                normalized(alias_out),
                normalized(canonical_out),
                "help alias '{alias}' does not render topic '{canonical}'"
            );
        }
    }

    #[test]
    fn group_topics_no_longer_resolve() {
        // `config`, `view` and `igcare` are real commands; the old group
        // labels below are gone (D1).
        for group in [
            "nav",
            "network",
            "fileops",
            "search",
            "other",
            "alias",
            "encdec",
            "python",
            "pathman",
            "syscmd",
            "clipboard",
            "editor",
            "teleport",
        ] {
            let (handled, _) = capture_help(|| print_topic_help(group));
            assert!(!handled, "old group topic '{group}' must not resolve");
        }
    }
}

#[cfg(test)]
mod registry_tests {
    use crate::shell::commands::builtin::{COMPACT_HELP, CONTEXTUAL_HELP, HELP_ORDER, HELP_TOPICS};
    use crate::shell::commands::core::help::{contextual_for, find_topic};
    use std::collections::HashMap;

    #[test]
    fn help_order_lists_every_topic_exactly_once() {
        let mut in_order: HashMap<&str, usize> = HashMap::new();
        for name in HELP_ORDER {
            *in_order.entry(name).or_default() += 1;
        }
        for topic in HELP_TOPICS {
            assert_eq!(
                in_order.get(topic.topic).copied(),
                Some(1),
                "topic '{}' must appear exactly once in HELP_ORDER",
                topic.topic
            );
        }
        for name in HELP_ORDER {
            assert!(
                HELP_TOPICS.iter().any(|topic| topic.topic == *name),
                "HELP_ORDER names unknown topic: {name}"
            );
        }
        assert_eq!(in_order.len(), HELP_TOPICS.len());
    }

    #[test]
    fn help_names_and_aliases_are_unambiguous() {
        let mut claimed: HashMap<&str, &str> = HashMap::new();
        for topic in HELP_TOPICS {
            for name in std::iter::once(topic.topic).chain(topic.aliases.iter().copied()) {
                if let Some(previous) = claimed.insert(name, topic.topic) {
                    assert_eq!(
                        previous, topic.topic,
                        "help name '{name}' is claimed by both '{previous}' and '{}'",
                        topic.topic
                    );
                }
            }
        }
    }

    #[test]
    fn compact_and_contextual_tables_resolve() {
        for (topic, label, key) in COMPACT_HELP {
            assert!(
                find_topic(topic).is_some(),
                "COMPACT_HELP names unknown topic: {topic}"
            );
            assert!(!label.is_empty(), "COMPACT_HELP label is empty for {topic}");
            assert!(!key.is_empty(), "COMPACT_HELP key is empty for {topic}");
        }
        for entry in CONTEXTUAL_HELP {
            assert!(
                contextual_for(entry.root).is_some(),
                "CONTEXTUAL_HELP root has no handler: {}",
                entry.root
            );
        }
    }
}
