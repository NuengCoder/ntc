use super::args::{parse_delimiter_value, parse_nsheet_args};
use super::tokenize;
use super::tui::NsheetTui;
use super::view::prepare_view_in;

#[test]
fn tokenize_keeps_quoted_filter() {
    let t = tokenize(r#"view a.csv --filter "age>30 && city~york" -l 5"#);
    assert_eq!(
        t,
        vec![
            "view",
            "a.csv",
            "--filter",
            "age>30 && city~york",
            "-l",
            "5"
        ]
    );
}

#[test]
fn args_parse_view_with_flags() {
    let toks = tokenize("data.csv --sort age --desc --filter age>30 -l 10");
    let a = parse_nsheet_args(&toks).unwrap();
    assert_eq!(a.file, "data.csv");
    assert_eq!(a.sort_col, Some("age".into()));
    assert!(a.sort_desc);
    assert_eq!(a.filter, Some("age>30".into()));
    assert_eq!(a.limit, Some(10));
}

#[test]
fn args_reject_unknown_and_missing_file() {
    assert!(parse_nsheet_args(&tokenize("--sort age")).is_err());
    assert!(parse_nsheet_args(&tokenize("a.csv --bogus")).is_err());
    assert!(parse_nsheet_args(&tokenize("a.csv --export xml")).is_err());
}

#[test]
fn delimiter_values() {
    assert_eq!(parse_delimiter_value("auto").unwrap(), None);
    assert_eq!(parse_delimiter_value("tab").unwrap(), Some('\t'));
    assert_eq!(parse_delimiter_value(";").unwrap(), Some(';'));
    assert!(parse_delimiter_value("xx").is_err());
}

#[test]
fn prepare_view_filters_and_sorts() {
    use std::io::Write as _;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.csv");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(f, "name,age\nbob,50\nal,20\ncid,30").unwrap();
    let args = parse_nsheet_args(&tokenize("t.csv --filter age>25 --sort age")).unwrap();
    let (headers, rows, total) = prepare_view_in(dir.path(), &args).unwrap();
    assert_eq!(headers, vec!["name", "age"]);
    assert_eq!(total, 3);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][0], "cid");
    assert_eq!(rows[1][0], "bob");
}

#[test]
fn prepare_view_missing_file_errors() {
    let dir = tempfile::tempdir().unwrap();
    let args = parse_nsheet_args(&tokenize("nope.csv")).unwrap();
    assert!(prepare_view_in(dir.path(), &args).is_err());
}

fn contains_color_sgr(s: &str) -> bool {
    let mut rest = s;
    while let Some(start) = rest.find("\x1b[") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find(|c: char| c.is_ascii_alphabetic()) else {
            break;
        };
        if &rest[end..end + 1] == "m" {
            let params = &rest[..end];
            if params.split(';').any(|p| {
                p.parse::<u16>().is_ok_and(
                    |n| matches!(n, 30..=37 | 90..=97 | 40..=47 | 100..=107 | 38 | 39 | 48 | 49),
                )
            }) {
                return true;
            }
        }
        rest = &rest[end + 1..];
    }
    false
}

#[test]
fn tui_render_emits_no_color_when_disabled() {
    let mut cfg = crate::config::Config::new();
    cfg.color_enabled = false;
    crate::config::Config::with_test_config(cfg, || {
        let mut tui = NsheetTui {
            title: "test.csv".into(),
            headers: vec!["name".into(), "value".into()],
            base: vec![vec!["a".into(), "1".into()]],
            view: vec![],
            cur_r: 0,
            cur_c: 0,
            off_r: 0,
            off_c: 0,
            term_w: 40,
            term_h: 10,
            search: String::new(),
            sort_col: None,
            sort_desc: false,
            filter: String::new(),
            status: String::new(),
        };
        tui.recompute();
        let mut out = Vec::new();
        tui.render(&mut out).unwrap();
        let rendered = String::from_utf8_lossy(&out);
        assert!(
            !contains_color_sgr(&rendered),
            "color escape emitted with color disabled: {rendered:?}"
        );
    });
}
